//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/tests/layout_spacing.md
//! @prompt-hash 53f549c9
//! @layer L1
//! @updated 2026-09-26

use super::*;

#[test]
fn native_pad_defaults_sides_none() {
    // P156L: sem named args → todos os sides são None (per
    // ADR-0064 Caso C; None ↔ default vanilla zero resolvido em uso).
    null_ctx!(ctx);
    let body = Content::text("body");
    let args = p(vec![Value::Content(body)]);
    let result = native_pad(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Pad(e)) = result {
        assert_eq!(e.body.plain_text(), "body");
        assert_eq!(e.sides.left, None);
        assert_eq!(e.sides.right, None);
        assert_eq!(e.sides.top, None);
        assert_eq!(e.sides.bottom, None);
    } else {
        panic!("esperado Content::Pad");
    }
}

#[test]
fn native_pad_lados_individuais() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("left".into(), Value::Length(Length::pt(1.0)));
    args.named.insert("right".into(), Value::Length(Length::pt(2.0)));
    args.named.insert("top".into(), Value::Length(Length::pt(3.0)));
    args.named.insert("bottom".into(), Value::Length(Length::pt(4.0)));
    let result = native_pad(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Pad(e)) = result {
        assert_eq!(e.sides.left, Some(Length::pt(1.0)));
        assert_eq!(e.sides.right, Some(Length::pt(2.0)));
        assert_eq!(e.sides.top, Some(Length::pt(3.0)));
        assert_eq!(e.sides.bottom, Some(Length::pt(4.0)));
    } else {
        panic!("esperado Content::Pad");
    }
}

#[test]
fn native_pad_atalhos_x_e_y() {
    // x cobre left+right; y cobre top+bottom.
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("x".into(), Value::Length(Length::pt(5.0)));
    args.named.insert("y".into(), Value::Length(Length::pt(7.0)));
    let result = native_pad(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Pad(e)) = result {
        assert_eq!(e.sides.left, Some(Length::pt(5.0)));
        assert_eq!(e.sides.right, Some(Length::pt(5.0)));
        assert_eq!(e.sides.top, Some(Length::pt(7.0)));
        assert_eq!(e.sides.bottom, Some(Length::pt(7.0)));
    } else {
        panic!("esperado Content::Pad");
    }
}

#[test]
fn native_pad_atalho_rest() {
    // rest cobre todos os 4 lados.
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("rest".into(), Value::Length(Length::pt(8.0)));
    let result = native_pad(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Pad(e)) = result {
        assert_eq!(e.sides.left, Some(Length::pt(8.0)));
        assert_eq!(e.sides.right, Some(Length::pt(8.0)));
        assert_eq!(e.sides.top, Some(Length::pt(8.0)));
        assert_eq!(e.sides.bottom, Some(Length::pt(8.0)));
    } else {
        panic!("esperado Content::Pad");
    }
}

#[test]
fn native_pad_precedencia_especifico_eixo_rest() {
    // left explícito sobrepõe-se a x; x sobrepõe-se a rest.
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("left".into(), Value::Length(Length::pt(1.0)));
    args.named.insert("x".into(), Value::Length(Length::pt(2.0)));
    args.named.insert("rest".into(), Value::Length(Length::pt(3.0)));
    let result = native_pad(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Pad(e)) = result {
        // left vence (específico)
        assert_eq!(e.sides.left, Some(Length::pt(1.0)));
        // right cai para x (eixo)
        assert_eq!(e.sides.right, Some(Length::pt(2.0)));
        // top cai para rest (não há y nem específico)
        assert_eq!(e.sides.top, Some(Length::pt(3.0)));
        assert_eq!(e.sides.bottom, Some(Length::pt(3.0)));
    } else {
        panic!("esperado Content::Pad");
    }
}

#[test]
fn native_pad_rejeita_padding_negativo() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("left".into(), Value::Length(Length::pt(-1.0)));
    let result = native_pad(&mut ctx, &args, &null_world(), test_file_id());
    assert!(result.is_err(), "padding negativo deve retornar Err em P156C/L");
}

#[test]
fn native_pad_rejeita_named_arg_desconhecido() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("strange".into(), Value::Length(Length::pt(1.0)));
    let result = native_pad(&mut ctx, &args, &null_world(), test_file_id());
    assert!(result.is_err(), "named arg desconhecido em pad() deve retornar Err");
}

#[test]
fn native_pad_aceita_int_e_float_como_pt() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("rest".into(), Value::Float(2.5));
    let r = native_pad(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Pad(e)) = r {
        assert_eq!(e.sides.left, Some(Length::pt(2.5)));
    } else {
        panic!("esperado Content::Pad");
    }
}

