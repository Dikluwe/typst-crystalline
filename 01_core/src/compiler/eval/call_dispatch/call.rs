//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/call_dispatch/call.md
//! @prompt-hash 0df8c330
//! @layer L1
//! @updated 2026-11-08

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

// A pure length read needs no owned receiver or argument evaluation.
fn borrowed_collection_len(call: FuncCallNode<'_>, scopes: &Scopes<'_>) -> Option<Value> {
    let Expr::FieldAccess(access) = call.callee() else {
        return None;
    };
    if access.field().as_str() != "len" || call.args().items().next().is_some() {
        return None;
    }
    let Expr::Ident(ident) = access.target() else {
        return None;
    };
    let name = ident.as_str();
    if matches!(name, "html" | "asset") {
        return None;
    }
    match scopes.get(name)? {
        Value::Array(array) => Some(Value::Int(array.len() as i64)),
        Value::Dict(dict) => Some(Value::Int(dict.len() as i64)),
        Value::None
        | Value::Bool(_)
        | Value::Int(_)
        | Value::Float(_)
        | Value::Str(_)
        | Value::Module(_)
        | Value::Datetime(_)
        | Value::Func(_)
        | Value::Content(_)
        | Value::LocatedContent(_, _)
        | Value::Auto
        | Value::Length(_)
        | Value::Relative(_)
        | Value::Ratio(_)
        | Value::Angle(_)
        | Value::Color(_)
        | Value::Stroke(_)
        | Value::Fraction(_)
        | Value::Align(_)
        | Value::Location(_)
        | Value::Gradient(_)
        | Value::Regex(_)
        | Value::Tiling(_)
        | Value::Bytes(_)
        | Value::Decimal(_)
        | Value::Duration(_)
        | Value::Version(_)
        | Value::Selector(_)
        | Value::Symbol(_)
        | Value::Args(_)
        | Value::State(_)
        | Value::Counter(_)
        | Value::Label(_)
        | Value::Dir(_)
        | Value::Path(_)
        | Value::Type(_)
        | Value::SpotColorant(_) => None,
    }
}

