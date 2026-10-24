//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/columns/segmented.md
//! @prompt-hash 9538ed4b
//! @layer L1
//! @updated 2026-10-24

use super::*;

pub(super) fn split_by_colbreak(content: &Content, count: usize) -> (Vec<Content>, bool) {
    let split = split_with_source_indices(content, count);
    (split.segments, split.had_colbreak)
}

pub(super) struct SplitSource {
    pub(super) segments: Vec<Content>,
    pub(super) had_colbreak: bool,
    pub(super) source_indices: Vec<Vec<usize>>,
    pub(super) sequence: bool,
    pub(super) outer_styled: bool,
}

pub(super) fn split_with_source_indices(content: &Content, count: usize) -> SplitSource {
    let mut segments: Vec<Vec<Content>> = vec![Vec::new()];
    let mut source_indices = vec![vec![]];
    // P1379 — `#set page(...)` transporta as propriedades da página num
    // único Styled exterior. Inspecionar o body para encontrar colbreaks e
    // reaplicar o mesmo wrapper uma vez por região irmã.
    let (content, outer_styles) = match content.source_body() {
        Content::Styled(styled) => (&styled.body, Some(&styled.styles)),
        other => (other, None),
    };
    let (children, sequence) = match content.source_body() {
        Content::Sequence(seq) => (seq.iter().collect::<Vec<_>>(), true),
        Content::Empty => (Vec::new(), false),
        other => (vec![other], false),
    };
    let mut had_colbreak = false;
    for (index, child) in children.into_iter().enumerate() {
        if matches!(child, Content::Colbreak(_)) {
            had_colbreak = true;
            segments.push(Vec::new());
            source_indices.push(vec![]);
        } else {
            segments.last_mut().unwrap().push(child.clone());
            source_indices.last_mut().unwrap().push(index);
        }
    }
    // Garantir `count` segmentos.
    while segments.len() < count {
        segments.push(Vec::new());
        source_indices.push(vec![]);
    }
    // Se por alguma razão houver mais segmentos do que count, fundir os
    // excessivos no último (preserva conteúdo).
    if segments.len() > count && count > 0 {
        let tail: Vec<Content> = segments.drain(count - 1..).flatten().collect();
        segments.push(tail);
        let tail = source_indices.drain(count - 1..).flatten().collect();
        source_indices.push(tail);
    }
    let segments = segments
        .into_iter()
        .map(|parts| {
            let segment = Content::sequence(parts);
            match outer_styles {
                Some(styles) => Content::styled(Box::new(segment), styles.clone()),
                None => segment,
            }
        })
        .collect();
    SplitSource {
        segments,
        had_colbreak,
        source_indices,
        sequence,
        outer_styled: outer_styles.is_some(),
    }
}

