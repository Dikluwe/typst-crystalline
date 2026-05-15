//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/region.md
//! @prompt-hash c5527e12
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
/// **P243 (M9d / M7+3 fase (a); ADR-0081 IMPLEMENTADO parcial 4/5)**
/// — extensão face P216B: introduz `backlog: Vec<Region>` + `last:
/// Option<Region>` fields (paridade vanilla literal) para preparar
/// consumer multi-region/multi-column. Fase (a) DEBT-56 §"Notas":
/// "introduzir `Regions { current, backlog, last }` mantendo
/// comportamento single-region". Single-region preservado literal
/// — `backlog` vazio + `last: None` em produção P243; populated
/// só em fase (b) DEBT-56 quando `Content::Columns` materializar.
///
/// Paridade vanilla simplificada per ADR-0078 PROPOSTO §"Decisão":
/// vanilla `Regions<'a> { current, backlog: &'a [Abs], last,
/// expand, full, root, ... }`; cristalino preserva subset
/// essencial (`backlog: Vec<Region>` owned + `last: Option<Region>`
/// owned), omitindo `expand`/`full`/`root` (semantic adiada per
/// ADR-0054 — cristalino não tem auto-expand explícito).
///
/// **Critério de activação `backlog`/`last`**: materialização de
/// `Content::Columns` consumer no Layouter (fase (b) DEBT-56).
/// Até lá, fase (a) P243 preserva 100% comportamento P216A/P216B
/// observable (`backlog` vazio).
///
/// Cohabitação semântica com `Region` no mesmo módulo (precedente:
/// `Sides<T>` em `sides.rs` cobre struct + helpers).
#[derive(Debug, Clone)]
pub struct Regions {
    /// Region actual onde Layouter escreve. Single-region em
    /// P216B; multi-region em fase (b) DEBT-56.
    pub current: Region,
    /// **P243 (M9d / M7+3 fase (a))** — regions ainda não consumidas.
    /// Vazio em fase (a) (single-region preservado); populated em
    /// fase (b) por `Content::Columns` arm.
    pub backlog: Vec<Region>,
    /// **P243 (M9d / M7+3 fase (a))** — última region preservada para
    /// overflow/fallback (e.g. medida final). `None` em fase (a);
    /// populated em fase (b) conforme columns/colbreak consumir.
    pub last: Option<Region>,
    /// **P246 (cell layout migration; activa A.4 breakable per-cell
    /// arquiteturalmente)** — cell region transient. `Some(r)` quando
    /// Layouter está dentro de célula Grid/TableCell; `None` em flow
    /// regular da página. Substitui campos Layouter `cell_available_h`
    /// + `cell_origin_w` (geometria) — `cell_origin_x` + `cell_origin_y`
    /// preservados como Layouter fields legacy (Region sem
    /// `origin: Point`; refactor futuro per DEBT opcional).
    ///
    /// Reader pattern: `regions.effective().height` retorna cell.height
    /// se activa, senão current.height (paridade semantic anterior
    /// `cell_available_h.unwrap_or(page_h)`).
    pub cell: Option<Region>,
}

impl Regions {
    /// Cria `Regions` com 1 region de dimensões dadas. `backlog`
    /// vazio; `last: None` (fase (a) P243 preserva semantic single-
    /// region P216B literal). `cell: None` (P246 — fora célula).
    pub fn single(width: f64, height: f64) -> Self {
        Self {
            current: Region::new(width, height),
            backlog: Vec::new(),
            last:    None,
            cell:    None,
        }
    }

    /// Reset region actual (delega a `Region::reset`).
    pub fn reset_current(&mut self) {
        self.current.reset();
    }

    /// **P243 (M9d / M7+3 fase (a))** — avança para próxima region.
    ///
    /// Comportamento conforme estado `backlog`:
    /// - **`backlog` não-vazio**: move `current` para `last`; consome
    ///   primeira region do `backlog` como novo `current`. Retorna
    ///   `Some(old_current)` (o caller pode commit para Page se
    ///   necessário).
    /// - **`backlog` vazio (fase (a))**: retorna `None` — caller
    ///   deve criar nova region externa (e.g. `new_page` no
    ///   Layouter). Preserva semantic P216B single-region literal.
    ///
    /// **Fase (a) P243**: callers existentes (`flush_line`,
    /// `new_page`) preservam fluxo P216A/B; método disponível
    /// apenas como infraestrutura para fase (b).
    pub fn advance(&mut self) -> Option<Region> {
        if self.backlog.is_empty() {
            // Fase (a): sem backlog. Caller cria nova region externa.
            return None;
        }
        // Fase (b): consome próxima do backlog.
        let next = self.backlog.remove(0);
        let prev = std::mem::replace(&mut self.current, next);
        self.last = Some(prev.clone());
        Some(prev)
    }

    /// **P246 (cell layout migration)** — region efectiva conforme
    /// estado activo. Retorna `cell` se Layouter está dentro de
    /// célula Grid/TableCell; `current` caso contrário. Paridade
    /// semantic literal do pre-P246 pattern
    /// `cell_available_h.unwrap_or(page_h)`.
    pub fn effective(&self) -> &Region {
        self.cell.as_ref().unwrap_or(&self.current)
    }

    /// **P246 (cell layout migration)** — entra célula com `region`
    /// dada (width = column width; height = row height). Retorna
    /// `Option<Region>` (saved) que caller passa a `exit_cell` ao
    /// sair da célula. Suporta aninhamento Grid-in-Grid.
    pub fn enter_cell(&mut self, cell: Region) -> Option<Region> {
        std::mem::replace(&mut self.cell, Some(cell))
    }

