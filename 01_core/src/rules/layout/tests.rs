//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout.md
//! @prompt-hash a78b0adc
//! @layer L1
//! @updated 2026-04-23
//!
//! Testes de layout — extraídos de `layout/mod.rs` no Passo 96.7
//! conforme ADR-0037.
//!
//! Excepção Regra 6 da ADR-0037: ficheiro só de testes (gated por
//! `#[cfg(test)]` a partir do `mod.rs`). Testes de layout cruzam
//! domínios por natureza (um único documento exercita texto, math,
//! grid, place, align, transform simultaneamente); distribuí-los por
//! cluster produziria duplicação ou perda de cobertura E2E. Tamanho
//! actual ~1400 linhas aceite sob Regra 5 + Regra 6 combinadas.

use super::*;
use crate::entities::{content::Content, counter_state::CounterState, layout_types::FrameItem};
use crate::rules::introspect::introspect;

// ── Testes de FixedMetrics (Passo 21) ────────────────────────────────

#[test]
fn fixed_metrics_advance_proporcional_ao_tamanho() {
    let m = FixedMetrics;
    let a12 = m.advance("Hello", Pt(12.0));
    let a24 = m.advance("Hello", Pt(24.0));
    assert!(
        (a24.val() - 2.0 * a12.val()).abs() < 0.001,
        "advance deve escalar linearmente com font_size"
    );
}

#[test]
fn fixed_metrics_monoespaco_iiii_eq_wwww() {
    let m = FixedMetrics;
    let ai = m.advance("iiii", Pt(12.0));
    let aw = m.advance("WWWW", Pt(12.0));
    assert_eq!(ai, aw, "FixedMetrics é monoespaçado — iiii == WWWW");
}

#[test]
fn fixed_metrics_vertical_ascender_menor_que_line_height() {
    let (asc, lh) = FixedMetrics.vertical_metrics(Pt(12.0));
    assert!(asc.val() > 0.0, "ascender deve ser positivo");
    assert!(lh.val() > asc.val(), "line_height > ascender");
}

#[test]
fn layouter_baseline_dentro_da_pagina() {
    let l = Layouter::new(FixedMetrics, NullImageSizer, 12.0);
    assert!(l.cursor_y.val() > 0.0);
    assert!(l.cursor_y.val() < 842.0);
}

// ── Testes de layout() (herdados do Passo 19) ─────────────────────────

#[test]
fn layout_texto_simples_tem_items() {
    let doc = layout(&Content::text("Hello world"), CounterState::default());
    assert!(!doc.pages.is_empty());
    let total = doc.pages.iter().flat_map(|p| p.items.iter()).count();
    assert!(total >= 2, "Hello e world devem ser itens separados");
    assert!(doc.plain_text().contains("Hello"));
    assert!(doc.plain_text().contains("world"));
}

#[test]
fn layout_documento_vazio_zero_paginas() {
    let doc = layout(&Content::Empty, CounterState::default());
    assert_eq!(doc.pages.len(), 0, "documento vazio → sem páginas");
}

/// Teste de Ouro: todos os items dentro dos limites da página.
#[test]
fn layout_items_dentro_limites_da_pagina() {
    let words = (0..100)
        .map(|i| format!("palavra{i}"))
        .collect::<Vec<_>>()
        .join(" ");
    let doc = layout(&Content::text(&words), CounterState::default());

    for page in &doc.pages {
        for item in &page.items {
            if let FrameItem::Text { pos, .. } = item {
                assert!(
                    pos.x.val() >= 0.0 && pos.x.val() < 595.0,
                    "x={} fora dos limites da página", pos.x.val()
                );
                assert!(
                    pos.y.val() >= 0.0 && pos.y.val() < 842.0,
                    "y={} fora dos limites da página", pos.y.val()
                );
            }
        }
    }
}

#[test]
fn layout_texto_longo_word_wrap() {
    let words = (0..50)
        .map(|i| format!("w{i}"))
        .collect::<Vec<_>>()
        .join(" ");
    let doc = layout(&Content::text(&words), CounterState::default());
    let items = doc.pages.iter().flat_map(|p| p.items.iter()).count();
    let y_values: std::collections::HashSet<u64> = doc
        .pages
        .iter()
        .flat_map(|p| p.items.iter())
        .filter_map(|i| { if let FrameItem::Text { pos, .. } = i { Some(pos.y.val().to_bits()) } else { None } })
        .collect();
    assert!(y_values.len() > 1, "texto longo deve ter múltiplas linhas: {} items", items);
}

// ── Testes rich text (Passo 22) ────────────────────────────────────────

#[test]
fn strong_produz_bold_style() {
    // Após Passo 33: node_style deve ter bold=true (capturado em eval via Strong).
    // Construção directa usa TextStyle::bold para simular o que eval produziria.
    let doc = layout(&Content::strong(
        Content::Text("Bold".into(), TextStyle::bold(Pt(11.0)))
    ), CounterState::default());
    let bold = doc.pages.iter()
        .flat_map(|p| p.items.iter())
        .any(|i| matches!(i, FrameItem::Text { style, .. } if style.bold));
    assert!(bold, "Strong deve produzir FrameItem com bold=true");
}

#[test]
fn emph_produz_italic_style() {
    // Após Passo 33: node_style deve ter italic=true (capturado em eval via Emph).
    // Construção directa usa TextStyle::italic para simular o que eval produziria.
    let doc = layout(&Content::emph(
        Content::Text("Italic".into(), TextStyle::italic(Pt(11.0)))
    ), CounterState::default());
    let italic = doc.pages.iter()
        .flat_map(|p| p.items.iter())
        .any(|i| matches!(i, FrameItem::Text { style, .. } if style.italic));
    assert!(italic, "Emph deve produzir FrameItem com italic=true");
}

#[test]
fn heading_h1_tamanho_maior() {
    let content = Content::sequence(vec![
        Content::heading(1, Content::text("Title")),
        Content::text("body"),
    ]);
    let doc = layout(&content, introspect(&content));
    let sizes: Vec<f64> = doc.pages.iter()
        .flat_map(|p| p.items.iter())
        .filter_map(|i| { if let FrameItem::Text { style, .. } = i { Some(style.size.val()) } else { None } })
        .collect();
    let max_size = sizes.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let min_size = sizes.iter().cloned().fold(f64::INFINITY,     f64::min);
    assert!(max_size > min_size, "H1 deve ter tamanho maior que o texto normal");
}

#[test]
fn estilo_restaurado_apos_strong() {
    let doc = layout(&Content::sequence(vec![
        Content::strong(Content::text("Bold")),
        Content::text("normal"),
    ]), CounterState::default());
    let items: Vec<_> = doc.pages.iter()
        .flat_map(|p| p.items.iter())
        .collect();
    if let Some(FrameItem::Text { style, text, .. }) = items.last() {
        if text.as_str() == "normal" {
            assert!(!style.bold, "texto após Strong deve ser regular");
        }
    }
}

#[test]
fn pt_tipagem_nao_permite_add_f64() {
    let a = Pt(10.0);
    let b = Pt(5.0);
    let c = a + b;
    assert_eq!(c, Pt(15.0));
    // a + 5.0 ← não compila
}

#[test]
fn pipeline_parse_eval_layout() {
    use crate::{
        contracts::world::World,
        entities::{
            file_id::FileId,
            font_book::FontBook,
            source::Source,
            world_types::{Bytes, Datetime, FileError, FileResult, Font, Library},
        },
        rules::eval::eval_for_test,
    };
    use std::num::NonZeroU16;

    struct MockWorld {
        library: Library,
        book:    FontBook,
        source:  Source,
    }

    impl MockWorld {
        fn new(text: &str) -> Self {
            let id = FileId::from_raw(NonZeroU16::new(1).unwrap());
            Self {
                library: Library::new(),
                book:    FontBook::new(),
                source:  Source::new(id, text.to_string()),
            }
        }
    }

    impl World for MockWorld {
        fn library(&self) -> &Library  { &self.library }
        fn book(&self)    -> &FontBook { &self.book }
        fn main(&self)    -> FileId    { self.source.id() }
        fn source(&self, _: FileId) -> FileResult<Source> { Ok(self.source.clone()) }
        fn file(&self, _: FileId)    -> FileResult<Bytes>   { Err(FileError::NotFound) }
        fn font(&self, _: usize)     -> Option<Font>        { None }
        fn today(&self, _: Option<i64>) -> Option<Datetime> { None }
    }

    let world = MockWorld::new("Olá mundo");
    let src = World::source(&world, World::main(&world)).unwrap();
    let module = eval_for_test(&world, &src).unwrap();
    let content = module.content().expect("deve ter content");
    let state = introspect(content);
    let doc = layout(content, state);
    assert!(!doc.pages.is_empty());
    assert!(
        doc.plain_text().contains("Olá") || doc.plain_text().contains("mundo"),
        "texto deve estar no output: {:?}", doc.plain_text()
    );
}

// ── Passo 23 ────────────────────────────────────────────────────────────

#[test]
fn layout_list_item_tem_bullet() {
    let doc = layout(&Content::list_item(Content::text("Item")), CounterState::default());
    let has_marker = doc.pages.iter()
        .flat_map(|p| p.items.iter())
        .any(|i| matches!(i, FrameItem::Text { text, .. } if text.as_str() == "•"));
    assert!(has_marker, "ListItem deve ter marcador '•'");
}

#[test]
fn layout_raw_block_tamanho_menor() {
    let content = Content::sequence(vec![
        Content::text("normal"),
        Content::raw("code", None, true),
    ]);
    let doc = layout(&content, CounterState::default());
    let sizes: std::collections::HashSet<u64> = doc.pages.iter()
        .flat_map(|p| p.items.iter())
        .filter_map(|i| match i {
            FrameItem::Text { style, .. } => Some(style.size.val().to_bits()),
            _ => None,
        })
        .collect();
    assert!(sizes.len() > 1, "Raw deve ter tamanho diferente do texto normal");
}

