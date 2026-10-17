//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/content/morphology.md
//! @prompt-hash b139a37e
//! @layer L1
//! @updated 2026-10-07

use super::*;

impl Content {
    pub fn morph_canon(&self) -> Content {
        let mut transform = |node: &Content| -> crate::entities::source_result::SourceResult<
            Option<Content>,
        > {
            Ok(match node {
                Content::Source(e) => Some(e.body.morph_canon()),
                Content::ShowRecipe(e) => Some(e.body.morph_canon()),
                Content::Located(e) => Some(e.body.morph_canon()),
                Content::Realized(e) => Some(e.visual.morph_canon()),
                Content::TableCell(e) => {
                    use crate::entities::elements::table_cell::{
                        CellFieldPresence as P, TableCellPresence,
                    };
                    let mut c = e.as_ref().clone();
                    c.source_span = crate::entities::span::Span::detached();
                    c.body = c.body.morph_canon();
                    if let Some(r) = c.standalone.take() {
                        let mut p = e.properties();
                        p.stroke_inherited = None;
                        c.x = p.x;
                        c.y = p.y;
                        c.colspan = p.colspan;
                        c.rowspan = p.rowspan;
                        c.fill = p.fill.clone();
                        c.stroke = p.stroke.clone();
                        c.align = p.align;
                        c.breakable = p.breakable;
                        c.presence = p.presence;
                        c.inset = p.inset.map(|s| {
                            crate::entities::sides::Sides::new(
                                s.left.unwrap_or_default(),
                                s.top.unwrap_or_default(),
                                s.right.unwrap_or_default(),
                                s.bottom.unwrap_or_default(),
                            )
                        });
                        for presence in [
                            &mut c.presence.x,
                            &mut c.presence.y,
                            &mut c.presence.fill,
                            &mut c.presence.align,
                            &mut c.presence.inset,
                            &mut c.presence.breakable,
                        ] {
                            if *presence == P::Omitted {
                                *presence = P::Auto;
                            }
                        }
                        if !c.presence.colspan {
                            c.colspan = Some(1);
                            c.presence.colspan = true;
                        }
                        if !c.presence.rowspan {
                            c.rowspan = Some(1);
                            c.presence.rowspan = true;
                        }
                        let partial = p.inset.is_some_and(|s| {
                            s.left.is_none()
                                || s.top.is_none()
                                || s.right.is_none()
                                || s.bottom.is_none()
                        });
                        if !c.presence.stroke || partial {
                            let mut r = r.as_ref().clone();
                            r.visual = Content::Empty;
                            r.guards.clear();
                            r.stage=crate::entities::elements::table_cell::TableCellStage::Prepared;
                            r.properties = if partial { Some(p) } else { None };
                            c.standalone = Some(Arc::new(r));
                        }
                    }
                    if let Some(r) = c.resolved.take() {
                        c.x = Some(r.x);
                        c.y = Some(r.y);
                        c.colspan = e.properties().colspan.or(Some(r.colspan));
                        c.rowspan = e.properties().rowspan.or(Some(r.rowspan));
                        c.fill = r.fill.clone();
                        c.stroke = r.stroke.clone();
                        c.align = r.align;
                        c.inset = Some(r.inset);
                        c.breakable = Some(r.breakable);
                        c.presence = TableCellPresence {
                            x: P::Explicit,
                            y: P::Explicit,
                            colspan: true,
                            rowspan: true,
                            fill: P::Explicit,
                            stroke: true,
                            align: if c.align.is_some() { P::Explicit } else { P::Auto },
                            inset: P::Explicit,
                            breakable: P::Explicit,
                        };
                    }
                    Some(Content::TableCell(Arc::new(c)))
                }
                Content::Table(e) => {
                    let mut t = e.as_ref().clone();
                    t.source_span = crate::entities::span::Span::detached();
                    t.resolved = None;
                    t.rows_explicit = false;
                    t.children = t.children.iter().map(Content::morph_canon).collect();
                    t.header = t.header.as_ref().map(Content::morph_canon);
                    t.footer = t.footer.as_ref().map(Content::morph_canon);
                    Some(Content::Table(Arc::new(t)))
                }
                Content::Text(s) => Some(Content::Text(s.clone())),
                Content::SpaceFromNewline => Some(Content::Space),
                // F-5a de-bake (P364): heading/equation numbering deixaram de
                // viver em campo assado — viajam como `custom` num
                // `Content::Styled` semanticamente vazio, já tratado
                // transparente pelo arm acima (desce no body). Os arms
                // dedicados (que zeravam `numbering_active`) tornaram-se
                // redundantes e foram removidos: o mecanismo é único (o custom
                // é render, não morfologia — P345 N1).
                Content::Styled(styled) if styled.styles.is_semantically_empty() => {
                    Some(styled.body.clone())
                }
                Content::Empty
                | Content::Space
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
                | Content::Equation(..)
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
                | Content::Styled(..)
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
                | Content::Repeat(..)
                | Content::Columns(..)
                | Content::Metadata(..)
                | Content::State(..)
                | Content::StateUpdate(..)
                | Content::StateDisplay(..)
                | Content::CounterDisplayCallback(..)
                | Content::ContextBlock(..)
                | Content::Dynamic(..) => None, // neutro: N16[β] — variantes sem canonicalização especial retornam None em morph_canon
            })
        };
        // `transform` é total (nunca devolve `Err`) → `map_content` não falha.
        self.map_content(&mut transform)
            .expect("morph_canon: transform total nunca devolve Err")
    }
}

#[cfg(test)]
mod morphology_tests {
    use super::Content;

    #[test]
    fn empty_is_already_canonical() {
        assert_eq!(Content::Empty.morph_canon(), Content::Empty);
    }
}
