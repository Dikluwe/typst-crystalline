//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/elements/table.md
//! @prompt-hash 586a64a6
//! @layer L1
//! @updated 2026-10-10
//!
//! `TableElem` — Lote 12 P327 (bloco grid/table cell). `table(columns, rows, …)`.
//! P459: `caption` opcional para numeração automática via `table.numbering`.
//! Contentor: recurse em cada child e no caption em map_content E map_text. Não-locatável.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::counter_update::CounterUpdate;
use crate::entities::element_kind::ElementKind;
use crate::entities::element_payload::ElementPayload;
use crate::entities::elements::Element;
use crate::entities::geometry::Stroke;
use crate::entities::layout_types::{Align2D, Color, Length, TrackSizing};
use crate::entities::sides::Sides;
use crate::entities::source_result::SourceResult;
use ecow::EcoString;

/// Tabela: `children` distribuídos em `columns`×`rows`; `stroke`/`fill` globais.
/// **P459**: `caption` opcional para numeração automática via `table.numbering`.
/// **P512**: `hlines`/`vlines` são renderizadas pelo layout_grid.
/// **P772v**: `header`/`footer` opcionais — mesmo mecanismo de row-group real
/// que `GridElem` ganhou em P772i (extensão directa, sem redesenho).
#[derive(Debug, Clone, PartialEq)]
pub struct TableElem {
    pub source_span: crate::entities::span::Span,
    pub rows_explicit: bool,
    pub resolved: Option<Arc<TableGridResolved>>,
    pub columns: Vec<TrackSizing>,
    pub rows: Vec<TrackSizing>,
    pub children: Vec<Content>,
    pub hlines: Vec<crate::entities::elements::table_hline::TableHLineElem>,
    pub vlines: Vec<crate::entities::elements::table_vline::TableVLineElem>,
    pub header: Option<Content>,
    pub footer: Option<Content>,
    pub stroke: Option<Stroke>,
    pub fill: Option<Color>,
    pub caption: Option<Content>,
    pub summary: Option<EcoString>,
    pub inset: Sides<Length>,
    pub align: Option<Align2D>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableGridResolved {
    pub cells: Vec<Content>,
    pub num_cols: usize,
    pub num_rows: usize,
    pub header_rows: usize,
    pub footer_start: usize,
    pub(crate) complete: bool,
}

// `Hash` manual via `Debug` (paridade `content_hash`): `TrackSizing`/`Stroke`/
// `Color` carregam `f64`. Ressalva `-0.0` vs `0.0` aceitável.
impl std::hash::Hash for TableElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{self:?}").hash(state);
    }
}

impl TableElem {
    pub(crate) fn map_content_if_changed<F>(
        &self,
        transform: &mut F,
    ) -> SourceResult<Option<Content>>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        if let Some(resolved) = &self.resolved {
            let cells = Content::map_children_if_changed(&resolved.cells, transform)?;
            let caption = self
                .caption
                .as_ref()
                .map(|c| c.map_content_if_changed(transform))
                .transpose()?
                .flatten();
            if cells.is_none() && caption.is_none() {
                return Ok(None);
            }
            let mut table = self.clone();
            if let Some(cells) = cells {
                let mut plan = resolved.as_ref().clone();
                plan.cells = cells;
                table.resolved = Some(Arc::new(plan));
            }
            if let Some(caption) = caption {
                table.caption = Some(caption);
            }
            return Ok(Some(Content::Table(Arc::new(table))));
        }
        let children = Content::map_children_if_changed(&self.children, transform)?;
        let header = self
            .header
            .as_ref()
            .map(|c| c.map_content_if_changed(transform))
            .transpose()?
            .flatten();
        let footer = self
            .footer
            .as_ref()
            .map(|c| c.map_content_if_changed(transform))
            .transpose()?
            .flatten();
        let caption = self
            .caption
            .as_ref()
            .map(|c| c.map_content_if_changed(transform))
            .transpose()?
            .flatten();
        if children.is_none() && header.is_none() && footer.is_none() && caption.is_none()
        {
            return Ok(None);
        }
        let mut table = self.clone();
        if let Some(children) = children {
            table.children = children;
        }
        if let Some(header) = header {
            table.header = Some(header);
        }
        if let Some(footer) = footer {
            table.footer = Some(footer);
        }
        if let Some(caption) = caption {
            table.caption = Some(caption);
        }
        Ok(Some(Content::Table(Arc::new(table))))
    }
}

