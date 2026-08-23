//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/foundations/cast.md
//! @prompt-hash 6f460cbf
//! @layer L1
//! @updated 2026-08-13
//!
//! Conversões de tipo e constructors primitivos: int, float, range, bytes,
//! datetime, symbol. Fatiado de `foundations.rs` no Passo 1032.

use ecow::EcoString;

use crate::compiler::eval::operators::error_formatting::vanilla_type_name;
use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::bytes::Bytes;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::symbol::{Symbol, SymbolVariant};
use crate::entities::value::Value;
use crate::entities::world_types::Datetime;

use crate::compiler::stdlib::{err, expect_no_named};

/// `int(v)` → inteiro. Aceita Int, Str (decimal), Bool.
/// Float → Err (semântica vanilla: Float não é `ToInt`).
/// P504: `int(str, base: n)` parseia string na base indicada (2–36).
pub fn native_int(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    // P504 — arg nomeado `base` (apenas este é aceite).
    let base = match args.named.get("base") {
        Some(Value::Int(b)) => {
            let base = *b as u32;
            if !(2..=36).contains(&base) {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("int() base deve estar entre 2 e 36, recebeu {}", b),
                )]);
            }
            Some(base)
        }
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "int() argumento 'base' requer Int, recebeu {}",
                    other.type_name()
                ),
            )]);
        }
        None => None,
    };
    if args.named.len() > 1 || (args.named.len() == 1 && !args.named.contains_key("base"))
    {
        let bad = args
            .named
            .keys()
            .find(|k| k.as_str() != "base")
            .map(|k| k.as_str())
            .unwrap_or("?");
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("int() argumento nomeado desconhecido: '{bad}'"),
        )]);
    }

    match args.items.as_slice() {
        [Value::Int(i)] => Ok(Value::Int(*i)),
        [Value::Bool(b)] => Ok(Value::Int(if *b { 1 } else { 0 })),
        [Value::Str(s)] => {
            if let Some(base) = base {
                i64::from_str_radix(s.as_str(), base).map(Value::Int).map_err(|_| {
                    vec![SourceDiagnostic::error(
                        args.span,
                        format!(
                            "int() não consegue parsear {:?} na base {}",
                            s.as_str(),
                            base
                        ),
                    )]
                })
            } else {
                s.parse::<i64>().map(Value::Int).map_err(|_| {
                    vec![SourceDiagnostic::error(
                        args.span,
                        format!("int() não consegue parsear {:?}", s.as_str()),
                    )]
                })
            }
        }
        [Value::Float(f)] => err(format!(
            "int() não converte float {f} — usar int(calc.round(x)) ou int(calc.floor(x))"
        )),
        [other] => err(format!("int() não suporta {}", other.type_name())),
        _ => err(format!("int() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

/// `float(v)` → float. Aceita Float, Int (coerção), Str.
pub fn native_float(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Float(f)] => Ok(Value::Float(*f)),
        [Value::Int(i)] => Ok(Value::Float(*i as f64)),
        [Value::Str(s)] => s.parse::<f64>().map(Value::Float).map_err(|_| {
            vec![SourceDiagnostic::error(
                args.span,
                format!("float() não consegue parsear {:?}", s.as_str()),
            )]
        }),
        [other] => err(format!("float() não suporta {}", other.type_name())),
        _ => err(format!("float() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

/// `range(n)` → Array de 0..n; `range(start, end)` → Array de start..end.
/// P504: `inclusive: true` inclui o `end`. P704: `step: n` (não-zero,
/// default 1, aceita negativo) — algoritmo verbatim do vanilla
/// (`foundations/array.rs:384-430`, `Array::range`).
pub fn native_range(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let inclusive = match args.named.get("inclusive") {
        Some(Value::Bool(b)) => *b,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "range() argumento 'inclusive' requer bool, recebeu {}",
                    other.type_name()
                ),
            )]);
        }
        None => false,
    };
    let step: i64 = match args.named.get("step") {
        Some(Value::Int(0)) => return err("number must not be zero"),
        Some(Value::Int(s)) => *s,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "range() argumento 'step' requer int, recebeu {}",
                    other.type_name()
                ),
            )]);
        }
        None => 1,
    };
    if let Some(bad) = args
        .named
        .keys()
        .find(|k| k.as_str() != "inclusive" && k.as_str() != "step")
    {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("range() argumento nomeado desconhecido: '{bad}'"),
        )]);
    }

    match args.items.as_slice() {
        [Value::Int(n)] => Ok(Value::Array(stepped_range(0, *n, step, inclusive))),
        [Value::Int(start), Value::Int(end)] => {
            Ok(Value::Array(stepped_range(*start, *end, step, inclusive)))
        }
        _ => err(format!("range() requer 1 ou 2 Int, recebeu {} args", args.items.len())),
    }
}

