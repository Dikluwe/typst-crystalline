//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/state_update.md
//! @prompt-hash 1b276c4e
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

use std::sync::Arc;

use crate::entities::func::Func;
use crate::entities::value::Value;

/// Operação sobre um state. P171 (M9 sub-passo 3): `Set`.
/// P172 (M9 sub-passo 4): `Func` variant adicionada.
///
/// `Box<Value>` evita ciclo Content→Value→Content (o mesmo padrão de
/// `Content::Metadata { value: Box<Value> }` em P169).
///
/// **Func variant — eval é stub em P172**: adicionado para completude
/// tipológica. `from_tags` reconhece a variant mas **não avalia** a
/// closure — `Func::call` requer `EvalContext + Engine` que não estão
/// disponíveis em walk nem em from_tags (Engine só existe durante
/// eval; walk e from_tags correm depois). Avaliação real requer
/// restructuring da pipeline (eval+walk integrados ou Engine kept-alive
/// — passo M7 fixpoint, ou refactor dedicado).
///
/// Em P172, encontrar `Func` em from_tags é **silenciosamente
/// ignorado** (comportamento defensivo coerente com P171
/// "update sem init é ignorado"). Stdlib `state_update_with(key, fn)`
/// constrói a variant; o efeito real depende de eval ser adicionada
/// em passo futuro.
#[derive(Debug, Clone)]
pub enum StateUpdate {
    /// Define o valor do state.
    Set(Box<Value>),
    /// **P172** — callback `fn(Value) -> Value`. **Stub em P172** —
    /// from_tags reconhece mas não avalia (sem contexto de eval).
    /// Eval real requer pipeline restructuring (M7+).
    Func(Func),
}

// `PartialEq` manual: `Func` interno é `Arc<FuncRepr>` que não impl
// `PartialEq`. Comparação por ponteiro Arc — duas Funcs distintas com
// mesmo comportamento são `!=` (paridade vanilla onde Func não compara
// estruturalmente).
impl PartialEq for StateUpdate {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (StateUpdate::Set(a),  StateUpdate::Set(b))  => a == b,
            (StateUpdate::Func(a), StateUpdate::Func(b)) => Arc::ptr_eq(&a.0, &b.0),
            _ => false,
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
        let f = Func::native("test_fn", |_, _, _, _, _| Ok(Value::Int(0)));
        let u1 = StateUpdate::Func(f.clone());
        let u2 = StateUpdate::Func(f.clone());
        // Mesmo Arc → iguais.
        assert_eq!(u1, u2);
    }

    #[test]
    fn func_variants_distintas_sao_diferentes() {
        // Duas Funcs nativas separadas têm Arcs diferentes mesmo com
        // mesmo comportamento.
        let f1 = Func::native("f1", |_, _, _, _, _| Ok(Value::Int(0)));
        let f2 = Func::native("f2", |_, _, _, _, _| Ok(Value::Int(0)));
        let u1 = StateUpdate::Func(f1);
        let u2 = StateUpdate::Func(f2);
        assert_ne!(u1, u2);
    }

    #[test]
    fn set_e_func_sao_distintos() {
        let f = Func::native("test", |_, _, _, _, _| Ok(Value::Int(0)));
        let set = StateUpdate::Set(Box::new(Value::Int(0)));
        let func_var = StateUpdate::Func(f);
        assert_ne!(set, func_var);
    }
}
