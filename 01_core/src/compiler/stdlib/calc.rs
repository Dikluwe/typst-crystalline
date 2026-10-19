//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/calc.md
//! @prompt-hash e54decda
//! @layer L1
//! @updated 2026-05-20
//!
//! Módulo `calc` — operações matemáticas escalares.
//! P27: base aritmética (abs, pow, sqrt, floor, ceil, round, min, max, clamp).
//! P96.5: extraído de `stdlib.rs` conforme ADR-0037.
//! P283: trig/hiperbólicas/log/exp + constantes (pi, tau, e, inf).
//! P306: aritmética inteira, combinatória, norma, raiz (15 funções).
//! P308: função erro de Gauss (`erf`) — paridade calc 41/41 = 100%.
//! P817: paridade fina (achado #4 de P810) — `asin/acos/atan/atan2` →
//! `Angle` + `sin/cos/tan` aceitam `Angle`; `quo` floored; `pow` expoente
//! int negativo → float + guards vanilla; `decimal` em abs/pow/floor/ceil/
//! trunc/fract/round + erro dedicado decimal×float; tipos de `round`/
//! `fract`; `log` com despacho de base; `root(radicand, index)` ordem
//! vanilla. Decisão E: `log10/deg/rad` mantidos como extensão cristalina.

use crate::entities::file_id::FileId;
use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

use super::{err, expect_no_named};

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::func::Func;
use crate::entities::layout_types::{Abs, Angle, Length, Ratio};
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

mod abs;
mod combinatorics;
mod erf;
mod extrema;
mod hyperbolic;
mod integer;
mod logarithms;
mod norm;
mod numeric;
mod power;
mod round;
mod trig;

pub(crate) use self::abs::calc_abs;
pub(crate) use self::combinatorics::{calc_binom, calc_fact, calc_perm};
pub(crate) use self::erf::calc_erf;
pub(crate) use self::extrema::{calc_clamp, calc_max, calc_min};
pub(crate) use self::hyperbolic::{
    calc_acosh, calc_asinh, calc_atanh, calc_cosh, calc_sinh, calc_tanh,
};
pub(crate) use self::integer::{
    calc_div_euclid, calc_even, calc_gcd, calc_lcm, calc_odd, calc_quo, calc_rem,
    calc_rem_euclid,
};
pub(crate) use self::logarithms::{
    calc_deg, calc_exp, calc_ln, calc_log, calc_log10, calc_rad,
};
pub(crate) use self::norm::calc_norm;
use self::numeric::{coerce_to_f64, guard_float};
pub(crate) use self::power::{calc_pow, calc_root, calc_sqrt};
pub(crate) use self::round::{calc_ceil, calc_floor, calc_fract, calc_round, calc_trunc};
pub(crate) use self::trig::{
    calc_acos, calc_asin, calc_atan, calc_atan2, calc_cos, calc_sin, calc_tan,
};
use self::trig::{trig_op, unary_f64};

// ── Módulo calc (Passo 27) ───────────────────────────────────────────────────

/// Constrói o módulo `calc` como `Value::Module` com 41 funções + 4 constantes.
///
/// **P731** — passou de `Value::Dict` a `Value::Module` (paridade vanilla —
/// medido: `type(calc)` → `module`). A representação antiga bloqueava
/// `#import calc: min, max` (cetz `aabb.typ:18`). Semântica de acesso
/// (`calc.abs`) mantém-se via `eval_field_access` sobre Module (P679).
///
/// P283 estendeu o módulo com trig/hiperbólicas/log/exp + constantes
/// (`pi`, `tau`, `e`, `inf`); decisão libm vs f64 e tratamento de domínio
/// documentados em `diagnostico-calc-passo-283.md`.
///
/// P306 adicionou 15 funções de aritmética inteira, combinatória, norma e
/// raiz (`trunc`, `fract`, `even`, `odd`, `rem`, `rem-euclid`, `div-euclid`,
/// `quo`, `gcd`, `lcm`, `fact`, `perm`, `binom`, `norm`, `root`).
///
/// P308 fecha a paridade `calc` 41/41 = 100% com `erf` (função erro de
/// Gauss). Vanilla delega a `libm::erf`; cristalino usa Caminho A
/// (Abramowitz & Stegun 7.1.26, polinomial puro f64; erro máximo 1.5e-7
/// — ADR-0054 graded). Diagnóstico inline em `stdlib.md` §"Função erro".
pub fn make_calc_module() -> Value {
    let mut dict: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
    dict.insert("abs".into(), Value::Func(Func::native("abs", calc_abs)));
    dict.insert("pow".into(), Value::Func(Func::native("calc.pow", calc_pow)));
    dict.insert("sqrt".into(), Value::Func(Func::native("calc.sqrt", calc_sqrt)));
    dict.insert("floor".into(), Value::Func(Func::native("calc.floor", calc_floor)));
    dict.insert("ceil".into(), Value::Func(Func::native("calc.ceil", calc_ceil)));
    dict.insert("round".into(), Value::Func(Func::native("calc.round", calc_round)));
    dict.insert("min".into(), Value::Func(Func::native("calc.min", calc_min)));
    dict.insert("max".into(), Value::Func(Func::native("calc.max", calc_max)));
    dict.insert("clamp".into(), Value::Func(Func::native("calc.clamp", calc_clamp)));
    // P283 — trig; P817-A — `sin/cos/tan` aceitam `Angle` (AngleLike vanilla)
    // e `asin/acos/atan/atan2` devolvem `Angle`.
    dict.insert("sin".into(), Value::Func(Func::native("calc.sin", calc_sin)));
    dict.insert("cos".into(), Value::Func(Func::native("calc.cos", calc_cos)));
    dict.insert("tan".into(), Value::Func(Func::native("calc.tan", calc_tan)));
    dict.insert("asin".into(), Value::Func(Func::native("calc.asin", calc_asin)));
    dict.insert("acos".into(), Value::Func(Func::native("calc.acos", calc_acos)));
    dict.insert("atan".into(), Value::Func(Func::native("calc.atan", calc_atan)));
    dict.insert("atan2".into(), Value::Func(Func::native("calc.atan2", calc_atan2)));
    // P283 — hiperbólicas.
    dict.insert("sinh".into(), Value::Func(Func::native("calc.sinh", calc_sinh)));
    dict.insert("cosh".into(), Value::Func(Func::native("calc.cosh", calc_cosh)));
    dict.insert("tanh".into(), Value::Func(Func::native("calc.tanh", calc_tanh)));
    dict.insert("asinh".into(), Value::Func(Func::native("calc.asinh", calc_asinh)));
    dict.insert("acosh".into(), Value::Func(Func::native("calc.acosh", calc_acosh)));
    dict.insert("atanh".into(), Value::Func(Func::native("calc.atanh", calc_atanh)));
    // P283 — exponencial / logaritmos.
    dict.insert("exp".into(), Value::Func(Func::native("calc.exp", calc_exp)));
    dict.insert("ln".into(), Value::Func(Func::native("calc.ln", calc_ln)));
    dict.insert("log".into(), Value::Func(Func::native("calc.log", calc_log)));
    dict.insert("log10".into(), Value::Func(Func::native("calc.log10", calc_log10)));
    // P501 — conversões deg/rad (extensão cristalina — P817-E: o vanilla
    // não expõe `calc.deg`/`calc.rad`/`calc.log10`; decisão: manter).
    dict.insert("deg".into(), Value::Func(Func::native("calc.deg", calc_deg)));
    dict.insert("rad".into(), Value::Func(Func::native("calc.rad", calc_rad)));
    // P306 — aritmética inteira, divisão e partes.
    dict.insert("trunc".into(), Value::Func(Func::native("calc.trunc", calc_trunc)));
    dict.insert("fract".into(), Value::Func(Func::native("calc.fract", calc_fract)));
    dict.insert("rem".into(), Value::Func(Func::native("calc.rem", calc_rem)));
    dict.insert(
        "rem-euclid".into(),
        Value::Func(Func::native("calc.rem-euclid", calc_rem_euclid)),
    );
    dict.insert(
        "div-euclid".into(),
        Value::Func(Func::native("calc.div-euclid", calc_div_euclid)),
    );
    dict.insert("quo".into(), Value::Func(Func::native("calc.quo", calc_quo)));
    // P306 — predicados inteiros.
    dict.insert("even".into(), Value::Func(Func::native("calc.even", calc_even)));
    dict.insert("odd".into(), Value::Func(Func::native("calc.odd", calc_odd)));
    // P306 — teoria dos números.
    dict.insert("gcd".into(), Value::Func(Func::native("calc.gcd", calc_gcd)));
    dict.insert("lcm".into(), Value::Func(Func::native("calc.lcm", calc_lcm)));
    // P306 — combinatória.
    dict.insert("fact".into(), Value::Func(Func::native("calc.fact", calc_fact)));
    dict.insert("perm".into(), Value::Func(Func::native("calc.perm", calc_perm)));
    dict.insert("binom".into(), Value::Func(Func::native("calc.binom", calc_binom)));
    // P306 — norma vectorial e raiz n-ésima.
    dict.insert("norm".into(), Value::Func(Func::native("calc.norm", calc_norm)));
    dict.insert("root".into(), Value::Func(Func::native("calc.root", calc_root)));
    // P308 — função erro de Gauss (paridade calc 41/41).
    dict.insert("erf".into(), Value::Func(Func::native("calc.erf", calc_erf)));
    // P283 — constantes ergonómicas (paridade vanilla).
    dict.insert("pi".into(), Value::Float(std::f64::consts::PI));
    dict.insert("tau".into(), Value::Float(std::f64::consts::TAU));
    dict.insert("e".into(), Value::Float(std::f64::consts::E));
    dict.insert("inf".into(), Value::Float(f64::INFINITY));
    // P731 — `Value::Module` (paridade vanilla), não `Value::Dict`.
    let mut scope = crate::entities::scope::Scope::new();
    for (name, value) in dict {
        scope.define(name.as_str(), value);
    }
    Value::Module(crate::entities::module::Module::new("calc", scope))
}

#[cfg(test)]
mod calc_test_support {
    use super::*;
    use crate::contracts::world::World;
    use crate::entities::font_book::FontBook;
    use crate::entities::source::Source;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library,
    };
    use std::num::NonZeroU16;

    #[derive(Default)]
    struct TestWorld {
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
            FileId::from_raw(NonZeroU16::new(1).unwrap())
        }
        fn source(&self, _: FileId) -> FileResult<Source> {
            Err(FileError::NotFound)
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

    pub(super) fn call(
        native: fn(&mut EvalContext, &Args, &dyn World, FileId) -> SourceResult<Value>,
        values: Vec<Value>,
    ) -> SourceResult<Value> {
        let world = TestWorld::default();
        let mut ctx = EvalContext::new();
        let args = Args::positional(values);
        native(&mut ctx, &args, &world, world.main())
    }
}

