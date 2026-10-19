//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/stream/shape.md
//! @prompt-hash de741ae5
//! @layer L3
//! @updated 2026-09-26

use super::{pdf_defaults, spot_bbox_for_shape, PageContext};
use crate::export::gradients::dedup_key_for_shape;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use typst_core::entities::geometry::LineCap;

/// A resource name shared by collection and operators, preserving f32 alpha.
pub(in crate::export) fn opacity_name(fill: f32, stroke: f32) -> String {
    format!("A{:08x}{:08x}", fill.to_bits(), stroke.to_bits())
}

pub(in crate::export) fn opacity_operator(fill: f32, stroke: f32) -> String {
    if fill == 1.0 && stroke == 1.0 {
        return String::new();
    }
    format!("/{} gs\n", opacity_name(fill, stroke))
}

pub(in crate::export) fn solid_opacity(
    item: &typst_core::entities::layout_types::FrameItem,
) -> (f32, f32) {
    use typst_core::entities::{frame_paint::FramePaint, layout_types::FrameItem};
    let color_alpha = |c: &typst_core::entities::color::Color| c.to_rgba_f32().3;
    let paint_alpha = |p: &FramePaint| match p {
        FramePaint::Solid(c) => color_alpha(c),
        FramePaint::Gradient(_) | FramePaint::Tiling(_) => 1.0,
    };
    match item {
        FrameItem::Shape { fill, stroke, .. } => (
            fill.as_ref().map(paint_alpha).unwrap_or(1.0),
            stroke.as_ref().map(|s| paint_alpha(&s.paint)).unwrap_or(1.0),
        ),
        FrameItem::Text { style, .. }
        | FrameItem::TextSeparator { style, .. }
        | FrameItem::TextShaped { style, .. }
        | FrameItem::Glyph { style, .. } => {
            let a = style.fill.as_ref().map(color_alpha).unwrap_or(1.0);
            (a, a)
        }
        FrameItem::Line { color, .. } => {
            (1.0, color.as_ref().map(color_alpha).unwrap_or(1.0))
        }
        FrameItem::Group { .. }
        | FrameItem::Link { .. }
        | FrameItem::Semantic { .. }
        | FrameItem::TextReplacement { .. }
        | FrameItem::Image { .. } => (1.0, 1.0),
    }
}

// ── Helpers — caminho Helvetica ────────────────────────────────────────────

/// Nome interno estável por identidade completa do colorante. Dois hashes
/// independentes evitam que nomes iguais com fallbacks distintos colidam
/// na prática sem expor IDs de objeto ou ordem da varredura ao stream.
pub(in crate::export) fn spot_resource_name(
    colorant: &typst_core::entities::color::SpotColorant,
) -> String {
    let mut first = DefaultHasher::new();
    colorant.hash(&mut first);
    let mut second = DefaultHasher::new();
    0x9e3779b97f4a7c15_u64.hash(&mut second);
    colorant.hash(&mut second);
    format!("s{:016x}{:016x}", first.finish(), second.finish())
}

pub(super) fn spot_tint_pdf(spot: &typst_core::entities::color::SpotColor) -> String {
    let value = (spot.tint.get().clamp(0.0, 1.0) * 255.0).round() / 255.0;
    format!("{value:.8}")
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
}

pub(super) fn spot_fill_operator(
    color: &typst_core::entities::layout_types::Color,
) -> Option<String> {
    let typst_core::entities::layout_types::Color::Spot(spot) = color else {
        return None;
    };
    Some(format!(
        "/{} cs {} scn\n",
        spot_resource_name(&spot.colorant),
        spot_tint_pdf(spot)
    ))
}

pub(super) fn spot_stroke_operator(
    color: &typst_core::entities::layout_types::Color,
) -> Option<String> {
    let typst_core::entities::layout_types::Color::Spot(spot) = color else {
        return None;
    };
    Some(format!(
        "/{} CS {} SCN\n",
        spot_resource_name(&spot.colorant),
        spot_tint_pdf(spot)
    ))
}

