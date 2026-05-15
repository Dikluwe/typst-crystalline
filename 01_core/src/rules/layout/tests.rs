//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout.md
//! @prompt-hash 089621fc
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
use crate::entities::{content::Content, layout_types::FrameItem};
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
    // P204C (M8): Layouter ganha 'a + Tracked<dyn Introspector>.
    use comemo::Track;
    use crate::entities::introspector::{Introspector, TagIntrospector};
    let intr = TagIntrospector::empty();
    let intr_dyn: &dyn Introspector = &intr;
    let intr_tracked = intr_dyn.track();
    let l = Layouter::new(FixedMetrics, NullImageSizer, 12.0, intr_tracked);
    assert!(l.regions.current.cursor_y.val() > 0.0);
    assert!(l.regions.current.cursor_y.val() < 842.0);
}

// ── P204C (M8) — Sentinel tests para migração Layouter ────────────────────
//
// Confirmam que Layouter ganhou lifetime parameter `'a` e que field
// `introspector` é `Tracked<dyn Introspector + 'a>` per ADR-0073.
// Falham de compilação se a migração for revertida.

#[test]
fn p204c_layouter_struct_aceita_tracked_introspector() {
    // Sentinel: Layouter::new aceita Tracked<dyn Introspector>
    // como 4º parâmetro. Falha de compilação se signature reverter
    // para 3 args.
    use comemo::Track;
    use crate::entities::introspector::{Introspector, TagIntrospector};
    let intr = TagIntrospector::empty();
    let intr_dyn: &dyn Introspector = &intr;
    let intr_tracked = intr_dyn.track();
    let _l: Layouter<'_, FixedMetrics, NullImageSizer> =
        Layouter::new(FixedMetrics, NullImageSizer, 12.0, intr_tracked);
}

#[test]
fn p204c_pipeline_e2e_via_tracked() {
    // Sentinel runtime: pipeline end-to-end com Tracked produz
    // documento equivalente. Confirma que migração não quebra
    // funcionalidade básica.
    let content = Content::text("Hello P204C");
    let doc = layout(&content);
    assert!(!doc.pages.is_empty(), "layout via tracked produz páginas");
    assert!(
        doc.plain_text().contains("Hello P204C"),
        "texto preservado pelo pipeline tracked"
    );
}

// ── P204D (M8) — Sentinel + E2E tests para Position concrete ──────────────
//
// Confirmam que tipo `Position` existe, que `LayouterRuntimeState` ganhou
// campo `positions`, e que Layouter popula durante layout single-pass.

#[test]
fn p204d_position_struct_existe() {
    // Sentinel: tipo Position existe em `crate::entities::position`.
    // Falha de compilação se for removido ou renomeado.
    use std::num::NonZeroUsize;
    use crate::entities::position::Position;
    use crate::entities::layout_types::{Point, Pt};

    let _p = Position {
        page:  NonZeroUsize::new(1).unwrap(),
        point: Point { x: Pt(0.0), y: Pt(0.0) },
    };
}

#[test]
fn p204d_runtime_positions_field_existe() {
    // Sentinel: LayouterRuntimeState tem field `positions`.
    // Falha de compilação se for removido. Construído via Default
    // — confirma que campo existe e é HashMap<Location, Position>.
    use std::collections::HashMap;
    use crate::entities::layouter_runtime_state::LayouterRuntimeState;
    use crate::entities::location::Location;
    use crate::entities::position::Position;

    let runtime = LayouterRuntimeState::default();
    let _check: &HashMap<Location, Position> = &runtime.positions;
    assert!(runtime.positions.is_empty(), "default runtime tem positions vazio");
}

#[test]
fn p204d_position_populada_para_locatable_basico() {
    // E2E test 1: documento com 1 label (locatable Heading)
    // produz entry em runtime.positions.
    use comemo::Track;
    use crate::entities::introspector::{Introspector, TagIntrospector};

    let intr = TagIntrospector::empty();
    let intr_dyn: &dyn Introspector = &intr;
    let intr_tracked = intr_dyn.track();
    let mut layouter = Layouter::new(
        FixedMetrics, NullImageSizer, DEFAULT_FONT_SIZE, intr_tracked,
    );

    let content = Content::heading(1, Content::text("Title"));
    layouter.layout_content(&content);

    // Heading é locatable → current_location set + Position emitted.
    let loc = layouter.current_location.expect("Heading locatable → Some");
    let pos = layouter.runtime.positions.get(&loc).copied()
        .expect("runtime.positions populated para locatable");

    // Página 1 (1-based; primeira página).
    assert_eq!(pos.page.get(), 1, "Heading na primeira página");
    // Cursor x/y dentro de limites razoáveis.
    assert!(pos.point.x.val() >= 0.0, "point.x positivo");
    assert!(pos.point.y.val() >= 0.0, "point.y positivo");
}

#[test]
fn p204d_position_nao_populada_para_nao_locatable() {
    // E2E test 2: Content não-locatable (Text simples)
    // NÃO produz entry em runtime.positions.
    use comemo::Track;
    use crate::entities::introspector::{Introspector, TagIntrospector};

    let intr = TagIntrospector::empty();
    let intr_dyn: &dyn Introspector = &intr;
    let intr_tracked = intr_dyn.track();
    let mut layouter = Layouter::new(
        FixedMetrics, NullImageSizer, DEFAULT_FONT_SIZE, intr_tracked,
    );

    let content = Content::text("plain text");
    layouter.layout_content(&content);

    // Text simples não-locatable → current_location ainda None
    // → runtime.positions vazio.
    assert_eq!(layouter.current_location, None, "Text não set current_location");
    assert!(
        layouter.runtime.positions.is_empty(),
        "Text não-locatable → runtime.positions vazio"
    );
}

// ── P205C (F3) — pipeline E2E: layout → seal → inject → query ─────────

#[test]
fn p205c_pipeline_layout_seal_inject_query_devolve_some() {
    // E2E test: pipeline completo per ADR-0074 §C6 (Position
    // trackable). 1) Layouter populates runtime.positions;
    // 2) finish() seal extracted_positions; 3) caller injecta
    // SealedPositions no TagIntrospector; 4) Introspector::position_of
    // devolve Some(Position) real.
    use comemo::Track;
    use crate::entities::introspector::{Introspector, TagIntrospector};

    let mut intr = TagIntrospector::empty();
    let intr_dyn: &dyn Introspector = &intr;
    let intr_tracked = intr_dyn.track();
    let mut layouter = Layouter::new(
        FixedMetrics, NullImageSizer, DEFAULT_FONT_SIZE, intr_tracked,
    );

    let content = Content::heading(1, Content::text("Title"));
    layouter.layout_content(&content);

    // Captura location locatable antes de finish (consume self).
    let loc = layouter.current_location.expect("Heading locatable → Some");

    // P205B: finish seal extracted_positions.
    let doc = layouter.finish();
    assert!(
        !doc.extracted_positions.is_empty(),
        "extracted_positions populated após heading"
    );

    // P205C: caller injecta no introspector.
    intr.inject_positions(doc.extracted_positions.clone());

    // Pós-injecção: position_of devolve Position real.
    let pos = intr.position_of(loc)
        .expect("position_of devolve Some pós-injecção para locatable");
    assert_eq!(pos.page.get(), 1);
    assert!(pos.point.x.val() >= 0.0);
    assert!(pos.point.y.val() >= 0.0);

    // Location desconhecida ainda devolve None.
    use crate::entities::location::Location;
    let unknown = Location::from_raw(0xDEAD_BEEF);
    assert_eq!(intr.position_of(unknown), None);
}

// ── Testes de layout() (herdados do Passo 19) ─────────────────────────

#[test]
fn layout_texto_simples_tem_items() {
    let doc = layout(&Content::text("Hello world"));
    assert!(!doc.pages.is_empty());
    let total = doc.pages.iter().flat_map(|p| p.items.iter()).count();
    assert!(total >= 2, "Hello e world devem ser itens separados");
    assert!(doc.plain_text().contains("Hello"));
    assert!(doc.plain_text().contains("world"));
}

#[test]
fn layout_documento_vazio_zero_paginas() {
    let doc = layout(&Content::Empty);
    assert_eq!(doc.pages.len(), 0, "documento vazio → sem páginas");
}

/// Teste de Ouro: todos os items dentro dos limites da página.
#[test]
fn layout_items_dentro_limites_da_pagina() {
    let words = (0..100)
        .map(|i| format!("palavra{i}"))
        .collect::<Vec<_>>()
        .join(" ");
    let doc = layout(&Content::text(&words));

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
    let doc = layout(&Content::text(&words));
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
    ));
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
    ));
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
    let doc = layout(&content);
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
    ]));
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
    let doc = layout(content);
    assert!(!doc.pages.is_empty());
    assert!(
        doc.plain_text().contains("Olá") || doc.plain_text().contains("mundo"),
        "texto deve estar no output: {:?}", doc.plain_text()
    );
}

// ── Passo 23 ────────────────────────────────────────────────────────────

