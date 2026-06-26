//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/enum_numbering.md
//! @prompt-hash 00000000
//! @layer L1
//! @updated 2026-06-26

use ecow::EcoString;

/// Esquema de numeração de lista ordenada. Subset minimal P470.
#[derive(Debug, Clone, PartialEq, Hash, Default)]
pub enum EnumNumbering {
    /// Arábico com ponto: `"1."`, `"2."`, ... (default).
    #[default]
    Decimal,
    /// Alpha minúsculo com parêntese: `"a)"`, `"b)"`, ...
    LowerAlpha,
    /// Alpha maiúsculo com parêntese: `"A)"`, `"B)"`, ...
    UpperAlpha,
    /// Romano minúsculo com parêntese: `"i)"`, `"ii)"`, ..., `"xii)"`.
    LowerRoman,
    /// Pattern não reconhecido — armazenado sem parse. Renderiza como Decimal.
    Custom(EcoString),
}

impl EnumNumbering {
    /// Formata o número `n` (1-based) de acordo com o esquema.
    pub fn format(&self, n: u32) -> String {
        match self {
            Self::Decimal    => format!("{}.", n),
            Self::LowerAlpha => format!("{})", nth_alpha(n, false)),
            Self::UpperAlpha => format!("{})", nth_alpha(n, true)),
            Self::LowerRoman => format!("{})", to_roman_lower(n)),
            Self::Custom(_)  => format!("{}.", n),
        }
    }

    /// Parse a partir de um pattern string Typst.
    pub fn from_pattern(p: &str) -> Self {
        match p {
            "1." | "1" => Self::Decimal,
            "a)"       => Self::LowerAlpha,
            "A)"       => Self::UpperAlpha,
            "i)"       => Self::LowerRoman,
            other      => Self::Custom(other.into()),
        }
    }
}

/// Letra do alfabeto para posição `n` (1-based), minúsculo ou maiúsculo.
/// n=1 → 'a'/'A', n=26 → 'z'/'Z', n>26 → repete a letra (aa, bb...).
fn nth_alpha(n: u32, upper: bool) -> String {
    let idx = ((n - 1) % 26) as u8;
    let base = if upper { b'A' } else { b'a' };
    let ch = (base + idx) as char;
    let reps = ((n - 1) / 26 + 1) as usize;
    ch.to_string().repeat(reps)
}

/// Romano minúsculo simples para n ∈ 1..=12; n > 12 → decimal string.
fn to_roman_lower(n: u32) -> String {
    const ROMAN: [&str; 12] = [
        "i", "ii", "iii", "iv", "v", "vi",
        "vii", "viii", "ix", "x", "xi", "xii",
    ];
    if n >= 1 && n <= 12 {
        ROMAN[(n - 1) as usize].to_string()
    } else {
        n.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decimal_formata() {
        assert_eq!(EnumNumbering::Decimal.format(1), "1.");
        assert_eq!(EnumNumbering::Decimal.format(3), "3.");
    }

    #[test]
    fn lower_alpha_formata() {
        assert_eq!(EnumNumbering::LowerAlpha.format(1), "a)");
        assert_eq!(EnumNumbering::LowerAlpha.format(2), "b)");
        assert_eq!(EnumNumbering::LowerAlpha.format(26), "z)");
    }

    #[test]
    fn upper_alpha_formata() {
        assert_eq!(EnumNumbering::UpperAlpha.format(1), "A)");
        assert_eq!(EnumNumbering::UpperAlpha.format(3), "C)");
    }

    #[test]
    fn lower_roman_formata() {
        assert_eq!(EnumNumbering::LowerRoman.format(1), "i)");
        assert_eq!(EnumNumbering::LowerRoman.format(4), "iv)");
        assert_eq!(EnumNumbering::LowerRoman.format(12), "xii)");
        assert_eq!(EnumNumbering::LowerRoman.format(13), "13)");
    }

    #[test]
    fn custom_faz_fallback_decimal() {
        assert_eq!(EnumNumbering::Custom("?!".into()).format(2), "2.");
    }

    #[test]
    fn from_pattern() {
        assert_eq!(EnumNumbering::from_pattern("1."), EnumNumbering::Decimal);
        assert_eq!(EnumNumbering::from_pattern("1"),  EnumNumbering::Decimal);
        assert_eq!(EnumNumbering::from_pattern("a)"), EnumNumbering::LowerAlpha);
        assert_eq!(EnumNumbering::from_pattern("A)"), EnumNumbering::UpperAlpha);
        assert_eq!(EnumNumbering::from_pattern("i)"), EnumNumbering::LowerRoman);
        assert_eq!(
            EnumNumbering::from_pattern("?!"),
            EnumNumbering::Custom("?!".into())
        );
    }

    #[test]
    fn default_is_decimal() {
        assert_eq!(EnumNumbering::default(), EnumNumbering::Decimal);
    }
}
