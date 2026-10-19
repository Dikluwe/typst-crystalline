//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/content/fields.md
//! @prompt-hash 3ce2f50f
//! @layer L1
//! @updated 2026-10-07

use super::*;

fn styled_equation(content: &Content) -> Option<&EquationElem> {
    match content {
        Content::Equation(e) => Some(e),
        Content::Styled(styled) => styled_equation(&styled.body),
        Content::Source(e) => styled_equation(&e.body),
        Content::ShowRecipe(e) => styled_equation(&e.body),
        Content::Located(e) => styled_equation(&e.body),
        Content::Realized(e) => styled_equation(&e.visual),
        Content::Empty
        | Content::Text(..)
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
        | Content::Sequence(..)
        | Content::HtmlElem(..)
        | Content::HtmlFrame(..)
        | Content::Par { .. }
        | Content::Heading(..)
        | Content::Title(..)
        | Content::Strong(..)
        | Content::Emph(..)
        | Content::Raw(..)
        | Content::ListItem(..)
        | Content::EnumItem(..)
        | Content::Link(..)
        | Content::MathSequence(..)
        | Content::MathIdent(..)
        | Content::MathText(..)
        | Content::MathFrac(..)
        | Content::MathAttach(..)
        | Content::MathPrimes(..)
        | Content::MathRoot(..)
        | Content::MathStretch(..)
        | Content::MathDelimited(..)
        | Content::MathLr(..)
        | Content::MathMid(..)
        | Content::MathOverbrace(..)
        | Content::MathUnderbrace(..)
        | Content::MathUnderbracket(..)
        | Content::MathUnderparen(..)
        | Content::MathUndershell(..)
        | Content::MathOverbracket(..)
        | Content::MathOverparen(..)
        | Content::MathOvershell(..)
        | Content::MathAlignPoint(..)
        | Content::Linebreak(..)
        | Content::MathMatrix(..)
        | Content::MathCases(..)
        | Content::MathAccent(..)
        | Content::MathCancel(..)
        | Content::MathUnderline(..)
        | Content::MathOverline(..)
        | Content::MathVec(..)
        | Content::MathClassOverride(..)
        | Content::MathLimitsOverride(..)
        | Content::MathUnderover(..)
        | Content::MathOp(..)
        | Content::MathStyled(..)
        | Content::Label(..)
        | Content::Ref(..)
        | Content::CounterDisplay(..)
        | Content::CounterUpdate(..)
        | Content::Outline(..)
        | Content::OutlineEntry(..)
        | Content::Figure(..)
        | Content::FigureCaption(..)
        | Content::Image(..)
        | Content::Shape(..)
        | Content::Curve(..)
        | Content::Transform(..)
        | Content::Grid(..)
        | Content::GridHeader(..)
        | Content::GridFooter(..)
        | Content::GridCell(..)
        | Content::SetPage(..)
        | Content::PageRun(..)
        | Content::Align(..)
        | Content::Place(..)
        | Content::Flush(..)
        | Content::Divider(..)
        | Content::Terms(..)
        | Content::TermItem(..)
        | Content::Quote(..)
        | Content::Document { .. }
        | Content::Asset { .. }
        | Content::SmartQuote(..)
        | Content::PdfAttach(..)
        | Content::PdfArtifact(..)
        | Content::Underline(..)
        | Content::Strike(..)
        | Content::Overline(..)
        | Content::SmallCaps { .. }
        | Content::Pad(..)
        | Content::Hide(..)
        | Content::HSpace(..)
        | Content::VSpace(..)
        | Content::Pagebreak(..)
        | Content::Colbreak(..)
        | Content::Stack(..)
        | Content::Boxed(..)
        | Content::Block(..)
        | Content::TableCell(..)
        | Content::Bibliography(..)
        | Content::Cite(..)
        | Content::Footnote(..)
        | Content::FootnoteEntry(..)
        | Content::TableHeader(..)
        | Content::TableFooter(..)
        | Content::GridHLine(..)
        | Content::GridVLine(..)
        | Content::TableHLine(..)
        | Content::TableVLine(..)
        | Content::Table(..)
        | Content::Repeat(..)
        | Content::Columns(..)
        | Content::Metadata(..)
        | Content::State(..)
        | Content::StateUpdate(..)
        | Content::StateDisplay(..)
        | Content::CounterDisplayCallback(..)
        | Content::ContextBlock(..)
        | Content::Dynamic(..) => None,
    }
}

