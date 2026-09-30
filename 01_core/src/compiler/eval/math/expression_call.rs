//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/math/expression_call.md
//! @prompt-hash e2649942
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

        // **§P906** — `underbrace`/`overbrace`/`underbracket`/`overbracket`:
        // conveniência de `Content::math_underover` (P297) para o caso
        // comum. Chars confirmados no vanilla real
        // (`ir/resolve.rs::resolve_underbrace/overbrace/underbracket/
        // overbracket`): underbrace=`⏟` U+23DF, overbrace=`⏞` U+23DE,
        // underbracket=`⎵` U+23B5, overbracket=`⎴` U+23B4. Assinatura:
        // `nome(body, annotation?)` — `body` posicional obrigatório,
        // `annotation` posicional opcional (paridade vanilla:
        // `UnderbraceElem { body: required, annotation: positional
        // Option }`). Sem anotação: `MathUnderover` de 1 nível
        // (`under`/`over` = chave). Com anotação: aninhamento de 2
        // `MathUnderover` — interno = `(body, chave)`, externo =
        // `(interno, anotação)`; o esticamento em `layout_underover`
        // usa `base_box.width` do seu PRÓPRIO `base`, logo no nível
        // interno a chave estica só sobre o body, e no nível externo a
        // anotação (texto normal) fica centrada sob/sobre o conjunto
        // já composto — ver `compiler/layout.md` §P906.
        "overbrace" => {
            let args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;
            crate::compiler::stdlib::math_overbrace_from_args(&args)
        }

        "underbrace" => {
            let args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;
            crate::compiler::stdlib::math_underbrace_from_args(&args)
        }

        "overbracket" => {
            let args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;
            crate::compiler::stdlib::math_overbracket_from_args(&args)
        }

        "overline" => {
            let args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;
            crate::compiler::stdlib::math_overline_from_args(&args)
        }

        "overparen" => {
            let args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;
            crate::compiler::stdlib::math_overparen_from_args(&args)
        }

        "overshell" => {
            let args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;
            crate::compiler::stdlib::math_overshell_from_args(&args)
        }

        "underbracket" => {
            let args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;
            crate::compiler::stdlib::math_underbracket_from_args(&args)
        }

        "underparen" => {
            let args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;
            crate::compiler::stdlib::math_underparen_from_args(&args)
        }

        "undershell" => {
            let args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;
            crate::compiler::stdlib::math_undershell_from_args(&args)
        }

        // `vec` converge com `math.vec`: identidade própria, children
        // variádicos e os mesmos casts/named do construtor público.
        "vec" => {
            let mut delim = ('(', ')');
            let mut align = crate::entities::layout_types::HAlign::Center;
            let mut gap = crate::entities::rel::Rel {
                rel: 0.0,
                abs: crate::entities::layout_types::Length::em(0.2),
            };
            let mut explicit =
                crate::entities::elements::math_vec::MathVecExplicit::default();
            for name in ["delim", "align", "gap"] {
                let key = format!("math.vec.{name}");
                if let Some(value) = engine.styles.custom(&key) {
                    apply_syntax_vec_option(
                        &mut delim,
                        &mut align,
                        &mut gap,
                        &mut explicit,
                        name,
                        value,
                        call.span(),
                        false,
                    )?;
                }
            }

            let mut children = Vec::new();
            for arg in call.args().items() {
                match arg {
                    Arg::Pos(expr) => {
                        children.push(eval_math_expr(scopes, ctx, engine, expr)?);
                    }
                    Arg::Named(named) => {
                        let value =
                            eval_math_arg_value(scopes, ctx, engine, named.expr())?;
                        apply_syntax_vec_option(
                            &mut delim,
                            &mut align,
                            &mut gap,
                            &mut explicit,
                            named.name().as_str(),
                            &value,
                            call.span(),
                            true,
                        )?;
                    }
                    Arg::Spread(_) => {}
                }
            }
            Ok(Content::math_vec_full(children, delim, align, gap, explicit))
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

        // Outros nomes: P301 auto-lookup math (sin, cos, lim, …)
        // + P302 preservação args via MathSequence + MathDelimited.
        // + P303 paralelo lookup-miss: identifier desconhecido
        //   também preserva args (simetria arquitectural).
        //
        // **P302 (HZ confirmado)**: vanilla parser distinguish
        // `sin(x)` como `sin` + `(x)` delimited; cristalino parser
        // produz FuncCall em math mode (divergência) mas eval
        // emula comportamento vanilla retornando
        // `MathSequence([MathOp, MathDelimited((x))])`.
        //
        // **P303 (HX')**: `undef(x)` (identifier sem lookup-hit)
        // produzia `MathIdent("undef")` descartando args (bug
        // pré-P301). Agora `MathSequence([MathIdent, MathDelimited])`
        // — paralelo arquitectural directo P302.
        _ => {
            // **P510** — chamada a funções do scope global em modo math
            // (ex.: `bb(x)`, `bold(x + y)`). Antes do fallback P302/P303,
            // tentar resolver o nome no scope global; se for uma `Func`,
            // avaliar os args como conteúdo math e aplicar.
            // P1292-B — `underline` has two distinct public identities:
            // the global binding is the textual decoration, whereas a
            // bare call in math syntax is sugar for `math.underline`.
            // Resolve only this collision through the math module; code
            // mode continues to see the global textual function.
            let maybe_func = if name == "underline" {
                match scopes.get("math") {
                    Some(Value::Module(module)) => {
                        module.scope().get("underline").cloned()
                    }
                    _ => None,
                }
            } else {
                scopes.get(&name).cloned()
            };
            if let Some(Value::Func(func)) = maybe_func {
                // **P899** — reaproveita `eval_math_arg_value` (já usado
                // pelo caminho de callee namespaced P772y, ex.:
                // `math.class(...)`), em vez de forçar todos os
                // argumentos por `eval_math_expr` + `Value::Content`.
                // Sem isto, um literal string (`bb("R")`) chegava a
                // `native_bb` já embrulhado em `Content::Text` (prosa,
                // via `value_to_display_content` — braço genérico de
                // `eval_math_expr`), nunca como `Value::Str` — o corpo
                // ficava fora do alcance de `apply_math_style`
                // (só cobre `MathIdent`/`MathText`/`MathSequence`/
                // `MathMatrix`), sintoma: `$ bb("R") $` compilava mas
                // sem estilo nenhum (`typst-passo-899-relatorio.md`,
                // achado da revisão pós Fase A — o teste directo de
                // `wrap_math_style` não cobria este caminho de
                // despacho). Unifica a semântica dos dois caminhos de
                // chamada em modo math (bare global vs namespaced).
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
                            "{}() em modo math deve devolver content, recebeu {}",
                            name,
                            other.type_name()
                        ),
                    )]),
                };
            }

            let pos_args: Vec<Expr<'_>> = call
                .args()
                .items()
                .filter_map(|a| match a {
                    Arg::Pos(e) => Some(e),
                    _ => None,
                })
                .collect();
            let base = if let Some(op) = lookup_math_op(scopes, &name) {
                op
            } else if let Some(sym) =
                crate::compiler::math::symbols::ident_to_unicode(&name)
            {
                // **P958** — símbolo com args (`Gamma(z)` → Γ(𝑧)):
                // a cadeia de símbolos do braço standalone
                // (`Expr::MathIdent`) espelhada aqui — antes, o
                // fallback saltava de `lookup_math_op` directo para
                // o literal, e `Gamma(z)` saía `Gamma(𝑧)`. Ver
                // `compiler/eval.md` §P958.
                Content::MathText(sym.into())
            } else if let Some(sym) = crate::compiler::stdlib::sym::sym_lookup(&name) {
                // **P958** — idem via módulo `sym` (símbolos bare de
                // P795), com warning de depreciação verbatim (P820).
                if let Some(msg) = crate::compiler::stdlib::sym::sym_deprecation(&name) {
                    engine.sink.warn_note(call.callee().span(), msg, "");
                }
                Content::MathText(sym.value.to_string().into())
            } else {
                Content::MathIdent(name.into())
            };
            if pos_args.is_empty() {
                // `sin()` / `undef()` — args vazios; só base sem wrapper.
                return Ok(base);
            }
            let body = if pos_args.len() == 1 {
                eval_math_expr(scopes, ctx, engine, pos_args[0])?
            } else {
                // Múltiplos args: separados por `, ` (paridade vanilla).
                let mut items: Vec<Content> = Vec::new();
                for (i, expr) in pos_args.iter().enumerate() {
                    if i > 0 {
                        items.push(Content::MathText(",".into()));
                    }
                    items.push(eval_math_expr(scopes, ctx, engine, *expr)?);
                }
                Content::MathSequence(std::sync::Arc::from(items))
            };
            let delimited = Content::math_delimited('(', body, ')');
            Ok(Content::MathSequence(std::sync::Arc::from(vec![base, delimited])))
        }
    }
}
