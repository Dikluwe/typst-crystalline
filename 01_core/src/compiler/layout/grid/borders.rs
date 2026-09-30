//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/grid/borders.md
//! @prompt-hash 040618ef
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// P234 — Bounds reais per `PlacedCell` × tracks resolved.
///
/// `current_row_start_y` é o cursor_y no início da emissão da row
/// `placed.row` (já considerou pagination). Para cells com
/// `rowspan > 1`, height = sum(row_heights[row..row+rowspan])
/// estende para baixo a partir de current_row_start_y.
pub(super) fn cell_bounds(
    placed: &PlacedCell,
    col_starts: &[f64],
    resolved_widths: &[f64],
    row_heights: &[f64],
    current_row_start_y: f64,
) -> (f64, f64, f64, f64) {
    let x0 = col_starts.get(placed.col).copied().unwrap_or(0.0);
    let y0 = current_row_start_y;
    let cell_w: f64 = (placed.col..placed.col + placed.colspan)
        .map(|i| resolved_widths.get(i).copied().unwrap_or(0.0))
        .sum();
    let cell_h: f64 = (placed.row..placed.row + placed.rowspan)
        .map(|i| row_heights.get(i).copied().unwrap_or(0.0))
        .sum();
    (x0, y0, cell_w, cell_h)
}

/// P888 (achado 3 de P885, secção 7 de P887) — extrai o `stroke` de uma
/// célula (`TableCell`/`GridCell`), com fallback para o `grid_stroke`
/// (default do `table()`/`grid()`). Mesmo padrão de precedência já usado
/// inline no loop principal (`cell_stroke.or(stroke)`), extraído para
/// reuso pela fusão de segmentos de borda (`generate_row_border_segments`/
/// `update_vline_segments`, abaixo).
pub(super) fn cell_effective_stroke(
    cell: &Content,
    grid_stroke: Option<&Stroke>,
) -> Option<Stroke> {
    let cell_stroke = match cell {
        Content::TableCell(e) => e.stroke.as_ref(),
        Content::GridCell(e) => e.stroke.as_ref(),
        _ => None, // neutro: N16[β] — Content não-cell retorna None para stroke (sem override de célula)
    };
    cell_stroke.or(grid_stroke).cloned()
}

/// P888 — coordenada X da posição de linha vertical `x_idx` (0..=num_cols).
pub(super) fn vline_x(
    x_idx: usize,
    col_starts: &[f64],
    resolved_widths: &[f64],
    num_cols: usize,
) -> f64 {
    if x_idx < num_cols {
        col_starts.get(x_idx).copied().unwrap_or(0.0)
    } else {
        col_starts.get(num_cols.saturating_sub(1)).copied().unwrap_or(0.0)
            + resolved_widths
                .get(num_cols.saturating_sub(1))
                .copied()
                .unwrap_or(0.0)
    }
}

/// P888 — emite um segmento horizontal (borda de topo/fundo de uma
/// célula, ou run fundido de várias) se tiver comprimento > 0.
pub(super) fn emit_hsegment(
    items: &mut Vec<FrameItem>,
    x0: f64,
    x1: f64,
    y: f64,
    stroke: Stroke,
) {
    if x1 > x0 {
        items.push(FrameItem::Shape {
            pos: Point { x: Pt(x0), y: Pt(y) },
            kind: ShapeKind::Line { dx: x1 - x0, dy: 0.0 },
            width: (x1 - x0).abs(),
            height: 0.0,
            fill: None,
            stroke: Some(stroke),
            fill_rule: Default::default(),
            parent_bbox_at_emit: None,
        });
    }
}

