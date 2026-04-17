//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/stdlib.md
//! @prompt-hash 3a694387
//! @layer L1
//! @updated 2026-04-13

//! Stdlib nativa mínima — Passo 17.
//!
//! Interface `fn(&Args) -> SourceResult<Value>` (Passo 64, DEBT-16):
//! aceita positional e named args. Sem moves, testável directamente sem world/eval.

use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::func::Func;
use crate::entities::layout_types::Length;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

fn err(msg: impl Into<String>) -> SourceResult<Value> {
    Err(vec![SourceDiagnostic::error(Span::detached(), msg.into())])
}

/// Verifica que não foram passados argumentos nomeados não esperados (Passo 64).
///
/// O Typst original é rigoroso: argumentos nomeados desconhecidos são
/// erros semânticos, não silenciosos. Ignorá-los criaria uma linguagem
/// permissiva que esconde typos do utilizador.
fn expect_no_named(named: &IndexMap<EcoString, Value, FxBuildHasher>) -> SourceResult<()> {
    if let Some((key, _)) = named.iter().next() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("argumento nomeado inesperado: '{}'", key),
        )]);
    }
    Ok(())
}

/// `type(v)` → nome do tipo como string Typst.
pub fn native_type(args: &Args) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [v] => Ok(Value::Str(v.type_name().into())),
        _   => err(format!("type() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

/// `len(v)` → comprimento de Str, Array ou Dict.
pub fn native_len(args: &Args) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(s)]   => Ok(Value::Int(s.chars().count() as i64)),
        [Value::Array(a)] => Ok(Value::Int(a.len() as i64)),
        [Value::Dict(d)]  => Ok(Value::Int(d.len() as i64)),
        [other]           => err(format!("len() não suporta {}", other.type_name())),
        _                 => err(format!("len() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

/// `rgb(r, g, b)` ou `rgb(r, g, b, a)` → Color.
///
/// Args em Int 0–255. Quatro args incluem canal alpha.
/// Fora de 0–255 → Err.
pub fn native_rgb(args: &Args) -> SourceResult<Value> {
    use crate::entities::layout_types::Color;
    expect_no_named(&args.named)?;
    fn check(v: i64, name: &str) -> SourceResult<u8> {
        if (0..=255).contains(&v) {
            Ok(v as u8)
        } else {
            Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("rgb(): componente {} fora de 0–255: {}", name, v),
            )])
        }
    }
    match args.items.as_slice() {
        [Value::Int(r), Value::Int(g), Value::Int(b)] => {
            Ok(Value::Color(Color::rgb(check(*r, "r")?, check(*g, "g")?, check(*b, "b")?)))
        }
        [Value::Int(r), Value::Int(g), Value::Int(b), Value::Int(a)] => {
            Ok(Value::Color(Color::rgba(check(*r, "r")?, check(*g, "g")?, check(*b, "b")?, check(*a, "a")?)))
        }
        _ => err(format!("rgb() requer 3 ou 4 Int, recebeu {} args", args.items.len())),
    }
}

/// `luma(l)` → Color::Rgb { r: l, g: l, b: l } (escala de cinzentos).
pub fn native_luma(args: &Args) -> SourceResult<Value> {
    use crate::entities::layout_types::Color;
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(l)] => {
            if !(0..=255).contains(l) {
                return err(format!("luma(): componente fora de 0–255: {}", l));
            }
            let l = *l as u8;
            Ok(Value::Color(Color::rgb(l, l, l)))
        }
        _ => err(format!("luma() requer 1 Int, recebeu {} args", args.items.len())),
    }
}

/// `range(n)` → Array de 0..n; `range(start, end)` → Array de start..end.
pub fn native_range(args: &Args) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(n)] => {
            if *n < 0 {
                return err("range() requer argumento não-negativo");
            }
            Ok(Value::Array((0..*n).map(Value::Int).collect()))
        }
        [Value::Int(start), Value::Int(end)] => {
            let items = if start <= end {
                (*start..*end).map(Value::Int).collect()
            } else {
                vec![]
            };
            Ok(Value::Array(items))
        }
        _ => err(format!("range() requer 1 ou 2 Int, recebeu {} args", args.items.len())),
    }
}

