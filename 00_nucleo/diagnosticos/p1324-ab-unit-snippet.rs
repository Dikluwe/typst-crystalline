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
            (false, None) => Func::native(name, |_, _, _, _| panic!("lookup invoked native")),
            (false, Some(scope)) => Func::native_with_namespace(name, |_, _, _, _| panic!("lookup invoked native"), scope),
            (true, None) => Func::native_with_engine(name, |_, _, _, _, _, _| panic!("lookup invoked engine native")),
            (true, Some(scope)) => Func::native_with_engine_and_namespace(name, |_, _, _, _, _, _| panic!("lookup invoked engine native"), scope),
        }
    }

    fn wrapped(mut func: Func, depth: usize) -> Func {
        for ordinal in 0..depth {
            func = func.with(Args::positional(vec![Value::Int(ordinal as i64 + 91)]));
        }
        func
    }

    fn anchors() -> [Span; 2] {
        let id = crate::entities::file_id::FileId::from_raw(std::num::NonZeroU16::new(271).unwrap());
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
                if populated { scope.define("present", Value::Int(42)); }
                let scope = Arc::new(scope);
                for (internal, public) in [("meteor", "meteor"), ("space.meteor", "meteor"), ("alpha.beta.gamma", "gamma")] {
                    for depth in [0, 1, 3] {
                        for field in ["missing", "absent_1324"] {
                            for span in anchors() {
                                let f = wrapped(subject(engine, internal, Some(scope.clone())), depth);
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
                let f = wrapped(subject(engine, "anything.owner", Some(scope.clone())), depth);
                for span in anchors() {
                    expect_error(f.clone(), "missing", span, "function `owner` does not contain field `missing`");
                    for (field, expected) in [("present", Value::Int(713)), ("nothing", Value::None), ("enabled", Value::Bool(false)), ("callable", Value::Func(member.clone()))] {
                        let actual = eval_value_field_access(Value::Func(f.clone()), field, span).unwrap();
                        assert_eq!(actual, expected);
                        if field == "callable" {
                            let Value::Func(actual) = actual else { panic!("member lost function kind") };
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
                    expect_error(f.clone(), "different", span, "function `original` does not contain field `different`");
                }
                assert!(f.namespace().is_none());
            }
        }
    }

    #[test]
    fn p1324_named_closure_and_user_element_are_not_native() {
        let closure = Func::closure(ClosureRepr {
            name: Some("alpha.beta.gamma".into()), params: vec![], sink_name: None,
            body: SyntaxNode::placeholder(SyntaxKind::Markup),
            captured: Arc::new(Scope::new()), capturer: Capturer::Function,
        });
        let element = Func::element("alpha.beta.gamma", Arc::new(|_| panic!("lookup invoked element")));
        for f in [closure, element] {
            for depth in [0, 1, 3] {
                for span in anchors() {
                    expect_error(wrapped(f.clone(), depth), "missing", span, "cannot access fields on type function");
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
            fn load(&self, _: &[u8]) -> Result<PluginModuleId, PluginError> { panic!("load") }
            fn exports(&self, _: PluginModuleId) -> Result<Vec<ecow::EcoString>, PluginError> { panic!("exports") }
            fn call(&self, _: PluginModuleId, _: &str, _: &[Bytes]) -> Result<Bytes, PluginError> { panic!("call") }
            fn transition(&self, _: PluginModuleId, _: &str, _: &[Bytes]) -> Result<PluginModuleId, PluginError> { panic!("transition") }
        }
        let f = Func::plugin(crate::entities::plugin_func::PluginFunc {
            host: Arc::new(UncalledHost), module: PluginModuleId(19), name: "alpha.beta.gamma".into(),
        });
        for depth in [0, 1, 3] {
            for span in anchors() {
                expect_error(wrapped(f.clone(), depth), "missing", span, "cannot access fields on type function");
            }
        }
    }
}
