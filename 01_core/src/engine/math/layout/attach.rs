//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/math/layout/attach.md
//! @prompt-hash 6496e54a
//! @layer L1
//! @updated 2026-04-23
//!
//! Método `layout_attach` de `MathLayouter`. Extraído de `math/layout/mod.rs`
//! no Passo 96.8 conforme ADR-0037.

use crate::engine::layout::FontMetrics;
use crate::entities::{
    content::Content,
    layout_types::{Pt, TextStyle},
};

use super::symbols;
use super::{offset_item, MathBox};
use crate::entities::glyph_variants::MathGlyphKern;

impl<'a, M: FontMetrics> super::MathLayouter<'a, M> {
    pub(super) fn layout_attach(
        &self,
        base: &Content,
        tl: Option<&Content>,
        bl: Option<&Content>,
        sub: Option<&Content>,
        sup: Option<&Content>,
        style: &TextStyle,
    ) -> MathBox {
        let base_box = self.layout_node(base, style);
        let script_style = TextStyle {
            size: style.size * self.constants.script_percent_scale_down,
            math_script: true,
            ..style.clone()
        };

        // Layout de todos os scripts primeiro para obter dimensões e métricas
        let tl_box = tl.map(|c| self.layout_node(c, &script_style));
        let bl_box = bl.map(|c| self.layout_node(c, &script_style));
        let sup_box = sup.map(|c| self.layout_node(c, &script_style));
        let sub_box = sub.map(|c| self.layout_node(c, &script_style));

        // Extrair chars para consulta a MathGlyphKern
        let base_char: Option<char> = match base {
            Content::MathIdent(s) | Content::MathText(s) => s.chars().next(),
            _ => None,
        };
        let extract_char = |c: Option<&Content>| -> Option<char> {
            match c {
                Some(Content::MathIdent(s)) | Some(Content::MathText(s)) => s.chars().next(),
                _ => None,
            }
        };
        let tl_char = extract_char(tl);
        let bl_char = extract_char(bl);
        let sup_char = extract_char(sup);
        let sub_char = extract_char(sub);

        let base_ascent = base_box.ascent;
        let base_descent = base_box.descent;
        let base_width = base_box.width;
        let is_text_like = matches!(base, Content::MathIdent(_) | Content::MathText(_));

        // P914 — Deslocamentos adaptativos de sub/sobrescrito (compute_script_shifts)
        let (sup_offset, sub_offset) = self.compute_script_shifts(
            base_ascent,
            base_descent,
            is_text_like,
            tl_box.as_ref(),
            sup_box.as_ref(),
            bl_box.as_ref(),
            sub_box.as_ref(),
            style,
        );

        // P914 — Kerning de duas alturas de correção por quadrante
        let tl_kern = if let Some(ref tb) = tl_box {
            self.compute_math_kern(
                base_char,
                tl_char,
                0, // TopLeft
                base_ascent - sup_offset,
                sup_offset - tb.descent,
                style,
            )
        } else {
            0.0
        };

        let bl_kern = if let Some(ref bb) = bl_box {
            self.compute_math_kern(
                base_char,
                bl_char,
                1, // BottomLeft
                bb.ascent - sub_offset,
                sub_offset - base_descent,
                style,
            )
        } else {
            0.0
        };

        let tl_push = tl_box.as_ref().map(|b| b.width + tl_kern).unwrap_or(0.0);
        let bl_push = bl_box.as_ref().map(|b| b.width + bl_kern).unwrap_or(0.0);
        let base_offset_x = tl_push.max(bl_push);

        let is_limits = self.block
            && match base {
                Content::MathIdent(s) | Content::MathText(s) => {
                    let ch = s.chars().next().unwrap_or('\0');
                    (symbols::is_large_operator(ch) && !symbols::is_integral_char(ch))
                        || symbols::is_limit_function(s.as_str())
                }
                Content::MathOp(e) => e.limits,
                _ => false,
            };

        let mut ascent = base_ascent;
        let mut descent = base_descent;
        let mut items = Vec::new();

        if let Some(tb) = tl_box {
            ascent = ascent.max(sup_offset + tb.ascent);
            let x_tl = base_offset_x - tl_push;
            for item in tb.items {
                items.push(offset_item(item, Pt(x_tl), Pt(-sup_offset)));
            }
        }

        if let Some(bb) = bl_box {
            descent = descent.max(sub_offset + bb.descent);
            let x_bl = base_offset_x - bl_push;
            for item in bb.items {
                items.push(offset_item(item, Pt(x_bl), Pt(sub_offset)));
            }
        }

        if is_limits {
            let upper_gap = self
                .constants
                .to_pt(self.constants.upper_limit_gap_min, style.size)
                .val();
            let lower_gap = self
                .constants
                .to_pt(self.constants.lower_limit_gap_min, style.size)
                .val();

            let sup_box_opt = sup_box;
            let sub_box_opt = sub_box;

            let max_content_w = [
                base_width,
                sup_box_opt.as_ref().map(|b| b.width).unwrap_or(0.0),
                sub_box_opt.as_ref().map(|b| b.width).unwrap_or(0.0),
            ]
            .iter()
            .cloned()
            .fold(0.0f64, f64::max);

            let total_w = base_offset_x + max_content_w;

            let x_base = base_offset_x + (max_content_w - base_width) / 2.0;
            for item in base_box.items {
                items.push(offset_item(item, Pt(x_base), Pt(0.0)));
            }

            if let Some(sb) = sup_box_opt {
                let y_sup = -(base_ascent + upper_gap + sb.descent);
                let x_sup = base_offset_x + (max_content_w - sb.width) / 2.0;
                ascent = ascent.max(base_ascent + upper_gap + sb.descent + sb.ascent);
                for item in sb.items {
                    items.push(offset_item(item, Pt(x_sup), Pt(y_sup)));
                }
            }

            if let Some(sb) = sub_box_opt {
                let y_sub = base_descent + lower_gap + sb.ascent;
                let x_sub = base_offset_x + (max_content_w - sb.width) / 2.0;
                descent = descent.max(base_descent + lower_gap + sb.ascent + sb.descent);
                for item in sb.items {
                    items.push(offset_item(item, Pt(x_sub), Pt(y_sub)));
                }
            }

            MathBox { width: total_w, ascent, descent, items }
        } else {
            for item in base_box.items {
                items.push(offset_item(item, Pt(base_offset_x), Pt(0.0)));
            }

            let scripts_x = base_offset_x + base_width;
            let mut post_width: f64 = 0.0;

            if let Some(sup_b) = sup_box {
                ascent = ascent.max(sup_offset + sup_b.ascent);

                let kern_sup = self.compute_math_kern(
                    base_char,
                    sup_char,
                    2, // TopRight
                    base_ascent - sup_offset,
                    sup_offset - sup_b.descent,
                    style,
                );

                for item in sup_b.items {
                    items.push(offset_item(item, Pt(scripts_x + kern_sup), Pt(-sup_offset)));
                }
                post_width = post_width.max(sup_b.width + kern_sup);
            }

            if let Some(sub_b) = sub_box {
                descent = descent.max(sub_offset + sub_b.descent);

                let kern_sub = self.compute_math_kern(
                    base_char,
                    sub_char,
                    3, // BottomRight
                    sub_b.ascent - sub_offset,
                    sub_offset - base_descent,
                    style,
                );

                for item in sub_b.items {
                    items.push(offset_item(item, Pt(scripts_x + kern_sub), Pt(sub_offset)));
                }
                post_width = post_width.max(sub_b.width + kern_sub);
            }

            MathBox { width: scripts_x + post_width, ascent, descent, items }
        }
    }

