//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/link_capture.md
//! @prompt-hash 84c1e0e4
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
fn clean_link_child(item: &mut FrameItem, target: &LinkTarget) -> bool {
    match item {
        FrameItem::Link { target: inner, .. } => same_target(inner, target),
        FrameItem::Group { items, .. }
        | FrameItem::Semantic { items, .. }
        | FrameItem::TextReplacement { items, .. } => {
            *items = remove_redundant(std::mem::take(items), target);
            false
        }
        FrameItem::Text { .. }
        | FrameItem::TextShaped { .. }
        | FrameItem::TextSeparator { .. }
        | FrameItem::Glyph { .. }
        | FrameItem::Line { .. }
        | FrameItem::Shape { .. }
        | FrameItem::Image { .. } => false,
    }
}

fn promote_link(item: FrameItem, target: &LinkTarget) -> Vec<FrameItem> {
    let FrameItem::Link { items, .. } = item else { unreachable!() };
    remove_redundant(items, target)
}

fn remove_redundant(mut items: Vec<FrameItem>, target: &LinkTarget) -> Vec<FrameItem> {
    let Some(first) = items.iter_mut().position(|item| clean_link_child(item, target)) else {
        return items;
    };

    let mut out = Vec::with_capacity(items.len());
    let mut items = items.into_iter();
    out.extend(items.by_ref().take(first));
    out.extend(promote_link(items.next().expect("first redundant link"), target));
    for mut item in items {
        if clean_link_child(&mut item, target) {
            out.extend(promote_link(item, target));
        } else {
            out.push(item);
        }
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

fn resolve_children<M: FontMetrics>(
    item: &mut FrameItem,
    capture: &Capture,
    metrics: &M,
) {
    match item {
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
}

fn append_item(
    out: &mut Vec<FrameItem>,
    stack: &mut [(usize, Vec<FrameItem>)],
    item: FrameItem,
) {
    if let Some((_, current)) = stack.last_mut() {
        current.push(item);
    } else {
        out.push(item);
    }
}

pub(super) fn resolve<M: FontMetrics>(
    items: &mut Vec<FrameItem>,
    capture: &Capture,
    metrics: &M,
) {
    // Only direct delimiters change this vector's membership. Descendants still
    // resolve their own delimiters, without moving this vector's ordinary items.
    if !items.iter().any(is_marker) {
        for item in items {
            resolve_children(item, capture, metrics);
        }
        return;
    }
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
        resolve_children(&mut item, capture, metrics);
        append_item(&mut out, &mut stack, item);
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
        self.capture_link_body(target, expand, |layouter| layouter.layout_content(body));
    }

    pub(super) fn layout_link_capture_planned(
        &mut self,
        body: &Content,
        target: LinkTarget,
        expand: bool,
        cursor: Option<super::page_running_requests::Cursor>,
    ) {
        self.capture_link_body(target, expand, |layouter| {
            if let Some(cursor) = cursor {
                layouter.layout_planned_cursor(cursor, body, false);
            } else {
                layouter.layout_content(body);
            }
        });
    }

    pub(super) fn layout_link_capture_mixed(
        &mut self,
        parts: &[Content],
        target: LinkTarget,
        expand: bool,
        routes: &[Option<super::page_running_requests::Cursor>],
    ) {
        self.capture_link_body(target, expand, |layouter| {
            super::sequence::layout_mixed_source_parts(layouter, parts, routes);
        });
    }

    fn capture_link_body(
        &mut self,
        target: LinkTarget,
        expand: bool,
        body: impl FnOnce(&mut Self),
    ) {
        let id = self.link_capture.targets.len();
        self.link_capture.targets.push(target);
        self.link_capture.active.push(id);
        self.regions.current.current_line.push(marker(id, true));
        self.regions.current.current_items.push(marker(id, true));
        body(self);
        if expand {
            self.expand_fr_spacings();
        }
        self.regions.current.current_line.push(marker(id, false));
        self.regions.current.current_items.push(marker(id, false));
        let closed = self.link_capture.active.pop();
        debug_assert_eq!(closed, Some(id));
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

    fn nested_link(target: &LinkTarget, items: Vec<FrameItem>) -> FrameItem {
        use crate::entities::layout_types::Size;
        FrameItem::Link {
            target: target.clone(),
            items,
            pos: Point { x: Pt(0.0), y: Pt(0.0) },
            size: Size { width: Pt(1.0), height: Pt(1.0) },
        }
    }

    fn envelope(items: Vec<FrameItem>) -> FrameItem {
        FrameItem::Semantic {
            kind: SemanticKind::Artifact(ArtifactKind::Other),
            alt: None,
            placement: SemanticPlacement::Inline,
            items,
        }
    }

    #[test]
    fn redundant_links_keep_buffers_with_the_same_direct_children() {
        let outer = LinkTarget::Url("https://example.test/outer".into());
        let distinct = LinkTarget::Url("https://example.test/distinct".into());
        let mut sibling = Vec::with_capacity(17);
        sibling.push(rectangle(1.0));
        let sibling_buffer = (sibling.as_ptr(), sibling.capacity());
        let mut parent = Vec::with_capacity(23);
        parent.push(envelope(sibling));
        parent.push(envelope(vec![
            nested_link(&outer, vec![rectangle(2.0)]),
            nested_link(&distinct, vec![nested_link(&outer, vec![rectangle(3.0)])]),
            nested_link(&outer, vec![]),
            rectangle(4.0),
        ]));
        let parent_buffer = (parent.as_ptr(), parent.capacity());

        let result = remove_redundant(parent, &outer);

        let [FrameItem::Semantic { items: sibling, .. }, FrameItem::Semantic { items: changed, .. }] =
            result.as_slice()
        else {
            panic!("expected the two original wrappers");
        };
        let [
            FrameItem::Shape { pos: first, .. },
            FrameItem::Link { target, items, .. },
            FrameItem::Shape { pos: last, .. },
        ] = changed.as_slice()
        else {
            panic!("expected promoted content, distinct link, and trailing shape");
        };
        assert_eq!((first.x, last.x), (Pt(2.0), Pt(4.0)));
        assert!(same_target(target, &distinct));
        assert!(matches!(
            items.as_slice(),
            [FrameItem::Link { target, .. }] if same_target(target, &outer)
        ));
        assert_eq!((result.as_ptr(), result.capacity()), parent_buffer);
        assert_eq!((sibling.as_ptr(), sibling.capacity()), sibling_buffer);
    }

    #[test]
    fn lexical_scope_closure_excludes_the_following_page() {
        use crate::entities::{
            image_sizer::NullImageSizer,
            introspector::{Introspector, TagIntrospector},
            layout_types::{walk_frame_items, FrameVisitor},
        };
        use comemo::Track;
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.layout_link_capture(
            &Content::text("LINK"),
            LinkTarget::Url("https://example.org".into()),
            false,
        );
        assert!(
            l.link_capture.active.is_empty(),
            "completed lexical scope must close in every build profile"
        );
        l.layout_content(&Content::pagebreak(false, None));
        l.layout_content(&Content::text("BODY"));
        let doc = l.finish();
        assert_eq!(doc.pages.len(), 2);
        struct Links(usize);
        impl FrameVisitor for Links {
            fn visit_link(&mut self, _: &FrameItem) {
                self.0 += 1;
            }
        }
        let mut linked = Links(0);
        walk_frame_items(&mut linked, &doc.pages[0].items);
        assert!(linked.0 > 0, "intended link survives");
        let mut body = Links(0);
        walk_frame_items(&mut body, &doc.pages[1].items);
        assert_eq!(body.0, 0, "unrelated BODY must remain unlinked");
    }

    #[test]
    fn lexical_scope_closure_restores_the_outer_capture() {
        use crate::entities::{
            image_sizer::NullImageSizer,
            introspector::{Introspector, TagIntrospector},
        };
        use comemo::Track;
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.capture_link_body(
            LinkTarget::Url("https://outer.example".into()),
            false,
            |l| {
                let outer = l.link_capture.active.clone();
                l.layout_content(&Content::text("BEFORE"));
                l.layout_link_capture(
                    &Content::text("INNER"),
                    LinkTarget::Url("https://inner.example".into()),
                    false,
                );
                assert_eq!(
                    l.link_capture.active, outer,
                    "nested closure must restore the outer lexical scope"
                );
                l.layout_content(&Content::text("AFTER"));
            },
        );
        assert!(l.link_capture.active.is_empty());
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

#[cfg(test)]
mod vector_retention_tests {
    use super::*;
    use crate::{compiler::layout::FixedMetrics, entities::layout_types::Pt};
    fn rect(x: f64) -> FrameItem {
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
    fn envelope(items: Vec<FrameItem>, alt: &str) -> FrameItem {
        FrameItem::Semantic {
            kind: SemanticKind::Artifact(ArtifactKind::Other),
            placement: SemanticPlacement::Inline,
            alt: Some(alt.into()),
            items,
        }
    }
    #[test]
    fn marker_free_vector_retains_its_actual_allocation() {
        let mut items = Vec::with_capacity(512);
        items.extend((0..128).map(|i| rect(i as f64)));
        let storage = (items.as_ptr(), items.capacity());
        resolve(&mut items, &Capture::default(), &FixedMetrics);
        assert_eq!(items.len(), 128);
        assert_eq!(
            (items.as_ptr(), items.capacity()),
            storage,
            "marker-free real resolver must retain the original vector storage"
        );
    }
    #[test]
    fn marker_free_descendants_retain_their_actual_allocations() {
        let mut children = Vec::with_capacity(256);
        children.extend((0..64).map(|i| rect(i as f64)));
        let child_storage = (children.as_ptr(), children.capacity());
        let mut items = Vec::with_capacity(128);
        items.push(envelope(children, "legitimate artifact"));
        let root_storage = (items.as_ptr(), items.capacity());
        resolve(&mut items, &Capture::default(), &FixedMetrics);
        assert_eq!((items.as_ptr(), items.capacity()), root_storage);
        let FrameItem::Semantic { items: children, .. } = &items[0] else { panic!() };
        assert_eq!((children.as_ptr(), children.capacity()), child_storage);
        assert_eq!(children.len(), 64);
    }
    #[test]
    fn marker_free_parent_still_resolves_its_descendant_scope() {
        let capture = Capture {
            targets: vec![LinkTarget::Url("https://a.example".into())],
            active: vec![],
        };
        let mut items = Vec::with_capacity(64);
        items.push(envelope(
            vec![marker(0, true), rect(30.0), marker(0, false)],
            "container",
        ));
        let storage = (items.as_ptr(), items.capacity());
        resolve(&mut items, &capture, &FixedMetrics);
        let FrameItem::Semantic { items: children, .. } = &items[0] else { panic!() };
        assert!(matches!(children.as_slice(), [FrameItem::Link { items, pos, size, .. }]
            if items.len() == 1 && pos.x.0 == 30.0 && size.width.0 == 5.0 && size.height.0 == 7.0));
        assert_eq!((items.as_ptr(), items.capacity()), storage);
    }
    #[test]
    fn unknown_delimiters_are_consumed_even_with_an_empty_registry() {
        let mut items = vec![marker(99, true), rect(30.0), marker(99, false)];
        resolve(&mut items, &Capture::default(), &FixedMetrics);
        assert!(
            matches!(items.as_slice(), [FrameItem::Shape { pos, .. }] if pos.x.0 == 30.0)
        );
    }
    #[test]
    fn position_carriers_and_visible_marker_like_artifacts_are_preserved() {
        let mut position = marker(0, true);
        let FrameItem::Semantic { alt, .. } = &mut position else { panic!() };
        *alt = Some("\0tekt-position:7".into());
        let mut visible = marker(0, true);
        let FrameItem::Semantic { items: children, .. } = &mut visible else { panic!() };
        children[0] = rect(42.0);
        let mut items = vec![position, visible];
        let expected = format!("{items:?}");
        resolve(&mut items, &Capture::default(), &FixedMetrics);
        assert_eq!(format!("{items:?}"), expected);
    }
    #[test]
    fn lexical_resolution_matches_the_frozen_resolver_for_mixed_programs() {
        let capture = Capture {
            targets: vec![
                LinkTarget::Url("https://a.example".into()),
                LinkTarget::Url("https://b.example".into()),
            ],
            active: vec![],
        };
        let programs = [
            vec![4],
            vec![0, 4, 1],
            vec![0, 2, 4, 3, 1],
            vec![0, 2, 4, 1, 3],
            vec![0, 4],
            vec![1, 4],
            vec![5, 4, 6],
            vec![0, 8, 1],
            vec![0, 9, 1],
            vec![0, 10, 1],
            vec![0, 7, 4, 1],
            vec![2, 4, 0, 4, 3, 1],
            vec![0, 2, 3, 1],
        ];
        let mut state = 1234567u64;
        for round in 0..109 {
            let program = if round < programs.len() {
                programs[round].clone()
            } else {
                (0..48)
                    .map(|_| {
                        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
                        ((state >> 32) % 11) as usize
                    })
                    .collect()
            };
            let mut actual: Vec<_> = program
                .iter()
                .enumerate()
                .map(|(i, op)| match op {
                    0 => marker(0, true),
                    1 => marker(0, false),
                    2 => marker(1, true),
                    3 => marker(1, false),
                    5 => marker(999, true),
                    6 => marker(999, false),
                    7 => envelope(vec![rect(i as f64)], "\0tekt-link:open:0"),
                    8 => envelope(vec![], "\0tekt-position:1"),
                    9 => wrap(vec![rect(i as f64)], &capture.targets[0], &FixedMetrics)
                        .pop()
                        .unwrap(),
                    10 => wrap(vec![rect(i as f64)], &capture.targets[1], &FixedMetrics)
                        .pop()
                        .unwrap(),
                    _ => rect(i as f64),
                })
                .collect();
            if round % 2 == 0 {
                actual = vec![envelope(actual, "nested")];
            }
            let mut expected = actual.clone();
            legacy_resolve(&mut expected, &capture, &FixedMetrics);
            resolve(&mut actual, &capture, &FixedMetrics);
            assert_eq!(format!("{actual:?}"), format!("{expected:?}"), "program {round}");
        }
    }

    fn legacy_append(
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

    fn legacy_resolve<M: FontMetrics>(
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
                } else if let Some(index) =
                    stack.iter().rposition(|(scope, _)| *scope == id)
                {
                    // Retain content even if a discarded frame removed an inner delimiter.
                    while stack.len() > index {
                        let (scope, children) = stack.pop().unwrap();
                        legacy_append(
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
                | FrameItem::Semantic { items, .. } => {
                    legacy_resolve(items, capture, metrics)
                }
                FrameItem::Text { .. }
                | FrameItem::TextShaped { .. }
                | FrameItem::TextSeparator { .. }
                | FrameItem::Glyph { .. }
                | FrameItem::Line { .. }
                | FrameItem::Shape { .. }
                | FrameItem::Image { .. } => {}
            }
            legacy_append(&mut out, &mut stack, [item]);
        }
        while let Some((scope, children)) = stack.pop() {
            legacy_append(
                &mut out,
                &mut stack,
                wrap(children, &capture.targets[scope], metrics),
            );
        }
        *items = out;
    }
}