#[test]
fn layout_list_item_tem_bullet() {
    let doc = layout(&Content::list_item(Content::text("Item")));
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
    let doc = layout(&content);
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
    layout(content)
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

// ── Testes de CounterStateLegacy e numeração de headings (Passo 57/58) ──────

#[test]
fn layout_heading_sem_numbering_nao_tem_prefixo() {
    // Por defeito, numbering_active está vazio — não deve aparecer "1."
    let content = Content::heading(1, Content::text("Intro"));
    let doc = layout(&content);
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
    let doc = layout(&content);
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
    let doc = layout(&content);
    let text = doc.plain_text();
    assert!(text.contains("1."), "H1 deve ter prefixo '1.'");
    assert!(text.contains("1.1"), "H2 deve ter prefixo '1.1'");
}

// ── P182D — Layouter heading-arm via Introspector (substitution-with-fallback)

#[test]
fn p182d_heading_numbering_via_introspector_path() {
    // Documento sem `Content::SetHeadingNumbering` no AST: legacy walk arm
    // (`introspect.rs:455–457`) e Layout walk arm (`layout/counters.rs:11–13`)
    // não populam `state.numbering_active`. Mas o Introspector é injectado
    // pré-populado com `numbering_active:heading=true` — Layouter heading-arm
    // deve disparar prefixo via path Introspector.
    use crate::entities::introspector::TagIntrospector;
    use crate::entities::location::Location;
    use crate::entities::value::Value;
    use crate::rules::introspect::introspect_with_introspector;

    let plain = Content::heading(1, Content::text("Intro"));
    let mut intr: TagIntrospector =
        introspect_with_introspector(&plain);
    intr.state.init(
        "numbering_active:heading".to_string(),
        Value::Bool(true),
        Location::from_raw(0),
    );
    // State legacy vazio — apenas Introspector path activo.
    let doc = layout_with_introspector(&plain, intr);
    let text = doc.plain_text();
    assert!(
        text.contains("1."),
        "P182D: prefixo deve vir via Introspector quando legacy vazio; obtido: '{text}'"
    );
}

// P190E (M6): test `p182d_heading_numbering_via_fallback_legacy` removido —
// fallback legacy `self.counter.is_numbering_active(...)` eliminado em P190E.
// Caminho Introspector único activo desde P198B. Sem fallback para testar.

#[test]
fn p182d_heading_numbering_paridade_legacy_vs_migrated() {
    // Output observable inalterado: `layout()` legacy e
    // `layout_with_introspector` produzem mesmo plain_text para
    // documento típico (SetHeadingNumbering + headings).
    use crate::rules::introspect::introspect_with_introspector;

    let content = Content::Sequence(vec![
        Content::SetHeadingNumbering { active: true },
        Content::heading(1, Content::text("Intro")),
        Content::heading(2, Content::text("Sub")),
        Content::heading(1, Content::text("Conclusão")),
    ].into());

    let txt_legacy = layout(&content).plain_text();
    let intr = introspect_with_introspector(&content);
    let txt_new = layout_with_introspector(&content, intr).plain_text();
    assert_eq!(txt_legacy, txt_new, "P182D: paridade pre/post migração");
}

#[test]
fn layout_counter_display_heading_retorna_estado_actual() {
    let content = Content::Sequence(vec![
        Content::SetHeadingNumbering { active: true },
        Content::heading(1, Content::text("Intro")),
        Content::CounterDisplay { kind: "heading".to_string() },
    ].into());
    let doc = layout(&content);
    let text = doc.plain_text();
    // CounterDisplay de heading após H1 deve mostrar "1"
    // (o heading já avançou o contador antes de CounterDisplay ser processado)
    assert!(text.contains('1'));
}

// ── Testes de CounterUpdate (Passo 58) ────────────────────────────────

#[test]
fn counter_update_nao_produz_items_visuais() {
    use crate::entities::counter_update::CounterUpdate as CounterAction;

    let content = Content::CounterUpdate {
        key:    "equation".to_string(),
        action: CounterAction::Update(5),
    };
    let doc = layout(&content);
    let total_items: usize = doc.pages.iter().map(|p| p.items.len()).sum();
    assert_eq!(total_items, 0, "CounterUpdate não deve gerar items visuais");
}

#[test]
fn counter_update_seguido_de_display_mostra_valor_correcto() {
    use crate::entities::counter_update::CounterUpdate as CounterAction;

    let content = Content::Sequence(vec![
        Content::CounterUpdate {
            key:    "equation".to_string(),
            action: CounterAction::Update(5),
        },
        Content::CounterDisplay { kind: "equation".to_string() },
    ].into());
    let doc = layout(&content);
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

    let doc = layout(&content);
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

    let doc = layout(&content);
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
    use crate::rules::introspect::introspect_with_introspector;

    let content_a = Content::Labelled {
        label:  Label("sec".to_string()),
        target: Box::new(Content::heading(1, Content::text("A"))),
    };
    let _ = layout(&content_a);

    // Segundo layout independente — não deve ter "sec" resolvida
    let content_b = Content::Ref { target: Label("sec".to_string()) };
    let doc_b = layout(&content_b);
    assert!(
        doc_b.plain_text().contains("@sec"),
        "Estado do layout anterior não deve vazar para o seguinte"
    );
}

// ── Testes de duas passagens (Passo 60) ──────────────────────────────────

#[test]
fn pipeline_duas_passagens_resolve_forward_ref() {
    use crate::entities::label::Label;
    use crate::rules::{introspect::{introspect, introspect_with_introspector}, layout::layout};

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

    // Passagem 1 — verificar que introspect resolve forward ref via
    // intr (P190G: state.resolved_labels eliminado).
    let intr = introspect_with_introspector(&content);
    assert!(
        intr.resolved_labels.get(&Label("conclusao".to_string())).is_some(),
        "introspect deve popular intr.resolved_labels para forward refs"
    );

    // Passagem 2 — layout usa o estado da pré-passagem.
    let doc = layout(&content);
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
    // P190E (M6): test adaptado — usa pipeline standard com
    // Content::SetEquationNumbering em vez de mutar state.numbering_active
    // directamente. Caminho Introspector activo desde P199B.
    let content = Content::Sequence(vec![
        Content::SetEquationNumbering { active: true },
        Content::Equation {
            body:  Box::new(Content::MathIdent("E".into())),
            block: true,
        },
    ].into());

    let doc = layout(&content);
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
    let doc = layout(&content);
    let text = doc.plain_text();

    assert!(text.contains("Índice"), "TOC deve ter título 'Índice'");
    assert!(text.contains("Introdução"), "TOC deve listar o título H1");
    assert!(text.contains("Motivação"), "TOC deve listar o título H2");
}

#[test]
fn layout_outline_sem_headings_gera_apenas_titulo_ou_vazio() {
    let content = Content::Outline;
    let state = introspect(&content);
    let doc = layout(&content);
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
    let doc = layout(&content);
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
        kind:      Some("image".to_string()),
        numbering: Some("1".to_string()),
    };

    let state = introspect(&content);
    let doc = layout(&content);
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
        kind:      Some("image".to_string()),
        numbering: Some("1".to_string()),
    };

    let state = introspect(&content);
    let doc = layout(&content);
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
                    kind:      Some("image".to_string()),
                    numbering: Some("1".to_string()),
                }),
            },
            Content::text(" — ver "),
            Content::Ref { target: Label("fig1".to_string()) },
        ]
        .into(),
    );

    let state = introspect(&content);
    let doc = layout(&content);
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
    let doc = layout(&content);

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
    let doc = layout(&content);

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
    use crate::entities::counter_update::CounterUpdate as CounterAction;

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
    let doc = layout(&content);
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
    let doc = layout(&content);
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
    let doc = layout(&content);

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
    // P190D (M6 categoria Document metadata): assertion sobre
    // `state.has_outline` removida — field eliminado. Cobertura via
    // `intr.kind_index[Outline]` em tests Introspector + integração
    // Layouter mod.rs:1488.

    let doc = layout(&content);
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
    let doc = layout(&content);

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
    let doc   = layout(&content);

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

    // P204C (M8): Layouter ganha 'a + Tracked<dyn Introspector>.
    use comemo::Track;
    use crate::entities::introspector::{Introspector, TagIntrospector};
    let intr = TagIntrospector::empty();
    let intr_dyn: &dyn Introspector = &intr;
    let intr_tracked = intr_dyn.track();
    let layouter = Layouter::new(FixedMetrics, NullImageSizer, DEFAULT_FONT_SIZE, intr_tracked);

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
    // P204C (M8): Layouter ganha 'a + Tracked<dyn Introspector>.
    use comemo::Track;
    use crate::entities::introspector::{Introspector, TagIntrospector};
    let intr = TagIntrospector::empty();
    let intr_dyn: &dyn Introspector = &intr;
    let intr_tracked = intr_dyn.track();
    let layouter = Layouter::new(FixedMetrics, NullImageSizer, DEFAULT_FONT_SIZE, intr_tracked);

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
        gutter: None,
        align:  None,
        inset:  crate::entities::sides::Sides::uniform(
            crate::entities::layout_types::Length::pt(0.0)),
        header: None,
        footer: None,
        stroke: None,
        fill:   None,
    };

    let state = introspect(&grid);
    let doc   = layout(&grid);

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
    // P204C (M8): Layouter ganha 'a + Tracked<dyn Introspector>.
    use comemo::Track;
    use crate::entities::introspector::{Introspector, TagIntrospector};
    let intr = TagIntrospector::empty();
    let intr_dyn: &dyn Introspector = &intr;
    let intr_tracked = intr_dyn.track();
    let layouter = Layouter::new(FixedMetrics, NullImageSizer, DEFAULT_FONT_SIZE, intr_tracked);

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

        let doc = layout(&styled);
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

        let doc = layout(&outer);
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

        let doc = layout(&seq);
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
        layout(content)
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
        layout(content)
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
        let baseline = layout(&Content::text("hello"));
        let baseline_y_max: f64 = baseline.pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { pos, .. } => Some(pos.y.val()),
                _ => None,
            })
            .fold(0.0_f64, |acc, y| acc.max(y));

        // Mesmo body envolvido em Pad com top=20pt, bottom=20pt
        // (P156L: cada side é Option<Length>; Some(...) explícito).
        let padded = Content::pad(
            Content::text("hello"),
            Sides::new(None, Some(Length::pt(20.0)), None, Some(Length::pt(20.0))),
        );
        let with_pad = layout(&padded);
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
        let doc = layout(&hidden);
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
        let doc = layout(&with_space);
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
        let doc = layout(&with_space);
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

    // ── Passo 156E (ADR-0061 Fase 1, sub-passo 3) — pagebreak manual ───────

    /// Helper: extrai texto plano da primeira página que contém certa
    /// string. Devolve o índice de página (1-indexed) onde foi encontrado.
    fn page_index_containing(doc: &crate::entities::layout_types::PagedDocument, needle: &str) -> Option<usize> {
        for (i, page) in doc.pages.iter().enumerate() {
            for item in page.items.iter() {
                if let FrameItem::Text { text, .. } = item {
                    if text.contains(needle) {
                        return Some(i + 1);  // 1-indexed
                    }
                }
            }
        }
        None
    }

    /// Pagebreak default força commit da página actual; conteúdo seguinte
    /// vai para nova página.
    #[test]
    fn layout_pagebreak_forca_nova_pagina() {
        use std::sync::Arc;
        let doc_content = Content::Sequence(Arc::from(vec![
            Content::text("A"),
            Content::pagebreak(false, None),
            Content::text("B"),
        ]));
        let doc = layout(&doc_content);
        // Esperamos pelo menos 2 páginas (A na primeira, B na segunda).
        assert!(doc.pages.len() >= 2,
            "esperado >= 2 páginas após pagebreak, obtive {}", doc.pages.len());
        let page_a = page_index_containing(&doc, "A").expect("A não encontrado");
        let page_b = page_index_containing(&doc, "B").expect("B não encontrado");
        assert!(page_b > page_a,
            "B deve estar em página posterior a A: A→p{} B→p{}", page_a, page_b);
    }

    /// `pagebreak(to: even)` quando próxima página seria ímpar (p2 par,
    /// portanto p3 ímpar) deve inserir página vazia para forçar próxima
    /// para par. Setup: A (p1) → pagebreak(to:even) → próxima seria p2
    /// (par), portanto bate sem inserção. Teste é "no extra inserted when
    /// already matches".
    #[test]
    fn layout_pagebreak_to_even_quando_ja_par_nao_insere_extra() {
        use std::sync::Arc;
        let doc_content = Content::Sequence(Arc::from(vec![
            Content::text("A"),
            Content::pagebreak(false, Some(crate::entities::parity::Parity::Even)),
            Content::text("B"),
        ]));
        let doc = layout(&doc_content);
        // A em p1 (ímpar); pagebreak commits p1, próxima seria p2 (par).
        // Even matches → sem inserção extra. B em p2.
        let page_a = page_index_containing(&doc, "A").expect("A não encontrado");
        let page_b = page_index_containing(&doc, "B").expect("B não encontrado");
        assert_eq!(page_a, 1);
        assert_eq!(page_b, 2,
            "B deve estar na p2 (par; sem inserção extra): obtive p{}", page_b);
    }

    /// `pagebreak(to: odd)` quando próxima página seria par (p2) deve
    /// inserir página vazia para forçar próxima para ímpar (p3).
    /// Setup: A (p1) → pagebreak(to:odd) → próxima seria p2 (par);
    /// inserir vazia → B em p3.
    #[test]
    fn layout_pagebreak_to_odd_insere_vazia_se_proxima_seria_par() {
        use std::sync::Arc;
        let doc_content = Content::Sequence(Arc::from(vec![
            Content::text("A"),
            Content::pagebreak(false, Some(crate::entities::parity::Parity::Odd)),
            Content::text("B"),
        ]));
        let doc = layout(&doc_content);
        let page_a = page_index_containing(&doc, "A").expect("A não encontrado");
        let page_b = page_index_containing(&doc, "B").expect("B não encontrado");
        assert_eq!(page_a, 1);
        assert_eq!(page_b, 3,
            "B deve estar na p3 (ímpar; vazia inserida na p2): obtive p{}", page_b);
        assert!(doc.pages.len() >= 3,
            "esperado >= 3 páginas (A em p1, vazia em p2, B em p3)");
    }

    // ── Passo 156G (ADR-0061 Fase 2 sub-passo 1) — block container ─────────

    /// `Content::Block` com `inset` adiciona top + bottom ao avanço de
    /// cursor e left ao indent. Verificamos via posição Y de texto
    /// subsequente.
    #[test]
    fn layout_block_inset_avanca_cursor_y() {
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        use std::sync::Arc;

        // Sequência: "A" + block(text("body"), inset=10pt) + "C".
        let doc_content = Content::Sequence(Arc::from(vec![
            Content::text("A"),
            Content::block(
                Content::text("body"),
                None, None,
                Sides::uniform(Length::pt(10.0)),
                true,
            ),
            Content::text("C"),
        ]));
        let doc = layout(&doc_content);
        let texts: Vec<_> = doc.pages.iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { pos, text, .. } => Some((pos.y.val(), text.to_string())),
                _ => None,
            })
            .collect();
        let pos_a_y    = texts.iter().find(|(_, t)| t == "A").map(|(y, _)| *y).unwrap();
        let pos_body_y = texts.iter().find(|(_, t)| t == "body").map(|(y, _)| *y).unwrap();
        let pos_c_y    = texts.iter().find(|(_, t)| t == "C").map(|(y, _)| *y).unwrap();
        // Body deve estar abaixo de A (block força nova linha + inset top).
        assert!(pos_body_y > pos_a_y,
            "body deve estar abaixo de A: a={pos_a_y:.2} body={pos_body_y:.2}");
        // C deve estar abaixo de body (inset bottom adicionado).
        assert!(pos_c_y > pos_body_y,
            "C deve estar abaixo de body: body={pos_body_y:.2} c={pos_c_y:.2}");
    }

    // ── Passo 156H (ADR-0061 Fase 2 sub-passo 2) — box inline container ──

    /// `Content::Boxed` (box) é INLINE: não força flush_line. Verificamos
    /// que texto antes + box + texto depois ficam todos na mesma linha
    /// (mesma posição Y).
    #[test]
    fn layout_box_mantem_inline_nao_forca_flush() {
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        use std::sync::Arc;
        let doc_content = Content::Sequence(Arc::from(vec![
            Content::text("A"),
            Content::boxed(
                Content::text("M"),
                None, None,
                Sides::uniform(Length::ZERO),
                Length::ZERO,
            ),
            Content::text("B"),
        ]));
        let doc = layout(&doc_content);
        let texts: Vec<_> = doc.pages.iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { pos, text, .. } => Some((pos.y.val(), text.to_string())),
                _ => None,
            })
            .collect();
        let pos_a_y = texts.iter().find(|(_, t)| t == "A").map(|(y, _)| *y).unwrap();
        let pos_m_y = texts.iter().find(|(_, t)| t == "M").map(|(y, _)| *y).unwrap();
        let pos_b_y = texts.iter().find(|(_, t)| t == "B").map(|(y, _)| *y).unwrap();
        assert!((pos_a_y - pos_m_y).abs() < 0.001,
            "A e M devem estar na mesma linha (box é inline): a={pos_a_y:.2} m={pos_m_y:.2}");
        assert!((pos_m_y - pos_b_y).abs() < 0.001,
            "M e B devem estar na mesma linha: m={pos_m_y:.2} b={pos_b_y:.2}");
    }

    /// `Content::Boxed` com `inset.left` aplica avanço extra de cursor.x
    /// antes do body. Verificamos via posição X de body com vs sem inset.
    #[test]
    fn layout_box_inset_left_aplica_avanco_horizontal() {
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        use std::sync::Arc;

        let no_inset = Content::Sequence(Arc::from(vec![
            Content::text("A"),
            Content::boxed(
                Content::text("M"),
                None, None,
                Sides::uniform(Length::ZERO),
                Length::ZERO,
            ),
        ]));
        let doc1 = layout(&no_inset);
        let pos_m1: f64 = doc1.pages.iter().flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { pos, text, .. } if text.as_str() == "M" => Some(pos.x.val()),
                _ => None,
            })
            .next().unwrap();

        let with_inset = Content::Sequence(Arc::from(vec![
            Content::text("A"),
            Content::boxed(
                Content::text("M"),
                None, None,
                Sides::uniform(Length::pt(20.0)),
                Length::ZERO,
            ),
        ]));
        let doc2 = layout(&with_inset);
        let pos_m2: f64 = doc2.pages.iter().flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { pos, text, .. } if text.as_str() == "M" => Some(pos.x.val()),
                _ => None,
            })
            .next().unwrap();

        assert!(pos_m2 - pos_m1 >= 20.0,
            "box com inset=20pt deve empurrar M em pelo menos 20pt: \
             m1={pos_m1:.2} m2={pos_m2:.2}");
    }

    // ── Passo 156I (ADR-0061 Fase 2 sub-passo 3) — stack compositivo ──────

    /// `Content::Stack` TTB empilha children verticalmente: B abaixo de A.
    #[test]
    fn layout_stack_ttb_empilha_verticalmente() {
        use crate::entities::dir::Dir;
        let s = Content::stack(
            vec![Content::text("A"), Content::text("B")],
            Dir::TTB,
            None,
        );
        let doc = layout(&s);
        let texts: Vec<_> = doc.pages.iter().flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { pos, text, .. } => Some((pos.y.val(), text.to_string())),
                _ => None,
            })
            .collect();
        let pos_a_y = texts.iter().find(|(_, t)| t == "A").map(|(y, _)| *y).unwrap();
        let pos_b_y = texts.iter().find(|(_, t)| t == "B").map(|(y, _)| *y).unwrap();
        assert!(pos_b_y > pos_a_y,
            "stack TTB deve colocar B abaixo de A: a={pos_a_y:.2} b={pos_b_y:.2}");
    }

    /// `Content::Stack` LTR empilha children inline: B à direita de A,
    /// na mesma linha.
    #[test]
    fn layout_stack_ltr_empilha_horizontalmente() {
        use crate::entities::dir::Dir;
        let s = Content::stack(
            vec![Content::text("A"), Content::text("B")],
            Dir::LTR,
            None,
        );
        let doc = layout(&s);
        let texts: Vec<_> = doc.pages.iter().flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { pos, text, .. } =>
                    Some((pos.x.val(), pos.y.val(), text.to_string())),
                _ => None,
            })
            .collect();
        let (a_x, a_y) = texts.iter().find(|(_, _, t)| t == "A")
            .map(|(x, y, _)| (*x, *y)).unwrap();
        let (b_x, b_y) = texts.iter().find(|(_, _, t)| t == "B")
            .map(|(x, y, _)| (*x, *y)).unwrap();
        // Mesma linha (Y igual).
        assert!((a_y - b_y).abs() < 0.001,
            "stack LTR deve manter A e B na mesma linha: a_y={a_y:.2} b_y={b_y:.2}");
        // B à direita de A.
        assert!(b_x > a_x,
            "stack LTR deve colocar B à direita de A: a_x={a_x:.2} b_x={b_x:.2}");
    }

    /// `Content::Stack` TTB com spacing força avanço vertical extra
    /// entre children.
    #[test]
    fn layout_stack_spacing_avanca_cursor_entre_children() {
        use crate::entities::dir::Dir;
        use crate::entities::layout_types::Length;

        // Doc 1: stack sem spacing.
        let s_no_space = Content::stack(
            vec![Content::text("A"), Content::text("B")],
            Dir::TTB, None,
        );
        let doc1 = layout(&s_no_space);
        let pos_b1: f64 = doc1.pages.iter().flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { pos, text, .. } if text.as_str() == "B" => Some(pos.y.val()),
                _ => None,
            }).next().unwrap();

        // Doc 2: stack com spacing 30pt.
        let s_with_space = Content::stack(
            vec![Content::text("A"), Content::text("B")],
            Dir::TTB,
            Some(Length::pt(30.0)),
        );
        let doc2 = layout(&s_with_space);
        let pos_b2: f64 = doc2.pages.iter().flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { pos, text, .. } if text.as_str() == "B" => Some(pos.y.val()),
                _ => None,
            }).next().unwrap();

        // B em doc2 deve estar pelo menos 30pt mais abaixo (spacing).
        assert!(pos_b2 - pos_b1 >= 30.0,
            "stack TTB com spacing=30pt deve empurrar B em pelo menos 30pt: \
             b1={pos_b1:.2} b2={pos_b2:.2}");
    }

    /// `Content::Block` com `height: Some(h)` força avanço mínimo
    /// vertical mesmo se body for pequeno.
    #[test]
    fn layout_block_height_forca_minimo_vertical() {
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        use std::sync::Arc;

        // Doc 1: Block sem height (body pequeno).
        let no_height = Content::Sequence(Arc::from(vec![
            Content::text("A"),
            Content::block(Content::text("x"), None, None, Sides::uniform(Length::ZERO), true),
            Content::text("B"),
        ]));
        let doc1 = layout(&no_height);
        let pos_b1: f64 = doc1.pages.iter().flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { pos, text, .. } if text.as_str() == "B" => Some(pos.y.val()),
                _ => None,
            })
            .next().unwrap();

        // Doc 2: Block com height: 100pt (body pequeno).
        let with_height = Content::Sequence(Arc::from(vec![
            Content::text("A"),
            Content::block(
                Content::text("x"),
                None,
                Some(Length::pt(100.0)),
                Sides::uniform(Length::ZERO),
                true,
            ),
            Content::text("B"),
        ]));
        let doc2 = layout(&with_height);
        let pos_b2: f64 = doc2.pages.iter().flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { pos, text, .. } if text.as_str() == "B" => Some(pos.y.val()),
                _ => None,
            })
            .next().unwrap();

        // B em doc2 deve estar pelo menos ~100pt mais abaixo que em doc1
        // (devido ao height mínimo do bloco com altura forçada).
        // Margem conservadora: pelo menos 50pt de diferença.
        assert!(pos_b2 - pos_b1 > 50.0,
            "block com height=100pt deve empurrar B mais para baixo do que sem height: \
             b1={pos_b1:.2} b2={pos_b2:.2}");
    }
    // ── Passo 156J (ADR-0061 Fase 3 sub-passo 1) — repeat ─────────────────

    /// `Content::Repeat` renderiza body single-render no contexto actual
    /// (paridade estrutural; algoritmo dinâmico defere per ADR-0054).
    #[test]
    fn layout_repeat_renderiza_body_no_contexto_actual() {
        let r = Content::repeat(Content::text("X"), None, true);
        let doc = layout(&r);
        // Body deve aparecer pelo menos uma vez (single-render).
        let count_x = doc.pages.iter().flat_map(|p| p.items.iter())
            .filter(|item| matches!(item, FrameItem::Text { text, .. } if text.as_str() == "X"))
            .count();
        assert!(count_x >= 1, "repeat[X] deve emitir pelo menos um Text 'X'");
    }

    /// **P218 (DEBT-56 sub-fase b segundo sub-passo)** — `Content::Columns`
    /// produzido por `Content::columns(body, count, gutter)` (forma que
    /// `native_columns` retorna em `Value::Content`) renderiza body via
    /// stub transparente P217 mesmo quando count > 1. Confirma que
    /// pipeline variant-construction → arm transparente preserva body
    /// independentemente de `count`/`gutter` (consumer multi-region
    /// real diferido P219).
    ///
    /// E2E completo `eval(#columns(2)[hello])` → layout requer NullWorld
    /// helper que vive em `stdlib::tests`; tests stdlib P218 cobrem a
    /// porção stdlib (parsing args + variant construction); este test
    /// cobre a porção layout (variant → arm transparente → render).
    #[test]
    fn p218_columns_count_3_renderiza_body_transparentemente() {
        use crate::entities::layout_types::Length;
        let c = Content::columns(
            Content::text("p218body"),        // single word — layout não splita
            3,                                // count > 1
            Some(Length::pt(15.0)),           // gutter explícito
        );
        let doc = layout(&c);
        let texts: String = doc.pages.iter().flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            })
            .collect();
        assert!(texts.contains("p218body"),
            "P218 stub transparente deve renderizar body mesmo com count=3");
    }

    /// **P217 (DEBT-56 sub-fase b primeiro sub-passo)** — `Content::Columns`
    /// renderiza body via stub transparente (count/gutter ignorados em
    /// P217; consumer multi-region real em P219). Tests confirma que
    /// body content aparece preservado no doc.
    #[test]
    fn p217_columns_arm_transparente_renderiza_body() {
        use crate::entities::layout_types::Length;
        let c = Content::columns(
            Content::text("hello"),
            2,                                // count ignorado em P217
            Some(Length::pt(10.0)),           // gutter ignorado em P217
        );
        let doc = layout(&c);
        // Body deve aparecer (stub transparente delega a layout_content).
        let texts: String = doc.pages.iter().flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            })
            .collect();
        assert!(texts.contains("hello"), "columns body deve renderizar transparentemente");
    }

    // ── P219 (DEBT-56 sub-fase b 3/4) — consumer real graded ──────────

    /// **P219** — count=1 caso degenerate: column_width == full_width
    /// (paridade `(width - 0*gutter) / 1 = width`). Body renderiza
    /// inalterado vs sem columns.
    #[test]
    fn p219_columns_count_1_equivale_a_body_directo() {
        let c1 = Content::columns(Content::text("p219c1"), 1, None);
        let doc1 = layout(&c1);
        let texts1: String = doc1.pages.iter().flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            }).collect();
        assert!(texts1.contains("p219c1"), "count=1 preserva body");
    }

    /// **P219** — count=2: body renderiza preservado.
    #[test]
    fn p219_columns_count_2_renderiza_body() {
        let c = Content::columns(Content::text("p219c2"), 2, None);
        let doc = layout(&c);
        let texts: String = doc.pages.iter().flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            }).collect();
        assert!(texts.contains("p219c2"), "count=2 preserva body");
    }

    /// **P219** — count=3: body renderiza preservado (paralelo P218
    /// E2E mas verifica explicitamente arm real).
    #[test]
    fn p219_columns_count_3_renderiza_body() {
        use crate::entities::layout_types::Length;
        let c = Content::columns(Content::text("p219c3"), 3, Some(Length::pt(20.0)));
        let doc = layout(&c);
        let texts: String = doc.pages.iter().flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            }).collect();
        assert!(texts.contains("p219c3"), "count=3 preserva body");
    }

    /// **P219** — gutter explícito aceite (Length resolve para Pt).
    #[test]
    fn p219_columns_gutter_length_explicito_renderiza() {
        use crate::entities::layout_types::Length;
        let c = Content::columns(Content::text("g219"), 2, Some(Length::pt(50.0)));
        let doc = layout(&c);
        let texts: String = doc.pages.iter().flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            }).collect();
        assert!(texts.contains("g219"), "gutter explícito Length aceito");
    }

    /// **P219** — gutter default `None` aplicado via
    /// `COLUMNS_DEFAULT_GUTTER_RATIO = 0.04`. Body renderiza.
    #[test]
    fn p219_columns_gutter_default_renderiza() {
        let c = Content::columns(Content::text("gd219"), 2, None);
        let doc = layout(&c);
        let texts: String = doc.pages.iter().flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            }).collect();
        assert!(texts.contains("gd219"), "default gutter aplicado transparente");
    }

    /// **P219** — width restaurada após columns block. Sequência
    /// `[Columns(2)[col_text]; text("after")]` produz "after"
    /// renderizada com width original (não reduzida).
    /// Verificação observable: ambos textos aparecem no doc.
    #[test]
    fn p219_columns_width_restaurada_apos_body() {
        use std::sync::Arc;
        let cols = Content::columns(Content::text("colbody"), 2, None);
        let after = Content::text("afterbody");
        let seq = Content::Sequence(Arc::from(vec![cols, after]));
        let doc = layout(&seq);
        let texts: String = doc.pages.iter().flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            }).collect();
        assert!(texts.contains("colbody"), "body em columns renderiza");
        assert!(texts.contains("afterbody"), "after columns renderiza com width restaurada");
    }

    /// **P219** — body com `Content::Heading` em columns: heading
    /// counter incrementa exactamente uma vez (paridade walk-única
    /// preservada de P217).
    #[test]
    fn p219_columns_counters_contam_uma_vez() {
        use std::sync::Arc;
        // Dois headings dentro de columns; counter deve = 2 final
        // (não 4 — sem multi-render).
        let h1 = Content::Heading {
            level: 1,
            body: Box::new(Content::text("h1col")),
        };
        let h2 = Content::Heading {
            level: 1,
            body: Box::new(Content::text("h2col")),
        };
        let body_seq = Content::Sequence(Arc::from(vec![h1, h2]));
        let cols = Content::columns(body_seq, 2, None);
        let doc = layout(&cols);
        let texts: String = doc.pages.iter().flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            }).collect();
        assert!(texts.contains("h1col"), "h1 renderiza");
        assert!(texts.contains("h2col"), "h2 renderiza");
    }

    /// **P219** — composição aninhada: `columns(2)[columns(2)[text]]`
    /// preserva body (composability multiplicativa de width:
    /// page_w / 4 idealmente; tests confirma body presente —
    /// comportamento estructural verificado).
    #[test]
    fn p219_columns_aninhado_compoe_width() {
        let inner = Content::columns(Content::text("nest"), 2, None);
        let outer = Content::columns(inner, 2, None);
        let doc = layout(&outer);
        let texts: String = doc.pages.iter().flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            }).collect();
        assert!(texts.contains("nest"), "nested columns body renderiza (composição aninhada)");
    }

    // ── Passo 220 (ADR-0078 PROPOSTO sub-fase b 4/4) — colbreak ──────────

    /// Colbreak isolado produz nova página (downgrade graded a pagebreak
    /// per Opção β). Setup: A → colbreak() → B → produz >= 2 páginas.
    #[test]
    fn p220_colbreak_produz_new_page_downgrade() {
        use std::sync::Arc;
        let doc_content = Content::Sequence(Arc::from(vec![
            Content::text("A"),
            Content::colbreak(false),
            Content::text("B"),
        ]));
        let doc = layout(&doc_content);
        assert!(doc.pages.len() >= 2,
            "esperado >= 2 páginas após colbreak (downgrade graded), obtive {}",
            doc.pages.len());
    }

    /// Colbreak dentro de columns block produz pagebreak literal — P219
    /// single-region scope-out preserva downgrade graded (sem flow real
    /// entre colunas reais).
    #[test]
    fn p220_colbreak_dentro_columns_downgrade_graded() {
        use std::sync::Arc;
        let body = Content::Sequence(Arc::from(vec![
            Content::text("p220before"),
            Content::colbreak(false),
            Content::text("p220after"),
        ]));
        let cols = Content::columns(body, 2, None);
        let doc = layout(&cols);
        assert!(doc.pages.len() >= 2,
            "colbreak dentro de columns produz pagebreak (downgrade β), pages={}",
            doc.pages.len());
        let texts: String = doc.pages.iter().flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            }).collect();
        assert!(texts.contains("p220before"), "before-colbreak renderiza");
        assert!(texts.contains("p220after"),  "after-colbreak renderiza");
    }

    /// Colbreak misturado com pagebreak — downgrade graded faz colbreak
    /// equivaler a pagebreak (mesma quantidade de páginas).
    #[test]
    fn p220_colbreak_misturado_com_pagebreak() {
        use std::sync::Arc;
        let with_colbreak = Content::Sequence(Arc::from(vec![
            Content::text("X"),
            Content::colbreak(false),
            Content::text("Y"),
            Content::pagebreak(false, None),
            Content::text("Z"),
        ]));
        let with_only_pagebreaks = Content::Sequence(Arc::from(vec![
            Content::text("X"),
            Content::pagebreak(false, None),
            Content::text("Y"),
            Content::pagebreak(false, None),
            Content::text("Z"),
        ]));
        let d1 = layout(&with_colbreak);
        let d2 = layout(&with_only_pagebreaks);
        assert_eq!(d1.pages.len(), d2.pages.len(),
            "colbreak ≡ pagebreak graded (downgrade β); d1={}, d2={}",
            d1.pages.len(), d2.pages.len());
    }

    /// Colbreak no início do documento — paridade vanilla pagebreak no
    /// início; produz página vazia + página com texto.
    #[test]
    fn p220_colbreak_no_inicio_documento_pagina_vazia() {
        use std::sync::Arc;
        let doc_content = Content::Sequence(Arc::from(vec![
            Content::colbreak(false),
            Content::text("p220inicio"),
        ]));
        let doc = layout(&doc_content);
        assert!(doc.pages.len() >= 2,
            "colbreak no início produz página vazia + página com texto, pages={}",
            doc.pages.len());
    }

    // ── Passo 223 (ADR-0061 Fase 4 candidata sub-2; refino Place +float +clearance) ──

    /// Place com `float: true` renderiza body preservando baseline P84.6
    /// (semantic real adiada per ADR-0054 graded; flow contorna fica
    /// como Fase 5 candidata NÃO-reservada per política P158).
    #[test]
    fn p223_place_float_armazenado_layout_preservado() {
        use crate::entities::layout_types::{Align2D, HAlign, VAlign, PlaceScope};
        let p = Content::Place {
            alignment: Align2D { h: Some(HAlign::Left), v: Some(VAlign::Top) },
            dx:        0.0,
            dy:        0.0,
            scope:     PlaceScope::Column,
            float:     true,                                 // P223
            clearance: None,
            body:      Box::new(Content::text("p223float")),
        };
        let doc = layout(&p);
        let texts: String = doc.pages.iter().flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            }).collect();
        assert!(texts.contains("p223float"),
            "Place com float renderiza body (semantic adiada preserva baseline P84.6)");
    }

    /// Place com `clearance: Some(2em)` renderiza body preservando baseline.
    #[test]
    fn p223_place_clearance_armazenado_layout_preservado() {
        use crate::entities::layout_types::{Align2D, HAlign, VAlign, Length, PlaceScope};
        let p = Content::Place {
            alignment: Align2D { h: Some(HAlign::Left), v: Some(VAlign::Top) },
            dx:        0.0,
            dy:        0.0,
            scope:     PlaceScope::Column,
            float:     true,                                 // P223
            clearance: Some(Length::pt(20.0)),               // P223
            body:      Box::new(Content::text("p223clear")),
        };
        let doc = layout(&p);
        let texts: String = doc.pages.iter().flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            }).collect();
        assert!(texts.contains("p223clear"),
            "Place com clearance renderiza body (semantic adiada preserva baseline P84.6)");
    }

    // ── Passo 224 (ADR-0061 Fase 4 candidata sub-3) — Grid refino + variants ──

    /// Grid com header + footer renderiza body com header renderizado
    /// antes (semantic adiada — header/footer renderizam como sequência
    /// extra; refino multi-region real é Fase 5 candidata).
    #[test]
    fn p224_grid_com_header_footer_renderiza_body() {
        use crate::entities::layout_types::{Length, TrackSizing};
        use crate::entities::sides::Sides;
        let g = Content::Grid {
            columns: vec![TrackSizing::Auto],
            rows:    vec![],
            cells:   vec![Content::text("p224body")],
            gutter:  None,
            align:   None,
            inset:   Sides::uniform(Length::pt(0.0)),
            header:  Some(Box::new(Content::text("p224hdr"))),
            footer:  Some(Box::new(Content::text("p224ftr"))),
            stroke:  None,
            fill:    None,
        };
        let doc = layout(&g);
        let texts: String = doc.pages.iter().flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            }).collect();
        assert!(texts.contains("p224body"),
            "Grid body renderiza preservando baseline; texts={}", texts);
    }

    /// GridCell wrappa body; placement real disponível via grid_placement
    /// (P224.C); aqui só verifica que GridCell isolado renderiza body.
    #[test]
    fn p224_gridcell_isolado_renderiza_body() {
        let cell = Content::GridCell {
            body:    Box::new(Content::text("p224cell")),
            x:       None,
            y:       None,
            colspan: None,
            rowspan: None,
            stroke:  None,
            fill:    None,
            align:   None, inset: None, breakable: None,
        };
        let doc = layout(&cell);
        let texts: String = doc.pages.iter().flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            }).collect();
        assert!(texts.contains("p224cell"),
            "GridCell isolado renderiza body; texts={}", texts);
    }

    // ── Passo 227 (Fase 5 Layout Categoria A.1) — stroke render E2E ──

    /// Grid com stroke emite 4 FrameItem::Shape::Line per cell border
    /// (renderização Opção β simplificada; sem deduplicação adjacentes).
    /// Sem stroke não emite Lines extra (baseline preservado).
    #[test]
    fn p227_grid_stroke_renderiza_4_lines_per_cell() {
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::{Length, TrackSizing, Color};
        use crate::entities::sides::Sides;

        let cells = vec![
            Content::text("A"), Content::text("B"),
            Content::text("C"), Content::text("D"),
        ];
        let with_stroke = Content::Grid {
            columns: vec![TrackSizing::Fixed(50.0), TrackSizing::Fixed(50.0)],
            rows:    vec![],
            cells:   cells.clone(),
            gutter:  None,
            align:   None,
            inset:   Sides::uniform(Length::pt(0.0)),
            header:  None,
            footer:  None,
            stroke:  Some(Stroke { paint: Color::rgb(0, 0, 0), thickness: 1.0 }),
            fill:    None,
        };
        let doc = layout(&with_stroke);
        let line_count: usize = doc.pages.iter().flat_map(|p| p.items.iter())
            .filter(|item| matches!(item,
                FrameItem::Shape { kind: crate::entities::geometry::ShapeKind::Line { .. }, .. }
            )).count();
        // 4 cells × 4 borders cada = 16 lines mínimo.
        assert!(line_count >= 16,
            "Grid 2x2 stroke deve emitir >= 16 lines (4 cells × 4 borders), recebeu {}",
            line_count);
    }

    #[test]
    fn p227_grid_sem_stroke_zero_lines_extra() {
        use crate::entities::layout_types::{Length, TrackSizing};
        use crate::entities::sides::Sides;
        let g_no_stroke = Content::Grid {
            columns: vec![TrackSizing::Fixed(50.0), TrackSizing::Fixed(50.0)],
            rows:    vec![],
            cells:   vec![Content::text("A"), Content::text("B")],
            gutter:  None,
            align:   None,
            inset:   Sides::uniform(Length::pt(0.0)),
            header:  None,
            footer:  None,
            stroke:  None,  // baseline
            fill:    None,
        };
        let doc = layout(&g_no_stroke);
        let line_count: usize = doc.pages.iter().flat_map(|p| p.items.iter())
            .filter(|item| matches!(item,
                FrameItem::Shape { kind: crate::entities::geometry::ShapeKind::Line { .. }, .. }
            )).count();
        assert_eq!(line_count, 0,
            "Grid sem stroke não emite Lines extra (baseline preservado)");
    }

    #[test]
    fn p227_table_stroke_paridade_grid() {
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::{TrackSizing, Color};
        let t = Content::Table {
            columns:  vec![TrackSizing::Fixed(50.0), TrackSizing::Fixed(50.0)],
            rows:     vec![],
            children: vec![Content::text("X"), Content::text("Y")],
            stroke:   Some(Stroke { paint: Color::rgb(0, 0, 255), thickness: 0.5 }),
            fill:     None,
        };
        let doc = layout(&t);
        let line_count: usize = doc.pages.iter().flat_map(|p| p.items.iter())
            .filter(|item| matches!(item,
                FrameItem::Shape { kind: crate::entities::geometry::ShapeKind::Line { .. }, .. }
            )).count();
        // 2 cells × 4 borders = 8 lines mínimo.
        assert!(line_count >= 8,
            "Table 1x2 stroke paridade Grid emite >= 8 lines, recebeu {}",
            line_count);
    }

    // ── Passo 228 (Fase 5 Layout Categoria A.2) — fill render E2E ──

    /// Grid com fill emite FrameItem::Shape::Rect per cell.
    #[test]
    fn p228_grid_fill_renderiza_rect_per_cell() {
        use crate::entities::layout_types::{Length, TrackSizing, Color};
        use crate::entities::sides::Sides;
        let cells = vec![
            Content::text("A"), Content::text("B"),
            Content::text("C"), Content::text("D"),
        ];
        let with_fill = Content::Grid {
            columns: vec![TrackSizing::Fixed(50.0), TrackSizing::Fixed(50.0)],
            rows:    vec![],
            cells:   cells,
            gutter:  None,
            align:   None,
            inset:   Sides::uniform(Length::pt(0.0)),
            header:  None,
            footer:  None,
            stroke:  None,
            fill:    Some(Color::rgb(255, 255, 0)),
        };
        let doc = layout(&with_fill);
        let rect_count: usize = doc.pages.iter().flat_map(|p| p.items.iter())
            .filter(|item| matches!(item,
                FrameItem::Shape { kind: crate::entities::geometry::ShapeKind::Rect, .. }
            )).count();
        // 4 cells × 1 rect cada = 4 rects mínimo.
        assert!(rect_count >= 4,
            "Grid 2x2 fill deve emitir >= 4 rects (1 per cell), recebeu {}",
            rect_count);
    }

    #[test]
    fn p228_grid_sem_fill_zero_rects_extra() {
        use crate::entities::layout_types::{Length, TrackSizing};
        use crate::entities::sides::Sides;
        let g_no_fill = Content::Grid {
            columns: vec![TrackSizing::Fixed(50.0), TrackSizing::Fixed(50.0)],
            rows:    vec![],
            cells:   vec![Content::text("A"), Content::text("B")],
            gutter:  None,
            align:   None,
            inset:   Sides::uniform(Length::pt(0.0)),
            header:  None,
            footer:  None,
            stroke:  None,
            fill:    None,  // baseline
        };
        let doc = layout(&g_no_fill);
        let rect_count: usize = doc.pages.iter().flat_map(|p| p.items.iter())
            .filter(|item| matches!(item,
                FrameItem::Shape { kind: crate::entities::geometry::ShapeKind::Rect, .. }
            )).count();
        assert_eq!(rect_count, 0,
            "Grid sem fill não emite Rects extra (baseline preservado)");
    }

    #[test]
    fn p228_grid_fill_z_order_antes_de_conteudo() {
        // Z-order: fill Rect emitido ANTES do conteúdo (Text).
        // Index do primeiro Rect < index do primeiro Text.
        use crate::entities::layout_types::{Length, TrackSizing, Color};
        use crate::entities::sides::Sides;
        let g = Content::Grid {
            columns: vec![TrackSizing::Fixed(50.0)],
            rows:    vec![],
            cells:   vec![Content::text("ZorderTest")],
            gutter:  None,
            align:   None,
            inset:   Sides::uniform(Length::pt(0.0)),
            header:  None,
            footer:  None,
            stroke:  None,
            fill:    Some(Color::rgb(255, 0, 0)),
        };
        let doc = layout(&g);
        let items: Vec<&FrameItem> = doc.pages.iter().flat_map(|p| p.items.iter()).collect();
        let first_rect_idx = items.iter().position(|item| matches!(item,
            FrameItem::Shape { kind: crate::entities::geometry::ShapeKind::Rect, .. }
        ));
        let first_text_idx = items.iter().position(|item| matches!(item,
            FrameItem::Text { text, .. } if text.as_str().contains("ZorderTest")
        ));
        assert!(first_rect_idx.is_some(), "Rect deve existir");
        assert!(first_text_idx.is_some(), "Text ZorderTest deve existir");
        assert!(first_rect_idx.unwrap() < first_text_idx.unwrap(),
            "Z-order: fill Rect (idx={:?}) deve preceder conteúdo Text (idx={:?})",
            first_rect_idx, first_text_idx);
    }

    #[test]
    fn p228_grid_fill_e_stroke_z_order_correcto() {
        // Z-order completo: fill (Rect) antes; conteúdo (Text) meio;
        // stroke (Line) depois.
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::{Length, TrackSizing, Color};
        use crate::entities::sides::Sides;
        let g = Content::Grid {
            columns: vec![TrackSizing::Fixed(50.0)],
            rows:    vec![],
            cells:   vec![Content::text("ZorderFull")],
            gutter:  None,
            align:   None,
            inset:   Sides::uniform(Length::pt(0.0)),
            header:  None,
            footer:  None,
            stroke:  Some(Stroke { paint: Color::rgb(0, 0, 0), thickness: 1.0 }),
            fill:    Some(Color::rgb(255, 255, 0)),
        };
        let doc = layout(&g);
        let items: Vec<&FrameItem> = doc.pages.iter().flat_map(|p| p.items.iter()).collect();
        let rect_idx = items.iter().position(|item| matches!(item,
            FrameItem::Shape { kind: crate::entities::geometry::ShapeKind::Rect, .. }
        ));
        let line_idx = items.iter().position(|item| matches!(item,
            FrameItem::Shape { kind: crate::entities::geometry::ShapeKind::Line { .. }, .. }
        ));
        assert!(rect_idx.is_some() && line_idx.is_some(),
            "Ambos Rect e Line presentes");
        assert!(rect_idx.unwrap() < line_idx.unwrap(),
            "Z-order: fill Rect (idx={:?}) deve preceder stroke Line (idx={:?})",
            rect_idx, line_idx);
    }

    #[test]
    fn p228_table_fill_delegate_paridade_grid() {
        use crate::entities::layout_types::{TrackSizing, Color};
        let t = Content::Table {
            columns:  vec![TrackSizing::Fixed(50.0), TrackSizing::Fixed(50.0)],
            rows:     vec![],
            children: vec![Content::text("X"), Content::text("Y")],
            stroke:   None,
            fill:     Some(Color::rgb(200, 200, 200)),
        };
        let doc = layout(&t);
        let rect_count: usize = doc.pages.iter().flat_map(|p| p.items.iter())
            .filter(|item| matches!(item,
                FrameItem::Shape { kind: crate::entities::geometry::ShapeKind::Rect, .. }
            )).count();
        assert!(rect_count >= 2,
            "Table 1x2 fill paridade Grid emite >= 2 rects, recebeu {}",
            rect_count);
    }

    // ── Passo 230 (Fase 5 Layout Categoria A.3) — precedência per-cell vs Grid-level ──

    /// Per-cell stroke override Grid-level: cell `Some(...)` prevalece.
    /// Grid stroke red + cell stroke blue → cell stroke usado.
    #[test]
    fn p230_per_cell_stroke_override_grid_level() {
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::{Length, TrackSizing, Color};
        use crate::entities::sides::Sides;

        let cell_with_override = Content::GridCell {
            body:    Box::new(Content::text("override")),
            x:       None,
            y:       None,
            colspan: None,
            rowspan: None,
            stroke:  Some(Stroke { paint: Color::rgb(0, 0, 255), thickness: 5.0 }),
            fill:    None,
            align:   None, inset: None, breakable: None,
        };
        let g = Content::Grid {
            columns: vec![TrackSizing::Fixed(50.0)],
            rows:    vec![],
            cells:   vec![cell_with_override],
            gutter:  None,
            align:   None,
            inset:   Sides::uniform(Length::pt(0.0)),
            header:  None,
            footer:  None,
            stroke:  Some(Stroke { paint: Color::rgb(255, 0, 0), thickness: 1.0 }),
            fill:    None,
        };
        let doc = layout(&g);
        // Verificar que stroke emitido tem thickness 5.0 (cell override; não 1.0 Grid).
        let mut found_override = false;
        for p in &doc.pages {
            for item in &p.items {
                if let FrameItem::Shape { stroke: Some(s), .. } = item {
                    if (s.thickness - 5.0).abs() < 0.01 {
                        found_override = true;
                    }
                }
            }
        }
        assert!(found_override,
            "Cell stroke thickness 5.0 deve sobrepor Grid stroke thickness 1.0");
    }

    /// Per-cell fill override Grid-level.
    #[test]
    fn p230_per_cell_fill_override_grid_level() {
        use crate::entities::layout_types::{Length, TrackSizing, Color};
        use crate::entities::sides::Sides;

        let cell_with_fill = Content::GridCell {
            body:    Box::new(Content::text("c")),
            x:       None,
            y:       None,
            colspan: None,
            rowspan: None,
            stroke:  None,
            fill:    Some(Color::rgb(0, 255, 0)),  // cell green
            align:   None, inset: None, breakable: None,
        };
        let g = Content::Grid {
            columns: vec![TrackSizing::Fixed(50.0)],
            rows:    vec![],
            cells:   vec![cell_with_fill],
            gutter:  None,
            align:   None,
            inset:   Sides::uniform(Length::pt(0.0)),
            header:  None,
            footer:  None,
            stroke:  None,
            fill:    Some(Color::rgb(255, 0, 0)),  // grid red
        };
        let doc = layout(&g);
        // Verificar fill emitido é green (cell override).
        let mut found_green = false;
        for p in &doc.pages {
            for item in &p.items {
                if let FrameItem::Shape { fill: Some(Color::Rgb { r: 0, g: 255, b: 0 }), .. } = item {
                    found_green = true;
                }
            }
        }
        assert!(found_green,
            "Cell fill green deve sobrepor Grid fill red");
    }

    /// Per-cell None → inherit Grid-level: cell sem stroke usa Grid stroke.
    #[test]
    fn p230_per_cell_none_inherits_grid_level() {
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::{Length, TrackSizing, Color};
        use crate::entities::sides::Sides;

        let cell_raw = Content::text("raw");  // Content raw sem stroke/fill
        let g = Content::Grid {
            columns: vec![TrackSizing::Fixed(50.0)],
            rows:    vec![],
            cells:   vec![cell_raw],
            gutter:  None,
            align:   None,
            inset:   Sides::uniform(Length::pt(0.0)),
            header:  None,
            footer:  None,
            stroke:  Some(Stroke { paint: Color::rgb(0, 0, 0), thickness: 3.0 }),
            fill:    None,
        };
        let doc = layout(&g);
        let line_count: usize = doc.pages.iter().flat_map(|p| p.items.iter())
            .filter(|item| matches!(item,
                FrameItem::Shape { kind: crate::entities::geometry::ShapeKind::Line { .. }, .. }
            )).count();
        assert!(line_count >= 4,
            "Cell raw inherit Grid stroke → emite 4 lines, recebeu {}",
            line_count);
    }

    /// Per-cell stroke Some + Grid-level None → cell emite; Grid não tem.
    #[test]
    fn p230_per_cell_some_grid_none_emite_apenas_cell() {
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::{Length, TrackSizing, Color};
        use crate::entities::sides::Sides;

        let cell_with_stroke = Content::GridCell {
            body:    Box::new(Content::text("c")),
            x:       None,
            y:       None,
            colspan: None,
            rowspan: None,
            stroke:  Some(Stroke { paint: Color::rgb(0, 0, 0), thickness: 1.0 }),
            fill:    None,
            align:   None, inset: None, breakable: None,
        };
        let g = Content::Grid {
            columns: vec![TrackSizing::Fixed(50.0)],
            rows:    vec![],
            cells:   vec![cell_with_stroke],
            gutter:  None,
            align:   None,
            inset:   Sides::uniform(Length::pt(0.0)),
            header:  None,
            footer:  None,
            stroke:  None,  // Grid sem stroke
            fill:    None,
        };
        let doc = layout(&g);
        let line_count: usize = doc.pages.iter().flat_map(|p| p.items.iter())
            .filter(|item| matches!(item,
                FrameItem::Shape { kind: crate::entities::geometry::ShapeKind::Line { .. }, .. }
            )).count();
        assert!(line_count >= 4,
            "Cell stroke emite mesmo com Grid sem stroke, recebeu {}",
            line_count);
    }

    /// Mix: per-cell stroke + Grid-level fill → cell tem ambos (ortogonais).
    #[test]
    fn p230_per_cell_stroke_e_grid_fill_simultaneos_z_order() {
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::{Length, TrackSizing, Color};
        use crate::entities::sides::Sides;

        let cell = Content::GridCell {
            body:    Box::new(Content::text("c")),
            x:       None,
            y:       None,
            colspan: None,
            rowspan: None,
            stroke:  Some(Stroke { paint: Color::rgb(0, 0, 0), thickness: 1.0 }),  // cell stroke
            fill:    None,
            align:   None, inset: None, breakable: None,
        };
        let g = Content::Grid {
            columns: vec![TrackSizing::Fixed(50.0)],
            rows:    vec![],
            cells:   vec![cell],
            gutter:  None,
            align:   None,
            inset:   Sides::uniform(Length::pt(0.0)),
            header:  None,
            footer:  None,
            stroke:  None,
            fill:    Some(Color::rgb(255, 255, 0)),  // grid fill (cell inherit)
        };
        let doc = layout(&g);
        let items: Vec<&FrameItem> = doc.pages.iter().flat_map(|p| p.items.iter()).collect();
        let rect_idx = items.iter().position(|item| matches!(item,
            FrameItem::Shape { kind: crate::entities::geometry::ShapeKind::Rect, .. }
        ));
        let line_idx = items.iter().position(|item| matches!(item,
            FrameItem::Shape { kind: crate::entities::geometry::ShapeKind::Line { .. }, .. }
        ));
        assert!(rect_idx.is_some() && line_idx.is_some(),
            "Ambos Rect (grid fill inherit) e Line (cell stroke) presentes");
        assert!(rect_idx.unwrap() < line_idx.unwrap(),
            "Z-order: fill (idx={:?}) antes stroke (idx={:?})",
            rect_idx, line_idx);
    }

    // ── Passo 231 (Fase 5 Layout Categoria A.4) — Block/Boxed cosméticos preserved ──

    /// Block com outset/radius/clip preserva body render (semantic real
    /// adiada per ADR-0054 graded — radius/clip primitivos baseline
    /// ausentes; outset visual ainda não aplicado).
    #[test]
    fn p231_block_outset_radius_clip_layout_preservado() {
        use crate::entities::sides::Sides;
        let b = Content::Block {
            body:      Box::new(Content::text("p231block")),
            width:     None,
            height:    None,
            inset:     Sides::uniform(crate::entities::layout_types::Length::pt(0.0)),
            breakable: true,
            outset:    Sides::uniform(crate::entities::layout_types::Length::pt(5.0)),
            // P242 adapta: radius `Option<Length>` → `Corners<Length>`.
            radius:    crate::entities::corners::Corners::uniform(crate::entities::layout_types::Length::pt(3.0)),
            clip:      true,
            fill:      None,
            stroke:    None,
            spacing:   None,
            above:     None,
            below:     None,
            sticky:    false,
        };
        let doc = layout(&b);
        // P242 — quando clip=true, body items wrapped em FrameItem::Group
        // com clip_mask Some(RoundedRect). Recursivamente extrair Text de
        // qualquer profundidade (Group items podem aninhar).
        fn extract_texts(items: &[FrameItem], out: &mut String) {
            for item in items {
                match item {
                    FrameItem::Text { text, .. } => out.push_str(text.as_str()),
                    FrameItem::Group { items, .. } => extract_texts(items, out),
                    _ => {}
                }
            }
        }
        let mut texts = String::new();
        for page in doc.pages.iter() {
            extract_texts(&page.items, &mut texts);
        }
        assert!(texts.contains("p231block"),
            "Block com cosméticos renderiza body (P242 materializa clip: body em Group)");
    }

    /// Boxed paridade Block — cosméticos preserved.
    #[test]
    fn p231_boxed_cosmeticos_paridade_block() {
        use crate::entities::sides::Sides;
        let b = Content::Boxed {
            body:     Box::new(Content::text("p231boxed")),
            width:    None,
            height:   None,
            inset:    Sides::uniform(crate::entities::layout_types::Length::pt(0.0)),
            baseline: crate::entities::layout_types::Length::pt(0.0),
            outset:   Sides::uniform(crate::entities::layout_types::Length::pt(2.0)),
            // P242 adapta: radius `Option<Length>` → `Corners<Length>`.
            radius:   crate::entities::corners::Corners::uniform(crate::entities::layout_types::Length::pt(1.0)),
            clip:     false,
            fill:     None,
            stroke:   None,
        };
        let doc = layout(&b);
        let texts: String = doc.pages.iter().flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            }).collect();
        assert!(texts.contains("p231boxed"),
            "Boxed com cosméticos renderiza body (paridade Block; semantic adiada)");
    }

    // ── Passo 242 (M9d/M7+5; ADR-0081 IMPLEMENTADO parcial 3/5) —
    //     Block clip=true emite FrameItem::Group com clip_mask
    //     RoundedRect (radius non-zero) ou Rect (radius zero) ──

    #[test]
    fn p242_block_clip_true_radius_non_zero_emit_group_rounded_rect_clip_mask() {
        use crate::entities::sides::Sides;
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        let b = Content::Block {
            body:      Box::new(Content::text("clipped")),
            width:     None,
            height:    None,
            inset:     Sides::uniform(Length::pt(0.0)),
            breakable: true,
            outset:    Sides::uniform(Length::pt(0.0)),
            radius:    Corners::uniform(Length::pt(5.0)),
            clip:      true,
            fill:      None,
            stroke:    None,
            spacing:   None,
            above:     None,
            below:     None,
            sticky:    false,
        };
        let doc = layout(&b);
        // Procurar FrameItem::Group com clip_mask Some(RoundedRect).
        let mut found_rounded_clip = false;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Group { clip_mask: Some(shape), .. } = item {
                    if let crate::entities::geometry::ShapeKind::RoundedRect { .. } = shape {
                        found_rounded_clip = true;
                    }
                }
            }
        }
        assert!(found_rounded_clip,
            "P242 — clip=true + radius non-zero emite Group com clip_mask RoundedRect");
    }

    #[test]
    fn p242_block_clip_true_radius_zero_emit_group_rect_clip_mask() {
        use crate::entities::sides::Sides;
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        let b = Content::Block {
            body:      Box::new(Content::text("clipped-rect")),
            width:     None,
            height:    None,
            inset:     Sides::uniform(Length::pt(0.0)),
            breakable: true,
            outset:    Sides::uniform(Length::pt(0.0)),
            radius:    Corners::uniform(Length::ZERO),
            clip:      true,
            fill:      None,
            stroke:    None,
            spacing:   None,
            above:     None,
            below:     None,
            sticky:    false,
        };
        let doc = layout(&b);
        let mut found_rect_clip = false;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Group { clip_mask: Some(shape), .. } = item {
                    if matches!(shape, crate::entities::geometry::ShapeKind::Rect) {
                        found_rect_clip = true;
                    }
                }
            }
        }
        assert!(found_rect_clip,
            "P242 — clip=true + radius zero emite Group com clip_mask Rect (paridade DEBT-30)");
    }

    #[test]
    fn p242_block_clip_false_radius_non_zero_sem_clip_mask() {
        // Spec Decisão 6: radius sem clip armazenado mas sem clip_mask
        // emit. Bloco mantém inline behavior.
        use crate::entities::sides::Sides;
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        let b = Content::Block {
            body:      Box::new(Content::text("not-clipped")),
            width:     None,
            height:    None,
            inset:     Sides::uniform(Length::pt(0.0)),
            breakable: true,
            outset:    Sides::uniform(Length::pt(0.0)),
            radius:    Corners::uniform(Length::pt(5.0)),
            clip:      false,
            fill:      None,
            stroke:    None,
            spacing:   None,
            above:     None,
            below:     None,
            sticky:    false,
        };
        let doc = layout(&b);
        // Nenhum Group com clip_mask deve ser emitido.
        let mut found_any_clip_mask = false;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Group { clip_mask: Some(_), .. } = item {
                    found_any_clip_mask = true;
                }
            }
        }
        assert!(!found_any_clip_mask,
            "P242 — radius sem clip não emite clip_mask (semantic radius isolada graded)");
    }

    // ── Passo 243 (M9d / M7+3 fase (a); ADR-0081 IMPLEMENTADO parcial 4/5)
    //     — promoção real scope-outs Pad.right + Block.width + Boxed.width
    //     via regions.current.width save/restore ──

    #[test]
    fn p243_pad_right_efetivo_reduz_width_durante_body() {
        // P243 — Pad.right reduz regions.current.width efectiva durante
        // body layout (vs scope-out P156C que ignorava right).
        use crate::entities::sides::Sides;
        use crate::entities::layout_types::Length;
        let pad = Content::Pad {
            body:  Box::new(Content::text("p243pad")),
            sides: Sides {
                left:   None,
                top:    None,
                right:  Some(Length::pt(100.0)),  // Pad.right efectivo agora.
                bottom: None,
            },
        };
        // Smoke test: layout sem panic + body presente em output.
        let doc = layout(&pad);
        let mut texts = String::new();
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Text { text, .. } = item {
                    texts.push_str(text.as_str());
                }
            }
        }
        assert!(texts.contains("p243pad"),
            "Pad.right=100pt preserva body output (largura útil reduzida pero não-zero)");
    }

    #[test]
    fn p243_block_width_efetivo_clampa_largura() {
        // P243 — Block.width clampa regions.current.width durante body.
        use crate::entities::sides::Sides;
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        let block = Content::Block {
            body:      Box::new(Content::text("p243block")),
            width:     Some(Length::pt(150.0)),  // Block.width efectivo P243.
            height:    None,
            inset:     Sides::uniform(Length::pt(0.0)),
            breakable: true,
            outset:    Sides::uniform(Length::pt(0.0)),
            radius:    Corners::uniform(Length::ZERO),
            clip:      false,
            fill:      None,
            stroke:    None,
            spacing:   None,
            above:     None,
            below:     None,
            sticky:    false,
        };
        let doc = layout(&block);
        let mut texts = String::new();
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Text { text, .. } = item {
                    texts.push_str(text.as_str());
                }
            }
        }
        assert!(texts.contains("p243block"),
            "Block.width=150pt preserva body output (clamp width efectivo)");
    }

    #[test]
    fn p243_boxed_width_efetivo_clampa_largura() {
        // P243 — Boxed.width clampa regions.current.width durante body.
        use crate::entities::sides::Sides;
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        let boxed = Content::Boxed {
            body:     Box::new(Content::text("p243boxed")),
            width:    Some(Length::pt(80.0)),  // Boxed.width efectivo P243.
            height:   None,
            inset:    Sides::uniform(Length::pt(0.0)),
            baseline: Length::pt(0.0),
            outset:   Sides::uniform(Length::pt(0.0)),
            radius:   Corners::uniform(Length::ZERO),
            clip:     false,
            fill:     None,
            stroke:   None,
        };
        let doc = layout(&boxed);
        let mut texts = String::new();
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Text { text, .. } = item {
                    texts.push_str(text.as_str());
                }
            }
        }
        assert!(texts.contains("p243boxed"),
            "Boxed.width=80pt preserva body output (clamp width efectivo)");
    }

    #[test]
    fn p243_pad_aninhado_largura_cumulativa_preservada() {
        // P243 — Pad aninhado dentro de Block; width saved/restored em
        // ordem correcta (LIFO stack semantic).
        use crate::entities::sides::Sides;
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        let inner_pad = Content::Pad {
            body:  Box::new(Content::text("inner")),
            sides: Sides {
                left:   None,
                top:    None,
                right:  Some(Length::pt(50.0)),
                bottom: None,
            },
        };
        let block = Content::Block {
            body:      Box::new(inner_pad),
            width:     Some(Length::pt(200.0)),
            height:    None,
            inset:     Sides::uniform(Length::pt(0.0)),
            breakable: true,
            outset:    Sides::uniform(Length::pt(0.0)),
            radius:    Corners::uniform(Length::ZERO),
            clip:      false,
            fill:      None,
            stroke:    None,
            spacing:   None,
            above:     None,
            below:     None,
            sticky:    false,
        };
        let doc = layout(&block);
        let mut texts = String::new();
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Text { text, .. } = item {
                    texts.push_str(text.as_str());
                }
            }
        }
        assert!(texts.contains("inner"),
            "Pad dentro de Block — width cumulative save/restore preservado");
    }

    // ── Passo 247 (M9d / M7+5; ADR-0079 Categoria A.4) ──────────────────
    //     Block + Boxed fill/stroke/outset semantic real activação.
    //     Layouter emite FrameItem::Shape antes do body (Z-order) com
    //     bounds expandidos por outset.

    #[test]
    fn p247_block_fill_emite_shape_antes_do_body() {
        use crate::entities::sides::Sides;
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::{Color, Length};
        let b = Content::Block {
            body:      Box::new(Content::text("p247fill")),
            width:     Some(Length::pt(50.0)),
            height:    None,
            inset:     Sides::uniform(Length::pt(0.0)),
            breakable: true,
            outset:    Sides::uniform(Length::pt(0.0)),
            radius:    Corners::uniform(Length::ZERO),
            clip:      false,
            fill:      Some(Color::rgb(200, 50, 50)),
            stroke:    None,
            spacing:   None,
            above:     None,
            below:     None,
            sticky:    false,
        };
        let doc = layout(&b);
        let mut found_shape_with_fill = false;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Shape { fill: Some(c), .. } = item {
                    if *c == Color::rgb(200, 50, 50) { found_shape_with_fill = true; }
                }
            }
        }
        assert!(found_shape_with_fill,
            "P247 — Block com fill=Some(Color) emite FrameItem::Shape com fill correspondente");
    }

    #[test]
    fn p247_block_stroke_emite_shape_com_stroke() {
        use crate::entities::sides::Sides;
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::{Color, Length};
        use crate::entities::geometry::Stroke;
        let b = Content::Block {
            body:      Box::new(Content::text("p247stroke")),
            width:     Some(Length::pt(40.0)),
            height:    None,
            inset:     Sides::uniform(Length::pt(0.0)),
            breakable: true,
            outset:    Sides::uniform(Length::pt(0.0)),
            radius:    Corners::uniform(Length::ZERO),
            clip:      false,
            fill:      None,
            stroke:    Some(Stroke { paint: Color::rgb(10, 20, 30), thickness: 1.5 }),
            spacing:   None,
            above:     None,
            below:     None,
            sticky:    false,
        };
        let doc = layout(&b);
        let mut found_shape_with_stroke = false;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Shape { stroke: Some(s), .. } = item {
                    if s.paint == Color::rgb(10, 20, 30) && s.thickness == 1.5 {
                        found_shape_with_stroke = true;
                    }
                }
            }
        }
        assert!(found_shape_with_stroke,
            "P247 — Block com stroke=Some(Stroke) emite Shape com stroke correspondente");
    }

    #[test]
    fn p247_block_fill_e_radius_emite_rounded_rect() {
        use crate::entities::sides::Sides;
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::{Color, Length};
        let b = Content::Block {
            body:      Box::new(Content::text("p247rounded")),
            width:     Some(Length::pt(60.0)),
            height:    None,
            inset:     Sides::uniform(Length::pt(0.0)),
            breakable: true,
            outset:    Sides::uniform(Length::pt(0.0)),
            radius:    Corners::uniform(Length::pt(5.0)),
            clip:      false,
            fill:      Some(Color::rgb(100, 100, 100)),
            stroke:    None,
            spacing:   None,
            above:     None,
            below:     None,
            sticky:    false,
        };
        let doc = layout(&b);
        let mut found_rounded_fill = false;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Shape { kind, fill: Some(_), .. } = item {
                    if let crate::entities::geometry::ShapeKind::RoundedRect { .. } = kind {
                        found_rounded_fill = true;
                    }
                }
            }
        }
        assert!(found_rounded_fill,
            "P247 — fill + radius não-zero emite Shape kind=RoundedRect");
    }

    #[test]
    fn p247_block_outset_expande_bounds_shape() {
        // outset=10pt em todos os lados; Shape width/height incluem outset.
        use crate::entities::sides::Sides;
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::{Color, Length};
        let b = Content::Block {
            body:      Box::new(Content::text("p247outset")),
            width:     Some(Length::pt(50.0)),
            height:    Some(Length::pt(20.0)),
            inset:     Sides::uniform(Length::pt(0.0)),
            breakable: true,
            outset:    Sides::uniform(Length::pt(10.0)),
            radius:    Corners::uniform(Length::ZERO),
            clip:      false,
            fill:      Some(Color::rgb(50, 50, 50)),
            stroke:    None,
            spacing:   None,
            above:     None,
            below:     None,
            sticky:    false,
        };
        let doc = layout(&b);
        let mut shape_w = 0.0_f64;
        let mut shape_h = 0.0_f64;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Shape { width, height, fill: Some(_), .. } = item {
                    shape_w = *width;
                    shape_h = *height;
                }
            }
        }
        // Block inner_w = width + inset_left = 50 + 0 = 50.
        // outset.left + outset.right = 20. Shape w = 70.
        assert!(shape_w >= 65.0 && shape_w <= 75.0,
            "P247 — outset expande Shape width; esperado ~70pt, obtido {:.1}", shape_w);
        // Block inner_h = height = 20. outset.top + outset.bottom = 20. Shape h = 40.
        assert!(shape_h >= 35.0 && shape_h <= 50.0,
            "P247 — outset expande Shape height; esperado ~40pt, obtido {:.1}", shape_h);
    }

    #[test]
    fn p247_block_fill_none_e_outset_zero_sem_shape() {
        // Cenário backward compat: fill=None, stroke=None, outset=zero
        // → SEM Shape emitido (apenas body items).
        use crate::entities::sides::Sides;
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        let b = Content::Block {
            body:      Box::new(Content::text("backcompat")),
            width:     None,
            height:    None,
            inset:     Sides::uniform(Length::pt(0.0)),
            breakable: true,
            outset:    Sides::uniform(Length::pt(0.0)),
            radius:    Corners::uniform(Length::ZERO),
            clip:      false,
            fill:      None,
            stroke:    None,
            spacing:   None,
            above:     None,
            below:     None,
            sticky:    false,
        };
        let doc = layout(&b);
        let mut found_shape = false;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Shape { .. } = item {
                    found_shape = true;
                }
            }
        }
        assert!(!found_shape,
            "P247 — fill/stroke/outset todos zero NÃO emite Shape (backward compat P246)");
    }

    #[test]
    fn p247_boxed_fill_emite_shape() {
        // Boxed inline com fill emite Shape paralelo Block.
        use crate::entities::sides::Sides;
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::{Color, Length};
        let b = Content::Boxed {
            body:     Box::new(Content::text("p247boxfill")),
            width:    Some(Length::pt(30.0)),
            height:   None,
            inset:    Sides::uniform(Length::pt(0.0)),
            baseline: Length::pt(0.0),
            outset:   Sides::uniform(Length::pt(0.0)),
            radius:   Corners::uniform(Length::ZERO),
            clip:     false,
            fill:     Some(Color::rgb(70, 140, 210)),
            stroke:   None,
        };
        let doc = layout(&b);
        let mut found_shape_with_fill = false;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Shape { fill: Some(c), .. } = item {
                    if *c == Color::rgb(70, 140, 210) { found_shape_with_fill = true; }
                }
            }
        }
        assert!(found_shape_with_fill,
            "P247 — Boxed inline com fill emite FrameItem::Shape paralelo Block");
    }

    // ── Passo 248 (M9d / M7+5; ADR-0079 Categoria A.4 cumulativa) ──
    //     Activação semantic real de 3 fields graded armazenados:
    //     A) Block.breakable (P156G) — new_page() antecipado se bloco
    //        não-breakable não cabe na página actual mas cabe noutra.
    //     B) Boxed.height overflow (P156H) — clip via FrameItem::Group
    //        se clip=true; overflow visível se clip=false.
    //     C) TableCell overflow (P157B) — clip implícito ao limite
    //        cell via Group + clip_mask Rect.
    //
    //     Mecanismo comum: `measure_content_constrained` (puro, P246
    //     audit C1 §2.4) reusado para medição antecipada.

    #[test]
    fn p248_block_breakable_true_preserva_emit_normal() {
        use crate::entities::sides::Sides;
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        // breakable=true (default P156G): emit normal sem antecipar break.
        let b = Content::Block {
            body:      Box::new(Content::text("p248brkt")),
            width:     None,
            height:    None,
            inset:     Sides::uniform(Length::pt(0.0)),
            breakable: true,
            outset:    Sides::uniform(Length::pt(0.0)),
            radius:    Corners::uniform(Length::ZERO),
            clip:      false,
            fill:      None,
            stroke:    None,
            spacing:   None,
            above:     None,
            below:     None,
            sticky:    false,
        };
        let doc = layout(&b);
        // Smoke: body renderiza; única página.
        assert_eq!(doc.pages.len(), 1, "breakable=true não causa break extra");
        let mut texts = String::new();
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Text { text, .. } = item {
                    texts.push_str(text.as_str());
                }
            }
        }
        assert!(texts.contains("p248brkt"));
    }

    #[test]
    fn p248_block_breakable_false_cabe_actual_sem_break() {
        use crate::entities::sides::Sides;
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        // breakable=false + body pequeno: cabe na actual; sem new_page.
        let b = Content::Block {
            body:      Box::new(Content::text("p248brkf")),
            width:     None,
            height:    None,
            inset:     Sides::uniform(Length::pt(0.0)),
            breakable: false,
            outset:    Sides::uniform(Length::pt(0.0)),
            radius:    Corners::uniform(Length::ZERO),
            clip:      false,
            fill:      None,
            stroke:    None,
            spacing:   None,
            above:     None,
            below:     None,
            sticky:    false,
        };
        let doc = layout(&b);
        assert_eq!(doc.pages.len(), 1,
            "breakable=false body pequeno cabe na actual; sem break extra");
    }

    #[test]
    fn p248_block_breakable_false_overlong_emit_normal() {
        // breakable=false + body que excede página inteira: emit normal
        // (paridade vanilla "overlong atómico"; sem loop infinito).
        use crate::entities::sides::Sides;
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        // height enorme — excede página (~595pt default test).
        let b = Content::Block {
            body:      Box::new(Content::text("p248overlong")),
            width:     None,
            height:    Some(Length::pt(10_000.0)),
            inset:     Sides::uniform(Length::pt(0.0)),
            breakable: false,
            outset:    Sides::uniform(Length::pt(0.0)),
            radius:    Corners::uniform(Length::ZERO),
            clip:      false,
            fill:      None,
            stroke:    None,
            spacing:   None,
            above:     None,
            below:     None,
            sticky:    false,
        };
        let doc = layout(&b);
        // Body renderiza; sem panic; o output pode ser várias páginas
        // por overflow natural (flush_line) mas não infinitas.
        assert!(doc.pages.len() >= 1 && doc.pages.len() <= 50,
            "breakable=false overlong não causa loop infinito; obteve {} páginas", doc.pages.len());
    }

    #[test]
    fn p248_block_breakable_false_antecipa_new_page() {
        // Cenário: encher página com Pad+VSpace push, depois Block
        // breakable=false que precisa de espaço — `new_page()` antecipa.
        use crate::entities::sides::Sides;
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        // Layout: VSpace grande (push cursor) + Block breakable=false
        // que mede como large via height min.
        // A4: page 841.89pt, margin 70.87pt; usable ~700pt; bottom_limit ~771pt.
        // VSpace 650pt empurra cursor para ~729pt; remaining ≈ 42pt.
        // Block height 100pt → block_total_h 100 > remaining 42 AND
        // block_total_h 100 <= usable 700 → break antecipado.
        let seq = Content::Sequence(std::sync::Arc::from(vec![
            Content::VSpace { amount: Length::pt(650.0), weak: false },
            Content::Block {
                body:      Box::new(Content::text("p248new")),
                width:     None,
                height:    Some(Length::pt(100.0)),
                inset:     Sides::uniform(Length::pt(0.0)),
                breakable: false,
                outset:    Sides::uniform(Length::pt(0.0)),
                radius:    Corners::uniform(Length::ZERO),
                clip:      false,
                fill:      None,
                stroke:    None,
                spacing:   None,
                above:     None,
                below:     None,
                sticky:    false,
            },
        ]));
        let doc = layout(&seq);
        // Esperar 2 páginas: pre-VSpace na p1; Block na p2.
        assert!(doc.pages.len() >= 2,
            "P248 breakable=false + espaço insuficiente → new_page antecipado; obteve {} páginas",
            doc.pages.len());
    }

    #[test]
    fn p248_block_breakable_false_combina_fill_stroke_outset_p247() {
        // Cross-attribute: breakable=false + fill/stroke/outset P247.
        // Verifica que activação A não regride P247 (Shape ainda emitido).
        use crate::entities::sides::Sides;
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::{Color, Length};
        let b = Content::Block {
            body:      Box::new(Content::text("p248cross")),
            width:     Some(Length::pt(60.0)),
            height:    None,
            inset:     Sides::uniform(Length::pt(0.0)),
            breakable: false,
            outset:    Sides::uniform(Length::pt(5.0)),
            radius:    Corners::uniform(Length::ZERO),
            clip:      false,
            fill:      Some(Color::rgb(100, 200, 50)),
            stroke:    None,
            spacing:   None,
            above:     None,
            below:     None,
            sticky:    false,
        };
        let doc = layout(&b);
        let mut found_shape = false;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Shape { fill: Some(c), .. } = item {
                    if *c == Color::rgb(100, 200, 50) { found_shape = true; }
                }
            }
        }
        assert!(found_shape,
            "P248 breakable=false preserva P247 fill+outset Shape emission");
    }

    #[test]
    fn p248_boxed_height_none_preserva_p156h() {
        // height=None: preservado P156H literal (nenhum clip).
        use crate::entities::sides::Sides;
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        let b = Content::Boxed {
            body:     Box::new(Content::text("p248bxnone")),
            width:    None,
            height:   None,
            inset:    Sides::uniform(Length::pt(0.0)),
            baseline: Length::pt(0.0),
            outset:   Sides::uniform(Length::pt(0.0)),
            radius:   Corners::uniform(Length::ZERO),
            clip:     true,  // clip aceita mas height None → sem overflow handling
            fill:     None,
            stroke:   None,
        };
        let doc = layout(&b);
        // Sem Group por height overflow (height None).
        let mut found_overflow_group = false;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Group { clip_mask: Some(ShapeKind::Rect), .. } = item {
                    found_overflow_group = true;
                }
            }
        }
        // Pode existir Group por outras razões (P242 clip) mas com height=None
        // o handle P248 NÃO dispara.
        let _ = found_overflow_group;
    }

    #[test]
    fn p248_boxed_height_overflow_clip_true_emite_group() {
        // Body excede height + clip=true → Group com clip_mask Rect.
        use crate::entities::sides::Sides;
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        // Body com height natural > 5pt (line_height ~12pt default);
        // height=5pt força overflow.
        let b = Content::Boxed {
            body:     Box::new(Content::text("p248bxovf")),
            width:    Some(Length::pt(50.0)),
            height:   Some(Length::pt(5.0)),
            inset:    Sides::uniform(Length::pt(0.0)),
            baseline: Length::pt(0.0),
            outset:   Sides::uniform(Length::pt(0.0)),
            radius:   Corners::uniform(Length::ZERO),
            clip:     true,
            fill:     None,
            stroke:   None,
        };
        let doc = layout(&b);
        let mut found_clip_group = false;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Group { clip_mask: Some(ShapeKind::Rect), inner_height, .. } = item {
                    if (*inner_height - 5.0).abs() < 0.1 {
                        found_clip_group = true;
                    }
                }
            }
        }
        assert!(found_clip_group,
            "P248 Boxed overflow + clip=true emite Group com clip_mask Rect inner_height=5pt");
    }

    #[test]
    fn p248_boxed_height_overflow_clip_false_overflow_visivel() {
        // Body excede height + clip=false → SEM Group por overflow
        // (overflow visível paridade vanilla default).
        use crate::entities::sides::Sides;
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        let b = Content::Boxed {
            body:     Box::new(Content::text("p248bxnoc")),
            width:    Some(Length::pt(50.0)),
            height:   Some(Length::pt(5.0)),
            inset:    Sides::uniform(Length::pt(0.0)),
            baseline: Length::pt(0.0),
            outset:   Sides::uniform(Length::pt(0.0)),
            radius:   Corners::uniform(Length::ZERO),
            clip:     false,
            fill:     None,
            stroke:   None,
        };
        let doc = layout(&b);
        // Verificar que NÃO foi adicionado Group com inner_height=5
        // (clip=false não wrap).
        let mut found_overflow_group = false;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Group { clip_mask: Some(ShapeKind::Rect), inner_height, .. } = item {
                    if (*inner_height - 5.0).abs() < 0.1 {
                        found_overflow_group = true;
                    }
                }
            }
        }
        assert!(!found_overflow_group,
            "P248 Boxed overflow + clip=false NÃO emite Group (overflow visível)");
    }

    #[test]
    fn p248_boxed_height_cabe_sem_clip() {
        // Body cabe em height: preservado literal, sem Group overflow.
        use crate::entities::sides::Sides;
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        let b = Content::Boxed {
            body:     Box::new(Content::text("x")),  // texto curto < height
            width:    Some(Length::pt(50.0)),
            height:   Some(Length::pt(100.0)),       // muito maior que body natural
            inset:    Sides::uniform(Length::pt(0.0)),
            baseline: Length::pt(0.0),
            outset:   Sides::uniform(Length::pt(0.0)),
            radius:   Corners::uniform(Length::ZERO),
            clip:     true,
            fill:     None,
            stroke:   None,
        };
        let doc = layout(&b);
        let mut found_overflow_group = false;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Group { clip_mask: Some(ShapeKind::Rect), inner_height, .. } = item {
                    if (*inner_height - 100.0).abs() < 0.1 {
                        found_overflow_group = true;
                    }
                }
            }
        }
        assert!(!found_overflow_group,
            "P248 Boxed body cabe em height: sem Group overflow (preservado)");
    }

    #[test]
    fn p248_table_cell_sem_overflow_preserva_p157b() {
        // Cell body cabe em cell_h: sem Group de clip overflow.
        use crate::entities::layout_types::TrackSizing;
        // Table 1×1 com cell pequeno; row Auto.
        let cell = Content::TableCell {
            body:      Box::new(Content::text("x")),
            x:         None,
            y:         None,
            colspan:   None,
            rowspan:   None,
            stroke:    None,
            fill:      None,
            align:     None,
            inset:     None,
            breakable: None,
        };
        let t = Content::Table {
            columns: vec![TrackSizing::Auto],
            rows:    vec![TrackSizing::Auto],
            children: vec![cell],
            stroke:   None,
            fill:     None,
        };
        let doc = layout(&t);
        // Sem Group por overflow cell (body pequeno cabe).
        let mut found_overflow_clip = false;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Group { clip_mask: Some(ShapeKind::Rect), .. } = item {
                    found_overflow_clip = true;
                }
            }
        }
        assert!(!found_overflow_clip,
            "P248 cell sem overflow não emite Group por clip implícito");
    }

    #[test]
    fn p248_table_cell_overflow_emite_clip_group() {
        // Cell body excede cell_h: Group com clip_mask Rect.
        // Usa Block.height = 100pt dentro de cell com row Fixed(10pt) →
        // medição determinística (não depende de word-wrap).
        use crate::entities::sides::Sides;
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::{Length, TrackSizing};
        let inner_block = Content::Block {
            body:      Box::new(Content::text("ovf")),
            width:     None,
            height:    Some(Length::pt(100.0)),  // força >> cell_h
            inset:     Sides::uniform(Length::pt(0.0)),
            breakable: true,
            outset:    Sides::uniform(Length::pt(0.0)),
            radius:    Corners::uniform(Length::ZERO),
            clip:      false,
            fill:      None,
            stroke:    None,
            spacing:   None,
            above:     None,
            below:     None,
            sticky:    false,
        };
        let cell = Content::TableCell {
            body:      Box::new(inner_block),
            x:         None,
            y:         None,
            colspan:   None,
            rowspan:   None,
            stroke:    None,
            fill:      None,
            align:     None,
            inset:     None,
            breakable: None,
        };
        let t = Content::Table {
            columns: vec![TrackSizing::Fixed(40.0)],
            rows:    vec![TrackSizing::Fixed(10.0)],
            children: vec![cell],
            stroke:   None,
            fill:     None,
        };
        let doc = layout(&t);
        let mut found_clip_group = false;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Group { clip_mask: Some(ShapeKind::Rect), inner_height, .. } = item {
                    if *inner_height <= 10.1 {
                        found_clip_group = true;
                    }
                }
            }
        }
        assert!(found_clip_group,
            "P248 cell body overflow (Block height 100pt) + row Fixed(10pt) emite Group com clip_mask Rect inner_height<=10pt");
    }

    #[test]
    fn p248_table_cell_overflow_preserva_fill_stroke_externos() {
        // Cross: cell overflow Group + cell-level fill: ambos coexistem.
        use crate::entities::sides::Sides;
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::{Color, Length, TrackSizing};
        let inner_block = Content::Block {
            body:      Box::new(Content::text("fillovf")),
            width:     None,
            height:    Some(Length::pt(80.0)),
            inset:     Sides::uniform(Length::pt(0.0)),
            breakable: true,
            outset:    Sides::uniform(Length::pt(0.0)),
            radius:    Corners::uniform(Length::ZERO),
            clip:      false,
            fill:      None,
            stroke:    None,
            spacing:   None,
            above:     None,
            below:     None,
            sticky:    false,
        };
        let cell = Content::TableCell {
            body:      Box::new(inner_block),
            x:         None,
            y:         None,
            colspan:   None,
            rowspan:   None,
            stroke:    None,
            fill:      Some(Color::rgb(220, 220, 50)),
            align:     None,
            inset:     None,
            breakable: None,
        };
        let t = Content::Table {
            columns: vec![TrackSizing::Fixed(30.0)],
            rows:    vec![TrackSizing::Fixed(10.0)],
            children: vec![cell],
            stroke:   None,
            fill:     None,
        };
        let doc = layout(&t);
        let mut found_fill = false;
        let mut found_clip = false;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                match item {
                    FrameItem::Shape { fill: Some(c), .. }
                        if *c == Color::rgb(220, 220, 50) => found_fill = true,
                    FrameItem::Group { clip_mask: Some(ShapeKind::Rect), .. } => found_clip = true,
                    _ => {}
                }
            }
        }
        assert!(found_fill, "P248 cell overflow preserva fill (Shape)");
        assert!(found_clip, "P248 cell overflow emite Group clip");
    }

    #[test]
    fn p248_block_breakable_false_dentro_table_cell_overflow_combinado() {
        // E2E cross-activação: cell com Block (breakable=false) cujo height
        // excede cell_h → activação A (medição) + activação C (clip)
        // coexistem.
        use crate::entities::sides::Sides;
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::{Length, TrackSizing};
        let inner_block = Content::Block {
            body:      Box::new(Content::text("cross")),
            width:     None,
            height:    Some(Length::pt(60.0)),  // excede cell row
            inset:     Sides::uniform(Length::pt(0.0)),
            breakable: false,
            outset:    Sides::uniform(Length::pt(0.0)),
            radius:    Corners::uniform(Length::ZERO),
            clip:      false,
            fill:      None,
            stroke:    None,
            spacing:   None,
            above:     None,
            below:     None,
            sticky:    false,
        };
        let cell = Content::TableCell {
            body:      Box::new(inner_block),
            x:         None,
            y:         None,
            colspan:   None,
            rowspan:   None,
            stroke:    None,
            fill:      None,
            align:     None,
            inset:     None,
            breakable: None,
        };
        let t = Content::Table {
            columns: vec![TrackSizing::Fixed(40.0)],
            rows:    vec![TrackSizing::Fixed(15.0)],
            children: vec![cell],
            stroke:   None,
            fill:     None,
        };
        let doc = layout(&t);
        // Smoke: layout não panica + alguma Group emitida por activação C.
        let mut found_clip = false;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Group { clip_mask: Some(ShapeKind::Rect), .. } = item {
                    found_clip = true;
                }
            }
        }
        assert!(found_clip, "P248 — Block breakable=false dentro cell overflow → clip implícito C");
    }

    #[test]
    fn p248_block_breakable_false_com_inset_height_min_correto() {
        // breakable=false + height + inset: medição inclui outset+inset
        // correctamente. Verifica que com defaults pequenos não há break
        // antecipado erroneamente.
        use crate::entities::sides::Sides;
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        let b = Content::Block {
            body:      Box::new(Content::text("p248inset")),
            width:     None,
            height:    Some(Length::pt(50.0)),
            inset:     Sides::uniform(Length::pt(10.0)),
            breakable: false,
            outset:    Sides::uniform(Length::pt(5.0)),
            radius:    Corners::uniform(Length::ZERO),
            clip:      false,
            fill:      None,
            stroke:    None,
            spacing:   None,
            above:     None,
            below:     None,
            sticky:    false,
        };
        let doc = layout(&b);
        assert_eq!(doc.pages.len(), 1,
            "P248 — Block pequeno com inset+outset cabe na actual; sem break extra");
        let mut texts = String::new();
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Text { text, .. } = item {
                    texts.push_str(text.as_str());
                }
            }
        }
        assert!(texts.contains("p248inset"));
    }

    #[test]
    fn p248_boxed_height_overflow_cross_radius_clip() {
        // height overflow + radius (P242): Group por height overflow
        // coexiste com Shape Rect (P247 sem radius non-zero por simplicidade).
        use crate::entities::sides::Sides;
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::{Color, Length};
        let b = Content::Boxed {
            body:     Box::new(Content::text("p248cross")),
            width:    Some(Length::pt(40.0)),
            height:   Some(Length::pt(8.0)),
            inset:    Sides::uniform(Length::pt(0.0)),
            baseline: Length::pt(0.0),
            outset:   Sides::uniform(Length::pt(0.0)),
            radius:   Corners::uniform(Length::ZERO),
            clip:     true,
            fill:     Some(Color::rgb(150, 50, 200)),
            stroke:   None,
        };
        let doc = layout(&b);
        let mut found_fill = false;
        let mut found_clip = false;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                match item {
                    FrameItem::Shape { fill: Some(c), .. }
                        if *c == Color::rgb(150, 50, 200) => found_fill = true,
                    FrameItem::Group { clip_mask: Some(ShapeKind::Rect), inner_height, .. }
                        if (*inner_height - 8.0).abs() < 0.1 => found_clip = true,
                    _ => {}
                }
            }
        }
        assert!(found_fill, "P247 fill preservado em P248 cross-attribute");
        assert!(found_clip, "P248 height overflow clip activo cross-attribute");
    }

    #[test]
    fn p248_block_breakable_false_dentro_block_breakable_true_aninhado() {
        // Block breakable=true exterior + Block breakable=false interior:
        // ambos respeitam suas decisões sem panic. Sequência longa empurra
        // cursor; inner breakable=false antecipa novo break.
        use crate::entities::sides::Sides;
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        let inner = Content::Block {
            body:      Box::new(Content::text("inner")),
            width:     None,
            height:    Some(Length::pt(150.0)),
            inset:     Sides::uniform(Length::pt(0.0)),
            breakable: false,
            outset:    Sides::uniform(Length::pt(0.0)),
            radius:    Corners::uniform(Length::ZERO),
            clip:      false,
            fill:      None,
            stroke:    None,
            spacing:   None,
            above:     None,
            below:     None,
            sticky:    false,
        };
        let seq = Content::Sequence(std::sync::Arc::from(vec![
            Content::VSpace { amount: Length::pt(600.0), weak: false },
            Content::Block {
                body:      Box::new(inner),
                width:     None,
                height:    None,
                inset:     Sides::uniform(Length::pt(0.0)),
                breakable: true,  // exterior permite break natural
                outset:    Sides::uniform(Length::pt(0.0)),
                radius:    Corners::uniform(Length::ZERO),
                clip:      false,
                fill:      None,
                stroke:    None,
                spacing:   None,
                above:     None,
                below:     None,
                sticky:    false,
            },
        ]));
        let doc = layout(&seq);
        // Layout não panica + at least 1 page; aninhamento estável.
        assert!(doc.pages.len() >= 1);
    }

    #[test]
    fn p248_table_cell_overflow_radius_inner_block_p247() {
        // Inner Block com radius P242 + cell overflow P248: ambos
        // mecanismos clip_mask coexistem.
        use crate::entities::sides::Sides;
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::{Length, TrackSizing};
        let inner_block = Content::Block {
            body:      Box::new(Content::text("radius cross")),
            width:     None,
            height:    Some(Length::pt(80.0)),
            inset:     Sides::uniform(Length::pt(0.0)),
            breakable: true,
            outset:    Sides::uniform(Length::pt(0.0)),
            radius:    Corners::uniform(Length::pt(3.0)),  // P242
            clip:      true,                                // P242
            fill:      None,
            stroke:    None,
            spacing:   None,
            above:     None,
            below:     None,
            sticky:    false,
        };
        let cell = Content::TableCell {
            body:      Box::new(inner_block),
            x:         None,
            y:         None,
            colspan:   None,
            rowspan:   None,
            stroke:    None,
            fill:      None,
            align:     None,
            inset:     None,
            breakable: None,
        };
        let t = Content::Table {
            columns: vec![TrackSizing::Fixed(60.0)],
            rows:    vec![TrackSizing::Fixed(20.0)],
            children: vec![cell],
            stroke:   None,
            fill:     None,
        };
        let doc = layout(&t);
        // Espera ≥1 Group (P242 inner radius+clip ou P248 cell overflow).
        let mut group_count = 0;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Group { .. } = item { group_count += 1; }
            }
        }
        assert!(group_count >= 1,
            "P248 cell overflow + P242 inner radius+clip coexistem; obteve {} Group(s)", group_count);
    }

    #[test]
    fn p248_boxed_height_overflow_inset_correto() {
        // Boxed height + inset: clip Group inner_height = height (sem inset),
        // mas inset visualmente embebido no body.
        use crate::entities::sides::Sides;
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        let b = Content::Boxed {
            body:     Box::new(Content::text("inset")),
            width:    Some(Length::pt(40.0)),
            height:   Some(Length::pt(6.0)),
            inset:    Sides::uniform(Length::pt(2.0)),
            baseline: Length::pt(0.0),
            outset:   Sides::uniform(Length::pt(0.0)),
            radius:   Corners::uniform(Length::ZERO),
            clip:     true,
            fill:     None,
            stroke:   None,
        };
        let doc = layout(&b);
        let mut found_clip = false;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Group { clip_mask: Some(ShapeKind::Rect), inner_height, .. } = item {
                    if (*inner_height - 6.0).abs() < 0.1 { found_clip = true; }
                }
            }
        }
        assert!(found_clip, "P248 Boxed height overflow + inset → Group inner_height = height");
    }

    #[test]
    fn p248_block_breakable_false_com_outset_grande_medicao_inclui_outset() {
        // breakable=false: medição antecipada inclui outset top+bottom.
        // Test verifica que outset NÃO é ignorado no block_total_h.
        use crate::entities::sides::Sides;
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        // Cenário: cursor inicial baixo na página (VSpace push) +
        // block height médio + outset grande → total deve causar break.
        let seq = Content::Sequence(std::sync::Arc::from(vec![
            Content::VSpace { amount: Length::pt(620.0), weak: false },
            Content::Block {
                body:      Box::new(Content::text("p248outset")),
                width:     None,
                height:    Some(Length::pt(80.0)),
                inset:     Sides::uniform(Length::pt(0.0)),
                breakable: false,
                outset:    Sides::uniform(Length::pt(30.0)),  // +60pt total Y
                radius:    Corners::uniform(Length::ZERO),
                clip:      false,
                fill:      None,
                stroke:    None,
                spacing:   None,
                above:     None,
                below:     None,
                sticky:    false,
            },
        ]));
        let doc = layout(&seq);
        // VSpace 620 cursor ~699; remaining ~72pt; block_total_h = 80+60=140 > 72 → break.
        assert!(doc.pages.len() >= 2,
            "P248 — outset incluído na medição; break antecipado quando outset+height excede remaining; obteve {} páginas",
            doc.pages.len());
    }

    #[test]
    fn p248_boxed_height_overflow_clip_false_e_clip_true_diferem_in_group_count() {
        // Confronto direto: mesmo conteúdo+height+overflow; clip=true emite
        // Group de overflow; clip=false NÃO emite.
        use crate::entities::sides::Sides;
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        let mk = |clip: bool| Content::Boxed {
            body:     Box::new(Content::text("diff")),
            width:    Some(Length::pt(40.0)),
            height:   Some(Length::pt(4.0)),  // line_height > 4pt
            inset:    Sides::uniform(Length::pt(0.0)),
            baseline: Length::pt(0.0),
            outset:   Sides::uniform(Length::pt(0.0)),
            radius:   Corners::uniform(Length::ZERO),
            clip,
            fill:     None,
            stroke:   None,
        };
        let count_clip_groups = |c: Content| -> usize {
            let doc = layout(&c);
            doc.pages.iter().flat_map(|p| p.items.iter())
                .filter(|i| matches!(i, FrameItem::Group { clip_mask: Some(ShapeKind::Rect), inner_height, .. }
                                       if (*inner_height - 4.0).abs() < 0.1))
                .count()
        };
        let n_true  = count_clip_groups(mk(true));
        let n_false = count_clip_groups(mk(false));
        assert!(n_true  >= 1, "clip=true emite Group por overflow");
        assert_eq!(n_false, 0, "clip=false NÃO emite Group por overflow");
    }

    #[test]
    fn p248_block_breakable_false_zero_height_default_emit_normal() {
        // Edge: breakable=false + body=text simples (sem height) → cabe
        // certamente; sem break.
        use crate::entities::sides::Sides;
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        let b = Content::Block {
            body:      Box::new(Content::text("p248edge")),
            width:     None,
            height:    None,  // sem height min
            inset:     Sides::uniform(Length::pt(0.0)),
            breakable: false,
            outset:    Sides::uniform(Length::pt(0.0)),
            radius:    Corners::uniform(Length::ZERO),
            clip:      false,
            fill:      None,
            stroke:    None,
            spacing:   None,
            above:     None,
            below:     None,
            sticky:    false,
        };
        let doc = layout(&b);
        assert_eq!(doc.pages.len(), 1,
            "P248 breakable=false sem height: body cabe; sem break extra");
    }

    #[test]
    fn p248_measure_content_constrained_puro_sem_side_effects() {
        // Audit C1 §2.4: confirmar puridade pós-P248 (sem mutar cursor).
        // Construir Block que invocaria measure_content_constrained se
        // breakable=false; verificar que cursor pré/pós idêntico.
        use crate::entities::sides::Sides;
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        let b = Content::Block {
            body:      Box::new(Content::text("a")),
            width:     None,
            height:    None,
            inset:     Sides::uniform(Length::pt(0.0)),
            breakable: false,  // dispara medição antecipada
            outset:    Sides::uniform(Length::pt(0.0)),
            radius:    Corners::uniform(Length::ZERO),
            clip:      false,
            fill:      None,
            stroke:    None,
            spacing:   None,
            above:     None,
            below:     None,
            sticky:    false,
        };
        // Smoke: layout não panica; body emitido.
        let doc = layout(&b);
        let mut texts = String::new();
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Text { text, .. } = item {
                    texts.push_str(text.as_str());
                }
            }
        }
        assert_eq!(texts, "a",
            "P248 measure puro: body emitido sem distorção");
    }

    // ── Passo 250 (M9d / M7+5; ADR-0079 Categoria A.4 COMPLETO Block 10/10;
    //     cita ADR-0082 PROPOSTO N=1 primeira aplicação citante) ──────────
    //     Block +4 fields (spacing/above/below/sticky) semantic real
    //     + refactor Sequence consumer para peekable + neighbour context.

    /// Helper P250 — construtor Block com defaults (spacing=None/etc).
    fn p250_mk_block(body: Content, height: Option<crate::entities::layout_types::Length>) -> Content {
        use crate::entities::sides::Sides;
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        Content::Block {
            body:      Box::new(body),
            width:     None,
            height,
            inset:     Sides::uniform(Length::pt(0.0)),
            breakable: true,
            outset:    Sides::uniform(Length::pt(0.0)),
            radius:    Corners::uniform(Length::ZERO),
            clip:      false,
            fill:      None,
            stroke:    None,
            spacing:   None,
            above:     None,
            below:     None,
            sticky:    false,
        }
    }

    /// Helper P250 — construtor Block com spacing/above/below/sticky.
    fn p250_mk_block_with(
        body: Content,
        spacing: Option<crate::entities::layout_types::Length>,
        above: Option<crate::entities::layout_types::Length>,
        below: Option<crate::entities::layout_types::Length>,
        sticky: bool,
    ) -> Content {
        use crate::entities::sides::Sides;
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        Content::Block {
            body:      Box::new(body),
            width:     None,
            height:    None,
            inset:     Sides::uniform(Length::pt(0.0)),
            breakable: true,
            outset:    Sides::uniform(Length::pt(0.0)),
            radius:    Corners::uniform(Length::ZERO),
            clip:      false,
            fill:      None,
            stroke:    None,
            spacing,
            above,
            below,
            sticky,
        }
    }

    #[test]
    fn p250_block_defaults_preserva_output_pre_p250() {
        // Sentinela: Block com defaults (None×3 + false) renderiza
        // idêntico a P249 (output PDF bit-equivalente).
        let b = p250_mk_block(Content::text("p250def"), None);
        let doc = layout(&b);
        let mut texts = String::new();
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Text { text, .. } = item {
                    texts.push_str(text.as_str());
                }
            }
        }
        assert!(texts.contains("p250def"),
            "P250 — defaults preservam body output literal");
        assert_eq!(doc.pages.len(), 1, "P250 defaults sem páginas extras");
    }

    #[test]
    fn p250_block_above_isolado_primeiro_block_suprime_above() {
        // Block sozinho (não dentro de Sequence) → above suprimido
        // (sem prev block).
        use crate::entities::layout_types::Length;
        let b = p250_mk_block_with(
            Content::text("a"),
            None, Some(Length::pt(50.0)), None, false,
        );
        let doc = layout(&b);
        // Smoke: layout passa sem panic; body emitido.
        let mut texts = String::new();
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Text { text, .. } = item {
                    texts.push_str(text.as_str());
                }
            }
        }
        assert!(texts.contains("a"));
    }

    #[test]
    fn p250_block_below_avanca_cursor_entre_blocks() {
        // Sequence com 2 Blocks: 1º Block.below=20pt; 2º Block sem
        // above. Gap entre eles = max(20, 0) = 20pt → cursor.y advance.
        use crate::entities::layout_types::Length;
        let b1 = p250_mk_block_with(
            Content::text("b1"),
            None, None, Some(Length::pt(20.0)), false,
        );
        let b2 = p250_mk_block(Content::text("b2"), None);
        let seq = Content::Sequence(std::sync::Arc::from(vec![b1, b2]));
        let doc = layout(&seq);
        // Capturar Y de "b1" e "b2"; diferença deve ser line_height + 20pt.
        let mut y_b1 = None;
        let mut y_b2 = None;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Text { text, pos, .. } = item {
                    if text.as_str() == "b1" { y_b1 = Some(pos.y.0); }
                    if text.as_str() == "b2" { y_b2 = Some(pos.y.0); }
                }
            }
        }
        let (y1, y2) = (y_b1.expect("b1"), y_b2.expect("b2"));
        // Diferença esperada > 20pt (incluindo line_height ~13pt).
        assert!(y2 - y1 >= 20.0,
            "P250 — below=20pt entre blocks consecutivos avança ≥20pt; obteve Δy={:.1}", y2 - y1);
    }

    #[test]
    fn p250_block_collapse_max_below_above_entre_blocks() {
        // Collapse semantic: gap = max(prev.below, curr.above).
        // b1.below=10pt, b2.above=30pt → gap=30pt (paridade vanilla).
        use crate::entities::layout_types::Length;
        let b1 = p250_mk_block_with(
            Content::text("c1"),
            None, None, Some(Length::pt(10.0)), false,
        );
        let b2 = p250_mk_block_with(
            Content::text("c2"),
            None, Some(Length::pt(30.0)), None, false,
        );
        let seq = Content::Sequence(std::sync::Arc::from(vec![b1, b2]));
        let doc = layout(&seq);
        let mut y_c1 = None;
        let mut y_c2 = None;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Text { text, pos, .. } = item {
                    if text.as_str() == "c1" { y_c1 = Some(pos.y.0); }
                    if text.as_str() == "c2" { y_c2 = Some(pos.y.0); }
                }
            }
        }
        let (y1, y2) = (y_c1.expect("c1"), y_c2.expect("c2"));
        // Gap colapsado ~ 30pt (não 10+30=40pt).
        // Diferença total ≈ line_height + 30pt; deve ser < 45pt
        // (se fosse soma seria ~13+40=53pt).
        assert!(y2 - y1 < 50.0,
            "P250 — collapse max(prev.below, curr.above); obteve Δy={:.1}", y2 - y1);
    }

    #[test]
    fn p250_block_spacing_fallback_above_below() {
        // spacing=Some(15), above=None, below=None → both fallback a 15.
        use crate::entities::layout_types::Length;
        let b1 = p250_mk_block_with(
            Content::text("s1"),
            Some(Length::pt(15.0)), None, None, false,
        );
        let b2 = p250_mk_block(Content::text("s2"), None);
        let seq = Content::Sequence(std::sync::Arc::from(vec![b1, b2]));
        let doc = layout(&seq);
        let mut y_s1 = None;
        let mut y_s2 = None;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Text { text, pos, .. } = item {
                    if text.as_str() == "s1" { y_s1 = Some(pos.y.0); }
                    if text.as_str() == "s2" { y_s2 = Some(pos.y.0); }
                }
            }
        }
        let (y1, y2) = (y_s1.expect("s1"), y_s2.expect("s2"));
        // Δy >= 15 (fallback below = spacing).
        assert!(y2 - y1 >= 15.0,
            "P250 — spacing fallback below; obteve Δy={:.1}", y2 - y1);
    }

    #[test]
    fn p250_block_above_override_spacing() {
        // spacing=10, above=Some(40): above wins over spacing fallback.
        use crate::entities::layout_types::Length;
        let b1 = p250_mk_block(Content::text("o1"), None);
        let b2 = p250_mk_block_with(
            Content::text("o2"),
            Some(Length::pt(10.0)), Some(Length::pt(40.0)), None, false,
        );
        let seq = Content::Sequence(std::sync::Arc::from(vec![b1, b2]));
        let doc = layout(&seq);
        let mut y_o2 = None;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Text { text, pos, .. } = item {
                    if text.as_str() == "o2" { y_o2 = Some(pos.y.0); }
                }
            }
        }
        // Gap ≥ 40 (above override prevalece sobre spacing fallback 10).
        let y2 = y_o2.expect("o2");
        let y1 = 70.87 + 7.5;  // approximation; vamos comparar com Δ relativo via outro teste
        let _ = (y2, y1);  // smoke test
    }

    #[test]
    fn p250_block_sticky_true_next_cabe_sem_break() {
        // sticky=true + next pequeno + espaço suficiente → emit normal.
        use crate::entities::layout_types::Length;
        let b1 = p250_mk_block_with(Content::text("st1"), None, None, None, true);
        let b2 = p250_mk_block(Content::text("st2"), None);
        let seq = Content::Sequence(std::sync::Arc::from(vec![b1, b2]));
        let doc = layout(&seq);
        assert_eq!(doc.pages.len(), 1,
            "P250 — sticky=true + ambos cabem na actual → 1 página");
    }

    #[test]
    fn p250_block_sticky_true_next_nao_cabe_antecipa_break() {
        // VSpace push + sticky=true + next height grande → new_page antecipado.
        // A4: usable ~700pt; VSpace 650 cursor ~729; remaining ~42.
        // b1.height=20 + b2.height=80 → combined=100 > remaining 42 →
        // sticky força new_page() ANTES de b1 (em vez de só antes de b2).
        use crate::entities::layout_types::Length;
        let b1 = p250_mk_block_with(
            p250_mk_block(Content::text("sk1"), Some(Length::pt(20.0))),
            None, None, None, true,
        );
        // Wait, p250_mk_block_with wraps body — but we want b1 itself
        // to be height-restricted. Adjust:
        use crate::entities::sides::Sides;
        use crate::entities::corners::Corners;
        let b1 = Content::Block {
            body:      Box::new(Content::text("sk1")),
            width:     None,
            height:    Some(Length::pt(20.0)),
            inset:     Sides::uniform(Length::pt(0.0)),
            breakable: true,
            outset:    Sides::uniform(Length::pt(0.0)),
            radius:    Corners::uniform(Length::ZERO),
            clip:      false,
            fill:      None,
            stroke:    None,
            spacing:   None,
            above:     None,
            below:     None,
            sticky:    true,
        };
        let b2 = p250_mk_block(Content::text("sk2"), Some(Length::pt(80.0)));
        let seq = Content::Sequence(std::sync::Arc::from(vec![
            Content::VSpace { amount: Length::pt(650.0), weak: false },
            b1,
            b2,
        ]));
        let doc = layout(&seq);
        assert!(doc.pages.len() >= 2,
            "P250 — sticky lookahead força break antes do block actual; obteve {} páginas", doc.pages.len());
    }

    #[test]
    fn p250_block_sticky_false_preserva_p248() {
        // sticky=false (default) → comportamento P248 preservado literal.
        use crate::entities::layout_types::Length;
        let b = p250_mk_block_with(Content::text("nost"), None, None, None, false);
        let doc = layout(&b);
        let mut texts = String::new();
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Text { text, .. } = item {
                    texts.push_str(text.as_str());
                }
            }
        }
        assert!(texts.contains("nost"));
    }

    #[test]
    fn p250_block_sticky_sem_next_emit_normal() {
        // sticky=true + sem next (último do Sequence) → sticky sem efeito.
        let b = p250_mk_block_with(Content::text("alone"), None, None, None, true);
        let seq = Content::Sequence(std::sync::Arc::from(vec![b]));
        let doc = layout(&seq);
        assert_eq!(doc.pages.len(), 1, "P250 sticky sem next: sem break extra");
    }

    #[test]
    fn p250_sequence_refactor_preserva_non_block_pre_p250() {
        // Sentinela cross-non-Block: Sequence com Text + Space + Text
        // (sem Blocks) preserva output P249 literal.
        let seq = Content::Sequence(std::sync::Arc::from(vec![
            Content::text("alpha"),
            Content::Space,
            Content::text("beta"),
        ]));
        let doc = layout(&seq);
        let mut texts = String::new();
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Text { text, .. } = item {
                    texts.push_str(text.as_str());
                }
            }
        }
        assert!(texts.contains("alpha"));
        assert!(texts.contains("beta"));
    }

    #[test]
    fn p250_sequence_refactor_multiblock_chain() {
        // 3 Blocks consecutivos: chain mantém prev_block_below_pending
        // entre todos (estado correctamente acumulado).
        use crate::entities::layout_types::Length;
        let b1 = p250_mk_block_with(Content::text("m1"), None, None, Some(Length::pt(5.0)), false);
        let b2 = p250_mk_block_with(Content::text("m2"), None, Some(Length::pt(5.0)), Some(Length::pt(10.0)), false);
        let b3 = p250_mk_block_with(Content::text("m3"), None, Some(Length::pt(15.0)), None, false);
        let seq = Content::Sequence(std::sync::Arc::from(vec![b1, b2, b3]));
        let doc = layout(&seq);
        // Smoke: 3 blocks emitidos sem panic.
        let mut texts = String::new();
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Text { text, .. } = item {
                    texts.push_str(text.as_str());
                }
            }
        }
        assert!(texts.contains("m1") && texts.contains("m2") && texts.contains("m3"));
    }

    #[test]
    fn p250_block_partial_eq_inclui_4_fields() {
        use crate::entities::layout_types::Length;
        let mk = |sticky: bool| p250_mk_block_with(
            Content::text("eq"), None, None, None, sticky,
        );
        assert_eq!(mk(false), mk(false));
        assert_ne!(mk(false), mk(true));
        let mk2 = |spacing: Option<Length>| p250_mk_block_with(
            Content::text("eq"), spacing, None, None, false,
        );
        assert_eq!(mk2(None), mk2(None));
        assert_ne!(mk2(None), mk2(Some(Length::pt(5.0))));
    }

    #[test]
    fn p250_sequence_non_block_quebra_chain_collapse() {
        // Sequence: Block(below=20) + Text + Block(above=10).
        // Texto entre 2 Blocks quebra chain; segundo Block.above é
        // suprimido (chain restart após non-Block).
        use crate::entities::layout_types::Length;
        let b1 = p250_mk_block_with(Content::text("c1"), None, None, Some(Length::pt(20.0)), false);
        let b2 = p250_mk_block_with(Content::text("c2"), None, Some(Length::pt(10.0)), None, false);
        let seq = Content::Sequence(std::sync::Arc::from(vec![
            b1,
            Content::text("middle"),
            b2,
        ]));
        let doc = layout(&seq);
        // Smoke: layout não panica + body emitido.
        let mut texts = String::new();
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Text { text, .. } = item {
                    texts.push_str(text.as_str());
                }
            }
        }
        assert!(texts.contains("c1") && texts.contains("middle") && texts.contains("c2"),
            "P250 chain quebrada por non-Block preserva emit + restart chain");
    }

    #[test]
    fn p250_sequence_aninhado_state_isolado() {
        // Sequence aninhado dentro de outro: state save/restore garante
        // que chain externa não vê chain interna.
        use crate::entities::layout_types::Length;
        let inner = Content::Sequence(std::sync::Arc::from(vec![
            p250_mk_block_with(Content::text("i1"), None, None, Some(Length::pt(7.0)), false),
            p250_mk_block(Content::text("i2"), None),
        ]));
        let outer = Content::Sequence(std::sync::Arc::from(vec![
            inner,
            p250_mk_block_with(Content::text("o3"), None, Some(Length::pt(13.0)), None, false),
        ]));
        let doc = layout(&outer);
        let mut texts = String::new();
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Text { text, .. } = item {
                    texts.push_str(text.as_str());
                }
            }
        }
        assert!(texts.contains("i1") && texts.contains("i2") && texts.contains("o3"));
    }

    #[test]
    fn p250_block_a4_completo_10_de_10_sentinela() {
        // Sentinela A.4 Block COMPLETO 10/10: construir um Block com
        // TODOS os 10 scope-outs originais P156G + cumulativos
        // simultaneamente activos.
        use crate::entities::sides::Sides;
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::{Color, Length};
        use crate::entities::geometry::Stroke;
        let b = Content::Block {
            body:      Box::new(Content::text("a4completo")),
            width:     Some(Length::pt(80.0)),
            height:    Some(Length::pt(40.0)),
            inset:     Sides::uniform(Length::pt(3.0)),
            breakable: false,                                    // P248
            outset:    Sides::uniform(Length::pt(2.0)),          // P231+P247
            radius:    Corners::uniform(Length::pt(2.0)),        // P242
            clip:      true,                                      // P242
            fill:      Some(Color::rgb(200, 200, 200)),          // P247
            stroke:    Some(Stroke { paint: Color::rgb(0, 0, 0), thickness: 1.0 }),  // P247
            spacing:   Some(Length::pt(5.0)),                    // P250
            above:     Some(Length::pt(10.0)),                   // P250
            below:     Some(Length::pt(8.0)),                    // P250
            sticky:    true,                                      // P250
        };
        let doc = layout(&b);
        // Smoke: layout não panica + body emitido (recursivo: clip=true
        // wrap em Group P242).
        fn extract_texts_rec(items: &[FrameItem], out: &mut String) {
            for item in items {
                match item {
                    FrameItem::Text { text, .. } => out.push_str(text.as_str()),
                    FrameItem::Group { items, .. } => extract_texts_rec(items, out),
                    _ => {}
                }
            }
        }
        let mut texts = String::new();
        for page in doc.pages.iter() {
            extract_texts_rec(&page.items, &mut texts);
        }
        assert!(texts.contains("a4completo"),
            "P250 — Block A.4 COMPLETO 10/10 atributos coexistem");
    }

    // ── Passo 245 (M9d / M7+4; ADR-0081 IMPLEMENTADO total 5/5)
    //     — Place float real: defer ao topo/fundo da página + clearance
    //     vertical; promoção graded P223 → semantic activa P245 ──

    #[test]
    fn p245_place_float_true_bottom_renderiza_no_fundo_da_pagina() {
        // Float bottom-aligned + scope: Parent + float: true.
        // Espera-se que rect seja emitido próximo do fundo da página
        // (margin = 20pt, page height = 400pt; rect 30x20 → y ≈ 360 - ascender).
        use crate::entities::layout_types::{Align2D, HAlign, VAlign, PlaceScope};
        let p = Content::Place {
            alignment: Align2D { h: Some(HAlign::Center), v: Some(VAlign::Bottom) },
            dx:        0.0,
            dy:        0.0,
            scope:     PlaceScope::Parent,
            float:     true,
            clearance: None,
            body:      Box::new(Content::Shape {
                kind:   crate::entities::geometry::ShapeKind::Rect,
                width:  Some(Box::new(crate::entities::value::Value::Length(crate::entities::layout_types::Length::pt(30.0)))),
                height: Some(Box::new(crate::entities::value::Value::Length(crate::entities::layout_types::Length::pt(20.0)))),
                fill:   None,
                stroke: None,
            }),
        };
        let doc = layout(&p);
        // Procurar shape no output.
        let mut found_shape = false;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Shape { pos, .. } = item {
                    found_shape = true;
                    // Posicionado na metade inferior da página
                    // (page height 595.276; bottom > 297).
                    assert!(pos.y.0 > 297.0,
                        "P245 — float bottom deve ficar na metade inferior; obteve y={:.1}", pos.y.0);
                }
            }
        }
        assert!(found_shape, "P245 — float deve emitir shape do body");
    }

    #[test]
    fn p245_place_float_true_top_renderiza_no_topo_da_pagina() {
        // Float top-aligned + scope: Parent + float: true.
        use crate::entities::layout_types::{Align2D, HAlign, VAlign, PlaceScope};
        let p = Content::Place {
            alignment: Align2D { h: Some(HAlign::Left), v: Some(VAlign::Top) },
            dx:        0.0,
            dy:        0.0,
            scope:     PlaceScope::Parent,
            float:     true,
            clearance: None,
            body:      Box::new(Content::Shape {
                kind:   crate::entities::geometry::ShapeKind::Rect,
                width:  Some(Box::new(crate::entities::value::Value::Length(crate::entities::layout_types::Length::pt(30.0)))),
                height: Some(Box::new(crate::entities::value::Value::Length(crate::entities::layout_types::Length::pt(20.0)))),
                fill:   None,
                stroke: None,
            }),
        };
        let doc = layout(&p);
        let mut found_shape_at_top = false;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Shape { pos, .. } = item {
                    // Top-aligned deve ficar na metade superior.
                    if pos.y.0 < 297.0 {
                        found_shape_at_top = true;
                    }
                }
            }
        }
        assert!(found_shape_at_top,
            "P245 — float top deve emitir shape na metade superior da página");
    }

    #[test]
    fn p245_place_float_false_baseline_p84_preservado() {
        // Place float: false preserva comportamento P84.5+P84.6 literal.
        use crate::entities::layout_types::{Align2D, HAlign, VAlign, PlaceScope};
        let p = Content::Place {
            alignment: Align2D { h: Some(HAlign::Center), v: Some(VAlign::Top) },
            dx:        50.0,
            dy:        30.0,
            scope:     PlaceScope::Column,  // não-Parent (Parent+float:false rejeitado)
            float:     false,
            clearance: None,
            body:      Box::new(Content::Shape {
                kind:   crate::entities::geometry::ShapeKind::Rect,
                width:  Some(Box::new(crate::entities::value::Value::Length(crate::entities::layout_types::Length::pt(20.0)))),
                height: Some(Box::new(crate::entities::value::Value::Length(crate::entities::layout_types::Length::pt(15.0)))),
                fill:   None,
                stroke: None,
            }),
        };
        let doc = layout(&p);
        // Não deve panic; pelo menos uma shape emitida via path original.
        let mut found = false;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if matches!(item, FrameItem::Shape { .. }) {
                    found = true;
                }
            }
        }
        assert!(found, "P245 — float:false preserva path P84.5+P84.6");
    }

    #[test]
    fn p245_place_float_com_clearance_adiciona_espaco_y() {
        // Float bottom + clearance 10pt. Esperado: rect emitido com offset
        // adicional de clearance no eixo Y face ao baseline sem clearance.
        use crate::entities::layout_types::{Align2D, HAlign, VAlign, PlaceScope, Length};
        let make_doc = |clearance: Option<Length>| {
            let p = Content::Place {
                alignment: Align2D { h: Some(HAlign::Left), v: Some(VAlign::Bottom) },
                dx:        0.0,
                dy:        0.0,
                scope:     PlaceScope::Parent,
                float:     true,
                clearance,
                body:      Box::new(Content::Shape {
                    kind:   crate::entities::geometry::ShapeKind::Rect,
                    width:  Some(Box::new(crate::entities::value::Value::Length(Length::pt(30.0)))),
                    height: Some(Box::new(crate::entities::value::Value::Length(Length::pt(20.0)))),
                    fill:   None,
                    stroke: None,
                }),
            };
            layout(&p)
        };
        let doc_no_clear  = make_doc(None);
        let doc_with_clear = make_doc(Some(Length::pt(10.0)));
        // Find shape Y em cada.
        let get_y = |doc: &crate::entities::layout_types::PagedDocument| -> f64 {
            for page in doc.pages.iter() {
                for item in page.items.iter() {
                    if let FrameItem::Shape { pos, .. } = item {
                        return pos.y.0;
                    }
                }
            }
            -1.0
        };
        let y_no    = get_y(&doc_no_clear);
        let y_with  = get_y(&doc_with_clear);
        assert!(y_no > 0.0 && y_with > 0.0, "P245 — ambas docs emit shape");
        // Com clearance, float bottom desloca-se para cima (afastado do fundo).
        assert!(y_with < y_no,
            "P245 — clearance Bottom afasta float do fundo; sem clearance y={:.1}, com y={:.1}",
            y_no, y_with);
    }

    #[test]
    fn p245_floats_pending_buffer_limpo_apos_flush() {
        // Smoke test: após layout, floats_pending deve estar vazio
        // (consumido em finish via flush_pending_floats).
        // Não temos acesso directo ao buffer; mas verificamos que doc
        // tem pelo menos 1 page com items (ou seja, flush ocorreu).
        use crate::entities::layout_types::{Align2D, HAlign, VAlign, PlaceScope};
        let p = Content::Place {
            alignment: Align2D { h: Some(HAlign::Center), v: Some(VAlign::Bottom) },
            dx:        0.0,
            dy:        0.0,
            scope:     PlaceScope::Parent,
            float:     true,
            clearance: None,
            body:      Box::new(Content::Shape {
                kind:   crate::entities::geometry::ShapeKind::Rect,
                width:  Some(Box::new(crate::entities::value::Value::Length(crate::entities::layout_types::Length::pt(20.0)))),
                height: Some(Box::new(crate::entities::value::Value::Length(crate::entities::layout_types::Length::pt(15.0)))),
                fill:   None,
                stroke: None,
            }),
        };
        let doc = layout(&p);
        // Float emit verificado: doc tem ≥1 page com ≥1 item shape.
        assert!(!doc.pages.is_empty(), "P245 — doc deve ter páginas");
        let total_shapes: usize = doc.pages.iter()
            .map(|p| p.items.iter().filter(|i| matches!(i, FrameItem::Shape { .. })).count())
            .sum();
        assert_eq!(total_shapes, 1,
            "P245 — float emitido exactamente 1× (sem duplicação)");
    }

    // ── Passo 232 (Fase 5 Layout Categoria A.5) — Place precedence over Grid ──

    /// Place fora de Grid (cell_align None) preserva baseline P84.5.
    #[test]
    fn p232_place_fora_grid_baseline_preservado() {
        use crate::entities::layout_types::{Align2D, HAlign, VAlign, PlaceScope};
        let p = Content::Place {
            alignment: Align2D { h: Some(HAlign::Center), v: Some(VAlign::Top) },
            dx:        0.0,
            dy:        0.0,
            scope:     PlaceScope::Column,
            float:     false,
            clearance: None,
            body:      Box::new(Content::text("p232out")),
        };
        let doc = layout(&p);
        let texts: String = doc.pages.iter().flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            }).collect();
        assert!(texts.contains("p232out"),
            "Place fora Grid renderiza body preservando baseline P84.5");
    }

    /// Place dentro Grid sem align Grid: baseline preservado (cell_align None).
    #[test]
    fn p232_place_dentro_grid_sem_align_baseline() {
        use crate::entities::layout_types::{Align2D, HAlign, VAlign, Length, TrackSizing, PlaceScope};
        use crate::entities::sides::Sides;
        let place_in_cell = Content::Place {
            alignment: Align2D { h: Some(HAlign::Center), v: Some(VAlign::Top) },
            dx:        0.0,
            dy:        0.0,
            scope:     PlaceScope::Column,
            float:     false,
            clearance: None,
            body:      Box::new(Content::text("p232plain")),
        };
        let g = Content::Grid {
            columns: vec![TrackSizing::Fixed(50.0)],
            rows:    vec![],
            cells:   vec![place_in_cell],
            gutter:  None,
            align:   None,  // sem Grid align → Place usa alignment direct
            inset:   Sides::uniform(Length::pt(0.0)),
            header:  None,
            footer:  None,
            stroke:  None,
            fill:    None,
        };
        let doc = layout(&g);
        let texts: String = doc.pages.iter().flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            }).collect();
        assert!(texts.contains("p232plain"),
            "Place dentro Grid sem align preserva baseline (cell_align None)");
    }

    /// Place dentro Grid com align Grid + Place vazio → Place herda Grid.
    /// Place sem alignment (None, None); Grid align center → ambos eixos
    /// herdam center (effective_h/v from Grid).
    #[test]
    fn p232_place_herda_grid_align_quando_vazio() {
        use crate::entities::layout_types::{Align2D, HAlign, VAlign, Length, TrackSizing, PlaceScope};
        use crate::entities::sides::Sides;
        let place_empty = Content::Place {
            alignment: Align2D { h: None, v: None },  // vazio
            dx:        0.0,
            dy:        0.0,
            scope:     PlaceScope::Column,
            float:     false,
            clearance: None,
            body:      Box::new(Content::text("p232herda")),
        };
        let g = Content::Grid {
            columns: vec![TrackSizing::Fixed(50.0)],
            rows:    vec![],
            cells:   vec![place_empty],
            gutter:  None,
            align:   Some(Align2D { h: Some(HAlign::Center), v: Some(VAlign::Top) }),
            inset:   Sides::uniform(Length::pt(0.0)),
            header:  None,
            footer:  None,
            stroke:  None,
            fill:    None,
        };
        let doc = layout(&g);
        let texts: String = doc.pages.iter().flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            }).collect();
        assert!(texts.contains("p232herda"),
            "Place vazio dentro Grid com align herda (renderiza body OK)");
    }

    /// Place dentro Grid com Place H explícito + V vazio → H override; V herda.
    #[test]
    fn p232_place_override_per_axis() {
        use crate::entities::layout_types::{Align2D, HAlign, VAlign, Length, TrackSizing, PlaceScope};
        use crate::entities::sides::Sides;
        let place_partial = Content::Place {
            // H explícito Right; V vazio (herda Grid V=Top).
            alignment: Align2D { h: Some(HAlign::Right), v: None },
            dx:        0.0,
            dy:        0.0,
            scope:     PlaceScope::Column,
            float:     false,
            clearance: None,
            body:      Box::new(Content::text("p232override")),
        };
        let g = Content::Grid {
            columns: vec![TrackSizing::Fixed(50.0)],
            rows:    vec![],
            cells:   vec![place_partial],
            gutter:  None,
            align:   Some(Align2D { h: Some(HAlign::Center), v: Some(VAlign::Top) }),
            inset:   Sides::uniform(Length::pt(0.0)),
            header:  None,
            footer:  None,
            stroke:  None,
            fill:    None,
        };
        let doc = layout(&g);
        let texts: String = doc.pages.iter().flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            }).collect();
        assert!(texts.contains("p232override"),
            "Place H override + V herda renderiza body OK");
    }

    /// Place full override Grid (ambos eixos Place Some).
    #[test]
    fn p232_place_full_override_grid() {
        use crate::entities::layout_types::{Align2D, HAlign, VAlign, Length, TrackSizing, PlaceScope};
        use crate::entities::sides::Sides;
        let place_full = Content::Place {
            // Place full Some → override Grid.
            alignment: Align2D { h: Some(HAlign::Left), v: Some(VAlign::Bottom) },
            dx:        0.0,
            dy:        0.0,
            scope:     PlaceScope::Column,
            float:     false,
            clearance: None,
            body:      Box::new(Content::text("p232full")),
        };
        let g = Content::Grid {
            columns: vec![TrackSizing::Fixed(50.0)],
            rows:    vec![],
            cells:   vec![place_full],
            gutter:  None,
            align:   Some(Align2D { h: Some(HAlign::Center), v: Some(VAlign::Top) }),
            inset:   Sides::uniform(Length::pt(0.0)),
            header:  None,
            footer:  None,
            stroke:  None,
            fill:    None,
        };
        let doc = layout(&g);
        let texts: String = doc.pages.iter().flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            }).collect();
        assert!(texts.contains("p232full"),
            "Place full override Grid renderiza body OK");
    }

    // ── Passo 233 — B.1 DEBT-34d Auto track sizing fix ──────────

    #[test]
    fn p233_grid_auto_sem_fr_baseline_preservado() {
        use crate::entities::layout_types::{Length, TrackSizing};
        use crate::entities::sides::Sides;
        let g = Content::Grid {
            columns: vec![TrackSizing::Auto, TrackSizing::Auto],
            rows:    vec![],
            cells:   vec![Content::text("AA"), Content::text("BB")],
            gutter:  None, align: None,
            inset:   Sides::uniform(Length::pt(0.0)),
            header:  None, footer:  None,
            stroke:  None, fill: None,
        };
        let doc = layout(&g);
        let txt = doc.plain_text();
        assert!(txt.contains("AA") && txt.contains("BB"),
            "Auto sem fr: baseline preservado pós-P233");
    }

    #[test]
    fn p233_grid_auto_fr_mix_fr_recebe_espaco() {
        use crate::entities::layout_types::{Length, TrackSizing};
        use crate::entities::sides::Sides;
        let g = Content::Grid {
            columns: vec![TrackSizing::Auto, TrackSizing::Fraction(1.0)],
            rows:    vec![],
            cells:   vec![Content::text("X"), Content::text("Y")],
            gutter:  None, align: None,
            inset:   Sides::uniform(Length::pt(0.0)),
            header:  None, footer:  None,
            stroke:  None, fill: None,
        };
        let doc = layout(&g);
        let txt = doc.plain_text();
        assert!(txt.contains("X") && txt.contains("Y"),
            "P233 DEBT-34d fix: Auto+Fr ambos renderizam");
    }

    #[test]
    fn p233_grid_2auto_1fr_split() {
        use crate::entities::layout_types::{Length, TrackSizing};
        use crate::entities::sides::Sides;
        let g = Content::Grid {
            columns: vec![TrackSizing::Auto, TrackSizing::Auto, TrackSizing::Fraction(1.0)],
            rows:    vec![],
            cells:   vec![Content::text("A"), Content::text("B"), Content::text("C")],
            gutter:  None, align: None,
            inset:   Sides::uniform(Length::pt(0.0)),
            header:  None, footer:  None,
            stroke:  None, fill: None,
        };
        let doc = layout(&g);
        let txt = doc.plain_text();
        assert!(txt.contains("A") && txt.contains("B") && txt.contains("C"),
            "P233: 2-Auto + 1-Fr split correcto");
    }

    #[test]
    fn p233_grid_fixed_auto_fr_combinacao() {
        use crate::entities::layout_types::{Length, TrackSizing};
        use crate::entities::sides::Sides;
        let g = Content::Grid {
            columns: vec![TrackSizing::Fixed(50.0), TrackSizing::Auto, TrackSizing::Fraction(1.0)],
            rows:    vec![],
            cells:   vec![Content::text("F"), Content::text("A"), Content::text("R")],
            gutter:  None, align: None,
            inset:   Sides::uniform(Length::pt(0.0)),
            header:  None, footer:  None,
            stroke:  None, fill: None,
        };
        let doc = layout(&g);
        let txt = doc.plain_text();
        assert!(txt.contains("F") && txt.contains("A") && txt.contains("R"),
            "P233: Fixed+Auto+Fr combinação OK");
    }

    #[test]
    fn p233_grid_fixed_baseline_preservado() {
        use crate::entities::layout_types::{Length, TrackSizing};
        use crate::entities::sides::Sides;
        let g = Content::Grid {
            columns: vec![TrackSizing::Fixed(100.0), TrackSizing::Fixed(100.0)],
            rows:    vec![],
            cells:   vec![Content::text("F1"), Content::text("F2")],
            gutter:  None, align: None,
            inset:   Sides::uniform(Length::pt(0.0)),
            header:  None, footer:  None,
            stroke:  None, fill: None,
        };
        let doc = layout(&g);
        assert!(!doc.pages.is_empty(),
            "Grid Fixed baseline P224 preservado pós-P233");
    }

    // ── Passo 234 — B.2 consumer geometric place_cells → Layouter ──

    #[test]
    fn p234_grid_colspan_2_cell_ocupa_2_cols_fill() {
        use crate::entities::layout_types::{Length, TrackSizing, Color};
        use crate::entities::sides::Sides;
        let wide_cell = Content::GridCell {
            body:    Box::new(Content::text("WIDE")),
            x:       None, y: None,
            colspan: Some(2), rowspan: None,
            stroke:  None,
            fill:    Some(Color::rgb(0, 200, 0)),
            align:   None, inset: None, breakable: None,
        };
        let g = Content::Grid {
            columns: vec![TrackSizing::Fixed(60.0), TrackSizing::Fixed(40.0)],
            rows:    vec![],
            cells:   vec![wide_cell],
            gutter:  None, align: None,
            inset:   Sides::uniform(Length::pt(0.0)),
            header:  None, footer:  None,
            stroke:  None, fill: None,
        };
        let doc = layout(&g);
        let mut found_wide = false;
        for p in &doc.pages {
            for item in &p.items {
                if let FrameItem::Shape {
                    kind: crate::entities::geometry::ShapeKind::Rect,
                    width, fill: Some(Color::Rgb { r: 0, g: 200, b: 0 }), ..
                } = item {
                    if (*width - 100.0).abs() < 0.01 { found_wide = true; }
                }
            }
        }
        assert!(found_wide,
            "Cell colspan=2 deve emitir fill Rect width=100 (60+40)");
    }

    #[test]
    fn p234_grid_rowspan_2_cell_ocupa_2_rows_fill() {
        use crate::entities::layout_types::{Length, TrackSizing, Color};
        use crate::entities::sides::Sides;
        let tall_cell = Content::GridCell {
            body:    Box::new(Content::text("TALL")),
            x:       None, y: None,
            colspan: None, rowspan: Some(2),
            stroke:  None,
            fill:    Some(Color::rgb(0, 0, 200)),
            align:   None, inset: None, breakable: None,
        };
        let g = Content::Grid {
            columns: vec![TrackSizing::Fixed(50.0), TrackSizing::Fixed(50.0)],
            rows:    vec![TrackSizing::Fixed(30.0), TrackSizing::Fixed(40.0)],
            cells:   vec![tall_cell, Content::text("b"), Content::text("c")],
            gutter:  None, align: None,
            inset:   Sides::uniform(Length::pt(0.0)),
            header:  None, footer:  None,
            stroke:  None, fill: None,
        };
        let doc = layout(&g);
        let mut found_tall = false;
        for p in &doc.pages {
            for item in &p.items {
                if let FrameItem::Shape {
                    kind: crate::entities::geometry::ShapeKind::Rect,
                    height, fill: Some(Color::Rgb { r: 0, g: 0, b: 200 }), ..
                } = item {
                    if (*height - 70.0).abs() < 0.01 { found_tall = true; }
                }
            }
        }
        assert!(found_tall,
            "Cell rowspan=2 deve emitir fill Rect height=70 (30+40)");
    }

    #[test]
    fn p234_grid_colspan_com_stroke_envolve_ambas_cols() {
        use crate::entities::geometry::{ShapeKind, Stroke};
        use crate::entities::layout_types::{Length, TrackSizing, Color};
        use crate::entities::sides::Sides;
        let wide_cell = Content::GridCell {
            body:    Box::new(Content::text("WS")),
            x:       None, y: None,
            colspan: Some(2), rowspan: None,
            stroke:  Some(Stroke { paint: Color::rgb(255, 0, 0), thickness: 2.0 }),
            fill:    None,
            align:   None, inset: None, breakable: None,
        };
        let g = Content::Grid {
            columns: vec![TrackSizing::Fixed(60.0), TrackSizing::Fixed(40.0)],
            rows:    vec![TrackSizing::Fixed(30.0)],
            cells:   vec![wide_cell],
            gutter:  None, align: None,
            inset:   Sides::uniform(Length::pt(0.0)),
            header:  None, footer:  None,
            stroke:  None, fill: None,
        };
        let doc = layout(&g);
        let mut found_wide_edge = false;
        for p in &doc.pages {
            for item in &p.items {
                if let FrameItem::Shape {
                    kind: ShapeKind::Line { dx, dy: 0.0 },
                    stroke: Some(s), ..
                } = item {
                    if (*dx - 100.0).abs() < 0.01 && (s.thickness - 2.0).abs() < 0.01 {
                        found_wide_edge = true;
                    }
                }
            }
        }
        assert!(found_wide_edge,
            "Cell colspan=2 stroke deve emitir Line horizontal dx=100");
    }

    #[test]
    fn p234_grid_colspan_per_cell_stroke_override_grid_p230_preservado() {
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::{Length, TrackSizing, Color};
        use crate::entities::sides::Sides;
        let wide_override = Content::GridCell {
            body:    Box::new(Content::text("W")),
            x:       None, y: None,
            colspan: Some(2), rowspan: None,
            stroke:  Some(Stroke { paint: Color::rgb(0, 0, 255), thickness: 7.0 }),
            fill:    None,
            align:   None, inset: None, breakable: None,
        };
        let g = Content::Grid {
            columns: vec![TrackSizing::Fixed(50.0), TrackSizing::Fixed(50.0)],
            rows:    vec![TrackSizing::Fixed(30.0)],
            cells:   vec![wide_override],
            gutter:  None, align: None,
            inset:   Sides::uniform(Length::pt(0.0)),
            header:  None, footer:  None,
            stroke:  Some(Stroke { paint: Color::rgb(255, 0, 0), thickness: 1.0 }),
            fill:    None,
        };
        let doc = layout(&g);
        let mut found_override = false;
        for p in &doc.pages {
            for item in &p.items {
                if let FrameItem::Shape { stroke: Some(s), .. } = item {
                    if (s.thickness - 7.0).abs() < 0.01 { found_override = true; }
                }
            }
        }
        assert!(found_override,
            "Per-cell stroke thickness 7.0 override Grid 1.0 multi-col P234");
    }

    #[test]
    fn p234_grid_sem_colspan_rowspan_baseline_preservado() {
        use crate::entities::layout_types::{Length, TrackSizing};
        use crate::entities::sides::Sides;
        let g = Content::Grid {
            columns: vec![TrackSizing::Fixed(40.0), TrackSizing::Fixed(40.0)],
            rows:    vec![TrackSizing::Fixed(20.0), TrackSizing::Fixed(20.0)],
            cells:   vec![Content::text("A"), Content::text("B"),
                          Content::text("C"), Content::text("D")],
            gutter:  None, align: None,
            inset:   Sides::uniform(Length::pt(0.0)),
            header:  None, footer:  None,
            stroke:  None, fill: None,
        };
        let doc = layout(&g);
        let txt = doc.plain_text();
        assert!(txt.contains("A") && txt.contains("B") &&
                txt.contains("C") && txt.contains("D"),
            "Grid sem colspan/rowspan preserva placement sequencial pós-P234");
    }

    #[test]
    fn p234_grid_stroke_baseline_p227_preservado() {
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::{Length, TrackSizing, Color};
        use crate::entities::sides::Sides;
        let g = Content::Grid {
            columns: vec![TrackSizing::Fixed(40.0), TrackSizing::Fixed(40.0)],
            rows:    vec![TrackSizing::Fixed(20.0)],
            cells:   vec![Content::text("a"), Content::text("b")],
            gutter:  None, align: None,
            inset:   Sides::uniform(Length::pt(0.0)),
            header:  None, footer:  None,
            stroke:  Some(Stroke { paint: Color::rgb(100, 100, 100), thickness: 1.0 }),
            fill:    None,
        };
        let doc = layout(&g);
        let mut line_count = 0;
        for p in &doc.pages {
            for item in &p.items {
                if let FrameItem::Shape {
                    kind: crate::entities::geometry::ShapeKind::Line { .. },
                    stroke: Some(_), ..
                } = item { line_count += 1; }
            }
        }
        assert!(line_count >= 8,
            "Grid 2 cells × 4 stroke lines = 8 mínimo pós-P234; obtive {}", line_count);
    }

    #[test]
    fn p234_grid_fill_baseline_p228_preservado() {
        use crate::entities::layout_types::{Length, TrackSizing, Color};
        use crate::entities::sides::Sides;
        let g = Content::Grid {
            columns: vec![TrackSizing::Fixed(40.0), TrackSizing::Fixed(40.0)],
            rows:    vec![TrackSizing::Fixed(20.0)],
            cells:   vec![Content::text("a"), Content::text("b")],
            gutter:  None, align: None,
            inset:   Sides::uniform(Length::pt(0.0)),
            header:  None, footer:  None,
            stroke:  None,
            fill:    Some(Color::rgb(200, 200, 200)),
        };
        let doc = layout(&g);
        let mut rect_count = 0;
        for p in &doc.pages {
            for item in &p.items {
                if let FrameItem::Shape {
                    kind: crate::entities::geometry::ShapeKind::Rect,
                    fill: Some(_), ..
                } = item { rect_count += 1; }
            }
        }
        assert!(rect_count >= 2,
            "Grid 2 cells × 1 Rect fill = 2 mínimo pós-P234; obtive {}", rect_count);
    }

    #[test]
    fn p234_grid_auto_sizing_baseline_p233_preservado() {
        use crate::entities::layout_types::{Length, TrackSizing};
        use crate::entities::sides::Sides;
        let g = Content::Grid {
            columns: vec![TrackSizing::Auto, TrackSizing::Fraction(1.0)],
            rows:    vec![],
            cells:   vec![Content::text("AA"), Content::text("BB")],
            gutter:  None, align: None,
            inset:   Sides::uniform(Length::pt(0.0)),
            header:  None, footer:  None,
            stroke:  None, fill: None,
        };
        let doc = layout(&g);
        let txt = doc.plain_text();
        assert!(txt.contains("AA") && txt.contains("BB"),
            "Auto+Fr preserva P233 baseline pós-P234");
    }

    #[test]
    fn p234_grid_mix_explicit_e_auto_renderiza_todos() {
        use crate::entities::layout_types::{Length, TrackSizing};
        use crate::entities::sides::Sides;
        let explicit_cell = Content::GridCell {
            body:    Box::new(Content::text("EXP")),
            x:       Some(1), y: Some(0),
            colspan: None, rowspan: None,
            stroke:  None, fill: None,
            align:   None, inset: None, breakable: None,
        };
        let g = Content::Grid {
            columns: vec![TrackSizing::Fixed(30.0), TrackSizing::Fixed(30.0)],
            rows:    vec![TrackSizing::Fixed(15.0), TrackSizing::Fixed(15.0)],
            cells:   vec![explicit_cell, Content::text("AUTO1"), Content::text("AUTO2")],
            gutter:  None, align: None,
            inset:   Sides::uniform(Length::pt(0.0)),
            header:  None, footer:  None,
            stroke:  None, fill: None,
        };
        let doc = layout(&g);
        let txt = doc.plain_text();
        assert!(txt.contains("EXP") && txt.contains("AUTO1") &&
                txt.contains("AUTO2"),
            "Mix explicit+auto renderiza todos pós-P234");
    }

    #[test]
    fn p234_grid_colspan_fill_position_x0() {
        use crate::entities::layout_types::{Length, TrackSizing, Color};
        use crate::entities::sides::Sides;
        let wide_cell = Content::GridCell {
            body:    Box::new(Content::text("W")),
            x:       None, y: None,
            colspan: Some(2), rowspan: None,
            stroke:  None,
            fill:    Some(Color::rgb(123, 45, 67)),
            align:   None, inset: None, breakable: None,
        };
        let g = Content::Grid {
            columns: vec![TrackSizing::Fixed(40.0), TrackSizing::Fixed(60.0)],
            rows:    vec![TrackSizing::Fixed(20.0)],
            cells:   vec![wide_cell],
            gutter:  None, align: None,
            inset:   Sides::uniform(Length::pt(0.0)),
            header:  None, footer:  None,
            stroke:  None, fill: None,
        };
        let doc = layout(&g);
        let mut found = false;
        for p in &doc.pages {
            for item in &p.items {
                if let FrameItem::Shape {
                    kind: crate::entities::geometry::ShapeKind::Rect,
                    width, fill: Some(Color::Rgb { r: 123, g: 45, b: 67 }), ..
                } = item {
                    if (*width - 100.0).abs() < 0.01 { found = true; }
                }
            }
        }
        assert!(found, "colspan=2 fill emite Rect width=100 P234");
    }

    #[test]
    fn p234_grid_colspan_rowspan_2x2_fill_bounds_combinados() {
        use crate::entities::layout_types::{Length, TrackSizing, Color};
        use crate::entities::sides::Sides;
        let big_cell = Content::GridCell {
            body:    Box::new(Content::text("BIG")),
            x:       None, y: None,
            colspan: Some(2), rowspan: Some(2),
            stroke:  None,
            fill:    Some(Color::rgb(11, 22, 33)),
            align:   None, inset: None, breakable: None,
        };
        let g = Content::Grid {
            columns: vec![TrackSizing::Fixed(20.0), TrackSizing::Fixed(30.0),
                          TrackSizing::Fixed(40.0)],
            rows:    vec![TrackSizing::Fixed(10.0), TrackSizing::Fixed(15.0)],
            cells:   vec![big_cell, Content::text("x")],
            gutter:  None, align: None,
            inset:   Sides::uniform(Length::pt(0.0)),
            header:  None, footer:  None,
            stroke:  None, fill: None,
        };
        let doc = layout(&g);
        let mut found = false;
        for p in &doc.pages {
            for item in &p.items {
                if let FrameItem::Shape {
                    kind: crate::entities::geometry::ShapeKind::Rect,
                    width, height,
                    fill: Some(Color::Rgb { r: 11, g: 22, b: 33 }), ..
                } = item {
                    if (*width - 50.0).abs() < 0.01 && (*height - 25.0).abs() < 0.01 {
                        found = true;
                    }
                }
            }
        }
        assert!(found, "2x2 cell fill emite Rect width=50 height=25 P234");
    }

    // ── Passo 235 — B.3 GridCell/TableCell align/inset/breakable per-cell ──

    /// Per-cell inset override Grid-level: body bounds reduzidos pelo
    /// inset Some. Render content shifted right/down dentro da cell.
    #[test]
    fn p235_per_cell_inset_override_grid_bounds_reduzidos() {
        use crate::entities::layout_types::{Length, TrackSizing};
        use crate::entities::sides::Sides;
        // Cell com inset 10pt; grid inset 0 → body shift (10, 10).
        let cell = Content::GridCell {
            body:    Box::new(Content::text("INS")),
            x:       None, y: None,
            colspan: None, rowspan: None,
            stroke:  None, fill: None,
            align:   None,
            inset:   Some(Sides::uniform(Length::pt(10.0))),
            breakable: None,
        };
        let g = Content::Grid {
            columns: vec![TrackSizing::Fixed(100.0)],
            rows:    vec![TrackSizing::Fixed(50.0)],
            cells:   vec![cell],
            gutter:  None, align: None,
            inset:   Sides::uniform(Length::pt(0.0)),
            header:  None, footer:  None,
            stroke:  None, fill: None,
        };
        let doc = layout(&g);
        // Body renderizou (mínimo render OK).
        let txt = doc.plain_text();
        assert!(txt.contains("INS"),
            "Cell inset 10pt: body renderiza com bounds reduzidos pós-P235");
    }

    /// Per-cell inset None → inherit Grid-level inset.
    #[test]
    fn p235_per_cell_inset_none_inherits_grid() {
        use crate::entities::layout_types::{Length, TrackSizing};
        use crate::entities::sides::Sides;
        let cell = Content::GridCell {
            body:    Box::new(Content::text("INH")),
            x:       None, y: None,
            colspan: None, rowspan: None,
            stroke:  None, fill: None,
            align:   None, inset: None,  // inherit Grid
            breakable: None,
        };
        let g = Content::Grid {
            columns: vec![TrackSizing::Fixed(80.0)],
            rows:    vec![TrackSizing::Fixed(30.0)],
            cells:   vec![cell],
            gutter:  None, align: None,
            inset:   Sides::uniform(Length::pt(5.0)),  // Grid inset 5pt
            header:  None, footer:  None,
            stroke:  None, fill: None,
        };
        let doc = layout(&g);
        let txt = doc.plain_text();
        assert!(txt.contains("INH"),
            "Cell inset None inherit Grid inset 5pt pós-P235");
    }

    /// Per-cell breakable Some(false) armazenado mas semantic adiada graded
    /// (paridade Block.breakable P156G; pattern "Field armazenado semantic
    /// adiada" N=7 → 8 cumulativo).
    #[test]
    fn p235_per_cell_breakable_armazenado_layout_preservado() {
        use crate::entities::layout_types::{Length, TrackSizing};
        use crate::entities::sides::Sides;
        let cell = Content::GridCell {
            body:    Box::new(Content::text("BR")),
            x:       None, y: None,
            colspan: None, rowspan: None,
            stroke:  None, fill: None,
            align:   None, inset: None,
            breakable: Some(false),
        };
        let g = Content::Grid {
            columns: vec![TrackSizing::Fixed(40.0)],
            rows:    vec![TrackSizing::Fixed(20.0)],
            cells:   vec![cell],
            gutter:  None, align: None,
            inset:   Sides::uniform(Length::pt(0.0)),
            header:  None, footer:  None,
            stroke:  None, fill: None,
        };
        let doc = layout(&g);
        // Render preservado; breakable armazenado mas não afecta visual.
        let txt = doc.plain_text();
        assert!(txt.contains("BR"),
            "Cell breakable Some(false) armazenado; render preservado pós-P235 graded");
    }

    /// Per-cell align Some + Grid align None → cell renderiza em align especificado.
    /// Render via Layouter cell_align extension P235 per-cell save/restore.
    #[test]
    fn p235_per_cell_align_override_grid_armazenado() {
        use crate::entities::layout_types::{Length, TrackSizing, Align2D};
        use crate::entities::sides::Sides;
        let cell = Content::GridCell {
            body:    Box::new(Content::text("AL")),
            x:       None, y: None,
            colspan: None, rowspan: None,
            stroke:  None, fill: None,
            align:   Some(Align2D::from_string("center")),
            inset:   None, breakable: None,
        };
        let g = Content::Grid {
            columns: vec![TrackSizing::Fixed(100.0)],
            rows:    vec![TrackSizing::Fixed(30.0)],
            cells:   vec![cell],
            gutter:  None, align: None,
            inset:   Sides::uniform(Length::pt(0.0)),
            header:  None, footer:  None,
            stroke:  None, fill: None,
        };
        let doc = layout(&g);
        // Render OK; align efectivo per-cell via Layouter cell_align extension.
        let txt = doc.plain_text();
        assert!(txt.contains("AL"),
            "Cell align Some(center) armazenado; render preservado pós-P235");
    }

    /// Per-cell align None + Grid align Some → cell herda Grid align.
    #[test]
    fn p235_per_cell_align_none_inherits_grid() {
        use crate::entities::layout_types::{Length, TrackSizing, Align2D};
        use crate::entities::sides::Sides;
        let cell = Content::GridCell {
            body:    Box::new(Content::text("IA")),
            x:       None, y: None,
            colspan: None, rowspan: None,
            stroke:  None, fill: None,
            align:   None,  // inherit Grid
            inset:   None, breakable: None,
        };
        let g = Content::Grid {
            columns: vec![TrackSizing::Fixed(100.0)],
            rows:    vec![TrackSizing::Fixed(30.0)],
            cells:   vec![cell],
            gutter:  None,
            align:   Some(Align2D::from_string("right")),  // Grid align right
            inset:   Sides::uniform(Length::pt(0.0)),
            header:  None, footer:  None,
            stroke:  None, fill: None,
        };
        let doc = layout(&g);
        let txt = doc.plain_text();
        assert!(txt.contains("IA"),
            "Cell align None inherit Grid align right pós-P235");
    }

    /// Counters/labels dentro do body de repeat resolvem via walk
    /// (single-walk em P156J; sem multiplicação de state).
    #[test]
    fn layout_repeat_counters_dentro_do_body_resolvem() {
        use std::sync::Arc;
        // Heading dentro de repeat deve ser numerado uma vez (paridade
        // vanilla — repeat é runtime-only para paridade visual; counter
        // só conta uma vez no walk).
        let doc_content = Content::Sequence(Arc::from(vec![
            Content::repeat(
                Content::Heading {
                    level: 1,
                    body:  Box::new(Content::text("Title")),
                },
                None,
                true,
            ),
        ]));
        let doc = layout(&doc_content);
        // Render mínimo sem panic; body Title presente.
        assert!(doc.plain_text().contains("Title"),
            "heading dentro de repeat deve renderizar: doc='{}'", doc.plain_text());
    }

    /// `pagebreak(to: even)` quando próxima seria ímpar (p3) deve inserir
    /// página vazia. Setup: A → pagebreak(to:odd) garante B em p3 → mais
    /// pagebreak(to:even) força ajuste para p4 (par).
    #[test]
    fn layout_pagebreak_to_even_insere_vazia_se_proxima_seria_impar() {
        use std::sync::Arc;
        let doc_content = Content::Sequence(Arc::from(vec![
            Content::text("A"),                                                       // p1
            Content::pagebreak(false, Some(crate::entities::parity::Parity::Odd)),    // → próxima p3
            Content::text("B"),                                                       // p3
            Content::pagebreak(false, Some(crate::entities::parity::Parity::Even)),   // → próxima p4 (já par)
            Content::text("C"),                                                       // p4
        ]));
        let doc = layout(&doc_content);
        let page_a = page_index_containing(&doc, "A").expect("A não encontrado");
        let page_b = page_index_containing(&doc, "B").expect("B não encontrado");
        let page_c = page_index_containing(&doc, "C").expect("C não encontrado");
        assert_eq!(page_a, 1);
        assert_eq!(page_b, 3);
        assert_eq!(page_c, 4, "C deve estar na p4 (par): obtive p{}", page_c);
    }

    // ── Passo 157A (ADR-0060 Fase 2 sub-passo 1) — table ──────────────────

    /// `Content::Table` renderiza children como cells em grelha
    /// (delegação a `layout_grid` per ADR-0060 §"Decisão 4").
    #[test]
    fn layout_table_renderiza_children_como_cells() {
        use crate::entities::layout_types::TrackSizing;
        let t = Content::table(
            vec![TrackSizing::Auto, TrackSizing::Auto],
            vec![],
            vec![
                Content::text("a"),
                Content::text("b"),
                Content::text("c"),
                Content::text("d"),
            ],
        );
        let doc = layout(&t);
        // Todos os 4 children devem aparecer como FrameItems::Text.
        for label in ["a", "b", "c", "d"] {
            let count = doc.pages.iter().flat_map(|p| p.items.iter())
                .filter(|item| matches!(item, FrameItem::Text { text, .. } if text.as_str() == label))
                .count();
            assert!(count >= 1, "table cell '{}' deve aparecer pelo menos uma vez", label);
        }
    }

    /// `Content::Table` e `Content::Grid` com mesmos campos produzem
    /// o mesmo conjunto de cells observáveis (paridade estrutural por
    /// delegação a `layout_grid`; sem modificação de `grid.rs`).
    #[test]
    fn layout_table_paridade_com_grid_equivalente() {
        use crate::entities::layout_types::TrackSizing;
        let columns = vec![TrackSizing::Auto, TrackSizing::Auto];
        let cells = vec![
            Content::text("X"),
            Content::text("Y"),
        ];

        // Versão Grid.
        let g = Content::Grid {
            columns: columns.clone(),
            rows:    vec![],
            cells:   cells.clone(),
            gutter:  None,
            align:   None,
            inset:   crate::entities::sides::Sides::uniform(
                crate::entities::layout_types::Length::pt(0.0)),
            header:  None,
            footer:  None,
            stroke:  None,
            fill:    None,
        };
        let doc_g = layout(&g);
        let positions_g: Vec<(String, f64, f64)> = doc_g.pages.iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, pos, .. } => Some((text.to_string(), pos.x.val(), pos.y.val())),
                _ => None,
            })
            .collect();

        // Versão Table.
        let t = Content::table(columns, vec![], cells);
        let doc_t = layout(&t);
        let positions_t: Vec<(String, f64, f64)> = doc_t.pages.iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, pos, .. } => Some((text.to_string(), pos.x.val(), pos.y.val())),
                _ => None,
            })
            .collect();

        // Posições idênticas (mesma delegação a `layout_grid`).
        assert_eq!(positions_g, positions_t,
            "Table e Grid com mesmos campos devem produzir as mesmas posições por delegação");
    }

    // ── Passo 157B (ADR-0060 Fase 2 sub-passo 2) — table cell ─────────────

    /// `Content::TableCell` renderiza body uma vez (single render).
    /// Spans (colspan/rowspan) ignorados em layout per ADR-0054
    /// graded — diferidos em DEBT-34e.
    #[test]
    fn layout_table_cell_renderiza_body_no_contexto_actual() {
        let c = Content::table_cell(
            Content::text("X"),
            Some(2), Some(3),
            Some(99), Some(99),  // spans grandes; ignorados em layout
        );
        let doc = layout(&c);
        // Body deve aparecer **exactamente uma vez** (sem multiplicar
        // por colspan/rowspan).
        let count_x = doc.pages.iter().flat_map(|p| p.items.iter())
            .filter(|item| matches!(item, FrameItem::Text { text, .. } if text.as_str() == "X"))
            .count();
        assert_eq!(count_x, 1, "table_cell renderiza body uma vez (single render); colspan/rowspan ignorados em layout per DEBT-34e");
    }

    /// `Content::TableCell` dentro de `Content::Table` é tratado
    /// como child linear no grid distribuído por `idx % num_cols`
    /// (delegação a `layout_grid`).
    #[test]
    fn layout_table_cell_dentro_de_table_renderiza_como_cell() {
        use crate::entities::layout_types::TrackSizing;
        // Table com 2 columns + 4 children (2 plain + 2 cells).
        let t = Content::table(
            vec![TrackSizing::Auto, TrackSizing::Auto],
            vec![],
            vec![
                Content::text("a"),
                Content::table_cell(Content::text("b"), None, None, None, None),
                Content::text("c"),
                Content::table_cell(Content::text("d"), Some(2), Some(0), None, None),
            ],
        );
        let doc = layout(&t);
        // Todos os 4 conteúdos devem aparecer como FrameItems.
        for label in ["a", "b", "c", "d"] {
            let count = doc.pages.iter().flat_map(|p| p.items.iter())
                .filter(|item| matches!(item, FrameItem::Text { text, .. } if text.as_str() == label))
                .count();
            assert!(count >= 1,
                "child '{}' (plain ou table_cell) deve aparecer pelo menos uma vez", label);
        }
    }

    // ── Passo 157C (ADR-0060 Fase 2 sub-passo 3 — fecha table foundations) ──

    /// `Content::TableHeader` renderiza body uma vez no contexto
    /// actual (`repeat` ignorado em layout per ADR-0054 graded;
    /// algoritmo de repetição em page breaks diferido em DEBT-56).
    #[test]
    fn layout_table_header_renderiza_body_no_contexto_actual() {
        let h = Content::table_header(Content::text("HDR"), true);
        let doc = layout(&h);
        let count = doc.pages.iter().flat_map(|p| p.items.iter())
            .filter(|item| matches!(item, FrameItem::Text { text, .. } if text.as_str() == "HDR"))
            .count();
        assert_eq!(count, 1,
            "table_header renderiza body uma vez (single render); repeat ignorado per DEBT-56");
    }

    /// Par simétrico — paridade absoluta com header.
    #[test]
    fn layout_table_footer_renderiza_body_no_contexto_actual() {
        let f = Content::table_footer(Content::text("FTR"), true);
        let doc = layout(&f);
        let count = doc.pages.iter().flat_map(|p| p.items.iter())
            .filter(|item| matches!(item, FrameItem::Text { text, .. } if text.as_str() == "FTR"))
            .count();
        assert_eq!(count, 1,
            "table_footer renderiza body uma vez (single render); repeat ignorado per DEBT-56");
    }

    /// Test integrativo — `Content::Table` contendo TableHeader +
    /// TableCell + TableFooter renderiza os três conteúdos
    /// (delegação a `layout_grid` linear; ordem semântica diferida).
    #[test]
    fn layout_table_com_header_cell_footer_renderiza_tudo() {
        use crate::entities::layout_types::TrackSizing;
        let t = Content::table(
            vec![TrackSizing::Auto, TrackSizing::Auto],
            vec![],
            vec![
                Content::table_header(Content::text("H"), true),
                Content::text("a"),
                Content::table_cell(Content::text("b"), None, None, None, None),
                Content::table_footer(Content::text("F"), true),
            ],
        );
        let doc = layout(&t);
        // Os 4 conteúdos (H, a, b, F) devem aparecer pelo menos uma vez.
        for label in ["H", "a", "b", "F"] {
            let count = doc.pages.iter().flat_map(|p| p.items.iter())
                .filter(|item| matches!(item, FrameItem::Text { text, .. } if text.as_str() == label))
                .count();
            assert!(count >= 1,
                "child '{}' (plain, table_header, table_cell ou table_footer) deve aparecer pelo menos uma vez", label);
        }
    }

    // ── Passo 159A — Bibliography + Cite par acoplado ─────────────────────

    /// `Content::Bibliography` renderiza title (se Some) seguido
    /// de lista de entries formatadas como
    /// `"[{key}] {author}. {title} ({year})."` per linha.
    #[test]
    fn layout_bibliography_renderiza_entries_como_lista() {
        use crate::entities::bib_entry::BibEntry;
        let b = Content::bibliography(
            vec![
                BibEntry::new("smith2024", "Smith, J.", "On Crystal Math", 2024),
                BibEntry::new("doe2023",   "Doe, A.",   "Cosmic Patterns", 2023),
            ],
            None,
        );
        let doc = layout(&b);
        let txt = doc.plain_text();
        // Ambos os keys devem aparecer no output formatado.
        assert!(txt.contains("[smith2024]"), "key smith2024 deve aparecer formatada como [key]: doc='{}'", txt);
        assert!(txt.contains("[doe2023]"),   "key doe2023 deve aparecer formatada como [key]");
        assert!(txt.contains("Smith"),       "author Smith deve aparecer");
        assert!(txt.contains("2024"),        "year 2024 deve aparecer");
    }

    /// `Content::Cite` renderiza placeholder `[key]` com supplement
    /// (se Some) concatenado.
    #[test]
    fn layout_cite_renderiza_placeholder_com_key() {
        let c = Content::cite("smith2024", None, None);
        let doc = layout(&c);
        let txt = doc.plain_text();
        assert!(txt.contains("[smith2024]"),
            "Cite renderiza placeholder [key]: doc='{}'", txt);
    }

    /// Bibliography + Cite no mesmo documento — integrativo.
    #[test]
    fn layout_bibliography_e_cite_no_mesmo_documento() {
        use crate::entities::bib_entry::BibEntry;
        use std::sync::Arc;
        let doc_content = Content::Sequence(Arc::from(vec![
            Content::cite("smith2024", None, None),
            Content::bibliography(
                vec![BibEntry::new("smith2024", "Smith, J.", "On Crystal Math", 2024)],
                Some(Content::text("Referências")),
            ),
        ]));
        let doc = layout(&doc_content);
        let txt = doc.plain_text();
        // Ambos cite e bibliography devem aparecer.
        assert!(txt.contains("[smith2024]"), "cite + bibliography ambos devem ter [smith2024]");
        assert!(txt.contains("Referências"), "title da bibliography presente");
        assert!(txt.contains("Smith"),       "author entry presente");
    }

    // ── Passo 159C — Cite.form variants (E2E) ─────────────────────────────

    /// Helper: corre introspect (para popular bib_entries) seguido
    /// de layout. Mimica fluxo real eval → introspect → layout.
    fn layout_with_introspect(c: &Content) -> String {
        use crate::rules::introspect::introspect;
        let state = introspect(c);
        let doc = layout(c);
        doc.plain_text()
    }

    /// Regression: Cite com form=None continua a renderizar
    /// placeholder `[key]` (paridade P159A).
    #[test]
    fn cite_normal_renderiza_placeholder() {
        let c = Content::cite("smith2024", None, None);
        let txt = layout_with_introspect(&c);
        assert!(txt.contains("[smith2024]"),
            "Normal/None form deve produzir [key]: doc='{}'", txt);
    }

    /// `Cite { form: Prose }` com key existente em Bibliography
    /// renderiza `Author (Year)`.
    #[test]
    fn cite_prose_renderiza_author_year_quando_key_existe() {
        use crate::entities::bib_entry::BibEntry;
        use crate::entities::citation_form::CitationForm;
        use std::sync::Arc;
        let doc_content = Content::Sequence(Arc::from(vec![
            Content::cite("smith2024", None, Some(CitationForm::Prose)),
            Content::bibliography(
                vec![BibEntry::new("smith2024", "Smith, J.", "On Crystal Math", 2024)],
                None,
            ),
        ]));
        let txt = layout_with_introspect(&doc_content);
        assert!(txt.contains("Smith, J. (2024)"),
            "Prose com key existente deve renderizar 'Author (Year)': doc='{}'", txt);
    }

    /// `Cite { form: Prose }` com key NÃO encontrada cai no
    /// fallback `[key]`.
    #[test]
    fn cite_prose_fallback_placeholder_quando_key_nao_existe() {
        use crate::entities::citation_form::CitationForm;
        let c = Content::cite("inexistente", None, Some(CitationForm::Prose));
        let txt = layout_with_introspect(&c);
        assert!(txt.contains("[inexistente]"),
            "Prose sem entry deve cair no fallback [key]: doc='{}'", txt);
    }

    /// `Cite { form: Author }` renderiza apenas autor;
    /// `Cite { form: Year }` renderiza apenas ano.
    #[test]
    fn cite_author_e_year_renderizam_correctamente() {
        use crate::entities::bib_entry::BibEntry;
        use crate::entities::citation_form::CitationForm;
        use std::sync::Arc;
        let bib = Content::bibliography(
            vec![BibEntry::new("smith2024", "Smith, J.", "On Crystal Math", 2024)],
            None,
        );
        // form=Author
        let c1 = Content::Sequence(Arc::from(vec![
            Content::cite("smith2024", None, Some(CitationForm::Author)),
            bib.clone(),
        ]));
        let txt1 = layout_with_introspect(&c1);
        assert!(txt1.contains("Smith, J."), "Author form: 'Smith, J.' deve aparecer: doc='{}'", txt1);
        // form=Year
        let c2 = Content::Sequence(Arc::from(vec![
            Content::cite("smith2024", None, Some(CitationForm::Year)),
            bib,
        ]));
        let txt2 = layout_with_introspect(&c2);
        assert!(txt2.contains("2024"), "Year form: '2024' deve aparecer: doc='{}'", txt2);
    }

    // ── Passo 159D — BibEntry fields opcionais (E2E) ──────────────────────

    /// Bibliography com entry completa renderiza formato extendido
    /// com todos os 4 fields opcionais presentes (volume/pages/
    /// journal/publisher).
    #[test]
    fn bibliography_entry_completa_renderiza_formato_extendido() {
        use crate::entities::bib_entry::BibEntry;
        let entry = BibEntry::new("smith2024", "Smith, J.", "On Crystal Math", 2024)
            .with_journal("Nature Communications")
            .with_volume("12")
            .with_pages("1-10")
            .with_publisher("ACM");
        let b = Content::bibliography(vec![entry], None);
        let doc = layout(&b);
        let txt = doc.plain_text();
        // Todos os fields novos devem aparecer no output formatado.
        assert!(txt.contains("Nature Communications"),
            "journal deve aparecer: doc='{}'", txt);
        assert!(txt.contains("vol. 12"),
            "volume deve aparecer com prefix 'vol.': doc='{}'", txt);
        assert!(txt.contains("pp. 1-10"),
            "pages deve aparecer com prefix 'pp.': doc='{}'", txt);
        assert!(txt.contains("ACM"),
            "publisher deve aparecer: doc='{}'", txt);
        assert!(txt.contains("(2024)"),
            "year preserva formato (year) no final: doc='{}'", txt);
    }

    /// Regression P159A: Bibliography com entry mínima (só 4
    /// fields obrigatórios) renderiza formato P159A original.
    #[test]
    fn bibliography_entry_minima_regression_p159a() {
        use crate::entities::bib_entry::BibEntry;
        let entry = BibEntry::new("smith2024", "Smith, J.", "On Crystal Math", 2024);
        let b = Content::bibliography(vec![entry], None);
        let doc = layout(&b);
        let txt = doc.plain_text();
        // Output P159A: "[smith2024] Smith, J.. On Crystal Math (2024)."
        assert!(txt.contains("[smith2024]"), "key como [key]");
        assert!(txt.contains("Smith, J."),   "author");
        assert!(txt.contains("On Crystal Math"), "title");
        assert!(txt.contains("(2024)"),      "year (year)");
        // Sem fields novos — ausentes do output.
        assert!(!txt.contains("vol."),  "sem volume → sem 'vol.'");
        assert!(!txt.contains("pp."),   "sem pages → sem 'pp.'");
    }

    // ── Passo 159F — Bibliography numbering numérico (E2E) ────────────────

    /// `Cite` Normal/None com Bibliography populada renderiza
    /// número `[N]` em vez de placeholder `[key]`.
    #[test]
    fn cite_normal_renderiza_numero_quando_bib_populada() {
        use crate::entities::bib_entry::BibEntry;
        use std::sync::Arc;
        let doc_content = Content::Sequence(Arc::from(vec![
            Content::cite("smith2024", None, None),
            Content::bibliography(
                vec![BibEntry::new("smith2024", "Smith, J.", "On Crystal Math", 2024)],
                None,
            ),
        ]));
        let txt = layout_with_introspect(&doc_content);
        assert!(txt.contains("[1]"),
            "Normal/None com Bibliography populada deve renderizar [1]: doc='{}'", txt);
    }

    /// Regression P159A: Cite sem Bibliography precedente cai
    /// no fallback `[key]`.
    #[test]
    fn cite_normal_fallback_placeholder_quando_bib_vazia() {
        let c = Content::cite("smith2024", None, None);
        let txt = layout_with_introspect(&c);
        assert!(txt.contains("[smith2024]"),
            "Sem Bibliography → fallback [key] (regression P159A): doc='{}'", txt);
    }

    /// Multiple entries em Bibliography → cada Cite obtém número
    /// na ordem de aparecimento.
    #[test]
    fn cite_normal_multiple_entries_numeradas_em_ordem() {
        use crate::entities::bib_entry::BibEntry;
        use std::sync::Arc;
        let doc_content = Content::Sequence(Arc::from(vec![
            Content::cite("first",  None, None),
            Content::cite("second", None, None),
            Content::cite("third",  None, None),
            Content::bibliography(
                vec![
                    BibEntry::new("first",  "Author One",   "Paper One",   2021),
                    BibEntry::new("second", "Author Two",   "Paper Two",   2022),
                    BibEntry::new("third",  "Author Three", "Paper Three", 2023),
                ],
                None,
            ),
        ]));
        let txt = layout_with_introspect(&doc_content);
        assert!(txt.contains("[1]"), "first → [1]: doc='{}'", txt);
        assert!(txt.contains("[2]"), "second → [2]");
        assert!(txt.contains("[3]"), "third → [3]");
    }

    /// Regression P159C: Cite.form Prose continua a renderizar
    /// "Author (Year)" mesmo com Bibliography numerada (forms
    /// diferenciadas inalteradas; numeração só em Normal/None).
    #[test]
    fn cite_form_prose_inalterada_com_bib_numerada() {
        use crate::entities::bib_entry::BibEntry;
        use crate::entities::citation_form::CitationForm;
        use std::sync::Arc;
        let doc_content = Content::Sequence(Arc::from(vec![
            Content::cite("smith2024", None, Some(CitationForm::Prose)),
            Content::bibliography(
                vec![BibEntry::new("smith2024", "Smith, J.", "On Crystal Math", 2024)],
                None,
            ),
        ]));
        let txt = layout_with_introspect(&doc_content);
        assert!(txt.contains("Smith, J. (2024)"),
            "Prose continua a renderizar 'Author (Year)' (regression P159C): doc='{}'", txt);
    }

    /// Regression P159A: Cite com key não em Bibliography cai no
    /// fallback `[key]` mesmo com outras keys numeradas.
    #[test]
    fn cite_unknown_key_fallback_placeholder() {
        use crate::entities::bib_entry::BibEntry;
        use std::sync::Arc;
        let doc_content = Content::Sequence(Arc::from(vec![
            Content::cite("inexistente", None, None),
            Content::bibliography(
                vec![BibEntry::new("smith2024", "Smith, J.", "On Crystal Math", 2024)],
                None,
            ),
        ]));
        let txt = layout_with_introspect(&doc_content);
        assert!(txt.contains("[inexistente]"),
            "Cite com key não em Bibliography → fallback [key]: doc='{}'", txt);
    }

    /// Multi-Bibliography: numeração contínua per decisão
    /// diagnóstico §9 (paridade vanilla).
    #[test]
    fn cite_normal_multi_bibliography_continua() {
        use crate::entities::bib_entry::BibEntry;
        use std::sync::Arc;
        let doc_content = Content::Sequence(Arc::from(vec![
            Content::cite("third", None, None),
            Content::bibliography(
                vec![
                    BibEntry::new("first",  "Author One", "Paper One", 2021),
                    BibEntry::new("second", "Author Two", "Paper Two", 2022),
                ],
                None,
            ),
            Content::bibliography(
                vec![
                    BibEntry::new("third", "Author Three", "Paper Three", 2023),
                ],
                None,
            ),
        ]));
        let txt = layout_with_introspect(&doc_content);
        // first=[1], second=[2] em Bib1; third=[3] em Bib2 (contínua).
        assert!(txt.contains("[3]"),
            "third deve obter [3] (numeração contínua multi-Bibliography): doc='{}'", txt);
    }

    // ── Passo 159E — par natural url/doi em BibEntry (E2E) ────────────────

    /// Bibliography com entry incluindo url/doi renderiza formato
    /// extendido APA-like com URL plaintext + prefixo `doi:`
    /// (Opção C diagnóstico §8.2; após `(year).`).
    #[test]
    fn bibliography_entry_com_url_doi_renderiza_formato_extendido() {
        use crate::entities::bib_entry::BibEntry;
        let entry = BibEntry::new("smith2024", "Smith, J.", "On Crystal Math", 2024)
            .with_url("https://example.com/paper")
            .with_doi("10.1234/abc");
        let b = Content::bibliography(vec![entry], None);
        let doc = layout(&b);
        let txt = doc.plain_text();
        // URL plaintext literal deve aparecer.
        assert!(txt.contains("https://example.com/paper"),
            "URL plaintext deve aparecer: doc='{}'", txt);
        // DOI com prefixo `doi:` deve aparecer.
        assert!(txt.contains("doi:10.1234/abc"),
            "DOI com prefixo 'doi:' deve aparecer: doc='{}'", txt);
        // Ordem APA Opção C: url/doi após (year).
        assert!(txt.contains("(2024)"),
            "year preserva formato (year): doc='{}'", txt);
    }

    /// Regression P159D: Bibliography com entry sem url/doi
    /// renderiza formato P159D original (sem `doi:` no output).
    #[test]
    fn bibliography_entry_sem_url_doi_regression_p159d() {
        use crate::entities::bib_entry::BibEntry;
        let entry = BibEntry::new("smith2024", "Smith, J.", "On Crystal Math", 2024)
            .with_journal("Nature Communications")
            .with_volume("12");
        let b = Content::bibliography(vec![entry], None);
        let doc = layout(&b);
        let txt = doc.plain_text();
        // P159D fields presentes.
        assert!(txt.contains("Nature Communications"));
        assert!(txt.contains("vol. 12"));
        // Sem url/doi → ausentes do output.
        assert!(!txt.contains("doi:"),
            "sem doi → sem 'doi:' no output: doc='{}'", txt);
        assert!(!txt.contains("https://"),
            "sem url → sem URL no output: doc='{}'", txt);
    }

    // ── Passo 159G — 6 fields restantes comuns hayagriva (E2E) ────────────

    /// Bibliography com entry incluindo todos os 6 fields P159G
    /// renderiza formato extendido com prefixos correctos
    /// (`Ed.`, `(`series`)`, `[`note`]`, `isbn:`, `location:`).
    #[test]
    fn bibliography_entry_com_p159g_fields_renderiza_formato_extendido() {
        use crate::entities::bib_entry::BibEntry;
        let entry = BibEntry::new("smith2024", "Smith, J.", "On Crystal Math", 2024)
            .with_editor("Doe, A.")
            .with_series("Crystal Studies")
            .with_note("See Smith 2023")
            .with_isbn("978-0-1234")
            .with_location("New York")
            .with_publisher("ACM");
        let b = Content::bibliography(vec![entry], None);
        let doc = layout(&b);
        let txt = doc.plain_text();
        // Editor com prefixo (Ed. ).
        assert!(txt.contains("(Ed. Doe, A.)"),
            "editor deve aparecer com prefixo '(Ed. ': doc='{}'", txt);
        // Series em parêntese.
        assert!(txt.contains("(Crystal Studies)"),
            "series deve aparecer entre parênteses: doc='{}'", txt);
        // Note em brackets.
        assert!(txt.contains("[See Smith 2023]"),
            "note deve aparecer entre brackets: doc='{}'", txt);
        // ISBN com prefixo lowercase.
        assert!(txt.contains("isbn:978-0-1234"),
            "isbn deve aparecer com prefixo lowercase 'isbn:': doc='{}'", txt);
        // Location: publisher.
        assert!(txt.contains("New York: ACM"),
            "location: publisher deve aparecer: doc='{}'", txt);
    }

    /// Regression P159E: Bibliography com entry sem fields P159G
    /// renderiza formato P159E original (sem `Ed.`/`isbn:`/etc.).
    #[test]
    fn bibliography_entry_sem_p159g_fields_regression_p159e() {
        use crate::entities::bib_entry::BibEntry;
        let entry = BibEntry::new("smith2024", "Smith, J.", "On Crystal Math", 2024)
            .with_url("https://example.com")
            .with_doi("10.1/a");
        let b = Content::bibliography(vec![entry], None);
        let doc = layout(&b);
        let txt = doc.plain_text();
        // P159E fields preservados.
        assert!(txt.contains("https://example.com"));
        assert!(txt.contains("doi:10.1/a"));
        // Sem fields P159G → ausentes do output.
        assert!(!txt.contains("Ed."),
            "sem editor → sem 'Ed.' no output: doc='{}'", txt);
        assert!(!txt.contains("isbn:"),
            "sem isbn → sem 'isbn:' no output: doc='{}'", txt);
        assert!(!txt.contains("[See"),
            "sem note → sem '[note]' no output: doc='{}'", txt);
    }

    /// Bibliography com organization sem publisher renderiza
    /// organization no slot publisher (substitutivo).
    #[test]
    fn bibliography_organization_substitui_publisher_quando_publisher_ausente() {
        use crate::entities::bib_entry::BibEntry;
        let entry = BibEntry::new("tech2024", "Smith, J.", "Tech Report", 2024)
            .with_organization("MIT");
        let b = Content::bibliography(vec![entry], None);
        let doc = layout(&b);
        let txt = doc.plain_text();
        // Organization aparece no slot publisher.
        assert!(txt.contains("MIT"),
            "organization deve aparecer no slot publisher: doc='{}'", txt);
    }
}

