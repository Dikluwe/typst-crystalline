//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/testing/math_oracle.md
//! @prompt-hash 745c153a
//! @layer L1
//! @updated 2026-08-05
//!
//! Oráculo de fórmulas de posição do vanilla (P969) — módulo de
//! verificação para testes, **não** caminho de execução de produção
//! (`#[cfg(test)]`). Ver o prompt para as regras do módulo.

pub(crate) mod math_oracle;
