// Disposable laboratory copy. Keep the proof and retention in the paged owner layer.
use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};
use typst_core::{
    contracts::world::World,
    entities::{
        bidi_context::{BidiSpan, ParagraphBidiInput, ParagraphBidiSegment::*},
        compiler_features::Features,
        layout_types::{
            FrameItem, Length, Page, PagedDocument, Point, TextEdge, TextStyle,
        },
    },
};

#[derive(Default)]
pub(in crate::pipeline) struct Session(Mutex<Option<Entry>>);

struct Entry {
    epoch: Arc<()>,
    features: Features,
    metrics_route: u8,
    pages: Vec<Option<SavedPage>>,
}

struct SavedPage {
    raw: Page,
    finished: Layers,
}

#[derive(Clone)]
struct Layers {
    body: Vec<FrameItem>,
    background: Vec<FrameItem>,
    foreground: Vec<FrameItem>,
}

impl Layers {
    fn capture(page: &Page) -> Self {
        Self {
            body: page.items.clone(),
            background: page.background.clone(),
            foreground: page.foreground.clone(),
        }
    }

    fn install(&self, page: &mut Page) {
        page.items.clone_from(&self.body);
        page.background.clone_from(&self.background);
        page.foreground.clone_from(&self.foreground);
    }
}

enum Slot {
    Hit(Page, SavedPage),
    Miss { raw: Option<Page>, keep: bool },
}

impl Session {
    pub(super) fn clear(&self) {
        self.0.lock().unwrap().take();
    }

    pub(super) fn process(
        &self,
        world: &dyn World,
        features: Features,
        metrics_route: u8,
        mut document: PagedDocument,
        transform: impl Fn(PagedDocument) -> PagedDocument,
    ) -> PagedDocument {
        let mut state = self.0.lock().unwrap();
        let trace = std::env::var_os("TEKT_LAB_WATCH_TRACE").is_some();
        let count = document.pages.len();
        let Some(epoch) = world.font_query_epoch().filter(|_| count <= 128) else {
            state.take();
            if trace {
                eprintln!("pre-bidi-pages total={count} checked=0 reused=0 reason=scope");
            }
            return transform(document);
        };
        let mut previous = state
            .take()
            .filter(|entry| {
                Arc::ptr_eq(&entry.epoch, &epoch)
                    && entry.features == features
                    && entry.metrics_route == metrics_route
            })
            .map(|entry| entry.pages)
            .unwrap_or_default();
        let mut checked = 0;
        let hits: Vec<bool> = document
            .pages
            .iter()
            .enumerate()
            .map(|(i, page)| {
                previous.get(i).and_then(Option::as_ref).is_some_and(|saved| {
                    checked += 1;
                    same_page(&saved.raw, page)
                })
            })
            .collect();
        let hit_count = hits.iter().filter(|hit| **hit).count();

        let mut slots = Vec::with_capacity(count);
        let mut dirty = Vec::with_capacity(count - hit_count);
        for (i, page) in std::mem::take(&mut document.pages).into_iter().enumerate() {
            if hits[i] {
                slots.push(Slot::Hit(page, previous[i].take().unwrap()));
            } else {
                let keep = same_page(&page, &page);
                let raw = (keep || hit_count != 0).then(|| page.clone());
                slots.push(Slot::Miss { raw, keep });
                dirty.push(page);
            }
        }
        drop(previous);

        document.pages = dirty;
        if !document.pages.is_empty() {
            document = transform(document);
        }
        if !world
            .font_query_epoch()
            .is_some_and(|current| Arc::ptr_eq(&current, &epoch))
        {
            if trace {
                eprintln!(
                    "pre-bidi-pages total={count} checked={checked} reused=0 reason=epoch"
                );
            }
            if hit_count == 0 {
                return document;
            }
            document.pages = slots
                .into_iter()
                .map(|slot| match slot {
                    Slot::Hit(page, _) => page,
                    Slot::Miss { raw, .. } => {
                        raw.expect("fallback input when there is a hit")
                    }
                })
                .collect();
            return transform(document);
        }

        let mut next = Vec::with_capacity(count);
        let mut processed = std::mem::take(&mut document.pages).into_iter();
        for slot in slots {
            match slot {
                Slot::Hit(mut page, saved) => {
                    saved.finished.install(&mut page);
                    document.pages.push(page);
                    next.push(Some(saved));
                }
                Slot::Miss { raw, keep } => {
                    let page = processed.next().expect("transform preserves page count");
                    next.push(keep.then(|| SavedPage {
                        raw: raw.unwrap(),
                        finished: Layers::capture(&page),
                    }));
                    document.pages.push(page);
                }
            }
        }
        assert!(processed.next().is_none());
        if trace {
            let ordinals: Vec<_> = hits
                .iter()
                .enumerate()
                .filter_map(|(i, hit)| hit.then_some(i + 1))
                .collect();
            eprintln!(
                "pre-bidi-pages total={count} checked={checked} reused={hit_count} ordinals={ordinals:?}"
            );
        }
        *state = Some(Entry { epoch, features, metrics_route, pages: next });
        document
    }
}

