//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/tests/transforms.md
//! @prompt-hash a5a380b1
//! @layer L1
//! @updated 2026-09-26

use super::*;

#[test]
fn native_skew_defaults_produz_identidade() {
    // Sem ax nem ay, skew produz matriz identidade.
    null_ctx!(ctx);
    use crate::entities::layout_types::TransformMatrix;
    let body = Content::text("body");
    let r = native_skew(
        &mut ctx,
        &p(vec![Value::Content(body)]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    if let Value::Content(Content::Transform(e)) = r {
        assert_eq!(e.matrix, TransformMatrix::identity());
    } else {
        panic!("esperado Content::Transform");
    }
}

#[test]
fn native_skew_com_ax_angle() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Angle;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("ax".into(), Value::Angle(Angle::deg(30.0)));
    let r = native_skew(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Transform(e)) = r {
        // c = tan(30°) ≈ 0.5774; a = 1; b = 0; d = 1.
        assert!((e.matrix.a - 1.0).abs() < 1e-9);
        assert!((e.matrix.b - 0.0).abs() < 1e-9);
        assert!(
            (e.matrix.c - 0.5774).abs() < 0.001,
            "c esperado tan(30°), obteve {}",
            e.matrix.c
        );
        assert!((e.matrix.d - 1.0).abs() < 1e-9);
    } else {
        panic!("esperado Content::Transform");
    }
}

#[test]
fn native_skew_com_ay_angle() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Angle;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("ay".into(), Value::Angle(Angle::deg(30.0)));
    let r = native_skew(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Transform(e)) = r {
        // b = tan(30°) ≈ 0.5774; c = 0.
        assert!(
            (e.matrix.b - 0.5774).abs() < 0.001,
            "b esperado tan(30°), obteve {}",
            e.matrix.b
        );
        assert!((e.matrix.c - 0.0).abs() < 1e-9);
    } else {
        panic!("esperado Content::Transform");
    }
}

#[test]
fn native_skew_combina_ax_e_ay() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Angle;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("ax".into(), Value::Angle(Angle::deg(15.0)));
    args.named.insert("ay".into(), Value::Angle(Angle::deg(45.0)));
    let r = native_skew(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Transform(e)) = r {
        assert!((e.matrix.c - 15.0_f64.to_radians().tan()).abs() < 1e-9);
        assert!((e.matrix.b - 1.0).abs() < 1e-9, "tan(45°) ≈ 1.0; obteve {}", e.matrix.b);
    } else {
        panic!("esperado Content::Transform");
    }
}

#[test]
fn native_skew_aceita_float_radianos() {
    // Consistente com native_rotate: float é radianos directos.
    null_ctx!(ctx);
    use crate::entities::layout_types::TransformMatrix;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("ax".into(), Value::Float(0.0));
    let r = native_skew(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Transform(e)) = r {
        // tan(0) = 0 → identidade.
        assert_eq!(e.matrix, TransformMatrix::identity());
    } else {
        panic!("esperado Content::Transform");
    }
}

#[test]
fn native_skew_rejeita_named_arg_desconhecido() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Angle;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("origin".into(), Value::Str("center".into()));
    let r = native_skew(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "origin scope-out — named arg desconhecido em P156F");
    let _ = Angle::deg(0.0); // suprime warning de import não usado se ramo skip
}

#[test]
fn native_skew_rejeita_ax_proximo_de_pi_meio() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    // 89.999° → ax_rad ≈ π/2 - 1.7e-5; abaixo do threshold (π/2 - 1e-3).
    // Mas usemos exactamente 90°: tan diverge.
    args.named
        .insert("ax".into(), Value::Float(std::f64::consts::FRAC_PI_2));
    let r = native_skew(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "skew(ax: π/2) deve retornar Err (tan diverge)");
}

#[test]
fn native_skew_rejeita_ax_nao_angle_nem_float() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("ax".into(), Value::Str("30deg".into()));
    let r = native_skew(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "skew(ax:) com tipo errado deve retornar Err");
}

