//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/bindings/binding.md
//! @prompt-hash 39a27016
//! @layer L1
//! @updated 2026-08-12
//!
//! Ligações e desestruturação: `#let`, padrões de array/dict, atribuição
//! (`=`) e atribuição por desestruturação.
//!
//! Extraído de `compiler/eval/bindings.rs` no Passo 1013 conforme ADR-0109
//! (atomização — forma B, free function no arquivo da unidade).

use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

use crate::compiler::scopes::Scopes;
use crate::entities::ast::code::{DestructAssignment, LetBinding, LetBindingKind};
use crate::entities::ast::expr::{
    BinOp, Binary, Destructuring, DestructuringItem, Expr, Pattern,
};
use crate::entities::ast::AstNode;
use crate::entities::engine::Engine;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::value::Value;

use crate::compiler::eval::{eval_expr, EvalContext};

use super::access::{access, access_dict};

pub(in crate::compiler::eval) fn eval_let(
    binding: LetBinding<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let mut value = match binding.init() {
        Some(init) => eval_expr(init, scopes, ctx, engine)?,
        None => Value::None,
    };

    match binding.kind() {
        LetBindingKind::Normal(pattern) => {
            // P715 — nomeação pós-hoc só faz sentido para o caso simples (um
            // único ident): permite recursão em `#let f = (n) => ...`. Padrões
            // de desestruturação não têm um único "nome" candidato — o
            // vanilla também não faz esta nomeação em `destructure()`
            // (`typst-eval/binding.rs:45-57`), é uma extensão só do caso
            // simples, preservada tal como estava.
            if let Pattern::Normal(Expr::Ident(ident)) = pattern {
                if let Value::Func(ref mut func) = value {
                    func.set_name(ident.as_str().to_string());
                }
            }
            destructure_let(pattern, value, scopes, ctx, engine)?;
        }
        LetBindingKind::Closure(ident) => {
            // Sintaxe function shorthand: #let fib(n) = ...
            // O nó Closure já carrega o nome — apenas definir no scope.
            // set_name() não é necessário: o nome vem de closure_expr.name()
            // no arm Expr::Closure (ver eval_expr).
            let name = ident.as_str().to_string();
            scopes.define(name, value);
        }
    }

    Ok(Value::None)
}

/// **P715** — desestruturação genérica de `Pattern` sobre um `Value`,
/// aplicando `f` a cada folha (par `Expr` alvo + valor). Mirror exacto de
/// `destructure_impl` do vanilla (`typst-eval/binding.rs:60-82`) — a única
/// diferença entre `let` e atribuição (`(a,b) = expr`) é o que `f` faz com
/// cada folha (`destructure_let` define; `eval_destruct_assignment` muta).
/// **P716** — `ctx`/`engine` enfiados até à folha: a folha da atribuição
/// usa o `Access` genérico, que avalia args de accessor methods (o vanilla
/// passa o `Vm` inteiro; aqui as três partes).
fn destructure_pattern<F>(
    pattern: Pattern<'_>,
    value: Value,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    f: &F,
) -> SourceResult<()>
where
    F: Fn(
        &mut Scopes<'_>,
        &mut EvalContext,
        &mut Engine<'_>,
        Expr<'_>,
        Value,
    ) -> SourceResult<()>,
{
    match pattern {
        Pattern::Normal(expr) => f(scopes, ctx, engine, expr, value)?,
        Pattern::Placeholder(_) => {}
        Pattern::Parenthesized(p) => {
            destructure_pattern(p.pattern(), value, scopes, ctx, engine, f)?
        }
        Pattern::Destructuring(d) => match value {
            Value::Array(arr) => destructure_array(d, arr, scopes, ctx, engine, f)?,
            Value::Dict(dict) => destructure_dict(d, dict, scopes, ctx, engine, f)?,
            other => {
                return Err(vec![SourceDiagnostic::error(
                    pattern.span(),
                    format!("cannot destructure {}", other.type_name()),
                )]);
            }
        },
    }
    Ok(())
}

