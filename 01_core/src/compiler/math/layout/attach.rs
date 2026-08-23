//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/math/layout/attach.md
//! @prompt-hash d1050800
//! @layer L1
//! @updated 2026-08-20
//!
//! Método `layout_attach` de `MathLayouter`. Extraído de `math/layout/mod.rs`
//! no Passo 96.8 conforme ADR-0037. Suporta 6 anexos concomitantes: t, b, tl, bl, tr, br.

use crate::compiler::layout::FontMetrics;
use crate::entities::{
    content::Content,
    layout_types::{FrameItem, MathSize, Pt, TextStyle},
};

use super::symbols;
use super::{offset_item, MathBox};

impl<'a, M: FontMetrics> super::MathLayouter<'a, M> {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn layout_attach(
        &self,
        base: &Content,
        t: Option<&Content>,
        b: Option<&Content>,
        tl: Option<&Content>,
        bl: Option<&Content>,
        tr: Option<&Content>,
        br: Option<&Content>,
        style: &TextStyle,
    ) -> MathBox {
        let base_box = self.layout_node(base, style);

        let (script_math_size, factor) = match style.math_size {
            MathSize::Display | MathSize::Text => {
                (MathSize::Script, self.constants.script_percent_scale_down)
            }
            MathSize::Script => (
                MathSize::ScriptScript,
                self.constants.script_script_percent_scale_down
                    / self.constants.script_percent_scale_down,
            ),
            MathSize::ScriptScript => (MathSize::ScriptScript, 1.0),
        };
        let top_style = TextStyle {
            size: style.size * factor,
            math_script: true,
            math_size: script_math_size,
            ..style.clone()
        };
        let bottom_style = TextStyle { cramped: true, ..top_style.clone() };

        // **P992** — `Content::MathLimitsOverride` (`limits()`/`scripts()`)
        // é verificado ANTES do `self.block &&` exterior: `limits(body,
        // inline: true)` (default vanilla) empilha mesmo em modo inline —
        // diferente de `Content::MathOp { limits, .. }`, que continua
        // gated por `self.block` (ver nota P772w acima).
        let unwrapped_base = {
            let mut b = base;
            loop {
                match b {
                    Content::MathLimitsOverride(e) => b = &e.body,
                    Content::MathClassOverride(e) => b = &e.body,
                    Content::MathStyled(e) => b = &e.body,
                    Content::Styled(body, _) => b = body,
                    _ => break,
                }
            }
            b
        };

        let is_limits = match base {
            Content::MathLimitsOverride(e) => e.limits && (e.inline || self.block),
            _ => {
                self.block
                    && match unwrapped_base {
                        Content::MathIdent(s) | Content::MathText(s) => {
                            let ch = s.chars().next().unwrap_or(' ');
                            (symbols::is_large_operator(ch)
                                && !symbols::is_integral_char(ch))
                                || symbols::is_limit_function(s.as_str())
                        }
                        Content::MathOp(e) => e.limits,
                        _ => false,
                    }
            }
        };

        // Resolução inteligente de t/b vs tr/br (paridade vanilla resolve_attach)
        let (t, tr) = if is_limits {
            if t.is_some() {
                (t, tr)
            } else {
                (tr, None)
            }
        } else if tr.is_some() {
            (t, tr)
        } else {
            (None, t)
        };

        let (b, br) = if is_limits {
            if b.is_some() {
                (b, br)
            } else {
                (br, None)
            }
        } else if br.is_some() {
            (b, br)
        } else {
            (None, b)
        };

        // Layout de todos os anexos presentes
        let t_box = t.map(|c| self.layout_node(c, &top_style));
        let b_box = b.map(|c| self.layout_node(c, &bottom_style));
        let tl_box = tl.map(|c| self.layout_node(c, &top_style));
        let bl_box = bl.map(|c| self.layout_node(c, &bottom_style));
        let tr_box = tr.map(|c| self.layout_node(c, &top_style));
        let br_box = br.map(|c| self.layout_node(c, &bottom_style));

        // Extrair chars para consulta a MathGlyphKern
        let base_char: Option<char> = match unwrapped_base {
            Content::MathIdent(s) | Content::MathText(s) => s.chars().next(),
            Content::MathOp(e) => match &e.text {
                Content::Text(s) | Content::MathText(s) | Content::MathIdent(s) => {
                    if s == "sum" {
                        Some('∑')
                    } else if s == "product" {
                        Some('∏')
                    } else if s == "integral" {
                        Some('∫')
                    } else {
                        s.chars().next()
                    }
                }
                _ => None,
            },
            _ => None,
        };
        let extract_char = |c: Option<&Content>| -> Option<char> {
            match c {
                Some(Content::MathIdent(s)) | Some(Content::MathText(s)) => {
                    s.chars().next()
                }
                _ => None,
            }
        };
        let tl_char = extract_char(tl);
        let bl_char = extract_char(bl);
        let tr_char = extract_char(tr);
        let br_char = extract_char(br);

        let base_ascent = base_box.ascent;
        let base_descent = base_box.descent;
        let base_width = base_box.width;

        let is_op = match unwrapped_base {
            Content::MathOp(_) => true,
            Content::MathIdent(s) | Content::MathText(s) => s
                .chars()
                .next()
                .map(|ch| symbols::is_large_operator(ch) || symbols::is_integral_char(ch))
                .unwrap_or(false),
            _ => false,
        };
        // P1132d: no vanilla, `text_like` pertence ao tipo do fragmento.
        // Sequências/anexos/delimitados produzem FrameFragment (false), ainda
        // que não sejam operadores. Só os fragmentos textuais atómicos ficam
        // true; uma variante/assembly extensível (`Glyph`) força false.
        fn content_is_text_like(c: &Content) -> bool {
            match c {
                Content::MathIdent(_) | Content::MathText(_) | Content::MathOp(_) => true,
                Content::MathDelimited(e) => content_is_text_like(&e.body),
                Content::MathSequence(xs) | Content::Sequence(xs) => {
                    xs.iter().all(content_is_text_like)
                }
                Content::MathClassOverride(e) => content_is_text_like(&e.body),
                Content::MathLimitsOverride(e) => content_is_text_like(&e.body),
                Content::MathStyled(e) => content_is_text_like(&e.body),
                Content::Styled(body, _) => content_is_text_like(body),
                // **P1133b** — espaçamento fraco de borda (`dif`) colapsa
                // antes do layout e não produz fragmento com `math_size`;
                // é neutro na agregação `all(fragment.is_text_like())`.
                Content::HSpace(e) if e.weak => true,
                _ => false,
            }
        }
        let has_extended_glyph = base_box.items.iter().any(|item| match item {
            FrameItem::Glyph { glyph_id, base_char, .. } => self
                .metrics
                .vertical_glyph_variants(*base_char, style)
                .variants
                .first()
                .map_or(true, |base| base.glyph_id != *glyph_id),
            _ => false,
        });
        // No vanilla, `extended_shape` é uma propriedade do glifo na fonte,
        // não apenas da variante escolhida. Operadores grandes continuam
        // não-text-like no inline mesmo usando o glifo base.
        let is_extended_shape_glyph = match unwrapped_base {
            Content::MathIdent(s) | Content::MathText(s) if s.chars().count() == 1 => {
                s.chars().next().is_some_and(symbols::is_large_operator)
            }
            _ => false,
        };
        let is_text_like = content_is_text_like(unwrapped_base)
            && !has_extended_glyph
            && !is_extended_shape_glyph;

        let glyph_ascent = self.metrics.text_edges(style.size, style).0.val().abs();
        let is_nested_attach = matches!(base, Content::MathAttach(_));
        let is_multi_letter_text_op = match unwrapped_base {
            Content::MathIdent(s) | Content::MathText(s) => s.chars().count() > 1,
            Content::MathOp(e) => match &e.text {
                Content::Text(s) | Content::MathText(s) | Content::MathIdent(s) => {
                    s.chars().count() > 1
                }
                _ => false,
            },
            _ => false,
        };
        let eff_base_ascent = if is_nested_attach && !is_op {
            base_ascent.min(glyph_ascent)
        } else {
            base_ascent
        };
        let eff_base_descent = if is_multi_letter_text_op || (is_nested_attach && !is_op)
        {
            0.0
        } else {
            base_descent
        };

        // Deslocamentos adaptativos de sub/sobrescrito (compute_script_shifts)
        let (shift_up, shift_down) = self.compute_script_shifts(
            eff_base_ascent,
            eff_base_descent,
            is_text_like,
            tl_box.as_ref(),
            tr_box.as_ref(),
            bl_box.as_ref(),
            br_box.as_ref(),
            style,
        );

        // Deslocamentos verticais de limites (compute_limit_shifts)
        let t_shift = t_box
            .as_ref()
            .map(|tb| {
                let upper_gap_min = self
                    .constants
                    .to_pt(self.constants.upper_limit_gap_min, style.size)
                    .val();
                let upper_rise_min = self
                    .constants
                    .to_pt(self.constants.upper_limit_baseline_rise_min, style.size)
                    .val();
                base_ascent + upper_rise_min.max(upper_gap_min + tb.descent)
            })
            .unwrap_or(0.0);

        let b_shift = b_box
            .as_ref()
            .map(|bb| {
                let lower_gap_min = self
                    .constants
                    .to_pt(self.constants.lower_limit_gap_min, style.size)
                    .val();
                let lower_drop_min = self
                    .constants
                    .to_pt(self.constants.lower_limit_baseline_drop_min, style.size)
                    .val();
                base_descent + lower_drop_min.max(lower_gap_min + bb.ascent)
            })
            .unwrap_or(0.0);

        let space_after_script = self
            .constants
            .to_pt(self.constants.space_after_script, style.size)
            .val();

        // Kerning dos 4 quadrantes
        let tl_kern = if let Some(ref tb) = tl_box {
            self.compute_math_kern(
                base_char,
                tl_char,
                0, // TopLeft
                base_ascent - shift_up,
                shift_up - tb.descent,
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
                bb.ascent - shift_down,
                shift_down - base_descent,
                style,
            )
        } else {
            0.0
        };

        let tr_kern = if let Some(ref tb) = tr_box {
            self.compute_math_kern(
                base_char,
                tr_char,
                2, // TopRight
                base_ascent - shift_up,
                shift_up - tb.descent,
                style,
            )
        } else {
            0.0
        };

        let base_ic = if is_multi_letter_text_op {
            // **P1133c** — `MathOp("lim")` é FrameFragment no vanilla;
            // não herda a IC do `m` final para centrar limites.
            0.0
        } else {
            base_box
                .items
                .iter()
                .find_map(|i| match i {
                    FrameItem::Glyph { glyph_id, .. } => Some(
                        self.metrics
                            .italics_correction(*glyph_id, style.size, style)
                            .val(),
                    ),
                    FrameItem::TextShaped { glyphs, .. } => glyphs.last().map(|g| {
                        self.metrics
                            .italics_correction(g.glyph_id, style.size, style)
                            .val()
                    }),
                    FrameItem::Text { text, .. } => text.chars().last().map(|c| {
                        self.metrics.char_italics_correction(c, style.size, style).val()
                    }),
                    _ => None,
                })
                .or_else(|| {
                    base_char.map(|c| {
                        self.metrics.char_italics_correction(c, style.size, style).val()
                    })
                })
                .unwrap_or(0.0)
        };
        let br_kern = if let Some(ref bb) = br_box {
            self.compute_math_kern(
                base_char,
                br_char,
                3, // BottomRight
                bb.ascent - shift_down,
                shift_down - base_descent,
                style,
            ) - base_ic
        } else {
            0.0
        };

        // Larguras de scripts
        let tl_pre = tl_box
            .as_ref()
            .map(|b| space_after_script + b.width + tl_kern)
            .unwrap_or(0.0);
        let bl_pre = bl_box
            .as_ref()
            .map(|b| space_after_script + b.width + bl_kern)
            .unwrap_or(0.0);

        let tr_post = tr_box
            .as_ref()
            .map(|b| space_after_script + b.width + tr_kern)
            .unwrap_or(0.0);
        let br_post = br_box
            .as_ref()
            .map(|b| space_after_script + b.width + br_kern)
            .unwrap_or(0.0);

        // Larguras de limits
        let delta = base_ic / 2.0;
        let (t_pre, t_post) = t_box
            .as_ref()
            .map(|tb| {
                let half = (tb.width - base_width) / 2.0;
                (half - delta, half + delta)
            })
            .unwrap_or((0.0, 0.0));

        let (b_pre, b_post) = b_box
            .as_ref()
            .map(|bb| {
                let half = (bb.width - base_width) / 2.0;
                (half + delta, half - delta)
            })
            .unwrap_or((0.0, 0.0));

        let pre_width = t_pre.max(b_pre).max(tl_pre).max(bl_pre).max(0.0);
        let post_width = t_post.max(b_post).max(tr_post).max(br_post).max(0.0);
        let total_width = pre_width + base_width + post_width;

        let mut ascent = base_ascent;
        let mut descent = base_descent;
        let mut items = Vec::new();

        // 1. Base
        for item in base_box.items {
            items.push(offset_item(item, Pt(pre_width), Pt(0.0)));
        }

        // 2. Pre-scripts
        if let Some(tb) = tl_box {
            ascent = ascent.max(shift_up + tb.ascent);
            let x_tl = pre_width - tl_pre + space_after_script;
            for item in tb.items {
                items.push(offset_item(item, Pt(x_tl), Pt(-shift_up)));
            }
        }

        if let Some(bb) = bl_box {
            descent = descent.max(shift_down + bb.descent);
            let x_bl = pre_width - bl_pre + space_after_script;
            for item in bb.items {
                items.push(offset_item(item, Pt(x_bl), Pt(shift_down)));
            }
        }

        // 3. Post-scripts
        if let Some(tb) = tr_box {
            ascent = ascent.max(shift_up + tb.ascent);
            let x_tr = pre_width + base_width + tr_kern;
            for item in tb.items {
                items.push(offset_item(item, Pt(x_tr), Pt(-shift_up)));
            }
        }

        if let Some(bb) = br_box {
            descent = descent.max(shift_down + bb.descent);
            let x_br = pre_width + base_width + br_kern;
            for item in bb.items {
                items.push(offset_item(item, Pt(x_br), Pt(shift_down)));
            }
        }

        // 4. Limits
        if let Some(tb) = t_box {
            ascent = ascent.max(t_shift + tb.ascent);
            let x_t = pre_width - t_pre;
            for item in tb.items {
                items.push(offset_item(item, Pt(x_t), Pt(-t_shift)));
            }
        }

        if let Some(bb) = b_box {
            descent = descent.max(b_shift + bb.descent);
            let x_b = pre_width - b_pre;
            for item in bb.items {
                items.push(offset_item(item, Pt(x_b), Pt(b_shift)));
            }
        }

        MathBox { width: total_width, ascent, descent, items }
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

        let sup_shift_up_const = if style.cramped {
            self.constants.superscript_shift_up_cramped
        } else {
            self.constants.superscript_shift_up
        };
        let sup_shift_up = self.constants.to_pt(sup_shift_up_const, size).val();
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
        let sub_shift_down =
            self.constants.to_pt(self.constants.subscript_shift_down, size).val();
        let sub_top_max =
            self.constants.to_pt(self.constants.subscript_top_max, size).val();
        let sub_drop_min = self
            .constants
            .to_pt(self.constants.subscript_baseline_drop_min, size)
            .val();

        let mut shift_up = 0.0_f64;
        let mut shift_down = 0.0_f64;

        if tl_box.is_some() || tr_box.is_some() {
            let drop_term =
                if is_text_like { 0.0 } else { (base_ascent - sup_drop_max).max(0.0) };
            let tl_descent = tl_box.map(|b| b.descent).unwrap_or(0.0);
            let tr_descent = tr_box.map(|b| b.descent).unwrap_or(0.0);
            shift_up = shift_up
                .max(sup_shift_up)
                .max(drop_term)
                .max(sup_bottom_min + tl_descent)
                .max(sup_bottom_min + tr_descent);
        }

        if bl_box.is_some() || br_box.is_some() {
            let drop_term =
                if is_text_like { 0.0 } else { (base_descent + sub_drop_min).max(0.0) };
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
                let sub_top = sup_b.descent - shift_down; // em coordenadas onde y cresce para baixo, sub_top é -shift_down + sub_b.ascent
                let gap = (shift_up + shift_down) - (sup_b.descent + sub_b.ascent);
                if gap < gap_min {
                    let increase = gap_min - gap;
                    let sup_only =
                        (sup_bottom_max_with_sub - sup_bottom).clamp(0.0, increase);
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
