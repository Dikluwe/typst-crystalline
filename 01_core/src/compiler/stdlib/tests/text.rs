//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/tests/text.md
//! @prompt-hash 571f6ecb
//! @layer L1
//! @updated 2026-09-28

use super::*;

#[test]
fn p284_native_underline_envolve_body_sem_named() {
    null_ctx!(ctx);
    let args = p(vec![Value::Content(Content::text("hello"))]);
    let r = native_underline(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Underline(e)) = r {
        assert_eq!(e.body.plain_text(), "hello");
        assert!(
            e.stroke.is_none() && e.offset.is_none() && e.extent.is_none(),
            "sem named: cosméticos preservados em None"
        );
    } else {
        panic!("esperado Value::Content(Content::Underline {{ .. }})");
    }
}

#[test]
fn p284_native_strike_e_overline_idem_underline() {
    null_ctx!(ctx);
    let args = p(vec![Value::Content(Content::text("x"))]);
    let s = native_strike(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    let o = native_overline(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    assert!(matches!(s, Value::Content(Content::Strike(_))));
    assert!(matches!(o, Value::Content(Content::Overline(_))));
}

#[test]
fn p284_native_underline_aceita_string_como_body() {
    null_ctx!(ctx);
    let args = p(vec![Value::Str("important".into())]);
    let r = native_underline(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    match r {
        Value::Content(Content::Underline(e)) => {
            assert_eq!(e.body.plain_text(), "important")
        }
        other => panic!("esperado Underline, obtido {other:?}"),
    }
}

#[test]
fn p284_native_underline_named_stroke_offset_extent() {
    use crate::entities::layout_types::{Color, Length};
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("y"))]);
    args.named
        .insert("stroke".into(), Value::Color(Color::rgb(255, 0, 0)));
    args.named.insert("offset".into(), Value::Length(Length::pt(2.5)));
    args.named.insert("extent".into(), Value::Length(Length::pt(-1.0)));
    let r = native_underline(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Underline(e)) = r {
        assert_eq!(e.stroke, Some(Color::rgb(255, 0, 0)));
        assert_eq!(e.offset, Some(Length::pt(2.5)));
        assert_eq!(e.extent, Some(Length::pt(-1.0)));
    } else {
        panic!("esperado Underline com cosméticos");
    }
}

#[test]
fn p284_native_decoration_sem_body_retorna_err() {
    null_ctx!(ctx);
    let args = p(vec![]);
    assert!(native_underline(&mut ctx, &args, &null_world(), test_file_id()).is_err());
    assert!(native_strike(&mut ctx, &args, &null_world(), test_file_id()).is_err());
    assert!(native_overline(&mut ctx, &args, &null_world(), test_file_id()).is_err());
}

#[test]
fn p284_native_decoration_scope_out_evade_e_background_erro_explicito() {
    // Mensagem de erro deve referenciar a decisão graded (não erro
    // genérico "argumento inesperado"). Cobre diagnóstico §A.1.
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("evade".into(), Value::Bool(true));
    let err =
        native_underline(&mut ctx, &args, &null_world(), test_file_id()).unwrap_err();
    let msg = format!("{:?}", err);
    assert!(
        msg.contains("evade") && msg.contains("ADR-0054"),
        "mensagem deve referir 'evade' e ADR-0054 graded: {msg}"
    );
    let mut args2 = p(vec![Value::Content(Content::text("x"))]);
    args2.named.insert("background".into(), Value::Bool(true));
    let err2 =
        native_overline(&mut ctx, &args2, &null_world(), test_file_id()).unwrap_err();
    assert!(format!("{:?}", err2).contains("background"));
}

#[test]
fn p287_native_smartquote_sem_args_emite_variant_double_true() {
    use super::native_smartquote;
    null_ctx!(ctx);
    let r =
        native_smartquote(&mut ctx, &p(vec![]), &null_world(), test_file_id()).unwrap();
    assert_eq!(
        r,
        Value::Content(Content::smartquote(true)),
        "smartquote() sem args → SmartQuote {{ double: true }} (vanilla default)"
    );
}

