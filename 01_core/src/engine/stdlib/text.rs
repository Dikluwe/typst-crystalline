//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/stdlib/text.md
//! @prompt-hash d55f9c11
//! @layer L1
//! @updated 2026-06-22
//!
//! Funções nativas de texto (upper, lower, replace, lorem, smallcaps).
//! Extraído de `stdlib.rs` no Passo 96.5 conforme ADR-0037.

use std::str::FromStr;

use super::{err, expect_no_named};
use crate::entities::file_id::FileId;

use crate::engine::eval::rules::{
    edge_cast_error, expected_length_error, type_mismatch, VANILLA_TEXT_SET_PROPS,
};
use crate::engine::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::font_book::FontWeight;
use crate::entities::font_list::{FontFamily, FontList};
use crate::entities::lang::Lang;
use crate::entities::layout_types::{Color, Length, Pt};
use crate::entities::regex::Regex;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::style::{Style, Styles};
use crate::entities::value::Value;

use super::shapes::parse_color;

// ── P492 — constructor `text(...)` ──────────────────────────────────────────

/// `text(...)` — constructor de elemento de texto (parcial).
///
/// P492: suporta `text(red, it)` (fill posicional inferido por tipo) e
/// `text(fill: red, it)`. Aceita body como `Content` ou `Str`.
pub fn native_text(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    // Determinar fill: named arg prevalece; senão, primeiro posicional Color.
    let mut fill: Option<Color> = None;
    let mut body_idx = 0usize;

    if let Some(v) = args.named.get("fill") {
        fill = match v {
            Value::None => None,
            Value::Color(c) => Some(*c),
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("text(fill:) espera color, recebeu {}", other.type_name()),
                )])
            }
        };
    }

    // Se o primeiro posicional for Color e fill ainda não definido, usá-lo como fill.
    if fill.is_none() {
        if let Some(Value::Color(c)) = args.items.first() {
            fill = Some(*c);
            body_idx = 1;
        }
    }

    let body = match args.items.get(body_idx) {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "text() espera content ou string como body, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "text() requer body como argumento".to_string(),
            )])
        }
    };

    // Validar que não há argumentos posicionais além do fill + body.
    if args.items.len() > body_idx + 1 {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!(
                "text() recebeu {} argumentos posicionais (espera 1-2)",
                args.items.len()
            ),
        )]);
    }

    let mut styles = Styles::new();
    if let Some(c) = fill {
        styles.push(Style::Fill(c));
    }

    // P865 — argumentos nomeados válidos em `#set text(...)` também são
    // válidos em `#text(...)`. A validação replica a de `eval_set_rule`
    // (`engine/eval/rules.rs`); os valores transportam-se pelo mesmo mecanismo
    // (`Style` tipado ou canal custom `"text.<campo>"`).
    for (key, value) in args.named.iter() {
        let key_str = key.as_str();
        if key_str == "fill" {
            // Já processado acima; `fill: none` remove o fill previamente definido.
            continue;
        }
        if key_str == "variations" {
            crate::entities::font_variations::FontVariations::from_value(value, args.span)?;
            styles = styles.push_custom("text.variations", value.clone());
            continue;
        }
        match key_str {
            "size" => {
                let length = require_length(value, args.span)?;
                styles.push(Style::Size(Pt(length.abs.to_pt())));
            }
            "weight" => {
                let w = parse_text_weight(value, args.span)?;
                styles.push(Style::Weight(w));
            }
            "style" => {
                let s = parse_text_style(value, args.span)?;
                styles = styles.push_custom("text.style", Value::Str(s));
            }
            "tracking" => {
                let length = require_length(value, args.span)?;
                styles.push(Style::Tracking(length));
            }
            "lang" => {
                let lang = parse_text_lang(value, args.span)?;
                styles.push(Style::Lang(lang));
            }
            "font" => {
                let font_list = parse_text_font(value, args.span)?;
                styles.push(Style::Font(font_list));
            }
            "top-edge" => {
                let v = parse_text_edge(value, args.span, true)?;
                styles = styles.push_custom("text.top-edge", v);
            }
            "bottom-edge" => {
                let v = parse_text_edge(value, args.span, false)?;
                styles = styles.push_custom("text.bottom-edge", v);
            }
            "dir" => {
                if let Value::Dir(dir) = value {
                    if dir.is_vertical() {
                        return Err(vec![SourceDiagnostic::error(
                            args.span,
                            "text direction must be horizontal".to_string(),
                        )]);
                    }
                    styles = styles.push_custom("text.dir", Value::Dir(*dir));
                }
            }
            "bold" | "italic" => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("unexpected argument: {}", key_str),
                )]);
            }
            _ => {
                // P816: nome fora da lista de propriedades settable do
                // `TextElem` vanilla → erro hard. Nomes dentro da lista mas
                // ainda não capturados são aceites (scope-out) sem efeito.
                if !VANILLA_TEXT_SET_PROPS.contains(&key_str) {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        format!("unexpected argument: {}", key_str),
                    )]);
                }
            }
        }
    }

    let styled = if styles.is_empty() {
        body
    } else {
        Content::Styled(Box::new(body), styles)
    };
    Ok(Value::Content(styled))
}

