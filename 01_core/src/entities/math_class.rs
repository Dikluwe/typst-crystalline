//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/math-class.md
//! @prompt-hash 6c7bac29
//! @layer L1
//! @updated 2026-03-23

use unicode_math_class;

/// Classificação Unicode de símbolos matemáticos.
///
/// Define o comportamento tipográfico de cada símbolo em modo math:
/// espaçamento automático, prioridade de operador, associatividade.
///
/// Especificação: https://www.unicode.org/reports/tr25/
///
/// Movido de `typst_utils::MathClass` — ADR-0009.
/// Completado com delegação a `unicode_math_class` — ADR-0011.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MathClass {
    Normal,
    Alphabetic,
    Binary,
    Closing,
    Diacritic,
    Fence,
    GlyphPart,
    Large,
    Opening,
    Punctuation,
    Relation,
    Space,
    Unary,
    Vary,
    Special,
}

/// Retorna a classe math padrão de Typst para um caractere Unicode.
///
/// Aplica primeiro os overrides Typst-específicos (ADR-0009); para os
/// restantes delega na tabela TR25 via `unicode_math_class` (ADR-0011).
///
/// Retorna `None` para caracteres sem classe math definida.
///
/// Inlinado de `typst_utils::default_math_class` — ADR-0009.
/// Licença da tabela de overrides: Apache-2.0 (upstream Typst).
pub fn default_math_class(c: char) -> Option<MathClass> {
    match c {
        // Better spacing.
        // https://github.com/typst/typst/commit/2e039cb052fcb768027053cbf02ce396f6d7a6be
        ':' => Some(MathClass::Relation),

        // Better spacing when used alongside + PLUS SIGN.
        // https://github.com/typst/typst/pull/1726
        '⋯' | '⋱' | '⋰' | '⋮' => Some(MathClass::Normal),

        // Better spacing.
        // https://github.com/typst/typst/pull/1855
        '.' | '/' => Some(MathClass::Normal),

        // ⊥ UP TACK should not be a relation, contrary to ⟂ PERPENDICULAR.
        // https://github.com/typst/typst/pull/5714
        '\u{22A5}' => Some(MathClass::Normal),

        // Used as a binary connector in linear logic ("par").
        // https://github.com/typst/typst/issues/5764
        '⅋' => Some(MathClass::Binary),

        // Those overrides should become the default in the next revision of
        // MathClass.txt.
        // https://github.com/typst/typst/issues/5764#issuecomment-2632435247
        '⎰' | '⟅' => Some(MathClass::Opening),
        '⎱' | '⟆' => Some(MathClass::Closing),

        // Both ∨ and ⟑ are classified as Binary.
        // https://github.com/typst/typst/issues/5764
        '⟇' => Some(MathClass::Binary),

        // Arabic comma.
        // https://github.com/latex3/unicode-math/pull/633#issuecomment-2028936135
        '،' => Some(MathClass::Punctuation),

        // Delegação à tabela TR25 completa — ADR-0011.
        c => unicode_math_class::class(c).map(from_unicode_math_class),
    }
}

