//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/position_marker.md
//! @prompt-hash 5cae35f5
//! @layer L1

use std::{collections::HashSet, num::NonZeroUsize};

use super::{FontMetrics, ImageSizer, Layouter};
use crate::entities::{
    elements::pdf_artifact::ArtifactKind,
    geometry::{FillRule, ShapeKind},
    label::Label,
    layout_types::{
        FrameItem, Point, Pt, SemanticKind, SemanticPlacement, TransformMatrix,
    },
    location::Location,
    position::Position,
};

const PREFIX: &str = "\0tekt-position:";

/// The index in the append-only capture list is also the logical write order.
#[derive(Clone, Debug)]
pub(super) enum Target {
    Location(Location),
    Label(Label),
}

fn marker(index: usize, point: Point) -> FrameItem {
    FrameItem::Semantic {
        kind: SemanticKind::Artifact(ArtifactKind::Other),
        placement: SemanticPlacement::Inline,
        alt: Some(format!("{PREFIX}{index}").into()),
        items: vec![FrameItem::Shape {
            pos: point,
            kind: ShapeKind::Rect,
            width: 0.0,
            height: 0.0,
            fill: None,
            stroke: None,
            fill_rule: FillRule::NonZero,
            parent_bbox_at_emit: None,
        }],
    }
}

fn decode(item: &FrameItem) -> Option<(usize, Point)> {
    let FrameItem::Semantic {
        kind: SemanticKind::Artifact(ArtifactKind::Other),
        placement: SemanticPlacement::Inline,
        alt: Some(alt),
        items,
    } = item
    else {
        return None;
    };
    let [FrameItem::Shape {
        pos,
        kind: ShapeKind::Rect,
        width,
        height,
        fill: None,
        stroke: None,
        ..
    }] = items.as_slice()
    else {
        return None;
    };
    if *width != 0.0 || *height != 0.0 {
        return None;
    }
    Some((alt.strip_prefix(PREFIX)?.parse().ok()?, *pos))
}

pub(super) fn is_marker(item: &FrameItem) -> bool {
    decode(item).is_some()
}

/// Strip carriers while retaining the first visual occurrence of each capture.
fn project(
    items: &mut Vec<FrameItem>,
    transform: TransformMatrix,
    page: NonZeroUsize,
    positions: &mut [Option<Position>],
) {
    let mut retained = Vec::with_capacity(items.len());
    for mut item in std::mem::take(items) {
        if let Some((index, point)) = decode(&item) {
            if let Some(slot) = positions.get_mut(index) {
                if slot.is_none() {
                    let (x, y) = transform.apply(point.x.0, point.y.0);
                    *slot = Some(Position { page, point: Point { x: Pt(x), y: Pt(y) } });
                }
            }
            continue;
        }
        match &mut item {
            FrameItem::Group { pos, matrix, items, .. } => {
                let nested = transform
                    .concat(&TransformMatrix::translate(pos.x.0, pos.y.0))
                    .concat(matrix);
                project(items, nested, page, positions);
            }
            FrameItem::Semantic { items, .. }
            | FrameItem::Link { items, .. }
            | FrameItem::TextReplacement { items, .. } => {
                project(items, transform, page, positions);
            }
            FrameItem::Text { .. }
            | FrameItem::TextShaped { .. }
            | FrameItem::TextSeparator { .. }
            | FrameItem::Glyph { .. }
            | FrameItem::Line { .. }
            | FrameItem::Shape { .. }
            | FrameItem::Image { .. } => {}
        }
        retained.push(item);
    }
    *items = retained;
}