// ── Funções de conversão de tipo (Passo 27) ─────────────────────────────────

/// `str(v)` → representação textual do valor.
pub fn native_str(args: &Args) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [v] => {
            let s: String = match v {
                Value::None        => "none".into(),
                Value::Bool(b)     => if *b { "true" } else { "false" }.into(),
                Value::Int(i)      => i.to_string(),
                Value::Float(f)    => format_float(*f),
                Value::Str(s)      => return Ok(Value::Str(s.clone())),
                Value::Auto        => "auto".into(),
                Value::Length(l)   => format_length(l),
                Value::Ratio(r)    => format!("{}%", r.to_percent()),
                Value::Angle(a)    => format!("{}deg", a.to_deg()),
                Value::Color(_)    => return err("str() não suporta color"),
                other => return err(format!("str() não suporta {}", other.type_name())),
            };
            Ok(Value::Str(EcoString::from(s)))
        }
        _ => err(format!("str() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

/// Formata f64 de forma compacta — sem trailing zeros desnecessários.
fn format_float(f: f64) -> String {
    let s = format!("{}", f);
    if s.contains('.') || s.contains('e') { s } else { format!("{s}.0") }
}

/// Formata Length como string (ex: "12pt", "1.5em", "6pt + 1em").
fn format_length(l: &Length) -> String {
    let abs = l.abs.to_pt();
    let em  = l.em;
    match (abs == 0.0, em == 0.0) {
        (true,  true)  => "0pt".into(),
        (false, true)  => format!("{abs}pt"),
        (true,  false) => format!("{em}em"),
        (false, false) => format!("{abs}pt + {em}em"),
    }
}

/// `int(v)` → inteiro. Aceita Int, Str (decimal), Bool.
/// Float → Err (semântica vanilla: Float não é `ToInt`).
pub fn native_int(args: &Args) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(i)]    => Ok(Value::Int(*i)),
        [Value::Bool(b)]   => Ok(Value::Int(if *b { 1 } else { 0 })),
        [Value::Str(s)]    => s.parse::<i64>()
            .map(Value::Int)
            .map_err(|_| vec![SourceDiagnostic::error(
                Span::detached(),
                format!("int() não consegue parsear {:?}", s.as_str()),
            )]),
        [Value::Float(f)]  => err(format!(
            "int() não converte float {f} — usar int(calc.round(x)) ou int(calc.floor(x))"
        )),
        [other] => err(format!("int() não suporta {}", other.type_name())),
        _ => err(format!("int() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

/// `float(v)` → float. Aceita Float, Int (coerção), Str.
pub fn native_float(args: &Args) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Float(f)] => Ok(Value::Float(*f)),
        [Value::Int(i)]   => Ok(Value::Float(*i as f64)),
        [Value::Str(s)]   => s.parse::<f64>()
            .map(Value::Float)
            .map_err(|_| vec![SourceDiagnostic::error(
                Span::detached(),
                format!("float() não consegue parsear {:?}", s.as_str()),
            )]),
        [other] => err(format!("float() não suporta {}", other.type_name())),
        _ => err(format!("float() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

// ── Módulo calc (Passo 27) ───────────────────────────────────────────────────

/// Constrói o módulo `calc` como `Value::Dict` com 9 funções.
///
/// Divergência: original usa `Value::Module`. Cristalino usa `Value::Dict`
/// porque não temos stdlib Module sem world. Semântica de acesso (`calc.abs`)
/// é idêntica via `eval_field_access` sobre Dict.
pub fn make_calc_module() -> Value {
    let mut dict: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
    dict.insert("abs".into(),   Value::Func(Func::native("calc.abs",   calc_abs)));
    dict.insert("pow".into(),   Value::Func(Func::native("calc.pow",   calc_pow)));
    dict.insert("sqrt".into(),  Value::Func(Func::native("calc.sqrt",  calc_sqrt)));
    dict.insert("floor".into(), Value::Func(Func::native("calc.floor", calc_floor)));
    dict.insert("ceil".into(),  Value::Func(Func::native("calc.ceil",  calc_ceil)));
    dict.insert("round".into(), Value::Func(Func::native("calc.round", calc_round)));
    dict.insert("min".into(),   Value::Func(Func::native("calc.min",   calc_min)));
    dict.insert("max".into(),   Value::Func(Func::native("calc.max",   calc_max)));
    dict.insert("clamp".into(), Value::Func(Func::native("calc.clamp", calc_clamp)));
    Value::Dict(dict)
}

pub(crate) fn calc_abs(args: &Args) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(i)]   => Ok(Value::Int(i.saturating_abs())),
        [Value::Float(f)] => Ok(Value::Float(f.abs())),
        [other] => err(format!("calc.abs() requer Int ou Float, recebeu {}", other.type_name())),
        _ => err(format!("calc.abs() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

pub(crate) fn calc_pow(args: &Args) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(base), Value::Int(exp)] => {
            if *exp < 0 {
                return err("calc.pow() expoente negativo requer Float");
            }
            Ok(Value::Int(base.saturating_pow(*exp as u32)))
        }
        [base, exp] => {
            let b = coerce_to_f64(base, "calc.pow() base")?;
            let e = coerce_to_f64(exp,  "calc.pow() expoente")?;
            // DEBT: migrar para libm::pow quando libm for dependência do workspace (ADR-0018)
            #[allow(clippy::disallowed_methods)]
            guard_float(b.powf(e))
        }
        _ => err(format!("calc.pow() requer 2 argumentos, recebeu {}", args.items.len())),
    }
}

pub(crate) fn calc_sqrt(args: &Args) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [v] => {
            let f = coerce_to_f64(v, "calc.sqrt()")?;
            if f < 0.0 {
                return err("calc.sqrt() argumento negativo");
            }
            guard_float(f.sqrt())
        }
        _ => err(format!("calc.sqrt() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

pub(crate) fn calc_floor(args: &Args) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(i)]   => Ok(Value::Int(*i)),
        [Value::Float(f)] => Ok(Value::Int(f.floor() as i64)),
        [other] => err(format!("calc.floor() requer Int ou Float, recebeu {}", other.type_name())),
        _ => err(format!("calc.floor() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

pub(crate) fn calc_ceil(args: &Args) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(i)]   => Ok(Value::Int(*i)),
        [Value::Float(f)] => Ok(Value::Int(f.ceil() as i64)),
        [other] => err(format!("calc.ceil() requer Int ou Float, recebeu {}", other.type_name())),
        _ => err(format!("calc.ceil() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

pub(crate) fn calc_round(args: &Args) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(i)]   => Ok(Value::Int(*i)),
        [Value::Float(f)] => Ok(Value::Int(f.round() as i64)),
        [other] => err(format!("calc.round() requer Int ou Float, recebeu {}", other.type_name())),
        _ => err(format!("calc.round() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

pub(crate) fn calc_min(args: &Args) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    if args.items.is_empty() {
        return err("calc.min() requer pelo menos 1 argumento");
    }
    let mut result = args.items[0].clone();
    for v in &args.items[1..] {
        result = match (&result, v) {
            (Value::Int(a),   Value::Int(b))   => Value::Int(*a.min(b)),
            (Value::Float(a), Value::Float(b)) => Value::Float(a.min(*b)),
            (Value::Int(a),   Value::Float(b)) => Value::Float((*a as f64).min(*b)),
            (Value::Float(a), Value::Int(b))   => Value::Float(a.min(*b as f64)),
            (_, other) => return err(format!(
                "calc.min() tipos incompatíveis: {} e {}", result.type_name(), other.type_name()
            )),
        };
    }
    Ok(result)
}

pub(crate) fn calc_max(args: &Args) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    if args.items.is_empty() {
        return err("calc.max() requer pelo menos 1 argumento");
    }
    let mut result = args.items[0].clone();
    for v in &args.items[1..] {
        result = match (&result, v) {
            (Value::Int(a),   Value::Int(b))   => Value::Int(*a.max(b)),
            (Value::Float(a), Value::Float(b)) => Value::Float(a.max(*b)),
            (Value::Int(a),   Value::Float(b)) => Value::Float((*a as f64).max(*b)),
            (Value::Float(a), Value::Int(b))   => Value::Float(a.max(*b as f64)),
            (_, other) => return err(format!(
                "calc.max() tipos incompatíveis: {} e {}", result.type_name(), other.type_name()
            )),
        };
    }
    Ok(result)
}

pub(crate) fn calc_clamp(args: &Args) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(v), Value::Int(lo), Value::Int(hi)] =>
            Ok(Value::Int((*v).clamp(*lo, *hi))),
        [v, lo, hi] => {
            let vf  = coerce_to_f64(v,  "calc.clamp() value")?;
            let lof = coerce_to_f64(lo, "calc.clamp() min")?;
            let hif = coerce_to_f64(hi, "calc.clamp() max")?;
            if lof > hif {
                return err(format!("calc.clamp() min ({lof}) > max ({hif})"));
            }
            Ok(Value::Float(vf.clamp(lof, hif)))
        }
        _ => err(format!("calc.clamp() requer 3 argumentos, recebeu {}", args.items.len())),
    }
}

fn coerce_to_f64(v: &Value, ctx: &str) -> SourceResult<f64> {
    match v {
        Value::Int(i)   => Ok(*i as f64),
        Value::Float(f) => Ok(*f),
        other => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{ctx}: esperava Int ou Float, recebeu {}", other.type_name()),
        )]),
    }
}