/// Converte `unicode_math_class::MathClass` para o tipo de domínio L1.
fn from_unicode_math_class(c: unicode_math_class::MathClass) -> MathClass {
    match c {
        unicode_math_class::MathClass::Normal      => MathClass::Normal,
        unicode_math_class::MathClass::Alphabetic  => MathClass::Alphabetic,
        unicode_math_class::MathClass::Binary      => MathClass::Binary,
        unicode_math_class::MathClass::Closing     => MathClass::Closing,
        unicode_math_class::MathClass::Diacritic   => MathClass::Diacritic,
        unicode_math_class::MathClass::Fence       => MathClass::Fence,
        unicode_math_class::MathClass::GlyphPart   => MathClass::GlyphPart,
        unicode_math_class::MathClass::Large       => MathClass::Large,
        unicode_math_class::MathClass::Opening     => MathClass::Opening,
        unicode_math_class::MathClass::Punctuation => MathClass::Punctuation,
        unicode_math_class::MathClass::Relation    => MathClass::Relation,
        unicode_math_class::MathClass::Space       => MathClass::Space,
        unicode_math_class::MathClass::Unary       => MathClass::Unary,
        unicode_math_class::MathClass::Vary        => MathClass::Vary,
        unicode_math_class::MathClass::Special     => MathClass::Special,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- overrides Typst (ADR-0009) ---

    #[test]
    fn dois_pontos_e_relacao() {
        assert_eq!(default_math_class(':'), Some(MathClass::Relation));
    }

    #[test]
    fn ponto_e_normal() {
        assert_eq!(default_math_class('.'), Some(MathClass::Normal));
    }

    #[test]
    fn barra_e_normal() {
        assert_eq!(default_math_class('/'), Some(MathClass::Normal));
    }

    #[test]
    fn reticencias_sao_normal() {
        assert_eq!(default_math_class('⋯'), Some(MathClass::Normal));
        assert_eq!(default_math_class('⋱'), Some(MathClass::Normal));
        assert_eq!(default_math_class('⋰'), Some(MathClass::Normal));
        assert_eq!(default_math_class('⋮'), Some(MathClass::Normal));
    }

    #[test]
    fn up_tack_e_normal() {
        assert_eq!(default_math_class('\u{22A5}'), Some(MathClass::Normal));
    }

    #[test]
    fn par_e_binary() {
        assert_eq!(default_math_class('⅋'), Some(MathClass::Binary));
        assert_eq!(default_math_class('⟇'), Some(MathClass::Binary));
    }

    #[test]
    fn opening_overrides() {
        assert_eq!(default_math_class('⎰'), Some(MathClass::Opening));
        assert_eq!(default_math_class('⟅'), Some(MathClass::Opening));
    }

    #[test]
    fn closing_overrides() {
        assert_eq!(default_math_class('⎱'), Some(MathClass::Closing));
        assert_eq!(default_math_class('⟆'), Some(MathClass::Closing));
    }

    #[test]
    fn virgula_arabe_e_punctuation() {
        assert_eq!(default_math_class('،'), Some(MathClass::Punctuation));
    }

    // --- delegação unicode_math_class (ADR-0011) ---

    #[test]
    fn letra_delega_para_alphabetic() {
        // 'a' não tem override Typst → delega a unicode_math_class → Alphabetic
        assert_eq!(default_math_class('a'), Some(MathClass::Alphabetic));
    }

    #[test]
    fn digito_delega_para_normal() {
        assert_eq!(default_math_class('0'), Some(MathClass::Normal));
    }

    #[test]
    fn mais_delega_para_vary() {
        // '+' PLUS SIGN: unicode_math_class = Vary
        assert_eq!(default_math_class('+'), Some(MathClass::Vary));
    }

    #[test]
    fn parentese_abre_delega_para_opening() {
        assert_eq!(default_math_class('('), Some(MathClass::Opening));
    }

    #[test]
    fn parentese_fecha_delega_para_closing() {
        assert_eq!(default_math_class(')'), Some(MathClass::Closing));
    }

    #[test]
    fn emoji_sem_classe_retorna_none() {
        // '😃' não é símbolo math — deve retornar None
        assert_eq!(default_math_class('😃'), None);
    }

    #[test]
    fn math_class_e_copy_e_eq() {
        let a = MathClass::Normal;
        let b = a; // Copy
        assert_eq!(a, b);
        assert_ne!(MathClass::Normal, MathClass::Binary);
    }

    #[test]
    fn diacritic_e_glyph_part_existem() {
        // Variantes adicionadas em ADR-0011 para cobrir unicode_math_class
        assert_ne!(MathClass::Diacritic, MathClass::Normal);
        assert_ne!(MathClass::GlyphPart, MathClass::Normal);
    }
}
