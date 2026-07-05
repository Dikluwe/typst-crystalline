//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/layout_bidi.md
//! @prompt-hash f958068a
//! @layer L3
//! @updated 2026-07-04
//!
//! **P562/P564** — Reordenação visual bidireccional de linhas.
//! Passagem posterior pura sobre `PagedDocument`, inserida entre layout
//! (L1) e shaping (L3). Corrige a ordem visual de palavras RTL (árabe,
//! hebraico) e recalcula as posições x com base nas larguras reais
//! fornecidas por `FontMetrics`.

#![allow(deprecated)] // FrameItem::Text é o input legítimo desta passagem

use typst_core::entities::layout_types::{FrameItem, Page, PagedDocument, Pt};
use typst_core::rules::layout::FontMetrics;
use unicode_bidi::BidiInfo;

/// Tolerância para agrupar items na mesma linha visual (baseline y).
const Y_TOLERANCE_PT: f64 = 0.01;

/// Reordena os `FrameItem::Text` dentro de cada linha visual que tenha
/// direcção base RTL, recalculando as coordenadas x a partir das
/// larguras reais devolvidas por `metrics`.
pub fn reorder_bidi_document(
    mut doc: PagedDocument,
    metrics: &dyn FontMetrics,
) -> PagedDocument {
    for page in &mut doc.pages {
        reorder_bidi_page(page, metrics);
    }
    doc
}

fn reorder_bidi_page(page: &mut Page, metrics: &dyn FontMetrics) {
    if page.items.is_empty() {
        return;
    }

    // Agrupar items por linha visual (baseline y dentro de tolerância).
    // Usa clustering em vez de agrupamento sequencial, para que items
    // não-texto (ex.: rectângulo de highlight) com y ligeiramente
    // diferente não separem textos da mesma linha.
    let mut lines: Vec<(f64, Vec<usize>)> = Vec::new();

    for i in 0..page.items.len() {
        let y = item_baseline_y(&page.items[i]).0;
        if let Some((_, indices)) = lines.iter_mut().find(|(line_y, _)| {
            (y - line_y).abs() <= Y_TOLERANCE_PT
        }) {
            indices.push(i);
        } else {
            lines.push((y, vec![i]));
        }
    }

    for (_, line) in &lines {
        reorder_bidi_line(&mut page.items, line, metrics);
    }
}

/// Devolve a baseline y de um item para efeitos de agrupamento por linha.
fn item_baseline_y(item: &FrameItem) -> Pt {
    match item {
        FrameItem::Text { pos, .. } => pos.y,
        FrameItem::TextShaped { pos, .. } => pos.y,
        FrameItem::Line { start, .. } => start.y,
        FrameItem::Glyph { pos, .. } => pos.y,
        FrameItem::Image { pos, .. } => pos.y,
        FrameItem::Shape { pos, .. } => pos.y,
        FrameItem::Group { pos, .. } => pos.y,
        FrameItem::Link { pos, .. } => pos.y,
    }
}