fn guard_float(f: f64) -> SourceResult<Value> {
    if f.is_nan()           { err("resultado não é um número (NaN)") }
    else if f.is_infinite() { err("resultado é infinito") }
    else                    { Ok(Value::Float(f)) }
}

// ── `figure()` — migrada de eval.rs (Passo 64, DEBT-16) ─────────────────────

/// `figure(body, caption: content)` → `Content::Figure`.
///
/// Migrada do interceptador em `eval.rs` para `stdlib.rs` — o avaliador deixa
/// de conhecer o nome "figure" (DEBT-16 encerrado).
///
/// - `body`: argumento posicional obrigatório.
/// - `caption:`: argumento nomeado opcional; `none` → sem legenda.
pub fn native_figure(args: &Args) -> SourceResult<Value> {
    // Argumento posicional: body (obrigatório)
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s))     => Content::text(s.as_str()),
        Some(_)                 => Content::Empty,
        None => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "figure() requer um argumento posicional (body)".to_string(),
        )]),
    };

    // Argumento nomeado: caption (opcional)
    // Value::None → ausência de legenda (comportamento intencional).
    let caption = args.named.get("caption").and_then(|v| match v {
        Value::Content(c) => Some(Box::new(c.clone())),
        Value::Str(s)     => Some(Box::new(Content::text(s.as_str()))),
        Value::None       => None,
        other             => Some(Box::new(Content::text(other.type_name()))),
    });

    Ok(Value::Content(Content::Figure {
        body:    Box::new(body),
        caption,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper de teste: cria Args apenas com posicionais.
    fn p(items: Vec<Value>) -> Args {
        Args::positional(items)
    }

    /// Helper de teste: cria Args com um named arg.
    fn pn(items: Vec<Value>, key: &str, val: Value) -> Args {
        let mut a = Args::positional(items);
        a.named.insert(key.into(), val);
        a
    }

    #[test]
    fn native_type_directo() {
        assert_eq!(native_type(&p(vec![Value::Int(1)])).unwrap(),     Value::Str("int".into()));
        assert_eq!(native_type(&p(vec![Value::Bool(true)])).unwrap(), Value::Str("bool".into()));
        assert_eq!(native_type(&p(vec![Value::None])).unwrap(),       Value::Str("none".into()));
        assert!(native_type(&p(vec![])).is_err());
        assert!(native_type(&p(vec![Value::Int(1), Value::Int(2)])).is_err());
    }

    #[test]
    fn native_type_named_arg_retorna_err() {
        let args = pn(vec![Value::Int(1)], "extra", Value::Bool(true));
        assert!(native_type(&args).is_err(), "named arg inesperado deve retornar Err");
    }

    #[test]
    fn native_len_directo() {
        assert_eq!(native_len(&p(vec![Value::Str("abc".into())])).unwrap(), Value::Int(3));
        assert_eq!(
            native_len(&p(vec![Value::Array(vec![Value::Int(1), Value::Int(2)])])).unwrap(),
            Value::Int(2)
        );
        assert!(native_len(&p(vec![Value::Int(1)])).is_err());
        assert!(native_len(&p(vec![])).is_err());
    }

    // ── Passo 25 — rgb/luma ──────────────────────────────────────────────────

    #[test]
    fn stdlib_rgb_tres_args() {
        use crate::entities::layout_types::Color;
        let r = native_rgb(&p(vec![Value::Int(255), Value::Int(0), Value::Int(128)])).unwrap();
        assert_eq!(r, Value::Color(Color::rgb(255, 0, 128)));
    }

    #[test]
    fn stdlib_rgb_quatro_args() {
        use crate::entities::layout_types::Color;
        let r = native_rgb(&p(vec![Value::Int(255), Value::Int(0), Value::Int(0), Value::Int(200)])).unwrap();
        assert_eq!(r, Value::Color(Color::rgba(255, 0, 0, 200)));
    }

    #[test]
    fn stdlib_rgb_out_of_range() {
        assert!(native_rgb(&p(vec![Value::Int(300), Value::Int(0), Value::Int(0)])).is_err());
    }

    #[test]
    fn stdlib_luma() {
        use crate::entities::layout_types::Color;
        let r = native_luma(&p(vec![Value::Int(128)])).unwrap();
        assert_eq!(r, Value::Color(Color::rgb(128, 128, 128)));
    }

    // ── Passo 27 — str/int/float ─────────────────────────────────────────────

    #[test]
    fn native_str_de_int() {
        assert_eq!(native_str(&p(vec![Value::Int(42)])).unwrap(), Value::Str("42".into()));
    }

    #[test]
    #[allow(clippy::approx_constant)] // 3.14 é valor literal de teste, não aproximação de PI
    fn native_str_de_float() {
        assert_eq!(native_str(&p(vec![Value::Float(3.14)])).unwrap(), Value::Str("3.14".into()));
    }

    #[test]
    fn native_str_de_bool() {
        assert_eq!(native_str(&p(vec![Value::Bool(true)])).unwrap(),  Value::Str("true".into()));
        assert_eq!(native_str(&p(vec![Value::Bool(false)])).unwrap(), Value::Str("false".into()));
    }

    #[test]
    fn native_str_identity() {
        assert_eq!(native_str(&p(vec![Value::Str("hello".into())])).unwrap(), Value::Str("hello".into()));
    }

    #[test]
    fn native_str_de_none() {
        assert_eq!(native_str(&p(vec![Value::None])).unwrap(), Value::Str("none".into()));
    }

    #[test]
    fn native_int_de_int() {
        assert_eq!(native_int(&p(vec![Value::Int(42)])).unwrap(), Value::Int(42));
    }

    #[test]
    fn native_int_de_str() {
        assert_eq!(native_int(&p(vec![Value::Str("42".into())])).unwrap(), Value::Int(42));
        assert!(native_int(&p(vec![Value::Str("abc".into())])).is_err());
    }

    #[test]
    fn native_int_de_bool() {
        assert_eq!(native_int(&p(vec![Value::Bool(true)])).unwrap(),  Value::Int(1));
        assert_eq!(native_int(&p(vec![Value::Bool(false)])).unwrap(), Value::Int(0));
    }

    #[test]
    fn native_int_float_retorna_err() {
        assert!(native_int(&p(vec![Value::Float(3.7)])).is_err());
    }

    #[test]
    fn native_float_de_int() {
        assert_eq!(native_float(&p(vec![Value::Int(3)])).unwrap(), Value::Float(3.0));
    }

    #[test]
    #[allow(clippy::approx_constant)] // 3.14 é valor literal de teste, não aproximação de PI
    fn native_float_de_str() {
        assert_eq!(native_float(&p(vec![Value::Str("3.14".into())])).unwrap(), Value::Float(3.14));
        assert!(native_float(&p(vec![Value::Str("abc".into())])).is_err());
    }

    // ── Passo 27 — calc ──────────────────────────────────────────────────────

    #[test]
    fn calc_abs_int() {
        assert_eq!(calc_abs(&p(vec![Value::Int(-5)])).unwrap(), Value::Int(5));
        assert_eq!(calc_abs(&p(vec![Value::Int(5)])).unwrap(),  Value::Int(5));
        assert_eq!(calc_abs(&p(vec![Value::Int(0)])).unwrap(),  Value::Int(0));
    }

    #[test]
    #[allow(clippy::approx_constant)] // 3.14 é valor literal de teste, não aproximação de PI
    fn calc_abs_float() {
        assert_eq!(calc_abs(&p(vec![Value::Float(-3.14)])).unwrap(), Value::Float(3.14));
    }

    #[test]
    fn calc_pow_int() {
        assert_eq!(calc_pow(&p(vec![Value::Int(2), Value::Int(10)])).unwrap(), Value::Int(1024));
        assert_eq!(calc_pow(&p(vec![Value::Int(2), Value::Int(0)])).unwrap(),  Value::Int(1));
    }

    #[test]
    fn calc_pow_float() {
        let r = calc_pow(&p(vec![Value::Float(2.0), Value::Float(0.5)])).unwrap();
        assert!(matches!(r, Value::Float(f) if (f - std::f64::consts::SQRT_2).abs() < 1e-10));
    }

    #[test]
    fn calc_pow_negativo_retorna_err() {
        assert!(calc_pow(&p(vec![Value::Int(2), Value::Int(-1)])).is_err());
    }

    #[test]
    fn calc_sqrt_positivo() {
        assert_eq!(calc_sqrt(&p(vec![Value::Float(4.0)])).unwrap(), Value::Float(2.0));
        assert_eq!(calc_sqrt(&p(vec![Value::Int(4)])).unwrap(),     Value::Float(2.0));
    }

    #[test]
    fn calc_sqrt_negativo_retorna_err() {
        assert!(calc_sqrt(&p(vec![Value::Float(-1.0)])).is_err());
    }

    #[test]
    fn calc_floor_ceil_round() {
        assert_eq!(calc_floor(&p(vec![Value::Float(3.7)])).unwrap(), Value::Int(3));
        assert_eq!(calc_ceil(&p(vec![Value::Float(3.2)])).unwrap(),  Value::Int(4));
        assert_eq!(calc_round(&p(vec![Value::Float(3.5)])).unwrap(), Value::Int(4));
        assert_eq!(calc_round(&p(vec![Value::Float(3.4)])).unwrap(), Value::Int(3));
    }

    #[test]
    fn calc_min_max_int() {
        assert_eq!(calc_min(&p(vec![Value::Int(3), Value::Int(1), Value::Int(2)])).unwrap(), Value::Int(1));
        assert_eq!(calc_max(&p(vec![Value::Int(3), Value::Int(1), Value::Int(2)])).unwrap(), Value::Int(3));
    }

    #[test]
    fn calc_min_vazio_retorna_err() {
        assert!(calc_min(&p(vec![])).is_err());
        assert!(calc_max(&p(vec![])).is_err());
    }

    #[test]
    fn calc_clamp_int() {
        assert_eq!(calc_clamp(&p(vec![Value::Int(5),  Value::Int(0), Value::Int(10)])).unwrap(), Value::Int(5));
        assert_eq!(calc_clamp(&p(vec![Value::Int(-5), Value::Int(0), Value::Int(10)])).unwrap(), Value::Int(0));
        assert_eq!(calc_clamp(&p(vec![Value::Int(15), Value::Int(0), Value::Int(10)])).unwrap(), Value::Int(10));
    }

    #[test]
    fn calc_clamp_min_maior_max_retorna_err() {
        assert!(calc_clamp(&p(vec![Value::Float(5.0), Value::Float(10.0), Value::Float(0.0)])).is_err());
    }

    #[test]
    fn native_range_directo() {
        assert_eq!(native_range(&p(vec![Value::Int(3)])).unwrap(),
                   Value::Array(vec![Value::Int(0), Value::Int(1), Value::Int(2)]));
        assert_eq!(native_range(&p(vec![Value::Int(2), Value::Int(5)])).unwrap(),
                   Value::Array(vec![Value::Int(2), Value::Int(3), Value::Int(4)]));
        assert_eq!(native_range(&p(vec![Value::Int(3), Value::Int(3)])).unwrap(),
                   Value::Array(vec![]));
        assert!(native_range(&p(vec![Value::Int(-1)])).is_err());
    }

    // ── Passo 64 — native_figure (DEBT-16) ──────────────────────────────────

    #[test]
    fn native_figure_com_body_e_caption() {
        use crate::entities::content::Content;
        let body_content = Content::text("Gráfico");
        let caption_content = Content::text("Legenda");
        let args = pn(
            vec![Value::Content(body_content)],
            "caption",
            Value::Content(caption_content),
        );
        let result = native_figure(&args).unwrap();
        assert!(matches!(result, Value::Content(Content::Figure { caption: Some(_), .. })),
            "figure com caption deve ter Some(caption): {:?}", result);
    }

    #[test]
    fn native_figure_sem_caption() {
        use crate::entities::content::Content;
        let body_content = Content::text("Diagrama");
        let args = p(vec![Value::Content(body_content)]);
        let result = native_figure(&args).unwrap();
        assert!(matches!(result, Value::Content(Content::Figure { caption: None, .. })),
            "figure sem caption deve ter None: {:?}", result);
    }

    #[test]
    fn native_figure_caption_none_value() {
        use crate::entities::content::Content;
        // caption: none → ausência de legenda
        let body_content = Content::text("Corpo");
        let args = pn(vec![Value::Content(body_content)], "caption", Value::None);
        let result = native_figure(&args).unwrap();
        assert!(matches!(result, Value::Content(Content::Figure { caption: None, .. })),
            "figure com caption: none deve ter caption None");
    }

    #[test]
    fn native_figure_sem_body_retorna_err() {
        let args = p(vec![]);
        assert!(native_figure(&args).is_err(), "figure sem body deve retornar Err");
    }

    #[test]
    fn expect_no_named_retorna_err_com_named_arg() {
        let mut a = p(vec![]);
        a.named.insert("foo".into(), Value::Int(1));
        let result: SourceResult<()> = expect_no_named(&a.named);
        assert!(result.is_err());
        let err_msg = &result.unwrap_err()[0].message;
        assert!(err_msg.contains("inesperado"), "mensagem: {:?}", err_msg);
    }

    #[test]
    fn expect_no_named_ok_com_vazio() {
        let a = p(vec![]);
        assert!(expect_no_named(&a.named).is_ok());
    }
}
