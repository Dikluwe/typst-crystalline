//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/call_dispatch.md
//! @prompt-hash d9f9c8e9
//! @layer L1
//! @updated 2026-09-01
//!
//! Dispatch de chamadas de função: avaliação de args, intercepção de method
//! calls especiais, aplicação de nativas/plugins/elementos. Extraído de
//! `compiler/eval/closures.rs` no Passo 1012 conforme ADR-0109 (atomização —
//! forma B, free function no arquivo da unidade).

use ecow::{EcoString, EcoVec};
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

use crate::compiler::scopes::Scopes;
use crate::compiler::stdlib::{
    extract_measure_body,
    native_array_bytes,
    native_bytes,
    // P737 — counter/state chamáveis via despacho de tipos.
    native_counter,
    native_datetime,
    native_decimal,
    native_duration,
    native_float,
    native_int,
    native_label,
    native_layout,
    native_measure,
    native_outline_entry_indented,
    native_outline_entry_inner,
    native_outline_entry_page,
    native_outline_entry_prefix,
    native_path,
    native_regex,
    native_selector,
    // P1473 — `color.spot(name, fallback)` chamável via `Value::Type`.
    native_spot_colorant_construct,
    native_state,
    native_str,
    native_stroke,
    native_symbol,
    native_tiling,
    native_type,
    native_version,
    try_dispatch_collection_method,
    CollectionCallSpans,
};
use crate::entities::args::{ArgOccurrence, Args};
use crate::entities::ast::expr::{Arg, Expr, FuncCall as FuncCallNode};
use crate::entities::ast::AstNode;
use crate::entities::bytes::Bytes;
use crate::entities::engine::Engine;
use crate::entities::func::{Func, FuncRepr};
use crate::entities::plugin_func::PluginFunc;
use crate::entities::source_result::SourceDiagnostic;
use crate::entities::source_result::SourceResult;
use crate::entities::source_result::Tracepoint;
use crate::entities::span::{Span, Spanned};
use crate::entities::value::{Type, Value};

use crate::compiler::eval::operators::error_formatting::vanilla_type_name;

use super::{bindings, closures, eval_expr, rules, EvalContext, EvalTarget};

mod apply;
mod arguments;
mod call;
/// Metadados sintáticos privados e pontuais para os diagnósticos de
/// `float.is-nan`. Não atravessam `entities::Args` nem sobrevivem à chamada.
mod call_spans;
mod invoke;
mod location;
mod selector_intercepts;
mod trace_plugin;
mod type_fields;
mod value_methods;

