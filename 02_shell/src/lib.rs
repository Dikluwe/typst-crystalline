//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/shell.md
//! @prompt-hash 3b6afeab
//! @layer L2
//! @updated 2026-04-23
//!
//! Shell — interface com utilizador (CLI, argparsing, formatters
//! de output high-level). Materializado no Passo 117 (ADR-0049)
//! depois de 4 passos (113–116) em que CLI viveu incorrectamente
//! em L3 e L4.

pub mod cli;
pub mod diagnostic;
