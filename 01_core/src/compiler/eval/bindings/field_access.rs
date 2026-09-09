//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/bindings/field_access.md
//! @prompt-hash 6f7aad92
//! @layer L1
//! @updated 2026-09-01
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

/// Public name of a native, including namespaces and partial applications.
fn native_public_name(mut func: &Func) -> Option<&str> {
    use crate::entities::func::FuncRepr;
    loop {
        match func.repr() {
            FuncRepr::Native(native) => {
                return native.name.rsplit('.').next();
            }
            FuncRepr::NativeWithEngine(native) => {
                return native.name.rsplit('.').next();
            }
            FuncRepr::With(with) => func = &with.0,
            _ => return None,
        }
    }
}

pub(in crate::compiler::eval) fn eval_field_access(
    access: crate::entities::ast::expr::FieldAccess<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    use crate::entities::ast::AstNode;

    let target = eval_expr(access.target(), scopes, ctx, engine)?;
    let field = access.field().as_str();

    // P509 — field access em coleções despacha para métodos de instância.
    if let Some(result) = crate::compiler::stdlib::try_dispatch_collection_method(
        target.clone(),
        field,
        crate::entities::args::Args::positional(vec![]),
        None,
        scopes,
        ctx,
        engine,
    ) {
        return result;
    }

    // P792 — `text.<campo>`: quando o target é Value::Func("text") e o field é
    // um parâmetro de estilo, ler da StyleChain activa.
    if let Value::Func(ref f) = target {
        if f.name() == Some("text") {
            match field {
                "size" => {
                    return Ok(Value::Length(crate::entities::layout_types::Length::pt(
                        engine.styles.size(),
                    )));
                }
                "lang" => {
                    let lang = match engine.styles.custom("text.lang") {
                        Some(Value::Str(s)) => s.clone(),
                        _ => "en".into(),
                    };
                    return Ok(Value::Str(lang));
                }
                _ => {} // cai no eval_value_field_access normal
            }
        }
    }

    // P820 (achado #7 de P810) — acesso a símbolo **depreciado** do módulo
    // `sym` (`#sym.join`): warning verbatim do vanilla com span no campo
    // (medido: `#sym.join` → warning @1:5, exit 0). O mecanismo geral do
    // vanilla é `Deprecation` em `Binding` (`foundations/scope.rs:288`);
    // aqui, data-driven pela tabela `SYM_DEPRECATED` do módulo `sym`.
    if let Value::Module(ref m) = target {
        if m.name() == "pdf"
            && matches!(field, "table-summary" | "header-cell" | "data-cell")
            && !ctx
                .features
                .contains(crate::entities::compiler_features::Feature::A11yExtras)
        {
            return Err(vec![SourceDiagnostic::error(
                access.field().span(),
                format!(
                    "cannot access field `{field}` because the `a11y-extras` feature is not enabled"
                ),
            )
            .with_hint("try enabling the `a11y-extras` feature")
            .with_hint("see https://typst.app/help/compiler-features for more details")]);
        }
        if m.name() == "sym" {
            if let Some(msg) = crate::compiler::stdlib::sym::sym_deprecation(field) {
                engine.sink.warn_note(access.field().span(), msg, "");
            }
        }
    }

    let span = if matches!(
        &target,
        Value::Module(_) | Value::Dict(_) | Value::Content(_) | Value::Float(_)
    ) || matches!(&target, Value::Func(f) if native_public_name(f).is_some())
    {
        access.field().span()
    } else {
        access.span()
    };
    eval_value_field_access(target, field, span)
}

