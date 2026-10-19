//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/shaper/reconciliation.md
//! @prompt-hash 4c71b399
//! @layer L3
//! @updated 2026-10-03

use crate::font_metrics::FallbackFontMetrics;
use typst_core::contracts::world::World;
use typst_core::entities::dir::Dir;
use typst_core::entities::layout_types::{FrameItem, Page, PagedDocument, Pt, TextStyle};

// ── P582 — redistribuição de posições x após shaping ─────────────────────────

/// Tolerância y para agrupar `FrameItem` na mesma linha visual.
/// Maior que a do bidi (0.01pt) porque aqui não estamos a reposicionar;
/// 0.5pt cobre sub-pixel rounding sem cruzar linhas adjacentes.
const Y_TOL_SHAPED: f64 = 0.5;

/// P582 — corrige posições x de todos os itens de uma linha usando os advances
/// reais dos glyphs obtidos no shaper. O Layouter usa `FallbackFontMetrics`
/// (estimativa) para calcular o cursor_x de cada palavra; o shaper usa a fonte
/// real (rustybuzz). Quando as duas fontes diferem (ex.: bold resolve face
/// diferente), as posições ficam descasadas. Esta passagem redistribui as
/// posições dentro de cada linha acumulando a diferença (width_real - width_est)
/// de forma a manter todos os espaçamentos e layouts relativos intactos.
///
/// Chamado na pipeline logo após `shape_document`.
pub fn fix_line_positions(world: &dyn World, doc: PagedDocument) -> PagedDocument {
    let metrics = FallbackFontMetrics::new(world);
    fix_line_positions_with_metrics(doc, &metrics)
}

pub(crate) fn fix_line_positions_with_metrics(
    mut doc: PagedDocument,
    metrics: &FallbackFontMetrics,
) -> PagedDocument {
    for page in &mut doc.pages {
        fix_line_positions_page(metrics, page);
    }
    doc
}

fn get_item_x(item: &FrameItem) -> Option<f64> {
    match item {
        FrameItem::TextSeparator { pos, .. } | FrameItem::Text { pos, .. } => {
            Some(pos.x.0)
        }
        FrameItem::TextShaped { pos, .. } => Some(pos.x.0),
        FrameItem::Glyph { pos, .. } => Some(pos.x.0),
        FrameItem::Image { pos, .. } => Some(pos.x.0),
        FrameItem::Shape { pos, .. } => Some(pos.x.0),
        FrameItem::Group { pos, .. } => Some(pos.x.0),
        FrameItem::Link { pos, .. } => Some(pos.x.0),
        FrameItem::Line { start, .. } => Some(start.x.0),
        FrameItem::TextReplacement { items, .. } | FrameItem::Semantic { items, .. } => {
            items.first().and_then(get_item_x)
        }
    }
}

fn set_item_x(item: &mut FrameItem, x: f64) {
    match item {
        FrameItem::TextSeparator { pos, .. } | FrameItem::Text { pos, .. } => {
            pos.x = Pt(x)
        }
        FrameItem::TextShaped { pos, .. } => pos.x = Pt(x),
        FrameItem::Glyph { pos, .. } => pos.x = Pt(x),
        FrameItem::Image { pos, .. } => pos.x = Pt(x),
        FrameItem::Shape { pos, .. } => pos.x = Pt(x),
        FrameItem::Group { pos, .. } => pos.x = Pt(x),
        FrameItem::Link { pos, .. } => pos.x = Pt(x),
        FrameItem::Line { start, end, .. } => {
            let dx = end.x.0 - start.x.0;
            start.x = Pt(x);
            end.x = Pt(x + dx);
        }
        FrameItem::TextReplacement { items, .. } | FrameItem::Semantic { items, .. } => {
            if let Some(old) = items.first().and_then(get_item_x) {
                let dx = x - old;
                for child in items {
                    if let Some(child_x) = get_item_x(child) {
                        set_item_x(child, child_x + dx);
                    }
                }
            }
        }
    }
}

