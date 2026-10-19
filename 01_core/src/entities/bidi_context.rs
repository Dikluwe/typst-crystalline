//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/bidi_context.md
//! @prompt-hash fea975be
//! @layer L1

use super::{dir::Dir, layout_types::TextSeparatorOrigin};
use ecow::EcoString;
use std::{ops::Range, sync::Arc};

#[derive(Clone, Debug)]
pub enum ParagraphBidiSegment {
    Text { text: EcoString, dir: Dir },
    Space { origin: TextSeparatorOrigin },
    Linebreak,
}
#[derive(Clone, Debug)]
pub struct ParagraphBidiInput {
    base_dir: Dir,
    segments: Vec<ParagraphBidiSegment>,
}
#[derive(Clone, Debug)]
pub struct BidiSpan {
    input: Arc<ParagraphBidiInput>,
    segment: usize,
    range: Range<usize>,
}
impl ParagraphBidiInput {
    pub fn new(base_dir: Dir, segments: Vec<ParagraphBidiSegment>) -> Option<Self> {
        if !base_dir.is_horizontal() || !segments.iter().any(|s| matches!(s, ParagraphBidiSegment::Text { text, .. } if !text.is_empty())) { return None; }
        Some(Self { base_dir, segments })
    }
    pub fn base_dir(&self) -> Dir {
        self.base_dir
    }
    pub fn segments(&self) -> &[ParagraphBidiSegment] {
        &self.segments
    }
}
impl BidiSpan {
    pub fn new(
        input: Arc<ParagraphBidiInput>,
        segment: usize,
        range: Range<usize>,
    ) -> Option<Self> {
        let text = match input.segments.get(segment)? {
            ParagraphBidiSegment::Text { text, .. } => text.as_str(),
            ParagraphBidiSegment::Space { .. } => " ",
            ParagraphBidiSegment::Linebreak => return None,
        };
        if range.start > range.end || text.get(range.clone()).is_none() {
            return None;
        }
        Some(Self { input, segment, range })
    }
    pub fn input(&self) -> &Arc<ParagraphBidiInput> {
        &self.input
    }
    pub fn segment(&self) -> usize {
        self.segment
    }
    pub fn range(&self) -> Range<usize> {
        self.range.clone()
    }
    pub fn subspan(&self, relative: Range<usize>) -> Option<Self> {
        if relative.start > relative.end
            || relative.end > self.range.end - self.range.start
        {
            return None;
        }
        Self::new(
            self.input.clone(),
            self.segment,
            self.range.start.checked_add(relative.start)?
                ..self.range.start.checked_add(relative.end)?,
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn input() -> Arc<ParagraphBidiInput> {
        Arc::new(
            ParagraphBidiInput::new(
                Dir::RTL,
                vec![
                    ParagraphBidiSegment::Text { text: "é aa aa".into(), dir: Dir::LTR },
                    ParagraphBidiSegment::Space { origin: TextSeparatorOrigin::Markup },
                    ParagraphBidiSegment::Linebreak,
                ],
            )
            .unwrap(),
        )
    }
    #[test]
    fn rejects_nonhorizontal_and_nonsubstantive_input() {
        for dir in [Dir::TTB, Dir::BTT, Dir::LTR] {
            assert!(ParagraphBidiInput::new(dir, vec![]).is_none());
        }
        assert!(ParagraphBidiInput::new(
            Dir::LTR,
            vec![ParagraphBidiSegment::Text { text: "".into(), dir: Dir::LTR }]
        )
        .is_none());
        assert!(ParagraphBidiInput::new(Dir::TTB, input().segments().to_vec()).is_none());
        assert_eq!(input().base_dir(), Dir::RTL);
    }
    #[test]
    fn validates_utf8_bounds_and_segment_kind() {
        let p = input();
        for (segment, range) in [
            (0, 1..2),
            (0, 0..9),
            (0, Range { start: 4, end: 3 }),
            (3, 0..0),
            (2, 0..0),
            (1, 0..2),
        ] {
            assert!(BidiSpan::new(p.clone(), segment, range).is_none());
        }
        assert!(BidiSpan::new(p, 1, 0..1).is_some());
    }
    #[test]
    fn subspans_retain_occurrences_and_check_relative_ranges() {
        let p = input();
        let s = BidiSpan::new(p.clone(), 0, 3..8).unwrap();
        let a = s.subspan(0..2).unwrap();
        let b = s.subspan(3..5).unwrap();
        assert_eq!(a.range(), 3..5);
        assert_eq!(b.range(), 6..8);
        assert!(Arc::ptr_eq(a.input(), b.input()));
        assert_eq!(a.segment(), 0);
        assert_eq!(b.clone().range(), 6..8);
        assert!(s.subspan(0..usize::MAX).is_none());
        assert!(s.subspan(Range { start: 3, end: 2 }).is_none());
        assert!(BidiSpan::new(p, 0, 0..2).unwrap().subspan(1..2).is_none());
    }
}
