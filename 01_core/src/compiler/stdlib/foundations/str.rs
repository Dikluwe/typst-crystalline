//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/foundations/str.md
//! @prompt-hash 729549f1
//! @layer L1
//! @updated 2026-08-13
//!
//! `str(v)` / `str.from-unicode(codepoint)` / `regex(pattern)`.
//! Fatiado de `foundations.rs` e absorvido `text/regex.rs` no Passo 1032.

use ecow::EcoString;

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::file_id::FileId;
use crate::entities::layout_types::Length;
use crate::entities::source_result::SourceResult;
use crate::entities::value::Value;

use crate::compiler::stdlib::{err, expect_no_named};

/// `str(v)` → representação textual do valor.
/// P491: `str(int, base: n)` converte inteiro para base 2–36.
/// P501: `str.from-unicode(codepoint)` constrói carácter a partir de um scalar Unicode.
pub fn native_str(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    // P491 — arg nomeado `base` (aplicável apenas a Int). Apenas `base` é aceite.
    let base_arg = args.named.get("base");
    if args.named.len() > 1 || (args.named.len() == 1 && base_arg.is_none()) {
        let bad = args
            .named
            .keys()
            .find(|k| k.as_str() != "base")
            .map(|k| k.as_str())
            .unwrap_or("?");
        return err(format!("str() argumento nomeado desconhecido: '{bad}'"));
    }

    match args.items.as_slice() {
        [v] => {
            // P491: str(Int, base: n)
            if let Some(base_val) = base_arg {
                if let Value::Int(i) = v {
                    let base = match base_val {
                        Value::Int(b) => *b as u32,
                        other => {
                            return err(format!(
                                "str() argumento 'base' requer Int, recebeu {}",
                                other.type_name()
                            ))
                        }
                    };
                    if !(2..=36).contains(&base) {
                        return err(format!(
                            "str() base deve estar entre 2 e 36, recebeu {}",
                            base
                        ));
                    }
                    return Ok(Value::Str(EcoString::from(format_radix(*i, base))));
                }
                return err(format!(
                    "str() argumento 'base' só se aplica a Int, recebeu {}",
                    v.type_name()
                ));
            }

            let s: String = match v {
                Value::None => "none".into(),
                Value::Bool(b) => if *b { "true" } else { "false" }.into(),
                Value::Int(i) => i.to_string(),
                Value::Float(f) => format_float(*f),
                Value::Str(s) => return Ok(Value::Str(s.clone())),
                Value::Symbol(symbol) => return Ok(Value::Str(symbol.value.clone())),
                Value::Label(label) => label.0.clone(),
                Value::Auto => "auto".into(),
                Value::Length(l) => format_length(l),
                Value::Ratio(r) => format!("{}%", r.to_percent()),
                Value::Angle(a) => format!("{}deg", a.to_deg()),
                Value::Bytes(b) => match std::str::from_utf8(b.as_slice()) {
                    Ok(s) => s.to_string(),
                    Err(_) => return err("bytes are not valid UTF-8"),
                },
                Value::Color(_) => return err("str() não suporta color"),
                other => return err(format!("str() não suporta {}", other.type_name())),
            };
            Ok(Value::Str(EcoString::from(s)))
        }
        _ => err(format!("str() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

/// `str.from-unicode(codepoint)` → carácter correspondente ao scalar Unicode.
pub fn native_str_from_unicode(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(i)] => match char::try_from(*i as u32) {
            Ok(c) if c != '\0' => Ok(Value::Str(EcoString::from(c.to_string()))),
            _ => err("str.from-unicode() requer um codepoint Unicode válido"),
        },
        [other] => {
            err(format!("str.from-unicode() requer Int, recebeu {}", other.type_name()))
        }
        _ => err(format!(
            "str.from-unicode() requer 1 argumento, recebeu {}",
            args.items.len()
        )),
    }
}

/// Formata f64 de forma compacta — sem trailing zeros desnecessários.
pub(crate) fn format_float(f: f64) -> String {
    let s = format!("{}", f);
    if s.contains('.') || s.contains('e') {
        s
    } else {
        format!("{s}.0")
    }
}

/// Converte um inteiro para a representação textual numa base 2–36.
/// P491: paridade vanilla `str(255, base: 16) == "ff"`.
pub(crate) fn format_radix(mut n: i64, base: u32) -> String {
    if n == 0 {
        return "0".to_string();
    }
    let negative = n < 0;
    let mut n = n.abs();
    let mut result = String::new();
    while n > 0 {
        let digit = (n % base as i64) as u32;
        let c = std::char::from_digit(digit, base).unwrap();
        result.push(c);
        n /= base as i64;
    }
    if negative {
        result.push('-');
    }
    result.chars().rev().collect()
}

/// Formata Length como string (ex: "12pt", "1.5em", "6pt + 1em").
pub(crate) fn format_length(l: &Length) -> String {
    let abs = l.abs.to_pt();
    let em = l.em;
    match (abs == 0.0, em == 0.0) {
        (true, true) => "0pt".into(),
        (false, true) => format!("{abs}pt"),
        (true, false) => format!("{em}em"),
        (false, false) => format!("{abs}pt + {em}em"),
    }
}
#[cfg(test)]
mod tests_p699b_str_bytes {
    use super::*;
    use crate::entities::bytes::Bytes;
    use crate::entities::font_book::FontBook;
    use crate::entities::source::Source;
    use crate::entities::world_types::{Datetime, FileError, FileResult, Font, Library};
    use std::num::NonZeroU16;

    fn ctx() -> EvalContext {
        EvalContext::new()
    }

    fn test_file_id() -> FileId {
        FileId::from_raw(NonZeroU16::new(1).unwrap())
    }

    #[derive(Default)]
    struct NullWorld {
        library: Library,
        book: FontBook,
    }

    impl crate::contracts::world::World for NullWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            test_file_id()
        }
        fn source(&self, _: FileId) -> FileResult<Source> {
            Err(FileError::NotFound)
        }
        fn file(&self, _: FileId) -> FileResult<crate::entities::world_types::Bytes> {
            Err(FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<Font> {
            None
        }
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<Datetime> {
            None
        }
        fn read_bytes(
            &self,
            _current_file: FileId,
            path: &str,
        ) -> Result<std::sync::Arc<Vec<u8>>, String> {
            Err(format!("ficheiro não encontrado: {}", path))
        }
    }

    fn null_world() -> NullWorld {
        NullWorld::default()
    }

    /// P699b — `str()` sobre `Value::Bytes` válido UTF-8 decodifica (paridade
    /// vanilla `foundations/str.rs:871`). Reproduz o gap encontrado ao
    /// compilar `str(plugin("hello.wasm").hello())` via documento real.
    #[test]
    fn str_de_bytes_valido_utf8_decodifica() {
        let args = Args::positional(vec![Value::Bytes(Bytes::new(b"hello".to_vec()))]);
        let v = native_str(&mut ctx(), &args, &null_world(), test_file_id()).unwrap();
        assert_eq!(v, Value::Str("hello".into()));
    }

    #[test]
    fn str_de_bytes_invalido_utf8_erro_verbatim() {
        let args = Args::positional(vec![Value::Bytes(Bytes::new(vec![0xFF, 0xFE]))]);
        let e = native_str(&mut ctx(), &args, &null_world(), test_file_id()).unwrap_err();
        assert!(
            e[0].message.contains("bytes are not valid UTF-8"),
            "msg: {}",
            e[0].message,
        );
    }
}
