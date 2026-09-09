#[cfg(test)]
mod p1334_dispatch_tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
    use std::sync::Arc;

    use ecow::EcoString;
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
