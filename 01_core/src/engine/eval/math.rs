//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/eval.md
//! @prompt-hash 3532b6fa
//! @layer L1
//! @updated 2026-04-22
//!
//! Avaliação de expressões matemáticas. Extraído de `eval.rs` no Passo 96.1
//! conforme ADR-0037 (coesão por domínio).

use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

use crate::engine::scopes::Scopes;
use crate::entities::args::Args;
use crate::entities::ast::expr::{Arg, ArrayItem, Expr};
use crate::entities::ast::math::{Math, MathTextKind};
use crate::entities::ast::AstNode;
use crate::entities::content::Content;
use crate::entities::engine::Engine;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

use super::{apply_func, eval_expr, EvalContext};

// ── Passo 301 — Auto-lookup math mode ─────────────────────────────────────
//
// P301 (HP + P301.A + (a) eval-time + (γ) híbrido): consulta scope
// `math` (P299 SSoT) antes de fallback `MathIdent`. Identifiers
// vanilla pré-definidos (`sin`/`cos`/`lim`/etc., 42 ops) resolvem
// automaticamente para `Content::MathOp` em math mode.
//
// Heurística pré-P301 preservada como fallback:
// - `is_limit_function`/`is_large_operator` em `attach.rs` continua
//   a aplicar para casos não cobertos por scope (operadores
//   Unicode literais).
// - Variables user (`x`, `f`, etc.) continuam `MathIdent` (lookup
//   retorna None).

/// Lookup helper: consulta scope `math` (P299) e retorna `MathOp`
/// clone se encontrado. None caso contrário (incluindo se valor
/// não é `Content::MathOp`). **P731** — `math` passou a `Value::Module`
/// (era `Value::Dict`); o lookup é no scope do módulo.
fn lookup_math_op(scopes: &Scopes<'_>, name: &str) -> Option<Content> {
    let Value::Module(math_module) = scopes.get("math")? else {
        return None;
    };
    let Value::Content(c) = math_module.scope().get(name)? else {
        return None;
    };
    Some(c.clone())
}

/// **P780** — mensagem de erro para identificador não resolvido em modo
/// math. Paridade `unknown_variable_math` (vanilla, `foundations/
/// scope.rs:439-472`) — **distinta** de `unknown_variable` (P772r, usada
/// em código normal): hints diferentes por caso, medidos directamente
/// contra o vanilla, não assumidos:
///
/// - `none`/`auto`/`false`/`true`: hint "adicionar `#` antes" (`#none`).
/// - conhecido em `base.global` mas não em math (`in_global`): 3 hints —
///   "não disponível directamente em math", "`#nome` em código",
///   "`std.nome` em math".
/// - desconhecido de todo: 2 hints — espaçar as letras (`f o o`), ou
///   citar como texto (`"foobarbaz"`).
fn unknown_variable_math(span: Span, name: &str, in_global: bool) -> SourceDiagnostic {
    let diag = SourceDiagnostic::error(span, format!("unknown variable: {name}"));
    if matches!(name, "none" | "auto" | "false" | "true") {
        diag.with_hint(format!(
            "if you meant to use a literal, try adding a hash before it: `#{name}`"
        ))
    } else if in_global {
        diag.with_hint(format!(
            "`{name}` is not available directly in math, but is in the standard library"
        ))
        .with_hint(format!(
            "to access `{name}` in code mode you can add a hash: `#{name}`"
        ))
        .with_hint(format!(
            "or access `{name}` in math mode by using the `std` module: `std.{name}`"
        ))
    } else {
        let spaced: String = name.chars().flat_map(|c| [' ', c]).skip(1).collect();
        diag.with_hint(format!(
            "if you meant to display multiple letters as is, \
             try adding spaces between each letter: `{spaced}`"
        ))
        .with_hint(format!(
            "or if you meant to display this as text, try placing it in quotes: `\"{name}\"`"
        ))
    }
}

