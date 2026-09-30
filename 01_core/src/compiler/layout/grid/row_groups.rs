//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/grid/row_groups.md
//! @prompt-hash 77487149
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// Concatena grupos de linhas e sinaliza conflito explícito com o header.
pub(super) fn combine_cells(
    cells: &[Content],
    header: Option<&Content>,
    footer: Option<&Content>,
    num_cols: usize,
) -> Result<Vec<Content>, usize> {
    // P772i — header/footer como row-group real: extrair as células do
    // corpo de `Content::GridHeader`/`GridFooter` (um `Content::Sequence`
    // se houver mais que uma célula, ou o `Content` da própria célula se
    // só houver uma — `Content::sequence` colapsa Vec de 1 elemento) e
    // colar antes/depois das células normais. Cada grupo é preenchido
    // com `Content::Empty` até múltiplo de `num_cols` — headers/footers
    // ocupam linhas inteiras (paridade vanilla, `resolve.rs` §"row
    // group"). **Scope-out explícito** (não silencioso — ver
    // 00_nucleo/diagnosticos/paridade-producao-p772i.md): renderiza uma
    // única vez, não repete em quebras de página (equivalente vanilla de
    // `Header`/`Footer`/`Repeatable<T>` com `range`/`level` não
    // implementado).
    fn row_group_cells(content: &Content, num_cols: usize) -> Vec<Content> {
        let mut group_cells = match content {
            Content::Sequence(items) => items.to_vec(),
            other => vec![other.clone()],
        };
        let remainder = group_cells.len() % num_cols;
        if remainder != 0 {
            group_cells
                .resize(group_cells.len() + (num_cols - remainder), Content::Empty);
        }
        group_cells
    }
    // P772v — `layout_grid` é partilhado por `Content::Grid` e
    // `Content::Table` (cluster P379); os braços `TableHeader`/
    // `TableFooter` estendem o mesmo mecanismo de row-group de P772i
    // para `table()`, extraindo `.body` da mesma forma.
    let header_cells: Vec<Content> = header
        .map(|h| match h {
            Content::GridHeader(e) => row_group_cells(&e.body, num_cols),
            Content::TableHeader(e) => row_group_cells(&e.body, num_cols),
            other => row_group_cells(other, num_cols),
        })
        .unwrap_or_default();
    let footer_cells: Vec<Content> = footer
        .map(|f| match f {
            Content::GridFooter(e) => row_group_cells(&e.body, num_cols),
            Content::TableFooter(e) => row_group_cells(&e.body, num_cols),
            other => row_group_cells(other, num_cols),
        })
        .unwrap_or_default();
    // P789 — detecção de conflito célula↔header (paridade vanilla
    // `check_for_conflicting_cell_row`, resolve.rs:2112): célula do
    // corpo com `y` explícito cujo range `y..y+rowspan` intersecta as
    // linhas do header (0..header_rows) é erro, não sobreposição
    // silenciosa. Mensagem e hint idênticos ao vanilla (observável ao
    // nível da língua — ADR-0107). Só células do corpo são verificadas
    // (equivalente ao `!in_row_group` do vanilla); células com
    // `y: None` são auto-posicionadas e contornam o header. Conflito
    // célula↔footer é scope-out documentado no L0 (layout.md §P789):
    // o range absoluto do footer só existe pós-placement no modelo
    // splice. `Span::detached()` — mesmo trade-off de P647 (elementos
    // não carregam span).
    let header_rows = header_cells.len() / num_cols;
    if header_rows > 0 {
        let conflict = cells.iter().find_map(|cell| {
            let (y, rowspan) = match cell {
                Content::GridCell(e) => (e.y, e.rowspan),
                Content::TableCell(e) => (e.y, e.rowspan),
                _ => (None, None), // neutro: N16[β] — Content não-cell retorna (None, None) para y/rowspan (sem posição explícita)
            };
            let y = y?;
            let rowspan = rowspan.unwrap_or(1).max(1);
            // Linhas do header são contíguas a partir de 0 no modelo
            // splice — a primeira linha do range dentro do header é `y`.
            (y < header_rows).then_some(y)
        });
        if let Some(row) = conflict {
            return Err(row);
        }
    }
    let combined_cells: Vec<Content> =
        if header_cells.is_empty() && footer_cells.is_empty() {
            cells.to_vec()
        } else {
            let mut combined =
                Vec::with_capacity(header_cells.len() + cells.len() + footer_cells.len());
            combined.extend(header_cells);
            combined.extend_from_slice(cells);
            combined.extend(footer_cells);
            combined
        };
    Ok(combined_cells)
}
