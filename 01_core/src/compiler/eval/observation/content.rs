//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/observation/content.md
//! @prompt-hash 11dd0499
//! @layer L1
//! @updated 2026-10-07
//!
//! Relação observacional causal — família `content`.

use super::*;

// Mechanical declaration expansion: exhaustive matches and all-field destructuring.
impl ObservationEq for crate::entities::content::Content {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Source(a) => match other {
                Self::Source(b) if a.span == b.span => {
                    a.body.observation_relation(&b.body)
                }
                _ => ObservationRelation::Different,
            },
            Self::Located(a) => match other {
                Self::Located(b) => {
                    if a.location == b.location {
                        a.body.observation_relation(&b.body)
                    } else {
                        ObservationRelation::Different
                    }
                }
                _ => ObservationRelation::Different,
            },
            Self::Realized(a) => match other {
                Self::Realized(b) => a
                    .original
                    .observation_relation(&b.original)
                    .and(a.visual.observation_relation(&b.visual)),
                _ => ObservationRelation::Different,
            },
            Self::ShowRecipe(a) => match other {
                Self::ShowRecipe(b) => {
                    let crate::entities::content::ShowRecipeContent {
                        body,
                        textual,
                        context_id_floor,
                        rules,
                        styles,
                        guards,
                        file,
                    } = a.as_ref();
                    body.observation_relation(&b.body)
                        .and(textual.observation_relation(&b.textual))
                        .and(context_id_floor.observation_relation(&b.context_id_floor))
                        .and(rules.observation_relation(&b.rules))
                        .and(styles.observation_relation(&b.styles))
                        .and(guards.observation_relation(&b.guards))
                        .and(file.observation_relation(&b.file))
                }
                _ => ObservationRelation::Different,
            },

