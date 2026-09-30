//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/tests/layout_containers.md
//! @prompt-hash 51514555
//! @layer L1
//! @updated 2026-09-26

use super::*;

#[test]
fn native_block_defaults_sem_args_named() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let r = native_block(
        &mut ctx,
        &p(vec![Value::Content(Content::text("body"))]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    if let Value::Content(Content::Block(e)) = r {
        assert_eq!(e.body.plain_text(), "body");
        assert_eq!(e.width, None);
        assert_eq!(e.height, None);
        assert_eq!(e.inset.left, Length::ZERO);
        assert!(e.breakable, "default breakable é true");
    } else {
        panic!("esperado Content::Block");
    }
}

#[test]
fn native_block_sem_body_aceita_empty() {
    // Vanilla aceita block() sem body; cristalino igualmente
    // (Content::Empty como fallback).
    null_ctx!(ctx);
    let r = native_block(&mut ctx, &p(vec![]), &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Block(e)) = r {
        assert!(e.body.is_empty());
    } else {
        panic!("esperado Content::Block com body Empty");
    }
}

#[test]
fn native_block_com_width_length() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("width".into(), Value::Length(Length::pt(100.0)));
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Block(e)) = r {
        assert_eq!(
            e.width,
            Some(crate::entities::rel::Rel { rel: 0.0, abs: Length::pt(100.0) })
        );
    } else {
        panic!("esperado Content::Block");
    }
}

#[test]
fn native_block_com_height_int_pt() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("height".into(), Value::Int(50));
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Block(e)) = r {
        assert_eq!(
            e.height,
            Some(crate::entities::elements::block::BlockHeight::Relative(
                crate::entities::rel::Rel { rel: 0.0, abs: Length::pt(50.0) }
            ))
        );
    } else {
        panic!("esperado Content::Block");
    }
}

#[test]
fn native_block_com_inset_uniforme() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("inset".into(), Value::Length(Length::pt(8.0)));
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Block(e)) = r {
        assert_eq!(e.inset.left, Length::pt(8.0));
        assert_eq!(e.inset.right, Length::pt(8.0));
        assert_eq!(e.inset.top, Length::pt(8.0));
        assert_eq!(e.inset.bottom, Length::pt(8.0));
    } else {
        panic!("esperado Content::Block");
    }
}

#[test]
fn native_block_com_breakable_false() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("breakable".into(), Value::Bool(false));
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Block(e)) = r {
        assert!(!e.breakable);
    } else {
        panic!("esperado Content::Block");
    }
}

#[test]
fn native_block_combina_atributos() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("width".into(), Value::Length(Length::pt(200.0)));
    args.named.insert("height".into(), Value::Length(Length::pt(80.0)));
    args.named.insert("inset".into(), Value::Length(Length::pt(4.0)));
    args.named.insert("breakable".into(), Value::Bool(false));
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Block(e)) = r {
        assert_eq!(
            e.width,
            Some(crate::entities::rel::Rel { rel: 0.0, abs: Length::pt(200.0) })
        );
        assert_eq!(
            e.height,
            Some(crate::entities::elements::block::BlockHeight::Relative(
                crate::entities::rel::Rel { rel: 0.0, abs: Length::pt(80.0) }
            ))
        );
        assert_eq!(e.inset.top, Length::pt(4.0));
        assert!(!e.breakable);
    } else {
        panic!("esperado Content::Block");
    }
}

#[test]
fn native_block_rejeita_named_arg_avancado() {
    // P247 — fill aceito (Color); Str("red") ainda inválido (tipo
    // errado). Test preserva intent: rejeitar tipo errado para fill.
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("fill".into(), Value::Str("red".into()));
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "fill com tipo errado (Str em vez de Color) deve retornar Err");
}

#[test]
fn p247_native_block_aceita_fill_color() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Color;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("fill".into(), Value::Color(Color::rgb(255, 0, 0)));
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Block(e)) = r {
        assert_eq!(e.fill, Some(Color::rgb(255, 0, 0)));
    } else {
        panic!("esperado Content::Block");
    }
}

#[test]
fn p247_native_block_aceita_stroke_length_shorthand() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("stroke".into(), Value::Length(Length::pt(2.0)));
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Block(e)) = r {
        let s = e.stroke.clone().expect("stroke deveria ser Some");
        assert_eq!(s.thickness, 2.0);
    } else {
        panic!("esperado Content::Block");
    }
}

#[test]
fn p247_native_block_fill_default_none() {
    null_ctx!(ctx);
    let r = native_block(
        &mut ctx,
        &p(vec![Value::Content(Content::text("x"))]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    if let Value::Content(Content::Block(e)) = r {
        assert_eq!(e.fill, None);
        assert!(e.stroke.is_none());
    } else {
        panic!("esperado Content::Block");
    }
}

#[test]
fn p247_native_block_fill_tipo_invalido_rejeitado() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("fill".into(), Value::Bool(true));
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "fill com Bool deve retornar Err");
}