/// Reordena uma única linha visual se o seu texto tiver direcção base RTL.
fn reorder_bidi_line(
    items: &mut [FrameItem],
    line: &[usize],
    metrics: &dyn FontMetrics,
) {
    // Índices dos items de texto desta linha, na ordem imposta pelo Layouter.
    let text_indices: Vec<usize> = line
        .iter()
        .copied()
        .filter(|&idx| matches!(items[idx], FrameItem::Text { .. }))
        .collect();

    if text_indices.len() <= 1 {
        return;
    }

    // Concatenar textos (sem espaços — os espaços são gaps de layout,
    // não fazem parte dos items).
    let mut buffer = String::new();
    for &idx in &text_indices {
        if let FrameItem::Text { text, .. } = &items[idx] {
            buffer.push_str(text.as_str());
        }
    }

    if buffer.is_empty() {
        return;
    }

    // Determinar a direcção base do parágrafo.
    let bidi = BidiInfo::new(&buffer, None);
    let base_rtl = bidi
        .paragraphs
        .first()
        .map(|para| para.level.is_rtl())
        .unwrap_or(false);

    if !base_rtl {
        return;
    }

    // Caso RTL simples: inverter a ordem visual dos textos/estilos e
    // recalcular as posições x usando as larguras reais.
    let mut pairs: Vec<(ecow::EcoString, typst_core::entities::layout_types::TextStyle)> =
        text_indices
            .iter()
            .filter_map(|&idx| {
                if let FrameItem::Text { text, style, .. } = &items[idx] {
                    Some((text.clone(), style.clone()))
                } else {
                    None
                }
            })
            .collect();

    // Larguras reais na ordem original (LTR imposta pelo Layouter).
    let widths: Vec<f64> = text_indices
        .iter()
        .map(|&idx| {
            if let FrameItem::Text { text, style, .. } = &items[idx] {
                metrics.advance(text.as_str(), style.size, style).0
            } else {
                0.0
            }
        })
        .collect();

    // Posição x do primeiro item e do último item na ordem original
    // (imposta pelo Layouter, da esquerda para a direita).
    let x_min = item_x(&items[text_indices[0]]);
    let x_max = item_x(&items[text_indices[text_indices.len() - 1]]);

    // O espaçamento entre items é inferido dos gaps originais. Como
    // `x_max` é a posição x do último item (não o seu fim), subtraímos
    // as larguras de todos os items excepto o último.
    let width_before_last: f64 = widths[..widths.len() - 1].iter().sum();
    let gap = if text_indices.len() > 1 {
        (x_max - x_min - width_before_last) / (text_indices.len() - 1) as f64
    } else {
        0.0
    };

    pairs.reverse();

    let mut current_x = x_min;
    for (&idx, (text, style)) in text_indices.iter().zip(pairs.into_iter()) {
        if let FrameItem::Text { text: t, style: s, pos } = &mut items[idx] {
            *t = text;
            *s = style;
            pos.x = Pt(current_x);
            let width = metrics.advance(t.as_str(), s.size, s).0;
            current_x += width + gap;
        }
    }
}

