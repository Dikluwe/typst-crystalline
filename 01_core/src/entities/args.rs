//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/args.md
//! @prompt-hash 00000000
//! @layer L1
//! @updated 2026-03-28

use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

use crate::entities::value::Value;

/// Argumentos de chamada de função.
///
/// Combina args posicionais e named (Passo 17). Spread adiado (ADR-0016).
#[derive(Debug, Clone, PartialEq)]
pub struct Args {
    /// Argumentos posicionais, em ordem.
    pub items: Vec<Value>,
    /// Argumentos nomeados (named args), preservando ordem de inserção.
    pub named: IndexMap<EcoString, Value, FxBuildHasher>,
}

impl Args {
    /// Cria Args apenas com posicionais (named vazio).
    pub fn positional(items: Vec<Value>) -> Self {
        Self { items, named: IndexMap::default() }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty() && self.named.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn args_vazio() {
        let a = Args::positional(vec![]);
        assert!(a.is_empty());
        assert_eq!(a.len(), 0);
    }

    #[test]
    fn args_com_items() {
        let a = Args::positional(vec![Value::Int(1), Value::Bool(true)]);
        assert!(!a.is_empty());
        assert_eq!(a.len(), 2);
        assert_eq!(a.items[0], Value::Int(1));
        assert_eq!(a.items[1], Value::Bool(true));
    }

    #[test]
    fn args_clone_e_eq() {
        let a1 = Args::positional(vec![Value::Int(42)]);
        let a2 = a1.clone();
        assert_eq!(a1, a2);
    }

    #[test]
    fn args_named() {
        let mut a = Args::positional(vec![]);
        a.named.insert("x".into(), Value::Int(1));
        assert!(!a.is_empty());
        assert_eq!(a.named.get("x"), Some(&Value::Int(1)));
    }
}
