//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/tests/layout_context.md
//! @prompt-hash ebff4ec8
//! @layer L1
//! @updated 2026-09-26

use super::*;

#[test]
fn p218_native_columns_count_valido_sem_gutter() {
    null_ctx!(ctx);
    let r = native_columns(
        &mut ctx,
        &p(vec![Value::Int(2), Value::Content(Content::text("hello"))]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    if let Value::Content(Content::Columns(e)) = r {
        assert_eq!(e.count, 2);
        assert_eq!(e.gutter, None);
        assert_eq!(e.body.plain_text(), "hello");
    } else {
        panic!("esperado Content::Columns");
    }
}

#[test]
fn p218_native_columns_count_zero_rejeita() {
    null_ctx!(ctx);
    let r = native_columns(
        &mut ctx,
        &p(vec![Value::Int(0), Value::Content(Content::text("."))]),
        &null_world(),
        test_file_id(),
    );
    assert!(r.is_err(), "count = 0 deve falhar");
}

#[test]
fn p218_native_columns_count_negativo_rejeita() {
    null_ctx!(ctx);
    let r = native_columns(
        &mut ctx,
        &p(vec![Value::Int(-1), Value::Content(Content::text("."))]),
        &null_world(),
        test_file_id(),
    );
    assert!(r.is_err(), "count = -1 deve falhar");
}

#[test]
fn p218_native_columns_count_nao_int_rejeita() {
    null_ctx!(ctx);
    let r = native_columns(
        &mut ctx,
        &p(vec![Value::Str("foo".into()), Value::Content(Content::text("."))]),
        &null_world(),
        test_file_id(),
    );
    assert!(r.is_err(), "count Str deve falhar");
}

#[test]
fn p218_native_columns_count_ausente_rejeita() {
    null_ctx!(ctx);
    let r = native_columns(&mut ctx, &p(vec![]), &null_world(), test_file_id());
    assert!(r.is_err(), "sem args deve falhar");
}

#[test]
fn p218_native_columns_body_ausente_rejeita() {
    null_ctx!(ctx);
    let r =
        native_columns(&mut ctx, &p(vec![Value::Int(2)]), &null_world(), test_file_id());
    assert!(r.is_err(), "só count sem body deve falhar");
}

#[test]
fn p218_native_columns_body_str_aceita() {
    // Body Value::Str → convertido para Content::text.
    null_ctx!(ctx);
    let r = native_columns(
        &mut ctx,
        &p(vec![Value::Int(3), Value::Str("texto".into())]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    if let Value::Content(Content::Columns(e)) = r {
        assert_eq!(e.body.plain_text(), "texto");
    } else {
        panic!("esperado Content::Columns com body de Str");
    }
}

#[test]
fn p218_native_columns_gutter_length_aceita() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Int(2), Value::Content(Content::text("."))]);
    args.named.insert("gutter".into(), Value::Length(Length::pt(10.0)));
    let r = native_columns(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Columns(e)) = r {
        assert_eq!(e.gutter, Some(Length::pt(10.0)));
    } else {
        panic!("esperado Content::Columns com gutter Some");
    }
}

#[test]
fn p218_native_columns_gutter_negativo_rejeita() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Int(2), Value::Content(Content::text("."))]);
    args.named.insert("gutter".into(), Value::Length(Length::pt(-1.0)));
    let r = native_columns(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "gutter negativo deve falhar");
}

#[test]
fn p218_native_columns_named_arg_desconhecido_rejeita() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Int(2), Value::Content(Content::text("."))]);
    args.named.insert("foo".into(), Value::Int(42));
    let r = native_columns(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "named arg desconhecido deve falhar");
}

#[test]
fn p218_native_columns_extra_positional_rejeita() {
    null_ctx!(ctx);
    let r = native_columns(
        &mut ctx,
        &p(vec![
            Value::Int(2),
            Value::Content(Content::text("a")),
            Value::Content(Content::text("b")), // extra
        ]),
        &null_world(),
        test_file_id(),
    );
    assert!(r.is_err(), ">2 posicionais deve falhar");
}

#[test]
fn p220_native_colbreak_sem_args_aceita() {
    null_ctx!(ctx);
    let r = native_colbreak(&mut ctx, &p(vec![]), &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Colbreak(e)) = r {
        assert!(!e.weak, "default weak == false");
    } else {
        panic!("esperado Value::Content(Content::Colbreak)");
    }
}

