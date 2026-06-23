//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/introspect/extract_payload.md
//! @prompt-hash 68404d88
//! @layer L1
//! @updated 2026-04-30
//!
//! `extract_payload` — função pura `&Content → Option<ElementPayload>`.
//! P162 sub-passo .D. Consumida pelo walk em P162 .E.

use crate::entities::content::Content;
#[cfg(test)]
use crate::entities::counter_update::CounterUpdate;
use crate::entities::element_payload::ElementPayload;
use crate::entities::elements::Element; // Modelo D (P316): trait p/ to_payload()

/// Extrai o `ElementPayload` correspondente a um `Content`, se for
/// uma variante locatable (Heading/Figure/Cite em M1).
pub fn extract_payload(content: &Content) -> Option<ElementPayload> {
    match content {
        // Modelo D (P316): absorção do locatável — o elemento fornece o payload.
        Content::Heading(h) => h.to_payload(),

        // Modelo D (Lote 13 P328): Figure locatável delega ao elemento.
        Content::Figure(e) => e.to_payload(),

        // Modelo D (Lote 9 P324): Cite locatável delega ao elemento.
        Content::Cite(e) => e.to_payload(),

        // Modelo D (Lote 6 P321): família state/counter locatável absorve o
        // payload no trait — o elemento fornece (precedente Heading P316).
        Content::Metadata(e)               => e.to_payload(),
        Content::State(e)                  => e.to_payload(),
        Content::StateUpdate(e)            => e.to_payload(),
        Content::StateDisplay(e)           => e.to_payload(),
        Content::CounterDisplayCallback(e) => e.to_payload(),

        // Lote F-2 S5 (P335): arms Set*Numbering removidos com as variantes.

        // P178 — Outline é unit. Payload também unit. Fecha lacuna #7.
        // Modelo D (Lote 8 P323): delega ao elemento.
        Content::Outline(e) => e.to_payload(),

        // P181D — Bibliography promovida a locatable (decisão P181A
        // cláusula 4 = Opção β). Captura entries completos por simetria
        // com walk arm actual `state.bib_entries.extend(...)`. `title`
        // ignorado por não ser relevante para introspecção.
        // Modelo D (Lote 10 P325): Bibliography locatável delega ao elemento.
        Content::Bibliography(e) => e.to_payload(),

        // P186C — Equation arm em estado intermédio. Arm declarado mas
        // **latente**: `is_locatable(Content::Equation)` ainda retorna
        // `false` (P186D activa), logo walk de introspect não chama
        // este arm. Inversão da ordem original (era P186D) preserva
        // sincronização-por-construção da ADR-0068 — sem janela em
        // que Layouter avança Locator para Equation enquanto walk não
        // emite tag.
        // `body` ignorado (não relevante para counter); `block`
        // propagado para gate em `from_tags` arm Equation (P186E).
        // Modelo D (Lote 10 P325): Equation locatável delega ao elemento.
        Content::Equation(e) => e.to_payload(),

        // P198C — CounterUpdate promovido a locatable (cenário
        // β-promote ADR-0069). Arm emite payload com (key, action)
        // pré-recursão. `from_tags` arm CounterUpdate aplica a
        // CounterRegistry via `apply_at` ou `apply_hierarchical_at`
        // conforme key/action. Walk arm legacy (E6 P189B) preservado
        // como write paralelo M5 porque `compute_*` helpers lêem
        // `state.flat`/`hierarchical` durante walk; cleanup em M6.
        // Modelo D (Lote 6 P321): CounterUpdate locatável delega ao elemento.
        Content::CounterUpdate(e) => e.to_payload(),

        // Lote F-1 (P334): a fronteira dinâmica delega ao elemento (S1) —
        // mantém `is_locatable ↔ extract_payload.is_some()` (locatable.rs).
        Content::Dynamic(e) => e.dyn_to_payload(),

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
        let c = Content::heading(2, Content::Text(EcoString::from("Section")));
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
        let c = Content::figure(Content::Empty, None, Some("image".into()), None);
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
        let c = Content::figure(Content::Empty, None, None, None);
        match extract_payload(&c) {
            Some(ElementPayload::Figure { kind, .. }) => assert_eq!(kind, None),
            other => panic!("esperado Some(Figure), obtido {other:?}"),
        }
    }

    #[test]
    fn cite_produz_some_payload() {
        let c = Content::cite("smith2024".to_string(), None, None);
        match extract_payload(&c) {
            Some(ElementPayload::Citation { key }) => {
                assert_eq!(key, "smith2024");
            }
            other => panic!("esperado Some(Citation), obtido {other:?}"),
        }
    }

    #[test]
    fn text_produz_none() {
        let c = Content::Text(EcoString::from("plain"));
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
        // P178: Content::outline() → Some(ElementPayload::Outline).
        let c = Content::outline();
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
        let c = Content::bibliography(vec![bib_entry("smith2024")], None);
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
        let c = Content::bibliography(vec![bib_entry("a"), bib_entry("b"), bib_entry("c")], None);
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
        let c = Content::bibliography(vec![bib_entry("k")], Some(Content::Empty));
        match extract_payload(&c) {
            Some(ElementPayload::Bibliography { entries }) => {
                assert_eq!(entries.len(), 1);
            }
            other => panic!("esperado Some(Bibliography), obtido {other:?}"),
        }
    }

    // Lote F-2 S5 (P335): testes set_heading_numbering_*_produz_state_update
    // removidos — a variante SetHeadingNumbering e o seu arm StateUpdate saíram
    // (numeração via chain léxica).

    // ── P186C — Equation arm ─────────────────────────────────────────────

    #[test]
    fn equation_block_true_produz_some_payload() {
        let c = Content::equation(Content::Empty, true);
        match extract_payload(&c) {
            Some(ElementPayload::Equation { block, counter_update, .. }) => {
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
        let c = Content::equation(Content::Empty, false);
        match extract_payload(&c) {
            Some(ElementPayload::Equation { block, counter_update, .. }) => {
                assert!(!block);
                assert_eq!(counter_update, CounterUpdate::Step);
            }
            other => panic!("esperado Some(Equation), obtido {other:?}"),
        }
    }

    #[test]
    fn equation_body_e_ignorado() {
        // body distinto não afecta payload — só block é capturado.
        let c1 = Content::equation(Content::Empty, true);
        let c2 = Content::equation(Content::Text(EcoString::from("E=mc^2")), true);
        assert_eq!(extract_payload(&c1), extract_payload(&c2));
    }
}
