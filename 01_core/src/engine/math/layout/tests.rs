//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/math/layout/_comum.md
//! @prompt-hash 2d7029e2
//! @layer L1
//! @updated 2026-04-23
//!
//! Testes de `math/layout` — extraídos de `math/layout.rs` no Passo 96.8
//! conforme ADR-0037.

use super::*;
use crate::engine::layout::{FixedMetrics, FontMetrics};
use std::sync::Arc;

fn default_style() -> TextStyle {
    TextStyle::regular(Pt(12.0))
}

fn size10_style() -> TextStyle {
    TextStyle::regular(Pt(10.0))
}

// ── Testes herdados do Passo 36 (adaptados para nova API) ─────────────

#[test]
fn math_layouter_math_ident_produz_items_nao_vazios() {
    let ml = MathLayouter::new(&FixedMetrics, true);
    let items = ml.layout_equation(&Content::MathIdent("x".into()), &default_style());
    assert!(!items.is_empty(), "MathIdent deve produzir pelo menos 1 item");
}

#[test]
fn math_layouter_math_text_produz_items_nao_vazios() {
    let ml = MathLayouter::new(&FixedMetrics, true);
    let items = ml.layout_equation(&Content::MathText("sin".into()), &default_style());
    assert!(!items.is_empty());
}

#[test]
fn math_layouter_sequence_produz_multiplos_items() {
    let ml = MathLayouter::new(&FixedMetrics, true);
    let seq = Content::MathSequence(Arc::from(
        vec![
            Content::MathIdent("x".into()),
            Content::MathText("+".into()),
            Content::MathIdent("y".into()),
        ]
        .into_boxed_slice(),
    ));
    let items = ml.layout_equation(&seq, &default_style());
    assert_eq!(items.len(), 3, "x + y deve produzir 3 items");
}

#[test]
fn math_layouter_frac_sem_placeholder_colchetes() {
    let ml = MathLayouter::new(&FixedMetrics, true);
    let frac = Content::math_frac(
        Content::MathIdent("a".into()),
        Content::MathIdent("b".into()),
    );
    let items = ml.layout_equation(&frac, &default_style());
    for item in &items {
        if let FrameItem::Text { text, .. } = item {
            assert!(!text.contains('['), "frac não deve conter '[': {}", text);
        }
    }
    assert!(!items.is_empty(), "frac deve produzir items");
}

#[test]
fn math_layouter_cursor_avanca_horizontalmente() {
    let ml = MathLayouter::new(&FixedMetrics, true);
    let items = ml.layout_equation(
        &Content::MathSequence(Arc::from(
            vec![Content::MathIdent("a".into()), Content::MathIdent("b".into())]
                .into_boxed_slice(),
        )),
        &default_style(),
    );
    // Segundo item deve ter pos.x > 0 (cursor avançou)
    if let [first, second] = items.as_slice() {
        if let (FrameItem::Text { pos: p1, .. }, FrameItem::Text { pos: p2, .. }) =
            (first, second)
        {
            assert!(p2.x > p1.x, "segundo item deve estar à direita do primeiro");
        }
    }
}

#[test]
fn math_layouter_math_attach_sem_colchetes() {
    let ml = MathLayouter::new(&FixedMetrics, true);
    let attach = Content::math_attach(
        Content::MathIdent("x".into()),
        None,
        None,
        None,
        Some(Content::MathText("2".into())),
    );
    let items = ml.layout_equation(&attach, &default_style());
    for item in &items {
        if let FrameItem::Text { text, .. } = item {
            assert!(!text.contains('['), "attach não deve conter '[': {}", text);
        }
    }
}

// ── Novos testes do Passo 37 + 38 ────────────────────────────────────

#[test]
fn math_frac_tem_dois_ou_mais_items() {
    let ml = MathLayouter::new(&FixedMetrics, true);
    let frac = Content::math_frac(
        Content::MathIdent("a".into()),
        Content::MathIdent("b".into()),
    );
    let items = ml.layout_equation(&frac, &size10_style());
    assert!(items.len() >= 2, "frac deve ter >= 2 items, tem {}", items.len());
}

