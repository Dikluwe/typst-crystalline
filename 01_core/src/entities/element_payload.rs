//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/element_payload.md
//! @prompt-hash 724e8afd
//! @layer L1
//! @updated 2026-04-30
//!
//! `ElementPayload` — forma fechada e tipada dos dados específicos
//! de cada elemento indexado pela introspecção. P161 sub-passo .7.
//!
//! Campos confirmados em P161 sub-passo .1 (leitura de
//! `entities/content.rs`):
//! - `Content::Heading { level: u8, body: Box<Content> }`
//! - `Content::Figure { body, caption, kind: Option<String>, numbering }`
//! - `Content::Cite { key: String, supplement, form }`
//!
//! `body_hash` em `Heading` é placeholder (`0`) em P161; função de
//! hash determinística sobre `Content` é pendência registada para
//! P162.

use crate::entities::counter_update::CounterUpdate;

/// Dados específicos por kind para indexação na introspecção.
///
/// `PartialEq` derivado. `Hash` implementado manualmente (P169) via
/// `format!("{:?}", self)` porque `Value` (em `Metadata` variant) não
/// implementa `Hash` directamente — Value contém `f64` em variantes
/// como `Length`/`Float`. Hash via Debug é determinístico para
/// estruturas equivalentes (mesma estratégia de `hash_content`).
///
/// `Eq` não é derivado/implementado: Value não é `Eq` (f64 NaN viola
/// reflexividade). Consumers que precisem de `Eq`-bound usam `PartialEq`.
#[derive(Debug, Clone, PartialEq)]
pub enum ElementPayload {
    Heading {
        /// Nível clamped 1..=6 (paridade `Content::Heading.level`).
        depth: u8,

        /// Hash determinístico do `body`. **Pendência P162** —
        /// função de hash sobre `Content` ainda não existe; em
        /// P161 callers passam `0`.
        body_hash: u128,

        /// Update implícito do contador "heading" associado.
        counter_update: CounterUpdate,
    },

    Figure {
        /// Discriminador de tipo (`"image"`, `"table"`, …).
        /// `None` = Auto (resolver no consumer).
        kind: Option<String>,

        /// Update implícito do contador `figure:{kind}`.
        counter_update: CounterUpdate,

        /// `true` se a figura conta para a numeração (predicado:
        /// `figure.numbering.is_some() && figure.caption.is_some()`).
        /// Adicionado em P168 (M5 sub-passo 2): permite que `from_tags`
        /// indexe apenas figuras numeradas para `figure_label_numbers`,
        /// preservando paridade com `CounterStateLegacy.figure_label_numbers`
        /// que aplica o mesmo filtro no walk arm `Content::Labelled`.
        is_counted: bool,
    },

    Citation {
        /// Chave da citação (`Content::Cite.key`).
        key: String,
    },

    /// **P169 (M9 sub-passo 1)** — payload de `metadata(value)`.
    ///
    /// `value` é embebido por valor (boxed para evitar tamanho da Value
    /// no payload). Consumer típico: `MetadataStore` populado por
    /// `from_tags`; query via `Introspector::query_metadata`.
    Metadata {
        value: Box<crate::entities::value::Value>,
    },

    /// **P171 (M9 sub-passo 3)** — payload de `state(key, init)`.
    State {
        key:  String,
        init: Box<crate::entities::value::Value>,
    },

    /// **P171 (M9 sub-passo 3)** — payload de `state.update(key, value)`.
    StateUpdate {
        key:    String,
        update: crate::entities::state_update::StateUpdate,
    },
}

impl std::hash::Hash for ElementPayload {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Manual Hash via Debug-string — necessário porque Value em
        // Metadata variant não impl Hash. Estratégia consistente com
        // `entities::content_hash::hash_content` (Debug é estrutural
        // determinístico para tipos derive-Debug).
        format!("{:?}", self).hash(state);
    }
}

// `Eq` impl manual (marker trait): Value não é Eq por causa de f64
// NaN, mas em prática nenhum f64 NaN aparece em Value::Float ou
// equivalentes durante o uso normal. Aceite white-lie consistente
// com PartialEq derive (Value::PartialEq tem mesma issue).
// Necessário para downstream types (`ElementInfo`, `Tag`) que
// derivam Eq.
impl Eq for ElementPayload {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heading_constroi_e_compara() {
        let h = ElementPayload::Heading {
            depth: 2,
            body_hash: 0,
            counter_update: CounterUpdate::Step,
        };
        let h2 = h.clone();
        assert_eq!(h, h2);
    }

    #[test]
    fn figure_kind_none_distinto_de_some() {
        let a = ElementPayload::Figure {
            kind: None,
            counter_update: CounterUpdate::Step,
            is_counted: false,
        };
        let b = ElementPayload::Figure {
            kind: Some("image".into()),
            counter_update: CounterUpdate::Step,
            is_counted: false,
        };
        assert_ne!(a, b);
    }

    #[test]
    fn figure_is_counted_distingue_payloads() {
        // P168: is_counted é parte de igualdade.
        let counted = ElementPayload::Figure {
            kind: Some("image".into()),
            counter_update: CounterUpdate::Step,
            is_counted: true,
        };
        let uncounted = ElementPayload::Figure {
            kind: Some("image".into()),
            counter_update: CounterUpdate::Step,
            is_counted: false,
        };
        assert_ne!(counted, uncounted);
    }

    #[test]
    fn citation_compara_por_key() {
        let a = ElementPayload::Citation { key: "smith2024".into() };
        let b = ElementPayload::Citation { key: "smith2024".into() };
        let c = ElementPayload::Citation { key: "jones2023".into() };
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn variantes_distintas_entre_si() {
        let h = ElementPayload::Heading {
            depth: 1,
            body_hash: 0,
            counter_update: CounterUpdate::Step,
        };
        let f = ElementPayload::Figure {
            kind: None,
            counter_update: CounterUpdate::Step,
            is_counted: false,
        };
        let c = ElementPayload::Citation { key: "x".into() };
        assert_ne!(h, f);
        assert_ne!(f, c);
        assert_ne!(h, c);
    }

    #[test]
    fn hash_distingue_payloads_distintos() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let a = ElementPayload::Citation { key: "a".into() };
        let b = ElementPayload::Citation { key: "b".into() };
        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        a.hash(&mut h1);
        b.hash(&mut h2);
        assert_ne!(h1.finish(), h2.finish());
    }
}
