//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/eval.md
//! @prompt-hash 96691e96
//! @layer L1
//! @updated 2026-07-20
//!
//! Bindings: `#let`, `#show` counter, e field access (P411 — Version; P412 — Duration). Extraído de `eval.rs` no Passo 96.1
//! conforme ADR-0037 (coesão por domínio). Assinaturas simplificadas no
//! Passo 109 (ADR-0044) via `Engine<'_>`.

use ecow::{EcoString, EcoVec};
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

use crate::engine::scopes::Scopes;
use crate::engine::stdlib::counter::{
    counter_at, counter_at_location, counter_display, counter_final, counter_get, counter_step,
    counter_update,
};
use crate::engine::stdlib::native_str_from_unicode;
use crate::engine::stdlib::state::{
    state_at_location, state_display, state_final, state_get, state_update,
};
use crate::entities::args::Args;
use crate::entities::ast::code::{DestructAssignment, LetBinding, LetBindingKind};
use crate::entities::ast::expr::{
    Arg, BinOp, Binary, Destructuring, DestructuringItem, Expr, Pattern,
};
use crate::entities::ast::AstNode;
use crate::entities::content::Content;
use crate::entities::counter::Counter;
use crate::entities::counter_update::CounterUpdate as CounterAction;
use crate::entities::element_kind::ElementKind;
use crate::entities::engine::Engine;
use crate::entities::func::Func;
use crate::entities::selector::Selector;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::state::State;
use crate::entities::value::{Type, Value};

use super::{eval_expr, EvalContext};

pub(super) fn eval_let(
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
pub(super) fn destructure_let(
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
pub(super) fn eval_destruct_assignment(
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
pub(super) fn eval_assign(
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
            super::operators::eval_binary_op(underlying, current, rhs)
                .map_err(|msg| vec![SourceDiagnostic::error(binary.span(), msg)])?
        }
    };
    *location = new_value;
    Ok(Value::None)
}

// ── P716 — `Access` genérico: mirror de `typst-eval/access.rs` e
// `methods.rs` (accessor methods). Referência mutável ao LOCAL nomeado
// pela expressão (elemento/campo), não substituição do valor completo. ────

/// **P716** — nome longo do tipo, como o vanilla o escreve nas mensagens de
/// erro do `Access` ("integer does not have accessible fields") — a mensagem
/// é o observável (ADR-0107). Só difere do `type_name()` curto nos escalares.
/// **P814** — promovido a `pub(crate)`: reutilizado por `stdlib/eval.rs`
/// para as mensagens de cast do vanilla ("expected string, found integer").
pub(crate) fn long_type_name(value: &Value) -> &'static str {
    match value {
        Value::Int(_) => "integer",
        Value::Str(_) => "string",
        Value::Bool(_) => "boolean",
        other => other.type_name(),
    }
}

/// **P716** — a lista exacta (e completa) de accessor methods do vanilla
/// (`typst-eval/methods.rs:19-21`): `first`, `last`, `at`. Não há outros.
fn is_accessor_method(method: &str) -> bool {
    matches!(method, "first" | "last" | "at")
}

/// **P716** — erro de chave ausente do vanilla (`Dict::at_mut`,
/// `foundations/dict.rs:99-104`), com o hint.
fn missing_key(span: Span, key: &str) -> Vec<SourceDiagnostic> {
    vec![SourceDiagnostic::error(
        span,
        format!("dictionary does not contain key {key:?}"),
    )
    .with_hint("use `insert` to add or update values".to_string())]
}

/// **P772r** — erro de variável desconhecida, paridade vanilla
/// `foundations/scope.rs::unknown_variable` (linha 424-437). Usado tanto
/// para leitura (`eval_expr`, `Expr::Ident`, `eval/mod.rs`) como para
/// mutação (`access`, abaixo) — mesma função no vanilla para os dois casos.
///
/// Heurística do hint (confirmada por sonda, não assumida): qualquer
/// hífen no nome (`name.contains('-')`) — sem verificar se as partes ao
/// redor são identificadores conhecidos. Plural ("signs") quando há mais
/// de um hífen; singular ("sign") para um só. Sem hífen → sem hint.
pub(super) fn unknown_variable(span: Span, name: &str) -> SourceDiagnostic {
    let diag = SourceDiagnostic::error(span, format!("unknown variable: {name}"));
    if name.contains('-') {
        let plural = if name.matches('-').count() > 1 { "s" } else { "" };
        diag.with_hint(format!(
            "if you meant to use subtraction, try adding spaces around the minus sign{plural}: `{}`",
            name.replace('-', " - ")
        ))
    } else {
        diag
    }
}

/// **P716** — mirror do trait `Access` do vanilla (`access.rs:14-27`), como
/// free function (o cristalino não tem `Vm`; recebe as três partes). Quatro
/// formas de alvo: `Ident`, `Parenthesized`, `FieldAccess`, `FuncCall` de
/// accessor method. Qualquer outra expressão avalia (para efeitos) e erra
/// "cannot mutate a temporary value" (`access.rs:21-24,71-72`).
fn access<'s>(
    expr: Expr<'_>,
    scopes: &'s mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<&'s mut Value> {
    match expr {
        Expr::Ident(ident) => {
            let name = ident.as_str();
            if scopes.get_mut(name).is_none() {
                // P772q — `name` só existe em `captured` (closure/`context`)
                // → mensagem específica por `Capturer`, paridade vanilla
                // `Binding::write()` (`foundations/scope.rs:316-323`);
                // verificado antes de `is_constant` (P772n): paridade
                // vanilla onde `get_mut` bem sucedido sobre um binding
                // `Captured` falha em `.write()`, sem chegar a `base`.
                // Ver `rules/eval.md` §P772q, §P772n, §P772r.
                let diag = if let Some(capturer) = scopes.captured_by(name) {
                    let origin = match capturer {
                        crate::entities::scope::Capturer::Function => "function",
                        crate::entities::scope::Capturer::Context => "context expression",
                    };
                    SourceDiagnostic::error(
                        ident.span(),
                        format!(
                            "variables from outside the {origin} are read-only and cannot be modified"
                        ),
                    )
                } else if scopes.is_constant(name) {
                    SourceDiagnostic::error(
                        ident.span(),
                        format!("cannot mutate a constant: {name}"),
                    )
                } else {
                    unknown_variable(ident.span(), name)
                };
                return Err(vec![diag]);
            }
            match scopes.get_mut(name) {
                Some(slot) => Ok(slot),
                None => unreachable!("verificado acima"),
            }
        }
        Expr::Parenthesized(paren) => access(paren.expr(), scopes, ctx, engine),
        Expr::FieldAccess(fa) => {
            let span = fa.span();
            let field: EcoString = fa.field().as_str().into();
            let dict = access_dict(fa, scopes, ctx, engine)?;
            match dict.get_mut(field.as_str()) {
                Some(slot) => Ok(slot),
                None => Err(missing_key(span, field.as_str())),
            }
        }
        Expr::FuncCall(call) => {
            if let Expr::FieldAccess(fa) = call.callee() {
                if is_accessor_method(fa.field().as_str()) {
                    let span = call.span();
                    let method: EcoString = fa.field().as_str().into();
                    // Ordem do vanilla (`access.rs:62-64`): args primeiro,
                    // access do target depois.
                    let args =
                        super::closures::eval_args(call.args(), scopes, ctx, engine)?;
                    let target = access(fa.target(), scopes, ctx, engine)?;
                    return call_method_access(target, method.as_str(), args, span);
                }
            }
            let _ = eval_expr(expr, scopes, ctx, engine)?;
            Err(vec![SourceDiagnostic::error(
                expr.span(),
                "cannot mutate a temporary value".to_string(),
            )])
        }
        other => {
            let _ = eval_expr(other, scopes, ctx, engine)?;
            Err(vec![SourceDiagnostic::error(
                other.span(),
                "cannot mutate a temporary value".to_string(),
            )])
        }
    }
}

