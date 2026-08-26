//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/numbering.md
//! @prompt-hash 00000000
//! @layer L1
//! @updated 2026-07-23
//!
//! Função nativa global `numbering()` e o algoritmo partilhado de
//! formatação de padrões de numeração (`format_pattern`), reutilizado
//! por `counter.display(pattern)` (`stdlib/counter.rs`).
//! Extraído de `stdlib/structural.rs` no P847 (um ficheiro, um prompt).

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::engine::Engine;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

/// Aplica uma numeração e converte o valor Typst resultante em markup.
pub fn realize_numbering(
    numbering: &crate::entities::numbering::Numbering,
    numbers: &[usize],
    span: Span,
    scopes: &mut crate::compiler::scopes::Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    let value = match numbering {
        crate::entities::numbering::Numbering::Pattern(pattern) => {
            let numbers: Vec<u32> = numbers.iter().map(|n| *n as u32).collect();
            Value::Str(format_pattern(engine, span, pattern, &numbers)?.into())
        }
        crate::entities::numbering::Numbering::Func(func) => {
            let args = Args {
                items: numbers.iter().map(|n| Value::Int(*n as i64)).collect(),
                named: indexmap::IndexMap::default(),
                span,
            };
            crate::compiler::eval::call_dispatch::apply_func(
                func.clone(),
                args,
                scopes,
                ctx,
                engine,
            )?
        }
    };
    Ok(crate::compiler::stdlib::value_to_content(&value))
}

/// `#numbering(pattern, ..numbers)` — função nativa de formatação.
/// Passo 793.
pub fn native_numbering(
    ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
    scopes: &mut crate::compiler::scopes::Scopes<'_>,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    if args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "numbering() exige pelo menos 1 argumento (o padrão)".to_string(),
        )]);
    }

    let pattern_val = &args.items[0];
    let numbers_args = &args.items[1..];

    let mut numbers = Vec::with_capacity(numbers_args.len());
    for v in numbers_args {
        match v {
            Value::Int(n) => {
                if *n >= 0 {
                    numbers.push(*n as u32);
                } else {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        format!(
                            "numbering(): números devem ser não-negativos, recebeu {}",
                            n
                        ),
                    )]);
                }
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!(
                        "numbering(): esperado inteiros como números, recebeu {}",
                        other.type_name()
                    ),
                )]);
            }
        }
    }

    match pattern_val {
        Value::Func(func) => {
            let size_arg = crate::entities::args::Args {
                items: numbers.into_iter().map(|n| Value::Int(n as i64)).collect(),
                named: indexmap::IndexMap::default(),
                span: args.span,
            };
            crate::compiler::eval::call_dispatch::apply_func(
                func.clone(),
                size_arg,
                scopes,
                ctx,
                engine,
            )
        }
        Value::Str(pat) => {
            let formatted = format_pattern(engine, args.span, pat.as_str(), &numbers)?;
            Ok(Value::Str(formatted.into()))
        }
        other => Err(vec![SourceDiagnostic::error(
            args.span,
            format!(
                "numbering(): esperado padrão (string ou função), recebeu {}",
                other.type_name()
            ),
        )]),
    }
}

/// **P844** (achado #53 de P831) — promovido a `pub(crate)` para
/// reuso por `counter.display(pattern)` (`stdlib/counter.rs`), que
/// passa a partilhar este algoritmo (tokens, descarte de tokens extra,
/// repetição do último token) em vez do stub "Pattern minimal".
pub(crate) fn format_pattern(
    engine: &mut Engine<'_>,
    span: Span,
    pat: &str,
    numbers: &[u32],
) -> SourceResult<String> {
    if numbers.is_empty() {
        return Ok(String::new());
    }

    let mut pieces = Vec::new();
    let mut handled = 0;

    for (i, c) in pat.char_indices() {
        // P844: token `①` (circled numbers) adicionado — paridade
        // vanilla `numbering("①", n)`.
        if matches!(c, '1' | 'a' | 'A' | 'i' | 'I' | 'א' | '①') {
            let prefix = &pat[handled..i];
            pieces.push((prefix, c));
            handled = i + c.len_utf8();
        }
    }
    let suffix = &pat[handled..];

    if pieces.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            span,
            "padrão de numeração inválido (deve conter pelo menos um símbolo de contagem como '1', 'a', 'i', etc.)".to_string(),
        )]);
    }

    let mut result = String::new();
    let mut numbers_iter = numbers.iter();

    for (idx, (prefix, symbol)) in pieces.iter().enumerate() {
        if let Some(&n) = numbers_iter.next() {
            result.push_str(prefix);
            let formatted_num = format_numeral(engine, span, *symbol, n)?;
            result.push_str(&formatted_num);
        } else {
            break;
        }
    }

    if let Some(&(last_prefix, last_symbol)) = pieces.last() {
        for &n in numbers_iter {
            if last_prefix.is_empty() {
                result.push_str(suffix);
            } else {
                result.push_str(last_prefix);
            }
            let formatted_num = format_numeral(engine, span, last_symbol, n)?;
            result.push_str(&formatted_num);
        }
    }

    result.push_str(suffix);
    Ok(result)
}