pub use self::apply::apply_func;
pub(super) use self::arguments::{eval_args, transport_native_call_span};
pub(super) use self::call::eval_func_call;
pub(super) use self::location::eval_location_method;
pub(crate) use self::type_fields::p1284_type_field;
pub(in crate::compiler::eval) use self::value_methods::selector_before_after;
pub(crate) use self::value_methods::take_p1339_self;
use self::{apply::*, call_spans::*, trace_plugin::*, value_methods::*};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selector_interception_distinguishes_logical_methods() {
        assert!(selector_intercepts::is_selector_logical("and"));
        assert!(selector_intercepts::is_selector_logical("or"));
        assert!(!selector_intercepts::is_selector_logical("within"));
    }

    #[test]
    fn atomized_dispatch_preserves_type_fields_and_with_order() {
        let Some(Value::Func(func)) = type_fields::p1284_type_field(Type::Angle, "deg")
        else {
            panic!("angle.deg must be callable")
        };
        assert_eq!(func.name(), Some("deg"));
        let merged = trace_plugin::merge_with_args(
            &Args::positional(vec![Value::Int(1)]),
            Args::positional(vec![Value::Int(2)]),
        );
        assert_eq!(merged.items, vec![Value::Int(1), Value::Int(2)]);
    }
    use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
    use std::sync::Arc;

    use ecow::{EcoString, EcoVec};
    use indexmap::IndexMap;
    use rustc_hash::FxBuildHasher;

    use crate::contracts::plugin_host::{PluginError, PluginHost, PluginModuleId};
    use crate::entities::bytes::Bytes;
    use crate::entities::file_id::FileId;

    fn p1293_span(start: usize) -> Span {
        use std::num::NonZeroU16;
        Span::from_range(FileId::from_raw(NonZeroU16::new(9).unwrap()), start..start + 1)
    }

    #[test]
    fn p1339_discovery_angle_and_function_methods() {
        for (ty, field) in [
            (Type::Angle, "deg"),
            (Type::Angle, "rad"),
            (Type::Function, "with"),
            (Type::Function, "where"),
        ] {
            let Some(Value::Func(func)) = p1284_type_field(ty, field) else {
                panic!("missing {}.{field}", ty.name());
            };
            assert_eq!(func.name(), Some(field));
        }
        assert!(p1284_type_field(Type::Angle, "signum").is_none());
        assert!(p1284_type_field(Type::Function, "before").is_none());
    }

    #[test]
    fn p1321_csv_transport_uses_identity_and_preserves_occurrences() {
        use crate::entities::args::ArgOccurrence;
        let native = Func::native("alias-not-csv", crate::compiler::stdlib::native_csv);
        let preargs = Args::from_occurrences(
            p1293_span(70),
            vec![ArgOccurrence {
                name: None,
                value: Value::Int(7),
                span: p1293_span(71),
                value_span: p1293_span(72),
            }],
        );
        let with = native.clone().with(preargs).with(Args::positional(vec![]));
        for func in [native, with] {
            for populated in [false, true] {
                let occurrences = if populated {
                    vec![ArgOccurrence {
                        name: Some("delimiter".into()),
                        value: Value::Int(3),
                        span: p1293_span(20),
                        value_span: p1293_span(21),
                    }]
                } else {
                    vec![]
                };
                let mut args = Args::from_occurrences(p1293_span(10), occurrences);
                transport_native_call_span(&func, &mut args, p1293_span(1));
                assert_eq!(args.span, p1293_span(1));
                assert!(args.items.is_empty());
                assert_eq!(args.named.len(), usize::from(populated));
                let after = args.occurrences.as_ref().unwrap();
                assert_eq!(after.len(), usize::from(populated));
                if populated {
                    assert_eq!(after[0].name.as_deref(), Some("delimiter"));
                    assert!(matches!(after[0].value, Value::Int(3)));
                    assert_eq!(after[0].span, p1293_span(20));
                    assert_eq!(after[0].value_span, p1293_span(21));
                }
            }
            if let FuncRepr::With(outer) = func.repr() {
                let FuncRepr::With(inner) = outer.0.repr() else { panic!() };
                assert_eq!(inner.1.span, p1293_span(70));
                let occurrence = &inner.1.occurrences.as_ref().unwrap()[0];
                assert!(matches!(occurrence.value, Value::Int(7)));
                assert_eq!(occurrence.span, p1293_span(71));
                assert_eq!(occurrence.value_span, p1293_span(72));
            }
        }
    }

    #[test]
    fn p1321_csv_transport_preserves_other_identities() {
        use crate::compiler::stdlib::{
            native_json, native_json_encode, native_panic, native_read,
            native_toml_encode, native_yaml_encode,
        };
        let fake = Func::native("csv", |_ctx, _args, _world, _file| Ok(Value::None));
        for func in [
            fake.clone(),
            fake.with(Args::positional(vec![])),
            Func::native("csv", native_read),
            Func::native("csv", native_json),
        ] {
            let mut args = Args::positional(vec![]);
            args.span = p1293_span(10);
            transport_native_call_span(&func, &mut args, p1293_span(1));
            assert_eq!(args.span, p1293_span(10));
            assert!(args.occurrences.is_none());
        }
        for func in [
            Func::native("alias", native_json_encode),
            Func::native("alias", native_toml_encode),
            Func::native("alias", native_yaml_encode),
            Func::native("alias", native_panic),
        ] {
            let mut args = Args::positional(vec![]);
            args.span = p1293_span(10);
            transport_native_call_span(&func, &mut args, p1293_span(1));
            assert_eq!(args.span, p1293_span(1));
            assert!(args.occurrences.is_none());
        }
    }

    #[test]
    fn p1293_c_identidades_html_e_ancoras_sao_estritas() {
        let module = crate::compiler::stdlib::make_html_module();
        let Value::Func(module) = &module else { panic!() };
        for (name, expected_void) in [
            ("button", false),
            ("col", true),
            ("iframe", false),
            ("select", false),
            ("template", false),
            ("video", false),
            ("wbr", true),
        ] {
            let Value::Func(func) = module.namespace().unwrap().get(name).unwrap() else {
                panic!()
            };
            assert!(
                matches!(
                    P1293HtmlNative::from_func(func),
                    Some(P1293HtmlNative::Void) if expected_void
                ) || matches!(
                    P1293HtmlNative::from_func(func),
                    Some(P1293HtmlNative::Normal) if !expected_void
                )
            );
        }
        let fake = Func::native("video", |_ctx, _args, _world, _file| Ok(Value::None));
        assert!(P1293HtmlNative::from_func(&fake).is_none());

        let mut named = IndexMap::default();
        named.insert(
            EcoString::from("width"),
            P1293NamedSpan { full: p1293_span(20), value: p1293_span(21) },
        );
        let spans = P1293HtmlCallSpans {
            kind: P1293HtmlNative::Normal,
            positional: vec![],
            named,
            has_spread: false,
        };
        let mut args = Args::positional(vec![]);
        args.named.insert("width".into(), Value::Int(-1));
        assert_eq!(spans.anchor(&args, "number must be at least zero"), p1293_span(21));
        assert_eq!(spans.anchor(&args, "unexpected argument: width"), p1293_span(20));
    }

    /// Host de teste (sem WASM) para o braço `call_plugin`. Conta chamadas e
    /// devolve `b"OK"`. Nomes de export únicos por teste evitam colisão no
    /// cache global do comemo (`PluginFunc::call` é memoizado).
    struct StubHost {
        next: AtomicU64,
        calls: AtomicUsize,
    }
    impl StubHost {
        fn new() -> Self {
            Self {
                next: AtomicU64::new(1),
                calls: AtomicUsize::new(0),
            }
        }
    }
    impl PluginHost for StubHost {
        fn load(&self, _bytes: &[u8]) -> Result<PluginModuleId, PluginError> {
            Ok(PluginModuleId(self.next.fetch_add(1, Ordering::Relaxed)))
        }
        fn exports(
            &self,
            _module: PluginModuleId,
        ) -> Result<Vec<EcoString>, PluginError> {
            Ok(Vec::new())
        }
        fn call(
            &self,
            _module: PluginModuleId,
            _func_name: &str,
            _args: &[Bytes],
        ) -> Result<Bytes, PluginError> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            Ok(Bytes::new(b"OK".to_vec()))
        }
        fn transition(
            &self,
            _module: PluginModuleId,
            _func_name: &str,
            _args: &[Bytes],
        ) -> Result<PluginModuleId, PluginError> {
            Ok(PluginModuleId(self.next.fetch_add(1, Ordering::Relaxed)))
        }
    }

    fn make_pf(name: &str) -> PluginFunc {
        let host: Arc<StubHost> = Arc::new(StubHost::new());
        let id = host.load(&[]).unwrap();
        PluginFunc {
            host: host as Arc<dyn PluginHost>,
            module: id,
            name: EcoString::from(name),
        }
    }

    #[test]
    fn call_plugin_devolve_bytes() {
        let pf = make_pf("cp_devolve");
        let args = Args::positional(vec![Value::Bytes(Bytes::new(b"x".to_vec()))]);
        let v = call_plugin(&pf, &args).unwrap();
        assert!(matches!(v, Value::Bytes(b) if b.as_slice() == b"OK"));
    }

    #[test]
    fn call_plugin_arg_nao_bytes_erro() {
        let pf = make_pf("cp_argbad");
        let args = Args::positional(vec![Value::Int(1)]);
        let e = call_plugin(&pf, &args).unwrap_err();
        // P819 — verbatim do vanilla (medido t6: `expected bytes, found integer`).
        assert_eq!(
            e[0].message.as_str(),
            "expected bytes, found integer",
            "msg: {}",
            e[0].message
        );
    }

    #[test]
    fn call_plugin_named_arg_erro() {
        let pf = make_pf("cp_named");
        let mut named = IndexMap::with_hasher(FxBuildHasher);
        named.insert(EcoString::from("x"), Value::None);
        let args = Args::from_parts(
            vec![Value::Bytes(Bytes::default())],
            named,
            Span::detached(),
        );
        let e = call_plugin(&pf, &args).unwrap_err();
        assert!(
            e[0].message.contains("argumento nomeado inesperado"),
            "msg: {}",
            e[0].message,
        );
    }
}

