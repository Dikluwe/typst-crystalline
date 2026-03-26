//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/value.md
//! @prompt-hash 8229696a
//! @layer L1
//! @updated 2026-03-26

/// Valor em tempo de avaliação do Typst.
///
/// Stub opaco — interior definido quando `typst-library/foundations/`
/// for analisada e `Value` real for migrado.
/// Ver ADR-0017.
pub struct Value(());

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_stub_exists() {
        // Contrato correcto — stub opaco compila e existe como tipo
        let _ = std::mem::size_of::<Value>();
    }
}