#[test]
fn native_pad_p156l_apenas_um_lado_outros_none() {
    // P156L: declarar só `top` deixa os outros 3 sides como None
    // (distinção semântica vs P156C onde ficavam Length::ZERO).
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("top".into(), Value::Length(Length::pt(7.0)));
    let r = native_pad(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Pad(e)) = r {
        assert_eq!(e.sides.top, Some(Length::pt(7.0)));
        assert_eq!(e.sides.left, None);
        assert_eq!(e.sides.right, None);
        assert_eq!(e.sides.bottom, None);
    } else {
        panic!("esperado Content::Pad");
    }
}

#[test]
fn native_pad_p156l_some_zero_distinct_from_none() {
    // P156L: declarar `left: 0pt` produz Some(zero), distinto de
    // não declarar (None). Per ADR-0064 Caso C distinção semântica.
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("left".into(), Value::Length(Length::ZERO));
    let r = native_pad(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Pad(e)) = r {
        assert_eq!(
            e.sides.left,
            Some(Length::ZERO),
            "left explicitamente declarado a zero é Some(ZERO), não None"
        );
        assert_eq!(e.sides.right, None, "right não declarado é None, não Some(ZERO)");
    } else {
        panic!("esperado Content::Pad");
    }
}

#[test]
fn native_pad_p156l_x_axis_apenas() {
    // Atalho `x` declara left+right; top/bottom ficam None.
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("x".into(), Value::Length(Length::pt(4.0)));
    let r = native_pad(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Pad(e)) = r {
        assert_eq!(e.sides.left, Some(Length::pt(4.0)));
        assert_eq!(e.sides.right, Some(Length::pt(4.0)));
        assert_eq!(e.sides.top, None);
        assert_eq!(e.sides.bottom, None);
    } else {
        panic!("esperado Content::Pad");
    }
}

#[test]
fn native_pad_p156l_top_overrides_y_overrides_rest() {
    // Cadeia de precedência: top > y > rest. Decorações:
    // top=10, y=20, rest=30 → top=10 (especifico), bottom=20 (eixo).
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("top".into(), Value::Length(Length::pt(10.0)));
    args.named.insert("y".into(), Value::Length(Length::pt(20.0)));
    args.named.insert("rest".into(), Value::Length(Length::pt(30.0)));
    let r = native_pad(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Pad(e)) = r {
        assert_eq!(e.sides.top, Some(Length::pt(10.0)), "top específico vence y e rest");
        assert_eq!(
            e.sides.bottom,
            Some(Length::pt(20.0)),
            "bottom cai para y (sem específico)"
        );
        assert_eq!(
            e.sides.left,
            Some(Length::pt(30.0)),
            "left cai para rest (sem específico nem x)"
        );
        assert_eq!(e.sides.right, Some(Length::pt(30.0)));
    } else {
        panic!("esperado Content::Pad");
    }
}

#[test]
fn native_pad_sem_body_retorna_err() {
    null_ctx!(ctx);
    let result = native_pad(&mut ctx, &p(vec![]), &null_world(), test_file_id());
    assert!(result.is_err(), "pad() sem body deve retornar Err");
}

#[test]
fn native_hide_envolve_body() {
    null_ctx!(ctx);
    let body = Content::text("invisivel");
    let args = p(vec![Value::Content(body)]);
    let result = native_hide(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Hide(e)) = result {
        assert_eq!(e.body.plain_text(), "invisivel");
    } else {
        panic!("esperado Content::Hide");
    }
}