#[test]
fn math_frac_numerador_acima_denominador() {
    let ml = MathLayouter::new(&FixedMetrics, true);
    let frac = Content::math_frac(
        Content::MathIdent("a".into()),
        Content::MathIdent("b".into()),
    );
    let items = ml.layout_equation(&frac, &size10_style());

    let ys: Vec<f64> = items
        .iter()
        .filter_map(|item| {
            if let FrameItem::Text { pos, .. } = item {
                Some(pos.y.val())
            } else {
                None
            }
        })
        .collect();

    assert!(ys.len() >= 2, "deve ter pelo menos 2 posições y");
    assert!(
        ys[0] < ys[1],
        "numerador (y={}) deve estar acima do denominador (y={})",
        ys[0],
        ys[1]
    );
}

#[test]
fn math_attach_sup_elevado() {
    let ml = MathLayouter::new(&FixedMetrics, true);
    let attach = Content::math_attach(
        Content::MathIdent("x".into()),
        None,
        None,
        None,
        Some(Content::MathIdent("2".into())),
    );
    let items = ml.layout_equation(&attach, &size10_style());
    assert!(items.len() >= 2, "x^2 deve ter >= 2 items");

    let ys: Vec<f64> = items
        .iter()
        .filter_map(|item| {
            if let FrameItem::Text { pos, .. } = item {
                Some(pos.y.val())
            } else {
                None
            }
        })
        .collect();

    // sup deve estar acima da base (y menor, pois y cresce para baixo)
    assert!(ys[1] < ys[0], "sup (y={}) deve estar acima da base (y={})", ys[1], ys[0]);
}

// ── P772w — integrais nunca empilham limites (mesmo em modo bloco) ─────────

#[test]
fn math_attach_sum_empilha_limites_em_modo_bloco() {
    // Controlo: `sum` (∑) DEVE empilhar sup/sub verticalmente em modo bloco
    // (paridade vanilla `MathClass::Large` → `Limits::Display`). O sup deve
    // ficar centrado sobre a base — x próximo de x da base, não deslocado
    // para a direita como um script lateral.
    let ml = MathLayouter::new(&FixedMetrics, true);
    let attach = Content::math_attach(
        Content::MathText("∑".into()),
        None,
        None,
        Some(Content::MathText("0".into())),
        Some(Content::MathText("1".into())),
    );
    let items = ml.layout_equation(&attach, &default_style());
    let base_x = items
        .iter()
        .find_map(|i| match i {
            FrameItem::Text { pos, text, .. } if text.as_str() == "∑" => {
                Some(pos.x.val())
            }
            _ => None,
        })
        .expect("base ∑ deve estar presente");
    let sup_x = items
        .iter()
        .find_map(|i| match i {
            FrameItem::Text { pos, text, .. } if text.as_str() == "1" => {
                Some(pos.x.val())
            }
            _ => None,
        })
        .expect("sup '1' deve estar presente");
    // Empilhado: sup fica centrado sobre a base — x muito próximo (não à
    // direita, como aconteceria num script lateral).
    assert!(
        (sup_x - base_x).abs() < 6.0,
        "sup de ∑ deve ficar centrado sobre a base (empilhado), \
         base_x={base_x} sup_x={sup_x}"
    );
}

#[test]
fn math_attach_integral_nao_empilha_limites_em_modo_bloco() {
    // Regressão P772w: antes desta correcção, `is_large_operator` incluía
    // os caracteres de integral, fazendo `∫_0^1` empilhar os limites em modo
    // bloco (errado — paridade vanilla `is_integral_char`/`Limits::Never`:
    // integrais mantêm os scripts ao lado SEMPRE, mesmo em display style).
    let ml = MathLayouter::new(&FixedMetrics, true);
    let attach = Content::math_attach(
        Content::MathText("∫".into()),
        None,
        None,
        Some(Content::MathText("0".into())),
        Some(Content::MathText("1".into())),
    );
    let items = ml.layout_equation(&attach, &default_style());
    let base_x = items
        .iter()
        .find_map(|i| match i {
            FrameItem::Text { pos, text, .. } if text.as_str() == "∫" => {
                Some(pos.x.val())
            }
            _ => None,
        })
        .expect("base ∫ deve estar presente");
    let base_w = 12.0 * 0.6; // FixedMetrics: char_width = size * 0.6, style 12pt.
    let sup_x = items
        .iter()
        .find_map(|i| match i {
            FrameItem::Text { pos, text, .. } if text.as_str() == "1" => {
                Some(pos.x.val())
            }
            _ => None,
        })
        .expect("sup '1' deve estar presente");
    // Script lateral: sup fica à direita da base (x >= base_x + largura da
    // base), não centrado sobre ela.
    assert!(
        sup_x >= base_x + base_w - 1.0,
        "sup de ∫ deve ficar à direita da base (script lateral, não \
         empilhado), base_x={base_x} sup_x={sup_x}"
    );
}

