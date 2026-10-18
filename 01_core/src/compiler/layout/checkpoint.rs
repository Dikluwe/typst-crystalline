//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/checkpoint.md
//! @prompt-hash 926ea8d4
//! @layer L1
//! @updated 2026-10-06

use super::*;

/// P1292-D v11 — checkpoint lógico do estado mutável capaz de tornar visível
/// uma cauda pós-marker ou de alterar seu fitting. Buffers que podem conter o
/// prefixo confirmado são representados somente pelos seus comprimentos; a
/// restauração trunca a cauda e nunca copia ou recompõe o prefixo.
pub(in crate::compiler) struct FlowSuffixCheckpoint {
    style: TextStyle,
    chain: StyleChain,
    text_tracking_state: text_tracking::State,
    page_config: PageConfig,
    pages_len: usize,
    current_region: RegionTailCheckpoint,
    cell_region: Option<RegionTailCheckpoint>,
    justify_opportunities_len: usize,
    initial_baseline_pending: bool,
    figure_progress: std::collections::HashMap<String, usize>,
    is_height_unconstrained: bool,
    is_sub_frame: bool,
    relative_region_size: Option<(f64, f64)>,
    sub_frame_origin_y: f64,
    flow_placement_end: Option<(f64, f64)>,
    flow_frame_extent: (f64, f64),
    last_sub_frame_flow_width: f64,
    block_place_region: Option<crate::entities::layout_types::Rect>,
    block_auto_width: bool,
    block_place_decorations: Vec<(usize, usize, usize)>,
    cell_origin_x: Option<f64>,
    cell_origin_y: Option<f64>,
    parent_bbox: Option<crate::entities::layout_types::Rect>,
    cell_align: Option<crate::entities::layout_types::Align2D>,
    locator_seed: LocatorSeed,
    current_location: Option<Location>,
    runtime: crate::entities::layouter_runtime_state::LayouterRuntimeState,
    pub(in crate::compiler::layout) floats_pending_len: usize,
    cursor_y_top_reserve: f64,
    cursor_y_bottom_reserve: f64,
    enum_counter: Option<u32>,
    par_line_counter: usize,
    par_line_page: Option<usize>,
    prev_block_below_pending: f64,
    fractional_block_height: Option<f64>,
    fractional_preflight_active: bool,
    fractional_flow_tail_pending: bool,
    block_chain_active: bool,
    prev_margin_is_parbreak: bool,
    prev_line_baseline: f64,
    terminal_paragraph_line: bool,
    empty_paragraph_weak_spacing: f64,
    last_flush_advance: f64,
    prev_block_equation_descent: f64,
    last_equation_descent: f64,
    last_block_descent_y: Option<f64>,
    line_inline_ascent: f64,
    line_inline_descent: f64,
    line_assumed_ascent: Option<f64>,
    position_captures_len: usize,
    link_capture_checkpoint: (usize, Vec<usize>),
    wrapped_inline_prefix_line_armed: bool,
    last_sub_frame_bottom: Option<f64>,
    last_sub_frame_width: f64,
    last_sub_frame_inline_extents: (f64, f64),
    pending_cell_tails_len: usize,
    pending_footnote_bodies_len: usize,
    decoration_lines_len: Option<usize>,
    smartquote_context: crate::compiler::lang::quotes::SmartQuoteContext,
    attachments_len: usize,
    artifact_marker_id: u64,
    bib_render_cache: Option<crate::compiler::layout::bib_csl::BibRenderCache>,
    bib_layout: Option<std::sync::Arc<super::bib_csl::plan::BibliographyPlan>>,
    pending_bib_prefix: Option<super::bibliography::PendingPrefix>,
    block_inline_region: Option<super::block::InlineRegion>,
    smallcaps: bool,
    paragraph_planning_suppressed: bool,
    bidi_scope_active: bool,
    bidi_cursor: Option<super::bidi_origin::Cursor>,
    text_bidi: Option<crate::entities::bidi_context::BidiSpan>,
    text_segment_chain: Option<StyleChain>,
    geometric_text_lines: bool,
    active_paragraph_dir: Option<crate::entities::dir::Dir>,
    pending_line_dir: Option<crate::entities::dir::Dir>,
    explicit_line_alignment: Option<crate::entities::layout_types::HAlign>,
    last_cited_key: Option<String>,
    previously_cited_keys: std::collections::HashSet<String>,
    last_was_loose_item: bool,
    last_seen_item_group: Option<ItemGroup>,
    parbreak_since_last_item: bool,
    column_mode: bool,
    column_origin_x: f64,
    column_width: f64,
    page_columns: Option<usize>,
    current_column: usize,
    segmented_par_line_column: Option<(usize, usize)>,
    segmented_par_line_overlay: Option<SegmentedParLineOverlay>,
    defer_segmented_par_line_patterns: bool,
    pending_segmented_par_line_patterns_len: usize,
    page_par_line_pattern_style: Option<TextStyle>,
    page_par_line_pattern_column_start: usize,
    column_page_items_len: usize,
    column_page_item_lens: Vec<usize>,
    column_x_offsets_len: usize,
    pending_page_numbering_len: usize,
    pending_par_line_numbering_len: usize,
    pending_equation_centering_len: usize,
    pending_equation_numbering_len: usize,
    pending_align_centering_len: usize,
    pending_align_v_centering_len: usize,
    pending_repeats_len: usize,
    pending_fr_natural_widths_len: usize,
    layout_warnings_len: usize,
    layout_errors_len: usize,
    page_run_boundary_empty: bool,
}

