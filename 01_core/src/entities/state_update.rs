//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/state_update.md
//! @prompt-hash 4caed633
//! @layer L1
//! @updated 2026-04-30
//!
//! Operação pura de State: registrada no índice e aplicada quando a chave é lida.
//! O init pertence ao objeto leitor; este enum não executa callbacks.

use std::sync::Arc;

use crate::entities::func::Func;
use crate::entities::value::Value;

/// Operação sobre um state. Set substitui o valor; Func transforma o acumulador.
/// A aplicação automática pertence ao resolver de leitura, não ao walk.
#[derive(Debug, Clone)]
pub enum StateUpdate {
    /// Define o valor do state.
    Set(Box<Value>),
    /// Callback com o valor corrente como argumento único.
    Func(Func),
}

// `PartialEq` manual: `Func` interno é `Arc<FuncRepr>` que não impl
// `PartialEq`. Comparação por ponteiro Arc — duas Funcs distintas com
// mesmo comportamento são `!=`. Esta igualdade Rust não define a igualdade da linguagem.
impl PartialEq for StateUpdate {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (StateUpdate::Set(a), StateUpdate::Set(b)) => a == b,
            (StateUpdate::Func(a), StateUpdate::Func(b)) => Arc::ptr_eq(&a.0, &b.0),
            (StateUpdate::Set(_), StateUpdate::Func(_))
            | (StateUpdate::Func(_), StateUpdate::Set(_)) => false,
        }
    }
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
    fn v16_state_update_function_identity_remains_stricter_than_func_names() {
        let first = Func::native("same", |_, _, _, _| Ok(Value::None));
        let second = Func::native("same", |_, _, _, _| Ok(Value::None));
        assert_eq!(first, second);
        let first_update = StateUpdate::Func(first.clone());
        let alias = StateUpdate::Func(first);
        let second_update = StateUpdate::Func(second);
        assert_eq!(first_update, alias);
        assert_ne!(first_update, second_update);
        assert_ne!(second_update, first_update);
        let set = StateUpdate::Set(Box::new(Value::None));
        assert_ne!(set, first_update);
        assert_ne!(first_update, set);
    }

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

    // ── P172 (M9 sub-passo 4) — Func variant ─────────────────────────

    #[test]
    fn func_variant_construi_e_compara_por_arc_ptr_eq() {
        let f = Func::native("test_fn", |_, _, _, _| Ok(Value::Int(0)));
        let u1 = StateUpdate::Func(f.clone());
        let u2 = StateUpdate::Func(f.clone());
        // Mesmo Arc → iguais.
        assert_eq!(u1, u2);
    }

    #[test]
    fn func_variants_distintas_sao_diferentes() {
        // Duas Funcs nativas separadas têm Arcs diferentes mesmo com
        // mesmo comportamento.
        let f1 = Func::native("f1", |_, _, _, _| Ok(Value::Int(0)));
        let f2 = Func::native("f2", |_, _, _, _| Ok(Value::Int(0)));
        let u1 = StateUpdate::Func(f1);
        let u2 = StateUpdate::Func(f2);
        assert_ne!(u1, u2);
    }

    #[test]
    fn set_e_func_sao_distintos() {
        let f = Func::native("test", |_, _, _, _| Ok(Value::Int(0)));
        let set = StateUpdate::Set(Box::new(Value::Int(0)));
        let func_var = StateUpdate::Func(f);
        assert_ne!(set, func_var);
    }
}