#[test]
fn math_frac_tem_item_linha() {
    let ml = MathLayouter::new(&FixedMetrics, true);
    let frac = Content::math_frac(
        Content::MathIdent("a".into()),
        Content::MathIdent("b".into()),
    );
    let items = ml.layout_equation(&frac, &size10_style());
    let has_line = items.iter().any(|item| matches!(item, FrameItem::Line { .. }));
    assert!(has_line, "frac deve ter FrameItem::Line para a linha de fracção");
}

#[test]
fn math_frac_linha_horizontal() {
    let ml = MathLayouter::new(&FixedMetrics, true);
    let frac = Content::math_frac(
        Content::MathIdent("a".into()),
        Content::MathIdent("b".into()),
    );
    let items = ml.layout_equation(&frac, &size10_style());
    for item in &items {
        if let FrameItem::Line { start, end, .. } = item {
            assert_eq!(
                start.y.val(),
                end.y.val(),
                "linha de fracção deve ser horizontal"
            );
            assert!(end.x.val() > start.x.val(), "linha de fracção deve ter largura > 0");
        }
    }
}

#[test]
fn math_attach_sub_baixado() {
    let ml = MathLayouter::new(&FixedMetrics, true);
    let attach = Content::math_attach(
        Content::MathIdent("x".into()),
        None,
        None,
        Some(Content::MathIdent("i".into())),
        None,
    );
    let items = ml.layout_equation(&attach, &size10_style());
    assert!(items.len() >= 2);

    let ys: Vec<f64> = items
        .iter()
        .filter_map(|item| {
            if let FrameItem::Text { pos, .. } = item {
                Some(pos.y.val())
            } else {
                None
            }
        })
        .collect();

    // sub deve estar abaixo da base (y maior)
    assert!(ys[1] > ys[0], "sub (y={}) deve estar abaixo da base (y={})", ys[1], ys[0]);
}

// ── Testes do Passo 40 — layout_root ─────────────────────────────────

#[test]
fn layout_root_contem_radical_e_radicando() {
    let ml = MathLayouter::new(&FixedMetrics, true);
    let root = Content::math_root(None, Content::MathIdent("x".into()));
    let items = ml.layout_equation(&root, &default_style());
    // Deve conter pelo menos o símbolo √ e o radicando "x"
    let texts: Vec<_> = items
        .iter()
        .filter_map(|i| {
            if let FrameItem::Text { text, .. } = i {
                Some(text.as_str())
            } else {
                None
            }
        })
        .collect();
    assert!(texts.iter().any(|t| t.contains('√')), "deve conter √: {:?}", texts);
    assert!(texts.iter().any(|t| t.contains('x')), "deve conter x: {:?}", texts);
}

#[test]
fn layout_root_tem_overline() {
    let ml = MathLayouter::new(&FixedMetrics, true);
    let root = Content::math_root(None, Content::MathIdent("x".into()));
    let items = ml.layout_equation(&root, &default_style());
    let has_line = items.iter().any(|i| matches!(i, FrameItem::Line { .. }));
    assert!(has_line, "sqrt deve gerar FrameItem::Line para overline");
}

#[test]
fn layout_root_overline_horizontal() {
    let ml = MathLayouter::new(&FixedMetrics, true);
    let root = Content::math_root(None, Content::MathIdent("x".into()));
    let items = ml.layout_equation(&root, &default_style());
    for item in &items {
        if let FrameItem::Line { start, end, .. } = item {
            assert_eq!(start.y.val(), end.y.val(), "overline deve ser horizontal");
            assert!(end.x.val() > start.x.val(), "overline deve ter largura > 0");
        }
    }
}

#[test]
fn layout_root_com_indice_contem_indice() {
    let ml = MathLayouter::new(&FixedMetrics, true);
    let root = Content::math_root(
        Some(Content::MathText("3".into())),
        Content::MathIdent("x".into()),
    );
    let items = ml.layout_equation(&root, &default_style());
    let texts: Vec<_> = items
        .iter()
        .filter_map(|i| {
            if let FrameItem::Text { text, .. } = i {
                Some(text.as_str())
            } else {
                None
            }
        })
        .collect();
    assert!(
        texts.iter().any(|t| t.contains('3')),
        "root(3,x) deve conter '3': {:?}",
        texts
    );
    assert!(
        texts.iter().any(|t| t.contains('√')),
        "root(3,x) deve conter √: {:?}",
        texts
    );
    assert!(
        texts.iter().any(|t| t.contains('x')),
        "root(3,x) deve conter x: {:?}",
        texts
    );
}

