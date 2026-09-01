//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/table.md
//! @prompt-hash 46425796
//! @layer L1
//! @updated 2026-06-25
//!
//! Atomização (ADR-0109, P380): o layout de `Table` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.
//! Cluster `layout_grid` (P379): delega ao mesmo motor que `Grid`.
//! P459: caption numerado posicionado acima da table.

use crate::entities::content::Content;
use crate::entities::counter::CounterKey;
use crate::entities::counter_format::format_counter;
use crate::entities::element_kind::ElementKind;
use crate::entities::elements::table::TableElem;
use crate::entities::elements::table_cell::{TableCellKind, TableHeaderScope};
use crate::entities::introspector::Introspector;
use crate::entities::layout_types::{FrameItem, SemanticKind, SemanticPlacement};
use crate::entities::selector::Selector;
use crate::entities::value::Value;

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de `table(...)` (P157A, ADR-0060 Fase 2): caption numerado acima
/// (P459) e delegação do grid para `layout_grid`.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &TableElem,
) {
    // P459 — ler `table.numbering` da chain e computar prefixo se houver caption.
    let numbering_pattern =
        layouter.chain.custom("table.numbering").and_then(|v| match v {
            Value::Str(s) => Some(s.as_str()),
            _ => None,
        });
    let caption_prefix: Option<String> = if e.caption.is_some() {
        if let Some(pattern) = numbering_pattern {
            // P461 — número via Introspector (`Selector(Kind(Table))`), não campo
            // local do Layouter. `current_location` foi actualizado no topo
            // de `layout_content` porque Table é locatable.
            let table_key = CounterKey::Selector(Selector::Kind(ElementKind::Table));
            let table_number = layouter
                .current_location
                .and_then(|loc| layouter.introspector.flat_counter_at(&table_key, loc))
                .unwrap_or(1);
            // P488 — regista página actual para LoT carry-forward entre iterações fixpoint.
            let page = layouter.current_page_number();
            layouter.runtime.table_page_numbers.push(page);
            let formatted = format_counter(&[table_number], pattern)
                .unwrap_or_else(|| table_number.to_string());
            Some(format!("Table {}: ", formatted))
        } else {
            None
        }
    } else {
        None
    };

    // P459 — caption posicionada ACIMA da table (vanilla default).
    if let Some(cap) = &e.caption {
        if let Some(prefix) = caption_prefix {
            let captioned =
                Content::Sequence(vec![Content::text(prefix), cap.clone()].into());
            layouter.layout_content(&captioned);
        } else {
            layouter.layout_content(cap);
        }
        layouter.layout_content(&Content::linebreak());
    }

    // P224+P227+P228 — Table delegate; herda stroke + fill.
    // P512 — passa hlines/vlines para o motor grid partilhado.
    // P772v — header/footer como row-group real, mesmo mecanismo de P772i.
    // P1050 — passa e.align e e.inset (default 5pt) para layout_grid.
    let completed_pages_before = layouter.pages.len();
    let visual_children = e.children.iter().map(visual_content).collect::<Vec<_>>();
    let visual_header = e.header.as_ref().map(visual_content);
    let visual_footer = e.footer.as_ref().map(visual_content);
    layouter.layout_grid(
        &e.columns,
        &e.rows,
        &visual_children,
        &e.hlines,
        &e.vlines,
        None,
        e.align,
        e.inset,
        visual_header.as_ref(),
        visual_footer.as_ref(),
        e.stroke.as_ref(),
        e.fill.as_ref(),
    );

    // P1288 — a árvore semântica é paralela e visualmente vazia. O grid
    // continua dono do desenho; estes carriers apenas preservam a identidade
    // lógica e os metadados até ao exporter.
    let table_id = e as *const TableElem as usize as u64;
    let header_cells = e.header.as_ref().map(flatten_cells).unwrap_or_default();
    let body_rows = rows_from_cells(&e.children, e.columns.len().max(1), false);
    let header_rows = rows_from_cells(&header_cells, e.columns.len().max(1), true);
    let affected_pages = layouter.pages.len() - completed_pages_before + 1;
    let chunks = partition_rows(&body_rows, affected_pages);
    let grouped = !header_rows.is_empty()
        && !header_rows
            .iter()
            .flatten()
            .any(|cell| cell.kind == TableCellKind::Data);

    for page_offset in 0..affected_pages {
        let carrier = table_carrier(
            table_id,
            e.summary.clone(),
            if page_offset == 0 { &header_rows } else { &[] },
            chunks.get(page_offset).copied().unwrap_or(&[]),
            grouped,
        );
        let page_index = completed_pages_before + page_offset;
        if page_index < layouter.pages.len() {
            layouter.pages[page_index].items.push(carrier);
        } else {
            layouter.regions.current.current_items.push(carrier);
        }
    }
}

fn visual_content(content: &Content) -> Content {
    match content {
        Content::TableCell(cell)
            if cell.x.is_none()
                && cell.y.is_none()
                && cell.colspan.is_none()
                && cell.rowspan.is_none()
                && cell.stroke.is_none()
                && cell.fill.is_none()
                && cell.align.is_none()
                && cell.inset.is_none()
                && cell.breakable.is_none() =>
        {
            visual_content(&cell.body)
        }
        Content::Sequence(items) => {
            Content::sequence(items.iter().map(visual_content).collect())
        }
        Content::TableHeader(header) => {
            Content::table_header(visual_content(&header.body), header.repeat)
        }
        Content::TableFooter(footer) => {
            Content::table_footer(visual_content(&footer.body), footer.repeat)
        }
        other => other.clone(),
    }
}

