//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/modules.md
//! @prompt-hash 31912dd7
//! @layer L1
//! @updated 2026-07-16
//!
//! Armos `ModuleImport` e `ModuleInclude` do eval — detecção de ciclos e
//! recursão em Route. Extraído do dispatcher no Passo 96.2 conforme
//! ADR-0037 Regra 4. Assinaturas simplificadas no Passo 109 (ADR-0044)
//! via `Engine<'_>`.

use std::str::FromStr;
use std::sync::Arc;

use comemo::TrackedMut;

use crate::compiler::scopes::Scopes;
use crate::entities::ast::code::{Imports, ModuleImport, ModuleInclude};
use crate::entities::ast::expr::Expr;
use crate::entities::ast::AstNode;
use crate::entities::engine::Engine;
use crate::entities::func::Func;
use crate::entities::module::Module;
use crate::entities::package_spec::PackageSpec;
use crate::entities::scope::Scope;
use crate::entities::show::{RuleId, ShowRule};
use crate::entities::source::Source;
use crate::entities::source_result::{SourceDiagnostic, SourceResult, Tracepoint};
use crate::entities::span::{Span, Spanned};
use crate::entities::style_chain::StyleChain;
use crate::entities::value::Value;
use crate::entities::world_types::{Library, Route};

use super::{eval_expr, eval_markup, EvalContext};

/// Avalia um ficheiro sintaticamente validado num módulo isolado.
///
/// O ficheiro é avaliado com scope base próprio (stdlib + cores predefinidas +
/// `text`) e um `Engine`/`EvalContext` locais, de modo que `#set`/`#show` e o
/// conteúdo de markup do ficheiro importado não vazam para o importador — só os
/// bindings (`#let`/`#fn`) são expostos via `Module::scope()`. Espelha o
/// `run_pass` do eval principal (Passo 109, ADR-0044), mas numa única passagem
/// (o conteúdo é descartado). Ver `00_nucleo/prompts/compiler/eval.md` §P679.
fn eval_imported_file(
    source: &Source,
    name: &str,
    ctx: &EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Module> {
    let src_id = source.id();

    // Scope base do módulo importado — paridade com o eval principal: o ficheiro
    // importado vê a stdlib (ex.: `range(3)`), as cores predefinidas e `text`.
    // P772n — mesmo mecanismo de `eval/mod.rs::run_pass`: `global` embrulhado
    // em `Library`, passado como `base` (só leitura, nunca mutável) — em vez
    // de achatado em `module_scopes.top`. Ver `rules/eval.md` §P772n.
    let mut global = Scope::new();
    // P694 — `make_stdlib` precisa de `SysInputs` (vêm do `World`); módulos
    // importados vêem os mesmos `sys.inputs` do documento principal.
    let stdlib = super::make_stdlib_with_features(&engine.world.inputs(), ctx.features);
    // P709 — `std` também tem de existir aqui: um ficheiro importado (ex.
    // `cetz`) que sombreia um builtin (`#let length = ...`) precisa de
    // `std.length` para aceder à versão não-sombreada, exactamente como o
    // documento principal (`eval/mod.rs::run_pass`). Mesmo mecanismo — clone
    // tirado antes de `stdlib` ser espalhado neste scope.
    global.define("std", Value::Module(Module::stdlib(stdlib.clone())));
    for (n, binding) in stdlib.iter() {
        global.define(n, binding.value().clone());
    }
    for (n, value) in crate::compiler::stdlib::predefined_color_bindings() {
        global.define(n.as_str(), value);
    }
    global.define(
        "text",
        Value::Func(Func::native("text", crate::compiler::stdlib::native_text)),
    );
    let library = Library::with_global(global);
    let mut module_scopes = Scopes::new(Some(&library));

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
        font_metrics: engine.font_metrics,
        route: local_route.track(),
        styles: &mut styles,
        show_rules: &mut show_rules,
        active_guards: &mut active_guards,
        current_file: src_id,
        sink: &mut local_sink,
    };

    // Contexto local: herda `full_error`, features e target, mas corre sem
    // aplicar show-rules: o conteúdo é descartado e só os bindings interessam.
    let mut local_ctx = EvalContext::new();
    local_ctx.full_error = ctx.full_error;
    local_ctx.features = ctx.features;
    local_ctx.target = ctx.target;
    local_ctx.apply_show_rules = false;

    eval_markup(source.root(), &mut module_scopes, &mut local_ctx, &mut local_engine)?;
    if let Some(flow) = local_ctx.flow {
        return Err(vec![flow.forbidden()]);
    }

    let module_scope = module_scopes.exit();
    Ok(Module::new(name.to_string(), module_scope))
}

