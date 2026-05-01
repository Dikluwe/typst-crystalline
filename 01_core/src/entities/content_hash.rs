//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/content_hash.md
//! @prompt-hash e1e9d070
//! @layer L1
//! @updated 2026-04-30
//!
//! `hash_content` — função pura que produz `u128` determinístico
//! sobre `Content`. P162 sub-passo .B (resolve pendência herdada
//! de P161 sobre `body_hash` placeholder em `ElementPayload`).
//!
//! Implementação minimalista: serializa via `format!("{:?}", c)` —
//! Debug derive em Content é estrutural recursivo. Hash duas vezes
//! com sementes distintas para obter 128 bits de output. Sem
//! dependência externa nova; usa std::hash::DefaultHasher.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::entities::content::Content;

/// Hash determinístico de 128 bits sobre Content.
///
/// Determinístico dentro da mesma versão de Rust+std. Dois Contents
/// que são `==` produzem o mesmo u128.
pub fn hash_content(content: &Content) -> u128 {
    let serialized = format!("{:?}", content);
    let lo = hash_with_seed(&serialized, 0);
    let hi = hash_with_seed(&serialized, 1);
    ((hi as u128) << 64) | (lo as u128)
}

fn hash_with_seed(s: &str, seed: u8) -> u64 {
    let mut h = DefaultHasher::new();
    seed.hash(&mut h);
    s.hash(&mut h);
    h.finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecow::EcoString;

    fn text(s: &str) -> Content {
        Content::Text(EcoString::from(s), Default::default())
    }

    #[test]
    fn iguais_produzem_mesmo_hash() {
        let a = text("hello");
        let b = text("hello");
        assert_eq!(hash_content(&a), hash_content(&b));
    }

    #[test]
    fn distintos_produzem_hashes_distintos() {
        let a = text("a");
        let b = text("b");
        let c = Content::Empty;
        let d = Content::Space;
        let e = Content::Heading {
            level: 1,
            body: Box::new(text("title")),
        };
        let hashes = [
            hash_content(&a),
            hash_content(&b),
            hash_content(&c),
            hash_content(&d),
            hash_content(&e),
        ];
        // Todos os 5 distintos entre si.
        for i in 0..hashes.len() {
            for j in (i + 1)..hashes.len() {
                assert_ne!(
                    hashes[i], hashes[j],
                    "Contents distintos #{i} e #{j} produziram hash igual"
                );
            }
        }
    }

    #[test]
    fn determinismo_em_100_chamadas() {
        let c = Content::Heading {
            level: 2,
            body: Box::new(text("section")),
        };
        let h0 = hash_content(&c);
        for _ in 0..100 {
            assert_eq!(hash_content(&c), h0);
        }
    }

    #[test]
    fn clone_preserva_hash() {
        let c = text("clone-test");
        let cloned = c.clone();
        assert_eq!(hash_content(&c), hash_content(&cloned));
    }

    #[test]
    fn variantes_estruturalmente_distintas_produzem_hashes_distintos() {
        let h1 = Content::Heading { level: 1, body: Box::new(Content::Empty) };
        let h2 = Content::Heading { level: 2, body: Box::new(Content::Empty) };
        assert_ne!(hash_content(&h1), hash_content(&h2));
    }
}
