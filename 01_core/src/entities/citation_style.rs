//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/citation_style.md
//! @prompt-hash 2f59ff74
//! @layer L1
//! @updated 2026-06-25
//!
//! **P468** — Estilo de citação bibliográfica (`author-date`, `numeric`,
//! `alphabetic`). Distinto de `CitationForm` (normal/prose/author/year),
//! que controla a forma da citação dentro de um estilo.

/// Estilo de citação bibliográfica — subset vanilla suportado no cristalino.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CitationStyle {
    /// Estilo autor-data, ex.: `(Kirsch, 1973)`.
    AuthorDate,
    /// Estilo numérico, ex.: `[1]`, `[2]`.
    Numeric,
    /// Estilo alfabético, ex.: `[Kir73]`.
    Alphabetic,
}

impl Default for CitationStyle {
    fn default() -> Self {
        Self::Numeric
    }
}

impl CitationStyle {
    /// Serialização inversa para parsing em stdlib/tests/debug.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AuthorDate => "author-date",
            Self::Numeric => "numeric",
            Self::Alphabetic => "alphabetic",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn citation_style_as_str() {
        assert_eq!(CitationStyle::AuthorDate.as_str(), "author-date");
        assert_eq!(CitationStyle::Numeric.as_str(), "numeric");
        assert_eq!(CitationStyle::Alphabetic.as_str(), "alphabetic");
    }

    #[test]
    fn citation_style_default_numeric() {
        assert_eq!(CitationStyle::default(), CitationStyle::Numeric);
    }
}
