//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/state_update.md
//! @prompt-hash d244bda8
//! @layer L1
//! @updated 2026-04-30
//!
//! `StateUpdate` — operação a aplicar a um state durante a passagem
//! de introspecção. P171 (M9 sub-passo 3).
//!
//! Vanilla `state.rs::StateUpdate { Set(Value), Func(Func) }`.
//! Cristalino P171 implementa apenas `Set` — callbacks (`Func`)
//! adiadas para passo M9+ quando Func eval em walk context for
//! materializado.

use crate::entities::value::Value;

/// Operação sobre um state. M9 sub-passo 3 (P171): apenas `Set`.
/// Variant `Func` reservada para passo futuro.
///
/// `Box<Value>` evita ciclo Content→Value→Content (o mesmo padrão de
/// `Content::Metadata { value: Box<Value> }` em P169).
#[derive(Debug, Clone, PartialEq)]
pub enum StateUpdate {
    /// Define o valor do state.
    Set(Box<Value>),
    // Func(Func) — adiado para passo M9+ com mecanismo de eval em walk.
}

impl std::hash::Hash for StateUpdate {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Manual Hash via Debug-string — Value contém f64 sem Hash.
        // Padrão consistente com `entities/element_payload.rs` P169.
        format!("{:?}", self).hash(state);
    }
}

impl Eq for StateUpdate {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_round_trip() {
        let u = StateUpdate::Set(Box::new(Value::Int(42)));
        let u2 = u.clone();
        assert_eq!(u, u2);
    }

    #[test]
    fn set_distintos_por_value() {
        let a = StateUpdate::Set(Box::new(Value::Int(1)));
        let b = StateUpdate::Set(Box::new(Value::Int(2)));
        assert_ne!(a, b);
    }

    #[test]
    fn hash_determinismo() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let u = StateUpdate::Set(Box::new(Value::Int(7)));
        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        u.hash(&mut h1);
        u.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }
}