fn scalar(a: f64, b: f64) -> bool {
    a.is_finite() && a.to_bits() == b.to_bits()
}

fn point(a: &Point, b: &Point) -> bool {
    scalar(a.x.0, b.x.0) && scalar(a.y.0, b.y.0)
}

fn length(a: Option<Length>, b: Option<Length>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => scalar(a.abs.to_pt(), b.abs.to_pt()) && scalar(a.em, b.em),
        _ => false,
    }
}

fn edge(a: &Option<TextEdge>, b: &Option<TextEdge>) -> bool {
    match (a, b) {
        (Some(TextEdge::Length(a)), Some(TextEdge::Length(b))) => {
            length(Some(*a), Some(*b))
        }
        _ => a == b,
    }
}

fn style(a: &TextStyle, b: &TextStyle) -> bool {
    a == b
        && scalar(a.size.0, b.size.0)
        && length(a.tracking, b.tracking)
        && length(a.leading, b.leading)
        && edge(&a.top_edge, &b.top_edge)
        && edge(&a.bottom_edge, &b.bottom_edge)
        && length(a.highlight_radius, b.highlight_radius)
        && length(a.highlight_extent, b.highlight_extent)
        && length(a.subscript_size, b.subscript_size)
        && length(a.superscript_size, b.superscript_size)
        && length(Some(a.baseline_offset), Some(b.baseline_offset))
        && match (&a.variations, &b.variations) {
            (None, None) => true,
            (Some(a), Some(b)) => {
                a.0.iter()
                    .zip(&b.0)
                    .all(|((_, a), (_, b))| a.to_bits() == b.to_bits())
            }
            _ => false,
        }
}

struct Proof {
    // Line resolution tests Arc co-membership. Preserve that equivalence relation,
    // in addition to the complete paragraph input beyond a physical page boundary.
    forward: HashMap<*const ParagraphBidiInput, *const ParagraphBidiInput>,
    reverse: HashSet<*const ParagraphBidiInput>,
    remaining: usize,
}

impl Proof {
    fn bidi(&mut self, a: &Option<BidiSpan>, b: &Option<BidiSpan>) -> bool {
        let (a, b) = match (a, b) {
            (None, None) => return true,
            (Some(a), Some(b)) => (a, b),
            _ => return false,
        };
        if a.segment() != b.segment() || a.range() != b.range() {
            return false;
        }
        let (old, new) = (a.input(), b.input());
        let (left, right) = (Arc::as_ptr(old), Arc::as_ptr(new));
        if let Some(mapped) = self.forward.get(&left) {
            return *mapped == right;
        }
        if !self.reverse.insert(right) {
            return false;
        }
        self.forward.insert(left, right);
        Arc::ptr_eq(old, new)
            || old.base_dir() == new.base_dir()
                && old.segments().len() == new.segments().len()
                && old.segments().iter().zip(new.segments()).all(|(a, b)| match (a, b) {
                    (Text { text, dir }, Text { text: t, dir: d }) => {
                        text == t && dir == d
                    }
                    (Space { origin }, Space { origin: o }) => origin == o,
                    (Linebreak, Linebreak) => true,
                    _ => false,
                })
    }

