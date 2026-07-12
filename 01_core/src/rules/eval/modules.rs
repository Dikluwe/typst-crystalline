//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/eval.md
//! @prompt-hash 824cf31b
//! @layer L1
//! @updated 2026-04-23
//!
//! Armos `ModuleImport` e `ModuleInclude` do eval — detecção de ciclos e
//! recursão em Route. Extraído do dispatcher no Passo 96.2 conforme
//! ADR-0037 Regra 4. Assinaturas simplificadas no Passo 109 (ADR-0044)
//! via `Engine<'_>`.

use std::str::FromStr;
use std::sync::Arc;

use comemo::{Track, TrackedMut};

use crate::entities::ast::code::{Imports, ModuleImport, ModuleInclude};
use crate::entities::ast::expr::Expr;
use crate::entities::ast::AstNode;
use crate::entities::engine::Engine;
use crate::entities::func::Func;
use crate::entities::module::Module;
use crate::entities::package_spec::PackageSpec;
use crate::entities::show::{RuleId, ShowRule};
use crate::entities::source::Source;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::style_chain::StyleChain;
use crate::entities::value::Value;
use crate::entities::world_types::Route;
use crate::rules::scopes::Scopes;

use super::{eval_expr, eval_markup, EvalContext};

/// Avalia um ficheiro importado num **módulo isolado** e devolve o seu [`Module`].
///
/// O ficheiro é avaliado com scope base próprio (stdlib + cores predefinidas +
/// `text`) e um `Engine`/`EvalContext` locais, de modo que `#set`/`#show` e o
/// conteúdo de markup do ficheiro importado não vazam para o importador — só os
/// bindings (`#let`/`#fn`) são expostos via `Module::scope()`. Espelha o
/// `run_pass` do eval principal (Passo 109, ADR-0044), mas numa única passagem
/// (o conteúdo é descartado). Ver `00_nucleo/prompts/rules/eval.md` §P679.
fn eval_imported_file(
    source: &Source,
    name: &str,
    ctx: &EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Module> {
    let src_id = source.id();

    // Scope base do módulo importado — paridade com o eval principal: o ficheiro
    // importado vê a stdlib (ex.: `range(3)`), as cores predefinidas e `text`.
    let mut module_scopes = Scopes::new(None);
    // P694 — `make_stdlib` precisa de `SysInputs` (vêm do `World`); módulos
    // importados vêem os mesmos `sys.inputs` do documento principal.
    let stdlib = super::make_stdlib(&engine.world.inputs());
    // P709 — `std` também tem de existir aqui: um ficheiro importado (ex.
    // `cetz`) que sombreia um builtin (`#let length = ...`) precisa de
    // `std.length` para aceder à versão não-sombreada, exactamente como o
    // documento principal (`eval/mod.rs::run_pass`). Mesmo mecanismo — clone
    // tirado antes de `stdlib` ser espalhado neste scope.
    module_scopes.define("std", Value::Module(Module::new("std", stdlib.clone())));
    for (n, binding) in stdlib.iter() {
        module_scopes.define(n, binding.value().clone());
    }
    for (n, value) in crate::rules::stdlib::predefined_color_bindings() {
        module_scopes.define(n.as_str(), value);
    }
    module_scopes.define(
        "text",
        Value::Func(Func::native("text", crate::rules::stdlib::native_text)),
    );
    module_scopes.enter(); // âmbito do módulo importado

    // Frame filho: segmento de rota com o `id` do ficheiro importado.
    let local_route = Route::extend(engine.route).with_id(src_id);

    // Engine local isolado: estilos/show-rules/guards próprios (não partilhados
    // com o importador). O `sink` é reborrowed para que warnings do ficheiro
    // importado cheguem ao caller. Quando a chamada retorna, o Engine do
    // chamador permanece intacto.
    let mut styles = StyleChain::default_chain();
    let mut show_rules: Arc<[ShowRule]> = Arc::from([]);
    let mut active_guards: Vec<RuleId> = Vec::new();
    let mut local_sink = TrackedMut::reborrow_mut(&mut *engine.sink);
    let mut local_engine = Engine {
        world: engine.world,
        route: local_route.track(),
        styles: &mut styles,
        show_rules: &mut show_rules,
        active_guards: &mut active_guards,
        current_file: src_id,
        sink: &mut local_sink,
    };

    // Contexto local: herda `full_error`, mas corre sem aplicar show-rules — o
    // conteúdo é descartado e só os bindings interessam.
    let mut local_ctx = EvalContext::new();
    local_ctx.full_error = ctx.full_error;
    local_ctx.apply_show_rules = false;

    eval_markup(source.root(), &mut module_scopes, &mut local_ctx, &mut local_engine)?;
    if let Some(flow) = local_ctx.flow {
        return Err(vec![flow.forbidden()]);
    }

    let module_scope = module_scopes.exit();
    Ok(Module::new(name.to_string(), module_scope))
}

pub(super) fn eval_module_import(
    import: ModuleImport<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    // Resolver a fonte do import para um `Module` + nome de ligação por omissão:
    //  - `Expr::Str` → ficheiro local (P679) ou pacote `@preview` (P681), avaliado
    //    num módulo isolado (ciclo + `eval_imported_file`); nome = `bare_name()`.
    //  - qualquer outra expressão que avalie para `Value::Module` (P683) — ex.:
    //    `#import util: x` (identificador) ou `#import deps.oxifmt: strfmt`
    //    (field-access sobre um módulo já ligado) → usa o módulo directamente;
    //    nome = `Module::name()`.
    let source_expr = import.source();
    let source_span = source_expr.span();

    let (module, default_bind_name): (Module, String) = match source_expr {
        Expr::Str(s) => {
            let path = s.get()?;

            // Para pacotes (@preview/...) a resolução (cache local, manifesto
            // `typst.toml`, entrypoint) é delegada a `world.resolve_package`
            // (P681; I/O em L3); para ficheiros locais, `world.include_source`
            // (P679). Em ambos o resultado é um `Source` pronto a avaliar.
            let source = if path.starts_with('@') {
                let spec = PackageSpec::from_str(&path)
                    .map_err(|e| vec![SourceDiagnostic::error(source_span, e.to_string())])?;
                engine
                    .world
                    .resolve_package(&spec)
                    .map_err(|msg| vec![SourceDiagnostic::error(Span::detached(), msg)])?
            } else {
                engine
                    .world
                    .include_source(engine.current_file, &path)
                    .map_err(|msg| vec![SourceDiagnostic::error(Span::detached(), msg)])?
            };
            let src_id = source.id();

            // Detecção de ciclo via `Route::contains` (ADR-0033, ADR-0036).
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

            // Nome do módulo (file_stem) — usado em `Module::new` e no bare import.
            let module_name = import.bare_name().map_err(|_| {
                vec![SourceDiagnostic::error(
                    source_span,
                    "module name would not be a valid identifier",
                )]
            })?;

            let module = eval_imported_file(&source, &module_name, ctx, engine)?;
            (module, module_name)
        }
        _ => {
            // P683 — fonte é uma expressão: avalia no scope do chamador e exige
            // `Value::Module` (identificador de módulo já ligado, ou field-access
            // que resolve para outro módulo).
            let value = eval_expr(source_expr, scopes, ctx, engine)?;
            match value {
                Value::Module(m) => {
                    let name = m.name().to_string();
                    (m, name)
                }
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        source_span,
                        format!(
                            "import: a fonte tem de ser um caminho string ou um módulo, recebeu {}",
                            other.type_name()
                        ),
                    )]);
                }
            }
        }
    };

    // 7. Aplicar bindings ao scope do chamador conforme a forma do import.
    match import.imports() {
        None => {
            // Bare import: liga o módulo sob `new_name` (`as`) ou o nome por
            // omissão (`bare_name` para ficheiro/pacote; `Module::name()` para
            // fonte-módulo, P683).
            let bind = import
                .new_name()
                .map(|i| i.get().to_string())
                .unwrap_or_else(|| default_bind_name.clone());
            scopes.define(&bind, Value::Module(module));
        }
        Some(Imports::Wildcard) => {
            for (name, binding) in module.scope().iter() {
                scopes.define(name, binding.value().clone());
            }
        }
        Some(Imports::Items(items)) => {
            for item in items.iter() {
                let orig = item.original_name().get().to_string();
                let bound = item.bound_name().get().to_string();
                let value = module.scope().get(&orig).cloned().ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        item.original_name().span(),
                        format!("unresolved import: `{}`", orig),
                    )]
                })?;
                scopes.define(&bound, value);
            }
        }
    }

    Ok(Value::None)
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
        sink: &mut local_sink,
    };
    eval_markup(source.root(), scopes, ctx, &mut local_engine)
}