fn stepped_range(start: i64, end: i64, step: i64, inclusive: bool) -> Vec<Value> {
    let step_dir = 0i64.cmp(&step);
    let in_bounds = |x: i64| {
        if inclusive {
            x.cmp(&end) != step_dir.reverse()
        } else {
            x.cmp(&end) == step_dir
        }
    };
    let mut out = Vec::new();
    let mut x = start;
    while in_bounds(x) {
        out.push(Value::Int(x));
        x += step;
    }
    out
}

/// `bytes(value)` → Bytes. Aceita Str (bytes UTF-8), Array de ints 0–255
/// e Bytes (passthrough) — paridade vanilla `ToBytes`
/// (foundations/bytes.rs:425-441). Int não é aceite nesta versão do
/// vanilla (medido: `bytes(3)` → erro).
pub fn native_bytes(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [] => Err(vec![SourceDiagnostic::error(args.span, "missing argument: value")]),
        [Value::Str(s)] => Ok(Value::Bytes(Bytes::from(s.as_str().as_bytes().to_vec()))),
        [Value::Array(arr)] => {
            let mut out = Vec::with_capacity(arr.len());
            for item in arr {
                match item {
                    Value::Int(i) if (0..=255).contains(i) => out.push(*i as u8),
                    Value::Int(_) => {
                        return Err(vec![SourceDiagnostic::error(
                            args.span,
                            "number must be between 0 and 255",
                        )])
                    }
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            args.span,
                            format!(
                                "expected integer, found {}",
                                vanilla_type_name(other)
                            ),
                        )])
                    }
                }
            }
            Ok(Value::Bytes(Bytes::from(out)))
        }
        [Value::Bytes(b)] => Ok(Value::Bytes(b.clone())),
        [other] => Err(vec![SourceDiagnostic::error(
            args.span,
            format!(
                "expected string, array, or bytes, found {}",
                vanilla_type_name(other)
            ),
        )]),
        _ => Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]),
    }
}

