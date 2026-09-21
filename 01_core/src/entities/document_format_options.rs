//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/document_format_options.md
//! @prompt-hash 37486278
//! @layer L1
//! @updated 2026-09-20

use std::num::NonZeroUsize;

/// Opções documentais do export SVG.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SvgFormatOptions {
    pub pretty: bool,
}

/// Opções documentais do export PNG.
#[derive(Debug, Clone, PartialEq)]
pub struct PngFormatOptions {
    pub ppi: f64,
}

impl Default for PngFormatOptions {
    fn default() -> Self {
        Self { ppi: 144.0 }
    }
}

/// Opções documentais do export HTML.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HtmlFormatOptions {
    pub pretty: bool,
}

/// Estado de tagging do export PDF: três estados da linguagem, não um
/// `Option<bool>` de conveniência.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PdfTagged {
    #[default]
    Auto,
    Explicit(bool),
}

/// **P1468** — um intervalo inclusivo de páginas físicas (1-indexed).
///
/// Espelha `typst-library/src/layout/page.rs:801-808` do vanilla ratificado.
/// Limite `None` de um lado é "aberto" (sem restrição desse lado).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageRange {
    pub start: Option<NonZeroUsize>,
    pub end: Option<NonZeroUsize>,
}

impl PageRange {
    /// Página única (`start == end == nr`), espelhando `PageRange::at` do
    /// vanilla (`typst-library/src/layout/page.rs:857-859`).
    pub fn at(nr: NonZeroUsize) -> Self {
        Self { start: Some(nr), end: Some(nr) }
    }

    pub fn includes(&self, page: NonZeroUsize) -> bool {
        match (self.start, self.end) {
            (Some(s), Some(e)) => (s..=e).contains(&page),
            (Some(s), None) => page >= s,
            (None, Some(e)) => page <= e,
            (None, None) => true,
        }
    }
}

/// **P1468** — lista de intervalos de páginas a exportar.
///
/// Não normaliza, deduplica nem ordena a lista interna (ver
/// `entities/document_format_options.md` P1468) — a semântica de "conjunto
/// físico único e ordenado" emerge exclusivamente de quem consulta
/// `includes_page` iterando as páginas físicas em ordem crescente
/// (`infra/export/builder.md`).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PageRanges(Vec<PageRange>);

impl PageRanges {
    pub fn new(ranges: Vec<PageRange>) -> Self {
        Self(ranges)
    }

    pub fn includes_page(&self, page: NonZeroUsize) -> bool {
        self.0.iter().any(|r| r.includes(page))
    }
}

/// Opções documentais do export PDF.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PdfFormatOptions {
    pub tagged: PdfTagged,
    pub pretty: bool,
    /// **P1468** — seleção de páginas a exportar (`#set format.pdf(pages:
    /// ...)`). `None` = todas as páginas (default).
    pub pages: Option<PageRanges>,
}

