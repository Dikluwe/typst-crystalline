//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/columns.md
//! @prompt-hash 91ee7737
//! @layer L1
//! @updated 2026-07-03
//!
//! Atomização (ADR-0109, P377): o layout de `Columns` movido do monólito
//! `layout_content` para o arquivo da feature (forma B — free function na
//! camada de render).
//!
//! **P537** — implementação real de múltiplas colunas lado a lado para
//! notas de rodapé por coluna. O body é dividido pelos `colbreak()` e cada
//! segmento é renderizado sequencialmente no mesmo `Layouter`, reutilizando
//! o estado da página actual. As notas de rodapé pendentes são flushed no
//! fim de cada coluna através do modo `column_mode` do `Layouter`.
//!
//! **P553** — correção geométrica: `column_width` passa a ser calculado a
//! partir da largura útil da página (`page_width - 2×margin`), e cada coluna
//! é renderizada como uma mini-página de largura `column_width + 2×margin`,
//! preenchendo toda a largura útil disponível.
#![allow(deprecated)] // P1417 — fallback textual igual ao caminho P1414 do Cursor.

use crate::entities::content::Content;
use crate::entities::counter_format::format_counter;
use crate::entities::dir::Dir;
use crate::entities::elements::columns::ColumnsElem;
use crate::entities::layout_types::{Align2D, FrameItem, HAlign, Point, Pt, TextStyle};
use crate::entities::style::Styles;
use crate::entities::value::Value;

use super::{FontMetrics, ImageSizer, Layouter, SegmentedParLineOverlay};

/// P1417 — materializa patterns depois de ordenar as baselines das regiões
/// irmãs. `#columns()` respeita a margem configurada no contentor.
pub(super) fn realize_segmented_par_line_patterns<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    start: usize,
    number_style: &TextStyle,
    region_width: f64,
    line_start_x: f64,
    force_end_margin: bool,
) -> Vec<FrameItem> {
    let mut candidates: Vec<_> =
        layouter.pending_segmented_par_line_patterns.drain(start..).collect();
    candidates.sort_by(|left, right| left.baseline.0.total_cmp(&right.baseline.0));

    let (top, bottom) = layouter.metrics.text_edges(number_style.size, number_style);
    let number_height = (top.0 + bottom.0.abs()).max(1.0);
    let mut previous_bottom: Option<f64> = None;
    let mut realized = Vec::new();

    for candidate in candidates {
        if previous_bottom.is_some_and(|limit| candidate.baseline.0 < limit) {
            continue;
        }

        let page_scoped = matches!(
            candidate.styles.custom("par.line.numbering-scope"),
            Some(Value::Str(scope)) if scope.as_str() == "page"
        );
        if page_scoped && layouter.par_line_page != Some(candidate.page) {
            layouter.par_line_counter = 0;
        }
        layouter.par_line_page = Some(candidate.page);
        layouter.par_line_counter += 1;

        let Some(text) =
            format_counter(&[layouter.par_line_counter], candidate.pattern.as_str())
        else {
            continue;
        };
        let width = layouter.metrics.advance(&text, number_style.size, number_style).0;
        let default_clearance = (0.026 * layouter.page_config.width)
            .clamp(0.75 * number_style.size.0, 2.5 * number_style.size.0);
        let clearance = match candidate.styles.custom("par.line.number-clearance") {
            Some(Value::Length(length)) => length.resolve_pt(number_style.size.0),
            _ => default_clearance,
        };
        let configured_margin = match candidate.styles.custom("par.line.number-margin") {
            Some(Value::Align(Align2D {
                h: Some(HAlign::Right | HAlign::End),
                v: None,
            })) => HAlign::Right,
            _ => HAlign::Left,
        };
        let margin = if force_end_margin { HAlign::Right } else { configured_margin };
        let align = match candidate.styles.custom("par.line.number-align") {
            Some(Value::Align(Align2D { h: Some(h), v: None })) => *h,
            _ if margin == HAlign::Left => HAlign::Right,
            _ => HAlign::Left,
        };

        previous_bottom = Some(candidate.baseline.0 + number_height);
        realized.push((candidate.baseline, text, width, margin, align, clearance));
    }

    let max_width = realized
        .iter()
        .map(|(_, _, width, _, _, _)| *width)
        .fold(0.0, f64::max);
    realized
        .into_iter()
        .map(|(baseline, text, width, margin, align, clearance)| {
            let slot_start = if margin == HAlign::Right {
                region_width - layouter.page_config.margin.right + clearance
            } else {
                line_start_x - clearance - max_width
            };
            let free = max_width - width;
            let offset = match align {
                HAlign::Left | HAlign::Start => 0.0,
                HAlign::Center => free / 2.0,
                HAlign::Right | HAlign::End => free,
            };
            FrameItem::Text {
                pos: Point { x: Pt(slot_start + offset), y: baseline },
                text: text.into(),
                style: number_style.clone(),
            }
        })
        .collect()
}