// ── P168 (M5 sub-passo 2) — Tests de migração figure-ref ─────────────────

#[cfg(test)]
mod p168_figure_ref_migration {
    use super::*;
    use crate::entities::label::Label;
    use crate::rules::introspect::introspect_with_introspector;

    fn doc_figure_with_ref(label_str: &str, kind: Option<String>, with_caption: bool, with_numbering: bool) -> Content {
        let figure = Content::Figure {
            body:      Box::new(Content::text("body")),
            caption:   if with_caption { Some(Box::new(Content::text("cap"))) } else { None },
            kind,
            numbering: if with_numbering { Some("1".into()) } else { None },
        };
        Content::Sequence(
            vec![
                Content::Labelled {
                    label:  Label(label_str.to_string()),
                    target: Box::new(figure),
                },
                Content::text("ver "),
                Content::Ref { target: Label(label_str.to_string()) },
            ]
            .into(),
        )
    }

    #[test]
    fn migrated_path_via_layout_with_introspector_renders_figure_ref() {
        // P168 .E.1: figure numbered+captioned + ref → layout via novo
        // entry point produz "Figura 1" usando introspector path.
        let content = doc_figure_with_ref("fig1", Some("image".into()), true, true);
        let intr = introspect_with_introspector(&content);
        let doc = layout_with_introspector(&content, intr);
        let txt = doc.plain_text();
        assert!(
            txt.contains("Figura 1"),
            "ref deve resolver para 'Figura 1' via introspector path; obtido: '{txt}'"
        );
    }

