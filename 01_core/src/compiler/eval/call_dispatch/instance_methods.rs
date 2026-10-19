//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/call_dispatch/instance_methods.md
//! @prompt-hash 0b22e30c
//! @layer L1
//! @updated 2026-10-01

use super::*;

/// Despacha métodos de instância conhecidos após os métodos mutantes.
pub(super) fn try_instance_method(
    call: FuncCallNode<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    cached_receiver: &mut Option<Value>,
) -> SourceResult<Option<Value>> {
    // **P506** — Métodos de instância para `state` e `counter`.
    if let Expr::FieldAccess(access) = call.callee() {
        let target =
            super::call::resolve_receiver(access, cached_receiver, scopes, ctx, engine)?;
        let method = access.field().as_str();
        match target {
            Value::State(state) => {
                return (super::bindings::eval_state_method(
                    state,
                    method,
                    call.args(),
                    scopes,
                    ctx,
                    engine,
                ))
                .map(Some);
            }
            Value::Counter(counter) => {
                return (super::bindings::eval_counter_method_value(
                    counter,
                    method,
                    call.args(),
                    scopes,
                    ctx,
                    engine,
                ))
                .map(Some);
            }
            Value::Type(kind) => match kind {
                crate::entities::value::Type::Counter => {
                    if matches!(method, "at" | "display") {
                        return (super::bindings::eval_counter_static_method_value(
                            method,
                            call.args(),
                            scopes,
                            ctx,
                            engine,
                        ))
                        .map(Some);
                    }
                }
                crate::entities::value::Type::None
                | crate::entities::value::Type::Auto
                | crate::entities::value::Type::Bool
                | crate::entities::value::Type::Int
                | crate::entities::value::Type::Float
                | crate::entities::value::Type::Str
                | crate::entities::value::Type::Array
                | crate::entities::value::Type::Dictionary
                | crate::entities::value::Type::Module
                | crate::entities::value::Type::Datetime
                | crate::entities::value::Type::Function
                | crate::entities::value::Type::Content
                | crate::entities::value::Type::Length
                | crate::entities::value::Type::Ratio
                | crate::entities::value::Type::Relative
                | crate::entities::value::Type::Angle
                | crate::entities::value::Type::Color
                | crate::entities::value::Type::Stroke
                | crate::entities::value::Type::Fraction
                | crate::entities::value::Type::Alignment
                | crate::entities::value::Type::Location
                | crate::entities::value::Type::Gradient
                | crate::entities::value::Type::Regex
                | crate::entities::value::Type::Tiling
                | crate::entities::value::Type::Bytes
                | crate::entities::value::Type::Decimal
                | crate::entities::value::Type::Duration
                | crate::entities::value::Type::Version
                | crate::entities::value::Type::Selector
                | crate::entities::value::Type::Symbol
                | crate::entities::value::Type::Arguments
                | crate::entities::value::Type::State
                | crate::entities::value::Type::Label
                | crate::entities::value::Type::Direction
                | crate::entities::value::Type::Path
                | crate::entities::value::Type::Type
                | crate::entities::value::Type::SpotColorant => {}
            },
            // **P742** — Métodos de instância de `Value::Color` (9, padrão
            // P506). Só intercepta os métodos conhecidos; os restantes caem
            // no caminho genérico (erro de field access pré-P742).
            Value::SpotColorant(colorant) => {
                if method == "tint" {
                    let args = eval_args(call.args(), scopes, ctx, engine)?;
                    return (crate::compiler::stdlib::color::dispatch_spot_colorant_tint(
                        colorant,
                        args,
                        ctx,
                        engine.world,
                        engine.current_file,
                    ))
                    .map(Some);
                }
            }
            Value::Color(color) => {
                if crate::compiler::stdlib::color::is_color_instance_method(method) {
                    return (super::bindings::eval_color_method(
                        color,
                        method,
                        call.args(),
                        scopes,
                        ctx,
                        engine,
                    ))
                    .map(Some);
                }
            }
            Value::Gradient(gradient) => {
                if crate::compiler::stdlib::gradients::is_gradient_instance_method(method)
                {
                    let args = eval_args(call.args(), scopes, ctx, engine)?;
                    return (crate::compiler::stdlib::gradients::dispatch_gradient_method(
                        gradient,
                        method,
                        args,
                        ctx,
                        engine.world,
                        engine.current_file,
                    )).map(Some);
                }
            }
            Value::Int(value) => {
                if crate::compiler::stdlib::is_int_instance_method(method) {
                    let args = eval_args(call.args(), scopes, ctx, engine)?;
                    return (crate::compiler::stdlib::dispatch_int_method(
                        *value,
                        method,
                        args,
                        ctx,
                        engine.world,
                        engine.current_file,
                    ))
                    .map(Some);
                }
            }
            Value::Float(value) => {
                if crate::compiler::stdlib::is_float_instance_method(method) {
                    let spans =
                        (method == "is-nan").then(|| FloatIsNanCallSpans::capture(call));
                    let mut args = eval_args(call.args(), scopes, ctx, engine)?;
                    if let Some(spans) = spans {
                        args.span = spans.anchor(&args, true);
                    }
                    if matches!(method, "signum" | "from-bytes" | "to-bytes") {
                        args.span = call.span();
                        let result =
                            crate::compiler::stdlib::dispatch_float_method_spanned(
                                *value,
                                access.target().span(),
                                method,
                                args,
                                ctx,
                                engine.world,
                                engine.current_file,
                            );
                        return (trace_call(result, Some(method), call.span(), engine))
                            .map(Some);
                    }
                    return (crate::compiler::stdlib::dispatch_float_method(
                        *value,
                        method,
                        args,
                        ctx,
                        engine.world,
                        engine.current_file,
                    ))
                    .map(Some);
                }
            }
            Value::Angle(angle) => {
                if matches!(method, "deg" | "rad") {
                    let args = eval_args(call.args(), scopes, ctx, engine)?;
                    let mut occurrences = args.occurrence_sequence();
                    occurrences.insert(
                        0,
                        ArgOccurrence {
                            name: None,
                            value: Value::Angle(*angle),
                            span: access.target().span(),
                            value_span: access.target().span(),
                        },
                    );
                    let result = angle_conversion(
                        &Args::from_occurrences(call.span(), occurrences),
                        method == "deg",
                    );
                    return (trace_call(result, Some(method), call.span(), engine))
                        .map(Some);
                }
            }
            Value::Dir(_) => {
                if matches!(method, "axis" | "end" | "inv" | "sign" | "start") {
                    let args = eval_args(call.args(), scopes, ctx, engine)?;
                    return (dispatch_p1284_value_method(
                        target.clone(),
                        method,
                        args,
                        scopes,
                        ctx,
                        engine,
                    ))
                    .map(Some);
                }
            }
            Value::Align(_) => {
                if matches!(method, "axis" | "inv") {
                    let args = eval_args(call.args(), scopes, ctx, engine)?;
                    return (dispatch_p1284_value_method(
                        target.clone(),
                        method,
                        args,
                        scopes,
                        ctx,
                        engine,
                    ))
                    .map(Some);
                }
            }
            Value::Duration(_) => {
                if matches!(method, "days" | "hours" | "minutes" | "seconds" | "weeks") {
                    let args = eval_args(call.args(), scopes, ctx, engine)?;
                    return (dispatch_p1284_value_method(
                        target.clone(),
                        method,
                        args,
                        scopes,
                        ctx,
                        engine,
                    ))
                    .map(Some);
                }
            }
            Value::Length(_) => {
                if matches!(method, "cm" | "inches" | "mm" | "pt" | "to-absolute") {
                    let args = eval_args(call.args(), scopes, ctx, engine)?;
                    return (dispatch_p1284_value_method(
                        target.clone(),
                        method,
                        args,
                        scopes,
                        ctx,
                        engine,
                    ))
                    .map(Some);
                }
            }
            // **P796** — `.at(index)` em `Value::Version` (único método de
            // instância; ver `entities/version.md` §8a). Só intercepta
            // "at"; outros métodos caem no caminho genérico existente.
            Value::Version(v) => {
                if method == "at" {
                    return (super::bindings::eval_version_method_value(
                        v,
                        method,
                        call.args(),
                        call.span(),
                        scopes,
                        ctx,
                        engine,
                    ))
                    .map(Some);
                }
            }
            Value::None
            | Value::Bool(..)
            | Value::Str(..)
            | Value::Array(..)
            | Value::Dict(..)
            | Value::Module(..)
            | Value::Datetime(..)
            | Value::Func(..)
            | Value::Content(..)
            | Value::LocatedContent(..)
            | Value::Auto
            | Value::Relative(..)
            | Value::Ratio(..)
            | Value::Stroke(..)
            | Value::Fraction(..)
            | Value::Location(..)
            | Value::Regex(..)
            | Value::Tiling(..)
            | Value::Bytes(..)
            | Value::Decimal(..)
            | Value::Selector(..)
            | Value::Symbol(..)
            | Value::Args(..)
            | Value::Label(..)
            | Value::Path(..) => {}
        }
    }

    Ok(None)
}