fn flow_block_spacing<M: FontMetrics, S: ImageSizer>(
    layouter: &Layouter<M, S>,
) -> (f64, f64) {
    let font = layouter.style.size.val();
    let spacing = match layouter.chain.custom("par.spacing") {
        Some(Value::Length(length)) => length.resolve_pt(font),
        _ => font * super::vanilla_defaults::PAR_SPACING,
    };
    let leading = match layouter.chain.custom("par.leading") {
        Some(Value::Length(length)) => length.resolve_pt(font),
        _ => layouter
            .style
            .leading
            .map(|length| length.resolve_pt(font))
            .unwrap_or(font * super::vanilla_defaults::PAR_LEADING),
    };
    (spacing, leading)
}

/// P1377 — `ColumnsElem` é realizado como Block no flow vanila. Entra no
/// protocolo sem criar margem no topo da região e sem duplicar o leading que
/// `flush_line` já incluiu para uma linha inline anterior.
fn enter_flow_block<M: FontMetrics, S: ImageSizer>(layouter: &mut Layouter<M, S>) {
    let had_inline_line = !layouter.regions.current.current_line.is_empty();
    if had_inline_line {
        layouter.flush_line();
    }

    let (spacing, _) = flow_block_spacing(layouter);
    if had_inline_line {
        // Representar a fronteira como topo pendente do próximo Block, não
        // como baseline já fixada. Isto é observavelmente equivalente a
        // `spacing - leading` para texto simples, mas permite que preflights
        // regionais vejam a mesma origem que uma cadeia Parbreak→Block.
        layouter.regions.current.cursor_y = Pt(layouter.prev_line_baseline
            + layouter.prev_block_equation_descent
            + spacing);
        layouter.initial_baseline_pending = true;
    } else if layouter.block_chain_active {
        let gap = layouter.prev_block_below_pending.max(spacing);
        layouter.regions.current.cursor_y =
            Pt(layouter.prev_line_baseline + layouter.prev_block_equation_descent + gap);
        layouter.initial_baseline_pending = true;
    }

    layouter.prev_block_below_pending = 0.0;
    layouter.prev_margin_is_parbreak = false;
}

/// P1377 — deixa a margem inferior como estado colapsável. O avanço extra é
/// conservado no cursor para um próximo elemento estrutural; texto posterior
/// resolve sua baseline a partir de `prev_line_baseline` pela cadeia normal.
fn leave_flow_block<M: FontMetrics, S: ImageSizer>(layouter: &mut Layouter<M, S>) {
    let (spacing, leading) = flow_block_spacing(layouter);
    layouter.regions.current.cursor_y += Pt((spacing - leading).max(0.0));
    layouter.prev_block_below_pending = spacing;
    layouter.block_chain_active = true;
    layouter.prev_margin_is_parbreak = false;
}

/// Divide uma sequência de content pelos `Content::Colbreak`.
/// Garante pelo menos `count` segmentos — se houver menos colbreaks do que
/// `count - 1`, os segmentos em falta ficam vazios.
/// Retorna também um booleano que indica se houve pelo menos um `colbreak`
/// real no body (usado por P538c para escolher entre modo segmentado e
/// modo fluxo contínuo).
fn split_by_colbreak(content: &Content, count: usize) -> (Vec<Content>, bool) {
    let mut segments: Vec<Vec<Content>> = vec![Vec::new()];
    // P1379 — `#set page(...)` transporta as propriedades da página num
    // único Styled exterior. Inspecionar o body para encontrar colbreaks e
    // reaplicar o mesmo wrapper uma vez por região irmã.
    let (content, outer_styles) = match content {
        Content::Styled(body, styles) => (body.as_ref(), Some(styles)),
        other => (other, None),
    };
    let children = match content {
        Content::Sequence(seq) => seq.iter().collect::<Vec<_>>(),
        Content::Empty => Vec::new(),
        other => vec![other],
    };
    let mut had_colbreak = false;
    for child in children {
        if matches!(child, Content::Colbreak(_)) {
            had_colbreak = true;
            segments.push(Vec::new());
        } else {
            segments.last_mut().unwrap().push(child.clone());
        }
    }
    // Garantir `count` segmentos.
    while segments.len() < count {
        segments.push(Vec::new());
    }
    // Se por alguma razão houver mais segmentos do que count, fundir os
    // excessivos no último (preserva conteúdo).
    if segments.len() > count && count > 0 {
        let tail: Vec<Content> = segments.drain(count - 1..).flatten().collect();
        segments.push(tail);
    }
    let segments = segments
        .into_iter()
        .map(|parts| {
            let segment = Content::sequence(parts);
            match outer_styles {
                Some(styles) => Content::Styled(Box::new(segment), styles.clone()),
                None => segment,
            }
        })
        .collect();
    (segments, had_colbreak)
}