fn flatten_cells(content: &Content) -> Vec<Content> {
    match content {
        Content::Sequence(items) => items.iter().flat_map(flatten_cells).collect(),
        Content::TableHeader(header) => flatten_cells(&header.body),
        other => vec![other.clone()],
    }
}

#[derive(Clone)]
struct LogicalCell {
    kind: TableCellKind,
    row: u32,
    column: u32,
    rowspan: u32,
    colspan: u32,
}

type LogicalRow = Vec<LogicalCell>;

fn rows_from_cells(
    cells: &[Content],
    columns: usize,
    in_header: bool,
) -> Vec<LogicalRow> {
    let mut rows = Vec::new();
    let mut row = Vec::new();
    let mut column = 0usize;
    let mut row_index = 0u32;
    for content in cells {
        let (kind, rowspan, colspan) = match content {
            Content::TableCell(cell) => (
                match cell.kind {
                    TableCellKind::Auto if in_header => TableCellKind::Header {
                        level: 1,
                        scope: TableHeaderScope::Column,
                    },
                    TableCellKind::Auto => TableCellKind::Data,
                    explicit => explicit,
                },
                cell.rowspan.unwrap_or(1).max(1) as u32,
                cell.colspan.unwrap_or(1).max(1) as u32,
            ),
            _ if in_header => (
                TableCellKind::Header { level: 1, scope: TableHeaderScope::Column },
                1,
                1,
            ),
            _ => (TableCellKind::Data, 1, 1),
        };
        if column > 0 && column + colspan as usize > columns {
            rows.push(std::mem::take(&mut row));
            row_index += 1;
            column = 0;
        }
        row.push(LogicalCell {
            kind,
            row: row_index,
            column: column as u32,
            rowspan,
            colspan,
        });
        column += colspan as usize;
        if column >= columns {
            rows.push(std::mem::take(&mut row));
            row_index += 1;
            column = 0;
        }
    }
    if !row.is_empty() {
        rows.push(row);
    }
    rows
}

fn partition_rows<'a>(rows: &'a [LogicalRow], pages: usize) -> Vec<&'a [LogicalRow]> {
    let mut chunks = Vec::with_capacity(pages);
    let mut start = 0usize;
    for page in 0..pages {
        let remaining_rows = rows.len().saturating_sub(start);
        let remaining_pages = pages - page;
        let take = remaining_rows.div_ceil(remaining_pages);
        let end = (start + take).min(rows.len());
        chunks.push(&rows[start..end]);
        start = end;
    }
    chunks
}

fn table_carrier(
    table_id: u64,
    summary: Option<ecow::EcoString>,
    header_rows: &[LogicalRow],
    body_rows: &[LogicalRow],
    grouped: bool,
) -> FrameItem {
    let mut items = Vec::new();
    if grouped && !header_rows.is_empty() {
        items.push(group_carrier(
            SemanticKind::TableHead { table_id },
            table_id,
            header_rows,
        ));
    }
    if grouped && !body_rows.is_empty() {
        items.push(group_carrier(
            SemanticKind::TableBody { table_id },
            table_id,
            body_rows,
        ));
    } else {
        items.extend(header_rows.iter().map(|row| row_carrier(table_id, row)));
        items.extend(body_rows.iter().map(|row| row_carrier(table_id, row)));
    }
    FrameItem::Semantic {
        kind: SemanticKind::Table { id: table_id },
        placement: SemanticPlacement::Block,
        alt: summary,
        items,
    }
}

fn group_carrier(kind: SemanticKind, table_id: u64, rows: &[LogicalRow]) -> FrameItem {
    FrameItem::Semantic {
        kind,
        placement: SemanticPlacement::Block,
        alt: None,
        items: rows.iter().map(|row| row_carrier(table_id, row)).collect(),
    }
}

fn row_carrier(table_id: u64, row: &LogicalRow) -> FrameItem {
    let row_number = row.first().map_or(0, |cell| cell.row);
    let items = row
        .iter()
        .map(|cell| {
            let kind = match cell.kind {
                TableCellKind::Header { level, scope } => SemanticKind::TableHeaderCell {
                    table_id,
                    row: cell.row,
                    column: cell.column,
                    level,
                    scope,
                    rowspan: cell.rowspan,
                    colspan: cell.colspan,
                },
                TableCellKind::Auto | TableCellKind::Data => {
                    SemanticKind::TableDataCell {
                        table_id,
                        row: cell.row,
                        column: cell.column,
                        rowspan: cell.rowspan,
                        colspan: cell.colspan,
                    }
                }
            };
            FrameItem::Semantic {
                kind,
                placement: SemanticPlacement::Block,
                alt: None,
                items: vec![],
            }
        })
        .collect();
    FrameItem::Semantic {
        kind: SemanticKind::TableRow { table_id, row: row_number },
        placement: SemanticPlacement::Block,
        alt: None,
        items,
    }
}
