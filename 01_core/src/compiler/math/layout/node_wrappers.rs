//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/math/layout/node_wrappers.md
//! @prompt-hash e070c121
//! @layer L1
//! @updated 2026-10-17
//!
//! Layout dos wrappers matemáticos de classe e estilo.

use super::*;
use crate::entities::elements::math_class_override::MathClassOverrideElem;
use crate::entities::elements::math_styled::MathStyledElem;

impl<'a, M: FontMetrics> MathLayouter<'a, M> {
    pub(super) fn layout_class_override(
        &self,
        e: &MathClassOverrideElem,
        style: &TextStyle,
    ) -> MathBox {
        // Um `Content::Text` diretamente promovido por
        // `math.class` representa o GlyphFragment de P1132w, não o
        // TextItem direto do catch-all P1293. Preserve a largura
        // vigente aqui; a IC do fragmento é aplicada abaixo.
        let mut box_ = if let Content::Text(text) = &e.body {
            let mut box_ = self.layout_text_node(text, style);
            box_.width = self.metrics.text_width(text, style.size, style).val();
            box_
        } else {
            self.layout_node(&e.body, style)
        };
        // **P1132w** — ao transformar o GlyphItem num frame com
        // classe explícita, o vanilla conserva a IC no avanço. O
        // wrapper cristalino era totalmente transparente e perdia-a.
        if let Some(c) = single_math_leaf_char(&e.body) {
            let extended = !self.metrics.vertical_glyph_variants(c, style).is_empty()
                || !self.metrics.vertical_glyph_assembly(c, style).is_empty()
                || !self.metrics.horizontal_glyph_variants(c, style).is_empty()
                || !self.metrics.horizontal_glyph_assembly(c, style).is_empty();
            if !extended {
                box_.width +=
                    self.metrics.char_italics_correction(c, style.size, style).val();
            }
        }
        box_
    }

    pub(super) fn layout_styled_node(
        &self,
        m: &MathStyledElem,
        style: &TextStyle,
    ) -> MathBox {
        let mut node = m;
        let mut glyph_kind = None;
        let mut size_kind = None;
        let mut bold = None;
        let mut italic = None;
        let mut cramped = None;

        let body = loop {
            if let Some(kind) = node.kind {
                if kind.is_size_variant() {
                    size_kind = Some(kind);
                } else {
                    glyph_kind = Some(kind);
                }
            }
            if node.bold.is_some() {
                bold = node.bold;
            }
            if node.italic.is_some() {
                italic = node.italic;
            }
            if node.cramped.is_some() {
                cramped = node.cramped;
            }

            match &node.body {
                Content::MathStyled(inner) => node = inner.as_ref(),
                body => break body,
            }
        };

        let transformed = apply_math_style(body, glyph_kind, bold, italic);
        let size_factor = size_kind.map(|k| k.size_factor()).unwrap_or(1.0);
        let mut math_style = style.clone();
        math_style.size = style.size * size_factor;
        if let Some(k) = size_kind {
            use crate::entities::layout_types::MathSize;
            use crate::entities::math_style::MathStyleKind;
            match k {
                MathStyleKind::Display => {
                    math_style.math_size = MathSize::Display;
                }
                MathStyleKind::Inline => {
                    math_style.math_size = MathSize::Text;
                }
                MathStyleKind::Script => {
                    math_style.math_size = MathSize::Script;
                }
                MathStyleKind::SScript => {
                    math_style.math_size = MathSize::ScriptScript;
                }
                MathStyleKind::Plain
                | MathStyleKind::SansSerif
                | MathStyleKind::Chancery
                | MathStyleKind::Roundhand
                | MathStyleKind::Fraktur
                | MathStyleKind::Monospace
                | MathStyleKind::DoubleStruck => {}
            }
        }
        if let Some(c) = cramped {
            math_style.cramped = c;
        }
        if matches!(transformed, Content::MathMatrix(_))
            && matches!(size_kind, Some(MathStyleKind::Display))
        {
            // `display(mat(..))` escolhe Text como nível exterior;
            // uma equação de bloco mantém Display→Text a tamanho
            // cheio conforme P945.
            math_style.math_size = crate::entities::layout_types::MathSize::Text;
        }
        // Se kind glyph foi aplicado (chars já variant-encoded) OU
        // italic explícito foi set, suprimir auto-itálico do
        // `MathIdent` handler. Itálico explícito é honrado via
        // codepoint já transformado (Italic plane).
        if glyph_kind.is_some()
            || size_kind.is_some()
            || italic.is_some()
            || bold.is_some()
            || cramped.is_some()
        {
            math_style.italic = false;
        }
        self.layout_node(&transformed, &math_style)
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn modulo_compila_e_carrega() {}
}
