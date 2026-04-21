//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/stdlib.md
//! @prompt-hash f6cc2443
//! @layer L1
//! @updated 2026-04-20

//! Stdlib nativa mínima — Passo 17.
//!
//! Interface `fn(&mut EvalContext<'_>, &Args) -> SourceResult<Value>` (Passo 71, DEBT-24):
//! aceita positional e named args. Funções sem I/O usam `_ctx`.

use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::geometry::{PathItem, ShapeKind, Stroke};
use crate::entities::ptr_eq_arc::PtrEqArc;
use crate::entities::func::Func;
use crate::entities::layout_types::{Align2D, Color, Length, Point, Pt, TrackSizing, TransformMatrix};
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;
use crate::rules::eval::EvalContext;

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
pub fn native_type(_ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [v] => Ok(Value::Str(v.type_name().into())),
        _   => err(format!("type() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

/// `len(v)` → comprimento de Str, Array ou Dict.
pub fn native_len(_ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
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
pub fn native_rgb(_ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
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
pub fn native_luma(_ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
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
pub fn native_range(_ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
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
pub fn native_str(_ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
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
pub fn native_int(_ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
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
pub fn native_float(_ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
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

pub(crate) fn calc_abs(_ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(i)]   => Ok(Value::Int(i.saturating_abs())),
        [Value::Float(f)] => Ok(Value::Float(f.abs())),
        [other] => err(format!("calc.abs() requer Int ou Float, recebeu {}", other.type_name())),
        _ => err(format!("calc.abs() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

pub(crate) fn calc_pow(_ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
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

pub(crate) fn calc_sqrt(_ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
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

pub(crate) fn calc_floor(_ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(i)]   => Ok(Value::Int(*i)),
        [Value::Float(f)] => Ok(Value::Int(f.floor() as i64)),
        [other] => err(format!("calc.floor() requer Int ou Float, recebeu {}", other.type_name())),
        _ => err(format!("calc.floor() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

pub(crate) fn calc_ceil(_ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(i)]   => Ok(Value::Int(*i)),
        [Value::Float(f)] => Ok(Value::Int(f.ceil() as i64)),
        [other] => err(format!("calc.ceil() requer Int ou Float, recebeu {}", other.type_name())),
        _ => err(format!("calc.ceil() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

pub(crate) fn calc_round(_ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(i)]   => Ok(Value::Int(*i)),
        [Value::Float(f)] => Ok(Value::Int(f.round() as i64)),
        [other] => err(format!("calc.round() requer Int ou Float, recebeu {}", other.type_name())),
        _ => err(format!("calc.round() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

pub(crate) fn calc_min(_ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
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

pub(crate) fn calc_max(_ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
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

pub(crate) fn calc_clamp(_ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
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

// ── `upper()` / `lower()` / `replace()` — motor map_text (Passo 67) ─────────

/// `upper(str | content)` → texto em maiúsculas.
pub fn native_upper(_ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(s)] => Ok(Value::Str(s.to_uppercase())),
        [Value::Content(c)] => {
            let mut f = |text: &str| text.to_uppercase();
            Ok(Value::Content(c.map_text(&mut f)))
        }
        [other] => err(format!("upper() espera string ou content, recebeu {}", other.type_name())),
        _ => err("upper() requer 1 argumento".to_string()),
    }
}

/// `lower(str | content)` → texto em minúsculas.
pub fn native_lower(_ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(s)] => Ok(Value::Str(s.to_lowercase())),
        [Value::Content(c)] => {
            let mut f = |text: &str| text.to_lowercase();
            Ok(Value::Content(c.map_text(&mut f)))
        }
        [other] => err(format!("lower() espera string ou content, recebeu {}", other.type_name())),
        _ => err("lower() requer 1 argumento".to_string()),
    }
}

/// `replace(fonte, padrão, substituição, count: N)` → string ou content com substituição.
///
/// `count` é global ao documento: persiste entre nós de texto via `FnMut`.
pub fn native_replace(_ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
    // Validar named args: apenas "count" é aceite.
    for key in args.named.keys() {
        if key.as_str() != "count" {
            return err(format!("argumento nomeado inesperado: '{}'", key));
        }
    }

    if args.items.len() < 3 {
        return err("replace() requer 3 argumentos: fonte, padrão, substituição".to_string());
    }

    let pattern = match &args.items[1] {
        Value::Str(s) => s.to_string(),
        other => return err(format!("replace(): padrão deve ser string, recebeu {}", other.type_name())),
    };
    let replacement = match &args.items[2] {
        Value::Str(s) => s.to_string(),
        other => return err(format!("replace(): substituição deve ser string, recebeu {}", other.type_name())),
    };

    // Bloquear padrão vazio: replacen("", ...) entra em ciclo infinito.
    if pattern.is_empty() {
        return err("replace(): o padrão de busca não pode estar vazio".to_string());
    }

    let mut remaining_count: Option<i64> = args.named.get("count")
        .and_then(|v| match v {
            Value::Int(i) => Some(*i),
            _ => None,
        });

    // A closure carrega `remaining_count` como estado mutável.
    // `map_text` usa `&mut F`, portanto o estado persiste entre nós do AST.
    // Isto garante que `count: N` é global ao documento, não por nó de texto.
    let mut do_replace = |text: &str| -> String {
        match remaining_count.as_mut() {
            Some(c) if *c <= 0 => text.to_string(),
            Some(c) => {
                let limit = *c as usize;
                let count_used = text.matches(pattern.as_str()).take(limit).count();
                let result = text.replacen(pattern.as_str(), replacement.as_str(), limit);
                *c -= count_used as i64;
                result
            }
            None => text.replace(pattern.as_str(), replacement.as_str()),
        }
    };

    match &args.items[0] {
        Value::Str(s) => Ok(Value::Str(do_replace(s.as_str()).into())),
        Value::Content(c) => Ok(Value::Content(c.map_text(&mut do_replace))),
        other => err(format!("replace(): 1º argumento deve ser string ou content, recebeu {}", other.type_name())),
    }
}

// ── `assert()` — prova de fogo dos named args (Passo 66, DEBT-16) ───────────

/// `assert(condition, message: ...)` → sem output; erro se condição for falsa.
///
/// Primeira função com named arg documentado (não apenas tolerado).
/// Prova de que o mecanismo de named args (DEBT-16) funciona de ponta a ponta.
pub fn native_assert(_ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
    // Validar named args: apenas "message" é aceite.
    for key in args.named.keys() {
        if key.as_str() != "message" {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("argumento nomeado inesperado: '{}'", key),
            )]);
        }
    }

    // Argumento posicional: condição (obrigatório).
    let condition = match args.items.first() {
        Some(Value::Bool(b)) => *b,
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("assert() requer condição booleana, recebeu {}", other.type_name()),
        )]),
        None => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "assert() requer 1 argumento posicional (condição)".to_string(),
        )]),
    };

    // Argumento nomeado: message (opcional).
    let message = args.named.get("message")
        .map(|v| match v {
            Value::Str(s)     => s.to_string(),
            Value::Content(c) => c.plain_text(),
            other             => other.type_name().to_string(),
        })
        .unwrap_or_else(|| "Asserção falhou".to_string());

    if !condition {
        return Err(vec![SourceDiagnostic::error(Span::detached(), message)]);
    }

    Ok(Value::None)
}

// ── Sentinelas e construtores de nós estruturais (Passo 69) ─────────────────

/// `strong(body)` — cria `Content::Strong` ou serve como selector em show rules.
pub fn native_strong(_ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s))     => Content::text(s.as_str()),
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("strong() espera content ou string, recebeu {}", other.type_name()),
        )]),
        None => Content::Empty,
    };
    Ok(Value::Content(Content::Strong(Box::new(body))))
}

/// `emph(body)` — cria `Content::Emph` ou serve como selector em show rules.
pub fn native_emph(_ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s))     => Content::text(s.as_str()),
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("emph() espera content ou string, recebeu {}", other.type_name()),
        )]),
        None => Content::Empty,
    };
    Ok(Value::Content(Content::Emph(Box::new(body))))
}

