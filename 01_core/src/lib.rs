//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core.md
//! @prompt-hash 3a4fa3da
//! @layer L1
//! @updated 2026-03-22

pub mod contracts;
pub mod engine;
pub mod entities;
// **P969** — oráculo de fórmulas do vanilla: só existe em builds de teste
// (ferramenta de verificação, nunca caminho de execução — ver
// `00_nucleo/prompts/testing/math_oracle.md`).
#[cfg(test)]
pub(crate) mod testing;
pub(crate) mod utils;
