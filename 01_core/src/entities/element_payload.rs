//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/element_payload.md
//! @prompt-hash 86032faf
//! @layer L1
//! @updated 2026-04-30
//!
//! `ElementPayload` — forma fechada e tipada dos dados específicos
//! de cada elemento indexado pela introspecção. P161 sub-passo .7.
//!
//! Campos confirmados em P161 sub-passo .1 (leitura de
//! `entities/content.rs`):
//! - `Content::Heading { level: u8, body: Box<Content> }`
//! - `Content::Figure { body, caption, kind: Option<String>, numbering }`
//! - `Content::Cite { key: String, supplement, form }`
//!
//! `body_hash` em `Heading` é placeholder (`0`) em P161; função de
//! hash determinística sobre `Content` é pendência registada para
//! P162.

use crate::entities::counter_update::CounterUpdate;
use crate::entities::label::Label;

/// Dados específicos por kind para indexação na introspecção.
///
/// `PartialEq` derivado. `Hash` implementado manualmente (P169) via
/// `format!("{:?}", self)` porque `Value` (em `Metadata` variant) não
/// implementa `Hash` directamente — Value contém `f64` em variantes
/// como `Length`/`Float`. Hash via Debug é determinístico para
/// estruturas equivalentes (mesma estratégia de `hash_content`).
///
/// `Eq` não é derivado/implementado: Value não é `Eq` (f64 NaN viola
/// reflexividade). Consumers que precisem de `Eq`-bound usam `PartialEq`.
#[derive(Debug, Clone, PartialEq)]
pub enum ElementPayload {
    Heading {
        /// Nível clamped 1..=6 (paridade `Content::Heading.level`).
        depth: u8,

        /// Hash determinístico do `body`. **Pendência P162** —
        /// função de hash sobre `Content` ainda não existe; em
        /// P161 callers passam `0`.
        body_hash: u128,

        /// Update implícito do contador "heading" associado.
        counter_update: CounterUpdate,
    },

    Figure {
        /// Discriminador de tipo (`"image"`, `"table"`, …).
        /// `None` = Auto (resolver no consumer).
        kind: Option<String>,

        /// Update implícito do contador `figure:{kind}`.
        counter_update: CounterUpdate,

        /// `true` se a figura conta para a numeração (predicado:
        /// `figure.numbering.is_some() && figure.caption.is_some()`).
        /// Adicionado em P168 (M5 sub-passo 2): permite que `from_tags`
        /// indexe apenas figuras numeradas para `figure_label_numbers`,
        /// preservando paridade com `CounterStateLegacy.figure_label_numbers`
        /// que aplica o mesmo filtro no walk arm `Content::Labelled`.
        is_counted: bool,
    },

    Citation {
        /// Chave da citação (`Content::Cite.key`).
        key: String,
    },

    /// **P169 (M9 sub-passo 1)** — payload de `metadata(value)`.
    ///
    /// `value` é embebido por valor (boxed para evitar tamanho da Value
    /// no payload). Consumer típico: `MetadataStore` populado por
    /// `from_tags`; query via `Introspector::query_metadata`.
    Metadata {
        value: Box<crate::entities::value::Value>,
    },

    /// **P171 (M9 sub-passo 3)** — payload de `state(key, init)`.
    State {
        key:  String,
        init: Box<crate::entities::value::Value>,
    },

    /// **P171 (M9 sub-passo 3)** — payload de `state.update(key, value)`.
    StateUpdate {
        key:    String,
        update: crate::entities::state_update::StateUpdate,
    },

    /// **P178** — payload de `Content::Outline`. Unit variant (Opção α):
    /// suficiente para `query("outline")` minimal contar locations.
    /// Refino futuro pode capturar `depth` e `title_hash`.
    Outline,

    /// **P181C** — payload de `Content::Bibliography`. Carrega entries
    /// completos (decisão P181A cláusula 2 — captura full por simetria
    /// com walk arm actual `state.bib_entries.extend(...)`). `from_tags`
    /// arm Bibliography (P181E pendente) extrai `entries` e popula
    /// `BibStore` via `add_bibliography(entries) + assign_number(key, n)`
    /// em loop. Hash via Debug (BibEntry deriva Debug; impl manual de
    /// Hash de ElementPayload cobre).
    Bibliography {
        entries: Vec<crate::entities::bib_entry::BibEntry>,
    },

