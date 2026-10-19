//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/tests/math.md
//! @prompt-hash 0888b713
//! @layer L1
//! @updated 2026-09-26

use super::*;
use crate::entities::math_style::MathStyleKind;

#[test]
fn p296_native_accent_base_e_accent_posicionais() {
    use super::native_accent;
    null_ctx!(ctx);
    let r = native_accent(
        &mut ctx,
        &p(vec![
            Value::Content(Content::MathIdent("a".into())),
            Value::Content(Content::MathText("^".into())),
        ]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    if let Value::Content(Content::MathAccent(e)) = r {
        assert_eq!(e.base.plain_text(), "a");
        assert_eq!(e.accent, '\u{0302}');
    } else {
        panic!("esperava Content::MathAccent");
    }
}

#[test]
fn p296_native_accent_strings_convertidas_para_text() {
    use super::native_accent;
    null_ctx!(ctx);
    let r = native_accent(
        &mut ctx,
        &p(vec![Value::Str("x".into()), Value::Str("~".into())]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    if let Value::Content(Content::MathAccent(e)) = r {
        assert_eq!(e.base.plain_text(), "x");
        assert_eq!(e.accent, '\u{0303}');
    }
}

#[test]
fn p296_native_accent_sem_base_retorna_err() {
    use super::native_accent;
    null_ctx!(ctx);
    let r = native_accent(&mut ctx, &p(vec![]), &null_world(), test_file_id());
    assert!(r.is_err());
    assert!(format!("{:?}", r).contains("base"));
}

#[test]
fn p296_native_accent_sem_accent_retorna_err() {
    use super::native_accent;
    null_ctx!(ctx);
    let r = native_accent(
        &mut ctx,
        &p(vec![Value::Str("a".into())]),
        &null_world(),
        test_file_id(),
    );
    assert!(r.is_err());
    assert!(format!("{:?}", r).contains("accent"));
}

#[test]
fn p296_native_accent_named_arg_rejeitado() {
    use super::native_accent;
    null_ctx!(ctx);
    let mut args = p(vec![Value::Str("a".into()), Value::Str("^".into())]);
    args.named.insert("size".into(), Value::Float(0.5));
    let r = native_accent(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "size/dotless cosméticos scope-out P296");
}

#[test]
fn p296_native_cancel_body_posicional() {
    use super::native_cancel;
    null_ctx!(ctx);
    let r = native_cancel(
        &mut ctx,
        &p(vec![Value::Content(Content::MathIdent("x".into()))]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    if let Value::Content(Content::MathCancel(e)) = r {
        assert_eq!(e.body.plain_text(), "x");
    } else {
        panic!("esperava Content::MathCancel");
    }
}

#[test]
fn p296_native_cancel_body_string_convertido() {
    use super::native_cancel;
    null_ctx!(ctx);
    let r = native_cancel(
        &mut ctx,
        &p(vec![Value::Str("y".into())]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    if let Value::Content(Content::MathCancel(e)) = r {
        assert_eq!(e.body.plain_text(), "y");
    }
}

#[test]
fn p296_native_cancel_sem_body_retorna_err() {
    use super::native_cancel;
    null_ctx!(ctx);
    let r = native_cancel(&mut ctx, &p(vec![]), &null_world(), test_file_id());
    assert!(r.is_err());
    assert!(format!("{:?}", r).contains("body"));
}

#[test]
fn p1291_native_cancel_named_angle_cross_e_desconhecido() {
    use super::native_cancel;
    null_ctx!(ctx);
    let mut args = p(vec![Value::Str("x".into())]);
    args.named.insert("angle".into(), Value::Angle(Angle::deg(17.0)));
    args.named.insert("cross".into(), Value::Bool(true));
    let r = native_cancel(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    let Value::Content(Content::MathCancel(cancel)) = r else {
        panic!("esperava Content::MathCancel");
    };
    assert!(cancel.cross, "cross nomeado deve ser preservado");
    assert!(
        matches!(
            &cancel.angle,
            crate::entities::elements::math_cancel::MathCancelAngle::Angle(angle)
                if (angle.to_deg() - 17.0).abs() < 1e-9
        ),
        "angle nomeado deve ser preservado"
    );

    let mut unknown = p(vec![Value::Str("x".into())]);
    unknown.named.insert("nope".into(), Value::Bool(true));
    let error = native_cancel(&mut ctx, &unknown, &null_world(), test_file_id())
        .expect_err("named desconhecido deve continuar rejeitado");
    assert!(format!("{error:?}").contains("unexpected argument: nope"));
}

#[test]
fn p296_math_accent_partial_eq_e_is_empty() {
    let a = Content::math_accent(Content::MathIdent("a".into()), '^');
    let b = a.clone();
    assert_eq!(a, b);
    // is_empty fallback é false (math structural sempre observable).
    assert!(!a.is_empty());
}

#[test]
fn p296_math_cancel_partial_eq() {
    let a = Content::math_cancel(Content::MathIdent("x".into()));
    let b = Content::math_cancel(Content::MathIdent("x".into()));
    let c = Content::math_cancel(Content::MathIdent("y".into()));
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn p297_native_underover_so_base_posicional() {
    use super::native_underover;
    null_ctx!(ctx);
    let r = native_underover(
        &mut ctx,
        &p(vec![Value::Content(Content::MathIdent("x".into()))]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    if let Value::Content(Content::MathUnderover(e)) = r {
        assert_eq!(e.base.plain_text(), "x");
        assert!(e.under.is_none(), "under None se não fornecido");
        assert!(e.over.is_none(), "over None se não fornecido");
    } else {
        panic!("esperava Content::MathUnderover");
    }
}

#[test]
fn p297_native_underover_com_under_named() {
    use super::native_underover;
    null_ctx!(ctx);
    let mut args = p(vec![Value::Str("base".into())]);
    args.named.insert("under".into(), Value::Str("u".into()));
    let r = native_underover(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::MathUnderover(e)) = r {
        assert!(e.under.is_some());
        assert!(e.over.is_none());
        assert_eq!(e.under.as_ref().unwrap().plain_text(), "u");
    }
}

#[test]
fn p297_native_underover_com_over_named() {
    use super::native_underover;
    null_ctx!(ctx);
    let mut args = p(vec![Value::Str("base".into())]);
    args.named.insert("over".into(), Value::Str("o".into()));
    let r = native_underover(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::MathUnderover(e)) = r {
        assert!(e.under.is_none());
        assert!(e.over.is_some());
        assert_eq!(e.over.as_ref().unwrap().plain_text(), "o");
    }
}

#[test]
fn p297_native_underover_com_ambos() {
    use super::native_underover;
    null_ctx!(ctx);
    let mut args = p(vec![Value::Str("b".into())]);
    args.named.insert("under".into(), Value::Str("u".into()));
    args.named.insert("over".into(), Value::Str("o".into()));
    let r = native_underover(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::MathUnderover(e)) = r {
        assert_eq!(e.base.plain_text(), "b");
        assert_eq!(e.under.as_ref().unwrap().plain_text(), "u");
        assert_eq!(e.over.as_ref().unwrap().plain_text(), "o");
    }
}

#[test]
fn p297_native_underover_sem_base_retorna_err() {
    use super::native_underover;
    null_ctx!(ctx);
    let r = native_underover(&mut ctx, &p(vec![]), &null_world(), test_file_id());
    assert!(r.is_err());
    assert!(format!("{:?}", r).contains("base"));
}

#[test]
fn p297_native_underover_named_arg_invalido_retorna_err() {
    use super::native_underover;
    null_ctx!(ctx);
    let mut args = p(vec![Value::Str("b".into())]);
    args.named.insert("style".into(), Value::Str("x".into()));
    let r = native_underover(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "named args além de under/over rejeitados");
}

#[test]
fn p297_math_underover_plain_text_ordem_visual() {
    // Ordem visual: over + base + under.
    let u = Content::math_underover(
        Content::text("BASE"),
        Some(Content::text("U")),
        Some(Content::text("O")),
    );
    assert_eq!(u.plain_text(), "OBASEU");
}

#[test]
fn p297_math_underover_partial_eq_structural() {
    let a = Content::math_underover(Content::text("x"), Some(Content::text("u")), None);
    let b = a.clone();
    assert_eq!(a, b);
    let c = Content::math_underover(Content::text("x"), None, None); // diferente em under
    assert_ne!(a, c);
}

#[test]
fn p297_math_underover_ambos_none_equivale_so_base() {
    // Caso degenerate: ambos None → plain_text == base.
    let u = Content::math_underover(Content::text("x"), None, None);
    assert_eq!(u.plain_text(), "x");
}

#[test]
fn p298_native_op_text_posicional_default_limits_false() {
    use super::native_op;
    null_ctx!(ctx);
    let r = with_engine(|engine, world| {
        native_op(
            &mut ctx,
            &p(vec![Value::Str("lim".into())]),
            world,
            test_file_id(),
            &mut Scopes::new(None),
            engine,
        )
    })
    .unwrap();
    if let Value::Content(Content::MathOp(e)) = r {
        assert_eq!(e.text.plain_text(), "lim");
        assert!(!e.limits, "limits default false");
    } else {
        panic!("esperava Content::MathOp");
    }
}

#[test]
fn p298_native_op_com_limits_true_named() {
    use super::native_op;
    null_ctx!(ctx);
    let mut args = p(vec![Value::Str("lim".into())]);
    args.named.insert("limits".into(), Value::Bool(true));
    let r = with_engine(|engine, world| {
        native_op(&mut ctx, &args, world, test_file_id(), &mut Scopes::new(None), engine)
    })
    .unwrap();
    if let Value::Content(Content::MathOp(e)) = r {
        assert!(e.limits, "limits=true respeitado");
    }
}

#[test]
fn p298_native_op_text_content_preservado() {
    use super::native_op;
    null_ctx!(ctx);
    let r = with_engine(|engine, world| {
        native_op(
            &mut ctx,
            &p(vec![Value::Content(Content::MathIdent("Σ".into()))]),
            world,
            test_file_id(),
            &mut Scopes::new(None),
            engine,
        )
    })
    .unwrap();
    if let Value::Content(Content::MathOp(e)) = r {
        // Content posicional preservado estructuralmente.
        assert_eq!(e.text.plain_text(), "Σ");
    }
}

#[test]
fn p298_native_op_sem_text_retorna_err() {
    use super::native_op;
    null_ctx!(ctx);
    let r = with_engine(|engine, world| {
        native_op(
            &mut ctx,
            &p(vec![]),
            world,
            test_file_id(),
            &mut Scopes::new(None),
            engine,
        )
    });
    assert!(r.is_err());
    assert!(format!("{:?}", r).contains("text"));
}

#[test]
fn p298_native_op_named_invalido_retorna_err() {
    use super::native_op;
    null_ctx!(ctx);
    let mut args = p(vec![Value::Str("lim".into())]);
    args.named.insert("style".into(), Value::Str("italic".into()));
    let r = with_engine(|engine, world| {
        native_op(&mut ctx, &args, world, test_file_id(), &mut Scopes::new(None), engine)
    });
    assert!(r.is_err(), "só 'limits' permitido");
}

#[test]
fn p298_native_op_limits_nao_bool_retorna_err() {
    use super::native_op;
    null_ctx!(ctx);
    let mut args = p(vec![Value::Str("lim".into())]);
    args.named.insert("limits".into(), Value::Int(1));
    let r = with_engine(|engine, world| {
        native_op(&mut ctx, &args, world, test_file_id(), &mut Scopes::new(None), engine)
    });
    assert!(r.is_err(), "limits espera bool");
}

#[test]
fn p298_math_op_partial_eq_structural_com_limits() {
    let a = Content::math_op(Content::text("lim"), true);
    let b = a.clone();
    assert_eq!(a, b);
    // limits diferente → desigual.
    let c = Content::math_op(Content::text("lim"), false);
    assert_ne!(a, c);
}

#[test]
fn p298_math_op_plain_text_so_text_sem_limits() {
    let o = Content::math_op(Content::text("lim"), true);
    // limits é discriminador layout — plain_text só retorna text.
    assert_eq!(o.plain_text(), "lim");
}

#[test]
fn p299_math_module_contem_sin_scripts_style() {
    let v = lookup_math("sin").expect("math.sin deve existir");
    if let Value::Content(Content::MathOp(e)) = v {
        assert_eq!(e.text.plain_text(), "sin");
        assert!(!e.limits, "sin é scripts-style (limits=false)");
    } else {
        panic!("math.sin deve ser MathOp");
    }
}

#[test]
fn p299_math_module_contem_lim_limits_style() {
    let v = lookup_math("lim").expect("math.lim deve existir");
    if let Value::Content(Content::MathOp(e)) = v {
        assert_eq!(e.text.plain_text(), "lim");
        assert!(e.limits, "lim é limits-style (limits=true)");
    }
}

#[test]
fn p299_math_module_contem_det_limits_style_novo() {
    // det NÃO estava em is_limit_function pré-P299 hardcoded.
    // P299 acrescenta via registo explícito.
    let v = lookup_math("det").expect("math.det deve existir");
    if let Value::Content(Content::MathOp(e)) = v {
        assert_eq!(e.text.plain_text(), "det");
        assert!(e.limits, "det é limits-style vanilla");
    }
}

#[test]
fn p299_math_module_contem_liminf_multi_word_text() {
    // liminf vanila → text com espaço fino U+2009 (multi-word).
    let v = lookup_math("liminf").expect("math.liminf deve existir");
    if let Value::Content(Content::MathOp(e)) = v {
        assert_eq!(
            e.text.plain_text(),
            "lim\u{2009}inf",
            "multi-word: name 'liminf' mapeia para text com U+2009"
        );
        assert!(e.limits);
    }
}

#[test]
fn p299_math_module_contem_limsup_multi_word_text() {
    let v = lookup_math("limsup").expect("math.limsup deve existir");
    if let Value::Content(Content::MathOp(e)) = v {
        // O separador morfológico é espaço fino U+2009.
        assert_eq!(e.text.plain_text(), "lim\u{2009}sup");
        assert!(e.limits);
    }
}

#[test]
fn p299_math_module_total_42_operadores() {
    use super::make_math_module;
    if let Value::Module(m) = make_math_module() {
        assert!(m.scope().len() >= 52, "P299+: operadores + simbolos herdados de sym");
        assert!(m.scope().get("sin").is_some());
        assert!(m.scope().get("planck").is_some());
    } else {
        panic!("make_math_module deve retornar Value::Module");
    }
}

#[test]
fn p299_math_module_todos_scripts_style_limits_false() {
    // Amostra de scripts-style: sin/cos/tan/ln/log/exp todos limits=false.
    for name in ["sin", "cos", "tan", "ln", "log", "exp", "arccos"] {
        let v = lookup_math(name).expect(name);
        if let Value::Content(Content::MathOp(e)) = v {
            assert!(!e.limits, "{} deve ser scripts-style", name);
        } else {
            panic!("{} deve ser MathOp", name);
        }
    }
}

#[test]
fn p299_math_module_pr_case_sensitive() {
    // vanilla `Pr` (probabilidade) é case-sensitive — capitalizado.
    let v = lookup_math("Pr").expect("math.Pr deve existir");
    if let Value::Content(Content::MathOp(e)) = v {
        assert_eq!(e.text.plain_text(), "Pr");
        assert!(e.limits, "Pr é limits-style vanilla");
    }
    // 'pr' minúsculo NÃO existe.
    assert!(lookup_math("pr").is_none(), "case sensitive: 'pr' minúsculo não existe");
}

#[test]
fn p299_math_module_nome_inexistente_retorna_none() {
    assert!(lookup_math("foo_bar_qux").is_none());
}

#[test]
fn p299_regressao_math_ident_lim_preservado_pre_p299() {
    // Heurística pré-P299 (is_limit_function) hardcoded preservada.
    // MathIdent("lim") sem usar math.lim continua a funcionar.
    // Este teste NÃO chama math.lim — só verifica que MathIdent existe.
    let m = Content::MathIdent("lim".into());
    // Não panic; pode usar plain_text e estar bem formado.
    assert_eq!(m.plain_text(), "lim");
}

#[test]
fn p311b_bb_wraps_double_struck() {
    let v =
        call_math_style(native_bb, vec![Value::Content(Content::MathIdent("x".into()))]);
    assert_styled(v, Some(MathStyleKind::DoubleStruck), None, None, None);
}

#[test]
fn p311b_bold_is_orthogonal_flag() {
    let v = call_math_style(
        native_bold,
        vec![Value::Content(Content::MathIdent("x".into()))],
    );
    assert_styled(v, None, Some(true), None, None);
}

#[test]
fn p311b_cal_wraps_chancery() {
    let v =
        call_math_style(native_cal, vec![Value::Content(Content::MathIdent("L".into()))]);
    assert_styled(v, Some(MathStyleKind::Chancery), None, None, None);
}

#[test]
fn p311b_frak_wraps_fraktur() {
    let v = call_math_style(
        native_frak,
        vec![Value::Content(Content::MathIdent("g".into()))],
    );
    assert_styled(v, Some(MathStyleKind::Fraktur), None, None, None);
}

#[test]
fn p811_frak_sem_argumento_erro_missing_body() {
    // P811 — paridade vanilla medida: `$ frak() $` →
    // `error: missing argument: body`. Antes: `Content::Empty`
    // silencioso, que deixava uma equação vazia chegar ao export e
    // produzia um PDF inválido (kid inexistente no /Pages).
    let v = call_math_style(native_frak, vec![]);
    let err = v.unwrap_err();
    assert!(
        format!("{:?}", err).contains("missing argument: body"),
        "mensagem literal do vanilla, obtido: {:?}",
        err
    );
    // Controlo: com body continua a funcionar.
    let ok = call_math_style(
        native_frak,
        vec![Value::Content(Content::MathIdent("A".into()))],
    );
    assert!(ok.is_ok(), "frak(A) deve continuar a funcionar");
}

#[test]
fn p811_bb_sem_argumento_erro_missing_body() {
    // Mesma validação partilhada (`wrap_math_style`) — cobre as 12 funções.
    let v = call_math_style(native_bb, vec![]);
    let err = v.unwrap_err();
    assert!(
        format!("{:?}", err).contains("missing argument: body"),
        "mensagem literal do vanilla, obtido: {:?}",
        err
    );
}

#[test]
fn p899_bb_com_argumento_string_produz_mathtext_nao_text() {
    let v = call_math_style(native_bb, vec![Value::Str("R".into())]);
    match v.unwrap() {
        Value::Content(Content::MathStyled(m)) => {
            assert!(
                matches!(&m.body, Content::MathText(s) if s.as_str() == "R"),
                "bb(\"R\") deve produzir corpo Content::MathText(\"R\"), obteve {:?}",
                m.body
            );
        }
        other => panic!("esperado Content::MathStyled, obteve {other:?}"),
    }
}

#[test]
fn p311b_italic_is_orthogonal_flag() {
    let v = call_math_style(
        native_math_italic,
        vec![Value::Content(Content::MathIdent("x".into()))],
    );
    assert_styled(v, None, None, Some(true), None);
}

#[test]
fn p311b_mono_wraps_monospace() {
    let v = call_math_style(
        native_mono,
        vec![Value::Content(Content::MathIdent("x".into()))],
    );
    assert_styled(v, Some(MathStyleKind::Monospace), None, None, None);
}

#[test]
fn p311b_sans_wraps_sans_serif() {
    let v = call_math_style(
        native_sans,
        vec![Value::Content(Content::MathIdent("x".into()))],
    );
    assert_styled(v, Some(MathStyleKind::SansSerif), None, None, None);
}

#[test]
fn p311b_scr_wraps_roundhand() {
    let v =
        call_math_style(native_scr, vec![Value::Content(Content::MathIdent("L".into()))]);
    assert_styled(v, Some(MathStyleKind::Roundhand), None, None, None);
}

#[test]
fn p311b_script_with_cramped() {
    let v = call_math_style(
        native_script,
        vec![Value::Content(Content::MathIdent("x".into()))],
    );
    assert_styled(v, Some(MathStyleKind::Script), None, None, Some(true));
}

#[test]
fn p311b_serif_forces_plain() {
    let v = call_math_style(
        native_serif,
        vec![Value::Content(Content::MathIdent("x".into()))],
    );
    assert_styled(v, Some(MathStyleKind::Plain), None, None, None);
}

#[test]
fn p311b_sscript_with_cramped() {
    let v = call_math_style(
        native_sscript,
        vec![Value::Content(Content::MathIdent("x".into()))],
    );
    assert_styled(v, Some(MathStyleKind::SScript), None, None, Some(true));
}

#[test]
fn p311b_upright_suppresses_italic() {
    let v = call_math_style(
        native_upright,
        vec![Value::Content(Content::MathIdent("x".into()))],
    );
    assert_styled(v, None, None, Some(false), None);
}

#[test]
fn p311b_two_args_errors() {
    let r = call_math_style(
        native_bb,
        vec![
            Value::Content(Content::MathIdent("x".into())),
            Value::Content(Content::MathIdent("y".into())),
        ],
    );
    assert!(r.is_err(), "esperava erro por arity > 1");
}

#[test]
fn p311b_int_arg_errors() {
    let r = call_math_style(native_bb, vec![Value::Int(1)]);
    assert!(r.is_err(), "esperava erro por tipo incoercível");
}

#[test]
fn p765b_display_wraps_display_size() {
    let v = call_math_style(
        native_display,
        vec![Value::Content(Content::MathIdent("x".into()))],
    );
    assert_styled(v, Some(MathStyleKind::Display), None, None, Some(false));
}

#[test]
fn p765b_inline_wraps_inline_size() {
    let v = call_math_style(
        native_inline,
        vec![Value::Content(Content::MathIdent("x".into()))],
    );
    assert_styled(v, Some(MathStyleKind::Inline), None, None, Some(false));
}

#[test]
fn p765b_script_named_cramped_false() {
    null_ctx!(ctx);
    let args = pn(
        vec![Value::Content(Content::MathIdent("x".into()))],
        "cramped",
        Value::Bool(false),
    );
    let v = native_script(&mut ctx, &args, &null_world(), test_file_id());
    assert_styled(v, Some(MathStyleKind::Script), None, None, Some(false));
}

#[test]
fn p765b_sscript_named_cramped_false() {
    null_ctx!(ctx);
    let args = pn(
        vec![Value::Content(Content::MathIdent("x".into()))],
        "cramped",
        Value::Bool(false),
    );
    let v = native_sscript(&mut ctx, &args, &null_world(), test_file_id());
    assert_styled(v, Some(MathStyleKind::SScript), None, None, Some(false));
}

#[test]
fn p765b_display_named_cramped_true() {
    null_ctx!(ctx);
    let args = pn(
        vec![Value::Content(Content::MathIdent("x".into()))],
        "cramped",
        Value::Bool(true),
    );
    let v = native_display(&mut ctx, &args, &null_world(), test_file_id());
    assert_styled(v, Some(MathStyleKind::Display), None, None, Some(true));
}

#[test]
fn p765b_inline_named_cramped_true() {
    null_ctx!(ctx);
    let args = pn(
        vec![Value::Content(Content::MathIdent("x".into()))],
        "cramped",
        Value::Bool(true),
    );
    let v = native_inline(&mut ctx, &args, &null_world(), test_file_id());
    assert_styled(v, Some(MathStyleKind::Inline), None, None, Some(true));
}

#[test]
fn p765b_bb_rejects_named_arg() {
    null_ctx!(ctx);
    let args = pn(
        vec![Value::Content(Content::MathIdent("x".into()))],
        "cramped",
        Value::Bool(true),
    );
    let r = native_bb(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "esperava erro por named arg inesperado");
}
