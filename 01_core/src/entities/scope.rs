//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/scope.md
//! @prompt-hash c25317a8
//! @layer L1
//! @updated 2026-11-08

use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

use crate::entities::value::Value;

/// **P772q** — por que motivo um scope foi capturado (closure vs bloco
/// `context`). Paridade vanilla `foundations/scope.rs::Capturer`. Um único
/// valor por invocação (não uma flag por-binding como `BindingKind`, ainda
/// omitido — ADR-0017): guardado em `ClosureRepr.capturer`
/// (`entities/func.rs`) e propagado para `Scopes.captured_by`
/// (`rules/scopes.rs`) no momento da chamada.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Capturer {
    /// Capturado por uma closure normal (`#let f() = { ... }`).
    Function,
    /// Capturado por um bloco `context { ... }`.
    Context,
}

/// Valor ligado a um nome num Scope.
///
/// Mantido acoplado ao stub Value::None neste passo.
/// Campos adicionais do original (kind, span, category, deprecation) são
/// adicionados quando Value real migrar — não antecipar. (ADR-0017)
#[derive(Debug, Clone)]
pub struct Binding {
    value: Value,
}

impl Binding {
    pub fn new(value: Value) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    /// P715 — acesso mutável ao valor, para atribuição (`x = v`,
    /// desestruturação em atribuição).
    pub fn value_mut(&mut self) -> &mut Value {
        &mut self.value
    }

    pub fn into_value(self) -> Value {
        self.value
    }
}

/// Âmbito de nomes do compilador Typst.
///
/// Usa IndexMap para preservar ordem de inserção — a ordem de declaração
/// de bindings em Typst é semanticamente significativa (ADR-0023).
/// Hasher: FxBuildHasher de rustc_hash (ADR-0018) — rápido para
/// identificadores curtos.
#[derive(Debug, Clone)]
pub struct Scope {
    map: IndexMap<String, Binding, FxBuildHasher>,
}

impl Scope {
    pub fn new() -> Self {
        Self { map: IndexMap::with_hasher(FxBuildHasher) }
    }

    /// Cria um scope vazio com espaço para pelo menos `capacity` bindings.
    pub fn with_capacity(capacity: usize) -> Self {
        Self { map: IndexMap::with_capacity_and_hasher(capacity, FxBuildHasher) }
    }

    /// Define um binding. Se o nome já existe, substitui no lugar
    /// (mantendo a posição na ordem de inserção).
    pub fn define(&mut self, name: impl Into<String>, value: Value) {
        self.map.insert(name.into(), Binding::new(value));
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.map.get(name).map(|b| b.value())
    }

    /// P715 — acesso mutável a um binding existente, para atribuição.
    /// Não cria o binding se ausente (distinto de `define`).
    pub fn get_mut(&mut self, name: &str) -> Option<&mut Value> {
        self.map.get_mut(name).map(|b| b.value_mut())
    }