/// **P281** — emit Glyph PDF dispatched por `FontScenario`.
///
/// P285 — Constrói o prefixo `r g b RG ` para emit de `FrameItem::Line`
/// com cor de stroke. `None` → string vazia (bit-exact pré-P285); `Some(c)`
/// → operador `RG` com componentes sRGB normalizados em [0.0, 1.0] (via
/// `to_rgba_f32`, paridade absoluta com `emit_stroke_paint` Solid path
/// linha 2236-2238). Formato canónico `{:.3} {:.3} {:.3} RG `, trailing
/// space inclusivo para concatenar imediatamente antes de `{:.3} w`.
pub(in crate::export) fn line_rg_prefix(
    color: &Option<typst_core::entities::layout_types::Color>,
) -> String {
    match color {
        None => String::new(),
        Some(c) => {
            if let Some(op) = spot_stroke_operator(c) {
                return op.replace('\n', " ");
            }
            let (r, g, b, _) = c.to_rgba_f32();
            format!("{:.3} {:.3} {:.3} RG ", r, g, b)
        }
    }
}

pub(super) fn line_cap_prefix(cap: LineCap) -> &'static str {
    match cap {
        // Omitir `0 J` preserva os bytes históricos dos produtores Butt.
        LineCap::Butt => "",
        LineCap::Round => "1 J ",
        LineCap::Square => "2 J ",
    }
}

pub(in crate::export) fn fill_rg_prefix(
    color: &Option<typst_core::entities::layout_types::Color>,
) -> String {
    match color {
        None => String::new(),
        Some(c) => {
            if let Some(op) = spot_fill_operator(c) {
                return op;
            }
            let (r, g, b, _) = c.to_rgba_f32();
            format!("{:.3} {:.3} {:.3} rg\n", r, g, b)
        }
    }
}

pub(super) fn paint_effective_bbox(
    paint: &typst_core::entities::frame_paint::FramePaint,
    pos: typst_core::entities::layout_types::Point,
    width: f64,
    height: f64,
    parent: Option<typst_core::entities::layout_types::Rect>,
) -> Option<typst_core::entities::layout_types::Rect> {
    if let typst_core::entities::frame_paint::FramePaint::Gradient(g) = paint {
        spot_bbox_for_shape(g, pos, width, height).or(parent)
    } else {
        parent
    }
}

/// Emite o preenchimento de um Paint sem perder a identidade spot.
pub(super) fn emit_fill_paint(
    ops: &mut String,
    item: &typst_core::entities::layout_types::FrameItem,
    paint: &typst_core::entities::frame_paint::FramePaint,
    effective_bbox: Option<typst_core::entities::layout_types::Rect>,
    ctx: &PageContext,
) {
    use typst_core::entities::frame_paint::FramePaint as Paint;
    match paint {
        Paint::Solid(color) => {
            if let Some(op) = spot_fill_operator(color) {
                ops.push_str(&op);
            } else {
                let (r, g, b, _) = color.to_rgba_f32();
                ops.push_str(&format!("{r:.3} {g:.3} {b:.3} rg\n"));
            }
        }
        Paint::Gradient(gradient) => {
            let key = dedup_key_for_shape(gradient, effective_bbox, item);
            if let Some(&idx) = ctx.pat_ptr_to_idx.get(&key) {
                ops.push_str(&format!("/Pattern cs\n/{} scn\n", ctx.pat_refs[idx].name));
            } else {
                let (r, g, b, _) =
                    typst_core::entities::paint::Paint::Gradient(gradient.clone())
                        .to_color()
                        .to_rgba_f32();
                ops.push_str(&format!("{r:.3} {g:.3} {b:.3} rg\n"));
            }
        }
        Paint::Tiling(cell) => {
            let name = ctx
                .tiling_names
                .and_then(|map| map.get(&crate::export::pdf_tiling::key(item, cell)))
                .expect("prepared tiling must be registered");
            ops.push_str(&format!("/Pattern cs\n/{name} scn\n"));
        }
    }
}

