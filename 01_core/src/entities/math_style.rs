//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/math_style.md
//! @prompt-hash a297f91f
//! @layer L1
//! @updated 2026-05-20

/// Família de variants math style suportadas em cristalino.
///
/// Sub-categorias:
/// - Variants glyph: afectam codepoint Unicode via `map_glyph`.
/// - Variants size: afectam `style.size` por factor multiplicativo no
///   `MathLayouter`. Não modificam codepoint (passa-through).
///
/// `bold` e `italic` são flags ortogonais aplicadas sobre `MathStyleKind`,
/// não fazem parte deste enum (vivem em `Content::MathStyled`).
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum MathStyleKind {
    Plain,
    SansSerif,
    Chancery,
    Roundhand,
    Fraktur,
    Monospace,
    DoubleStruck,
    Script,
    SScript,
    Display,
    Inline,
}

impl MathStyleKind {
    /// `true` se este variant aplica factor de tamanho em vez de glyph.
    pub fn is_size_variant(self) -> bool {
        matches!(
            self,
            MathStyleKind::Script
                | MathStyleKind::SScript
                | MathStyleKind::Display
                | MathStyleKind::Inline
        )
    }

    /// Factor multiplicativo aplicado a `style.size` para variants size.
    /// Retorna `1.0` para variants glyph e para Display/Inline.
    pub fn size_factor(self) -> f64 {
        match self {
            MathStyleKind::Script => 0.7,
            MathStyleKind::SScript => 0.5,
            MathStyleKind::Display | MathStyleKind::Inline => 1.0,
            _ => 1.0,
        }
    }
}

/// Mapeia um caractere para o seu codepoint Unicode variant.
///
/// Para chars não-cobertos (símbolos, espaços, codepoints não-ASCII),
/// retorna o char de entrada inalterado.
pub fn map_glyph(c: char, kind: MathStyleKind, bold: bool, italic: bool) -> char {
    if kind.is_size_variant() {
        return c;
    }

    if let Some(replacement) = bmp_exception(c, kind, bold, italic) {
        return replacement;
    }

    if c.is_ascii_alphabetic() {
        if let Some(base) = letter_base(kind, bold, italic) {
            let offset = if c.is_ascii_uppercase() {
                c as u32 - b'A' as u32
            } else {
                26 + (c as u32 - b'a' as u32)
            };
            return char::from_u32(base + offset).unwrap_or(c);
        }
        return c;
    }

    if c.is_ascii_digit() {
        if let Some(base) = digit_base(kind, bold) {
            let offset = c as u32 - b'0' as u32;
            return char::from_u32(base + offset).unwrap_or(c);
        }
        return c;
    }

    c
}

fn letter_base(kind: MathStyleKind, bold: bool, italic: bool) -> Option<u32> {
    match (kind, bold, italic) {
        (MathStyleKind::Plain, false, false) => None,
        (MathStyleKind::Plain, true, false) => Some(0x1D400),
        (MathStyleKind::Plain, false, true) => Some(0x1D434),
        (MathStyleKind::Plain, true, true) => Some(0x1D468),
        (MathStyleKind::Chancery, false, _) => Some(0x1D49C),
        (MathStyleKind::Chancery, true, _) => Some(0x1D4D0),
        (MathStyleKind::Roundhand, _, _) => Some(0x1D4D0),
        (MathStyleKind::Fraktur, false, _) => Some(0x1D504),
        (MathStyleKind::Fraktur, true, _) => Some(0x1D56C),
        (MathStyleKind::DoubleStruck, _, _) => Some(0x1D538),
        (MathStyleKind::SansSerif, false, false) => Some(0x1D5A0),
        (MathStyleKind::SansSerif, true, false) => Some(0x1D5D4),
        (MathStyleKind::SansSerif, false, true) => Some(0x1D608),
        (MathStyleKind::SansSerif, true, true) => Some(0x1D63C),
        (MathStyleKind::Monospace, _, _) => Some(0x1D670),
        (MathStyleKind::Script | MathStyleKind::SScript, _, _) => None,
        _ => None,
    }
}

