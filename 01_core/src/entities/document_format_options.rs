//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/document_format_options.md
//! @prompt-hash 18b2b57f
//! @layer L1
//! @updated 2026-09-19

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

/// Opções de formato resolvidas pela cascata do documento.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DocumentFormatOptions {
    pub svg: SvgFormatOptions,
    pub png: PngFormatOptions,
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
        second.svg.pretty = true;
        assert_ne!(first, second);
        assert!(!first.svg.pretty);
    }
}
