//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/tag.md
//! @prompt-hash 37add3cc
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
    Start(Location, Box<ElementInfo>),

    /// Fim do elemento. `Location` corresponde ao `Start`
    /// emparelhado; `u128` é content_hash (paridade vanilla,
    /// pendência P162 para a função de hash).
    End(Location, u128),
}

#[cfg(test)]
mod tests {
    #[test]
    fn public_enum_budget_tag_tracks_end_payload() {
        assert!(std::mem::size_of::<Tag>() <= 3 * std::mem::size_of::<Location>());
    }

    #[test]
    fn boxed_tag_clone_keeps_payload_ownership_independent() {
        use crate::entities::label::Label;
        use std::hash::{Hash, Hasher};
        fn hash(tag: &Tag) -> u64 {
            let mut state = std::collections::hash_map::DefaultHasher::new();
            tag.hash(&mut state);
            state.finish()
        }
        let original = Tag::Start(loc(7), Box::new(info()));
        let independent = Tag::Start(loc(7), Box::new(info()));
        assert_eq!(original, independent);
        assert_eq!(hash(&original), hash(&independent));
        let mut copied = original.clone();
        let Tag::Start(location, payload) = &mut copied else { unreachable!() };
        assert_eq!(*location, loc(7));
        payload.label = Some(Label("copied".into()));
        let Tag::Start(_, payload) = &original else { unreachable!() };
        assert_eq!(payload.label, None);
        assert_ne!(original, copied);
    }

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
            counter_update: CounterUpdate::step(),
            numbering_active: false,
        })
    }

    #[test]
    fn start_e_end_distintos() {
        let s = Tag::Start(loc(1), Box::new(info()));
        let e = Tag::End(loc(1), 0);
        assert_ne!(s, e);
    }

    #[test]
    fn start_iguais_se_location_e_info_iguais() {
        let s1 = Tag::Start(loc(7), Box::new(info()));
        let s2 = Tag::Start(loc(7), Box::new(info()));
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
        let s = Tag::Start(loc(5), Box::new(info()));
        let s2 = s.clone();
        assert_eq!(s, s2);
    }
}