#[test]
fn native_hide_aceita_string() {
    null_ctx!(ctx);
    let args = p(vec![Value::Str("placeholder".into())]);
    let result = native_hide(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    assert!(matches!(result, Value::Content(Content::Hide(_))));
}

#[test]
fn native_hide_rejeita_named_arg() {
    null_ctx!(ctx);
    let args = pn(vec![Value::Content(Content::text("x"))], "weak", Value::Bool(true));
    let result = native_hide(&mut ctx, &args, &null_world(), test_file_id());
    assert!(result.is_err(), "hide() não aceita named args (P156C)");
}

#[test]
fn native_hide_sem_body_retorna_err() {
    null_ctx!(ctx);
    let result = native_hide(&mut ctx, &p(vec![]), &null_world(), test_file_id());
    assert!(result.is_err(), "hide() sem body deve retornar Err");
}

#[test]
fn native_h_aceita_length() {
    null_ctx!(ctx);
    use crate::entities::elements::h_space::Spacing;
    use crate::entities::layout_types::Length;
    let args = p(vec![Value::Length(Length::pt(12.0))]);
    let r = native_h(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::HSpace(e)) = r {
        assert_eq!(e.amount, Spacing::Absolute(Length::pt(12.0)));
        assert!(!e.weak); // default
    } else {
        panic!("esperado Content::HSpace");
    }
}

#[test]
fn native_h_aceita_int_e_float_como_pt() {
    null_ctx!(ctx);
    use crate::entities::elements::h_space::Spacing;
    use crate::entities::layout_types::Length;
    // Int interpretado em pt.
    let r = native_h(&mut ctx, &p(vec![Value::Int(5)]), &null_world(), test_file_id())
        .unwrap();
    if let Value::Content(Content::HSpace(e)) = r {
        assert_eq!(e.amount, Spacing::Absolute(Length::pt(5.0)));
    } else {
        panic!("esperado Content::HSpace");
    }
    // Float interpretado em pt.
    let r =
        native_h(&mut ctx, &p(vec![Value::Float(2.5)]), &null_world(), test_file_id())
            .unwrap();
    if let Value::Content(Content::HSpace(e)) = r {
        assert_eq!(e.amount, Spacing::Absolute(Length::pt(2.5)));
    } else {
        panic!("esperado Content::HSpace");
    }
}

#[test]
fn native_h_aceita_weak_true() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Length(Length::pt(3.0))]);
    args.named.insert("weak".into(), Value::Bool(true));
    let r = native_h(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::HSpace(e)) = r {
        assert!(e.weak);
    } else {
        panic!("esperado Content::HSpace");
    }
}

#[test]
fn native_h_aceita_amount_zero() {
    null_ctx!(ctx);
    use crate::entities::elements::h_space::Spacing;
    use crate::entities::layout_types::Length;
    let r = native_h(
        &mut ctx,
        &p(vec![Value::Length(Length::ZERO)]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    if let Value::Content(Content::HSpace(e)) = r {
        assert_eq!(e.amount, Spacing::Absolute(Length::ZERO));
    } else {
        panic!("esperado Content::HSpace");
    }
}

#[test]
fn p842_l7_native_h_aceita_fraction() {
    // Medido no vanilla (`temp/p842/l7_h_1fr.typ`): `#h(1fr)` compila e
    // distribui o espaço restante da linha proporcionalmente.
    null_ctx!(ctx);
    use crate::entities::elements::h_space::Spacing;
    let r =
        native_h(&mut ctx, &p(vec![Value::Fraction(1.0)]), &null_world(), test_file_id())
            .unwrap();
    if let Value::Content(Content::HSpace(e)) = r {
        assert_eq!(e.amount, Spacing::Fractional(1.0));
        assert!(!e.weak);
    } else {
        panic!("esperado Content::HSpace");
    }
}

#[test]
fn p842_l7_native_v_rejeita_fraction_com_mensagem_previa() {
    // v(1fr) fica fora de escopo em P842 (distribuição vertical é outro
    // mecanismo) — a rejeição pré-P842 preserva-se verbatim.
    null_ctx!(ctx);
    let r =
        native_v(&mut ctx, &p(vec![Value::Fraction(1.0)]), &null_world(), test_file_id());
    match r {
        Err(diags) => assert!(
            diags[0]
                .message
                .contains("v() espera amount como length, recebeu fraction"),
            "mensagem inesperada: {}",
            diags[0].message
        ),
        Ok(v) => panic!("v(1fr) devia ser rejeitado em P842, obteve {v:?}"),
    }
}

#[test]
fn native_h_rejeita_amount_negativo() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let r = native_h(
        &mut ctx,
        &p(vec![Value::Length(Length::pt(-1.0))]),
        &null_world(),
        test_file_id(),
    );
    assert!(r.is_err(), "amount negativo deve retornar Err em P156D");
}

#[test]
fn native_h_rejeita_named_arg_desconhecido() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Length(Length::pt(1.0))]);
    args.named.insert("attached".into(), Value::Bool(true));
    let r = native_h(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "named arg desconhecido em h() deve retornar Err");
}

#[test]
fn native_h_rejeita_weak_nao_bool() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Length(Length::pt(1.0))]);
    args.named.insert("weak".into(), Value::Int(1)); // tipo errado
    let r = native_h(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "weak não-bool deve retornar Err");
}

#[test]
fn native_h_sem_amount_retorna_err() {
    null_ctx!(ctx);
    let r = native_h(&mut ctx, &p(vec![]), &null_world(), test_file_id());
    assert!(r.is_err(), "h() sem amount deve retornar Err");
}