/// Opções de formato resolvidas pela cascata do documento.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DocumentFormatOptions {
    pub svg: SvgFormatOptions,
    pub png: PngFormatOptions,
    pub html: HtmlFormatOptions,
    pub pdf: PdfFormatOptions,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1462_defaults_clone_and_equality_are_independent() {
        let first = DocumentFormatOptions::default();
        let mut second = first.clone();

        assert_eq!(first, second);
        assert!(!first.svg.pretty);
        assert!(!first.html.pretty);
        second.svg.pretty = true;
        assert_ne!(first, second);
        assert!(!first.svg.pretty);
        assert!(!first.html.pretty);
    }

    #[test]
    fn p1465_html_default_clone_and_equality_are_independent() {
        let first = DocumentFormatOptions::default();
        let mut second = first.clone();

        assert_eq!(first, second);
        assert!(!first.html.pretty);
        second.html.pretty = true;
        assert_ne!(first, second);
        assert!(!first.html.pretty);
    }

    #[test]
    fn p1467_pdf_default_is_auto_tagged_and_compact_pretty() {
        let options = PdfFormatOptions::default();

        assert_eq!(options.tagged, PdfTagged::Auto);
        assert!(!options.pretty);
    }

    #[test]
    fn p1467_document_default_pdf_is_auto_tagged_and_compact_pretty() {
        let document = DocumentFormatOptions::default();

        assert_eq!(document.pdf.tagged, PdfTagged::Auto);
        assert!(!document.pdf.pretty);
    }

    #[test]
    fn p1467_pdf_tagged_clone_and_equality_are_independent() {
        let first = DocumentFormatOptions::default();
        let mut second = first.clone();

        assert_eq!(first, second);
        assert_eq!(first.pdf.tagged, PdfTagged::Auto);
        second.pdf.tagged = PdfTagged::Explicit(true);
        assert_ne!(first, second);
        assert_eq!(first.pdf.tagged, PdfTagged::Auto);
    }

    #[test]
    fn p1467_pdf_pretty_clone_and_equality_are_independent() {
        let first = DocumentFormatOptions::default();
        let mut second = first.clone();

        assert_eq!(first, second);
        assert!(!first.pdf.pretty);
        second.pdf.pretty = true;
        assert_ne!(first, second);
        assert!(!first.pdf.pretty);
    }

    #[test]
    fn p1467_pdf_tagged_variants_are_pairwise_distinct() {
        assert_ne!(PdfTagged::Explicit(true), PdfTagged::Explicit(false));
        assert_ne!(PdfTagged::Explicit(true), PdfTagged::Auto);
        assert_ne!(PdfTagged::Explicit(false), PdfTagged::Auto);
        assert_eq!(PdfTagged::Explicit(true), PdfTagged::Explicit(true));
        assert_eq!(PdfTagged::Auto, PdfTagged::Auto);
    }

    #[test]
    fn p1467_pdf_field_is_orthogonal_to_svg_png_html() {
        let baseline = DocumentFormatOptions::default();
        let mut only_pdf_changed = baseline.clone();
        only_pdf_changed.pdf.tagged = PdfTagged::Explicit(false);
        only_pdf_changed.pdf.pretty = true;

        assert_eq!(only_pdf_changed.svg, baseline.svg);
        assert_eq!(only_pdf_changed.png, baseline.png);
        assert_eq!(only_pdf_changed.html, baseline.html);
        assert_ne!(only_pdf_changed.pdf, baseline.pdf);

        let mut only_others_changed = baseline.clone();
        only_others_changed.svg.pretty = true;
        only_others_changed.html.pretty = true;
        only_others_changed.png.ppi = 300.0;

        assert_eq!(only_others_changed.pdf, baseline.pdf);
        assert_eq!(only_others_changed.pdf.tagged, PdfTagged::Auto);
        assert!(!only_others_changed.pdf.pretty);
    }

    // ── P1468 (RED — Agente B) — `PageRange`/`PageRanges` e
    // `PdfFormatOptions::pages`. Nenhum dos três existe neste baseline
    // (`0b322a926dd2f86b49aabfeb0d2f287ce06aaace`); este módulo falha por
    // erro de compilação até o carrier P1468 (mesmo gate ADR-0127) ser
    // materializado por outro agente em paralelo.
    use std::num::NonZeroUsize;

    fn page(n: usize) -> NonZeroUsize {
        NonZeroUsize::new(n).unwrap()
    }

    fn closed(start: usize, end: usize) -> PageRange {
        PageRange { start: Some(page(start)), end: Some(page(end)) }
    }

    fn open_at_start(end: usize) -> PageRange {
        PageRange { start: None, end: Some(page(end)) }
    }

    fn open_at_end(start: usize) -> PageRange {
        PageRange { start: Some(page(start)), end: None }
    }

    fn universal() -> PageRange {
        PageRange { start: None, end: None }
    }

    #[test]
    fn p1468_page_range_includes_closed_range() {
        let range = closed(2, 4);
        assert!(!range.includes(page(1)));
        assert!(range.includes(page(2)));
        assert!(range.includes(page(3)));
        assert!(range.includes(page(4)));
        assert!(!range.includes(page(5)));
    }

    #[test]
    fn p1468_page_range_includes_open_at_start() {
        // Aberto no início (sem `start`): cobre 1..=end.
        let range = open_at_start(3);
        assert!(range.includes(page(1)));
        assert!(range.includes(page(3)));
        assert!(!range.includes(page(4)));
    }

    #[test]
    fn p1468_page_range_includes_open_at_end() {
        // Aberto no fim (sem `end`): cobre start..
        let range = open_at_end(3);
        assert!(!range.includes(page(2)));
        assert!(range.includes(page(3)));
        assert!(range.includes(page(1000)));
    }

    #[test]
    fn p1468_page_range_includes_universal_range() {
        let range = universal();
        assert!(range.includes(page(1)));
        assert!(range.includes(page(9999)));
    }

    #[test]
    fn p1468_page_ranges_includes_page_with_overlapping_and_disjoint_ranges() {
        let ranges = PageRanges::new(vec![closed(1, 2), closed(2, 4), closed(10, 10)]);
        assert!(ranges.includes_page(page(1)));
        assert!(ranges.includes_page(page(2)));
        assert!(ranges.includes_page(page(3)));
        assert!(ranges.includes_page(page(4)));
        assert!(!ranges.includes_page(page(5)));
        assert!(!ranges.includes_page(page(9)));
        assert!(ranges.includes_page(page(10)));
        assert!(!ranges.includes_page(page(11)));
    }

    #[test]
    fn p1468_page_ranges_default_is_empty_and_pdf_default_pages_is_none() {
        let ranges = PageRanges::default();
        assert!(!ranges.includes_page(page(1)));

        let pdf = PdfFormatOptions::default();
        assert_eq!(pdf.pages, None);

        let document = DocumentFormatOptions::default();
        assert_eq!(document.pdf.pages, None);
    }

    #[test]
    fn p1468_page_ranges_does_not_normalize_dedupe_or_sort() {
        let a = closed(3, 3);
        let b = closed(1, 1);
        // Mesmos ranges, ordens diferentes: a lista interna não é
        // normalizada/ordenada (ver `entities/document_format_options.md`
        // P1468), logo a igualdade derivada é sensível à ordem.
        assert_ne!(
            PageRanges::new(vec![a.clone(), b.clone()]),
            PageRanges::new(vec![b, a])
        );

        // Duplicatas exatas não são deduplicadas: `includes_page` continua
        // correto porque é um `.any()`, não porque a lista foi limpa.
        let duplicated = PageRanges::new(vec![closed(5, 5), closed(5, 5)]);
        assert!(duplicated.includes_page(page(5)));
        assert!(!duplicated.includes_page(page(6)));
    }

    #[test]
    fn p1468_page_range_and_page_ranges_derive_eq_and_clone_are_independent() {
        let first = PdfFormatOptions::default();
        let mut second = first.clone();

        assert_eq!(first, second);
        assert_eq!(first.pages, None);
        second.pages = Some(PageRanges::new(vec![closed(1, 1)]));
        assert_ne!(first, second);
        assert_eq!(first.pages, None);

        assert_eq!(closed(1, 2), closed(1, 2));
        assert_ne!(closed(1, 2), closed(1, 3));
        assert_ne!(closed(1, 2), open_at_end(1));
    }

    #[test]
    fn p1468_pdf_pages_field_is_orthogonal_to_tagged_pretty_svg_png_html() {
        let baseline = DocumentFormatOptions::default();
        let mut only_pages_changed = baseline.clone();
        only_pages_changed.pdf.pages = Some(PageRanges::new(vec![closed(2, 3)]));

        assert_eq!(only_pages_changed.svg, baseline.svg);
        assert_eq!(only_pages_changed.png, baseline.png);
        assert_eq!(only_pages_changed.html, baseline.html);
        assert_eq!(only_pages_changed.pdf.tagged, baseline.pdf.tagged);
        assert_eq!(only_pages_changed.pdf.pretty, baseline.pdf.pretty);
        assert_ne!(only_pages_changed.pdf.pages, baseline.pdf.pages);

        let mut only_others_changed = baseline.clone();
        only_others_changed.pdf.tagged = PdfTagged::Explicit(true);
        only_others_changed.pdf.pretty = true;
        only_others_changed.svg.pretty = true;

        assert_eq!(only_others_changed.pdf.pages, baseline.pdf.pages);
    }
}