pub(in crate::compiler::eval) fn eval_value_field_access(
    target: Value,
    field: &str,
    span: Span,
) -> SourceResult<Value> {
    use crate::entities::source_result::SourceDiagnostic;
    match target {
        Value::Dict(d) => d.get(field).cloned().ok_or_else(|| {
            vec![SourceDiagnostic::error(
                span,
                format!("dictionary does not contain key \"{field}\""),
            )]
        }),
        // Field access em elementos estruturados — usado por show rules (Passo 68).
        Value::LocatedContent(c, _) => {
            let value = match c.fields() {
                Some(fields) => fields.get(field).cloned(),
                None => c.content().get_field(field),
            };
            value.ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    span,
                    format!(
                        "{} does not have field \"{field}\"",
                        c.content().elem_name()
                    ),
                )]
            })
        }
        Value::Content(c) => c.get_field(field).ok_or_else(|| {
            vec![SourceDiagnostic::error(
                span,
                format!("{} does not have field \"{field}\"", c.elem_name()),
            )]
        }),
        // P785b — Field access em Value::Relative (RelativeLength / Rel)
        Value::Relative(rel) => match field {
            "ratio" => Ok(Value::Ratio(crate::entities::layout_types::Ratio(rel.rel))),
            "length" => Ok(Value::Length(rel.abs)),
            _ => Err(vec![SourceDiagnostic::error(
                span,
                format!("relative length does not contain field \"{field}\""),
            )]),
        },
        // P785b — Field access em Value::Align (Alignment)
        Value::Align(align) => match field {
            "x" => Ok(align
                .h
                .map(|h| {
                    Value::Align(crate::entities::layout_types::Align2D {
                        h: Some(h),
                        v: None,
                    })
                })
                .unwrap_or(Value::None)),
            "y" => Ok(align
                .v
                .map(|v| {
                    Value::Align(crate::entities::layout_types::Align2D {
                        h: None,
                        v: Some(v),
                    })
                })
                .unwrap_or(Value::None)),
            _ => Err(vec![SourceDiagnostic::error(
                span,
                format!("alignment does not contain field \"{field}\""),
            )]),
        },
        // P785b — Field access em Value::Length
        Value::Length(len) => match field {
            "em" => Ok(Value::Float(len.em)),
            "abs" => Ok(Value::Length(crate::entities::layout_types::Length::pt(
                len.abs.to_pt(),
            ))),
            _ => Err(vec![SourceDiagnostic::error(
                span,
                format!("length does not contain field \"{field}\""),
            )]),
        },
        // P785b — Field access em Value::Stroke
        Value::Stroke(stroke) => match field {
            "paint" => Ok(Value::Color(stroke.paint.to_color())),
            "thickness" => Ok(Value::Length(crate::entities::layout_types::Length::pt(
                stroke.thickness,
            ))),
            _ => Err(vec![SourceDiagnostic::error(
                span,
                format!("stroke does not contain field \"{field}\""),
            )]),
        },
        // P684 — Field access em Value::Version
        Value::Version(v) => match field {
            "major" => Ok(Value::Int(v.component(0) as i64)),
            "minor" => Ok(Value::Int(v.component(1) as i64)),
            "patch" => Ok(Value::Int(v.component(2) as i64)),
            _ => Err(vec![SourceDiagnostic::error(
                span,
                format!("version does not contain field \"{field}\""),
            )]),
        },
        // P412 — Field access em Value::Duration
        Value::Duration(d) => {
            const NANOS_PER_SECOND: f64 = 1_000_000_000.0;
            const NANOS_PER_MINUTE: f64 = 60_000_000_000.0;
            const NANOS_PER_HOUR: f64 = 3_600_000_000_000.0;
            const NANOS_PER_DAY: f64 = 86_400_000_000_000.0;
            match field {
                "seconds" => Ok(Value::Float(d.nanos as f64 / NANOS_PER_SECOND)),
                "minutes" => Ok(Value::Float(d.nanos as f64 / NANOS_PER_MINUTE)),
                "hours" => Ok(Value::Float(d.nanos as f64 / NANOS_PER_HOUR)),
                "days" => Ok(Value::Float(d.nanos as f64 / NANOS_PER_DAY)),
                _ => Err(vec![SourceDiagnostic::error(
                    span,
                    format!("duration does not contain field \"{field}\""),
                )]),
            }
        }
        // P493a — Field access em Value::Array
        Value::Array(arr) => match field {
            "len" => Ok(Value::Int(arr.len() as i64)),
            "first" => Ok(arr.first().cloned().unwrap_or(Value::None)),
            "last" => Ok(arr.last().cloned().unwrap_or(Value::None)),
            _ => Err(vec![SourceDiagnostic::error(
                span,
                format!("array does not contain field \"{field}\""),
            )]),
        },
        // P504 — Field access em Value::Args
        Value::Args(a) => match field {
            "named" => Ok(Value::Dict(a.named.clone())),
            "positional" => Ok(Value::Array(a.items.clone())),
            _ => Err(vec![SourceDiagnostic::error(
                span,
                format!("arguments does not contain field \"{field}\""),
            )]),
        },
        // P493b — Field access em Value::Func com namespace
        Value::Func(f) => match f.namespace().and_then(|ns| ns.get(field)) {
            Some(value) => Ok(value.clone()),
            None => Err(vec![SourceDiagnostic::error(
                span,
                match native_public_name(&f) {
                    Some(name) => {
                        format!("function `{name}` does not contain field `{field}`")
                    }
                    None => "cannot access fields on type function".to_string(),
                },
            )]),
        },
        // P685 — Field access em valor-tipo
        Value::Type(t) => match (t, field) {
            (Type::Float, _) => crate::compiler::stdlib::float_type_field(field)
                .ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        span,
                        format!("type float does not contain field \"{field}\""),
                    )]
                }),
            (Type::Int, _) => {
                crate::compiler::stdlib::int_type_field(field).ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        span,
                        format!("type int does not contain field \"{field}\""),
                    )]
                })
            }
            (Type::Counter, _) => crate::compiler::stdlib::counter::counter_type_field(
                field,
            )
            .ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    span,
                    format!("type counter does not contain field \"{field}\""),
                )]
            }),
            (Type::Content, _) => content_type_field(field).ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    span,
                    format!("type content does not contain field \"{field}\""),
                )]
            }),
            (Type::Str, "from-unicode") => {
                Ok(Value::Func(Func::native("str.from-unicode", native_str_from_unicode)))
            }
            (Type::Array, _) => {
                crate::compiler::stdlib::collection_type_field(Type::Array, field)
                    .ok_or_else(|| {
                        vec![SourceDiagnostic::error(
                            span,
                            format!("type array does not contain field \"{field}\""),
                        )]
                    })
            }
            (Type::Dictionary, _) => {
                crate::compiler::stdlib::collection_type_field(Type::Dictionary, field)
                    .ok_or_else(|| {
                        vec![SourceDiagnostic::error(
                            span,
                            format!("type dictionary does not contain field \"{field}\""),
                        )]
                    })
            }
            (Type::Str, _) => {
                crate::compiler::stdlib::collection_type_field(Type::Str, field)
                    .ok_or_else(|| {
                        vec![SourceDiagnostic::error(
                            span,
                            format!("type str does not contain field \"{field}\""),
                        )]
                    })
            }
            (Type::Bytes, _) => {
                crate::compiler::stdlib::collection_type_field(Type::Bytes, field)
                    .ok_or_else(|| {
                        vec![SourceDiagnostic::error(
                            span,
                            format!("type bytes does not contain field \"{field}\""),
                        )]
                    })
            }
            (Type::Arguments, _) => {
                crate::compiler::stdlib::collection_type_field(Type::Arguments, field)
                    .ok_or_else(|| {
                        vec![SourceDiagnostic::error(
                            span,
                            format!("type arguments does not contain field \"{field}\""),
                        )]
                    })
            }
            (
                Type::Direction
                | Type::Alignment
                | Type::Duration
                | Type::Length
                | Type::Selector
                | Type::State
                | Type::Location,
                _,
            ) => crate::compiler::eval::call_dispatch::p1284_type_field(t, field)
                .ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        span,
                        format!("type {} does not contain field \"{field}\"", t.name()),
                    )]
                }),
            (Type::Color, _) => crate::compiler::stdlib::color_type_field(field)
                .ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        span,
                        format!("type color does not contain field `{field}`"),
                    )]
                }),
            (Type::Gradient, _) => crate::compiler::stdlib::gradient_type_field(field)
                .ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        span,
                        format!("type gradient does not contain field `{field}`"),
                    )]
                }),
            (Type::Datetime, _) => crate::compiler::stdlib::datetime_type_field(field)
                .ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        span,
                        format!("type datetime does not contain field \"{field}\""),
                    )]
                }),
            _ => Err(vec![SourceDiagnostic::error(
                span,
                format!("type {} does not contain field \"{field}\"", t.name()),
            )]),
        },
        // P679 — Field access em Value::Module
        Value::Module(m) => m.scope().get(field).cloned().ok_or_else(|| {
            let name = m.name();
            vec![SourceDiagnostic::error(
                span,
                format!("module `{name}` does not contain `{field}`"),
            )]
        }),
        // P765a — Field access em Value::Symbol
        Value::Symbol(s) => s.modified(field).map(Value::Symbol).ok_or_else(|| {
            vec![SourceDiagnostic::error(
                span,
                format!("unknown symbol modifier '{field}'"),
            )]
        }),
        other => Err(vec![SourceDiagnostic::error(
            span,
            format!("cannot access fields on type {}", other.type_name()),
        )]),
    }
}