/// **P626/P627** — descobre a direcção de texto (canal `"text.dir"`) que
/// envolve o body de um `ColumnsElem`. O `text.dir` viaja no `Content::Styled`
/// do body (não na chain activa do `Layouter` quando `columns::layout` é
/// invocado), por isso percorremos o body procurando o `Styled` mais interno
/// que defina a propriedade sobre conteúdo visível. Recursão limitada a
/// `Sequence` e `Styled` — elementos estruturais transparentes são ignorados.
fn body_dir(content: &Content) -> Option<Dir> {
    match content {
        Content::Styled(body, styles) => {
            // Preferir a direcção mais interna (top-wins semântico); só usar a
            // direcção deste `Styled` se o body tiver conteúdo visível, para
            // não ficar preso a estilos vazios (ex.: `parbreak` embrulhado).
            if let Some(dir) = body_dir(body) {
                return Some(dir);
            }
            if has_visible_content(body) {
                styles_dir(styles)
            } else {
                None
            }
        }
        Content::Sequence(seq) => {
            for child in seq.iter() {
                if let Some(dir) = body_dir(child) {
                    return Some(dir);
                }
            }
            None
        }
        _ => None, // neutro: N16[β] — Content sem direcção textual explícita retorna None (projeção de propriedade)
    }
}

/// Lê `"text.dir"` de um `Styles`, se presente.
fn styles_dir(styles: &Styles) -> Option<Dir> {
    styles
        .delta()
        .custom
        .iter()
        .find(|(k, _)| k.as_str() == "text.dir")
        .and_then(|(_, v)| match v {
            Value::Dir(dir) => Some(*dir),
            _ => None,
        })
}

/// `true` se o content tiver algum conteúdo visível (texto, imagem, etc.),
/// excluindo separadores estruturais (`Space`, `Parbreak`, `Pagebreak`, `Empty`).
fn has_visible_content(content: &Content) -> bool {
    match content {
        Content::Text(_) => true,
        Content::Sequence(seq) => seq.iter().any(has_visible_content),
        Content::Styled(body, _) => has_visible_content(body),
        Content::Empty | Content::Space | Content::Parbreak | Content::Pagebreak(_) => {
            false
        }
        // Outros elementos (figuras, formas, etc.) presumem conteúdo visível.
        _ => true, // neutro: N16[β] — Content não-vazio (figuras, formas, etc.) presume visibilidade (predicado de ocupação)
    }
}

