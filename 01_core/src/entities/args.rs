//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/args.md
//! @prompt-hash bae502b4
//! @layer L1
//! @updated 2026-03-28

use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

use crate::entities::span::Span;
use crate::entities::value::Value;

/// Uma ocorrência avaliada, com as duas âncoras da sua origem.
#[derive(Debug, Clone)]
pub struct ArgOccurrence {
    pub name: Option<EcoString>,
    pub value: Value,
    pub span: Span,
    pub value_span: Span,
}

/// Argumentos de chamada, com views posicionais/nomeadas e sequência causal.
#[derive(Clone)]
pub struct Args {
    /// Argumentos posicionais, em ordem.
    pub items: Vec<Value>,
    /// Argumentos nomeados (named args), preservando ordem de inserção.
    pub named: IndexMap<EcoString, Value, FxBuildHasher>,
    /// Âncora agregada, independente das âncoras individuais.
    pub span: Span,
    /// Sequência autoritativa quando há origem causal disponível.
    pub occurrences: Option<Vec<ArgOccurrence>>,
}

impl Args {
    /// Cria Args apenas com posicionais (named vazio), `span` detached —
    /// usado por construções internas/sintéticas sem chamada real no
    /// documento. Para uma chamada real, construir `Args` directamente
    /// com o `span` da lista de argumentos (ver `eval/closures.rs::eval_args`).
    pub fn positional(items: Vec<Value>) -> Self {
        Self::from_parts(items, IndexMap::default(), Span::detached())
    }

    pub fn from_parts(
        items: Vec<Value>,
        named: IndexMap<EcoString, Value, FxBuildHasher>,
        span: Span,
    ) -> Self {
        Self { items, named, span, occurrences: None }
    }

    pub fn from_occurrences(span: Span, occurrences: Vec<ArgOccurrence>) -> Self {
        let mut items = Vec::new();
        let mut named = IndexMap::default();
        for occurrence in &occurrences {
            if let Some(name) = &occurrence.name {
                named.insert(name.clone(), occurrence.value.clone());
            } else {
                items.push(occurrence.value.clone());
            }
        }
        Self { items, named, span, occurrences: Some(occurrences) }
    }

    pub fn occurrence_sequence(&self) -> Vec<ArgOccurrence> {
        if let Some(occurrences) = &self.occurrences {
            return occurrences.clone();
        }
        self.items
            .iter()
            .map(|value| (None, value))
            .chain(self.named.iter().map(|(name, value)| (Some(name.clone()), value)))
            .map(|(name, value)| ArgOccurrence {
                name,
                value: value.clone(),
                span: Span::detached(),
                value_span: Span::detached(),
            })
            .collect()
    }

    pub fn invalidate_occurrences(&mut self) {
        self.occurrences = None;
    }

    pub fn remove_positional(&mut self, index: usize) -> Option<Value> {
        if let Some(occurrences) = &mut self.occurrences {
            let position = occurrences
                .iter()
                .enumerate()
                .filter(|(_, occurrence)| occurrence.name.is_none())
                .nth(index)
                .map(|(position, _)| position)?;
            let value = occurrences.remove(position).value;
            let occurrences = self.occurrences.take().unwrap();
            *self = Self::from_occurrences(self.span, occurrences);
            Some(value)
        } else if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    pub fn remove_named(&mut self, name: &str) -> Option<Value> {
        if let Some(occurrences) = &mut self.occurrences {
            let value = occurrences
                .iter()
                .rev()
                .find(|occurrence| occurrence.name.as_deref() == Some(name))?
                .value
                .clone();
            occurrences.retain(|occurrence| occurrence.name.as_deref() != Some(name));
            let occurrences = self.occurrences.take().unwrap();
            *self = Self::from_occurrences(self.span, occurrences);
            Some(value)
        } else {
            self.named.shift_remove(name)
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty() && self.named.is_empty()
    }
}

impl PartialEq for Args {
    fn eq(&self, other: &Self) -> bool {
        self.items == other.items && self.named == other.named && self.span == other.span
    }
}

impl std::fmt::Debug for Args {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Args")
            .field("items", &self.items)
            .field("named", &self.named)
            .field("span", &self.span)
            .finish()
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