/// P888 — emite um segmento vertical (borda de coluna, possivelmente
/// fundida ao longo de várias linhas) se tiver comprimento > 0.
pub(super) fn emit_vsegment(
    items: &mut Vec<FrameItem>,
    x: f64,
    y0: f64,
    y1: f64,
    stroke: Stroke,
) {
    if y1 > y0 {
        items.push(FrameItem::Shape {
            pos: Point { x: Pt(x), y: Pt(y0) },
            kind: ShapeKind::Line { dx: 0.0, dy: y1 - y0 },
            width: 0.0,
            height: (y1 - y0).abs(),
            fill: None,
            stroke: Some(stroke),
            fill_rule: Default::default(),
            parent_bbox_at_emit: None,
        });
    }
}

/// P888 — descarrega (emite) todos os segmentos verticais abertos em
/// `open`. Chamado antes de uma quebra de página (uma linha vertical não
/// pode atravessar páginas) e no fim da emissão do grid/table.
pub(super) fn flush_all_vsegments(
    open: &mut [Option<(f64, f64, Stroke)>],
    col_starts: &[f64],
    resolved_widths: &[f64],
    num_cols: usize,
    items: &mut Vec<FrameItem>,
) {
    for (x_idx, slot) in open.iter_mut().enumerate() {
        if let Some((y0, y1, stroke)) = slot.take() {
            let x = vline_x(x_idx, col_starts, resolved_widths, num_cols);
            emit_vsegment(items, x, y0, y1, stroke);
        }
    }
}

