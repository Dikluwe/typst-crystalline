//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/bindings/field_access.md
//! @prompt-hash eac63159
//! @layer L1
//! @updated 2026-09-20
//!
//! Acesso a campo (`a.b`) sobre valores e sobre `Content`, os métodos de
//! `Content`, e as mensagens de erro de campo e de callee não-chamável.
//!
//! Extraído de `compiler/eval/bindings.rs` no Passo 1013 conforme ADR-0109
//! (atomização — forma B, free function no arquivo da unidade).

use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

use crate::compiler::scopes::Scopes;
use crate::compiler::stdlib::native_str_from_unicode;
use crate::entities::args::Args;
use crate::entities::engine::Engine;
use crate::entities::func::Func;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::{Type, Value};

use crate::compiler::eval::{eval_expr, EvalContext};

use crate::compiler::eval::operators::error_formatting::vanilla_type_name;

use super::method_dispatch::{expect_positional, finish_args};

#[cfg(test)]
mod p1324_tests {
    use super::*;
    use crate::entities::args::Args;
    use crate::entities::func::{ClosureRepr, Func};
    use crate::entities::scope::{Capturer, Scope};
    use crate::entities::source_result::Severity;
    use crate::entities::span::Span;
    use crate::entities::syntax_kind::SyntaxKind;
    use crate::entities::syntax_node::SyntaxNode;
    use std::sync::Arc;

    fn subject(engine: bool, name: &'static str, scope: Option<Arc<Scope>>) -> Func {
        match (engine, scope) {
            (false, None) => {
                Func::native(name, |_, _, _, _| panic!("lookup invoked native"))
            }
            (false, Some(scope)) => Func::native_with_namespace(
                name,
                |_, _, _, _| panic!("lookup invoked native"),
                scope,
            ),
            (true, None) => Func::native_with_engine(name, |_, _, _, _, _, _| {
                panic!("lookup invoked engine native")
            }),
            (true, Some(scope)) => Func::native_with_engine_and_namespace(
                name,
                |_, _, _, _, _, _| panic!("lookup invoked engine native"),
                scope,
            ),
        }
    }

    fn wrapped(mut func: Func, depth: usize) -> Func {
        for ordinal in 0..depth {
            func = func.with(Args::positional(vec![Value::Int(ordinal as i64 + 91)]));
        }
        func
    }

    fn anchors() -> [Span; 2] {
        let id = crate::entities::file_id::FileId::from_raw(
            std::num::NonZeroU16::new(271).unwrap(),
        );
        [Span::detached(), Span::from_range(id, 37..53)]
    }

