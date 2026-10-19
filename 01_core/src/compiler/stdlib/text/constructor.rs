//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/text/constructor.md
//! @prompt-hash 01524e68
//! @layer L1
//! @updated 2026-09-28
//!
//! `text(...)` — constructor de elemento de texto e os seus validadores.
//!
//! Extraído de `stdlib/text.rs` (2026-08-13) conforme ADR-0109 — atomização
//! forma B: a lógica vive no ficheiro da unidade, o hub é só fronteira.

use std::str::FromStr;

use crate::compiler::eval::rules::{
    edge_cast_error, expected_length_error, type_mismatch, VANILLA_TEXT_SET_PROPS,
};
use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::file_id::FileId;
use crate::entities::font_book::FontWeight;
use crate::entities::font_list::{FontFamily, FontList};
use crate::entities::lang::Lang;
use crate::entities::layout_types::Length;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::style::{Style, Styles};
use crate::entities::value::Value;

// ── P492 — constructor `text(...)` ──────────────────────────────────────────

/// `text(...)` — constructor de elemento de texto (parcial).
///
/// Size e fill posicionais são encontrados por tipo, com preferência nomeada.
/// O corpo restante aceita `Content`, `Str` ou `None`.
pub fn native_text(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let mut remaining = args.clone();
    let mut styles = Styles::new();

    // Named declarations have priority; positional search is type-directed.
    if args.named.contains_key("size") {
        for arg in args
            .occurrence_sequence()
            .iter()
            .filter(|arg| arg.name.as_deref() == Some("size"))
        {
            styles.push(Style::Size(require_length(
                &arg.value,
                argument_span(arg.value_span, args.span),
            )?));
        }
        remaining.remove_named("size");
    } else if let Some(index) = remaining
        .items
        .iter()
        .position(|value| matches!(value, Value::Length(_)))
    {
        if let Some(Value::Length(length)) = remaining.remove_positional(index) {
            styles.push(Style::Size(length));
        }
    }
    if args.named.contains_key("fill") {
        for arg in args
            .occurrence_sequence()
            .iter()
            .filter(|arg| arg.name.as_deref() == Some("fill"))
        {
            match &arg.value {
                Value::Color(color) => styles.push(Style::Fill(color.clone())),
                Value::Gradient(_) | Value::Tiling(_) => return Err(vec![SourceDiagnostic::error(args.span,
                    format!("text(fill:) espera color, recebeu {}", arg.value.type_name()))]),
                other => return Err(vec![SourceDiagnostic::error(argument_span(arg.value_span, args.span),
                    format!("expected color, gradient, or tiling, found {}", crate::compiler::eval::operators::error_formatting::vanilla_type_name(other)))]),
            }
        }
        remaining.remove_named("fill");
    } else if let Some(index) = remaining
        .items
        .iter()
        .position(|value| matches!(value, Value::Color(_)))
    {
        if let Some(Value::Color(color)) = remaining.remove_positional(index) {
            styles.push(Style::Fill(color));
        }
    }

    let body_arg = remaining
        .occurrence_sequence()
        .into_iter()
        .find(|arg| arg.name.is_none());
    let body = match body_arg {
        Some(arg) => {
            remaining.remove_positional(0);
            match arg.value {
                Value::Content(content) => content,
                Value::Str(text) => Content::text(text.as_str()),
                Value::None => Content::empty(),
                other => return Err(vec![SourceDiagnostic::error(argument_span(arg.value_span, args.span),
                    format!("expected content, found {}", crate::compiler::eval::operators::error_formatting::vanilla_type_name(&other)))]),
            }
        }
        None => {
            if let Some(arg) = remaining
                .occurrence_sequence()
                .iter()
                .find(|arg| arg.name.as_deref() == Some("body"))
            {
                return Err(vec![SourceDiagnostic::error(
                    argument_span(arg.span, args.span),
                    "the argument `body` is positional",
                )
                .with_hint("try removing `body:`")]);
            }
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: body",
            )]);
        }
    };
    if let Some(arg) =
        remaining.occurrence_sequence().iter().find(|arg| arg.name.is_none())
    {
        return Err(vec![SourceDiagnostic::error(
            argument_span(arg.span, args.span),
            "unexpected argument",
        )]);
    }

    // P865 — argumentos nomeados válidos em `#set text(...)` também são
    // válidos em `#text(...)`. A validação replica a de `eval_set_rule`
    // (`compiler/eval/rules.rs`); os valores transportam-se pelo mesmo mecanismo
    // (`Style` tipado ou canal custom `"text.<campo>"`).
    for (key, value) in args.named.iter() {
        let key_str = key.as_str();
        if key_str == "fill" || key_str == "size" {
            // Processados acima por busca tipada e sequência causal.
            continue;
        }
        if key_str == "variations" {
            crate::entities::font_variations::FontVariations::from_value(
                value, args.span,
            )?;
            styles = styles.push_custom("text.variations", value.clone());
            continue;
        }
        match key_str {
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
            "costs" => {
                let span = args
                    .occurrences
                    .as_ref()
                    .and_then(|items| {
                        items
                            .iter()
                            .rev()
                            .find(|item| item.name.as_deref() == Some("costs"))
                    })
                    .map_or(args.span, |item| item.value_span);
                for (key, value) in crate::compiler::text_costs::parse(value, span)? {
                    styles = styles.push_custom(key, value);
                }
            }
            "overhang" => {
                let span = args
                    .occurrences
                    .as_ref()
                    .and_then(|items| {
                        items
                            .iter()
                            .rev()
                            .find(|item| item.name.as_deref() == Some("overhang"))
                    })
                    .map_or(args.span, |item| item.value_span);
                match value {
                    Value::Bool(_) => {
                        styles = styles.push_custom("text.overhang", value.clone())
                    }
                    other => {
                        return Err(vec![break_policy_error(
                            "expected boolean",
                            other,
                            span,
                        )])
                    }
                }
            }
            "hyphenate" => {
                let span = args
                    .occurrences
                    .as_ref()
                    .and_then(|items| {
                        items
                            .iter()
                            .rev()
                            .find(|item| item.name.as_deref() == Some("hyphenate"))
                    })
                    .map_or(args.span, |item| item.value_span);
                match value {
                    Value::Bool(_) | Value::Auto => {
                        styles = styles.push_custom("text.hyphenate", value.clone())
                    }
                    other => {
                        return Err(vec![break_policy_error(
                            "expected boolean or auto",
                            other,
                            span,
                        )])
                    }
                }
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

    let styled =
        if styles.is_empty() { body } else { Content::styled(Box::new(body), styles) };
    Ok(Value::Content(styled))
}

fn argument_span(origin: Span, fallback: Span) -> Span {
    if origin.is_detached() {
        fallback
    } else {
        origin
    }
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
                    Value::Str(s) => {
                        crate::entities::font_list::FontNamePattern::Literal(s)
                    }
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
                let weight = d
                    .get("weight")
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

/// Diagnostic vocabulary differs from the public short names of types.
fn break_policy_error(expected: &str, found: &Value, span: Span) -> SourceDiagnostic {
    let name = match found.type_name() {
        "int" => "integer",
        "str" => "string",
        "bool" => "boolean",
        other => other,
    };
    SourceDiagnostic::error(span, format!("{expected}, found {name}"))
}
