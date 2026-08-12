//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/math/layout/attach.md
//! @prompt-hash ed266740
//! @layer L1
//! @updated 2026-04-23
//!
//! Método `layout_attach` de `MathLayouter`. Extraído de `math/layout/mod.rs`
//! no Passo 96.8 conforme ADR-0037.

use crate::compiler::layout::FontMetrics;
use crate::entities::{
    content::Content,
    layout_types::{FrameItem, MathSize, Pt, TextStyle},
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
        // **P915** — `top_style` (tl/sup) herda `cramped` do estilo ambiente,
        // sem forçar; `bottom_style` (bl/sub) força `cramped: true` — achado
        // do vanilla (`style_for_subscript` = `[style_for_superscript,
        // style_cramped()]`, `style.rs:333`): só o subscrito é cramped, o
        // superscrito nunca é forçado. Ver `attach.md` §P915.
        // **P945** — `math_size` mantido honesto: scripts descem um nível
        // (`style_for_superscript`, `style.rs:315-323`: Display|Text→Script,
        // Script|ScriptScript→ScriptScript). Só o campo — o factor de
        // tamanho actual NÃO muda neste passo (ver `attach.md` §P945).
        let script_math_size = match style.math_size {
            MathSize::Display | MathSize::Text => MathSize::Script,
            MathSize::Script | MathSize::ScriptScript => MathSize::ScriptScript,
        };
        let top_style = TextStyle {
            size: style.size * self.constants.script_percent_scale_down,
            math_script: true,
            math_size: script_math_size,
            ..style.clone()
        };
        let bottom_style = TextStyle { cramped: true, ..top_style.clone() };

        // Layout de todos os scripts primeiro para obter dimensões e métricas
        let tl_box = tl.map(|c| self.layout_node(c, &top_style));
        let bl_box = bl.map(|c| self.layout_node(c, &bottom_style));
        let sup_box = sup.map(|c| self.layout_node(c, &top_style));
        let sub_box = sub.map(|c| self.layout_node(c, &bottom_style));

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
        // **P963** — `is_text_like` do vanilla é sobre o FRAGMENTO
        // (`fragment/mod.rs:129-135`): um glifo é text-like só se
        // `!extended_shape` — uma base ESTICADA (variante de Display ou
        // assembly, `FrameItem::Glyph` no cristalino) não é text-like, e o
        // termo `base_ascent − sup_drop_max` (drop) aplica-se ao
        // `shift_up` do script lateral. Antes derivava-se do Content
        // (`MathIdent`/`MathText`) — verdadeiro para `∫` mesmo esticado,
        // deixando o sup lateral de integrais em Display ~8pt abaixo do
        // vanilla (medido; auditoria externa 3ª ronda lia "limite superior
        // ~5× errado"). Ver `attach.md` §P963.
        let base_is_extended = base_box
            .items
            .iter()
            .any(|i| matches!(i, FrameItem::Glyph { .. }));
        let is_text_like =
            matches!(base, Content::MathIdent(_) | Content::MathText(_)) && !base_is_extended;

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

        // **P992** — `Content::MathLimitsOverride` (`limits()`/`scripts()`)
        // é verificado ANTES do `self.block &&` exterior: `limits(body,
        // inline: true)` (default vanilla) empilha mesmo em modo inline —
        // diferente de `Content::MathOp { limits, .. }`, que continua
        // gated por `self.block` (ver nota P772w acima). Fórmula cobre os
        // 3 casos (ver `attach.md` §P992): `scripts()` (`limits: false`)
        // nunca empilha; `limits()` default (`inline: true`) empilha
        // sempre; `limits(.., inline: false)` só empilha em modo bloco.
        let is_limits = match base {
            Content::MathLimitsOverride(e) => e.limits && (e.inline || self.block),
            _ => {
                self.block
                    && match base {
                        Content::MathIdent(s) | Content::MathText(s) => {
                            let ch = s.chars().next().unwrap_or('\0');
                            (symbols::is_large_operator(ch)
                                && !symbols::is_integral_char(ch))
                                || symbols::is_limit_function(s.as_str())
                        }
                        Content::MathOp(e) => e.limits,
                        _ => false,
                    }
            }
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
            // P959 — fórmula do vanilla (`compute_limit_shifts`,
            // lab/typst-original/crates/typst-layout/src/math/scripts.rs:290-313):
            // shifts baseline-a-baseline com piso `max(rise/drop, gap + extent)`.
            // Ver `compiler/math/layout/attach.md` §P959.
            let upper_gap = self
                .constants
                .to_pt(self.constants.upper_limit_gap_min, style.size)
                .val();
            let lower_gap = self
                .constants
                .to_pt(self.constants.lower_limit_gap_min, style.size)
                .val();
            let upper_rise = self
                .constants
                .to_pt(self.constants.upper_limit_baseline_rise_min, style.size)
                .val();
            let lower_drop = self
                .constants
                .to_pt(self.constants.lower_limit_baseline_drop_min, style.size)
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
                // t_shift já inclui base_ascent: a caixa deriva directamente
                // do shift (t_shift + sup.ascent), sem somar os termos outra vez.
                let t_shift = base_ascent + upper_rise.max(upper_gap + sb.descent);
                let y_sup = -t_shift;
                let x_sup = base_offset_x + (max_content_w - sb.width) / 2.0;
                ascent = ascent.max(t_shift + sb.ascent);
                for item in sb.items {
                    items.push(offset_item(item, Pt(x_sup), Pt(y_sup)));
                }
            }

            if let Some(sb) = sub_box_opt {
                // b_shift já inclui base_descent (mesma nota do sup acima).
                let b_shift = base_descent + lower_drop.max(lower_gap + sb.ascent);
                let y_sub = b_shift;
                let x_sub = base_offset_x + (max_content_w - sb.width) / 2.0;
                descent = descent.max(b_shift + sb.descent);
                for item in sb.items {
                    items.push(offset_item(item, Pt(x_sub), Pt(y_sub)));
                }
            }

            MathBox { width: total_w, ascent, descent, items }
        } else {
            // **P971** — italics correction da base para o termo do
            // subscrito pós-fixado (vanilla `compute_post_script_widths`,
            // `lab/typst-original/crates/typst-layout/src/math/
            // scripts.rs:222-227`: `br_kern − base.italics_correction()`).
            // Extraída do glyph id real da base — para uma base esticada
            // (`∫` em display) é a IC da **variante** (integral.v1=450du),
            // não a do glifo base (180du). Bases `Text` não carregam glyph
            // id em L1 → IC=0 (residual registado em `attach.md` §P971).
            // Extraída ANTES de `base_box` ser consumida abaixo.
            let base_ic = base_box
                .items
                .iter()
                .find_map(|i| match i {
                    FrameItem::Glyph { glyph_id, .. } => Some(*glyph_id),
                    _ => None,
                })
                .map(|gid| {
                    self.metrics.italics_correction(gid, style.size, style).val()
                })
                .unwrap_or(0.0);

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

                // **P971** — o subscrito pós-fixado recua a IC da base
                // (vanilla `scripts.rs:222-227`: "a bounding box da base já
                // conta com a italic correction"). Para o ∫ esticado em
                // display: recuo de 450du = 5.4pt a 12pt — o subscrito
                // acompanha a inclinação do símbolo. O sobrescrito não leva
                // termo. Bases com IC=0 ficam bit-a-bit iguais.
                let kern_sub = self.compute_math_kern(
                    base_char,
                    sub_char,
                    3, // BottomRight
                    sub_b.ascent - sub_offset,
                    sub_offset - base_descent,
                    style,
                ) - base_ic;

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

        // **P915** — `cramped` do estilo AMBIENTE (o `style` recebido por
        // `layout_attach`, não `top_style`/`bottom_style` internos) decide
        // entre `superscript_shift_up`/`superscript_shift_up_cramped` —
        // achado do vanilla (`scripts.rs:100,325-330`): é o único termo da
        // fórmula afectado por `cramped`. Ver `attach.md` §P915.
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
