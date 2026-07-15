//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra.md
//! @prompt-hash 4eecd2a1
//! @layer L3
//! @updated 2026-03-26

// `diagnostic_format` migrado para L2 (`typst_shell::diagnostic`)
// no Passo 119 (ADR-0050). L3 já não conhece formatação user-facing.
pub mod embedded_fonts;
pub mod export;
pub mod fallback_fonts;
pub mod font_metrics;
pub mod font_variant;
pub mod fontdb;
pub mod fonts;
pub mod image_sizer;
pub mod layout;
pub mod layout_bidi;
pub mod measurements;
pub mod package_downloader;
pub mod pipeline;
pub mod plugin_host;
pub mod query_helpers;
pub mod shaper;
pub mod world;

#[cfg(test)]
mod integration_tests;

#[cfg(test)]
mod p307b_snapshot_tests;