// ── P815 — método inexistente / dict-key-call (eval_field_callee) ───────────

/// **P815** — mirror de `element_or_type_with_name` do vanilla
/// (`typst-eval/src/call.rs:359-365`): `("element", nome do elemento)` para
/// content, `("type", nome longo do tipo)` nos restantes.
fn element_or_type_with_name(value: &Value) -> (&'static str, String) {
    match value {
        Value::Content(c) => ("element", c.elem_name().to_string()),
        Value::LocatedContent(c, _) => ("element", c.content().elem_name().to_string()),
        _ => ("type", vanilla_type_name(value).to_string()),
    }
}

/// **P815** — mirror do ramo de erro de `eval_field_callee` do vanilla
/// (`typst-eval/src/call.rs:258-345`), para um callee `target.field` chamado
/// como função depois de todos os despachos de método legítimos terem falhado.
///
/// Devolve `None` para `Symbol`/`Func`/`Type`/`Module` — os únicos tipos que
/// o vanilla deixa chamar campos directamente (`call.rs:258-263`) —, caindo
/// no caminho genérico existente. Nos restantes alvos:
///
/// - **O campo existe** (dict key, campo de content/length/args/…):
///   - dict → `cannot directly call dictionary keys as functions` + 2 hints;
///   - args → `cannot directly call named argument fields as functions` + 2 hints;
///   - outros → `` `{field}` is not a valid method for {kind} `{name}` `` + hint.
///   O primeiro hint muda se o valor guardado for função:
///   `to call the stored function, wrap the field access in parentheses:
///   `({full_text})(..)`` — senão `to access the `{field}` {key|argument|
///   field}, remove the function arguments: `{full_text}``.
/// - **O campo não existe** → `{kind} {name} has no method `{field}``
///   (ex.: `type integer has no method `foo``, `element strong has no method
///   `zzz``).
///
/// As mensagens são verbatim do vanilla (medidas por sonda em P810/P815 — a
/// mensagem é o observável, ADR-0107). Nota P810/P815: o vanilla **não** usa
/// distância de edição aqui (`call.rs:339-340`: "We don't try as hard on the
/// error here to avoid assuming the user's intent") — a hipótese do prompt
/// está refutada pela fonte e pela sonda.
pub(in crate::compiler::eval) fn field_callee_error(
    target: &Value,
    access: crate::entities::ast::expr::FieldAccess<'_>,
) -> Option<Vec<SourceDiagnostic>> {
    use crate::entities::ast::AstNode;

    if matches!(
        target,
        Value::Symbol(_) | Value::Func(_) | Value::Type(_) | Value::Module(_)
    ) {
        return None;
    }

    let field = access.field().as_str();
    let span = access.span();
    let is_dict = matches!(target, Value::Dict(_));
    let is_named = matches!(target, Value::Args(_));

    match eval_value_field_access(target.clone(), field, span) {
        Ok(callee_value) => {
            let mut err = if is_dict {
                SourceDiagnostic::error(
                    span,
                    "cannot directly call dictionary keys as functions",
                )
            } else if is_named {
                SourceDiagnostic::error(
                    span,
                    "cannot directly call named argument fields as functions",
                )
            } else {
                let (kind, name) = element_or_type_with_name(target);
                SourceDiagnostic::error(
                    span,
                    format!("`{field}` is not a valid method for {kind} `{name}`"),
                )
            };
            let full_text = access.to_untyped().clone().into_text();
            if matches!(callee_value, Value::Func(_)) {
                err = err.with_hint(format!(
                    "to call the stored function, wrap the field access in parentheses: `({full_text})(..)`"
                ));
            } else {
                let what = if is_dict {
                    "key"
                } else if is_named {
                    "argument"
                } else {
                    "field"
                };
                err = err.with_hint(format!(
                    "to access the `{field}` {what}, remove the function arguments: `{full_text}`"
                ));
            }
            if is_dict {
                err = err.with_hint(
                    "dictionary keys cannot be used with method syntax as keys could conflict with built-in method names".to_string(),
                );
            } else if is_named {
                err = err.with_hint(
                    "named arguments cannot be used with method syntax as argument names could conflict with built-in method names".to_string(),
                );
            }
            Some(vec![err])
        }
        Err(_) => {
            let (kind, name) = element_or_type_with_name(target);
            Some(vec![SourceDiagnostic::error(
                span,
                format!("{kind} {name} has no method `{field}`"),
            )])
        }
    }
}

