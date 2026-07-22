//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/math/layout/_comum.md
//! @prompt-hash 6805c548
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

// ── P799 — sub+sup empilham na mesma origem x (paridade vanilla scripts.rs) ──

#[test]
fn math_attach_sub_sup_partilham_origem_x() {
    // Regressão P799: antes desta correcção, sub e sup eram compostos em
    // sequência horizontal (o cursor avançava depois do sup e o sub era
    // colocado a seguir a ele), em vez de partilharem a mesma origem x
    // imediatamente à direita da base (vanilla `scripts.rs`: `tr_x` e `br_x`
    // são ambos `pre_width + base_width + kern`).
    let ml = MathLayouter::new(&FixedMetrics, true);
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
    let ml = MathLayouter::new(&FixedMetrics, true);
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
    let ml = MathLayouter::new(&FixedMetrics, true);
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
    let ml = MathLayouter::new(&FixedMetrics, true);
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
    let ml = MathLayouter::new(&FixedMetrics, true);
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
    let ml = MathLayouter::new(&FixedMetrics, true);
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
    let ml = MathLayouter::new(&FixedMetrics, true);
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
    let ml = MathLayouter::new(&FixedMetrics, true);
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
    let ml = MathLayouter::new(&FixedMetrics, true);
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
    let ml = MathLayouter::new(&FixedMetrics, true);
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
    let ml = MathLayouter::new(&FixedMetrics, true);
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
    assert!(texts.iter().any(|t| t.contains('𝑥')), "deve conter x: {:?}", texts);
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
        texts.iter().any(|t| t.contains('𝑥')),
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
    assert!(texts.iter().any(|t| t.contains('𝑎')), "deve conter 'a': {:?}", texts);
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
    assert!(texts.iter().any(|t| t.contains('𝑎')), "numerador: {:?}", texts);
    assert!(texts.iter().any(|t| t.contains('𝑏')), "denominador: {:?}", texts);
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
        texts.iter().any(|t| t.contains('√') || t.contains('𝑥')),
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
    let ml = MathLayouter::new(&FixedMetrics, true);
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
    let ml = MathLayouter::new(&FixedMetrics, true);
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
    let ml = MathLayouter::new(&FixedMetrics, true);
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
    let ml = MathLayouter::new(&FixedMetrics, true);
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
    let ml = MathLayouter::new(&FixedMetrics, true);
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
    let ml = MathLayouter::new(&FixedMetrics, true);
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
    let ml = MathLayouter::new(&FixedMetrics, true);
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