#[test]
fn p220_native_colbreak_weak_true_aceita() {
    null_ctx!(ctx);
    let mut args = p(vec![]);
    args.named.insert("weak".into(), Value::Bool(true));
    let r = native_colbreak(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Colbreak(e)) = r {
        assert!(e.weak);
    } else {
        panic!("esperado Value::Content(Content::Colbreak {{ weak: true }})");
    }
}

#[test]
fn p220_native_colbreak_posicional_rejeita() {
    null_ctx!(ctx);
    let r = native_colbreak(
        &mut ctx,
        &p(vec![Value::Str("oops".into())]),
        &null_world(),
        test_file_id(),
    );
    assert!(r.is_err(), "posicional deve falhar");
}

#[test]
fn p220_native_colbreak_weak_nao_bool_rejeita() {
    null_ctx!(ctx);
    let mut args = p(vec![]);
    args.named.insert("weak".into(), Value::Str("true".into()));
    let r = native_colbreak(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "weak não-Bool deve falhar");
}

#[test]
fn p220_native_colbreak_named_desconhecido_rejeita() {
    null_ctx!(ctx);
    let mut args = p(vec![]);
    args.named.insert("foo".into(), Value::Bool(true));
    let r = native_colbreak(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "named desconhecido deve falhar");
}

#[test]
fn p220_native_colbreak_to_rejeita() {
    // Paridade Pagebreak mas SEM `to` em colbreak (vanilla
    // ColbreakElem não tem; paridade só faz sentido em páginas).
    null_ctx!(ctx);
    let mut args = p(vec![]);
    args.named.insert("to".into(), Value::Str("even".into()));
    let r = native_colbreak(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "to: deve falhar (named desconhecido)");
}

#[test]
fn p222_extract_measure_body_content_aceita() {
    let body = extract_measure_body(&p(vec![Value::Content(Content::text("texto"))]));
    assert!(body.is_ok());
}

#[test]
fn p222_extract_measure_body_str_aceita() {
    // Str shortcut → Content::text wrapping.
    let body = extract_measure_body(&p(vec![Value::Str("texto".into())]));
    assert!(body.is_ok());
}

#[test]
fn p222_extract_measure_body_ausente_rejeita() {
    let body = extract_measure_body(&p(vec![]));
    assert!(body.is_err(), "body ausente deve falhar");
}

#[test]
fn p222_extract_measure_body_tipo_errado_rejeita() {
    let body = extract_measure_body(&p(vec![Value::Int(42)]));
    assert!(body.is_err(), "body Int deve falhar");
}

#[test]
fn p222_extract_measure_body_extra_positional_rejeita() {
    let body = extract_measure_body(&p(vec![
        Value::Content(Content::text("a")),
        Value::Content(Content::text("b")),
    ]));
    assert!(body.is_err(), ">1 posicional deve falhar");
}

#[test]
fn p222_extract_measure_body_named_arg_rejeita() {
    // Opção β graded: width override scope-out per ADR-0054.
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("a"))]);
    args.named.insert("width".into(), Value::Length(Length::pt(50.0)));
    let body = extract_measure_body(&args);
    assert!(body.is_err(), "named arg width deve falhar (scope-out graded)");
}

#[test]
fn p712_native_measure_fallback_indirecto_falha_sempre() {
    // O NativeFn deixou de fazer a medição (não tem acesso a
    // engine.styles) — é só o fallback para invocação indirecta, e
    // falha sempre (ADR-0108: falha alto em vez de devolver (0,0)).
    null_ctx!(ctx);
    let r = native_measure(
        &mut ctx,
        &p(vec![Value::Content(Content::text("x"))]),
        &null_world(),
        test_file_id(),
    );
    assert!(r.is_err(), "native_measure directo deve falhar sempre (P712)");
}

#[test]
fn p712_measure_texto_simples_nao_e_mais_zero() {
    // A regressão que P712 fecha: measure_content (o helper antigo) não
    // trata Content::Text — dava sempre (0, 0). measure_content_real usa
    // o motor de layout real (mesmo mecanismo de layout_sub_frame já
    // usado por Content::Place/Transform/grid) e devolve dimensões reais.
    let (w, h) = measure_default(&Content::text("Texto de teste"));
    assert!(w > 0.0, "largura de texto não deve ser zero, recebeu {w}");
    assert!(h > 0.0, "altura de texto não deve ser zero, recebeu {h}");
}

