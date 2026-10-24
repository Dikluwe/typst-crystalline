//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/decorations.md
//! @prompt-hash a7ec1279
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P378): o layout das decorações de texto
//! (`Underline`/`Strike`/`Overline`) movido do monólito `layout_content` para
//! o arquivo da feature (forma B). Primeiro arm AGRUPADO atomizado — a free
//! function recebe `&Content` e re-match interno. Content-preserving.

use crate::entities::content::Content;
use crate::entities::layout_types::{FrameItem, Point, Pt};

use super::{DecoSegment, FontMetrics, ImageSizer, Layouter};

/// Layout de uma decoração de texto (`underline`/`strike`/`overline`):
/// colecciona os segmentos de linha do body e emite um `FrameItem::Line` por
/// segmento, ao offset vertical próprio de cada tipo. `content` é o nó
/// agrupado (`Underline`/`Strike`/`Overline`).
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    content: &Content,
) {
    // Modelo D (Lote 4 P319): destructure de Arc<Elem> + kind_em
    // num só match (os 3 são tipos `Arc` distintos — sem `|`).
    let (body, stroke, offset, extent, kind_em, edge) = match content {
        Content::Underline(e) => (
            &e.body,
            e.stroke.clone(),
            e.offset,
            e.extent,
            0.10_f64,
            crate::compiler::introspect::page_running::PlanEdge::UnderlineBody,
        ),
        Content::Strike(e) => (
            &e.body,
            e.stroke.clone(),
            e.offset,
            e.extent,
            -0.25,
            crate::compiler::introspect::page_running::PlanEdge::StrikeBody,
        ),
        Content::Overline(e) => (
            &e.body,
            e.stroke.clone(),
            e.offset,
            e.extent,
            -0.80,
            crate::compiler::introspect::page_running::PlanEdge::OverlineBody,
        ),
        _ => unreachable!("arm gates Underline/Strike/Overline"),
    };
    let font_pt = layouter.style.size.val();
    let offset_pt = offset.map(|l| l.resolve_pt(font_pt)).unwrap_or(kind_em * font_pt);
    let extent_pt = extent.map_or(0.0, |l| l.resolve_pt(font_pt));
    let thickness = (font_pt * 0.05).max(0.4);
    // P285 §A.3: utilizador explícito > herança do texto > default.
    let color = stroke.clone().or(layouter.style.fill.clone());

    // P286 — snapshot inicial + activa collector.
    let initial_line_len = layouter.regions.current.current_line.len();
    let initial_committed_len = layouter.regions.current.current_items.len();
    let material_prefix =
        super::html_frame::items_have_material(&layouter.regions.current.current_line);
    let cursor_x_initial = layouter.regions.current.cursor_x;
    let start_x_initial = super::paragraph_indent::cursor_start(layouter);
    let baseline_y_initial = layouter.regions.current.cursor_y;
    let initial_baseline_pending = layouter.initial_baseline_pending;
    let prev_collector = layouter.decoration_lines_collector.take();
    layouter.decoration_lines_collector = Some(Vec::new());

    layouter.layout_planned_child(edge, body);

    let mut segments = layouter.decoration_lines_collector.take().unwrap_or_default();
    // Restaurar collector outer (suporta decorações aninhadas
    // hipotéticas; LIFO save/restore standard).
    layouter.decoration_lines_collector = prev_collector;

    // A settled no-flush slice has exact original append boundaries. Its
    // actual local text runs, rather than a provisional outer cursor, own
    // underline position and width. No source is replayed or reconstructed.
    let no_outer_flush = layouter.regions.current.current_line.len() >= initial_line_len
        && (initial_line_len == 0 || material_prefix)
        && layouter.regions.current.current_items[initial_committed_len..]
            .iter()
            .all(super::html_frame::is_context_marker)
        && layouter.regions.current.pending_fr.is_empty()
        && layouter.pending_repeats.is_empty();
    if matches!(content, Content::Underline(_))
        && no_outer_flush
        && inline_runs_supported(
            &layouter.regions.current.current_line[initial_line_len..],
        )
        && has_inline_text(&layouter.regions.current.current_line[initial_line_len..])
    {
        let paint = underline_runs(
            &mut layouter.regions.current.current_line[initial_line_len..],
            &layouter.metrics,
            &stroke,
            offset,
            extent,
        );
        for item in paint {
            let line = super::repeat::piece_line(layouter);
            super::repeat::register_piece_paint(layouter, line);
            layouter.regions.current.current_line.push(item);
        }
        return;
    }

    let uniform = if matches!(content, Content::Underline(_)) && !material_prefix {
        uniform_multiline_text(
            layouter.regions.current.current_items[initial_committed_len..]
                .iter()
                .chain(layouter.regions.current.current_line.iter()),
        )
    } else {
        None
    };
    let settled_initial_baseline = initial_baseline_pending && uniform.is_some();
    let (offset_pt, extent_pt, thickness, color) = if let Some((style, text)) = uniform {
        let (font_offset, thickness) =
            layouter.metrics.underline_metrics(&text, style.size, &style);
        (
            offset.map_or(font_offset.0, |value| value.resolve_pt(style.size.0)),
            extent.map_or(0.0, |value| value.resolve_pt(style.size.0)),
            thickness.0,
            stroke.or(style.fill),
        )
    } else {
        (offset_pt, extent_pt, thickness, color)
    };

    // P286 — patch do primeiro segment: o `start_x` real é
    // o snapshot inicial (não line_start_x), porque o body
    // pode começar a meio de uma linha já em curso.
    if let Some(first) = segments.first_mut() {
        first.start_x = start_x_initial;
        if !settled_initial_baseline {
            first.baseline_y = baseline_y_initial;
        }
    }
    // P286 — acrescentar segment "final não-flushed" (a linha
    // onde o body terminou sem causar wrap final).
    let final_end_x = layouter.regions.current.cursor_x;
    let final_baseline_y = layouter.regions.current.cursor_y;
    let final_start_x = if segments.is_empty() {
        start_x_initial
    } else {
        Pt(super::paragraph_indent::bounds(layouter).0)
    };
    let final_admitted = start_x_initial == cursor_x_initial
        || !segments.is_empty()
        || super::paragraph_indent::origin_applied(layouter);
    if final_admitted && final_end_x.val() > final_start_x.val() {
        segments.push(DecoSegment {
            start_x: final_start_x,
            end_x: final_end_x,
            baseline_y: final_baseline_y,
            piece_line: super::repeat::piece_line(layouter),
        });
    }

    // P286 — emite 1 `FrameItem::Line` por segment. Extent
    // aplicado simetricamente em cada linha (§A.3 opção α).
    for seg in &segments {
        let line_y = Pt(seg.baseline_y.val() + offset_pt);
        super::repeat::register_piece_paint(layouter, seg.piece_line);
        // Cada Line vai para `current_line` da página actual.
        // Para segments do meio (já flushed), o seu baseline_y
        // pertence a uma linha já em `current_items`; ainda
        // assim push em current_line é válido — o Layouter
        // não reordena por Y, apenas concatena no flush
        // seguinte (paridade vanilla painter pós-frame).
        layouter.regions.current.current_line.push(FrameItem::Line {
            start: Point { x: Pt(seg.start_x.val() - extent_pt), y: line_y },
            end: Point { x: Pt(seg.end_x.val() + extent_pt), y: line_y },
            thickness,
            color: color.clone(),
            cap: crate::entities::geometry::LineCap::Butt,
        });
    }
}