fn get_item_y(item: &FrameItem) -> Option<f64> {
    match item {
        FrameItem::TextSeparator { pos, .. } | FrameItem::Text { pos, .. } => {
            Some(pos.y.0)
        }
        FrameItem::TextShaped { pos, .. } => Some(pos.y.0),
        FrameItem::Glyph { pos, .. } => Some(pos.y.0),
        FrameItem::Image { pos, .. } => Some(pos.y.0),
        FrameItem::Shape { pos, .. } => Some(pos.y.0),
        FrameItem::Group { pos, .. } => Some(pos.y.0),
        FrameItem::Link { pos, .. } => Some(pos.y.0),
        FrameItem::Line { start, .. } => Some(start.y.0),
        FrameItem::TextReplacement { items, .. } | FrameItem::Semantic { items, .. } => {
            items.first().and_then(get_item_y)
        }
    }
}

fn estimate_width(metrics: &FallbackFontMetrics, text: &str, style: &TextStyle) -> f64 {
    use typst_core::compiler::layout::FontMetrics;
    // **P593** — delegar para `FontMetrics::text_width`, a fonte única do
    // nível palavra (shaping + tracking).
    metrics.text_width(text, style.size, style).val()
}

pub(super) fn reconcile_rtl_positions(
    xs: &[f64],
    estimated: &[f64],
    real: &[f64],
) -> Vec<f64> {
    debug_assert_eq!(xs.len(), estimated.len());
    debug_assert_eq!(xs.len(), real.len());
    if xs.is_empty() {
        return Vec::new();
    }

    let mut corrected = vec![0.0; xs.len()];
    let last = xs.len() - 1;
    let right_edge = xs[last] + estimated[last];
    corrected[last] = right_edge - real[last];
    for i in (0..last).rev() {
        let gap = xs[i + 1] - (xs[i] + estimated[i]);
        corrected[i] = corrected[i + 1] - gap - real[i];
    }
    corrected
}

fn estimated_and_real_width(
    metrics: &FallbackFontMetrics,
    item: &FrameItem,
) -> (f64, f64) {
    match item {
        FrameItem::TextSeparator { advance, .. } => (advance.0, advance.0),
        FrameItem::TextShaped { text, style, glyphs, units_per_em, .. } => {
            let estimated = estimate_width(metrics, text, style);
            let upem = (*units_per_em).max(1) as f64;
            let real = super::tracking::width(glyphs, style, upem);
            (estimated, real)
        }
        FrameItem::Text { text, style, .. } => {
            let width = estimate_width(metrics, text, style);
            (width, width)
        }
        FrameItem::Glyph { x_advance, .. } => (x_advance.0, x_advance.0),
        FrameItem::Image { width, .. } => (width.0, width.0),
        FrameItem::Shape { width, .. } => (*width, *width),
        FrameItem::Group { inner_width, .. } => (*inner_width, *inner_width),
        FrameItem::Link { size, .. } => (size.width.0, size.width.0),
        FrameItem::Line { start, end, .. } => {
            let width = (end.x.0 - start.x.0).abs();
            (width, width)
        }
        FrameItem::TextReplacement { items, .. } => items
            .iter()
            .map(|item| estimated_and_real_width(metrics, item))
            .fold((0.0, 0.0), |(e, r), (ce, cr)| (e + ce, r + cr)),
        FrameItem::Semantic { .. } => (0.0, 0.0),
    }
}