/// P263 — Emite operadores de stroke colour para um Paint (Solid ou Gradient).
///
/// Para `Paint::Solid(c)`: emit `r g b RG` literal P261 preservado.
/// Para `Paint::Gradient(g)`: emit `/Pattern CS /P{n} SCN` (set colour
/// space pattern + apply pattern).
pub(in crate::export) fn emit_stroke_paint(
    ops: &mut String,
    item: &typst_core::entities::layout_types::FrameItem,
    paint: &typst_core::entities::frame_paint::FramePaint,
    thickness: f64,
    effective_bbox: Option<typst_core::entities::layout_types::Rect>,
    ctx: &PageContext,
) {
    use typst_core::entities::frame_paint::FramePaint as Paint;
    match paint {
        Paint::Solid(c) => {
            if let Some(op) = spot_stroke_operator(c) {
                ops.push_str(&op);
                ops.push_str(&format!("{:.2} w\n", thickness));
                return;
            }
            let (r, g, b, _) = c.to_rgba_f32();
            ops.push_str(&format!("{:.3} {:.3} {:.3} RG\n{:.2} w\n", r, g, b, thickness));
        }
        Paint::Gradient(g) => {
            // P273.12 — lookup via DedupKey bbox-aware (substitui ptr-only).
            let key = dedup_key_for_shape(g, effective_bbox, item);
            if let Some(&idx) = ctx.pat_ptr_to_idx.get(&key) {
                let r = &ctx.pat_refs[idx];
                ops.push_str(&format!(
                    "/Pattern CS\n/{} SCN\n{:.2} w\n",
                    r.name, thickness
                ));
            } else {
                // Fallback paranóide — gradient não registado em scan_all_gradients.
                let c =
                    typst_core::entities::paint::Paint::Gradient(g.clone()).to_color();
                let (r, g, b, _) = c.to_rgba_f32();
                ops.push_str(&format!(
                    "{:.3} {:.3} {:.3} RG\n{:.2} w\n",
                    r, g, b, thickness
                ));
            }
        }
        Paint::Tiling(cell) => {
            let name = ctx
                .tiling_names
                .and_then(|map| map.get(&crate::export::pdf_tiling::key(item, cell)))
                .expect("prepared tiling must be registered");
            ops.push_str(&format!("/Pattern CS\n/{name} SCN\n{thickness:.2} w\n"));
        }
    }
}

/// Emite os operadores de path de uma forma no espaço LOCAL de um Group.
///
/// NÃO usa page_height. A matriz `cm` já inverteu o eixo Y.
/// Chamada para emitir clip_mask antes de `W n`.
pub(in crate::export) fn emit_shape_path_local(
    ops: &mut String,
    kind: &typst_core::entities::geometry::ShapeKind<
        typst_core::entities::layout_types::Pt,
    >,
    width: f64,
    height: f64,
) {
    use typst_core::entities::geometry::{PathItem, ShapeKind};
    match kind {
        ShapeKind::Rect => {
            ops.push_str(&format!("0.00 0.00 {:.2} {:.2} re\n", width, height));
        }
        ShapeKind::RoundedRect { radii } => {
            // The shared formatter uses Y-up corner names. Swap vertical
            // corners so the resulting path has the requested Y-down radii.
            let local = typst_core::entities::corners::Corners::new(
                radii.bottom_left,
                radii.bottom_right,
                radii.top_right,
                radii.top_left,
            );
            emit_rounded_rect_ops(ops, 0.0, 0.0, width, height, &local);
        }
        ShapeKind::Path(items) => {
            for item in items {
                match item {
                    PathItem::MoveTo(p) => {
                        ops.push_str(&format!("{:.2} {:.2} m\n", p.x.0, p.y.0,))
                    }
                    PathItem::LineTo(p) => {
                        ops.push_str(&format!("{:.2} {:.2} l\n", p.x.0, p.y.0,))
                    }
                    PathItem::CubicTo(p1, p2, p3) => ops.push_str(&format!(
                        "{:.2} {:.2} {:.2} {:.2} {:.2} {:.2} c\n",
                        p1.x.0, p1.y.0, p2.x.0, p2.y.0, p3.x.0, p3.y.0,
                    )),
                    PathItem::ClosePath => ops.push_str("h\n"),
                }
            }
        }
        ShapeKind::Ellipse => {
            let rx = width / 2.0;
            let ry = height / 2.0;
            let ox = rx * pdf_defaults::BEZIER_CIRCLE_KAPPA;
            let oy = ry * pdf_defaults::BEZIER_CIRCLE_KAPPA;
            ops.push_str(&format!("{rx:.3} {height:.3} m\n"));
            for [x1, y1, x2, y2, x3, y3] in [
                [rx + ox, height, width, ry + oy, width, ry],
                [width, ry - oy, rx + ox, 0.0, rx, 0.0],
                [rx - ox, 0.0, 0.0, ry - oy, 0.0, ry],
                [0.0, ry + oy, rx - ox, height, rx, height],
            ] {
                ops.push_str(&format!(
                    "{x1:.3} {y1:.3} {x2:.3} {y2:.3} {x3:.3} {y3:.3} c\n"
                ));
            }
            ops.push_str("h\n");
        }
        ShapeKind::Line { dx, dy } => {
            let (x0, x1) = if *dx < 0.0 { (width, 0.0) } else { (0.0, width) };
            let (y0, y1) = if *dy < 0.0 { (height, 0.0) } else { (0.0, height) };
            ops.push_str(&format!("{x0:.3} {y0:.3} m\n{x1:.3} {y1:.3} l\n"));
        }
    }
}

