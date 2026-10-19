//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/repr/paint.md
//! @prompt-hash c5400b7d
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// Representação de uma `Color` — paridade vanilla `ProcessColor::repr`
/// (visualize/color.rs:1918). sRGB imprime-se em hex (`rgb("#rrggbb")`,
/// com sufixo alpha quando != 255); os restantes espaços imprimem-se como
/// chamada de construtor. Alpha diferente de 1.0 aparece como argumento
/// extra (excepto CMYK, que não tem alpha).
pub(super) fn repr_color(c: &Color) -> String {
    let pct = |v: f32| format_float_with_unit(v as f64 * 100.0, "%");
    let comp = |v: f32| format_float_component(v as f64);
    let hue = |h: f32| format_float_with_unit((h as f64).rem_euclid(360.0), "deg");
    match c.clone() {
        Color::Srgb { r, g, b, a } => {
            let to_u8 = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
            let (r, g, b, a) = (to_u8(r), to_u8(g), to_u8(b), to_u8(a));
            if a == 255 {
                format!("rgb(\"#{r:02x}{g:02x}{b:02x}\")")
            } else {
                format!("rgb(\"#{r:02x}{g:02x}{b:02x}{a:02x}\")")
            }
        }
        Color::Luma { l, a } => {
            if a == 1.0 {
                format!("luma({})", pct(l))
            } else {
                format!("luma({}, {})", pct(l), pct(a))
            }
        }
        Color::LinearRgb { r, g, b, a } => {
            if a == 1.0 {
                format!("color.linear-rgb({}, {}, {})", pct(r), pct(g), pct(b))
            } else {
                format!(
                    "color.linear-rgb({}, {}, {}, {})",
                    pct(r),
                    pct(g),
                    pct(b),
                    pct(a)
                )
            }
        }
        Color::Oklab { l, a, b, alpha } => {
            if alpha == 1.0 {
                format!("oklab({}, {}, {})", pct(l), comp(a), comp(b))
            } else {
                format!("oklab({}, {}, {}, {})", pct(l), comp(a), comp(b), pct(alpha))
            }
        }
        Color::Oklch { l, c: chroma, h, alpha } => {
            if alpha == 1.0 {
                format!("oklch({}, {}, {})", pct(l), comp(chroma), hue(h))
            } else {
                format!("oklch({}, {}, {}, {})", pct(l), comp(chroma), hue(h), pct(alpha))
            }
        }
        Color::Hsl { h, s, l, a } => {
            if a == 1.0 {
                format!("color.hsl({}, {}, {})", hue(h), pct(s), pct(l))
            } else {
                format!("color.hsl({}, {}, {}, {})", hue(h), pct(s), pct(l), pct(a))
            }
        }
        Color::Hsv { h, s, v, a } => {
            if a == 1.0 {
                format!("color.hsv({}, {}, {})", hue(h), pct(s), pct(v))
            } else {
                format!("color.hsv({}, {}, {}, {})", hue(h), pct(s), pct(v), pct(a))
            }
        }
        Color::Cmyk { c, m, y, k } => {
            format!("cmyk({}, {}, {}, {})", pct(c), pct(m), pct(y), pct(k))
        }
        Color::Spot(spot) => {
            let name = match &spot.colorant.name {
                Some(n) => format!("\"{}\"", n.as_str().escape_debug()),
                None => "none".to_string(),
            };
            format!(
                "color.spot({name}, {}).tint({})",
                repr_color(&spot.colorant.fallback),
                format_float_with_unit(spot.tint.get() * 100.0, "%")
            )
        }
    }
}