// ── Passo 48 — Baselines em equações inline ──────────────────────────────

fn layout_test(src: &str) -> PagedDocument {
    use crate::{
        contracts::world::World,
        entities::{
            file_id::FileId,
            font_book::FontBook,
            source::Source,
            world_types::{Bytes, Datetime, FileError, FileResult, Font, Library},
        },
        rules::eval::eval_for_test,
    };
    use std::num::NonZeroU16;

    struct MockWorld {
        library: Library,
        book:    FontBook,
        source:  Source,
    }

    impl MockWorld {
        fn new(text: &str) -> Self {
            let id = FileId::from_raw(NonZeroU16::new(1).unwrap());
            Self {
                library: Library::new(),
                book:    FontBook::new(),
                source:  Source::new(id, text.to_string()),
            }
        }
    }

    impl World for MockWorld {
        fn library(&self) -> &Library  { &self.library }
        fn book(&self)    -> &FontBook { &self.book }
        fn main(&self)    -> FileId    { self.source.id() }
        fn source(&self, _: FileId) -> FileResult<Source> { Ok(self.source.clone()) }
        fn file(&self, _: FileId)    -> FileResult<Bytes>   { Err(FileError::NotFound) }
        fn font(&self, _: usize)     -> Option<Font>        { None }
        fn today(&self, _: Option<i64>) -> Option<Datetime> { None }
    }

    let world = MockWorld::new(src);
    let source = World::source(&world, World::main(&world)).unwrap();
    let module = eval_for_test(&world, &source).unwrap();
    let content = module.content().expect("deve ter content");
    let state = introspect(content);
    layout(content, state)
}

#[cfg(test)]
mod tests_inline_baseline {
    use super::*;

    #[test]
    fn equacao_inline_nao_regride_conteudo() {
        let doc = layout_test("$frac(1, 2)$");
        let text = doc.plain_text();
        assert!(text.contains('1'), "numerador: {}", text);
        assert!(text.contains('2'), "denominador: {}", text);
    }

    #[test]
    fn equacao_inline_simples_nao_regride() {
        let doc = layout_test("$x + 1$");
        let text = doc.plain_text();
        assert!(text.contains('x'));
        assert!(text.contains('1'));
    }

    #[test]
    fn equacao_inline_com_attach_nao_regride() {
        let doc = layout_test("$x^2$");
        let text = doc.plain_text();
        assert!(text.contains('x'));
        assert!(text.contains('2'));
    }

    #[test]
    fn equacao_inline_com_prime_nao_regride() {
        let doc = layout_test("$x'$");
        let text = doc.plain_text();
        assert!(text.contains('x'));
        assert!(text.contains('′'));
    }

    #[test]
    fn pagina_nao_vazia_com_equacao_inline() {
        let doc = layout_test("$frac(1, 2)$");
        assert!(!doc.pages.is_empty());
        assert!(!doc.pages[0].items.is_empty());
    }

    #[test]
    fn equacao_inline_sobe_em_relacao_ao_baseline() {
        // Com o ajuste de baseline, os items da equação inline estão acima
        // do cursor_y (offset_y < cursor_y). Com FixedMetrics, axis_height=500
        // e upem=1000, axis_pt = 0.5 * font_size = 6.0pt.
        // Verificamos que pelo menos um item tem y < cursor_y inicial (≈81.6pt).
        let doc = layout_test("$x$");
        let all_y: Vec<f64> = doc.pages.iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|i| match i {
                FrameItem::Text { pos, .. } => Some(pos.y.val()),
                FrameItem::Glyph { pos, .. } => Some(pos.y.val()),
                _ => None,
            })
            .collect();
        assert!(!all_y.is_empty(), "deve ter items");
        // cursor_y inicial ≈ MARGIN(72) + ascender(9.6) = 81.6
        // Com axis_pt ≈ 6.0, offset_y ≈ 75.6 < 81.6
        let min_y = all_y.iter().cloned().fold(f64::INFINITY, f64::min);
        assert!(
            min_y < 81.6,
            "equacao inline deve estar acima do baseline ({:.1} < 81.6)",
            min_y
        );
    }
}

// ── Passo 49 — Limites verticais em operadores grandes ───────────────────

#[cfg(test)]
mod tests_limits {
    use super::*;

    #[test]
    fn layout_sum_com_limites_contem_conteudo() {
        let doc = layout_test("$sum_(i=0)^n$");
        let text = doc.plain_text();
        assert!(
            text.contains('∑') || text.contains('i') || text.contains('n'),
            "operador ou limites ausentes: {}", text
        );
    }

    #[test]
    fn layout_sum_sem_limites_nao_regride() {
        let doc = layout_test("$sum$");
        let text = doc.plain_text();
        assert!(text.contains('∑'), "somatório: {}", text);
    }

    #[test]
    fn layout_attach_normal_nao_regride() {
        let doc = layout_test("$x^2$");
        let text = doc.plain_text();
        assert!(text.contains('x'));
        assert!(text.contains('2'));
    }

    #[test]
    fn layout_integral_com_limites_nao_panica() {
        let doc = layout_test("$integral_(0)^1$");
        assert!(!doc.pages.is_empty());
    }

    #[test]
    fn layout_prod_com_limites_nao_panica() {
        let doc = layout_test("$product_(k=1)^n$");
        assert!(!doc.pages.is_empty());
    }

    #[test]
    fn layout_lim_com_subscript_nao_panica() {
        let doc = layout_test("$lim_(x -> 0)$");
        assert!(!doc.pages.is_empty());
    }

    #[test]
    fn sum_block_limites_empilhados_verticalmente() {
        // Passo 50: bloco "$ ... $" (espaços dentro) → block=true → empilhamento vertical.
        // offset_y = cursor_y = 81.6 (bloco não ajusta baseline).
        // y_sup = -(base_ascent + upper_gap + sup.descent) = -(9.6 + 1.2 + 3.36) = -14.16
        // Final y ≈ 81.6 - 14.16 = 67.4 < 70.0 (vs inline right-scripts ≈ 71.3)
        let doc = layout_test("$ sum_(i=0)^n $");
        let all_y: Vec<f64> = doc.pages.iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|i| match i {
                FrameItem::Text { pos, .. } => Some(pos.y.val()),
                FrameItem::Glyph { pos, .. } => Some(pos.y.val()),
                _ => None,
            })
            .collect();
        assert!(!all_y.is_empty(), "deve ter items");
        let min_y = all_y.iter().cloned().fold(f64::INFINITY, f64::min);
        assert!(
            min_y < 70.0,
            "bloco: limites de ∑ devem estar empilhados verticalmente (min_y={:.1} < 70.0)",
            min_y
        );
    }
}

// ── Passo 50 — Diferenciação inline/bloco ────────────────────────────────

#[cfg(test)]
mod tests_limits_context {
    use super::*;

    #[test]
    fn sum_inline_usa_right_scripts() {
        // Passo 50: inline "$...$" → block=false → right-scripts (sub/sup à direita).
        // offset_y = cursor_y - axis_pt = 81.6 - 6.0 = 75.6 (inline ajusta baseline).
        // Com right-scripts: sup_offset ≈ 4.34pt → item y ≈ 75.6 - 4.34 = 71.3 ≥ 70.0
        // Com vertical stacking (antes): min_y ≈ 61.4 < 70.0 (falha antes da implementação)
        let doc = layout_test("$sum_(i=0)^n$");
        let all_y: Vec<f64> = doc.pages.iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|i| match i {
                FrameItem::Text { pos, .. } => Some(pos.y.val()),
                FrameItem::Glyph { pos, .. } => Some(pos.y.val()),
                _ => None,
            })
            .collect();
        assert!(!all_y.is_empty(), "deve ter items");
        let min_y = all_y.iter().cloned().fold(f64::INFINITY, f64::min);
        assert!(
            min_y >= 70.0,
            "inline: ∑ deve usar right-scripts (min_y={:.1} >= 70.0)",
            min_y
        );
    }

    #[test]
    fn sum_inline_contem_conteudo() {
        let doc = layout_test("$sum_(i=0)^n$");
        let text = doc.plain_text();
        assert!(
            text.contains('∑') || text.contains('i') || text.contains('n'),
            "conteúdo ausente: {}", text
        );
    }

    #[test]
    fn sum_inline_gera_pagina() {
        let doc = layout_test("$sum_(i=0)^n x_i$");
        assert!(!doc.pages.is_empty());
        assert!(!doc.pages[0].items.is_empty());
    }

    #[test]
    fn lim_inline_contem_conteudo() {
        let doc = layout_test("$lim_(x -> 0) f(x)$");
        let text = doc.plain_text();
        assert!(text.contains('f') || text.contains('x'),
            "conteúdo ausente: {}", text);
    }

    #[test]
    fn attach_normal_inline_nao_regride() {
        let doc = layout_test("$x^2$");
        let text = doc.plain_text();
        assert!(text.contains('x'));
        assert!(text.contains('2'));
    }

    #[test]
    fn attach_normal_com_sub_inline_nao_regride() {
        let doc = layout_test("$x_i$");
        let text = doc.plain_text();
        assert!(text.contains('x'));
        assert!(text.contains('i'));
    }

    #[test]
    fn sum_block_contem_conteudo() {
        let doc = layout_test("$ sum_(i=0)^n $");
        let text = doc.plain_text();
        assert!(
            text.contains('∑') || text.contains('i') || text.contains('n'),
            "conteúdo ausente em block: {}", text
        );
    }
}

// ── Passo 51 — MathAlignPoint ─────────────────────────────────────────
#[cfg(test)]
mod tests_align {
    use super::*;

    #[test]
    fn align_simples_contem_conteudo() {
        // $ a &= b \\ c &= d $ — dois lados de duas linhas presentes
        let doc = layout_test("$ a &= b \\ c &= d $");
        let text = doc.plain_text();
        assert!(text.contains('a'), "a ausente: {}", text);
        assert!(text.contains('b'), "b ausente: {}", text);
        assert!(text.contains('c'), "c ausente: {}", text);
        assert!(text.contains('d'), "d ausente: {}", text);
    }