/// **P242 (M9d / M7+5)** — emite operadores PDF para um rectângulo com
/// cantos arredondados via Bezier 4 corners (paridade vanilla
/// `typst-pdf/.../shape.rs::draw_rounded_rect`).
///
/// Coordenadas em sistema PDF (Y crescente para cima). `(x, y)` é o
/// canto inferior-esquerdo; `w` largura; `h` altura positivos. `radii`
/// em `Corners<Pt>` (top_left/top_right/bottom_right/bottom_left
/// sentido horário começando top-left).
///
/// **Bezier kappa = 0.552_284_749_831** (paridade `ShapeKind::Ellipse`
/// neste mesmo ficheiro). Quarto de círculo aproximado com 2 control
/// points por canto.
///
/// **Output**: sequência `m` (move) + `l` (line) + `c` (cubic) + `h`
/// (closePath) — formato compatível com `B`/`S`/`W n` paint operators.
pub(in crate::export) fn emit_rounded_rect_ops(
    ops: &mut String,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    radii: &typst_core::entities::corners::Corners<
        typst_core::entities::layout_types::Pt,
    >,
) {
    // P1133 — raios já resolvidos em L1. Clamp cada raio a metade da menor dimensão (paridade vanilla evita
    // overflow geométrico).
    let max_r = (w.min(h)) / 2.0;
    let tl = radii.top_left.0.clamp(0.0, max_r);
    let tr = radii.top_right.0.clamp(0.0, max_r);
    let br = radii.bottom_right.0.clamp(0.0, max_r);
    let bl = radii.bottom_left.0.clamp(0.0, max_r);

    // Sentido horário em PDF coords (Y para cima). Sequência:
    // start top-left edge → top edge → top-right corner → right edge →
    // bottom-right corner → bottom edge → bottom-left corner → left edge →
    // top-left corner → close.
    let x_left = x;
    let x_right = x + w;
    let y_top = y + h;
    let y_bottom = y;

    // MoveTo: começa no início da edge top (após canto top-left).
    ops.push_str(&format!("{:.3} {:.3} m\n", x_left + tl, y_top));
    // Linha top edge.
    ops.push_str(&format!("{:.3} {:.3} l\n", x_right - tr, y_top));
    // Cubic top-right corner.
    if tr > 0.0 {
        ops.push_str(&format!(
            "{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
            x_right - tr + tr * pdf_defaults::BEZIER_CIRCLE_KAPPA,
            y_top,
            x_right,
            y_top - tr + tr * pdf_defaults::BEZIER_CIRCLE_KAPPA,
            x_right,
            y_top - tr
        ));
    }
    // Linha right edge.
    ops.push_str(&format!("{:.3} {:.3} l\n", x_right, y_bottom + br));
    // Cubic bottom-right corner.
    if br > 0.0 {
        ops.push_str(&format!(
            "{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
            x_right,
            y_bottom + br - br * pdf_defaults::BEZIER_CIRCLE_KAPPA,
            x_right - br + br * pdf_defaults::BEZIER_CIRCLE_KAPPA,
            y_bottom,
            x_right - br,
            y_bottom
        ));
    }
    // Linha bottom edge.
    ops.push_str(&format!("{:.3} {:.3} l\n", x_left + bl, y_bottom));
    // Cubic bottom-left corner.
    if bl > 0.0 {
        ops.push_str(&format!(
            "{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
            x_left + bl - bl * pdf_defaults::BEZIER_CIRCLE_KAPPA,
            y_bottom,
            x_left,
            y_bottom + bl - bl * pdf_defaults::BEZIER_CIRCLE_KAPPA,
            x_left,
            y_bottom + bl
        ));
    }
    // Linha left edge.
    ops.push_str(&format!("{:.3} {:.3} l\n", x_left, y_top - tl));
    // Cubic top-left corner.
    if tl > 0.0 {
        ops.push_str(&format!(
            "{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
            x_left,
            y_top - tl + tl * pdf_defaults::BEZIER_CIRCLE_KAPPA,
            x_left + tl - tl * pdf_defaults::BEZIER_CIRCLE_KAPPA,
            y_top,
            x_left + tl,
            y_top
        ));
    }
    // Fecha o path.
    ops.push_str("h\n");
}