#[test]
fn native_v_aceita_length_e_weak() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    // Caso composto: amount + weak; cobre paths principais de native_v
    // (que partilha lógica com native_h via build_spacing).
    let mut args = p(vec![Value::Length(Length::pt(15.0))]);
    args.named.insert("weak".into(), Value::Bool(true));
    let r = native_v(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::VSpace(e)) = r {
        assert_eq!(e.amount, Length::pt(15.0));
        assert!(e.weak);
    } else {
        panic!("esperado Content::VSpace");
    }
}

#[test]
fn native_v_rejeita_amount_negativo() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let r = native_v(
        &mut ctx,
        &p(vec![Value::Length(Length::pt(-2.0))]),
        &null_world(),
        test_file_id(),
    );
    assert!(r.is_err(), "amount negativo deve retornar Err em P156D");
}

#[test]
fn native_v_sem_amount_retorna_err() {
    null_ctx!(ctx);
    let r = native_v(&mut ctx, &p(vec![]), &null_world(), test_file_id());
    assert!(r.is_err(), "v() sem amount deve retornar Err");
}

#[test]
fn native_pagebreak_defaults() {
    null_ctx!(ctx);
    let r =
        native_pagebreak(&mut ctx, &p(vec![]), &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Pagebreak(e)) = r {
        assert!(!e.weak);
        assert_eq!(e.to, None);
    } else {
        panic!("esperado Content::Pagebreak");
    }
}

#[test]
fn native_pagebreak_com_weak_true() {
    null_ctx!(ctx);
    let mut args = p(vec![]);
    args.named.insert("weak".into(), Value::Bool(true));
    let r = native_pagebreak(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Pagebreak(e)) = r {
        assert!(e.weak);
    } else {
        panic!("esperado Content::Pagebreak");
    }
}

#[test]
fn native_pagebreak_com_to_even() {
    null_ctx!(ctx);
    use crate::entities::parity::Parity;
    let mut args = p(vec![]);
    args.named.insert("to".into(), Value::Str("even".into()));
    let r = native_pagebreak(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Pagebreak(e)) = r {
        assert_eq!(e.to, Some(Parity::Even));
    } else {
        panic!("esperado Content::Pagebreak");
    }
}

#[test]
fn native_pagebreak_com_to_odd() {
    null_ctx!(ctx);
    use crate::entities::parity::Parity;
    let mut args = p(vec![]);
    args.named.insert("to".into(), Value::Str("odd".into()));
    let r = native_pagebreak(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Pagebreak(e)) = r {
        assert_eq!(e.to, Some(Parity::Odd));
    } else {
        panic!("esperado Content::Pagebreak");
    }
}

#[test]
fn native_pagebreak_combina_weak_e_to() {
    null_ctx!(ctx);
    use crate::entities::parity::Parity;
    let mut args = p(vec![]);
    args.named.insert("weak".into(), Value::Bool(true));
    args.named.insert("to".into(), Value::Str("even".into()));
    let r = native_pagebreak(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Pagebreak(e)) = r {
        assert!(e.weak);
        assert_eq!(e.to, Some(Parity::Even));
    } else {
        panic!("esperado Content::Pagebreak");
    }
}

#[test]
fn native_pagebreak_rejeita_to_invalido() {
    null_ctx!(ctx);
    let mut args = p(vec![]);
    args.named.insert("to".into(), Value::Str("middle".into()));
    let r = native_pagebreak(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "pagebreak(to: \"middle\") deve retornar Err");
}

#[test]
fn native_pagebreak_rejeita_to_nao_string() {
    null_ctx!(ctx);
    let mut args = p(vec![]);
    args.named.insert("to".into(), Value::Int(2));
    let r = native_pagebreak(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "pagebreak(to:) com tipo errado deve retornar Err");
}

#[test]
fn native_pagebreak_rejeita_named_arg_desconhecido() {
    null_ctx!(ctx);
    let mut args = p(vec![]);
    args.named.insert("strange".into(), Value::Bool(false));
    let r = native_pagebreak(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "named arg desconhecido em pagebreak() deve retornar Err");
}

#[test]
fn native_pagebreak_rejeita_argumento_posicional() {
    null_ctx!(ctx);
    let r = native_pagebreak(
        &mut ctx,
        &p(vec![Value::Bool(true)]),
        &null_world(),
        test_file_id(),
    );
    assert!(r.is_err(), "pagebreak() não aceita argumentos posicionais");
}

#[test]
fn native_pagebreak_rejeita_weak_nao_bool() {
    null_ctx!(ctx);
    let mut args = p(vec![]);
    args.named.insert("weak".into(), Value::Int(1));
    let r = native_pagebreak(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "weak não-Bool deve retornar Err");
}
