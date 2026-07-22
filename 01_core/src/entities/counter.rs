//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/stdlib/counter.md
//! @prompt-hash 8a270829
//! @layer L1
//! @updated 2026-06-30
//!
//! `Counter` — valor de primeira classe representando counter documental.
//! P506 — runtime state via `context`.

use ecow::EcoString;

/// Counter documental identificado por `key` (ex: "heading", "figure").
#[derive(Debug, Clone, PartialEq)]
pub struct Counter {
    pub key: EcoString,
}