#[cfg(test)]
mod v16_c_structural_tests {
    use super::*;

    use crate::contracts::world::World;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library,
    };
    use crate::entities::{file_id::FileId, font_book::FontBook, source::Source};
    struct TestWorld {
        library: Library,
        book: FontBook,
        source: Source,
    }
    impl TestWorld {
        fn new(text: &str) -> Self {
            Self {
                library: Library::new(),
                book: FontBook::new(),
                source: Source::new(
                    FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()),
                    text.into(),
                ),
            }
        }
    }
    impl World for TestWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            self.source.id()
        }
        fn source(&self, _: FileId) -> FileResult<Source> {
            Ok(self.source.clone())
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
        fn include_source(&self, _: FileId, _: &str) -> Result<Source, String> {
            panic!("refused include accessed World")
        }
        fn include_path(
            &self,
            _: &crate::entities::path::RootedPath,
        ) -> Result<Source, String> {
            panic!("refused include accessed World")
        }
    }

    fn with_engine(run: impl FnOnce(&mut Scopes<'_>, &mut EvalContext, &mut Engine<'_>)) {
        use comemo::Track;
        let world = TestWorld::new("");
        let library =
            Library::with_global(crate::compiler::stdlib::make_stdlib_with_features(
                &crate::contracts::world::SysInputs::default(),
                crate::entities::compiler_features::Features::empty(),
            ));
        let mut scopes = Scopes::new(Some(&library));
        let mut ctx = EvalContext::new();
        let mut styles = crate::entities::style_chain::StyleChain::default_chain();
        let mut rules = std::sync::Arc::from([]);
        let mut guards = Vec::new();
        let route = crate::entities::world_types::Route::root().with_id(world.main());
        let metrics = crate::compiler::layout::FixedMetrics;
        let mut sink = crate::entities::sink::Sink::new();
        let mut tracked_sink = sink.track_mut();
        let mut engine = Engine {
            world: &world,
            font_metrics: &metrics,
            route: route.track(),
            styles: &mut styles,
            show_rules: &mut rules,
            active_guards: &mut guards,
            current_file: world.main(),
            sink: &mut tracked_sink,
        };
        run(&mut scopes, &mut ctx, &mut engine);
    }

    #[test]
    fn v16_c_dispatch_refusals_preserve_cache_and_skip_arguments() {
        use crate::entities::ast::AstNode;
        let source = Source::new_with_parser(
            TestWorld::new("").main(),
            "receiver.__missing(panic(\"argument executed\"))".into(),
            crate::compiler::parse::parse_code,
        );
        let call = source
            .root()
            .children()
            .find_map(|n| {
                if let Some(Expr::FuncCall(c)) = Expr::from_untyped(n) {
                    Some(c)
                } else {
                    None
                }
            })
            .unwrap();
        let mut values = all_values();
        values.extend([
            Value::Type(crate::entities::value::Type::None),
            Value::Type(crate::entities::value::Type::Auto),
            Value::Type(crate::entities::value::Type::Bool),
            Value::Type(crate::entities::value::Type::Int),
            Value::Type(crate::entities::value::Type::Float),
            Value::Type(crate::entities::value::Type::Str),
            Value::Type(crate::entities::value::Type::Array),
            Value::Type(crate::entities::value::Type::Dictionary),
            Value::Type(crate::entities::value::Type::Module),
            Value::Type(crate::entities::value::Type::Datetime),
            Value::Type(crate::entities::value::Type::Function),
            Value::Type(crate::entities::value::Type::Content),
            Value::Type(crate::entities::value::Type::Length),
            Value::Type(crate::entities::value::Type::Ratio),
            Value::Type(crate::entities::value::Type::Relative),
            Value::Type(crate::entities::value::Type::Angle),
            Value::Type(crate::entities::value::Type::Color),
            Value::Type(crate::entities::value::Type::Stroke),
            Value::Type(crate::entities::value::Type::Fraction),
            Value::Type(crate::entities::value::Type::Alignment),
            Value::Type(crate::entities::value::Type::Location),
            Value::Type(crate::entities::value::Type::Gradient),
            Value::Type(crate::entities::value::Type::Regex),
            Value::Type(crate::entities::value::Type::Tiling),
            Value::Type(crate::entities::value::Type::Bytes),
            Value::Type(crate::entities::value::Type::Decimal),
            Value::Type(crate::entities::value::Type::Duration),
            Value::Type(crate::entities::value::Type::Version),
            Value::Type(crate::entities::value::Type::Selector),
            Value::Type(crate::entities::value::Type::Symbol),
            Value::Type(crate::entities::value::Type::Arguments),
            Value::Type(crate::entities::value::Type::State),
            Value::Type(crate::entities::value::Type::Counter),
            Value::Type(crate::entities::value::Type::Label),
            Value::Type(crate::entities::value::Type::Direction),
            Value::Type(crate::entities::value::Type::Path),
            Value::Type(crate::entities::value::Type::Type),
            Value::Type(crate::entities::value::Type::SpotColorant),
        ]);
        with_engine(|scopes, ctx, engine| {
            for value in values {
                let before = value.clone();
                let mut cached = Some(value);
                let result = try_instance_method(call, scopes, ctx, engine, &mut cached);
                if matches!(&before, Value::State(_) | Value::Counter(_)) {
                    let errors = result.unwrap_err();
                    assert!(!errors[0].message.contains("argument executed"));
                    assert!(errors[0].message.contains("__missing"));
                } else {
                    assert!(result.unwrap().is_none());
                }
                assert_eq!(cached, Some(before));
            }
        });
    }
    fn all_values() -> Vec<Value> {
        let world = TestWorld::new("");
        let mut values = vec![];
        for expression in [
            "none",
            "true",
            "42",
            "1.5",
            "\"A\"",
            "(1,)",
            "(a:1)",
            "std",
            "datetime(year:2020,month:1,day:1)",
            "(x)=>x",
            "[x]",
            "auto",
            "1pt",
            "1em + 10%",
            "10%",
            "1deg",
            "red",
            "stroke()",
            "1fr",
            "left",
            "gradient.linear(red,blue)",
            "regex(\"x\")",
            "tiling(size:(1pt,1pt),[x])",
            "bytes((1,2))",
            "decimal(\"1\")",
            "duration(seconds:1)",
            "version(1,2)",
            "heading.where()",
            "sym.alpha",
            "arguments(1,a:2)",
            "state(\"x\",0)",
            "counter(\"x\")",
            "<x>",
            "ltr",
            "int",
            "color.spot(\"x\",red)",
        ] {
            let (value, _) = crate::compiler::eval::eval_expression(&world, expression);
            values
                .push(value.unwrap_or_else(|errors| panic!("{expression}: {errors:?}")));
        }
        values.push(Value::Path(crate::entities::path::RootedPath::new(
            crate::entities::path::VirtualRoot::Project,
            crate::entities::path::VirtualPath::new("/a").unwrap(),
        )));
        let location = crate::entities::locator::Locator::new().next();
        values.push(Value::Location(location));
        values.push(Value::LocatedContent(
            crate::entities::value::IntrospectedContent::new(
                crate::entities::content::Content::Empty,
                None,
            ),
            location,
        ));
        let kinds: std::collections::HashSet<_> =
            values.iter().map(std::mem::discriminant).collect();
        assert_eq!(values.len(), 39);
        assert_eq!(kinds.len(), 39, "each current Value variant must be exercised");
        values
    }
}
