use super::*;
use crate::entities::{
    elements::pdf_artifact::ArtifactKind,
    introspector::{Introspector, TagIntrospector},
    layout_types::{SemanticKind, SemanticPlacement},
};
use comemo::Track;
use page_running_requests::{Cursor, PlannedPass, TraceSection};

const END: &str = "\0tekt-lab-outline-end";

#[derive(Debug, Clone, PartialEq)]
pub(in crate::compiler) struct FlowEnd {
    page: usize,
    cursor: [u64; 7],
    placement: Option<(u64, u64)>,
    baseline: Option<u64>,
    flags: [bool; 4],
}

impl<M: FontMetrics, S: ImageSizer> Layouter<'_, M, S> {
    pub(super) fn lab_record_outline_end(&mut self) {
        if !self.lab_capture_outline || self.is_sub_frame {
            return;
        }
        let region = &self.regions.current;
        self.lab_outline_end = Some(FlowEnd {
            page: self.pages.len(),
            cursor: [
                region.cursor_x.0, region.cursor_y.0, region.line_start_x.0,
                self.prev_line_baseline, self.prev_block_equation_descent,
                self.prev_block_below_pending,
                self.last_block_descent_y.unwrap_or(f64::NAN),
            ].map(f64::to_bits),
            placement: self.flow_placement_end.map(|(a,b)| (a.to_bits(), b.to_bits())),
            baseline: self.line_assumed_ascent.map(f64::to_bits),
            flags: [self.block_chain_active, self.prev_margin_is_parbreak,
                    self.initial_baseline_pending, region.current_line.is_empty()],
        });
        self.regions.current.current_items.push(FrameItem::Semantic {
            kind: SemanticKind::Artifact(ArtifactKind::Other),
            placement: SemanticPlacement::Inline,
            alt: Some(END.into()),
            items: vec![],
        });
    }
}

fn end_marker(item: &FrameItem) -> bool {
    matches!(item, FrameItem::Semantic { alt: Some(alt), items, .. }
             if alt.as_str() == END && items.is_empty())
}

fn prefix_end(page: &Page) -> Option<usize> {
    let mut matches = page.items.iter().enumerate().filter(|(_,item)| end_marker(item));
    let (at, _) = matches.next()?;
    matches.next().is_none().then_some(at)
}

// This lays out only the path from document setup through the outline.
// In particular the root's sequence exits before the first body heading.
#[inline(never)]
fn layout_prefix<M: FontMetrics, S: ImageSizer>(
    body: &PageRunningBody,
    input: &TagIntrospector,
    metrics: M,
    sizer: S,
    sealed: &SealedPageRunning,
) -> Option<(PagedDocument, FlowEnd)> {
    let mut layouter = Layouter::new(metrics, sizer, 11.0, (input as &dyn Introspector).track());
    layouter.lab_capture_outline = true;
    layouter.lab_prefix_only = true;
    layouter.runtime.known_page_store = input.page_store.clone();
    layouter.locator = Locator::from_seed(body.plan.allocation_start);
    layouter.page_running_pass = Some(PlannedPass::new(body, Some(sealed)));
    layouter.layout_planned_cursor(Cursor { plan: body.plan.clone(), packet: body.plan.root,
        section: TraceSection::Body, paint_only: false, generated_edges: vec![], anchor: None,
    }, body.content(), false);
    let end = layouter.lab_outline_end.take()?;
    let document = layouter.finish_with_running().0;
    (document.pages.len() == 1 && end.page == 0
        && document.layout_errors.is_empty() && document.layout_warnings.is_empty()).then_some((document, end))
}

pub(super) fn splice<M: FontMetrics, S: ImageSizer>(
    mut retained: PageRunningDocument,
    body: &PageRunningBody,
    input: &TagIntrospector,
    metrics: M,
    sizer: S,
    sealed: &SealedPageRunning,
) -> Result<PageRunningDocument, &'static str> {
    let old_end = retained.trace.data.lab_outline_end.as_ref().ok_or("old prefix boundary")?;
    let (mut prefix, new_end) = layout_prefix(body, input, metrics, sizer, sealed).ok_or("prefix layout")?;
    if old_end != &new_end {
        return Err("flow at prefix boundary");
    }
    let document = &mut retained.document;
    if !document.layout_errors.is_empty() || !document.layout_warnings.is_empty()
        || !prefix.extracted_positions.lab_exact_subset(&document.extracted_positions)
        || !prefix.extracted_label_pages.iter().all(|(k,v)| document.extracted_label_pages.get(k)==Some(v))
        || !prefix.extracted_label_positions.iter().all(|(k,v)| document.extracted_label_positions.get(k)==Some(v))
    {
        return Err("prefix positions or diagnostics");
    }
    let old_page = document.pages.first_mut().ok_or("missing retained page")?;
    let new_page = prefix.pages.first_mut().ok_or("missing prefix page")?;
    let old_mark = prefix_end(old_page).ok_or("retained prefix marker")?;
    let new_mark = prefix_end(new_page).ok_or("new prefix marker")?;
    if new_mark + 1 != new_page.items.len()
        || old_page.width.to_bits() != new_page.width.to_bits()
        || old_page.height.to_bits() != new_page.height.to_bits()
    {
        return Err("prefix extent");
    }
    retained.trace.lab_rebind_prefix(body)?;
    old_page.items.splice(..=old_mark, new_page.items.drain(..new_mark));
    Ok(retained)
}
