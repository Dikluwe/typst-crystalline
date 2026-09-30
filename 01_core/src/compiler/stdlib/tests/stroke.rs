//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/tests/stroke.md
//! @prompt-hash abe27a1e
//! @layer L1
//! @updated 2026-09-26

use super::*;

#[test]
fn p227_value_stroke_type_name_e_eq() {
    use crate::entities::geometry::Stroke;
    use crate::entities::layout_types::Color;
    use crate::entities::paint::Paint;
    let s = Stroke {
        paint: Paint::Solid(Color::rgb(255, 0, 0)),
        thickness: 2.5,
        overhang: false,
        ..Stroke::default()
    };
    let v = Value::Stroke(s.clone());
    assert_eq!(v.type_name(), "stroke");
    let v2 = Value::Stroke(s);
    assert_eq!(v, v2);
}

#[test]
fn p227_native_stroke_defaults_aceita() {
    // stroke() sem args → Stroke { BLACK, 1.0pt }.
    null_ctx!(ctx);
    let r = native_stroke(&mut ctx, &p(vec![]), &null_world(), test_file_id()).unwrap();
    if let Value::Stroke(s) = r {
        assert_eq!(s.thickness, 1.0);
    } else {
        panic!("esperado Value::Stroke");
    }
}

#[test]
fn p1224_native_stroke_preserva_campos_complexos() {
    use crate::entities::geometry::{DashLength, LineCap, LineJoin};
    use crate::entities::layout_types::Length;
    null_ctx!(ctx);
    let mut args = p(vec![]);
    args.named.insert("cap".into(), Value::Str("round".into()));
    args.named.insert("join".into(), Value::Str("bevel".into()));
    let mut dash = IndexMap::with_hasher(FxBuildHasher);
    dash.insert(
        "array".into(),
        Value::Array(vec![Value::Length(Length::pt(2.0)), Value::Str("dot".into())]),
    );
    dash.insert("phase".into(), Value::Length(Length::pt(-0.5)));
    args.named.insert("dash".into(), Value::Dict(dash));
    args.named.insert("miter-limit".into(), Value::Float(2.0));
    let Value::Stroke(stroke) =
        native_stroke(&mut ctx, &args, &null_world(), test_file_id()).unwrap()
    else {
        panic!("esperado stroke");
    };
    assert_eq!(stroke.cap, LineCap::Round);
    assert_eq!(stroke.join, LineJoin::Bevel);
    assert_eq!(stroke.miter_limit, 2.0);
    let dash = stroke.dash.unwrap();
    assert_eq!(dash.array, vec![DashLength::Length(2.0), DashLength::LineWidth]);
    assert_eq!(dash.phase, -0.5);
}

#[test]
fn p227_native_stroke_thickness_2pt_aceita() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![]);
    args.named.insert("thickness".into(), Value::Length(Length::pt(2.0)));
    let r = native_stroke(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Stroke(s) = r {
        assert_eq!(s.thickness, 2.0);
    } else {
        panic!("esperado Value::Stroke");
    }
}

#[test]
fn p227_native_stroke_thickness_negativo_rejeita() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![]);
    args.named.insert("thickness".into(), Value::Length(Length::pt(-1.0)));
    let r = native_stroke(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "thickness negativo deve falhar");
}

#[test]
fn p227_native_stroke_thickness_zero_rejeita() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![]);
    args.named.insert("thickness".into(), Value::Length(Length::pt(0.0)));
    let r = native_stroke(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "thickness 0 deve falhar (paridade vanilla)");
}

#[test]
fn p227_native_stroke_named_desconhecido_rejeita() {
    null_ctx!(ctx);
    let mut args = p(vec![]);
    args.named.insert("foo".into(), Value::Bool(true));
    let r = native_stroke(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "named desconhecido deve falhar");
}

#[test]
fn p227_native_stroke_posicional_tipo_invalido_rejeita() {
    null_ctx!(ctx);
    let r = native_stroke(
        &mut ctx,
        &p(vec![Value::Bool(true)]),
        &null_world(),
        test_file_id(),
    );
    assert!(r.is_err(), "posicional Bool deve falhar");
}

