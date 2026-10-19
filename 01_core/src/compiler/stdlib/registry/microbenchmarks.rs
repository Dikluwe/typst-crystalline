//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/registry/microbenchmarks.md
//! @prompt-hash 98aa8422
//! @layer L1
//! @updated 2026-09-26

#![cfg(test)]

#[cfg(test)]
mod benchmarks {
    use super::super::make_stdlib_with_features;
    use crate::contracts::world::SysInputs;
    use crate::entities::compiler_features::{Feature, Features};

    fn profiles() -> Vec<(String, Features)> {
        (0..8)
            .map(|mask| {
                let mut features = Features::empty();
                for (bit, feature) in
                    [Feature::Html, Feature::A11yExtras, Feature::Bundle]
                        .into_iter()
                        .enumerate()
                {
                    if mask & (1 << bit) != 0 {
                        features.enable(feature);
                    }
                }
                (
                    format!(
                        "html{}_a11y{}_bundle{}",
                        mask & 1,
                        (mask >> 1) & 1,
                        (mask >> 2) & 1
                    ),
                    features,
                )
            })
            .collect()
    }

    #[test]
    fn microbenchmark_profiles_cover_all_feature_combinations() {
        let profiles = profiles();
        assert_eq!(profiles.len(), 8);
        let mut masks = std::collections::BTreeSet::new();
        for (_, features) in profiles {
            let mask = u8::from(features.contains(Feature::Html))
                | (u8::from(features.contains(Feature::A11yExtras)) << 1)
                | (u8::from(features.contains(Feature::Bundle)) << 2);
            assert!(masks.insert(mask));
            let scope = make_stdlib_with_features(&SysInputs::default(), features);
            assert_eq!(
                scope.len(),
                if features.contains(Feature::Bundle) { 179 } else { 178 }
            );
            assert_eq!(scope.get("asset").is_some(), features.contains(Feature::Bundle));
        }
    }

    use crate::compiler::eval::EvalContext;
    use crate::contracts::world::World;
    use crate::entities::args::Args;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::scope::Scope;
    use crate::entities::source::Source;
    use crate::entities::source_result::SourceResult;
    use crate::entities::value::Value;
    use crate::entities::world_types::{Bytes, Datetime, FileResult, Font, Library};
    use criterion::{BenchmarkId, Criterion};
    use std::hint::black_box;
    use std::time::Duration;

    #[derive(Default)]
    struct NoIoWorld {
        library: Library,
        book: FontBook,
    }