/// **P716** — mirror de `access_dict` do vanilla (`access.rs:76-107`):
/// resolve o target de um `FieldAccess` como dict mutável. Os três níveis
/// de erro para não-dict seguem o vanilla: tipos com field getters próprios
/// → "cannot mutate fields on {ty}"; tipos em `fields_on`
/// (`fields.rs:77-91`: Version, Length, Rel, Stroke, Alignment) → "fields on
/// {ty} are not yet mutable" + hint; resto → "{ty} does not have accessible
/// fields". Duration cristalino (campos de leitura, P412) fica no último
/// braço — o vanilla não o tem em `fields_on` e a mensagem é o observável.
fn access_dict<'s>(
    fa: crate::entities::ast::expr::FieldAccess<'_>,
    scopes: &'s mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<&'s mut IndexMap<EcoString, Value, FxBuildHasher>> {
    let target_span = fa.target().span();
    match access(fa.target(), scopes, ctx, engine)? {
        Value::Dict(dict) => Ok(dict),
        value => {
            let ty = long_type_name(value);
            match value {
                Value::Symbol(_)
                | Value::Content(_)
                | Value::Module(_)
                | Value::Func(_)
                | Value::Args(_) => Err(vec![SourceDiagnostic::error(
                    target_span,
                    format!("cannot mutate fields on {ty}"),
                )]),
                Value::Version(_)
                | Value::Length(_)
                | Value::Relative(_)
                | Value::Stroke(_)
                | Value::Align(_) => Err(vec![SourceDiagnostic::error(
                    target_span,
                    format!("fields on {ty} are not yet mutable"),
                )
                .with_hint(format!(
                    "try creating a new {ty} with the updated field value instead"
                ))]),
                _ => Err(vec![SourceDiagnostic::error(
                    target_span,
                    format!("{ty} does not have accessible fields"),
                )]),
            }
        }
    }
}

/// **P716** — mirror de `Args::expect`: tira o primeiro posicional ou erra
/// "missing argument: {what}" (mensagem do vanilla).
fn expect_positional(args: &mut Args, span: Span, what: &str) -> SourceResult<Value> {
    if args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            span,
            format!("missing argument: {what}"),
        )]);
    }
    Ok(args.items.remove(0))
}

/// **P716** — mirror de `Args::finish`: args por consumir são erro. Corre
/// DEPOIS do acesso (ordem do vanilla, `methods.rs:96` — `arr.at(5,
/// default: 0)` erra out-of-bounds, não unexpected argument).
fn finish_args(args: &Args, span: Span) -> SourceResult<()> {
    if let Some(name) = args.named.keys().next() {
        return Err(vec![SourceDiagnostic::error(
            span,
            format!("unexpected argument: {name}"),
        )]);
    }
    if !args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            span,
            "unexpected argument".to_string(),
        )]);
    }
    Ok(())
}

/// **P716** — se o tipo tem um método (só de leitura) com este nome, o erro
/// do vanilla é "cannot mutate a temporary value"; senão "type {ty} has no
/// method `{method}`" (`methods.rs:72-80`, `ty.scope().get(method)`). O
/// espelho consulta a superfície de métodos do vanilla: str/bytes têm
/// `first`/`last`/`at`; content tem `func`/`has`/`at`/`fields`/`location`
/// (P829 — `foundations/content/mod.rs:510-590`); version e arguments têm `at`.
fn has_readonly_method(value: &Value, method: &str) -> bool {
    match (value, method) {
        (Value::Str(_) | Value::Bytes(_), "first" | "last" | "at") => true,
        (Value::Content(_), "func" | "has" | "at" | "fields" | "location") => true,
        (Value::Version(_) | Value::Args(_), "at") => true,
        _ => false,
    }
}

/// **P716** — mirror de `call_method_access` do vanilla (`methods.rs:66-98`):
/// devolve a referência mutável ao elemento/campo. Array: `first`/`last`/
/// `at(index)` (índice negativo conta do fim — `locate_opt`, mesma regra do
/// `array.at` de leitura, P714; a mensagem de out-of-bounds da escrita NÃO
/// tem o sufixo "and no default value was specified"). Dict: só `at(key)`.
fn call_method_access<'a>(
    value: &'a mut Value,
    method: &str,
    mut args: Args,
    span: Span,
) -> SourceResult<&'a mut Value> {
    if !matches!(value, Value::Array(_) | Value::Dict(_)) {
        return Err(vec![if has_readonly_method(value, method) {
            SourceDiagnostic::error(span, "cannot mutate a temporary value".to_string())
        } else {
            SourceDiagnostic::error(
                span,
                format!("type {} has no method `{method}`", long_type_name(value)),
            )
        }]);
    }

    let slot = match value {
        Value::Array(arr) => match method {
            "first" => match arr.first_mut() {
                Some(slot) => slot,
                None => {
                    return Err(vec![SourceDiagnostic::error(
                        span,
                        "array is empty".to_string(),
                    )])
                }
            },
            "last" => match arr.last_mut() {
                Some(slot) => slot,
                None => {
                    return Err(vec![SourceDiagnostic::error(
                        span,
                        "array is empty".to_string(),
                    )])
                }
            },
            "at" => {
                let index = match expect_positional(&mut args, span, "index")? {
                    Value::Int(i) => i,
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            span,
                            format!("expected integer, found {}", long_type_name(&other)),
                        )])
                    }
                };
                let len = arr.len() as i64;
                let resolved =
                    if index >= 0 { Some(index) } else { len.checked_add(index) };
                match resolved
                    .filter(|&v| v >= 0 && v < len)
                    .and_then(|v| arr.get_mut(v as usize))
                {
                    Some(slot) => slot,
                    None => {
                        return Err(vec![SourceDiagnostic::error(
                            span,
                            format!(
                                "array index out of bounds (index: {index}, len: {len})"
                            ),
                        )])
                    }
                }
            }
            _ => unreachable!("is_accessor_method garante first/last/at"),
        },
        Value::Dict(dict) => match method {
            "at" => {
                let key = match expect_positional(&mut args, span, "key")? {
                    Value::Str(s) => s,
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            span,
                            format!("expected string, found {}", long_type_name(&other)),
                        )])
                    }
                };
                match dict.get_mut(key.as_str()) {
                    Some(slot) => slot,
                    None => return Err(missing_key(span, key.as_str())),
                }
            }
            // dict não tem `first`/`last` (nem no vanilla) → missing method.
            _ => {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    format!("type dictionary has no method `{method}`"),
                )])
            }
        },
        _ => unreachable!("filtrado pelo guard acima"),
    };

    finish_args(&args, span)?;
    Ok(slot)
}

// ── P717 — métodos mutantes (`push`, `pop`, `insert`, `remove`): mirror de
// `is_mutating_method`/`call_method_mut` (vanilla `typst-eval/methods.rs:
// 9-16,24-63`) e do despacho `maybe_resolve_mutating` (`call.rs:189-212`),
// sobre a fundação `access()` de P716. ─────────────────────────────────────

/// **P717** — a lista exacta (e completa) de mutating methods do vanilla
/// (`methods.rs:9-11`): `push`, `pop`, `insert`, `remove`. Não há outros.
pub(super) fn is_mutating_method(method: &str) -> bool {
    matches!(method, "push" | "pop" | "insert" | "remove")
}

/// **P717** — subconjunto aplicável a dict (`methods.rs:14-16`): dict não
/// tem `push`/`pop`.
fn is_dict_mutating_method(method: &str) -> bool {
    matches!(method, "insert" | "remove")
}