    /// **P186B** — payload de `Content::Equation`. Forma paralela a
    /// `Figure` (P184B): `block: bool` distingue display-mode de inline,
    /// `counter_update` registado para futura flexibilidade (`Step`
    /// agora; `Update`/`Reset` quando equation set rule materializar).
    /// `from_tags` arm Equation (P186E) gate
    /// `block && state.value_at("numbering_active:equation", loc) ==
    /// Some(Bool(true))` — counter dormente em produção até
    /// `Content::SetEquationNumbering` (passo dedicado, fora da série
    /// P186). Suporta C2 desbloqueio per ADR-0068 (eixo 2 P183C);
    /// consumer migra em P188 via `flat_counter_at("equation",
    /// current_location)`.
    Equation {
        block:          bool,
        counter_update: CounterUpdate,
    },

    /// **P195B** — payload de `Content::Labelled` emitido em **post-recursion**
    /// pelo walk arm (per ADR-0069). Diferente dos outros variants
    /// (que vêm de `extract_payload` puro pre-recursion), este é
    /// produzido directamente pelo walk arm Labelled após recursão
    /// no target porque `resolved_text` depende de state mutado
    /// durante walk recursivo (counter formatting, lang).
    ///
    /// Campos:
    /// - `label`: chave para `intr.resolved_labels` populate.
    /// - `resolved_text`: texto pré-computed pelo walk arm
    ///   (`"Secção 1.2"`, `"Equação (3)"`, `"Figura 5"`, ou vazio).
    ///   `Option` porque walk arm pode não conseguir resolver para
    ///   alguns target types (per match `_ => None` actual).
    /// - `figure_number`: `Some(n)` apenas quando target é Figure
    ///   numerada+captioned; `None` caso contrário. Usado para
    ///   popular `intr.figure_label_numbers` em paralelo com P168
    ///   arm Figure (write redundante mas inofensivo).
    ///
    /// `from_tags` arm Labelled (P195C) popula ambos sub-stores.
    /// Walk arm legacy (E4 P189B) **mantém** mutação directa em
    /// `state.resolved_labels` + `state.figure_label_numbers`
    /// como write paralelo durante janela compat M5; cleanup em M6.
    Labelled {
        label:         Label,
        resolved_text: Option<String>,
        figure_number: Option<usize>,
    },

    /// **P198C** — payload de `Content::CounterUpdate` (key + action).
    /// Promote `Content::CounterUpdate` a locatable em P198C
    /// (cenário β-promote ADR-0069). `extract_payload` emite este
    /// payload pré-recursão; `from_tags` arm CounterUpdate aplica
    /// à `CounterRegistry` via `apply_at` (flat) ou
    /// `apply_hierarchical_at` (key="heading").
    ///
    /// Campos:
    /// - `key`: chave do counter (`"heading"`, `"equation"`, `"page"`, ...).
    /// - `action`: operação a aplicar (`Step` ou `Update(usize)`).
    ///
    /// Walk arm legacy (E6 P189B) **mantém** mutação directa em
    /// `state.step_*` / `state.update_flat` como write paralelo M5
    /// porque `compute_*` helpers (P195D Equation, P196B Heading,
    /// P197B Figure) lêem counters durante walk; cleanup em M6.
    CounterUpdate {
        key:    String,
        action: CounterUpdate,
    },

    /// **P200B** (M5 universal completo) — Tag derivada de Heading
    /// para popular sub-store `intr.headings_for_toc`. Emitida
    /// pelo walk arm Heading pós-recursão (3ª Tag depois de
    /// Heading + Labelled auto-toc P196B; mesma `emitted_loc`).
    /// `from_tags` arm `HeadingForToc` faz push directo em
    /// `intr.headings_for_toc`. Fecha **E2-residuo** (lacuna #3
    /// declarada desde P189B/P196B) e completa estruturalmente E2
    /// (4ª mutação).
    ///
    /// Campos:
    /// - `label`: auto-label sintetizada `"auto-toc-N"` (paralela
    ///   à utilizada em `resolved_labels` P195D para reference).
    /// - `body`: Content materializado (com counters resolvidos
    ///   via `materialize_time`). Outline render usa este body
    ///   para preservar formatação original do título.
    /// - `level`: nível do heading (1-based; `usize` per paridade
    ///   com `state.headings_for_toc` legacy).
    ///
    /// Mutação 4 legacy (`state.headings_for_toc.push`) preservada
    /// como write paralelo M5 — Layouter assignments
    /// `mod.rs:1490, 1521` dependem; cleanup orgânico em M6.
    HeadingForToc {
        label: Label,
        body:  crate::entities::content::Content,
        level: usize,
    },
}

