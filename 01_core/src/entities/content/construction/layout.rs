//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/content/construction/layout.md
//! @prompt-hash a9d6d39e
//! @layer L1
//! @updated 2026-10-06

use super::*;

impl Content {
    /// `pad(body, sides)` — Passo 156C (ADR-0061 Fase 1) /
    /// Passo 156L (refino sides individualizadas per ADR-0064 Caso C).
    pub fn pad(body: Content, sides: Sides<Option<Length>>) -> Self {
        Self::Pad(Arc::new(PadElem { body, sides }))
    }

    /// **Lote 10 P325** — `Content::Equation` (equação matemática).
    pub fn equation(body: Content, block: bool) -> Self {
        Self::Equation(Arc::new(EquationElem::new(body, block)))
    }
    /// Equação numerada **na forma de transporte** (F-5a de-bake, P364,
    /// `f_fronteira_e1.md` §3a.9). Campo assado removido; a numeração vive **só
    /// na chain** — produz o `Content::Styled` com `custom("equation.numbering")
    /// ="(1)"`, a forma que `#set math.equation(numbering:)` gera em produção. O
    /// gate efetivo continua `block && numbering` no consumidor.
    /// P456: pattern default "(1)" (format_counter) em vez de Bool.
    pub fn equation_numbered(body: Content, block: bool) -> Self {
        use crate::entities::style::Styles;
        use crate::entities::value::Value;
        Self::styled(
            Box::new(Self::equation(body, block)),
            Styles::new().push_custom("equation.numbering", Value::Str("(1)".into())),
        )
    }

    /// **Lote 11 P326** — `Content::Footnote` (nota de rodapé).
    pub fn footnote(body: Content) -> Self {
        Self::Footnote(Arc::new(FootnoteElem { body, numbering: None }))
    }

    /// **P502** — `Content::Footnote` com `numbering` customizado.
    pub fn footnote_with_numbering(body: Content, numbering: Option<EcoString>) -> Self {
        Self::Footnote(Arc::new(FootnoteElem { body, numbering }))
    }

    /// **P1473** — `Content::FootnoteEntry` (`footnote.entry(...)` qualificado,
    /// escopo reduzido — construção sem realização).
    pub fn footnote_entry(
        note: Content,
        separator: Option<Content>,
        clearance: Option<crate::entities::layout_types::Length>,
        gap: Option<crate::entities::layout_types::Length>,
        indent: Option<crate::entities::layout_types::Length>,
    ) -> Self {
        Self::FootnoteEntry(Arc::new(FootnoteEntryElem {
            note,
            separator,
            clearance,
            gap,
            indent,
        }))
    }

    /// **Lote 11 P326** — `Content::Shape` (geometria).
    pub fn shape(
        kind: ShapeKind,
        width: Option<Box<crate::entities::value::Value>>,
        height: Option<Box<crate::entities::value::Value>>,
        fill: Option<crate::entities::paint::Paint>,
        stroke: Option<Stroke>,
    ) -> Self {
        Self::shape_with_radius(
            kind,
            width,
            height,
            fill,
            stroke,
            Corners::uniform(Length::ZERO),
        )
    }

    /// **P852** — `Content::Shape` com radius (rounded rect).
    pub fn shape_with_radius(
        kind: ShapeKind,
        width: Option<Box<crate::entities::value::Value>>,
        height: Option<Box<crate::entities::value::Value>>,
        fill: Option<crate::entities::paint::Paint>,
        stroke: Option<Stroke>,
        radius: Corners<Length>,
    ) -> Self {
        let kind = match kind {
            ShapeKind::Rect if radius != Corners::uniform(Length::ZERO) => {
                ShapeKind::RoundedRect { radii: radius }
            }
            other => other,
        };
        Self::Shape(Arc::new(ShapeElem {
            kind,
            width,
            height,
            fill,
            stroke,
            fill_rule: crate::entities::geometry::FillRule::NonZero,
        }))
    }

    /// `hide(body)` — Passo 156C (ADR-0061 Fase 1).
    pub fn hide(body: Content) -> Self {
        Self::Hide(Arc::new(HideElem { body }))
    }