impl Element for TableElem {
    fn plain_text(&self) -> String {
        let cells = self
            .resolved
            .as_ref()
            .map_or(&self.children, |r| &r.cells)
            .iter()
            .map(|c| c.plain_text())
            .collect::<Vec<_>>()
            .join(" ");
        let cap = self.caption.as_ref().map(|c| c.plain_text()).unwrap_or_default();
        match (cells.is_empty(), cap.is_empty()) {
            (false, false) => format!("{} {}", cap, cells),
            (false, true) => cells,
            (true, false) => cap,
            (true, true) => String::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.resolved.as_ref().map_or_else(
            || self.children.is_empty(),
            |r| r.cells.iter().all(Content::is_empty),
        ) && self.caption.as_ref().is_none_or(|c| c.is_empty())
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(self
            .map_content_if_changed(transform)?
            .unwrap_or_else(|| Content::Table(Arc::new(self.clone()))))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        if let Some(r) = &self.resolved {
            let mut table = self.clone();
            let mut r = r.as_ref().clone();
            r.cells = r.cells.iter().map(|c| c.map_text(transform)).collect();
            table.resolved = Some(Arc::new(r));
            table.caption = self.caption.as_ref().map(|c| c.map_text(transform));
            return Content::Table(Arc::new(table));
        }
        Content::Table(Arc::new(TableElem {
            source_span: self.source_span,
            resolved: self.resolved.clone(),
            rows_explicit: self.rows_explicit,
            columns: self.columns.clone(),
            rows: self.rows.clone(),
            children: self.children.iter().map(|c| c.map_text(transform)).collect(),
            hlines: self.hlines.clone(),
            vlines: self.vlines.clone(),
            header: self.header.as_ref().map(|h| h.map_text(transform)),
            footer: self.footer.as_ref().map(|f| f.map_text(transform)),
            stroke: self.stroke.clone(),
            fill: self.fill.clone(),
            caption: self.caption.as_ref().map(|c| c.map_text(transform)),
            summary: self.summary.clone(),
            inset: self.inset,
            align: self.align,
        }))
    }

    fn element_kind(&self) -> Option<ElementKind> {
        Some(ElementKind::Table)
    }

    fn to_payload(&self) -> Option<ElementPayload> {
        // P461: `is_counted` placeholder = caption.is_some(); o gate
        // `table.numbering` é ANDado pela chain no walk top (espelho
        // Figure P365). O elemento não baka o padrão.
        Some(ElementPayload::Table {
            counter_update: CounterUpdate::step(),
            is_counted: self.caption.is_some(),
            caption_text: self.caption.as_ref().map(|c| c.plain_text()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn ex() -> TableElem {
        TableElem {
            source_span: crate::entities::span::Span::detached(),
            resolved: None,
            rows_explicit: false,
            columns: vec![],
            rows: vec![],
            children: vec![Content::text("a"), Content::text("b")],
            hlines: vec![],
            vlines: vec![],
            header: None,
            footer: None,
            stroke: None,
            fill: None,
            caption: None,
            summary: None,
            inset: Sides::uniform(Length::pt(5.0)),
            align: None,
        }
    }

    #[test]
    fn plain_text_concatena_children() {
        assert_eq!(ex().plain_text(), "a b");
    }

    #[test]
    fn is_empty_so_sem_children() {
        assert!(!ex().is_empty());
        assert!(TableElem {
            source_span: crate::entities::span::Span::detached(),
            resolved: None,
            rows_explicit: false,
            columns: vec![],
            rows: vec![],
            children: vec![],
            hlines: vec![],
            vlines: vec![],
            header: None,
            footer: None,
            stroke: None,
            fill: None,
            caption: None,
            summary: None,
            inset: Sides::uniform(Length::pt(5.0)),
            align: None,
        }
        .is_empty());
    }

    #[test]
    fn map_content_recurse_children() {
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s) if s.as_str() == "a" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        match ex().map_content(&mut f).unwrap() {
            Content::Table(e) => {
                assert!(matches!(&e.children[0], Content::Text(s) if s.as_str() == "Z"))
            }
            _ => panic!("esperado Table"),
        }
    }

    fn h(e: &TableElem) -> u64 {
        let mut s = DefaultHasher::new();
        e.hash(&mut s);
        s.finish()
    }

    #[test]
    fn payload_diferente_produz_hash_diferente() {
        let outro = TableElem {
            source_span: crate::entities::span::Span::detached(),
            resolved: None,
            rows_explicit: false,
            columns: vec![],
            rows: vec![],
            children: vec![],
            hlines: vec![],
            vlines: vec![],
            header: None,
            footer: None,
            stroke: None,
            fill: None,
            caption: None,
            summary: None,
            inset: Sides::uniform(Length::pt(5.0)),
            align: None,
        };
        assert_ne!(h(&ex()), h(&outro));
    }
}