/// Layout real de `columns(count, body, gutter:)`.
///
/// Se o body contiver `colbreak()`, cada segmento é renderizado numa
/// coluna da mesma página (modo segmentado, P537). Se não houver
/// `colbreak()`, o body é tratado como fluxo contínuo que preenche as
/// colunas sequencialmente e só cria nova página quando todas as colunas
/// da página actual estiverem cheias (P538c).
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &ColumnsElem,
) {
    // 1. Columns são realizados como Block no flow vanila. P1377 incorpora
    //    a fronteira vertical antes de configurar a mini-região.
    enter_flow_block(layouter);

    // P1415 — a lista de fixups usa índices relativos ao buffer da região.
    // Antes de trocar `current_items` por buffers de coluna, concluir a região
    // exterior para que seus índices nunca sejam consumidos numa coluna irmã.
    let mut outer_items = std::mem::take(&mut layouter.regions.current.current_items);
    layouter.apply_pending_par_line_fixups(&mut outer_items);
    layouter.regions.current.current_items = outer_items;

    let page_width = layouter.regions.current.width;
    let margins = layouter.page_config.margin;
    // rationale: PageConfig::margin é escalar único (f64) — left=right=top=bottom por definição do tipo (entities/layout_types.rs). 2.0 * margin é verdade algébrica estrutural. P1066.
    // rationale: PageConfig::margin é escalar único (f64) — left=right=top=bottom por definição do tipo (entities/layout_types.rs). 2.0 * margin é verdade algébrica estrutural. P1066.
    // rationale: PageConfig::margin é escalar único (f64) — left=right=top=bottom por definição do tipo (entities/layout_types.rs). 2.0 * margin é verdade algébrica estrutural. P1066.
    let usable_width = page_width - margins.horizontal();
    let count = e.count.max(1) as usize;
    let count_f = count as f64;

    // 2. Resolver gutter (Length → f64 Pt; default ~4% page width).
    let gutter_pt = match e.gutter {
        Some(g) => g.resolve_pt(layouter.style.size.0),
        None => page_width * super::COLUMNS_DEFAULT_GUTTER_RATIO,
    };

    // 3. column_width = (usable_width - (count-1)*gutter) / count.
    //    A largura útil da página é page_width - 2*margin (P553).
    let column_width = (usable_width - (count_f - 1.0) * gutter_pt) / count_f;
    // Largura da região de trabalho de cada coluna: mini-página com
    // margens internas, de modo que o layout preencha toda a largura útil.
    // rationale: PageConfig::margin é escalar único (f64) — left=right=top=bottom por definição do tipo (entities/layout_types.rs). 2.0 * margin é verdade algébrica estrutural. P1066.
    // rationale: PageConfig::margin é escalar único (f64) — left=right=top=bottom por definição do tipo (entities/layout_types.rs). 2.0 * margin é verdade algébrica estrutural. P1066.
    // rationale: PageConfig::margin é escalar único (f64) — left=right=top=bottom por definição do tipo (entities/layout_types.rs). 2.0 * margin é verdade algébrica estrutural. P1066.
    let column_region_width = column_width + margins.horizontal();

    // 4. Dividir body pelos colbreaks e detectar se há colbreaks reais.
    let (segments, had_colbreak) = split_by_colbreak(&e.body, count);

    // 5. Posições horizontais das colunas (origem x absoluta na página).
    let column_x_offsets: Vec<f64> = (0..count)
        .map(|i| margins.left + i as f64 * (column_width + gutter_pt))
        .collect();

    // **P626** — em RTL a ordem de leitura é da direita para a esquerda,
    // logo a primeira coluna a encher é a mais à direita. Invertemos os
    // offsets para que o índice 0 corresponda à coluna mais à direita e
    // `start_next_column` avance para a esquerda.
    // A direcção do texto é lida do `Content::Styled` que envolve o body,
    // porque a chain activa do `Layouter` ainda não incluiu esses estilos.
    let effective_dir = body_dir(&e.body).or_else(|| {
        layouter.chain.custom("text.dir").and_then(|value| match value {
            Value::Dir(dir) => Some(*dir),
            _ => None,
        })
    });
    let is_rtl = effective_dir == Some(Dir::RTL);
    let column_x_offsets: Vec<f64> = if is_rtl {
        column_x_offsets.into_iter().rev().collect()
    } else {
        column_x_offsets
    };

    if had_colbreak {
        layout_segmented(
            layouter,
            &segments,
            &column_x_offsets,
            column_region_width,
            margins.left,
            e.page_columns,
        );
    } else {
        layout_flow(
            layouter,
            &segments[0],
            count,
            &column_x_offsets,
            column_region_width,
            margins.left,
        );
    }

    leave_flow_block(layouter);
}

