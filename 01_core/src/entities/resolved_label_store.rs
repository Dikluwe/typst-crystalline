//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/resolved_label_store.md
//! @prompt-hash 661dcacb
//! @layer L1
//! @updated 2026-05-04
//!
//! `ResolvedLabelStore` — sub-store de `TagIntrospector` para
//! mapeamento Label → texto resolvido. P193B (M5 sequência §9 P189
//! passo 1).
//!
//! Replica shape de `CounterStateLegacy.resolved_labels`
//! (`HashMap<Label, String>`). Read-only após construção; mutação
//! só via `pub(crate) fn` durante `from_tags` (arm de populate em
//! P195 pendente — sub-store fica **vazio em produção** até lá).

use std::collections::HashMap;

use crate::entities::label::Label;

/// Mapeamento Label → texto resolvido para cross-references.
///
/// Populated por `from_tags` (P195 pendente) ao processar
/// `Tag::Start` para `Content::Heading` (auto-toc) e
/// `Content::Labelled` (explicit). Consumer C4 (`layout/references.rs`)
/// migra em P194 para consultar via trait
/// `Introspector::resolved_label_for(&Label)`.
///
/// **Estado em P193B**: vazio em produção. Janela compat M5 —
/// walk arms (E2/E4 P189B) continuam a popular
/// `state.resolved_labels` legacy directamente; consumer C4 lê
/// legacy. Sub-store novo activa em P195 (arm em `from_tags`)
/// + P194 (consumer migrado).
#[derive(Debug, Clone, Default)]
pub struct ResolvedLabelStore {
    labels: HashMap<Label, String>,
}

impl ResolvedLabelStore {
    /// Cria store vazio. Equivalente a `Default::default()`.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Lookup por `Label`. `Some(&str)` se label registada;
    /// `None` caso contrário. Replica
    /// `state.resolved_labels.get(label).map(|s| s.as_str())` actual
    /// em `layout/references.rs:53` (P194 migrará caller).
    pub fn get(&self, label: &Label) -> Option<&str> {
        self.labels.get(label).map(|s| s.as_str())
    }

    /// Número de labels registadas.
    pub fn len(&self) -> usize {
        self.labels.len()
    }

    /// `true` se nenhuma label foi registada.
    pub fn is_empty(&self) -> bool {
        self.labels.is_empty()
    }

    /// Regista mapeamento Label → texto resolvido. Replica
    /// `state.resolved_labels.insert(label, text)` actual em
    /// `introspect.rs` walk arms Heading/Labelled (E2/E4 P189B).
    /// Sobrescreve valor anterior se label já registada (paridade
    /// com `HashMap::insert` legacy).
    pub(crate) fn insert(&mut self, label: Label, resolved: String) {
        self.labels.insert(label, resolved);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lbl(s: &str) -> Label {
        Label(s.to_string())
    }

    #[test]
    fn empty_store_returns_none() {
        let store = ResolvedLabelStore::empty();
        assert_eq!(store.get(&lbl("foo")), None);
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn insert_then_get() {
        let mut store = ResolvedLabelStore::empty();
        store.insert(lbl("intro"), "Capítulo 1".to_string());
        assert_eq!(store.get(&lbl("intro")), Some("Capítulo 1"));
        assert!(!store.is_empty());
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn multiple_labels_isolated() {
        let mut store = ResolvedLabelStore::empty();
        store.insert(lbl("intro"), "Secção 1".to_string());
        store.insert(lbl("metodos"), "Secção 2".to_string());
        store.insert(lbl("conclusao"), "Secção 3".to_string());

        assert_eq!(store.get(&lbl("intro")), Some("Secção 1"));
        assert_eq!(store.get(&lbl("metodos")), Some("Secção 2"));
        assert_eq!(store.get(&lbl("conclusao")), Some("Secção 3"));
        assert_eq!(store.get(&lbl("ausente")), None);
        assert_eq!(store.len(), 3);
    }

    #[test]
    fn insert_sobrescreve_label_existente() {
        // Paridade com HashMap::insert legacy — se walk arm dispara
        // duas vezes para a mesma label (improvável mas possível),
        // último valor ganha.
        let mut store = ResolvedLabelStore::empty();
        store.insert(lbl("dup"), "primeiro".to_string());
        store.insert(lbl("dup"), "segundo".to_string());
        assert_eq!(store.get(&lbl("dup")), Some("segundo"));
        assert_eq!(store.len(), 1);
    }
}