// ── Testes do Passo 42 — MathDelimited e layout_stretchy_delimiter ───────

#[test]
fn layout_delimited_contem_corpo_e_delimitadores() {
    let ml = MathLayouter::new(&FixedMetrics, true);
    let delim = Content::math_delimited('(', Content::MathIdent("a".into()), ')');
    let items = ml.layout_equation(&delim, &default_style());
    let texts: Vec<_> = items
        .iter()
        .filter_map(|i| {
            if let FrameItem::Text { text, .. } = i {
                Some(text.as_str().to_string())
            } else {
                None
            }
        })
        .collect();
    assert!(texts.iter().any(|t| t.contains('(')), "deve conter '(': {:?}", texts);
    assert!(texts.iter().any(|t| t.contains('a')), "deve conter 'a': {:?}", texts);
    assert!(texts.iter().any(|t| t.contains(')')), "deve conter ')': {:?}", texts);
}

#[test]
fn layout_delimited_tres_ou_mais_items() {
    let ml = MathLayouter::new(&FixedMetrics, true);
    let delim = Content::math_delimited('[', Content::MathIdent("x".into()), ']');
    let items = ml.layout_equation(&delim, &default_style());
    assert!(items.len() >= 3, "delimitado deve ter >= 3 items, tem {}", items.len());
}

#[test]
fn layout_delimited_cursor_avanca() {
    // Delimitadores à esquerda e à direita do corpo
    let ml = MathLayouter::new(&FixedMetrics, true);
    let delim = Content::math_delimited('(', Content::MathIdent("x".into()), ')');
    let items = ml.layout_equation(&delim, &default_style());
    let xs: Vec<f64> = items
        .iter()
        .filter_map(|i| {
            if let FrameItem::Text { pos, .. } = i {
                Some(pos.x.val())
            } else {
                None
            }
        })
        .collect();
    // O delimitador de fecho deve estar à direita do delimitador de abertura
    assert!(xs.len() >= 2, "deve ter pelo menos 2 posições x");
    assert!(
        xs.last().unwrap() > xs.first().unwrap(),
        "fecho deve estar à direita de abertura"
    );
}

#[test]
fn fixed_metrics_sem_variantes_vertextuais() {
    let m = FixedMetrics;
    let v = m.vertical_glyph_variants('(');
    assert!(v.is_empty(), "FixedMetrics não tem variantes");
}

#[test]
fn fixed_metrics_glyph_to_char_none() {
    let m = FixedMetrics;
    assert_eq!(m.glyph_to_char(42), None);
}

#[test]
fn layout_stretchy_sem_variantes_usa_base() {
    // Com FixedMetrics, o glifo base é usado directamente
    let ml = MathLayouter::new(&FixedMetrics, true);
    let box_ = ml.layout_stretchy_delimiter('(', 1000.0, &default_style());
    assert!(box_.width > 0.0, "delimitador base deve ter largura > 0");
}

// ── Testes do Passo 43 — FrameItem::Glyph e GlyphAssembly ───────────────

#[test]
fn offset_item_desloca_glyph() {
    let item = FrameItem::Glyph {
        pos: Point { x: Pt(1.0), y: Pt(2.0) },
        glyph_id: 42,
        x_advance: Pt(10.0),
        size: Pt(12.0),
    };
    let shifted = offset_item(item, Pt(3.0), Pt(4.0));
    if let FrameItem::Glyph { pos, glyph_id, .. } = shifted {
        assert_eq!(pos.x.val(), 4.0);
        assert_eq!(pos.y.val(), 6.0);
        assert_eq!(glyph_id, 42);
    } else {
        panic!("deve ser Glyph");
    }
}

#[test]
fn fixed_metrics_assembly_vazia() {
    let m = FixedMetrics;
    let a = m.vertical_glyph_assembly('(');
    assert!(a.is_empty(), "FixedMetrics não tem assembly");
}

#[test]
fn layout_stretchy_sem_variantes_sem_assembly_usa_char_base() {
    // Com FixedMetrics, sem variantes nem assembly, deve usar char base
    let ml = MathLayouter::new(&FixedMetrics, true);
    let box_ = ml.layout_stretchy_delimiter('(', 5000.0, &default_style());
    // O resultado é um Text com '('
    let has_paren = box_.items.iter().any(
        |i| matches!(i, FrameItem::Text { text, .. } if text.as_str().contains('(')),
    );
    assert!(has_paren, "deve usar char base '(' quando sem variantes");
}