#[cfg(test)]
mod tests {
    use super::*;
    use typst_core::entities::corners::Corners;
    use typst_core::entities::layout_types::Pt;
    #[test]
    fn p1133_pdf_rounded_rect_preserva_quatro_cantos_em_pt() {
        let mut ops = String::new();
        emit_rounded_rect_ops(
            &mut ops,
            10.0,
            20.0,
            100.0,
            80.0,
            &Corners::new(Pt(2.0), Pt(4.0), Pt(6.0), Pt(8.0)),
        );
        assert!(ops.contains("12.000 100.000 m"));
        assert!(ops.contains("106.000 100.000 l"));
        assert!(ops.contains("110.000 26.000 l"));
        assert!(ops.contains("18.000 20.000 l"));
    }
    #[test]
    fn group_clip_path_preserves_y_down_vertices_and_controls() {
        use typst_core::entities::geometry::{PathItem, ShapeKind};
        use typst_core::entities::layout_types::Point;
        let p = |x, y| Point { x: Pt(x), y: Pt(y) };
        let path = ShapeKind::Path(vec![
            PathItem::MoveTo(p(1.0, 2.0)),
            PathItem::LineTo(p(3.0, 4.0)),
            PathItem::CubicTo(p(5.0, 6.0), p(7.0, 8.0), p(9.0, 10.0)),
            PathItem::ClosePath,
        ]);
        let mut ops = String::new();
        emit_shape_path_local(&mut ops, &path, 12.0, 14.0);
        let numbers: Vec<f64> =
            ops.split_whitespace().filter_map(|x| x.parse().ok()).collect();
        assert_eq!(numbers, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]);
        assert!(ops.ends_with("h\n"));
    }

    #[test]
    fn group_clip_rounded_rect_keeps_asymmetric_physical_corners() {
        use typst_core::entities::geometry::ShapeKind;
        let mut ops = String::new();
        emit_shape_path_local(
            &mut ops,
            &ShapeKind::RoundedRect {
                radii: Corners::new(Pt(2.0), Pt(4.0), Pt(6.0), Pt(8.0)),
            },
            40.0,
            30.0,
        );
        // Tangency endpoints specify which physical corner owns each radius.
        let endpoints: Vec<(f64, f64)> = ops
            .lines()
            .filter_map(|line| {
                let tokens: Vec<_> = line.split_whitespace().collect();
                match tokens.last().copied() {
                    Some("m" | "l") => {
                        Some((tokens[0].parse().unwrap(), tokens[1].parse().unwrap()))
                    }
                    Some("c") => {
                        Some((tokens[4].parse().unwrap(), tokens[5].parse().unwrap()))
                    }
                    _ => None,
                }
            })
            .collect();
        for expected in [
            (2.0, 0.0),
            (36.0, 0.0),
            (40.0, 4.0),
            (40.0, 24.0),
            (34.0, 30.0),
            (8.0, 30.0),
            (0.0, 22.0),
            (0.0, 2.0),
        ] {
            assert!(
                endpoints.contains(&expected),
                "missing physical corner {expected:?}: {ops}"
            );
        }
    }

    #[test]
    fn group_clip_ellipse_covers_its_local_box() {
        use typst_core::entities::geometry::ShapeKind;
        let mut ops = String::new();
        emit_shape_path_local(&mut ops, &ShapeKind::Ellipse, 40.0, 30.0);
        let endpoints: Vec<(f64, f64)> = ops
            .lines()
            .filter_map(|line| {
                let t: Vec<_> = line.split_whitespace().collect();
                (t.last().copied() == Some("c"))
                    .then(|| (t[4].parse().unwrap(), t[5].parse().unwrap()))
            })
            .collect();
        assert_eq!(endpoints.len(), 4);
        for p in [(0.0, 15.0), (40.0, 15.0), (20.0, 0.0), (20.0, 30.0)] {
            assert!(endpoints.contains(&p), "missing ellipse extremum {p:?}: {ops}");
        }
        assert!(ops.ends_with("h\n"));
    }
}
