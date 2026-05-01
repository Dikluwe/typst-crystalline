//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/element_kind.md
//! @prompt-hash 90bffae0
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
}

impl ElementKind {
    /// Forma textual estável (para diagnóstico e debug).
    pub fn as_str(self) -> &'static str {
        match self {
            ElementKind::Heading     => "heading",
            ElementKind::Figure      => "figure",
            ElementKind::Citation    => "citation",
            ElementKind::Metadata    => "metadata",
            ElementKind::State       => "state",
            ElementKind::StateUpdate => "state_update",
        }
    }

    /// Parse inverso: aceita "heading"/"figure"/"citation"/"metadata"/
    /// "state"/"state_update". P175 (M9 sub-passo 5) — usado por
    /// stdlib `query(kind_str)` para construir `Selector::Kind`.
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "heading"      => Some(ElementKind::Heading),
            "figure"       => Some(ElementKind::Figure),
            "citation"     => Some(ElementKind::Citation),
            "metadata"     => Some(ElementKind::Metadata),
            "state"        => Some(ElementKind::State),
            "state_update" => Some(ElementKind::StateUpdate),
            _              => None,
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
}