/// **P865** — extrai um `Length` para `size:` / `tracking:`.
fn require_length(value: &Value, span: Span) -> SourceResult<Length> {
    match value {
        Value::Length(l) => Ok(*l),
        other => Err(vec![expected_length_error(other, span)]),
    }
}

/// **P865** — valida `weight:` como `Int` (raw 0-1000) ou nome simbólico.
fn parse_text_weight(value: &Value, span: Span) -> SourceResult<u16> {
    match value {
        Value::Int(n) => {
            if let Ok(w) = u16::try_from(*n) {
                Ok(w)
            } else {
                Err(vec![SourceDiagnostic::error(
                    span,
                    "font weight must be between 100 and 900".to_string(),
                )])
            }
        }
        Value::Str(s) => match FontWeight::from_name(s.as_str()) {
            Some(fw) => Ok(fw.to_number()),
            None => Err(vec![SourceDiagnostic::error(
                span,
                format!("unknown font weight name: {s}"),
            )]),
        },
        other => Err(vec![type_mismatch("int or string", other, span)]),
    }
}

/// **P865** — valida `style:` como uma das strings canónicas do vanilla.
fn parse_text_style(value: &Value, span: Span) -> SourceResult<ecow::EcoString> {
    if let Value::Str(s) = value {
        match s.as_str() {
            "normal" | "italic" | "oblique" => Ok(s.clone()),
            _ => Err(vec![SourceDiagnostic::error(
                span,
                format!("unknown font style name: {s}"),
            )]),
        }
    } else {
        Err(vec![type_mismatch("string", value, span)])
    }
}

/// **P865** — valida `lang:` como código BCP-47.
fn parse_text_lang(value: &Value, span: Span) -> SourceResult<Lang> {
    if let Value::Str(s) = value {
        match Lang::from_str(s.as_str()) {
            Ok(lang) => Ok(lang),
            Err(msg) => Err(vec![SourceDiagnostic::error(span, msg.to_string())]),
        }
    } else {
        Err(vec![type_mismatch("string", value, span)])
    }
}

