//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/table.md
//! @prompt-hash ba7aec95
//! @layer L1
//! @updated 2026-10-07
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
    // The prepared source table is a block; Grid itself also serves raw
    // programmatic callers and does not own this Table margin boundary.
    let prepared_flow = e.resolved.is_some() && e.caption.is_none();
    let table_indent = if prepared_flow {
        super::paragraph_indent::end(layouter);
        let indent = std::mem::replace(
            &mut layouter.paragraph_indent,
            super::paragraph_indent::State::inline(),
        );
        let (origin, _) = super::block::admit_prepared_frame(layouter);
        layouter.regions.current.cursor_y = origin.y;
        layouter.initial_baseline_pending = true;
        layouter.block_chain_active = false;
        layouter.prev_margin_is_parbreak = false;
        Some(indent)
    } else {
        None
    };
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
            let parts = vec![Content::text(prefix), cap.clone()];
            if let Some(cursor) = layouter.planned_child_cursor(
                crate::compiler::introspect::page_running::PlanEdge::TableCaption,
            ) {
                super::sequence::layout_mixed_source_parts(
                    layouter,
                    &parts,
                    &[None, Some(cursor)],
                );
            } else {
                layouter.layout_content(&Content::Sequence(parts.into()));
            }
        } else {
            layouter.layout_planned_child(
                crate::compiler::introspect::page_running::PlanEdge::TableCaption,
                cap,
            );
        }
        layouter.layout_content(&Content::linebreak());
    }

    // P224+P227+P228 — Table delegate; herda stroke + fill.
    // P512 — passa hlines/vlines para o motor grid partilhado.
    // P772v — header/footer como row-group real, mesmo mecanismo de P772i.
    // P1050 — passa e.align e e.inset (default 5pt) para layout_grid.
    if e.items.is_some() && e.resolved.is_none() {
        layouter.layout_errors.push(
            crate::entities::source_result::SourceDiagnostic::error(
                e.source_span,
                "ordered table items require logical preparation",
            ),
        );
        return;
    }
    let completed_pages_before = layouter.pages.len();
    let planned = layouter
        .page_running_pass
        .as_ref()
        .is_some_and(|pass| pass.cursor.is_some());
    let visual_children = e.resolved.as_ref().map_or_else(
        || {
            if planned {
                e.children.clone()
            } else {
                e.children.iter().map(visual_content).collect::<Vec<_>>()
            }
        },
        |r| r.cells.clone(),
    );
    let visual_header = if e.resolved.is_some() {
        None
    } else if planned {
        e.header.clone()
    } else {
        e.header.as_ref().map(visual_content)
    };
    let visual_footer = if e.resolved.is_some() {
        None
    } else if planned {
        e.footer.clone()
    } else {
        e.footer.as_ref().map(visual_content)
    };
    use crate::entities::elements::table::TableAlignment;
    let inherited_alignment = if e.resolved.is_none() && e.align.is_none() {
        match layouter
            .chain
            .custom("table.align")
            .map(|value| crate::compiler::stdlib::table_alignment(value, e.source_span))
            .transpose()
        {
            Ok(alignment) => alignment,
            Err(errors) => {
                layouter.layout_errors.extend(errors);
                return;
            }
        }
    } else {
        None
    };
    let declaration = e.align.as_ref().or(inherited_alignment.as_ref());
    let (align, table_alignment) = if e.resolved.is_some() {
        (None, None)
    } else {
        match declaration {
            None => (None, None),
            Some(TableAlignment::Uniform(align)) => (Some(*align), None),
            Some(TableAlignment::Auto | TableAlignment::Columns(_)) => {
                (None, declaration)
            }
        }
    };
    layouter.layout_grid(super::grid::GridLayoutInput {
        header_groups: e.resolved.as_ref().map_or(&[], |r| r.header_groups.as_slice()),
        columns: &e.columns,
        rows: &e.rows,
        cells: &visual_children,
        hlines: &e.hlines,
        vlines: &e.vlines,
        gutter: None,
        align,
        table_alignment,
        inset: e.inset,
        header: visual_header.as_ref(),
        footer: visual_footer.as_ref(),
        stroke: e.stroke.as_ref(),
        fill: e.fill.as_ref(),
    });

    if let Some(indent) = table_indent {
        let end = layouter.regions.current.cursor_y.0;
        layouter.prev_line_baseline = end;
        layouter.prev_block_equation_descent = 0.0;
        layouter.last_equation_descent = 0.0;
        layouter.last_block_descent_y = Some(end);
        layouter.last_flush_advance = 0.0;
        layouter.line_assumed_ascent = None;
        layouter.initial_baseline_pending = false;
        layouter.prev_block_below_pending =
            super::paragraph::spacing_for_size(&layouter.chain, layouter.style.size.0);
        layouter.block_chain_active = true;
        layouter.prev_margin_is_parbreak = true;
        layouter.record_flow_placement_end(end);
        layouter.paragraph_indent = indent;
    }

    // P1288 — a árvore semântica é paralela e visualmente vazia. O grid
    // continua dono do desenho; estes carriers apenas preservam a identidade
    // lógica e os metadados até ao exporter.
    let table_id = e as *const TableElem as usize as u64;
    let header_cells = e.header.as_ref().map(flatten_cells).unwrap_or_default();
    let body_cells = if e.items.is_some() {
        e.resolved
            .as_ref()
            .map_or(e.children.as_slice(), |r| r.cells.as_slice())
    } else {
        e.children.as_slice()
    };
    let body_rows = rows_from_cells(body_cells, e.columns.len().max(1), false);
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
        Content::TableCell(cell) => {
            if cell.resolved.is_some() {
                content.clone()
            } else if cell.x.is_none()
                && cell.y.is_none()
                && cell.colspan.is_none()
                && cell.rowspan.is_none()
                && cell.stroke.is_none()
                && cell.fill.is_none()
                && cell.align.is_none()
                && cell.inset.is_none()
                && cell.breakable.is_none()
            {
                visual_content(&cell.body)
            } else {
                content.clone()
            }
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

fn partition_rows(rows: &[LogicalRow], pages: usize) -> Vec<&[LogicalRow]> {
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

#[cfg(test)]
mod readiness_flow_tests {
    use super::*;
    use crate::entities::{
        elements::table::TableGridResolved,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::TrackSizing,
    };
    use comemo::Track;
    use std::sync::Arc;

    fn prepared_table() -> Content {
        let Content::Table(mut table) = Content::table(
            vec![TrackSizing::Fixed(80.0)],
            vec![TrackSizing::Fixed(40.0)],
            vec![Content::text("CELL")],
        ) else {
            unreachable!()
        };
        Arc::make_mut(&mut table).resolved = Some(Arc::new(TableGridResolved {
            header_groups: Vec::new(),
            cells: vec![Content::text("CELL")],
            num_cols: 1,
            num_rows: 1,
            header_rows: 0,
            footer_start: 1,
            complete: true,
        }));
        Content::Table(table)
    }

    #[test]
    fn readiness_table_keeps_following_prose_below_its_rows() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.layout_content(&prepared_table());
        let bottom = l.regions.current.cursor_y.0;
        l.layout_content(&Content::text("AFTER"));
        let top = l.metrics.text_edges(l.style.size, &l.style).0 .0;
        assert!(
            (l.regions.current.cursor_y.0 - bottom - 13.2 - top).abs() < 1e-8,
            "following prose must respect the full row height and below margin"
        );
    }

    #[test]
    fn readiness_table_keeps_following_raw_below_its_rows() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.layout_content(&prepared_table());
        let bottom = l.regions.current.cursor_y.0;
        let outer = l.style.clone();
        let raw = super::super::raw::default_style(&outer, &l.chain);
        let _ = super::super::raw::begin_block(&mut l, raw.size.0, &outer);
        l.style = raw;
        l.layout_content(&Content::text("RAW"));
        assert!(
            l.regions.current.cursor_y.0 > bottom,
            "raw must start after the full preceding table, not its old baseline"
        );
    }
}