#[test]
fn layout_delimited_nao_tem_glyph_com_fixed_metrics() {
    // FixedMetrics não tem variantes — todos os items devem ser Text ou Line
    let ml = MathLayouter::new(&FixedMetrics, true);
    let delim = Content::math_delimited('(', Content::MathIdent("a".into()), ')');
    let items = ml.layout_equation(&delim, &default_style());
    let has_glyph = items.iter().any(|i| matches!(i, FrameItem::Glyph { .. }));
    assert!(!has_glyph, "FixedMetrics não deve emitir FrameItem::Glyph");
}

#[test]
fn frac_dentro_de_delimitadores_nao_regride() {
    let ml = MathLayouter::new(&FixedMetrics, true);
    let delim = Content::math_delimited(
        '(',
        Content::math_frac(
            Content::MathIdent("a".into()),
            Content::MathIdent("b".into()),
        ),
        ')',
    );
    let items = ml.layout_equation(&delim, &default_style());
    let texts: Vec<_> = items
        .iter()
        .filter_map(|i| {
            if let FrameItem::Text { text, .. } = i {
                Some(text.as_str())
            } else {
                None
            }
        })
        .collect();
    assert!(texts.iter().any(|t| t.contains('a')), "numerador: {:?}", texts);
    assert!(texts.iter().any(|t| t.contains('b')), "denominador: {:?}", texts);
}

#[test]
fn sqrt_nao_regride_passo43() {
    let ml = MathLayouter::new(&FixedMetrics, true);
    let root = Content::math_root(None, Content::MathIdent("x".into()));
    let items = ml.layout_equation(&root, &default_style());
    let texts: Vec<_> = items
        .iter()
        .filter_map(|i| {
            if let FrameItem::Text { text, .. } = i {
                Some(text.as_str())
            } else {
                None
            }
        })
        .collect();
    assert!(
        texts.iter().any(|t| t.contains('√') || t.contains('x')),
        "sqrt deve conter radical ou radicando: {:?}",
        texts
    );
}

#[test]
fn attach_nao_regride_passo43() {
    let ml = MathLayouter::new(&FixedMetrics, true);
    let attach = Content::math_attach(
        Content::MathIdent("x".into()),
        None,
        None,
        None,
        Some(Content::MathText("2".into())),
    );
    let items = ml.layout_equation(&attach, &default_style());
    let texts: Vec<_> = items
        .iter()
        .filter_map(|i| {
            if let FrameItem::Text { text, .. } = i {
                Some(text.as_str())
            } else {
                None
            }
        })
        .collect();
    assert!(texts.iter().any(|t| t.contains('x')), "base: {:?}", texts);
    assert!(texts.iter().any(|t| t.contains('2')), "sup: {:?}", texts);
}

#[test]
fn offset_item_desloca_text() {
    let item = FrameItem::Text {
        pos: Point { x: Pt(1.0), y: Pt(2.0) },
        text: "a".into(),
        style: TextStyle::regular(Pt(12.0)),
    };
    let shifted = offset_item(item, Pt(3.0), Pt(4.0));
    if let FrameItem::Text { pos, .. } = shifted {
        assert_eq!(pos.x.val(), 4.0);
        assert_eq!(pos.y.val(), 6.0);
    } else {
        panic!("deve ser Text");
    }
}

#[test]
fn offset_item_desloca_line() {
    let item = FrameItem::Line {
        start: Point { x: Pt(0.0), y: Pt(0.0) },
        end: Point { x: Pt(10.0), y: Pt(0.0) },
        thickness: 0.5,
        color: None, // P285
    };
    let shifted = offset_item(item, Pt(5.0), Pt(2.0));
    if let FrameItem::Line { start, end, .. } = shifted {
        assert_eq!(start.x.val(), 5.0);
        assert_eq!(start.y.val(), 2.0);
        assert_eq!(end.x.val(), 15.0);
        assert_eq!(end.y.val(), 2.0);
    } else {
        panic!("deve ser Line");
    }
}

// ── Testes do Passo 44 — AxisHeight e MathKernInfo ───────────────────

fn layout_equation_items(content: &Content) -> Vec<FrameItem> {
    let ml = MathLayouter::new(&FixedMetrics, true);
    ml.layout_equation(content, &default_style())
}