// ── P829 — métodos de `content`: func/has/at/fields/location ───────────────

/// **P829** — estado de um campo de content para os métodos `has`/`at`/
/// `fields` (paridade vanilla `Content::has`/`at`/`fields`,
/// `foundations/content/mod.rs:510-590`): o vanilla distingue campo
/// **assente no constructor** de default resolvido pela chain — medido:
/// `heading[H].has("level")` → false, `heading(level: 2)[H].has("level")` →
/// true, markup `= H` assenta `depth` (não `level`).
enum ContentField {
    /// Assente — `has` → true, `at` devolve o valor.
    Set(Value),
    /// Declarado no elemento mas não assente — `has` → false; `at` erra
    /// `field "{f}" in {elem} is not known at this point` (verbatim vanilla).
    Unset,
    /// Não existe no elemento — `at` erra `{elem} does not have field "{f}"`.
    Undeclared,
}

/// **P829** — classifica um campo de content para `has`/`at`. Distinto de
/// `Content::get_field` (field access de show rules, que devolve valores
/// baked): aqui só conta o que foi assente no constructor, espelhando o
/// `field.has()`/`get()` do vanilla. A máscara `set_fields` de `HeadingElem`
/// (P829) é a única pista de "explicitamente assente" que o modelo cristalino
/// retém; nos restantes elementos os campos expostos são sempre assentes
/// (body/text), logo o fallback via `get_field` é exacto. `label` fica
/// scope-out: no cristalino a label é um nó irmão (`Content::Label`), não
/// metadado do elemento — divergência registada no L0.
fn content_field(c: &crate::entities::content::Content, field: &str) -> ContentField {
    use crate::entities::content::Content;
    use crate::entities::elements::heading::{
        HEADING_SET_DEPTH, HEADING_SET_LEVEL, HEADING_SET_OUTLINED,
    };
    match c {
        Content::Styled(child, styles) if equation_descendant(child).is_some() => {
            let key = match field {
                "numbering" => Some("equation.numbering"),
                "number-align" => Some("equation.number-align"),
                "supplement" => Some("equation.supplement"),
                "alt" => Some("equation.alt"),
                _ => None,
            };
            if let Some(value) = key.and_then(|key| {
                styles
                    .delta()
                    .custom
                    .iter()
                    .rev()
                    .find(|(candidate, _)| candidate.as_str() == key)
                    .map(|(_, value)| value.clone())
            }) {
                ContentField::Set(value)
            } else {
                content_field(child, field)
            }
        }
        Content::Strong(e) => match field {
            "body" => ContentField::Set(Value::Content(e.body.clone())),
            // `delta` é declarado em strong no vanilla mas nunca assente no
            // cristalino (`native_strong` não aceita named — scope-out).
            "delta" => ContentField::Unset,
            _ => ContentField::Undeclared,
        },
        Content::Emph(e) => match field {
            "body" => ContentField::Set(Value::Content(e.body.clone())),
            _ => ContentField::Undeclared,
        },
        Content::Title(t) => match field {
            "body" => ContentField::Set(Value::Content(t.body.clone())),
            _ => ContentField::Undeclared,
        },
        Content::Text(t) => match field {
            "text" => ContentField::Set(Value::Str(t.clone())),
            _ => ContentField::Undeclared,
        },
        Content::Linebreak(e) => match field {
            "justify" if e.justify_explicit => ContentField::Set(Value::Bool(e.justify)),
            "justify" => ContentField::Unset,
            _ => ContentField::Undeclared,
        },
        Content::Heading(h) => match field {
            "body" => ContentField::Set(Value::Content(h.body.clone())),
            "level" => {
                if h.set_fields & HEADING_SET_LEVEL != 0 {
                    ContentField::Set(Value::Int(h.level as i64))
                } else {
                    ContentField::Unset
                }
            }
            "depth" => {
                if h.set_fields & HEADING_SET_DEPTH != 0 {
                    ContentField::Set(Value::Int(h.level as i64))
                } else {
                    ContentField::Undeclared
                }
            }
            "outlined" => {
                if h.set_fields & HEADING_SET_OUTLINED != 0 {
                    ContentField::Set(Value::Bool(h.outlined))
                } else {
                    ContentField::Unset
                }
            }
            "bookmarked" => match h.bookmarked {
                Some(b) => ContentField::Set(Value::Bool(b)),
                None => ContentField::Unset,
            },
            _ => ContentField::Undeclared,
        },
        Content::Equation(e) => match field {
            "block" => ContentField::Set(Value::Bool(e.block)),
            "body" => ContentField::Set(Value::Content(e.body.clone())),
            _ => ContentField::Undeclared,
        },
        other => match other.get_field(field) {
            Some(v) => ContentField::Set(v),
            None => ContentField::Undeclared,
        },
    }
}