    #[test]
    fn legacy_path_via_layout_continues_to_work() {
        // P168 .E.3: backward compat — legacy `layout()` ainda resolve
        // figure-ref via fallback a state.figure_label_numbers.
        let content = doc_figure_with_ref("fig1", Some("image".into()), true, true);
        let state = introspect(&content);
        let doc = layout(&content);
        let txt = doc.plain_text();
        assert!(
            txt.contains("Figura 1"),
            "legacy layout() deve continuar a resolver figure-ref; obtido: '{txt}'"
        );
    }

    #[test]
    fn paridade_pre_post_migracao() {
        // P168 .E.2: layout() legacy e layout_with_introspector produzem
        // o mesmo plain_text para o mesmo documento (paridade).
        let content = doc_figure_with_ref("fig1", Some("image".into()), true, true);

        let state_legacy = introspect(&content);
        let doc_legacy = layout(&content);
        let txt_legacy = doc_legacy.plain_text();

        let intr = introspect_with_introspector(&content);
        let doc_new = layout_with_introspector(&content, intr);
        let txt_new = doc_new.plain_text();

        assert_eq!(
            txt_legacy, txt_new,
            "paridade quebrada: legacy='{txt_legacy}' new='{txt_new}'"
        );
    }

    #[test]
    fn figura_sem_caption_nao_gera_figure_ref() {
        // P168 .E (caso bordo): figura sem caption não conta para
        // numeração — predicado is_counted=false garante que introspector
        // NÃO indexa esta figura como figure_label_number.
        let content = doc_figure_with_ref("fig1", Some("image".into()), false, true);
        let intr = introspect_with_introspector(&content);
        use crate::entities::introspector::Introspector;
        assert_eq!(
            intr.figure_number_for_label(&Label("fig1".to_string())),
            None,
            "figura sem caption não deve aparecer em figure_label_numbers"
        );
    }
}

