//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval.md
//! @prompt-hash 3ed6811a
//! @layer L1
//! @updated 2026-04-22
//!
//! Avaliação de expressões matemáticas. Extraído de `eval.rs` no Passo 96.1
//! conforme ADR-0037 (coesão por domínio).

use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

use crate::compiler::scopes::Scopes;
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
        // **P997** — uma sub-`MathSequence` com `Linebreak` ao seu nível do
        // topo é especialada na sequência pai (os itens entram em linha, a
        // quebra sobe para o nível do run): no vanilla a quebra divide a
        // run INTEIRA (`expand_multiline_fence` devolve os
        // `RawMathItem::Linebreak` no nível do run,
        // `ir/multiline.rs:56-107`), não fica presa dentro do grupo — sem
        // isto, o conteúdo a seguir a `(n \ k)` ancorava na linha de cima.
        // Sem `Linebreak`: aninhamento preservado (inalterado).
        match node {
            Content::MathSequence(items)
                if items.iter().any(|c| matches!(c, Content::Linebreak(_))) =>
            {
                nodes.extend(items.iter().cloned());
            }
            Content::Empty => {}
            other => nodes.push(other),
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
            } else if let Some(sym) = crate::compiler::stdlib::sym::sym_lookup(name) {
                // P820 — símbolo depreciado (ex.: `join`): resolve, mas com
                // warning verbatim do vanilla (span na raiz — medido
                // `$join.r$` → warning @1:1).
                if let Some(msg) = crate::compiler::stdlib::sym::sym_deprecation(name) {
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
            // (`compiler/lexer/math.rs`), logo não há fronteira
            // letra-única/multi-letra a replicar aqui: já vem resolvida
            // pelo lexer.
            if let Some(value) = scopes.get_local(name) {
                return Ok(super::value_to_display_content(value.clone())
                    .unwrap_or(Content::Empty));
            }
            // 1. Símbolo grego ou operador Unicode (alpha → α etc.)
            if let Some(sym) = crate::compiler::math::symbols::ident_to_unicode(name) {
                return Ok(Content::MathText(sym.into()));
            }
            // 2. P301 — auto-lookup scope `math` (42 operadores P299 via SSoT MathOp).
            if let Some(op) = lookup_math_op(scopes, name) {
                return Ok(op);
            }
            // P795 — auto-lookup no módulo `sym` para símbolos bare (ex.: arrow, dif)
            if let Some(sym) = crate::compiler::stdlib::sym::sym_lookup(name) {
                // P820 — símbolo depreciado (ex.: `join`): resolve com
                // warning verbatim do vanilla (`$join$` medido @1:1).
                if let Some(msg) = crate::compiler::stdlib::sym::sym_deprecation(name) {
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
            // **P996** — `\` é quebra de linha ANTES do emparelhamento lr
            // (documentação oficial + vanilla medido em P995:
            // `expand_multiline_fence`, `ir/multiline.rs:56-107` — os
            // delimitadores dimensionam-se pelo segmento próprio, nunca
            // pela pilha). Corpo com `Linebreak` ao nível do topo NÃO
            // emparelha: os delimitadores ficam como glifos normais
            // (tamanho natural), um por linha, e o caminho de
            // quebra/grelha existente (P991) centra as linhas.
            if let Content::MathSequence(items) = &body {
                if items.iter().any(|c| matches!(c, Content::Linebreak(_))) {
                    let mut new_items: Vec<Content> =
                        Vec::with_capacity(items.len() + 2);
                    new_items.push(Content::MathText(open_str.as_str().into()));
                    new_items.extend(items.iter().cloned());
                    new_items.push(Content::MathText(close_str.as_str().into()));
                    return Ok(Content::MathSequence(std::sync::Arc::from(new_items)));
                }
            }
            Ok(Content::math_delimited(open, body, close))
        }

        // frac() e outras funções nativas de math (Passo 38)
        Expr::FuncCall(call) => {
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
                    if matches!(access.target(), Expr::MathIdent(t) if t.get() == "dot")
                        && access.field().as_str() == "double" =>
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
                    let accent_char = match name.as_str() {
                        "hat" => '\u{0302}',
                        "tilde" => '\u{0303}',
                        "dot" => '\u{0307}',
                        "dot.double" => '\u{0308}',
                        _ => unreachable!(),
                    };
                    let base = eval_math_expr(scopes, ctx, engine, pos_args[0])?;
                    Ok(Content::math_accent(base, Content::MathText(accent_char.into())))
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
                                "lr espera exactamente 1 argumento, recebeu {}",
                                pos_args.len()
                            ),
                        )]);
                    }
                    let body = eval_math_expr(scopes, ctx, engine, pos_args[0])?;
                    Ok(rewrite_lr_body(body))
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
                    Ok(Content::math_limits_override(body, false, true))
                }

                "limits" => {
                    let mut pos_args: Vec<Expr<'_>> = Vec::new();
                    let mut inline = true;
                    let mut errors: Vec<SourceDiagnostic> = Vec::new();
                    for arg in call.args().items() {
                        match arg {
                            Arg::Pos(e) => pos_args.push(e),
                            Arg::Named(n) if n.name().as_str() == "inline" => {
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
                                        match eval_math_arg_value(
                                            scopes, ctx, engine, other,
                                        ) {
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
                    Ok(Content::math_limits_override(body, true, inline))
                }

                "abs" | "norm" | "floor" | "ceil" | "round" | "bar" => {
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
                        "abs" | "bar" => ('|', '|'),
                        "norm" => ('‖', '‖'),
                        "floor" => ('⌊', '⌋'),
                        "ceil" => ('⌈', '⌉'),
                        "round" => ('⌊', '⌉'),
                        _ => unreachable!(),
                    };
                    let body = eval_math_expr(scopes, ctx, engine, pos_args[0])?;
                    Ok(Content::math_delimited(open, body, close))
                }

                // **P899 (Parte D)** — `binom(upper, lower1, lower2, ...)`:
                // no vanilla é uma fracção SEM barra (`resolve_binom` reusa
                // `resolve_vertical_frac_like`, o mesmo mecanismo de
                // `frac()`, com a barra suprimida), envolvida em parênteses
                // esticados. Cristalino não tem um modo "sem barra" para
                // `Content::math_frac` (`MathFracElem` desenha sempre a
                // barra); reaproveita-se `Content::math_matrix` — já produz
                // exactamente "pilha vertical de linhas sem barra entre
                // elas, envolvida em delimitadores esticados" (mesmo
                // mecanismo de `vec`/`cases`/`mat`). 2 linhas: `[upper]` e
                // `[lower]` — os args de `lower` juntam-se numa única
                // célula separados por `", "` (paridade vanilla: uma única
                // sequência com vírgulas entre elementos, não colunas
                // separadas de matriz).
                "binom" => {
                    let pos_args: Vec<Expr<'_>> = call
                        .args()
                        .items()
                        .filter_map(|a| match a {
                            Arg::Pos(e) => Some(e),
                            _ => None,
                        })
                        .collect();
                    if pos_args.len() < 2 {
                        return Err(vec![SourceDiagnostic::error(
                            call.span(),
                            "missing argument: lower".to_string(),
                        )]);
                    }
                    let upper = eval_math_expr(scopes, ctx, engine, pos_args[0])?;
                    let mut lower_items: Vec<Content> = Vec::new();
                    for (i, expr) in pos_args[1..].iter().enumerate() {
                        if i > 0 {
                            lower_items.push(Content::MathText(", ".into()));
                        }
                        lower_items.push(eval_math_expr(scopes, ctx, engine, *expr)?);
                    }
                    let lower = Content::MathSequence(std::sync::Arc::from(lower_items));
                    Ok(Content::math_matrix(vec![vec![upper], vec![lower]], ('(', ')')))
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
                "underbrace" | "overbrace" | "underbracket" | "overbracket" => {
                    let pos_args: Vec<Expr<'_>> = call
                        .args()
                        .items()
                        .filter_map(|a| match a {
                            Arg::Pos(e) => Some(e),
                            _ => None,
                        })
                        .collect();
                    if pos_args.is_empty() {
                        return Err(vec![SourceDiagnostic::error(
                            call.span(),
                            format!("{} espera pelo menos 1 argumento (body)", name),
                        )]);
                    }
                    let brace_char = match name.as_str() {
                        "underbrace" => '\u{23DF}',
                        "overbrace" => '\u{23DE}',
                        "underbracket" => '\u{23B5}',
                        "overbracket" => '\u{23B4}',
                        _ => unreachable!(),
                    };
                    let is_under = matches!(name.as_str(), "underbrace" | "underbracket");

                    let body = eval_math_expr(scopes, ctx, engine, pos_args[0])?;
                    let brace = Content::MathText(brace_char.into());
                    let inner = if is_under {
                        Content::math_underover(body, Some(brace), None)
                    } else {
                        Content::math_underover(body, None, Some(brace))
                    };

                    if let Some(annotation_expr) = pos_args.get(1) {
                        let annotation = eval_math_expr(scopes, ctx, engine, *annotation_expr)?;
                        Ok(if is_under {
                            Content::math_underover(inner, Some(annotation), None)
                        } else {
                            Content::math_underover(inner, None, Some(annotation))
                        })
                    } else {
                        Ok(inner)
                    }
                }

                // vec(...) — vector coluna (Passo 55): cada arg torna-se uma linha de uma célula.
                // Os args são planos (sem `;`), por isso não há Arrays intermediários.
                "vec" => {
                    let mut delim = ('(', ')');
                    let mut pos_args: Vec<Expr<'_>> = Vec::new();
                    for arg in call.args().items() {
                        match arg {
                            Arg::Pos(e) => pos_args.push(e),
                            Arg::Named(n) if n.name().as_str() == "delim" => {
                                if let Ok(val) = eval_math_arg_value(scopes, ctx, engine, n.expr()) {
                                    if let Some(d) = parse_delim_val(&val) {
                                        delim = d;
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    let mut rows: Vec<Vec<Content>> = Vec::new();
                    for expr in pos_args {
                        let cell = eval_math_expr(scopes, ctx, engine, expr)?;
                        rows.push(vec![cell]);
                    }
                    Ok(Content::math_matrix(rows, delim))
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
                    let mut delim = ('(', ')');
                    let mut pos_args: Vec<Expr<'_>> = Vec::new();
                    for arg in call.args().items() {
                        match arg {
                            Arg::Pos(e) => pos_args.push(e),
                            Arg::Named(n) if n.name().as_str() == "delim" => {
                                if let Ok(val) = eval_math_arg_value(scopes, ctx, engine, n.expr()) {
                                    if let Some(d) = parse_delim_val(&val) {
                                        delim = d;
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
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
                    Ok(Content::math_matrix(rows, delim))
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
                    } else if let Some(sym) = crate::compiler::math::symbols::ident_to_unicode(&name)
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
                        if let Some(msg) =
                            crate::compiler::stdlib::sym::sym_deprecation(&name)
                        {
                            engine
                                .sink
                                .warn_note(call.callee().span(), msg, "");
                        }
                        Content::MathText(sym.ch.to_string().into())
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
        // `embedded_code_expr`, `compiler/parse/math.rs:62`) e field access
        // bare (`sym.suit.heart` sem `#` — o lexer já monta um nó
        // `SyntaxKind::FieldAccess` genérico, `compiler/parse/math.rs:63-64`,
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
        // nó `FieldAccess` (montado pelo lexer math, `compiler/lexer/math.rs`
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

use crate::entities::math_class::MathClass;

/// **P981** — reescrita do corpo de `lr(...)`: uma `MathSequence` com
/// ≥2 itens cujo primeiro item é um delimitador de abertura (classe
/// Opening/Fence) e o último de fecho (Closing/Fence) — verificação de
/// classe do vanilla em `ir/resolve.rs:880-891` — é convertida em
/// `MathDelimited(primeiro, meio, último)`. Qualquer outro corpo é
/// devolvido inalterado (já delimitado ou sem delimitadores).
fn rewrite_lr_body(body: Content) -> Content {
    let Content::MathSequence(items) = &body else {
        return body;
    };
    if items.len() < 2 {
        return body;
    }
    let open = lr_delim_char(&items[0], &[MathClass::Opening, MathClass::Fence]);
    let close =
        lr_delim_char(&items[items.len() - 1], &[MathClass::Closing, MathClass::Fence]);
    if let (Some(o), Some(c)) = (open, close) {
        let middle: Vec<Content> = items[1..items.len() - 1].to_vec();
        let mid = match middle.len() {
            0 => Content::Empty,
            1 => middle.into_iter().next().unwrap(),
            _ => Content::MathSequence(middle.into()),
        };
        return Content::math_delimited(o, mid, c);
    }
    body
}

/// **P981** — o carácter de um item de 1 carácter se a sua classe math
/// (`entities::math_class::default_math_class`) estiver em `classes`.
fn lr_delim_char(item: &Content, classes: &[MathClass]) -> Option<char> {
    let text = match item {
        Content::MathText(s) | Content::MathIdent(s) => s.as_str(),
        _ => return None,
    };
    let mut chars = text.chars();
    let c = chars.next()?;
    if chars.next().is_some() {
        return None; // mais de um carácter — não é um delimitador solto
    }
    let class = crate::entities::math_class::default_math_class(c)?;
    classes.contains(&class).then_some(c)
}

fn parse_delim_val(val: &Value) -> Option<(char, char)> {
    match val {
        Value::Str(s) => Some(parse_delim_str(s.as_str())),
        Value::None => Some(('\0', '\0')),
        Value::Array(arr) => {
            let left = arr.get(0).and_then(parse_delim_char).unwrap_or('\0');
            let right = arr.get(1).and_then(parse_delim_char).unwrap_or(left);
            Some((left, right))
        }
        _ => None,
    }
}

fn parse_delim_char(val: &Value) -> Option<char> {
    match val {
        Value::Str(s) => s.chars().next(),
        Value::None => Some('\0'),
        _ => None,
    }
}

fn parse_delim_str(s: &str) -> (char, char) {
    match s {
        "(" | ")" => ('(', ')'),
        "[" | "]" => ('[', ']'),
        "{" | "}" => ('{', '}'),
        "|" => ('|', '|'),
        "||" | "|||" => ('‖', '‖'),
        "⌊" | "floor" => ('⌊', '⌋'),
        "⌈" | "ceil" => ('⌈', '⌉'),
        "<" | ">" | "chevron" => ('⟨', '⟩'),
        "" | "none" => ('\0', '\0'),
        other => {
            let mut chars = other.chars();
            let first = chars.next().unwrap_or('\0');
            let second = chars.next().unwrap_or(first);
            (first, second)
        }
    }
}