#[cfg(test)]
mod p1334_dispatch_tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
    use std::sync::Arc;

    use ecow::{EcoString, EcoVec};
    use indexmap::IndexMap;
    use rustc_hash::FxBuildHasher;

    use crate::contracts::plugin_host::{PluginError, PluginHost, PluginModuleId};
    use crate::entities::bytes::Bytes;
    use crate::entities::file_id::FileId;

    fn p1293_span(start: usize) -> Span {
        use std::num::NonZeroU16;
        Span::from_range(FileId::from_raw(NonZeroU16::new(9).unwrap()), start..start + 1)
    }

    #[test]
    fn p1334_abs_transport_uses_identity_and_preserves_occurrences() {
        use crate::entities::args::ArgOccurrence;
        let Value::Module(module) = crate::compiler::stdlib::make_calc_module() else {
            panic!()
        };
        let Value::Func(native) = module.scope().get("abs").unwrap().clone() else {
            panic!()
        };
        let preargs = Args::from_occurrences(
            p1293_span(70),
            vec![ArgOccurrence {
                name: None,
                value: Value::Int(7),
                span: p1293_span(71),
                value_span: p1293_span(72),
            }],
        );
        let with = native.clone().with(preargs).with(Args::positional(vec![]));
        for func in [native, with] {
            for populated in [false, true] {
                let occurrences = if populated {
                    vec![ArgOccurrence {
                        name: Some("delimiter".into()),
                        value: Value::Int(3),
                        span: p1293_span(20),
                        value_span: p1293_span(21),
                    }]
                } else {
                    vec![]
                };
                let mut args = Args::from_occurrences(p1293_span(10), occurrences);
                transport_native_call_span(&func, &mut args, p1293_span(1));
                assert_eq!(args.span, p1293_span(1));
                assert!(args.items.is_empty());
                assert_eq!(args.named.len(), usize::from(populated));
                let after = args.occurrences.as_ref().unwrap();
                assert_eq!(after.len(), usize::from(populated));
                if populated {
                    assert_eq!(after[0].name.as_deref(), Some("delimiter"));
                    assert!(matches!(after[0].value, Value::Int(3)));
                    assert_eq!(after[0].span, p1293_span(20));
                    assert_eq!(after[0].value_span, p1293_span(21));
                }
            }
            if let FuncRepr::With(outer) = func.repr() {
                let FuncRepr::With(inner) = outer.0.repr() else { panic!() };
                assert_eq!(inner.1.span, p1293_span(70));
                let occurrence = &inner.1.occurrences.as_ref().unwrap()[0];
                assert!(matches!(occurrence.value, Value::Int(7)));
                assert_eq!(occurrence.span, p1293_span(71));
                assert_eq!(occurrence.value_span, p1293_span(72));
            }
        }
    }

    #[test]
    fn p1334_abs_transport_preserves_other_identities() {
        use crate::compiler::stdlib::{
            native_csv, native_json, native_json_encode, native_panic, native_read,
            native_toml_encode, native_yaml_encode,
        };
        let fake = Func::native("abs", |_ctx, _args, _world, _file| Ok(Value::None));
        for func in [
            fake.clone(),
            fake.with(Args::positional(vec![])),
            Func::native("abs", native_read),
            Func::native("abs", native_json),
        ] {
            let mut args = Args::positional(vec![]);
            args.span = p1293_span(10);
            transport_native_call_span(&func, &mut args, p1293_span(1));
            assert_eq!(args.span, p1293_span(10));
            assert!(args.occurrences.is_none());
        }
        for func in [
            Func::native("alias", native_csv),
            Func::native("alias", native_json_encode),
            Func::native("alias", native_toml_encode),
            Func::native("alias", native_yaml_encode),
            Func::native("alias", native_panic),
        ] {
            let mut args = Args::positional(vec![]);
            args.span = p1293_span(10);
            transport_native_call_span(&func, &mut args, p1293_span(1));
            assert_eq!(args.span, p1293_span(1));
            assert!(args.occurrences.is_none());
        }
    }
}