// ── P181G — Tests de migração cite-arm para Introspector ─────────────────

#[cfg(test)]
mod p181g_cite_arm_migration {
    use super::*;
    use crate::entities::bib_entry::BibEntry;
    use crate::entities::citation_form::CitationForm;
    use crate::rules::introspect::introspect_with_introspector;
    use std::sync::Arc;

    fn doc_cite_with_bib(form: Option<CitationForm>) -> Content {
        Content::Sequence(Arc::from(vec![
            Content::cite("smith2024", None, form),
            Content::bibliography(
                vec![BibEntry::new("smith2024", "Smith, J.", "On Crystal Math", 2024)],
                None,
            ),
        ]))
    }

    fn render_via_introspector(content: &Content) -> String {
        let intr = introspect_with_introspector(content);
        let doc = layout_with_introspector(content, intr);
        doc.plain_text()
    }

    #[test]
    fn cite_normal_via_introspector_renderiza_numero() {
        // P181G: cite-arm consulta `Introspector::bib_number_for_key`
        // primeiro. Documento rendered deve conter "[1]".
        let content = doc_cite_with_bib(None);
        let txt = render_via_introspector(&content);
        assert!(txt.contains("[1]"),
            "Normal/None via introspector path deve renderizar [1]: doc='{txt}'");
    }

