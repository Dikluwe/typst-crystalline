//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/counter_format.md
//! @prompt-hash 7cb205ab
//! @layer L1
//! @updated 2026-06-24
//!
//! Formatação de counters hierárquicos com patterns (P451).
//! Subset suportado: "1.", "1.1", "I.", "(a)", "A.".

/// Formata um vector de valores hierárquicos segundo um pattern.
///
/// Cada caractere do pattern que for um token de nível (`1`, `I`, `a`, `A`)
/// é substituído pela representação do valor correspondente. Os restantes
/// caracteres são copiados literalmente.
///
/// Exemplos:
/// - `[1]` + `"1."` → `"1."`
/// - `[1, 2]` + `"1.1"` → `"1.2"`
/// - `[4]` + `"I."` → `"IV."`
/// - `[2]` + `"(a)"` → `"(b)"`
/// - `[3]` + `"A."` → `"C."`
///
/// Se `values` estiver vazio, ou se o pattern não contiver nenhum token
/// reconhecido, retorna `None`.
pub fn format_counter(values: &[usize], pattern: &str) -> Option<String> {
    if values.is_empty() {
        return None;
    }

    let mut out = String::with_capacity(pattern.len() * 2);
    let mut level = 0usize;
    let mut has_token = false;

    for ch in pattern.chars() {
        match token_kind(ch) {
            Some(kind) => {
                let value = *values.get(level)?;
                out.push_str(&format_level(value, kind));
                level += 1;
                has_token = true;
            }
            None => out.push(ch),
        }
    }

    if has_token {
        Some(out)
    } else {
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TokenKind {
    Arabic,
    Roman,
    LetterLower,
    LetterUpper,
}

fn token_kind(ch: char) -> Option<TokenKind> {
    match ch {
        '1' => Some(TokenKind::Arabic),
        'I' => Some(TokenKind::Roman),
        'a' => Some(TokenKind::LetterLower),
        'A' => Some(TokenKind::LetterUpper),
        _ => None,
    }
}

fn format_level(value: usize, kind: TokenKind) -> String {
    match kind {
        TokenKind::Arabic => value.to_string(),
        TokenKind::Roman => to_roman(value),
        TokenKind::LetterLower => to_letters(value, false),
        TokenKind::LetterUpper => to_letters(value, true),
    }
}

/// Converte 1..=3999 para numeral romano maiúsculo.
fn to_roman(mut n: usize) -> String {
    if n == 0 || n > 3999 {
        // Fora do range clássico: fallback para arábico.
        return n.to_string();
    }
    let table: &[(usize, &str)] = &[
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];
    let mut out = String::new();
    for &(value, symbol) in table {
        while n >= value {
            out.push_str(symbol);
            n -= value;
        }
    }
    out
}

/// Converte 1.. para letras (a, b, ..., z, aa, ab, ...).
fn to_letters(n: usize, upper: bool) -> String {
    if n == 0 {
        return "?".to_string();
    }
    let base = 26;
    let mut n = n;
    let mut chars = Vec::new();
    while n > 0 {
        n -= 1;
        chars.push((n % base) as u8);
        n /= base;
    }
    chars.reverse();
    chars
        .iter()
        .map(|&offset| {
            let code = if upper { b'A' + offset } else { b'a' + offset };
            code as char
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_counter_vazio_devolve_none() {
        assert_eq!(format_counter(&[], "1."), None);
    }

    #[test]
    fn format_counter_pattern_sem_token_devolve_none() {
        assert_eq!(format_counter(&[1], "."), None);
    }

    #[test]
    fn format_1_ponto() {
        assert_eq!(format_counter(&[1], "1."), Some("1.".to_string()));
        assert_eq!(format_counter(&[2], "1."), Some("2.".to_string()));
    }

    #[test]
    fn format_1_ponto_1() {
        assert_eq!(format_counter(&[1, 1], "1.1"), Some("1.1".to_string()));
        assert_eq!(format_counter(&[1, 2], "1.1"), Some("1.2".to_string()));
        assert_eq!(format_counter(&[2, 1], "1.1"), Some("2.1".to_string()));
        assert_eq!(format_counter(&[1, 2, 3], "1.1"), Some("1.2".to_string()));
    }

    #[test]
    fn format_romano() {
        assert_eq!(format_counter(&[1], "I."), Some("I.".to_string()));
        assert_eq!(format_counter(&[4], "I."), Some("IV.".to_string()));
        assert_eq!(format_counter(&[9], "I."), Some("IX.".to_string()));
        assert_eq!(format_counter(&[49], "I."), Some("XLIX.".to_string()));
    }

    #[test]
    fn format_letras_minusculas() {
        assert_eq!(format_counter(&[1], "(a)"), Some("(a)".to_string()));
        assert_eq!(format_counter(&[2], "(a)"), Some("(b)".to_string()));
        assert_eq!(format_counter(&[26], "(a)"), Some("(z)".to_string()));
        assert_eq!(format_counter(&[27], "(a)"), Some("(aa)".to_string()));
    }

    #[test]
    fn format_letras_maiusculas() {
        assert_eq!(format_counter(&[1], "A."), Some("A.".to_string()));
        assert_eq!(format_counter(&[3], "A."), Some("C.".to_string()));
        assert_eq!(format_counter(&[26], "A."), Some("Z.".to_string()));
        assert_eq!(format_counter(&[27], "A."), Some("AA.".to_string()));
    }

    #[test]
    fn format_hierarquico_misto() {
        // Primeiro nível romano, segundo arábico.
        assert_eq!(format_counter(&[2, 3], "I.1"), Some("II.3".to_string()));
    }

    #[test]
    fn valores_insuficientes_devolve_none() {
        assert_eq!(format_counter(&[1], "1.1"), None);
    }
}
