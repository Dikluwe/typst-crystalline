//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/layout.md
//! @prompt-hash 67b023a5
//! @layer L1
//! @updated 2026-07-14
//!
//! Interface `FontMetrics` e implementação `FixedMetrics` para layout.
//! Extraído de `layout/mod.rs` no Passo 96.7 conforme ADR-0037.

use crate::entities::{
    glyph_variants::{GlyphAssembly, GlyphVariants, MathGlyphKern},
    layout_types::{FrameItem, Pt, TextEdge, TextStyle},
    math_constants::MathConstants,
};
use unicode_script::{Script, UnicodeScript};

/// Interface de métricas de fonte para o Layouter.
///
/// Minimalista — não armazena `font_size` nem vaza `ttf-parser` para L1.
/// `font_size` é passado em cada chamada para suportar tamanhos mistos
/// (rich text futuro).
pub trait FontMetrics: Send + Sync {
    /// Avanço horizontal de uma string em pontos tipográficos.
    ///
    /// **P544** — o estilo de texto é passado para que implementações com
    /// fallback multi-script (L3) saibam quais fontes primárias resolver.
    fn advance(&self, text: &str, size: Pt, style: &TextStyle) -> Pt;

    /// Métricas verticais: `(ascender, line_height)` em pontos tipográficos.
    ///
    /// - `ascender`: distância da baseline ao topo das maiúsculas.
    /// - `line_height`: distância total entre duas baselines consecutivas.
    ///
    /// **P760** — o `style` é passado para que implementações L3 possam
    /// resolver a fonte correcta, alinhando `line_height` com a face que será
    /// efectivamente usada para renderizar o texto.
    fn vertical_metrics(&self, size: Pt, style: &TextStyle) -> (Pt, Pt);

    /// **P750/P752** — distância da baseline ao topo das maiúsculas
    /// (cap-height).
    ///
    /// Usado para posicionar a primeira baseline do texto a
    /// `margem + cap-height`, paridade com o vanilla (`text(top-edge:
    /// "cap-height")` por omissão). Implementações sem métrica real devem
    /// aproximar proporcionalmente (ex: `size * 0.7`). O `style` é passado
    /// para que implementações L3 possam resolver a fonte correcta (P752).
    fn cap_height(&self, size: Pt, style: &TextStyle) -> Pt;

    /// **P762** — offsets superior e inferior da linha em pontos
    /// tipográficos, conforme `top-edge`/`bottom-edge` do estilo.
    ///
    /// Devolve `(top, bottom)` medidos a partir da baseline. Valores
    /// positivos significam distância para cima (top) ou para baixo
    /// (bottom). `top` é sempre >= 0; `bottom` <= 0 para edges abaixo da
    /// baseline. Implementações devem suportar pelo menos `"baseline"`,
    /// `"cap-height"`, `"ascender"` e `"descender"`.
    fn text_edges(&self, size: Pt, style: &TextStyle) -> (Pt, Pt);

    /// **P813** — limites de tinta ("ink") do texto: `(ascent, descent)`
    /// em pontos, ambos >= 0, medidos da união das bounding boxes reais
    /// dos glyphs. Paridade vanilla: o ascent/descent de um frame math
    /// vem das bboxes dos glyphs, não das métricas globais da fonte.
    ///
    /// Default conservador — cap-height acima e zero abaixo — para
    /// métricas sem acesso a bboxes (`FixedMetrics`, stubs de teste); a
    /// implementação L3 com fonte real sobrescreve com `glyph_index` +
    /// `glyph_bounding_box`.
    fn text_ink_bounds(&self, text: &str, size: Pt, style: &TextStyle) -> (Pt, Pt) {
        let _ = text;
        (self.cap_height(size, style), Pt(0.0))
    }

    /// Constantes da tabela OpenType MATH, se disponível.
    ///
    /// Default: `MathConstants::fallback()` para fontes sem tabela MATH.
    fn math_constants(&self) -> MathConstants {
        MathConstants::fallback()
    }

    /// Variantes de tamanho vertical para um glifo extensível.
    ///
    /// Retorna as variantes ordenadas por tamanho crescente (design units).
    /// Default: sem variantes — fallback para glifo base.
    fn vertical_glyph_variants(&self, c: char) -> GlyphVariants {
        let _ = c;
        GlyphVariants::default()
    }

    /// Mapeamento reverso: glyph_id → char Unicode.
    ///
    /// Necessário para emitir glifos variantes como `FrameItem::Text`.
    /// Default: None — usar glifo base.
    fn glyph_to_char(&self, glyph_id: u16) -> Option<char> {
        let _ = glyph_id;
        None
    }

