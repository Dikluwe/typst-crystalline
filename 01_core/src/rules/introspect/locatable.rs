//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/introspect/locatable.md
//! @prompt-hash 186cea9d
//! @layer L1
//! @updated 2026-04-30
//!
//! `is_locatable` — função pura `&Content → bool`. P164 (M2 Introspection).
//!
//! Match exaustivo (sem `_ => false`): compilador força revisão quando
//! variant novo é adicionado a `Content`. Invariante:
//! `is_locatable(c) == extract_payload(c).is_some()` para todo c.

use crate::entities::content::Content;

/// Classifica se `content` é uma variante locatable (queryable pela
/// introspecção). M1 cobre 3 kinds: `Heading`, `Figure`, `Cite`.
///
/// Equivalente a `extract_payload(c).is_some()` mas mais barato — sem
/// construção de payload nem cálculo de hash.
pub fn is_locatable(content: &Content) -> bool {
    match content {
        // ── Locatable em M1 ──────────────────────────────────────────
        Content::Heading { .. } => true,
        Content::Figure  { .. } => true,
        Content::Cite    { .. } => true,

        // ── Locatable em M9 (P169) — Metadata é queriable ──────────
        Content::Metadata { .. } => true,

        // ── Locatable em M9 (P171) — State e StateUpdate ───────────
        Content::State { .. } => true,
        Content::StateUpdate { .. } => true,

        // ── Locatable em P178 — Outline fecha lacuna #7 ────────────
        Content::Outline => true,

        // ── Locatable em P181D — Bibliography (decisão P181A
        // cláusula 4 = Opção β walk puro). `from_tags` arm popula
        // `BibStore` (P181E pendente). Suporta plano P181 para
        // fechar lacuna #6.
        Content::Bibliography { .. } => true,

        // ── Locatable em P182C — SetHeadingNumbering emite
        // `StateUpdate { key: "numbering_active:heading", ... }` via
        // `extract_payload`. `from_tags` arm `StateUpdate` (P171/P173)
        // popula `StateRegistry`. Suporta plano P182 para fechar
        // lacuna #4. Walk arm canonical em `introspect.rs:455–457`
        // continua write paralelo legacy (M6 elimina).
        Content::SetHeadingNumbering { .. } => true,

        // ── Não-locatable (47 variants) ──────────────────────────────
        Content::Empty
        | Content::Text(_, _)
        | Content::Space
        | Content::Sequence(_)
        | Content::Raw { .. }
        | Content::ListItem(_)
        | Content::EnumItem { .. }
        | Content::Link { .. }
        | Content::Equation { .. }
        | Content::MathSequence(_)
        | Content::MathIdent(_)
        | Content::MathText(_)
        | Content::MathFrac { .. }
        | Content::MathAttach { .. }
        | Content::MathRoot { .. }
        | Content::MathDelimited { .. }
        | Content::MathAlignPoint
        | Content::Linebreak
        | Content::MathMatrix { .. }
        | Content::MathCases { .. }
        | Content::Labelled { .. }
        | Content::Ref { .. }
        | Content::CounterDisplay { .. }
        | Content::CounterUpdate { .. }
        | Content::SetFigureNumbering { .. }
        | Content::Image { .. }
        | Content::Shape { .. }
        | Content::Transform { .. }
        | Content::Grid { .. }
        | Content::SetPage { .. }
        | Content::Align { .. }
        | Content::Place { .. }
        | Content::Styled(_, _)
        | Content::Divider
        | Content::Terms { .. }
        | Content::TermItem { .. }
        | Content::Quote { .. }
        | Content::Pad { .. }
        | Content::Hide { .. }
        | Content::HSpace { .. }
        | Content::VSpace { .. }
        | Content::Pagebreak { .. }
        | Content::Stack { .. }
        | Content::Boxed { .. }
        | Content::Block { .. }
        | Content::TableCell { .. }
        | Content::TableHeader { .. }
        | Content::TableFooter { .. }
        | Content::Table { .. }
        | Content::Repeat { .. } => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::introspect::extract_payload::extract_payload;
    use ecow::EcoString;

    // ── Cobertura locatable ──────────────────────────────────────────

    #[test]
    fn heading_e_locatable() {
        let c = Content::Heading {
            level: 1,
            body:  Box::new(Content::Empty),
        };
        assert!(is_locatable(&c));
    }

    #[test]
    fn figure_e_locatable() {
        let c = Content::Figure {
            body:      Box::new(Content::Empty),
            caption:   None,
            kind:      None,
            numbering: None,
        };
        assert!(is_locatable(&c));
    }

    #[test]
    fn cite_e_locatable() {
        let c = Content::Cite {
            key:        "k".to_string(),
            supplement: None,
            form:       None,
        };
        assert!(is_locatable(&c));
    }

    // ── Cobertura não-locatable ─────────────────────────────────────

    #[test]
    fn text_nao_e_locatable() {
        let c = Content::Text(EcoString::from("plain"), Default::default());
        assert!(!is_locatable(&c));
    }

    #[test]
    fn empty_nao_e_locatable() {
        assert!(!is_locatable(&Content::Empty));
    }

    #[test]
    fn space_nao_e_locatable() {
        assert!(!is_locatable(&Content::Space));
    }

    #[test]
    fn sequence_nao_e_locatable() {
        let c = Content::Sequence(std::sync::Arc::from(vec![Content::Empty]));
        assert!(!is_locatable(&c));
    }

    #[test]
    fn labelled_nao_e_locatable_mesmo_que_target_seja() {
        // Labelled em si não é locatable — o target é (via wrapping
        // mechanism em walk). Esta é uma propriedade da função pura
        // is_locatable: olha apenas para o nó actual, não para
        // children.
        let c = Content::Labelled {
            target: Box::new(Content::Heading {
                level: 1,
                body:  Box::new(Content::Empty),
            }),
            label:  crate::entities::label::Label("x".to_string()),
        };
        assert!(!is_locatable(&c));
    }

    // ── Invariante: is_locatable(c) == extract_payload(c).is_some() ──

    fn build_minimal_for_each_variant() -> Vec<Content> {
        // Constrói representante de cada bucket relevante. Não cobre
        // todas as 56 variants — uma instância por bucket é suficiente
        // para verificar a invariante (a invariante é estrutural sobre
        // o match em ambas as funções).
        vec![
            // Locatable (3)
            Content::Heading { level: 1, body: Box::new(Content::Empty) },
            Content::Figure { body: Box::new(Content::Empty), caption: None, kind: None, numbering: None },
            Content::Cite { key: "k".into(), supplement: None, form: None },
            // Não-locatable: amostra representativa
            Content::Empty,
            Content::Text(EcoString::from("t"), Default::default()),
            Content::Space,
            Content::Sequence(std::sync::Arc::from(vec![Content::Empty])),
            Content::Labelled {
                target: Box::new(Content::Empty),
                label:  crate::entities::label::Label("x".to_string()),
            },
            Content::Ref { target: crate::entities::label::Label("y".to_string()) },
            Content::Outline,
            Content::Linebreak,
            Content::Divider,
            Content::MathAlignPoint,
            Content::ListItem(Box::new(Content::Empty)),
            Content::SetHeadingNumbering { active: true },
        ]
    }

    #[test]
    fn invariante_is_locatable_equivale_extract_payload_is_some() {
        for c in build_minimal_for_each_variant() {
            assert_eq!(
                is_locatable(&c),
                extract_payload(&c).is_some(),
                "invariante violada para variant {c:?}"
            );
        }
    }

    // ── P169 (M9 sub-passo 1) — Metadata locatable ───────────────────────

    #[test]
    fn metadata_e_locatable() {
        let c = Content::Metadata {
            value: Box::new(crate::entities::value::Value::Int(42)),
        };
        assert!(is_locatable(&c));
        // Invariante: extract_payload deve produzir Some.
        assert!(extract_payload(&c).is_some());
    }

    // ── P181D — Bibliography locatable ───────────────────────────────────

    #[test]
    fn bibliography_e_locatable() {
        let c = Content::Bibliography {
            entries: vec![],
            title:   None,
        };
        assert!(is_locatable(&c));
        // Invariante: extract_payload deve produzir Some.
        assert!(extract_payload(&c).is_some());
    }

    // ── P182C — SetHeadingNumbering locatable ────────────────────────────

    #[test]
    fn set_heading_numbering_e_locatable() {
        let c = Content::SetHeadingNumbering { active: true };
        assert!(is_locatable(&c));
        // Invariante: extract_payload deve produzir Some.
        assert!(extract_payload(&c).is_some());
        // Simétrico para active=false.
        let c_false = Content::SetHeadingNumbering { active: false };
        assert!(is_locatable(&c_false));
        assert!(extract_payload(&c_false).is_some());
    }
}
