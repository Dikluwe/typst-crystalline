//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/list_marker.md
//! @prompt-hash 58f1a485
//! @layer L1
//! @updated 2026-06-26

use ecow::EcoString;

/// Marcador de item de lista não ordenada. Subset minimal P470:
/// bullet padrão, string customizada ou array de marcadores por nível.
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum ListMarker {
    /// Bullet padrão (`•` U+2022).
    Default,
    /// Marcador customizado (ex: `"→"`, `"-"`, `"*"`).
    Custom(EcoString),
    /// **P494** — Array de marcadores (vanilla `list(marker: ("a", "b"))`).
    /// Renderização usa o marcador do nível actual; para o nível raiz,
    /// usa o primeiro elemento.
    Array(Vec<ListMarker>),
}

impl Default for ListMarker {
    fn default() -> Self { Self::Default }
}

impl ListMarker {
    pub fn render(&self) -> &str {
        match self {
            Self::Default   => "•",
            Self::Custom(s) => s.as_str(),
            Self::Array(v)  => v.first().map(|m| m.render()).unwrap_or("•"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_renderiza_bullet() {
        assert_eq!(ListMarker::Default.render(), "•");
    }

    #[test]
    fn custom_renderiza_string() {
        let m = ListMarker::Custom("→".into());
        assert_eq!(m.render(), "→");
    }

    #[test]
    fn default_impl() {
        assert_eq!(ListMarker::default(), ListMarker::Default);
    }

    #[test]
    fn igualdade() {
        let a = ListMarker::Custom("x".into());
        let b = ListMarker::Custom("x".into());
        let c = ListMarker::Custom("y".into());
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_ne!(ListMarker::Default, ListMarker::Custom("•".into()));
    }
}