/// Devolve a coordenada x de um item.
fn item_x(item: &FrameItem) -> f64 {
    match item {
        FrameItem::Text { pos, .. } => pos.x.0,
        FrameItem::TextShaped { pos, .. } => pos.x.0,
        FrameItem::Line { start, .. } => start.x.0,
        FrameItem::Glyph { pos, .. } => pos.x.0,
        FrameItem::Image { pos, .. } => pos.x.0,
        FrameItem::Shape { pos, .. } => pos.x.0,
        FrameItem::Group { pos, .. } => pos.x.0,
        FrameItem::Link { pos, .. } => pos.x.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use typst_core::entities::layout_types::{FrameItem, Page, PagedDocument, Point, Pt, TextStyle};
    use typst_core::rules::layout::FixedMetrics;

    fn text_item(x: f64, y: f64, text: &str) -> FrameItem {
        FrameItem::Text {
            pos: Point { x: Pt(x), y: Pt(y) },
            text: text.into(),
            style: TextStyle::regular(Pt(12.0)),
        }
    }

    fn page_with(items: Vec<FrameItem>) -> Page {
        Page {
            width: 595.0,
            height: 842.0,
            numbering: None,
            items,
        }
    }

    #[test]
    fn p564_latin_no_change() {
        let doc = PagedDocument::new(vec![page_with(vec![
            text_item(100.0, 100.0, "Hello"),
            text_item(150.0, 100.0, "world"),
        ])]);
        let out = reorder_bidi_document(doc, &FixedMetrics);
        let items = &out.pages[0].items;
        assert_eq!(extract_text(&items[0]), "Hello");
        assert_eq!(extract_text(&items[1]), "world");
        assert!((item_x(&items[0]) - 100.0).abs() < 0.001);
        assert!((item_x(&items[1]) - 150.0).abs() < 0.001);
    }

    #[test]
    fn p564_reorder_arabic_line() {
        let doc = PagedDocument::new(vec![page_with(vec![
            text_item(100.0, 100.0, "الكتاب"),
            text_item(200.0, 100.0, "على"),
            text_item(280.0, 100.0, "الطاولة"),
        ])]);
        let out = reorder_bidi_document(doc, &FixedMetrics);
        let items = &out.pages[0].items;

        // Textos invertidos (ordem visual RTL).
        assert_eq!(extract_text(&items[0]), "الطاولة");
        assert_eq!(extract_text(&items[1]), "على");
        assert_eq!(extract_text(&items[2]), "الكتاب");

        // Larguras com FixedMetrics (size * 0.6 por codepoint):
        // الكتاب  = 6 chars → 43.2 pt
        // على     = 3 chars → 21.6 pt
        // الطاولة = 7 chars → 50.4 pt
        // width_before_last = 64.8; span = 280 - 100 = 180; gap = 57.6
        // Posições recalculadas a partir de x_min = 100:
        // الطاولة: 100.0
        // على:     100.0 + 50.4 + 57.6 = 208.0
        // الكتاب:  208.0 + 21.6 + 57.6 = 287.2
        assert!((item_x(&items[0]) - 100.0).abs() < 0.001);
        assert!((item_x(&items[1]) - 208.0).abs() < 0.001);
        assert!((item_x(&items[2]) - 287.2).abs() < 0.001);
    }

    #[test]
    fn p564_mixed_latin_arabic() {
        let doc = PagedDocument::new(vec![page_with(vec![
            text_item(100.0, 100.0, "الكتاب"),
            text_item(200.0, 100.0, "42"),
            text_item(240.0, 100.0, "على"),
            text_item(300.0, 100.0, "الطاولة"),
        ])]);
        let out = reorder_bidi_document(doc, &FixedMetrics);
        let items = &out.pages[0].items;
        assert_eq!(extract_text(&items[0]), "الطاولة");
        assert_eq!(extract_text(&items[1]), "على");
        assert_eq!(extract_text(&items[2]), "42");
        assert_eq!(extract_text(&items[3]), "الكتاب");

        // Larguras: 43.2 + 14.4 + 21.6 + 50.4 = 129.6
        // x_min = 100, x_max = 300, span = 200
        // width_before_last = 79.2; gap = (200 - 79.2) / 3 = 40.2666...
        let gap = (200.0 - 79.2) / 3.0;
        let mut expected_x = 100.0;
        assert!((item_x(&items[0]) - expected_x).abs() < 0.001);
        expected_x += 50.4 + gap;
        assert!((item_x(&items[1]) - expected_x).abs() < 0.001);
        expected_x += 21.6 + gap;
        assert!((item_x(&items[2]) - expected_x).abs() < 0.001);
        expected_x += 14.4 + gap;
        assert!((item_x(&items[3]) - expected_x).abs() < 0.001);
    }

    #[test]
    fn p564_empty_text_unchanged() {
        let doc = PagedDocument::new(vec![page_with(vec![
            text_item(100.0, 100.0, ""),
            text_item(150.0, 100.0, "x"),
        ])]);
        let out = reorder_bidi_document(doc, &FixedMetrics);
        let items = &out.pages[0].items;
        assert_eq!(extract_text(&items[0]), "");
        assert_eq!(extract_text(&items[1]), "x");
    }

    #[test]
    fn p564_line_with_shape_unchanged() {
        use typst_core::entities::geometry::ShapeKind;
        use typst_core::entities::layout_types::Color;
        let doc = PagedDocument::new(vec![page_with(vec![
            text_item(100.0, 100.0, "الكتاب"),
            FrameItem::Shape {
                pos: Point { x: Pt(180.0), y: Pt(90.0) },
                kind: ShapeKind::Rect,
                width: 10.0,
                height: 10.0,
                fill: Some(Color::rgb(0, 0, 0)),
                stroke: None,
                parent_bbox_at_emit: None,
            },
            text_item(200.0, 100.0, "الطاولة"),
        ])]);
        let out = reorder_bidi_document(doc, &FixedMetrics);
        let items = &out.pages[0].items;
        assert_eq!(extract_text(&items[0]), "الطاولة");
        assert!(matches!(items[1], FrameItem::Shape { .. }));
        assert_eq!(extract_text(&items[2]), "الكتاب");

        // O shape não participa; apenas os dois textos entram no cálculo.
        // x_min = 100, x_max = 200, width_before_last = 43.2, gap = 56.8.
        assert!((item_x(&items[0]) - 100.0).abs() < 0.001);
        assert!((item_x(&items[2]) - 207.2).abs() < 0.001);
    }

    fn extract_text(item: &FrameItem) -> String {
        match item {
            FrameItem::Text { text, .. } => text.to_string(),
            _ => panic!("expected Text"),
        }
    }
}
