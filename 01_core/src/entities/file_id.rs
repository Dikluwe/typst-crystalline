//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/file-id.md
//! @prompt-hash 601326dc
//! @layer L1
//! @updated 2026-03-22

use std::fmt::{self, Debug, Formatter};
use std::num::NonZeroU16;

/// An opaque handle identifying a file in the Typst compiler.
///
/// In the original Typst codebase, `FileId` wraps a global interner that maps
/// `RootedPath → NonZeroU16`. That interner uses `static LazyLock<RwLock<...>>`
/// which violates V13. In L1 we keep only the opaque handle; the interner lives
/// in L3.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct FileId(NonZeroU16);

impl FileId {
    /// Construct from a raw number.
    ///
    /// Should only be used with numbers retrieved via [`into_raw`](Self::into_raw).
    pub const fn from_raw(v: NonZeroU16) -> Self {
        Self(v)
    }

    /// Extract the raw underlying number.
    pub const fn into_raw(self) -> NonZeroU16 {
        self.0
    }
}

impl Debug for FileId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "FileId({})", self.0)
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU16;

    use super::FileId;

    #[test]
    fn roundtrip() {
        let raw = NonZeroU16::new(42).unwrap();
        let id = FileId::from_raw(raw);
        assert_eq!(id.into_raw(), raw);
    }

    #[test]
    fn equality() {
        let a = FileId::from_raw(NonZeroU16::new(1).unwrap());
        let b = FileId::from_raw(NonZeroU16::new(1).unwrap());
        let c = FileId::from_raw(NonZeroU16::new(2).unwrap());
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn is_copy() {
        let id = FileId::from_raw(NonZeroU16::new(7).unwrap());
        let copy = id;
        assert_eq!(id, copy);
    }
}