// Only actual flat text and its existing explicit-linebreak marker certify
// one typography for the legacy multiline collector. Nested paint is opaque.
fn uniform_multiline_text<'a>(
    items: impl Iterator<Item = &'a FrameItem>,
) -> Option<(crate::entities::layout_types::TextStyle, String)> {
    use crate::entities::layout_types::{SemanticKind, TextStyle};
    let mut actual: Option<TextStyle> = None;
    let mut sample = String::new();
    for item in items {
        if super::html_frame::is_context_marker(item) {
            continue;
        }
        let (text, style) = match item {
            FrameItem::Text { text, style, .. } => {
                if !text.is_ascii() || text.chars().any(char::is_control) {
                    return None;
                }
                (text.as_str(), style)
            }
            FrameItem::TextSeparator { style, .. } => (" ", style),
            FrameItem::Semantic {
                kind: SemanticKind::ExplicitLinebreakBoundary,
                items,
                ..
            } if items.iter().all(
                |item| matches!(item, FrameItem::Text { text, .. } if text.is_empty()),
            ) =>
            {
                continue
            }
            FrameItem::TextShaped { .. }
            | FrameItem::Glyph { .. }
            | FrameItem::Line { .. }
            | FrameItem::Shape { .. }
            | FrameItem::Image { .. }
            | FrameItem::Group { .. }
            | FrameItem::Link { .. }
            | FrameItem::Semantic { .. }
            | FrameItem::TextReplacement { .. } => return None,
        };
        if style.tracking.is_some()
            || style.math
            || style.subscript
            || style.superscript
            || !matches!(style.dir, None | Some(crate::entities::dir::Dir::LTR))
        {
            return None;
        }
        if actual.as_ref().is_some_and(|previous| previous != style) {
            return None;
        }
        actual.get_or_insert_with(|| style.clone());
        sample.push_str(text);
    }
    if sample.is_empty() {
        return None;
    }
    Some((actual?, sample))
}

