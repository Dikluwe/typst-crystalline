//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/layout_bidi.md
//! @prompt-hash a1bb59da
//! @layer L3
//! @updated 2026-07-04
//!
//! **P562** — Reordenação visual bidireccional de linhas.
//! Passagem posterior pura sobre `PagedDocument`, inserida entre layout
//! (L1) e shaping (L3). Corrige a ordem visual de palavras RTL (árabe,
//! hebraico) sem alterar a quebra de linha nem o shaping interno.

#![allow(deprecated)] // FrameItem::Text é o input legítimo desta passagem

use typst_core::entities::layout_types::{FrameItem, Page, PagedDocument, Pt};
use unicode_bidi::BidiInfo;

/// Tolerância para agrupar items na mesma linha visual (baseline y).
const Y_TOLERANCE_PT: f64 = 0.01;

/// Reordena os `FrameItem::Text` dentro de cada linha visual que tenha
/// direcção base RTL.
///
/// A implementação actual é uma aproximação para documentos RTL simples:
/// quando o primeiro caractere direccional da linha é RTL, inverte a
/// ordem dos textos entre as posições x existentes. Isso coloca a
/// primeira palavra lida à direita e a última à esquerda, como esperado
/// para árabe/hebraico, incluindo trechos LTR (números) no meio.
pub fn reorder_bidi_document(mut doc: PagedDocument) -> PagedDocument {
    for page in &mut doc.pages {
        reorder_bidi_page(page);
    }
    doc
}

fn reorder_bidi_page(page: &mut Page) {
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
        reorder_bidi_line(&mut page.items, line);
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
fn reorder_bidi_line(items: &mut [FrameItem], line: &[usize]) {
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

    // Caso RTL simples: inverter os textos/estilos entre as posições x
    // existentes. As posições x são preservadas, pelo que visualmente a
    // primeira palavra lida fica à direita e a última à esquerda.
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

    pairs.reverse();

    for (&idx, (text, style)) in text_indices.iter().zip(pairs.into_iter()) {
        if let FrameItem::Text { text: t, style: s, .. } = &mut items[idx] {
            *t = text;
            *s = style;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use typst_core::entities::layout_types::{FrameItem, Page, PagedDocument, Point, Pt, TextStyle};

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
    fn p562_latin_no_change() {
        let doc = PagedDocument::new(vec![page_with(vec![
            text_item(100.0, 100.0, "Hello"),
            text_item(150.0, 100.0, "world"),
        ])]);
        let out = reorder_bidi_document(doc);
        let items = &out.pages[0].items;
        assert_eq!(extract_text(&items[0]), "Hello");
        assert_eq!(extract_text(&items[1]), "world");
    }

    #[test]
    fn p562_reorder_arabic_line() {
        let doc = PagedDocument::new(vec![page_with(vec![
            text_item(100.0, 100.0, "الكتاب"),
            text_item(200.0, 100.0, "على"),
            text_item(280.0, 100.0, "الطاولة"),
        ])]);
        let out = reorder_bidi_document(doc);
        let items = &out.pages[0].items;
        // Posições x preservadas; textos invertidos.
        assert_eq!(extract_text(&items[0]), "الطاولة");
        assert_eq!(extract_text(&items[1]), "على");
        assert_eq!(extract_text(&items[2]), "الكتاب");
    }

    #[test]
    fn p562_mixed_latin_arabic() {
        let doc = PagedDocument::new(vec![page_with(vec![
            text_item(100.0, 100.0, "الكتاب"),
            text_item(200.0, 100.0, "42"),
            text_item(240.0, 100.0, "على"),
            text_item(300.0, 100.0, "الطاولة"),
        ])]);
        let out = reorder_bidi_document(doc);
        let items = &out.pages[0].items;
        assert_eq!(extract_text(&items[0]), "الطاولة");
        assert_eq!(extract_text(&items[1]), "على");
        assert_eq!(extract_text(&items[2]), "42");
        assert_eq!(extract_text(&items[3]), "الكتاب");
    }

    #[test]
    fn p562_empty_text_unchanged() {
        let doc = PagedDocument::new(vec![page_with(vec![
            text_item(100.0, 100.0, ""),
            text_item(150.0, 100.0, "x"),
        ])]);
        let out = reorder_bidi_document(doc);
        let items = &out.pages[0].items;
        assert_eq!(extract_text(&items[0]), "");
        assert_eq!(extract_text(&items[1]), "x");
    }

    #[test]
    fn p562_line_with_shape_unchanged() {
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
        let out = reorder_bidi_document(doc);
        let items = &out.pages[0].items;
        assert_eq!(extract_text(&items[0]), "الطاولة");
        assert!(matches!(items[1], FrameItem::Shape { .. }));
        assert_eq!(extract_text(&items[2]), "الكتاب");
    }

    fn extract_text(item: &FrameItem) -> String {
        match item {
            FrameItem::Text { text, .. } => text.to_string(),
            _ => panic!("expected Text"),
        }
    }
}