#[test]
fn p712_measure_texto_mais_longo_e_mais_largo() {
    // Proporcionalidade real (FixedMetrics é monoespaçado — 0.6×size por
    // codepoint — mas a proporcionalidade com o comprimento do texto é
    // real, não aproximada por tipo de Content).
    let (w_curto, _) = measure_default(&Content::text("Oi"));
    let (w_longo, _) = measure_default(&Content::text(
        "Um texto bastante mais comprido do que o anterior",
    ));
    assert!(
        w_longo > w_curto,
        "texto mais longo deve medir mais largo: {w_longo} vs {w_curto}"
    );
}

#[test]
fn p712_measure_tamanho_maior_produz_texto_mais_largo() {
    // Paridade com o vanilla: measure() depende do #set text(size:)
    // activo (P711 já corrigiu o mesmo princípio para `context`).
    use crate::compiler::layout::measure_content_real;
    use crate::entities::style::{Style, Styles};
    use crate::entities::style_chain::StyleChain;

    let content = Content::text("mesmo texto");
    let chain_pequena =
        StyleChain::default_chain().push_styles(&Styles::from_iter([Style::Size(
            crate::entities::layout_types::Length::pt(10.0),
        )]));
    let chain_grande =
        StyleChain::default_chain().push_styles(&Styles::from_iter([Style::Size(
            crate::entities::layout_types::Length::pt(40.0),
        )]));

    let (w_pequena, _) = measure_content_real(&content, &chain_pequena, &FixedMetrics);
    let (w_grande, _) = measure_content_real(&content, &chain_grande, &FixedMetrics);
    assert!(
        w_grande > w_pequena,
        "tamanho maior deve medir mais largo: {w_grande} vs {w_pequena}"
    );
}

#[test]
fn p712_measure_dimensoes_para_shape_rect() {
    use crate::entities::geometry::ShapeKind;
    use crate::entities::layout_types::Length;
    let rect = Content::shape(
        ShapeKind::Rect,
        Some(Box::new(Value::Length(Length::pt(40.0)))),
        Some(Box::new(Value::Length(Length::pt(20.0)))),
        None,
        None,
    );
    let (w, h) = measure_default(&rect);
    assert!(w > 0.0, "rect width > 0 esperado, recebeu {w}");
    assert!(h > 0.0, "rect height > 0 esperado, recebeu {h}");
}

#[test]
fn p712_measure_dimensoes_zero_para_empty() {
    let (w, h) = measure_default(&Content::Empty);
    assert_eq!(w, 0.0);
    assert_eq!(h, 0.0);
}

#[test]
fn p223_native_place_float_aceita() {
    // place(top, float: true, body) → float armazenado.
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("a"))]);
    args.named.insert("float".into(), Value::Bool(true));
    let r = native_place(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Place(e)) = r {
        assert!(e.float);
    } else {
        panic!("esperado Content::Place");
    }
}

#[test]
fn p223_native_place_float_default_false() {
    // place(body) sem float → float == false.
    null_ctx!(ctx);
    let r = native_place(
        &mut ctx,
        &p(vec![Value::Content(Content::text("a"))]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    if let Value::Content(Content::Place(e)) = r {
        assert!(!e.float, "default float == false");
    } else {
        panic!("esperado Content::Place");
    }
}

#[test]
fn p223_native_place_float_nao_bool_rejeita() {
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("a"))]);
    args.named.insert("float".into(), Value::Str("yes".into()));
    let r = native_place(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "float não-Bool deve falhar");
}

#[test]
fn p223_native_place_clearance_length_aceita() {
    // place(top, float: true, clearance: 5pt, body) → Some(5pt).
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("a"))]);
    args.named.insert("float".into(), Value::Bool(true));
    args.named.insert("clearance".into(), Value::Length(Length::pt(5.0)));
    let r = native_place(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Place(e)) = r {
        assert_eq!(e.clearance, Some(Length::pt(5.0)));
    } else {
        panic!("esperado Content::Place");
    }
}

#[test]
fn p223_native_place_clearance_negativo_rejeita() {
    null_ctx!(ctx);
    use crate::entities::layout_types::Length;
    let mut args = p(vec![Value::Content(Content::text("a"))]);
    args.named.insert("clearance".into(), Value::Length(Length::pt(-5.0)));
    let r = native_place(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_err(), "clearance negativo deve falhar");
}