#[test]
fn p247_native_box_aceita_fill_color() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Color;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("fill".into(), Value::Color(Color::rgb(0, 255, 0)));
    let r = native_box(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Boxed(e)) = r {
        assert_eq!(e.fill, Some(Color::rgb(0, 255, 0)));
    } else {
        panic!("esperado Content::Boxed");
    }
}

#[test]
fn p247_native_box_aceita_stroke_color_shorthand() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Color;
    use crate::entities::paint::Paint;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named
        .insert("stroke".into(), Value::Color(Color::rgb(0, 0, 255)));
    let r = native_box(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Boxed(e)) = r {
        let s = e.stroke.clone().expect("stroke deveria ser Some");
        assert_eq!(s.paint, Paint::Solid(Color::rgb(0, 0, 255)));
        assert_eq!(s.thickness, 1.0, "Color shorthand default 1pt thickness");
    } else {
        panic!("esperado Content::Boxed");
    }
}

#[test]
fn p247_native_box_fill_default_none() {
    null_ctx!(ctx);
    let r = native_box(
        &mut ctx,
        &p(vec![Value::Content(Content::text("x"))]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    if let Value::Content(Content::Boxed(e)) = r {
        assert_eq!(e.fill, None);
        assert!(e.stroke.is_none());
    } else {
        panic!("esperado Content::Boxed");
    }
}

#[test]
fn p247_native_block_combina_fill_stroke_radius_clip_outset() {
    null_ctx!(ctx);
    use crate::entities::layout_types::{Color, Length};
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named
        .insert("fill".into(), Value::Color(Color::rgb(50, 100, 150)));
    args.named.insert("stroke".into(), Value::Length(Length::pt(1.5)));
    args.named.insert("radius".into(), Value::Length(Length::pt(3.0)));
    args.named.insert("clip".into(), Value::Bool(true));
    args.named.insert("outset".into(), Value::Length(Length::pt(2.0)));
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Block(e)) = r {
        assert_eq!(e.fill, Some(Color::rgb(50, 100, 150)));
        assert_eq!(e.stroke.clone().unwrap().thickness, 1.5);
        assert_eq!(e.radius.top_left, Length::pt(3.0));
        assert_eq!(e.clip, true);
        assert_eq!(e.outset.left, Length::pt(2.0));
    } else {
        panic!("esperado Content::Block");
    }
}

#[test]
fn native_block_rejeita_width_negativo() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("width".into(), Value::Length(Length::pt(-10.0)));
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "width negativo deve retornar Err em P156G");
}

#[test]
fn p248_native_block_breakable_false_propagado_a_variant() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("breakable".into(), Value::Bool(false));
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Block(e)) = r {
        assert_eq!(
            e.breakable, false,
            "P248 — native_block(breakable: false) propaga literal para variant"
        );
    } else {
        panic!("esperado Content::Block");
    }
}

#[test]
fn p248_native_block_breakable_default_true_p156g() {
    null_ctx!(ctx);
    let r = native_block(
        &mut ctx,
        &p(vec![Value::Content(Content::text("x"))]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    if let Value::Content(Content::Block(e)) = r {
        assert_eq!(e.breakable, true, "P248 — default preservado P156G");
    } else {
        panic!("esperado Content::Block");
    }
}

#[test]
fn p248_native_box_height_some_propagado_a_variant() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("height".into(), Value::Length(Length::pt(50.0)));
    args.named.insert("clip".into(), Value::Bool(true));
    let r = native_box(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Boxed(e)) = r {
        assert_eq!(e.height, Some(Length::pt(50.0)));
        assert_eq!(e.clip, true);
    } else {
        panic!("esperado Content::Boxed");
    }
}

#[test]
fn p248_native_box_height_default_none_p156h() {
    null_ctx!(ctx);
    let r = native_box(
        &mut ctx,
        &p(vec![Value::Content(Content::text("x"))]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    if let Value::Content(Content::Boxed(e)) = r {
        assert_eq!(e.height, None);
        assert_eq!(e.clip, false, "P248 — defaults preservados P156H");
    } else {
        panic!("esperado Content::Boxed");
    }
}

#[test]
fn p250_native_block_aceita_spacing_above_below_sticky() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("spacing".into(), Value::Length(Length::pt(12.0)));
    args.named.insert("above".into(), Value::Length(Length::pt(20.0)));
    args.named.insert("below".into(), Value::Length(Length::pt(8.0)));
    args.named.insert("sticky".into(), Value::Bool(true));
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Block(e)) = r {
        assert_eq!(e.spacing, Some(Length::pt(12.0)));
        assert_eq!(e.above, Some(Length::pt(20.0)));
        assert_eq!(e.below, Some(Length::pt(8.0)));
        assert_eq!(e.sticky, true);
    } else {
        panic!("esperado Content::Block");
    }
}

#[test]
fn p250_native_block_defaults_4_fields() {
    null_ctx!(ctx);
    let r = native_block(
        &mut ctx,
        &p(vec![Value::Content(Content::text("x"))]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    if let Value::Content(Content::Block(e)) = r {
        assert_eq!(e.spacing, None);
        assert_eq!(e.above, None);
        assert_eq!(e.below, None);
        assert_eq!(e.sticky, false, "P250 defaults preservam pre-P250");
    } else {
        panic!("esperado Content::Block");
    }
}

#[test]
fn p250_native_block_spacing_negativo_rejeitado() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("spacing".into(), Value::Length(Length::pt(-5.0)));
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "P250 spacing negativo deve retornar Err");
}

#[test]
fn p250_native_block_sticky_nao_bool_rejeitado() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("sticky".into(), Value::Int(1));
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "P250 sticky Int em vez de Bool deve retornar Err");
}

