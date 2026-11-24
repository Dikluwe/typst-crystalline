// A standalone restart is permitted only on the blank first region with
// at most a few source-position carriers and no live line or deferred content.
pub(in crate::compiler) struct OutlineCheckpoint {
    flow: FlowSuffixCheckpoint,
    cursor: Cursor,
    body_only: bool,
    position_captures: Vec<position_marker::Target>,
    regions: crate::entities::region::Regions,
    document_language: Option<crate::entities::lang::Lang>,
    locations_reserved: bool,
    line_optical_overhang: Option<bool>,
}

impl<M: FontMetrics, S: ImageSizer> Layouter<'_, M, S> {
    pub(super) fn lab_record_outline_checkpoint(&mut self, cursor: &Cursor, body_only: bool) {
        // One eligible outline is required by the closed-source guard.
        self.lab_capture_checkpoint = false;
        if self.is_sub_frame || cursor.paint_only || cursor.anchor.is_some()
            || cursor.section != TraceSection::Body || !cursor.generated_edges.is_empty()
            || !self.pages.is_empty() || self.regions.cell.is_some()
            || !self.regions.backlog.is_empty() || self.regions.last.is_some()
            || self.regions.current.current_items.len() + self.regions.current.current_line.len() > 32
            || !self.regions.current.current_items.iter()
                .chain(&self.regions.current.current_line).all(position_marker::is_marker)
            || !self.regions.current.pending_fr.is_empty()
            || self.position_captures.len() > 32 || self.link_capture.checkpoint() != (0, Vec::new())
            || !self.floats_pending.is_empty() || !self.pending_footnote_bodies.is_empty()
            || !self.pending_cell_tails.is_empty() || self.page_columns.is_some()
            || self.flow_suffix_transaction_active || self.flow_suffix_checkpoint_pending
            || self.decoration_lines_collector.is_some() || !self.justify_opportunities.is_empty()
            || self.page_running_pass.as_ref().is_none_or(|pass|
                pass.invalid || !Arc::ptr_eq(&pass.source, &cursor.plan))
            || !matches!(cursor.plan.node(cursor.packet).map(|node| &node.content), Some(Content::Outline(_)))
        {
            return;
        }

        self.lab_outline_checkpoint = Some(Box::new(OutlineCheckpoint {
            flow: self.capture_flow_suffix_checkpoint(),
            cursor: cursor.clone(),
            body_only,
            position_captures: self.position_captures.clone(),
            regions: self.regions.clone(),
            document_language: self.document_language,
            locations_reserved: self.locations_reserved,
            line_optical_overhang: self.line_optical_overhang,
        }));
    }
}

#[inline(never)]
fn layout_from_checkpoint<M: FontMetrics, S: ImageSizer>(
    checkpoint: Box<OutlineCheckpoint>,
    body: &PageRunningBody,
    input: &TagIntrospector,
    metrics: M,
    sizer: S,
    sealed: &SealedPageRunning,
) -> Option<(PagedDocument, FlowEnd)> {
    let OutlineCheckpoint {
        flow, mut cursor, body_only, position_captures, regions, document_language,
        locations_reserved, line_optical_overhang,
    } = *checkpoint;
    let old = cursor.plan.node(cursor.packet)?;
    let new = body.plan.node(cursor.packet)?;
    if cursor.packet == body.plan.root
        || old.span != new.span || old.location != new.location || old.environment != new.environment
        || !matches!(&new.content, Content::Outline(_))
    {
        return None;
    }
    cursor.plan = body.plan.clone();

    let mut layouter = Layouter::new(metrics, sizer, 11.0, (input as &dyn Introspector).track());
    layouter.regions = regions;
    layouter.position_captures = position_captures;
    layouter.restore_measurement_checkpoint(flow);
    layouter.document_language = document_language;
    layouter.locations_reserved = locations_reserved;
    layouter.line_optical_overhang = line_optical_overhang;
    layouter.lab_capture_outline = true;
    layouter.runtime.known_page_store = input.page_store.clone();
    // The scratch trace is discarded. The retained trace is rebound and
    // validated independently after the replacement.
    layouter.page_running_pass.as_mut()?.lab_rebase_for_outline(body, sealed)?;
    layouter.layout_planned_cursor(cursor, &new.content, body_only);
    let end = layouter.lab_outline_end.take()?;
    let document = layouter.finish_with_running().0;
    (document.pages.len() == 1 && end.page == 0
        && document.layout_errors.is_empty() && document.layout_warnings.is_empty()).then_some((document, end))
}