/// **P829** — campos assentes de um content, na ordem de declaração do
/// vanilla (para heading: level, depth, outlined, bookmarked, body — medido
/// em b8/b18: `(level: 2, body: [H])`, `(depth: 1, body: [H])`).
fn content_set_fields(
    c: &crate::entities::content::Content,
) -> Vec<(&'static str, Value)> {
    use crate::entities::content::Content;
    let candidates: &[&'static str] = match c {
        Content::Heading(_) => &["level", "depth", "outlined", "bookmarked", "body"],
        Content::Equation(_) => &["block", "body"],
        Content::Styled(child, _) if equation_descendant(child).is_some() => {
            &["block", "numbering", "number-align", "supplement", "alt", "body"]
        }
        Content::Text(_) => &["text"],
        Content::Linebreak(_) => &["justify"],
        _ => &["body"],
    };
    candidates
        .iter()
        .filter_map(|name| match content_field(c, name) {
            ContentField::Set(v) => Some((*name, v)),
            _ => None,
        })
        .collect()
}

fn equation_descendant(
    content: &crate::entities::content::Content,
) -> Option<&crate::entities::elements::equation::EquationElem> {
    use crate::entities::content::Content;
    match content {
        Content::Equation(e) => Some(e),
        Content::Styled(child, _) => equation_descendant(child),
        _ => None,
    }
}