#[test]
fn p250_native_block_above_tipo_invalido_rejeitado() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("above".into(), Value::Bool(true));
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "P250 above com Bool deve retornar Err");
}

#[test]
fn p252_native_block_stroke_length_atalho_overhang_default_true() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("stroke".into(), Value::Length(Length::pt(2.5)));
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Block(e)) = r {
        let s = e.stroke.clone().expect("stroke deveria ser Some");
        assert_eq!(s.thickness, 2.5);
        assert_eq!(
            s.overhang, true,
            "P252 — Length atalho default overhang=true (paridade vanilla)"
        );
    } else {
        panic!("esperado Content::Block com stroke");
    }
}

#[test]
fn p252_native_block_stroke_color_atalho_overhang_default_true() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Color;
    use crate::entities::paint::Paint;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named
        .insert("stroke".into(), Value::Color(Color::rgb(255, 0, 0)));
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Block(e)) = r {
        let s = e.stroke.clone().expect("stroke deveria ser Some");
        assert_eq!(s.paint, Paint::Solid(Color::rgb(255, 0, 0)));
        assert_eq!(s.thickness, 1.0);
        assert_eq!(
            s.overhang, true,
            "P252 — Color atalho default overhang=true (paridade vanilla)"
        );
    } else {
        panic!("esperado Content::Block com stroke");
    }
}

#[test]
fn p252_native_stroke_overhang_explicit_false() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    // stroke(thickness: 2pt, overhang: false) → Stroke com
    // overhang=false explícito.
    let mut args = p(vec![]);
    args.named.insert("thickness".into(), Value::Length(Length::pt(2.0)));
    args.named.insert("overhang".into(), Value::Bool(false));
    let r = native_stroke(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Stroke(s) = r {
        assert_eq!(s.thickness, 2.0);
        assert_eq!(s.overhang, false);
    } else {
        panic!("esperado Value::Stroke");
    }
}

#[test]
fn p252_native_stroke_overhang_default_true_paridade_vanilla() {
    null_ctx!(ctx);
    // stroke() sem args → Stroke { BLACK, 1.0pt, overhang: true }.
    let r = native_stroke(&mut ctx, &p(vec![]), &null_world(), test_file_id()).unwrap();
    if let Value::Stroke(s) = r {
        assert_eq!(
            s.overhang, true,
            "P252 — native_stroke default vanilla overhang=true"
        );
    } else {
        panic!("esperado Value::Stroke");
    }
}

#[test]
fn p252_native_stroke_overhang_nao_bool_rejeitado() {
    null_ctx!(ctx);
    let mut args = p(vec![]);
    args.named.insert("overhang".into(), Value::Int(1));
    let r = native_stroke(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "P252 overhang com Int em vez de Bool deve retornar Err");
}

#[test]
fn native_block_rejeita_inset_negativo() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("inset".into(), Value::Length(Length::pt(-2.0)));
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "inset negativo deve retornar Err em P156G");
}

#[test]
fn native_block_rejeita_breakable_nao_bool() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("breakable".into(), Value::Int(1));
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "breakable não-Bool deve retornar Err");
}

