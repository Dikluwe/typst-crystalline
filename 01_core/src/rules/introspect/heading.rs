//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/atomizacao_elementos.md
//! @prompt-hash 3331d6ba
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P383): a lógica de introspeção por-elemento de
//! `Heading` (`compute_heading_auto_toc` + `compute_heading_for_toc`) movida do
//! tronco `introspect.rs` para o arquivo do elemento, na convenção do submódulo
//! `rules/introspect/`. Content-preserving — chamadas pelo walk arm `Heading`.

use crate::entities::content::Content;
use crate::entities::introspector::Introspector;
use crate::entities::label::Label;
use crate::entities::location::Location;

/// Computa `(auto_label, resolved_text)` para a auto-toc de um `Heading`
/// (ADR-0069/P196B). Função pura sobre `(intr, location, auto_label_n,
/// numbering_active)` — sem mutação. Sempre retorna `(Label, String)`
/// (resolved_text vazio quando numbering inactivo — paridade legacy).
pub(super) fn compute_heading_auto_toc<I: Introspector>(
    intr:             &I,
    location:         Location,
    auto_label_n:     usize,
    numbering_active: bool,
) -> (Label, String) {
    let auto_label = Label(format!("auto-toc-{}", auto_label_n));
    // Lote F-2 S5 (P335): gate pelo `numbering_active` **assado** no
    // `HeadingElem` (escopo léxico via chain).
    let resolved_text = if numbering_active {
        // P359 (DEBT-60 b): o número do heading, **sem** o supplement "Secção" — o
        // outline mostra o numbering (paridade vanilla). Formato `{n}.` espelha o
        // corpo do heading (o nº do outline == o nº do corpo).
        intr.formatted_counter_at("heading", location)
            .map(|n| format!("{}.", n))
            .unwrap_or_default()
    } else {
        String::new()
    };
    (auto_label, resolved_text)
}

/// Projecta a entry de outline para um `Heading` (ADR-0069/P200B). Função pura
/// sobre `(auto_label_n, frozen_body, level)` — sem mutação. `frozen_body` já
/// materializado pelo walk arm Heading. Sempre retorna `Some(...)` (paridade
/// com o push incondicional legacy).
pub(super) fn compute_heading_for_toc(
    auto_label_n: usize,
    frozen_body:  Content,
    level:        usize,
) -> Option<(Label, Content, usize)> {
    let auto_label = Label(format!("auto-toc-{}", auto_label_n));
    Some((auto_label, frozen_body, level))
}
