//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/module.md
//! @prompt-hash bb646d26
//! @layer L1
//! @updated 2026-03-26

/// Resultado da avaliação de um ficheiro Typst.
///
/// Stub opaco — interior definido quando `typst-library/foundations/`
/// for analisada e `Module` real for migrado.
/// Ver ADR-0016.
pub struct Module(());

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_stub_exists() {
        // Contrato correcto — stub opaco compila e existe como tipo
        let _ = std::mem::size_of::<Module>();
    }
}
