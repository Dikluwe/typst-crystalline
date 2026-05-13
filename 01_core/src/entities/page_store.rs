//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/page_store.md
//! @prompt-hash 47a8d343
//! @layer L1
//! @updated 2026-05-12
//!
//! **P207D (M9c)** — Sub-store sealed para metadata page-level per
//! ADR-0076 (PROPOSTO 2026-05-12; Bloco II + Bloco VIII parcial).
//!
//! Pattern arquitectural reusa literal P205B/C (`SealedPositions` +
//! `inject_positions`): sub-store immutable construído pós-layout,
//! injectado em `TagIntrospector` via `inject_pages`. Pre-injecção,
//! todos os queries retornam `None`/`0`.
//!
//! Cristalino diverge intencionalmente do vanilla
//! (`PagedIntrospector::new(&pages)` pre-computa numberings +
//! supplements globais; cristalino faz sealing por sub-store, per
//! P205A.div-1 + P205B). Numbering é `EcoString` pattern (não enum
//! vanilla `Numbering`) per ADR-0024.

use std::num::NonZeroUsize;

use ecow::EcoString;

use crate::entities::content::Content;

/// Sub-store sealed para metadata page-level.
///
/// Pre-injecção (`empty`), `total_pages == None` e queries
/// retornam `None`. Pós-injecção via `from_total_pages` (P207D
/// minimal) ou `from_runtime` (futuro), queries page-aware
/// resolvem.
#[derive(Debug, Clone, Default)]
pub struct PageStore {
    total_pages: Option<NonZeroUsize>,
    numberings:  Vec<Option<EcoString>>,
    supplements: Vec<Content>,
}

impl PageStore {
    /// Construtor vazio. Equivalente a `Default::default()`.
    pub fn empty() -> Self {
        Self::default()
    }

    /// **P207D minimal** — Construtor com apenas `total_pages`
    /// populado; `numberings` e `supplements` vazios. Usado por
    /// `Layouter::finish` enquanto a captura de numbering+
    /// supplement no walk de layout não está materializada
    /// (deferred a passo futuro per Bloco VIII).
    pub fn from_total_pages(total: NonZeroUsize) -> Self {
        Self {
            total_pages: Some(total),
            numberings:  Vec::new(),
            supplements: Vec::new(),
        }
    }

    /// Construtor completo. `numberings.len()` e `supplements.len()`
    /// devem ser `total.get()` (1 entrada por página, 1-based
    /// indexada como `page.get() - 1`). Construtor não valida —
    /// caller responsável.
    pub fn from_runtime(
        total:       NonZeroUsize,
        numberings:  Vec<Option<EcoString>>,
        supplements: Vec<Content>,
    ) -> Self {
        Self {
            total_pages: Some(total),
            numberings,
            supplements,
        }
    }

    /// Total de páginas, ou `None` pre-injecção.
    pub fn total_pages(&self) -> Option<NonZeroUsize> {
        self.total_pages
    }

    /// Numbering pattern para `page` (1-based).
    ///
    /// Devolve `None` quando:
    /// - Pre-injecção (`is_empty()`).
    /// - `page.get() > numberings.len()` (fora de range, ex.
    ///   construtor minimal `from_total_pages`).
    /// - Página tem `None` numbering (sem pattern atribuído).
    pub fn numbering_for_page(&self, page: NonZeroUsize)
        -> Option<&EcoString>
    {
        self.numberings.get(page.get() - 1).and_then(|slot| slot.as_ref())
    }

    /// Supplement para `page` (1-based).
    ///
    /// Devolve `None` quando pre-injecção, fora de range, ou
    /// sem supplement capturado. Distingue "sem supplement"
    /// de "supplement vazio" (vanilla colapsa ambos via
    /// `Content::empty()`).
    pub fn supplement_for_page(&self, page: NonZeroUsize)
        -> Option<&Content>
    {
        self.supplements.get(page.get() - 1)
    }

    /// True se pre-injecção (sem `total_pages` definido). Não
    /// reflecte se `numberings`/`supplements` foram populados.
    pub fn is_empty(&self) -> bool {
        self.total_pages.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn nz(n: usize) -> NonZeroUsize {
        NonZeroUsize::new(n).unwrap()
    }

    // ── P207D Sentinelas ─────────────────────────────────────────────

    #[test]
    fn p207d_page_store_struct_existe() {
        // Sentinel: confirma que construtores existem. Falha de
        // compilação se removidos.
        let _empty: PageStore = PageStore::empty();
        let _minimal: PageStore = PageStore::from_total_pages(nz(1));
        let _full: PageStore = PageStore::from_runtime(
            nz(1), Vec::new(), Vec::new(),
        );
    }

    // ── Tests de unidade ─────────────────────────────────────────────

    #[test]
    fn empty_devolve_none_em_todos_os_queries() {
        let s = PageStore::empty();
        assert_eq!(s.total_pages(), None);
        assert_eq!(s.numbering_for_page(nz(1)), None);
        assert_eq!(s.supplement_for_page(nz(1)), None);
        assert!(s.is_empty());
    }

    #[test]
    fn from_total_pages_preserva_total_mas_vecs_vazios() {
        let s = PageStore::from_total_pages(nz(5));
        assert_eq!(s.total_pages(), Some(nz(5)));
        assert!(!s.is_empty());
        // Vecs vazios → todos os lookups None.
        assert_eq!(s.numbering_for_page(nz(1)), None);
        assert_eq!(s.numbering_for_page(nz(5)), None);
        assert_eq!(s.supplement_for_page(nz(1)), None);
    }

    #[test]
    fn from_runtime_resolve_queries_por_pagina() {
        let numberings = vec![
            Some(EcoString::from("1")),  // page 1
            None,                          // page 2 sem numbering
            Some(EcoString::from("I")),  // page 3
        ];
        let supplements = vec![
            Content::Empty,
            Content::Empty,
            Content::Empty,
        ];
        let s = PageStore::from_runtime(nz(3), numberings, supplements);

        assert_eq!(s.total_pages(), Some(nz(3)));
        assert_eq!(
            s.numbering_for_page(nz(1)).map(|e| e.as_str()),
            Some("1"),
        );
        assert_eq!(s.numbering_for_page(nz(2)), None);
        assert_eq!(
            s.numbering_for_page(nz(3)).map(|e| e.as_str()),
            Some("I"),
        );
        // Supplement.
        assert!(s.supplement_for_page(nz(1)).is_some());
        assert!(s.supplement_for_page(nz(2)).is_some());
        assert!(s.supplement_for_page(nz(3)).is_some());
    }

    #[test]
    fn fora_de_range_devolve_none_sem_panic() {
        let s = PageStore::from_runtime(
            nz(2),
            vec![Some(EcoString::from("1")), Some(EcoString::from("2"))],
            vec![Content::Empty, Content::Empty],
        );
        // Page 3 não existe (total = 2).
        assert_eq!(s.numbering_for_page(nz(3)), None);
        assert_eq!(s.supplement_for_page(nz(3)), None);
        // Page 100 idem.
        assert_eq!(s.numbering_for_page(nz(100)), None);
        assert_eq!(s.supplement_for_page(nz(100)), None);
    }
}
