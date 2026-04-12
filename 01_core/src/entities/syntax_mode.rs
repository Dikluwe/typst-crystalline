//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/syntax-mode.md
//! @prompt-hash 0f54f077
//! @layer L1
//! @updated 2026-03-23

/// O modo sintáctico de uma porção de código Typst.
///
/// Determina como o lexer tokeniza e o parser interpreta o texto.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum SyntaxMode {
    /// Texto e markup — o nível de topo.
    Markup,
    /// Átomos matemáticos, operadores, etc. — dentro de equações.
    Math,
    /// Palavras-chave, literais e operadores — após `#`.
    Code,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn syntax_mode_e_copy_e_eq() {
        let a = SyntaxMode::Markup;
        let b = a; // Copy
        assert_eq!(a, b);
    }

    #[test]
    fn variantes_distintas() {
        assert_ne!(SyntaxMode::Markup, SyntaxMode::Math);
        assert_ne!(SyntaxMode::Math, SyntaxMode::Code);
        assert_ne!(SyntaxMode::Markup, SyntaxMode::Code);
    }
}
