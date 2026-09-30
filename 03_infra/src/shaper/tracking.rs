//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/shaper/tracking.md
//! @prompt-hash 38880fd1
//! @layer L3
//! @updated 2026-09-29

use typst_core::entities::layout_types::{ShapedGlyph, TextStyle};

/// Recover the fractional part lost by the stored integer font-unit increment.
pub(crate) fn residual(glyphs: &[ShapedGlyph], index: usize, style: &TextStyle, upem: f64) -> f64 {
    let Some(glyph) = glyphs.get(index) else { return 0.0 };
    let Some(next) = glyphs.get(index + 1) else { return 0.0 };
    if glyph.cluster == next.cluster || style.size.0 <= 0.0 || !style.size.0.is_finite() {
        return 0.0;
    }
    let t = style.tracking.map(|t| t.resolve_pt(style.size.0)).unwrap_or(0.0);
    if !t.is_finite() || t == 0.0 { return 0.0; }
    let upem = upem.max(1.0);
    let integer = (t * upem / style.size.0).round() as i32;
    t - integer as f64 / upem * style.size.0
}

pub(crate) fn width(glyphs: &[ShapedGlyph], style: &TextStyle, upem: f64) -> f64 {
    let upem = upem.max(1.0);
    glyphs.iter().enumerate().map(|(i,g)| {
        g.x_advance as f64 / upem * style.size.0 + residual(glyphs,i,style,upem)
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use typst_core::entities::layout_types::{Length, Pt};
    #[test]
    fn fractional_tracking_respects_clusters_and_signed_pairs() {
        let mut style = TextStyle::regular(Pt(12.0));
        for t in [0.2,-0.2,0.333,0.0,1.2] {
            style.tracking = Some(Length::pt(t));
            let q=(t/12.0*1000.0).round() as i32;
            let glyphs: Vec<_> = [0,0,1].into_iter().enumerate().map(|(i,cluster)| ShapedGlyph {
                glyph_id:1, x_advance:500+if i==1 {q}else{0}, x_offset:0,y_offset:0,cluster,char_code:'A',
            }).collect();
            assert_eq!(residual(&glyphs,0,&style,1000.0),0.0);
            assert_eq!(residual(&glyphs,2,&style,1000.0),0.0);
            assert!((width(&glyphs,&style,1000.0)-(18.0+t)).abs()<1e-10);
        }
    }
}