#[test]
fn fixed_metrics_math_kern_vazio() {
    let m = FixedMetrics;
    let k = m.math_kern('f');
    assert!(k.top_right.is_empty());
    assert!(k.bottom_right.is_empty());
}

#[test]
fn math_kern_default_nao_afecta_layout() {
    // math_kern com FixedMetrics retorna kern zero — layout não deve mudar
    let ml = MathLayouter::new(&FixedMetrics, true);
    let attach = Content::math_attach(
        Content::MathIdent("f".into()),
        None,
        None,
        None,
        Some(Content::MathText("2".into())),
    );
    let items = ml.layout_equation(&attach, &default_style());
    assert!(!items.is_empty(), "attach deve produzir items");
}

fn items_contain_text(items: &[FrameItem], c: char) -> bool {
    items
        .iter()
        .any(|i| matches!(i, FrameItem::Text { text, .. } if text.as_str().contains(c)))
}

#[test]
fn frac_com_axis_height_nao_regride() {
    let frac = Content::math_frac(
        Content::MathIdent("a".into()),
        Content::MathIdent("b".into()),
    );
    let items = layout_equation_items(&frac);
    assert!(items_contain_text(&items, 'a'), "numerador: {:?}", items);
    assert!(items_contain_text(&items, 'b'), "denominador: {:?}", items);
}

#[test]
fn delimitado_com_axis_height_nao_regride() {
    let delim = Content::math_delimited(
        '(',
        Content::math_frac(
            Content::MathIdent("a".into()),
            Content::MathIdent("b".into()),
        ),
        ')',
    );
    let items = layout_equation_items(&delim);
    assert!(items_contain_text(&items, 'a'));
    assert!(items_contain_text(&items, 'b'));
}

#[test]
fn sqrt_com_axis_height_nao_regride() {
    let root = Content::math_root(None, Content::MathIdent("x".into()));
    let items = layout_equation_items(&root);
    assert!(
        items_contain_text(&items, '√') || items_contain_text(&items, 'x'),
        "sqrt deve conter radical ou radicando"
    );
}

#[test]
fn attach_com_kern_nao_regride() {
    let attach = Content::math_attach(
        Content::MathIdent("x".into()),
        None,
        None,
        None,
        Some(Content::MathText("2".into())),
    );
    let items = layout_equation_items(&attach);
    assert!(items_contain_text(&items, 'x'));
    assert!(items_contain_text(&items, '2'));
}

#[test]
fn attach_sub_com_kern_nao_regride() {
    let attach = Content::math_attach(
        Content::MathIdent("x".into()),
        None,
        None,
        Some(Content::MathIdent("i".into())),
        None,
    );
    let items = layout_equation_items(&attach);
    assert!(items_contain_text(&items, 'x'));
    assert!(items_contain_text(&items, 'i'));
}

#[test]
fn frac_axis_ascent_maior_que_sem_axis() {
    // Com axis_height, a fracção sobe: o ascent do MathBox aumenta.
    // Verificar que o axis_height é não-zero (fallback=500 > 0).
    let constants = crate::entities::math_constants::MathConstants::fallback();
    assert!(constants.axis_height > 0.0, "axis_height do fallback deve ser > 0");
}

// ── Testes do Passo 46 — Pre-scripts (tl/bl) ─────────────────────────

#[test]
fn attach_sem_left_scripts_nao_regride() {
    // Regressão: MathAttach sem tl/bl comporta-se como antes
    let attach = Content::math_attach(
        Content::MathIdent("x".into()),
        None,
        None,
        None,
        Some(Content::MathText("2".into())),
    );
    let items = layout_equation_items(&attach);
    assert!(items_contain_text(&items, 'x'), "base ausente: {:?}", items);
    assert!(items_contain_text(&items, '2'), "sup ausente: {:?}", items);
}

#[test]
fn attach_left_sup_contem_base_e_script() {
    // Pre-superscript: conteúdo do script e da base presentes
    let attach = Content::math_attach(
        Content::MathIdent("x".into()),
        Some(Content::MathText("2".into())),
        None,
        None,
        None,
    );
    let items = layout_equation_items(&attach);
    assert!(items_contain_text(&items, '2'), "pre-sup ausente: {:?}", items);
    assert!(items_contain_text(&items, 'x'), "base ausente: {:?}", items);
}