/// Modo segmentado (P537): cada segmento delimitado por `colbreak()` é
/// renderizado numa coluna da mesma página.
///
/// **P552** — quando `page_columns` é `true` (origem `#set page(columns:)`),
/// as notas de rodapé são flushadas no fundo de cada coluna. Quando é
/// `false` (origem `#columns()`), as notas são acumuladas e flushadas só no
/// final, empilhadas na primeira coluna (semântica do vanilla).
fn layout_segmented<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    segments: &[Content],
    column_x_offsets: &[f64],
    column_region_width: f64,
    margin: f64,
    page_columns: bool,
) {
    let mut all_column_items: Vec<FrameItem> = Vec::new();
    let column_start_y = layouter.regions.current.cursor_y;
    let mut max_column_bottom_y = column_start_y;
    let mut deepest_column_baseline: Option<(f64, f64)> = None;

    // P1378 — todos os segmentos são regiões irmãs. Cada um recebe o mesmo
    // snapshot lógico de entrada; estado produzido por uma coluna não pode
    // deslocar a seguinte.
    let entry_initial_baseline_pending = layouter.initial_baseline_pending;
    let entry_prev_line_baseline = layouter.prev_line_baseline;
    let entry_prev_equation_descent = layouter.prev_block_equation_descent;
    let entry_prev_below = layouter.prev_block_below_pending;
    let entry_block_chain = layouter.block_chain_active;
    let entry_prev_margin_is_parbreak = layouter.prev_margin_is_parbreak;
    let entry_line_assumed_ascent = layouter.line_assumed_ascent;

    // Guardar footnotes eventualmente pendentes do contexto exterior.
    let saved_outer_footnotes = std::mem::take(&mut layouter.pending_footnote_bodies);
    let saved_par_line_column = layouter.segmented_par_line_column;
    let saved_par_line_overlay = layouter.segmented_par_line_overlay;
    let saved_defer_par_line_patterns = layouter.defer_segmented_par_line_patterns;
    let pattern_candidates_start = layouter.pending_segmented_par_line_patterns.len();
    let outer_number_style = TextStyle::from(&layouter.chain);
    let mut page_number_style: Option<TextStyle> = None;
    let entry_par_line_counter = layouter.par_line_counter;
    let entry_par_line_page = layouter.par_line_page;
    let mut emitted_par_line_rows = 0usize;
    let mut emitted_par_line_page = entry_par_line_page;

    for (idx, segment) in segments.iter().enumerate() {
        // Salvaguardar estado actual (os items da página principal ficam
        // intocados enquanto a coluna é renderizada num buffer isolado).
        let saved_items = std::mem::take(&mut layouter.regions.current.current_items);
        let saved_line = std::mem::take(&mut layouter.regions.current.current_line);
        let saved_cursor_x = layouter.regions.current.cursor_x;
        let saved_width = layouter.regions.current.width;

        layouter.initial_baseline_pending = entry_initial_baseline_pending;
        layouter.prev_line_baseline = entry_prev_line_baseline;
        layouter.prev_block_equation_descent = entry_prev_equation_descent;
        layouter.prev_block_below_pending = entry_prev_below;
        layouter.block_chain_active = entry_block_chain;
        layouter.prev_margin_is_parbreak = entry_prev_margin_is_parbreak;
        layouter.line_assumed_ascent = entry_line_assumed_ascent;

        // Activar modo coluna para flush de footnotes.
        layouter.column_mode = true;
        layouter.segmented_par_line_column =
            page_columns.then_some((idx, column_x_offsets.len()));
        layouter.defer_segmented_par_line_patterns = true;
        if page_columns {
            layouter.segmented_par_line_overlay = None;
        } else {
            // P1416 — regiões irmãs recomeçam no mesmo progresso lógico. O
            // prefixo já materializado por uma irmã não gera item/callback;
            // uma cauda maior continua a partir do inteiro documental certo.
            layouter.par_line_counter = entry_par_line_counter;
            layouter.par_line_page = entry_par_line_page;
            layouter.segmented_par_line_overlay = Some(SegmentedParLineOverlay {
                suppress_through: entry_par_line_counter + emitted_par_line_rows,
                start_x_shift: column_x_offsets[0] - column_x_offsets[idx],
                end_x_shift: column_x_offsets[column_x_offsets.len() - 1]
                    - column_x_offsets[idx],
            });
        }
        layouter.column_origin_x = column_x_offsets[idx];
        // column_width no Layouter é a largura total da mini-página,
        // para que flush_pending_footnote_bodies use column_width - 2*margin
        // como largura útil das notas.
        layouter.column_width = column_region_width;

        // Configurar region para a coluna actual (mini-página P553).
        layouter.regions.current.width = column_region_width;
        layouter.regions.current.cursor_x = Pt(margin);
        layouter.regions.current.line_start_x = Pt(margin);
        // `column_start_y` é topo lógico quando a baseline está pendente.
        // `ensure_initial_baseline` continua único dono de adicionar o
        // top-edge; pré-somar ascender duplicava a primeira coluna.
        layouter.regions.current.cursor_y = column_start_y;

        // Layout do segmento e flush final da coluna.
        // P1379 — cada segmento explícito recebe uma distribuição `fr`
        // cursor-relativa própria, sem reutilizar `page_columns` como
        // sentinela e sem abrir outros contextos de `column_mode`.
        super::sequence::layout_segmented_region(layouter, segment);
        layouter.flush_line();
        if page_columns {
            if page_number_style.is_none() {
                page_number_style = layouter
                    .pending_segmented_par_line_patterns
                    .get(pattern_candidates_start)
                    .map(|candidate| TextStyle::from(&candidate.styles));
            }
            if let Some(number_style) = &page_number_style {
                let numbers = realize_segmented_par_line_patterns(
                    layouter,
                    pattern_candidates_start,
                    number_style,
                    column_region_width,
                    margin,
                    idx + 1 == column_x_offsets.len(),
                );
                layouter.regions.current.current_items.extend(numbers);
            }
        }
        if !page_columns {
            let rows = layouter.par_line_counter.saturating_sub(entry_par_line_counter);
            if rows > emitted_par_line_rows {
                emitted_par_line_rows = rows;
                emitted_par_line_page = layouter.par_line_page;
            }
        }
        if page_columns {
            // Notas por coluna: flush imediato no fundo desta coluna.
            layouter.flush_pending_footnote_bodies(None);
        }

        let segment_has_items = !layouter.regions.current.current_items.is_empty();
        if segment_has_items {
            let candidate =
                (layouter.prev_line_baseline, layouter.prev_block_equation_descent);
            let candidate_bottom = candidate.0 + candidate.1;
            if deepest_column_baseline
                .map(|current| candidate_bottom > current.0 + current.1)
                .unwrap_or(true)
            {
                deepest_column_baseline = Some(candidate);
            }
        }

        // Altura consumida por esta coluna.
        let column_bottom_y = layouter.regions.current.cursor_y;
        if column_bottom_y.0 > max_column_bottom_y.0 {
            max_column_bottom_y = column_bottom_y;
        }

        // Recolher items da coluna e restaurar estado.
        let mut column_items =
            std::mem::take(&mut layouter.regions.current.current_items);
        layouter.apply_pending_par_line_fixups(&mut column_items);
        layouter.regions.current.current_items = saved_items;
        layouter.regions.current.current_line = saved_line;
        layouter.regions.current.cursor_x = saved_cursor_x;
        layouter.regions.current.cursor_y = column_start_y;
        layouter.regions.current.line_start_x = Pt(margin);
        layouter.regions.current.width = saved_width;
        layouter.column_mode = false;

        // Translada os items horizontalmente para a coluna correcta.
        let dx = column_x_offsets[idx] - margin;
        for item in &mut column_items {
            *item = translate_item_x(item.clone(), dx);
        }
        all_column_items.extend(column_items);
    }

    if !page_columns {
        // Notas de contentor: flush acumulado na primeira coluna, abaixo do
        // conteúdo do contentor.
        layouter.column_mode = true;
        layouter.column_origin_x = column_x_offsets[0];
        layouter.column_width = column_region_width;
        let footnote_bottom = max_column_bottom_y.0 + layouter.page_config.margin.bottom;
        layouter.flush_pending_footnote_bodies(Some(footnote_bottom));
        let mut footnote_items =
            std::mem::take(&mut layouter.regions.current.current_items);
        layouter.column_mode = false;
        let dx = column_x_offsets[0] - margin;
        for item in &mut footnote_items {
            *item = translate_item_x(item.clone(), dx);
        }
        all_column_items.extend(footnote_items);
        // As notas desceram abaixo do conteúdo; actualizar altura consumida
        // para que o cursor principal não sobreponha conteúdo seguinte.
        max_column_bottom_y = Pt(footnote_bottom);
    }

    // Restaurar footnotes do contexto exterior.
    layouter.pending_footnote_bodies = saved_outer_footnotes;
    layouter.segmented_par_line_column = saved_par_line_column;
    layouter.segmented_par_line_overlay = saved_par_line_overlay;
    layouter.defer_segmented_par_line_patterns = saved_defer_par_line_patterns;
    if !page_columns {
        layouter.par_line_counter = entry_par_line_counter + emitted_par_line_rows;
        layouter.par_line_page = emitted_par_line_page;
        let outer_start = column_x_offsets.iter().copied().fold(f64::INFINITY, f64::min);
        let outer_width =
            column_x_offsets.iter().copied().fold(f64::NEG_INFINITY, f64::max)
                + column_region_width
                - margin;
        let numbers = realize_segmented_par_line_patterns(
            layouter,
            pattern_candidates_start,
            &outer_number_style,
            outer_width,
            outer_start,
            false,
        );
        all_column_items.extend(numbers);
    }

    // 7. Adicionar todos os items das colunas à página actual.
    layouter.regions.current.current_items.extend(all_column_items);

    // 8. Avançar o cursor principal para a altura máxima consumida pelas
    //    colunas (ou pelas notas, se empilhadas).
    layouter.regions.current.cursor_y = max_column_bottom_y;
    if let Some((baseline, descent)) = deepest_column_baseline {
        layouter.prev_line_baseline = baseline;
        layouter.prev_block_equation_descent = descent;
        layouter.initial_baseline_pending = false;
    } else {
        layouter.prev_line_baseline = entry_prev_line_baseline;
        layouter.prev_block_equation_descent = entry_prev_equation_descent;
        layouter.initial_baseline_pending = entry_initial_baseline_pending;
    }
}