// The caller proves the enclosing line did not flush. This further positive
// classification keeps already decorated, shaped or complex-script content
// on its existing collector path.
fn inline_runs_supported(items: &[FrameItem]) -> bool {
    items.iter().all(|item| {
        if super::html_frame::is_context_marker(item) {
            return true;
        }
        match item {
            FrameItem::Text { text, style, .. } => {
                text.is_ascii()
                    && !text.chars().any(char::is_control)
                    && style.tracking.is_none()
                    && !style.math
                    && !style.subscript
                    && !style.superscript
                    && matches!(style.dir, None | Some(crate::entities::dir::Dir::LTR))
            }
            FrameItem::Group { items, .. }
            | FrameItem::Link { items, .. }
            | FrameItem::Semantic { items, .. }
            | FrameItem::TextReplacement { items, .. } => inline_runs_supported(items),
            FrameItem::TextSeparator { style, .. } => style.tracking.is_none(),
            FrameItem::Shape { .. } | FrameItem::Image { .. } => false,
            FrameItem::TextShaped { .. }
            | FrameItem::Glyph { .. }
            | FrameItem::Line { .. } => false,
        }
    })
}

struct UnderlineRun {
    start: Point,
    end_x: f64,
    next_x: f64,
    style: crate::entities::layout_types::TextStyle,
    text: String,
}

fn underline_runs<M: FontMetrics>(
    items: &mut [FrameItem],
    metrics: &M,
    stroke: &Option<crate::entities::layout_types::Color>,
    offset: Option<crate::entities::layout_types::Length>,
    extent: Option<crate::entities::layout_types::Length>,
) -> Vec<FrameItem> {
    fn close<M: FontMetrics>(
        current: &mut Option<UnderlineRun>,
        out: &mut Vec<FrameItem>,
        metrics: &M,
        stroke: &Option<crate::entities::layout_types::Color>,
        offset: Option<crate::entities::layout_types::Length>,
        extent: Option<crate::entities::layout_types::Length>,
    ) {
        let Some(run) = current.take() else { return };
        if run.end_x <= run.start.x.0 {
            return;
        }
        let (font_offset, thickness) =
            metrics.underline_metrics(&run.text, run.style.size, &run.style);
        let offset =
            offset.map_or(font_offset.0, |value| value.resolve_pt(run.style.size.0));
        let extent = extent.map_or(0.0, |value| value.resolve_pt(run.style.size.0));
        let y = run.start.y + Pt(offset);
        out.push(FrameItem::Line {
            start: Point { x: Pt(run.start.x.0 - extent), y },
            end: Point { x: Pt(run.end_x + extent), y },
            thickness: thickness.0,
            color: stroke.clone().or(run.style.fill),
            cap: crate::entities::geometry::LineCap::Butt,
        });
    }
    let mut out = Vec::new();
    let mut current: Option<UnderlineRun> = None;
    for item in items.iter_mut() {
        if super::html_frame::is_context_marker(item) {
            continue;
        }
        match item {
            FrameItem::Text { pos, text, style, .. } => {
                let end = pos.x.0 + super::text_measure::word(metrics, text, style).0;
                if let Some(run) = current.as_mut().filter(|run| {
                    run.style == *style && run.start.y == pos.y && run.next_x == pos.x.0
                }) {
                    run.text.push_str(text);
                    run.end_x = end;
                    run.next_x = end;
                } else {
                    close(&mut current, &mut out, metrics, stroke, offset, extent);
                    current = Some(UnderlineRun {
                        start: *pos,
                        end_x: end,
                        next_x: end,
                        style: style.clone(),
                        text: text.to_string(),
                    });
                }
            }
            FrameItem::TextSeparator { pos, advance, style, origin, .. } => {
                if let Some(run) = current.as_mut().filter(|run| {
                    run.style == *style && run.start.y == pos.y && run.next_x == pos.x.0
                }) {
                    run.next_x += advance.0;
                    run.text.push(' ');
                    if *origin
                        == crate::entities::layout_types::TextSeparatorOrigin::Literal
                    {
                        run.end_x = run.next_x;
                    }
                } else {
                    close(&mut current, &mut out, metrics, stroke, offset, extent);
                }
            }
            FrameItem::Group { items, .. }
            | FrameItem::Link { items, .. }
            | FrameItem::Semantic { items, .. }
            | FrameItem::TextReplacement { items, .. } => {
                close(&mut current, &mut out, metrics, stroke, offset, extent);
                let nested = underline_runs(items, metrics, stroke, offset, extent);
                // The existing carrier owns the complete local frame and its
                // transform; added paint cannot escape that frame's geometry.
                items.extend(nested);
            }
            FrameItem::TextShaped { .. }
            | FrameItem::Glyph { .. }
            | FrameItem::Line { .. }
            | FrameItem::Shape { .. }
            | FrameItem::Image { .. } => {
                close(&mut current, &mut out, metrics, stroke, offset, extent);
            }
        }
    }
    close(&mut current, &mut out, metrics, stroke, offset, extent);
    out
}