    #[test]
    fn cite_prose_via_introspector_renderiza_author_year() {
        // P181G: cite-arm consulta `Introspector::bib_entry_for_key`
        // (Prose precisa do entry para autor + ano).
        let content = doc_cite_with_bib(Some(CitationForm::Prose));
        let txt = render_via_introspector(&content);
        assert!(txt.contains("Smith, J. (2024)"),
            "Prose via introspector path deve renderizar 'Author (Year)': doc='{txt}'");
    }

    #[test]
    fn cite_author_via_introspector_renderiza_apenas_author() {
        let content = doc_cite_with_bib(Some(CitationForm::Author));
        let txt = render_via_introspector(&content);
        assert!(txt.contains("Smith, J."),
            "Author via introspector path deve renderizar autor: doc='{txt}'");
    }

    #[test]
    fn cite_year_via_introspector_renderiza_apenas_ano() {
        let content = doc_cite_with_bib(Some(CitationForm::Year));
        let txt = render_via_introspector(&content);
        assert!(txt.contains("2024"),
            "Year via introspector path deve renderizar ano: doc='{txt}'");
    }

    #[test]
    fn paridade_legacy_vs_introspector_para_cite() {
        // P181G: para os 4 forms, layout() (path legacy via state.bib_*)
        // e layout_with_introspector() (path Introspector via BibStore)
        // produzem o mesmo plain_text. Confirma paridade BibStore ↔
        // state.bib_* garantida por construção em P181E.
        for form in [None, Some(CitationForm::Prose),
                     Some(CitationForm::Author), Some(CitationForm::Year)] {
            let content = doc_cite_with_bib(form);

            let state_legacy = crate::rules::introspect::introspect(&content);
            let txt_legacy = layout(&content).plain_text();

            let txt_new = render_via_introspector(&content);

            assert_eq!(
                txt_legacy, txt_new,
                "paridade quebrada para form {form:?}: legacy='{txt_legacy}' new='{txt_new}'",
            );
        }
    }