/// **P865** — converte o valor de `font:` para `FontList`.
/// Aceita string, array de strings, ou dict (forma nomeada ou legada).
fn parse_text_font(value: &Value, span: Span) -> SourceResult<FontList> {
    match value {
        Value::Str(s) => Ok(FontList::single(s.clone())),
        Value::Array(arr) => {
            if arr.is_empty() {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    "font fallback list must not be empty".to_string(),
                )]);
            }
            let mut families = Vec::with_capacity(arr.len());
            for item in arr {
                if let Value::Str(s) = item {
                    families.push(FontFamily::new(s.clone()));
                } else {
                    return Err(vec![SourceDiagnostic::error(
                        span,
                        "font array must contain only strings".to_string(),
                    )]);
                }
            }
            FontList::new(families).ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    span,
                    "font fallback list must not be empty".to_string(),
                )]
            })
        }
        Value::Dict(d) => {
            const NAMED_FIELDS: &[&str] = &["family", "variant", "weight", "style"];
            let is_named = d.keys().any(|k| NAMED_FIELDS.contains(&k.as_str()));
            if is_named {
                let family = d
                    .get("family")
                    .ok_or_else(|| {
                        vec![SourceDiagnostic::error(
                            span,
                            "font dict must contain a family".to_string(),
                        )]
                    })?
                    .clone();
                let name_pattern = match family {
                    Value::Str(s) => crate::entities::font_list::FontNamePattern::Literal(s),
                    _ => {
                        return Err(vec![SourceDiagnostic::error(
                            span,
                            "font family must be a string".to_string(),
                        )])
                    }
                };
                let variant = match d.get("variant") {
                    Some(Value::Str(s)) => Some(s.clone()),
                    Some(_) => {
                        return Err(vec![SourceDiagnostic::error(
                            span,
                            "font dict field 'variant' expects string".to_string(),
                        )])
                    }
                    None => None,
                };
                let weight = d.get("weight")
                    .map(|v| match v {
                        Value::Int(i) => Ok(ecow::EcoString::from(i.to_string())),
                        Value::Str(s) => Ok(s.clone()),
                        _ => Err(vec![SourceDiagnostic::error(
                            span,
                            "font weight must be an integer or string".to_string(),
                        )]),
                    })
                    .transpose()?;
                let style = match d.get("style") {
                    Some(Value::Str(s)) => Some(s.clone()),
                    Some(_) => {
                        return Err(vec![SourceDiagnostic::error(
                            span,
                            "font dict field 'style' expects string".to_string(),
                        )])
                    }
                    None => None,
                };
                let family = FontFamily::new_named(
                    name_pattern,
                    Vec::new(),
                    variant,
                    weight,
                    style,
                );
                Ok(FontList::new(vec![family]).expect("lista non-empty"))
            } else {
                // Forma legada: chaves são nomes de família, valores são
                // variantes (string ou array de strings).
                let mut families = Vec::with_capacity(d.len());
                for (name, variants_val) in d.iter() {
                    let variants = match variants_val {
                        Value::Str(s) => vec![s.clone()],
                        Value::Array(a) => {
                            let mut out = Vec::with_capacity(a.len());
                            for item in a {
                                if let Value::Str(s) = item {
                                    out.push(s.clone());
                                } else {
                                    return Err(vec![SourceDiagnostic::error(
                                        span,
                                        "font variants must be strings".to_string(),
                                    )]);
                                }
                            }
                            out
                        }
                        _ => {
                            return Err(vec![SourceDiagnostic::error(
                                span,
                                "font variants must be a string or array".to_string(),
                            )])
                        }
                    };
                    families.push(FontFamily::new_literal(name.clone(), variants));
                }
                if families.is_empty() {
                    return Err(vec![SourceDiagnostic::error(
                        span,
                        "font fallback list must not be empty".to_string(),
                    )]);
                }
                Ok(FontList::new(families).expect("lista non-empty"))
            }
        }
        _ => Err(vec![SourceDiagnostic::error(
            span,
            "font expects a string, array of strings, or dict".to_string(),
        )]),
    }
}

/// **P865** — valida `top-edge:` / `bottom-edge:` (métrica nomeada ou Length).
fn parse_text_edge(value: &Value, span: Span, is_top: bool) -> SourceResult<Value> {
    match value {
        Value::Str(s) => {
            let valid = if is_top {
                &["ascender", "cap-height", "x-height", "baseline", "bounds"][..]
            } else {
                &["baseline", "descender", "bounds"][..]
            };
            if valid.contains(&s.as_str()) {
                Ok(Value::Str(s.clone()))
            } else {
                Err(vec![edge_cast_error(is_top, value, span)])
            }
        }
        Value::Length(l) => Ok(Value::Length(*l)),
        _ => Err(vec![edge_cast_error(is_top, value, span)]),
    }
}

