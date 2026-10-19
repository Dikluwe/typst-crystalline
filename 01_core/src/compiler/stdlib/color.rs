//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/color.md
//! @prompt-hash f9e64263
//! @layer L1
//! @updated 2026-06-27
//!
//! Módulo `color` — tipo `color` (P736/P1143) com constructors, operadores e
//! as 18 cores predefinidas do vanilla ratificado.
//! **P742** — semântica dos operadores corrigida por medição vanilla
//! (lighten/darken no espaço da cor, saturate/desaturate via HSV, negate em
//! Oklab) + despacho de métodos de instância (`red.lighten(20%)`).

use ecow::EcoString;
use std::sync::Arc;

use crate::compiler::eval::operators::error_formatting::vanilla_type_name;
use crate::compiler::eval::EvalContext;
use crate::entities::args::{ArgOccurrence, Args};
use crate::entities::color::{ColorSpaceError, SpotColor, SpotColorant};
use crate::entities::file_id::FileId;
use crate::entities::func::Func;
use crate::entities::layout_types::{Color, ColorSpace};
use crate::entities::module::Module;
use crate::entities::scope::Scope;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::{Type, Value};

use super::err;

mod alpha;
mod arguments;
mod inspect;
mod light;
mod mix;
mod palette;
mod spot;
mod transform;

pub(crate) use self::alpha::{native_color_opacify, native_color_transparentize};
use self::arguments::*;
pub(crate) use self::inspect::{
    native_color_components, native_color_space, native_color_to_hex,
};
pub(crate) use self::light::{native_color_darken, native_color_lighten};
pub(crate) use self::mix::native_color_mix;
use self::palette::predefined_color;
pub use self::palette::predefined_color_bindings;
pub(crate) use self::spot::{
    dispatch_spot_colorant_tint, native_spot_colorant_construct, native_spot_tint,
};
pub(crate) use self::transform::{
    native_color_desaturate, native_color_negate, native_color_rotate,
    native_color_saturate,
};

fn err_typed<T>(msg: impl Into<String>) -> SourceResult<T> {
    Err(vec![SourceDiagnostic::error(Span::detached(), msg.into())])
}

/// Devolve o valor associado a `field` no tipo `color` (P736).
///
/// `color` é `Value::Type(Type::Color)` no scope global (paridade vanilla
/// — medido: `type(color)` → `type`); os fields resolvem-se por field
/// access em `Value::Type` (`eval/bindings.rs`), que delega aqui.
///
/// **P742 — 17 fields** (paridade do inventário medido P736): 8 constructors
/// (cinco também globais: `rgb`, `luma`, `cmyk`, `oklab`, `oklch`; três somente
/// qualificados: `linear-rgb`, `hsl`, `hsv`) + 9 operadores. Os nomes das
/// funcs são os **nomes plain do vanilla** — medido: `repr(color.rgb)` →
/// `rgb`, `repr(color.lighten)` → `lighten`; `color.rgb == rgb` → `true`
/// (igualdade por nome, `entities/func.rs`). `None` para campo inexistente
/// — o chamador emite "type color does not contain field `<f>`" (verbatim
/// vanilla).
///
/// Histórico: até P736 era `make_color_module() -> Value` (`Value::Dict`
/// com os 6 operadores; os constructors não eram acessíveis via `color.*`).
pub fn color_type_field(field: &str) -> Option<Value> {
    use super::foundations::{
        native_cmyk, native_hsl, native_hsv, native_linear_rgb, native_luma,
        native_oklab, native_oklch, native_rgb,
    };
    if field == "map" {
        return Some(make_color_map_module());
    }
    let function = match field {
        "rgb" => Value::Func(Func::native("rgb", native_rgb)),
        "linear-rgb" => Value::Func(Func::native("linear-rgb", native_linear_rgb)),
        "luma" => Value::Func(Func::native("luma", native_luma)),
        "cmyk" => Value::Func(Func::native("cmyk", native_cmyk)),
        "hsl" => Value::Func(Func::native("hsl", native_hsl)),
        "hsv" => Value::Func(Func::native("hsv", native_hsv)),
        "oklab" => Value::Func(Func::native("oklab", native_oklab)),
        "oklch" => Value::Func(Func::native("oklch", native_oklch)),
        "lighten" => Value::Func(Func::native("lighten", native_color_lighten)),
        "darken" => Value::Func(Func::native("darken", native_color_darken)),
        "mix" => Value::Func(Func::native("mix", native_color_mix)),
        "negate" => Value::Func(Func::native("negate", native_color_negate)),
        "saturate" => Value::Func(Func::native("saturate", native_color_saturate)),
        "desaturate" => Value::Func(Func::native("desaturate", native_color_desaturate)),
        "rotate" => Value::Func(Func::native("rotate", native_color_rotate)),
        "components" => Value::Func(Func::native("components", native_color_components)),
        "space" => Value::Func(Func::native("space", native_color_space)),
        "to-hex" => Value::Func(Func::native("to-hex", native_color_to_hex)),
        "transparentize" => {
            Value::Func(Func::native("transparentize", native_color_transparentize))
        }
        "opacify" => Value::Func(Func::native("opacify", native_color_opacify)),
        // **P1473** — `color.spot` é o tipo-valor `SpotColorant`, paralelo a
        // como `color` é `Value::Type(Type::Color)`. `color.spot(name,
        // fallback)` despacha como construtor via `Value::Type(t) => match
        // t {...}` em `eval/call_dispatch.rs` (mesmo mecanismo de `int`/
        // `str`/`stroke`/etc. — sem novo caminho de dispatch).
        "spot" => Value::Type(Type::SpotColorant),
        _ => return predefined_color(field).map(Value::Color),
    };
    Some(function)
}