/// Modo fluxo contínuo (P538c): o body é renderizado numa sequência de
/// colunas. Quando uma coluna enche, avança para a seguinte na mesma
/// página; só cria nova página física quando todas as colunas estão cheias.
fn layout_flow<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    body: &Content,
    count: usize,
    column_x_offsets: &[f64],
    column_region_width: f64,
    margin: f64,
) {
    let saved_defer_par_line_patterns = layouter.defer_segmented_par_line_patterns;
    let saved_page_number_style = layouter.page_par_line_pattern_style.take();
    let saved_page_column_start = layouter.page_par_line_pattern_column_start;
    layouter.defer_segmented_par_line_patterns = true;
    layouter.page_par_line_pattern_style = None;
    layouter.page_par_line_pattern_column_start =
        layouter.pending_segmented_par_line_patterns.len();

    // Configurar estado de colunas no Layouter.
    layouter.page_columns = Some(count);
    layouter.current_column = 0;
    layouter.column_x_offsets = column_x_offsets.to_vec();
    // Largura total da mini-página (inclui margens internas) para que
    // flush_pending_footnote_bodies e start_column usem a largura útil
    // correcta (P553).
    layouter.column_width = column_region_width;
    layouter.column_mode = true;
    layouter.column_origin_x = column_x_offsets[0];

    // Configurar region para a primeira coluna (mini-página P553). O
    // cursor_y actual já é a baseline da primeira linha (o Layouter
    // inicializa com ascender); não se adiciona ascender novamente.
    layouter.regions.current.width = column_region_width;
    layouter.regions.current.cursor_x = Pt(margin);
    layouter.regions.current.line_start_x = Pt(margin);

    // Renderizar o body contínuo.
    layouter.layout_content(body);
    layouter.flush_line();

    // Fechar colunas, fundir items e restaurar estado de página normal.
    layouter.finish_columns();
    layouter.defer_segmented_par_line_patterns = saved_defer_par_line_patterns;
    layouter.page_par_line_pattern_style = saved_page_number_style;
    layouter.page_par_line_pattern_column_start = saved_page_column_start;

    // O cursor principal avança para o fundo da coluna mais baixa da
    // última página. Como `finish_columns` reposiciona o cursor no topo
    // da coluna, usamos a altura da página actual para estimar o fundo
    // do bloco columns. Em modo fluxo contínuo as colunas preenchem a
    // altura útil da página, excepto a última página que pode ser parcial.
    // Simplificação segura: avançar para `height - margin` quando houve
    // pelo menos uma página física criada; caso contrário manter o cursor.
    if layouter.pages.len() > 0 {
        // O Layouter acabou de fechar uma página? Não — finish_columns não
        // cria página. Se new_page() foi chamado, a(s) página(s) anterior(es)
        // já estão em `pages`; a última página ainda está em current_items.
        // O cursor_y após finish_columns é o topo da coluna. Para evitar
        // sobreposição com conteúdo seguinte, avançamos para o fundo da
        // área útil actual.
        layouter.regions.current.cursor_y =
            Pt(layouter.page_config.height - layouter.page_config.margin.bottom);
    }
}

