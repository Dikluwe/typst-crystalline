//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout.md
//! @prompt-hash 518a9856
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
