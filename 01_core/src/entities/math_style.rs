//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/math_style.md
//! @prompt-hash 7c871fd3
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
            // intencional: variants glyph operam no plano de codepoints Unicode e mantêm size_factor identidade (1.0)
            MathStyleKind::Plain
            | MathStyleKind::SansSerif
            | MathStyleKind::Chancery
            | MathStyleKind::Roundhand
            | MathStyleKind::Fraktur
            | MathStyleKind::Monospace
            | MathStyleKind::DoubleStruck => 1.0,
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

    // **P812-C** — `scr` (Roundhand) usa os MESMOS codepoints de `cal`
    // (Chancery), diferenciados apenas pelo variation selector U+FE01
    // (`map_glyph_vs`) — paridade codex `to_roundhand = to_script + VS2`.
    // A regra anterior ("Roundhand = Bold Script") foi refutada por medição:
    // `scr` non-bold mapeia para o bloco script, não bold-script.
    let kind =
        if kind == MathStyleKind::Roundhand { MathStyleKind::Chancery } else { kind };

    if let Some(replacement) = bmp_exception(c, kind, bold, italic) {
        return replacement;
    }

    // P809 — Greek (só `Plain`; outros kinds passa-through, scope-out
    // registado no L0). Grego maiúsculo sem flags é no-op (upright por
    // defeito — paridade vanilla medida: `ΓΔΩ𝛼`).
    if kind == MathStyleKind::Plain {
        if let Some(replacement) = greek_plain(c, bold, italic) {
            return replacement;
        }
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

/// **P812-C** — variation selector a emitir DEPOIS de um carácter mapeado
/// por `cal`/`scr` (paridade codex `to_chancery`/`to_roundhand`): VS1
/// (U+FE00) para `cal` (Chancery), VS2 (U+FE01) para `scr` (Roundhand),
/// apenas para letras latinas (dígitos e não-latinos não levam selector).
/// Os consumers devem anexar o selector a seguir ao char mapeado por
/// `map_glyph` (que devolve o codepoint script partilhado pelos dois).
pub fn map_glyph_vs(c: char, kind: MathStyleKind) -> Option<char> {
    if !c.is_ascii_alphabetic() {
        return None;
    }
    match kind {
        MathStyleKind::Chancery => Some('\u{FE00}'),
        MathStyleKind::Roundhand => Some('\u{FE01}'),
        _other => None, // neutro: N16[β] — sem selector de variação correspondente para o estilo pedido,
    }
}

/// **P809** — `true` se o carácter tem itálico matemático **por defeito**
/// (paridade codex `MathStyle::select`): letras latinas ASCII e grego
/// minúsculo (`'α'..='ω'`). Grego maiúsculo e dígitos são upright por
/// defeito. Formas de símbolo gregas (`ϵ ϑ ϰ ϕ ϱ ϖ`) e `∂` ficam de fora
/// (scope-out registado — o vanilla inclui-as em `is_lower_greek`).
pub fn is_math_italic_default(c: char) -> bool {
    c.is_ascii_alphabetic()
        || ('α'..='ω').contains(&c)
        // **P964** — formas de símbolo gregas + ∂ (medido no vanilla:
        // mapeiam para o plano itálico math; `math_style.md` §P964).
        || matches!(c, 'ϑ' | 'ϕ' | 'ϖ' | 'ϰ' | 'ϱ' | 'ϵ' | '∂')
}

/// P809 — Greek com `kind == Plain`. Os blocos Unicode math seguem a ordem
/// alfabética grega completa (contígua, com `ς`/`σ` adjacentes nas
/// minúsculas; o buraco U+03A2 alinha com a ranhura `ϴ` nas maiúsculas).
fn greek_plain(c: char, bold: bool, italic: bool) -> Option<char> {
    // **P964** — formas de símbolo gregas + ∂: fora dos blocos contíguos,
    // mapeamento explícito para o plano itálico math (medido no vanilla;
    // `math_style.md` §P964). Só Plain+italic (o vanilla não mapeia estas
    // formas em bold/bold-italic por esta via — escopo preservado).
    if !bold && italic {
        let mapped = match c {
            'ϵ' => Some('\u{1D716}'),
            'ϑ' => Some('\u{1D717}'),
            'ϖ' => Some('\u{1D718}'),
            'ϕ' => Some('\u{1D719}'),
            'ϱ' => Some('\u{1D71A}'),
            'ϰ' => Some('\u{1D71B}'),
            '∂' => Some('\u{1D715}'),
            _other => None, // neutro: N16[β] — caractere sem mapeamento itálico no bloco grego plain,
        };
        if mapped.is_some() {
            return mapped;
        }
    }
    let lower = ('α'..='ω').contains(&c);
    let upper = ('Α'..='Ρ').contains(&c) || ('Σ'..='Ω').contains(&c);
    if !lower && !upper {
        return None;
    }
    let base = match (bold, italic) {
        (false, true) if lower => 0x1D6FC_u32,
        (true, false) if lower => 0x1D6C2,
        (true, true) if lower => 0x1D736,
        (false, true) if upper => 0x1D6E2,
        (true, false) if upper => 0x1D6A8,
        (true, true) if upper => 0x1D71C,
        _ => return None,
    };
    let origin = if lower { 'α' } else { 'Α' };
    char::from_u32(base + (c as u32 - origin as u32))
}

fn letter_base(kind: MathStyleKind, bold: bool, italic: bool) -> Option<u32> {
    match (kind, bold, italic) {
        (MathStyleKind::Plain, false, false) => None,
        (MathStyleKind::Plain, true, false) => Some(0x1D400),
        (MathStyleKind::Plain, false, true) => Some(0x1D434),
        (MathStyleKind::Plain, true, true) => Some(0x1D468),
        (MathStyleKind::Chancery, false, _) => Some(0x1D49C),
        (MathStyleKind::Chancery, true, _) => Some(0x1D4D0),
        (MathStyleKind::Fraktur, false, _) => Some(0x1D504),
        (MathStyleKind::Fraktur, true, _) => Some(0x1D56C),
        (MathStyleKind::DoubleStruck, _, _) => Some(0x1D538),
        (MathStyleKind::SansSerif, false, false) => Some(0x1D5A0),
        (MathStyleKind::SansSerif, true, false) => Some(0x1D5D4),
        (MathStyleKind::SansSerif, false, true) => Some(0x1D608),
        (MathStyleKind::SansSerif, true, true) => Some(0x1D63C),
        (MathStyleKind::Monospace, _, _) => Some(0x1D670),
        (MathStyleKind::Script | MathStyleKind::SScript, _, _) => None,
        _other => None, // neutro: N16[β] — sem base alfabética para o estilo de letra pedido,
    }
}

fn digit_base(kind: MathStyleKind, bold: bool) -> Option<u32> {
    match (kind, bold) {
        (MathStyleKind::Plain, true) => Some(0x1D7CE),
        (MathStyleKind::DoubleStruck, _) => Some(0x1D7D8),
        (MathStyleKind::SansSerif, false) => Some(0x1D7E2),
        (MathStyleKind::SansSerif, true) => Some(0x1D7EC),
        (MathStyleKind::Monospace, _) => Some(0x1D7F6),
        _other => None, // neutro: N16[β] — sem base numérica para o estilo de dígito pedido,
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

        _other => None, // neutro: N16[β] — sem exceção BMP para a combinação de glifo e estilo,
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
        // P809 — 'α' com kind Plain deixou de ser passthrough (Greek coberto);
        // 'ϑ' (forma de símbolo) e '∇' (kind não-Plain) continuam.
        assert_eq!(map_glyph('ϑ', MathStyleKind::Plain, true, false), 'ϑ');
        assert_eq!(map_glyph('∇', MathStyleKind::DoubleStruck, false, false), '∇');
    }

    #[test]
    fn roundhand_aliases_bold_script() {
        // P812-C — REFUTADO por medição: o L0 pré-P812 dizia "Roundhand =
        // Bold Script", mas codex `to_roundhand = to_script + VS2(U+FE01)`
        // — `scr` usa os MESMOS codepoints de `cal` (script), diferenciados
        // pelo variation selector. `scr` non-bold = script block (não bold).
        assert_eq!(
            map_glyph('A', MathStyleKind::Roundhand, false, false),
            map_glyph('A', MathStyleKind::Chancery, false, false),
        );
        // Com bold: ambos vão para o bloco bold-script.
        assert_eq!(
            map_glyph('A', MathStyleKind::Roundhand, true, false),
            map_glyph('A', MathStyleKind::Chancery, true, false),
        );
    }

    #[test]
    fn p812c_scr_usa_bloco_script_nao_bold_script() {
        // P812-C — paridade codex medida: `$scr(A)$` → script block
        // (U+1D49C para 'A'), NÃO bold-script (U+1D4D0). Excepções
        // letterlike partilhadas com cal: `$scr(L)$` → ℒ U+2112.
        assert_eq!(map_glyph('A', MathStyleKind::Roundhand, false, false), '\u{1D49C}');
        assert_eq!(map_glyph('L', MathStyleKind::Roundhand, false, false), '\u{2112}');
        assert_eq!(map_glyph('B', MathStyleKind::Roundhand, false, false), '\u{212C}');
        assert_eq!(map_glyph('g', MathStyleKind::Roundhand, false, false), '\u{210A}');
    }

    #[test]
    fn p812c_variation_selectors_cal_scr() {
        // P812-C — codex `to_chancery`/`to_roundhand`: VS1 (U+FE00) após
        // cada letra latina de `cal`, VS2 (U+FE01) para `scr`, independente
        // de bold. Não-latinos (dígitos) não levam selector.
        assert_eq!(map_glyph_vs('A', MathStyleKind::Chancery), Some('\u{FE00}'));
        assert_eq!(map_glyph_vs('L', MathStyleKind::Chancery), Some('\u{FE00}'));
        assert_eq!(map_glyph_vs('A', MathStyleKind::Roundhand), Some('\u{FE01}'));
        assert_eq!(map_glyph_vs('L', MathStyleKind::Roundhand), Some('\u{FE01}'));
        assert_eq!(map_glyph_vs('5', MathStyleKind::Chancery), None);
        assert_eq!(map_glyph_vs('5', MathStyleKind::Roundhand), None);
        assert_eq!(map_glyph_vs('A', MathStyleKind::Fraktur), None);
        assert_eq!(map_glyph_vs('A', MathStyleKind::Plain), None);
    }

    // ── P809 — Greek + itálico por defeito ───────────────────────────────

    #[test]
    fn p809_greek_lowercase_italic_contiguo() {
        // Bloco contíguo na ordem alfabética grega (inclui ς/σ adjacentes).
        assert_eq!(map_glyph('α', MathStyleKind::Plain, false, true), '\u{1D6FC}');
        assert_eq!(map_glyph('β', MathStyleKind::Plain, false, true), '\u{1D6FD}');
        assert_eq!(map_glyph('ρ', MathStyleKind::Plain, false, true), '\u{1D70C}');
        assert_eq!(map_glyph('ς', MathStyleKind::Plain, false, true), '\u{1D70D}');
        assert_eq!(map_glyph('σ', MathStyleKind::Plain, false, true), '\u{1D70E}');
        assert_eq!(map_glyph('ω', MathStyleKind::Plain, false, true), '\u{1D714}');
    }

    #[test]
    fn p809_greek_uppercase_só_com_modificador() {
        // Maiúsculas: sem flags é no-op (upright por defeito — medido ΓΔΩ𝛼);
        // com italic/bold explícito mapeia para o bloco correspondente.
        assert_eq!(map_glyph('Γ', MathStyleKind::Plain, false, false), 'Γ');
        assert_eq!(map_glyph('Γ', MathStyleKind::Plain, false, true), '\u{1D6E4}');
        assert_eq!(map_glyph('Ω', MathStyleKind::Plain, false, true), '\u{1D6FA}');
        assert_eq!(map_glyph('Δ', MathStyleKind::Plain, true, false), '\u{1D6AB}');
        assert_eq!(map_glyph('Σ', MathStyleKind::Plain, false, true), '\u{1D6F4}');
    }

    #[test]
    fn p809_greek_bold_e_bold_italic_lowercase() {
        assert_eq!(map_glyph('α', MathStyleKind::Plain, true, false), '\u{1D6C2}');
        assert_eq!(map_glyph('α', MathStyleKind::Plain, true, true), '\u{1D736}');
        assert_eq!(map_glyph('ω', MathStyleKind::Plain, true, true), '\u{1D74E}');
    }

    #[test]
    fn p809_greek_outros_kinds_passthrough() {
        // Scope-out registado: Greek com kind não-Plain não mapeia.
        assert_eq!(map_glyph('α', MathStyleKind::Fraktur, false, false), 'α');
        assert_eq!(map_glyph('α', MathStyleKind::DoubleStruck, false, false), 'α');
    }

    /// **P964** — formas de símbolo gregas e ∂ no plano itálico math
    /// (escopo-out de P809 revogado — medido no vanilla, ToUnicode real:
    /// ϵ→𝜖, ϑ→𝜗, ϖ→𝜘, ϕ→𝜙, ϱ→𝜚, ϰ→𝜅, ∂→𝜕; o vanilla NÃO mapeia
    /// ϐ/Ϝ/ϝ/ϴ/∇ — guardas incluídos). Fonte canónica: codex `styling.rs`.
    #[test]
    fn p964_greek_variantes_e_partial_no_plano_italico() {
        assert_eq!(map_glyph('ϵ', MathStyleKind::Plain, false, true), '\u{1D716}');
        assert_eq!(map_glyph('ϑ', MathStyleKind::Plain, false, true), '\u{1D717}');
        assert_eq!(map_glyph('ϖ', MathStyleKind::Plain, false, true), '\u{1D718}');
        assert_eq!(map_glyph('ϕ', MathStyleKind::Plain, false, true), '\u{1D719}');
        assert_eq!(map_glyph('ϱ', MathStyleKind::Plain, false, true), '\u{1D71A}');
        assert_eq!(map_glyph('ϰ', MathStyleKind::Plain, false, true), '\u{1D71B}');
        assert_eq!(map_glyph('∂', MathStyleKind::Plain, false, true), '\u{1D715}');
        // Guardas: os que o vanilla NÃO mapeia ficam no bloco grego.
        assert_eq!(map_glyph('ϐ', MathStyleKind::Plain, false, true), 'ϐ');
        assert_eq!(map_glyph('ϝ', MathStyleKind::Plain, false, true), 'ϝ');
        assert_eq!(map_glyph('ϴ', MathStyleKind::Plain, false, true), 'ϴ');
        assert_eq!(map_glyph('∇', MathStyleKind::Plain, false, true), '∇');
    }

    /// **P964** — o gate `is_math_italic_default` cobre os 7 codepoints de
    /// variante/∂ (sem eles o `apply_math_default` nem chama `map_glyph`).
    #[test]
    fn p964_is_math_italic_default_cobre_variantes() {
        for c in ['ϵ', 'ϑ', 'ϖ', 'ϕ', 'ϱ', 'ϰ', '∂'] {
            assert!(is_math_italic_default(c), "{c} deve ter itálico por defeito");
        }
        // E não cobre os que o vanilla não mapeia.
        for c in ['ϐ', 'ϝ', 'ϴ', '∇'] {
            assert!(!is_math_italic_default(c), "{c} NÃO deve ter itálico por defeito");
        }
    }

    #[test]
    fn p809_is_math_italic_default() {
        assert!(is_math_italic_default('x'));
        assert!(is_math_italic_default('Z'));
        assert!(is_math_italic_default('α'));
        assert!(is_math_italic_default('ω'));
        assert!(!is_math_italic_default('Γ')); // grego maiúsculo: upright por defeito
        assert!(!is_math_italic_default('5'));
        assert!(!is_math_italic_default('+'));
        // P964 — as formas de símbolo (ϑ etc.) PASSAM a ter itálico por
        // defeito (o scope-out de P809 foi revogado por medição do vanilla);
        // a cobertura completa está em `p964_is_math_italic_default_cobre_
        // variantes`, incl. os guardas do que o vanilla não mapeia.
        assert!(is_math_italic_default('ϑ'));
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