    /// Montagem por partes para um glifo extensível.
    ///
    /// Retorna as peças ordenadas bottom→top para montagem vertical.
    /// Default: sem assembly — fallback para variante máxima disponível.
    fn vertical_glyph_assembly(&self, c: char) -> GlyphAssembly {
        let _ = c;
        GlyphAssembly::default()
    }

    /// Kern matemático por quadrante para um glifo.
    ///
    /// `c` é o caractere base cujos scripts vão ser posicionados.
    ///
    /// **P891** — `style` foi adicionado para que implementações com
    /// fallback multi-face (`FallbackFontMetrics`, L3) saibam resolver qual
    /// face activa cobre `c` antes de ler a tabela MATH — sem `style`, não
    /// há como decidir a face (ao contrário de `FontBookMetrics`, que tem
    /// uma única face fixa).
    ///
    /// Default: sem kern — todos os quadrantes vazios (espaçamento rectilíneo).
    fn math_kern(&self, c: char, style: &TextStyle) -> MathGlyphKern {
        let _ = (c, style);
        MathGlyphKern::default()
    }

    /// **P591** — avanço horizontal já com forma de escrita aplicada
    /// (shaping). Implementações L3 podem devolver `Some(width)` para
    /// scripts que alteram a forma dos caracteres em contexto (árabe,
    /// síriaco, etc.); `None` mantém o caminho normal `advance`.
    fn advance_shaped(&self, text: &str, size: Pt, style: &TextStyle) -> Option<Pt> {
        let _ = (text, size, style);
        None
    }

    /// **P593** — largura de uma palavra/trecho de texto, incluindo tracking.
    /// Fonte única de verdade para o nível "palavra": usa `advance_shaped`
    /// quando disponível (scripts contextuais) e cai em `advance` para os
    /// restantes, aplicando tracking de forma consistente.
    fn text_width(&self, text: &str, size: Pt, style: &TextStyle) -> Pt {
        let base = self
            .advance_shaped(text, size, style)
            .map(|p| p.val())
            .unwrap_or_else(|| self.advance(text, size, style).val());
        let tracking_extra = style
            .tracking
            .map(|t| {
                let tracking_pt = t.resolve_pt(size.val());
                let n = text.chars().count();
                tracking_pt * n.saturating_sub(1) as f64
            })
            .unwrap_or(0.0);
        Pt(base + tracking_extra)
    }

    /// **P593** — limite direito real de uma linha, calculado a partir das
    /// bounding boxes dos items desenhados. Fonte única de verdade para o
    /// nível "linha": usa `text_width` para texto e os campos naturais dos
    /// outros `FrameItem`s.
    fn line_content_right(&self, items: &[&FrameItem]) -> f64 {
        items
            .iter()
            .copied()
            .map(|item| {
                let x = match item {
                    FrameItem::Text { pos, .. }
                    | FrameItem::TextShaped { pos, .. }
                    | FrameItem::Glyph { pos, .. }
                    | FrameItem::Image { pos, .. }
                    | FrameItem::Shape { pos, .. }
                    | FrameItem::Group { pos, .. }
                    | FrameItem::Link { pos, .. } => pos.x.0,
                    FrameItem::Line { start, .. } => start.x.0,
                };
                let w = match item {
                    FrameItem::Text { text, style, .. } => {
                        self.text_width(text.as_str(), style.size, style).0
                    }
                    FrameItem::TextShaped { glyphs, style, units_per_em, .. } => {
                        let size_pt = style.size.val();
                        let upem = *units_per_em as f64;
                        glyphs.iter().map(|g| g.x_advance as f64 / upem * size_pt).sum()
                    }
                    FrameItem::Line { start, end, .. } => (end.x.0 - start.x.0).abs(),
                    FrameItem::Glyph { x_advance, .. } => x_advance.0,
                    FrameItem::Image { width, .. } => width.0,
                    FrameItem::Shape { width, .. } => *width,
                    FrameItem::Group { inner_width, .. } => *inner_width,
                    FrameItem::Link { size, .. } => size.width.0,
                };
                x + w
            })
            .fold(0.0, f64::max)
    }
}

/// **P591 / P623** — detecta se um trecho de texto precisa de medição com
/// shaping porque pertence a um script com formas contextuais obrigatórias
/// (ligaduras ou conjuntas que reduzem a largura total face à soma de
/// advances isolados).
pub fn needs_shaped_width(text: &str) -> bool {
    text.chars().any(|c| {
        matches!(
            c.script(),
            Script::Arabic
                | Script::Syriac
                | Script::Mongolian
                | Script::Nko
                | Script::Mandaic
                | Script::Devanagari
        )
    })
}