pub(in crate::compiler::eval) fn eval_func_call(
    call: FuncCallNode<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    if let Some(value) = borrowed_collection_len(call, scopes) {
        return Ok(value);
    }

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

#[cfg(test)]
mod borrowed_len_tests {
    use super::borrowed_collection_len;
    use crate::compiler::{eval::eval_expression, parse::parse_code, scopes::Scopes};
    use crate::contracts::world::World;
    use crate::entities::ast::{expr::FuncCall as FuncCallNode, AstNode};
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::source::Source;
    use crate::entities::value::Value;
    use crate::entities::world_types::{Bytes, Datetime, FileError, FileResult, Font, Library};
    use std::num::NonZeroU16;

    fn apply(source: &str, scopes: &Scopes<'_>) -> Option<Value> {
        let root = parse_code(source);
        assert!(!root.erroneous(), "invalid fixture {source}: {:?}", root.errors());
        let call = root.children().find_map(FuncCallNode::from_untyped).expect("call fixture");
        borrowed_collection_len(call, scopes)
    }

    struct LenWorld {
        library: Library,
        book: FontBook,
        main: Source,
        module: Source,
    }

    impl LenWorld {
        fn new() -> Self {
            Self {
                library: Library::new(),
                book: FontBook::new(),
                main: Source::new(FileId::from_raw(NonZeroU16::new(1).unwrap()), String::new()).unwrap(),
                module: Source::new(
                    FileId::from_raw(NonZeroU16::new(2).unwrap()),
                    "#let rows = (8, 9)\n#let len() = 13\n".into(),
                ).unwrap(),
            }
        }
    }

    impl World for LenWorld {
        fn library(&self) -> &Library { &self.library }
        fn book(&self) -> &FontBook { &self.book }
        fn main(&self) -> FileId { self.main.id() }
        fn source(&self, id: FileId) -> FileResult<Source> {
            if id == self.main.id() { Ok(self.main.clone()) }
            else if id == self.module.id() { Ok(self.module.clone()) }
            else { Err(FileError::NotFound) }
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> { Err(FileError::NotFound) }
        fn font(&self, _: usize) -> Option<Font> { None }
        fn today(&self, _: Option<crate::entities::duration::Duration>) -> Option<Datetime> { None }
        fn include_source(&self, _: FileId, path: &str) -> Result<Source, String> {
            if path == "rows.typ" { Ok(self.module.clone()) }
            else { Err(format!("missing fixture: {path}")) }
        }
    }

    fn evaluate(world: &LenWorld, expression: &str) -> Value {
        let (result, warnings) = eval_expression(world, expression);
        assert!(warnings.is_empty(), "{expression}: {warnings:?}");
        result.unwrap_or_else(|errors| panic!("{expression}: {errors:?}"))
    }

    fn check(expression: &str) {
        assert_eq!(evaluate(&LenWorld::new(), expression), Value::Bool(true), "{expression}");
    }

    fn error(world: &LenWorld, expression: &str, expected: &str) {
        let (result, warnings) = eval_expression(world, expression);
        assert!(warnings.is_empty(), "{expression}: {warnings:?}");
        let errors = result.expect_err(expression);
        assert_eq!(errors.len(), 1, "{expression}: {errors:?}");
        assert_eq!(errors[0].message, expected, "{expression}");
    }

    #[test]
    fn actual_collection_variants_have_no_size_threshold() {
        let mut scopes = Scopes::new(None);
        for n in [0usize, 1, 3, 256, 4096] {
            scopes.define("a", Value::Array((0..n).map(|i| Value::Int(i as i64)).collect()));
            assert_eq!(apply("a.len()", &scopes), Some(Value::Int(n as i64)));
            let Value::Dict(mut d) = Value::Dict(Default::default()) else { unreachable!() };
            for i in 0..n { d.insert(format!("k{i}").into(), Value::Int(i as i64)); }
            scopes.define("d", Value::Dict(d));
            assert_eq!(apply("d.len()", &scopes), Some(Value::Int(n as i64)));
        }
    }

    #[test]
    fn outer_length_result_survives_binding_replacement() {
        let mut scopes = Scopes::new(None);
        scopes.define("a", Value::Array(vec![Value::Array(vec![Value::Int(1); 9]), Value::Dict(Default::default())]));
        let result = apply("a.len()", &scopes);
        scopes.define("a", Value::Array(vec![]));
        assert_eq!(result, Some(Value::Int(2)));
        assert_eq!(apply("a.len()", &scopes), Some(Value::Int(0)));
    }

    #[test]
    fn lookup_uses_outer_shadowed_and_captured_bindings() {
        let mut scopes = Scopes::new(None);
        scopes.define("a", Value::Array(vec![Value::None; 3]));
        let captured = std::sync::Arc::new(scopes.snapshot());
        scopes.enter();
        assert_eq!(apply("a.len()", &scopes), Some(Value::Int(3)));
        scopes.define("a", Value::Array(vec![Value::None]));
        assert_eq!(apply("a.len()", &scopes), Some(Value::Int(1)));
        scopes.exit();
        assert_eq!(apply("a.len()", &scopes), Some(Value::Int(3)));
        let mut closure_scopes = Scopes::with_parent(
            captured,
            crate::entities::scope::Capturer::Function,
        );
        assert_eq!(apply("a.len()", &closure_scopes), Some(Value::Int(3)));
        closure_scopes.define("a", Value::Dict(Default::default()));
        assert_eq!(apply("a.len()", &closure_scopes), Some(Value::Int(0)));
    }

    #[test]
    fn only_exact_call_syntax_is_eligible() {
        let mut scopes = Scopes::new(None);
        scopes.define("a", Value::Array(vec![Value::None; 7]));
        for source in [
            "a.len(1)", "a.len(x: 1)", "a.len(..())", "a.len(..(:))", "a.len(..a)",
            "a.len(panic(\"x\"))", "a.first()", "a.length()", "a.LEN()", "(a).len()",
            "((a)).len()", "a.at(0).len()", "a.rows.len()", "{ a }.len()", "f().len()",
            "len(a)", "array.len(a)", "missing.len()",
        ] {
            assert_eq!(apply(source, &scopes), None, "must fall through: {source}");
        }
    }

    #[test]
    fn reserved_names_are_ineligible_even_when_shadowed() {
        let mut scopes = Scopes::new(None);
        for name in ["html", "asset"] {
            scopes.define(name, Value::Array(vec![Value::None; 2]));
            assert_eq!(apply(&format!("{name}.len()"), &scopes), None);
            scopes.define(name, Value::Dict(Default::default()));
            assert_eq!(apply(&format!("{name}.len()"), &scopes), None);
        }
        scopes.define("htmlx", Value::Array(vec![Value::None; 2]));
        assert_eq!(apply("htmlx.len()", &scopes), Some(Value::Int(2)));
    }

    #[test]
    fn names_functions_and_modules_do_not_authenticate_collections() {
        let world = LenWorld::new();
        let mut scopes = Scopes::new(None);
        for expression in ["none", "true", "9", "2.0", "\"abcd\"", "array", "calc", "(x) => x"] {
            scopes.define("a", evaluate(&world, expression));
            assert_eq!(apply("a.len()", &scopes), None, "{expression}");
        }
    }

    #[test]
    fn aliases_and_dictionary_mutations_remain_independent() {
        check("{ let a=(1,2,3); let b=a; a.push(4); b.push(5); let d=(x:1); let e=d; d.insert(\"y\",2); e.insert(\"z\",3); a==(1,2,3,4) and b==(1,2,3,5) and a.len()==4 and b.len()==4 and d.len()==2 and e.len()==2 and not (\"z\" in d) and not (\"y\" in e) }");
    }

    #[test]
    fn nested_mutations_and_detached_lengths_keep_value_semantics() {
        check("{ let a=((1,2),(3,4),(5,6)); let b=a; let n=a.len(); a.first().push(9); a.last().push(8); a.at(1).push(7); b.at(1).push(0); a.push(()); n==3 and a.len()==4 and a.first().len()==3 and a.last().len()==0 and a.at(1).last()==7 and b==((1,2),(3,4,0),(5,6)) }");
    }

    #[test]
    fn lexical_snapshot_defaults_callbacks_and_local_shadowing() {
        check("{ let a=(10,20); let f()=a.len(); a.push(30); let sizes(xs:(3,5,7))=xs.len(); let shadow(a)=a.len(); f()==2 and a.len()==3 and sizes()==3 and sizes(xs:(1,))==1 and sizes()==3 and ((1,),(1,2),()).map(xs=>xs.len())==(1,2,0) and shadow((:))==0 and { let a=(x:1); a.len()==1 } }");
    }

    #[test]
    fn receiver_and_argument_effects_keep_original_order() {
        check("{ let seen=(); let r=({seen.push(1);(1,2,3)}).len(); r==3 and seen.len()==1 }");
        check("{ let a=(1,2); let selected=a.at({a.push(9);-1}); selected==2 and a==(1,2,9) and a.len()==3 }");
        let world = LenWorld::new();
        for expression in [
            "{let a=(1,2);a.len(panic(\"len-positional\"))}",
            "{let a=(1,2);a.len(unused:panic(\"len-named\"))}",
            "{let a=(1,2);a.len(..panic(\"len-spread\"))}",
        ] {
            let marker = if expression.contains("positional") { "len-positional" }
                else if expression.contains("named") { "len-named" } else { "len-spread" };
            error(&world, expression, &format!("panicked with: {marker}"));
        }
    }

    #[test]
    fn invalid_arguments_and_empty_spreads_follow_existing_validation() {
        let world = LenWorld::new();
        for receiver in ["(1,2)", "(x:1)"] {
            error(&world, &format!("{{let a={receiver};a.len(1)}}"), "unexpected argument");
            error(&world, &format!("{{let a={receiver};a.len(unused:1)}}"), "unexpected argument: unused");
            error(&world, &format!("{{let a={receiver};a.len(..(1,))}}"), "unexpected argument");
        }
        check("{let a=(1,2);let d=(x:1);a.len(..())==2 and d.len(..(:))==1}");
        check("{let a=(1,2);let seen=();let n=a.len(..{seen.push(1);()});n==2 and seen.len()==1}");
    }

    #[test]
    fn reserved_feature_gates_and_asset_shadow_are_preserved() {
        let world = LenWorld::new();
        error(&world, "{let html=(1,2);html.len()}", "cannot access variable `html` because the `html` feature is not enabled");
        error(&world, "asset.len()", "cannot access variable `asset` because the `bundle` feature is not enabled");
        check("{let asset=(1,2);asset.len()==2}");
    }

    #[test]
    fn strings_parentheses_fields_modules_and_dictionary_len_keys() {
        check("{let a=(1,2,3);let d=(rows:a);let s=\"hello\";(a).len()==3 and d.rows.len()==3 and s.len()==5 and (s).len()==5 and array.len(a)==3}");
        check("{import \"rows.typ\" as m;let a=m; a.len()==13 and m.rows.len()==2}");
        check("{let d=(len:()=>99,x:1); d.len()==2}");
    }

    #[test]
    fn unknown_methods_skip_arguments_and_keep_diagnostics() {
        let world = LenWorld::new();
        error(&world, "{let a=(1,2);a.__missing(panic(\"must not run\"))}", "type array has no method `__missing`");
        error(&world, "{let a=7;a.len()}", "type integer has no method `len`");
        error(&world, "missing_c15.len()", "unknown variable `missing_c15`");
    }

    #[test]
    fn rejected_call_does_not_poison_subsequent_valid_evaluation() {
        let world = LenWorld::new();
        error(&world, "{let a=(1,2);a.len(unused:1)}", "unexpected argument: unused");
        assert_eq!(evaluate(&world, "{let a=(1,2);a.len()}"), Value::Int(2));
        error(&world, "{let a=(1,2);a.len(panic(\"control\"))}", "panicked with: control");
        assert_eq!(evaluate(&world, "{let d=(:);d.len()}"), Value::Int(0));
    }
}
