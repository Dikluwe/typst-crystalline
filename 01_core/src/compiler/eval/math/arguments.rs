//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/math/arguments.md
//! @prompt-hash a9a7e47f
//! @layer L1
//! @updated 2026-09-30

use super::*;

pub(super) fn extract_math_named<'a>(
    node: &'a crate::entities::syntax_node::SyntaxNode,
) -> Option<crate::entities::ast::expr::Named<'a>> {
    if node.kind() == crate::entities::syntax_kind::SyntaxKind::Named {
        return crate::entities::ast::AstNode::from_untyped(node);
    }
    None
}

/// **P772y** — avalia um argumento posicional/nomeado de uma `FuncCall`
/// namespaced em modo math (ex.: `math.class("relation", body)`). Um
/// literal string avalia directamente para `Value::Str` (paridade com
/// chamadas de código normal — `class` em `math.class` é uma string, não
/// content); literais escalares (int/float/bool/numeric) avaliam em modo
/// código (P825 — paridade vanilla: `#class(3, x)` reporta
/// `found integer`); qualquer outro expr passa por `eval_math_expr` e
/// embrulha-se em `Value::Content`, tal como o mecanismo P510 já fazia
/// para `bb(x)`/`bold(x + y)`.
pub(super) fn eval_math_arg_value(
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    expr: Expr<'_>,
) -> SourceResult<Value> {
    match expr {
        Expr::Str(s) => Ok(Value::Str(EcoString::from(s.get()?))),
        // The math parser keeps an argument expression in a synthetic `Math`
        // wrapper. Peel that wrapper only when its sole child is executable;
        // every ordinary math body keeps the existing Content path.
        Expr::Math(math) => {
            if math.exprs().any(|expr| matches!(expr, Expr::MathAlignPoint(_))) {
                return Ok(Value::Content(eval_math_content(scopes, ctx, engine, math)?));
            }
            let text = math.to_untyped().clone().into_text();
            let reparsed = crate::compiler::parse::parse_anchored(
                text.as_str(),
                crate::entities::syntax_mode::SyntaxMode::Code,
                math.span(),
            );
            let mut code_exprs = reparsed.children().filter_map(Expr::from_untyped);
            let first = code_exprs.next();
            let second = code_exprs.next();
            drop(code_exprs);
            match (first, second) {
                (
                    Some(
                        inner @ (Expr::Str(_)
                        | Expr::Int(_)
                        | Expr::Float(_)
                        | Expr::Bool(_)
                        | Expr::Numeric(_)
                        | Expr::None(_)
                        | Expr::Parenthesized(_)
                        | Expr::CodeBlock(_)
                        | Expr::Array(_)
                        | Expr::Dict(_)
                        | Expr::Closure(_)
                        | Expr::Contextual(_)),
                    ),
                    None,
                ) => eval_expr(inner, scopes, ctx, engine),
                _ => Ok(Value::Content(eval_math_content(scopes, ctx, engine, math)?)),
            }
        }
        // **P825 (sub-A)** — literais escalares avaliam em modo código
        // (paridade vanilla: após `#`, os args são código — `class(3, x)`
        // reporta `found integer`, não `found content`). Só afecta
        // chamadas namespaced via `#` — chamadas bare de módulos são
        // rejeitadas antes (sub-B).
        Expr::Int(_)
        | Expr::Float(_)
        | Expr::Bool(_)
        | Expr::Numeric(_)
        | Expr::None(_)
        | Expr::Parenthesized(_)
        | Expr::CodeBlock(_)
        | Expr::Array(_)
        | Expr::Dict(_)
        | Expr::Closure(_)
        | Expr::Contextual(_) => eval_expr(expr, scopes, ctx, engine),
        other => {
            // **P994** — um ident que resolve no scope para um valor
            // NÃO-Content (ex.: `center` → `Alignment` em
            // `$ #align(center)[$a+b$] $`) é avaliado pelo caminho de
            // scope (`eval_math_callee`) em vez de empacotado como
            // `Value::Content`. Sem isto, o ident virava
            // `Content::Text("center")` via `value_to_display_content` e
            // `native_align` escolhia-o como body (o primeiro `Content`
            // posicional), descartando o corpo real — o nome do
            // alinhamento "vazava" como texto e o corpo desaparecia.
            // Cobre `Expr::Ident` (chamadas `#f(...)` embutidas — o
            // parser entra em modo Code após `#`, `parse/code.rs:61`) e
            // `Expr::MathIdent` (chamadas bare em math). Vanilla resolve
            // `center` como `Alignment` no mesmo ponto (`ir/resolve.rs` —
            // o arg de `align` não é Content).
            //
            // Regra final (documentada, medida contra a suíte):
            // - bindings de utilizador (`get_local`) mantêm o caminho
            //   actual — paridade `get_in_math` do vanilla
            //   (`#let center = [x]` sombreia o alinhamento global);
            // - valores `Content`/`Func`/`Module` mantêm o caminho actual
            //   (já avaliam correctamente; `Module` bare em math é erro
            //   deliberado de P825 — não contornar);
            // - qualquer outro valor global (`Alignment`, `Auto`, `Int`,
            //   `Color`, `None`, …) desvia para `eval_math_callee`, que
            //   devolve o valor real do scope.
            let ident_name = match &other {
                Expr::Ident(i) => Some(i.as_str()),
                Expr::MathIdent(i) => Some(i.as_str()),
                _ => None,
            };
            if let Some(name) = ident_name {
                let desvia = scopes.get_local(name).is_none()
                    && matches!(scopes.get(name),
                        Some(v) if !matches!(v,
                            Value::Content(_) | Value::Func(_) | Value::Module(_)));
                if desvia {
                    return eval_math_callee(scopes, ctx, engine, other);
                }
            }
            Ok(Value::Content(eval_math_expr(scopes, ctx, engine, other)?))
        }
    }
}

