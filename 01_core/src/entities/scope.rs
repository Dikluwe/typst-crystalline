//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/scope.md
//! @prompt-hash 9c418396
//! @layer L1
//! @updated 2026-03-28

use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

use crate::entities::value::Value;

/// Valor ligado a um nome num Scope.
///
/// Mantido acoplado ao stub Value::None neste passo.
/// Campos adicionais do original (kind, span, category, deprecation) são
/// adicionados quando Value real migrar — não antecipar. (ADR-0017)
#[derive(Debug)]
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
#[derive(Debug)]
pub struct Scope {
    map: IndexMap<String, Binding, FxBuildHasher>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            map: IndexMap::with_hasher(FxBuildHasher::default()),
        }
    }

    /// Define um binding. Se o nome já existe, substitui no lugar
    /// (mantendo a posição na ordem de inserção).
    pub fn define(&mut self, name: impl Into<String>, value: Value) {
        self.map.insert(name.into(), Binding::new(value));
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.map.get(name).map(|b| b.value())
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
        scope.define("a", Value::None);  // redefinição
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
}
