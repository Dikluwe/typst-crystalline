//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/repr.md
//! @prompt-hash 9b235cfa
//! @layer L1
//! @updated 2026-10-08
//!
//! P421 — `repr()` exaustivo para `Value`, `Content` e `Selector`.
//!
//! Implementação como free functions (ADR-0109 forma B). A representação é
//! **reconhecível**, não garante round-trip. Variants complexos usam
//! representação scope-out ("function", "module", nome do tipo).

use crate::entities::content::Content;
use crate::entities::elements::math_attach::MathAttachSlot;
use crate::entities::func::{Func, FuncRepr};
use crate::entities::geometry::Stroke;
use crate::entities::gradient::Gradient;
use crate::entities::layout_types::{
    Align2D, Angle, Color, HAlign, Length, Ratio, VAlign,
};
use crate::entities::paint::Paint;
use crate::entities::rel::Rel;
use crate::entities::selector::Selector;
use crate::entities::tiling::Tiling;
use crate::entities::value::{Type, Value};

mod content;
mod format;
mod math;
mod measure;
mod paint;
mod selector;
mod value;

pub use self::content::repr_content;
use self::format::*;
use self::math::*;
use self::measure::*;
pub(super) use self::paint::repr_stroke;
use self::paint::{repr_align, repr_color, repr_gradient, repr_tiling};
pub use self::selector::repr_selector;
pub use self::value::repr_value;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::content::Content;
    use crate::entities::elements::heading::HeadingElem;
    use crate::entities::label::Label;
    use crate::entities::selector::Selector;
    use crate::entities::value::Value;

    #[test]
    fn p1399_context_block_repr_e_opaca_e_sem_argumentos() {
        use crate::entities::elements::context_block::ContextBlockElem;

        let closure = crate::entities::func::Func::native(
            "p1399-context",
            |_ctx, _args, _world, _file| Ok(Value::Content(Content::text("hidden"))),
        );

        for id in [1, u64::MAX] {
            let contextual = Content::ContextBlock(Arc::new(ContextBlockElem {
                id,
                closure: closure.clone(),
            }));
            assert_eq!(repr_content(&contextual), "context()");
        }
    }

    #[test]
    fn p1339_element_repr_preserves_empty_and_ordered_group() {
        let function = crate::entities::func::Func::native(
            "heading",
            crate::compiler::stdlib::native_heading,
        );
        let render = |fields| {
            repr_value(&Value::Selector(Selector::Element {
                function: function.clone(),
                fields,
            }))
        };
        assert_eq!(render(Default::default()), "heading.where(:)");
        assert_eq!(
            render(
                [("level".into(), Value::Int(1)), ("outlined".into(), Value::Bool(true))]
                    .into_iter()
                    .collect()
            ),
            "heading.where(level: 1, outlined: true)"
        );
        let long = "a".repeat(60);
        let rendered = render(
            [("body".into(), Value::Str(long.clone().into()))]
                .into_iter()
                .collect(),
        );
        assert!(rendered.starts_with("heading.where(\n"));
        assert!(rendered.contains(&long));
    }

    #[test]
    fn p1293_c_html_elem_usa_formatter_canonico_curto_e_multiline() {
        use crate::entities::html::{HtmlAttrs, HtmlBody, HtmlElem};
        use std::sync::Arc;

        let mut short_attrs = HtmlAttrs::default();
        short_attrs.insert("span".into(), "2".into());
        short_attrs.insert("id".into(), "c".into());
        let short = Content::HtmlElem(Arc::new(HtmlElem::new(
            "col".into(),
            Some(short_attrs),
            HtmlBody::Unset,
        )));
        assert_eq!(
            repr_content(&short),
            "elem(tag: \"col\", attrs: (span: \"2\", id: \"c\"))"
        );

        let mut long_attrs = HtmlAttrs::default();
        long_attrs.insert("value".into(), "a rather long attribute value".into());
        long_attrs.insert("aria-label".into(), "another long value".into());
        let long = Content::HtmlElem(Arc::new(HtmlElem::new(
            "button".into(),
            Some(long_attrs),
            HtmlBody::Content(Box::new(Content::text("Body"))),
        )));
        assert_eq!(
            repr_content(&long),
            "elem(\n  tag: \"button\",\n  attrs: (\n    value: \"a rather long attribute value\",\n    aria-label: \"another long value\",\n  ),\n  body: [Body],\n)",
        );
    }
    use std::sync::Arc;

    #[test]
    fn repr_value_primitives() {
        assert_eq!(repr_value(&Value::Int(1)), "1");
        assert_eq!(repr_value(&Value::Float(1.0)), "1.0");
        assert_eq!(repr_value(&Value::Bool(true)), "true");
        assert_eq!(repr_value(&Value::None), "none");
        assert_eq!(repr_value(&Value::Auto), "auto");
        assert_eq!(repr_value(&Value::Str("hello".into())), "\"hello\"");
    }

    #[test]
    fn repr_value_array_and_dict() {
        let arr = Value::Array(vec![Value::Int(1), Value::Int(2)]);
        assert_eq!(repr_value(&arr), "(1, 2)");

        let mut dict = indexmap::IndexMap::default();
        dict.insert("a".into(), Value::Int(1));
        assert_eq!(repr_value(&Value::Dict(dict)), "(a: 1)");
    }

    #[test]
    fn repr_value_empty_array_e_dict_distinguem() {
        // P695 — `()` (array vazio) e `(:)` (dict vazio) não podem colidir.
        let empty_arr = Value::Array(vec![]);
        let empty_dict = Value::Dict(indexmap::IndexMap::default());
        assert_eq!(repr_value(&empty_arr), "()");
        assert_eq!(repr_value(&empty_dict), "(:)");
        assert_ne!(repr_value(&empty_arr), repr_value(&empty_dict));
    }

    #[test]
    fn repr_value_array_um_elemento_virgula_final() {
        // P801 — array de exactamente 1 elemento leva vírgula final
        // (paridade vanilla `pretty_array_like(_, len == 1)`), distinguindo-o
        // de parênteses de agrupamento. Controlos: 0 e 2+ elementos sem
        // vírgula final.
        let single = Value::Array(vec![Value::Int(5)]);
        assert_eq!(repr_value(&single), "(5,)");
        let nested = Value::Array(vec![Value::Array(vec![Value::Int(5)])]);
        assert_eq!(repr_value(&nested), "((5,),)");
        let empty = Value::Array(vec![]);
        assert_eq!(repr_value(&empty), "()");
        let two = Value::Array(vec![Value::Int(1), Value::Int(2)]);
        assert_eq!(repr_value(&two), "(1, 2)");
    }

    #[test]
    fn repr_value_selector() {
        let sel = Value::Selector(Selector::Kind(
            crate::entities::element_kind::ElementKind::Heading,
        ));
        assert_eq!(repr_value(&sel), "heading");
    }

    #[test]
    fn repr_value_complex_types() {
        use crate::entities::{
            bytes::Bytes,
            decimal::Decimal,
            duration::Duration,
            gradient::Gradient,
            layout_types::{Angle, Color, Length, Ratio},
            location::Location,
            regex::Regex,
            version::Version,
        };

        let r = repr_value(&Value::Length(Length::pt(12.0)));
        assert_eq!(r, "12pt", "Length repr: {r}");
        let r = repr_value(&Value::Ratio(Ratio::from_percent(50.0)));
        assert_eq!(r, "50%", "Ratio repr: {r}");
        let r = repr_value(&Value::Angle(Angle::deg(90.0)));
        assert_eq!(r, "90deg", "Angle repr: {r}");
        let r = repr_value(&Value::Color(Color::rgb(255, 0, 0)));
        assert_eq!(r, "rgb(\"#ff0000\")", "Color repr: {r}");
        assert_eq!(repr_value(&Value::Fraction(0.5)), "0.5fr");
        assert_eq!(repr_value(&Value::Location(Location::from_raw(42))), "location(..)");
        assert_eq!(
            repr_value(&Value::Gradient(Gradient::Linear(std::sync::Arc::new(
                crate::entities::gradient::Linear {
                    angle: Angle::deg(0.0),
                    stops: std::sync::Arc::from([]),
                    space: crate::entities::layout_types::ColorSpace::Oklab,
                    relative: None,
                    anti_alias: true,
                }
            )))),
            "gradient.linear()"
        );
        assert_eq!(
            repr_value(&Value::Regex(Regex::new("\\d+").unwrap())),
            "regex(\"\\d+\")"
        );
        assert_eq!(
            repr_value(&Value::Tiling(std::sync::Arc::new(
                crate::entities::tiling::Tiling::new(
                    crate::entities::tiling::TilingBody::Color(Color::rgb(0, 0, 0))
                )
            ))),
            "tiling(...)"
        );
        assert_eq!(repr_value(&Value::Bytes(Bytes::from(vec![0u8, 1, 2]))), "bytes(3)");
        assert_eq!(
            repr_value(&Value::Decimal(Decimal::from_i64(123))),
            "decimal(\"123\")"
        );
        assert_eq!(
            repr_value(&Value::Duration(Duration::from_nanos(1_000_000_000))),
            "duration(seconds: 1)"
        );
        assert_eq!(
            repr_value(&Value::Version(std::sync::Arc::new(Version::new(1, 2, 3)))),
            "version(1, 2, 3)"
        );
    }

    #[test]
    fn repr_value_float_with_decimal() {
        assert_eq!(repr_value(&Value::Float(1.5)), "1.5");
    }

    #[test]
    fn repr_value_module() {
        use crate::entities::{module::Module, scope::Scope};
        let m = Module::new("mylib", Scope::new());
        assert_eq!(repr_value(&Value::Module(m)), "<module mylib>");
    }

    #[test]
    fn repr_value_func_named() {
        // **P742** — paridade vanilla medida: `repr(rgb)` → `rgb` (sem `#`).
        let f =
            crate::entities::func::Func::native("repr", |_ctx, _args, _world, _cf| {
                Ok(Value::None)
            });
        assert_eq!(repr_value(&Value::Func(f)), "repr");
    }

    #[test]
    fn repr_value_func_unnamed() {
        let f = crate::entities::func::Func::native("", |_ctx, _args, _world, _cf| {
            Ok(Value::None)
        });
        assert_eq!(repr_value(&Value::Func(f)), "#function(...)");
    }

    #[test]
    fn p744_repr_closure() {
        use crate::entities::func::{ClosureParam, ClosureRepr};
        use crate::entities::scope::{Capturer, Scope};
        use crate::entities::source::Source;
        let source = Source::detached("x + 1");
        let body = source.root().clone();
        let f = crate::entities::func::Func::closure(ClosureRepr {
            name: Some("f".into()),
            params: vec![ClosureParam { name: "x".into(), default: None, pattern: None }],
            sink_name: None,
            body,
            captured: std::sync::Arc::new(Scope::new()),
            capturer: Capturer::Function,
        });
        assert_eq!(repr_value(&Value::Func(f)), "(..) => ..");
    }

    #[test]
    fn repr_value_version() {
        use crate::entities::version::Version;
        let v = Value::Version(std::sync::Arc::new(Version::new(0, 11, 0)));
        assert_eq!(repr_value(&v), "version(0, 11, 0)");
    }

    #[test]
    fn repr_value_bytes() {
        use crate::entities::bytes::Bytes;
        let b = Value::Bytes(Bytes::from(vec![0u8; 10]));
        assert_eq!(repr_value(&b), "bytes(10)");
    }

    #[test]
    fn repr_value_datetime_date_only() {
        use crate::entities::world_types::Datetime;
        let dt = Value::Datetime(Datetime::new_date(2026, 6, 25).unwrap());
        assert_eq!(repr_value(&dt), "datetime(year: 2026, month: 6, day: 25)");
    }

    #[test]
    fn repr_value_datetime_with_time() {
        use crate::entities::world_types::Datetime;
        let dt = Value::Datetime(Datetime::new_datetime(2026, 6, 25, 14, 30, 0).unwrap());
        // 62 colunas > 50 → modo vertical (paridade vanilla
        // `pretty_comma_list`), medido em `temp/p843/f5_datetime.typ`.
        assert_eq!(
            repr_value(&dt),
            "datetime(\n  year: 2026,\n  month: 6,\n  day: 25,\n  hour: 14,\n  minute: 30,\n  second: 0,\n)"
        );
    }

    #[test]
    fn repr_value_duration() {
        use crate::entities::duration::Duration;
        let d = Value::Duration(Duration::from_seconds(3661));
        assert_eq!(repr_value(&d), "duration(hours: 1, minutes: 1, seconds: 1)");
    }

    #[test]
    fn repr_value_regex() {
        use crate::entities::regex::Regex;
        let r = Value::Regex(Regex::new("a+").unwrap());
        assert_eq!(repr_value(&r), "regex(\"a+\")");
    }

    #[test]
    fn repr_value_relative_conserva_abs_zero() {
        use crate::entities::layout_types::Length;
        use crate::entities::rel::Rel;
        let r = Value::Relative(Rel::<Length>::from_percent(50.0));
        assert_eq!(repr_value(&r), "50% + 0pt");
    }

    #[test]
    fn repr_value_relative_with_abs_offset() {
        use crate::entities::layout_types::Length;
        use crate::entities::rel::Rel;
        let r = Value::Relative(Rel::<Length>::from_percent(50.0) + Length::cm(2.0));
        // 2cm = 56.692pt → arredondado a 2 decimais (paridade vanilla).
        assert_eq!(repr_value(&r), "50% + 56.69pt");
    }

    // ── P721 — repr Typst (não Debug do Rust) ──────────────────────────────

    #[test]
    fn p721_repr_length_pt_em_e_combinado() {
        use crate::entities::layout_types::Length;
        assert_eq!(repr_value(&Value::Length(Length::pt(6.0))), "6pt");
        assert_eq!(repr_value(&Value::Length(Length::pt(6.5))), "6.5pt");
        assert_eq!(repr_value(&Value::Length(Length::pt(-3.0))), "-3pt");
        assert_eq!(repr_value(&Value::Length(Length::pt(0.1))), "0.1pt");
        assert_eq!(repr_value(&Value::Length(Length::em(2.0))), "2em");
        assert_eq!(
            repr_value(&Value::Length(Length::pt(2.0) + Length::em(1.0))),
            "2pt + 1em"
        );
        assert_eq!(repr_value(&Value::Length(Length::ZERO)), "0pt");
    }

    #[test]
    fn p721_repr_ratio_arredonda_2_decimais() {
        use crate::entities::layout_types::Ratio;
        assert_eq!(repr_value(&Value::Ratio(Ratio::from_percent(33.333))), "33.33%");
        assert_eq!(repr_value(&Value::Ratio(Ratio::from_percent(0.5))), "0.5%");
    }

    #[test]
    fn p721_repr_angle_sempre_graus() {
        use crate::entities::layout_types::Angle;
        assert_eq!(repr_value(&Value::Angle(Angle::deg(45.0))), "45deg");
        assert_eq!(repr_value(&Value::Angle(Angle::deg(45.5))), "45.5deg");
        assert_eq!(repr_value(&Value::Angle(Angle::rad(1.0))), "57.3deg");
    }

    #[test]
    fn p721_repr_fraction_com_unidade_fr() {
        assert_eq!(repr_value(&Value::Fraction(1.0)), "1fr");
        assert_eq!(repr_value(&Value::Fraction(2.5)), "2.5fr");
    }

    #[test]
    fn p721_repr_color_srgb_hex_com_e_sem_alpha() {
        use crate::entities::layout_types::Color;
        assert_eq!(repr_value(&Value::Color(Color::rgb(255, 0, 0))), "rgb(\"#ff0000\")");
        assert_eq!(
            repr_value(&Value::Color(Color::rgba(255, 0, 0, 128))),
            "rgb(\"#ff000080\")"
        );
    }

    #[test]
    fn p721_repr_color_espacos_nao_srgb() {
        use crate::entities::layout_types::Color;
        assert_eq!(repr_value(&Value::Color(Color::luma(0.5))), "luma(50%)");
        assert_eq!(
            repr_value(&Value::Color(Color::linear_rgb(0.5, 0.5, 0.5, 1.0))),
            "color.linear-rgb(50%, 50%, 50%)"
        );
        assert_eq!(
            repr_value(&Value::Color(Color::cmyk(0.0, 0.5, 1.0, 0.0))),
            "cmyk(0%, 50%, 100%, 0%)"
        );
        assert_eq!(
            repr_value(&Value::Color(Color::oklab(0.5, 0.1, 0.1, 1.0))),
            "oklab(50%, 0.1, 0.1)"
        );
        assert_eq!(
            repr_value(&Value::Color(Color::oklch(0.5, 0.1, 30.0, 1.0))),
            "oklch(50%, 0.1, 30deg)"
        );
        assert_eq!(
            repr_value(&Value::Color(Color::hsl(0.0, 1.0, 0.5, 1.0))),
            "color.hsl(0deg, 100%, 50%)"
        );
        assert_eq!(
            repr_value(&Value::Color(Color::hsv(120.0, 0.5, 0.8, 1.0))),
            "color.hsv(120deg, 50%, 80%)"
        );
    }

    #[test]
    fn p721_repr_color_alpha_omitido_quando_1() {
        use crate::entities::layout_types::Color;
        // Alpha != 1.0 aparece como argumento extra (paridade vanilla).
        assert_eq!(
            repr_value(&Value::Color(Color::Luma { l: 0.5, a: 0.5 })),
            "luma(50%, 50%)"
        );
        assert_eq!(
            repr_value(&Value::Color(Color::oklab(0.5, 0.1, 0.1, 0.5))),
            "oklab(50%, 0.1, 0.1, 50%)"
        );
    }

    #[test]
    fn p721_repr_stroke_thickness_mais_paint() {
        use crate::entities::geometry::{Stroke, StrokeFields};
        use crate::entities::layout_types::Color;
        use crate::entities::paint::Paint;
        // `stroke(paint: red, thickness: 2pt)` — red Typst = #ff4136.
        let s = Stroke {
            paint: Paint::Solid(Color::rgb(255, 65, 54)),
            thickness: 2.0,
            specified: StrokeFields {
                paint: true,
                thickness: true,
                ..StrokeFields::default()
            },
            overhang: true,
            ..Stroke::default()
        };
        assert_eq!(repr_value(&Value::from(s)), "2pt + rgb(\"#ff4136\")");
    }

    #[test]
    fn p1490_repr_stroke_preserva_quatro_estados_morfologicos() {
        use crate::entities::geometry::{Stroke, StrokeFields};
        use crate::entities::layout_types::Color;
        use crate::entities::paint::Paint;

        let automatic = Stroke::default();
        let thickness = Stroke {
            specified: StrokeFields { thickness: true, ..StrokeFields::default() },
            ..Stroke::default()
        };
        let paint = Stroke {
            paint: Paint::Solid(Color::luma(0.0)),
            specified: StrokeFields { paint: true, ..StrokeFields::default() },
            ..Stroke::default()
        };
        let both = Stroke {
            paint: Paint::Solid(Color::luma(0.0)),
            specified: StrokeFields {
                paint: true,
                thickness: true,
                ..StrokeFields::default()
            },
            ..Stroke::default()
        };

        assert_eq!(repr_value(&Value::from(automatic.clone())), "stroke()");
        assert_eq!(repr_value(&Value::from(thickness.clone())), "1pt");
        assert_eq!(repr_value(&Value::from(paint.clone())), "luma(0%)");
        assert_eq!(repr_value(&Value::from(both.clone())), "1pt + luma(0%)");
        assert_eq!(
            repr_value(&Value::Array(vec![
                Value::from(automatic),
                Value::from(thickness),
                Value::from(paint),
                Value::from(both),
            ])),
            "(stroke(), 1pt, luma(0%), 1pt + luma(0%))"
        );
    }

    #[test]
    fn p721_repr_align_horizontal_primeiro() {
        use crate::entities::layout_types::{Align2D, HAlign, VAlign};
        let left = Align2D { h: Some(HAlign::Left), v: None };
        assert_eq!(repr_value(&Value::Align(left)), "left");
        let ch = Align2D { h: Some(HAlign::Center), v: Some(VAlign::Horizon) };
        assert_eq!(repr_value(&Value::Align(ch)), "center + horizon");
        // `top + right` → "right + top" (horizontal primeiro, paridade vanilla).
        let tr = Align2D { h: Some(HAlign::Right), v: Some(VAlign::Top) };
        assert_eq!(repr_value(&Value::Align(tr)), "right + top");
        let start = Align2D { h: Some(HAlign::Start), v: None };
        assert_eq!(repr_value(&Value::Align(start)), "start");
    }

    #[test]
    fn repr_value_content_heading() {
        let h = Content::Heading(Arc::new(HeadingElem::new(1, Content::text("Title"))));
        assert_eq!(repr_value(&Value::Content(h)), "heading(level: 1)[[Title]]");
    }

    #[test]
    fn repr_value_content_label() {
        use crate::entities::elements::label::LabelElem;
        let l = Content::Label(Arc::new(LabelElem {
            name: "sec1".into(),
            body: Content::text("Section"),
            auto: false,
        }));
        assert_eq!(repr_content(&l), "label(\"sec1\", [Section])");
    }

    #[test]
    fn p11402_repr_value_label_literal_ou_constructor() {
        use crate::entities::label::Label;

        assert_eq!(repr_value(&Value::Label(Label("sec1".into()))), "<sec1>");
        assert_eq!(repr_value(&Value::Label(Label("a b".into()))), "label(\"a b\")");
    }

    #[test]
    fn repr_value_mixed_array() {
        let arr = Value::Array(vec![Value::Int(1), Value::Str("a".into()), Value::None]);
        assert_eq!(repr_value(&arr), "(1, \"a\", none)");
    }

    #[test]
    fn repr_content_text_and_sequence() {
        let seq =
            Content::Sequence(Arc::from(vec![Content::text("hello"), Content::Space]));
        assert_eq!(repr_content(&seq), "sequence([hello], [ ])");
    }

    #[test]
    fn p1140_9_repr_content_spacing_e_breaks_canonicos() {
        use crate::entities::layout_types::Abs;
        use crate::entities::parity::Parity;

        let combined = Length { abs: Abs::pt(2.0), em: 1.5 };
        assert_eq!(
            repr_content(&Content::h_space_with_weak_presence(combined, false, false,)),
            "h(amount: 2pt + 1.5em)"
        );
        assert_eq!(
            repr_content(&Content::h_space_fraction_with_weak_presence(
                1.25, false, true,
            )),
            "h(amount: 1.25fr, weak: false)"
        );
        assert_eq!(
            repr_content(&Content::v_space_with_weak_presence(
                Length::em(2.0),
                true,
                true,
            )),
            "v(amount: 2em, weak: true)"
        );
        assert_eq!(
            repr_content(&Content::pagebreak_with_weak_presence(
                false,
                false,
                Some(Parity::Odd),
            )),
            "pagebreak(to: \"odd\")"
        );
        assert_eq!(
            repr_content(&Content::pagebreak_with_weak_presence(
                false,
                true,
                Some(Parity::Even),
            )),
            "pagebreak(weak: false, to: \"even\")"
        );
        assert_eq!(
            repr_content(&Content::colbreak_with_weak_presence(false, false)),
            "colbreak()"
        );
    }

    #[test]
    fn repr_content_heading() {
        let h = Content::Heading(Arc::new(HeadingElem {
            level: 2,
            body: Content::text("Title"),
            outlined: true,
            bookmarked: None,
            set_fields: 0,
        }));
        assert_eq!(repr_content(&h), "heading(level: 2)[[Title]]");
    }

    #[test]
    fn repr_content_cite_and_bibliography() {
        let c = Content::Cite(Arc::new(crate::entities::elements::cite::CiteElem {
            key: "key".into(),
            form: None,
            supplement: None,
            style: None,
        }));
        assert_eq!(repr_content(&c), "cite(<key>)");
    }

    #[test]
    fn repr_selector_variants() {
        assert_eq!(
            repr_selector(&Selector::Kind(
                crate::entities::element_kind::ElementKind::Heading
            )),
            "heading"
        );
        assert_eq!(repr_selector(&Selector::Label(Label("l".to_string()))), "<l>");
        assert_eq!(
            repr_selector(&Selector::Where {
                base: Box::new(Selector::Kind(
                    crate::entities::element_kind::ElementKind::Heading
                )),
                field: "level".into(),
                value: Box::new(Value::Int(1)),
            }),
            "heading.where(level: 1)"
        );
    }

    // ── P843 (F1) — repr(duration): formato nomeado do vanilla ─────────────
    //
    // Medido no vanilla (`temp/p843/f1_duration.typ`,
    // `lab/typst-original` release, 2026-07-22): só os componentes não-zero,
    // por ordem weeks→seconds; sub-segundo truncado; zero → `duration()`.

    #[test]
    fn p843_f1_repr_duration_segundos_simples() {
        use crate::entities::duration::Duration;
        assert_eq!(
            repr_value(&Value::Duration(Duration::from_seconds(3))),
            "duration(seconds: 3)"
        );
    }

    #[test]
    fn p843_f1_repr_duration_composta_horas_minutos_segundos() {
        use crate::entities::duration::Duration;
        assert_eq!(
            repr_value(&Value::Duration(Duration::from_seconds(3661))),
            "duration(hours: 1, minutes: 1, seconds: 1)"
        );
    }

    #[test]
    fn p843_f1_repr_duration_dias_horas_minutos_segundos() {
        use crate::entities::duration::Duration;
        let d = Duration::from_nanos(
            Duration::from_days(1).nanos
                + Duration::from_hours(2).nanos
                + Duration::from_minutes(3).nanos
                + Duration::from_seconds(4).nanos,
        );
        assert_eq!(
            repr_value(&Value::Duration(d)),
            "duration(days: 1, hours: 2, minutes: 3, seconds: 4)"
        );
    }

    #[test]
    fn p843_f1_repr_duration_minutos_normalizam_para_horas() {
        use crate::entities::duration::Duration;
        assert_eq!(
            repr_value(&Value::Duration(Duration::from_minutes(90))),
            "duration(hours: 1, minutes: 30)"
        );
    }

    #[test]
    fn p843_f1_repr_duration_semanas_e_dias() {
        use crate::entities::duration::Duration;
        assert_eq!(
            repr_value(&Value::Duration(Duration::from_days(8))),
            "duration(weeks: 1, days: 1)"
        );
    }

    #[test]
    fn p843_f1_repr_duration_zero_e_subsegundo() {
        use crate::entities::duration::Duration;
        assert_eq!(repr_value(&Value::Duration(Duration::ZERO)), "duration()");
        // Medido vanilla: `repr(duration(seconds: 1) / 3)` → "duration()"
        // (componentes inteiros; sub-segundo não aparece).
        assert_eq!(
            repr_value(&Value::Duration(Duration::from_nanos(500_000_000))),
            "duration()"
        );
    }

    #[test]
    fn p850_repr_duration_negative() {
        use crate::entities::duration::Duration;
        assert_eq!(
            repr_value(&Value::Duration(-Duration::from_seconds(3))),
            "duration(seconds: -3)"
        );
        assert_eq!(
            repr_value(&Value::Duration(Duration::from_nanos(-2_000_000_000))),
            "duration(seconds: -2)"
        );
        assert_eq!(
            repr_value(&Value::Duration(Duration::from_nanos(
                -86_400_000_000_000 - 3_600_000_000_000
            ))),
            "duration(days: -1, hours: -1)"
        );
    }

    // ── P843 (F2) — repr de content: formato estrutural do vanilla ─────────
    //
    // Medido no vanilla (`temp/p843/f2_content.typ`, `f2_nest.typ`,
    // `f2_misc.typ`): Text → `[texto]` (cru), Space → `[ ]`, vazio → `[]`,
    // sequência → `sequence(...)` com `pretty_array_like` (horizontal até 50
    // colunas, vertical com vírgula final acima disso), Strong/Emph →
    // `strong(body: ...)` / `emph(body: ...)`.

    #[test]
    fn p843_f2_repr_content_texto_cru_sem_aspas() {
        assert_eq!(repr_content(&Content::text("hi")), "[hi]");
    }

    #[test]
    fn p843_f2_repr_content_vazio_e_sequencia_vazia() {
        assert_eq!(repr_content(&Content::Empty), "[]");
        let seq = Content::Sequence(Arc::from(Vec::new()));
        assert_eq!(repr_content(&seq), "[]");
    }

    #[test]
    fn p843_f2_repr_content_sequencia_com_strong() {
        use crate::entities::elements::strong::StrongElem;
        let seq = Content::Sequence(Arc::from(vec![
            Content::text("hi"),
            Content::Space,
            Content::Strong(Arc::new(StrongElem::new(Content::text("bold")))),
        ]));
        assert_eq!(repr_content(&seq), "sequence([hi], [ ], strong(body: [bold]))");
    }

    #[test]
    fn p843_f2_repr_content_aninhamento_dois_niveis() {
        use crate::entities::elements::emph::EmphElem;
        use crate::entities::elements::strong::StrongElem;
        // strong(body: emph(body: [it])) — recursão no campo body.
        let c = Content::Strong(Arc::new(StrongElem::new(Content::Emph(Arc::new(
            EmphElem::new(Content::text("it")),
        )))));
        assert_eq!(repr_content(&c), "strong(body: emph(body: [it]))");
    }

    #[test]
    fn p843_f2_repr_content_sequencia_longa_quebra_vertical() {
        use crate::entities::elements::emph::EmphElem;
        use crate::entities::elements::strong::StrongElem;
        // 59 colunas > 50 → modo vertical (paridade vanilla
        // `pretty_comma_list`, MAX_WIDTH = 50): uma peça por linha,
        // indentação de 2 espaços, vírgula final.
        let seq = Content::Sequence(Arc::from(vec![
            Content::text("a"),
            Content::Space,
            Content::Strong(Arc::new(StrongElem::new(Content::Sequence(Arc::from(
                vec![
                    Content::text("b"),
                    Content::Space,
                    Content::Emph(Arc::new(EmphElem::new(Content::text("c")))),
                ],
            ))))),
        ]));
        let esperado = "sequence(\n  [a],\n  [ ],\n  strong(body: sequence([b], [ ], emph(body: [c]))),\n)";
        assert_eq!(repr_content(&seq), esperado);
    }

    // ── P843 (F3) — repr de type(none)/type(auto) ──────────────────────────
    //
    // Medido no vanilla (`temp/p843/f3_type_none_auto.typ`):
    // `repr(type(none))` → "type(none)", `repr(type(auto))` → "type(auto)";
    // os restantes tipos mantêm o nome curto (`repr(type(1))` → "int").

    #[test]
    fn p843_f3_repr_type_none_e_auto() {
        use crate::entities::value::Type;
        assert_eq!(repr_value(&Value::Type(Type::None)), "type(none)");
        assert_eq!(repr_value(&Value::Type(Type::Auto)), "type(auto)");
    }

    #[test]
    fn p843_f3_repr_type_restantes_nome_curto() {
        use crate::entities::value::Type;
        assert_eq!(repr_value(&Value::Type(Type::Int)), "int");
        assert_eq!(repr_value(&Value::Type(Type::Str)), "str");
        assert_eq!(repr_value(&Value::Type(Type::Bytes)), "bytes");
    }

    #[test]
    fn p1290_array_vazio_singleton_e_curto() {
        assert_eq!(repr_value(&Value::Array(vec![])), "()");
        assert_eq!(repr_value(&Value::Array(vec![Value::Int(0)])), "(0,)");
        assert_eq!(
            repr_value(&Value::Array(vec![Value::Int(0), Value::Int(1), Value::Int(2)])),
            "(0, 1, 2)"
        );
    }

    #[test]
    fn p1290_array_fronteira_ascii_49_50_51() {
        let array_with_text = |count: usize| {
            repr_value(&Value::Array(vec![Value::Str("x".repeat(count).into())]))
        };

        assert_eq!(
            array_with_text(47),
            "(\"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\",)"
        );
        assert_eq!(
            array_with_text(48),
            "(\"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\",)"
        );
        assert_eq!(
            array_with_text(49),
            "(\n  \"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\",\n)"
        );
    }

    #[test]
    fn p1290_array_item_multiline_reindenta_recursivamente() {
        let inner = Value::Array(vec![Value::Str("x".repeat(49).into())]);
        let outer = Value::Array(vec![inner]);

        assert_eq!(
            repr_value(&outer),
            "(\n  (\n    \"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\",\n  ),\n)"
        );
    }

    #[test]
    fn p1290_math_op_estrutura_geral_preserva_limits_false_e_true() {
        let false_op = Content::math_op(Content::text("tensor-op"), false);
        let true_op = Content::math_op(Content::text("custom-limit"), true);

        assert_eq!(repr_content(&false_op), "op(text: [tensor-op], limits: false)");
        assert_eq!(repr_content(&true_op), "op(text: [custom-limit], limits: true)");
    }

    #[test]
    fn p1290_math_op_campos_em_ordem_e_conteudo_markup_multiline() {
        use crate::entities::elements::strong::StrongElem;

        let markup = Content::Sequence(Arc::from(vec![
            Content::Strong(Arc::new(StrongElem::new(Content::text("f")))),
            Content::Space,
            Content::text("oo"),
        ]));
        let op = Content::math_op(markup, false);

        assert_eq!(
            repr_content(&op),
            "op(\n  text: sequence(strong(body: [f]), [ ], [oo]),\n  limits: false,\n)"
        );
    }

    #[test]
    fn p1290_math_op_conteudo_preserva_aspas_e_barra_invertida() {
        let op = Content::math_op(Content::text(r#"a "quote" \ slash"#), false);

        assert_eq!(repr_content(&op), r#"op(text: [a "quote" \ slash], limits: false)"#);
    }

    #[test]
    fn p1290_math_op_conteudo_escapado_estrutural() {
        let escaped = Content::Sequence(Arc::from(vec![
            Content::text("a"),
            Content::Space,
            Content::text("#"),
            Content::Space,
            Content::text("*"),
        ]));
        let op = Content::math_op(escaped, false);

        assert_eq!(
            repr_content(&op),
            "op(\n  text: sequence([a], [ ], [#], [ ], [*]),\n  limits: false,\n)"
        );
    }

    #[test]
    fn p1292_c_v7_math_vec_projeta_folhas_math_diretas_como_content() {
        let vec = Content::math_vec(vec![
            Content::MathIdent("a".into()),
            Content::MathText("23".into()),
            Content::MathText("α".into()),
        ]);

        assert_eq!(repr_content(&vec), "vec(children: ([a], [23], [α]))");
    }

    #[test]
    fn p1292_c_v7_math_vec_nao_recursa_nem_duplica_markup() {
        let structured = Content::math_frac(
            Content::MathIdent("a".into()),
            Content::MathText("1".into()),
        );
        let vec = Content::math_vec(vec![
            structured,
            Content::strong(Content::text("b")),
            Content::text("quoted"),
        ]);

        assert_eq!(
            repr_content(&vec),
            "vec(\n  children: (\n    frac(num: [a], denom: [1]),\n    strong(body: [b]),\n    [quoted],\n  ),\n)"
        );
    }

    #[test]
    fn p1292_d_flush_repr_is_the_zero_field_constructor() {
        assert_eq!(repr_content(&Content::flush()), "flush()");
    }

    #[test]
    fn p1293_b_attach_repr_preserva_slots_ordem_e_none_explicito() {
        let attach = Content::math_attach(
            Content::text("x"),
            MathAttachSlot::ExplicitNone,
            MathAttachSlot::Present(Content::text("b")),
            MathAttachSlot::Present(Content::text("tl")),
            MathAttachSlot::Present(Content::text("bl")),
            MathAttachSlot::Present(Content::text("tr")),
            MathAttachSlot::Present(Content::text("br")),
        );
        assert_eq!(
            repr_content(&attach),
            "attach(\n  base: [x],\n  t: none,\n  b: [b],\n  tl: [tl],\n  bl: [bl],\n  tr: [tr],\n  br: [br],\n)"
        );

        let omitted = Content::math_attach(
            Content::text("x"),
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
        );
        assert_eq!(repr_content(&omitted), "attach(base: [x])");
    }

    #[test]
    fn p1293_b_binom_repr_preserva_singleton_e_lower_variadico() {
        let singleton = Content::math_delimited(
            '(',
            Content::math_frac_unlined(
                Content::text("n"),
                Content::MathSequence(Arc::from(vec![Content::text("k")])),
            ),
            ')',
        );
        assert_eq!(repr_content(&singleton), "binom(upper: [n], lower: ([k],))");

        let multiple = Content::math_delimited(
            '(',
            Content::math_frac_unlined(
                Content::text("n"),
                Content::MathSequence(Arc::from(vec![
                    Content::text("k"),
                    Content::MathText(", ".into()),
                    Content::text("j"),
                    Content::MathText(", ".into()),
                    Content::text("m"),
                ])),
            ),
            ')',
        );
        assert_eq!(repr_content(&multiple), "binom(upper: [n], lower: ([k], [j], [m]))");
    }

    #[test]
    fn p1293_b_independent_red_projeta_folhas_e_preserva_styled() {
        let attach = Content::math_attach(
            Content::MathIdent("x".into()),
            MathAttachSlot::Present(Content::MathIdent("T".into())),
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
        );
        assert_eq!(repr_content(&attach), "attach(base: [x], t: [T])");

        let binom = Content::math_delimited(
            '(',
            Content::math_frac_unlined(
                Content::MathIdent("n".into()),
                Content::MathSequence(Arc::from(vec![
                    Content::MathIdent("k".into()),
                    Content::MathText(", ".into()),
                    Content::MathIdent("j".into()),
                ])),
            ),
            ')',
        );
        assert_eq!(repr_content(&binom), "binom(upper: [n], lower: ([k], [j]))");

        let mono = Content::math_styled(
            Some(crate::entities::math_style::MathStyleKind::Monospace),
            None,
            None,
            Content::MathIdent("x".into()),
            None,
        );
        let script = Content::math_styled(
            Some(crate::entities::math_style::MathStyleKind::Script),
            None,
            None,
            Content::MathIdent("x".into()),
            Some(true),
        );
        assert_eq!(repr_content(&mono), "styled(child: [x], ..)");
        assert_eq!(repr_content(&script), "styled(child: [x], ..)");
    }

    #[test]
    fn p1397_class_e_todos_os_eixos_styled_preservam_morfologia() {
        use crate::entities::math_class::MathClass;
        use crate::entities::math_style::MathStyleKind;

        let styled = [
            Content::math_styled(
                Some(MathStyleKind::DoubleStruck),
                None,
                None,
                Content::MathIdent("x".into()),
                None,
            ),
            Content::math_styled(
                None,
                Some(true),
                None,
                Content::MathIdent("x".into()),
                None,
            ),
            Content::math_styled(
                None,
                None,
                Some(false),
                Content::MathText("d".into()),
                None,
            ),
            Content::math_styled(
                None,
                None,
                None,
                Content::MathIdent("x".into()),
                Some(false),
            ),
        ];
        for value in styled {
            let expected = if value.plain_text() == "d" {
                "styled(child: [d], ..)"
            } else {
                "styled(child: [x], ..)"
            };
            assert_eq!(repr_content(&value), expected);
        }

        let class = Content::math_class_override(
            MathClass::Unary,
            Content::math_styled(
                None,
                None,
                Some(false),
                Content::MathText("d".into()),
                None,
            ),
        );
        assert_eq!(
            repr_content(&class),
            "class(class: \"unary\", body: styled(child: [d], ..))"
        );
    }

    #[test]
    fn p1293_b_binom_repr_rejeita_envelopes_nao_canonicos() {
        let lined = Content::math_delimited(
            '(',
            Content::math_frac(Content::text("n"), Content::text("k")),
            ')',
        );
        let wrong_delimiter = Content::math_delimited(
            '[',
            Content::math_frac_unlined(
                Content::text("n"),
                Content::MathSequence(Arc::from(vec![Content::text("k")])),
            ),
            ']',
        );
        let wrong_separator = Content::math_delimited(
            '(',
            Content::math_frac_unlined(
                Content::text("n"),
                Content::MathSequence(Arc::from(vec![
                    Content::text("k"),
                    Content::MathText(",".into()),
                    Content::text("j"),
                ])),
            ),
            ')',
        );

        for noncanonical in [lined, wrong_delimiter, wrong_separator] {
            assert!(
                !repr_content(&noncanonical).starts_with("binom("),
                "noncanonical envelope must keep the generic representation"
            );
        }
    }

    #[test]
    fn p1401_repr_before_after_omite_default_inclusive() {
        let heading = Selector::Kind(crate::entities::element_kind::ElementKind::Heading);
        assert_eq!(
            repr_selector(&Selector::Before {
                base: Box::new(heading.clone()),
                end: Box::new(heading.clone()),
                inclusive: true,
            }),
            "heading.before(heading)"
        );
        assert_eq!(
            repr_selector(&Selector::After {
                base: Box::new(heading.clone()),
                start: Box::new(heading),
                inclusive: false,
            }),
            "heading.after(heading, inclusive: false)"
        );
    }
}
