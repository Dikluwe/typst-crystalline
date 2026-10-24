//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/call_dispatch/call.md
//! @prompt-hash 3acabf14
//! @layer L1
//! @updated 2026-09-30

use super::*;

pub(super) fn resolve_receiver<'a>(
    access: crate::entities::ast::expr::FieldAccess<'_>,
    cached_receiver: &'a mut Option<Value>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<&'a Value> {
    if cached_receiver.is_none() {
        *cached_receiver = Some(eval_expr(access.target(), scopes, ctx, engine)?);
    }
    Ok(cached_receiver.as_ref().expect("receiver initialized"))
}

pub(super) fn resolve_callee<'a>(
    call: FuncCallNode<'_>,
    cached_receiver: &mut Option<Value>,
    resolved_callee: &'a mut Option<Value>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<&'a Value> {
    if resolved_callee.is_none() {
        let value = if let Expr::FieldAccess(access) = call.callee() {
            if cached_receiver.is_none() {
                resolve_receiver(access, cached_receiver, scopes, ctx, engine)?;
            }
            bindings::eval_field_access_with_target(
                access,
                cached_receiver.take().expect("receiver initialized"),
                scopes,
                ctx,
                engine,
            )?
        } else {
            eval_expr(call.callee(), scopes, ctx, engine)?
        };
        *resolved_callee = Some(value);
    }
    Ok(resolved_callee.as_ref().expect("callee initialized"))
}

pub(in crate::compiler::eval) fn eval_func_call(
    call: FuncCallNode<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    // As duas rotas preservam o receiver resolvido mesmo no fallback.
    let mut resolved_callee = None;
    let mut cached_receiver = None;
    let mut resolved_args = None;
    if let Expr::FieldAccess(access) = call.callee() {
        let method = access.field().as_str();
        if matches!(method, "where" | "with") {
            let target = eval_expr(access.target(), scopes, ctx, engine)?;
            if let Value::Func(function) = target {
                let mut args = eval_args(call.args(), scopes, ctx, engine)?;
                args.span = call.span();
                let result = if method == "where" {
                    bindings::eval_element_where(&function, &args).map(Value::Selector)
                } else {
                    Ok(function_with(function, args))
                };
                return trace_call(result, Some(method), call.span(), engine);
            }
            if let Some(error) = bindings::field_callee_error(&target, access) {
                return Err(error);
            }
            resolved_callee = Some(bindings::eval_value_field_access(
                target,
                method,
                access.field().span(),
            )?);
        }
    }

    if resolved_callee.is_none() {
        if let Some(value) = super::selector_intercepts::try_selector_call(
            call,
            scopes,
            ctx,
            engine,
            &mut cached_receiver,
        )? {
            return Ok(value);
        }

        // **P717** — Métodos mutantes (`push`/`pop`/`insert`/`remove`): mirror
        // de `maybe_resolve_mutating` (vanilla `call.rs:189-212`), sobre o
        // `access()` de P716. Tem de correr **antes** do bloco P466, que avalia
        // o target como valor (clone) — mutação exige o local. `Ok(None)` =
        // fall-through para a cadeia normal (módulos/funcs com campos com estes
        // nomes continuam a resolver abaixo).
        if let Expr::FieldAccess(access) = call.callee() {
            if bindings::is_mutating_method(access.field().as_str()) {
                match bindings::try_eval_mutating_method(
                    access,
                    call.args(),
                    call.span(),
                    scopes,
                    ctx,
                    engine,
                )? {
                    bindings::MutatingCall::Handled(result) => return Ok(result),
                    bindings::MutatingCall::Fallback { target, args } => {
                        cached_receiver = Some(target);
                        resolved_args = Some(args);
                    }
                }
            }
        }

        if let Some(result) = super::instance_methods::try_instance_method(
            call,
            scopes,
            ctx,
            engine,
            &mut cached_receiver,
        )? {
            return Ok(result);
        }

        if let Some(result) = super::collection_methods::try_collection_method(
            call,
            scopes,
            ctx,
            engine,
            &mut cached_receiver,
        )? {
            return Ok(result);
        }

        if let Some(result) = super::contextual_calls::try_contextual_call(
            call,
            scopes,
            ctx,
            engine,
            &mut cached_receiver,
            &mut resolved_callee,
        )? {
            return Ok(result);
        }

        if let Some(result) = super::content_methods::try_content_methods(
            call,
            scopes,
            ctx,
            engine,
            &mut cached_receiver,
        )? {
            return Ok(result);
        }

        // **P815** — `eval_field_callee` do vanilla (`call.rs:239-345`): depois
        // de todos os despachos de método legítimos acima, um callee
        // `target.field` chamado como função cujo alvo não é
        // Symbol/Func/Type/Module produz os erros verbatim do vanilla —
        // método inexistente (`type integer has no method `foo``), dict-key-call
        // com hints, "not a valid method". Sem isto, o caminho genérico avaliava
        // o field access e errava com mensagens divergentes (ou, pior, chamava
        // funções guardadas em dict keys — medido em P815).
        if let Expr::FieldAccess(access) = call.callee() {
            if resolved_callee.is_none() {
                let target =
                    resolve_receiver(access, &mut cached_receiver, scopes, ctx, engine)?;
                if let Some(err) = bindings::field_callee_error(target, access) {
                    return Err(err);
                }
                resolve_callee(
                    call,
                    &mut cached_receiver,
                    &mut resolved_callee,
                    scopes,
                    ctx,
                    engine,
                )?;
            }
        }
    }
    super::invoke::invoke_resolved_call(
        resolved_callee,
        resolved_args,
        call,
        scopes,
        ctx,
        engine,
    )
}