/// **P717** — mirror de `maybe_resolve_mutating` (vanilla `call.rs:189-212`):
/// avalia os **args primeiro** (`call.rs:196-198` — `access()` toma o
/// empréstimo mutável de `scopes`), depois resolve o target como local via
/// `access()` (P716; targets temporários erram `cannot mutate a temporary
/// value` antes de qualquer resolução). Devolve `Ok(None)` = fall-through
/// para a cadeia normal de `eval_func_call` — só para tipos cujos campos
/// podem resolver para função (ex.: módulo com função `insert`).
/// Divergência medida e aceite (ver `rules/eval.md` §P717): o fall-through
/// re-avalia target e args (o vanilla passa os já avaliados) — dupla
/// avaliação de efeitos só nesse caminho, sem consumidor em `cetz`.
pub(super) fn try_eval_mutating_method(
    fa: crate::entities::ast::expr::FieldAccess<'_>,
    args_node: crate::entities::ast::expr::Args<'_>,
    span: Span,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Option<Value>> {
    let method: EcoString = fa.field().as_str().into();
    let args = super::closures::eval_args(args_node, scopes, ctx, engine)?;
    match access(fa.target(), scopes, ctx, engine)? {
        // dict não tem push/pop, e dicts deliberadamente não resolvem campos
        // como métodos (vanilla `call.rs:233-238`) — mesma mensagem que o
        // fall-through do vanilla produz, sem a maquinaria.
        Value::Dict(_) if !is_dict_mutating_method(method.as_str()) => {
            Err(vec![SourceDiagnostic::error(
                span,
                format!("type dictionary has no method `{method}`"),
            )])
        }
        target @ (Value::Array(_) | Value::Dict(_)) => {
            call_method_mut(target, method.as_str(), args, span).map(Some)
        }
        Value::Module(_)
        | Value::Func(_)
        | Value::Type(_)
        | Value::Symbol(_)
        | Value::Content(_) => Ok(None),
        // Restantes tipos (escalares, str, bytes, …): nenhum caminho pode
        // resolver o método — erro verbatim do vanilla (medido para string).
        other => Err(vec![SourceDiagnostic::error(
            span,
            format!("type {} has no method `{method}`", long_type_name(other)),
        )]),
    }
}

/// **P717** — mirror de `call_method_mut` (vanilla `methods.rs:24-63`):
/// muta o local e devolve o output (`pop`/`remove` devolvem o elemento
/// removido; `push`/`insert` devolvem none). `insert` de array usa `locate`
/// com `end_ok=true` (`array.rs:246-257`: índice == len permitido, negativo
/// conta do fim) e erra **sem** o sufixo de default; `remove` tem `default:`
/// (`array.rs:261-274`, `dict.rs:241-251`: fora de limites/chave ausente →
/// default **sem mutar**, senão erro — array **com** sufixo, dict **sem
/// hint**, ao contrário do `at_mut` de P716). `finish_args` corre **depois**
/// da mutação (`methods.rs:61`).
fn call_method_mut(
    value: &mut Value,
    method: &str,
    mut args: Args,
    span: Span,
) -> SourceResult<Value> {
    let mut output = Value::None;

    match value {
        Value::Array(arr) => match method {
            "push" => {
                let v = expect_positional(&mut args, span, "value")?;
                arr.push(v);
            }
            "pop" => {
                output = match arr.pop() {
                    Some(v) => v,
                    None => {
                        return Err(vec![SourceDiagnostic::error(
                            span,
                            "array is empty".to_string(),
                        )])
                    }
                };
            }
            "insert" => {
                let index = match expect_positional(&mut args, span, "index")? {
                    Value::Int(i) => i,
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            span,
                            format!("expected integer, found {}", long_type_name(&other)),
                        )])
                    }
                };
                let v = expect_positional(&mut args, span, "value")?;
                let len = arr.len() as i64;
                let resolved =
                    if index >= 0 { Some(index) } else { len.checked_add(index) };
                match resolved.filter(|&i| i >= 0 && i <= len) {
                    Some(i) => arr.insert(i as usize, v),
                    None => {
                        return Err(vec![SourceDiagnostic::error(
                            span,
                            format!(
                                "array index out of bounds (index: {index}, len: {len})"
                            ),
                        )])
                    }
                }
            }
            "remove" => {
                let index = match expect_positional(&mut args, span, "index")? {
                    Value::Int(i) => i,
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            span,
                            format!("expected integer, found {}", long_type_name(&other)),
                        )])
                    }
                };
                let default = args.named.shift_remove("default");
                let len = arr.len() as i64;
                let resolved =
                    if index >= 0 { Some(index) } else { len.checked_add(index) };
                output = match resolved.filter(|&i| i >= 0 && i < len) {
                    Some(i) => arr.remove(i as usize),
                    None => match default {
                        Some(v) => v,
                        None => return Err(vec![SourceDiagnostic::error(
                            span,
                            format!(
                                "array index out of bounds (index: {index}, len: {len}) \
                                     and no default value was specified"
                            ),
                        )]),
                    },
                };
            }
            _ => unreachable!("is_mutating_method garante push/pop/insert/remove"),
        },
        Value::Dict(dict) => match method {
            "insert" => {
                let key = match expect_positional(&mut args, span, "key")? {
                    Value::Str(s) => s,
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            span,
                            format!("expected string, found {}", long_type_name(&other)),
                        )])
                    }
                };
                let v = expect_positional(&mut args, span, "value")?;
                dict.insert(key, v);
            }
            "remove" => {
                let key = match expect_positional(&mut args, span, "key")? {
                    Value::Str(s) => s,
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            span,
                            format!("expected string, found {}", long_type_name(&other)),
                        )])
                    }
                };
                let default = args.named.shift_remove("default");
                output = match dict.shift_remove(key.as_str()) {
                    Some(v) => v,
                    None => match default {
                        Some(v) => v,
                        None => {
                            return Err(vec![SourceDiagnostic::error(
                                span,
                                format!(
                                    "dictionary does not contain key {:?}",
                                    key.as_str()
                                ),
                            )])
                        }
                    },
                };
            }
            _ => unreachable!("try_eval_mutating_method filtra push/pop para dict"),
        },
        _ => unreachable!("try_eval_mutating_method só passa Array/Dict"),
    }

    finish_args(&args, span)?;
    Ok(output)
}

/// **P506** — Despacha métodos de `Value::State`: `.update()`, `.get()`,
/// `.display()`.
pub(super) fn eval_state_method(
    state: &State,
    method: &str,
    args: crate::entities::ast::expr::Args<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    use crate::engine::eval::closures::eval_args;
    let span = args.span();
    match method {
        "update" => {
            let args = eval_args(args, scopes, ctx, engine)?;
            match args.items.as_slice() {
                [value] => Ok(state_update(state.key.clone(), value.clone())),
                _ => Err(vec![SourceDiagnostic::error(
                    span,
                    "state.update() requer 1 argumento".to_string(),
                )]),
            }
        }
        "get" => {
            let _ = eval_args(args, scopes, ctx, engine)?;
            state_get(state, ctx, span)
        }
        "display" => {
            let args = eval_args(args, scopes, ctx, engine)?;
            state_display(state, &args, scopes, ctx, engine, span)
        }
        // **P844** (achado #49 de P831) — `.at()`/`.final()` existiam como
        // nativas (`native_state_at`/`native_state_final`, foundations.rs)
        // mas não estavam ligados no dispatch de métodos.
        "at" => state_at_dispatch(state, args, scopes, ctx, engine, span),
        "final" => {
            let args = eval_args(args, scopes, ctx, engine)?;
            // Mensagens verbatim medidas no vanilla 0.15.0:
            // `s.final(1)` → "unexpected argument".
            if !args.items.is_empty() {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    "unexpected argument".to_string(),
                )]);
            }
            state_final(state, ctx, span)
        }
        _ => Err(vec![SourceDiagnostic::error(
            span,
            format!("state não tem método '{}'", method),
        )]),
    }
}