/// Geometria e fronteiras de cauda de uma região já existente. Os `FrameItem`
/// anteriores ficam no próprio buffer vivo e são preservados por `truncate`.
pub(in crate::compiler::layout) struct RegionTailCheckpoint {
    cursor_x: Pt,
    cursor_y: Pt,
    line_start_x: Pt,
    current_items_len: usize,
    current_line_len: usize,
    pending_fr_len: usize,
    width: f64,
    height: f64,
}

impl RegionTailCheckpoint {
    pub(in crate::compiler::layout) fn capture(
        region: &crate::entities::region::Region,
    ) -> Self {
        Self {
            cursor_x: region.cursor_x,
            cursor_y: region.cursor_y,
            line_start_x: region.line_start_x,
            current_items_len: region.current_items.len(),
            current_line_len: region.current_line.len(),
            pending_fr_len: region.pending_fr.len(),
            width: region.width,
            height: region.height,
        }
    }

    pub(in crate::compiler::layout) fn restore(
        self,
        region: &mut crate::entities::region::Region,
    ) {
        region.cursor_x = self.cursor_x;
        region.cursor_y = self.cursor_y;
        region.line_start_x = self.line_start_x;
        region.current_items.truncate(self.current_items_len);
        region.current_line.truncate(self.current_line_len);
        region.pending_fr.truncate(self.pending_fr_len);
        region.width = self.width;
        region.height = self.height;
    }
}

