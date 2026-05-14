//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/introspect/extract_payload.md
//! @prompt-hash 68404d88
//! @layer L1
//! @updated 2026-04-30
//!
//! `extract_payload` — função pura `&Content → Option<ElementPayload>`.
//! P162 sub-passo .D. Consumida pelo walk em P162 .E.

use crate::entities::content::Content;
use crate::entities::content_hash::hash_content;
use crate::entities::counter_update::CounterUpdate;
use crate::entities::element_payload::ElementPayload;
use crate::entities::state_update::StateUpdate;
use crate::entities::value::Value;

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

        // P240 (M9d/M7+1) — state.display(key, callback) feature.
        // Walk emite Tag aqui; `apply_state_displays` pós-fixpoint
        // (paralelo `apply_state_funcs` P191B) pre-renderiza Content
        // resultado callback aplicada ao state value at this loc.
        Content::StateDisplay { key, callback } => Some(ElementPayload::StateDisplay {
            key:      key.clone(),
            callback: callback.clone(),
        }),

        // P182C (M9) — SetHeadingNumbering emite StateUpdate sob chave
        // canónica `numbering_active:heading`. Reusa infra P171/P173 —
        // sem novo ElementPayload variant; `from_tags` arm StateUpdate
        // popula StateRegistry. Walk arm canonical em
        // `introspect.rs:455–457` continua write paralelo legacy
        // (M6 elimina). Suporte ao plano P182 (lacuna #4).
        Content::SetHeadingNumbering { active } => Some(ElementPayload::StateUpdate {
            key:    "numbering_active:heading".to_string(),
            update: StateUpdate::Set(Box::new(Value::Bool(*active))),
        }),

        // P199B — SetEquationNumbering emite StateUpdate sob chave
        // canónica `numbering_active:equation`. Reusa arm genérica
        // `from_tags::StateUpdate` (P171/P173). Cenário α por
        // construção (ADR-0069): toda infraestrutura downstream já
        // pronta (Layouter equation.rs:32-33 substitution-with-fallback
        // antes adormecida). Walk arm canonical em
        // `introspect.rs:Content::SetEquationNumbering` continua
        // write paralelo legacy (M6 elimina). Fecha Reserva 1 desde
        // P189B (E1).
        Content::SetEquationNumbering { active } => Some(ElementPayload::StateUpdate {
            key:    "numbering_active:equation".to_string(),
            update: StateUpdate::Set(Box::new(Value::Bool(*active))),
        }),

        // P178 — Outline é unit. Payload também unit. Fecha lacuna #7.
        Content::Outline => Some(ElementPayload::Outline),

        // P181D — Bibliography promovida a locatable (decisão P181A
        // cláusula 4 = Opção β). Captura entries completos por simetria
        // com walk arm actual `state.bib_entries.extend(...)`. `title`
        // ignorado por não ser relevante para introspecção.
        Content::Bibliography { entries, .. } => Some(ElementPayload::Bibliography {
            entries: entries.clone(),
        }),

        // P186C — Equation arm em estado intermédio. Arm declarado mas
        // **latente**: `is_locatable(Content::Equation)` ainda retorna
        // `false` (P186D activa), logo walk de introspect não chama
        // este arm. Inversão da ordem original (era P186D) preserva
        // sincronização-por-construção da ADR-0068 — sem janela em
        // que Layouter avança Locator para Equation enquanto walk não
        // emite tag.
        // `body` ignorado (não relevante para counter); `block`
        // propagado para gate em `from_tags` arm Equation (P186E).
        Content::Equation { block, .. } => Some(ElementPayload::Equation {
            block:          *block,
            counter_update: CounterUpdate::Step,
        }),

        // P198C — CounterUpdate promovido a locatable (cenário
        // β-promote ADR-0069). Arm emite payload com (key, action)
        // pré-recursão. `from_tags` arm CounterUpdate aplica a
        // CounterRegistry via `apply_at` ou `apply_hierarchical_at`
        // conforme key/action. Walk arm legacy (E6 P189B) preservado
        // como write paralelo M5 porque `compute_*` helpers lêem
        // `state.flat`/`hierarchical` durante walk; cleanup em M6.
        Content::CounterUpdate { key, action } => Some(ElementPayload::CounterUpdate {
            key:    key.clone(),
            action: action.clone(),
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

    #[test]
    fn outline_produz_some_payload() {
        // P178: Content::Outline → Some(ElementPayload::Outline).
        let c = Content::Outline;
        assert_eq!(extract_payload(&c), Some(ElementPayload::Outline));
    }

    // ── P181D — Bibliography arm ────────────────────────────────────────

    fn bib_entry(key: &str) -> crate::entities::bib_entry::BibEntry {
        crate::entities::bib_entry::BibEntry {
            key:          key.to_string(),
            author:       String::new(),
            title:        String::new(),
            year:         0,
            volume:       None,
            pages:        None,
            journal:      None,
            publisher:    None,
            url:          None,
            doi:          None,
            editor:       None,
            series:       None,
            note:         None,
            isbn:         None,
            location:     None,
            organization: None,
        }
    }

    #[test]
    fn bibliography_produz_some_payload_com_entries() {
        let c = Content::Bibliography {
            entries: vec![bib_entry("smith2024")],
            title:   None,
        };
        match extract_payload(&c) {
            Some(ElementPayload::Bibliography { entries }) => {
                assert_eq!(entries.len(), 1);
                assert_eq!(entries[0].key, "smith2024");
            }
            other => panic!("esperado Some(Bibliography), obtido {other:?}"),
        }
    }

    #[test]
    fn bibliography_clona_entries_para_payload() {
        let c = Content::Bibliography {
            entries: vec![bib_entry("a"), bib_entry("b"), bib_entry("c")],
            title:   None,
        };
        let payload = extract_payload(&c).expect("bibliography deve produzir Some");
        if let ElementPayload::Bibliography { entries } = payload {
            assert_eq!(entries.len(), 3);
            assert_eq!(entries[0].key, "a");
            assert_eq!(entries[1].key, "b");
            assert_eq!(entries[2].key, "c");
        } else {
            panic!("variant errado");
        }
    }

    #[test]
    fn bibliography_com_title_continua_a_extrair_apenas_entries() {
        // P181D ignora `title` — apenas `entries` entra no payload.
        // Layouter (P181G+) continuará a renderizar `title` via path
        // separado se necessário.
        let c = Content::Bibliography {
            entries: vec![bib_entry("k")],
            title:   Some(Box::new(Content::Empty)),
        };
        match extract_payload(&c) {
            Some(ElementPayload::Bibliography { entries }) => {
                assert_eq!(entries.len(), 1);
            }
            other => panic!("esperado Some(Bibliography), obtido {other:?}"),
        }
    }

    // ── P182C — SetHeadingNumbering arm ──────────────────────────────────

    #[test]
    fn set_heading_numbering_active_true_produz_state_update_bool_true() {
        let c = Content::SetHeadingNumbering { active: true };
        match extract_payload(&c) {
            Some(ElementPayload::StateUpdate { key, update }) => {
                assert_eq!(key, "numbering_active:heading");
                match update {
                    StateUpdate::Set(boxed) => assert_eq!(*boxed, Value::Bool(true)),
                    other => panic!("esperado StateUpdate::Set, obtido {other:?}"),
                }
            }
            other => panic!("esperado Some(StateUpdate), obtido {other:?}"),
        }
    }

    #[test]
    fn set_heading_numbering_active_false_produz_state_update_bool_false() {
        let c = Content::SetHeadingNumbering { active: false };
        match extract_payload(&c) {
            Some(ElementPayload::StateUpdate { key, update }) => {
                assert_eq!(key, "numbering_active:heading");
                match update {
                    StateUpdate::Set(boxed) => assert_eq!(*boxed, Value::Bool(false)),
                    other => panic!("esperado StateUpdate::Set, obtido {other:?}"),
                }
            }
            other => panic!("esperado Some(StateUpdate), obtido {other:?}"),
        }
    }

    // ── P186C — Equation arm ─────────────────────────────────────────────

    #[test]
    fn equation_block_true_produz_some_payload() {
        let c = Content::Equation {
            body:  Box::new(Content::Empty),
            block: true,
        };
        match extract_payload(&c) {
            Some(ElementPayload::Equation { block, counter_update }) => {
                assert!(block);
                assert_eq!(counter_update, CounterUpdate::Step);
            }
            other => panic!("esperado Some(Equation), obtido {other:?}"),
        }
    }

    #[test]
    fn equation_block_false_propaga_flag() {
        // Inline equation: gate em P186E (block && state-active) vai
        // bloquear; payload preserva block=false para downstream.
        let c = Content::Equation {
            body:  Box::new(Content::Empty),
            block: false,
        };
        match extract_payload(&c) {
            Some(ElementPayload::Equation { block, counter_update }) => {
                assert!(!block);
                assert_eq!(counter_update, CounterUpdate::Step);
            }
            other => panic!("esperado Some(Equation), obtido {other:?}"),
        }
    }

    #[test]
    fn equation_body_e_ignorado() {
        // body distinto não afecta payload — só block é capturado.
        let c1 = Content::Equation {
            body:  Box::new(Content::Empty),
            block: true,
        };
        let c2 = Content::Equation {
            body:  Box::new(Content::Text(EcoString::from("E=mc^2"), Default::default())),
            block: true,
        };
        assert_eq!(extract_payload(&c1), extract_payload(&c2));
    }
}
