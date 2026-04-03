//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/math.md
//! @prompt-hash 8be87568
//! @layer L1
//! @updated 2026-04-03

use ecow::EcoString;

use crate::entities::{
    content::Content,
    layout_types::{FrameItem, Point, Pt, TextStyle},
};
use crate::rules::layout::FontMetrics;

/// Caixa tipográfica de um nó matemático.
/// Todas as medidas são em pontos, relativas à baseline da equação.
/// `ascent` > 0 (acima da baseline), `descent` > 0 (abaixo da baseline).
#[derive(Debug, Clone)]
struct MathBox {
    width:   f64,
    ascent:  f64,
    descent: f64,
    /// Items com posições relativas ao topo esquerdo deste MathBox.
    items: Vec<FrameItem>,
}

impl MathBox {
    fn height(&self) -> f64 {
        self.ascent + self.descent
    }

    /// Converte para FrameItems com posição no frame pai.
    ///
    /// `x_origin`: deslocamento horizontal no frame pai.
    /// `baseline_y`: posição da baseline no frame pai (y cresce para baixo).
    ///
    /// Conversão: `parent_y = baseline_y - ascent + local_y`
    ///   - `local_y = 0` → topo do box (acima da baseline)
    ///   - `local_y = ascent` → na baseline
    fn place(self, x_origin: f64, baseline_y: f64) -> Vec<FrameItem> {
        self.items.into_iter().map(|mut item| {
            match item {
                FrameItem::Text { ref mut pos, .. } => {
                    pos.x = Pt(pos.x.val() + x_origin);
                    pos.y = Pt(baseline_y - self.ascent + pos.y.val());
                }
                FrameItem::Line { ref mut start, ref mut end, .. } => {
                    start.x = Pt(start.x.val() + x_origin);
                    end.x   = Pt(end.x.val() + x_origin);
                    start.y = Pt(baseline_y - self.ascent + start.y.val());
                    end.y   = Pt(baseline_y - self.ascent + end.y.val());
                }
            }
            item
        }).collect()
    }
}

/// Motor de layout matemático — stateless.
///
/// Recebe `Content` matemático e produz um `Vec<FrameItem>` com posições
/// relativas à origem `(0, 0)`, prontas para integração no layouter principal.
///
/// **Passo 36**: `MathIdent` e `MathText` → `FrameItem::Text`.
/// **Passo 37**: `MathFrac` (numerador/denominador) e `MathAttach`
///   (sup/sub com posicionamento vertical) implementados via `MathBox`.
pub struct MathLayouter<'a, M: FontMetrics> {
    metrics: &'a M,
}

impl<'a, M: FontMetrics> MathLayouter<'a, M> {
    pub fn new(metrics: &'a M) -> Self {
        Self { metrics }
    }

    /// Ponto de entrada: recebe o body de uma equação e produz `Vec<FrameItem>`.
    ///
    /// Os items retornados têm posições relativas à origem — o layouter principal
    /// é responsável por ajustar para posição absoluta na página.
    pub fn layout_equation(
        &self,
        body:  &Content,
        style: &TextStyle,
    ) -> Vec<FrameItem> {
        let math_box = self.layout_node(body, style);
        // Baseline no topo do box (simplificado: Passo 38+ alinhará com x-height)
        let baseline_y = math_box.ascent;
        math_box.place(0.0, baseline_y)
    }

    /// Percorre a árvore de Content matemático recursivamente, produzindo um `MathBox`.
    fn layout_node(&self, content: &Content, style: &TextStyle) -> MathBox {
        match content {
            Content::MathIdent(text) | Content::MathText(text) => {
                self.layout_text_node(text, style)
            }

            Content::MathSequence(nodes) => {
                self.layout_sequence(nodes, style)
            }

            Content::MathFrac { num, den } => {
                self.layout_frac(num, den, style)
            }

            Content::MathAttach { base, sub, sup } => {
                self.layout_attach(base, sub.as_deref(), sup.as_deref(), style)
            }

            Content::MathRoot { index: _, radicand } => {
                let inner  = self.layout_node(radicand, style);
                let prefix = self.layout_text_node(&EcoString::from("√"), style);
                self.hconcat(vec![prefix, inner])
            }

            other => {
                let text: EcoString = other.plain_text().into();
                if text.trim().is_empty() {
                    MathBox { width: 0.0, ascent: 0.0, descent: 0.0, items: vec![] }
                } else {
                    self.layout_text_node(&text, style)
                }
            }
        }
    }