/// Representação de um `Stroke` — paridade vanilla `Stroke::repr`
/// (`visualize/stroke.rs:306-320`). `StrokeFields` preserva os slots
/// automáticos mesmo depois de os valores resolvidos terem sido preenchidos.
pub(in crate::compiler::eval) fn repr_stroke(s: &Stroke) -> String {
    use crate::entities::geometry::{DashLength, LineCap, LineJoin};
    let complex = s.specified.cap
        || s.specified.join
        || s.specified.dash
        || s.specified.miter_limit;
    if !complex {
        if !s.specified.paint
            && !s.specified.thickness
            && s.paint == Stroke::default().paint
            && s.thickness == 1.0
        {
            return "stroke()".to_string();
        }
        return match (s.specified.paint, s.specified.thickness) {
            (true, false) => repr_paint(&s.paint),
            (false, true) => format_float_with_unit(s.thickness, "pt"),
            _ => format!(
                "{} + {}",
                format_float_with_unit(s.thickness, "pt"),
                repr_paint(&s.paint)
            ),
        };
    }
    let mut fields = Vec::new();
    if s.specified.paint {
        fields.push(format!("paint: {}", repr_paint(&s.paint)));
    }
    if s.specified.thickness {
        fields.push(format!("thickness: {}", format_float_with_unit(s.thickness, "pt")));
    }
    if s.specified.cap {
        fields.push(format!(
            "cap: {:?}",
            match s.cap {
                LineCap::Butt => "butt",
                LineCap::Round => "round",
                LineCap::Square => "square",
            }
        ));
    }
    if s.specified.join {
        fields.push(format!(
            "join: {:?}",
            match s.join {
                LineJoin::Miter => "miter",
                LineJoin::Round => "round",
                LineJoin::Bevel => "bevel",
            }
        ));
    }
    if s.specified.dash {
        if let Some(dash) = &s.dash {
            let array = dash
                .array
                .iter()
                .map(|value| match value {
                    DashLength::Length(value) => format_float_with_unit(*value, "pt"),
                    DashLength::LineWidth => "\"dot\"".to_string(),
                })
                .collect::<Vec<_>>()
                .join(", ");
            fields.push(format!(
                "dash: (array: ({array}), phase: {})",
                format_float_with_unit(dash.phase, "pt")
            ));
        } else {
            fields.push("dash: none".to_string());
        }
    }
    if s.specified.miter_limit {
        let value = if s.miter_limit.fract() == 0.0 {
            format!("{:.1}", s.miter_limit)
        } else {
            format_float_component(s.miter_limit)
        };
        fields.push(format!("miter-limit: {value}"));
    }
    format!("({})", fields.join(", "))
}

/// Representação de um gradient linear com stops já convertidas para o espaço
/// de interpolação e offsets automáticos resolvidos.
pub(super) fn repr_gradient(gradient: &Gradient) -> String {
    match gradient {
        Gradient::Linear(linear) => {
            let offsets = linear.effective_offsets();
            let stops = linear
                .stops
                .iter()
                .zip(offsets)
                .map(|(stop, offset)| {
                    format!(
                        "({}, {})",
                        repr_color(
                            &stop
                                .color
                                .clone()
                                .to_space(linear.space.clone())
                                .unwrap_or_else(|_| stop.color.to_process())
                        ),
                        repr_ratio(Ratio(offset as f64))
                    )
                })
                .collect::<Vec<_>>()
                .join(", ");
            format!("gradient.linear({stops})")
        }
        Gradient::Radial(_) | Gradient::Conic(_) => "gradient(...)".to_string(),
    }
}

/// Representação pública mínima de tiling: a dimensão explícita é observável;
/// o corpo declarativo permanece representado pelo spread morfológico.
pub(super) fn repr_tiling(tiling: &Tiling) -> String {
    match tiling.size {
        Some(size) => format!(
            "tiling(({}, {}), ..)",
            format_float_with_unit(size.width.val(), "pt"),
            format_float_with_unit(size.height.val(), "pt")
        ),
        None => "tiling(...)".to_string(),
    }
}

/// Representação de um `Paint` delegada às mesmas formas públicas de `Value`.
pub(super) fn repr_paint(p: &Paint) -> String {
    match p {
        Paint::Solid(c) => repr_color(c),
        Paint::Gradient(gradient) => repr_gradient(gradient),
        Paint::Tiling(tiling) => repr_tiling(tiling),
    }
}

/// Representação de um `Align2D` — paridade vanilla `Alignment::repr`
/// (layout/align.rs:250): componente horizontal primeiro (`"{h} + {v}"`).
pub(super) fn repr_align(a: &Align2D) -> String {
    let h = a.h.map(|h| match h {
        HAlign::Start => "start",
        HAlign::Left => "left",
        HAlign::Center => "center",
        HAlign::Right => "right",
        HAlign::End => "end",
    });
    let v = a.v.map(|v| match v {
        VAlign::Top => "top",
        VAlign::Horizon => "horizon",
        VAlign::Bottom => "bottom",
    });
    match (h, v) {
        (Some(h), Some(v)) => format!("{h} + {v}"),
        (Some(h), None) => h.to_string(),
        (None, Some(v)) => v.to_string(),
        // `Align2D { None, None }` comporta-se como `left + top`
        // (entities/layout_types.rs:359-361).
        (None, None) => "left + top".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn srgb_color_uses_hex_morphology() {
        assert_eq!(
            repr_color(&Color::Srgb { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }),
            "rgb(\"#ff0000\")"
        );
    }
}
