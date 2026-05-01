//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/label_registry.md
//! @prompt-hash 8bfee760
//! @layer L1
//! @updated 2026-04-30
//!
//! `LabelRegistry` — sub-store Label→Location para `Introspector`.
//! P165 sub-passo .B (M3 Introspection).
//!
//! Read-only após construção. Mutação só via `pub(crate) fn add`
//! durante construção em `from_tags`.

use std::collections::HashMap;

use crate::entities::label::Label;
use crate::entities::location::Location;

/// Mapeamento Label → Location construído pelo motor de introspecção.
///
/// Primeira label inserida ganha; duplicadas são silenciosamente
/// ignoradas (decisão M3). M9+ pode introduzir validação stricter.
#[derive(Debug, Clone, Default)]
pub struct LabelRegistry {
    inner: HashMap<Label, Location>,
}

impl LabelRegistry {
    /// Cria registry vazio.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Lookup de label. Retorna `Some(location)` se a label foi
    /// adicionada por `from_tags`; `None` caso contrário.
    pub fn lookup(&self, label: &Label) -> Option<Location> {
        self.inner.get(label).copied()
    }

    /// Número de labels únicas registadas.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// `true` se nenhuma label foi adicionada.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Insere par `(label, location)`. Apenas usado pelo construtor
    /// `from_tags` em `rules/introspect/from_tags.rs`. Duplicados
    /// preservam a primeira location.
    pub(crate) fn add(&mut self, label: Label, location: Location) {
        self.inner.entry(label).or_insert(location);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn label(s: &str) -> Label {
        Label(s.to_string())
    }

    #[test]
    fn empty_lookup_devolve_none() {
        let r = LabelRegistry::empty();
        assert_eq!(r.lookup(&label("foo")), None);
        assert!(r.is_empty());
        assert_eq!(r.len(), 0);
    }

    #[test]
    fn add_e_lookup_round_trip() {
        let mut r = LabelRegistry::empty();
        let loc = Location::from_raw(42);
        r.add(label("intro"), loc);
        assert_eq!(r.lookup(&label("intro")), Some(loc));
        assert_eq!(r.len(), 1);
        assert!(!r.is_empty());
    }

    #[test]
    fn cinco_labels_distintos_resolvem_correctamente() {
        let mut r = LabelRegistry::empty();
        let pares: Vec<(&str, u128)> = vec![
            ("a", 1), ("b", 2), ("c", 3), ("d", 4), ("e", 5),
        ];
        for (k, raw) in &pares {
            r.add(label(k), Location::from_raw(*raw));
        }
        for (k, raw) in &pares {
            assert_eq!(r.lookup(&label(k)), Some(Location::from_raw(*raw)));
        }
        assert_eq!(r.len(), 5);
    }

    #[test]
    fn duplicada_preserva_primeira_location() {
        let mut r = LabelRegistry::empty();
        let loc1 = Location::from_raw(7);
        let loc2 = Location::from_raw(99);
        r.add(label("dup"), loc1);
        r.add(label("dup"), loc2);
        assert_eq!(r.lookup(&label("dup")), Some(loc1), "primeira ganha");
        assert_eq!(r.len(), 1);
    }

    #[test]
    fn lookup_de_label_inexistente_devolve_none() {
        let mut r = LabelRegistry::empty();
        r.add(label("real"), Location::from_raw(1));
        assert_eq!(r.lookup(&label("fake")), None);
    }
}