/// `raw(text)` — cria `Content::Raw` ou serve como selector em show rules.
/// Aceita apenas string — não faz sentido semântico aceitar Content aqui.
pub fn native_raw(_ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    let text: EcoString = match args.items.first() {
        Some(Value::Str(s)) => s.clone(),
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("raw() espera string, recebeu {}", other.type_name()),
        )]),
        None => EcoString::default(),
    };
    Ok(Value::Content(Content::Raw { text, lang: None, block: false }))
}

// ── `heading()` — sentinel para show rules (Passo 68, DEBT-21) ──────────────

/// Sentinel de `heading` como função — existe em scope para que show rules
/// do tipo `#show heading: it => ...` possam resolver o selector.
///
/// A criação real de headings usa a sintaxe de markup `= Título`.
/// Chamar `heading()` directamente retorna Err (DEBT-21).
pub fn native_heading(_ctx: &mut EvalContext<'_>, _args: &Args) -> SourceResult<Value> {
    Err(vec![SourceDiagnostic::error(
        Span::detached(),
        "heading() como função directa não suportada; use a sintaxe de markup `= Título`"
            .to_string(),
    )])
}

// ── `figure()` — migrada de eval.rs (Passo 64, DEBT-16) ─────────────────────

/// `figure(body, caption: content)` → `Content::Figure`.
///
/// Migrada do interceptador em `eval.rs` para `stdlib.rs` — o avaliador deixa
/// de conhecer o nome "figure" (DEBT-16 encerrado).
///
/// - `body`: argumento posicional obrigatório.
/// - `caption:`: argumento nomeado opcional; `none` → sem legenda.
pub fn native_figure(ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
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

    // Argumento nomeado: kind (Passo 75, DEBT-15).
    let kind = args.named.get("kind")
        .and_then(|v| if let Value::Str(s) = v { Some(s.to_string()) } else { None })
        .unwrap_or_else(|| "image".to_string());

    // Numeração capturada do contexto (Passo 75, DEBT-14).
    // Reflecte o estado activo de `#set figure(numbering: ...)` no momento da chamada.
    let numbering = ctx.figure_numbering.clone();

    Ok(Value::Content(Content::Figure {
        body: Box::new(body),
        caption,
        kind,
        numbering,
    }))
}

// ── `image()` — carregamento de imagens do disco (Passo 71, DEBT-24) ─────────

/// `image(path, width?, height?)` → `Content::Image`.
///
/// Lê os bytes do ficheiro através de `ctx.world.read_bytes(path)`.
/// `width` e `height` são preservados no AST para o Passo 72 (dimensões reais).
/// O layouter usa placeholder 100×100 pt neste passo (DEBT-24b).
pub fn native_image(ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
    // Validar named args: apenas "width" e "height" são aceites.
    for key in args.named.keys() {
        if key.as_str() != "width" && key.as_str() != "height" {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("argumento nomeado inesperado em image(): '{}'", key),
            )]);
        }
    }

    let path = match args.items.first() {
        Some(Value::Str(s)) => s.to_string(),
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("image() requer string com o caminho, recebeu {}", other.type_name()),
        )]),
        None => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "image() requer 1 argumento posicional (caminho do ficheiro)".to_string(),
        )]),
    };

    let data = match ctx.world.read_bytes(ctx.current_file, &path) {
        Ok(arc) => arc,
        Err(msg) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("image(): não foi possível ler '{}': {}", path, msg),
        )]),
    };

    let width  = args.named.get("width").cloned().map(Box::new);
    let height = args.named.get("height").cloned().map(Box::new);

    Ok(Value::Content(Content::Image { path, data: PtrEqArc(data), width, height }))
}

// ── Primitivas geométricas (Passo 76) ────────────────────────────────────────

/// Converte um `Value` em `Color`.
///
/// Suporta nomes de cor conhecidos (`Value::Str`) e `Value::Color` directo.
/// Valores hex (`#rrggbb`) ficam para passo futuro — o parser real de cores
/// Typst requer um lexer dedicado.
fn parse_color(val: &Value) -> Option<Color> {
    match val {
        Value::Color(c) => Some(*c),
        Value::Str(s) => match s.as_str() {
            "red"   => Some(Color::rgb(255, 0,   0)),
            "green" => Some(Color::rgb(0,   128, 0)),
            "blue"  => Some(Color::rgb(0,   0,   255)),
            "black" => Some(Color::rgb(0,   0,   0)),
            "white" => Some(Color::rgb(255, 255, 255)),
            _       => None,
        },
        _ => None,
    }
}