#[cfg(test)]
mod p1492_location_html_external_tests {
    use super::*;
    use crate::compiler::eval::EvalTarget;
    use crate::entities::location::Location;

    #[test]
    fn html_externo_nao_fabrica_pagina_ou_posicao() {
        let mut ctx = EvalContext::new();
        ctx.target = EvalTarget::Html;
        let loc = Location::from_raw(1492);

        assert!(matches!(
            eval_location_method(loc, "page", &mut ctx, Span::detached()),
            Ok(Value::None)
        ));
        assert!(matches!(
            eval_location_method(loc, "position", &mut ctx, Span::detached()),
            Ok(Value::None)
        ));
    }

    #[test]
    fn paged_e_bundle_preservam_defaults_sem_posicao_selada() {
        let loc = Location::from_raw(1492);
        for target in [EvalTarget::Paged, EvalTarget::Bundle] {
            let mut ctx = EvalContext::new();
            ctx.target = target;

            assert!(matches!(
                eval_location_method(loc, "page", &mut ctx, Span::detached()),
                Ok(Value::Int(1))
            ));
            let Ok(Value::Dict(position)) =
                eval_location_method(loc, "position", &mut ctx, Span::detached())
            else {
                panic!("target {target:?} deve preservar a posição paginada default");
            };
            assert!(matches!(position.get("page"), Some(Value::Int(1))));
        }
    }
}