    /// Nó folha: texto com métricas tipográficas.
    ///
    /// Posição do item dentro do MathBox: `(0, 0)` — relativo ao topo esquerdo.
    fn layout_text_node(&self, text: &EcoString, style: &TextStyle) -> MathBox {
        if text.is_empty() {
            return MathBox { width: 0.0, ascent: 0.0, descent: 0.0, items: vec![] };
        }
        let width  = self.metrics.advance(text, style.size).val();
        let vm     = self.metrics.vertical_metrics(style.size);
        let ascent  = vm.0.val();
        let descent = (vm.1 - vm.0).val();
        MathBox {
            width,
            ascent,
            descent,
            items: vec![FrameItem::Text {
                pos:   Point { x: Pt(0.0), y: Pt(0.0) },
                text:  text.clone(),
                style: *style,
            }],
        }
    }

    fn layout_sequence(&self, nodes: &[Content], style: &TextStyle) -> MathBox {
        let boxes: Vec<MathBox> = nodes.iter()
            .map(|n| self.layout_node(n, style))
            .collect();
        self.hconcat(boxes)
    }

    /// Concatenação horizontal: posiciona MathBoxes lado a lado.
    fn hconcat(&self, boxes: Vec<MathBox>) -> MathBox {
        let mut x       = 0.0_f64;
        let mut ascent  = 0.0_f64;
        let mut descent = 0.0_f64;
        let mut items   = Vec::new();

        for b in boxes {
            ascent  = ascent.max(b.ascent);
            descent = descent.max(b.descent);
            for mut item in b.items {
                match item {
                    FrameItem::Text { ref mut pos, .. } => {
                        pos.x = Pt(pos.x.val() + x);
                    }
                    FrameItem::Line { ref mut start, ref mut end, .. } => {
                        start.x = Pt(start.x.val() + x);
                        end.x   = Pt(end.x.val() + x);
                    }
                }
                items.push(item);
            }
            x += b.width;
        }

        MathBox { width: x, ascent, descent, items }
    }

    /// Fracção: numerador acima da linha de fracção, denominador abaixo.
    ///
    /// Tamanho do sub-estilo: 70% do estilo base.
    /// A linha de fracção não é renderizada neste passo (Passo 38+).
    fn layout_frac(&self, num: &Content, den: &Content, style: &TextStyle) -> MathBox {
        let sub_style = TextStyle { size: style.size * 0.7, ..*style };

        let num_box = self.layout_node(num, &sub_style);
        let den_box = self.layout_node(den, &sub_style);

        let width = num_box.width.max(den_box.width);

        let rule_thickness = (style.size * 0.05).val();
        let gap            = (style.size * 0.1).val();

        // ascent cobre todo o numerador + espaço + metade da linha
        let ascent  = num_box.height() + gap + rule_thickness / 2.0;
        // descent cobre metade da linha + espaço + todo o denominador
        let descent = den_box.height() + gap + rule_thickness / 2.0;

        // Centrar horizontalmente
        let num_x = (width - num_box.width) / 2.0;
        let den_x = (width - den_box.width) / 2.0;

        // Numerador: topo do MathBox (local_y = 0)
        let num_y = 0.0_f64;
        // Denominador: abaixo do numerador + linha de fracção
        let den_y = num_box.height() + gap + rule_thickness + gap;
        // Linha de fracção: centro entre numerador e denominador
        let rule_local_y = num_box.height() + gap + rule_thickness / 2.0;

        let mut items = Vec::new();

        for mut item in num_box.items {
            if let FrameItem::Text { ref mut pos, .. } = item {
                pos.x = Pt(pos.x.val() + num_x);
                pos.y = Pt(pos.y.val() + num_y);
            }
            items.push(item);
        }

        // Linha de fracção posicionada entre numerador e denominador.
        items.push(FrameItem::Line {
            start:     Point { x: Pt(0.0),    y: Pt(rule_local_y) },
            end:       Point { x: Pt(width),  y: Pt(rule_local_y) },
            thickness: rule_thickness,
        });

        for mut item in den_box.items {
            if let FrameItem::Text { ref mut pos, .. } = item {
                pos.x = Pt(pos.x.val() + den_x);
                pos.y = Pt(pos.y.val() + den_y);
            }
            items.push(item);
        }

        MathBox { width, ascent, descent, items }
    }