/// `rect(width?, height?, fill?, stroke?)` → `Content::Shape { kind: Rect, ... }`.
///
/// Fallback determinístico: sem `fill` nem `stroke` → stroke preta de 1pt.
/// Este é o único local onde este fallback existe — nem o layouter nem o
/// exportador têm permissão para inventar cores ou espessuras.
pub fn native_rect(_ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
    for key in args.named.keys() {
        if !["width", "height", "fill", "stroke"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("argumento nomeado inesperado em rect(): '{}'", key),
            )]);
        }
    }

    let width  = args.named.get("width").cloned().map(Box::new);
    let height = args.named.get("height").cloned().map(Box::new);
    let fill   = args.named.get("fill").and_then(parse_color);

    let parsed_stroke: Option<Stroke> = args.named.get("stroke")
        .and_then(parse_color)
        .map(|c| Stroke { paint: c, thickness: 1.0 });

    // Fallback determinístico: sem fill nem stroke → stroke preta de 1pt.
    let final_stroke = if fill.is_none() && parsed_stroke.is_none() {
        Some(Stroke { paint: Color::rgb(0, 0, 0), thickness: 1.0 })
    } else {
        parsed_stroke
    };

    Ok(Value::Content(Content::Shape {
        kind:   ShapeKind::Rect,
        width,
        height,
        fill,
        stroke: final_stroke,
    }))
}

/// `ellipse(width?, height?, fill?, stroke?)` → `Content::Shape { kind: Ellipse, ... }`.
///
/// Mesmo padrão de fallback que `native_rect`.
pub fn native_ellipse(_ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
    for key in args.named.keys() {
        if !["width", "height", "fill", "stroke"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("argumento nomeado inesperado em ellipse(): '{}'", key),
            )]);
        }
    }

    let width  = args.named.get("width").cloned().map(Box::new);
    let height = args.named.get("height").cloned().map(Box::new);
    let fill   = args.named.get("fill").and_then(parse_color);

    let parsed_stroke: Option<Stroke> = args.named.get("stroke")
        .and_then(parse_color)
        .map(|c| Stroke { paint: c, thickness: 1.0 });

    let final_stroke = if fill.is_none() && parsed_stroke.is_none() {
        Some(Stroke { paint: Color::rgb(0, 0, 0), thickness: 1.0 })
    } else {
        parsed_stroke
    };

    Ok(Value::Content(Content::Shape {
        kind: ShapeKind::Ellipse,
        width,
        height,
        fill,
        stroke: final_stroke,
    }))
}

/// `circle(radius?, fill?, stroke?)` → `Content::Shape { kind: Ellipse, width==height }`.
///
/// `radius` em pt. Converte para `width = height = radius * 2`.
pub fn native_circle(_ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
    for key in args.named.keys() {
        if !["radius", "fill", "stroke"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("argumento nomeado inesperado em circle(): '{}'", key),
            )]);
        }
    }

    fn extract_pt(val: &Value) -> f64 {
        match val {
            Value::Float(f)  => *f,
            Value::Int(i)    => *i as f64,
            Value::Length(l) => l.abs.to_pt(),
            _ => 0.0,
        }
    }

    let (width, height) = match args.named.get("radius") {
        Some(r) => {
            let diameter = Value::Float(extract_pt(r) * 2.0);
            (Some(Box::new(diameter.clone())), Some(Box::new(diameter)))
        }
        None => (None, None),
    };

    let fill = args.named.get("fill").and_then(parse_color);

    let parsed_stroke: Option<Stroke> = args.named.get("stroke")
        .and_then(parse_color)
        .map(|c| Stroke { paint: c, thickness: 1.0 });

    let final_stroke = if fill.is_none() && parsed_stroke.is_none() {
        Some(Stroke { paint: Color::rgb(0, 0, 0), thickness: 1.0 })
    } else {
        parsed_stroke
    };

    Ok(Value::Content(Content::Shape {
        kind: ShapeKind::Ellipse,
        width,
        height,
        fill,
        stroke: final_stroke,
    }))
}

/// `line(dx?, dy?, stroke?)` → `Content::Shape { kind: Line, ... }`.
///
/// `dx`/`dy`: Float ou Length em pt. Omitidos → 0.0 (linha degenerada, válida).
/// Stroke preta por omissão — linhas não têm fill.
pub fn native_line(_ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
    for key in args.named.keys() {
        if !["dx", "dy", "stroke"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("argumento nomeado inesperado em line(): '{}'", key),
            )]);
        }
    }

    fn extract_pt(val: &Value) -> f64 {
        match val {
            Value::Float(f) => *f,
            Value::Int(i)   => *i as f64,
            Value::Length(l) => l.abs.to_pt(),
            _ => 0.0,
        }
    }

    let dx = args.named.get("dx").map(extract_pt).unwrap_or(0.0);
    let dy = args.named.get("dy").map(extract_pt).unwrap_or(0.0);

    let stroke_color = args.named.get("stroke")
        .and_then(parse_color)
        .unwrap_or(Color::rgb(0, 0, 0)); // preto por omissão

    Ok(Value::Content(Content::Shape {
        kind:   ShapeKind::Line { dx, dy },
        width:  None,
        height: None,
        fill:   None,
        stroke: Some(Stroke { paint: stroke_color, thickness: 1.0 }),
    }))
}

/// Extrai um par de coordenadas (x, y) de um `Value::Array` com dois elementos numéricos.
fn extract_coordinate(val: &Value) -> Option<(f64, f64)> {
    match val {
        Value::Array(arr) if arr.len() == 2 => {
            let x = arr[0].cast_float()?;
            let y = arr[1].cast_float()?;
            Some((x, y))
        }
        _ => None,
    }
}

/// `polygon(pt1, pt2, ...; fill?, stroke?)` → `Content::Shape { kind: Path, ... }`.
///
/// Cada argumento posicional é um array `[x, y]` em pontos tipográficos.
/// A bounding box é calculada pelos pontos de controlo (DEBT-33).
pub fn native_polygon(_ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
    let mut path_items: Vec<PathItem> = Vec::new();
    let mut min_x = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for (i, val) in args.items.iter().enumerate() {
        let (x, y) = extract_coordinate(val)
            .ok_or_else(|| vec![SourceDiagnostic::error(
                Span::detached(),
                format!("polygon(): argumento {} não é uma coordenada válida", i),
            )])?;

        if i == 0 {
            path_items.push(PathItem::MoveTo(Point { x: Pt(x), y: Pt(y) }));
        } else {
            path_items.push(PathItem::LineTo(Point { x: Pt(x), y: Pt(y) }));
        }

        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }

    if path_items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "polygon() requer pelo menos um ponto".to_string(),
        )]);
    }

    path_items.push(PathItem::ClosePath);

    let fill   = args.named.get("fill").and_then(parse_color);
    let stroke = args.named.get("stroke").and_then(|v| {
        parse_color(v).map(|c| Stroke { paint: c, thickness: 1.0 })
    });

    let width  = if max_x > min_x { Some(Box::new(Value::Float(max_x - min_x))) } else { None };
    let height = if max_y > min_y { Some(Box::new(Value::Float(max_y - min_y))) } else { None };

    Ok(Value::Content(Content::Shape {
        kind: ShapeKind::Path(path_items),
        width,
        height,
        fill,
        stroke,
    }))
}

