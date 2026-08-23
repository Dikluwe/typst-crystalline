//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/atomizacao_elementos.md
//! @prompt-hash 59c9666b
//! @layer L1
//! @updated 2026-08-18
//!
//! Atomização (ADR-0109, P383): a lógica de introspeção por-elemento de
//! `Labelled` (`compute_labelled`) movida do tronco `introspect.rs` para o
//! arquivo do elemento, na convenção do submódulo `rules/introspect/`.
//! Content-preserving — chamada pelo walk arm `Labelled`.
//! **P1018** — chaves de counter migradas para `CounterKey`.

use crate::entities::content::Content;
use crate::entities::counter::CounterKey;
use crate::entities::element_kind::ElementKind;
use crate::entities::introspector::Introspector;
use crate::entities::location::Location;
use crate::entities::selector::Selector;

/// Computa `(resolved_text, figure_number)` para uma `Label` apontando a um
/// `target` locatável (Heading/Equation/Figure), via o `Introspector`
/// location-aware. Função pura — sem mutação. (ADR-0069/P195D/P191C.)
pub(super) fn compute_labelled<I: Introspector>(
    intr: &I,
    location: Location,
    target: &Content,
    lang: Option<&crate::entities::lang::Lang>,
) -> (Option<String>, Option<usize>) {
    match target {
        Content::Heading(_) => {
            let heading_key = CounterKey::Selector(Selector::Kind(ElementKind::Heading));
            let supp = if lang.map(|l| l.as_str() == "pt").unwrap_or(false) {
                "Seção"
            } else {
                "Section"
            };
            (
                intr.formatted_counter_at(&heading_key, location)
                    .map(|n| format!("{} {}", supp, n)),
                None,
            )
        }
        Content::Equation(e) if e.block => {
            let equation_key =
                CounterKey::Selector(Selector::Kind(ElementKind::Equation));
            let n = intr.flat_counter_at(&equation_key, location).unwrap_or(0);
            if n > 0 {
                (Some(format!("Equação ({})", n)), None)
            } else {
                (None, None)
            }
        }
        Content::Figure(e) => {
            // F-5a de-bake (P365): o gate (padrão + caption) não é mais lido de um
            // campo — vive no **contador** `figure:{kind}`, que só é avançado quando
            // `is_counted` (padrão-da-chain && caption) na emissão (walk top). Logo
            // `flat_counter_at` location-aware devolve >0 só para figuras numeradas
            // (espelho do arm Equation, que confia no contador). Sem leitura de campo.
            let kind_key = e.kind.as_deref().unwrap_or("image");
            let counter_key = CounterKey::Str(format!("figure:{}", kind_key).into());
            let n = intr.flat_counter_at(&counter_key, location).unwrap_or(0);
            if n > 0 {
                let supplement =
                    crate::compiler::lang::figure_supplement::figure_supplement_for_lang(
                        kind_key, lang,
                    );
                (Some(format!("{} {}", supplement, n)), Some(n))
            } else {
                (Some(String::new()), None)
            }
        }
        _ => (None, None), // neutro: N16[γ] — Content sem suporte de labelling retorna (None, None) (conservador: novos nós rotuláveis exigirão arm)
    }
}
