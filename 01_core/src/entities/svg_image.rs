//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/svg_image.md
//! @prompt-hash f24132d2
//! @layer L1
//! @updated 2026-11-01

use crate::entities::{file_id::FileId, path::RootedPath, span::Span};
use std::sync::Arc;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SvgOrigin {
    File(RootedPath),
    Inline(FileId),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SvgSource {
    pub origin: SvgOrigin,
    pub span: Span,
}
#[derive(Debug, Clone)]
pub struct PreparedSvg {
    pub data: Arc<Vec<u8>>,
    pub width_px: f64,
    pub height_px: f64,
    pub warnings: Vec<String>,
}
