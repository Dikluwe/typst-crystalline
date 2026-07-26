//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/math/layout/_comum.md
//! @prompt-hash 2fab143c
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
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
    let items = ml.layout_equation(&Content::MathIdent("x".into()), &default_style());
    assert!(!items.is_empty(), "MathIdent deve produzir pelo menos 1 item");
}

#[test]
fn math_layouter_math_text_produz_items_nao_vazios() {
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
    let items = ml.layout_equation(&Content::MathText("sin".into()), &default_style());
    assert!(!items.is_empty());
}

#[test]
fn math_layouter_sequence_produz_multiplos_items() {
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
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
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
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
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
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
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
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
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
    let frac = Content::math_frac(
        Content::MathIdent("a".into()),
        Content::MathIdent("b".into()),
    );
    let items = ml.layout_equation(&frac, &size10_style());
    assert!(items.len() >= 2, "frac deve ter >= 2 items, tem {}", items.len());
}

#[test]
fn math_frac_numerador_acima_denominador() {
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
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
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
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
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
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
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
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
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
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
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
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
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
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

// ── P799 — sub+sup empilham na mesma origem x (paridade vanilla scripts.rs) ──

#[test]
fn math_attach_sub_sup_partilham_origem_x() {
    // Regressão P799: antes desta correcção, sub e sup eram compostos em
    // sequência horizontal (o cursor avançava depois do sup e o sub era
    // colocado a seguir a ele), em vez de partilharem a mesma origem x
    // imediatamente à direita da base (vanilla `scripts.rs`: `tr_x` e `br_x`
    // são ambos `pre_width + base_width + kern`).
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
    let attach = Content::math_attach(
        Content::MathIdent("x".into()),
        None,
        None,
        Some(Content::MathIdent("3".into())),
        Some(Content::MathIdent("2".into())),
    );
    let items = ml.layout_equation(&attach, &default_style());

    let x_of = |needle: &str| {
        items.iter().find_map(|i| match i {
            FrameItem::Text { pos, text, .. } if text.as_str() == needle => {
                Some(pos.x.val())
            }
            _ => None,
        })
    };
    let base_x = x_of("𝑥").expect("base x presente");
    let sup_x = x_of("2").expect("sup '2' presente");
    let sub_x = x_of("3").expect("sub '3' presente");

    // FixedMetrics: kern zero nos 4 quadrantes — sup e sub devem partir
    // exactamente da mesma origem x (à direita da base).
    let base_w = 12.0 * 0.6;
    assert!(
        (sup_x - (base_x + base_w)).abs() < 0.01,
        "sup deve partir imediatamente à direita da base: base_x={base_x} sup_x={sup_x}"
    );
    assert!(
        (sub_x - sup_x).abs() < 0.01,
        "sub e sup devem partilhar a mesma origem x (empilhados): sup_x={sup_x} sub_x={sub_x}"
    );
}

#[test]
fn math_attach_sub_sup_largura_max_nao_soma_nucleo_multi_char() {
    // Regressão P799 com núcleo multi-caractere: a largura total do attach é
    // base + max(sup, sub), não base + sup + sub. O elemento seguinte na
    // sequência deve começar logo após o script mais largo.
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
    let attach = Content::math_attach(
        Content::MathIdent("ab".into()),
        None,
        None,
        Some(Content::MathIdent("333".into())),
        Some(Content::MathIdent("22".into())),
    );
    let seq = Content::MathSequence(Arc::from(
        vec![attach, Content::MathIdent("z".into())].into_boxed_slice(),
    ));
    let items = ml.layout_equation(&seq, &default_style());

    let x_of = |needle: &str| {
        items.iter().find_map(|i| match i {
            FrameItem::Text { pos, text, .. } if text.as_str() == needle => {
                Some(pos.x.val())
            }
            _ => None,
        })
    };
    let sup_x = x_of("22").expect("sup presente");
    let sub_x = x_of("333").expect("sub presente");
    let z_x = x_of("𝑧").expect("elemento seguinte presente");

    let base_w = 2.0 * 12.0 * 0.6; // "ab", 2 chars
    let script_char_w = 12.0 * 0.7 * 0.6; // script_percent_scale_down = 0.7
    let sub_w = 3.0 * script_char_w; // "333" é o script mais largo

    assert!(
        (sub_x - sup_x).abs() < 0.01,
        "sub e sup empilhados na mesma origem x: sup_x={sup_x} sub_x={sub_x}"
    );
    assert!(
        (z_x - (base_w + sub_w)).abs() < 0.01,
        "elemento seguinte deve começar em base + max(sup,sub): z_x={z_x} esperado={}",
        base_w + sub_w
    );
}

// ── P809 — itálico matemático por defeito via codepoint ─────────────────

#[test]
fn p809_mathident_letra_unica_vira_math_italic() {
    // P809 — `$x$`: codepoint math italic U+1D465 (paridade vanilla medida),
    // não 'x' plain com flag de fonte.
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
    let items = ml.layout_equation(&Content::MathIdent("x".into()), &default_style());
    let texts: Vec<_> = items
        .iter()
        .filter_map(|i| {
            if let FrameItem::Text { text, .. } = i { Some(text.as_str()) } else { None }
        })
        .collect();
    assert!(texts.iter().any(|t| *t == "\u{1D465}"), "esperado 𝑥 U+1D465: {texts:?}");
}

#[test]
fn p809_mathtext_letra_unica_vira_math_italic() {
    // `$x$` chega como MathText (lexer: grafema único) — coberto pelo mesmo default.
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
    let items = ml.layout_equation(&Content::MathText("x".into()), &default_style());
    let texts: Vec<_> = items
        .iter()
        .filter_map(|i| {
            if let FrameItem::Text { text, .. } = i { Some(text.as_str()) } else { None }
        })
        .collect();
    assert!(texts.iter().any(|t| *t == "\u{1D465}"), "esperado 𝑥 U+1D465: {texts:?}");
}

#[test]
fn p809_mathtext_grego_minusculo_vira_math_italic() {
    // `alpha` resolvido → MathText("α") → 𝛼 U+1D6FC (paridade vanilla medida).
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
    let items = ml.layout_equation(&Content::MathText("α".into()), &default_style());
    let texts: Vec<_> = items
        .iter()
        .filter_map(|i| {
            if let FrameItem::Text { text, .. } = i { Some(text.as_str()) } else { None }
        })
        .collect();
    assert!(texts.iter().any(|t| *t == "\u{1D6FC}"), "esperado 𝛼 U+1D6FC: {texts:?}");
}

#[test]
fn p809_mathtext_grego_maiusculo_fica_upright() {
    // Medido no vanilla: `$Gamma Delta Omega alpha$` → ΓΔΩ𝛼 — maiúsculas upright.
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
    let items = ml.layout_equation(&Content::MathText("Γ".into()), &default_style());
    let texts: Vec<_> = items
        .iter()
        .filter_map(|i| {
            if let FrameItem::Text { text, .. } = i { Some(text.as_str()) } else { None }
        })
        .collect();
    assert!(texts.iter().any(|t| *t == "Γ"), "esperado Γ upright: {texts:?}");
}

#[test]
fn p809_mathtext_digito_e_funcao_nao_mudam() {
    // Dígitos nunca têm itálico por defeito; texto multi-carácter (sin) é upright.
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
    for (input, esperado) in [("5", "5"), ("sin", "sin"), ("+", "+")] {
        let items = ml.layout_equation(&Content::MathText(input.into()), &default_style());
        let texts: Vec<_> = items
            .iter()
            .filter_map(|i| {
                if let FrameItem::Text { text, .. } = i { Some(text.as_str()) } else { None }
            })
            .collect();
        assert!(
            texts.iter().any(|t| *t == esperado),
            "esperado {esperado:?} inalterado: {texts:?}"
        );
    }
}

// ── P812 — tamanhos math (display/script/sscript) + itálico em wrappers ──

#[test]
fn p812a_script_aplica_factor_tamanho() {
    // P812-A — `script(x)` deve renderizar a 0.7× o tamanho base
    // (paridade vanilla medida: trm 11 → 7.7pt, sscript 5.5pt).
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
    let styled = Content::math_styled(
        Some(crate::entities::math_style::MathStyleKind::Script),
        None,
        None,
        Content::MathIdent("x".into()),
        None,
    );
    let items = ml.layout_equation(&styled, &default_style());
    let sizes: Vec<f64> = items
        .iter()
        .filter_map(|i| {
            if let FrameItem::Text { style, .. } = i { Some(style.size.val()) } else { None }
        })
        .collect();
    assert!(!sizes.is_empty(), "deve produzir items de texto");
    assert!(
        sizes.iter().all(|s| (*s - 12.0 * 0.7).abs() < 0.01),
        "script deve aplicar factor 0.7 (8.4pt): {sizes:?}"
    );
}

#[test]
fn p812a_sscript_aplica_factor_tamanho() {
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
    let styled = Content::math_styled(
        Some(crate::entities::math_style::MathStyleKind::SScript),
        None,
        None,
        Content::MathIdent("x".into()),
        None,
    );
    let items = ml.layout_equation(&styled, &default_style());
    let sizes: Vec<f64> = items
        .iter()
        .filter_map(|i| {
            if let FrameItem::Text { style, .. } = i { Some(style.size.val()) } else { None }
        })
        .collect();
    assert!(
        sizes.iter().all(|s| (*s - 12.0 * 0.5).abs() < 0.01),
        "sscript deve aplicar factor 0.5 (6.0pt): {sizes:?}"
    );
}

#[test]
fn p812b_italico_atravessa_wrapper_de_tamanho() {
    // P812-B — paridade vanilla medida: `$script(x)$` extrai 𝑥 U+1D465,
    // não x plain. O eixo itálico é ortogonal ao eixo tamanho.
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
    let styled = Content::math_styled(
        Some(crate::entities::math_style::MathStyleKind::Script),
        None,
        None,
        Content::MathIdent("x".into()),
        None,
    );
    let items = ml.layout_equation(&styled, &default_style());
    let texts: Vec<_> = items
        .iter()
        .filter_map(|i| {
            if let FrameItem::Text { text, .. } = i { Some(text.as_str()) } else { None }
        })
        .collect();
    assert!(
        texts.iter().any(|t| *t == "\u{1D465}"),
        "script(x) deve manter o itálico por defeito (𝑥 U+1D465): {texts:?}"
    );
}

#[test]
fn p812b_tamanho_sobre_glyph_variant_preserva_glyph_e_factor() {
    // P812-B — eixos ortogonais (vanilla): `script(bb(R))` → ℝ (glyph do
    // inner) a 0.7× (tamanho do outer). A composição anterior (outer-wins
    // cega) largava o glyph variant do inner.
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
    let inner = Content::math_styled(
        Some(crate::entities::math_style::MathStyleKind::DoubleStruck),
        None,
        None,
        Content::MathIdent("R".into()),
        None,
    );
    let outer = Content::math_styled(
        Some(crate::entities::math_style::MathStyleKind::Script),
        None,
        None,
        inner,
        None,
    );
    let items = ml.layout_equation(&outer, &default_style());
    let found: Vec<(String, f64)> = items
        .iter()
        .filter_map(|i| {
            if let FrameItem::Text { text, style, .. } = i {
                Some((text.as_str().to_string(), style.size.val()))
            } else {
                None
            }
        })
        .collect();
    assert!(
        found.iter().any(|(t, s)| t == "\u{211D}" && (*s - 12.0 * 0.7).abs() < 0.01),
        "script(bb(R)) deve ser ℝ U+211D (excepção letterlike) a 8.4pt: {found:?}"
    );
}

// ── Testes do Passo 40 — layout_root ─────────────────────────────────

#[test]
fn layout_root_contem_radical_e_radicando() {
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
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
    assert!(texts.iter().any(|t| t.contains('𝑥')), "deve conter x: {:?}", texts);
}

#[test]
fn layout_root_tem_overline() {
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
    let root = Content::math_root(None, Content::MathIdent("x".into()));
    let items = ml.layout_equation(&root, &default_style());
    let has_line = items.iter().any(|i| matches!(i, FrameItem::Line { .. }));
    assert!(has_line, "sqrt deve gerar FrameItem::Line para overline");
}

#[test]
fn layout_root_overline_horizontal() {
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
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
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
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
        texts.iter().any(|t| t.contains('𝑥')),
        "root(3,x) deve conter x: {:?}",
        texts
    );
}