const COLOR_MAP_DATA: &str = include_str!("color_maps.hex");

fn make_color_map_module() -> Value {
    let mut scope = Scope::new();
    for line in COLOR_MAP_DATA.lines() {
        let mut fields = line.split_ascii_whitespace();
        let Some(name) = fields.next() else {
            continue;
        };
        let colors = fields
            .map(|token| {
                let rgba = u32::from_str_radix(token, 16)
                    .expect("color_maps.hex is generated from the pinned vanilla source");
                Value::Color(Color::rgba(
                    (rgba >> 24) as u8,
                    (rgba >> 16) as u8,
                    (rgba >> 8) as u8,
                    rgba as u8,
                ))
            })
            .collect();
        scope.define(name, Value::Array(colors));
    }
    Value::Module(Module::new("map", scope))
}

/// **P742** — os 9 métodos de instância de `Value::Color` (despacho P506
/// em `eval/closures.rs`). Método fora desta lista cai no caminho genérico
/// de field access (comportamento pré-P742 preservado).
pub fn is_color_instance_method(method: &str) -> bool {
    matches!(
        method,
        "lighten"
            | "darken"
            | "mix"
            | "negate"
            | "saturate"
            | "desaturate"
            | "rotate"
            | "components"
            | "space"
            | "to-hex"
            | "transparentize"
            | "opacify"
    )
}

#[cfg(test)]
mod atomization_test_support {
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
        args: Args,
    ) -> SourceResult<Value> {
        let world = TestWorld::default();
        let mut ctx = EvalContext::new();
        native(&mut ctx, &args, &world, world.main())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bytes_of(name: &str) -> Option<(u8, u8, u8)> {
        for (n, v) in predefined_color_bindings() {
            if n == name {
                if let Value::Color(c) = v {
                    let (r, g, b, _a) = c.to_srgb();
                    return Some((r, g, b));
                }
            }
        }
        None
    }

    fn has(name: &str) -> bool {
        predefined_color_bindings().iter().any(|(n, _)| n == name)
    }

    #[test]
    fn p687_cores_vanilla_18_srgb_exacto() {
        let esperado: [(&str, (u8, u8, u8)); 18] = [
            ("black", (0x00, 0x00, 0x00)),
            ("gray", (0xAA, 0xAA, 0xAA)),
            ("silver", (0xDD, 0xDD, 0xDD)),
            ("white", (0xFF, 0xFF, 0xFF)),
            ("navy", (0x00, 0x1F, 0x3F)),
            ("blue", (0x00, 0x74, 0xD9)),
            ("aqua", (0x7F, 0xDB, 0xFF)),
            ("teal", (0x39, 0xCC, 0xCC)),
            ("eastern", (0x23, 0x9D, 0xAD)),
            ("purple", (0xB1, 0x0D, 0xC9)),
            ("fuchsia", (0xF0, 0x12, 0xBE)),
            ("maroon", (0x85, 0x14, 0x4B)),
            ("red", (0xFF, 0x41, 0x36)),
            ("orange", (0xFF, 0x85, 0x1B)),
            ("yellow", (0xFF, 0xDC, 0x00)),
            ("olive", (0x3D, 0x99, 0x70)),
            ("green", (0x2E, 0xCC, 0x40)),
            ("lime", (0x01, 0xFF, 0x70)),
        ];
        for (nome, bytes) in esperado {
            assert_eq!(
                bytes_of(nome),
                Some(bytes),
                "cor '{nome}' deve estar ligada com sRGB exacto {bytes:?}"
            );
        }
    }

    #[test]
    fn p687_cores_inexistentes_no_vanilla_ausentes() {
        // Confirmado por sonda: vanilla 0.15.0 não tem `ostrich` nem `pink`.
        assert!(!has("ostrich"));
        assert!(!has("pink"));
    }

    #[test]
    fn p687_extras_pre_p687_sem_regressao() {
        // Mantidos para não regredir documentos/testes anteriores a P687.
        assert_eq!(bytes_of("cyan"), Some((0x00, 0xB3, 0xB3)));
        assert_eq!(bytes_of("magenta"), Some((0xE5, 0x00, 0xE5)));
        assert!(has("none"));
    }
}

