//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/lang.md
//! @prompt-hash 4426dbc0
//! @layer L1
//! @updated 2026-04-25
//!
//! Regras lang-aware (ADR-0057 / ADR-0060 Fase 1).
//!
//! Módulo agrupa funcionalidades cujo comportamento depende do
//! `text.lang` activo. Inicialmente contém apenas `quotes` (smart-quotes,
//! Passo 155); hyphenation continua em `rules/layout/hyphenation.rs`
//! (refactor de unificação adiado a passo separado se priorizado).

pub mod quotes;
