//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/regex.md
//! @prompt-hash 377d975d
//! @layer L1
//! @updated 2026-05-12
//!
//! **P209D (M9c)** — Wrapper L1 sobre `regex::Regex` crate per
//! ADR-0077 (PROPOSTO 2026-05-12).
//!
//! `regex::Regex` não deriva `Hash`/`PartialEq`/`Eq`/`Clone`;
//! wrapper materializa estes traits manualmente via field
//! `pattern: String` como key. Mesma pattern → semanticamente
//! mesma regex (compilação determinística do crate).
//!
//! Consumer único actual: `Selector::Regex(Regex)` (P209D
//! Bloco VI). Query arm `Regex` é stub `vec![]` documentado —
//! cristalino single-pass não tem Content text durante query
//! phase; semântica funcional fica deferred per P209A A3.

use std::fmt;
use std::hash::{Hash, Hasher};

/// Erro de construção de `Regex` — pattern inválida.
#[derive(thiserror::Error, Debug)]
pub enum RegexError {
    #[error("regex inválida: {0}")]
    Invalid(String),
}

/// Wrapper L1 sobre `regex::Regex`. Hash/Eq/PartialEq via pattern.
pub struct Regex {
    pattern:  String,
    compiled: regex::Regex,
}

impl Regex {
    /// Constrói uma nova `Regex`. Erro contextual se pattern
    /// inválido (rejeitado por `regex::Regex::new`).
    pub fn new(pattern: &str) -> Result<Self, RegexError> {
        let compiled = regex::Regex::new(pattern)
            .map_err(|e| RegexError::Invalid(e.to_string()))?;
        Ok(Self {
            pattern: pattern.to_string(),
            compiled,
        })
    }

    /// Pattern original (string).
    pub fn pattern(&self) -> &str {
        &self.pattern
    }

    /// Verifica se a regex matchea `text`. Custo amortizado
    /// O(|text|) por invocação (sem cache cristalino).
    pub fn is_match(&self, text: &str) -> bool {
        self.compiled.is_match(text)
    }
}

impl Hash for Regex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash via pattern: mesma pattern → mesmo hash.
        // `compiled` é opaco; não pode contribuir.
        self.pattern.hash(state);
    }
}

impl PartialEq for Regex {
    fn eq(&self, other: &Self) -> bool {
        // Eq via pattern. Compilação é determinística per crate.
        self.pattern == other.pattern
    }
}

impl Eq for Regex {}

impl Clone for Regex {
    fn clone(&self) -> Self {
        // Re-construção via `Regex::new`. Pattern já validada
        // no `new` original; expect documentado.
        Self::new(&self.pattern)
            .expect("Regex::clone: pattern previamente válida")
    }
}

impl fmt::Debug for Regex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Regex")
            .field("pattern", &self.pattern)
            .finish()
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
        // `[` é bracket-class sem fecho — pattern inválida.
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
        // Mesma pattern em 2 instances → mesmo hash.
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
        // Igualdade structural.
        assert_eq!(r, cloned);
        // Funcionalmente equivalente.
        assert_eq!(r.is_match("hello"), cloned.is_match("hello"));
        assert_eq!(r.is_match("123"), cloned.is_match("123"));
    }

    #[test]
    fn regex_debug_oculta_compiled() {
        let r = Regex::new("foo").unwrap();
        let debug = format!("{:?}", r);
        assert!(debug.contains("pattern"));
        assert!(debug.contains("foo"));
        // `compiled` é opaco; não deve aparecer formatado.
    }
}