impl std::hash::Hash for ElementPayload {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Manual Hash via Debug-string — necessário porque Value em
        // Metadata variant não impl Hash. Estratégia consistente com
        // `entities::content_hash::hash_content` (Debug é estrutural
        // determinístico para tipos derive-Debug).
        format!("{:?}", self).hash(state);
    }
}

// `Eq` impl manual (marker trait): Value não é Eq por causa de f64
// NaN, mas em prática nenhum f64 NaN aparece em Value::Float ou
// equivalentes durante o uso normal. Aceite white-lie consistente
// com PartialEq derive (Value::PartialEq tem mesma issue).
// Necessário para downstream types (`ElementInfo`, `Tag`) que
// derivam Eq.
impl Eq for ElementPayload {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heading_constroi_e_compara() {
        let h = ElementPayload::Heading {
            depth: 2,
            body_hash: 0,
            counter_update: CounterUpdate::Step,
        };
        let h2 = h.clone();
        assert_eq!(h, h2);
    }

    #[test]
    fn figure_kind_none_distinto_de_some() {
        let a = ElementPayload::Figure {
            kind: None,
            counter_update: CounterUpdate::Step,
            is_counted: false,
        };
        let b = ElementPayload::Figure {
            kind: Some("image".into()),
            counter_update: CounterUpdate::Step,
            is_counted: false,
        };
        assert_ne!(a, b);
    }

    #[test]
    fn figure_is_counted_distingue_payloads() {
        // P168: is_counted é parte de igualdade.
        let counted = ElementPayload::Figure {
            kind: Some("image".into()),
            counter_update: CounterUpdate::Step,
            is_counted: true,
        };
        let uncounted = ElementPayload::Figure {
            kind: Some("image".into()),
            counter_update: CounterUpdate::Step,
            is_counted: false,
        };
        assert_ne!(counted, uncounted);
    }

    #[test]
    fn citation_compara_por_key() {
        let a = ElementPayload::Citation { key: "smith2024".into() };
        let b = ElementPayload::Citation { key: "smith2024".into() };
        let c = ElementPayload::Citation { key: "jones2023".into() };
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn outline_e_unit_e_distinto_de_outras() {
        // P178: Outline é unit variant.
        let o1 = ElementPayload::Outline;
        let o2 = ElementPayload::Outline;
        assert_eq!(o1, o2);
        // Distinto de outras variants.
        assert_ne!(o1, ElementPayload::Citation { key: "x".into() });
    }

    #[test]
    fn variantes_distintas_entre_si() {
        let h = ElementPayload::Heading {
            depth: 1,
            body_hash: 0,
            counter_update: CounterUpdate::Step,
        };
        let f = ElementPayload::Figure {
            kind: None,
            counter_update: CounterUpdate::Step,
            is_counted: false,
        };
        let c = ElementPayload::Citation { key: "x".into() };
        assert_ne!(h, f);
        assert_ne!(f, c);
        assert_ne!(h, c);
    }

    #[test]
    fn hash_distingue_payloads_distintos() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let a = ElementPayload::Citation { key: "a".into() };
        let b = ElementPayload::Citation { key: "b".into() };
        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        a.hash(&mut h1);
        b.hash(&mut h2);
        assert_ne!(h1.finish(), h2.finish());
    }

    // ── P181C — Bibliography variant ────────────────────────────────────

    fn bib_entry(key: &str) -> crate::entities::bib_entry::BibEntry {
        crate::entities::bib_entry::BibEntry {
            key:          key.to_string(),
            author:       String::new(),
            title:        String::new(),
            year:         0,
            volume:       None,
            pages:        None,
            journal:      None,
            publisher:    None,
            url:          None,
            doi:          None,
            editor:       None,
            series:       None,
            note:         None,
            isbn:         None,
            location:     None,
            organization: None,
        }
    }

    #[test]
    fn bibliography_constroi_e_compara() {
        let entries = vec![bib_entry("smith2024"), bib_entry("jones2023")];
        let p1 = ElementPayload::Bibliography { entries: entries.clone() };
        let p2 = ElementPayload::Bibliography { entries };
        assert_eq!(p1, p2);
    }

    #[test]
    fn bibliography_distinto_de_outras_variants() {
        let bib = ElementPayload::Bibliography { entries: vec![] };
        let outline = ElementPayload::Outline;
        let cite = ElementPayload::Citation { key: "x".into() };
        assert_ne!(bib, outline);
        assert_ne!(bib, cite);
    }