fn fix_line_positions_page(metrics: &FallbackFontMetrics, page: &mut Page) {
    if page.items.is_empty() {
        return;
    }

    // 1. Recolher todos os índices de itens que têm coordenada x e y,
    // agrupados por linha visual (y).
    let mut lines: Vec<Vec<usize>> = Vec::new();
    for i in 0..page.items.len() {
        let y = match get_item_y(&page.items[i]) {
            Some(y) => y,
            None => continue,
        };
        if let Some(line) = lines.iter_mut().find(|l| {
            let ly = get_item_y(&page.items[*l.first().unwrap()]).unwrap_or(0.0);
            (y - ly).abs() <= Y_TOL_SHAPED
        }) {
            line.push(i);
        } else {
            lines.push(vec![i]);
        }
    }

    // 2. Para cada linha, ordenar e aplicar os desvios cumulativos
    for line in &lines {
        if line.len() < 2 {
            continue; // sem vizinhos para acumular desvios
        }

        // Ordenar por x original crescente (esquerda→direita).
        let mut sorted = line.clone();
        sorted.sort_by(|&a, &b| {
            let xa = get_item_x(&page.items[a]).unwrap_or(0.0);
            let xb = get_item_x(&page.items[b]).unwrap_or(0.0);
            xa.partial_cmp(&xb).unwrap_or(std::cmp::Ordering::Equal)
        });

        let is_math_line = sorted.iter().any(|&idx| match &page.items[idx] {
            FrameItem::TextShaped { style, .. } => style.math,
            FrameItem::Glyph { .. } => true,
            FrameItem::TextReplacement { .. }
            | FrameItem::TextSeparator { .. }
            | FrameItem::Semantic { .. }
            | FrameItem::Text { .. }
            | FrameItem::Line { .. }
            | FrameItem::Image { .. }
            | FrameItem::Shape { .. }
            | FrameItem::Group { .. }
            | FrameItem::Link { .. } => false,
        });
        if is_math_line {
            continue;
        }

        // Verificar se a linha é RTL (se algum item tem direcção RTL)
        let certified_dir = sorted.iter().find_map(|&i| {
            crate::bidi_context::span(&page.items[i]).map(|s| s.input().base_dir())
        });
        let is_rtl_line = certified_dir.map(|d| d == Dir::RTL).unwrap_or_else(|| {
            sorted.iter().any(|&idx| match &page.items[idx] {
                FrameItem::TextShaped { style, .. } => style.dir == Some(Dir::RTL),
                FrameItem::Text { style, .. } => style.dir == Some(Dir::RTL),
                FrameItem::TextReplacement { .. }
                | FrameItem::TextSeparator { .. }
                | FrameItem::Semantic { .. }
                | FrameItem::Line { .. }
                | FrameItem::Glyph { .. }
                | FrameItem::Image { .. }
                | FrameItem::Shape { .. }
                | FrameItem::Group { .. }
                | FrameItem::Link { .. } => false, // neutro: N16[β] — itens não-textuais não possuem direcção RTL,
            })
        });

        if is_rtl_line {
            let xs: Vec<f64> = sorted
                .iter()
                .map(|idx| get_item_x(&page.items[*idx]).unwrap_or(0.0))
                .collect();
            let widths: Vec<(f64, f64)> = sorted
                .iter()
                .map(|idx| estimated_and_real_width(metrics, &page.items[*idx]))
                .collect();
            let estimated: Vec<f64> = widths.iter().map(|pair| pair.0).collect();
            let real: Vec<f64> = widths.iter().map(|pair| pair.1).collect();
            let corrected = reconcile_rtl_positions(&xs, &estimated, &real);
            for (idx, x) in sorted.iter().zip(corrected) {
                set_item_x(&mut page.items[*idx], x);
            }
        } else {
            // Linha LTR normal: ancoramos o primeiro item (mais à esquerda)
            // e acumulamos desvios para a direita (valores positivos de shift).
            let mut shift = 0.0;
            for &idx in &sorted {
                let x_orig = match get_item_x(&page.items[idx]) {
                    Some(x) => x,
                    None => continue,
                };
                set_item_x(&mut page.items[idx], x_orig + shift);

                let (w_est, w_real) = match &page.items[idx] {
                    FrameItem::TextReplacement { .. } | FrameItem::TextShaped { .. } => {
                        estimated_and_real_width(metrics, &page.items[idx])
                    }
                    FrameItem::TextSeparator { .. }
                    | FrameItem::Text { .. }
                    | FrameItem::Line { .. }
                    | FrameItem::Glyph { .. }
                    | FrameItem::Image { .. }
                    | FrameItem::Shape { .. }
                    | FrameItem::Group { .. }
                    | FrameItem::Link { .. }
                    | FrameItem::Semantic { .. } => (0.0, 0.0),
                };
                shift += w_real - w_est;
            }
        }
    }
}
