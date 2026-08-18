//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/columns.md
//! @prompt-hash b359ce08
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

use crate::entities::content::Content;
use crate::entities::dir::Dir;
use crate::entities::elements::columns::ColumnsElem;
use crate::entities::layout_types::{FrameItem, Pt};
use crate::entities::style::Styles;
use crate::entities::value::Value;

use super::{FontMetrics, ImageSizer, Layouter};

/// Divide uma sequência de content pelos `Content::Colbreak`.
/// Garante pelo menos `count` segmentos — se houver menos colbreaks do que
/// `count - 1`, os segmentos em falta ficam vazios.
/// Retorna também um booleano que indica se houve pelo menos um `colbreak`
/// real no body (usado por P538c para escolher entre modo segmentado e
/// modo fluxo contínuo).
fn split_by_colbreak(content: &Content, count: usize) -> (Vec<Content>, bool) {
    let mut segments: Vec<Vec<Content>> = vec![Vec::new()];
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
    (segments.into_iter().map(Content::sequence).collect(), had_colbreak)
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
    // 1. Flush line pendente (columns são structural — começam
    //    em nova linha lógica).
    if layouter.regions.current.cursor_x.0 > layouter.regions.current.line_start_x.0 {
        layouter.flush_line();
    }

    let page_width = layouter.regions.current.width;
    let margin = layouter.page_config.margin;
    // rationale: PageConfig::margin é escalar único (f64) — left=right=top=bottom por definição do tipo (entities/layout_types.rs). 2.0 * margin é verdade algébrica estrutural. P1066.
    // rationale: PageConfig::margin é escalar único (f64) — left=right=top=bottom por definição do tipo (entities/layout_types.rs). 2.0 * margin é verdade algébrica estrutural. P1066.
    // rationale: PageConfig::margin é escalar único (f64) — left=right=top=bottom por definição do tipo (entities/layout_types.rs). 2.0 * margin é verdade algébrica estrutural. P1066.
    let usable_width = page_width - 2.0 * margin;
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
        let column_region_width = column_width + 2.0 * margin;

    // 4. Dividir body pelos colbreaks e detectar se há colbreaks reais.
    let (segments, had_colbreak) = split_by_colbreak(&e.body, count);

    // 5. Posições horizontais das colunas (origem x absoluta na página).
    let column_x_offsets: Vec<f64> = (0..count)
        .map(|i| margin + i as f64 * (column_width + gutter_pt))
        .collect();

    // **P626** — em RTL a ordem de leitura é da direita para a esquerda,
    // logo a primeira coluna a encher é a mais à direita. Invertemos os
    // offsets para que o índice 0 corresponda à coluna mais à direita e
    // `start_next_column` avance para a esquerda.
    // A direcção do texto é lida do `Content::Styled` que envolve o body,
    // porque a chain activa do `Layouter` ainda não incluiu esses estilos.
    let is_rtl = body_dir(&e.body) == Some(Dir::RTL);
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
            margin,
            e.page_columns,
        );
    } else {
        layout_flow(
            layouter,
            &segments[0],
            count,
            &column_x_offsets,
            column_region_width,
            margin,
        );
    }
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
    let ascender = layouter
        .metrics
        .vertical_metrics(layouter.style.size, &layouter.style)
        .0;
    let mut all_column_items: Vec<FrameItem> = Vec::new();
    let column_start_y = layouter.regions.current.cursor_y;
    let mut max_column_bottom_y = column_start_y;

    // Guardar footnotes eventualmente pendentes do contexto exterior.
    let saved_outer_footnotes = std::mem::take(&mut layouter.pending_footnote_bodies);

    for (idx, segment) in segments.iter().enumerate() {
        // Salvaguardar estado actual (os items da página principal ficam
        // intocados enquanto a coluna é renderizada num buffer isolado).
        let saved_items = std::mem::take(&mut layouter.regions.current.current_items);
        let saved_line = std::mem::take(&mut layouter.regions.current.current_line);
        let saved_cursor_x = layouter.regions.current.cursor_x;
        let saved_width = layouter.regions.current.width;

        // Activar modo coluna para flush de footnotes.
        layouter.column_mode = true;
        layouter.column_origin_x = column_x_offsets[idx];
        // column_width no Layouter é a largura total da mini-página,
        // para que flush_pending_footnote_bodies use column_width - 2*margin
        // como largura útil das notas.
        layouter.column_width = column_region_width;

        // Configurar region para a coluna actual (mini-página P553).
        layouter.regions.current.width = column_region_width;
        layouter.regions.current.cursor_x = Pt(margin);
        layouter.regions.current.line_start_x = Pt(margin);
        layouter.regions.current.cursor_y = column_start_y + ascender;

        // Layout do segmento e flush final da coluna.
        layouter.layout_content(segment);
        layouter.flush_line();
        if page_columns {
            // Notas por coluna: flush imediato no fundo desta coluna.
            layouter.flush_pending_footnote_bodies(None);
        }

        // Altura consumida por esta coluna.
        let column_bottom_y = layouter.regions.current.cursor_y;
        if column_bottom_y.0 > max_column_bottom_y.0 {
            max_column_bottom_y = column_bottom_y;
        }

        // Recolher items da coluna e restaurar estado.
        let mut column_items =
            std::mem::take(&mut layouter.regions.current.current_items);
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
        let footnote_bottom = max_column_bottom_y.0 + layouter.page_config.margin;
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

    // 7. Adicionar todos os items das colunas à página actual.
    layouter.regions.current.current_items.extend(all_column_items);

    // 8. Avançar o cursor principal para a altura máxima consumida pelas
    //    colunas (ou pelas notas, se empilhadas).
    layouter.regions.current.cursor_y = max_column_bottom_y;
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
            Pt(layouter.page_config.height - layouter.page_config.margin);
    }
}

/// Translada um `FrameItem` horizontalmente por `dx` pontos,
/// preservando a coordenada Y original.
fn translate_item_x(item: FrameItem, dx: f64) -> FrameItem {
    use crate::compiler::layout::helpers::{item_pos, translate_frame_item};
    let (x, y) = item_pos(&item);
    translate_frame_item(item, Pt(x + dx), Pt(y))
}