    #[test]
    fn bibliography_hash_diferente_para_entries_distintos() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let a = ElementPayload::Bibliography { entries: vec![bib_entry("a")] };
        let b = ElementPayload::Bibliography { entries: vec![bib_entry("b")] };
        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        a.hash(&mut h1);
        b.hash(&mut h2);
        assert_ne!(h1.finish(), h2.finish());
    }

    // ── P186B — Equation variant ────────────────────────────────────────

    #[test]
    fn equation_constroi_e_compara() {
        let a = ElementPayload::Equation {
            block:          true,
            counter_update: CounterUpdate::Step,
        };
        let b = a.clone();
        assert_eq!(a, b);
    }

    #[test]
    fn equation_block_distingue_payloads() {
        let display = ElementPayload::Equation {
            block:          true,
            counter_update: CounterUpdate::Step,
        };
        let inline = ElementPayload::Equation {
            block:          false,
            counter_update: CounterUpdate::Step,
        };
        assert_ne!(display, inline);
    }

    #[test]
    fn equation_distinto_de_outras_variants() {
        let eq = ElementPayload::Equation {
            block:          true,
            counter_update: CounterUpdate::Step,
        };
        let fig = ElementPayload::Figure {
            kind:           None,
            counter_update: CounterUpdate::Step,
            is_counted:     false,
        };
        let outline = ElementPayload::Outline;
        let cite = ElementPayload::Citation { key: "x".into() };
        assert_ne!(eq, fig);
        assert_ne!(eq, outline);
        assert_ne!(eq, cite);
    }

    #[test]
    fn equation_hash_diferente_para_block_distinto() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let a = ElementPayload::Equation {
            block:          true,
            counter_update: CounterUpdate::Step,
        };
        let b = ElementPayload::Equation {
            block:          false,
            counter_update: CounterUpdate::Step,
        };
        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        a.hash(&mut h1);
        b.hash(&mut h2);
        assert_ne!(h1.finish(), h2.finish());
    }

    // ── P195B — Labelled variant ────────────────────────────────────────

    fn lbl(s: &str) -> Label {
        Label(s.to_string())
    }

    #[test]
    fn labelled_construivel_e_compara() {
        let a = ElementPayload::Labelled {
            label:         lbl("intro"),
            resolved_text: Some("Capítulo 1".to_string()),
            figure_number: None,
        };
        let b = a.clone();
        assert_eq!(a, b);
    }

    #[test]
    fn labelled_distincao_de_outras_variants() {
        let labelled = ElementPayload::Labelled {
            label:         lbl("intro"),
            resolved_text: Some("Capítulo 1".to_string()),
            figure_number: None,
        };
        let equation = ElementPayload::Equation {
            block:          true,
            counter_update: CounterUpdate::Step,
        };
        let cite = ElementPayload::Citation { key: "k".into() };
        assert_ne!(labelled, equation);
        assert_ne!(labelled, cite);
    }

    #[test]
    fn labelled_distingue_por_label() {
        let a = ElementPayload::Labelled {
            label:         lbl("intro"),
            resolved_text: Some("Secção 1".to_string()),
            figure_number: None,
        };
        let b = ElementPayload::Labelled {
            label:         lbl("conclusao"),
            resolved_text: Some("Secção 1".to_string()),
            figure_number: None,
        };
        assert_ne!(a, b);
    }

    #[test]
    fn labelled_hash_diferente_para_label_distinto() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let a = ElementPayload::Labelled {
            label:         lbl("a"),
            resolved_text: Some("text".to_string()),
            figure_number: None,
        };
        let b = ElementPayload::Labelled {
            label:         lbl("b"),
            resolved_text: Some("text".to_string()),
            figure_number: None,
        };
        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        a.hash(&mut h1);
        b.hash(&mut h2);
        assert_ne!(h1.finish(), h2.finish());
    }

    #[test]
    fn labelled_figure_number_distingue_payloads() {
        let sem_figura = ElementPayload::Labelled {
            label:         lbl("ref1"),
            resolved_text: Some("text".to_string()),
            figure_number: None,
        };
        let com_figura = ElementPayload::Labelled {
            label:         lbl("ref1"),
            resolved_text: Some("text".to_string()),
            figure_number: Some(3),
        };
        assert_ne!(sem_figura, com_figura);
    }
}