#[test]
fn native_pad_e_hide_continuam_a_funcionar_apos_p156g() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    // Pad regression
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("rest".into(), Value::Length(Length::pt(5.0)));
    let r = native_pad(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    assert!(
        matches!(r, Value::Content(Content::Pad(e))),
        "regressão: native_pad deveria produzir Content::Pad"
    );
    // Hide regression
    let r = native_hide(
        &mut ctx,
        &p(vec![Value::Content(Content::text("y"))]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    assert!(
        matches!(r, Value::Content(Content::Hide(_))),
        "regressão: native_hide deveria produzir Content::Hide"
    );
}

#[test]
fn native_box_defaults_sem_args_named() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let r = native_box(
        &mut ctx,
        &p(vec![Value::Content(Content::text("body"))]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    if let Value::Content(Content::Boxed(e)) = r {
        assert_eq!(e.body.plain_text(), "body");
        assert_eq!(e.width, None);
        assert_eq!(e.height, None);
        assert_eq!(e.inset.left, Length::ZERO);
        assert_eq!(e.baseline, Length::ZERO);
    } else {
        panic!("esperado Content::Boxed");
    }
}

#[test]
fn native_box_sem_body_aceita_empty() {
    null_ctx!(ctx);
    let r = native_box(&mut ctx, &p(vec![]), &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Boxed(e)) = r {
        assert!(e.body.is_empty());
    } else {
        panic!("esperado Content::Boxed com body Empty");
    }
}

#[test]
fn native_box_com_width_length() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("width".into(), Value::Length(Length::pt(80.0)));
    let r = native_box(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Boxed(e)) = r {
        assert_eq!(e.width, Some(Length::pt(80.0)));
    } else {
        panic!("esperado Content::Boxed");
    }
}

#[test]
fn native_box_com_height_int_pt() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("height".into(), Value::Int(20));
    let r = native_box(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Boxed(e)) = r {
        assert_eq!(e.height, Some(Length::pt(20.0)));
    } else {
        panic!("esperado Content::Boxed");
    }
}

#[test]
fn native_box_com_inset_uniforme() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("inset".into(), Value::Length(Length::pt(4.0)));
    let r = native_box(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Boxed(e)) = r {
        assert_eq!(e.inset.left, Length::pt(4.0));
        assert_eq!(e.inset.right, Length::pt(4.0));
        assert_eq!(e.inset.top, Length::pt(4.0));
        assert_eq!(e.inset.bottom, Length::pt(4.0));
    } else {
        panic!("esperado Content::Boxed");
    }
}

#[test]
fn native_box_com_baseline() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("baseline".into(), Value::Length(Length::pt(3.0)));
    let r = native_box(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Boxed(e)) = r {
        assert_eq!(e.baseline, Length::pt(3.0));
    } else {
        panic!("esperado Content::Boxed");
    }
}

#[test]
fn native_box_baseline_negativo_aceito() {
    // Diferente de width/height/inset: baseline negativo move
    // box para cima — semantic legítima.
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("baseline".into(), Value::Length(Length::pt(-5.0)));
    let r = native_box(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Boxed(e)) = r {
        assert_eq!(e.baseline, Length::pt(-5.0));
    } else {
        panic!("esperado Content::Boxed");
    }
}

#[test]
fn native_box_combina_atributos() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("width".into(), Value::Length(Length::pt(100.0)));
    args.named.insert("height".into(), Value::Length(Length::pt(30.0)));
    args.named.insert("inset".into(), Value::Length(Length::pt(2.0)));
    args.named.insert("baseline".into(), Value::Length(Length::pt(1.0)));
    let r = native_box(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Boxed(e)) = r {
        assert_eq!(e.width, Some(Length::pt(100.0)));
        assert_eq!(e.height, Some(Length::pt(30.0)));
        assert_eq!(e.inset.top, Length::pt(2.0));
        assert_eq!(e.baseline, Length::pt(1.0));
    } else {
        panic!("esperado Content::Boxed");
    }
}

#[test]
fn native_box_rejeita_atributo_avancado() {
    // fill/stroke/etc são scope-out per ADR-0054 graded.
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("fill".into(), Value::Str("red".into()));
    let r = native_box(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "fill é scope-out em P156H; deve retornar Err");
}

#[test]
fn native_box_rejeita_width_negativo() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("width".into(), Value::Length(Length::pt(-5.0)));
    let r = native_box(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "width negativo deve retornar Err em P156H");
}

#[test]
fn native_box_rejeita_inset_negativo() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("inset".into(), Value::Length(Length::pt(-1.0)));
    let r = native_box(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "inset negativo deve retornar Err em P156H");
}

#[test]
fn native_box_rejeita_named_arg_desconhecido() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("alignment".into(), Value::Str("center".into()));
    let r = native_box(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "named arg desconhecido em box() deve retornar Err");
}

#[test]
fn native_stack_defaults_sem_args() {
    null_ctx!(ctx);
    use crate::entities::dir::Dir;
    let r = native_stack(&mut ctx, &p(vec![]), &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Stack(e)) = r {
        assert!(e.children.is_empty());
        assert_eq!(e.dir, Dir::TTB); // default
        assert_eq!(e.spacing, None);
    } else {
        panic!("esperado Content::Stack");
    }
}

#[test]
fn native_stack_aceita_dir_ltr() {
    null_ctx!(ctx);
    use crate::entities::dir::Dir;
    let mut args = p(vec![Value::Content(Content::text("a"))]);
    args.named.insert("dir".into(), Value::Dir(Dir::LTR));
    let r = native_stack(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Stack(e)) = r {
        assert_eq!(e.dir, Dir::LTR);
    } else {
        panic!("esperado Content::Stack");
    }
}