/// **P626/P627** — descobre a direcção de texto (canal `"text.dir"`) que
/// envolve o body de um `ColumnsElem`. O `text.dir` viaja no `Content::Styled`
/// do body (não na chain activa do `Layouter` quando `columns::layout` é
/// invocado), por isso percorremos o body procurando o `Styled` mais interno
/// que defina a propriedade sobre conteúdo visível. Recursão limitada a
/// `Sequence` e `Styled` — elementos estruturais transparentes são ignorados.
pub(super) fn body_dir(content: &Content) -> Option<Dir> {
    match content.source_body() {
        Content::Styled(styled) => {
            let body = &styled.body;
            let styles = &styled.styles;
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
pub(super) fn has_visible_content(content: &Content) -> bool {
    match content.source_body() {
        Content::Text(_) => true,
        Content::Sequence(seq) => seq.iter().any(has_visible_content),
        Content::Styled(styled) => has_visible_content(&styled.body),
        Content::Empty
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
        | Content::Pagebreak(_) => false,
        // Outros elementos (figuras, formas, etc.) presumem conteúdo visível.
        _ => true, // neutro: N16[β] — Content não-vazio (figuras, formas, etc.) presume visibilidade (predicado de ocupação)
    }
}

/// Modo segmentado (P537): cada segmento delimitado por `colbreak()` é
/// renderizado numa coluna da mesma página.
///
/// **P552** — quando `page_columns` é `true` (origem `#set page(columns:)`),
/// as notas de rodapé são flushadas no fundo de cada coluna. Quando é
/// `false` (origem `#columns()`), as notas são acumuladas e flushadas só no
/// final, empilhadas na primeira coluna (semântica do vanilla).
pub(super) fn layout_segmented<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    segments: &[Content],
    column_x_offsets: &[f64],
    column_region_width: f64,
    margin: f64,
    page_columns: bool,
    planned: Option<(&[Vec<super::super::page_running_requests::Cursor>], bool)>,
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
    let saved_outer_footnote_plans = std::mem::take(&mut layouter.pending_footnote_plans);
    let saved_outer_footnote_frames =
        std::mem::take(&mut layouter.pending_footnote_frames);
    let saved_outer_footnote_reserve =
        std::mem::replace(&mut layouter.footnote_reserved_height, 0.0);
    let saved_outer_footnote_capture = layouter.footnote_capture.take();
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
        layouter.open_link_segments();
        let saved_tracking = layouter.text_tracking_state.clone();
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
        if page_columns {
            layouter.footnote_reserved_height = 0.0;
            layouter.refresh_footnote_reservation();
        }

        // Layout do segmento e flush final da coluna.
        // P1379 — cada segmento explícito recebe uma distribuição `fr`
        // cursor-relativa própria, sem reutilizar `page_columns` como
        // sentinela e sem abrir outros contextos de `column_mode`.
        if let Some((routes, outer_styled)) = planned {
            let (routes, scopes) = normalize_singleton_route(
                layouter,
                routes.get(idx).map(Vec::as_slice).unwrap_or(&[]),
                outer_styled,
            );
            super::super::sequence::layout_segmented_region_planned(
                layouter,
                segment,
                Some(&routes),
            );
            for scope in scopes.into_iter().rev() {
                layouter.end_planned_packet(scope);
            }
        } else {
            super::super::sequence::layout_segmented_region(layouter, segment);
        }
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

        let segment_has_items = super::super::html_frame::items_have_material(
            &layouter.regions.current.current_items,
        );
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
        layouter.close_link_items();
        let mut column_items =
            std::mem::take(&mut layouter.regions.current.current_items);
        layouter.apply_pending_par_line_fixups(&mut column_items);
        layouter.regions.current.current_items = saved_items;
        layouter.regions.current.current_line = saved_line;
        layouter.text_tracking_state = saved_tracking;
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

    let local_notes = std::mem::take(&mut layouter.pending_footnote_bodies);
    let local_plans = std::mem::take(&mut layouter.pending_footnote_plans);
    let local_frames = std::mem::take(&mut layouter.pending_footnote_frames);

    // Restaurar footnotes do contexto exterior.
    layouter.pending_footnote_bodies = saved_outer_footnotes;
    layouter.pending_footnote_plans = saved_outer_footnote_plans;
    layouter.pending_footnote_frames = saved_outer_footnote_frames;
    layouter.footnote_reserved_height = saved_outer_footnote_reserve;
    layouter.footnote_capture = saved_outer_footnote_capture;
    layouter.pending_footnote_bodies.extend(local_notes);
    layouter.pending_footnote_plans.extend(local_plans);
    layouter.pending_footnote_frames.extend(local_frames);
    layouter.refresh_footnote_reservation();
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

/// Match only normalization performed by the known segment constructor/helper.
fn normalize_singleton_route<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    routes: &[super::super::page_running_requests::Cursor],
    outer_styled: bool,
) -> (
    Vec<super::super::page_running_requests::Cursor>,
    Vec<super::super::page_running_requests::PacketScope>,
) {
    use crate::compiler::introspect::page_running::PlanEdge;
    if routes.len() != 1 {
        return (routes.to_vec(), vec![]);
    }
    let mut cursor = routes[0].clone();
    let mut scopes = vec![];
    while let Some(child) = cursor.plan.child(cursor.packet, PlanEdge::SourceBody) {
        scopes.push(layouter.begin_planned_packet(cursor.clone()));
        cursor.packet = child;
    }
    if !outer_styled {
        if let Some(child) = cursor.plan.child(cursor.packet, PlanEdge::StyledBody) {
            scopes.push(layouter.begin_planned_packet(cursor.clone()));
            cursor.packet = child;
            while let Some(child) = cursor.plan.child(cursor.packet, PlanEdge::SourceBody)
            {
                scopes.push(layouter.begin_planned_packet(cursor.clone()));
                cursor.packet = child;
            }
        }
    }
    let sequence_len = cursor.plan.node(cursor.packet).and_then(|node| {
        if let Content::Sequence(parts) = &node.content {
            Some(parts.len())
        } else {
            None
        }
    });
    if let Some(length) = sequence_len {
        scopes.push(layouter.begin_planned_packet(cursor.clone()));
        let routes = (0..length)
            .filter_map(|index| {
                let child =
                    cursor.plan.child(cursor.packet, PlanEdge::SequencePart(index));
                if child.is_none() {
                    if let Some(pass) = &mut layouter.page_running_pass {
                        pass.invalid = true;
                    }
                    layouter.layout_errors.push(
                        crate::entities::source_result::SourceDiagnostic::error(
                            layouter.current_source_span,
                            "missing singleton column source child",
                        ),
                    );
                }
                child.map(|packet| super::super::page_running_requests::Cursor {
                    packet,
                    ..cursor.clone()
                })
            })
            .collect();
        (routes, scopes)
    } else {
        (vec![cursor], scopes)
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn modulo_compila_e_carrega() {}
}
