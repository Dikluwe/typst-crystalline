//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/document_format_options.md
//! @prompt-hash 1d1fd53f
//! @layer L1
//! @updated 2026-09-20

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

/// Opções documentais do export PDF.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PdfFormatOptions {
    pub tagged: PdfTagged,
    pub pretty: bool,
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
}