    /// `h(amount, weak)` — Passo 156D (ADR-0061 Fase 1 sub-passo 2).
    pub fn h_space(amount: Length, weak: bool) -> Self {
        Self::h_space_with_weak_presence(amount, weak, weak)
    }

    /// `h(amount, weak)` preservando a presença do named `weak` (P1140.9).
    pub fn h_space_with_weak_presence(
        amount: Length,
        weak: bool,
        weak_explicit: bool,
    ) -> Self {
        Self::HSpace(Arc::new(HSpaceElem {
            amount: crate::entities::elements::h_space::Spacing::Absolute(amount),
            weak,
            weak_explicit,
        }))
    }

    /// **P842 (#38)** — `h(amount.fr, weak)`: fração do espaço restante da
    /// linha (paridade vanilla `Spacing::Fractional`, `layout/spacing.rs`).
    pub fn h_space_fraction(fr: f64, weak: bool) -> Self {
        Self::h_space_fraction_with_weak_presence(fr, weak, weak)
    }

    /// `h(amount.fr, weak)` preservando a presença do named `weak` (P1140.9).
    pub fn h_space_fraction_with_weak_presence(
        fr: f64,
        weak: bool,
        weak_explicit: bool,
    ) -> Self {
        Self::HSpace(Arc::new(HSpaceElem {
            amount: crate::entities::elements::h_space::Spacing::Fractional(fr),
            weak,
            weak_explicit,
        }))
    }

    /// `v(amount, weak)` — Passo 156D (ADR-0061 Fase 1 sub-passo 2).
    pub fn v_space(amount: Length, weak: bool) -> Self {
        Self::v_space_with_weak_presence(amount, weak, weak)
    }

    /// `v(amount, weak)` preservando a presença do named `weak` (P1140.9).
    pub fn v_space_with_weak_presence(
        amount: Length,
        weak: bool,
        weak_explicit: bool,
    ) -> Self {
        Self::VSpace(Arc::new(VSpaceElem { amount, weak, weak_explicit }))
    }

    /// `pagebreak(weak, to)` — Passo 156E (ADR-0061 Fase 1 sub-passo 3).
    pub fn pagebreak(weak: bool, to: Option<Parity>) -> Self {
        Self::pagebreak_with_weak_presence(weak, weak, to)
    }

    /// `pagebreak(weak, to)` preservando a presença do named `weak` (P1140.9).
    pub fn pagebreak_with_weak_presence(
        weak: bool,
        weak_explicit: bool,
        to: Option<Parity>,
    ) -> Self {
        Self::Pagebreak(Arc::new(PagebreakElem { weak, weak_explicit, to }))
    }