    impl World for NoIoWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            FileId::from_raw(std::num::NonZeroU16::new(1).unwrap())
        }
        fn source(&self, _: FileId) -> FileResult<Source> {
            panic!("benchmark must not request a source")
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            panic!("benchmark must not request a file")
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

    type Native = fn(&mut EvalContext, &Args, &dyn World, FileId) -> SourceResult<Value>;

    fn native(scope: &Scope, name: &str) -> Native {
        let Some(Value::Func(function)) = scope.get(name) else {
            panic!("missing native {name}")
        };
        function.native_fn_addr().expect("ordinary native")
    }

    struct CallCase {
        name: &'static str,
        call: Native,
        args: Args,
        expected: Option<Value>,
    }

    fn calls(scope: &Scope) -> Vec<CallCase> {
        let Some(Value::Module(calc)) = scope.get("calc") else { panic!("missing calc") };
        let abs = native(calc.scope(), "abs");
        let upper = native(scope, "upper");
        let mut named = Args::positional(vec![Value::Str("abc".into())]);
        named.named.insert("unexpected".into(), Value::Bool(true));
        vec![
            CallCase {
                name: "calc_abs/int",
                call: abs,
                args: Args::positional(vec![Value::Int(-42)]),
                expected: Some(Value::Int(42)),
            },
            CallCase {
                name: "calc_abs/float",
                call: abs,
                args: Args::positional(vec![Value::Float(-42.5)]),
                expected: Some(Value::Float(42.5)),
            },
            CallCase {
                name: "calc_abs/overflow",
                call: abs,
                args: Args::positional(vec![Value::Int(i64::MIN)]),
                expected: None,
            },
            CallCase {
                name: "upper/ascii",
                call: upper,
                args: Args::positional(vec![Value::Str("crystalline".into())]),
                expected: Some(Value::Str("CRYSTALLINE".into())),
            },
            CallCase {
                name: "upper/unicode",
                call: upper,
                args: Args::positional(vec![Value::Str("ação straße".into())]),
                expected: Some(Value::Str("AÇÃO STRASSE".into())),
            },
            CallCase {
                name: "upper/wrong_type",
                call: upper,
                args: Args::positional(vec![Value::Bool(true)]),
                expected: None,
            },
            CallCase {
                name: "upper/missing_arg",
                call: upper,
                args: Args::positional(vec![]),
                expected: None,
            },
            CallCase {
                name: "upper/extra_arg",
                call: upper,
                args: Args::positional(vec![Value::Str("abc".into()), Value::Int(1)]),
                expected: None,
            },
            CallCase {
                name: "upper/named_arg",
                call: upper,
                args: named,
                expected: None,
            },
        ]
    }

    fn assert_calls(cases: &[CallCase], ctx: &mut EvalContext, world: &NoIoWorld) {
        for case in cases {
            let result = (case.call)(ctx, &case.args, world, world.main());
            match &case.expected {
                Some(expected) => assert_eq!(&result.unwrap(), expected, "{}", case.name),
                None => assert!(!result.expect_err(case.name).is_empty()),
            }
        }
    }

    #[test]
    fn microbenchmark_native_fixtures_have_fixed_semantics() {
        let scope = make_stdlib_with_features(&SysInputs::default(), Features::empty());
        assert_calls(&calls(&scope), &mut EvalContext::new(), &NoIoWorld::default());
    }

    #[test]
    #[ignore = "Criterion microbenchmarks: execute explicitly in release"]
    fn stdlib_microbenchmarks() {
        #[expect(
            clippy::assertions_on_constants,
            reason = "The ignored runner must reject explicit invocation in debug builds"
        )]
        {
            assert!(!cfg!(debug_assertions), "run with cargo test --release");
        }
        microbenchmark_profiles_cover_all_feature_combinations();
        let mut criterion = Criterion::default()
            .without_plots()
            .sample_size(30)
            .warm_up_time(Duration::from_millis(150))
            .measurement_time(Duration::from_millis(300));
        let inputs = SysInputs::default();
        let mut registry = criterion.benchmark_group("stdlib/registry");
        for (name, features) in profiles() {
            registry.bench_function(name, |b| {
                b.iter(|| {
                    black_box(make_stdlib_with_features(
                        black_box(&inputs),
                        black_box(features),
                    ))
                })
            });
        }
        registry.finish();

        let mut input_group = criterion.benchmark_group("stdlib/inputs");
        for count in [0, 16, 256] {
            let mut inputs = SysInputs::default();
            for index in 0..count {
                inputs.insert(format!("key{index}").into(), "value".into());
            }
            input_group.bench_with_input(
                BenchmarkId::from_parameter(count),
                &inputs,
                |b, inputs| {
                    b.iter(|| {
                        black_box(make_stdlib_with_features(
                            black_box(inputs),
                            black_box(Features::empty()),
                        ))
                    })
                },
            );
        }
        input_group.finish();

        let scope = make_stdlib_with_features(&inputs, Features::empty());
        let mut lookup = criterion.benchmark_group("stdlib/lookup");
        for name in ["upper", "not-a-binding"] {
            assert_eq!(scope.get(name).is_some(), name == "upper");
            lookup.bench_function(name, |b| {
                b.iter(|| black_box(black_box(&scope).get(black_box(name))))
            });
        }
        let Some(Value::Module(calc)) = scope.get("calc") else { panic!("missing calc") };
        assert!(calc.scope().get("abs").is_some());
        lookup.bench_function("calc_abs", |b| {
            b.iter(|| black_box(black_box(calc.scope()).get(black_box("abs"))))
        });
        lookup.finish();

        let world = NoIoWorld::default();
        let mut ctx = EvalContext::new();
        let cases = calls(&scope);
        assert_calls(&cases, &mut ctx, &world);
        let file = world.main();
        let mut call_group = criterion.benchmark_group("stdlib/native");
        for case in cases {
            call_group.bench_function(case.name, |b| {
                b.iter(|| {
                    black_box((black_box(case.call))(
                        black_box(&mut ctx),
                        black_box(&case.args),
                        black_box(&world),
                        black_box(file),
                    ))
                })
            });
        }
        call_group.finish();
        criterion.final_summary();
    }
}