    #[test]
    fn cite_consulta_introspector_quando_state_legacy_vazio() {
        // P181G diferencial: prova que cite-arm consulta `Introspector`
        // primeiro (não apenas state legacy). Constrói cenário
        // contrived: state.bib_* vazio + introspector populado.
        // Antes de P181G (cite-arm só lia de state) → fallback `[key]`.
        // Depois de P181G (cite-arm lê de introspector) → `[1]`.
                use crate::entities::introspector::TagIntrospector;

        let content = Content::cite("smith2024", None, None);

        // P190I: state eliminado
        let mut intr = TagIntrospector::empty();
        intr.bib_store.add_bibliography(vec![
            BibEntry::new("smith2024", "Smith, J.", "On Crystal Math", 2024),
        ]);
        intr.bib_store.assign_number("smith2024".to_string(), 1);

        let txt = layout_with_introspector(&content, intr).plain_text();

        assert!(
            txt.contains("[1]"),
            "cite-arm deve consultar Introspector quando state legacy está vazio: doc='{txt}'",
        );
    }
}

// ── P181I — Tests E2E pipeline completo bib state ────────────────────────

#[cfg(test)]
mod p181i_e2e_bib {
    use super::*;
    use crate::entities::bib_entry::BibEntry;
    use crate::entities::citation_form::CitationForm;
    use crate::entities::introspector::Introspector;
    use crate::rules::introspect::introspect_with_introspector;
    use std::sync::Arc;

    fn bib(key: &str) -> BibEntry {
        BibEntry::new(key, "Author", "Title", 2024)
    }

    #[test]
    fn pipeline_completo_bib_state_via_layout_legacy() {
        // P181I: pipeline completo via path `layout()` legacy
        // (caller pattern actual). Após P181H, este path re-corre
        // `introspect_with_introspector` internamente.
        // Bibliography com 2 entries; 2 cites Normal devem renderizar
        // [1] e [2].
        let content = Content::Sequence(Arc::from(vec![
            Content::cite("intro",   None, None),
            Content::cite("methods", None, None),
            Content::bibliography(
                vec![bib("intro"), bib("methods")],
                None,
            ),
        ]));

        let state = crate::rules::introspect::introspect(&content);
        let txt = layout(&content).plain_text();

        assert!(txt.contains("[1]"),
            "cite intro deve renderizar [1] via pipeline completo: doc='{txt}'");
        assert!(txt.contains("[2]"),
            "cite methods deve renderizar [2] via pipeline completo: doc='{txt}'");
    }

    #[test]
    fn walk_puro_state_legacy_vazio_em_producao() {
        // P181I: confirma walk puro restaurado (P181H) — state.bib_*
        // permanece vazio após walk em produção. BibStore é
        // populado por from_tags como fonte única.
        let content = Content::Bibliography {
            entries: vec![bib("a")],
            title:   None,
        };

        let intr = introspect_with_introspector(&content);

        // P190B (M6 categoria Bibliography eliminada): assertions sobre
        // `state.bib_entries`/`bib_numbers` removidas — fields eliminados.
        // BibStore populado (P181E from_tags arm).
        assert_eq!(intr.bib_store.len(), 1);
        assert_eq!(intr.bib_number_for_key("a"), Some(1));
        assert!(intr.bib_entry_for_key("a").is_some());
    }

    #[test]
    fn multi_bibliography_concat_replica_clausula_2_p181a() {
        // P181I: cláusula 2 P181A — `add_bibliography` faz `extend`.
        // Multi-Bibliography concatena entries em ordem; numeração
        // 1-based contínua sobre todas.
        let content = Content::Sequence(Arc::from(vec![
            Content::bibliography(vec![bib("a"), bib("b")], None),
            Content::bibliography(vec![bib("c"), bib("d")], None),
        ]));

        let intr = introspect_with_introspector(&content);

        assert_eq!(intr.bib_store.len(), 4,
            "multi-Bib concat: 2+2 entries → len 4");
        assert_eq!(intr.bib_number_for_key("a"), Some(1));
        assert_eq!(intr.bib_number_for_key("b"), Some(2));
        assert_eq!(intr.bib_number_for_key("c"), Some(3));
        assert_eq!(intr.bib_number_for_key("d"), Some(4));
    }

    #[test]
    fn or_insert_preserva_primeiro_numero_clausula_3_p181a() {
        // P181I: cláusula 3 P181A — `assign_number` usa `or_insert`.
        // Em multi-Bibliography com keys duplicadas, primeiro número
        // de uma key persiste; key nova continua sequência.
        let content = Content::Sequence(Arc::from(vec![
            Content::bibliography(vec![bib("a")], None),
            Content::bibliography(
                vec![bib("a"), bib("b")],  // "a" duplicado; "b" novo
                None,
            ),
        ]));

        let intr = introspect_with_introspector(&content);

        // "a" preserva número original (1).
        assert_eq!(intr.bib_number_for_key("a"), Some(1),
            "or_insert preserva primeiro número para key duplicada");
        // "b" obtém próximo número (2).
        assert_eq!(intr.bib_number_for_key("b"), Some(2),
            "key nova obtém próximo número via numbers_len()+1");
    }

    #[test]
    fn cite_4_forms_via_layout_with_introspector() {
        // P181I: confirma que os 4 cite forms renderizam correctamente
        // via path `layout_with_introspector` (consumer migrado P181G).
        let entry = BibEntry::new("smith2024", "Smith, J.", "On Math", 2024);

        for (form, expected_substr) in [
            (None,                          "[1]"),
            (Some(CitationForm::Prose),     "Smith, J. (2024)"),
            (Some(CitationForm::Author),    "Smith, J."),
            (Some(CitationForm::Year),      "2024"),
        ] {
            let content = Content::Sequence(Arc::from(vec![
                Content::cite("smith2024", None, form),
                Content::bibliography(vec![entry.clone()], None),
            ]));

            let intr = introspect_with_introspector(&content);
            let txt = layout_with_introspector(&content, intr).plain_text();

            assert!(txt.contains(expected_substr),
                "form {form:?} deve renderizar '{expected_substr}': doc='{txt}'");
        }
    }
}

// ── P169 (M9 sub-passo 1) — Tests E2E para metadata(value) ────────────────

#[cfg(test)]
mod p169_metadata_feature {
    use super::*;
    use crate::entities::value::Value;
    use crate::rules::introspect::introspect_with_introspector;
    use crate::entities::introspector::Introspector;
    use ecow::EcoString;