impl Content {
    pub(crate) fn constructor_heading(
        &self,
    ) -> Option<(&Content, &crate::entities::style::Styles)> {
        let Content::Styled(styled) = self.source_body() else { return None };
        if !matches!(styled.body.source_body(), Content::Heading(_)) {
            return None;
        }
        let marker = styled
            .styles
            .delta()
            .custom
            .iter()
            .rev()
            .find(|(key, _)| key.as_str() == "heading.constructor");
        marker
            .filter(|(_, value)| {
                matches!(value, crate::entities::value::Value::Bool(true))
            })
            .map(|_| (styled.body.source_body(), &styled.styles))
    }

    pub fn get_field(&self, field: &str) -> Option<crate::entities::value::Value> {
        use crate::entities::value::Value;
        match (self, field) {
            (Content::Source(e), f) => e.body.get_field(f),
            (Content::ShowRecipe(e), f) => e.body.get_field(f),
            (Content::Located(e), f) => e.body.get_field(f),
            (Content::Realized(e), f) => e.visual.get_field(f),
            (Content::Empty, "children") => Some(Value::Array(vec![])),
            (Content::Sequence(children), "children") => {
                Some(Value::Array(children.iter().cloned().map(Value::Content).collect()))
            }
            (Content::Text(text), "text") => Some(Value::Str(text.clone())),
            (Content::HSpace(e), "amount") => Some(match e.amount {
                crate::entities::elements::h_space::Spacing::Absolute(length) => {
                    Value::Length(length)
                }
                crate::entities::elements::h_space::Spacing::Fractional(fr) => {
                    Value::Fraction(fr)
                }
            }),
            (Content::VSpace(e), "amount") => Some(Value::Length(e.amount)),
            (Content::HSpace(e), "weak") => Some(Value::Bool(e.weak)),
            (Content::VSpace(e), "weak") => Some(Value::Bool(e.weak)),
            (Content::Boxed(e), "body") => Some(Value::Content(e.body.clone())),
            (Content::Boxed(e), "width") => {
                e.width_fraction.map(Value::Fraction).or_else(|| {
                    e.width.map(|abs| {
                        Value::Relative(crate::entities::rel::Rel { rel: 0.0, abs })
                    })
                })
            }
            (Content::Boxed(e), "height") => e
                .height
                .map(|abs| Value::Relative(crate::entities::rel::Rel { rel: 0.0, abs })),
            (Content::Block(e), "body") => Some(Value::Content(e.body.clone())),
            (Content::Block(e), "width") => e.width.map(Value::Relative),
            (Content::Block(e), "height") => e.height.map(|height| match height {
                crate::entities::elements::block::BlockHeight::Relative(length) => {
                    Value::Relative(length)
                }
                crate::entities::elements::block::BlockHeight::Fractional(fr) => {
                    Value::Fraction(fr)
                }
            }),
            // Modelo D (P316): Heading delega ao elemento.
            (Content::Heading(h), f) => h.get_field(f),
            (Content::Raw(e), f) => e.get_field(f),
            (Content::Transform(e), f) => e.get_field(f),
            (Content::OutlineEntry(e), f) => e.get_field(f),
            // P765a: Title delega ao elemento.
            (Content::Title(t), f) => t.get_field(f),
            // F-5b fatia 1 (P371): strong/emph delegam (ex.: `it.body`).
            (Content::Strong(e), f) => e.get_field(f),
            (Content::Emph(e), f) => e.get_field(f),
            (Content::Linebreak(e), f) => e.get_field(f),
            (Content::SmartQuote(e), "double") => {
                Some(Value::Bool(e.double.unwrap_or(true)))
            }
            (Content::SmartQuote(e), "enabled") => {
                Some(Value::Bool(e.enabled.unwrap_or(true)))
            }
            (Content::SmartQuote(e), "alternative") => {
                Some(Value::Bool(e.alternative.unwrap_or(false)))
            }
            (Content::SmartQuote(e), "quotes") => {
                use crate::entities::elements::smartquote::SmartQuoteQuotes;
                Some(match &e.quotes {
                    Some(SmartQuoteQuotes::Custom(overrides)) => {
                        let pair = |p: &Option<
                            crate::entities::elements::smartquote::SmartQuotePair,
                        >| {
                            p.as_ref()
                                .map(|p| {
                                    Value::Array(vec![
                                        Value::Str(p.open.clone()),
                                        Value::Str(p.close.clone()),
                                    ])
                                })
                                .unwrap_or(Value::Auto)
                        };
                        Value::Dict(
                            [
                                ("double".into(), pair(&overrides.double)),
                                ("single".into(), pair(&overrides.single)),
                            ]
                            .into_iter()
                            .collect(),
                        )
                    }
                    _ => Value::Auto,
                })
            }
            (Content::Figure(e), f) => e.get_field(f),
            (Content::FigureCaption(e), f) => e.get_field(f),
            (Content::Equation(e), f) => e.get_field(f),
            (Content::MathMatrix(e), f) => e.get_field(f),
            (Content::TableCell(e), f) => e.get_field(f),
            (Content::MathPrimes(e), f) => e.get_field(f),
            (Content::MathLr(e), f) => e.get_field(f),
            (Content::MathUnderbrace(e), f) => e.get_field(f),
            (Content::MathUnderbracket(e), f) => e.get_field(f),
            (Content::MathUnderparen(e), f) => e.get_field(f),
            (Content::MathUndershell(e), f) => e.get_field(f),
            (Content::MathStretch(e), f) => e.get_field(f),
            (Content::PdfArtifact(e), f) => e.get_field(f),
            (Content::Styled(_), f) if self.constructor_heading().is_some() => {
                let (base, styles) =
                    self.constructor_heading().expect("constructor guard");
                let lookup = |key: &str| {
                    styles
                        .delta()
                        .custom
                        .iter()
                        .rev()
                        .find(|(candidate, _)| candidate.as_str() == key)
                        .map(|(_, value)| value.clone())
                };
                match f {
                    "supplement" => lookup("heading.supplement"),
                    "numbering" => lookup("heading.numbering").map(|gate| {
                        if gate == Value::Bool(true) {
                            lookup("heading.numbering.pattern").unwrap_or(Value::None)
                        } else {
                            Value::None
                        }
                    }),
                    _ => base.get_field(f),
                }
            }
            (Content::Styled(styled), f) if styled_equation(&styled.body).is_some() => {
                let child = &styled.body;
                let styles = &styled.styles;
                let key = match f {
                    "numbering" => Some("equation.numbering"),
                    "number-align" => Some("equation.number-align"),
                    "supplement" => Some("equation.supplement"),
                    "alt" => Some("equation.alt"),
                    _ => None,
                };
                key.and_then(|key| {
                    styles
                        .delta()
                        .custom
                        .iter()
                        .rev()
                        .find(|(candidate, _)| candidate.as_str() == key)
                        .map(|(_, value)| value.clone())
                })
                .or_else(|| child.get_field(f))
            }
            // P408: smallcaps expõe `body` para show rules (`it.body`).
            (Content::SmallCaps { body }, "body") => {
                Some(Value::Content(body.as_ref().clone()))
            }
            // P863: parágrafo expõe `body` para show rules.
            (Content::Par { body }, "body") => {
                Some(Value::Content(body.as_ref().clone()))
            }
            // Lote F-1 (P334): leitura de campos da fronteira dinâmica (S7) —
            // o que o closure de `#show` usará (F-2+).
            (Content::Dynamic(e), f) => e.dyn_get_field(f),
            // **P844** (achado #47 de P831) — `query(<meta>).first().value`
            // (paridade vanilla `MetadataElem.value`).
            (Content::Metadata(e), "value") => Some(e.value.as_ref().clone()),
            (
                Content::Empty
                | Content::Text(..)
                | Content::Space
                | Content::SpaceFromNewline
                | Content::Parbreak
                | Content::Sequence(..)
                | Content::HtmlElem(..)
                | Content::HtmlFrame(..)
                | Content::Par { .. }
                | Content::ListItem(..)
                | Content::EnumItem(..)
                | Content::Link(..)
                | Content::MathSequence(..)
                | Content::MathIdent(..)
                | Content::MathText(..)
                | Content::MathFrac(..)
                | Content::MathAttach(..)
                | Content::MathRoot(..)
                | Content::MathDelimited(..)
                | Content::MathMid(..)
                | Content::MathOverbrace(..)
                | Content::MathOverbracket(..)
                | Content::MathOverparen(..)
                | Content::MathOvershell(..)
                | Content::MathAlignPoint(..)
                | Content::MathCases(..)
                | Content::MathAccent(..)
                | Content::MathCancel(..)
                | Content::MathUnderline(..)
                | Content::MathOverline(..)
                | Content::MathVec(..)
                | Content::MathClassOverride(..)
                | Content::MathLimitsOverride(..)
                | Content::MathUnderover(..)
                | Content::MathOp(..)
                | Content::MathStyled(..)
                | Content::Label(..)
                | Content::Ref(..)
                | Content::CounterDisplay(..)
                | Content::CounterUpdate(..)
                | Content::Outline(..)
                | Content::Image(..)
                | Content::Shape(..)
                | Content::Curve(..)
                | Content::Grid(..)
                | Content::GridHeader(..)
                | Content::GridFooter(..)
                | Content::GridCell(..)
                | Content::SetPage(..)
                | Content::PageRun(..)
                | Content::Align(..)
                | Content::Place(..)
                | Content::Flush(..)
                | Content::Styled(..)
                | Content::Divider(..)
                | Content::Terms(..)
                | Content::TermItem(..)
                | Content::Quote(..)
                | Content::Document { .. }
                | Content::Asset { .. }
                | Content::SmartQuote(..)
                | Content::PdfAttach(..)
                | Content::Underline(..)
                | Content::Strike(..)
                | Content::Overline(..)
                | Content::SmallCaps { .. }
                | Content::Pad(..)
                | Content::Hide(..)
                | Content::HSpace(..)
                | Content::VSpace(..)
                | Content::Pagebreak(..)
                | Content::Colbreak(..)
                | Content::Stack(..)
                | Content::Boxed(..)
                | Content::Block(..)
                | Content::Bibliography(..)
                | Content::Cite(..)
                | Content::Footnote(..)
                | Content::FootnoteEntry(..)
                | Content::TableHeader(..)
                | Content::TableFooter(..)
                | Content::GridHLine(..)
                | Content::GridVLine(..)
                | Content::TableHLine(..)
                | Content::TableVLine(..)
                | Content::Table(..)
                | Content::Repeat(..)
                | Content::Columns(..)
                | Content::Metadata(..)
                | Content::State(..)
                | Content::StateUpdate(..)
                | Content::StateDisplay(..)
                | Content::CounterDisplayCallback(..)
                | Content::ContextBlock(..),
                _,
            ) => None, // neutro: N16[β] — variantes de Content sem campos expostos retornam None em get_field
        }
    }
}