    /// **P246 (cell layout migration)** — sai célula restaurando o
    /// `saved` retornado pelo `enter_cell` correspondente. Quando
    /// sai célula top-level, `saved` é `None` → `cell` volta a
    /// `None`. Suporta aninhamento.
    pub fn exit_cell(&mut self, saved: Option<Region>) {
        self.cell = saved;
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

    // ── P243 (M9d / M7+3 fase (a); ADR-0081 IMPLEMENTADO parcial 4/5)
    //     — Regions extends with backlog + last + advance ──

    #[test]
    fn p243_regions_single_backlog_vazio_last_none() {
        // Fase (a): single-region preservado literal P216B.
        let rs = Regions::single(595.28, 841.89);
        assert!(rs.backlog.is_empty(), "fase (a) inicia com backlog vazio");
        assert!(rs.last.is_none(),     "fase (a) inicia com last=None");
        assert_eq!(rs.current.width,  595.28);
        assert_eq!(rs.current.height, 841.89);
    }

    #[test]
    fn p243_regions_advance_fase_a_retorna_none() {
        // Fase (a): sem backlog → advance retorna None.
        // Caller (Layouter::new_page) cria nova region externa.
        let mut rs = Regions::single(100.0, 200.0);
        rs.current.cursor_x = Pt(50.0);
        let result = rs.advance();
        assert!(result.is_none(),
            "fase (a) advance retorna None (backlog vazio)");
        // Estado preservado.
        assert_eq!(rs.current.cursor_x.0, 50.0);
        assert!(rs.last.is_none());
    }

    #[test]
    fn p243_regions_advance_fase_b_consome_backlog() {
        // Fase (b) simulada: backlog populated; advance move
        // current→last + consome próximo do backlog.
        let mut rs = Regions::single(100.0, 200.0);
        rs.current.cursor_x = Pt(50.0);
        // Simular fase (b): adicionar region ao backlog.
        let next = Region::new(80.0, 200.0);   // coluna mais estreita.
        rs.backlog.push(next);

        let prev = rs.advance();
        assert!(prev.is_some(), "fase (b) advance retorna prev");
        assert_eq!(prev.unwrap().width, 100.0);
        // current agora é a próxima coluna.
        assert_eq!(rs.current.width, 80.0);
        // last guarda a anterior.
        assert!(rs.last.is_some());
        assert_eq!(rs.last.as_ref().unwrap().width, 100.0);
        // Backlog consumido.
        assert!(rs.backlog.is_empty());
    }

    #[test]
    fn p243_regions_clone_preserva_backlog_last() {
        let mut rs = Regions::single(100.0, 200.0);
        rs.backlog.push(Region::new(50.0, 200.0));
        rs.last = Some(Region::new(70.0, 200.0));
        let rs2 = rs.clone();
        assert_eq!(rs2.backlog.len(), 1);
        assert!(rs2.last.is_some());
        assert_eq!(rs2.backlog[0].width, 50.0);
        assert_eq!(rs2.last.as_ref().unwrap().width, 70.0);
    }

    // ── P246 (cell layout migration; activa A.4 breakable per-cell
    //     arquiteturalmente) — Regions.cell + effective/enter/exit ──

    #[test]
    fn p246_regions_single_cell_none_inicial() {
        let rs = Regions::single(100.0, 200.0);
        assert!(rs.cell.is_none(),
            "fase inicial: cell=None (fora célula)");
    }

    #[test]
    fn p246_regions_effective_sem_cell_retorna_current() {
        let rs = Regions::single(100.0, 200.0);
        assert_eq!(rs.effective().width,  100.0);
        assert_eq!(rs.effective().height, 200.0);
    }

    #[test]
    fn p246_regions_effective_com_cell_retorna_cell() {
        let mut rs = Regions::single(100.0, 200.0);
        rs.cell = Some(Region::new(50.0, 75.0));
        assert_eq!(rs.effective().width,  50.0);
        assert_eq!(rs.effective().height, 75.0);
    }

    #[test]
    fn p246_regions_enter_exit_cell_top_level() {
        // Entra célula top-level; saved=None; exit restaura cell=None.
        let mut rs = Regions::single(100.0, 200.0);
        let saved = rs.enter_cell(Region::new(60.0, 80.0));
        assert!(saved.is_none(), "top-level enter: saved=None");
        assert!(rs.cell.is_some());
        assert_eq!(rs.cell.as_ref().unwrap().width, 60.0);
        rs.exit_cell(saved);
        assert!(rs.cell.is_none(),
            "exit top-level: cell volta a None");
    }

    #[test]
    fn p246_regions_enter_exit_cell_aninhado() {
        // Aninhamento: enter outer; enter inner; exit inner restaura
        // outer; exit outer restaura None.
        let mut rs = Regions::single(100.0, 200.0);
        let saved_outer = rs.enter_cell(Region::new(80.0, 100.0));
        assert!(saved_outer.is_none());
        let saved_inner = rs.enter_cell(Region::new(40.0, 50.0));
        assert!(saved_inner.is_some());
        assert_eq!(saved_inner.as_ref().unwrap().width, 80.0);
        assert_eq!(rs.cell.as_ref().unwrap().width, 40.0);

        rs.exit_cell(saved_inner);
        // De volta à outer.
        assert_eq!(rs.cell.as_ref().unwrap().width, 80.0);

        rs.exit_cell(saved_outer);
        assert!(rs.cell.is_none());
    }

    #[test]
    fn p246_regions_clone_preserva_cell() {
        let mut rs = Regions::single(100.0, 200.0);
        rs.cell = Some(Region::new(50.0, 75.0));
        let rs2 = rs.clone();
        assert!(rs2.cell.is_some());
        assert_eq!(rs2.cell.as_ref().unwrap().width, 50.0);
    }
}
