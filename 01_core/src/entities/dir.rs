//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/dir.md
//! @prompt-hash 657f9389
//! @layer L1
//! @updated 2026-04-26
//!
//! Tipo enum `Dir` (LTR / RTL / TTB / BTT) — direcção de
//! empilhamento e de fluxo de texto. Adicionado no Passo 156I
//! (Fase 2 Layout sub-passo 3, ADR-0061) como atributo de
//! `Content::Stack`. Análogo a `Parity` (P156E) — infraestrutura
//! genérica reusável.
//!
//! Réplica simplificada de
//! `lab/typst-original/crates/typst-library/src/layout/dir.rs::Dir`.
//! Vanilla expõe `Smart<Dir>` em alguns sítios; cristalino usa
//! `Dir` directo (default `TTB` para stack).

/// Direcção de empilhamento ou fluxo (4 cardinais).
///
/// - `LTR`: left-to-right (texto ocidental, stack horizontal).
/// - `RTL`: right-to-left (árabe, hebraico).
/// - `TTB`: top-to-bottom (default stack vertical).
/// - `BTT`: bottom-to-top (raro; equações empilhadas inversas).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    LTR,
    RTL,
    TTB,
    BTT,
}

impl Dir {
    /// `true` se a direcção é horizontal (LTR ou RTL).
    pub fn is_horizontal(self) -> bool {
        matches!(self, Dir::LTR | Dir::RTL)
    }

    /// `true` se a direcção é vertical (TTB ou BTT).
    pub fn is_vertical(self) -> bool {
        matches!(self, Dir::TTB | Dir::BTT)
    }

    /// `true` se a direcção é "reverse" (RTL ou BTT — direcção
    /// negativa face ao sistema de coordenadas Y-down/X-right).
    pub fn is_reverse(self) -> bool {
        matches!(self, Dir::RTL | Dir::BTT)
    }
}

impl Default for Dir {
    /// Default `TTB` — coerente com vanilla `stack()` default.
    fn default() -> Self {
        Self::TTB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dir_default_e_ttb() {
        assert_eq!(Dir::default(), Dir::TTB);
    }

    #[test]
    fn is_horizontal_vs_vertical() {
        assert!(Dir::LTR.is_horizontal());
        assert!(Dir::RTL.is_horizontal());
        assert!(!Dir::TTB.is_horizontal());
        assert!(!Dir::BTT.is_horizontal());

        assert!(Dir::TTB.is_vertical());
        assert!(Dir::BTT.is_vertical());
        assert!(!Dir::LTR.is_vertical());
        assert!(!Dir::RTL.is_vertical());
    }

    #[test]
    fn is_reverse() {
        assert!(!Dir::LTR.is_reverse());
        assert!(Dir::RTL.is_reverse());
        assert!(!Dir::TTB.is_reverse());
        assert!(Dir::BTT.is_reverse());
    }

    #[test]
    fn dir_partial_eq() {
        assert_eq!(Dir::LTR, Dir::LTR);
        assert_ne!(Dir::LTR, Dir::RTL);
    }
}
