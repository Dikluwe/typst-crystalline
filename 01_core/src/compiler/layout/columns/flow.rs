//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/columns/flow.md
//! @prompt-hash 2d175c9f
//! @layer L1
//! @updated 2026-10-24

use super::*;

pub(super) fn flow_block_spacing<M: FontMetrics, S: ImageSizer>(
    layouter: &Layouter<M, S>,
) -> (f64, f64) {
    let font = layouter.style.size.val();
    let spacing = match layouter.chain.custom("par.spacing") {
        Some(Value::Length(length)) => length.resolve_pt(font),
        _ => font * super::super::vanilla_defaults::PAR_SPACING,
    };
    let leading = match layouter.chain.custom("par.leading") {
        Some(Value::Length(length)) => length.resolve_pt(font),
        _ => layouter
            .style
            .leading
            .map(|length| length.resolve_pt(font))
            .unwrap_or(font * super::super::vanilla_defaults::PAR_LEADING),
    };
    (spacing, leading)
}

/// P1377 — `ColumnsElem` é realizado como Block no flow vanila. Entra no
/// protocolo sem criar margem no topo da região e sem duplicar o leading que
/// `flush_line` já incluiu para uma linha inline anterior.
pub(super) fn enter_flow_block<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
) {
    let had_inline_line = super::super::html_frame::items_have_material(
        &layouter.regions.current.current_line,
    );
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
pub(super) fn leave_flow_block<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
) {
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
/// Modo fluxo contínuo (P538c): o body é renderizado numa sequência de
/// colunas. Quando uma coluna enche, avança para a seguinte na mesma
/// página; só cria nova página física quando todas as colunas estão cheias.
pub(super) fn layout_flow<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    body: &Content,
    count: usize,
    column_x_offsets: &[f64],
    column_region_width: f64,
    margin: f64,
    page_column_notes: bool,
) {
    let saved_note_scope = layouter.page_column_notes;
    layouter.page_column_notes = page_column_notes;
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
    layouter.layout_planned_child(
        crate::compiler::introspect::page_running::PlanEdge::ColumnsBody,
        body,
    );
    let had_carrier = super::super::html_frame::items_have_causal_line(
        &layouter.regions.current.current_line,
    ) || layouter.terminal_paragraph_line;
    layouter.flush_line();
    if page_column_notes
        && layouter
            .pending_footnote_frames
            .first()
            .and_then(Option::as_ref)
            .is_some_and(|frame| frame.source_lines)
    {
        if had_carrier {
            layouter.regions.current.cursor_y = Pt(layouter.prev_line_baseline);
            if layouter.last_block_descent_y.is_none() {
                layouter.last_block_descent_y = Some(layouter.prev_line_baseline);
            }
        }
        layouter.flush_pending_footnote_bodies(None);
        layouter.drain_pending_footnotes(None, true);
    }

    // Fechar colunas, fundir items e restaurar estado de página normal.
    layouter.finish_columns();
    layouter.page_column_notes = saved_note_scope;
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
    if !layouter.pages.is_empty() {
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

#[cfg(test)]
mod smoke {
    #[test]
    fn modulo_compila_e_carrega() {}
}