#[test]
fn native_stack_aceita_todas_4_direcoes() {
    null_ctx!(ctx);
    use crate::entities::dir::Dir;
    for d in [Dir::LTR, Dir::RTL, Dir::TTB, Dir::BTT] {
        let mut args = p(vec![]);
        args.named.insert("dir".into(), Value::Dir(d));
        let r = native_stack(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        if let Value::Content(Content::Stack(e)) = r {
            assert_eq!(e.dir, d, "dir={d:?}");
        } else {
            panic!("esperado Content::Stack para dir={d:?}");
        }
    }
}

#[test]
fn native_stack_aceita_spacing() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("a"))]);
    args.named.insert("spacing".into(), Value::Length(Length::pt(8.0)));
    let r = native_stack(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Stack(e)) = r {
        assert_eq!(e.spacing, Some(Length::pt(8.0)));
    } else {
        panic!("esperado Content::Stack");
    }
}

#[test]
fn native_stack_com_children_variadicos() {
    null_ctx!(ctx);
    let args = p(vec![
        Value::Content(Content::text("a")),
        Value::Content(Content::text("b")),
        Value::Content(Content::text("c")),
    ]);
    let r = native_stack(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Stack(e)) = r {
        assert_eq!(e.children.len(), 3);
        assert_eq!(e.children[0].plain_text(), "a");
        assert_eq!(e.children[1].plain_text(), "b");
        assert_eq!(e.children[2].plain_text(), "c");
    } else {
        panic!("esperado Content::Stack");
    }
}

#[test]
fn native_stack_aceita_str_como_child() {
    null_ctx!(ctx);
    let args = p(vec![Value::Str("hello".into())]);
    let r = native_stack(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Stack(e)) = r {
        assert_eq!(e.children.len(), 1);
        assert_eq!(e.children[0].plain_text(), "hello");
    } else {
        panic!("esperado Content::Stack");
    }
}

#[test]
fn native_stack_rejeita_dir_string() {
    null_ctx!(ctx);
    let mut args = p(vec![]);
    args.named.insert("dir".into(), Value::Str("ltr".into()));
    let r = native_stack(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "dir string deve retornar Err (paridade vanilla)");
}

#[test]
fn native_stack_rejeita_dir_invalido() {
    null_ctx!(ctx);
    let mut args = p(vec![]);
    args.named.insert("dir".into(), Value::Str("middle".into()));
    let r = native_stack(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "dir inválido deve retornar Err");
}

#[test]
fn native_stack_rejeita_spacing_negativo() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![]);
    args.named.insert("spacing".into(), Value::Length(Length::pt(-1.0)));
    let r = native_stack(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "spacing negativo deve retornar Err");
}

#[test]
fn native_stack_rejeita_named_arg_desconhecido() {
    null_ctx!(ctx);
    let mut args = p(vec![]);
    args.named.insert("baseline".into(), Value::Bool(true));
    let r = native_stack(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "named arg desconhecido em stack() deve retornar Err");
}

#[test]
fn native_stack_rejeita_child_nao_content() {
    null_ctx!(ctx);
    let r =
        native_stack(&mut ctx, &p(vec![Value::Int(42)]), &null_world(), test_file_id());
    assert!(r.is_err(), "child Int deve retornar Err");
}

#[test]
fn native_stack_combina_dir_spacing_children() {
    null_ctx!(ctx);
    use crate::entities::dir::Dir;
    use crate::entities::layout_types::Length;
    let mut args =
        p(vec![Value::Content(Content::text("a")), Value::Content(Content::text("b"))]);
    args.named.insert("dir".into(), Value::Dir(Dir::LTR));
    args.named.insert("spacing".into(), Value::Length(Length::pt(4.0)));
    let r = native_stack(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Stack(e)) = r {
        assert_eq!(e.children.len(), 2);
        assert_eq!(e.dir, Dir::LTR);
        assert_eq!(e.spacing, Some(Length::pt(4.0)));
    } else {
        panic!("esperado Content::Stack");
    }
}

