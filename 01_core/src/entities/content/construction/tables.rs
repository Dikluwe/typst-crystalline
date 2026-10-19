//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/content/construction/tables.md
//! @prompt-hash c981f713
//! @layer L1
//! @updated 2026-10-07

use super::*;

impl Content {
    /// `table(columns, rows, ..children)` — Passo 157A (ADR-0060
    /// Fase 2 sub-passo 1; **primeiro sub-passo Model Fase 2**).
    /// Subset minimal: cells distribuídas como `Content::Grid`;
    /// TableCell estruturado + Header/Footer diferidos para P157B/C.
    pub fn table(
        columns: Vec<TrackSizing>,
        rows: Vec<TrackSizing>,
        children: Vec<Content>,
    ) -> Self {
        Self::Table(Arc::new(TableElem {
            source_span: crate::entities::span::Span::detached(),
            resolved: None,
            rows_explicit: !rows.is_empty(),
            columns,
            rows,
            children,
            hlines: vec![],
            vlines: vec![],
            header: None,
            footer: None,
            stroke: None,
            fill: None,
            caption: None,
            summary: None,
            inset: crate::entities::sides::Sides::uniform(
                crate::entities::layout_types::Length::pt(5.0),
            ),
            align: None,
        }))
    }

    /// `table_with_caption(columns, rows, children, caption)` — P459.
    /// Variante programática com caption opcional para numeração automática.
    pub fn table_with_caption(
        columns: Vec<TrackSizing>,
        rows: Vec<TrackSizing>,
        children: Vec<Content>,
        caption: Option<Content>,
    ) -> Self {
        Self::Table(Arc::new(TableElem {
            source_span: crate::entities::span::Span::detached(),
            resolved: None,
            rows_explicit: !rows.is_empty(),
            columns,
            rows,
            children,
            hlines: vec![],
            vlines: vec![],
            header: None,
            footer: None,
            stroke: None,
            fill: None,
            caption,
            summary: None,
            inset: crate::entities::sides::Sides::uniform(
                crate::entities::layout_types::Length::pt(5.0),
            ),
            align: None,
        }))
    }

    /// `table_cell(body, x, y, colspan, rowspan)` — Passo 157B
    /// (ADR-0060 Fase 2 sub-passo 2). `x`/`y` ADR-0064 Caso A;
    /// `colspan`/`rowspan` Caso C. Placement algorítmico diferido
    /// em DEBT-34e — fields armazenados mas ignorados em layout.
    pub fn table_cell(
        body: Content,
        x: Option<usize>,
        y: Option<usize>,
        colspan: Option<usize>,
        rowspan: Option<usize>,
    ) -> Self {
        Self::TableCell(Arc::new(TableCellElem {
            source_span: crate::entities::span::Span::detached(),
            resolved: None,
            standalone: None,
            presence: crate::entities::elements::table_cell::TableCellPresence {
                x: if x.is_some() {
                    crate::entities::elements::table_cell::CellFieldPresence::Explicit
                } else {
                    crate::entities::elements::table_cell::CellFieldPresence::Omitted
                },
                y: if y.is_some() {
                    crate::entities::elements::table_cell::CellFieldPresence::Explicit
                } else {
                    crate::entities::elements::table_cell::CellFieldPresence::Omitted
                },
                colspan: colspan.is_some(),
                rowspan: rowspan.is_some(),
                ..Default::default()
            },
            body,
            x,
            y,
            colspan,
            rowspan,
            stroke: None,
            fill: None,
            align: None,
            inset: None,
            breakable: None,
            kind: crate::entities::elements::table_cell::TableCellKind::Auto,
        }))
    }

    /// `table_header(body, repeat)` — Passo 157C (ADR-0060 Fase 2
    /// sub-passo 3 — **fecha table foundations**). `repeat: bool`
    /// ADR-0064 Caso D (default `true` paridade vanilla — primeira
    /// aplicação Caso D em Model). Algoritmo de repetição diferido
    /// em DEBT-56.
    pub fn table_header(body: Content, repeat: bool) -> Self {
        Self::TableHeader(Arc::new(TableHeaderElem { body, repeat }))
    }

    /// `table_footer(body, repeat)` — par simétrico de `table_header`
    /// (Passo 157C). Mesma decisão Caso D + DEBT-56.
    pub fn table_footer(body: Content, repeat: bool) -> Self {
        Self::TableFooter(Arc::new(TableFooterElem { body, repeat }))
    }
    /// `table_hline(start, end, row, stroke, position)` — Passo 512.
    pub fn table_hline(
        start: usize,
        end: Option<usize>,
        row: usize,
        stroke: Option<Stroke>,
        position: EcoString,
    ) -> Self {
        Self::TableHLine(Arc::new(TableHLineElem { start, end, row, stroke, position }))
    }
    /// `table_vline(start, end, col, stroke, position)` — Passo 512.
    pub fn table_vline(
        start: usize,
        end: Option<usize>,
        col: usize,
        stroke: Option<Stroke>,
        position: EcoString,
    ) -> Self {
        Self::TableVLine(Arc::new(TableVLineElem { start, end, col, stroke, position }))
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn modulo_compila_e_carrega() {}
}