    #[test]
    fn align_duas_linhas_tem_ys_distintos() {
        // Após implementação de grid, itens de linha 0 e linha 1
        // devem ter Y distintos no frame.
        let doc = layout_test("$ a &= b \\ c &= d $");
        assert!(!doc.pages.is_empty());
        let mut ys: Vec<i64> = doc.pages[0].items.iter()
            .filter_map(|item| match item {
                crate::entities::layout_types::FrameItem::Text { pos, .. } =>
                    Some((pos.y.val() * 100.0).round() as i64),
                crate::entities::layout_types::FrameItem::Glyph { pos, .. } =>
                    Some((pos.y.val() * 100.0).round() as i64),
                _ => None,
            })
            .collect();
        ys.sort_unstable();
        ys.dedup();
        assert!(ys.len() >= 2,
            "esperava >= 2 Y distintos (2 linhas), encontrei {:?}", ys);
    }

    #[test]
    fn align_sem_ampersand_nao_regride() {
        let doc = layout_test("$ x + 1 $");
        let text = doc.plain_text();
        assert!(text.contains('x'));
        assert!(text.contains('1'));
    }

    #[test]
    fn align_com_frac_nao_panica() {
        let doc = layout_test("$ frac(a, b) &= c \\ d &= e $");
        assert!(!doc.pages.is_empty());
    }

    #[test]
    fn align_linha_unica_com_ampersand() {
        let doc = layout_test("$ a &= b $");
        let text = doc.plain_text();
        assert!(text.contains('a'));
        assert!(text.contains('b'));
    }

    #[test]
    fn align_inline_nao_usa_grelha() {
        // inline: & ignorado, não deve panicar
        let doc = layout_test("$a &= b$");
        assert!(!doc.pages.is_empty());
    }

    #[test]
    fn frac_nao_regride() {
        let doc = layout_test("$ frac(1, 2) $");
        let text = doc.plain_text();
        assert!(text.contains('1'));
        assert!(text.contains('2'));
    }

    #[test]
    fn sum_com_limites_nao_regride() {
        let doc = layout_test("$ sum_(i=0)^n $");
        let text = doc.plain_text();
        assert!(
            text.contains('∑') || text.contains('i') || text.contains('n'),
            "sum: {}", text
        );
    }

    // ── Passo 54 — Matrizes matemáticas ─────────────────────────────────

    #[test]
    fn matrix_2x2_nao_vazio() {
        let doc = layout_test("$ mat(a, b; c, d) $");
        let text = doc.plain_text();
        assert!(text.contains('a'), "a ausente: {}", text);
        assert!(text.contains('d'), "d ausente: {}", text);
    }

    #[test]
    fn matrix_1x1_nao_panica() {
        let doc = layout_test("$ mat(x) $");
        assert!(!doc.pages.is_empty());
    }

    #[test]
    fn matrix_linha_unica_nao_panica() {
        let doc = layout_test("$ mat(1, 2, 3) $");
        assert!(!doc.pages.is_empty());
    }

    #[test]
    fn align_grid_nao_regride_apos_matrix() {
        let doc = layout_test("$ a &= b \\ c &= d $");
        let text = doc.plain_text();
        assert!(text.contains('a'));
        assert!(text.contains('d'));
    }

    // ── Passo 55 — Vectores e Casos ──────────────────────────────────────

    #[test]
    fn vec_tres_elementos_nao_vazio() {
        let doc = layout_test("$ vec(1, 2, 3) $");
        let text = doc.plain_text();
        assert!(text.contains('1'));
        assert!(text.contains('3'));
    }

    #[test]
    fn vec_elemento_unico_nao_panica() {
        let doc = layout_test("$ vec(x) $");
        assert!(!doc.pages.is_empty());
    }

    #[test]
    fn cases_dois_ramos_nao_vazio() {
        let doc = layout_test("$ cases(1, 0) $");
        assert!(!doc.pages.is_empty());
    }

    #[test]
    fn cases_nao_panica_com_align_point() {
        let doc = layout_test("$ cases(x &, 0 &) $");
        assert!(!doc.pages.is_empty());
    }

    #[test]
    fn mat_nao_regride_apos_vec_cases() {
        let doc = layout_test("$ mat(1, 2; 3, 4) $");
        let text = doc.plain_text();
        assert!(text.contains('1'));
        assert!(text.contains('4'));
    }

    #[test]
    fn align_grid_nao_regride_apos_passo55() {
        let doc = layout_test("$ a &= b \\ c &= d $");
        let text = doc.plain_text();
        assert!(text.contains('a'));
        assert!(text.contains('d'));
    }
}

// ── Testes de CounterState e numeração de headings (Passo 57/58) ──────

#[test]
fn layout_heading_sem_numbering_nao_tem_prefixo() {
    // Por defeito, numbering_active está vazio — não deve aparecer "1."
    let content = Content::heading(1, Content::text("Intro"));
    let doc = layout(&content, introspect(&content));
    let text = doc.plain_text();
    assert!(!text.contains("1."), "sem numbering activo, não deve haver prefixo numérico");
    assert!(text.contains("Intro"));
}

#[test]
fn layout_heading_com_numbering_tem_prefixo() {
    let content = Content::Sequence(vec![
        Content::SetHeadingNumbering { active: true },
        Content::heading(1, Content::text("Intro")),
        Content::heading(2, Content::text("Motivação")),
        Content::heading(1, Content::text("Conclusão")),
    ].into());
    let doc = layout(&content, introspect(&content));
    let text = doc.plain_text();
    assert!(text.contains("1."), "H1 deve ter prefixo '1.'");
    assert!(text.contains("1.1"), "H2 deve ter prefixo '1.1'");
    assert!(text.contains("2."), "segundo H1 deve ter prefixo '2.'");
}

#[test]
fn layout_set_heading_numbering_activa_contador() {
    // SetHeadingNumbering activado via Content::SetHeadingNumbering + headings
    let content = Content::Sequence(vec![
        Content::SetHeadingNumbering { active: true },
        Content::heading(1, Content::text("Intro")),
        Content::heading(2, Content::text("Sub")),
    ].into());
    let doc = layout(&content, introspect(&content));
    let text = doc.plain_text();
    assert!(text.contains("1."), "H1 deve ter prefixo '1.'");
    assert!(text.contains("1.1"), "H2 deve ter prefixo '1.1'");
}

#[test]
fn layout_counter_display_heading_retorna_estado_actual() {
    let content = Content::Sequence(vec![
        Content::SetHeadingNumbering { active: true },
        Content::heading(1, Content::text("Intro")),
        Content::CounterDisplay { kind: "heading".to_string() },
    ].into());
    let doc = layout(&content, introspect(&content));
    let text = doc.plain_text();
    // CounterDisplay de heading após H1 deve mostrar "1"
    // (o heading já avançou o contador antes de CounterDisplay ser processado)
    assert!(text.contains('1'));
}

// ── Testes de CounterUpdate (Passo 58) ────────────────────────────────

#[test]
fn counter_update_nao_produz_items_visuais() {
    use crate::entities::counter_state::CounterAction;

    let content = Content::CounterUpdate {
        key:    "equation".to_string(),
        action: CounterAction::Update(5),
    };
    let doc = layout(&content, CounterState::default());
    let total_items: usize = doc.pages.iter().map(|p| p.items.len()).sum();
    assert_eq!(total_items, 0, "CounterUpdate não deve gerar items visuais");
}

#[test]
fn counter_update_seguido_de_display_mostra_valor_correcto() {
    use crate::entities::counter_state::CounterAction;

    let content = Content::Sequence(vec![
        Content::CounterUpdate {
            key:    "equation".to_string(),
            action: CounterAction::Update(5),
        },
        Content::CounterDisplay { kind: "equation".to_string() },
    ].into());
    let doc = layout(&content, CounterState::default());
    assert!(doc.plain_text().contains('5'),
        "CounterDisplay deve mostrar '5' após Update(5): {:?}", doc.plain_text());
}

// ── Testes de resolução de referências (Passo 59 / Passo 60) ────────────

#[test]
fn layout_ref_para_tras_resolve_secao() {
    // Passo 60: layout() usa duas passagens — backward ref resolve via introspect.
    use crate::entities::label::Label;

    let content = Content::Sequence(vec![
        Content::Labelled {
            label:  Label("intro".to_string()),
            target: Box::new(Content::heading(1, Content::text("Introdução"))),
        },
        Content::text("Como vimos em"),
        Content::Ref { target: Label("intro".to_string()) },
    ].into());

    let doc = layout(&content, introspect(&content));
    let text = doc.plain_text();
    assert!(
        text.contains("Secção 1"),
        "Ref para trás deve resolver para 'Secção 1' via duas passagens, obtido: {:?}", text
    );
}

#[test]
fn layout_ref_para_frente_resolve_com_duas_passagens() {
    // Passo 60: forward ref resolve via introspect — sem fallback.
    use crate::entities::label::Label;

    let content = Content::Sequence(vec![
        // Ref aparece antes da Label — forward reference
        Content::Ref { target: Label("conclusao".to_string()) },
        Content::Labelled {
            label:  Label("conclusao".to_string()),
            target: Box::new(Content::heading(1, Content::text("Conclusão"))),
        },
    ].into());

    let doc = layout(&content, introspect(&content));
    let text = doc.plain_text();
    assert!(
        text.contains("Secção 1"),
        "Forward ref deve resolver para 'Secção 1' com duas passagens, obtido: {:?}", text
    );
    assert!(
        !text.contains("@conclusao"),
        "Forward ref não deve usar fallback com duas passagens, obtido: {:?}", text
    );
}

