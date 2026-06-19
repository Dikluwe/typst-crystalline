//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout.md
//! @prompt-hash 12536b5c
//! @layer L1
//! @updated 2026-04-23
//!
//! Braço `Content::Equation` do `layout_content`. Extraído de `layout/mod.rs`
//! no Passo 96.7 conforme ADR-0037.

use crate::entities::{
    content::Content,
    elements::equation::EquationElem,
    image_sizer::ImageSizer,
    layout_types::{FrameItem, Point, Pt},
};
use crate::rules::math;

use super::metrics::FontMetrics;

impl<'a, M: FontMetrics, S: ImageSizer> super::Layouter<'a, M, S> {
    /// Layout de `Content::Equation { body, block }`.
    pub(super) fn layout_equation(&mut self, body: &Content, block: bool, numbering_active: bool) {
        // Auto-numeração: equações de bloco numeradas avançam o contador antes de
        // desenhar (Passo 59). O número (N) é acrescentado depois da equação.
        // Lote F-2 S2 (P335): o "ativo" é **assado** no `EquationElem` (escopo
        // léxico via chain, fecha o canal global StateRegistry). O **valor** do
        // contador continua via Introspector (`flat_counter_at`), gateado pelo
        // mesmo `numbering_active` assado (via payload, em `from_tags`).
        let is_numbered = block && numbering_active;
        // P190F (M6 categoria Counters core): Layouter mutação
        // `self.counter.step_flat` removida — counter equation
        // populated via Introspector path (CounterRegistry +
        // gate em `from_tags` arm Equation P186E activado por
        // SetEquationNumbering P199B). Layouter só lê.
        let _ = is_numbered;

        let math_layouter = math::layout::MathLayouter::new(&self.metrics, block);
        let math_items    = math_layouter.layout_equation(body, &self.style);

        if block
            && self.regions.current.cursor_x.0 > self.page_config.margin { self.flush_line(); }

        // Integrar items matemáticos no frame actual.
        // pos.x e pos.y são relativos à origem da equação —
        // pos.y inclui deslocamento vertical (sup/sub, frac).
        let offset_x = self.regions.current.cursor_x;
        // Equações inline: deslocar para cima por axis_pt de modo a que o
        // eixo matemático (axis_height acima da baseline) coincida com o
        // baseline do texto circundante (Passo 48).
        let axis_pt = if block {
            Pt(0.0)
        } else {
            let c = self.metrics.math_constants();
            c.to_pt(c.axis_height, self.style.size)
        };
        let offset_y = self.regions.current.cursor_y - axis_pt;
        for item in math_items {
            match item {
                FrameItem::Text { pos, text, style } => {
                    let abs_pos = Point {
                        x: offset_x + pos.x,
                        y: offset_y + pos.y,
                    };
                    let advance = self.metrics.advance(&text, style.size);
                    self.regions.current.current_line.push(FrameItem::Text { pos: abs_pos, text, style });
                    self.regions.current.cursor_x += advance;
                }
                FrameItem::Line { start, end, thickness, color } => {
                    let abs_start = Point { x: offset_x + start.x, y: offset_y + start.y };
                    let abs_end   = Point { x: offset_x + end.x,   y: offset_y + end.y };
                    self.regions.current.current_line.push(FrameItem::Line {
                        start: abs_start, end: abs_end, thickness,
                        // P285: equação preserva cor da Line original (math
                        // frac/sqrt usam None → preto bit-exact).
                        color,
                    });
                }
                FrameItem::Image { .. } => {}   // imagens não ocorrem em math inline
                FrameItem::Shape { .. } => {}   // formas não ocorrem em math inline
                FrameItem::Group { .. } => {}   // grupos não ocorrem em math inline
                FrameItem::Glyph { pos, glyph_id, x_advance, size } => {
                    let abs_pos = Point {
                        x: offset_x + pos.x,
                        y: offset_y + pos.y,
                    };
                    self.regions.current.current_line.push(FrameItem::Glyph {
                        pos: abs_pos, glyph_id, x_advance, size,
                    });
                    self.regions.current.cursor_x += x_advance;
                }
            }
        }

        if block { self.flush_line(); }

        // Acrescentar número da equação inline após o flush (Passo 59).
        // DEBT: alinhamento à direita real requer largura de página — por agora inline.
        if is_numbered {
            // P190F (M6 categoria Counters core): fallback legacy
            // `self.counter.get_flat("equation")` removido. Caminho
            // Introspector único — gate em `from_tags::Equation`
            // (P186E) activado por SetEquationNumbering (P199B);
            // CounterRegistry chave "equation" populated.
            use crate::entities::introspector::Introspector;
            let n = self.current_location
                .and_then(|loc| self.introspector
                    .flat_counter_at("equation", loc))
                .unwrap_or(0);
            self.layout_content(&Content::text(format!("({})", n)));
            self.flush_line();
        }
    }

    /// Braço `Content::Equation` do `layout_content` (atomização ADR-0109
    /// P382): decodifica o gate de numeração da chain e delega a
    /// `layout_equation`. Content-preserving — era inline no `layout_content`.
    pub(super) fn layout_equation_arm(&mut self, e: &EquationElem) {
        // F-5a de-bake (P364, §3a.9): o gate vive **só na chain**; lido
        // de `self.chain`. `layout_equation` mantém `block && numbering`.
        let numbering_active = matches!(
            self.chain.custom("equation.numbering"),
            Some(crate::entities::value::Value::Bool(true)),
        );
        self.layout_equation(&e.body, e.block, numbering_active);
    }

    /// Fallback de nós matemáticos que aparecem **fora** de um `Content::Equation`
    /// (atomização ADR-0109 P382). Normalmente não ocorrem directamente no
    /// layout — o despacho real é `MathLayouter::layout_node`. Se aparecerem,
    /// renderiza como texto. Content-preserving — era inline no `layout_content`.
    pub(super) fn layout_math_fallback(&mut self, content: &Content) {
        let text = content.plain_text();
        for word in text.split_whitespace() {
            self.layout_word(word);
        }
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {
        // V2 smoke test — submódulo extraído no Passo 96.7 (ADR-0037).
        // A cobertura funcional vive em `layout/tests.rs`.
    }
}
