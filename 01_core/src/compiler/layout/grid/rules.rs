//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/grid/rules.md
//! @prompt-hash b7c4c683
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// Emite as regras h/v explícitas sobre as células após paginação.
#[allow(clippy::too_many_arguments)]
pub(super) fn emit_explicit_rules<
    M: FontMetrics,
    S: ImageSizer,
    H: LayoutHLine,
    V: LayoutVLine,
>(
    hlines: &[H],
    vlines: &[V],
    num_rows_produced_final: usize,
    num_cols: usize,
    emitted_row_starts: &[f64],
    row_heights: &[f64],
    col_starts: &[f64],
    resolved_widths: &[f64],
    layouter: &mut super::super::Layouter<M, S>,
) {
    // P512 — desenhar hlines/vlines por cima das células, usando os
    // Y reais emitidos (já consideraram paginação).
    if !hlines.is_empty() || !vlines.is_empty() {
        let grid_left = col_starts.first().copied().unwrap_or(0.0);
        let row_bottom = |r: usize| {
            emitted_row_starts.get(r).copied().unwrap_or(0.0)
                + row_heights.get(r).copied().unwrap_or(0.0)
        };
        let col_right = |c: usize| {
            col_starts.get(c).copied().unwrap_or(0.0)
                + resolved_widths.get(c).copied().unwrap_or(0.0)
        };

        for h in hlines {
            // **P739A** — `stroke: none` → linha não desenhada (paridade
            // vanilla, medido; zero-thickness seria hairline em PDF).
            let Some(stroke) = h.stroke() else {
                continue;
            };
            let row = h.row().min(num_rows_produced_final.saturating_sub(1));
            let y = match h.position() {
                "bottom" => row_bottom(row),
                _ => emitted_row_starts.get(row).copied().unwrap_or(0.0),
            };
            let start = h.start().min(num_cols);
            let end = h.end().map(|e| e.min(num_cols)).unwrap_or(num_cols);
            if start >= end {
                continue;
            }
            let x0 = col_starts.get(start).copied().unwrap_or(grid_left);
            let x1 = col_right(end.saturating_sub(1));
            // P887 (achado 3 de P885, extensão) — `width` = `dx.abs()`,
            // não `0.0` (ver comentário equivalente nas bordas de
            // célula acima); sem isto o traço tem comprimento zero.
            let prepared_stroke =
                super::super::tiling::resolve_stroke(layouter, Some(stroke));
            layouter.regions.current.current_items.push(FrameItem::Shape {
                pos: Point { x: Pt(x0), y: Pt(y) },
                kind: ShapeKind::Line { dx: x1 - x0, dy: 0.0 },
                width: (x1 - x0).abs(),
                height: 0.0,
                fill: None,
                stroke: prepared_stroke,
                fill_rule: Default::default(),
                parent_bbox_at_emit: None,
            });
        }

        for v in vlines {
            let Some(stroke) = v.stroke() else { continue };
            let col = v.col().min(num_cols.saturating_sub(1));
            let x = match v.position() {
                "right" => col_right(col),
                _ => col_starts.get(col).copied().unwrap_or(grid_left),
            };
            let start = v.start().min(num_rows_produced_final);
            let end = v
                .end()
                .map(|e| e.min(num_rows_produced_final))
                .unwrap_or(num_rows_produced_final);
            if start >= end {
                continue;
            }
            let y0 = emitted_row_starts.get(start).copied().unwrap_or(0.0);
            let y1 = row_bottom(end.saturating_sub(1));
            // P887 (achado 3 de P885, extensão) — `height` = `dy.abs()`,
            // não `0.0` (mesma correcção do hline acima).
            let prepared_stroke =
                super::super::tiling::resolve_stroke(layouter, Some(stroke));
            layouter.regions.current.current_items.push(FrameItem::Shape {
                pos: Point { x: Pt(x), y: Pt(y0) },
                kind: ShapeKind::Line { dx: 0.0, dy: y1 - y0 },
                width: 0.0,
                height: (y1 - y0).abs(),
                fill: None,
                stroke: prepared_stroke,
                fill_rule: Default::default(),
                parent_bbox_at_emit: None,
            });
        }
    }
}