/// `move(dx?, dy?, body)` → `Content::Transform { matrix: translate(dx, dy), body }`.
pub fn native_move(_ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
    fn extract_pt(val: &Value) -> f64 {
        match val {
            Value::Float(f)  => *f,
            Value::Int(i)    => *i as f64,
            Value::Length(l) => l.abs.to_pt(),
            _ => 0.0,
        }
    }
    let dx = args.named.get("dx").map(extract_pt).unwrap_or(0.0);
    let dy = args.named.get("dy").map(extract_pt).unwrap_or(0.0);
    let body = args.items.iter()
        .find_map(|v| if let Value::Content(c) = v { Some(c.clone()) } else { None })
        .ok_or_else(|| vec![SourceDiagnostic::error(Span::detached(),
            "move() exige um corpo de conteúdo".to_string())])?;
    Ok(Value::Content(Content::Transform {
        matrix: TransformMatrix::translate(dx, dy),
        body:   Box::new(body),
    }))
}

/// `rotate(angle, body)` → `Content::Transform { matrix: rotate(rad), body }`.
///
/// `angle` pode ser `Value::Angle` (graus→radianos via `to_rad()`) ou `Value::Float` (radianos).
pub fn native_rotate(_ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
    let angle_rad = match args.named.get("angle") {
        Some(Value::Angle(a)) => a.to_rad(),
        Some(Value::Float(f)) => *f,
        _ => {
            // Fallback: primeiro arg posicional que seja Angle ou Float.
            match args.items.iter().find(|v| matches!(v, Value::Angle(_) | Value::Float(_))) {
                Some(Value::Angle(a)) => a.to_rad(),
                Some(Value::Float(f)) => *f,
                _ => 0.0,
            }
        }
    };
    let body = args.items.iter()
        .find_map(|v| if let Value::Content(c) = v { Some(c.clone()) } else { None })
        .ok_or_else(|| vec![SourceDiagnostic::error(Span::detached(),
            "rotate() exige um corpo de conteúdo".to_string())])?;
    Ok(Value::Content(Content::Transform {
        matrix: TransformMatrix::rotate(angle_rad),
        body:   Box::new(body),
    }))
}

/// `scale(x?, y?, body)` → `Content::Transform { matrix: scale(sx, sy), body }`.
///
/// `x` e `y` são factores de escala (Float ou Int). Se `y` omitido, escala uniforme.
pub fn native_scale(_ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
    fn extract_factor(val: &Value) -> f64 {
        match val {
            Value::Float(f) => *f,
            Value::Int(i)   => *i as f64,
            _               => 1.0,
        }
    }
    let sx = args.named.get("x").map(extract_factor).unwrap_or(1.0);
    let sy = args.named.get("y").map(extract_factor).unwrap_or(sx);
    let body = args.items.iter()
        .find_map(|v| if let Value::Content(c) = v { Some(c.clone()) } else { None })
        .ok_or_else(|| vec![SourceDiagnostic::error(Span::detached(),
            "scale() exige um corpo de conteúdo".to_string())])?;
    Ok(Value::Content(Content::Transform {
        matrix: TransformMatrix::scale(sx, sy),
        body:   Box::new(body),
    }))
}

// ── Align / Place (Passo 82) ────────────────────────────────────────────────

/// `align(alignment, body)` → `Content::Align`.
///
/// `alignment` é uma string como `"center"` ou `"top-right"` (DEBT-36).
/// `body` é o primeiro argumento posicional do tipo Content.
pub fn native_align(_ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
    expect_no_named(&args.named)?;

    let position_str = args.items.iter()
        .find_map(|v| if let Value::Str(s) = v { Some(s.to_string()) } else { None })
        .unwrap_or_else(|| "left".to_string());

    let body = args.items.iter()
        .find_map(|v| if let Value::Content(c) = v { Some(c.clone()) } else { None })
        .ok_or_else(|| vec![SourceDiagnostic::error(Span::detached(),
            "align() exige um bloco de conteúdo".to_string())])?;

    Ok(Value::Content(Content::Align {
        alignment: Align2D::from_string(&position_str),
        body:      Box::new(body),
    }))
}

/// `place(alignment, dx?, dy?, body)` → `Content::Place`.
///
/// `dx`/`dy` em pt deslocam o conteúdo a partir da posição alinhada.
pub fn native_place(_ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
    for key in args.named.keys() {
        if !["dx", "dy"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("argumento nomeado inesperado em place(): '{}'", key),
            )]);
        }
    }

    fn extract_pt(val: &Value) -> f64 {
        match val {
            Value::Float(f)  => *f,
            Value::Int(i)    => *i as f64,
            Value::Length(l) => l.abs.to_pt(),
            _ => 0.0,
        }
    }

    let position_str = args.items.iter()
        .find_map(|v| if let Value::Str(s) = v { Some(s.to_string()) } else { None })
        .unwrap_or_else(|| "top-left".to_string());

    let dx = args.named.get("dx").map(extract_pt).unwrap_or(0.0);
    let dy = args.named.get("dy").map(extract_pt).unwrap_or(0.0);

    let body = args.items.iter()
        .find_map(|v| if let Value::Content(c) = v { Some(c.clone()) } else { None })
        .ok_or_else(|| vec![SourceDiagnostic::error(Span::detached(),
            "place() exige um bloco de conteúdo".to_string())])?;

    Ok(Value::Content(Content::Place {
        alignment: Align2D::from_string(&position_str),
        dx,
        dy,
        body: Box::new(body),
    }))
}

// ── Grid (Passo 80) ────────────────────────────────────────────────────────

