//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/table.md
//! @prompt-hash 271d8e84
//! @layer L1

use crate::compiler::eval::operators::error_formatting::vanilla_type_name;
use crate::entities::content::Content;
use crate::entities::elements::table_cell::{TableCellElem, TableCellKind};
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

/// Normaliza o cast de acessibilidade `content-or-table.cell` sem perder os
/// campos visuais e geométricos de uma célula já construída.
pub(crate) fn normalize_table_cell(value: &Value) -> SourceResult<TableCellElem> {
    match value {
        Value::Content(Content::TableCell(cell)) => Ok((**cell).clone()),
        Value::Content(body) => Ok(TableCellElem {
            body: body.clone(),
            x: None,
            y: None,
            colspan: None,
            rowspan: None,
            stroke: None,
            fill: None,
            align: None,
            inset: None,
            breakable: None,
            kind: TableCellKind::Auto,
        }),
        other => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("expected content, found {}", vanilla_type_name(other)),
        )]),
    }
}
