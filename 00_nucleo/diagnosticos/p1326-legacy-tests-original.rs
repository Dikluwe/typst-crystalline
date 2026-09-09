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
                                expect_error(f, field, span, &format!("function `{public}` does not contain field `{field}`"));
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
        for f in [closure, element] {
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
        for f in [closure, element, plugin] {
            assert_error(f.clone(), "cannot access fields on type function");
            assert_error(
                f.with(Args::positional(vec![])),
                "cannot access fields on type function",
            );
        }
    }
}

