//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/bindings/method_dispatch.md
//! @prompt-hash ed4365fe
//! @layer L1
//! @updated 2026-08-12
//!
//! Despacho de métodos sobre `Value`: mutantes (`push`/`pop`/`insert`/…),
//! de acesso (`at`), e as tabelas que classificam cada método.
//!
//! Extraído de `compiler/eval/bindings.rs` no Passo 1013 conforme ADR-0109
//! (atomização — forma B, free function no arquivo da unidade).

use ecow::EcoString;

use crate::compiler::scopes::Scopes;
use crate::entities::args::Args;
use crate::entities::engine::Engine;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

use crate::compiler::eval::EvalContext;

use super::access::{access, long_type_name, missing_key};

/// **P716** — a lista exacta (e completa) de accessor methods do vanilla
/// (`typst-eval/methods.rs:19-21`): `first`, `last`, `at`. Não há outros.
pub(super) fn is_accessor_method(method: &str) -> bool {
    matches!(method, "first" | "last" | "at")
}

/// **P716** — mirror de `Args::expect`: tira o primeiro posicional ou erra
/// "missing argument: {what}" (mensagem do vanilla).
pub(super) fn expect_positional(args: &mut Args, span: Span, what: &str) -> SourceResult<Value> {
    if args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            span,
            format!("missing argument: {what}"),
        )]);
    }
    Ok(args.items.remove(0))
}

/// **P716** — mirror de `Args::finish`: args por consumir são erro. Corre
/// DEPOIS do acesso (ordem do vanilla, `methods.rs:96` — `arr.at(5,
/// default: 0)` erra out-of-bounds, não unexpected argument).
pub(super) fn finish_args(args: &Args, span: Span) -> SourceResult<()> {
    if let Some(name) = args.named.keys().next() {
        return Err(vec![SourceDiagnostic::error(
            span,
            format!("unexpected argument: {name}"),
        )]);
    }
    if !args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            span,
            "unexpected argument".to_string(),
        )]);
    }
    Ok(())
}

/// **P716** — se o tipo tem um método (só de leitura) com este nome, o erro
/// do vanilla é "cannot mutate a temporary value"; senão "type {ty} has no
/// method `{method}`" (`methods.rs:72-80`, `ty.scope().get(method)`). O
/// espelho consulta a superfície de métodos do vanilla: str/bytes têm
/// `first`/`last`/`at`; content tem `func`/`has`/`at`/`fields`/`location`
/// (P829 — `foundations/content/mod.rs:510-590`); version e arguments têm `at`.
fn has_readonly_method(value: &Value, method: &str) -> bool {
    match (value, method) {
        (Value::Str(_) | Value::Bytes(_), "first" | "last" | "at") => true,
        (Value::Content(_), "func" | "has" | "at" | "fields" | "location") => true,
        (Value::Version(_) | Value::Args(_), "at") => true,
        _ => false,
    }
}

/// **P716** — mirror de `call_method_access` do vanilla (`methods.rs:66-98`):
/// devolve a referência mutável ao elemento/campo. Array: `first`/`last`/
/// `at(index)` (índice negativo conta do fim — `locate_opt`, mesma regra do
/// `array.at` de leitura, P714; a mensagem de out-of-bounds da escrita NÃO
/// tem o sufixo "and no default value was specified"). Dict: só `at(key)`.
pub(super) fn call_method_access<'a>(
    value: &'a mut Value,
    method: &str,
    mut args: Args,
    span: Span,
) -> SourceResult<&'a mut Value> {
    if !matches!(value, Value::Array(_) | Value::Dict(_)) {
        return Err(vec![if has_readonly_method(value, method) {
            SourceDiagnostic::error(span, "cannot mutate a temporary value".to_string())
        } else {
            SourceDiagnostic::error(
                span,
                format!("type {} has no method `{method}`", long_type_name(value)),
            )
        }]);
    }

    let slot = match value {
        Value::Array(arr) => match method {
            "first" => match arr.first_mut() {
                Some(slot) => slot,
                None => {
                    return Err(vec![SourceDiagnostic::error(
                        span,
                        "array is empty".to_string(),
                    )])
                }
            },
            "last" => match arr.last_mut() {
                Some(slot) => slot,
                None => {
                    return Err(vec![SourceDiagnostic::error(
                        span,
                        "array is empty".to_string(),
                    )])
                }
            },
            "at" => {
                let index = match expect_positional(&mut args, span, "index")? {
                    Value::Int(i) => i,
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            span,
                            format!("expected integer, found {}", long_type_name(&other)),
                        )])
                    }
                };
                let len = arr.len() as i64;
                let resolved =
                    if index >= 0 { Some(index) } else { len.checked_add(index) };
                match resolved
                    .filter(|&v| v >= 0 && v < len)
                    .and_then(|v| arr.get_mut(v as usize))
                {
                    Some(slot) => slot,
                    None => {
                        return Err(vec![SourceDiagnostic::error(
                            span,
                            format!(
                                "array index out of bounds (index: {index}, len: {len})"
                            ),
                        )])
                    }
                }
            }
            _ => unreachable!("is_accessor_method garante first/last/at"),
        },
        Value::Dict(dict) => match method {
            "at" => {
                let key = match expect_positional(&mut args, span, "key")? {
                    Value::Str(s) => s,
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            span,
                            format!("expected string, found {}", long_type_name(&other)),
                        )])
                    }
                };
                match dict.get_mut(key.as_str()) {
                    Some(slot) => slot,
                    None => return Err(missing_key(span, key.as_str())),
                }
            }
            // dict não tem `first`/`last` (nem no vanilla) → missing method.
            _ => {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    format!("type dictionary has no method `{method}`"),
                )])
            }
        },
        _ => unreachable!("filtrado pelo guard acima"),
    };

    finish_args(&args, span)?;
    Ok(slot)
}