#[cfg(test)]
mod p1493_location_html_frame_tests {
    use super::*;
    use crate::compiler::eval::EvalTarget;
    use crate::entities::layout_types::{Point, Pt};
    use crate::entities::location::Location;
    use crate::entities::position::HtmlPosition;
    use std::collections::HashMap;

    #[test]
    fn html_frame_projeta_x_y_sem_pagina_e_regista_demanda() {
        let loc = Location::from_raw(1493);
        let mut ctx = EvalContext::new();
        ctx.target = EvalTarget::Html;
        ctx.introspector.inject_html_positions(HashMap::from([(
            loc,
            HtmlPosition { point: Point { x: Pt(30.0), y: Pt(20.0) } },
        )]));

        let Ok(Value::Dict(position)) =
            eval_location_method(loc, "position", &mut ctx, Span::detached())
        else {
            panic!("posição local HTML deve ser um dict");
        };
        assert_eq!(position.len(), 2);
        assert!(position.get("page").is_none());
        assert!(matches!(
            position.get("x"),
            Some(Value::Length(length)) if length.resolve_pt(0.0) == 30.0
        ));
        assert!(matches!(
            position.get("y"),
            Some(Value::Length(length)) if length.resolve_pt(0.0) == 20.0
        ));
        assert!(ctx.has_location_position_reads());
        assert!(matches!(
            eval_location_method(loc, "page", &mut ctx, Span::detached()),
            Ok(Value::None)
        ));
    }

    #[test]
    fn leitura_apenas_de_page_nao_seleciona_posicao_html() {
        let loc = Location::from_raw(1493);
        let mut ctx = EvalContext::new();
        ctx.target = EvalTarget::Html;
        assert!(matches!(
            eval_location_method(loc, "page", &mut ctx, Span::detached()),
            Ok(Value::None)
        ));
        assert!(!ctx.has_location_position_reads());
    }
}
