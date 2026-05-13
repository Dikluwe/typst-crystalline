//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/region.md
//! @prompt-hash 0d428ad0
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

/// Wrapper sobre regions sequenciais.
///
/// **P216B (DEBT-56 sub-fase a parte 2)**: introduzido para
/// preparar consumer multi-column em P219 (sub-fase b).
///
/// Forma **minimal por anti-inflação 11ª aplicação cumulativa
/// pós-P205D** — apenas `current` field. Fields `backlog: Vec<Region>`
/// + `last: Option<Region>` (paridade vanilla literal) **diferidos
/// a P219** quando emergir consumer real (`Content::Columns` arm
/// no Layouter).
///
/// Critério de reabertura `backlog`/`last`: materialização de
/// `Content::Columns` consumer no Layouter (P219). Até lá,
/// single-region é suficiente — `Regions { current: Region }`
/// preserva 100% comportamento P216A.
///
/// Paridade vanilla simplificada per ADR-0078 PROPOSTO §"Decisão":
/// vanilla `Regions<'a> { current, backlog: &'a [Abs], last,
/// expand, full, root, ... }`; cristalino reduz a 1 field até
/// consumer emergir.
///
/// Cohabitação semântica com `Region` no mesmo módulo (precedente:
/// `Sides<T>` em `sides.rs` cobre struct + helpers).
#[derive(Debug, Clone)]
pub struct Regions {
    /// Region actual onde Layouter escreve. Single-region em
    /// P216B; multi-region em P219.
    pub current: Region,
}

impl Regions {
    /// Cria `Regions` com 1 region de dimensões dadas.
    pub fn single(width: f64, height: f64) -> Self {
        Self {
            current: Region::new(width, height),
        }
    }

    /// Reset region actual (delega a `Region::reset`).
    pub fn reset_current(&mut self) {
        self.current.reset();
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

    // ── P216B (DEBT-56 sub-fase a parte 2) — Regions wrapper ────────

    #[test]
    fn p216b_regions_single_cria_current_com_dimensoes() {
        let rs = Regions::single(595.28, 841.89);
        assert_eq!(rs.current.width, 595.28);
        assert_eq!(rs.current.height, 841.89);
        assert_eq!(rs.current.cursor_x.0, 0.0);
        assert_eq!(rs.current.cursor_y.0, 0.0);
        assert!(rs.current.current_items.is_empty());
    }

    #[test]
    fn p216b_regions_reset_current_delega() {
        let mut rs = Regions::single(100.0, 200.0);
        rs.current.cursor_x = Pt(50.0);
        rs.current.cursor_y = Pt(75.0);
        rs.current.line_start_x = Pt(20.0);

        rs.reset_current();

        // Dimensões preservadas; cursor reseta para line_start_x.
        assert_eq!(rs.current.width, 100.0);
        assert_eq!(rs.current.height, 200.0);
        assert_eq!(rs.current.cursor_x.0, 20.0);
        assert_eq!(rs.current.cursor_y.0, 0.0);
    }

    #[test]
    fn p216b_regions_clone_funciona() {
        let rs = Regions::single(100.0, 200.0);
        let rs2 = rs.clone();
        assert_eq!(rs.current.width, rs2.current.width);
        assert_eq!(rs.current.height, rs2.current.height);
    }
}
