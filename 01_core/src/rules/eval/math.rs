//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/eval.md
//! @prompt-hash a8523b4b
//! @layer L1
//! @updated 2026-04-22
//!
//! Avaliação de expressões matemáticas. Extraído de `eval.rs` no Passo 96.1
//! conforme ADR-0037 (coesão por domínio).

use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

use crate::entities::args::Args;
use crate::entities::ast::expr::{Arg, ArrayItem, Expr};
use crate::entities::ast::math::{Math, MathTextKind};
use crate::entities::ast::AstNode;
use crate::entities::content::Content;
use crate::entities::engine::Engine;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::value::Value;
use crate::rules::scopes::Scopes;

use super::{apply_func, EvalContext};

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
/// não é `Content::MathOp`).
fn lookup_math_op(scopes: &Scopes<'_>, name: &str) -> Option<Content> {
    let Value::Dict(math_module) = scopes.get("math")? else {
        return None;
    };
    let Value::Content(c) = math_module.get(name)? else {
        return None;
    };
    if matches!(c, Content::MathOp { .. }) {
        Some(c.clone())
    } else {
        None
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
            // 1. Símbolo grego ou operador Unicode (alpha → α etc.)
            if let Some(sym) = crate::rules::math::symbols::ident_to_unicode(name) {
                return Ok(Content::MathText(sym.into()));
            }
            // 2. P301 — auto-lookup scope `math` (42 operadores P299 via SSoT MathOp).
            if let Some(op) = lookup_math_op(scopes, name) {
                return Ok(op);
            }
            // 3. Fallback: variável, função, ou identificador desconhecido
            //    — manter como MathIdent (regressão pré-P301 preservada).
            Ok(Content::MathIdent(name.into()))
        }
        Expr::MathText(text) => {
            let s = match text.get() {
                MathTextKind::Grapheme(s) => s,
                MathTextKind::Number(s)   => s,
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
            let sub  = attach.bottom()
                .map(|e| eval_math_expr(scopes, ctx, engine, e))
                .transpose()?;
            let sup  = attach.top()
                .map(|e| eval_math_expr(scopes, ctx, engine, e))
                .transpose()?;

            // Primes (′ ″ ‴ ⁗) — convertidos para superscript.
            // MathPrimes::count() retorna o número de apóstrofos usando o comprimento em bytes.
            let prime_count = attach.primes()
                .map(|p| p.count())
                .unwrap_or(0);
            let prime_char: Option<Content> = if prime_count == 0 {
                None
            } else {
                let s: EcoString = match prime_count {
                    1 => "′".into(),          // U+2032
                    2 => "″".into(),          // U+2033
                    3 => "‴".into(),          // U+2034
                    4 => "⁗".into(),          // U+2057
                    n => "′".repeat(n).into(), // U+2032 × n para n > 4
                };
                Some(Content::MathText(s))
            };

            // Merge prime com sup existente: primes primeiro, depois o sup original.
            let sup_final: Option<Content> = match (prime_char, sup) {
                (Some(p), None)    => Some(p),
                (None,    Some(s)) => Some(s),
                (Some(p), Some(s)) => Some(Content::MathSequence(
                    std::sync::Arc::from(vec![p, s])
                )),
                (None,    None)    => None,
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
            let open_str  = delim.open().to_untyped().text();
            let close_str = delim.close().to_untyped().text();
            let open  = open_str.as_str().chars().next().unwrap_or('(');
            let close = close_str.as_str().chars().next().unwrap_or(')');
            Ok(Content::math_delimited(open, body, close))
        }

        // frac() e outras funções nativas de math (Passo 38)
        Expr::FuncCall(call) => {
            let name = match call.callee() {
                Expr::MathIdent(ident) => ident.get().to_string(),
                _ => return Ok(Content::Empty),
            };
            match name.as_str() {
                "frac" => {
                    let mut pos_args = call.args().items().filter_map(|arg| match arg {
                        Arg::Pos(expr) => Some(expr),
                        _ => None,
                    });
                    if let (Some(num_expr), Some(den_expr)) = (pos_args.next(), pos_args.next()) {
                        let num = eval_math_expr(scopes, ctx, engine, num_expr)?;
                        let den = eval_math_expr(scopes, ctx, engine, den_expr)?;
                        Ok(Content::math_frac(num, den))
                    } else {
                        Ok(Content::Empty)
                    }
                }
                // sqrt(x) — 1 argumento posicional → Content::MathRoot { index: None }
                "sqrt" => {
                    let args: Vec<_> = call.args().items().filter_map(|a| match a {
                        Arg::Pos(e) => Some(e),
                        _ => None,
                    }).collect();
                    if args.len() != 1 {
                        return Err(vec![SourceDiagnostic::error(
                            call.span(),
                            format!("sqrt espera exactamente 1 argumento, recebeu {}", args.len()),
                        )]);
                    }
                    let radicand = eval_math_expr(scopes, ctx, engine, args[0])?;
                    Ok(Content::math_root(None, radicand))
                }
                // root(n, x) — 2 argumentos posicionais: índice, radicando
                "root" => {
                    let args: Vec<_> = call.args().items().filter_map(|a| match a {
                        Arg::Pos(e) => Some(e),
                        _ => None,
                    }).collect();
                    if args.len() != 2 {
                        return Err(vec![SourceDiagnostic::error(
                            call.span(),
                            format!("root espera exactamente 2 argumentos, recebeu {}", args.len()),
                        )]);
                    }
                    let index    = eval_math_expr(scopes, ctx, engine, args[0])?;
                    let radicand = eval_math_expr(scopes, ctx, engine, args[1])?;
                    Ok(Content::math_root(Some(index), radicand))
                }
                // vec(...) — vector coluna (Passo 55): cada arg torna-se uma linha de uma célula.
                // Os args são planos (sem `;`), por isso não há Arrays intermediários.
                "vec" => {
                    let pos_args: Vec<Expr<'_>> = call.args().items()
                        .filter_map(|a| match a { Arg::Pos(e) => Some(e), _ => None })
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
                    let pos_args: Vec<Expr<'_>> = call.args().items()
                        .filter_map(|a| match a { Arg::Pos(e) => Some(e), _ => None })
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
                                        other => cols.last_mut().unwrap().push(other.clone()),
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
                    let pos_args: Vec<Expr<'_>> = call.args().items()
                        .filter_map(|a| match a { Arg::Pos(e) => Some(e), _ => None })
                        .collect();
                    let has_row_arrays = pos_args.first()
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
                                            row.push(eval_math_expr(scopes, ctx, engine, e)?);
                                        }
                                    }
                                }
                                other => row.push(eval_math_expr(scopes, ctx, engine, *other)?),
                            }
                            rows.push(row);
                        }
                    } else {
                        let mut row = Vec::new();
                        for e in &pos_args {
                            row.push(eval_math_expr(scopes, ctx, engine, *e)?);
                        }
                        if !row.is_empty() { rows.push(row); }
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
                                    let content = eval_math_expr(scopes, ctx, engine, expr)?;
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
                        let args = Args { items, named };
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

                    let pos_args: Vec<Expr<'_>> = call.args().items()
                        .filter_map(|a| match a { Arg::Pos(e) => Some(e), _ => None })
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
        Expr::Linebreak(_)      => Ok(Content::linebreak()),

        // Primes e outros nós não implementados → placeholder vazio
        _ => Ok(Content::Empty),
    }
}