// ── `upper()` / `lower()` / `replace()` — motor map_text (Passo 67) ─────────

/// `upper(str | content)` → texto em maiúsculas.
pub fn native_upper(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(s)] => Ok(Value::Str(s.to_uppercase())),
        [Value::Content(c)] => {
            let mut f = |text: &str| text.to_uppercase();
            Ok(Value::Content(c.map_text(&mut f)))
        }
        [other] => err(format!(
            "upper() espera string ou content, recebeu {}",
            other.type_name()
        )),
        _ => err("upper() requer 1 argumento".to_string()),
    }
}

/// `lower(str | content)` → texto em minúsculas.
pub fn native_lower(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(s)] => Ok(Value::Str(s.to_lowercase())),
        [Value::Content(c)] => {
            let mut f = |text: &str| text.to_lowercase();
            Ok(Value::Content(c.map_text(&mut f)))
        }
        [other] => err(format!(
            "lower() espera string ou content, recebeu {}",
            other.type_name()
        )),
        _ => err("lower() requer 1 argumento".to_string()),
    }
}

/// `replace(fonte, padrão, substituição, count: N)` → string ou content com substituição.
///
/// `count` é global ao documento: persiste entre nós de texto via `FnMut`.
pub fn native_replace(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    // Validar named args: apenas "count" é aceite.
    for key in args.named.keys() {
        if key.as_str() != "count" {
            return err(format!("argumento nomeado inesperado: '{}'", key));
        }
    }

    if args.items.len() < 3 {
        return err(
            "replace() requer 3 argumentos: fonte, padrão, substituição".to_string()
        );
    }

    let pattern = match &args.items[1] {
        Value::Str(s) => s.to_string(),
        other => {
            return err(format!(
                "replace(): padrão deve ser string, recebeu {}",
                other.type_name()
            ))
        }
    };
    let replacement = match &args.items[2] {
        Value::Str(s) => s.to_string(),
        other => {
            return err(format!(
                "replace(): substituição deve ser string, recebeu {}",
                other.type_name()
            ))
        }
    };

    // Bloquear padrão vazio: replacen("", ...) entra em ciclo infinito.
    if pattern.is_empty() {
        return err("replace(): o padrão de busca não pode estar vazio".to_string());
    }

    let mut remaining_count: Option<i64> =
        args.named.get("count").and_then(|v| match v {
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
        other => err(format!(
            "replace(): 1º argumento deve ser string ou content, recebeu {}",
            other.type_name()
        )),
    }
}

// ── Passo 284 — decoração textual (underline / strike / overline) ───────────
//
// Paridade vanilla `text/deco.rs`: três funções com `body` posicional
// obrigatório + cosméticos `stroke`/`offset`/`extent` opcionais (named).
// Atributos scope-out (`evade`, `background`, objecto Stroke rico) per
// diagnóstico P284 §A.1 + ADR-0054 graded.

/// Discriminador interno para escolher o construtor de `Content::*` no
/// helper partilhado `build_decoration`. Não é exposto em L0 — é apenas
/// um eixo de variação dentro de `text.rs`.
#[derive(Clone, Copy)]
enum DecoKind {
    Underline,
    Strike,
    Overline,
}

fn build_decoration(kind: DecoKind, args: &Args, fn_name: &str) -> SourceResult<Value> {
    let body = match args.items.as_slice() {
        [Value::Content(c)] => c.clone(),
        [Value::Str(s)] => Content::text(s.as_str()),
        [other] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "{fn_name}() espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("{fn_name}() exige body como argumento posicional"),
            )])
        }
        _ => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "{fn_name}() recebeu {} argumentos posicionais (espera 1)",
                    args.items.len()
                ),
            )])
        }
    };

    let mut stroke: Option<Color> = None;
    let mut offset: Option<Length> = None;
    let mut extent: Option<Length> = None;

    for (key, value) in args.named.iter() {
        match key.as_str() {
            "stroke" => {
                // Apenas Color (paint puro) — Stroke rico vanilla é
                // scope-out per diagnóstico §A.1. `none` desactiva default.
                stroke =
                    match value {
                        Value::None => None,
                        v => match parse_color(v) {
                            Some(c) => Some(c),
                            None => return Err(vec![SourceDiagnostic::error(
                                args.span,
                                format!(
                                    "{fn_name}(stroke:) espera color ou none, recebeu {}",
                                    v.type_name()
                                ),
                            )]),
                        },
                    };
            }
            "offset" => {
                offset = match value {
                    Value::None => None,
                    Value::Length(l) => Some(*l),
                    Value::Int(i) => Some(Length::pt(*i as f64)),
                    Value::Float(f) => Some(Length::pt(*f)),
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            args.span,
                            format!(
                                "{fn_name}(offset:) espera length, recebeu {}",
                                other.type_name()
                            ),
                        )])
                    }
                };
            }
            "extent" => {
                extent = match value {
                    Value::None => None,
                    Value::Length(l) => Some(*l),
                    Value::Int(i) => Some(Length::pt(*i as f64)),
                    Value::Float(f) => Some(Length::pt(*f)),
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            args.span,
                            format!(
                                "{fn_name}(extent:) espera length, recebeu {}",
                                other.type_name()
                            ),
                        )])
                    }
                };
            }
            // P284 §A.1 scope-out: erros amigáveis para parâmetros vanilla
            // adiados, em vez de "argumento inesperado" genérico.
            "evade" | "background" => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!(
                        "{fn_name}({}:) não suportado neste passo \
                         (P284 §A.1 scope-out / ADR-0054 graded)",
                        key,
                    ),
                )]);
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("{fn_name}(): argumento nomeado inesperado '{}'", other),
                )])
            }
        }
    }

    // Modelo D (Lote 4 P319): construtores ergonómicos (body sem Box).
    let c = match kind {
        DecoKind::Underline => Content::underline(body, stroke, offset, extent),
        DecoKind::Strike => Content::strike(body, stroke, offset, extent),
        DecoKind::Overline => Content::overline(body, stroke, offset, extent),
    };
    Ok(Value::Content(c))
}

