//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/math/layout.md
//! @prompt-hash c45536b1
//! @layer L1
//! @updated 2026-04-23
//!
//! Método `layout_attach` de `MathLayouter`. Extraído de `math/layout/mod.rs`
//! no Passo 96.8 conforme ADR-0037.

use crate::entities::{
    content::Content,
    layout_types::{Pt, TextStyle},
};
use crate::rules::layout::FontMetrics;

use super::{MathBox, offset_item};
use crate::entities::glyph_variants::MathGlyphKern;
use super::symbols;

impl<'a, M: FontMetrics> super::MathLayouter<'a, M> {
    pub(super) fn layout_attach(
        &self,
        base: &Content,
        tl:   Option<&Content>,
        bl:   Option<&Content>,
        sub:  Option<&Content>,
        sup:  Option<&Content>,
        style: &TextStyle,
    ) -> MathBox {
        let base_box     = self.layout_node(base, style);
        let script_style = TextStyle {
            size: style.size * self.constants.script_percent_scale_down,
            ..style.clone()
        };

        let sup_offset = self.constants.to_pt(
            self.constants.superscript_shift_up, style.size
        ).val();
        let sub_offset = self.constants.to_pt(
            self.constants.subscript_shift_down, style.size
        ).val();

        // Extrair o char da base para consultar MathKernInfo e detectar operador grande.
        // Apenas MathIdent/MathText têm char único; outros ficam com kern zero.
        let base_char: Option<char> = match base {
            Content::MathIdent(s) | Content::MathText(s) => s.chars().next(),
            _ => None,
        };
        let base_kern: MathGlyphKern = base_char
            .map(|c| self.metrics.math_kern(c))
            .unwrap_or_default();

        // Passo 49/50 — empilhamento vertical apenas em bloco (display mode).
        // Inline: sub/sup à direita para não expandir a linha de texto.
        let is_limits = self.block && match base {
            Content::MathIdent(s) | Content::MathText(s) => {
                let ch = s.chars().next().unwrap_or('\0');
                symbols::is_large_operator(ch) || symbols::is_limit_function(s.as_str())
            }
            _ => false,
        };

        // ── Passo 3a/3b/3c — Coluna esquerda (pre-scripts) ──────────────
        // Layout dos left-scripts para obter larguras.
        let tl_box = tl.map(|c| self.layout_node(c, &script_style));
        let bl_box = bl.map(|c| self.layout_node(c, &script_style));

        // Kern dos quadrantes esquerdos — cada script avalia o kern no ponto de
        // contacto com a base: tl pelo seu descent (parte inferior), bl pelo ascent.
        let tl_kern = if let Some(ref tb) = tl_box {
            let h_du = tb.descent * self.constants.upem / style.size.val().max(0.001);
            self.constants.to_pt(base_kern.top_left.kern_at(h_du), style.size).val()
        } else { 0.0 };
        let bl_kern = if let Some(ref bb) = bl_box {
            let h_du = bb.ascent * self.constants.upem / style.size.val().max(0.001);
            self.constants.to_pt(base_kern.bottom_left.kern_at(h_du), style.size).val()
        } else { 0.0 };

        // Passo 53 — Kern diferenciado por quadrante esquerdo.
        // Cada left-script tem o seu próprio afastamento (push = largura + kern).
        // kern negativo = aproximação da base (sem .abs() — geometria correcta).
        // base_offset_x = max dos dois pushes; scripts independentes em x.
        let tl_push = tl_box.as_ref().map(|b| b.width + tl_kern).unwrap_or(0.0);
        let bl_push = bl_box.as_ref().map(|b| b.width + bl_kern).unwrap_or(0.0);
        let base_offset_x = tl_push.max(bl_push);

        // Salvar métricas da base antes de consumir base_box.items.
        let base_ascent  = base_box.ascent;
        let base_descent = base_box.descent;
        let base_width   = base_box.width;

        // ── Construção dos items ──────────────────────────────────────────
        let mut ascent  = base_ascent;
        let mut descent = base_descent;
        let mut items   = Vec::new();

        // Posicionar tl (pre-superscript): alinhado à direita da coluna esquerda,
        // elevado pelo sup_offset acima da baseline da base.
        if let Some(tb) = tl_box {
            ascent = ascent.max(sup_offset + tb.ascent);
            let x_tl = base_offset_x - tl_push;
            for item in tb.items {
                items.push(offset_item(item, Pt(x_tl), Pt(-sup_offset)));
            }
        }

        // Posicionar bl (pre-subscript): aproxima-se da base com kern independente.
        if let Some(bb) = bl_box {
            descent = descent.max(sub_offset + bb.descent);
            let x_bl = base_offset_x - bl_push;
            for item in bb.items {
                items.push(offset_item(item, Pt(x_bl), Pt(sub_offset)));
            }
        }

        if is_limits {
            // ── Passo 49 — Empilhamento vertical para operadores grandes ──────
            //
            // sup fica centrado ACIMA da base, separado por upper_limit_gap_min.
            // sub fica centrado ABAIXO da base, separado por lower_limit_gap_min.
            let upper_gap = self.constants.to_pt(
                self.constants.upper_limit_gap_min, style.size
            ).val();
            let lower_gap = self.constants.to_pt(
                self.constants.lower_limit_gap_min, style.size
            ).val();

            let sup_box_opt = sup.map(|c| self.layout_node(c, &script_style));
            let sub_box_opt = sub.map(|c| self.layout_node(c, &script_style));

            // Largura máxima dos três elementos para centrar em X.
            let max_content_w = [
                base_width,
                sup_box_opt.as_ref().map(|b| b.width).unwrap_or(0.0),
                sub_box_opt.as_ref().map(|b| b.width).unwrap_or(0.0),
            ].iter().cloned().fold(0.0f64, f64::max);

            let total_w = base_offset_x + max_content_w;

            // Base centrada.
            let x_base = base_offset_x + (max_content_w - base_width) / 2.0;
            for item in base_box.items {
                items.push(offset_item(item, Pt(x_base), Pt(0.0)));
            }

            // Limite superior: bottom do sup fica upper_gap acima do top da base.
            // y_sup = -(base_ascent + upper_gap + sup.descent)
            if let Some(sb) = sup_box_opt {
                let y_sup = -(base_ascent + upper_gap + sb.descent);
                let x_sup = base_offset_x + (max_content_w - sb.width) / 2.0;
                ascent = ascent.max(base_ascent + upper_gap + sb.descent + sb.ascent);
                for item in sb.items {
                    items.push(offset_item(item, Pt(x_sup), Pt(y_sup)));
                }
            }

            // Limite inferior: top do sub fica lower_gap abaixo do bottom da base.
            // y_sub = base_descent + lower_gap + sub.ascent
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
            // ── Right-scripts (sub/sup à direita — layout horizontal) ────────
            // Base: posicionada em x = base_offset_x.
            for item in base_box.items {
                items.push(offset_item(item, Pt(base_offset_x), Pt(0.0)));
            }

            // Right-scripts partem de base_offset_x + base_width.
            let mut x = base_offset_x + base_width;

            if let Some(sup_content) = sup {
                let sup_box = self.layout_node(sup_content, &script_style);
                ascent = ascent.max(sup_offset + sup_box.ascent);

                // Kern: quadrante top-right. Altura de conexão = ascent do sup.
                let sup_h_du = sup_box.ascent * self.constants.upem
                    / style.size.val().max(0.001);
                let kern_sup = self.constants.to_pt(
                    base_kern.top_right.kern_at(sup_h_du), style.size
                ).val();

                for item in sup_box.items {
                    items.push(offset_item(item, Pt(x + kern_sup), Pt(-sup_offset)));
                }
                x += sup_box.width + kern_sup;
            }

            if let Some(sub_content) = sub {
                let sub_box = self.layout_node(sub_content, &script_style);
                descent = descent.max(sub_offset + sub_box.descent);

                // Kern: quadrante bottom-right. Altura de conexão = ascent do sub.
                let sub_h_du = sub_box.ascent * self.constants.upem
                    / style.size.val().max(0.001);
                let kern_sub = self.constants.to_pt(
                    base_kern.bottom_right.kern_at(sub_h_du), style.size
                ).val();

                for item in sub_box.items {
                    items.push(offset_item(item, Pt(x + kern_sub), Pt(sub_offset)));
                }
            }

            MathBox { width: x, ascent, descent, items }
        }
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {}
}