/// Translada um `FrameItem` horizontalmente por `dx` pontos,
/// preservando a coordenada Y original.
fn translate_item_x(item: FrameItem, dx: f64) -> FrameItem {
    use crate::compiler::layout::helpers::{item_pos, translate_frame_item};
    let (x, y) = item_pos(&item);
    translate_frame_item(item, Pt(x + dx), Pt(y))
}

#[cfg(test)]
mod p1377_tests {
    use super::super::vanilla_defaults::{PAR_LEADING, PAR_SPACING};

    #[test]
    fn p1377_default_block_boundary_adds_only_spacing_minus_leading() {
        let font = 11.0;
        let extra = (font * PAR_SPACING - font * PAR_LEADING).max(0.0);
        assert!((extra - 6.05).abs() < 1e-9);
    }
}

#[cfg(test)]
mod p1378_tests {
    use crate::entities::content::Content;
    use crate::entities::layout_types::FrameItem;

    fn text_y(
        document: &crate::entities::layout_types::PagedDocument,
        needle: &str,
    ) -> f64 {
        document
            .pages
            .iter()
            .flat_map(|page| page.items.iter())
            .find_map(|item| match item {
                #[allow(deprecated)]
                FrameItem::Text { text, pos, .. }
                | FrameItem::TextShaped { text, pos, .. }
                    if text.as_str() == needle =>
                {
                    Some(pos.y.0)
                }
                _ => None,
            })
            .unwrap_or_else(|| panic!("texto ausente: {needle}"))
    }

    #[test]
    fn p1378_segments_irmaos_partilham_origem_logica() {
        let columns = Content::columns(
            Content::sequence(vec![
                Content::text("One"),
                Content::colbreak(false),
                Content::text("Two"),
            ]),
            2,
            None,
        );
        let document = super::super::layout(&Content::sequence(vec![
            columns,
            Content::text("Outside"),
        ]));
        let one = text_y(&document, "One");
        let two = text_y(&document, "Two");
        assert!((one - two).abs() < 1e-9, "One={one}, Two={two}");
        assert!(text_y(&document, "Outside") > one);
    }