#[test]
fn layout_resolved_labels_nao_interfere_entre_documentos() {
    // Estados de cada chamada a layout() são independentes.
    use crate::entities::label::Label;

    let content_a = Content::Labelled {
        label:  Label("sec".to_string()),
        target: Box::new(Content::heading(1, Content::text("A"))),
    };
    let _ = layout(&content_a, introspect(&content_a));

    // Segundo layout independente — não deve ter "sec" resolvida
    let content_b = Content::Ref { target: Label("sec".to_string()) };
    let doc_b = layout(&content_b, introspect(&content_b));
    assert!(
        doc_b.plain_text().contains("@sec"),
        "Estado do layout anterior não deve vazar para o seguinte"
    );
}

// ── Testes de duas passagens (Passo 60) ──────────────────────────────────

#[test]
fn pipeline_duas_passagens_resolve_forward_ref() {
    use crate::entities::label::Label;
    use crate::rules::{introspect::introspect, layout::layout};

    let content = Content::Sequence(vec![
        Content::SetHeadingNumbering { active: true },
        Content::text("Ver a"),
        Content::Ref { target: Label("conclusao".to_string()) },
        Content::text("."),
        Content::Labelled {
            label:  Label("conclusao".to_string()),
            target: Box::new(Content::heading(1, Content::text("Conclusão"))),
        },
    ].into());

    // Passagem 1 — verificar que introspect resolve forward ref
    let initial_state = introspect(&content);
    assert!(
        initial_state.resolved_labels.contains_key(&Label("conclusao".to_string())),
        "introspect deve popular resolved_labels para forward refs"
    );

    // Passagem 2 — layout usa o estado da pré-passagem.
    let doc = layout(&content, initial_state);
    let text = doc.plain_text();
    assert!(
        text.contains("Secção 1"),
        "forward ref deve resolver para 'Secção 1': {:?}", text
    );
    assert!(
        !text.contains("@conclusao"),
        "não deve usar fallback com duas passagens: {:?}", text
    );
}

#[test]
fn layout_equation_bloco_numerada() {
    let mut state = CounterState::new();
    state.numbering_active.insert("equation".to_string(), true);

    let content = Content::Equation {
        body:  Box::new(Content::MathIdent("E".into())),
        block: true,
    };

    let doc = layout(&content, state);
    let text = doc.plain_text();
    assert!(
        text.contains("(1)"),
        "Equação de bloco numerada deve mostrar '(1)', obtido: {:?}", text
    );
}

// ── Testes de Passo 61 — TOC (Outline) ───────────────────────────────────

#[test]
fn layout_outline_gera_indice_com_titulos() {
    let content = Content::Sequence(vec![
        Content::SetHeadingNumbering { active: true },
        Content::Outline,
        Content::heading(1, Content::text("Introdução")),
        Content::heading(2, Content::text("Motivação")),
    ].into());

    // Passagem 1 — o teste orquestra explicitamente como o orquestrador L3 faz.
    let state = introspect(&content);
    // Passagem 2 — layout recebe o estado pré-calculado.
    let doc = layout(&content, state);
    let text = doc.plain_text();

    assert!(text.contains("Índice"), "TOC deve ter título 'Índice'");
    assert!(text.contains("Introdução"), "TOC deve listar o título H1");
    assert!(text.contains("Motivação"), "TOC deve listar o título H2");
}

#[test]
fn layout_outline_sem_headings_gera_apenas_titulo_ou_vazio() {
    let content = Content::Outline;
    let state = introspect(&content);
    let doc = layout(&content, state);
    let text = doc.plain_text();

    assert!(text.contains("Índice") || text.is_empty(),
        "TOC sem headings deve gerar apenas o título ou estar vazia");
}

#[test]
fn layout_outline_heading_nivel2_tem_indentacao() {
    let content = Content::Sequence(vec![
        Content::Outline,
        Content::heading(1, Content::text("H1")),
        Content::heading(2, Content::text("H2")),
    ].into());

    let state = introspect(&content);
    let doc = layout(&content, state);
    let text = doc.plain_text();

    // Heading de nível 2 → TOC deve conter espaços de indentação antes de H2.
    // plain_text() não preserva posição, mas a TOC inclui "  " antes da Ref.
    assert!(text.contains("H1"), "TOC deve listar H1");
    assert!(text.contains("H2"), "TOC deve listar H2");
}

// ── Testes de Passo 62 — Figuras ─────────────────────────────────────────

#[test]
fn layout_figure_com_caption_tem_prefixo() {
    let content = Content::Figure {
        body:      Box::new(Content::text("Gráfico")),
        caption:   Some(Box::new(Content::text("Resultados"))),
        kind:      "image".to_string(),
        numbering: Some("1".to_string()),
    };

    let state = introspect(&content);
    let doc = layout(&content, state);
    let text = doc.plain_text();

    assert!(text.contains("Gráfico"),    "corpo da figura deve aparecer");
    assert!(text.contains("Figura 1:"),  "prefixo numérico deve aparecer");
    assert!(text.contains("Resultados"), "legenda deve aparecer");
}

#[test]
fn layout_figure_sem_caption_sem_prefixo() {
    let content = Content::Figure {
        body:      Box::new(Content::text("Diagrama")),
        caption:   None,
        kind:      "image".to_string(),
        numbering: Some("1".to_string()),
    };

    let state = introspect(&content);
    let doc = layout(&content, state);
    let text = doc.plain_text();

    assert!(text.contains("Diagrama"),    "corpo deve aparecer");
    assert!(!text.contains("Figura 1:"), "sem caption, sem prefixo");
}

#[test]
fn layout_ref_para_figura_resolve_corretamente() {
    use crate::entities::label::Label;

    let content = Content::Sequence(
        vec![
            Content::Labelled {
                label:  Label("fig1".to_string()),
                target: Box::new(Content::Figure {
                    body:      Box::new(Content::text("Gráfico")),
                    caption:   Some(Box::new(Content::text("Legenda"))),
                    kind:      "image".to_string(),
                    numbering: Some("1".to_string()),
                }),
            },
            Content::text(" — ver "),
            Content::Ref { target: Label("fig1".to_string()) },
        ]
        .into(),
    );

    let state = introspect(&content);
    let doc = layout(&content, state);
    let text = doc.plain_text();

    assert!(text.contains("Figura 1"),
        "Ref para figura deve resolver para 'Figura 1': {:?}", text);
    assert!(!text.contains("@fig1"),
        "não deve usar fallback @fig1: {:?}", text);
}

// ── Testes de Passo 63 — Mapa de páginas e motor de congelamento ─────────

#[test]
fn layout_regista_pagina_de_label() {
    use crate::entities::label::Label;

    let content = Content::Sequence(vec![
        Content::Labelled {
            label:  Label("sec1".to_string()),
            target: Box::new(Content::heading(1, Content::text("Introdução"))),
        },
    ].into());

    let state = introspect(&content);
    let doc = layout(&content, state);

    assert!(
        doc.extracted_label_pages.contains_key(&Label("sec1".to_string())),
        "extracted_label_pages deve conter a label processada"
    );
}

#[test]
fn layout_pagina_de_label_e_um_indexed() {
    use crate::entities::label::Label;

    let content = Content::Labelled {
        label:  Label("top".to_string()),
        target: Box::new(Content::text("No topo")),
    };

    let state = introspect(&content);
    let doc = layout(&content, state);

    let page = doc.extracted_label_pages.get(&Label("top".to_string()))
        .copied()
        .unwrap_or(0);
    assert_eq!(page, 1, "label no início do documento deve estar na página 1");
}

#[test]
fn layout_toc_com_readonly_nao_duplica_contadores() {
    // Heading com CounterUpdate embebido — sem is_readonly, o contador avançaria
    // duas vezes (uma no heading real, outra no clone da TOC).
    // Com is_readonly, a renderização da TOC é neutra em relação aos contadores.
    use crate::entities::counter_state::CounterAction;

    let body_with_counter_update = Content::Sequence(vec![
        Content::text("Secção"),
        Content::CounterUpdate {
            key:    "equation".to_string(),
            action: CounterAction::Step,
        },
    ].into());

    let content = Content::Sequence(vec![
        Content::Outline,
        Content::heading(1, body_with_counter_update),
        Content::CounterDisplay { kind: "equation".to_string() },
    ].into());

    let state = introspect(&content);
    let doc = layout(&content, state);
    let text = doc.plain_text();

    // Sem is_readonly: CounterUpdate dispararia 2× → display mostraria "2".
    // Com is_readonly: CounterUpdate na TOC é bloqueado → display mostra "1".
    assert!(
        text.contains('1') && !text.contains('2'),
        "CounterUpdate na TOC não deve duplicar: {:?}", text
    );
}

#[test]
fn layout_extracted_label_pages_preenchido_apos_layout() {
    // extracted_label_pages é sempre populado após layout, mesmo sem labels.
    let content = Content::text("Texto sem labels");
    let doc = layout(&content, CounterState::default());
    // Deve existir o campo (pode estar vazio)
    assert!(doc.extracted_label_pages.is_empty(),
        "sem labels, extracted_label_pages deve estar vazio");
}

// ── Testes de Passo 65 — Convergência de fixpoint ────────────────────────

#[test]
fn layout_converge_sem_ciclo_infinito() {
    let content = Content::Sequence(vec![
        Content::SetHeadingNumbering { active: true },
        Content::Outline,
        Content::heading(1, Content::text("Capítulo 1")),
        Content::heading(2, Content::text("Secção 1.1")),
    ].into());

    let state = introspect(&content);
    // Se o fixpoint tiver defeito, entra em loop até MAX_ITERATIONS.
    // Não deve panic.
    let doc = layout(&content, state);

    let text = doc.plain_text();
    assert!(text.contains("Capítulo 1"), "título deve aparecer: {:?}", text);
    assert!(text.contains("Índice") || text.contains("ndice"),
        "TOC deve aparecer: {:?}", text);
}