    /// P914 — Função auxiliar para calcular deslocamentos adaptativos de sub/sobrescritos.
    #[allow(clippy::too_many_arguments)]
    fn compute_script_shifts(
        &self,
        base_ascent: f64,
        base_descent: f64,
        is_text_like: bool,
        tl_box: Option<&MathBox>,
        tr_box: Option<&MathBox>,
        bl_box: Option<&MathBox>,
        br_box: Option<&MathBox>,
        style: &TextStyle,
    ) -> (f64, f64) {
        let size = style.size;

        let sup_shift_up = self
            .constants
            .to_pt(self.constants.superscript_shift_up, size)
            .val();
        let sup_bottom_min = self
            .constants
            .to_pt(self.constants.superscript_bottom_min, size)
            .val();
        let sup_bottom_max_with_sub = self
            .constants
            .to_pt(self.constants.superscript_bottom_max_with_subscript, size)
            .val();
        let sup_drop_max = self
            .constants
            .to_pt(self.constants.superscript_baseline_drop_max, size)
            .val();
        let gap_min = self
            .constants
            .to_pt(self.constants.sub_superscript_gap_min, size)
            .val();
        let sub_shift_down = self
            .constants
            .to_pt(self.constants.subscript_shift_down, size)
            .val();
        let sub_top_max = self
            .constants
            .to_pt(self.constants.subscript_top_max, size)
            .val();
        let sub_drop_min = self
            .constants
            .to_pt(self.constants.subscript_baseline_drop_min, size)
            .val();

        let mut shift_up = 0.0_f64;
        let mut shift_down = 0.0_f64;

        if tl_box.is_some() || tr_box.is_some() {
            let drop_term = if is_text_like {
                0.0
            } else {
                (base_ascent - sup_drop_max).max(0.0)
            };
            let tl_descent = tl_box.map(|b| b.descent).unwrap_or(0.0);
            let tr_descent = tr_box.map(|b| b.descent).unwrap_or(0.0);
            shift_up = shift_up
                .max(sup_shift_up)
                .max(drop_term)
                .max(sup_bottom_min + tl_descent)
                .max(sup_bottom_min + tr_descent);
        }

        if bl_box.is_some() || br_box.is_some() {
            let drop_term = if is_text_like {
                0.0
            } else {
                (base_descent + sub_drop_min).max(0.0)
            };
            let bl_ascent = bl_box.map(|b| b.ascent).unwrap_or(0.0);
            let br_ascent = br_box.map(|b| b.ascent).unwrap_or(0.0);
            shift_down = shift_down
                .max(sub_shift_down)
                .max(drop_term)
                .max(bl_ascent - sub_top_max)
                .max(br_ascent - sub_top_max);
        }

        // Ajuste simultâneo para os pares de scripts (tl, bl) e (tr, br)
        for (sup, sub) in [(tl_box, bl_box), (tr_box, br_box)] {
            if let (Some(sup_b), Some(sub_b)) = (sup, sub) {
                let sup_bottom = shift_up - sup_b.descent;
                let sub_top = sub_b.ascent - shift_down;
                let gap = sup_bottom - sub_top;
                if gap < gap_min {
                    let increase = gap_min - gap;
                    let sup_only = (sup_bottom_max_with_sub - sup_bottom).clamp(0.0, increase);
                    let rest = (increase - sup_only) / 2.0;
                    shift_up += sup_only + rest;
                    shift_down += rest;
                }
            }
        }

        (shift_up, shift_down)
    }

