//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/atomizacao_elementos.md
//! @prompt-hash a54abe5a
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

/// Formata o valor hierárquico de um counter como string terminada em ponto.
///
/// Usado para o número do outline (DEBT-60b / P428): emite `"1."`, `"1.1."`,
/// etc., sem supplement. Retorna `None` se a numeração estiver inactiva ou o
/// counter não tiver valor no momento da `location`.
fn format_heading_number<I: Introspector>(
    intr: &I,
    location: Location,
    numbering_active: bool,
) -> Option<String> {
    if !numbering_active {
        return None;
    }
    intr.formatted_counter_at("heading", location)
        .map(|n| format!("{}.", n))
}

/// Computa `(auto_label, resolved_text)` para a auto-toc de um `Heading`
/// (ADR-0069/P196B). Função pura sobre `(intr, location, auto_label_n,
/// numbering_active)` — sem mutação. Sempre retorna `(Label, String)`
/// (resolved_text vazio quando numbering inactivo — paridade legacy).
pub(super) fn compute_heading_auto_toc<I: Introspector>(
    intr: &I,
    location: Location,
    auto_label_n: usize,
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
/// sobre `(intr, location, auto_label_n, frozen_body, level, numbering_active)`
/// — sem mutação. `frozen_body` já materializado pelo walk arm Heading. O
/// `number` é computado a partir do counter do Introspector, separando-o do
/// body e do supplement (DEBT-60b / P428). Sempre retorna `Some(...)` (paridade
/// com o push incondicional legacy).
pub(super) fn compute_heading_for_toc<I: Introspector>(
    intr: &I,
    location: Location,
    auto_label_n: usize,
    frozen_body: Content,
    level: usize,
    numbering_active: bool,
) -> Option<(Label, Option<String>, Content, usize)> {
    let auto_label = Label(format!("auto-toc-{}", auto_label_n));
    let number = format_heading_number(intr, location, numbering_active);
    Some((auto_label, number, frozen_body, level))
}