/// **P829** — fallback de `func()` para variantes sem constructor nativo
/// exposto (Sequence, Styled, Label, math, Dynamic, …): o `Value::Func`
/// existe (repr = nome do elemento — paridade medida de `func()`) mas a
/// chamada não é suportada. Caso não medido no vanilla (elementos internos
/// não expostos no scope); mensagem própria, registada no L0.
fn content_func_not_callable(
    _ctx: &mut crate::compiler::eval::EvalContext,
    _args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: crate::entities::file_id::FileId,
) -> SourceResult<Value> {
    Err(vec![SourceDiagnostic::error(
        Span::detached(),
        "calling this element function is not supported".to_string(),
    )])
}

/// **P829** — `content.func()` (vanilla `Content::func`,
/// `foundations/content/mod.rs:516-519`): a função do elemento. Igualdade
/// por nome (P742) dá `strong[x].func() == strong` → true (medido b9).
fn content_elem_func(c: &crate::entities::content::Content) -> Value {
    use crate::entities::content::Content;
    let call: fn(
        &mut crate::compiler::eval::EvalContext,
        &Args,
        &dyn crate::contracts::world::World,
        crate::entities::file_id::FileId,
    ) -> SourceResult<Value> = match c {
        Content::Text(_) => crate::compiler::stdlib::native_text,
        Content::Strong(_) => crate::compiler::stdlib::native_strong,
        Content::Emph(_) => crate::compiler::stdlib::native_emph,
        Content::Heading(_) => crate::compiler::stdlib::native_heading,
        Content::Title(_) => crate::compiler::stdlib::native_title,
        Content::Raw(_) => crate::compiler::stdlib::native_raw,
        Content::Figure(_) => crate::compiler::stdlib::native_figure,
        Content::Link(_) => crate::compiler::stdlib::native_link,
        Content::SmallCaps { .. } => crate::compiler::stdlib::native_smallcaps,
        Content::Linebreak(_) => crate::compiler::stdlib::native_linebreak,
        _ => content_func_not_callable,
    };
    Value::Func(Func::native(c.elem_name(), call))
}