#[cfg(test)]
mod call_once_tests {
    use crate::compiler::eval::eval_expression;
    use crate::contracts::world::World;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::source::Source;
    use crate::entities::value::Value;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library,
    };
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
                map.insert(
                    path.to_string(),
                    Source::new(id, text.to_string()).expect("valid Source test fixture"),
                );
            }
            Self {
                library: Library::new(),
                book: FontBook::new(),
                main: Source::new(main_id, main_text.to_string())
                    .expect("valid Source test fixture"),
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
    }

    fn check(expression: &str, expected: Value) {
        let world = ImportMockWorld::new("", &[("sample.typ", "#let f(x)=x\n#let measure(x)=x\n#let layout(x)=x\n#let within(x)=x\n#let before(x)=x\n#let after(x)=x\n#let func()=42\n#let has(x)=x\n#let body(x)=x\n#let insert(x)=x\n#let remove(x)=x\n#let push(x)=x\n#let pop(x)=x\n")]);
        let (actual, warnings) = eval_expression(&world, expression);
        assert!(warnings.is_empty(), "{expression}: {warnings:?}");
        assert_eq!(actual.unwrap(), expected, "{expression}");
    }
    fn values(items: &[i64]) -> Value {
        Value::Array(items.iter().copied().map(Value::Int).collect())
    }

    #[test]
    fn call_once_module_type_and_content_receivers_and_arguments() {
        for (receiver, method, arg, expected) in [
            ("calc", "abs", "-3", Value::Int(3)),
            ("array", "len", "(1,2)", Value::Int(2)),
            ("str", "from-unicode", "65", Value::Str("A".into())),
            ("[abc]", "has", "\"text\"", Value::Bool(true)),
        ] {
            let expression = format!("{{ let n=0; let a=0; let r=({{n+=1; {receiver}}}).{method}({{a+=1; {arg}}}); (n,a,r) }}");
            check(
                &expression,
                Value::Array(vec![Value::Int(1), Value::Int(1), expected]),
            );
        }
    }

    #[test]
    fn call_once_non_intercepted_selector_context_and_content_names() {
        for method in
            ["f", "measure", "layout", "within", "before", "after", "has", "body"]
        {
            check(&format!("{{ import \"sample.typ\" as m; let n=0; let a=0; let r=({{n+=1;m}}).{method}({{a+=1;42}}); (n,a,r) }}"), values(&[1,1,42]));
        }
        check(
            "{ import \"sample.typ\" as m; let n=0; let r=({n+=1;m}).func(); (n,r) }",
            values(&[1, 42]),
        );
    }

    #[test]
    fn call_once_mutating_fallback_reuses_arguments() {
        for method in ["insert", "remove", "push", "pop"] {
            check(&format!("{{ import \"sample.typ\" as source; let m=source; let a=0; let r=m.{method}({{a+=1;42}}); (a,r) }}"), values(&[1,42]));
        }
    }

    #[test]
    fn call_once_unknown_methods_do_not_execute_arguments() {
        let world = ImportMockWorld::new("", &[]);
        for (receiver, message) in [
            ("calc", "module `calc` does not contain `__missing`"),
            ("array", "type array does not contain field `__missing`"),
            ("[abc]", "element text has no method `__missing`"),
            ("(1,2)", "type array has no method `__missing`"),
            ("(a:1)", "type dictionary has no method `__missing`"),
            ("\"abc\"", "type string has no method `__missing`"),
            ("42", "type integer has no method `__missing`"),
            ("version(1,2,3)", "type version has no method `__missing`"),
        ] {
            let expr = format!("({receiver}).__missing(panic(\"argument executed\"))");
            let (result, _) = eval_expression(&world, &expr);
            let errors = result.unwrap_err();
            assert_eq!(errors[0].message, message, "{expr}");
            if receiver == "array" {
                let source = Source::new_with_parser(
                    world.main(),
                    expr.clone(),
                    crate::compiler::parse::parse_code,
                )
                .expect("valid Source test fixture");
                let start = expr.find("__missing").unwrap();
                assert_eq!(
                    source.span_byte_range(errors[0].span),
                    Some(start..start + "__missing".len())
                );
            }
        }
    }

    #[test]
    fn call_once_collections_where_with_selector_and_mutation_controls() {
        check(
            "{ let n=0; let a=0; let r=({n+=1;(1,2)}).at({a+=1;1}); (n,a,r) }",
            values(&[1, 1, 2]),
        );
        check(
            "{ let n=0; let a=0; let r=({n+=1;calc.abs}).with({a+=1;-3}); (n,a,r()) }",
            values(&[1, 1, 3]),
        );
        check("{ let n=0; let a=0; let r=({n+=1;heading}).where(level:{a+=1;1}); (n,a,type(r)==selector) }", Value::Array(vec![Value::Int(1),Value::Int(1),Value::Bool(true)]));
        check("{ let n=0; let a=0; let r=({n+=1;heading.where()}).or({a+=1;figure.where()}); (n,a,type(r)==selector) }", Value::Array(vec![Value::Int(1),Value::Int(1),Value::Bool(true)]));
        check("{ let a=(1,); a.push(2); a }", values(&[1, 2]));
    }

    #[test]
    fn call_once_matrix_keeps_each_legitimate_effect() {
        check("{ let n=0; let r=math.mat({n+=1;1}); n }", Value::Int(1));
        check("{ let n=(1,); let r=math.mat({n.push(1);1}); n }", values(&[1, 1]));
        check("{ let n=(); let r=math.mat({n.push(1);1}); n }", values(&[1]));
    }

    fn all_values() -> Vec<Value> {
        let world = ImportMockWorld::new("", &[]);
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
            let (value, _) = eval_expression(&world, expression);
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

    fn with_engine(
        run: impl FnOnce(
            &mut crate::compiler::scopes::Scopes<'_>,
            &mut crate::compiler::eval::EvalContext,
            &mut crate::entities::engine::Engine<'_>,
        ),
    ) {
        use comemo::Track;
        let world = ImportMockWorld::new("", &[]);
        let library =
            Library::with_global(crate::compiler::stdlib::make_stdlib_with_features(
                &crate::contracts::world::SysInputs::default(),
                crate::entities::compiler_features::Features::empty(),
            ));
        let mut scopes = crate::compiler::scopes::Scopes::new(Some(&library));
        let mut ctx = crate::compiler::eval::EvalContext::new();
        let mut styles = crate::entities::style_chain::StyleChain::default_chain();
        let mut show_rules = std::sync::Arc::from([]);
        let mut active_guards = Vec::new();
        let route = crate::entities::world_types::Route::root().with_id(world.main());
        let metrics = crate::compiler::layout::FixedMetrics;
        let mut sink = crate::entities::sink::Sink::new();
        let mut tracked_sink = sink.track_mut();
        let mut engine = crate::entities::engine::Engine {
            world: &world,
            font_metrics: &metrics,
            route: route.track(),
            styles: &mut styles,
            show_rules: &mut show_rules,
            active_guards: &mut active_guards,
            current_file: world.main(),
            sink: &mut tracked_sink,
        };
        run(&mut scopes, &mut ctx, &mut engine);
    }

    #[test]
    fn call_once_unhandled_keeps_every_value_and_does_not_evaluate_arguments() {
        let values = all_values();
        with_engine(|scopes, ctx, engine| {
            for value in values {
                let original = value.clone();
                let pointer = if let Value::Array(array) = &value {
                    Some(array.as_ptr())
                } else {
                    None
                };
                let result = crate::compiler::stdlib::try_dispatch_collection_method_with(
                    value,
                    "__unknown",
                    |_, _, _| panic!("unknown operation evaluated arguments"),
                    None,
                    scopes,
                    ctx,
                    engine,
                );
                let crate::compiler::stdlib::CollectionDispatch::Unhandled(value) =
                    result
                else {
                    panic!("unknown operation handled")
                };
                assert_eq!(
                    std::mem::discriminant(&original),
                    std::mem::discriminant(&value)
                );
                assert_eq!(original, value);
                if let Value::Array(array) = &value {
                    assert_eq!(pointer, Some(array.as_ptr()));
                }
            }
        });
    }

    #[test]
    fn call_once_callable_classification_covers_all_value_variants_before_arguments() {
        use crate::entities::ast::{expr::Expr, AstNode};
        let values = all_values();
        let source = Source::new_with_parser(
            FileId::from_raw(NonZeroU16::new(1).unwrap()),
            "placeholder(panic(\"argument executed\"))".into(),
            crate::compiler::parse::parse_code,
        )
        .expect("valid Source test fixture");
        let call = source
            .root()
            .children()
            .find_map(|child| {
                if let Some(Expr::FuncCall(call)) = Expr::from_untyped(child) {
                    Some(call)
                } else {
                    None
                }
            })
            .expect("call fixture");
        with_engine(|scopes, ctx, engine| {
            for value in values {
                let callable = matches!(value, Value::Func(_) | Value::Type(_));
                let expected = if callable {
                    "panicked with: argument executed".to_owned()
                } else {
                    format!("expected function, found {}", crate::compiler::eval::operators::error_formatting::vanilla_type_name(&value))
                };
                let errors = super::super::invoke::invoke_resolved_call(
                    Some(value),
                    None,
                    call,
                    scopes,
                    ctx,
                    engine,
                )
                .unwrap_err();
                assert_eq!(errors[0].message, expected);
                if !callable {
                    assert_eq!(errors[0].span, call.callee().span());
                }
            }
        });
    }
}
