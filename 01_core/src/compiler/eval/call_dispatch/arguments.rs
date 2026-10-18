//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/call_dispatch/arguments.md
//! @prompt-hash a2fba560
//! @layer L1
//! @updated 2026-10-07

use super::*;

/// Avalia a lista de argumentos de uma chamada de função.
///
/// Posicionais são avaliados em ordem; named args são avaliados e indexados
/// por nome. **P718** — `..spread` mirror de `ast::Args::eval` (vanilla
/// `call.rs:367-416`): `none` ignorado; `array` vira posicionais; `dict`
/// vira nomeados; `Value::Args` (reencaminhamento de um sink `..rest` de
/// outra closure) funde posicionais e nomeados; outro tipo erra `cannot
/// spread {ty}` — **sem** o sufixo "into X" das mensagens de literal
/// (`Expr::Array`/`Expr::Dict`, `mod.rs`), mensagem distinta no vanilla.
pub(in crate::compiler::eval) fn eval_args(
    args_node: crate::entities::ast::expr::Args<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Args> {
    // P772s — span da lista de argumentos da chamada real no documento,
    // propagado para todas as mensagens de erro de validação de argumento
    // das funções nativas que usam `args.span` em vez de `Span::detached()`.
    let call_span = args_node.span();
    let mut occurrences = Vec::new();
    for arg in args_node.items() {
        match arg {
            Arg::Pos(expr) => occurrences.push(ArgOccurrence {
                name: None,
                value: eval_expr(expr, scopes, ctx, engine)?,
                span: expr.span(),
                value_span: expr.span(),
            }),
            Arg::Named(name_expr) => {
                occurrences.push(ArgOccurrence {
                    name: Some(name_expr.name().as_str().into()),
                    value: eval_expr(name_expr.expr(), scopes, ctx, engine)?,
                    span: name_expr.span(),
                    value_span: name_expr.expr().span(),
                });
            }
            Arg::Spread(spread) => {
                let value = eval_expr(spread.expr(), scopes, ctx, engine)?;
                match value {
                    Value::None => {}
                    Value::Array(arr) => {
                        occurrences.extend(arr.into_iter().map(|value| ArgOccurrence {
                            name: None,
                            value,
                            span: spread.span(),
                            value_span: spread.span(),
                        }))
                    }
                    Value::Dict(dict) => {
                        occurrences.extend(dict.into_iter().map(|(name, value)| {
                            ArgOccurrence {
                                name: Some(name),
                                value,
                                span: spread.span(),
                                value_span: spread.span(),
                            }
                        }))
                    }
                    Value::Args(args) => {
                        occurrences.extend(args.occurrence_sequence());
                    }
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            spread.span(),
                            format!("cannot spread {}", vanilla_type_name(&other)),
                        )]);
                    }
                }
            }
        }
    }
    Ok(Args::from_occurrences(call_span, occurrences))
}

/// Transporta a chamada inteira para nativas cujo erro usa o agregado.
pub(in crate::compiler::eval) fn transport_native_call_span(
    func: &Func,
    args: &mut Args,
    call_span: Span,
) {
    if p1339_native_call_span(func) {
        args.span = call_span;
        return;
    }
    use crate::compiler::stdlib::{
        calc_abs, native_csv, native_json_encode, native_panic, native_toml_encode,
        native_yaml_encode,
    };
    match func.repr() {
        FuncRepr::With(with) => transport_native_call_span(&with.0, args, call_span),
        FuncRepr::Native(native) => {
            if std::ptr::fn_addr_eq(
                native.call,
                native_json_encode as fn(_, _, _, _) -> _,
            ) || std::ptr::fn_addr_eq(
                native.call,
                native_toml_encode as fn(_, _, _, _) -> _,
            ) || std::ptr::fn_addr_eq(
                native.call,
                native_yaml_encode as fn(_, _, _, _) -> _,
            ) || std::ptr::fn_addr_eq(native.call, native_panic as fn(_, _, _, _) -> _)
                || std::ptr::fn_addr_eq(native.call, native_csv as fn(_, _, _, _) -> _)
                || std::ptr::fn_addr_eq(native.call, calc_abs as fn(_, _, _, _) -> _)
                || std::ptr::fn_addr_eq(
                    native.call,
                    crate::compiler::eval::bindings::native_sequence
                        as fn(_, _, _, _) -> _,
                )
                || std::ptr::fn_addr_eq(
                    native.call,
                    crate::compiler::stdlib::native_table as fn(_, _, _, _) -> _,
                )
                || std::ptr::fn_addr_eq(
                    native.call,
                    crate::compiler::stdlib::native_table_cell as fn(_, _, _, _) -> _,
                )
            {
                args.span = call_span;
            }
        }
        _ => {}
    }
}