impl<'a, M: FontMetrics, S: ImageSizer> Layouter<'a, M, S> {
    /// Captura estado lógico e comprimentos de cauda na fronteira pós-marker.
    /// Campos de capacidade imutável (`metrics`, `sizer`, `introspector` e
    /// callbacks) não participam; prefixos de buffers permanecem no estado
    /// vivo e não são copiados.
    pub(in crate::compiler::layout) fn capture_flow_suffix_checkpoint(
        &self,
    ) -> FlowSuffixCheckpoint {
        FlowSuffixCheckpoint {
            style: self.style.clone(),
            chain: self.chain.clone(),
            text_tracking_state: self.text_tracking_state.clone(),
            page_config: self.page_config.clone(),
            pages_len: self.pages.len(),
            current_region: RegionTailCheckpoint::capture(&self.regions.current),
            cell_region: self.regions.cell.as_ref().map(RegionTailCheckpoint::capture),
            justify_opportunities_len: self.justify_opportunities.len(),
            initial_baseline_pending: self.initial_baseline_pending,
            figure_progress: self.figure_progress.clone(),
            is_height_unconstrained: self.is_height_unconstrained,
            is_sub_frame: self.is_sub_frame,
            relative_region_size: self.relative_region_size,
            sub_frame_origin_y: self.sub_frame_origin_y,
            flow_placement_end: self.flow_placement_end,
            flow_frame_extent: self.flow_frame_extent,
            last_sub_frame_flow_width: self.last_sub_frame_flow_width,
            block_place_region: self.block_place_region,
            block_auto_width: self.block_auto_width,
            block_place_decorations: self.block_place_decorations.clone(),
            cell_origin_x: self.cell_origin_x,
            cell_origin_y: self.cell_origin_y,
            parent_bbox: self.parent_bbox,
            cell_align: self.cell_align,
            current_location: self.current_location,
            runtime: self.runtime.clone(),
            locator_seed: self.locator.checkpoint(),
            floats_pending_len: self.floats_pending.len(),
            cursor_y_top_reserve: self.cursor_y_top_reserve,
            cursor_y_bottom_reserve: self.cursor_y_bottom_reserve,
            enum_counter: self.enum_counter,
            par_line_counter: self.par_line_counter,
            par_line_page: self.par_line_page,
            prev_block_below_pending: self.prev_block_below_pending,
            fractional_block_height: self.fractional_block_height,
            fractional_preflight_active: self.fractional_preflight_active,
            fractional_flow_tail_pending: self.fractional_flow_tail_pending,
            block_chain_active: self.block_chain_active,
            prev_margin_is_parbreak: self.prev_margin_is_parbreak,
            prev_line_baseline: self.prev_line_baseline,
            terminal_paragraph_line: self.terminal_paragraph_line,
            empty_paragraph_weak_spacing: self.empty_paragraph_weak_spacing,
            last_flush_advance: self.last_flush_advance,
            prev_block_equation_descent: self.prev_block_equation_descent,
            last_equation_descent: self.last_equation_descent,
            last_block_descent_y: self.last_block_descent_y,
            line_inline_ascent: self.line_inline_ascent,
            line_inline_descent: self.line_inline_descent,
            line_assumed_ascent: self.line_assumed_ascent,
            position_captures_len: self.position_captures.len(),
            link_capture_checkpoint: self.link_capture.checkpoint(),
            wrapped_inline_prefix_line_armed: self.wrapped_inline_prefix_line_armed,
            last_sub_frame_bottom: self.last_sub_frame_bottom,
            last_sub_frame_width: self.last_sub_frame_width,
            last_sub_frame_inline_extents: self.last_sub_frame_inline_extents,
            pending_cell_tails_len: self.pending_cell_tails.len(),
            pending_footnote_bodies_len: self.pending_footnote_bodies.len(),
            decoration_lines_len: self.decoration_lines_collector.as_ref().map(Vec::len),
            smartquote_context: self.smartquote_context.clone(),
            attachments_len: self.attachments.len(),
            artifact_marker_id: self.artifact_marker_id,
            bib_render_cache: self.bib_render_cache.clone(),
            bib_layout: self.bib_layout.clone(),
            pending_bib_prefix: self.pending_bib_prefix.clone(),
            block_inline_region: self.block_inline_region,
            smallcaps: self.smallcaps,
            paragraph_planning_suppressed: self.paragraph_planning_suppressed,
            bidi_scope_active: self.bidi_scope_active,
            bidi_cursor: self.bidi_cursor.clone(),
            text_bidi: self.text_bidi.clone(),
            text_segment_chain: self.text_segment_chain.clone(),
            geometric_text_lines: self.geometric_text_lines,
            active_paragraph_dir: self.active_paragraph_dir,
            pending_line_dir: self.pending_line_dir,
            explicit_line_alignment: self.explicit_line_alignment,
            last_cited_key: self.last_cited_key.clone(),
            previously_cited_keys: self.previously_cited_keys.clone(),
            last_was_loose_item: self.last_was_loose_item,
            last_seen_item_group: self.last_seen_item_group,
            parbreak_since_last_item: self.parbreak_since_last_item,
            column_mode: self.column_mode,
            column_origin_x: self.column_origin_x,
            column_width: self.column_width,
            page_columns: self.page_columns,
            current_column: self.current_column,
            segmented_par_line_column: self.segmented_par_line_column,
            segmented_par_line_overlay: self.segmented_par_line_overlay,
            defer_segmented_par_line_patterns: self.defer_segmented_par_line_patterns,
            pending_segmented_par_line_patterns_len: self
                .pending_segmented_par_line_patterns
                .len(),
            page_par_line_pattern_style: self.page_par_line_pattern_style.clone(),
            page_par_line_pattern_column_start: self.page_par_line_pattern_column_start,
            column_page_items_len: self.column_page_items.len(),
            column_page_item_lens: self.column_page_items.iter().map(Vec::len).collect(),
            column_x_offsets_len: self.column_x_offsets.len(),
            pending_page_numbering_len: self.pending_page_numbering.len(),
            pending_par_line_numbering_len: self.pending_par_line_numbering.len(),
            pending_equation_centering_len: self.pending_equation_centering.len(),
            pending_equation_numbering_len: self.pending_equation_numbering.len(),
            pending_align_centering_len: self.pending_align_centering.len(),
            pending_align_v_centering_len: self.pending_align_v_centering.len(),
            pending_repeats_len: self.pending_repeats.len(),
            pending_fr_natural_widths_len: self.pending_fr_natural_widths.len(),
            layout_warnings_len: self.layout_warnings.len(),
            layout_errors_len: self.layout_errors.len(),
            page_run_boundary_empty: self.page_run_boundary_empty,
        }
    }

