//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/layout_bidi.md
//! @prompt-hash 383df31b
//! @layer L3
//! @updated 2026-07-04
//!
//! **P562/P564/P565** — Reordenação visual bidireccional de linhas e
//! reflow de blocos RTL. Passagem posterior pura sobre `PagedDocument`,
//! inserida entre layout (L1) e shaping (L3). Corrige a ordem visual de
//! palavras RTL (árabe, hebraico), recalcula as posições x com base nas
//! larguras reais e funde blocos de linhas RTL adjacentes quando o texto
//! total cabe na largura útil.

#![allow(deprecated)] // FrameItem::Text é o input legítimo desta passagem

use typst_core::entities::layout_types::{FrameItem, Page, PagedDocument, Point, Pt};
use typst_core::rules::layout::FontMetrics;
use unicode_bidi::BidiInfo;

/// Tolerância para agrupar items na mesma linha visual (baseline y).
const Y_TOLERANCE_PT: f64 = 0.01;

/// Reordena os `FrameItem::Text` dentro de cada linha visual que tenha
/// direcção base RTL, recalcula as coordenadas x a partir das larguras
/// reais devolvidas por `metrics`, e funde blocos de linhas RTL
/// adjacentes quando isso corrigir quebras de linha provocadas pelo
/// layout LTR.
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

    // Detectar quais linhas são RTL (antes de qualquer reordenação, para
    // que o reflow trabalhe com os textos originais do Layouter).
    let line_is_rtl: Vec<bool> = lines
        .iter()
        .map(|(_, line)| detect_rtl_line(&page.items, line))
        .collect();

    // Reflow primeiro: fundir blocos de linhas RTL consecutivas quando o
    // texto total cabe na largura útil da página.
    let fused_lines = reflow_rtl_blocks(page, &lines, &line_is_rtl, metrics);

    // Reordenar visualmente as linhas que não foram fundidas.
    for (i, (_, line)) in lines.iter().enumerate() {
        if line_is_rtl[i] && !fused_lines.contains(&i) {
            reorder_bidi_line(&mut page.items, line, metrics);
        }
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

/// Reordena visualmente (RTL) os `FrameItem::Text` de uma linha,
/// assumindo que a linha já foi detectada como RTL. Recalcula as
/// posições x usando as larguras reais.
fn reorder_bidi_line(
    items: &mut [FrameItem],
    line: &[usize],
    metrics: &dyn FontMetrics,
) {
    let text_indices: Vec<usize> = line
        .iter()
        .copied()
        .filter(|&idx| matches!(items[idx], FrameItem::Text { .. }))
        .collect();

    if text_indices.len() <= 1 {
        return;
    }

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

    let x_min = item_x(&items[text_indices[0]]);
    let x_max = item_x(&items[text_indices[text_indices.len() - 1]]);
    let width_before_last: f64 = widths[..widths.len() - 1].iter().sum();
    let gap = if text_indices.len() > 1 {
        (x_max - x_min - width_before_last) / (text_indices.len() - 1) as f64
    } else {
        0.0
    };

    let target_y = item_baseline_y(&items[text_indices[0]]).0;
    reorder_indices(items, &text_indices, metrics, x_min, gap, target_y);
}

/// Aplica a ordem visual RTL a uma lista de índices, recalculando x e y.
fn reorder_indices(
    items: &mut [FrameItem],
    indices: &[usize],
    metrics: &dyn FontMetrics,
    x_min: f64,
    gap: f64,
    target_y: f64,
) {
    let mut pairs: Vec<(ecow::EcoString, typst_core::entities::layout_types::TextStyle)> =
        indices
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

    let mut current_x = x_min;
    for (&idx, (text, style)) in indices.iter().zip(pairs.into_iter()) {
        if let FrameItem::Text { text: t, style: s, pos } = &mut items[idx] {
            *t = text;
            *s = style;
            pos.x = Pt(current_x);
            pos.y = Pt(target_y);
            let width = metrics.advance(t.as_str(), s.size, s).0;
            current_x += width + gap;
        }
    }
}

/// Detecta se uma linha tem direcção base RTL, mesmo que tenha apenas
/// um item de texto.
fn detect_rtl_line(items: &[FrameItem], line: &[usize]) -> bool {
    let mut buffer = String::new();
    for &idx in line {
        if let FrameItem::Text { text, .. } = &items[idx] {
            buffer.push_str(text.as_str());
        }
    }
    if buffer.is_empty() {
        return false;
    }
    let bidi = BidiInfo::new(&buffer, None);
    bidi.paragraphs
        .first()
        .map(|para| para.level.is_rtl())
        .unwrap_or(false)
}

/// Identifica e funde blocos de linhas RTL consecutivas quando o texto
/// total cabe na largura útil da página. Devolve o conjunto de índices
/// de linhas que foram fundidas (e portanto não devem ser reordenadas
/// novamente individualmente).
fn reflow_rtl_blocks(
    page: &mut Page,
    lines: &[(f64, Vec<usize>)],
    line_is_rtl: &[bool],
    metrics: &dyn FontMetrics,
) -> std::collections::HashSet<usize> {
    let mut rtl_blocks: Vec<Vec<usize>> = Vec::new();
    let mut current_block: Vec<usize> = Vec::new();

    for (i, is_rtl) in line_is_rtl.iter().enumerate() {
        if *is_rtl {
            current_block.push(i);
        } else if !current_block.is_empty() {
            rtl_blocks.push(current_block.clone());
            current_block.clear();
        }
    }
    if !current_block.is_empty() {
        rtl_blocks.push(current_block);
    }

    // Para cada bloco, decidir se funde.
    let mut fused_lines: std::collections::HashSet<usize> = std::collections::HashSet::new();
    for block in &rtl_blocks {
        if block.len() <= 1 {
            continue;
        }

        // Apenas fundir se todas as linhas do bloco contiverem apenas
        // items de texto (não movemos shapes/images entre linhas).
        let has_non_text = block.iter().any(|&line_idx| {
            lines[line_idx].1.iter().any(|&item_idx| {
                !matches!(page.items[item_idx], FrameItem::Text { .. })
            })
        });
        if has_non_text {
            continue;
        }

        // Coletar todos os items de texto do bloco, na ordem original
        // do Layouter (que é a ordem lógica do texto).
        let mut block_text_indices: Vec<usize> = Vec::new();
        for &line_idx in block {
            block_text_indices.extend(
                lines[line_idx]
                    .1
                    .iter()
                    .copied()
                    .filter(|&idx| matches!(page.items[idx], FrameItem::Text { .. })),
            );
        }

        if block_text_indices.len() <= 1 {
            continue;
        }

        let widths: Vec<f64> = block_text_indices
            .iter()
            .map(|&idx| {
                if let FrameItem::Text { text, style, .. } = &page.items[idx] {
                    metrics.advance(text.as_str(), style.size, style).0
                } else {
                    0.0
                }
            })
            .collect();

        let sum_widths: f64 = widths.iter().sum();
        let x_min = block_text_indices
            .iter()
            .map(|&idx| item_x(&page.items[idx]))
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0);
        let available_width = page.width - 2.0 * x_min;

        if sum_widths > available_width {
            // Não cabe nem mesmo sem espaçamento; manter quebra original.
            continue;
        }

        // Fundir o bloco numa única linha, usando y da primeira linha.
        let target_y = lines[block[0]].0;
        let gap = if block_text_indices.len() > 1 {
            (available_width - sum_widths) / (block_text_indices.len() - 1) as f64
        } else {
            0.0
        };

        reorder_indices(page.items.as_mut_slice(), &block_text_indices, metrics, x_min, gap, target_y);

        // Marcar todas as linhas do bloco como fundidas, para que não
        // sejam reordenadas novamente individualmente.
        for &line_idx in block {
            fused_lines.insert(line_idx);
        }
    }

    fused_lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use typst_core::entities::geometry::ShapeKind;
    use typst_core::entities::layout_types::{Color, FrameItem, Page, PagedDocument, Point, Pt, TextStyle};
    use typst_core::rules::layout::FixedMetrics;

    fn text_item(x: f64, y: f64, text: &str) -> FrameItem {
        text_item_with_size(x, y, text, Pt(12.0))
    }

    fn text_item_with_size(x: f64, y: f64, text: &str, size: Pt) -> FrameItem {
        FrameItem::Text {
            pos: Point { x: Pt(x), y: Pt(y) },
            text: text.into(),
            style: TextStyle::regular(size),
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
    fn p565_latin_no_change() {
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
    fn p565_reorder_arabic_line() {
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
    fn p565_mixed_latin_arabic() {
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
    fn p565_empty_text_unchanged() {
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
    fn p565_line_with_shape_unchanged() {
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

    #[test]
    fn p565_reflow_merges_rtl_lines_when_fits() {
        // Simular o que o Layouter LTR produziu para "الكتاب 42 على الطاولة"
        // a 40 pt, mas com margens artificiais que fazem o texto caber.
        // Larguras FixedMetrics a 40 pt:
        //   الكتاب = 144, 42 = 48, على = 72, الطاولة = 168
        // Total = 432. Com gap = 10, total = 462.
        // Página 595 x 842; margem = 66.5 → available = 595 - 133 = 462.
        // Cabe exactamente.
        let doc = PagedDocument::new(vec![page_with(vec![
            text_item_with_size(66.5, 100.0, "الكتاب", Pt(40.0)),
            text_item_with_size(220.5, 100.0, "42", Pt(40.0)),
            text_item_with_size(278.5, 100.0, "على", Pt(40.0)),
            // Layouter colocou الطاولة numa segunda linha
            text_item_with_size(66.5, 130.0, "الطاولة", Pt(40.0)),
        ])]);
        let out = reorder_bidi_document(doc, &FixedMetrics);
        let items = &out.pages[0].items;

        // Todas as palavras devem estar na mesma linha (y = 100).
        assert_eq!(items.len(), 4);
        for item in items {
            if let FrameItem::Text { pos, .. } = item {
                assert!((pos.y.0 - 100.0).abs() < 0.001);
            }
        }

        // Ordem visual: الطاولة, على, 42, الكتاب
        assert_eq!(extract_text(&items[0]), "الطاولة");
        assert_eq!(extract_text(&items[1]), "على");
        assert_eq!(extract_text(&items[2]), "42");
        assert_eq!(extract_text(&items[3]), "الكتاب");

        // Posições: começam em x_min = 66.5, gap = 10.
        assert!((item_x(&items[0]) - 66.5).abs() < 0.001);
        assert!((item_x(&items[1]) - (66.5 + 168.0 + 10.0)).abs() < 0.001);
        assert!((item_x(&items[2]) - (66.5 + 168.0 + 10.0 + 72.0 + 10.0)).abs() < 0.001);
        assert!((item_x(&items[3]) - (66.5 + 168.0 + 10.0 + 72.0 + 10.0 + 48.0 + 10.0)).abs() < 0.001);
    }

    #[test]
    fn p565_reflow_does_not_merge_when_too_wide() {
        // Mesmo texto, mas página mais estreita: não deve fundir.
        let mut page = page_with(vec![
            text_item_with_size(66.5, 100.0, "الكتاب", Pt(40.0)),
            text_item_with_size(220.5, 100.0, "42", Pt(40.0)),
            text_item_with_size(278.5, 100.0, "على", Pt(40.0)),
            text_item_with_size(66.5, 130.0, "الطاولة", Pt(40.0)),
        ]);
        page.width = 400.0; // available = 400 - 133 = 267 < 432
        let doc = PagedDocument::new(vec![page]);
        let out = reorder_bidi_document(doc, &FixedMetrics);
        let items = &out.pages[0].items;

        // Ainda deve haver duas linhas.
        let mut y_values: Vec<f64> = items
            .iter()
            .filter_map(|item| {
                if let FrameItem::Text { pos, .. } = item {
                    Some((pos.y.0 * 100.0).round() / 100.0)
                } else {
                    None
                }
            })
            .collect();
        y_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        y_values.dedup_by(|a, b| (*a - *b).abs() < 0.001);
        assert_eq!(y_values.len(), 2);
    }

    fn extract_text(item: &FrameItem) -> String {
        match item {
            FrameItem::Text { text, .. } => text.to_string(),
            _ => panic!("expected Text"),
        }
    }
}