fn digit_base(kind: MathStyleKind, bold: bool) -> Option<u32> {
    match (kind, bold) {
        (MathStyleKind::Plain, true) => Some(0x1D7CE),
        (MathStyleKind::DoubleStruck, _) => Some(0x1D7D8),
        (MathStyleKind::SansSerif, false) => Some(0x1D7E2),
        (MathStyleKind::SansSerif, true) => Some(0x1D7EC),
        (MathStyleKind::Monospace, _) => Some(0x1D7F6),
        _ => None,
    }
}

fn bmp_exception(c: char, kind: MathStyleKind, bold: bool, italic: bool) -> Option<char> {
    match (kind, bold, italic, c) {
        (MathStyleKind::Plain, false, true, 'h') => Some('\u{210E}'),

        (MathStyleKind::Chancery, false, _, 'B') => Some('\u{212C}'),
        (MathStyleKind::Chancery, false, _, 'E') => Some('\u{2130}'),
        (MathStyleKind::Chancery, false, _, 'F') => Some('\u{2131}'),
        (MathStyleKind::Chancery, false, _, 'H') => Some('\u{210B}'),
        (MathStyleKind::Chancery, false, _, 'I') => Some('\u{2110}'),
        (MathStyleKind::Chancery, false, _, 'L') => Some('\u{2112}'),
        (MathStyleKind::Chancery, false, _, 'M') => Some('\u{2133}'),
        (MathStyleKind::Chancery, false, _, 'R') => Some('\u{211B}'),
        (MathStyleKind::Chancery, false, _, 'e') => Some('\u{212F}'),
        (MathStyleKind::Chancery, false, _, 'g') => Some('\u{210A}'),
        (MathStyleKind::Chancery, false, _, 'o') => Some('\u{2134}'),

        (MathStyleKind::Fraktur, false, _, 'C') => Some('\u{212D}'),
        (MathStyleKind::Fraktur, false, _, 'H') => Some('\u{210C}'),
        (MathStyleKind::Fraktur, false, _, 'I') => Some('\u{2111}'),
        (MathStyleKind::Fraktur, false, _, 'R') => Some('\u{211C}'),
        (MathStyleKind::Fraktur, false, _, 'Z') => Some('\u{2128}'),

        (MathStyleKind::DoubleStruck, false, _, 'C') => Some('\u{2102}'),
        (MathStyleKind::DoubleStruck, false, _, 'H') => Some('\u{210D}'),
        (MathStyleKind::DoubleStruck, false, _, 'N') => Some('\u{2115}'),
        (MathStyleKind::DoubleStruck, false, _, 'P') => Some('\u{2119}'),
        (MathStyleKind::DoubleStruck, false, _, 'Q') => Some('\u{211A}'),
        (MathStyleKind::DoubleStruck, false, _, 'R') => Some('\u{211D}'),
        (MathStyleKind::DoubleStruck, false, _, 'Z') => Some('\u{2124}'),

        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kind_is_size_variant() {
        assert!(MathStyleKind::Script.is_size_variant());
        assert!(MathStyleKind::SScript.is_size_variant());
        assert!(MathStyleKind::Display.is_size_variant());
        assert!(MathStyleKind::Inline.is_size_variant());
        assert!(!MathStyleKind::Plain.is_size_variant());
        assert!(!MathStyleKind::DoubleStruck.is_size_variant());
    }

    #[test]
    fn kind_size_factor() {
        assert_eq!(MathStyleKind::Script.size_factor(), 0.7);
        assert_eq!(MathStyleKind::SScript.size_factor(), 0.5);
        assert_eq!(MathStyleKind::Display.size_factor(), 1.0);
        assert_eq!(MathStyleKind::Inline.size_factor(), 1.0);
        assert_eq!(MathStyleKind::Plain.size_factor(), 1.0);
        assert_eq!(MathStyleKind::Fraktur.size_factor(), 1.0);
    }

    #[test]
    fn plain_no_flags_is_noop() {
        assert_eq!(map_glyph('x', MathStyleKind::Plain, false, false), 'x');
        assert_eq!(map_glyph('A', MathStyleKind::Plain, false, false), 'A');
        assert_eq!(map_glyph('5', MathStyleKind::Plain, false, false), '5');
    }

    #[test]
    fn plain_bold_letters() {
        assert_eq!(map_glyph('A', MathStyleKind::Plain, true, false), '\u{1D400}');
        assert_eq!(map_glyph('Z', MathStyleKind::Plain, true, false), '\u{1D419}');
        assert_eq!(map_glyph('a', MathStyleKind::Plain, true, false), '\u{1D41A}');
        assert_eq!(map_glyph('z', MathStyleKind::Plain, true, false), '\u{1D433}');
    }

    #[test]
    fn plain_italic_letters_with_h_exception() {
        assert_eq!(map_glyph('A', MathStyleKind::Plain, false, true), '\u{1D434}');
        assert_eq!(map_glyph('a', MathStyleKind::Plain, false, true), '\u{1D44E}');
        assert_eq!(map_glyph('h', MathStyleKind::Plain, false, true), '\u{210E}');
    }

    #[test]
    fn double_struck_with_exceptions() {
        assert_eq!(
            map_glyph('A', MathStyleKind::DoubleStruck, false, false),
            '\u{1D538}'
        );
        assert_eq!(
            map_glyph('B', MathStyleKind::DoubleStruck, false, false),
            '\u{1D539}'
        );
        assert_eq!(map_glyph('C', MathStyleKind::DoubleStruck, false, false), '\u{2102}');
        assert_eq!(map_glyph('H', MathStyleKind::DoubleStruck, false, false), '\u{210D}');
        assert_eq!(map_glyph('N', MathStyleKind::DoubleStruck, false, false), '\u{2115}');
        assert_eq!(map_glyph('P', MathStyleKind::DoubleStruck, false, false), '\u{2119}');
        assert_eq!(map_glyph('Q', MathStyleKind::DoubleStruck, false, false), '\u{211A}');
        assert_eq!(map_glyph('R', MathStyleKind::DoubleStruck, false, false), '\u{211D}');
        assert_eq!(map_glyph('Z', MathStyleKind::DoubleStruck, false, false), '\u{2124}');
        assert_eq!(
            map_glyph('x', MathStyleKind::DoubleStruck, false, false),
            '\u{1D569}'
        );
    }

    #[test]
    fn fraktur_with_exceptions() {
        assert_eq!(map_glyph('A', MathStyleKind::Fraktur, false, false), '\u{1D504}');
        assert_eq!(map_glyph('C', MathStyleKind::Fraktur, false, false), '\u{212D}');
        assert_eq!(map_glyph('H', MathStyleKind::Fraktur, false, false), '\u{210C}');
        assert_eq!(map_glyph('I', MathStyleKind::Fraktur, false, false), '\u{2111}');
        assert_eq!(map_glyph('R', MathStyleKind::Fraktur, false, false), '\u{211C}');
        assert_eq!(map_glyph('Z', MathStyleKind::Fraktur, false, false), '\u{2128}');
        assert_eq!(map_glyph('k', MathStyleKind::Fraktur, false, false), '\u{1D528}');
    }

    #[test]
    fn chancery_with_exceptions() {
        assert_eq!(map_glyph('A', MathStyleKind::Chancery, false, false), '\u{1D49C}');
        assert_eq!(map_glyph('B', MathStyleKind::Chancery, false, false), '\u{212C}');
        assert_eq!(map_glyph('H', MathStyleKind::Chancery, false, false), '\u{210B}');
        assert_eq!(map_glyph('e', MathStyleKind::Chancery, false, false), '\u{212F}');
        assert_eq!(map_glyph('g', MathStyleKind::Chancery, false, false), '\u{210A}');
        assert_eq!(map_glyph('a', MathStyleKind::Chancery, false, false), '\u{1D4B6}');
    }

    #[test]
    fn sans_serif_combinations() {
        assert_eq!(map_glyph('A', MathStyleKind::SansSerif, false, false), '\u{1D5A0}');
        assert_eq!(map_glyph('A', MathStyleKind::SansSerif, true, false), '\u{1D5D4}');
        assert_eq!(map_glyph('A', MathStyleKind::SansSerif, false, true), '\u{1D608}');
        assert_eq!(map_glyph('A', MathStyleKind::SansSerif, true, true), '\u{1D63C}');
    }

    #[test]
    fn monospace_letters_and_digits() {
        assert_eq!(map_glyph('A', MathStyleKind::Monospace, false, false), '\u{1D670}');
        assert_eq!(map_glyph('z', MathStyleKind::Monospace, false, false), '\u{1D6A3}');
        assert_eq!(map_glyph('0', MathStyleKind::Monospace, false, false), '\u{1D7F6}');
        assert_eq!(map_glyph('9', MathStyleKind::Monospace, false, false), '\u{1D7FF}');
    }

    #[test]
    fn double_struck_digits() {
        assert_eq!(
            map_glyph('0', MathStyleKind::DoubleStruck, false, false),
            '\u{1D7D8}'
        );
        assert_eq!(
            map_glyph('7', MathStyleKind::DoubleStruck, false, false),
            '\u{1D7DF}'
        );
    }

    #[test]
    fn sans_serif_digits() {
        assert_eq!(map_glyph('0', MathStyleKind::SansSerif, false, false), '\u{1D7E2}');
        assert_eq!(map_glyph('0', MathStyleKind::SansSerif, true, false), '\u{1D7EC}');
    }

    #[test]
    fn plain_bold_digits() {
        assert_eq!(map_glyph('0', MathStyleKind::Plain, true, false), '\u{1D7CE}');
        assert_eq!(map_glyph('9', MathStyleKind::Plain, true, false), '\u{1D7D7}');
    }

    #[test]
    fn size_variants_passthrough() {
        assert_eq!(map_glyph('x', MathStyleKind::Script, false, false), 'x');
        assert_eq!(map_glyph('A', MathStyleKind::Script, true, false), 'A');
        assert_eq!(map_glyph('x', MathStyleKind::SScript, false, false), 'x');
        assert_eq!(map_glyph('5', MathStyleKind::SScript, false, false), '5');
    }

    #[test]
    fn non_ascii_passthrough() {
        assert_eq!(map_glyph('+', MathStyleKind::Fraktur, false, false), '+');
        assert_eq!(map_glyph(' ', MathStyleKind::DoubleStruck, false, false), ' ');
        assert_eq!(map_glyph('α', MathStyleKind::Plain, true, false), 'α');
        assert_eq!(map_glyph('∇', MathStyleKind::DoubleStruck, false, false), '∇');
    }

    #[test]
    fn roundhand_aliases_bold_script() {
        for c in ['A', 'Z', 'a', 'z'] {
            assert_eq!(
                map_glyph(c, MathStyleKind::Roundhand, false, false),
                map_glyph(c, MathStyleKind::Chancery, true, false),
            );
        }
    }

    #[test]
    fn kind_traits() {
        let a = MathStyleKind::Plain;
        let b = a;
        assert_eq!(a, b);
        assert_ne!(MathStyleKind::Plain, MathStyleKind::DoubleStruck);
        use std::collections::HashSet;
        let mut s: HashSet<MathStyleKind> = HashSet::new();
        s.insert(MathStyleKind::Plain);
        assert!(s.contains(&MathStyleKind::Plain));
    }
}