#[test]
fn p287_native_smartquote_double_false_emite_variant_single() {
    use super::native_smartquote;
    null_ctx!(ctx);
    let mut args = p(vec![]);
    args.named.insert("double".into(), Value::Bool(false));
    let r = native_smartquote(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    assert_eq!(r, Value::Content(Content::smartquote(false)));
}

#[test]
fn p287_native_smartquote_enabled_false_emite_text_ascii_directo() {
    // Paridade vanilla `set smartquote(enabled: false)` — ASCII literal,
    // não passa pelo variant SmartQuote (consumer Layouter ignora).
    use super::native_smartquote;
    null_ctx!(ctx);
    // enabled=false + double=true → Text("\"").
    let mut args = p(vec![]);
    args.named.insert("enabled".into(), Value::Bool(false));
    let r = native_smartquote(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    assert_eq!(r, Value::Content(Content::text("\"")));
    // enabled=false + double=false → Text("'").
    let mut args2 = p(vec![]);
    args2.named.insert("enabled".into(), Value::Bool(false));
    args2.named.insert("double".into(), Value::Bool(false));
    let r2 = native_smartquote(&mut ctx, &args2, &null_world(), test_file_id()).unwrap();
    assert_eq!(r2, Value::Content(Content::text("'")));
}

#[test]
fn p287_native_smartquote_alternative_preserva_override_no_carrier() {
    use super::native_smartquote;
    null_ctx!(ctx);
    let mut args = p(vec![]);
    args.named.insert("alternative".into(), Value::Bool(true));
    let value =
        native_smartquote(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    match value {
        Value::Content(Content::SmartQuote(e)) => {
            assert!(e.double);
            assert_eq!(e.alternative, Some(true));
            assert!(e.quotes.is_none());
        }
        other => panic!("esperado SmartQuote carrier, obtido {other:?}"),
    }

    let mut invalid = p(vec![]);
    invalid.named.insert("alternative".into(), Value::Int(1));
    let err =
        native_smartquote(&mut ctx, &invalid, &null_world(), test_file_id()).unwrap_err();
    assert!(format!("{err:?}").contains("bool"));
}

#[test]
fn p287_native_smartquote_quotes_custom_preserva_par_no_carrier() {
    use super::native_smartquote;
    null_ctx!(ctx);
    let mut args = p(vec![]);
    args.named.insert("quotes".into(), Value::Str("()".into()));
    let value =
        native_smartquote(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    match value {
        Value::Content(Content::SmartQuote(e)) => match e.quotes.as_ref() {
            Some(crate::entities::elements::smartquote::SmartQuoteQuotes::Custom(
                overrides,
            )) => {
                assert!(overrides.single.is_none());
                let pair = overrides.double.as_ref().expect("override double");
                assert_eq!(pair.open.as_str(), "(");
                assert_eq!(pair.close.as_str(), ")");
            }
            other => panic!("esperado quotes custom, obtido {other:?}"),
        },
        other => panic!("esperado SmartQuote carrier, obtido {other:?}"),
    }

    let mut invalid = p(vec![]);
    invalid.named.insert("quotes".into(), Value::Str("x".into()));
    let err =
        native_smartquote(&mut ctx, &invalid, &null_world(), test_file_id()).unwrap_err();
    assert!(format!("{err:?}").contains("expected 2 characters"));
}

#[test]
fn p287_native_smartquote_arg_posicional_retorna_err() {
    use super::native_smartquote;
    null_ctx!(ctx);
    let r = native_smartquote(
        &mut ctx,
        &p(vec![Value::Bool(true)]),
        &null_world(),
        test_file_id(),
    );
    assert!(r.is_err(), "args posicionais rejeitados (vanilla usa só named)");
}

#[test]
fn p408_native_smallcaps_envolve_content() {
    use super::native_smallcaps;
    null_ctx!(ctx);
    let args = p(vec![Value::Content(Content::text("Hello"))]);
    let r = native_smallcaps(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    match r {
        Value::Content(Content::SmallCaps { body }) => {
            assert_eq!(body.plain_text(), "Hello");
        }
        other => panic!("esperado Content::SmallCaps, obtido {other:?}"),
    }
}

#[test]
fn p408_native_smallcaps_aceita_string() {
    use super::native_smallcaps;
    null_ctx!(ctx);
    let args = p(vec![Value::Str("World".into())]);
    let r = native_smallcaps(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    match r {
        Value::Content(Content::SmallCaps { body }) => {
            assert_eq!(body.plain_text(), "World");
        }
        other => panic!("esperado Content::SmallCaps, obtido {other:?}"),
    }
}

#[test]
fn p408_native_smallcaps_sem_body_retorna_err() {
    use super::native_smallcaps;
    null_ctx!(ctx);
    let args = p(vec![]);
    assert!(native_smallcaps(&mut ctx, &args, &null_world(), test_file_id()).is_err());
}

#[test]
fn p408_native_smallcaps_rejeita_named_arg() {
    use super::native_smallcaps;
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("stroke".into(), Value::Int(1));
    assert!(native_smallcaps(&mut ctx, &args, &null_world(), test_file_id()).is_err());
}

#[test]
fn p408_native_smallcaps_rejeita_tipo_errado() {
    use super::native_smallcaps;
    null_ctx!(ctx);
    let args = p(vec![Value::Int(42)]);
    assert!(native_smallcaps(&mut ctx, &args, &null_world(), test_file_id()).is_err());
}

#[test]
fn p408_native_smallcaps_body_acessivel_via_get_field() {
    use super::native_smallcaps;
    null_ctx!(ctx);
    let args = p(vec![Value::Content(Content::text("abc"))]);
    let r = native_smallcaps(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(c) = r {
        assert_eq!(
            c.get_field("body"),
            Some(Value::Content(Content::text("abc"))),
            "it.body deve devolver o body original"
        );
    } else {
        panic!("esperado Value::Content");
    }
}

#[test]
fn lorem_0_retorna_string_vazia() {
    null_ctx!(ctx);
    let r =
        native_lorem(&mut ctx, &p(vec![Value::Int(0)]), &null_world(), test_file_id())
            .unwrap();
    assert_eq!(r, Value::Str("".into()));
}

#[test]
fn lorem_p805_byte_parity_vanilla() {
    // P805 — byte-parity com o vanilla 0.15.0 (strings extraídas do PDF
    // vanilla por pdftotext na sonda de P805, normalizadas). Antes:
    // vocabulário cíclico próprio sem pontuação (scope-out de Passo 391,
    // revogado por este passo).
    let casos: [(i64, &str); 3] = [
        (1, "Lorem."),
        (10, "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do."),
        (
            30,
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do \
             eiusmod tempor incididunt ut labore et dolore magnam aliquam \
             quaerat voluptatem. Ut enim aeque doleamus animo, cum corpore \
             dolemus, fieri.",
        ),
    ];
    for (n, esperado) in casos {
        // `esperado` tem continuações de linha do Rust — normalizar.
        let esperado_norm: String =
            esperado.split_whitespace().collect::<Vec<_>>().join(" ");
        null_ctx!(ctx);
        let r = native_lorem(
            &mut ctx,
            &p(vec![Value::Int(n)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        let Value::Str(s) = r else { panic!("esperava Value::Str") };
        let obtido_norm: String = s.split_whitespace().collect::<Vec<_>>().join(" ");
        assert_eq!(obtido_norm, esperado_norm, "lorem({n}) byte-parity");
    }
}

#[test]
fn lorem_1_retorna_uma_palavra_sem_espaco() {
    null_ctx!(ctx);
    let r =
        native_lorem(&mut ctx, &p(vec![Value::Int(1)]), &null_world(), test_file_id())
            .unwrap();
    if let Value::Str(s) = r {
        assert_eq!(s.split_whitespace().count(), 1);
        assert!(!s.ends_with(' '));
    } else {
        panic!("esperava Value::Str");
    }
}

#[test]
fn lorem_5_retorna_5_palavras() {
    null_ctx!(ctx);
    let r =
        native_lorem(&mut ctx, &p(vec![Value::Int(5)]), &null_world(), test_file_id())
            .unwrap();
    if let Value::Str(s) = r {
        assert_eq!(s.split_whitespace().count(), 5);
    } else {
        panic!("esperava Value::Str");
    }
}

#[test]
fn lorem_100_retorna_100_palavras() {
    null_ctx!(ctx);
    let r =
        native_lorem(&mut ctx, &p(vec![Value::Int(100)]), &null_world(), test_file_id())
            .unwrap();
    if let Value::Str(s) = r {
        assert_eq!(s.split_whitespace().count(), 100);
    } else {
        panic!("esperava Value::Str");
    }
}

#[test]
fn lorem_negativo_retorna_erro() {
    null_ctx!(ctx);
    let r =
        native_lorem(&mut ctx, &p(vec![Value::Int(-1)]), &null_world(), test_file_id());
    assert!(r.is_err(), "lorem(-1) deve retornar erro");
}

#[test]
fn lorem_rejeita_named_arg() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Int(5)]);
    args.named.insert("foo".into(), Value::Int(1));
    assert!(native_lorem(&mut ctx, &args, &null_world(), test_file_id()).is_err());
}

#[test]
fn lorem_rejeita_tipo_errado() {
    null_ctx!(ctx);
    let r = native_lorem(
        &mut ctx,
        &p(vec![Value::Str("x".into())]),
        &null_world(),
        test_file_id(),
    );
    assert!(r.is_err(), "lorem(string) deve retornar erro");
}

#[test]
fn p471_highlight_sem_args_ok() {
    null_ctx!(ctx);
    let r = text::native_highlight(
        &mut ctx,
        &p(vec![Value::Content(Content::text("x"))]),
        &null_world(),
        test_file_id(),
    );
    assert!(r.is_ok());
    assert!(matches!(r.unwrap(), Value::Content(_)));
}

#[test]
fn p471_highlight_radius_produz_campo() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("radius".into(), Value::Length(Length::pt(3.0)));
    let r =
        text::native_highlight(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Styled(styled)) = r {
        let styles = &styled.styles;
        assert_eq!(styles.delta().highlight_radius, Some(Length::pt(3.0)));
    } else {
        panic!("esperado Content::Styled");
    }
}

#[test]
fn p471_highlight_extent_produz_campo() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("extent".into(), Value::Length(Length::pt(2.0)));
    let r =
        text::native_highlight(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Styled(styled)) = r {
        let styles = &styled.styles;
        assert_eq!(styles.delta().highlight_extent, Some(Length::pt(2.0)));
    } else {
        panic!("esperado Content::Styled");
    }
}

#[test]
fn p471_highlight_named_desconhecido_erro() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("color".into(), Value::Length(Length::pt(1.0)));
    let r = text::native_highlight(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err());
}

#[test]
fn p471_sub_sem_size_sem_campo() {
    null_ctx!(ctx);
    let r = text::native_subscript(
        &mut ctx,
        &p(vec![Value::Content(Content::text("x"))]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    if let Value::Content(Content::Styled(styled)) = r {
        let styles = &styled.styles;
        assert_eq!(styles.delta().subscript_size, None);
    } else {
        panic!("esperado Content::Styled");
    }
}

#[test]
fn p471_sub_com_size_produz_campo() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("size".into(), Value::Length(Length::pt(8.0)));
    let r =
        text::native_subscript(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Styled(styled)) = r {
        let styles = &styled.styles;
        assert_eq!(styles.delta().subscript_size, Some(Length::pt(8.0)));
    } else {
        panic!("esperado Content::Styled");
    }
}

#[test]
fn p471_super_com_size_produz_campo() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("size".into(), Value::Length(Length::pt(9.0)));
    let r =
        text::native_superscript(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Styled(styled)) = r {
        let styles = &styled.styles;
        assert_eq!(styles.delta().superscript_size, Some(Length::pt(9.0)));
    } else {
        panic!("esperado Content::Styled");
    }
}

#[test]
fn p471_sub_named_desconhecido_erro() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("offset".into(), Value::Length(Length::pt(1.0)));
    let r = text::native_subscript(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err());
}

#[test]
fn break_policy_native_text_validates_and_transports_hyphenate() {
    for value in [Value::Bool(false), Value::Bool(true), Value::Auto] {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("hyphenate".into(), value.clone());
        let result = crate::compiler::stdlib::text::native_text(
            &mut ctx,
            &args,
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        let Value::Content(Content::Styled(s)) = result else {
            panic!("policy not captured")
        };
        assert!(s
            .styles
            .delta()
            .custom
            .iter()
            .any(|(k, v)| k == "text.hyphenate" && v == &value));
        assert_eq!(s.body.plain_text(), "x");
    }
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("hyphenate".into(), Value::Int(1));
    let err = crate::compiler::stdlib::text::native_text(
        &mut ctx,
        &args,
        &null_world(),
        test_file_id(),
    )
    .unwrap_err();
    assert_eq!(err[0].message, "expected boolean or auto, found integer");
}

#[test]
fn hardcode_native_overhang_is_boolean_and_transported() {
    for value in [Value::Bool(false), Value::Bool(true)] {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("overhang".into(), value.clone());
        let result = text::native_text(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
        let Value::Content(Content::Styled(s)) = result else { panic!("missing policy") };
        assert!(s.styles.delta().custom.iter().any(|(k,v)| k == "text.overhang" && v == &value));
        assert_eq!(s.body.plain_text(), "x");
    }
    for (value, kind) in [(Value::Auto,"auto"),(Value::Int(1),"integer"),(Value::Str("false".into()),"string")] {
        null_ctx!(ctx);
        let mut args = p(vec![Value::Content(Content::text("x"))]);
        args.named.insert("overhang".into(), value);
        let err=text::native_text(&mut ctx,&args,&null_world(),test_file_id()).unwrap_err();
        assert_eq!(err[0].message,format!("expected boolean, found {kind}"));
    }
}

#[test]
fn custom_costs_native_text_transports_four_fields_and_rejects_types() {
    use crate::entities::layout_types::Ratio;
    let mut dict=match Value::Dict(Default::default()) {Value::Dict(d)=>d,_=>unreachable!()};
    for key in ["hyphenation","runt","widow","orphan"] {dict.insert(key.into(),Value::Ratio(Ratio(2.0)));}
    null_ctx!(ctx);let mut args=p(vec![Value::Content(Content::text("x"))]);
    args.named.insert("costs".into(),Value::Dict(dict));
    let value=text::native_text(&mut ctx,&args,&null_world(),test_file_id()).unwrap();
    let Value::Content(Content::Styled(s))=value else {panic!("costs discarded")};
    for key in ["hyphenation","runt","widow","orphan"] {
        assert!(s.styles.delta().custom.iter().any(|(k,v)|k.as_str()==format!("text.costs.{key}")&&v==&Value::Ratio(Ratio(2.0))));
    }
    assert_eq!(s.body.plain_text(),"x");
    args.named.insert("costs".into(),Value::Int(1));
    assert_eq!(text::native_text(&mut ctx,&args,&null_world(),test_file_id()).unwrap_err()[0].message,"expected dictionary, found integer");
}
