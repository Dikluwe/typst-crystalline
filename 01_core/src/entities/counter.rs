//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/counter.md
//! @prompt-hash bb6d2ad1
//! @layer L1
//! @updated 2026-08-12
//!
//! `Counter` — valor de primeira classe representando counter documental.
//! P506 — runtime state via `context`.
//! **P1018** — `CounterKey` separa string de utilizador (`"foo"`) de
//! selector de elemento (`heading`), espelhando o vanilla.

use ecow::EcoString;

use crate::entities::selector::Selector;

/// Chave de counter documental. Espelha `CounterKey` do vanilla
/// (`typst-library/src/introspection/counter.rs:527`):
/// - `Str`: contador manual identificado por string de utilizador.
/// - `Selector`: contador automático de elemento locatable.
/// - `Page`: contador de página (paridade futura; reservado).
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum CounterKey {
    Page,
    Selector(Selector),
    Str(EcoString),
}

// `Eq` manual (marker trait): `Selector` contém `Value` com `f64`, que não
// é `Eq` por causa de NaN, mas em prática nenhum counter key transporta
// NaN. White-lie consistente com `ElementPayload::Eq` (P169).
impl Eq for CounterKey {}

impl From<String> for CounterKey {
    fn from(s: String) -> Self {
        CounterKey::Str(s.into())
    }
}

impl From<&str> for CounterKey {
    fn from(s: &str) -> Self {
        CounterKey::Str(s.into())
    }
}

impl From<EcoString> for CounterKey {
    fn from(s: EcoString) -> Self {
        CounterKey::Str(s)
    }
}

/// Counter documental identificado por `key`.
#[derive(Debug, Clone, PartialEq)]
pub struct Counter {
    pub key: CounterKey,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::element_kind::ElementKind;

    #[test]
    fn counter_key_str_e_selector_sao_distintos() {
        // **P1018** — `counter("heading")` (manual) e `counter(heading)`
        // (automático) não podem colidir no registry.
        let manual = CounterKey::Str("heading".into());
        let auto = CounterKey::Selector(Selector::Kind(ElementKind::Heading));
        assert_ne!(manual, auto);

        let mut map = std::collections::HashMap::new();
        map.insert(manual.clone(), 1);
        map.insert(auto.clone(), 2);
        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&manual), Some(&1));
        assert_eq!(map.get(&auto), Some(&2));
    }

    #[test]
    fn counter_key_page_isolado() {
        let page = CounterKey::Page;
        let also_page = CounterKey::Page;
        let not_page = CounterKey::Str("page".into());
        assert_eq!(page, also_page);
        assert_ne!(page, not_page);
    }

    #[test]
    fn counter_construcao_via_from() {
        let c = Counter { key: CounterKey::from("foo") };
        assert!(matches!(c.key, CounterKey::Str(s) if s == "foo"));

        let c2 = Counter { key: CounterKey::from(String::from("bar")) };
        assert!(matches!(c2.key, CounterKey::Str(s) if s == "bar"));
    }
}
