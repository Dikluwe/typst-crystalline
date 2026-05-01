//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/metadata_store.md
//! @prompt-hash e976de26
//! @layer L1
//! @updated 2026-04-30
//!
//! `MetadataStore` — sub-store de `Introspector` para feature
//! `metadata(value)` vanilla. P169 (M9 sub-passo 1).
//!
//! Acumula `Value`s embebidos no documento via `Content::Metadata`.
//! Read-only após construção. Mutação só via `pub(crate) fn add`
//! durante construção em `from_tags`.

use crate::entities::value::Value;

/// Acumulador de metadata values indexados por ordem de aparecimento.
///
/// `from_tags` adiciona entries quando processa `Tag::Start(_, info)`
/// onde `info.payload == ElementPayload::Metadata { value }`.
///
/// Order-preserving: `query()` retorna na ordem em que metadata
/// aparece no walk.
#[derive(Debug, Clone, Default)]
pub struct MetadataStore {
    values: Vec<Value>,
}

impl MetadataStore {
    /// Cria store vazio.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Slice de values na ordem de aparecimento.
    pub fn query(&self) -> &[Value] {
        &self.values
    }

    /// Número de entradas acumuladas.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// `true` se nenhum metadata foi adicionado.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Adiciona um value. Apenas usado pelo construtor `from_tags`.
    pub(crate) fn add(&mut self, value: Value) {
        self.values.push(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_devolve_vazio() {
        let s = MetadataStore::empty();
        assert!(s.is_empty());
        assert_eq!(s.len(), 0);
        assert_eq!(s.query(), &[]);
    }

    #[test]
    fn add_e_query_round_trip() {
        let mut s = MetadataStore::empty();
        s.add(Value::Int(42));
        assert_eq!(s.len(), 1);
        assert_eq!(s.query(), &[Value::Int(42)]);
    }

    #[test]
    fn add_preserva_ordem() {
        let mut s = MetadataStore::empty();
        s.add(Value::Int(1));
        s.add(Value::Int(2));
        s.add(Value::Int(3));
        assert_eq!(s.query(), &[Value::Int(1), Value::Int(2), Value::Int(3)]);
    }

    #[test]
    fn add_aceita_values_heterogeneos() {
        use ecow::EcoString;
        let mut s = MetadataStore::empty();
        s.add(Value::Int(7));
        s.add(Value::Str(EcoString::from("hello")));
        s.add(Value::Bool(true));
        assert_eq!(s.len(), 3);
    }
}