    /// Attach: base na baseline, sup elevado, sub baixado.
    ///
    /// Tamanho do sub-estilo: 65% do estilo base.
    /// sup elevado a 50% do ascender; sub baixado a 30% da descida.
    fn layout_attach(
        &self,
        base: &Content,
        sub:  Option<&Content>,
        sup:  Option<&Content>,
        style: &TextStyle,
    ) -> MathBox {
        let base_box     = self.layout_node(base, style);
        let script_style = TextStyle { size: style.size * 0.65, ..*style };

        let vm         = self.metrics.vertical_metrics(style.size);
        let sup_offset = vm.0.val() * 0.5;           // 50% do ascender
        let sub_offset = (vm.1 - vm.0).val() * 0.3;  // 30% da descida

        let mut x       = base_box.width;
        let mut ascent  = base_box.ascent;
        let mut descent = base_box.descent;
        let mut items   = base_box.items;

        if let Some(sup_content) = sup {
            let sup_box = self.layout_node(sup_content, &script_style);
            ascent = ascent.max(sup_offset + sup_box.ascent);
            for mut item in sup_box.items {
                if let FrameItem::Text { ref mut pos, .. } = item {
                    pos.x = Pt(pos.x.val() + x);
                    // sup elevado: y negativo (acima da baseline local)
                    pos.y = Pt(pos.y.val() - sup_offset);
                }
                items.push(item);
            }
            x += sup_box.width;
        }

        if let Some(sub_content) = sub {
            let sub_box = self.layout_node(sub_content, &script_style);
            descent = descent.max(sub_offset + sub_box.descent);
            for mut item in sub_box.items {
                if let FrameItem::Text { ref mut pos, .. } = item {
                    pos.x = Pt(pos.x.val() + x);
                    // sub baixado: y positivo (abaixo da baseline local)
                    pos.y = Pt(pos.y.val() + sub_offset);
                }
                items.push(item);
            }
        }

        MathBox { width: x, ascent, descent, items }
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

    fn size10_style() -> TextStyle {
        TextStyle::regular(Pt(10.0))
    }

    // ── Testes herdados do Passo 36 (adaptados para nova API) ─────────────

    #[test]
    fn math_layouter_math_ident_produz_items_nao_vazios() {
        let ml    = MathLayouter::new(&FixedMetrics);
        let items = ml.layout_equation(
            &Content::MathIdent("x".into()),
            &default_style(),
        );
        assert!(!items.is_empty(), "MathIdent deve produzir pelo menos 1 item");
    }

    #[test]
    fn math_layouter_math_text_produz_items_nao_vazios() {
        let ml    = MathLayouter::new(&FixedMetrics);
        let items = ml.layout_equation(
            &Content::MathText("sin".into()),
            &default_style(),
        );
        assert!(!items.is_empty());
    }

    #[test]
    fn math_layouter_sequence_produz_multiplos_items() {
        let ml = MathLayouter::new(&FixedMetrics);
        let seq = Content::MathSequence(
            Arc::from(vec![
                Content::MathIdent("x".into()),
                Content::MathText("+".into()),
                Content::MathIdent("y".into()),
            ].into_boxed_slice())
        );
        let items = ml.layout_equation(&seq, &default_style());
        assert_eq!(items.len(), 3, "x + y deve produzir 3 items");
    }

    #[test]
    fn math_layouter_frac_sem_placeholder_colchetes() {
        let ml = MathLayouter::new(&FixedMetrics);
        let frac = Content::MathFrac {
            num: Box::new(Content::MathIdent("a".into())),
            den: Box::new(Content::MathIdent("b".into())),
        };
        let items = ml.layout_equation(&frac, &default_style());
        for item in &items {
            if let FrameItem::Text { text, .. } = item {
                assert!(!text.contains('['),
                    "frac não deve conter '[': {}", text);
            }
        }
        assert!(!items.is_empty(), "frac deve produzir items");
    }

    #[test]
    fn math_layouter_cursor_avanca_horizontalmente() {
        let ml    = MathLayouter::new(&FixedMetrics);
        let items = ml.layout_equation(
            &Content::MathSequence(Arc::from(vec![
                Content::MathIdent("a".into()),
                Content::MathIdent("b".into()),
            ].into_boxed_slice())),
            &default_style(),
        );
        // Segundo item deve ter pos.x > 0 (cursor avançou)
        if let [first, second] = items.as_slice() {
            if let (FrameItem::Text { pos: p1, .. }, FrameItem::Text { pos: p2, .. }) = (first, second) {
                assert!(p2.x > p1.x, "segundo item deve estar à direita do primeiro");
            }
        }
    }

    #[test]
    fn math_layouter_math_attach_sem_colchetes() {
        let ml = MathLayouter::new(&FixedMetrics);
        let attach = Content::MathAttach {
            base: Box::new(Content::MathIdent("x".into())),
            sub:  None,
            sup:  Some(Box::new(Content::MathText("2".into()))),
        };
        let items = ml.layout_equation(&attach, &default_style());
        for item in &items {
            if let FrameItem::Text { text, .. } = item {
                assert!(!text.contains('['), "attach não deve conter '[': {}", text);
            }
        }
    }

    // ── Novos testes do Passo 37 + 38 ────────────────────────────────────

    #[test]
    fn math_frac_tem_dois_ou_mais_items() {
        let ml   = MathLayouter::new(&FixedMetrics);
        let frac = Content::MathFrac {
            num: Box::new(Content::MathIdent("a".into())),
            den: Box::new(Content::MathIdent("b".into())),
        };
        let items = ml.layout_equation(&frac, &size10_style());
        assert!(items.len() >= 2, "frac deve ter >= 2 items, tem {}", items.len());
    }

    #[test]
    fn math_frac_numerador_acima_denominador() {
        let ml   = MathLayouter::new(&FixedMetrics);
        let frac = Content::MathFrac {
            num: Box::new(Content::MathIdent("a".into())),
            den: Box::new(Content::MathIdent("b".into())),
        };
        let items = ml.layout_equation(&frac, &size10_style());

        let ys: Vec<f64> = items.iter().filter_map(|item| {
            if let FrameItem::Text { pos, .. } = item { Some(pos.y.val()) }
            else { None }
        }).collect();

        assert!(ys.len() >= 2, "deve ter pelo menos 2 posições y");
        assert!(ys[0] < ys[1],
            "numerador (y={}) deve estar acima do denominador (y={})", ys[0], ys[1]);
    }

    #[test]
    fn math_attach_sup_elevado() {
        let ml = MathLayouter::new(&FixedMetrics);
        let attach = Content::MathAttach {
            base: Box::new(Content::MathIdent("x".into())),
            sub:  None,
            sup:  Some(Box::new(Content::MathIdent("2".into()))),
        };
        let items = ml.layout_equation(&attach, &size10_style());
        assert!(items.len() >= 2, "x^2 deve ter >= 2 items");

        let ys: Vec<f64> = items.iter().filter_map(|item| {
            if let FrameItem::Text { pos, .. } = item { Some(pos.y.val()) }
            else { None }
        }).collect();

        // sup deve estar acima da base (y menor, pois y cresce para baixo)
        assert!(ys[1] < ys[0],
            "sup (y={}) deve estar acima da base (y={})", ys[1], ys[0]);
    }

    #[test]
    fn math_frac_tem_item_linha() {
        let ml   = MathLayouter::new(&FixedMetrics);
        let frac = Content::MathFrac {
            num: Box::new(Content::MathIdent("a".into())),
            den: Box::new(Content::MathIdent("b".into())),
        };
        let items = ml.layout_equation(&frac, &size10_style());
        let has_line = items.iter().any(|item| matches!(item, FrameItem::Line { .. }));
        assert!(has_line, "frac deve ter FrameItem::Line para a linha de fracção");
    }

    #[test]
    fn math_frac_linha_horizontal() {
        let ml   = MathLayouter::new(&FixedMetrics);
        let frac = Content::MathFrac {
            num: Box::new(Content::MathIdent("a".into())),
            den: Box::new(Content::MathIdent("b".into())),
        };
        let items = ml.layout_equation(&frac, &size10_style());
        for item in &items {
            if let FrameItem::Line { start, end, .. } = item {
                assert_eq!(start.y.val(), end.y.val(),
                    "linha de fracção deve ser horizontal");
                assert!(end.x.val() > start.x.val(),
                    "linha de fracção deve ter largura > 0");
            }
        }
    }

    #[test]
    fn math_attach_sub_baixado() {
        let ml = MathLayouter::new(&FixedMetrics);
        let attach = Content::MathAttach {
            base: Box::new(Content::MathIdent("x".into())),
            sub:  Some(Box::new(Content::MathIdent("i".into()))),
            sup:  None,
        };
        let items = ml.layout_equation(&attach, &size10_style());
        assert!(items.len() >= 2);

        let ys: Vec<f64> = items.iter().filter_map(|item| {
            if let FrameItem::Text { pos, .. } = item { Some(pos.y.val()) }
            else { None }
        }).collect();

        // sub deve estar abaixo da base (y maior)
        assert!(ys[1] > ys[0],
            "sub (y={}) deve estar abaixo da base (y={})", ys[1], ys[0]);
    }
}