    /// P914 — Função auxiliar para calcular o kerning de duas alturas de correção.
    /// `quadrant`: 0 = TopLeft, 1 = BottomLeft, 2 = TopRight, 3 = BottomRight.
    fn compute_math_kern(
        &self,
        base_char: Option<char>,
        script_char: Option<char>,
        quadrant: u8,
        h_top_pt: f64,
        h_bot_pt: f64,
        style: &TextStyle,
    ) -> f64 {
        let base_kern = base_char
            .map(|c| self.metrics.math_kern(c, style))
            .unwrap_or_default();
        let script_kern = script_char
            .map(|c| self.metrics.math_kern(c, style))
            .unwrap_or_default();

        let (base_q_kern, script_inv_kern) = match quadrant {
            0 => (&base_kern.top_left, &script_kern.bottom_right),
            1 => (&base_kern.bottom_left, &script_kern.top_right),
            2 => (&base_kern.top_right, &script_kern.bottom_left),
            _ => (&base_kern.bottom_right, &script_kern.top_left),
        };

        let summed_at = |h_pt: f64| -> f64 {
            let h_du = h_pt * self.constants.upem / style.size.val().max(0.001);
            let bk = base_q_kern.kern_at(h_du);
            let sk = script_inv_kern.kern_at(h_du);
            self.constants.to_pt(bk + sk, style.size).val()
        };

        summed_at(h_top_pt).max(summed_at(h_bot_pt))
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {}
}
