//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/tag.md
//! @prompt-hash 399e1b67
//! @layer L1
//! @updated 2026-04-30
//!
//! `Tag` — marcador de início/fim de elemento indexável durante a
//! passagem de introspecção. P161 sub-passo .9.
//!
//! P161 só cria a definição. Walk não emite tags ainda — isso é
//! P162.

use crate::entities::element_info::ElementInfo;
use crate::entities::location::Location;

/// Marcador Start/End de elemento durante o walk de introspecção.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Tag {
    /// Início de um elemento indexável. `Location` única (gerada
    /// por `Locator`); `ElementInfo` carrega payload + label.
    Start(Location, ElementInfo),

    /// Fim do elemento. `Location` corresponde ao `Start`
    /// emparelhado; `u128` é content_hash (paridade vanilla,
    /// pendência P162 para a função de hash).
    End(Location, u128),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::counter_update::CounterUpdate;
    use crate::entities::element_payload::ElementPayload;

    fn loc(raw: u128) -> Location {
        Location::from_raw(raw)
    }

    fn info() -> ElementInfo {
        ElementInfo::new(ElementPayload::Heading {
            depth: 1,
            body_hash: 0,
            counter_update: CounterUpdate::Step,
        })
    }

    #[test]
    fn start_e_end_distintos() {
        let s = Tag::Start(loc(1), info());
        let e = Tag::End(loc(1), 0);
        assert_ne!(s, e);
    }

    #[test]
    fn start_iguais_se_location_e_info_iguais() {
        let s1 = Tag::Start(loc(7), info());
        let s2 = Tag::Start(loc(7), info());
        assert_eq!(s1, s2);
    }

    #[test]
    fn end_compara_por_location_e_hash() {
        let a = Tag::End(loc(2), 0xABCD);
        let b = Tag::End(loc(2), 0xABCD);
        let c = Tag::End(loc(2), 0xBEEF);
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn clone_preserva_variant_e_dados() {
        let s = Tag::Start(loc(5), info());
        let s2 = s.clone();
        assert_eq!(s, s2);
    }
}