/// `datetime(year:, month:, day:, hour:, minute:, second:)` → Datetime.
///
/// Paridade vanilla `Datetime::construct` (foundations/datetime.rs:265-352):
/// os 6 argumentos são nomeados e opcionais; data completa (year+month+day),
/// hora completa (hour+minute+second) ou ambas. Mensagens e hints medidos
/// no vanilla (`temp/p843/f5_*.typ`).
pub fn native_datetime(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    const FIELDS: [&str; 6] = ["year", "month", "day", "hour", "minute", "second"];

    if !args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]);
    }
    for key in args.named.keys() {
        if !FIELDS.contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("unexpected argument: {}", key.as_str()),
            )]);
        }
    }

    let get_int = |name: &str| -> SourceResult<Option<i64>> {
        match args.named.get(name) {
            None => Ok(None),
            Some(Value::Int(i)) => Ok(Some(*i)),
            Some(other) => Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected integer, found {}", vanilla_type_name(other)),
            )]),
        }
    };

    let year = get_int("year")?;
    let month = get_int("month")?;
    let day = get_int("day")?;
    let hour = get_int("hour")?;
    let minute = get_int("minute")?;
    let second = get_int("second")?;

    fn format_missing_args(missing: &[&str]) -> String {
        match missing {
            [arg] => format!("the `{arg}` argument"),
            [a, b] => format!("the `{a}` and `{b}` arguments"),
            [rest @ .., tail] => format!(
                "the {}, and `{tail}` arguments",
                rest.iter().map(|a| format!("`{a}`")).collect::<Vec<_>>().join(", ")
            ),
            [] => unreachable!(),
        }
    }

    let time = match (hour, minute, second) {
        (Some(h), Some(m), Some(s)) => {
            let (Ok(h), Ok(m), Ok(s)) =
                (u8::try_from(h), u8::try_from(m), u8::try_from(s))
            else {
                return Err(vec![SourceDiagnostic::error(args.span, "time is invalid")]);
            };
            match time::Time::from_hms(h, m, s) {
                Ok(t) => Some(t),
                Err(_) => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        "time is invalid",
                    )])
                }
            }
        }
        (None, None, None) => None,
        (h, m, s) => {
            let mut missing = Vec::new();
            if h.is_none() {
                missing.push("hour");
            }
            if m.is_none() {
                missing.push("minute");
            }
            if s.is_none() {
                missing.push("second");
            }
            return Err(vec![SourceDiagnostic::error(args.span, "time is incomplete")
                .with_hint(format!(
                    "add {} to get a valid time",
                    format_missing_args(&missing)
                ))]);
        }
    };

    let date = match (year, month, day) {
        (Some(y), Some(mo), Some(d)) => {
            let month =
                match u8::try_from(mo).ok().and_then(|m| time::Month::try_from(m).ok()) {
                    Some(m) => m,
                    None => {
                        return Err(vec![SourceDiagnostic::error(
                            args.span,
                            "month is invalid",
                        )])
                    }
                };
            let (Ok(y), Ok(d)) = (i32::try_from(y), u8::try_from(d)) else {
                return Err(vec![SourceDiagnostic::error(args.span, "date is invalid")]);
            };
            match time::Date::from_calendar_date(y, month, d) {
                Ok(date) => Some(date),
                Err(_) => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        "date is invalid",
                    )])
                }
            }
        }
        (None, None, None) => None,
        (y, mo, d) => {
            let mut missing = Vec::new();
            if y.is_none() {
                missing.push("year");
            }
            if mo.is_none() {
                missing.push("month");
            }
            if d.is_none() {
                missing.push("day");
            }
            return Err(vec![SourceDiagnostic::error(args.span, "date is incomplete")
                .with_hint(format!(
                    "add {} to get a valid date",
                    format_missing_args(&missing)
                ))]);
        }
    };

    match Datetime::from_parts(date, time) {
        Some(dt) => Ok(Value::Datetime(dt)),
        None => Err(vec![SourceDiagnostic::error(
            args.span,
            "at least one of date or time must be fully specified",
        )
        .with_hint("add the `hour`, `minute`, and `second` arguments to get a valid time")
        .with_hint("add the `year`, `month`, and `day` arguments to get a valid date")]),
    }
}

/// `symbol(...)` — constrói um símbolo Unicode nomeado com variantes.
///
/// **P765a**: paridade com vanilla CLI 0.15.0. Cada argumento posicional é
/// uma variante:
/// - string de um só grapheme → variante base;
/// - array `(modifiers, char)` → variante nomeada.
///
/// Ex.: `symbol("🖂", ("stamped", "🖃"))`.
pub fn native_symbol(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use unicode_segmentation::UnicodeSegmentation;

    expect_no_named(&args.named)?;
    if args.items.is_empty() {
        return err("expected at least one variant");
    }

    let mut variants: Vec<SymbolVariant> = Vec::with_capacity(args.items.len());
    for v in args.items.iter() {
        match v {
            Value::Str(s) => {
                let s = s.as_str();
                if s.graphemes(true).count() != 1 {
                    return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        format!(
                            "invalid variant value: {}",
                            s.escape_debug().collect::<String>()
                        ),
                    )]);
                }
                variants.push((EcoString::default(), s.chars().next().unwrap()));
            }
            Value::Array(arr) if arr.len() == 2 => {
                let mods = match &arr[0] {
                    Value::Str(s) => s.clone(),
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            Span::detached(),
                            format!(
                                "symbol modifier must be string, found {}",
                                other.type_name()
                            ),
                        )]);
                    }
                };
                let ch = match &arr[1] {
                    Value::Str(s) => {
                        let s = s.as_str();
                        if s.graphemes(true).count() != 1 {
                            return Err(vec![SourceDiagnostic::error(
                                Span::detached(),
                                format!(
                                    "invalid variant value: {}",
                                    s.escape_debug().collect::<String>()
                                ),
                            )]);
                        }
                        s.chars().next().unwrap()
                    }
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            Span::detached(),
                            format!(
                                "symbol variant value must be string, found {}",
                                other.type_name()
                            ),
                        )]);
                    }
                };
                variants.push((mods, ch));
            }
            Value::Array(arr) => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "symbol variant array must have length 2, found {}",
                        arr.len()
                    ),
                )]);
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "symbol variant must be string or array, found {}",
                        other.type_name()
                    ),
                )]);
            }
        }
    }

    Ok(Value::Symbol(Symbol::runtime(variants)))
}
#[cfg(test)]
mod tests_p704_range_step {
    use super::*;

