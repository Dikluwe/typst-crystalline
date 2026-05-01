//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/introspect/convergence.md
//! @prompt-hash 6d658703
//! @layer L1
//! @updated 2026-04-29
//!
//! `compute_tags_hash` — helper de detecção de convergência para o
//! mecanismo de fixpoint (P174 sub-passo .C / M7 sub-passo 1).
//!
//! `Vec<Tag>` (output do walk em `rules/introspect.rs::walk`) é a
//! representação canónica do que vai virar `TagIntrospector`. Hash
//! determinístico sobre tags = detecção de convergência: se hash da
//! iter N é igual ao hash da iter N-1, fixpoint convergiu.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::entities::tag::Tag;

/// Hash determinístico de uma sequência de tags.
///
/// Usado por `run_fixpoint` (P174 sub-passo .D) para detectar
/// convergência entre iterações. `Tag` deriva `Hash` (P162); slice
/// implementa `Hash` automaticamente.
///
/// `DefaultHasher` é SipHash-1-3 — qualidade suficiente para
/// detecção de mudança estrutural (não criptográfico). Colisão
/// teórica com probabilidade desprezável (~2⁻⁶⁴).
pub fn compute_tags_hash(tags: &[Tag]) -> u64 {
    let mut hasher = DefaultHasher::new();
    tags.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::counter_update::CounterUpdate;
    use crate::entities::element_info::ElementInfo;
    use crate::entities::element_payload::ElementPayload;
    use crate::entities::location::Location;

    fn loc(raw: u128) -> Location {
        Location::from_raw(raw)
    }

    fn heading_payload() -> ElementPayload {
        ElementPayload::Heading {
            depth:          1,
            body_hash:      0,
            counter_update: CounterUpdate::Step,
        }
    }

    #[test]
    fn vazio_consistente() {
        let h1 = compute_tags_hash(&[]);
        let h2 = compute_tags_hash(&[]);
        assert_eq!(h1, h2);
    }

    #[test]
    fn tags_identicas_produzem_mesmo_hash() {
        let tags1 = vec![
            Tag::Start(loc(1), ElementInfo::new(heading_payload())),
            Tag::End(loc(1), 0xdead),
        ];
        let tags2 = vec![
            Tag::Start(loc(1), ElementInfo::new(heading_payload())),
            Tag::End(loc(1), 0xdead),
        ];
        assert_eq!(compute_tags_hash(&tags1), compute_tags_hash(&tags2));
    }

    #[test]
    fn payload_diferente_produz_hash_diferente() {
        let tags1 = vec![Tag::Start(loc(1), ElementInfo::new(heading_payload()))];
        let figure_payload = ElementPayload::Figure {
            kind:           Some("image".into()),
            counter_update: CounterUpdate::Step,
            is_counted:     true,
        };
        let tags2 = vec![Tag::Start(loc(1), ElementInfo::new(figure_payload))];
        assert_ne!(compute_tags_hash(&tags1), compute_tags_hash(&tags2));
    }

    #[test]
    fn locations_diferentes_produzem_hash_diferente() {
        let tags1 = vec![Tag::Start(loc(1), ElementInfo::new(heading_payload()))];
        let tags2 = vec![Tag::Start(loc(2), ElementInfo::new(heading_payload()))];
        assert_ne!(compute_tags_hash(&tags1), compute_tags_hash(&tags2));
    }

    #[test]
    fn ordem_das_tags_afecta_hash() {
        let t1 = Tag::Start(loc(1), ElementInfo::new(heading_payload()));
        let t2 = Tag::End(loc(1), 0xbeef);
        let tags_ordem_a = vec![t1.clone(), t2.clone()];
        let tags_ordem_b = vec![t2, t1];
        assert_ne!(
            compute_tags_hash(&tags_ordem_a),
            compute_tags_hash(&tags_ordem_b),
            "Vec<Tag>::hash deve ser sensível à ordem (slice hash inclui length + items in order)"
        );
    }
}