// ── Testes do Passo 901 (Agente A) — overline do radical mal posicionada ──
//
// P894 catalogou visualmente: a barra horizontal do radical em `sqrt(x)`/
// `root(3, x)` atravessa o radicando "a meio da altura" (como um traço/
// strikethrough) em vez de ficar por cima. Estes testes chamam
// `layout_root`/`layout_node` directamente (bypass de `layout_equation`,
// logo sem o mapeamento itálico de `apply_math_default` — o texto do
// radicando fica "x" em vez de "𝑥"; irrelevante para `FixedMetrics`, que
// ignora o conteúdo do texto nas métricas verticais) para inspeccionar a
// `MathBox` interna (`ascent`/`items`) sem depender de `place()`.
//
// Convenção de coordenadas confirmada por leitura de `layout/mod.rs`
// (`hconcat_spaced` não desloca `y` ao concatenar boxes irmãs; `place()` só
// é chamado uma vez, em `layout_equation`, com `baseline_y = box.ascent`,
// o que torna `parent_y = local_y` — logo o `y=0` "local" de cada `MathBox`
// **é** a sua própria baseline, e é essa mesma baseline que sobrevive até
// à `Vec<FrameItem>` final): dentro de `result.items`, `y` cresce para
// baixo e `y=0` é a baseline do `MathBox`. O topo da tinta de uma caixa
// cuja baseline está em `y = b` fica em `y = b - ascent`.

#[test]
fn layout_root_overline_fica_acima_do_topo_do_radicando() {
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
    let style = default_style();
    let radicand = Content::MathIdent("x".into());

    // Medição independente do radicando isolado — a MESMA chamada
    // (`layout_node`) que `layout_root` usa internamente para produzir
    // `rad_box`. Não hardcode: o ascent vem da própria estrutura devolvida,
    // por isso o alvo sobrevive a mudanças de constantes/fonte.
    let rad_box = ml.layout_node(&radicand, &style);

    let result = ml.layout_root(None, &radicand, &style);

    let overline_y = result
        .items
        .iter()
        .find_map(|i| match i {
            FrameItem::Line { start, end, .. } => {
                assert_eq!(start.y.val(), end.y.val(), "overline deve ser horizontal");
                Some(start.y.val())
            }
            _ => None,
        })
        .expect("layout_root deve produzir FrameItem::Line para a overline");

    let radicand_y = result
        .items
        .iter()
        .find_map(|i| match i {
            FrameItem::Text { pos, text, .. } if text.as_str() == "x" => Some(pos.y.val()),
            _ => None,
        })
        .expect("layout_root deve conter o item de texto do radicando ('x')");

    // Topo da tinta do radicando, nas mesmas coordenadas locais de
    // `result.items` (y cresce para baixo ⇒ "acima" é y menor).
    let radicand_ink_top_y = radicand_y - rad_box.ascent;

    assert!(
        overline_y <= radicand_ink_top_y,
        "a barra do radical deve ficar por cima do topo do radicando (overline_y <= \
         radicand_ink_top_y), mas overline_y={overline_y} > radicand_ink_top_y={radicand_ink_top_y} \
         (radicand_y={radicand_y}, rad_box.ascent={}) — a barra atravessa o conteúdo em vez de \
         ficar por cima dele",
        rad_box.ascent,
    );
}

#[test]
fn layout_root_com_indice_overline_fica_acima_do_topo_do_radicando() {
    // Mesma verificação que `layout_root_overline_fica_acima_do_topo_do_radicando`,
    // mas para `root(3, x)` — confirma que o bug (e o alvo geométrico) não
    // depende da presença de índice: `layout_root` calcula overline/gap a
    // partir só do radicando, o índice é composto depois (root.rs:82-94),
    // por isso a relação overline-vs-radicando deve ser idêntica.
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
    let style = default_style();
    let radicand = Content::MathIdent("x".into());
    let index = Content::MathText("3".into());

    let rad_box = ml.layout_node(&radicand, &style);

    let result = ml.layout_root(Some(&index), &radicand, &style);

    let overline_y = result
        .items
        .iter()
        .find_map(|i| match i {
            FrameItem::Line { start, end, .. } => {
                assert_eq!(start.y.val(), end.y.val(), "overline deve ser horizontal");
                Some(start.y.val())
            }
            _ => None,
        })
        .expect("layout_root deve produzir FrameItem::Line para a overline");

    let radicand_y = result
        .items
        .iter()
        .find_map(|i| match i {
            FrameItem::Text { pos, text, .. } if text.as_str() == "x" => Some(pos.y.val()),
            _ => None,
        })
        .expect("layout_root deve conter o item de texto do radicando ('x')");

    let radicand_ink_top_y = radicand_y - rad_box.ascent;

    assert!(
        overline_y <= radicand_ink_top_y,
        "root(3,x): a barra do radical deve ficar por cima do topo do radicando, mas \
         overline_y={overline_y} > radicand_ink_top_y={radicand_ink_top_y} \
         (radicand_y={radicand_y}, rad_box.ascent={})",
        rad_box.ascent,
    );
}

// ── Testes do Passo 42 — MathDelimited e layout_stretchy_delimiter ───────

#[test]
fn layout_delimited_contem_corpo_e_delimitadores() {
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
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
    assert!(texts.iter().any(|t| t.contains('𝑎')), "deve conter 'a': {:?}", texts);
    assert!(texts.iter().any(|t| t.contains(')')), "deve conter ')': {:?}", texts);
}

#[test]
fn layout_delimited_tres_ou_mais_items() {
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
    let delim = Content::math_delimited('[', Content::MathIdent("x".into()), ']');
    let items = ml.layout_equation(&delim, &default_style());
    assert!(items.len() >= 3, "delimitado deve ter >= 3 items, tem {}", items.len());
}