#[test]
fn native_skew_sem_body_retorna_err() {
    null_ctx!(ctx);
    let r = native_skew(&mut ctx, &p(vec![]), &null_world(), test_file_id());
    assert!(r.is_err(), "skew() sem body deve retornar Err");
}

#[test]
fn native_move_continua_a_produzir_transform_apos_p156f() {
    null_ctx!(ctx);
    use crate::entities::layout_types::TransformMatrix;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("dx".into(), Value::Float(10.0));
    args.named.insert("dy".into(), Value::Float(5.0));
    let r = native_move(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Transform(e)) = r {
        assert_eq!(e.matrix, TransformMatrix::translate(10.0, 5.0));
    } else {
        panic!("regressão: native_move deveria produzir Content::Transform");
    }
}

#[test]
fn native_rotate_continua_a_produzir_transform_apos_p156f() {
    null_ctx!(ctx);
    use crate::entities::layout_types::{Angle, TransformMatrix};
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("angle".into(), Value::Angle(Angle::deg(90.0)));
    let r = native_rotate(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Transform(e)) = r {
        // Comparação de matriz aproximada (rotate usa cos/sin).
        let expected = TransformMatrix::rotate(std::f64::consts::FRAC_PI_2);
        assert!((e.matrix.a - expected.a).abs() < 1e-9);
        assert!((e.matrix.b - expected.b).abs() < 1e-9);
        assert!((e.matrix.c - expected.c).abs() < 1e-9);
        assert!((e.matrix.d - expected.d).abs() < 1e-9);
    } else {
        panic!("regressão: native_rotate deveria produzir Content::Transform");
    }
}

#[test]
fn native_scale_continua_a_produzir_transform_apos_p156f() {
    null_ctx!(ctx);
    use crate::entities::layout_types::TransformMatrix;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("x".into(), Value::Float(2.0));
    let r = native_scale(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Transform(e)) = r {
        // x: 2.0; y default = sx = 2.0.
        assert_eq!(e.matrix, TransformMatrix::scale(2.0, 2.0));
    } else {
        panic!("regressão: native_scale deveria produzir Content::Transform");
    }
}

#[test]
fn p953_native_scale_factor_posicional_ratio() {
    null_ctx!(ctx);
    use crate::entities::layout_types::{Ratio, TransformMatrix};
    let args = p(vec![Value::Ratio(Ratio(1.5)), Value::Content(Content::text("x"))]);
    let r = native_scale(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Transform(e)) = r {
        assert_eq!(
            e.matrix,
            TransformMatrix::scale(1.5, 1.5),
            "P953: #scale(150%) deve produzir scale(1.5, 1.5), não identidade"
        );
    } else {
        panic!("P953: native_scale deve produzir Content::Transform");
    }
}

#[test]
fn p953_native_scale_dois_posicionais_e_ratio_nomeado() {
    null_ctx!(ctx);
    use crate::entities::layout_types::{Ratio, TransformMatrix};
    let args = p(vec![
        Value::Ratio(Ratio(2.0)),
        Value::Ratio(Ratio(0.5)),
        Value::Content(Content::text("x")),
    ]);
    let r = native_scale(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Transform(e)) = r {
        assert_eq!(e.matrix, TransformMatrix::scale(2.0, 0.5));
    } else {
        panic!("P953: native_scale deve produzir Content::Transform");
    }
    let mut args2 = p(vec![Value::Content(Content::text("x"))]);
    args2.named.insert("x".into(), Value::Ratio(Ratio(1.25)));
    let r2 = native_scale(&mut ctx, &args2, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Transform(e)) = r2 {
        assert_eq!(e.matrix, TransformMatrix::scale(1.25, 1.25));
    } else {
        panic!("P953: native_scale com x: Ratio nomeado deve produzir Transform");
    }
}
