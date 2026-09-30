//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/introspect/parent_index.md
//! @prompt-hash 110cf959
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// **P504** — Constrói mapa `Location → Location` do parent imediato a
/// partir da sequência `Start`/`End` de tags. Cada `Start` empilha a sua
/// Location; `End` desempilha. Filhos são registados contra o topo da
/// pilha no momento do seu `Start`.
pub(super) fn build_parent_index(tags: &[Tag]) -> HashMap<Location, Location> {
    use crate::entities::element_payload::ElementPayload;
    use crate::entities::tag::Tag;
    let mut parent_map = HashMap::new();
    let mut stack: Vec<Location> = Vec::new();
    for tag in tags {
        match tag {
            Tag::Start(loc, info) => {
                // P504: só nós locatable reais (Heading, Figure, etc.) são
                // entradas de parentesco. Tags pós-recursão (Labelled,
                // HeadingForToc) reutilizam a mesma Location e não devem
                // sobrescrever o parent do nó real.
                if matches!(
                    info.payload,
                    ElementPayload::Heading { .. }
                        | ElementPayload::NativeElement
                        | ElementPayload::Figure { .. }
                        | ElementPayload::Citation { .. }
                        | ElementPayload::Metadata { .. }
                        | ElementPayload::State { .. }
                        | ElementPayload::StateUpdate { .. }
                        | ElementPayload::Outline
                        | ElementPayload::Bibliography { .. }
                        | ElementPayload::Equation { .. }
                        | ElementPayload::Table { .. }
                        | ElementPayload::CounterUpdate { .. }
                ) {
                    parent_map.insert(*loc, stack.last().copied());
                }
                stack.push(*loc);
            }
            Tag::End(loc, _) => {
                if stack.last() == Some(loc) {
                    stack.pop();
                }
            }
        }
    }
    parent_map
        .into_iter()
        .filter_map(|(loc, parent)| parent.map(|p| (loc, p)))
        .collect()
}
