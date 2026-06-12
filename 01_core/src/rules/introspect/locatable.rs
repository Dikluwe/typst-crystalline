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
        Content::Heading(_) => true,
        Content::Figure  { .. } => true,
        Content::Cite    { .. } => true,

        // ── Locatable em M9 (P169) — Metadata é queriable ──────────
        Content::Metadata(_) => true,

        // ── Locatable em M9 (P171) — State e StateUpdate ───────────
        Content::State(_) => true,
        Content::StateUpdate(_) => true,

        // ── Locatable em P240 (M9d/M7+1) — StateDisplay. Walk emite
        // tag para que `apply_state_displays` pre-renderize Content
        // resultado callback pós-fixpoint; layout arm consome via
        // `state_display_value`.
        Content::StateDisplay(_) => true,

        // ── Locatable em P241 (M9d/M7+2) — CounterDisplayCallback
        // paralelo StateDisplay. Walk emite tag; `apply_counter_displays`
        // pré-renderiza Content via `apply_func(callback,
        // [Value::Array(counter_state)], ctx, engine)` pós-fixpoint;
        // layout arm consome via `counter_display_value`.
        // Distinto de `Content::CounterDisplay { kind }` legacy
        // (não-locatable; single-pass Layouter directo).
        Content::CounterDisplayCallback(_) => true,

        // ── Locatable em P178 — Outline fecha lacuna #7 ────────────
        Content::Outline(_) => true,

        // ── Locatable em P181D — Bibliography (decisão P181A
        // cláusula 4 = Opção β walk puro). `from_tags` arm popula
        // `BibStore` (P181E pendente). Suporta plano P181 para
        // fechar lacuna #6.
        Content::Bibliography(_) => true,

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
        Content::Equation(_) => true,

        // ── Locatable em P198C — CounterUpdate (cenário β-promote
        // ADR-0069). `extract_payload` emite
        // `ElementPayload::CounterUpdate { key, action }` pré-recursão;
        // `from_tags` arm aplica a `CounterRegistry` via `apply_at`
        // (flat) ou `apply_hierarchical_at` (key="heading"). Walk
        // arm legacy (E6 P189B) preservado como write paralelo M5
        // porque `compute_*` helpers leem `state.flat`/`hierarchical`
        // durante walk; cleanup orgânico em M6.
        Content::CounterUpdate(_) => true,

        // ── Não-locatable ──────────────────────────────────────────
        Content::Empty
        | Content::Text(_, _)
        | Content::Space
        | Content::Sequence(_)
        | Content::Raw(_)
        | Content::ListItem(_)
        | Content::EnumItem(_)
        | Content::Link(_)
        | Content::MathSequence(_)
        | Content::MathIdent(_)
        | Content::MathText(_)
        | Content::MathFrac(_)
        | Content::MathAttach(_)
        | Content::MathRoot(_)
        | Content::MathDelimited(_)
        | Content::MathAlignPoint(_)
        | Content::Linebreak(_)
        | Content::MathMatrix(_)
        | Content::MathCases(_)
        | Content::Labelled { .. }
        | Content::Ref(_)
        | Content::CounterDisplay(_)
        | Content::SetFigureNumbering { .. }
        | Content::Image(_)
        | Content::Shape { .. }
        | Content::Transform(_)
        | Content::Grid(_)
        | Content::SetPage { .. }
        | Content::Align(_)
        | Content::Place(_)
        | Content::Styled(_, _)
        | Content::Divider(_)
        | Content::Terms(_)
        | Content::TermItem(_)
        | Content::Quote(_)
        // P284 — text decoration: não-locatable (cosmético inline; sem
        // identidade observable, paridade Quote/Link).
        | Content::Underline(_)
        | Content::Strike(_)
        | Content::Overline(_)
        // P287 — SmartQuote leaf não-locatable (glyph único; sem identidade
        // queryable; paridade Space/Linebreak).
        | Content::SmartQuote(_)
        | Content::Pad(_)
        | Content::Hide(_)
        | Content::HSpace(_)
        | Content::VSpace(_)
        | Content::Pagebreak(_)
        // P220: Colbreak não-locatable (event leaf; paridade Pagebreak).
        | Content::Colbreak(_)
        | Content::Stack(_)
        | Content::Boxed { .. }
        | Content::Block { .. }
        | Content::TableCell(_)
        | Content::TableHeader { .. }
        | Content::TableFooter { .. }
        | Content::Table(_)
        // P224 — Grid refino + variants novos não-locatable (paridade
        // Table*; events estructurais sem identidade observable).
        | Content::GridHeader { .. }
        | Content::GridFooter { .. }
        | Content::GridCell(_)
        | Content::Repeat(_)
        // P217 — Columns container não-locatable (transparente para
        // introspect; consumer multi-region em P219).
        | Content::Columns(_)
        // P295 — Footnote Fase 1 marker only: não-locatable. Frente
        // futura P295.X (footnote reference via `<label>`) tornaria
        // locatable; preserved scope-out aqui per ADR-0054 graded.
        | Content::Footnote { .. }
        // P296 — Math accent/cancel não-locatable (paralelo
        // MathFrac/MathRoot/MathDelimited; math structural inerte).
        | Content::MathAccent(_)
        | Content::MathCancel(_)
        // P297 — Math underover não-locatable (paralelo P296).
        | Content::MathUnderover(_)
        // P298 — Math op não-locatable.
        | Content::MathOp(_)
        // P311b.2 — MathStyled não-locatable (math structural; wrap glyph).
        | Content::MathStyled(_) => false,
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
        let c = Content::heading(1, Content::Empty);
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
        let c = Content::cite("k".to_string(), None, None);
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
            target: Box::new(Content::heading(1, Content::Empty)),
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
            Content::heading(1, Content::Empty),
            Content::Figure { body: Box::new(Content::Empty), caption: None, kind: None, numbering: None },
            Content::cite("k", None, None),
            // Não-locatable: amostra representativa
            Content::Empty,
            Content::Text(EcoString::from("t"), Default::default()),
            Content::Space,
            Content::Sequence(std::sync::Arc::from(vec![Content::Empty])),
            Content::Labelled {
                target: Box::new(Content::Empty),
                label:  crate::entities::label::Label("x".to_string()),
            },
            Content::reference(crate::entities::label::Label("y".to_string())),
            Content::outline(),
            Content::linebreak(),
            Content::divider(),
            Content::math_align_point(),
            Content::list_item(Content::Empty),
            Content::SetHeadingNumbering { active: true },
            // P186D: Equation cobertura no test de invariante.
            // Lacuna pré-existente — Equation estava omitida do
            // helper, escondendo divergências entre is_locatable e
            // extract_payload se houvesse erro de sincronização.
            Content::equation(Content::Empty, true),
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
        let c = Content::metadata(crate::entities::value::Value::Int(42));
        assert!(is_locatable(&c));
        // Invariante: extract_payload deve produzir Some.
        assert!(extract_payload(&c).is_some());
    }

    // ── P181D — Bibliography locatable ───────────────────────────────────

    #[test]
    fn bibliography_e_locatable() {
        let c = Content::bibliography(vec![], None);
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