    /// Page-run lexical interno. O binding público será materializado em P1140.21.
    #[expect(
        clippy::too_many_arguments,
        reason = "C023: Construtor público puro com argumentos correspondentes ao PageRun: ausência e numbering Option<Option<_>> têm significado próprio"
    )]
    pub fn page_run_with_geometry(
        paper: Option<crate::entities::page_geometry::Paper>,
        flipped: Option<bool>,
        binding: Option<crate::entities::page_geometry::PageBinding>,
        width: Option<crate::entities::layout_types::PageDimension>,
        height: Option<crate::entities::layout_types::PageDimension>,
        margin: Option<crate::entities::layout_types::PageMarginSpec>,
        numbering: Option<Option<crate::entities::numbering::Numbering>>,
        columns: Option<usize>,
        body: Content,
    ) -> Self {
        Self::PageRun(Arc::new(PageRunElem {
            paper,
            flipped,
            binding,
            width,
            height,
            margin,
            numbering,
            number_align: None,
            header: None,
            header_ascent: None,
            footer: None,
            footer_descent: None,
            supplement: None,
            columns,
            bleed: None,
            fill: None,
            background: None,
            foreground: None,
            body,
        }))
    }

    pub fn page_run(
        width: Option<crate::entities::layout_types::PageDimension>,
        height: Option<crate::entities::layout_types::PageDimension>,
        margin: Option<crate::entities::layout_types::PageMarginSpec>,
        numbering: Option<Option<crate::entities::numbering::Numbering>>,
        columns: Option<usize>,
        body: Content,
    ) -> Self {
        Self::page_run_with_geometry(
            None, None, None, width, height, margin, numbering, columns, body,
        )
    }

    /// `colbreak(weak)` — Passo 220 (ADR-0078 PROPOSTO sub-fase b 4/4).
    pub fn colbreak(weak: bool) -> Self {
        Self::colbreak_with_weak_presence(weak, weak)
    }

    /// `colbreak(weak)` preservando a presença do named `weak` (P1140.9).
    pub fn colbreak_with_weak_presence(weak: bool, weak_explicit: bool) -> Self {
        Self::Colbreak(Arc::new(ColbreakElem { weak, weak_explicit }))
    }

    /// `linebreak()` — Modelo D (Lote 5 P320).
    pub fn linebreak() -> Self {
        Self::linebreak_with_justify_presence(false, false)
    }

    /// `linebreak(justify:)` preservando valor e presença do named (P1140.10).
    pub fn linebreak_with_justify_presence(
        justify: bool,
        justify_explicit: bool,
    ) -> Self {
        Self::Linebreak(Arc::new(LinebreakElem { justify, justify_explicit }))
    }
    /// `grid_header(body, repeat)` — Modelo D (Lote 5 P320).
    pub fn grid_header(body: Content, repeat: bool) -> Self {
        Self::GridHeader(Arc::new(GridHeaderElem { body, repeat }))
    }
    /// `grid_footer(body, repeat)` — Modelo D (Lote 5 P320).
    pub fn grid_footer(body: Content, repeat: bool) -> Self {
        Self::GridFooter(Arc::new(GridFooterElem { body, repeat }))
    }
    /// `grid_hline(start, end, row, stroke, position)` — Passo 512.
    pub fn grid_hline(
        start: usize,
        end: Option<usize>,
        row: usize,
        stroke: Option<Stroke>,
        position: EcoString,
    ) -> Self {
        Self::GridHLine(Arc::new(GridHLineElem { start, end, row, stroke, position }))
    }
    /// `grid_vline(start, end, col, stroke, position)` — Passo 512.
    pub fn grid_vline(
        start: usize,
        end: Option<usize>,
        col: usize,
        stroke: Option<Stroke>,
        position: EcoString,
    ) -> Self {
        Self::GridVLine(Arc::new(GridVLineElem { start, end, col, stroke, position }))
    }

    /// `block(body, width, height, inset, breakable)` — Passo 156G
    /// (ADR-0061 Fase 2 sub-passo 1). Construtor com defaults sensatos
    /// (None/zero/true) para uso programático.
    pub fn block(
        body: Content,
        width: Option<Length>,
        height: Option<Length>,
        inset: Sides<Length>,
        breakable: bool,
    ) -> Self {
        Self::Block(Arc::new(BlockElem {
            body,
            width: width.map(|abs| crate::entities::rel::Rel { rel: 0.0, abs }),
            height: height.map(|abs| {
                crate::entities::elements::block::BlockHeight::Relative(
                    crate::entities::rel::Rel { rel: 0.0, abs },
                )
            }),
            inset,
            breakable,
            outset: Sides::new(
                Length::pt(0.0),
                Length::pt(0.0),
                Length::pt(0.0),
                Length::pt(0.0),
            ),
            // P242 — radius `Corners<Length>` substitui `Option<Length>` P231.
            radius: crate::entities::corners::Corners::uniform(Length::ZERO),
            clip: false,
            // P247 — fill/stroke default `None` (paridade pattern P242).
            fill: None,
            stroke: None,
            // P250 — spacing/above/below/sticky defaults (None×3 + false).
            spacing: None,
            above: None,
            below: None,
            sticky: false,
        }))
    }

    /// `box(body, width, height, inset, baseline)` — Passo 156H
    /// (ADR-0061 Fase 2 sub-passo 2). Naming `boxed` evita conflito com
    /// `std::boxed::Box`; stdlib expõe `#box(...)` (paridade vanilla).
    pub fn boxed(
        body: Content,
        width: Option<Length>,
        height: Option<Length>,
        inset: Sides<Length>,
        baseline: Length,
    ) -> Self {
        Self::Boxed(Arc::new(BoxedElem {
            body,
            width,
            width_fraction: None,
            height,
            inset,
            baseline,
            outset: Sides::new(
                Length::pt(0.0),
                Length::pt(0.0),
                Length::pt(0.0),
                Length::pt(0.0),
            ),
            // P242 — radius `Corners<Length>` substitui `Option<Length>` P231.
            radius: crate::entities::corners::Corners::uniform(Length::ZERO),
            clip: false,
            // P247 — fill/stroke default `None` paralelo Block.
            fill: None,
            stroke: None,
        }))
    }

    pub fn boxed_fractional(body: Content, width_fraction: f64) -> Self {
        Self::Boxed(Arc::new(BoxedElem {
            body,
            width: None,
            width_fraction: Some(width_fraction),
            height: None,
            inset: Sides::uniform(Length::ZERO),
            baseline: Length::ZERO,
            outset: Sides::uniform(Length::ZERO),
            radius: crate::entities::corners::Corners::uniform(Length::ZERO),
            clip: false,
            fill: None,
            stroke: None,
        }))
    }

    /// `stack(dir, spacing, ..children)` — Passo 156I (ADR-0061 Fase 2
    /// sub-passo 3). Atinge target 72% Layout. Aceita `Vec<Content>` que
    /// converte para `Arc<[Content]>` (clone O(1) per ADR-0026 revisão).
    pub fn stack(children: Vec<Content>, dir: Dir, spacing: Option<Length>) -> Self {
        Self::Stack(Arc::new(StackElem { children: children.into(), dir, spacing }))
    }

    /// **Lote 9 P324** — `Content::SmartQuote` (aspa lang-aware).
    pub fn smartquote(double: bool) -> Self {
        Self::SmartQuote(Arc::new(SmartQuoteElem {
            double: Some(double),
            enabled: None,
            alternative: None,
            quotes: None,
        }))
    }

    pub fn pdf_attach(elem: PdfAttachElem) -> Self {
        Self::PdfAttach(Arc::new(elem))
    }

    pub fn pdf_artifact(elem: PdfArtifactElem) -> Self {
        Self::PdfArtifact(Arc::new(elem))
    }

    /// **Lote 9 P324** — `Content::Transform` (transformação afim 2D).
    pub fn transform(matrix: TransformMatrix, body: Content) -> Self {
        Self::Transform(Arc::new(TransformElem {
            operation: crate::entities::elements::transform::TransformOperation::Matrix(
                matrix,
            ),
            body,
        }))
    }

    /// **Lote 9 P324** — `Content::Place` (posicionamento absoluto/flutuante).
    #[allow(clippy::too_many_arguments)]
    pub fn place(
        alignment: Align2D,
        dx: f64,
        dy: f64,
        scope: PlaceScope,
        float: bool,
        clearance: Option<Length>,
        body: Content,
    ) -> Self {
        Self::Place(Arc::new(PlaceElem {
            alignment,
            dx,
            dy,
            scope,
            float,
            clearance,
            body,
        }))
    }

    /// Constrói uma sentinela `place.flush()` sem campos.
    pub fn flush() -> Self {
        Self::Flush(Arc::new(FlushElem))
    }

    /// `repeat(body, gap, justify)` — Passo 156J (ADR-0061 Fase 3
    /// sub-passo 1). **Primeira Fase 3**. Default `justify == true`
    /// (paridade vanilla); algoritmo dinâmico de quantidade-para-encher
    /// diferido per ADR-0054 graded.
    pub fn repeat(body: Content, gap: Option<Length>, justify: bool) -> Self {
        Self::Repeat(Arc::new(RepeatElem { body, gap, justify }))
    }

    /// **P217** — Construtor `Content::Columns` (multi-column container).
    /// Stdlib `native_columns` em P218 com validação `count >= 1`.
    /// Consumer multi-region real em P219.
    pub fn columns(body: Content, count: usize, gutter: Option<Length>) -> Self {
        Self::Columns(Arc::new(ColumnsElem { count, gutter, body, page_columns: false }))
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn modulo_compila_e_carrega() {}
}
