//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/text/constructor.md
//! @prompt-hash c06ea006
//! @layer L1
//! @updated 2026-08-13
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
use crate::entities::layout_types::{Color, Length, Pt};
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::style::{Style, Styles};
use crate::entities::value::Value;

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
    // (`compiler/eval/rules.rs`); os valores transportam-se pelo mesmo mecanismo
    // (`Style` tipado ou canal custom `"text.<campo>"`).
    for (key, value) in args.named.iter() {
        let key_str = key.as_str();
        if key_str == "fill" {
            // Já processado acima; `fill: none` remove o fill previamente definido.
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

    let styled =
        if styles.is_empty() { body } else { Content::Styled(Box::new(body), styles) };
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
