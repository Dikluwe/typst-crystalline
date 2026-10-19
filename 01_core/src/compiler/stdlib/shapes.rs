//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/shapes.md
//! @prompt-hash 981e9e63
//! @layer L1
//! @updated 2026-06-24
//!
//! Funções nativas de formas geométricas (rect, square, ellipse, circle, line, polygon).
//! Extraído de `stdlib.rs` no Passo 96.5 conforme ADR-0037.

use std::sync::Arc;

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::corners::Corners;
use crate::entities::elements::curve::{CloseMode, CurvePoint, CurveSegment};
use crate::entities::elements::shape::ShapeElem;
use crate::entities::file_id::FileId;
use crate::entities::geometry::{FillRule, PathItem, ShapeKind, Stroke};
use crate::entities::layout_types::{Color, Length, Point, Pt};
use crate::entities::paint::Paint;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

mod coordinates;
mod curve;
mod ellipse;
mod line;
mod paint;
mod polygon;
mod rect;

use self::coordinates::*;
pub use self::curve::{
    native_curve, native_curve_close, native_curve_cubic, native_curve_line,
    native_curve_move, native_curve_quad,
};
pub use self::ellipse::{native_circle, native_ellipse};
pub use self::line::native_line;
use self::paint::parse_shape_stroke;
pub(super) use self::paint::{parse_color, parse_paint};
pub use self::polygon::{native_polygon, native_polygon_regular};
pub use self::rect::{native_rect, native_square};

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
    use crate::entities::layout_types::{Angle, Length};
    use indexmap::IndexMap;

    #[test]
    fn p1455_parse_color_rejeita_catalogo_string_hardcoded() {
        for name in [
            "red", "green", "blue", "black", "white", "yellow", "cyan", "magenta",
            "orange", "purple", "gray", "grey", "silver", "maroon", "navy", "olive",
            "teal", "lime", "aqua",
        ] {
            assert_eq!(parse_color(&Value::Str(name.into())), None, "string {name}");
        }
        let typed = Color::rgb(1, 2, 3);
        assert_eq!(parse_color(&Value::Color(typed.clone())), Some(typed));
    }

    fn ctx() -> EvalContext {
        EvalContext::new()
    }
    fn tfid() -> FileId {
        FileId::from_raw(std::num::NonZeroU16::new(1).unwrap())
    }

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
            tfid()
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

    // ── P1043: Pares de independência testcase() para shapes.rs:395 e shapes.rs:791 ─

    #[test]
    fn p1043_line_length_only_isolada() {
        // 395 - C1=T, C2=F: args.named contains length && not angle -> length/angle branch
        let mut c = ctx();
        let mut named = IndexMap::default();
        named.insert("length".into(), Value::Length(Length::pt(20.0)));
        let args = Args {
            occurrences: None,
            items: vec![],
            named,
            span: Span::detached(),
        };
        let res = native_line(&mut c, &args, &NullWorld::default(), tfid()).unwrap();
        assert!(matches!(res, Value::Content(Content::Shape { .. })));
    }

    #[test]
    fn p1043_line_angle_only_isolada() {
        // 395 - C1=F, C2=T: args.named contains angle && not length -> length/angle branch
        let mut c = ctx();
        let mut named = IndexMap::default();
        named.insert("angle".into(), Value::Angle(Angle::deg(90.0)));
        let args = Args {
            occurrences: None,
            items: vec![],
            named,
            span: Span::detached(),
        };
        let res = native_line(&mut c, &args, &NullWorld::default(), tfid()).unwrap();
        assert!(matches!(res, Value::Content(Content::Shape { .. })));
    }

    #[test]
    fn p1043_line_default_end_isolada() {
        // 395 - C1=F, C2=F: args.named has neither length nor angle -> default end branch
        let mut c = ctx();
        let args = Args {
            occurrences: None,
            items: vec![],
            named: IndexMap::default(),
            span: Span::detached(),
        };
        let res = native_line(&mut c, &args, &NullWorld::default(), tfid()).unwrap();
        assert!(matches!(res, Value::Content(Content::Shape { .. })));
    }

    #[test]
    fn p1043_curve_array_valid_cmd_isolada() {
        // 791 - C1=T, C2=T: !a.is_empty() && matches!(a[0], Value::Str(_)) -> processa comando
        let mut c = ctx();
        let args = Args::positional(vec![Value::Array(vec![
            Value::Str("move".into()),
            Value::Array(vec![Value::Length(Length::ZERO), Value::Length(Length::ZERO)]),
        ])]);
        let res = native_curve(&mut c, &args, &NullWorld::default(), tfid()).unwrap();
        assert!(matches!(res, Value::Content(Content::Shape { .. })));
    }

    #[test]
    fn p1043_curve_array_empty_isolada() {
        // 791 - C1=F, C2=_: a.is_empty() -> Err(expected content, found array)
        let mut c = ctx();
        let args = Args::positional(vec![Value::Array(vec![])]);
        let res = native_curve(&mut c, &args, &NullWorld::default(), tfid());
        assert!(res.is_err());
    }

    #[test]
    fn p1043_curve_array_non_str_first_isolada() {
        // 791 - C1=T, C2=F: !a.is_empty() && !matches!(a[0], Str) -> Err(expected content, found array)
        let mut c = ctx();
        let args = Args::positional(vec![Value::Array(vec![Value::Int(10)])]);
        let res = native_curve(&mut c, &args, &NullWorld::default(), tfid());
        assert!(res.is_err());
    }
}