/// P888 — para a linha `row_idx`, emite as bordas de topo/fundo fundidas
/// (runs contíguos de células com o mesmo `effective_stroke`) e actualiza
/// `open_vsegments` com a continuação/quebra dos segmentos verticais.
///
/// Substitui a antiga "Opção β simplificada" (4 `FrameItem::Shape::Line`
/// por célula, sempre) por fusão: bordas horizontais fundem dentro da
/// própria linha (sem estado entre linhas — cada linha é uma unidade
/// fechada); bordas verticais fundem entre linhas via `open_vsegments`,
/// que persiste ao longo da chamada a `layout_grid` e é descarregado em
/// quebras de stroke, quebras de página, ou no fim da função.
///
/// **Segurança com `stroke` por célula divergente** (`table.cell(stroke:
/// ..)`/`grid.cell(stroke: ..)`, confirmado activo em `grid.rs` — ver
/// `typst-passo-888-relatorio.md` secção 1): quando os dois lados de uma
/// fronteira (horizontal ou vertical) têm `effective_stroke` diferentes,
/// **não funde** — em vez de tentar decidir um vencedor (como o sistema de
/// prioridade do vanilla, `lines.rs::StrokePriority`, fora de âmbito desta
/// correcção per ADR-0107), desenha os dois lados separadamente, tal como
/// o comportamento anterior a este passo — sem risco de regressão visual.
#[allow(clippy::too_many_arguments)]
pub(super) fn emit_row_borders(
    items: &mut Vec<FrameItem>,
    row_idx: usize,
    row_start_y: f64,
    row_h: f64,
    row_cells_sorted: &[usize],
    placed_cells: &[PlacedCell],
    grid_owner: &[Vec<Option<usize>>],
    open_vsegments: &mut [Option<(f64, f64, Stroke)>],
    col_starts: &[f64],
    resolved_widths: &[f64],
    num_cols: usize,
    grid_stroke: Option<&Stroke>,
) {
    // ── Bordas horizontais (topo + fundo), fundidas dentro da linha ──
    let mut top_run: Option<(f64, f64, Stroke)> = None; // (x0, x1, stroke)
    let mut bottom_run: Option<(f64, f64, Stroke)> = None;
    let can_extend_run = |same_stroke: bool, right: f64, next_left: f64| {
        same_stroke && (right - next_left).abs() < 1e-6
    };
    for &placed_idx in row_cells_sorted {
        let placed = &placed_cells[placed_idx];
        // Largura real (cobre colspan) directamente de `col_starts`/
        // `resolved_widths` — não usa `cell_bounds` aqui porque a altura
        // desta linha (`row_h`) já vem resolvida do caller (que soma
        // `row_heights[row..row+rowspan]` para linhas com rowspan); só a
        // posição/largura X é necessária para as bordas horizontais.
        let cx = col_starts.get(placed.col).copied().unwrap_or(0.0);
        let cw: f64 = (placed.col..placed.col + placed.colspan)
            .map(|i| resolved_widths.get(i).copied().unwrap_or(0.0))
            .sum();
        let stroke = cell_effective_stroke(&placed.body, grid_stroke);

        match (&top_run, &stroke) {
            (Some((rx0, rx1, rs)), Some(s)) if can_extend_run(rs == s, *rx1, cx) => {
                top_run = Some((*rx0, cx + cw, rs.clone()));
            }
            _ => {
                if let Some((rx0, rx1, rs)) = top_run.take() {
                    emit_hsegment(items, rx0, rx1, row_start_y, rs);
                }
                top_run = stroke.clone().map(|s| (cx, cx + cw, s));
            }
        }
        match (&bottom_run, &stroke) {
            (Some((rx0, rx1, rs)), Some(s)) if can_extend_run(rs == s, *rx1, cx) => {
                bottom_run = Some((*rx0, cx + cw, rs.clone()));
            }
            _ => {
                if let Some((rx0, rx1, rs)) = bottom_run.take() {
                    emit_hsegment(items, rx0, rx1, row_start_y + row_h, rs);
                }
                bottom_run = stroke.map(|s| (cx, cx + cw, s));
            }
        }
    }
    if let Some((rx0, rx1, rs)) = top_run {
        emit_hsegment(items, rx0, rx1, row_start_y, rs);
    }
    if let Some((rx0, rx1, rs)) = bottom_run {
        emit_hsegment(items, rx0, rx1, row_start_y + row_h, rs);
    }

    // ── Bordas verticais, fundidas entre linhas via `open_vsegments` ──
    let row_end_y = row_start_y + row_h;
    for x_idx in 0..=num_cols {
        let left_owner = if x_idx > 0 { grid_owner[row_idx][x_idx - 1] } else { None };
        let right_owner =
            if x_idx < num_cols { grid_owner[row_idx][x_idx] } else { None };

        // Interior de colspan (as duas posições pertencem à mesma célula):
        // não desenhar linha aqui (paridade vanilla, `lines.rs::
        // vline_stroke_at_row`, "returns None" quando cruza colspan).
        let resolved: Option<Stroke> = if left_owner.is_some()
            && left_owner == right_owner
        {
            None
        } else {
            let left_stroke = left_owner
                .and_then(|i| cell_effective_stroke(&placed_cells[i].body, grid_stroke));
            let right_stroke = right_owner
                .and_then(|i| cell_effective_stroke(&placed_cells[i].body, grid_stroke));
            match (left_stroke, right_stroke) {
                (Some(l), Some(r)) if l == r => Some(l),
                (Some(l), Some(r)) => {
                    // Strokes divergentes dos dois lados: não funde — desenha
                    // os dois directamente para esta linha (comportamento
                    // seguro, idêntico ao pré-P888 nesta posição específica).
                    let x = vline_x(x_idx, col_starts, resolved_widths, num_cols);
                    emit_vsegment(items, x, row_start_y, row_end_y, l);
                    emit_vsegment(items, x, row_start_y, row_end_y, r);
                    None
                }
                (Some(l), None) => Some(l),
                (None, Some(r)) => Some(r),
                (None, None) => None,
            }
        };

        let continues = matches!(
            (&open_vsegments[x_idx], &resolved),
            (Some((_, _, open_stroke)), Some(r)) if open_stroke == r
        );
        if continues {
            if let Some(slot) = open_vsegments[x_idx].as_mut() {
                slot.1 = row_end_y;
            }
        } else {
            if let Some((y0, y1, s)) = open_vsegments[x_idx].take() {
                let x = vline_x(x_idx, col_starts, resolved_widths, num_cols);
                emit_vsegment(items, x, y0, y1, s);
            }
            open_vsegments[x_idx] = resolved.map(|s| (row_start_y, row_end_y, s));
        }
    }
}