#[test]
fn native_block_pad_hide_continuam_a_funcionar_apos_p156h() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    // Block regression
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("inset".into(), Value::Length(Length::pt(3.0)));
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    assert!(
        matches!(r, Value::Content(Content::Block(e))),
        "regressão: native_block deveria produzir Content::Block"
    );
    // Pad regression
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("rest".into(), Value::Length(Length::pt(5.0)));
    let r = native_pad(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    assert!(
        matches!(r, Value::Content(Content::Pad(e))),
        "regressão: native_pad deveria produzir Content::Pad"
    );
    // Hide regression
    let r = native_hide(
        &mut ctx,
        &p(vec![Value::Content(Content::text("y"))]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    assert!(
        matches!(r, Value::Content(Content::Hide(_))),
        "regressão: native_hide deveria produzir Content::Hide"
    );
}

#[test]
fn native_block_box_pad_hide_continuam_a_funcionar_apos_p156i() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    // Block
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("width".into(), Value::Length(Length::pt(50.0)));
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    assert!(matches!(r, Value::Content(Content::Block(e))));
    // Box (Boxed)
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("baseline".into(), Value::Length(Length::pt(2.0)));
    let r = native_box(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    assert!(matches!(r, Value::Content(Content::Boxed(e))));
    // Pad
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("rest".into(), Value::Length(Length::pt(2.0)));
    let r = native_pad(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    assert!(matches!(r, Value::Content(Content::Pad(e))));
    // Hide
    let r = native_hide(
        &mut ctx,
        &p(vec![Value::Content(Content::text("y"))]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    assert!(matches!(r, Value::Content(Content::Hide(_))));
}

#[test]
fn native_repeat_defaults_gap_none_justify_true() {
    null_ctx!(ctx);
    let r = native_repeat(
        &mut ctx,
        &p(vec![Value::Content(Content::text("."))]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    if let Value::Content(Content::Repeat(e)) = r {
        assert_eq!(e.body.plain_text(), ".");
        assert_eq!(e.gap, None);
        assert!(e.justify, "default justify == true (paridade vanilla)");
    } else {
        panic!("esperado Content::Repeat");
    }
}

#[test]
fn native_repeat_aceita_str_como_body() {
    null_ctx!(ctx);
    let r = native_repeat(
        &mut ctx,
        &p(vec![Value::Str(".".into())]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    if let Value::Content(Content::Repeat(e)) = r {
        assert_eq!(e.body.plain_text(), ".");
    } else {
        panic!("esperado Content::Repeat");
    }
}

#[test]
fn native_repeat_com_gap_length() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("."))]);
    args.named.insert("gap".into(), Value::Length(Length::pt(5.0)));
    let r = native_repeat(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Repeat(e)) = r {
        assert_eq!(e.gap, Some(Length::pt(5.0)));
    } else {
        panic!("esperado Content::Repeat");
    }
}

#[test]
fn native_repeat_com_justify_false() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("."))]);
    args.named.insert("justify".into(), Value::Bool(false));
    let r = native_repeat(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Repeat(e)) = r {
        assert!(!e.justify);
    } else {
        panic!("esperado Content::Repeat");
    }
}

#[test]
fn native_repeat_combina_gap_e_justify() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("o"))]);
    args.named.insert("gap".into(), Value::Length(Length::pt(2.0)));
    args.named.insert("justify".into(), Value::Bool(false));
    let r = native_repeat(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Repeat(e)) = r {
        assert_eq!(e.body.plain_text(), "o");
        assert_eq!(e.gap, Some(Length::pt(2.0)));
        assert!(!e.justify);
    } else {
        panic!("esperado Content::Repeat");
    }
}

#[test]
fn native_repeat_rejeita_sem_body() {
    null_ctx!(ctx);
    let r = native_repeat(&mut ctx, &p(vec![]), &null_world(), test_file_id());
    assert!(r.is_err(), "repeat() sem body deve retornar Err");
}

#[test]
fn native_repeat_rejeita_body_int() {
    null_ctx!(ctx);
    let r =
        native_repeat(&mut ctx, &p(vec![Value::Int(42)]), &null_world(), test_file_id());
    assert!(r.is_err(), "repeat() com body Int deve retornar Err");
}

#[test]
fn native_repeat_rejeita_gap_negativo() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("."))]);
    args.named.insert("gap".into(), Value::Length(Length::pt(-1.0)));
    let r = native_repeat(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "gap negativo deve retornar Err");
}

#[test]
fn native_repeat_rejeita_gap_nao_length() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("."))]);
    args.named.insert("gap".into(), Value::Str("x".into()));
    let r = native_repeat(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "gap não-length deve retornar Err");
}

#[test]
fn native_repeat_rejeita_justify_nao_bool() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("."))]);
    args.named.insert("justify".into(), Value::Int(1));
    let r = native_repeat(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "justify não-bool deve retornar Err");
}

#[test]
fn native_repeat_rejeita_named_arg_desconhecido() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("."))]);
    args.named.insert("count".into(), Value::Int(3));
    let r = native_repeat(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "named arg desconhecido em repeat() deve retornar Err");
}