/// Avalia uma lista de argumentos math uma vez e preserva a forma de chamada
/// para os constructors canônicos P1293 de `attach` e `binom`.
pub(super) fn eval_p1293_math_args(
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    call_args: AstArgs<'_>,
) -> SourceResult<Args> {
    let mut occurrences = Vec::new();
    for arg in call_args.items() {
        match arg {
            Arg::Pos(expr) => occurrences.push(ArgOccurrence {
                name: None,
                value: eval_math_arg_value(scopes, ctx, engine, expr)?,
                span: expr.span(),
                value_span: expr.span(),
            }),
            Arg::Named(named_arg) => {
                let value = eval_math_arg_value(scopes, ctx, engine, named_arg.expr())?;
                occurrences.push(ArgOccurrence {
                    name: Some(named_arg.name().as_str().into()),
                    value,
                    span: named_arg.span(),
                    value_span: named_arg.expr().span(),
                });
            }
            Arg::Spread(spread) => {
                let value = eval_math_arg_value(scopes, ctx, engine, spread.expr())?;
                match value {
                    Value::None => {}
                    Value::Array(values) => {
                        occurrences.extend(values.into_iter().map(|value| {
                            ArgOccurrence {
                                name: None,
                                value,
                                span: spread.span(),
                                value_span: spread.span(),
                            }
                        }));
                    }
                    Value::Dict(values) => {
                        occurrences.extend(values.into_iter().map(|(name, value)| {
                            ArgOccurrence {
                                name: Some(name),
                                value,
                                span: spread.span(),
                                value_span: spread.span(),
                            }
                        }));
                    }
                    Value::Args(args) => occurrences.extend(args.occurrence_sequence()),
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            spread.span(),
                            format!("cannot spread {}", other.type_name()),
                        )]);
                    }
                }
            }
        }
    }
    Ok(Args::from_occurrences(call_args.span(), occurrences))
}

