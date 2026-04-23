//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/eval.md
//! @prompt-hash 19073424
//! @layer L1
//! @updated 2026-04-22
//!
//! Armos `ModuleImport` e `ModuleInclude` do eval — detecção de ciclos e
//! recursão em Route. Extraído do dispatcher no Passo 96.2 conforme
//! ADR-0037 Regra 4.

use std::sync::Arc;

use comemo::{Tracked, TrackedMut};

use crate::entities::ast::code::{ModuleImport, ModuleInclude};
use crate::entities::file_id::FileId;
use crate::entities::ast::AstNode;
use crate::entities::show::{RuleId, ShowRule};
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::style_chain::StyleChain;
use crate::entities::value::Value;
use crate::entities::world_types::{Route, Sink};
use crate::rules::scopes::Scopes;

use super::{eval_expr, eval_markup, EvalContext};

pub(super) fn eval_module_import(import: ModuleImport<'_>) -> SourceResult<Value> {
    // import não implementado — Passo 33+
    // A estrutura de detecção de ciclos (EvalContext::enter_import)
    // está pronta para uso quando import for implementado.
    Err(vec![SourceDiagnostic::error(
        import.span(),
        "import não implementado nesta versão do cristalino",
    )])
}

pub(super) fn eval_module_include<'r>(
    include: ModuleInclude<'_>,
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
    // Avaliar a expressão do caminho (normalmente uma string literal).
    let path_val = eval_expr(include.source(), scopes, ctx, route, styles, show_rules, active_guards, current_file, figure_numbering, sink)?;
    let path = match path_val {
        Value::Str(s) => s.to_string(),
        other => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("include: caminho deve ser string, recebeu {}", other.type_name()),
        )]),
    };

    // Carregar o ficheiro incluído com resolução relativa ao ficheiro actual.
    let source = ctx.world.include_source(current_file, &path)
        .map_err(|msg| vec![SourceDiagnostic::error(Span::detached(), msg)])?;

    let src_id = source.id();
    // Detecção de ciclo via `Route::contains` real (ADR-0033, ADR-0036).
    // Paridade directa com o vanilla (`typst-eval/src/import.rs:232`).
    if route.contains(src_id) {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!(
                "ciclo de importação detectado: ficheiro {:?} já está \
                 na cadeia de avaliação activa",
                src_id
            ),
        )]);
    }

    // Frame filho: segmento de rota com `id` do módulo incluído. A
    // cadeia de `outer` percorre via `Tracked<'_, Route>`.
    let child_route = Route::extend(route).with_id(src_id);

    // Passo 98 (ADR-0036): `current_file` deixou de ser campo mutável do ctx.
    // O novo valor viaja como variável local — quando a chamada retorna
    // (sucesso ou erro via `?`), o `current_file` do chamador permanece
    // intacto porque nunca foi tocado. Elimina o bug latente do padrão
    // save/set/restore: se `eval_markup` retornasse `Err`, o restore não
    // corria e o campo ficava corrompido.
    let child_current_file = src_id;
    eval_markup(source.root(), scopes, ctx, child_route.track(), styles, show_rules, active_guards, child_current_file, figure_numbering, sink)
}
