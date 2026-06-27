//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/symbol.md
//! @prompt-hash c22fae14
//! @layer L1
//! @updated 2026-06-26

use ecow::EcoString;

/// Símbolo Unicode nomeado. Subset minimal P471.
///
/// O vanilla suporta modificadores encadeados (`sym.arrow.r.double`) via
/// `Modifier` struct. O cristalino implementa apenas nomes simples e compostos
/// pré-definidos como chaves planas (`"arrow.r"`, `"eq.not"`).
/// Divergência declarada; modificadores encadeados são scope-out futuro.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol {
    pub ch:   char,
    pub name: EcoString,
}

impl Symbol {
    pub fn new(ch: char, name: impl Into<EcoString>) -> Self {
        Self { ch, name: name.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbol_new_preserva_campos() {
        let s = Symbol::new('→', "arrow");
        assert_eq!(s.ch, '→');
        assert_eq!(s.name.as_str(), "arrow");
    }

    #[test]
    fn symbol_igualdade() {
        let a = Symbol::new('α', "alpha");
        let b = Symbol::new('α', "alpha");
        assert_eq!(a, b);
    }

    #[test]
    fn symbol_hash_via_derive() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(Symbol::new('→', "arrow"));
        set.insert(Symbol::new('→', "arrow"));
        assert_eq!(set.len(), 1);
    }
}