    fn expect_error(func: Func, field: &str, span: Span, message: &str) {
        let errors = eval_value_field_access(Value::Func(func), field, span).unwrap_err();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].severity, Severity::Error);
        assert_eq!(errors[0].message, message);
        assert_eq!(errors[0].span, span, "pure lookup must preserve the supplied anchor");
        assert!(errors[0].hints.is_empty());
        assert!(errors[0].trace.is_empty());
    }

    #[test]
    fn p1324_some_empty_and_populated_all_native_shapes() {
        for engine in [false, true] {
            for populated in [false, true] {
                let mut scope = Scope::new();
                if populated {
                    scope.define("present", Value::Int(42));
                }
                let scope = Arc::new(scope);
                for (internal, public) in [
                    ("meteor", "meteor"),
                    ("space.meteor", "meteor"),
                    ("alpha.beta.gamma", "gamma"),
                ] {
                    for depth in [0, 1, 3] {
                        for field in ["missing", "absent_1324"] {
                            for span in anchors() {
                                let f = wrapped(
                                    subject(engine, internal, Some(scope.clone())),
                                    depth,
                                );
                                expect_error(
                                    f,
                                    field,
                                    span,
                                    &format!(
                                        "function `{public}` does not contain field `{field}`"
                                    ),
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn p1324_present_values_and_function_kind_survive_wrapping_and_missing_lookup() {
        let member = subject(false, "namespace.member", None);
        let mut scope = Scope::new();
        scope.define("present", Value::Int(713));
        scope.define("nothing", Value::None);
        scope.define("enabled", Value::Bool(false));
        scope.define("callable", Value::Func(member.clone()));
        let scope = Arc::new(scope);
        for engine in [false, true] {
            for depth in [0, 1, 3] {
                let f = wrapped(
                    subject(engine, "anything.owner", Some(scope.clone())),
                    depth,
                );
                for span in anchors() {
                    expect_error(
                        f.clone(),
                        "missing",
                        span,
                        "function `owner` does not contain field `missing`",
                    );
                    for (field, expected) in [
                        ("present", Value::Int(713)),
                        ("nothing", Value::None),
                        ("enabled", Value::Bool(false)),
                        ("callable", Value::Func(member.clone())),
                    ] {
                        let actual =
                            eval_value_field_access(Value::Func(f.clone()), field, span)
                                .unwrap();
                        assert_eq!(actual, expected);
                        if field == "callable" {
                            let Value::Func(actual) = actual else {
                                panic!("member lost function kind")
                            };
                            assert_eq!(actual.name(), Some("namespace.member"));
                        }
                    }
                }
                assert_eq!(scope.len(), 4);
            }
        }
    }

    #[test]
    fn p1324_none_remains_the_p1311_diagnostic_without_namespace_fabrication() {
        for engine in [false, true] {
            for depth in [0, 1, 3] {
                let f = wrapped(subject(engine, "outer.original", None), depth);
                for span in anchors() {
                    expect_error(
                        f.clone(),
                        "different",
                        span,
                        "function `original` does not contain field `different`",
                    );
                }
                assert!(f.namespace().is_none());
            }
        }
    }

    #[test]
    fn p1324_named_closure_and_user_element_are_not_native() {
        let closure = Func::closure(ClosureRepr {
            name: Some("alpha.beta.gamma".into()),
            params: vec![],
            sink_name: None,
            body: SyntaxNode::placeholder(SyntaxKind::Markup),
            captured: Arc::new(Scope::new()),
            capturer: Capturer::Function,
        });
        let element = Func::element(
            "alpha.beta.gamma",
            Arc::new(|_| panic!("lookup invoked element")),
        );
        for depth in [0, 1, 3] {
            for span in anchors() {
                expect_error(
                    wrapped(closure.clone(), depth),
                    "missing",
                    span,
                    "cannot access fields on user-defined functions",
                );
            }
        }
        for f in [element] {
            for depth in [0, 1, 3] {
                for span in anchors() {
                    expect_error(
                        wrapped(f.clone(), depth),
                        "missing",
                        span,
                        "cannot access fields on type function",
                    );
                }
            }
        }
    }

    #[test]
    fn p1324_named_plugin_is_not_native() {
        use crate::contracts::plugin_host::{PluginError, PluginHost, PluginModuleId};
        use crate::entities::bytes::Bytes;
        struct UncalledHost;
        impl PluginHost for UncalledHost {
            fn load(&self, _: &[u8]) -> Result<PluginModuleId, PluginError> {
                panic!("load")
            }
            fn exports(
                &self,
                _: PluginModuleId,
            ) -> Result<Vec<ecow::EcoString>, PluginError> {
                panic!("exports")
            }
            fn call(
                &self,
                _: PluginModuleId,
                _: &str,
                _: &[Bytes],
            ) -> Result<Bytes, PluginError> {
                panic!("call")
            }
            fn transition(
                &self,
                _: PluginModuleId,
                _: &str,
                _: &[Bytes],
            ) -> Result<PluginModuleId, PluginError> {
                panic!("transition")
            }
        }
        let f = Func::plugin(crate::entities::plugin_func::PluginFunc {
            host: Arc::new(UncalledHost),
            module: PluginModuleId(19),
            name: "alpha.beta.gamma".into(),
        });
        for depth in [0, 1, 3] {
            for span in anchors() {
                expect_error(
                    wrapped(f.clone(), depth),
                    "missing",
                    span,
                    "cannot access fields on type function",
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::file_id::FileId;
    use crate::entities::scope::{Capturer, Scope};
    use std::num::NonZeroU16;
    use std::sync::Arc;

    fn field_span() -> Span {
        Span::from_range(FileId::from_raw(NonZeroU16::new(1).unwrap()), 20..27)
    }

    fn native(name: &'static str) -> Func {
        Func::native(name, |_, _, _, _| panic!("field lookup must not call native"))
    }

    fn assert_error(func: Func, expected: &str) {
        let errors = eval_value_field_access(Value::Func(func), "missing", field_span())
            .unwrap_err();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].message, expected);
        assert_eq!(errors[0].span, field_span());
        assert!(errors[0].hints.is_empty());
        assert!(errors[0].trace.is_empty());
    }

    #[test]
    fn p1311_plain_native_missing_field_names_function() {
        for name in ["csv", "read", "xml", "unlisted-native"] {
            assert_error(
                native(name),
                &format!("function `{name}` does not contain field `missing`"),
            );
        }
    }

    #[test]
    fn p1311_native_public_name_and_nested_with() {
        for f in [
            native("calc.abs"),
            native("calc.abs")
                .with(Args::positional(vec![Value::Int(7)]))
                .with(Args::positional(vec![])),
        ] {
            assert_error(f, "function `abs` does not contain field `missing`");
        }
    }

    #[test]
    fn p1311_engine_native_without_namespace() {
        let f = Func::native_with_engine("eval", |_, _, _, _, _, _| {
            panic!("field lookup must not call engine native")
        });
        assert_error(f, "function `eval` does not contain field `missing`");
    }

    #[test]
    fn p1311_existing_namespaces_including_empty_preserved() {
        for populated in [false, true] {
            let mut scope = Scope::new();
            if populated {
                scope.define("present", Value::Int(42));
            }
            let ns = Arc::new(scope);
            let native = Func::native_with_namespace(
                "same-name",
                |_, _, _, _| panic!("lookup must not call function"),
                ns.clone(),
            );
            let engine = Func::native_with_engine_and_namespace(
                "same-name",
                |_, _, _, _, _, _| panic!("lookup must not call function"),
                ns,
            );
            for f in [native, engine] {
                if populated {
                    assert_eq!(
                        eval_value_field_access(
                            Value::Func(f.clone()),
                            "present",
                            field_span()
                        )
                        .unwrap(),
                        Value::Int(42)
                    );
                }
                assert_error(
                    f.clone(),
                    "function `same-name` does not contain field `missing`",
                );
                assert_error(
                    f.with(Args::positional(vec![])),
                    "function `same-name` does not contain field `missing`",
                );
            }
        }
    }

    #[test]
    fn p1311_named_non_native_categories_preserved() {
        use crate::contracts::plugin_host::{PluginError, PluginHost, PluginModuleId};
        use crate::entities::bytes::Bytes;
        struct NoCalls;
        impl PluginHost for NoCalls {
            fn load(&self, _: &[u8]) -> Result<PluginModuleId, PluginError> {
                panic!("no load")
            }
            fn exports(&self, _: PluginModuleId) -> Result<Vec<EcoString>, PluginError> {
                panic!("no exports")
            }
            fn call(
                &self,
                _: PluginModuleId,
                _: &str,
                _: &[Bytes],
            ) -> Result<Bytes, PluginError> {
                panic!("no call")
            }
            fn transition(
                &self,
                _: PluginModuleId,
                _: &str,
                _: &[Bytes],
            ) -> Result<PluginModuleId, PluginError> {
                panic!("no transition")
            }
        }
        let closure = Func::closure(crate::entities::func::ClosureRepr {
            name: Some("csv".into()),
            params: vec![],
            sink_name: None,
            body: crate::entities::source::Source::detached("none").root().clone(),
            captured: Arc::new(Scope::new()),
            capturer: Capturer::Function,
        });
        let element = Func::element("csv", Arc::new(|_| panic!("no constructor")));
        let plugin = Func::plugin(crate::entities::plugin_func::PluginFunc {
            host: Arc::new(NoCalls),
            module: PluginModuleId(1),
            name: "csv".into(),
        });
        assert_error(closure.clone(), "cannot access fields on user-defined functions");
        assert_error(
            closure.with(Args::positional(vec![])),
            "cannot access fields on user-defined functions",
        );
        for f in [element, plugin] {
            assert_error(f.clone(), "cannot access fields on type function");
            assert_error(
                f.with(Args::positional(vec![])),
                "cannot access fields on type function",
            );
        }
    }
}

/// Public name of a native, including namespaces and partial applications.
#[cfg(test)]
mod atomization_tests {
    #[test]
    fn content_type_methods_keep_known_and_unknown_fields() {
        assert!(super::content_methods::content_type_field("has").is_some());
        assert!(super::content_methods::content_type_field("unknown").is_none());
    }
}

mod callee_error;
mod content_methods;
mod fields;
mod identity;

pub(in crate::compiler::eval) use self::callee_error::field_callee_error;
pub(in crate::compiler::eval) use self::content_methods::content_type_field;
#[cfg(test)]
pub(crate) use self::content_methods::eval_content_method_at;
pub(crate) use self::content_methods::{
    eval_content_method, eval_introspected_content_method_at,
    eval_outline_entry_body_method,
};
pub(in crate::compiler::eval) use self::fields::{
    check_module_field_feature, eval_field_access, eval_field_access_with_target,
    eval_value_field_access,
};
pub(in crate::compiler::eval) use self::identity::warn_deprecated_symbol_binding;
use self::{content_methods::*, identity::*};

#[cfg(test)]
mod p1325_tests {
    use crate::compiler::eval::{eval_with_full_error_target_and_features, EvalTarget};
    use crate::contracts::world::World;
    use crate::entities::compiler_features::{Feature, Features};
    use crate::entities::element_registry::ElementRegistry;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::sink::Sink;
    use crate::entities::source::Source;
    use crate::entities::source_result::Severity;
    use crate::entities::world_types::Route;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library, Routines, Traced,
    };
    use comemo::Track;

    struct TestWorld {
        source: Source,
        library: Library,
        book: FontBook,
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
    }

    fn check(text: &str, needle: &str, message: &str) {
        for mask in 0..4 {
            let world = TestWorld {
                source: Source::detached(text),
                library: Library::new(),
                book: FontBook::new(),
            };
            let mut features = Features::empty();
            if mask & 1 != 0 {
                features.enable(Feature::Html);
            }
            if mask & 2 != 0 {
                features.enable(Feature::A11yExtras);
            }
            let routines = Routines::new();
            let traced = Traced::default();
            let mut sink = Sink::new();
            let route = Route::root();
            let registry = ElementRegistry::new();
            let result = eval_with_full_error_target_and_features(
                &routines,
                &world,
                traced.track(),
                sink.track_mut(),
                route.track(),
                &world.source,
                &registry,
                false,
                EvalTarget::Paged,
                features,
            );
            let errors = match result {
                Ok(_) => panic!("expected error for {text}"),
                Err(errors) => errors,
            };
            assert_eq!(errors.len(), 1, "{text}");
            let error = &errors[0];
            assert_eq!(error.severity, Severity::Error);
            assert_eq!(error.message, message, "{text}");
            assert!(error.hints.is_empty(), "{text}");
            assert!(error.trace.is_empty(), "{text}");
            assert!(sink.into_diagnostics().is_empty(), "{text}");
            let start = text.rfind(needle).unwrap();
            assert_eq!(
                world.source.span_byte_range(error.span),
                Some(start..start + needle.len()),
                "{text}"
            );
        }
    }

    #[test]
    fn p1325_dict_direct_alias_multiline_unicode() {
        for (text, field) in [
            ("#let result = (x: 1).nope", "nope"),
            ("#let d = (x: 1)\n#let result = d.long-missing-field", "long-missing-field"),
            (
                "#let d = (x: 1)\n#let alias = d\n#let result = (alias).ausência",
                "ausência",
            ),
            ("#let result = (\n  (x: 1)\n).absent", "absent"),
        ] {
            check(text, field, &format!("dictionary does not contain key \"{field}\""));
        }
    }

    #[test]
    fn p1325_content_direct_alias_and_known_lookup_residual() {
        for (text, field, kind) in [
            ("#let result = [x].nope", "nope", "text"),
            ("#let result = strong[x].absent", "absent", "strong"),
            (
                "#let c = strong[x]\n#let alias = c\n#let result = alias.ausência",
                "ausência",
                "strong",
            ),
        ] {
            check(text, field, &format!("{kind} does not have field \"{field}\""));
        }
    }

    #[test]
    fn p1325_float_fields_without_reflection() {
        for (text, field) in [
            ("#let result = (1.0).nope", "nope"),
            ("#let f = 1.0\n#let result = f.is-infinite", "is-infinite"),
            ("#let f = 1.0\n#let result = f.ausência", "ausência"),
            ("#let result = (1.0).is-nan", "is-nan"),
        ] {
            check(text, field, "cannot access fields on type float");
        }
    }

    #[test]
    fn p1325_positive_lookup_values_and_morphology() {
        let world = TestWorld {
            source: Source::detached(""),
            library: Library::new(),
            book: FontBook::new(),
        };
        for expression in [
            "[x].text == \"x\"",
            "(x: 1).x == 1",
            "(ausência: 17).ausência == 17",
            "{ let d = (body: strong[x]); d.body == strong[x] }",
            "strong[x].body == [x]",
            "(1.0).is-nan() == false",
            "{ let d = (x: (y: 4)); d.x.y == 4 }",
        ] {
            let (result, warnings) =
                crate::compiler::eval::eval_expression(&world, expression);
            assert!(warnings.is_empty());
            assert!(
                matches!(result, Ok(crate::entities::value::Value::Bool(true))),
                "{expression}: {result:?}"
            );
        }
    }
}
#[cfg(test)]
mod p1325_located_tests {
    use crate::compiler::eval::{eval_markup, EvalContext};
    use crate::compiler::layout::FixedMetrics;
    use crate::compiler::scopes::Scopes;
    use crate::contracts::world::World;
    use crate::entities::content::Content;
    use crate::entities::engine::Engine;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::locator::Locator;
    use crate::entities::sink::Sink;
    use crate::entities::source::Source;
    use crate::entities::style_chain::StyleChain;
    use crate::entities::value::{IntrospectedContent, Value};
    use crate::entities::world_types::Route;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library,
    };
    use comemo::Track;
    use std::sync::Arc;

    struct TestWorld {
        source: Source,
        library: Library,
        book: FontBook,
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
    }

    #[test]
    fn p1325_located_ast_preserves_whole_access_for_snapshot_and_legacy() {
        for snapshot in [false, true] {
            let text = "#subject.nope";
            let world = TestWorld {
                source: Source::detached(text),
                library: Library::new(),
                book: FontBook::new(),
            };
            let mut scopes = Scopes::new(None);
            let mut locator = Locator::new();
            let content = IntrospectedContent::new(
                Content::Text("x".into()),
                snapshot.then(Default::default),
            );
            scopes.define("subject", Value::LocatedContent(content, locator.next()));
            let mut ctx = EvalContext::new();
            let route = Route::root().with_id(world.source.id());
            let mut styles = StyleChain::default_chain();
            let mut rules = Arc::from([]);
            let mut guards = Vec::new();
            let mut sink_local = Sink::new();
            let mut sink = sink_local.track_mut();
            let mut engine = Engine {
                world: &world,
                font_metrics: &FixedMetrics,
                route: route.track(),
                styles: &mut styles,
                show_rules: &mut rules,
                active_guards: &mut guards,
                current_file: world.source.id(),
                sink: &mut sink,
            };
            let errors =
                eval_markup(world.source.root(), &mut scopes, &mut ctx, &mut engine)
                    .unwrap_err();
            assert_eq!(errors.len(), 1);
            assert_eq!(errors[0].message, "text does not have field \"nope\"");
            assert_eq!(world.source.span_byte_range(errors[0].span), Some(1..text.len()));
            assert!(errors[0].hints.is_empty());
            assert!(errors[0].trace.is_empty());
        }
    }
}

#[cfg(test)]
mod p1338_tests {
    use super::{eval_value_field_access, Span, Value};
    use crate::compiler::eval::{
        eval_expression_with_features, eval_with_full_error_target_and_features,
        EvalTarget,
    };
    use crate::contracts::world::World;
    use crate::entities::compiler_features::{Feature, Features};
    use crate::entities::element_registry::ElementRegistry;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::sink::Sink;
    use crate::entities::source::Source;
    use crate::entities::source_result::Severity;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library, Route, Routines, Traced,
    };
    use comemo::Track;
    use std::num::NonZeroU16;

    struct TestWorld {
        source: Source,
        library: Library,
        book: FontBook,
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
    }

    fn profiles() -> Vec<Features> {
        (0..4)
            .map(|mask| {
                let mut features = Features::empty();
                if mask & 1 != 0 {
                    features.enable(Feature::Html);
                }
                if mask & 2 != 0 {
                    features.enable(Feature::A11yExtras);
                }
                features
            })
            .collect()
    }

    fn check_ast(text: &str, anchor: Option<&str>, message: &str) {
        for features in profiles() {
            let world = TestWorld {
                source: Source::detached(text),
                library: Library::new(),
                book: FontBook::new(),
            };
            let mut sink = Sink::new();
            let errors = eval_with_full_error_target_and_features(
                &Routines::new(),
                &world,
                Traced::default().track(),
                sink.track_mut(),
                Route::root().track(),
                &world.source,
                &ElementRegistry::new(),
                false,
                EvalTarget::Paged,
                features,
            )
            .unwrap_err();
            assert!(sink.into_diagnostics().is_empty(), "{text}");
            assert_eq!(errors.len(), 1, "{text}");
            let d = &errors[0];
            assert_eq!(d.message, message, "{text}");
            assert_eq!(d.severity, Severity::Error);
            assert!(d.hints.is_empty(), "{text}");
            assert!(d.trace.is_empty(), "{text}");
            let expected = anchor.map(|anchor| {
                let start = text.rfind(anchor).unwrap();
                start..start + anchor.len()
            });
            assert_eq!(world.source.span_byte_range(d.span), expected, "{text}");
            if anchor.is_none() {
                assert_eq!(d.span, Span::detached());
            }
        }
    }

    #[test]
    fn p1338_pure_missing_array_fields_message_and_received_span() {
        let file = FileId::from_raw(NonZeroU16::new(41).unwrap());
        for span in [
            Span::detached(),
            Span::from_range(file, 0..0),
            Span::from_range(file, 5..19),
            Span::from_range(file, 111..148),
        ] {
            for values in [
                vec![],
                vec![Value::None],
                vec![
                    Value::Bool(false),
                    Value::Int(73),
                    Value::Str("ação 🦊".into()),
                    Value::Auto,
                ],
                vec![Value::Array(vec![Value::Int(-1)]), Value::None],
            ] {
                for field in [
                    "missing",
                    "ausência",
                    "unknown-long-field",
                    "lenx",
                    "First",
                    "last-",
                    "",
                ] {
                    let errors = eval_value_field_access(
                        Value::Array(values.clone()),
                        field,
                        span,
                    )
                    .unwrap_err();
                    assert_eq!(errors.len(), 1);
                    let d = &errors[0];
                    assert_eq!(d.message, "cannot access fields on type array");
                    assert_eq!(d.span, span);
                    assert_eq!(d.severity, Severity::Error);
                    assert!(d.hints.is_empty());
                    assert!(d.trace.is_empty());
                }
            }
        }
    }

    #[test]
    fn p1338_pure_preserve_len_first_last_and_empty_none() {
        for values in [
            vec![],
            vec![Value::Auto],
            vec![Value::Str("primeiro".into()), Value::Bool(false), Value::Int(83)],
            vec![Value::None, Value::Array(vec![Value::Int(9)])],
        ] {
            for span in [
                Span::detached(),
                Span::from_range(FileId::from_raw(NonZeroU16::new(47).unwrap()), 9..31),
            ] {
                for (field, expected) in [
                    ("len", Value::Int(values.len() as i64)),
                    ("first", values.first().cloned().unwrap_or(Value::None)),
                    ("last", values.last().cloned().unwrap_or(Value::None)),
                ] {
                    let array = Value::Array(values.clone());
                    assert_eq!(
                        eval_value_field_access(array, field, span).unwrap(),
                        expected,
                        "{field}"
                    );
                }
            }
        }
    }

    #[test]
    fn p1338_ast_empty_heterogeneous_alias_unicode_multiline() {
        for (text, field) in [
            ("#(().absent)", "absent"),
            ("#((1,).missing)", "missing"),
            ("#((false, none, auto, \"ação 🦊\").ausência)", "ausência"),
            ("#((((1, 2))).long-missing-field)", "long-missing-field"),
            (
                "#let array-value = (1, 2)\n#let alias = array-value\n#alias.unknown",
                "unknown",
            ),
            ("#let início = ()\n#(\n (início)\n .desconhecido\n)", "desconhecido"),
            ("#let repeated = (none,)\n#repeated.repeated", "repeated"),
        ] {
            check_ast(text, Some(field), "cannot access fields on type array");
        }
    }

    #[test]
    fn p1338_preserve_predispatch_empty_first_last() {
        for text in ["#(().first)", "#(().last)"] {
            check_ast(text, None, "array is empty and no default value was specified");
        }
    }

    #[test]
    fn p1338_preserve_other_categories_diagnostics() {
        for (text, anchor, message) in [
            ("#((1pt).nope)", "(1pt).nope", "length does not contain field \"nope\""),
            ("#array.nope", "nope", "type array does not contain field `nope`"),
            ("#(true.nope)", "nope", "cannot access fields on type boolean"),
            ("#(none.nope)", "nope", "cannot access fields on type none"),
            ("#(auto.nope)", "nope", "cannot access fields on type auto"),
            ("#((12).nope)", "nope", "cannot access fields on type integer"),
            ("#(\"abc\".nope)", "nope", "cannot access fields on type string"),
        ] {
            check_ast(text, Some(anchor), message);
        }
    }

    #[test]
    fn p1338_preserve_ast_members_and_real_method_calls() {
        for features in profiles() {
            let world = TestWorld {
                source: Source::detached(""),
                library: Library::new(),
                book: FontBook::new(),
            };
            for (expr, expected) in [
                ("().len", Value::Int(0)),
                ("(7, 19).len", Value::Int(2)),
                ("(7, 19).first", Value::Int(7)),
                ("(7, 19).last", Value::Int(19)),
                ("(7, 19).len()", Value::Int(2)),
                ("array.len((7, 19))", Value::Int(2)),
                ("(7, 19).first()", Value::Int(7)),
                ("(7, 19).last()", Value::Int(19)),
                ("(7, 19).at(1)", Value::Int(19)),
                ("{let alias = array.len; alias((none, auto, false))}", Value::Int(3)),
                ("type((none, auto)) == array", Value::Bool(true)),
                ("repr((none, auto))", Value::Str("(none, auto)".into())),
            ] {
                let (result, warnings) =
                    eval_expression_with_features(&world, expr, features);
                assert!(warnings.is_empty(), "{expr}");
                assert_eq!(result.unwrap(), expected, "{expr}");
            }
        }
    }
}

#[cfg(test)]
mod p1337_tests {
    use super::{eval_value_field_access, Span, Value};
    use crate::compiler::eval::{
        eval_expression_with_features, eval_with_full_error_target_and_features,
        EvalTarget,
    };
    use crate::contracts::world::World;
    use crate::entities::compiler_features::{Feature, Features};
    use crate::entities::element_registry::ElementRegistry;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::sink::Sink;
    use crate::entities::source::Source;
    use crate::entities::source_result::Severity;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library, Route, Routines, Traced,
    };
    use comemo::Track;
    use std::num::NonZeroU16;

    struct TestWorld {
        source: Source,
        library: Library,
        book: FontBook,
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
    }

    fn profiles() -> Vec<Features> {
        (0..4)
            .map(|mask| {
                let mut features = Features::empty();
                if mask & 1 != 0 {
                    features.enable(Feature::Html);
                }
                if mask & 2 != 0 {
                    features.enable(Feature::A11yExtras);
                }
                features
            })
            .collect()
    }

    fn check_ast(text: &str, anchor: &str, message: &str) {
        for features in profiles() {
            let world = TestWorld {
                source: Source::detached(text),
                library: Library::new(),
                book: FontBook::new(),
            };
            let mut sink = Sink::new();
            let errors = eval_with_full_error_target_and_features(
                &Routines::new(),
                &world,
                Traced::default().track(),
                sink.track_mut(),
                Route::root().track(),
                &world.source,
                &ElementRegistry::new(),
                false,
                EvalTarget::Paged,
                features,
            )
            .unwrap_err();
            assert!(sink.into_diagnostics().is_empty(), "{text}");
            assert_eq!(errors.len(), 1, "{text}");
            let d = &errors[0];
            assert_eq!(d.message, message, "{text}");
            assert_eq!(d.severity, Severity::Error);
            assert!(d.hints.is_empty(), "{text}");
            assert!(d.trace.is_empty(), "{text}");
            let start = text.rfind(anchor).unwrap();
            assert_eq!(
                world.source.span_byte_range(d.span),
                Some(start..start + anchor.len()),
                "{text}"
            );
        }
    }

    #[test]
    fn p1338_successor_array_missing_message_and_field_span() {
        // Array is an excluded baseline debt, not a P1337 parity correction.
        check_ast("#((1, 2).missing)", "missing", "cannot access fields on type array");
    }

    #[test]
    fn p1337_pure_all_singletons_bool_values_names_and_supplied_spans() {
        let id = FileId::from_raw(NonZeroU16::new(37).unwrap());
        for span in [
            Span::detached(),
            Span::from_range(id, 0..0),
            Span::from_range(id, 7..19),
            Span::from_range(id, 81..113),
        ] {
            for (value, name) in [
                (Value::Bool(true), "boolean"),
                (Value::Bool(false), "boolean"),
                (Value::None, "none"),
                (Value::Auto, "auto"),
            ] {
                for field in
                    ["nope", "ausência", "unknown-long-field", "len", "value", ""]
                {
                    let errors =
                        eval_value_field_access(value.clone(), field, span).unwrap_err();
                    assert_eq!(errors.len(), 1);
                    let d = &errors[0];
                    assert_eq!(d.message, format!("cannot access fields on type {name}"));
                    assert_eq!(d.span, span);
                    assert_eq!(d.severity, Severity::Error);
                    assert!(d.hints.is_empty());
                    assert!(d.trace.is_empty());
                }
            }
        }
    }

    #[test]
    fn p1337_ast_variants_aliases_parentheses_unicode_and_multiline() {
        for (literal, name) in [
            ("true", "boolean"),
            ("false", "boolean"),
            ("none", "none"),
            ("auto", "auto"),
        ] {
            for (text, field) in [
                (format!("#(({literal}).nope)"), "nope"),
                (format!("#((({literal})).absent-long-field)"), "absent-long-field"),
                (
                    format!(
                        "#let início = {literal}\n#let alias = início\n#alias.ausência"
                    ),
                    "ausência",
                ),
                (
                    format!(
                        "#let variável = {literal}\n#(\n (variável)\n .campo-ausente\n)"
                    ),
                    "campo-ausente",
                ),
                (format!("#let repeated = {literal}\n#repeated.repeated"), "repeated"),
            ] {
                check_ast(&text, field, &format!("cannot access fields on type {name}"));
            }
        }
    }

    #[test]
    fn p1337_preserve_values_type_and_repr_and_ordinary_calls() {
        for features in profiles() {
            let world = TestWorld {
                source: Source::detached(""),
                library: Library::new(),
                book: FontBook::new(),
            };
            for (expr, expected) in [
                ("type(true) == bool", Value::Bool(true)),
                ("type(false) == bool", Value::Bool(true)),
                ("repr(true)", Value::Str("true".into())),
                ("repr(false)", Value::Str("false".into())),
                ("repr(none)", Value::Str("none".into())),
                ("repr(auto)", Value::Str("auto".into())),
                ("{let flag = false; not flag}", Value::Bool(true)),
                ("{let identity(x) = x; identity(none)}", Value::None),
                ("{let identity(x) = x; identity.with(auto)()}", Value::Auto),
                ("(truth: true, nothing: none, automatic: auto).nothing", Value::None),
                ("(truth: true, nothing: none, automatic: auto).automatic", Value::Auto),
            ] {
                let (value, warnings) =
                    eval_expression_with_features(&world, expr, features);
                assert!(warnings.is_empty(), "{expr}");
                assert_eq!(value.unwrap(), expected, "{expr}");
            }
        }
    }
}

#[cfg(test)]
mod p1336_tests {
    use super::{eval_value_field_access, Span, Value};
    use crate::compiler::eval::{
        eval_expression_with_features, eval_with_full_error_target_and_features,
        EvalTarget,
    };
    use crate::contracts::world::World;
    use crate::entities::compiler_features::{Feature, Features};
    use crate::entities::element_registry::ElementRegistry;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::sink::Sink;
    use crate::entities::source::Source;
    use crate::entities::source_result::Severity;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library, Route, Routines, Traced,
    };
    use comemo::Track;
    use std::num::NonZeroU16;

    struct TestWorld {
        source: Source,
        library: Library,
        book: FontBook,
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
    }

    fn profiles() -> Vec<Features> {
        (0..4)
            .map(|mask| {
                let mut features = Features::empty();
                if mask & 1 != 0 {
                    features.enable(Feature::Html);
                }
                if mask & 2 != 0 {
                    features.enable(Feature::A11yExtras);
                }
                features
            })
            .collect()
    }

    fn check_ast(text: &str, anchor: &str, message: &str) {
        for features in profiles() {
            let world = TestWorld {
                source: Source::detached(text),
                library: Library::new(),
                book: FontBook::new(),
            };
            let mut sink = Sink::new();
            let errors = eval_with_full_error_target_and_features(
                &Routines::new(),
                &world,
                Traced::default().track(),
                sink.track_mut(),
                Route::root().track(),
                &world.source,
                &ElementRegistry::new(),
                false,
                EvalTarget::Paged,
                features,
            )
            .unwrap_err();
            assert!(sink.into_diagnostics().is_empty(), "{text}");
            assert_eq!(errors.len(), 1, "{text}");
            let d = &errors[0];
            assert_eq!(d.message, message, "{text}");
            assert_eq!(d.severity, Severity::Error);
            assert!(d.hints.is_empty(), "{text}");
            assert!(d.trace.is_empty(), "{text}");
            let start = text.rfind(anchor).unwrap();
            assert_eq!(
                world.source.span_byte_range(d.span),
                Some(start..start + anchor.len()),
                "{text}"
            );
        }
    }

    #[test]
    fn p1336_pure_lookup_names_and_supplied_span() {
        let file = FileId::from_raw(NonZeroU16::new(9).unwrap());
        for span in [
            Span::detached(),
            Span::from_range(file, 3..8),
            Span::from_range(file, 88..119),
        ] {
            for (value, name) in [
                (Value::Int(0), "integer"),
                (Value::Int(-812), "integer"),
                (Value::Int(i64::MAX), "integer"),
                (Value::Str("".into()), "string"),
                (Value::Str("ação 🦊".into()), "string"),
            ] {
                for field in
                    ["missing", "absent-long-field", "ausência", "len", "bit-and"]
                {
                    let errors =
                        eval_value_field_access(value.clone(), field, span).unwrap_err();
                    assert_eq!(errors.len(), 1);
                    assert_eq!(
                        errors[0].message,
                        format!("cannot access fields on type {name}")
                    );
                    assert_eq!(errors[0].span, span);
                    assert_eq!(errors[0].severity, Severity::Error);
                    assert!(errors[0].hints.is_empty());
                    assert!(errors[0].trace.is_empty());
                }
            }
        }
    }

    #[test]
    fn p1336_int_ast_identifier_span() {
        for (text, field) in [
            ("#((0).missing)", "missing"),
            ("#(((-812)).absent-long-field)", "absent-long-field"),
            ("#let n = 823\n#let alias = n\n#alias.unknown", "unknown"),
            ("#let número = 19\n#número.ausência", "ausência"),
            ("#let n = 7\n#(\n (n)\n .missing\n)", "missing"),
            ("#((3).bit-and)", "bit-and"),
        ] {
            check_ast(text, field, "cannot access fields on type integer");
        }
    }

    #[test]
    fn p1336_str_ast_identifier_span() {
        for (text, field) in [
            ("#(\"\".missing)", "missing"),
            ("#(((\"ação 🦊\")).absent-long-field)", "absent-long-field"),
            ("#let s = \"abc\"\n#let alias = s\n#alias.unknown", "unknown"),
            ("#let texto = \"ação\"\n#texto.ausência", "ausência"),
            ("#let s = \"abc\"\n#(\n (s)\n .missing\n)", "missing"),
        ] {
            check_ast(text, field, "cannot access fields on type string");
        }
    }

    #[test]
    fn p1337_successor_bool_none_auto_lookup_names_and_ast_span() {
        for (text, anchor, message) in [
            ("#(true.nope)", "nope", "cannot access fields on type boolean"),
            ("#(none.nope)", "nope", "cannot access fields on type none"),
            ("#(auto.nope)", "nope", "cannot access fields on type auto"),
        ] {
            check_ast(text, anchor, message);
        }
    }

    #[test]
    fn p1336_preserve_int_str_methods_and_type_namespaces() {
        for features in profiles() {
            let world = TestWorld {
                source: Source::detached(""),
                library: Library::new(),
                book: FontBook::new(),
            };
            for (expr, expected) in [
                ("(7).bit-and(3)", Value::Int(3)),
                ("int.bit-and(7,3)", Value::Int(3)),
                ("\"abc\".len()", Value::Int(3)),
                ("str.len(\"abc\")", Value::Int(3)),
                ("\"abc\".len", Value::Int(3)),
                ("\"abc\".contains(\"b\")", Value::Bool(true)),
                ("type(19) == int", Value::Bool(true)),
                ("type(\"abc\") == str", Value::Bool(true)),
                ("repr(19)", Value::Str("19".into())),
                ("repr(\"abc\")", Value::Str("\"abc\"".into())),
            ] {
                let (result, warnings) =
                    eval_expression_with_features(&world, expr, features);
                assert!(warnings.is_empty(), "{expr}");
                assert_eq!(result.unwrap(), expected, "{expr}");
            }
        }
    }
}

#[cfg(test)]
mod p1326_tests {
    use crate::compiler::eval::{
        eval_expression_with_features, eval_with_full_error_target_and_features,
    };
    use crate::contracts::world::World;
    use crate::entities::compiler_features::{Feature, Features};
    use crate::entities::element_registry::ElementRegistry;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::module::Module;
    use crate::entities::sink::Sink;
    use crate::entities::source::Source;
    use crate::entities::source_result::{Severity, SourceDiagnostic, SourceResult};
    use crate::entities::value::Value;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library, Route, Routines, Traced,
    };
    use comemo::Track;

    struct TestWorld {
        source: Source,
        library: Library,
        book: FontBook,
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
    }
    fn profiles() -> Vec<Features> {
        (0..4)
            .map(|bits| {
                let mut f = Features::empty();
                if bits & 1 != 0 {
                    f.enable(Feature::Html);
                }
                if bits & 2 != 0 {
                    f.enable(Feature::A11yExtras);
                }
                f
            })
            .collect()
    }
    fn full(
        text: &str,
        features: Features,
    ) -> (Source, SourceResult<Module>, Vec<SourceDiagnostic>) {
        let source = Source::detached(text);
        let world = TestWorld {
            source: source.clone(),
            library: Library::new(),
            book: FontBook::new(),
        };
        let mut sink = Sink::new();
        let result = eval_with_full_error_target_and_features(
            &Routines::new(),
            &world,
            Traced::default().track(),
            sink.track_mut(),
            Route::root().track(),
            &source,
            &ElementRegistry::new(),
            false,
            crate::compiler::eval::EvalTarget::Paged,
            features,
        );
        (source, result, sink.into_diagnostics())
    }
    fn error(text: &str, anchor: &str, message: &str) {
        for features in profiles() {
            let (source, result, warnings) = full(text, features);
            assert!(warnings.is_empty(), "{text}: {warnings:?}");
            let errors = result.unwrap_err();
            assert_eq!(errors.len(), 1, "{text}");
            let d = &errors[0];
            assert_eq!(d.message, message, "{text}");
            assert_eq!(d.severity, Severity::Error);
            assert!(d.hints.is_empty(), "{text}: {:?}", d.hints);
            assert!(d.trace.is_empty(), "{text}: {:?}", d.trace);
            let start = text.rfind(anchor).unwrap();
            assert_eq!(
                source.span_byte_range(d.span),
                Some(start..start + anchor.len()),
                "{text}"
            );
        }
    }
    #[test]
    fn p1326_full_ast_closure_message_and_exact_identifier() {
        for (text, field) in [
            ("#let f(x) = x\n#f.nope", "nope"),
            ("#((x => x).absent)", "absent"),
            ("#let f(x) = x\n#let alias = f\n#alias.unknown", "unknown"),
            ("#let f(x) = x\n#f.with(7).nope", "nope"),
            (
                "#let f(x, y) = x+y\n#f.with(7).with(8).missing-long-field",
                "missing-long-field",
            ),
            ("#let função(x) = x\n#função.café", "café"),
            ("#let f(x) = x\n#(\n  f\n  .ausência\n)", "ausência"),
            ("#let f(x) = panic(\"must not run\")\n#f.nope(7)", "nope"),
            ("#let f(x) = x\n#let alias = f.with(4)\n#alias.nope()", "nope"),
        ] {
            error(text, field, "cannot access fields on user-defined functions");
        }
    }
    #[test]
    fn p1326_full_ast_native_module_dict_content_float_boundaries() {
        for (text, field, message) in [
            ("#json.nope", "nope", "function `json` does not contain field `nope`"),
            ("#csv.nope", "nope", "function `csv` does not contain field `nope`"),
            ("#calc.nope", "nope", "module `calc` does not contain `nope`"),
            ("#((x: 1).nope)", "nope", "dictionary does not contain key \"nope\""),
            ("#(strong[x].nope)", "nope", "strong does not have field \"nope\""),
            ("#((1.0).nope)", "nope", "cannot access fields on type float"),
        ] {
            error(text, field, message);
        }
    }
    #[test]
    fn p1326_existing_function_calls_with_alias_args_and_kind() {
        for features in profiles() {
            let world = TestWorld {
                source: Source::detached(""),
                library: Library::new(),
                book: FontBook::new(),
            };
            for (expr, expected) in [
                ("{let f(x)=x+1; f(8)}", Value::Int(9)),
                ("{let f(x,y)=x+y; let a=f.with(8); a(3)}", Value::Int(11)),
                ("{let f(x,y)=x+y; f.with(8).with(3)()}", Value::Int(11)),
                ("{let f(x)=x; type(f)==function}", Value::Bool(true)),
                ("{let f(x)=x; let a=f; a(19)}", Value::Int(19)),
                ("json.encode((x: 1))", Value::Str("{\n  \"x\": 1\n}".into())),
            ] {
                let (actual, warnings) =
                    eval_expression_with_features(&world, expr, features);
                assert!(warnings.is_empty(), "{expr}");
                assert_eq!(actual.unwrap(), expected, "{expr}");
            }
        }
    }
}

/// P1471 — RED isolado (Agente B), a partir de
/// `compiler/eval/bindings/field_access.md` P1471: com `pdf` retificado
/// para `Value::Func`, o alvo 1 de `is_a11y_pdf_target`
/// (`Value::Module(module) => module.name() == "pdf"`) fica morto —
/// `pdf` nunca mais é `Value::Module`. O predicate deve ser simplificado
/// para o único caso remanescente (`Value::Func(func) =>
/// is_format_pdf_function(func)`). Nenhuma implementação foi consultada.
///
/// Item 7 do passo — este owner "não ganha nem perde diagnóstico,
/// mensagem, hint ou span" para `format.pdf.table-summary`/`header-cell`/
/// `data-cell` sem `a11y-extras`: o comportamento observável medido em
/// P1464 é **idêntico antes e depois** da simplificação. Por isso
/// `p1471_format_pdf_a11y_gate_message_and_hints_stable_across_a11y` não é
/// RED por si — é teste de não-regressão que deve continuar GREEN antes e
/// depois, provando que a limpeza do branch morto não quebra o gate.
///
/// Nota de medição (ADR-0108): o comportamento do alvo *bare* `pdf.<gated>`
/// sem `a11y-extras` (hoje servido pelo alvo 1, que reconhece qualquer
/// `Value::Module` chamado "pdf") não é fixado por este L0 para depois da
/// retificação — `is_format_pdf_namespace` exige `pages`/`standard`/
/// `tagged`/`pretty` no namespace, que o `pdf` bruto não tem (só
/// `format.pdf` os tem). Por isso nenhum teste aqui fixa a mensagem exata
/// do caso bare pós-retificação; só se afirma que continua a ser erro
/// (cobertura equivalente já está em `compiler/stdlib/pdf.rs`
/// `p1471_pdf_field_access_members_present_across_profiles`).
#[cfg(test)]
mod p1471_tests {
    use super::{eval_value_field_access, Span, Value};
    use crate::compiler::eval::eval_expression_with_features;
    use crate::contracts::world::World;
    use crate::entities::compiler_features::{Feature, Features};
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::source::Source;
    use crate::entities::source_result::Severity;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library,
    };

    struct TestWorld {
        source: Source,
        library: Library,
        book: FontBook,
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
    }

    fn world_for(text: &str) -> TestWorld {
        TestWorld {
            source: Source::detached(text),
            library: Library::new(),
            book: FontBook::new(),
        }
    }

    fn profiles() -> Vec<Features> {
        (0..4u8)
            .map(|mask| {
                let mut features = Features::empty();
                if mask & 1 != 0 {
                    features.enable(Feature::Html);
                }
                if mask & 2 != 0 {
                    features.enable(Feature::A11yExtras);
                }
                features
            })
            .collect()
    }

    /// Item 7 — não-regressão: `format.pdf.table-summary`/`header-cell`/
    /// `data-cell` sem `a11y-extras` continuam a produzir exatamente a
    /// mensagem e os dois hints medidos em P1464, com o field ancorado; com
    /// `a11y-extras`, o lookup ordinário devolve a função. Deve valer nos
    /// quatro perfis (a gate independe de `Feature::Html`) e continuar
    /// GREEN antes e depois de P1471 — a simplificação de
    /// `is_a11y_pdf_target` remove só o branch morto de `Value::Module`.
    #[test]
    fn p1471_format_pdf_a11y_gate_message_and_hints_stable_across_profiles() {
        for features in profiles() {
            let has_a11y = features.contains(Feature::A11yExtras);
            for field in ["table-summary", "header-cell", "data-cell"] {
                let expr = format!("format.pdf.{field}");
                let world = world_for(&expr);
                let (result, warnings) =
                    eval_expression_with_features(&world, &expr, features);
                assert!(warnings.is_empty(), "{expr} em {features:?}: {warnings:?}");
                if has_a11y {
                    let Value::Func(func) = result.unwrap() else {
                        panic!("format.pdf.{field} deve ser função com a11y-extras");
                    };
                    let expected_name = format!("pdf.{field}");
                    assert_eq!(func.name(), Some(expected_name.as_str()));
                } else {
                    let errors = result.unwrap_err();
                    assert_eq!(errors.len(), 1, "{expr} em {features:?}: {errors:?}");
                    let d = &errors[0];
                    assert_eq!(
                        d.message,
                        format!(
                            "cannot access field `{field}` because the `a11y-extras` feature is not enabled"
                        ),
                        "{expr} em {features:?}"
                    );
                    assert_eq!(d.severity, Severity::Error);
                    assert_eq!(
                        d.hints,
                        vec![
                            "try enabling the `a11y-extras` feature".to_string(),
                            "see https://typst.app/help/compiler-features for more details"
                                .to_string(),
                        ],
                        "{expr} em {features:?}"
                    );
                    // Nota de medição: `eval_expression_with_features` faz o
                    // parsing num `Source` interno próprio (não o
                    // `world.source` construído aqui só para satisfazer o
                    // trait `World`), e spans de ficheiros Typst são
                    // *numbered spans* (`entities/span.rs`) — só decodíveis
                    // pela árvore que os atribuiu. Por isso este teste
                    // verifica apenas que o span está ancorado (não
                    // detached), sem tentar decodificar o byte range a
                    // partir de um `Source` diferente do que foi parseado.
                    assert_ne!(
                        d.span,
                        Span::detached(),
                        "{expr} em {features:?}: span deve estar ancorado no field"
                    );
                }
            }
        }
    }

    /// Não-regressão auxiliar — `eval_value_field_access` pura continua a
    /// tratar um `Value::Module` avulso (não a raiz `pdf`/`format`) como
    /// hoje: sem o intercept especial, cai no diagnóstico genérico de
    /// campo ausente. Confirma que a simplificação do predicate não
    /// amplia o alcance do gate para módulos homónimos arbitrários.
    #[test]
    fn p1471_arbitrary_module_named_pdf_is_not_special_cased_by_pure_lookup() {
        use crate::entities::scope::Scope;
        let module = crate::entities::module::Module::new("pdf", Scope::new());
        let err = eval_value_field_access(
            Value::Module(module),
            "table-summary",
            Span::detached(),
        )
        .unwrap_err();
        assert_eq!(err.len(), 1);
        assert_ne!(
            err[0].message,
            "cannot access field `table-summary` because the `a11y-extras` feature is not enabled",
            "eval_value_field_access pura não deve aplicar o gate a11y: {}",
            err[0].message
        );
    }
}
