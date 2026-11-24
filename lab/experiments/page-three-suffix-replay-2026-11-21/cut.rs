
fn lab_three_cut<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<'_, M, S>,
    parts: &[Content],
    route: Option<&super::page_running_requests::Cursor>,
    start: usize,
    end: usize,
    text_segment_start: usize,
    owns_text_segment: bool,
    paragraph: &super::paragraph_plan::ParagraphCheckpoint,
) -> bool {
    use super::physical_memo::{lab_three_cut_receipt, LabThreeBoundary, LabThreeReplay};
    let Some(attempt) = l.lab_three_attempt else { return false };
    let fail = |reason| {
        lab_three_cut_receipt(reason, 2, 0);
        false
    };
    let (Some(pass), Some(route)) = (l.page_running_pass.as_ref(), route) else {
        return fail(3);
    };
    if l.is_sub_frame || l.pages.len() != 2 || !lab_sequence_boundary(parts, start, end)
        || !std::sync::Arc::ptr_eq(&pass.source, &route.plan)
        || route.anchor.is_some() || route.paint_only || !route.generated_edges.is_empty()
        || route.section != super::page_running_requests::TraceSection::Body
        || !l.link_capture.active.is_empty() || !l.pending_page_numbering.is_empty()
        || !l.floats_pending.is_empty() || !l.pending_footnote_bodies.is_empty()
        || !l.pending_footnote_frames.is_empty() || !l.pending_cell_tails.is_empty()
        || l.regions.cell.is_some() || !l.regions.backlog.is_empty() || l.regions.last.is_some()
        || l.page_columns.is_some() || l.current_column != 0 || l.flow_suffix_transaction_active
        || !l.regions.current.pending_fr.is_empty() || !l.layout_errors.is_empty()
        || !l.layout_warnings.is_empty()
        || !l.regions.current.current_items.iter().chain(l.regions.current.current_line.iter())
            .all(super::position_marker::is_marker)
    {
        return fail(4);
    }
    let region = &l.regions.current;
    let boundary = LabThreeBoundary {
        start, end, text_segment_start, owns_text_segment, packet: route.packet,
        line: paragraph.lab_three_line(), page: l.page_config.clone(),
        scalars: [
            region.width, region.height, region.cursor_x.0, region.cursor_y.0, region.line_start_x.0,
            l.prev_line_baseline, l.prev_block_equation_descent, l.prev_block_below_pending,
            l.last_block_descent_y.unwrap_or(f64::NAN), l.line_assumed_ascent.unwrap_or(f64::NAN),
            l.text_horizontal_offset.0,
        ].map(f64::to_bits),
        flags: [
            l.block_chain_active, l.prev_margin_is_parbreak, l.initial_baseline_pending,
            l.terminal_paragraph_line, l.page_has_unpainted_material,
        ],
    };
    let cut = pass.lab_three_cut(boundary);
    if let Some(previous) = &attempt.previous {
        match pass.lab_prepare_three_splice(&previous.product.trace, &cut) {
            Ok(splice) => {
                l.lab_three_replay = Some(LabThreeReplay {
                    previous: std::sync::Arc::clone(previous), splice,
                });
                return true;
            }
            Err(reason) => { fail(reason); }
        }
    } else {
        fail(attempt.refuse);
    }
    l.page_running_pass.as_mut().unwrap().trace.data.lab_three_cut = Some(cut);
    false
}