fn format_numeral(
    engine: &mut Engine<'_>,
    span: Span,
    symbol: char,
    n: u32,
) -> SourceResult<String> {
    match symbol {
        '1' => Ok(n.to_string()),
        'a' => Ok(nth_alpha_char(n, false)),
        'A' => Ok(nth_alpha_char(n, true)),
        'i' => Ok(to_roman_numeral(n, false)),
        'I' => Ok(to_roman_numeral(n, true)),
        'א' => {
            if n == 0 {
                engine.sink.warn_note(
                    span,
                    "the numeral system `hebrew` cannot represent zero",
                    "",
                );
                Ok("0".to_string())
            } else {
                Ok(to_hebrew_numeral(n))
            }
        }
        // **P844** (achado #53 de P831) — circled numbers. Medido no
        // vanilla 0.15.0: 0 → "⓪"; 1..=50 → ①..㊿; >50 → warning
        // "the number {n} is too large to be represented with the
        // `arabic.o` numeral system" + fallback decimal.
        '①' => {
            if n > 50 {
                let msg = format!(
                    "the number {n} is too large to be represented with the `arabic.o` numeral system"
                );
                engine.sink.warn_note(span, &msg, "");
                Ok(n.to_string())
            } else {
                Ok(to_circled_number(n))
            }
        }
        _ => Ok(n.to_string()),
    }
}

/// **P844** — 0..=50 para número circulado (⓪, ①..⑳, ㉑..㉟, ㊱..㊿).
fn to_circled_number(n: u32) -> String {
    let ch = match n {
        0 => '\u{24EA}',                                     // ⓪
        1..=20 => char::from_u32(0x2460 + n - 1).unwrap(),   // ①..⑳
        21..=35 => char::from_u32(0x3251 + n - 21).unwrap(), // ㉑..㉟
        _ => char::from_u32(0x32B1 + n - 36).unwrap(),       // ㊱..㊿
    };
    ch.to_string()
}

fn nth_alpha_char(n: u32, upper: bool) -> String {
    if n == 0 {
        return "0".to_string();
    }
    let idx = ((n - 1) % 26) as u8;
    let base = if upper { b'A' } else { b'a' };
    let ch = (base + idx) as char;
    let reps = ((n - 1) / 26 + 1) as usize;
    ch.to_string().repeat(reps)
}

fn to_roman_numeral(mut n: u32, upper: bool) -> String {
    if n == 0 {
        return "0".to_string();
    }
    let mut roman = String::new();
    const MAPPING: [(u32, &str); 13] = [
        (1000, "m"),
        (900, "cm"),
        (500, "d"),
        (400, "cd"),
        (100, "c"),
        (90, "xc"),
        (50, "l"),
        (40, "xl"),
        (10, "x"),
        (9, "ix"),
        (5, "v"),
        (4, "iv"),
        (1, "i"),
    ];
    for &(val, sym) in &MAPPING {
        while n >= val {
            roman.push_str(sym);
            n -= val;
        }
    }
    if upper {
        roman.to_uppercase()
    } else {
        roman
    }
}

fn to_hebrew_numeral(mut n: u32) -> String {
    if n == 0 {
        return String::new();
    }

    let mut result = String::new();

    let thousands = n / 1000;
    if thousands > 0 {
        result.push_str(&to_hebrew_numeral(thousands));
        result.push_str("'");
        n %= 1000;
    }

    let hundreds = n / 100;
    n %= 100;

    let mut h_val = hundreds * 100;
    while h_val > 0 {
        if h_val >= 400 {
            result.push('ת');
            h_val -= 400;
        } else if h_val >= 300 {
            result.push('ש');
            h_val -= 300;
        } else if h_val >= 200 {
            result.push('ר');
            h_val -= 200;
        } else if h_val >= 100 {
            result.push('ק');
            h_val -= 100;
        }
    }

    if n == 15 {
        result.push_str("טו");
        return result;
    }
    if n == 16 {
        result.push_str("טז");
        return result;
    }

    let tens = n / 10;
    let units = n % 10;

    match tens {
        1 => result.push('י'),
        2 => result.push('כ'),
        3 => result.push('ל'),
        4 => result.push('מ'),
        5 => result.push('נ'),
        6 => result.push('ס'),
        7 => result.push('ע'),
        8 => result.push('פ'),
        9 => result.push('צ'),
        _ => {}
    }

    match units {
        1 => result.push('א'),
        2 => result.push('ב'),
        3 => result.push('ג'),
        4 => result.push('ד'),
        5 => result.push('ה'),
        6 => result.push('ו'),
        7 => result.push('ז'),
        8 => result.push('ח'),
        9 => result.push('ט'),
        _ => {}
    }

    result
}
