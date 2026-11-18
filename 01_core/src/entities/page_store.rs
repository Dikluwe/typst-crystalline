//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/page_store.md
//! @prompt-hash aca28b75
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

use crate::entities::content::Content;
use crate::entities::numbering::Numbering;
use crate::entities::span::Span;

/// Sub-store sealed para metadata page-level.
///
/// Pre-injecção (`empty`), `total_pages == None` e queries
/// retornam `None`. Pós-injecção via `from_total_pages` (P207D
/// minimal) ou `from_runtime` (futuro), queries page-aware
/// resolvem.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PageStore {
    total_pages: Option<NonZeroUsize>,
    numberings: Vec<Option<Numbering>>,
    logical_numbers: Vec<usize>,
    numbering_spans: Vec<Span>,
    visible_numberings: Vec<Option<Content>>,
    reference_numberings: Vec<Option<Content>>,
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
            numberings: Vec::new(),
            logical_numbers: Vec::new(),
            numbering_spans: Vec::new(),
            visible_numberings: Vec::new(),
            reference_numberings: Vec::new(),
            supplements: Vec::new(),
        }
    }

    /// Construtor completo. `numberings.len()` e `supplements.len()`
    /// devem ser `total.get()` (1 entrada por página, 1-based
    /// indexada como `page.get() - 1`). Construtor não valida —
    /// caller responsável.
    pub fn from_runtime(
        total: NonZeroUsize,
        numberings: Vec<Option<Numbering>>,
        supplements: Vec<Content>,
    ) -> Self {
        Self {
            total_pages: Some(total),
            logical_numbers: (1..=total.get()).collect(),
            numbering_spans: vec![Span::detached(); total.get()],
            visible_numberings: vec![None; total.get()],
            reference_numberings: vec![None; total.get()],
            numberings,
            supplements,
        }
    }

    /// Constrói o snapshot completo, depois de callbacks terem sido
    /// realizados pelo pipeline L3.
    #[allow(clippy::too_many_arguments)]
    pub fn from_realized(
        total: NonZeroUsize,
        numberings: Vec<Option<Numbering>>,
        logical_numbers: Vec<usize>,
        numbering_spans: Vec<Span>,
        visible_numberings: Vec<Option<Content>>,
        reference_numberings: Vec<Option<Content>>,
        supplements: Vec<Content>,
    ) -> Self {
        debug_assert!(numberings.len() == total.get());
        debug_assert!(logical_numbers.len() == total.get());
        debug_assert!(numbering_spans.len() == total.get());
        debug_assert!(visible_numberings.len() == total.get());
        debug_assert!(reference_numberings.len() == total.get());
        debug_assert!(supplements.len() == total.get());
        Self {
            total_pages: Some(total),
            numberings,
            logical_numbers,
            numbering_spans,
            visible_numberings,
            reference_numberings,
            supplements,
        }
    }

    /// Internal coverage proof; absent optional values still require a real slot.
    pub(crate) fn contains_complete_interval(
        &self,
        first: NonZeroUsize,
        total: NonZeroUsize,
    ) -> bool {
        let Some(last) = first.get().checked_add(total.get() - 1) else {
            return false;
        };
        self.total_pages.is_some_and(|available| last <= available.get())
            && self.numberings.len() >= last
            && self.logical_numbers.len() >= last
            && self.numbering_spans.len() >= last
            && self.visible_numberings.len() >= last
            && self.reference_numberings.len() >= last
            && self.supplements.len() >= last
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
    pub fn numbering_for_page(&self, page: NonZeroUsize) -> Option<&Numbering> {
        self.numberings.get(page.get() - 1).and_then(|slot| slot.as_ref())
    }

    pub fn logical_number_for_page(&self, page: NonZeroUsize) -> Option<usize> {
        self.logical_numbers.get(page.get() - 1).copied()
    }

    pub fn numbering_span_for_page(&self, page: NonZeroUsize) -> Option<Span> {
        self.numbering_spans.get(page.get() - 1).copied()
    }

    pub fn visible_numbering_for_page(&self, page: NonZeroUsize) -> Option<&Content> {
        self.visible_numberings.get(page.get() - 1).and_then(Option::as_ref)
    }

    pub fn reference_numbering_for_page(&self, page: NonZeroUsize) -> Option<&Content> {
        self.reference_numberings.get(page.get() - 1).and_then(Option::as_ref)
    }

    /// Supplement para `page` (1-based).
    ///
    /// Devolve `None` quando pre-injecção, fora de range, ou
    /// sem supplement capturado. Distingue "sem supplement"
    /// de "supplement vazio" (vanilla colapsa ambos via
    /// `Content::empty()`).
    pub fn supplement_for_page(&self, page: NonZeroUsize) -> Option<&Content> {
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
    use ecow::EcoString;

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
        let _full: PageStore = PageStore::from_runtime(nz(1), Vec::new(), Vec::new());
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
            Some(Numbering::Pattern(EcoString::from("1"))), // page 1
            None,                                           // page 2 sem numbering
            Some(Numbering::Pattern(EcoString::from("I"))), // page 3
        ];
        let supplements = vec![Content::Empty, Content::Empty, Content::Empty];
        let s = PageStore::from_runtime(nz(3), numberings, supplements);

        assert_eq!(s.total_pages(), Some(nz(3)));
        assert_eq!(s.numbering_for_page(nz(1)).map(|e| e.as_str()), Some("1"),);
        assert_eq!(s.numbering_for_page(nz(2)), None);
        assert_eq!(s.numbering_for_page(nz(3)).map(|e| e.as_str()), Some("I"),);
        // Supplement.
        assert!(s.supplement_for_page(nz(1)).is_some());
        assert!(s.supplement_for_page(nz(2)).is_some());
        assert!(s.supplement_for_page(nz(3)).is_some());
    }

    #[test]
    fn fora_de_range_devolve_none_sem_panic() {
        let s = PageStore::from_runtime(
            nz(2),
            vec![
                Some(Numbering::Pattern(EcoString::from("1"))),
                Some(Numbering::Pattern(EcoString::from("2"))),
            ],
            vec![Content::Empty, Content::Empty],
        );
        // Page 3 não existe (total = 2).
        assert_eq!(s.numbering_for_page(nz(3)), None);
        assert_eq!(s.supplement_for_page(nz(3)), None);
        // Page 100 idem.
        assert_eq!(s.numbering_for_page(nz(100)), None);
        assert_eq!(s.supplement_for_page(nz(100)), None);
    }

    #[test]
    fn p1159_vistas_realizadas_preservam_vazio_e_ausencia() {
        let s = PageStore::from_realized(
            nz(2),
            vec![Some(Numbering::Pattern("1 / 1".into())), None],
            vec![7, 8],
            vec![Span::detached(), Span::detached()],
            vec![Some(Content::text("N7/8")), None],
            vec![Some(Content::Empty), None],
            vec![Content::Empty, Content::Empty],
        );
        assert_eq!(s.logical_number_for_page(nz(1)), Some(7));
        assert_eq!(s.logical_number_for_page(nz(2)), Some(8));
        assert_eq!(s.visible_numbering_for_page(nz(1)).unwrap().plain_text(), "N7/8");
        assert_eq!(s.reference_numbering_for_page(nz(1)), Some(&Content::Empty));
        assert_eq!(s.reference_numbering_for_page(nz(2)), None);
        assert_eq!(s.visible_numbering_for_page(nz(3)), None);
        assert_eq!(s.numbering_span_for_page(nz(3)), None);
    }
}