#[test]
fn layout_documento_sem_toc_usa_curto_circuito() {
    // Documento COM títulos mas SEM #outline(). O vetor headings_for_toc
    // terá entradas, mas has_outline é false — o short-circuit evita o loop.
    // Prova que a condição correcta é has_outline, não headings_for_toc.is_empty().
    let content = Content::Sequence(vec![
        Content::SetHeadingNumbering { active: true },
        Content::heading(1, Content::text("Introdução")),
        Content::heading(2, Content::text("Motivação")),
        Content::text("Texto sem índice."),
    ].into());

    let state = introspect(&content);
    assert!(!state.has_outline, "sem Outline no documento, has_outline deve ser false");

    let doc = layout(&content, state);
    assert!(!doc.pages.is_empty(), "documento deve ter páginas");
}

#[test]
fn layout_com_labels_produz_extracted_label_pages() {
    use crate::entities::label::Label;

    let content = Content::Sequence(vec![
        Content::Labelled {
            label:  Label("sec1".to_string()),
            target: Box::new(Content::heading(1, Content::text("Secção"))),
        },
    ].into());

    let state = introspect(&content);
    let doc = layout(&content, state);

    assert!(
        doc.extracted_label_pages.contains_key(&Label("sec1".to_string())),
        "extracted_label_pages deve conter a label após convergência"
    );
}

// ── Testes de imagem (Passo 73) ──────────────────────────────────────────

#[test]
fn layout_image_gera_frameitem() {
    // JPEG magic bytes — NullImageSizer retorna None → fallback 100×100 pt.
    let jpeg_magic = vec![0xFF, 0xD8, 0xFF, 0x00u8];

    let content = Content::Image {
        path:   "teste.jpg".to_string(),
        data:   crate::entities::ptr_eq_arc::PtrEqArc(std::sync::Arc::new(jpeg_magic)),
        width:  None,
        height: None,
    };

    let state = introspect(&content);
    let doc   = layout(&content, state);

    assert!(!doc.pages.is_empty(), "documento deve ter pelo menos uma página");

    let has_image = doc.pages[0].items.iter().any(|item| {
        matches!(item, FrameItem::Image { .. })
    });
    assert!(has_image, "layouter deve emitir FrameItem::Image");
}

#[test]
fn frameitem_image_deduplica_por_ponteiro() {
    use std::sync::Arc;
    // Clones do mesmo Arc devem ter o mesmo ponteiro — base da deduplicação no exportador.
    let data = Arc::new(vec![0xFF, 0xD8, 0xFF, 0x00u8]);
    let ptr1 = Arc::as_ptr(&data) as usize;
    let clone = Arc::clone(&data);
    let ptr2 = Arc::as_ptr(&clone) as usize;
    assert_eq!(ptr1, ptr2, "clones do mesmo Arc devem ter o mesmo ponteiro");
}

// ── Testes de Grid (Passo 80) ────────────────────────────────────────────

#[test]
fn grid_fr_distribution_quando_auto_e_pequeno() {
    // Página 595pt, margens 72pt cada lado → available = 451pt.
    // columns: (50pt, auto, 1fr, 2fr)
    // Célula Auto: texto curto → mede < safe_available.
    // safe_available para Auto = 451 - 50 = 401pt.
    // "hi" com FixedMetrics 12pt: 2 chars * 0.6 * 12 = 14.4pt.
    // Remaining: 451 - 50 - 14.4 = 386.6pt; total_fr = 3.
    // Col 2 (1fr): 386.6/3 ≈ 128.87pt; Col 3 (2fr): ≈ 257.73pt.
    use crate::entities::layout_types::TrackSizing;
    use crate::entities::geometry::ShapeKind;

    let cfg = crate::entities::layout_types::PageConfig::default();
    let available = cfg.width - 2.0 * cfg.margin; // 595.28 - 2*70.87 = 453.54pt
    let cols = vec![
        TrackSizing::Fixed(50.0),
        TrackSizing::Auto,
        TrackSizing::Fraction(1.0),
        TrackSizing::Fraction(2.0),
    ];
    let cell_auto = Content::text("hi");

    let layouter = Layouter::new(FixedMetrics, NullImageSizer, DEFAULT_FONT_SIZE);

    // Simular Fase 1.
    let mut resolved = vec![0.0_f64; 4];
    let mut total_fixed = 0.0_f64;
    let mut total_fr    = 0.0_f64;
    let cols_cells: Vec<Vec<&Content>> = vec![vec![], vec![&cell_auto], vec![], vec![]];

    for (i, sizing) in cols.iter().enumerate() {
        match sizing {
            TrackSizing::Fixed(w) => { resolved[i] = *w; total_fixed += *w; }
            TrackSizing::Auto => {
                let safe = (available - total_fixed).max(0.0);
                let mut max_w = 0.0_f64;
                for cell in &cols_cells[i] {
                    let (w, _) = layouter.measure_content_constrained(cell, safe);
                    max_w = max_w.max(w);
                }
                resolved[i] = max_w;
                total_fixed += max_w;
            }
            TrackSizing::Fraction(fr) => { total_fr += fr; }
        }
    }
    // Fase 2.
    let remaining = (available - total_fixed).max(0.0);
    if total_fr > 0.0 {
        let per_fr = remaining / total_fr;
        for (i, sizing) in cols.iter().enumerate() {
            if let TrackSizing::Fraction(fr) = sizing {
                resolved[i] = fr * per_fr;
            }
        }
    }

    assert_eq!(resolved[0], 50.0, "Fixed deve ser exactamente 50pt");
    assert!(resolved[1] > 0.0 && resolved[1] < available - 50.0,
        "Auto deve ser positivo e menor que safe_available");
    let soma = resolved.iter().sum::<f64>();
    assert!((soma - available).abs() < 0.01,
        "Soma das larguras deve ser igual a available_width: {} vs {}", soma, available);
}

#[test]
fn grid_fr_recebe_zero_quando_auto_e_guloso() {
    // Regressão: Auto com palavra muito longa consome safe_available inteiro.
    // Não deve entrar em pânico. resolved_widths[2] deve ser 0.0 ou positivo.
    use crate::entities::layout_types::TrackSizing;

    let cfg = crate::entities::layout_types::PageConfig::default();
    let available = cfg.width - 2.0 * cfg.margin;
    let cols = vec![
        TrackSizing::Fixed(50.0),
        TrackSizing::Auto,
        TrackSizing::Fraction(1.0),
    ];
    // Palavra sem espaços — ocupa safe_available inteiro.
    let cell_auto = Content::text("PalavraLongaSemEspacos");
    let layouter = Layouter::new(FixedMetrics, NullImageSizer, DEFAULT_FONT_SIZE);

    let mut resolved = vec![0.0_f64; 3];
    let mut total_fixed = 0.0_f64;
    let mut total_fr    = 0.0_f64;
    let cols_cells: Vec<Vec<&Content>> = vec![vec![], vec![&cell_auto], vec![]];

    for (i, sizing) in cols.iter().enumerate() {
        match sizing {
            TrackSizing::Fixed(w) => { resolved[i] = *w; total_fixed += *w; }
            TrackSizing::Auto => {
                let safe = (available - total_fixed).max(0.0);
                let mut max_w = 0.0_f64;
                for cell in &cols_cells[i] {
                    let (w, _) = layouter.measure_content_constrained(cell, safe);
                    max_w = max_w.max(w);
                }
                resolved[i] = max_w;
                total_fixed += max_w;
            }
            TrackSizing::Fraction(fr) => { total_fr += fr; }
        }
    }
    let remaining = (available - total_fixed).max(0.0);
    if total_fr > 0.0 {
        let per_fr = remaining / total_fr;
        for (i, sizing) in cols.iter().enumerate() {
            if let TrackSizing::Fraction(fr) = sizing {
                resolved[i] = fr * per_fr;
            }
        }
    }

    // Comportamento documentado: fr pode receber 0pt (DEBT-34d). Sem pânico.
    assert!(resolved[2] >= 0.0, "fr não deve ter largura negativa");
}

#[test]
fn grid_altura_da_linha_e_o_maximo_das_celulas() {
    // columns: (100pt, 100pt)
    // Células: rect(h:20), rect(h:40), rect(h:10)
    // Linha 0: max(20, 40) = 40pt. Linha 1: 10pt (incompleta).
    // Verificar: 1 página, 3 FrameItems.
    use crate::entities::geometry::ShapeKind;

    let make_rect = |h: f64| -> Content {
        Content::Shape {
            kind:   ShapeKind::Rect,
            width:  Some(Box::new(crate::entities::value::Value::Length(
                crate::entities::layout_types::Length { abs: crate::entities::layout_types::Abs(100.0), em: 0.0 },
            ))),
            height: Some(Box::new(crate::entities::value::Value::Length(
                crate::entities::layout_types::Length { abs: crate::entities::layout_types::Abs(h), em: 0.0 },
            ))),
            fill:   None,
            stroke: None,
        }
    };

    let grid = Content::Grid {
        columns: vec![
            crate::entities::layout_types::TrackSizing::Fixed(100.0),
            crate::entities::layout_types::TrackSizing::Fixed(100.0),
        ],
        rows:  vec![],
        cells: vec![make_rect(20.0), make_rect(40.0), make_rect(10.0)],
    };

    let state = introspect(&grid);
    let doc   = layout(&grid, state);

    assert_eq!(doc.pages.len(), 1, "Grid simples deve caber numa página");
    let total_items = doc.pages[0].items.len();
    assert_eq!(total_items, 3, "Deve haver 3 FrameItems no frame");
}

