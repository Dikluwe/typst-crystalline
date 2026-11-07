//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/math/layout/op.md
//! @prompt-hash c6f094c9
//! @layer L1
//! @updated 2026-11-01
//!
//! Método `layout_op` de `MathLayouter`. Extraído de `math/layout/mod.rs`
//! no Passo 909 (completa o padrão de fatiamento de P314/ADR-0104, deixado
//! para trás por este handler ter sido adicionado depois, em P298).

use crate::compiler::layout::FontMetrics;
use crate::entities::{content::Content, layout_types::TextStyle};

use super::MathBox;

impl<'a, M: FontMetrics> super::MathLayouter<'a, M> {
    /// **P298** — Layout op: emite text como math child standard.
    /// Handler trivial (delegate) — verdadeira lógica está em
    /// `layout_attach` que detecta `Content::MathOp { limits: true, .. }`
    /// como base e renderiza scripts em limits-style (ver `op.md`).
    pub(super) fn layout_op(&self, text: &Content, style: &TextStyle) -> MathBox {
        let local = TextStyle {
            dir: Some(crate::entities::dir::Dir::LTR),
            ..style.clone()
        };
        self.layout_node(text, &local)
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {}
}

#[cfg(test)]
mod issue_8990_direction_tests {
    use crate::compiler::layout::FixedMetrics;
    use crate::compiler::math::layout::MathLayouter;
    use crate::entities::dir::Dir;
    use crate::entities::layout_types::{FrameItem, Pt, TextStyle};
    use crate::entities::style::Styles;
    use crate::entities::{content::Content, value::Value};

    #[test]
    fn issue8990_ab_operator_default_is_local_ltr_and_keeps_textitem() {
        use crate::entities::layout_types::MathSize;
        for outer_dir in [None, Some(Dir::RTL)] {
            for size in [MathSize::Text, MathSize::Script, MathSize::ScriptScript] {
                let style = TextStyle {
                    math: true,
                    dir: outer_dir,
                    math_size: size,
                    ..TextStyle::regular(Pt(11.0))
                };
                let layouter = MathLayouter::new(&FixedMetrics, false, &style);
                let result = layouter.layout_op(&Content::text("אבג."), &style);
                let Some(FrameItem::Text { text, style: emitted, .. }) =
                    result.items.first()
                else {
                    panic!("operator text missing")
                };
                assert_eq!(text.as_str(), "אבג.");
                assert_eq!(emitted.dir, Some(Dir::LTR));
                assert_eq!(emitted.math_size, size);
                assert!(emitted.math_text_item);
                assert_eq!(style.dir, outer_dir);
            }
        }
    }

    #[test]
    fn issue_8990_operator_preserves_explicit_text_direction() {
        for text in ["p.v.", "אבג."] {
            for direction in [Dir::LTR, Dir::RTL] {
                let style = TextStyle { math: true, ..TextStyle::regular(Pt(11.0)) };
                let content = Content::styled(
                    Content::text(text),
                    Styles::new().push_custom("text.dir", Value::Dir(direction)),
                );
                let layouter = MathLayouter::new(&FixedMetrics, false, &style);
                let result = layouter.layout_op(&content, &style);
                let leaves: Vec<_> = result
                    .items
                    .iter()
                    .filter_map(|item| match item {
                        FrameItem::Text { text, style, .. }
                        | FrameItem::TextShaped { text, style, .. } => {
                            Some((text, style))
                        }
                        FrameItem::TextReplacement { .. }
                        | FrameItem::TextSeparator { .. }
                        | FrameItem::Semantic { .. }
                        | FrameItem::Line { .. }
                        | FrameItem::Glyph { .. }
                        | FrameItem::Image { .. }
                        | FrameItem::Shape { .. }
                        | FrameItem::Group { .. }
                        | FrameItem::Link { .. } => None,
                    })
                    .collect();
                assert!(!leaves.is_empty(), "operator must retain its authored text");
                assert_eq!(
                    leaves.iter().map(|(text, _)| text.as_str()).collect::<String>(),
                    text
                );
                assert!(
                    leaves.iter().all(|(_, emitted)| emitted.dir == Some(direction)),
                    "explicit {direction:?} was lost for {text:?}: {leaves:?}"
                );
                assert_eq!(style.dir, None, "outer style must remain unchanged");
            }
        }
    }
}
