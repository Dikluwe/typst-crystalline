//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/math/native_dispatch.md
//! @prompt-hash a054e1a9
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// Mantém a tabela exaustiva de funções matemáticas com despacho estático.
pub(super) fn dispatch_math_native(
    name: String,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    call: crate::entities::ast::expr::FuncCall<'_>,
) -> SourceResult<Content> {
    match name.as_str() {
        "frac" => {
            let args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;
            apply_p1293_math_constructor("frac", args, scopes, ctx, engine)
        }
        // sqrt(x) — 1 argumento posicional → Content::MathRoot { index: None }
        "sqrt" => {
            let args: Vec<_> = call
                .args()
                .items()
                .filter_map(|a| match a {
                    Arg::Pos(e) => Some(e),
                    _ => None,
                })
                .collect();
            if args.len() != 1 {
                return Err(vec![SourceDiagnostic::error(
                    call.span(),
                    format!(
                        "sqrt espera exactamente 1 argumento, recebeu {}",
                        args.len()
                    ),
                )]);
            }
            let radicand = eval_math_expr(scopes, ctx, engine, args[0])?;
            Ok(crate::compiler::stdlib::sqrt_content(radicand))
        }
        // root(n, x) — 2 argumentos posicionais: índice, radicando
        "root" => {
            let args: Vec<_> = call
                .args()
                .items()
                .filter_map(|a| match a {
                    Arg::Pos(e) => Some(e),
                    _ => None,
                })
                .collect();
            if args.len() != 2 {
                return Err(vec![SourceDiagnostic::error(
                    call.span(),
                    if args.len() < 2 {
                        "missing argument: radicand".to_string()
                    } else {
                        "unexpected argument".to_string()
                    },
                )]);
            }
            let index = match eval_math_arg_value(scopes, ctx, engine, args[0])? {
                Value::None => None,
                Value::Content(content) => Some(content),
                value => {
                    Some(super::value_to_display_content(value).unwrap_or(Content::Empty))
                }
            };
            let radicand = eval_math_expr(scopes, ctx, engine, args[1])?;
            Ok(Content::math_root(index, radicand, true))
        }
        // **P899 (Parte A)** — `hat`/`tilde`/`dot` são funções de
        // acento (`dot.double` é tratado à parte, callee com
        // FieldAccess — ver braço `other_callee` acima). Mecanismo
        // vanilla (`Symbol::func` + `Accent::combining`,
        // `foundations/symbol.rs`/`math/accent.rs`): resolve o
        // símbolo, procura o combining-mark na tabela `ACCENTS`,
        // chama `accent(base, combining_char)`. Cristalino não
        // replica o mecanismo genérico "símbolo chamável"; mapeia
        // directamente para o combining-mark (mesmos valores da
        // tabela `ACCENTS` do vanilla), reaproveitando
        // `Content::math_accent` (já existente desde Passo 296,
        // `layout_accent` já funciona sem mudança). Achado da Fase A
        // que corrige o catálogo do próprio passo: `bar(x)` NÃO é
        // acento — confirmado no vanilla real, `bar(x)` → `|x|`
        // (delimitador via `sym.bar` = `|`) — implementado em
        // "Parte B" (braço `"abs" | "bar" | ...`), não aqui.
        "hat" | "tilde" | "dot" | "dot.double" => {
            let args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;
            if args.items.len() != 1 {
                return Err(vec![SourceDiagnostic::error(
                    call.span(),
                    if args.items.is_empty() {
                        "missing argument: base".to_string()
                    } else {
                        "unexpected argument".to_string()
                    },
                )]);
            }
            let accent_char = match name.as_str() {
                "hat" => '\u{0302}',
                "tilde" => '\u{0303}',
                "dot" => '\u{0307}',
                "dot.double" => '\u{0308}',
                _ => unreachable!(),
            };
            let mut occurrences = args.occurrence_sequence();
            let base_position = occurrences
                .iter()
                .position(|occurrence| occurrence.name.is_none())
                .expect("one positional base");
            occurrences.insert(
                base_position + 1,
                ArgOccurrence {
                    name: None,
                    value: Value::Str(accent_char.to_string().into()),
                    span: call.span(),
                    value_span: call.span(),
                },
            );
            let args = Args::from_occurrences(call.span(), occurrences);
            crate::compiler::stdlib::math_accent_content(&args)
        }

        // **P899 (Parte B)** — `abs`/`norm`/`floor`/`ceil`/`round`/
        // `bar`: wrappers finos de `delimited(body, open, close)` no
        // vanilla (`typst-library/src/math/lr.rs`); reaproveitam
        // directamente `Content::math_delimited`, o mesmo
        // construtor já usado por `(x)`/`[x]` literais e pelo
        // fallback `sin(x)`. `round` usa o par assimétrico `⌊`/`⌉`
        // (paridade vanilla medida em `lab/typst-original`). `bar`
        // — apesar de listado como "Parte A" (acento) na
        // materialização — confirmado no vanilla real (`lab/
        // typst-original`) como delimitador `|x|`, NÃO acento
        // (`sym.bar` resolve para `|`, chamado via
        // `get_lr_wrapper_func`, não `Accent::combining`) — corrigido
        // aqui, ver `typst-passo-899-relatorio.md`. Sem suporte ao
        // named arg `size:` (scope-out — `size` no vanilla é
        // `Rel<Length>` relativo à altura do conteúdo vindo do
        // stretch automático, que já é o comportamento por omissão;
        // `Content::math_delimited` não tem campo para o override
        // manual).
        // **P981** — `lr(body)`: o corpo inclui os delimitadores
        // (vanilla `math/lr.rs` + `ir/resolve.rs:850-940`). O
        // cristalino já estica delimitadores por omissão no caminho
        // `MathDelimited`, logo: grupo já delimitado → devolvido
        // inalterado; sequência com opener/closer soltos (ex.:
        // `lr(chevron.l a/b chevron.r)`) → reescrita para
        // `math_delimited`. Scope-out: named `size:` (ver
        // `compiler/eval.md` §P981).
        "lr" => {
            let args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;
            crate::compiler::stdlib::math_lr_from_args(&args)
        }

        "mid" => {
            let args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;
            crate::compiler::stdlib::math_mid_from_args(&args)
        }

        "norm" => {
            let args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;
            crate::compiler::stdlib::math_norm_from_args(&args)
        }

        "round" => {
            let args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;
            crate::compiler::stdlib::math_round_from_args(&args)
        }

        "abs" => {
            let args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;
            crate::compiler::stdlib::math_abs_from_args(&args)
        }

        "stretch" => {
            let args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;
            crate::compiler::stdlib::math_stretch_from_args(&args)
        }

        "scripts" | "limits" | "attach" => {
            super::native_attachments::dispatch_math_attachment(
                name.as_str(),
                scopes,
                ctx,
                engine,
                call,
            )
        }

        "floor" | "ceil" | "bar" => {
            let pos_args: Vec<Expr<'_>> = call
                .args()
                .items()
                .filter_map(|a| match a {
                    Arg::Pos(e) => Some(e),
                    _ => None,
                })
                .collect();
            if pos_args.len() != 1 {
                return Err(vec![SourceDiagnostic::error(
                    call.span(),
                    format!(
                        "{} espera exactamente 1 argumento, recebeu {}",
                        name,
                        pos_args.len()
                    ),
                )]);
            }
            let (open, close) = match name.as_str() {
                "bar" => ('|', '|'),
                "norm" => ('‖', '‖'),
                "floor" => ('⌊', '⌋'),
                "ceil" => ('⌈', '⌉'),
                _ => unreachable!(),
            };
            let body = eval_math_expr(scopes, ctx, engine, pos_args[0])?;
            Ok(Content::math_delimited(open, body, close))
        }

        // P1293 — mesma validação e mesmo payload canônico da função
        // pública, preservando lower variádico e fração sem barra.
        "binom" => {
            let args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;
            apply_p1293_math_constructor("binom", args, scopes, ctx, engine)
        }

        "overbrace" | "underbrace" | "overbracket" | "overline" | "overparen"
        | "overshell" | "underbracket" | "underparen" | "undershell" | "vec" => {
            super::native_decorations::dispatch_math_decoration(
                name.as_str(),
                scopes,
                ctx,
                engine,
                call,
            )
        }

        // P1430 — `cases` bare e qualificado compartilham validação e
        // conservam children/MathAlignPoint até ao layout.
        "cases" => {
            let args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;
            apply_p1293_math_constructor("cases", args, scopes, ctx, engine)
        }

        // P1431 — bare e qualificado convergem no mesmo constructor.
        "mat" => {
            let rollback = (!ctx.apply_show_rules).then(|| {
                (
                    scopes.top.clone(),
                    scopes.scopes.clone(),
                    scopes.captured.clone(),
                    scopes.captured_by,
                )
            });
            let args = eval_p1431_mat_args(scopes, ctx, engine, call.args())?;
            if let Some((top, parents, captured, captured_by)) = rollback {
                scopes.top = top;
                scopes.scopes = parents;
                scopes.captured = captured;
                scopes.captured_by = captured_by;
            }
            apply_p1293_math_constructor("mat", args, scopes, ctx, engine)
        }

        _ => {
            super::call_fallback::eval_math_call_fallback(name, scopes, ctx, engine, call)
        }
    }
}
