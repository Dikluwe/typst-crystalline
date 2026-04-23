//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/eval.md
//! @prompt-hash 19073424
//! @layer L1
//! @updated 2026-04-22
//!
//! Bindings: `#let`, e métodos de counter. Extraído de `eval.rs` no Passo 96.1
//! conforme ADR-0037 (coesão por domínio).

use std::sync::Arc;

use comemo::{Tracked, TrackedMut};

use crate::entities::ast::code::{LetBinding, LetBindingKind};
use crate::entities::file_id::FileId;
use crate::entities::ast::expr::{Arg, Expr};
use crate::entities::content::Content;
use crate::entities::counter_state::CounterAction;
use crate::entities::show::{RuleId, ShowRule};
use crate::entities::source_result::SourceResult;
use crate::entities::style_chain::StyleChain;
use crate::entities::value::Value;
use crate::entities::world_types::{Route, Sink};
use crate::rules::scopes::Scopes;

use super::{eval_expr, EvalContext};

pub(super) fn eval_let<'r>(
    binding: LetBinding<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext<'_>,
    route: Tracked<'r, Route<'r>>,
    styles: &mut StyleChain,
    show_rules: &mut Arc<[ShowRule]>,
    active_guards: &mut Vec<RuleId>,
    current_file: FileId,
    figure_numbering: &mut Option<String>,
    sink: &mut TrackedMut<'_, Sink>,
) -> SourceResult<Value> {
    let mut value = match binding.init() {
        Some(init) => eval_expr(init, scopes, ctx, route, styles, show_rules, active_guards, current_file, figure_numbering, sink)?,
        None => Value::None,
    };

    match binding.kind() {
        LetBindingKind::Normal(pattern) => {
            // Binding simples: #let x = ... → Pattern::Normal(Expr::Ident(x))
            let bindings = pattern.bindings();
            if let Some(ident) = bindings.into_iter().next() {
                let name = ident.as_str().to_string();
                // Se a closure ainda não tem nome, dar-lhe o nome da binding (para recursão)
                if let Value::Func(ref mut func) = value {
                    func.set_name(name.clone());
                }
                scopes.define(name, value);
            }
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

/// Retorna `None` se a expressão não for uma chamada a `counter`.
pub(super) fn extract_counter_key(expr: Expr<'_>) -> Option<String> {
    let call = match expr {
        Expr::FuncCall(c) => c,
        _ => return None,
    };
    // Verificar que o callee é o identificador "counter"
    let callee_name = match call.callee() {
        Expr::Ident(id) => id.as_str().to_string(),
        _ => return None,
    };
    if callee_name != "counter" { return None; }

    // Extrair o primeiro argumento posicional como chave string
    let first_arg = call.args().items().next()?;
    match first_arg {
        Arg::Pos(Expr::Ident(id)) => Some(id.as_str().to_string()),
        Arg::Pos(Expr::Str(s))    => Some(s.get().to_string()),
        _ => None,
    }
}

/// Avalia um método de contador: step(), update(), get(), display().
pub(super) fn eval_counter_method<'a, 'r>(
    key:    &str,
    method: &str,
    args:   crate::entities::ast::expr::Args<'a>,
    scopes: &mut Scopes<'_>,
    ctx:    &mut EvalContext<'_>,
    route:  Tracked<'r, Route<'r>>,
    styles: &mut StyleChain,
    show_rules: &mut Arc<[ShowRule]>,
    active_guards: &mut Vec<RuleId>,
    current_file: FileId,
    figure_numbering: &mut Option<String>,
    sink: &mut TrackedMut<'_, Sink>,
) -> SourceResult<Value> {
    match method {
        "step" => Ok(Value::Content(Content::CounterUpdate {
            key:    key.to_string(),
            action: CounterAction::Step,
        })),

        "update" => {
            // Extrair o valor numérico do primeiro argumento.
            // Defensivo: se o argumento não for Int, usar 0 silenciosamente.
            let val = args.items().next()
                .and_then(|arg| match arg {
                    Arg::Pos(expr) => {
                        if let Ok(Value::Int(n)) = eval_expr(expr, scopes, ctx, route, styles, show_rules, active_guards, current_file, figure_numbering, sink) {
                            Some(n.max(0) as usize)
                        } else {
                            None
                        }
                    }
                    _ => None,
                })
                .unwrap_or(0);
            Ok(Value::Content(Content::CounterUpdate {
                key:    key.to_string(),
                action: CounterAction::Update(val),
            }))
        }

        // get(), display() e outros — fallback até motor de introspecção completo
        _ => Ok(Value::Content(Content::CounterDisplay {
            kind: key.to_string(),
        })),
    }
}

// ── Dispatcher arms: FieldAccess (Passo 96.2, ADR-0037 Regra 4) ───────────

pub(super) fn eval_field_access<'r>(
    access: crate::entities::ast::expr::FieldAccess<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext<'_>,
    route: Tracked<'r, Route<'r>>,
    styles: &mut StyleChain,
    show_rules: &mut Arc<[ShowRule]>,
    active_guards: &mut Vec<RuleId>,
    current_file: FileId,
    figure_numbering: &mut Option<String>,
    sink: &mut TrackedMut<'_, Sink>,
) -> SourceResult<Value> {
    use crate::entities::ast::AstNode;
    use crate::entities::source_result::SourceDiagnostic;

    let target = eval_expr(access.target(), scopes, ctx, route, styles, show_rules, active_guards, current_file, figure_numbering, sink)?;
    let field  = access.field().as_str().to_string();
    match target {
        Value::Dict(d) => d.get(field.as_str())
            .cloned()
            .ok_or_else(|| vec![SourceDiagnostic::error(
                access.span(),
                format!("campo '{field}' não existe"),
            )]),
        // Field access em elementos estruturados — usado por show rules (Passo 68).
        // Ex: `it.body` onde `it` é Content::Heading retorna Value::Content(body).
        Value::Content(c) => c.get_field(field.as_str())
            .ok_or_else(|| vec![SourceDiagnostic::error(
                access.span(),
                format!("campo '{field}' não existe neste elemento de conteúdo"),
            )]),
        other => Err(vec![SourceDiagnostic::error(
            access.span(),
            format!("field access não suportado em {}", other.type_name()),
        )]),
    }
}