#[cfg(test)]
mod p1328_tests {
    use crate::compiler::eval::{
        eval_expression_with_features, eval_with_full_error_target_and_features,
        EvalContext, EvalTarget,
    };
    use crate::contracts::world::World;
    use crate::entities::args::{ArgOccurrence, Args};
    use crate::entities::compiler_features::{Feature, Features};
    use crate::entities::content::Content;
    use crate::entities::decimal::Decimal;
    use crate::entities::element_registry::ElementRegistry;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::layout_types::{Angle, Length, Ratio};
    use crate::entities::locator::Locator;
    use crate::entities::module::Module;
    use crate::entities::sink::Sink;
    use crate::entities::source::Source;
    use crate::entities::source_result::{
        Severity, SourceDiagnostic, SourceResult, Tracepoint,
    };
    use crate::entities::span::Span;
    use crate::entities::value::{IntrospectedContent, Value};
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library, Route, Routines, Traced,
    };
    use comemo::Track;

    const CONTENT_ERROR: &str = "expected integer, float, length, angle, ratio, fraction, or decimal, found content";

    struct TestWorld {
        source: Source,
        library: Library,
        book: FontBook,
    }
    impl TestWorld {
        fn new(text: &str) -> Self {
            Self {
                source: Source::detached(text),
                library: Library::new(),
                book: FontBook::new(),
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
        let world = TestWorld::new(text);
        let mut sink = Sink::new();
        let result = eval_with_full_error_target_and_features(
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
        );
        (world.source.clone(), result, sink.into_diagnostics())
    }
    fn bare_diagnostic(errors: &[SourceDiagnostic], message: &str, span: Span) {
        assert_eq!(errors.len(), 1, "{errors:?}");
        let d = &errors[0];
        assert_eq!(d.message, message);
        assert_eq!(d.severity, Severity::Error);
        assert!(d.hints.is_empty(), "{d:?}");
        assert!(d.trace.is_empty(), "{d:?}");
        assert_eq!(d.span, span);
    }
    fn native(args: &Args, features: Features) -> SourceResult<Value> {
        let world = TestWorld::new("");
        let mut ctx = EvalContext::new();
        ctx.features = features;
        super::calc_abs(&mut ctx, args, &world, world.main())
    }
    fn content_values() -> Vec<Value> {
        let c = Content::strong(Content::text("-1"));
        vec![
            Value::Content(c.clone()),
            Value::LocatedContent(
                IntrospectedContent::new(c, None),
                Locator::new().next(),
            ),
        ]
    }
    #[test]
    fn p1328_native_content_both_representations_value_span_only() {
        let source = Source::detached("aggregated occurrence value other");
        let aggregate = Span::from_range(source.id(), 0..32);
        let occurrence = Span::from_range(source.id(), 11..21);
        let origin = Span::from_range(source.id(), 22..27);
        for features in profiles() {
            for value in content_values() {
                assert_eq!(value.type_name(), "content");
                let args = Args::from_occurrences(
                    aggregate,
                    vec![ArgOccurrence {
                        name: None,
                        value,
                        span: occurrence,
                        value_span: origin,
                    }],
                );
                bare_diagnostic(
                    &native(&args, features).unwrap_err(),
                    CONTENT_ERROR,
                    origin,
                );
            }
        }
    }
    #[test]
    fn p1328_native_missing_or_detached_origin_is_not_fabricated() {
        let source = Source::detached("aggregate occurrence");
        let aggregate = Span::from_range(source.id(), 0..20);
        let occurrence = Span::from_range(source.id(), 10..20);
        for features in profiles() {
            for value in content_values() {
                let args =
                    Args::from_parts(vec![value.clone()], Default::default(), aggregate);
                bare_diagnostic(
                    &native(&args, features).unwrap_err(),
                    CONTENT_ERROR,
                    Span::detached(),
                );
                let args = Args::from_occurrences(
                    aggregate,
                    vec![ArgOccurrence {
                        name: None,
                        value,
                        span: occurrence,
                        value_span: Span::detached(),
                    }],
                );
                bare_diagnostic(
                    &native(&args, features).unwrap_err(),
                    CONTENT_ERROR,
                    Span::detached(),
                );
            }
        }
    }
    #[test]
    fn p1328_native_guards_before_content_and_numeric_controls() {
        for features in profiles() {
            let value = Value::Content(Content::empty());
            let mut args = Args::positional(vec![value.clone(), value.clone()]);
            args.named.insert("unexpected".into(), Value::Int(1));
            bare_diagnostic(
                &native(&args, features).unwrap_err(),
                CONTENT_ERROR,
                Span::detached(),
            );
            for count in [0, 2] {
                let args = Args::positional(vec![value.clone(); count]);
                bare_diagnostic(
                    &native(&args, features).unwrap_err(),
                    &if count == 0 {
                        String::from("missing argument: value")
                    } else {
                        CONTENT_ERROR.into()
                    },
                    Span::detached(),
                );
            }
            bare_diagnostic(
                &native(&Args::positional(vec![Value::Int(i64::MIN)]), features)
                    .unwrap_err(),
                "the result is too large",
                Span::detached(),
            );
            for (input, expected) in [
                (Value::Int(-19), Value::Int(19)),
                (Value::Float(-1.25), Value::Float(1.25)),
                (
                    Value::Decimal(Decimal::new(-125, 2)),
                    Value::Decimal(Decimal::new(125, 2)),
                ),
            ] {
                assert_eq!(
                    native(&Args::positional(vec![input]), features).unwrap(),
                    expected
                );
            }
        }
    }
    fn public_error(text: &str, anchor: &str, call_trace: Option<&str>) {
        for features in profiles() {
            let (source, result, warnings) = full(text, features);
            assert!(warnings.is_empty(), "{text}: {warnings:?}");
            let errors = result.unwrap_err();
            assert_eq!(errors.len(), 1, "{text}: {errors:?}");
            let d = &errors[0];
            assert_eq!(d.message, CONTENT_ERROR, "{text}");
            assert_eq!(d.severity, Severity::Error);
            assert!(d.hints.is_empty(), "{text}: {d:?}");
            let start = text.rfind(anchor).unwrap();
            assert_eq!(
                source.span_byte_range(d.span),
                Some(start..start + anchor.len()),
                "{text}: {d:?}"
            );
            assert_eq!(d.trace.len(), usize::from(call_trace.is_some()), "{text}: {d:?}");
            if let Some(call) = call_trace {
                let start = text.rfind(call).unwrap();
                assert_eq!(
                    source.span_byte_range(d.trace[0].span),
                    Some(start..start + call.len())
                );
                // Dispatcher name is a measured pre-existing debt; native adds no trace.
                assert_eq!(d.trace[0].v, Tracepoint::Call(Some("abs".into())));
            }
        }
    }
    #[test]
    fn p1328_public_math_markup_alias_origins_utf8_all_profiles() {
        for (text, anchor) in [
            ("#calc.abs([])", "[]"),
            ("#calc.abs([*bold*])", "[*bold*]"),
            ("$std.calc.abs(-1)$", "-1"),
            ("#let ab = calc.abs\n$ab(-1)$", "-1"),
            ("#let ab = calc.abs\n#ab([x])", "[x]"),
            ("#let ab = calc.abs.with()\n#ab([x])", "[x]"),
            ("#let x = [x]\n#let y = [x]\n#calc.abs(y)", "y"),
            ("#let café = [á]\n#calc.abs(\n café\n)", "café"),
            ("texto á\n#calc.abs([β])", "[β]"),
            ("#calc.abs(..([x],))", "..([x],)"),
        ] {
            public_error(text, anchor, None);
        }
    }
    #[test]
    fn p1328_public_with_and_argument_spread_keep_prebound_origin() {
        for (text, anchor, call) in [
            ("#let ab = calc.abs.with([x])\n#ab()", "[x]", "ab()"),
            ("#let ab = calc.abs.with([x]).with()\n#ab()", "[x]", "ab()"),
            ("#let aa = arguments([x])\n#calc.abs(..aa)", "[x]", "calc.abs(..aa)"),
        ] {
            public_error(text, anchor, Some(call));
        }
    }
    #[test]
    fn p1328_public_warning_survives_error_all_profiles() {
        let text = "#import std\n#calc.abs([x])";
        for features in profiles() {
            let (source, result, warnings) = full(text, features);
            assert_eq!(warnings.len(), 1, "{warnings:?}");
            let w = &warnings[0];
            assert_eq!(w.message, "this import has no effect");
            assert_eq!(w.severity, Severity::Warning);
            assert!(w.hints.is_empty());
            assert!(w.trace.is_empty());
            assert_eq!(source.span_byte_range(w.span), Some(8..11));
            let errors = result.unwrap_err();
            assert_eq!(errors.len(), 1);
            let d = &errors[0];
            assert_eq!(d.message, CONTENT_ERROR);
            assert_eq!(d.severity, Severity::Error);
            assert!(d.hints.is_empty());
            assert!(d.trace.is_empty());
            let start = text.find("[x]").unwrap();
            assert_eq!(source.span_byte_range(d.span), Some(start..start + 3));
        }
    }
    #[test]
    fn p1328_public_numeric_and_untouched_rejections_all_profiles() {
        let world = TestWorld::new("");
        for features in profiles() {
            for (expr, expected) in [
                ("calc.abs(-2pt)", Value::Length(Length::pt(2.0))),
                ("calc.abs(-2deg)", Value::Angle(Angle::deg(2.0))),
                ("calc.abs(-2%)", Value::Ratio(Ratio::from_percent(2.0))),
                ("calc.abs(-2fr)", Value::Fraction(2.0)),
                ("calc.abs(-19)", Value::Int(19)),
                ("calc.abs(-1.25)", Value::Float(1.25)),
                ("calc.abs(decimal(\"-1.25\"))", Value::Decimal(Decimal::new(125, 2))),
            ] {
                let (result, warnings) =
                    eval_expression_with_features(&world, expr, features);
                assert!(warnings.is_empty(), "{expr}: {warnings:?}");
                assert_eq!(result.unwrap(), expected, "{expr}");
            }
            let expr = "calc.abs(-9223372036854775807 - 1)";
            let (result, warnings) =
                eval_expression_with_features(&world, expr, features);
            assert!(warnings.is_empty(), "{expr}: {warnings:?}");
            let errors = result.unwrap_err();
            assert_eq!(errors.len(), 1);
            let d = &errors[0];
            assert_eq!(d.message, "the result is too large");
            assert_eq!(d.severity, Severity::Error);
            assert!(d.hints.is_empty());
            assert!(d.trace.is_empty());
            let source = Source::new_with_parser(
                world.main(),
                expr.into(),
                crate::compiler::parse::parse_code,
            );
            assert_eq!(source.span_byte_range(d.span), Some(9..expr.len() - 1));
            for (expr, message, name) in [
                (
                    "calc.abs(\"x\")",
                    "expected integer, float, length, angle, ratio, fraction, or decimal, found string",
                    "abs",
                ),
                (
                    "calc.abs(sym.alpha)",
                    "expected integer, float, length, angle, ratio, fraction, or decimal, found symbol",
                    "abs",
                ),
                (
                    "calc.sqrt([x])",
                    "calc.sqrt(): esperava Int ou Float, recebeu content",
                    "calc.sqrt",
                ),
            ] {
                let (result, warnings) =
                    eval_expression_with_features(&world, expr, features);
                assert!(warnings.is_empty(), "{expr}: {warnings:?}");
                let errors = result.unwrap_err();
                assert_eq!(errors.len(), 1, "{expr}: {errors:?}");
                let d = &errors[0];
                assert_eq!(d.message, message);
                assert_eq!(d.severity, Severity::Error);
                assert!(d.hints.is_empty());
                let source = Source::new_with_parser(
                    world.main(),
                    expr.into(),
                    crate::compiler::parse::parse_code,
                );
                if name == "abs" {
                    assert_eq!(source.span_byte_range(d.span), Some(9..expr.len() - 1));
                    assert!(d.trace.is_empty(), "{expr}: {d:?}");
                } else {
                    assert!(d.span.is_detached(), "{expr}: {d:?}");
                    assert_eq!(d.trace.len(), 1);
                    assert_eq!(d.trace[0].v, Tracepoint::Call(Some(name.into())));
                    assert_eq!(source.span_byte_range(d.trace[0].span), Some(0..expr.len()));
                }
            }
        }
    }
}

#[cfg(test)]
mod p1329_tests {
    use crate::compiler::eval::{
        eval_expression_with_features, eval_with_full_error_target_and_features,
        EvalContext, EvalTarget,
    };
    use crate::contracts::world::World;
    use crate::entities::args::{ArgOccurrence, Args};
    use crate::entities::compiler_features::{Feature, Features};
    use crate::entities::element_registry::ElementRegistry;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::layout_types::{Abs, Angle, Length, Ratio};
    use crate::entities::module::Module;
    use crate::entities::sink::Sink;
    use crate::entities::source::Source;
    use crate::entities::source_result::{
        Severity, SourceDiagnostic, SourceResult, Tracepoint,
    };
    use crate::entities::span::Span;
    use crate::entities::value::Value;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library, Route, Routines, Traced,
    };
    use comemo::Track;

    const CONTENT_ERROR: &str = "expected integer, float, length, angle, ratio, fraction, or decimal, found content";

    struct TestWorld {
        source: Source,
        library: Library,
        book: FontBook,
    }
    impl TestWorld {
        fn new(text: &str) -> Self {
            Self {
                source: Source::detached(text),
                library: Library::new(),
                book: FontBook::new(),
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
        let world = TestWorld::new(text);
        let mut sink = Sink::new();
        let result = eval_with_full_error_target_and_features(
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
        );
        (world.source.clone(), result, sink.into_diagnostics())
    }
    fn bare_diagnostic(errors: &[SourceDiagnostic], message: &str, span: Span) {
        assert_eq!(errors.len(), 1, "{errors:?}");
        let d = &errors[0];
        assert_eq!(d.message, message);
        assert_eq!(d.severity, Severity::Error);
        assert!(d.hints.is_empty(), "{d:?}");
        assert!(d.trace.is_empty(), "{d:?}");
        assert_eq!(d.span, span);
    }
    fn native(args: &Args, features: Features) -> SourceResult<Value> {
        let world = TestWorld::new("");
        let mut ctx = EvalContext::new();
        ctx.features = features;
        super::calc_abs(&mut ctx, args, &world, world.main())
    }

    const MIXED_ERROR: &str = "cannot take absolute value of this length";

    // Observe the public language species and scalar magnitude, not render bytes
    // or Value's structural PartialEq. NaN is observed as a class, never equality.
    fn magnitude(actual: f64, expected: f64) {
        if expected.is_nan() {
            assert!(actual.is_nan(), "expected NaN, got {actual}");
        } else if expected.is_infinite() {
            assert!(actual.is_infinite() && actual.is_sign_positive(), "{actual}");
        } else {
            assert!(
                (actual - expected).abs() <= 1e-12 * expected.abs().max(1.0),
                "expected {expected}, got {actual}"
            );
        }
    }
    fn semantic(actual: Value, expected: Value) {
        assert_eq!(actual.type_name(), expected.type_name());
        match (actual, expected) {
            (Value::Length(a), Value::Length(e)) => {
                magnitude(a.abs.to_pt(), e.abs.to_pt());
                magnitude(a.em, e.em);
            }
            (Value::Angle(a), Value::Angle(e)) => magnitude(a.to_rad(), e.to_rad()),
            (Value::Ratio(a), Value::Ratio(e)) => magnitude(a.get(), e.get()),
            (Value::Fraction(a), Value::Fraction(e)) => magnitude(a, e),
            (Value::Float(a), Value::Float(e)) => magnitude(a, e),
            (a, e) => panic!("unexpected semantic comparison: {a:?} / {e:?}"),
        }
    }
    #[test]
    fn p1329_native_scalar_species_signs_zero_and_overturn() {
        for features in profiles() {
            for scalar in [-720.25_f64, -3.5, -0.0, 0.0, 2.25, 810.0] {
                for (input, expected) in [
                    (
                        Value::Length(Length::pt(scalar)),
                        Value::Length(Length::pt(scalar.abs())),
                    ),
                    (
                        Value::Length(Length::em(scalar)),
                        Value::Length(Length::em(scalar.abs())),
                    ),
                    (
                        Value::Angle(Angle::rad(scalar)),
                        Value::Angle(Angle::rad(scalar.abs())),
                    ),
                    (Value::Ratio(Ratio(scalar)), Value::Ratio(Ratio(scalar.abs()))),
                    (Value::Fraction(scalar), Value::Fraction(scalar.abs())),
                ] {
                    semantic(
                        native(&Args::positional(vec![input]), features).unwrap(),
                        expected,
                    );
                }
            }
            for (abs, em) in [(-0.0, -4.5), (-4.5, -0.0), (0.0, -0.0), (-0.0, -0.0)] {
                semantic(
                    native(
                        &Args::positional(vec![Value::Length(Length {
                            abs: Abs::pt(abs),
                            em,
                        })]),
                        features,
                    )
                    .unwrap(),
                    Value::Length(Length { abs: Abs::pt(abs.abs()), em: em.abs() }),
                );
            }
        }
    }
    #[test]
    fn p1329_native_nonfinite_values_are_already_constructed() {
        for features in profiles() {
            for scalar in [f64::NAN, f64::NEG_INFINITY, f64::INFINITY] {
                for (input, expected) in [
                    (
                        Value::Length(Length::pt(scalar)),
                        Value::Length(Length::pt(scalar.abs())),
                    ),
                    (
                        Value::Length(Length::em(scalar)),
                        Value::Length(Length::em(scalar.abs())),
                    ),
                    (
                        Value::Angle(Angle::rad(scalar)),
                        Value::Angle(Angle::rad(scalar.abs())),
                    ),
                    (Value::Ratio(Ratio(scalar)), Value::Ratio(Ratio(scalar.abs()))),
                    (Value::Fraction(scalar), Value::Fraction(scalar.abs())),
                    (Value::Float(scalar), Value::Float(scalar.abs())),
                ] {
                    semantic(
                        native(&Args::positional(vec![input]), features).unwrap(),
                        expected,
                    );
                }
                for (abs, em) in [(scalar, 1.0), (1.0, scalar), (scalar, scalar)] {
                    bare_diagnostic(
                        &native(
                            &Args::positional(vec![Value::Length(Length {
                                abs: Abs::pt(abs),
                                em,
                            })]),
                            features,
                        )
                        .unwrap_err(),
                        MIXED_ERROR,
                        Span::detached(),
                    );
                }
            }
        }
    }
    #[test]
    fn p1329_native_mixed_all_signs_value_origin_and_synthetic_traps() {
        let source = Source::detached("aggregate occurrence value other");
        let aggregate = Span::from_range(source.id(), 0..32);
        let occurrence = Span::from_range(source.id(), 10..20);
        let origin = Span::from_range(source.id(), 21..26);
        for features in profiles() {
            for (abs, em) in
                [(2.0, 3.0), (-2.0, -3.0), (2.0, -3.0), (-2.0, 3.0), (1e-100, 1e-100)]
            {
                let value = Value::Length(Length { abs: Abs::pt(abs), em });
                for value_span in [origin, Span::detached()] {
                    let args = Args::from_occurrences(
                        aggregate,
                        vec![ArgOccurrence {
                            name: None,
                            value: value.clone(),
                            span: occurrence,
                            value_span,
                        }],
                    );
                    bare_diagnostic(
                        &native(&args, features).unwrap_err(),
                        MIXED_ERROR,
                        value_span,
                    );
                }
                let args =
                    Args::from_parts(vec![value.clone()], Default::default(), aggregate);
                bare_diagnostic(
                    &native(&args, features).unwrap_err(),
                    MIXED_ERROR,
                    Span::detached(),
                );
                let mut empty_occurrences = args.clone();
                empty_occurrences.occurrences = Some(Vec::new());
                bare_diagnostic(
                    &native(&empty_occurrences, features).unwrap_err(),
                    MIXED_ERROR,
                    Span::detached(),
                );
                // Synthetic occurrence metadata deliberately precedes the first
                // positional origin: it must not hijack the argument's anchor.
                let mut positional_after_named = args;
                positional_after_named.occurrences = Some(vec![
                    ArgOccurrence {
                        name: Some("metadata".into()),
                        value: Value::Int(1),
                        span: aggregate,
                        value_span: occurrence,
                    },
                    ArgOccurrence {
                        name: None,
                        value,
                        span: occurrence,
                        value_span: origin,
                    },
                ]);
                bare_diagnostic(
                    &native(&positional_after_named, features).unwrap_err(),
                    MIXED_ERROR,
                    origin,
                );
            }
        }
    }
    #[test]
    fn p1329_native_guards_precede_dimension_dispatch() {
        for features in profiles() {
            for value in [
                Value::Length(Length::pt(-2.0)),
                Value::Length(Length { abs: Abs::pt(2.0), em: 3.0 }),
                Value::Angle(Angle::deg(-2.0)),
                Value::Ratio(Ratio(-2.0)),
                Value::Fraction(-2.0),
            ] {
                let mut args = Args::positional(vec![value.clone(), value.clone()]);
                args.named.insert("bad".into(), Value::Int(1));
                bare_diagnostic(
                    &native(&args, features).unwrap_err(),
                    if matches!(&value, Value::Length(l) if l.abs.to_pt() != 0.0 && l.em != 0.0)
                    {
                        MIXED_ERROR
                    } else {
                        "unexpected argument"
                    },
                    Span::detached(),
                );
                args.named.clear();
                bare_diagnostic(
                    &native(&args, features).unwrap_err(),
                    if matches!(&value, Value::Length(l) if l.abs.to_pt() != 0.0 && l.em != 0.0)
                    {
                        MIXED_ERROR
                    } else {
                        "unexpected argument"
                    },
                    Span::detached(),
                );
            }
        }
    }
    #[test]
    fn p1329_public_units_species_alias_with_spread() {
        let world = TestWorld::new("");
        for features in profiles() {
            for (expr, expected) in [
                ("calc.abs(-2cm)", Value::Length(Length::pt(7200.0 / 127.0))),
                ("calc.abs(-2mm)", Value::Length(Length::pt(720.0 / 127.0))),
                ("calc.abs(-2in)", Value::Length(Length::pt(144.0))),
                ("calc.abs(-2em)", Value::Length(Length::em(2.0))),
                ("calc.abs(0pt - 3em)", Value::Length(Length::em(3.0))),
                ("calc.abs(-2pt + 0em)", Value::Length(Length::pt(2.0))),
                ("calc.abs(-0em)", Value::Length(Length::ZERO)),
                ("calc.abs(-2rad)", Value::Angle(Angle::rad(2.0))),
                ("calc.abs(-810deg)", Value::Angle(Angle::deg(810.0))),
                ("calc.abs(-250%)", Value::Ratio(Ratio(2.5))),
                ("calc.abs(-2.5fr)", Value::Fraction(2.5)),
                ("{let ab=calc.abs; ab(-2em)}", Value::Length(Length::em(2.0))),
                (
                    "{let ab=calc.abs.with(-2rad).with(); ab()}",
                    Value::Angle(Angle::rad(2.0)),
                ),
                ("calc.abs(..(-250%,))", Value::Ratio(Ratio(2.5))),
                ("{let aa=arguments(-2.5fr); calc.abs(..aa)}", Value::Fraction(2.5)),
                ("{let café = -2em;\n calc.abs(café)}", Value::Length(Length::em(2.0))),
            ] {
                let (result, warnings) =
                    eval_expression_with_features(&world, expr, features);
                assert!(warnings.is_empty(), "{expr}: {warnings:?}");
                semantic(result.unwrap(), expected);
            }
        }
    }
    fn public_error(text: &str, anchor: &str, message: &str, call: Option<&str>) {
        for features in profiles() {
            let (source, result, warnings) = full(text, features);
            assert!(warnings.is_empty(), "{text}: {warnings:?}");
            let errors = result.unwrap_err();
            assert_eq!(errors.len(), 1, "{text}: {errors:?}");
            let d = &errors[0];
            assert_eq!(d.message, message, "{text}");
            assert_eq!(d.severity, Severity::Error);
            assert!(d.hints.is_empty());
            let start = text.rfind(anchor).unwrap();
            assert_eq!(
                source.span_byte_range(d.span),
                Some(start..start + anchor.len()),
                "{text}"
            );
            assert_eq!(d.trace.len(), usize::from(call.is_some()), "{text}: {d:?}");
            if let Some(call) = call {
                let start = text.rfind(call).unwrap();
                assert_eq!(
                    source.span_byte_range(d.trace[0].span),
                    Some(start..start + call.len())
                );
                assert_eq!(d.trace[0].v, Tracepoint::Call(Some("abs".into())));
            }
        }
    }
    #[test]
    fn p1329_public_mixed_origins_all_signs_utf8_alias_with_spread() {
        for (text, anchor, call) in [
            ("#calc.abs(2pt + 3em)", "2pt + 3em", None),
            ("#calc.abs(-2pt - 3em)", "-2pt - 3em", None),
            ("#calc.abs(2pt - 3em)", "2pt - 3em", None),
            ("#calc.abs(-2pt + 3em)", "-2pt + 3em", None),
            ("#let ab = calc.abs\n#ab(2pt + 3em)", "2pt + 3em", None),
            ("#let ab = calc.abs.with()\n#ab(2pt + 3em)", "2pt + 3em", None),
            ("#let ab = calc.abs.with(2pt + 3em)\n#ab()", "2pt + 3em", Some("ab()")),
            (
                "#let ab = calc.abs.with(2pt + 3em).with()\n#ab()",
                "2pt + 3em",
                Some("ab()"),
            ),
            ("#calc.abs(..(2pt + 3em,))", "..(2pt + 3em,)", None),
            (
                "#let aa = arguments(2pt + 3em)\n#calc.abs(..aa)",
                "2pt + 3em",
                Some("calc.abs(..aa)"),
            ),
            ("#let x = 2pt + 3em\n#let y = 2pt + 3em\n#calc.abs(y)", "y", None),
            ("texto á\n#let café = 2pt + 3em\n#calc.abs(\n café\n)", "café", None),
        ] {
            public_error(text, anchor, MIXED_ERROR, call);
        }
    }
    #[test]
    fn p1329_math_dimensional_text_stays_content() {
        for (text, anchor) in [
            ("$std.calc.abs(-2deg)$", "-2deg"),
            ("#let ab = calc.abs\n$ab(-2deg)$", "-2deg"),
        ] {
            public_error(text, anchor, CONTENT_ERROR, None);
        }
    }
    #[test]
    fn p1329_warning_survives_both_success_and_mixed_error() {
        for features in profiles() {
            for (text, success) in [
                ("#import std\n#calc.abs(-2em)", true),
                ("#import std\n#calc.abs(2pt + 3em)", false),
            ] {
                let (source, result, warnings) = full(text, features);
                assert_eq!(warnings.len(), 1);
                let w = &warnings[0];
                assert_eq!(w.message, "this import has no effect");
                assert_eq!(w.severity, Severity::Warning);
                assert!(w.hints.is_empty());
                assert!(w.trace.is_empty());
                assert_eq!(source.span_byte_range(w.span), Some(8..11));
                if success {
                    assert!(result.is_ok());
                } else {
                    let start = text.find("2pt + 3em").unwrap();
                    let errors = result.unwrap_err();
                    assert_eq!(errors.len(), 1);
                    assert_eq!(errors[0].message, MIXED_ERROR);
                    assert_eq!(errors[0].severity, Severity::Error);
                    assert!(errors[0].hints.is_empty());
                    assert!(errors[0].trace.is_empty());
                    assert_eq!(
                        source.span_byte_range(errors[0].span),
                        Some(start..start + 9)
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod p1330_tests {
    use crate::compiler::eval::{
        eval_expression_with_features, eval_with_full_error_target_and_features,
        EvalContext, EvalTarget,
    };
    use crate::contracts::world::World;
    use crate::entities::args::{ArgOccurrence, Args};
    use crate::entities::compiler_features::{Feature, Features};
    use crate::entities::decimal::Decimal;
    use crate::entities::element_registry::ElementRegistry;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::module::Module;
    use crate::entities::sink::Sink;
    use crate::entities::source::Source;
    use crate::entities::source_result::{
        Severity, SourceDiagnostic, SourceResult, Tracepoint,
    };
    use crate::entities::span::Span;
    use crate::entities::value::Value;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library, Route, Routines, Traced,
    };
    use comemo::Track;

    const CONTENT_ERROR: &str = "expected integer, float, length, angle, ratio, fraction, or decimal, found content";

    struct TestWorld {
        source: Source,
        library: Library,
        book: FontBook,
    }
    impl TestWorld {
        fn new(text: &str) -> Self {
            Self {
                source: Source::detached(text),
                library: Library::new(),
                book: FontBook::new(),
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
        let world = TestWorld::new(text);
        let mut sink = Sink::new();
        let result = eval_with_full_error_target_and_features(
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
        );
        (world.source.clone(), result, sink.into_diagnostics())
    }
    fn bare_diagnostic(errors: &[SourceDiagnostic], message: &str, span: Span) {
        assert_eq!(errors.len(), 1, "{errors:?}");
        let d = &errors[0];
        assert_eq!(d.message, message);
        assert_eq!(d.severity, Severity::Error);
        assert!(d.hints.is_empty(), "{d:?}");
        assert!(d.trace.is_empty(), "{d:?}");
        assert_eq!(d.span, span);
    }
    fn native(args: &Args, features: Features) -> SourceResult<Value> {
        let world = TestWorld::new("");
        let mut ctx = EvalContext::new();
        ctx.features = features;
        super::calc_abs(&mut ctx, args, &world, world.main())
    }

    fn public_error(text: &str, anchor: &str, message: &str, call: Option<&str>) {
        for features in profiles() {
            let (source, result, warnings) = full(text, features);
            assert!(warnings.is_empty(), "{text}: {warnings:?}");
            let errors = result.unwrap_err();
            assert_eq!(errors.len(), 1, "{text}: {errors:?}");
            let d = &errors[0];
            assert_eq!(d.message, message, "{text}");
            assert_eq!(d.severity, Severity::Error);
            assert!(d.hints.is_empty());
            let start = text.rfind(anchor).unwrap();
            assert_eq!(
                source.span_byte_range(d.span),
                Some(start..start + anchor.len()),
                "{text}"
            );
            assert_eq!(d.trace.len(), usize::from(call.is_some()), "{text}: {d:?}");
            if let Some(call) = call {
                let start = text.rfind(call).unwrap();
                assert_eq!(
                    source.span_byte_range(d.trace[0].span),
                    Some(start..start + call.len())
                );
                assert_eq!(d.trace[0].v, Tracepoint::Call(Some("abs".into())));
            }
        }
    }

    const OVERFLOW: &str = "the result is too large";

    #[test]
    fn p1330_native_large_noninteger_species_do_not_inherit_integer_limit() {
        for features in profiles() {
            for (input, expected) in [
                (-9223372036854775808.0, 9223372036854775808.0),
                (-1e30, 1e30),
                (-0.0, 0.0),
            ] {
                match native(&Args::positional(vec![Value::Float(input)]), features)
                    .unwrap()
                {
                    Value::Float(actual) => assert_eq!(actual, expected),
                    other => panic!("float abs changed species: {other:?}"),
                }
            }
            match native(
                &Args::positional(vec![Value::Decimal(Decimal::new(i64::MIN, 0))]),
                features,
            )
            .unwrap()
            {
                Value::Decimal(actual) => {
                    assert_eq!(actual.to_string(), "9223372036854775808");
                }
                other => panic!("decimal abs changed species: {other:?}"),
            }
        }
    }

    #[test]
    fn p1330_native_integer_limits_exact_species_and_magnitude() {
        for features in profiles() {
            // Explicit expected values avoid reusing the implementation formula.
            for (input, expected) in [
                (i64::MIN + 1, i64::MAX),
                (i64::MIN + 2, i64::MAX - 1),
                (-9007199254740993, 9007199254740993),
                (-2147483648, 2147483648),
                (-1, 1),
                (0, 0),
                (1, 1),
                (2147483648, 2147483648),
                (9007199254740993, 9007199254740993),
                (i64::MAX - 1, i64::MAX - 1),
                (i64::MAX, i64::MAX),
            ] {
                match native(&Args::positional(vec![Value::Int(input)]), features)
                    .unwrap()
                {
                    Value::Int(actual) => assert_eq!(actual, expected, "input {input}"),
                    other => panic!("integer abs changed species: {other:?}"),
                }
            }
            bare_diagnostic(
                &native(&Args::positional(vec![Value::Int(i64::MIN)]), features)
                    .unwrap_err(),
                OVERFLOW,
                Span::detached(),
            );
        }
    }

    #[test]
    fn p1330_native_overflow_origin_first_positional_and_no_fabrication() {
        let source = Source::detached("aggregate occurrence origin decoy");
        let aggregate = Span::from_range(source.id(), 0..33);
        let occurrence = Span::from_range(source.id(), 10..20);
        let origin = Span::from_range(source.id(), 21..27);
        let decoy = Span::from_range(source.id(), 28..33);
        for features in profiles() {
            for value_span in [origin, Span::detached()] {
                let mut args = Args::from_parts(
                    vec![Value::Int(i64::MIN)],
                    Default::default(),
                    aggregate,
                );
                // Metadata is intentionally inconsistent with the named view to
                // distinguish occurrence filtering from the named-argument guard.
                args.occurrences = Some(vec![
                    ArgOccurrence {
                        name: Some("metadata".into()),
                        value: Value::Int(1),
                        span: aggregate,
                        value_span: decoy,
                    },
                    ArgOccurrence {
                        name: None,
                        value: Value::Int(i64::MIN),
                        span: occurrence,
                        value_span,
                    },
                    ArgOccurrence {
                        name: None,
                        value: Value::Int(i64::MIN),
                        span: decoy,
                        value_span: decoy,
                    },
                ]);
                bare_diagnostic(
                    &native(&args, features).unwrap_err(),
                    OVERFLOW,
                    value_span,
                );
            }
            for occurrences in [None, Some(vec![])] {
                let mut args = Args::from_parts(
                    vec![Value::Int(i64::MIN)],
                    Default::default(),
                    aggregate,
                );
                args.occurrences = occurrences;
                bare_diagnostic(
                    &native(&args, features).unwrap_err(),
                    OVERFLOW,
                    Span::detached(),
                );
            }
        }
    }

    #[test]
    fn p1330_native_named_and_arity_guards_precede_overflow() {
        for features in profiles() {
            for count in [0, 1, 2] {
                let mut args = Args::positional(vec![Value::Int(i64::MIN); count]);
                args.named.insert("bad".into(), Value::Int(1));
                bare_diagnostic(
                    &native(&args, features).unwrap_err(),
                    if count == 0 { "missing argument: value" } else { OVERFLOW },
                    Span::detached(),
                );
                if count != 1 {
                    args.named.clear();
                    bare_diagnostic(
                        &native(&args, features).unwrap_err(),
                        &if count == 0 {
                            String::from("missing argument: value")
                        } else {
                            OVERFLOW.into()
                        },
                        Span::detached(),
                    );
                }
            }
        }
    }

    #[test]
    fn p1330_public_integer_boundaries_and_positive_routes() {
        let world = TestWorld::new("");
        for features in profiles() {
            for (expr, expected) in [
                ("calc.abs(-9223372036854775807)", i64::MAX),
                ("calc.abs(-9223372036854775807 + 1)", i64::MAX - 1),
                ("calc.abs(9223372036854775807)", i64::MAX),
                ("calc.abs(-9007199254740993)", 9007199254740993),
                ("calc.abs(-0)", 0),
                ("{let ab=calc.abs; ab(-1)}", 1),
                ("{let ab=calc.abs.with(-19).with(); ab()}", 19),
                ("{let aa=arguments(-19); calc.abs(..aa)}", 19),
            ] {
                let (result, warnings) =
                    eval_expression_with_features(&world, expr, features);
                assert!(warnings.is_empty(), "{expr}: {warnings:?}");
                match result.unwrap() {
                    Value::Int(actual) => assert_eq!(actual, expected, "{expr}"),
                    other => panic!("{expr}: wrong species {other:?}"),
                }
            }
        }
    }

    #[test]
    fn p1330_public_overflow_real_origins_alias_with_arguments_utf8() {
        for (text, anchor, trace) in [
            ("#calc.abs(-9223372036854775807 - 1)", "-9223372036854775807 - 1", None),
            ("#let ab = calc.abs\n#ab(-9223372036854775807 - 1)", "-9223372036854775807 - 1", None),
            ("#let ab = calc.abs.with()\n#ab(-9223372036854775807 - 1)", "-9223372036854775807 - 1", None),
            ("#let ab = calc.abs.with(-9223372036854775807 - 1)\n#ab()", "-9223372036854775807 - 1", Some("ab()")),
            ("#let ab = calc.abs.with(-9223372036854775807 - 1).with()\n#ab()", "-9223372036854775807 - 1", Some("ab()")),
            ("#calc.abs(..(-9223372036854775807 - 1,))", "..(-9223372036854775807 - 1,)", None),
            ("#let aa = arguments(-9223372036854775807 - 1)\n#calc.abs(..aa)", "-9223372036854775807 - 1", Some("calc.abs(..aa)")),
            ("#let x = -9223372036854775807 - 1\n#let y = -9223372036854775807 - 1\n#calc.abs(y)", "y", None),
            ("texto á\n#let café = -9223372036854775807 - 1\n#calc.abs(\n café\n)", "café", None),
        ] {
            public_error(text, anchor, OVERFLOW, trace);
        }
    }

    #[test]
    fn p1330_math_spelling_remains_content_without_integer_coercion() {
        for text in [
            "$std.calc.abs(-9223372036854775807 - 1)$",
            "#let ab = calc.abs\n$ab(-9223372036854775807 - 1)$",
        ] {
            public_error(text, "-9223372036854775807 - 1", CONTENT_ERROR, None);
        }
    }

    #[test]
    fn p1330_warning_survives_overflow_and_boundary_success() {
        for features in profiles() {
            for (text, fails) in [
                ("#import std\n#calc.abs(-9223372036854775807)", false),
                ("#import std\n#calc.abs(-9223372036854775807 - 1)", true),
            ] {
                let (source, result, warnings) = full(text, features);
                assert_eq!(warnings.len(), 1);
                let w = &warnings[0];
                assert_eq!(w.message, "this import has no effect");
                assert_eq!(w.severity, Severity::Warning);
                assert!(w.hints.is_empty());
                assert!(w.trace.is_empty());
                assert_eq!(source.span_byte_range(w.span), Some(8..11));
                if fails {
                    let errors = result.unwrap_err();
                    assert_eq!(errors.len(), 1);
                    let d = &errors[0];
                    assert_eq!(d.message, OVERFLOW);
                    assert_eq!(d.severity, Severity::Error);
                    assert!(d.hints.is_empty());
                    assert!(d.trace.is_empty());
                    let start = text.find("-9223372036854775807 - 1").unwrap();
                    assert_eq!(
                        source.span_byte_range(d.span),
                        Some(start..start + "-9223372036854775807 - 1".len())
                    );
                } else {
                    assert!(result.is_ok());
                }
            }
        }
    }
}

#[cfg(test)]
mod p1331_tests {
    use crate::compiler::eval::{
        eval_expression_with_features, eval_with_full_error_target_and_features,
        EvalContext, EvalTarget,
    };
    use crate::contracts::world::World;
    use crate::entities::args::{ArgOccurrence, Args};
    use crate::entities::compiler_features::{Feature, Features};
    use crate::entities::decimal::Decimal;
    use crate::entities::element_registry::ElementRegistry;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::layout_types::Length;
    use crate::entities::locator::Locator;
    use crate::entities::module::Module;
    use crate::entities::path::{RootedPath, VirtualPath, VirtualRoot};
    use crate::entities::rel::Rel;
    use crate::entities::sink::Sink;
    use crate::entities::source::Source;
    use crate::entities::source_result::{
        Severity, SourceDiagnostic, SourceResult, Tracepoint,
    };
    use crate::entities::span::Span;
    use crate::entities::value::Value;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library, Route, Routines, Traced,
    };
    use comemo::Track;

    const CONTENT_ERROR: &str = "expected integer, float, length, angle, ratio, fraction, or decimal, found content";

    struct TestWorld {
        source: Source,
        library: Library,
        book: FontBook,
    }
    impl TestWorld {
        fn new(text: &str) -> Self {
            Self {
                source: Source::detached(text),
                library: Library::new(),
                book: FontBook::new(),
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
        // The fixture has one virtual file at the project root. Resolution
        // constructs a value in RAM; it never reads a file or host path.
        fn resolve_path(
            &self,
            current_file: FileId,
            path: &str,
        ) -> Result<RootedPath, String> {
            if current_file != self.main() {
                return Err("unknown virtual file in P1331 fixture".into());
            }
            let vpath = VirtualPath::new(path)
                .map_err(|error| format!("invalid virtual path: {error:?}"))?;
            Ok(RootedPath::new(VirtualRoot::Project, vpath))
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
        let world = TestWorld::new(text);
        let mut sink = Sink::new();
        let result = eval_with_full_error_target_and_features(
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
        );
        (world.source.clone(), result, sink.into_diagnostics())
    }
    fn bare_diagnostic(errors: &[SourceDiagnostic], message: &str, span: Span) {
        assert_eq!(errors.len(), 1, "{errors:?}");
        let d = &errors[0];
        assert_eq!(d.message, message);
        assert_eq!(d.severity, Severity::Error);
        assert!(d.hints.is_empty(), "{d:?}");
        assert!(d.trace.is_empty(), "{d:?}");
        assert_eq!(d.span, span);
    }
    fn native(args: &Args, features: Features) -> SourceResult<Value> {
        let world = TestWorld::new("");
        let mut ctx = EvalContext::new();
        ctx.features = features;
        super::calc_abs(&mut ctx, args, &world, world.main())
    }

    fn public_error(text: &str, anchor: &str, message: &str, call: Option<&str>) {
        for features in profiles() {
            let (source, result, warnings) = full(text, features);
            assert!(warnings.is_empty(), "{text}: {warnings:?}");
            let errors = result.unwrap_err();
            assert_eq!(errors.len(), 1, "{text}: {errors:?}");
            let d = &errors[0];
            assert_eq!(d.message, message, "{text}");
            assert_eq!(d.severity, Severity::Error);
            assert!(d.hints.is_empty());
            let start = text.rfind(anchor).unwrap();
            assert_eq!(
                source.span_byte_range(d.span),
                Some(start..start + anchor.len()),
                "{text}"
            );
            assert_eq!(d.trace.len(), usize::from(call.is_some()), "{text}: {d:?}");
            if let Some(call) = call {
                let start = text.rfind(call).unwrap();
                assert_eq!(
                    source.span_byte_range(d.trace[0].span),
                    Some(start..start + call.len())
                );
                assert_eq!(d.trace[0].v, Tracepoint::Call(Some("abs".into())));
            }
        }
    }

    fn rejection(kind: &str) -> String {
        format!("expected integer, float, length, angle, ratio, fraction, or decimal, found {kind}")
    }

    fn family_values(features: Features) -> Vec<(Value, &'static str)> {
        let world = TestWorld::new("");
        let mut values = Vec::new();
        for (expression, kind) in [
            ("true", "boolean"),
            ("false", "boolean"),
            ("none", "none"),
            ("auto", "auto"),
            ("\"-19\"", "string"),
            ("\"1.25\"", "string"),
            ("\"1pt\"", "string"),
            ("\"\"", "string"),
            ("\"á\"", "string"),
            ("(1,2)", "array"),
            ("(a: 1)", "dictionary"),
            ("calc", "module"),
            ("(x => x)", "function"),
            ("datetime(year: 2026, month: 1, day: 2)", "datetime"),
            ("red", "color"),
            ("1pt + red", "stroke"),
            ("left + top", "alignment"),
            ("gradient.linear(red, blue)", "gradient"),
            ("regex(\"a\")", "regex"),
            ("tiling(size: (2pt, 2pt))[x]", "tiling"),
            ("bytes((1,2))", "bytes"),
            ("duration(seconds: 1)", "duration"),
            ("version(1,2)", "version"),
            ("selector(heading)", "selector"),
            ("sym.alpha", "symbol"),
            ("arguments(1)", "arguments"),
            ("state(\"p1331\", 1)", "state"),
            ("counter(heading)", "counter"),
            ("<x>", "label"),
            ("ltr", "direction"),
            ("location", "type"),
        ] {
            let (result, warnings) =
                eval_expression_with_features(&world, expression, features);
            assert!(warnings.is_empty(), "constructor {expression}: {warnings:?}");
            values.push((result.expect(expression), kind));
        }
        values.push((Value::Relative(Rel::zero()), "relative length"));
        values.push((
            Value::Relative(Rel { rel: 0.5, abs: Length::pt(2.0) }),
            "relative length",
        ));
        values.push((
            Value::Path(RootedPath::new(
                VirtualRoot::Project,
                VirtualPath::new("/p1331.typ").unwrap(),
            )),
            "path",
        ));
        // Preconstructed Location only: this does not test introspection production.
        values.push((Value::Location(Locator::new().next()), "location"));
        values
    }

    #[test]
    fn p1331_native_every_rejected_family_complete_diagnostic_and_origins() {
        let source = Source::detached("aggregate occurrence origin decoy");
        let aggregate = Span::from_range(source.id(), 0..33);
        let occurrence = Span::from_range(source.id(), 10..20);
        let origin = Span::from_range(source.id(), 21..27);
        let decoy = Span::from_range(source.id(), 28..33);
        for features in profiles() {
            for (value, kind) in family_values(features) {
                for value_span in [origin, Span::detached()] {
                    let mut args = Args::from_parts(
                        vec![value.clone()],
                        Default::default(),
                        aggregate,
                    );
                    args.occurrences = Some(vec![
                        ArgOccurrence {
                            name: Some("metadata".into()),
                            value: Value::Int(1),
                            span: aggregate,
                            value_span: decoy,
                        },
                        ArgOccurrence {
                            name: None,
                            value: value.clone(),
                            span: occurrence,
                            value_span,
                        },
                        ArgOccurrence {
                            name: None,
                            value: value.clone(),
                            span: decoy,
                            value_span: decoy,
                        },
                    ]);
                    bare_diagnostic(
                        &native(&args, features).unwrap_err(),
                        &rejection(kind),
                        value_span,
                    );
                }
                for occurrences in [
                    None,
                    Some(vec![]),
                    Some(vec![ArgOccurrence {
                        name: Some("metadata".into()),
                        value: value.clone(),
                        span: aggregate,
                        value_span: decoy,
                    }]),
                ] {
                    let mut args = Args::from_parts(
                        vec![value.clone()],
                        Default::default(),
                        aggregate,
                    );
                    args.occurrences = occurrences;
                    bare_diagnostic(
                        &native(&args, features).unwrap_err(),
                        &rejection(kind),
                        Span::detached(),
                    );
                }
            }
        }
    }

    #[test]
    fn p1331_native_guards_still_precede_every_rejected_family() {
        for features in profiles() {
            for (value, kind) in family_values(features) {
                for count in [0, 1, 2] {
                    let mut args = Args::positional(vec![value.clone(); count]);
                    args.named.insert("bad".into(), Value::Int(1));
                    bare_diagnostic(
                        &native(&args, features).unwrap_err(),
                        &if count == 0 {
                            "missing argument: value".into()
                        } else {
                            rejection(kind)
                        },
                        Span::detached(),
                    );
                    if count != 1 {
                        args.named.clear();
                        bare_diagnostic(
                            &native(&args, features).unwrap_err(),
                            &if count == 0 {
                                String::from("missing argument: value")
                            } else {
                                rejection(kind)
                            },
                            Span::detached(),
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn p1331_public_rejected_families_anchor_the_whole_argument() {
        for (expression, kind) in [
            ("true", "boolean"),
            ("false", "boolean"),
            ("none", "none"),
            ("auto", "auto"),
            ("\"-19\"", "string"),
            ("\"1.25\"", "string"),
            ("\"1pt\"", "string"),
            ("\"\"", "string"),
            ("\"á\"", "string"),
            ("(1,2)", "array"),
            ("(a: 1)", "dictionary"),
            ("calc", "module"),
            ("(x => x)", "function"),
            ("datetime(year: 2026, month: 1, day: 2)", "datetime"),
            ("red", "color"),
            ("1pt + red", "stroke"),
            ("left + top", "alignment"),
            ("gradient.linear(red, blue)", "gradient"),
            ("regex(\"a\")", "regex"),
            ("tiling(size: (2pt, 2pt))[x]", "tiling"),
            ("bytes((1,2))", "bytes"),
            ("duration(seconds: 1)", "duration"),
            ("version(1,2)", "version"),
            ("selector(heading)", "selector"),
            ("sym.alpha", "symbol"),
            ("arguments(1)", "arguments"),
            ("state(\"p1331\", 1)", "state"),
            ("counter(heading)", "counter"),
            ("<x>", "label"),
            ("ltr", "direction"),
            ("location", "type"),
            ("0% + 0pt", "relative length"),
            ("50% + 2pt", "relative length"),
            ("path(\"p1331.typ\")", "path"),
        ] {
            let text = format!("#calc.abs({expression})");
            public_error(&text, expression, &rejection(kind), None);
        }
    }

    #[test]
    fn p1331_public_real_origins_alias_with_spread_arguments_utf8() {
        for (text, anchor, kind, trace) in [
            ("#let ab=calc.abs\n#ab(false)", "false", "boolean", None),
            ("#let ab=calc.abs.with()\n#ab(\"x\")", "\"x\"", "string", None),
            ("#let ab=calc.abs.with(\"x\")\n#ab()", "\"x\"", "string", Some("ab()")),
            (
                "#let ab=calc.abs.with(false).with()\n#ab()",
                "false",
                "boolean",
                Some("ab()"),
            ),
            ("#calc.abs(..(false,))", "..(false,)", "boolean", None),
            (
                "#let aa=arguments(\"x\")\n#calc.abs(..aa)",
                "\"x\"",
                "string",
                Some("calc.abs(..aa)"),
            ),
            ("#let x=\"x\"\n#let y=\"x\"\n#calc.abs(y)", "y", "string", None),
            ("texto á\n#let café=\"á\"\n#calc.abs(\n café\n)", "café", "string", None),
        ] {
            public_error(text, anchor, &rejection(kind), trace);
        }
    }

    #[test]
    fn p1331_math_explicit_string_remains_string_without_numeric_coercion() {
        for text in ["$std.calc.abs(\"-1\")$", "#let ab=calc.abs\n$ab(\"-1\")$"] {
            public_error(text, "\"-1\"", &rejection("string"), None);
        }
    }

    #[test]
    fn p1331_warning_survives_rejection_with_all_fields() {
        let text = "#import std\n#calc.abs(false)";
        for features in profiles() {
            let (source, result, warnings) = full(text, features);
            assert_eq!(warnings.len(), 1);
            let w = &warnings[0];
            assert_eq!(w.message, "this import has no effect");
            assert_eq!(w.severity, Severity::Warning);
            assert!(w.hints.is_empty());
            assert!(w.trace.is_empty());
            assert_eq!(source.span_byte_range(w.span), Some(8..11));
            let errors = result.unwrap_err();
            assert_eq!(errors.len(), 1);
            let d = &errors[0];
            assert_eq!(d.message, rejection("boolean"));
            assert_eq!(d.severity, Severity::Error);
            assert!(d.hints.is_empty());
            assert!(d.trace.is_empty());
            assert_eq!(source.span_byte_range(d.span), Some(22..27));
        }
    }

    #[test]
    fn p1331_existing_numeric_species_and_special_errors_remain() {
        let world = TestWorld::new("");
        for features in profiles() {
            for (expression, expected) in [
                ("(type(calc.abs(-19)), calc.abs(-19))", "(int, 19)"),
                ("(type(calc.abs(-1.25)), calc.abs(-1.25))", "(float, 1.25)"),
                (
                    "(type(calc.abs(decimal(\"-1.25\"))), calc.abs(decimal(\"-1.25\")))",
                    "(decimal, decimal(\"1.25\"))",
                ),
                ("(type(calc.abs(-2pt)), calc.abs(-2pt))", "(length, 2pt)"),
                ("(type(calc.abs(-2em)), calc.abs(-2em))", "(length, 2em)"),
                ("(type(calc.abs(-810deg)), calc.abs(-810deg))", "(angle, 810deg)"),
                ("(type(calc.abs(-250%)), calc.abs(-250%))", "(ratio, 250%)"),
                ("(type(calc.abs(-2fr)), calc.abs(-2fr))", "(fraction, 2fr)"),
            ] {
                let (result, warnings) = eval_expression_with_features(
                    &world,
                    &format!("repr({expression})"),
                    features,
                );
                assert!(warnings.is_empty(), "{expression}: {warnings:?}");
                match result.unwrap() {
                    Value::Str(actual) => assert_eq!(actual.as_str(), expected),
                    other => panic!("repr returned non-string {other:?}"),
                }
            }
            match native(
                &Args::positional(vec![Value::Decimal(Decimal::new(-125, 2))]),
                features,
            )
            .unwrap()
            {
                Value::Decimal(actual) => assert_eq!(actual.to_string(), "1.25"),
                other => panic!("decimal species changed: {other:?}"),
            }
        }
        public_error(
            "#calc.abs(-9223372036854775807 - 1)",
            "-9223372036854775807 - 1",
            "the result is too large",
            None,
        );
        public_error(
            "#calc.abs(2pt + 3em)",
            "2pt + 3em",
            "cannot take absolute value of this length",
            None,
        );
        public_error("#calc.abs([x])", "[x]", CONTENT_ERROR, None);
    }
    #[test]
    fn p1331_fixture_path_constructor_succeeds_before_abs() {
        // This sentinel does not call calc.abs. It establishes the constructor
        // precondition for the unchanged public rejection assertion.
        let world = TestWorld::new("");
        for features in profiles() {
            let (result, warnings) =
                eval_expression_with_features(&world, "path(\"p1331.typ\")", features);
            assert!(warnings.is_empty(), "Path fixture warnings: {warnings:?}");
            match result.expect("Path must be constructed before testing abs") {
                Value::Path(path) => {
                    assert!(matches!(path.root(), VirtualRoot::Project));
                    assert_eq!(path.vpath().get_with_slash(), "/p1331.typ");
                }
                other => panic!("Path constructor changed language type: {other:?}"),
            }
        }
    }
}

#[cfg(test)]
mod p1332_tests {
    use crate::compiler::eval::{
        eval_expression_with_features, eval_with_full_error_target_and_features,
        EvalTarget,
    };
    use crate::contracts::world::World;
    use crate::entities::compiler_features::{Feature, Features};
    use crate::entities::element_registry::ElementRegistry;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::module::Module;
    use crate::entities::sink::Sink;
    use crate::entities::source::Source;
    use crate::entities::source_result::{
        Severity, SourceDiagnostic, SourceResult, Tracepoint,
    };
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
    impl TestWorld {
        fn new(text: &str) -> Self {
            Self {
                source: Source::detached(text),
                library: Library::new(),
                book: FontBook::new(),
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
        let world = TestWorld::new(text);
        let mut sink = Sink::new();
        let result = eval_with_full_error_target_and_features(
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
        );
        (world.source.clone(), result, sink.into_diagnostics())
    }

    #[test]
    fn p1332_intrinsic_name_follows_lookup_alias_import_and_nested_with() {
        let world = TestWorld::new("");
        for features in profiles() {
            for expression in [
                "calc.abs",
                "std.calc.abs",
                "{let a=calc.abs; a}",
                "{import calc: abs; abs}",
                "calc.abs.with()",
                "calc.abs.with(-19)",
                "calc.abs.with().with(-19)",
                "{import calc: abs; let a=abs.with(-19).with(); a}",
            ] {
                let (result, warnings) =
                    eval_expression_with_features(&world, expression, features);
                assert!(warnings.is_empty(), "{expression}: {warnings:?}");
                match result.expect(expression) {
                    Value::Func(function) => {
                        assert_eq!(function.name(), Some("abs"), "{expression}")
                    }
                    other => panic!("function species lost: {expression}: {other:?}"),
                }
            }
            for (expression, expected) in [
                ("calc.sqrt", "calc.sqrt"),
                ("calc.pow", "calc.pow"),
                ("calc.sqrt.with(-1)", "calc.sqrt"),
            ] {
                let (result, warnings) =
                    eval_expression_with_features(&world, expression, features);
                assert!(warnings.is_empty());
                match result.unwrap() {
                    Value::Func(function) => assert_eq!(function.name(), Some(expected)),
                    other => panic!("other function species lost: {other:?}"),
                }
            }
        }
    }
    #[test]
    fn p1332_language_identity_and_with_result_are_preserved() {
        let world = TestWorld::new("");
        for features in profiles() {
            for expression in [
                "calc.abs == std.calc.abs",
                "{import calc: abs; let a=abs; a == calc.abs}",
                "calc.abs != calc.sqrt",
                "calc.abs != calc.pow",
                "calc.abs != (x => x)",
                "{let a=calc.abs.with(-19); let b=a; a == b}",
            ] {
                let (result, warnings) =
                    eval_expression_with_features(&world, expression, features);
                assert!(warnings.is_empty());
                assert!(
                    matches!(result.expect(expression), Value::Bool(true)),
                    "{expression}"
                );
            }
            for expression in [
                "{import calc: abs; abs(-9007199254740993)}",
                "{import calc: abs; let a=abs.with().with(-9007199254740993); a()}",
            ] {
                let (result, warnings) =
                    eval_expression_with_features(&world, expression, features);
                assert!(warnings.is_empty());
                match result.expect(expression) {
                    Value::Int(actual) => assert_eq!(actual, 9007199254740993),
                    other => panic!("exact integer result changed: {other:?}"),
                }
            }
            for (expression, expected) in [
                ("repr(calc.abs)", "abs"),
                ("{import calc: abs; repr(abs)}", "abs"),
                ("{let a=calc.abs; repr(a)}", "abs"),
                ("repr(calc.abs.with(-1))", "(..) => .."),
                ("repr(calc.abs.with().with(-1))", "(..) => .."),
                ("repr(calc.sqrt)", "sqrt"),
            ] {
                let (result, warnings) =
                    eval_expression_with_features(&world, expression, features);
                assert!(warnings.is_empty());
                match result.expect(expression) {
                    Value::Str(actual) => assert_eq!(actual.as_str(), expected),
                    other => panic!("repr result species changed: {other:?}"),
                }
            }
        }
    }

    fn traced_error(
        text: &str,
        anchor: Option<&str>,
        message: &str,
        call: &str,
        name: &str,
        warning: bool,
    ) {
        for features in profiles() {
            let (source, result, warnings) = full(text, features);
            assert_eq!(warnings.len(), usize::from(warning), "{text}: {warnings:?}");
            if warning {
                let w = &warnings[0];
                assert_eq!(w.message, "this import has no effect");
                assert_eq!(w.severity, Severity::Warning);
                assert!(w.hints.is_empty());
                assert!(w.trace.is_empty());
                let start = text.find("std").unwrap();
                assert_eq!(source.span_byte_range(w.span), Some(start..start + 3));
            }
            let errors = result.unwrap_err();
            assert_eq!(errors.len(), 1, "{text}: {errors:?}");
            let d = &errors[0];
            assert_eq!(d.message, message, "{text}");
            assert_eq!(d.severity, Severity::Error);
            assert!(d.hints.is_empty());
            if let Some(anchor) = anchor {
                let start = text.rfind(anchor).unwrap();
                assert_eq!(
                    source.span_byte_range(d.span),
                    Some(start..start + anchor.len()),
                    "{text}"
                );
            } else {
                assert!(d.span.is_detached(), "{text}: {d:?}");
            }
            if message == "missing argument: value" {
                assert!(d.trace.is_empty(), "{text}: {d:?}");
                continue;
            }
            assert_eq!(d.trace.len(), 1, "{text}: {d:?}");
            assert_eq!(d.trace[0].v, Tracepoint::Call(Some(name.into())));
            let start = text.rfind(call).unwrap();
            assert_eq!(
                source.span_byte_range(d.trace[0].span),
                Some(start..start + call.len())
            );
        }
    }

    #[test]
    fn p1332_imported_abs_external_origins_all_error_families() {
        for (value, message) in [
            ("[x]", "expected integer, float, length, angle, ratio, fraction, or decimal, found content"),
            ("2pt + 3em", "cannot take absolute value of this length"),
            ("-9223372036854775807 - 1", "the result is too large"),
            ("false", "expected integer, float, length, angle, ratio, fraction, or decimal, found boolean"),
            ("\"x\"", "expected integer, float, length, angle, ratio, fraction, or decimal, found string"),
        ] {
            for text in [
                format!("#import calc: abs\n#let a=abs.with({value})\n#a()"),
                format!("#import calc: abs\n#let a=abs.with({value}).with()\n#a()"),
                format!("#import calc: abs\n#let aa=arguments({value})\n#abs(..aa)"),
            ] {
                let call = if text.ends_with("#a()") { "a()" } else { "abs(..aa)" };
                traced_error(&text, Some(value), message, call, "abs", false);
            }
        }
    }
    #[test]
    fn p1332_trace_rename_retains_warning_utf8_and_guard_debt() {
        traced_error(
            "#import std\n#let café=calc.abs.with([á])\n#café()",
            Some("[á]"),
            "expected integer, float, length, angle, ratio, fraction, or decimal, found content",
            "café()", "abs", true,
        );
        for (text, message, call) in [
            ("#calc.abs()", "missing argument: value", "calc.abs()"),
            (
                "#let a=calc.abs.with()\n#a()",
                "missing argument: value",
                "a()",
            ),
            (
                "#let a=calc.abs.with(false, 1)\n#a()",
                "expected integer, float, length, angle, ratio, fraction, or decimal, found boolean",
                "a()",
            ),
            (
                "#let a=calc.abs.with(false, bad: 1)\n#a()",
                "expected integer, float, length, angle, ratio, fraction, or decimal, found boolean",
                "a()",
            ),
        ] {
            traced_error(text, if text.contains("with(false,") { Some("false") } else { Some(call) }, message, call, "abs", false);
        }
        traced_error(
            "#let a=calc.sqrt.with(\"x\")\n#a()",
            None,
            "calc.sqrt(): esperava Int ou Float, recebeu str",
            "a()",
            "calc.sqrt",
            false,
        );
    }
}

#[cfg(test)]
mod p1333_tests {
    use crate::compiler::eval::{
        eval_expression_with_features, eval_with_full_error_target_and_features,
        EvalContext, EvalTarget,
    };
    use crate::contracts::world::World;
    use crate::entities::args::{ArgOccurrence, Args};
    use crate::entities::compiler_features::{Feature, Features};
    use crate::entities::decimal::Decimal;
    use crate::entities::element_registry::ElementRegistry;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::layout_types::Length;
    use crate::entities::locator::Locator;
    use crate::entities::module::Module;
    use crate::entities::path::{RootedPath, VirtualPath, VirtualRoot};
    use crate::entities::rel::Rel;
    use crate::entities::sink::Sink;
    use crate::entities::source::Source;
    use crate::entities::source_result::{
        Severity, SourceDiagnostic, SourceResult, Tracepoint,
    };
    use crate::entities::span::Span;
    use crate::entities::value::Value;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library, Route, Routines, Traced,
    };
    use comemo::Track;

    const CONTENT_ERROR: &str = "expected integer, float, length, angle, ratio, fraction, or decimal, found content";

    struct TestWorld {
        source: Source,
        library: Library,
        book: FontBook,
    }
    impl TestWorld {
        fn new(text: &str) -> Self {
            Self {
                source: Source::detached(text),
                library: Library::new(),
                book: FontBook::new(),
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
        // The fixture has one virtual file at the project root. Resolution
        // constructs a value in RAM; it never reads a file or host path.
        fn resolve_path(
            &self,
            current_file: FileId,
            path: &str,
        ) -> Result<RootedPath, String> {
            if current_file != self.main() {
                return Err("unknown virtual file in P1331 fixture".into());
            }
            let vpath = VirtualPath::new(path)
                .map_err(|error| format!("invalid virtual path: {error:?}"))?;
            Ok(RootedPath::new(VirtualRoot::Project, vpath))
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
        let world = TestWorld::new(text);
        let mut sink = Sink::new();
        let result = eval_with_full_error_target_and_features(
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
        );
        (world.source.clone(), result, sink.into_diagnostics())
    }
    fn bare_diagnostic(errors: &[SourceDiagnostic], message: &str, span: Span) {
        assert_eq!(errors.len(), 1, "{errors:?}");
        let d = &errors[0];
        assert_eq!(d.message, message);
        assert_eq!(d.severity, Severity::Error);
        assert!(d.hints.is_empty(), "{d:?}");
        assert!(d.trace.is_empty(), "{d:?}");
        assert_eq!(d.span, span);
    }
    fn native(args: &Args, features: Features) -> SourceResult<Value> {
        let world = TestWorld::new("");
        let mut ctx = EvalContext::new();
        ctx.features = features;
        super::calc_abs(&mut ctx, args, &world, world.main())
    }

    fn public_error(text: &str, anchor: &str, message: &str, call: Option<&str>) {
        for features in profiles() {
            let (source, result, warnings) = full(text, features);
            assert!(warnings.is_empty(), "{text}: {warnings:?}");
            let errors = result.unwrap_err();
            assert_eq!(errors.len(), 1, "{text}: {errors:?}");
            let d = &errors[0];
            assert_eq!(d.message, message, "{text}");
            assert_eq!(d.severity, Severity::Error);
            assert!(d.hints.is_empty());
            let start = text.rfind(anchor).unwrap();
            assert_eq!(
                source.span_byte_range(d.span),
                Some(start..start + anchor.len()),
                "{text}"
            );
            assert_eq!(d.trace.len(), usize::from(call.is_some()), "{text}: {d:?}");
            if let Some(call) = call {
                let start = text.rfind(call).unwrap();
                assert_eq!(
                    source.span_byte_range(d.trace[0].span),
                    Some(start..start + call.len())
                );
                assert_eq!(d.trace[0].v, Tracepoint::Call(Some("abs".into())));
            }
        }
    }

    fn rejection(kind: &str) -> String {
        format!("expected integer, float, length, angle, ratio, fraction, or decimal, found {kind}")
    }

    fn family_values(features: Features) -> Vec<(Value, &'static str)> {
        let world = TestWorld::new("");
        let mut values = Vec::new();
        for (expression, kind) in [
            ("true", "boolean"),
            ("false", "boolean"),
            ("none", "none"),
            ("auto", "auto"),
            ("\"-19\"", "string"),
            ("\"1.25\"", "string"),
            ("\"1pt\"", "string"),
            ("\"\"", "string"),
            ("\"á\"", "string"),
            ("(1,2)", "array"),
            ("(a: 1)", "dictionary"),
            ("calc", "module"),
            ("(x => x)", "function"),
            ("datetime(year: 2026, month: 1, day: 2)", "datetime"),
            ("red", "color"),
            ("1pt + red", "stroke"),
            ("left + top", "alignment"),
            ("gradient.linear(red, blue)", "gradient"),
            ("regex(\"a\")", "regex"),
            ("tiling(size: (2pt, 2pt))[x]", "tiling"),
            ("bytes((1,2))", "bytes"),
            ("duration(seconds: 1)", "duration"),
            ("version(1,2)", "version"),
            ("selector(heading)", "selector"),
            ("sym.alpha", "symbol"),
            ("arguments(1)", "arguments"),
            ("state(\"p1331\", 1)", "state"),
            ("counter(heading)", "counter"),
            ("<x>", "label"),
            ("ltr", "direction"),
            ("location", "type"),
        ] {
            let (result, warnings) =
                eval_expression_with_features(&world, expression, features);
            assert!(warnings.is_empty(), "constructor {expression}: {warnings:?}");
            values.push((result.expect(expression), kind));
        }
        values.push((Value::Relative(Rel::zero()), "relative length"));
        values.push((
            Value::Relative(Rel { rel: 0.5, abs: Length::pt(2.0) }),
            "relative length",
        ));
        values.push((
            Value::Path(RootedPath::new(
                VirtualRoot::Project,
                VirtualPath::new("/p1331.typ").unwrap(),
            )),
            "path",
        ));
        // Preconstructed Location only: this does not test introspection production.
        values.push((Value::Location(Locator::new().next()), "location"));
        values
    }

    fn failures(features: Features) -> Vec<(Value, String)> {
        use crate::entities::content::Content;
        use crate::entities::value::IntrospectedContent;
        let mut values: Vec<_> = family_values(features)
            .into_iter()
            .map(|(value, kind)| (value, rejection(kind)))
            .collect();
        let content = Content::strong(Content::text("-1"));
        values.extend([
            (Value::Content(content.clone()), CONTENT_ERROR.into()),
            (
                Value::LocatedContent(
                    IntrospectedContent::new(content, None),
                    Locator::new().next(),
                ),
                CONTENT_ERROR.into(),
            ),
            (Value::Int(i64::MIN), "the result is too large".into()),
        ]);
        for abs in [-2.0, 2.0] {
            for em in [-3.0, 3.0] {
                values.push((
                    Value::Length(Length {
                        abs: crate::entities::layout_types::Abs::pt(abs),
                        em,
                    }),
                    "cannot take absolute value of this length".into(),
                ));
            }
        }
        values
    }

    #[test]
    fn p1333_native_first_failure_wins_all_families_and_origins() {
        let source = Source::detached("aggregate occurrence origin decoy");
        let aggregate = Span::from_range(source.id(), 0..33);
        let occurrence = Span::from_range(source.id(), 10..20);
        let origin = Span::from_range(source.id(), 21..27);
        let decoy = Span::from_range(source.id(), 28..33);
        for features in profiles() {
            for (value, message) in failures(features) {
                for count in [1, 2, 3] {
                    for named in [false, true] {
                        for value_span in [origin, Span::detached()] {
                            let mut items = vec![value.clone()];
                            items.extend(vec![Value::Int(-7); count - 1]);
                            let mut args =
                                Args::from_parts(items, Default::default(), aggregate);
                            if named {
                                args.named.insert("bad".into(), Value::Int(1));
                            }
                            args.occurrences = Some(vec![
                                ArgOccurrence {
                                    name: Some("bad".into()),
                                    value: Value::Int(1),
                                    span: aggregate,
                                    value_span: decoy,
                                },
                                // Deliberately inconsistent metadata must not replace Args.items.
                                ArgOccurrence {
                                    name: None,
                                    value: Value::Int(7),
                                    span: occurrence,
                                    value_span,
                                },
                                ArgOccurrence {
                                    name: None,
                                    value: Value::Bool(false),
                                    span: decoy,
                                    value_span: decoy,
                                },
                            ]);
                            bare_diagnostic(
                                &native(&args, features).unwrap_err(),
                                &message,
                                value_span,
                            );
                            for origins in [
                                None,
                                Some(vec![]),
                                Some(vec![ArgOccurrence {
                                    name: Some("bad".into()),
                                    value: value.clone(),
                                    span: occurrence,
                                    value_span: decoy,
                                }]),
                            ] {
                                args.occurrences = origins;
                                bare_diagnostic(
                                    &native(&args, features).unwrap_err(),
                                    &message,
                                    Span::detached(),
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn p1333_native_valid_first_never_substitutes_a_later_failure() {
        use crate::entities::layout_types::{Angle, Ratio};
        let source = Source::detached("aggregate origin");
        let aggregate = Span::from_range(source.id(), 0..16);
        for features in profiles() {
            for first in [
                Value::Int(-19),
                Value::Float(-1.25),
                Value::Float(f64::NAN),
                Value::Float(f64::NEG_INFINITY),
                Value::Decimal(Decimal::new(-125, 2)),
                Value::Length(Length::pt(-2.0)),
                Value::Length(Length {
                    abs: crate::entities::layout_types::Abs::pt(0.0),
                    em: -2.0,
                }),
                Value::Angle(Angle::deg(-810.0)),
                Value::Ratio(Ratio(-2.5)),
                Value::Fraction(-2.0),
            ] {
                for later in [
                    Value::Bool(false),
                    Value::Int(i64::MIN),
                    Value::Length(Length {
                        abs: crate::entities::layout_types::Abs::pt(2.0),
                        em: 3.0,
                    }),
                ] {
                    let mut args = Args::from_parts(
                        vec![first.clone(), later],
                        Default::default(),
                        aggregate,
                    );
                    bare_diagnostic(
                        &native(&args, features).unwrap_err(),
                        "unexpected argument",
                        Span::detached(),
                    );
                    args.named.insert("bad".into(), Value::Int(1));
                    bare_diagnostic(
                        &native(&args, features).unwrap_err(),
                        "unexpected argument",
                        Span::detached(),
                    );
                    args.items.truncate(1);
                    bare_diagnostic(
                        &native(&args, features).unwrap_err(),
                        "unexpected argument: bad",
                        Span::detached(),
                    );
                }
            }
            let mut args = Args::from_parts(vec![], Default::default(), aggregate);
            // P1334: coherent synthetic missing fixture (historical Some was stale).
            bare_diagnostic(
                &native(&args, features).unwrap_err(),
                "missing argument: value",
                aggregate,
            );
            args.named.insert("bad".into(), Value::Int(1));
            bare_diagnostic(
                &native(&args, features).unwrap_err(),
                "missing argument: value",
                aggregate,
            );
        }
    }

    #[test]
    fn p1333_public_errors_precede_named_and_extra_without_losing_origin() {
        for (value, message) in [
            ("false", rejection("boolean")),
            ("\"-19\"", rejection("string")),
            ("[x]", CONTENT_ERROR.into()),
            ("-9223372036854775807 - 1", "the result is too large".into()),
            ("2pt + 3em", "cannot take absolute value of this length".into()),
        ] {
            for text in [
                format!("#calc.abs({value}, 2)"),
                format!("#calc.abs({value}, bad: 1)"),
                format!("#calc.abs(bad: 1, {value}, 2)"),
                format!("#let a=calc.abs\n#a({value}, bad: 1, 2)"),
                format!("#import calc: abs\n#abs(bad: 1, {value}, 2)"),
            ] {
                public_error(&text, value, &message, None);
            }
            for text in [
                format!("#let a=calc.abs.with({value}, bad: 1)\n#a(2)"),
                format!(
                    "#import calc: abs\n#let a=abs.with(bad: 1).with({value})\n#a(2)"
                ),
            ] {
                public_error(&text, value, &message, Some("a(2)"));
            }
            let text = format!("#let aa=arguments(bad: 1, {value}, 2)\n#calc.abs(..aa)");
            public_error(&text, value, &message, Some("calc.abs(..aa)"));
        }
        public_error(
            "#calc.abs(..(false, 2), bad: 1)",
            "..(false, 2)",
            &rejection("boolean"),
            None,
        );
        public_error(
            "texto á\n#let café=[á]\n#calc.abs(bad: 1,\n café, 2)",
            "café",
            CONTENT_ERROR,
            None,
        );
    }

    #[test]
    fn p1333_eager_argument_failure_still_precedes_native_entry() {
        let world = TestWorld::new("");
        for features in profiles() {
            for expr in [
                "calc.abs(false, panic(\"p1333-eager\"))",
                "calc.abs(false, bad: panic(\"p1333-eager\"))",
                "calc.abs(bad: panic(\"p1333-eager\"), false)",
            ] {
                let (result, warnings) =
                    eval_expression_with_features(&world, expr, features);
                assert!(warnings.is_empty());
                let errors = result.unwrap_err();
                assert_eq!(errors.len(), 1);
                assert_eq!(errors[0].message, "panicked with: p1333-eager");
                assert_eq!(errors[0].severity, Severity::Error);
                assert!(errors[0].hints.is_empty());
            }
        }
    }
}

#[cfg(test)]
mod p1334_tests {
    use crate::compiler::eval::{
        eval_with_full_error_target_and_features, EvalContext, EvalTarget,
    };
    use crate::contracts::world::World;
    use crate::entities::args::{ArgOccurrence, Args};
    use crate::entities::compiler_features::{Feature, Features};
    use crate::entities::decimal::Decimal;
    use crate::entities::element_registry::ElementRegistry;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::layout_types::Length;

    use crate::entities::module::Module;
    use crate::entities::path::{RootedPath, VirtualPath, VirtualRoot};

    use crate::entities::sink::Sink;
    use crate::entities::source::Source;
    use crate::entities::source_result::{
        Severity, SourceDiagnostic, SourceResult, Tracepoint,
    };
    use crate::entities::span::Span;
    use crate::entities::value::Value;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library, Route, Routines, Traced,
    };
    use comemo::Track;

    const CONTENT_ERROR: &str = "expected integer, float, length, angle, ratio, fraction, or decimal, found content";

    struct TestWorld {
        source: Source,
        library: Library,
        book: FontBook,
    }
    impl TestWorld {
        fn new(text: &str) -> Self {
            Self {
                source: Source::detached(text),
                library: Library::new(),
                book: FontBook::new(),
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
        // The fixture has one virtual file at the project root. Resolution
        // constructs a value in RAM; it never reads a file or host path.
        fn resolve_path(
            &self,
            current_file: FileId,
            path: &str,
        ) -> Result<RootedPath, String> {
            if current_file != self.main() {
                return Err("unknown virtual file in P1331 fixture".into());
            }
            let vpath = VirtualPath::new(path)
                .map_err(|error| format!("invalid virtual path: {error:?}"))?;
            Ok(RootedPath::new(VirtualRoot::Project, vpath))
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
        let world = TestWorld::new(text);
        let mut sink = Sink::new();
        let result = eval_with_full_error_target_and_features(
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
        );
        (world.source.clone(), result, sink.into_diagnostics())
    }
    fn native(args: &Args, features: Features) -> SourceResult<Value> {
        let world = TestWorld::new("");
        let mut ctx = EvalContext::new();
        ctx.features = features;
        super::calc_abs(&mut ctx, args, &world, world.main())
    }

    fn occurrence(
        name: Option<&str>,
        value: Value,
        span: Span,
        value_span: Span,
    ) -> ArgOccurrence {
        ArgOccurrence {
            name: name.map(Into::into),
            value,
            span,
            value_span,
        }
    }
    fn diagnostic(args: &Args, message: &str, span: Span, hint: bool) {
        for features in profiles() {
            let before = args.occurrence_sequence();
            let errors = native(args, features).unwrap_err();
            assert_eq!(errors.len(), 1);
            let d = &errors[0];
            assert_eq!(d.message, message);
            assert_eq!(d.severity, Severity::Error);
            assert_eq!(d.span, span);
            assert!(d.trace.is_empty());
            let hints: Vec<&str> = d.hints.iter().map(|s| s.as_str()).collect();
            assert_eq!(hints, if hint { vec!["try removing `value:`"] } else { vec![] });
            let after = args.occurrence_sequence();
            assert_eq!(before.len(), after.len());
            for (a, b) in before.iter().zip(&after) {
                assert_eq!(a.name, b.name);
                assert_eq!(a.span, b.span);
                assert_eq!(a.value_span, b.value_span);
            }
        }
    }
    #[test]
    fn p1334_missing_uses_received_aggregate_even_with_unrelated_named() {
        let source = Source::detached("aggregate named value");
        for aggregate in [Span::from_range(source.id(), 0..20), Span::detached()] {
            for args in [
                Args::from_occurrences(aggregate, vec![]),
                Args::from_occurrences(
                    aggregate,
                    vec![occurrence(
                        Some("bad"),
                        Value::Bool(false),
                        Span::from_range(source.id(), 10..20),
                        Span::from_range(source.id(), 16..20),
                    )],
                ),
                Args::from_parts(vec![], Default::default(), aggregate),
            ] {
                diagnostic(&args, "missing argument: value", aggregate, false);
            }
        }
    }
    #[test]
    fn p1334_named_value_selects_first_full_occurrence_and_hint_without_cast() {
        use std::num::NonZeroU16;
        let file_a = FileId::from_raw(NonZeroU16::new(121).unwrap());
        let file_b = FileId::from_raw(NonZeroU16::new(122).unwrap());
        let aggregate = Span::from_range(file_b, 0..80);
        let first = Span::from_range(file_a, 10..30);
        for argspan in [first, Span::detached()] {
            let args = Args::from_occurrences(
                aggregate,
                vec![
                    occurrence(
                        Some("bad"),
                        Value::Int(1),
                        Span::from_range(file_b, 2..8),
                        Span::from_range(file_b, 7..8),
                    ),
                    occurrence(
                        Some("value"),
                        Value::Bool(false),
                        argspan,
                        Span::from_range(file_a, 18..23),
                    ),
                    occurrence(
                        Some("value"),
                        Value::Int(-19),
                        Span::from_range(file_b, 40..50),
                        Span::from_range(file_b, 47..50),
                    ),
                ],
            );
            diagnostic(&args, "the argument `value` is positional", argspan, true);
        }
        let mut synthetic = Args::from_parts(vec![], Default::default(), aggregate);
        synthetic.named.insert("bad".into(), Value::Int(1));
        synthetic.named.insert("value".into(), Value::Bool(false));
        diagnostic(
            &synthetic,
            "the argument `value` is positional",
            Span::detached(),
            true,
        );
    }
    #[test]
    fn p1334_surplus_joint_order_full_span_and_value_name_has_no_hint() {
        let source = Source::detached("aggregate first extra named");
        let aggregate = Span::from_range(source.id(), 0..27);
        let first = occurrence(
            None,
            Value::Int(-19),
            Span::from_range(source.id(), 10..15),
            Span::from_range(source.id(), 11..15),
        );
        let extra = occurrence(
            None,
            Value::Bool(false),
            Span::from_range(source.id(), 16..21),
            Span::from_range(source.id(), 17..21),
        );
        let named = occurrence(
            Some("value"),
            Value::Int(7),
            Span::from_range(source.id(), 22..27),
            Span::from_range(source.id(), 24..27),
        );
        for (sequence, message, span) in [
            (
                vec![first.clone(), extra.clone(), named.clone()],
                "unexpected argument",
                extra.span,
            ),
            (
                vec![named.clone(), first.clone(), extra.clone()],
                "unexpected argument: value",
                named.span,
            ),
            (
                vec![first.clone(), named.clone(), extra.clone()],
                "unexpected argument: value",
                named.span,
            ),
        ] {
            diagnostic(
                &Args::from_occurrences(aggregate, sequence),
                message,
                span,
                false,
            );
        }
        let mut synthetic = Args::from_parts(
            vec![Value::Int(-1), Value::Bool(false)],
            Default::default(),
            aggregate,
        );
        synthetic.named.insert("value".into(), Value::Int(1));
        diagnostic(&synthetic, "unexpected argument", Span::detached(), false);
        synthetic.items.truncate(1);
        diagnostic(&synthetic, "unexpected argument: value", Span::detached(), false);
    }
    #[test]
    fn p1334_coherent_first_invalid_remains_so_before_any_surplus() {
        use crate::entities::content::Content;
        use crate::entities::layout_types::Abs;
        let source = Source::detached("aggregate first extra named");
        let aggregate = Span::from_range(source.id(), 0..27);
        for (value,message) in [
            (Value::Bool(false),"expected integer, float, length, angle, ratio, fraction, or decimal, found boolean"),
            (Value::Content(Content::empty()),CONTENT_ERROR),
            (Value::Int(i64::MIN),"the result is too large"),
            (Value::Length(Length { abs: Abs::pt(2.0), em: 3.0 }),"cannot take absolute value of this length"),
        ] {
            for origin in [Span::from_range(source.id(),11..15),Span::detached()] {
                let args=Args::from_occurrences(aggregate,vec![
                    occurrence(Some("value"),Value::Int(1),aggregate,aggregate),
                    occurrence(None,value.clone(),Span::from_range(source.id(),10..15),origin),
                    occurrence(None,Value::Int(7),aggregate,aggregate),
                ]);
                diagnostic(&args,message,origin,false);
            }
        }
    }
    #[test]
    fn p1334_all_valid_species_report_the_surplus_without_converting_it() {
        use crate::entities::layout_types::{Abs, Angle, Ratio};
        let source = Source::detached("first surplus");
        let extra = Span::from_range(source.id(), 6..13);
        for first in [
            Value::Int(-19),
            Value::Float(-1.25),
            Value::Decimal(Decimal::new(-125, 2)),
            Value::Length(Length::pt(-2.0)),
            Value::Length(Length { abs: Abs::pt(0.0), em: -2.0 }),
            Value::Angle(Angle::deg(-810.0)),
            Value::Ratio(Ratio(-2.5)),
            Value::Fraction(-2.0),
            Value::Float(f64::NAN),
            Value::Float(f64::NEG_INFINITY),
        ] {
            let args = Args::from_occurrences(
                Span::detached(),
                vec![
                    occurrence(None, first, Span::detached(), Span::detached()),
                    occurrence(None, Value::Int(i64::MIN), extra, Span::detached()),
                ],
            );
            diagnostic(&args, "unexpected argument", extra, false);
        }
    }
    fn public_error(
        text: &str,
        anchor: &str,
        message: &str,
        hint: bool,
        trace: Option<&str>,
    ) {
        for features in profiles() {
            let (source, result, warnings) = full(text, features);
            assert!(warnings.is_empty(), "{text}: {warnings:?}");
            let errors = result.unwrap_err();
            assert_eq!(errors.len(), 1);
            let d = &errors[0];
            assert_eq!(d.message, message, "{text}");
            assert_eq!(d.severity, Severity::Error);
            let start = text.rfind(anchor).unwrap();
            assert_eq!(
                source.span_byte_range(d.span),
                Some(start..start + anchor.len()),
                "{text}"
            );
            let hints: Vec<&str> = d.hints.iter().map(|x| x.as_str()).collect();
            assert_eq!(hints, if hint { vec!["try removing `value:`"] } else { vec![] });
            assert_eq!(d.trace.len(), usize::from(trace.is_some()), "{text}: {d:?}");
            if let Some(call) = trace {
                let start = text.rfind(call).unwrap();
                assert_eq!(
                    source.span_byte_range(d.trace[0].span),
                    Some(start..start + call.len())
                );
                assert_eq!(d.trace[0].v, Tracepoint::Call(Some("abs".into())));
            }
        }
    }
    #[test]
    fn p1334_public_missing_alias_import_with_spread_and_math() {
        for (text, anchor) in [
            ("#calc.abs()", "calc.abs()"),
            ("#let café=calc.abs\n#café()", "café()"),
            ("#import calc: abs\n#abs()", "abs()"),
            ("#let a=calc.abs.with().with()\n#a()", "a()"),
            ("#calc.abs(..none)", "calc.abs(..none)"),
            ("#calc.abs(..())", "calc.abs(..())"),
            ("#let aa=arguments()\n#calc.abs(..aa)", "calc.abs(..aa)"),
            ("$std.calc.abs()$", "std.calc.abs()"),
        ] {
            public_error(text, anchor, "missing argument: value", false, None);
        }
    }
    #[test]
    fn p1334_public_named_and_surplus_preserve_external_origins() {
        for (text, anchor, message, hint, trace) in [
            (
                "#calc.abs(bad: 1, value: false)",
                "value: false",
                "the argument `value` is positional",
                true,
                None,
            ),
            (
                "#let a=calc.abs.with(bad: 1).with(value: false).with(value: -1)\n#a()",
                "value: false",
                "the argument `value` is positional",
                true,
                Some("a()"),
            ),
            (
                "#let aa=arguments(bad: 1, value: false)\n#calc.abs(..aa)",
                "value: false",
                "the argument `value` is positional",
                true,
                Some("calc.abs(..aa)"),
            ),
            (
                "#calc.abs(..(value: false))",
                "..(value: false)",
                "the argument `value` is positional",
                true,
                None,
            ),
            ("#calc.abs(-1, 2, bad: 1)", "2", "unexpected argument", false, None),
            (
                "#calc.abs(value: 2, -1)",
                "value: 2",
                "unexpected argument: value",
                false,
                None,
            ),
            (
                "#let a=calc.abs.with(-1, bad: false).with(2)\n#a()",
                "bad: false",
                "unexpected argument: bad",
                false,
                Some("a()"),
            ),
            (
                "#let aa=arguments(-1, 2, bad: 1)\n#calc.abs(..aa)",
                "2",
                "unexpected argument",
                false,
                Some("calc.abs(..aa)"),
            ),
            ("#calc.abs(..(-1,2))", "..(-1,2)", "unexpected argument", false, None),
        ] {
            public_error(text, anchor, message, hint, trace);
        }
    }
}