/// **P844** (achado #49 de P831) — Despacha `state.at(selector)`.
/// Aceita `Location` directa (ex.: `here()`) ou `<label>` resolvida via
/// introspector — paridade vanilla `State::at` (`LocatableSelector`).
/// Mensagens verbatim medidas no vanilla 0.15.0:
/// - `s.at()` → `missing argument: selector`
/// - `s.at(1)` → `expected label, function, location, or selector, found integer`
/// - `s.at("x")` → `text is not locatable`
/// - `s.at(<inexistente>)` → ``label `<inexistente>` does not exist in the document``
/// A validação de argumentos precede o gate de contexto (ordem medida:
/// `s.at(1)` fora de contexto → erro de tipo, não gate).
fn state_at_dispatch(
    state: &State,
    args: crate::entities::ast::expr::Args<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    span: Span,
) -> SourceResult<Value> {
    use crate::entities::introspector::Introspector;

    let mut items = args.items();
    let Some(first) = items.next() else {
        return Err(vec![SourceDiagnostic::error(
            span,
            "missing argument: selector".to_string(),
        )]);
    };
    if items.next().is_some() {
        return Err(vec![SourceDiagnostic::error(
            span,
            "unexpected argument".to_string(),
        )]);
    }

    // Resolve o selector para uma Location (label → lookup no introspector).
    let resolve_label = |label: &crate::entities::label::Label,
                         ctx: &EvalContext|
     -> SourceResult<crate::entities::location::Location> {
        ctx.introspector.query_by_label(label).ok_or_else(|| {
            vec![SourceDiagnostic::error(
                span,
                format!("label `<{}>` does not exist in the document", label.0),
            )]
        })
    };

    match first {
        Arg::Pos(Expr::Label(node)) => {
            let label = crate::entities::label::Label(node.get().to_string());
            let loc = resolve_label(&label, ctx)?;
            state_at_location(state, loc, ctx, span)
        }
        Arg::Pos(expr) => {
            let value = eval_expr(expr, scopes, ctx, engine)?;
            match value {
                Value::Location(loc) => state_at_location(state, loc, ctx, span),
                Value::Label(label) => {
                    let loc = resolve_label(&label, ctx)?;
                    state_at_location(state, loc, ctx, span)
                }
                Value::Content(crate::entities::content::Content::Label(e)) => {
                    let label = crate::entities::label::Label(e.name.to_string());
                    let loc = resolve_label(&label, ctx)?;
                    state_at_location(state, loc, ctx, span)
                }
                Value::Str(_) => Err(vec![SourceDiagnostic::error(
                    span,
                    "text is not locatable".to_string(),
                )]),
                other => Err(vec![SourceDiagnostic::error(
                    span,
                    format!(
                        "expected label, function, location, or selector, found {}",
                        long_type_name(&other)
                    ),
                )]),
            }
        }
        Arg::Named(named) => Err(vec![SourceDiagnostic::error(
            named.span(),
            format!("unexpected argument: {}", named.name().as_str()),
        )]),
        Arg::Spread(spread) => Err(vec![SourceDiagnostic::error(
            spread.span(),
            "unexpected argument".to_string(),
        )]),
    }
}

/// **P742** — Despacha métodos de `Value::Color` (padrão P506).
///
/// Sintetiza `Args` com a cor como primeiro posicional e delega nas
/// nativas estáticas de `rules/stdlib/color.rs` — validação, semântica e
/// mensagens idênticas nos dois caminhos (medido: `red.lighten(20%)` ≡
/// `color.lighten(red, 20%)`). Só é chamado para os 9 métodos de
/// `is_color_instance_method`; o braço em `closures.rs` filtra os restantes
/// (que caem no caminho genérico de field access).
pub(super) fn eval_color_method(
    color: &crate::entities::layout_types::Color,
    method: &str,
    args: crate::entities::ast::expr::Args<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    use crate::engine::eval::closures::eval_args;
    use crate::engine::stdlib::color as color_rules;
    let span = args.span();
    let mut synth = eval_args(args, scopes, ctx, engine)?;
    synth.items.insert(0, Value::Color(*color));
    let world = engine.world;
    let current_file = engine.current_file;
    match method {
        "lighten" => color_rules::native_color_lighten(ctx, &synth, world, current_file),
        "darken" => color_rules::native_color_darken(ctx, &synth, world, current_file),
        "mix" => {
            // **P744** — o método de instância `red.mix(blue)` não aceita
            // `weight:` (só `space:`); o peso é sempre 0.5. A estática
            // `color.mix(red, blue, weight: ...)` mantém o argumento.
            if synth.named.contains_key("weight") {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    "unexpected argument: weight".to_string(),
                )]);
            }
            color_rules::native_color_mix(ctx, &synth, world, current_file)
        }
        "negate" => color_rules::native_color_negate(ctx, &synth, world, current_file),
        "saturate" => {
            color_rules::native_color_saturate(ctx, &synth, world, current_file)
        }
        "desaturate" => {
            color_rules::native_color_desaturate(ctx, &synth, world, current_file)
        }
        "rotate" => color_rules::native_color_rotate(ctx, &synth, world, current_file),
        "components" => {
            color_rules::native_color_components(ctx, &synth, world, current_file)
        }
        "space" => color_rules::native_color_space(ctx, &synth, world, current_file),
        "to-hex" => color_rules::native_color_to_hex(ctx, &synth, world, current_file),
        "transparentize" => {
            color_rules::native_color_transparentize(ctx, &synth, world, current_file)
        }
        "opacify" => color_rules::native_color_opacify(ctx, &synth, world, current_file),
        _ => Err(vec![SourceDiagnostic::error(
            span,
            format!("color não tem método '{method}'"),
        )]),
    }
}

/// **P640** — Faz parse e validação dos argumentos de `counter.display(...)`.
/// Devolve o argumento nomeado `at:` (se válido) e o argumento posicional
/// pattern/callback (se válido). Produz erro claro para tipos inválidos,
/// argumentos não reconhecidos ou argumentos posicionais a mais.
fn parse_counter_display_args(
    args: crate::entities::ast::expr::Args<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<(Option<crate::entities::label::Label>, Option<Value>)> {
    use crate::entities::ast::expr::Arg;

    let mut at_label: Option<crate::entities::label::Label> = None;
    let mut pattern: Option<Value> = None;

    for arg in args.items() {
        match arg {
            Arg::Named(named) if named.name().as_str() == "at" => {
                at_label =
                    Some(extract_display_at_label(named.expr(), scopes, ctx, engine)?);
            }
            Arg::Named(named) => {
                return Err(vec![SourceDiagnostic::error(
                    named.span(),
                    format!("unexpected argument: {}", named.name().as_str()),
                )]);
            }
            Arg::Pos(expr) if pattern.is_none() => {
                let value = eval_expr(expr, scopes, ctx, engine)?;
                match value {
                    Value::Str(_) | Value::Func(_) => pattern = Some(value),
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            expr.span(),
                            format!(
                                "expected string, function, or auto, found {}",
                                other.type_name()
                            ),
                        )]);
                    }
                }
            }
            Arg::Pos(expr) => {
                return Err(vec![SourceDiagnostic::error(
                    expr.span(),
                    "counter.display() takes at most one positional argument".to_string(),
                )]);
            }
            Arg::Spread(spread) => {
                return Err(vec![SourceDiagnostic::error(
                    spread.span(),
                    "spread not allowed in counter.display()".to_string(),
                )]);
            }
        }
    }

    Ok((at_label, pattern))
}