impl<M: FontMetrics, S: ImageSizer> Layouter<'_, M, S> {
    fn capture_position(&mut self, target: Target, point: Point) {
        // The HTML island already carries its own contextual observations.
        if self.capture_html_context_markers {
            return;
        }
        // Labels are recorded after their body. Once its line is committed,
        // keep the anchor with that body instead of the next admitted line.
        let committed_label = matches!(&target, Target::Label(_))
            && !super::html_frame::items_have_causal_line(
                &self.regions.current.current_line,
            )
            && super::html_frame::items_have_causal_line(
                &self.regions.current.current_items,
            );
        let index = self.position_captures.len();
        self.position_captures.push(target);
        if committed_label {
            self.regions.current.current_items.push(marker(index, point));
        } else {
            self.regions.current.current_line.push(marker(index, point));
        }
    }

    pub(super) fn record_location_position(
        &mut self,
        location: Location,
        position: Position,
        only_if_absent: bool,
    ) {
        if only_if_absent && self.runtime.positions.contains_key(&location) {
            return;
        }
        self.runtime.positions.insert(location, position);
        self.capture_position(Target::Location(location), position.point);
    }

    pub(super) fn record_label_position(
        &mut self,
        label: Label,
        page: usize,
        point: Point,
    ) {
        self.runtime.label_pages.insert(label.clone(), page);
        self.runtime.label_positions.insert(label.clone(), point);
        self.capture_position(Target::Label(label), point);
    }

    pub(super) fn resolve_position_markers(&mut self) {
        let mut projected = vec![None; self.position_captures.len()];
        for (index, page) in self.pages.iter_mut().enumerate() {
            let physical_page = NonZeroUsize::new(index + 1).unwrap();
            for layer in [&mut page.background, &mut page.items, &mut page.foreground] {
                project(
                    layer,
                    TransformMatrix::identity(),
                    physical_page,
                    &mut projected,
                );
            }
        }
        // Resolve the latest logical write, even if its visual carrier was dropped.
        let mut locations = HashSet::new();
        let mut labels = HashSet::new();
        for (target, position) in self.position_captures.iter().zip(projected).rev() {
            match target {
                Target::Location(location) => {
                    if locations.insert(*location) {
                        if let Some(position) = position {
                            self.runtime.positions.insert(*location, position);
                        }
                    }
                }
                Target::Label(label) => {
                    if labels.insert(label) {
                        if let Some(position) = position {
                            self.runtime
                                .label_pages
                                .insert(label.clone(), position.page.get());
                            self.runtime
                                .label_positions
                                .insert(label.clone(), position.point);
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        compiler::layout::FixedMetrics,
        entities::{
            content::Content,
            image_sizer::NullImageSizer,
            introspector::{Introspector, TagIntrospector},
            layout_types::Page,
            page_canvas::PageFill,
        },
    };
    use comemo::Track;

    fn point(x: f64, y: f64) -> Point {
        Point { x: Pt(x), y: Pt(y) }
    }
    fn position(x: f64, y: f64) -> Position {
        Position {
            page: NonZeroUsize::new(1).unwrap(),
            point: point(x, y),
        }
    }
    fn group(
        x: f64,
        y: f64,
        matrix: TransformMatrix,
        items: Vec<FrameItem>,
    ) -> FrameItem {
        FrameItem::Group {
            paint_root: false,
            pos: point(x, y),
            matrix,
            clip_mask: None,
            inner_width: 40.0,
            inner_height: 40.0,
            items,
        }
    }
    fn page(items: Vec<FrameItem>) -> Page {
        Page {
            width: 100.0,
            height: 100.0,
            numbering: None,
            logical_number: 1,
            supplement: Content::Empty,
            bleed: Default::default(),
            fill: PageFill::None,
            background: vec![],
            foreground: vec![],
            items,
        }
    }

    #[test]
    fn projection_composes_group_matrices_and_keeps_the_physical_page() {
        let mut items = vec![group(
            10.0,
            20.0,
            TransformMatrix::translate(3.0, 4.0)
                .concat(&TransformMatrix::scale(2.0, 3.0)),
            vec![FrameItem::TextReplacement {
                text: "".into(),
                items: vec![marker(0, point(1.0, 2.0))],
            }],
        )];
        let mut positions = vec![None];
        project(
            &mut items,
            TransformMatrix::identity(),
            NonZeroUsize::new(2).unwrap(),
            &mut positions,
        );
        let observed = positions[0].unwrap();
        assert_eq!(observed.page.get(), 2);
        assert_eq!(observed.point, point(15.0, 30.0));
        let FrameItem::Group { items, .. } = &items[0] else { panic!("group retained") };
        let FrameItem::TextReplacement { items, .. } = &items[0] else {
            panic!("wrapper retained")
        };
        assert!(items.is_empty(), "private carrier removed");
    }

    #[test]
    fn latest_logical_write_wins_even_when_painting_order_is_reversed() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        let loc = Location::from_raw(42);
        l.record_location_position(loc, position(1.0, 2.0), false);
        let first = l.regions.current.current_line.pop().unwrap();
        l.record_location_position(loc, position(3.0, 4.0), false);
        let second = l.regions.current.current_line.pop().unwrap();
        l.record_location_position(loc, position(999.0, 999.0), true);
        assert!(
            l.regions.current.current_line.is_empty(),
            "or_insert must not create a write"
        );
        l.pages.push(page(vec![
            group(20.0, 0.0, TransformMatrix::identity(), vec![second]),
            group(10.0, 0.0, TransformMatrix::identity(), vec![first]),
        ]));
        l.resolve_position_markers();
        assert_eq!(l.runtime.positions[&loc].point, point(23.0, 4.0));
        assert_eq!(l.runtime.positions.len(), 1);
    }

    #[test]
    fn visual_copies_do_not_replace_the_original_observation() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        let loc = Location::from_raw(42);
        l.record_location_position(loc, position(1.0, 2.0), false);
        let original = l.regions.current.current_line.pop().unwrap();
        l.pages.push(page(vec![
            group(10.0, 0.0, TransformMatrix::identity(), vec![original.clone()]),
            group(30.0, 0.0, TransformMatrix::identity(), vec![original]),
        ]));
        l.resolve_position_markers();
        assert_eq!(l.runtime.positions[&loc].point, point(11.0, 2.0));
        assert_eq!(l.runtime.positions.len(), 1);
    }

    #[test]
    fn rollback_discards_aborted_captures_before_indices_are_reused() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        let loc = Location::from_raw(42);
        l.record_location_position(loc, position(1.0, 2.0), false);
        let checkpoint = l.capture_flow_suffix_checkpoint();
        l.record_location_position(loc, position(900.0, 900.0), false);
        l.restore_flow_suffix_checkpoint(checkpoint);
        l.record_location_position(loc, position(3.0, 4.0), false);
        let markers = std::mem::take(&mut l.regions.current.current_line);
        l.pages
            .push(page(vec![group(20.0, 0.0, TransformMatrix::identity(), markers)]));
        l.resolve_position_markers();
        assert_eq!(l.runtime.positions[&loc].point, point(23.0, 4.0));
        assert_eq!(l.position_captures.len(), 2);
    }

    #[test]
    fn a_named_artifact_with_material_geometry_is_not_a_private_carrier() {
        let mut item = marker(0, point(1.0, 2.0));
        let FrameItem::Semantic { items, .. } = &mut item else { unreachable!() };
        let FrameItem::Shape { width, .. } = &mut items[0] else { unreachable!() };
        *width = 5.0;
        assert!(!is_marker(&item));
        let mut items = vec![item];
        project(
            &mut items,
            TransformMatrix::identity(),
            NonZeroUsize::new(1).unwrap(),
            &mut [],
        );
        assert_eq!(items.len(), 1, "legitimate artifact preserved");
    }
    #[test]
    fn discarded_latest_capture_keeps_runtime_fallback_for_locations_and_labels() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        let loc = Location::from_raw(42);
        let label = Label("anchor".into());
        l.record_location_position(loc, position(1.0, 2.0), false);
        l.record_label_position(label.clone(), 1, point(1.0, 2.0));
        let originals = std::mem::take(&mut l.regions.current.current_line);
        l.record_location_position(loc, position(30.0, 40.0), false);
        l.record_label_position(label.clone(), 3, point(30.0, 40.0));
        l.regions.current.current_line.clear();
        l.pages.push(page(vec![group(
            10.0,
            20.0,
            TransformMatrix::identity(),
            originals,
        )]));
        l.resolve_position_markers();
        assert_eq!(l.runtime.positions[&loc].point, point(30.0, 40.0));
        assert_eq!(l.runtime.label_positions[&label], point(30.0, 40.0));
        assert_eq!(l.runtime.label_pages[&label], 3);
    }
    #[test]
    fn explicit_pagebreak_keeps_positions_after_a_closed_line_on_previous_page() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.layout_content(&Content::text("A"));
        l.flush_line();
        let loc = Location::from_raw(42);
        let label = Label("anchor".into());
        l.record_location_position(loc, position(10.0, 20.0), false);
        l.record_label_position(label.clone(), 1, point(10.0, 20.0));
        l.layout_content(&Content::Pagebreak(std::sync::Arc::new(
            crate::entities::elements::pagebreak::PagebreakElem {
                weak: false,
                weak_explicit: false,
                to: None,
            },
        )));
        l.layout_content(&Content::text("B"));
        let doc = l.finish();
        assert_eq!(doc.pages.len(), 2);
        assert_eq!(doc.extracted_positions.position_of(loc).unwrap().page.get(), 1);
        assert_eq!(doc.extracted_label_pages[&label], 1);
        assert_eq!(doc.extracted_label_positions[&label], point(10.0, 20.0));
    }
    fn boundary_document(
        contents: &[Content],
    ) -> crate::entities::layout_types::PagedDocument {
        use crate::entities::layout_types::{PageMarginSpec, PageMargins};
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.page_config.width = 160.0;
        l.page_config.height = 40.0;
        l.page_config.margin = PageMargins::uniform(10.0);
        l.page_config.margin_spec = PageMarginSpec::uniform(10.0);
        l.regions.current.width = 160.0;
        l.regions.current.height = 40.0;
        l.regions.current.cursor_x = Pt(10.0);
        l.regions.current.line_start_x = Pt(10.0);
        l.regions.current.cursor_y = Pt(10.0);
        for body in contents {
            l.layout_content(body);
        }
        l.finish()
    }

    #[test]
    fn boundary_label_after_closed_heading_stays_with_heading_on_automatic_break() {
        let heading = Content::heading(1, Content::text("TITLE"));
        let doc = boundary_document(&[
            Content::label("anchor", heading.clone()),
            Content::text("BODY"),
        ]);
        let control = boundary_document(&[heading, Content::text("BODY")]);
        assert_eq!(doc.pages.len(), 2);
        assert_eq!(
            doc.pages.iter().map(Page::plain_text).collect::<Vec<_>>(),
            control.pages.iter().map(Page::plain_text).collect::<Vec<_>>()
        );
        assert!(doc.pages[0].plain_text().contains("TITLE"));
        assert!(doc.pages[1].plain_text().contains("BODY"));
        assert_eq!(doc.extracted_label_pages[&Label("anchor".into())], 1);
        assert_eq!(
            doc.extracted_label_positions[&Label("anchor".into())],
            point(10.0, 10.0)
        );
    }

    #[test]
    fn boundary_label_on_pending_body_follows_its_automatic_admission() {
        let heading = Content::heading(1, Content::text("TITLE"));
        let doc = boundary_document(&[
            heading.clone(),
            Content::label("anchor", Content::text("BODY")),
        ]);
        let control = boundary_document(&[heading, Content::text("BODY")]);
        assert_eq!(doc.pages.len(), 2);
        assert_eq!(
            doc.pages.iter().map(Page::plain_text).collect::<Vec<_>>(),
            control.pages.iter().map(Page::plain_text).collect::<Vec<_>>()
        );
        assert_eq!(doc.extracted_label_pages[&Label("anchor".into())], 2);
    }

    #[test]
    fn boundary_inline_label_keeps_pending_line_and_neighbors() {
        let doc = boundary_document(&[
            Content::text("P"),
            Content::label("anchor", Content::text("A")),
            Content::text("Q"),
        ]);
        let control = boundary_document(&[
            Content::text("P"),
            Content::text("A"),
            Content::text("Q"),
        ]);
        assert_eq!(doc.pages.len(), 1);
        assert_eq!(doc.pages[0].plain_text(), control.pages[0].plain_text());
        assert_eq!(doc.extracted_label_pages[&Label("anchor".into())], 1);
        assert!(doc.extracted_label_positions[&Label("anchor".into())].x.0 > 10.0);
    }
}