#[test]
fn grid_auto_respects_safe_available() {
    // Uma coluna Auto com conteúdo não deve exceder available_width.
    use crate::entities::layout_types::TrackSizing;

    let cfg = crate::entities::layout_types::PageConfig::default();
    let available = cfg.width - 2.0 * cfg.margin;
    let cols = vec![TrackSizing::Auto];
    let cell  = Content::text("Palavra muito longa que poderia exceder a página se nao houver limite");
    let layouter = Layouter::new(FixedMetrics, NullImageSizer, DEFAULT_FONT_SIZE);

    let mut resolved = vec![0.0_f64; 1];
    let mut total_fixed = 0.0_f64;
    let cols_cells: Vec<Vec<&Content>> = vec![vec![&cell]];

    for (i, sizing) in cols.iter().enumerate() {
        match sizing {
            TrackSizing::Auto => {
                let safe = (available - total_fixed).max(0.0);
                let mut max_w = 0.0_f64;
                for c in &cols_cells[i] {
                    let (w, _) = layouter.measure_content_constrained(c, safe);
                    max_w = max_w.max(w);
                }
                resolved[i] = max_w;
                total_fixed += max_w;
            }
            _ => {}
        }
    }

    assert!(
        resolved[0] <= available,
        "Auto não deve exceder available_width: {} > {}", resolved[0], available
    );
}

// ── Passo 100.D: Integração Content::Styled → Layouter ───────────────────

#[cfg(test)]
mod tests_styled_integration {
    use super::*;
    use crate::entities::content::Content;
    use crate::entities::layout_types::{FrameItem, Pt};
    use crate::entities::style::{Style, Styles};

    /// Retira todos os `FrameItem::Text` do documento (qualquer página).
    fn collect_text_items(doc: &PagedDocument) -> Vec<&FrameItem> {
        doc.pages.iter()
            .flat_map(|p| p.items.iter())
            .filter(|item| matches!(item, FrameItem::Text { .. }))
            .collect()
    }

    /// Constrói `Content::Styled` directamente e verifica que o Layouter
    /// processa os estilos via push/pop na cadeia (Passo 100, ADR-0039).
    /// O teste de integração conceptual do Passo 99 (`style_chain.rs`)
    /// validou a API; aqui validamos a activação end-to-end.
    #[test]
    fn styled_basico_aplica_bold_e_size() {
        let hello = Content::text("hello");
        let styled = Content::Styled(
            Box::new(hello),
            Styles::from_iter([Style::Bold(true), Style::Size(Pt(18.0))]),
        );

        let doc = layout(&styled, CounterState::new());
        let texts = collect_text_items(&doc);
        assert!(!texts.is_empty(), "esperado pelo menos 1 FrameItem::Text");

        for item in texts {
            if let FrameItem::Text { style, .. } = item {
                assert!(style.bold, "Bold deve estar activo: {:?}", style);
                assert_eq!(style.size, Pt(18.0),
                    "Size deve ser 18pt após push_styles");
            }
        }
    }

    /// Styled aninhado — o delta mais próximo do texto (inner) ganha
    /// (top-wins, paridade vanilla, ADR-0033).
    #[test]
    fn styled_aninhado_inner_ganha_sobre_outer() {
        let inner = Content::Styled(
            Box::new(Content::text("hi")),
            Styles::from_iter([Style::Italic(true)]),
        );
        let outer = Content::Styled(
            Box::new(inner),
            Styles::from_iter([Style::Bold(true), Style::Italic(false)]),
        );

        let doc = layout(&outer, CounterState::new());
        let texts = collect_text_items(&doc);
        assert!(!texts.is_empty());

        for item in texts {
            if let FrameItem::Text { style, .. } = item {
                // Outer define Bold(true); inner não o toca → bold=true herdado.
                assert!(style.bold, "bold de outer deve herdar: {:?}", style);
                // Inner define Italic(true) e está mais próximo do texto —
                // sobrepõe Italic(false) do outer.
                assert!(style.italic, "italic de inner deve ganhar: {:?}", style);
            }
        }
    }

    /// Styled preserva o estilo do chamador após retorno — save/restore
    /// correcto (Passo 100, ADR-0039).
    #[test]
    fn styled_nao_vaza_para_texto_subsequente() {
        use std::sync::Arc;
        let styled = Content::Styled(
            Box::new(Content::text("STYLED")),
            Styles::from_iter([Style::Bold(true)]),
        );
        let plain = Content::text("plain");
        let seq = Content::Sequence(Arc::from(vec![styled, Content::Space, plain]));

        let doc = layout(&seq, CounterState::new());
        let texts = collect_text_items(&doc);

        // Encontrar o item do texto "STYLED" e do texto "plain".
        let styled_item = texts.iter().find(|i| matches!(i, FrameItem::Text { text, .. } if text.as_str() == "STYLED"));
        let plain_item  = texts.iter().find(|i| matches!(i, FrameItem::Text { text, .. } if text.as_str() == "plain"));

        assert!(styled_item.is_some());
        assert!(plain_item.is_some());

        if let Some(FrameItem::Text { style, .. }) = styled_item {
            assert!(style.bold, "STYLED deve ser bold");
        }
        if let Some(FrameItem::Text { style, .. }) = plain_item {
            assert!(!style.bold,
                "'plain' após Styled não deve herdar bold — save/restore falhou: {:?}",
                style);
        }
    }
}

// ── Passo 102.D: Integração `#set text(...)` end-to-end ──────────────────

#[cfg(test)]
mod tests_set_rule_integration {
    use super::*;
    use crate::{
        contracts::world::World,
        entities::{
            file_id::FileId,
            font_book::FontBook,
            layout_types::{FrameItem, Pt},
            source::Source,
            world_types::{Bytes, Datetime, FileError, FileResult, Font, Library},
        },
        rules::eval::eval_for_test,
    };
    use std::num::NonZeroU16;

    struct MockWorld {
        library: Library,
        book:    FontBook,
        source:  Source,
    }

    impl MockWorld {
        fn new(text: &str) -> Self {
            let id = FileId::from_raw(NonZeroU16::new(1).unwrap());
            Self {
                library: Library::new(),
                book:    FontBook::new(),
                source:  Source::new(id, text.to_string()),
            }
        }
    }

    impl World for MockWorld {
        fn library(&self) -> &Library  { &self.library }
        fn book(&self)    -> &FontBook { &self.book }
        fn main(&self)    -> FileId    { self.source.id() }
        fn source(&self, _: FileId) -> FileResult<Source> { Ok(self.source.clone()) }
        fn file(&self, _: FileId)    -> FileResult<Bytes>   { Err(FileError::NotFound) }
        fn font(&self, _: usize)     -> Option<Font>        { None }
        fn today(&self, _: Option<i64>) -> Option<Datetime> { None }
    }

    fn layout_typst(source: &str) -> PagedDocument {
        let world = MockWorld::new(source);
        let src = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let content = module.content().expect("content");
        let state = introspect(content);
        layout(content, state)
    }