#[test]
fn p1499_stroke_posicional_preserva_automatico_e_explicito() {
    use crate::compiler::eval::repr::repr_value;
    null_ctx!(ctx);
    let automatic =
        native_stroke(&mut ctx, &p(vec![]), &null_world(), test_file_id()).unwrap();
    let rebuilt = native_stroke(
        &mut ctx,
        &p(vec![automatic.clone()]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    assert_eq!(rebuilt, automatic);
    assert_eq!(repr_value(&rebuilt), "stroke()");

    let mut named = p(vec![]);
    named
        .named
        .insert("paint".into(), Value::Color(Color::rgb(255, 65, 54)));
    named.named.insert("thickness".into(), Value::Length(Length::pt(2.0)));
    named.named.insert("cap".into(), Value::Str("round".into()));
    named.named.insert("overhang".into(), Value::Bool(false));
    let explicit =
        native_stroke(&mut ctx, &named, &null_world(), test_file_id()).unwrap();
    let rebuilt = native_stroke(
        &mut ctx,
        &p(vec![explicit.clone()]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    assert_eq!(rebuilt, explicit);
    assert_eq!(repr_value(&rebuilt), repr_value(&explicit));
}

#[test]
fn p1499_stroke_posicional_length_inclusive_zero_negativo() {
    use crate::compiler::eval::repr::repr_value;
    null_ctx!(ctx);
    for thickness in [2.0, 0.0, -1.0] {
        let value = native_stroke(
            &mut ctx,
            &p(vec![Value::Length(Length::pt(thickness))]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        let Value::Stroke(stroke) = &value else { panic!("esperado stroke") };
        assert_eq!(stroke.thickness, thickness);
        assert!(stroke.specified.thickness);
        assert!(!stroke.specified.paint);
        assert!(stroke.overhang);
        assert_eq!(repr_value(&value), format!("{thickness}pt"));
    }
}

#[test]
fn p1499_stroke_posicional_color_gradient_tiling_preenche_so_paint() {
    use crate::entities::gradient::{Gradient, GradientStop};
    use crate::entities::paint::Paint;
    use crate::entities::tiling::{Tiling, TilingBody};
    null_ctx!(ctx);
    let red = Color::rgb(255, 65, 54);
    let blue = Color::rgb(0, 0, 255);
    let gradient = Gradient::linear(
        vec![GradientStop::unspaced(red.clone()), GradientStop::unspaced(blue)],
        Angle::rad(0.0),
    );
    let tiling = Tiling::new(TilingBody::Color(red.clone()));
    let cases = [
        (Value::Color(red.clone()), Paint::Solid(red)),
        (Value::Gradient(gradient.clone()), Paint::Gradient(gradient)),
        (Value::Tiling(Arc::new(tiling.clone())), Paint::Tiling(tiling)),
    ];
    for (input, expected_paint) in cases {
        let value =
            native_stroke(&mut ctx, &p(vec![input]), &null_world(), test_file_id())
                .unwrap();
        let Value::Stroke(stroke) = value else { panic!("esperado stroke") };
        assert_eq!(stroke.paint, expected_paint);
        assert_eq!(stroke.thickness, 1.0);
        assert!(stroke.specified.paint);
        assert!(!stroke.specified.thickness);
        assert!(stroke.overhang);
    }
}

#[test]
fn p1499_stroke_posicional_dict_auto_e_campos_explicitos() {
    use crate::entities::geometry::{LineCap, StrokeFields};
    use crate::entities::paint::Paint;
    null_ctx!(ctx);
    let mut dict = IndexMap::with_hasher(FxBuildHasher);
    dict.insert("paint".into(), Value::Auto);
    dict.insert("thickness".into(), Value::Length(Length::pt(2.0)));
    dict.insert("cap".into(), Value::Str("round".into()));
    let value = native_stroke(
        &mut ctx,
        &p(vec![Value::Dict(dict)]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    let Value::Stroke(stroke) = value else { panic!("esperado stroke") };
    assert_eq!(stroke.paint, Paint::Solid(Color::rgb(0, 0, 0)));
    assert_eq!(stroke.thickness, 2.0);
    assert_eq!(stroke.cap, LineCap::Round);
    assert_eq!(
        stroke.specified,
        StrokeFields {
            thickness: true,
            cap: true,
            ..StrokeFields::default()
        }
    );
}

#[test]
fn p1499_stroke_posicional_rejeita_tipos_extras_mistura_e_dict_invalido() {
    null_ctx!(ctx);
    for value in [
        Value::Int(42),
        Value::Float(1.0),
        Value::Ratio(Ratio(0.5)),
        Value::Auto,
        Value::None,
        Value::Str("red".into()),
    ] {
        assert!(native_stroke(&mut ctx, &p(vec![value]), &null_world(), test_file_id(),)
            .is_err());
    }
    let length = Value::Length(Length::pt(1.0));
    assert!(native_stroke(
        &mut ctx,
        &p(vec![length.clone(), length.clone()]),
        &null_world(),
        test_file_id(),
    )
    .is_err());
    assert!(native_stroke(
        &mut ctx,
        &pn(vec![length], "paint", Value::Color(Color::rgb(255, 65, 54))),
        &null_world(),
        test_file_id(),
    )
    .is_err());
    for (key, value) in [
        ("overhang", Value::Bool(false)),
        ("thickness", Value::Str("2pt".into())),
        ("paint", Value::Int(42)),
    ] {
        let mut dict = IndexMap::with_hasher(FxBuildHasher);
        dict.insert(key.into(), value);
        assert!(native_stroke(
            &mut ctx,
            &p(vec![Value::Dict(dict)]),
            &null_world(),
            test_file_id(),
        )
        .is_err());
    }
}

#[test]
fn p1499_stroke_posicional_dict_auto_none_solid_e_miter_negativo() {
    null_ctx!(ctx);
    for (dash, specified, array_len) in [
        (Value::Auto, false, None),
        (Value::None, true, None),
        (Value::Str("solid".into()), true, Some(0)),
        (Value::Array(vec![]), true, Some(0)),
    ] {
        let mut dict = IndexMap::with_hasher(FxBuildHasher);
        dict.insert("dash".into(), dash);
        dict.insert("thickness".into(), Value::Length(Length::pt(-1.0)));
        dict.insert("miter-limit".into(), Value::Float(-1.0));
        let result = native_stroke(
            &mut ctx,
            &p(vec![Value::Dict(dict)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        let Value::Stroke(stroke) = result else { panic!("esperado stroke") };
        assert_eq!(stroke.thickness, -1.0);
        assert_eq!(stroke.miter_limit, -1.0);
        assert!(stroke.specified.thickness);
        assert!(stroke.specified.miter_limit);
        assert_eq!(stroke.specified.dash, specified);
        assert_eq!(stroke.dash.as_ref().map(|dash| dash.array.len()), array_len);
    }

    let mut invalid = IndexMap::with_hasher(FxBuildHasher);
    invalid.insert("thickness".into(), Value::Int(2));
    assert!(native_stroke(
        &mut ctx,
        &p(vec![Value::Dict(invalid)]),
        &null_world(),
        test_file_id(),
    )
    .is_err());
}

#[test]
fn p227_native_grid_stroke_length_shorthand() {
    // grid(stroke: 2pt) → Stroke{BLACK, 2pt}.
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("a"))]);
    args.named.insert("stroke".into(), Value::Length(Length::pt(2.0)));
    let r = native_grid(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Grid(e)) = r {
        assert!(e.stroke.is_some());
        assert_eq!(e.stroke.clone().unwrap().thickness, 2.0);
    } else {
        panic!("esperado Content::Grid");
    }
}

#[test]
fn p227_native_grid_stroke_color_shorthand() {
    // grid(stroke: red) → Stroke{red, 1pt}.
    null_ctx!(ctx);
    use crate::entities::layout_types::Color;
    let mut args = p(vec![Value::Content(Content::text("a"))]);
    args.named
        .insert("stroke".into(), Value::Color(Color::rgb(255, 0, 0)));
    let r = native_grid(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Grid(e)) = r {
        let s = e.stroke.clone().expect("stroke presente");
        assert_eq!(s.thickness, 1.0);
    } else {
        panic!("esperado Content::Grid");
    }
}

#[test]
fn p227_native_grid_stroke_value_stroke_passa() {
    // grid(stroke: stroke(thickness: 3pt)) → preserva Stroke.
    null_ctx!(ctx);
    use crate::entities::geometry::Stroke;
    use crate::entities::layout_types::Color;
    use crate::entities::paint::Paint;
    let s = Stroke {
        paint: Paint::Solid(Color::rgb(0, 255, 0)),
        thickness: 3.0,
        overhang: false,
        ..Stroke::default()
    };
    let mut args = p(vec![Value::Content(Content::text("a"))]);
    args.named.insert("stroke".into(), Value::Stroke(s.clone()));
    let r = native_grid(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Grid(e)) = r {
        assert_eq!(e.stroke, Some(s));
    } else {
        panic!("esperado Content::Grid");
    }
}

#[test]
fn p227_native_table_stroke_paridade_grid() {
    // table(stroke: 1pt) paridade native_grid.
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("a"))]);
    args.named.insert("stroke".into(), Value::Length(Length::pt(1.0)));
    let r = native_table(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Table(e)) = r {
        assert!(e.stroke.is_some());
    } else {
        panic!("esperado Content::Table");
    }
}

#[test]
fn p1250a_stroke_miter_limit_distingue_finito_positivo() {
    for invalid in [f64::NAN, f64::INFINITY, 0.0, -1.0] {
        assert!(!super::layout::is_valid_miter_limit(invalid));
    }
    assert!(super::layout::is_valid_miter_limit(2.5));
}