/// **P715** — desestruturação de array. Mirror de `destructure_array` do
/// vanilla (`typst-eval/binding.rs:84-127`): posicionais consomem 1 elemento
/// cada; `..sink` absorve o resto (tamanho calculado, não iterativo);
/// `Named` não é permitido (só faz sentido contra `dict`).
fn destructure_array<F>(
    d: Destructuring<'_>,
    arr: Vec<Value>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    f: &F,
) -> SourceResult<()>
where
    F: Fn(
        &mut Scopes<'_>,
        &mut EvalContext,
        &mut Engine<'_>,
        Expr<'_>,
        Value,
    ) -> SourceResult<()>,
{
    let len = arr.len();
    let items: Vec<_> = d.items().collect();
    let item_count = items.len();
    let mut i = 0usize;

    for item in items {
        match item {
            DestructuringItem::Pattern(pattern) => {
                if i >= len {
                    return Err(vec![wrong_number_of_elements(d, len)]);
                }
                destructure_pattern(pattern, arr[i].clone(), scopes, ctx, engine, f)?;
                i += 1;
            }
            DestructuringItem::Spread(spread) => {
                let sink_size = (1 + len).checked_sub(item_count);
                let Some(sink_size) = sink_size else {
                    return Err(vec![wrong_number_of_elements(d, len)]);
                };
                if i + sink_size > len {
                    return Err(vec![wrong_number_of_elements(d, len)]);
                }
                if let Some(expr) = spread.sink_expr() {
                    f(
                        scopes,
                        ctx,
                        engine,
                        expr,
                        Value::Array(arr[i..i + sink_size].to_vec()),
                    )?;
                }
                i += sink_size;
            }
            DestructuringItem::Named(named) => {
                return Err(vec![SourceDiagnostic::error(
                    named.span(),
                    "cannot destructure named pattern from an array".to_string(),
                )]);
            }
        }
    }

    if i < len {
        return Err(vec![wrong_number_of_elements(d, len)]);
    }
    Ok(())
}

/// **P715** — desestruturação de dict. Mirror de `destructure_dict` do
/// vanilla (`typst-eval/binding.rs:129-175`): ident nu é atalho para
/// `key: key` (chave = nome do ident); `Named` renomeia (`key: pattern`,
/// suporta padrão aninhado); `..sink` recolhe as chaves não usadas num novo
/// dict; padrão posicional nu (não-ident) é erro (só faz sentido contra
/// `array`).
fn destructure_dict<F>(
    d: Destructuring<'_>,
    dict: IndexMap<EcoString, Value, FxBuildHasher>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    f: &F,
) -> SourceResult<()>
where
    F: Fn(
        &mut Scopes<'_>,
        &mut EvalContext,
        &mut Engine<'_>,
        Expr<'_>,
        Value,
    ) -> SourceResult<()>,
{
    let mut sink: Option<Expr<'_>> = None;
    let mut used: std::collections::HashSet<EcoString> = std::collections::HashSet::new();

    for item in d.items() {
        match item {
            DestructuringItem::Pattern(Pattern::Normal(Expr::Ident(ident))) => {
                let key = ident.as_str();
                let v = dict.get(key).cloned().ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        ident.span(),
                        format!("dictionary does not contain key {:?}", key),
                    )]
                })?;
                f(scopes, ctx, engine, Expr::Ident(ident), v)?;
                used.insert(EcoString::from(key));
            }
            DestructuringItem::Named(named) => {
                let name = named.name();
                let key = name.as_str();
                let v = dict.get(key).cloned().ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        name.span(),
                        format!("dictionary does not contain key {:?}", key),
                    )]
                })?;
                destructure_pattern(named.pattern(), v, scopes, ctx, engine, f)?;
                used.insert(EcoString::from(key));
            }
            DestructuringItem::Spread(spread) => {
                sink = spread.sink_expr();
            }
            DestructuringItem::Pattern(other) => {
                return Err(vec![SourceDiagnostic::error(
                    other.span(),
                    "cannot destructure unnamed pattern from dictionary".to_string(),
                )]);
            }
        }
    }

    if let Some(expr) = sink {
        let mut sink_dict: IndexMap<EcoString, Value, FxBuildHasher> =
            IndexMap::default();
        for (key, value) in dict {
            if !used.contains(&key) {
                sink_dict.insert(key, value);
            }
        }
        f(scopes, ctx, engine, expr, Value::Dict(sink_dict))?;
    }

    Ok(())
}

/// A mensagem exacta de erro de aridade do vanilla (`typst-eval/binding.rs:180-209`).
fn wrong_number_of_elements(d: Destructuring<'_>, len: usize) -> SourceDiagnostic {
    let mut count = 0;
    let mut spread = false;
    for item in d.items() {
        match item {
            DestructuringItem::Pattern(_) => count += 1,
            DestructuringItem::Spread(_) => spread = true,
            DestructuringItem::Named(_) => {}
        }
    }

    let quantifier = if len > count { "too many" } else { "not enough" };
    let expected = if spread {
        if count == 1 {
            "at least 1 element".to_string()
        } else {
            format!("at least {count} elements")
        }
    } else {
        match count {
            0 => "an empty array".to_string(),
            1 => "a single element".to_string(),
            c => format!("{c} elements"),
        }
    };

    SourceDiagnostic::error(d.span(), format!("{quantifier} elements to destructure"))
        .with_hint(format!(
            "the provided array has a length of {len}, but the pattern expects {expected}"
        ))
}