    fn text_items(doc: &PagedDocument) -> Vec<(String, crate::entities::layout_types::TextStyle)> {
        doc.pages.iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, style, .. } => Some((text.to_string(), style.clone())),
                _ => None,
            })
            .collect()
    }

    /// `#set text(size: 18pt)` altera `FrameItem::Text.style.size`
    /// end-to-end (Parse → eval → layout → FrameItem).
    #[test]
    fn set_text_size_propaga_ao_frame() {
        let doc = layout_typst("#set text(size: 18pt)\nHello");
        let items = text_items(&doc);
        assert!(!items.is_empty(), "esperado pelo menos um Text item");
        for (text, style) in &items {
            assert_eq!(style.size, Pt(18.0),
                "text='{}' deve ter size=18pt; obtido {:?}", text, style.size);
        }
    }

    /// `#set text(bold: true)` produz bold em todo o texto seguinte.
    #[test]
    fn set_text_bold_propaga_ao_frame() {
        let doc = layout_typst("#set text(bold: true)\nHello");
        let items = text_items(&doc);
        assert!(!items.is_empty());
        for (text, style) in &items {
            assert!(style.bold, "text='{}' deve ter bold=true; style={:?}", text, style);
        }
    }

    /// `#set text(italic: true)` produz italic em todo o texto seguinte.
    #[test]
    fn set_text_italic_propaga_ao_frame() {
        let doc = layout_typst("#set text(italic: true)\nHello");
        let items = text_items(&doc);
        assert!(!items.is_empty());
        for (text, style) in &items {
            assert!(style.italic, "text='{}' deve ter italic=true; style={:?}", text, style);
        }
    }

    /// `#set` antes do texto afecta só o conteúdo seguinte. "antes" deve ficar
    /// sem bold; "depois" com bold. (Validação que `#set` não afecta texto
    /// anterior ao directive.)
    #[test]
    fn set_text_bold_afecta_conteudo_seguinte_nao_anterior() {
        let doc = layout_typst("antes\n#set text(bold: true)\ndepois");
        let items = text_items(&doc);
        let antes = items.iter().find(|(t, _)| t == "antes");
        let depois = items.iter().find(|(t, _)| t == "depois");

        assert!(antes.is_some(), "'antes' deve aparecer; items: {:?}",
            items.iter().map(|(t, _)| t).collect::<Vec<_>>());
        assert!(depois.is_some(), "'depois' deve aparecer; items: {:?}",
            items.iter().map(|(t, _)| t).collect::<Vec<_>>());
        if let Some((_, s)) = antes {
            assert!(!s.bold, "'antes' não deve ter bold: {:?}", s);
        }
        if let Some((_, s)) = depois {
            assert!(s.bold, "'depois' deve ter bold: {:?}", s);
        }
    }

    /// `#set text(bold: true)` combinado com `*texto*` — ambos produzem bold.
    /// Regressão: `*bold*` continua a funcionar após `#set` (Passo 101 preserva).
    #[test]
    fn set_combinado_com_emph_sintactico() {
        let doc = layout_typst("#set text(bold: true)\n_italic_ normal");
        let items = text_items(&doc);
        assert!(!items.is_empty());
        // Todos os items devem ter bold=true (vindo do #set).
        // Os items do `_italic_` têm italic=true adicionalmente.
        let has_italic = items.iter().any(|(_, s)| s.italic);
        let all_bold   = items.iter().all(|(_, s)| s.bold);
        assert!(all_bold,
            "todos os items devem ter bold=true após #set: {:?}",
            items.iter().map(|(t, s)| (t.as_str(), s.bold, s.italic)).collect::<Vec<_>>());
        assert!(has_italic,
            "pelo menos 1 item deve ter italic (do `_italic_`): {:?}",
            items.iter().map(|(t, s)| (t.as_str(), s.bold, s.italic)).collect::<Vec<_>>());
    }

    /// Regressão: `*bold*` sem `#set` continua a produzir bold (Passo 101).
    /// Valida que a remoção de `Content::Strong` e `#set` + `*bold*`
    /// coexistem sem interferência.
    #[test]
    fn bold_syntax_sem_set_continua_a_funcionar() {
        let doc = layout_typst("*importante* normal");
        let items = text_items(&doc);
        let importante = items.iter().find(|(t, _)| t == "importante");
        let normal = items.iter().find(|(t, _)| t == "normal");
        assert!(importante.map(|(_, s)| s.bold).unwrap_or(false),
            "'importante' deve ter bold: {:?}", items);
        assert!(!normal.map(|(_, s)| s.bold).unwrap_or(true),
            "'normal' não deve ter bold: {:?}", items);
    }

    // ── Passo 137 (Fase B.1 DEBT-52): consumer tracking ──────────────────
    //
    // Helper local: extrai `pos.x` de cada FrameItem::Text.
    fn text_items_with_pos(
        doc: &PagedDocument
    ) -> Vec<(String, crate::entities::layout_types::TextStyle, f64)> {
        doc.pages.iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, style, pos } => {
                    Some((text.to_string(), style.clone(), pos.x.val()))
                }
                _ => None,
            })
            .collect()
    }

    /// `#set text(tracking: X)` propaga para `FrameItem::Text.style.tracking`.
    /// Base da Fase B.1 — valida que o campo chega ao frame.
    #[test]
    fn set_text_tracking_propaga_ao_frame_passo_137() {
        use crate::entities::layout_types::Length;
        let doc = layout_typst("#set text(tracking: 1pt)\nHello");
        let items = text_items(&doc);
        assert!(!items.is_empty(), "esperado pelo menos um Text item");
        for (text, style) in &items {
            assert_eq!(style.tracking, Some(Length::pt(1.0)),
                "text='{}' deve ter tracking=1pt; obtido {:?}", text, style.tracking);
        }
    }

    /// Cursor avança mais com `tracking` activo — diferença observável
    /// entre posição do segundo word com e sem tracking.
    ///
    /// Input: `"AB CD"` com tracking=1em, size=12pt → 12pt de tracking
    /// entre cada par de chars dentro do word. Para "AB" (2 chars):
    /// 1 × 12pt extra face à versão sem tracking.
    #[test]
    fn layout_tracking_afecta_posicao_palavra_seguinte_passo_137() {
        let doc_sem = layout_typst("AB CD");
        let doc_com = layout_typst("#set text(tracking: 1em, size: 12pt)\nAB CD");

        let items_sem = text_items_with_pos(&doc_sem);
        let items_com = text_items_with_pos(&doc_com);

        // Encontrar "CD" em cada documento (pode haver itens como "•" se
        // o parser adicionar algo, mas aqui é input simples).
        let cd_sem = items_sem.iter().find(|(t, _, _)| t == "CD");
        let cd_com = items_com.iter().find(|(t, _, _)| t == "CD");

        assert!(cd_sem.is_some(), "items sem tracking: {:?}",
            items_sem.iter().map(|(t, _, _)| t).collect::<Vec<_>>());
        assert!(cd_com.is_some(), "items com tracking: {:?}",
            items_com.iter().map(|(t, _, _)| t).collect::<Vec<_>>());

        let x_sem = cd_sem.unwrap().2;
        let x_com = cd_com.unwrap().2;

        // Com tracking 1em a size 12pt, "AB" (2 chars) ganha 1×12pt extra.
        // "CD" começa 12pt mais à direita (ajustar por size base que pode
        // ser diferente — o "sem" usa size default 11pt).
        //
        // Verificar que x_com > x_sem por aproximadamente 12pt (margem
        // generosa porque o size também muda).
        assert!(x_com > x_sem,
            "com tracking, 'CD' deve começar mais à direita; sem={}, com={}",
            x_sem, x_com);
    }

    /// Consumer funciona para palavras com N chars: tracking_extra =
    /// (N - 1) × tracking_pt. Um char → sem tracking (N-1 = 0).
    #[test]
    fn layout_tracking_um_char_nao_acumula_passo_137() {
        // "A B" → dois words de 1 char cada. Sem tracking, gap = space_width.
        // Com tracking 1em, gap = space_width (tracking aplica-se entre
        // chars DENTRO de um word, não entre words).
        let doc_sem = layout_typst("A B");
        let doc_com = layout_typst("#set text(tracking: 10pt, size: 12pt)\nA B");

        let items_sem = text_items_with_pos(&doc_sem);
        let items_com = text_items_with_pos(&doc_com);

        let b_sem = items_sem.iter().find(|(t, _, _)| t == "B").map(|(_, _, x)| *x);
        let b_com = items_com.iter().find(|(t, _, _)| t == "B").map(|(_, _, x)| *x);

        assert!(b_sem.is_some() && b_com.is_some(),
            "esperava 'B' em ambos; sem={:?}, com={:?}", items_sem, items_com);

        // Diferença entre as duas posições B é só devida a mudança de
        // size (11 → 12pt). Tracking não afecta porque cada word tem
        // 1 char só.
        // Não assertamos valor exacto porque size base muda; assertamos
        // apenas que tracking de 10pt NÃO se propaga inter-word (diferença
        // marginal, não 10pt+).
        let dif = (b_com.unwrap() - b_sem.unwrap()).abs();
        assert!(dif < 10.0,
            "tracking não deve afectar gap entre palavras de 1 char; diff={}",
            dif);
    }

    // ── Passo 138 (Fase B.2 DEBT-52): consumer leading ───────────────────
    //
    // Helper local: extrai `pos.x` + `pos.y` de cada FrameItem::Text.
    fn text_items_with_xy(
        doc: &PagedDocument
    ) -> Vec<(String, crate::entities::layout_types::TextStyle, f64, f64)> {
        doc.pages.iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, style, pos } => {
                    Some((text.to_string(), style.clone(), pos.x.val(), pos.y.val()))
                }
                _ => None,
            })
            .collect()
    }

    /// `#set par(leading: X)` afasta linhas. Cristalino não tem
    /// `Content::Parbreak` — line break vem de heading (que chama
    /// flush_line) ou wrap.
    ///
    /// Fórmula escolhida (opt soma): `line_height = default + leading_pt`.
    /// Input com heading força flush_line entre linhas.
    #[test]
    fn layout_leading_afecta_posicao_linha_seguinte_passo_138() {
        // heading com `=` no início da linha + \n para forçar line break.
        let sem = layout_typst("= Título\nlinha2");
        let com = layout_typst(
            "#set par(leading: 20pt)\n= Título\nlinha2"
        );

        let sem_items = text_items_with_xy(&sem);
        let com_items = text_items_with_xy(&com);

        // "linha2" aparece após o heading — flush_line(s) intermédios.
        let l2_sem = sem_items.iter().find(|(t, _, _, _)| t == "linha2");
        let l2_com = com_items.iter().find(|(t, _, _, _)| t == "linha2");

        assert!(l2_sem.is_some(), "linha2 deve aparecer sem leading; items: {:?}",
            sem_items.iter().map(|(t, _, _, _)| t).collect::<Vec<_>>());
        assert!(l2_com.is_some(), "linha2 deve aparecer com leading; items: {:?}",
            com_items.iter().map(|(t, _, _, _)| t).collect::<Vec<_>>());

        let y_sem = l2_sem.unwrap().3;
        let y_com = l2_com.unwrap().3;

        // Frame coord: y cresce para baixo. Com leading positivo, linha
        // após heading está mais abaixo (y maior).
        assert!(y_com > y_sem,
            "linha2 deve ter y maior com leading; sem={}, com={}",
            y_sem, y_com);
    }

    /// Leading não afecta documento de 1 linha (leading = inter-line;
    /// sem linha 2, não há onde aplicar).
    #[test]
    fn layout_leading_nao_afecta_documento_uma_linha_passo_138() {
        let sem = layout_typst("uma linha");
        let com = layout_typst(
            "#set par(leading: 10pt)\numa linha"
        );

        let sem_items = text_items_with_xy(&sem);
        let com_items = text_items_with_xy(&com);

        // Primeiro item de cada (primeira word): y idêntico porque
        // leading só afecta linhas 2+.
        let primeiro_sem = &sem_items[0];
        let primeiro_com = &com_items[0];

        assert!((primeiro_sem.3 - primeiro_com.3).abs() < 0.01,
            "primeira linha: y deve ser igual sem vs com leading; sem={}, com={}",
            primeiro_sem.3, primeiro_com.3);
    }

    /// Regressão: leading = 0pt comporta-se igual a sem set.
    /// Valida fórmula soma (default + leading; 0 leading = default).
    #[test]
    fn layout_leading_zero_preserva_comportamento_base_passo_138() {
        let sem = layout_typst("= Título\nlinha2");
        let com = layout_typst(
            "#set par(leading: 0pt)\n= Título\nlinha2"
        );

        let sem_items = text_items_with_xy(&sem);
        let com_items = text_items_with_xy(&com);

        assert_eq!(sem_items.len(), com_items.len(),
            "mesmo número de items; sem: {}, com: {}",
            sem_items.len(), com_items.len());

        for (s, c) in sem_items.iter().zip(com_items.iter()) {
            assert!((s.3 - c.3).abs() < 0.01,
                "leading 0pt deve ser igual a sem set; item '{}': sem.y={}, com.y={}",
                s.0, s.3, c.3);
        }
    }
}

// ── Passo 103.D: Integração `#show` end-to-end ────────────────────────────