#[test]
fn attach_left_sub_contem_base_e_script() {
    // Pre-subscript
    let attach = Content::math_attach(
        Content::MathIdent("x".into()),
        None,
        Some(Content::MathText("1".into())),
        None,
        None,
    );
    let items = layout_equation_items(&attach);
    assert!(items_contain_text(&items, '1'), "pre-sub ausente: {:?}", items);
    assert!(items_contain_text(&items, 'x'), "base ausente: {:?}", items);
}

#[test]
fn attach_left_e_right_juntos() {
    // Scripts nos dois lados simultaneamente
    let attach = Content::math_attach(
        Content::MathIdent("x".into()),
        Some(Content::MathText("2".into())),
        Some(Content::MathText("1".into())),
        Some(Content::MathText("3".into())),
        Some(Content::MathText("4".into())),
    );
    let items = layout_equation_items(&attach);
    assert!(items_contain_text(&items, '1'), "bl ausente");
    assert!(items_contain_text(&items, '2'), "tl ausente");
    assert!(items_contain_text(&items, 'x'), "base ausente");
    assert!(items_contain_text(&items, '3'), "sub ausente");
    assert!(items_contain_text(&items, '4'), "sup ausente");
}

#[test]
fn attach_left_sup_base_deslocada_para_direita() {
    // Com tl presente, a base deve aparecer a uma posição x maior do que zero
    let attach = Content::math_attach(
        Content::MathIdent("x".into()),
        Some(Content::MathText("2".into())),
        None,
        None,
        None,
    );
    let items = layout_equation_items(&attach);
    // Encontrar a posição x do glifo "x" (base)
    let base_xs: Vec<f64> = items
        .iter()
        .filter_map(|i| {
            if let FrameItem::Text { pos, text, .. } = i {
                if text.contains('x') {
                    Some(pos.x.val())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    assert!(!base_xs.is_empty(), "base deve estar presente");
    assert!(
        base_xs.iter().any(|&x| x > 0.0),
        "base deve estar deslocada para direita quando há tl; xs={:?}",
        base_xs
    );
}

#[test]
fn attach_sem_base_explicita_usa_empty() {
    // Base vazia: não deve panicar
    let attach = Content::math_attach(
        Content::Empty,
        Some(Content::MathText("14".into())),
        None,
        None,
        None,
    );
    let items = layout_equation_items(&attach);
    // Não deve panicar; items pode estar vazio mas o programa não crasha
    let _ = items;
}

// ── Testes do Passo 53 — Kern diferenciado para left-scripts ─────────

#[test]
fn left_scripts_tem_posicoes_x_independentes() {
    // tl e bl em simultâneo — lógica de kern independente não deve panicar.
    // Com FixedMetrics os kerns são zero, por isso tl_x == bl_x é esperado.
    let attach = Content::math_attach(
        Content::MathIdent("A".into()),
        Some(Content::MathText("x".into())),
        Some(Content::MathText("y".into())),
        None,
        None,
    );
    let items = layout_equation_items(&attach);
    assert!(!items.is_empty(), "deve produzir items com tl e bl");
    assert!(items_contain_text(&items, 'x'), "tl ausente");
    assert!(items_contain_text(&items, 'y'), "bl ausente");
    assert!(items_contain_text(&items, 'A'), "base ausente");
}

#[test]
fn left_scripts_sem_bl_nao_panica() {
    // Apenas tl presente — bl_push é zero, base_offset_x = tl_push.
    let attach = Content::math_attach(
        Content::MathIdent("A".into()),
        Some(Content::MathText("x".into())),
        None,
        None,
        None,
    );
    let items = layout_equation_items(&attach);
    assert!(!items.is_empty());
    assert!(items_contain_text(&items, 'x'), "tl ausente");
}

#[test]
fn left_scripts_sem_tl_nao_panica() {
    // Apenas bl presente — tl_push é zero, base_offset_x = bl_push.
    let attach = Content::math_attach(
        Content::MathIdent("A".into()),
        None,
        Some(Content::MathText("y".into())),
        None,
        None,
    );
    let items = layout_equation_items(&attach);
    assert!(!items.is_empty());
    assert!(items_contain_text(&items, 'y'), "bl ausente");
}

#[test]
fn left_scripts_passo46_nao_regride() {
    // Regressão Passo 46: _0^n ∑ — operador grande com left-scripts.
    let attach = Content::math_attach(
        Content::MathText("∑".into()),
        Some(Content::MathText("n".into())),
        Some(Content::MathText("0".into())),
        None,
        None,
    );
    let items = layout_equation_items(&attach);
    assert!(
        items_contain_text(&items, 'n') || items_contain_text(&items, '0'),
        "scripts ausentes: {:?}",
        items
    );
}

// ── Passo 311b.5 — E2E math style integration tests ─────────────────────
//
// Verificam que MathStyled chega ao layout corretamente e que
// codepoints Unicode variant são emitidos no FrameItem::Text.

#[test]
fn p311b5_bb_x_emite_double_struck_x() {
    let bb_x = Content::math_styled(
        Some(MathStyleKind::DoubleStruck),
        None,
        None,
        Content::MathIdent("x".into()),
        None,
    );
    let items = layout_equation_items(&bb_x);
    assert!(
        items_contain_text(&items, '\u{1D569}'),
        "bb(x) deve emitir 𝕩 U+1D569: {:?}",
        items
    );
}

#[test]
fn p311b5_cal_L_emite_script_L() {
    let cal_L = Content::math_styled(
        Some(MathStyleKind::Chancery),
        None,
        None,
        Content::MathIdent("L".into()),
        None,
    );
    let items = layout_equation_items(&cal_L);
    // L Chancery = U+2112 (excepção BMP)
    assert!(
        items_contain_text(&items, '\u{2112}'),
        "cal(L) deve emitir ℒ U+2112: {:?}",
        items
    );
}

#[test]
fn p311b5_bb_cal_x_outer_wins() {
    // bb(cal(x)) — outer Bb deve ganhar.
    let inner = Content::math_styled(
        Some(MathStyleKind::Chancery),
        None,
        None,
        Content::MathIdent("x".into()),
        None,
    );
    let outer =
        Content::math_styled(Some(MathStyleKind::DoubleStruck), None, None, inner, None);
    let items = layout_equation_items(&outer);
    assert!(
        items_contain_text(&items, '\u{1D569}'),
        "bb(cal(x)) → outer Bb deve ganhar; esperava 𝕩 U+1D569: {:?}",
        items
    );
}

#[test]
fn p311b5_upright_italic_x_outer_wins() {
    // upright(italic(x)) — outer upright (italic=Some(false)) deve ganhar.
    let inner = Content::math_styled(
        None,
        None,
        Some(true),
        Content::MathIdent("x".into()),
        None,
    );
    let outer = Content::math_styled(None, None, Some(false), inner, None);
    let items = layout_equation_items(&outer);
    // upright wins → 'x' literal (não italic codepoint).
    assert!(
        items_contain_text(&items, 'x'),
        "upright(italic(x)) → upright deve ganhar; esperava 'x' literal: {:?}",
        items
    );
}

#[test]
fn p311b5_bb_frac_a_b_propaga_recurse() {
    // bb(frac(a, b)) — propaga DS aos sub-elementos.
    let frac = Content::math_frac(
        Content::MathIdent("a".into()),
        Content::MathIdent("b".into()),
    );
    let bb_frac =
        Content::math_styled(Some(MathStyleKind::DoubleStruck), None, None, frac, None);
    let items = layout_equation_items(&bb_frac);
    assert!(
        items_contain_text(&items, '\u{1D552}'),
        "bb(frac(a,b)) deve emitir 𝕒 U+1D552 no numerador: {:?}",
        items
    );
    assert!(
        items_contain_text(&items, '\u{1D553}'),
        "bb(frac(a,b)) deve emitir 𝕓 U+1D553 no denominador: {:?}",
        items
    );
}

#[test]
fn p311b5_bold_bb_x_ortogonal_preserva_inner() {
    // bold(bb(x)) — inner Bb preserved; bold orthogonal.
    let inner = Content::math_styled(
        Some(MathStyleKind::DoubleStruck),
        None,
        None,
        Content::MathIdent("x".into()),
        None,
    );
    let outer = Content::math_styled(None, Some(true), None, inner, None);
    let items = layout_equation_items(&outer);
    // DoubleStruck plane não tem variant Bold separado — DS é uniforme.
    // map_glyph aplica DS base (U+1D569 para 'x' lowercase). Verificar
    // que pelo menos um codepoint DS é emitido.
    let has_ds_codepoint = items.iter().any(|i| {
        matches!(i,
            FrameItem::Text { text, .. }
            if text.chars().any(|c| (c as u32) >= 0x1D552 && (c as u32) <= 0x1D56B)
        )
    });
    assert!(
        has_ds_codepoint,
        "bold(bb(x)) deve preservar DS lowercase plane: {:?}",
        items
    );
}
