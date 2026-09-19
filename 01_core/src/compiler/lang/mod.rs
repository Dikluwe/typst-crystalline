//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/lang.md
//! @prompt-hash c53da034
//! @layer L1
//! @updated 2026-04-25
//!
//! Regras lang-aware (ADR-0057 / ADR-0060 Fase 1).
//!
//! Módulo agrupa funcionalidades cujo comportamento depende do
//! `text.lang` activo. Inicialmente contém apenas `quotes` (smart-quotes,
//! Passo 155); hyphenation continua em `compiler/layout/hyphenation.rs`
//! (refactor de unificação adiado a passo separado se priorizado).

pub mod equation_supplement;
pub mod figure_supplement;
mod heading_supplement;
// **P1034** — título por defeito do `outline()`, localizado.
pub mod outline_title;
pub mod quotes;

pub(in crate::compiler) use heading_supplement::heading_supplement_for_lang;