/// **P640** — Extrai uma label do argumento nomeado `at:` de `counter.display`.
/// Aceita `<label>` (nó AST), string, ou content label.
fn extract_display_at_label(
    expr: Expr<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<crate::entities::label::Label> {
    let span = expr.span();
    match expr {
        Expr::Label(node) => Ok(crate::entities::label::Label(node.get().to_string())),
        other => {
            let value = eval_expr(other, scopes, ctx, engine)?;
            match value {
                Value::Str(s) => Ok(crate::entities::label::Label(s.to_string())),
                Value::Content(crate::entities::content::Content::Label(e)) => {
                    Ok(crate::entities::label::Label(e.name.to_string()))
                }
                other => Err(vec![SourceDiagnostic::error(
                    span,
                    format!(
                        "expected label, function, location, selector, or auto, found {}",
                        other.type_name()
                    ),
                )]),
            }
        }
    }
}

/// **P640** — Renderiza `counter.display(..., at: <label>)` para texto plano.
fn render_counter_at_label(
    key: &str,
    label: &crate::entities::label::Label,
    pattern: Option<&Value>,
    ctx: &EvalContext,
) -> SourceResult<Value> {
    use crate::entities::introspector::Introspector;
    let text = ctx
        .introspector
        .query_by_label(label)
        .and_then(|loc| ctx.introspector.formatted_counter_at(key, loc))
        .unwrap_or_default();

    let rendered = match pattern {
        Some(Value::Str(p)) if !p.is_empty() => {
            if p.as_str().ends_with('.') {
                let mut out = text.clone();
                out.push('.');
                out
            } else {
                format!("{p}{text}")
            }
        }
        Some(Value::Func(_)) => {
            // Callbacks com `at:` não são suportados nesta fase; ignorar o
            // pattern e devolver o texto formatado pelo introspector.
            text
        }
        _ => text,
    };

    Ok(Value::Content(Content::text(rendered)))
}

/// **P506** — Despacha métodos de `Value::Counter`: `.update()`, `.step()`,
/// `.get()`, `.display()`, `.at()`.
pub(super) fn eval_counter_method_value(
    counter: &Counter,
    method: &str,
    args: crate::entities::ast::expr::Args<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    use crate::engine::eval::closures::eval_args;
    let span = args.span();
    match method {
        "update" => {
            let args = eval_args(args, scopes, ctx, engine)?;
            counter_update(
                counter.key.clone(),
                args.items.into_iter().next().unwrap_or(Value::None),
            )
        }
        "step" => {
            let _ = eval_args(args, scopes, ctx, engine)?;
            Ok(counter_step(counter.key.clone()))
        }
        "get" => {
            let _ = eval_args(args, scopes, ctx, engine)?;
            counter_get(counter, ctx, span)
        }
        "display" => {
            // P640 — parse unificado e validação estrita dos argumentos.
            let (at_label, pattern) =
                parse_counter_display_args(args, scopes, ctx, engine)?;
            if let Some(label) = at_label {
                render_counter_at_label(
                    counter.key.as_str(),
                    &label,
                    pattern.as_ref(),
                    ctx,
                )
            } else {
                let args = Args::positional(pattern.into_iter().collect());
                counter_display(counter, &args, scopes, ctx, engine, span)
            }
        }
        "at" => {
            // P506 — counter.at(label): o parser cristalino avalia `<label>`
            // como Value::None; extraímos a string directamente do nó AST
            // para suportar a sintaxe vanilla.
            //
            // **P844** (achado #50 de P831) — aceita também `Location`
            // directa (ex.: `here()`) — paridade vanilla `Counter::at`
            // (`introspection/counter.rs:452`). Mensagens verbatim medidas
            // no vanilla 0.15.0: `counter.at()` →
            // `missing argument: selector`; `counter.at(1)` →
            // `expected label, function, location, or selector, found integer`.
            let mut items = args.items();
            let Some(first) = items.next() else {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    "missing argument: selector".to_string(),
                )]);
            };
            if items.next().is_some() {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    "unexpected argument".to_string(),
                )]);
            }
            match first {
                Arg::Pos(Expr::Label(node)) => {
                    let label = crate::entities::label::Label(node.get().to_string());
                    counter_at(counter, label, ctx, span)
                }
                Arg::Pos(expr) => {
                    let value = eval_expr(expr, scopes, ctx, engine)?;
                    match value {
                        Value::Str(s) => counter_at(
                            counter,
                            crate::entities::label::Label(s.to_string()),
                            ctx,
                            span,
                        ),
                        Value::Content(crate::entities::content::Content::Label(e)) => {
                            counter_at(
                                counter,
                                crate::entities::label::Label(e.name.to_string()),
                                ctx,
                                span,
                            )
                        }
                        Value::Label(l) => counter_at(counter, l.clone(), ctx, span),
                        Value::Location(loc) => counter_at_location(counter, loc, ctx),
                        other => Err(vec![SourceDiagnostic::error(
                            span,
                            format!(
                                "expected label, function, location, or selector, found {}",
                                long_type_name(&other)
                            ),
                        )]),
                    }
                }
                Arg::Named(named) => Err(vec![SourceDiagnostic::error(
                    named.span(),
                    format!("unexpected argument: {}", named.name().as_str()),
                )]),
                Arg::Spread(spread) => Err(vec![SourceDiagnostic::error(
                    spread.span(),
                    "unexpected argument".to_string(),
                )]),
            }
        }
        // **P844** (achado #49 de P831) — `counter.final()` existia como
        // nativa (`native_counter_final`, foundations.rs) mas não estava
        // ligado no dispatch de métodos. Mensagem verbatim medida no
        // vanilla 0.15.0: `counter.final(1)` → `unexpected argument`.
        "final" => {
            let args = eval_args(args, scopes, ctx, engine)?;
            if !args.items.is_empty() {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    "unexpected argument".to_string(),
                )]);
            }
            counter_final(counter, ctx, span)
        }
        _ => Err(vec![SourceDiagnostic::error(
            span,
            format!("counter não tem método '{}'", method),
        )]),
    }
}

/// **P796** — Despacha `.at(index)`, o único método de instância de
/// `Value::Version` (`entities/version.md` §8a). Mesmo padrão de
/// `eval_counter_method_value`/`eval_color_method`: recebe os argumentos AST
/// não avaliados, avalia-os aqui.
pub(super) fn eval_version_method_value(
    version: &crate::entities::version::Version,
    method: &str,
    args: crate::entities::ast::expr::Args<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    use crate::engine::eval::closures::eval_args;
    let span = args.span();
    match method {
        "at" => {
            let args = eval_args(args, scopes, ctx, engine)?;
            let index = match args.items.as_slice() {
                [Value::Int(i)] => *i,
                [other] => {
                    return Err(vec![SourceDiagnostic::error(
                        span,
                        format!("expected integer, found {}", long_type_name(other)),
                    )])
                }
                _ => {
                    return Err(vec![SourceDiagnostic::error(
                        span,
                        "version.at() requires exactly one positional argument"
                            .to_string(),
                    )])
                }
            };
            version
                .at(index)
                .map(Value::Int)
                .map_err(|msg| vec![SourceDiagnostic::error(span, msg)])
        }
        _ => Err(vec![SourceDiagnostic::error(
            span,
            format!("version não tem método '{}'", method),
        )]),
    }
}

/// **P417 (M)** — Tenta avaliar `<elemento>.where(field: value)`.
///
/// Retorna `Ok(Some(Selector::Where { ... }))` se o target for uma função
/// nativa de elemento suportada (heading, figure, strong, emph, raw) e houver
/// exatamente um named argumento. Retorna `Ok(None)` se o target não for um
/// elemento nativo (deixa o caller continuar com field access normal).
/// Retorna `Err` se for elemento mas os argumentos forem inválidos.
pub(super) fn eval_element_where<'a>(
    target_expr: Expr<'_>,
    args_node: crate::entities::ast::expr::Args<'a>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Option<Selector>> {
    use crate::engine::stdlib::{
        native_emph, native_figure, native_heading, native_raw, native_strong,
    };
    use std::ptr::fn_addr_eq;

    let target = eval_expr(target_expr, scopes, ctx, engine)?;
    let Value::Func(ref f) = target else {
        return Ok(None);
    };

    let kind = match f.native_fn_addr() {
        Some(addr) if fn_addr_eq(addr, native_heading as fn(_, _, _, _) -> _) => {
            ElementKind::Heading
        }
        Some(addr) if fn_addr_eq(addr, native_figure as fn(_, _, _, _) -> _) => {
            ElementKind::Figure
        }
        Some(addr) if fn_addr_eq(addr, native_strong as fn(_, _, _, _) -> _) => {
            return Err(vec![SourceDiagnostic::error(
                target_expr.span(),
                "selector where não suportado para strong".to_string(),
            )])
        }
        Some(addr) if fn_addr_eq(addr, native_emph as fn(_, _, _, _) -> _) => {
            return Err(vec![SourceDiagnostic::error(
                target_expr.span(),
                "selector where não suportado para emph".to_string(),
            )])
        }
        Some(addr) if fn_addr_eq(addr, native_raw as fn(_, _, _, _) -> _) => {
            return Err(vec![SourceDiagnostic::error(
                target_expr.span(),
                "selector where não suportado para raw".to_string(),
            )])
        }
        _ => return Ok(None),
    };

    let mut named_args: Vec<(EcoString, Value)> = Vec::new();
    for arg in args_node.items() {
        match arg {
            Arg::Pos(_) => {
                return Err(vec![SourceDiagnostic::error(
                    args_node.span(),
                    "heading.where() requer argumentos nomeados (ex.: level: 1)"
                        .to_string(),
                )]);
            }
            Arg::Named(named) => {
                let field = named.name().as_str().into();
                let value = eval_expr(named.expr(), scopes, ctx, engine)?;
                named_args.push((field, value));
            }
            Arg::Spread(_) => {}
        }
    }

    if named_args.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            args_node.span(),
            "heading.where() requer pelo menos um argumento nomeado".to_string(),
        )]);
    }

    let mut selector = Selector::Kind(kind);
    for (field, value) in named_args {
        selector = Selector::Where {
            base: Box::new(selector),
            field,
            value: Box::new(value),
        };
    }
    Ok(Some(selector))
}

