//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/math/native_attachments.md
//! @prompt-hash 0f2381de
//! @layer L1
//! @updated 2026-10-01

use super::*;

/// Despacha scripts, limits e attach com validação sintática original.
pub(super) fn dispatch_math_attachment(
    name: &str,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    call: crate::entities::ast::expr::FuncCall<'_>,
) -> SourceResult<Content> {
    match name {
        // **P992** — `scripts(body)`/`limits(body, inline:)`: força
        // o discriminador `is_limits` do `MathAttach` pai (lateral
        // vs empilhado). Vanilla: `ScriptsElem`/`LimitsElem`
        // (`math/attach.rs`), consolidados aqui num só elemento
        // (`Content::MathLimitsOverride`, ver `entities/elements/
        // math_limits_override.md`). Ver `engine.md` §P992.
        "scripts" => {
            // **P992b** — named args desconhecidos são erro
            // (`unexpected argument: foo`, vanilla medido) — antes
            // descartados silenciosamente pelo `filter_map`.
            let mut pos_args: Vec<Expr<'_>> = Vec::new();
            let mut errors: Vec<SourceDiagnostic> = Vec::new();
            for arg in call.args().items() {
                match arg {
                    Arg::Pos(e) => pos_args.push(e),
                    Arg::Named(n) => errors.push(SourceDiagnostic::error(
                        n.name().span(),
                        format!("unexpected argument: {}", n.name().as_str()),
                    )),
                    Arg::Spread(_) => {}
                }
            }
            if !errors.is_empty() {
                return Err(errors);
            }
            if pos_args.len() != 1 {
                return Err(vec![SourceDiagnostic::error(
                    call.span(),
                    format!(
                        "scripts espera exactamente 1 argumento, recebeu {}",
                        pos_args.len()
                    ),
                )]);
            }
            let body = eval_math_expr(scopes, ctx, engine, pos_args[0])?;
            Ok(Content::math_limits_override(body, false, true, false))
        }

        "limits" => {
            let mut pos_args: Vec<Expr<'_>> = Vec::new();
            let mut inline = true;
            let mut inline_explicit = false;
            let mut errors: Vec<SourceDiagnostic> = Vec::new();
            for arg in call.args().items() {
                match arg {
                    Arg::Pos(e) => pos_args.push(e),
                    Arg::Named(n) if n.name().as_str() == "inline" => {
                        inline_explicit = true;
                        // **P992** — `true`/`false` sem `#` em modo
                        // math lexam como `MathIdent` (só
                        // `compiler/lexer/code.rs::keyword` reconhece o
                        // token `Bool`, não a lexagem de math) —
                        // `eval_math_arg_value`/`Expr::Bool` nunca
                        // dispara para a sintaxe bare do vanilla
                        // (`limits(body, inline: false)`, sem `#`).
                        // Caso especial, mesmo padrão de
                        // `parse_delim_val` para `delim: "["`.
                        match n.expr() {
                            Expr::MathIdent(id) if id.get() == "true" => {
                                inline = true;
                            }
                            Expr::MathIdent(id) if id.get() == "false" => {
                                inline = false;
                            }
                            other => {
                                // **P992b** — valor não-booleano é
                                // erro (`expected boolean, found
                                // content`, vanilla medido) — antes
                                // o `if let Ok(Value::Bool)`
                                // descartava e `inline` ficava
                                // preso em `true`.
                                match eval_math_arg_value(scopes, ctx, engine, other) {
                                    Ok(Value::Bool(b)) => inline = b,
                                    Ok(v) => errors.push(SourceDiagnostic::error(
                                        n.expr().span(),
                                        format!(
                                            "expected boolean, found {}",
                                            v.type_name()
                                        ),
                                    )),
                                    Err(mut e) => errors.append(&mut e),
                                }
                            }
                        }
                    }
                    // **P992b** — named arg desconhecido é erro
                    // (`unexpected argument: foo`, vanilla medido) —
                    // antes `_ => {}` silencioso.
                    Arg::Named(n) => errors.push(SourceDiagnostic::error(
                        n.name().span(),
                        format!("unexpected argument: {}", n.name().as_str()),
                    )),
                    Arg::Spread(_) => {}
                }
            }
            if !errors.is_empty() {
                return Err(errors);
            }
            if pos_args.len() != 1 {
                return Err(vec![SourceDiagnostic::error(
                    call.span(),
                    format!(
                        "limits espera exactamente 1 argumento, recebeu {}",
                        pos_args.len()
                    ),
                )]);
            }
            let body = eval_math_expr(scopes, ctx, engine, pos_args[0])?;
            Ok(Content::math_limits_override(body, true, inline, inline_explicit))
        }

        // P1293 — sintaxe e `math.attach` convergem na mesma validação
        // e no mesmo constructor puro do owner estrutural.
        "attach" => {
            let args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;
            apply_p1293_math_constructor("attach", args, scopes, ctx, engine)
        }

        _ => unreachable!("math attachment already classified"),
    }
}