/// Avalia o corpo de uma equação matemática — produz `Content` a partir de `Math<'_>`.
///
/// Stub intencional (Passo 34): produz a estrutura de nós correcta sem motor de
/// renderização. O motor real (Passo 36+) substitui esta função com layout tipográfico.
pub(super) fn eval_math_content(
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    math: Math<'_>,
) -> SourceResult<Content> {
    let mut nodes: Vec<Content> = Vec::new();
    for expr in math.exprs() {
        let node = eval_math_expr(scopes, ctx, engine, expr)?;
        if !matches!(node, Content::Empty) {
            nodes.push(node);
        }
    }
    match nodes.len() {
        0 => Ok(Content::Empty),
        1 => Ok(nodes.remove(0)),
        _ => Ok(Content::MathSequence(nodes.into())),
    }
}

/// **P772y** — resolve um `Expr::FieldAccess`/`Expr::MathIdent` em modo
/// math a um `Value`, quando o alvo não pode passar pelo `eval_expr`
/// genérico. Usado originalmente só para o callee de uma `FuncCall` (ex.:
/// `math.class(...)`, `calc.foo(...)`); **P782** reutiliza-o também para
/// `Expr::FieldAccess` standalone numa sequência math (`sym.suit.heart`,
/// bare ou via `#`), não só como callee. Não delega directamente ao
/// `eval_expr` genérico porque este trata `Expr::MathIdent` como fronteira
/// deliberada (devolve `Value::None` — `eval/mod.rs`, comentário
/// "Fronteira deliberada") — o alvo de um field access em modo math (ex.
/// `math` em `math.class`, `sym` em `sym.suit`) precisa de ser resolvido
/// no scope directamente quando é um `MathIdent`. Cobre os tipos de target
/// relevantes (`Value::Module`/`Dict`/`Symbol`); outros tipos produzem
/// erro claro em vez de silenciosamente devolver `Content::Empty`.
fn eval_math_callee(
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    expr: Expr<'_>,
) -> SourceResult<Value> {
    match expr {
        Expr::FieldAccess(access) => {
            let target = eval_math_callee(scopes, ctx, engine, access.target())?;
            let field = access.field().as_str();
            super::bindings::eval_value_field_access(target, field, access.span())
        }
        Expr::MathIdent(ident) => {
            let name = ident.get();
            if let Some(val) = scopes.get(name).cloned() {
                // **P825 (sub-achado B de P810 §12)** — módulos globais
                // (`math`, `sym`, `calc`, `emoji`, …) NÃO são acessíveis
                // bare em modo math — paridade vanilla medida:
                // `error: unknown variable: math` + 3 hints. Excepção
                // medida: `std` É acessível bare (`$ std.math.class(...) $`
                // compila nos dois). Funções (`Value::Func`), bindings de
                // utilizador e outros valores não são afectados. Reforço da
                // validação de P782 (não duplicação): o caminho via `#`
                // (target `Expr::Ident` de código) passa pelo braço
                // `other => eval_expr` e não é tocado.
                if matches!(val, Value::Module(_)) && name != "std" {
                    return Err(vec![unknown_variable_math(ident.span(), name, true)]);
                }
                Ok(val)
            } else if let Some(sym) = crate::engine::stdlib::sym::sym_lookup(name) {
                // P820 — símbolo depreciado (ex.: `join`): resolve, mas com
                // warning verbatim do vanilla (span na raiz — medido
                // `$join.r$` → warning @1:1).
                if let Some(msg) = crate::engine::stdlib::sym::sym_deprecation(name) {
                    engine.sink.warn_note(ident.span(), msg, "");
                }
                Ok(Value::Symbol(sym))
            } else {
                // P820 (sub-achado (b) de P810 §7): a mensagem portuguesa
                // `variável desconhecida` era divergente — o vanilla usa
                // sempre `unknown variable` + hints (`$foo.bar$` medido).
                Err(vec![unknown_variable_math(
                    ident.span(),
                    name,
                    scopes.has_global(name),
                )])
            }
        }
        other => eval_expr(other, scopes, ctx, engine),
    }
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
fn eval_math_arg_value(
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    expr: Expr<'_>,
) -> SourceResult<Value> {
    match expr {
        Expr::Str(s) => Ok(Value::Str(EcoString::from(s.get()?))),
        // **P825 (sub-A)** — literais escalares avaliam em modo código
        // (paridade vanilla: após `#`, os args são código — `class(3, x)`
        // reporta `found integer`, não `found content`). Só afecta
        // chamadas namespaced via `#` — chamadas bare de módulos são
        // rejeitadas antes (sub-B).
        Expr::Int(_) | Expr::Float(_) | Expr::Bool(_) | Expr::Numeric(_) => {
            eval_expr(expr, scopes, ctx, engine)
        }
        other => Ok(Value::Content(eval_math_expr(scopes, ctx, engine, other)?)),
    }
}

/// Avalia um nó de expressão em modo matemático.
fn eval_math_expr(
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    expr: Expr<'_>,
) -> SourceResult<Content> {
    match expr {
        Expr::MathIdent(ident) => {
            let name = ident.get();
            // **P780** — 0. Scope local/utilizador tem prioridade absoluta
            // (paridade `get_in_math`, vanilla — medido: `#let sin = 42;
            // $sin$` mostra "42", não o operador; `#let alpha = [x];
            // $alpha$` mostra "x", não α). Nota: o léxico já garante que
            // `ident` aqui é sempre multi-grapheme — um único grapheme
            // tokeniza como `MathText`, nunca `MathIdent`
            // (`engine/lexer/math.rs`), logo não há fronteira
            // letra-única/multi-letra a replicar aqui: já vem resolvida
            // pelo lexer.
            if let Some(value) = scopes.get_local(name) {
                return Ok(super::value_to_display_content(value.clone())
                    .unwrap_or(Content::Empty));
            }
            // 1. Símbolo grego ou operador Unicode (alpha → α etc.)
            if let Some(sym) = crate::engine::math::symbols::ident_to_unicode(name) {
                return Ok(Content::MathText(sym.into()));
            }
            // 2. P301 — auto-lookup scope `math` (42 operadores P299 via SSoT MathOp).
            if let Some(op) = lookup_math_op(scopes, name) {
                return Ok(op);
            }
            // P795 — auto-lookup no módulo `sym` para símbolos bare (ex.: arrow, dif)
            if let Some(sym) = crate::engine::stdlib::sym::sym_lookup(name) {
                // P820 — símbolo depreciado (ex.: `join`): resolve com
                // warning verbatim do vanilla (`$join$` medido @1:1).
                if let Some(msg) = crate::engine::stdlib::sym::sym_deprecation(name) {
                    engine.sink.warn_note(ident.span(), msg, "");
                }
                return Ok(Content::MathText(sym.ch.to_string().into()));
            }
            // 3. **P780** — identificador realmente desconhecido: erro com
            // hints (paridade `unknown_variable_math`, vanilla). Substitui
            // o fallback pré-P780 ("manter como MathIdent" — regressão
            // pré-P301 preservada até aqui).
            Err(vec![unknown_variable_math(ident.span(), name, scopes.has_global(name))])
        }
        Expr::MathText(text) => {
            let s = match text.get() {
                MathTextKind::Grapheme(s) => s,
                MathTextKind::Number(s) => s,
            };
            Ok(Content::MathText(s.into()))
        }
        Expr::MathShorthand(sh) => Ok(Content::MathText(sh.get().to_string().into())),
        Expr::MathFrac(frac) => {
            let num = eval_math_expr(scopes, ctx, engine, frac.num())?;
            let den = eval_math_expr(scopes, ctx, engine, frac.denom())?;
            Ok(Content::math_frac(num, den))
        }
        Expr::MathAttach(attach) => {
            let base = eval_math_expr(scopes, ctx, engine, attach.base())?;
            let sub = attach
                .bottom()
                .map(|e| eval_math_expr(scopes, ctx, engine, e))
                .transpose()?;
            let sup = attach
                .top()
                .map(|e| eval_math_expr(scopes, ctx, engine, e))
                .transpose()?;

            // Primes (′ ″ ‴ ⁗) — convertidos para superscript.
            // MathPrimes::count() retorna o número de apóstrofos usando o comprimento em bytes.
            let prime_count = attach.primes().map(|p| p.count()).unwrap_or(0);
            let prime_char: Option<Content> = if prime_count == 0 {
                None
            } else {
                let s: EcoString = match prime_count {
                    1 => "′".into(),           // U+2032
                    2 => "″".into(),           // U+2033
                    3 => "‴".into(),           // U+2034
                    4 => "⁗".into(),           // U+2057
                    n => "′".repeat(n).into(), // U+2032 × n para n > 4
                };
                Some(Content::MathText(s))
            };

            // Merge prime com sup existente: primes primeiro, depois o sup original.
            let sup_final: Option<Content> = match (prime_char, sup) {
                (Some(p), None) => Some(p),
                (None, Some(s)) => Some(s),
                (Some(p), Some(s)) => {
                    Some(Content::MathSequence(std::sync::Arc::from(vec![p, s])))
                }
                (None, None) => None,
            };

            Ok(Content::math_attach(base, None, None, sub, sup_final))
        }
        Expr::MathRoot(root) => {
            // root.index() retorna Option<u8> — converter para Content::MathText se presente
            let index = root.index().map(|n| Content::MathText(n.to_string().into()));
            let radicand = eval_math_expr(scopes, ctx, engine, root.radicand())?;
            Ok(Content::math_root(index, radicand))
        }
        Expr::Math(inner) => eval_math_content(scopes, ctx, engine, inner),

        // MathDelimited: preservar estrutura para layout extensível (Passo 42)
        Expr::MathDelimited(delim) => {
            let body = eval_math_content(scopes, ctx, engine, delim.body())?;
            // Extrair o char delimitador do expr (MathText ou MathIdent com 1 char)
            let open_str = delim.open().to_untyped().text();
            let close_str = delim.close().to_untyped().text();
            let open = open_str.as_str().chars().next().unwrap_or('(');
            let close = close_str.as_str().chars().next().unwrap_or(')');
            Ok(Content::math_delimited(open, body, close))
        }

        // frac() e outras funções nativas de math (Passo 38)
        Expr::FuncCall(call) => {
            let name = match call.callee() {
                Expr::MathIdent(ident) => ident.get().to_string(),
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
                        let target =
                            eval_math_callee(scopes, ctx, engine, access.target())?;
                        if let Some(err) =
                            super::bindings::field_callee_error(&target, access)
                        {
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
                    let mut items = Vec::new();
                    let mut named: IndexMap<EcoString, Value, FxBuildHasher> =
                        IndexMap::default();
                    for arg in call.args().items() {
                        match arg {
                            Arg::Pos(expr) => {
                                items.push(eval_math_arg_value(
                                    scopes, ctx, engine, expr,
                                )?);
                            }
                            Arg::Named(name_expr) => {
                                let value = eval_math_arg_value(
                                    scopes,
                                    ctx,
                                    engine,
                                    name_expr.expr(),
                                )?;
                                named.insert(name_expr.name().as_str().into(), value);
                            }
                            Arg::Spread(_) => {}
                        }
                    }
                    let args = Args { items, named, span: call.args().span() };
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
                    let mut pos_args = call.args().items().filter_map(|arg| match arg {
                        Arg::Pos(expr) => Some(expr),
                        _ => None,
                    });
                    if let (Some(num_expr), Some(den_expr)) =
                        (pos_args.next(), pos_args.next())
                    {
                        let num = eval_math_expr(scopes, ctx, engine, num_expr)?;
                        let den = eval_math_expr(scopes, ctx, engine, den_expr)?;
                        Ok(Content::math_frac(num, den))
                    } else {
                        Ok(Content::Empty)
                    }
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
                    Ok(Content::math_root(None, radicand))
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
                            format!(
                                "root espera exactamente 2 argumentos, recebeu {}",
                                args.len()
                            ),
                        )]);
                    }
                    let index = eval_math_expr(scopes, ctx, engine, args[0])?;
                    let radicand = eval_math_expr(scopes, ctx, engine, args[1])?;
                    Ok(Content::math_root(Some(index), radicand))
                }
                // vec(...) — vector coluna (Passo 55): cada arg torna-se uma linha de uma célula.
                // Os args são planos (sem `;`), por isso não há Arrays intermediários.
                "vec" => {
                    let pos_args: Vec<Expr<'_>> = call
                        .args()
                        .items()
                        .filter_map(|a| match a {
                            Arg::Pos(e) => Some(e),
                            _ => None,
                        })
                        .collect();
                    let mut rows: Vec<Vec<Content>> = Vec::new();
                    for expr in pos_args {
                        let cell = eval_math_expr(scopes, ctx, engine, expr)?;
                        rows.push(vec![cell]);
                    }
                    Ok(Content::math_matrix(rows, ('(', ')')))
                }

                // cases(...) — função por ramos (Passo 55): args separados por vírgula.
                // `&` dentro de cada arg produz MathAlignPoint que parte as células.
                "cases" => {
                    let pos_args: Vec<Expr<'_>> = call
                        .args()
                        .items()
                        .filter_map(|a| match a {
                            Arg::Pos(e) => Some(e),
                            _ => None,
                        })
                        .collect();
                    let mut rows: Vec<Vec<Content>> = Vec::new();
                    for expr in pos_args {
                        let content = eval_math_expr(scopes, ctx, engine, expr)?;
                        let cells = match &content {
                            Content::MathSequence(items) => {
                                let mut cols: Vec<Vec<Content>> = vec![vec![]];
                                for item in items.iter() {
                                    match item {
                                        Content::MathAlignPoint(_) => cols.push(vec![]),
                                        other => {
                                            cols.last_mut().unwrap().push(other.clone())
                                        }
                                    }
                                }
                                cols.retain(|c| !c.is_empty());
                                cols.into_iter()
                                    .map(|c| Content::MathSequence(c.into()))
                                    .collect::<Vec<Content>>()
                            }
                            _ => vec![content],
                        };
                        rows.push(cells);
                    }
                    Ok(Content::math_cases(rows))
                }

                // mat(...) — matriz matemática (Passo 54)
                // O parser converte `;` em Arrays: cada Arg::Pos(Expr::Array(...)) é uma linha.
                // Sem `;`: todos os args são células de uma única linha.
                "mat" => {
                    let pos_args: Vec<Expr<'_>> = call
                        .args()
                        .items()
                        .filter_map(|a| match a {
                            Arg::Pos(e) => Some(e),
                            _ => None,
                        })
                        .collect();
                    let has_row_arrays = pos_args
                        .first()
                        .map(|e| matches!(e, Expr::Array(_)))
                        .unwrap_or(false);
                    let mut rows: Vec<Vec<Content>> = Vec::new();
                    if has_row_arrays {
                        for arg in &pos_args {
                            let mut row = Vec::new();
                            match arg {
                                Expr::Array(arr) => {
                                    for item in arr.items() {
                                        if let ArrayItem::Pos(e) = item {
                                            row.push(eval_math_expr(
                                                scopes, ctx, engine, e,
                                            )?);
                                        }
                                    }
                                }
                                other => {
                                    row.push(eval_math_expr(scopes, ctx, engine, *other)?)
                                }
                            }
                            rows.push(row);
                        }
                    } else {
                        let mut row = Vec::new();
                        for e in &pos_args {
                            row.push(eval_math_expr(scopes, ctx, engine, *e)?);
                        }
                        if !row.is_empty() {
                            rows.push(row);
                        }
                    }
                    Ok(Content::math_matrix(rows, ('(', ')')))
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
                    let maybe_func = scopes.get(&name).cloned();
                    if let Some(Value::Func(func)) = maybe_func {
                        let mut items = Vec::new();
                        let mut named: IndexMap<EcoString, Value, FxBuildHasher> =
                            IndexMap::default();
                        for arg in call.args().items() {
                            match arg {
                                Arg::Pos(expr) => {
                                    let content =
                                        eval_math_expr(scopes, ctx, engine, expr)?;
                                    items.push(Value::Content(content));
                                }
                                Arg::Named(name_expr) => {
                                    let content = eval_math_expr(
                                        scopes,
                                        ctx,
                                        engine,
                                        name_expr.expr(),
                                    )?;
                                    named.insert(
                                        name_expr.name().as_str().into(),
                                        Value::Content(content),
                                    );
                                }
                                Arg::Spread(_) => {}
                            }
                        }
                        // P772s — span da lista de argumentos da chamada real.
                        let args = Args { items, named, span: call.args().span() };
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
                                items.push(Content::MathText(", ".into()));
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

        // Ponto de alinhamento (`&`) e quebra de linha (`\\`) em equações
        Expr::MathAlignPoint(_) => Ok(Content::math_align_point()),
        Expr::Linebreak(_) => Ok(Content::linebreak()),

        // **P782** — qualquer outro `Expr` (não especificamente math) chegado
        // a uma sequência math: `#expr` (`SyntaxKind::Hash` →
        // `embedded_code_expr`, `engine/parse/math.rs:62`) e field access
        // bare (`sym.suit.heart` sem `#` — o lexer já monta um nó
        // `SyntaxKind::FieldAccess` genérico, `engine/parse/math.rs:63-64`,
        // "The lexer manages creating full FieldAccess nodes if needed")
        // produzem literalmente o **mesmo** `Expr::FieldAccess`/`Expr::Ident`
        // — sem distinção sintáctica entre os dois casos nesta arquitectura,
        // ao contrário do vanilla (`Expr::MathFieldAccess` dedicado vs
        // `Expr::FieldAccess` genérico via Hash). Paridade conceptual com
        // `ExprExt::eval_display` (vanilla) = `self.eval(vm)?.display()`:
        // delega ao avaliador genérico de código, depois converte para
        // `Content` via `value_to_display_content` (P780). Cobre `#hc`
        // (Ident), `#sym.suit.heart`/`sym.suit.heart` bare (FieldAccess),
        // `#let x = 5` (agora executa de facto — antes descartado em
        // silêncio, nunca mutava o scope) e qualquer outro literal
        // (`#5`, `#"x"`, ...). Seguro por construção: `eval_expr` já trata
        // `Expr::MathIdent`/`MathText`/etc. como "fronteira deliberada"
        // (`Ok(Value::None)`, `eval/mod.rs`) — mas esses nunca chegam aqui,
        // são apanhados pelos braços específicos acima. **Excepção**:
        // `Expr::FieldAccess` (`sym.suit.heart`, bare ou `#`) tem de passar
        // por `eval_math_callee`, não `eval_expr` directo — o alvo do
        // access (`sym`) é lexado como `Expr::MathIdent` mesmo dentro de um
        // nó `FieldAccess` (montado pelo lexer math, `engine/lexer/math.rs`
        // — comentário "The lexer manages creating full FieldAccess nodes
        // if needed"); `eval_expr` genérico trata `Expr::MathIdent` como
        // fronteira deliberada (`Value::None`), o que faria o field access
        // falhar com "field access não suportado em none" (medido, mesma
        // causa que `eval_math_callee` já contorna para o caminho de
        // callee — P772y).
        other @ Expr::FieldAccess(_) => {
            let value = eval_math_callee(scopes, ctx, engine, other)?;
            match value {
                Value::Symbol(s) => Ok(Content::MathText(s.ch.to_string().into())),
                other_val => {
                    Ok(super::value_to_display_content(other_val)
                        .unwrap_or(Content::Empty))
                }
            }
        }
        other => {
            let value = eval_expr(other, scopes, ctx, engine)?;
            Ok(super::value_to_display_content(value).unwrap_or(Content::Empty))
        }
    }
}