/// **P423 (S-M)** — Tenta avaliar `<selector>.or(other)` ou
/// `<selector>.and(other)`.
///
/// Retorna `Ok(Some(Selector::Or(...)))` ou `Ok(Some(Selector::And(...)))`
/// se o target avaliar para `Value::Selector` e houver exactamente um
/// argumento posicional que também avalie para `Value::Selector`.
/// Retorna `Ok(None)` se o target não for um selector (deixa o caller
/// continuar com field access normal).
/// Converte `Value::Func` nativo de elemento (heading, figure) ou
/// `Value::Selector` num `Selector` de query. Retorna `None` se o valor não
/// for convertível.
fn value_to_query_selector(value: &Value) -> Option<Selector> {
    match value {
        Value::Selector(s) => Some(s.clone()),
        Value::Func(f) => {
            use crate::engine::stdlib::{native_figure, native_heading};
            use std::ptr::fn_addr_eq;
            f.native_fn_addr().and_then(|addr| {
                if fn_addr_eq(addr, native_heading as fn(_, _, _, _) -> _) {
                    Some(Selector::Kind(ElementKind::Heading))
                } else if fn_addr_eq(addr, native_figure as fn(_, _, _, _) -> _) {
                    Some(Selector::Kind(ElementKind::Figure))
                } else {
                    None
                }
            })
        }
        _ => None,
    }
}

pub(super) fn eval_selector_or_and<'a>(
    target_expr: Expr<'_>,
    method: &str,
    args_node: crate::entities::ast::expr::Args<'a>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Option<Selector>> {
    let target = eval_expr(target_expr, scopes, ctx, engine)?;
    let Some(base) = value_to_query_selector(&target) else {
        return Ok(None);
    };

    let args = super::closures::eval_args(args_node, scopes, ctx, engine)?;
    let other = args.items.into_iter().next().ok_or_else(|| {
        vec![SourceDiagnostic::error(
            args_node.span(),
            format!("selector.{}() requer um argumento posicional", method),
        )]
    })?;
    let Some(other_sel) = value_to_query_selector(&other) else {
        return Err(vec![SourceDiagnostic::error(
            args_node.span(),
            format!(
                "selector.{}() espera um selector, recebeu {}",
                method,
                other.type_name()
            ),
        )]);
    };

    match method {
        "or" => Ok(Some(Selector::Or(EcoVec::from(vec![base, other_sel])))),
        "and" => Ok(Some(Selector::And(EcoVec::from(vec![base, other_sel])))),
        _ => Ok(None),
    }
}

/// **P504** — Intercepta `selector(base).within(ancestor)`.
pub(super) fn eval_selector_within<'a>(
    target_expr: Expr<'_>,
    args_node: crate::entities::ast::expr::Args<'a>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Option<Selector>> {
    let target = eval_expr(target_expr, scopes, ctx, engine)?;
    let Some(base) = value_to_query_selector(&target) else {
        return Ok(None);
    };

    let args = super::closures::eval_args(args_node, scopes, ctx, engine)?;
    let other = args.items.into_iter().next().ok_or_else(|| {
        vec![SourceDiagnostic::error(
            args_node.span(),
            "selector.within() requer um argumento posicional".to_string(),
        )]
    })?;
    let Some(ancestor) = value_to_query_selector(&other) else {
        return Err(vec![SourceDiagnostic::error(
            args_node.span(),
            format!(
                "selector.within() espera um selector, recebeu {}",
                other.type_name()
            ),
        )]);
    };

    Ok(Some(Selector::Within { base: Box::new(base), ancestor: Box::new(ancestor) }))
}

// ── Dispatcher arms: FieldAccess (Passo 96.2, ADR-0037 Regra 4) ───────────

pub(super) fn eval_field_access(
    access: crate::entities::ast::expr::FieldAccess<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    use crate::entities::ast::AstNode;

    let target = eval_expr(access.target(), scopes, ctx, engine)?;
    let field = access.field().as_str();

    // P509 — field access em coleções despacha para métodos de instância.
    if let Some(result) = crate::engine::stdlib::try_dispatch_collection_method(
        target.clone(),
        field,
        crate::entities::args::Args::positional(vec![]),
        scopes,
        ctx,
        engine,
    ) {
        return result;
    }

    // P792 — `text.<campo>`: quando o target é Value::Func("text") e o field é
    // um parâmetro de estilo, ler da StyleChain activa.
    if let Value::Func(ref f) = target {
        if f.name() == Some("text") {
            match field {
                "lang" => {
                    let lang = match engine.styles.custom("text.lang") {
                        Some(Value::Str(s)) => s.clone(),
                        _ => "en".into(),
                    };
                    return Ok(Value::Str(lang));
                }
                _ => {} // cai no eval_value_field_access normal
            }
        }
    }

    // P820 (achado #7 de P810) — acesso a símbolo **depreciado** do módulo
    // `sym` (`#sym.join`): warning verbatim do vanilla com span no campo
    // (medido: `#sym.join` → warning @1:5, exit 0). O mecanismo geral do
    // vanilla é `Deprecation` em `Binding` (`foundations/scope.rs:288`);
    // aqui, data-driven pela tabela `SYM_DEPRECATED` do módulo `sym`.
    if let Value::Module(ref m) = target {
        if m.name() == "sym" {
            if let Some(msg) = crate::engine::stdlib::sym::sym_deprecation(field) {
                engine.sink.warn_note(access.field().span(), msg, "");
            }
        }
    }

    eval_value_field_access(target, field, access.span())
}