    pub fn get_binding(&self, name: &str) -> Option<&Binding> {
        self.map.get(name)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &Binding)> {
        self.map.iter().map(|(k, v)| (k.as_str(), v))
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::value::Value;

    #[test]
    fn define_e_get() {
        let mut scope = Scope::new();
        scope.define("x", Value::None);
        assert!(scope.get("x").is_some());
        assert!(scope.get("y").is_none());
    }

    #[test]
    fn ordem_preservada() {
        // Propriedade central de IndexMap — obrigatório verificar
        let mut scope = Scope::new();
        scope.define("z", Value::None);
        scope.define("a", Value::None);
        scope.define("m", Value::None);
        let names: Vec<&str> = scope.iter().map(|(n, _)| n).collect();
        assert_eq!(names, vec!["z", "a", "m"]);
    }

    #[test]
    fn redefine_mantem_posicao() {
        // IndexMap::insert em chave existente mantém posição
        let mut scope = Scope::new();
        scope.define("a", Value::None);
        scope.define("b", Value::None);
        scope.define("a", Value::None); // redefinição
        let names: Vec<&str> = scope.iter().map(|(n, _)| n).collect();
        assert_eq!(names, vec!["a", "b"]);
    }

    #[test]
    fn vazio() {
        let scope = Scope::new();
        assert!(scope.is_empty());
        assert_eq!(scope.len(), 0);
    }

    #[test]
    fn get_binding() {
        let mut scope = Scope::new();
        scope.define("x", Value::None);
        assert!(scope.get_binding("x").is_some());
        assert!(scope.get_binding("missing").is_none());
    }

    // ── P715 — get_mut / value_mut ──────────────────────────────────────────

    #[test]
    fn p715_get_mut_muta_binding_existente() {
        let mut scope = Scope::new();
        scope.define("x", Value::Int(1));
        *scope.get_mut("x").unwrap() = Value::Int(99);
        assert_eq!(scope.get("x"), Some(&Value::Int(99)));
    }

    #[test]
    fn p715_get_mut_ausente_devolve_none() {
        let mut scope = Scope::new();
        assert!(scope.get_mut("missing").is_none());
    }

    #[test]
    fn known_cardinality_zero_is_empty_without_allocation() {
        for scope in [Scope::with_capacity(0), Scope::new(), Scope::default()] {
            assert!(scope.is_empty());
            assert_eq!(scope.len(), 0);
            assert_eq!(scope.map.capacity(), 0);
            assert_eq!(scope.iter().count(), 0);
            assert!(scope.get("missing").is_none());
        }
    }

    #[test]
    fn known_cardinality_fill_and_replace_keep_initial_capacity_and_order() {
        for count in [1usize, 3, 4, 7, 8, 17, 257, 4096] {
            let mut scope = Scope::with_capacity(count);
            assert!(scope.is_empty());
            assert_eq!(scope.len(), 0);
            let capacity = scope.map.capacity();
            assert!(capacity >= count, "reservation must cover {count} bindings");
            let names: Vec<String> = (0..count)
                .map(|index| format!("key-{:05}", count - index))
                .collect();
            for (index, name) in names.iter().enumerate() {
                scope.define(name.clone(), Value::Int(index as i64));
                assert_eq!(scope.map.capacity(), capacity, "insert {index}/{count}");
            }
            assert_eq!(scope.len(), count);
            assert_eq!(scope.iter().map(|(name, _)| name).collect::<Vec<_>>(),
                names.iter().map(String::as_str).collect::<Vec<_>>());
            for (index, name) in names.iter().enumerate() {
                assert_eq!(scope.get(name), Some(&Value::Int(index as i64)));
                assert_eq!(scope.get_binding(name).unwrap().value(),
                    &Value::Int(index as i64));
            }
            let replaced = &names[count / 2];
            scope.define(replaced.clone(), Value::Bool(true));
            assert_eq!(scope.get(replaced), Some(&Value::Bool(true)));
            *scope.get_mut(replaced).unwrap() = Value::Str("changed".into());
            assert_eq!(scope.get(replaced), Some(&Value::Str("changed".into())));
            assert!(scope.get_mut("missing").is_none());
            assert_eq!(scope.len(), count);
            assert_eq!(scope.map.capacity(), capacity);
            assert_eq!(scope.iter().map(|(name, _)| name).collect::<Vec<_>>(),
                names.iter().map(String::as_str).collect::<Vec<_>>());
        }
    }

    #[test]
    fn known_cardinality_growth_past_actual_capacity_preserves_all_bindings() {
        let mut scope = Scope::with_capacity(3);
        let initial_capacity = scope.map.capacity();
        assert!(initial_capacity >= 3);
        for index in 0..=initial_capacity {
            scope.define(format!("descending-{}", initial_capacity - index),
                Value::Int(index as i64));
        }
        assert_eq!(scope.len(), initial_capacity + 1);
        assert!(scope.map.capacity() > initial_capacity);
        for (index, (name, binding)) in scope.iter().enumerate() {
            assert_eq!(name, format!("descending-{}", initial_capacity - index));
            assert_eq!(binding.value(), &Value::Int(index as i64));
        }
    }

    #[test]
    fn known_cardinality_clone_and_large_to_small_scopes_are_independent() {
        let mut original = Scope::with_capacity(4096);
        original.define("z", Value::Int(41));
        original.define("a", Value::None);
        let original_capacity = original.map.capacity();
        let mut cloned = original.clone();
        cloned.define("z", Value::Int(99));
        cloned.define("clone-only", Value::Bool(true));
        assert_eq!(original.get("z"), Some(&Value::Int(41)));
        assert!(original.get("clone-only").is_none());
        assert_eq!(original.map.capacity(), original_capacity);
        assert_eq!(original.len(), 2);
        let mut small = Scope::with_capacity(original.len());
        let small_capacity = small.map.capacity();
        assert!(small_capacity >= original.len());
        for (name, binding) in original.iter() {
            small.define(name.to_owned(), binding.value().clone());
            assert_eq!(small.map.capacity(), small_capacity);
        }
        assert_eq!(small.iter().map(|(name, _)| name).collect::<Vec<_>>(), ["z", "a"]);
        *small.get_mut("z").unwrap() = Value::Bool(false);
        assert_eq!(original.get("z"), Some(&Value::Int(41)));
        assert_eq!(cloned.get("z"), Some(&Value::Int(99)));
        let empty = Scope::with_capacity(Scope::new().len());
        assert_eq!(empty.len(), 0);
        assert_eq!(empty.map.capacity(), 0);
    }

}