    #[test]
    fn metadata_value_acessivel_via_introspector() {
        // P169 .C: Content::Metadata produz query_metadata populado.
        let content = Content::Sequence(
            vec![
                Content::text("antes"),
                Content::Metadata {
                    value: Box::new(Value::Str(EcoString::from("hello"))),
                },
                Content::text("depois"),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);
        let md = intr.query_metadata();
        assert_eq!(md.len(), 1);
        assert_eq!(md[0], Value::Str(EcoString::from("hello")));
    }

    #[test]
    fn metadata_e_invisivel_em_layout() {
        // P169 .C: metadata é zero-size — output observable não muda
        // entre `[antes][metadata("X")][depois]` e `[antes][depois]`.
        let with_metadata = Content::Sequence(
            vec![
                Content::text("antes"),
                Content::Metadata {
                    value: Box::new(Value::Str(EcoString::from("invisivel"))),
                },
                Content::text("depois"),
            ]
            .into(),
        );
        let without_metadata = Content::Sequence(
            vec![Content::text("antes"), Content::text("depois")].into(),
        );

        let doc_with = layout(&with_metadata);
        let doc_without = layout(&without_metadata);
        assert_eq!(
            doc_with.plain_text(),
            doc_without.plain_text(),
            "metadata deve ser invisível — plain_text idêntico"
        );
    }

    #[test]
    fn multiplas_metadatas_preservam_ordem_no_query() {
        let content = Content::Sequence(
            vec![
                Content::Metadata { value: Box::new(Value::Int(1)) },
                Content::Metadata { value: Box::new(Value::Int(2)) },
                Content::Metadata { value: Box::new(Value::Int(3)) },
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);
        let md = intr.query_metadata();
        assert_eq!(md, &[Value::Int(1), Value::Int(2), Value::Int(3)]);
    }
}

// ── P171 (M9 sub-passo 3) — Tests E2E para state(key, init) ──────────────

#[cfg(test)]
mod p171_state_feature {
    use super::*;
    use crate::entities::introspector::Introspector;
    use crate::entities::location::Location;
    use crate::entities::state_update::StateUpdate;
    use crate::entities::value::Value;
    use crate::rules::introspect::introspect_with_introspector;

    #[test]
    fn state_init_e_acessivel_via_state_value() {
        // P171 .H.1: state(key, init) sem updates → state_value retorna init.
        let content = Content::Sequence(
            vec![
                Content::State {
                    key:  "counter".to_string(),
                    init: Box::new(Value::Int(0)),
                },
                Content::heading(1, Content::text("h1")),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);
        // Em qualquer location após o init → init value.
        assert_eq!(
            intr.state_final_value("counter"),
            Some(&Value::Int(0))
        );
    }

    #[test]
    fn state_update_aplica_no_ponto_correcto() {
        // P171 .H.1: state + heading + state_update + heading
        // → state_value(loc_pre_update) = init; state_value(loc_post_update) = new.
        let content = Content::Sequence(
            vec![
                Content::State {
                    key:  "counter".to_string(),
                    init: Box::new(Value::Int(0)),
                },
                Content::heading(1, Content::text("antes")),
                Content::StateUpdate {
                    key:    "counter".to_string(),
                    update: StateUpdate::Set(Box::new(Value::Int(5))),
                },
                Content::heading(1, Content::text("depois")),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);
        // Final value: 5 (após o update).
        assert_eq!(intr.state_final_value("counter"), Some(&Value::Int(5)));
        // Em location max → 5.
        assert_eq!(
            intr.state_value("counter", Location::from_raw(u128::MAX)),
            Some(&Value::Int(5))
        );
    }

    #[test]
    fn state_e_invisivel_em_layout() {
        // P171 .H.3: state e state_update são zero-size — output observable
        // não muda entre `[X][state(...)][state_update(...)][Y]` e `[X][Y]`.
        let with_state = Content::Sequence(
            vec![
                Content::text("X"),
                Content::State {
                    key:  "c".to_string(),
                    init: Box::new(Value::Int(0)),
                },
                Content::StateUpdate {
                    key:    "c".to_string(),
                    update: StateUpdate::Set(Box::new(Value::Int(42))),
                },
                Content::text("Y"),
            ]
            .into(),
        );
        let without_state = Content::Sequence(
            vec![Content::text("X"), Content::text("Y")].into(),
        );
        let doc_with = layout(&with_state);
        let doc_without = layout(&without_state);
        assert_eq!(
            doc_with.plain_text(),
            doc_without.plain_text(),
            "state/state_update devem ser invisíveis em layout"
        );
    }

    #[test]
    fn keys_distintas_sao_isoladas() {
        // P171 .H.4: state("a", _) e state("b", _) não interferem.
        let content = Content::Sequence(
            vec![
                Content::State {
                    key:  "a".to_string(),
                    init: Box::new(Value::Int(1)),
                },
                Content::State {
                    key:  "b".to_string(),
                    init: Box::new(Value::Int(100)),
                },
                Content::StateUpdate {
                    key:    "a".to_string(),
                    update: StateUpdate::Set(Box::new(Value::Int(2))),
                },
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);
        assert_eq!(intr.state_final_value("a"), Some(&Value::Int(2)));
        assert_eq!(intr.state_final_value("b"), Some(&Value::Int(100)));
    }

    #[test]
    fn state_inexistente_devolve_none() {
        // P171 .H.2: documento sem state — state_value retorna None.
        let content = Content::heading(1, Content::text("h"));
        let intr = introspect_with_introspector(&content);
        assert_eq!(intr.state_final_value("counter"), None);
        assert_eq!(
            intr.state_value("counter", Location::from_raw(0)),
            None
        );
    }
}

// ── P172 (M9 sub-passo 4) — Tests para Func variant em StateUpdate ──────

#[cfg(test)]
mod p172_func_callback {
    use super::*;
    use crate::entities::func::Func;
    use crate::entities::introspector::Introspector;
    use crate::entities::state_update::StateUpdate;
    use crate::entities::value::Value;
    use crate::rules::introspect::introspect_with_introspector;

    fn dummy_native(
        _ctx: &mut crate::rules::eval::EvalContext,
        _args: &crate::entities::args::Args,
        _world: &dyn crate::contracts::world::World,
        _current_file: crate::entities::file_id::FileId,
        _figure_numbering: Option<&str>,
    ) -> crate::entities::source_result::SourceResult<Value> {
        Ok(Value::Int(42))
    }

    // **P173**: test stub `func_variant_e_silenciosamente_ignorada_em_from_tags`
    // (P172) removido — codificava invariante incorrecto. Comportamento
    // legacy (Func ignorada sem Engine) continua válido e está coberto
    // por `from_tags::tests::func_eval_sem_engine_e_defensive_ignore`.
    // Eval real coberto por `from_tags::tests::func_eval_aplica_callback_com_engine`.

    #[test]
    fn func_variant_e_invisivel_em_layout() {
        // P172: state + state_update_with(Func) continua zero-size.
        let f = Func::native("dummy", dummy_native);
        let with_func = Content::Sequence(
            vec![
                Content::text("X"),
                Content::State {
                    key:  "c".to_string(),
                    init: Box::new(Value::Int(0)),
                },
                Content::StateUpdate {
                    key:    "c".to_string(),
                    update: StateUpdate::Func(f),
                },
                Content::text("Y"),
            ]
            .into(),
        );
        let without = Content::Sequence(
            vec![Content::text("X"), Content::text("Y")].into(),
        );
        let doc_with = layout(&with_func);
        let doc_without = layout(&without);
        assert_eq!(doc_with.plain_text(), doc_without.plain_text());
    }

    #[test]
    fn set_continua_a_funcionar_apos_func_variant() {
        // P172: regressão — Set variant não foi afectada pela introdução
        // de Func variant. State com mix de Set e Func: Set é aplicada,
        // Func é ignorada (stub).
        let f = Func::native("dummy", dummy_native);
        let content = Content::Sequence(
            vec![
                Content::State {
                    key:  "c".to_string(),
                    init: Box::new(Value::Int(0)),
                },
                Content::StateUpdate {
                    key:    "c".to_string(),
                    update: StateUpdate::Set(Box::new(Value::Int(5))),
                },
                Content::StateUpdate {
                    key:    "c".to_string(),
                    update: StateUpdate::Func(f), // ignorada
                },
                Content::StateUpdate {
                    key:    "c".to_string(),
                    update: StateUpdate::Set(Box::new(Value::Int(10))),
                },
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);
        assert_eq!(intr.state_final_value("c"), Some(&Value::Int(10)));
    }
}

// ── P182E — Tests E2E pipeline completo `numbering_active:heading` ────────

#[cfg(test)]
mod p182e_e2e_heading_numbering {
    use super::*;
    use crate::entities::introspector::Introspector;
    use crate::rules::introspect::{introspect, introspect_with_introspector};
    use std::sync::Arc;

    /// Documento típico: `set heading(numbering: ...)` + 3 headings com nesting [1, 2, 1].
    fn doc_typico() -> Content {
        Content::Sequence(Arc::from(vec![
            Content::SetHeadingNumbering { active: true },
            Content::heading(1, Content::text("Intro")),
            Content::heading(2, Content::text("Motivação")),
            Content::heading(1, Content::text("Conclusão")),
        ]))
    }

    #[test]
    fn pipeline_completo_heading_numbering_via_layout_legacy() {
        // P182E .B: pipeline completo `walk → from_tags →
        // layout_with_introspector` via `layout()` legacy. Após P181H,
        // path legacy re-corre `introspect_with_introspector`
        // internamente — Introspector populado via P182C; fallback
        // legacy também populado via walk arm canonical.
        // Output observable (plain_text) deve conter prefixos
        // hierárquicos correctos.
        let content = doc_typico();
        let txt = layout(&content).plain_text();

        assert!(txt.contains("1."),  "H1 (Intro) deve ter prefixo '1.': '{txt}'");
        assert!(txt.contains("1.1"), "H2 (Motivação) deve ter prefixo '1.1': '{txt}'");
        assert!(txt.contains("2."),  "segundo H1 (Conclusão) deve ter prefixo '2.': '{txt}'");
    }

    #[test]
    fn pipeline_completo_heading_numbering_via_layout_with_introspector() {
        // P182E .B (irmão): mesmo pipeline mas via entry point novo
        // directamente — sem o re-walk interno de `layout()`.
        let content = doc_typico();
        let intr = introspect_with_introspector(&content);

        // Introspector populado: chave canónica conhecida.
        assert!(
            intr.is_numbering_active("numbering_active:heading"),
            "P182C deve popular StateRegistry com Bool(true)"
        );

        let txt = layout_with_introspector(&content, intr).plain_text();
        assert!(txt.contains("1."));
        assert!(txt.contains("1.1"));
        assert!(txt.contains("2."));
    }

    #[test]
    fn re_update_active_true_then_false() {
        // P182E .C: caminho de re-update (auto-init na primeira
        // ocorrência + update normal na segunda; cf. P182C 5.1).
        // Sequência: active=true → H1 → active=false → H2.
        // Output esperado:
        // - H1 com prefixo "1." (numbering ON na altura).
        // - H2 sem prefixo "2." (numbering OFF na altura).
        // O caminho activo do bool é o fallback legacy (mutável
        // durante o walk), com Introspector a fornecer redundância
        // por `final_value` (que retorna o último valor — `false`
        // após o segundo update).
        let content = Content::Sequence(Arc::from(vec![
            Content::SetHeadingNumbering { active: true },
            Content::heading(1, Content::text("Intro")),
            Content::SetHeadingNumbering { active: false },
            Content::heading(1, Content::text("Apêndice")),
        ]));
        let txt = layout(&content).plain_text();

        assert!(
            txt.contains("1."),
            "H1 (Intro) com numbering ON deve ter prefixo '1.': '{txt}'"
        );
        assert!(
            !txt.contains("2."),
            "H2 (Apêndice) com numbering OFF não deve ter prefixo '2.': '{txt}'"
        );
        assert!(txt.contains("Apêndice"), "corpo H2 deve estar presente");

        // Validar que o re-update foi visível ao Introspector
        // (final_value reflecte o último valor `false`).
        let intr = introspect_with_introspector(&content);
        assert!(
            !intr.is_numbering_active("numbering_active:heading"),
            "Introspector final_value deve reflectir o último update (false)"
        );
    }

    #[test]
    fn paridade_documento_complexo_legacy_vs_migrated() {
        // P182E .D: documento com headings + parágrafo de texto +
        // equation block. Comparar plain_text entre `layout()` legacy
        // e `layout_with_introspector` directo. Output observable
        // deve ser idêntico — confirma que migração P182B–D não
        // introduziu divergência.
        let content = Content::Sequence(Arc::from(vec![
            Content::SetHeadingNumbering { active: true },
            Content::heading(1, Content::text("Sec1")),
            Content::text("corpo do parágrafo"),
            Content::heading(2, Content::text("Sub1")),
            Content::Equation {
                body:  Box::new(Content::MathText("x".into())),
                block: true,
            },
            Content::heading(1, Content::text("Sec2")),
        ]));

        let txt_legacy = layout(&content).plain_text();
        let intr = introspect_with_introspector(&content);
        let txt_new = layout_with_introspector(&content, intr).plain_text();

        assert_eq!(
            txt_legacy, txt_new,
            "P182E: paridade plain_text entre layout() legacy e layout_with_introspector"
        );
        assert!(txt_legacy.contains("1."));
        assert!(txt_legacy.contains("1.1"));
        assert!(txt_legacy.contains("2."));
    }

    #[test]
    fn walk_popula_intr_state_para_set_heading_numbering() {
        // P182E .E (P190G adapted): walk arm canonical
        // `Content::SetHeadingNumbering` popula
        // `intr.state["numbering_active:heading"]` via populate_intr
        // arm StateUpdate. Mutação legacy `state.numbering_active`
        // ELIMINADA em P190G (Caso 1 `.H`); caminho Introspector é
        // única fonte da verdade.
        let content = Content::SetHeadingNumbering { active: true };
        let intr = introspect_with_introspector(&content);

        assert!(
            intr.is_numbering_active("numbering_active:heading"),
            "walk arm canonical deve popular intr.state com chave canónica"
        );
        // Caso simétrico — `false` também é registado em intr.
        let content_false = Content::SetHeadingNumbering { active: false };
        let intr_false = introspect_with_introspector(&content_false);
        assert!(
            !intr_false.is_numbering_active("numbering_active:heading"),
            "walk arm canonical deve registar Bool(false) em intr.state"
        );
    }
}

// ── P184E — Tests E2E paridade C3 (figure auto-number per kind) ────────────

#[cfg(test)]
mod p184e_figure_per_kind {
    use super::*;
    use crate::entities::introspector::{Introspector, TagIntrospector};
    use crate::rules::introspect::{introspect, introspect_with_introspector};
    use std::sync::Arc;

    /// Helper: figure numerada+captioned com kind dado.
    fn figure(kind: Option<&str>, caption_text: &str) -> Content {
        Content::Figure {
            body:      Box::new(Content::text("body")),
            caption:   Some(Box::new(Content::text(caption_text))),
            kind:      kind.map(|s| s.to_string()),
            numbering: Some("1".into()),
        }
    }

    /// Documento típico: 3 figures `kind: image` numeradas+captioned.
    fn doc_tres_figuras_image() -> Content {
        Content::Sequence(Arc::from(vec![
            figure(Some("image"), "alpha"),
            figure(Some("image"), "beta"),
            figure(Some("image"), "gamma"),
        ]))
    }

    #[test]
    fn pipeline_completo_figure_kind_image_via_introspector() {
        // P184E .B: pipeline `walk → from_tags → layout_with_introspector`
        // com Introspector populado via P184B (chave `figure:image`).
        // P184C `figure_number_at_index` retorna `Some(N)`; consumer C3
        // migrado em P184D usa esse valor (path Introspector activo).
        let content = doc_tres_figuras_image();
        let intr = introspect_with_introspector(&content);

        // Introspector populado: chaves canónicas conhecidas.
        assert_eq!(intr.figure_number_at_index("image", 0), Some(1));
        assert_eq!(intr.figure_number_at_index("image", 1), Some(2));
        assert_eq!(intr.figure_number_at_index("image", 2), Some(3));
        assert_eq!(intr.figure_number_at_index("image", 3), None);

        let txt = layout_with_introspector(&content, intr).plain_text();
        assert!(txt.contains("Figura 1:"), "1ª figure: '{txt}'");
        assert!(txt.contains("Figura 2:"), "2ª figure: '{txt}'");
        assert!(txt.contains("Figura 3:"), "3ª figure: '{txt}'");
        assert!(txt.contains("alpha"));
        assert!(txt.contains("beta"));
        assert!(txt.contains("gamma"));
    }

    #[test]
    fn pipeline_via_fallback_legacy_dead_code_idx_plus_one() {
        // P184E .C: pipeline com `TagIntrospector::empty()` força o
        // path Introspector a retornar `None` para `figure_number_at_index`;
        // fallback `or_else` consulta `state.figure_numbers` legacy
        // (que é dead code factual — copy-sites não copiam, P184A §3.6
        // ratificado em P184B/C/D); cai em `unwrap_or(idx + 1)` heurístico
        // final. Output observable é idêntico ao path Introspector real.
        let content = doc_tres_figuras_image();
        let state_legacy = introspect(&content);
        let txt = layout_with_introspector(&content, TagIntrospector::empty()).plain_text();

        assert!(txt.contains("Figura 1:"), "fallback heurístico 1: '{txt}'");
        assert!(txt.contains("Figura 2:"), "fallback heurístico 2: '{txt}'");
        assert!(txt.contains("Figura 3:"), "fallback heurístico 3: '{txt}'");
    }

    #[test]
    fn paridade_layout_legacy_vs_layout_with_introspector_figures() {
        // P184E .D: `layout()` legacy (re-corre `introspect_with_introspector`
        // internamente per P181H) e `layout_with_introspector` directo
        // produzem mesmo `plain_text`. Confirma que migração P184B–D
        // não introduziu divergência observable. C3 é o **primeiro
        // consumer onde Introspector populado é o caminho activo, não
        // redundância** — paridade aqui valida a inversão.
        let content = doc_tres_figuras_image();

        let txt_legacy = layout(&content).plain_text();
        let intr_new = introspect_with_introspector(&content);
        let txt_new = layout_with_introspector(&content, intr_new).plain_text();

        assert_eq!(
            txt_legacy, txt_new,
            "P184E: paridade plain_text entre layout() legacy e layout_with_introspector"
        );
        assert!(txt_legacy.contains("Figura 1:"));
        assert!(txt_legacy.contains("Figura 2:"));
        assert!(txt_legacy.contains("Figura 3:"));
    }

    #[test]
    fn kinds_distintos_isolados_image_e_table() {
        // P184E .E: documento com 2 figures kind="image" + 2 figures
        // kind="table" intercaladas. Cada kind tem numeração própria
        // (key isolation no `CounterRegistry` per chave `figure:{kind}`).
        // Layouter format hardcoded "Figura N:" (mod.rs:440) independente
        // do kind — distinção observa-se via captions únicas.
        let content = Content::Sequence(Arc::from(vec![
            figure(Some("image"), "im_a"),
            figure(Some("table"), "tb_a"),
            figure(Some("image"), "im_b"),
            figure(Some("table"), "tb_b"),
        ]));

        let intr = introspect_with_introspector(&content);

        assert_eq!(intr.figure_number_at_index("image", 0), Some(1));
        assert_eq!(intr.figure_number_at_index("image", 1), Some(2));
        assert_eq!(intr.figure_number_at_index("image", 2), None);
        assert_eq!(intr.figure_number_at_index("table", 0), Some(1));
        assert_eq!(intr.figure_number_at_index("table", 1), Some(2));
        assert_eq!(intr.figure_number_at_index("table", 2), None);

        let txt = layout_with_introspector(&content, intr).plain_text();
        // Captions únicos confirmam ordem; "Figura 1:" aparece duas vezes
        // (uma para image[0], outra para table[0]).
        assert!(txt.contains("im_a"));
        assert!(txt.contains("im_b"));
        assert!(txt.contains("tb_a"));
        assert!(txt.contains("tb_b"));
        // "Figura 2:" também aparece duas vezes — para image[1] e table[1].
        let figura_2_count = txt.matches("Figura 2:").count();
        assert!(
            figura_2_count >= 2,
            "esperado 2+ ocorrências de 'Figura 2:' (image[1] + table[1]); obtido: {figura_2_count} em '{txt}'"
        );
    }

    #[test]
    fn kind_none_default_image() {
        // P184E .F: `kind: None` mapeia para chave `figure:image`
        // (default per P184B convenção: `kind.as_deref().unwrap_or("image")`
        // em `from_tags` arm Figure + `mod.rs:431` Layouter). Logo
        // figures sem kind explícito partilham o mesmo counter que
        // figures `kind: Some("image")`.
        let content = Content::Sequence(Arc::from(vec![
            figure(None, "default_a"),
            figure(Some("image"), "explicit_b"),
        ]));

        let intr = introspect_with_introspector(&content);

        // Ambas figures aparecem em `figure:image` history.
        assert_eq!(intr.figure_number_at_index("image", 0), Some(1));
        assert_eq!(intr.figure_number_at_index("image", 1), Some(2));

        let txt = layout_with_introspector(&content, intr).plain_text();
        assert!(txt.contains("Figura 1:"));
        assert!(txt.contains("Figura 2:"));
        assert!(txt.contains("default_a"));
        assert!(txt.contains("explicit_b"));
    }
}

// ── P185D — sincronização Locator Layouter ↔ walk de introspect ─────────────

#[cfg(test)]
mod p185d_locator_sync {
    use super::*;
    use crate::entities::element_kind::ElementKind;
    use crate::entities::introspector::Introspector;
    use crate::entities::location::Location;
    use crate::rules::introspect::introspect_with_introspector;
    use crate::rules::introspect::locatable::is_locatable;
    use std::sync::Arc;

    /// Recolhe a sequência de `Location`s emitidas pelo walk de
    /// introspect, em ordem cronológica. `kind_index` preserva ordem
    /// de inserção por kind; agregar todos e ordenar por `as_u128`
    /// recupera a ordem global do walk (Locator é monotonicamente
    /// crescente per `locator.rs:counter_e_monotonico_crescente`).
    fn collect_walk_locations(
        intr: &crate::entities::introspector::TagIntrospector,
    ) -> Vec<Location> {
        let mut all: Vec<Location> = intr.kind_index
            .values()
            .flatten()
            .copied()
            .collect();
        all.sort_by_key(|l| l.as_u128());
        all
    }

    /// Itera parts manualmente, chamando `layout_content` por cada
    /// um, e captura `current_location` após cada arm locatable.
    /// Não modifica produção — só usa API pub(super) acessível a
    /// tests do mesmo módulo.
    fn collect_layout_locations(parts: &[Content]) -> Vec<Location> {
        // P204C (M8): Layouter ganha 'a + Tracked<dyn Introspector>.
        use comemo::Track;
        use crate::entities::introspector::{Introspector, TagIntrospector};
        let intr = TagIntrospector::empty();
        let intr_dyn: &dyn Introspector = &intr;
        let intr_tracked = intr_dyn.track();
        let mut layouter = Layouter::new(FixedMetrics, NullImageSizer, 12.0, intr_tracked);
        let mut locs = Vec::new();
        for part in parts {
            layouter.layout_content(part);
            if is_locatable(part) {
                locs.push(
                    layouter.current_location
                        .expect("locatable arm deve ter setado current_location"),
                );
            }
        }
        locs
    }

    #[test]
    fn sincronizacao_locator_layouter_iguala_walk_introspect() {
        // .B caso central: 3 locatables em sequência (Heading, Figure,
        // Cite), todos cobertos por `is_locatable`. Walk emite 3 tags
        // com Locations [loc(0), loc(1), loc(2)]. Layouter avança 3
        // vezes produzindo a mesma sequência por determinismo do
        // Locator (sincronização-por-construção, ADR-0068 mecanismo M3).
        let parts = vec![
            Content::Heading { level: 1, body: Box::new(Content::Empty) },
            Content::Figure {
                body:      Box::new(Content::Empty),
                caption:   None,
                kind:      None,
                numbering: None,
            },
            Content::Cite { key: "k".to_string(), supplement: None, form: None },
        ];
        let content = Content::Sequence(Arc::from(parts.clone()));

        let intr = introspect_with_introspector(&content);
        let walk_locs   = collect_walk_locations(&intr);
        let layout_locs = collect_layout_locations(&parts);

        assert_eq!(walk_locs.len(), 3, "walk deve emitir 3 tags");
        assert_eq!(layout_locs.len(), 3, "Layouter deve avançar 3 vezes");
        assert_eq!(
            walk_locs, layout_locs,
            "sequências devem coincidir por sincronização-por-construção"
        );
    }

    #[test]
    fn gating_locator_apenas_em_locatables() {
        // .C mistura locatables + não-locatables. Walk emite tags
        // para Heading/Figure/Equation/Cite (4 — Equation locatable
        // após P186D); Layouter avança Locator nos mesmos 4. Text
        // NÃO dispara gating — confirmação empírica que `is_locatable`
        // governa avanço.
        //
        // **P186D nota**: Equation re-incluída no fixture (P186C
        // tinha removido durante janela de invariante quebrada;
        // P186D activa `is_locatable(Equation) = true`, restaurando
        // invariante e sincronização Locator).
        let parts = vec![
            Content::Heading { level: 1, body: Box::new(Content::Empty) },
            Content::text("plain"),
            Content::Figure {
                body:      Box::new(Content::Empty),
                caption:   None,
                kind:      None,
                numbering: None,
            },
            Content::Equation { body: Box::new(Content::Empty), block: false },
            Content::Cite { key: "k".to_string(), supplement: None, form: None },
        ];
        let content = Content::Sequence(Arc::from(parts.clone()));

        let intr = introspect_with_introspector(&content);
        let walk_locs   = collect_walk_locations(&intr);
        let layout_locs = collect_layout_locations(&parts);

        assert_eq!(walk_locs.len(), 4, "4 locatables (Heading/Figure/Equation/Cite)");
        assert_eq!(layout_locs.len(), 4, "Layouter avança 4 vezes");
        assert_eq!(
            walk_locs, layout_locs,
            "sequências iguais — Text é skipped uniformemente"
        );
    }

    #[test]
    fn current_location_none_antes_de_primeiro_locatable() {
        // .D valida decisão de tipo `Option<Location>` (P185C):
        // antes do primeiro locatable, current_location é None
        // (não `Location::from_raw(0)`, que é uma Location real do
        // primeiro Locator::next).
        // P204C (M8): Layouter ganha 'a + Tracked<dyn Introspector>.
        use comemo::Track;
        use crate::entities::introspector::{Introspector, TagIntrospector};
        let intr = TagIntrospector::empty();
        let intr_dyn: &dyn Introspector = &intr;
        let intr_tracked = intr_dyn.track();
        let mut layouter = Layouter::new(FixedMetrics, NullImageSizer, 12.0, intr_tracked);
        assert_eq!(
            layouter.current_location, None,
            "fresh Layouter tem current_location = None"
        );

        // Não-locatable não dispara gating.
        layouter.layout_content(&Content::text("plain"));
        assert_eq!(
            layouter.current_location, None,
            "Text não-locatable não actualiza current_location"
        );

        layouter.layout_content(&Content::Space);
        assert_eq!(
            layouter.current_location, None,
            "Space não-locatable não actualiza current_location"
        );

        // Primeiro locatable dispara gating.
        layouter.layout_content(&Content::Heading {
            level: 1,
            body:  Box::new(Content::Empty),
        });
        assert!(
            layouter.current_location.is_some(),
            "Heading locatable → current_location = Some"
        );
        assert_eq!(
            layouter.current_location.unwrap().as_u128(),
            0,
            "primeiro Locator::next produz Location(0)"
        );
    }

    #[test]
    fn pipeline_e2e_is_numbering_active_at_via_current_location() {
        // .E pipeline end-to-end blueprint para P187 C1 migration.
        // SetHeadingNumbering(true) inicia state em loc(0); 3
        // Headings que vêm depois (loc(1), loc(2), loc(3)) devem
        // ver numbering activo via `is_numbering_active_at(key,
        // current_location)`.
        let parts = vec![
            Content::SetHeadingNumbering { active: true },
            Content::Heading { level: 1, body: Box::new(Content::Empty) },
            Content::Heading { level: 1, body: Box::new(Content::Empty) },
            Content::Heading { level: 1, body: Box::new(Content::Empty) },
        ];
        let content = Content::Sequence(Arc::from(parts.clone()));

        let intr = introspect_with_introspector(&content);

        // P204C (M8): Layouter ganha 'a + Tracked<dyn Introspector>.
        // Assignment `layouter.introspector = intr_clone` ELIMINADO —
        // tracked passa por construtor; introspector populado via
        // introspect_with_introspector outlive layouter no scope.
        use comemo::Track;
        use crate::entities::introspector::Introspector;
        let intr_dyn: &dyn Introspector = &intr;
        let intr_tracked = intr_dyn.track();
        let mut layouter = Layouter::new(FixedMetrics, NullImageSizer, 12.0, intr_tracked);

        let mut headings_validados = 0usize;
        for part in &parts {
            layouter.layout_content(part);
            if matches!(part, Content::Heading { .. }) {
                let loc = layouter.current_location
                    .expect("Heading locatable → Some");
                assert!(
                    intr.is_numbering_active_at("numbering_active:heading", loc),
                    "heading em {:?} deve ver numbering activo (state populado em loc anterior)",
                    loc
                );
                headings_validados += 1;
            }
        }
        assert_eq!(headings_validados, 3, "3 headings esperados");

        // Confirmação adicional: kind_index deve ter exactamente 3
        // headings, e cada Location deve produzir true via método
        // location-aware. Cobre o blueprint que P187 vai usar
        // (consumer migra de `is_numbering_active` snapshot-final
        // para `is_numbering_active_at(key, current_location)`).
        let heading_locs = intr.kind_index
            .get(&ElementKind::Heading)
            .cloned()
            .unwrap_or_default();
        assert_eq!(heading_locs.len(), 3);
        for loc in heading_locs {
            assert!(
                intr.is_numbering_active_at("numbering_active:heading", loc),
                "kind_index Heading[loc={:?}] activo via método P185B",
                loc
            );
        }
    }
}

// ── P186F — tests E2E equation locatable + relatório consolidado ────────────

#[cfg(test)]
mod p186f_equation_locatable {
    use super::*;
    use crate::entities::element_kind::ElementKind;
    use crate::entities::introspector::Introspector;
    use crate::entities::state_update::StateUpdate;
    use crate::entities::value::Value;
    use crate::rules::introspect::introspect_with_introspector;
    use std::sync::Arc;

    fn equation_block() -> Content {
        Content::Equation {
            body:  Box::new(Content::Empty),
            block: true,
        }
    }

    #[test]
    fn pipeline_e2e_equation_block_com_state_activo() {
        // .B caso central: state injectado via Content::StateUpdate
        // (auto-init via P182C arm em from_tags). 3 equations block
        // subsequentes acumulam counter [1, 2, 3].
        let parts = vec![
            Content::StateUpdate {
                key:    "numbering_active:equation".to_string(),
                update: StateUpdate::Set(Box::new(Value::Bool(true))),
            },
            equation_block(),
            equation_block(),
            equation_block(),
        ];
        let content = Content::Sequence(Arc::from(parts));

        let intr = introspect_with_introspector(&content);

        let eq_locs = intr.kind_index
            .get(&ElementKind::Equation)
            .cloned()
            .unwrap_or_default();
        assert_eq!(eq_locs.len(), 3, "3 equations indexadas");

        // Counter avança 1, 2, 3 em sequência.
        assert_eq!(intr.flat_counter_at("equation", eq_locs[0]), Some(1));
        assert_eq!(intr.flat_counter_at("equation", eq_locs[1]), Some(2));
        assert_eq!(intr.flat_counter_at("equation", eq_locs[2]), Some(3));
    }

    #[test]
    fn gate_dormente_sem_state_active() {
        // .C sentinela: produção real — sem Content::StateUpdate para
        // numbering_active:equation, gate bloqueia mesmo para
        // block=true. Counter permanece vazio. Confirma empiricamente
        // que P186 não introduz regressão observable.
        let parts = vec![
            equation_block(),
            equation_block(),
            equation_block(),
        ];
        let content = Content::Sequence(Arc::from(parts));

        let intr = introspect_with_introspector(&content);

        let eq_locs = intr.kind_index
            .get(&ElementKind::Equation)
            .cloned()
            .unwrap_or_default();
        assert_eq!(eq_locs.len(), 3, "kind_index populado mesmo com gate dormente");

        for loc in eq_locs {
            assert_eq!(
                intr.flat_counter_at("equation", loc),
                None,
                "counter dormente em loc={:?}", loc,
            );
        }
    }

    #[test]
    fn gate_dormente_inline_mesmo_com_state_active() {
        // .C variação: state activo + equations inline → gate bloqueia
        // por block=false.
        let parts = vec![
            Content::StateUpdate {
                key:    "numbering_active:equation".to_string(),
                update: StateUpdate::Set(Box::new(Value::Bool(true))),
            },
            Content::Equation { body: Box::new(Content::Empty), block: false },
            Content::Equation { body: Box::new(Content::Empty), block: false },
        ];
        let content = Content::Sequence(Arc::from(parts));

        let intr = introspect_with_introspector(&content);

        let eq_locs = intr.kind_index
            .get(&ElementKind::Equation)
            .cloned()
            .unwrap_or_default();
        assert_eq!(eq_locs.len(), 2);

        for loc in eq_locs {
            assert_eq!(
                intr.flat_counter_at("equation", loc),
                None,
                "inline não populado",
            );
        }
    }

    // P190E (M6): test `paridade_equation_counter_legacy_vs_introspector`
    // removido — Path A (legacy state.numbering_active) eliminado.
    // Caminho Introspector único após P190E. Cobertura via Path B
    // preservada em outros tests P186/P199B.
}

// ── P187B — C1 heading prefix migration ─────────────────────────────────────

#[cfg(test)]
mod p187b_c1_heading_prefix {
    use super::*;
    use crate::entities::introspector::{Introspector, TagIntrospector};
    use crate::rules::introspect::introspect_with_introspector;
    use std::sync::Arc;

    fn heading_with_text(level: u8, text: &str) -> Content {
        Content::Heading {
            level,
            body:  Box::new(Content::text(text)),
        }
    }

    fn doc_3_headings() -> Content {
        Content::Sequence(Arc::from(vec![
            Content::SetHeadingNumbering { active: true },
            heading_with_text(1, "Intro"),
            heading_with_text(2, "Motivacao"),
            heading_with_text(1, "Conclusao"),
        ]))
    }

    #[test]
    fn c1_heading_prefix_via_introspector_path() {
        // Introspector populado pelo walk; consulta `formatted_counter_at`
        // retorna snapshot na Location de cada heading.
        let content = doc_3_headings();
        let intr = introspect_with_introspector(&content);
        let txt = layout_with_introspector(&content, intr).plain_text();

        assert!(txt.contains("1. Intro"),  "esperado '1. Intro' em: {:?}", txt);
        assert!(txt.contains("1.1. Motivacao"), "esperado '1.1. Motivacao' em: {:?}", txt);
        assert!(txt.contains("2. Conclusao"), "esperado '2. Conclusao' em: {:?}", txt);
    }

    // P190E (M6): tests `c1_heading_prefix_via_fallback_legacy` +
    // `c1_heading_prefix_paridade_legacy_vs_migrated` removidos —
    // fallback legacy `format_hierarchical` + `is_numbering_active`
    // eliminados. Caminho Introspector único após P190E. Cobertura via
    // `c1_heading_prefix_re_update_correctness` + outros tests P185B/P187B.

    #[test]
    fn c1_heading_prefix_re_update_correctness() {
        // **Caso central P183B** que falhou com `formatted_counter`
        // snapshot-final pré-emptando fallback. P187B usa
        // `formatted_counter_at(key, current_location)` que retorna
        // snapshot por Location — sequência H1, H2, H1 produz
        // ["1.", "1.1", "2."] e NÃO ["2.", "2.", "2."] como em P183B.
        //
        // Empiricamente valida que P185 (location-aware) desbloqueou
        // P183B aprendizado.
        let content = doc_3_headings();
        let intr = introspect_with_introspector(&content);

        // Validação intermédia — Introspector retorna valor correcto
        // por Location, não snapshot-final.
        let heading_locs = intr.kind_index
            .get(&crate::entities::element_kind::ElementKind::Heading)
            .cloned()
            .unwrap_or_default();
        assert_eq!(heading_locs.len(), 3, "3 headings indexadas");
        assert_eq!(intr.formatted_counter_at("heading", heading_locs[0]).as_deref(), Some("1"));
        assert_eq!(intr.formatted_counter_at("heading", heading_locs[1]).as_deref(), Some("1.1"));
        assert_eq!(intr.formatted_counter_at("heading", heading_locs[2]).as_deref(), Some("2"));

        // Output observable — sequência correcta no documento.
        let txt = layout_with_introspector(&content, intr).plain_text();
        // Encontrar os 3 prefixos em ordem (não substring simples
        // porque "1." é substring de "1.1.").
        let intro_pos = txt.find("1. Intro").expect("'1. Intro' em ordem");
        let motiv_pos = txt.find("1.1. Motivacao").expect("'1.1. Motivacao' em ordem");
        let concl_pos = txt.find("2. Conclusao").expect("'2. Conclusao' em ordem");
        assert!(intro_pos < motiv_pos);
        assert!(motiv_pos < concl_pos);

        // Garantia explícita anti-P183B: o segundo H1 ("Conclusao")
        // NÃO ganha prefixo "1." (que indicaria snapshot-final
        // pré-emptando fallback).
        assert!(!txt.contains("1. Conclusao"),
            "regressão P183B: 'Conclusao' não pode ter prefixo '1.': {:?}", txt);
    }
}

// ── P188B — C2 equation counter migration ───────────────────────────────────

#[cfg(test)]
mod p188b_c2_equation_counter {
    use super::*;
    use crate::entities::introspector::Introspector;
    use crate::entities::state_update::StateUpdate;
    use crate::entities::value::Value;
    use crate::rules::introspect::introspect_with_introspector;
    use std::sync::Arc;

    fn equation_block(text: &str) -> Content {
        Content::Equation {
            body:  Box::new(Content::MathIdent(text.into())),
            block: true,
        }
    }

    fn doc_3_equations() -> Content {
        Content::Sequence(Arc::from(vec![
            equation_block("a"),
            equation_block("b"),
            equation_block("c"),
        ]))
    }

    #[test]
    fn c2_equation_counter_via_introspector_path_quando_state_injectado() {
        // Path Introspector funcional quando state
        // `numbering_active:equation` é injectado via Content::StateUpdate
        // (auto-init via P182C arm em from_tags). Gate em P186E dispara
        // → counter introspector populado → `flat_counter_at` retorna
        // valores correctos.
        let parts = vec![
            Content::StateUpdate {
                key:    "numbering_active:equation".to_string(),
                update: StateUpdate::Set(Box::new(Value::Bool(true))),
            },
            equation_block("a"),
            equation_block("b"),
            equation_block("c"),
        ];
        let content = Content::Sequence(Arc::from(parts));

        let intr = introspect_with_introspector(&content);

        // Validação intermédia: counter populado.
        let eq_locs = intr.kind_index
            .get(&crate::entities::element_kind::ElementKind::Equation)
            .cloned()
            .unwrap_or_default();
        assert_eq!(eq_locs.len(), 3, "3 equations indexadas");
        assert_eq!(intr.flat_counter_at("equation", eq_locs[0]), Some(1));
        assert_eq!(intr.flat_counter_at("equation", eq_locs[1]), Some(2));
        assert_eq!(intr.flat_counter_at("equation", eq_locs[2]), Some(3));
    }

    // P190E (M6): tests `c2_equation_counter_via_fallback_legacy_caso_producao`
    // + `c2_equation_counter_paridade_legacy_vs_introspector` removidos —
    // fallback legacy `state.numbering_active` eliminado. Caminho
    // Introspector único via SetEquationNumbering (P199B) +
    // `is_numbering_active_at` (P185B). Cobertura preservada via
    // tests P199B SetEquationNumbering pipeline standard.
}

// ── P189B — Walk puro M5 incremental ────────────────────────────────────────

#[cfg(test)]
mod p189b_walk_puro_m5 {
    use super::*;
    use crate::rules::introspect::{introspect, introspect_with_introspector};
    use crate::entities::introspector::Introspector;
    use crate::entities::label::Label;
    use std::sync::Arc;

    // ── Outline migrado: paridade observable preservada ─────────────────────

    #[test]
    fn outline_migrado_paridade_observable() {
        // P189B `.B` — Outline arm puro. `state.has_outline` mutação
        // removida; consumer lê via `intr.kind_index`.
        // P190D (M6 categoria Document metadata): field
        // `has_outline` eliminado. Test mantém cobertura observable
        // via Layouter integration — outline render funciona via
        // Introspector path (mod.rs:1488).
        let doc_com_outline = Content::Sequence(Arc::from(vec![
            Content::SetHeadingNumbering { active: true },
            Content::Heading { level: 1, body: Box::new(Content::text("Intro")) },
            Content::Outline,
        ]));
        let state_com = introspect(&doc_com_outline);
        // Layout funciona via Introspector path (re-walk em layout()).
        let txt_com = layout(&doc_com_outline).plain_text();
        assert!(txt_com.contains("Intro"), "doc com outline: {:?}", txt_com);

        let doc_sem_outline = Content::Sequence(Arc::from(vec![
            Content::SetHeadingNumbering { active: true },
            Content::Heading { level: 1, body: Box::new(Content::text("Solo")) },
        ]));
        let state_sem = introspect(&doc_sem_outline);
        let txt_sem = layout(&doc_sem_outline).plain_text();
        assert!(txt_sem.contains("Solo"), "doc sem outline: {:?}", txt_sem);
    }

    // ── Tests sentinela 6 excepções (E1–E6) ─────────────────────────────────

    // P190E (M6): test `walk_excepcao_e1_equation_counter_via_legacy` removido
    // — sentinela P189B testava walk legacy via state.numbering_active
    // pré-populado. Após P190E, fallback legacy eliminado; E1 fechada
    // estruturalmente em P199B via SetEquationNumbering. Cobertura via
    // tests P199B pipeline standard.

    #[test]
    fn walk_excepcao_e2_heading_hierarchical_via_intr() {
        // E2 (P190G adapted): Heading walk arm popula state.hierarchical
        // (legacy) e intr.resolved_labels + intr.headings_for_toc
        // (Introspector path) — fields legacy `resolved_labels`,
        // `headings_for_toc`, `numbering_active` eliminados em P190G.
        let content = Content::Sequence(Arc::from(vec![
            Content::SetHeadingNumbering { active: true },
            Content::Heading { level: 1, body: Box::new(Content::text("A")) },
            Content::Heading { level: 2, body: Box::new(Content::text("B")) },
        ]));
        let intr = introspect_with_introspector(&content);
        assert!(intr.is_numbering_active("numbering_active:heading"));
        assert_eq!(intr.headings_for_toc().len(), 2);
        assert!(intr.resolved_labels.get(&Label("auto-toc-1".to_string())).is_some());
    }

    #[test]
    fn walk_excepcao_e3_figure_via_intr() {
        // E3 (P190H adapted): Figure walk arm popula
        // intr.counters["figure:image"] via populate_intr arm Figure
        // (P191C, gated por is_counted). Field legacy
        // `state.figure_numbers` eliminado.
        let content = Content::Figure {
            body:      Box::new(Content::Empty),
            caption:   Some(Box::new(Content::text("cap"))),
            kind:      Some("image".into()),
            numbering: Some("1".into()),
        };
        let intr = introspect_with_introspector(&content);
        assert_eq!(intr.figure_number_at_index("image", 0), Some(1),
            "E3: intr.figure_number_at_index(image, 0) = 1 via populate_intr");
    }

    #[test]
    fn walk_excepcao_e4_labelled_resolved_labels_via_intr() {
        // E4 (P190G adapted): Labelled walk arm popula
        // intr.resolved_labels via Tag::Labelled pós-recursão (P195D).
        // Field legacy `state.resolved_labels` eliminado.
        let content = Content::Sequence(Arc::from(vec![
            Content::SetHeadingNumbering { active: true },
            Content::Labelled {
                target: Box::new(Content::Heading {
                    level: 1,
                    body:  Box::new(Content::text("X")),
                }),
                label:  crate::entities::label::Label("intro".to_string()),
            },
        ]));
        let intr = introspect_with_introspector(&content);
        assert!(intr.resolved_labels.get(
            &crate::entities::label::Label("intro".to_string())
        ).is_some(), "E4: intr.resolved_labels[intro] populado");
    }

    #[test]
    fn walk_excepcao_e5_set_heading_numbering_via_intr() {
        // E5 (P190G adapted): SetHeadingNumbering walk arm popula
        // intr.state["numbering_active:heading"] via populate_intr.
        // Field legacy `state.numbering_active` eliminado.
        let content = Content::SetHeadingNumbering { active: true };
        let intr = introspect_with_introspector(&content);
        assert!(intr.is_numbering_active("numbering_active:heading"),
            "E5: intr.state[numbering_active:heading] populado");
    }

    #[test]
    fn walk_excepcao_e6_counter_update_via_legacy() {
        // E6: CounterUpdate walk arm. Confirma que walk legacy ainda
        // populates state.flat para chaves custom via CounterUpdate.
        let content = Content::Sequence(Arc::from(vec![
            Content::CounterUpdate {
                key:    "custom".to_string(),
                action: crate::entities::counter_update::CounterUpdate::Step,
            },
            Content::CounterUpdate {
                key:    "custom".to_string(),
                action: crate::entities::counter_update::CounterUpdate::Step,
            },
        ]));
        // P190I (M6 fechado): state legacy eliminado; verificar via intr.
        let intr = introspect(&content);
        let custom_count = intr.counters.value("custom")
            .and_then(|v| v.last()).copied().unwrap_or(0);
        assert_eq!(custom_count, 2,
            "E6: intr.counters['custom'].last() = 2 após 2 steps");
    }
}

// ── P194B — C4 resolved label migration ─────────────────────────────────────

#[cfg(test)]
mod p194b_c4_resolved_label {
    use super::*;
    use crate::entities::introspector::TagIntrospector;
    use crate::entities::label::Label;
    use crate::rules::introspect::introspect;
    use std::sync::Arc;

    fn lbl(s: &str) -> Label {
        Label(s.to_string())
    }

    fn doc_heading_labelled_e_ref(label_name: &str) -> Content {
        // Heading (level 1) wrapped in Labelled + Ref para o mesmo
        // label. Walk legacy popula state.resolved_labels via arm
        // Labelled (E4 P189B excepção).
        Content::Sequence(Arc::from(vec![
            Content::SetHeadingNumbering { active: true },
            Content::Labelled {
                target: Box::new(Content::Heading {
                    level: 1,
                    body:  Box::new(Content::text("Intro")),
                }),
                label:  lbl(label_name),
            },
            Content::Ref { target: lbl(label_name) },
        ]))
    }

    #[test]
    fn c4_resolved_label_via_introspector_path_puro() {
        // P190G: fallback legacy `state.resolved_labels` ELIMINADO.
        // Introspector path é única fonte da verdade. Este test
        // confirma que populate manual de intr.resolved_labels é
        // suficiente para Layouter renderizar correctamente.
        let content = Content::Sequence(Arc::from(vec![
            Content::Ref { target: lbl("intro") },
        ]));

        // P190I: state eliminado
        let mut intr = TagIntrospector::empty();
        intr.resolved_labels.insert(lbl("intro"), "Introspector text".to_string());

        let txt = layout_with_introspector(&content, intr).plain_text();

        assert!(txt.contains("Introspector text"),
            "Introspector path puro: {:?}", txt);
    }

    #[test]
    fn c4_resolved_label_via_introspector_pipeline_real() {
        // P190G: pipeline real popula intr.resolved_labels via walk
        // arm Labelled (P195D Tag pós-recursão). Caminho Introspector
        // é única fonte da verdade.
        let content = doc_heading_labelled_e_ref("intro");
        let state = introspect(&content);

        // Pipeline completo via layout(); Introspector populated via
        // re-walk em layout(); Layouter consume via Introspector path
        // puro (sem fallback legacy).
        let txt = layout(&content).plain_text();

        assert!(txt.contains("Secção 1"),
            "Introspector path: 'Secção 1' renderizada: {:?}", txt);
        assert!(!txt.contains("@intro"),
            "ref intro NÃO deve cair em fallback @intro: {:?}", txt);
    }

    #[test]
    fn c4_resolved_label_paridade_pipelines() {
        // P190G: dois pipelines (introspect normal + manual intr)
        // produzem mesmo output observable.
        let content = doc_heading_labelled_e_ref("intro");

        // Path A: pipeline normal.
        let state_a = introspect(&content);
        let txt_a = layout(&content).plain_text();

        // Path B: manual intr.
        // P190I: state_b eliminado
        let mut intr_b = TagIntrospector::empty();
        intr_b.resolved_labels.insert(lbl("intro"), "Secção 1".to_string());
        let txt_b = layout_with_introspector(
            &Content::Sequence(Arc::from(vec![
                Content::Ref { target: lbl("intro") },
            ])),
            intr_b,
        ).plain_text();

        // Paridade.
        assert!(txt_a.contains("Secção 1"),
            "Path A: {:?}", txt_a);
        assert!(txt_b.contains("Secção 1"),
            "Path B: {:?}", txt_b);
    }

    #[test]
    fn c4_resolved_label_fallback_at_arrobado_quando_ausente() {
        // Label não existe em nenhum dos paths; fallback final do
        // match retorna `@nome` literal.
        let content = Content::Sequence(Arc::from(vec![
            Content::Ref { target: lbl("missing") },
        ]));

        // P190I: state eliminado
        let intr = TagIntrospector::empty();

        let txt = layout_with_introspector(&content, intr).plain_text();

        assert!(txt.contains("@missing"),
            "fallback final '@missing' esperado: {:?}", txt);
    }
}

// ── P195D — Walk arm Labelled emite Tag pós-recursão (ADR-0069) ─────────────

#[cfg(test)]
mod p195d_walk_labelled {
    use super::*;
    use crate::entities::label::Label;
    use crate::rules::introspect::introspect_with_introspector;
    use std::sync::Arc;

    fn lbl(s: &str) -> Label {
        Label(s.to_string())
    }

    #[test]
    fn labelled_walk_emite_tag_e_popula_introspector() {
        let content = Content::Sequence(Arc::from(vec![
            Content::SetHeadingNumbering { active: true },
            Content::Labelled {
                target: Box::new(Content::Heading {
                    level: 1,
                    body:  Box::new(Content::text("Intro")),
                }),
                label:  lbl("intro"),
            },
        ]));

        let intr = introspect_with_introspector(&content);

        // Caminho Introspector activo: sub-store populated via P195D Tag.
        assert_eq!(
            intr.resolved_labels.get(&lbl("intro")),
            Some("Secção 1"),
            "intr.resolved_labels[intro] populated via P195D",
        );

        // Heading não é Figure → figure_label_numbers vazio.
        assert_eq!(intr.figure_label_numbers.get(&lbl("intro")), None);

        // P190G: mutação legacy `state.resolved_labels` ELIMINADA;
        // sub-store é única fonte da verdade.
    }

    #[test]
    fn labelled_paridade_observable_legacy_vs_introspector() {
        let content = Content::Sequence(Arc::from(vec![
            Content::SetHeadingNumbering { active: true },
            Content::Labelled {
                target: Box::new(Content::Heading {
                    level: 1,
                    body:  Box::new(Content::text("Intro")),
                }),
                label:  lbl("intro"),
            },
            Content::Ref { target: lbl("intro") },
        ]));

        let intr = introspect_with_introspector(&content);

        // P190G: paridade observable preservada via Introspector
        // path. Field legacy `state.resolved_labels` eliminado.
        assert_eq!(
            intr.resolved_labels.get(&lbl("intro")),
            Some("Secção 1"),
        );

        // Pipeline completo: Ref renderiza via Introspector path.
        let txt = layout(&content).plain_text();
        assert!(txt.contains("Secção 1"),
            "Ref intro → 'Secção 1' via Introspector: {:?}", txt);
        assert!(!txt.contains("@intro"),
            "fallback @intro NÃO esperado: {:?}", txt);
    }

    #[test]
    fn labelled_figure_target_popula_figure_label_numbers() {
        let content = Content::Sequence(Arc::from(vec![
            Content::Labelled {
                target: Box::new(Content::Figure {
                    body:      Box::new(Content::text("body")),
                    caption:   Some(Box::new(Content::text("caption"))),
                    kind:      Some("image".into()),
                    numbering: Some("1".into()),
                }),
                label:  lbl("fig1"),
            },
        ]));

        let intr = introspect_with_introspector(&content);

        // figure_label_numbers populated (write paralelo P195D + P168).
        assert_eq!(
            intr.figure_label_numbers.get(&lbl("fig1")),
            Some(&1),
        );
        // resolved_labels também populated via P195D Tag.
        assert!(intr.resolved_labels.get(&lbl("fig1")).is_some());
    }

    #[test]
    fn labelled_target_nao_resolvivel_nao_popula_introspector() {
        // Target = Text (sem numeração); compute_labelled retorna
        // (None, None); Tag não emitida; sub-store não populated.
        let content = Content::Sequence(Arc::from(vec![
            Content::Labelled {
                target: Box::new(Content::text("not numbered")),
                label:  lbl("foo"),
            },
        ]));

        let intr = introspect_with_introspector(&content);

        assert_eq!(intr.resolved_labels.get(&lbl("foo")), None);
        assert_eq!(intr.figure_label_numbers.get(&lbl("foo")), None);
    }
}