fn has_inline_text(items: &[FrameItem]) -> bool {
    items.iter().any(|item| match item {
        FrameItem::Text { text, .. } => !text.is_empty(),
        FrameItem::Group { items, .. }
        | FrameItem::Link { items, .. }
        | FrameItem::Semantic { items, .. }
        | FrameItem::TextReplacement { items, .. } => has_inline_text(items),
        FrameItem::TextShaped { .. }
        | FrameItem::Glyph { .. }
        | FrameItem::Line { .. }
        | FrameItem::Shape { .. }
        | FrameItem::Image { .. }
        | FrameItem::TextSeparator { .. } => false,
    })
}

#[cfg(test)]
mod remaining_underline_tests {
    use super::*;
    use crate::entities::{
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{Color, Length, PageMarginSpec, PageMargins, TextStyle},
    };
    use comemo::Track;

    struct RunMetrics;
    impl FontMetrics for RunMetrics {
        fn advance(&self, text: &str, size: Pt, _: &TextStyle) -> Pt {
            Pt(text.chars().count() as f64 * size.0 * 0.6)
        }
        fn vertical_metrics(&self, size: Pt, _: &TextStyle) -> (Pt, Pt) {
            (size * 0.7, size)
        }
        fn cap_height(&self, size: Pt, _: &TextStyle) -> Pt {
            size * 0.7
        }
        fn text_edges(&self, size: Pt, _: &TextStyle) -> (Pt, Pt) {
            (size * 0.7, Pt(0.0))
        }
        fn underline_metrics(&self, _: &str, size: Pt, _: &TextStyle) -> (Pt, Pt) {
            (size * 0.125, size * 0.03125)
        }
    }
    fn render(
        offset: Option<Length>,
        extent: Option<Length>,
        color: Option<Color>,
    ) -> Vec<FrameItem> {
        render_body(Content::text("LEFT RIGHT"), offset, extent, color)
    }
    fn render_body(
        body: Content,
        offset: Option<Length>,
        extent: Option<Length>,
        color: Option<Color>,
    ) -> Vec<FrameItem> {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            RunMetrics,
            NullImageSizer,
            12.0,
            (&intr as &dyn Introspector).track(),
        );
        l.chain = l.chain.push_styles(&crate::entities::style::Styles::from_iter([
            crate::entities::style::Style::Size(Length::pt(12.0)),
        ]));
        l.style = TextStyle::from(&l.chain);
        l.page_config.width = 160.0;
        l.page_config.height = f64::INFINITY;
        l.page_config.margin = PageMargins::uniform(0.0);
        l.page_config.margin_spec = PageMarginSpec::uniform(0.0);
        l.regions.current.width = 160.0;
        l.regions.current.height = f64::INFINITY;
        l.regions.current.cursor_x = Pt(0.0);
        l.regions.current.line_start_x = Pt(0.0);
        l.regions.current.cursor_y = Pt(0.0);
        l.layout_content(&Content::underline(body, color, offset, extent));
        l.finish().pages.remove(0).items
    }
    fn lines(items: &[FrameItem], out: &mut Vec<(Point, Point, f64, Option<Color>)>) {
        for item in items {
            match item {
                FrameItem::Line { start, end, thickness, color, .. } => {
                    out.push((*start, *end, *thickness, color.clone()))
                }
                FrameItem::Group { items, .. }
                | FrameItem::Link { items, .. }
                | FrameItem::Semantic { items, .. }
                | FrameItem::TextReplacement { items, .. } => lines(items, out),
                FrameItem::Text { .. }
                | FrameItem::TextShaped { .. }
                | FrameItem::TextSeparator { .. }
                | FrameItem::Glyph { .. }
                | FrameItem::Shape { .. }
                | FrameItem::Image { .. } => {}
            }
        }
    }
    #[test]
    fn remaining_underline_uses_injected_run_metrics_and_one_real_separator_segment() {
        let mut found = Vec::new();
        lines(&render(None, None, None), &mut found);
        assert_eq!(found.len(), 1, "represented word separator joins this real text run");
        let (start, end, thickness, color) = &found[0];
        assert!((start.x.0 - 0.0).abs() < 1e-8);
        assert!((end.x.0 - 10.0 * 12.0 * 0.6).abs() < 1e-8);
        assert!((start.y.0 - (12.0 * 0.7 + 12.0 * 0.125)).abs() < 1e-8);
        assert!((*thickness - 12.0 * 0.03125).abs() < 1e-8);
        assert_eq!(*color, None);
    }
    #[test]
    fn remaining_underline_explicit_signed_offset_extent_and_color_keep_priority() {
        let red = Color::rgb(255, 0, 0);
        let mut found = Vec::new();
        lines(
            &render(Some(Length::pt(-2.0)), Some(Length::pt(3.0)), Some(red.clone())),
            &mut found,
        );
        assert_eq!(found.len(), 1);
        let (start, end, thickness, color) = &found[0];
        assert!((start.x.0 + 3.0).abs() < 1e-8);
        assert!((end.x.0 - (10.0 * 12.0 * 0.6 + 3.0)).abs() < 1e-8);
        assert!((start.y.0 - (12.0 * 0.7 - 2.0)).abs() < 1e-8);
        assert!((*thickness - 12.0 * 0.03125).abs() < 1e-8);
        assert_eq!(*color, Some(red));
    }
    #[test]
    fn remaining_underline_uniform_multiline_uses_actual_settled_baselines_and_metrics() {
        let items = render_body(
            Content::sequence(vec![
                Content::text("WIDE"),
                Content::Linebreak(std::sync::Arc::new(
                    crate::entities::elements::linebreak::LinebreakElem {
                        justify: false,
                        justify_explicit: false,
                    },
                )),
                Content::text("X"),
            ]),
            None,
            None,
            None,
        );
        let mut found = Vec::new();
        lines(&items, &mut found);
        assert_eq!(found.len(), 2);
        for (index, marker) in ["WIDE", "X"].into_iter().enumerate() {
            let baseline = items
                .iter()
                .find_map(|item| match item {
                    FrameItem::Text { text, pos, .. } if text == marker => Some(pos.y.0),
                    FrameItem::Text { .. }
                    | FrameItem::TextShaped { .. }
                    | FrameItem::TextSeparator { .. }
                    | FrameItem::Glyph { .. }
                    | FrameItem::Line { .. }
                    | FrameItem::Shape { .. }
                    | FrameItem::Image { .. }
                    | FrameItem::Group { .. }
                    | FrameItem::Link { .. }
                    | FrameItem::Semantic { .. }
                    | FrameItem::TextReplacement { .. } => None,
                })
                .expect("actual source text remains once");
            let (start, end, thickness, _) = &found[index];
            assert!((start.y.0 - baseline - 12.0 * 0.125).abs() < 1e-8);
            assert!((*thickness - 12.0 * 0.03125).abs() < 1e-8);
            assert!(
                (end.x.0 - start.x.0 - marker.len() as f64 * 12.0 * 0.6).abs() < 1e-8
            );
        }
    }
}
