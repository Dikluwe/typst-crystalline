//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/eval.md
//! @prompt-hash 19073424
//! @layer L1
//! @updated 2026-04-22
//!
//! Avaliação de expressões matemáticas. Extraído de `eval.rs` no Passo 96.1
//! conforme ADR-0037 (coesão por domínio).

use ecow::EcoString;

use crate::entities::ast::expr::{Arg, ArrayItem, Expr};
use crate::entities::ast::math::{Math, MathTextKind};
use crate::entities::ast::AstNode;
use crate::entities::content::Content;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::rules::scopes::Scopes;

use super::EvalContext;

/// Avalia o corpo de uma equação matemática — produz `Content` a partir de `Math<'_>`.
///
/// Stub intencional (Passo 34): produz a estrutura de nós correcta sem motor de
/// renderização. O motor real (Passo 36+) substitui esta função com layout tipográfico.
pub(super) fn eval_math_content(
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext<'_>,
    math: Math<'_>,
) -> SourceResult<Content> {
    let mut nodes: Vec<Content> = Vec::new();
    for expr in math.exprs() {
        let node = eval_math_expr(scopes, ctx, expr)?;
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
    ctx: &mut EvalContext<'_>,
    expr: Expr<'_>,
) -> SourceResult<Content> {
    match expr {
        Expr::MathIdent(ident) => {
            let name = ident.get();
            if let Some(sym) = crate::rules::math::symbols::ident_to_unicode(name) {
                // Símbolo grego ou operador: converter para Unicode
                Ok(Content::MathText(sym.into()))
            } else {
                // Variável, função, ou identificador desconhecido — manter como MathIdent
                Ok(Content::MathIdent(name.into()))
            }
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
            let num = eval_math_expr(scopes, ctx, frac.num())?;
            let den = eval_math_expr(scopes, ctx, frac.denom())?;
            Ok(Content::MathFrac { num: Box::new(num), den: Box::new(den) })
        }
        Expr::MathAttach(attach) => {
            let base = eval_math_expr(scopes, ctx, attach.base())?;
            let sub  = attach.bottom()
                .map(|e| eval_math_expr(scopes, ctx, e))
                .transpose()?
                .map(Box::new);
            let sup  = attach.top()
                .map(|e| eval_math_expr(scopes, ctx, e))
                .transpose()?
                .map(Box::new);

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
            let sup_final: Option<Box<Content>> = match (prime_char, sup) {
                (Some(p), None)    => Some(Box::new(p)),
                (None,    Some(s)) => Some(s),
                (Some(p), Some(s)) => Some(Box::new(Content::MathSequence(
                    std::sync::Arc::from(vec![p, *s])
                ))),
                (None,    None)    => None,
            };

            Ok(Content::MathAttach { base: Box::new(base), tl: None, bl: None, sub, sup: sup_final })
        }
        Expr::MathRoot(root) => {
            // root.index() retorna Option<u8> — converter para Content::MathText se presente
            let index = root.index().map(|n| Box::new(Content::MathText(n.to_string().into())));
            let radicand = eval_math_expr(scopes, ctx, root.radicand())?;
            Ok(Content::MathRoot { index, radicand: Box::new(radicand) })
        }
        Expr::Math(inner) => eval_math_content(scopes, ctx, inner),

        // MathDelimited: preservar estrutura para layout extensível (Passo 42)
        Expr::MathDelimited(delim) => {
            let body = eval_math_content(scopes, ctx, delim.body())?;
            // Extrair o char delimitador do expr (MathText ou MathIdent com 1 char)
            let open_str  = delim.open().to_untyped().text();
            let close_str = delim.close().to_untyped().text();
            let open  = open_str.as_str().chars().next().unwrap_or('(');
            let close = close_str.as_str().chars().next().unwrap_or(')');
            Ok(Content::MathDelimited { open, body: Box::new(body), close })
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
                        let num = eval_math_expr(scopes, ctx, num_expr)?;
                        let den = eval_math_expr(scopes, ctx, den_expr)?;
                        Ok(Content::MathFrac { num: Box::new(num), den: Box::new(den) })
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
                    let radicand = eval_math_expr(scopes, ctx, args[0])?;
                    Ok(Content::MathRoot { index: None, radicand: Box::new(radicand) })
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
                    let index    = eval_math_expr(scopes, ctx, args[0])?;
                    let radicand = eval_math_expr(scopes, ctx, args[1])?;
                    Ok(Content::MathRoot { index: Some(Box::new(index)), radicand: Box::new(radicand) })
                }
                // vec(...) — vector coluna (Passo 55): cada arg torna-se uma linha de uma célula.
                // Os args são planos (sem `;`), por isso não há Arrays intermediários.
                "vec" => {
                    let pos_args: Vec<Expr<'_>> = call.args().items()
                        .filter_map(|a| match a { Arg::Pos(e) => Some(e), _ => None })
                        .collect();
                    let mut rows: Vec<Vec<Content>> = Vec::new();
                    for expr in pos_args {
                        let cell = eval_math_expr(scopes, ctx, expr)?;
                        rows.push(vec![cell]);
                    }
                    Ok(Content::MathMatrix { rows, delim: ('(', ')') })
                }

                // cases(...) — função por ramos (Passo 55): args separados por vírgula.
                // `&` dentro de cada arg produz MathAlignPoint que parte as células.
                "cases" => {
                    let pos_args: Vec<Expr<'_>> = call.args().items()
                        .filter_map(|a| match a { Arg::Pos(e) => Some(e), _ => None })
                        .collect();
                    let mut rows: Vec<Vec<Content>> = Vec::new();
                    for expr in pos_args {
                        let content = eval_math_expr(scopes, ctx, expr)?;
                        let cells = match &content {
                            Content::MathSequence(items) => {
                                let mut cols: Vec<Vec<Content>> = vec![vec![]];
                                for item in items.iter() {
                                    match item {
                                        Content::MathAlignPoint => cols.push(vec![]),
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
                    Ok(Content::MathCases { rows })
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
                                            row.push(eval_math_expr(scopes, ctx, e)?);
                                        }
                                    }
                                }
                                other => row.push(eval_math_expr(scopes, ctx, *other)?),
                            }
                            rows.push(row);
                        }
                    } else {
                        let mut row = Vec::new();
                        for e in &pos_args {
                            row.push(eval_math_expr(scopes, ctx, *e)?);
                        }
                        if !row.is_empty() { rows.push(row); }
                    }
                    Ok(Content::MathMatrix { rows, delim: ('(', ')') })
                }

                // Outros nomes: tratar como MathIdent (sin, cos, lim, …)
                _ => Ok(Content::MathIdent(name.into())),
            }
        }

        // Ponto de alinhamento (`&`) e quebra de linha (`\\`) em equações
        Expr::MathAlignPoint(_) => Ok(Content::MathAlignPoint),
        Expr::Linebreak(_)      => Ok(Content::Linebreak),

        // Primes e outros nós não implementados → placeholder vazio
        _ => Ok(Content::Empty),
    }
}
