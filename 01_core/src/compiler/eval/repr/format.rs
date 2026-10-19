//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/repr/format.md
//! @prompt-hash fa500158
//! @layer L1
//! @updated 2026-09-30

// ── P843 — `pretty_comma_list` / `pretty_array_like` do vanilla ────────────
//
// Porte literal de `foundations/repr.rs:170-223`: horizontal se a soma dos
// comprimentos das peças + separadores couber em 50 colunas; caso contrário
// vertical (uma peça por linha, vírgula final sempre). `pretty_array_like`
// envolve em parênteses, indentando cada linha com 2 espaços no modo
// vertical.

/// Lista separada por vírgulas — paridade vanilla `pretty_comma_list`.
pub(super) fn pretty_comma_list(pieces: &[String], trailing_comma: bool) -> String {
    const MAX_WIDTH: usize = 50;
    let mut buf = String::new();
    let len = pieces.iter().map(|s| s.len()).sum::<usize>()
        + 2 * pieces.len().saturating_sub(1);

    if !pieces.iter().any(|piece| piece.contains('\n')) && len <= MAX_WIDTH {
        for (i, piece) in pieces.iter().enumerate() {
            if i > 0 {
                buf.push_str(", ");
            }
            buf.push_str(piece);
        }
        if trailing_comma {
            buf.push(',');
        }
    } else {
        for piece in pieces {
            buf.push_str(piece.trim());
            buf.push_str(",\n");
        }
    }
    buf
}

/// Construto tipo-array `(...)` — paridade vanilla `pretty_array_like`.
pub(super) fn pretty_array_like(parts: &[String], trailing_comma: bool) -> String {
    let list = pretty_comma_list(parts, trailing_comma);
    let mut buf = String::new();
    buf.push('(');
    if list.contains('\n') {
        buf.push('\n');
        for (i, line) in list.lines().enumerate() {
            if i > 0 {
                buf.push('\n');
            }
            buf.push_str("  ");
            buf.push_str(line);
        }
        buf.push('\n');
    } else {
        buf.push_str(&list);
    }
    buf.push(')');
    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wide_list_breaks_vertically_and_keeps_trailing_commas() {
        let result = pretty_array_like(&["x".repeat(51)], false);
        assert_eq!(result, format!("(\n  {},\n)", "x".repeat(51)));
    }
}
