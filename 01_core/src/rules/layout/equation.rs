//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout/equation.md
//! @prompt-hash c43d6d75
//! @layer L1
//! @updated 2026-04-23
//!
//! Braço `Content::Equation` do `layout_content`. Extraído de `layout/mod.rs`
//! no Passo 96.7 conforme ADR-0037. P456: numeração de bloco formatada pelo
//! pattern da chain e posicionada à direita da página.
#![allow(deprecated)] // P483 — FrameItem::Text fallback path legítimo
use crate::entities::{
    content::Content,
    counter_format::format_counter,
    elements::equation::EquationElem,
    image_sizer::ImageSizer,
    layout_types::{FrameItem, Point, Pt},
};
use crate::rules::math;

use super::metrics::FontMetrics;

impl<'a, M: FontMetrics, S: ImageSizer> super::Layouter<'a, M, S> {
    /// Layout de `Content::Equation { body, block }`.
    /// `numbering_pattern`: pattern de numeração vindo da chain (`None` se
    /// a equação não deve ser numerada).
    pub(super) fn layout_equation(
        &mut self,
        body: &Content,
        block: bool,
        numbering_pattern: Option<&str>,
    ) {
        // **P751** — fixar a baseline inicial com o estilo activo antes de
        // posicionar texto/equação real.
        self.ensure_initial_baseline();
        // Auto-numeração: equações de bloco numeradas avançam o contador antes de
        // desenhar (Passo 59). O número (N) é acrescentado depois da equação.
        // Lote F-2 S2 (P335): o "ativo" é **assado** no `EquationElem` (escopo
        // léxico via chain, fecha o canal global StateRegistry). O **valor** do
        // contador continua via Introspector (`flat_counter_at`), gateado pelo
        // mesmo `numbering_active` assado (via payload, em `from_tags`).
        // P456: o pattern vem da chain como `Value::Str`; ausência = não numerada.
        let is_numbered = block && numbering_pattern.is_some();
        // P190F (M6 categoria Counters core): Layouter mutação
        // `self.counter.step_flat` removida — counter equation
        // populated via Introspector path (CounterRegistry +
        // gate em `from_tags` arm Equation P186E activado por
        // SetEquationNumbering P199B). Layouter só lê.

        let math_layouter = math::layout::MathLayouter::new(&self.metrics, block);
        let math_items = math_layouter.layout_equation(body, &self.style);

        if block && self.regions.current.cursor_x.0 > self.page_config.margin {
            self.flush_line();
        }

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
                    let advance = self.metrics.advance(&text, style.size, &style);
                    self.regions.current.current_line.push(FrameItem::Text {
                        pos: abs_pos,
                        text,
                        style,
                    });
                    self.regions.current.cursor_x += advance;
                }
                FrameItem::Line { start, end, thickness, color } => {
                    let abs_start = Point {
                        x: offset_x + start.x,
                        y: offset_y + start.y,
                    };
                    let abs_end = Point {
                        x: offset_x + end.x,
                        y: offset_y + end.y,
                    };
                    self.regions.current.current_line.push(FrameItem::Line {
                        start: abs_start,
                        end: abs_end,
                        thickness,
                        // P285: equação preserva cor da Line original (math
                        // frac/sqrt usam None → preto bit-exact).
                        color,
                    });
                }
                FrameItem::TextShaped { .. } => {} // TextShaped não ocorre antes do shaper em math inline
                FrameItem::Image { .. } => {} // imagens não ocorrem em math inline
                FrameItem::Shape { .. } => {} // formas não ocorrem em math inline
                FrameItem::Group { .. } => {} // grupos não ocorrem em math inline
                FrameItem::Link { .. } => {}  // links não ocorrem em math inline
                FrameItem::Glyph { pos, glyph_id, x_advance, size } => {
                    let abs_pos = Point {
                        x: offset_x + pos.x,
                        y: offset_y + pos.y,
                    };
                    self.regions.current.current_line.push(FrameItem::Glyph {
                        pos: abs_pos,
                        glyph_id,
                        x_advance,
                        size,
                    });
                    self.regions.current.cursor_x += x_advance;
                }
            }
        }

        // Guardar a baseline da linha antes do flush; usada para posicionar o
        // número à direita verticalmente alinhado com a equação.
        let equation_baseline_y = self.regions.current.cursor_y;

        if block {
            self.flush_line();
        }

        // Acrescentar número da equação à direita da página (P456).
        if is_numbered {
            // P190F (M6 categoria Counters core): fallback legacy
            // `self.counter.get_flat("equation")` removido. Caminho
            // Introspector único — gate em `from_tags::Equation`
            // (P186E) activado por SetEquationNumbering (P199B);
            // CounterRegistry chave "equation" populated.
            use crate::entities::introspector::Introspector;
            let n = self
                .current_location
                .and_then(|loc| self.introspector.flat_counter_at("equation", loc))
                .unwrap_or(0);
            let pattern = numbering_pattern.unwrap_or("(1)");
            let formatted = format_counter(&[n], pattern)
                .unwrap_or_else(|| n.to_string());

            let number_text: ecow::EcoString = formatted.into();
            let number_width = self.metrics.advance(&number_text, self.style.size, &self.style);
            let right_x =
                Pt(self.regions.current.width - self.page_config.margin) - number_width;

            self.regions.current.current_items.push(FrameItem::Text {
                pos: Point {
                    x: right_x,
                    y: equation_baseline_y,
                },
                text: number_text,
                style: self.style.clone(),
            });
        }
    }

    /// Braço `Content::Equation` do `layout_content` (atomização ADR-0109
    /// P382): decodifica o gate de numeração da chain e delega a
    /// `layout_equation`. Content-preserving — era inline no `layout_content`.
    pub(super) fn layout_equation_arm(&mut self, e: &EquationElem) {
        // F-5a de-bake (P364, §3a.9): o gate vive **só na chain**; lido
        // de `self.chain`. P456: pattern é `Value::Str` (análogo a
        // figure.numbering em P454).
        let numbering_pattern: Option<String> = self
            .chain
            .custom("equation.numbering")
            .and_then(|v| match v {
                crate::entities::value::Value::Str(s) => Some(s.to_string()),
                _ => None,
            });
        let pat = numbering_pattern.as_deref();
        self.layout_equation(&e.body, e.block, pat);
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
