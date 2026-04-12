//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/syntax-text.md
//! @prompt-hash 7a9a8b09
//! @layer L1
//! @updated 2026-03-22

use std::fmt;
use std::sync::Arc;

/// Opaque domain string for syntactic token text.
///
/// The internal representation (`Arc<str>`) is a private detail — L1
/// defines what a domain string *is*, not how it is stored.  If future
/// performance requirements demand a different backing (e.g.
/// `ecow::EcoString`), only this module needs to change; the public
/// interface of L1 remains stable.
///
/// The corresponding `From<ecow::EcoString>` conversion lives in L3,
/// at the boundary where parser output enters the domain.
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SyntaxText(Arc<str>);

impl SyntaxText {
    /// Create an empty string.  Allocates a single shared empty `Arc`.
    pub fn new() -> Self {
        Self(Arc::from(""))
    }

    /// View the contents as a `str` slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Byte length of the string.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Whether the string is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Default for SyntaxText {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&str> for SyntaxText {
    fn from(s: &str) -> Self {
        Self(Arc::from(s))
    }
}

impl From<String> for SyntaxText {
    fn from(s: String) -> Self {
        Self(Arc::from(s.as_str()))
    }
}

impl fmt::Display for SyntaxText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl fmt::Debug for SyntaxText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&*self.0, f)
    }
}

impl PartialEq<str> for SyntaxText {
    fn eq(&self, other: &str) -> bool {
        &*self.0 == other
    }
}

impl PartialEq<SyntaxText> for str {
    fn eq(&self, other: &SyntaxText) -> bool {
        self == &*other.0
    }
}

impl PartialEq<&str> for SyntaxText {
    fn eq(&self, other: &&str) -> bool {
        &*self.0 == *other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_slice() {
        let t = SyntaxText::from("hello");
        assert_eq!(t.as_str(), "hello");
        assert_eq!(t.len(), 5);
        assert!(!t.is_empty());
    }

    #[test]
    fn from_string() {
        let s = String::from("world");
        let t = SyntaxText::from(s);
        assert_eq!(t.as_str(), "world");
    }

    #[test]
    fn empty_via_new() {
        let t = SyntaxText::new();
        assert!(t.is_empty());
        assert_eq!(t.len(), 0);
        assert_eq!(t.as_str(), "");
    }

    #[test]
    fn clone_is_cheap() {
        let a = SyntaxText::from("shared");
        let b = a.clone();
        assert_eq!(a, b);
        // Both point at the same Arc — comparing via as_str is sufficient
        assert_eq!(a.as_str(), b.as_str());
    }

    #[test]
    fn display() {
        let t = SyntaxText::from("display-me");
        assert_eq!(t.to_string(), "display-me");
    }
}