#[test]
fn native_stack_block_box_pad_hide_continuam_a_funcionar_apos_p156j() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    // Stack
    let args = p(vec![Value::Content(Content::text("a"))]);
    let r = native_stack(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    assert!(matches!(r, Value::Content(Content::Stack(e))));
    // Block
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("width".into(), Value::Length(Length::pt(50.0)));
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    assert!(matches!(r, Value::Content(Content::Block(e))));
    // Box
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("baseline".into(), Value::Length(Length::pt(2.0)));
    let r = native_box(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    assert!(matches!(r, Value::Content(Content::Boxed(e))));
    // Pad
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("rest".into(), Value::Length(Length::pt(2.0)));
    let r = native_pad(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    assert!(matches!(r, Value::Content(Content::Pad(e))));
    // Hide
    let r = native_hide(
        &mut ctx,
        &p(vec![Value::Content(Content::text("y"))]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    assert!(matches!(r, Value::Content(Content::Hide(_))));
}

#[test]
fn p231_native_block_outset_length_aceita() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("body"))]);
    args.named.insert("outset".into(), Value::Length(Length::pt(5.0)));
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Block(e)) = r {
        assert_eq!(e.outset.left, Length::pt(5.0));
    } else {
        panic!("esperado Block");
    }
}

#[test]
fn p231_native_block_outset_negativo_rejeita() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("body"))]);
    args.named.insert("outset".into(), Value::Length(Length::pt(-3.0)));
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "outset negativo deve falhar");
}

#[test]
fn p231_native_block_radius_length_aceita() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("body"))]);
    args.named.insert("radius".into(), Value::Length(Length::pt(3.0)));
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Block(e)) = r {
        // P242 adapta: e.radius `Option<Length>` → `Corners<Length>`.
        // Length uniforme via stdlib → `Corners::uniform`.
        assert_eq!(e.radius.top_left, Length::pt(3.0));
        assert_eq!(e.radius.top_right, Length::pt(3.0));
        assert_eq!(e.radius.bottom_right, Length::pt(3.0));
        assert_eq!(e.radius.bottom_left, Length::pt(3.0));
    } else {
        panic!("esperado Block");
    }
}

#[test]
fn p231_native_block_radius_negativo_rejeita() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("body"))]);
    args.named.insert("radius".into(), Value::Length(Length::pt(-1.0)));
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "radius negativo deve falhar");
}

#[test]
fn p231_native_block_clip_bool_aceita() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("body"))]);
    args.named.insert("clip".into(), Value::Bool(true));
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Block(e)) = r {
        assert_eq!(e.clip, true);
    } else {
        panic!("esperado Block");
    }
}

#[test]
fn p231_native_block_clip_tipo_errado_rejeita() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("body"))]);
    args.named.insert("clip".into(), Value::Str("yes".into()));
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "clip Str deve falhar (espera Bool)");
}

#[test]
fn p231_native_box_paridade_block() {
    // native_box aceita os 3 cosméticos paralelo native_block.
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("body"))]);
    args.named.insert("outset".into(), Value::Length(Length::pt(2.0)));
    args.named.insert("radius".into(), Value::Length(Length::pt(1.0)));
    args.named.insert("clip".into(), Value::Bool(true));
    let r = native_box(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Boxed(e)) = r {
        assert_eq!(e.outset.top, Length::pt(2.0));
        // P242 adapta: e.radius `Corners<Length>` via uniform.
        assert_eq!(e.radius.top_left, Length::pt(1.0));
        assert_eq!(e.clip, true);
    } else {
        panic!("esperado Boxed");
    }
}

#[test]
fn p231_native_block_3_fields_simultaneos() {
    // outset + radius + clip + breakable simultâneos.
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("body"))]);
    args.named.insert("outset".into(), Value::Length(Length::pt(1.0)));
    args.named.insert("radius".into(), Value::Length(Length::pt(2.0)));
    args.named.insert("clip".into(), Value::Bool(true));
    args.named.insert("breakable".into(), Value::Bool(false));
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Block(e)) = r {
        assert_eq!(e.outset.left, Length::pt(1.0));
        // P242 adapta: e.radius `Corners<Length>` via uniform.
        assert_eq!(e.radius.top_left, Length::pt(2.0));
        assert_eq!(e.clip, true);
        assert_eq!(e.breakable, false);
    } else {
        panic!("esperado Block");
    }
}