    fn ctx() -> EvalContext {
        EvalContext::new()
    }
    fn tfid() -> FileId {
        FileId::from_raw(std::num::NonZeroU16::new(1).unwrap())
    }

    #[derive(Default)]
    struct NullWorld {
        library: crate::entities::world_types::Library,
        book: crate::entities::font_book::FontBook,
    }
    impl crate::contracts::world::World for NullWorld {
        fn library(&self) -> &crate::entities::world_types::Library {
            &self.library
        }
        fn book(&self) -> &crate::entities::font_book::FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            tfid()
        }
        fn source(
            &self,
            _: FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::source::Source>
        {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn file(
            &self,
            _: FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::world_types::Bytes>
        {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<crate::entities::world_types::Font> {
            None
        }
        fn today(
            &self,
            _: Option<i64>,
        ) -> Option<crate::entities::world_types::Datetime> {
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

    fn r(items: Vec<Value>, named: &[(&str, Value)]) -> Value {
        let mut args = Args::positional(items);
        for (k, v) in named {
            args.named.insert((*k).into(), v.clone());
        }
        native_range(&mut ctx(), &args, &NullWorld::default(), tfid()).unwrap()
    }

    fn arr(items: &[i64]) -> Value {
        Value::Array(items.iter().map(|i| Value::Int(*i)).collect())
    }

    #[test]
    fn sem_step_sem_regressao() {
        assert_eq!(r(vec![Value::Int(3)], &[]), arr(&[0, 1, 2]));
        assert_eq!(r(vec![Value::Int(2), Value::Int(5)], &[]), arr(&[2, 3, 4]));
        assert_eq!(r(vec![Value::Int(5), Value::Int(2)], &[]), arr(&[]));
    }

    #[test]
    fn negativo_devolve_vazio_nao_erro() {
        // P704 — corrige divergência: o vanilla devolve `()`, não Err.
        assert_eq!(r(vec![Value::Int(-5)], &[]), arr(&[]));
    }

    #[test]
    fn step_positivo_e_negativo() {
        assert_eq!(
            r(vec![Value::Int(90), Value::Int(40)], &[("step", Value::Int(-12))]),
            arr(&[90, 78, 66, 54, 42])
        );
        assert_eq!(
            r(vec![Value::Int(0), Value::Int(10)], &[("step", Value::Int(2))]),
            arr(&[0, 2, 4, 6, 8])
        );
        assert_eq!(
            r(vec![Value::Int(10), Value::Int(0)], &[("step", Value::Int(-1))]),
            arr(&[10, 9, 8, 7, 6, 5, 4, 3, 2, 1])
        );
    }

    #[test]
    fn step_com_1_argumento() {
        assert_eq!(
            r(vec![Value::Int(20)], &[("step", Value::Int(4))]),
            arr(&[0, 4, 8, 12, 16])
        );
        assert_eq!(
            r(vec![Value::Int(21)], &[("step", Value::Int(4))]),
            arr(&[0, 4, 8, 12, 16, 20])
        );
    }

    #[test]
    fn direcao_incompativel_devolve_vazio() {
        assert_eq!(
            r(vec![Value::Int(0), Value::Int(10)], &[("step", Value::Int(-1))]),
            arr(&[])
        );
    }

    #[test]
    fn step_com_inclusive() {
        assert_eq!(
            r(
                vec![Value::Int(-6)],
                &[("step", Value::Int(-2)), ("inclusive", Value::Bool(true))]
            ),
            arr(&[0, -2, -4, -6])
        );
        assert_eq!(
            r(vec![Value::Int(0)], &[("inclusive", Value::Bool(true))]),
            arr(&[0])
        );
        assert_eq!(
            r(vec![Value::Int(7), Value::Int(10)], &[("inclusive", Value::Bool(true))]),
            arr(&[7, 8, 9, 10])
        );
    }

    #[test]
    fn step_zero_erro_verbatim() {
        let mut args = Args::positional(vec![Value::Int(0), Value::Int(10)]);
        args.named.insert("step".into(), Value::Int(0));
        let e =
            native_range(&mut ctx(), &args, &NullWorld::default(), tfid()).unwrap_err();
        assert!(
            e[0].message.contains("number must not be zero"),
            "msg: {}",
            e[0].message
        );
    }
}
