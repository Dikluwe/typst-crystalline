//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/math/layout/op.md
//! @prompt-hash c41d5aa5
//! @layer L1
//! @updated 2026-07-25
//!
//! Método `layout_op` de `MathLayouter`. Extraído de `math/layout/mod.rs`
//! no Passo 909 (completa o padrão de fatiamento de P314/ADR-0104, deixado
//! para trás por este handler ter sido adicionado depois, em P298).

use crate::engine::layout::FontMetrics;
use crate::entities::{content::Content, layout_types::TextStyle};

use super::MathBox;

impl<'a, M: FontMetrics> super::MathLayouter<'a, M> {
    /// **P298** — Layout op: emite text como math child standard.
    /// Handler trivial (delegate) — verdadeira lógica está em
    /// `layout_attach` que detecta `Content::MathOp { limits: true, .. }`
    /// como base e renderiza scripts em limits-style (ver `op.md`).
    pub(super) fn layout_op(&self, text: &Content, style: &TextStyle) -> MathBox {
        self.layout_node(text, style)
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {}
}