            Self::Empty => match other {
                Self::Empty => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Symbol(a0) => match other {
                Self::Symbol(b0) => a0.observation_relation(b0),
                _ => ObservationRelation::Different,
            },
            Self::Text(a0) => match other {
                Self::Text(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Space => match other {
                Self::Space => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            // Origem causal distinta para a estabilização; o == da linguagem
            // permanece independente desta relação observacional.
            Self::SpaceFromNewline => match other {
                Self::SpaceFromNewline => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Parbreak => match other {
                Self::Parbreak => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Sequence(a0) => match other {
                Self::Sequence(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::HtmlElem(a0) => match other {
                Self::HtmlElem(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::HtmlFrame(a0) => match other {
                Self::HtmlFrame(b0) => {
                    ObservationRelation::Same.and(a0.body.observation_relation(&b0.body))
                }
                _ => ObservationRelation::Different,
            },
            Self::Par { body: a0 } => match other {
                Self::Par { body: b0 } => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Heading(a0) => match other {
                Self::Heading(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Title(a0) => match other {
                Self::Title(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Strong(a0) => match other {
                Self::Strong(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Emph(a0) => match other {
                Self::Emph(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Raw(a0) => match other {
                Self::Raw(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::ListItem(a0) => match other {
                Self::ListItem(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::EnumItem(a0) => match other {
                Self::EnumItem(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Link(a0) => match other {
                Self::Link(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Equation(a0) => match other {
                Self::Equation(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathSequence(a0) => match other {
                Self::MathSequence(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathIdent(a0) => match other {
                Self::MathIdent(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathText(a0) => match other {
                Self::MathText(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathFrac(a0) => match other {
                Self::MathFrac(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathAttach(a0) => match other {
                Self::MathAttach(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathPrimes(a0) => match other {
                Self::MathPrimes(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathRoot(a0) => match other {
                Self::MathRoot(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathStretch(a0) => match other {
                Self::MathStretch(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathDelimited(a0) => match other {
                Self::MathDelimited(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathLr(a0) => match other {
                Self::MathLr(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathMid(a0) => match other {
                Self::MathMid(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathOverbrace(a0) => match other {
                Self::MathOverbrace(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathUnderbrace(a0) => match other {
                Self::MathUnderbrace(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathUnderbracket(a0) => match other {
                Self::MathUnderbracket(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathUnderparen(a0) => match other {
                Self::MathUnderparen(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathUndershell(a0) => match other {
                Self::MathUndershell(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathOverbracket(a0) => match other {
                Self::MathOverbracket(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathOverparen(a0) => match other {
                Self::MathOverparen(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathOvershell(a0) => match other {
                Self::MathOvershell(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathAlignPoint(a0) => match other {
                Self::MathAlignPoint(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Linebreak(a0) => match other {
                Self::Linebreak(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathMatrix(a0) => match other {
                Self::MathMatrix(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathCases(a0) => match other {
                Self::MathCases(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathAccent(a0) => match other {
                Self::MathAccent(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathCancel(a0) => match other {
                Self::MathCancel(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathUnderline(a0) => match other {
                Self::MathUnderline(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathOverline(a0) => match other {
                Self::MathOverline(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathVec(a0) => match other {
                Self::MathVec(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathClassOverride(a0) => match other {
                Self::MathClassOverride(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathLimitsOverride(a0) => match other {
                Self::MathLimitsOverride(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathUnderover(a0) => match other {
                Self::MathUnderover(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathOp(a0) => match other {
                Self::MathOp(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::MathStyled(a0) => match other {
                Self::MathStyled(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Label(a0) => match other {
                Self::Label(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Ref(a0) => match other {
                Self::Ref(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::CounterDisplay(a0) => match other {
                Self::CounterDisplay(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::CounterUpdate(a0) => match other {
                Self::CounterUpdate(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Outline(a0) => match other {
                Self::Outline(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Figure(a0) => match other {
                Self::Figure(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::FigureCaption(a0) => match other {
                Self::FigureCaption(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Image(a0) => match other {
                Self::Image(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Shape(a0) => match other {
                Self::Shape(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Curve(a0) => match other {
                Self::Curve(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Transform(a0) => match other {
                Self::Transform(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Grid(a0) => match other {
                Self::Grid(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::GridHeader(a0) => match other {
                Self::GridHeader(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::GridFooter(a0) => match other {
                Self::GridFooter(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::GridCell(a0) => match other {
                Self::GridCell(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::SetPage(a) => match other {
                Self::SetPage(b) => ObservationRelation::Same
                    .and(a.paper.observation_relation(&b.paper))
                    .and(a.flipped.observation_relation(&b.flipped))
                    .and(a.binding.observation_relation(&b.binding))
                    .and(a.width.observation_relation(&b.width))
                    .and(a.height.observation_relation(&b.height))
                    .and(a.margin.observation_relation(&b.margin))
                    .and(a.numbering.observation_relation(&b.numbering))
                    .and(a.number_align.observation_relation(&b.number_align))
                    .and(a.header.observation_relation(&b.header))
                    .and(a.header_ascent.observation_relation(&b.header_ascent))
                    .and(a.footer.observation_relation(&b.footer))
                    .and(a.footer_descent.observation_relation(&b.footer_descent))
                    .and(a.supplement.observation_relation(&b.supplement))
                    .and(a.columns.observation_relation(&b.columns))
                    .and(a.bleed.observation_relation(&b.bleed))
                    .and(a.fill.observation_relation(&b.fill))
                    .and(a.background.observation_relation(&b.background))
                    .and(a.foreground.observation_relation(&b.foreground)),
                _ => ObservationRelation::Different,
            },
            Self::PageRun(a0) => match other {
                Self::PageRun(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Align(a0) => match other {
                Self::Align(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Place(a0) => match other {
                Self::Place(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Flush(a0) => match other {
                Self::Flush(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Styled(a) => match other {
                Self::Styled(b) => ObservationRelation::Same
                    .and(a.body.observation_relation(&b.body))
                    .and(a.styles.observation_relation(&b.styles)),
                _ => ObservationRelation::Different,
            },
            Self::Divider(a0) => match other {
                Self::Divider(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Terms(a0) => match other {
                Self::Terms(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::TermItem(a0) => match other {
                Self::TermItem(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Quote(a0) => match other {
                Self::Quote(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Document {
                path: a0,
                body: a1,
                format: a2,
                title: a3,
                author: a4,
                date: a5,
                keywords: a6,
            } => match other {
                Self::Document {
                    path: b0,
                    body: b1,
                    format: b2,
                    title: b3,
                    author: b4,
                    date: b5,
                    keywords: b6,
                } => ObservationRelation::Same
                    .and(a0.observation_relation(b0))
                    .and(a1.observation_relation(b1))
                    .and(a2.observation_relation(b2))
                    .and(a3.observation_relation(b3))
                    .and(a4.observation_relation(b4))
                    .and(a5.observation_relation(b5))
                    .and(a6.observation_relation(b6)),
                _ => ObservationRelation::Different,
            },
            Self::Asset { path: a0, data: a1 } => match other {
                Self::Asset { path: b0, data: b1 } => {
                    if a0 == b0 && a1 == b1 {
                        ObservationRelation::Same
                    } else {
                        ObservationRelation::Different
                    }
                }
                _ => ObservationRelation::Different,
            },
            Self::SmartQuote(a0) => match other {
                Self::SmartQuote(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::PdfAttach(a0) => match other {
                Self::PdfAttach(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::PdfArtifact(a0) => match other {
                Self::PdfArtifact(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Underline(a0) => match other {
                Self::Underline(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Strike(a0) => match other {
                Self::Strike(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Overline(a0) => match other {
                Self::Overline(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::SmallCaps { body: a0 } => match other {
                Self::SmallCaps { body: b0 } => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Pad(a0) => match other {
                Self::Pad(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Hide(a0) => match other {
                Self::Hide(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::HSpace(a0) => match other {
                Self::HSpace(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::VSpace(a0) => match other {
                Self::VSpace(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Pagebreak(a0) => match other {
                Self::Pagebreak(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Colbreak(a0) => match other {
                Self::Colbreak(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Stack(a0) => match other {
                Self::Stack(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Boxed(a0) => match other {
                Self::Boxed(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Block(a0) => match other {
                Self::Block(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::TableCell(a0) => match other {
                Self::TableCell(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Bibliography(a0) => match other {
                Self::Bibliography(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Cite(a0) => match other {
                Self::Cite(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Footnote(a0) => match other {
                Self::Footnote(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::FootnoteEntry(a0) => match other {
                Self::FootnoteEntry(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::TableHeader(a0) => match other {
                Self::TableHeader(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::TableFooter(a0) => match other {
                Self::TableFooter(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::GridHLine(a0) => match other {
                Self::GridHLine(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::GridVLine(a0) => match other {
                Self::GridVLine(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::TableHLine(a0) => match other {
                Self::TableHLine(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::TableVLine(a0) => match other {
                Self::TableVLine(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Table(a0) => match other {
                Self::Table(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Repeat(a0) => match other {
                Self::Repeat(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Columns(a0) => match other {
                Self::Columns(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Metadata(a0) => match other {
                Self::Metadata(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::State(a0) => match other {
                Self::State(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::StateUpdate(a0) => match other {
                Self::StateUpdate(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::StateDisplay(a0) => match other {
                Self::StateDisplay(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::CounterDisplayCallback(a0) => match other {
                Self::CounterDisplayCallback(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::ContextBlock(a0) => match other {
                Self::ContextBlock(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::OutlineEntry(a0) => match other {
                Self::OutlineEntry(b0) if a0 == b0 => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Dynamic(a0) => match other {
                Self::Dynamic(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::content::Content;

    #[test]
    fn distingue_variante_e_diferenca_tardia_na_arvore() {
        let same = Content::sequence(vec![Content::text("a"), Content::text("b")]);
        let late = Content::sequence(vec![Content::text("a"), Content::text("c")]);
        assert_eq!(same.observation_relation(&same), ObservationRelation::Same);
        assert_eq!(same.observation_relation(&late), ObservationRelation::Different);
        assert_eq!(
            Content::Space.observation_relation(&Content::Empty),
            ObservationRelation::Different
        );
    }
}

#[cfg(test)]
mod figure_recipe_observation_tests {
    use super::*;
    use crate::entities::content::{Content, ShowRecipeContent};
    use crate::entities::elements::strong::StrongElem;
    use crate::entities::file_id::FileId;
    use crate::entities::func::Func;
    use crate::entities::show::{Selector, ShowRule, Transformation};
    use crate::entities::style_chain::{StyleChain, StyleDelta};
    use crate::entities::value::Value;
    use std::num::NonZeroU16;
    use std::sync::Arc;

    fn file(raw: u16) -> FileId {
        FileId::from_raw(NonZeroU16::new(raw).unwrap())
    }

    fn recipe() -> ShowRecipeContent {
        ShowRecipeContent {
            body: Content::text("body"),
            textual: false,
            context_id_floor: 11,
            rules: vec![
                ShowRule {
                    id: 1,
                    selector: Selector::Text("a".into()),
                    transform: Transformation::Str("b".into()),
                },
                ShowRule {
                    id: 2,
                    selector: Selector::Text("c".into()),
                    transform: Transformation::Content(Content::text("d")),
                },
            ]
            .into(),
            styles: StyleChain::empty().push_custom("captured", Value::Int(1)),
            guards: vec![1, 2],
            file: file(1),
        }
    }

    fn content(recipe: ShowRecipeContent) -> Content {
        Content::ShowRecipe(Arc::new(recipe))
    }

    #[test]
    fn figure_v16_recipe_observation_compares_every_causal_field_and_order() {
        let original = recipe();
        let baseline = content(original.clone());
        assert_eq!(
            baseline.observation_relation(&content(recipe())),
            ObservationRelation::Same
        );
        let mut changes = Vec::new();
        let mut changed = original.clone();
        changed.body = Content::text("changed");
        changes.push(("body", changed));
        let mut changed = original.clone();
        changed.textual = true;
        changes.push(("textual", changed));
        let mut changed = original.clone();
        changed.context_id_floor += 1;
        changes.push(("floor", changed));
        let mut changed = original.clone();
        let mut rules = changed.rules.to_vec();
        rules[0].id += 1;
        changed.rules = rules.into();
        changes.push(("rule id", changed));
        let mut changed = original.clone();
        let mut rules = changed.rules.to_vec();
        rules[0].selector = Selector::Text("changed".into());
        changed.rules = rules.into();
        changes.push(("rule selector", changed));
        let mut changed = original.clone();
        let mut rules = changed.rules.to_vec();
        rules[0].transform = Transformation::Str("changed".into());
        changed.rules = rules.into();
        changes.push(("rule transform", changed));
        let mut changed = original.clone();
        let mut rules = changed.rules.to_vec();
        rules.reverse();
        changed.rules = rules.into();
        changes.push(("rule order", changed));
        let mut changed = original.clone();
        changed.styles = changed.styles.push(StyleDelta::empty());
        changes.push(("style presence", changed));
        let mut changed = original.clone();
        changed.styles = StyleChain::empty().push_custom("captured", Value::Int(2));
        changes.push(("style value", changed));
        let mut changed = original.clone();
        changed.guards.reverse();
        changes.push(("guard order", changed));
        let mut changed = original.clone();
        changed.guards.push(3);
        changes.push(("guard presence", changed));
        let mut changed = original;
        changed.file = file(2);
        changes.push(("file", changed));
        for (field, changed) in changes {
            let changed = content(changed);
            assert_eq!(
                baseline.observation_relation(&changed),
                ObservationRelation::Different,
                "{field}"
            );
            assert_eq!(
                changed.observation_relation(&baseline),
                ObservationRelation::Different,
                "{field}, reverse"
            );
        }
        assert_eq!(
            baseline.observation_relation(&Content::Empty),
            ObservationRelation::Different
        );
        assert_eq!(
            Content::Empty.observation_relation(&baseline),
            ObservationRelation::Different
        );
    }

    #[test]
    fn figure_v16_recipe_observation_preserves_opaque_body_styles_and_rule_capabilities()
    {
        let opaque = Content::Dynamic(Arc::new(StrongElem::new(Content::text("opaque"))));
        let mut body = recipe();
        body.body = opaque.clone();
        let mut styles = recipe();
        styles.styles = styles.styles.push_custom("opaque", Value::Content(opaque));
        let mut rules = recipe();
        rules.rules = vec![ShowRule {
            id: 1,
            selector: Selector::Text("a".into()),
            transform: Transformation::Func(Func::element(
                "opaque",
                Arc::new(|_| panic!("observation must not execute callbacks")),
            )),
        }]
        .into();
        for recipe in [body, styles, rules] {
            let original = content(recipe);
            assert_eq!(
                original.observation_relation(&original.clone()),
                ObservationRelation::Unproven
            );
        }
    }
}
