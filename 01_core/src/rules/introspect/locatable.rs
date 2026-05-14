//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/introspect/locatable.md
//! @prompt-hash aaf16c83
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

        // ── Locatable em P240 (M9d/M7+1) — StateDisplay. Walk emite
        // tag para que `apply_state_displays` pre-renderize Content
        // resultado callback pós-fixpoint; layout arm consome via
        // `state_display_value`.
        Content::StateDisplay { .. } => true,

        // ── Locatable em P241 (M9d/M7+2) — CounterDisplayCallback
        // paralelo StateDisplay. Walk emite tag; `apply_counter_displays`
        // pré-renderiza Content via `apply_func(callback,
        // [Value::Array(counter_state)], ctx, engine)` pós-fixpoint;
        // layout arm consome via `counter_display_value`.
        // Distinto de `Content::CounterDisplay { kind }` legacy
        // (não-locatable; single-pass Layouter directo).
        Content::CounterDisplayCallback { .. } => true,

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

        // ── Locatable em P199B — SetEquationNumbering emite
        // `StateUpdate { key: "numbering_active:equation", ... }` via
        // `extract_payload`. Reusa arm `from_tags::StateUpdate`
        // (P171/P173) genérica. Materializa Reserva 1 (E1 P189B).
        // Cenário α por construção (ADR-0069) — caminho Introspector
        // activa imediatamente porque toda infraestrutura downstream
        // já estava pronta (Layouter equation.rs:32-33
        // substitution-with-fallback adormecida). Walk arm canonical
        // continua write paralelo legacy (M6 elimina).
        Content::SetEquationNumbering { .. } => true,

        // ── Locatable em P186D — Equation. Combinado com arm em
        // `extract_payload` (P186C) repõe invariante
        // `is_locatable ↔ extract_payload.is_some()`. `from_tags`
        // arm Equation (P186E) gate `block && state numbering_active:equation`
        // — counter dormente em produção até `Content::SetEquationNumbering`
        // (passo dedicado, fora da série P186). Suporta C2
        // desbloqueio per ADR-0068 (eixo 2 P183C); consumer migra
        // em P188.
        Content::Equation { .. } => true,

        // ── Locatable em P198C — CounterUpdate (cenário β-promote
        // ADR-0069). `extract_payload` emite
        // `ElementPayload::CounterUpdate { key, action }` pré-recursão;
        // `from_tags` arm aplica a `CounterRegistry` via `apply_at`
        // (flat) ou `apply_hierarchical_at` (key="heading"). Walk
        // arm legacy (E6 P189B) preservado como write paralelo M5
        // porque `compute_*` helpers leem `state.flat`/`hierarchical`
        // durante walk; cleanup orgânico em M6.
        Content::CounterUpdate { .. } => true,

        // ── Não-locatable ──────────────────────────────────────────
        Content::Empty
        | Content::Text(_, _)
        | Content::Space
        | Content::Sequence(_)
        | Content::Raw { .. }
        | Content::ListItem(_)
        | Content::EnumItem { .. }
        | Content::Link { .. }
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
        // P220: Colbreak não-locatable (event leaf; paridade Pagebreak).
        | Content::Colbreak { .. }
        | Content::Stack { .. }
        | Content::Boxed { .. }
        | Content::Block { .. }
        | Content::TableCell { .. }
        | Content::TableHeader { .. }
        | Content::TableFooter { .. }
        | Content::Table { .. }
        // P224 — Grid refino + variants novos não-locatable (paridade
        // Table*; events estructurais sem identidade observable).
        | Content::GridHeader { .. }
        | Content::GridFooter { .. }
        | Content::GridCell { .. }
        | Content::Repeat { .. }
        // P217 — Columns container não-locatable (transparente para
        // introspect; consumer multi-region em P219).
        | Content::Columns { .. } => false,
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
            // P186D: Equation cobertura no test de invariante.
            // Lacuna pré-existente — Equation estava omitida do
            // helper, escondendo divergências entre is_locatable e
            // extract_payload se houvesse erro de sincronização.
            Content::Equation { body: Box::new(Content::Empty), block: true },
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