#[cfg(test)]
mod fields_tests {
    use super::Content;

    #[test]
    fn unsupported_field_is_absent() {
        assert_eq!(Content::Empty.get_field("missing"), None);
    }
}

#[cfg(test)]
mod content_view_regressions {
    use crate::entities::{
        content::{
            Content, LocatedContent, RealizedContent, ShowRecipeContent, SourceContent,
        },
        elements::equation::EquationElem,
        file_id::FileId,
        locator::Locator,
        span::Span,
        style::Styles,
        style_chain::StyleChain,
        value::Value,
    };
    use std::sync::Arc;
    fn wrap(kind: usize, body: Content) -> Content {
        match kind {
            0 => Content::Source(Arc::new(SourceContent {
                body,
                span: Span::from_range(
                    FileId::from_raw(std::num::NonZeroU16::new(7).unwrap()),
                    0..1,
                ),
            })),
            1 => Content::Located(Arc::new(LocatedContent {
                body,
                location: Locator::new().next(),
            })),
            2 => Content::Realized(Arc::new(RealizedContent {
                original: Content::text("causal original"),
                visual: body,
            })),
            3 => Content::ShowRecipe(Arc::new(ShowRecipeContent {
                body,
                textual: false,
                context_id_floor: 0,
                rules: Arc::from([]),
                styles: StyleChain::default_chain(),
                guards: vec![],
                file: FileId::from_raw(std::num::NonZeroU16::new(7).unwrap()),
            })),
            _ => panic!("probe kind"),
        }
    }
    fn styles() -> Styles {
        Styles::new()
            .push_custom("equation.numbering", Value::Str("(1)".into()))
            .push_custom("equation.supplement", Value::Str("Eq".into()))
            .push_custom("equation.alt", Value::Str("alternative".into()))
            .push_custom("equation.number-align", Value::None)
    }

