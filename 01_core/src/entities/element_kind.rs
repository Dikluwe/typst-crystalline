//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/element_kind.md
//! @prompt-hash 1c2f3200
//! @layer L1
//! @updated 2026-04-30
//!
//! `ElementKind` — discriminador estreito dos tipos de elemento que
//! entram no índice de introspecção. P161 sub-passo .5: apenas 3
//! kinds em M1.

/// Tipo de elemento indexado pela introspecção.
///
/// Apenas três kinds em P161. Outras (Equation, Footnote, ListItem,
/// etc.) ficam para passos correspondentes às features.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElementKind {
    Heading,
    Figure,
    Citation,
    /// **P169 (M9 sub-passo 1)** — feature `metadata(value)` vanilla.
    /// Valor opaco embebido para query via `Introspector::query_metadata`.
    Metadata,
    /// **P171 (M9 sub-passo 3)** — `state(key, init)` runtime state.
    State,
    /// **P171 (M9 sub-passo 3)** — `state.update(key, value)` runtime update.
    StateUpdate,
    /// **P178** — `Content::Outline` agora locatable; permite
    /// `query("outline")` retornar count correcto. Fecha lacuna #7
    /// (`has_outline`).
    Outline,
    /// **P181C** — `Content::Bibliography` promovido a locatable em
    /// P181D (decisão P181A cláusula 4 — Opção β walk puro). `from_tags`
    /// arm Bibliography (P181E) popula `BibStore`. Suporta plano
    /// P181 para fechar lacuna #6 (`bib_entries`/`bib_numbers`).
    Bibliography,
    /// **P186B** — `Content::Equation` promovido a locatable em
    /// P186C/D/E. Indexa locations de equações em `kind_index`.
    /// Suporta C2 (equation counter) desbloqueio per ADR-0068
    /// (eixo 2 P183C); consumer migra em P188.
    Equation,
    /// **P198C** — `Content::CounterUpdate` promovido a locatable
    /// em P198C (cenário β-promote ADR-0069). Indexa locations de
    /// CounterUpdate em `kind_index`. `from_tags` arm aplica à
    /// CounterRegistry via `apply_at` (flat) ou `apply_hierarchical_at`
    /// (key="heading"). Suporta E6 fechar estruturalmente.
    CounterUpdate,
}

impl ElementKind {
    /// Forma textual estável (para diagnóstico e debug).
    pub fn as_str(self) -> &'static str {
        match self {
            ElementKind::Heading       => "heading",
            ElementKind::Figure        => "figure",
            ElementKind::Citation      => "citation",
            ElementKind::Metadata      => "metadata",
            ElementKind::State         => "state",
            ElementKind::StateUpdate   => "state_update",
            ElementKind::Outline       => "outline",
            ElementKind::Bibliography  => "bibliography",
            ElementKind::Equation      => "equation",
            ElementKind::CounterUpdate => "counter_update",
        }
    }

    /// Parse inverso: aceita os nomes textuais dos kinds.
    /// P175 (M9 sub-passo 5) — usado por stdlib `query(kind_str)`.
    /// **P178**: `"outline"` adicionado.
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "heading"        => Some(ElementKind::Heading),
            "figure"         => Some(ElementKind::Figure),
            "citation"       => Some(ElementKind::Citation),
            "metadata"       => Some(ElementKind::Metadata),
            "state"          => Some(ElementKind::State),
            "state_update"   => Some(ElementKind::StateUpdate),
            "outline"        => Some(ElementKind::Outline),
            "bibliography"   => Some(ElementKind::Bibliography),
            "equation"       => Some(ElementKind::Equation),
            "counter_update" => Some(ElementKind::CounterUpdate),
            _                => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_str_devolve_nome_estavel() {
        assert_eq!(ElementKind::Heading.as_str(), "heading");
        assert_eq!(ElementKind::Figure.as_str(), "figure");
        assert_eq!(ElementKind::Citation.as_str(), "citation");
    }

    #[test]
    fn variantes_distintas() {
        assert_ne!(ElementKind::Heading, ElementKind::Figure);
        assert_ne!(ElementKind::Figure, ElementKind::Citation);
        assert_ne!(ElementKind::Heading, ElementKind::Citation);
    }

    #[test]
    fn copy_e_eq() {
        let k = ElementKind::Heading;
        let c = k;
        assert_eq!(k, c);
    }

    #[test]
    fn usavel_como_chave_em_hashmap() {
        use std::collections::HashMap;
        let mut m: HashMap<ElementKind, i32> = HashMap::new();
        m.insert(ElementKind::Heading, 1);
        m.insert(ElementKind::Figure, 2);
        m.insert(ElementKind::Citation, 3);
        assert_eq!(m.get(&ElementKind::Heading).copied(), Some(1));
        assert_eq!(m.get(&ElementKind::Figure).copied(), Some(2));
        assert_eq!(m.get(&ElementKind::Citation).copied(), Some(3));
    }

    // ── P178 — Outline variant ──────────────────────────────────────────

    #[test]
    fn outline_existe_e_distinto() {
        assert_eq!(ElementKind::Outline, ElementKind::Outline);
        assert_ne!(ElementKind::Outline, ElementKind::Heading);
        assert_ne!(ElementKind::Outline, ElementKind::Figure);
    }

    #[test]
    fn outline_as_str() {
        assert_eq!(ElementKind::Outline.as_str(), "outline");
    }

    #[test]
    fn from_name_outline() {
        assert_eq!(ElementKind::from_name("outline"), Some(ElementKind::Outline));
    }

    // ── P181C — Bibliography variant ────────────────────────────────────

    #[test]
    fn bibliography_existe_e_distinto() {
        let k = ElementKind::Bibliography;
        assert_eq!(k, ElementKind::Bibliography);
        assert_ne!(k, ElementKind::Outline);
        assert_ne!(k, ElementKind::Heading);
        assert_ne!(k, ElementKind::Citation);
    }

    #[test]
    fn bibliography_as_str() {
        assert_eq!(ElementKind::Bibliography.as_str(), "bibliography");
    }

    #[test]
    fn from_name_bibliography() {
        assert_eq!(
            ElementKind::from_name("bibliography"),
            Some(ElementKind::Bibliography),
        );
    }

    // ── P186B — Equation variant ────────────────────────────────────────

    #[test]
    fn equation_existe_e_distinto() {
        let k = ElementKind::Equation;
        assert_eq!(k, ElementKind::Equation);
        assert_ne!(k, ElementKind::Heading);
        assert_ne!(k, ElementKind::Figure);
        assert_ne!(k, ElementKind::Bibliography);
    }

    #[test]
    fn equation_as_str() {
        assert_eq!(ElementKind::Equation.as_str(), "equation");
    }

    #[test]
    fn from_name_equation() {
        assert_eq!(
            ElementKind::from_name("equation"),
            Some(ElementKind::Equation),
        );
    }
}
