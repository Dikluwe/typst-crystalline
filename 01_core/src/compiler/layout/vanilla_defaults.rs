//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/vanilla_defaults.md
//! @prompt-hash 5266f93f
//! @layer L1
//! @updated 2026-08-15
//!
//! Constantes canónicas de layout extraídas do vanilla Typst com proveniência
//! rastreável e fundamentação tipográfica (Passo 1058).

/// Espaçamento vertical por defeito entre parágrafos distintos (em unidades `em`).
/// ref: lab/typst-original/crates/typst-library/src/model/par.rs:224
/// Confirmado por medição directa (P1057): delta de -6.05pt eliminado ao usar este
/// valor em vez de PAR_LEADING para o gap entre parágrafos.
pub const PAR_SPACING: f64 = 1.2;

/// Espaçamento entre linhas dentro do mesmo parágrafo (leading, em unidades `em`).
/// ref: lab/typst-original/crates/typst-library/src/model/par.rs:210
pub const PAR_LEADING: f64 = 0.65;

/// Espaçamento por defeito de blocos genéricos (equation, divider, etc.) quando não
/// participam do fluxo de parágrafo directamente (em unidades `em`).
/// ref: lab/typst-original/crates/typst-library/src/layout/container.rs:342
pub const BLOCK_SPACING: f64 = 1.2;