pub fn native_underline(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    build_decoration(DecoKind::Underline, args, "underline")
}

pub fn native_strike(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    build_decoration(DecoKind::Strike, args, "strike")
}

pub fn native_overline(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    build_decoration(DecoKind::Overline, args, "overline")
}

// ── Passo 408 — `smallcaps(body)` ───────────────────────────────────────────
//
// Paridade vanilla `text/smallcaps.rs::SmallcapsElem`: elemento de texto que
// transforma o body em small capitals. Consumer real requer shaping OpenType
// (`smcp` / `c2sc`) — DEBT-53 scope-out XL. Neste passo materializa-se o
// variant `Content::SmallCaps` e a stdlib; o consumer em layout é stub
// transparente (output byte-idêntico ao body).
//
// Modelo minimal: só `body` (sem atributos opcionais). Aceita `content` ou
// `string` como argumento posicional único; named args são rejeitados.

/// `smallcaps(body)` → content embrulhado em `Content::SmallCaps`.
pub fn native_smallcaps(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;

    let body = match args.items.as_slice() {
        [Value::Content(c)] => c.clone(),
        [Value::Str(s)] => Content::text(s.as_str()),
        [other] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "smallcaps() espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "smallcaps() exige body como argumento posicional".to_string(),
            )])
        }
        _ => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "smallcaps() recebeu {} argumentos posicionais (espera 1)",
                    args.items.len()
                ),
            )])
        }
    };

    Ok(Value::Content(Content::smallcaps(body)))
}

