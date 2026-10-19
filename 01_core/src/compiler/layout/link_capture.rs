//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/link_capture.md
//! @prompt-hash 0754d763
//! @layer L1

use super::{FontMetrics, ImageSizer, Layouter};
use crate::entities::{
    content::Content,
    elements::pdf_artifact::ArtifactKind,
    geometry::{FillRule, ShapeKind},
    layout_types::{FrameItem, LinkTarget, Point, SemanticKind, SemanticPlacement},
};

const PREFIX: &str = "\0tekt-link:";

#[derive(Clone, Debug, Default)]
pub(super) struct Capture {
    targets: Vec<LinkTarget>,
    pub(super) active: Vec<usize>,
}

impl Capture {
    pub(super) fn checkpoint(&self) -> (usize, Vec<usize>) {
        (self.targets.len(), self.active.clone())
    }
    pub(super) fn restore(&mut self, (len, active): (usize, Vec<usize>)) {
        self.targets.truncate(len);
        self.active = active;
    }
}

fn marker(id: usize, open: bool) -> FrameItem {
    FrameItem::Semantic {
        kind: SemanticKind::Artifact(ArtifactKind::Other),
        placement: SemanticPlacement::Inline,
        alt: Some(format!("{PREFIX}{}:{id}", if open { "open" } else { "close" }).into()),
        items: vec![FrameItem::Shape {
            pos: Point::ZERO,
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
fn decode(item: &FrameItem) -> Option<(usize, bool)> {
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
    let (event, id) = alt.strip_prefix(PREFIX)?.split_once(':')?;
    let open = match event {
        "open" => true,
        "close" => false,
        _ => return None,
    };
    Some((id.parse().ok()?, open))
}
pub(super) fn is_marker(item: &FrameItem) -> bool {
    decode(item).is_some()
}
fn starts(items: &mut Vec<FrameItem>, active: &[usize]) {
    items.extend(active.iter().map(|&id| marker(id, true)));
}
fn ends(items: &mut Vec<FrameItem>, active: &[usize]) {
    items.extend(active.iter().rev().map(|&id| marker(id, false)));
}
fn same_target(a: &LinkTarget, b: &LinkTarget) -> bool {
    match (a, b) {
        (LinkTarget::Url(a), LinkTarget::Url(b)) => a == b,
        (LinkTarget::Destination(a), LinkTarget::Destination(b)) => a == b,
        (LinkTarget::Url(_), LinkTarget::Destination(_))
        | (LinkTarget::Destination(_), LinkTarget::Url(_)) => false,
    }
}
fn remove_redundant(items: Vec<FrameItem>, target: &LinkTarget) -> Vec<FrameItem> {
    let mut out = vec![];
    for mut item in items {
        match item {
            FrameItem::Link { target: ref inner, .. } if same_target(inner, target) => {
                let FrameItem::Link { items, .. } = item else { unreachable!() };
                out.extend(remove_redundant(items, target));
                continue;
            }
            FrameItem::Group { ref mut items, .. }
            | FrameItem::Semantic { ref mut items, .. }
            | FrameItem::TextReplacement { ref mut items, .. } => {
                *items = remove_redundant(std::mem::take(items), target);
            }
            FrameItem::Link { .. }
            | FrameItem::Text { .. }
            | FrameItem::TextShaped { .. }
            | FrameItem::TextSeparator { .. }
            | FrameItem::Glyph { .. }
            | FrameItem::Line { .. }
            | FrameItem::Shape { .. }
            | FrameItem::Image { .. } => {}
        }
        out.push(item);
    }
    out
}
fn wrap<M: FontMetrics>(
    items: Vec<FrameItem>,
    target: &LinkTarget,
    metrics: &M,
) -> Vec<FrameItem> {
    if !super::html_frame::items_have_material(&items) {
        return items;
    }
    let items = remove_redundant(items, target);
    let (pos, size) = super::link::link_bbox(&items, metrics);
    if size.width.0 <= 0.0 || size.height.0 <= 0.0 {
        return items;
    }
    vec![FrameItem::Link { target: target.clone(), items, pos, size }]
}
fn append(
    out: &mut Vec<FrameItem>,
    stack: &mut [(usize, Vec<FrameItem>)],
    items: impl IntoIterator<Item = FrameItem>,
) {
    if let Some((_, current)) = stack.last_mut() {
        current.extend(items);
    } else {
        out.extend(items);
    }
}

pub(super) fn resolve<M: FontMetrics>(
    items: &mut Vec<FrameItem>,
    capture: &Capture,
    metrics: &M,
) {
    let mut out = vec![];
    let mut stack: Vec<(usize, Vec<FrameItem>)> = vec![];
    for mut item in std::mem::take(items) {
        if let Some((id, open)) = decode(&item) {
            if capture.targets.get(id).is_none() {
                continue;
            }
            if open {
                stack.push((id, vec![]));
            } else if let Some(index) = stack.iter().rposition(|(scope, _)| *scope == id)
            {
                // Retain content even if a discarded frame removed an inner delimiter.
                while stack.len() > index {
                    let (scope, children) = stack.pop().unwrap();
                    append(
                        &mut out,
                        &mut stack,
                        wrap(children, &capture.targets[scope], metrics),
                    );
                }
            }
            continue;
        }
        match &mut item {
            FrameItem::Group { items, .. }
            | FrameItem::Link { items, .. }
            | FrameItem::TextReplacement { items, .. }
            | FrameItem::Semantic { items, .. } => resolve(items, capture, metrics),
            FrameItem::Text { .. }
            | FrameItem::TextShaped { .. }
            | FrameItem::TextSeparator { .. }
            | FrameItem::Glyph { .. }
            | FrameItem::Line { .. }
            | FrameItem::Shape { .. }
            | FrameItem::Image { .. } => {}
        }
        append(&mut out, &mut stack, [item]);
    }
    while let Some((scope, children)) = stack.pop() {
        append(&mut out, &mut stack, wrap(children, &capture.targets[scope], metrics));
    }
    *items = out;
}

impl<M: FontMetrics, S: ImageSizer> Layouter<'_, M, S> {
    pub(super) fn layout_link_capture(
        &mut self,
        body: &Content,
        target: LinkTarget,
        expand: bool,
    ) {
        let id = self.link_capture.targets.len();
        self.link_capture.targets.push(target);
        self.link_capture.active.push(id);
        self.regions.current.current_line.push(marker(id, true));
        self.regions.current.current_items.push(marker(id, true));
        self.layout_content(body);
        if expand {
            self.expand_fr_spacings();
        }
        self.regions.current.current_line.push(marker(id, false));
        self.regions.current.current_items.push(marker(id, false));
        debug_assert_eq!(self.link_capture.active.pop(), Some(id));
    }
    pub(super) fn close_link_line_segments(&mut self) {
        ends(&mut self.regions.current.current_items, &self.link_capture.active);
        ends(&mut self.regions.current.current_line, &self.link_capture.active);
    }
    pub(super) fn open_link_segments(&mut self) {
        starts(&mut self.regions.current.current_items, &self.link_capture.active);
        starts(&mut self.regions.current.current_line, &self.link_capture.active);
    }
    pub(super) fn close_link_items(&mut self) {
        ends(&mut self.regions.current.current_items, &self.link_capture.active);
    }
    pub(super) fn open_link_items(&mut self) {
        starts(&mut self.regions.current.current_items, &self.link_capture.active);
    }
    pub(super) fn open_link_line(&mut self) {
        starts(&mut self.regions.current.current_line, &self.link_capture.active);
    }
    pub(super) fn pause_link_items(&mut self) -> Vec<usize> {
        self.close_link_items();
        std::mem::take(&mut self.link_capture.active)
    }
    pub(super) fn resume_link_items(&mut self, active: Vec<usize>) {
        self.link_capture.active = active;
        self.open_link_items();
    }
    pub(super) fn resolve_links(&mut self) {
        for page in &mut self.pages {
            for items in [&mut page.background, &mut page.items, &mut page.foreground] {
                resolve(items, &self.link_capture, &self.metrics);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{compiler::layout::FixedMetrics, entities::layout_types::Pt};

    fn rectangle(x: f64) -> FrameItem {
        FrameItem::Shape {
            pos: Point { x: Pt(x), y: Pt(20.0) },
            kind: ShapeKind::Rect,
            width: 5.0,
            height: 7.0,
            fill: None,
            stroke: None,
            fill_rule: FillRule::NonZero,
            parent_bbox_at_emit: None,
        }
    }

    #[test]
    fn scope_resolution_preserves_neighbors_and_final_fragment_bounds() {
        let target = LinkTarget::Url("https://example.org".into());
        let capture = Capture { targets: vec![target], active: vec![] };
        let mut items = vec![
            rectangle(1.0),
            marker(0, true),
            rectangle(30.0),
            rectangle(40.0),
            marker(0, false),
            rectangle(70.0),
        ];
        resolve(&mut items, &capture, &FixedMetrics);
        let [FrameItem::Shape { pos: before, .. }, FrameItem::Link { target, pos, size, items: body }, FrameItem::Shape { pos: after, .. }] =
            items.as_slice()
        else {
            panic!("capture must retain both unlinked neighbors");
        };
        assert_eq!((before.x.0, after.x.0), (1.0, 70.0));
        assert!(
            matches!(target, LinkTarget::Url(url) if url.as_str() == "https://example.org")
        );
        assert_eq!(
            (pos.x.0, pos.y.0, size.width.0, size.height.0),
            (30.0, 20.0, 15.0, 7.0)
        );
        assert_eq!(body.len(), 2);
        assert!(body.iter().all(|item| matches!(item, FrameItem::Shape { .. })));
    }

    #[test]
    fn metadata_only_scope_never_creates_clickable_area() {
        let capture = Capture {
            targets: vec![LinkTarget::Url("https://example.org".into())],
            active: vec![],
        };
        let mut items = vec![marker(0, true), marker(0, false), rectangle(70.0)];
        resolve(&mut items, &capture, &FixedMetrics);
        assert!(
            matches!(items.as_slice(), [FrameItem::Shape { pos, .. }] if pos.x.0 == 70.0)
        );
    }

    #[test]
    fn visible_artifact_with_marker_like_alt_is_not_consumed() {
        let mut artifact = marker(0, true);
        let FrameItem::Semantic { items, .. } = &mut artifact else { unreachable!() };
        items[0] = rectangle(30.0);
        let mut items = vec![artifact];
        resolve(&mut items, &Capture::default(), &FixedMetrics);
        let [FrameItem::Semantic { items: children, .. }] = items.as_slice() else {
            panic!("visible semantic artifact must survive");
        };
        assert!(matches!(
            children.as_slice(),
            [FrameItem::Shape { width: 5.0, height: 7.0, .. }]
        ));
    }
}