/// Variante de P1431: parênteses posicionais em `mat((a, b), (c, d))`
/// representam linhas. A decomposição acontece ainda no AST, antes de o
/// conteúdo matemático esconder as fronteiras entre células.
pub(super) fn eval_p1431_mat_args(
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    call_args: AstArgs<'_>,
) -> SourceResult<Args> {
    let mut occurrences = Vec::new();
    let row_mode = call_args.items().any(|arg| matches!(arg, Arg::Pos(Expr::Array(_))));
    let mut trailing_row = Vec::new();
    for arg in call_args.items() {
        match arg {
            Arg::Pos(Expr::Array(array)) => {
                if !trailing_row.is_empty() {
                    occurrences.push(ArgOccurrence {
                        name: None,
                        value: Value::Array(std::mem::take(&mut trailing_row)),
                        span: array.span(),
                        value_span: array.span(),
                    });
                }
                let mut row = Vec::new();
                for item in array.items() {
                    let ArrayItem::Pos(expr) = item else { continue };
                    if let Some(named) = extract_math_named(expr.to_untyped()) {
                        occurrences.push(ArgOccurrence {
                            name: Some(named.name().as_str().into()),
                            value: eval_math_arg_value(
                                scopes,
                                ctx,
                                engine,
                                named.expr(),
                            )?,
                            span: named.span(),
                            value_span: named.expr().span(),
                        });
                    } else {
                        row.push(Value::Content(eval_math_expr(
                            scopes, ctx, engine, expr,
                        )?));
                    }
                }
                if !row.is_empty() {
                    occurrences.push(ArgOccurrence {
                        name: None,
                        value: Value::Array(row),
                        span: array.span(),
                        value_span: array.span(),
                    });
                }
            }
            Arg::Pos(Expr::Math(math)) if math.was_deparenthesized() => {
                let values = math
                    .exprs()
                    .map(|expr| eval_math_arg_value(scopes, ctx, engine, expr))
                    .collect::<SourceResult<Vec<_>>>()?;
                occurrences.push(ArgOccurrence {
                    name: None,
                    value: Value::Array(values),
                    span: math.span(),
                    value_span: math.span(),
                });
            }
            Arg::Pos(expr) => {
                if row_mode {
                    if let Some(named) = extract_math_named(expr.to_untyped()) {
                        occurrences.push(ArgOccurrence {
                            name: Some(named.name().as_str().into()),
                            value: eval_math_arg_value(
                                scopes,
                                ctx,
                                engine,
                                named.expr(),
                            )?,
                            span: named.span(),
                            value_span: named.expr().span(),
                        });
                    } else {
                        trailing_row.push(Value::Content(eval_math_expr(
                            scopes, ctx, engine, expr,
                        )?));
                    }
                } else {
                    occurrences.push(ArgOccurrence {
                        name: None,
                        value: eval_math_arg_value(scopes, ctx, engine, expr)?,
                        span: expr.span(),
                        value_span: expr.span(),
                    });
                }
            }
            Arg::Named(named) => occurrences.push(ArgOccurrence {
                name: Some(named.name().as_str().into()),
                value: eval_math_arg_value(scopes, ctx, engine, named.expr())?,
                span: named.span(),
                value_span: named.expr().span(),
            }),
            Arg::Spread(spread) => {
                let value = eval_math_arg_value(scopes, ctx, engine, spread.expr())?;
                match value {
                    Value::None => {}
                    Value::Array(values) => {
                        occurrences.extend(values.into_iter().map(|value| {
                            ArgOccurrence {
                                name: None,
                                value,
                                span: spread.span(),
                                value_span: spread.span(),
                            }
                        }));
                    }
                    Value::Dict(values) => {
                        occurrences.extend(values.into_iter().map(|(name, value)| {
                            ArgOccurrence {
                                name: Some(name),
                                value,
                                span: spread.span(),
                                value_span: spread.span(),
                            }
                        }));
                    }
                    Value::Args(args) => occurrences.extend(args.occurrence_sequence()),
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            spread.span(),
                            format!("cannot spread {}", other.type_name()),
                        )]);
                    }
                }
            }
        }
    }
    if !trailing_row.is_empty() {
        occurrences.push(ArgOccurrence {
            name: None,
            value: Value::Array(trailing_row),
            span: call_args.span(),
            value_span: call_args.span(),
        });
    }
    Ok(Args::from_occurrences(call_args.span(), occurrences))
}
