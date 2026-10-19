//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/tests/shapes.md
//! @prompt-hash 7fe87aa0
//! @layer L1
//! @updated 2026-10-08

use super::*;

#[test]
fn rect_sem_cores_tem_stroke_preta_1pt() {
    // #rect() sem fill nem stroke → stroke preta de 1pt.
    // Confirma que a stdlib é o único local onde este fallback existe.
    null_ctx!(ctx);
    use crate::entities::layout_types::Color;
    use crate::entities::paint::Paint;
    let result =
        native_rect(&mut ctx, &p(vec![]), &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Shape(e)) = result {
        assert!(e.fill.is_none(), "rect sem fill deve ter fill: None");
        let s = e.stroke.clone().expect("rect sem cores deve ter stroke de fallback");
        assert_eq!(
            s.paint,
            Paint::Solid(Color::rgb(0, 0, 0)),
            "stroke de fallback deve ser preta"
        );
        assert_eq!(s.thickness, 1.0, "espessura de fallback deve ser 1pt");
    } else {
        panic!("Esperado Content::Shape");
    }
}

#[test]
fn rect_com_fill_nao_tem_stroke_fallback() {
    null_ctx!(ctx);
    let mut args = Args::positional(vec![]);
    args.named.insert("fill".into(), Value::Color(Color::rgb(255, 0, 0)));
    let result = native_rect(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Shape(e)) = result {
        assert!(e.fill.is_some(), "fill red deve estar presente");
        assert!(
            e.stroke.is_none(),
            "sem stroke explícito e com fill → stroke deve ser None"
        );
    } else {
        panic!("Esperado Content::Shape");
    }
}

