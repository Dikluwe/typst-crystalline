//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/counter_format.md
//! @prompt-hash 13fef53b
//! @layer L1
//! @updated 2026-06-24
//!
//! Formatação de counters hierárquicos com patterns (P451).
//! Subset suportado: "1.", "1.1", "I.", "(a)", "A.".

/// Verifica se um caractere é um token de numeração reconhecido.
pub fn is_numbering_token(ch: char) -> bool {
    matches!(ch, '1' | 'I' | 'i' | 'a' | 'A')
}

/// Conta o número de tokens de numeração num pattern.
pub fn count_numbering_tokens(pattern: &str) -> usize {
    pattern.chars().filter(|c| is_numbering_token(*c)).count()
}

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
///
/// **P1036** — o pattern decompõe-se em pares `(prefixo, token)` mais **um**
/// sufixo (tudo o que vem depois do último token). Daí as duas regras de
/// excedente, medidas no vanilla ratificado e alinhadas com
/// `compiler/stdlib/numbering.rs::format_pattern`:
///
/// - **mais `values` do que tokens** — o último par `(prefixo, token)` repete-se
///   por cada valor a mais: `[1,1,1]` + `"1."` → `"1.1.1."`;
/// - **mais tokens do que `values`** — a formatação pára no último valor
///   disponível e o sufixo é emitido a seguir: `[1]` + `"1.1"` → `"1"`.
///
/// Ver `00_nucleo/prompts/entities/counter_format.md` §P1036.
pub fn format_counter(values: &[usize], pattern: &str) -> Option<String> {
    if values.is_empty() {
        return None;
    }

    // Decomposição em `(prefixo, token)` + sufixo único.
    let mut pieces: Vec<(&str, TokenKind)> = Vec::new();
    let mut handled = 0usize;
    for (i, ch) in pattern.char_indices() {
        if let Some(kind) = token_kind(ch) {
            pieces.push((&pattern[handled..i], kind));
            handled = i + ch.len_utf8();
        }
    }
    let suffix = &pattern[handled..];

    if pieces.is_empty() {
        return None;
    }

    let mut out = String::with_capacity(pattern.len() * 2);
    let mut values_iter = values.iter();

    // Tokens excedentes são descartados: o `zip` pára no que acabar primeiro.
    for (prefix, kind) in pieces.iter() {
        match values_iter.next() {
            Some(&value) => {
                out.push_str(prefix);
                out.push_str(&format_level(value, *kind));
            }
            None => break,
        }
    }

    // Valores excedentes repetem o último par `(prefixo, token)`. Quando esse
    // prefixo é vazio, o vanilla usa o sufixo como separador — é o caso de
    // `"1."`, que produz `1.1.1.` e não `111.`.
    if let Some(&(last_prefix, last_kind)) = pieces.last() {
        for &value in values_iter {
            if last_prefix.is_empty() {
                out.push_str(suffix);
            } else {
                out.push_str(last_prefix);
            }
            out.push_str(&format_level(value, last_kind));
        }
    }

    out.push_str(suffix);
    Some(out)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TokenKind {
    Arabic,
    Roman,
    RomanLower,
    LetterLower,
    LetterUpper,
}

fn token_kind(ch: char) -> Option<TokenKind> {
    match ch {
        '1' => Some(TokenKind::Arabic),
        'I' => Some(TokenKind::Roman),
        'i' => Some(TokenKind::RomanLower),
        'a' => Some(TokenKind::LetterLower),
        'A' => Some(TokenKind::LetterUpper),
        _ => None,
    }
}

fn format_level(value: usize, kind: TokenKind) -> String {
    match kind {
        TokenKind::Arabic => value.to_string(),
        TokenKind::Roman => to_roman(value),
        TokenKind::RomanLower => to_roman(value).to_lowercase(),
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

    // ── P1036 — semântica de excedente, medida contra o vanilla ────────────
    //
    // Todas as expectativas abaixo foram medidas no vanilla ratificado
    // (`/usr/local/bin/typst`, md5 36da18895eeb5e0136c068a7634e3f82) com
    // `#numbering(pattern, ..numeros)` em 2026-08-13, e coincidem com
    // `compiler/stdlib/numbering.rs::format_pattern`, que já as satisfazia.
    // Ver `00_nucleo/prompts/entities/counter_format.md` §P1036.

    #[test]
    fn valores_em_excesso_repetem_ultimo_token_com_prefixo() {
        // vanilla: #numbering("1.", 1, 1, 1) → "1.1.1."
        assert_eq!(format_counter(&[1, 1, 1], "1."), Some("1.1.1.".to_string()));
        // vanilla: #numbering("1.1", 1, 2, 3) → "1.2.3"
        assert_eq!(format_counter(&[1, 2, 3], "1.1"), Some("1.2.3".to_string()));
        // vanilla: #numbering("A.1.a", 1, 1, 1, 1) → "A.1.a.a"
        assert_eq!(format_counter(&[1, 1, 1, 1], "A.1.a"), Some("A.1.a.a".to_string()));
        // vanilla: #numbering("(a)", 1, 2) → "(a(b)" — o prefixo do último
        // token repete-se; o sufixo sai uma só vez, no fim.
        assert_eq!(format_counter(&[1, 2], "(a)"), Some("(a(b)".to_string()));
    }

    #[test]
    fn tokens_em_excesso_sao_descartados_e_o_sufixo_preservado() {
        // vanilla: #numbering("1.1", 1) → "1"  (não `None`, não "1.")
        assert_eq!(format_counter(&[1], "1.1"), Some("1".to_string()));
        // vanilla: #numbering("A.1.a", 1) → "A"
        assert_eq!(format_counter(&[1], "A.1.a"), Some("A".to_string()));
        // vanilla: #numbering("A.1.a", 1, 2) → "A.2"
        assert_eq!(format_counter(&[1, 2], "A.1.a"), Some("A.2".to_string()));
        // Sufixo depois do último token consumido é preservado.
        assert_eq!(format_counter(&[7], "1.1)"), Some("7)".to_string()));
    }

    #[test]
    fn heading_hierarquico_do_corpo_bate_com_o_vanilla() {
        // O caso que motivou P1036: `#set heading(numbering: "1.")` com
        // `=` / `==` / `===` / `====`.
        assert_eq!(format_counter(&[1], "1."), Some("1.".to_string()));
        assert_eq!(format_counter(&[1, 1], "1."), Some("1.1.".to_string()));
        assert_eq!(format_counter(&[1, 1, 1], "1."), Some("1.1.1.".to_string()));
        assert_eq!(format_counter(&[1, 1, 1, 1], "1."), Some("1.1.1.1.".to_string()));
        // Romanos, o mesmo documento: I. / I.I. / I.I.I.
        assert_eq!(format_counter(&[1, 1, 1], "I."), Some("I.I.I.".to_string()));
    }
}
