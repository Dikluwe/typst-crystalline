//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/numbering.md
//! @prompt-hash 00000000
//! @layer L1
//! @updated 2026-08-25

use ecow::EcoString;

use crate::entities::func::Func;

/// Forma pública de uma numeração Typst.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Numbering {
    Pattern(EcoString),
    Func(Func),
}

impl Numbering {
    pub fn as_pattern(&self) -> Option<&str> {
        match self {
            Self::Pattern(pattern) => Some(pattern.as_str()),
            Self::Func(_) => None,
        }
    }

    pub fn as_str(&self) -> &str {
        self.as_pattern().unwrap_or("")
    }
}

impl From<EcoString> for Numbering {
    fn from(value: EcoString) -> Self {
        Self::Pattern(value)
    }
}

impl From<&str> for Numbering {
    fn from(value: &str) -> Self {
        Self::Pattern(value.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1158_pattern_preserva_string() {
        let numbering = Numbering::Pattern("1 / 1".into());
        assert_eq!(numbering.as_pattern(), Some("1 / 1"));
    }
}