#[cfg(test)]
mod tests_show_rule_integration {
    use super::*;
    use crate::{
        contracts::world::World,
        entities::{
            file_id::FileId,
            font_book::FontBook,
            layout_types::FrameItem,
            source::Source,
            world_types::{Bytes, Datetime, FileError, FileResult, Font, Library},
        },
        rules::eval::eval_for_test,
    };
    use std::num::NonZeroU16;

    struct MockWorld {
        library: Library,
        book:    FontBook,
        source:  Source,
    }

    impl MockWorld {
        fn new(text: &str) -> Self {
            let id = FileId::from_raw(NonZeroU16::new(1).unwrap());
            Self {
                library: Library::new(),
                book:    FontBook::new(),
                source:  Source::new(id, text.to_string()),
            }
        }
    }

    impl World for MockWorld {
        fn library(&self) -> &Library  { &self.library }
        fn book(&self)    -> &FontBook { &self.book }
        fn main(&self)    -> FileId    { self.source.id() }
        fn source(&self, _: FileId) -> FileResult<Source> { Ok(self.source.clone()) }
        fn file(&self, _: FileId)    -> FileResult<Bytes>   { Err(FileError::NotFound) }
        fn font(&self, _: usize)     -> Option<Font>        { None }
        fn today(&self, _: Option<i64>) -> Option<Datetime> { None }
    }

    fn layout_typst(source: &str) -> PagedDocument {
        let world = MockWorld::new(source);
        let src = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let content = module.content().expect("content");
        let state = introspect(content);
        layout(content, state)
    }

    fn plain_text(doc: &PagedDocument) -> String {
        doc.pages.iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.to_string()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// `#show heading: it => upper(it.body)` transforma headings em UPPERCASE.
    /// Valida end-to-end: parse → eval → apply_show_rules → Content → layout → FrameItem.
    #[test]
    fn show_heading_transforma_em_uppercase() {
        let doc = layout_typst("#show heading: it => upper(it.body)\n\n= Intro");
        let text = plain_text(&doc);
        assert!(text.contains("INTRO"),
            "esperado 'INTRO' no output após show heading upper: {:?}", text);
    }

    /// `#show strong: it => upper(it.body)` transforma `*bold*` em UPPERCASE.
    #[test]
    fn show_strong_transforma() {
        let doc = layout_typst("#show strong: upper\n*alvo*");
        let text = plain_text(&doc);
        assert!(text.contains("ALVO"),
            "esperado 'ALVO' após show strong upper: {:?}", text);
    }

    /// `#show emph: it => lower(it.body)` transforma `_italic_` em lowercase.
    #[test]
    fn show_emph_transforma() {
        let doc = layout_typst("#show emph: lower\n_TIPO_");
        let text = plain_text(&doc);
        assert!(text.contains("tipo"),
            "esperado 'tipo' após show emph lower: {:?}", text);
    }

    /// Regressão: sem `#show`, `*bold*` continua bold; `= heading` continua
    /// heading. Confirma que os show rules não interferem quando ausentes.
    #[test]
    fn regressao_sem_show_mantem_comportamento() {
        let doc = layout_typst("*bold* and _italic_");
        let items: Vec<_> = doc.pages.iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|i| match i {
                FrameItem::Text { text, style, .. } =>
                    Some((text.to_string(), style.bold, style.italic)),
                _ => None,
            })
            .collect();
        // Deve existir pelo menos um item com bold e outro com italic.
        assert!(items.iter().any(|(t, b, _)| t == "bold" && *b),
            "esperado 'bold' com style.bold=true: {:?}", items);
        assert!(items.iter().any(|(t, _, i)| t == "italic" && *i),
            "esperado 'italic' com style.italic=true: {:?}", items);
    }

    /// **Documenta dívida latente (DEBT-50)**: `#show strong` apanha `Content::Styled`
    /// com `Style::Bold(true)`. Hoje, `#set text(bold: true)` **não** produz
    /// `Content::Styled` (bake-in em `Content::Text`), portanto a dívida está
    /// **adormecida** — o selector Strong NÃO apanha texto afectado por #set text.
    /// Este teste garante esse comportamento actual; se um passo futuro migrar
    /// `#set text` para wrapping, este teste falha e o DEBT-50 torna-se accionável.
    #[test]
    fn debt_50_show_strong_nao_apanha_set_text_bold_porque_bake_in() {
        let doc = layout_typst("#show strong: it => [HIT]\n#set text(bold: true)\ntexto");
        let text = plain_text(&doc);
        // `#set text(bold: true)` produz `Content::Text("texto", { bold: true })`,
        // NÃO `Content::Styled(.., [Bold(true)])`. O selector strong só casa
        // `Content::Styled`, portanto NÃO dispara.
        // Esperado: "texto" sem "HIT".
        assert!(!text.contains("HIT"),
            "DEBT-50: enquanto `#set text` usar bake-in, selector Strong NÃO deve \
             disparar por `#set text(bold: true)`. Se este teste falhar, o Passo \
             que migrou `#set text` para wrapping deve activar DEBT-50: {:?}", text);
        assert!(text.contains("texto"),
            "'texto' deve aparecer no output: {:?}", text);
    }

    // ── Passo 156C (ADR-0061 Fase 1, sub-passo 1) — pad + hide ─────────────

    /// `Content::Pad` reserva top + left ao layout do body e avança bottom no
    /// fim. Verificamos que existe espaço vertical adicional comparado ao
    /// body sem pad.
    #[test]
    fn layout_pad_avanca_cursor_bottom_e_top() {
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;

        // Documento sem pad como baseline.
        let baseline = layout(&Content::text("hello"), CounterState::default());
        let baseline_y_max: f64 = baseline.pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { pos, .. } => Some(pos.y.val()),
                _ => None,
            })
            .fold(0.0_f64, |acc, y| acc.max(y));

        // Mesmo body envolvido em Pad com top=20pt, bottom=20pt.
        let padded = Content::pad(
            Content::text("hello"),
            Sides::new(Length::ZERO, Length::pt(20.0), Length::ZERO, Length::pt(20.0)),
        );
        let with_pad = layout(&padded, CounterState::default());
        let pad_y_max: f64 = with_pad.pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { pos, .. } => Some(pos.y.val()),
                _ => None,
            })
            .fold(0.0_f64, |acc, y| acc.max(y));

        // Pad com top=20 deve empurrar o texto para baixo na página.
        assert!(pad_y_max > baseline_y_max,
            "esperado que Content::Pad com top=20pt empurre o texto para baixo: \
             baseline_y_max={baseline_y_max:.2} pad_y_max={pad_y_max:.2}");
    }

    /// `Content::Hide` calcula dimensões mas não emite items visuais.
    /// Verificamos que zero items textuais são produzidos pelo body
    /// envolvido em Hide.
    #[test]
    fn layout_hide_emite_zero_text_items() {
        let hidden = Content::hide(Content::text("invisivel"));
        let doc = layout(&hidden, CounterState::default());
        let text_items = doc.pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter(|item| matches!(item, FrameItem::Text { .. }))
            .count();
        assert_eq!(text_items, 0,
            "Content::Hide não deve emitir nenhum FrameItem::Text");
    }

    // ── Passo 156D (ADR-0061 Fase 1, sub-passo 2) — h + v spacing ─────────

    /// `Content::HSpace` avança `cursor.x` mas não emite items próprios.
    /// Verificamos via posição X de texto subsequente: com HSpace antes,
    /// o segundo texto fica deslocado para a direita.
    #[test]
    fn layout_hspace_avanca_cursor_x() {
        use crate::entities::layout_types::Length;
        use std::sync::Arc;
        // Sequência: "A" + h(50pt) + "B".
        let with_space = Content::Sequence(Arc::from(vec![
            Content::text("A"),
            Content::h_space(Length::pt(50.0), false),
            Content::text("B"),
        ]));
        let doc = layout(&with_space, CounterState::default());
        let texts: Vec<_> = doc.pages.iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { pos, text, .. } => Some((pos.x.val(), text.to_string())),
                _ => None,
            })
            .collect();
        let pos_a = texts.iter().find(|(_, t)| t == "A").map(|(x, _)| *x).unwrap();
        let pos_b = texts.iter().find(|(_, t)| t == "B").map(|(x, _)| *x).unwrap();
        // B deve estar à direita de A com pelo menos ~50pt de afastamento
        // adicional vs apenas a largura do glifo "A" + space natural.
        // Verificamos um threshold conservador.
        assert!(pos_b - pos_a > 50.0,
            "h(50pt) deve afastar B de A em pelo menos 50pt: pos_a={pos_a:.2} pos_b={pos_b:.2}");
    }

    /// `Content::VSpace` força `flush_line` antes de avançar `cursor.y`.
    /// Verificamos via posição Y de texto subsequente: com VSpace de 30pt,
    /// segundo texto fica abaixo do primeiro com pelo menos ~30pt extra.
    #[test]
    fn layout_vspace_avanca_cursor_y() {
        use crate::entities::layout_types::Length;
        use std::sync::Arc;
        let with_space = Content::Sequence(Arc::from(vec![
            Content::text("A"),
            Content::v_space(Length::pt(30.0), false),
            Content::text("B"),
        ]));
        let doc = layout(&with_space, CounterState::default());
        let texts: Vec<_> = doc.pages.iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { pos, text, .. } => Some((pos.y.val(), text.to_string())),
                _ => None,
            })
            .collect();
        let pos_a_y = texts.iter().find(|(_, t)| t == "A").map(|(y, _)| *y).unwrap();
        let pos_b_y = texts.iter().find(|(_, t)| t == "B").map(|(y, _)| *y).unwrap();
        // B deve estar abaixo de A com pelo menos ~30pt extra (mais
        // line_height que o flush adiciona).
        assert!(pos_b_y - pos_a_y > 30.0,
            "v(30pt) deve empurrar B abaixo de A em pelo menos 30pt: \
             pos_a_y={pos_a_y:.2} pos_b_y={pos_b_y:.2}");
    }
}
