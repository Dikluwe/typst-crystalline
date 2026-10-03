//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/shaper.md
//! @prompt-hash 7be7dc42
//! @layer L3
//! @updated 2026-10-03
//! Post-layout shaping facade. Implementation remains in private L3 descendants.

#![allow(deprecated)]

mod bidi;
mod cache;
mod font_selection;
mod reconciliation;
mod shaping;
mod traversal;
mod text_identity;
pub(crate) mod tracking;

#[cfg(test)]
mod tests;

#[allow(unused_imports)] // Preserve the existing crate-visible path.
pub(crate) use cache::CachedFace;
pub(crate) use cache::FaceCache;
pub use reconciliation::fix_line_positions;
pub(crate) use reconciliation::fix_line_positions_with_metrics;
pub(crate) use shaping::shaped_width;
pub use traversal::shape_document;
