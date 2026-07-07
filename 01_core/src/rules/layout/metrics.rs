//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout.md
//! @prompt-hash 5249700d
//! @layer L1
//! @updated 2026-04-23
//!
//! Interface `FontMetrics` e implementação `FixedMetrics` para layout.
//! Extraído de `layout/mod.rs` no Passo 96.7 conforme ADR-0037.

use crate::entities::{
    glyph_variants::{GlyphAssembly, GlyphVariants, MathGlyphKern},
    layout_types::{Pt, TextStyle},
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
    fn vertical_metrics(&self, size: Pt) -> (Pt, Pt);

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
    /// Default: sem kern — todos os quadrantes vazios (espaçamento rectilíneo).
    fn math_kern(&self, c: char) -> MathGlyphKern {
        let _ = c;
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
}

/// **P591** — detecta se um trecho de texto precisa de medição com shaping
/// porque pertence a um script com formas contextuais obrigatórias.
pub fn needs_shaped_width(text: &str) -> bool {
    text.chars().any(|c| {
        matches!(
            c.script(),
            Script::Arabic
                | Script::Syriac
                | Script::Mongolian
                | Script::Nko
                | Script::Mandaic
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

    fn vertical_metrics(&self, size: Pt) -> (Pt, Pt) {
        // ascender ≈ 0.8 * size; line_height = 1.2 * size
        (size * 0.8, size * 1.2)
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