/// P1473 (Agente B) — contrato RED de `color.spot` (escopo reduzido: sem
/// `.tint()`, sem variante em `Color`). Fonte:
/// `lab/history/construction/records/00_nucleo/materialization/typst-passo-1473.md` +
/// `00_nucleo/prompts/core/compiler/stdlib/color.md` §P1473 +
/// `00_nucleo/prompts/core/entities/color.md` §P1473 +
/// `00_nucleo/prompts/core/entities/value.md` §P1473. Escrito sem ver a
/// implementação (isolamento Agente A/B).
///
/// `Type::SpotColorant`/`Value::SpotColorant` (ou equivalente) ainda não
/// existem neste baseline. Para que o RED seja "member missing"/runtime
/// (não "compilação partida" — o que impediria confirmar o RED de
/// qualquer outro teste no mesmo crate), **todos** os observáveis abaixo
/// passam exclusivamente por `eval_expression` (avaliação de expressões
/// Typst) e por mecanismos genéricos já existentes sobre `Value`
/// (`Type::name()`, `repr()`, `==`/`!=`) — nunca por construção Rust
/// directa de `SpotColorant`/`Type::SpotColorant`. Isto é deliberado, não
/// uma limitação contornada: `type(color.spot(...))` já devolve
/// `Value::Type(t)` através do `native_type` genérico existente
/// (`Ok(Value::Type(v.type_of()))`, `foundations/ty.rs:28`), e `t.name()`
/// funciona para **qualquer** variante de `Type` sem eu precisar de a
/// nomear — nem sequer depois de o Agente A a acrescentar.
#[cfg(test)]
mod p1473_tests {
    use super::*;
    use crate::compiler::eval::eval_expression;
    use crate::entities::source_result::SourceResult;