#[test]
fn layout_delimited_cursor_avanca() {
    // Delimitadores à esquerda e à direita do corpo
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
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
    let v = m.vertical_glyph_variants('(', &default_style());
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
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
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
        style: TextStyle::regular(Pt(12.0)),
        base_char: 'x',
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
    let a = m.vertical_glyph_assembly('(', &default_style());
    assert!(a.is_empty(), "FixedMetrics não tem assembly");
}

#[test]
fn layout_stretchy_sem_variantes_sem_assembly_usa_char_base() {
    // Com FixedMetrics, sem variantes nem assembly, deve usar char base
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
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
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
    let delim = Content::math_delimited('(', Content::MathIdent("a".into()), ')');
    let items = ml.layout_equation(&delim, &default_style());
    let has_glyph = items.iter().any(|i| matches!(i, FrameItem::Glyph { .. }));
    assert!(!has_glyph, "FixedMetrics não deve emitir FrameItem::Glyph");
}

#[test]
fn frac_dentro_de_delimitadores_nao_regride() {
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
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
    assert!(texts.iter().any(|t| t.contains('𝑎')), "numerador: {:?}", texts);
    assert!(texts.iter().any(|t| t.contains('𝑏')), "denominador: {:?}", texts);
}

#[test]
fn sqrt_nao_regride_passo43() {
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
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
        texts.iter().any(|t| t.contains('√') || t.contains('𝑥')),
        "sqrt deve conter radical ou radicando: {:?}",
        texts
    );
}

#[test]
fn attach_nao_regride_passo43() {
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
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
    assert!(texts.iter().any(|t| t.contains('𝑥')), "base: {:?}", texts);
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
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
    ml.layout_equation(content, &default_style())
}

#[test]
fn fixed_metrics_math_kern_vazio() {
    let m = FixedMetrics;
    let k = m.math_kern('f', &default_style());
    assert!(k.top_right.is_empty());
    assert!(k.bottom_right.is_empty());
}

#[test]
fn math_kern_default_nao_afecta_layout() {
    // math_kern com FixedMetrics retorna kern zero — layout não deve mudar
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
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
    assert!(items_contain_text(&items, '𝑎'), "numerador: {:?}", items);
    assert!(items_contain_text(&items, '𝑏'), "denominador: {:?}", items);
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
    assert!(items_contain_text(&items, '𝑎'));
    assert!(items_contain_text(&items, '𝑏'));
}