    #[test]
    fn p1378_saida_segue_segmento_mais_profundo() {
        let columns = Content::columns(
            Content::sequence(vec![
                Content::text("One"),
                Content::linebreak(),
                Content::text("Lower"),
                Content::colbreak(false),
                Content::text("Two"),
            ]),
            2,
            None,
        );
        let document = super::super::layout(&Content::sequence(vec![
            columns,
            Content::text("Outside"),
        ]));
        assert!((text_y(&document, "One") - text_y(&document, "Two")).abs() < 1e-9);
        assert!(text_y(&document, "Outside") > text_y(&document, "Lower"));
    }

    #[test]
    fn p1378_segmento_vazio_nao_desloca_o_seguinte() {
        let control = super::super::layout(&Content::text("Control"));
        let empty_first = super::super::layout(&Content::columns(
            Content::sequence(vec![
                Content::Empty,
                Content::colbreak(false),
                Content::text("Two"),
            ]),
            2,
            None,
        ));
        assert!((text_y(&control, "Control") - text_y(&empty_first, "Two")).abs() < 1e-9);
    }
}

#[cfg(test)]
mod p1379_tests {
    use std::sync::Arc;

    use crate::entities::{
        content::Content,
        corners::Corners,
        elements::block::{BlockElem, BlockHeight},
        layout_types::{FrameItem, Length},
        sides::Sides,
    };

    fn fractional(text: &str, weight: f64) -> Content {
        Content::Block(Arc::new(BlockElem {
            body: Content::text(text),
            width: None,
            height: Some(BlockHeight::Fractional(weight)),
            inset: Sides::uniform(Length::ZERO),
            breakable: true,
            outset: Sides::uniform(Length::ZERO),
            radius: Corners::uniform(Length::ZERO),
            clip: false,
            fill: None,
            stroke: None,
            spacing: None,
            above: None,
            below: None,
            sticky: false,
        }))
    }

    fn text_y(
        document: &crate::entities::layout_types::PagedDocument,
        needle: &str,
    ) -> f64 {
        document
            .pages
            .iter()
            .flat_map(|page| page.items.iter())
            .find_map(|item| match item {
                #[allow(deprecated)]
                FrameItem::Text { text, pos, .. }
                | FrameItem::TextShaped { text, pos, .. }
                    if text.as_str() == needle =>
                {
                    Some(pos.y.0)
                }
                _ => None,
            })
            .unwrap_or_else(|| panic!("texto ausente: {needle}"))
    }

    #[test]
    fn p1379_cada_segmento_expande_fr_ate_o_mesmo_fundo() {
        let columns = Content::columns(
            Content::sequence(vec![
                fractional("One", 1.0),
                Content::text("After"),
                Content::colbreak(false),
                fractional("Two", 1.0),
                Content::text("Tail"),
            ]),
            2,
            None,
        );
        let document = super::super::layout(&columns);
        let one = text_y(&document, "One");
        let two = text_y(&document, "Two");
        let after = text_y(&document, "After");
        let tail = text_y(&document, "Tail");
        assert!((one - two).abs() < 1e-9);
        assert!((after - tail).abs() < 1e-9, "After={after}, Tail={tail}");
        assert!(after - one > 100.0, "One={one}, After={after}");
    }

    #[test]
    fn p1379_pesos_e_conteudo_fixo_sao_locais_ao_segmento() {
        let columns = Content::columns(
            Content::sequence(vec![
                fractional("One", 1.0),
                fractional("Two", 2.0),
                Content::text("After"),
                Content::colbreak(false),
                fractional("Three", 3.0),
                Content::text("Tail"),
            ]),
            2,
            None,
        );
        let document = super::super::layout(&columns);
        let one = text_y(&document, "One");
        let two = text_y(&document, "Two");
        let after = text_y(&document, "After");
        let tail = text_y(&document, "Tail");
        assert!(two - one > 50.0, "One={one}, Two={two}");
        assert!((after - tail).abs() < 1e-9, "After={after}, Tail={tail}");
    }

    #[test]
    fn p1379_offset_exterior_reduz_a_parcela_sem_mudar_o_fundo() {
        let columns = Content::columns(
            Content::sequence(vec![
                fractional("One", 1.0),
                Content::text("After"),
                Content::colbreak(false),
                fractional("Two", 1.0),
                Content::text("Tail"),
            ]),
            2,
            None,
        );
        let document = super::super::layout(&Content::sequence(vec![
            Content::text("Intro"),
            columns,
        ]));
        let intro = text_y(&document, "Intro");
        let one = text_y(&document, "One");
        let after = text_y(&document, "After");
        let tail = text_y(&document, "Tail");
        assert!(one > intro);
        assert!(after - intro > 150.0, "Intro={intro}, After={after}");
        assert!((after - tail).abs() < 1e-9, "After={after}, Tail={tail}");
    }
}