#[test]
fn p1221_rect_preserva_stroke_rico_com_fill() {
    use crate::entities::geometry::Stroke;
    use crate::entities::paint::Paint;
    null_ctx!(ctx);
    let expected = Stroke {
        paint: Paint::Solid(Color::rgb(0, 116, 217)),
        thickness: 2.0,
        overhang: true,
        ..Stroke::default()
    };
    let mut args = Args::positional(vec![]);
    args.named
        .insert("fill".into(), Value::Color(Color::rgb(255, 65, 54)));
    args.named.insert("stroke".into(), Value::from(expected.clone()));
    let result = native_rect(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    let Value::Content(Content::Shape(shape)) = result else { panic!("esperado shape") };
    assert_eq!(shape.stroke, Some(expected));
}

#[test]
fn rect_radius_uniforme_produz_rounded_rect() {
    null_ctx!(ctx);
    use crate::entities::geometry::ShapeKind;
    let mut args = Args::positional(vec![]);
    args.named.insert("radius".into(), Value::Length(Length::pt(5.0)));
    let result = native_rect(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Shape(e)) = result {
        assert!(
            matches!(e.kind, ShapeKind::RoundedRect { .. }),
            "radius ≠ 0 deve produzir RoundedRect"
        );
    } else {
        panic!("Esperado Content::Shape");
    }
}

#[test]
fn rect_radius_zero_mantem_rect() {
    null_ctx!(ctx);
    use crate::entities::geometry::ShapeKind;
    let mut args = Args::positional(vec![]);
    args.named.insert("radius".into(), Value::Length(Length::ZERO));
    let result = native_rect(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Shape(e)) = result {
        assert!(matches!(e.kind, ShapeKind::Rect), "radius zero deve manter Rect");
    } else {
        panic!("Esperado Content::Shape");
    }
}

#[test]
fn square_posicional_1cm_produz_rect_lados_iguais() {
    use crate::entities::geometry::ShapeKind;
    null_ctx!(ctx);
    let w = Value::Length(Length::pt(Length::PT_PER_CM));
    let result =
        native_square(&mut ctx, &p(vec![w.clone()]), &null_world(), test_file_id())
            .unwrap();
    if let Value::Content(Content::Shape(e)) = result {
        assert!(matches!(e.kind, ShapeKind::Rect));
        assert_eq!(e.width.as_deref(), Some(&w));
        assert_eq!(e.height.as_deref(), Some(&w));
    } else {
        panic!("Esperado Content::Shape");
    }
}

#[test]
fn square_com_height_diferente_aceita_fallback() {
    use crate::entities::geometry::ShapeKind;
    null_ctx!(ctx);
    let w = Value::Length(Length::pt(Length::PT_PER_CM));
    let h = Value::Length(Length::pt(2.0 * Length::PT_PER_CM));
    let mut args = Args::positional(vec![w.clone()]);
    args.named.insert("height".into(), h.clone());
    let result = native_square(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Shape(e)) = result {
        assert!(matches!(e.kind, ShapeKind::Rect));
        assert_eq!(e.width.as_deref(), Some(&w));
        assert_eq!(e.height.as_deref(), Some(&h));
    } else {
        panic!("Esperado Content::Shape");
    }
}

#[test]
fn square_sem_width_gera_erro() {
    null_ctx!(ctx);
    let result = native_square(&mut ctx, &p(vec![]), &null_world(), test_file_id());
    assert!(result.is_err(), "square() sem width deve retornar Err");
}

#[test]
fn square_rejeita_named_arg_invalido() {
    null_ctx!(ctx);
    let args = pn(vec![Value::Length(Length::pt(10.0))], "foo", Value::Int(1));
    assert!(native_square(&mut ctx, &args, &null_world(), test_file_id()).is_err());
}

#[test]
fn square_sem_cores_tem_stroke_preta_1pt() {
    use crate::entities::geometry::ShapeKind;
    use crate::entities::paint::Paint;
    null_ctx!(ctx);
    let result = native_square(
        &mut ctx,
        &p(vec![Value::Length(Length::pt(10.0))]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    if let Value::Content(Content::Shape(e)) = result {
        assert!(matches!(e.kind, ShapeKind::Rect));
        assert!(e.fill.is_none(), "square sem fill deve ter fill: None");
        let s = e
            .stroke
            .clone()
            .expect("square sem cores deve ter stroke de fallback");
        assert_eq!(
            s.paint,
            Paint::Solid(Color::rgb(0, 0, 0)),
            "stroke de fallback deve ser preta"
        );
        assert_eq!(s.thickness, 1.0, "espessura de fallback deve ser 1pt");
    } else {
        panic!("Esperado Content::Shape");
    }
}

#[test]
fn line_tem_kind_line_e_stroke_preta_por_omissao() {
    use crate::entities::geometry::ShapeKind;
    null_ctx!(ctx);
    let mut args = Args::positional(vec![]);
    args.named.insert("dx".into(), Value::Float(100.0));
    args.named.insert("dy".into(), Value::Float(50.0));
    let result = native_line(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Shape(e)) = result {
        assert!(
            matches!(e.kind, ShapeKind::Line { dx, dy } if dx == 100.0 && dy == 50.0)
        );
        assert!(e.fill.is_none(), "linha não tem fill");
        assert!(e.stroke.is_some(), "linha tem stroke por omissão");
    } else {
        panic!("Esperado Content::Shape");
    }
}

#[test]
fn p1221_line_preserva_stroke_rico() {
    use crate::entities::geometry::Stroke;
    use crate::entities::paint::Paint;
    null_ctx!(ctx);
    let expected = Stroke {
        paint: Paint::Solid(Color::rgb(255, 65, 54)),
        thickness: 2.0,
        overhang: true,
        ..Stroke::default()
    };
    let mut args = Args::positional(vec![]);
    args.named.insert("stroke".into(), Value::from(expected.clone()));
    let result = native_line(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    let Value::Content(Content::Shape(shape)) = result else { panic!("esperado shape") };
    assert_eq!(shape.stroke, Some(expected));
}

#[test]
fn p804_line_length_sozinho() {
    // P804 — `#line(length: 3cm)` era rejeitado ("argumento nomeado
    // inesperado"); vanilla aceita: dx = cos(0)·length, dy = sin(0)·length.
    use crate::entities::layout_types::Length;
    null_ctx!(ctx);
    let mut args = Args::positional(vec![]);
    args.named.insert("length".into(), Value::Length(Length::pt(85.04))); // 3cm
    let result = native_line(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Shape(e)) = result {
        let (start, end) = p804_open_path_points(&e.kind);
        assert_eq!(start, (0.0, 0.0));
        assert!((end.0 - 85.04).abs() < 0.01 && end.1.abs() < 0.01, "end={end:?}");
    } else {
        panic!("Esperado Content::Shape");
    }
}

#[test]
fn p804_line_length_com_angle() {
    // P804 — `#line(length: 4cm, angle: 90deg)` → dx≈0, dy=4cm
    // (paridade vanilla layout_line: delta = (cos·len, sin·len)).
    use crate::entities::layout_types::{Angle, Length};
    null_ctx!(ctx);
    let mut args = Args::positional(vec![]);
    args.named.insert("length".into(), Value::Length(Length::pt(113.39))); // 4cm
    args.named.insert("angle".into(), Value::Angle(Angle::deg(90.0)));
    let result = native_line(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Shape(e)) = result {
        let (start, end) = p804_open_path_points(&e.kind);
        assert_eq!(start, (0.0, 0.0));
        assert!(end.0.abs() < 0.01 && (end.1 - 113.39).abs() < 0.01, "end={end:?}");
    } else {
        panic!("Esperado Content::Shape");
    }
}

#[test]
fn p804_line_length_ignorado_com_end() {
    // P804 — medido no vanilla 0.15.0: `#line(length: 3cm, end: (1cm, 1cm))`
    // compila e `length` é IGNORADO ("only respected if end is none").
    use crate::entities::layout_types::Length;
    null_ctx!(ctx);
    let mut args = Args::positional(vec![]);
    args.named.insert("length".into(), Value::Length(Length::pt(85.04)));
    args.named.insert(
        "end".into(),
        Value::Array(vec![
            Value::Length(Length::pt(Length::PT_PER_CM)),
            Value::Length(Length::pt(Length::PT_PER_CM)),
        ]),
    );
    let result = native_line(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Shape(e)) = result {
        let (start, end) = p804_open_path_points(&e.kind);
        assert_eq!(start, (0.0, 0.0));
        assert!(
            (end.0 - Length::PT_PER_CM).abs() < 0.01
                && (end.1 - Length::PT_PER_CM).abs() < 0.01,
            "end deve prevalecer sobre length: {end:?}"
        );
    } else {
        panic!("Esperado Content::Shape");
    }
}

#[test]
fn p804_line_length_nao_combinavel_com_dx() {
    // P804 — `length`/`angle` não combinam com o legado `dx`/`dy`.
    use crate::entities::layout_types::Length;
    null_ctx!(ctx);
    let mut args = Args::positional(vec![]);
    args.named.insert("length".into(), Value::Length(Length::pt(85.04)));
    args.named.insert("dx".into(), Value::Float(10.0));
    let err = native_line(&mut ctx, &args, &null_world(), test_file_id()).unwrap_err();
    assert!(
        format!("{:?}", err).contains("não pode ser combinado"),
        "erro de combinação inválida, obtido: {:?}",
        err
    );
}

#[test]
fn polygon_sem_pontos_gera_erro() {
    null_ctx!(ctx);
    let args = Args::positional(vec![]);
    let result = native_polygon(&mut ctx, &args, &null_world(), test_file_id());
    assert!(result.is_err(), "polygon() sem pontos deve retornar Err");
    let msg = result.unwrap_err();
    let msg_str = format!("{:?}", msg);
    assert!(
        msg_str.contains("pelo menos um ponto"),
        "Mensagem de erro deve mencionar 'pelo menos um ponto', obteve: {}",
        msg_str
    );
}

#[test]
fn polygon_com_um_ponto_gera_moveto_e_closepath() {
    use crate::entities::geometry::PathItem;
    use crate::entities::layout_types::Length;
    null_ctx!(ctx);
    // P734 — vértices são Length (paridade vanilla); números rejeitados.
    let args = Args::positional(vec![Value::Array(vec![
        Value::Length(Length::pt(10.0)),
        Value::Length(Length::pt(20.0)),
    ])]);
    let result = native_polygon(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Shape(e)) = result {
        let crate::entities::geometry::ShapeKind::Path(items) = &e.kind else {
            panic!("esperado ShapeKind::Path");
        };
        assert_eq!(items.len(), 2, "Um ponto deve gerar MoveTo + ClosePath");
        assert!(matches!(items[0], PathItem::MoveTo(_)), "Primeiro item deve ser MoveTo");
        assert!(
            matches!(items[1], PathItem::ClosePath),
            "Último item deve ser ClosePath"
        );
    } else {
        panic!("Esperado Content::Shape com ShapeKind::Path");
    }
}

#[test]
fn polygon_triangulo_gera_moveto_lineto_lineto_closepath() {
    use crate::entities::geometry::PathItem;
    use crate::entities::layout_types::Length;
    null_ctx!(ctx);
    // P734 — vértices são Length (paridade vanilla); números rejeitados.
    let args = Args::positional(vec![
        Value::Array(vec![
            Value::Length(Length::pt(0.0)),
            Value::Length(Length::pt(0.0)),
        ]),
        Value::Array(vec![
            Value::Length(Length::pt(50.0)),
            Value::Length(Length::pt(0.0)),
        ]),
        Value::Array(vec![
            Value::Length(Length::pt(25.0)),
            Value::Length(Length::pt(50.0)),
        ]),
    ]);
    let result = native_polygon(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Shape(e)) = result {
        let crate::entities::geometry::ShapeKind::Path(items) = &e.kind else {
            panic!("esperado ShapeKind::Path");
        };
        assert_eq!(items.len(), 4); // MoveTo + 2×LineTo + ClosePath
        assert!(matches!(items[0], PathItem::MoveTo(_)));
        assert!(matches!(items[1], PathItem::LineTo(_)));
        assert!(matches!(items[2], PathItem::LineTo(_)));
        assert!(matches!(items[3], PathItem::ClosePath));
    } else {
        panic!("Esperado Content::Shape com ShapeKind::Path");
    }
}

#[test]
fn p732_polygon_aceita_coordenadas_length() {
    // P732 — medido: o vanilla exige relative length nos vértices
    // ("expected relative length, found integer"); o cristalino
    // rejeitava Length ("argumento 0 não é uma coordenada válida").
    use crate::entities::geometry::{PathItem, ShapeKind};
    null_ctx!(ctx);
    let args = p732_polygon_args_length();
    let result = native_polygon(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Shape(e)) = result {
        let ShapeKind::Path(items) = &e.kind else {
            panic!("esperado ShapeKind::Path");
        };
        assert_eq!(items.len(), 4); // MoveTo + 2×LineTo + ClosePath
        assert!(matches!(items[0], PathItem::MoveTo(_)));
        if let PathItem::LineTo(p) = items[1] {
            assert_eq!(p.x.0, 50.0, "x do segundo vértice em pt");
            assert_eq!(p.y.0, 0.0, "y do segundo vértice em pt");
        } else {
            panic!("segundo item deve ser LineTo");
        }
    } else {
        panic!("Esperado Content::Shape com ShapeKind::Path");
    }
}

#[test]
fn p732_polygon_sem_cores_tem_stroke_preta_1pt() {
    // P732 — paridade vanilla `Smart::Auto`
    // (lab/typst-original/crates/typst-layout/src/shapes.rs:336-339):
    // polygon sem fill nem stroke → stroke preta de 1pt. Sem este
    // fallback o polígono renderiza página em branco (medido: 0 px
    // não-brancos no cristalino vs 898 px no vanilla).
    null_ctx!(ctx);
    use crate::entities::layout_types::Color;
    use crate::entities::paint::Paint;
    let args = p732_polygon_args_length();
    let result = native_polygon(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Shape(e)) = result {
        assert!(e.fill.is_none(), "polygon sem fill deve ter fill: None");
        let s = e
            .stroke
            .clone()
            .expect("P732: polygon sem cores deve ter stroke de fallback");
        assert_eq!(
            s.paint,
            Paint::Solid(Color::rgb(0, 0, 0)),
            "stroke de fallback deve ser preta"
        );
        assert_eq!(s.thickness, 1.0, "espessura de fallback deve ser 1pt");
    } else {
        panic!("Esperado Content::Shape");
    }
}

#[test]
fn p732_polygon_com_fill_nao_tem_stroke_fallback() {
    // P732 — paridade vanilla: com fill, o stroke default desaparece.
    null_ctx!(ctx);
    let mut args = p732_polygon_args_length();
    args.named.insert("fill".into(), Value::Color(Color::rgb(255, 0, 0)));
    let result = native_polygon(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Shape(e)) = result {
        assert!(e.fill.is_some(), "fill red deve estar presente");
        assert!(
            e.stroke.is_none(),
            "com fill e sem stroke explícito → stroke deve ser None"
        );
    } else {
        panic!("Esperado Content::Shape");
    }
}

#[test]
fn p732_polygon_com_stroke_explicito_preserva() {
    // P732 — stroke explícito não é substituído pelo fallback.
    null_ctx!(ctx);
    use crate::entities::layout_types::Color;
    use crate::entities::paint::Paint;
    let mut args = p732_polygon_args_length();
    args.named
        .insert("stroke".into(), Value::Color(Color::rgb(0, 0, 255)));
    let result = native_polygon(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Shape(e)) = result {
        let s = e.stroke.clone().expect("stroke explícito deve estar presente");
        assert_eq!(
            s.paint,
            Paint::Solid(Color::rgb(0, 0, 255)),
            "stroke explícito blue deve ser preservado"
        );
    } else {
        panic!("Esperado Content::Shape");
    }
}

#[test]
fn p741_polygon_ratio_relativo_scope_out_mensagem() {
    // **P741** — caminho real do utilizador: `50%` chega como
    // `Value::Relative` (abs zero), não `Value::Ratio`. Antes de P741
    // a mensagem era absurda ("expected relative length, found
    // relative length"). O scope-out mantém-se (resolução exige
    // altura de contentor inline, inexistente — custo medido no
    // relatório do passo), agora com mensagem explícita.
    null_ctx!(ctx);
    let args = Args::positional(vec![Value::Array(vec![
        Value::Relative(crate::entities::rel::Rel {
            rel: 0.5,
            abs: crate::entities::layout_types::Length::ZERO,
        }),
        Value::Length(crate::entities::layout_types::Length::pt(0.0)),
    ])]);
    let err = native_polygon(&mut ctx, &args, &null_world(), test_file_id())
        .expect_err("polygon com coordenada relativa deve ser scope-out (P741)");
    let msg = err.first().map(|d| d.message.to_string()).unwrap_or_default();
    assert!(msg.contains("scope-out") && msg.contains("contentor"), "msg: {msg}");
}

#[test]
fn p734_polygon_rejeita_int_com_mensagem_vanilla() {
    // Medido vanilla: "expected relative length, found integer".
    null_ctx!(ctx);
    let args = Args::positional(vec![
        Value::Array(vec![Value::Int(0), Value::Int(0)]),
        Value::Array(vec![Value::Int(50), Value::Int(0)]),
        Value::Array(vec![Value::Int(25), Value::Int(40)]),
    ]);
    let err = native_polygon(&mut ctx, &args, &null_world(), test_file_id())
        .expect_err("polygon com Int deve ser rejeitado (P734)");
    let msg = err.first().map(|d| d.message.to_string()).unwrap_or_default();
    assert_eq!(msg, "expected relative length, found integer", "msg: {msg}");
}

#[test]
fn p734_polygon_rejeita_float_com_mensagem_vanilla() {
    // Medido vanilla: "expected relative length, found float".
    null_ctx!(ctx);
    let args =
        Args::positional(vec![Value::Array(vec![Value::Float(0.0), Value::Float(0.0)])]);
    let err = native_polygon(&mut ctx, &args, &null_world(), test_file_id())
        .expect_err("polygon com Float deve ser rejeitado (P734)");
    let msg = err.first().map(|d| d.message.to_string()).unwrap_or_default();
    assert_eq!(msg, "expected relative length, found float", "msg: {msg}");
}

#[test]
fn p734_curve_tuplos_com_numeros_sem_regressao() {
    // A restrição a Length NÃO se aplica a curve: a interface por
    // tuples é divergência intencional documentada (o vanilla rejeita
    // tuples — "expected content, found array", medido em P734).
    null_ctx!(ctx);
    let args = Args::positional(vec![
        Value::Array(vec![
            Value::Str("move".into()),
            Value::Array(vec![Value::Float(0.0), Value::Float(0.0)]),
        ]),
        Value::Array(vec![
            Value::Str("line".into()),
            Value::Array(vec![Value::Float(50.0), Value::Float(50.0)]),
        ]),
    ]);
    let result = native_curve(&mut ctx, &args, &null_world(), test_file_id());
    assert!(
        result.is_ok(),
        "curve com tuplos numéricos deve continuar a funcionar: {:?}",
        result.err()
    );
}

#[test]
fn p293_curve_move_line_basico() {
    // curve com move + line + close — equivalente a polygon mas via
    // sintaxe descritiva ("move"/"line"/"close").
    use crate::entities::geometry::PathItem;
    null_ctx!(ctx);
    let args = Args::positional(vec![
        Value::Array(vec![
            Value::Str("move".into()),
            Value::Array(vec![Value::Float(0.0), Value::Float(0.0)]),
        ]),
        Value::Array(vec![
            Value::Str("line".into()),
            Value::Array(vec![Value::Float(100.0), Value::Float(0.0)]),
        ]),
        Value::Array(vec![Value::Str("close".into())]),
    ]);
    let result = native_curve(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Shape(e)) = result {
        let crate::entities::geometry::ShapeKind::Path(items) = &e.kind else {
            panic!("esperado ShapeKind::Path");
        };
        assert_eq!(items.len(), 3);
        assert!(matches!(items[0], PathItem::MoveTo(_)));
        assert!(matches!(items[1], PathItem::LineTo(_)));
        assert!(matches!(items[2], PathItem::ClosePath));
    } else {
        panic!("Esperado Content::Shape com ShapeKind::Path");
    }
}

#[test]
fn p293_curve_cubic_activa_pathitem_cubicto() {
    // **CRÍTICO** — activação inaugural de `PathItem::CubicTo` via
    // stdlib (H6 descoberta empírica A.0.0). Pré-P293, nenhuma
    // stdlib construía CubicTo (apenas P277 `path_bbox` consume +
    // emit já existia em export.rs).
    use crate::entities::geometry::PathItem;
    null_ctx!(ctx);
    let args = Args::positional(vec![
        Value::Array(vec![
            Value::Str("move".into()),
            Value::Array(vec![Value::Float(0.0), Value::Float(0.0)]),
        ]),
        Value::Array(vec![
            Value::Str("cubic".into()),
            Value::Array(vec![Value::Float(25.0), Value::Float(0.0)]), // c1
            Value::Array(vec![Value::Float(75.0), Value::Float(50.0)]), // c2
            Value::Array(vec![Value::Float(100.0), Value::Float(50.0)]), // end
        ]),
    ]);
    let result = native_curve(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Shape(e)) = result {
        let crate::entities::geometry::ShapeKind::Path(items) = &e.kind else {
            panic!("esperado ShapeKind::Path");
        };
        assert_eq!(items.len(), 2);
        assert!(matches!(items[0], PathItem::MoveTo(_)));
        // **Activação pos-P293 — CubicTo construído via stdlib**
        assert!(
            matches!(items[1], PathItem::CubicTo(_, _, _)),
            "P293 H6: CubicTo deve ser construível via curve('cubic', c1, c2, end)"
        );
    } else {
        panic!("Esperado Content::Shape com ShapeKind::Path");
    }
}

#[test]
fn p293_curve_cubic_preserva_control_points() {
    // Verificar literalmente que c1/c2/end são preservados na ordem
    // certa no PathItem::CubicTo construído.
    use crate::entities::geometry::PathItem;
    null_ctx!(ctx);
    let args = Args::positional(vec![Value::Array(vec![
        Value::Str("cubic".into()),
        Value::Array(vec![Value::Float(10.0), Value::Float(20.0)]),
        Value::Array(vec![Value::Float(30.0), Value::Float(40.0)]),
        Value::Array(vec![Value::Float(50.0), Value::Float(60.0)]),
    ])]);
    let result = native_curve(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Shape(e)) = result {
        let crate::entities::geometry::ShapeKind::Path(items) = &e.kind else {
            panic!("esperado ShapeKind::Path");
        };
        assert_eq!(items.len(), 1);
        if let PathItem::CubicTo(c1, c2, end) = items[0] {
            assert_eq!((c1.x.val(), c1.y.val()), (10.0, 20.0));
            assert_eq!((c2.x.val(), c2.y.val()), (30.0, 40.0));
            assert_eq!((end.x.val(), end.y.val()), (50.0, 60.0));
        } else {
            panic!("Esperado PathItem::CubicTo");
        }
    }
}

#[test]
fn p294_curve_quadratic_activa_via_conversao_q2c() {
    // Substitui o anterior p293_curve_quadratic_scope_out — o scope-out
    // foi removido em P294 H1' por descoberta empírica vanilla.
    null_ctx!(ctx);
    let args = Args::positional(vec![
        Value::Array(vec![
            Value::Str("move".into()),
            Value::Array(vec![Value::Float(0.0), Value::Float(0.0)]),
        ]),
        Value::Array(vec![
            Value::Str("quadratic".into()),
            Value::Array(vec![Value::Float(10.0), Value::Float(20.0)]), // control
            Value::Array(vec![Value::Float(30.0), Value::Float(40.0)]), // end
        ]),
    ]);
    let result = native_curve(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    // Esperado: 2 PathItems — MoveTo + CubicTo (conversão q→c).
    if let Value::Content(Content::Shape(e)) = &result {
        let crate::entities::geometry::ShapeKind::Path(items) = &e.kind else {
            panic!("esperado ShapeKind::Path");
        };
        assert_eq!(items.len(), 2, "esperava 2 items (MoveTo + CubicTo)");
        assert!(matches!(items[0], crate::entities::geometry::PathItem::MoveTo(_)));
        assert!(
            matches!(items[1], crate::entities::geometry::PathItem::CubicTo(_, _, _)),
            "quadratic deve materializar como CubicTo via conversão q→c"
        );
    } else {
        panic!("esperava Content::Shape com ShapeKind::Path");
    }
}

#[test]
fn p294_curve_quadratic_aplica_formula_q2c_exacta() {
    // Verifica fórmula matemática exacta:
    // P0 = (0, 0), Q = (10, 20), P2 = (30, 40)
    // C1 = (P0 + 2Q) / 3 = (20/3, 40/3)
    // C2 = (P2 + 2Q) / 3 = (50/3, 80/3)
    use crate::entities::geometry::PathItem;
    null_ctx!(ctx);
    let args = Args::positional(vec![
        Value::Array(vec![
            Value::Str("move".into()),
            Value::Array(vec![Value::Float(0.0), Value::Float(0.0)]),
        ]),
        Value::Array(vec![
            Value::Str("quadratic".into()),
            Value::Array(vec![Value::Float(10.0), Value::Float(20.0)]),
            Value::Array(vec![Value::Float(30.0), Value::Float(40.0)]),
        ]),
    ]);
    let result = native_curve(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Shape(e)) = &result {
        let crate::entities::geometry::ShapeKind::Path(items) = &e.kind else {
            panic!("esperado ShapeKind::Path");
        };
        if let PathItem::CubicTo(c1, c2, end) = items[1] {
            let eps = 1e-9;
            assert!((c1.x.0 - 20.0 / 3.0).abs() < eps, "C1.x: {}", c1.x.0);
            assert!((c1.y.0 - 40.0 / 3.0).abs() < eps, "C1.y: {}", c1.y.0);
            assert!((c2.x.0 - 50.0 / 3.0).abs() < eps, "C2.x: {}", c2.x.0);
            assert!((c2.y.0 - 80.0 / 3.0).abs() < eps, "C2.y: {}", c2.y.0);
            assert!((end.x.0 - 30.0).abs() < eps);
            assert!((end.y.0 - 40.0).abs() < eps);
        } else {
            panic!("esperava CubicTo");
        }
    }
}

#[test]
fn p294_curve_quadratic_sem_move_anterior_usa_origem() {
    // Caso fronteira: quadratic antes de qualquer move.
    // last_point fallback (0,0); curva é válida.
    use crate::entities::geometry::PathItem;
    null_ctx!(ctx);
    let args = Args::positional(vec![Value::Array(vec![
        Value::Str("quadratic".into()),
        Value::Array(vec![Value::Float(6.0), Value::Float(0.0)]),
        Value::Array(vec![Value::Float(12.0), Value::Float(0.0)]),
    ])]);
    let result = native_curve(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Shape(e)) = &result {
        let crate::entities::geometry::ShapeKind::Path(items) = &e.kind else {
            panic!("esperado ShapeKind::Path");
        };
        assert_eq!(items.len(), 1);
        if let PathItem::CubicTo(c1, _, _) = items[0] {
            // P0 = (0,0), Q = (6,0) → C1 = (0+12)/3 = 4
            let eps = 1e-9;
            assert!((c1.x.0 - 4.0).abs() < eps);
            assert!(c1.y.0.abs() < eps);
        }
    }
}

#[test]
fn p294_curve_quadratic_aridade_errada_retorna_err() {
    null_ctx!(ctx);
    let args = Args::positional(vec![Value::Array(vec![
        Value::Str("quadratic".into()),
        Value::Array(vec![Value::Float(1.0), Value::Float(2.0)]),
        // falta end
    ])]);
    let err = native_curve(&mut ctx, &args, &null_world(), test_file_id()).unwrap_err();
    assert!(format!("{:?}", err).contains("quadratic"));
}

#[test]
fn p294_curve_quadratic_encadeada_actualiza_last_point() {
    // Duas quadratics consecutivas: a segunda usa o end da primeira
    // como P0 (last_point).
    use crate::entities::geometry::PathItem;
    null_ctx!(ctx);
    let args = Args::positional(vec![
        Value::Array(vec![
            Value::Str("move".into()),
            Value::Array(vec![Value::Float(0.0), Value::Float(0.0)]),
        ]),
        Value::Array(vec![
            Value::Str("quadratic".into()),
            Value::Array(vec![Value::Float(3.0), Value::Float(0.0)]),
            Value::Array(vec![Value::Float(6.0), Value::Float(0.0)]), // end = (6,0)
        ]),
        Value::Array(vec![
            Value::Str("quadratic".into()),
            Value::Array(vec![Value::Float(9.0), Value::Float(0.0)]),
            Value::Array(vec![Value::Float(12.0), Value::Float(0.0)]),
        ]),
    ]);
    let result = native_curve(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Shape(e)) = &result {
        let crate::entities::geometry::ShapeKind::Path(items) = &e.kind else {
            panic!("esperado ShapeKind::Path");
        };
        assert_eq!(items.len(), 3);
        // Segunda quadratic: P0=(6,0), Q=(9,0) → C1 = (6 + 18)/3 = 8
        if let PathItem::CubicTo(c1, _, _) = items[2] {
            let eps = 1e-9;
            assert!(
                (c1.x.0 - 8.0).abs() < eps,
                "last_point tracking falhou: c1.x={} (esperava 8.0)",
                c1.x.0
            );
        }
    }
}

#[test]
fn p294_curve_quadratic_e_cubic_misturados() {
    use crate::entities::geometry::PathItem;
    null_ctx!(ctx);
    let args = Args::positional(vec![
        Value::Array(vec![
            Value::Str("move".into()),
            Value::Array(vec![Value::Float(0.0), Value::Float(0.0)]),
        ]),
        Value::Array(vec![
            Value::Str("quadratic".into()),
            Value::Array(vec![Value::Float(5.0), Value::Float(10.0)]),
            Value::Array(vec![Value::Float(10.0), Value::Float(0.0)]),
        ]),
        Value::Array(vec![
            Value::Str("cubic".into()),
            Value::Array(vec![Value::Float(15.0), Value::Float(5.0)]),
            Value::Array(vec![Value::Float(20.0), Value::Float(-5.0)]),
            Value::Array(vec![Value::Float(25.0), Value::Float(0.0)]),
        ]),
        Value::Array(vec![Value::Str("close".into())]),
    ]);
    let result = native_curve(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Shape(e)) = &result {
        let crate::entities::geometry::ShapeKind::Path(items) = &e.kind else {
            panic!("esperado ShapeKind::Path");
        };
        assert_eq!(items.len(), 4);
        assert!(matches!(items[0], PathItem::MoveTo(_)));
        assert!(matches!(items[1], PathItem::CubicTo(_, _, _)));
        assert!(matches!(items[2], PathItem::CubicTo(_, _, _)));
        assert!(matches!(items[3], PathItem::ClosePath));
    }
}

#[test]
fn p293_curve_kind_desconhecido_retorna_err() {
    null_ctx!(ctx);
    let args = Args::positional(vec![Value::Array(vec![Value::Str("bogus".into())])]);
    let err = native_curve(&mut ctx, &args, &null_world(), test_file_id()).unwrap_err();
    assert!(format!("{:?}", err).contains("desconhecido"));
}

#[test]
fn p293_curve_vazia_retorna_err() {
    null_ctx!(ctx);
    let args = Args::positional(vec![]);
    let err = native_curve(&mut ctx, &args, &null_world(), test_file_id()).unwrap_err();
    assert!(format!("{:?}", err).contains("pelo menos um segmento"));
}

#[test]
fn p803_curve_arg_nao_content_erro_paridade_vanilla() {
    // P803 — argumento posicional que não é `Content::Curve` nem tuplo
    // legado (array com 1º elemento string) → `expected content, found
    // {type}` (paridade vanilla 0.15.0 medida: `#curve((0pt, 0pt))` →
    // "expected content, found array"). Antes: mensagens próprias
    // ("argumento N não é um array de segmento válido" / "primeiro
    // elemento deve ser string (kind)").
    null_ctx!(ctx);
    // Caso do achado: array de lengths (ponto bare).
    let args = Args::positional(vec![Value::Array(vec![
        Value::Length(Length::pt(0.0)),
        Value::Length(Length::pt(0.0)),
    ])]);
    let err = native_curve(&mut ctx, &args, &null_world(), test_file_id()).unwrap_err();
    let msg = format!("{:?}", err);
    assert!(
        msg.contains("expected content, found array"),
        "mensagem deve ser a do vanilla, obtido: {msg}"
    );
    // Controlo genérico: não-array, não-content → mesmo padrão.
    let args2 = Args::positional(vec![Value::Int(42)]);
    let err2 = native_curve(&mut ctx, &args2, &null_world(), test_file_id()).unwrap_err();
    let msg2 = format!("{:?}", err2);
    assert!(
        msg2.contains("expected content, found int"),
        "mensagem deve ser a do vanilla, obtido: {msg2}"
    );
}

#[test]
fn p293_curve_bbox_analitica_via_path_bbox_p277() {
    // Verificar que bbox cubic é calculada via P277 analítica
    // (não via min/max dos control points).
    use crate::entities::geometry::ShapeKind;
    let _: ShapeKind = ShapeKind::Rect; // sanity
    null_ctx!(ctx);
    let args = Args::positional(vec![
        Value::Array(vec![
            Value::Str("move".into()),
            Value::Array(vec![Value::Float(0.0), Value::Float(0.0)]),
        ]),
        Value::Array(vec![
            Value::Str("cubic".into()),
            Value::Array(vec![Value::Float(0.0), Value::Float(100.0)]), // c1 alto
            Value::Array(vec![Value::Float(100.0), Value::Float(100.0)]), // c2 alto
            Value::Array(vec![Value::Float(100.0), Value::Float(0.0)]), // end
        ]),
    ]);
    let result = native_curve(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Shape(e)) = result {
        // e.height da curva deve ser < 100 (extremo analítico, não 100 control point)
        // P277 calcula extremo da Bézier que é menor que max control point.
        let h = match e.height.as_deref() {
            Some(Value::Float(f)) => *f,
            _ => 0.0,
        };
        let w = match e.width.as_deref() {
            Some(Value::Float(f)) => *f,
            _ => 0.0,
        };
        assert!(w > 0.0 && h > 0.0, "bbox deve ser positiva; got w={w} h={h}");
        assert!(
            h < 100.0,
            "P277 bbox analítica: extremo da Bézier < max control point (100); got h={h}"
        );
    }
}

#[test]
fn p293_curve_named_fill_e_stroke() {
    use crate::entities::layout_types::Color;
    null_ctx!(ctx);
    let mut args = Args::positional(vec![
        Value::Array(vec![
            Value::Str("move".into()),
            Value::Array(vec![Value::Float(0.0), Value::Float(0.0)]),
        ]),
        Value::Array(vec![
            Value::Str("line".into()),
            Value::Array(vec![Value::Float(50.0), Value::Float(50.0)]),
        ]),
    ]);
    args.named.insert("fill".into(), Value::Color(Color::rgb(255, 0, 0)));
    args.named
        .insert("stroke".into(), Value::Color(Color::rgb(0, 0, 255)));
    let result = native_curve(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Shape(e)) = result {
        assert!(e.fill.is_some(), "fill deve ser parseado");
        assert!(e.stroke.is_some(), "stroke deve ser parseado");
    }
}

#[test]
fn p513_curve_move_devolve_content_curve() {
    use crate::entities::elements::curve::CurveSegment;
    use crate::entities::layout_types::Length;
    null_ctx!(ctx);
    let args = Args::positional(vec![Value::Array(vec![
        Value::Float(10.0),
        Value::Length(Length::pt(20.0)),
    ])]);
    let result =
        native_curve_move(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Curve(e)) = result {
        assert_eq!(e.segments.len(), 1);
        assert!(
            matches!(&e.segments[0], CurveSegment::Move(p) if p.x == Length::pt(10.0) && p.y == Length::pt(20.0))
        );
    } else {
        panic!("esperado Content::Curve");
    }
}

#[test]
fn p513_curve_line_devolve_content_curve() {
    use crate::entities::elements::curve::CurveSegment;
    use crate::entities::layout_types::Length;
    null_ctx!(ctx);
    let args = Args::positional(vec![Value::Array(vec![Value::Int(5), Value::Int(15)])]);
    let result =
        native_curve_line(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Curve(e)) = result {
        assert!(
            matches!(&e.segments[0], CurveSegment::Line(p) if p.x == Length::pt(5.0) && p.y == Length::pt(15.0))
        );
    } else {
        panic!("esperado Content::Curve");
    }
}

#[test]
fn p513_curve_cubic_devolve_content_curve() {
    use crate::entities::elements::curve::CurveSegment;
    use crate::entities::layout_types::Length;
    null_ctx!(ctx);
    let args = Args::positional(vec![
        Value::Array(vec![Value::Float(0.0), Value::Float(0.0)]),
        Value::Array(vec![Value::Float(50.0), Value::Float(50.0)]),
        Value::Array(vec![Value::Float(100.0), Value::Float(0.0)]),
    ]);
    let result =
        native_curve_cubic(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Curve(e)) = result {
        assert!(matches!(&e.segments[0], CurveSegment::Cubic(c1, c2, end)
            if c1.x == Length::pt(0.0) && c1.y == Length::pt(0.0)
            && c2.x == Length::pt(50.0) && c2.y == Length::pt(50.0)
            && end.x == Length::pt(100.0) && end.y == Length::pt(0.0)));
    } else {
        panic!("esperado Content::Curve");
    }
}

#[test]
fn p513_curve_quad_devolve_content_curve() {
    use crate::entities::elements::curve::CurveSegment;
    use crate::entities::layout_types::Length;
    null_ctx!(ctx);
    let args = Args::positional(vec![
        Value::Array(vec![Value::Float(25.0), Value::Float(25.0)]),
        Value::Array(vec![Value::Float(50.0), Value::Float(0.0)]),
    ]);
    let result =
        native_curve_quad(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Curve(e)) = result {
        assert!(matches!(&e.segments[0], CurveSegment::Quad(c, end)
            if c.x == Length::pt(25.0) && c.y == Length::pt(25.0)
            && end.x == Length::pt(50.0) && end.y == Length::pt(0.0)));
    } else {
        panic!("esperado Content::Curve");
    }
}

#[test]
fn p513_curve_close_devolve_content_curve() {
    use crate::entities::elements::curve::CurveSegment;
    null_ctx!(ctx);
    let args = Args::positional(vec![]);
    let result =
        native_curve_close(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Curve(e)) = result {
        assert!(matches!(e.segments[0], CurveSegment::Close(_)));
    } else {
        panic!("esperado Content::Curve");
    }
}

#[test]
fn p513_curve_aceita_content_curve_como_argumento() {
    // `curve(curve.move(...), curve.line(...), curve.close())` deve
    // concatenar os segmentos num `Content::Shape` final.
    use crate::entities::geometry::{PathItem, ShapeKind};
    null_ctx!(ctx);
    let seg1 = native_curve_move(
        &mut ctx,
        &Args::positional(vec![Value::Array(vec![Value::Float(0.0), Value::Float(0.0)])]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    let seg2 = native_curve_line(
        &mut ctx,
        &Args::positional(vec![Value::Array(vec![
            Value::Float(100.0),
            Value::Float(0.0),
        ])]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    let seg3 = native_curve_close(
        &mut ctx,
        &Args::positional(vec![]),
        &null_world(),
        test_file_id(),
    )
    .unwrap();
    let args = Args::positional(vec![seg1, seg2, seg3]);
    let result = native_curve(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Shape(e)) = result {
        let ShapeKind::Path(items) = &e.kind else { panic!("esperado Path") };
        assert_eq!(items.len(), 4);
        assert!(matches!(items[0], PathItem::MoveTo(_)));
        assert!(matches!(items[1], PathItem::LineTo(_)));
        let PathItem::CubicTo(c1, c2, end) = items[2] else {
            panic!("curve.close() default deve preservar fechamento smooth")
        };
        assert_eq!((c1.x.0, c1.y.0), (100.0, 0.0));
        assert_eq!((c2.x.0, c2.y.0), (0.0, 0.0));
        assert_eq!((end.x.0, end.y.0), (0.0, 0.0));
        assert!(matches!(items[3], PathItem::ClosePath));
    } else {
        panic!("esperado Content::Shape");
    }
}

#[test]
fn p727_curve_sem_cores_tem_stroke_preta_1pt() {
    // P727 — paridade vanilla `Smart::Auto`
    // (lab/typst-original/crates/typst-layout/src/shapes.rs:126-129):
    // curve sem fill nem stroke → stroke preta de 1pt (fallback
    // idêntico a `native_rect`). Sem este fallback a curva renderiza
    // página em branco (path no PDF sem operador de pintura).
    null_ctx!(ctx);
    use crate::entities::layout_types::Color;
    use crate::entities::paint::Paint;
    let result =
        native_curve(&mut ctx, &p727_curve_args_minimos(), &null_world(), test_file_id())
            .unwrap();
    if let Value::Content(Content::Shape(e)) = result {
        assert!(e.fill.is_none(), "curve sem fill deve ter fill: None");
        let s = e
            .stroke
            .clone()
            .expect("P727: curve sem cores deve ter stroke de fallback");
        assert_eq!(
            s.paint,
            Paint::Solid(Color::rgb(0, 0, 0)),
            "stroke de fallback deve ser preta"
        );
        assert_eq!(s.thickness, 1.0, "espessura de fallback deve ser 1pt");
    } else {
        panic!("Esperado Content::Shape");
    }
}

#[test]
fn p727_curve_com_fill_nao_tem_stroke_fallback() {
    // P727 — paridade vanilla: "When setting a fill, the default stroke
    // disappears" (curve.rs:46-47 vanilla).
    null_ctx!(ctx);
    let mut args = p727_curve_args_minimos();
    args.named.insert("fill".into(), Value::Color(Color::rgb(255, 0, 0)));
    let result = native_curve(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Shape(e)) = result {
        assert!(e.fill.is_some(), "fill red deve estar presente");
        assert!(
            e.stroke.is_none(),
            "com fill e sem stroke explícito → stroke deve ser None"
        );
    } else {
        panic!("Esperado Content::Shape");
    }
}

#[test]
fn p727_curve_com_stroke_explicito_preserva() {
    // P727 — stroke explícito não é substituído pelo fallback.
    null_ctx!(ctx);
    use crate::entities::layout_types::Color;
    use crate::entities::paint::Paint;
    let mut args = p727_curve_args_minimos();
    args.named
        .insert("stroke".into(), Value::Color(Color::rgb(0, 0, 255)));
    let result = native_curve(&mut ctx, &args, &null_world(), test_file_id()).unwrap();
    if let Value::Content(Content::Shape(e)) = result {
        let s = e.stroke.clone().expect("stroke explícito deve estar presente");
        assert_eq!(
            s.paint,
            Paint::Solid(Color::rgb(0, 0, 255)),
            "stroke explícito blue deve ser preservado"
        );
    } else {
        panic!("Esperado Content::Shape");
    }
}

#[test]
fn parse_color_nomes_conhecidos() {
    let color = Color::rgb(1, 2, 3);
    assert_eq!(parse_color(&Value::Color(color.clone())), Some(color));
    assert_eq!(parse_color(&Value::Str("red".into())), None);
    assert_eq!(parse_color(&Value::Int(42)), None);
}

#[test]
fn p1455_parse_color_nao_reintroduz_catalogo_string() {
    for name in [
        "red",
        "green",
        "blue",
        "black",
        "white",
        "yellow",
        "cyan",
        "magenta",
        "orange",
        "purple",
        "gray",
        "grey",
        "silver",
        "maroon",
        "navy",
        "olive",
        "teal",
        "lime",
        "aqua",
        "ultraviolet",
    ] {
        assert_eq!(parse_color(&Value::Str(name.into())), None, "string {name}");
    }
}

fn stroke_paint_shape_fixture() -> crate::entities::gradient::Gradient {
    use crate::entities::gradient::{Gradient, GradientStop};
    Gradient::linear(
        vec![
            GradientStop::unspaced(Color::rgb(255, 65, 54)),
            GradientStop::unspaced(Color::rgb(0, 116, 217)),
        ],
        Angle::rad(0.0),
    )
}

#[test]
fn stroke_paint_shapes_direct_preserve_gradient_and_tiling() {
    use crate::entities::{
        paint::Paint,
        tiling::{Tiling, TilingBody},
    };
    null_ctx!(ctx);
    let g = stroke_paint_shape_fixture();
    let t = Tiling::new(TilingBody::Color(Color::rgb(255, 65, 54)));
    for (input, paint) in [
        (Value::Gradient(g.clone()), Paint::Gradient(g)),
        (Value::Tiling(Arc::new(t.clone())), Paint::Tiling(t)),
    ] {
        let args = pn(vec![], "stroke", input);
        for result in [
            native_rect(&mut ctx, &args, &null_world(), test_file_id()),
            native_ellipse(&mut ctx, &args, &null_world(), test_file_id()),
            native_circle(&mut ctx, &args, &null_world(), test_file_id()),
        ] {
            let Value::Content(Content::Shape(shape)) = result.unwrap() else {
                panic!("shape")
            };
            let stroke = shape.stroke.as_ref().expect("explicit stroke");
            assert_eq!(stroke.paint, paint);
            assert_eq!(stroke.thickness, 1.0);
            assert!(stroke.specified.paint);
        }
    }
}

#[test]
fn stroke_paint_uniform_dictionary_preserves_rich_fields() {
    use crate::entities::{geometry::LineCap, paint::Paint};
    null_ctx!(ctx);
    let gradient = stroke_paint_shape_fixture();
    let mut dict = IndexMap::with_hasher(FxBuildHasher);
    dict.insert("paint".into(), Value::Gradient(gradient.clone()));
    dict.insert("thickness".into(), Value::Length(Length::pt(12.0)));
    dict.insert("cap".into(), Value::Str("round".into()));
    let mut args = pn(vec![], "stroke", Value::Dict(dict));
    args.named.insert("width".into(), Value::Length(Length::pt(40.0)));
    for result in [
        native_rect(&mut ctx, &args, &null_world(), test_file_id()),
        native_square(&mut ctx, &args, &null_world(), test_file_id()),
    ] {
        let Value::Content(Content::Shape(shape)) = result.unwrap() else {
            panic!("shape")
        };
        let stroke = shape.stroke.as_ref().expect("explicit stroke");
        assert_eq!(stroke.paint, Paint::Gradient(gradient.clone()));
        assert_eq!(stroke.thickness, 12.0);
        assert_eq!(stroke.cap, LineCap::Round);
        assert!(
            stroke.specified.paint && stroke.specified.thickness && stroke.specified.cap
        );
    }
}

#[test]
fn stroke_paint_empty_dictionary_disables_rect_and_square_contour() {
    null_ctx!(ctx);
    let mut args =
        pn(vec![], "stroke", Value::Dict(IndexMap::with_hasher(FxBuildHasher)));
    args.named.insert("width".into(), Value::Length(Length::pt(40.0)));
    for result in [
        native_rect(&mut ctx, &args, &null_world(), test_file_id()),
        native_square(&mut ctx, &args, &null_world(), test_file_id()),
    ] {
        let Value::Content(Content::Shape(shape)) = result.unwrap() else {
            panic!("shape")
        };
        assert!(shape.stroke.is_none());
        assert!(shape.fill.is_none());
    }
}

#[test]
fn stroke_paint_invalid_uniform_dictionary_is_not_discarded() {
    null_ctx!(ctx);
    for (key, value) in
        [("paint", Value::Int(42)), ("wrong", Value::Length(Length::pt(2.0)))]
    {
        let mut dict = IndexMap::with_hasher(FxBuildHasher);
        dict.insert(key.into(), value);
        let args = pn(vec![], "stroke", Value::Dict(dict));
        assert!(native_rect(&mut ctx, &args, &null_world(), test_file_id()).is_err());
    }
}
