//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/introspect/extract_payload.md
//! @prompt-hash 493cdaed
//! @layer L1
//! @updated 2026-04-30
//!
//! `extract_payload` — função pura `&Content → Option<ElementPayload>`.
//! P162 sub-passo .D. Consumida pelo walk em P162 .E.

use crate::entities::content::Content;
use crate::entities::content_hash::hash_content;
use crate::entities::counter_update::CounterUpdate;
use crate::entities::element_payload::ElementPayload;

/// Extrai o `ElementPayload` correspondente a um `Content`, se for
/// uma variante locatable (Heading/Figure/Cite em M1).
pub fn extract_payload(content: &Content) -> Option<ElementPayload> {
    match content {
        Content::Heading { level, body } => Some(ElementPayload::Heading {
            depth:          *level,
            body_hash:      hash_content(body),
            counter_update: CounterUpdate::Step,
        }),

        Content::Figure { kind, numbering, caption, .. } => Some(ElementPayload::Figure {
            kind:           kind.clone(),
            counter_update: CounterUpdate::Step,
            // P168 (M5 sub-passo 2): figura conta para numeração apenas
            // se tiver `numbering` E `caption` (paridade com walk arm
            // `Content::Labelled` em introspect.rs:366).
            is_counted:     numbering.is_some() && caption.is_some(),
        }),

        Content::Cite { key, .. } => Some(ElementPayload::Citation {
            key: key.clone(),
        }),

        // P169 (M9 sub-passo 1) — metadata(value) feature.
        Content::Metadata { value } => Some(ElementPayload::Metadata {
            value: value.clone(),
        }),

        // P171 (M9 sub-passo 3) — state(key, init) feature.
        Content::State { key, init } => Some(ElementPayload::State {
            key:  key.clone(),
            init: init.clone(),
        }),

        // P171 (M9 sub-passo 3) — state.update(key, value) feature.
        Content::StateUpdate { key, update } => Some(ElementPayload::StateUpdate {
            key:    key.clone(),
            update: update.clone(),
        }),

        // Todas as outras variantes não são locatable em M1.
        // Adicionar uma variant locatable nova exige edição explícita
        // deste match (compilador não força exaustividade aqui porque
        // usamos catch-all `_`, mas o L0 mandata revisão).
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecow::EcoString;

    #[test]
    fn heading_produz_some_payload() {
        let c = Content::Heading {
            level: 2,
            body:  Box::new(Content::Text(EcoString::from("Section"), Default::default())),
        };
        match extract_payload(&c) {
            Some(ElementPayload::Heading { depth, body_hash, counter_update }) => {
                assert_eq!(depth, 2);
                assert_ne!(body_hash, 0); // hash de "Section" é não-zero
                assert_eq!(counter_update, CounterUpdate::Step);
            }
            other => panic!("esperado Some(Heading), obtido {other:?}"),
        }
    }

    #[test]
    fn figure_produz_some_payload() {
        let c = Content::Figure {
            body:      Box::new(Content::Empty),
            caption:   None,
            kind:      Some("image".into()),
            numbering: None,
        };
        match extract_payload(&c) {
            Some(ElementPayload::Figure { kind, counter_update, is_counted: _ }) => {
                assert_eq!(kind, Some("image".to_string()));
                assert_eq!(counter_update, CounterUpdate::Step);
            }
            other => panic!("esperado Some(Figure), obtido {other:?}"),
        }
    }

    #[test]
    fn figure_kind_none_preserva_none() {
        let c = Content::Figure {
            body:      Box::new(Content::Empty),
            caption:   None,
            kind:      None,
            numbering: None,
        };
        match extract_payload(&c) {
            Some(ElementPayload::Figure { kind, .. }) => assert_eq!(kind, None),
            other => panic!("esperado Some(Figure), obtido {other:?}"),
        }
    }

    #[test]
    fn cite_produz_some_payload() {
        let c = Content::Cite {
            key:        "smith2024".to_string(),
            supplement: None,
            form:       None,
        };
        match extract_payload(&c) {
            Some(ElementPayload::Citation { key }) => {
                assert_eq!(key, "smith2024");
            }
            other => panic!("esperado Some(Citation), obtido {other:?}"),
        }
    }

    #[test]
    fn text_produz_none() {
        let c = Content::Text(EcoString::from("plain"), Default::default());
        assert_eq!(extract_payload(&c), None);
    }

    #[test]
    fn empty_e_space_produzem_none() {
        assert_eq!(extract_payload(&Content::Empty), None);
        assert_eq!(extract_payload(&Content::Space), None);
    }

    #[test]
    fn sequence_e_outras_produzem_none() {
        let seq = Content::Sequence(std::sync::Arc::from(vec![Content::Empty]));
        assert_eq!(extract_payload(&seq), None);
    }
}
