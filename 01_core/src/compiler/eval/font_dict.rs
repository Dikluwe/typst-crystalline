//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/font_dict.md
//! @prompt-hash 2df0f680
//! @layer L1
//! @updated 2026-08-12
//!
//! Parsing do argumento `text.font`: formatos named fields (vanilla) e legacy
//! (cristalino). Extraído de `compiler/eval/rules.rs` no Passo 1011 conforme
//! ADR-0109 (atomização — forma B, free function no arquivo da unidade).

use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

use crate::compiler::eval::{eval_expr, EvalContext};
use crate::compiler::scopes::Scopes;
use crate::entities::ast::expr::{Dict, DictItem, Expr};
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;
use crate::entities::engine::Engine;

/// P414: parsing do dict `text.font` no formato named fields vanilla.
///
/// Campos suportados: `family` (Str|Regex), `variant` (Str),
/// `weight` (Int|Str), `style` (Str), `fallback` (Bool, default true).
/// `stretch` e outros campos são rejeitados.
pub(crate) fn parse_font_dict_named_fields<'a>(
    dict_node: Dict<'a>,
    span: Span,
    scopes: &mut Scopes,
    ctx: &mut EvalContext,
    engine: &mut Engine,
) -> SourceResult<Vec<Value>> {
    use crate::entities::value::Value;

    let mut family: Option<Value> = None;
    let mut variant: Option<EcoString> = None;
    let mut weight: Option<EcoString> = None;
    let mut style: Option<EcoString> = None;
    let mut fallback = true;

    for item in dict_node.items() {
        let named = match item {
            DictItem::Named(n) => n,
            _ => {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    "font dict named fields must use identifiers as keys".to_string(),
                )]);
            }
        };

        let field = named.name().as_str();
        let value = eval_expr(named.expr(), scopes, ctx, engine)?;

        match field {
            "family" => {
                if !matches!(value, Value::Str(_) | Value::Regex(_)) {
                    return Err(vec![SourceDiagnostic::error(
                        span,
                        format!(
                            "font dict field 'family' expects string or regex, recebeu {}",
                            value.type_name()
                        ),
                    )]);
                }
                family = Some(value);
            }
            "variant" => match value {
                Value::Str(s) => variant = Some(s),
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        span,
                        format!(
                            "font dict field 'variant' expects string, recebeu {}",
                            other.type_name()
                        ),
                    )]);
                }
            },
            "weight" => {
                let s = match value {
                    Value::Int(n) => EcoString::from(n.to_string()),
                    Value::Str(s) => s,
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            span,
                            format!(
                                "font dict field 'weight' expects integer or string, recebeu {}",
                                other.type_name()
                            ),
                        )]);
                    }
                };
                weight = Some(s);
            }
            "style" => match value {
                Value::Str(s) => style = Some(s),
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        span,
                        format!(
                            "font dict field 'style' expects string, recebeu {}",
                            other.type_name()
                        ),
                    )]);
                }
            },
            "fallback" => match value {
                Value::Bool(b) => fallback = b,
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        span,
                        format!(
                            "font dict field 'fallback' expects boolean, recebeu {}",
                            other.type_name()
                        ),
                    )]);
                }
            },
            other => {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    format!("unknown font dict field: {}", other),
                )]);
            }
        }
    }

    let family = family.ok_or_else(|| {
        vec![SourceDiagnostic::error(
            span,
            "font dict missing field 'family'".to_string(),
        )]
    })?;

    if !fallback {
        // `fallback: false` semanticamente força fonte única. Como a forma
        // named fields gera um único item, a verificação é documental; o
        // layout respeita a lista resultante.
    }

    let mut entry: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
    entry.insert(EcoString::from("name"), family);
    entry.insert(EcoString::from("variants"), Value::Array(vec![]));
    if let Some(v) = variant {
        entry.insert(EcoString::from("variant"), Value::Str(v));
    }
    if let Some(w) = weight {
        entry.insert(EcoString::from("weight"), Value::Str(w));
    }
    if let Some(s) = style {
        entry.insert(EcoString::from("style"), Value::Str(s));
    }

    Ok(vec![Value::Dict(entry)])
}

/// P407: parsing do dict `text.font` no formato legado cristalino.
///
/// Chaves são nomes de família (string literal, identificador ou regex);
/// valores são variant names (string ou array de strings).
pub(crate) fn parse_font_dict_legacy<'a>(
    dict_node: Dict<'a>,
    span: Span,
    scopes: &mut Scopes,
    ctx: &mut EvalContext,
    engine: &mut Engine,
) -> SourceResult<Vec<Value>> {
    let mut items = Vec::new();
    for dict_item in dict_node.items() {
        let (name_val, variants_val) = match dict_item {
            DictItem::Keyed(keyed) => {
                let key_expr = keyed.key();
                let name_val = match key_expr {
                    Expr::Str(node) => Value::Str(EcoString::from(node.get()?)),
                    Expr::FuncCall(call) => {
                        // regex("...") avalia para Value::Regex.
                        match eval_expr(Expr::FuncCall(call), scopes, ctx, engine)? {
                            Value::Regex(re) => Value::Regex(re),
                            other => {
                                return Err(vec![SourceDiagnostic::error(
                                    span,
                                    format!("font dict key must be string or regex, recebeu {}", other.type_name()),
                                )]);
                            }
                        }
                    }
                    _other => {
                        return Err(vec![SourceDiagnostic::error(
                            span,
                            "font dict key must be string or regex".to_string(),
                        )]);
                    }
                };
                let value = eval_expr(keyed.expr(), scopes, ctx, engine)?;
                let variants = variants_from_value(value, span)?;
                (name_val, variants)
            }
            DictItem::Named(named_item) => {
                // Identificador como key: "Name": value é syntax sugar
                // para string literal lowercased.
                let name = named_item.name().as_str();
                let name_val = Value::Str(EcoString::from(name));
                let value = eval_expr(named_item.expr(), scopes, ctx, engine)?;
                let variants = variants_from_value(value, span)?;
                (name_val, variants)
            }
            DictItem::Spread(_) => {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    "font dict spread not supported".to_string(),
                )]);
            }
        };
        let mut entry: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
        entry.insert(EcoString::from("name"), name_val);
        entry.insert(EcoString::from("variants"), Value::Array(variants_val));
        items.push(Value::Dict(entry));
    }
    if items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            span,
            "font dict must not be empty".to_string(),
        )]);
    }
    Ok(items)
}

/// Helper partilhado entre os dois parsers de dict de fonte.
pub(crate) fn variants_from_value(value: Value, span: Span) -> SourceResult<Vec<Value>> {
    match value {
        Value::Str(s) => Ok(vec![Value::Str(s)]),
        Value::Array(arr) => {
            let mut vs = Vec::with_capacity(arr.len());
            for v in arr.iter() {
                if let Value::Str(s) = v {
                    vs.push(Value::Str(s.clone()));
                } else {
                    return Err(vec![SourceDiagnostic::error(
                        span,
                        "font variants must be strings".to_string(),
                    )]);
                }
            }
            Ok(vs)
        }
        other => Err(vec![SourceDiagnostic::error(
            span,
            format!(
                "font dict value must be string or array of strings, recebeu {}",
                other.type_name()
            ),
        )]),
    }
}