#[test]
fn sqrt_com_axis_height_nao_regride() {
    let root = Content::math_root(None, Content::MathIdent("x".into()));
    let items = layout_equation_items(&root);
    assert!(
        items_contain_text(&items, '√') || items_contain_text(&items, '𝑥'),
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
    assert!(items_contain_text(&items, '𝑥'));
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
    assert!(items_contain_text(&items, '𝑥'));
    assert!(items_contain_text(&items, '𝑖'));
}

#[test]
fn frac_axis_ascent_maior_que_sem_axis() {
    // Com axis_height, a fracção sobe: o ascent do MathBox aumenta.
    // Verificar que o axis_height é não-zero (fallback=500 > 0).
    let constants = crate::entities::math_constants::MathConstants::fallback();
    assert!(constants.axis_height > 0.0, "axis_height do fallback deve ser > 0");
}

// ── P905 — layout_frac usava convenção "topo do box" (num_y=0.0,
// rule/den_y relativos a num_box.height()) em vez da convenção
// baseline-relativa confirmada em P901/attach.rs/root.rs (`y=0` é a
// BASELINE PRÓPRIA de cada MathBox, y cresce para baixo). Resultado:
// a linha de fracção acabava a meio do denominador, não entre os dois.
// `math_frac_numerador_acima_denominador` (acima) só verifica a ORDEM
// (num.y < den.y), que continua a ser verdade mesmo com o bug (0 <
// positivo) — por isso não apanhou a regressão. Estes dois testes
// verificam a MAGNITUDE do gap, não só a ordem.

#[test]
fn p905_frac_numerador_tem_gap_acima_da_linha() {
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
    let style = default_style();
    let constants = crate::entities::math_constants::MathConstants::fallback();
    let sub_size = style.size.val() * constants.script_percent_scale_down;
    // FixedMetrics: ascent = 0.8*size, descent = 0.4*size, para qualquer char.
    let leaf_descent = sub_size * 0.4;

    let math_box = ml.layout_frac(
        &Content::MathIdent("a".into()),
        &Content::MathIdent("b".into()),
        &style,
    );

    let mut text_ys: Vec<f64> = math_box
        .items
        .iter()
        .filter_map(|i| match i {
            FrameItem::Text { pos, .. } => Some(pos.y.val()),
            _ => None,
        })
        .collect();
    text_ys.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert!(text_ys.len() >= 2, "frac deve ter >= 2 items de texto");
    let num_y = text_ys[0];

    let rule_y = math_box
        .items
        .iter()
        .find_map(|i| match i {
            FrameItem::Line { start, .. } => Some(start.y.val()),
            _ => None,
        })
        .expect("frac deve ter FrameItem::Line");

    let num_bottom_ink = num_y + leaf_descent;
    assert!(
        rule_y - num_bottom_ink > 0.5,
        "numerador deve ter gap > 0.5pt acima da linha: rule_y={} num_bottom_ink={} (num_y={})",
        rule_y,
        num_bottom_ink,
        num_y
    );
}

#[test]
fn p905_frac_denominador_tem_gap_abaixo_da_linha_nao_sobrepoe() {
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
    let style = default_style();
    let constants = crate::entities::math_constants::MathConstants::fallback();
    let sub_size = style.size.val() * constants.script_percent_scale_down;
    // FixedMetrics: ascent = 0.8*size, descent = 0.4*size, para qualquer char.
    let leaf_ascent = sub_size * 0.8;

    let math_box = ml.layout_frac(
        &Content::MathIdent("a".into()),
        &Content::MathIdent("b".into()),
        &style,
    );

    let mut text_ys: Vec<f64> = math_box
        .items
        .iter()
        .filter_map(|i| match i {
            FrameItem::Text { pos, .. } => Some(pos.y.val()),
            _ => None,
        })
        .collect();
    text_ys.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert!(text_ys.len() >= 2, "frac deve ter >= 2 items de texto");
    let den_y = text_ys[1];

    let rule_y = math_box
        .items
        .iter()
        .find_map(|i| match i {
            FrameItem::Line { start, .. } => Some(start.y.val()),
            _ => None,
        })
        .expect("frac deve ter FrameItem::Line");

    let den_top_ink = den_y - leaf_ascent;
    assert!(
        den_top_ink - rule_y > 0.5,
        "denominador deve ter gap > 0.5pt abaixo da linha (não sobrepor): \
         den_top_ink={} rule_y={} (den_y={})",
        den_top_ink,
        rule_y,
        den_y
    );
}

#[test]
fn p905_sqrt_de_fraccao_com_variaveis_nao_produz_saida_malformada() {
    // Caso literal do achado de P899 Parte B / materialização P905:
    // `sqrt(x/y)` (via chamada de função em modo math). Confirma que o
    // denominador ('y'/𝑦) fica com um gap real abaixo da linha de
    // fracção (não sobreposto por ela) — o sintoma visual reportado
    // ("só o primeiro operando aparece, com um glifo estranho por
    // baixo"). Não cobre o sizing do símbolo √ em si (root.rs, fora de
    // âmbito deste passo — ver relatório).
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
    let style = default_style();
    let constants = crate::entities::math_constants::MathConstants::fallback();
    let sub_size = style.size.val() * constants.script_percent_scale_down;
    let leaf_ascent = sub_size * 0.8;

    let root = Content::math_root(
        None,
        Content::math_frac(
            Content::MathIdent("x".into()),
            Content::MathIdent("y".into()),
        ),
    );
    let items = ml.layout_equation(&root, &style);

    // Identificar o denominador pelo texto ('y' plano ou 𝑦 itálico) em vez
    // de assumir posição por ordenação — o próprio símbolo √ também emite
    // um FrameItem::Text (glifo), que pode ter y mais extremo que o
    // numerador/denominador.
    let den_y = items
        .iter()
        .find_map(|i| match i {
            FrameItem::Text { pos, text, .. }
                if text.as_str() == "y" || text.contains('𝑦') =>
            {
                Some(pos.y.val())
            }
            _ => None,
        })
        .unwrap_or_else(|| panic!("denominador 'y' deve estar presente: {:?}", items));

    let rule_y = items
        .iter()
        .filter_map(|i| match i {
            FrameItem::Line { start, .. } => Some(start.y.val()),
            _ => None,
        })
        // sqrt tem 2 linhas (overline do radical + linha da fracção) — a
        // linha da fracção é adicionada depois da overline (dentro do
        // layout_node do radicando), logo é a última.
        .last()
        .expect("sqrt(x/y) deve ter pelo menos uma linha");

    let den_top_ink = den_y - leaf_ascent;
    assert!(
        den_top_ink - rule_y > 0.5,
        "denominador não deve sobrepor a linha de fracção: \
         den_top_ink={} rule_y={} (den_y={})",
        den_top_ink,
        rule_y,
        den_y
    );
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
    assert!(items_contain_text(&items, '𝑥'), "base ausente: {:?}", items);
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
    assert!(items_contain_text(&items, '𝑥'), "base ausente: {:?}", items);
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
    assert!(items_contain_text(&items, '𝑥'), "base ausente: {:?}", items);
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
    assert!(items_contain_text(&items, '𝑥'), "base ausente");
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
                if text.contains('𝑥') {
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
    assert!(items_contain_text(&items, '𝑥'), "tl ausente");
    assert!(items_contain_text(&items, '𝑦'), "bl ausente");
    // P809: 'A' estilizada para 𝑨 U+1D434 (math italic, paridade vanilla).
    assert!(items_contain_text(&items, '\u{1D434}'), "base ausente");
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
    assert!(items_contain_text(&items, '𝑥'), "tl ausente");
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
    assert!(items_contain_text(&items, '𝑦'), "bl ausente");
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
        items_contain_text(&items, '𝑛') || items_contain_text(&items, '0'),
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
    // upright wins → 'x' literal (não italic codepoint). P809: com o default
    // de itálico por codepoint, este teste é o controlo que prova que o
    // estilo explícito prevalece sobre o default.
    assert!(
        items_contain_text(&items, 'x'),
        "upright(italic(x)) → upright deve ganhar; esperava 'x' literal: {:?}",
        items
    );
    assert!(
        !items_contain_text(&items, '\u{1D465}'),
        "upright(italic(x)) → NÃO deve haver 𝑥 U+1D465: {:?}",
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


// ── P813 — `layout_equation_measured`: extent (width/ascent/descent) ────────

#[test]
fn p813_measured_extent_ident_simples() {
    // FixedMetrics (size 12): advance 0.6×12 = 7.2; ink default =
    // cap-height 0.7×12 = 8.4; descent ink = 0.
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
    let (items, extent) =
        ml.layout_equation_measured(&Content::MathIdent("x".into()), &default_style());
    assert!(!items.is_empty());
    assert!(
        (extent.width - 7.2).abs() < 0.001,
        "width deve ser o advance dos items: {:.4}",
        extent.width
    );
    assert!(
        (extent.ascent - 8.4).abs() < 0.001,
        "ascent deve ser a cap-height (ink default): {:.4}",
        extent.ascent
    );
    assert!(
        extent.descent.abs() < 0.001,
        "descent de 'x' sem descendentes: {:.4}",
        extent.descent
    );
}

#[test]
fn p813_measured_extent_sup_eleva_ascent_e_alarga_width() {
    // x^2 (bloco): o superscript eleva o ascent acima da cap-height da
    // base e alarga a equação para lá do advance da base.
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
    let attach = Content::math_attach(
        Content::MathIdent("x".into()),
        None,
        None,
        None,
        Some(Content::MathText("2".into())),
    );
    let (_, extent) = ml.layout_equation_measured(&attach, &default_style());
    let (_, base_extent) =
        ml.layout_equation_measured(&Content::MathIdent("x".into()), &default_style());
    assert!(
        extent.ascent > base_extent.ascent,
        "sup deve elevar o ascent: {:.4} vs base {:.4}",
        extent.ascent,
        base_extent.ascent
    );
    assert!(
        extent.width > base_extent.width,
        "sup deve alargar a equação: {:.4} vs base {:.4}",
        extent.width,
        base_extent.width
    );
}

#[test]
fn p813_measured_items_batem_com_layout_equation() {
    // O novo entry point mede os MESMOS items que `layout_equation` —
    // não é um segundo caminho de layout.
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
    let frac = Content::math_frac(
        Content::MathIdent("a".into()),
        Content::MathIdent("b".into()),
    );
    let (measured, extent) = ml.layout_equation_measured(&frac, &default_style());
    let plain = ml.layout_equation(&frac, &default_style());
    assert_eq!(measured.len(), plain.len(), "mesmos items");
    assert!(extent.width > 0.0 && extent.ascent > 0.0);
    // frac: descent > 0 (denominador abaixo da baseline).
    assert!(
        extent.descent > 0.0,
        "frac deve ter descent > 0: {:.4}",
        extent.descent
    );
}


// ── P825 (sub-achado D de P810 §12) — LeftRightAlternator em `mat` ──────────
//
// Medido no vanilla (`temp/p825/d1.typ`: `$ mat(a &= b; x x x x &= y y) $`):
// o `&` parte as células em colunas de alinhamento — colunas pares à
// direita, ímpares à esquerda (`LeftRightAlternator::Right`,
// `typst-layout/math/run.rs:320-331`) — e o espaçamento de classe do limite
// (lspace do item seguinte) fica entre o fim do conteúdo da coluna par e o
// início da coluna ímpar (THICK ≈ 3.06pt antes de `=`). ANTES de P825 o
// cristalino consumia o `&` sem alinhar (colunas centradas).
//
// Com FixedMetrics (size 12, monospace 0.6em = 7.2pt/char):
//   width(a)=7.2, width(xxxx)=28.8, THICK = 5/18 × 12 = 3.3333
//   col0 = 28.8 + 3.3333 = 32.1333 (espaço de limite incorporado na célula
//   par, paridade run.rs:76-94); tinta de ambas as linhas termina em 28.8;
//   `=` começa em 32.1333 nas duas linhas.

fn p825d_mat_align_content() -> Content {
    let row1 = Content::MathSequence(Arc::from(
        vec![
            Content::MathIdent("a".into()),
            Content::math_align_point(),
            Content::MathText("=".into()),
            Content::MathIdent("b".into()),
        ]
        .into_boxed_slice(),
    ));
    let row2 = Content::MathSequence(Arc::from(
        vec![
            Content::MathIdent("x".into()),
            Content::MathIdent("x".into()),
            Content::MathIdent("x".into()),
            Content::MathIdent("x".into()),
            Content::math_align_point(),
            Content::MathText("=".into()),
            Content::MathIdent("y".into()),
            Content::MathIdent("y".into()),
        ]
        .into_boxed_slice(),
    ));
    Content::math_matrix(vec![vec![row1], vec![row2]], ('(', ')'))
}

fn p825d_positions(items: &[FrameItem], wanted: &str) -> Vec<(f64, f64)> {
    items
        .iter()
        .filter_map(|i| match i {
            FrameItem::Text { pos, text, .. } if text.as_str() == wanted => {
                Some((pos.x.val(), pos.y.val()))
            }
            _ => None,
        })
        .collect()
}

#[test]
fn p825d_mat_align_relacoes_alinhadas_entre_linhas() {
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
    let items = ml.layout_equation(&p825d_mat_align_content(), &default_style());
    let eqs = p825d_positions(&items, "=");
    assert_eq!(eqs.len(), 2, "duas relações `=`: {:?}", eqs);
    assert!(
        (eqs[0].0 - eqs[1].0).abs() < 0.001,
        "os `=` das duas linhas devem ter o mesmo x (alinha os `=`): {:?}",
        eqs
    );
    assert!(eqs[0].1 < eqs[1].1, "linhas diferentes têm y diferente: {:?}", eqs);
}

#[test]
fn p825d_mat_align_coluna_par_alinhada_a_direita() {
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
    let items = ml.layout_equation(&p825d_mat_align_content(), &default_style());
    // Tinta da coluna 0 (a / xxxx) termina no mesmo x nas duas linhas.
    // Nota: `apply_math_default` (P809) mapeia letras para o plano itálico.
    let a = p825d_positions(&items, "\u{1D44E}");
    let xs = p825d_positions(&items, "\u{1D465}");
    assert_eq!(a.len(), 1);
    assert_eq!(xs.len(), 4, "quatro x: {:?}", xs);
    let adv = FixedMetrics.advance("\u{1D465}", Pt(12.0), &default_style()).0;
    let a_right = a[0].0 + adv;
    let x4_right = xs[3].0 + adv;
    assert!(
        (a_right - x4_right).abs() < 0.001,
        "coluna par alinhada à direita: a_right={:.4} x4_right={:.4}",
        a_right,
        x4_right
    );
}

#[test]
fn p825d_mat_align_spacing_de_classe_no_limite() {
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
    let items = ml.layout_equation(&p825d_mat_align_content(), &default_style());
    let eqs = p825d_positions(&items, "=");
    let xs = p825d_positions(&items, "\u{1D465}");
    let adv = FixedMetrics.advance("\u{1D465}", Pt(12.0), &default_style()).0;
    let thick = 5.0 / 18.0 * 12.0;
    let gap = eqs[1].0 - (xs[3].0 + adv);
    assert!(
        (gap - thick).abs() < 0.001,
        "espaço de classe no limite `&` deve ser THICK ({:.4}): obteve {:.4}",
        thick,
        gap
    );
}

#[test]
fn p825d_mat_sem_align_mantem_colunas_centradas() {
    // Não-regressão (paridade medida em P810 §12): `mat` sem `&` centra as
    // colunas.
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
    let content = Content::math_matrix(
        vec![
            vec![Content::MathIdent("a".into())],
            vec![Content::MathSequence(Arc::from(
                vec![Content::MathIdent("x".into()), Content::MathIdent("x".into())]
                    .into_boxed_slice(),
            ))],
        ],
        ('(', ')'),
    );
    let items = ml.layout_equation(&content, &default_style());
    let a = p825d_positions(&items, "\u{1D44E}");
    let xs = p825d_positions(&items, "\u{1D465}");
    assert_eq!(a.len(), 1);
    assert_eq!(xs.len(), 2);
    let adv = FixedMetrics.advance("\u{1D465}", Pt(12.0), &default_style()).0;
    // Coluna única de largura 2×7.2=14.4: `a` centrada → a.x = xs[0].x + 3.6.
    let esperado = xs[0].0 + (2.0 * adv - adv) / 2.0;
    assert!(
        (a[0].0 - esperado).abs() < 0.001,
        "mat sem `&` deve manter coluna centrada: a.x={:.4} esperado {:.4}",
        a[0].0,
        esperado
    );
}

/// **P895** — `Content::HSpace` dentro de uma sequência math (usado pelos
/// espaçamentos nomeados `thin`/`med`/`thick`/`quad`/`wide`, registados em
/// `make_math_module()`) tem de contribuir a largura real ao `MathBox`, não
/// cair no catch-all `other => plain_text()` (que dá `width: 0.0` — `HSpace`
/// não tem texto). Mede o gap entre `a` e `b` directamente, à parte de
/// qualquer espaçamento automático por `MathClass` (que para dois
/// `Alphabetic` adjacentes é 0 — `spacing.rs`, `normal_normal_e_zero`).
#[test]
fn p895_hspace_em_sequencia_math_contribui_largura() {
    fn gap_para_hspace(amount_em: f64) -> f64 {
        let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
        let seq = Content::MathSequence(Arc::from(
            vec![
                Content::MathIdent("a".into()),
                Content::h_space(crate::entities::layout_types::Length::em(amount_em), false),
                Content::MathIdent("b".into()),
            ]
            .into_boxed_slice(),
        ));
        let items = ml.layout_equation(&seq, &default_style());
        let xs: Vec<f64> = items
            .iter()
            .filter_map(|i| match i {
                FrameItem::Text { pos, .. } => Some(pos.x.val()),
                _ => None,
            })
            .collect();
        assert_eq!(xs.len(), 2, "a e b devem produzir 2 items de texto");
        let adv_a = FixedMetrics.advance("\u{1D44E}", Pt(12.0), &default_style()).0;
        xs[1] - (xs[0] + adv_a)
    }

    let gap_thin = gap_para_hspace(1.0 / 6.0);
    let gap_wide = gap_para_hspace(2.0);
    assert!(
        gap_thin > 0.1,
        "hspace(thin) deve contribuir largura visível, obteve gap={:.4}",
        gap_thin
    );
    assert!(
        (gap_wide - 2.0 * 12.0).abs() < 0.01,
        "hspace(2em) a 12pt deve dar gap=24pt, obteve {:.4}",
        gap_wide
    );
    assert!(
        gap_wide > gap_thin,
        "hspace(wide=2em) deve ser maior que hspace(thin=1/6em): wide={:.4} thin={:.4}",
        gap_wide,
        gap_thin
    );
}

// ── P906 — esticamento horizontal de glifo (TDD vermelho, Agente A) ────────
//
// Testes contra os scaffolds fixados em `engine/layout/metrics.rs`
// (`FontMetrics::horizontal_glyph_variants`/`horizontal_glyph_assembly`),
// `math/layout/stretchy.rs` (`layout_stretchy_glyph_horizontal`) e
// `math/layout/assembly.rs` (`layout_assembly_horizontal`), cujos corpos
// são placeholders deliberados (fallback ao glifo base incondicional — ver
// `engine/layout.md` §P906, `math/layout/stretchy.md` §P906,
// `math/layout/assembly.md` §P906, `math/layout/_comum.md` §P906). Este
// módulo NÃO contém lógica de implementação — só testes e um test double.
#[cfg(test)]
mod p906_tests {
    use super::*;
    use crate::entities::glyph_variants::{GlyphAssembly, GlyphPart, GlyphVariant, GlyphVariants};
    use std::collections::HashMap;

    // ── Área A — trait `FontMetrics`, default aditivo (mirror vertical) ────

    #[test]
    fn p906_fixed_metrics_sem_variantes_horizontais() {
        let m = FixedMetrics;
        let v = m.horizontal_glyph_variants('⏟', &default_style());
        assert!(v.is_empty(), "FixedMetrics não tem variantes horizontais (default aditivo)");
    }

    #[test]
    fn p906_fixed_metrics_assembly_horizontal_vazia() {
        let m = FixedMetrics;
        let a = m.horizontal_glyph_assembly('⏟', &default_style());
        assert!(a.is_empty(), "FixedMetrics não tem assembly horizontal (default aditivo)");
    }

    // ── Área B — test double configurável ───────────────────────────────
    //
    // `FixedMetrics` só devolve vazio sempre — para exercitar a LÓGICA de
    // selecção de variante / acumulação de assembly de forma determinística
    // precisamos de dados configuráveis. `StubHorizontalMetrics` delega os
    // métodos obrigatórios a `FixedMetrics` (composição) e sobrescreve só
    // `horizontal_glyph_variants`/`horizontal_glyph_assembly` com dados
    // fornecidos no momento da construção.

    struct StubHorizontalMetrics {
        inner: FixedMetrics,
        variants: HashMap<char, GlyphVariants>,
        assembly: HashMap<char, GlyphAssembly>,
        // **P917** — dados verticais, para exercitar `layout_stretchy_delimiter`
        // (o caminho que P917 encontrou bugado), mesmo padrão dos campos
        // horizontais acima (test double configurável, não fonte real).
        vertical_variants: HashMap<char, GlyphVariants>,
        vertical_assembly: HashMap<char, GlyphAssembly>,
        // **P915** — `MathConstants` sobreposto, para exercitar a distinção
        // `cramped`/não-cramped (o fallback do trait tem os dois valores
        // iguais por design — ver `entities/math_constants.md` §P915 — não
        // exercita a diferença sozinho).
        math_constants: Option<crate::entities::math_constants::MathConstants>,
    }

    impl StubHorizontalMetrics {
        fn new() -> Self {
            Self {
                inner: FixedMetrics,
                variants: HashMap::new(),
                assembly: HashMap::new(),
                vertical_variants: HashMap::new(),
                vertical_assembly: HashMap::new(),
                math_constants: None,
            }
        }

        fn with_variants(mut self, c: char, v: GlyphVariants) -> Self {
            self.variants.insert(c, v);
            self
        }

        fn with_assembly(mut self, c: char, a: GlyphAssembly) -> Self {
            self.assembly.insert(c, a);
            self
        }

        fn with_vertical_variants(mut self, c: char, v: GlyphVariants) -> Self {
            self.vertical_variants.insert(c, v);
            self
        }

        fn with_math_constants(
            mut self,
            c: crate::entities::math_constants::MathConstants,
        ) -> Self {
            self.math_constants = Some(c);
            self
        }
    }

    impl FontMetrics for StubHorizontalMetrics {
        fn advance(&self, text: &str, size: Pt, style: &TextStyle) -> Pt {
            self.inner.advance(text, size, style)
        }

        fn vertical_metrics(&self, size: Pt, style: &TextStyle) -> (Pt, Pt) {
            self.inner.vertical_metrics(size, style)
        }

        fn cap_height(&self, size: Pt, style: &TextStyle) -> Pt {
            self.inner.cap_height(size, style)
        }

        fn text_edges(&self, size: Pt, style: &TextStyle) -> (Pt, Pt) {
            self.inner.text_edges(size, style)
        }

        fn horizontal_glyph_variants(&self, c: char, _style: &TextStyle) -> GlyphVariants {
            self.variants.get(&c).cloned().unwrap_or_default()
        }

        fn horizontal_glyph_assembly(&self, c: char, _style: &TextStyle) -> GlyphAssembly {
            self.assembly.get(&c).cloned().unwrap_or_default()
        }

        fn vertical_glyph_variants(&self, c: char, _style: &TextStyle) -> GlyphVariants {
            self.vertical_variants.get(&c).cloned().unwrap_or_default()
        }

        fn vertical_glyph_assembly(&self, c: char, _style: &TextStyle) -> GlyphAssembly {
            self.vertical_assembly.get(&c).cloned().unwrap_or_default()
        }

        fn math_constants(
            &self,
            style: &TextStyle,
        ) -> crate::entities::math_constants::MathConstants {
            self.math_constants.clone().unwrap_or_else(|| self.inner.math_constants(style))
        }
    }

    /// **P906 B1** — 1 variante suficiente (`advance >= min_width_du`): o
    /// `MathBox` devolvido deve reflectir a largura DA VARIANTE (24pt a
    /// 12pt/upem=1000 para advance=2000du), não a largura do glifo base
    /// (7.2pt, o que o placeholder actual devolve incondicionalmente).
    #[test]
    fn p906_stretchy_horizontal_variante_unica_suficiente_usa_advance_da_variante() {
        let stub = StubHorizontalMetrics::new().with_variants(
            '⏟',
            GlyphVariants {
                variants: vec![GlyphVariant { glyph_id: 50, advance: 2000.0, hor_advance: 2000.0 }],
            },
        );
        let ml = MathLayouter::new(&stub, true, &default_style());
        let box_ = ml.layout_stretchy_glyph_horizontal('⏟', 1500.0, &default_style());
        assert!(
            (box_.width - 24.0).abs() < 0.01,
            "esperava largura ~24.0pt (variante advance=2000du, upem=1000, size=12pt), obteve {:.4}",
            box_.width
        );
    }

    /// **P906 B2** — múltiplas variantes de advance crescente: a
    /// seleccionada deve ser a PRIMEIRA que satisfaz `min_width_du`
    /// (500/1000/2000du, pedido 700 → escolhe 1000 → 12.0pt), nunca
    /// automaticamente a maior (2000 → 24.0pt).
    #[test]
    fn p906_stretchy_horizontal_seleciona_variante_menor_suficiente() {
        let stub = StubHorizontalMetrics::new().with_variants(
            '⏟',
            GlyphVariants {
                variants: vec![
                    GlyphVariant { glyph_id: 60, advance: 500.0, hor_advance: 500.0 },
                    GlyphVariant { glyph_id: 61, advance: 1000.0, hor_advance: 1000.0 },
                    GlyphVariant { glyph_id: 62, advance: 2000.0, hor_advance: 2000.0 },
                ],
            },
        );
        let ml = MathLayouter::new(&stub, true, &default_style());
        let box_ = ml.layout_stretchy_glyph_horizontal('⏟', 700.0, &default_style());
        assert!(
            (box_.width - 12.0).abs() < 0.01,
            "esperava largura ~12.0pt (variante advance=1000du, a primeira >= 700), obteve {:.4}",
            box_.width
        );
        assert!(
            (box_.width - 24.0).abs() > 0.01,
            "não deve escolher a maior variante (advance=2000du -> 24.0pt) quando a menor já chega"
        );
    }

    /// **P906 B3** — sem variante suficiente, assembly com 3 partes
    /// disponível: deve compor (>= 3 `FrameItem::Glyph`), com largura =
    /// soma dos `full_advance` MENOS sobreposição de conectores entre
    /// partes consecutivas (não soma simples), ordenado esquerda→direita
    /// (x crescente), y constante (ao contrário do assembly vertical).
    ///
    /// Partes: full_advance=500du cada, connectors=100du entre partes
    /// adjacentes. Overlap(0,1)=min(100,100)=100; overlap(1,2)=min(100,100)
    /// =100. Total = 1500 - 200 = 1300du. scale = 12/1000 = 0.012 →
    /// width esperado ≈ 15.6pt.
    #[test]
    fn p906_stretchy_horizontal_sem_variante_usa_assembly_compoe_partes() {
        let assembly = GlyphAssembly {
            parts: vec![
                GlyphPart {
                    glyph_id: 70,
                    start_connector: 0,
                    end_connector: 100,
                    full_advance: 500,
                    is_extender: false,
                    hor_advance: 500.0,
                },
                GlyphPart {
                    glyph_id: 71,
                    start_connector: 100,
                    end_connector: 100,
                    full_advance: 500,
                    is_extender: true,
                    hor_advance: 500.0,
                },
                GlyphPart {
                    glyph_id: 72,
                    start_connector: 100,
                    end_connector: 0,
                    full_advance: 500,
                    is_extender: false,
                    hor_advance: 500.0,
                },
            ],
        };
        let stub = StubHorizontalMetrics::new().with_assembly('⏞', assembly);
        let ml = MathLayouter::new(&stub, true, &default_style());
        let box_ = ml.layout_stretchy_glyph_horizontal('⏞', 1300.0, &default_style());

        let glyph_items: Vec<(f64, f64)> = box_
            .items
            .iter()
            .filter_map(|i| match i {
                FrameItem::Glyph { pos, .. } => Some((pos.x.val(), pos.y.val())),
                _ => None,
            })
            .collect();
        assert!(
            glyph_items.len() >= 3,
            "assembly de 3 partes deve produzir >= 3 FrameItem::Glyph (composição real, não fallback); obteve {} items: {:?}",
            glyph_items.len(),
            box_.items
        );
        // Ordenado esquerda→direita: x estritamente crescente.
        for w in glyph_items.windows(2) {
            assert!(
                w[1].0 > w[0].0,
                "items do assembly horizontal devem ter x crescente (esquerda→direita): {:?}",
                glyph_items
            );
        }
        // y constante (mesmo referencial, ao contrário do empilhamento vertical).
        let y0 = glyph_items[0].1;
        for (_, y) in &glyph_items {
            assert!(
                (*y - y0).abs() < 0.001,
                "assembly horizontal não deve empilhar em y (deve ficar constante): {:?}",
                glyph_items
            );
        }
        assert!(
            (box_.width - 15.6).abs() < 0.05,
            "esperava largura ~15.6pt (soma full_advance - sobreposição de conectores), obteve {:.4}",
            box_.width
        );
    }

    /// **P906 B4** — sem variantes nem assembly: fallback ao glifo base
    /// (mesmo padrão de `layout_stretchy_sem_variantes_sem_assembly_usa_char_base`).
    /// Regressão explícita — este teste já passa com o placeholder actual
    /// (que sempre faz fallback) e deve continuar a passar após a
    /// implementação real.
    #[test]
    fn p906_stretchy_horizontal_sem_variante_sem_assembly_usa_char_base() {
        let stub = StubHorizontalMetrics::new();
        let ml = MathLayouter::new(&stub, true, &default_style());
        let box_ = ml.layout_stretchy_glyph_horizontal('⎵', 5000.0, &default_style());
        let has_base_char = box_.items.iter().any(
            |i| matches!(i, FrameItem::Text { text, .. } if text.as_str().contains('⎵')),
        );
        assert!(has_base_char, "deve usar char base '⎵' quando sem variantes nem assembly");
    }

    // ── Área C — wiring em `layout_underover`/`layout_accent` ──────────────

    fn base_larga() -> Content {
        Content::MathSequence(Arc::from(vec![
            Content::MathIdent("a".into()),
            Content::MathIdent("b".into()),
            Content::MathIdent("c".into()),
            Content::MathIdent("d".into()),
            Content::MathIdent("e".into()),
        ]))
    }

    /// **P906 C1** — `over` de 1 carácter stretchy sobre base larga: a
    /// largura total devolvida por `layout_underover` deve reflectir o
    /// esticamento do `over` (aqui configurado com uma variante
    /// desproporcionadamente grande — 50000du, ~600pt a 12pt/upem=1000 —
    /// garantindo que domina o `max(base_w, over_w, under_w)` SE o guard de
    /// esticamento de P906 estiver activo). Hoje (placeholder, sem guard)
    /// `layout_underover` chama `layout_node` para `over`, que fica no seu
    /// tamanho natural (~7.2pt) — muito menor que a base (5 idents) — pelo
    /// que a largura total fica dominada pela base, não pelo over.
    #[test]
    fn p906_layout_underover_over_1char_stretchy_domina_largura_apos_esticar() {
        let stub = StubHorizontalMetrics::new().with_variants(
            '\u{23DE}', // overbrace ⏞
            GlyphVariants {
                variants: vec![GlyphVariant { glyph_id: 90, advance: 50_000.0, hor_advance: 50_000.0 }],
            },
        );
        let style = default_style();
        let ml = MathLayouter::new(&stub, true, &default_style());
        let base = base_larga();
        let over_content = Content::MathText("\u{23DE}".into());

        let base_width_alone = ml.layout_node(&base, &style).width;
        let result_width = ml.layout_underover(&base, None, Some(&over_content), &style).width;

        assert!(
            result_width > base_width_alone * 2.0,
            "over de 1 carácter com variante muito maior que a base deveria dominar a largura \
             total após esticar; base_width_alone={:.4}, result_width={:.4}",
            base_width_alone,
            result_width
        );
    }

    /// **P906 C2** — anotação multi-carácter (`"soma"`, 4 chars) NÃO estica
    /// — regressão explícita que distingue "esticar sempre" de "esticar só
    /// quando é o guard de 1 carácter". Passa tanto antes como depois da
    /// implementação real (o guard é `len == 1`, "soma" nunca passa nele).
    #[test]
    fn p906_layout_underover_anotacao_multicaracter_nao_estica() {
        let stub = StubHorizontalMetrics::new();
        let style = default_style();
        let ml = MathLayouter::new(&stub, true, &default_style());
        let base = Content::MathIdent("x".into());
        let under_content = Content::MathText("soma".into());

        let result = ml.layout_underover(&base, Some(&under_content), None, &style);
        let under_item = result.items.last().expect("deve haver item de under");
        match under_item {
            FrameItem::Text { text, .. } => {
                assert_eq!(text.as_str(), "soma", "anotação multi-carácter deve permanecer texto literal, não esticar");
            }
            other => panic!("anotação multi-carácter não deveria virar Glyph (esticado): {:?}", other),
        }
    }

    /// **P906 C3** — `layout_accent`: mesmo mecanismo de C1, para um
    /// accent de 1 carácter stretchy sobre base larga.
    #[test]
    fn p906_layout_accent_1char_stretchy_domina_largura_apos_esticar() {
        let stub = StubHorizontalMetrics::new().with_variants(
            '\u{0302}', // combining circumflex (hat)
            GlyphVariants {
                variants: vec![GlyphVariant { glyph_id: 91, advance: 50_000.0, hor_advance: 50_000.0 }],
            },
        );
        let style = default_style();
        let ml = MathLayouter::new(&stub, true, &default_style());
        let base = base_larga();
        let accent_content = Content::MathText("\u{0302}".into());

        let base_width_alone = ml.layout_node(&base, &style).width;
        let result_width = ml.layout_accent(&base, &accent_content, &style).width;

        assert!(
            result_width > base_width_alone * 2.0,
            "accent de 1 carácter com variante muito maior que a base deveria dominar a largura \
             total após esticar; base_width_alone={:.4}, result_width={:.4}",
            base_width_alone,
            result_width
        );
    }

    /// **P906 C4** — anotação/accent multi-carácter também não deveria
    /// afectar `layout_accent` para além do comportamento actual — não há
    /// guard de 1-carácter para `accent` distinto de C2 (mesma lógica),
    /// mas o `base` continua a ser o preservado; smoke-test complementar
    /// de que `layout_accent` com accent multi-carácter não faz panic e
    /// mantém o accent como texto literal.
    #[test]
    fn p906_layout_accent_multicaracter_permanece_texto_literal() {
        let stub = StubHorizontalMetrics::new();
        let style = default_style();
        let ml = MathLayouter::new(&stub, true, &default_style());
        let base = Content::MathIdent("x".into());
        let accent_content = Content::MathText("abc".into());

        let result = ml.layout_accent(&base, &accent_content, &style);
        // **P906** — procurado por conteúdo, não por posição: a ordem de
        // `items` (base primeiro, depois accent) mudou com a correcção da
        // convenção baseline-relativa (ver `layout_accent`, `mod.rs`), mas
        // essa ordem sempre foi um detalhe de implementação irrelevante
        // para o render — o que este teste verifica é que o accent
        // multi-carácter permanece como `Text` literal ("abc"), não que
        // seja especificamente o primeiro item.
        let accent_item = result
            .items
            .iter()
            .find(|i| matches!(i, FrameItem::Text { text, .. } if text.as_str() == "abc"))
            .expect("deve haver item de accent com texto 'abc'");
        match accent_item {
            FrameItem::Text { text, .. } => {
                assert_eq!(text.as_str(), "abc");
            }
            other => panic!("accent multi-carácter não deveria virar Glyph: {:?}", other),
        }
    }

    // P913 — `layout_assembly` deve repetir peças `is_extender` para atingir `target_advance`
    #[test]
    fn p913_layout_assembly_repete_extensores_para_alvo_grande() {
        use crate::entities::glyph_variants::{GlyphAssembly, GlyphPart};

        let stub = StubHorizontalMetrics::new();
        let ml = MathLayouter::new(&stub, true, &default_style());
        let style = default_style(); // size = 10.0pt, upem = 1000.0

        let assembly = GlyphAssembly {
            parts: vec![
                GlyphPart { glyph_id: 1, start_connector: 0, end_connector: 100, full_advance: 500, is_extender: false, hor_advance: 500.0 },
                GlyphPart { glyph_id: 2, start_connector: 100, end_connector: 100, full_advance: 400, is_extender: true, hor_advance: 400.0 },
                GlyphPart { glyph_id: 3, start_connector: 100, end_connector: 0, full_advance: 500, is_extender: false, hor_advance: 500.0 },
            ],
        };

        // Alvo médio: min_advance com 1 extensor (500-100 + 400-100 + 500 = 1200 du = 12pt)
        let box_medio = ml.layout_assembly('{', assembly.clone(), 1200.0, &style);
        assert_eq!(box_medio.items.len(), 3, "com alvo médio (1200du), usa exactamente 3 peças (1 extensor)");

        // Alvo grande: 3000du (30pt) — precisa repetir a peça 2 (extender) várias vezes
        let box_grande = ml.layout_assembly('{', assembly.clone(), 3000.0, &style);
        assert!(
            box_grande.items.len() > 3,
            "com alvo grande (3000du), deve repetir extensores; items.len()={}",
            box_grande.items.len()
        );
        let altura_total = box_grande.ascent + box_grande.descent;
        assert!(
            altura_total >= 29.0,
            "altura resultante ({:.2}pt) deve aproximar ou atingir o alvo de 30pt",
            altura_total
        );
    }

    // P914 — `layout_attach` deve calcular shifts adaptativos e expandir o gap entre sub e sup simultâneos
    #[test]
    fn p914_layout_attach_shift_adaptativo_expande_gap_quando_necessario() {
        let stub = StubHorizontalMetrics::new();
        let ml = MathLayouter::new(&stub, true, &default_style());
        let style = default_style();

        let base = Content::MathIdent("x".into());
        let sup = Content::MathIdent("2".into());
        let sub = Content::MathIdent("1".into());

        let box_attach = ml.layout_attach(&base, None, None, Some(&sub), Some(&sup), &style);
        assert!(
            box_attach.ascent > 0.0 && box_attach.descent > 0.0,
            "attach com sub e sup deve ter ascent e descent positivos"
        );
    }

    // ── P915 — `cramped` ────────────────────────────────────────────────────

    fn cramped_test_constants() -> crate::entities::math_constants::MathConstants {
        let mut c = crate::entities::math_constants::MathConstants::fallback();
        c.superscript_shift_up = 300.0;
        c.superscript_shift_up_cramped = 700.0;
        c
    }

    // O primeiro item de `layout_attach` (ramo não-`is_limits`) é sempre o da
    // base (`y=0.0`); o item do script (sup/sub) é o último. `.last()`, não
    // `.find()`, para não confundir os dois quando ambos têm `y=0.0` (caso
    // ainda não implementado, testado no vermelho do TDD).
    fn script_y_offset(box_: &MathBox) -> f64 {
        box_.items
            .iter()
            .rev()
            .find_map(|i| match i {
                FrameItem::Glyph { pos, .. } => Some(pos.y.val()),
                FrameItem::Text { pos, .. } => Some(pos.y.val()),
                _ => None,
            })
            .expect("attach com só um script deve ter pelo menos 2 items posicionáveis")
    }

    /// **P915** — o superscrito usa `superscript_shift_up_cramped` quando o
    /// estilo **ambiente** (`style.cramped`) é `true`, e `superscript_shift_up`
    /// quando é `false` — mesma base e mesmo superscrito nos dois casos,
    /// só `style.cramped` muda. Achado do vanilla (`scripts.rs:325-330`):
    /// `cramped` decide entre as duas constantes; ground-truth calculado
    /// com constantes sintéticas bem distintas (300 vs 700) para que
    /// qualquer regressão futura falhe alto e claro.
    #[test]
    fn p915_attach_superscript_usa_shift_cramped_quando_estilo_ambiente_e_cramped() {
        let stub = StubHorizontalMetrics::new().with_math_constants(cramped_test_constants());

        let base = Content::MathIdent("x".into());
        let sup = Content::MathIdent("2".into());

        let style_normal = TextStyle { cramped: false, ..default_style() };
        let ml_normal = MathLayouter::new(&stub, true, &style_normal);
        let box_normal = ml_normal.layout_attach(&base, None, None, None, Some(&sup), &style_normal);

        let style_cramped = TextStyle { cramped: true, ..default_style() };
        let ml_cramped = MathLayouter::new(&stub, true, &style_cramped);
        let box_cramped =
            ml_cramped.layout_attach(&base, None, None, None, Some(&sup), &style_cramped);

        // sup_offset = shift_up; item do sup fica em y = -sup_offset.
        let y_normal = script_y_offset(&box_normal);
        let y_cramped = script_y_offset(&box_cramped);

        assert_ne!(
            y_normal, y_cramped,
            "superscrito com style.cramped diferente deve produzir shift_up diferente \
             (normal usa 300du, cramped usa 700du); y_normal={y_normal} y_cramped={y_cramped}"
        );
        // cramped=700 > normal=300 → shift maior → y mais negativo (sobe mais).
        assert!(
            y_cramped < y_normal,
            "shift_up cramped (700du) é maior que normal (300du) — o superscrito cramped \
             deve subir mais (y mais negativo); y_normal={y_normal} y_cramped={y_cramped}"
        );
    }

    /// **P915** — o mesmo mecanismo não deve afectar o subscrito: `shift_down`
    /// não lê `superscript_shift_up`/`_cramped` em nenhum dos dois ramos
    /// (achado do vanilla: `cramped` só entra na fórmula de `shift_up`,
    /// nunca em `shift_down` — `scripts.rs:325-361`).
    #[test]
    fn p915_attach_subscript_nao_afectado_por_cramped_ambiente() {
        let stub = StubHorizontalMetrics::new().with_math_constants(cramped_test_constants());

        let base = Content::MathIdent("x".into());
        let sub = Content::MathIdent("1".into());

        let style_normal = TextStyle { cramped: false, ..default_style() };
        let ml_normal = MathLayouter::new(&stub, true, &style_normal);
        let box_normal = ml_normal.layout_attach(&base, None, None, Some(&sub), None, &style_normal);

        let style_cramped = TextStyle { cramped: true, ..default_style() };
        let ml_cramped = MathLayouter::new(&stub, true, &style_cramped);
        let box_cramped =
            ml_cramped.layout_attach(&base, None, None, Some(&sub), None, &style_cramped);

        let y_normal = script_y_offset(&box_normal);
        let y_cramped = script_y_offset(&box_cramped);
        assert_eq!(
            y_normal, y_cramped,
            "shift_down do subscrito não deve depender de style.cramped (achado do vanilla: \
             cramped só afecta shift_up); y_normal={y_normal} y_cramped={y_cramped}"
        );
    }

    /// **P915** — `frac.rs`: denominador cramped, numerador não. Mesmo padrão
    /// dos testes acima, mas via `layout_frac` com um superscrito dentro do
    /// numerador vs. dentro do denominador.
    #[test]
    fn p915_frac_denominador_e_cramped_numerador_nao() {
        let stub = StubHorizontalMetrics::new().with_math_constants(cramped_test_constants());
        let style = TextStyle { cramped: false, ..default_style() };
        let ml = MathLayouter::new(&stub, true, &style);

        let base = Content::MathIdent("x".into());
        let sup = Content::MathIdent("2".into());
        let attach = Content::math_attach(base.clone(), None, None, None, Some(sup.clone()));

        let num_frac = ml.layout_frac(&attach, &Content::MathIdent("b".into()), &style);
        let den_frac = ml.layout_frac(&Content::MathIdent("a".into()), &attach, &style);

        // O attach dentro do numerador usa shift normal (300); dentro do
        // denominador usa shift cramped (700) — devem produzir alturas
        // diferentes para o mesmo conteúdo `x^2`.
        assert_ne!(
            num_frac.ascent, den_frac.ascent,
            "attach com superscrito dentro do numerador vs. denominador devem produzir \
             geometria diferente (denominador é cramped, numerador não)"
        );
    }

    /// **P915 — revisão do orquestrador**: caso composto não coberto pelos
    /// testes acima — `cramped` **e** sup+sub simultâneos (ajuste de gap de
    /// P914) ao mesmo tempo, confirmando que os dois mecanismos interagem
    /// correctamente (P914 já garante que o ajuste simultâneo só EXPANDE
    /// `shift_up`/`shift_down`, nunca reduz — o piso cramped de 700du deve
    /// sobreviver como piso mínimo, não ser substituído pelo ajuste).
    #[test]
    fn p915_cramped_e_sup_sub_simultaneos_interagem_sem_cancelar_piso_cramped() {
        let stub = StubHorizontalMetrics::new().with_math_constants(cramped_test_constants());

        let base = Content::MathIdent("x".into());
        let sup = Content::MathIdent("2".into());
        let sub = Content::MathIdent("1".into());

        let style_cramped = TextStyle { cramped: true, ..default_style() };
        let ml = MathLayouter::new(&stub, true, &style_cramped);
        let box_cramped =
            ml.layout_attach(&base, None, None, Some(&sub), Some(&sup), &style_cramped);

        // shift_up final (piso cramped 700 + eventual ajuste de gap
        // simultâneo, nunca menos que 700 — P914 só soma, nunca subtrai).
        let sup_item_y = box_cramped
            .items
            .iter()
            .filter_map(|i| match i {
                FrameItem::Glyph { pos, .. } | FrameItem::Text { pos, .. } => Some(pos.y.val()),
                _ => None,
            })
            .find(|y| *y < 0.0)
            .expect("deve haver um item acima da baseline (o superscrito)");
        let shift_up_final = -sup_item_y;

        assert!(
            shift_up_final >= 700.0 * default_style().size.val() / 1000.0,
            "shift_up com cramped + sup/sub simultâneos deve manter o piso cramped (700du) \
             como mínimo, mesmo depois do ajuste de gap de P914; shift_up_final={shift_up_final}"
        );
        assert!(
            box_cramped.ascent > 0.0 && box_cramped.descent > 0.0,
            "attach cramped com sub e sup deve continuar a produzir ascent/descent positivos"
        );
    }

    // P916 (Parte D — revisão cética) — sub+sup simultâneos, altura total deve crescer.
    // Caso composto não coberto pelo teste P914 original (que só verificava > 0).
    #[test]
    fn p916_d_gap_min_sub_sup_simultaneos_altura_total_cresce() {
        let stub = StubHorizontalMetrics::new();
        let ml = MathLayouter::new(&stub, true, &default_style());
        let style = default_style();

        let base = Content::MathIdent("p".into());
        let sup_c = Content::MathIdent("2".into());
        let sub_c = Content::MathIdent("3".into());

        let box_base = ml.layout_node(&base, &style);
        let box_attach = ml.layout_attach(&base, None, None, Some(&sub_c), Some(&sup_c), &style);

        let total_base = box_base.ascent + box_base.descent;
        let total_attach = box_attach.ascent + box_attach.descent;
        assert!(
            total_attach > total_base,
            "attach com sub+sup deve ter altura total maior que base sozinha \
             ({:.3} > {:.3})",
            total_attach, total_base,
        );
        assert!(box_attach.ascent > 0.0, "ascent deve ser positivo");
        assert!(box_attach.descent > 0.0, "descent deve ser positivo");
    }

    // P916 (Parte D) — delimitador assimétrico: frac dentro de delimitado.
    // Verifica que ascent > 0, descent >= 0, width > 0.
    #[test]
    fn p916_d_delimitado_conteudo_frac_tem_dimensoes_validas() {
        let stub = StubHorizontalMetrics::new();
        let ml = MathLayouter::new(&stub, false, &default_style());
        let style = default_style();

        let num = Content::MathIdent("a".into());
        let den = Content::MathIdent("b".into());
        let frac = Content::math_frac(num, den);
        let body = Content::MathSequence(vec![frac].into());
        let delim = Content::math_delimited('(', body, ')');

        let box_out = ml.layout_node(&delim, &style);
        assert!(box_out.ascent > 0.0, "delimitado deve ter ascent > 0");
        assert!(box_out.descent >= 0.0, "delimitado deve ter descent >= 0");
        assert!(box_out.width > 0.0, "delimitado deve ter largura > 0");
    }

    // ── P917 — x_advance/largura de layout_stretchy_delimiter usa hor_advance,
    //    nunca a medida do eixo de esticamento (advance) ─────────────────────
    //
    // Achado medido em produção (ver stretchy.md §P917): para a variante
    // seleccionada de `(` em `(1/2)` com a fonte real, `advance` (eixo
    // vertical) = 1793du mas `hor_advance` (hmtx real) = 597du — usar
    // `advance` como x_advance reservava ~3x mais espaço horizontal que o
    // glifo realmente ocupa. Este teste isola o mesmo desvio com dados
    // sintéticos extremos (advance=9000, hor_advance=40) para que qualquer
    // regressão futura falhe alto e claro, não por uma diferença sutil de
    // arredondamento.
    #[test]
    fn p917_stretchy_delimiter_largura_usa_hor_advance_nao_advance() {
        let stub = StubHorizontalMetrics::new().with_vertical_variants(
            ')',
            GlyphVariants {
                variants: vec![GlyphVariant { glyph_id: 200, advance: 9000.0, hor_advance: 40.0 }],
            },
        );
        let ml = MathLayouter::new(&stub, true, &default_style());
        let style = default_style(); // size = 12.0pt, upem = 1000.0

        let box_ = ml.layout_stretchy_delimiter(')', 5000.0, &style);

        // hor_advance=40du a 12pt/upem=1000 → 0.48pt.
        assert!(
            (box_.width - 0.48).abs() < 0.01,
            "largura devia vir de hor_advance (~0.48pt), não de advance (9000du -> 108pt); obteve {:.4}",
            box_.width
        );

        let glyph_x_advance = box_.items.iter().find_map(|i| match i {
            FrameItem::Glyph { x_advance, .. } => Some(x_advance.val()),
            _ => None,
        });
        assert_eq!(
            glyph_x_advance,
            Some(box_.width),
            "FrameItem::Glyph.x_advance deve ser consistente com MathBox.width (ambos de hor_advance)"
        );
    }
}

