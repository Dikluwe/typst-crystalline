//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/math/expression_call.md
//! @prompt-hash d1c90e4c
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// Avalia uma chamada em modo matemático na ordem lexical e nativa vigente.
pub(super) fn eval_math_func_call(
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    call: crate::entities::ast::expr::FuncCall<'_>,
) -> SourceResult<Content> {
    // P1395 — um callee bare consulta primeiro o scope lexical. O
    // namespace math só participa quando nenhum binding local o
    // sombreia, inclusive para nomes interceptados abaixo como
    // `abs` e `sqrt`.
    if let Expr::MathIdent(ident) = call.callee() {
        let local = scopes.get_local(ident.get()).cloned();
        if let Some(value) = local {
            if let Value::Func(func) = value {
                let mut args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;
                super::call_dispatch::transport_native_call_span(
                    &func,
                    &mut args,
                    call.span(),
                );
                return match apply_func(func, args, scopes, ctx, engine)? {
                    Value::Content(content) => Ok(content),
                    other => Err(vec![SourceDiagnostic::error(
                        call.span(),
                        format!(
                            "{}() em modo math deve devolver content, recebeu {}",
                            ident.get(),
                            other.type_name()
                        ),
                    )]),
                };
            }

            let mut arguments = Vec::new();
            let mut diagnostics = Vec::new();
            for arg in call.args().items() {
                match arg {
                    Arg::Pos(expr) => {
                        if !arguments.is_empty() {
                            arguments.push(Content::MathText(", ".into()));
                        }
                        arguments.push(eval_math_expr(scopes, ctx, engine, expr)?);
                    }
                    Arg::Named(named) => {
                        let fixed = named
                            .to_untyped()
                            .clone()
                            .into_text()
                            .to_string()
                            .replacen(':', "\\:", 1);
                        diagnostics.push(
                            SourceDiagnostic::error(
                                named.span(),
                                "named-argument syntax can only be used with functions",
                            )
                            .with_hint(format!(
                                "to render the colon as text, escape it: `{fixed}`"
                            )),
                        );
                    }
                    Arg::Spread(spread) => {
                        let fixed = spread
                            .to_untyped()
                            .clone()
                            .into_text()
                            .to_string()
                            .replacen("..", ".. ", 1);
                        diagnostics.push(
                            SourceDiagnostic::error(
                                spread.span(),
                                "spread-argument syntax can only be used with functions",
                            )
                            .with_hint(format!(
                                "to render the dots as text, add a space: `{fixed}`"
                            )),
                        );
                    }
                }
            }
            if !diagnostics.is_empty() {
                return Err(diagnostics);
            }

            let base = super::value_to_display_content(value).unwrap_or(Content::Empty);
            let body = Content::MathSequence(std::sync::Arc::from(arguments));
            let parens = Content::math_delimited('(', body, ')');
            return Ok(Content::MathSequence(std::sync::Arc::from(vec![base, parens])));
        }
    }

    let name = match call.callee() {
        Expr::MathIdent(ident) => ident.get().to_string(),
        // **P899** — `dot.double(x)`: caso especial ANTES do
        // despacho namespaced genérico abaixo. Sem isto, o callee
        // `FieldAccess(MathIdent("dot"), "double")` cairia no braço
        // `other_callee`, que chama `eval_math_callee` → tenta
        // resolver "dot" como `Value::Symbol` e aplicar o
        // MODIFICADOR "double" (`Symbol::modified`) — falha com
        // "unknown symbol modifier 'double'" porque `dot` é uma
        // entrada `SYM_SIMPLE` plana (sem tabela de variantes,
        // achado de P895 para o mesmo tipo de confusão
        // símbolo-modificado vs função). `dot.double(x)` é
        // despacho de FUNÇÃO de acento (duplo ponto, U+0308), não
        // um símbolo modificado — trata-se aqui como `name =
        // "dot.double"`, caindo no mesmo `match` hardcoded abaixo
        // que `hat`/`tilde`/`dot`.
        Expr::FieldAccess(access)
            if matches!(
                (access.target(), access.field().as_str()),
                (Expr::MathIdent(t), "double") if t.get() == "dot"
            ) =>
        {
            "dot.double".to_string()
        }
        // P772y — callee namespaced (`math.class(...)`, `calc.foo(...)`):
        // fora do despacho nativo bare-ident abaixo. Resolve via
        // avaliador geral de expressões; se for `Value::Func`,
        // aplica com args avaliados em modo math — mesmo mecanismo
        // do fallback P510 (scope global) mais abaixo, generalizado
        // para callees com field access.
        other_callee => {
            // **P829-C** — callee `target.field(...)` em modo math
            // passa pelo mesmo despacho de erro de P815
            // (`field_callee_error`): o vanilla usa a MESMA rotina de
            // chamada dentro e fora de math (`call.rs:
            // eval_field_callee` — medido c1–c4: `$#d.x()$` produz o
            // erro dict-key + hints verbatim). O target é avaliado
            // uma única vez, tal como `eval_math_callee` faria no
            // braço `FieldAccess`; para alvos
            // Symbol/Func/Type/Module o fallback devolve `None` e o
            // campo resolve normalmente (ex.: `math.class`).
            let callee_value = if let Expr::FieldAccess(access) = other_callee {
                let target = eval_math_callee(scopes, ctx, engine, access.target())?;
                if let Some(err) = super::bindings::field_callee_error(&target, access) {
                    return Err(err);
                }
                super::bindings::eval_value_field_access(
                    target,
                    access.field().as_str(),
                    access.span(),
                )?
            } else {
                eval_math_callee(scopes, ctx, engine, other_callee)?
            };
            let Value::Func(func) = callee_value else {
                return Err(vec![SourceDiagnostic::error(
                    call.span(),
                    format!(
                        "chamada em modo math espera função, recebeu {}",
                        callee_value.type_name()
                    ),
                )]);
            };
            let mut args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;
            super::call_dispatch::transport_native_call_span(
                &func,
                &mut args,
                call.span(),
            );
            return match apply_func(func, args, scopes, ctx, engine)? {
                Value::Content(c) => Ok(c),
                other => Err(vec![SourceDiagnostic::error(
                    call.span(),
                    format!(
                        "chamada em modo math deve devolver content, recebeu {}",
                        other.type_name()
                    ),
                )]),
            };
        }
    };
    super::native_dispatch::dispatch_math_native(name, scopes, ctx, engine, call)
}
