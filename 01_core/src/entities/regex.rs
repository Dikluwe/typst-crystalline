//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/regex.md
//! @prompt-hash 5cea7413
//! @layer L1
//! @updated 2026-08-18
//!
//! **P209D (M9c)** — Wrapper L1 sobre `regex::Regex` crate per
//! ADR-0077. **P402** — refino para tipo de primeiro-cidadão:
//! `pattern: EcoString`, `compiled: Arc<regex::Regex>` (clone O(1)),
//! integração com `Value::Regex` e cast `Str → Regex`.
//!
//! `regex::Regex` não deriva `Hash`/`PartialEq`/`Eq`/`Debug`;
//! o wrapper materializa estes traits manualmente via field
//! `pattern` como key. Mesma pattern → semanticamente mesma regex.

use ecow::EcoString;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

/// Erro de construção de `Regex` — pattern inválida.
#[derive(thiserror::Error, Debug)]
pub enum RegexError {
    #[error("regex inválida: {0}")]
    Invalid(String),
}

/// Wrapper L1 sobre `regex::Regex`. Hash/Eq/PartialEq via pattern.
#[derive(Clone)]
pub struct Regex {
    pattern: EcoString,
    compiled: Arc<regex::Regex>,
}

impl Regex {
    /// Constrói uma nova `Regex`. Erro contextual se pattern inválido.
    pub fn new(pattern: &str) -> Result<Self, RegexError> {
        let compiled =
            regex::Regex::new(pattern).map_err(|e| RegexError::Invalid(e.to_string()))?;
        Ok(Self {
            pattern: EcoString::from(pattern),
            compiled: Arc::new(compiled),
        })
    }

    /// Pattern original.
    pub fn pattern(&self) -> &str {
        &self.pattern
    }

    /// Verifica se a regex matcheia `text`.
    pub fn is_match(&self, text: &str) -> bool {
        self.compiled.is_match(text)
    }

    /// **P689** — Primeiro match de `text`: posições em **bytes** (paridade
    /// vanilla), texto do match e capturas dos grupos em ordem posicional
    /// (grupo opcional não participante → `None` / P1075). `None` se não houver match.
    pub fn captures_first(&self, text: &str) -> Option<RegexMatch> {
        let caps = self.compiled.captures(text)?;
        let m = caps.get(0)?;
        let mut captures = Vec::with_capacity(caps.len().saturating_sub(1));
        for i in 1..caps.len() {
            captures.push(caps.get(i).map(|g| g.as_str().to_string()));
        }
        Some(RegexMatch {
            start: m.start(),
            end: m.end(),
            text: m.as_str().to_string(),
            captures,
        })
    }

    /// **P692** — Todos os matches de `text` (não sobrepostos), em ordem, cada um
    /// com posições em **bytes**, texto e capturas. Generalização de
    /// `captures_first` (P689). `Vec` vazio se não houver match.
    pub fn captures_all(&self, text: &str) -> Vec<RegexMatch> {
        let mut out = Vec::new();
        for caps in self.compiled.captures_iter(text) {
            let Some(m) = caps.get(0) else { continue };
            let mut captures = Vec::with_capacity(caps.len().saturating_sub(1));
            for i in 1..caps.len() {
                captures.push(caps.get(i).map(|g| g.as_str().to_string()));
            }
            out.push(RegexMatch {
                start: m.start(),
                end: m.end(),
                text: m.as_str().to_string(),
                captures,
            });
        }
        out
    }
}

/// Resultado do primeiro match de uma `Regex` (P689). Índices em **bytes**.
#[derive(Debug, Clone)]
pub struct RegexMatch {
    pub start: usize,
    pub end: usize,
    pub text: String,
    pub captures: Vec<Option<String>>,
}

impl Hash for Regex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pattern.hash(state);
    }
}

impl PartialEq for Regex {
    fn eq(&self, other: &Self) -> bool {
        self.pattern == other.pattern
    }
}

impl Eq for Regex {}

impl fmt::Debug for Regex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Regex")
            .field("pattern", &self.pattern.as_str())
            .finish()
    }
}

impl Default for Regex {
    fn default() -> Self {
        // Pattern vazia é uma regex válida.
        Self::new("").expect("empty regex is valid")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;

    #[test]
    fn regex_new_valido_ok() {
        let r = Regex::new("a+b");
        assert!(r.is_ok());
        assert_eq!(r.unwrap().pattern(), "a+b");
    }

    #[test]
    fn regex_new_invalido_err() {
        let r = Regex::new("[");
        assert!(r.is_err());
        match r {
            Err(RegexError::Invalid(msg)) => {
                assert!(!msg.is_empty(), "mensagem de erro deve ser populada");
            }
            _ => panic!("expected Invalid error"),
        }
    }

    #[test]
    fn regex_hash_determinismo() {
        let a = Regex::new("\\d+").unwrap();
        let b = Regex::new("\\d+").unwrap();
        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        a.hash(&mut h1);
        b.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }

    #[test]
    fn regex_is_match_basico() {
        let r = Regex::new("\\d+").unwrap();
        assert!(r.is_match("abc123"));
        assert!(!r.is_match("abc"));
        assert!(r.is_match("999"));
    }

    #[test]
    fn regex_eq_via_pattern() {
        let a = Regex::new("a+").unwrap();
        let b = Regex::new("a+").unwrap();
        let c = Regex::new("b+").unwrap();
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn regex_clone_preserva_semantica() {
        let r = Regex::new("[a-z]+").unwrap();
        let cloned = r.clone();
        assert_eq!(r, cloned);
        assert_eq!(r.is_match("hello"), cloned.is_match("hello"));
        assert_eq!(r.is_match("123"), cloned.is_match("123"));
    }

    #[test]
    fn regex_default_empty_pattern() {
        let r = Regex::default();
        assert_eq!(r.pattern(), "");
        assert!(r.is_match(""));
    }
}