fn parse_track_sizing(val: &Value) -> Option<TrackSizing> {
    match val {
        Value::Float(f)    => Some(TrackSizing::Fixed(*f)),
        Value::Length(l)   => Some(TrackSizing::Fixed(l.abs.to_pt())),
        Value::Fraction(fr) => Some(TrackSizing::Fraction(*fr)),
        Value::Auto        => Some(TrackSizing::Auto),
        Value::Str(s) if s.as_str() == "auto" => Some(TrackSizing::Auto),
        _ => None,
    }
}

fn extract_tracks(val: Option<&Value>) -> Vec<TrackSizing> {
    match val {
        Some(Value::Array(arr)) => arr.iter().filter_map(parse_track_sizing).collect(),
        Some(v) => parse_track_sizing(v).into_iter().collect(),
        None    => vec![],
    }
}

/// `grid(columns?, rows?, ...cells)` → `Content::Grid`.
pub fn native_grid(_ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
    for key in args.named.keys() {
        if !["columns", "rows"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("argumento nomeado inesperado em grid(): '{}'", key),
            )]);
        }
    }
    let columns = extract_tracks(args.named.get("columns"));
    let rows    = extract_tracks(args.named.get("rows"));
    let cells: Vec<Content> = args.items.iter()
        .filter_map(|v| if let Value::Content(c) = v { Some(c.clone()) } else { None })
        .collect();
    Ok(Value::Content(Content::Grid { columns, rows, cells }))
}

