//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/stdlib/state.md
//! @prompt-hash 0b933904
//! @layer L1
//! @updated 2026-06-30
//!
//! `State` — valor de primeira classe representando estado documental
//! mutável. P506 — runtime state via `context`.

use ecow::EcoString;

use crate::entities::value::Value;

/// Estado documental identificado por `key` com valor inicial `init`.
#[derive(Debug, Clone, PartialEq)]
pub struct State {
    pub key:  EcoString,
    pub init: Box<Value>,
}
