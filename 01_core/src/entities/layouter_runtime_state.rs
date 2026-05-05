//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/layouter_runtime_state.md
//! @layer L1
//! @updated 2026-05-04
//!
//! **P190C** — struct dedicada para state Layouter-runtime que **não é
//! derivado de Content pre-pass**. Diferente de `TagIntrospector` (state
//! derivado de walk + from_tags pipeline), este struct é populated
//! durante o layout pelo Layouter — campos têm semântica de página
//! (`label_pages`, `known_page_numbers`) que só existem em runtime de
//! render, não em introspecção.
//!
//! Pattern arquitectural estabelecido em P190C: "Layouter-runtime →
//! struct dedicada" — replicado para outros campos Layouter-runtime
//! (`is_readonly`, `lang`) em P190D.
//!
//! Cross-references:
//! - **P190A §3 achado crítico** — 4 campos Layouter-runtime
//!   identificados como não cabendo em Introspector.
//! - **P190B** — pattern "eliminação write paralelo M5" 1ª aplicação
//!   (Bibliography); P190C 2ª aplicação introduz padrão Layouter-runtime.
//! - **DEBT-12** — page tracking via `known_page_numbers` (Pass 3 final).
//! - **DEBT-13** — outline freeze via `is_readonly` (P190D).

use std::collections::HashMap;

use crate::entities::label::Label;

/// State Layouter-runtime — campos populated durante o layout
/// (não derivados de Content pre-pass).
///
/// Usado pelo `Layouter<M, S>::runtime` field para isolar state
/// que não cabe em `TagIntrospector` (não é derivado de walk +
/// from_tags pipeline).
#[derive(Debug, Default, Clone)]
pub struct LayouterRuntimeState {
    /// Mapeia label para número de página onde foi resolvida.
    /// Populated por `references.rs` durante layout (write).
    /// Lido por `mod.rs:layout()` no fim para preencher
    /// `PagedDocument.extracted_label_pages`.
    pub label_pages: HashMap<Label, usize>,

    /// Page numbers conhecidos da iteração anterior do fixpoint
    /// (referência para `outline.rs` resolver "  N" em entries TOC).
    /// Populated por `mod.rs:layout()` no início de cada iteração
    /// (Pass 2 vazio; Pass 3+ com dados Pass anterior).
    /// **DEBT-12**: page tracking single-pass não suporta refs
    /// para a frente em iteração 0.
    pub known_page_numbers: HashMap<Label, usize>,

    /// **P190D** — modo read-only do Layouter (DEBT-13).
    /// Quando `true`, `layout_counter_update` retorna early sem
    /// avançar contadores. Usado por `outline.rs` durante render
    /// das entries TOC para impedir que `Content::CounterUpdate`
    /// embebido nos clones de heading-body avance contadores.
    /// Set/unset por `outline.rs:73-76` em volta de
    /// `layouter.layout_content(&line)`.
    pub is_readonly: bool,
}