// ── P717 — métodos mutantes (`push`, `pop`, `insert`, `remove`): mirror de
// `is_mutating_method`/`call_method_mut` (vanilla `typst-eval/methods.rs:
// 9-16,24-63`) e do despacho `maybe_resolve_mutating` (`call.rs:189-212`),
// sobre a fundação `access()` de P716. ─────────────────────────────────────

/// **P717** — a lista exacta (e completa) de mutating methods do vanilla
/// (`methods.rs:9-11`): `push`, `pop`, `insert`, `remove`. Não há outros.
pub(in crate::compiler::eval) fn is_mutating_method(method: &str) -> bool {
    matches!(method, "push" | "pop" | "insert" | "remove")
}

/// **P717** — subconjunto aplicável a dict (`methods.rs:14-16`): dict não
/// tem `push`/`pop`.
fn is_dict_mutating_method(method: &str) -> bool {
    matches!(method, "insert" | "remove")
}

/// **P717** — mirror de `maybe_resolve_mutating` (vanilla `call.rs:189-212`):
/// avalia os **args primeiro** (`call.rs:196-198` — `access()` toma o
/// empréstimo mutável de `scopes`), depois resolve o target como local via
/// `access()` (P716; targets temporários erram `cannot mutate a temporary
/// value` antes de qualquer resolução). Devolve `Ok(None)` = fall-through
/// para a cadeia normal de `eval_func_call` — só para tipos cujos campos
/// podem resolver para função (ex.: módulo com função `insert`).
/// Divergência medida e aceite (ver `rules/eval.md` §P717): o fall-through
/// re-avalia target e args (o vanilla passa os já avaliados) — dupla
/// avaliação de efeitos só nesse caminho, sem consumidor em `cetz`.
pub(in crate::compiler::eval) fn try_eval_mutating_method(
    fa: crate::entities::ast::expr::FieldAccess<'_>,
    args_node: crate::entities::ast::expr::Args<'_>,
    span: Span,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Option<Value>> {
    let method: EcoString = fa.field().as_str().into();
    let args = crate::compiler::eval::call_dispatch::eval_args(args_node, scopes, ctx, engine)?;
    match access(fa.target(), scopes, ctx, engine)? {
        // dict não tem push/pop, e dicts deliberadamente não resolvem campos
        // como métodos (vanilla `call.rs:233-238`) — mesma mensagem que o
        // fall-through do vanilla produz, sem a maquinaria.
        Value::Dict(_) if !is_dict_mutating_method(method.as_str()) => {
            Err(vec![SourceDiagnostic::error(
                span,
                format!("type dictionary has no method `{method}`"),
            )])
        }
        target @ (Value::Array(_) | Value::Dict(_)) => {
            call_method_mut(target, method.as_str(), args, span).map(Some)
        }
        Value::Module(_)
        | Value::Func(_)
        | Value::Type(_)
        | Value::Symbol(_)
        | Value::Content(_) => Ok(None),
        // Restantes tipos (escalares, str, bytes, …): nenhum caminho pode
        // resolver o método — erro verbatim do vanilla (medido para string).
        other => Err(vec![SourceDiagnostic::error(
            span,
            format!("type {} has no method `{method}`", long_type_name(other)),
        )]),
    }
}

/// **P717** — mirror de `call_method_mut` (vanilla `methods.rs:24-63`):
/// muta o local e devolve o output (`pop`/`remove` devolvem o elemento
/// removido; `push`/`insert` devolvem none). `insert` de array usa `locate`
/// com `end_ok=true` (`array.rs:246-257`: índice == len permitido, negativo
/// conta do fim) e erra **sem** o sufixo de default; `remove` tem `default:`
/// (`array.rs:261-274`, `dict.rs:241-251`: fora de limites/chave ausente →
/// default **sem mutar**, senão erro — array **com** sufixo, dict **sem
/// hint**, ao contrário do `at_mut` de P716). `finish_args` corre **depois**
/// da mutação (`methods.rs:61`).
fn call_method_mut(
    value: &mut Value,
    method: &str,
    mut args: Args,
    span: Span,
) -> SourceResult<Value> {
    let mut output = Value::None;

    match value {
        Value::Array(arr) => match method {
            "push" => {
                let v = expect_positional(&mut args, span, "value")?;
                arr.push(v);
            }
            "pop" => {
                output = match arr.pop() {
                    Some(v) => v,
                    None => {
                        return Err(vec![SourceDiagnostic::error(
                            span,
                            "array is empty".to_string(),
                        )])
                    }
                };
            }
            "insert" => {
                let index = match expect_positional(&mut args, span, "index")? {
                    Value::Int(i) => i,
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            span,
                            format!("expected integer, found {}", long_type_name(&other)),
                        )])
                    }
                };
                let v = expect_positional(&mut args, span, "value")?;
                let len = arr.len() as i64;
                let resolved =
                    if index >= 0 { Some(index) } else { len.checked_add(index) };
                match resolved.filter(|&i| i >= 0 && i <= len) {
                    Some(i) => arr.insert(i as usize, v),
                    None => {
                        return Err(vec![SourceDiagnostic::error(
                            span,
                            format!(
                                "array index out of bounds (index: {index}, len: {len})"
                            ),
                        )])
                    }
                }
            }
            "remove" => {
                let index = match expect_positional(&mut args, span, "index")? {
                    Value::Int(i) => i,
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            span,
                            format!("expected integer, found {}", long_type_name(&other)),
                        )])
                    }
                };
                let default = args.named.shift_remove("default");
                let len = arr.len() as i64;
                let resolved =
                    if index >= 0 { Some(index) } else { len.checked_add(index) };
                output = match resolved.filter(|&i| i >= 0 && i < len) {
                    Some(i) => arr.remove(i as usize),
                    None => match default {
                        Some(v) => v,
                        None => return Err(vec![SourceDiagnostic::error(
                            span,
                            format!(
                                "array index out of bounds (index: {index}, len: {len}) \
                                     and no default value was specified"
                            ),
                        )]),
                    },
                };
            }
            _ => unreachable!("is_mutating_method garante push/pop/insert/remove"),
        },
        Value::Dict(dict) => match method {
            "insert" => {
                let key = match expect_positional(&mut args, span, "key")? {
                    Value::Str(s) => s,
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            span,
                            format!("expected string, found {}", long_type_name(&other)),
                        )])
                    }
                };
                let v = expect_positional(&mut args, span, "value")?;
                dict.insert(key, v);
            }
            "remove" => {
                let key = match expect_positional(&mut args, span, "key")? {
                    Value::Str(s) => s,
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            span,
                            format!("expected string, found {}", long_type_name(&other)),
                        )])
                    }
                };
                let default = args.named.shift_remove("default");
                output = match dict.shift_remove(key.as_str()) {
                    Some(v) => v,
                    None => match default {
                        Some(v) => v,
                        None => {
                            return Err(vec![SourceDiagnostic::error(
                                span,
                                format!(
                                    "dictionary does not contain key {:?}",
                                    key.as_str()
                                ),
                            )])
                        }
                    },
                };
            }
            _ => unreachable!("try_eval_mutating_method filtra push/pop para dict"),
        },
        _ => unreachable!("try_eval_mutating_method só passa Array/Dict"),
    }

    finish_args(&args, span)?;
    Ok(output)
}