/// Métricas fixas monoespaçadas — para layout sem FontBook real.
///
/// Passo 21: substituída por `FontBookMetrics` em L3 quando disponível.
/// **P544**: `Clone + Copy` para poder ser reutilizada em múltiplos
/// Layouters no fixpoint loop sem dependências externas.
#[derive(Clone, Copy)]
pub struct FixedMetrics;

impl FontMetrics for FixedMetrics {
    fn advance(&self, text: &str, size: Pt, _style: &TextStyle) -> Pt {
        // 0.6 * size por codepoint — monoespaçado
        size * (text.chars().count() as f64 * 0.6)
    }

    fn vertical_metrics(&self, size: Pt, _style: &TextStyle) -> (Pt, Pt) {
        // ascender ≈ 0.8 * size; line_height = 1.2 * size
        (size * 0.8, size * 1.2)
    }

    fn cap_height(&self, size: Pt, _style: &TextStyle) -> Pt {
        // **P750/P752** — aproximação proporcional consistente com a razão
        // típica cap-height/em; usada apenas quando não há fonte real.
        size * 0.7
    }

    fn text_edges(&self, size: Pt, style: &TextStyle) -> (Pt, Pt) {
        // **P762** — aproximações proporcionais para métricas fixas.
        // **P837** — `Length` explícito resolve a partir da baseline no
        // font-size (paridade `FontInstance::edges` do vanilla:
        // `top = length.at(size)`; bottom negativo = abaixo da baseline).
        let top = match style.top_edge.as_ref() {
            Some(TextEdge::Length(l)) => Pt(l.resolve_pt(size.val())),
            Some(TextEdge::Metric(m)) => match m.as_str() {
                "baseline" => Pt(0.0),
                "x-height" => size * 0.5,
                "ascender" => size * 0.8,
                _ => size * 0.7, // "cap-height" e default vanilla
            },
            None => size * 0.7, // default vanilla = cap-height
        };
        let bottom = match style.bottom_edge.as_ref() {
            Some(TextEdge::Length(l)) => Pt(l.resolve_pt(size.val())),
            Some(TextEdge::Metric(m)) => match m.as_str() {
                "descender" => size * -0.2,
                _ => Pt(0.0), // "baseline" e default vanilla
            },
            None => Pt(0.0),
        };
        (top, bottom)
    }
}

/// **P858** — permite passar uma referência a um trait object `FontMetrics`
/// (ex: `&dyn FontMetrics` vindo do `Engine`) para o `Layouter`, que espera
/// um tipo genérico `M: FontMetrics`. Delega cada método obrigatório para o
/// trait object subjacente; os métodos com default usam esses quatro.
impl FontMetrics for &dyn FontMetrics {
    fn advance(&self, text: &str, size: Pt, style: &TextStyle) -> Pt {
        (*self).advance(text, size, style)
    }

    fn vertical_metrics(&self, size: Pt, style: &TextStyle) -> (Pt, Pt) {
        (*self).vertical_metrics(size, style)
    }

    fn cap_height(&self, size: Pt, style: &TextStyle) -> Pt {
        (*self).cap_height(size, style)
    }

    fn text_edges(&self, size: Pt, style: &TextStyle) -> (Pt, Pt) {
        (*self).text_edges(size, style)
    }
}

#[cfg(test)]
mod smoke {
    use super::needs_shaped_width;

    #[test]
    fn module_compila_e_carrega() {
        // V2 smoke test — submódulo extraído no Passo 96.7 (ADR-0037).
        // A cobertura funcional vive em `layout/tests.rs`.
    }

    /// P623 — devanágari denso em conjuntas precisa de shaping na medição.
    #[test]
    fn devanagari_precisa_shaped_width() {
        assert!(needs_shaped_width("धर्मक्षेत्रे"));
        assert!(needs_shaped_width("कुरुक्षेत्रे"));
        assert!(needs_shaped_width("नमस्ते"));
    }

    /// P623 — latim continua no caminho rápido.
    #[test]
    fn latim_nao_precisa_shaped_width() {
        assert!(!needs_shaped_width("Hello world"));
    }

    /// P591 — scripts contextuais já conhecidos continuam marcados.
    #[test]
    fn arabic_precisa_shaped_width() {
        assert!(needs_shaped_width("الكتاب"));
    }
}