#[test]
fn p231_native_block_outset_radius_clip_defaults() {
    // Sem args → outset=Sides::uniform(zero); radius=None; clip=false.
    null_ctx!(ctx);
    let r = native_block(
        &mut ctx,
        &p(vec![Value::Content(Content::text("body"))]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    if let Value::Content(Content::Block(e)) = r {
        assert_eq!(e.outset.left, crate::entities::layout_types::Length::ZERO);
        // P242 adapta: e.radius default `Corners::uniform(Length::ZERO)`.
        assert_eq!(e.radius.top_left, crate::entities::layout_types::Length::ZERO);
        assert_eq!(e.radius.top_right, crate::entities::layout_types::Length::ZERO);
        assert_eq!(e.radius.bottom_right, crate::entities::layout_types::Length::ZERO);
        assert_eq!(e.radius.bottom_left, crate::entities::layout_types::Length::ZERO);
        assert_eq!(e.clip, false);
    } else {
        panic!("esperado Block");
    }
}

#[test]
fn p475_block_inset_dict_per_side() {
    use ecow::EcoString;
    use indexmap::IndexMap;
    use rustc_hash::FxBuildHasher;
    null_ctx!(ctx);
    let mut d: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
    d.insert("left".into(), Value::Length(Length::pt(3.0)));
    d.insert("right".into(), Value::Length(Length::pt(5.0)));
    let mut args = p(vec![Value::Content(crate::entities::content::Content::text("x"))]);
    args.named.insert("inset".into(), Value::Dict(d));
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(crate::entities::content::Content::Block(e)) = r {
        assert_eq!(e.inset.left, Length::pt(3.0), "inset.left");
        assert_eq!(e.inset.right, Length::pt(5.0), "inset.right");
        assert_eq!(e.inset.top, Length::ZERO, "inset.top default zero");
        assert_eq!(e.inset.bottom, Length::ZERO, "inset.bottom default zero");
    } else {
        panic!("esperado Block");
    }
}

#[test]
fn p475_block_outset_dict_x_axis() {
    use ecow::EcoString;
    use indexmap::IndexMap;
    use rustc_hash::FxBuildHasher;
    null_ctx!(ctx);
    let mut d: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
    d.insert("x".into(), Value::Length(Length::pt(2.0)));
    let mut args = p(vec![Value::Content(crate::entities::content::Content::text("x"))]);
    args.named.insert("outset".into(), Value::Dict(d));
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(crate::entities::content::Content::Block(e)) = r {
        assert_eq!(e.outset.left, Length::pt(2.0), "outset.left via x");
        assert_eq!(e.outset.right, Length::pt(2.0), "outset.right via x");
        assert_eq!(e.outset.top, Length::ZERO, "outset.top zero (y não declarado)");
    } else {
        panic!("esperado Block");
    }
}

#[test]
fn p475_box_inset_dict_per_side() {
    use ecow::EcoString;
    use indexmap::IndexMap;
    use rustc_hash::FxBuildHasher;
    null_ctx!(ctx);
    let mut d: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
    d.insert("left".into(), Value::Length(Length::pt(3.0)));
    d.insert("top".into(), Value::Length(Length::pt(5.0)));
    let mut args = p(vec![Value::Content(crate::entities::content::Content::text("x"))]);
    args.named.insert("inset".into(), Value::Dict(d));
    let r = native_box(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(crate::entities::content::Content::Boxed(e)) = r {
        assert_eq!(e.inset.left, Length::pt(3.0), "inset.left");
        assert_eq!(e.inset.top, Length::pt(5.0), "inset.top");
        assert_eq!(e.inset.right, Length::ZERO, "inset.right default zero");
        assert_eq!(e.inset.bottom, Length::ZERO, "inset.bottom default zero");
    } else {
        panic!("esperado Boxed");
    }
}

#[test]
fn p475_block_inset_uniforme_ainda_funciona() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(crate::entities::content::Content::text("x"))]);
    args.named.insert("inset".into(), Value::Length(Length::pt(4.0)));
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(crate::entities::content::Content::Block(e)) = r {
        assert_eq!(e.inset.left, Length::pt(4.0), "inset uniforme left");
        assert_eq!(e.inset.right, Length::pt(4.0), "inset uniforme right");
        assert_eq!(e.inset.top, Length::pt(4.0), "inset uniforme top");
    } else {
        panic!("esperado Block");
    }
}

#[test]
fn p475_relative_aceite_em_extract_length_parte_abs() {
    use crate::entities::rel::Rel;
    null_ctx!(ctx);
    // Rel { rel: 0.5, abs: 2pt } → extract_length → Some(2pt); parte rel truncada.
    let rel = Rel { rel: 0.5, abs: Length::pt(2.0) };
    let mut args = p(vec![Value::Content(crate::entities::content::Content::text("x"))]);
    args.named.insert("inset".into(), Value::Relative(rel));
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(crate::entities::content::Content::Block(e)) = r {
        assert_eq!(
            e.inset.left,
            Length::pt(2.0),
            "Relative {{ rel: 0.5, abs: 2pt }} → inset = 2pt (rel truncado)"
        );
    } else {
        panic!("esperado Block");
    }
}

#[test]
fn p475_block_inset_dict_chave_invalida_erro() {
    use ecow::EcoString;
    use indexmap::IndexMap;
    use rustc_hash::FxBuildHasher;
    null_ctx!(ctx);
    let mut d: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
    d.insert("diagonal".into(), Value::Length(Length::pt(1.0)));
    let mut args = p(vec![Value::Content(crate::entities::content::Content::text("x"))]);
    args.named.insert("inset".into(), Value::Dict(d));
    let r = native_block(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "chave desconhecida no dict de inset deve dar erro");
}
