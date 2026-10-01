//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/call_dispatch/content_methods.md
//! @prompt-hash 7af86074
//! @layer L1
//! @updated 2026-10-01

use super::*;

/// Intercepta Location, Content e OutlineEntry após os demais métodos.
pub(super) fn try_content_methods(
    call: FuncCallNode<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Option<Value>> {
    // **P792** — Métodos de `Location`: `loc.page()`, `loc.position()`,
    // `loc.page-numbering()` — paridade vanilla `location.rs #[scope]`.
    if let Expr::FieldAccess(access) = call.callee() {
        let method = access.field().as_str();
        if matches!(method, "page" | "position" | "page-numbering") {
            let target = eval_expr(access.target(), scopes, ctx, engine)?;
            if let Value::Location(loc) = target {
                let _args = eval_args(call.args(), scopes, ctx, engine)?;
                return eval_location_method(loc, method, ctx, call.span()).map(Some);
            }
        }
    }

    // **P829-B** — Métodos de instância de `Value::Content`:
    // `func`/`has`/`at`/`fields`/`location` — paridade vanilla `Content`
    // `#[scope]` (`foundations/content/mod.rs:510-590`). Corre depois de
    // todos os despachos legítimos acima (nenhum intercepta estes nomes para
    // Content) e antes do fallback P815, que de outra forma reportaria
    // `element strong has no method `func`` (medido m14/b1).
    if let Expr::FieldAccess(access) = call.callee() {
        let method = access.field().as_str();
        if matches!(method, "func" | "has" | "at" | "fields" | "location") {
            let target = eval_expr(access.target(), scopes, ctx, engine)?;
            if let Value::Content(c) = target {
                let args = eval_args(call.args(), scopes, ctx, engine)?;
                return bindings::eval_content_method(&c, method, args, call.span())
                    .map(Some);
            }
            if let Value::LocatedContent(c, loc) = target {
                let args = eval_args(call.args(), scopes, ctx, engine)?;
                return bindings::eval_introspected_content_method_at(
                    &c,
                    loc,
                    method,
                    args,
                    call.span(),
                )
                .map(Some);
            }
        }
    }

    // P1402/P1403 — métodos próprios da entry.
    if let Expr::FieldAccess(access) = call.callee() {
        let method = access.field().as_str();
        if matches!(method, "body" | "prefix" | "page" | "inner" | "indented") {
            let target = eval_expr(access.target(), scopes, ctx, engine)?;
            let content = match &target {
                Value::Content(content) => Some(content),
                Value::LocatedContent(content, _) => Some(content.content()),
                _ => None,
            };
            if let Some(content @ crate::entities::content::Content::OutlineEntry(_)) =
                content
            {
                let mut args = eval_args(call.args(), scopes, ctx, engine)?;
                if method == "body" {
                    return bindings::eval_outline_entry_body_method(
                        content,
                        args,
                        call.span(),
                    )
                    .map(Some);
                }
                args.items.insert(0, Value::Content(content.clone()));
                args.occurrences = None;
                let world = engine.world;
                let current_file = engine.current_file;
                return match method {
                    "prefix" => {
                        native_outline_entry_prefix(ctx, &args, world, current_file)
                    }
                    "page" => native_outline_entry_page(
                        ctx,
                        &args,
                        world,
                        current_file,
                        scopes,
                        engine,
                    ),
                    "inner" => native_outline_entry_inner(
                        ctx,
                        &args,
                        world,
                        current_file,
                        scopes,
                        engine,
                    ),
                    "indented" => native_outline_entry_indented(
                        ctx,
                        &args,
                        world,
                        current_file,
                        scopes,
                        engine,
                    ),
                    _ => unreachable!("outline entry method already checked"),
                }
                .map(Some);
            }
        }
    }

    Ok(None)
}