/// `#set page(width: w, height: h, margin: m)` — configura as dimensões da página (Passo 81).
pub fn native_page(_ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value> {
    fn extract_pt(val: &Value) -> Option<f64> {
        match val {
            Value::Length(l) => Some(l.abs.to_pt()),
            Value::Float(f)  => Some(*f),
            Value::Int(i)    => Some(*i as f64),
            _                => None,
        }
    }

    for key in args.named.keys() {
        if !["width", "height", "margin"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("argumento nomeado inesperado em page(): '{}'", key),
            )]);
        }
    }

    let width  = args.named.get("width") .and_then(extract_pt);
    let height = args.named.get("height").and_then(extract_pt);
    let margin = args.named.get("margin").and_then(extract_pt);

    Ok(Value::Content(Content::SetPage { width, height, margin }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::world::World;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::source::Source;
    use crate::entities::world_types::{Bytes, Datetime, FileError, FileResult, Font, Library};
    use std::num::NonZeroU16;

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

    /// Mundo nulo para testes de stdlib que não precisam de I/O.
    #[derive(Default)]
    struct NullWorld {
        library: Library,
        book:    FontBook,
        files:   std::collections::HashMap<String, std::sync::Arc<Vec<u8>>>,
    }
    impl World for NullWorld {
        fn library(&self) -> &Library { &self.library }
        fn book(&self) -> &FontBook { &self.book }
        fn main(&self) -> FileId { FileId::from_raw(NonZeroU16::new(1).unwrap()) }
        fn source(&self, _: FileId) -> FileResult<Source> { Err(FileError::NotFound) }
        fn file(&self, _: FileId) -> FileResult<Bytes> { Err(FileError::NotFound) }
        fn font(&self, _: usize) -> Option<Font> { None }
        fn today(&self, _: Option<i64>) -> Option<Datetime> { None }
        fn read_bytes(&self, _current_file: FileId, path: &str) -> Result<std::sync::Arc<Vec<u8>>, String> {
            self.files.get(path)
                .map(std::sync::Arc::clone)
                .ok_or_else(|| format!("ficheiro não encontrado: {}", path))
        }
    }

    /// Helper que cria um EvalContext nulo para tests que não usam o ctx.
    macro_rules! null_ctx {
        ($ctx:ident) => {
            let _world = NullWorld::default();
            let _dummy_id = crate::entities::file_id::FileId::from_raw(
                std::num::NonZeroU16::new(1).unwrap()
            );
            let mut $ctx = EvalContext::new(&_world, _dummy_id);
        }
    }

    #[test]
    fn native_type_directo() {
        null_ctx!(ctx);
        assert_eq!(native_type(&mut ctx, &p(vec![Value::Int(1)])).unwrap(),     Value::Str("int".into()));
        assert_eq!(native_type(&mut ctx, &p(vec![Value::Bool(true)])).unwrap(), Value::Str("bool".into()));
        assert_eq!(native_type(&mut ctx, &p(vec![Value::None])).unwrap(),       Value::Str("none".into()));
        assert!(native_type(&mut ctx, &p(vec![])).is_err());
        assert!(native_type(&mut ctx, &p(vec![Value::Int(1), Value::Int(2)])).is_err());
    }

    #[test]
    fn native_type_named_arg_retorna_err() {
        null_ctx!(ctx);
        let args = pn(vec![Value::Int(1)], "extra", Value::Bool(true));
        assert!(native_type(&mut ctx, &args).is_err(), "named arg inesperado deve retornar Err");
    }

    #[test]
    fn native_len_directo() {
        null_ctx!(ctx);
        assert_eq!(native_len(&mut ctx, &p(vec![Value::Str("abc".into())])).unwrap(), Value::Int(3));
        assert_eq!(
            native_len(&mut ctx, &p(vec![Value::Array(vec![Value::Int(1), Value::Int(2)])])).unwrap(),
            Value::Int(2)
        );
        assert!(native_len(&mut ctx, &p(vec![Value::Int(1)])).is_err());
        assert!(native_len(&mut ctx, &p(vec![])).is_err());
    }

    // ── Passo 25 — rgb/luma ──────────────────────────────────────────────────

    #[test]
    fn stdlib_rgb_tres_args() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Color;
        let r = native_rgb(&mut ctx, &p(vec![Value::Int(255), Value::Int(0), Value::Int(128)])).unwrap();
        assert_eq!(r, Value::Color(Color::rgb(255, 0, 128)));
    }

    #[test]
    fn stdlib_rgb_quatro_args() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Color;
        let r = native_rgb(&mut ctx, &p(vec![Value::Int(255), Value::Int(0), Value::Int(0), Value::Int(200)])).unwrap();
        assert_eq!(r, Value::Color(Color::rgba(255, 0, 0, 200)));
    }

    #[test]
    fn stdlib_rgb_out_of_range() {
        null_ctx!(ctx);
        assert!(native_rgb(&mut ctx, &p(vec![Value::Int(300), Value::Int(0), Value::Int(0)])).is_err());
    }

    #[test]
    fn stdlib_luma() {
        null_ctx!(ctx);
        use crate::entities::layout_types::Color;
        let r = native_luma(&mut ctx, &p(vec![Value::Int(128)])).unwrap();
        assert_eq!(r, Value::Color(Color::rgb(128, 128, 128)));
    }

    // ── Passo 27 — str/int/float ─────────────────────────────────────────────

    #[test]
    fn native_str_de_int() {
        null_ctx!(ctx);
        assert_eq!(native_str(&mut ctx, &p(vec![Value::Int(42)])).unwrap(), Value::Str("42".into()));
    }

    #[test]
    #[allow(clippy::approx_constant)] // 3.14 é valor literal de teste, não aproximação de PI
    fn native_str_de_float() {
        null_ctx!(ctx);
        assert_eq!(native_str(&mut ctx, &p(vec![Value::Float(3.14)])).unwrap(), Value::Str("3.14".into()));
    }

    #[test]
    fn native_str_de_bool() {
        null_ctx!(ctx);
        assert_eq!(native_str(&mut ctx, &p(vec![Value::Bool(true)])).unwrap(),  Value::Str("true".into()));
        assert_eq!(native_str(&mut ctx, &p(vec![Value::Bool(false)])).unwrap(), Value::Str("false".into()));
    }

    #[test]
    fn native_str_identity() {
        null_ctx!(ctx);
        assert_eq!(native_str(&mut ctx, &p(vec![Value::Str("hello".into())])).unwrap(), Value::Str("hello".into()));
    }

    #[test]
    fn native_str_de_none() {
        null_ctx!(ctx);
        assert_eq!(native_str(&mut ctx, &p(vec![Value::None])).unwrap(), Value::Str("none".into()));
    }

    #[test]
    fn native_int_de_int() {
        null_ctx!(ctx);
        assert_eq!(native_int(&mut ctx, &p(vec![Value::Int(42)])).unwrap(), Value::Int(42));
    }

    #[test]
    fn native_int_de_str() {
        null_ctx!(ctx);
        assert_eq!(native_int(&mut ctx, &p(vec![Value::Str("42".into())])).unwrap(), Value::Int(42));
        assert!(native_int(&mut ctx, &p(vec![Value::Str("abc".into())])).is_err());
    }

    #[test]
    fn native_int_de_bool() {
        null_ctx!(ctx);
        assert_eq!(native_int(&mut ctx, &p(vec![Value::Bool(true)])).unwrap(),  Value::Int(1));
        assert_eq!(native_int(&mut ctx, &p(vec![Value::Bool(false)])).unwrap(), Value::Int(0));
    }

    #[test]
    fn native_int_float_retorna_err() {
        null_ctx!(ctx);
        assert!(native_int(&mut ctx, &p(vec![Value::Float(3.7)])).is_err());
    }

    #[test]
    fn native_float_de_int() {
        null_ctx!(ctx);
        assert_eq!(native_float(&mut ctx, &p(vec![Value::Int(3)])).unwrap(), Value::Float(3.0));
    }

    #[test]
    #[allow(clippy::approx_constant)] // 3.14 é valor literal de teste, não aproximação de PI
    fn native_float_de_str() {
        null_ctx!(ctx);
        assert_eq!(native_float(&mut ctx, &p(vec![Value::Str("3.14".into())])).unwrap(), Value::Float(3.14));
        assert!(native_float(&mut ctx, &p(vec![Value::Str("abc".into())])).is_err());
    }

    // ── Passo 27 — calc ──────────────────────────────────────────────────────

    #[test]
    fn calc_abs_int() {
        null_ctx!(ctx);
        assert_eq!(calc_abs(&mut ctx, &p(vec![Value::Int(-5)])).unwrap(), Value::Int(5));
        assert_eq!(calc_abs(&mut ctx, &p(vec![Value::Int(5)])).unwrap(),  Value::Int(5));
        assert_eq!(calc_abs(&mut ctx, &p(vec![Value::Int(0)])).unwrap(),  Value::Int(0));
    }

    #[test]
    #[allow(clippy::approx_constant)] // 3.14 é valor literal de teste, não aproximação de PI
    fn calc_abs_float() {
        null_ctx!(ctx);
        assert_eq!(calc_abs(&mut ctx, &p(vec![Value::Float(-3.14)])).unwrap(), Value::Float(3.14));
    }

    #[test]
    fn calc_pow_int() {
        null_ctx!(ctx);
        assert_eq!(calc_pow(&mut ctx, &p(vec![Value::Int(2), Value::Int(10)])).unwrap(), Value::Int(1024));
        assert_eq!(calc_pow(&mut ctx, &p(vec![Value::Int(2), Value::Int(0)])).unwrap(),  Value::Int(1));
    }

    #[test]
    fn calc_pow_float() {
        null_ctx!(ctx);
        let r = calc_pow(&mut ctx, &p(vec![Value::Float(2.0), Value::Float(0.5)])).unwrap();
        assert!(matches!(r, Value::Float(f) if (f - std::f64::consts::SQRT_2).abs() < 1e-10));
    }

    #[test]
    fn calc_pow_negativo_retorna_err() {
        null_ctx!(ctx);
        assert!(calc_pow(&mut ctx, &p(vec![Value::Int(2), Value::Int(-1)])).is_err());
    }

    #[test]
    fn calc_sqrt_positivo() {
        null_ctx!(ctx);
        assert_eq!(calc_sqrt(&mut ctx, &p(vec![Value::Float(4.0)])).unwrap(), Value::Float(2.0));
        assert_eq!(calc_sqrt(&mut ctx, &p(vec![Value::Int(4)])).unwrap(),     Value::Float(2.0));
    }

    #[test]
    fn calc_sqrt_negativo_retorna_err() {
        null_ctx!(ctx);
        assert!(calc_sqrt(&mut ctx, &p(vec![Value::Float(-1.0)])).is_err());
    }

    #[test]
    fn calc_floor_ceil_round() {
        null_ctx!(ctx);
        assert_eq!(calc_floor(&mut ctx, &p(vec![Value::Float(3.7)])).unwrap(), Value::Int(3));
        assert_eq!(calc_ceil(&mut ctx, &p(vec![Value::Float(3.2)])).unwrap(),  Value::Int(4));
        assert_eq!(calc_round(&mut ctx, &p(vec![Value::Float(3.5)])).unwrap(), Value::Int(4));
        assert_eq!(calc_round(&mut ctx, &p(vec![Value::Float(3.4)])).unwrap(), Value::Int(3));
    }

    #[test]
    fn calc_min_max_int() {
        null_ctx!(ctx);
        assert_eq!(calc_min(&mut ctx, &p(vec![Value::Int(3), Value::Int(1), Value::Int(2)])).unwrap(), Value::Int(1));
        assert_eq!(calc_max(&mut ctx, &p(vec![Value::Int(3), Value::Int(1), Value::Int(2)])).unwrap(), Value::Int(3));
    }

    #[test]
    fn calc_min_vazio_retorna_err() {
        null_ctx!(ctx);
        assert!(calc_min(&mut ctx, &p(vec![])).is_err());
        assert!(calc_max(&mut ctx, &p(vec![])).is_err());
    }

    #[test]
    fn calc_clamp_int() {
        null_ctx!(ctx);
        assert_eq!(calc_clamp(&mut ctx, &p(vec![Value::Int(5),  Value::Int(0), Value::Int(10)])).unwrap(), Value::Int(5));
        assert_eq!(calc_clamp(&mut ctx, &p(vec![Value::Int(-5), Value::Int(0), Value::Int(10)])).unwrap(), Value::Int(0));
        assert_eq!(calc_clamp(&mut ctx, &p(vec![Value::Int(15), Value::Int(0), Value::Int(10)])).unwrap(), Value::Int(10));
    }

    #[test]
    fn calc_clamp_min_maior_max_retorna_err() {
        null_ctx!(ctx);
        assert!(calc_clamp(&mut ctx, &p(vec![Value::Float(5.0), Value::Float(10.0), Value::Float(0.0)])).is_err());
    }

    #[test]
    fn native_range_directo() {
        null_ctx!(ctx);
        assert_eq!(native_range(&mut ctx, &p(vec![Value::Int(3)])).unwrap(),
                   Value::Array(vec![Value::Int(0), Value::Int(1), Value::Int(2)]));
        assert_eq!(native_range(&mut ctx, &p(vec![Value::Int(2), Value::Int(5)])).unwrap(),
                   Value::Array(vec![Value::Int(2), Value::Int(3), Value::Int(4)]));
        assert_eq!(native_range(&mut ctx, &p(vec![Value::Int(3), Value::Int(3)])).unwrap(),
                   Value::Array(vec![]));
        assert!(native_range(&mut ctx, &p(vec![Value::Int(-1)])).is_err());
    }

    // ── Passo 64 — native_figure (DEBT-16) ──────────────────────────────────

    #[test]
    fn native_figure_com_body_e_caption() {
        null_ctx!(ctx);
        use crate::entities::content::Content;
        let body_content = Content::text("Gráfico");
        let caption_content = Content::text("Legenda");
        let args = pn(
            vec![Value::Content(body_content)],
            "caption",
            Value::Content(caption_content),
        );
        let result = native_figure(&mut ctx, &args).unwrap();
        assert!(matches!(result, Value::Content(Content::Figure { caption: Some(_), .. })),
            "figure com caption deve ter Some(caption): {:?}", result);
    }

    #[test]
    fn native_figure_sem_caption() {
        null_ctx!(ctx);
        use crate::entities::content::Content;
        let body_content = Content::text("Diagrama");
        let args = p(vec![Value::Content(body_content)]);
        let result = native_figure(&mut ctx, &args).unwrap();
        assert!(matches!(result, Value::Content(Content::Figure { caption: None, .. })),
            "figure sem caption deve ter None: {:?}", result);
    }

    #[test]
    fn native_figure_caption_none_value() {
        null_ctx!(ctx);
        use crate::entities::content::Content;
        // caption: none → ausência de legenda
        let body_content = Content::text("Corpo");
        let args = pn(vec![Value::Content(body_content)], "caption", Value::None);
        let result = native_figure(&mut ctx, &args).unwrap();
        assert!(matches!(result, Value::Content(Content::Figure { caption: None, .. })),
            "figure com caption: none deve ter caption None");
    }

    #[test]
    fn native_figure_sem_body_retorna_err() {
        null_ctx!(ctx);
        let args = p(vec![]);
        assert!(native_figure(&mut ctx, &args).is_err(), "figure sem body deve retornar Err");
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

    // ── Passo 66 — native_assert (prova de fogo de named args) ───────────────

    #[test]
    fn native_assert_true_nao_gera_erro() {
        null_ctx!(ctx);
        let args = p(vec![Value::Bool(true)]);
        assert!(native_assert(&mut ctx, &args).is_ok(), "assert(true) deve ter sucesso");
    }

    #[test]
    fn native_assert_false_gera_erro_com_mensagem_padrao() {
        null_ctx!(ctx);
        let args = p(vec![Value::Bool(false)]);
        let result = native_assert(&mut ctx, &args);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err[0].message.contains("falhou") || err[0].message.contains("Asser"),
            "mensagem de erro padrão deve mencionar a asserção: {:?}", err[0].message
        );
    }

    #[test]
    fn native_assert_false_gera_erro_com_mensagem_personalizada() {
        null_ctx!(ctx);
        // Mensagem sem acentos para evitar problemas de codificação em CI.
        let args = pn(vec![Value::Bool(false)], "message", Value::Str("Matematica falhou".into()));
        let result = native_assert(&mut ctx, &args);
        assert!(result.is_err());
        assert!(result.unwrap_err()[0].message.contains("Matematica falhou"));
    }

    #[test]
    fn native_assert_rejeita_named_arg_invalido() {
        null_ctx!(ctx);
        let args = pn(vec![Value::Bool(true)], "bla", Value::Str("bla".into()));
        let result = native_assert(&mut ctx, &args);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err[0].message.contains("inesperado") && err[0].message.contains("bla"),
            "named arg desconhecido deve gerar erro: {:?}", err[0].message
        );
    }

    // ── Passo 71 — native_image ───────────────────────────────────────────────

    #[test]
    fn native_image_retorna_content_image() {
        let mut world = NullWorld::default();
        world.files.insert("foto.png".to_string(), std::sync::Arc::new(vec![1, 2, 3]));
        let dummy_id = crate::entities::file_id::FileId::from_raw(std::num::NonZeroU16::new(1).unwrap());
        let mut ctx = EvalContext::new(&world, dummy_id);
        let args = p(vec![Value::Str("foto.png".into())]);
        let result = native_image(&mut ctx, &args).unwrap();
        assert!(matches!(result, Value::Content(Content::Image { .. })));
    }

    #[test]
    fn native_image_ficheiro_inexistente_gera_erro() {
        null_ctx!(ctx);
        let args = p(vec![Value::Str("naoexiste.png".into())]);
        assert!(native_image(&mut ctx, &args).is_err());
    }

    #[test]
    fn native_image_rejeita_named_arg_invalido() {
        null_ctx!(ctx);
        let args = pn(vec![Value::Str("foto.png".into())], "cor", Value::Str("red".into()));
        assert!(native_image(&mut ctx, &args).is_err());
    }

    // ── Passo 76 — primitivas geométricas ────────────────────────────────────

    #[test]
    fn rect_sem_cores_tem_stroke_preta_1pt() {
        // #rect() sem fill nem stroke → stroke preta de 1pt.
        // Confirma que a stdlib é o único local onde este fallback existe.
        null_ctx!(ctx);
        let result = native_rect(&mut ctx, &p(vec![])).unwrap();
        if let Value::Content(Content::Shape { fill, stroke, .. }) = result {
            assert!(fill.is_none(), "rect sem fill deve ter fill: None");
            let s = stroke.expect("rect sem cores deve ter stroke de fallback");
            assert_eq!(s.paint, Color::rgb(0, 0, 0), "stroke de fallback deve ser preta");
            assert_eq!(s.thickness, 1.0, "espessura de fallback deve ser 1pt");
        } else {
            panic!("Esperado Content::Shape");
        }
    }

    #[test]
    fn rect_com_fill_nao_tem_stroke_fallback() {
        null_ctx!(ctx);
        let mut args = Args::positional(vec![]);
        args.named.insert("fill".into(), Value::Str("red".into()));
        let result = native_rect(&mut ctx, &args).unwrap();
        if let Value::Content(Content::Shape { fill, stroke, .. }) = result {
            assert!(fill.is_some(), "fill red deve estar presente");
            assert!(stroke.is_none(), "sem stroke explícito e com fill → stroke deve ser None");
        } else {
            panic!("Esperado Content::Shape");
        }
    }

    #[test]
    fn line_tem_kind_line_e_stroke_preta_por_omissao() {
        use crate::entities::geometry::ShapeKind;
        null_ctx!(ctx);
        let mut args = Args::positional(vec![]);
        args.named.insert("dx".into(), Value::Float(100.0));
        args.named.insert("dy".into(), Value::Float(50.0));
        let result = native_line(&mut ctx, &args).unwrap();
        if let Value::Content(Content::Shape { kind, fill, stroke, .. }) = result {
            assert!(matches!(kind, ShapeKind::Line { dx, dy } if dx == 100.0 && dy == 50.0));
            assert!(fill.is_none(), "linha não tem fill");
            assert!(stroke.is_some(), "linha tem stroke por omissão");
        } else {
            panic!("Esperado Content::Shape");
        }
    }

    #[test]
    fn polygon_sem_pontos_gera_erro() {
        null_ctx!(ctx);
        let args = Args::positional(vec![]);
        let result = native_polygon(&mut ctx, &args);
        assert!(result.is_err(), "polygon() sem pontos deve retornar Err");
        let msg = result.unwrap_err();
        let msg_str = format!("{:?}", msg);
        assert!(msg_str.contains("pelo menos um ponto"),
            "Mensagem de erro deve mencionar 'pelo menos um ponto', obteve: {}", msg_str);
    }

    #[test]
    fn polygon_com_um_ponto_gera_moveto_e_closepath() {
        use crate::entities::geometry::{PathItem, ShapeKind};
        null_ctx!(ctx);
        let args = Args::positional(vec![
            Value::Array(vec![Value::Float(10.0), Value::Float(20.0)]),
        ]);
        let result = native_polygon(&mut ctx, &args).unwrap();
        if let Value::Content(Content::Shape { kind: ShapeKind::Path(items), .. }) = result {
            assert_eq!(items.len(), 2, "Um ponto deve gerar MoveTo + ClosePath");
            assert!(matches!(items[0], PathItem::MoveTo(_)), "Primeiro item deve ser MoveTo");
            assert!(matches!(items[1], PathItem::ClosePath), "Último item deve ser ClosePath");
        } else {
            panic!("Esperado Content::Shape com ShapeKind::Path");
        }
    }

    #[test]
    fn polygon_triangulo_gera_moveto_lineto_lineto_closepath() {
        use crate::entities::geometry::{PathItem, ShapeKind};
        null_ctx!(ctx);
        let args = Args::positional(vec![
            Value::Array(vec![Value::Float(0.0),  Value::Float(0.0)]),
            Value::Array(vec![Value::Float(50.0), Value::Float(0.0)]),
            Value::Array(vec![Value::Float(25.0), Value::Float(50.0)]),
        ]);
        let result = native_polygon(&mut ctx, &args).unwrap();
        if let Value::Content(Content::Shape { kind: ShapeKind::Path(items), .. }) = result {
            assert_eq!(items.len(), 4); // MoveTo + 2×LineTo + ClosePath
            assert!(matches!(items[0], PathItem::MoveTo(_)));
            assert!(matches!(items[1], PathItem::LineTo(_)));
            assert!(matches!(items[2], PathItem::LineTo(_)));
            assert!(matches!(items[3], PathItem::ClosePath));
        } else {
            panic!("Esperado Content::Shape com ShapeKind::Path");
        }
    }

    #[test]
    fn parse_color_nomes_conhecidos() {
        assert_eq!(parse_color(&Value::Str("red".into())),   Some(Color::rgb(255, 0, 0)));
        assert_eq!(parse_color(&Value::Str("green".into())), Some(Color::rgb(0, 128, 0)));
        assert_eq!(parse_color(&Value::Str("blue".into())),  Some(Color::rgb(0, 0, 255)));
        assert_eq!(parse_color(&Value::Str("black".into())), Some(Color::rgb(0, 0, 0)));
        assert_eq!(parse_color(&Value::Str("white".into())), Some(Color::rgb(255, 255, 255)));
        assert_eq!(parse_color(&Value::Str("purple".into())), None);
        assert_eq!(parse_color(&Value::Int(42)), None);
    }
}