pub(super) fn eval_value_field_access(
    target: Value,
    field: &str,
    span: Span,
) -> SourceResult<Value> {
    use crate::entities::source_result::SourceDiagnostic;
    match target {
        Value::Dict(d) => d.get(field).cloned().ok_or_else(|| {
            vec![SourceDiagnostic::error(
                span,
                format!("dictionary does not contain key \"{field}\""),
            )]
        }),
        // Field access em elementos estruturados — usado por show rules (Passo 68).
        Value::Content(c) => c.get_field(field).ok_or_else(|| {
            vec![SourceDiagnostic::error(
                span,
                format!("{} does not have field \"{field}\"", c.elem_name()),
            )]
        }),
        // P785b — Field access em Value::Relative (RelativeLength / Rel)
        Value::Relative(rel) => match field {
            "ratio" => Ok(Value::Ratio(crate::entities::layout_types::Ratio(rel.rel))),
            "length" => Ok(Value::Length(rel.abs)),
            _ => Err(vec![SourceDiagnostic::error(
                span,
                format!("relative length does not contain field \"{field}\""),
            )]),
        },
        // P785b — Field access em Value::Align (Alignment)
        Value::Align(align) => match field {
            "x" => Ok(align
                .h
                .map(|h| {
                    Value::Align(crate::entities::layout_types::Align2D {
                        h: Some(h),
                        v: None,
                    })
                })
                .unwrap_or(Value::None)),
            "y" => Ok(align
                .v
                .map(|v| {
                    Value::Align(crate::entities::layout_types::Align2D {
                        h: None,
                        v: Some(v),
                    })
                })
                .unwrap_or(Value::None)),
            _ => Err(vec![SourceDiagnostic::error(
                span,
                format!("alignment does not contain field \"{field}\""),
            )]),
        },
        // P785b — Field access em Value::Length
        Value::Length(len) => match field {
            "em" => Ok(Value::Float(len.em)),
            "abs" => Ok(Value::Length(crate::entities::layout_types::Length::pt(
                len.abs.to_pt(),
            ))),
            _ => Err(vec![SourceDiagnostic::error(
                span,
                format!("length does not contain field \"{field}\""),
            )]),
        },
        // P785b — Field access em Value::Stroke
        Value::Stroke(stroke) => match field {
            "paint" => Ok(Value::Color(stroke.paint.to_color())),
            "thickness" => Ok(Value::Length(crate::entities::layout_types::Length::pt(
                stroke.thickness,
            ))),
            _ => Err(vec![SourceDiagnostic::error(
                span,
                format!("stroke does not contain field \"{field}\""),
            )]),
        },
        // P684 — Field access em Value::Version
        Value::Version(v) => match field {
            "major" => Ok(Value::Int(v.component(0) as i64)),
            "minor" => Ok(Value::Int(v.component(1) as i64)),
            "patch" => Ok(Value::Int(v.component(2) as i64)),
            _ => Err(vec![SourceDiagnostic::error(
                span,
                format!("version does not contain field \"{field}\""),
            )]),
        },
        // P412 — Field access em Value::Duration
        Value::Duration(d) => {
            const NANOS_PER_SECOND: f64 = 1_000_000_000.0;
            const NANOS_PER_MINUTE: f64 = 60_000_000_000.0;
            const NANOS_PER_HOUR: f64 = 3_600_000_000_000.0;
            const NANOS_PER_DAY: f64 = 86_400_000_000_000.0;
            match field {
                "seconds" => Ok(Value::Float(d.nanos as f64 / NANOS_PER_SECOND)),
                "minutes" => Ok(Value::Float(d.nanos as f64 / NANOS_PER_MINUTE)),
                "hours" => Ok(Value::Float(d.nanos as f64 / NANOS_PER_HOUR)),
                "days" => Ok(Value::Float(d.nanos as f64 / NANOS_PER_DAY)),
                _ => Err(vec![SourceDiagnostic::error(
                    span,
                    format!("duration does not contain field \"{field}\""),
                )]),
            }
        }
        // P493a — Field access em Value::Array
        Value::Array(arr) => match field {
            "len" => Ok(Value::Int(arr.len() as i64)),
            "first" => Ok(arr.first().cloned().unwrap_or(Value::None)),
            "last" => Ok(arr.last().cloned().unwrap_or(Value::None)),
            _ => Err(vec![SourceDiagnostic::error(
                span,
                format!("array does not contain field \"{field}\""),
            )]),
        },
        // P504 — Field access em Value::Args
        Value::Args(a) => match field {
            "named" => Ok(Value::Dict(a.named.clone())),
            "positional" => Ok(Value::Array(a.items.clone())),
            _ => Err(vec![SourceDiagnostic::error(
                span,
                format!("arguments does not contain field \"{field}\""),
            )]),
        },
        // P493b — Field access em Value::Func com namespace
        Value::Func(f) => match f.namespace() {
            Some(ns) => ns.get(field).cloned().ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    span,
                    format!("function does not contain field \"{field}\""),
                )]
            }),
            None => Err(vec![SourceDiagnostic::error(
                span,
                "cannot access fields on type function".to_string(),
            )]),
        },
        // P685 — Field access em valor-tipo
        Value::Type(t) => match (t, field) {
            (Type::Int, "min") => Ok(Value::Int(i64::MIN)),
            (Type::Int, "max") => Ok(Value::Int(i64::MAX)),
            (Type::Str, "from-unicode") => {
                Ok(Value::Func(Func::native("str.from-unicode", native_str_from_unicode)))
            }
            (Type::Color, _) => crate::engine::stdlib::color_type_field(field)
                .ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        span,
                        format!("type color does not contain field `{field}`"),
                    )]
                }),
            (Type::Gradient, _) => crate::engine::stdlib::gradient_type_field(field)
                .ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        span,
                        format!("type gradient does not contain field `{field}`"),
                    )]
                }),
            _ => Err(vec![SourceDiagnostic::error(
                span,
                format!("type {} does not contain field \"{field}\"", t.name()),
            )]),
        },
        // P679 — Field access em Value::Module
        Value::Module(m) => m.scope().get(field).cloned().ok_or_else(|| {
            vec![SourceDiagnostic::error(
                span,
                format!("module '{}' does not contain field \"{field}\"", m.name()),
            )]
        }),
        // P765a — Field access em Value::Symbol
        Value::Symbol(s) => s.modified(field).map(Value::Symbol).ok_or_else(|| {
            vec![SourceDiagnostic::error(
                span,
                format!("unknown symbol modifier '{field}'"),
            )]
        }),
        other => Err(vec![SourceDiagnostic::error(
            span,
            format!("cannot access fields on type {}", other.type_name()),
        )]),
    }
}

// ── P815 — método inexistente / dict-key-call (eval_field_callee) ───────────

/// **P815** — mirror de `element_or_type_with_name` do vanilla
/// (`typst-eval/src/call.rs:359-365`): `("element", nome do elemento)` para
/// content, `("type", nome longo do tipo)` nos restantes.
fn element_or_type_with_name(value: &Value) -> (&'static str, String) {
    if let Value::Content(c) = value {
        ("element", c.elem_name().to_string())
    } else {
        ("type", long_type_name(value).to_string())
    }
}

/// **P815** — mirror do ramo de erro de `eval_field_callee` do vanilla
/// (`typst-eval/src/call.rs:258-345`), para um callee `target.field` chamado
/// como função depois de todos os despachos de método legítimos terem falhado.
///
/// Devolve `None` para `Symbol`/`Func`/`Type`/`Module` — os únicos tipos que
/// o vanilla deixa chamar campos directamente (`call.rs:258-263`) —, caindo
/// no caminho genérico existente. Nos restantes alvos:
///
/// - **O campo existe** (dict key, campo de content/length/args/…):
///   - dict → `cannot directly call dictionary keys as functions` + 2 hints;
///   - args → `cannot directly call named argument fields as functions` + 2 hints;
///   - outros → `` `{field}` is not a valid method for {kind} `{name}` `` + hint.
///   O primeiro hint muda se o valor guardado for função:
///   `to call the stored function, wrap the field access in parentheses:
///   `({full_text})(..)`` — senão `to access the `{field}` {key|argument|
///   field}, remove the function arguments: `{full_text}``.
/// - **O campo não existe** → `{kind} {name} has no method `{field}``
///   (ex.: `type integer has no method `foo``, `element strong has no method
///   `zzz``).
///
/// As mensagens são verbatim do vanilla (medidas por sonda em P810/P815 — a
/// mensagem é o observável, ADR-0107). Nota P810/P815: o vanilla **não** usa
/// distância de edição aqui (`call.rs:339-340`: "We don't try as hard on the
/// error here to avoid assuming the user's intent") — a hipótese do prompt
/// está refutada pela fonte e pela sonda.
pub(super) fn field_callee_error(
    target: &Value,
    access: crate::entities::ast::expr::FieldAccess<'_>,
) -> Option<Vec<SourceDiagnostic>> {
    use crate::entities::ast::AstNode;

    if matches!(
        target,
        Value::Symbol(_) | Value::Func(_) | Value::Type(_) | Value::Module(_)
    ) {
        return None;
    }

    let field = access.field().as_str();
    let span = access.span();
    let is_dict = matches!(target, Value::Dict(_));
    let is_named = matches!(target, Value::Args(_));

    match eval_value_field_access(target.clone(), field, span) {
        Ok(callee_value) => {
            let mut err = if is_dict {
                SourceDiagnostic::error(span, "cannot directly call dictionary keys as functions")
            } else if is_named {
                SourceDiagnostic::error(
                    span,
                    "cannot directly call named argument fields as functions",
                )
            } else {
                let (kind, name) = element_or_type_with_name(target);
                SourceDiagnostic::error(
                    span,
                    format!("`{field}` is not a valid method for {kind} `{name}`"),
                )
            };
            let full_text = access.to_untyped().clone().into_text();
            if matches!(callee_value, Value::Func(_)) {
                err = err.with_hint(format!(
                    "to call the stored function, wrap the field access in parentheses: `({full_text})(..)`"
                ));
            } else {
                let what =
                    if is_dict { "key" } else if is_named { "argument" } else { "field" };
                err = err.with_hint(format!(
                    "to access the `{field}` {what}, remove the function arguments: `{full_text}`"
                ));
            }
            if is_dict {
                err = err.with_hint(
                    "dictionary keys cannot be used with method syntax as keys could conflict with built-in method names".to_string(),
                );
            } else if is_named {
                err = err.with_hint(
                    "named arguments cannot be used with method syntax as argument names could conflict with built-in method names".to_string(),
                );
            }
            Some(vec![err])
        }
        Err(_) => {
            let (kind, name) = element_or_type_with_name(target);
            Some(vec![SourceDiagnostic::error(
                span,
                format!("{kind} {name} has no method `{field}`"),
            )])
        }
    }
}

// ── P829 — métodos de `content`: func/has/at/fields/location ───────────────

/// **P829** — estado de um campo de content para os métodos `has`/`at`/
/// `fields` (paridade vanilla `Content::has`/`at`/`fields`,
/// `foundations/content/mod.rs:510-590`): o vanilla distingue campo
/// **assente no constructor** de default resolvido pela chain — medido:
/// `heading[H].has("level")` → false, `heading(level: 2)[H].has("level")` →
/// true, markup `= H` assenta `depth` (não `level`).
enum ContentField {
    /// Assente — `has` → true, `at` devolve o valor.
    Set(Value),
    /// Declarado no elemento mas não assente — `has` → false; `at` erra
    /// `field "{f}" in {elem} is not known at this point` (verbatim vanilla).
    Unset,
    /// Não existe no elemento — `at` erra `{elem} does not have field "{f}"`.
    Undeclared,
}