// ── Passo 448 — `sub(body)` / `super(body)` ─────────────────────────────────
//
// Paridade vanilla `text/sub.rs::SubElem` e `text/superscript.rs::SuperElem`:
// elementos de texto que deslocam a baseline e reduzem o corpo. Modelo
// minimal: `Content::Styled` com `Style::Subscript`/`Style::Superscript`.

/// `sub(body, size:?)` → content embrulhado em `Content::Styled([Subscript(true)])`.
/// P471: argumento nomeado `size: Length` define tamanho explícito do script.
pub fn native_subscript(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let body = match args.items.as_slice() {
        [Value::Content(c)] => c.clone(),
        [Value::Str(s)] => Content::text(s.as_str()),
        [other] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("sub() espera content ou string, recebeu {}", other.type_name()),
            )])
        }
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "sub() exige body como argumento posicional".to_string(),
            )])
        }
        _ => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "sub() recebeu {} argumentos posicionais (espera 1)",
                    args.items.len()
                ),
            )])
        }
    };
    let mut size: Option<Length> = None;
    for (key, value) in args.named.iter() {
        match key.as_str() {
            "size" => {
                size = match value {
                    Value::Length(l) => Some(*l),
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            args.span,
                            format!(
                                "sub(size:) espera length, recebeu {}",
                                other.type_name()
                            ),
                        )])
                    }
                };
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("sub() argumento nomeado inesperado '{}'", other),
                )])
            }
        }
    }
    Ok(Value::Content(Content::sub_with_size(body, size)))
}

/// `super(body, size:?)` → content embrulhado em `Content::Styled([Superscript(true)])`.
/// P471: argumento nomeado `size: Length` define tamanho explícito do script.
pub fn native_superscript(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let body = match args.items.as_slice() {
        [Value::Content(c)] => c.clone(),
        [Value::Str(s)] => Content::text(s.as_str()),
        [other] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "super() espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "super() exige body como argumento posicional".to_string(),
            )])
        }
        _ => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "super() recebeu {} argumentos posicionais (espera 1)",
                    args.items.len()
                ),
            )])
        }
    };
    let mut size: Option<Length> = None;
    for (key, value) in args.named.iter() {
        match key.as_str() {
            "size" => {
                size = match value {
                    Value::Length(l) => Some(*l),
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            args.span,
                            format!(
                                "super(size:) espera length, recebeu {}",
                                other.type_name()
                            ),
                        )])
                    }
                };
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("super() argumento nomeado inesperado '{}'", other),
                )])
            }
        }
    }
    Ok(Value::Content(Content::superscript_with_size(body, size)))
}

// ── Passo 449 — `highlight(body, fill: color)` ──────────────────────────────
//
// Paridade vanilla `text/highlight.rs::HighlightElem`: fundo colorido por
// detrás do texto. Modelo minimal: `Content::Styled` com `Style::Highlight`.

/// Cor amarela padrão do Typst para highlight (`#ff236`).
fn default_highlight_color() -> Color {
    Color::rgba(255, 242, 54, 255)
}

