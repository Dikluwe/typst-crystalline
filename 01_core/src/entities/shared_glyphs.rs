//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/shared_glyphs.md
//! @prompt-hash e48087bd
//! @layer L1
//! @updated 2026-11-07

use std::fmt;
use std::ops::Deref;
use std::sync::Arc;

use crate::entities::shaped_glyph::ShapedGlyph;

/// Immutable shaped glyphs shared by caches and positioned occurrences.
#[derive(Clone, PartialEq, Default)]
pub struct SharedGlyphs(Arc<[ShapedGlyph]>);

impl fmt::Debug for SharedGlyphs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl From<Vec<ShapedGlyph>> for SharedGlyphs {
    fn from(glyphs: Vec<ShapedGlyph>) -> Self {
        Self(Arc::from(glyphs))
    }
}

impl Deref for SharedGlyphs {
    type Target = [ShapedGlyph];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<[ShapedGlyph]> for SharedGlyphs {
    fn as_ref(&self) -> &[ShapedGlyph] {
        self
    }
}

impl<'a> IntoIterator for &'a SharedGlyphs {
    type Item = &'a ShapedGlyph;
    type IntoIter = std::slice::Iter<'a, ShapedGlyph>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests;