fn trace_source_errors(
    mut errors: Vec<SourceDiagnostic>,
    point: Tracepoint,
    span: Span,
) -> Vec<SourceDiagnostic> {
    for error in &mut errors {
        error.trace.push(Spanned::new(point.clone(), span));
    }
    errors
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
    //    ligação bare = nome lexical da AST, não o nome público do objeto.
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
                let spec = PackageSpec::from_str(&path).map_err(|e| {
                    vec![SourceDiagnostic::error(source_span, e.to_string())]
                })?;
                engine
                    .world
                    .resolve_package(&spec)
                    .map_err(|msg| vec![SourceDiagnostic::error(source_span, msg)])?
            } else {
                engine
                    .world
                    .include_source(engine.current_file, &path)
                    .map_err(|msg| vec![SourceDiagnostic::error(source_span, msg)])?
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

            super::syntax_diagnostics::validate_syntax(source.root(), |diagnostic| {
                engine.sink.warn_diagnostic(diagnostic);
            })
            .map_err(|errors| {
                trace_source_errors(
                    errors,
                    Tracepoint::Import(path.to_string()),
                    import.span(),
                )
            })?;

            // Nome do módulo (file_stem) — usado em `Module::new` e no bare import.
            let module_name = import.bare_name().map_err(|_| {
                vec![SourceDiagnostic::error(
                    source_span,
                    "module name would not be a valid identifier",
                )]
            })?;

            let module = eval_imported_file(&source, &module_name, ctx, engine).map_err(
                |errors| {
                    trace_source_errors(
                        errors,
                        Tracepoint::Import(path.to_string()),
                        import.span(),
                    )
                },
            )?;
            (module, module_name)
        }
        _ => {
            // P683 — fonte é uma expressão: avalia no scope do chamador e exige
            // `Value::Module` (identificador de módulo já ligado, ou field-access
            // que resolve para outro módulo).
            let value = eval_expr(source_expr, scopes, ctx, engine)?;
            match value {
                Value::Module(m) => {
                    let name =
                        if import.new_name().is_none() && import.imports().is_none() {
                            import.bare_name().map_err(|_| {
                                vec![SourceDiagnostic::error(
                                    source_span,
                                    "dynamic import requires an explicit name",
                                )
                                .with_hint("you can name the import with `as`")]
                            })?
                        } else {
                            m.name().to_string()
                        };
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
            if import.new_name().is_none() && matches!(source_expr, Expr::Ident(_)) {
                engine.sink.warn_note(source_span, "this import has no effect", "");
            }
            // Bare import: liga o módulo sob `new_name` (`as`) ou o nome por
            // omissão (`bare_name` lexical, independente do nome do objeto).
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
    let path_source = include.source();
    let path_span = path_source.span();
    let path_val = eval_expr(path_source, scopes, ctx, engine)?;
    let trace_path = match &path_val {
        Value::Str(path) => path.clone(),
        Value::Path(path) => path.vpath().get_with_slash().into(),
        _ => "<invalid path>".into(),
    };
    let source = match path_val {
        Value::Str(s) => engine
            .world
            .include_source(engine.current_file, &s)
            .map_err(|msg| vec![SourceDiagnostic::error(path_span, msg)])?,
        Value::Path(path) => engine
            .world
            .include_path(&path)
            .map_err(|msg| vec![SourceDiagnostic::error(path_span, msg)])?,
        other => {
            return Err(vec![SourceDiagnostic::error(
                path_span,
                format!(
                    "include: caminho deve ser path ou string, recebeu {}",
                    other.type_name()
                ),
            )]);
        }
    };

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

    super::syntax_diagnostics::validate_syntax(source.root(), |diagnostic| {
        engine.sink.warn_diagnostic(diagnostic);
    })
    .map_err(|errors| {
        trace_source_errors(
            errors,
            Tracepoint::Include(trace_path.to_string()),
            include.span(),
        )
    })?;

    // Frame filho: segmento de rota com `id` do módulo incluído.
    let child_route = Route::extend(engine.route).with_id(src_id);

    // Engine local com `route` e `current_file` novos (Passo 109, ADR-0044).
    // Quando a chamada retorna (sucesso ou erro via `?`), o Engine do
    // chamador permanece intacto.
    let mut local_sink = TrackedMut::reborrow_mut(&mut *engine.sink);
    let mut local_engine = Engine {
        world: engine.world,
        font_metrics: engine.font_metrics,
        route: child_route.track(),
        styles: &mut *engine.styles,
        show_rules: &mut *engine.show_rules,
        active_guards: &mut *engine.active_guards,
        current_file: src_id,
        sink: &mut local_sink,
    };
    eval_markup(source.root(), scopes, ctx, &mut local_engine).map_err(|errors| {
        trace_source_errors(
            errors,
            Tracepoint::Include(trace_path.to_string()),
            include.span(),
        )
    })
}

#[cfg(test)]
mod import_syntax_tests {
    use super::*;
    use crate::compiler::eval::{eval_with_full_error_target_and_features, EvalTarget};
    use crate::contracts::world::World;
    use crate::entities::compiler_features::{Feature, Features};
    use crate::entities::element_registry::ElementRegistry;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::sink::Sink;
    use crate::entities::source_result::Severity;
    use crate::entities::syntax_node::SyntaxErrorKind;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Routines, Traced,
    };
    use comemo::Track;
    use std::num::NonZeroU16;
    struct ImportMockWorld {
        library: Library,
        book: FontBook,
        main: Source,
        files: std::collections::HashMap<String, Source>,
    }

    impl ImportMockWorld {
        fn new(main_text: &str, files: &[(&str, &str)]) -> Self {
            let main_id = FileId::from_raw(NonZeroU16::new(1).unwrap());
            let mut map = std::collections::HashMap::new();
            let mut next = 2u16;
            for (path, text) in files {
                let id = FileId::from_raw(NonZeroU16::new(next).unwrap());
                next += 1;
                map.insert(path.to_string(), Source::new(id, text.to_string()));
            }
            Self {
                library: Library::new(),
                book: FontBook::new(),
                main: Source::new(main_id, main_text.to_string()),
                files: map,
            }
        }
    }

    impl World for ImportMockWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            self.main.id()
        }
        fn source(&self, id: FileId) -> FileResult<Source> {
            if id == self.main.id() {
                Ok(self.main.clone())
            } else {
                self.files
                    .values()
                    .find(|s| s.id() == id)
                    .cloned()
                    .ok_or(FileError::NotFound)
            }
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<Font> {
            None
        }
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<Datetime> {
            None
        }
        fn include_source(
            &self,
            _current_file: FileId,
            path: &str,
        ) -> Result<Source, String> {
            self.files
                .get(path)
                .cloned()
                .ok_or_else(|| format!("ficheiro não encontrado: {}", path))
        }

        fn resolve_path(
            &self,
            _: FileId,
            path: &str,
        ) -> Result<crate::entities::path::RootedPath, String> {
            Ok(crate::entities::path::RootedPath::new(
                crate::entities::path::VirtualRoot::Project,
                crate::entities::path::VirtualPath::new(path)
                    .map_err(|e| format!("{e:?}"))?,
            ))
        }
        fn include_path(
            &self,
            path: &crate::entities::path::RootedPath,
        ) -> Result<Source, String> {
            self.include_source(
                self.main(),
                path.vpath().get_with_slash().trim_start_matches('/'),
            )
        }
        fn resolve_package(
            &self,
            _: &crate::entities::package_spec::PackageSpec,
        ) -> Result<Source, String> {
            self.include_source(self.main(), "bad.typ")
        }
    }

    fn run(
        world: &ImportMockWorld,
        features: Features,
        target: EvalTarget,
    ) -> (SourceResult<Module>, Vec<SourceDiagnostic>) {
        let mut sink = Sink::new();
        let result = eval_with_full_error_target_and_features(
            &Routines::new(),
            world,
            Traced::default().track(),
            sink.track_mut(),
            Route::root().track(),
            &world.main,
            &ElementRegistry::new(),
            false,
            target,
            features,
        );
        (result, sink.into_diagnostics())
    }
    fn features(mask: u8) -> Features {
        let mut features = Features::empty();
        for (bit, flag) in
            [(1, Feature::Html), (2, Feature::A11yExtras), (4, Feature::Bundle)]
        {
            if mask & bit != 0 {
                features.enable(flag);
            }
        }
        features
    }
    fn hard_errors(world: &ImportMockWorld) -> Vec<SourceDiagnostic> {
        world.files["bad.typ"]
            .root()
            .errors()
            .into_iter()
            .filter(|error| {
                !matches!(
                    error.kind,
                    SyntaxErrorKind::NoTextWithinStars
                        | SyntaxErrorKind::NoTextWithinUnderscores
                )
            })
            .map(|error| {
                let mut d =
                    SourceDiagnostic::error(error.span, error.message.to_string());
                d.hints = error.hints.iter().map(ToString::to_string).collect();
                d
            })
            .collect()
    }
    fn check_invalid(text: &str, bad: &str, trace: &[&str]) {
        let world = ImportMockWorld::new(
            text,
            &[("bad.typ", bad), ("nested.typ", "#import \"bad.typ\" as child")],
        );
        let expected = hard_errors(&world);
        assert!(!expected.is_empty());
        let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            run(&world, Features::empty(), EvalTarget::Paged)
        }));
        let (result, _) =
            outcome.unwrap_or_else(|_| panic!("invalid source panicked: {text}/{bad}"));
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), expected.len(), "{text}/{bad}: {errors:?}");
        for (actual, expected) in errors.iter().zip(expected) {
            assert_eq!(actual.severity, Severity::Error);
            assert_eq!(actual.message, expected.message);
            assert_eq!(actual.span, expected.span);
            assert_eq!(actual.hints, expected.hints);
            assert!(world.files["bad.typ"].span_byte_range(actual.span).is_some());
            assert_eq!(actual.trace.len(), trace.len(), "{text}: {:?}", actual.trace);
            for (point, name) in actual.trace.iter().zip(trace) {
                match &point.v {
                    Tracepoint::Import(path) | Tracepoint::Include(path) => {
                        assert_eq!(path, name)
                    }
                    other => panic!("unexpected trace {other:?}"),
                }
                assert!(!point.span.is_detached());
            }
        }
    }

    #[test]
    fn import_syntax_rejects_malformed_alias_items_and_wildcards_without_panic() {
        for bad in [
            "#let or(x)=x",
            "#let x = \"unterminated",
            "#let x =",
            "#let a = 0xzz\n#let b = \"\\u{110000}\"",
        ] {
            for text in [
                "#import \"bad.typ\" as sample",
                "#import \"bad.typ\": x",
                "#import \"bad.typ\": *",
            ] {
                check_invalid(text, bad, &["bad.typ"]);
            }
        }
    }
    #[test]
    fn import_syntax_include_string_path_and_nested_preserve_error_origin() {
        for text in ["#include \"bad.typ\"", "#include path(\"bad.typ\")"] {
            check_invalid(
                text,
                "#let x = \"unterminated",
                &[if text.contains("path(") { "/bad.typ" } else { "bad.typ" }],
            );
        }
        check_invalid(
            "#import \"nested.typ\" as outer",
            "#let or(x)=x",
            &["bad.typ", "nested.typ"],
        );
        check_invalid(
            "#include \"nested.typ\"",
            "#let or(x)=x",
            &["bad.typ", "nested.typ"],
        );
    }
    #[test]
    fn import_syntax_package_source_is_validated_with_package_trace() {
        check_invalid(
            "#import \"@preview/test:0.1.0\" as sample",
            "#let or(x)=x",
            &["@preview/test:0.1.0"],
        );
    }
    #[test]
    fn import_syntax_parse_error_precedes_prefix_effect_and_preserves_warnings() {
        check_invalid(
            "#import \"bad.typ\" as sample",
            "#panic(\"prefix executed\")\n#let or(x)=x",
            &["bad.typ"],
        );
        let world = ImportMockWorld::new(
            "#import \"bad.typ\" as sample",
            &[("bad.typ", "**\n#let or(x)=x")],
        );
        let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            run(&world, Features::empty(), EvalTarget::Paged)
        }));
        let (result, warnings) = outcome.expect("parser diagnostics must not panic");
        assert!(result.is_err());
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].severity, Severity::Warning);
        assert_eq!(world.files["bad.typ"].span_byte_range(warnings[0].span), Some(0..2));
        assert!(!warnings[0].hints.is_empty());
    }
    #[test]
    fn import_syntax_empty_markup_warnings_are_nonfatal_and_deduplicated() {
        for text in [
            "#import \"bad.typ\" as a\n#import \"bad.typ\" as b",
            "#include \"bad.typ\"\n#include \"bad.typ\"",
        ] {
            let world = ImportMockWorld::new(text, &[("bad.typ", "** __\n#let x=42")]);
            let (result, warnings) = run(&world, Features::empty(), EvalTarget::Paged);
            result.unwrap();
            assert_eq!(warnings.len(), 2, "{warnings:?}");
            for (warning, range) in warnings.iter().zip([0..2, 3..5]) {
                assert_eq!(warning.severity, Severity::Warning);
                assert_eq!(
                    world.files["bad.typ"].span_byte_range(warning.span),
                    Some(range)
                );
                assert!(!warning.hints.is_empty());
                assert!(warning.trace.is_empty());
            }
        }
    }
    #[test]
    fn import_syntax_features_are_inherited_in_all_eight_profiles() {
        for mask in 0..8 {
            for (expr, bit, field, feature) in [
                ("std.html", 1, "html", "html"),
                ("std.pdf.header-cell", 2, "header-cell", "a11y-extras"),
                ("std.asset", 4, "asset", "bundle"),
            ] {
                let text = format!("#let value={expr}");
                let world = ImportMockWorld::new(
                    "#import \"bad.typ\" as sample",
                    &[("bad.typ", &text)],
                );
                let (result, warnings) = run(&world, features(mask), EvalTarget::Paged);
                assert!(warnings.is_empty());
                if mask & bit != 0 {
                    let module = result.unwrap();
                    assert!(matches!(
                        module.scope().get("sample"),
                        Some(Value::Module(_))
                    ));
                } else {
                    assert_eq!(result.unwrap_err()[0].message,format!("cannot access field `{field}` because the `{feature}` feature is not enabled"));
                }
            }
        }
    }
    #[test]
    fn import_syntax_bundle_target_is_inherited_without_enabling_other_targets() {
        for target in [EvalTarget::Paged, EvalTarget::Html, EvalTarget::Bundle] {
            let world = ImportMockWorld::new(
                "#import \"bad.typ\" as sample",
                &[("bad.typ", "#let child=document(\"child.html\")[hello]")],
            );
            let (result, _) = run(&world, features(5), target);
            if target == EvalTarget::Bundle {
                let module = result.unwrap();
                let Some(Value::Module(sample)) = module.scope().get("sample") else {
                    panic!("module binding")
                };
                assert!(matches!(sample.scope().get("child"), Some(Value::Content(_))));
            } else {
                assert_eq!(
                    result.unwrap_err()[0].message,
                    "constructing a document is only supported in the bundle target"
                );
            }
        }
    }
    #[test]
    fn import_syntax_runtime_errors_keep_import_trace_and_valid_exports() {
        let world = ImportMockWorld::new(
            "#import \"bad.typ\" as sample",
            &[("bad.typ", "#panic(\"runtime executed\")")],
        );
        let (result, _) = run(&world, Features::empty(), EvalTarget::Paged);
        let errors = result.unwrap_err();
        assert_eq!(errors[0].message, "panicked with: runtime executed");
        assert!(
            matches!(errors[0].trace.last().map(|t|&t.v),Some(Tracepoint::Import(path)) if path=="bad.typ")
        );
        let world = ImportMockWorld::new(
            "#import \"bad.typ\": x\n#let result=x+1",
            &[("bad.typ", "#let x=42")],
        );
        let (result, warnings) = run(&world, Features::empty(), EvalTarget::Paged);
        assert!(warnings.is_empty());
        assert_eq!(result.unwrap().scope().get("result"), Some(&Value::Int(43)));
    }
}