/// **P829** — mirror dos métodos do `#[scope]` de `Content` do vanilla
/// (`foundations/content/mod.rs:510-590`): `func()`, `has(field)`,
/// `at(field, default:?)`, `fields()`, `location()`. Mensagens de erro de
/// argumentos verbatim (medidas b13–b17): `missing argument: field`,
/// `expected string, found {tipo}`, `unexpected argument`,
/// `unexpected argument: {nome}`.
///
/// `location()` devolve sempre `none`: o cristalino não retém metadados de
/// location em `Content` (medido: content inline → none nos dois binários;
/// content de show rule/query → `location(..)` no vanilla — divergência
/// registada no L0, requer introspecção de locations, fora do proporcional).
pub(crate) fn eval_content_method(
    c: &crate::entities::content::Content,
    method: &str,
    args: Args,
    span: Span,
) -> SourceResult<Value> {
    eval_content_method_at(c, None, method, args, span)
}

pub(crate) fn eval_content_method_at(
    c: &crate::entities::content::Content,
    location: Option<crate::entities::location::Location>,
    method: &str,
    args: Args,
    span: Span,
) -> SourceResult<Value> {
    content_method_with_fields(c, location, None, method, args, span)
}

pub(crate) fn eval_introspected_content_method_at(
    c: &crate::entities::value::IntrospectedContent,
    location: crate::entities::location::Location,
    method: &str,
    args: Args,
    span: Span,
) -> SourceResult<Value> {
    content_method_with_fields(
        c.content(),
        Some(location),
        c.fields(),
        method,
        args,
        span,
    )
}

