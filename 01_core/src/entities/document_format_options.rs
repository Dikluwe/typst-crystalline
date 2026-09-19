//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/document_format_options.md
//! @prompt-hash 14cf779c
//! @layer L1
//! @updated 2026-09-19

/// Opções documentais do export SVG.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SvgFormatOptions {
    pub pretty: bool,
}

/// Opções de formato resolvidas pela cascata do documento.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DocumentFormatOptions {
    pub svg: SvgFormatOptions,
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
