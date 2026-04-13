//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/span.md
//! @prompt-hash 80d1ba02
//! @layer L1
//! @updated 2026-03-22

use std::fmt::{self, Debug, Formatter};
use std::num::NonZeroU64;
use std::ops::Range;

use super::file_id::FileId;

/// Defines a range in a file.
///
/// This is used throughout the compiler to track which source section an
/// element stems from or an error applies to.
///
/// This type takes up 8 bytes and is copyable and null-optimized (i.e.
/// `Option<Span>` also takes 8 bytes).
///
/// Spans come in two flavors: Numbered spans and raw range spans.
///
/// # Numbered spans
/// Typst source files use _numbered spans._ Rather than using byte ranges,
/// which shift a lot as you type, each AST node gets a unique number.
///
/// # Raw range spans
/// Non Typst-files use raw ranges instead of numbered spans. The maximum
/// encodable value for start and end is 2^23. Larger values will be saturated.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Span(NonZeroU64);

impl Span {
    /// The full range of numbers available for source file span numbering.
    pub(crate) const FULL: Range<u64> = 2..(1 << 47);

    /// The value reserved for the detached span.
    const DETACHED: u64 = 1;

    /// Data layout:
    /// | 16 bits file id | 48 bits number |
    const NUMBER_BITS: usize = 48;
    const FILE_ID_SHIFT: usize = Self::NUMBER_BITS;
    const NUMBER_MASK: u64 = (1 << Self::NUMBER_BITS) - 1;
    const RANGE_BASE: u64 = Self::FULL.end;
    const RANGE_PART_BITS: usize = 23;
    const RANGE_PART_SHIFT: usize = Self::RANGE_PART_BITS;
    const RANGE_PART_MASK: u64 = (1 << Self::RANGE_PART_BITS) - 1;

    /// Create a span that does not point into any file.
    pub const fn detached() -> Self {
        Self(NonZeroU64::new(Self::DETACHED).unwrap())
    }

    /// Create a new span from a file id and a number.
    ///
    /// Returns `None` if `number` is not contained in `FULL`.
    pub(crate) const fn from_number(id: FileId, number: u64) -> Option<Self> {
        if number < Self::FULL.start || number >= Self::FULL.end {
            return None;
        }
        Some(Self::pack(id, number))
    }

    /// Create a new span from a raw byte range instead of a span number.
    ///
    /// If one of the range's parts exceeds the maximum value (2^23), it is
    /// saturated.
    pub const fn from_range(id: FileId, range: Range<usize>) -> Self {
        let max = 1 << Self::RANGE_PART_BITS;
        let start = if range.start > max { max } else { range.start } as u64;
        let end = if range.end > max { max } else { range.end } as u64;
        let number = (start << Self::RANGE_PART_SHIFT) | end;
        Self::pack(id, Self::RANGE_BASE + number)
    }

    /// Construct from a raw number.
    ///
    /// Should only be used with numbers retrieved via [`into_raw`](Self::into_raw).
    pub const fn from_raw(v: NonZeroU64) -> Self {
        Self(v)
    }

    /// Pack a file ID and the low bits into a span.
    const fn pack(id: FileId, low: u64) -> Self {
        let bits = ((id.into_raw().get() as u64) << Self::FILE_ID_SHIFT) | low;
        Self(NonZeroU64::new(bits).unwrap())
    }

    /// Whether the span is detached.
    pub const fn is_detached(self) -> bool {
        self.0.get() == Self::DETACHED
    }

    /// The id of the file the span points into.
    ///
    /// Returns `None` if the span is detached.
    pub const fn id(self) -> Option<FileId> {
        use std::num::NonZeroU16;
        match NonZeroU16::new((self.0.get() >> Self::FILE_ID_SHIFT) as u16) {
            Some(v) => Some(FileId::from_raw(v)),
            None => None,
        }
    }

    /// The unique number of the span within its source.
    pub(crate) const fn number(self) -> u64 {
        self.0.get() & Self::NUMBER_MASK
    }

