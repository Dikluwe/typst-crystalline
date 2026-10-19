//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/introspect/extract_payload.md
//! @prompt-hash 6bc497b1
//! @layer L1
//! @updated 2026-10-16
//!
//! `extract_payload` — função pura `&Content → Option<ElementPayload>`.
//! P162 sub-passo .D. Consumida pelo walk em P162 .E.

use crate::entities::content::Content;
#[cfg(test)]
use crate::entities::counter_update::CounterUpdate;
use crate::entities::element_payload::ElementPayload;
use crate::entities::elements::Element; // Modelo D (P316): trait p/ to_payload()

/// Extrai o payload próprio do nó, normalizando a origem transparente Source.
/// OutlineEntry delega ao elemento; a síntese pós-walk mantém sua responsabilidade.
pub fn extract_payload(content: &Content) -> Option<ElementPayload> {
    let content = content.source_body();
    match content {
        Content::Located(e) => extract_payload(&e.body),
        Content::ShowRecipe(_) | Content::Realized(_) => None,
        Content::Strong(e) => e.to_payload(),
        Content::Emph(e) => e.to_payload(),
        // Modelo D (P316): absorção do locatável — o elemento fornece o payload.
        Content::Heading(h) => h.to_payload(),
        Content::OutlineEntry(e) => e.to_payload(),

        // Modelo D (Lote 13 P328): Figure locatável delega ao elemento.
        Content::Figure(e) => e.to_payload(),

        // Modelo D (Lote 9 P324): Cite locatável delega ao elemento.
        Content::Cite(e) => e.to_payload(),

        // Modelo D (Lote 6 P321): família state/counter locatável absorve o
        // payload no trait — o elemento fornece (precedente Heading P316).
        Content::Metadata(e) => e.to_payload(),
        Content::State(e) => e.to_payload(),
        Content::StateUpdate(e) => e.to_payload(),
        Content::StateDisplay(e) => e.to_payload(),
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

        // P461 — Table promovido a locatable. Counter flat `"table"`
        // populado pelo walk top + populate_intr arm quando
        // numbering+caption activos.
        Content::Table(e) => e.to_payload(),

        // P1016 — Footnote promovido a locatable. Counter flat
        // `"footnote"` populado sem gate (toda a nota conta); mantém o
        // invariante `is_locatable ↔ extract_payload.is_some()`.
        Content::Footnote(e) => e.to_payload(),

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

        // P506 — ContextBlock captura a sua localização no walk para a
        // expansão pós-introspecção. Delega ao elemento para o payload.
        Content::ContextBlock(e) => e.to_payload(),

        // As demais variantes atuais não fornecem payload próprio aqui.
        // Source já foi normalizado e não possui payload próprio.
        Content::Source(_)
        | Content::Empty
        | Content::Text(_)
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
        | Content::Sequence(_)
        | Content::HtmlElem(_)
        | Content::HtmlFrame(_)
        | Content::Par { .. }
        | Content::Title(_)
        | Content::Raw(_)
        | Content::ListItem(_)
        | Content::EnumItem(_)
        | Content::Link(_)
        | Content::MathSequence(_)
        | Content::MathIdent(_)
        | Content::MathText(_)
        | Content::MathFrac(_)
        | Content::MathAttach(_)
        | Content::MathPrimes(_)
        | Content::MathRoot(_)
        | Content::MathStretch(_)
        | Content::MathDelimited(_)
        | Content::MathLr(_)
        | Content::MathMid(_)
        | Content::MathOverbrace(_)
        | Content::MathUnderbrace(_)
        | Content::MathUnderbracket(_)
        | Content::MathUnderparen(_)
        | Content::MathUndershell(_)
        | Content::MathOverbracket(_)
        | Content::MathOverparen(_)
        | Content::MathOvershell(_)
        | Content::MathAlignPoint(_)
        | Content::Linebreak(_)
        | Content::MathMatrix(_)
        | Content::MathCases(_)
        | Content::MathAccent(_)
        | Content::MathCancel(_)
        | Content::MathUnderline(_)
        | Content::MathOverline(_)
        | Content::MathVec(_)
        | Content::MathClassOverride(_)
        | Content::MathLimitsOverride(_)
        | Content::MathUnderover(_)
        | Content::MathOp(_)
        | Content::MathStyled(_)
        | Content::Label(_)
        | Content::Ref(_)
        | Content::CounterDisplay(_)
        | Content::FigureCaption(_)
        | Content::Image(_)
        | Content::Shape(_)
        | Content::Curve(_)
        | Content::Transform(_)
        | Content::Grid(_)
        | Content::GridHeader(_)
        | Content::GridFooter(_)
        | Content::GridCell(_)
        | Content::SetPage(_)
        | Content::PageRun(_)
        | Content::Align(_)
        | Content::Place(_)
        | Content::Flush(_)
        | Content::Styled(_)
        | Content::Divider(_)
        | Content::Terms(_)
        | Content::TermItem(_)
        | Content::Quote(_)
        | Content::Document { .. }
        | Content::Asset { .. }
        | Content::SmartQuote(_)
        | Content::PdfAttach(_)
        | Content::PdfArtifact(_)
        | Content::Underline(_)
        | Content::Strike(_)
        | Content::Overline(_)
        | Content::SmallCaps { .. }
        | Content::Pad(_)
        | Content::Hide(_)
        | Content::HSpace(_)
        | Content::VSpace(_)
        | Content::Pagebreak(_)
        | Content::Colbreak(_)
        | Content::Stack(_)
        | Content::Boxed(_)
        | Content::Block(_)
        | Content::TableCell(_)
        | Content::FootnoteEntry(_)
        | Content::TableHeader(_)
        | Content::TableFooter(_)
        | Content::GridHLine(_)
        | Content::GridVLine(_)
        | Content::TableHLine(_)
        | Content::TableVLine(_)
        | Content::Repeat(_)
        | Content::Columns(_) => None,
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
            Some(ElementPayload::Heading {
                depth, body_hash, counter_update, ..
            }) => {
                assert_eq!(depth, 2);
                assert_ne!(body_hash, 0); // hash de "Section" é não-zero
                assert_eq!(counter_update, CounterUpdate::step());
            }
            other => panic!("esperado Some(Heading), obtido {other:?}"),
        }
    }

    #[test]
    fn figure_produz_some_payload() {
        let c = Content::figure(Content::Empty, None, Some("image".into()), None);
        match extract_payload(&c) {
            Some(ElementPayload::Figure {
                kind, counter_update, is_counted: _, ..
            }) => {
                assert_eq!(kind, Some("image".to_string()));
                assert_eq!(counter_update, CounterUpdate::step());
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
        // P622: Parbreak não é locatable.
        assert_eq!(extract_payload(&Content::Parbreak), None);
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
            key: key.to_string(),
            author: String::new(),
            title: String::new(),
            year: 0,
            volume: None,
            pages: None,
            journal: None,
            publisher: None,
            url: None,
            doi: None,
            editor: None,
            series: None,
            note: None,
            isbn: None,
            location: None,
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
        let c = Content::bibliography(
            vec![bib_entry("a"), bib_entry("b"), bib_entry("c")],
            None,
        );
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
                assert_eq!(counter_update, CounterUpdate::step());
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
                assert_eq!(counter_update, CounterUpdate::step());
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

#[cfg(test)]
mod v16_origins_tests {
    use super::*;
    use crate::entities::file_id::FileId;
    use crate::entities::span::Span;
    use crate::entities::value::Value;
    use std::num::NonZeroU16;
    use std::sync::Arc;

    fn origin_span() -> Span {
        Span::from_range(FileId::from_raw(NonZeroU16::new(1).unwrap()), 2..5)
    }

    #[test]
    fn v16_origins_source_heading_keeps_body_payload() {
        let body = Content::heading(2, Content::text("section"));
        let expected = extract_payload(&body);
        assert!(expected.is_some());
        let sourced = body.with_source_span(origin_span());
        assert_eq!(extract_payload(&sourced), expected);
    }

    #[test]
    fn v16_origins_source_emphasis_and_metadata_keep_own_payloads() {
        for body in [
            Content::strong(Content::Empty),
            Content::emph(Content::Empty),
            Content::metadata(Value::Int(42)),
        ] {
            let expected = extract_payload(&body);
            assert!(expected.is_some());
            assert_eq!(extract_payload(&body.with_source_span(origin_span())), expected);
        }
    }

    #[test]
    fn v16_origins_source_non_producers_do_not_gain_payloads() {
        for body in [
            Content::Empty,
            Content::Space,
            Content::SpaceFromNewline,
            Content::text("text"),
            Content::Sequence(Arc::from(vec![Content::text("child")])),
        ] {
            assert!(extract_payload(&body.with_source_span(origin_span())).is_none());
        }
    }

    #[test]
    fn v16_walk_entry_payload_alignment_matches_native_delegate() {
        let entry = Content::outline_entry(1, Content::heading(1, Content::Empty), None);
        assert!(crate::compiler::introspect::locatable::is_locatable(&entry));
        assert_eq!(extract_payload(&entry), Some(ElementPayload::NativeElement));
        assert_eq!(
            extract_payload(&entry.with_source_span(origin_span())),
            Some(ElementPayload::NativeElement)
        );
    }
}