impl PageStore {
    /// Conservative exact comparison for the laboratory scalar page snapshot.
    pub fn same_scalar_snapshot(&self, other: &Self) -> bool {
        fn content(a: &Content, b: &Content, depth: usize, budget: &mut usize) -> bool {
            if depth > 32 || *budget == 0 {
                return false;
            }
            *budget -= 1;
            match a {
                Content::Empty => matches!(b, Content::Empty),
                Content::Text(a) => matches!(b,Content::Text(b) if a==b),
                Content::Space => matches!(b, Content::Space),
                Content::SpaceFromNewline => matches!(b, Content::SpaceFromNewline),
                Content::Parbreak => matches!(b, Content::Parbreak),
                Content::Sequence(a) => {
                    matches!(b,Content::Sequence(b) if a.len()==b.len() && a.iter().zip(b.iter()).all(|(a,b)|content(a,b,depth+1,budget)))
                }
                Content::Source(..)
                | Content::ShowRecipe(..)
                | Content::Realized(..)
                | Content::Located(..)
                | Content::Symbol(..)
                | Content::HtmlElem(..)
                | Content::HtmlFrame(..)
                | Content::Par { .. }
                | Content::Heading(..)
                | Content::Title(..)
                | Content::Strong(..)
                | Content::Emph(..)
                | Content::Raw(..)
                | Content::ListItem(..)
                | Content::EnumItem(..)
                | Content::Link(..)
                | Content::Equation(..)
                | Content::MathSequence(..)
                | Content::MathIdent(..)
                | Content::MathText(..)
                | Content::MathFrac(..)
                | Content::MathAttach(..)
                | Content::MathPrimes(..)
                | Content::MathRoot(..)
                | Content::MathStretch(..)
                | Content::MathDelimited(..)
                | Content::MathLr(..)
                | Content::MathMid(..)
                | Content::MathOverbrace(..)
                | Content::MathUnderbrace(..)
                | Content::MathUnderbracket(..)
                | Content::MathUnderparen(..)
                | Content::MathUndershell(..)
                | Content::MathOverbracket(..)
                | Content::MathOverparen(..)
                | Content::MathOvershell(..)
                | Content::MathAlignPoint(..)
                | Content::Linebreak(..)
                | Content::MathMatrix(..)
                | Content::MathCases(..)
                | Content::MathAccent(..)
                | Content::MathCancel(..)
                | Content::MathUnderline(..)
                | Content::MathOverline(..)
                | Content::MathVec(..)
                | Content::MathClassOverride(..)
                | Content::MathLimitsOverride(..)
                | Content::MathUnderover(..)
                | Content::MathOp(..)
                | Content::MathStyled(..)
                | Content::Label(..)
                | Content::Ref(..)
                | Content::CounterDisplay(..)
                | Content::CounterUpdate(..)
                | Content::Outline(..)
                | Content::OutlineEntry(..)
                | Content::Figure(..)
                | Content::FigureCaption(..)
                | Content::Image(..)
                | Content::Shape(..)
                | Content::Curve(..)
                | Content::Transform(..)
                | Content::Grid(..)
                | Content::GridHeader(..)
                | Content::GridFooter(..)
                | Content::GridCell(..)
                | Content::SetPage(..)
                | Content::PageRun(..)
                | Content::Align(..)
                | Content::Place(..)
                | Content::Flush(..)
                | Content::Styled(..)
                | Content::Divider(..)
                | Content::Terms(..)
                | Content::TermItem(..)
                | Content::Quote(..)
                | Content::Document { .. }
                | Content::Asset { .. }
                | Content::SmartQuote(..)
                | Content::PdfAttach(..)
                | Content::PdfArtifact(..)
                | Content::Underline(..)
                | Content::Strike(..)
                | Content::Overline(..)
                | Content::SmallCaps { .. }
                | Content::Pad(..)
                | Content::Hide(..)
                | Content::HSpace(..)
                | Content::VSpace(..)
                | Content::Pagebreak(..)
                | Content::Colbreak(..)
                | Content::Stack(..)
                | Content::Boxed(..)
                | Content::Block(..)
                | Content::TableCell(..)
                | Content::Bibliography(..)
                | Content::Cite(..)
                | Content::Footnote(..)
                | Content::FootnoteEntry(..)
                | Content::TableHeader(..)
                | Content::TableFooter(..)
                | Content::GridHLine(..)
                | Content::GridVLine(..)
                | Content::TableHLine(..)
                | Content::TableVLine(..)
                | Content::Table(..)
                | Content::Repeat(..)
                | Content::Columns(..)
                | Content::Metadata(..)
                | Content::State(..)
                | Content::StateUpdate(..)
                | Content::StateDisplay(..)
                | Content::CounterDisplayCallback(..)
                | Content::ContextBlock(..)
                | Content::Dynamic(..) => false,
            }
        }
        fn optional(
            a: &Option<Content>,
            b: &Option<Content>,
            budget: &mut usize,
        ) -> bool {
            match (a, b) {
                (None, None) => true,
                (Some(a), Some(b)) => content(a, b, 0, budget),
                (None, Some(_)) | (Some(_), None) => false,
            }
        }
        let mut budget = 8192;
        self.total_pages == other.total_pages
            && self.logical_numbers == other.logical_numbers
            && self.numbering_spans == other.numbering_spans
            && self.numberings.len() == other.numberings.len()
            && self
                .numberings
                .iter()
                .zip(&other.numberings)
                .all(|(a, b)| match (a, b) {
                    (None, None) => true,
                    (Some(Numbering::Pattern(a)), Some(Numbering::Pattern(b))) => a == b,
                    (Some(Numbering::Func(_)), _)
                    | (_, Some(Numbering::Func(_)))
                    | (None, Some(Numbering::Pattern(_)))
                    | (Some(Numbering::Pattern(_)), None) => false,
                })
            && self.visible_numberings.len() == other.visible_numberings.len()
            && self
                .visible_numberings
                .iter()
                .zip(&other.visible_numberings)
                .all(|(a, b)| optional(a, b, &mut budget))
            && self.reference_numberings.len() == other.reference_numberings.len()
            && self
                .reference_numberings
                .iter()
                .zip(&other.reference_numberings)
                .all(|(a, b)| optional(a, b, &mut budget))
            && self.supplements.len() == other.supplements.len()
            && self
                .supplements
                .iter()
                .zip(&other.supplements)
                .all(|(a, b)| content(a, b, 0, &mut budget))
    }
}
#[cfg(test)]
mod physical_retention_page_tests {
    use super::*;
    #[test]
    fn physical_retention_page_identical_scalar_data_is_certified() {
        let s = PageStore::from_runtime(
            NonZeroUsize::new(1).unwrap(),
            vec![Some("1".into())],
            vec![Content::text("Page")],
        );
        assert!(s.same_scalar_snapshot(&s.clone()));
        assert!(PageStore::empty().same_scalar_snapshot(&PageStore::empty()));
    }
    #[test]
    fn physical_retention_page_origin_and_missing_slots_are_distinct() {
        let a = PageStore::from_runtime(
            NonZeroUsize::new(1).unwrap(),
            vec![None],
            vec![Content::Space],
        );
        let b = PageStore::from_runtime(
            NonZeroUsize::new(1).unwrap(),
            vec![None],
            vec![Content::SpaceFromNewline],
        );
        let c = PageStore::from_total_pages(NonZeroUsize::new(1).unwrap());
        assert!(!a.same_scalar_snapshot(&b));
        assert!(!a.same_scalar_snapshot(&c));
    }
}
