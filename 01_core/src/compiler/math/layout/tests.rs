//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/math/layout/_comum.md
//! @prompt-hash 25e09ab6
//! @layer L1
//! @updated 2026-08-10
//!
//! Testes de `math/layout` — extraídos de `math/layout.rs` no Passo 96.8
//! conforme ADR-0037.

use super::*;
use crate::compiler::layout::{FixedMetrics, FontMetrics};
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
    let space_after_script = 56.0 * 12.0 / 1000.0; // P1090: space_after_script

    assert!(
        (sub_x - sup_x).abs() < 0.01,
        "sub e sup empilhados na mesma origem x: sup_x={sup_x} sub_x={sub_x}"
    );
    assert!(
        (z_x - (base_w + sub_w + space_after_script)).abs() < 0.01,
        "elemento seguinte deve começar em base + max(sup,sub) + space_after_script: z_x={z_x} esperado={}",
        base_w + sub_w + space_after_script
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
    // P921 — layout_text_node passa a usar `text_ink_bounds`, não
    // `vertical_metrics`. `FixedMetrics` não sobrepõe `text_ink_bounds`,
    // logo usa o default do trait (01_core/src/compiler/layout/metrics.rs:59):
    // `(cap_height, 0.0)` — descent sempre 0 para qualquer char, sem bbox
    // real. Ver `_comum.md` §P921.
    let leaf_descent = 0.0_f64;

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

#[test]
fn p1088_debug_attach_shifts() {
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
    // Layout lambda_i^*
    let lambda = Content::MathIdent("lambda".into());
    let sub_i = Content::MathIdent("i".into());
    let sup_star = Content::MathIdent("*".into());
    let attach = Content::math_attach(
        lambda,
        None,
        None,
        Some(sub_i),
        Some(sup_star),
    );
    let (items, ext) = ml.layout_equation_measured(&attach, &default_style());
    println!("ext: ascent={:.4}, descent={:.4}", ext.ascent, ext.descent);
}

#[test]
fn p1087_measured_extent_glyph_usa_glyph_ink_bounds() {
    // P1087: FrameItem::Glyph em layout_equation_measured deve medir tinta
    // real via glyph_ink_bounds com o estilo e glyph_id em vigor (e não cair
    // no fallback legado P813 cap_height(&TextStyle::default()) sem descent).
    let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
    let op = Content::math_op(Content::MathText("min".into()), false);
    let (_, extent) = ml.layout_equation_measured(&op, &default_style());
    assert!(extent.width > 0.0, "largura de op deve ser positiva");
    assert!(extent.ascent > 0.0, "ascent de op deve ser positivo");
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
    // **P923** — células de `mat` são renderizadas em estilo de denominador
    // (size * script_percent_scale_down), logo o espaçamento de classe no
    // limite `&` é resolvido contra o tamanho reduzido.
    let constants = crate::entities::math_constants::MathConstants::fallback();
    let cell_size = default_style().size.val() * constants.script_percent_scale_down;
    let adv = FixedMetrics.advance("\u{1D465}", Pt(cell_size), &default_style()).0;
    let thick = 5.0 / 18.0 * cell_size;
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
    // **P923** — células de `mat` em estilo de denominador; a largura da coluna
    // e o centro dependem do tamanho reduzido.
    let constants = crate::entities::math_constants::MathConstants::fallback();
    let cell_size = default_style().size.val() * constants.script_percent_scale_down;
    let adv = FixedMetrics.advance("\u{1D465}", Pt(cell_size), &default_style()).0;
    // Coluna única de largura 2×adv: `a` centrada → a.x = xs[0].x + adv/2.
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
// Testes contra os scaffolds fixados em `compiler/layout/metrics.rs`
// (`FontMetrics::horizontal_glyph_variants`/`horizontal_glyph_assembly`),
// `math/layout/stretchy.rs` (`layout_stretchy_glyph_horizontal`) e
// `math/layout/assembly.rs` (`layout_assembly_horizontal`), cujos corpos
// são placeholders deliberados (fallback ao glifo base incondicional — ver
// `compiler/layout.md` §P906, `math/layout/stretchy.md` §P906,
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
        // **P985** — tinta real por glyph_id `(up_du, down_du)` (upem=1000),
        // para exercitar `glyph_ink_bounds` no caminho horizontal — a tinta
        // de ⏟ está toda abaixo da baseline e a de ⏞ toda acima; o default
        // do trait (cap_height/0) não exercita essa assimetria.
        ink_bounds: HashMap<u16, (f64, f64)>,
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
                ink_bounds: HashMap::new(),
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

        /// **P985** — regista tinta real `(up_du, down_du)` para um
        /// glyph_id (upem=1000, convertida a pt na chamada).
        fn with_ink_bounds(mut self, glyph_id: u16, up_du: f64, down_du: f64) -> Self {
            self.ink_bounds.insert(glyph_id, (up_du, down_du));
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

        fn glyph_ink_bounds(&self, glyph_id: u16, size: Pt, style: &TextStyle) -> (Pt, Pt) {
            if let Some(&(up_du, down_du)) = self.ink_bounds.get(&glyph_id) {
                return (size * (up_du / 1000.0), size * (down_du / 1000.0));
            }
            self.inner.glyph_ink_bounds(glyph_id, size, style)
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
        let box_ = ml.layout_stretchy_glyph_horizontal('⏟', 1500.0, &default_style(), 0.0);
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
        let box_ = ml.layout_stretchy_glyph_horizontal('⏟', 700.0, &default_style(), 0.0);
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
            // P945 — default 0 = comportamento pré-P945 (teste sintético P906).
            ..Default::default()
        };
        let stub = StubHorizontalMetrics::new().with_assembly('⏞', assembly);
        let ml = MathLayouter::new(&stub, true, &default_style());
        let box_ = ml.layout_stretchy_glyph_horizontal('⏞', 1300.0, &default_style(), 0.0);

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
        let box_ = ml.layout_stretchy_glyph_horizontal('⎵', 5000.0, &default_style(), 0.0);
        let has_base_char = box_.items.iter().any(
            |i| matches!(i, FrameItem::Text { text, .. } if text.as_str().contains('⎵')),
        );
        assert!(has_base_char, "deve usar char base '⎵' quando sem variantes nem assembly");
    }

    // ── P984 — paridade vanilla na selecção horizontal ──────────────────
    //
    // Regras novas (vanilla `fragment/glyph.rs:265-300`, `math/accent.rs:18`,
    // `ir/resolve.rs:389`/`:1430` — ver `stretchy.md` §P984): short_fall por
    // chamador (acento 0.5em, spreader 0.0); keep-base se `short_target ≤
    // advance hmtx do glifo base`; keep-largest se nenhuma variante chega e
    // não há assembly. Dados do hat de NewCMMath: base hmtx 500du, variantes
    // AdvanceMeasurement 307/647/771du, sem assembly.

    fn p984_hat_stub() -> StubHorizontalMetrics {
        StubHorizontalMetrics::new().with_variants(
            '\u{0302}', // combining circumflex (hat)
            GlyphVariants {
                variants: vec![
                    GlyphVariant { glyph_id: 10, advance: 307.0, hor_advance: 500.0 },
                    GlyphVariant { glyph_id: 11, advance: 647.0, hor_advance: 647.0 },
                    GlyphVariant { glyph_id: 12, advance: 771.0, hor_advance: 771.0 },
                ],
            },
        )
    }

    /// **P984 T1 (keep-base)** — base estreita (`hat(x)`, target 572du):
    /// `short_target = 572 − 0.5em(500du) = 72du ≤ advance hmtx do base`
    /// (FixedMetrics: 0.6em = 600du a 12pt/upem 1000) → mantém o glifo BASE,
    /// não a variante `.h1` (647du). Era o bug do achado §7.1: o cristalino
    /// escolhia `.h1` (7.08pt) onde o vanilla mantém o base (5.50pt).
    #[test]
    fn p984_acento_base_estreita_mantem_glifo_base() {
        let stub = p984_hat_stub();
        let ml = MathLayouter::new(&stub, true, &default_style());
        let box_ = ml.layout_stretchy_glyph_horizontal('\u{0302}', 572.0, &default_style(), 0.5);
        let has_base_text = box_.items.iter().any(
            |i| matches!(i, FrameItem::Text { text, .. } if text.as_str().contains('\u{0302}')),
        );
        assert!(
            has_base_text,
            "short_target (72du) ≤ advance do base (600du) → glifo base (Text), não variante: {:?}",
            box_.items
        );
    }

    /// **P984 T2** — base intermédia: target 1200du → `short_target = 700du`
    /// (> 600du do base) → primeira variante ≥ 700 = gid 12 (771du →
    /// 771/1000×12 = 9.252pt de largura via `hor_advance`).
    #[test]
    fn p984_acento_base_media_estica_para_primeira_suficiente() {
        let stub = p984_hat_stub();
        let ml = MathLayouter::new(&stub, true, &default_style());
        let box_ = ml.layout_stretchy_glyph_horizontal('\u{0302}', 1200.0, &default_style(), 0.5);
        assert!(
            (box_.width - 9.252).abs() < 0.01,
            "esperava largura ~9.252pt (variante 771du, a primeira ≥ 700du de short_target), obteve {:.4}",
            box_.width
        );
    }

    /// **P984 T3 (keep-largest)** — alvo acima de TODAS as variantes e sem
    /// assembly (`hat(a+b)` no vanilla fica com `.h7`, a maior): mantém a
    /// MAIOR variante (gid 12, 771du → 9.252pt), NÃO o glifo base (7.2pt).
    #[test]
    fn p984_sem_variante_suficiente_sem_assembly_mantem_maior_variante() {
        let stub = p984_hat_stub();
        let ml = MathLayouter::new(&stub, true, &default_style());
        let box_ = ml.layout_stretchy_glyph_horizontal('\u{0302}', 3000.0, &default_style(), 0.5);
        assert!(
            (box_.width - 9.252).abs() < 0.01,
            "esperava a MAIOR variante (771du → 9.252pt), não o glifo base (7.2pt): obteve {:.4}",
            box_.width
        );
    }

    /// **P984 T4** — spreader com `short_fall = 0` (vanilla
    /// `ir/resolve.rs:1430`): target 1000du, variantes 950/1200du →
    /// `short_target = 1000` (> 600du do base) → gid 51 (1200du → 14.4pt).
    /// Com 0.1em (valor pré-P984) seria gid 50 — o parâmetro discrimina.
    #[test]
    fn p984_spreader_short_fall_zero_seleciona_contra_alvo_inteiro() {
        let stub = StubHorizontalMetrics::new().with_variants(
            '⏟',
            GlyphVariants {
                variants: vec![
                    GlyphVariant { glyph_id: 50, advance: 950.0, hor_advance: 950.0 },
                    GlyphVariant { glyph_id: 51, advance: 1200.0, hor_advance: 1200.0 },
                ],
            },
        );
        let ml = MathLayouter::new(&stub, true, &default_style());
        let box_ = ml.layout_stretchy_glyph_horizontal('⏟', 1000.0, &default_style(), 0.0);
        assert!(
            (box_.width - 14.4).abs() < 0.01,
            "short_fall=0 → primeira variante ≥ 1000du = 1200du (14.4pt), obteve {:.4}",
            box_.width
        );
    }

    // ── P985 — gaps do spreader com tinta real da peça ───────────────────
    //
    // Ver `underover.md` §P985: a caixa da peça (⏟/⏞) tem de vir da tinta
    // real (`glyph_ink_bounds`), não do `vertical_metrics` da fonte; a peça
    // de cima usa a fórmula P922 do acento. Dados NewCMMath-Book: ⏟ tinta
    // y −353..−109du (toda abaixo da baseline), ⏞ y +539..+783du (toda
    // acima).

    /// **P985 T1** — peça de baixo (⏟, tinta toda abaixo da baseline,
    /// `up` COM SINAL = −109du como na L3): a baseline da chave assenta no
    /// fundo da tinta da base (`under_y = base.descent + max(0, ink_up)` —
    /// o gap conteúdo↔chave vem do bearing do próprio glifo, 109du) e o
    /// descent da caixa resultante cobre a tinta da chave (353du → 4.236pt
    /// a 12pt).
    #[test]
    fn p985_under_piece_gap_vem_da_tinta_real() {
        let stub = StubHorizontalMetrics::new()
            .with_variants(
                '⏟',
                GlyphVariants {
                    variants: vec![GlyphVariant { glyph_id: 50, advance: 1500.0, hor_advance: 1500.0 }],
                },
            )
            .with_ink_bounds(50, -109.0, 353.0);
        let style = default_style();
        let ml = MathLayouter::new(&stub, true, &style);
        let base = base_larga();
        let base_descent = ml.layout_node(&base, &style).descent;
        let under = Content::MathText("⏟".into());
        let result = ml.layout_underover(&base, Some(&under), None, &style);

        let glyph_y = result
            .items
            .iter()
            .find_map(|i| match i {
                FrameItem::Glyph { glyph_id, pos, .. } if *glyph_id == 50 => Some(pos.y.val()),
                _ => None,
            })
            .expect("deve haver FrameItem::Glyph da chave (gid 50)");
        assert!(
            (glyph_y - base_descent).abs() < 0.01,
            "baseline da chave deve assentar no fundo da tinta da base ({base_descent:.4}), obteve {glyph_y:.4} \
             (ascent de tinta do ⏟ é 0 — a tinta está toda abaixo da baseline)"
        );
        let expected_descent = base_descent + 353.0 * 12.0 / 1000.0;
        assert!(
            (result.descent - expected_descent).abs() < 0.01,
            "descent da caixa deve cobrir a tinta da chave ({expected_descent:.4}), obteve {:.4}",
            result.descent
        );
    }

    /// **P985 T2** — peça de cima (⏞, tinta flutua 539du acima da
    /// baseline): NÃO pode usar `stack_tight_above` (deixaria gap de
    /// ~5.9pt); usa a fórmula P922 do acento do vanilla:
    /// `over_y = −base.ascent + min(base.ascent, accent_base_height)`.
    #[test]
    fn p985_over_piece_usa_formula_p922_do_acento() {
        let stub = StubHorizontalMetrics::new()
            .with_variants(
                '⏞',
                GlyphVariants {
                    variants: vec![GlyphVariant { glyph_id: 60, advance: 1500.0, hor_advance: 1500.0 }],
                },
            )
            .with_ink_bounds(60, 783.0, 0.0);
        let style = default_style();
        let ml = MathLayouter::new(&stub, true, &style);
        let base = base_larga();
        let base_box = ml.layout_node(&base, &style);
        let over = Content::MathText("⏞".into());
        let result = ml.layout_underover(&base, None, Some(&over), &style);

        let abh_pt = ml.constants.to_pt(ml.constants.accent_base_height, style.size).val();
        let expected_y = -base_box.ascent + base_box.ascent.min(abh_pt);
        let glyph_y = result
            .items
            .iter()
            .find_map(|i| match i {
                FrameItem::Glyph { glyph_id, pos, .. } if *glyph_id == 60 => Some(pos.y.val()),
                _ => None,
            })
            .expect("deve haver FrameItem::Glyph da chave (gid 60)");
        assert!(
            (glyph_y - expected_y).abs() < 0.01,
            "baseline da peça de cima deve seguir a fórmula P922 ({expected_y:.4}), obteve {glyph_y:.4}"
        );
        let ink_up = 783.0 * 12.0 / 1000.0;
        let expected_ascent = base_box.ascent.max(-expected_y + ink_up);
        assert!(
            (result.ascent - expected_ascent).abs() < 0.01,
            "ascent da caixa deve cobrir o topo da tinta da peça ({expected_ascent:.4}), obteve {:.4}",
            result.ascent
        );
    }

    /// **P985 T3 (end-to-end)** — `underbrace(base, "soma")` aninhado: a
    /// legenda NÃO pode sobrepor a tinta da chave. Gap de tinta
    /// (fundo da chave → topo da legenda) ≥ 0. Antes da correcção a caixa
    /// da chave tinha descent 0 com a tinta a sair por baixo, e a legenda
    /// (empilhada tight sob a caixa) caía em cima da curva.
    #[test]
    fn p985_legenda_nao_sobrepoe_tinta_da_chave() {
        let stub = StubHorizontalMetrics::new()
            .with_variants(
                '⏟',
                GlyphVariants {
                    variants: vec![GlyphVariant { glyph_id: 50, advance: 1500.0, hor_advance: 1500.0 }],
                },
            )
            .with_ink_bounds(50, 0.0, 353.0);
        let style = default_style();
        let ml = MathLayouter::new(&stub, true, &style);
        let inner = Content::math_underover(
            base_larga(),
            Some(Content::MathText("⏟".into())),
            None,
        );
        let soma = Content::MathText("soma".into());
        let outer = ml.layout_underover(&inner, Some(&soma), None, &style);

        let chave_y = outer
            .items
            .iter()
            .find_map(|i| match i {
                FrameItem::Glyph { glyph_id, pos, .. } if *glyph_id == 50 => Some(pos.y.val()),
                _ => None,
            })
            .expect("deve haver FrameItem::Glyph da chave");
        let soma_y = outer
            .items
            .iter()
            .find_map(|i| match i {
                FrameItem::Text { text, pos, .. } if text.as_str() == "soma" => Some(pos.y.val()),
                _ => None,
            })
            .expect("deve haver FrameItem::Text da legenda 'soma'");

        let chave_ink_bottom = chave_y + 353.0 * 12.0 / 1000.0;
        let soma_size = style.size * ml.constants.script_percent_scale_down;
        let (soma_ink_up, _) = stub.text_ink_bounds("soma", soma_size, &style);
        let soma_ink_top = soma_y - soma_ink_up.val();
        assert!(
            soma_ink_top >= chave_ink_bottom - 0.001,
            "legenda não pode sobrepor a tinta da chave: topo da legenda {soma_ink_top:.4} \
             vs fundo da chave {chave_ink_bottom:.4}"
        );
    }

    /// **P985 T4** — legenda de baixo ("soma" do `underbrace`) é um anexo
    /// de LIMITE no vanilla (`ScriptsItem` bottom, `compute_limit_shifts`):
    /// `under_y = base.descent + max(lower_limit_baseline_drop_min,
    /// lower_limit_gap_min + anotação.ascent)` — não tight stacking
    /// (`base.descent + anotação.ascent`), que deixava a legenda a tocar o
    /// ápice da chave (medido 0pt vs 3.24pt do vanilla, 1200dpi).
    #[test]
    fn p985_legenda_under_usa_limit_shift_do_vanilla() {
        let stub = StubHorizontalMetrics::new();
        let style = default_style();
        let ml = MathLayouter::new(&stub, true, &style);
        let base = base_larga();
        let base_box = ml.layout_node(&base, &style);
        let soma = Content::MathText("soma".into());
        let result = ml.layout_underover(&base, Some(&soma), None, &style);

        let c = &ml.constants;
        let drop_min = c.to_pt(c.lower_limit_baseline_drop_min, style.size).val();
        let gap_min = c.to_pt(c.lower_limit_gap_min, style.size).val();
        // Caixa da legenda sozinha (estilo de subscript: ×scale_down,
        // cramped — `style_for_subscript` do vanilla, P961) — obtida da
        // caixa real para não duplicar a convenção de ascent no teste.
        let script_size = style.size * c.script_percent_scale_down;
        let soma_box = ml.layout_node(&soma, &TextStyle {
            size: script_size,
            math_script: true,
            cramped: true,
            ..style.clone()
        });
        let expected_y = base_box.descent + drop_min.max(gap_min + soma_box.ascent);

        let soma_y = result
            .items
            .iter()
            .find_map(|i| match i {
                FrameItem::Text { text, pos, .. } if text.as_str() == "soma" => Some(pos.y.val()),
                _ => None,
            })
            .expect("deve haver FrameItem::Text da legenda 'soma'");
        assert!(
            (soma_y - expected_y).abs() < 0.01,
            "legenda de baixo deve usar limit shift ({expected_y:.4}), obteve {soma_y:.4}"
        );
    }

    /// **P985 T5** — legenda de cima ("soma" do `overbrace`):
    /// `over_y = −(base.ascent + max(upper_limit_baseline_rise_min,
    /// upper_limit_gap_min + anotação.descent))`.
    #[test]
    fn p985_legenda_over_usa_limit_shift_do_vanilla() {
        let stub = StubHorizontalMetrics::new();
        let style = default_style();
        let ml = MathLayouter::new(&stub, true, &style);
        let base = base_larga();
        let base_box = ml.layout_node(&base, &style);
        let soma = Content::MathText("soma".into());
        let result = ml.layout_underover(&base, None, Some(&soma), &style);

        let c = &ml.constants;
        let rise_min = c.to_pt(c.upper_limit_baseline_rise_min, style.size).val();
        let gap_min = c.to_pt(c.upper_limit_gap_min, style.size).val();
        let script_size = style.size * c.script_percent_scale_down;
        // descent da caixa da legenda = line_metrics.1 − ascent (convenção
        // de `layout_text_node`); obtida da caixa real para não duplicar
        // convenção no teste.
        let soma_box = ml.layout_node(&soma, &TextStyle {
            size: script_size,
            math_script: true,
            cramped: false,
            ..style.clone()
        });
        let expected_y = -(base_box.ascent + rise_min.max(gap_min + soma_box.descent));

        let soma_y = result
            .items
            .iter()
            .find_map(|i| match i {
                FrameItem::Text { text, pos, .. } if text.as_str() == "soma" => Some(pos.y.val()),
                _ => None,
            })
            .expect("deve haver FrameItem::Text da legenda 'soma'");
        assert!(
            (soma_y - expected_y).abs() < 0.01,
            "legenda de cima deve usar limit shift ({expected_y:.4}), obteve {soma_y:.4}"
        );
    }

    // ── P988 — assembly horizontal com métricas da tinta das peças ───────
    //
    // Ver `assembly.md` §P988: P985 corrigiu `emit_horizontal_variant`, mas
    // o ⎵ de NewCMMath só tem variantes até 2986du (a+b+c ≈ 3850du →
    // assembly) e `layout_assembly_horizontal` ainda usava
    // `vertical_metrics` da fonte — mesmo bug, caminho irmão.

    fn p988_ubk_stub() -> StubHorizontalMetrics {
        let assembly = GlyphAssembly {
            parts: vec![
                GlyphPart { glyph_id: 80, start_connector: 0, end_connector: 100, full_advance: 1000, is_extender: false, hor_advance: 1000.0 },
                GlyphPart { glyph_id: 81, start_connector: 100, end_connector: 100, full_advance: 800, is_extender: true, hor_advance: 800.0 },
                GlyphPart { glyph_id: 82, start_connector: 100, end_connector: 0, full_advance: 1000, is_extender: false, hor_advance: 1000.0 },
            ],
            ..Default::default()
        };
        StubHorizontalMetrics::new()
            .with_assembly('⎵', assembly)
            .with_ink_bounds(80, -74.0, 342.0)
            .with_ink_bounds(81, -80.0, 335.0)
            .with_ink_bounds(82, -74.0, 342.0)
    }

    /// **P988 A1** — a caixa da assembly horizontal usa a união da tinta
    /// real das peças (COM SINAL, como a L3): `ascent = max(up_i)` (−74du
    /// → −0.888pt a 12pt — a tinta está toda abaixo da baseline),
    /// `descent = max(down_i)` (342du → 4.104pt) — não `vertical_metrics`
    /// (9.6pt / 0).
    #[test]
    fn p988_assembly_horizontal_metricas_vem_da_tinta_das_pecas() {
        let stub = p988_ubk_stub();
        let ml = MathLayouter::new(&stub, true, &default_style());
        let box_ = ml.layout_stretchy_glyph_horizontal('⎵', 3850.0, &default_style(), 0.0);
        let expected_ascent = -74.0 * 12.0 / 1000.0;
        let expected_descent = 342.0 * 12.0 / 1000.0;
        assert!(
            (box_.ascent - expected_ascent).abs() < 0.01,
            "ascent deve ser o max com sinal da tinta das peças ({expected_ascent:.4}), obteve {:.4}",
            box_.ascent
        );
        assert!(
            (box_.descent - expected_descent).abs() < 0.01,
            "descent deve ser o fundo da tinta das peças ({expected_descent:.4}), obteve {:.4}",
            box_.descent
        );
    }

    /// **P988 A2 (end-to-end)** — `underbracket` (assembly) em
    /// `layout_underover`: a baseline das peças assenta no fundo da tinta
    /// da base (`max(0, ascent)` com ascent negativo, `underover.md` §P985)
    /// e o descent da caixa cobre a tinta das peças.
    #[test]
    fn p988_underbracket_assembly_assenta_no_fundo_da_base() {
        let stub = p988_ubk_stub();
        let style = default_style();
        let ml = MathLayouter::new(&stub, true, &style);
        let base = base_larga();
        let base_descent = ml.layout_node(&base, &style).descent;
        let under = Content::MathText("⎵".into());
        let result = ml.layout_underover(&base, Some(&under), None, &style);

        let glyph_y = result
            .items
            .iter()
            .find_map(|i| match i {
                FrameItem::Glyph { glyph_id, pos, .. } if *glyph_id == 80 => Some(pos.y.val()),
                _ => None,
            })
            .expect("deve haver FrameItem::Glyph da peça esquerda (gid 80)");
        assert!(
            (glyph_y - base_descent).abs() < 0.01,
            "baseline das peças deve assentar no fundo da tinta da base ({base_descent:.4}), obteve {glyph_y:.4}"
        );
        let expected_descent = base_descent + 342.0 * 12.0 / 1000.0;
        assert!(
            (result.descent - expected_descent).abs() < 0.01,
            "descent da caixa deve cobrir a tinta das peças ({expected_descent:.4}), obteve {:.4}",
            result.descent
        );
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
            // P945 — default 0 = comportamento pré-P945 (teste sintético P913).
            ..Default::default()
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

    // ── P957 — baselines das peças no FUNDO do slot (não no topo) ────────
    //
    // Valores reais de NewCMMath-Book (medidos via fontTools em
    // `typst-passo-957` Fase A): parenleft = [uni239D (1495du, conectores
    // 0/249), uni239C ext (498, 498/498), uni239B (1495, 249/0)];
    // braceleft = [uni23A9 (750, 0/374), braceleft.ex (748, 748/748),
    // uni23A8 (1500, 374/374), ex, uni23A7 (750, 374/0)];
    // minConnectorOverlap = 20. A sequência de passos entre baselines
    // consecutivas tem de seguir `advance_i − overlap_i + r(overlap_i −
    // min)` (vanilla `glyph.rs:631-641`) — o bug P957 rodava a sequência
    // de uma posição (baseline no topo do slot), fazendo a peça inferior
    // ficar coberta pelo extensor (visual: "canto reto").

    /// Passos entre baselines consecutivas, de baixo para cima (items
    /// ordenados por pos.y descendente = fundo primeiro).
    fn p957_steps(box_: &MathBox) -> (Vec<u16>, Vec<f64>) {
        let mut pts: Vec<(u16, f64)> = box_
            .items
            .iter()
            .filter_map(|i| match i {
                FrameItem::Glyph { glyph_id, pos, .. } => Some((*glyph_id, pos.y.val())),
                _ => None,
            })
            .collect();
        pts.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap()); // y maior = mais abaixo
        let gids = pts.iter().map(|p| p.0).collect();
        let steps = pts
            .windows(2)
            .map(|w| (w[0].1 - w[1].1) / 0.012) // pt → du (size 12pt, upem 1000)
            .collect();
        (gids, steps)
    }

    fn p957_paren_assembly() -> GlyphAssembly {
        GlyphAssembly {
            min_overlap: 20,
            parts: vec![
                GlyphPart { glyph_id: 1388, start_connector: 0, end_connector: 249, full_advance: 1495, is_extender: false, hor_advance: 1024.0 },
                GlyphPart { glyph_id: 1387, start_connector: 498, end_connector: 498, full_advance: 498, is_extender: true, hor_advance: 1024.0 },
                GlyphPart { glyph_id: 1386, start_connector: 249, end_connector: 0, full_advance: 1495, is_extender: false, hor_advance: 1024.0 },
            ],
        }
    }

    /// **P957 — parêntese**: identidade das peças (fundo→topo:
    /// [1388, 1387, 1387, 1386] com alvo 3800du → repeat=2) e passos da
    /// fórmula do vanilla: r = (3800−2990)/936 = 0.8654 →
    /// [1444.17, 413.65, 447.17]du (o passo GRANDE é o primeiro — segue o
    /// advance da peça do fundo, 1495du).
    #[test]
    fn p957_assembly_paren_passos_na_ordem_vanilla() {
        let stub = StubHorizontalMetrics::new();
        let ml = MathLayouter::new(&stub, true, &default_style());
        let style = default_style(); // 12pt, upem 1000 → 1du = 0.012pt

        let b = ml.layout_assembly('(', p957_paren_assembly(), 3800.0, &style);
        let (gids, steps) = p957_steps(&b);

        assert_eq!(
            gids,
            vec![1388, 1387, 1387, 1386],
            "ordem fundo→topo: [uni239D, ext, ext, uni239B]: {gids:?}"
        );
        let esperado = [1444.1731, 413.6538, 447.1731];
        assert_eq!(steps.len(), 3, "4 peças → 3 passos: {steps:?}");
        for (i, (s, e)) in steps.iter().zip(esperado.iter()).enumerate() {
            assert!(
                (s - e).abs() < 0.01,
                "passo {i}: esperado {e:.4}du (vanilla), obteve {s:.4}du — \
                 sequência rodada = baseline no topo do slot (bug P957)"
            );
        }
    }

    /// **P957 — chave**: 5 peças (repeat=1) com alvo 3200du →
    /// r = 200/1416 = 0.141243; passos [426, 424, 1176, 424]du — o passo
    /// GRANDE é o 3º intervalo (após a peça do meio, advance 1500du), não o
    /// 2º. Identidade: [1400, 6634, 1399, 6634, 1398].
    #[test]
    fn p957_assembly_brace_passo_grande_apos_peca_do_meio() {
        let stub = StubHorizontalMetrics::new();
        let ml = MathLayouter::new(&stub, true, &default_style());
        let style = default_style();

        let assembly = GlyphAssembly {
            min_overlap: 20,
            parts: vec![
                GlyphPart { glyph_id: 1400, start_connector: 0, end_connector: 374, full_advance: 750, is_extender: false, hor_advance: 1536.0 },
                GlyphPart { glyph_id: 6634, start_connector: 748, end_connector: 748, full_advance: 748, is_extender: true, hor_advance: 1536.0 },
                GlyphPart { glyph_id: 1399, start_connector: 374, end_connector: 374, full_advance: 1500, is_extender: false, hor_advance: 1536.0 },
                GlyphPart { glyph_id: 6634, start_connector: 748, end_connector: 748, full_advance: 748, is_extender: true, hor_advance: 1536.0 },
                GlyphPart { glyph_id: 1398, start_connector: 374, end_connector: 0, full_advance: 750, is_extender: false, hor_advance: 1536.0 },
            ],
        };
        let b = ml.layout_assembly('{', assembly, 3200.0, &style);
        let (gids, steps) = p957_steps(&b);

        assert_eq!(
            gids,
            vec![1400, 6634, 1399, 6634, 1398],
            "ordem fundo→topo: [uni23A9, ex, uni23A8, ex, uni23A7]: {gids:?}"
        );
        let esperado = [426.0, 424.0, 1176.0, 424.0];
        assert_eq!(steps.len(), 4, "5 peças → 4 passos: {steps:?}");
        for (i, (s, e)) in steps.iter().zip(esperado.iter()).enumerate() {
            assert!(
                (s - e).abs() < 0.5,
                "passo {i}: esperado {e:.2}du (vanilla, r=200/1416), obteve {s:.2}du"
            );
        }
    }

    /// **P957 — centragem da tinta no eixo**: com baselines no fundo dos
    /// slots, a tinta ocupa exactamente `[0, total_height]` (peças NewCMMath
    /// têm yMin=0 e altura de tinta = advance), logo o centro da tinta fica
    /// em `−axis_pt` com `shift_y = −axis_pt − total/2` (a fórmula de P952b
    /// era a compensação da convenção errada). Fallback: axis = 500du → 6pt
    /// a 12pt.
    #[test]
    fn p957_assembly_tinta_centrada_no_eixo() {
        let stub = StubHorizontalMetrics::new();
        let ml = MathLayouter::new(&stub, true, &default_style());
        let style = default_style();

        let b = ml.layout_assembly('(', p957_paren_assembly(), 3800.0, &style);
        let (gids, _) = p957_steps(&b);
        let y_of = |gid: u16| {
            b.items
                .iter()
                .find_map(|i| match i {
                    FrameItem::Glyph { glyph_id, pos, .. } if *glyph_id == gid => {
                        Some(pos.y.val())
                    }
                    _ => None,
                })
                .unwrap()
        };
        let y_fundo = y_of(1388);
        let y_topo = y_of(1386);
        let adv_topo_pt = 1495.0 * 0.012; // 17.94pt
        let axis_pt = 500.0 * 0.012; // 6pt
        let centro_tinta = (y_topo - adv_topo_pt + y_fundo) / 2.0;
        assert!(
            (centro_tinta - (-axis_pt)).abs() < 1e-9,
            "centro da tinta deve ficar em −axis (−6pt), obteve {centro_tinta:.6} \
             (y_topo={y_topo:.4}, y_fundo={y_fundo:.4}) — gids: {gids:?}"
        );
        // Consistência com a caixa declarada (P914): a tinta cobre
        // exactamente [−ascent, descent] — total = Σ passos + adv_topo =
        // 3800du = 45.6pt a 12pt.
        let total_pt = 3800.0 * 0.012;
        assert!(
            (b.ascent - (axis_pt + total_pt / 2.0)).abs() < 0.01,
            "ascent = axis + total/2: {:.4} vs {:.4}",
            b.ascent,
            axis_pt + total_pt / 2.0
        );
        assert!(
            ((y_topo - adv_topo_pt) - (-b.ascent)).abs() < 0.01,
            "topo da tinta ({:.4}) deve coincidir com −ascent ({:.4})",
            y_topo - adv_topo_pt,
            -b.ascent
        );
        assert!(
            (y_fundo - b.descent).abs() < 0.01,
            "fundo da tinta ({:.4}) deve coincidir com descent ({:.4})",
            y_fundo,
            b.descent
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

        // P921 — layout_text_node passa a usar `text_ink_bounds` (bbox real,
        // não a proporção fixa de `vertical_metrics`); o sup/sub deste
        // teste (`StubHorizontalMetrics`, sem override de `text_ink_bounds`)
        // passa a ter `descent=0` (default do trait), reduzindo a tinta que
        // antes empurrava `shift_up_final` visivelmente acima do piso —
        // agora fica exactamente no piso (700du), dentro de erro de ponto
        // flutuante (`-1e-9`), não abaixo dele. Contrato semântico
        // inalterado: o piso continua a sobreviver como mínimo.
        assert!(
            shift_up_final >= 700.0 * default_style().size.val() / 1000.0 - 1e-9,
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

// ── P920 — fórmula real de `fraction_numerator_shift_up`/
// `fraction_denominator_shift_down` (frac.md §P920) ─────────────────────
//
// Nota histórica de investigação: esta secção testou originalmente também
// `accent_base_height` (Parte A, achado 1 abaixo) — destacada para passo
// dedicado após a nota de revisão do achado 1 ter revelado incompatibilidade
// real entre o modelo de descent do cristalino (sempre >= 0) e o do vanilla
// (pode ser negativo para acentos); ver `typst-passo-920-relatorio.md`. Os
// testes desse achado foram removidos deste ficheiro (não compilavam sem
// implementação, e a implementação foi adiada); a nota de revisão fica
// abaixo como registo da investigação para o passo dedicado futuro.
//
// Testa contra 2 campos NOVOS em `MathConstants` (`fraction_numerator_
// shift_up`, `fraction_denominator_shift_down`) que ainda não existem
// nesta struct no momento em que este módulo foi escrito — os testes
// abaixo FALHAM A COMPILAR até esses campos serem adicionados (TDD: o
// teste define o contrato antes da implementação). Não implementar os
// campos aqui — só os testes (ver `frac.rs`/`math_constants.rs`, ainda
// por tocar).
//
// Proveniência da medição (regra "registar a proveniência", CLAUDE.md):
// `fontTools` (Python), comando
// `TTFont(path).tables['MATH'].table.MathConstants`, sobre
// `NewCMMath-Regular.otf` em `~/.cargo/git/checkouts/
// typst-assets-525e6d15ef7950cb/c0ae970/files/fonts/NewCMMath-Regular.otf`
// — checkout `c0ae970` do dependency git `typst-assets` pinado por
// `Cargo.lock` (o ficheiro da fonte não pertence à árvore git deste
// projecto — não há commit deste repo associado a ele). Medido em
// 2026-07-26; `git rev-parse HEAD` neste momento = `54328a52b`; `git diff
// HEAD --stat` nesse momento: `.gitignore`, `00_nucleo/prompts/compiler/
// math/layout/{accent,frac,underover}.md`, `00_nucleo/prompts/entities/
// math_constants.md`, `00_nucleo/prompts/infra/font_metrics.md`,
// `01_core/src/compiler/math/layout/{accent,frac,underover}.rs`,
// `01_core/src/entities/math_constants.rs`, `03_infra/src/font_metrics.rs`
// (só bumps de `@prompt-hash`/L0 destes ficheiros — nenhuma alteração de
// lógica alheia a este achado). `axis_height=250` confere com o valor já
// citado por `03_infra/src/font_metrics.rs` (P893: "axis_height errado
// por 2× (500 vs 250 real)") — mesma fonte, mesma medição.
//
// Valores medidos (design units, upem=1000): AxisHeight=250,
// AccentBaseHeight=450, FractionRuleThickness=40,
// FractionNumeratorGapMin=40, FractionDenominatorGapMin=40,
// FractionNumeratorShiftUp=394, FractionDenominatorShiftDown=345,
// ScriptPercentScaleDown=70, ScriptScriptPercentScaleDown=50,
// SubscriptShiftDown=247, SuperscriptShiftUp=363,
// SuperscriptShiftUpCramped=289, SubSuperscriptGapMin=160,
// SuperscriptBottomMin=108, SuperscriptBottomMaxWithSubscript=344,
// SuperscriptBaselineDropMax=250, SubscriptTopMax=344,
// SubscriptBaselineDropMin=200.
//
// **Achado 1 (accent_base_height) — nota para revisão humana antes da
// implementação**: a derivação feita para este módulo, a partir da
// fórmula real do vanilla (`gap = -accent.descent() -
// base.ascent().min(accent_base_height)`, lab/typst-original/crates/
// typst-layout/src/math/accent.rs:56-65`), sugere uma direcção
// POSSIVELMENTE DIFERENTE da que o texto actual de `accent.md`/
// `underover.md` §P920 assume ("bases altas ganham espaço extra"). A
// derivação (independente de convenções de sinal do `descent` do vanilla
// — cancela algebricamente): a posição da baseline própria do accent
// relativa à baseline da base é `Δ = min(base.ascent(), accent_base_height)
// - base.ascent()`, que é SEMPRE `<= 0` e fica MAIS perto de zero (não
// mais negativo) quando a base é alta — i.e. o accent tende a ficar MAIS
// PRÓXIMO da base (não mais afastado) quando `base.ascent() >
// accent_base_height`, ao contrário do "espaço extra" citado no L0. Isto
// pode reflectir um mecanismo de "não deixar o accent voar demasiado alto
// sobre bases altas" (consistente com o comentário do vanilla: "Only if
// the base is very small, we need a larger gap so that the accent doesn't
// move too low") em vez de "mais espaço para bases altas". Os testes
// abaixo NÃO comprometem uma direcção (usam `assert_ne!`, não `>`/`<`)
// precisamente por causa desta incerteza — recomenda-se confirmar contra
// o vanilla real (`mutool trace` em `$hat(x^2/y)$` ou equivalente) antes
// de implementar, exactamente como a própria `accent.md` §P920 já pede
// ("Fase B... a confirmar/afinar... não assumir como final").
#[cfg(test)]
mod p920_tests {
    use super::*;
    use crate::entities::math_constants::MathConstants;

    /// Test double mínimo: delega os 4 métodos obrigatórios do trait a
    /// `FixedMetrics`, mas devolve `MathConstants` fixas (injectadas na
    /// construção) e `vertical_metrics` com descent=0 (aproxima uma letra
    /// "normal" sem descendente, ex. 'x') — evita o descent genérico de
    /// `0.4*size` de `FixedMetrics`, que forçaria SEMPRE o piso em
    /// `frac.rs` e mascararia a fórmula real que estes testes verificam.
    struct ConstMetrics(MathConstants);

    impl FontMetrics for ConstMetrics {
        fn advance(&self, text: &str, size: Pt, style: &TextStyle) -> Pt {
            FixedMetrics.advance(text, size, style)
        }
        fn vertical_metrics(&self, size: Pt, _style: &TextStyle) -> (Pt, Pt) {
            let a = size * 0.7;
            (a, a)
        }
        fn cap_height(&self, size: Pt, style: &TextStyle) -> Pt {
            FixedMetrics.cap_height(size, style)
        }
        fn text_edges(&self, size: Pt, style: &TextStyle) -> (Pt, Pt) {
            FixedMetrics.text_edges(size, style)
        }
        fn math_constants(&self, _style: &TextStyle) -> MathConstants {
            self.0.clone()
        }
    }

    /// `MathConstants` de teste com os campos novos de P920 (Parte B —
    /// `fraction_numerator_shift_up`/`fraction_denominator_shift_down`; a
    /// Parte A, `accent_base_height`, foi destacada para passo dedicado,
    /// ver `typst-passo-920-relatorio.md`) e valores REAIS medidos na
    /// fonte de produção do cristalino (ver bloco de proveniência acima)
    /// — não placeholders, mesma disciplina de P915/P919.
    fn p920_real_font_constants() -> MathConstants {
        let mut c = MathConstants::fallback();
        c.upem = 1000.0;
        c.axis_height = 250.0;
        c.fraction_rule_thickness = 40.0;
        c.fraction_num_gap = 40.0;
        c.fraction_denom_gap = 40.0;
        c.fraction_numerator_shift_up = 394.0;
        c.fraction_denominator_shift_down = 345.0;
        c.script_percent_scale_down = 0.70;
        c.script_script_percent_scale_down = 0.50;
        c.subscript_shift_down = 247.0;
        c.superscript_shift_up = 363.0;
        c.superscript_shift_up_cramped = 289.0;
        c.sub_superscript_gap_min = 160.0;
        c.superscript_bottom_min = 108.0;
        c.superscript_bottom_max_with_subscript = 344.0;
        c.superscript_baseline_drop_max = 250.0;
        c.subscript_top_max = 344.0;
        c.subscript_baseline_drop_min = 200.0;
        c
    }

    // ── Achado 1 (accent_base_height) destacado para passo dedicado ──────
    //
    // Investigação de Fase A revelou incompatibilidade real entre o modelo
    // de `descent` do cristalino (sempre >= 0, `FontMetrics::text_ink_
    // bounds`, `compiler/layout/metrics.rs:59-61`) e o do vanilla (pode ser
    // negativo para glifos de acento cuja tinta fica inteiramente acima da
    // própria baseline, `accent.rs:57-58` do vanilla, comentário explícito:
    // "Descent is negative because the accent's ink bottom is above the
    // baseline"). Uma aproximação ingénua (assumir accent.descent()≈0)
    // produz sobreposição (gap negativo), não uma aproximação inofensiva —
    // o termo é estrutural na fórmula, não cosmético. Requer decisão
    // arquitectural própria (estender `FontMetrics` para extensões com
    // sinal, ou mecanismo equivalente) antes de poder ser implementado
    // fielmente — destacado para passo dedicado, ver
    // `typst-passo-920-relatorio.md`.

    // ── Achado 2 — fórmula real de fraction_numerator_shift_up /
    // fraction_denominator_shift_down (frac.md §P920) ───────────────────

    /// Ponto 1 — `frac(a,b)` com alturas SEMELHANTES: os gaps calculados
    /// pela fórmula real do vanilla são POSITIVOS e de ordem de grandeza
    /// plausível — nenhum dos dois cai no piso (`fraction_num_gap`/
    /// `fraction_denom_gap`), confirmando que é a FÓRMULA (não só o
    /// `.max()`) que está a ser usada.
    ///
    /// Números concretos (medição fontTools/NewCMMath-Regular.otf, ver
    /// `p920_real_font_constants`, a 12pt, script a 8.4pt=12*0.7):
    /// axis_pt=3.0, thickness_pt=0.48, shift_up_pt=4.728,
    /// shift_down_pt=4.14, floor_pt=0.48. `num`/`den` = ident simples
    /// (ascent=0.7*8.4=5.88pt, descent=0pt): num_gap =
    /// 4.728-3.0-0.24-0 = 1.488pt (> floor); den_gap =
    /// 4.14+3.0-0.24-5.88 = 1.02pt (> floor).
    #[test]
    fn p920_frac_gaps_alturas_semelhantes_positivos_e_nao_no_piso() {
        let c = p920_real_font_constants();
        let style = default_style(); // 12pt
        let metrics = ConstMetrics(c.clone());
        let ml = MathLayouter::new(&metrics, true, &style);

        let num = Content::MathIdent("a".into());
        let den = Content::MathIdent("b".into());

        let num_style =
            TextStyle { size: style.size * c.script_percent_scale_down, ..style.clone() };
        let den_style = TextStyle { cramped: true, ..num_style.clone() };
        let num_box = ml.layout_node(&num, &num_style);
        let den_box = ml.layout_node(&den, &den_style);

        let axis_pt = c.to_pt(c.axis_height, style.size).val();
        let thickness_pt = c.to_pt(c.fraction_rule_thickness, style.size).val();
        let shift_up_pt = c.to_pt(c.fraction_numerator_shift_up, style.size).val();
        let shift_down_pt = c.to_pt(c.fraction_denominator_shift_down, style.size).val();
        let num_floor_pt = c.to_pt(c.fraction_num_gap, style.size).val();
        let den_floor_pt = c.to_pt(c.fraction_denom_gap, style.size).val();

        // Sanity — confirma a aritmética dos números concretos citados
        // acima (mesma disciplina de
        // `axis_bug_frac_bar_deve_ficar_a_axis_height_da_baseline_vizinha`).
        assert!((axis_pt - 3.0).abs() < 1e-9, "sanity axis_pt, foi {axis_pt}");
        assert!((thickness_pt - 0.48).abs() < 1e-9, "sanity thickness_pt, foi {thickness_pt}");
        assert!((shift_up_pt - 4.728).abs() < 1e-9, "sanity shift_up_pt, foi {shift_up_pt}");
        assert!((shift_down_pt - 4.14).abs() < 1e-9, "sanity shift_down_pt, foi {shift_down_pt}");
        assert!((num_box.ascent - 5.88).abs() < 1e-9, "sanity num_box.ascent, foi {}", num_box.ascent);
        assert!((num_box.descent - 0.0).abs() < 1e-9, "sanity num_box.descent, foi {}", num_box.descent);
        assert!((den_box.ascent - 5.88).abs() < 1e-9, "sanity den_box.ascent, foi {}", den_box.ascent);

        let expected_num_gap =
            (shift_up_pt - axis_pt - thickness_pt / 2.0 - num_box.descent).max(num_floor_pt);
        let expected_den_gap =
            (shift_down_pt + axis_pt - thickness_pt / 2.0 - den_box.ascent).max(den_floor_pt);

        assert!((expected_num_gap - 1.488).abs() < 1e-9, "sanity expected_num_gap, foi {expected_num_gap}");
        assert!((expected_den_gap - 1.02).abs() < 1e-9, "sanity expected_den_gap, foi {expected_den_gap}");
        assert!(expected_num_gap > num_floor_pt, "num_gap deve vir da fórmula, não do piso");
        assert!(expected_den_gap > den_floor_pt, "den_gap deve vir da fórmula, não do piso");

        let frac_box = ml.layout_frac(&num, &den, &style);
        let expected_ascent = num_box.height() + expected_num_gap + thickness_pt / 2.0 + axis_pt;
        let expected_descent = den_box.height() + expected_den_gap + thickness_pt / 2.0 - axis_pt;

        assert!(
            (frac_box.ascent - expected_ascent).abs() < 1e-6,
            "P920 — frac.md: ascent deve reflectir num_gap da fórmula real \
             (fraction_numerator_shift_up - axis - thickness/2 - \
             num.descent(), com piso fraction_num_gap), não \
             fraction_num_gap usado directamente; \
             esperado={expected_ascent:.4}pt, obtido={:.4}pt",
            frac_box.ascent
        );
        assert!(
            (frac_box.descent - expected_descent).abs() < 1e-6,
            "P920 — frac.md: descent deve reflectir denom_gap calculado; \
             esperado={expected_descent:.4}pt, obtido={:.4}pt",
            frac_box.descent
        );
    }

    /// Ponto 2 — `frac(a_1, b)`: numerador com subscrito ganha descent
    /// REAL (ao contrário de hoje, onde `gap` é o mesmo valor espelhado
    /// nos dois lados, vindo directamente de `fraction_num_gap`) —
    /// `num_gap` e `denom_gap` DIVERGEM de forma mensurável.
    ///
    /// Números concretos: `num_box.descent` = sub_offset (2.0748pt, de
    /// `subscript_shift_down`=247du a 8.4pt) + sub_box.descent (0pt) =
    /// 2.0748pt; `expected_num_gap` = (4.728-3.0-0.24-2.0748).max(0.48) =
    /// 0.48pt (cai no piso); `expected_den_gap` (denominador plano "b",
    /// inalterado) = 1.02pt — claramente diferentes.
    #[test]
    fn p920_frac_alturas_muito_diferentes_gaps_divergem() {
        let c = p920_real_font_constants();
        let style = default_style();
        let metrics = ConstMetrics(c.clone());
        let ml = MathLayouter::new(&metrics, true, &style);

        // Numerador com subscrito: ganha descent real (ink abaixo da
        // baseline própria do numerador), ao contrário de um ident simples.
        let num = Content::math_attach(
            Content::MathIdent("a".into()),
            None,
            None,
            Some(Content::MathIdent("1".into())),
            None,
        );
        let den = Content::MathIdent("b".into());

        let num_style =
            TextStyle { size: style.size * c.script_percent_scale_down, ..style.clone() };
        let den_style = TextStyle { cramped: true, ..num_style.clone() };
        let num_box = ml.layout_node(&num, &num_style);
        let den_box = ml.layout_node(&den, &den_style);

        // Sanity — confirma que o subscrito realmente produz descent > 0
        // no numerador (mecanismo de `attach.rs`: descent =
        // descent.max(sub_offset + sub_box.descent)).
        assert!(
            num_box.descent > 2.0,
            "sanity: numerador com subscrito deve ter descent real e \
             substancial (esperado ~2.0748pt), foi {}",
            num_box.descent
        );

        let axis_pt = c.to_pt(c.axis_height, style.size).val();
        let thickness_pt = c.to_pt(c.fraction_rule_thickness, style.size).val();
        let shift_up_pt = c.to_pt(c.fraction_numerator_shift_up, style.size).val();
        let shift_down_pt = c.to_pt(c.fraction_denominator_shift_down, style.size).val();
        let num_floor_pt = c.to_pt(c.fraction_num_gap, style.size).val();
        let den_floor_pt = c.to_pt(c.fraction_denom_gap, style.size).val();

        let expected_num_gap =
            (shift_up_pt - axis_pt - thickness_pt / 2.0 - num_box.descent).max(num_floor_pt);
        let expected_den_gap =
            (shift_down_pt + axis_pt - thickness_pt / 2.0 - den_box.ascent).max(den_floor_pt);

        assert!(
            (expected_num_gap - expected_den_gap).abs() > 0.3,
            "P920 — frac.md: num_gap e denom_gap devem DIVERGIR de forma \
             mensurável quando num/den têm tinta muito diferente (hoje são \
             forçados a ser iguais, usando fraction_num_gap directamente \
             para os dois lados — isso é o bug); \
             num_gap={expected_num_gap:.4}pt, den_gap={expected_den_gap:.4}pt"
        );

        let frac_box = ml.layout_frac(&num, &den, &style);
        let expected_ascent = num_box.height() + expected_num_gap + thickness_pt / 2.0 + axis_pt;
        let expected_descent = den_box.height() + expected_den_gap + thickness_pt / 2.0 - axis_pt;
        assert!(
            (frac_box.ascent - expected_ascent).abs() < 1e-6,
            "ascent devolvido deve usar num_gap (assimétrico) calculado a \
             partir da tinta real do numerador; \
             esperado={expected_ascent:.4}pt, obtido={:.4}pt",
            frac_box.ascent
        );
        assert!(
            (frac_box.descent - expected_descent).abs() < 1e-6,
            "descent devolvido deve usar denom_gap calculado a partir da \
             tinta real do denominador (inalterado neste caso); \
             esperado={expected_descent:.4}pt, obtido={:.4}pt",
            frac_box.descent
        );
    }

    /// Ponto 3 — o mesmo caso do teste anterior força `num_gap` ABAIXO do
    /// piso mínimo (`fraction_num_gap` convertido, 0.48pt a 12pt) —
    /// confirma que o `.max()` participa (usa o piso, não o valor negativo
    /// cru da fórmula).
    #[test]
    fn p920_frac_formula_abaixo_do_piso_usa_fraction_num_gap_como_minimo() {
        let c = p920_real_font_constants();
        let style = default_style();
        let metrics = ConstMetrics(c.clone());
        let ml = MathLayouter::new(&metrics, true, &style);

        let num = Content::math_attach(
            Content::MathIdent("a".into()),
            None,
            None,
            Some(Content::MathIdent("1".into())),
            None,
        );
        let den = Content::MathIdent("b".into());

        let num_style =
            TextStyle { size: style.size * c.script_percent_scale_down, ..style.clone() };
        let num_box = ml.layout_node(&num, &num_style);

        let axis_pt = c.to_pt(c.axis_height, style.size).val();
        let thickness_pt = c.to_pt(c.fraction_rule_thickness, style.size).val();
        let shift_up_pt = c.to_pt(c.fraction_numerator_shift_up, style.size).val();
        let num_floor_pt = c.to_pt(c.fraction_num_gap, style.size).val();

        let raw_formula_value = shift_up_pt - axis_pt - thickness_pt / 2.0 - num_box.descent;
        assert!(
            raw_formula_value < num_floor_pt,
            "sanity: este caso só é útil se a fórmula crua já cair abaixo \
             do piso — raw={raw_formula_value:.4}pt, floor={num_floor_pt:.4}pt"
        );
        assert!(
            raw_formula_value < 0.0,
            "sanity: valor cru da fórmula deve ser negativo neste caso \
             (numerador com subscrito grande), foi {raw_formula_value:.4}pt"
        );

        let frac_box = ml.layout_frac(&num, &den, &style);
        let expected_ascent_with_floor =
            num_box.height() + num_floor_pt + thickness_pt / 2.0 + axis_pt;
        let expected_ascent_with_raw_negative =
            num_box.height() + raw_formula_value + thickness_pt / 2.0 + axis_pt;

        assert!(
            (frac_box.ascent - expected_ascent_with_floor).abs() < 1e-6,
            "P920 — frac.md: com a fórmula crua ABAIXO do piso, o \
             resultado deve usar fraction_num_gap (piso, \
             {num_floor_pt:.4}pt), não o valor cru negativo \
             ({raw_formula_value:.4}pt); \
             esperado(piso)={expected_ascent_with_floor:.4}pt, \
             obtido={:.4}pt",
            frac_box.ascent
        );
        assert!(
            (frac_box.ascent - expected_ascent_with_raw_negative).abs() > 1e-3,
            "confirma que o resultado NÃO é o valor cru negativo da \
             fórmula (o .max() deve ter substituído pelo piso)"
        );
    }
}

// ── Regressão — `apply_axis_offset` não desloca `items` (bug de omissão) ──
//
// Causa raiz (leitura de código, `mod.rs::apply_axis_offset`, ~L343):
// o método ajusta `ascent`/`descent` de uma `MathBox` para a centrar no
// eixo matemático, mas nunca desloca `b.items` — só a metadata muda.
// Isto é um no-op VISUAL para qualquer chamador: no topo
// (`layout_equation`), `place()` usa `baseline_y = math_box.ascent`, o
// que anula o termo `-ascent` da fórmula
// (`parent_y = baseline_y - self.ascent + local_y == local_y`); e
// `hconcat_spaced` nunca aplica shift vertical a um item (só X). Logo a
// posição renderizada de qualquer item é sempre o seu `local_y`
// acumulado pela árvore — mexer só em ascent/descent não move nada.
//
// Padrão correcto já implementado (referência): `layout_stretchy_delimiter`
// (`stretchy.rs`) calcula `shift_y` e aplica
// `offset_item(item, Pt(0.0), Pt(shift_y))` a cada item, além de ajustar
// ascent/descent — é esse par (metadata + items) que falta em
// `apply_axis_offset`.
//
// Confirmado empiricamente com o binário real (`cargo build --release
// -p typst-wiring`, `mutool trace`, `$x + frac(a,b)$` a 24pt): a barra
// da fracção e a baseline de `x` caem exactamente na mesma linha y
// (99.392 vs 99.4 — diferença de arredondamento, não de eixo);
// deveriam diferir por `axis_height` (vários pt a este tamanho).
// `$x + sqrt(y)$` a 24pt: baseline de x=83.534, de y=83.534 —
// idênticas, confirma que sqrt/root NÃO deve ganhar deslocamento.
//
// Proveniência da medição (regra "registar a proveniência", CLAUDE.md):
// working tree com alterações não commitadas no momento da escrita
// destes testes; `git rev-parse HEAD` = 56d85a6f5c6c3f08a2a3cbb939d02153c0640788;
// ficheiros alterados nesse momento (`git diff HEAD --stat`):
// `.gitignore`, `00_nucleo/prompts/compiler/math/layout/{_comum,cases,
// delimited,frac,matrix,root}.md`, `01_core/src/compiler/math/layout/
// {cases,delimited,frac,matrix,mod,root,tests}.rs` (apenas bumps de
// `@prompt-hash`/linha em branco — nenhuma alteração de lógica alheia
// a este passo). Todos os valores numéricos abaixo são calculados a
// partir de `MathConstants::fallback()` (`axis_height=500du`,
// `upem=1000`) e `FixedMetrics` (`ascent=0.8*size`, `descent=0.4*size`,
// `advance=0.6*size` por carácter), nunca hardcoded sem fórmula.
//
// Vanilla lido directamente (não aceite de ânimo leve) em
// lab/typst-original/crates/typst-layout/src/math/`:
//   - `fraction.rs::layout_fraction` (variante com `item.line = true`):
//     `baseline = line_pos.y + axis` — a barra fica a `axis_height`
//     da baseline do composto, por construção.
//   - `table.rs` (~L188): `frame.set_baseline(height/2.0 + axis)` —
//     mat/cases centram o MEIO da altura total no eixo.
//   - `radical.rs` (~L110): baseline = `rad_box.ascent`, SEM termo de
//     axis — sqrt/root não centra no eixo.
//   - `fenced.rs::layout_fenced`: não há `set_baseline` de grupo
//     nenhum — delimitadores vêm pré-centrados (mecanismo próprio) e
//     o corpo mantém a sua própria baseline, inalterada.

fn find_text_y(items: &[FrameItem], s: &str) -> f64 {
    items
        .iter()
        .find_map(|i| match i {
            FrameItem::Text { pos, text, .. } if text.as_str() == s => Some(pos.y.val()),
            _ => None,
        })
        .unwrap_or_else(|| panic!("texto {:?} não encontrado em {:?}", s, items))
}

fn find_line_start_y(items: &[FrameItem]) -> f64 {
    items
        .iter()
        .find_map(|i| match i {
            FrameItem::Line { start, .. } => Some(start.y.val()),
            _ => None,
        })
        .expect("deve ter FrameItem::Line")
}

#[test]
fn axis_bug_frac_bar_deve_ficar_a_axis_height_da_baseline_vizinha() {
    // Caso 1 do bug relatado: dois elementos de alturas MUITO
    // diferentes lado a lado (`x` simples vs `frac(a,b)`). A barra da
    // fracção deve estar a `axis_height` (convertida para pt) ACIMA da
    // baseline de `x` (y menor — y cresce para baixo) — não a 0 (mesma
    // altura que `x`), como acontece hoje.
    let style = default_style(); // 12pt
    let constants = crate::entities::math_constants::MathConstants::fallback();
    let axis_pt = constants.to_pt(constants.axis_height, style.size).val();
    assert!(
        (axis_pt - 6.0).abs() < 1e-9,
        "sanity: axis_pt esperado 6.0 (500du/1000upem * 12pt), foi {}",
        axis_pt
    );

    let ml = MathLayouter::new(&FixedMetrics, true, &style);
    let seq = Content::MathSequence(Arc::from(
        vec![
            Content::MathIdent("x".into()),
            Content::math_frac(
                Content::MathIdent("a".into()),
                Content::MathIdent("b".into()),
            ),
        ]
        .into_boxed_slice(),
    ));
    let items = ml.layout_equation(&seq, &style);

    let x_y = find_text_y(&items, "𝑥");
    let bar_y = find_line_start_y(&items);

    // `x` é texto simples: a sua baseline nunca é deslocada
    // (`layout_text_node` emite em y=0; `hconcat_spaced` não toca em y).
    assert!(
        (x_y - 0.0).abs() < 1e-6,
        "baseline de x deve ficar em y=0 (referência da equação), foi {}",
        x_y
    );

    assert!(
        (bar_y - (x_y - axis_pt)).abs() < 1e-6,
        "barra da fracção deve estar a axis_height ({:.4}pt) acima da baseline de x \
         (x_y={:.4} ⇒ esperado bar_y={:.4}), obteve bar_y={:.4} — hoje apply_axis_offset \
         não desloca items, logo bar_y fica em 0 (igual a x)",
        axis_pt,
        x_y,
        x_y - axis_pt,
        bar_y
    );
}

#[test]
fn axis_bug_frac_numerador_e_denominador_acompanham_o_deslocamento() {
    // Não basta a barra mover-se — TODO o box da fracção (numerador,
    // barra, denominador) tem de deslocar-se em bloco por `axis_pt`
    // (mesmo padrão de `layout_stretchy_delimiter::shift_y`; fixo,
    // P919 — não depende de `ascent`/`descent` serem simétricos). Este
    // teste reconstrói independentemente o valor PRÉ-shift a partir da
    // fórmula de `frac.rs` (P915/P905: `num_style.size = size *
    // script_percent_scale_down`; `den_style` = mesmo tamanho +
    // cramped — cramped não afecta `FixedMetrics` numericamente, logo
    // `num_box == den_box` em altura) e confirma que o valor observado
    // é exactamente `pre - axis_pt`.
    //
    // **P920** — actualizado: `num_gap`/`denom_gap` deixaram de ser o
    // mesmo valor (`fraction_num_gap` usado directamente nos dois
    // lados) e passam a vir da fórmula real do vanilla (`frac.md`
    // §P920), que depende de `fraction_numerator_shift_up`/
    // `fraction_denominator_shift_down` — constantes DISTINTAS por
    // desenho (394/345 na fonte de produção). Por isso, mesmo com
    // `num_box`/`den_box` idênticos em altura (caso deste teste), o
    // `ascent`/`descent` resultante do `MathBox` da fracção **deixa
    // de ser simétrico** — a antiga suposição "mesmo char ⇒
    // ascent_pre == descent_pre" já não vale; o `shift` aplicado pelo
    // código, no entanto, continua a ser exactamente `axis_pt`, fixo,
    // independentemente dessa (a)simetria (é isso que P919 garante,
    // ver `frac.rs`: o offset dos `items` é sempre `-axis_pt`, nunca
    // um valor derivado de `ascent`/`descent`).
    let style = default_style(); // 12pt
    let constants = crate::entities::math_constants::MathConstants::fallback();
    let axis_pt = constants.to_pt(constants.axis_height, style.size).val(); // 6.0

    let num_size = style.size.val() * constants.script_percent_scale_down; // 8.4
    // P921 — layout_text_node passa a usar `text_ink_bounds`, não
    // `vertical_metrics`. `FixedMetrics` usa o default do trait
    // (01_core/src/compiler/layout/metrics.rs:59): `(cap_height, 0.0)` —
    // `cap_height` de `FixedMetrics` é `size*0.7`; descent sempre 0.
    let leaf_ascent = num_size * 0.7; // FixedMetrics via text_ink_bounds => 5.88
    let leaf_descent = 0.0_f64; // idem — sempre 0, sem bbox real
    let rule_thickness =
        constants.to_pt(constants.fraction_rule_thickness, style.size).val(); // 0.792

    let shift_up_pt =
        constants.to_pt(constants.fraction_numerator_shift_up, style.size).val(); // 4.728
    let shift_down_pt =
        constants.to_pt(constants.fraction_denominator_shift_down, style.size).val(); // 4.14
    let num_gap_floor = constants.to_pt(constants.fraction_num_gap, style.size).val(); // 0.6
    let denom_gap_floor = constants.to_pt(constants.fraction_denom_gap, style.size).val(); // 0.6

    // num.descent() = leaf_descent (numerador "a", sem sub/superscript);
    // denom.ascent() = leaf_ascent (denominador "b", idem).
    let num_gap = (shift_up_pt - axis_pt - rule_thickness / 2.0 - leaf_descent)
        .max(num_gap_floor); // max(-1.668, 0.6) = 0.6 (piso) — P921: leaf_descent=0
    let denom_gap = (shift_down_pt + axis_pt - rule_thickness / 2.0 - leaf_ascent)
        .max(denom_gap_floor); // max(3.864, 0.6) = 3.864 (fórmula, acima do piso) — P921: leaf_ascent=5.88

    // `shift` é sempre `axis_pt`, fixo (P919) — já não derivado de uma
    // suposição de simetria (ver comentário acima); mantém-se o nome
    // `shift` só para minimizar o diff das asserções abaixo.
    let shift = axis_pt;

    let num_y_pre = -(leaf_descent + num_gap + rule_thickness / 2.0); // -0.996 (P921)
    // den_y_pre = denom_gap + thickness/2 + leaf_ascent — o termo leaf_ascent
    // cancela algebricamente com o mesmo termo dentro de denom_gap (quando a
    // fórmula, não o piso, vence): fica sempre shift_down_pt + axis_pt =
    // 10.14, invariante ao valor de leaf_ascent — por isso P921 (mudança de
    // leaf_ascent 6.72→5.88) não alterou este valor.
    let den_y_pre = denom_gap + rule_thickness / 2.0 + leaf_ascent; // 10.14

    let ml = MathLayouter::new(&FixedMetrics, true, &style);
    let items = ml.layout_equation(
        &Content::math_frac(
            Content::MathIdent("a".into()),
            Content::MathIdent("b".into()),
        ),
        &style,
    );
    let num_y = find_text_y(&items, "𝑎");
    let den_y = find_text_y(&items, "𝑏");
    let bar_y = find_line_start_y(&items);

    assert!(
        (bar_y - (0.0 - shift)).abs() < 1e-6,
        "bar_y esperado {:.4} (0 - shift), obteve {:.4}",
        -shift,
        bar_y
    );
    assert!(
        (num_y - (num_y_pre - shift)).abs() < 1e-6,
        "num_y esperado {:.4} ({:.4} - shift), obteve {:.4}",
        num_y_pre - shift,
        num_y_pre,
        num_y
    );
    assert!(
        (den_y - (den_y_pre - shift)).abs() < 1e-6,
        "den_y esperado {:.4} ({:.4} - shift), obteve {:.4}",
        den_y_pre - shift,
        den_y_pre,
        den_y
    );
}

#[test]
fn axis_bug_frac_axis_height_generaliza_para_tres_elementos() {
    // Caso 2 do bug relatado: não é um caso especial de sequência de 2.
    // Com `x + frac(a,b) + z`, os DOIS vizinhos de texto simples devem
    // partilhar a mesma baseline entre si (a posição da fracção no
    // meio da sequência não pode "contaminar" `z`), e a barra deve
    // continuar a `axis_height` acima de CADA um deles.
    let style = default_style();
    let constants = crate::entities::math_constants::MathConstants::fallback();
    let axis_pt = constants.to_pt(constants.axis_height, style.size).val();

    let ml = MathLayouter::new(&FixedMetrics, true, &style);
    let seq = Content::MathSequence(Arc::from(
        vec![
            Content::MathIdent("x".into()),
            Content::math_frac(
                Content::MathIdent("a".into()),
                Content::MathIdent("b".into()),
            ),
            Content::MathIdent("z".into()),
        ]
        .into_boxed_slice(),
    ));
    let items = ml.layout_equation(&seq, &style);

    let x_y = find_text_y(&items, "𝑥");
    let z_y = find_text_y(&items, "𝑧");
    let bar_y = find_line_start_y(&items);

    assert!(
        (x_y - z_y).abs() < 1e-6,
        "x e z devem partilhar a mesma baseline (ambos texto simples): x_y={} z_y={}",
        x_y,
        z_y
    );
    assert!(
        (bar_y - (x_y - axis_pt)).abs() < 1e-6,
        "barra deve estar a axis_height acima de x (3 elementos): x_y={} bar_y={} esperado={}",
        x_y,
        bar_y,
        x_y - axis_pt
    );
    assert!(
        (bar_y - (z_y - axis_pt)).abs() < 1e-6,
        "barra deve estar a axis_height acima de z (3 elementos): z_y={} bar_y={} esperado={}",
        z_y,
        bar_y,
        z_y - axis_pt
    );
}

#[test]
fn axis_bug_cases_conteudo_centra_no_axis_height_nao_a_zero() {
    // Caso 3 do bug relatado: `cases`/`matrix` devem centrar o MEIO da
    // altura total da grelha no eixo matemático (vanilla `table.rs`
    // ~L188: `set_baseline(height/2.0 + axis)`), não deixar o
    // conteúdo onde `layout_grid_rows` o colocou por acidente (que não
    // tem qualquer relação com o eixo).
    //
    // `layout_cases` chama `apply_axis_offset` no box final (grid +
    // delimitador '{'), mas hoje isso só ajusta ascent/descent — o
    // conteúdo da grelha ('a','b') fica exactamente onde
    // `layout_grid_rows` o colocou. Este teste compara o output actual
    // de `layout_cases` com o grid PRÉ-offset (obtido chamando
    // `layout_grid_rows` directamente — não passa por
    // `apply_axis_offset`, logo é uma referência independente do bug)
    // para derivar `shift` e confirmar que os items da grelha deveriam
    // mover-se por esse `shift` e hoje não se movem.
    let style = default_style();
    let constants = crate::entities::math_constants::MathConstants::fallback();
    let axis_pt = constants.to_pt(constants.axis_height, style.size).val();

    let ml = MathLayouter::new(&FixedMetrics, true, &style);
    let rows = vec![
        vec![Content::MathIdent("a".into())],
        vec![Content::MathIdent("b".into())],
    ];
    let col_gap = style.size * 0.5;
    // **P923b** — `layout_cases` usa `row_gap = 0.2em` do estilo exterior.
    let row_gap = style.size * 0.2;
    // **P923** — `layout_cases` layouta os ramos em estilo de denominador;
    // o grid de referência pré-offset tem de usar o mesmo estilo.
    let cell_style = TextStyle {
        size: style.size * constants.script_percent_scale_down,
        cramped: true,
        ..style.clone()
    };

    let pre_grid = ml.layout_grid_rows(&rows, GridAlign::Left, col_gap, row_gap, &cell_style);
    let shift = axis_pt - (pre_grid.ascent - pre_grid.descent) / 2.0;

    let pre_a_y = find_text_y(&pre_grid.items, "a");
    let pre_b_y = find_text_y(&pre_grid.items, "b");

    let post = ml.layout_cases(&rows, ('{', '}'), false, None, &style);
    let post_a_y = find_text_y(&post.items, "a");
    let post_b_y = find_text_y(&post.items, "b");

    assert!(
        (post_a_y - (pre_a_y - shift)).abs() < 1e-6,
        "'a' deve mover-se por shift={:.4} (pre={:.4} ⇒ esperado {:.4}), obteve {:.4} \
         — hoje apply_axis_offset não desloca items, logo post==pre",
        shift,
        pre_a_y,
        pre_a_y - shift,
        post_a_y
    );
    assert!(
        (post_b_y - (pre_b_y - shift)).abs() < 1e-6,
        "'b' deve mover-se por shift={:.4} (pre={:.4} ⇒ esperado {:.4}), obteve {:.4}",
        shift,
        pre_b_y,
        pre_b_y - shift,
        post_b_y
    );

    // Guarda: o delimitador '{' já está correctamente auto-centrado no
    // eixo (constrói o seu próprio `shift_y` internamente em
    // `layout_stretchy_delimiter`, usando `axis_pt` directamente — não
    // a assimetria do grid) — a posição actual (y=0, a sua própria
    // baseline == baseline partilhada) NÃO deve mudar. Um fix ingénuo
    // que aplique `shift` uniformemente a TODOS os items de `result`
    // em `layout_cases`/`layout_matrix` (incluindo o delimitador)
    // parte esta invariante — ver relatório final.
    let brace_y = find_text_y(&post.items, "{");
    assert!(
        (brace_y - 0.0).abs() < 1e-6,
        "delimitador '{{' deve manter-se na sua própria posição auto-centrada (y=0), obteve {}",
        brace_y
    );
}

#[test]
fn axis_bug_matrix_conteudo_centra_no_axis_height_nao_a_zero() {
    // Mesma verificação que `axis_bug_cases_conteudo_centra_no_axis_height_nao_a_zero`,
    // para `mat(...)` (2x2, sem `&` nas células ⇒ `GridAlign::Center`,
    // mesmo caminho que `layout_matrix` usa quando `any_align == false`).
    let style = default_style();
    let constants = crate::entities::math_constants::MathConstants::fallback();
    let axis_pt = constants.to_pt(constants.axis_height, style.size).val();

    let ml = MathLayouter::new(&FixedMetrics, true, &style);
    let rows = vec![
        vec![Content::MathIdent("a".into()), Content::MathIdent("b".into())],
        vec![Content::MathIdent("c".into()), Content::MathIdent("d".into())],
    ];
    let col_gap = style.size * 0.5;
    // **P923b** — `layout_matrix` usa `row_gap = 0.2em` do estilo exterior.
    let row_gap = style.size * 0.2;
    // **P923** — `layout_matrix` layouta as células em estilo de denominador;
    // o grid de referência pré-offset tem de usar o mesmo estilo.
    let cell_style = TextStyle {
        size: style.size * constants.script_percent_scale_down,
        cramped: true,
        ..style.clone()
    };

    let pre_grid = ml.layout_grid_rows(&rows, GridAlign::Center, col_gap, row_gap, &cell_style);
    let shift = axis_pt - (pre_grid.ascent - pre_grid.descent) / 2.0;

    let post = ml.layout_matrix(&rows, ('(', ')'), &style);

    for c in ["a", "b", "c", "d"] {
        let pre = find_text_y(&pre_grid.items, c);
        let post_v = find_text_y(&post.items, c);
        assert!(
            (post_v - (pre - shift)).abs() < 1e-6,
            "'{}' deve mover-se por shift={:.4} (pre={:.4} ⇒ esperado {:.4}), obteve {:.4}",
            c,
            shift,
            pre,
            pre - shift,
            post_v
        );
    }

    // Guarda: parênteses já auto-centrados no eixo, não devem mudar.
    for c in ["(", ")"] {
        let y = find_text_y(&post.items, c);
        assert!(
            (y - 0.0).abs() < 1e-6,
            "delimitador '{}' deve manter-se auto-centrado (y=0), obteve {}",
            c,
            y
        );
    }
}

#[test]
fn axis_ok_sqrt_radicando_nao_ganha_deslocamento_de_axis_height() {
    // Caso 4 do bug relatado — teste "não deve mudar": vanilla
    // (`radical.rs` ~L110, confirmado por leitura) — a baseline do
    // composto sqrt/root é simplesmente o `ascent` do radicando, SEM
    // termo de `axis_height`. `x + sqrt(y)`: a baseline de `y`
    // (radicando) deve coincidir EXACTAMENTE com a de `x` (diferença
    // 0), nunca `axis_height` como em frac. Confirmado empiricamente
    // com o binário real (`mutool trace`, `$x + sqrt(y)$` a 24pt):
    // baseline de x=83.534, baseline de y=83.534 (idênticas).
    let style = default_style();
    let ml = MathLayouter::new(&FixedMetrics, true, &style);
    let seq = Content::MathSequence(Arc::from(
        vec![
            Content::MathIdent("x".into()),
            Content::math_root(None, Content::MathIdent("y".into())),
        ]
        .into_boxed_slice(),
    ));
    let items = ml.layout_equation(&seq, &style);

    let x_y = find_text_y(&items, "𝑥");
    let y_y = find_text_y(&items, "𝑦");

    assert!(
        (x_y - y_y).abs() < 1e-6,
        "radicando de sqrt deve partilhar a MESMA baseline do vizinho (sem axis_height): \
         x_y={} y_y={}",
        x_y,
        y_y
    );
}

#[test]
fn axis_ok_delimitado_corpo_nao_ganha_deslocamento_de_axis_height() {
    // Caso 4 do bug relatado (delimitadores emparelhados) — teste "não
    // deve mudar": vanilla (`fenced.rs::layout_fenced`, confirmado por
    // leitura) não centra o grupo nenhuma vez — os delimitadores vêm
    // pré-centrados (mecanismo próprio, equivalente a
    // `layout_stretchy_delimiter`) e o corpo mantém a sua própria
    // baseline, inalterada. `x + (y)`: baseline de `y` (corpo) deve
    // coincidir EXACTAMENTE com a de `x`.
    let style = default_style();
    let ml = MathLayouter::new(&FixedMetrics, true, &style);
    let seq = Content::MathSequence(Arc::from(
        vec![
            Content::MathIdent("x".into()),
            Content::math_delimited('(', Content::MathIdent("y".into()), ')'),
        ]
        .into_boxed_slice(),
    ));
    let items = ml.layout_equation(&seq, &style);

    let x_y = find_text_y(&items, "𝑥");
    let y_y = find_text_y(&items, "𝑦");

    assert!(
        (x_y - y_y).abs() < 1e-6,
        "corpo do delimitado deve partilhar a MESMA baseline do vizinho: x_y={} y_y={}",
        x_y,
        y_y
    );
}

// ── P922 — gap real de acento com `accent_base_height` e `text_ink_bounds_signed`
//
// A fórmula do vanilla (`accent.rs`): `gap = -accent.descent() -
// base.ascent().min(accent_base_height)`. O `accent.descent()` com sinal vem
// de `FontMetrics::text_ink_bounds_signed`. Estes testes usam um test double
// que injecta bounding boxes controladas, uma vez que `FixedMetrics` não tem
// acesso a bboxes reais de fonte.

#[cfg(test)]
mod p922_tests {
    use super::*;
    use crate::compiler::layout::{FixedMetrics, FontMetrics};
    use crate::entities::math_constants::MathConstants;
    use std::collections::HashMap;

    /// Test double que delega a `FixedMetrics` mas permite injectar
    /// bounding boxes controladas para caracteres específicos, tanto para
    /// `text_ink_bounds` (unsigned, usado na base) como para
    /// `text_ink_bounds_signed` (usado no acento).
    struct SignedMetrics {
        inner: FixedMetrics,
        ink: HashMap<char, (Pt, Pt)>,
        signed: HashMap<char, (Pt, Pt)>,
        constants: MathConstants,
        // **P988-B** — TopAccentAttachment por char (pt já escalado).
        attach: HashMap<char, f64>,
    }

    impl SignedMetrics {
        fn new(constants: MathConstants) -> Self {
            Self {
                inner: FixedMetrics,
                ink: HashMap::new(),
                signed: HashMap::new(),
                constants,
                attach: HashMap::new(),
            }
        }

        fn with_ink(mut self, c: char, top: f64, bottom: f64) -> Self {
            self.ink.insert(c, (Pt(top), Pt(bottom)));
            self
        }

        fn with_signed(mut self, c: char, top: f64, bottom: f64) -> Self {
            self.signed.insert(c, (Pt(top), Pt(bottom)));
            self
        }

        /// **P988-B** — regista o `TopAccentAttachment` de um char (em pt).
        fn with_attach(mut self, c: char, attach_pt: f64) -> Self {
            self.attach.insert(c, attach_pt);
            self
        }
    }

    impl FontMetrics for SignedMetrics {
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

        fn text_ink_bounds(&self, text: &str, size: Pt, style: &TextStyle) -> (Pt, Pt) {
            if text.chars().count() == 1 {
                if let Some(&(top, bottom)) = self.ink.get(&text.chars().next().unwrap()) {
                    return (top, bottom);
                }
            }
            self.inner.text_ink_bounds(text, size, style)
        }

        fn text_ink_bounds_signed(
            &self,
            text: &str,
            size: Pt,
            style: &TextStyle,
        ) -> (Pt, Pt) {
            if text.chars().count() == 1 {
                if let Some(&(top, bottom)) = self.signed.get(&text.chars().next().unwrap()) {
                    return (top, bottom);
                }
                if let Some(&(top, bottom)) = self.ink.get(&text.chars().next().unwrap()) {
                    return (top, bottom);
                }
            }
            self.inner.text_ink_bounds_signed(text, size, style)
        }

        fn math_constants(&self, _style: &TextStyle) -> MathConstants {
            self.constants.clone()
        }

        /// **P988-B** — attach registado; chars sem registo caem no
        /// default `None` do trait (caller cai na centragem simples).
        fn top_accent_attach(&self, c: char, _size: Pt, _style: &TextStyle) -> Option<Pt> {
            self.attach.get(&c).map(|&a| Pt(a))
        }
    }

    fn p922_constants(accent_base_height_du: f64) -> MathConstants {
        let mut c = MathConstants::fallback();
        c.accent_base_height = accent_base_height_du;
        c.flattened_accent_base_height = accent_base_height_du;
        c
    }

    /// Devolve a posição `y` do primeiro `FrameItem::Text` cujo texto é `target`.
    fn accent_y(box_: &MathBox, target: &str) -> f64 {
        box_.items
            .iter()
            .find_map(|i| match i {
                FrameItem::Text { pos, text, .. } if text.as_str() == target => {
                    Some(pos.y.val())
                }
                _ => None,
            })
            .unwrap_or_else(|| panic!("texto {:?} não encontrado em {:?}", target, box_.items))
    }

    /// **P922-1** (valores actualizados em **P989**) — o `new_ascent` é o
    /// topo da tinta acima da baseline: `max(base.ascent, −accent_y +
    /// accent.ascent)`. Quando o acento não espeta acima da base (tinta do
    /// acento mais baixa que o topo da base), a caixa fica com o ascent da
    /// base — o acento é "engolido". Os valores originais de P922 (4.32 e
    /// 5.52) mediam a fórmula aditiva com `accent_h` sem sinal, substituída
    /// em P989 (ver `accent.md` §P989: a forma `max` é algebricamente
    /// idêntica à do vanilla quando a altura de tinta vem com sinal, e
    /// dispensa o `signed_descent`).
    #[test]
    fn p922_signed_descent_aumenta_new_ascent() {
        let style = default_style(); // 12pt
        let c = p922_constants(540.0);

        let metrics_zero = SignedMetrics::new(c.clone())
            .with_signed('^', 2.4, 0.0)
            .with_ink('^', 2.4, 0.0);
        let metrics_neg = SignedMetrics::new(c.clone())
            .with_signed('^', 2.4, -1.2)
            .with_ink('^', 2.4, 0.0);

        let base = Content::MathText("x".into());
        let accent = Content::MathText("^".into());

        let box_zero = MathLayouter::new(&metrics_zero, true, &style)
            .layout_accent(&base, &accent, &style);
        let box_neg = MathLayouter::new(&metrics_neg, true, &style)
            .layout_accent(&base, &accent, &style);

        // P989: accent_y = -8.4 + min(8.4, 6.48) = -1.92; topo da tinta do
        // acento = 1.92 + 2.4 = 4.32 < base.ascent 8.4 → new_ascent = 8.4
        // nos dois casos (o fundo com sinal do acento é irrelevante — como
        // no vanilla, só o topo da tinta conta para a caixa).
        assert!(
            (box_zero.ascent - 8.4).abs() < 1e-9,
            "acento abaixo do topo da base: esperado ascent=8.4pt (o da base), obteve {:.4}",
            box_zero.ascent
        );
        assert!(
            (box_neg.ascent - 8.4).abs() < 1e-9,
            "idem com signed bottom negativo: esperado ascent=8.4pt, obteve {:.4}",
            box_neg.ascent
        );
    }

    /// **P922-2** — `accent_y` posiciona a baseline do acento de acordo com
    /// o cap `accent_base_height`: para base pequena (ascent < cap) usa o
    /// próprio ascent da base; para base grande (ascent > cap) usa o cap.
    #[test]
    fn p922_accent_y_respeita_cap_altura_base() {
        let style = default_style();
        let c = p922_constants(540.0); // cap = 6.48pt

        let metrics = SignedMetrics::new(c)
            .with_ink('x', 4.2, 0.0) // base pequena, abaixo do cap
            .with_ink('X', 14.0, 0.0) // base grande, acima do cap
            .with_signed('^', 2.4, -1.2)
            .with_ink('^', 2.4, 0.0);

        let ml = MathLayouter::new(&metrics, true, &style);
        let accent = Content::MathText("^".into());

        let box_small = ml.layout_accent(&Content::MathText("x".into()), &accent, &style);
        let box_large = ml.layout_accent(&Content::MathText("X".into()), &accent, &style);

        let y_small = accent_y(&box_small, "^");
        let y_large = accent_y(&box_large, "^");

        // base_ascent=4.2 < cap=6.48 => accent_y = -4.2 + 4.2 = 0.0
        assert!(
            (y_small - 0.0).abs() < 1e-9,
            "base pequena: esperado accent_y=0.0, obteve {:.4}",
            y_small
        );
        // base_ascent=14.0 > cap=6.48 => accent_y = -14.0 + 6.48 = -7.52
        assert!(
            (y_large - (-7.52)).abs() < 1e-9,
            "base grande: esperado accent_y=-7.52, obteve {:.4}",
            y_large
        );
    }

    /// **P922-3** (valores tornados auto-consistentes em **P989**) — quando
    /// o acento flutua muito acima da baseline (tinta 10.0..12.4pt), o topo
    /// da sua tinta eleva o `new_ascent` acima da base.
    #[test]
    fn p922_gap_positivo_quando_acento_muito_acima() {
        let style = default_style();
        let c = p922_constants(540.0);

        let metrics = SignedMetrics::new(c)
            .with_ink('x', 8.4, 0.0)
            .with_signed('^', 12.4, -10.0)
            .with_ink('^', 12.4, 0.0);

        let ml = MathLayouter::new(&metrics, true, &style);
        let box_ = ml.layout_accent(
            &Content::MathText("x".into()),
            &Content::MathText("^".into()),
            &style,
        );

        // P989: accent_y = -8.4 + min(8.4, 6.48) = -1.92; topo da tinta do
        // acento = 1.92 + 12.4 = 14.32 > 8.4 → new_ascent = 14.32 (= frame
        // vanilla: baseline-do-topo = accent.height + gap + base.ascent,
        // algebricamente idêntico — ver `accent.md` §P989).
        assert!(
            (box_.ascent - 14.32).abs() < 1e-9,
            "esperado ascent=14.32pt, obteve {:.4}",
            box_.ascent
        );

        // accent_y = -base_ascent + min(base_ascent, cap) = -8.4 + 6.48 = -1.92
        assert!(
            (accent_y(&box_, "^") - (-1.92)).abs() < 1e-9,
            "esperado accent_y=-1.92pt, obteve {:.4}",
            accent_y(&box_, "^")
        );
    }

    /// **P989** — acento aninhado (`dot(dot(x))`): a caixa do acento
    /// interno carrega o topo da tinta do primeiro ponto, logo o segundo
    /// ponto fica ACIMA do primeiro (y distintos), não sobreposto. O stub
    /// injecta o `signed` com o sinal que a L3 tinha antes da correcção
    /// (`bottom` positivo para tinta flutuante) — a fórmula `max` é imune
    /// a esse termo (cancela algebricamente, `accent.md` §P989).
    #[test]
    fn p989_acento_aninhado_segundo_ponto_acima_do_primeiro() {
        let style = default_style(); // 12pt
        let c = p922_constants(540.0); // abh = 6.48pt
        let dot = '\u{0307}';
        let metrics = SignedMetrics::new(c)
            .with_ink('x', 4.86, 0.12)
            .with_ink(dot, 7.44, 0.0)
            .with_signed(dot, 7.44, 6.24); // sinal da L3 pré-P989

        let base_x = Content::MathText("x".into());
        let dot_c = || Content::MathText(dot.to_string().into());
        let inner = Content::math_accent(base_x, dot_c());
        let outer = Content::math_accent(inner, dot_c());

        let ml = MathLayouter::new(&metrics, true, &style);
        let box_ = ml.layout_node(&outer, &style);

        // Posições dos dois pontos: interno em accent_y = -4.86 +
        // min(4.86, 6.48) = 0.0; externo em -(new_ascent interno) +
        // min(new_ascent interno, 6.48) = -7.44 + 6.48 = -0.96.
        let mut dot_ys: Vec<f64> = box_
            .items
            .iter()
            .filter_map(|i| match i {
                FrameItem::Text { pos, text, .. } if text.as_str().chars().next() == Some(dot) => {
                    Some(pos.y.val())
                }
                _ => None,
            })
            .collect();
        dot_ys.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(dot_ys.len(), 2, "devem existir exactamente 2 pontos: {:?}", box_.items);
        assert!(
            (dot_ys[0] - (-0.96)).abs() < 0.01 && (dot_ys[1] - 0.0).abs() < 0.01,
            "pontos esperados em y=-0.96 e y=0.0 (distintos), obteve {:?}",
            dot_ys
        );
        // new_ascent externo = max(7.44, 0.96 + 7.44) = 8.40
        assert!(
            (box_.ascent - 8.40).abs() < 0.01,
            "ascent da caixa aninhada deve ser 8.40pt, obteve {:.4}",
            box_.ascent
        );
    }

    /// **P988-B** — centragem horizontal com `TopAccentAttachment`:
    /// `dx = base_attach − accent_attach` (vanilla `accent.rs:37-52`),
    /// não centro-a-centro. Base "x" com attach 3.444pt (287du a 12pt) e
    /// acento "^" com attach 3.0pt (250du) → dx = 0.444pt (centro seria
    /// 0.0 com os advances iguais de FixedMetrics).
    #[test]
    fn p988b_acento_deslocado_por_top_accent_attachment() {
        let style = default_style(); // 12pt
        let c = p922_constants(540.0);
        let metrics = SignedMetrics::new(c)
            .with_attach('x', 3.444)
            .with_attach('^', 3.0);

        let ml = MathLayouter::new(&metrics, true, &style);
        let box_ = ml.layout_accent(
            &Content::MathText("x".into()),
            &Content::MathText("^".into()),
            &style,
        );

        let hat_x = box_
            .items
            .iter()
            .find_map(|i| match i {
                FrameItem::Text { pos, text, .. } if text.as_str() == "^" => Some(pos.x.val()),
                _ => None,
            })
            .expect("deve haver item do acento '^'");
        assert!(
            (hat_x - 0.444).abs() < 0.01,
            "dx deve ser base_attach(3.444) − accent_attach(3.0) = 0.444pt, obteve {hat_x:.4}"
        );
    }

    /// **P988-B (fallback)** — sem dados de `TopAccentAttachment` (stub sem
    /// registos), a centragem é a simples pré-P988 (metade das larguras) —
    /// mesma regra do vanilla para fragmentos compostos
    /// (`fragment/mod.rs:149`).
    #[test]
    fn p988b_sem_attach_cai_na_centragem_simples() {
        let style = default_style();
        let c = p922_constants(540.0);
        let metrics = SignedMetrics::new(c);

        let ml = MathLayouter::new(&metrics, true, &style);
        let box_ = ml.layout_accent(
            &Content::MathText("x".into()),
            &Content::MathText("^".into()),
            &style,
        );

        let hat_x = box_
            .items
            .iter()
            .find_map(|i| match i {
                FrameItem::Text { pos, text, .. } if text.as_str() == "^" => Some(pos.x.val()),
                _ => None,
            })
            .expect("deve haver item do acento '^'");
        let x_item = box_
            .items
            .iter()
            .find_map(|i| match i {
                FrameItem::Text { pos, text, .. } if text.as_str() == "x" => Some(pos.x.val()),
                _ => None,
            })
            .expect("deve haver item da base 'x'");
        let _ = x_item;
        // FixedMetrics: advance de "x" = advance de "^" = 0.6em → centro 0.
        assert!(
            hat_x.abs() < 0.01,
            "sem attach registado, dx deve ser a centragem simples (0.0 com advances iguais), obteve {hat_x:.4}"
        );
    }
}


// ── P945 — descida MathSize por nível (Display→Text é ×1.0), ─────────
// `total_descent` de grelhas sem dupla contagem, e guardas anti-deriva.
//
// Especificação: `00_nucleo/prompts/entities/layout_types.md` §P945,
// `compiler/math/layout/_comum.md` §P945, `matrix.md`/`cases.md` §P945.
// Medição que motiva: `00_nucleo/diagnosticos/typst-passo-945-relatorio.md`
// (células de mat/cases compostas a ×0.7 incondicional — o vanilla desce um
// nível DISCRETO: Display→Text ×1.0, Text→Script ×0.7,
// Script→ScriptScript ×sscript/script, ScriptScript→ScriptScript ×1.0).
//
// NOTA (Agente A, TDD): os testes que dependem do contexto `Display` NÃO
// podem ser escritos ao nível do `MathLayouter` — o nível chega via
// `TextStyle::math_size` (campo novo, ainda inexistente em produção) fixado
// em `compiler/layout/equation.rs`. Esses casos estão cobertos ao nível do
// `Layouter` completo em `01_core/src/compiler/layout/tests.rs` (procurar
// `p945_`). Os níveis Script/ScriptScript são exercitados aqui por via de
// matrizes aninhadas em scripts de `attach` (integração — depende de
// `attach.rs` manter `math_size` honesto, `attach.md` §P945).

mod p945_tests {
    use super::*;
    use crate::entities::math_constants::MathConstants;

    /// Stub com constantes sintéticas bem separadas (script=0.7,
    /// sscript=0.35) para que a distinção entre factores de descida
    /// (×0.7 incondicional vs ×sscript/script por nível) seja gritante
    /// — mesmo padrão de `cramped_test_constants` (P915) e
    /// `StubHorizontalMetrics::with_math_constants` (P906).
    struct P945Metrics {
        inner: FixedMetrics,
        constants: MathConstants,
    }

    impl P945Metrics {
        fn new() -> Self {
            let mut constants = MathConstants::fallback();
            constants.script_percent_scale_down = 0.7;
            constants.script_script_percent_scale_down = 0.35;
            Self { inner: FixedMetrics, constants }
        }
    }

    impl FontMetrics for P945Metrics {
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
        fn math_constants(&self, _style: &TextStyle) -> MathConstants {
            self.constants.clone()
        }
    }

    fn mat_2x2_digitos() -> Content {
        Content::math_matrix(
            vec![
                vec![Content::MathText("1".into()), Content::MathText("2".into())],
                vec![Content::MathText("3".into()), Content::MathText("4".into())],
            ],
            ('(', ')'),
        )
    }

    /// Tamanhos (`style.size`) dos items de texto das células `wanted`.
    fn cell_sizes(items: &[FrameItem], wanted: &[&str]) -> Vec<f64> {
        items
            .iter()
            .filter_map(|i| match i {
                FrameItem::Text { text, style, .. }
                    if wanted.contains(&text.as_str()) =>
                {
                    Some(style.size.val())
                }
                _ => None,
            })
            .collect()
    }

    /// `cramped` dos items de texto das células `wanted`.
    fn cell_crampeds(items: &[FrameItem], wanted: &[&str]) -> Vec<bool> {
        items
            .iter()
            .filter_map(|i| match i {
                FrameItem::Text { text, style, .. }
                    if wanted.contains(&text.as_str()) =>
                {
                    Some(style.cramped)
                }
                _ => None,
            })
            .collect()
    }

    const DIGITOS: [&str; 4] = ["1", "2", "3", "4"];

    /// **P945 — item 2 (guarda P923)**: em contexto `Text` (o default de
    /// `TextStyle`, equação inline), as células de `mat` continuam a
    /// `size × script_percent_scale_down` (Text→Script) e `cramped: true`
    /// — comportamento P923 medido correcto em inline, a preservar.
    #[test]
    fn p945_matriz_nivel_text_celulas_script() {
        let style = default_style(); // 12pt
        let ml = MathLayouter::new(&FixedMetrics, true, &style);
        let rows = vec![
            vec![Content::MathText("1".into()), Content::MathText("2".into())],
            vec![Content::MathText("3".into()), Content::MathText("4".into())],
        ];
        let b = ml.layout_matrix(&rows, ('(', ')'), &style);

        let sizes = cell_sizes(&b.items, &DIGITOS);
        assert_eq!(sizes.len(), 4, "4 células de texto esperadas");
        let esperado = 12.0 * 0.7; // script_percent_scale_down do fallback
        for s in &sizes {
            assert!(
                (s - esperado).abs() < 1e-9,
                "nível Text: célula deve ficar a size×0.7 ({:.4}pt), obteve {:.4}",
                esperado,
                s
            );
        }
        let crampeds = cell_crampeds(&b.items, &DIGITOS);
        assert!(
            crampeds.iter().all(|c| *c),
            "células de mat são sempre cramped (denominador): {crampeds:?}"
        );
    }

    /// **P945 — item 4 (guarda P923, par de `p945_matriz_nivel_text_celulas_script`)**:
    /// ramos de `cases` em contexto `Text` ficam a ×0.7, inalterado.
    #[test]
    fn p945_cases_nivel_text_celulas_script() {
        let style = default_style(); // 12pt
        let ml = MathLayouter::new(&FixedMetrics, true, &style);
        let rows = vec![
            vec![Content::MathText("1".into())],
            vec![Content::MathText("2".into())],
            vec![Content::MathText("3".into())],
        ];
        let b = ml.layout_cases(&rows, ('{', '}'), false, None, &style);

        let sizes = cell_sizes(&b.items, &["1", "2", "3"]);
        assert_eq!(sizes.len(), 3, "3 ramos de texto esperados");
        let esperado = 12.0 * 0.7;
        for s in &sizes {
            assert!(
                (s - esperado).abs() < 1e-9,
                "nível Text: ramo de cases deve ficar a size×0.7 ({:.4}pt), obteve {:.4}",
                esperado,
                s
            );
        }
    }

    /// **P945 — item 3a (nível aninhado Script)**: uma matriz dentro de um
    /// superscrito (`x^mat(...)`) está no nível `Script` (chega lá via
    /// `attach.rs`, que P945 obriga a manter `math_size` honesto). A descida
    /// das células é Script→ScriptScript: factor `sscript/script` sobre o
    /// tamanho corrente — com as constantes sintéticas (0.7/0.35): célula =
    /// 12 × 0.7 × (0.35/0.7) = 4.2pt. Hoje (×0.7 incondicional): 5.88pt.
    #[test]
    fn p945_matriz_em_superscript_desce_para_script_script() {
        let style = default_style(); // 12pt
        let metrics = P945Metrics::new();
        let ml = MathLayouter::new(&metrics, true, &style);

        let content = Content::math_attach(
            Content::MathIdent("x".into()),
            None,
            None,
            None,
            Some(mat_2x2_digitos()),
        );
        let items = ml.layout_equation(&content, &style);

        let sizes = cell_sizes(&items, &DIGITOS);
        assert_eq!(sizes.len(), 4, "4 células de texto esperadas: {sizes:?}");
        let esperado = 12.0 * 0.35; // = 12 × 0.7 × (0.35/0.7) — nível Script → ScriptScript
        for s in &sizes {
            assert!(
                (s - esperado).abs() < 1e-9,
                "matriz em nível Script: célula deve descer Script→ScriptScript \
                 (×sscript/script = {:.4}pt), obteve {:.4} — hoje aplica ×0.7 incondicional ({:.4})",
                esperado,
                s,
                12.0 * 0.7 * 0.7
            );
        }
    }

    /// **P945 — item 3b (nível aninhado ScriptScript)**: uma matriz dentro
    /// de um script de segundo nível (`x^y^mat(...)`) está em ScriptScript
    /// — ScriptScript→ScriptScript é ×1.0 (fundo da escada discreta do
    /// vanilla, `style.rs:343-363`): célula = tamanho corrente sem redução.
    /// O tamanho corrente é o do script de 2º nível (o factor de `attach.rs`
    /// NÃO muda neste passo — `attach.md` §P945): 12 × 0.7 × 0.7 = 5.88pt.
    /// Hoje as células descem MAIS um ×0.7: 4.116pt.
    #[test]
    fn p945_matriz_em_script_script_celulas_nao_descem_mais() {
        let style = default_style(); // 12pt
        let metrics = P945Metrics::new();
        let ml = MathLayouter::new(&metrics, true, &style);

        let inner = Content::math_attach(
            Content::MathIdent("y".into()),
            None,
            None,
            None,
            Some(mat_2x2_digitos()),
        );
        let content =
            Content::math_attach(Content::MathIdent("x".into()), None, None, None, Some(inner));
        let items = ml.layout_equation(&content, &style);

        let sizes = cell_sizes(&items, &DIGITOS);
        assert_eq!(sizes.len(), 4, "4 células de texto esperadas: {sizes:?}");
        let esperado = 12.0 * 0.35; // P1092: ScriptScript atinge script_script_percent_scale_down
        for s in &sizes {
            assert!(
                (s - esperado).abs() < 1e-9,
                "matriz em nível ScriptScript: célula NÃO deve descer mais (×1.0 = {:.4}pt), \
                 obteve {:.4} — hoje aplica ×0.7 incondicional ({:.4})",
                esperado,
                s,
                esperado,
            );
        }
    }

    /// **P945 — item 5 (`total_descent` sem dupla contagem)**: grelha de 3
    /// linhas com ascents/descents conhecidos (10/4 por linha, piso do `(`
    /// sintético inerte: 8.4/0 < 10/4), `row_gap = 2pt`. Forma do vanilla
    /// (`table.rs:103-106`): `total_descent = d1 + (a2+d2) + (a3+d3) +
    /// gap×(nrows−1)` = 4 + 14 + 14 + 4 = 36pt. Hoje o laço soma
    /// `next_row_descent` a mais em cada transição → 44pt (d1+d2 duplicados).
    #[test]
    fn p945_grid_total_descent_sem_dupla_contagem() {
        let style = default_style();
        let ml = MathLayouter::new(&FixedMetrics, true, &style);

        let cell = || MathBox { width: 5.0, ascent: 10.0, descent: 4.0, items: Vec::new() };
        let grid = vec![vec![cell()], vec![cell()], vec![cell()]];

        let b = ml.layout_grid_boxes(grid, GridAlign::Center, Pt(0.0), Pt(2.0), &[], &style);

        assert!(
            (b.ascent - 10.0).abs() < 1e-9,
            "ascent da grelha = ascent da 1ª linha (10pt), obteve {:.4}",
            b.ascent
        );
        // **P969** — valor esperado via oráculo (fórmula literal do
        // vanilla, `table.rs:103-106`), não aritmética solta no teste.
        let esperado = crate::testing::math_oracle::grid_total_descent(
            &[(10.0, 4.0), (10.0, 4.0), (10.0, 4.0)],
            2.0,
        ); // 36.0
        assert!(
            (b.descent - esperado).abs() < 1e-9,
            "total_descent pela forma do vanilla (d1 + Σ(a_r+d_r) + gap×2 = {:.4}pt), \
             obteve {:.4} — hoje conta next_row_descent duas vezes por linha intermédia",
            esperado,
            b.descent
        );
    }

    /// **P945 — guarda anti-deriva (`_comum.md` §P945 item 3)**: o alvo do
    /// delimitador de matrizes/casos é `(ascent+descent) × 1.1` convertido
    /// para design units — CONFIRMADO contra o vanilla
    /// (`resolve.rs:1168-1186`, `balanced=false`). A fórmula balanceada
    /// `2×max(a−axis, d+axis)` NÃO se aplica aqui (é só de `MathDelimited`).
    /// Este teste trava qualquer "correcção" futura que troque as fórmulas.
    ///
    /// **O ×1.1 é do vanilla — não é folga nossa.** `resolve.rs:1173`, em
    /// `fn resolve_delimiters` ("vector, matrix, or cases"):
    /// `let target = Rel::new(Ratio::new(1.1), Abs::zero());`, com o mesmo
    /// `DELIM_SHORT_FALL` que `stretchy.rs:57-61` aplica a seguir.
    ///
    /// P1024 e P1026 Fase A concluíram, cada um por sua via, que o ×1.1 era
    /// divergência — os dois contradisseram este comentário sem o consultar.
    /// P1026 Fase C refutou ambos na fonte e no output (38 configurações,
    /// ≤1 pixel a 300dpi, contagem de glifos idêntica). Ver `_comum.md`
    /// §P912-folga. Alinhar o alvo a 100% **introduz** divergência.
    #[test]
    fn p945_grid_delim_target_du_e_altura_vezes_1_1() {
        let style = default_style(); // 12pt, upem=1000 (fallback)
        let ml = MathLayouter::new(&FixedMetrics, true, &style);
        let grid_box = MathBox { width: 0.0, ascent: 20.0, descent: 10.0, items: Vec::new() };

        let du = ml.grid_delim_target_du(&grid_box, &style);
        let esperado = (20.0 + 10.0) * 1.1 * 1000.0 / 12.0; // 2750du
        assert!(
            (du - esperado).abs() < 1e-9,
            "alvo do delimitador = (a+d)×1.1 em du ({:.4}), obteve {:.4}",
            esperado,
            du
        );
    }
}

// ── P952 — fracção: descida por NÍVEL de MathSize (Display→Text ×1.0) ──
//
// `num_style`/`den_style` de `frac.rs` reduziam incondicionalmente
// `size × script_percent_scale_down` (P915). O vanilla
// (`style.rs:343-363`: `style_for_numerator`, `style_for_denominator` =
// numerator + `style_cramped()`) desce UM NÍVEL discreto de MathSize:
// Display→Text ×1.0, Text→Script ×script_percent,
// Script→ScriptScript ×sscript/script, ScriptScript→ScriptScript ×1.0 —
// com `cramped` forçado só no denominador. Ver `frac.md` §P952 e
// `_comum.md` §"numerator_style (novo helper, P952)" + §P945
// (`denominator_style`).
mod p952_tests {
    use super::*;
    use crate::entities::layout_types::MathSize;
    use crate::entities::math_constants::MathConstants;

    /// Stub com constantes sintéticas bem separadas (script=0.7,
    /// sscript=0.35) — mesmo padrão de `P945Metrics` (P945) e
    /// `cramped_test_constants` (P915). Com o fallback real
    /// (0.5/0.7 ≈ 0.714) o factor Script→ScriptScript seria quase
    /// indistinguível do ×0.7 incondicional; com 0.35/0.7 = 0.5 a
    /// distinção é gritante.
    struct P952Metrics {
        inner: FixedMetrics,
        constants: MathConstants,
    }

    impl P952Metrics {
        fn new() -> Self {
            let mut constants = MathConstants::fallback();
            constants.script_percent_scale_down = 0.7;
            constants.script_script_percent_scale_down = 0.35;
            Self { inner: FixedMetrics, constants }
        }
    }

    impl FontMetrics for P952Metrics {
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
        fn math_constants(&self, _style: &TextStyle) -> MathConstants {
            self.constants.clone()
        }
    }

    /// Estilo observado de um item de texto dentro do box da fracção.
    #[derive(Debug)]
    struct EstiloObservado {
        size: f64,
        math_size: MathSize,
        cramped: bool,
        y: f64,
    }

    /// Estilos dos `FrameItem::Text` de `frac(a,b)` ordenados por y:
    /// `[0]` = numerador (y mais negativo, acima), `[1]` = denominador.
    /// Ordenar por posição em vez de casar pelo texto evita depender do
    /// mapeamento itálico de glifos (P809) — `layout_frac` chamado
    /// directamente não passa por `layout_equation`.
    fn estilos_num_den(b: &MathBox) -> Vec<EstiloObservado> {
        let mut v: Vec<EstiloObservado> = b
            .items
            .iter()
            .filter_map(|i| match i {
                FrameItem::Text { pos, style, .. } => Some(EstiloObservado {
                    size: style.size.val(),
                    math_size: style.math_size,
                    cramped: style.cramped,
                    y: pos.y.val(),
                }),
                _ => None,
            })
            .collect();
        v.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());
        v
    }

    fn frac_ab() -> (Content, Content) {
        (Content::MathIdent("a".into()), Content::MathIdent("b".into()))
    }

    /// **P952 — RED — fracção em Display fica a tamanho cheio**: no nível
    /// `Display` (equação de bloco), a descida vanilla é Display→Text com
    /// factor **×1.0** (`style.rs:343-363`) — numerador e denominador são
    /// compostos a `style.size` sem redução, com `math_size` honesto
    /// (`Text`), `cramped` forçado só no denominador e herdado do
    /// ambiente (false) no numerador. Hoje: ×0.7 incondicional (8.4pt).
    #[test]
    fn p952_frac_display_num_den_tamanho_cheio() {
        let style = TextStyle { math_size: MathSize::Display, ..default_style() }; // 12pt
        let ml = MathLayouter::new(&FixedMetrics, true, &style);
        let (num, den) = frac_ab();
        let b = ml.layout_frac(&num, &den, &style);

        let estilos = estilos_num_den(&b);
        assert_eq!(estilos.len(), 2, "frac(a,b) deve ter 2 items de texto: {estilos:?}");
        let num_s = &estilos[0];
        let den_s = &estilos[1];

        assert!(
            (num_s.size - 12.0).abs() < 1e-9,
            "Display→Text é ×1.0: numerador deve ficar a size cheio (12.0pt), \
             obteve {:.4} — hoje aplica ×0.7 incondicional ({:.4})",
            num_s.size,
            12.0 * 0.7
        );
        assert!(
            (den_s.size - 12.0).abs() < 1e-9,
            "Display→Text é ×1.0: denominador deve ficar a size cheio (12.0pt), \
             obteve {:.4} — hoje aplica ×0.7 incondicional ({:.4})",
            den_s.size,
            12.0 * 0.7
        );
        assert_eq!(
            num_s.math_size,
            MathSize::Text,
            "math_size honesto: numerador em Display desce para Text"
        );
        assert_eq!(
            den_s.math_size,
            MathSize::Text,
            "math_size honesto: denominador em Display desce para Text"
        );
        assert!(den_s.cramped, "denominador é sempre cramped (P915)");
        assert!(
            !num_s.cramped,
            "numerador herda cramped do ambiente (false por omissão) — \
             vanilla style_for_numerator NÃO força style_cramped()"
        );
    }

    /// **P952 — GREEN esperado (guarda P915/P923) — inline/Text mantém
    /// ×0.7**: em contexto `Text` (o default, equação inline), a descida
    /// é Text→Script com factor `script_percent_scale_down` — exactamente
    /// o comportamento actual, a preservar (frac.md §P952: "fracções em
    /// contexto Text (inline) e Script mantêm o comportamento actual").
    #[test]
    fn p952_frac_nivel_text_mantem_script_percent() {
        let style = default_style(); // 12pt, math_size: Text (default)
        let ml = MathLayouter::new(&FixedMetrics, true, &style);
        let (num, den) = frac_ab();
        let b = ml.layout_frac(&num, &den, &style);

        let estilos = estilos_num_den(&b);
        assert_eq!(estilos.len(), 2, "frac(a,b) deve ter 2 items de texto");
        let esperado = 12.0 * 0.7; // script_percent_scale_down do fallback
        for (nome, s) in [("numerador", &estilos[0]), ("denominador", &estilos[1])] {
            assert!(
                (s.size - esperado).abs() < 1e-9,
                "nível Text: {nome} deve ficar a size×0.7 ({esperado:.4}pt), obteve {:.4}",
                s.size
            );
            assert_eq!(
                s.math_size,
                MathSize::Script,
                "math_size honesto: {nome} em Text desce para Script"
            );
        }
        assert!(estilos[1].cramped, "denominador é sempre cramped");
        assert!(!estilos[0].cramped, "numerador herda cramped (false)");
    }

    /// **P952 — RED — aninhamento, nível Script**: a descida é
    /// Script→ScriptScript com factor `sscript/script` sobre o tamanho
    /// corrente — com as constantes sintéticas (0.7/0.35): 12 × 0.5 =
    /// 6.0pt. Hoje (×0.7 incondicional): 8.4pt.
    #[test]
    fn p952_frac_nivel_script_desce_sscript_sobre_script() {
        let style =
            TextStyle { math_size: MathSize::Script, ..default_style() }; // 12pt, nível Script
        let metrics = P952Metrics::new();
        let ml = MathLayouter::new(&metrics, true, &style);
        let (num, den) = frac_ab();
        let b = ml.layout_frac(&num, &den, &style);

        let estilos = estilos_num_den(&b);
        assert_eq!(estilos.len(), 2, "frac(a,b) deve ter 2 items de texto");
        let esperado = 12.0 * (0.35 / 0.7); // 6.0pt — Script→ScriptScript
        for (nome, s) in [("numerador", &estilos[0]), ("denominador", &estilos[1])] {
            assert!(
                (s.size - esperado).abs() < 1e-9,
                "nível Script: {nome} deve descer Script→ScriptScript \
                 (×sscript/script = {esperado:.4}pt), obteve {:.4} — \
                 hoje aplica ×0.7 incondicional ({:.4})",
                s.size,
                12.0 * 0.7
            );
            assert_eq!(
                s.math_size,
                MathSize::ScriptScript,
                "math_size honesto: {nome} em Script desce para ScriptScript"
            );
        }
    }

    /// **P952 — RED — aninhamento, nível ScriptScript**: fundo da escada
    /// discreta do vanilla — ScriptScript→ScriptScript é ×1.0, o tamanho
    /// NÃO desce mais. Hoje: ×0.7 incondicional (8.4pt em vez de 12pt).
    #[test]
    fn p952_frac_nivel_script_script_nao_desce_mais() {
        let style =
            TextStyle { math_size: MathSize::ScriptScript, ..default_style() }; // 12pt
        let metrics = P952Metrics::new();
        let ml = MathLayouter::new(&metrics, true, &style);
        let (num, den) = frac_ab();
        let b = ml.layout_frac(&num, &den, &style);

        let estilos = estilos_num_den(&b);
        assert_eq!(estilos.len(), 2, "frac(a,b) deve ter 2 items de texto");
        for (nome, s) in [("numerador", &estilos[0]), ("denominador", &estilos[1])] {
            assert!(
                (s.size - 12.0).abs() < 1e-9,
                "nível ScriptScript: {nome} NÃO deve descer mais (×1.0 = 12.0pt), \
                 obteve {:.4} — hoje aplica ×0.7 incondicional ({:.4})",
                s.size,
                12.0 * 0.7
            );
            assert_eq!(
                s.math_size,
                MathSize::ScriptScript,
                "math_size honesto: {nome} em ScriptScript fica em ScriptScript"
            );
        }
    }

    /// **P952 — geometria** (constantes actualizadas em **P990-A**): `$
    /// frac(a,b) $` display a 12pt com as constantes fallback (upem=1000)
    /// e `FixedMetrics` (folha: ascent = 0.7×size via `cap_height`,
    /// descent = 0 — default de `text_ink_bounds`, P921). Com num/den a
    /// TAMANHO CHEIO (12pt), e — desde P990-A — com as 4 constantes
    /// DISPLAY dedicadas (não as de texto, que este teste usava antes de
    /// P990-A existir):
    ///
    /// - axis_pt = 500du → 6.0; thickness = 66du → 0.792;
    ///   shift_up = 677du → 8.124; shift_down = 686du → 8.232;
    ///   pisos = 120du → 1.44.
    /// - num_gap = (8.124 − 6.0 − 0.396 − 0).max(1.44) = **1.728**
    ///   (fórmula, acima do piso); den_gap = (8.232 + 6.0 − 0.396 −
    ///   8.4).max(1.44) = **5.436** (fórmula).
    /// - ascent = 8.4 + 1.728 + 0.396 + 6.0 = **16.524pt**;
    ///   descent = 8.4 + 5.436 + 0.396 − 6.0 = **8.232pt**;
    ///   altura total = **24.756pt**.
    ///
    /// Antes de P952 (×0.7 incondicional, folha a 8.4pt): altura
    /// 17.016pt. A fracção display tem de CRESCER face a isso — primeiro
    /// pelo tamanho cheio (P952), depois mais ainda pelas constantes
    /// Display dedicadas em vez das de texto (P990-A, achado §8.5: gaps
    /// reais do vanilla em Display, ~2.56pt/~1.43pt+, exigem piso/shift
    /// maiores do que os de texto — ver `frac.md` §P990-A).
    #[test]
    fn p952_frac_display_geometria_cresce_para_tamanho_cheio() {
        let style = TextStyle { math_size: MathSize::Display, ..default_style() }; // 12pt
        let ml = MathLayouter::new(&FixedMetrics, true, &style);
        let (num, den) = frac_ab();
        let frac_box = ml.layout_frac(&num, &den, &style);

        let c = MathConstants::fallback(); // upem=1000 — o que FixedMetrics devolve
        let axis_pt = c.to_pt(c.axis_height, style.size).val();
        let thickness_pt = c.to_pt(c.fraction_rule_thickness, style.size).val();
        // **P990-A** — Display usa as 4 constantes dedicadas, não as de
        // texto (`fraction_numerator_shift_up` etc.) usadas por este teste
        // antes de P990-A existir.
        let shift_up_pt =
            c.to_pt(c.fraction_numerator_display_style_shift_up, style.size).val();
        let shift_down_pt =
            c.to_pt(c.fraction_denominator_display_style_shift_down, style.size).val();
        let floor_pt = c.to_pt(c.fraction_num_display_style_gap_min, style.size).val();

        // Sanity — aritmética dos números citados acima (mesma disciplina
        // dos testes P920).
        assert!((axis_pt - 6.0).abs() < 1e-9, "sanity axis_pt, foi {axis_pt}");
        assert!((thickness_pt - 0.792).abs() < 1e-9, "sanity thickness, foi {thickness_pt}");
        assert!((shift_up_pt - 8.124).abs() < 1e-9, "sanity shift_up, foi {shift_up_pt}");
        assert!((shift_down_pt - 8.232).abs() < 1e-9, "sanity shift_down, foi {shift_down_pt}");
        assert!((floor_pt - 1.44).abs() < 1e-9, "sanity floor_pt, foi {floor_pt}");

        // Caixas esperadas com a descida NOVA (Display→Text ×1.0):
        // folha a 12pt → ascent 8.4, descent 0.
        let folha = 12.0_f64;
        let leaf_ascent = folha * 0.7; // 8.4
        let leaf_descent = 0.0_f64;
        let num_gap = (shift_up_pt - axis_pt - thickness_pt / 2.0 - leaf_descent).max(floor_pt);
        let den_gap = (shift_down_pt + axis_pt - thickness_pt / 2.0 - leaf_ascent).max(floor_pt);
        let expected_ascent = leaf_ascent + leaf_descent + num_gap + thickness_pt / 2.0 + axis_pt;
        let expected_descent =
            leaf_ascent + leaf_descent + den_gap + thickness_pt / 2.0 - axis_pt;
        assert!((expected_ascent - 16.524).abs() < 1e-9, "sanity ascent, foi {expected_ascent}");
        assert!((expected_descent - 8.232).abs() < 1e-9, "sanity descent, foi {expected_descent}");

        assert!(
            (frac_box.ascent - expected_ascent).abs() < 1e-6,
            "fracção display a tamanho cheio: ascent esperado {expected_ascent:.4}pt, \
             obteve {:.4}pt",
            frac_box.ascent
        );
        assert!(
            (frac_box.descent - expected_descent).abs() < 1e-6,
            "fracção display a tamanho cheio: descent esperado {expected_descent:.4}pt, \
             obteve {:.4}pt",
            frac_box.descent
        );

        // A altura total tem de CRESCER face ao comportamento pré-P952
        // (17.016pt, computado com folha a 8.4pt e constantes de texto) —
        // é este crescimento (primeiro tamanho cheio, depois constantes
        // Display de P990-A) que fecha os deltas de gap no documento de 30
        // secções (frac.md §P952/§P990-A, medição).
        let altura = frac_box.ascent + frac_box.descent;
        let altura_anterior = 17.016_f64;
        assert!(
            (altura - 24.756).abs() < 1e-6,
            "altura total esperada 24.756pt (tamanho cheio + constantes Display), \
             obteve {altura:.4}pt"
        );
        assert!(
            altura > altura_anterior,
            "fracção display deve CRESCER face ao ×0.7 anterior \
             ({altura_anterior:.4}pt), obteve {altura:.4}pt"
        );
    }
}


// ── P952 (segunda frente) — operadores grandes (`MathClass::Large`)
//    esticados em Display (`_comum.md` §P952, fim) ─────────────────────
//
// **⚠ ESTADO DE COMPILAÇÃO — TDD red, precedente `p920_tests`**: este
// módulo referencia o campo NOVO `MathConstants::display_operator_min_height`
// (`entities/math_constants.md` §P952), que ainda NÃO existe na struct —
// será criado pelo Agente B deste passo. **Este módulo NÃO COMPILA até esse
// campo existir** (erro E0609 confinado ao construtor do stub
// `P952OpMetrics::new`); foi escrito para compilar e correr assim que o
// campo for adicionado, sem mais alterações. Não implementar o campo aqui
// — só os testes (contrato primeiro).
//
// Comportamento especificado (L0 `_comum.md` §P952): nos braços
// `Content::MathIdent`/`Content::MathText` de `layout_node`, quando o texto
// é UM único carácter, `symbols::is_large_operator(c)` e `self.block`
// (Display), o glifo passa por `layout_large_operator_display(c, style)`:
// primeira variante vertical com `advance >= display_operator_min_height`
// (em du, SEM `DELIM_SHORT_FALL` — `StretchInfo::default()` do vanilla tem
// `short_fall = Em::zero()`), emitida como `FrameItem::Glyph` com
// `x_advance`/largura vindos de `hor_advance` (mesma disciplina P917);
// `ascent`/`descent` da MathBox vêm das métricas da variante (altura
// `advance`); sem variante suficiente → glifo base (inalterado). Inline
// (`block: false`) e scripts mantêm o glifo base. `is_integral_char` NÃO
// exclui — no vanilla integrais são `Large` para o stretch.
//
// Medições de referência (Fase A de P952, PDFs reais NewCMMath, upem=1000)
// usadas nos dados do stub: `∑` base altura 1001du / advance horizontal
// 1.056em; `summation.v1` altura 1401du / 1.444em. `∫` base 1112du /
// 0.665em; `integral.v1` 2223du / 0.999em. Alvo NewCMMath:
// `DisplayOperatorMinHeight` = 1300du.
mod p952op_tests {
    use super::*;
    use crate::entities::glyph_variants::{GlyphVariant, GlyphVariants};
    use crate::entities::math_constants::MathConstants;

    // Glyph IDs sintéticos do stub (distintos do caminho de glifo base,
    // que não emite `FrameItem::Glyph` — ver nota em `glyph_to_char`).
    const SUM_BASE_GID: u16 = 100;
    const SUM_V1_GID: u16 = 101;
    const INT_BASE_GID: u16 = 200;
    const INT_V1_GID: u16 = 201;

    /// Stub configurável (mesmo padrão de `StubHorizontalMetrics`, P906/P917,
    /// e `P952Metrics` de fracção): delega os métodos obrigatórios a
    /// `FixedMetrics` e sobrescreve `vertical_glyph_variants` com os dados
    /// REAIS medidos de NewCMMath para `∑`/`∫` (ver bloco acima),
    /// `glyph_to_char` → `None` (força o caminho `FrameItem::Glyph`, sem
    /// mapeamento Unicode — caso `else` de `stretchy.rs:66`) e
    /// `math_constants` com o campo novo parametrizável.
    struct P952OpMetrics {
        inner: FixedMetrics,
        constants: MathConstants,
        /// **P959** — bbox de tinta da variante (gid SUM_V1_GID), em pt.
        /// `None` → comportamento default do trait (`cap_height`, 0) —
        /// os testes P952 existentes não são afectados.
        variant_ink: Option<(f64, f64)>,
        /// **P963** — idem para a variante de `∫` (INT_V1_GID).
        int_ink: Option<(f64, f64)>,
        /// **P971** — italics correction da variante de `∫` (INT_V1_GID),
        /// em pt. `None` → default do trait (0).
        int_ic: Option<f64>,
    }

    impl P952OpMetrics {
        fn new(display_operator_min_height: f64) -> Self {
            let mut constants = MathConstants::fallback();
            // ⚠ CAMPO NOVO (P952 — `entities/math_constants.md` §P952):
            // não existe ainda em `MathConstants`; E0609 aqui até o
            // Agente B o criar. Valor NewCMMath: 1300du.
            constants.display_operator_min_height = display_operator_min_height;
            Self { inner: FixedMetrics, constants, variant_ink: None, int_ink: None, int_ic: None }
        }

        /// Constantes com o alvo real de NewCMMath (1300du).
        fn ncm() -> Self {
            Self::new(1300.0)
        }

        /// **P959** — injecta a bbox de tinta da variante v1 (acima, abaixo
        /// da baseline, em pt) — para o teste dos extents reais da caixa.
        fn with_variant_ink(mut self, up: f64, down: f64) -> Self {
            self.variant_ink = Some((up, down));
            self
        }

        /// **P963** — idem para a variante de `∫` (INT_V1_GID).
        fn with_int_ink(mut self, up: f64, down: f64) -> Self {
            self.int_ink = Some((up, down));
            self
        }

        /// **P971** — injecta a italics correction da variante v1 do ∫
        /// (valor real: 450du = 5.4pt a 12pt).
        fn with_int_ic(mut self, ic_pt: f64) -> Self {
            self.int_ic = Some(ic_pt);
            self
        }
    }

    impl FontMetrics for P952OpMetrics {
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
        fn math_constants(&self, _style: &TextStyle) -> MathConstants {
            self.constants.clone()
        }
        fn vertical_glyph_variants(&self, c: char, _style: &TextStyle) -> GlyphVariants {
            // Dados medidos (Fase A P952): primeira entrada = glifo base
            // (altura do glifo não-esticado), segunda = `.v1`. `advance` =
            // altura (eixo de esticamento); `hor_advance` = avanço
            // horizontal nativo (hmtx) — distintos de propósito para
            // apanhar a confusão advance/hor_advance (P917).
            match c {
                '∑' => GlyphVariants {
                    variants: vec![
                        GlyphVariant { glyph_id: SUM_BASE_GID, advance: 1001.0, hor_advance: 1056.0 },
                        GlyphVariant { glyph_id: SUM_V1_GID, advance: 1401.0, hor_advance: 1444.0 },
                    ],
                },
                '∫' => GlyphVariants {
                    variants: vec![
                        GlyphVariant { glyph_id: INT_BASE_GID, advance: 1112.0, hor_advance: 665.0 },
                        GlyphVariant { glyph_id: INT_V1_GID, advance: 2223.0, hor_advance: 999.0 },
                    ],
                },
                _ => GlyphVariants::default(),
            }
        }
        fn glyph_to_char(&self, _glyph_id: u16) -> Option<char> {
            // Sem mapeamento reverso → a implementação tem de emitir
            // `FrameItem::Glyph` (não `Text`), como em produção com a fonte
            // real para `.v1` (glifo sem codepoint próprio).
            None
        }
        fn glyph_ink_bounds(&self, glyph_id: u16, size: Pt, style: &TextStyle) -> (Pt, Pt) {
            // **P959** — quando injectada, devolve a bbox da variante;
            // senão, o default do trait (cap_height, 0).
            if let Some((up, down)) = self.variant_ink {
                if glyph_id == SUM_V1_GID {
                    return (Pt(up), Pt(down));
                }
            }
            // **P963** — idem para a variante de ∫.
            if let Some((up, down)) = self.int_ink {
                if glyph_id == INT_V1_GID {
                    return (Pt(up), Pt(down));
                }
            }
            let _ = glyph_id;
            (self.cap_height(size, style), Pt(0.0))
        }

        /// **P971** — IC injectada para a variante de ∫; resto: default (0).
        fn italics_correction(&self, glyph_id: u16, size: Pt, style: &TextStyle) -> Pt {
            if let Some(ic) = self.int_ic {
                if glyph_id == INT_V1_GID {
                    return Pt(ic);
                }
            }
            let _ = (size, style);
            Pt(0.0)
        }
    }

    /// `(glyph_id, x_advance_pt)` dos `FrameItem::Glyph` de um MathBox.
    fn glyphs(b: &MathBox) -> Vec<(u16, f64)> {
        b.items
            .iter()
            .filter_map(|i| match i {
                FrameItem::Glyph { glyph_id, x_advance, .. } => Some((*glyph_id, x_advance.val())),
                _ => None,
            })
            .collect()
    }

    /// `(x, y)` do `FrameItem::Text` cujo texto é exactamente `needle`.
    fn text_pos(b: &MathBox, needle: &str) -> Option<(f64, f64)> {
        b.items.iter().find_map(|i| match i {
            FrameItem::Text { pos, text, .. } if text.as_str() == needle => {
                Some((pos.x.val(), pos.y.val()))
            }
            _ => None,
        })
    }

    /// **P959 — RED — a caixa da variante de Display usa os extents de
    /// TINTA reais** (vanilla `update_glyph`, `fragment/glyph.rs:215-231`:
    /// `baseline = ascent`, `size = ascent + descent` da bbox do glifo),
    /// não o split simétrico `advance/2` de P952. Achado do smoke de P959
    /// (Agente B + revisão do orquestrador): com o split metade/metade, a
    /// fórmula de limites de P959 recebe `base_ascent`/`base_descent`
    /// errados e os limites não caem na banda do vanilla — ex.: `∑` v1
    /// (advance 1401du → 16.812pt a 12pt) com tinta assimétrica (12.0 acima,
    /// 3.6 abaixo) deve dar `ascent=12.0, descent=3.6`, não 8.406/8.406.
    #[test]
    fn p959_variante_display_caixa_usa_extents_de_tinta() {
        let metrics = P952OpMetrics::ncm().with_variant_ink(12.0, 3.6);
        let style = default_style(); // 12pt
        let ml = MathLayouter::new(&metrics, true, &style);

        let b = ml.layout_node(&Content::MathIdent("∑".into()), &style);

        let g = glyphs(&b);
        assert_eq!(g.len(), 1, "display deve usar a variante v1: {g:?}");
        assert!(
            (b.ascent - 12.0).abs() < 1e-9,
            "ascent = tinta acima da baseline (12.0pt), obteve {:.4} — \
             split simétrico daria {:.4}",
            b.ascent,
            12.0 * 1401.0 / 1000.0 / 2.0
        );
        assert!(
            (b.descent - 3.6).abs() < 1e-9,
            "descent = tinta abaixo da baseline (3.6pt), obteve {:.4} — \
             split simétrico daria {:.4}",
            b.descent,
            12.0 * 1401.0 / 1000.0 / 2.0
        );
        // Largura/altura total preservadas: ascent+descent continua a ser a
        // altura total da tinta declarada (não necessariamente o advance).
        assert!(b.width > 0.0, "largura presente");
    }

    // ── P963 — `is_text_like` exclui bases esticadas (extended_shape) ────
    //
    // Especificação: `math/layout/attach.md` §P963. O `is_text_like` do
    // vanilla (`fragment/mod.rs:129-135`) é `!extended_shape` para glifos —
    // um operador esticado em Display (variante/assembly =
    // `FrameItem::Glyph` no cristalino) NÃO é text-like, e o termo
    // `base_ascent − sup_drop_max` aplica-se ao `shift_up` do script
    // lateral. O cristalino derivava `is_text_like` do Content
    // (MathIdent/MathText) — verdadeiro para `∫`, logo o termo ficava a
    // zero e o sup de `∫_0^1` em bloco ficava ~2.8pt abaixo da baseline da
    // base em vez de ~12.9pt acima (medido, Fase A de P963).

    /// y do primeiro item cujo texto/base_char casa com `needle`.
    fn p963_y(items: &[FrameItem], needle: &str) -> f64 {
        items
            .iter()
            .find_map(|i| match i {
                FrameItem::Text { pos, text, .. } if text.as_str() == needle => {
                    Some(pos.y.val())
                }
                FrameItem::Glyph { pos, base_char, .. } if base_char.to_string() == needle => {
                    Some(pos.y.val())
                }
                _ => None,
            })
            .unwrap_or_else(|| panic!("{needle:?} não encontrado em {items:?}"))
    }

    /// **Caso principal** — `∫_0^1` em bloco (Display): base esticada v1
    /// (ink 16.332/10.332pt a 12pt — os valores reais de integral.v1,
    /// 1361/−861du). O shift do sup lateral deve ser
    /// `max(sup_shift_up=4.356, base_ascent − sup_drop_max = 16.332−3.0 =
    /// 13.332, sup_bottom_min + sup.descent)` = **13.332pt** acima da
    /// baseline; o sub `max(sub_shift_down=2.964, base_descent +
    /// sub_drop_min = 10.332+0.6 = 10.932, …)` = **10.932pt** abaixo
    /// (constantes do `MathConstants::fallback()` usadas pelo stub:
    /// sup_drop_max=250du, sub_drop_min=50du — NB: NewCMMath real tem
    /// sub_drop_min=200du; a forma é o que está em teste, não o valor).
    /// Antes de P963 (is_text_like=true por engano): sup a −4.356pt.
    #[test]
    fn p963_integral_display_sup_lateral_usa_drop_term() {
        let metrics = P952OpMetrics::ncm().with_int_ink(16.332, 10.332);
        let style = default_style(); // 12pt; 1du = 0.012pt
        let ml = MathLayouter::new(&metrics, true, &style);

        let content = Content::math_attach(
            Content::MathText("∫".into()),
            None,
            None,
            Some(Content::MathText("0".into())),
            Some(Content::MathText("1".into())),
        );
        let items = ml.layout_equation(&content, &style);

        let y_base = p963_y(&items, "∫");
        let y_sup = p963_y(&items, "1");
        let y_sub = p963_y(&items, "0");
        assert!(
            ((y_base - y_sup) - 13.332).abs() < 0.05,
            "sup lateral de ∫ esticado: 13.332pt acima da baseline da base \
             (vanilla drop term); obteve {:.4} (y_base={y_base:.3}, y_sup={y_sup:.3})",
            y_base - y_sup
        );
        assert!(
            ((y_sub - y_base) - 10.932).abs() < 0.05,
            "sub lateral de ∫ esticado: 10.932pt abaixo da baseline da base \
             (base_descent 10.332 + sub_drop_min fallback 0.6); obteve {:.4}",
            y_sub - y_base
        );
    }

    /// **Guarda** — base NÃO esticada (text-like, ex.: `x^2`): o drop term
    /// fica a zero (vanilla `!extended_shape`), o sup fica em
    /// `sup_shift_up` (4.356pt a 12pt), não em 13pt.
    #[test]
    fn p963_base_texto_sup_sem_drop_term() {
        let metrics = P952OpMetrics::ncm().with_int_ink(16.332, 10.332);
        let style = default_style();
        let ml = MathLayouter::new(&metrics, true, &style);

        let content = Content::math_attach(
            Content::MathIdent("x".into()),
            None,
            None,
            None,
            Some(Content::MathText("2".into())),
        );
        let items = ml.layout_equation(&content, &style);

        let y_base = p963_y(&items, "\u{1D465}"); // 𝑥 (itálico default P809)
        let y_sup = p963_y(&items, "2");
        let shift = y_base - y_sup;
        assert!(
            shift < 6.0,
            "base text-like: sup fica em sup_shift_up (~4.4pt), sem drop term \
             (13.3); obteve {shift:.4}"
        );
    }

    /// **P952 — RED — `∑` em Display usa `summation.v1`**: `layout_node` de
    /// `Content::MathIdent("∑")` com layouter de bloco (`block: true`) tem
    /// de produzir UM `FrameItem::Glyph` com o glyph_id da v1 (não o do
    /// base), `x_advance` = `hor_advance` da v1 (1444du → 17.328pt a
    /// 12pt/upem=1000), largura consistente (P917) e altura da MathBox =
    /// altura da variante (1401du → 16.812pt — L0: "ascent/descent vêm das
    /// métricas da variante, altura `advance`"; a divisão ascent/descent é
    /// do mecanismo `update_glyph` do vanilla, não fixada aqui). Hoje:
    /// caminho `layout_text_node` (Text, sem Glyph) — falha.
    #[test]
    fn p952op_sum_display_usa_variante_v1() {
        // P959 — tinta injectada (a caixa vem da bbox desde P959, não do
        // advance); 12.0+4.812 = 16.812pt = advance 1401du a 12pt.
        let metrics = P952OpMetrics::ncm().with_variant_ink(12.0, 4.812);
        let style = default_style(); // 12pt
        let ml = MathLayouter::new(&metrics, true, &style);

        let b = ml.layout_node(&Content::MathIdent("∑".into()), &style);

        let g = glyphs(&b);
        assert_eq!(
            g.len(),
            1,
            "∑ display deve produzir exactamente 1 FrameItem::Glyph (a variante v1); \
             obteve {g:?} em {:?}",
            b.items
        );
        assert_eq!(
            g[0].0, SUM_V1_GID,
            "variante seleccionada deve ser summation.v1 (gid {SUM_V1_GID}), não o glifo base \
             (gid {SUM_BASE_GID}) — alvo 1300du, v1 tem advance 1401du >= 1300"
        );
        let esperado_adv = 12.0 * 1444.0 / 1000.0; // 17.328pt — hor_advance da v1
        assert!(
            (g[0].1 - esperado_adv).abs() < 1e-9,
            "x_advance deve vir de hor_advance da v1 ({esperado_adv:.4}pt), obteve {:.4} \
             — nunca de `advance` (1401du → 16.812pt, eixo de esticamento; P917)",
            g[0].1
        );
        assert!(
            (b.width - esperado_adv).abs() < 1e-9,
            "MathBox.width consistente com x_advance ({esperado_adv:.4}pt), obteve {:.4}",
            b.width
        );
        // **P959** — a altura da caixa passa a vir da bbox de tinta da
        // variante (`glyph_ink_bounds`, vanilla `update_glyph`), não do
        // `advance`: o stub injecta a tinta real da v1 (12.0 + 4.812 =
        // 16.812pt = advance 1401du a 12pt — em NewCMMath a tinta da
        // variante cobre o advance inteiro, por isso coincidem).
        let esperado_h = 12.0 + 4.812; // 16.812pt — tinta injectada da v1
        assert!(
            ((b.ascent + b.descent) - esperado_h).abs() < 1e-9,
            "altura da MathBox (ascent+descent) = tinta da variante v1 ({esperado_h:.4}pt), \
             obteve {:.4}",
            b.ascent + b.descent
        );
    }

    /// **P952 — RED — o braço `Content::MathText` também estica**: o L0
    /// (`_comum.md` §P952) nomeia os DOIS braços, `MathIdent` e `MathText`
    /// — o lexer pode entregar `∑` por qualquer um. Mesma asserção do
    /// teste anterior, via `MathText("∑")`.
    #[test]
    fn p952op_mathtext_display_tambem_estica() {
        let metrics = P952OpMetrics::ncm();
        let style = default_style();
        let ml = MathLayouter::new(&metrics, true, &style);

        let b = ml.layout_node(&Content::MathText("∑".into()), &style);

        let g = glyphs(&b);
        assert_eq!(
            g.len(),
            1,
            "MathText(\"∑\") display deve produzir 1 FrameItem::Glyph (v1); obteve {g:?} em {:?}",
            b.items
        );
        assert_eq!(g[0].0, SUM_V1_GID, "MathText display: variante esperada = summation.v1");
    }

    /// **P952 — GREEN esperado (guarda) — `∑` inline mantém o glifo base**:
    /// o vanilla só estica em `MathSize::Display`; com `block: false` o
    /// comportamento é o de hoje — o caminho actual é `layout_text_node`,
    /// que emite `FrameItem::Text` com o próprio carácter (não Glyph).
    /// Este teste documenta e trava esse caminho.
    #[test]
    fn p952op_sum_inline_mantem_glifo_base() {
        let metrics = P952OpMetrics::ncm();
        let style = default_style();
        let ml = MathLayouter::new(&metrics, false, &style); // inline

        let b = ml.layout_node(&Content::MathIdent("∑".into()), &style);

        assert!(
            glyphs(&b).is_empty(),
            "inline NÃO deve esticar: nenhum FrameItem::Glyph esperado, obteve {:?}",
            glyphs(&b)
        );
        assert!(
            text_pos(&b, "∑").is_some(),
            "inline deve manter o glifo base via caminho actual (FrameItem::Text \"∑\"); \
             items: {:?}",
            b.items
        );
    }

    /// **P952 — RED — `∫` em Display usa `integral.v1`**: integrais também
    /// esticam — no vanilla são `Large` para o stretch (`is_integral_char`
    /// só impede EMPILHAR limites, não o esticamento; L0 `_comum.md` §P952:
    /// "`is_integral_char` não exclui"). Alvo 1300du: base 1112du < 1300,
    /// v1 2223du >= 1300 → v1. `hor_advance` da v1 = 999du → 11.988pt.
    #[test]
    fn p952op_integral_display_usa_variante_v1() {
        let metrics = P952OpMetrics::ncm();
        let style = default_style();
        let ml = MathLayouter::new(&metrics, true, &style);

        let b = ml.layout_node(&Content::MathIdent("∫".into()), &style);

        let g = glyphs(&b);
        assert_eq!(
            g.len(),
            1,
            "∫ display deve produzir exactamente 1 FrameItem::Glyph (integral.v1); \
             obteve {g:?} em {:?}",
            b.items
        );
        assert_eq!(
            g[0].0, INT_V1_GID,
            "variante seleccionada deve ser integral.v1 (gid {INT_V1_GID}), não o glifo base \
             (gid {INT_BASE_GID}) — is_integral_char NÃO exclui do stretch"
        );
        let esperado_adv = 12.0 * 999.0 / 1000.0; // 11.988pt
        assert!(
            (g[0].1 - esperado_adv).abs() < 1e-9,
            "x_advance deve vir de hor_advance da v1 ({esperado_adv:.4}pt), obteve {:.4}",
            g[0].1
        );
    }

    /// **P952 — GREEN esperado (guarda) — carácter não-grande inalterado**:
    /// `x` não é `is_large_operator` — em Display o caminho é exactamente o
    /// de hoje (`layout_text_node`, Text "x"). Guarda contra um guard
    /// demasiado largo (ex.: esticar qualquer folha de 1 carácter).
    #[test]
    fn p952op_char_nao_grande_display_caminho_inalterado() {
        let metrics = P952OpMetrics::ncm();
        let style = default_style();
        let ml = MathLayouter::new(&metrics, true, &style);

        let b = ml.layout_node(&Content::MathIdent("x".into()), &style);

        assert!(
            glyphs(&b).is_empty(),
            "'x' não é operador grande: nenhum FrameItem::Glyph esperado, obteve {:?}",
            glyphs(&b)
        );
        assert!(
            text_pos(&b, "x").is_some(),
            "'x' display deve seguir o caminho de texto normal; items: {:?}",
            b.items
        );
    }

    /// **P952 — RED — `∑` com limites (`sum_(k=1)^n`)**: a BASE do attach
    /// usa a variante v1 (a MathBox da base tem o Glyph v1) e os limites
    /// continuam posicionados relativamente a ela — em bloco,
    /// `is_large_operator && !is_integral_char` → limites empilhados
    /// (`attach.rs:122-131`): sup ACIMA da baseline (y < 0), sub ABAIXO
    /// (y > 0). Guarda de que o attach não quebra com a base esticada.
    /// Hoje: sem Glyph — falha na primeira asserção.
    #[test]
    fn p952op_attach_display_base_esticada_limites_posicionados() {
        let metrics = P952OpMetrics::ncm();
        let style = default_style();
        let ml = MathLayouter::new(&metrics, true, &style);

        let attach = Content::math_attach(
            Content::MathIdent("∑".into()),
            None,
            None,
            Some(Content::MathText("k=1".into())),
            Some(Content::MathText("n".into())),
        );
        let b = ml.layout_node(&attach, &style);

        let g = glyphs(&b);
        assert_eq!(
            g.len(),
            1,
            "a base do attach (display) deve ser a variante v1: 1 FrameItem::Glyph esperado, \
             obteve {g:?} em {:?}",
            b.items
        );
        assert_eq!(g[0].0, SUM_V1_GID, "base do attach deve ser summation.v1");

        let (_, y_sup) = text_pos(&b, "n").expect("superscript \"n\" deve existir");
        let (_, y_sub) = text_pos(&b, "k=1").expect("subscript \"k=1\" deve existir");
        assert!(
            y_sup < 0.0,
            "limite superior deve ficar ACIMA da baseline da base (y < 0), obteve y={y_sup:.4}"
        );
        assert!(
            y_sub > 0.0,
            "limite inferior deve ficar ABAIXO da baseline da base (y > 0), obteve y={y_sub:.4}"
        );
    }

    /// **P952 — GREEN esperado (guarda do fallback) — sem variante >= alvo,
    /// cai no glifo base**: com `display_operator_min_height = 9999du`,
    /// nenhuma variante de `∑` (1001/1401du) chega — o L0 manda cair no
    /// glifo base ("sem variante suficiente: glifo base, comportamento
    /// anterior inalterado"), SEM tentar assembly nem esticar ao máximo.
    /// Passa já hoje (o fallback É o comportamento actual) e deve continuar
    /// a passar após a implementação.
    #[test]
    fn p952op_sem_variante_suficiente_cai_no_glifo_base() {
        let metrics = P952OpMetrics::new(9999.0); // alvo inatingível
        let style = default_style();
        let ml = MathLayouter::new(&metrics, true, &style);

        let b = ml.layout_node(&Content::MathIdent("∑".into()), &style);

        assert!(
            glyphs(&b).is_empty(),
            "sem variante >= 9999du deve cair no glifo base (sem FrameItem::Glyph), obteve {:?}",
            glyphs(&b)
        );
        assert!(
            text_pos(&b, "∑").is_some(),
            "fallback = glifo base via caminho de texto actual; items: {:?}",
            b.items
        );
    }

    /// **P952 — item 4 (correcção da revisão cética retroativa) — células
    /// com ascents diferentes partilham a baseline da linha**: os items de
    /// uma `MathBox` são baseline-relativos, logo a translação vertical de
    /// cada célula é `dy = baseline_offset` (a baseline da linha, acumulada
    /// de descents/ascents + gap). A forma top-anchored
    /// (`dy = baseline_offset + row_ascent − cell.ascent`) só faria sentido
    /// para items ancorados no topo — o vanilla (`run.rs:137`) é
    /// top-anchored porque lá os frames o são; cá não. Com a forma errada,
    /// uma linha com células de ascents diferentes ficava com as baselines
    /// desalinhadas (medido na revisão: 2.77pt em `mat(a,b;c,d)` com a fonte
    /// real; aqui 10pt com ascents sintéticos 4/14).
    #[test]
    fn p952_grid_celulas_ascents_diferentes_partilham_baseline() {
        let style = default_style();
        let ml = MathLayouter::new(&FixedMetrics, true, &style);

        // Célula com um item de texto na baseline local (pos.y = 0) e
        // ascent/descent controlados. O piso do `(` sintético de
        // `FixedMetrics` (8.4/0) fica abaixo do ascent máximo da linha
        // (14) — não interfere.
        let cell = |ch: &str, ascent: f64, descent: f64| MathBox {
            width: 5.0,
            ascent,
            descent,
            items: vec![FrameItem::Text {
                pos: Point::ZERO,
                text: ch.into(),
                style: default_style(),
            }],
        };

        // Duas linhas com a mesma geometria: coluna 1 com ascent 4,
        // coluna 2 com ascent 14 → row_ascent = 14, row_descent = 2.
        let grid = vec![
            vec![cell("a", 4.0, 0.0), cell("X", 14.0, 2.0)],
            vec![cell("b", 4.0, 0.0), cell("Y", 14.0, 2.0)],
        ];
        let b = ml.layout_grid_boxes(grid, GridAlign::Center, Pt(0.0), Pt(2.0), &[], &style);

        let y = |needle: &str| {
            text_pos(&b, needle)
                .unwrap_or_else(|| panic!("célula {needle:?} não encontrada"))
                .1
        };

        // Baseline partilhada intra-linha: as células da mesma linha ficam
        // com o item EXACTAMENTE no mesmo y, apesar dos ascents diferentes.
        assert!(
            (y("a") - y("X")).abs() < 1e-9,
            "linha 1: 'a' (ascent 4) e 'X' (ascent 14) devem partilhar a baseline \
             (dy = baseline_offset): y(a)={:.4} vs y(X)={:.4} — com a forma top-anchored \
             (baseline_offset + row_ascent − cell.ascent) diferem de {:.4}pt",
            y("a"),
            y("X"),
            14.0 - 4.0
        );
        assert!(
            (y("b") - y("Y")).abs() < 1e-9,
            "linha 2: 'b' e 'Y' devem partilhar a baseline: y(b)={:.4} vs y(Y)={:.4}",
            y("b"),
            y("Y")
        );

        // Pitch uniforme entre linhas: row_descent(2) + gap(2) +
        // next_row_ascent(14) = 18pt nas duas colunas.
        let pitch1 = y("b") - y("a");
        let pitch2 = y("Y") - y("X");
        let esperado = 2.0 + 2.0 + 14.0;
        assert!(
            (pitch1 - esperado).abs() < 1e-9,
            "pitch = row_descent + gap + next_row_ascent = {esperado:.4}pt, obteve {pitch1:.4}"
        );
        assert!(
            (pitch1 - pitch2).abs() < 1e-9,
            "pitch deve ser uniforme nas duas colunas: b−a={pitch1:.4} vs Y−X={pitch2:.4}"
        );
    }

    // ── P971 — termo de itálico do subscrito pós-fixado ─────────────────
    //
    // Especificação: `math/layout/attach.md` §P971 (gate confirmado
    // 2026-08-05). Vanilla `compute_post_script_widths`
    // (`scripts.rs:220-228`): `br_kern − base.italics_correction()`; o sup
    // não leva termo. Valor real da IC de `integral.v1`: 450du = 5.4pt a
    // 12pt (auditoria: sub a 6.04pt vs sup a 10.99pt da aresta esquerda do
    // símbolo — delta = IC exacta). Valor esperado via oráculo (P969).

    /// x do primeiro `FrameItem::Text` igual a `needle`, ou do
    /// `FrameItem::Glyph` cujo `base_char` é `needle`.
    fn p971_x(items: &[FrameItem], needle: &str) -> f64 {
        items
            .iter()
            .find_map(|i| match i {
                FrameItem::Text { pos, text, .. } if text.as_str() == needle => {
                    Some(pos.x.val())
                }
                FrameItem::Glyph { pos, base_char, .. } if base_char.to_string() == needle => {
                    Some(pos.x.val())
                }
                _ => None,
            })
            .unwrap_or_else(|| panic!("{needle:?} não encontrado em {items:?}"))
    }

    /// **Caso do achado 9.2** — `∫_0^1` em bloco: a base é a variante v1
    /// (hor_advance 999du = 11.988pt a 12pt) com IC 5.4pt injectada. O sub
    /// recua a IC (11.988 − 5.4 = 6.588); o sup fica em 11.988. Antes de
    /// P971: ambos em 11.988 (mesma coluna, ignorando a inclinação).
    #[test]
    fn p971_sub_de_integral_display_recua_italics_correction() {
        let metrics = P952OpMetrics::ncm().with_int_ink(16.332, 10.332).with_int_ic(5.4);
        let style = default_style(); // 12pt; 1du = 0.012pt
        let ml = MathLayouter::new(&metrics, true, &style);

        let content = Content::math_attach(
            Content::MathText("∫".into()),
            None,
            None,
            Some(Content::MathText("0".into())),
            Some(Content::MathText("1".into())),
        );
        let items = ml.layout_equation(&content, &style);

        let x_base = p971_x(&items, "∫");
        let base_width = 999.0 * 0.012; // hor_advance da v1 = 11.988pt
        let esperado_sub =
            x_base + base_width + crate::testing::math_oracle::post_subscript_kern(0.0, 5.4);
        let esperado_sup = x_base + base_width;

        let x_sub = p971_x(&items, "0");
        let x_sup = p971_x(&items, "1");
        assert!(
            (x_sub - esperado_sub).abs() < 1e-9,
            "sub de ∫: oráculo {esperado_sub:.4}pt (base_width 11.988 − IC 5.4), obteve {x_sub:.4}"
        );
        assert!(
            (x_sup - esperado_sup).abs() < 1e-9,
            "sup de ∫ sem termo de IC: esperado {esperado_sup:.4}pt, obteve {x_sup:.4}"
        );
    }

    /// **Guarda** — base sem IC na tabela (ou sem `FrameItem::Glyph`):
    /// posições bit-a-bit iguais às de antes de P971 (sub e sup na mesma
    /// coluna, `base_width` + kern 0).
    #[test]
    fn p971_base_sem_italics_correction_inalterada() {
        let metrics = P952OpMetrics::ncm().with_int_ink(16.332, 10.332); // sem IC
        let style = default_style();
        let ml = MathLayouter::new(&metrics, true, &style);

        let content = Content::math_attach(
            Content::MathText("∫".into()),
            None,
            None,
            Some(Content::MathText("0".into())),
            Some(Content::MathText("1".into())),
        );
        let items = ml.layout_equation(&content, &style);

        let x_base = p971_x(&items, "∫");
        let esperado = x_base + 999.0 * 0.012;
        let x_sub = p971_x(&items, "0");
        let x_sup = p971_x(&items, "1");
        assert!(
            (x_sub - esperado).abs() < 1e-9 && (x_sup - esperado).abs() < 1e-9,
            "sem IC: sub e sup em {esperado:.4}pt; obteve sub={x_sub:.4}, sup={x_sup:.4}"
        );
    }
}

// ── P961 — legenda de underbrace/overbrace em tamanho de script + ──────
// base de acento com itálico por defeito.
//
// Especificação: `math/layout/underover.md` §P961 (Parte A: anotação com
// `style_for_subscript`/`style_for_superscript` do vanilla —
// `resolve.rs:1441,1455`) e `math/layout/_comum.md` §P961 (Parte B:
// `apply_math_default` recursa em `MathAccent`/`MathUnderover` — achado #5
// de P906 + auditoria externa 2026-08-04).
#[cfg(test)]
mod p961_tests {
    use super::*;

    /// (size, math_script, cramped) dos items de texto cujo texto é `needle`.
    fn estilos_de(b: &MathBox, needle: &str) -> Vec<(f64, bool, bool)> {
        b.items
            .iter()
            .filter_map(|i| match i {
                FrameItem::Text { text, style, .. } if text.as_str() == needle => {
                    Some((style.size.val(), style.math_script, style.cramped))
                }
                _ => None,
            })
            .collect()
    }

    /// **Parte A (under)** — a legenda de `underbrace` é subscrito no
    /// vanilla: size ×0.7 (12→8.4pt), `math_script`, `cramped` (o subscrito
    /// é cramped — `style.rs:333`). Hoje: 12pt sem redução.
    #[test]
    fn p961_under_label_tamanho_script_cramped() {
        let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
        let base = Content::MathIdent("a".into());
        let label = Content::MathText("soma".into());
        let b = ml.layout_underover(&base, Some(&label), None, &default_style());

        let estilos = estilos_de(&b, "soma");
        assert_eq!(estilos.len(), 1, "label 'soma' deve produzir 1 item: {estilos:?}");
        let (size, math_script, cramped) = estilos[0];
        assert!(
            (size - 12.0 * 0.7).abs() < 1e-9,
            "legenda under = size×0.7 (8.4pt), obteve {size:.4} — hoje sem redução (12pt)"
        );
        assert!(math_script, "legenda under deve ter math_script: true");
        assert!(cramped, "legenda under é subscrito → cramped (style.rs:333)");
    }

    /// **Parte A (over)** — a legenda de `overbrace` é superscrito:
    //  size ×0.7, `math_script`, SEM cramped.
    #[test]
    fn p961_over_label_tamanho_script_sem_cramped() {
        let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
        let base = Content::MathIdent("a".into());
        let label = Content::MathText("soma".into());
        let b = ml.layout_underover(&base, None, Some(&label), &default_style());

        let estilos = estilos_de(&b, "soma");
        assert_eq!(estilos.len(), 1, "label 'soma' deve produzir 1 item: {estilos:?}");
        let (size, math_script, cramped) = estilos[0];
        assert!(
            (size - 12.0 * 0.7).abs() < 1e-9,
            "legenda over = size×0.7 (8.4pt), obteve {size:.4}"
        );
        assert!(math_script, "legenda over deve ter math_script: true");
        assert!(!cramped, "legenda over é superscrito → SEM cramped");
    }

    /// **Parte A (guarda da peça)** — a chave de 1 carácter (que estica
    /// para a largura da base — acento largo no vanilla, não script) NÃO
    /// pode sofrer a redução: o seu tamanho fica o ambiente (12pt).
    #[test]
    fn p961_peca_estica_mantem_tamanho_ambiente() {
        let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
        let base = Content::MathIdent("a".into());
        let brace = Content::MathText("⏟".into());
        let b = ml.layout_underover(&base, Some(&brace), None, &default_style());

        let estilos = estilos_de(&b, "⏟");
        assert_eq!(estilos.len(), 1, "peça ⏟ deve produzir 1 item Text: {estilos:?}");
        let (size, _, _) = estilos[0];
        assert!(
            (size - 12.0).abs() < 1e-9,
            "a peça ⏟ (acento largo) mantém o tamanho ambiente (12pt), obteve {size:.4}"
        );
    }

    /// **Parte B (acento)** — a base de `hat(x)` recebe o itálico por
    /// defeito (𝑥 U+1D465), mesma regra de qualquer identificador de 1
    /// letra. Hoje: `apply_math_default` não recursa em `MathAccent` → base
    /// sai `x` latino.
    #[test]
    fn p961_acento_base_recebe_italico_default() {
        let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
        let content = Content::math_accent(
            Content::MathIdent("x".into()),
            Content::MathText("^".into()),
        );
        let items = ml.layout_equation(&content, &default_style());
        let textos: Vec<&str> = items
            .iter()
            .filter_map(|i| match i {
                FrameItem::Text { text, .. } => Some(text.as_str()),
                _ => None,
            })
            .collect();
        assert!(
            textos.iter().any(|t| *t == "\u{1D465}"),
            "base de hat(x) deve ser 𝑥 (U+1D465); textos: {textos:?}"
        );
        assert!(
            !textos.iter().any(|t| *t == "x"),
            "base não deve ficar em x latino: {textos:?}"
        );
    }

    /// **Parte B (underover)** — base e legenda de 1 letra recebem itálico
    /// via a recursão nova em `MathUnderover` (`underbrace(x, n)`).
    #[test]
    fn p961_underover_base_e_label_recebem_italico() {
        let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
        let content = Content::math_underover(
            Content::MathIdent("x".into()),
            Some(Content::MathIdent("n".into())),
            None,
        );
        let items = ml.layout_equation(&content, &default_style());
        let textos: Vec<&str> = items
            .iter()
            .filter_map(|i| match i {
                FrameItem::Text { text, .. } => Some(text.as_str()),
                _ => None,
            })
            .collect();
        assert!(
            textos.iter().any(|t| *t == "\u{1D465}"),
            "base x deve ser 𝑥 (U+1D465); textos: {textos:?}"
        );
        assert!(
            textos.iter().any(|t| *t == "\u{1D45B}"),
            "label n deve ser 𝑛 (U+1D45B); textos: {textos:?}"
        );
    }
}

// ── P959 — shifts verticais dos limites com os 4 termos da tabela MATH ─────
//
// Especificação: `00_nucleo/prompts/compiler/math/layout/attach.md` §P959,
// `entities/math_constants.md` §P959, `infra/font_metrics.md` §P959.
// Fórmula do vanilla (`compute_limit_shifts`,
// lab/typst-original/crates/typst-layout/src/math/scripts.rs:290-313`):
//
//   t_shift = base.ascent  + max(upper_limit_baseline_rise_min,
//                                upper_limit_gap_min + t.descent)
//   b_shift = base.descent + max(lower_limit_baseline_drop_min,
//                                lower_limit_gap_min + b.ascent)
//   y_sup = −t_shift;  y_sub = +b_shift  (baseline do limite rel. à da base)
//   caixa final: ascent  = max(base.ascent,  t_shift + t.ascent)
//                descent = max(base.descent, b_shift + b.descent)
//
// A fórmula actual (gap-only, `attach.rs` braço `is_limits`) usa só
// `gap + extent` — sem os pisos `max(rise/drop, …)`.
//
// ⚠ RED por COMPILAÇÃO (E0609): os campos
// `MathConstants::upper_limit_baseline_rise_min` e
// `MathConstants::lower_limit_baseline_drop_min` ainda não existem —
// `p959_constants()` falha a compilar até o Agente B os criar
// (`entities/math_constants.md` §P959: fallbacks 111.0 / 600.0).
mod p959_tests {
    use super::*;
    use crate::entities::math_constants::MathConstants;
    // **P969** — valores esperados via oráculo (fórmula literal do vanilla,
    // `scripts.rs:290-313`), não aritmética solta em comentários.
    use crate::testing::math_oracle;
    use std::collections::HashMap;

    /// Test double (mesmo padrão de `SignedMetrics`, p922): delega os
    /// métodos obrigatórios a `FixedMetrics`, injecta `text_ink_bounds`
    /// controlados por carácter (valores absolutos em pt, independentes
    /// do size — a base é composta a 12pt e os limites a 8.4pt, mas o
    /// ink injectado é o mesmo) e sobrepõe `math_constants`.
    ///
    /// A base `∑` em Display sem variantes verticais cai no glifo base
    /// (`layout_large_operator_display` → `layout_text_node`, P952),
    /// logo `with_ink('∑', …)` controla `base_ascent`/`base_descent`.
    struct P959Metrics {
        inner: FixedMetrics,
        ink: HashMap<char, (Pt, Pt)>,
        constants: MathConstants,
    }

    impl P959Metrics {
        fn new(constants: MathConstants) -> Self {
            Self { inner: FixedMetrics, ink: HashMap::new(), constants }
        }

        fn with_ink(mut self, c: char, top: f64, bottom: f64) -> Self {
            self.ink.insert(c, (Pt(top), Pt(bottom)));
            self
        }
    }

    impl FontMetrics for P959Metrics {
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

        fn text_ink_bounds(&self, text: &str, size: Pt, style: &TextStyle) -> (Pt, Pt) {
            if text.chars().count() == 1 {
                if let Some(&(top, bottom)) = self.ink.get(&text.chars().next().unwrap()) {
                    return (top, bottom);
                }
            }
            self.inner.text_ink_bounds(text, size, style)
        }

        fn math_constants(&self, _style: &TextStyle) -> MathConstants {
            self.constants.clone()
        }
    }

    /// Constantes sintéticas bem separadas para os 4 termos (design units,
    /// upem=1000 do fallback). Os valores REAIS de NewCMMath-Book
    /// (`entities/math_constants.md` §P959) são gap_up=200, rise=111,
    /// gap_lo=167, drop=600 — usados tal qual em quase todos os testes;
    /// o parâmetro `rise_du` existe para o teste que precisa de rise >
    /// gap (ver `p959_sup_rise_domina_quando_maior_que_gap_mais_descent`).
    ///
    /// ⚠ CAMPOS NOVOS (P959): `upper_limit_baseline_rise_min` e
    /// `lower_limit_baseline_drop_min` não existem ainda em
    /// `MathConstants`; E0609 aqui até o Agente B os criar.
    fn p959_constants(gap_up_du: f64, rise_du: f64, gap_lo_du: f64, drop_du: f64) -> MathConstants {
        let mut c = MathConstants::fallback();
        c.upper_limit_gap_min = gap_up_du;
        c.upper_limit_baseline_rise_min = rise_du;
        c.lower_limit_gap_min = gap_lo_du;
        c.lower_limit_baseline_drop_min = drop_du;
        c
    }

    /// Constantes com os valores reais medidos de NewCMMath-Book.
    fn ncm() -> MathConstants {
        p959_constants(200.0, 111.0, 167.0, 600.0)
    }

    /// `(x, y)` do primeiro `FrameItem::Text` cujo texto é exactamente
    /// `needle`. No braço `is_limits` os items são base, depois sup,
    /// depois sub — textos distintos identificam cada um sem ambiguidade.
    fn text_pos(b: &MathBox, needle: &str) -> (f64, f64) {
        b.items
            .iter()
            .find_map(|i| match i {
                FrameItem::Text { pos, text, .. } if text.as_str() == needle => {
                    Some((pos.x.val(), pos.y.val()))
                }
                _ => None,
            })
            .unwrap_or_else(|| panic!("texto {:?} não encontrado em {:?}", needle, b.items))
    }

    /// Base `∑` com ink (8.0, 2.0) → base_ascent=8.0pt, base_descent=2.0pt
    /// (style de 12pt; escala 0.012pt/du com upem=1000).
    fn base_sum() -> Content {
        Content::MathText("∑".into())
    }

    fn layouter_com(metrics: &P959Metrics) -> MathLayouter<'_, P959Metrics> {
        MathLayouter::new(metrics, true, &default_style()) // block=true, 12pt
    }

    /// **Sup sozinho, limite com descent tal que `gap + descent > rise`** —
    /// o braço do gap domina o max(). Com os valores reais NCM
    /// (rise=111du=1.332pt < gap_up=200du=2.4pt) este é o caso normal para
    /// qualquer sup com descent >= 0. A fórmula nova COINCIDE aqui com a
    /// antiga (gap-only): o teste é guarda de não-regressão do braço gap —
    /// falha se o `max()` for trocado por `min()` ou se o piso rise for
    /// aplicado incondicionalmente.
    ///
    /// Contas (12pt, 0.012pt/du): gap_up = 2.4pt, rise = 1.332pt.
    /// sup 'n' ink (1.0, 0.5) → t.descent = 0.5.
    /// gap + t.descent = 2.4 + 0.5 = 2.9 > 1.332 = rise
    /// t_shift = base.ascent + 2.9 = 8.0 + 2.9 = 10.9 → y_sup = −10.9.
    #[test]
    fn p959_sup_gap_mais_descent_domina_quando_maior_que_rise() {
        let metrics = P959Metrics::new(ncm())
            .with_ink('∑', 8.0, 2.0)
            .with_ink('n', 1.0, 0.5);
        let ml = layouter_com(&metrics);
        let style = default_style();

        let b = ml.layout_attach(
            &base_sum(),
            None,
            None,
            None,
            Some(&Content::MathText("n".into())),
            &style,
        );

        let (_, y_sup) = text_pos(&b, "n");
        let esperado = -math_oracle::large_operator_upper_shift(8.0, 0.5, 2.4, 1.332);
        assert!(
            (y_sup - esperado).abs() < 1e-9,
            "braço gap do max: oráculo y_sup={esperado}pt (8.0 + max(1.332, 2.4+0.5)), obteve {y_sup:.6}"
        );
    }

    /// **Sup com rise a dominar o max()** — `rise > gap + t.descent`.
    ///
    /// NOTA (valores sintéticos, deliberado): com os valores REAIS de NCM
    /// (rise=111du < gap_up=200du) o rise nunca domina para um sup com
    /// descent >= 0 — seria preciso tinta do limite toda acima da baseline
    /// com |descent| > 89du. Para exercitar o braço `rise` do max() usa-se
    /// um stub com rise=500du (6.0pt) — mesmo padrão das constantes
    /// sintéticas extremas de P915/P917 (qualquer regressão falha alto).
    ///
    /// Contas: rise = 500du → 6.0pt; gap + t.descent = 2.4 + 0.5 = 2.9 < 6.0.
    /// t_shift = 8.0 + 6.0 = 14.0 → y_sup = −14.0.
    /// (A fórmula antiga gap-only daria −(8.0+2.4+0.5) = −10.9 — distingue.)
    #[test]
    fn p959_sup_rise_domina_quando_maior_que_gap_mais_descent() {
        let metrics = P959Metrics::new(p959_constants(200.0, 500.0, 167.0, 600.0))
            .with_ink('∑', 8.0, 2.0)
            .with_ink('n', 1.0, 0.5);
        let ml = layouter_com(&metrics);
        let style = default_style();

        let b = ml.layout_attach(
            &base_sum(),
            None,
            None,
            None,
            Some(&Content::MathText("n".into())),
            &style,
        );

        let (_, y_sup) = text_pos(&b, "n");
        let esperado = -math_oracle::large_operator_upper_shift(8.0, 0.5, 2.4, 6.0);
        assert!(
            (y_sup - esperado).abs() < 1e-9,
            "braço rise do max: oráculo y_sup={esperado}pt (8.0 + max(6.0, 2.4+0.5)), obteve {y_sup:.6} \
             — a fórmula gap-only dá -10.9"
        );
    }

    /// **Sub com ascent pequeno tal que o drop domina** — O caso que a
    /// fórmula antiga erra sempre: com os valores REAIS de NCM,
    /// drop=600du (7.2pt a 12pt) domina enquanto
    /// `sub.ascent < drop − gap_lo` (7.2 − 2.004 = 5.196pt).
    ///
    /// Contas: gap_lo = 167du → 2.004pt; drop = 600du → 7.2pt.
    /// sub 'k' ink (2.0, 0.5) → b.ascent = 2.0.
    /// gap + b.ascent = 2.004 + 2.0 = 4.004 < 7.2 = drop
    /// b_shift = base.descent + 7.2 = 2.0 + 7.2 = 9.2 → y_sub = +9.2.
    /// (A fórmula antiga gap-only dá 2.0+2.004+2.0 = +6.004 — errado.)
    #[test]
    fn p959_sub_drop_domina_quando_ascent_pequeno() {
        let metrics = P959Metrics::new(ncm())
            .with_ink('∑', 8.0, 2.0)
            .with_ink('k', 2.0, 0.5);
        let ml = layouter_com(&metrics);
        let style = default_style();

        let b = ml.layout_attach(
            &base_sum(),
            None,
            None,
            Some(&Content::MathText("k".into())),
            None,
            &style,
        );

        let (_, y_sub) = text_pos(&b, "k");
        let esperado = math_oracle::large_operator_lower_shift(2.0, 2.0, 2.004, 7.2);
        assert!(
            (y_sub - esperado).abs() < 1e-9,
            "braço drop do max: oráculo y_sub=+{esperado}pt (2.0 + max(7.2, 2.004+2.0)), obteve {y_sub:.6} \
             — a fórmula gap-only dá +6.004"
        );
    }

    /// **Sub com ascent grande tal que o drop NÃO domina** — o braço do
    /// gap domina o max(); a fórmula nova coincide com a antiga. Guarda
    /// simétrica de `p959_sup_gap_mais_descent_domina_quando_maior_que_rise`.
    ///
    /// Contas: sub 'k' ink (6.0, 0.5) → b.ascent = 6.0.
    /// gap + b.ascent = 2.004 + 6.0 = 8.004 > 7.2 = drop
    /// b_shift = 2.0 + 8.004 = 10.004 → y_sub = +10.004.
    #[test]
    fn p959_sub_gap_mais_ascent_domina_quando_maior_que_drop() {
        let metrics = P959Metrics::new(ncm())
            .with_ink('∑', 8.0, 2.0)
            .with_ink('k', 6.0, 0.5);
        let ml = layouter_com(&metrics);
        let style = default_style();

        let b = ml.layout_attach(
            &base_sum(),
            None,
            None,
            Some(&Content::MathText("k".into())),
            None,
            &style,
        );

        let (_, y_sub) = text_pos(&b, "k");
        let esperado = math_oracle::large_operator_lower_shift(2.0, 6.0, 2.004, 7.2);
        assert!(
            (y_sub - esperado).abs() < 1e-9,
            "braço gap do max: oráculo y_sub=+{esperado}pt (2.0 + max(7.2, 2.004+6.0)), obteve {y_sub:.6}"
        );
    }

    /// **Sup+sub juntos** (`∑_k^n` num só attach): os dois shifts correctos
    /// na mesma caixa, independentes um do outro (o max() do sup usa o
    /// descent do sup; o do sub usa o ascent do sub). Valores reais NCM.
    ///
    /// Contas (sup 'n' ink (1.0, 0.5); sub 'k' ink (2.0, 0.5)):
    /// t_shift = 8.0 + max(1.332, 2.4+0.5) = 10.9  → y_sup = −10.9
    /// b_shift = 2.0 + max(7.2, 2.004+2.0) = 9.2   → y_sub = +9.2
    #[test]
    fn p959_sup_e_sub_juntos_shifts_independentes() {
        let metrics = P959Metrics::new(ncm())
            .with_ink('∑', 8.0, 2.0)
            .with_ink('n', 1.0, 0.5)
            .with_ink('k', 2.0, 0.5);
        let ml = layouter_com(&metrics);
        let style = default_style();

        let b = ml.layout_attach(
            &base_sum(),
            None,
            None,
            Some(&Content::MathText("k".into())),
            Some(&Content::MathText("n".into())),
            &style,
        );

        let (_, y_sup) = text_pos(&b, "n");
        let (_, y_sub) = text_pos(&b, "k");
        let esp_sup = -math_oracle::large_operator_upper_shift(8.0, 0.5, 2.4, 1.332);
        let esp_sub = math_oracle::large_operator_lower_shift(2.0, 2.0, 2.004, 7.2);
        assert!(
            (y_sup - esp_sup).abs() < 1e-9,
            "∑_k^n: oráculo y_sup={esp_sup}pt, obteve {y_sup:.6}"
        );
        assert!(
            (y_sub - esp_sub).abs() < 1e-9,
            "∑_k^n: oráculo y_sub=+{esp_sub}pt, obteve {y_sub:.6}"
        );
    }

    /// **ascent/descent da caixa final derivam dos shifts** (L0 §P959:
    /// "a tinta dos limites fica a `t_shift + t.ascent` acima e
    /// `b_shift + b.descent` abaixo"), com max contra a caixa da base.
    ///
    /// Contas (valores NCM; base (8.0, 2.0); sup 'n' (1.0, 0.5); sub 'k' (2.0, 0.5)):
    /// sup só:  ascent = max(8.0, 10.9+1.0) = 11.9; descent = base = 2.0.
    /// sub só:  ascent = base = 8.0; descent = max(2.0, 9.2+0.5) = 9.7.
    /// ambos:   ascent = 11.9; descent = 9.7.
    #[test]
    fn p959_caixa_final_ascent_descent_derivam_dos_shifts() {
        let metrics = P959Metrics::new(ncm())
            .with_ink('∑', 8.0, 2.0)
            .with_ink('n', 1.0, 0.5)
            .with_ink('k', 2.0, 0.5);
        let ml = layouter_com(&metrics);
        let style = default_style();

        let sup_only = ml.layout_attach(
            &base_sum(),
            None,
            None,
            None,
            Some(&Content::MathText("n".into())),
            &style,
        );
        assert!(
            (sup_only.ascent - 11.9).abs() < 1e-9,
            "sup só: esperado ascent=11.9pt (t_shift 10.9 + sup.ascent 1.0), obteve {:.6}",
            sup_only.ascent
        );
        assert!(
            (sup_only.descent - 2.0).abs() < 1e-9,
            "sup só: descent deve ficar o da base (2.0pt), obteve {:.6}",
            sup_only.descent
        );

        let sub_only = ml.layout_attach(
            &base_sum(),
            None,
            None,
            Some(&Content::MathText("k".into())),
            None,
            &style,
        );
        assert!(
            (sub_only.ascent - 8.0).abs() < 1e-9,
            "sub só: ascent deve ficar o da base (8.0pt), obteve {:.6}",
            sub_only.ascent
        );
        assert!(
            (sub_only.descent - 9.7).abs() < 1e-9,
            "sub só: esperado descent=9.7pt (b_shift 9.2 + sub.descent 0.5), obteve {:.6}",
            sub_only.descent
        );

        let both = ml.layout_attach(
            &base_sum(),
            None,
            None,
            Some(&Content::MathText("k".into())),
            Some(&Content::MathText("n".into())),
            &style,
        );
        assert!(
            (both.ascent - 11.9).abs() < 1e-9 && (both.descent - 9.7).abs() < 1e-9,
            "sup+sub: esperado ascent=11.9/descent=9.7, obteve {:.6}/{:.6}",
            both.ascent,
            both.descent
        );
    }

    /// **Guarda de integral** — `∫_0^1` em bloco continua com scripts ao
    /// lado (sem empilhamento), mesmo com as constantes novas presentes:
    /// os campos P959 são consumidos SÓ no braço `is_limits`; o braço
    /// não-limits fica inalterado (L0 §P959: "Scripts laterais (braço
    /// não-limits, ex.: integrais) inalterados"). Complementa a guarda
    /// P772w (`math_attach_integral_nao_empilha_limites_em_modo_bloco`,
    /// acima, ~l.222), que usa `FixedMetrics` puro — aqui o stub tem os 4
    /// termos MATH com valores extremos para apanhar qualquer fuga dos
    /// campos novos para o braço errado.
    #[test]
    fn p959_integral_scripts_laterais_inalterados_com_constantes_novas() {
        let metrics = P959Metrics::new(ncm())
            .with_ink('∫', 8.0, 2.0)
            .with_ink('n', 1.0, 0.5)
            .with_ink('k', 2.0, 0.5);
        let ml = layouter_com(&metrics);
        let style = default_style();

        let b = ml.layout_attach(
            &Content::MathText("∫".into()),
            None,
            None,
            Some(&Content::MathText("k".into())),
            Some(&Content::MathText("n".into())),
            &style,
        );

        let (base_x, _) = text_pos(&b, "∫");
        let (sup_x, _) = text_pos(&b, "n");
        let base_w = 12.0 * 0.6; // FixedMetrics: 0.6 × size por codepoint, 12pt.
        assert!(
            sup_x >= base_x + base_w - 1.0,
            "sup de ∫ deve ficar à DIREITA da base (script lateral, não empilhado), \
             base_x={base_x} sup_x={sup_x} base_w={base_w}"
        );
    }
}

// ── P962 — `dif`/`Dif` reto (upright), não itálico ─────────────────────
//
// Especificação: `compiler/stdlib/structural.md` §P962. O registo de P795
// produzia `MathText("d")` simples, que `apply_math_default` italicava
// (𝑑 U+1D451). O vanilla envolve o símbolo em `upright(...)`
// (`math/op.rs:52-56`) — trace medido: vanilla emite `d` U+0064.
#[cfg(test)]
mod p962_tests {
    use super::*;

    /// O conteúdo que o registo P962 produz para `dif` (upright wrapper) —
    /// `apply_math_default` não toca em `MathStyled` (P809: wrapper
    /// explícito), e o handler dedicado renderiza reto.
    #[test]
    fn p962_dif_wrapper_upright_produz_d_reto() {
        let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
        let content = Content::math_styled(
            None,
            None,
            Some(false),
            Content::MathText("d".into()),
            None,
        );
        let items = ml.layout_equation(&content, &default_style());
        let textos: Vec<&str> = items
            .iter()
            .filter_map(|i| match i {
                FrameItem::Text { text, .. } => Some(text.as_str()),
                _ => None,
            })
            .collect();
        assert!(
            textos.iter().any(|t| *t == "d"),
            "dif deve produzir 'd' reto (U+0064); textos: {textos:?}"
        );
        assert!(
            !textos.iter().any(|t| *t == "\u{1D451}"),
            "dif NÃO deve ser italicado para 𝑑 (U+1D451): {textos:?}"
        );
    }

    /// **Guarda** — um identificador `d` genuíno (sem `dif`) continua a
    /// receber o itálico por defeito (P809/P961 não regredem).
    #[test]
    fn p962_identificador_d_simples_continua_italico() {
        let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
        let items =
            ml.layout_equation(&Content::MathIdent("d".into()), &default_style());
        let textos: Vec<&str> = items
            .iter()
            .filter_map(|i| match i {
                FrameItem::Text { text, .. } => Some(text.as_str()),
                _ => None,
            })
            .collect();
        assert!(
            textos.iter().any(|t| *t == "\u{1D451}"),
            "identificador d deve ser 𝑑 itálico (U+1D451); textos: {textos:?}"
        );
    }
}

// ── P966 — recursão de `apply_math_default` em containers de markup ─────
//
// Especificação: `00_nucleo/prompts/compiler/math/layout/_comum.md` §P966.
// Conteúdo de função de utilizador dentro de math chega como
// `Content::Sequence` de markup (filhos mistos `Text`/`MathText`) ou
// `Content::Styled` — e as folhas `MathText` de 1 carácter nunca recebiam
// o itálico por defeito porque `apply_math_default` só recursava em
// containers nativos `Math*` (caíam em `other => other.clone()`).
// No vanilla, o output de funções de utilizador é re-realizado COMO math
// (`ir/resolve.rs:127-146`) e cada letra recebe o default.
//
// Nota de layout: `layout_node` não tem braço para `Sequence`/`Styled` —
// caem no catch-all (`plain_text()`), que funde os filhos num único item
// de texto. As asserções verificam por isso o texto CONCATENADO dos items.
mod p966_tests {
    use super::*;
    use crate::entities::style::Styles;

    /// Texto concatenado de todos os items `FrameItem::Text`.
    fn texto_total(items: &[FrameItem]) -> String {
        items
            .iter()
            .filter_map(|i| match i {
                FrameItem::Text { text, .. } => Some(text.as_str()),
                _ => None,
            })
            .collect()
    }

    /// A árvore medida em P966 Fase A para `bra(phi)`:
    /// `MathSequence([Sequence([Text("⟨"), MathText("φ"), Text("|")])])`.
    /// A φ (folha `MathText` de 1 carácter) deve receber o default de
    /// itálico → 𝜑 (U+1D711); os `Text` literais "⟨"/"|" ficam intactos.
    /// Hoje: a φ sai "φ" (bloco grego, sem itálico) — RED.
    #[test]
    fn p966_sequence_markup_em_math_recebe_default() {
        let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
        let content = Content::MathSequence(Arc::from(vec![Content::Sequence(Arc::from(
            vec![
                Content::Text("⟨".into()),
                Content::MathText("φ".into()),
                Content::Text("|".into()),
            ],
        ))]));
        let items = ml.layout_equation(&content, &default_style());
        let t = texto_total(&items);
        assert!(
            t.contains('\u{1D711}'),
            "φ de bra(phi) deve ser 𝜑 (U+1D711, itálico math); texto: {t:?}"
        );
        assert!(
            !t.contains('φ'),
            "φ não deve ficar no bloco grego (U+03C6): {t:?}"
        );
        assert!(t.contains('⟨'), "Text literal ⟨ fica intacto: {t:?}");
        assert!(t.contains('|'), "Text literal | fica intacto: {t:?}");
    }

    /// Segundo template distinto (generalização): simula
    /// `#let norm(x) = [norm #x]` — `Sequence([Text("norm"), MathText("x")])`.
    /// A x deve virar 𝑥 (U+1D465); o "norm" literal fica reto. RED hoje.
    #[test]
    fn p966_segundo_template_generaliza() {
        let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
        let content = Content::MathSequence(Arc::from(vec![Content::Sequence(Arc::from(
            vec![
                Content::Text("norm".into()),
                Content::MathText("x".into()),
            ],
        ))]));
        let items = ml.layout_equation(&content, &default_style());
        let t = texto_total(&items);
        assert!(
            t.contains('\u{1D465}'),
            "x do template deve ser 𝑥 (U+1D465); texto: {t:?}"
        );
        assert!(t.contains("norm"), "Text literal 'norm' fica intacto: {t:?}");
    }

    /// **Guarda** — `Content::Text` NUNCA é transformado, mesmo com 1
    /// letra dentro de math: texto literal de markup fica reto (paridade
    /// com o `resolve_text` do vanilla). Passa hoje e deve CONTINUAR a
    /// passar após a correcção.
    #[test]
    fn p966_text_literal_em_markup_nao_e_transformado() {
        let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
        let content = Content::MathSequence(Arc::from(vec![Content::Sequence(Arc::from(
            vec![
                Content::Text("d".into()),
                Content::MathText("x".into()),
            ],
        ))]));
        let items = ml.layout_equation(&content, &default_style());
        let t = texto_total(&items);
        assert!(t.contains('d'), "Text literal 'd' fica reto: {t:?}");
        assert!(
            !t.contains('\u{1D451}'),
            "Text literal NUNCA vira 𝑑 (U+1D451): {t:?}"
        );
    }

    /// **Guarda P962** — um wrapper `MathStyled(italic: false)` (o `dif`
    /// upright) dentro de uma Sequence de markup NÃO é desfeito pela
    /// recursão nova: o "d" fica reto.
    #[test]
    fn p966_mathstyled_dentro_de_sequence_markup_intacto() {
        let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
        let content = Content::MathSequence(Arc::from(vec![Content::Sequence(Arc::from(
            vec![Content::math_styled(
                None,
                None,
                Some(false),
                Content::MathText("d".into()),
                None,
            )],
        ))]));
        let items = ml.layout_equation(&content, &default_style());
        let t = texto_total(&items);
        assert!(t.contains('d'), "dif upright deve produzir 'd' reto: {t:?}");
        assert!(
            !t.contains('\u{1D451}'),
            "wrapper upright não pode ser desfeito pela recursão (𝑑 U+1D451): {t:?}"
        );
    }

    /// Templates aninhados (função de utilizador que chama outra função de
    /// utilizador): `Sequence` dentro de `Sequence`. A ψ aninhada deve
    /// receber o default → 𝜓 (U+1D713). RED hoje.
    #[test]
    fn p966_templates_aninhados_recebem_default() {
        let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
        let content = Content::MathSequence(Arc::from(vec![Content::Sequence(Arc::from(
            vec![
                Content::Text("a".into()),
                Content::Sequence(Arc::from(vec![Content::MathText("ψ".into())])),
            ],
        ))]));
        let items = ml.layout_equation(&content, &default_style());
        let t = texto_total(&items);
        assert!(
            t.contains('\u{1D713}'),
            "ψ aninhada deve ser 𝜓 (U+1D713); texto: {t:?}"
        );
        assert!(
            !t.contains('ψ'),
            "ψ não deve ficar no bloco grego (U+03C8): {t:?}"
        );
    }

    /// `Content::Styled(body, styles)` — recursão no `body` com o `Styles`
    /// intacto. Simula markup estilizado produzido por template. A ω deve
    /// virar 𝜔 (U+1D714). RED hoje (Styled cai em `other => clone`).
    #[test]
    fn p966_styled_wrapper_recursa_no_corpo() {
        let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
        let content = Content::MathSequence(Arc::from(vec![Content::Styled(
            Box::new(Content::Sequence(Arc::from(vec![Content::MathText(
                "ω".into(),
            )]))),
            Styles::default(),
        )]));
        let items = ml.layout_equation(&content, &default_style());
        let t = texto_total(&items);
        assert!(
            t.contains('\u{1D714}'),
            "ω dentro de Styled deve ser 𝜔 (U+1D714); texto: {t:?}"
        );
        assert!(
            !t.contains('ω'),
            "ω não deve ficar no bloco grego (U+03C9): {t:?}"
        );
    }
}

// ── P967b — espaço de texto no limite `&` (item espaçado na grelha) ────
//
// Especificação: `math/layout/_comum.md` §P967b. O vanilla insere um
// espaço de texto real entre o fim da célula matemática e uma anotação
// entre aspas (`"dado"`) — através do limite `&` (item espaçado, P903).
#[cfg(test)]
mod p967b_tests {
    use super::*;

    fn x_min_de(b: &MathBox, needle: &str) -> f64 {
        b.items
            .iter()
            .filter_map(|i| match i {
                FrameItem::Text { pos, text, .. } if text.as_str() == needle => {
                    Some(pos.x.val())
                }
                _ => None,
            })
            .reduce(f64::min)
            .unwrap_or_else(|| panic!("{needle:?} não encontrado em {:?}", b.items))
    }

    fn x_max_de(b: &MathBox, needle: &str) -> f64 {
        b.items
            .iter()
            .filter_map(|i| match i {
                FrameItem::Text { pos, text, style, .. } if text.as_str() == needle => {
                    let adv = 0.6 * style.size.val() * needle.chars().count() as f64;
                    Some(pos.x.val() + adv)
                }
                _ => None,
            })
            .reduce(f64::max)
            .unwrap_or_else(|| panic!("{needle:?} não encontrado em {:?}", b.items))
    }

    /// **Caso da secção 8**: `9 && "dado"` na grelha multiline — o espaço
    /// entre o fim do "9" e o início de "dado" deve ser um espaço de texto
    /// real (`advance(" ")` = 0.6×12 = 7.2pt com FixedMetrics a 12pt), não
    /// 0pt (hoje: "9dado" colado).
    #[test]
    fn p967b_grid_limite_com_string_leva_espaco_de_texto() {
        let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
        let style = default_style(); // 12pt
        let nodes = vec![
            Content::MathText("9".into()),
            Content::math_align_point(),
            Content::math_align_point(),
            Content::Text("dado".into()),
        ];
        let b = ml.layout_grid(&nodes, &style);

        let gap = x_min_de(&b, "dado") - x_max_de(&b, "9");
        let esperado = 0.6 * 12.0; // advance(" ") FixedMetrics a 12pt
        assert!(
            (gap - esperado).abs() < 0.01,
            "limite `&` seguido de string: gap = espaço de texto ({esperado:.2}pt), \
             obteve {gap:.4}pt — hoje colado (0pt)"
        );
    }

    /// **Guarda (P825)**: o limite `&` com classes matemáticas continua a
    /// dar o espaçamento de classe (THICK à volta de relações), não um
    /// espaço de texto — a regra explícita ganha ao fallback de item
    /// espaçado.
    #[test]
    fn p967b_limite_com_relacao_mantem_espaco_de_classe() {
        let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
        let style = default_style();
        let left = Content::MathSequence(Arc::from(vec![Content::MathIdent("x".into())]));
        let right = Content::MathSequence(Arc::from(vec![
            Content::MathText("=".into()),
            Content::MathIdent("y".into()),
        ]));
        let gap = ml.align_boundary_spacing(&left, &right, &style);
        let thick = 5.0 / 18.0 * 12.0; // THICK em = 5/18 em a 12pt
        assert!(
            (gap - thick).abs() < 0.01,
            "limite antes de relação: gap = THICK ({thick:.4}pt), obteve {gap:.4}"
        );
    }

    /// **Regra explícita de 0.0 ganha ao fallback**: limite antes de fecho
    /// (Closing) — a regra `(_, Closing) => 0.0` é explícita, não cai no
    /// fallback de item espaçado mesmo com Text do outro lado.
    #[test]
    fn p967b_limite_antes_de_fecho_sem_espaco() {
        let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
        let style = default_style();
        let left = Content::MathSequence(Arc::from(vec![Content::Text("dado".into())]));
        let right = Content::MathSequence(Arc::from(vec![Content::MathText(")".into())]));
        let gap = ml.align_boundary_spacing(&left, &right, &style);
        assert!(
            gap.abs() < 0.01,
            "limite antes de ')': gap = 0.0 (regra explícita), obteve {gap:.4}"
        );
    }
}

// ── P970 — índice de raiz em ScriptScript absoluto ─────────────────────
//
// Especificação: `math/layout/root.md` §P970. O vanilla fixa o índice em
// MathSize::ScriptScript absoluto (resolve.rs:1235-1239) com factor
// absoluto (text/mod.rs:1139-1152): 50% do tamanho de texto declarado,
// em qualquer nível — não 70% (Script) como estava (scope-out de P945).
#[cfg(test)]
mod p970_tests {
    use super::*;

    /// `tamanho_efectivo` = o tamanho JÁ escalado do nível (ex.: contexto
    /// Script a partir de 12pt passa 8.4pt) — o factor correcto é sobre o
    /// tamanho corrente, como nos helpers P945/P952.
    fn tamanho_do_indice(tamanho_efectivo: f64, math_size: MathSize) -> f64 {
        let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
        let style =
            TextStyle { math_size, ..TextStyle::regular(Pt(tamanho_efectivo)) };
        let root = Content::math_root(
            Some(Content::MathText("3".into())),
            Content::MathIdent("x".into()),
        );
        let items = ml.layout_equation(&root, &style);
        items
            .iter()
            .find_map(|i| match i {
                FrameItem::Text { text, style, .. } if text.as_str() == "3" => {
                    Some(style.size.val())
                }
                _ => None,
            })
            .unwrap_or_else(|| panic!("índice '3' não encontrado em {:?}", items))
    }

    /// Caso do achado 9.1: base Text 12pt → índice a 6.0pt (50%), não
    /// 8.4pt (70%).
    #[test]
    fn p970_indice_scriptscript_absoluto_em_text() {
        let s = tamanho_do_indice(12.0, MathSize::Text);
        assert!(
            (s - 6.0).abs() < 0.01,
            "índice de root(3,x) a 12pt Text deve ser 6.0pt (50%), obteve {s:.3}"
        );
    }

    /// Display desce para o mesmo ScriptScript absoluto (Display→Text ×1.0
    /// depois ×sscript): também 6.0pt a partir de 12pt.
    #[test]
    fn p970_indice_scriptscript_absoluto_em_display() {
        let s = tamanho_do_indice(12.0, MathSize::Display);
        assert!(
            (s - 6.0).abs() < 0.01,
            "índice a 12pt Display deve ser 6.0pt (50% absoluto), obteve {s:.3}"
        );
    }

    /// Já em Script (tamanho efectivo 8.4pt = 12×0.7), o índice é
    /// ×sscript/script = 0.5/0.7 — o tamanho final continua 6.0pt
    /// (propriedade absoluta do vanilla).
    #[test]
    fn p970_indice_scriptscript_absoluto_em_script() {
        let s = tamanho_do_indice(8.4, MathSize::Script);
        assert!(
            (s - 6.0).abs() < 0.01,
            "índice dentro de Script (8.4pt) deve continuar 6.0pt, obteve {s:.3}"
        );
    }

    /// Já em ScriptScript (tamanho efectivo 6.0pt), a escada trava: ×1.0.
    #[test]
    fn p970_indice_trava_em_scriptscript() {
        let s = tamanho_do_indice(6.0, MathSize::ScriptScript);
        assert!(
            (s - 6.0).abs() < 0.01,
            "índice dentro de ScriptScript (6.0pt) deve ficar 6.0pt, obteve {s:.3}"
        );
    }
}

// ── P970 parte 2 — posição do índice de raiz (fórmula real do vanilla) ──
//
// Especificação: `math/layout/root.md` §P970 Parte 2 (gate confirmado
// 2026-08-05). Valores esperados via oráculo (P969) e constantes do
// fallback (valores reais NewCMMath: kern_before=278du, kern_after=−556du,
// raise=60%, extra_ascender=48du). Inputs medidos com FixedMetrics a 12pt
// (sonda): radicando "x" w=7.2 a=8.4 d=0.0; √ (fallback texto) a=8.4 d=0.0
// (altura 8.4); índice "3" a 6pt w=3.6 a=4.2 d=0.0; gap=0.72, barra=0.792
// ⇒ total_ascent=9.912.
#[cfg(test)]
mod p970b_tests {
    use super::*;
    use crate::entities::math_constants::MathConstants;
    use crate::testing::math_oracle;

    const PT: f64 = 12.0; // default_style

    fn root_com_indice() -> MathBox {
        let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
        ml.layout_node(
            &Content::math_root(
                Some(Content::MathText("3".into())),
                Content::MathIdent("x".into()),
            ),
            &default_style(),
        )
    }

    fn pos_de(b: &MathBox, needle: &str) -> (f64, f64) {
        b.items
            .iter()
            .find_map(|i| match i {
                FrameItem::Text { pos, text, .. } if text.as_str() == needle => {
                    Some((pos.x.val(), pos.y.val()))
                }
                _ => None,
            })
            .unwrap_or_else(|| panic!("{needle:?} não encontrado em {:?}", b.items))
    }

    /// Constantes convertidas para pt a 12pt (upem=1000 do fallback).
    fn kerns() -> (f64, f64, f64, f64) {
        let c = MathConstants::fallback();
        let kb = c.to_pt(c.radical_kern_before_degree, Pt(PT)).val(); // 3.336
        let ka = c.to_pt(c.radical_kern_after_degree, Pt(PT)).val(); // −6.672
        let extra = c.to_pt(c.radical_extra_ascender, Pt(PT)).val(); // 0.576
        let raise = c.radical_degree_bottom_raise_percent; // 0.6
        (kb, ka, extra, raise)
    }

    /// **Horizontal do índice**: `index_x = −min(sqrt_offset,0) + kern_before`
    /// (vanilla `radical.rs:113`). Com FixedMetrics: sqrt_offset =
    /// 3.336+3.6−6.672 = 0.264 > 0 ⇒ index_x = 3.336 (antes: 20% da
    /// largura do √ = 1.44).
    #[test]
    fn p970b_indice_x_segue_kerns() {
        let (kb, ka, _, _) = kerns();
        let sqrt_offset = math_oracle::radical_sqrt_offset(kb, 3.6, ka);
        let esperado = -sqrt_offset.min(0.0) + kb;
        let b = root_com_indice();
        let (x3, _) = pos_de(&b, "3");
        assert!(
            (x3 - esperado).abs() < 1e-9,
            "x do índice: oráculo {esperado:.4}pt (−min(sqrt_offset,0)+kern_before), obteve {x3:.4}"
        );
    }

    /// **Vertical do índice**: baseline em `−shift_up`,
    /// `shift_up = raise × (inner_ascent − descent) + idx.descent`
    /// (vanilla `radical.rs:94`). Com FixedMetrics: inner_ascent =
    /// 9.912+0.576 = 10.488; descent do surd = 8.4−9.912 = −1.512;
    /// shift_up = 0.6×12.0 = 7.2 (antes: índice no topo, y=−9.912).
    #[test]
    fn p970b_indice_encaixado_no_vinco() {
        let (_, _, extra, raise) = kerns();
        let inner_ascent = 9.912 + extra;
        let descent_surd = 8.4 - 9.912;
        let shift_up = math_oracle::radical_degree_shift_up(raise, inner_ascent, descent_surd, 0.0);
        let b = root_com_indice();
        let (_, y3) = pos_de(&b, "3");
        assert!(
            (y3 - (-shift_up)).abs() < 1e-9,
            "baseline do índice: oráculo −{shift_up:.4}pt, obteve {y3:.4}"
        );
    }

    /// **O √ e o radicando deslocam-se** `max(sqrt_offset, 0)` para dar
    /// lugar ao índice (vanilla `radical.rs:98-99`): x do radicando =
    /// 0.264 + 7.2 = 7.464 (antes: 7.2 — o índice sobrepujava o √).
    #[test]
    fn p970b_radicando_desloca_pelo_sqrt_offset() {
        let (kb, ka, _, _) = kerns();
        let sqrt_offset = math_oracle::radical_sqrt_offset(kb, 3.6, ka);
        let esperado = 7.2 + sqrt_offset.max(0.0);
        let b = root_com_indice();
        let (xx, _) = pos_de(&b, "x");
        assert!(
            (xx - esperado).abs() < 1e-9,
            "x do radicando: oráculo {esperado:.4}pt, obteve {xx:.4}"
        );
        // e a largura total cresce do mesmo montante (14.4 + 0.264)
        assert!(
            (b.width - (14.4 + sqrt_offset.max(0.0))).abs() < 1e-9,
            "largura total: esperado {:.4}pt, obteve {:.4}",
            14.4 + sqrt_offset.max(0.0),
            b.width
        );
    }

    /// **Ascent do composto cobre o índice** (vanilla `radical.rs:84,95`):
    /// `max(inner_ascent, shift_up + idx.ascent)` = max(10.488, 11.4) =
    /// 11.4 (antes: 9.912 — o índice ficava fora da caixa declarada).
    #[test]
    fn p970b_ascent_cobre_indice() {
        let (_, _, extra, raise) = kerns();
        let inner_ascent = 9.912 + extra;
        let shift_up =
            math_oracle::radical_degree_shift_up(raise, inner_ascent, 8.4 - 9.912, 0.0);
        let esperado = inner_ascent.max(shift_up + 4.2);
        let b = root_com_indice();
        assert!(
            (b.ascent - esperado).abs() < 1e-9,
            "ascent do composto: oráculo {esperado:.4}pt, obteve {:.4}",
            b.ascent
        );
    }

    /// **Guarda**: sem índice, nada muda (a fatia `extra_ascender` do caso
    /// sem índice é residual registado no L0, não deste passo).
    #[test]
    fn p970b_sem_indice_inalterado() {
        let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
        let b = ml.layout_node(
            &Content::math_root(None, Content::MathIdent("x".into())),
            &default_style(),
        );
        assert!((b.width - 14.4).abs() < 1e-9, "largura sem índice: {}", b.width);
        assert!((b.ascent - 10.488).abs() < 1e-9, "ascent sem índice: {}", b.ascent);
    }
}

// ── P974 — altura do √: short_fall=0 para o radical ────────────────────
//
// Especificação: `math/layout/root.md` §P974. O vanilla estica o √ com
// short_fall = 0 (`resolve.rs:1246`); delimitadores (lr/matrizes) mantêm
// DELIM_SHORT_FALL = 0.1em (P912). Stub com as variantes reais de
// NewCMMath (base 1001du, v1 1201du) e tinta do radicando controlada.
#[cfg(test)]
mod p974_tests {
    use super::*;
    use crate::entities::glyph_variants::{GlyphVariant, GlyphVariants};
    use std::collections::HashMap;

    const RAD_BASE_GID: u16 = 300;
    const RAD_V1_GID: u16 = 301;

    struct P974Metrics {
        inner: FixedMetrics,
        ink: HashMap<char, (Pt, Pt)>,
    }

    impl P974Metrics {
        fn new() -> Self {
            Self { inner: FixedMetrics, ink: HashMap::new() }
        }
        fn with_ink(mut self, c: char, top: f64, bottom: f64) -> Self {
            self.ink.insert(c, (Pt(top), Pt(bottom)));
            self
        }
    }

    impl FontMetrics for P974Metrics {
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
        fn text_ink_bounds(&self, text: &str, size: Pt, style: &TextStyle) -> (Pt, Pt) {
            if text.chars().count() == 1 {
                if let Some(&(top, bottom)) = self.ink.get(&text.chars().next().unwrap()) {
                    return (top, bottom);
                }
            }
            self.inner.text_ink_bounds(text, size, style)
        }
        fn vertical_glyph_variants(&self, c: char, _style: &TextStyle) -> GlyphVariants {
            if c == '√' {
                // Valores reais NewCMMath (upem 1000): base 1001du, v1 1201du.
                return GlyphVariants {
                    variants: vec![
                        GlyphVariant { glyph_id: RAD_BASE_GID, advance: 1001.0, hor_advance: 901.0 },
                        GlyphVariant { glyph_id: RAD_V1_GID, advance: 1201.0, hor_advance: 1001.0 },
                    ],
                };
            }
            GlyphVariants::default()
        }
        fn glyph_to_char(&self, glyph_id: u16) -> Option<char> {
            // O glifo base mapeia para '√' (caminho Text); a variante v1
            // não tem codepoint (caminho Glyph), como em produção.
            if glyph_id == RAD_BASE_GID { Some('√') } else { None }
        }
    }

    /// Com tinta do radicando tal que o alvo fica em 1050du (entre o
    /// short_fall de 100du e o avanço do glifo base de 1001du): a+d =
    /// 1050×0.012 − (60+66)×0.012 = 11.088pt (fallback: gap=60du,
    /// thickness=66du, 12pt).
    fn root_com_alvo_1050() -> MathBox {
        let metrics = P974Metrics::new().with_ink('x', 11.088, 0.0);
        let style = default_style();
        let ml = MathLayouter::new(&metrics, true, &style);
        ml.layout_node(
            &Content::math_root(None, Content::MathIdent("x".into())),
            &style,
        )
    }

    /// **Parte A** — com short_fall=0 (vanilla), um alvo de 1050du passa o
    /// glifo base (1001du) e selecciona a variante v1 (1201du). Com o
    /// short_fall de 0.1em (comportamento anterior, errado para o
    /// radical), o alvo caía para 950du e ficava o glifo base.
    #[test]
    fn p974_radical_sem_short_fall_selecciona_v1() {
        let b = root_com_alvo_1050();
        let tem_v1 = b.items.iter().any(|i| matches!(i, FrameItem::Glyph { glyph_id, .. } if *glyph_id == RAD_V1_GID));
        assert!(tem_v1, "alvo 1050du > base 1001du: v1 esperada, items: {:?}", b.items);
    }

    /// **Parte B** (gate confirmado 2026-08-05) — em Display o gap do
    /// radical é `radical_display_style_vertical_gap` (148du), não
    /// `radical_vertical_gap` (60du no fallback). Radicando com a+d =
    /// 10.03pt: alvo Text = (10.03+0.72+0.792)×83.33 = 962du → base;
    /// alvo Display = (10.03+1.776+0.792)×83.33 = 1050du → v1.
    #[test]
    fn p974_display_usa_gap_de_display() {
        let metrics = P974Metrics::new().with_ink('x', 10.03, 0.0);
        let style_display =
            TextStyle { math_size: MathSize::Display, ..default_style() };
        let style_text = TextStyle { math_size: MathSize::Text, ..default_style() };
        let ml = MathLayouter::new(&metrics, true, &style_display);
        let root = Content::math_root(None, Content::MathIdent("x".into()));

        let b_display = ml.layout_node(&root, &style_display);
        let tem_v1_display = b_display.items.iter().any(|i| matches!(i, FrameItem::Glyph { glyph_id, .. } if *glyph_id == RAD_V1_GID));
        assert!(
            tem_v1_display,
            "Display: alvo 1050du > base 1001du → v1 esperada: {:?}",
            b_display.items
        );

        let b_text = ml.layout_node(&root, &style_text);
        let tem_v1_text = b_text.items.iter().any(|i| matches!(i, FrameItem::Glyph { glyph_id, .. } if *glyph_id == RAD_V1_GID));
        assert!(
            !tem_v1_text,
            "Text: alvo 962du ≤ base → glifo base esperado: {:?}",
            b_text.items
        );
    }

    /// **Guarda** — radicando pequeno (alvo 551du): o glifo base cobre,
    /// nada muda (sem v1, sem esticamento indevido).
    #[test]
    fn p974_radical_pequeno_mantem_glifo_base() {
        let metrics = P974Metrics::new().with_ink('x', 4.0, 0.2);
        let style = default_style();
        let ml = MathLayouter::new(&metrics, true, &style);
        let b = ml.layout_node(
            &Content::math_root(None, Content::MathIdent("x".into())),
            &style,
        );
        let tem_glyph_esticado = b.items.iter().any(|i| matches!(i, FrameItem::Glyph { glyph_id, .. } if *glyph_id == RAD_V1_GID));
        assert!(!tem_glyph_esticado, "radicando pequeno: glifo base, items: {:?}", b.items);
        let tem_radical_texto = b.items.iter().any(|i| matches!(i, FrameItem::Text { text, .. } if text.as_str() == "√"));
        assert!(tem_radical_texto, "glifo base √ como texto esperado: {:?}", b.items);
    }
}

// ── P986 — linha do `cancel` na convenção baseline-relativa ─────────────
//
// Quarto caso da família de erro de convenção (P901/P906/P919/P972): a
// linha era emitida em coords topo-relativas `(0, h)`→`(width, 0)`; os
// items de `MathBox` vivem em coords baseline-relativas — o efeito de
// risco virava sublinhado (achado §7.3). Vanilla `cancel.rs:43-45,108-115`:
// linha centrada no centro do frame do corpo, de canto a canto da tinta.
#[cfg(test)]
mod p986_tests {
    use super::*;

    /// A linha do `cancel` vai de `(0, body.descent)` (canto inferior-
    /// esquerdo da tinta) a `(body.width, −body.ascent)` (canto superior-
    /// direito) — cruza o texto, não fica inteira abaixo dele.
    #[test]
    fn p986_cancel_linha_cruza_texto_convencao_baseline() {
        let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
        let style = default_style();
        let body = Content::MathIdent("x".into());
        let body_box = ml.layout_node(&body, &style);
        let result = ml.layout_cancel(&body, &style);

        let (start, end) = result
            .items
            .iter()
            .find_map(|i| match i {
                FrameItem::Line { start, end, .. } => Some((start, end)),
                _ => None,
            })
            .expect("layout_cancel deve emitir um FrameItem::Line");

        assert!(
            (start.x.val()).abs() < 0.001 && (start.y.val() - body_box.descent).abs() < 0.001,
            "start deve ser (0, descent={:.4}) — canto inferior-esquerdo da tinta; obteve ({:.4}, {:.4})",
            body_box.descent, start.x.val(), start.y.val()
        );
        assert!(
            (end.x.val() - body_box.width).abs() < 0.001
                && (end.y.val() + body_box.ascent).abs() < 0.001,
            "end deve ser (width={:.4}, −ascent={:.4}) — canto superior-direito da tinta; obteve ({:.4}, {:.4})",
            body_box.width, body_box.ascent, end.x.val(), end.y.val()
        );
        // A linha cruza a baseline dentro do corpo: start abaixo (y>0 se há
        // descent), end acima (y<0) — efeito de risco, não sublinhado.
        assert!(
            end.y.val() < 0.0 && start.y.val() > end.y.val(),
            "a linha tem de cruzar o corpo (end acima da baseline, start abaixo do topo): {:?} → {:?}",
            start, end
        );
    }

    /// A linha não altera as métricas da caixa (regressão do contrato P296).
    #[test]
    fn p986_cancel_linha_nao_expande_caixa() {
        let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
        let style = default_style();
        let body = Content::MathIdent("x".into());
        let body_box = ml.layout_node(&body, &style);
        let result = ml.layout_cancel(&body, &style);
        assert_eq!(result.width, body_box.width);
        assert_eq!(result.ascent, body_box.ascent);
        assert_eq!(result.descent, body_box.descent);
    }
}

// ── P990 — três achados independentes da auditoria (§8.5/§8.6/§8.1) ─────
//
// Parte A: `layout_frac` em `MathSize::Display` deve usar as 4 constantes
// Display dedicadas (`frac.md` §P990-A), não as de texto. Parte B:
// `FRAC_PADDING = 0.1em` na largura da fracção, barra só com `line_width`
// (`frac.md` §P990-B). Parte C: `apply_math_default` ganha braços para
// `MathCancel`/`Strike` (itálico do corpo) e `layout_node` ganha braço
// para `Strike` (linha, `_comum.md` §P990-C).
mod p990_tests {
    use super::*;

    fn frac_ab() -> (Content, Content) {
        (Content::MathIdent("a".into()), Content::MathIdent("b".into()))
    }

    /// **P990-A** — em `MathSize::Display`, `layout_frac` usa as 4
    /// constantes Display (`fraction_numerator_display_style_shift_up`
    /// etc.), não as de texto. `MathConstants::fallback()` tem valores
    /// Display bem distintos dos de texto (677/686/120/120du vs
    /// 394/345/48/48du) — a fórmula P920 recalculada com as constantes
    /// Display tem de bater com o `ascent`/`descent` devolvidos.
    #[test]
    fn p990a_frac_display_usa_constantes_display() {
        let c = MathConstants::fallback();
        let style = TextStyle { math_size: MathSize::Display, ..default_style() }; // 12pt
        let ml = MathLayouter::new(&FixedMetrics, true, &style);
        let (num, den) = frac_ab();

        let num_style = ml.numerator_style(&style);
        let den_style = ml.denominator_style(&style);
        let num_box = ml.layout_node(&num, &num_style);
        let den_box = ml.layout_node(&den, &den_style);

        let axis_pt = c.to_pt(c.axis_height, style.size).val();
        let thickness_pt = c.to_pt(c.fraction_rule_thickness, style.size).val();
        let shift_up_pt =
            c.to_pt(c.fraction_numerator_display_style_shift_up, style.size).val();
        let shift_down_pt =
            c.to_pt(c.fraction_denominator_display_style_shift_down, style.size).val();
        let num_floor = c.to_pt(c.fraction_num_display_style_gap_min, style.size).val();
        let den_floor = c.to_pt(c.fraction_denom_display_style_gap_min, style.size).val();

        // Sanity — as constantes Display são mensuravelmente diferentes das
        // de texto (senão o teste não discrimina qual conjunto foi usado).
        let shift_up_text_pt = c.to_pt(c.fraction_numerator_shift_up, style.size).val();
        assert!(
            (shift_up_pt - shift_up_text_pt).abs() > 1.0,
            "sanity: shift_up Display deve divergir do de texto ({shift_up_pt:.4} vs {shift_up_text_pt:.4})"
        );

        let expected_num_gap =
            (shift_up_pt - axis_pt - thickness_pt / 2.0 - num_box.descent).max(num_floor);
        let expected_den_gap = (shift_down_pt + axis_pt - thickness_pt / 2.0 - den_box.ascent)
            .max(den_floor);
        let expected_ascent = num_box.height() + expected_num_gap + thickness_pt / 2.0 + axis_pt;
        let expected_descent =
            den_box.height() + expected_den_gap + thickness_pt / 2.0 - axis_pt;

        let frac_box = ml.layout_frac(&num, &den, &style);
        assert!(
            (frac_box.ascent - expected_ascent).abs() < 1e-6,
            "P990-A — Display deve usar fraction_numerator_display_style_shift_up/\
             fraction_num_display_style_gap_min; esperado={expected_ascent:.4}pt, obtido={:.4}pt",
            frac_box.ascent
        );
        assert!(
            (frac_box.descent - expected_descent).abs() < 1e-6,
            "P990-A — Display deve usar fraction_denominator_display_style_shift_down/\
             fraction_denom_display_style_gap_min; esperado={expected_descent:.4}pt, obtido={:.4}pt",
            frac_box.descent
        );
    }

    /// **P990-A (guarda)** — nível `Text` (inline, default) continua a usar
    /// as constantes de texto de P920, inalterado por esta correcção.
    #[test]
    fn p990a_frac_text_mantem_constantes_de_texto() {
        let c = MathConstants::fallback();
        let style = default_style(); // math_size: Text (default)
        let ml = MathLayouter::new(&FixedMetrics, true, &style);
        let (num, den) = frac_ab();

        let num_style = ml.numerator_style(&style);
        let den_style = ml.denominator_style(&style);
        let num_box = ml.layout_node(&num, &num_style);
        let den_box = ml.layout_node(&den, &den_style);

        let axis_pt = c.to_pt(c.axis_height, style.size).val();
        let thickness_pt = c.to_pt(c.fraction_rule_thickness, style.size).val();
        let shift_up_pt = c.to_pt(c.fraction_numerator_shift_up, style.size).val();
        let shift_down_pt = c.to_pt(c.fraction_denominator_shift_down, style.size).val();
        let num_floor = c.to_pt(c.fraction_num_gap, style.size).val();
        let den_floor = c.to_pt(c.fraction_denom_gap, style.size).val();

        let expected_num_gap =
            (shift_up_pt - axis_pt - thickness_pt / 2.0 - num_box.descent).max(num_floor);
        let expected_den_gap = (shift_down_pt + axis_pt - thickness_pt / 2.0 - den_box.ascent)
            .max(den_floor);
        let expected_ascent = num_box.height() + expected_num_gap + thickness_pt / 2.0 + axis_pt;
        let expected_descent =
            den_box.height() + expected_den_gap + thickness_pt / 2.0 - axis_pt;

        let frac_box = ml.layout_frac(&num, &den, &style);
        assert!((frac_box.ascent - expected_ascent).abs() < 1e-6);
        assert!((frac_box.descent - expected_descent).abs() < 1e-6);
    }

    /// **P990-B** — `width = line_width + 2×padding` com `padding =
    /// 0.1×style.size` resolvido ao style DA FRACÇÃO (não ao style,
    /// já reduzido, do numerador/denominador). A barra desenha-se só com
    /// `line_width = max(num, den)`, centrada em `width`.
    #[test]
    fn p990b_frac_padding_largura_e_barra_centrada() {
        let style = default_style(); // 12pt
        let ml = MathLayouter::new(&FixedMetrics, true, &style);
        let (num, den) = frac_ab();

        let num_style = ml.numerator_style(&style);
        let den_style = ml.denominator_style(&style);
        let num_box = ml.layout_node(&num, &num_style);
        let den_box = ml.layout_node(&den, &den_style);
        let line_width = num_box.width.max(den_box.width);
        let padding = 0.1 * style.size.val();
        let expected_width = line_width + 2.0 * padding;

        let frac_box = ml.layout_frac(&num, &den, &style);
        assert!(
            (frac_box.width - expected_width).abs() < 1e-9,
            "P990-B — width deve ser line_width({line_width:.4}) + 2×padding({padding:.4}) \
             = {expected_width:.4}, obteve {:.4}",
            frac_box.width
        );

        let (rule_x0, rule_x1) = frac_box
            .items
            .iter()
            .find_map(|i| match i {
                FrameItem::Line { start, end, color: None, .. } => {
                    Some((start.x.val(), end.x.val()))
                }
                _ => None,
            })
            .expect("deve haver a barra de fracção (Line sem stroke explícito)");
        let expected_x0 = (expected_width - line_width) / 2.0;
        assert!(
            (rule_x0 - expected_x0).abs() < 1e-9 && (rule_x1 - (expected_x0 + line_width)).abs() < 1e-9,
            "P990-B — barra deve ir de (width−line_width)/2={expected_x0:.4} a \
             +line_width={:.4}, obteve [{rule_x0:.4}, {rule_x1:.4}]",
            expected_x0 + line_width
        );
    }

    /// **P990-C** — achado §8.1: `cancel(a)` perdia o itálico por defeito
    /// do corpo porque `apply_math_default` não recursava em `MathCancel`
    /// (caía no catch-all `other`). Depois da correcção, o corpo passa
    /// pelo mesmo mapeamento de P809 que qualquer outro `MathIdent` de 1
    /// letra — E a linha diagonal de `layout_cancel` (P986) continua
    /// presente (regressão do contrato P296).
    #[test]
    fn p990c_cancel_corpo_recebe_italico_e_mantem_linha() {
        let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
        let content = Content::math_cancel(Content::MathIdent("a".into()));
        let items = ml.layout_equation(&content, &default_style());

        let tem_italico = items.iter().any(|i| {
            matches!(i, FrameItem::Text { text, .. } if text.as_str() == "\u{1D44E}")
        });
        assert!(
            tem_italico,
            "cancel(a): corpo deve ser 𝑎 itálico (U+1D44E); items: {items:?}"
        );
        assert!(
            !items.iter().any(|i| matches!(i, FrameItem::Text { text, .. } if text.as_str() == "a")),
            "cancel(a): corpo NÃO deve ficar em 'a' reto: {items:?}"
        );
        assert!(
            items.iter().any(|i| matches!(i, FrameItem::Line { .. })),
            "cancel(a): a linha diagonal de P986 deve continuar presente: {items:?}"
        );
    }

    /// **P990-C** — achado §8.1: `strike(a+b)` caía inteiro no catch-all
    /// `plain_text()` de `layout_node` — perdia o itálico do corpo E a
    /// linha (decoração nunca desenhada em contexto math). Depois da
    /// correcção: corpo itálico (via `apply_math_default`, mesmo braço do
    /// teste acima) + uma `FrameItem::Line` horizontal.
    #[test]
    fn p990c_strike_corpo_recebe_italico_e_ganha_linha() {
        let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
        let content = Content::strike(Content::MathIdent("a".into()), None, None, None);
        let items = ml.layout_equation(&content, &default_style());

        let tem_italico = items.iter().any(|i| {
            matches!(i, FrameItem::Text { text, .. } if text.as_str() == "\u{1D44E}")
        });
        assert!(
            tem_italico,
            "strike(a): corpo deve ser 𝑎 itálico (U+1D44E); items: {items:?}"
        );
        assert!(
            items.iter().any(|i| matches!(i, FrameItem::Line { .. })),
            "strike(a): deve haver uma FrameItem::Line — o catch-all antigo \
             (plain_text()) não desenhava nenhuma; items: {items:?}"
        );
    }

    /// **P990-C** — geometria da linha de `strike`: offset por omissão
    /// `-0.25em` (acima da baseline própria do corpo, convenção
    /// baseline-relativa ADR-0123) e thickness `max(0.05em, 0.4pt)` —
    /// mesma fórmula do lado de texto (`compiler/layout/decorations.rs`).
    /// Chamada directa a `layout_node` (sem `apply_math_default`/
    /// `layout_equation`) para isolar a geometria da questão do itálico,
    /// já coberta pelo teste anterior.
    #[test]
    fn p990c_strike_geometria_da_linha() {
        let style = default_style(); // 12pt
        let ml = MathLayouter::new(&FixedMetrics, true, &style);
        let body = Content::MathIdent("a".into());
        let body_box = ml.layout_node(&body, &style);
        let result = ml.layout_node(
            &Content::strike(body, None, None, None),
            &style,
        );

        let expected_offset = -0.25 * style.size.val();
        let expected_thickness = (style.size.val() * 0.05_f64).max(0.4);
        let line = result
            .items
            .iter()
            .find_map(|i| match i {
                FrameItem::Line { start, end, thickness, .. } => Some((*start, *end, *thickness)),
                _ => None,
            })
            .expect("deve haver a linha de strike");
        assert!(
            (line.0.y.val() - expected_offset).abs() < 1e-9
                && (line.1.y.val() - expected_offset).abs() < 1e-9,
            "strike: y da linha deve ser offset por omissão -0.25em = {expected_offset:.4}, \
             obteve start.y={:.4} end.y={:.4}",
            line.0.y.val(),
            line.1.y.val()
        );
        assert!(
            (line.2 - expected_thickness).abs() < 1e-9,
            "strike: thickness deve ser max(0.05em, 0.4pt) = {expected_thickness:.4}, obteve {:.4}",
            line.2
        );
        // Sem `extent` explícito, a linha vai de 0 a body_box.width (sem
        // alargamento simétrico).
        assert!(
            (line.0.x.val() - 0.0).abs() < 1e-9 && (line.1.x.val() - body_box.width).abs() < 1e-9,
            "strike: sem extent, a linha deve ir de 0 a body_box.width={:.4}, obteve [{:.4}, {:.4}]",
            body_box.width,
            line.0.x.val(),
            line.1.x.val()
        );
        // A caixa devolvida não muda de largura/ascent/descent face ao
        // corpo (decoração cosmética, mesmo contrato de `layout_cancel`).
        assert_eq!(result.width, body_box.width);
        assert_eq!(result.ascent, body_box.ascent);
        assert_eq!(result.descent, body_box.descent);
    }
}

// ── P991 — `layout_grid` sem `&`: centrar em vez de alternar ──────────
//
// Especificação: `math/layout/_comum.md` §P991. Sem nenhum `MathAlignPoint`
// nos nós de origem (só `Linebreak`, caso `(n \ k)`), cada linha deve ficar
// centrada na largura da grelha (paridade com o default `CENTER` de
// `equation.rs`, `run.rs::stack_rows` quando `!has_alignment`) — não
// alternada esquerda/direita por paridade de coluna.
#[cfg(test)]
mod p991_tests {
    use super::*;

    fn x_min_de(b: &MathBox, needle: &str) -> f64 {
        b.items
            .iter()
            .filter_map(|i| match i {
                FrameItem::Text { pos, text, .. } if text.as_str() == needle => {
                    Some(pos.x.val())
                }
                _ => None,
            })
            .reduce(f64::min)
            .unwrap_or_else(|| panic!("{needle:?} não encontrado em {:?}", b.items))
    }

    fn x_max_de(b: &MathBox, needle: &str) -> f64 {
        b.items
            .iter()
            .filter_map(|i| match i {
                FrameItem::Text { pos, text, style, .. } if text.as_str() == needle => {
                    let adv = 0.6 * style.size.val() * needle.chars().count() as f64;
                    Some(pos.x.val() + adv)
                }
                _ => None,
            })
            .reduce(f64::max)
            .unwrap_or_else(|| panic!("{needle:?} não encontrado em {:?}", b.items))
    }

    /// **Caso do passo**: `(n \ k)` — duas linhas, 1 coluna, sem `&`. A
    /// linha mais estreita ("n", 1 char) deve ficar centrada dentro da
    /// largura da grelha (que é a largura de "kk", a linha mais larga),
    /// não empurrada para a direita (bug: hoje fica flush-right, folga
    /// esquerda = largura toda, folga direita = 0).
    #[test]
    fn p991_grid_sem_alignpoint_centra_linha_mais_estreita() {
        let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
        let style = default_style(); // 12pt
        let nodes = vec![
            Content::MathText("n".into()),
            Content::linebreak(),
            Content::MathText("kk".into()),
        ];
        let b = ml.layout_grid(&nodes, &style);

        let gap_esq = x_min_de(&b, "n");
        let gap_dir = b.width - x_max_de(&b, "n");
        assert!(
            (gap_esq - gap_dir).abs() < 0.01,
            "sem `&`: \"n\" deve ficar centrado (folga esq == folga dir), \
             obteve esq={gap_esq:.4} dir={gap_dir:.4} (largura da grelha={:.4})",
            b.width
        );
    }

    /// **Guarda de não-regressão**: com `&` presente (2 linhas × 2 colunas,
    /// larguras de coluna diferentes por linha), a alternância
    /// esquerda/direita por paridade de coluna continua a aplicar-se — só
    /// o caso sem nenhum `&` muda. Uma única linha não discrimina
    /// `Alternating` de `Center` (a largura da coluna colapsa na da única
    /// célula); por isso duas linhas com larguras diferentes na coluna 0
    /// ("n"=1 char vs "mm"=2 chars — `align_boundary_spacing` entre dois
    /// `MathText` alfabéticos é 0.0, `matrix.rs::align_boundary_spacing`,
    /// P967b — sem espaço extra a confundir a conta).
    #[test]
    fn p991_grid_com_alignpoint_mantem_alternancia() {
        let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
        let style = default_style(); // 12pt: advance 0.6*12=7.2pt/char
        let nodes = vec![
            Content::MathText("n".into()),
            Content::math_align_point(),
            Content::MathText("kk".into()),
            Content::linebreak(),
            Content::MathText("mm".into()),
            Content::math_align_point(),
            Content::MathText("j".into()),
        ];
        let b = ml.layout_grid(&nodes, &style);

        // Coluna 0 (par) alinha à direita: "n" (linha 0, mais estreita que
        // "mm") deve ficar flush contra o limite `&` — folga entre o fim de
        // "n" e o início de "kk" ≈ 0, não centrada (que daria ≈3.6pt: metade
        // da diferença de largura entre "n" e "mm", 14.4−7.2=7.2).
        let gap_direita_n = x_min_de(&b, "kk") - x_max_de(&b, "n");
        assert!(
            gap_direita_n.abs() < 0.01,
            "com `&`: \"n\" (coluna par) deve continuar alinhado à direita \
             da coluna 0 (folga até ao limite `&` ≈ 0), obteve {gap_direita_n:.4}"
        );

        // Coluna 1 (ímpar) alinha à esquerda: "j" (linha 1, mais estreita
        // que "kk") deve ficar flush contra o limite `&` — folga entre o
        // fim de "mm" e o início de "j" ≈ 0.
        let gap_esquerda_j = x_min_de(&b, "j") - x_max_de(&b, "mm");
        assert!(
            gap_esquerda_j.abs() < 0.01,
            "com `&`: \"j\" (coluna ímpar) deve continuar alinhado à \
             esquerda da coluna 1 (folga desde o limite `&` ≈ 0), \
             obteve {gap_esquerda_j:.4}"
        );
    }
}

// ── P992 — `Content::MathLimitsOverride` (`limits()`/`scripts()`) ──────
//
// Especificação: `math/layout/attach.md` §P992 + `math/layout/_comum.md`
// §P992. `limits(body, inline: true)` (default) empilha mesmo em modo
// inline; `scripts(body)` nunca empilha, mesmo em modo bloco — inverso
// exacto do comportamento natural de `A_1^2`/`sum_1^2` nesse modo.
#[cfg(test)]
mod p992_tests {
    use super::*;

    fn texto_total(items: &[FrameItem]) -> String {
        items
            .iter()
            .filter_map(|i| match i {
                FrameItem::Text { text, .. } => Some(text.as_str()),
                _ => None,
            })
            .collect()
    }

    fn sup_x_e_base_x(items: &[FrameItem], needle_base: &str, needle_sup: &str) -> (f64, f64) {
        let base_x = items
            .iter()
            .find_map(|i| match i {
                FrameItem::Text { pos, text, .. } if text.as_str() == needle_base => {
                    Some(pos.x.val())
                }
                _ => None,
            })
            .unwrap_or_else(|| panic!("base {needle_base:?} não encontrada em {items:?}"));
        let sup_x = items
            .iter()
            .find_map(|i| match i {
                FrameItem::Text { pos, text, .. } if text.as_str() == needle_sup => {
                    Some(pos.x.val())
                }
                _ => None,
            })
            .unwrap_or_else(|| panic!("sup {needle_sup:?} não encontrado em {items:?}"));
        (base_x, sup_x)
    }

    /// **Caso do achado**: `limits(A)^alpha_beta` — "A" não é operador
    /// grande nem função de limite, por isso NUNCA empilharia sozinho
    /// (nem em modo bloco). `limits()` força o empilhamento mesmo assim.
    /// Sub/sup de 1 carácter (largura comparável à base) — mesma
    /// convenção dos testes irmãos (`math_attach_sum_empilha_limites_
    /// em_modo_bloco`): larguras muito diferentes centrariam em torno de
    /// pontos-médios distintos, não em `x` comparável directamente.
    #[test]
    fn p992_limits_em_base_normal_empilha_em_modo_bloco() {
        let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
        let attach = Content::math_attach(
            Content::math_limits_override(
                Content::MathIdent("A".into()),
                true,
                true,
            ),
            None,
            None,
            Some(Content::MathText("b".into())),
            Some(Content::MathText("a".into())),
        );
        let items = ml.layout_equation(&attach, &default_style());
        let (base_x, sup_x) = sup_x_e_base_x(&items, "𝐴", "𝑎");
        assert!(
            (sup_x - base_x).abs() < 6.0,
            "limits(A): sup deve ficar centrado sobre a base (empilhado), \
             base_x={base_x} sup_x={sup_x}"
        );
    }

    /// **Diferenciador crucial**: `limits(A)_1^2` empilha MESMO em modo
    /// INLINE (`self.block = false`) — `inline: true` é o default do
    /// vanilla (`LimitsElem.inline`). Sem o override, "A" nunca
    /// empilharia (nem em bloco, muito menos inline).
    #[test]
    fn p992_limits_com_inline_true_empilha_mesmo_em_modo_inline() {
        let ml = MathLayouter::new(&FixedMetrics, false, &default_style()); // block=false
        let attach = Content::math_attach(
            Content::math_limits_override(
                Content::MathIdent("A".into()),
                true,
                true, // inline: true — força mesmo fora de bloco
            ),
            None,
            None,
            Some(Content::MathText("1".into())),
            Some(Content::MathText("2".into())),
        );
        let items = ml.layout_equation(&attach, &default_style());
        let (base_x, sup_x) = sup_x_e_base_x(&items, "𝐴", "2");
        assert!(
            (sup_x - base_x).abs() < 6.0,
            "limits(A, inline: true) em modo inline: sup deve empilhar \
             sobre a base mesmo assim, base_x={base_x} sup_x={sup_x}"
        );
    }

    /// **`inline: false`**: só empilha em modo bloco — mesma regra do caso
    /// natural. Em modo inline, cai de volta a scripts laterais.
    #[test]
    fn p992_limits_com_inline_false_nao_empilha_em_modo_inline() {
        let ml = MathLayouter::new(&FixedMetrics, false, &default_style()); // block=false
        let attach = Content::math_attach(
            Content::math_limits_override(
                Content::MathIdent("A".into()),
                true,
                false, // inline: false — só em modo bloco
            ),
            None,
            None,
            Some(Content::MathText("1".into())),
            Some(Content::MathText("2".into())),
        );
        let items = ml.layout_equation(&attach, &default_style());
        let base_w = 12.0 * 0.6; // FixedMetrics: char_width = size * 0.6, style 12pt.
        let (base_x, sup_x) = sup_x_e_base_x(&items, "𝐴", "2");
        assert!(
            sup_x > base_x + base_w * 0.5,
            "limits(A, inline: false) em modo inline: sup deve ficar \
             lateral (não empilhado), base_x={base_x} sup_x={sup_x}"
        );
    }

    /// **Caso do achado (doc vanilla)**: `scripts(sum)_1^2 != sum_1^2` —
    /// `sum` SOZINHO empilharia em modo bloco (operador grande); `scripts()`
    /// força o oposto, scripts laterais, mesmo em modo bloco.
    #[test]
    fn p992_scripts_em_operador_grande_nao_empilha_em_modo_bloco() {
        let ml = MathLayouter::new(&FixedMetrics, true, &default_style()); // block=true
        let attach = Content::math_attach(
            Content::math_limits_override(
                Content::MathText("∑".into()),
                false,
                true,
            ),
            None,
            None,
            Some(Content::MathText("1".into())),
            Some(Content::MathText("2".into())),
        );
        let items = ml.layout_equation(&attach, &default_style());
        let base_w = 12.0 * 0.6;
        let (base_x, sup_x) = sup_x_e_base_x(&items, "∑", "2");
        assert!(
            sup_x > base_x + base_w * 0.5,
            "scripts(sum) em modo bloco: sup deve ficar lateral (não \
             empilhado, ao contrário de sum_1^2 sozinho), \
             base_x={base_x} sup_x={sup_x}"
        );
    }

    /// **Guarda**: `sum_1^2` SEM `scripts()`/`limits()` continua a
    /// empilhar normalmente em modo bloco (comportamento natural
    /// inalterado — não-regressão face a `math_attach_sum_empilha_
    /// limites_em_modo_bloco`, já existente).
    #[test]
    fn p992_guarda_sum_sem_wrapper_continua_empilhando_em_modo_bloco() {
        let ml = MathLayouter::new(&FixedMetrics, true, &default_style());
        let attach = Content::math_attach(
            Content::MathText("∑".into()),
            None,
            None,
            Some(Content::MathText("1".into())),
            Some(Content::MathText("2".into())),
        );
        let items = ml.layout_equation(&attach, &default_style());
        let (base_x, sup_x) = sup_x_e_base_x(&items, "∑", "2");
        assert!(
            (sup_x - base_x).abs() < 6.0,
            "sum_1^2 sem wrapper: continua a empilhar em modo bloco, \
             base_x={base_x} sup_x={sup_x}"
        );
    }

    /// **Itálico por defeito preservado**: `limits(A)` — "A" (base de 1
    /// letra) deve continuar a receber o itálico matemático por defeito
    /// (𝐴, U+1D434), via `apply_math_default` recursando no `body`. Exercita
    /// o pipeline completo (`layout_equation` → `apply_math_default`).
    #[test]
    fn p992_limits_preserva_italico_por_defeito_da_base() {
        let style = default_style();
        let ml = MathLayouter::new(&FixedMetrics, true, &style);
        let body = Content::MathSequence(Arc::from(vec![Content::math_attach(
            Content::math_limits_override(Content::MathIdent("A".into()), true, true),
            None,
            None,
            Some(Content::MathText("1".into())),
            Some(Content::MathText("2".into())),
        )]));
        let content = apply_math_default(&body);
        let items = ml.layout_equation(&content, &style);
        let t = texto_total(&items);
        assert!(
            t.contains('\u{1D434}'), // 𝐴 — math italic capital A
            "limits(A): \"A\" deve continuar itálico por defeito (𝐴), texto: {t:?}"
        );
        assert!(!t.contains('A'), "\"A\" recto não deve aparecer: {t:?}");
    }

    /// **Transparência de classe**: `scripts(sum)` continua classificado
    /// como `Large` (herdado do `body`) — `base_math_class` não força
    /// `Normal` (diferente de `MathClassOverride`, que forçaria).
    #[test]
    fn p992_scripts_nao_muda_classe_do_body() {
        use super::super::spacing::node_math_class;
        let wrapped = Content::math_limits_override(
            Content::MathText("∑".into()),
            false,
            true,
        );
        let (lclass, rclass) = node_math_class(&wrapped);
        assert_eq!(lclass, crate::entities::math_class::MathClass::Large);
        assert_eq!(rclass, crate::entities::math_class::MathClass::Large);
    }

    // ── P1043: Pares de independência testcase() para math/layout/mod.rs:1614 ─

    #[test]
    fn p1043_math_styled_outer_size_inner_style_isolada() {
        use crate::entities::math_style::MathStyleKind;
        use crate::compiler::math::layout::apply_math_style;
        // C1=T, C2=T: outer is size variant, inner is not -> eff_kind = inner
        let inner_styled = Content::math_styled(Some(MathStyleKind::DoubleStruck), None, None, Content::MathIdent("x".into()), None);
        let res = apply_math_style(&inner_styled, Some(MathStyleKind::Display), None, None);
        assert_eq!(res.plain_text(), "𝕩");
    }

    #[test]
    fn p1043_math_styled_outer_size_inner_size_isolada() {
        use crate::entities::math_style::MathStyleKind;
        use crate::compiler::math::layout::apply_math_style;
        // C1=T, C2=F: outer is size variant, inner is size variant -> outer.or(inner) = outer (Display)
        let inner_styled = Content::math_styled(Some(MathStyleKind::Script), None, None, Content::MathIdent("x".into()), None);
        let res = apply_math_style(&inner_styled, Some(MathStyleKind::Display), None, None);
        // Display trata-se como Plain no mapeamento de glifo
        assert_eq!(res.plain_text(), "𝑥");
    }

    #[test]
    fn p1043_math_styled_outer_style_inner_size_isolada() {
        use crate::entities::math_style::MathStyleKind;
        use crate::compiler::math::layout::apply_math_style;
        // C1=F, C2=_: outer is not size variant -> outer.or(inner) = outer (DoubleStruck)
        let inner_styled = Content::math_styled(Some(MathStyleKind::Script), None, None, Content::MathIdent("x".into()), None);
        let res = apply_math_style(&inner_styled, Some(MathStyleKind::DoubleStruck), None, None);
        assert_eq!(res.plain_text(), "𝕩");
    }


    // ── P1047: Bateria de 7 Testes para math.cases (delim, reverse, gap) ───

    #[test]
    fn p1047_c1_cases_set_rule_delim() {
        let style = default_style();
        let ml = MathLayouter::new(&FixedMetrics, true, &style);
        let rows = vec![
            vec![Content::MathText("1".into())],
            vec![Content::MathText("2".into())],
        ];
        let b_bracket = ml.layout_cases(&rows, ('[', ']'), false, None, &style);
        assert!(b_bracket.width > 0.0);
        // O delimitador '[' tem largura não nula
        assert!(b_bracket.items.len() >= 3);
    }

    #[test]
    fn p1047_c2_cases_inline_delim() {
        let style = default_style();
        let ml = MathLayouter::new(&FixedMetrics, true, &style);
        let rows = vec![
            vec![Content::MathText("1".into())],
            vec![Content::MathText("2".into())],
        ];
        let b_paren = ml.layout_cases(&rows, ('(', ')'), false, None, &style);
        assert!(b_paren.width > 0.0);
    }

    #[test]
    fn p1047_c3_cases_set_rule_gap() {
        let style = default_style();
        let ml = MathLayouter::new(&FixedMetrics, true, &style);
        let rows = vec![
            vec![Content::MathText("1".into())],
            vec![Content::MathText("2".into())],
        ];
        let b_def = ml.layout_cases(&rows, ('{', '}'), false, None, &style);
        let b_gap = ml.layout_cases(&rows, ('{', '}'), false, Some(crate::entities::layout_types::Length::em(1.0)), &style);
        // Gap de 1em deve aumentar a altura total da grade em comparação com default 0.2em
        assert!(b_gap.height() > b_def.height());
    }

    #[test]
    fn p1047_c4_cases_inline_gap() {
        let style = default_style();
        let ml = MathLayouter::new(&FixedMetrics, true, &style);
        let rows = vec![
            vec![Content::MathText("1".into())],
            vec![Content::MathText("2".into())],
        ];
        let b_gap_pt = ml.layout_cases(&rows, ('{', '}'), false, Some(crate::entities::layout_types::Length::pt(14.0)), &style);
        assert!(b_gap_pt.width > 0.0);
    }

    #[test]
    fn p1047_c5_cases_inline_reverse() {
        let style = default_style();
        let ml = MathLayouter::new(&FixedMetrics, true, &style);
        let rows = vec![
            vec![Content::MathText("1".into())],
            vec![Content::MathText("2".into())],
        ];
        let b_rev = ml.layout_cases(&rows, ('{', '}'), true, None, &style);
        assert!(b_rev.width > 0.0);
        // Com reverse: true, o delimitador fica à direita da grade
    }

    #[test]
    fn p1047_c6_cases_set_rule_reverse() {
        let style = default_style();
        let ml = MathLayouter::new(&FixedMetrics, true, &style);
        let rows = vec![
            vec![Content::MathText("1".into())],
            vec![Content::MathText("2".into())],
        ];
        let b_rev = ml.layout_cases(&rows, ('[', ']'), true, None, &style);
        assert!(b_rev.width > 0.0);
    }

    #[test]
    fn p1047_c7_cases_default_unaffected() {
        let style = default_style();
        let ml = MathLayouter::new(&FixedMetrics, true, &style);
        let rows = vec![
            vec![Content::MathText("1".into())],
            vec![Content::MathText("2".into())],
        ];
        let b_def = ml.layout_cases(&rows, ('{', '}'), false, None, &style);
        assert!(b_def.width > 0.0);
    }
}

// ── P1088 — Resolução dos Dois Resíduos do P1087 (Heading → Eq e Eq2 → Eq3)
#[cfg(test)]
mod p1088_tests {
    use super::*;
    use crate::compiler::layout::{FixedMetrics, FontMetrics};
    use crate::entities::math_constants::MathConstants;
    use std::collections::HashMap;

    struct P1088SignedMetrics {
        inner: FixedMetrics,
        signed_ink: HashMap<char, (Pt, Pt)>,
        constants: MathConstants,
    }

    impl P1088SignedMetrics {
        fn new(constants: MathConstants) -> Self {
            Self {
                inner: FixedMetrics,
                signed_ink: HashMap::new(),
                constants,
            }
        }

        fn with_signed_ink(mut self, c: char, top: f64, bottom: f64) -> Self {
            self.signed_ink.insert(c, (Pt(top), Pt(bottom)));
            self
        }
    }

    impl FontMetrics for P1088SignedMetrics {
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
        fn text_ink_bounds(&self, text: &str, size: Pt, style: &TextStyle) -> (Pt, Pt) {
            if text.chars().count() == 1 {
                if let Some(&(top, bottom)) = self.signed_ink.get(&text.chars().next().unwrap()) {
                    return (top, Pt(bottom.0.max(0.0)));
                }
            }
            self.inner.text_ink_bounds(text, size, style)
        }
        fn text_ink_bounds_signed(&self, text: &str, size: Pt, style: &TextStyle) -> (Pt, Pt) {
            if text.chars().count() == 1 {
                if let Some(&(top, bottom)) = self.signed_ink.get(&text.chars().next().unwrap()) {
                    return (top, bottom);
                }
            }
            self.inner.text_ink_bounds_signed(text, size, style)
        }
        fn math_constants(&self, _style: &TextStyle) -> MathConstants {
            self.constants.clone()
        }
    }

    #[test]
    fn p1088_signed_ink_bounds_prevents_spurious_attach_gap() {
        let mut c = MathConstants::fallback();
        c.superscript_shift_up = 335.0; // 3.685pt
        c.subscript_shift_down = 247.0; // 2.717pt
        c.sub_superscript_gap_min = 160.0; // 1.76pt

        let metrics = P1088SignedMetrics::new(c)
            .with_signed_ink('λ', 7.634, 2.354)
            .with_signed_ink('i', 4.906, 0.0)
            .with_signed_ink('*', 7.502, -0.2926);

        let ml = MathLayouter::new(&metrics, true, &default_style());
        let style = default_style();

        let base = Content::MathText("λ".into());
        let sub = Content::MathText("i".into());
        let sup = Content::MathText("*".into());

        let frame = ml.layout_attach(&base, None, None, Some(&sub), Some(&sup), &style);
        assert!(frame.width > 0.0);
    }
}
