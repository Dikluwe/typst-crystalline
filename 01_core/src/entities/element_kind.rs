//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/element_kind.md
//! @prompt-hash a8e6ef80
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
    /// **P461** — `Content::Table` promovido a locatable. Indexa
    /// locations de tables em `kind_index`; counter flat `"table"`
    /// populado quando numbering+caption activos. Alinha com heading/
    /// figure/equation e desbloqueia label/ref para tables (Trilha 2).
    Table,
    /// **P494** — Selector `list`. Cristalino não materializa
    /// `Content::List` (lista é `Sequence` de `ListItem`), por isso
    /// `kind_index` fica vazio; a contagem é feita em L3 por análise
    /// do `Content` (`query_helpers.rs`). O kind existe em L1 para
    /// permitir `Selector::Kind(List)` e paridade de parse.
    List,
    /// **P494** — Selector `enum`. Cristalino não materializa
    /// `Content::Enum` (enum é `Sequence` de `EnumItem`); contagem em
    /// L3 por análise do `Content`.
    Enum,
    /// **P494** — Selector `par`. Cristalino não materializa
    /// `Content::Par` (parágrafo é texto plano numa `Sequence`);
    /// contagem aproximada em L3 (presença de texto no documento).
    Par,
    /// **P494** — Selector `link`. `Content::Link` existe em L1;
    /// contagem por análise do `Content` em L3 (alternativa: tornar
    /// locatable em passo futuro).
    Link,
    /// **P494** — Selector `raw`. `Content::Raw` existe em L1;
    /// contagem por análise do `Content` em L3.
    Raw,
    /// **P494** — Selector `quote`. `Content::Quote` existe em L1;
    /// contagem por análise do `Content` em L3.
    Quote,
    /// **P494** — Selector `footnote`. `Content::Footnote` existe em
    /// L1; contagem por análise do `Content` em L3.
    Footnote,
    /// **P240 (M9d/M7+1)** — `Content::StateDisplay` promovido a
    /// locatable. Indexa locations de StateDisplay em `kind_index`;
    /// valor pre-rendered é produzido em `apply_state_displays`
    /// pós-fixpoint (paralelo `apply_state_funcs` P191B).
    StateDisplay,
    /// **P241 (M9d/M7+2)** — `Content::CounterDisplayCallback`
    /// promovido a locatable paralelo absoluto `StateDisplay` P240.
    /// Indexa locations em `kind_index`; valor pre-rendered produzido
    /// em `apply_counter_displays` pós-fixpoint. Distinto de
    /// `Content::CounterDisplay { kind }` legacy (não-locatable).
    CounterDisplay,

    /// **P506** — `Content::ContextBlock` (delayed evaluation via
    /// `context { expr }`). Locatável para capturar a Location do
    /// ponto do documento onde o bloco aparece.
    ContextBlock,
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
            ElementKind::Table         => "table",
            ElementKind::List          => "list",
            ElementKind::Enum          => "enum",
            ElementKind::Par           => "par",
            ElementKind::Link          => "link",
            ElementKind::Raw           => "raw",
            ElementKind::Quote         => "quote",
            ElementKind::Footnote      => "footnote",
            ElementKind::StateDisplay  => "state_display",
            ElementKind::CounterDisplay => "counter_display",
            ElementKind::ContextBlock  => "context_block",
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
            "table"          => Some(ElementKind::Table),
            "list"           => Some(ElementKind::List),
            "enum"           => Some(ElementKind::Enum),
            "par"            => Some(ElementKind::Par),
            "link"           => Some(ElementKind::Link),
            "raw"            => Some(ElementKind::Raw),
            "quote"          => Some(ElementKind::Quote),
            "footnote"       => Some(ElementKind::Footnote),
            "state_display"  => Some(ElementKind::StateDisplay),
            "counter_display" => Some(ElementKind::CounterDisplay),
            "context_block"  => Some(ElementKind::ContextBlock),
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

    // ── P494 — Selectors de elementos de documento ──────────────────────

    #[test]
    fn p494_document_element_kinds_existem_e_sao_distintos() {
        let list = ElementKind::List;
        let enu = ElementKind::Enum;
        let par = ElementKind::Par;
        let link = ElementKind::Link;
        let raw = ElementKind::Raw;
        let quote = ElementKind::Quote;
        let footnote = ElementKind::Footnote;

        assert_eq!(list, ElementKind::List);
        assert_eq!(enu, ElementKind::Enum);
        assert_eq!(par, ElementKind::Par);
        assert_eq!(link, ElementKind::Link);
        assert_eq!(raw, ElementKind::Raw);
        assert_eq!(quote, ElementKind::Quote);
        assert_eq!(footnote, ElementKind::Footnote);

        assert_ne!(list, enu);
        assert_ne!(enu, par);
        assert_ne!(par, link);
        assert_ne!(link, raw);
        assert_ne!(raw, quote);
        assert_ne!(quote, footnote);
        assert_ne!(footnote, list);

        // Devem ser distintos dos kinds clássicos.
        assert_ne!(list, ElementKind::Heading);
        assert_ne!(link, ElementKind::Figure);
        assert_ne!(raw, ElementKind::Equation);
    }

    #[test]
    fn p494_document_element_as_str() {
        assert_eq!(ElementKind::List.as_str(), "list");
        assert_eq!(ElementKind::Enum.as_str(), "enum");
        assert_eq!(ElementKind::Par.as_str(), "par");
        assert_eq!(ElementKind::Link.as_str(), "link");
        assert_eq!(ElementKind::Raw.as_str(), "raw");
        assert_eq!(ElementKind::Quote.as_str(), "quote");
        assert_eq!(ElementKind::Footnote.as_str(), "footnote");
    }

    #[test]
    fn p494_document_element_from_name() {
        assert_eq!(ElementKind::from_name("list"), Some(ElementKind::List));
        assert_eq!(ElementKind::from_name("enum"), Some(ElementKind::Enum));
        assert_eq!(ElementKind::from_name("par"), Some(ElementKind::Par));
        assert_eq!(ElementKind::from_name("link"), Some(ElementKind::Link));
        assert_eq!(ElementKind::from_name("raw"), Some(ElementKind::Raw));
        assert_eq!(ElementKind::from_name("quote"), Some(ElementKind::Quote));
        assert_eq!(ElementKind::from_name("footnote"), Some(ElementKind::Footnote));
    }
}