#[test]
fn p223_native_place_parent_sem_float_rejeita() {
    // DEBT-37 §"Divergência" restaurada (Decisão 3 Opção α).
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("a"))]);
    args.named.insert("scope".into(), Value::Str("parent".into()));
    let r = native_place(&mut ctx, &args, &null_world(), test_file_id());
    assert!(
        r.is_err(),
        "scope 'parent' sem float deve falhar (paridade vanilla; DEBT-37 fechada)"
    );
}

#[test]
fn p223_native_place_parent_com_float_aceita() {
    // place(scope: "parent", float: true, body) → OK.
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("a"))]);
    args.named.insert("scope".into(), Value::Str("parent".into()));
    args.named.insert("float".into(), Value::Bool(true));
    let r = native_place(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    use crate::entities::layout_types::PlaceScope;
    if let Value::Content(Content::Place(e)) = r {
        assert!(matches!(e.scope, PlaceScope::Parent));
        assert!(e.float);
    } else {
        panic!("esperado Content::Place");
    }
}

#[test]
fn p223_native_place_column_sem_restricao() {
    // scope: "column" OK independente de float (default scope).
    null_ctx!(ctx);
    let mut args = p(vec![Value::Content(Content::text("a"))]);
    args.named.insert("scope".into(), Value::Str("column".into()));
    // sem float
    let r = native_place(&mut ctx, &args, &null_world(), test_file_id());
    assert!(r.is_ok(), "scope 'column' sem float OK (não tem restrição vanilla)");
}

#[test]
fn p1250a_page_paper_posicional_exige_nome_e_body_separado() {
    assert!(super::layout::is_positional_paper("a4", 2));
    assert!(!super::layout::is_positional_paper("a4", 1));
    assert!(!super::layout::is_positional_paper("not-paper", 2));
}

#[test]
fn p1250a_page_binding_aceita_so_alinhamento_horizontal_puro() {
    use crate::entities::layout_types::{Align2D, HAlign, VAlign};
    let left = Align2D { h: Some(HAlign::Left), v: None };
    let right = Align2D { h: Some(HAlign::Right), v: None };
    let vertical = Align2D { h: Some(HAlign::Left), v: Some(VAlign::Top) };
    assert!(super::layout::is_horizontal_page_binding(left, HAlign::Left));
    assert!(super::layout::is_horizontal_page_binding(right, HAlign::Right));
    assert!(!super::layout::is_horizontal_page_binding(vertical, HAlign::Left));
    assert!(!super::layout::is_horizontal_page_binding(left, HAlign::Right));
}

#[test]
fn ts1_text_positional_size_fill_body_permutations() {
    use crate::entities::layout_types::{Color, Length};
    use crate::entities::style_chain::StyleChain;
    let values = [
        Value::Length(Length::em(2.0)),
        Value::Color(Color::rgb(255, 0, 0)),
        Value::Content(Content::text("A")),
    ];
    for order in [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]] {
        null_ctx!(ctx);
        let result = native_text(
            &mut ctx,
            &p(order.into_iter().map(|i| values[i].clone()).collect()),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        let Value::Content(Content::Styled(styled)) = result else {
            panic!("text must retain styles")
        };
        assert_eq!(styled.body.plain_text(), "A");
        let chain = StyleChain::default_chain().push_styles(&styled.styles);
        assert_eq!(chain.size(), 22.0);
        assert_eq!(chain.fill(), Some(Color::rgb(255, 0, 0)));
    }
}

#[test]
fn ts1_text_none_is_empty_and_body_is_required() {
    null_ctx!(ctx);
    let Value::Content(c) =
        native_text(&mut ctx, &p(vec![Value::None]), &null_world(), test_file_id())
            .unwrap()
    else {
        panic!("content")
    };
    assert!(c.plain_text().is_empty());
    let errors =
        native_text(&mut ctx, &p(vec![]), &null_world(), test_file_id()).unwrap_err();
    assert_eq!(errors[0].message.as_str(), "missing argument: body");
}

#[test]
fn ts1_named_size_does_not_consume_positional_length() {
    use crate::entities::layout_types::Length;
    null_ctx!(ctx);
    let mut args =
        p(vec![Value::Length(Length::pt(18.0)), Value::Content(Content::text("A"))]);
    args.named.insert("size".into(), Value::Length(Length::pt(20.0)));
    let errors = native_text(&mut ctx, &args, &null_world(), test_file_id()).unwrap_err();
    assert_eq!(errors[0].message.as_str(), "expected content, found length");
}