    fn items(&mut self, a: &[FrameItem], b: &[FrameItem], depth: usize) -> bool {
        if depth > 48 || a.len() != b.len() || a.len() > self.remaining {
            return false;
        }
        self.remaining -= a.len();
        use FrameItem::*;
        a.iter().zip(b).all(|(a, b)| match (a, b) {
            (
                Text { pos, text, style: s, bidi },
                Text { pos: p, text: t, style: z, bidi: d },
            ) => point(pos, p) && text == t && style(s, z) && self.bidi(bidi, d),
            (
                TextShaped {
                    pos,
                    text,
                    style: s,
                    bidi,
                    glyphs,
                    font,
                    units_per_em,
                },
                TextShaped {
                    pos: p,
                    text: t,
                    style: z,
                    bidi: d,
                    glyphs: g,
                    font: f,
                    units_per_em: em,
                },
            ) => {
                point(pos, p)
                    && text == t
                    && style(s, z)
                    && glyphs == g
                    && units_per_em == em
                    && match (font, f) {
                        (Some(a), Some(b)) => {
                            std::ptr::eq(a.as_slice(), b.as_slice()) || a == b
                        }
                        (None, None) => true,
                        _ => false,
                    }
                    && self.bidi(bidi, d)
            }
            (
                TextSeparator { pos, advance, style: s, origin, bidi },
                TextSeparator { pos: p, advance: x, style: z, origin: o, bidi: d },
            ) => {
                point(pos, p)
                    && scalar(advance.0, x.0)
                    && style(s, z)
                    && origin == o
                    && self.bidi(bidi, d)
            }
            (TextReplacement { text, items }, TextReplacement { text: t, items: b }) => {
                text == t && self.items(items, b, depth + 1)
            }
            (
                Semantic { kind, placement, alt, items },
                Semantic { kind: k, placement: p, alt: t, items: b },
            ) => {
                kind == k && placement == p && alt == t && self.items(items, b, depth + 1)
            }
            (
                Link { target, items, pos, size },
                Link { target: t, items: b, pos: p, size: s },
            ) => {
                target == t
                    && point(pos, p)
                    && scalar(size.width.0, s.width.0)
                    && scalar(size.height.0, s.height.0)
                    && self.items(items, b, depth + 1)
            }
            (
                Group {
                    paint_root,
                    pos,
                    matrix: m,
                    clip_mask: None,
                    inner_width,
                    inner_height,
                    items,
                },
                Group {
                    paint_root: r,
                    pos: p,
                    matrix: n,
                    clip_mask: None,
                    inner_width: w,
                    inner_height: h,
                    items: b,
                },
            ) => {
                paint_root == r
                    && point(pos, p)
                    && scalar(*inner_width, *w)
                    && scalar(*inner_height, *h)
                    && [m.a, m.b, m.c, m.d, m.tx, m.ty]
                        .into_iter()
                        .zip([n.a, n.b, n.c, n.d, n.tx, n.ty])
                        .all(|(a, b)| scalar(a, b))
                    && self.items(items, b, depth + 1)
            }
            _ => false,
        })
    }
}

fn same_page(a: &Page, b: &Page) -> bool {
    let mut proof = Proof {
        forward: HashMap::new(),
        reverse: HashSet::new(),
        remaining: 8192,
    };
    scalar(a.width, b.width)
        && scalar(a.height, b.height)
        && a.bleed.is_zero()
        && b.bleed.is_zero()
        && a.fill == b.fill
        && proof.items(&a.background, &b.background, 0)
        && proof.items(&a.items, &b.items, 0)
        && proof.items(&a.foreground, &b.foreground, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use typst_core::entities::dir::Dir;

    fn input(tail: &str) -> Arc<ParagraphBidiInput> {
        Arc::new(
            ParagraphBidiInput::new(
                Dir::LTR,
                vec![
                    Text { text: "visible".into(), dir: Dir::LTR },
                    Text { text: tail.into(), dir: Dir::RTL },
                ],
            )
            .unwrap(),
        )
    }

    fn span(input: Arc<ParagraphBidiInput>) -> Option<BidiSpan> {
        BidiSpan::new(input, 0, 0..7)
    }

    fn proof() -> Proof {
        Proof {
            forward: HashMap::new(),
            reverse: HashSet::new(),
            remaining: 64,
        }
    }

    #[test]
    fn proves_complete_context_past_the_page() {
        let mut p = proof();
        assert!(!p.bidi(&span(input("א")), &span(input("ב"))));
        assert!(proof().bidi(&span(input("א")), &span(input("א"))));
    }

    #[test]
    fn refuses_changed_sharing_in_both_directions() {
        let shared = span(input("א"));
        let first = span(input("א"));
        let second = span(input("א"));
        let mut p = proof();
        assert!(p.bidi(&shared, &first));
        assert!(!p.bidi(&shared, &second));
        let mut p = proof();
        assert!(p.bidi(&first, &shared));
        assert!(!p.bidi(&second, &shared));
    }
}