    /// Extract a raw byte range from the span, if it is a raw range span.
    pub const fn range(self) -> Option<Range<usize>> {
        let Some(number) = self.number().checked_sub(Self::RANGE_BASE) else {
            return None;
        };
        let start = (number >> Self::RANGE_PART_SHIFT) as usize;
        let end = (number & Self::RANGE_PART_MASK) as usize;
        Some(start..end)
    }

    /// Extract the raw underlying number.
    pub const fn into_raw(self) -> NonZeroU64 {
        self.0
    }

    /// Return `other` if `self` is detached and `self` otherwise.
    pub fn or(self, other: Self) -> Self {
        if self.is_detached() { other } else { self }
    }

    /// Find the first non-detached span in the iterator.
    pub fn find(iter: impl IntoIterator<Item = Self>) -> Self {
        iter.into_iter()
            .find(|span| !span.is_detached())
            .unwrap_or(Span::detached())
    }
}

/// A value with a span locating it in the source code.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Spanned<T> {
    /// The spanned value.
    pub v: T,
    /// The value's location in source code.
    pub span: Span,
}

impl<T> Spanned<T> {
    /// Create a new instance from a value and its span.
    pub const fn new(v: T, span: Span) -> Self {
        Self { v, span }
    }

    /// Create a new instance with a span that does not point into any file.
    pub const fn detached(v: T) -> Self {
        Self { v, span: Span::detached() }
    }

    /// Convert from `&Spanned<T>` to `Spanned<&T>`
    pub const fn as_ref(&self) -> Spanned<&T> {
        Spanned { v: &self.v, span: self.span }
    }

    /// Map the value using a function.
    pub fn map<F, U>(self, f: F) -> Spanned<U>
    where
        F: FnOnce(T) -> U,
    {
        Spanned { v: f(self.v), span: self.span }
    }
}

impl<T: Debug> Debug for Spanned<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.v.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU16;

    use super::super::file_id::FileId;
    use super::{Span, Spanned};

    fn id(n: u16) -> FileId {
        FileId::from_raw(NonZeroU16::new(n).unwrap())
    }

    #[test]
    fn detached_span() {
        let span = Span::detached();
        assert!(span.is_detached());
        assert_eq!(span.id(), None);
        assert_eq!(span.range(), None);
    }

    #[test]
    fn number_encoding() {
        let file = id(5);
        let span = Span::from_number(file, 10).unwrap();
        assert_eq!(span.id(), Some(file));
        assert_eq!(span.number(), 10);
        assert_eq!(span.range(), None);
    }

    #[test]
    fn range_encoding() {
        let file = id(u16::MAX);
        let roundtrip = |range: std::ops::Range<usize>| {
            let span = Span::from_range(file, range.clone());
            assert_eq!(span.id(), Some(file));
            assert_eq!(span.range(), Some(range));
        };

        roundtrip(0..0);
        roundtrip(177..233);
        roundtrip(0..8388607);
        roundtrip(8388606..8388607);
    }

    #[test]
    fn span_or() {
        let file = id(1);
        let a = Span::detached();
        let b = Span::from_number(file, 5).unwrap();
        assert_eq!(a.or(b), b);
        assert_eq!(b.or(a), b);
    }

    #[test]
    fn span_find() {
        let file = id(2);
        let spans = vec![
            Span::detached(),
            Span::from_number(file, 3).unwrap(),
            Span::from_number(file, 7).unwrap(),
        ];
        let found = Span::find(spans);
        assert_eq!(found.number(), 3);
    }

    #[test]
    fn spanned_new() {
        let span = Span::detached();
        let s = Spanned::new(42u32, span);
        assert_eq!(s.v, 42);
        assert_eq!(s.span, span);
    }

    #[test]
    fn spanned_map() {
        let s = Spanned::detached(10u32);
        let doubled = s.map(|v| v * 2);
        assert_eq!(doubled.v, 20);
        assert_eq!(doubled.span, Span::detached());
    }
}
