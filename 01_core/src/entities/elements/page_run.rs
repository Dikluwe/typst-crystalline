//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/page_run.md
//! @prompt-hash ca20382f
//! @layer L1
//! @updated 2026-08-24
//!
//! `PageRunElem` — configuração lexical de página aplicada somente ao body.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::layout_types::{PageDimension, PageMarginSpec};
use crate::entities::page_canvas::{PageBleedSpec, PageFill};
use crate::entities::page_geometry::{PageBinding, Paper};
use crate::entities::source_result::SourceResult;

#[derive(Debug, Clone, PartialEq)]
pub struct PageRunElem {
    pub paper: Option<Paper>,
    pub flipped: Option<bool>,
    pub binding: Option<PageBinding>,
    pub width: Option<PageDimension>,
    pub height: Option<PageDimension>,
    pub margin: Option<PageMarginSpec>,
    pub numbering: Option<Option<crate::entities::numbering::Numbering>>,
    pub number_align: Option<crate::entities::page_running::PageNumberAlign>,
    pub header: Option<crate::entities::page_running::PageMarginal>,
    pub header_ascent: Option<crate::entities::page_running::PageMarginalOffset>,
    pub footer: Option<crate::entities::page_running::PageMarginal>,
    pub footer_descent: Option<crate::entities::page_running::PageMarginalOffset>,
    pub supplement: Option<crate::entities::page_supplement::PageSupplement>,
    pub columns: Option<usize>,
    pub bleed: Option<PageBleedSpec>,
    pub fill: Option<PageFill>,
    pub background: Option<Option<Content>>,
    pub foreground: Option<Option<Content>>,
    pub body: Content,
}

impl std::hash::Hash for PageRunElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        use std::hash::Hash;
        format!("{self:?}").hash(state);
    }
}

impl Element for PageRunElem {
    fn plain_text(&self) -> String {
        self.body.plain_text()
    }

    fn is_empty(&self) -> bool {
        false
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::PageRun(Arc::new(Self {
            paper: self.paper,
            flipped: self.flipped,
            binding: self.binding,
            width: self.width,
            height: self.height,
            margin: self.margin,
            numbering: self.numbering.clone(),
            number_align: self.number_align,
            header: self
                .header
                .as_ref()
                .map(|value| match value {
                    crate::entities::page_running::PageMarginal::Content(content) => {
                        content
                            .map_content(transform)
                            .map(Arc::new)
                            .map(crate::entities::page_running::PageMarginal::Content)
                    }
                    other => Ok(other.clone()),
                })
                .transpose()?,
            header_ascent: self.header_ascent,
            footer: self
                .footer
                .as_ref()
                .map(|value| match value {
                    crate::entities::page_running::PageMarginal::Content(content) => {
                        content
                            .map_content(transform)
                            .map(Arc::new)
                            .map(crate::entities::page_running::PageMarginal::Content)
                    }
                    other => Ok(other.clone()),
                })
                .transpose()?,
            footer_descent: self.footer_descent,
            supplement: self
                .supplement
                .as_ref()
                .map(|value| match value {
                    crate::entities::page_supplement::PageSupplement::Content(
                        content,
                    ) => content
                        .map_content(transform)
                        .map(Arc::new)
                        .map(crate::entities::page_supplement::PageSupplement::Content),
                    other => Ok(other.clone()),
                })
                .transpose()?,
            columns: self.columns,
            bleed: self.bleed,
            fill: self.fill.clone(),
            background: self
                .background
                .as_ref()
                .map(|layer| {
                    layer
                        .as_ref()
                        .map(|content| content.map_content(transform))
                        .transpose()
                })
                .transpose()?,
            foreground: self
                .foreground
                .as_ref()
                .map(|layer| {
                    layer
                        .as_ref()
                        .map(|content| content.map_content(transform))
                        .transpose()
                })
                .transpose()?,
            body: self.body.map_content(transform)?,
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::PageRun(Arc::new(Self {
            paper: self.paper,
            flipped: self.flipped,
            binding: self.binding,
            width: self.width,
            height: self.height,
            margin: self.margin,
            numbering: self.numbering.clone(),
            number_align: self.number_align,
            header: self.header.as_ref().map(|value| match value {
                crate::entities::page_running::PageMarginal::Content(content) => {
                    crate::entities::page_running::PageMarginal::Content(Arc::new(
                        content.map_text(transform),
                    ))
                }
                other => other.clone(),
            }),
            header_ascent: self.header_ascent,
            footer: self.footer.as_ref().map(|value| match value {
                crate::entities::page_running::PageMarginal::Content(content) => {
                    crate::entities::page_running::PageMarginal::Content(Arc::new(
                        content.map_text(transform),
                    ))
                }
                other => other.clone(),
            }),
            footer_descent: self.footer_descent,
            supplement: self.supplement.as_ref().map(|value| match value {
                crate::entities::page_supplement::PageSupplement::Content(content) => {
                    crate::entities::page_supplement::PageSupplement::Content(Arc::new(
                        content.map_text(transform),
                    ))
                }
                other => other.clone(),
            }),
            columns: self.columns,
            bleed: self.bleed,
            fill: self.fill.clone(),
            background: self
                .background
                .as_ref()
                .map(|layer| layer.as_ref().map(|content| content.map_text(transform))),
            foreground: self
                .foreground
                .as_ref()
                .map(|layer| layer.as_ref().map(|content| content.map_text(transform))),
            body: self.body.map_text(transform),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(body: Content) -> PageRunElem {
        PageRunElem {
            paper: None,
            flipped: None,
            binding: None,
            width: Some(PageDimension::Length(100.0)),
            height: None,
            margin: None,
            numbering: None,
            number_align: None,
            header: None,
            header_ascent: None,
            footer: None,
            footer_descent: None,
            supplement: None,
            columns: Some(2),
            bleed: None,
            fill: None,
            background: None,
            foreground: None,
            body,
        }
    }

    #[test]
    fn p1140_19_body_vazio_continua_nao_vazio() {
        assert!(!run(Content::Empty).is_empty());
        assert_eq!(run(Content::Empty).plain_text(), "");
    }

    #[test]
    fn p1140_19_map_text_recurse_e_preserva_configuracao() {
        let mapped = run(Content::text("a")).map_text(&mut |s| s.to_uppercase());
        let Content::PageRun(mapped) = mapped else { panic!("esperado PageRun") };
        assert_eq!(mapped.width, Some(PageDimension::Length(100.0)));
        assert_eq!(mapped.columns, Some(2));
        assert_eq!(mapped.body.plain_text(), "A");
    }
}
