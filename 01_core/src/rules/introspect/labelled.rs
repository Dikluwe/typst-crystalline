//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/atomizacao_elementos.md
//! @prompt-hash 3331d6ba
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P383): a lógica de introspeção por-elemento de
//! `Labelled` (`compute_labelled`) movida do tronco `introspect.rs` para o
//! arquivo do elemento, na convenção do submódulo `rules/introspect/`.
//! Content-preserving — chamada pelo walk arm `Labelled`.

use crate::entities::content::Content;
use crate::entities::introspector::Introspector;
use crate::entities::location::Location;

/// Computa `(resolved_text, figure_number)` para uma `Label` apontando a um
/// `target` locatável (Heading/Equation/Figure), via o `Introspector`
/// location-aware. Função pura — sem mutação. (ADR-0069/P195D/P191C.)
pub(super) fn compute_labelled<I: Introspector>(
    intr:     &I,
    location: Location,
    target:   &Content,
    lang:     Option<&crate::entities::lang::Lang>,
) -> (Option<String>, Option<usize>) {
    match target {
        Content::Heading(_) => (
            intr.formatted_counter_at("heading", location)
                .map(|n| format!("Secção {}", n)),
            None,
        ),
        Content::Equation(e) if e.block => {
            let n = intr
                .flat_counter_at("equation", location)
                .unwrap_or(0);
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
            let n = intr
                .flat_counter_at(&format!("figure:{}", kind_key), location)
                .unwrap_or(0);
            if n > 0 {
                let supplement = crate::rules::lang::figure_supplement::figure_supplement_for_lang(
                    kind_key,
                    lang,
                );
                (Some(format!("{} {}", supplement, n)), Some(n))
            } else {
                (Some(String::new()), None)
            }
        }
        _ => (None, None),
    }
}