fn content_method_with_fields(
    c: &crate::entities::content::Content,
    location: Option<crate::entities::location::Location>,
    fields: Option<&IndexMap<EcoString, Value, FxBuildHasher>>,
    method: &str,
    mut args: Args,
    span: Span,
) -> SourceResult<Value> {
    match method {
        "func" => {
            finish_args(&args, span)?;
            Ok(content_elem_func(c))
        }
        "fields" => {
            finish_args(&args, span)?;
            if let Some(fields) = fields {
                return Ok(Value::Dict(fields.clone()));
            }
            let mut dict: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
            for (name, value) in content_set_fields(c) {
                dict.insert(name.into(), value);
            }
            Ok(Value::Dict(dict))
        }
        "location" => {
            finish_args(&args, span)?;
            Ok(location.map(Value::Location).unwrap_or(Value::None))
        }
        "has" | "at" => {
            let field_value = expect_positional(&mut args, span, "field")?;
            let field = match field_value {
                Value::Str(s) => s,
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        span,
                        format!("expected string, found {}", vanilla_type_name(&other)),
                    )])
                }
            };
            let default =
                if method == "at" { args.remove_named("default") } else { None };
            finish_args(&args, span)?;
            let field = field.as_str();
            let resolved = match fields {
                Some(fields) => fields
                    .get(field)
                    .cloned()
                    .map(ContentField::Set)
                    .unwrap_or(ContentField::Undeclared),
                None => content_field(c, field),
            };
            match method {
                "has" => Ok(Value::Bool(matches!(
                    resolved,
                    ContentField::Set(_)
                ))),
                _ => match resolved {
                    ContentField::Set(v) => Ok(v),
                    ContentField::Unset => match default {
                        Some(v) => Ok(v),
                        None => Err(vec![SourceDiagnostic::error(
                            span,
                            format!(
                                "field \"{field}\" in {} is not known at this point and no default was specified",
                                c.elem_name()
                            ),
                        )]),
                    },
                    ContentField::Undeclared => match default {
                        Some(v) => Ok(v),
                        None => Err(vec![SourceDiagnostic::error(
                            span,
                            format!(
                                "{} does not have field \"{field}\" and no default was specified",
                                c.elem_name()
                            ),
                        )]),
                    },
                },
            }
        }
        _ => unreachable!("eval_content_method: método desconhecido {method}"),
    }
}

fn static_content_receiver(args: &Args, name: &str) -> SourceResult<(Value, Args)> {
    let Some(first) = args.items.first() else {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("{name} requires content as its first argument"),
        )]);
    };
    let content = match first {
        Value::Content(_) | Value::LocatedContent(_, _) => first.clone(),
        _ => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("{name} requires content as its first argument"),
            )])
        }
    };
    let mut rest = args.clone();
    rest.remove_positional(0);
    Ok((content, rest))
}

pub(in crate::compiler::eval) fn content_type_field(field: &str) -> Option<Value> {
    let (name, call) = match field {
        "func" => ("content.func", native_content_func_static as _),
        "has" => ("content.has", native_content_has_static as _),
        "at" => ("content.at", native_content_at_static as _),
        "fields" => ("content.fields", native_content_fields_static as _),
        "location" => ("content.location", native_content_location_static as _),
        _ => return None,
    };
    Some(Value::Func(Func::native(name, call)))
}

fn static_content_method(args: &Args, name: &str, method: &str) -> SourceResult<Value> {
    let (content, rest) = static_content_receiver(args, name)?;
    match content {
        Value::Content(content) => {
            eval_content_method_at(&content, None, method, rest, args.span)
        }
        Value::LocatedContent(content, location) => eval_introspected_content_method_at(
            &content, location, method, rest, args.span,
        ),
        _ => unreachable!("receiver was validated as content"),
    }
}

macro_rules! static_content_native {
    ($fn_name:ident, $public_name:literal, $method:literal) => {
        fn $fn_name(
            _ctx: &mut EvalContext,
            args: &Args,
            _world: &dyn crate::contracts::world::World,
            _file: crate::entities::file_id::FileId,
        ) -> SourceResult<Value> {
            static_content_method(args, $public_name, $method)
        }
    };
}

static_content_native!(native_content_func_static, "content.func", "func");
static_content_native!(native_content_has_static, "content.has", "has");
static_content_native!(native_content_at_static, "content.at", "at");
static_content_native!(native_content_fields_static, "content.fields", "fields");
static_content_native!(native_content_location_static, "content.location", "location");

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
            ("#let result = [x].text", "text", "text"),
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
