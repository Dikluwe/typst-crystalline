//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/region.md
//! @prompt-hash 2d938d3d
//! @layer L1
//! @updated 2026-05-12
//!
//! **P216A (DEBT-56 sub-fase a parte 1)** — Region: abstracção
//! para área de layout single-column.
//!
//! Introduzida em P216A para agrupar state geométrico previamente
//! disperso no Layouter:
//! - Cursor (x/y/line_start_x).
//! - Buffers (current_items + current_line).
//! - Dimensões (width/height) — duplicadas de `PageConfig` por
//!   redundância controlada per Caminho B1 (P216A C4).
//!
//! Single-region por design em P216A — `Regions` (Vec<Region>)
//! introduzido em P216B; consumer multi-column em P219.
//!
//! Paridade vanilla simplificada per ADR-0078 PROPOSTO:
//! - Sem `expand` axes (cristalino não tem auto-expand explícito).
//! - Sem `full` flag (cristalino infere via cursor_y vs height).
//! - Owned (não borrowed) — vanilla `Regions<'a>` borrow inválido
//!   no contexto cristalino single-pass.
//!
//! Pattern arquitectural "Layouter-state agregado em struct
//! dedicada" — N=2 (precedente `LayouterRuntimeState` P190C).

use crate::entities::layout_types::{FrameItem, Pt};

/// Área de layout single-column.
///
/// **Estado geométrico** previamente disperso em `Layouter`:
/// - Posição corrente do cursor (x/y).
/// - Início horizontal da linha actual (`line_start_x`).
/// - Buffer de items pendentes para a region (`current_items`).
/// - Buffer de items pendentes na linha actual (`current_line`).
/// - Dimensões fixas da region (`width`/`height`).
///
/// P216A apenas reorganiza; P216B introduz `Regions` wrapper;
/// P219 consumer multi-column.
#[derive(Debug, Clone)]
pub struct Region {
    /// Posição horizontal corrente do cursor (Pt newtype).
    pub cursor_x: Pt,
    /// Posição vertical corrente do cursor (Pt newtype).
    pub cursor_y: Pt,
    /// Início horizontal da linha actual (Pt newtype). Após
    /// `flush_line`, `cursor_x` reseta para este valor.
    pub line_start_x: Pt,

    /// Itens já flushed para a region (espera flush_page).
    pub current_items: Vec<FrameItem>,
    /// Itens pendentes na linha actual (esperam flush_line).
    pub current_line: Vec<FrameItem>,

    /// Largura disponível da region (f64; paridade
    /// `PageConfig.width`).
    pub width: f64,
    /// Altura disponível da region (f64; paridade
    /// `PageConfig.height`).
    pub height: f64,
}

impl Region {
    /// Cria region nova com cursor zerado, sem items pendentes.
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            cursor_x: Pt(0.0),
            cursor_y: Pt(0.0),
            line_start_x: Pt(0.0),
            current_items: Vec::new(),
            current_line: Vec::new(),
            width,
            height,
        }
    }

    /// Reseta cursor + buffers para nova page (mantém width/height).
    pub fn reset(&mut self) {
        self.cursor_x = self.line_start_x;
        self.cursor_y = Pt(0.0);
        self.current_items.clear();
        self.current_line.clear();
    }

    /// True se há items pendentes em qualquer buffer.
    pub fn has_pending(&self) -> bool {
        !self.current_items.is_empty() || !self.current_line.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p216a_region_new_inicia_cursor_zero() {
        let r = Region::new(595.28, 841.89);
        assert_eq!(r.cursor_x.0, 0.0);
        assert_eq!(r.cursor_y.0, 0.0);
        assert_eq!(r.line_start_x.0, 0.0);
        assert_eq!(r.width, 595.28);
        assert_eq!(r.height, 841.89);
        assert!(r.current_items.is_empty());
        assert!(r.current_line.is_empty());
    }

    #[test]
    fn p216a_region_reset_preserva_dimensoes() {
        let mut r = Region::new(595.28, 841.89);
        r.cursor_x = Pt(100.0);
        r.cursor_y = Pt(200.0);
        r.line_start_x = Pt(50.0);

        r.reset();

        // Dimensões preservadas.
        assert_eq!(r.width, 595.28);
        assert_eq!(r.height, 841.89);
        // line_start_x preservado; cursor_x reseta para line_start_x.
        assert_eq!(r.line_start_x.0, 50.0);
        assert_eq!(r.cursor_x.0, 50.0);
        assert_eq!(r.cursor_y.0, 0.0);
        // Buffers limpos.
        assert!(r.current_items.is_empty());
        assert!(r.current_line.is_empty());
    }

    #[test]
    fn p216a_region_has_pending_false_apos_new() {
        let r = Region::new(100.0, 100.0);
        assert!(!r.has_pending());
    }

    #[test]
    fn p216a_region_clone_funciona() {
        let r = Region::new(100.0, 200.0);
        let r2 = r.clone();
        assert_eq!(r.width, r2.width);
        assert_eq!(r.height, r2.height);
        assert_eq!(r.cursor_x.0, r2.cursor_x.0);
    }
}