/// `highlight(body, fill:?, radius:?, extent:?)` → content com fundo colorido.
/// P471: adiciona `radius: Length` (cantos arredondados) e `extent: Length`
/// (extensão horizontal). Cor default: amarelo vanilla `#fff236`.
pub fn native_highlight(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let body = match args.items.as_slice() {
        [Value::Content(c)] => c.clone(),
        [Value::Str(s)] => Content::text(s.as_str()),
        [other] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "highlight() espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "highlight() exige body como argumento posicional".to_string(),
            )])
        }
        _ => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "highlight() recebeu {} argumentos posicionais (espera 1)",
                    args.items.len()
                ),
            )])
        }
    };

    let mut fill: Option<Color> = Some(default_highlight_color());
    let mut radius: Option<Length> = None;
    let mut extent: Option<Length> = None;
    for (key, value) in args.named.iter() {
        match key.as_str() {
            "fill" => {
                fill =
                    match value {
                        Value::None => None,
                        v => match parse_color(v) {
                            Some(c) => Some(c),
                            None => return Err(vec![SourceDiagnostic::error(
                                args.span,
                                format!(
                                    "highlight(fill:) espera color ou none, recebeu {}",
                                    v.type_name()
                                ),
                            )]),
                        },
                    };
            }
            "radius" => {
                radius =
                    match value {
                        Value::None => None,
                        Value::Length(l) => Some(*l),
                        other => return Err(vec![SourceDiagnostic::error(
                            args.span,
                            format!(
                                "highlight(radius:) espera length ou none, recebeu {}",
                                other.type_name()
                            ),
                        )]),
                    };
            }
            "extent" => {
                extent =
                    match value {
                        Value::None => None,
                        Value::Length(l) => Some(*l),
                        other => return Err(vec![SourceDiagnostic::error(
                            args.span,
                            format!(
                                "highlight(extent:) espera length ou none, recebeu {}",
                                other.type_name()
                            ),
                        )]),
                    };
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("highlight() argumento nomeado inesperado '{}'", other),
                )])
            }
        }
    }

    Ok(Value::Content(Content::highlight_full(body, fill, radius, extent)))
}

// ── Passo 287 — função `#smartquote(double, enabled, alternative)` ──────────
//
// Paridade vanilla `text/smartquote.rs::SmartQuoteElem`. Cristalino:
// - `double: bool = true`  — aspas duplas (default).
// - `enabled: bool = true` — smart-quotes activas. Quando `false`, emite
//                            o glyph ASCII literal (`"`/`'`) sem alternância
//                            lang-aware (paridade vanilla `set smartquote(enabled: false)`).
// - `alternative: bool`    — scope-out per ADR-0054 graded (rejeitado com erro
//                            educacional; passo dedicado futuro condicional).
//
// Diagnóstico P287 §A.2: variant `Content::smartquote(double)` é leaf
// (não rico) — 1 campo bool required. Quando `enabled = false`, função emite
// `Content::Text` directo (não passa pelo variant; estado open/close do
// Layouter preservado intacto).

pub fn native_smartquote(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if !args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!(
                "smartquote() não aceita argumentos posicionais (recebeu {})",
                args.items.len()
            ),
        )]);
    }

    let mut double: bool = true; // vanilla default
    let mut enabled: bool = true; // vanilla default

    for (key, value) in args.named.iter() {
        match key.as_str() {
            "double" => match value {
                Value::Bool(b) => double = *b,
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        format!(
                            "smartquote(double:) espera bool, recebeu {}",
                            other.type_name()
                        ),
                    )])
                }
            },
            "enabled" => match value {
                Value::Bool(b) => enabled = *b,
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        format!(
                            "smartquote(enabled:) espera bool, recebeu {}",
                            other.type_name()
                        ),
                    )])
                }
            },
            // Scope-out per diagnóstico P287 §A.2: `alternative` rejeitada
            // com erro educacional (ADR-0054 graded); passo dedicado futuro.
            "alternative" | "quotes" => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!(
                        "smartquote({}:) não suportado neste passo \
                         (P287 §A.2 scope-out / ADR-0054 graded)",
                        key,
                    ),
                )]);
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("smartquote(): argumento nomeado inesperado '{}'", other),
                )])
            }
        }
    }

    if !enabled {
        // Paridade vanilla `enabled: false` — emite glyph ASCII literal
        // sem alternância lang-aware. Não passa pelo variant SmartQuote
        // (consumer Layouter não vê este caso).
        let glyph: &str = if double { "\"" } else { "'" };
        return Ok(Value::Content(Content::text(glyph)));
    }

    Ok(Value::Content(Content::smartquote(double)))
}

// ── Passo 391 + P805 — `lorem(n)` ───────────────────────────────────────

