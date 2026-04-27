//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/citation_form.md
//! @prompt-hash 677849cb
//! @layer L1
//! @updated 2026-04-27
//!
//! Enum entity `CitationForm` (forms de citação minimal).
//! Adicionado no Passo 159C (Model Bibliography + Cite Fase 2
//! sub-passo 2) como suporte a `Content::Cite { ..., form, .. }`.
//!
//! Subset minimal de forms vanilla per ADR-0054 graded:
//! 4 forms universais (Normal/Prose/Author/Year). Forms vanilla
//! adicionais (`Full`, CSL-specific) **diferidos** para refinos
//! futuros NÃO reservados.
//!
//! Quinta aplicação consecutiva do padrão "tipo entity em ficheiro
//! próprio" (Sides P156C → Parity P156E → Dir P156I → BibEntry
//! P159A → CitationForm P159C).

/// Forma de citação — vanilla `CiteForm` reduzido a 4 forms
/// universais.
///
/// **Subset minimal** per ADR-0054 graded e diagnóstico P159C
/// §1. Render real CSL é diferido (depende hayagriva ADR-0062);
/// cristalino renderiza placeholder melhorado por form com lookup
/// Bibliography same-document.
///
/// Variants:
/// - `Normal` — `[key]` placeholder (default; paridade P159A).
/// - `Prose`  — `Author (Year)` quando key existe; fallback `[key]`.
/// - `Author` — apenas autor; fallback `[key]`.
/// - `Year`   — apenas ano; fallback `[key]`.
///
/// `Default` explícito como `Normal` (paridade vanilla
/// `CiteForm::Normal`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CitationForm {
    Normal,
    Prose,
    Author,
    Year,
}

impl Default for CitationForm {
    fn default() -> Self {
        Self::Normal
    }
}

impl CitationForm {
    /// Serialização inversa para tests/debug.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Prose  => "prose",
            Self::Author => "author",
            Self::Year   => "year",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn citation_form_constructor_each_variant() {
        let n = CitationForm::Normal;
        let p = CitationForm::Prose;
        let a = CitationForm::Author;
        let y = CitationForm::Year;
        assert_eq!(n.as_str(), "normal");
        assert_eq!(p.as_str(), "prose");
        assert_eq!(a.as_str(), "author");
        assert_eq!(y.as_str(), "year");
    }

    #[test]
    fn citation_form_partial_eq() {
        assert_eq!(CitationForm::Normal, CitationForm::Normal);
        assert_ne!(CitationForm::Normal, CitationForm::Prose);
        assert_ne!(CitationForm::Author, CitationForm::Year);
    }

    #[test]
    fn citation_form_default_normal() {
        assert_eq!(CitationForm::default(), CitationForm::Normal);
    }
}
