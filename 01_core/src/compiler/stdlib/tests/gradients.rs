//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/tests/gradients.md
//! @prompt-hash 60b9e7cb
//! @layer L1
//! @updated 2026-09-26

use super::*;

#[test]
fn p262_gradient_linear_2_color_stops_no_offset() {
    use crate::entities::gradient::Gradient;
    use crate::entities::layout_types::Color;
    null_ctx!(ctx);
    let args =
        p(vec![Value::Color(Color::rgb(255, 0, 0)), Value::Color(Color::rgb(0, 0, 255))]);
    let r =
        native_gradient_linear(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Gradient(Gradient::Linear(l)) = r {
        assert_eq!(l.stops.len(), 2);
        assert_eq!(l.angle.to_rad(), 0.0);
        assert_eq!(l.stops[0].offset, None);
        assert_eq!(l.stops[1].offset, None);
    } else {
        panic!("esperado Value::Gradient(Linear)");
    }
}

#[test]
fn p262_gradient_linear_explicit_offsets() {
    use crate::entities::gradient::Gradient;
    use crate::entities::layout_types::{Color, Ratio};
    null_ctx!(ctx);
    let args = p(vec![
        Value::Array(vec![Value::Color(Color::rgb(255, 0, 0)), Value::Ratio(Ratio(0.0))]),
        Value::Array(vec![Value::Color(Color::rgb(0, 0, 255)), Value::Ratio(Ratio(1.0))]),
    ]);
    let r =
        native_gradient_linear(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Gradient(Gradient::Linear(l)) = r {
        assert_eq!(l.stops[0].offset, Some(Ratio(0.0)));
        assert_eq!(l.stops[1].offset, Some(Ratio(1.0)));
    } else {
        panic!("esperado Value::Gradient(Linear)");
    }
}

#[test]
fn p262_gradient_linear_angle_named() {
    use crate::entities::gradient::Gradient;
    use crate::entities::layout_types::{Angle, Color};
    null_ctx!(ctx);
    let args = pn(
        vec![Value::Color(Color::rgb(0, 0, 0)), Value::Color(Color::rgb(255, 255, 255))],
        "angle",
        Value::Angle(Angle::rad(std::f64::consts::FRAC_PI_2)),
    );
    let r =
        native_gradient_linear(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Gradient(Gradient::Linear(l)) = r {
        assert!((l.angle.to_rad() - std::f64::consts::FRAC_PI_2).abs() < 1e-9);
    } else {
        panic!("esperado Value::Gradient(Linear)");
    }
}

#[test]
fn p262_gradient_linear_zero_stops_erro() {
    null_ctx!(ctx);
    let args = p(vec![]);
    let r = native_gradient_linear(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "stops vazios deve retornar Err");
}

#[test]
fn p262_gradient_linear_offset_out_of_range_erro() {
    use crate::entities::layout_types::{Color, Ratio};
    null_ctx!(ctx);
    let args = p(vec![Value::Array(vec![
        Value::Color(Color::rgb(0, 0, 0)),
        Value::Ratio(Ratio(1.5)),
    ])]);
    let r = native_gradient_linear(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "offset 1.5 fora de [0,1] deve retornar Err");
}

#[test]
fn p262_gradient_linear_named_invalido_erro() {
    use crate::entities::layout_types::Color;
    null_ctx!(ctx);
    let args = pn(vec![Value::Color(Color::rgb(0, 0, 0))], "unknown", Value::Int(1));
    let r = native_gradient_linear(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "named arg desconhecido deve retornar Err");
}

#[test]
fn p262_gradient_linear_value_type_name() {
    use crate::entities::layout_types::Color;
    null_ctx!(ctx);
    let args = p(vec![
        Value::Color(Color::rgb(0, 0, 0)),
        Value::Color(Color::rgb(255, 255, 255)),
    ]);
    let r =
        native_gradient_linear(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    assert_eq!(r.type_name(), "gradient");
}

#[test]
fn p264_gradient_radial_2_color_stops_defaults() {
    use crate::entities::gradient::Gradient;
    use crate::entities::layout_types::{Color, Ratio};
    null_ctx!(ctx);
    let args =
        p(vec![Value::Color(Color::rgb(255, 0, 0)), Value::Color(Color::rgb(0, 0, 255))]);
    let r =
        native_gradient_radial(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Gradient(Gradient::Radial(rad)) = r {
        assert_eq!(rad.stops.len(), 2);
        // Defaults: center (50%, 50%); radius 50%.
        assert_eq!(rad.center.x, Ratio(0.5));
        assert_eq!(rad.center.y, Ratio(0.5));
        assert_eq!(rad.radius, Ratio(0.5));
    } else {
        panic!("esperado Value::Gradient(Radial)");
    }
}

#[test]
fn p264_gradient_radial_custom_center_radius() {
    use crate::entities::gradient::Gradient;
    use crate::entities::layout_types::{Color, Ratio};
    null_ctx!(ctx);
    let mut args =
        p(vec![Value::Color(Color::rgb(255, 0, 0)), Value::Color(Color::rgb(0, 0, 255))]);
    args.named.insert(
        "center".into(),
        Value::Array(vec![Value::Ratio(Ratio(0.25)), Value::Ratio(Ratio(0.75))]),
    );
    args.named.insert("radius".into(), Value::Ratio(Ratio(0.4)));
    let r =
        native_gradient_radial(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Gradient(Gradient::Radial(rad)) = r {
        assert_eq!(rad.center.x, Ratio(0.25));
        assert_eq!(rad.center.y, Ratio(0.75));
        assert_eq!(rad.radius, Ratio(0.4));
    } else {
        panic!("esperado Value::Gradient(Radial)");
    }
}

#[test]
fn p264_gradient_radial_zero_stops_erro() {
    null_ctx!(ctx);
    let args = p(vec![]);
    let r = native_gradient_radial(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "stops vazios deve retornar Err");
}

#[test]
fn p264_gradient_radial_radius_out_of_range_erro() {
    use crate::entities::layout_types::{Color, Ratio};
    null_ctx!(ctx);
    let mut args = p(vec![Value::Color(Color::rgb(0, 0, 0))]);
    args.named.insert("radius".into(), Value::Ratio(Ratio(1.5)));
    let r = native_gradient_radial(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "radius 1.5 fora de [0,1] deve retornar Err");
}

#[test]
fn p264_gradient_radial_value_type_name() {
    use crate::entities::layout_types::Color;
    null_ctx!(ctx);
    let args = p(vec![
        Value::Color(Color::rgb(0, 0, 0)),
        Value::Color(Color::rgb(255, 255, 255)),
    ]);
    let r =
        native_gradient_radial(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    assert_eq!(r.type_name(), "gradient");
}

#[test]
fn p269_stdlib_radial_focal_center_named() {
    use crate::entities::gradient::Gradient;
    use crate::entities::layout_types::{Color, Ratio};
    null_ctx!(ctx);
    let mut args =
        p(vec![Value::Color(Color::rgb(255, 0, 0)), Value::Color(Color::rgb(0, 0, 255))]);
    args.named.insert(
        "focal-center".into(),
        Value::Array(vec![Value::Ratio(Ratio(0.3)), Value::Ratio(Ratio(0.4))]),
    );
    let r =
        native_gradient_radial(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Gradient(Gradient::Radial(rad)) = r {
        assert_eq!(rad.focal_center.x, Ratio(0.3));
        assert_eq!(rad.focal_center.y, Ratio(0.4));
        assert_eq!(rad.focal_radius, Ratio(0.0), "default focal_radius=0");
    } else {
        panic!("esperado Value::Gradient(Radial)");
    }
}

#[test]
fn p269_stdlib_radial_focal_radius_named() {
    use crate::entities::gradient::Gradient;
    use crate::entities::layout_types::{Color, Ratio};
    null_ctx!(ctx);
    let mut args =
        p(vec![Value::Color(Color::rgb(255, 0, 0)), Value::Color(Color::rgb(0, 0, 255))]);
    args.named.insert("focal-radius".into(), Value::Ratio(Ratio(0.1)));
    let r =
        native_gradient_radial(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Gradient(Gradient::Radial(rad)) = r {
        assert_eq!(rad.focal_radius, Ratio(0.1));
        assert_eq!(rad.focal_center, rad.center, "default focal_center=center");
    } else {
        panic!("esperado Value::Gradient(Radial)");
    }
}

#[test]
fn p269_stdlib_radial_focal_ambos_named() {
    use crate::entities::gradient::Gradient;
    use crate::entities::layout_types::{Color, Ratio};
    null_ctx!(ctx);
    let mut args =
        p(vec![Value::Color(Color::rgb(255, 0, 0)), Value::Color(Color::rgb(0, 0, 255))]);
    args.named.insert(
        "focal-center".into(),
        Value::Array(vec![Value::Ratio(Ratio(0.25)), Value::Ratio(Ratio(0.35))]),
    );
    args.named.insert("focal-radius".into(), Value::Ratio(Ratio(0.08)));
    let r =
        native_gradient_radial(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Gradient(Gradient::Radial(rad)) = r {
        assert_eq!(rad.focal_center.x, Ratio(0.25));
        assert_eq!(rad.focal_center.y, Ratio(0.35));
        assert_eq!(rad.focal_radius, Ratio(0.08));
    } else {
        panic!("esperado Value::Gradient(Radial)");
    }
}

#[test]
fn p269_stdlib_radial_focal_defaults_preserva_p264() {
    // Sem named args focal_* → comportamento P264: focal=(center, 0).
    use crate::entities::gradient::Gradient;
    use crate::entities::layout_types::{Color, Ratio};
    null_ctx!(ctx);
    let args =
        p(vec![Value::Color(Color::rgb(255, 0, 0)), Value::Color(Color::rgb(0, 0, 255))]);
    let r =
        native_gradient_radial(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Gradient(Gradient::Radial(rad)) = r {
        assert_eq!(rad.focal_center, rad.center);
        assert_eq!(rad.focal_radius, Ratio(0.0));
    } else {
        panic!("esperado Value::Gradient(Radial)");
    }
}

#[test]
fn p269_stdlib_radial_focal_radius_maior_radius_erro() {
    // Validação vanilla portada: focal_radius > radius → erro.
    use crate::entities::layout_types::{Color, Ratio};
    null_ctx!(ctx);
    let mut args = p(vec![
        Value::Color(Color::rgb(0, 0, 0)),
        Value::Color(Color::rgb(255, 255, 255)),
    ]);
    args.named.insert("radius".into(), Value::Ratio(Ratio(0.3)));
    args.named.insert("focal-radius".into(), Value::Ratio(Ratio(0.4))); // > radius
    let r = native_gradient_radial(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "focal_radius 0.4 > radius 0.3 deve retornar Err");
}

#[test]
fn p270_stdlib_linear_space_default_oklab_preserva_p262() {
    use crate::entities::gradient::Gradient;
    use crate::entities::layout_types::{Color, ColorSpace};
    null_ctx!(ctx);
    let args =
        p(vec![Value::Color(Color::rgb(255, 0, 0)), Value::Color(Color::rgb(0, 0, 255))]);
    let r =
        native_gradient_linear(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Gradient(Gradient::Linear(l)) = r {
        assert_eq!(l.space, ColorSpace::Oklab, "default sem named arg deve ser Oklab");
    } else {
        panic!("expected Linear");
    }
}

#[test]
fn p270_stdlib_linear_space_named_hsl() {
    use crate::entities::gradient::Gradient;
    use crate::entities::layout_types::{Color, ColorSpace};
    use ecow::EcoString;
    null_ctx!(ctx);
    let mut args =
        p(vec![Value::Color(Color::rgb(255, 0, 0)), Value::Color(Color::rgb(0, 0, 255))]);
    args.named.insert("space".into(), Value::Str(EcoString::from("hsl")));
    let r =
        native_gradient_linear(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Gradient(Gradient::Linear(l)) = r {
        assert_eq!(l.space, ColorSpace::Hsl);
    } else {
        panic!("expected Linear");
    }
}

#[test]
fn p270_stdlib_linear_space_named_luma() {
    use crate::entities::gradient::Gradient;
    use crate::entities::layout_types::{Color, ColorSpace};
    use ecow::EcoString;
    null_ctx!(ctx);
    let mut args =
        p(vec![Value::Color(Color::rgb(255, 0, 0)), Value::Color(Color::rgb(0, 0, 255))]);
    args.named.insert("space".into(), Value::Str(EcoString::from("luma")));
    let r =
        native_gradient_linear(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Gradient(Gradient::Linear(l)) = r {
        assert_eq!(l.space, ColorSpace::Luma);
    } else {
        panic!("expected Linear");
    }
}

#[test]
fn p270_stdlib_linear_space_named_invalido_erro() {
    use crate::entities::layout_types::Color;
    use ecow::EcoString;
    null_ctx!(ctx);
    let mut args = p(vec![Value::Color(Color::rgb(0, 0, 0))]);
    args.named
        .insert("space".into(), Value::Str(EcoString::from("xyz_unknown")));
    let r = native_gradient_linear(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "space inválido deve retornar Err");
}

#[test]
fn p270_stdlib_radial_space_default_oklab_preserva_p264() {
    use crate::entities::gradient::Gradient;
    use crate::entities::layout_types::{Color, ColorSpace};
    null_ctx!(ctx);
    let args =
        p(vec![Value::Color(Color::rgb(255, 0, 0)), Value::Color(Color::rgb(0, 0, 255))]);
    let r =
        native_gradient_radial(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Gradient(Gradient::Radial(rad)) = r {
        assert_eq!(rad.space, ColorSpace::Oklab);
    } else {
        panic!("expected Radial");
    }
}

#[test]
fn p270_stdlib_radial_space_named_oklch() {
    use crate::entities::gradient::Gradient;
    use crate::entities::layout_types::{Color, ColorSpace};
    use ecow::EcoString;
    null_ctx!(ctx);
    let mut args =
        p(vec![Value::Color(Color::rgb(255, 0, 0)), Value::Color(Color::rgb(0, 0, 255))]);
    args.named
        .insert("space".into(), Value::Str(EcoString::from("oklch")));
    let r =
        native_gradient_radial(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Gradient(Gradient::Radial(rad)) = r {
        assert_eq!(rad.space, ColorSpace::Oklch);
    } else {
        panic!("expected Radial");
    }
}

#[test]
fn p270_stdlib_radial_space_named_cmyk() {
    use crate::entities::gradient::Gradient;
    use crate::entities::layout_types::{Color, ColorSpace};
    use ecow::EcoString;
    null_ctx!(ctx);
    let mut args =
        p(vec![Value::Color(Color::rgb(255, 0, 0)), Value::Color(Color::rgb(0, 0, 255))]);
    args.named.insert("space".into(), Value::Str(EcoString::from("cmyk")));
    let r =
        native_gradient_radial(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Gradient(Gradient::Radial(rad)) = r {
        assert_eq!(rad.space, ColorSpace::Cmyk);
    } else {
        panic!("expected Radial");
    }
}

#[test]
fn p270_stdlib_radial_space_named_invalido_erro() {
    use crate::entities::layout_types::Color;
    use ecow::EcoString;
    null_ctx!(ctx);
    let mut args = p(vec![Value::Color(Color::rgb(0, 0, 0))]);
    args.named
        .insert("space".into(), Value::Str(EcoString::from("invalid_space")));
    let r = native_gradient_radial(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "space inválido deve retornar Err");
}

#[test]
fn p270_stdlib_conic_space_default_oklab_preserva_p267() {
    use crate::entities::gradient::Gradient;
    use crate::entities::layout_types::{Color, ColorSpace};
    null_ctx!(ctx);
    let args =
        p(vec![Value::Color(Color::rgb(255, 0, 0)), Value::Color(Color::rgb(0, 0, 255))]);
    let r =
        native_gradient_conic(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Gradient(Gradient::Conic(c)) = r {
        assert_eq!(c.space, ColorSpace::Oklab);
    } else {
        panic!("expected Conic");
    }
}

#[test]
fn p270_stdlib_conic_space_named_hsv() {
    use crate::entities::gradient::Gradient;
    use crate::entities::layout_types::{Color, ColorSpace};
    use ecow::EcoString;
    null_ctx!(ctx);
    let mut args =
        p(vec![Value::Color(Color::rgb(255, 0, 0)), Value::Color(Color::rgb(0, 0, 255))]);
    args.named.insert("space".into(), Value::Str(EcoString::from("hsv")));
    let r =
        native_gradient_conic(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Gradient(Gradient::Conic(c)) = r {
        assert_eq!(c.space, ColorSpace::Hsv);
    } else {
        panic!("expected Conic");
    }
}

#[test]
fn p270_stdlib_conic_space_named_srgb() {
    use crate::entities::gradient::Gradient;
    use crate::entities::layout_types::{Color, ColorSpace};
    use ecow::EcoString;
    null_ctx!(ctx);
    let mut args =
        p(vec![Value::Color(Color::rgb(255, 0, 0)), Value::Color(Color::rgb(0, 0, 255))]);
    args.named.insert("space".into(), Value::Str(EcoString::from("srgb")));
    let r =
        native_gradient_conic(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Gradient(Gradient::Conic(c)) = r {
        assert_eq!(c.space, ColorSpace::Srgb);
    } else {
        panic!("expected Conic");
    }
}

#[test]
fn p270_stdlib_conic_space_named_invalido_erro() {
    use crate::entities::layout_types::Color;
    use ecow::EcoString;
    null_ctx!(ctx);
    let mut args = p(vec![Value::Color(Color::rgb(0, 0, 0))]);
    args.named.insert("space".into(), Value::Str(EcoString::from("???")));
    let r = native_gradient_conic(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "space inválido deve retornar Err");
}

#[test]
fn p273_stdlib_linear_relative_self() {
    use crate::entities::gradient::{Gradient, RelativeTo};
    use crate::entities::layout_types::Color;
    use ecow::EcoString;
    null_ctx!(ctx);
    let mut args =
        p(vec![Value::Color(Color::rgb(255, 0, 0)), Value::Color(Color::rgb(0, 0, 255))]);
    args.named
        .insert("relative".into(), Value::Str(EcoString::from("self")));
    let r =
        native_gradient_linear(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Gradient(Gradient::Linear(l)) = r {
        assert_eq!(l.relative, Some(RelativeTo::Self_));
    } else {
        panic!("expected Linear");
    }
}

#[test]
fn p273_stdlib_linear_relative_parent() {
    use crate::entities::gradient::{Gradient, RelativeTo};
    use crate::entities::layout_types::Color;
    use ecow::EcoString;
    null_ctx!(ctx);
    let mut args =
        p(vec![Value::Color(Color::rgb(0, 255, 0)), Value::Color(Color::rgb(0, 0, 255))]);
    args.named
        .insert("relative".into(), Value::Str(EcoString::from("parent")));
    let r =
        native_gradient_linear(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Gradient(Gradient::Linear(l)) = r {
        assert_eq!(l.relative, Some(RelativeTo::Parent));
    } else {
        panic!("expected Linear");
    }
}

#[test]
fn p273_stdlib_linear_relative_auto_default_none() {
    use crate::entities::gradient::Gradient;
    use crate::entities::layout_types::Color;
    use ecow::EcoString;
    null_ctx!(ctx);
    let mut args =
        p(vec![Value::Color(Color::rgb(0, 0, 255)), Value::Color(Color::rgb(255, 0, 0))]);
    args.named
        .insert("relative".into(), Value::Str(EcoString::from("auto")));
    let r =
        native_gradient_linear(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Gradient(Gradient::Linear(l)) = r {
        assert_eq!(l.relative, None, "auto explícito = None (Auto sentinel)");
    } else {
        panic!("expected Linear");
    }
}

#[test]
fn p273_stdlib_linear_relative_default_none_preserva_p270() {
    // Sem named arg `relative` → None (preserve P270.x behavior).
    use crate::entities::gradient::Gradient;
    use crate::entities::layout_types::Color;
    null_ctx!(ctx);
    let args = p(vec![
        Value::Color(Color::rgb(255, 255, 0)),
        Value::Color(Color::rgb(0, 0, 255)),
    ]);
    let r =
        native_gradient_linear(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Gradient(Gradient::Linear(l)) = r {
        assert_eq!(l.relative, None);
    } else {
        panic!("expected Linear");
    }
}

#[test]
fn p273_stdlib_radial_relative_parent() {
    use crate::entities::gradient::{Gradient, RelativeTo};
    use crate::entities::layout_types::Color;
    use ecow::EcoString;
    null_ctx!(ctx);
    let mut args = p(vec![
        Value::Color(Color::rgb(255, 0, 255)),
        Value::Color(Color::rgb(0, 255, 0)),
    ]);
    args.named
        .insert("relative".into(), Value::Str(EcoString::from("parent")));
    let r =
        native_gradient_radial(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Gradient(Gradient::Radial(rad)) = r {
        assert_eq!(rad.relative, Some(RelativeTo::Parent));
    } else {
        panic!("expected Radial");
    }
}

#[test]
fn p273_stdlib_conic_relative_self() {
    use crate::entities::gradient::{Gradient, RelativeTo};
    use crate::entities::layout_types::Color;
    use ecow::EcoString;
    null_ctx!(ctx);
    let mut args = p(vec![
        Value::Color(Color::rgb(0, 255, 255)),
        Value::Color(Color::rgb(255, 0, 0)),
    ]);
    args.named
        .insert("relative".into(), Value::Str(EcoString::from("self")));
    let r =
        native_gradient_conic(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Gradient(Gradient::Conic(c)) = r {
        assert_eq!(c.relative, Some(RelativeTo::Self_));
    } else {
        panic!("expected Conic");
    }
}

#[test]
fn p273_stdlib_relative_invalido_erro() {
    use crate::entities::layout_types::Color;
    use ecow::EcoString;
    null_ctx!(ctx);
    let mut args = p(vec![
        Value::Color(Color::rgb(0, 0, 0)),
        Value::Color(Color::rgb(255, 255, 255)),
    ]);
    args.named
        .insert("relative".into(), Value::Str(EcoString::from("inválido")));
    let r = native_gradient_linear(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "relative inválido deve retornar Err");
}

#[test]
fn p267_gradient_conic_2_color_stops_defaults() {
    use crate::entities::gradient::Gradient;
    use crate::entities::layout_types::{Color, Ratio};
    null_ctx!(ctx);
    let args =
        p(vec![Value::Color(Color::rgb(255, 0, 0)), Value::Color(Color::rgb(0, 0, 255))]);
    let r =
        native_gradient_conic(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Gradient(Gradient::Conic(c)) = r {
        assert_eq!(c.stops.len(), 2);
        assert_eq!(c.center.x, Ratio(0.5));
        assert_eq!(c.center.y, Ratio(0.5));
        assert_eq!(c.angle.to_rad(), 0.0);
    } else {
        panic!("esperado Value::Gradient(Conic)");
    }
}

#[test]
fn p267_gradient_conic_custom_center_angle() {
    use crate::entities::gradient::Gradient;
    use crate::entities::layout_types::{Angle, Color, Ratio};
    null_ctx!(ctx);
    let mut args =
        p(vec![Value::Color(Color::rgb(255, 0, 0)), Value::Color(Color::rgb(0, 0, 255))]);
    args.named.insert(
        "center".into(),
        Value::Array(vec![Value::Ratio(Ratio(0.3)), Value::Ratio(Ratio(0.7))]),
    );
    args.named.insert("angle".into(), Value::Angle(Angle::deg(45.0)));
    let r =
        native_gradient_conic(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Gradient(Gradient::Conic(c)) = r {
        assert_eq!(c.center.x, Ratio(0.3));
        assert_eq!(c.center.y, Ratio(0.7));
        assert!((c.angle.to_rad() - std::f64::consts::FRAC_PI_4).abs() < 1e-9);
    } else {
        panic!("esperado Value::Gradient(Conic)");
    }
}

#[test]
fn p267_gradient_conic_zero_stops_erro() {
    null_ctx!(ctx);
    let args = p(vec![]);
    let r = native_gradient_conic(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "stops vazios deve retornar Err");
}

#[test]
fn p267_gradient_conic_named_invalido_erro() {
    use crate::entities::layout_types::Color;
    null_ctx!(ctx);
    let mut args = p(vec![Value::Color(Color::rgb(0, 0, 0))]);
    args.named.insert("unknown".into(), Value::Int(1));
    let r = native_gradient_conic(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "named arg desconhecido deve retornar Err");
}

#[test]
fn p267_gradient_conic_value_type_name() {
    use crate::entities::layout_types::Color;
    null_ctx!(ctx);
    let args = p(vec![
        Value::Color(Color::rgb(0, 0, 0)),
        Value::Color(Color::rgb(255, 255, 255)),
    ]);
    let r =
        native_gradient_conic(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    assert_eq!(r.type_name(), "gradient");
}