/// `lorem(n)` → `Value::Str` com `n` palavras de Lorem Ipsum.
///
/// `n` deve ser um `Int` ≥ 0. Argumentos nomeados são rejeitados.
///
/// **P805 — byte-parity com o vanilla** (substitui o vocabulário cíclico de
/// Passo 391, cujo scope-out "texto exacto não precisa de coincidir" foi
/// revogado por este passo): usa a mesma crate do vanilla — `lipsum`
/// (whitelist `[l1_allowed_external.lipsum]`) — com a lógica de junção
/// portada do `lorem_impl` do vanilla
/// (`lab/typst-original/crates/typst-library/src/text/lorem.rs`, MIT —
/// baseado na crate lipsum, © 2017 Martin Geisler). A cadeia é construída
/// por chamada (L1 proíbe estado global; o vanilla usa `LazyLock`).
pub fn native_lorem(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;

    let n = match args.items.as_slice() {
        [Value::Int(n)] => *n,
        [other] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("lorem() espera um inteiro, recebeu {}", other.type_name()),
            )])
        }
        _ => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "lorem() requer 1 argumento inteiro",
            )])
        }
    };

    if n < 0 {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "lorem() não aceita números negativos",
        )]);
    }

    Ok(Value::Str(lorem_impl(n as usize).into()))
}

/// Port do `lorem_impl` do vanilla (`typst-library/src/text/lorem.rs`):
/// gera `n` palavras com a Markov chain da crate `lipsum` (ordem 2,
/// `LOREM_IPSUM` + `LIBER_PRIMUS`, iterada de `("Lorem", "ipsum")` com o RNG
/// determinístico interno da crate — ChaCha20Rng seed 97). `--` vira en-dash
/// (U+2013) sem contar como palavra; capitaliza após `.`/`!`/`?`; garante
/// ponto final.
fn lorem_impl(n: usize) -> String {
    use lipsum::{LIBER_PRIMUS, LOREM_IPSUM, MarkovChain};

    if n == 0 {
        return String::new();
    }

    let mut chain = MarkovChain::new();
    chain.learn(LOREM_IPSUM);
    chain.learn(LIBER_PRIMUS);
    let mut iter = chain.iter_from(("Lorem", "ipsum"));

    // Pontuação que termina uma frase.
    const PUNCTUATION: [char; 3] = ['.', '!', '?'];

    let mut sentence = String::new();
    let mut word_count = 0;
    let mut needs_cap = false;

    while word_count < n {
        let Some(word) = iter.next() else { break };

        if word_count > 0 {
            sentence.push(' ');
        }

        // Saltar `--` sem contar como palavra; anexar en-dash ao output.
        if word == "--" {
            sentence.push('\u{2013}');
            continue;
        }

        if needs_cap {
            if let Some(c) = word.chars().next() {
                sentence.extend(c.to_uppercase());
                sentence.push_str(&word[c.len_utf8()..]);
            }
        } else {
            sentence.push_str(word);
        }

        needs_cap = sentence.ends_with(PUNCTUATION);
        word_count += 1;
    }

    // Garantir que a frase termina com um de ".!?".
    if !sentence.ends_with(PUNCTUATION) {
        // Truncar pontuação final pendente para não adicionar '.' após ','.
        let idx = sentence.trim_end_matches(|c: char| c.is_ascii_punctuation()).len();
        sentence.truncate(idx);
        sentence.push('.');
    }

    sentence
}

/// `regex(pattern)` → `Value::Regex` (P393).
///
/// Recebe um único argumento posicional `Str` com uma pattern regex válida.
/// Argumentos nomeados são rejeitados. Pattern inválida → erro de eval.
pub fn native_regex(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;

    match args.items.as_slice() {
        [Value::Str(pattern)] => match Regex::new(pattern.as_str()) {
            Ok(re) => Ok(Value::Regex(re)),
            Err(e) => Err(vec![SourceDiagnostic::error(
                args.span,
                format!("regex inválida: {}", e),
            )]),
        },
        [other] => err(format!("regex() espera string, recebeu {}", other.type_name())),
        _ => err("regex() requer 1 argumento (pattern)".to_string()),
    }
}