/// P1473 (Agente B) — contrato RED de `polygon.regular`. Fonte:
/// `lab/history/construction/records/00_nucleo/materialization/typst-passo-1473.md` +
/// `00_nucleo/prompts/core/compiler/stdlib/shapes.md` §P1473. Escrito sem ver a
/// implementação (isolamento Agente A/B) — usa `eval_expression` (avaliação
/// de expressões Typst) como oráculo de observáveis de língua; a fórmula
/// geométrica é reproduzida aqui em `f64` puro (transcrita byte-a-byte do
/// L0, que por sua vez transcreve `visualize/polygon.rs:53-88` da fonte
/// pinada), não importada da implementação ainda inexistente.
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

    /// Extrai os pontos (`MoveTo`/`LineTo`, em pt) de um `Content::Shape`
    /// com `ShapeKind::Path` — ignora `ClosePath`/`CubicTo` (não usados por
    /// `polygon.regular`). Devolve `None` se o valor não é a forma esperada.
    fn extract_path_points(value: &Value) -> Option<Vec<(f64, f64)>> {
        let Value::Content(Content::Shape(elem)) = value else {
            return None;
        };
        let ShapeKind::Path(items) = &elem.kind else {
            return None;
        };
        Some(
            items
                .iter()
                .filter_map(|item| match item {
                    PathItem::MoveTo(p) | PathItem::LineTo(p) => {
                        Some((p.x.val(), p.y.val()))
                    }
                    _ => None,
                })
                .collect(),
        )
    }

    /// Fórmula pinada (transcrita do L0, que transcreve
    /// `visualize/polygon.rs:53-88` byte-a-byte): `vertices+1` pontos
    /// (`0..=vertices` inclusive fecha a forma repetindo o primeiro no
    /// fim), normalizados pelo mínimo `x`/`y` observado (incluindo o
    /// candidato-semente `radius` do fold do vanilla).
    fn regular_polygon_points(size_pt: f64, vertices: u64) -> Vec<(f64, f64)> {
        let radius = size_pt / 2.0;
        let angle = |v: u64| -> f64 {
            let offset = std::f64::consts::PI * (0.5 - 1.0 / vertices as f64);
            let rotation = 2.0 * std::f64::consts::PI * (v as f64 / vertices as f64);
            offset + rotation
        };
        let raw: Vec<(f64, f64)> = (0..=vertices)
            .map(|v| {
                let a = angle(v);
                (radius * libm::cos(a) + radius, radius * libm::sin(a) + radius)
            })
            .collect();
        let (h_off, v_off) = raw
            .iter()
            .fold((radius, radius), |(mnx, mny), &(x, y)| (mnx.min(x), mny.min(y)));
        raw.iter().map(|&(x, y)| (x - h_off, y - v_off)).collect()
    }

    fn assert_points_close(actual: &[(f64, f64)], expected: &[(f64, f64)]) {
        assert_eq!(actual.len(), expected.len(), "contagem de pontos diverge");
        for (i, (a, e)) in actual.iter().zip(expected.iter()).enumerate() {
            assert!(
                (a.0 - e.0).abs() < 1e-6 && (a.1 - e.1).abs() < 1e-6,
                "vértice {i}: actual={a:?} expected={e:?}"
            );
        }
    }

    #[test]
    fn p1473_polygon_regular_type_is_function() {
        assert_eq!(expect_ok("type(polygon.regular) == function"), Value::Bool(true));
    }

    #[test]
    fn p1473_polygon_regular_defaults_close_the_shape_with_vertices_plus_one_points() {
        // size default 1em (simbólico — sem dimensão de referência em
        // tempo de eval, não comparável a pt aqui); vertices default 3 —
        // o loop 0..=vertices fecha a forma repetindo o primeiro vértice
        // (medido no L0: 4 pares, o último igual ao primeiro).
        let value = expect_ok("polygon.regular()");
        let points = extract_path_points(&value)
            .unwrap_or_else(|| panic!("esperava Content::Shape(Path), obteve {value:?}"));
        assert_eq!(points.len(), 4, "vertices(3)+1 pontos");
        assert_points_close(&[points[0]], &[points[3]]);
    }

    #[test]
    fn p1473_polygon_regular_explicit_args_match_pinned_formula() {
        let value = expect_ok("polygon.regular(size: 100pt, vertices: 5)");
        let points = extract_path_points(&value)
            .unwrap_or_else(|| panic!("esperava Content::Shape(Path), obteve {value:?}"));
        let expected = regular_polygon_points(100.0, 5);
        assert_points_close(&points, &expected);
        assert_points_close(&[points[0]], &[points[5]]);
    }

    #[test]
    fn p1473_polygon_regular_unknown_named() {
        assert_eq!(
            expect_err_message("polygon.regular(foo: 1)"),
            "unexpected argument: foo"
        );
    }

    #[test]
    fn p1473_polygon_regular_unexpected_positional() {
        assert_eq!(expect_err_message("polygon.regular(1)"), "unexpected argument");
    }

    #[test]
    fn p1473_polygon_regular_negative_vertices() {
        assert_eq!(
            expect_err_message("polygon.regular(vertices: -1)"),
            "number must be at least zero"
        );
    }

    #[test]
    fn p1473_polygon_regular_float_vertices() {
        assert_eq!(
            expect_err_message("polygon.regular(vertices: 2.5)"),
            "expected integer, found float"
        );
    }

    #[test]
    fn p1473_polygon_parent_keeps_function_kind_and_direct_call() {
        assert_eq!(expect_ok("type(polygon) == function"), Value::Bool(true));
        eval("polygon((0pt, 0pt), (50pt, 0pt), (25pt, 40pt))")
            .unwrap_or_else(|e| panic!("chamada directa de polygon() regrediu: {e:?}"));
    }
}
