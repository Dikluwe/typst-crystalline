//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/stream/shape.md
//! @prompt-hash 74dbb008
//! @layer L3
//! @updated 2026-09-26

use super::{dedup_key_for, pdf_defaults, spot_bbox_for_shape, DedupKey, PatternRef};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use typst_core::entities::geometry::LineCap;

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
    paint: &typst_core::entities::paint::Paint,
    pos: typst_core::entities::layout_types::Point,
    width: f64,
    height: f64,
    parent: Option<typst_core::entities::layout_types::Rect>,
) -> Option<typst_core::entities::layout_types::Rect> {
    if let typst_core::entities::paint::Paint::Gradient(g) = paint {
        spot_bbox_for_shape(g, pos, width, height).or(parent)
    } else {
        parent
    }
}

/// Emite o preenchimento de um Paint sem perder a identidade spot.
pub(super) fn emit_fill_paint(
    ops: &mut String,
    paint: &typst_core::entities::paint::Paint,
    effective_bbox: Option<typst_core::entities::layout_types::Rect>,
    pat_ptr_to_idx: &HashMap<DedupKey, usize>,
    pat_refs: &[PatternRef],
) {
    use typst_core::entities::paint::Paint;
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
            let key = dedup_key_for(gradient, effective_bbox);
            if let Some(&idx) = pat_ptr_to_idx.get(&key) {
                ops.push_str(&format!("/Pattern cs\n/{} scn\n", pat_refs[idx].name));
            } else {
                let (r, g, b, _) = paint.to_color().to_rgba_f32();
                ops.push_str(&format!("{r:.3} {g:.3} {b:.3} rg\n"));
            }
        }
        Paint::Tiling(_) => {
            let (r, g, b, _) = paint.to_color().to_rgba_f32();
            ops.push_str(&format!("{r:.3} {g:.3} {b:.3} rg\n"));
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
    paint: &typst_core::entities::paint::Paint,
    thickness: f64,
    effective_bbox: Option<typst_core::entities::layout_types::Rect>,
    pat_ptr_to_idx: &HashMap<DedupKey, usize>,
    pat_refs: &[PatternRef],
) {
    use typst_core::entities::paint::Paint;
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
            let key = dedup_key_for(g, effective_bbox);
            if let Some(&idx) = pat_ptr_to_idx.get(&key) {
                let r = &pat_refs[idx];
                ops.push_str(&format!(
                    "/Pattern CS\n/{} SCN\n{:.2} w\n",
                    r.name, thickness
                ));
            } else {
                // Fallback paranóide — gradient não registado em scan_all_gradients.
                let c = paint.to_color();
                let (r, g, b, _) = c.to_rgba_f32();
                ops.push_str(&format!(
                    "{:.3} {:.3} {:.3} RG\n{:.2} w\n",
                    r, g, b, thickness
                ));
            }
        }
        Paint::Tiling(_) => {
            // P395 — pattern fill de Tiling é scope-out ADR-0054 graded.
            // Fallback para cor representativa (Color body → cor; Image → preto).
            let c = paint.to_color();
            let (r, g, b, _) = c.to_rgba_f32();
            ops.push_str(&format!("{:.3} {:.3} {:.3} RG\n{:.2} w\n", r, g, b, thickness));
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
            ops.push_str(&format!("0.00 {:.2} {:.2} {:.2} re\n", -height, width, height));
        }
        ShapeKind::RoundedRect { radii } => {
            // P242 (M9d / M7+5) — Bezier 4 corners path em espaço local
            // (origem 0,0; Y invertido pela matriz cm). Coords iguais a
            // ShapeKind::Rect: (0, -height, width, height).
            emit_rounded_rect_ops(ops, 0.0, -height, width, height, radii);
        }
        ShapeKind::Path(items) => {
            for item in items {
                match item {
                    PathItem::MoveTo(p) => {
                        ops.push_str(&format!("{:.2} {:.2} m\n", p.x.0, -p.y.0,))
                    }
                    PathItem::LineTo(p) => {
                        ops.push_str(&format!("{:.2} {:.2} l\n", p.x.0, -p.y.0,))
                    }
                    PathItem::CubicTo(p1, p2, p3) => ops.push_str(&format!(
                        "{:.2} {:.2} {:.2} {:.2} {:.2} {:.2} c\n",
                        p1.x.0, -p1.y.0, p2.x.0, -p2.y.0, p3.x.0, -p3.y.0,
                    )),
                    PathItem::ClosePath => ops.push_str("h\n"),
                }
            }
        }
        _ => {} // neutro: N16[α] — FrameItem não-Shape sem path a emitir no stream PDF (despacho modular fechado)
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
}