#[cfg(test)]
mod raw_alignment_scope_tests {
    use super::*;
    use crate::entities::{
        elements::table::TableAlignment,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{Align2D, FrameItem, FrameVisitor, TrackSizing},
        style::Styles,
        value::Value,
    };
    use comemo::Track;
    use std::sync::Arc;

    fn render(
        inherited: Option<Value>,
        explicit: Option<TableAlignment>,
        sourced: bool,
    ) -> (Vec<(String, f64, f64)>, Vec<crate::entities::source_result::SourceDiagnostic>)
    {
        let intr = TagIntrospector::empty();
        let mut l = super::super::Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        let children = ["AA", "BBBB"]
            .into_iter()
            .enumerate()
            .map(|(index, text)| {
                let body = Content::text(text);
                if sourced {
                    let id = crate::entities::file_id::FileId::from_raw(
                        std::num::NonZeroU16::new(1).unwrap(),
                    );
                    body.with_source_span(crate::entities::span::Span::from_range(
                        id,
                        index..index + 1,
                    ))
                } else {
                    body
                }
            })
            .collect();
        let mut body = Content::table(
            vec![TrackSizing::Fixed(100.0); 2],
            vec![TrackSizing::Fixed(40.0)],
            children,
        );
        let Content::Table(table) = &mut body else { unreachable!() };
        Arc::make_mut(table).align = explicit;
        if let Some(value) = inherited {
            body = Content::styled(body, Styles::new().push_custom("table.align", value));
        }
        l.layout_content(&body);
        let errors = l.layout_errors.clone();
        struct Words(Vec<(String, f64, f64)>);
        impl FrameVisitor for Words {
            fn visit_text(&mut self, item: &FrameItem) {
                if let FrameItem::Text { text, pos, .. } = item {
                    if !text.trim().is_empty() {
                        self.0.push((text.to_string(), pos.x.0, pos.y.0));
                    }
                }
            }
        }
        let mut words = Words(Vec::new());
        for page in l.finish().pages {
            words.visit_items(&page.items);
        }
        (words.0, errors)
    }
    fn declaration() -> TableAlignment {
        TableAlignment::Columns(vec![
            Some(Align2D::from_string("left")),
            Some(Align2D::from_string("right")),
        ])
    }
    fn value() -> Value {
        Value::Array(vec![
            Value::Align(Align2D::from_string("left")),
            Value::Align(Align2D::from_string("right")),
        ])
    }
    fn same(a: &[(String, f64, f64)], b: &[(String, f64, f64)]) {
        assert_eq!(a.len(), 2);
        assert_eq!(a.len(), b.len());
        for (a, b) in a.iter().zip(b) {
            assert_eq!(a.0, b.0);
            assert!(
                (a.1 - b.1).abs() < 0.001 && (a.2 - b.2).abs() < 0.001,
                "{a:?} vs {b:?}"
            );
        }
    }
    #[test]
    fn table_declarative_raw_omission_inherits_at_insertion() {
        let expected = render(None, Some(declaration()), false);
        for sourced in [false, true] {
            let actual = render(Some(value()), None, sourced);
            assert!(actual.1.is_empty());
            same(&actual.0, &expected.0);
        }
    }
    #[test]
    fn table_declarative_raw_explicit_declaration_wins() {
        for explicit in
            [TableAlignment::Auto, TableAlignment::Uniform(Align2D::from_string("left"))]
        {
            let expected = render(None, Some(explicit.clone()), false);
            let actual = render(Some(value()), Some(explicit), true);
            assert!(actual.1.is_empty());
            same(&actual.0, &expected.0);
        }
    }
    #[test]
    fn table_declarative_raw_invalid_style_is_not_silently_ignored() {
        let actual = render(Some(Value::Array(vec![Value::Bool(true)])), None, false);
        assert!(
            actual
                .1
                .iter()
                .any(|e| e.message == "expected alignment or auto, found boolean"),
            "{:?}",
            actual.1
        );
        assert!(actual.0.is_empty());
    }
}