    /// Restaura atomicamente a fronteira. `Locator` é reconstruído a partir
    /// da última Location confirmada porque ele não é clonável por design;
    /// assim o replay consome as mesmas ocorrências sem saltos no contador.
    pub(in crate::compiler::layout) fn restore_flow_suffix_checkpoint(
        &mut self,
        checkpoint: FlowSuffixCheckpoint,
    ) {
        self.style = checkpoint.style;
        self.chain = checkpoint.chain;
        self.text_tracking_state = checkpoint.text_tracking_state;
        self.page_config = checkpoint.page_config;
        self.pages.truncate(checkpoint.pages_len);
        checkpoint.current_region.restore(&mut self.regions.current);
        match (checkpoint.cell_region, self.regions.cell.as_mut()) {
            (Some(region_checkpoint), Some(region)) => region_checkpoint.restore(region),
            (None, _) => self.regions.cell = None,
            (Some(_), None) => {
                debug_assert!(false, "região de célula saiu durante transação rejeitada");
            }
        }
        self.justify_opportunities
            .truncate(checkpoint.justify_opportunities_len);
        self.initial_baseline_pending = checkpoint.initial_baseline_pending;
        self.figure_progress = checkpoint.figure_progress;
        self.is_height_unconstrained = checkpoint.is_height_unconstrained;
        self.is_sub_frame = checkpoint.is_sub_frame;
        self.relative_region_size = checkpoint.relative_region_size;
        self.sub_frame_origin_y = checkpoint.sub_frame_origin_y;
        self.flow_placement_end = checkpoint.flow_placement_end;
        self.flow_frame_extent = checkpoint.flow_frame_extent;
        self.last_sub_frame_flow_width = checkpoint.last_sub_frame_flow_width;
        self.block_place_region = checkpoint.block_place_region;
        self.block_auto_width = checkpoint.block_auto_width;
        self.block_place_decorations = checkpoint.block_place_decorations;
        self.cell_origin_x = checkpoint.cell_origin_x;
        self.cell_origin_y = checkpoint.cell_origin_y;
        self.parent_bbox = checkpoint.parent_bbox;
        self.cell_align = checkpoint.cell_align;
        self.current_location = checkpoint.current_location;
        self.locator = Locator::from_seed(checkpoint.locator_seed);
        self.runtime = checkpoint.runtime;
        self.floats_pending.truncate(checkpoint.floats_pending_len);
        self.cursor_y_top_reserve = checkpoint.cursor_y_top_reserve;
        self.cursor_y_bottom_reserve = checkpoint.cursor_y_bottom_reserve;
        self.enum_counter = checkpoint.enum_counter;
        self.par_line_counter = checkpoint.par_line_counter;
        self.par_line_page = checkpoint.par_line_page;
        self.prev_block_below_pending = checkpoint.prev_block_below_pending;
        self.fractional_block_height = checkpoint.fractional_block_height;
        self.fractional_preflight_active = checkpoint.fractional_preflight_active;
        self.fractional_flow_tail_pending = checkpoint.fractional_flow_tail_pending;
        self.block_chain_active = checkpoint.block_chain_active;
        self.prev_margin_is_parbreak = checkpoint.prev_margin_is_parbreak;
        self.prev_line_baseline = checkpoint.prev_line_baseline;
        self.terminal_paragraph_line = checkpoint.terminal_paragraph_line;
        self.empty_paragraph_weak_spacing = checkpoint.empty_paragraph_weak_spacing;
        self.last_flush_advance = checkpoint.last_flush_advance;
        self.prev_block_equation_descent = checkpoint.prev_block_equation_descent;
        self.last_equation_descent = checkpoint.last_equation_descent;
        self.last_block_descent_y = checkpoint.last_block_descent_y;
        self.line_inline_ascent = checkpoint.line_inline_ascent;
        self.line_inline_descent = checkpoint.line_inline_descent;
        self.line_assumed_ascent = checkpoint.line_assumed_ascent;
        self.position_captures.truncate(checkpoint.position_captures_len);
        self.link_capture.restore(checkpoint.link_capture_checkpoint);
        self.wrapped_inline_prefix_line_armed =
            checkpoint.wrapped_inline_prefix_line_armed;
        self.last_sub_frame_bottom = checkpoint.last_sub_frame_bottom;
        self.last_sub_frame_width = checkpoint.last_sub_frame_width;
        self.last_sub_frame_inline_extents = checkpoint.last_sub_frame_inline_extents;
        self.pending_cell_tails.truncate(checkpoint.pending_cell_tails_len);
        self.pending_footnote_bodies
            .truncate(checkpoint.pending_footnote_bodies_len);
        match (checkpoint.decoration_lines_len, self.decoration_lines_collector.as_mut())
        {
            (Some(len), Some(lines)) => lines.truncate(len),
            (None, _) => self.decoration_lines_collector = None,
            (Some(_), None) => {
                debug_assert!(false, "collector saiu durante transação rejeitada");
            }
        }
        self.smartquote_context = checkpoint.smartquote_context;
        self.attachments.truncate(checkpoint.attachments_len);
        self.artifact_marker_id = checkpoint.artifact_marker_id;
        self.bib_render_cache = checkpoint.bib_render_cache;
        self.bib_layout = checkpoint.bib_layout;
        self.pending_bib_prefix = checkpoint.pending_bib_prefix;
        self.block_inline_region = checkpoint.block_inline_region;
        self.smallcaps = checkpoint.smallcaps;
        self.paragraph_planning_suppressed = checkpoint.paragraph_planning_suppressed;
        self.bidi_scope_active = checkpoint.bidi_scope_active;
        self.bidi_cursor = checkpoint.bidi_cursor;
        self.text_bidi = checkpoint.text_bidi;
        self.text_segment_chain = checkpoint.text_segment_chain;
        self.geometric_text_lines = checkpoint.geometric_text_lines;
        self.active_paragraph_dir = checkpoint.active_paragraph_dir;
        self.pending_line_dir = checkpoint.pending_line_dir;
        self.explicit_line_alignment = checkpoint.explicit_line_alignment;
        self.last_cited_key = checkpoint.last_cited_key;
        self.previously_cited_keys = checkpoint.previously_cited_keys;
        self.last_was_loose_item = checkpoint.last_was_loose_item;
        self.last_seen_item_group = checkpoint.last_seen_item_group;
        self.parbreak_since_last_item = checkpoint.parbreak_since_last_item;
        self.column_mode = checkpoint.column_mode;
        self.column_origin_x = checkpoint.column_origin_x;
        self.column_width = checkpoint.column_width;
        self.page_columns = checkpoint.page_columns;
        self.current_column = checkpoint.current_column;
        self.segmented_par_line_column = checkpoint.segmented_par_line_column;
        self.segmented_par_line_overlay = checkpoint.segmented_par_line_overlay;
        self.defer_segmented_par_line_patterns =
            checkpoint.defer_segmented_par_line_patterns;
        self.pending_segmented_par_line_patterns
            .truncate(checkpoint.pending_segmented_par_line_patterns_len);
        self.page_par_line_pattern_style = checkpoint.page_par_line_pattern_style;
        self.page_par_line_pattern_column_start =
            checkpoint.page_par_line_pattern_column_start;
        self.column_page_items.truncate(checkpoint.column_page_items_len);
        for (items, len) in self
            .column_page_items
            .iter_mut()
            .zip(checkpoint.column_page_item_lens)
        {
            items.truncate(len);
        }
        self.column_x_offsets.truncate(checkpoint.column_x_offsets_len);
        self.pending_page_numbering
            .truncate(checkpoint.pending_page_numbering_len);
        self.pending_par_line_numbering
            .truncate(checkpoint.pending_par_line_numbering_len);
        self.pending_equation_centering
            .truncate(checkpoint.pending_equation_centering_len);
        self.pending_equation_numbering
            .truncate(checkpoint.pending_equation_numbering_len);
        self.pending_align_centering
            .truncate(checkpoint.pending_align_centering_len);
        self.pending_align_v_centering
            .truncate(checkpoint.pending_align_v_centering_len);
        self.pending_repeats.truncate(checkpoint.pending_repeats_len);
        self.pending_fr_natural_widths
            .truncate(checkpoint.pending_fr_natural_widths_len);
        self.layout_warnings.truncate(checkpoint.layout_warnings_len);
        self.layout_errors.truncate(checkpoint.layout_errors_len);
        self.page_run_boundary_empty = checkpoint.page_run_boundary_empty;
        self.flow_suffix_checkpoint_pending = false;
        self.flow_suffix_transaction_active = false;
        self.flow_suffix_transaction_rejected = false;
        self.flow_suffix_checkpoint = None;
        self.flow_suffix_replay.clear();
        self.flow_suffix_dispatch_depth = 0;
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn modulo_compila_e_carrega() {}
}
