//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/eval.md
//! @prompt-hash 19073424
//! @layer L1
//! @updated 2026-04-23
//!
//! Armos `ModuleImport` e `ModuleInclude` do eval — detecção de ciclos e
//! recursão em Route. Extraído do dispatcher no Passo 96.2 conforme
//! ADR-0037 Regra 4. Assinaturas simplificadas no Passo 109 (ADR-0044)
//! via `Engine<'_>`.

use comemo::TrackedMut;

use crate::entities::ast::code::{ModuleImport, ModuleInclude};
use crate::entities::ast::AstNode;
use crate::entities::engine::Engine;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;
use crate::entities::world_types::Route;
use crate::rules::scopes::Scopes;

use super::{eval_expr, eval_markup, EvalContext};

pub(super) fn eval_module_import(import: ModuleImport<'_>) -> SourceResult<Value> {
    // import não implementado — Passo 33+
    Err(vec![SourceDiagnostic::error(
        import.span(),
        "import não implementado nesta versão do cristalino",
    )])
}

pub(super) fn eval_module_include(
    include: ModuleInclude<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    // Avaliar a expressão do caminho (normalmente uma string literal).
    let path_val = eval_expr(include.source(), scopes, ctx, engine)?;
    let path = match path_val {
        Value::Str(s) => s.to_string(),
        other => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("include: caminho deve ser string, recebeu {}", other.type_name()),
        )]),
    };

    // Carregar o ficheiro incluído com resolução relativa ao ficheiro actual.
    let source = engine.world.include_source(engine.current_file, &path)
        .map_err(|msg| vec![SourceDiagnostic::error(Span::detached(), msg)])?;

    let src_id = source.id();
    // Detecção de ciclo via `Route::contains` real (ADR-0033, ADR-0036).
    if engine.route.contains(src_id) {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!(
                "ciclo de importação detectado: ficheiro {:?} já está \
                 na cadeia de avaliação activa",
                src_id
            ),
        )]);
    }

    // Frame filho: segmento de rota com `id` do módulo incluído.
    let child_route = Route::extend(engine.route).with_id(src_id);

    // Engine local com `route` e `current_file` novos (Passo 109, ADR-0044).
    // Quando a chamada retorna (sucesso ou erro via `?`), o Engine do
    // chamador permanece intacto.
    let mut local_sink = TrackedMut::reborrow_mut(&mut *engine.sink);
    let mut local_engine = Engine {
        world: engine.world,
        route: child_route.track(),
        styles: &mut *engine.styles,
        show_rules: &mut *engine.show_rules,
        active_guards: &mut *engine.active_guards,
        current_file: src_id,
        figure_numbering: &mut *engine.figure_numbering,
        sink: &mut local_sink,
    };
    eval_markup(source.root(), scopes, ctx, &mut local_engine)
}