    #[test]
    fn content_views_styled_equation_carriers_preserve_fields() {
        let eq = Content::Equation(Arc::new(EquationElem::new(Content::text("x"), true)));
        let expected = Content::styled(eq.clone(), styles());
        let fields = [
            "numbering",
            "supplement",
            "alt",
            "number-align",
            "body",
            "block",
            "missing",
        ];
        let mut differences = Vec::new();
        for kind in 0..4 {
            for position in 0..3 {
                let observed = match position {
                    0 => Content::styled(wrap(kind, eq.clone()), styles()),
                    1 => wrap(kind, expected.clone()),
                    2 => Content::styled(
                        wrap(kind, Content::styled(eq.clone(), Styles::new())),
                        styles(),
                    ),
                    _ => unreachable!(),
                };
                for field in fields {
                    let a = expected.get_field(field);
                    let z = observed.get_field(field);
                    if a != z {
                        differences
                            .push(format!("{kind}/{position}/{field}: {a:?} != {z:?}"));
                    }
                }
            }
        }
        assert!(differences.is_empty(), "{}", differences.join("\n"));
    }
    #[test]
    fn content_views_non_equations_and_style_precedence_stay_separate() {
        let eq = Content::Equation(Arc::new(EquationElem::new(Content::text("x"), true)));
        for kind in 0..4 {
            let non_equation = Content::styled(wrap(kind, Content::text("x")), styles());
            for key in ["alt", "body", "block", "numbering"] {
                assert_eq!(non_equation.get_field(key), None, "{kind}/{key}");
            }
            let visual_text = Content::styled(
                Content::Realized(Arc::new(RealizedContent {
                    original: eq.clone(),
                    visual: Content::text("x"),
                })),
                styles(),
            );
            assert_eq!(visual_text.get_field("alt"), None);
            let inner = Content::styled(
                wrap(kind, eq.clone()),
                Styles::new().push_custom("equation.alt", Value::Str("inner".into())),
            );
            let outer = Content::styled(
                wrap(kind, inner.clone()),
                Styles::new().push_custom("equation.alt", Value::Str("outer".into())),
            );
            assert_eq!(
                outer.get_field("alt"),
                Some(Value::Str("outer".into())),
                "{kind}/outer"
            );
            let fallback = Content::styled(wrap(kind, inner), Styles::new());
            assert_eq!(
                fallback.get_field("alt"),
                Some(Value::Str("inner".into())),
                "{kind}/inner"
            );
        }
    }
}

#[cfg(test)]
mod sequence_reflection_tests {
    use super::Content;
    use crate::entities::{span::Span, value::Value};

    #[test]
    fn sequence_reflection_getter_keeps_immediate_children_and_empty_text() {
        let children = vec![
            Content::text(""),
            Content::sequence(vec![Content::text("A"), Content::text("B")]),
        ];
        let span = Span::from_range(
            crate::entities::file_id::FileId::from_raw(
                std::num::NonZeroU16::new(7).unwrap(),
            ),
            0..1,
        );
        let c = Content::Sequence(children.clone().into()).with_source_span(span);
        assert_eq!(
            c.get_field("children"),
            Some(Value::Array(children.into_iter().map(Value::Content).collect()))
        );
        assert_eq!(Content::Empty.get_field("children"), Some(Value::Array(vec![])));
        assert_eq!(Content::text("").get_field("children"), None);
        assert_eq!(c.get_field("body"), None);
    }
}