/// **P715** — desestruturação para `#let`: cada folha tem de ser um `Ident`
/// (define no scope actual). Mirror de `destructure()` do vanilla
/// (`typst-eval/binding.rs:45-57`).
/// **P723** — `pub(super)`: reaproveitada por `run_for_loop`
/// (`control_flow.rs`), que passa a delegar o binding de cada item aqui
/// (spread `..sink`, mensagens de aridade do vanilla).
pub(in crate::compiler::eval) fn destructure_let(
    pattern: Pattern<'_>,
    value: Value,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<()> {
    destructure_pattern(
        pattern,
        value,
        scopes,
        ctx,
        engine,
        &|scopes, _ctx, _engine, expr, value| match expr {
            Expr::Ident(ident) => {
                scopes.define(ident.as_str(), value);
                Ok(())
            }
            other => Err(vec![SourceDiagnostic::error(
                other.span(),
                "cannot assign to this expression".to_string(),
            )]),
        },
    )
}

/// **P715** — `(a, b) = expr`: desestruturação em atribuição. Cada folha tem
/// de já existir; não cria bindings novos. Mirror de
/// `DestructAssignment::eval` do vanilla (`typst-eval/binding.rs:30-42`):
/// a folha escreve via `Access` genérico (**P716** — suporta `Ident`,
/// `FieldAccess` e accessor methods; **sem** o caso especial de insert do
/// `=` puro, tal como o vanilla — `(d.novo,) = (2,)` erra missing key).
pub(in crate::compiler::eval) fn eval_destruct_assignment(
    node: DestructAssignment<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let value = eval_expr(node.value(), scopes, ctx, engine)?;
    destructure_pattern(
        node.pattern(),
        value,
        scopes,
        ctx,
        engine,
        &|scopes, ctx, engine, expr, value| {
            *access(expr, scopes, ctx, engine)? = value;
            Ok(())
        },
    )?;
    Ok(Value::None)
}

/// **P715/P716** — atribuição simples/composta (`x = v`, `+=`, `-=`, `*=`,
/// `/=`). Mirror de `apply_assignment` do vanilla (`typst-eval/ops.rs:69-91`):
/// **rhs primeiro**; `=` puro com lhs `FieldAccess` é o caso especial que
/// **cria** a chave (`access_dict` + `insert`, `ops.rs:77-85`); tudo o resto
/// resolve o lhs como **local** via `Access` genérico e escreve no sítio
/// (`mem::replace` para ler o valor actual nas formas compostas). A
/// intercepção antes do dispatch genérico em `eval_expr` (`eval/mod.rs`)
/// continua necessária: o lhs não pode ser avaliado como valor.
pub(in crate::compiler::eval) fn eval_assign(
    binary: Binary<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let rhs = eval_expr(binary.rhs(), scopes, ctx, engine)?;

    // Caso especial (vanilla ops.rs:77-85): atribuição pura a um campo de
    // dict pode CRIAR o campo — não passa pelo `at_mut` do campo.
    if matches!(binary.op(), BinOp::Assign) {
        if let Expr::FieldAccess(fa) = binary.lhs() {
            let field: EcoString = fa.field().as_str().into();
            let dict = access_dict(fa, scopes, ctx, engine)?;
            dict.insert(field, rhs);
            return Ok(Value::None);
        }
    }

    let location = access(binary.lhs(), scopes, ctx, engine)?;
    let new_value = match binary.op() {
        BinOp::Assign => rhs,
        op => {
            let current = std::mem::replace(location, Value::None);
            let underlying = match op {
                BinOp::AddAssign => BinOp::Add,
                BinOp::SubAssign => BinOp::Sub,
                BinOp::MulAssign => BinOp::Mul,
                BinOp::DivAssign => BinOp::Div,
                _ => unreachable!("filtrado por eval_expr antes de chamar eval_assign"),
            };
            crate::compiler::eval::operators::eval_binary_op(underlying, current, rhs)
                .map_err(|msg| vec![SourceDiagnostic::error(binary.span(), msg)])?
        }
    };
    *location = new_value;
    Ok(Value::None)
}

// ── P716 — `Access` genérico: mirror de `typst-eval/access.rs` e
// `methods.rs` (accessor methods). Referência mutável ao LOCAL nomeado
// pela expressão (elemento/campo), não substituição do valor completo. ────
