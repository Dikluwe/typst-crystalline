//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/oracle.md
//! @prompt-hash fe541492
//! @layer L3
//! @updated 2026-08-05
//!
//! **P980** — oráculo de paridade de operador: transformações aplicadas
//! ao content stream já construído pelo modo verbose, **só** no caminho
//! da flag `--oracle-pdf`. Nunca corre na saída principal (ver o prompt).

/// **P980 — transformação 1**: colapsa arrays `[ … ] TJ` cujos ajustes
/// são todos zero em `<hex…> Tj` (paridade com o padrão do vanilla).
///
/// Formato esperado (o nosso verbose): entries `<XXXX>` e inteiros
/// separados por espaços dentro de `[ … ] TJ`, tudo numa linha. Regras:
/// - todos os números inteiros do array são 0 (um `-0` conta como zero) ⇒
///   reescreve como os hex concatenados + ` Tj`;
/// - qualquer número ≠ 0, token inesperado, ou array vazio ⇒ intocado.
///
/// A varredura é de bytes ASCII e conservadora: fora de `[ … ] TJ`
/// bem-formado, nada é tocado.
pub(crate) fn collapse_trivial_tj(content: &str) -> String {
    let bytes = content.as_bytes();
    let mut out = String::with_capacity(content.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'[' {
            if let Some((replacement, end)) = try_collapse(bytes, i) {
                out.push_str(&replacement);
                i = end;
                continue;
            }
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    out
}

/// Tenta colapsar o array que começa em `bytes[start]` (`[`). Devolve
/// `(replacement, end)` onde `end` é o índice logo após `TJ`, ou `None`
/// se o array não é um TJ trivial.
fn try_collapse(bytes: &[u8], start: usize) -> Option<(String, usize)> {
    let mut i = start + 1;
    let mut hex_parts: Vec<String> = Vec::new();
    let mut saw_any = false;
    loop {
        // espaços entre tokens
        while i < bytes.len() && bytes[i] == b' ' {
            i += 1;
        }
        if i >= bytes.len() {
            return None;
        }
        match bytes[i] {
            b'<' => {
                let end = bytes[i..].iter().position(|&b| b == b'>')? + i;
                let hex = std::str::from_utf8(&bytes[i + 1..end]).ok()?;
                if hex.is_empty() || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
                    return None;
                }
                hex_parts.push(hex.to_string());
                saw_any = true;
                i = end + 1;
            }
            b'0'..=b'9' | b'-' => {
                let start_n = i;
                if bytes[i] == b'-' {
                    i += 1;
                }
                while i < bytes.len() && bytes[i].is_ascii_digit() {
                    i += 1;
                }
                let num = std::str::from_utf8(&bytes[start_n..i]).ok()?;
                let value: i64 = num.parse().ok()?;
                if value != 0 {
                    return None;
                }
            }
            b']' => {
                // espera-se " TJ" a seguir
                if bytes.get(i + 1..i + 4) == Some(b" TJ") {
                    if !saw_any {
                        return None;
                    }
                    let mut hex_all = String::new();
                    for part in &hex_parts {
                        hex_all.push_str(part);
                    }
                    return Some((format!("<{hex_all}> Tj"), i + 4));
                }
                return None;
            }
            _ => return None,
        }
    }
}

/// Wrapper de bytes para `collapse_trivial_tj` (o content stream verbose
/// é ASCII por construção; se algum dia não for UTF-8 válido, devolve o
/// input intocado — nunca corrompe).
pub(crate) fn collapse_trivial_tj_bytes(bytes: &[u8]) -> Vec<u8> {
    match std::str::from_utf8(bytes) {
        Ok(s) => collapse_trivial_tj(s).into_bytes(),
        Err(_) => bytes.to_vec(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p980_array_so_zeros_colapsa() {
        let input = "[ <0041> 0 <0042> 0 ] TJ\n";
        assert_eq!(collapse_trivial_tj(input), "<00410042> Tj\n");
    }

    #[test]
    fn p980_ajuste_de_fronteira_zero_tambem_colapsa() {
        let input = "[ <0041> 0 0 <0042> 0 ] TJ\n";
        assert_eq!(collapse_trivial_tj(input), "<00410042> Tj\n");
    }

    #[test]
    fn p980_ajuste_nao_zero_fica_intocado() {
        let input = "[ <0041> 0 -233 <0042> 0 ] TJ\n";
        assert_eq!(collapse_trivial_tj(input), input);
    }

    #[test]
    fn p980_menos_zero_conta_como_zero() {
        let input = "[ <0041> -0 <0042> 0 ] TJ\n";
        assert_eq!(collapse_trivial_tj(input), "<00410042> Tj\n");
    }

    #[test]
    fn p980_tj_simples_e_texto_normal_intocados() {
        let input = "<0041> Tj\nBT\n1 0 0 -1 0 0 Tm\n";
        assert_eq!(collapse_trivial_tj(input), input);
    }

    #[test]
    fn p980_array_vazio_ou_malformado_intocado() {
        assert_eq!(collapse_trivial_tj("[ ] TJ"), "[ ] TJ");
        assert_eq!(collapse_trivial_tj("[ <0041 "), "[ <0041 ");
        assert_eq!(collapse_trivial_tj("[ 5 ] TJ"), "[ 5 ] TJ");
    }
}
