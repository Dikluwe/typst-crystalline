//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/math.md
//! @prompt-hash 8be87568
//! @layer L1
//! @updated 2026-04-03

use ecow::EcoString;

use crate::entities::{
    content::Content,
    layout_types::{Frame, FrameItem, Point, Pt, Size, TextStyle},
};
use crate::rules::layout::FontMetrics;

/// Motor de layout matemático.
///
/// Recebe `Content` matemático e produz um `Frame` com `FrameItem::Text`
/// posicionados horizontalmente a partir da origem `(0, 0)`.
///
/// **Passo 36**: `MathIdent` e `MathText` → `FrameItem::Text`.
/// Restantes variantes (`MathFrac`, `MathAttach`, `MathRoot`) → texto plano
/// sem `[...]`. Passo 37+ implementa posicionamento vertical tipográfico.
pub struct MathLayouter<'a, M: FontMetrics> {
    metrics:  &'a M,
    items:    Vec<FrameItem>,
    cursor_x: Pt,
}

impl<'a, M: FontMetrics> MathLayouter<'a, M> {
    pub fn new(metrics: &'a M) -> Self {
        Self { metrics, items: Vec::new(), cursor_x: Pt(0.0) }
    }

    /// Ponto de entrada: recebe o body de uma equação e produz um Frame.
    ///
    /// O `Frame` retornado tem posições relativas à origem — o integrante
    /// (layouter principal) é responsável por ajustar para posição absoluta.
    pub fn layout_equation(
        &mut self,
        body:  &Content,
        style: &TextStyle,
    ) -> Frame {
        self.layout_math_content(body, style);
        let (_, line_height) = self.metrics.vertical_metrics(style.size);
        Frame {
            size:  Size { width: self.cursor_x, height: line_height },
            items: std::mem::take(&mut self.items),
        }
    }

    /// Percorre a árvore de Content matemático recursivamente.
    fn layout_math_content(&mut self, content: &Content, style: &TextStyle) {
        match content {
            Content::MathIdent(text) | Content::MathText(text) => {
                self.emit_text(text.clone(), style);
            }

            Content::MathSequence(nodes) => {
                for node in nodes.iter() {
                    self.layout_math_content(node, style);
                }
            }

            // Passo 37+: layout tipográfico correcto.
            // Por ora: texto plano sem placeholder [ ].
            Content::MathFrac { num, den } => {
                let text = EcoString::from(
                    format!("{}/{}", num.plain_text(), den.plain_text())
                );
                self.emit_text(text, style);
            }

            Content::MathAttach { base, sub, sup } => {
                self.layout_math_content(base, style);
                if let Some(s) = sup {
                    self.emit_text(EcoString::from("^"), style);
                    self.layout_math_content(s, style);
                }
                if let Some(s) = sub {
                    self.emit_text(EcoString::from("_"), style);
                    self.layout_math_content(s, style);
                }
            }

            Content::MathRoot { index, radicand } => {
                let prefix = match index {
                    None    => EcoString::from("√"),
                    Some(i) => EcoString::from(format!("{}√", i.plain_text())),
                };
                self.emit_text(prefix, style);
                self.layout_math_content(radicand, style);
            }

            // Content não-matemático dentro de uma equação (raro):
            // usar plain_text como fallback.
            other => {
                let text = EcoString::from(other.plain_text());
                if !text.trim().is_empty() {
                    self.emit_text(text, style);
                }
            }
        }
    }

    /// Emite um `FrameItem::Text` na posição actual e avança o cursor.
    fn emit_text(&mut self, text: EcoString, style: &TextStyle) {
        if text.is_empty() { return; }
        let width = self.metrics.advance(&text, style.size);
        self.items.push(FrameItem::Text {
            pos:   Point { x: self.cursor_x, y: Pt(0.0) },
            text,
            style: *style,
        });
        self.cursor_x = self.cursor_x + width;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use crate::rules::layout::FixedMetrics;

    fn default_style() -> TextStyle {
        TextStyle::regular(Pt(12.0))
    }

    #[test]
    fn math_layouter_math_ident_produz_frame_nao_vazio() {
        let mut ml = MathLayouter::new(&FixedMetrics);
        let frame  = ml.layout_equation(
            &Content::MathIdent("x".into()),
            &default_style(),
        );
        assert!(!frame.items.is_empty(), "MathIdent deve produzir pelo menos 1 item");
    }

    #[test]
    fn math_layouter_math_text_produz_frame_nao_vazio() {
        let mut ml = MathLayouter::new(&FixedMetrics);
        let frame  = ml.layout_equation(
            &Content::MathText("sin".into()),
            &default_style(),
        );
        assert!(!frame.items.is_empty());
    }

    #[test]
    fn math_layouter_sequence_produz_multiplos_items() {
        let mut ml = MathLayouter::new(&FixedMetrics);
        let seq = Content::MathSequence(
            Arc::from(vec![
                Content::MathIdent("x".into()),
                Content::MathText("+".into()),
                Content::MathIdent("y".into()),
            ].into_boxed_slice())
        );
        let frame = ml.layout_equation(&seq, &default_style());
        assert_eq!(frame.items.len(), 3, "x + y deve produzir 3 items");
    }

    #[test]
    fn math_layouter_frac_sem_placeholder_colchetes() {
        let mut ml = MathLayouter::new(&FixedMetrics);
        let frac = Content::MathFrac {
            num: Box::new(Content::MathIdent("a".into())),
            den: Box::new(Content::MathIdent("b".into())),
        };
        let frame = ml.layout_equation(&frac, &default_style());
        for item in &frame.items {
            if let FrameItem::Text { text, .. } = item {
                assert!(!text.contains('['),
                    "frac não deve conter '[': {}", text);
            }
        }
        assert!(!frame.items.is_empty(), "frac deve produzir items");
    }

    #[test]
    fn math_layouter_cursor_avanca_horizontalmente() {
        let mut ml = MathLayouter::new(&FixedMetrics);
        let frame  = ml.layout_equation(
            &Content::MathSequence(Arc::from(vec![
                Content::MathIdent("a".into()),
                Content::MathIdent("b".into()),
            ].into_boxed_slice())),
            &default_style(),
        );
        // Segundo item deve ter pos.x > 0 (cursor avançou)
        if let [first, second] = frame.items.as_slice() {
            if let (FrameItem::Text { pos: p1, .. }, FrameItem::Text { pos: p2, .. }) = (first, second) {
                assert!(p2.x > p1.x, "segundo item deve estar à direita do primeiro");
            }
        }
    }

    #[test]
    fn math_layouter_math_attach_sem_colchetes() {
        let mut ml = MathLayouter::new(&FixedMetrics);
        let attach = Content::MathAttach {
            base: Box::new(Content::MathIdent("x".into())),
            sub:  None,
            sup:  Some(Box::new(Content::MathText("2".into()))),
        };
        let frame = ml.layout_equation(&attach, &default_style());
        for item in &frame.items {
            if let FrameItem::Text { text, .. } = item {
                assert!(!text.contains('['), "attach não deve conter '[': {}", text);
            }
        }
    }
}