    #[derive(Default)]
    struct NullWorld {
        library: crate::entities::world_types::Library,
        book: crate::entities::font_book::FontBook,
    }
    impl crate::contracts::world::World for NullWorld {
        fn library(&self) -> &crate::entities::world_types::Library {
            &self.library
        }
        fn book(&self) -> &crate::entities::font_book::FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            FileId::from_raw(std::num::NonZeroU16::new(1473).unwrap())
        }
        fn source(
            &self,
            _: FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::source::Source>
        {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn file(
            &self,
            _: FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::world_types::Bytes>
        {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<crate::entities::world_types::Font> {
            None
        }
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<crate::entities::world_types::Datetime> {
            None
        }
    }

    fn eval(src: &str) -> SourceResult<Value> {
        eval_expression(&NullWorld::default(), src).0
    }

    fn expect_ok(src: &str) -> Value {
        match eval(src) {
            Ok(v) => v,
            Err(errors) => panic!("{src}: esperava Ok, obteve Err({errors:?})"),
        }
    }

    fn expect_ok_str(src: &str) -> String {
        match expect_ok(src) {
            Value::Str(s) => s.to_string(),
            other => panic!("{src}: esperava Value::Str, obteve {other:?}"),
        }
    }

    fn expect_err_message(src: &str) -> String {
        match eval(src) {
            Err(errors) => {
                assert_eq!(
                    errors.len(),
                    1,
                    "{src}: esperava exactamente 1 diagnóstico, obteve {errors:?}"
                );
                errors[0].message.clone()
            }
            Ok(v) => panic!("{src}: esperava Err, obteve Ok({v:?})"),
        }
    }

    const FALLBACK: &str = "cmyk(0%, 0%, 0%, 0%)";

    #[test]
    fn p1473_color_spot_bare_is_a_type_value() {
        // Medido no L0: type(color.spot) → "type" no vanilla — `color.spot`
        // não chamado é um valor-tipo, tal como `color`/`int`/`str`.
        assert_eq!(expect_ok("type(color.spot) == type"), Value::Bool(true));
    }

    #[test]
    fn p1473_color_spot_constructed_instance_has_spot_type_name() {
        // Medido no L0: type(color.spot("X", cmyk(..))) → "spot" — sem
        // tint ainda não é Color. `t.name()` é genérico: não preciso de
        // nomear `Type::SpotColorant` para o chamar.
        match eval(&format!(r#"type(color.spot("X", {FALLBACK}))"#)) {
            Ok(Value::Type(t)) => assert_eq!(t.name(), "spot"),
            other => panic!("esperava Ok(Value::Type(_)), obteve {other:?}"),
        }
    }

    #[test]
    fn p1473_color_spot_repr_with_name() {
        let fallback_repr = expect_ok_str(&format!("repr({FALLBACK})"));
        let expected = format!(r#"color.spot("X", {fallback_repr})"#);
        assert_eq!(
            expect_ok_str(&format!(r#"repr(color.spot("X", {FALLBACK}))"#)),
            expected
        );
    }

    #[test]
    fn p1473_color_spot_accepts_none_name_and_repr_preserves_it() {
        let fallback_repr = expect_ok_str(&format!("repr({FALLBACK})"));
        let expected = format!("color.spot(none, {fallback_repr})");
        assert_eq!(
            expect_ok_str(&format!("repr(color.spot(none, {FALLBACK}))")),
            expected
        );
    }

    #[test]
    fn p1473_color_spot_equality_by_value() {
        assert_eq!(
            expect_ok(&format!(
                r#"color.spot("X", {FALLBACK}) == color.spot("X", {FALLBACK})"#
            )),
            Value::Bool(true)
        );
        assert_eq!(
            expect_ok(&format!(
                r#"color.spot("X", {FALLBACK}) != color.spot("Y", {FALLBACK})"#
            )),
            Value::Bool(true)
        );
    }

    #[test]
    fn p1473_color_spot_rejects_spot_of_spot_fallback() {
        assert_eq!(
            expect_err_message(&format!(
                r#"color.spot("X", color.spot("Y", {FALLBACK}))"#
            )),
            "expected color, found spot colorant"
        );
    }

    #[test]
    fn p1473_color_spot_name_wrong_type() {
        assert_eq!(
            expect_err_message(&format!("color.spot(5, {FALLBACK})")),
            "expected \"all\", string, or none, found integer"
        );
    }

    #[test]
    fn p1473_color_spot_missing_fallback() {
        assert_eq!(
            expect_err_message(r#"color.spot("X")"#),
            "missing argument: fallback"
        );
    }

    #[test]
    fn p1473_color_parent_keeps_type_kind_and_direct_call() {
        // Não-regressão: `color` continua `type()==type` e os fields
        // pré-existentes (`color.rgb`, etc.) continuam a funcionar.
        assert_eq!(expect_ok("type(color) == type"), Value::Bool(true));
        eval("color.rgb(255, 0, 0)")
            .unwrap_or_else(|e| panic!("color.rgb(..) directo regrediu: {e:?}"));
    }
    #[test]
    fn p1482_tint_is_a_color_with_stable_repr_and_equality() {
        let source = format!(r#"color.spot("Brand", {FALLBACK})"#);
        assert_eq!(
            expect_ok(&format!("type({source}.tint(50%)) == color")),
            Value::Bool(true)
        );
        assert_eq!(
            expect_ok_str(&format!("repr({source}.tint(50%))")),
            format!("{source}.tint(50%)")
        );
        assert_eq!(
            expect_ok(&format!("{source}.tint(50%) == {source}.tint(50%)")),
            Value::Bool(true)
        );
        assert_eq!(
            expect_ok(&format!("{source}.tint(50%) != {source}.tint(25%)")),
            Value::Bool(true)
        );
        assert_eq!(
            expect_ok(&format!("color.spot.tint({source}, 50%) == {source}.tint(50%)")),
            Value::Bool(true)
        );
    }

    #[test]
    fn p1482_tint_bounds_and_errors() {
        let source = format!(r#"color.spot("Brand", {FALLBACK})"#);
        assert_eq!(
            expect_ok_str(&format!("repr({source}.tint(0%))")),
            format!("{source}.tint(0%)")
        );
        assert_eq!(
            expect_ok_str(&format!("repr({source}.tint(100%))")),
            format!("{source}.tint(100%)")
        );
        assert_eq!(
            expect_err_message(&format!("{source}.tint(-1%)")),
            "spot color tint must be positive"
        );
        assert_eq!(
            expect_err_message(&format!("{source}.tint(101%)")),
            "spot color tint must not exceed 100%"
        );
        assert_eq!(
            expect_err_message(&format!("{source}.tint(0.5)")),
            "expected ratio, found float"
        );
    }

    #[test]
    fn p1482_tint_process_projection_and_operations() {
        let source = r##"color.spot("Brand", rgb("#ff4136"))"##.to_string();
        assert_eq!(expect_ok_str(&format!("{source}.tint(0%).to-hex()")), "#ffffff");
        assert_eq!(expect_ok_str(&format!("{source}.tint(100%).to-hex()")), "#ff4136");
        assert_eq!(
            expect_ok_str(&format!("repr({source}.tint(50%).lighten(20%))")),
            format!("{source}.tint(40%)")
        );
        assert_eq!(
            expect_ok_str(&format!("repr({source}.tint(50%).darken(20%))")),
            format!("{source}.tint(60%)")
        );
        assert_eq!(
            expect_ok(&format!("{source}.tint(50%).space() == {source}")),
            Value::Bool(true)
        );
        assert_eq!(
            expect_ok(&format!("{source}.tint(50%).components() == (50%,)")),
            Value::Bool(true)
        );
        assert_eq!(
            expect_ok(&format!(
                "{source}.tint(12.3456789%).components().at(0) == 12.3456789%"
            )),
            Value::Bool(true)
        );
    }

    #[test]
    fn p1482_mix_and_gradient_keep_matching_spot_identity() {
        let spot = r#"color.spot("Brand", red)"#;
        let a = format!("{spot}.tint(50%)");
        let b = format!("{spot}.tint(25%)");
        let repr = expect_ok_str(&format!("repr({spot})"));
        assert_eq!(
            expect_ok_str(&format!("repr({a}.mix({b}))")),
            format!("{repr}.tint(37.5%)")
        );
        assert_eq!(
            expect_ok(&format!("gradient.linear({a}, {b}).space() == {spot}")),
            Value::Bool(true)
        );
        assert_eq!(
            expect_ok_str(&format!("repr(gradient.linear({a}, {b}).sample(50%))")),
            format!("{repr}.tint(37.5%)")
        );
        assert_eq!(
            expect_ok(&format!("gradient.linear({a}, blue).space() == color.oklab")),
            Value::Bool(true)
        );
    }

    #[test]
    fn p1482_rejects_incompatible_spot_space_and_operators() {
        let spot = r#"color.spot("Brand", red)"#;
        let a = format!("{spot}.tint(50%)");
        assert_eq!(
            expect_err_message(&format!("{a}.mix(blue, space: {spot})")),
            "cannot convert process color to spot colorant space"
        );
        assert_eq!(
            expect_err_message(&format!("gradient.linear({a}, blue, space: {spot})")),
            "cannot convert process color to spot colorant space"
        );
        assert_eq!(
            expect_err_message(&format!("{a}.saturate(20%)")),
            "cannot saturate spot color"
        );
        assert_eq!(
            expect_err_message(&format!("{a}.desaturate(20%)")),
            "cannot desaturate spot color"
        );
        assert_eq!(
            expect_err_message(&format!("{a}.transparentize(20%)")),
            "spot colors do not have an alpha component"
        );
        assert_eq!(
            expect_err_message(&format!("{a}.opacify(20%)")),
            "spot colors do not have an alpha component"
        );
        assert_eq!(
            expect_err_message(&format!("{a}.rotate(90deg)")),
            "cannot convert process color to spot colorant space"
        );
        assert_eq!(
            expect_err_message(&format!("{a}.negate(space: rgb)")),
            "cannot convert process color to spot colorant space"
        );
    }

    #[test]
    fn p1482_fallback_cast_and_tint_call_contract() {
        let spot = r##"color.spot("Brand", rgb("#ff4136"))"##;
        let a = format!("{spot}.tint(50%)");
        assert_eq!(expect_ok_str(&format!("rgb({a}).to-hex()")), "#ffa09a");
        assert_eq!(
            expect_err_message(&format!("{spot}.tint(value: 50%)")),
            "the argument `value` is positional"
        );
        assert_eq!(
            expect_err_message(&format!("{spot}.tint()")),
            "missing argument: value"
        );
        assert_eq!(
            expect_err_message("color.spot.tint(red, 50%)"),
            "expected spot colorant, found color"
        );
        assert_eq!(
            expect_err_message(&format!(r#"color.spot("X", {a})"#)),
            "spot color found where process color is required"
        );
    }
}
