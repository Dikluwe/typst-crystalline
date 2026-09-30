//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/svg/serialization.md
//! @prompt-hash 95e0c330
//! @layer L3
//! @updated 2026-09-26

use typst_core::entities::layout_types::Color;

pub(super) fn color_to_css(color: Color) -> String {
    let (r, g, b, a) = color.to_srgb();
    if a == 255 {
        format!("#{r:02x}{g:02x}{b:02x}")
    } else {
        format!("rgba({r},{g},{b},{:.3})", a as f64 / 255.0)
    }
}
pub(super) fn fmt_num(n: f64) -> String {
    const ROUNDING_FACTOR: f64 = 1_000_000_000.0;
    let n = (ROUNDING_FACTOR * n).round() / ROUNDING_FACTOR;
    if n == n as i64 as f64 {
        format!("{}", n as i64)
    } else {
        let mut buf = ryu::Buffer::new();
        buf.format(n).to_string()
    }
}

pub(super) fn fmt_pair(x: f64, y: f64) -> String {
    format!("{} {}", fmt_num(x), fmt_num(y))
}