/// **P829** — classifica um campo de content para `has`/`at`. Distinto de
/// `Content::get_field` (field access de show rules, que devolve valores
/// baked): aqui só conta o que foi assente no constructor, espelhando o
/// `field.has()`/`get()` do vanilla. A máscara `set_fields` de `HeadingElem`
/// (P829) é a única pista de "explicitamente assente" que o modelo cristalino
/// retém; nos restantes elementos os campos expostos são sempre assentes
/// (body/text), logo o fallback via `get_field` é exacto. `label` fica
/// scope-out: no cristalino a label é um nó irmão (`Content::Label`), não
/// metadado do elemento — divergência registada no L0.
fn content_field(c: &crate::entities::content::Content, field: &str) -> ContentField {
    use crate::entities::content::Content;
    use crate::entities::elements::heading::{
        HEADING_SET_DEPTH, HEADING_SET_LEVEL, HEADING_SET_OUTLINED,
    };
    match c {
        Content::Strong(e) => match field {
            "body" => ContentField::Set(Value::Content(e.body.clone())),
            // `delta` é declarado em strong no vanilla mas nunca assente no
            // cristalino (`native_strong` não aceita named — scope-out).
            "delta" => ContentField::Unset,
            _ => ContentField::Undeclared,
        },
        Content::Emph(e) => match field {
            "body" => ContentField::Set(Value::Content(e.body.clone())),
            _ => ContentField::Undeclared,
        },
        Content::Title(t) => match field {
            "body" => ContentField::Set(Value::Content(t.body.clone())),
            _ => ContentField::Undeclared,
        },
        Content::Text(t) => match field {
            "text" => ContentField::Set(Value::Str(t.clone())),
            _ => ContentField::Undeclared,
        },
        Content::Heading(h) => match field {
            "body" => ContentField::Set(Value::Content(h.body.clone())),
            "level" => {
                if h.set_fields & HEADING_SET_LEVEL != 0 {
                    ContentField::Set(Value::Int(h.level as i64))
                } else {
                    ContentField::Unset
                }
            }
            "depth" => {
                if h.set_fields & HEADING_SET_DEPTH != 0 {
                    ContentField::Set(Value::Int(h.level as i64))
                } else {
                    ContentField::Undeclared
                }
            }
            "outlined" => {
                if h.set_fields & HEADING_SET_OUTLINED != 0 {
                    ContentField::Set(Value::Bool(h.outlined))
                } else {
                    ContentField::Unset
                }
            }
            "bookmarked" => match h.bookmarked {
                Some(b) => ContentField::Set(Value::Bool(b)),
                None => ContentField::Unset,
            },
            _ => ContentField::Undeclared,
        },
        other => match other.get_field(field) {
            Some(v) => ContentField::Set(v),
            None => ContentField::Undeclared,
        },
    }
}

/// **P829** — campos assentes de um content, na ordem de declaração do
/// vanilla (para heading: level, depth, outlined, bookmarked, body — medido
/// em b8/b18: `(level: 2, body: [H])`, `(depth: 1, body: [H])`).
fn content_set_fields(
    c: &crate::entities::content::Content,
) -> Vec<(&'static str, Value)> {
    use crate::entities::content::Content;
    let candidates: &[&'static str] = match c {
        Content::Heading(_) => &["level", "depth", "outlined", "bookmarked", "body"],
        Content::Text(_) => &["text"],
        _ => &["body"],
    };
    candidates
        .iter()
        .filter_map(|name| match content_field(c, name) {
            ContentField::Set(v) => Some((*name, v)),
            _ => None,
        })
        .collect()
}

/// **P829** — fallback de `func()` para variantes sem constructor nativo
/// exposto (Sequence, Styled, Label, math, Dynamic, …): o `Value::Func`
/// existe (repr = nome do elemento — paridade medida de `func()`) mas a
/// chamada não é suportada. Caso não medido no vanilla (elementos internos
/// não expostos no scope); mensagem própria, registada no L0.
fn content_func_not_callable(
    _ctx: &mut crate::engine::eval::EvalContext,
    _args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: crate::entities::file_id::FileId,
) -> SourceResult<Value> {
    Err(vec![SourceDiagnostic::error(
        Span::detached(),
        "calling this element function is not supported".to_string(),
    )])
}

/// **P829** — `content.func()` (vanilla `Content::func`,
/// `foundations/content/mod.rs:516-519`): a função do elemento. Igualdade
/// por nome (P742) dá `strong[x].func() == strong` → true (medido b9).
fn content_elem_func(c: &crate::entities::content::Content) -> Value {
    use crate::entities::content::Content;
    let call: fn(
        &mut crate::engine::eval::EvalContext,
        &Args,
        &dyn crate::contracts::world::World,
        crate::entities::file_id::FileId,
    ) -> SourceResult<Value> = match c {
        Content::Text(_) => crate::engine::stdlib::native_text,
        Content::Strong(_) => crate::engine::stdlib::native_strong,
        Content::Emph(_) => crate::engine::stdlib::native_emph,
        Content::Heading(_) => crate::engine::stdlib::native_heading,
        Content::Title(_) => crate::engine::stdlib::native_title,
        Content::Raw(_) => crate::engine::stdlib::native_raw,
        Content::Figure(_) => crate::engine::stdlib::native_figure,
        Content::Link(_) => crate::engine::stdlib::native_link,
        Content::SmallCaps { .. } => crate::engine::stdlib::native_smallcaps,
        _ => content_func_not_callable,
    };
    Value::Func(Func::native(c.elem_name(), call))
}

/// **P829** — mirror dos métodos do `#[scope]` de `Content` do vanilla
/// (`foundations/content/mod.rs:510-590`): `func()`, `has(field)`,
/// `at(field, default:?)`, `fields()`, `location()`. Mensagens de erro de
/// argumentos verbatim (medidas b13–b17): `missing argument: field`,
/// `expected string, found {tipo}`, `unexpected argument`,
/// `unexpected argument: {nome}`.
///
/// `location()` devolve sempre `none`: o cristalino não retém metadados de
/// location em `Content` (medido: content inline → none nos dois binários;
/// content de show rule/query → `location(..)` no vanilla — divergência
/// registada no L0, requer introspecção de locations, fora do proporcional).
pub(super) fn eval_content_method(
    c: &crate::entities::content::Content,
    method: &str,
    mut args: Args,
    span: Span,
) -> SourceResult<Value> {
    match method {
        "func" => {
            finish_args(&args, span)?;
            Ok(content_elem_func(c))
        }
        "fields" => {
            finish_args(&args, span)?;
            let mut dict: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
            for (name, value) in content_set_fields(c) {
                dict.insert(name.into(), value);
            }
            Ok(Value::Dict(dict))
        }
        "location" => {
            finish_args(&args, span)?;
            Ok(Value::None)
        }
        "has" | "at" => {
            let field_value = expect_positional(&mut args, span, "field")?;
            let field = match field_value {
                Value::Str(s) => s,
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        span,
                        format!("expected string, found {}", long_type_name(&other)),
                    )])
                }
            };
            let default = if method == "at" {
                args.named.shift_remove("default")
            } else {
                None
            };
            finish_args(&args, span)?;
            let field = field.as_str();
            match method {
                "has" => Ok(Value::Bool(matches!(
                    content_field(c, field),
                    ContentField::Set(_)
                ))),
                _ => match content_field(c, field) {
                    ContentField::Set(v) => Ok(v),
                    ContentField::Unset => match default {
                        Some(v) => Ok(v),
                        None => Err(vec![SourceDiagnostic::error(
                            span,
                            format!(
                                "field \"{field}\" in {} is not known at this point and no default was specified",
                                c.elem_name()
                            ),
                        )]),
                    },
                    ContentField::Undeclared => match default {
                        Some(v) => Ok(v),
                        None => Err(vec![SourceDiagnostic::error(
                            span,
                            format!(
                                "{} does not have field \"{field}\" and no default was specified",
                                c.elem_name()
                            ),
                        )]),
                    },
                },
            }
        }
        _ => unreachable!("eval_content_method: método desconhecido {method}"),
    }
}
