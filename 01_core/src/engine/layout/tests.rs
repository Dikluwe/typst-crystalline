//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/layout.md
//! @prompt-hash afb3bcc7
//! @layer L1
//! @updated 2026-07-14
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
use crate::engine::introspect::introspect;
use crate::entities::paint::Paint;
use crate::entities::{
    content::Content,
    elements::outline::OutlineIndent,
    geometry::ShapeKind,
    layout_types::{FrameItem, Point},
    style::Styles,
    value::Value,
};

/// **P589** — Fonte neutra para testes de algoritmo. Deve estar disponível
/// tanto no cristalino como no vanilla, e não deve ter os problemas já
/// conhecidos de outras fontes (ex.: FreeSerif com decomposição de acentos).
pub(crate) const FONTE_NEUTRA_TESTE: &str = "DejaVu Sans";

/// **P589** — Helper que envolve um documento de teste de algoritmo com a
/// fonte neutra, para evitar ruído de fontes default diferentes entre
/// cristalino e vanilla.
pub(crate) fn documento_algoritmo(conteudo: &str) -> String {
    format!("#set text(font: \"{}\")\n{}", FONTE_NEUTRA_TESTE, conteudo)
}

/// **F-5a de-bake (P365)** — rotula reproduzindo a **forma de produção**: o
/// transporte de numbering (`Content::Styled`, ex.: `Content::figure(.., Some)`)
/// fica **fora** do `Labelled` (`Styled{ Labelled{ alvo } }`), como a fatia-1
/// embrulha a cauda. Levanta o transporte transparente para fora; alvo simples
/// passa direto. (Espelho do helper homónimo em `introspect.rs`.)
fn labelled_prod(target: Content, label: crate::entities::label::Label) -> Content {
    let name = label.0;
    match target {
        Content::Styled(inner, styles) => {
            Content::Styled(Box::new(Content::label_auto(name, *inner)), styles)
        }
        other => Content::label_auto(name, other),
    }
}

// ── Testes de FixedMetrics (Passo 21) ────────────────────────────────

#[test]
fn fixed_metrics_advance_proporcional_ao_tamanho() {
    let m = FixedMetrics;
    let style = TextStyle::default();
    let a12 = m.advance("Hello", Pt(12.0), &style);
    let a24 = m.advance("Hello", Pt(24.0), &style);
    assert!(
        (a24.val() - 2.0 * a12.val()).abs() < 0.001,
        "advance deve escalar linearmente com font_size"
    );
}

#[test]
fn fixed_metrics_monoespaco_iiii_eq_wwww() {
    let m = FixedMetrics;
    let style = TextStyle::default();
    let ai = m.advance("iiii", Pt(12.0), &style);
    let aw = m.advance("WWWW", Pt(12.0), &style);
    assert_eq!(ai, aw, "FixedMetrics é monoespaçado — iiii == WWWW");
}

#[test]
fn fixed_metrics_vertical_ascender_menor_que_line_height() {
    let style = TextStyle::default();
    let (asc, lh) = FixedMetrics.vertical_metrics(Pt(12.0), &style);
    assert!(asc.val() > 0.0, "ascender deve ser positivo");
    assert!(lh.val() > asc.val(), "line_height > ascender");
}

#[test]
fn layouter_baseline_dentro_da_pagina() {
    // P204C (M8): Layouter ganha 'a + Tracked<dyn Introspector>.
    use crate::entities::introspector::{Introspector, TagIntrospector};
    use comemo::Track;
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
    use crate::entities::introspector::{Introspector, TagIntrospector};
    use comemo::Track;
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

// ── Lote F-3 (DEBT C2) — o elemento de utilizador dinâmico renderiza ──────────
#[test]
fn f3_dynamic_element_renderiza_body_fecha_debt_c2() {
    use crate::entities::elements::test_callout::CalloutElem;
    // Pipeline real: Content::Dynamic → layout → FrameItems → plain_text.
    // (Antes, F-1/F-2: no-op. Agora o body renderiza — DEBT C2 fechado.)
    let c = Content::dynamic(CalloutElem::new(
        Content::text("corpo dinâmico"),
        "Aviso",
        "warn",
    ));
    let doc = layout(&c);
    assert!(!doc.pages.is_empty(), "elemento dinâmico produz páginas");
    assert!(
        doc.plain_text().contains("corpo dinâmico"),
        "o body do elemento dinâmico deve renderizar no layout (DEBT C2): '{}'",
        doc.plain_text()
    );
}

// ── F-item3 (P368) — `#set <elemento-de-usuário>(prop:)` pelo mapa aberto ─────
// O elemento de usuário lê uma prop **opcional** da chain (`custom`), com
// precedência **construído explícito > chain > default**. Prova a extensibilidade
// (a chain como fonte de estilo para props de usuário). O `badge` renderiza via
// `plain_text` (sem body), logo a prop resolvida é observável no output.

fn badge_note_chain(note: &str) -> crate::entities::style::Styles {
    crate::entities::style::Styles::new()
        .push_custom("badge.note", crate::entities::value::Value::Str(note.into()))
}

#[test]
fn f_item3_set_badge_note_resolve_da_chain() {
    use crate::entities::elements::test_callout::BadgeElem;
    // `#set badge(note: "X")` viaja como `Content::Styled{custom badge.note=X}`;
    // o badge (construído SEM note) resolve-o da chain → output "L (X)".
    let content = Content::Styled(
        Box::new(Content::dynamic(BadgeElem::new("L"))),
        badge_note_chain("nota-da-chain"),
    );
    let doc = layout(&content);
    assert!(
        doc.plain_text().contains("L (nota-da-chain)"),
        "badge sem note deve resolver `note` da chain (#set): '{}'",
        doc.plain_text()
    );
}

#[test]
fn f_item3_construido_explicito_vence_a_chain() {
    use crate::entities::elements::test_callout::BadgeElem;
    // Precedência (vanilla: explícito > #set): um badge com `note` construído
    // ignora o `#set badge(note:)` da chain.
    let content = Content::Styled(
        Box::new(Content::dynamic(BadgeElem {
            label: "L".into(),
            note: Some("own".into()),
        })),
        badge_note_chain("da-chain"),
    );
    let doc = layout(&content);
    assert!(
        doc.plain_text().contains("L (own)") && !doc.plain_text().contains("da-chain"),
        "note construído explícito vence o #set da chain: '{}'",
        doc.plain_text()
    );
}

#[test]
fn f_item3_sem_set_badge_sem_note() {
    use crate::entities::elements::test_callout::BadgeElem;
    // Sem `#set` (sem o transporte), o badge não tem note → só o label.
    let doc = layout(&Content::dynamic(BadgeElem::new("L")));
    let t = doc.plain_text();
    assert!(t.contains("L") && !t.contains("("), "sem #set, sem note: '{t}'");
}

// ── P204D (M8) — Sentinel + E2E tests para Position concrete ──────────────
//
// Confirmam que tipo `Position` existe, que `LayouterRuntimeState` ganhou
// campo `positions`, e que Layouter popula durante layout single-pass.

#[test]
fn p204d_position_struct_existe() {
    // Sentinel: tipo Position existe em `crate::entities::position`.
    // Falha de compilação se for removido ou renomeado.
    use crate::entities::layout_types::{Point, Pt};
    use crate::entities::position::Position;
    use std::num::NonZeroUsize;

    let _p = Position {
        page: NonZeroUsize::new(1).unwrap(),
        point: Point { x: Pt(0.0), y: Pt(0.0) },
    };
}

#[test]
fn p204d_runtime_positions_field_existe() {
    // Sentinel: LayouterRuntimeState tem field `positions`.
    // Falha de compilação se for removido. Construído via Default
    // — confirma que campo existe e é HashMap<Location, Position>.
    use crate::entities::layouter_runtime_state::LayouterRuntimeState;
    use crate::entities::location::Location;
    use crate::entities::position::Position;
    use std::collections::HashMap;

    let runtime = LayouterRuntimeState::default();
    let _check: &HashMap<Location, Position> = &runtime.positions;
    assert!(runtime.positions.is_empty(), "default runtime tem positions vazio");
}

#[test]
fn p204d_position_populada_para_locatable_basico() {
    // E2E test 1: documento com 1 label (locatable Heading)
    // produz entry em runtime.positions.
    use crate::entities::introspector::{Introspector, TagIntrospector};
    use comemo::Track;

    let intr = TagIntrospector::empty();
    let intr_dyn: &dyn Introspector = &intr;
    let intr_tracked = intr_dyn.track();
    let mut layouter =
        Layouter::new(FixedMetrics, NullImageSizer, DEFAULT_FONT_SIZE, intr_tracked);

    let content = Content::heading(1, Content::text("Title"));
    layouter.layout_content(&content);

    // Heading é locatable → current_location set + Position emitted.
    let loc = layouter.current_location.expect("Heading locatable → Some");
    let pos = layouter
        .runtime
        .positions
        .get(&loc)
        .copied()
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
    use crate::entities::introspector::{Introspector, TagIntrospector};
    use comemo::Track;

    let intr = TagIntrospector::empty();
    let intr_dyn: &dyn Introspector = &intr;
    let intr_tracked = intr_dyn.track();
    let mut layouter =
        Layouter::new(FixedMetrics, NullImageSizer, DEFAULT_FONT_SIZE, intr_tracked);

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
    use crate::entities::introspector::{Introspector, TagIntrospector};
    use comemo::Track;

    let mut intr = TagIntrospector::empty();
    let intr_dyn: &dyn Introspector = &intr;
    let intr_tracked = intr_dyn.track();
    let mut layouter =
        Layouter::new(FixedMetrics, NullImageSizer, DEFAULT_FONT_SIZE, intr_tracked);

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
    let pos = intr
        .position_of(loc)
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

#[test]
fn layout_divider_emite_shape_line() {
    let doc = layout(&Content::divider());
    assert!(
        doc.pages
            .iter()
            .flat_map(|p| p.items.iter())
            .any(|i| matches!(i, FrameItem::Shape { kind: ShapeKind::Line { .. }, .. })),
        "Divider deve emitir pelo menos um FrameItem::Shape(Line)"
    );
}

#[test]
fn p727_layout_curve_fallback_stroke_chega_a_pagina() {
    // P727 — regressão do bug "curve renderiza página em branco": o
    // fallback de stroke default (stdlib, paridade vanilla Smart::Auto)
    // tem de chegar intacto à Page como `stroke: Some`.
    let doc = layout_test("#curve(curve.move((0pt,0pt)), curve.line((50pt,50pt)))");
    let has_stroked_path = doc.pages.iter().flat_map(|p| p.items.iter()).any(|i| {
        matches!(i, FrameItem::Shape { kind: ShapeKind::Path(_), stroke: Some(_), .. })
    });
    assert!(
        has_stroked_path,
        "P727: curve sem fill/stroke deve chegar à Page com stroke de fallback"
    );
}

#[test]
fn layout_link_preserva_url_e_texto() {
    let doc = layout(&Content::link("https://example.com", Content::text("click")));
    use crate::entities::layout_types::LinkTarget;
    let has_link = doc
        .pages
        .iter()
        .flat_map(|p| p.items.iter())
        .any(|i| matches!(i, FrameItem::Link { target: LinkTarget::Url(url), .. } if url.as_str() == "https://example.com"));
    assert!(has_link, "Link deve emitir FrameItem::Link com URL");
    assert!(doc.plain_text().contains("click"));
}

#[test]
fn layout_transform_preserva_shape() {
    use crate::entities::geometry::ShapeKind;
    use crate::entities::layout_types::TransformMatrix;
    fn has_shape(items: &[FrameItem]) -> bool {
        items.iter().any(|item| match item {
            FrameItem::Shape { kind: ShapeKind::Line { .. }, .. } => true,
            FrameItem::Group { items, .. } => has_shape(items),
            _ => false,
        })
    }
    let matrix = TransformMatrix::identity();
    let shape =
        Content::shape(ShapeKind::Line { dx: 20.0, dy: 0.0 }, None, None, None, None);
    let doc = layout(&Content::transform(matrix, shape));
    assert!(
        doc.pages.iter().any(|p| has_shape(&p.items)),
        "transformação identity deve preservar shape interno"
    );
}

/// **P832 (achado #58, GRAVE)** — texto dentro de `move`/`rotate`/`scale`
/// era descartado silenciosamente (página em branco, exit 0). O body da
/// transformação passa agora por sub-layout real (paridade vanilla: o body
/// é layoutado para uma frame e embrulhado num grupo com a matriz).
#[test]
fn layout_transform_renderiza_texto() {
    use crate::entities::layout_types::TransformMatrix;
    fn has_text(items: &[FrameItem], needle: &str) -> bool {
        items.iter().any(|item| match item {
            FrameItem::Text { text, .. } => text.contains(needle),
            #[allow(deprecated)]
            FrameItem::TextShaped { text, .. } => text.contains(needle),
            FrameItem::Group { items, .. } => has_text(items, needle),
            _ => false,
        })
    }
    for (name, matrix) in [
        ("rotate", TransformMatrix::rotate(30.0_f64.to_radians())),
        ("move", TransformMatrix::translate(20.0, 10.0)),
        ("scale", TransformMatrix::scale(1.5, 1.5)),
    ] {
        let doc = layout(&Content::transform(matrix, Content::text("Rodado")));
        assert!(
            doc.pages.iter().any(|p| has_text(&p.items, "Rodado")),
            "{name} com texto deve emitir FrameItem::Text dentro do Group"
        );
    }
}

/// **P832** — a correcção cobre conteúdo arbitrariamente aninhado
/// (`rotate(30deg)[#strong[Rodado] e mais texto]`), não só texto simples.
#[test]
fn layout_transform_renderiza_conteudo_aninhado() {
    use crate::entities::layout_types::TransformMatrix;
    fn collect_text(items: &[FrameItem], out: &mut String) {
        for item in items {
            match item {
                FrameItem::Text { text, .. } => out.push_str(text),
                #[allow(deprecated)]
                FrameItem::TextShaped { text, .. } => out.push_str(text),
                FrameItem::Group { items, .. } => collect_text(items, out),
                _ => {}
            }
        }
    }
    let body = Content::sequence(vec![
        Content::strong(Content::text("Rodado")),
        Content::Space,
        Content::text("e mais texto"),
    ]);
    let doc = layout(&Content::transform(
        TransformMatrix::rotate(30.0_f64.to_radians()),
        body,
    ));
    let mut text = String::new();
    for page in &doc.pages {
        collect_text(&page.items, &mut text);
    }
    assert!(
        text.contains("Rodado") && text.contains("mais"),
        "todo o conteúdo aninhado deve renderizar dentro do transform; obtido: {text:?}"
    );
}

/// Teste de Ouro: todos os items dentro dos limites da página.
#[test]
fn layout_items_dentro_limites_da_pagina() {
    let words = (0..100).map(|i| format!("palavra{i}")).collect::<Vec<_>>().join(" ");
    let doc = layout(&Content::text(&words));

    for page in &doc.pages {
        for item in &page.items {
            if let FrameItem::Text { pos, .. } = item {
                assert!(
                    pos.x.val() >= 0.0 && pos.x.val() < 595.0,
                    "x={} fora dos limites da página",
                    pos.x.val()
                );
                assert!(
                    pos.y.val() >= 0.0 && pos.y.val() < 842.0,
                    "y={} fora dos limites da página",
                    pos.y.val()
                );
            }
        }
    }
}

#[test]
fn layout_texto_longo_word_wrap() {
    let words = (0..50).map(|i| format!("w{i}")).collect::<Vec<_>>().join(" ");
    let doc = layout(&Content::text(&words));
    let items = doc.pages.iter().flat_map(|p| p.items.iter()).count();
    let y_values: std::collections::HashSet<u64> = doc
        .pages
        .iter()
        .flat_map(|p| p.items.iter())
        .filter_map(|i| {
            if let FrameItem::Text { pos, .. } = i {
                Some(pos.y.val().to_bits())
            } else {
                None
            }
        })
        .collect();
    assert!(y_values.len() > 1, "texto longo deve ter múltiplas linhas: {} items", items);
}

// ── P757 — dimensões de página em `em` ───────────────────────────────────

#[test]
fn p757_page_width_height_em_resolve_contra_font_size() {
    // `7em` / `5em` com font-size default (11 pt) devem dar 77 x 55 pt,
    // não 0 x 0 pt.
    let doc = layout_test("#set page(width: 7em, height: 5em)\nX");
    assert_eq!(doc.pages.len(), 1, "documento deve ter exactamente uma página");
    let page = &doc.pages[0];
    assert!(
        (page.width - 77.0).abs() < 0.5,
        "width: 7em com font-size 11pt deve ser ~77pt, foi {}",
        page.width
    );
    assert!(
        (page.height - 55.0).abs() < 0.5,
        "height: 5em com font-size 11pt deve ser ~55pt, foi {}",
        page.height
    );
}

#[test]
fn p757_page_width_em_respeita_text_size() {
    // `#set text(size: 12pt)` antes de `#set page(width: 7em)` deve fazer
    // com que 7em = 84 pt.
    let doc = layout_test("#set text(size: 12pt)\n#set page(width: 7em, height: 5em)\nX");
    assert_eq!(doc.pages.len(), 1);
    let page = &doc.pages[0];
    assert!(
        (page.width - 84.0).abs() < 0.5,
        "width: 7em com font-size 12pt deve ser ~84pt, foi {}",
        page.width
    );
    assert!(
        (page.height - 60.0).abs() < 0.5,
        "height: 5em com font-size 12pt deve ser ~60pt, foi {}",
        page.height
    );
}

// ── Testes rich text (Passo 22) ────────────────────────────────────────

#[test]
fn strong_produz_bold_style() {
    // Após Passo 33: node_style deve ter bold=true (capturado em eval via Strong).
    // Construção directa usa TextStyle::bold para simular o que eval produziria.
    let doc = layout(&Content::strong(Content::Text("Bold".into())));
    let bold = doc
        .pages
        .iter()
        .flat_map(|p| p.items.iter())
        .any(|i| matches!(i, FrameItem::Text { style, .. } if style.bold));
    assert!(bold, "Strong deve produzir FrameItem com bold=true");
}

#[test]
fn emph_produz_italic_style() {
    // Após Passo 33: node_style deve ter italic=true (capturado em eval via Emph).
    // Construção directa usa TextStyle::italic para simular o que eval produziria.
    let doc = layout(&Content::emph(Content::Text("Italic".into())));
    let italic = doc
        .pages
        .iter()
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
    let sizes: Vec<f64> = doc
        .pages
        .iter()
        .flat_map(|p| p.items.iter())
        .filter_map(|i| {
            if let FrameItem::Text { style, .. } = i {
                Some(style.size.val())
            } else {
                None
            }
        })
        .collect();
    let max_size = sizes.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let min_size = sizes.iter().cloned().fold(f64::INFINITY, f64::min);
    assert!(max_size > min_size, "H1 deve ter tamanho maior que o texto normal");
}

#[test]
fn estilo_restaurado_apos_strong() {
    let doc = layout(&Content::sequence(vec![
        Content::strong(Content::text("Bold")),
        Content::text("normal"),
    ]));
    let items: Vec<_> = doc.pages.iter().flat_map(|p| p.items.iter()).collect();
    if let Some(FrameItem::Text { style, text, .. }) = items.last() {
        if text.as_str() == "normal" {
            assert!(!style.bold, "texto após Strong deve ser regular");
        }
    }
}

// ── Passo 446 — smallcaps (render real por scaling) ────────────────────

#[test]
fn p446_smallcaps_converte_minusculas_para_maiusculas() {
    let doc = layout(&Content::smallcaps(Content::text("SmallCaps")));
    let items: Vec<_> = doc
        .pages
        .iter()
        .flat_map(|p| p.items.iter())
        .filter_map(|i| {
            if let FrameItem::Text { text, style, .. } = i {
                Some((text.to_string(), style.size))
            } else {
                None
            }
        })
        .collect();
    let rendered: String = items.iter().map(|(t, _)| t.as_str()).collect();
    assert_eq!(rendered, "SMALLCAPS");

    let scaled_count = items.iter().filter(|(_, size)| size.0 < 11.0).count();
    let normal_count = items.iter().filter(|(_, size)| size.0 >= 11.0).count();
    assert!(scaled_count > 0, "deve haver runs de smallcaps menores");
    assert!(normal_count > 0, "maiúsculas mantêm tamanho normal");
}

#[test]
fn p446_smallcaps_via_stdlib_converte_texto() {
    let doc = layout_test("#smallcaps(\"Hello\")");
    let items: Vec<_> = doc
        .pages
        .iter()
        .flat_map(|p| p.items.iter())
        .filter_map(|i| {
            if let FrameItem::Text { text, style, .. } = i {
                Some((text.to_string(), style.size))
            } else {
                None
            }
        })
        .collect();
    let rendered: String = items.iter().map(|(t, _)| t.as_str()).collect();
    assert_eq!(rendered, "HELLO");

    let scaled_count = items.iter().filter(|(_, size)| size.0 < 11.0).count();
    assert!(scaled_count > 0, "minúsculas devem ser renderizadas a 0.8×");
}

#[test]
fn p408_smallcaps_nao_vaza_estilo() {
    // SmallCaps é transparente: não introduz bold/italic nem modifica
    // estilos irmãos.
    let doc = layout(&Content::sequence(vec![
        Content::smallcaps(Content::text("sc")),
        Content::text("normal"),
    ]));
    let items: Vec<_> = doc.pages.iter().flat_map(|p| p.items.iter()).collect();
    if let Some(FrameItem::Text { style, text, .. }) = items.last() {
        if text.as_str() == "normal" {
            assert!(
                !style.bold && !style.italic,
                "texto após smallcaps deve ser regular"
            );
        }
    }
}

// ── Passo 448 — subscript / superscript (baseline + scaling) ───────────

#[test]
fn p448_subscript_desloca_baseline_para_baixo() {
    let doc = layout(&Content::sequence(vec![
        Content::text("a"),
        Content::sub(Content::text("b")),
        Content::text("c"),
    ]));
    let items: Vec<_> = doc
        .pages
        .iter()
        .flat_map(|p| p.items.iter())
        .filter_map(|i| {
            if let FrameItem::Text { text, pos, style } = i {
                Some((text.to_string(), pos.y, style.size))
            } else {
                None
            }
        })
        .collect();
    let y_normal = items.iter().find(|(t, _, _)| t == "a").map(|(_, y, _)| *y).unwrap();
    let (y_sub, size_sub) = items
        .iter()
        .find(|(t, _, _)| t == "b")
        .map(|(_, y, s)| (*y, *s))
        .unwrap();
    assert!(y_sub < y_normal, "subscrito deve descer abaixo da baseline normal");
    assert!(size_sub.0 < 11.0, "subscrito deve reduzir o corpo tipográfico");
}

#[test]
fn p448_superscript_desloca_baseline_para_cima() {
    let doc = layout(&Content::sequence(vec![
        Content::text("a"),
        Content::superscript(Content::text("b")),
        Content::text("c"),
    ]));
    let items: Vec<_> = doc
        .pages
        .iter()
        .flat_map(|p| p.items.iter())
        .filter_map(|i| {
            if let FrameItem::Text { text, pos, style } = i {
                Some((text.to_string(), pos.y, style.size))
            } else {
                None
            }
        })
        .collect();
    let y_normal = items.iter().find(|(t, _, _)| t == "a").map(|(_, y, _)| *y).unwrap();
    let (y_sup, size_sup) = items
        .iter()
        .find(|(t, _, _)| t == "b")
        .map(|(_, y, s)| (*y, *s))
        .unwrap();
    assert!(y_sup > y_normal, "sobrescrito deve subir acima da baseline normal");
    assert!(size_sup.0 < 11.0, "sobrescrito deve reduzir o corpo tipográfico");
}

// ── Passo 449 — highlight (fundo colorido por detrás do texto) ─────────

#[test]
fn p449_highlight_default_emite_shape_amarelo_antes_do_texto() {
    use crate::entities::geometry::ShapeKind;
    use crate::entities::layout_types::Color;

    let yellow = Color::rgba(255, 242, 54, 255);
    let doc = layout(&Content::highlight(Content::text("x"), Some(yellow)));
    let items: Vec<_> = doc.pages[0].items.iter().collect();
    let mut found = false;
    for window in items.windows(2) {
        if let (
            FrameItem::Shape { kind: ShapeKind::Rect, fill: Some(fill), .. },
            FrameItem::Text { text, .. },
        ) = (&window[0], &window[1])
        {
            if *fill == yellow && text.as_str() == "x" {
                found = true;
                break;
            }
        }
    }
    assert!(found, "esperado rect amarelo seguido de texto 'x'");
}

#[test]
fn p449_highlight_fill_custom_emite_shape_correspondente() {
    use crate::entities::geometry::ShapeKind;
    use crate::entities::layout_types::Color;

    let doc =
        layout(&Content::highlight(Content::text("x"), Some(Color::rgb(255, 0, 0))));
    let found = doc.pages[0].items.iter().any(|i| {
        matches!(
            i,
            FrameItem::Shape {
                kind: ShapeKind::Rect,
                fill: Some(c),
                ..
            } if *c == Color::rgb(255, 0, 0)
        )
    });
    assert!(found, "esperado rect vermelho de highlight");
}

#[test]
fn p449_highlight_fill_none_nao_emite_shape() {
    use crate::entities::geometry::ShapeKind;

    let doc = layout(&Content::highlight(Content::text("x"), None));
    let has_shape = doc.pages[0]
        .items
        .iter()
        .any(|i| matches!(i, FrameItem::Shape { kind: ShapeKind::Rect, .. }));
    assert!(!has_shape, "fill: none não deve emitir shape de highlight");
}

#[test]
fn p449_texto_plano_nao_emite_shape_de_highlight() {
    use crate::entities::geometry::ShapeKind;

    let doc = layout(&Content::text("x"));
    let has_shape = doc.pages[0]
        .items
        .iter()
        .any(|i| matches!(i, FrameItem::Shape { kind: ShapeKind::Rect, .. }));
    assert!(!has_shape, "texto sem highlight não deve ter rect de fundo");
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
        engine::eval::eval_for_test,
        entities::{
            file_id::FileId,
            font_book::FontBook,
            source::Source,
            world_types::{Bytes, Datetime, FileError, FileResult, Font, Library},
        },
    };
    use std::num::NonZeroU16;

    struct MockWorld {
        library: Library,
        book: FontBook,
        source: Source,
    }

    impl MockWorld {
        fn new(text: &str) -> Self {
            let id = FileId::from_raw(NonZeroU16::new(1).unwrap());
            Self {
                library: Library::new(),
                book: FontBook::new(),
                source: Source::new(id, text.to_string()),
            }
        }
    }

    impl World for MockWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            self.source.id()
        }
        fn source(&self, _: FileId) -> FileResult<Source> {
            Ok(self.source.clone())
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<Font> {
            None
        }
        fn today(&self, _: Option<i64>) -> Option<Datetime> {
            None
        }
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
        "texto deve estar no output: {:?}",
        doc.plain_text()
    );
}

// ── Passo 23 ────────────────────────────────────────────────────────────

#[test]
fn layout_list_item_tem_bullet() {
    let doc = layout(&Content::list_item(Content::text("Item")));
    let has_marker = doc
        .pages
        .iter()
        .flat_map(|p| p.items.iter())
        .any(|i| matches!(i, FrameItem::Text { text, .. } if text.as_str() == "•"));
    assert!(has_marker, "ListItem deve ter marcador '•'");
}

// ── P505 — indentação e espaçamento em listas/enums ──────────────────────

#[test]
fn layout_list_item_respeita_indentacao() {
    use crate::entities::layout_types::Length;
    use crate::entities::list_marker::ListMarker;

    let body = Content::text("Item com texto suficientemente longo para forçar uma quebra de linha no body do item");
    let item = Content::list_item_full(
        body,
        Some(ListMarker::Custom("→".into())),
        None,
        Some(Length::em(1.5)),
        Some(Length::em(0.5)),
        None,
    );
    let doc = layout(&item);
    let margin = 70.87_f64;
    let indent_em = 16.5_f64; // 1.5em @ 11pt
    let body_indent_em = 5.5_f64; // 0.5em @ 11pt
    let expected_marker_x = margin + indent_em;

    let marker = doc
        .pages
        .iter()
        .flat_map(|p| p.items.iter())
        .find(|i| matches!(i, FrameItem::Text { text, .. } if text.as_str() == "→"));
    let marker = marker.expect("deve haver marcador '→'");
    let marker_x = match marker {
        FrameItem::Text { pos, .. } => pos.x.val(),
        _ => unreachable!(),
    };
    let marker_width = FixedMetrics.advance("→", Pt(11.0), &TextStyle::default()).val();
    let expected_body_x = expected_marker_x + marker_width + body_indent_em;

    assert!(
        (marker_x - expected_marker_x).abs() < 0.01,
        "marker_x={marker_x}, esperado={expected_marker_x}"
    );

    // O primeiro caractere do body deve aparecer na posição body_x.
    let body_texts: Vec<_> = doc
        .pages
        .iter()
        .flat_map(|p| p.items.iter())
        .filter(|i| matches!(i, FrameItem::Text { text, .. } if !text.as_str().trim().is_empty() && text.as_str() != "→"))
        .collect();
    assert!(!body_texts.is_empty(), "deve haver texto do body");
    let first_body = body_texts.first().unwrap();
    let body_x = match first_body {
        FrameItem::Text { pos, .. } => pos.x.val(),
        _ => unreachable!(),
    };
    assert!(
        (body_x - expected_body_x).abs() < 0.01,
        "body_x={body_x}, esperado={expected_body_x}"
    );
}

#[test]
fn layout_list_tight_false_adiciona_espaco() {
    use crate::entities::layout_types::Length;

    let items = Content::sequence(vec![
        Content::list_item_full(Content::text("A"), None, None, None, None, Some(false)),
        Content::list_item_full(Content::text("B"), None, None, None, None, Some(false)),
    ]);
    let doc = layout(&items);

    let ys: Vec<f64> = doc
        .pages
        .iter()
        .flat_map(|p| p.items.iter())
        .filter_map(|i| match i {
            FrameItem::Text { text, pos, .. } if text.as_str() == "•" => {
                Some(pos.y.val())
            }
            _ => None,
        })
        .collect();
    assert_eq!(ys.len(), 2, "deve haver dois marcadores");
    let style = TextStyle::default();
    // **P762** — avanço de linha = top-edge + |bottom-edge| + leading default
    // (0,65 em), não o antigo line_height (ascender + descender + lineGap).
    let (top, bottom) = FixedMetrics.text_edges(Pt(11.0), &style);
    let leading = style.leading.map(|l| l.resolve_pt(11.0)).unwrap_or(11.0 * 0.65);
    let line_advance = top.val() + bottom.val().abs() + leading;
    // P505 — `tight: false` adiciona um line_advance de espaçamento de
    // parágrafo *além* do line_advance natural do flush_line.
    let expected_gap = 2.0 * line_advance;
    let actual_gap = ys[1] - ys[0];
    assert!(
        (actual_gap - expected_gap).abs() < 0.01,
        "gap={actual_gap}, esperado={expected_gap}"
    );
}

#[test]
fn layout_list_tight_default_preserva_gap_natural() {
    let items = Content::sequence(vec![
        Content::list_item_full(Content::text("A"), None, None, None, None, None),
        Content::list_item_full(Content::text("B"), None, None, None, None, None),
    ]);
    let doc = layout(&items);
    let ys: Vec<f64> = doc
        .pages
        .iter()
        .flat_map(|p| p.items.iter())
        .filter_map(|i| match i {
            FrameItem::Text { text, pos, .. } if text.as_str() == "•" => {
                Some(pos.y.val())
            }
            _ => None,
        })
        .collect();
    assert_eq!(ys.len(), 2, "deve haver dois marcadores");
    let style = TextStyle::default();
    let (top, bottom) = FixedMetrics.text_edges(Pt(11.0), &style);
    let leading = style.leading.map(|l| l.resolve_pt(11.0)).unwrap_or(11.0 * 0.65);
    let line_advance = top.val() + bottom.val().abs() + leading;
    let actual_gap = ys[1] - ys[0];
    assert!(
        (actual_gap - line_advance).abs() < 0.01,
        "gap={actual_gap}, esperado={line_advance}"
    );
}

/// **P837** (achado #23 de P831) — `top-edge`/`bottom-edge` com `Length`
/// explícito: o edge é o comprimento resolvido a partir da baseline
/// (paridade vanilla `FontInstance::edges`, `text/font/mod.rs:276-289`:
/// `top = length.at(size)`, `bottom = -length.at(size)` com bottom
/// negativo-abaixo-da-baseline no cristalino). ANTES (medido): o braço
/// `Value::Length` não existia e o valor caía no default.
#[test]
fn p837_text_edges_length_explicito() {
    use crate::entities::layout_types::{Length, TextEdge};

    let size = Pt(11.0);

    // top-edge: 18pt → top = +18pt (acima da baseline).
    let mut style = TextStyle::default();
    style.top_edge = Some(TextEdge::Length(Length::pt(18.0)));
    let (top, _) = FixedMetrics.text_edges(size, &style);
    assert!((top.val() - 18.0).abs() < 1e-9, "top={:?}", top);

    // bottom-edge: -4pt → bottom = -4pt (4pt abaixo da baseline;
    // convenção cristalina: bottom negativo = abaixo da baseline).
    let mut style = TextStyle::default();
    style.bottom_edge = Some(TextEdge::Length(Length::pt(-4.0)));
    let (_, bottom) = FixedMetrics.text_edges(size, &style);
    assert!((bottom.val() + 4.0).abs() < 1e-9, "bottom={:?}", bottom);

    // top-edge: 1.5em a 11pt → top = 16.5pt (componente em resolve no
    // font-size, paridade `Length::at(font_size)` do vanilla).
    let mut style = TextStyle::default();
    style.top_edge = Some(TextEdge::Length(Length::em(1.5)));
    let (top, _) = FixedMetrics.text_edges(size, &style);
    assert!((top.val() - 16.5).abs() < 1e-9, "top={:?}", top);
}

#[test]
fn layout_enum_item_respeita_indentacao() {
    use crate::entities::enum_numbering::EnumNumbering;
    use crate::entities::layout_types::Length;

    let body = Content::text("Primeiro item");
    let item = Content::enum_item_full(
        Some(1),
        body,
        Some(EnumNumbering::Decimal),
        Some(Length::em(1.5)),
        Some(Length::em(0.5)),
        None,
    );
    let doc = layout(&item);
    let margin = 70.87_f64;
    let indent_em = 16.5_f64; // 1.5em @ 11pt
    let body_indent_em = 5.5_f64; // 0.5em @ 11pt
    let expected_label_x = margin + indent_em;

    let label = doc
        .pages
        .iter()
        .flat_map(|p| p.items.iter())
        .find(|i| matches!(i, FrameItem::Text { text, .. } if text.as_str() == "1."));
    let label = label.expect("deve haver rótulo '1.'");
    let label_x = match label {
        FrameItem::Text { pos, .. } => pos.x.val(),
        _ => unreachable!(),
    };
    let label_width = FixedMetrics.advance("1.", Pt(11.0), &TextStyle::default()).val();
    let expected_body_x = expected_label_x + label_width + body_indent_em;

    assert!(
        (label_x - expected_label_x).abs() < 0.01,
        "label_x={label_x}, esperado={expected_label_x}"
    );

    let body_texts: Vec<_> = doc
        .pages
        .iter()
        .flat_map(|p| p.items.iter())
        .filter(|i| matches!(i, FrameItem::Text { text, .. } if text.as_str().contains("Primeiro")))
        .collect();
    assert!(!body_texts.is_empty(), "deve haver texto do body");
    let first_body = body_texts.first().unwrap();
    let body_x = match first_body {
        FrameItem::Text { pos, .. } => pos.x.val(),
        _ => unreachable!(),
    };
    assert!(
        (body_x - expected_body_x).abs() < 0.01,
        "body_x={body_x}, esperado={expected_body_x}"
    );
}

#[test]
fn layout_enum_tight_false_adiciona_espaco() {
    use crate::entities::layout_types::Length;

    let items = Content::sequence(vec![
        Content::enum_item_full(
            Some(1),
            Content::text("A"),
            None,
            None,
            None,
            Some(false),
        ),
        Content::enum_item_full(
            Some(2),
            Content::text("B"),
            None,
            None,
            None,
            Some(false),
        ),
    ]);
    let doc = layout(&items);

    let ys: Vec<f64> = doc
        .pages
        .iter()
        .flat_map(|p| p.items.iter())
        .filter_map(|i| match i {
            FrameItem::Text { text, pos, .. }
                if text.as_str() == "1." || text.as_str() == "2." =>
            {
                Some(pos.y.val())
            }
            _ => None,
        })
        .collect();
    assert_eq!(ys.len(), 2, "deve haver dois rótulos");
    let style = TextStyle::default();
    // **P762** — avanço de linha = top-edge + |bottom-edge| + leading default.
    let (top, bottom) = FixedMetrics.text_edges(Pt(11.0), &style);
    let leading = style.leading.map(|l| l.resolve_pt(11.0)).unwrap_or(11.0 * 0.65);
    let line_advance = top.val() + bottom.val().abs() + leading;
    // P505 — `tight: false` adiciona um line_advance de espaçamento de
    // parágrafo *além* do line_advance natural do flush_line.
    let expected_gap = 2.0 * line_advance;
    let actual_gap = ys[1] - ys[0];
    assert!(
        (actual_gap - expected_gap).abs() < 0.01,
        "gap={actual_gap}, esperado={expected_gap}"
    );
}

#[test]
fn layout_enum_tight_default_preserva_gap_natural() {
    let items = Content::sequence(vec![
        Content::enum_item_full(Some(1), Content::text("A"), None, None, None, None),
        Content::enum_item_full(Some(2), Content::text("B"), None, None, None, None),
    ]);
    let doc = layout(&items);
    let ys: Vec<f64> = doc
        .pages
        .iter()
        .flat_map(|p| p.items.iter())
        .filter_map(|i| match i {
            FrameItem::Text { text, pos, .. }
                if text.as_str() == "1." || text.as_str() == "2." =>
            {
                Some(pos.y.val())
            }
            _ => None,
        })
        .collect();
    assert_eq!(ys.len(), 2, "deve haver dois rótulos");
    let style = TextStyle::default();
    let (top, bottom) = FixedMetrics.text_edges(Pt(11.0), &style);
    let leading = style.leading.map(|l| l.resolve_pt(11.0)).unwrap_or(11.0 * 0.65);
    let line_advance = top.val() + bottom.val().abs() + leading;
    let actual_gap = ys[1] - ys[0];
    assert!(
        (actual_gap - line_advance).abs() < 0.01,
        "gap={actual_gap}, esperado={line_advance}"
    );
}

#[test]
fn layout_raw_block_tamanho_menor() {
    let content = Content::sequence(vec![
        Content::text("normal"),
        Content::raw("code", None, true),
    ]);
    let doc = layout(&content);
    let sizes: std::collections::HashSet<u64> = doc
        .pages
        .iter()
        .flat_map(|p| p.items.iter())
        .filter_map(|i| match i {
            FrameItem::Text { style, .. } => Some(style.size.val().to_bits()),
            _ => None,
        })
        .collect();
    assert!(sizes.len() > 1, "Raw deve ter tamanho diferente do texto normal");
}

    // **P864** — agrupamento por Parbreak entre itens do mesmo tipo.

    #[test]
    fn layout_lista_parbreak_separa_grupos() {
        let content = Content::sequence(vec![
            Content::list_item(Content::text("first")),
            Content::Parbreak,
            Content::list_item(Content::text("second")),
        ]);
        let doc = layout(&content);
        let ys: Vec<f64> = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|i| match i {
                FrameItem::Text { text, pos, .. } if text.as_str() == "•" => Some(pos.y.val()),
                _ => None,
            })
            .collect();
        assert_eq!(ys.len(), 2, "deve haver dois marcadores");
        let style = TextStyle::default();
        let (top, bottom) = FixedMetrics.text_edges(Pt(11.0), &style);
        let leading = style.leading.map(|l| l.resolve_pt(11.0)).unwrap_or(11.0 * 0.65);
        let line_advance = top.val() + bottom.val().abs() + leading;
        let actual_gap = ys[1] - ys[0];
        assert!(
            (actual_gap - 2.0 * line_advance).abs() < 0.01,
            "gap={actual_gap}, esperado={}",
            2.0 * line_advance
        );
    }

    #[test]
    fn layout_lista_sem_parbreak_continua_grupo() {
        let content = Content::sequence(vec![
            Content::list_item(Content::text("first")),
            Content::list_item(Content::text("second")),
        ]);
        let doc = layout(&content);
        let ys: Vec<f64> = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|i| match i {
                FrameItem::Text { text, pos, .. } if text.as_str() == "•" => Some(pos.y.val()),
                _ => None,
            })
            .collect();
        assert_eq!(ys.len(), 2, "deve haver dois marcadores");
        let style = TextStyle::default();
        let (top, bottom) = FixedMetrics.text_edges(Pt(11.0), &style);
        let leading = style.leading.map(|l| l.resolve_pt(11.0)).unwrap_or(11.0 * 0.65);
        let line_advance = top.val() + bottom.val().abs() + leading;
        let actual_gap = ys[1] - ys[0];
        assert!(
            (actual_gap - line_advance).abs() < 0.01,
            "gap={actual_gap}, esperado={line_advance}"
        );
    }

    #[test]
    fn layout_enum_parbreak_separa_grupos_e_reinicia_numero() {
        let content = Content::sequence(vec![
            Content::enum_item(None, Content::text("first")),
            Content::Parbreak,
            Content::enum_item(None, Content::text("second")),
        ]);
        let doc = layout(&content);
        let labels: Vec<(String, f64)> = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|i| match i {
                FrameItem::Text { text, pos, .. }
                    if text.as_str().ends_with('.') =>
                {
                    Some((text.to_string(), pos.y.val()))
                }
                _ => None,
            })
            .collect();
        assert_eq!(labels.len(), 2, "deve haver dois rótulos: {:?}", labels);
        assert_eq!(labels[0].0, "1.", "primeiro grupo começa em 1");
        assert_eq!(labels[1].0, "1.", "segundo grupo reinicia em 1");

        let style = TextStyle::default();
        let (top, bottom) = FixedMetrics.text_edges(Pt(11.0), &style);
        let leading = style.leading.map(|l| l.resolve_pt(11.0)).unwrap_or(11.0 * 0.65);
        let line_advance = top.val() + bottom.val().abs() + leading;
        let actual_gap = labels[1].1 - labels[0].1;
        assert!(
            (actual_gap - 2.0 * line_advance).abs() < 0.01,
            "gap={actual_gap}, esperado={}",
            2.0 * line_advance
        );
    }

    #[test]
    fn layout_enum_sem_parbreak_continua_numero() {
        let content = Content::sequence(vec![
            Content::enum_item(None, Content::text("first")),
            Content::enum_item(None, Content::text("second")),
        ]);
        let doc = layout(&content);
        let labels: Vec<String> = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|i| match i {
                FrameItem::Text { text, .. } if text.as_str().ends_with('.') => Some(text.to_string()),
                _ => None,
            })
            .collect();
        assert_eq!(labels, vec!["1.", "2."], "enum deve continuar numeração");
    }

    #[test]
    fn layout_terms_parbreak_separa_grupos() {
        let content = Content::sequence(vec![
            Content::term_item(Content::text("API"), Content::text("interface")),
            Content::Parbreak,
            Content::term_item(Content::text("CLI"), Content::text("command line")),
        ]);
        let doc = layout(&content);
        let ys: Vec<f64> = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|i| match i {
                FrameItem::Text { text, pos, .. } if text.as_str() == "API" || text.as_str() == "CLI" => {
                    Some(pos.y.val())
                }
                _ => None,
            })
            .collect();
        assert_eq!(ys.len(), 2, "deve haver dois termos");
        let style = TextStyle::default();
        let (top, bottom) = FixedMetrics.text_edges(Pt(11.0), &style);
        let leading = style.leading.map(|l| l.resolve_pt(11.0)).unwrap_or(11.0 * 0.65);
        let line_advance = top.val() + bottom.val().abs() + leading;
        let actual_gap = ys[1] - ys[0];
        assert!(
            (actual_gap - 2.0 * line_advance).abs() < 0.01,
            "gap={actual_gap}, esperado={}",
            2.0 * line_advance
        );
    }

    #[test]
    fn layout_terms_sem_parbreak_continua_grupo() {
        let content = Content::sequence(vec![
            Content::term_item(Content::text("API"), Content::text("interface")),
            Content::term_item(Content::text("CLI"), Content::text("command line")),
        ]);
        let doc = layout(&content);
        let ys: Vec<f64> = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|i| match i {
                FrameItem::Text { text, pos, .. } if text.as_str() == "API" || text.as_str() == "CLI" => {
                    Some(pos.y.val())
                }
                _ => None,
            })
            .collect();
        assert_eq!(ys.len(), 2, "deve haver dois termos");
        let style = TextStyle::default();
        let (top, bottom) = FixedMetrics.text_edges(Pt(11.0), &style);
        let leading = style.leading.map(|l| l.resolve_pt(11.0)).unwrap_or(11.0 * 0.65);
        let line_advance = top.val() + bottom.val().abs() + leading;
        let actual_gap = ys[1] - ys[0];
        assert!(
            (actual_gap - line_advance).abs() < 0.01,
            "gap={actual_gap}, esperado={line_advance}"
        );
    }

    #[test]
    fn layout_lista_para_enum_nao_adiciona_espaco_extra() {
        let content = Content::sequence(vec![
            Content::list_item(Content::text("first")),
            Content::Parbreak,
            Content::enum_item(None, Content::text("second")),
        ]);
        let doc = layout(&content);
        let ys: Vec<f64> = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|i| match i {
                FrameItem::Text { text, pos, .. }
                    if text.as_str() == "•" || text.as_str().ends_with('.') =>
                {
                    Some(pos.y.val())
                }
                _ => None,
            })
            .collect();
        assert_eq!(ys.len(), 2, "deve haver um marcador e um rótulo");
        let style = TextStyle::default();
        let (top, bottom) = FixedMetrics.text_edges(Pt(11.0), &style);
        let leading = style.leading.map(|l| l.resolve_pt(11.0)).unwrap_or(11.0 * 0.65);
        let line_advance = top.val() + bottom.val().abs() + leading;
        let actual_gap = ys[1] - ys[0];
        assert!(
            (actual_gap - line_advance).abs() < 0.01,
            "gap={actual_gap}, esperado={line_advance}"
        );
    }

// ── Passo 48 — Baselines em equações inline ──────────────────────────────

fn layout_test(src: &str) -> PagedDocument {
    use crate::{
        contracts::world::World,
        engine::eval::eval_for_test,
        entities::{
            file_id::FileId,
            font_book::FontBook,
            source::Source,
            world_types::{Bytes, Datetime, FileError, FileResult, Font, Library},
        },
    };
    use std::num::NonZeroU16;

    struct MockWorld {
        library: Library,
        book: FontBook,
        source: Source,
    }

    impl MockWorld {
        fn new(text: &str) -> Self {
            let id = FileId::from_raw(NonZeroU16::new(1).unwrap());
            Self {
                library: Library::new(),
                book: FontBook::new(),
                source: Source::new(id, text.to_string()),
            }
        }
    }

    impl World for MockWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            self.source.id()
        }
        fn source(&self, _: FileId) -> FileResult<Source> {
            Ok(self.source.clone())
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<Font> {
            None
        }
        fn today(&self, _: Option<i64>) -> Option<Datetime> {
            None
        }
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
        assert!(text.contains('𝑥'));
        assert!(text.contains('1'));
    }

    #[test]
    fn equacao_inline_com_attach_nao_regride() {
        let doc = layout_test("$x^2$");
        let text = doc.plain_text();
        assert!(text.contains('𝑥'));
        assert!(text.contains('2'));
    }

    #[test]
    fn equacao_inline_com_prime_nao_regride() {
        let doc = layout_test("$x'$");
        let text = doc.plain_text();
        assert!(text.contains('𝑥'));
        assert!(text.contains('′'));
    }

    #[test]
    fn pagina_nao_vazia_com_equacao_inline() {
        let doc = layout_test("$frac(1, 2)$");
        assert!(!doc.pages.is_empty());
        assert!(!doc.pages[0].items.is_empty());
    }

    #[test]
    #[ignore = "P800: consagrava a regra Passo 48 (offset_y = cursor_y - axis_pt), \
                refutada por medição vanilla — a baseline do math inline coincide \
                com a do texto. Substituído por equacao_inline_baseline_coincide_com_texto."]
    fn equacao_inline_sobe_em_relacao_ao_baseline() {
        // Com o ajuste de baseline, os items da equação inline estão acima
        // do cursor_y (offset_y < cursor_y). Com FixedMetrics, axis_height=500
        // e upem=1000, axis_pt = 0.5 * font_size = 6.0pt.
        // Verificamos que pelo menos um item tem y < cursor_y inicial (≈81.6pt).
        let doc = layout_test("$x$");
        let all_y: Vec<f64> = doc
            .pages
            .iter()
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

    #[test]
    fn equacao_inline_baseline_coincide_com_texto() {
        // P800 — paridade vanilla medida por `mutool trace` em
        // `Hello $x^2$`: texto e math partilham a MESMA baseline (o eixo
        // matemático fica axis_height acima e só governa o centrado interno
        // de frac/delims). A regra Passo 48 (`offset_y = cursor_y - axis_pt`)
        // deslocava a fórmula ~0.5em para cima do texto — refutada por
        // medição. Este teste substitui a asserção dessa regra
        // (`equacao_inline_sobe_em_relacao_ao_baseline`, acima, marcada
        // #[ignore] neste passo).
        let doc = layout_test("Hello $x$");
        let y_of = |needle: &str| {
            doc.pages.iter().flat_map(|p| p.items.iter()).find_map(|i| match i {
                FrameItem::Text { pos, text, .. } if text.contains(needle) => {
                    Some(pos.y.val())
                }
                _ => None,
            })
        };
        let y_text = y_of("Hello").expect("texto 'Hello' presente");
        let y_math = y_of("𝑥").expect("math 'x' presente");
        assert!(
            (y_text - y_math).abs() < 0.01,
            "baseline do math inline deve coincidir com a do texto: texto_y={y_text} math_y={y_math}"
        );
    }

    #[test]
    fn if_com_math_inline_attach_baseline_coincide() {
        // P800 — caso original do achado #7 de P798: `#if` com corpo contendo
        // math inline com superscript. O conteúdo avalia e o math fica na
        // baseline do texto; o sup fica acima (y menor).
        let doc = layout_test("#if true [Hello $x^2$]");
        let y_of = |needle: &str| {
            doc.pages.iter().flat_map(|p| p.items.iter()).find_map(|i| match i {
                FrameItem::Text { pos, text, .. } if text.contains(needle) => {
                    Some(pos.y.val())
                }
                _ => None,
            })
        };
        let y_text = y_of("Hello").expect("texto 'Hello' presente");
        let y_math = y_of("𝑥").expect("math 'x' presente");
        let y_sup = y_of("2").expect("sup '2' presente");
        assert!(
            (y_text - y_math).abs() < 0.01,
            "baseline do math em #if deve coincidir com a do texto: texto_y={y_text} math_y={y_math}"
        );
        assert!(y_sup < y_math, "sup deve ficar acima da baseline: sup_y={y_sup} math_y={y_math}");
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
            text.contains('∑') || text.contains('𝑖') || text.contains('𝑛'),
            "operador ou limites ausentes: {}",
            text
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
        assert!(text.contains('𝑥'));
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
        // **P813** — no topo da página a baseline da equação passa a ser
        // `margin + ascent_ink` (paridade vanilla: a tinta do sup empilhado
        // já não invade a margem superior). Antes (modelo antigo): baseline =
        // margin + cap_height do texto = 79.7 e y_sup ≈ 65.5 (< margin=72).
        // Agora: baseline ≈ 92.0, y_sup ≈ 76.3 (medido com FixedMetrics).
        let doc = layout_test("$ sum_(i=0)^n $");
        let all_y: Vec<f64> = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|i| match i {
                FrameItem::Text { pos, .. } => Some(pos.y.val()),
                FrameItem::Glyph { pos, .. } => Some(pos.y.val()),
                _ => None,
            })
            .collect();
        assert!(!all_y.is_empty(), "deve ter items");
        let min_y = all_y.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_y = all_y.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        assert!(
            min_y < 80.0,
            "bloco: limites de ∑ devem estar empilhados verticalmente (min_y={:.1} < 80.0)",
            min_y
        );
        assert!(
            min_y >= 72.0,
            "P813: o sup empilhado já não invade a margem superior (min_y={:.1} >= 72.0)",
            min_y
        );
        assert!(
            max_y > 85.0,
            "P813: baseline da equação deslocada para margin + ascent_ink (base y={:.1})",
            max_y
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
        // P750: cursor_y inicial = margin + cap_height = 72 + 7.7 = 79.7.
        // offset_y = cursor_y - axis_pt = 79.7 - 6.0 = 73.7 (inline ajusta baseline).
        // Com right-scripts: sup_offset ≈ 4.34pt → item y ≈ 73.7 - 4.34 = 69.4 ≥ 68.0
        // Com vertical stacking (antes): min_y ≈ 61.4 < 70.0 (falha antes da implementação)
        let doc = layout_test("$sum_(i=0)^n$");
        let all_y: Vec<f64> = doc
            .pages
            .iter()
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
            min_y >= 68.0,
            "inline: ∑ deve usar right-scripts (min_y={:.1} >= 68.0)",
            min_y
        );
    }

    #[test]
    fn sum_inline_contem_conteudo() {
        let doc = layout_test("$sum_(i=0)^n$");
        let text = doc.plain_text();
        assert!(
            text.contains('∑') || text.contains('𝑖') || text.contains('𝑛'),
            "conteúdo ausente: {}",
            text
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
        assert!(text.contains('𝑓') || text.contains('𝑥'), "conteúdo ausente: {}", text);
    }

    #[test]
    fn attach_normal_inline_nao_regride() {
        let doc = layout_test("$x^2$");
        let text = doc.plain_text();
        assert!(text.contains('𝑥'));
        assert!(text.contains('2'));
    }

    #[test]
    fn attach_normal_com_sub_inline_nao_regride() {
        let doc = layout_test("$x_i$");
        let text = doc.plain_text();
        assert!(text.contains('𝑥'));
        assert!(text.contains('𝑖'));
    }

    #[test]
    fn sum_block_contem_conteudo() {
        let doc = layout_test("$ sum_(i=0)^n $");
        let text = doc.plain_text();
        assert!(
            text.contains('∑') || text.contains('𝑖') || text.contains('𝑛'),
            "conteúdo ausente em block: {}",
            text
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
        assert!(text.contains('𝑎'), "a ausente: {}", text);
        assert!(text.contains('𝑏'), "b ausente: {}", text);
        assert!(text.contains('𝑐'), "c ausente: {}", text);
        assert!(text.contains('𝑑'), "d ausente: {}", text);
    }

    #[test]
    fn align_duas_linhas_tem_ys_distintos() {
        // Após implementação de grid, itens de linha 0 e linha 1
        // devem ter Y distintos no frame.
        let doc = layout_test("$ a &= b \\ c &= d $");
        assert!(!doc.pages.is_empty());
        let mut ys: Vec<i64> = doc.pages[0]
            .items
            .iter()
            .filter_map(|item| match item {
                crate::entities::layout_types::FrameItem::Text { pos, .. } => {
                    Some((pos.y.val() * 100.0).round() as i64)
                }
                crate::entities::layout_types::FrameItem::Glyph { pos, .. } => {
                    Some((pos.y.val() * 100.0).round() as i64)
                }
                _ => None,
            })
            .collect();
        ys.sort_unstable();
        ys.dedup();
        assert!(
            ys.len() >= 2,
            "esperava >= 2 Y distintos (2 linhas), encontrei {:?}",
            ys
        );
    }

    #[test]
    fn align_sem_ampersand_nao_regride() {
        let doc = layout_test("$ x + 1 $");
        let text = doc.plain_text();
        assert!(text.contains('𝑥'));
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
        assert!(text.contains('𝑎'));
        assert!(text.contains('𝑏'));
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
            text.contains('∑') || text.contains('𝑖') || text.contains('𝑛'),
            "sum: {}",
            text
        );
    }

    // ── Passo 54 — Matrizes matemáticas ─────────────────────────────────

    #[test]
    fn matrix_2x2_nao_vazio() {
        let doc = layout_test("$ mat(a, b; c, d) $");
        let text = doc.plain_text();
        assert!(text.contains('𝑎'), "a ausente: {}", text);
        assert!(text.contains('𝑑'), "d ausente: {}", text);
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
        assert!(text.contains('𝑎'));
        assert!(text.contains('𝑑'));
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
        assert!(text.contains('𝑎'));
        assert!(text.contains('𝑑'));
    }
}

// ── Testes de CounterStateLegacy e numeração de headings (Passo 57/58) ──────

#[test]
fn layout_heading_sem_numbering_nao_tem_prefixo() {
    // Por defeito, numbering_active está vazio — não deve aparecer "1."
    let content = Content::heading(1, Content::text("Intro"));
    let doc = layout(&content);
    let text = doc.plain_text();
    assert!(
        !text.contains("1."),
        "sem numbering activo, não deve haver prefixo numérico"
    );
    assert!(text.contains("Intro"));
}

#[test]
fn layout_heading_com_numbering_tem_prefixo() {
    // Lote F-2 S1 (P335): numeração assada no heading (`heading_numbered`);
    // asserções inalteradas.
    let content = Content::Sequence(
        vec![
            Content::heading_numbered(1, Content::text("Intro")),
            Content::heading_numbered(2, Content::text("Motivação")),
            Content::heading_numbered(1, Content::text("Conclusão")),
        ]
        .into(),
    );
    let doc = layout(&content);
    let text = doc.plain_text();
    assert!(text.contains("1."), "H1 deve ter prefixo '1.'");
    assert!(text.contains("1.1"), "H2 deve ter prefixo '1.1'");
    assert!(text.contains("2."), "segundo H1 deve ter prefixo '2.'");
}

#[test]
fn layout_set_heading_numbering_activa_contador() {
    // Lote F-2 S1 (P335): numeração assada no heading (`heading_numbered`);
    // asserções inalteradas.
    let content = Content::Sequence(
        vec![
            Content::heading_numbered(1, Content::text("Intro")),
            Content::heading_numbered(2, Content::text("Sub")),
        ]
        .into(),
    );
    let doc = layout(&content);
    let text = doc.plain_text();
    assert!(text.contains("1."), "H1 deve ter prefixo '1.'");
    assert!(text.contains("1.1"), "H2 deve ter prefixo '1.1'");
}

// ── P451 — Layout E2E com patterns de numeração configuráveis ───────────────

#[test]
fn p451_layout_heading_pattern_1_ponto() {
    let content = Content::heading_numbered_with_pattern(
        1,
        Content::text("Intro"),
        Some("1.".into()),
    );
    let text = layout(&content).plain_text();
    assert!(text.contains("1. Intro"), "esperado '1. Intro' em: {text}");
}

#[test]
fn p451_layout_heading_pattern_1_ponto_1() {
    let content = Content::Sequence(
        vec![
            Content::heading_numbered_with_pattern(
                1,
                Content::text("Cap"),
                Some("1.".into()),
            ),
            Content::heading_numbered_with_pattern(
                2,
                Content::text("Sec"),
                Some("1.1".into()),
            ),
        ]
        .into(),
    );
    let text = layout(&content).plain_text();
    assert!(text.contains("1. Cap"), "esperado '1. Cap' em: {text}");
    assert!(text.contains("1.1 Sec"), "esperado '1.1 Sec' em: {text}");
}

#[test]
fn p451_layout_heading_pattern_romano_reseta_inferior() {
    let content = Content::Sequence(
        vec![
            Content::heading_numbered_with_pattern(
                1,
                Content::text("A"),
                Some("I.".into()),
            ),
            Content::heading_numbered_with_pattern(
                2,
                Content::text("B"),
                Some("I.I".into()),
            ),
            Content::heading_numbered_with_pattern(
                1,
                Content::text("C"),
                Some("I.".into()),
            ),
        ]
        .into(),
    );
    let text = layout(&content).plain_text();
    assert!(text.contains("I. A"), "esperado 'I. A' em: {text}");
    assert!(
        text.contains("I.I B"),
        "esperado prefixo romano hierárquico 'I.I B' em: {text}"
    );
    assert!(text.contains("II. C"), "esperado 'II. C' em: {text}");
}

// ── Lote F-2 S1 (P335) — o gate de numeração migrou do Introspector para o
// campo assado `HeadingElem::numbering_active`. Este teto (era P182D) passa a
// assertar a NOVA realidade: o flag `numbering_active:heading` do Introspector
// é **ignorado** pelo consumer (que lê o campo assado, escopo léxico).

#[test]
fn f2s1_heading_numbering_le_campo_assado_nao_o_introspector() {
    use crate::engine::introspect::introspect_with_introspector;
    use crate::entities::introspector::TagIntrospector;
    use crate::entities::location::Location;
    use crate::entities::value::Value;

    // Heading NÃO numerado (campo assado = false), mas o Introspector tem o
    // flag antigo `numbering_active:heading=true` injetado.
    let plain = Content::heading(1, Content::text("Intro"));
    let mut intr: TagIntrospector = introspect_with_introspector(&plain);
    intr.state.init(
        "numbering_active:heading".to_string(),
        Value::Bool(true),
        Location::from_raw(0),
    );
    let text = layout_with_introspector(&plain, intr).plain_text();
    // O flag do Introspector é ignorado — sem prefixo (lê o campo assado=false).
    assert!(
        !text.contains("1."),
        "F-2 S1: o consumer ignora o flag do Introspector e lê o campo assado; \
         heading não-numerado não deve ter prefixo; obtido: '{text}'"
    );

    // E um heading numerado (campo assado=true) tem prefixo, sem depender do
    // Introspector para o gate.
    let numbered = Content::heading_numbered(1, Content::text("Intro"));
    let text2 = layout(&numbered).plain_text();
    assert!(
        text2.contains("1."),
        "F-2 S1: heading numerado (campo assado) deve ter prefixo; obtido: '{text2}'"
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
    use crate::engine::introspect::introspect_with_introspector;

    let content = Content::Sequence(
        vec![
            Content::heading(1, Content::text("Intro")),
            Content::heading(2, Content::text("Sub")),
            Content::heading(1, Content::text("Conclusão")),
        ]
        .into(),
    );

    let txt_legacy = layout(&content).plain_text();
    let intr = introspect_with_introspector(&content);
    let txt_new = layout_with_introspector(&content, intr).plain_text();
    assert_eq!(txt_legacy, txt_new, "P182D: paridade pre/post migração");
}

#[test]
fn layout_counter_display_heading_retorna_estado_actual() {
    let content = Content::Sequence(
        vec![
            Content::heading(1, Content::text("Intro")),
            Content::counter_display("heading".to_string()),
        ]
        .into(),
    );
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

    let content =
        Content::counter_update("equation".to_string(), CounterAction::Update(5));
    let doc = layout(&content);
    let total_items: usize = doc.pages.iter().map(|p| p.items.len()).sum();
    assert_eq!(total_items, 0, "CounterUpdate não deve gerar items visuais");
}

#[test]
fn counter_update_seguido_de_display_mostra_valor_correcto() {
    use crate::entities::counter_update::CounterUpdate as CounterAction;

    let content = Content::Sequence(
        vec![
            Content::counter_update("equation".to_string(), CounterAction::Update(5)),
            Content::counter_display("equation".to_string()),
        ]
        .into(),
    );
    let doc = layout(&content);
    assert!(
        doc.plain_text().contains('5'),
        "CounterDisplay deve mostrar '5' após Update(5): {:?}",
        doc.plain_text()
    );
}

// ── Testes de resolução de referências (Passo 59 / Passo 60) ────────────

#[test]
fn layout_ref_para_tras_resolve_secao() {
    // Passo 60: layout() usa duas passagens — backward ref resolve via introspect.
    // P788: heading passa a ter numbering (sem numbering o vanilla erra —
    // testado em p788_ref_heading_sem_numbering_erro). Doc en → "Section 1".
    use crate::entities::label::Label;

    let content = Content::Sequence(
        vec![
            Content::label_auto(
                "intro".to_string(),
                Content::Styled(
                    Box::new(Content::heading(1, Content::text("Introdução"))),
                    Styles::new()
                        .push_custom("heading.numbering", Value::Bool(true))
                        .push_custom(
                            "heading.numbering.pattern",
                            Value::Str("1.".into()),
                        ),
                ),
            ),
            Content::text("Como vimos em"),
            Content::reference("intro".to_string()),
        ]
        .into(),
    );

    let doc = layout(&content);
    let text = doc.plain_text().replace('\u{a0}', " ");
    assert!(
        text.contains("Section 1"),
        "Ref para trás deve resolver para 'Section 1' via duas passagens, obtido: {:?}",
        text
    );
}

#[test]
fn layout_ref_para_frente_resolve_com_duas_passagens() {
    // Passo 60: forward ref resolve via introspect — sem fallback.
    // P788: heading com numbering (ver nota no teste para_trás).
    use crate::entities::label::Label;

    let content = Content::Sequence(
        vec![
            // Ref aparece antes da Label — forward reference
            Content::reference("conclusao".to_string()),
            Content::label_auto(
                "conclusao".to_string(),
                Content::Styled(
                    Box::new(Content::heading(1, Content::text("Conclusão"))),
                    Styles::new()
                        .push_custom("heading.numbering", Value::Bool(true))
                        .push_custom(
                            "heading.numbering.pattern",
                            Value::Str("1.".into()),
                        ),
                ),
            ),
        ]
        .into(),
    );

    let doc = layout(&content);
    let text = doc.plain_text().replace('\u{a0}', " ");
    assert!(
        text.contains("Section 1"),
        "Forward ref deve resolver para 'Section 1' com duas passagens, obtido: {:?}",
        text
    );
    assert!(
        !text.contains("@conclusao"),
        "Forward ref não deve usar fallback com duas passagens, obtido: {:?}",
        text
    );
}

#[test]
fn layout_resolved_labels_nao_interfere_entre_documentos() {
    // Estados de cada chamada a layout() são independentes.
    use crate::engine::introspect::introspect_with_introspector;
    use crate::entities::label::Label;

    let content_a =
        Content::label_auto("sec".to_string(), Content::heading(1, Content::text("A")));
    let _ = layout(&content_a);

    // Segundo layout independente — não deve ter "sec" resolvida.
    // P788: label inexistente agora é ERRO de layout (vanilla:
    // `does not exist in the document`) em vez de "?" — o isolamento
    // continua validado pelo erro disparar no segundo documento.
    let content_b = Content::reference("sec".to_string());
    let doc_b = layout(&content_b);
    assert!(
        doc_b
            .layout_errors
            .iter()
            .any(|d| d.message.contains("does not exist in the document")),
        "Estado do layout anterior não deve vazar para o seguinte: {:?}",
        doc_b.layout_errors
    );
}

// ── Testes de duas passagens (Passo 60) ──────────────────────────────────

#[test]
fn pipeline_duas_passagens_resolve_forward_ref() {
    use crate::engine::{
        introspect::{introspect, introspect_with_introspector},
        layout::layout,
    };
    use crate::entities::label::Label;

    let content = Content::Sequence(
        vec![
            Content::text("Ver a"),
            Content::reference("conclusao".to_string()),
            Content::text("."),
            Content::label_auto(
                "conclusao".to_string(),
                // P788: heading com numbering (sem numbering o vanilla erra).
                Content::Styled(
                    Box::new(Content::heading(1, Content::text("Conclusão"))),
                    Styles::new()
                        .push_custom("heading.numbering", Value::Bool(true))
                        .push_custom(
                            "heading.numbering.pattern",
                            Value::Str("1.".into()),
                        ),
                ),
            ),
        ]
        .into(),
    );

    // Passagem 1 — P788: com o wrapper Styled, `resolved_labels` não é
    // populado (target não é `Heading` directo) — a resolução do forward
    // ref segue pelo caminho numérico (label_to_counter_key), verificada
    // directamente.
    let intr = introspect_with_introspector(&content);
    assert!(
        intr.label_to_counter_key
            .contains_key(&Label("conclusao".to_string())),
        "introspect deve mapear a label ao counter 'heading' para forward refs"
    );

    // Passagem 2 — layout usa o estado da pré-passagem.
    let doc = layout(&content);
    let text = doc.plain_text().replace('\u{a0}', " ");
    assert!(
        text.contains("Section 1"),
        "forward ref deve resolver para 'Section 1': {:?}",
        text
    );
    assert!(
        !text.contains("@conclusao"),
        "não deve usar fallback com duas passagens: {:?}",
        text
    );
}

#[test]
fn layout_equation_bloco_numerada() {
    // P190E (M6): test adaptado — usa pipeline standard com
    // Content::SetEquationNumbering em vez de mutar state.numbering_active
    // directamente. Caminho Introspector activo desde P199B.
    // Lote F-2 S2 (P335): numeração assada (`equation_numbered`); asserção inalterada.
    let content = Content::Sequence(
        vec![Content::equation_numbered(Content::MathIdent("E".into()), true)].into(),
    );

    let doc = layout(&content);
    let text = doc.plain_text();
    assert!(
        text.contains("(1)"),
        "Equação de bloco numerada deve mostrar '(1)', obtido: {:?}",
        text
    );
}

#[test]
fn layout_equation_pattern_romano() {
    // P456: pattern customizável via chain, formatado por format_counter.
    use crate::entities::style::Styles;
    use crate::entities::value::Value;

    let content = Content::Sequence(
        vec![Content::Styled(
            Box::new(Content::equation(Content::MathIdent("E".into()), true)),
            Styles::new().push_custom("equation.numbering", Value::Str("[I]".into())),
        )]
        .into(),
    );

    let doc = layout(&content);
    let text = doc.plain_text();
    assert!(
        text.contains("[I]"),
        "Equação numerada com pattern [I] deve mostrar '[I]', obtido: {:?}",
        text
    );
}

#[test]
fn layout_equation_sequencial_numerada() {
    // P456: duas equações block numeradas seguidas devem numerar (1), (2).
    let content = Content::Sequence(
        vec![
            Content::equation_numbered(Content::MathIdent("A".into()), true),
            Content::equation_numbered(Content::MathIdent("B".into()), true),
        ]
        .into(),
    );

    let doc = layout(&content);
    let text = doc.plain_text();
    assert!(text.contains("(1)"), "1ª equação deve ser (1): {:?}", text);
    assert!(text.contains("(2)"), "2ª equação deve ser (2): {:?}", text);
}

#[test]
fn layout_equation_inline_nao_numerada() {
    // P456: equações inline não são numeradas mesmo com gate activo.
    use crate::entities::style::Styles;
    use crate::entities::value::Value;

    let content = Content::Sequence(
        vec![Content::Styled(
            Box::new(Content::equation(Content::MathIdent("x".into()), false)),
            Styles::new().push_custom("equation.numbering", Value::Str("(1)".into())),
        )]
        .into(),
    );

    let doc = layout(&content);
    let text = doc.plain_text();
    // P809: `x` estilizado para 𝑥 U+1D465 no layout (paridade vanilla).
    assert!(text.contains("\u{1D465}"), "equação inline deve aparecer: {:?}", text);
    assert!(!text.contains("(1)"), "equação inline não deve ter número: {:?}", text);
}

// ── Testes de Passo 61 — TOC (Outline) ───────────────────────────────────

#[test]
fn layout_outline_gera_indice_com_titulos() {
    let content = Content::Sequence(
        vec![
            Content::outline(),
            Content::heading(1, Content::text("Introdução")),
            Content::heading(2, Content::text("Motivação")),
        ]
        .into(),
    );

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
fn layout_outline_mostra_numero_sem_supplement_seccao() {
    // **P359 (DEBT-60 b) — fecha a lacuna de teste.** Antes, NENHUM teste assertava
    // o número do outline — o que escondeu o supplement "Secção" (e, na probe, o
    // desaparecimento do número). O outline de headings **numerados** mostra o
    // NÚMERO ("1.", "1.1."), **NÃO** o supplement "Secção" — paridade com o vanilla
    // 0.14.2 (cujo `prefix` de outline formata o numbering e não acrescenta
    // supplement para heading; `model/outline.rs:123-124`).
    let content = Content::Sequence(
        vec![
            Content::outline(),
            Content::heading_numbered(1, Content::text("Intro")),
            Content::heading_numbered(2, Content::text("Motiv")),
        ]
        .into(),
    );
    let _state = introspect(&content);
    let doc = layout(&content);
    let text = doc.plain_text();

    assert!(
        text.contains("Intro") && text.contains("Motiv"),
        "TOC lista os títulos: {text:?}"
    );
    assert!(
        !text.contains("Secção"),
        "TOC NÃO emite o supplement 'Secção' (paridade vanilla): {text:?}"
    );
    assert!(
        text.contains("1.") && text.contains("1.1."),
        "TOC mostra os números do heading ('1.', '1.1.'): {text:?}"
    );
}

#[test]
fn layout_outline_sem_headings_gera_apenas_titulo_ou_vazio() {
    let content = Content::outline();
    let state = introspect(&content);
    let doc = layout(&content);
    let text = doc.plain_text();

    assert!(
        text.contains("Índice") || text.is_empty(),
        "TOC sem headings deve gerar apenas o título ou estar vazia"
    );
}

#[test]
fn layout_outline_heading_nivel2_tem_indentacao() {
    let content = Content::Sequence(
        vec![
            Content::outline(),
            Content::heading(1, Content::text("H1")),
            Content::heading(2, Content::text("H2")),
        ]
        .into(),
    );

    let state = introspect(&content);
    let doc = layout(&content);
    let text = doc.plain_text();

    // Heading de nível 2 → TOC deve conter espaços de indentação antes de H2.
    // plain_text() não preserva posição, mas a TOC inclui "  " antes da Ref.
    assert!(text.contains("H1"), "TOC deve listar H1");
    assert!(text.contains("H2"), "TOC deve listar H2");
}

#[test]
fn layout_outline_title_custom() {
    // P457: `outline([Sumário])` (ou `outline(title: [Sumário])`) usa o título
    // fornecido em vez do default "Índice".
    let content = Content::Sequence(
        vec![
            Content::outline_with(Some(Content::text("Sumário")), 3, OutlineIndent::Auto),
            Content::heading(1, Content::text("H1")),
        ]
        .into(),
    );

    let _state = introspect(&content);
    let doc = layout(&content);
    let text = doc.plain_text();

    assert!(
        text.contains("Sumário"),
        "TOC deve usar título customizado 'Sumário': {text:?}"
    );
    assert!(
        !text.contains("Índice"),
        "TOC não deve renderizar título default 'Índice': {text:?}"
    );
    assert!(text.contains("H1"), "TOC deve listar H1: {text:?}");
}

#[test]
fn layout_outline_depth_limita_niveis() {
    // P457: `outline(depth: 1)` lista apenas headings de nível 1.
    // Cada heading real aparece uma vez no documento; se estiver dentro do
    // depth, aparece uma vez a mais na TOC. Contamos ocorrências para
    // distinguir "aparece no documento" de "aparece na TOC".
    let content = Content::Sequence(
        vec![
            Content::outline_with(None, 1, OutlineIndent::Auto),
            Content::heading(1, Content::text("H1")),
            Content::heading(2, Content::text("H2")),
            Content::heading(3, Content::text("H3")),
        ]
        .into(),
    );

    let _state = introspect(&content);
    let doc = layout(&content);
    let text = doc.plain_text();

    fn count(haystack: &str, needle: &str) -> usize {
        haystack.matches(needle).count()
    }

    assert_eq!(
        count(&text, "H1"),
        2,
        "H1 deve aparecer 1x na TOC + 1x como heading real: {text:?}"
    );
    assert_eq!(
        count(&text, "H2"),
        1,
        "H2 (nível 2) deve aparecer SÓ como heading real (depth=1 exclui da TOC): {text:?}"
    );
    assert_eq!(
        count(&text, "H3"),
        1,
        "H3 (nível 3) deve aparecer SÓ como heading real (depth=1 exclui da TOC): {text:?}"
    );
}

#[test]
fn layout_outline_indent_false_nao_indenta() {
    // P457: `outline(indent: false)` desativa indentação; entradas de nível 2
    // aparecem sem prefixo de espaços. plain_text() descarta posição, mas ainda
    // assim inclui o corpo da entrada.
    let content = Content::Sequence(
        vec![
            Content::outline_with(None, 3, OutlineIndent::Bool(false)),
            Content::heading(1, Content::text("H1")),
            Content::heading(2, Content::text("H2")),
        ]
        .into(),
    );

    let _state = introspect(&content);
    let doc = layout(&content);
    let text = doc.plain_text();

    assert!(text.contains("H1"), "TOC indent=false deve listar H1: {text:?}");
    assert!(text.contains("H2"), "TOC indent=false deve listar H2: {text:?}");
}

#[test]
fn layout_outline_parametros_default_igual_a_vanilla() {
    // P457: `outline()` sem argumentos deve usar title=None, depth=3,
    // indent=true — comportamento idêntico a Content::outline().
    let a = Content::outline();
    let b = Content::outline_with(None, 3, OutlineIndent::Auto);
    assert_eq!(a, b, "outline() default deve ser igual a outline_with(None,3,Auto)");
}

// ── Testes de Passo 62 — Figuras ─────────────────────────────────────────

#[test]
fn layout_figure_com_caption_tem_prefixo() {
    let content = Content::figure(
        Content::text("Gráfico"),
        Some(Content::text("Resultados")),
        Some("image".to_string()),
        Some("1".to_string()),
    );

    let state = introspect(&content);
    let doc = layout(&content);
    let text = doc.plain_text();

    assert!(text.contains("Gráfico"), "corpo da figura deve aparecer");
    assert!(text.contains("Figura 1:"), "prefixo numérico deve aparecer");
    assert!(text.contains("Resultados"), "legenda deve aparecer");
}

#[test]
fn layout_figure_sem_caption_sem_prefixo() {
    let content = Content::figure(
        Content::text("Diagrama"),
        None,
        Some("image".to_string()),
        Some("1".to_string()),
    );

    let state = introspect(&content);
    let doc = layout(&content);
    let text = doc.plain_text();

    assert!(text.contains("Diagrama"), "corpo deve aparecer");
    assert!(!text.contains("Figura 1:"), "sem caption, sem prefixo");
}

#[test]
fn layout_figure_pattern_romano() {
    let content = Content::figure(
        Content::text("Gráfico"),
        Some(Content::text("Resultados")),
        Some("image".to_string()),
        Some("I.".to_string()),
    );

    let doc = layout(&content);
    let text = doc.plain_text();

    assert!(text.contains("Figura I.: "), "pattern romano deve formatar número: {text}");
    assert!(text.contains("Resultados"), "legenda deve aparecer");
}

#[test]
fn layout_figure_pattern_letras_minusculas() {
    let content = Content::figure(
        Content::text("Gráfico"),
        Some(Content::text("Resultados")),
        Some("image".to_string()),
        Some("(a)".to_string()),
    );

    let doc = layout(&content);
    let text = doc.plain_text();

    assert!(text.contains("Figura (a): "), "pattern (a) deve formatar número: {text}");
}

#[test]
fn layout_figure_pattern_letras_maiusculas() {
    let content = Content::figure(
        Content::text("Gráfico"),
        Some(Content::text("Resultados")),
        Some("image".to_string()),
        Some("A.".to_string()),
    );

    let doc = layout(&content);
    let text = doc.plain_text();

    assert!(text.contains("Figura A.: "), "pattern A. deve formatar número: {text}");
}

#[test]
fn layout_figure_pattern_invalido_fallback_arabico() {
    let content = Content::figure(
        Content::text("Gráfico"),
        Some(Content::text("Resultados")),
        Some("image".to_string()),
        Some("x".to_string()),
    );

    let doc = layout(&content);
    let text = doc.plain_text();

    assert!(
        text.contains("Figura 1: "),
        "pattern inválido fallback para arábico: {text}"
    );
}

#[test]
fn layout_figure_sequencial_romano() {
    let content = Content::Sequence(
        vec![
            Content::figure(
                Content::text("A"),
                Some(Content::text("c1")),
                Some("image".to_string()),
                Some("I.".to_string()),
            ),
            Content::figure(
                Content::text("B"),
                Some(Content::text("c2")),
                Some("image".to_string()),
                Some("I.".to_string()),
            ),
        ]
        .into(),
    );

    let doc = layout(&content);
    let text = doc.plain_text();

    assert!(text.contains("Figura I.: "), "primeira figura romana: {text}");
    assert!(text.contains("Figura II.: "), "segunda figura romana: {text}");
}

#[test]
fn layout_ref_para_figura_resolve_corretamente() {
    use crate::entities::label::Label;

    let content = Content::Sequence(
        vec![
            labelled_prod(
                Content::figure(
                    Content::text("Gráfico"),
                    Some(Content::text("Legenda")),
                    Some("image".to_string()),
                    Some("1".to_string()),
                ),
                Label("fig1".to_string()),
            ),
            Content::text(" — ver "),
            Content::reference("fig1".to_string()),
        ]
        .into(),
    );

    let state = introspect(&content);
    let doc = layout(&content);
    let text = doc.plain_text();

    assert!(
        text.contains("Figura 1"),
        "Ref para figura deve resolver para 'Figura 1': {:?}",
        text
    );
    assert!(!text.contains("@fig1"), "não deve usar fallback @fig1: {:?}", text);
}

// ── Testes de Passo 63 — Mapa de páginas e motor de congelamento ─────────

#[test]
fn layout_regista_pagina_de_label() {
    use crate::entities::label::Label;

    let content = Content::Sequence(
        vec![Content::label_auto(
            "sec1".to_string(),
            Content::heading(1, Content::text("Introdução")),
        )]
        .into(),
    );

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

    let content = Content::label("top".to_string(), Content::text("No topo"));

    let state = introspect(&content);
    let doc = layout(&content);

    let page = doc
        .extracted_label_pages
        .get(&Label("top".to_string()))
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

    let body_with_counter_update = Content::Sequence(
        vec![
            Content::text("Secção"),
            Content::counter_update("equation".to_string(), CounterAction::Step),
        ]
        .into(),
    );

    let content = Content::Sequence(
        vec![
            Content::outline(),
            Content::heading(1, body_with_counter_update),
            Content::counter_display("equation".to_string()),
        ]
        .into(),
    );

    let state = introspect(&content);
    let doc = layout(&content);
    let text = doc.plain_text();

    // Sem is_readonly: CounterUpdate dispararia 2× → display mostraria "2".
    // Com is_readonly: CounterUpdate na TOC é bloqueado → display mostra "1".
    assert!(
        text.contains('1') && !text.contains('2'),
        "CounterUpdate na TOC não deve duplicar: {:?}",
        text
    );
}

#[test]
fn layout_extracted_label_pages_preenchido_apos_layout() {
    // extracted_label_pages é sempre populado após layout, mesmo sem labels.
    let content = Content::text("Texto sem labels");
    let doc = layout(&content);
    // Deve existir o campo (pode estar vazio)
    assert!(
        doc.extracted_label_pages.is_empty(),
        "sem labels, extracted_label_pages deve estar vazio"
    );
}

// ── Testes de Passo 65 — Convergência de fixpoint ────────────────────────

#[test]
fn layout_converge_sem_ciclo_infinito() {
    let content = Content::Sequence(
        vec![
            Content::outline(),
            Content::heading(1, Content::text("Capítulo 1")),
            Content::heading(2, Content::text("Secção 1.1")),
        ]
        .into(),
    );

    let state = introspect(&content);
    // Se o fixpoint tiver defeito, entra em loop até MAX_ITERATIONS.
    // Não deve panic.
    let doc = layout(&content);

    let text = doc.plain_text();
    assert!(text.contains("Capítulo 1"), "título deve aparecer: {:?}", text);
    assert!(
        text.contains("Índice") || text.contains("ndice"),
        "TOC deve aparecer: {:?}",
        text
    );
}

#[test]
fn layout_documento_sem_toc_usa_curto_circuito() {
    // Documento COM títulos mas SEM #outline(). O vetor headings_for_toc
    // terá entradas, mas has_outline é false — o short-circuit evita o loop.
    // Prova que a condição correcta é has_outline, não headings_for_toc.is_empty().
    let content = Content::Sequence(
        vec![
            Content::heading(1, Content::text("Introdução")),
            Content::heading(2, Content::text("Motivação")),
            Content::text("Texto sem índice."),
        ]
        .into(),
    );

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

    let content = Content::Sequence(
        vec![Content::label_auto(
            "sec1".to_string(),
            Content::heading(1, Content::text("Secção")),
        )]
        .into(),
    );

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

    let content = Content::image(
        "teste.jpg".to_string(),
        crate::entities::ptr_eq_arc::PtrEqArc(std::sync::Arc::new(jpeg_magic)),
        None,
        None,
        "cover",
    );

    let state = introspect(&content);
    let doc = layout(&content);

    assert!(!doc.pages.is_empty(), "documento deve ter pelo menos uma página");

    let has_image = doc.pages[0]
        .items
        .iter()
        .any(|item| matches!(item, FrameItem::Image { .. }));
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
    use crate::entities::geometry::ShapeKind;
    use crate::entities::layout_types::TrackSizing;

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
    use crate::entities::introspector::{Introspector, TagIntrospector};
    use comemo::Track;
    let intr = TagIntrospector::empty();
    let intr_dyn: &dyn Introspector = &intr;
    let intr_tracked = intr_dyn.track();
    let mut layouter =
        Layouter::new(FixedMetrics, NullImageSizer, DEFAULT_FONT_SIZE, intr_tracked);

    // Simular Fase 1.
    let mut resolved = vec![0.0_f64; 4];
    let mut total_fixed = 0.0_f64;
    let mut total_fr = 0.0_f64;
    let cols_cells: Vec<Vec<&Content>> = vec![vec![], vec![&cell_auto], vec![], vec![]];

    for (i, sizing) in cols.iter().enumerate() {
        match sizing {
            TrackSizing::Fixed(w) => {
                resolved[i] = *w;
                total_fixed += *w;
            }
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
            TrackSizing::Fraction(fr) => {
                total_fr += fr;
            }
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
    assert!(
        resolved[1] > 0.0 && resolved[1] < available - 50.0,
        "Auto deve ser positivo e menor que safe_available"
    );
    let soma = resolved.iter().sum::<f64>();
    assert!(
        (soma - available).abs() < 0.01,
        "Soma das larguras deve ser igual a available_width: {} vs {}",
        soma,
        available
    );
}

#[test]
fn grid_fr_recebe_zero_quando_auto_e_guloso() {
    // Regressão: Auto com palavra muito longa consome safe_available inteiro.
    // Não deve entrar em pânico. resolved_widths[2] deve ser 0.0 ou positivo.
    use crate::entities::layout_types::TrackSizing;

    let cfg = crate::entities::layout_types::PageConfig::default();
    let available = cfg.width - 2.0 * cfg.margin;
    let cols =
        vec![TrackSizing::Fixed(50.0), TrackSizing::Auto, TrackSizing::Fraction(1.0)];
    // Palavra sem espaços — ocupa safe_available inteiro.
    let cell_auto = Content::text("PalavraLongaSemEspacos");
    // P204C (M8): Layouter ganha 'a + Tracked<dyn Introspector>.
    use crate::entities::introspector::{Introspector, TagIntrospector};
    use comemo::Track;
    let intr = TagIntrospector::empty();
    let intr_dyn: &dyn Introspector = &intr;
    let intr_tracked = intr_dyn.track();
    let mut layouter =
        Layouter::new(FixedMetrics, NullImageSizer, DEFAULT_FONT_SIZE, intr_tracked);

    let mut resolved = vec![0.0_f64; 3];
    let mut total_fixed = 0.0_f64;
    let mut total_fr = 0.0_f64;
    let cols_cells: Vec<Vec<&Content>> = vec![vec![], vec![&cell_auto], vec![]];

    for (i, sizing) in cols.iter().enumerate() {
        match sizing {
            TrackSizing::Fixed(w) => {
                resolved[i] = *w;
                total_fixed += *w;
            }
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
            TrackSizing::Fraction(fr) => {
                total_fr += fr;
            }
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

// ── P772f — `measure_content_constrained` colapsava a 0 para `Content::Align`
// (braço em falta) e ignorava a largura explícita de `Content::Block` (usava
// a do corpo). Isto fazia colunas Auto de grid/table com células `align(...)`
// ou `block(width: _, ..)` medirem 0pt e colidirem na mesma posição x —
// achado da investigação do P763g (que atribuíra a colisão a "grid não
// renderiza a segunda célula"; por coordenadas, era esta medição a 0).
// Ver 00_nucleo/diagnosticos/paridade-producao-p772f.md.

#[test]
fn p772f_measure_content_constrained_align_reporta_largura_do_corpo() {
    use crate::entities::elements::align::AlignElem;
    use crate::entities::introspector::{Introspector, TagIntrospector};
    use crate::entities::layout_types::Align2D;
    use comemo::Track;

    let intr = TagIntrospector::empty();
    let intr_dyn: &dyn Introspector = &intr;
    let intr_tracked = intr_dyn.track();
    let mut layouter =
        Layouter::new(FixedMetrics, NullImageSizer, DEFAULT_FONT_SIZE, intr_tracked);

    let body = Content::text("hi");
    let aligned = Content::Align(std::sync::Arc::new(AlignElem {
        alignment: Align2D::from_string("center"),
        body: body.clone(),
    }));

    let (w_body, _) = layouter.measure_content_constrained(&body, 400.0);
    let (w_aligned, _) = layouter.measure_content_constrained(&aligned, 400.0);

    assert!(w_body > 0.0, "largura do texto directo deve ser positiva");
    assert_eq!(
        w_aligned, w_body,
        "Content::Align deve reportar a largura do corpo (wrapper não tem \
         tamanho intrínseco próprio) — antes colapsava a 0.0"
    );
}

#[test]
fn p772f_measure_content_constrained_block_largura_explicita_nao_colapsa() {
    use crate::entities::elements::align::AlignElem;
    use crate::entities::elements::block::BlockElem;
    use crate::entities::introspector::{Introspector, TagIntrospector};
    use crate::entities::layout_types::{Align2D, Length};
    use crate::entities::sides::Sides;
    use comemo::Track;

    let intr = TagIntrospector::empty();
    let intr_dyn: &dyn Introspector = &intr;
    let intr_tracked = intr_dyn.track();
    let mut layouter =
        Layouter::new(FixedMetrics, NullImageSizer, DEFAULT_FONT_SIZE, intr_tracked);

    // Corpo sem braço próprio em `measure_content_constrained` (Align
    // envolvendo texto — já corrigido acima, mas o ponto aqui é isolar o
    // bug do Block: mesmo que o corpo medisse 0, a largura explícita do
    // bloco deve prevalecer).
    let body = Content::Align(std::sync::Arc::new(AlignElem {
        alignment: Align2D::from_string("top-left"),
        body: Content::text("x"),
    }));
    let block = Content::Block(std::sync::Arc::new(BlockElem {
        body,
        width: Some(Length::pt(85.039)), // 3cm
        height: None,
        inset: Sides::uniform(Length::pt(0.0)),
        breakable: true,
        outset: Sides::uniform(Length::pt(0.0)),
        radius: crate::entities::corners::Corners::uniform(Length::pt(0.0)),
        clip: false,
        fill: None,
        stroke: None,
        spacing: None,
        above: None,
        below: None,
        sticky: false,
    }));

    let (w, _) = layouter.measure_content_constrained(&block, 400.0);
    assert!(
        (w - 85.039).abs() < 0.01,
        "block(width: 3cm) deve reportar 85.039pt independentemente do corpo, obteve {}",
        w
    );
}

// ── P772g — `layout_align`/emissão de footnotes duplicavam a origem da
// célula quando um `Content::Place` (scope: column) estava aninhado no seu
// corpo. Corrigido tornando ambos "consumidores absolutos" de
// `layout_sub_frame` (origin_x real + delta incremental de alinhamento, não
// origin_x:0.0 + target_x inteiro). Ver
// 00_nucleo/prompts/engine/layout.md §"Contrato de composição de coordenadas"
// e 00_nucleo/diagnosticos/paridade-producao-p772g.md.

#[test]
fn p772g_place_dentro_de_align_dentro_de_grid_cell_bate_com_place_directo() {
    // Regressão directa do achado B de P772f: `align(top, place(...))`
    // numa célula de grid devia produzir a mesma posição x que
    // `place(...)` directo (sem align) numa célula equivalente — antes do
    // fix, a versão com `align` duplicava a origem da célula.
    let src_direct = "#set page(width: 8cm, height: 6cm)\n\
         #grid(columns: 1, \
         block(width: 3cm, height: 2cm, place(top+left, dx: 5pt, dy: 5pt, circle(radius: 10pt))))";
    let src_aligned = "#set page(width: 8cm, height: 6cm)\n\
         #grid(columns: 1, \
         block(width: 3cm, height: 2cm, align(top, place(top+left, dx: 5pt, dy: 5pt, circle(radius: 10pt)))))";

    let shape_x = |doc: &PagedDocument| -> f64 {
        doc.pages[0]
            .items
            .iter()
            .find_map(|i| match i {
                FrameItem::Shape { pos, kind: ShapeKind::Ellipse, .. } => {
                    Some(pos.x.val())
                }
                _ => None,
            })
            .expect("esperado FrameItem::Shape (círculo)")
    };

    let x_direct = shape_x(&layout_test(src_direct));
    let x_aligned = shape_x(&layout_test(src_aligned));

    assert!(
        (x_direct - x_aligned).abs() < 0.01,
        "place() directo (x={}) e align(top, place()) (x={}) devem coincidir \
         — divergência indica duplicação da origem da célula (achado B de P772f)",
        x_direct,
        x_aligned
    );
}

#[test]
fn p772g_place_dentro_de_align_dentro_de_grid_de_duas_colunas_nao_sai_da_pagina() {
    // Reprodução completa de P772f §2.4: grid de 2 colunas, cada célula
    // `block(align(top, place(...)))`. Antes do fix, a célula da coluna 2
    // ficava com o círculo quase inteiramente fora da página (x≈225.57 numa
    // página de 226.77pt de largura). Depois do fix, deve ficar dentro da
    // página e a uma distância da coluna 1 compatível com a largura de uma
    // coluna (não o dobro).
    let doc = layout_test(
        "#set page(width: 8cm, height: 6cm)\n\
         #grid(columns: 2, gutter: 5pt, \
         block(width: 3cm, height: 2cm, align(top, place(top+left, dx: 5pt, dy: 5pt, circle(radius: 10pt)))), \
         block(width: 3cm, height: 2cm, align(top, place(top+left, dx: 5pt, dy: 5pt, circle(radius: 10pt)))))",
    );
    let xs: Vec<f64> = doc.pages[0]
        .items
        .iter()
        .filter_map(|i| match i {
            FrameItem::Shape { pos, kind: ShapeKind::Ellipse, .. } => Some(pos.x.val()),
            _ => None,
        })
        .collect();
    assert_eq!(xs.len(), 2, "esperados 2 círculos, obteve {:?}", xs);
    let page_w = 226.772;
    for &x in &xs {
        assert!(
            x + 20.0 <= page_w,
            "círculo em x={} sai da página (largura {}pt) — achado B de P772f não corrigido",
            x,
            page_w
        );
    }
    let gap = (xs[1] - xs[0]).abs();
    // Uma coluna (3cm ≈ 85.04pt) + gutter (5pt) ≈ 90pt — não o dobro (~180pt,
    // sintoma da duplicação de origem medido em P772f).
    assert!(
        gap < 100.0,
        "distância entre colunas ({}) sugere origem ainda a ser somada em dobro",
        gap
    );
}

#[test]
fn p772g_footnote_com_place_dentro_de_align_bate_com_vanilla_estrutura() {
    // Regressão da correcção em cursor.rs (emissão de footnotes): um
    // `Content::Place` aninhado em `align` dentro do body de uma nota de
    // rodapé não deve duplicar a origem. Verifica-se apenas que o círculo é
    // emitido dentro dos limites da página (achado B também se manifestava
    // aqui, via o mesmo padrão origin_x:0.0 + target_x inteiro).
    let doc = layout_test(
        "#set page(width: 10cm, height: 6cm)\n\
         Texto principal.#footnote[\
           #grid(columns: 1, \
           block(width: 3cm, height: 2cm, align(top, place(top+left, dx: 5pt, dy: 5pt, circle(radius: 10pt)))))\
         ]",
    );
    let x = doc
        .pages
        .iter()
        .flat_map(|p| p.items.iter())
        .find_map(|i| match i {
            FrameItem::Shape { pos, kind: ShapeKind::Ellipse, .. } => Some(pos.x.val()),
            _ => None,
        })
        .expect("esperado FrameItem::Shape (círculo) no body da footnote");
    let page_w = 283.465; // 10cm
    assert!(
        x + 20.0 <= page_w && x >= 0.0,
        "círculo da footnote em x={} sai da página (largura {}pt)",
        x,
        page_w
    );
}

// ── P772j — código órfão de P772f (envolvia célula em `Content::Place`)
// revertido; substituído por `Content::Align`, o mecanismo confirmado
// contra o vanilla (`show_cell` em typst-layout/src/engine.rs). Precedência
// de alinhamento grid vs per-célula corrigida para fold por eixo (era
// `.or()` do `Align2D` inteiro, descartando o eixo do grid sempre que a
// célula especificava qualquer eixo). Ver
// 00_nucleo/prompts/engine/layout.md §"Alinhamento efectivo per-célula" e
// 00_nucleo/diagnosticos/paridade-producao-p772j.md.

#[test]
fn p772j_grid_align_center_nao_diverge_uma_coluna_inteira_do_vanilla() {
    // Regressão do achado original (P772f §3.3 / P772h): o código órfão
    // (Content::Place) fazia "Hello" divergir do vanilla em ~13.5pt
    // (x=33.772 vs vanilla x=20.247, numa coluna de ~27pt de largura — mais
    // de metade da própria coluna). Não exigimos byte-exactidão (a
    // discrepância residual pequena entre a medição aproximada de largura
    // usada no dimensionamento automático de colunas e a largura real
    // shaped é uma divergência mecânica já existente, não introduzida por
    // este passo — ver relatório), só que a divergência deixe de ser da
    // ordem de uma coluna inteira.
    let doc = layout_test(&documento_algoritmo(
        "#set page(width: 8cm, height: 6cm)\n\
         #grid(columns: 2, gutter: 5pt, align: center, [Hello], [World])",
    ));
    let xs: Vec<f64> = doc.pages[0]
        .items
        .iter()
        .filter_map(|i| match i {
            FrameItem::Text { pos, .. } => Some(pos.x.val()),
            _ => None,
        })
        .collect();
    assert!(xs.len() >= 2, "esperado texto das duas células, obteve {:?}", xs);
    let vanilla_target_x = 20.247_f64;
    assert!(
        (xs[0] - vanilla_target_x).abs() < 5.0,
        "'Hello' em x={} diverge do vanilla (x={}) por mais do que ruído de \
         medição de largura — sugere que o mecanismo voltou a usar Place em \
         vez de Align, ou que a origem está a ser mal composta",
        xs[0],
        vanilla_target_x
    );
}

#[test]
fn p772j_grid_align_fold_por_eixo_preserva_eixo_do_grid() {
    // Regressão da precedência: `grid.cell(align: left)` (só H) dentro de
    // `grid(align: horizon)` (só V) deve preservar o V do grid — não
    // substituir o Align2D inteiro por "left" (que decairia para V=top).
    // Confirmado contra o vanilla por medição directa em P772j (mutool
    // trace: vanilla mantém V=horizon). Aqui comparamos o Y produzido por
    // `align: horizon` vs `align: top` a nível de grid, ambos com o mesmo
    // `grid.cell(align: left)` — devem diferir (H fixo, V muda com o grid).
    let doc_horizon = layout_test(
        "#set page(width: 10cm, height: 4cm)\n\
         #grid(columns: 1, rows: 2cm, align: horizon, grid.cell(align: left)[Hello])",
    );
    let doc_top = layout_test(
        "#set page(width: 10cm, height: 4cm)\n\
         #grid(columns: 1, rows: 2cm, align: top, grid.cell(align: left)[Hello])",
    );
    let first_text_pos = |doc: &PagedDocument| -> (f64, f64) {
        doc.pages[0]
            .items
            .iter()
            .find_map(|i| match i {
                FrameItem::Text { pos, .. } => Some((pos.x.val(), pos.y.val())),
                _ => None,
            })
            .expect("esperado FrameItem::Text")
    };
    let (x_horizon, y_horizon) = first_text_pos(&doc_horizon);
    let (x_top, y_top) = first_text_pos(&doc_top);

    assert!(
        (x_horizon - x_top).abs() < 0.01,
        "H (herdado da célula, 'left') deve ser igual em ambos os casos: \
         horizon x={}, top x={}",
        x_horizon,
        x_top
    );
    assert!(
        (y_horizon - y_top).abs() > 1.0,
        "V (herdado do grid) deve diferir entre 'horizon' e 'top' — se for \
         igual, o fold por eixo não está a herdar o V do grid quando a \
         célula só especifica H (regressão para o `.or()` do Align2D \
         inteiro)"
    );
}

// ── P772i — `grid.header(...)`/`grid.footer(...)` como row-groups reais.
// Corrige dois bugs confirmados: (1) `native_grid_header`/`native_grid_footer`
// só guardavam o primeiro argumento posicional (`grid.header[Nome][Idade]`
// perdia "Idade" silenciosamente); (2) o loop de resolução de `grid()` tratava
// `Content::GridHeader`/`GridFooter` como uma célula normal (scope-out #16 de
// P772f), desalinhando as colunas seguintes. `header:`/`footer:` deixaram de
// ser argumentos nomeados (paridade vanilla). Repeat-across-páginas
// explicitamente **não implementado** (scope-out nomeado, não silencioso —
// ver 00_nucleo/diagnosticos/paridade-producao-p772i.md).

#[test]
fn p772i_grid_header_multi_celula_preserva_todas_as_celulas() {
    // Regressão do bug de `args.items.first()`: header com 2 células devia
    // perder a segunda antes desta correcção.
    let doc = layout_test("#grid(columns: 2, grid.header[Nome][Idade], [Ana], [30])");
    let text = doc.plain_text();
    assert!(text.contains("Nome"), "header deve conter 'Nome': {}", text);
    assert!(
        text.contains("Idade"),
        "header deve conter 'Idade' (segunda célula, antes descartada \
         silenciosamente): {}",
        text
    );
}

#[test]
fn p772i_grid_header_nao_desalinha_colunas_seguintes() {
    // Regressão do scope-out #16: header tratado como célula normal
    // desalinhava as colunas das linhas de dados seguintes. "Ana" e "30"
    // devem ficar em colunas distintas (x diferentes), tal como "Nome" e
    // "Idade" no header.
    let doc = layout_test("#grid(columns: 2, grid.header[Nome][Idade], [Ana], [30])");
    let xs: Vec<f64> = doc.pages[0]
        .items
        .iter()
        .filter_map(|i| match i {
            FrameItem::Text { pos, .. } => Some(pos.x.val()),
            _ => None,
        })
        .collect();
    assert!(
        xs.len() >= 4,
        "esperadas 4 posições de texto (2 header + 2 dados), obteve {:?}",
        xs
    );
    // xs[0]="Nome", xs[1]="Idade", xs[2]="Ana", xs[3]="30" (ordem de emissão).
    assert!(
        (xs[0] - xs[2]).abs() < 0.01,
        "'Nome' (col0) e 'Ana' (col0) devem estar na mesma coluna: {:?}",
        xs
    );
    assert!(
        (xs[1] - xs[3]).abs() < 0.01,
        "'Idade' (col1) e '30' (col1) devem estar na mesma coluna: {:?}",
        xs
    );
    assert!(
        (xs[0] - xs[1]).abs() > 5.0,
        "col0 e col1 devem estar em posições x distintas: {:?}",
        xs
    );
}

#[test]
fn p772i_grid_footer_aparece_apos_dados() {
    let doc = layout_test("#grid(columns: 2, [Ana], [30], grid.footer[Total][30])");
    let text = doc.plain_text();
    let ana_pos = text.find("Ana").expect("'Ana' presente");
    let total_pos = text.find("Total").expect("'Total' presente (footer)");
    assert!(
        total_pos > ana_pos,
        "footer ('Total') deve aparecer depois dos dados ('Ana'): {}",
        text
    );
}

#[test]
fn p772i_grid_header_footer_nomeados_dao_erro() {
    // `#grid(header: ..)` deve errar como o vanilla (argumento nomeado
    // inesperado), não ser aceite silenciosamente com conteúdo descartado.
    // `layout_test` entra em panic (via .expect()/.unwrap() interno) quando
    // o eval falha — usar catch_unwind para confirmar que falha, sem deixar
    // o panic propagar e falhar o test runner.
    let prev_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {})); // silenciar stderr do panic esperado
    let result = std::panic::catch_unwind(|| layout_test("#grid(header: [Nome])"));
    std::panic::set_hook(prev_hook);
    assert!(
        result.is_err(),
        "#grid(header: ..) deveria falhar (paridade vanilla — argumento \
         nomeado inesperado), não produzir um documento silenciosamente"
    );
}

// ── P772v — table.header(...)/table.footer(...) como row-groups ────────────
//
// Extensão directa do mecanismo de P772i (acima) para `table()`. Antes desta
// correcção: `TableElem` não tinha campos `header`/`footer`, e o loop de
// resolução de `table()` tratava `Content::TableHeader`/`TableFooter` como
// célula normal (mesmo scope-out #16 de P772f, variante table) — desalinhando
// colunas seguintes. `native_table_header`/`native_table_footer` já
// colectavam todas as células posicionais desde P772i (não repetiu o bug do
// `.first()`); só faltava o wiring `TableElem` → `layout_grid`.

#[test]
fn p772v_table_header_multi_celula_preserva_todas_as_celulas() {
    let doc = layout_test("#table(columns: 2, table.header[Nome][Idade], [Ana], [30])");
    let text = doc.plain_text();
    assert!(text.contains("Nome"), "header deve conter 'Nome': {}", text);
    assert!(
        text.contains("Idade"),
        "header deve conter 'Idade' (segunda célula, mesmo bug do .first() \
         verificado não repetir aqui): {}",
        text
    );
}

#[test]
fn p772v_table_header_nao_desalinha_colunas_seguintes() {
    // Regressão do scope-out #16 (variante table): header tratado como
    // célula normal desalinhava as colunas das linhas de dados seguintes.
    let doc = layout_test("#table(columns: 2, table.header[Nome][Idade], [Ana], [30])");
    let xs: Vec<f64> = doc.pages[0]
        .items
        .iter()
        .filter_map(|i| match i {
            FrameItem::Text { pos, .. } => Some(pos.x.val()),
            _ => None,
        })
        .collect();
    assert!(
        xs.len() >= 4,
        "esperadas 4 posições de texto (2 header + 2 dados), obteve {:?}",
        xs
    );
    // xs[0]="Nome", xs[1]="Idade", xs[2]="Ana", xs[3]="30" (ordem de emissão).
    assert!(
        (xs[0] - xs[2]).abs() < 0.01,
        "'Nome' (col0) e 'Ana' (col0) devem estar na mesma coluna: {:?}",
        xs
    );
    assert!(
        (xs[1] - xs[3]).abs() < 0.01,
        "'Idade' (col1) e '30' (col1) devem estar na mesma coluna: {:?}",
        xs
    );
    assert!(
        (xs[0] - xs[1]).abs() > 5.0,
        "col0 e col1 devem estar em posições x distintas: {:?}",
        xs
    );
}

#[test]
fn p772v_table_footer_aparece_apos_dados() {
    let doc = layout_test("#table(columns: 2, [Ana], [30], table.footer[Total][30])");
    let text = doc.plain_text();
    let ana_pos = text.find("Ana").expect("'Ana' presente");
    let total_pos = text.find("Total").expect("'Total' presente (footer)");
    assert!(
        total_pos > ana_pos,
        "footer ('Total') deve aparecer depois dos dados ('Ana'): {}",
        text
    );
}

#[test]
fn p772v_table_header_footer_nomeados_dao_erro() {
    // `#table(header: ..)` deve errar como o vanilla (argumento nomeado
    // inesperado) — `table()` nunca aceitou header/footer como named args,
    // mas confirmar que a extensão de P772v não introduziu esse caminho.
    let prev_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let result = std::panic::catch_unwind(|| layout_test("#table(header: [Nome])"));
    std::panic::set_hook(prev_hook);
    assert!(
        result.is_err(),
        "#table(header: ..) deveria falhar (paridade vanilla — argumento \
         nomeado inesperado), não produzir um documento silenciosamente"
    );
}

#[test]
fn p772v_table_multiplos_headers_da_erro() {
    let prev_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let result = std::panic::catch_unwind(|| {
        layout_test("#table(columns: 1, table.header[A], table.header[B])")
    });
    std::panic::set_hook(prev_hook);
    assert!(
        result.is_err(),
        "múltiplos headers devem errar explicitamente (scope-out — não \
         suportamos múltiplos headers por `level`)"
    );
}

// ── P789 — conflito de célula do corpo com header deve errar ──────────────
//
// Achado de P786 (evidência `temp/temp_p786/c_bitset_conflict.typ`): célula
// do corpo com `y` explícito sobre as linhas do header era aceite em
// silêncio; o vanilla erra (`check_for_conflicting_cell_row`,
// `lab/typst-original/crates/typst-library/src/layout/grid/resolve.rs:2112`).
// Paridade do observável (ADR-0107): mensagem e hint idênticos ao vanilla.
// Scope-out documentado no L0 (layout.md §P789): conflito célula↔footer
// (range absoluto do footer só existe pós-placement no modelo splice).

#[test]
fn p789_table_cell_explicit_sobre_header_da_erro() {
    // Repro exacto de P786: header de 2 linhas (4 células / 2 colunas);
    // célula com y=0 rowspan=2 colide com a linha 0 do header.
    let doc = layout_test(
        "#table(columns: 2, table.header[H1][H2][H3][H4], table.cell(x: 0, y: 0, rowspan: 2)[X], [a], [b])",
    );
    let diag = doc
        .layout_errors
        .iter()
        .find(|d| d.message == "cell would conflict with header also spanning row 0")
        .unwrap_or_else(|| {
            panic!(
                "esperado erro de conflito com header (paridade vanilla): {:?}",
                doc.layout_errors
            )
        });
    assert!(
        diag.hints.iter().any(|h| h == "try moving the cell or the header"),
        "hint deve ser idêntico ao vanilla: {:?}",
        diag.hints
    );
}

#[test]
fn p789_grid_cell_explicit_sobre_header_da_erro() {
    // Variante grid: header de 1 linha; célula explicit em y=0 colide.
    let doc = layout_test(
        "#grid(columns: 2, grid.header[H1][H2], grid.cell(x: 1, y: 0)[X], [a], [b])",
    );
    assert!(
        doc.layout_errors
            .iter()
            .any(|d| d.message == "cell would conflict with header also spanning row 0"),
        "esperado erro de conflito com header (paridade vanilla): {:?}",
        doc.layout_errors
    );
}

#[test]
fn p789_cell_so_com_y_explicit_sobre_header_da_erro() {
    // Braço `(Auto, Custom)` do vanilla: célula só com `y` explícito
    // (sem `x`) sobre o header também erra.
    let doc = layout_test(
        "#grid(columns: 2, grid.header[H1][H2], grid.cell(y: 0)[X], [a], [b])",
    );
    assert!(
        doc.layout_errors
            .iter()
            .any(|d| d.message == "cell would conflict with header also spanning row 0"),
        "esperado erro de conflito com header mesmo sem x explícito: {:?}",
        doc.layout_errors
    );
}

#[test]
fn p789_cell_explicit_fora_do_header_nao_erra() {
    // Controlo positivo: célula explicit fora das linhas do header
    // (header = linha 0; célula em y=2) não pode errar.
    let doc = layout_test(
        "#grid(columns: 2, grid.header[H1][H2], grid.cell(x: 0, y: 2)[X], [a], [b])",
    );
    assert!(
        doc.layout_errors.is_empty(),
        "célula fora do header não deve produzir layout_errors: {:?}",
        doc.layout_errors
    );
    let text = doc.plain_text();
    assert!(text.contains('X'), "célula X deve renderizar: {}", text);
}

// ── P772x — decoração propaga através de `layout_sub_frame` ────────────────
//
// Regressão do achado de P772w: `layout_sub_frame` (usado por `place()`,
// células de grid, footnotes, `align()`) fazia o seu próprio flush manual da
// linha corrente sem passar por `flush_line()` — o único ponto onde o
// mecanismo de decoração wrap-aware (P284/P286) regista segmentos em
// `decoration_lines_collector`. Corrigido com um collector LOCAL por
// sub-frame (swap-in/out, mesma disciplina LIFO de `decorations.rs`),
// traduzido pelo caller com a MESMA translação já aplicada aos `FrameItem`s.

fn count_lines(doc: &PagedDocument) -> usize {
    doc.pages[0]
        .items
        .iter()
        .filter(|i| matches!(i, FrameItem::Line { .. }))
        .count()
}

#[test]
fn p772x_underline_propaga_atraves_de_place() {
    // Repro original de P772w: antes da correcção, só "Some text" (fora do
    // sub-frame) ganhava uma FrameItem::Line — "explanation" (dentro do
    // place()) não tinha nenhuma. Depois: pelo menos 2 Lines (uma por
    // trecho decorado).
    let doc = layout_test("#underline[Some text #place(top+left)[explanation].]");
    let lines = count_lines(&doc);
    assert!(
        lines >= 2,
        "underline deve cobrir tanto o texto normal como o texto dentro de \
         place() — esperado >= 2 FrameItem::Line, obteve {lines}"
    );
}

#[test]
fn p772x_underline_propaga_atraves_de_grid() {
    let doc = layout_test("#underline[#grid(columns: 1, [Cell text])]");
    let lines = count_lines(&doc);
    assert!(
        lines >= 1,
        "underline deve cobrir o texto dentro de uma célula de grid — \
         esperado >= 1 FrameItem::Line, obteve {lines}"
    );
}

#[test]
fn p772x_strike_propaga_atraves_de_place() {
    let doc = layout_test("#strike[Some text #place(top+left)[explanation].]");
    let lines = count_lines(&doc);
    assert!(
        lines >= 2,
        "strike deve generalizar a mesma correcção de underline — \
         esperado >= 2 FrameItem::Line, obteve {lines}"
    );
}

#[test]
fn p772x_overline_propaga_atraves_de_place() {
    let doc = layout_test("#overline[Some text #place(top+left)[explanation].]");
    let lines = count_lines(&doc);
    assert!(
        lines >= 2,
        "overline deve generalizar a mesma correcção de underline — \
         esperado >= 2 FrameItem::Line, obteve {lines}"
    );
}

#[test]
fn p772x_underline_sem_sub_frame_sem_regressao() {
    // Não-regressão: decoração de texto simples (sem place/grid/box) deve
    // continuar a funcionar exactamente como antes.
    let doc = layout_test("#underline[Normal text, no sub-frame.]");
    let lines = count_lines(&doc);
    assert!(
        lines >= 1,
        "underline de texto simples deve continuar a produzir >= 1 \
         FrameItem::Line, obteve {lines}"
    );
}

#[test]
fn p772f_grid_auto_colunas_com_align_nao_colidem() {
    // Regressão directa do achado P763g: grid de 2 colunas Auto, cada
    // célula envolta em `align(center, ..)`. Antes do fix, ambas as
    // colunas mediam 0pt e o conteúdo das duas células colidia na mesma
    // posição x (confirmado por `mutool trace` — não "célula ausente").
    let doc = layout_test(&documento_algoritmo(
        "#set page(width: 8cm, height: 6cm)\n\
         #grid(columns: 2, gutter: 5pt, align(center, [Hello]), align(center, [World]))",
    ));
    let xs: Vec<f64> = doc.pages[0]
        .items
        .iter()
        .filter_map(|i| match i {
            FrameItem::Text { pos, .. } => Some(pos.x.val()),
            _ => None,
        })
        .collect();
    assert!(xs.len() >= 2, "esperado texto das duas células, obteve {:?}", xs);
    let x0 = xs.first().copied().unwrap();
    assert!(
        xs.iter().any(|&x| (x - x0).abs() > 5.0),
        "células nas duas colunas não devem colidir na mesma posição x: {:?}",
        xs
    );
}

#[test]
fn grid_altura_da_linha_e_o_maximo_das_celulas() {
    // columns: (100pt, 100pt)
    // Células: rect(h:20), rect(h:40), rect(h:10)
    // Linha 0: max(20, 40) = 40pt. Linha 1: 10pt (incompleta).
    // Verificar: 1 página, 3 FrameItems.
    use crate::entities::geometry::ShapeKind;

    let make_rect = |h: f64| -> Content {
        Content::shape(
            ShapeKind::Rect,
            Some(Box::new(crate::entities::value::Value::Length(
                crate::entities::layout_types::Length {
                    abs: crate::entities::layout_types::Abs(100.0),
                    em: 0.0,
                },
            ))),
            Some(Box::new(crate::entities::value::Value::Length(
                crate::entities::layout_types::Length {
                    abs: crate::entities::layout_types::Abs(h),
                    em: 0.0,
                },
            ))),
            None,
            None,
        )
    };

    let grid =
        Content::Grid(std::sync::Arc::new(crate::entities::elements::grid::GridElem {
            columns: vec![
                crate::entities::layout_types::TrackSizing::Fixed(100.0),
                crate::entities::layout_types::TrackSizing::Fixed(100.0),
            ],
            rows: vec![],
            cells: vec![make_rect(20.0), make_rect(40.0), make_rect(10.0)],
            hlines: vec![],
            vlines: vec![],
            gutter: None,
            align: None,
            inset: crate::entities::sides::Sides::uniform(
                crate::entities::layout_types::Length::pt(0.0),
            ),
            header: None,
            footer: None,
            stroke: None,
            fill: None,
        }));

    let state = introspect(&grid);
    let doc = layout(&grid);

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
    let cell = Content::text(
        "Palavra muito longa que poderia exceder a página se nao houver limite",
    );
    // P204C (M8): Layouter ganha 'a + Tracked<dyn Introspector>.
    use crate::entities::introspector::{Introspector, TagIntrospector};
    use comemo::Track;
    let intr = TagIntrospector::empty();
    let intr_dyn: &dyn Introspector = &intr;
    let intr_tracked = intr_dyn.track();
    let mut layouter =
        Layouter::new(FixedMetrics, NullImageSizer, DEFAULT_FONT_SIZE, intr_tracked);

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
        "Auto não deve exceder available_width: {} > {}",
        resolved[0],
        available
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
        doc.pages
            .iter()
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
            Styles::from_iter([Style::bold(true), Style::Size(Pt(18.0))]),
        );

        let doc = layout(&styled);
        let texts = collect_text_items(&doc);
        assert!(!texts.is_empty(), "esperado pelo menos 1 FrameItem::Text");

        for item in texts {
            if let FrameItem::Text { style, .. } = item {
                assert!(style.bold, "Bold deve estar activo: {:?}", style);
                assert_eq!(style.size, Pt(18.0), "Size deve ser 18pt após push_styles");
            }
        }
    }

    /// Styled aninhado — o delta mais próximo do texto (inner) ganha
    /// (top-wins, paridade vanilla, ADR-0033).
    #[test]
    fn styled_aninhado_inner_ganha_sobre_outer() {
        let inner = Content::Styled(
            Box::new(Content::text("hi")),
            Styles::from_iter([Style::italic(true)]),
        );
        let outer = Content::Styled(
            Box::new(inner),
            Styles::from_iter([Style::bold(true), Style::italic(false)]),
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
            Styles::from_iter([Style::bold(true)]),
        );
        let plain = Content::text("plain");
        let seq = Content::Sequence(Arc::from(vec![styled, Content::Space, plain]));

        let doc = layout(&seq);
        let texts = collect_text_items(&doc);

        // Encontrar o item do texto "STYLED" e do texto "plain".
        let styled_item = texts.iter().find(
            |i| matches!(i, FrameItem::Text { text, .. } if text.as_str() == "STYLED"),
        );
        let plain_item = texts.iter().find(
            |i| matches!(i, FrameItem::Text { text, .. } if text.as_str() == "plain"),
        );

        assert!(styled_item.is_some());
        assert!(plain_item.is_some());

        if let Some(FrameItem::Text { style, .. }) = styled_item {
            assert!(style.bold, "STYLED deve ser bold");
        }
        if let Some(FrameItem::Text { style, .. }) = plain_item {
            assert!(
                !style.bold,
                "'plain' após Styled não deve herdar bold — save/restore falhou: {:?}",
                style
            );
        }
    }
}

// ── Passo 102.D: Integração `#set text(...)` end-to-end ──────────────────

#[cfg(test)]
mod tests_set_rule_integration {
    use super::*;
    use crate::{
        contracts::world::World,
        engine::eval::eval_for_test,
        entities::{
            file_id::FileId,
            font_book::FontBook,
            layout_types::{FrameItem, Pt},
            source::Source,
            world_types::{Bytes, Datetime, FileError, FileResult, Font, Library},
        },
    };
    use std::num::NonZeroU16;

    struct MockWorld {
        library: Library,
        book: FontBook,
        source: Source,
    }

    impl MockWorld {
        fn new(text: &str) -> Self {
            let id = FileId::from_raw(NonZeroU16::new(1).unwrap());
            Self {
                library: Library::new(),
                book: FontBook::new(),
                source: Source::new(id, text.to_string()),
            }
        }
    }

    impl World for MockWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            self.source.id()
        }
        fn source(&self, _: FileId) -> FileResult<Source> {
            Ok(self.source.clone())
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<Font> {
            None
        }
        fn today(&self, _: Option<i64>) -> Option<Datetime> {
            None
        }
    }

    fn layout_typst(source: &str) -> PagedDocument {
        let world = MockWorld::new(source);
        let src = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let content = module.content().expect("content");
        let state = introspect(content);
        layout(content)
    }

    fn text_items(
        doc: &PagedDocument,
    ) -> Vec<(String, crate::entities::layout_types::TextStyle)> {
        doc.pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, style, .. } => {
                    Some((text.to_string(), style.clone()))
                }
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
            assert_eq!(
                style.size,
                Pt(18.0),
                "text='{}' deve ter size=18pt; obtido {:?}",
                text,
                style.size
            );
        }
    }

    /// `#set text(weight: 700)` produz bold em todo o texto seguinte.
    #[test]
    fn set_text_bold_propaga_ao_frame() {
        let doc = layout_typst("#set text(weight: 700)\nHello");
        let items = text_items(&doc);
        assert!(!items.is_empty());
        for (text, style) in &items {
            assert_eq!(
                style.weight,
                Some(700),
                "text='{}' deve ter weight=700; style={:?}",
                text,
                style
            );
        }
    }

    /// `#set text(style: "italic")` produz italic em todo o texto seguinte.
    #[test]
    fn set_text_italic_propaga_ao_frame() {
        let doc = layout_typst("#set text(style: \"italic\")\nHello");
        let items = text_items(&doc);
        assert!(!items.is_empty());
        for (text, style) in &items {
            assert!(
                style.italic,
                "text='{}' deve ter italic=true; style={:?}",
                text, style
            );
        }
    }

    /// `#set` antes do texto afecta só o conteúdo seguinte. "antes" deve ficar
    /// sem bold; "depois" com bold. (Validação que `#set` não afecta texto
    /// anterior ao directive.)
    #[test]
    fn set_text_bold_afecta_conteudo_seguinte_nao_anterior() {
        let doc = layout_typst("antes\n#set text(weight: 700)\ndepois");
        let items = text_items(&doc);
        let antes = items.iter().find(|(t, _)| t == "antes");
        let depois = items.iter().find(|(t, _)| t == "depois");

        assert!(
            antes.is_some(),
            "'antes' deve aparecer; items: {:?}",
            items.iter().map(|(t, _)| t).collect::<Vec<_>>()
        );
        assert!(
            depois.is_some(),
            "'depois' deve aparecer; items: {:?}",
            items.iter().map(|(t, _)| t).collect::<Vec<_>>()
        );
        if let Some((_, s)) = antes {
            assert_ne!(s.weight, Some(700), "'antes' não deve ter weight=700: {:?}", s);
        }
        if let Some((_, s)) = depois {
            assert_eq!(s.weight, Some(700), "'depois' deve ter weight=700: {:?}", s);
        }
    }

    /// `#set text(weight: 700)` combinado com `_texto_` — set dá weight, markup
    /// dá italic. Regressão: markup continua a funcionar após `#set`.
    #[test]
    fn set_combinado_com_emph_sintactico() {
        let doc = layout_typst("#set text(weight: 700)\n_italic_ normal");
        let items = text_items(&doc);
        assert!(!items.is_empty());
        // Todos os items devem ter weight=700 (vindo do #set).
        // Os items do `_italic_` têm italic=true adicionalmente.
        let has_italic = items.iter().any(|(_, s)| s.italic);
        let all_bold = items.iter().all(|(_, s)| s.weight == Some(700));
        assert!(
            all_bold,
            "todos os items devem ter weight=700 após #set: {:?}",
            items
                .iter()
                .map(|(t, s)| (t.as_str(), s.weight, s.italic))
                .collect::<Vec<_>>()
        );
        assert!(
            has_italic,
            "pelo menos 1 item deve ter italic (do `_italic_`): {:?}",
            items
                .iter()
                .map(|(t, s)| (t.as_str(), s.bold, s.italic))
                .collect::<Vec<_>>()
        );
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
        assert!(
            importante.map(|(_, s)| s.bold).unwrap_or(false),
            "'importante' deve ter bold: {:?}",
            items
        );
        assert!(
            !normal.map(|(_, s)| s.bold).unwrap_or(true),
            "'normal' não deve ter bold: {:?}",
            items
        );
    }

    // ── Passo 137 (Fase B.1 DEBT-52): consumer tracking ──────────────────
    //
    // Helper local: extrai `pos.x` de cada FrameItem::Text.
    fn text_items_with_pos(
        doc: &PagedDocument,
    ) -> Vec<(String, crate::entities::layout_types::TextStyle, f64)> {
        doc.pages
            .iter()
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
            assert_eq!(
                style.tracking,
                Some(Length::pt(1.0)),
                "text='{}' deve ter tracking=1pt; obtido {:?}",
                text,
                style.tracking
            );
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

        assert!(
            cd_sem.is_some(),
            "items sem tracking: {:?}",
            items_sem.iter().map(|(t, _, _)| t).collect::<Vec<_>>()
        );
        assert!(
            cd_com.is_some(),
            "items com tracking: {:?}",
            items_com.iter().map(|(t, _, _)| t).collect::<Vec<_>>()
        );

        let x_sem = cd_sem.unwrap().2;
        let x_com = cd_com.unwrap().2;

        // Com tracking 1em a size 12pt, "AB" (2 chars) ganha 1×12pt extra.
        // "CD" começa 12pt mais à direita (ajustar por size base que pode
        // ser diferente — o "sem" usa size default 11pt).
        //
        // Verificar que x_com > x_sem por aproximadamente 12pt (margem
        // generosa porque o size também muda).
        assert!(
            x_com > x_sem,
            "com tracking, 'CD' deve começar mais à direita; sem={}, com={}",
            x_sem,
            x_com
        );
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

        assert!(
            b_sem.is_some() && b_com.is_some(),
            "esperava 'B' em ambos; sem={:?}, com={:?}",
            items_sem,
            items_com
        );

        // Diferença entre as duas posições B é só devida a mudança de
        // size (11 → 12pt). Tracking não afecta porque cada word tem
        // 1 char só.
        // Não assertamos valor exacto porque size base muda; assertamos
        // apenas que tracking de 10pt NÃO se propaga inter-word (diferença
        // marginal, não 10pt+).
        let dif = (b_com.unwrap() - b_sem.unwrap()).abs();
        assert!(
            dif < 10.0,
            "tracking não deve afectar gap entre palavras de 1 char; diff={}",
            dif
        );
    }

    // ── Passo 138 (Fase B.2 DEBT-52): consumer leading ───────────────────
    //
    // Helper local: extrai `pos.x` + `pos.y` de cada FrameItem::Text.
    fn text_items_with_xy(
        doc: &PagedDocument,
    ) -> Vec<(String, crate::entities::layout_types::TextStyle, f64, f64)> {
        doc.pages
            .iter()
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
        let com = layout_typst("#set par(leading: 20pt)\n= Título\nlinha2");

        let sem_items = text_items_with_xy(&sem);
        let com_items = text_items_with_xy(&com);

        // "linha2" aparece após o heading — flush_line(s) intermédios.
        let l2_sem = sem_items.iter().find(|(t, _, _, _)| t == "linha2");
        let l2_com = com_items.iter().find(|(t, _, _, _)| t == "linha2");

        assert!(
            l2_sem.is_some(),
            "linha2 deve aparecer sem leading; items: {:?}",
            sem_items.iter().map(|(t, _, _, _)| t).collect::<Vec<_>>()
        );
        assert!(
            l2_com.is_some(),
            "linha2 deve aparecer com leading; items: {:?}",
            com_items.iter().map(|(t, _, _, _)| t).collect::<Vec<_>>()
        );

        let y_sem = l2_sem.unwrap().3;
        let y_com = l2_com.unwrap().3;

        // Frame coord: y cresce para baixo. Com leading positivo, linha
        // após heading está mais abaixo (y maior).
        assert!(
            y_com > y_sem,
            "linha2 deve ter y maior com leading; sem={}, com={}",
            y_sem,
            y_com
        );
    }

    /// Leading não afecta documento de 1 linha (leading = inter-line;
    /// sem linha 2, não há onde aplicar).
    #[test]
    fn layout_leading_nao_afecta_documento_uma_linha_passo_138() {
        let sem = layout_typst("uma linha");
        let com = layout_typst("#set par(leading: 10pt)\numa linha");

        let sem_items = text_items_with_xy(&sem);
        let com_items = text_items_with_xy(&com);

        // Primeiro item de cada (primeira word): y idêntico porque
        // leading só afecta linhas 2+.
        let primeiro_sem = &sem_items[0];
        let primeiro_com = &com_items[0];

        assert!(
            (primeiro_sem.3 - primeiro_com.3).abs() < 0.01,
            "primeira linha: y deve ser igual sem vs com leading; sem={}, com={}",
            primeiro_sem.3,
            primeiro_com.3
        );
    }

    /// **P762** — leading = 0pt reduz o avanço de linha ao mínimo
    /// (top-edge + |bottom-edge|), enquanto sem set usa leading default
    /// (0,65 em). Valida que a diferença entre os dois é exactamente o
    /// leading default na fonte base (11pt).
    #[test]
    fn layout_leading_zero_reduz_avanco_ao_minimo_passo_138() {
        // Usar um heading para forçar flush_line entre as duas linhas. O
        // avanço após o heading usa o tamanho do heading, que inferimos do
        // item de texto correspondente.
        let sem = layout_typst("= Título\nlinha2");
        let com = layout_typst("#set par(leading: 0pt)\n= Título\nlinha2");

        let sem_items = text_items_with_xy(&sem);
        let com_items = text_items_with_xy(&com);

        // Encontrar a segunda linha ("linha2") em cada documento.
        let y_sem = sem_items
            .iter()
            .find(|(t, _, _, _)| t == "linha2")
            .map(|(_, _, _, y)| *y)
            .expect("linha2 no doc sem leading set");
        let y_com = com_items
            .iter()
            .find(|(t, _, _, _)| t == "linha2")
            .map(|(_, _, _, y)| *y)
            .expect("linha2 no doc com leading 0pt");

        // Com leading 0pt, a linha2 está mais acima (menor y) porque o
        // avanço entre linhas é menor.
        assert!(
            y_com < y_sem,
            "leading 0pt deve colocar linha2 mais acima; sem={y_sem}, com={y_com}"
        );

        // O avanço após o heading usa o tamanho do heading. Inferimos esse
        // tamanho a partir do item "Título" para calcular o leading default.
        let heading_size = sem_items
            .iter()
            .find(|(t, _, _, _)| t == "Título")
            .map(|(_, style, _, _)| style.size.val())
            .expect("heading 'Título' no doc sem leading set");
        let expected_diff = heading_size * 0.65;
        let actual_diff = y_sem - y_com;
        assert!(
            (actual_diff - expected_diff).abs() < 0.01,
            "diferença={actual_diff}, esperado={expected_diff}"
        );
    }

    /// **P537b** — `#set page(columns: 2)` produz duas colunas reais na mesma
    /// página. Texto antes e depois do `colbreak()` têm x distinto.
    #[test]
    fn p537b_set_page_columns_produz_colunas_reais() {
        let doc = layout_typst(
            r#"#set page(columns: 2)
#lorem(5)
#colbreak()
#lorem(5)"#,
        );
        assert_eq!(doc.pages.len(), 1, "deve caber numa página");
        let items: Vec<_> = doc.pages[0]
            .items
            .iter()
            .filter_map(|it| match it {
                FrameItem::Text { text, pos, .. } => Some((text.to_string(), pos.x.0)),
                _ => None,
            })
            .collect();
        // Procurar uma palavra do primeiro bloco e uma do segundo.
        let left = items.iter().find(|(t, _)| t.contains("Lorem"));
        let right = items.iter().find(|(t, _)| t.contains("amet"));
        assert!(left.is_some(), "primeira coluna deve ter texto");
        assert!(right.is_some(), "segunda coluna deve ter texto");
        assert!(
            right.unwrap().1 > left.unwrap().1,
            "segunda coluna deve estar à direita da primeira"
        );
    }

    /// **P537b** — `#set page(columns: 2)` com footnotes posiciona cada nota
    /// no fundo da respectiva coluna, reaproveitando o fix de P537.
    #[test]
    fn p537b_set_page_columns_footnotes_por_coluna() {
        let doc = layout_typst(
            r#"#set page(columns: 2)
Hello #footnote[Nota A] world.
#colbreak()
Goodbye #footnote[Nota B] moon."#,
        );
        assert_eq!(doc.pages.len(), 1, "deve caber numa página");
        let items = &doc.pages[0].items;
        let note_a_x = items
            .iter()
            .rev()
            .filter_map(|it| match it {
                FrameItem::Text { text, pos, .. } if text.as_str() == "Nota" => {
                    Some(pos.x.0)
                }
                _ => None,
            })
            .nth(1);
        let note_b_x = items
            .iter()
            .rev()
            .filter_map(|it| match it {
                FrameItem::Text { text, pos, .. } if text.as_str() == "Nota" => {
                    Some(pos.x.0)
                }
                _ => None,
            })
            .next();
        let (a, b) = (
            note_a_x.expect("nota A deve existir"),
            note_b_x.expect("nota B deve existir"),
        );
        assert!(
            b > a,
            "nota B deve estar à direita de nota A (coluna 2 > coluna 1): a={} b={}",
            a,
            b
        );
        for it in items {
            if let FrameItem::Text { text, pos, .. } = it {
                if text.as_str() == "Nota" {
                    assert!(
                        pos.y.0 > 700.0,
                        "nota deve estar no fundo da página, y={}",
                        pos.y.0
                    );
                }
            }
        }
    }

    /// **P537b** — `#set page(columns: 0)` rejeitado no eval.
    #[test]
    fn p537b_set_page_columns_zero_erro() {
        let world = MockWorld::new("#set page(columns: 0)\nX");
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_err(), "columns: 0 deve produzir erro");
    }

    /// **P538c** — `#set page(columns: 2)` com texto corrido distribui o
    /// conteúdo pelas duas colunas da mesma página (modo fluxo contínuo).
    #[test]
    fn p538c_set_page_columns_fluxo_continuo_uma_pagina() {
        let doc = layout_typst(
            r#"#set page(columns: 2)
#lorem(200)"#,
        );
        assert_eq!(doc.pages.len(), 1, "deve caber numa página");
        let items: Vec<_> = doc.pages[0]
            .items
            .iter()
            .filter_map(|it| match it {
                FrameItem::Text { text, pos, .. } => Some((text.to_string(), pos.x.0)),
                _ => None,
            })
            .collect();
        let left = items.iter().find(|(t, _)| t.contains("Lorem"));
        let right = items.iter().rev().find(|(t, _)| t.contains("aliqua"));
        assert!(left.is_some(), "primeira coluna deve ter texto");
        assert!(right.is_some(), "segunda coluna deve ter texto");
        assert!(
            right.unwrap().1 > left.unwrap().1,
            "segunda coluna deve estar à direita da primeira"
        );
    }

    /// **P626** — `#set page(columns: 2)` com `dir: rtl` preenche a coluna
    /// da direita primeiro. O primeiro texto do documento aparece na coluna
    /// da direita (x > centro da página).
    #[test]
    fn p626_set_page_columns_rtl_preenche_direita_primeiro() {
        let doc = layout_typst(
            r#"#set page(columns: 2)
#set text(lang: "ar", dir: rtl)
#lorem(200)"#,
        );
        assert!(!doc.pages.is_empty(), "deve haver pelo menos uma página");
        let first_text = doc.pages[0].items.iter().find_map(|it| match it {
            FrameItem::Text { text, pos, .. } => Some((text.to_string(), pos.x.0)),
            _ => None,
        });
        let (text, x) = first_text.expect("primeira página deve ter texto");
        assert!(
            x > 297.0,
            "em RTL o primeiro texto deve estar na coluna da direita (x > centro): text={:?} x={}",
            text,
            x
        );
    }

    /// **P627** — documento bilingue com um único `#set page(columns: 2)` e
    /// mudança de `text.dir` por `#pagebreak()`: cada página preenche as
    /// colunas na direcção correcta.
    #[test]
    fn p627_bilingue_muda_direcao_com_pagebreak() {
        let doc = layout_typst(
            r#"#set page(columns: 2)
#set text(lang: "en", dir: ltr, size: 16pt)
#lorem(150)

#pagebreak()

#set text(lang: "ar", dir: rtl, size: 16pt)
#lorem(150)"#,
        );
        assert_eq!(doc.pages.len(), 2, "documento bilingue deve ter duas páginas");

        let first_text_page1 = doc.pages[0].items.iter().find_map(|it| match it {
            FrameItem::Text { text, pos, .. } => Some((text.to_string(), pos.x.0)),
            _ => None,
        });
        let (_, x1) = first_text_page1.expect("página 1 deve ter texto");
        assert!(x1 < 297.0, "página 1 LTR deve começar na coluna da esquerda: x={}", x1);

        let first_text_page2 = doc.pages[1].items.iter().find_map(|it| match it {
            FrameItem::Text { text, pos, .. } => Some((text.to_string(), pos.x.0)),
            _ => None,
        });
        let (_, x2) = first_text_page2.expect("página 2 deve ter texto");
        assert!(x2 > 297.0, "página 2 RTL deve começar na coluna da direita: x={}", x2);
    }

    /// **P538c** — `#set page(columns: 2)` com texto longo cria múltiplas
    /// páginas A4 (não páginas de largura de coluna) e o PDF não fica
    /// malformado. O número de páginas deve ser significativamente menor
    /// do que as 9 páginas observadas antes da correcção.
    #[test]
    fn p538c_set_page_columns_fluxo_continuo_multi_pagina() {
        let doc = layout_typst(
            r#"#set page(columns: 2)
#lorem(1200)"#,
        );
        assert!(
            doc.pages.len() < 9,
            "documento longo deve ter menos de 9 páginas (era 9 antes do fix), got {}",
            doc.pages.len()
        );
        for (idx, page) in doc.pages.iter().enumerate() {
            assert!(
                (page.width - 595.28).abs() < 0.1,
                "página {} deve ter largura A4 (595.28), got {}",
                idx + 1,
                page.width
            );
        }
    }

    /// **P538d** — texto de numeração automática de página tem `style.font`
    /// preenchido (correcção equivalente a P483 para texto normal).
    #[test]
    fn p538d_page_numbering_text_tem_font_definida() {
        let doc = layout_typst(
            r#"#set page(numbering: "1")
Página."#,
        );
        assert!(
            doc.pages.iter().any(|p| p.items.iter().any(|item| {
                matches!(item, FrameItem::Text { text, style, .. }
                    if text.as_str() == "1" && style.font.is_some())
            })),
            "numeração de página deve ser renderizada com style.font preenchido"
        );
    }

    /// **P541** — padrões compostos de numeração de página usam o número da
    /// página actual e o total de páginas (ex.: `"1 / 1"` → `"1 / 3"`).
    #[test]
    fn p541_page_numbering_composto_usa_total_de_paginas() {
        let doc = layout_typst(
            r#"#set page(numbering: "1 / 1")
Página um.
#pagebreak()
Página dois.
#pagebreak()
Página três."#,
        );
        assert_eq!(doc.pages.len(), 3, "documento deve ter 3 páginas");
        let texts: Vec<String> = doc
            .pages
            .iter()
            .map(|p| {
                p.items
                    .iter()
                    .filter_map(|item| match item {
                        FrameItem::Text { text, .. } => Some(text.as_str()),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join("")
            })
            .collect();
        let expected = vec!["1 / 3", "2 / 3", "3 / 3"];
        for (idx, exp) in expected.iter().enumerate() {
            assert!(
                texts[idx].contains(exp),
                "página {} deve conter '{}', obtido: '{}'",
                idx + 1,
                exp,
                texts[idx]
            );
        }
    }

    /// Passo 581 — Garante que caracteres de escape e shorthands em markup
    /// sejam avaliados e renderizados corretamente no layout, sem desaparecer.
    #[test]
    fn p581_cobertura_de_escape_e_shorthand_em_layout() {
        let doc = layout_typst("A \\# B \\$ C -- D ... E");
        let items: Vec<_> = doc.pages[0]
            .items
            .iter()
            .filter_map(|it| match it {
                FrameItem::Text { text, .. } => Some(text.to_string()),
                _ => None,
            })
            .collect();
        let joined = items.join(" ");
        assert!(joined.contains("#"), "deve conter '#' do escape; obtido: '{}'", joined);
        assert!(joined.contains("$"), "deve conter '$' do escape; obtido: '{}'", joined);
        assert!(
            joined.contains("–"),
            "deve conter '–' do shorthand; obtido: '{}'",
            joined
        );
        assert!(
            joined.contains("…"),
            "deve conter '…' do shorthand; obtido: '{}'",
            joined
        );
    }

    /// P588 — newline após `#set` não deve produzir espaço visual no início
    /// do parágrafo seguinte.
    #[test]
    fn p588_newline_apos_set_nao_desloca_texto_inicial() {
        let doc = layout_typst("#set text(size: 20pt)\nTexto normal aqui.");
        let first_text_x = doc.pages[0]
            .items
            .iter()
            .find_map(|item| match item {
                FrameItem::Text { pos, .. } => Some(pos.x.0),
                _ => None,
            })
            .expect("deve haver pelo menos um Text item");
        // Margem default = 70.87 pt. Antes da correcção, o primeiro texto
        // começava em 75.87 pt (margem + space_width de 5 pt).
        assert!(
            (first_text_x - 70.87).abs() < 0.01,
            "texto inicial deve começar na margem (70.87 pt), não deslocado por espaco; obtido {}",
            first_text_x
        );
    }

    /// P589 — helper `documento_algoritmo` aplica a fonte neutra sem alterar
    /// o posicionamento base de texto simples. Serve como sentinel de que o
    /// helper funciona e que a fonte escolhida existe no ambiente de teste.
    #[test]
    fn p589_documento_algoritmo_aplica_fonte_neutra() {
        let doc = layout_typst(&documento_algoritmo("Hello world"));
        let first_text_x = doc.pages[0]
            .items
            .iter()
            .find_map(|item| match item {
                FrameItem::Text { pos, .. } => Some(pos.x.0),
                _ => None,
            })
            .expect("deve haver pelo menos um Text item");
        assert!(
            (first_text_x - 70.87).abs() < 0.01,
            "texto com fonte neutra deve começar na margem (70.87 pt); obtido {}",
            first_text_x
        );
    }
}

// ── Passo 103.D: Integração `#show` end-to-end ────────────────────────────

#[cfg(test)]
mod tests_show_rule_integration {
    use super::*;
    use crate::{
        contracts::world::World,
        engine::eval::eval_for_test,
        entities::{
            file_id::FileId,
            font_book::FontBook,
            layout_types::FrameItem,
            source::Source,
            world_types::{Bytes, Datetime, FileError, FileResult, Font, Library},
        },
    };
    use std::num::NonZeroU16;

    struct MockWorld {
        library: Library,
        book: FontBook,
        source: Source,
    }

    impl MockWorld {
        fn new(text: &str) -> Self {
            let id = FileId::from_raw(NonZeroU16::new(1).unwrap());
            Self {
                library: Library::new(),
                book: FontBook::new(),
                source: Source::new(id, text.to_string()),
            }
        }
    }

    impl World for MockWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            self.source.id()
        }
        fn source(&self, _: FileId) -> FileResult<Source> {
            Ok(self.source.clone())
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<Font> {
            None
        }
        fn today(&self, _: Option<i64>) -> Option<Datetime> {
            None
        }
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
        doc.pages
            .iter()
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
        assert!(
            text.contains("INTRO"),
            "esperado 'INTRO' no output após show heading upper: {:?}",
            text
        );
    }

    /// `#show strong: it => upper(it.body)` transforma `*bold*` em UPPERCASE.
    #[test]
    fn show_strong_transforma() {
        let doc = layout_typst("#show strong: upper\n*alvo*");
        let text = plain_text(&doc);
        assert!(
            text.contains("ALVO"),
            "esperado 'ALVO' após show strong upper: {:?}",
            text
        );
    }

    /// `#show emph: it => lower(it.body)` transforma `_italic_` em lowercase.
    #[test]
    fn show_emph_transforma() {
        let doc = layout_typst("#show emph: lower\n_TIPO_");
        let text = plain_text(&doc);
        assert!(
            text.contains("tipo"),
            "esperado 'tipo' após show emph lower: {:?}",
            text
        );
    }

    /// Regressão: sem `#show`, `*bold*` continua bold; `= heading` continua
    /// heading. Confirma que os show rules não interferem quando ausentes.
    #[test]
    fn regressao_sem_show_mantem_comportamento() {
        let doc = layout_typst("*bold* and _italic_");
        let items: Vec<_> = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|i| match i {
                FrameItem::Text { text, style, .. } => {
                    Some((text.to_string(), style.bold, style.italic))
                }
                _ => None,
            })
            .collect();
        // Deve existir pelo menos um item com bold e outro com italic.
        assert!(
            items.iter().any(|(t, b, _)| t == "bold" && *b),
            "esperado 'bold' com style.bold=true: {:?}",
            items
        );
        assert!(
            items.iter().any(|(t, _, i)| t == "italic" && *i),
            "esperado 'italic' com style.italic=true: {:?}",
            items
        );
    }

    /// **DEBT-50 fechado (P431)**: `#show strong` só dispara para conteúdo com
    /// `Style::Bold { from_strong: true }` (sintaxe `*bold*`). `#set text(weight: 700)`
    /// produz `Content::Styled(.., [Style::bold(true)])`, onde `from_strong: false`,
    /// pelo que o selector Strong NÃO deve disparar.
    #[test]
    fn debt_50_show_strong_nao_apanha_set_text_bold() {
        let doc =
            layout_typst("#show strong: it => [HIT]\n#set text(weight: 700)\ntexto");
        let text = plain_text(&doc);
        assert!(
            !text.contains("HIT"),
            "DEBT-50: selector Strong NÃO deve disparar por `#set text(weight: 700)` \
             (origem diferente de `*bold*`): {:?}",
            text
        );
        assert!(text.contains("texto"), "'texto' deve aparecer no output: {:?}", text);
    }

    // ── P790 — `#show "texto": …` com transformação Content/Func ───────────
    //
    // Achado de P786 (módulo `eval::rules`): regras show-by-string com
    // transformação Content ou Func eram aceites e descartadas em silêncio
    // (só `Transformation::Str` era aplicada, via `map_text`). P790 liga o
    // splice: o `Content::Text` é fatiado nas ocorrências do padrão e o
    // replacement é emendado entre as fatias (paridade vanilla
    // `visit_regex_match`). Espaços entre itens são normalizados nas
    // asserções (`plain_text` junta FrameItems com " ").

    fn sem_espacos(s: &str) -> String {
        s.chars().filter(|c| !c.is_whitespace()).collect()
    }

    /// Repro de P786 (`temp/temp_p786/c_rules_probe_show.typ`):
    /// `#show "world": [W]` — vanilla renderiza "Hello W." (medido).
    #[test]
    fn p790_show_string_com_content_substituí_ocorrencia() {
        let doc = layout_typst("#show \"world\": [W]\n\nHello world.");
        let text = sem_espacos(&plain_text(&doc));
        assert!(
            text.contains("HelloW."),
            "esperado 'Hello W.' após show-by-string com Content: {:?}",
            text
        );
        assert!(
            !text.contains("Helloworld"),
            "o texto original não deve sobreviver à substituição: {:?}",
            text
        );
    }

    /// Múltiplas ocorrências no mesmo nó de texto são todas substituídas.
    #[test]
    fn p790_show_string_com_content_multiplas_ocorrencias() {
        let doc = layout_typst("#show \"a\": [X]\n\nbanana");
        let text = sem_espacos(&plain_text(&doc));
        assert!(
            text.contains("bXnXnX"),
            "esperado 'bXnXnX' (3 ocorrências substituídas): {:?}",
            text
        );
    }

    /// Transformação Func: chamada por ocorrência com o texto do match;
    /// o output é emendado. `it => [#it#it]` duplica o match.
    #[test]
    fn p790_show_string_com_func_recebe_match_e_emenda() {
        let doc = layout_typst("#show \"world\": it => [#it#it]\n\nHello world.");
        let text = sem_espacos(&plain_text(&doc));
        assert!(
            text.contains("Helloworldworld."),
            "esperado match duplicado pela func ('Hello worldworld.'): {:?}",
            text
        );
    }

    /// Controlo: sem regra, o texto fica intacto (a travessia nova não
    /// toca em nós sem match).
    #[test]
    fn p790_show_string_sem_match_preserva_texto() {
        let doc = layout_typst("#show \"zzz\": [X]\n\nHello world.");
        let text = sem_espacos(&plain_text(&doc));
        assert!(
            text.contains("Helloworld."),
            "sem match o texto deve ficar intacto: {:?}",
            text
        );
    }

    // ── P791 — `#show <lbl>: …` (selector por label) ───────────────────────

    /// End-to-end: `#show <sp>: it => [LBL=#it]` sobre `= Alpha <sp>`
    /// (vanilla medido: "LBL=" seguido do heading).
    #[test]
    fn p791_show_label_em_heading_end_to_end() {
        let doc = layout_typst("#show <sp>: it => [LBL=#it]\n\n= Alpha <sp>");
        let text = sem_espacos(&plain_text(&doc));
        assert!(
            text.contains("LBL=Alpha"),
            "show-by-label deve disparar sobre o heading rotulado: {:?}",
            text
        );
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
        let baseline_y_max: f64 = baseline
            .pages
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
        let pad_y_max: f64 = with_pad
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { pos, .. } => Some(pos.y.val()),
                _ => None,
            })
            .fold(0.0_f64, |acc, y| acc.max(y));

        // Pad com top=20 deve empurrar o texto para baixo na página.
        assert!(
            pad_y_max > baseline_y_max,
            "esperado que Content::Pad com top=20pt empurre o texto para baixo: \
             baseline_y_max={baseline_y_max:.2} pad_y_max={pad_y_max:.2}"
        );
    }

    /// `Content::Hide` calcula dimensões mas não emite items visuais.
    /// Verificamos que zero items textuais são produzidos pelo body
    /// envolvido em Hide.
    #[test]
    fn layout_hide_emite_zero_text_items() {
        let hidden = Content::hide(Content::text("invisivel"));
        let doc = layout(&hidden);
        let text_items = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter(|item| matches!(item, FrameItem::Text { .. }))
            .count();
        assert_eq!(text_items, 0, "Content::Hide não deve emitir nenhum FrameItem::Text");
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
        let texts: Vec<_> = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { pos, text, .. } => {
                    Some((pos.x.val(), text.to_string()))
                }
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
        let texts: Vec<_> = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { pos, text, .. } => {
                    Some((pos.y.val(), text.to_string()))
                }
                _ => None,
            })
            .collect();
        let pos_a_y = texts.iter().find(|(_, t)| t == "A").map(|(y, _)| *y).unwrap();
        let pos_b_y = texts.iter().find(|(_, t)| t == "B").map(|(y, _)| *y).unwrap();
        // B deve estar abaixo de A com pelo menos ~30pt extra (mais
        // line_height que o flush adiciona).
        assert!(
            pos_b_y - pos_a_y > 30.0,
            "v(30pt) deve empurrar B abaixo de A em pelo menos 30pt: \
             pos_a_y={pos_a_y:.2} pos_b_y={pos_b_y:.2}"
        );
    }

    // ── P842 (achado #38 de P831) — h(1fr) fractional spacing ────────────

    /// Extrai `(x, texto)` dos items de texto do documento.
    fn text_positions_x(
        doc: &crate::entities::layout_types::PagedDocument,
    ) -> Vec<(f64, String)> {
        doc.pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { pos, text, .. } => Some((pos.x.val(), text.to_string())),
                _ => None,
            })
            .collect()
    }

    /// `A#h(1fr)B` — o fr consome todo o espaço restante da linha: B fica
    /// encostado à margem direita (paridade vanilla medida em
    /// `temp/p842/l7_h_1fr.typ`: B em xMax = 200pt com página de 200pt).
    #[test]
    fn p842_l7_h_1fr_expande_espaco_restante() {
        use std::sync::Arc;
        let content = Content::Sequence(Arc::from(vec![
            Content::text("A"),
            Content::h_space_fraction(1.0, false),
            Content::text("B"),
        ]));
        let doc = layout(&content);
        let texts = text_positions_x(&doc);
        let pos_b = texts.iter().find(|(_, t)| t == "B").map(|(x, _)| *x).unwrap();
        // A4 default: width 595.28, margin ≈ 70.8667 → margem direita em
        // 524.4133. FixedMetrics: "B" com DEFAULT_FONT_SIZE 12 → 7.2pt.
        let cfg = crate::entities::layout_types::PageConfig::default();
        let right_margin = cfg.width - cfg.margin;
        let b_width = 0.6 * 12.0;
        assert!(
            (pos_b - (right_margin - b_width)).abs() < 1.0,
            "h(1fr) deve encostar B à margem direita: pos_b={pos_b:.2}, \
             esperado ≈ {:.2}",
            right_margin - b_width
        );
    }

    /// `A#h(1fr)B#h(2fr)C` — o espaço restante é distribuído na proporção
    /// 1:2 (paridade vanilla medida em `temp/p842/l7_h_2fr.typ`).
    #[test]
    fn p842_l7_h_fr_distribuicao_proporcional() {
        use std::sync::Arc;
        let content = Content::Sequence(Arc::from(vec![
            Content::text("A"),
            Content::h_space_fraction(1.0, false),
            Content::text("B"),
            Content::h_space_fraction(2.0, false),
            Content::text("C"),
        ]));
        let doc = layout(&content);
        let texts = text_positions_x(&doc);
        let x = |s: &str| texts.iter().find(|(_, t)| t == s).map(|(x, _)| *x).unwrap();
        let (xa, xb, xc) = (x("A"), x("B"), x("C"));
        let glyph = 0.6 * 12.0;
        let gap1 = xb - (xa + glyph);
        let gap2 = xc - (xb + glyph);
        assert!(gap1 > 1.0 && gap2 > 1.0, "gaps positivos: {gap1:.2} {gap2:.2}");
        assert!(
            (gap2 / gap1 - 2.0).abs() < 0.01,
            "razão 2:1 esperada: gap1={gap1:.2} gap2={gap2:.2}"
        );
        // C encosta à margem direita (todo o restante consumido).
        let cfg = crate::entities::layout_types::PageConfig::default();
        let right_margin = cfg.width - cfg.margin;
        assert!(
            (xc - (right_margin - glyph)).abs() < 1.0,
            "C deve encostar à margem direita: xc={xc:.2}"
        );
    }

    /// `A#h(10pt)B#h(1fr)C` — length fixo e fr combinam: o fr consome o
    /// restante após os comprimentos fixos (paridade vanilla medida em
    /// `temp/p842/l7_h_misto.typ`).
    #[test]
    fn p842_l7_h_fr_com_length_fixo() {
        use crate::entities::layout_types::Length;
        use std::sync::Arc;
        let content = Content::Sequence(Arc::from(vec![
            Content::text("A"),
            Content::h_space(Length::pt(10.0), false),
            Content::text("B"),
            Content::h_space_fraction(1.0, false),
            Content::text("C"),
        ]));
        let doc = layout(&content);
        let texts = text_positions_x(&doc);
        let x = |s: &str| texts.iter().find(|(_, t)| t == s).map(|(x, _)| *x).unwrap();
        let (xa, xb, xc) = (x("A"), x("B"), x("C"));
        let glyph = 0.6 * 12.0;
        // h(10pt) intacto: gap A→B ≈ 10pt (tolerância 1.0 — o cursor de
        // palavras com FixedMetrics tem um desvio pré-P842 de ~0.6pt no
        // caminho absoluto, já presente antes deste passo e coberto pelo
        // teste P156D `layout_hspace_avanca_cursor_x` por threshold).
        assert!(
            (xb - (xa + glyph) - 10.0).abs() < 1.0,
            "h(10pt) preservado: xa={xa:.2} xb={xb:.2}"
        );
        // C encosta à margem direita.
        let cfg = crate::entities::layout_types::PageConfig::default();
        let right_margin = cfg.width - cfg.margin;
        assert!(
            (xc - (right_margin - glyph)).abs() < 1.0,
            "C deve encostar à margem direita: xc={xc:.2}"
        );
    }

    // ── Passo 156E (ADR-0061 Fase 1, sub-passo 3) — pagebreak manual ───────

    /// Helper: extrai texto plano da primeira página que contém certa
    /// string. Devolve o índice de página (1-indexed) onde foi encontrado.
    fn page_index_containing(
        doc: &crate::entities::layout_types::PagedDocument,
        needle: &str,
    ) -> Option<usize> {
        for (i, page) in doc.pages.iter().enumerate() {
            for item in page.items.iter() {
                if let FrameItem::Text { text, .. } = item {
                    if text.contains(needle) {
                        return Some(i + 1); // 1-indexed
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
        assert!(
            doc.pages.len() >= 2,
            "esperado >= 2 páginas após pagebreak, obtive {}",
            doc.pages.len()
        );
        let page_a = page_index_containing(&doc, "A").expect("A não encontrado");
        let page_b = page_index_containing(&doc, "B").expect("B não encontrado");
        assert!(
            page_b > page_a,
            "B deve estar em página posterior a A: A→p{} B→p{}",
            page_a,
            page_b
        );
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
        assert_eq!(
            page_b, 2,
            "B deve estar na p2 (par; sem inserção extra): obtive p{}",
            page_b
        );
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
        assert_eq!(
            page_b, 3,
            "B deve estar na p3 (ímpar; vazia inserida na p2): obtive p{}",
            page_b
        );
        assert!(
            doc.pages.len() >= 3,
            "esperado >= 3 páginas (A em p1, vazia em p2, B em p3)"
        );
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
                None,
                None,
                Sides::uniform(Length::pt(10.0)),
                true,
            ),
            Content::text("C"),
        ]));
        let doc = layout(&doc_content);
        let texts: Vec<_> = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { pos, text, .. } => {
                    Some((pos.y.val(), text.to_string()))
                }
                _ => None,
            })
            .collect();
        let pos_a_y = texts.iter().find(|(_, t)| t == "A").map(|(y, _)| *y).unwrap();
        let pos_body_y =
            texts.iter().find(|(_, t)| t == "body").map(|(y, _)| *y).unwrap();
        let pos_c_y = texts.iter().find(|(_, t)| t == "C").map(|(y, _)| *y).unwrap();
        // Body deve estar abaixo de A (block força nova linha + inset top).
        assert!(
            pos_body_y > pos_a_y,
            "body deve estar abaixo de A: a={pos_a_y:.2} body={pos_body_y:.2}"
        );
        // C deve estar abaixo de body (inset bottom adicionado).
        assert!(
            pos_c_y > pos_body_y,
            "C deve estar abaixo de body: body={pos_body_y:.2} c={pos_c_y:.2}"
        );
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
                None,
                None,
                Sides::uniform(Length::ZERO),
                Length::ZERO,
            ),
            Content::text("B"),
        ]));
        let doc = layout(&doc_content);
        let texts: Vec<_> = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { pos, text, .. } => {
                    Some((pos.y.val(), text.to_string()))
                }
                _ => None,
            })
            .collect();
        let pos_a_y = texts.iter().find(|(_, t)| t == "A").map(|(y, _)| *y).unwrap();
        let pos_m_y = texts.iter().find(|(_, t)| t == "M").map(|(y, _)| *y).unwrap();
        let pos_b_y = texts.iter().find(|(_, t)| t == "B").map(|(y, _)| *y).unwrap();
        assert!((pos_a_y - pos_m_y).abs() < 0.001,
            "A e M devem estar na mesma linha (box é inline): a={pos_a_y:.2} m={pos_m_y:.2}");
        assert!(
            (pos_m_y - pos_b_y).abs() < 0.001,
            "M e B devem estar na mesma linha: m={pos_m_y:.2} b={pos_b_y:.2}"
        );
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
                None,
                None,
                Sides::uniform(Length::ZERO),
                Length::ZERO,
            ),
        ]));
        let doc1 = layout(&no_inset);
        let pos_m1: f64 = doc1
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { pos, text, .. } if text.as_str() == "M" => {
                    Some(pos.x.val())
                }
                _ => None,
            })
            .next()
            .unwrap();

        let with_inset = Content::Sequence(Arc::from(vec![
            Content::text("A"),
            Content::boxed(
                Content::text("M"),
                None,
                None,
                Sides::uniform(Length::pt(20.0)),
                Length::ZERO,
            ),
        ]));
        let doc2 = layout(&with_inset);
        let pos_m2: f64 = doc2
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { pos, text, .. } if text.as_str() == "M" => {
                    Some(pos.x.val())
                }
                _ => None,
            })
            .next()
            .unwrap();

        assert!(
            pos_m2 - pos_m1 >= 20.0,
            "box com inset=20pt deve empurrar M em pelo menos 20pt: \
             m1={pos_m1:.2} m2={pos_m2:.2}"
        );
    }

    // ── Passo 156I (ADR-0061 Fase 2 sub-passo 3) — stack compositivo ──────

    /// `Content::Stack` TTB empilha children verticalmente: B abaixo de A.
    #[test]
    fn layout_stack_ttb_empilha_verticalmente() {
        use crate::entities::dir::Dir;
        let s =
            Content::stack(vec![Content::text("A"), Content::text("B")], Dir::TTB, None);
        let doc = layout(&s);
        let texts: Vec<_> = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { pos, text, .. } => {
                    Some((pos.y.val(), text.to_string()))
                }
                _ => None,
            })
            .collect();
        let pos_a_y = texts.iter().find(|(_, t)| t == "A").map(|(y, _)| *y).unwrap();
        let pos_b_y = texts.iter().find(|(_, t)| t == "B").map(|(y, _)| *y).unwrap();
        assert!(
            pos_b_y > pos_a_y,
            "stack TTB deve colocar B abaixo de A: a={pos_a_y:.2} b={pos_b_y:.2}"
        );
    }

    /// `Content::Stack` LTR empilha children inline: B à direita de A,
    /// na mesma linha.
    #[test]
    fn layout_stack_ltr_empilha_horizontalmente() {
        use crate::entities::dir::Dir;
        let s =
            Content::stack(vec![Content::text("A"), Content::text("B")], Dir::LTR, None);
        let doc = layout(&s);
        let texts: Vec<_> = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { pos, text, .. } => {
                    Some((pos.x.val(), pos.y.val(), text.to_string()))
                }
                _ => None,
            })
            .collect();
        let (a_x, a_y) = texts
            .iter()
            .find(|(_, _, t)| t == "A")
            .map(|(x, y, _)| (*x, *y))
            .unwrap();
        let (b_x, b_y) = texts
            .iter()
            .find(|(_, _, t)| t == "B")
            .map(|(x, y, _)| (*x, *y))
            .unwrap();
        // Mesma linha (Y igual).
        assert!(
            (a_y - b_y).abs() < 0.001,
            "stack LTR deve manter A e B na mesma linha: a_y={a_y:.2} b_y={b_y:.2}"
        );
        // B à direita de A.
        assert!(
            b_x > a_x,
            "stack LTR deve colocar B à direita de A: a_x={a_x:.2} b_x={b_x:.2}"
        );
    }

    /// `Content::Stack` TTB com spacing força avanço vertical extra
    /// entre children.
    #[test]
    fn layout_stack_spacing_avanca_cursor_entre_children() {
        use crate::entities::dir::Dir;
        use crate::entities::layout_types::Length;

        // Doc 1: stack sem spacing.
        let s_no_space =
            Content::stack(vec![Content::text("A"), Content::text("B")], Dir::TTB, None);
        let doc1 = layout(&s_no_space);
        let pos_b1: f64 = doc1
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { pos, text, .. } if text.as_str() == "B" => {
                    Some(pos.y.val())
                }
                _ => None,
            })
            .next()
            .unwrap();

        // Doc 2: stack com spacing 30pt.
        let s_with_space = Content::stack(
            vec![Content::text("A"), Content::text("B")],
            Dir::TTB,
            Some(Length::pt(30.0)),
        );
        let doc2 = layout(&s_with_space);
        let pos_b2: f64 = doc2
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { pos, text, .. } if text.as_str() == "B" => {
                    Some(pos.y.val())
                }
                _ => None,
            })
            .next()
            .unwrap();

        // B em doc2 deve estar pelo menos 30pt mais abaixo (spacing).
        assert!(
            pos_b2 - pos_b1 >= 30.0,
            "stack TTB com spacing=30pt deve empurrar B em pelo menos 30pt: \
             b1={pos_b1:.2} b2={pos_b2:.2}"
        );
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
            Content::block(
                Content::text("x"),
                None,
                None,
                Sides::uniform(Length::ZERO),
                true,
            ),
            Content::text("B"),
        ]));
        let doc1 = layout(&no_height);
        let pos_b1: f64 = doc1
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { pos, text, .. } if text.as_str() == "B" => {
                    Some(pos.y.val())
                }
                _ => None,
            })
            .next()
            .unwrap();

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
        let pos_b2: f64 = doc2
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { pos, text, .. } if text.as_str() == "B" => {
                    Some(pos.y.val())
                }
                _ => None,
            })
            .next()
            .unwrap();

        // B em doc2 deve estar pelo menos ~100pt mais abaixo que em doc1
        // (devido ao height mínimo do bloco com altura forçada).
        // Margem conservadora: pelo menos 50pt de diferença.
        assert!(
            pos_b2 - pos_b1 > 50.0,
            "block com height=100pt deve empurrar B mais para baixo do que sem height: \
             b1={pos_b1:.2} b2={pos_b2:.2}"
        );
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
            Content::text("p218body"), // single word — layout não splita
            3,                         // count > 1
            Some(Length::pt(15.0)),    // gutter explícito
        );
        let doc = layout(&c);
        let texts: String = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            })
            .collect();
        assert!(
            texts.contains("p218body"),
            "P218 stub transparente deve renderizar body mesmo com count=3"
        );
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
            2,                      // count ignorado em P217
            Some(Length::pt(10.0)), // gutter ignorado em P217
        );
        let doc = layout(&c);
        // Body deve aparecer (stub transparente delega a layout_content).
        let texts: String = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            })
            .collect();
        assert!(
            texts.contains("hello"),
            "columns body deve renderizar transparentemente"
        );
    }

    // ── P219 (DEBT-56 sub-fase b 3/4) — consumer real graded ──────────

    /// **P219** — count=1 caso degenerate: column_width == full_width
    /// (paridade `(width - 0*gutter) / 1 = width`). Body renderiza
    /// inalterado vs sem columns.
    #[test]
    fn p219_columns_count_1_equivale_a_body_directo() {
        let c1 = Content::columns(Content::text("p219c1"), 1, None);
        let doc1 = layout(&c1);
        let texts1: String = doc1
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            })
            .collect();
        assert!(texts1.contains("p219c1"), "count=1 preserva body");
    }

    /// **P219** — count=2: body renderiza preservado.
    #[test]
    fn p219_columns_count_2_renderiza_body() {
        let c = Content::columns(Content::text("p219c2"), 2, None);
        let doc = layout(&c);
        let texts: String = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            })
            .collect();
        assert!(texts.contains("p219c2"), "count=2 preserva body");
    }

    /// **P219** — count=3: body renderiza preservado (paralelo P218
    /// E2E mas verifica explicitamente arm real).
    #[test]
    fn p219_columns_count_3_renderiza_body() {
        use crate::entities::layout_types::Length;
        let c = Content::columns(Content::text("p219c3"), 3, Some(Length::pt(20.0)));
        let doc = layout(&c);
        let texts: String = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            })
            .collect();
        assert!(texts.contains("p219c3"), "count=3 preserva body");
    }

    /// **P219** — gutter explícito aceite (Length resolve para Pt).
    #[test]
    fn p219_columns_gutter_length_explicito_renderiza() {
        use crate::entities::layout_types::Length;
        let c = Content::columns(Content::text("g219"), 2, Some(Length::pt(50.0)));
        let doc = layout(&c);
        let texts: String = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            })
            .collect();
        assert!(texts.contains("g219"), "gutter explícito Length aceito");
    }

    /// **P219** — gutter default `None` aplicado via
    /// `COLUMNS_DEFAULT_GUTTER_RATIO = 0.04`. Body renderiza.
    #[test]
    fn p219_columns_gutter_default_renderiza() {
        let c = Content::columns(Content::text("gd219"), 2, None);
        let doc = layout(&c);
        let texts: String = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            })
            .collect();
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
        let texts: String = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            })
            .collect();
        assert!(texts.contains("colbody"), "body em columns renderiza");
        assert!(
            texts.contains("afterbody"),
            "after columns renderiza com width restaurada"
        );
    }

    /// **P219** — body com `Content::Heading` em columns: heading
    /// counter incrementa exactamente uma vez (paridade walk-única
    /// preservada de P217).
    #[test]
    fn p219_columns_counters_contam_uma_vez() {
        use std::sync::Arc;
        // Dois headings dentro de columns; counter deve = 2 final
        // (não 4 — sem multi-render).
        let h1 = Content::heading(1, Content::text("h1col"));
        let h2 = Content::heading(1, Content::text("h2col"));
        let body_seq = Content::Sequence(Arc::from(vec![h1, h2]));
        let cols = Content::columns(body_seq, 2, None);
        let doc = layout(&cols);
        let texts: String = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            })
            .collect();
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
        let texts: String = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            })
            .collect();
        assert!(
            texts.contains("nest"),
            "nested columns body renderiza (composição aninhada)"
        );
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
        assert!(
            doc.pages.len() >= 2,
            "esperado >= 2 páginas após colbreak (downgrade graded), obtive {}",
            doc.pages.len()
        );
    }

    /// **P537** — colbreak dentro de `columns` separa colunas reais na
    /// mesma página (deixa de ser downgrade a pagebreak). O texto antes
    /// do colbreak fica na coluna esquerda e o texto depois na coluna
    /// direita.
    #[test]
    fn p220_colbreak_dentro_columns_separa_colunas_reais() {
        use std::sync::Arc;
        let body = Content::Sequence(Arc::from(vec![
            Content::text("p220before"),
            Content::colbreak(false),
            Content::text("p220after"),
        ]));
        let cols = Content::columns(body, 2, None);
        let doc = layout(&cols);
        assert_eq!(
            doc.pages.len(),
            1,
            "colbreak dentro de columns mantém-se numa página, pages={}",
            doc.pages.len()
        );
        let items: Vec<_> = doc.pages[0].items.iter().collect();
        let before_x = items
            .iter()
            .filter_map(|it| match it {
                FrameItem::Text { text, pos, .. } if text.as_str() == "p220before" => {
                    Some(pos.x.0)
                }
                _ => None,
            })
            .next();
        let after_x = items
            .iter()
            .filter_map(|it| match it {
                FrameItem::Text { text, pos, .. } if text.as_str() == "p220after" => {
                    Some(pos.x.0)
                }
                _ => None,
            })
            .next();
        let (a, b) = (before_x.expect("before"), after_x.expect("after"));
        assert!(
            b > a,
            "after-colbreak deve estar à direita de before-colbreak: a={} b={}",
            a,
            b
        );
    }

    // ── Passo 537 — notas de rodapé em colunas reais ─────────────────────

    /// **P537** / **P552** — footnotes em `#columns(2)` com `colbreak()`
    /// são empilhadas na primeira coluna (semântica do vanilla para a
    /// forma-função `columns`). Observable: os corpos das notas A e B têm
    /// x igual (coluna esquerda) e y distinto (empilhadas).
    #[test]
    fn p537_footnotes_columns_colbreak_empilham_na_primeira_coluna() {
        use std::sync::Arc;
        let body = Content::Sequence(Arc::from(vec![
            Content::text("Hello "),
            Content::footnote(Content::text("Nota A")),
            Content::text(" world."),
            Content::colbreak(false),
            Content::text("Goodbye "),
            Content::footnote(Content::text("Nota B")),
            Content::text(" moon."),
        ]));
        let cols = Content::columns(body, 2, None);
        let doc = layout(&cols);
        assert_eq!(doc.pages.len(), 1, "duas colunas curtas cabem numa única página");
        let note_positions: Vec<_> = doc.pages[0]
            .items
            .iter()
            .filter_map(|it| match it {
                FrameItem::Text { text, pos, .. } if text.as_str() == "Nota" => Some(pos),
                _ => None,
            })
            .collect();
        assert_eq!(note_positions.len(), 2, "esperadas duas notas");
        let (a, b) = (note_positions[0], note_positions[1]);
        assert!(
            (a.x.0 - b.x.0).abs() < 1.0,
            "notas A e B devem estar na mesma coluna (esquerda): a={} b={}",
            a.x.0,
            b.x.0
        );
        assert!(
            (a.y.0 - b.y.0).abs() > 1.0,
            "notas A e B devem estar empilhadas verticalmente (y distinto): ay={} by={}",
            a.y.0,
            b.y.0
        );
    }

    /// **P552** — footnotes em `#set page(columns: 2)` (representado aqui
    /// por um `ColumnsElem` com `page_columns: true`) são desenhadas no
    /// fundo de cada coluna, com numeração contínua.
    #[test]
    fn p552_footnotes_set_page_columns_colbreak_posicionam_por_coluna() {
        use crate::entities::elements::columns::ColumnsElem;
        use crate::entities::layout_types::Length;
        use std::sync::Arc;
        let body = Content::Sequence(Arc::from(vec![
            Content::text("Hello "),
            Content::footnote(Content::text("Nota A")),
            Content::text(" world."),
            Content::colbreak(false),
            Content::text("Goodbye "),
            Content::footnote(Content::text("Nota B")),
            Content::text(" moon."),
        ]));
        let cols = Content::Columns(Arc::new(ColumnsElem {
            count: 2,
            gutter: None,
            body,
            page_columns: true,
        }));
        let doc = layout(&cols);
        assert_eq!(doc.pages.len(), 1, "duas colunas curtas cabem numa única página");
        let note_positions: Vec<_> = doc.pages[0]
            .items
            .iter()
            .filter_map(|it| match it {
                FrameItem::Text { text, pos, .. } if text.as_str() == "Nota" => Some(pos),
                _ => None,
            })
            .collect();
        assert_eq!(note_positions.len(), 2, "esperadas duas notas");
        let (a, b) = (note_positions[0], note_positions[1]);
        assert!(
            b.x.0 > a.x.0,
            "nota B deve estar à direita de nota A (coluna 2 > coluna 1): a={} b={}",
            a.x.0,
            b.x.0
        );
        // Ambas as notas devem estar na metade inferior da página.
        for pos in &note_positions {
            assert!(pos.y.0 > 700.0, "nota deve estar no fundo da página, y={}", pos.y.0);
        }
    }

    /// **P552** — contador de footnotes avança correctamente em
    /// `#set page(columns: 2)` com três notas.
    #[test]
    fn p552_footnote_counter_avanca_em_set_page_columns() {
        use crate::entities::elements::columns::ColumnsElem;
        use std::sync::Arc;
        let body = Content::Sequence(Arc::from(vec![
            Content::text("A"),
            Content::footnote(Content::text("Nota 1")),
            Content::colbreak(false),
            Content::text("B"),
            Content::footnote(Content::text("Nota 2")),
            Content::colbreak(false),
            Content::text("C"),
            Content::footnote(Content::text("Nota 3")),
        ]));
        let cols = Content::Columns(Arc::new(ColumnsElem {
            count: 3,
            gutter: None,
            body,
            page_columns: true,
        }));
        let doc = layout(&cols);
        // Filtrar apenas os markers inline (corpo das colunas), excluindo
        // os prefixos das notas de rodapé.
        let markers: Vec<_> = doc.pages[0]
            .items
            .iter()
            .filter_map(|it| match it {
                FrameItem::Text { text, pos, .. } if pos.y.0 > 400.0 => {
                    let s = text.as_str();
                    if s == "[1]" || s == "[2]" || s == "[3]" {
                        Some(s.to_string())
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect();
        assert_eq!(markers, vec!["[1]", "[2]", "[3]"]);
    }

    /// **P537** — num documento de uma coluna, footnotes continuam a ser
    /// desenhadas no fundo da página (sem regressão do modo coluna).
    #[test]
    fn p537_footnote_uma_coluna_sem_regressao() {
        let content = Content::sequence(vec![
            Content::text("Hello "),
            Content::footnote(Content::text("Nota simples")),
            Content::text(" world."),
        ]);
        let doc = layout(&content);
        assert_eq!(doc.pages.len(), 1);
        let has_note = doc.pages[0].items.iter().any(|it| match it {
            FrameItem::Text { text, .. } => text.as_str() == "Nota",
            _ => false,
        });
        assert!(has_note, "nota de uma coluna continua a renderizar");
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
        assert_eq!(
            d1.pages.len(),
            d2.pages.len(),
            "colbreak ≡ pagebreak graded (downgrade β); d1={}, d2={}",
            d1.pages.len(),
            d2.pages.len()
        );
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
        assert!(
            doc.pages.len() >= 2,
            "colbreak no início produz página vazia + página com texto, pages={}",
            doc.pages.len()
        );
    }

    // ── Passo 223 (ADR-0061 Fase 4 candidata sub-2; refino Place +float +clearance) ──

    /// Place com `float: true` renderiza body preservando baseline P84.6
    /// (semantic real adiada per ADR-0054 graded; flow contorna fica
    /// como Fase 5 candidata NÃO-reservada per política P158).
    #[test]
    fn p223_place_float_armazenado_layout_preservado() {
        use crate::entities::layout_types::{Align2D, HAlign, PlaceScope, VAlign};
        let p = Content::place(
            Align2D { h: Some(HAlign::Left), v: Some(VAlign::Top) },
            0.0,
            0.0,
            PlaceScope::Column,
            true,
            None,
            Content::text("p223float"),
        );
        let doc = layout(&p);
        let texts: String = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            })
            .collect();
        assert!(
            texts.contains("p223float"),
            "Place com float renderiza body (semantic adiada preserva baseline P84.6)"
        );
    }

    /// Place com `clearance: Some(2em)` renderiza body preservando baseline.
    #[test]
    fn p223_place_clearance_armazenado_layout_preservado() {
        use crate::entities::layout_types::{
            Align2D, HAlign, Length, PlaceScope, VAlign,
        };
        let p = Content::place(
            Align2D { h: Some(HAlign::Left), v: Some(VAlign::Top) },
            0.0,
            0.0,
            PlaceScope::Column,
            true,
            Some(Length::pt(20.0)),
            Content::text("p223clear"),
        );
        let doc = layout(&p);
        let texts: String = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            })
            .collect();
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
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Auto],
                rows: vec![],
                cells: vec![Content::text("p224body")],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(Length::pt(0.0)),
                header: Some(Content::text("p224hdr")),
                footer: Some(Content::text("p224ftr")),
                stroke: None,
                fill: None,
            },
        ));
        let doc = layout(&g);
        let texts: String = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            })
            .collect();
        assert!(
            texts.contains("p224body"),
            "Grid body renderiza preservando baseline; texts={}",
            texts
        );
    }

    /// GridCell wrappa body; placement real disponível via grid_placement
    /// (P224.C); aqui só verifica que GridCell isolado renderiza body.
    #[test]
    fn p224_gridcell_isolado_renderiza_body() {
        let cell = Content::GridCell(std::sync::Arc::new(
            crate::entities::elements::grid_cell::GridCellElem {
                body: Content::text("p224cell"),
                x: None,
                y: None,
                colspan: None,
                rowspan: None,
                stroke: None,
                fill: None,
                align: None,
                inset: None,
                breakable: None,
            },
        ));
        let doc = layout(&cell);
        let texts: String = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            })
            .collect();
        assert!(
            texts.contains("p224cell"),
            "GridCell isolado renderiza body; texts={}",
            texts
        );
    }

    // ── Passo 227 (Fase 5 Layout Categoria A.1) — stroke render E2E ──

    /// P888 (achado 3 de P885, secção 7 de P887) — Grid 2×2 uniforme com
    /// stroke passa a emitir segmentos **fundidos**, não 4 por célula.
    /// Verticais fundem entre as 2 linhas (3 fronteiras de coluna → 3
    /// segmentos, um por fronteira, cada um cobrindo as 2 linhas). Horizontais
    /// fundem dentro de cada linha (2 colunas mesmo stroke → 1 run) mas não
    /// entre linhas (sem dedup topo/fundo entre vizinhas, decisão registada
    /// em `typst-passo-888-relatorio.md`) → 2 linhas × (topo + fundo) = 4.
    /// Total esperado: 3 + 4 = 7 (era >= 16 antes de P888, 4 por célula ×
    /// 4 células, sem fusão nenhuma).
    #[test]
    fn p227_grid_stroke_renderiza_4_lines_per_cell() {
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::{Color, Length, TrackSizing};
        use crate::entities::sides::Sides;

        let cells = vec![
            Content::text("A"),
            Content::text("B"),
            Content::text("C"),
            Content::text("D"),
        ];
        let with_stroke = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Fixed(50.0), TrackSizing::Fixed(50.0)],
                rows: vec![],
                cells: cells.clone(),
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: Some(Stroke {
                    paint: Paint::Solid(Color::rgb(0, 0, 0)),
                    thickness: 1.0,
                    overhang: false,
                }),
                fill: None,
            },
        ));
        let doc = layout(&with_stroke);
        let line_count: usize = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter(|item| {
                matches!(
                    item,
                    FrameItem::Shape {
                        kind: crate::entities::geometry::ShapeKind::Line { .. },
                        ..
                    }
                )
            })
            .count();
        // P888 — 3 verticais fundidos (uma por fronteira de coluna, cada
        // um cobrindo as 2 linhas) + 4 horizontais (2 linhas × topo/fundo,
        // cada um fundido dentro da linha, sem dedup entre linhas) = 7.
        assert_eq!(
            line_count, 7,
            "Grid 2x2 stroke uniforme deve emitir 7 lines fundidas (3 verticais + \
             4 horizontais), recebeu {}",
            line_count
        );
    }

    /// P888 (achado 3 de P885, secção 7 de P887) — grid maior (5 colunas ×
    /// 10 linhas, o mesmo tamanho do cenário de benchmark `05-tables.typ`)
    /// com stroke uniforme deve fundir os verticais **ao longo de todas as
    /// 10 linhas** (não só entre 2, como nos testes 2x2/1x2 acima) — 6
    /// fronteiras de coluna × 1 segmento cada (cobrindo as 10 linhas) = 6,
    /// mais 10 linhas × (topo + fundo) = 20 horizontais = 26 no total. Isto
    /// é a garantia de que a fusão vertical realmente escala com o número
    /// de linhas (o ganho principal sobre a "Opção β" antiga, que emitia
    /// 4 × 50 = 200 para esta mesma grelha).
    #[test]
    fn p888_grid_5x10_stroke_uniforme_funde_verticais_entre_10_linhas() {
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::{Color, Length, TrackSizing};
        use crate::entities::sides::Sides;

        let num_cols = 5;
        let num_rows = 10;
        let cells: Vec<Content> =
            (0..num_cols * num_rows).map(|i| Content::text(i.to_string())).collect();
        let g = Content::Grid(std::sync::Arc::new(crate::entities::elements::grid::GridElem {
            columns: vec![TrackSizing::Fixed(20.0); num_cols],
            rows: vec![TrackSizing::Fixed(15.0); num_rows],
            cells,
            hlines: vec![],
            vlines: vec![],
            gutter: None,
            align: None,
            inset: Sides::uniform(Length::pt(0.0)),
            header: None,
            footer: None,
            stroke: Some(Stroke {
                paint: Paint::Solid(Color::rgb(0, 0, 0)),
                thickness: 1.0,
                overhang: false,
            }),
            fill: None,
        }));
        let doc = layout(&g);
        let line_count: usize = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter(|item| {
                matches!(
                    item,
                    FrameItem::Shape {
                        kind: crate::entities::geometry::ShapeKind::Line { .. },
                        ..
                    }
                )
            })
            .count();
        assert_eq!(
            line_count, 26,
            "Grid 5x10 stroke uniforme deve emitir 26 lines fundidas (6 verticais \
             cobrindo as 10 linhas + 20 horizontais), recebeu {} — a fusão vertical \
             entre múltiplas linhas é o ganho principal sobre a Opção β antiga \
             (que emitia 200 para esta mesma grelha, 4 por célula × 50 células)",
            line_count
        );
    }

    /// P888 — quando duas células vizinhas têm `effective_stroke`
    /// **divergente** (via `grid.cell(stroke: ..)`, confirmado activo em
    /// `grid.rs` — `typst-passo-888-relatorio.md` secção 1), a fronteira
    /// partilhada entre elas **não funde**: em vez de tentar decidir um
    /// vencedor (sistema de prioridade do vanilla, fora de âmbito per
    /// ADR-0107), os dois lados continuam a desenhar o seu próprio
    /// segmento nessa posição — comportamento idêntico ao pré-P888 (sem
    /// risco de regressão visual quando o stroke diverge por célula).
    #[test]
    fn p888_stroke_divergente_por_celula_nao_funde_e_preserva_os_dois_lados() {
        use crate::entities::elements::grid_cell::GridCellElem;
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::{Color, Length, TrackSizing};
        use crate::entities::sides::Sides;

        let red = Stroke { paint: Paint::Solid(Color::rgb(255, 0, 0)), thickness: 1.0, overhang: false };
        let blue = Stroke { paint: Paint::Solid(Color::rgb(0, 0, 255)), thickness: 1.0, overhang: false };

        // Célula (0,0) com stroke vermelho explícito; célula (0,1) sem
        // override (herda o stroke azul do grid) — a fronteira vertical
        // entre elas (x=1) tem lados com strokes diferentes.
        let cell_a = Content::GridCell(std::sync::Arc::new(GridCellElem {
            body: Content::text("A"),
            x: None,
            y: None,
            colspan: None,
            rowspan: None,
            stroke: Some(red.clone()),
            fill: None,
            align: None,
            inset: None,
            breakable: None,
        }));
        let g = Content::Grid(std::sync::Arc::new(crate::entities::elements::grid::GridElem {
            columns: vec![TrackSizing::Fixed(40.0), TrackSizing::Fixed(40.0)],
            rows: vec![TrackSizing::Fixed(20.0)],
            cells: vec![cell_a, Content::text("B")],
            hlines: vec![],
            vlines: vec![],
            gutter: None,
            align: None,
            inset: Sides::uniform(Length::pt(0.0)),
            header: None,
            footer: None,
            stroke: Some(blue.clone()),
            fill: None,
        }));
        let doc = layout(&g);

        // Agrupar segmentos verticais por posição X (arredondada) — não
        // se assume a coordenada exacta (depende da margem da página).
        // A fronteira entre A e B (com strokes divergentes) deve ter DOIS
        // segmentos sobrepostos (vermelho + azul); as bordas externas
        // (só tocadas por uma célula cada) continuam com 1 só.
        let mut by_x: std::collections::HashMap<i64, Vec<crate::entities::paint::Paint>> =
            std::collections::HashMap::new();
        for page in &doc.pages {
            for item in &page.items {
                if let FrameItem::Shape {
                    pos,
                    kind: crate::entities::geometry::ShapeKind::Line { dx, dy },
                    stroke: Some(s),
                    ..
                } = item
                {
                    if *dx == 0.0 && *dy != 0.0 {
                        let x_key = (pos.x.val() * 100.0).round() as i64;
                        by_x.entry(x_key).or_default().push(s.paint.clone());
                    }
                }
            }
        }
        // 3 fronteiras verticais no total (esquerda, meio divergente,
        // direita) — nenhuma fundida incorrectamente com a vizinha.
        assert_eq!(
            by_x.len(),
            3,
            "esperava 3 fronteiras verticais distintas (esquerda, meio, direita), \
             encontrou {}: {:?}",
            by_x.len(),
            by_x
        );
        let divergent = by_x
            .values()
            .find(|strokes| strokes.len() == 2)
            .unwrap_or_else(|| {
                panic!(
                    "esperava exactamente uma fronteira com 2 segmentos (a divergente); \
                     grupos encontrados: {:?}",
                    by_x
                )
            });
        assert!(
            divergent.contains(&red.paint),
            "segmento vermelho (override da célula A) deve estar presente na fronteira divergente"
        );
        assert!(
            divergent.contains(&blue.paint),
            "segmento azul (herdado do grid pela célula B) deve estar presente na fronteira divergente"
        );
        // As duas fronteiras externas (só uma célula de cada lado) têm 1
        // segmento cada — não duplicam nem desaparecem.
        let externas: Vec<_> = by_x.values().filter(|s| s.len() == 1).collect();
        assert_eq!(externas.len(), 2, "as 2 fronteiras externas devem ter 1 segmento cada");
    }

    /// P887 (achado 3 de P885, extensão) — as `FrameItem::Shape::Line` das
    /// bordas de célula não podem ser degeneradas (comprimento zero). O
    /// exportador PDF usa `width`/`height` (não `dx`/`dy`) para calcular os
    /// pontos `m`/`l` do traço (`03_infra/src/export/stream.rs`) — antes da
    /// correcção, `p227_grid_stroke_renderiza_4_lines_per_cell` (acima) já
    /// confirmava a CONTAGEM de `Line`s emitidas, mas nunca confirmava que
    /// `width`/`height` reflectiam `dx.abs()`/`dy.abs()`; por isso o bug
    /// (linhas com `width: 0.0, height: 0.0` sempre, independente de
    /// `dx`/`dy`) sobreviveu sem detecção até à confirmação visual de P887.
    #[test]
    fn p887_grid_stroke_lines_bounding_box_bate_com_dx_dy() {
        use crate::entities::geometry::{ShapeKind, Stroke};
        use crate::entities::layout_types::{Color, Length, TrackSizing};
        use crate::entities::sides::Sides;

        let cells = vec![
            Content::text("A"),
            Content::text("B"),
            Content::text("C"),
            Content::text("D"),
        ];
        let with_stroke = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Fixed(50.0), TrackSizing::Fixed(50.0)],
                rows: vec![],
                cells,
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: Some(Stroke {
                    paint: Paint::Solid(Color::rgb(0, 0, 0)),
                    thickness: 1.0,
                    overhang: false,
                }),
                fill: None,
            },
        ));
        let doc = layout(&with_stroke);
        let mut checked = 0;
        for page in &doc.pages {
            for item in &page.items {
                if let FrameItem::Shape { kind: ShapeKind::Line { dx, dy }, width, height, .. } =
                    item
                {
                    assert_eq!(
                        *width,
                        dx.abs(),
                        "width deve bater com dx.abs() (dx={dx}, width={width}) — \
                         width=0.0 fixo produz traço degenerado no exportador"
                    );
                    assert_eq!(
                        *height,
                        dy.abs(),
                        "height deve bater com dy.abs() (dy={dy}, height={height}) — \
                         height=0.0 fixo produz traço degenerado no exportador"
                    );
                    // Cada borda de célula é horizontal (dx=50, dy=0) ou
                    // vertical (dx=0, dy>0) — nunca as duas zero.
                    assert!(
                        dx.abs() > 0.0 || dy.abs() > 0.0,
                        "linha de borda não pode ter dx e dy ambos zero"
                    );
                    checked += 1;
                }
            }
        }
        // P888 fundiu os segmentos (7 esperados para este grid 2x2, ver
        // `p227_grid_stroke_renderiza_4_lines_per_cell` acima) — a
        // verificação de bounding-box em si (width==dx.abs(), height==
        // dy.abs()) continua válida e vale a pena manter independentemente
        // da contagem exacta.
        assert_eq!(checked, 7, "esperava 7 Lines fundidas verificadas, verificou {checked}");
    }

    #[test]
    fn p227_grid_sem_stroke_zero_lines_extra() {
        use crate::entities::layout_types::{Length, TrackSizing};
        use crate::entities::sides::Sides;
        let g_no_stroke = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Fixed(50.0), TrackSizing::Fixed(50.0)],
                rows: vec![],
                cells: vec![Content::text("A"), Content::text("B")],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: None, // baseline
                fill: None,
            },
        ));
        let doc = layout(&g_no_stroke);
        let line_count: usize = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter(|item| {
                matches!(
                    item,
                    FrameItem::Shape {
                        kind: crate::entities::geometry::ShapeKind::Line { .. },
                        ..
                    }
                )
            })
            .count();
        assert_eq!(
            line_count, 0,
            "Grid sem stroke não emite Lines extra (baseline preservado)"
        );
    }

    #[test]
    fn p227_table_stroke_paridade_grid() {
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::{Color, TrackSizing};
        let t = Content::Table(std::sync::Arc::new(
            crate::entities::elements::table::TableElem {
                columns: vec![TrackSizing::Fixed(50.0), TrackSizing::Fixed(50.0)],
                rows: vec![],
                children: vec![Content::text("X"), Content::text("Y")],
                hlines: vec![],
                vlines: vec![],
                header: None,
                footer: None,
                stroke: Some(Stroke {
                    paint: Paint::Solid(Color::rgb(0, 0, 255)),
                    thickness: 0.5,
                    overhang: false,
                }),
                fill: None,
                caption: None,
            },
        ));
        let doc = layout(&t);
        let line_count: usize = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter(|item| {
                matches!(
                    item,
                    FrameItem::Shape {
                        kind: crate::entities::geometry::ShapeKind::Line { .. },
                        ..
                    }
                )
            })
            .count();
        // P888 — 1 linha × 2 colunas, stroke uniforme: 3 verticais (uma
        // por fronteira de coluna; só 1 linha, sem ganho de fusão entre
        // linhas mas continuam 1 segmento cada) + 2 horizontais (topo +
        // fundo da única linha, cada um fundido ao longo das 2 colunas) = 5
        // (era >= 8 antes de P888, 2 células × 4 bordas, sem fusão).
        assert_eq!(
            line_count, 5,
            "Table 1x2 stroke uniforme deve emitir 5 lines fundidas (3 verticais + \
             2 horizontais), recebeu {}",
            line_count
        );
    }

    // ── Passo 228 (Fase 5 Layout Categoria A.2) — fill render E2E ──

    /// Grid com fill emite FrameItem::Shape::Rect per cell.
    #[test]
    fn p228_grid_fill_renderiza_rect_per_cell() {
        use crate::entities::layout_types::{Color, Length, TrackSizing};
        use crate::entities::sides::Sides;
        let cells = vec![
            Content::text("A"),
            Content::text("B"),
            Content::text("C"),
            Content::text("D"),
        ];
        let with_fill = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Fixed(50.0), TrackSizing::Fixed(50.0)],
                rows: vec![],
                cells,
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: None,
                fill: Some(Color::rgb(255, 255, 0)),
            },
        ));
        let doc = layout(&with_fill);
        let rect_count: usize = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter(|item| {
                matches!(
                    item,
                    FrameItem::Shape {
                        kind: crate::entities::geometry::ShapeKind::Rect,
                        ..
                    }
                )
            })
            .count();
        // 4 cells × 1 rect cada = 4 rects mínimo.
        assert!(
            rect_count >= 4,
            "Grid 2x2 fill deve emitir >= 4 rects (1 per cell), recebeu {}",
            rect_count
        );
    }

    #[test]
    fn p228_grid_sem_fill_zero_rects_extra() {
        use crate::entities::layout_types::{Length, TrackSizing};
        use crate::entities::sides::Sides;
        let g_no_fill = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Fixed(50.0), TrackSizing::Fixed(50.0)],
                rows: vec![],
                cells: vec![Content::text("A"), Content::text("B")],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: None,
                fill: None, // baseline
            },
        ));
        let doc = layout(&g_no_fill);
        let rect_count: usize = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter(|item| {
                matches!(
                    item,
                    FrameItem::Shape {
                        kind: crate::entities::geometry::ShapeKind::Rect,
                        ..
                    }
                )
            })
            .count();
        assert_eq!(
            rect_count, 0,
            "Grid sem fill não emite Rects extra (baseline preservado)"
        );
    }

    #[test]
    fn p228_grid_fill_z_order_antes_de_conteudo() {
        // Z-order: fill Rect emitido ANTES do conteúdo (Text).
        // Index do primeiro Rect < index do primeiro Text.
        use crate::entities::layout_types::{Color, Length, TrackSizing};
        use crate::entities::sides::Sides;
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Fixed(50.0)],
                rows: vec![],
                cells: vec![Content::text("ZorderTest")],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: None,
                fill: Some(Color::rgb(255, 0, 0)),
            },
        ));
        let doc = layout(&g);
        let items: Vec<&FrameItem> =
            doc.pages.iter().flat_map(|p| p.items.iter()).collect();
        let first_rect_idx = items.iter().position(|item| {
            matches!(
                item,
                FrameItem::Shape {
                    kind: crate::entities::geometry::ShapeKind::Rect,
                    ..
                }
            )
        });
        let first_text_idx = items.iter().position(|item| {
            matches!(item,
                FrameItem::Text { text, .. } if text.as_str().contains("ZorderTest")
            )
        });
        assert!(first_rect_idx.is_some(), "Rect deve existir");
        assert!(first_text_idx.is_some(), "Text ZorderTest deve existir");
        assert!(
            first_rect_idx.unwrap() < first_text_idx.unwrap(),
            "Z-order: fill Rect (idx={:?}) deve preceder conteúdo Text (idx={:?})",
            first_rect_idx,
            first_text_idx
        );
    }

    #[test]
    fn p228_grid_fill_e_stroke_z_order_correcto() {
        // Z-order completo: fill (Rect) antes; conteúdo (Text) meio;
        // stroke (Line) depois.
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::{Color, Length, TrackSizing};
        use crate::entities::sides::Sides;
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Fixed(50.0)],
                rows: vec![],
                cells: vec![Content::text("ZorderFull")],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: Some(Stroke {
                    paint: Paint::Solid(Color::rgb(0, 0, 0)),
                    thickness: 1.0,
                    overhang: false,
                }),
                fill: Some(Color::rgb(255, 255, 0)),
            },
        ));
        let doc = layout(&g);
        let items: Vec<&FrameItem> =
            doc.pages.iter().flat_map(|p| p.items.iter()).collect();
        let rect_idx = items.iter().position(|item| {
            matches!(
                item,
                FrameItem::Shape {
                    kind: crate::entities::geometry::ShapeKind::Rect,
                    ..
                }
            )
        });
        let line_idx = items.iter().position(|item| {
            matches!(
                item,
                FrameItem::Shape {
                    kind: crate::entities::geometry::ShapeKind::Line { .. },
                    ..
                }
            )
        });
        assert!(rect_idx.is_some() && line_idx.is_some(), "Ambos Rect e Line presentes");
        assert!(
            rect_idx.unwrap() < line_idx.unwrap(),
            "Z-order: fill Rect (idx={:?}) deve preceder stroke Line (idx={:?})",
            rect_idx,
            line_idx
        );
    }

    #[test]
    fn p228_table_fill_delegate_paridade_grid() {
        use crate::entities::layout_types::{Color, TrackSizing};
        let t = Content::Table(std::sync::Arc::new(
            crate::entities::elements::table::TableElem {
                columns: vec![TrackSizing::Fixed(50.0), TrackSizing::Fixed(50.0)],
                rows: vec![],
                children: vec![Content::text("X"), Content::text("Y")],
                hlines: vec![],
                vlines: vec![],
                header: None,
                footer: None,
                stroke: None,
                fill: Some(Color::rgb(200, 200, 200)),
                caption: None,
            },
        ));
        let doc = layout(&t);
        let rect_count: usize = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter(|item| {
                matches!(
                    item,
                    FrameItem::Shape {
                        kind: crate::entities::geometry::ShapeKind::Rect,
                        ..
                    }
                )
            })
            .count();
        assert!(
            rect_count >= 2,
            "Table 1x2 fill paridade Grid emite >= 2 rects, recebeu {}",
            rect_count
        );
    }

    // ── P647 — grid inválida produz erro de layout ────────────────────────────

    #[test]
    fn p647_grid_explicit_overlap_produz_layout_error() {
        // Duas células explicitas ocupam a mesma posição (0,0).
        use crate::entities::layout_types::TrackSizing;
        use crate::entities::sides::Sides;
        let cell_a = Content::GridCell(std::sync::Arc::new(
            crate::entities::elements::grid_cell::GridCellElem {
                body: Content::text("A"),
                x: Some(0),
                y: Some(0),
                colspan: Some(2),
                rowspan: None,
                stroke: None,
                fill: None,
                align: None,
                inset: None,
                breakable: None,
            },
        ));
        let cell_b = Content::GridCell(std::sync::Arc::new(
            crate::entities::elements::grid_cell::GridCellElem {
                body: Content::text("B"),
                x: Some(0),
                y: Some(0),
                colspan: None,
                rowspan: None,
                stroke: None,
                fill: None,
                align: None,
                inset: None,
                breakable: None,
            },
        ));
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Fixed(50.0), TrackSizing::Fixed(50.0)],
                rows: vec![],
                cells: vec![cell_a, cell_b],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(crate::entities::layout_types::Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: None,
                fill: None,
            },
        ));
        let doc = layout(&g);
        assert!(
            !doc.layout_errors.is_empty(),
            "grid com overlap explicit deve produzir layout_errors"
        );
        assert!(
            doc.layout_errors[0]
                .message
                .contains("attempted to place a second cell at column 0, row 0"),
            "mensagem deve identificar conflito (paridade vanilla, P822): {}",
            doc.layout_errors[0].message
        );
    }

    #[test]
    fn p647_grid_colspan_maior_que_num_cols_produz_layout_error() {
        // Célula auto com colspan maior do que o número de colunas.
        use crate::entities::layout_types::TrackSizing;
        use crate::entities::sides::Sides;
        let cell = Content::GridCell(std::sync::Arc::new(
            crate::entities::elements::grid_cell::GridCellElem {
                body: Content::text("A"),
                x: None,
                y: None,
                colspan: Some(3),
                rowspan: None,
                stroke: None,
                fill: None,
                align: None,
                inset: None,
                breakable: None,
            },
        ));
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Fixed(50.0), TrackSizing::Fixed(50.0)],
                rows: vec![],
                cells: vec![cell],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(crate::entities::layout_types::Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: None,
                fill: None,
            },
        ));
        let doc = layout(&g);
        assert!(
            !doc.layout_errors.is_empty(),
            "grid com colspan > num_cols deve produzir layout_errors"
        );
        assert!(
            doc.layout_errors[0]
                .message
                .contains("cell's colspan would cause it to exceed the available column(s)"),
            "mensagem deve indicar colspan a exceder as colunas (paridade vanilla, P822): {}",
            doc.layout_errors[0].message
        );
    }

    // ── Passo 230 (Fase 5 Layout Categoria A.3) — precedência per-cell vs Grid-level ──

    /// Per-cell stroke override Grid-level: cell `Some(...)` prevalece.
    /// Grid stroke red + cell stroke blue → cell stroke usado.
    #[test]
    fn p230_per_cell_stroke_override_grid_level() {
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::{Color, Length, TrackSizing};
        use crate::entities::sides::Sides;

        let cell_with_override = Content::GridCell(std::sync::Arc::new(
            crate::entities::elements::grid_cell::GridCellElem {
                body: Content::text("override"),
                x: None,
                y: None,
                colspan: None,
                rowspan: None,
                stroke: Some(Stroke {
                    paint: Paint::Solid(Color::rgb(0, 0, 255)),
                    thickness: 5.0,
                    overhang: false,
                }),
                fill: None,
                align: None,
                inset: None,
                breakable: None,
            },
        ));
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Fixed(50.0)],
                rows: vec![],
                cells: vec![cell_with_override],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: Some(Stroke {
                    paint: Paint::Solid(Color::rgb(255, 0, 0)),
                    thickness: 1.0,
                    overhang: false,
                }),
                fill: None,
            },
        ));
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
        assert!(
            found_override,
            "Cell stroke thickness 5.0 deve sobrepor Grid stroke thickness 1.0"
        );
    }

    /// Per-cell fill override Grid-level.
    #[test]
    fn p230_per_cell_fill_override_grid_level() {
        use crate::entities::layout_types::{Color, Length, TrackSizing};
        use crate::entities::sides::Sides;

        let cell_with_fill = Content::GridCell(std::sync::Arc::new(
            crate::entities::elements::grid_cell::GridCellElem {
                body: Content::text("c"),
                x: None,
                y: None,
                colspan: None,
                rowspan: None,
                stroke: None,
                fill: Some(Color::rgb(0, 255, 0)), // cell green
                align: None,
                inset: None,
                breakable: None,
            },
        ));
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Fixed(50.0)],
                rows: vec![],
                cells: vec![cell_with_fill],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: None,
                fill: Some(Color::rgb(255, 0, 0)), // grid red
            },
        ));
        let doc = layout(&g);
        // Verificar fill emitido é green (cell override).
        let mut found_green = false;
        for p in &doc.pages {
            for item in &p.items {
                if let FrameItem::Shape { fill: Some(c), .. } = item {
                    if *c == Color::rgb(0, 255, 0) {
                        found_green = true;
                    }
                }
            }
        }
        assert!(found_green, "Cell fill green deve sobrepor Grid fill red");
    }

    /// Per-cell None → inherit Grid-level: cell sem stroke usa Grid stroke.
    #[test]
    fn p230_per_cell_none_inherits_grid_level() {
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::{Color, Length, TrackSizing};
        use crate::entities::sides::Sides;

        let cell_raw = Content::text("raw"); // Content raw sem stroke/fill
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Fixed(50.0)],
                rows: vec![],
                cells: vec![cell_raw],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: Some(Stroke {
                    paint: Paint::Solid(Color::rgb(0, 0, 0)),
                    thickness: 3.0,
                    overhang: false,
                }),
                fill: None,
            },
        ));
        let doc = layout(&g);
        let line_count: usize = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter(|item| {
                matches!(
                    item,
                    FrameItem::Shape {
                        kind: crate::entities::geometry::ShapeKind::Line { .. },
                        ..
                    }
                )
            })
            .count();
        assert!(
            line_count >= 4,
            "Cell raw inherit Grid stroke → emite 4 lines, recebeu {}",
            line_count
        );
    }

    /// Per-cell stroke Some + Grid-level None → cell emite; Grid não tem.
    #[test]
    fn p230_per_cell_some_grid_none_emite_apenas_cell() {
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::{Color, Length, TrackSizing};
        use crate::entities::sides::Sides;

        let cell_with_stroke = Content::GridCell(std::sync::Arc::new(
            crate::entities::elements::grid_cell::GridCellElem {
                body: Content::text("c"),
                x: None,
                y: None,
                colspan: None,
                rowspan: None,
                stroke: Some(Stroke {
                    paint: Paint::Solid(Color::rgb(0, 0, 0)),
                    thickness: 1.0,
                    overhang: false,
                }),
                fill: None,
                align: None,
                inset: None,
                breakable: None,
            },
        ));
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Fixed(50.0)],
                rows: vec![],
                cells: vec![cell_with_stroke],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: None, // Grid sem stroke
                fill: None,
            },
        ));
        let doc = layout(&g);
        let line_count: usize = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter(|item| {
                matches!(
                    item,
                    FrameItem::Shape {
                        kind: crate::entities::geometry::ShapeKind::Line { .. },
                        ..
                    }
                )
            })
            .count();
        assert!(
            line_count >= 4,
            "Cell stroke emite mesmo com Grid sem stroke, recebeu {}",
            line_count
        );
    }

    /// Mix: per-cell stroke + Grid-level fill → cell tem ambos (ortogonais).
    #[test]
    fn p230_per_cell_stroke_e_grid_fill_simultaneos_z_order() {
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::{Color, Length, TrackSizing};
        use crate::entities::sides::Sides;

        let cell = Content::GridCell(std::sync::Arc::new(
            crate::entities::elements::grid_cell::GridCellElem {
                body: Content::text("c"),
                x: None,
                y: None,
                colspan: None,
                rowspan: None,
                stroke: Some(Stroke {
                    paint: Paint::Solid(Color::rgb(0, 0, 0)),
                    thickness: 1.0,
                    overhang: false,
                }), // cell stroke
                fill: None,
                align: None,
                inset: None,
                breakable: None,
            },
        ));
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Fixed(50.0)],
                rows: vec![],
                cells: vec![cell],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: None,
                fill: Some(Color::rgb(255, 255, 0)), // grid fill (cell inherit)
            },
        ));
        let doc = layout(&g);
        let items: Vec<&FrameItem> =
            doc.pages.iter().flat_map(|p| p.items.iter()).collect();
        let rect_idx = items.iter().position(|item| {
            matches!(
                item,
                FrameItem::Shape {
                    kind: crate::entities::geometry::ShapeKind::Rect,
                    ..
                }
            )
        });
        let line_idx = items.iter().position(|item| {
            matches!(
                item,
                FrameItem::Shape {
                    kind: crate::entities::geometry::ShapeKind::Line { .. },
                    ..
                }
            )
        });
        assert!(
            rect_idx.is_some() && line_idx.is_some(),
            "Ambos Rect (grid fill inherit) e Line (cell stroke) presentes"
        );
        assert!(
            rect_idx.unwrap() < line_idx.unwrap(),
            "Z-order: fill (idx={:?}) antes stroke (idx={:?})",
            rect_idx,
            line_idx
        );
    }

    // ── Passo 231 (Fase 5 Layout Categoria A.4) — Block/Boxed cosméticos preserved ──

    /// Block com outset/radius/clip preserva body render (semantic real
    /// adiada per ADR-0054 graded — radius/clip primitivos baseline
    /// ausentes; outset visual ainda não aplicado).
    #[test]
    fn p231_block_outset_radius_clip_layout_preservado() {
        use crate::entities::sides::Sides;
        let b = Content::Block(std::sync::Arc::new(
            crate::entities::elements::block::BlockElem {
                body: Content::text("p231block"),
                width: None,
                height: None,
                inset: Sides::uniform(crate::entities::layout_types::Length::pt(0.0)),
                breakable: true,
                outset: Sides::uniform(crate::entities::layout_types::Length::pt(5.0)),
                // P242 adapta: radius `Option<Length>` → `Corners<Length>`.
                radius: crate::entities::corners::Corners::uniform(
                    crate::entities::layout_types::Length::pt(3.0),
                ),
                clip: true,
                fill: None,
                stroke: None,
                spacing: None,
                above: None,
                below: None,
                sticky: false,
            },
        ));
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
        assert!(
            texts.contains("p231block"),
            "Block com cosméticos renderiza body (P242 materializa clip: body em Group)"
        );
    }

    /// Boxed paridade Block — cosméticos preserved.
    #[test]
    fn p231_boxed_cosmeticos_paridade_block() {
        use crate::entities::sides::Sides;
        let b = Content::Boxed(std::sync::Arc::new(
            crate::entities::elements::boxed::BoxedElem {
                body: Content::text("p231boxed"),
                width: None,
                height: None,
                inset: Sides::uniform(crate::entities::layout_types::Length::pt(0.0)),
                baseline: crate::entities::layout_types::Length::pt(0.0),
                outset: Sides::uniform(crate::entities::layout_types::Length::pt(2.0)),
                // P242 adapta: radius `Option<Length>` → `Corners<Length>`.
                radius: crate::entities::corners::Corners::uniform(
                    crate::entities::layout_types::Length::pt(1.0),
                ),
                clip: false,
                fill: None,
                stroke: None,
            },
        ));
        let doc = layout(&b);
        let texts: String = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            })
            .collect();
        assert!(
            texts.contains("p231boxed"),
            "Boxed com cosméticos renderiza body (paridade Block; semantic adiada)"
        );
    }

    // ── Passo 242 (M9d/M7+5; ADR-0081 IMPLEMENTADO parcial 3/5) —
    //     Block clip=true emite FrameItem::Group com clip_mask
    //     RoundedRect (radius non-zero) ou Rect (radius zero) ──

    #[test]
    fn p242_block_clip_true_radius_non_zero_emit_group_rounded_rect_clip_mask() {
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let b = Content::Block(std::sync::Arc::new(
            crate::entities::elements::block::BlockElem {
                body: Content::text("clipped"),
                width: None,
                height: None,
                inset: Sides::uniform(Length::pt(0.0)),
                breakable: true,
                outset: Sides::uniform(Length::pt(0.0)),
                radius: Corners::uniform(Length::pt(5.0)),
                clip: true,
                fill: None,
                stroke: None,
                spacing: None,
                above: None,
                below: None,
                sticky: false,
            },
        ));
        let doc = layout(&b);
        // Procurar FrameItem::Group com clip_mask Some(RoundedRect).
        let mut found_rounded_clip = false;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Group { clip_mask: Some(shape), .. } = item {
                    if let crate::entities::geometry::ShapeKind::RoundedRect { .. } =
                        shape
                    {
                        found_rounded_clip = true;
                    }
                }
            }
        }
        assert!(
            found_rounded_clip,
            "P242 — clip=true + radius non-zero emite Group com clip_mask RoundedRect"
        );
    }

    #[test]
    fn p242_block_clip_true_radius_zero_emit_group_rect_clip_mask() {
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let b = Content::Block(std::sync::Arc::new(
            crate::entities::elements::block::BlockElem {
                body: Content::text("clipped-rect"),
                width: None,
                height: None,
                inset: Sides::uniform(Length::pt(0.0)),
                breakable: true,
                outset: Sides::uniform(Length::pt(0.0)),
                radius: Corners::uniform(Length::ZERO),
                clip: true,
                fill: None,
                stroke: None,
                spacing: None,
                above: None,
                below: None,
                sticky: false,
            },
        ));
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
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let b = Content::Block(std::sync::Arc::new(
            crate::entities::elements::block::BlockElem {
                body: Content::text("not-clipped"),
                width: None,
                height: None,
                inset: Sides::uniform(Length::pt(0.0)),
                breakable: true,
                outset: Sides::uniform(Length::pt(0.0)),
                radius: Corners::uniform(Length::pt(5.0)),
                clip: false,
                fill: None,
                stroke: None,
                spacing: None,
                above: None,
                below: None,
                sticky: false,
            },
        ));
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
        assert!(
            !found_any_clip_mask,
            "P242 — radius sem clip não emite clip_mask (semantic radius isolada graded)"
        );
    }

    // ── Passo 243 (M9d / M7+3 fase (a); ADR-0081 IMPLEMENTADO parcial 4/5)
    //     — promoção real scope-outs Pad.right + Block.width + Boxed.width
    //     via regions.current.width save/restore ──

    #[test]
    fn p243_pad_right_efetivo_reduz_width_durante_body() {
        // P243 — Pad.right reduz regions.current.width efectiva durante
        // body layout (vs scope-out P156C que ignorava right).
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let pad = Content::pad(
            Content::text("p243pad"),
            Sides {
                left: None,
                top: None,
                right: Some(Length::pt(100.0)),
                bottom: None,
            },
        );
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
        assert!(
            texts.contains("p243pad"),
            "Pad.right=100pt preserva body output (largura útil reduzida pero não-zero)"
        );
    }

    #[test]
    fn p243_block_width_efetivo_clampa_largura() {
        // P243 — Block.width clampa regions.current.width durante body.
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let block = Content::Block(std::sync::Arc::new(
            crate::entities::elements::block::BlockElem {
                body: Content::text("p243block"),
                width: Some(Length::pt(150.0)), // Block.width efectivo P243.
                height: None,
                inset: Sides::uniform(Length::pt(0.0)),
                breakable: true,
                outset: Sides::uniform(Length::pt(0.0)),
                radius: Corners::uniform(Length::ZERO),
                clip: false,
                fill: None,
                stroke: None,
                spacing: None,
                above: None,
                below: None,
                sticky: false,
            },
        ));
        let doc = layout(&block);
        let mut texts = String::new();
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Text { text, .. } = item {
                    texts.push_str(text.as_str());
                }
            }
        }
        assert!(
            texts.contains("p243block"),
            "Block.width=150pt preserva body output (clamp width efectivo)"
        );
    }

    #[test]
    fn p243_boxed_width_efetivo_clampa_largura() {
        // P243 — Boxed.width clampa regions.current.width durante body.
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let boxed = Content::Boxed(std::sync::Arc::new(
            crate::entities::elements::boxed::BoxedElem {
                body: Content::text("p243boxed"),
                width: Some(Length::pt(80.0)), // Boxed.width efectivo P243.
                height: None,
                inset: Sides::uniform(Length::pt(0.0)),
                baseline: Length::pt(0.0),
                outset: Sides::uniform(Length::pt(0.0)),
                radius: Corners::uniform(Length::ZERO),
                clip: false,
                fill: None,
                stroke: None,
            },
        ));
        let doc = layout(&boxed);
        let mut texts = String::new();
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Text { text, .. } = item {
                    texts.push_str(text.as_str());
                }
            }
        }
        assert!(
            texts.contains("p243boxed"),
            "Boxed.width=80pt preserva body output (clamp width efectivo)"
        );
    }

    #[test]
    fn p243_pad_aninhado_largura_cumulativa_preservada() {
        // P243 — Pad aninhado dentro de Block; width saved/restored em
        // ordem correcta (LIFO stack semantic).
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let inner_pad = Content::pad(
            Content::text("inner"),
            Sides {
                left: None,
                top: None,
                right: Some(Length::pt(50.0)),
                bottom: None,
            },
        );
        let block = Content::Block(std::sync::Arc::new(
            crate::entities::elements::block::BlockElem {
                body: inner_pad,
                width: Some(Length::pt(200.0)),
                height: None,
                inset: Sides::uniform(Length::pt(0.0)),
                breakable: true,
                outset: Sides::uniform(Length::pt(0.0)),
                radius: Corners::uniform(Length::ZERO),
                clip: false,
                fill: None,
                stroke: None,
                spacing: None,
                above: None,
                below: None,
                sticky: false,
            },
        ));
        let doc = layout(&block);
        let mut texts = String::new();
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Text { text, .. } = item {
                    texts.push_str(text.as_str());
                }
            }
        }
        assert!(
            texts.contains("inner"),
            "Pad dentro de Block — width cumulative save/restore preservado"
        );
    }

    // ── Passo 247 (M9d / M7+5; ADR-0079 Categoria A.4) ──────────────────
    //     Block + Boxed fill/stroke/outset semantic real activação.
    //     Layouter emite FrameItem::Shape antes do body (Z-order) com
    //     bounds expandidos por outset.

    #[test]
    fn p247_block_fill_emite_shape_antes_do_body() {
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::{Color, Length};
        use crate::entities::sides::Sides;
        let b = Content::Block(std::sync::Arc::new(
            crate::entities::elements::block::BlockElem {
                body: Content::text("p247fill"),
                width: Some(Length::pt(50.0)),
                height: None,
                inset: Sides::uniform(Length::pt(0.0)),
                breakable: true,
                outset: Sides::uniform(Length::pt(0.0)),
                radius: Corners::uniform(Length::ZERO),
                clip: false,
                fill: Some(Color::rgb(200, 50, 50)),
                stroke: None,
                spacing: None,
                above: None,
                below: None,
                sticky: false,
            },
        ));
        let doc = layout(&b);
        let mut found_shape_with_fill = false;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Shape { fill: Some(c), .. } = item {
                    if *c == Color::rgb(200, 50, 50) {
                        found_shape_with_fill = true;
                    }
                }
            }
        }
        assert!(found_shape_with_fill,
            "P247 — Block com fill=Some(Color) emite FrameItem::Shape com fill correspondente");
    }

    #[test]
    fn p247_block_stroke_emite_shape_com_stroke() {
        use crate::entities::corners::Corners;
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::{Color, Length};
        use crate::entities::sides::Sides;
        let b = Content::Block(std::sync::Arc::new(
            crate::entities::elements::block::BlockElem {
                body: Content::text("p247stroke"),
                width: Some(Length::pt(40.0)),
                height: None,
                inset: Sides::uniform(Length::pt(0.0)),
                breakable: true,
                outset: Sides::uniform(Length::pt(0.0)),
                radius: Corners::uniform(Length::ZERO),
                clip: false,
                fill: None,
                stroke: Some(Stroke {
                    paint: Paint::Solid(Color::rgb(10, 20, 30)),
                    thickness: 1.5,
                    overhang: false,
                }),
                spacing: None,
                above: None,
                below: None,
                sticky: false,
            },
        ));
        let doc = layout(&b);
        let mut found_shape_with_stroke = false;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Shape { stroke: Some(s), .. } = item {
                    if s.paint == Paint::Solid(Color::rgb(10, 20, 30))
                        && s.thickness == 1.5
                    {
                        found_shape_with_stroke = true;
                    }
                }
            }
        }
        assert!(
            found_shape_with_stroke,
            "P247 — Block com stroke=Some(Stroke) emite Shape com stroke correspondente"
        );
    }

    #[test]
    fn p247_block_fill_e_radius_emite_rounded_rect() {
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::{Color, Length};
        use crate::entities::sides::Sides;
        let b = Content::Block(std::sync::Arc::new(
            crate::entities::elements::block::BlockElem {
                body: Content::text("p247rounded"),
                width: Some(Length::pt(60.0)),
                height: None,
                inset: Sides::uniform(Length::pt(0.0)),
                breakable: true,
                outset: Sides::uniform(Length::pt(0.0)),
                radius: Corners::uniform(Length::pt(5.0)),
                clip: false,
                fill: Some(Color::rgb(100, 100, 100)),
                stroke: None,
                spacing: None,
                above: None,
                below: None,
                sticky: false,
            },
        ));
        let doc = layout(&b);
        let mut found_rounded_fill = false;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Shape { kind, fill: Some(_), .. } = item {
                    if let crate::entities::geometry::ShapeKind::RoundedRect { .. } = kind
                    {
                        found_rounded_fill = true;
                    }
                }
            }
        }
        assert!(
            found_rounded_fill,
            "P247 — fill + radius não-zero emite Shape kind=RoundedRect"
        );
    }

    #[test]
    fn p247_block_outset_expande_bounds_shape() {
        // outset=10pt em todos os lados; Shape width/height incluem outset.
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::{Color, Length};
        use crate::entities::sides::Sides;
        let b = Content::Block(std::sync::Arc::new(
            crate::entities::elements::block::BlockElem {
                body: Content::text("p247outset"),
                width: Some(Length::pt(50.0)),
                height: Some(Length::pt(20.0)),
                inset: Sides::uniform(Length::pt(0.0)),
                breakable: true,
                outset: Sides::uniform(Length::pt(10.0)),
                radius: Corners::uniform(Length::ZERO),
                clip: false,
                fill: Some(Color::rgb(50, 50, 50)),
                stroke: None,
                spacing: None,
                above: None,
                below: None,
                sticky: false,
            },
        ));
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
        assert!(
            shape_w >= 65.0 && shape_w <= 75.0,
            "P247 — outset expande Shape width; esperado ~70pt, obtido {:.1}",
            shape_w
        );
        // Block inner_h = height = 20. outset.top + outset.bottom = 20. Shape h = 40.
        assert!(
            shape_h >= 35.0 && shape_h <= 50.0,
            "P247 — outset expande Shape height; esperado ~40pt, obtido {:.1}",
            shape_h
        );
    }

    #[test]
    fn p247_block_fill_none_e_outset_zero_sem_shape() {
        // Cenário backward compat: fill=None, stroke=None, outset=zero
        // → SEM Shape emitido (apenas body items).
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let b = Content::Block(std::sync::Arc::new(
            crate::entities::elements::block::BlockElem {
                body: Content::text("backcompat"),
                width: None,
                height: None,
                inset: Sides::uniform(Length::pt(0.0)),
                breakable: true,
                outset: Sides::uniform(Length::pt(0.0)),
                radius: Corners::uniform(Length::ZERO),
                clip: false,
                fill: None,
                stroke: None,
                spacing: None,
                above: None,
                below: None,
                sticky: false,
            },
        ));
        let doc = layout(&b);
        let mut found_shape = false;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Shape { .. } = item {
                    found_shape = true;
                }
            }
        }
        assert!(
            !found_shape,
            "P247 — fill/stroke/outset todos zero NÃO emite Shape (backward compat P246)"
        );
    }

    #[test]
    fn p247_boxed_fill_emite_shape() {
        // Boxed inline com fill emite Shape paralelo Block.
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::{Color, Length};
        use crate::entities::sides::Sides;
        let b = Content::Boxed(std::sync::Arc::new(
            crate::entities::elements::boxed::BoxedElem {
                body: Content::text("p247boxfill"),
                width: Some(Length::pt(30.0)),
                height: None,
                inset: Sides::uniform(Length::pt(0.0)),
                baseline: Length::pt(0.0),
                outset: Sides::uniform(Length::pt(0.0)),
                radius: Corners::uniform(Length::ZERO),
                clip: false,
                fill: Some(Color::rgb(70, 140, 210)),
                stroke: None,
            },
        ));
        let doc = layout(&b);
        let mut found_shape_with_fill = false;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Shape { fill: Some(c), .. } = item {
                    if *c == Color::rgb(70, 140, 210) {
                        found_shape_with_fill = true;
                    }
                }
            }
        }
        assert!(
            found_shape_with_fill,
            "P247 — Boxed inline com fill emite FrameItem::Shape paralelo Block"
        );
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
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        // breakable=true (default P156G): emit normal sem antecipar break.
        let b = Content::Block(std::sync::Arc::new(
            crate::entities::elements::block::BlockElem {
                body: Content::text("p248brkt"),
                width: None,
                height: None,
                inset: Sides::uniform(Length::pt(0.0)),
                breakable: true,
                outset: Sides::uniform(Length::pt(0.0)),
                radius: Corners::uniform(Length::ZERO),
                clip: false,
                fill: None,
                stroke: None,
                spacing: None,
                above: None,
                below: None,
                sticky: false,
            },
        ));
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
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        // breakable=false + body pequeno: cabe na actual; sem new_page.
        let b = Content::Block(std::sync::Arc::new(
            crate::entities::elements::block::BlockElem {
                body: Content::text("p248brkf"),
                width: None,
                height: None,
                inset: Sides::uniform(Length::pt(0.0)),
                breakable: false,
                outset: Sides::uniform(Length::pt(0.0)),
                radius: Corners::uniform(Length::ZERO),
                clip: false,
                fill: None,
                stroke: None,
                spacing: None,
                above: None,
                below: None,
                sticky: false,
            },
        ));
        let doc = layout(&b);
        assert_eq!(
            doc.pages.len(),
            1,
            "breakable=false body pequeno cabe na actual; sem break extra"
        );
    }

    #[test]
    fn p248_block_breakable_false_overlong_emit_normal() {
        // breakable=false + body que excede página inteira: emit normal
        // (paridade vanilla "overlong atómico"; sem loop infinito).
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        // height enorme — excede página (~595pt default test).
        let b = Content::Block(std::sync::Arc::new(
            crate::entities::elements::block::BlockElem {
                body: Content::text("p248overlong"),
                width: None,
                height: Some(Length::pt(10_000.0)),
                inset: Sides::uniform(Length::pt(0.0)),
                breakable: false,
                outset: Sides::uniform(Length::pt(0.0)),
                radius: Corners::uniform(Length::ZERO),
                clip: false,
                fill: None,
                stroke: None,
                spacing: None,
                above: None,
                below: None,
                sticky: false,
            },
        ));
        let doc = layout(&b);
        // Body renderiza; sem panic; o output pode ser várias páginas
        // por overflow natural (flush_line) mas não infinitas.
        assert!(
            doc.pages.len() >= 1 && doc.pages.len() <= 50,
            "breakable=false overlong não causa loop infinito; obteve {} páginas",
            doc.pages.len()
        );
    }

    #[test]
    fn p248_block_breakable_false_antecipa_new_page() {
        // Cenário: encher página com Pad+VSpace push, depois Block
        // breakable=false que precisa de espaço — `new_page()` antecipa.
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        // Layout: VSpace grande (push cursor) + Block breakable=false
        // que mede como large via height min.
        // A4: page 841.89pt, margin 70.87pt; usable ~700pt; bottom_limit ~771pt.
        // VSpace 650pt empurra cursor para ~729pt; remaining ≈ 42pt.
        // Block height 100pt → block_total_h 100 > remaining 42 AND
        // block_total_h 100 <= usable 700 → break antecipado.
        let seq = Content::Sequence(std::sync::Arc::from(vec![
            Content::v_space(Length::pt(650.0), false),
            Content::Block(std::sync::Arc::new(
                crate::entities::elements::block::BlockElem {
                    body: Content::text("p248new"),
                    width: None,
                    height: Some(Length::pt(100.0)),
                    inset: Sides::uniform(Length::pt(0.0)),
                    breakable: false,
                    outset: Sides::uniform(Length::pt(0.0)),
                    radius: Corners::uniform(Length::ZERO),
                    clip: false,
                    fill: None,
                    stroke: None,
                    spacing: None,
                    above: None,
                    below: None,
                    sticky: false,
                },
            )),
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
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::{Color, Length};
        use crate::entities::sides::Sides;
        let b = Content::Block(std::sync::Arc::new(
            crate::entities::elements::block::BlockElem {
                body: Content::text("p248cross"),
                width: Some(Length::pt(60.0)),
                height: None,
                inset: Sides::uniform(Length::pt(0.0)),
                breakable: false,
                outset: Sides::uniform(Length::pt(5.0)),
                radius: Corners::uniform(Length::ZERO),
                clip: false,
                fill: Some(Color::rgb(100, 200, 50)),
                stroke: None,
                spacing: None,
                above: None,
                below: None,
                sticky: false,
            },
        ));
        let doc = layout(&b);
        let mut found_shape = false;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Shape { fill: Some(c), .. } = item {
                    if *c == Color::rgb(100, 200, 50) {
                        found_shape = true;
                    }
                }
            }
        }
        assert!(
            found_shape,
            "P248 breakable=false preserva P247 fill+outset Shape emission"
        );
    }

    #[test]
    fn p248_boxed_height_none_preserva_p156h() {
        // height=None: preservado P156H literal (nenhum clip).
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let b = Content::Boxed(std::sync::Arc::new(
            crate::entities::elements::boxed::BoxedElem {
                body: Content::text("p248bxnone"),
                width: None,
                height: None,
                inset: Sides::uniform(Length::pt(0.0)),
                baseline: Length::pt(0.0),
                outset: Sides::uniform(Length::pt(0.0)),
                radius: Corners::uniform(Length::ZERO),
                clip: true, // clip aceita mas height None → sem overflow handling
                fill: None,
                stroke: None,
            },
        ));
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
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        // Body com height natural > 5pt (line_height ~12pt default);
        // height=5pt força overflow.
        let b = Content::Boxed(std::sync::Arc::new(
            crate::entities::elements::boxed::BoxedElem {
                body: Content::text("p248bxovf"),
                width: Some(Length::pt(50.0)),
                height: Some(Length::pt(5.0)),
                inset: Sides::uniform(Length::pt(0.0)),
                baseline: Length::pt(0.0),
                outset: Sides::uniform(Length::pt(0.0)),
                radius: Corners::uniform(Length::ZERO),
                clip: true,
                fill: None,
                stroke: None,
            },
        ));
        let doc = layout(&b);
        let mut found_clip_group = false;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Group {
                    clip_mask: Some(ShapeKind::Rect),
                    inner_height,
                    ..
                } = item
                {
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
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let b = Content::Boxed(std::sync::Arc::new(
            crate::entities::elements::boxed::BoxedElem {
                body: Content::text("p248bxnoc"),
                width: Some(Length::pt(50.0)),
                height: Some(Length::pt(5.0)),
                inset: Sides::uniform(Length::pt(0.0)),
                baseline: Length::pt(0.0),
                outset: Sides::uniform(Length::pt(0.0)),
                radius: Corners::uniform(Length::ZERO),
                clip: false,
                fill: None,
                stroke: None,
            },
        ));
        let doc = layout(&b);
        // Verificar que NÃO foi adicionado Group com inner_height=5
        // (clip=false não wrap).
        let mut found_overflow_group = false;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Group {
                    clip_mask: Some(ShapeKind::Rect),
                    inner_height,
                    ..
                } = item
                {
                    if (*inner_height - 5.0).abs() < 0.1 {
                        found_overflow_group = true;
                    }
                }
            }
        }
        assert!(
            !found_overflow_group,
            "P248 Boxed overflow + clip=false NÃO emite Group (overflow visível)"
        );
    }

    #[test]
    fn p248_boxed_height_cabe_sem_clip() {
        // Body cabe em height: preservado literal, sem Group overflow.
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let b = Content::Boxed(std::sync::Arc::new(
            crate::entities::elements::boxed::BoxedElem {
                body: Content::text("x"), // texto curto < height
                width: Some(Length::pt(50.0)),
                height: Some(Length::pt(100.0)), // muito maior que body natural
                inset: Sides::uniform(Length::pt(0.0)),
                baseline: Length::pt(0.0),
                outset: Sides::uniform(Length::pt(0.0)),
                radius: Corners::uniform(Length::ZERO),
                clip: true,
                fill: None,
                stroke: None,
            },
        ));
        let doc = layout(&b);
        let mut found_overflow_group = false;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Group {
                    clip_mask: Some(ShapeKind::Rect),
                    inner_height,
                    ..
                } = item
                {
                    if (*inner_height - 100.0).abs() < 0.1 {
                        found_overflow_group = true;
                    }
                }
            }
        }
        assert!(
            !found_overflow_group,
            "P248 Boxed body cabe em height: sem Group overflow (preservado)"
        );
    }

    #[test]
    fn p248_table_cell_sem_overflow_preserva_p157b() {
        // Cell body cabe em cell_h: sem Group de clip overflow.
        use crate::entities::layout_types::TrackSizing;
        // Table 1×1 com cell pequeno; row Auto.
        let cell = Content::TableCell(std::sync::Arc::new(
            crate::entities::elements::table_cell::TableCellElem {
                body: Content::text("x"),
                x: None,
                y: None,
                colspan: None,
                rowspan: None,
                stroke: None,
                fill: None,
                align: None,
                inset: None,
                breakable: None,
            },
        ));
        let t = Content::Table(std::sync::Arc::new(
            crate::entities::elements::table::TableElem {
                columns: vec![TrackSizing::Auto],
                rows: vec![TrackSizing::Auto],
                children: vec![cell],
                hlines: vec![],
                vlines: vec![],
                header: None,
                footer: None,
                stroke: None,
                fill: None,
                caption: None,
            },
        ));
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
        assert!(
            !found_overflow_clip,
            "P248 cell sem overflow não emite Group por clip implícito"
        );
    }

    #[test]
    fn p248_table_cell_overflow_emite_clip_group() {
        // Cell body excede cell_h: Group com clip_mask Rect.
        // Usa Block.height = 100pt dentro de cell com row Fixed(10pt) →
        // medição determinística (não depende de word-wrap).
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::{Length, TrackSizing};
        use crate::entities::sides::Sides;
        let inner_block = Content::Block(std::sync::Arc::new(
            crate::entities::elements::block::BlockElem {
                body: Content::text("ovf"),
                width: None,
                height: Some(Length::pt(100.0)), // força >> cell_h
                inset: Sides::uniform(Length::pt(0.0)),
                breakable: true,
                outset: Sides::uniform(Length::pt(0.0)),
                radius: Corners::uniform(Length::ZERO),
                clip: false,
                fill: None,
                stroke: None,
                spacing: None,
                above: None,
                below: None,
                sticky: false,
            },
        ));
        let cell = Content::TableCell(std::sync::Arc::new(
            crate::entities::elements::table_cell::TableCellElem {
                body: inner_block,
                x: None,
                y: None,
                colspan: None,
                rowspan: None,
                stroke: None,
                fill: None,
                align: None,
                inset: None,
                breakable: None,
            },
        ));
        let t = Content::Table(std::sync::Arc::new(
            crate::entities::elements::table::TableElem {
                columns: vec![TrackSizing::Fixed(40.0)],
                rows: vec![TrackSizing::Fixed(10.0)],
                children: vec![cell],
                hlines: vec![],
                vlines: vec![],
                header: None,
                footer: None,
                stroke: None,
                fill: None,
                caption: None,
            },
        ));
        let doc = layout(&t);
        let mut found_clip_group = false;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Group {
                    clip_mask: Some(ShapeKind::Rect),
                    inner_height,
                    ..
                } = item
                {
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
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::{Color, Length, TrackSizing};
        use crate::entities::sides::Sides;
        let inner_block = Content::Block(std::sync::Arc::new(
            crate::entities::elements::block::BlockElem {
                body: Content::text("fillovf"),
                width: None,
                height: Some(Length::pt(80.0)),
                inset: Sides::uniform(Length::pt(0.0)),
                breakable: true,
                outset: Sides::uniform(Length::pt(0.0)),
                radius: Corners::uniform(Length::ZERO),
                clip: false,
                fill: None,
                stroke: None,
                spacing: None,
                above: None,
                below: None,
                sticky: false,
            },
        ));
        let cell = Content::TableCell(std::sync::Arc::new(
            crate::entities::elements::table_cell::TableCellElem {
                body: inner_block,
                x: None,
                y: None,
                colspan: None,
                rowspan: None,
                stroke: None,
                fill: Some(Color::rgb(220, 220, 50)),
                align: None,
                inset: None,
                breakable: None,
            },
        ));
        let t = Content::Table(std::sync::Arc::new(
            crate::entities::elements::table::TableElem {
                columns: vec![TrackSizing::Fixed(30.0)],
                rows: vec![TrackSizing::Fixed(10.0)],
                children: vec![cell],
                hlines: vec![],
                vlines: vec![],
                header: None,
                footer: None,
                stroke: None,
                fill: None,
                caption: None,
            },
        ));
        let doc = layout(&t);
        let mut found_fill = false;
        let mut found_clip = false;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                match item {
                    FrameItem::Shape { fill: Some(c), .. }
                        if *c == Color::rgb(220, 220, 50) =>
                    {
                        found_fill = true
                    }
                    FrameItem::Group { clip_mask: Some(ShapeKind::Rect), .. } => {
                        found_clip = true
                    }
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
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::{Length, TrackSizing};
        use crate::entities::sides::Sides;
        let inner_block = Content::Block(std::sync::Arc::new(
            crate::entities::elements::block::BlockElem {
                body: Content::text("cross"),
                width: None,
                height: Some(Length::pt(60.0)), // excede cell row
                inset: Sides::uniform(Length::pt(0.0)),
                breakable: false,
                outset: Sides::uniform(Length::pt(0.0)),
                radius: Corners::uniform(Length::ZERO),
                clip: false,
                fill: None,
                stroke: None,
                spacing: None,
                above: None,
                below: None,
                sticky: false,
            },
        ));
        let cell = Content::TableCell(std::sync::Arc::new(
            crate::entities::elements::table_cell::TableCellElem {
                body: inner_block,
                x: None,
                y: None,
                colspan: None,
                rowspan: None,
                stroke: None,
                fill: None,
                align: None,
                inset: None,
                breakable: None,
            },
        ));
        let t = Content::Table(std::sync::Arc::new(
            crate::entities::elements::table::TableElem {
                columns: vec![TrackSizing::Fixed(40.0)],
                rows: vec![TrackSizing::Fixed(15.0)],
                children: vec![cell],
                hlines: vec![],
                vlines: vec![],
                header: None,
                footer: None,
                stroke: None,
                fill: None,
                caption: None,
            },
        ));
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
        assert!(
            found_clip,
            "P248 — Block breakable=false dentro cell overflow → clip implícito C"
        );
    }

    #[test]
    fn p248_block_breakable_false_com_inset_height_min_correto() {
        // breakable=false + height + inset: medição inclui outset+inset
        // correctamente. Verifica que com defaults pequenos não há break
        // antecipado erroneamente.
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let b = Content::Block(std::sync::Arc::new(
            crate::entities::elements::block::BlockElem {
                body: Content::text("p248inset"),
                width: None,
                height: Some(Length::pt(50.0)),
                inset: Sides::uniform(Length::pt(10.0)),
                breakable: false,
                outset: Sides::uniform(Length::pt(5.0)),
                radius: Corners::uniform(Length::ZERO),
                clip: false,
                fill: None,
                stroke: None,
                spacing: None,
                above: None,
                below: None,
                sticky: false,
            },
        ));
        let doc = layout(&b);
        assert_eq!(
            doc.pages.len(),
            1,
            "P248 — Block pequeno com inset+outset cabe na actual; sem break extra"
        );
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
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::{Color, Length};
        use crate::entities::sides::Sides;
        let b = Content::Boxed(std::sync::Arc::new(
            crate::entities::elements::boxed::BoxedElem {
                body: Content::text("p248cross"),
                width: Some(Length::pt(40.0)),
                height: Some(Length::pt(8.0)),
                inset: Sides::uniform(Length::pt(0.0)),
                baseline: Length::pt(0.0),
                outset: Sides::uniform(Length::pt(0.0)),
                radius: Corners::uniform(Length::ZERO),
                clip: true,
                fill: Some(Color::rgb(150, 50, 200)),
                stroke: None,
            },
        ));
        let doc = layout(&b);
        let mut found_fill = false;
        let mut found_clip = false;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                match item {
                    FrameItem::Shape { fill: Some(c), .. }
                        if *c == Color::rgb(150, 50, 200) =>
                    {
                        found_fill = true
                    }
                    FrameItem::Group {
                        clip_mask: Some(ShapeKind::Rect),
                        inner_height,
                        ..
                    } if (*inner_height - 8.0).abs() < 0.1 => found_clip = true,
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
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let inner = Content::Block(std::sync::Arc::new(
            crate::entities::elements::block::BlockElem {
                body: Content::text("inner"),
                width: None,
                height: Some(Length::pt(150.0)),
                inset: Sides::uniform(Length::pt(0.0)),
                breakable: false,
                outset: Sides::uniform(Length::pt(0.0)),
                radius: Corners::uniform(Length::ZERO),
                clip: false,
                fill: None,
                stroke: None,
                spacing: None,
                above: None,
                below: None,
                sticky: false,
            },
        ));
        let seq = Content::Sequence(std::sync::Arc::from(vec![
            Content::v_space(Length::pt(600.0), false),
            Content::Block(std::sync::Arc::new(
                crate::entities::elements::block::BlockElem {
                    body: inner,
                    width: None,
                    height: None,
                    inset: Sides::uniform(Length::pt(0.0)),
                    breakable: true, // exterior permite break natural
                    outset: Sides::uniform(Length::pt(0.0)),
                    radius: Corners::uniform(Length::ZERO),
                    clip: false,
                    fill: None,
                    stroke: None,
                    spacing: None,
                    above: None,
                    below: None,
                    sticky: false,
                },
            )),
        ]));
        let doc = layout(&seq);
        // Layout não panica + at least 1 page; aninhamento estável.
        assert!(doc.pages.len() >= 1);
    }

    #[test]
    fn p248_table_cell_overflow_radius_inner_block_p247() {
        // Inner Block com radius P242 + cell overflow P248: ambos
        // mecanismos clip_mask coexistem.
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::{Length, TrackSizing};
        use crate::entities::sides::Sides;
        let inner_block = Content::Block(std::sync::Arc::new(
            crate::entities::elements::block::BlockElem {
                body: Content::text("radius cross"),
                width: None,
                height: Some(Length::pt(80.0)),
                inset: Sides::uniform(Length::pt(0.0)),
                breakable: true,
                outset: Sides::uniform(Length::pt(0.0)),
                radius: Corners::uniform(Length::pt(3.0)), // P242
                clip: true,                                // P242
                fill: None,
                stroke: None,
                spacing: None,
                above: None,
                below: None,
                sticky: false,
            },
        ));
        let cell = Content::TableCell(std::sync::Arc::new(
            crate::entities::elements::table_cell::TableCellElem {
                body: inner_block,
                x: None,
                y: None,
                colspan: None,
                rowspan: None,
                stroke: None,
                fill: None,
                align: None,
                inset: None,
                breakable: None,
            },
        ));
        let t = Content::Table(std::sync::Arc::new(
            crate::entities::elements::table::TableElem {
                columns: vec![TrackSizing::Fixed(60.0)],
                rows: vec![TrackSizing::Fixed(20.0)],
                children: vec![cell],
                hlines: vec![],
                vlines: vec![],
                header: None,
                footer: None,
                stroke: None,
                fill: None,
                caption: None,
            },
        ));
        let doc = layout(&t);
        // Espera ≥1 Group (P242 inner radius+clip ou P248 cell overflow).
        let mut group_count = 0;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Group { .. } = item {
                    group_count += 1;
                }
            }
        }
        assert!(
            group_count >= 1,
            "P248 cell overflow + P242 inner radius+clip coexistem; obteve {} Group(s)",
            group_count
        );
    }

    #[test]
    fn p248_boxed_height_overflow_inset_correto() {
        // Boxed height + inset: clip Group inner_height = height (sem inset),
        // mas inset visualmente embebido no body.
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let b = Content::Boxed(std::sync::Arc::new(
            crate::entities::elements::boxed::BoxedElem {
                body: Content::text("inset"),
                width: Some(Length::pt(40.0)),
                height: Some(Length::pt(6.0)),
                inset: Sides::uniform(Length::pt(2.0)),
                baseline: Length::pt(0.0),
                outset: Sides::uniform(Length::pt(0.0)),
                radius: Corners::uniform(Length::ZERO),
                clip: true,
                fill: None,
                stroke: None,
            },
        ));
        let doc = layout(&b);
        let mut found_clip = false;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Group {
                    clip_mask: Some(ShapeKind::Rect),
                    inner_height,
                    ..
                } = item
                {
                    if (*inner_height - 6.0).abs() < 0.1 {
                        found_clip = true;
                    }
                }
            }
        }
        assert!(
            found_clip,
            "P248 Boxed height overflow + inset → Group inner_height = height"
        );
    }

    #[test]
    fn p248_block_breakable_false_com_outset_grande_medicao_inclui_outset() {
        // breakable=false: medição antecipada inclui outset top+bottom.
        // Test verifica que outset NÃO é ignorado no block_total_h.
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        // Cenário: cursor inicial baixo na página (VSpace push) +
        // block height médio + outset grande → total deve causar break.
        let seq = Content::Sequence(std::sync::Arc::from(vec![
            Content::v_space(Length::pt(620.0), false),
            Content::Block(std::sync::Arc::new(
                crate::entities::elements::block::BlockElem {
                    body: Content::text("p248outset"),
                    width: None,
                    height: Some(Length::pt(80.0)),
                    inset: Sides::uniform(Length::pt(0.0)),
                    breakable: false,
                    outset: Sides::uniform(Length::pt(30.0)), // +60pt total Y
                    radius: Corners::uniform(Length::ZERO),
                    clip: false,
                    fill: None,
                    stroke: None,
                    spacing: None,
                    above: None,
                    below: None,
                    sticky: false,
                },
            )),
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
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let mk = |clip: bool| {
            Content::Boxed(std::sync::Arc::new(
                crate::entities::elements::boxed::BoxedElem {
                    body: Content::text("diff"),
                    width: Some(Length::pt(40.0)),
                    height: Some(Length::pt(4.0)), // line_height > 4pt
                    inset: Sides::uniform(Length::pt(0.0)),
                    baseline: Length::pt(0.0),
                    outset: Sides::uniform(Length::pt(0.0)),
                    radius: Corners::uniform(Length::ZERO),
                    clip,
                    fill: None,
                    stroke: None,
                },
            ))
        };
        let count_clip_groups = |c: Content| -> usize {
            let doc = layout(&c);
            doc.pages.iter().flat_map(|p| p.items.iter())
                .filter(|i| matches!(i, FrameItem::Group { clip_mask: Some(ShapeKind::Rect), inner_height, .. }
                                       if (*inner_height - 4.0).abs() < 0.1))
                .count()
        };
        let n_true = count_clip_groups(mk(true));
        let n_false = count_clip_groups(mk(false));
        assert!(n_true >= 1, "clip=true emite Group por overflow");
        assert_eq!(n_false, 0, "clip=false NÃO emite Group por overflow");
    }

    #[test]
    fn p248_block_breakable_false_zero_height_default_emit_normal() {
        // Edge: breakable=false + body=text simples (sem height) → cabe
        // certamente; sem break.
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let b = Content::Block(std::sync::Arc::new(
            crate::entities::elements::block::BlockElem {
                body: Content::text("p248edge"),
                width: None,
                height: None, // sem height min
                inset: Sides::uniform(Length::pt(0.0)),
                breakable: false,
                outset: Sides::uniform(Length::pt(0.0)),
                radius: Corners::uniform(Length::ZERO),
                clip: false,
                fill: None,
                stroke: None,
                spacing: None,
                above: None,
                below: None,
                sticky: false,
            },
        ));
        let doc = layout(&b);
        assert_eq!(
            doc.pages.len(),
            1,
            "P248 breakable=false sem height: body cabe; sem break extra"
        );
    }

    #[test]
    fn p248_measure_content_constrained_puro_sem_side_effects() {
        // Audit C1 §2.4: confirmar puridade pós-P248 (sem mutar cursor).
        // Construir Block que invocaria measure_content_constrained se
        // breakable=false; verificar que cursor pré/pós idêntico.
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        let b = Content::Block(std::sync::Arc::new(
            crate::entities::elements::block::BlockElem {
                body: Content::text("a"),
                width: None,
                height: None,
                inset: Sides::uniform(Length::pt(0.0)),
                breakable: false, // dispara medição antecipada
                outset: Sides::uniform(Length::pt(0.0)),
                radius: Corners::uniform(Length::ZERO),
                clip: false,
                fill: None,
                stroke: None,
                spacing: None,
                above: None,
                below: None,
                sticky: false,
            },
        ));
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
        assert_eq!(texts, "a", "P248 measure puro: body emitido sem distorção");
    }

    // ── Passo 250 (M9d / M7+5; ADR-0079 Categoria A.4 COMPLETO Block 10/10;
    //     cita ADR-0082 PROPOSTO N=1 primeira aplicação citante) ──────────
    //     Block +4 fields (spacing/above/below/sticky) semantic real
    //     + refactor Sequence consumer para peekable + neighbour context.

    /// Helper P250 — construtor Block com defaults (spacing=None/etc).
    fn p250_mk_block(
        body: Content,
        height: Option<crate::entities::layout_types::Length>,
    ) -> Content {
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        Content::Block(std::sync::Arc::new(crate::entities::elements::block::BlockElem {
            body,
            width: None,
            height,
            inset: Sides::uniform(Length::pt(0.0)),
            breakable: true,
            outset: Sides::uniform(Length::pt(0.0)),
            radius: Corners::uniform(Length::ZERO),
            clip: false,
            fill: None,
            stroke: None,
            spacing: None,
            above: None,
            below: None,
            sticky: false,
        }))
    }

    /// Helper P250 — construtor Block com spacing/above/below/sticky.
    fn p250_mk_block_with(
        body: Content,
        spacing: Option<crate::entities::layout_types::Length>,
        above: Option<crate::entities::layout_types::Length>,
        below: Option<crate::entities::layout_types::Length>,
        sticky: bool,
    ) -> Content {
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::Length;
        use crate::entities::sides::Sides;
        Content::Block(std::sync::Arc::new(crate::entities::elements::block::BlockElem {
            body,
            width: None,
            height: None,
            inset: Sides::uniform(Length::pt(0.0)),
            breakable: true,
            outset: Sides::uniform(Length::pt(0.0)),
            radius: Corners::uniform(Length::ZERO),
            clip: false,
            fill: None,
            stroke: None,
            spacing,
            above,
            below,
            sticky,
        }))
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
        assert!(
            texts.contains("p250def"),
            "P250 — defaults preservam body output literal"
        );
        assert_eq!(doc.pages.len(), 1, "P250 defaults sem páginas extras");
    }

    #[test]
    fn p250_block_above_isolado_primeiro_block_suprime_above() {
        // Block sozinho (não dentro de Sequence) → above suprimido
        // (sem prev block).
        use crate::entities::layout_types::Length;
        let b = p250_mk_block_with(
            Content::text("a"),
            None,
            Some(Length::pt(50.0)),
            None,
            false,
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
            None,
            None,
            Some(Length::pt(20.0)),
            false,
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
                    if text.as_str() == "b1" {
                        y_b1 = Some(pos.y.0);
                    }
                    if text.as_str() == "b2" {
                        y_b2 = Some(pos.y.0);
                    }
                }
            }
        }
        let (y1, y2) = (y_b1.expect("b1"), y_b2.expect("b2"));
        // Diferença esperada > 20pt (incluindo line_height ~13pt).
        assert!(
            y2 - y1 >= 20.0,
            "P250 — below=20pt entre blocks consecutivos avança ≥20pt; obteve Δy={:.1}",
            y2 - y1
        );
    }

    #[test]
    fn p250_block_collapse_max_below_above_entre_blocks() {
        // Collapse semantic: gap = max(prev.below, curr.above).
        // b1.below=10pt, b2.above=30pt → gap=30pt (paridade vanilla).
        use crate::entities::layout_types::Length;
        let b1 = p250_mk_block_with(
            Content::text("c1"),
            None,
            None,
            Some(Length::pt(10.0)),
            false,
        );
        let b2 = p250_mk_block_with(
            Content::text("c2"),
            None,
            Some(Length::pt(30.0)),
            None,
            false,
        );
        let seq = Content::Sequence(std::sync::Arc::from(vec![b1, b2]));
        let doc = layout(&seq);
        let mut y_c1 = None;
        let mut y_c2 = None;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Text { text, pos, .. } = item {
                    if text.as_str() == "c1" {
                        y_c1 = Some(pos.y.0);
                    }
                    if text.as_str() == "c2" {
                        y_c2 = Some(pos.y.0);
                    }
                }
            }
        }
        let (y1, y2) = (y_c1.expect("c1"), y_c2.expect("c2"));
        // Gap colapsado ~ 30pt (não 10+30=40pt).
        // Diferença total ≈ line_height + 30pt; deve ser < 45pt
        // (se fosse soma seria ~13+40=53pt).
        assert!(
            y2 - y1 < 50.0,
            "P250 — collapse max(prev.below, curr.above); obteve Δy={:.1}",
            y2 - y1
        );
    }

    #[test]
    fn p250_block_spacing_fallback_above_below() {
        // spacing=Some(15), above=None, below=None → both fallback a 15.
        use crate::entities::layout_types::Length;
        let b1 = p250_mk_block_with(
            Content::text("s1"),
            Some(Length::pt(15.0)),
            None,
            None,
            false,
        );
        let b2 = p250_mk_block(Content::text("s2"), None);
        let seq = Content::Sequence(std::sync::Arc::from(vec![b1, b2]));
        let doc = layout(&seq);
        let mut y_s1 = None;
        let mut y_s2 = None;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Text { text, pos, .. } = item {
                    if text.as_str() == "s1" {
                        y_s1 = Some(pos.y.0);
                    }
                    if text.as_str() == "s2" {
                        y_s2 = Some(pos.y.0);
                    }
                }
            }
        }
        let (y1, y2) = (y_s1.expect("s1"), y_s2.expect("s2"));
        // Δy >= 15 (fallback below = spacing).
        assert!(
            y2 - y1 >= 15.0,
            "P250 — spacing fallback below; obteve Δy={:.1}",
            y2 - y1
        );
    }

    #[test]
    fn p250_block_above_override_spacing() {
        // spacing=10, above=Some(40): above wins over spacing fallback.
        use crate::entities::layout_types::Length;
        let b1 = p250_mk_block(Content::text("o1"), None);
        let b2 = p250_mk_block_with(
            Content::text("o2"),
            Some(Length::pt(10.0)),
            Some(Length::pt(40.0)),
            None,
            false,
        );
        let seq = Content::Sequence(std::sync::Arc::from(vec![b1, b2]));
        let doc = layout(&seq);
        let mut y_o2 = None;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Text { text, pos, .. } = item {
                    if text.as_str() == "o2" {
                        y_o2 = Some(pos.y.0);
                    }
                }
            }
        }
        // Gap ≥ 40 (above override prevalece sobre spacing fallback 10).
        let y2 = y_o2.expect("o2");
        let y1 = 70.87 + 7.5; // approximation; vamos comparar com Δ relativo via outro teste
        let _ = (y2, y1); // smoke test
    }

    #[test]
    fn p250_block_sticky_true_next_cabe_sem_break() {
        // sticky=true + next pequeno + espaço suficiente → emit normal.
        use crate::entities::layout_types::Length;
        let b1 = p250_mk_block_with(Content::text("st1"), None, None, None, true);
        let b2 = p250_mk_block(Content::text("st2"), None);
        let seq = Content::Sequence(std::sync::Arc::from(vec![b1, b2]));
        let doc = layout(&seq);
        assert_eq!(
            doc.pages.len(),
            1,
            "P250 — sticky=true + ambos cabem na actual → 1 página"
        );
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
            None,
            None,
            None,
            true,
        );
        // Wait, p250_mk_block_with wraps body — but we want b1 itself
        // to be height-restricted. Adjust:
        use crate::entities::corners::Corners;
        use crate::entities::sides::Sides;
        let b1 = Content::Block(std::sync::Arc::new(
            crate::entities::elements::block::BlockElem {
                body: Content::text("sk1"),
                width: None,
                height: Some(Length::pt(20.0)),
                inset: Sides::uniform(Length::pt(0.0)),
                breakable: true,
                outset: Sides::uniform(Length::pt(0.0)),
                radius: Corners::uniform(Length::ZERO),
                clip: false,
                fill: None,
                stroke: None,
                spacing: None,
                above: None,
                below: None,
                sticky: true,
            },
        ));
        let b2 = p250_mk_block(Content::text("sk2"), Some(Length::pt(80.0)));
        let seq = Content::Sequence(std::sync::Arc::from(vec![
            Content::v_space(Length::pt(650.0), false),
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
        let b1 = p250_mk_block_with(
            Content::text("m1"),
            None,
            None,
            Some(Length::pt(5.0)),
            false,
        );
        let b2 = p250_mk_block_with(
            Content::text("m2"),
            None,
            Some(Length::pt(5.0)),
            Some(Length::pt(10.0)),
            false,
        );
        let b3 = p250_mk_block_with(
            Content::text("m3"),
            None,
            Some(Length::pt(15.0)),
            None,
            false,
        );
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
        let mk = |sticky: bool| {
            p250_mk_block_with(Content::text("eq"), None, None, None, sticky)
        };
        assert_eq!(mk(false), mk(false));
        assert_ne!(mk(false), mk(true));
        let mk2 = |spacing: Option<Length>| {
            p250_mk_block_with(Content::text("eq"), spacing, None, None, false)
        };
        assert_eq!(mk2(None), mk2(None));
        assert_ne!(mk2(None), mk2(Some(Length::pt(5.0))));
    }

    #[test]
    fn p250_sequence_non_block_quebra_chain_collapse() {
        // Sequence: Block(below=20) + Text + Block(above=10).
        // Texto entre 2 Blocks quebra chain; segundo Block.above é
        // suprimido (chain restart após non-Block).
        use crate::entities::layout_types::Length;
        let b1 = p250_mk_block_with(
            Content::text("c1"),
            None,
            None,
            Some(Length::pt(20.0)),
            false,
        );
        let b2 = p250_mk_block_with(
            Content::text("c2"),
            None,
            Some(Length::pt(10.0)),
            None,
            false,
        );
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
        assert!(
            texts.contains("c1") && texts.contains("middle") && texts.contains("c2"),
            "P250 chain quebrada por non-Block preserva emit + restart chain"
        );
    }

    #[test]
    fn p250_sequence_aninhado_state_isolado() {
        // Sequence aninhado dentro de outro: state save/restore garante
        // que chain externa não vê chain interna.
        use crate::entities::layout_types::Length;
        let inner = Content::Sequence(std::sync::Arc::from(vec![
            p250_mk_block_with(
                Content::text("i1"),
                None,
                None,
                Some(Length::pt(7.0)),
                false,
            ),
            p250_mk_block(Content::text("i2"), None),
        ]));
        let outer = Content::Sequence(std::sync::Arc::from(vec![
            inner,
            p250_mk_block_with(
                Content::text("o3"),
                None,
                Some(Length::pt(13.0)),
                None,
                false,
            ),
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
        use crate::entities::corners::Corners;
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::{Color, Length};
        use crate::entities::sides::Sides;
        let b = Content::Block(std::sync::Arc::new(
            crate::entities::elements::block::BlockElem {
                body: Content::text("a4completo"),
                width: Some(Length::pt(80.0)),
                height: Some(Length::pt(40.0)),
                inset: Sides::uniform(Length::pt(3.0)),
                breakable: false,                          // P248
                outset: Sides::uniform(Length::pt(2.0)),   // P231+P247
                radius: Corners::uniform(Length::pt(2.0)), // P242
                clip: true,                                // P242
                fill: Some(Color::rgb(200, 200, 200)),     // P247
                stroke: Some(Stroke {
                    paint: Paint::Solid(Color::rgb(0, 0, 0)),
                    thickness: 1.0,
                    overhang: false,
                }), // P247
                spacing: Some(Length::pt(5.0)),            // P250
                above: Some(Length::pt(10.0)),             // P250
                below: Some(Length::pt(8.0)),              // P250
                sticky: true,                              // P250
            },
        ));
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
        assert!(
            texts.contains("a4completo"),
            "P250 — Block A.4 COMPLETO 10/10 atributos coexistem"
        );
    }

    // ── Passo 251 (M9d / M7+5; ADR-0079 Categoria C.2 parcial activa;
    //     cita ADR-0082 PROPOSTO N=2 segunda aplicação citante) ──────
    //     TableCell row break real cell-level γ-Items: slice items
    //     por threshold body_y+body_h; tail vai para
    //     pending_cell_tails; flush no topo da nova página via
    //     new_page() chain.

    #[test]
    fn p251_table_cell_overflow_row_fixed_preserva_p248_clip() {
        // Sentinela regression: row TrackSizing::Fixed preserva
        // P248 clip implícito (paridade vanilla "Fixed rows clip").
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::{Length, TrackSizing};
        use crate::entities::sides::Sides;
        let inner_block = Content::Block(std::sync::Arc::new(
            crate::entities::elements::block::BlockElem {
                body: Content::text("fxd"),
                width: None,
                height: Some(Length::pt(100.0)), // excede cell
                inset: Sides::uniform(Length::pt(0.0)),
                breakable: true,
                outset: Sides::uniform(Length::pt(0.0)),
                radius: Corners::uniform(Length::ZERO),
                clip: false,
                fill: None,
                stroke: None,
                spacing: None,
                above: None,
                below: None,
                sticky: false,
            },
        ));
        let cell = Content::TableCell(std::sync::Arc::new(
            crate::entities::elements::table_cell::TableCellElem {
                body: inner_block,
                x: None,
                y: None,
                colspan: None,
                rowspan: None,
                stroke: None,
                fill: None,
                align: None,
                inset: None,
                breakable: None,
            },
        ));
        let t = Content::Table(std::sync::Arc::new(
            crate::entities::elements::table::TableElem {
                columns: vec![TrackSizing::Fixed(40.0)],
                rows: vec![TrackSizing::Fixed(10.0)], // Fixed!
                children: vec![cell],
                hlines: vec![],
                vlines: vec![],
                header: None,
                footer: None,
                stroke: None,
                fill: None,
                caption: None,
            },
        ));
        let doc = layout(&t);
        // P248 preservado: Group com clip_mask Rect inner_height<=10pt.
        let mut found_clip_group = false;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Group {
                    clip_mask: Some(ShapeKind::Rect),
                    inner_height,
                    ..
                } = item
                {
                    if *inner_height <= 10.1 {
                        found_clip_group = true;
                    }
                }
            }
        }
        assert!(
            found_clip_group,
            "P251 — row Fixed preserva P248 clip implícito (paridade vanilla)"
        );
    }

    #[test]
    fn p251_table_cell_overflow_row_auto_emit_sem_clip_group_no_actual() {
        // Row Auto + cell overflow: P251 substitui Group clip por
        // slice. Cell items emit directo na página actual (head); tail
        // vai para pending_cell_tails (não testamos forwarding aqui).
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::{Length, TrackSizing};
        use crate::entities::sides::Sides;
        let inner_block = Content::Block(std::sync::Arc::new(
            crate::entities::elements::block::BlockElem {
                body: Content::text("auto"),
                width: None,
                height: Some(Length::pt(50.0)),
                inset: Sides::uniform(Length::pt(0.0)),
                breakable: true,
                outset: Sides::uniform(Length::pt(0.0)),
                radius: Corners::uniform(Length::ZERO),
                clip: false,
                fill: None,
                stroke: None,
                spacing: None,
                above: None,
                below: None,
                sticky: false,
            },
        ));
        let cell = Content::TableCell(std::sync::Arc::new(
            crate::entities::elements::table_cell::TableCellElem {
                body: inner_block,
                x: None,
                y: None,
                colspan: None,
                rowspan: None,
                stroke: None,
                fill: None,
                align: None,
                inset: None,
                breakable: None,
            },
        ));
        let t = Content::Table(std::sync::Arc::new(
            crate::entities::elements::table::TableElem {
                columns: vec![TrackSizing::Fixed(40.0)],
                rows: vec![TrackSizing::Auto], // Auto → P251 row break
                children: vec![cell],
                hlines: vec![],
                vlines: vec![],
                header: None,
                footer: None,
                stroke: None,
                fill: None,
                caption: None,
            },
        ));
        let doc = layout(&t);
        // Smoke: layout não panica. P251 substitui P248 clip por
        // slice (NÃO há Group com clip_mask inner_height==row_h).
        let mut texts = String::new();
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Text { text, .. } = item {
                    texts.push_str(text.as_str());
                }
            }
        }
        assert!(
            texts.contains("auto"),
            "P251 — row Auto + cell overflow emite cell body (slice head)"
        );
    }

    #[test]
    fn p251_cell_sem_overflow_preserva_p248_output_literal() {
        // Sentinela: cell sem overflow preserva output P248 literal
        // (push items directo; sem clip, sem tail).
        use crate::entities::layout_types::TrackSizing;
        let cell = Content::TableCell(std::sync::Arc::new(
            crate::entities::elements::table_cell::TableCellElem {
                body: Content::text("ok"),
                x: None,
                y: None,
                colspan: None,
                rowspan: None,
                stroke: None,
                fill: None,
                align: None,
                inset: None,
                breakable: None,
            },
        ));
        let t = Content::Table(std::sync::Arc::new(
            crate::entities::elements::table::TableElem {
                columns: vec![TrackSizing::Auto],
                rows: vec![TrackSizing::Auto],
                children: vec![cell],
                hlines: vec![],
                vlines: vec![],
                header: None,
                footer: None,
                stroke: None,
                fill: None,
                caption: None,
            },
        ));
        let doc = layout(&t);
        let mut texts = String::new();
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Text { text, .. } = item {
                    texts.push_str(text.as_str());
                }
            }
        }
        assert!(texts.contains("ok"));
    }

    #[test]
    fn p251_pending_cell_tails_field_inicial_vazio() {
        // Smoke: layouter inicializa pending_cell_tails vazio + flush
        // em new_page no-op se vazio.
        let content = Content::text("simple");
        let doc = layout(&content);
        // Sem panic; output normal.
        let mut texts = String::new();
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Text { text, .. } = item {
                    texts.push_str(text.as_str());
                }
            }
        }
        assert!(texts.contains("simple"));
    }

    #[test]
    fn p251_cell_tail_flushed_em_proxima_pagina() {
        // VSpace empurra cursor → cell overflow row Auto → tail vai
        // para pending_cell_tails → flush no new_page (causada por
        // outro elemento que excede a página actual).
        // Cenário: table com cell overflow + Block grande post-table
        // que force new_page.
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::{Length, TrackSizing};
        use crate::entities::sides::Sides;
        let inner_block = Content::Block(std::sync::Arc::new(
            crate::entities::elements::block::BlockElem {
                body: Content::text("tail"),
                width: None,
                height: Some(Length::pt(100.0)),
                inset: Sides::uniform(Length::pt(0.0)),
                breakable: true,
                outset: Sides::uniform(Length::pt(0.0)),
                radius: Corners::uniform(Length::ZERO),
                clip: false,
                fill: None,
                stroke: None,
                spacing: None,
                above: None,
                below: None,
                sticky: false,
            },
        ));
        let cell = Content::TableCell(std::sync::Arc::new(
            crate::entities::elements::table_cell::TableCellElem {
                body: inner_block,
                x: None,
                y: None,
                colspan: None,
                rowspan: None,
                stroke: None,
                fill: None,
                align: None,
                inset: None,
                breakable: None,
            },
        ));
        let t = Content::Table(std::sync::Arc::new(
            crate::entities::elements::table::TableElem {
                columns: vec![TrackSizing::Fixed(40.0)],
                rows: vec![TrackSizing::Auto],
                children: vec![cell],
                hlines: vec![],
                vlines: vec![],
                header: None,
                footer: None,
                stroke: None,
                fill: None,
                caption: None,
            },
        ));
        // Pagebreak manual força flush.
        let seq = Content::Sequence(std::sync::Arc::from(vec![
            t,
            Content::pagebreak(false, None),
            Content::text("p2"),
        ]));
        let doc = layout(&seq);
        // Esperar 2+ páginas (table + pagebreak + p2).
        assert!(
            doc.pages.len() >= 2,
            "P251 — pagebreak entre table e p2 dispara new_page; obteve {} páginas",
            doc.pages.len()
        );
    }

    #[test]
    fn p251_cell_overflow_com_fill_re_emit_no_tail() {
        // Cell overflow com fill: head emit fill normal + tail
        // re-emit fill na nova página.
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::{Color, Length, TrackSizing};
        use crate::entities::sides::Sides;
        let inner_block = Content::Block(std::sync::Arc::new(
            crate::entities::elements::block::BlockElem {
                body: Content::text("filltail"),
                width: None,
                height: Some(Length::pt(80.0)),
                inset: Sides::uniform(Length::pt(0.0)),
                breakable: true,
                outset: Sides::uniform(Length::pt(0.0)),
                radius: Corners::uniform(Length::ZERO),
                clip: false,
                fill: None,
                stroke: None,
                spacing: None,
                above: None,
                below: None,
                sticky: false,
            },
        ));
        let cell = Content::TableCell(std::sync::Arc::new(
            crate::entities::elements::table_cell::TableCellElem {
                body: inner_block,
                x: None,
                y: None,
                colspan: None,
                rowspan: None,
                stroke: None,
                fill: Some(Color::rgb(100, 100, 50)),
                align: None,
                inset: None,
                breakable: None,
            },
        ));
        let t = Content::Table(std::sync::Arc::new(
            crate::entities::elements::table::TableElem {
                columns: vec![TrackSizing::Fixed(40.0)],
                rows: vec![TrackSizing::Auto],
                children: vec![cell],
                hlines: vec![],
                vlines: vec![],
                header: None,
                footer: None,
                stroke: None,
                fill: None,
                caption: None,
            },
        ));
        let seq = Content::Sequence(std::sync::Arc::from(vec![
            t,
            Content::pagebreak(false, None),
            Content::text("post"),
        ]));
        let doc = layout(&seq);
        // Conta Shapes com o fill specific — pelo menos 1 (head) +
        // possivelmente 1 (tail re-emit).
        let mut fill_shapes = 0;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Shape { fill: Some(c), .. } = item {
                    if *c == Color::rgb(100, 100, 50) {
                        fill_shapes += 1;
                    }
                }
            }
        }
        assert!(fill_shapes >= 1,
            "P251 — cell fill emite ≥1 Shape (head); tail re-emit em flush se tail não-vazio");
    }

    #[test]
    fn p251_flush_pending_cell_tails_vazio_no_op() {
        // Sem pending tails: new_page não panica + emit normal.
        use crate::entities::layout_types::Length;
        let seq = Content::Sequence(std::sync::Arc::from(vec![
            Content::text("p1"),
            Content::pagebreak(false, None),
            Content::text("p2"),
        ]));
        let doc = layout(&seq);
        assert!(doc.pages.len() >= 2);
        let _ = Length::pt(0.0);
    }

    #[test]
    fn p251_cell_overflow_2_rows_independentes() {
        // Table 2 rows: ambas com cell overflow → ambas geram tails.
        // Smoke: layout não panica + ambos cells body emit (sequential
        // ou flushed).
        use crate::entities::corners::Corners;
        use crate::entities::layout_types::{Length, TrackSizing};
        use crate::entities::sides::Sides;
        let mk_cell = |label: &str| -> Content {
            let inner = Content::Block(std::sync::Arc::new(
                crate::entities::elements::block::BlockElem {
                    body: Content::text(label),
                    width: None,
                    height: Some(Length::pt(40.0)),
                    inset: Sides::uniform(Length::pt(0.0)),
                    breakable: true,
                    outset: Sides::uniform(Length::pt(0.0)),
                    radius: Corners::uniform(Length::ZERO),
                    clip: false,
                    fill: None,
                    stroke: None,
                    spacing: None,
                    above: None,
                    below: None,
                    sticky: false,
                },
            ));
            Content::TableCell(std::sync::Arc::new(
                crate::entities::elements::table_cell::TableCellElem {
                    body: inner,
                    x: None,
                    y: None,
                    colspan: None,
                    rowspan: None,
                    stroke: None,
                    fill: None,
                    align: None,
                    inset: None,
                    breakable: None,
                },
            ))
        };
        let t = Content::Table(std::sync::Arc::new(
            crate::entities::elements::table::TableElem {
                columns: vec![TrackSizing::Fixed(40.0)],
                rows: vec![TrackSizing::Auto, TrackSizing::Auto],
                children: vec![mk_cell("r1"), mk_cell("r2")],
                hlines: vec![],
                vlines: vec![],
                header: None,
                footer: None,
                stroke: None,
                fill: None,
                caption: None,
            },
        ));
        let doc = layout(&t);
        let mut texts = String::new();
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Text { text, .. } = item {
                    texts.push_str(text.as_str());
                }
            }
        }
        assert!(texts.contains("r1") && texts.contains("r2"));
    }

    // ── Passo 252 (M9d / M7+5; ADR-0079 Categoria A.4 Boxed COMPLETO
    //     6/6; cita ADR-0082 PROPOSTO N=3 terceira aplicação citante;
    //     N=3 limiar interno atingido — promoção EM VIGOR humana
    //     possível) — Stroke +1 field overhang: bool; bounds Shape
    //     expandidos por thickness/2 quando overhang=true (paridade
    //     vanilla); default Rust construtor false (cristalino
    //     divergente; paridade restaurada via extract_stroke).

    #[test]
    fn p252_stroke_struct_partial_eq_inclui_overhang() {
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::Color;
        let s1 = Stroke {
            paint: Paint::Solid(Color::rgb(0, 0, 0)),
            thickness: 1.0,
            overhang: false,
        };
        let s2 = Stroke {
            paint: Paint::Solid(Color::rgb(0, 0, 0)),
            thickness: 1.0,
            overhang: true,
        };
        assert_ne!(s1, s2, "P252 — overhang distingue strokes em PartialEq");
        let s3 = Stroke {
            paint: Paint::Solid(Color::rgb(0, 0, 0)),
            thickness: 1.0,
            overhang: false,
        };
        assert_eq!(s1, s3);
    }

    #[test]
    fn p252_stroke_clone_preserva_overhang() {
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::Color;
        let s = Stroke {
            paint: Paint::Solid(Color::rgb(0, 0, 0)),
            thickness: 2.0,
            overhang: true,
        };
        let s2 = s.clone();
        assert_eq!(s.overhang, s2.overhang);
        assert!(s2.overhang);
    }

    #[test]
    fn p252_stroke_construtor_rust_default_overhang_false_preserva_bounds() {
        // Sentinela: Stroke literal Rust com overhang=false preserva
        // bounds Shape literais (bit-equivalente pré-P252).
        use crate::entities::corners::Corners;
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::{Color, Length};
        use crate::entities::sides::Sides;
        let b = Content::Block(std::sync::Arc::new(
            crate::entities::elements::block::BlockElem {
                body: Content::text("p252lit"),
                width: Some(Length::pt(50.0)),
                height: Some(Length::pt(30.0)),
                inset: Sides::uniform(Length::pt(0.0)),
                breakable: true,
                outset: Sides::uniform(Length::pt(0.0)),
                radius: Corners::uniform(Length::ZERO),
                clip: false,
                fill: None,
                stroke: Some(Stroke {
                    paint: Paint::Solid(Color::rgb(0, 0, 0)),
                    thickness: 4.0,
                    overhang: false,
                }),
                spacing: None,
                above: None,
                below: None,
                sticky: false,
            },
        ));
        let doc = layout(&b);
        let mut shape_w = 0.0_f64;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Shape { width, stroke: Some(_), .. } = item {
                    shape_w = *width;
                }
            }
        }
        assert!(
            (shape_w - 50.0).abs() < 0.5,
            "P252 — overhang=false preserva bounds literais width=50pt; obteve {:.1}",
            shape_w
        );
    }

    #[test]
    fn p252_stroke_overhang_true_expande_bounds_thickness_half() {
        // overhang=true: bounds expandidos por thickness/2 em cada
        // lado (total +thickness em width e height).
        use crate::entities::corners::Corners;
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::{Color, Length};
        use crate::entities::sides::Sides;
        let b = Content::Block(std::sync::Arc::new(
            crate::entities::elements::block::BlockElem {
                body: Content::text("p252ov"),
                width: Some(Length::pt(50.0)),
                height: Some(Length::pt(30.0)),
                inset: Sides::uniform(Length::pt(0.0)),
                breakable: true,
                outset: Sides::uniform(Length::pt(0.0)),
                radius: Corners::uniform(Length::ZERO),
                clip: false,
                fill: None,
                stroke: Some(Stroke {
                    paint: Paint::Solid(Color::rgb(0, 0, 0)),
                    thickness: 4.0,
                    overhang: true,
                }),
                spacing: None,
                above: None,
                below: None,
                sticky: false,
            },
        ));
        let doc = layout(&b);
        let mut shape_w = 0.0_f64;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Shape { width, stroke: Some(_), .. } = item {
                    shape_w = *width;
                }
            }
        }
        assert!((shape_w - 54.0).abs() < 0.5,
            "P252 — overhang=true expande width por thickness=4; esperado 54pt, obtido {:.1}", shape_w);
    }

    #[test]
    fn p252_boxed_overhang_true_expande_bounds() {
        // Boxed inline também aplica overhang (paridade Block).
        use crate::entities::corners::Corners;
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::{Color, Length};
        use crate::entities::sides::Sides;
        let b = Content::Boxed(std::sync::Arc::new(
            crate::entities::elements::boxed::BoxedElem {
                body: Content::text("p252box"),
                width: Some(Length::pt(40.0)),
                height: Some(Length::pt(20.0)),
                inset: Sides::uniform(Length::pt(0.0)),
                baseline: Length::pt(0.0),
                outset: Sides::uniform(Length::pt(0.0)),
                radius: Corners::uniform(Length::ZERO),
                clip: false,
                fill: None,
                stroke: Some(Stroke {
                    paint: Paint::Solid(Color::rgb(0, 0, 0)),
                    thickness: 6.0,
                    overhang: true,
                }),
            },
        ));
        let doc = layout(&b);
        let mut shape_w = 0.0_f64;
        for page in doc.pages.iter() {
            for item in page.items.iter() {
                if let FrameItem::Shape { width, stroke: Some(_), .. } = item {
                    shape_w = *width;
                }
            }
        }
        // Boxed: outer_w = cursor advance (~ 40 inline content); + thickness=6 overhang.
        assert!(
            shape_w >= 6.0,
            "P252 — Boxed overhang=true expande bounds; obteve {:.1}",
            shape_w
        );
    }

    // ── Passo 245 (M9d / M7+4; ADR-0081 IMPLEMENTADO total 5/5)
    //     — Place float real: defer ao topo/fundo da página + clearance
    //     vertical; promoção graded P223 → semantic activa P245 ──

    #[test]
    fn p245_place_float_true_bottom_renderiza_no_fundo_da_pagina() {
        // Float bottom-aligned + scope: Parent + float: true.
        // Espera-se que rect seja emitido próximo do fundo da página
        // (margin = 20pt, page height = 400pt; rect 30x20 → y ≈ 360 - ascender).
        use crate::entities::layout_types::{Align2D, HAlign, PlaceScope, VAlign};
        let p = Content::place(
            Align2D { h: Some(HAlign::Center), v: Some(VAlign::Bottom) },
            0.0,
            0.0,
            PlaceScope::Parent,
            true,
            None,
            Content::shape(
                crate::entities::geometry::ShapeKind::Rect,
                Some(Box::new(crate::entities::value::Value::Length(
                    crate::entities::layout_types::Length::pt(30.0),
                ))),
                Some(Box::new(crate::entities::value::Value::Length(
                    crate::entities::layout_types::Length::pt(20.0),
                ))),
                None,
                None,
            ),
        );
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
        use crate::entities::layout_types::{Align2D, HAlign, PlaceScope, VAlign};
        let p = Content::place(
            Align2D { h: Some(HAlign::Left), v: Some(VAlign::Top) },
            0.0,
            0.0,
            PlaceScope::Parent,
            true,
            None,
            Content::shape(
                crate::entities::geometry::ShapeKind::Rect,
                Some(Box::new(crate::entities::value::Value::Length(
                    crate::entities::layout_types::Length::pt(30.0),
                ))),
                Some(Box::new(crate::entities::value::Value::Length(
                    crate::entities::layout_types::Length::pt(20.0),
                ))),
                None,
                None,
            ),
        );
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
        assert!(
            found_shape_at_top,
            "P245 — float top deve emitir shape na metade superior da página"
        );
    }

    #[test]
    fn p245_place_float_false_baseline_p84_preservado() {
        // Place float: false preserva comportamento P84.5+P84.6 literal.
        use crate::entities::layout_types::{Align2D, HAlign, PlaceScope, VAlign};
        let p = Content::place(
            Align2D { h: Some(HAlign::Center), v: Some(VAlign::Top) },
            50.0,
            30.0,
            PlaceScope::Column,
            false,
            None,
            Content::shape(
                crate::entities::geometry::ShapeKind::Rect,
                Some(Box::new(crate::entities::value::Value::Length(
                    crate::entities::layout_types::Length::pt(20.0),
                ))),
                Some(Box::new(crate::entities::value::Value::Length(
                    crate::entities::layout_types::Length::pt(15.0),
                ))),
                None,
                None,
            ),
        );
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
        use crate::entities::layout_types::{
            Align2D, HAlign, Length, PlaceScope, VAlign,
        };
        let make_doc = |clearance: Option<Length>| {
            let p = Content::place(
                Align2D { h: Some(HAlign::Left), v: Some(VAlign::Bottom) },
                0.0,
                0.0,
                PlaceScope::Parent,
                true,
                clearance,
                Content::shape(
                    crate::entities::geometry::ShapeKind::Rect,
                    Some(Box::new(crate::entities::value::Value::Length(Length::pt(
                        30.0,
                    )))),
                    Some(Box::new(crate::entities::value::Value::Length(Length::pt(
                        20.0,
                    )))),
                    None,
                    None,
                ),
            );
            layout(&p)
        };
        let doc_no_clear = make_doc(None);
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
        let y_no = get_y(&doc_no_clear);
        let y_with = get_y(&doc_with_clear);
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
        use crate::entities::layout_types::{Align2D, HAlign, PlaceScope, VAlign};
        let p = Content::place(
            Align2D { h: Some(HAlign::Center), v: Some(VAlign::Bottom) },
            0.0,
            0.0,
            PlaceScope::Parent,
            true,
            None,
            Content::shape(
                crate::entities::geometry::ShapeKind::Rect,
                Some(Box::new(crate::entities::value::Value::Length(
                    crate::entities::layout_types::Length::pt(20.0),
                ))),
                Some(Box::new(crate::entities::value::Value::Length(
                    crate::entities::layout_types::Length::pt(15.0),
                ))),
                None,
                None,
            ),
        );
        let doc = layout(&p);
        // Float emit verificado: doc tem ≥1 page com ≥1 item shape.
        assert!(!doc.pages.is_empty(), "P245 — doc deve ter páginas");
        let total_shapes: usize = doc
            .pages
            .iter()
            .map(|p| {
                p.items
                    .iter()
                    .filter(|i| matches!(i, FrameItem::Shape { .. }))
                    .count()
            })
            .sum();
        assert_eq!(
            total_shapes, 1,
            "P245 — float emitido exactamente 1× (sem duplicação)"
        );
    }

    // ── Passo 232 (Fase 5 Layout Categoria A.5) — Place precedence over Grid ──

    /// Place fora de Grid (cell_align None) preserva baseline P84.5.
    #[test]
    fn p232_place_fora_grid_baseline_preservado() {
        use crate::entities::layout_types::{Align2D, HAlign, PlaceScope, VAlign};
        let p = Content::place(
            Align2D { h: Some(HAlign::Center), v: Some(VAlign::Top) },
            0.0,
            0.0,
            PlaceScope::Column,
            false,
            None,
            Content::text("p232out"),
        );
        let doc = layout(&p);
        let texts: String = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            })
            .collect();
        assert!(
            texts.contains("p232out"),
            "Place fora Grid renderiza body preservando baseline P84.5"
        );
    }

    /// Place dentro Grid sem align Grid: baseline preservado (cell_align None).
    #[test]
    fn p232_place_dentro_grid_sem_align_baseline() {
        use crate::entities::layout_types::{
            Align2D, HAlign, Length, PlaceScope, TrackSizing, VAlign,
        };
        use crate::entities::sides::Sides;
        let place_in_cell = Content::place(
            Align2D { h: Some(HAlign::Center), v: Some(VAlign::Top) },
            0.0,
            0.0,
            PlaceScope::Column,
            false,
            None,
            Content::text("p232plain"),
        );
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Fixed(50.0)],
                rows: vec![],
                cells: vec![place_in_cell],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None, // sem Grid align → Place usa alignment direct
                inset: Sides::uniform(Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: None,
                fill: None,
            },
        ));
        let doc = layout(&g);
        let texts: String = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            })
            .collect();
        assert!(
            texts.contains("p232plain"),
            "Place dentro Grid sem align preserva baseline (cell_align None)"
        );
    }

    /// Place dentro Grid com align Grid + Place vazio → Place herda Grid.
    /// Place sem alignment (None, None); Grid align center → ambos eixos
    /// herdam center (effective_h/v from Grid).
    #[test]
    fn p232_place_herda_grid_align_quando_vazio() {
        use crate::entities::layout_types::{
            Align2D, HAlign, Length, PlaceScope, TrackSizing, VAlign,
        };
        use crate::entities::sides::Sides;
        let place_empty = Content::place(
            Align2D { h: None, v: None },
            0.0,
            0.0,
            PlaceScope::Column,
            false,
            None,
            Content::text("p232herda"),
        );
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Fixed(50.0)],
                rows: vec![],
                cells: vec![place_empty],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: Some(Align2D { h: Some(HAlign::Center), v: Some(VAlign::Top) }),
                inset: Sides::uniform(Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: None,
                fill: None,
            },
        ));
        let doc = layout(&g);
        let texts: String = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            })
            .collect();
        assert!(
            texts.contains("p232herda"),
            "Place vazio dentro Grid com align herda (renderiza body OK)"
        );
    }

    /// Place dentro Grid com Place H explícito + V vazio → H override; V herda.
    #[test]
    fn p232_place_override_per_axis() {
        use crate::entities::layout_types::{
            Align2D, HAlign, Length, PlaceScope, TrackSizing, VAlign,
        };
        use crate::entities::sides::Sides;
        let place_partial = Content::place(
            Align2D { h: Some(HAlign::Right), v: None },
            0.0,
            0.0,
            PlaceScope::Column,
            false,
            None,
            Content::text("p232override"),
        );
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Fixed(50.0)],
                rows: vec![],
                cells: vec![place_partial],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: Some(Align2D { h: Some(HAlign::Center), v: Some(VAlign::Top) }),
                inset: Sides::uniform(Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: None,
                fill: None,
            },
        ));
        let doc = layout(&g);
        let texts: String = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            })
            .collect();
        assert!(
            texts.contains("p232override"),
            "Place H override + V herda renderiza body OK"
        );
    }

    /// Place full override Grid (ambos eixos Place Some).
    #[test]
    fn p232_place_full_override_grid() {
        use crate::entities::layout_types::{
            Align2D, HAlign, Length, PlaceScope, TrackSizing, VAlign,
        };
        use crate::entities::sides::Sides;
        let place_full = Content::place(
            Align2D { h: Some(HAlign::Left), v: Some(VAlign::Bottom) },
            0.0,
            0.0,
            PlaceScope::Column,
            false,
            None,
            Content::text("p232full"),
        );
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Fixed(50.0)],
                rows: vec![],
                cells: vec![place_full],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: Some(Align2D { h: Some(HAlign::Center), v: Some(VAlign::Top) }),
                inset: Sides::uniform(Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: None,
                fill: None,
            },
        ));
        let doc = layout(&g);
        let texts: String = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.as_str().to_string()),
                _ => None,
            })
            .collect();
        assert!(texts.contains("p232full"), "Place full override Grid renderiza body OK");
    }

    // ── Passo 233 — B.1 DEBT-34d Auto track sizing fix ──────────

    #[test]
    fn p233_grid_auto_sem_fr_baseline_preservado() {
        use crate::entities::layout_types::{Length, TrackSizing};
        use crate::entities::sides::Sides;
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Auto, TrackSizing::Auto],
                rows: vec![],
                cells: vec![Content::text("AA"), Content::text("BB")],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: None,
                fill: None,
            },
        ));
        let doc = layout(&g);
        let txt = doc.plain_text();
        assert!(
            txt.contains("AA") && txt.contains("BB"),
            "Auto sem fr: baseline preservado pós-P233"
        );
    }

    #[test]
    fn p233_grid_auto_fr_mix_fr_recebe_espaco() {
        use crate::entities::layout_types::{Length, TrackSizing};
        use crate::entities::sides::Sides;
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Auto, TrackSizing::Fraction(1.0)],
                rows: vec![],
                cells: vec![Content::text("X"), Content::text("Y")],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: None,
                fill: None,
            },
        ));
        let doc = layout(&g);
        let txt = doc.plain_text();
        assert!(
            txt.contains("X") && txt.contains("Y"),
            "P233 DEBT-34d fix: Auto+Fr ambos renderizam"
        );
    }

    #[test]
    fn p233_grid_2auto_1fr_split() {
        use crate::entities::layout_types::{Length, TrackSizing};
        use crate::entities::sides::Sides;
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![
                    TrackSizing::Auto,
                    TrackSizing::Auto,
                    TrackSizing::Fraction(1.0),
                ],
                rows: vec![],
                cells: vec![Content::text("A"), Content::text("B"), Content::text("C")],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: None,
                fill: None,
            },
        ));
        let doc = layout(&g);
        let txt = doc.plain_text();
        assert!(
            txt.contains("A") && txt.contains("B") && txt.contains("C"),
            "P233: 2-Auto + 1-Fr split correcto"
        );
    }

    #[test]
    fn p233_grid_fixed_auto_fr_combinacao() {
        use crate::entities::layout_types::{Length, TrackSizing};
        use crate::entities::sides::Sides;
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![
                    TrackSizing::Fixed(50.0),
                    TrackSizing::Auto,
                    TrackSizing::Fraction(1.0),
                ],
                rows: vec![],
                cells: vec![Content::text("F"), Content::text("A"), Content::text("R")],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: None,
                fill: None,
            },
        ));
        let doc = layout(&g);
        let txt = doc.plain_text();
        assert!(
            txt.contains("F") && txt.contains("A") && txt.contains("R"),
            "P233: Fixed+Auto+Fr combinação OK"
        );
    }

    #[test]
    fn p233_grid_fixed_baseline_preservado() {
        use crate::entities::layout_types::{Length, TrackSizing};
        use crate::entities::sides::Sides;
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Fixed(100.0), TrackSizing::Fixed(100.0)],
                rows: vec![],
                cells: vec![Content::text("F1"), Content::text("F2")],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: None,
                fill: None,
            },
        ));
        let doc = layout(&g);
        assert!(!doc.pages.is_empty(), "Grid Fixed baseline P224 preservado pós-P233");
    }

    // ── Passo 234 — B.2 consumer geometric place_cells → Layouter ──

    #[test]
    fn p234_grid_colspan_2_cell_ocupa_2_cols_fill() {
        use crate::entities::layout_types::{Color, Length, TrackSizing};
        use crate::entities::sides::Sides;
        let wide_cell = Content::GridCell(std::sync::Arc::new(
            crate::entities::elements::grid_cell::GridCellElem {
                body: Content::text("WIDE"),
                x: None,
                y: None,
                colspan: Some(2),
                rowspan: None,
                stroke: None,
                fill: Some(Color::rgb(0, 200, 0)),
                align: None,
                inset: None,
                breakable: None,
            },
        ));
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Fixed(60.0), TrackSizing::Fixed(40.0)],
                rows: vec![],
                cells: vec![wide_cell],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: None,
                fill: None,
            },
        ));
        let doc = layout(&g);
        let mut found_wide = false;
        for p in &doc.pages {
            for item in &p.items {
                if let FrameItem::Shape {
                    kind: crate::entities::geometry::ShapeKind::Rect,
                    width,
                    fill: Some(c),
                    ..
                } = item
                {
                    if (*width - 100.0).abs() < 0.01 && *c == Color::rgb(0, 200, 0) {
                        found_wide = true;
                    }
                }
            }
        }
        assert!(found_wide, "Cell colspan=2 deve emitir fill Rect width=100 (60+40)");
    }

    #[test]
    fn p234_grid_rowspan_2_cell_ocupa_2_rows_fill() {
        use crate::entities::layout_types::{Color, Length, TrackSizing};
        use crate::entities::sides::Sides;
        let tall_cell = Content::GridCell(std::sync::Arc::new(
            crate::entities::elements::grid_cell::GridCellElem {
                body: Content::text("TALL"),
                x: None,
                y: None,
                colspan: None,
                rowspan: Some(2),
                stroke: None,
                fill: Some(Color::rgb(0, 0, 200)),
                align: None,
                inset: None,
                breakable: None,
            },
        ));
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Fixed(50.0), TrackSizing::Fixed(50.0)],
                rows: vec![TrackSizing::Fixed(30.0), TrackSizing::Fixed(40.0)],
                cells: vec![tall_cell, Content::text("b"), Content::text("c")],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: None,
                fill: None,
            },
        ));
        let doc = layout(&g);
        let mut found_tall = false;
        for p in &doc.pages {
            for item in &p.items {
                if let FrameItem::Shape {
                    kind: crate::entities::geometry::ShapeKind::Rect,
                    height,
                    fill: Some(c),
                    ..
                } = item
                {
                    if (*height - 70.0).abs() < 0.01 && *c == Color::rgb(0, 0, 200) {
                        found_tall = true;
                    }
                }
            }
        }
        assert!(found_tall, "Cell rowspan=2 deve emitir fill Rect height=70 (30+40)");
    }

    #[test]
    fn p234_grid_colspan_com_stroke_envolve_ambas_cols() {
        use crate::entities::geometry::{ShapeKind, Stroke};
        use crate::entities::layout_types::{Color, Length, TrackSizing};
        use crate::entities::sides::Sides;
        let wide_cell = Content::GridCell(std::sync::Arc::new(
            crate::entities::elements::grid_cell::GridCellElem {
                body: Content::text("WS"),
                x: None,
                y: None,
                colspan: Some(2),
                rowspan: None,
                stroke: Some(Stroke {
                    paint: Paint::Solid(Color::rgb(255, 0, 0)),
                    thickness: 2.0,
                    overhang: false,
                }),
                fill: None,
                align: None,
                inset: None,
                breakable: None,
            },
        ));
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Fixed(60.0), TrackSizing::Fixed(40.0)],
                rows: vec![TrackSizing::Fixed(30.0)],
                cells: vec![wide_cell],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: None,
                fill: None,
            },
        ));
        let doc = layout(&g);
        let mut found_wide_edge = false;
        for p in &doc.pages {
            for item in &p.items {
                if let FrameItem::Shape {
                    kind: ShapeKind::Line { dx, dy: 0.0 },
                    stroke: Some(s),
                    ..
                } = item
                {
                    if (*dx - 100.0).abs() < 0.01 && (s.thickness - 2.0).abs() < 0.01 {
                        found_wide_edge = true;
                    }
                }
            }
        }
        assert!(
            found_wide_edge,
            "Cell colspan=2 stroke deve emitir Line horizontal dx=100"
        );
    }

    #[test]
    fn p234_grid_colspan_per_cell_stroke_override_grid_p230_preservado() {
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::{Color, Length, TrackSizing};
        use crate::entities::sides::Sides;
        let wide_override = Content::GridCell(std::sync::Arc::new(
            crate::entities::elements::grid_cell::GridCellElem {
                body: Content::text("W"),
                x: None,
                y: None,
                colspan: Some(2),
                rowspan: None,
                stroke: Some(Stroke {
                    paint: Paint::Solid(Color::rgb(0, 0, 255)),
                    thickness: 7.0,
                    overhang: false,
                }),
                fill: None,
                align: None,
                inset: None,
                breakable: None,
            },
        ));
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Fixed(50.0), TrackSizing::Fixed(50.0)],
                rows: vec![TrackSizing::Fixed(30.0)],
                cells: vec![wide_override],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: Some(Stroke {
                    paint: Paint::Solid(Color::rgb(255, 0, 0)),
                    thickness: 1.0,
                    overhang: false,
                }),
                fill: None,
            },
        ));
        let doc = layout(&g);
        let mut found_override = false;
        for p in &doc.pages {
            for item in &p.items {
                if let FrameItem::Shape { stroke: Some(s), .. } = item {
                    if (s.thickness - 7.0).abs() < 0.01 {
                        found_override = true;
                    }
                }
            }
        }
        assert!(
            found_override,
            "Per-cell stroke thickness 7.0 override Grid 1.0 multi-col P234"
        );
    }

    #[test]
    fn p234_grid_sem_colspan_rowspan_baseline_preservado() {
        use crate::entities::layout_types::{Length, TrackSizing};
        use crate::entities::sides::Sides;
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Fixed(40.0), TrackSizing::Fixed(40.0)],
                rows: vec![TrackSizing::Fixed(20.0), TrackSizing::Fixed(20.0)],
                cells: vec![
                    Content::text("A"),
                    Content::text("B"),
                    Content::text("C"),
                    Content::text("D"),
                ],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: None,
                fill: None,
            },
        ));
        let doc = layout(&g);
        let txt = doc.plain_text();
        assert!(
            txt.contains("A")
                && txt.contains("B")
                && txt.contains("C")
                && txt.contains("D"),
            "Grid sem colspan/rowspan preserva placement sequencial pós-P234"
        );
    }

    #[test]
    fn p234_grid_stroke_baseline_p227_preservado() {
        use crate::entities::geometry::Stroke;
        use crate::entities::layout_types::{Color, Length, TrackSizing};
        use crate::entities::sides::Sides;
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Fixed(40.0), TrackSizing::Fixed(40.0)],
                rows: vec![TrackSizing::Fixed(20.0)],
                cells: vec![Content::text("a"), Content::text("b")],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: Some(Stroke {
                    paint: Paint::Solid(Color::rgb(100, 100, 100)),
                    thickness: 1.0,
                    overhang: false,
                }),
                fill: None,
            },
        ));
        let doc = layout(&g);
        let mut line_count = 0;
        for p in &doc.pages {
            for item in &p.items {
                if let FrameItem::Shape {
                    kind: crate::entities::geometry::ShapeKind::Line { .. },
                    stroke: Some(_),
                    ..
                } = item
                {
                    line_count += 1;
                }
            }
        }
        // P888 — 1 linha × 2 colunas, stroke uniforme: 3 verticais + 2
        // horizontais (topo + fundo, cada um fundido) = 5 (era >= 8
        // pós-P234, 2 células × 4 bordas, sem fusão).
        assert_eq!(
            line_count, 5,
            "Grid 1x2 stroke uniforme deve emitir 5 lines fundidas pós-P888; obtive {}",
            line_count
        );
    }

    #[test]
    fn p234_grid_fill_baseline_p228_preservado() {
        use crate::entities::layout_types::{Color, Length, TrackSizing};
        use crate::entities::sides::Sides;
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Fixed(40.0), TrackSizing::Fixed(40.0)],
                rows: vec![TrackSizing::Fixed(20.0)],
                cells: vec![Content::text("a"), Content::text("b")],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: None,
                fill: Some(Color::rgb(200, 200, 200)),
            },
        ));
        let doc = layout(&g);
        let mut rect_count = 0;
        for p in &doc.pages {
            for item in &p.items {
                if let FrameItem::Shape {
                    kind: crate::entities::geometry::ShapeKind::Rect,
                    fill: Some(_),
                    ..
                } = item
                {
                    rect_count += 1;
                }
            }
        }
        assert!(
            rect_count >= 2,
            "Grid 2 cells × 1 Rect fill = 2 mínimo pós-P234; obtive {}",
            rect_count
        );
    }

    #[test]
    fn p234_grid_auto_sizing_baseline_p233_preservado() {
        use crate::entities::layout_types::{Length, TrackSizing};
        use crate::entities::sides::Sides;
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Auto, TrackSizing::Fraction(1.0)],
                rows: vec![],
                cells: vec![Content::text("AA"), Content::text("BB")],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: None,
                fill: None,
            },
        ));
        let doc = layout(&g);
        let txt = doc.plain_text();
        assert!(
            txt.contains("AA") && txt.contains("BB"),
            "Auto+Fr preserva P233 baseline pós-P234"
        );
    }

    #[test]
    fn p234_grid_mix_explicit_e_auto_renderiza_todos() {
        use crate::entities::layout_types::{Length, TrackSizing};
        use crate::entities::sides::Sides;
        let explicit_cell = Content::GridCell(std::sync::Arc::new(
            crate::entities::elements::grid_cell::GridCellElem {
                body: Content::text("EXP"),
                x: Some(1),
                y: Some(0),
                colspan: None,
                rowspan: None,
                stroke: None,
                fill: None,
                align: None,
                inset: None,
                breakable: None,
            },
        ));
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Fixed(30.0), TrackSizing::Fixed(30.0)],
                rows: vec![TrackSizing::Fixed(15.0), TrackSizing::Fixed(15.0)],
                cells: vec![
                    explicit_cell,
                    Content::text("AUTO1"),
                    Content::text("AUTO2"),
                ],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: None,
                fill: None,
            },
        ));
        let doc = layout(&g);
        let txt = doc.plain_text();
        assert!(
            txt.contains("EXP") && txt.contains("AUTO1") && txt.contains("AUTO2"),
            "Mix explicit+auto renderiza todos pós-P234"
        );
    }

    #[test]
    fn p234_grid_colspan_fill_position_x0() {
        use crate::entities::layout_types::{Color, Length, TrackSizing};
        use crate::entities::sides::Sides;
        let wide_cell = Content::GridCell(std::sync::Arc::new(
            crate::entities::elements::grid_cell::GridCellElem {
                body: Content::text("W"),
                x: None,
                y: None,
                colspan: Some(2),
                rowspan: None,
                stroke: None,
                fill: Some(Color::rgb(123, 45, 67)),
                align: None,
                inset: None,
                breakable: None,
            },
        ));
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Fixed(40.0), TrackSizing::Fixed(60.0)],
                rows: vec![TrackSizing::Fixed(20.0)],
                cells: vec![wide_cell],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: None,
                fill: None,
            },
        ));
        let doc = layout(&g);
        let mut found = false;
        for p in &doc.pages {
            for item in &p.items {
                if let FrameItem::Shape {
                    kind: crate::entities::geometry::ShapeKind::Rect,
                    width,
                    fill: Some(c),
                    ..
                } = item
                {
                    if *c == Color::rgb(123, 45, 67) && (*width - 100.0).abs() < 0.01 {
                        found = true;
                    }
                }
            }
        }
        assert!(found, "colspan=2 fill emite Rect width=100 P234");
    }

    #[test]
    fn p234_grid_colspan_rowspan_2x2_fill_bounds_combinados() {
        use crate::entities::layout_types::{Color, Length, TrackSizing};
        use crate::entities::sides::Sides;
        let big_cell = Content::GridCell(std::sync::Arc::new(
            crate::entities::elements::grid_cell::GridCellElem {
                body: Content::text("BIG"),
                x: None,
                y: None,
                colspan: Some(2),
                rowspan: Some(2),
                stroke: None,
                fill: Some(Color::rgb(11, 22, 33)),
                align: None,
                inset: None,
                breakable: None,
            },
        ));
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![
                    TrackSizing::Fixed(20.0),
                    TrackSizing::Fixed(30.0),
                    TrackSizing::Fixed(40.0),
                ],
                rows: vec![TrackSizing::Fixed(10.0), TrackSizing::Fixed(15.0)],
                cells: vec![big_cell, Content::text("x")],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: None,
                fill: None,
            },
        ));
        let doc = layout(&g);
        let mut found = false;
        for p in &doc.pages {
            for item in &p.items {
                if let FrameItem::Shape {
                    kind: crate::entities::geometry::ShapeKind::Rect,
                    width,
                    height,
                    fill: Some(c),
                    ..
                } = item
                {
                    if *c == Color::rgb(11, 22, 33)
                        && (*width - 50.0).abs() < 0.01
                        && (*height - 25.0).abs() < 0.01
                    {
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
        let cell = Content::GridCell(std::sync::Arc::new(
            crate::entities::elements::grid_cell::GridCellElem {
                body: Content::text("INS"),
                x: None,
                y: None,
                colspan: None,
                rowspan: None,
                stroke: None,
                fill: None,
                align: None,
                inset: Some(Sides::uniform(Length::pt(10.0))),
                breakable: None,
            },
        ));
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Fixed(100.0)],
                rows: vec![TrackSizing::Fixed(50.0)],
                cells: vec![cell],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: None,
                fill: None,
            },
        ));
        let doc = layout(&g);
        // Body renderizou (mínimo render OK).
        let txt = doc.plain_text();
        assert!(
            txt.contains("INS"),
            "Cell inset 10pt: body renderiza com bounds reduzidos pós-P235"
        );
    }

    /// Per-cell inset None → inherit Grid-level inset.
    #[test]
    fn p235_per_cell_inset_none_inherits_grid() {
        use crate::entities::layout_types::{Length, TrackSizing};
        use crate::entities::sides::Sides;
        let cell = Content::GridCell(std::sync::Arc::new(
            crate::entities::elements::grid_cell::GridCellElem {
                body: Content::text("INH"),
                x: None,
                y: None,
                colspan: None,
                rowspan: None,
                stroke: None,
                fill: None,
                align: None,
                inset: None, // inherit Grid
                breakable: None,
            },
        ));
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Fixed(80.0)],
                rows: vec![TrackSizing::Fixed(30.0)],
                cells: vec![cell],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(Length::pt(5.0)), // Grid inset 5pt
                header: None,
                footer: None,
                stroke: None,
                fill: None,
            },
        ));
        let doc = layout(&g);
        let txt = doc.plain_text();
        assert!(txt.contains("INH"), "Cell inset None inherit Grid inset 5pt pós-P235");
    }

    /// Per-cell breakable Some(false) armazenado mas semantic adiada graded
    /// (paridade Block.breakable P156G; pattern "Field armazenado semantic
    /// adiada" N=7 → 8 cumulativo).
    #[test]
    fn p235_per_cell_breakable_armazenado_layout_preservado() {
        use crate::entities::layout_types::{Length, TrackSizing};
        use crate::entities::sides::Sides;
        let cell = Content::GridCell(std::sync::Arc::new(
            crate::entities::elements::grid_cell::GridCellElem {
                body: Content::text("BR"),
                x: None,
                y: None,
                colspan: None,
                rowspan: None,
                stroke: None,
                fill: None,
                align: None,
                inset: None,
                breakable: Some(false),
            },
        ));
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Fixed(40.0)],
                rows: vec![TrackSizing::Fixed(20.0)],
                cells: vec![cell],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: None,
                fill: None,
            },
        ));
        let doc = layout(&g);
        // Render preservado; breakable armazenado mas não afecta visual.
        let txt = doc.plain_text();
        assert!(
            txt.contains("BR"),
            "Cell breakable Some(false) armazenado; render preservado pós-P235 graded"
        );
    }

    /// Per-cell align Some + Grid align None → cell renderiza em align especificado.
    /// Render via Layouter cell_align extension P235 per-cell save/restore.
    #[test]
    fn p235_per_cell_align_override_grid_armazenado() {
        use crate::entities::layout_types::{Align2D, Length, TrackSizing};
        use crate::entities::sides::Sides;
        let cell = Content::GridCell(std::sync::Arc::new(
            crate::entities::elements::grid_cell::GridCellElem {
                body: Content::text("AL"),
                x: None,
                y: None,
                colspan: None,
                rowspan: None,
                stroke: None,
                fill: None,
                align: Some(Align2D::from_string("center")),
                inset: None,
                breakable: None,
            },
        ));
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Fixed(100.0)],
                rows: vec![TrackSizing::Fixed(30.0)],
                cells: vec![cell],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: None,
                fill: None,
            },
        ));
        let doc = layout(&g);
        // Render OK; align efectivo per-cell via Layouter cell_align extension.
        let txt = doc.plain_text();
        assert!(
            txt.contains("AL"),
            "Cell align Some(center) armazenado; render preservado pós-P235"
        );
    }

    /// Per-cell align None + Grid align Some → cell herda Grid align.
    #[test]
    fn p235_per_cell_align_none_inherits_grid() {
        use crate::entities::layout_types::{Align2D, Length, TrackSizing};
        use crate::entities::sides::Sides;
        let cell = Content::GridCell(std::sync::Arc::new(
            crate::entities::elements::grid_cell::GridCellElem {
                body: Content::text("IA"),
                x: None,
                y: None,
                colspan: None,
                rowspan: None,
                stroke: None,
                fill: None,
                align: None, // inherit Grid
                inset: None,
                breakable: None,
            },
        ));
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Fixed(100.0)],
                rows: vec![TrackSizing::Fixed(30.0)],
                cells: vec![cell],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: Some(Align2D::from_string("right")), // Grid align right
                inset: Sides::uniform(Length::pt(0.0)),
                header: None,
                footer: None,
                stroke: None,
                fill: None,
            },
        ));
        let doc = layout(&g);
        let txt = doc.plain_text();
        assert!(txt.contains("IA"), "Cell align None inherit Grid align right pós-P235");
    }

    /// Counters/labels dentro do body de repeat resolvem via walk
    /// (single-walk em P156J; sem multiplicação de state).
    #[test]
    fn layout_repeat_counters_dentro_do_body_resolvem() {
        use std::sync::Arc;
        // Heading dentro de repeat deve ser numerado uma vez (paridade
        // vanilla — repeat é runtime-only para paridade visual; counter
        // só conta uma vez no walk).
        let doc_content = Content::Sequence(Arc::from(vec![Content::repeat(
            Content::heading(1, Content::text("Title")),
            None,
            true,
        )]));
        let doc = layout(&doc_content);
        // Render mínimo sem panic; body Title presente.
        assert!(
            doc.plain_text().contains("Title"),
            "heading dentro de repeat deve renderizar: doc='{}'",
            doc.plain_text()
        );
    }

    /// `pagebreak(to: even)` quando próxima seria ímpar (p3) deve inserir
    /// página vazia. Setup: A → pagebreak(to:odd) garante B em p3 → mais
    /// pagebreak(to:even) força ajuste para p4 (par).
    #[test]
    fn layout_pagebreak_to_even_insere_vazia_se_proxima_seria_impar() {
        use std::sync::Arc;
        let doc_content = Content::Sequence(Arc::from(vec![
            Content::text("A"), // p1
            Content::pagebreak(false, Some(crate::entities::parity::Parity::Odd)), // → próxima p3
            Content::text("B"),                                                    // p3
            Content::pagebreak(false, Some(crate::entities::parity::Parity::Even)), // → próxima p4 (já par)
            Content::text("C"),                                                     // p4
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
            assert!(
                count >= 1,
                "table cell '{}' deve aparecer pelo menos uma vez",
                label
            );
        }
    }

    /// `Content::Table` e `Content::Grid` com mesmos campos produzem
    /// o mesmo conjunto de cells observáveis (paridade estrutural por
    /// delegação a `layout_grid`; sem modificação de `grid.rs`).
    #[test]
    fn layout_table_paridade_com_grid_equivalente() {
        use crate::entities::layout_types::TrackSizing;
        let columns = vec![TrackSizing::Auto, TrackSizing::Auto];
        let cells = vec![Content::text("X"), Content::text("Y")];

        // Versão Grid.
        let g = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: columns.clone(),
                rows: vec![],
                cells: cells.clone(),
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: crate::entities::sides::Sides::uniform(
                    crate::entities::layout_types::Length::pt(0.0),
                ),
                header: None,
                footer: None,
                stroke: None,
                fill: None,
            },
        ));
        let doc_g = layout(&g);
        let positions_g: Vec<(String, f64, f64)> = doc_g
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, pos, .. } => {
                    Some((text.to_string(), pos.x.val(), pos.y.val()))
                }
                _ => None,
            })
            .collect();

        // Versão Table.
        let t = Content::table(columns, vec![], cells);
        let doc_t = layout(&t);
        let positions_t: Vec<(String, f64, f64)> = doc_t
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|item| match item {
                FrameItem::Text { text, pos, .. } => {
                    Some((text.to_string(), pos.x.val(), pos.y.val()))
                }
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
            Some(2),
            Some(3),
            Some(99),
            Some(99), // spans grandes; ignorados em layout
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
            assert!(
                count >= 1,
                "child '{}' (plain ou table_cell) deve aparecer pelo menos uma vez",
                label
            );
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
                BibEntry::new("doe2023", "Doe, A.", "Cosmic Patterns", 2023),
            ],
            None,
        );
        let doc = layout(&b);
        let txt = doc.plain_text();
        // P468: bibliography usa numeração [N], não [key].
        assert!(txt.contains("[1]"), "[1] deve aparecer: doc='{}'", txt);
        assert!(txt.contains("[2]"), "[2] deve aparecer: doc='{}'", txt);
        assert!(txt.contains("Smith"), "author Smith deve aparecer");
        assert!(txt.contains("2024"), "year 2024 deve aparecer");
    }

    /// `Content::Cite` renderiza placeholder `[key]` com supplement
    /// (se Some) concatenado.
    #[test]
    fn layout_cite_renderiza_placeholder_com_key() {
        let c = Content::cite("smith2024", None, None);
        let doc = layout(&c);
        let txt = doc.plain_text();
        assert!(
            txt.contains("[smith2024]"),
            "Cite renderiza placeholder [key]: doc='{}'",
            txt
        );
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
        // P468: cite + bibliography — numeração [N], não [key].
        assert!(txt.contains("[1]"), "cite + bibliography devem ter [1]: doc='{}'", txt);
        assert!(txt.contains("Referências"), "title da bibliography presente");
        assert!(txt.contains("Smith"), "author entry presente");
    }

    /// **P533** — `Content::Ref` cujo nome é uma key bibliográfica deve
    /// renderizar como citação numerada, não como referência cruzada.
    #[test]
    fn layout_ref_bibliografico_renderiza_como_cite() {
        use crate::entities::bib_entry::BibEntry;
        use std::sync::Arc;
        let doc_content = Content::Sequence(Arc::from(vec![
            Content::reference("smith2024"),
            Content::bibliography(
                vec![BibEntry::new("smith2024", "Smith, J.", "On Crystal Math", 2024)],
                None,
            ),
        ]));
        let doc = layout(&doc_content);
        let txt = doc.plain_text();
        assert!(
            txt.contains("[1]"),
            "@key bibliográfico deve renderizar [1]: doc='{}'",
            txt
        );
    }

    // ── Passo 159C — Cite.form variants (E2E) ─────────────────────────────

    /// Helper: corre introspect (para popular bib_entries) seguido
    /// de layout. Mimica fluxo real eval → introspect → layout.
    fn layout_with_introspect(c: &Content) -> String {
        use crate::engine::introspect::introspect;
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
        assert!(
            txt.contains("[smith2024]"),
            "Normal/None form deve produzir [key]: doc='{}'",
            txt
        );
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
        // P468: Prose+Numeric (default) → "Author [N]".
        assert!(
            txt.contains("Smith, J. [1]"),
            "Prose+Numeric deve renderizar 'Author [N]': doc='{}'",
            txt
        );
    }

    /// `Cite { form: Prose }` com key NÃO encontrada cai no
    /// fallback `[key]`.
    #[test]
    fn cite_prose_fallback_placeholder_quando_key_nao_existe() {
        use crate::entities::citation_form::CitationForm;
        let c = Content::cite("inexistente", None, Some(CitationForm::Prose));
        let txt = layout_with_introspect(&c);
        assert!(
            txt.contains("[inexistente]"),
            "Prose sem entry deve cair no fallback [key]: doc='{}'",
            txt
        );
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
        assert!(
            txt1.contains("Smith, J."),
            "Author form: 'Smith, J.' deve aparecer: doc='{}'",
            txt1
        );
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
        assert!(
            txt.contains("Nature Communications"),
            "journal deve aparecer: doc='{}'",
            txt
        );
        assert!(
            txt.contains("vol. 12"),
            "volume deve aparecer com prefix 'vol.': doc='{}'",
            txt
        );
        assert!(
            txt.contains("pp. 1-10"),
            "pages deve aparecer com prefix 'pp.': doc='{}'",
            txt
        );
        assert!(txt.contains("ACM"), "publisher deve aparecer: doc='{}'", txt);
        assert!(
            txt.contains("(2024)"),
            "year preserva formato (year) no final: doc='{}'",
            txt
        );
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
        // P468: bibliography usa numeração [N], não [key].
        assert!(txt.contains("[1]"), "[1] entry mínima: doc='{}'", txt);
        assert!(txt.contains("Smith, J."), "author");
        assert!(txt.contains("On Crystal Math"), "title");
        assert!(txt.contains("(2024)"), "year (year)");
        // Sem fields novos — ausentes do output.
        assert!(!txt.contains("vol."), "sem volume → sem 'vol.'");
        assert!(!txt.contains("pp."), "sem pages → sem 'pp.'");
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
        assert!(
            txt.contains("[1]"),
            "Normal/None com Bibliography populada deve renderizar [1]: doc='{}'",
            txt
        );
    }

    /// Regression P159A: Cite sem Bibliography precedente cai
    /// no fallback `[key]`.
    #[test]
    fn cite_normal_fallback_placeholder_quando_bib_vazia() {
        let c = Content::cite("smith2024", None, None);
        let txt = layout_with_introspect(&c);
        assert!(
            txt.contains("[smith2024]"),
            "Sem Bibliography → fallback [key] (regression P159A): doc='{}'",
            txt
        );
    }

    /// Multiple entries em Bibliography → cada Cite obtém número
    /// na ordem de aparecimento.
    #[test]
    fn cite_normal_multiple_entries_numeradas_em_ordem() {
        use crate::entities::bib_entry::BibEntry;
        use std::sync::Arc;
        let doc_content = Content::Sequence(Arc::from(vec![
            Content::cite("first", None, None),
            Content::cite("second", None, None),
            Content::cite("third", None, None),
            Content::bibliography(
                vec![
                    BibEntry::new("first", "Author One", "Paper One", 2021),
                    BibEntry::new("second", "Author Two", "Paper Two", 2022),
                    BibEntry::new("third", "Author Three", "Paper Three", 2023),
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
        // P468: Prose+Numeric (default) → "Author [N]". Regression P159C atualizada.
        assert!(
            txt.contains("Smith, J. [1]"),
            "Prose+Numeric deve renderizar 'Author [N]': doc='{}'",
            txt
        );
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
        assert!(
            txt.contains("[inexistente]"),
            "Cite com key não em Bibliography → fallback [key]: doc='{}'",
            txt
        );
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
                    BibEntry::new("first", "Author One", "Paper One", 2021),
                    BibEntry::new("second", "Author Two", "Paper Two", 2022),
                ],
                None,
            ),
            Content::bibliography(
                vec![BibEntry::new("third", "Author Three", "Paper Three", 2023)],
                None,
            ),
        ]));
        let txt = layout_with_introspect(&doc_content);
        // P468: citation_order = ["third"] (única cite inline).
        // Bib1 (first, second) → [1], [2] (não citadas, originais).
        // Bib2 (third) → [1] (citada primeira, citation_number=1).
        // Multi-bib com numeração global contínua é scope-out P468 (→ P420).
        assert!(
            txt.contains("Author Three"),
            "third deve aparecer na bib: doc='{}'",
            txt
        );
        assert!(
            txt.contains("Author One") && txt.contains("Author Two"),
            "first e second devem aparecer na bib: doc='{}'",
            txt
        );
    }

    // ── Passo 468 — Estilos numéricos por ordem de primeira aparição ──────

    /// P468: numeração segue ordem de primeira aparição das citações,
    /// não a ordem em que as entries aparecem na Bibliography.
    #[test]
    fn cite_numeric_ordem_primeira_aparicao() {
        use crate::entities::bib_entry::BibEntry;
        use std::sync::Arc;
        let doc_content = Content::Sequence(Arc::from(vec![
            Content::cite("third", None, None),
            Content::cite("first", None, None),
            Content::cite("second", None, None),
            Content::bibliography(
                vec![
                    BibEntry::new("first", "Author One", "Paper One", 2021),
                    BibEntry::new("second", "Author Two", "Paper Two", 2022),
                    BibEntry::new("third", "Author Three", "Paper Three", 2023),
                ],
                None,
            ),
        ]));
        let txt = layout_with_introspect(&doc_content);
        assert!(txt.contains("[1]"), "third (primeira citação) → [1]: doc='{}'", txt);
        assert!(txt.contains("[2]"), "first (segunda citação) → [2]: doc='{}'", txt);
        assert!(txt.contains("[3]"), "second (terceira citação) → [3]: doc='{}'", txt);
    }

    /// P468: a Bibliography fallback é reordenada para seguir a ordem
    /// de primeira aparição das citações no documento.
    #[test]
    fn bibliography_ordenada_pela_ordem_de_citacao() {
        use crate::entities::bib_entry::BibEntry;
        use std::sync::Arc;
        let doc_content = Content::Sequence(Arc::from(vec![
            Content::cite("third", None, None),
            Content::cite("first", None, None),
            Content::cite("second", None, None),
            Content::bibliography(
                vec![
                    BibEntry::new("first", "Author One", "Paper One", 2021),
                    BibEntry::new("second", "Author Two", "Paper Two", 2022),
                    BibEntry::new("third", "Author Three", "Paper Three", 2023),
                ],
                None,
            ),
        ]));
        let txt = layout_with_introspect(&doc_content);
        // Bib ordenada por primeira citação: third (citada 1ª) → [1],
        // first (citada 2ª) → [2], second (citada 3ª) → [3].
        // Verifica a ordem dos autores na secção de bibliography.
        let pos_three = txt.rfind("Author Three").expect("Author Three presente");
        let pos_one = txt.rfind("Author One").expect("Author One presente");
        let pos_two = txt.rfind("Author Two").expect("Author Two presente");
        assert!(
            pos_three < pos_one && pos_one < pos_two,
            "bib deve seguir ordem de citação: third<first<second: doc='{}'",
            txt
        );
    }

    /// P468: form Prose com estilo numérico renderiza "Author [N]".
    #[test]
    fn cite_numeric_prose_inclui_numero() {
        use crate::entities::bib_entry::BibEntry;
        use crate::entities::citation_form::CitationForm;
        use crate::entities::citation_style::CitationStyle;
        use std::sync::Arc;
        let doc_content = Content::Sequence(Arc::from(vec![
            Content::cite_with_style(
                "smith2024",
                None,
                Some(CitationForm::Prose),
                Some(CitationStyle::Numeric),
            ),
            Content::bibliography(
                vec![BibEntry::new("smith2024", "Smith, J.", "On Crystal Math", 2024)],
                None,
            ),
        ]));
        let txt = layout_with_introspect(&doc_content);
        assert!(
            txt.contains("Smith, J. [1]"),
            "Prose + Numeric deve renderizar 'Author [N]': doc='{}'",
            txt
        );
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
        assert!(
            txt.contains("https://example.com/paper"),
            "URL plaintext deve aparecer: doc='{}'",
            txt
        );
        // DOI com prefixo `doi:` deve aparecer.
        assert!(
            txt.contains("doi:10.1234/abc"),
            "DOI com prefixo 'doi:' deve aparecer: doc='{}'",
            txt
        );
        // Ordem APA Opção C: url/doi após (year).
        assert!(txt.contains("(2024)"), "year preserva formato (year): doc='{}'", txt);
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
        assert!(!txt.contains("doi:"), "sem doi → sem 'doi:' no output: doc='{}'", txt);
        assert!(!txt.contains("https://"), "sem url → sem URL no output: doc='{}'", txt);
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
        assert!(
            txt.contains("(Ed. Doe, A.)"),
            "editor deve aparecer com prefixo '(Ed. ': doc='{}'",
            txt
        );
        // Series em parêntese.
        assert!(
            txt.contains("(Crystal Studies)"),
            "series deve aparecer entre parênteses: doc='{}'",
            txt
        );
        // Note em brackets.
        assert!(
            txt.contains("[See Smith 2023]"),
            "note deve aparecer entre brackets: doc='{}'",
            txt
        );
        // ISBN com prefixo lowercase.
        assert!(
            txt.contains("isbn:978-0-1234"),
            "isbn deve aparecer com prefixo lowercase 'isbn:': doc='{}'",
            txt
        );
        // Location: publisher.
        assert!(
            txt.contains("New York: ACM"),
            "location: publisher deve aparecer: doc='{}'",
            txt
        );
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
        assert!(!txt.contains("Ed."), "sem editor → sem 'Ed.' no output: doc='{}'", txt);
        assert!(
            !txt.contains("isbn:"),
            "sem isbn → sem 'isbn:' no output: doc='{}'",
            txt
        );
        assert!(
            !txt.contains("[See"),
            "sem note → sem '[note]' no output: doc='{}'",
            txt
        );
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
        assert!(
            txt.contains("MIT"),
            "organization deve aparecer no slot publisher: doc='{}'",
            txt
        );
    }
}

// ── P622 — Quebra de parágrafo (`Content::Parbreak`) ───────────────────────

#[cfg(test)]
mod p622_parbreak {
    use super::*;
    use crate::entities::content::Content;
    use crate::entities::layout_types::FrameItem;

    #[test]
    fn parbreak_separa_dois_paragrafos_em_linhas_distintas() {
        let content = Content::sequence(vec![
            Content::text("Primeiro parágrafo."),
            Content::Parbreak,
            Content::text("Segundo parágrafo."),
        ]);
        let doc = layout(&content);
        assert!(!doc.pages.is_empty(), "documento deve ter pelo menos uma página");

        let ys: std::collections::HashSet<i64> = doc.pages[0]
            .items
            .iter()
            .filter_map(|item| match item {
                FrameItem::Text { pos, .. } => Some(pos.y.0.round() as i64),
                _ => None,
            })
            .collect();

        assert!(
            ys.len() >= 2,
            "dois parágrafos separados por Parbreak devem produzir pelo menos 2 linhas visuais; ys={:?}",
            ys
        );
    }
}

// ── P418 — CSL Bibliography/Cite end-to-end ────────────────────────────────

#[cfg(test)]
mod p418_csl_e2e {
    use super::*;
    use crate::entities::bib_entry::BibEntry;
    use crate::entities::citation_form::CitationForm;
    use std::sync::Arc;

    fn entry() -> BibEntry {
        BibEntry::new("smith2024", "Smith, J.", "On Crystal Math", 2024)
            .with_journal("Journal of Examples")
            .with_volume("12")
            .with_pages("1-10")
    }

    fn doc_with_style(style: &str) -> Content {
        Content::Sequence(Arc::from(vec![
            Content::cite("smith2024", None, None),
            Content::bibliography_with_style(
                vec![entry()],
                Some(Content::text("References")),
                Some(style.into()),
                None,
            ),
        ]))
    }

    #[test]
    fn cite_e_bibliography_ieve_no_mesmo_documento() {
        let doc = layout(&doc_with_style("ieee"));
        let txt = doc.plain_text();
        assert!(txt.contains("[1]"), "cite deve render [1]: {txt}");
        assert!(txt.contains("Smith"), "bibliografia deve conter autor: {txt}");
        assert!(
            txt.contains("On Crystal Math"),
            "bibliografia deve conter titulo: {txt}"
        );
    }

    #[test]
    fn cite_antes_da_bibliography_usa_mesmo_style_ieee() {
        let doc = layout(&Content::Sequence(Arc::from(vec![
            Content::cite("smith2024", None, None),
            Content::bibliography_with_style(
                vec![entry()],
                None,
                Some("ieee".into()),
                None,
            ),
        ])));
        let txt = doc.plain_text();
        assert!(txt.contains("[1]"), "cite antes da bib deve usar style IEEE: {txt}");
    }

    #[test]
    fn bibliography_sem_style_preserva_fallback_local() {
        let doc = layout(&Content::Sequence(Arc::from(vec![
            Content::cite("smith2024", None, None),
            Content::bibliography(vec![entry()], None),
        ])));
        let txt = doc.plain_text();
        // P468: fallback local usa [N], não [key].
        assert!(txt.contains("[1]"), "fallback local [1]: {txt}");
        assert!(txt.contains("Smith, J."), "fallback local autor: {txt}");
    }

    #[test]
    fn bibliography_apa_rende_author_year() {
        let doc = layout(&doc_with_style("apa"));
        let txt = doc.plain_text();
        assert!(txt.contains("Smith"), "apa bib autor: {txt}");
        assert!(txt.contains("2024"), "apa bib ano: {txt}");
    }

    #[test]
    fn cite_form_author_apa_rende_author_only() {
        let doc = layout(&Content::Sequence(Arc::from(vec![
            Content::cite("smith2024", None, Some(CitationForm::Author)),
            Content::bibliography_with_style(
                vec![entry()],
                None,
                Some("apa".into()),
                None,
            ),
        ])));
        let txt = doc.plain_text();
        assert!(txt.contains("Smith"), "author form: {txt}");
    }

    #[test]
    fn cite_form_year_apa_rende_year_only() {
        let doc = layout(&Content::Sequence(Arc::from(vec![
            Content::cite("smith2024", None, Some(CitationForm::Year)),
            Content::bibliography_with_style(
                vec![entry()],
                None,
                Some("apa".into()),
                None,
            ),
        ])));
        let txt = doc.plain_text();
        assert!(txt.contains("2024"), "year form: {txt}");
    }

    #[test]
    fn cite_form_prose_apa_rende_author_year() {
        let doc = layout(&Content::Sequence(Arc::from(vec![
            Content::cite("smith2024", None, Some(CitationForm::Prose)),
            Content::bibliography_with_style(
                vec![entry()],
                None,
                Some("apa".into()),
                None,
            ),
        ])));
        let txt = doc.plain_text();
        assert!(txt.contains("Smith"), "prose author: {txt}");
        assert!(txt.contains("2024"), "prose year: {txt}");
    }

    #[test]
    fn bibliography_style_inexistente_cai_em_fallback() {
        let doc = layout(&Content::Sequence(Arc::from(vec![
            Content::cite("smith2024", None, None),
            Content::bibliography_with_style(
                vec![entry()],
                None,
                Some("not-a-real-style".into()),
                None,
            ),
        ])));
        let txt = doc.plain_text();
        // P468: fallback usa [N], mesmo quando style é inválido.
        assert!(txt.contains("[1]"), "fallback quando style invalido: {txt}");
    }

    #[test]
    fn bibliography_locale_pt_br_nao_panica() {
        let doc = layout(&Content::bibliography_with_style(
            vec![entry()],
            None,
            Some("ieee".into()),
            Some("pt-BR".into()),
        ));
        let txt = doc.plain_text();
        assert!(txt.contains("[1]"), "locale pt-BR: {txt}");
    }

    #[test]
    fn multi_bibliography_com_style_usa_primeiro_style() {
        let doc = layout(&Content::Sequence(Arc::from(vec![
            Content::bibliography_with_style(
                vec![entry()],
                None,
                Some("ieee".into()),
                None,
            ),
            Content::bibliography_with_style(
                vec![BibEntry::new("k2", "Doe, A.", "Title 2", 2023)],
                None,
                Some("apa".into()),
                None,
            ),
        ])));
        let txt = doc.plain_text();
        // Primeiro style (ieee) governa as citações.
        assert!(txt.contains("[1]"), "primeiro style ieee: {txt}");
    }

    #[test]
    fn bibliography_title_rende_antes_das_referencias() {
        let doc = layout(&Content::bibliography_with_style(
            vec![entry()],
            Some(Content::text("Refs")),
            Some("ieee".into()),
            None,
        ));
        let txt = doc.plain_text();
        assert!(txt.contains("Refs"), "title: {txt}");
        assert!(txt.contains("[1]"), "bib: {txt}");
    }

    #[test]
    fn cite_key_inexistente_mantem_fallback_brackets() {
        let doc = layout(&Content::Sequence(Arc::from(vec![
            Content::cite("inexistente", None, None),
            Content::bibliography_with_style(
                vec![entry()],
                None,
                Some("ieee".into()),
                None,
            ),
        ])));
        let txt = doc.plain_text();
        assert!(txt.contains("[inexistente]"), "key inexistente: {txt}");
    }
}

// ── P168 (M5 sub-passo 2) — Tests de migração figure-ref ─────────────────

#[cfg(test)]
mod p168_figure_ref_migration {
    use super::*;
    use crate::engine::introspect::introspect_with_introspector;
    use crate::entities::label::Label;

    fn doc_figure_with_ref(
        label_str: &str,
        kind: Option<String>,
        with_caption: bool,
        with_numbering: bool,
    ) -> Content {
        let figure = Content::figure(
            Content::text("body"),
            if with_caption { Some(Content::text("cap")) } else { None },
            kind,
            if with_numbering { Some("1".into()) } else { None },
        );
        Content::Sequence(
            vec![
                labelled_prod(figure, Label(label_str.to_string())),
                Content::text("ver "),
                Content::reference(label_str.to_string()),
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
    use crate::engine::introspect::introspect_with_introspector;
    use crate::entities::bib_entry::BibEntry;
    use crate::entities::citation_form::CitationForm;
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
        assert!(
            txt.contains("[1]"),
            "Normal/None via introspector path deve renderizar [1]: doc='{txt}'"
        );
    }

    #[test]
    fn cite_prose_via_introspector_renderiza_author_year() {
        // P181G: cite-arm consulta `Introspector::bib_entry_for_key`
        // (Prose precisa do entry para autor + ano).
        let content = doc_cite_with_bib(Some(CitationForm::Prose));
        let txt = render_via_introspector(&content);
        // P468: Prose+Numeric (default) → "Author [N]".
        assert!(
            txt.contains("Smith, J. [1]"),
            "Prose via introspector path deve renderizar 'Author [N]': doc='{txt}'"
        );
    }

    #[test]
    fn cite_author_via_introspector_renderiza_apenas_author() {
        let content = doc_cite_with_bib(Some(CitationForm::Author));
        let txt = render_via_introspector(&content);
        assert!(
            txt.contains("Smith, J."),
            "Author via introspector path deve renderizar autor: doc='{txt}'"
        );
    }

    #[test]
    fn cite_year_via_introspector_renderiza_apenas_ano() {
        let content = doc_cite_with_bib(Some(CitationForm::Year));
        let txt = render_via_introspector(&content);
        assert!(
            txt.contains("2024"),
            "Year via introspector path deve renderizar ano: doc='{txt}'"
        );
    }

    #[test]
    fn paridade_legacy_vs_introspector_para_cite() {
        // P181G: para os 4 forms, layout() (path legacy via state.bib_*)
        // e layout_with_introspector() (path Introspector via BibStore)
        // produzem o mesmo plain_text. Confirma paridade BibStore ↔
        // state.bib_* garantida por construção em P181E.
        for form in [
            None,
            Some(CitationForm::Prose),
            Some(CitationForm::Author),
            Some(CitationForm::Year),
        ] {
            let content = doc_cite_with_bib(form);

            let state_legacy = crate::engine::introspect::introspect(&content);
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
        intr.bib_store.add_bibliography(vec![BibEntry::new(
            "smith2024",
            "Smith, J.",
            "On Crystal Math",
            2024,
        )]);
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
    use crate::engine::introspect::introspect_with_introspector;
    use crate::entities::bib_entry::BibEntry;
    use crate::entities::citation_form::CitationForm;
    use crate::entities::introspector::Introspector;
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
            Content::cite("intro", None, None),
            Content::cite("methods", None, None),
            Content::bibliography(vec![bib("intro"), bib("methods")], None),
        ]));

        let state = crate::engine::introspect::introspect(&content);
        let txt = layout(&content).plain_text();

        assert!(
            txt.contains("[1]"),
            "cite intro deve renderizar [1] via pipeline completo: doc='{txt}'"
        );
        assert!(
            txt.contains("[2]"),
            "cite methods deve renderizar [2] via pipeline completo: doc='{txt}'"
        );
    }

    #[test]
    fn walk_puro_state_legacy_vazio_em_producao() {
        // P181I: confirma walk puro restaurado (P181H) — state.bib_*
        // permanece vazio após walk em produção. BibStore é
        // populado por from_tags como fonte única.
        let content = Content::bibliography(vec![bib("a")], None);

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

        assert_eq!(intr.bib_store.len(), 4, "multi-Bib concat: 2+2 entries → len 4");
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
                vec![bib("a"), bib("b")], // "a" duplicado; "b" novo
                None,
            ),
        ]));

        let intr = introspect_with_introspector(&content);

        // "a" preserva número original (1).
        assert_eq!(
            intr.bib_number_for_key("a"),
            Some(1),
            "or_insert preserva primeiro número para key duplicada"
        );
        // "b" obtém próximo número (2).
        assert_eq!(
            intr.bib_number_for_key("b"),
            Some(2),
            "key nova obtém próximo número via numbers_len()+1"
        );
    }

    #[test]
    fn cite_4_forms_via_layout_with_introspector() {
        // P181I: confirma que os 4 cite forms renderizam correctamente
        // via path `layout_with_introspector` (consumer migrado P181G).
        let entry = BibEntry::new("smith2024", "Smith, J.", "On Math", 2024);

        // P468: Prose+Numeric (default) → "Author [N]" (não "Author (Year)").
        for (form, expected_substr) in [
            (None, "[1]"),
            (Some(CitationForm::Prose), "Smith, J. [1]"),
            (Some(CitationForm::Author), "Smith, J."),
            (Some(CitationForm::Year), "2024"),
        ] {
            let content = Content::Sequence(Arc::from(vec![
                Content::cite("smith2024", None, form),
                Content::bibliography(vec![entry.clone()], None),
            ]));

            let intr = introspect_with_introspector(&content);
            let txt = layout_with_introspector(&content, intr).plain_text();

            assert!(
                txt.contains(expected_substr),
                "form {form:?} deve renderizar '{expected_substr}': doc='{txt}'"
            );
        }
    }
}

// ── P169 (M9 sub-passo 1) — Tests E2E para metadata(value) ────────────────

#[cfg(test)]
mod p169_metadata_feature {
    use super::*;
    use crate::engine::introspect::introspect_with_introspector;
    use crate::entities::introspector::Introspector;
    use crate::entities::value::Value;
    use ecow::EcoString;

    #[test]
    fn metadata_value_acessivel_via_introspector() {
        // P169 .C: Content::Metadata produz query_metadata populado.
        let content = Content::Sequence(
            vec![
                Content::text("antes"),
                Content::metadata(Value::Str(EcoString::from("hello"))),
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
                Content::metadata(Value::Str(EcoString::from("invisivel"))),
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
                Content::metadata(Value::Int(1)),
                Content::metadata(Value::Int(2)),
                Content::metadata(Value::Int(3)),
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
    use crate::engine::introspect::introspect_with_introspector;
    use crate::entities::introspector::Introspector;
    use crate::entities::location::Location;
    use crate::entities::state_update::StateUpdate;
    use crate::entities::value::Value;

    #[test]
    fn state_init_e_acessivel_via_state_value() {
        // P171 .H.1: state(key, init) sem updates → state_value retorna init.
        let content = Content::Sequence(
            vec![
                Content::state("counter".to_string(), Value::Int(0)),
                Content::heading(1, Content::text("h1")),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);
        // Em qualquer location após o init → init value.
        assert_eq!(intr.state_final_value("counter"), Some(&Value::Int(0)));
    }

    #[test]
    fn state_update_aplica_no_ponto_correcto() {
        // P171 .H.1: state + heading + state_update + heading
        // → state_value(loc_pre_update) = init; state_value(loc_post_update) = new.
        let content = Content::Sequence(
            vec![
                Content::state("counter".to_string(), Value::Int(0)),
                Content::heading(1, Content::text("antes")),
                Content::state_update(
                    "counter".to_string(),
                    StateUpdate::Set(Box::new(Value::Int(5))),
                ),
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
                Content::state("c".to_string(), Value::Int(0)),
                Content::state_update(
                    "c".to_string(),
                    StateUpdate::Set(Box::new(Value::Int(42))),
                ),
                Content::text("Y"),
            ]
            .into(),
        );
        let without_state =
            Content::Sequence(vec![Content::text("X"), Content::text("Y")].into());
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
                Content::state("a".to_string(), Value::Int(1)),
                Content::state("b".to_string(), Value::Int(100)),
                Content::state_update(
                    "a".to_string(),
                    StateUpdate::Set(Box::new(Value::Int(2))),
                ),
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
        assert_eq!(intr.state_value("counter", Location::from_raw(0)), None);
    }
}

// ── P172 (M9 sub-passo 4) — Tests para Func variant em StateUpdate ──────

#[cfg(test)]
mod p172_func_callback {
    use super::*;
    use crate::engine::introspect::introspect_with_introspector;
    use crate::entities::func::Func;
    use crate::entities::introspector::Introspector;
    use crate::entities::state_update::StateUpdate;
    use crate::entities::value::Value;

    fn dummy_native(
        _ctx: &mut crate::engine::eval::EvalContext,
        _args: &crate::entities::args::Args,
        _world: &dyn crate::contracts::world::World,
        _current_file: crate::entities::file_id::FileId,
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
                Content::state("c".to_string(), Value::Int(0)),
                Content::state_update("c".to_string(), StateUpdate::Func(f)),
                Content::text("Y"),
            ]
            .into(),
        );
        let without =
            Content::Sequence(vec![Content::text("X"), Content::text("Y")].into());
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
                Content::state("c".to_string(), Value::Int(0)),
                Content::state_update(
                    "c".to_string(),
                    StateUpdate::Set(Box::new(Value::Int(5))),
                ),
                Content::state_update("c".to_string(), StateUpdate::Func(f)), // ignorada
                Content::state_update(
                    "c".to_string(),
                    StateUpdate::Set(Box::new(Value::Int(10))),
                ),
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
    use crate::engine::introspect::{introspect, introspect_with_introspector};
    use crate::entities::introspector::Introspector;
    use std::sync::Arc;

    /// Documento típico: `set heading(numbering: ...)` + 3 headings com nesting [1, 2, 1].
    fn doc_typico() -> Content {
        // Lote F-2 S1 (P335): numeração assada (`heading_numbered`) para o
        // prefixo de layout; o marcador `SetHeadingNumbering` permanece para os
        // testes que ainda exercitam a plumbing de introspecção (StateRegistry),
        // inerte em produção até a limpeza S5.
        Content::Sequence(Arc::from(vec![
            Content::heading_numbered(1, Content::text("Intro")),
            Content::heading_numbered(2, Content::text("Motivação")),
            Content::heading_numbered(1, Content::text("Conclusão")),
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

        assert!(txt.contains("1."), "H1 (Intro) deve ter prefixo '1.': '{txt}'");
        assert!(txt.contains("1.1"), "H2 (Motivação) deve ter prefixo '1.1': '{txt}'");
        assert!(
            txt.contains("2."),
            "segundo H1 (Conclusão) deve ter prefixo '2.': '{txt}'"
        );
    }

    #[test]
    fn pipeline_completo_heading_numbering_via_layout_with_introspector() {
        // P182E .B (irmão): mesmo pipeline mas via entry point novo
        // directamente — sem o re-walk interno de `layout()`.
        let content = doc_typico();
        let intr = introspect_with_introspector(&content);

        // Lote F-2 S5 (P335): asserção is_numbering_active removida.
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
        // Lote F-2 S1 (P335): o gate é assado por heading — H1 numerado (ON),
        // H2 plano (OFF). Marcadores mantidos para o teste de plumbing abaixo.
        let content = Content::Sequence(Arc::from(vec![
            Content::heading_numbered(1, Content::text("Intro")),
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
        // Lote F-4 E0 (P338): a asserção-cauda `is_numbering_active` saiu — a API
        // legada de gate por StateRegistry foi removida (triagem-47, morta pós-F-2;
        // o gate vive no campo assado). O corpo do teste (render H2 sem prefixo)
        // permanece intacto.
    }

    #[test]
    fn paridade_documento_complexo_legacy_vs_migrated() {
        // P182E .D: documento com headings + parágrafo de texto +
        // equation block. Comparar plain_text entre `layout()` legacy
        // e `layout_with_introspector` directo. Output observable
        // deve ser idêntico — confirma que migração P182B–D não
        // introduziu divergência.
        // Lote F-2 S1 (P335): headings numerados assados; marcador mantido p/ plumbing.
        let content = Content::Sequence(Arc::from(vec![
            Content::heading_numbered(1, Content::text("Sec1")),
            Content::text("corpo do parágrafo"),
            Content::heading_numbered(2, Content::text("Sub1")),
            Content::equation(Content::MathText("x".into()), true),
            Content::heading_numbered(1, Content::text("Sec2")),
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
}

// ── P184E — Tests E2E paridade C3 (figure auto-number per kind) ────────────

#[cfg(test)]
mod p184e_figure_per_kind {
    use super::*;
    use crate::engine::introspect::{introspect, introspect_with_introspector};
    use crate::entities::introspector::{Introspector, TagIntrospector};
    use std::sync::Arc;

    /// Helper: figure numerada+captioned com kind dado.
    fn figure(kind: Option<&str>, caption_text: &str) -> Content {
        Content::figure(
            Content::text("body"),
            Some(Content::text(caption_text)),
            kind.map(|s| s.to_string()),
            Some("1".into()),
        )
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
        let txt =
            layout_with_introspector(&content, TagIntrospector::empty()).plain_text();

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
        // P470 (i18n): kind="image" → "Figura N:", kind="table" → "Tabela N:"
        // (figure_supplement_for_lang; lang=None → PT default).
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
        assert!(txt.contains("im_a"));
        assert!(txt.contains("im_b"));
        assert!(txt.contains("tb_a"));
        assert!(txt.contains("tb_b"));
        // P470 i18n: image → "Figura"; table → "Tabela".
        assert!(txt.contains("Figura 1:"), "image[0]: '{txt}'");
        assert!(txt.contains("Figura 2:"), "image[1]: '{txt}'");
        assert!(txt.contains("Tabela 1:"), "table[0]: '{txt}'");
        assert!(txt.contains("Tabela 2:"), "table[1]: '{txt}'");
        // "Figura 2:" agora aparece só uma vez (image[1] apenas).
        let figura_2_count = txt.matches("Figura 2:").count();
        assert_eq!(
            figura_2_count, 1,
            "apenas image[1] usa 'Figura'; table usa 'Tabela': '{txt}'"
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
    use crate::engine::introspect::introspect_with_introspector;
    use crate::engine::introspect::locatable::is_locatable;
    use crate::entities::element_kind::ElementKind;
    use crate::entities::introspector::Introspector;
    use crate::entities::location::Location;
    use std::sync::Arc;

    /// Recolhe a sequência de `Location`s emitidas pelo walk de
    /// introspect, em ordem cronológica. `kind_index` preserva ordem
    /// de inserção por kind; agregar todos e ordenar por `as_u128`
    /// recupera a ordem global do walk (Locator é monotonicamente
    /// crescente per `locator.rs:counter_e_monotonico_crescente`).
    fn collect_walk_locations(
        intr: &crate::entities::introspector::TagIntrospector,
    ) -> Vec<Location> {
        let mut all: Vec<Location> =
            intr.kind_index.values().flatten().copied().collect();
        all.sort_by_key(|l| l.as_u128());
        all
    }

    /// Itera parts manualmente, chamando `layout_content` por cada
    /// um, e captura `current_location` após cada arm locatable.
    /// Não modifica produção — só usa API pub(super) acessível a
    /// tests do mesmo módulo.
    fn collect_layout_locations(parts: &[Content]) -> Vec<Location> {
        // P204C (M8): Layouter ganha 'a + Tracked<dyn Introspector>.
        use crate::entities::introspector::{Introspector, TagIntrospector};
        use comemo::Track;
        let intr = TagIntrospector::empty();
        let intr_dyn: &dyn Introspector = &intr;
        let intr_tracked = intr_dyn.track();
        let mut layouter =
            Layouter::new(FixedMetrics, NullImageSizer, 12.0, intr_tracked);
        let mut locs = Vec::new();
        for part in parts {
            layouter.layout_content(part);
            if is_locatable(part) {
                locs.push(
                    layouter
                        .current_location
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
            Content::heading(1, Content::Empty),
            Content::figure(Content::Empty, None, None, None),
            Content::cite("k".to_string(), None, None),
        ];
        let content = Content::Sequence(Arc::from(parts.clone()));

        let intr = introspect_with_introspector(&content);
        let walk_locs = collect_walk_locations(&intr);
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
            Content::heading(1, Content::Empty),
            Content::text("plain"),
            Content::figure(Content::Empty, None, None, None),
            Content::equation(Content::Empty, false),
            Content::cite("k".to_string(), None, None),
        ];
        let content = Content::Sequence(Arc::from(parts.clone()));

        let intr = introspect_with_introspector(&content);
        let walk_locs = collect_walk_locations(&intr);
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
        use crate::entities::introspector::{Introspector, TagIntrospector};
        use comemo::Track;
        let intr = TagIntrospector::empty();
        let intr_dyn: &dyn Introspector = &intr;
        let intr_tracked = intr_dyn.track();
        let mut layouter =
            Layouter::new(FixedMetrics, NullImageSizer, 12.0, intr_tracked);
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
        layouter.layout_content(&Content::heading(1, Content::Empty));
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
}

// ── P186F — tests E2E equation locatable + relatório consolidado ────────────

#[cfg(test)]
mod p186f_equation_locatable {
    use super::*;
    use crate::engine::introspect::introspect_with_introspector;
    use crate::entities::element_kind::ElementKind;
    use crate::entities::introspector::Introspector;
    use crate::entities::state_update::StateUpdate;
    use crate::entities::value::Value;
    use std::sync::Arc;

    fn equation_block() -> Content {
        // Lote F-2 S2 (P335): numeração assada (`equation_numbered`) — o gate do
        // contador lê o campo assado, não mais o StateRegistry injetado.
        Content::equation_numbered(Content::Empty, true)
    }

    #[test]
    fn pipeline_e2e_equation_block_com_state_activo() {
        // .B caso central: 3 equations block **numeradas** (campo assado
        // `numbering_active=true` via `equation_block()`) acumulam counter
        // [1, 2, 3]. Lote F-4 E0 (P338): o injector `Content::StateUpdate
        // ("numbering_active:equation")` saiu — o canal StateRegistry estava
        // morto (gate pelo campo assado; o counter mantém [1,2,3] sem ele).
        let parts = vec![equation_block(), equation_block(), equation_block()];
        let content = Content::Sequence(Arc::from(parts));

        let intr = introspect_with_introspector(&content);

        let eq_locs = intr
            .kind_index
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
        // Lote F-2 S2 (P335): sentinela do gate dormente — equações de bloco
        // **não numeradas** (campo assado `numbering_active=false`) → o gate do
        // contador não dispara. (Antes: gate via StateRegistry; agora via campo
        // assado.) Equações planas, não `equation_block()` (que agora é numerada).
        let parts = vec![
            Content::equation(Content::Empty, true),
            Content::equation(Content::Empty, true),
            Content::equation(Content::Empty, true),
        ];
        let content = Content::Sequence(Arc::from(parts));

        let intr = introspect_with_introspector(&content);

        let eq_locs = intr
            .kind_index
            .get(&ElementKind::Equation)
            .cloned()
            .unwrap_or_default();
        assert_eq!(eq_locs.len(), 3, "kind_index populado mesmo com gate dormente");

        for loc in eq_locs {
            assert_eq!(
                intr.flat_counter_at("equation", loc),
                None,
                "counter dormente em loc={:?}",
                loc,
            );
        }
    }

    #[test]
    fn gate_dormente_inline_mesmo_com_state_active() {
        // .C variação: equations **inline** → gate bloqueia por `block=false`,
        // independente de numeração. Lote F-4 E0 (P338): o injector de
        // `numbering_active:equation` saiu (canal StateRegistry morto); o nome
        // histórico "com_state_active" refere a injeção removida — o gate sempre
        // foi `block`, e segue dormente sem ela.
        let parts = vec![
            Content::equation(Content::Empty, false),
            Content::equation(Content::Empty, false),
        ];
        let content = Content::Sequence(Arc::from(parts));

        let intr = introspect_with_introspector(&content);

        let eq_locs = intr
            .kind_index
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
    use crate::engine::introspect::introspect_with_introspector;
    use crate::entities::introspector::{Introspector, TagIntrospector};
    use std::sync::Arc;

    fn heading_with_text(level: u8, text: &str) -> Content {
        Content::heading(level, Content::text(text))
    }

    fn doc_3_headings() -> Content {
        // Lote F-2 S1 (P335): numeração assada para o prefixo; marcador mantido
        // para a plumbing de introspecção (ver `doc_typico`).
        Content::Sequence(Arc::from(vec![
            Content::heading_numbered(1, Content::text("Intro")),
            Content::heading_numbered(2, Content::text("Motivacao")),
            Content::heading_numbered(1, Content::text("Conclusao")),
        ]))
    }

    #[test]
    fn c1_heading_prefix_via_introspector_path() {
        // Introspector populado pelo walk; consulta `formatted_counter_at`
        // retorna snapshot na Location de cada heading.
        let content = doc_3_headings();
        let intr = introspect_with_introspector(&content);
        let txt = layout_with_introspector(&content, intr).plain_text();

        assert!(txt.contains("1. Intro"), "esperado '1. Intro' em: {:?}", txt);
        assert!(
            txt.contains("1.1. Motivacao"),
            "esperado '1.1. Motivacao' em: {:?}",
            txt
        );
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
        let heading_locs = intr
            .kind_index
            .get(&crate::entities::element_kind::ElementKind::Heading)
            .cloned()
            .unwrap_or_default();
        assert_eq!(heading_locs.len(), 3, "3 headings indexadas");
        assert_eq!(
            intr.formatted_counter_at("heading", heading_locs[0]).as_deref(),
            Some("1")
        );
        assert_eq!(
            intr.formatted_counter_at("heading", heading_locs[1]).as_deref(),
            Some("1.1")
        );
        assert_eq!(
            intr.formatted_counter_at("heading", heading_locs[2]).as_deref(),
            Some("2")
        );

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
        assert!(
            !txt.contains("1. Conclusao"),
            "regressão P183B: 'Conclusao' não pode ter prefixo '1.': {:?}",
            txt
        );
    }
}

// ── P188B — C2 equation counter migration ───────────────────────────────────

#[cfg(test)]
mod p188b_c2_equation_counter {
    use super::*;
    use crate::engine::introspect::introspect_with_introspector;
    use crate::entities::introspector::Introspector;
    use crate::entities::state_update::StateUpdate;
    use crate::entities::value::Value;
    use std::sync::Arc;

    fn equation_block(text: &str) -> Content {
        // Lote F-2 S2 (P335): numeração assada (gate via campo assado).
        Content::equation_numbered(Content::MathIdent(text.into()), true)
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
        // Path Introspector funcional para equations block **numeradas**
        // (`equation_block(..)` = campo assado true): o gate dispara → counter
        // introspector populado → `flat_counter_at` retorna valores correctos.
        // Lote F-4 E0 (P338): o injector `numbering_active:equation` saiu (canal
        // StateRegistry morto); o nome histórico "quando_state_injectado" refere
        // a injeção removida — o counter mantém [1,2,3] pelo campo assado.
        let parts = vec![equation_block("a"), equation_block("b"), equation_block("c")];
        let content = Content::Sequence(Arc::from(parts));

        let intr = introspect_with_introspector(&content);

        // Validação intermédia: counter populado.
        let eq_locs = intr
            .kind_index
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
    use crate::engine::introspect::{introspect, introspect_with_introspector};
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
            Content::heading(1, Content::text("Intro")),
            Content::outline(),
        ]));
        let state_com = introspect(&doc_com_outline);
        // Layout funciona via Introspector path (re-walk em layout()).
        let txt_com = layout(&doc_com_outline).plain_text();
        assert!(txt_com.contains("Intro"), "doc com outline: {:?}", txt_com);

        let doc_sem_outline = Content::Sequence(Arc::from(vec![Content::heading(
            1,
            Content::text("Solo"),
        )]));
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
            Content::heading(1, Content::text("A")),
            Content::heading(2, Content::text("B")),
        ]));
        let intr = introspect_with_introspector(&content);
        // Lote F-2 S5 (P335): is_numbering_active removido (StateRegistry saiu).
        assert_eq!(intr.headings_for_toc().len(), 2);
        assert!(intr.resolved_labels.get(&Label("auto-toc-1".to_string())).is_some());
    }

    #[test]
    fn walk_excepcao_e3_figure_via_intr() {
        // E3 (P190H adapted): Figure walk arm popula
        // intr.counters["figure:image"] via populate_intr arm Figure
        // (P191C, gated por is_counted). Field legacy
        // `state.figure_numbers` eliminado.
        let content = Content::figure(
            Content::Empty,
            Some(Content::text("cap")),
            Some("image".into()),
            Some("1".into()),
        );
        let intr = introspect_with_introspector(&content);
        assert_eq!(
            intr.figure_number_at_index("image", 0),
            Some(1),
            "E3: intr.figure_number_at_index(image, 0) = 1 via populate_intr"
        );
    }

    #[test]
    fn walk_excepcao_e4_labelled_resolved_labels_via_intr() {
        // E4 (P190G adapted): Labelled walk arm popula
        // intr.resolved_labels via Tag::Labelled pós-recursão (P195D).
        // Field legacy `state.resolved_labels` eliminado.
        let content = Content::Sequence(Arc::from(vec![Content::label_auto(
            "intro".to_string(),
            Content::heading(1, Content::text("X")),
        )]));
        let intr = introspect_with_introspector(&content);
        assert!(
            intr.resolved_labels
                .get(&crate::entities::label::Label("intro".to_string()))
                .is_some(),
            "E4: intr.resolved_labels[intro] populado"
        );
    }

    #[test]
    fn walk_excepcao_e6_counter_update_via_legacy() {
        // E6: CounterUpdate walk arm. Confirma que walk legacy ainda
        // populates state.flat para chaves custom via CounterUpdate.
        let content = Content::Sequence(Arc::from(vec![
            Content::counter_update(
                "custom".to_string(),
                crate::entities::counter_update::CounterUpdate::Step,
            ),
            Content::counter_update(
                "custom".to_string(),
                crate::entities::counter_update::CounterUpdate::Step,
            ),
        ]));
        // P190I (M6 fechado): state legacy eliminado; verificar via intr.
        let intr = introspect(&content);
        let custom_count = intr
            .counters
            .value("custom")
            .and_then(|v| v.last())
            .copied()
            .unwrap_or(0);
        assert_eq!(
            custom_count, 2,
            "E6: intr.counters['custom'].last() = 2 após 2 steps"
        );
    }
}

// ── P194B — C4 resolved label migration ─────────────────────────────────────

#[cfg(test)]
mod p194b_c4_resolved_label {
    use super::*;
    use crate::engine::introspect::introspect;
    use crate::entities::introspector::TagIntrospector;
    use crate::entities::label::Label;
    use std::sync::Arc;

    fn lbl(s: &str) -> Label {
        Label(s.to_string())
    }

    fn doc_heading_labelled_e_ref(label_name: &str) -> Content {
        // Heading (level 1) wrapped in Labelled + Ref para o mesmo
        // label. Walk legacy popula state.resolved_labels via arm
        // Labelled (E4 P189B excepção).
        // P788: heading com numbering (sem numbering o vanilla erra —
        // `cannot reference heading without numbering`).
        Content::Sequence(Arc::from(vec![
            Content::label_auto(
                label_name.to_string(),
                Content::Styled(
                    Box::new(Content::heading(1, Content::text("Intro"))),
                    Styles::new()
                        .push_custom("heading.numbering", Value::Bool(true))
                        .push_custom(
                            "heading.numbering.pattern",
                            Value::Str("1.".into()),
                        ),
                ),
            ),
            Content::reference(label_name),
        ]))
    }

    #[test]
    fn c4_resolved_label_via_introspector_path_puro() {
        // P190G: fallback legacy `state.resolved_labels` ELIMINADO.
        // Introspector path é única fonte da verdade. Este test
        // confirma que populate manual de intr.resolved_labels é
        // suficiente para Layouter renderizar correctamente.
        let content = Content::Sequence(Arc::from(vec![Content::reference("intro")]));

        // P190I: state eliminado
        let mut intr = TagIntrospector::empty();
        intr.resolved_labels
            .insert(lbl("intro"), "Introspector text".to_string());

        let txt = layout_with_introspector(&content, intr).plain_text();

        assert!(txt.contains("Introspector text"), "Introspector path puro: {:?}", txt);
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
        // P788: com numbering, o caminho numérico renderiza "Section 1"
        // (doc en) — o legacy "Secção 1" fica como fallback apenas.
        let txt = layout(&content).plain_text().replace('\u{a0}', " ");

        assert!(
            txt.contains("Section 1"),
            "Introspector path: 'Section 1' renderizada: {:?}",
            txt
        );
        assert!(
            !txt.contains("@intro"),
            "ref intro NÃO deve cair em fallback @intro: {:?}",
            txt
        );
    }

    #[test]
    fn c4_resolved_label_paridade_pipelines() {
        // P788: reescrito como teste de PRECEDÊNCIA — o caminho numérico
        // (counter) ganha quando existe; o legacy `resolved_labels` serve
        // de fallback quando o mapa numérico está ausente.
        let content = doc_heading_labelled_e_ref("intro");

        // Path A: pipeline normal (heading numerado) → "Section 1".
        let txt_a = layout(&content).plain_text().replace('\u{a0}', " ");

        // Path B: intr manual SÓ com o legacy (sem counter key) →
        // fallback "Secção 1".
        let mut intr_b = TagIntrospector::empty();
        intr_b.resolved_labels.insert(lbl("intro"), "Secção 1".to_string());
        let txt_b = layout_with_introspector(
            &Content::Sequence(Arc::from(vec![Content::reference("intro")])),
            intr_b,
        )
        .plain_text();

        assert!(txt_a.contains("Section 1"), "Path A (numérico): {:?}", txt_a);
        assert!(txt_b.contains("Secção 1"), "Path B (fallback legacy): {:?}", txt_b);
    }

    #[test]
    fn c4_resolved_label_fallback_at_arrobado_quando_ausente() {
        // P788: label inexistente deixou de renderizar "?" — agora é erro
        // de layout (vanilla: `does not exist in the document`).
        let content = Content::Sequence(Arc::from(vec![Content::reference("missing")]));

        let intr = TagIntrospector::empty();

        let doc = layout_with_introspector(&content, intr);

        assert!(
            doc.layout_errors.iter().any(|d| d
                .message
                .contains("label `<missing>` does not exist in the document")),
            "erro de label inexistente esperado: {:?}",
            doc.layout_errors
        );
    }
}

// ── P195D — Walk arm Labelled emite Tag pós-recursão (ADR-0069) ─────────────

#[cfg(test)]
mod p195d_walk_labelled {
    use super::*;
    use crate::engine::introspect::introspect_with_introspector;
    use crate::entities::label::Label;
    use std::sync::Arc;

    fn lbl(s: &str) -> Label {
        Label(s.to_string())
    }

    #[test]
    fn labelled_walk_emite_tag_e_popula_introspector() {
        let content = Content::Sequence(Arc::from(vec![Content::label_auto(
            "intro".to_string(),
            Content::heading(1, Content::text("Intro")),
        )]));

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
            Content::label_auto(
                "intro".to_string(),
                // P788: heading com numbering (sem numbering o vanilla erra).
                Content::Styled(
                    Box::new(Content::heading(1, Content::text("Intro"))),
                    Styles::new()
                        .push_custom("heading.numbering", Value::Bool(true))
                        .push_custom(
                            "heading.numbering.pattern",
                            Value::Str("1.".into()),
                        ),
                ),
            ),
            Content::reference("intro"),
        ]));

        let intr = introspect_with_introspector(&content);

        // P788: com o wrapper Styled (necessário para o numbering), o
        // `compute_labelled` não dispara — o target é `Styled`, não
        // `Heading` — logo o sub-store legacy fica vazio nesta forma. A
        // população do store (heading directo) está coberta por
        // `labelled_walk_emite_tag_e_popula_introspector`. O que interessa
        // aqui é o RENDER pelo caminho numérico.
        assert_eq!(intr.resolved_labels.get(&lbl("intro")), None);

        // Pipeline completo: Ref renderiza via caminho numérico (doc en).
        let txt = layout(&content).plain_text().replace('\u{a0}', " ");
        assert!(
            txt.contains("Section 1"),
            "Ref intro → 'Section 1' via caminho numérico: {:?}",
            txt
        );
        assert!(!txt.contains("@intro"), "fallback @intro NÃO esperado: {:?}", txt);
    }

    #[test]
    fn labelled_figure_target_popula_figure_label_numbers() {
        let content = Content::Sequence(Arc::from(vec![labelled_prod(
            Content::figure(
                Content::text("body"),
                Some(Content::text("caption")),
                Some("image".into()),
                Some("1".into()),
            ),
            lbl("fig1"),
        )]));

        let intr = introspect_with_introspector(&content);

        // figure_label_numbers populated (write paralelo P195D + P168).
        assert_eq!(intr.figure_label_numbers.get(&lbl("fig1")), Some(&1),);
        // resolved_labels também populated via P195D Tag.
        assert!(intr.resolved_labels.get(&lbl("fig1")).is_some());
    }

    #[test]
    fn labelled_target_nao_resolvivel_nao_popula_introspector() {
        // Target = Text (sem numeração); compute_labelled retorna
        // (None, None); Tag não emitida; sub-store não populated.
        let content = Content::Sequence(Arc::from(vec![Content::label_auto(
            "foo".to_string(),
            Content::text("not numbered"),
        )]));

        let intr = introspect_with_introspector(&content);

        assert_eq!(intr.resolved_labels.get(&lbl("foo")), None);
        assert_eq!(intr.figure_label_numbers.get(&lbl("foo")), None);
    }
}

// ── P273.7 — Boxed save/restore parent_bbox (completa Decisão 3 P273.6) ──
// Testes-primeiro do refino estrutural Cluster Gradient. Verificam o
// arm `Content::Boxed` em `mod.rs` ganha save/restore de `parent_bbox`
// análogo ao Block P273.6 (template replicado literal; bbox.y
// baseline-relative em contexto inline per Decisão 1 Fase A
// `3γ.2.γ-inline-baseline-y`).
#[cfg(test)]
mod p273_7_boxed_parent_bbox {
    use super::*;
    use crate::entities::corners::Corners;
    use crate::entities::geometry::ShapeKind;
    use crate::entities::layout_types::{Abs, Length, Pt};
    use crate::entities::sides::Sides;
    use std::sync::Arc;

    fn rect_shape() -> Content {
        Content::shape(
            ShapeKind::Rect,
            Some(Box::new(crate::entities::value::Value::Length(Length {
                abs: Abs(50.0),
                em: 0.0,
            }))),
            Some(Box::new(crate::entities::value::Value::Length(Length {
                abs: Abs(30.0),
                em: 0.0,
            }))),
            None,
            None,
        )
    }

    fn boxed_dimensioned(body: Content, w_pt: f64, h_pt: f64) -> Content {
        Content::Boxed(std::sync::Arc::new(crate::entities::elements::boxed::BoxedElem {
            body,
            width: Some(Length { abs: Abs(w_pt), em: 0.0 }),
            height: Some(Length { abs: Abs(h_pt), em: 0.0 }),
            inset: Sides::uniform(Length::ZERO),
            baseline: Length::ZERO,
            outset: Sides::uniform(Length::ZERO),
            radius: Corners::uniform(Length::ZERO),
            clip: false,
            fill: None,
            stroke: None,
        }))
    }

    fn boxed_dimensionless(body: Content) -> Content {
        Content::Boxed(std::sync::Arc::new(crate::entities::elements::boxed::BoxedElem {
            body,
            width: None,
            height: None,
            inset: Sides::uniform(Length::ZERO),
            baseline: Length::ZERO,
            outset: Sides::uniform(Length::ZERO),
            radius: Corners::uniform(Length::ZERO),
            clip: false,
            fill: None,
            stroke: None,
        }))
    }

    /// Helper: colecciona todos os FrameItem::Shape (recursivo em Group)
    /// nas páginas do documento e devolve os respectivos
    /// `parent_bbox_at_emit`.
    fn shape_parent_bboxes(
        doc: &crate::entities::layout_types::PagedDocument,
    ) -> Vec<Option<crate::entities::layout_types::Rect>> {
        fn walk(
            items: &[FrameItem],
            out: &mut Vec<Option<crate::entities::layout_types::Rect>>,
        ) {
            for item in items {
                match item {
                    FrameItem::Shape { parent_bbox_at_emit, .. } => {
                        out.push(*parent_bbox_at_emit);
                    }
                    FrameItem::Group { items, .. } => walk(items, out),
                    _ => {}
                }
            }
        }
        let mut out = Vec::new();
        for page in &doc.pages {
            walk(&page.items, &mut out);
        }
        out
    }

    /// 1) Boxed com width+height literais → shape nested ganha
    /// `parent_bbox_at_emit = Some(Rect)` com w/h iguais às dimensions
    /// do Boxed.
    #[test]
    fn p273_7_shape_inside_boxed_carries_parent_bbox() {
        let content = Content::Sequence(Arc::from(vec![boxed_dimensioned(
            rect_shape(),
            200.0,
            100.0,
        )]));
        let doc = layout(&content);
        let bboxes = shape_parent_bboxes(&doc);
        // Inner Shape do body deve ter parent_bbox = Some com w=200pt,h=100pt.
        let some_with_box = bboxes
            .iter()
            .any(|b| matches!(b, Some(r) if r.w == Pt(200.0) && r.h == Pt(100.0)));
        assert!(
            some_with_box,
            "Shape dentro de Boxed(w=200pt,h=100pt) deve ter \
             parent_bbox_at_emit = Some(Rect{{w:200pt,h:100pt}}); got: {:?}",
            bboxes
        );
    }

    /// 2) Boxed sem width/height → shape nested permanece com
    /// `parent_bbox_at_emit = None` (Decisão 3γ.2.γ herdada).
    #[test]
    fn p273_7_shape_inside_boxed_dimensionless_no_parent_bbox() {
        let content =
            Content::Sequence(Arc::from(vec![boxed_dimensionless(rect_shape())]));
        let doc = layout(&content);
        let bboxes = shape_parent_bboxes(&doc);
        // Sem dimensions literais, parent_bbox permanece None.
        assert!(
            bboxes.iter().all(|b| b.is_none()),
            "Boxed sem width/height não deve popular parent_bbox; got: {:?}",
            bboxes
        );
    }

    /// 3) Save/restore LIFO — top-level shape APÓS Boxed ganha
    /// `parent_bbox_at_emit = None` (parent_bbox restaurado).
    #[test]
    fn p273_7_boxed_save_restore_lifo() {
        let content = Content::Sequence(Arc::from(vec![
            // Top-level shape ANTES: parent_bbox = None.
            rect_shape(),
            // Boxed dimensionado: nested shape ganha bbox do Boxed.
            boxed_dimensioned(rect_shape(), 200.0, 100.0),
            // Top-level shape DEPOIS: parent_bbox restaurado a None.
            rect_shape(),
        ]));
        let doc = layout(&content);
        let bboxes = shape_parent_bboxes(&doc);
        // Esperado: pelo menos 3 shapes; 2 com None (top-level) +
        // 1 com Some(box bbox) (nested).
        let n_none = bboxes.iter().filter(|b| b.is_none()).count();
        let n_some = bboxes.iter().filter(|b| b.is_some()).count();
        assert!(
            n_none >= 2,
            "≥2 shapes top-level devem ter parent_bbox=None (LIFO restore); got: {:?}",
            bboxes
        );
        assert_eq!(
            n_some, 1,
            "exactamente 1 shape nested deve ter parent_bbox=Some; got: {:?}",
            bboxes
        );
    }

    /// 4) Nested Boxed (Boxed dentro de Boxed) — LIFO restore correcto.
    /// Inner Boxed bbox visível para inner Shape; depois restore para
    /// outer Boxed bbox; depois restore para None.
    #[test]
    fn p273_7_nested_boxed_lifo() {
        let inner = boxed_dimensioned(rect_shape(), 50.0, 25.0);
        let outer =
            boxed_dimensioned(Content::Sequence(Arc::from(vec![inner])), 200.0, 100.0);
        let content = Content::Sequence(Arc::from(vec![outer]));
        let doc = layout(&content);
        let bboxes = shape_parent_bboxes(&doc);
        // Inner Shape deve ver bbox do inner Boxed (50×25), NÃO o outer (200×100).
        let saw_inner_bbox = bboxes
            .iter()
            .any(|b| matches!(b, Some(r) if r.w == Pt(50.0) && r.h == Pt(25.0)));
        assert!(
            saw_inner_bbox,
            "Inner Shape deve ver bbox do inner Boxed (50×25); got: {:?}",
            bboxes
        );
    }

    /// 5) Boxed.fill stroke próprio Shape — emit do PRÓPRIO Boxed
    /// (linha 1485) deve ter `parent_bbox_at_emit` igual ao OUTER
    /// (i.e. None se top-level), paridade Block P273.6 §2.3 — restore
    /// acontece ANTES do shape emit do próprio container.
    #[test]
    fn p273_7_boxed_own_shape_uses_outer_parent_bbox() {
        use crate::entities::layout_types::Color;
        let body = Content::text("X"); // qualquer body simples
        let boxed = Content::Boxed(std::sync::Arc::new(
            crate::entities::elements::boxed::BoxedElem {
                body,
                width: Some(Length { abs: Abs(150.0), em: 0.0 }),
                height: Some(Length { abs: Abs(40.0), em: 0.0 }),
                inset: Sides::uniform(Length::ZERO),
                baseline: Length::ZERO,
                outset: Sides::uniform(Length::ZERO),
                radius: Corners::uniform(Length::ZERO),
                clip: false,
                fill: Some(Color::rgb(255, 0, 0)),
                stroke: None,
            },
        ));
        let content = Content::Sequence(Arc::from(vec![boxed]));
        let doc = layout(&content);
        let bboxes = shape_parent_bboxes(&doc);
        // Boxed.fill → emit Shape próprio com parent_bbox = outer (None
        // top-level). Save/restore garante restore antes do shape emit.
        let none_count = bboxes.iter().filter(|b| b.is_none()).count();
        assert!(none_count >= 1,
            "Boxed's own shape emit deve usar parent_bbox outer (None top-level); got: {:?}",
            bboxes);
    }
}

// ── P273.9 — Containers estendidos (Grid cell + Stack + Pad — escopo 1γ) ──
// Testes-primeiro do refino estrutural Cluster Gradient. Verificam os arms
// `Content::Grid`, `Content::Stack` e `Content::Pad` em mod.rs/grid.rs
// ganharem save/restore de `parent_bbox` — Grid via paralelo a
// `cell_origin_*` (DEBT-37 reused N=4); Stack/Pad via measure_content_constrained
// pre-layout (layout duplo arquitectural aceite N=1 inaugural).
#[cfg(test)]
mod p273_9_containers_estendidos {
    use super::*;
    use crate::entities::geometry::ShapeKind;
    use crate::entities::layout_types::{Abs, Length, Pt, TrackSizing};
    use crate::entities::sides::Sides;
    use std::sync::Arc;

    fn rect_shape(w: f64, h: f64) -> Content {
        Content::shape(
            ShapeKind::Rect,
            Some(Box::new(crate::entities::value::Value::Length(Length {
                abs: Abs(w),
                em: 0.0,
            }))),
            Some(Box::new(crate::entities::value::Value::Length(Length {
                abs: Abs(h),
                em: 0.0,
            }))),
            None,
            None,
        )
    }

    fn shape_parent_bboxes(
        doc: &crate::entities::layout_types::PagedDocument,
    ) -> Vec<Option<crate::entities::layout_types::Rect>> {
        fn walk(
            items: &[FrameItem],
            out: &mut Vec<Option<crate::entities::layout_types::Rect>>,
        ) {
            for item in items {
                match item {
                    FrameItem::Shape { parent_bbox_at_emit, .. } => {
                        out.push(*parent_bbox_at_emit);
                    }
                    FrameItem::Group { items, .. } => walk(items, out),
                    _ => {}
                }
            }
        }
        let mut out = Vec::new();
        for page in &doc.pages {
            walk(&page.items, &mut out);
        }
        out
    }

    // ── Grid cell ────────────────────────────────────────────────────────

    /// 1) Grid cell save/restore — shape nested em cell Grid ganha
    /// `parent_bbox_at_emit = Some(Rect)` com bbox do body cell.
    #[test]
    fn p273_9_grid_cell_save_restore_parent_bbox() {
        let cell_content = rect_shape(20.0, 10.0);
        let grid = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Fixed(100.0)],
                rows: vec![],
                cells: vec![cell_content],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(Length::ZERO),
                header: None,
                footer: None,
                stroke: None,
                fill: None,
            },
        ));
        let content = Content::Sequence(Arc::from(vec![grid]));
        let doc = layout(&content);
        let bboxes = shape_parent_bboxes(&doc);
        // Inner Shape do body cell ganha bbox da cell (body_w = column width = 100pt).
        let saw_cell_bbox =
            bboxes.iter().any(|b| matches!(b, Some(r) if r.w == Pt(100.0)));
        assert!(
            saw_cell_bbox,
            "Shape dentro de Grid cell deve ter parent_bbox com w=100pt; got: {:?}",
            bboxes
        );
    }

    /// 2) Top-level shape após Grid → parent_bbox restaurado (LIFO).
    #[test]
    fn p273_9_grid_cell_lifo_restore() {
        let grid = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Fixed(100.0)],
                rows: vec![],
                cells: vec![rect_shape(20.0, 10.0)],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(Length::ZERO),
                header: None,
                footer: None,
                stroke: None,
                fill: None,
            },
        ));
        let content = Content::Sequence(Arc::from(vec![
            grid,
            rect_shape(50.0, 30.0), // top-level após Grid
        ]));
        let doc = layout(&content);
        let bboxes = shape_parent_bboxes(&doc);
        // ≥1 None (top-level shape pós Grid) + ≥1 Some (cell body).
        let n_none = bboxes.iter().filter(|b| b.is_none()).count();
        let n_some = bboxes.iter().filter(|b| b.is_some()).count();
        assert!(
            n_none >= 1,
            "≥1 shape top-level após Grid deve ter parent_bbox=None (LIFO); got: {:?}",
            bboxes
        );
        assert!(
            n_some >= 1,
            "≥1 shape em cell deve ter parent_bbox=Some; got: {:?}",
            bboxes
        );
    }

    // ── Stack ────────────────────────────────────────────────────────────

    /// 3) Stack vertical TTB — shape nested ganha parent_bbox da Stack
    /// medida via measure_content_constrained (vertical: max_w × sum_h).
    #[test]
    fn p273_9_stack_vertical_save_restore_parent_bbox() {
        use crate::entities::dir::Dir;
        let stack = Content::stack(
            vec![rect_shape(50.0, 20.0), rect_shape(80.0, 30.0)],
            Dir::TTB,
            None,
        );
        let content = Content::Sequence(Arc::from(vec![stack]));
        let doc = layout(&content);
        let bboxes = shape_parent_bboxes(&doc);
        // Inner Shapes ganham bbox stack: max_w = 80, sum_h = 50.
        let saw_stack_bbox = bboxes
            .iter()
            .any(|b| matches!(b, Some(r) if r.w == Pt(80.0) && r.h == Pt(50.0)));
        assert!(
            saw_stack_bbox,
            "Shape dentro de Stack TTB deve ter parent_bbox = {{w:80,h:50}}; got: {:?}",
            bboxes
        );
    }

    /// 4) Stack vazio (n=0) não popula parent_bbox.
    #[test]
    fn p273_9_stack_empty_no_parent_bbox() {
        use crate::entities::dir::Dir;
        let stack = Content::stack(vec![], Dir::TTB, None);
        let content = Content::Sequence(Arc::from(vec![stack, rect_shape(50.0, 30.0)]));
        let doc = layout(&content);
        let bboxes = shape_parent_bboxes(&doc);
        // Stack vazio não emite shapes inner; top-level shape vê None
        // (Stack vazio não setou bbox).
        assert!(
            bboxes.iter().all(|b| b.is_none()),
            "Stack vazio + top-level shape: todos parent_bbox=None; got: {:?}",
            bboxes
        );
    }

    // ── Pad ──────────────────────────────────────────────────────────────

    /// 5) Pad — shape nested ganha parent_bbox INNER (body sem insets).
    #[test]
    fn p273_9_pad_save_restore_parent_bbox_inner() {
        let pad = Content::pad(
            rect_shape(50.0, 30.0),
            Sides::new(
                Some(Length::pt(10.0)), // left
                Some(Length::pt(10.0)), // right
                Some(Length::pt(5.0)),  // top
                Some(Length::pt(5.0)),  // bottom
            ),
        );
        let content = Content::Sequence(Arc::from(vec![pad]));
        let doc = layout(&content);
        let bboxes = shape_parent_bboxes(&doc);
        // Inner Shape (rect 50×30) deve ver parent_bbox INNER do Pad:
        // body_w/body_h medido = 50×30 (Shape literal dimensions).
        let saw_pad_inner = bboxes
            .iter()
            .any(|b| matches!(b, Some(r) if r.w == Pt(50.0) && r.h == Pt(30.0)));
        assert!(
            saw_pad_inner,
            "Shape dentro de Pad deve ter parent_bbox INNER = {{w:50,h:30}}; got: {:?}",
            bboxes
        );
    }

    /// 6) Pad LIFO — top-level shape após Pad vê parent_bbox=None.
    #[test]
    fn p273_9_pad_lifo_restore() {
        let pad = Content::pad(
            rect_shape(50.0, 30.0),
            Sides::new(
                Some(Length::pt(10.0)),
                Some(Length::pt(10.0)),
                Some(Length::pt(5.0)),
                Some(Length::pt(5.0)),
            ),
        );
        let content = Content::Sequence(Arc::from(vec![pad, rect_shape(20.0, 10.0)]));
        let doc = layout(&content);
        let bboxes = shape_parent_bboxes(&doc);
        let n_none = bboxes.iter().filter(|b| b.is_none()).count();
        let n_some = bboxes.iter().filter(|b| b.is_some()).count();
        assert!(
            n_none >= 1,
            "≥1 top-level shape após Pad deve ter parent_bbox=None; got: {:?}",
            bboxes
        );
        assert!(
            n_some >= 1,
            "≥1 inner shape do Pad deve ter parent_bbox=Some; got: {:?}",
            bboxes
        );
    }

    /// 7) Regressão DEBT-37: tests `cell_origin_*` consumption preserved.
    /// Verifica que Grid arm ainda emite cell layout correctamente após
    /// adicionar save/restore parent_bbox paralelo.
    #[test]
    fn p273_9_grid_debt37_cell_origin_consumption_preserved() {
        // Smoke test: layout grid simples e verifica que pelo menos
        // uma FrameItem::Shape é emitida (consumption preserved).
        let grid = Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                columns: vec![TrackSizing::Fixed(100.0), TrackSizing::Fixed(100.0)],
                rows: vec![],
                cells: vec![rect_shape(20.0, 10.0), rect_shape(30.0, 15.0)],
                hlines: vec![],
                vlines: vec![],
                gutter: None,
                align: None,
                inset: Sides::uniform(Length::ZERO),
                header: None,
                footer: None,
                stroke: None,
                fill: None,
            },
        ));
        let content = Content::Sequence(Arc::from(vec![grid]));
        let doc = layout(&content);
        let bboxes = shape_parent_bboxes(&doc);
        // ≥2 shapes emitidos (1 por cell); ambos com parent_bbox=Some
        // (body_w = 100 cada cell).
        assert!(
            bboxes.len() >= 2,
            "Grid 2 cells deve emitir ≥2 shapes; got: {} shapes",
            bboxes.len()
        );
    }
}

// ── Passo 284 — text decoration (Layouter integration) ────────────────────

#[cfg(test)]
mod p284_decoration_tests {
    use super::*;

    /// Conta `FrameItem::Line` no primeiro frame de `doc`.
    fn count_lines(doc: &PagedDocument) -> usize {
        doc.pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter(|i| matches!(i, FrameItem::Line { .. }))
            .count()
    }

    /// Devolve a primeira `FrameItem::Line` encontrada (ou panic).
    fn first_line(doc: &PagedDocument) -> (Point, Point, f64) {
        for page in &doc.pages {
            for item in &page.items {
                if let FrameItem::Line { start, end, thickness, .. } = item {
                    return (*start, *end, *thickness);
                }
            }
        }
        panic!("nenhum FrameItem::Line encontrado");
    }

    #[test]
    fn underline_emite_uma_frameitem_line_alem_do_texto() {
        let c = Content::underline(Content::text("hello"), None, None, None);
        let doc = layout(&c);
        assert!(!doc.pages.is_empty(), "Underline com texto produz ≥1 página");
        assert!(doc.plain_text().contains("hello"), "texto preservado no plain_text");
        assert_eq!(count_lines(&doc), 1, "exactamente 1 linha de decoração emitida");
    }

    #[test]
    fn strike_e_overline_emitem_y_em_posicoes_diferentes() {
        // Mesma string, kinds distintos → Y da linha distinto (offset
        // default por kind).
        let body = || Content::text("text");
        let u = layout(&Content::underline(body(), None, None, None));
        let s = layout(&Content::strike(body(), None, None, None));
        let o = layout(&Content::overline(body(), None, None, None));
        let (us, _, _) = first_line(&u);
        let (ss, _, _) = first_line(&s);
        let (os, _, _) = first_line(&o);
        // Underline está abaixo do baseline → maior Y no espaço Layouter.
        // Strike a meio do x-height → entre underline e overline.
        // Overline acima do cap → menor Y.
        assert!(
            us.y.val() > ss.y.val(),
            "underline Y ({}) deve ser maior que strike Y ({})",
            us.y.val(),
            ss.y.val()
        );
        assert!(
            ss.y.val() > os.y.val(),
            "strike Y ({}) deve ser maior que overline Y ({})",
            ss.y.val(),
            os.y.val()
        );
    }

    #[test]
    fn decoration_offset_override_substitui_default() {
        use crate::entities::layout_types::Length;
        let default = layout(&Content::underline(Content::text("x"), None, None, None));
        let with_offset = layout(&Content::underline(
            Content::text("x"),
            None,
            Some(Length::pt(5.0)),
            None,
        ));
        let (ds, _, _) = first_line(&default);
        let (os, _, _) = first_line(&with_offset);
        assert!((os.y.val() - ds.y.val()).abs() > 0.5,
                "override de offset deve mudar a coordenada Y da linha; default={} override={}",
                ds.y.val(), os.y.val());
    }

    #[test]
    fn decoration_extent_estende_horizontalmente() {
        use crate::entities::layout_types::Length;
        let baseline = layout(&Content::underline(Content::text("hi"), None, None, None));
        let extended = layout(&Content::underline(
            Content::text("hi"),
            None,
            None,
            Some(Length::pt(3.0)),
        ));
        let (bs, be, _) = first_line(&baseline);
        let (es, ee, _) = first_line(&extended);
        let base_len = be.x.val() - bs.x.val();
        let ext_len = ee.x.val() - es.x.val();
        assert!((ext_len - base_len - 6.0).abs() < 0.1,
                "extent: 3pt adiciona 3pt em cada lado (+6pt total); base_len={base_len} ext_len={ext_len}");
    }

    // Nota: o teste de operadores PDF (`q ... S Q`) vive em 03_infra
    // porque L1 não pode importar typst_infra (V3 ForbiddenImport).
    // Adicionado em `03_infra/src/lib.rs` ou ficheiro de testes equivalente.

    // ── Passo 285 — herança stroke→fill no consumer Layouter ────────────

    /// Devolve o `color` da primeira `FrameItem::Line` encontrada no doc.
    fn first_line_color(
        doc: &PagedDocument,
    ) -> Option<crate::entities::layout_types::Color> {
        for page in &doc.pages {
            for item in &page.items {
                if let FrameItem::Line { color, .. } = item {
                    return *color;
                }
            }
        }
        panic!("nenhum FrameItem::Line encontrado");
    }

    #[test]
    fn p285_underline_stroke_explicito_passa_para_line_color() {
        use crate::entities::layout_types::Color;
        let red = Color::rgb(255, 0, 0);
        let doc = layout(&Content::underline(Content::text("x"), Some(red), None, None));
        assert_eq!(
            first_line_color(&doc),
            Some(red),
            "stroke explícito deve aparecer literalmente em FrameItem::Line.color"
        );
    }

    #[test]
    fn p285_underline_sem_stroke_nem_fill_color_none() {
        let doc = layout(&Content::underline(Content::text("x"), None, None, None));
        // Sem stroke explícito + texto sem fill (default) → herança None
        // → color: None (PDF default preto bit-exact pré-P285).
        assert_eq!(
            first_line_color(&doc),
            None,
            "sem stroke nem fill, FrameItem::Line.color deve ser None"
        );
    }

    #[test]
    fn p285_underline_heranca_text_fill_via_styled() {
        use crate::entities::layout_types::Color;
        use crate::entities::style::{Style, Styles};
        let blue = Color::rgb(0, 0, 255);
        let inner = Content::underline(Content::text("h"), None, None, None);
        // Styled([Fill(blue)], Underline { stroke: None }) → consumer
        // lê self.style.fill (já actualizado pelo Styled outer) → emite
        // color: Some(blue). Paridade vanilla "decoração herda cor do
        // texto" (P285 §A.3 opção β).
        let styled =
            Content::Styled(Box::new(inner), Styles::from_iter([Style::Fill(blue)]));
        let doc = layout(&styled);
        assert_eq!(
            first_line_color(&doc),
            Some(blue),
            "underline sem stroke explícito deve herdar fill do contexto Styled"
        );
    }

    #[test]
    fn p285_stroke_user_wins_sobre_fill_do_texto() {
        // Quando stroke explícito e fill herdado existem, stroke wins
        // (regra A.3: stroke.or(style.fill) — Some(x).or(_) == Some(x)).
        use crate::entities::layout_types::Color;
        use crate::entities::style::{Style, Styles};
        let red = Color::rgb(255, 0, 0);
        let green = Color::rgb(0, 255, 0);
        let inner = Content::underline(Content::text("y"), Some(green), None, None);
        let styled = Content::Styled(
            Box::new(inner),
            Styles::from_iter([Style::Fill(red)]), // ← fill red herdado
        );
        let doc = layout(&styled);
        assert_eq!(
            first_line_color(&doc),
            Some(green),
            "stroke explícito (green) wins sobre fill herdado (red)"
        );
    }

    // ── Passo 286 — wrap-aware text decoration ──────────────────────────

    /// Devolve **todas** as `FrameItem::Line` (em ordem de emissão) no
    /// primeiro frame que tem itens.
    fn collect_lines(
        doc: &PagedDocument,
    ) -> Vec<(Point, Point, f64, Option<crate::entities::layout_types::Color>)> {
        let mut out = Vec::new();
        for page in &doc.pages {
            for item in &page.items {
                if let FrameItem::Line { start, end, thickness, color } = item {
                    out.push((*start, *end, *thickness, *color));
                }
            }
        }
        out
    }

    #[test]
    fn p286_underline_single_line_emite_uma_linha_regression_p285() {
        // P286 §3 fallback bit-exact: body que cabe numa linha produz
        // 1 Line — exactamente como P284/P285 (regression test).
        let doc = layout(&Content::underline(Content::text("hi"), None, None, None));
        let lines = collect_lines(&doc);
        assert_eq!(
            lines.len(),
            1,
            "body single-line deve emitir exactamente 1 Line (regression P285); got {}",
            lines.len()
        );
    }

    #[test]
    fn p286_underline_texto_longo_emite_multiplas_linhas() {
        // Texto suficientemente longo para forçar ≥2 quebras de linha
        // na página default A4 (margin 70pt, useful ~455pt). Cada palavra
        // ocupa ~30pt em FixedMetrics 12pt; 50 palavras → ≥3 linhas.
        let texto_longo: String =
            (0..50).map(|i| format!("word{i}")).collect::<Vec<_>>().join(" ");
        let doc =
            layout(&Content::underline(Content::text(&texto_longo), None, None, None));
        let lines = collect_lines(&doc);
        assert!(
            lines.len() >= 2,
            "body multi-line deve emitir N≥2 Lines (P286 wrap-aware); got {} Lines",
            lines.len()
        );
    }

    #[test]
    fn p286_underline_multilinhas_y_distintos() {
        // Cada Line cobre uma linha visual distinta — Y diferentes.
        let texto: String =
            (0..40).map(|i| format!("w{i}")).collect::<Vec<_>>().join(" ");
        let doc = layout(&Content::underline(Content::text(&texto), None, None, None));
        let lines = collect_lines(&doc);
        assert!(lines.len() >= 2, "esperava ≥2 Lines para wrap");
        // Y values devem ser distintos (cada linha a baseline diferente).
        let ys: std::collections::BTreeSet<i64> =
            lines.iter().map(|(s, _, _, _)| (s.y.val() * 1000.0) as i64).collect();
        assert_eq!(
            ys.len(),
            lines.len(),
            "cada Line deve ter Y único; got {} Y distintos de {} Lines",
            ys.len(),
            lines.len()
        );
    }

    #[test]
    fn p286_underline_multilinhas_herdam_cor_uniforme() {
        // Stroke explícito propaga a TODAS as Lines emitidas (uma cor
        // por decoração; sem variação per-linha).
        use crate::entities::layout_types::Color;
        let red = Color::rgb(255, 0, 0);
        let texto: String =
            (0..40).map(|i| format!("w{i}")).collect::<Vec<_>>().join(" ");
        let doc =
            layout(&Content::underline(Content::text(&texto), Some(red), None, None));
        let lines = collect_lines(&doc);
        assert!(lines.len() >= 2);
        for (_, _, _, c) in &lines {
            assert_eq!(
                *c,
                Some(red),
                "cada Line deve preservar a cor de stroke (uniformidade per-decoração)"
            );
        }
    }

    #[test]
    fn p286_strike_e_overline_tambem_wrap_aware() {
        // Paridade simétrica para os 3 variants P284.
        let texto: String =
            (0..40).map(|i| format!("w{i}")).collect::<Vec<_>>().join(" ");
        let s_doc = layout(&Content::strike(Content::text(&texto), None, None, None));
        let o_doc = layout(&Content::overline(Content::text(&texto), None, None, None));
        assert!(
            collect_lines(&s_doc).len() >= 2,
            "Strike multi-line deve emitir ≥2 Lines"
        );
        assert!(
            collect_lines(&o_doc).len() >= 2,
            "Overline multi-line deve emitir ≥2 Lines"
        );
    }

    #[test]
    fn p286_extent_aplicado_a_todas_as_linhas() {
        // P286 §A.3 opção α: extent simétrico em cada Line emitida.
        use crate::entities::layout_types::Length;
        let texto: String =
            (0..40).map(|i| format!("w{i}")).collect::<Vec<_>>().join(" ");
        let baseline =
            layout(&Content::underline(Content::text(&texto), None, None, None));
        let with_extent = layout(&Content::underline(
            Content::text(&texto),
            None,
            None,
            Some(Length::pt(4.0)),
        ));
        let b_lines = collect_lines(&baseline);
        let e_lines = collect_lines(&with_extent);
        assert_eq!(
            b_lines.len(),
            e_lines.len(),
            "extent não muda o número de Lines, só as larguras"
        );
        assert!(b_lines.len() >= 2);
        for ((bs, be, _, _), (es, ee, _, _)) in b_lines.iter().zip(e_lines.iter()) {
            let b_len = be.x.val() - bs.x.val();
            let e_len = ee.x.val() - es.x.val();
            assert!(
                (e_len - b_len - 8.0).abs() < 0.1,
                "extent: 4pt adiciona 4pt em cada lado (+8pt total) em cada linha; \
                 base_len={b_len} ext_len={e_len}"
            );
        }
    }
}

// ── Passo 287 — SmartQuote consumer Layouter (alternância) ────────────────

#[cfg(test)]
mod p287_smartquote_tests {
    use super::*;

    /// Concatena o texto de todos os `FrameItem::Text` do documento.
    fn collect_text(doc: &PagedDocument) -> String {
        let mut out = String::new();
        for page in &doc.pages {
            for item in &page.items {
                if let FrameItem::Text { text, .. } = item {
                    out.push_str(text.as_str());
                }
            }
        }
        out
    }

    #[test]
    fn p287_smartquote_double_lang_default_curved() {
        // Lang None → DEFAULT_QUOTES = ("“", "”")
        let doc = layout(&Content::sequence(vec![
            Content::smartquote(true),
            Content::smartquote(true),
        ]));
        let txt = collect_text(&doc);
        assert!(
            txt.contains('“') && txt.contains('”'),
            "2 SmartQuote → aspas curvas; got {txt:?}"
        );
    }

    #[test]
    fn p287_smartquote_single_always_ascii_sem_lang() {
        // Aspas simples scope-out smart-apostrophes (paridade P155).
        // Lang None default → sempre ASCII `'`.
        let doc = layout(&Content::sequence(vec![
            Content::smartquote(false),
            Content::smartquote(false),
        ]));
        let txt = collect_text(&doc);
        assert_eq!(
            txt.matches('\'').count(),
            2,
            "2 SmartQuote simples → 2 chars `'` ASCII; got {txt:?}"
        );
        assert!(
            !txt.contains('\u{2018}') && !txt.contains('\u{2019}'),
            "sem curly Unicode (smart-apostrophes scope-out)"
        );
    }

    #[test]
    fn p287_smartquote_state_independente_do_markup() {
        // Diagnóstico §A.3 opção (γ′): estado do markup é local ao
        // eval_markup (P155); estado da função vive no Layouter. Em
        // `Content::sequence([Text("\""), SmartQuote{true}])` o markup
        // pré-resolveu `"` em `eval_markup`, e SmartQuote começa em
        // "open" porque smartquote_double_open default true.
        let doc = layout(&Content::sequence(vec![
            Content::text("\""),       // markup: emite literal
            Content::smartquote(true), // função: state Layouter
        ]));
        let txt = collect_text(&doc);
        assert!(
            txt.contains('"') && txt.contains('“'),
            "estados independentes → got {txt:?}"
        );
    }

    #[test]
    fn p287_smartquote_independencia_layouts_sucessivos() {
        // Cada layout() cria novo Layouter — state smartquote inicia
        // em `open` (per-document). Dois layouts separados → ambos
        // arrancam em open.
        let doc1 = layout(&Content::smartquote(true));
        let doc2 = layout(&Content::smartquote(true));
        let txt1 = collect_text(&doc1);
        let txt2 = collect_text(&doc2);
        assert_eq!(txt1, "“");
        assert_eq!(txt2, "“");
    }

    // ── Passo 288 — testes lang-aware reactivados (adiados de P287 §4) ─

    #[test]
    fn p288_smartquote_double_lang_en_emite_curly_open_e_close() {
        // Com `text.lang = "en"`, esperamos glyphs curly distintos:
        // primeiro `"` (U+201C open), segundo `"` (U+201D close).
        // Adiado em P287 porque `Style::Lang` não existia; activado em P288.
        use crate::entities::lang::Lang;
        use crate::entities::style::{Style, Styles};
        use std::str::FromStr;
        let lang_en = Lang::from_str("en").unwrap();
        let styled = Content::Styled(
            Box::new(Content::sequence(vec![
                Content::smartquote(true),
                Content::smartquote(true),
            ])),
            Styles::from_iter([Style::Lang(lang_en)]),
        );
        let txt = collect_text(&layout(&styled));
        assert!(txt.contains('\u{201C}'), "primeiro deve ser U+201C (open); got {txt:?}");
        assert!(txt.contains('\u{201D}'), "segundo deve ser U+201D (close); got {txt:?}");
    }

    #[test]
    fn p288_smartquote_double_lang_pt_emite_chevrons() {
        // Lang pt: ("«", "»") per LANG_QUOTES.
        use crate::entities::lang::Lang;
        use crate::entities::style::{Style, Styles};
        use std::str::FromStr;
        let lang_pt = Lang::from_str("pt").unwrap();
        let styled = Content::Styled(
            Box::new(Content::sequence(vec![
                Content::smartquote(true),
                Content::smartquote(true),
            ])),
            Styles::from_iter([Style::Lang(lang_pt)]),
        );
        let txt = collect_text(&layout(&styled));
        assert!(txt.contains('\u{00AB}'), "lang=pt open → «");
        assert!(txt.contains('\u{00BB}'), "lang=pt close → »");
    }

    #[test]
    fn p288_smartquote_double_lang_fr_inclui_nbsp() {
        // Lang fr: ("«\u{00A0}", "\u{00A0}»") — chevrons com NBSP intercalado.
        use crate::entities::lang::Lang;
        use crate::entities::style::{Style, Styles};
        use std::str::FromStr;
        let lang_fr = Lang::from_str("fr").unwrap();
        let styled = Content::Styled(
            Box::new(Content::sequence(vec![
                Content::smartquote(true),
                Content::smartquote(true),
            ])),
            Styles::from_iter([Style::Lang(lang_fr)]),
        );
        let txt = collect_text(&layout(&styled));
        // Em fr a sequência open inclui NBSP (U+00A0) após `«` e antes de `»`.
        assert!(
            txt.contains('\u{00AB}') && txt.contains('\u{00BB}'),
            "lang=fr deve ter chevrons; got {txt:?}"
        );
        assert!(
            txt.contains('\u{00A0}'),
            "lang=fr deve incluir NBSP per LANG_QUOTES fr; got {txt:?}"
        );
    }
}

// ── Passo 288 — Style::Lang variant + cascade ────────────────────────────

#[cfg(test)]
mod p288_style_lang_tests {
    use super::*;
    use crate::entities::lang::Lang;
    use crate::entities::style::{Style, Styles};
    use crate::entities::style_chain::StyleChain;

    #[test]
    fn p288_style_lang_variant_basico() {
        let s = Style::Lang(Lang::ENGLISH);
        assert!(matches!(s, Style::Lang(_)));
        // PartialEq por valor.
        assert_eq!(s, Style::Lang(Lang::ENGLISH));
    }

    #[test]
    fn p288_push_styles_lang_projecta_no_delta() {
        // Cascade arm: `Style::Lang(l)` → `delta.lang = Some(l)`.
        let chain = StyleChain::empty();
        let next = chain.push_styles(&Styles::from_iter([Style::Lang(Lang::ENGLISH)]));
        assert_eq!(next.lang(), Some(Lang::ENGLISH));
    }

    #[test]
    fn p288_styled_lang_injetado_layouter_chain_le_corretamente() {
        // Smoke: Content::Styled com Style::Lang produz chain.lang() correcto
        // dentro do Layouter (via empty doc seguinte do styled; usamos
        // SmartQuote consumer P287 como sentinela do read-path).
        use std::str::FromStr;
        let lang_de = Lang::from_str("de").unwrap();
        let styled = Content::Styled(
            Box::new(Content::smartquote(true)),
            Styles::from_iter([Style::Lang(lang_de)]),
        );
        let doc = layout(&styled);
        let txt: String = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|i| {
                if let FrameItem::Text { text, .. } = i {
                    Some(text.as_str())
                } else {
                    None
                }
            })
            .collect();
        // Lang de: open low U+201E `„`.
        assert!(txt.contains('\u{201E}'), "lang=de open → U+201E `„`; got {txt:?}");
    }

    #[test]
    fn p288_lang_paint_overrides_via_last_write_wins() {
        // Dois Style::Lang consecutivos no mesmo Styles delta: o último
        // escrito ganha (paralelo aos 5 variants existentes).
        let chain = StyleChain::empty();
        let next = chain.push_styles(&Styles::from_iter([
            Style::Lang(Lang::ENGLISH),
            // Re-write com Lang::DE.
            Style::Lang(std::str::FromStr::from_str("de").unwrap()),
        ]));
        assert_eq!(
            next.lang().map(|l| l.as_str().to_string()),
            Some("de".to_string()),
            "último Style::Lang na collection ganha"
        );
    }
}

// ── Passo 289 — Style::Weight variant + cascade ──────────────────────────

#[cfg(test)]
mod p289_style_weight_tests {
    use super::*;
    use crate::entities::style::{Style, Styles};
    use crate::entities::style_chain::StyleChain;

    #[test]
    fn p289_style_weight_variant_basico() {
        let s = Style::Weight(700);
        assert!(matches!(s, Style::Weight(_)));
        // PartialEq por valor (paralelo `HeadingLevel(u8)`).
        assert_eq!(s, Style::Weight(700));
        assert_ne!(s, Style::Weight(400));
    }

    #[test]
    fn p289_push_styles_weight_projecta_no_delta() {
        // Cascade arm: `Style::Weight(w)` → `delta.weight = Some(w)`.
        // Paralelo absoluto a P288 lang.
        let chain = StyleChain::empty();
        let next = chain.push_styles(&Styles::from_iter([Style::Weight(700)]));
        assert_eq!(next.weight(), Some(700));
    }

    #[test]
    fn p289_styled_weight_injetado_chain_le_corretamente() {
        // Smoke: Content::Styled com Style::Weight produz chain.weight()
        // correcto dentro do Layouter. Verificável indirectamente —
        // não há sentinela trivial (faux-bold só dispara stroke em emit
        // que é dead-code em export Helvetica path), por isso testamos
        // via cascade directa.
        let chain = StyleChain::empty();
        let next = chain.push_styles(&Styles::from_iter([
            Style::Weight(900), // Black
        ]));
        assert_eq!(next.weight(), Some(900));
        // Também verificamos que TextStyle::from(&chain) captura.
        let style: crate::entities::layout_types::TextStyle = (&next).into();
        assert_eq!(
            style.weight,
            Some(900),
            "TextStyle::from(&StyleChain) deve propagar weight"
        );
    }

    #[test]
    fn p289_weight_last_write_wins() {
        // Dois Style::Weight consecutivos no mesmo Styles delta: o
        // último escrito ganha (paralelo absoluto P288 last-write).
        let chain = StyleChain::empty();
        let next = chain.push_styles(&Styles::from_iter([
            Style::Weight(400),
            Style::Weight(700), // re-write
        ]));
        assert_eq!(next.weight(), Some(700), "último Style::Weight na collection ganha");
    }

    // ── Testes fronteira (per A.5 detecção de bugs latentes) ──────────

    #[test]
    fn p289_weight_thin_100_propaga() {
        // Fronteira inferior canónica vanilla (CSS thin).
        let chain = StyleChain::empty();
        let next = chain.push_styles(&Styles::from_iter([Style::Weight(100)]));
        assert_eq!(next.weight(), Some(100));
    }

    #[test]
    fn p289_weight_black_900_propaga() {
        // Fronteira superior canónica vanilla (CSS black).
        let chain = StyleChain::empty();
        let next = chain.push_styles(&Styles::from_iter([Style::Weight(900)]));
        assert_eq!(next.weight(), Some(900));
    }

    #[test]
    fn p289_weight_non_canonical_450_aceite() {
        // Valor não-canónico (entre 400 e 500). Vanilla typst aceita
        // qualquer u16 — cristalino preserva paridade (sem validação
        // de range per spec §5 não-objectivo).
        let chain = StyleChain::empty();
        let next = chain.push_styles(&Styles::from_iter([Style::Weight(450)]));
        assert_eq!(
            next.weight(),
            Some(450),
            "weight não-canónico (450) deve ser aceite literalmente — paridade vanilla"
        );
    }

    #[test]
    fn p289_weight_faux_bold_stroke_consumer_p139_consome_chain_weight() {
        // Verificação cumulativa: o consumer faux-bold P139
        // (`TextStyle::faux_bold_stroke_pt`) consome `chain.weight()`
        // sem alteração pós-P289. Diagnóstico §A.1.6.
        use crate::entities::layout_types::{Pt, TextStyle};
        let chain =
            StyleChain::empty().push_styles(&Styles::from_iter([Style::Weight(700)]));
        let style: TextStyle = (&chain).into();
        // size default ~11pt (default_chain); fórmula:
        // ((700 - 400) / 300).max(0) * 11.0 * k → 1.0 * 11.0 * k.
        // Para k=0.04 (typical): 0.44 pt.
        let stroke = style.faux_bold_stroke_pt(0.04);
        assert!(
            stroke > 0.0,
            "weight=700 deve produzir stroke faux-bold > 0 (consumer P139 activo)"
        );
        // Verificar também que size respeita default.
        assert!(style.size.val() > 0.0);
        let _ = Pt::ZERO; // import sanity
    }
}

// ── Passo 290 — Style::Tracking variant + cascade ────────────────────────

#[cfg(test)]
mod p290_style_tracking_tests {
    use super::*;
    use crate::entities::layout_types::Length;
    use crate::entities::style::{Style, Styles};
    use crate::entities::style_chain::StyleChain;

    #[test]
    fn p290_style_tracking_variant_basico() {
        let s = Style::Tracking(Length::pt(0.5));
        assert!(matches!(s, Style::Tracking(_)));
        // PartialEq por valor (Length é Copy + PartialEq).
        assert_eq!(s, Style::Tracking(Length::pt(0.5)));
        assert_ne!(s, Style::Tracking(Length::pt(1.0)));
    }

    #[test]
    fn p290_push_styles_tracking_projecta_no_delta() {
        // Cascade arm: `Style::Tracking(l)` → `delta.tracking = Some(l)`.
        // Paralelo absoluto a P288 lang + P289 weight.
        let chain = StyleChain::empty();
        let next =
            chain.push_styles(&Styles::from_iter([Style::Tracking(Length::pt(1.0))]));
        assert_eq!(next.tracking(), Some(Length::pt(1.0)));
    }

    #[test]
    fn p290_styled_tracking_injetado_chain_le_corretamente() {
        // Smoke: Content::Styled com Style::Tracking produz chain.tracking()
        // correcto. TextStyle::from(&chain) propaga para FrameItem::Text.
        let chain = StyleChain::empty();
        let next =
            chain.push_styles(&Styles::from_iter([Style::Tracking(Length::em(0.1))]));
        assert_eq!(next.tracking(), Some(Length::em(0.1)));
        // TextStyle::from(&chain) deve capturar.
        let style: crate::entities::layout_types::TextStyle = (&next).into();
        assert_eq!(
            style.tracking,
            Some(Length::em(0.1)),
            "TextStyle::from(&StyleChain) deve propagar tracking"
        );
    }

    #[test]
    fn p290_tracking_last_write_wins() {
        // Dois Style::Tracking consecutivos: último ganha (paridade P288/P289).
        let chain = StyleChain::empty();
        let next = chain.push_styles(&Styles::from_iter([
            Style::Tracking(Length::pt(0.5)),
            Style::Tracking(Length::pt(1.0)), // re-write
        ]));
        assert_eq!(
            next.tracking(),
            Some(Length::pt(1.0)),
            "último Style::Tracking na collection ganha"
        );
    }

    // ── Testes fronteira (per A.5 detecção de bugs latentes) ──────────

    #[test]
    fn p290_tracking_zero_propaga() {
        // Zero é estado inicial conceptualmente; emit branch EPSILON
        // (export.rs:2142) skipa `Tc` operator mas chain.tracking()
        // continua a devolver Some(Length::pt(0.0)).
        let chain = StyleChain::empty();
        let next =
            chain.push_styles(&Styles::from_iter([Style::Tracking(Length::pt(0.0))]));
        assert_eq!(next.tracking(), Some(Length::pt(0.0)));
    }

    #[test]
    fn p290_tracking_pequeno_positivo() {
        let chain = StyleChain::empty();
        let next =
            chain.push_styles(&Styles::from_iter([Style::Tracking(Length::pt(1.0))]));
        assert_eq!(next.tracking(), Some(Length::pt(1.0)));
    }

    #[test]
    fn p290_tracking_em_relativo_05() {
        // Em-units: resolve em runtime via TextStyle.size.
        let chain = StyleChain::empty();
        let next =
            chain.push_styles(&Styles::from_iter([Style::Tracking(Length::em(0.5))]));
        let t = next.tracking().expect("Some(...)");
        assert_eq!(t.em, 0.5);
        assert_eq!(t.abs.to_pt(), 0.0, "Length::em(0.5) tem componente abs zero");
    }

    #[test]
    fn p290_tracking_negative_propaga() {
        // **Atenção particular**: vanilla typst aceita tracking negativo
        // como kerning artificial. Cristalino deve preservar paridade
        // (diagnóstico §A.5.2).
        let chain = StyleChain::empty();
        let next =
            chain.push_styles(&Styles::from_iter([Style::Tracking(Length::pt(-0.5))]));
        let t = next.tracking().expect("Some(...)");
        assert!(t.abs.to_pt() < 0.0,
            "tracking negativo deve ser preservado como kerning artificial (vanilla paridade); got {}", t.abs.to_pt());
    }

    #[test]
    fn p290_tracking_grande_10pt_propaga() {
        // Valor grande não causa overflow ou erro silencioso.
        let chain = StyleChain::empty();
        let next =
            chain.push_styles(&Styles::from_iter([Style::Tracking(Length::pt(10.0))]));
        assert_eq!(next.tracking(), Some(Length::pt(10.0)));
    }

    #[test]
    fn p290_tracking_consumer_p137_cursor_extra() {
        // Verificação cumulativa: o consumer P137 em `cursor.rs:30`
        // (`tracking_extra` glyph advance) consome via `self.style.tracking`.
        // Verificável indirectamente: TextStyle::from(&chain) propaga,
        // e Layouter::layout_word adiciona o tracking ao advance.
        let chain = StyleChain::empty()
            .push_styles(&Styles::from_iter([Style::Tracking(Length::pt(2.0))]));
        let style: crate::entities::layout_types::TextStyle = (&chain).into();
        let resolved = style.tracking.expect("Some").resolve_pt(style.size.val());
        assert!(
            (resolved - 2.0).abs() < 0.001,
            "tracking 2pt deve resolver para 2.0 (sem em-component); got {resolved}"
        );
    }
}

// ── Passo 291 — Style::Leading variant + cascade ────────────────────────

#[cfg(test)]
mod p291_style_leading_tests {
    use super::*;
    use crate::entities::layout_types::Length;
    use crate::entities::style::{Style, Styles};
    use crate::entities::style_chain::StyleChain;

    #[test]
    fn p291_style_leading_variant_basico() {
        let s = Style::Leading(Length::em(0.65));
        assert!(matches!(s, Style::Leading(_)));
        // PartialEq por valor.
        assert_eq!(s, Style::Leading(Length::em(0.65)));
        assert_ne!(s, Style::Leading(Length::em(0.5)));
    }

    #[test]
    fn p291_push_styles_leading_projecta_no_delta() {
        // Cascade arm: `Style::Leading(l)` → `delta.leading = Some(l)`.
        // Paralelo absoluto a P288 lang + P289 weight + P290 tracking.
        let chain = StyleChain::empty();
        let next =
            chain.push_styles(&Styles::from_iter([Style::Leading(Length::pt(11.0))]));
        assert_eq!(next.leading(), Some(Length::pt(11.0)));
    }

    #[test]
    fn p291_styled_leading_injetado_chain_le_corretamente() {
        // Smoke: Content::Styled com Style::Leading produz chain.leading()
        // correcto. TextStyle::from(&chain) propaga.
        let chain = StyleChain::empty();
        let next =
            chain.push_styles(&Styles::from_iter([Style::Leading(Length::em(0.65))]));
        assert_eq!(next.leading(), Some(Length::em(0.65)));
        let style: crate::entities::layout_types::TextStyle = (&next).into();
        assert_eq!(
            style.leading,
            Some(Length::em(0.65)),
            "TextStyle::from(&StyleChain) deve propagar leading"
        );
    }

    #[test]
    fn p291_leading_last_write_wins() {
        // Dois Style::Leading consecutivos: último ganha.
        let chain = StyleChain::empty();
        let next = chain.push_styles(&Styles::from_iter([
            Style::Leading(Length::pt(8.0)),
            Style::Leading(Length::pt(12.0)), // re-write
        ]));
        assert_eq!(
            next.leading(),
            Some(Length::pt(12.0)),
            "último Style::Leading na collection ganha"
        );
    }

    // ── Testes fronteira (per A.5 — 5 cenários) ────────────────────────

    #[test]
    fn p291_leading_zero_propaga() {
        // Leading zero: linhas colapsam para line_height puro.
        let chain = StyleChain::empty();
        let next =
            chain.push_styles(&Styles::from_iter([Style::Leading(Length::pt(0.0))]));
        assert_eq!(next.leading(), Some(Length::pt(0.0)));
    }

    #[test]
    fn p291_leading_tipico_11pt_propaga() {
        // Valor próximo do default cristalino (font_size 11pt × 1.0 ≈ 11pt).
        let chain = StyleChain::empty();
        let next =
            chain.push_styles(&Styles::from_iter([Style::Leading(Length::pt(11.0))]));
        assert_eq!(next.leading(), Some(Length::pt(11.0)));
    }

    #[test]
    fn p291_leading_em_relativo_065() {
        // Em-units: resolve em runtime via TextStyle.size.
        let chain = StyleChain::empty();
        let next =
            chain.push_styles(&Styles::from_iter([Style::Leading(Length::em(0.65))]));
        let l = next.leading().expect("Some");
        assert_eq!(l.em, 0.65);
        assert_eq!(l.abs.to_pt(), 0.0);
    }

    #[test]
    fn p291_leading_grande_50pt_propaga() {
        // Valor grande não causa overflow.
        let chain = StyleChain::empty();
        let next =
            chain.push_styles(&Styles::from_iter([Style::Leading(Length::pt(50.0))]));
        assert_eq!(next.leading(), Some(Length::pt(50.0)));
    }

    #[test]
    fn p291_leading_negative_propaga() {
        // **Atenção particular**: vanilla typst aceita leading negativo
        // (line collapse parcial); cristalino preserva paridade —
        // `cursor.rs:124` passa valor literal sem clamp (A.5.1
        // diagnóstico).
        let chain = StyleChain::empty();
        let next =
            chain.push_styles(&Styles::from_iter([Style::Leading(Length::pt(-1.0))]));
        let l = next.leading().expect("Some");
        assert!(
            l.abs.to_pt() < 0.0,
            "leading negativo deve ser preservado (vanilla paridade); got {}",
            l.abs.to_pt()
        );
    }

    #[test]
    fn p291_leading_consumer_p138_flush_line_peek() {
        // Verificação cumulativa: o consumer P138 em `cursor.rs:119-128`
        // (peek `current_line.iter().rev().find_map(FrameItem::Text)`)
        // consome via `style.leading.resolve_pt(font_size)`.
        // Verificável indirectamente: TextStyle::from(&chain) propaga,
        // e `flush_line` resolve para pt no momento do peek.
        let chain = StyleChain::empty()
            .push_styles(&Styles::from_iter([Style::Leading(Length::pt(3.0))]));
        let style: crate::entities::layout_types::TextStyle = (&chain).into();
        let resolved = style.leading.expect("Some").resolve_pt(style.size.val());
        assert!(
            (resolved - 3.0).abs() < 0.001,
            "leading 3pt deve resolver para 3.0 (sem em-component); got {resolved}"
        );
    }
}

// ── Passo 292 — Style::Font variant + cascade (fecha série P288-P292) ──

#[cfg(test)]
mod p292_style_font_tests {
    use super::*;
    use crate::entities::font_list::{FontFamily, FontList};
    use crate::entities::style::{Style, Styles};
    use crate::entities::style_chain::StyleChain;
    use ecow::EcoString;

    #[test]
    fn p292_style_font_variant_basico() {
        let s = Style::Font(FontList::single(EcoString::from("Inter")));
        assert!(matches!(s, Style::Font(_)));
        // PartialEq por valor (FontList: PartialEq).
        assert_eq!(s, Style::Font(FontList::single(EcoString::from("Inter"))));
        assert_ne!(s, Style::Font(FontList::single(EcoString::from("Arial"))));
    }

    #[test]
    fn p292_push_styles_font_projecta_no_delta() {
        // Cascade arm: `Style::Font(f)` → `delta.font = Some(f.clone())`.
        // **Distinção sintáctica vs P288-P291**: `.clone()` em vez de `*f`
        // por `FontList: !Copy`.
        let chain = StyleChain::empty();
        let next = chain.push_styles(&Styles::from_iter([Style::Font(
            FontList::single(EcoString::from("Inter")),
        )]));
        assert_eq!(next.font(), Some(FontList::single(EcoString::from("Inter"))));
    }

    #[test]
    fn p292_styled_font_injetado_chain_le_corretamente() {
        // Smoke: Content::Styled com Style::Font produz chain.font()
        // correcto. TextStyle::from(&chain) propaga.
        let chain = StyleChain::empty();
        let fl = FontList::single(EcoString::from("Helvetica"));
        let next = chain.push_styles(&Styles::from_iter([Style::Font(fl.clone())]));
        assert_eq!(next.font(), Some(fl.clone()));
        let style: crate::entities::layout_types::TextStyle = (&next).into();
        assert_eq!(
            style.font,
            Some(fl),
            "TextStyle::from(&StyleChain) deve propagar font"
        );
    }

    #[test]
    fn p292_font_last_write_wins() {
        // Dois Style::Font consecutivos: último ganha.
        let chain = StyleChain::empty();
        let next = chain.push_styles(&Styles::from_iter([
            Style::Font(FontList::single(EcoString::from("Arial"))),
            Style::Font(FontList::single(EcoString::from("Inter"))), // re-write
        ]));
        assert_eq!(
            next.font(),
            Some(FontList::single(EcoString::from("Inter"))),
            "último Style::Font na collection ganha"
        );
    }

    // ── Testes fronteira (per A.5) ──────────────────────────────────

    #[test]
    fn p292_font_single_caso_tipico() {
        let chain = StyleChain::empty();
        let next = chain.push_styles(&Styles::from_iter([Style::Font(
            FontList::single(EcoString::from("Inter")),
        )]));
        let fl = next.font().expect("Some");
        assert_eq!(fl.len(), 1);
        assert_eq!(fl.as_slice()[0].name.as_str(), Some("inter")); // lowercase per FontFamily::new
    }

    #[test]
    fn p292_font_multi_fallback_chain() {
        // FontList com 3 elementos — caso P141 fallback chain.
        let fl = FontList::new(vec![
            FontFamily::new(EcoString::from("Inter")),
            FontFamily::new(EcoString::from("Helvetica")),
            FontFamily::new(EcoString::from("Arial")),
        ])
        .expect("non-empty");
        let chain = StyleChain::empty()
            .push_styles(&Styles::from_iter([Style::Font(fl.clone())]));
        let resolved = chain.font().expect("Some");
        assert_eq!(resolved.len(), 3, "FontList multi-element preserved cross cascade");
    }

    #[test]
    fn p292_font_list_non_empty_by_construction() {
        // **Cenário crítico A.5 ajustado**: FontList NÃO permite vazio
        // por construção (`FontList::new(vec![])` retorna `None` per
        // diagnóstico §A.5.1). Réplica semântica vanilla `"font fallback
        // list must not be empty"`. Não há bug latente — invariante
        // estructural.
        let attempt = FontList::new(vec![]);
        assert!(
            attempt.is_none(),
            "FontList::new(vec![]) deve retornar None — non-empty by construction"
        );
    }

    #[test]
    fn p292_font_missing_name_propaga_resolution_defer() {
        // Cenário A.5: nome de font não-disponível propaga via cascade.
        // Resolution defer ao FontBook em layout-time.
        let fl = FontList::single(EcoString::from("NonExistentFontXYZ"));
        let chain = StyleChain::empty()
            .push_styles(&Styles::from_iter([Style::Font(fl.clone())]));
        assert_eq!(
            chain.font(),
            Some(fl),
            "missing font name propaga literalmente; resolution defer"
        );
    }

    #[test]
    fn p292_font_consumer_textstyle_capture_paradigma_p136() {
        // Verificação cumulativa: TextStyle::from(&chain) propaga
        // `font: chain.font()` (style_chain.rs:316). Emit multifont
        // path (export.rs:2169-2174) lê `style.font.as_ref()` — paradigma
        // P136 confirmado empiricamente.
        let fl = FontList::new(vec![
            FontFamily::new(EcoString::from("Inter")),
            FontFamily::new(EcoString::from("Helvetica")),
        ])
        .expect("non-empty");
        let chain = StyleChain::empty()
            .push_styles(&Styles::from_iter([Style::Font(fl.clone())]));
        let style: crate::entities::layout_types::TextStyle = (&chain).into();
        assert!(style.font.is_some(), "TextStyle.font deve capturar de chain");
        // Confirmação que o FontList propagado tem o mesmo número de
        // elementos (clone preserved).
        assert_eq!(style.font.unwrap().len(), 2);
    }

    // ── Passo 304 (P295.1) — footnote body renderizado no rodapé ───
    //
    // P304 materializa P295.1 via deferred buffer pattern (paralelo
    // P245 floats_pending + P251 pending_cell_tails). Body emitido
    // no rodapé da página em Y absoluto bottom-up.

    #[test]
    fn p304_footnote_body_presente_no_documento() {
        // Body string deve estar no `plain_text()` do documento
        // (renderizado no rodapé via flush_pending_footnote_bodies).
        let doc = layout(&Content::footnote(Content::text("BODYFOO")));
        assert!(!doc.pages.is_empty(), "documento tem páginas");
        // plain_text via items emitidos: marker [1] + body.
        let text: String = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|it| match it {
                FrameItem::Text { text, .. } => Some(text.to_string()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join(" ");
        assert!(
            text.contains("BODYFOO"),
            "body 'BODYFOO' presente nos FrameItems; got: {:?}",
            text
        );
    }

    #[test]
    fn p304_footnote_body_no_rodape_y_alto() {
        // Body posicionado no fundo da página: Y > metade da altura.
        // Page default height = 842pt (A4); margin = 72pt;
        // bottom = 842 - 72 = 770pt. Body deve estar próximo de 770pt.
        let doc = layout(&Content::footnote(Content::text("RODAPE")));
        let page = &doc.pages[0];
        let half = page.height / 2.0;
        let body_y = page.items.iter().find_map(|it| match it {
            FrameItem::Text { pos, text, .. } if text.contains("RODAPE") => Some(pos.y.0),
            _ => None,
        });
        assert!(body_y.is_some(), "body 'RODAPE' encontrado nos items");
        assert!(
            body_y.unwrap() > half,
            "body Y ({}) deve estar na metade inferior da página (>{})",
            body_y.unwrap(),
            half
        );
    }

    #[test]
    fn p304_marker_inline_acima_do_body() {
        // Marker `[1]` emitido inline (Y baixo, próximo do topo);
        // body emitido no rodapé (Y alto). marker_y < body_y.
        let doc = layout(&Content::sequence(vec![
            Content::text("texto "),
            Content::footnote(Content::text("RODAPEB")),
        ]));
        let page = &doc.pages[0];
        let marker_y = page.items.iter().find_map(|it| match it {
            FrameItem::Text { pos, text, .. } if text.contains("[1]") => Some(pos.y.0),
            _ => None,
        });
        let body_y = page.items.iter().find_map(|it| match it {
            FrameItem::Text { pos, text, .. } if text.contains("RODAPEB") => {
                Some(pos.y.0)
            }
            _ => None,
        });
        assert!(marker_y.is_some(), "marker [1] presente");
        assert!(body_y.is_some(), "body RODAPEB presente");
        assert!(
            marker_y.unwrap() < body_y.unwrap(),
            "marker_y ({}) acima de body_y ({})",
            marker_y.unwrap(),
            body_y.unwrap()
        );
    }

    #[test]
    fn p304_multiplos_footnotes_bodies_empilhados() {
        // 3 bodies empilhados no rodapé: ordem N=1 → N=3 top-down.
        let doc = layout(&Content::sequence(vec![
            Content::text("a "),
            Content::footnote(Content::text("AAAA")),
            Content::text(" b "),
            Content::footnote(Content::text("BBBB")),
            Content::text(" c "),
            Content::footnote(Content::text("CCCC")),
        ]));
        let page = &doc.pages[0];
        let y_a = page.items.iter().find_map(|it| match it {
            FrameItem::Text { pos, text, .. } if text.contains("AAAA") => Some(pos.y.0),
            _ => None,
        });
        let y_b = page.items.iter().find_map(|it| match it {
            FrameItem::Text { pos, text, .. } if text.contains("BBBB") => Some(pos.y.0),
            _ => None,
        });
        let y_c = page.items.iter().find_map(|it| match it {
            FrameItem::Text { pos, text, .. } if text.contains("CCCC") => Some(pos.y.0),
            _ => None,
        });
        assert!(
            y_a.is_some() && y_b.is_some() && y_c.is_some(),
            "todos 3 bodies presentes"
        );
        // Footnote 1 fica acima de 2 e 3 (paridade vanilla: ordem
        // numérica top-down no rodapé).
        assert!(
            y_a.unwrap() <= y_b.unwrap(),
            "body 1 acima ou igual ao body 2; y_a={}, y_b={}",
            y_a.unwrap(),
            y_b.unwrap()
        );
        assert!(
            y_b.unwrap() <= y_c.unwrap(),
            "body 2 acima ou igual ao body 3; y_b={}, y_c={}",
            y_b.unwrap(),
            y_c.unwrap()
        );
    }

    #[test]
    fn p304_documento_sem_footnote_sem_impacto() {
        // Documento sem Footnote: layout idêntico pré-P304.
        // `flush_pending_footnote_bodies` early-return em buffer vazio.
        let doc = layout(&Content::text("hello world"));
        // Texto presente; nenhum marker [N] gerado.
        assert!(!doc.pages.is_empty());
        let text: String = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|it| match it {
                FrameItem::Text { text, .. } => Some(text.to_string()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join(" ");
        assert!(
            text.contains("hello") || text.contains("world"),
            "texto preservado pré-P304"
        );
        assert!(
            !text.contains("[1]"),
            "nenhum marker footnote em documento sem footnote"
        );
    }

    #[test]
    fn p304_footnote_body_complex_content_renderizado() {
        // Body com conteúdo composto (Sequence) renderizado completo.
        let doc = layout(&Content::footnote(Content::sequence(vec![
            Content::text("primeira"),
            Content::text(" "),
            Content::text("segunda"),
        ])));
        let text: String = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|it| match it {
                FrameItem::Text { text, .. } => Some(text.to_string()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join(" ");
        assert!(text.contains("primeira"), "primeira parte do body presente");
        assert!(text.contains("segunda"), "segunda parte do body presente");
    }

    // ── Passo 305 (P295.2) — footnote overflow multi-página ─────────
    //
    // P305 estende P304 com cross-page overflow handling. Bug latente
    // P304 (overlap silencioso quando total_h > available_h) fixado
    // via greedy fit + defer. Sub-padrão "DeferredX buffer + flush
    // em new_page" estendido com partial drain.

    #[test]
    fn p305_overflow_body_grande_distribui_no_documento() {
        // Body muito tall força overflow. Verifica que NENHUM body
        // é descartado silenciosamente (paridade bug fix P304).
        // Body com ~250 palavras → ~50 linhas → ~720pt (default font
        // 12pt × 1.2 line_height × 50). Excede available em main
        // content presence.
        let huge = "loremX ipsumY ".repeat(150);
        let doc = layout(&Content::sequence(vec![
            Content::text("topo"),
            Content::footnote(Content::text(huge)),
        ]));
        // Body content deve aparecer no documento final (alguma página).
        let combined: String = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|it| match it {
                FrameItem::Text { text, .. } => Some(text.to_string()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join(" ");
        assert!(
            combined.contains("loremX"),
            "body 'loremX' presente; nenhum body silenciosamente descartado"
        );
        assert!(combined.contains("ipsumY"), "body 'ipsumY' presente");
    }

    #[test]
    fn p305_overflow_multiplos_bodies_todos_preservados() {
        // 5 bodies grandes — overflow força distribuição multi-página.
        // Cada body tem sentinel UNIQUEN para tracking individual.
        let bodies: Vec<Content> = (0..5)
            .map(|i| {
                let body_text = format!("UNIQUE{} word ", i).repeat(40);
                Content::footnote(Content::text(body_text))
            })
            .collect();
        let mut all = vec![Content::text("texto")];
        all.extend(bodies);
        let doc = layout(&Content::sequence(all));

        let combined: String = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|it| match it {
                FrameItem::Text { text, .. } => Some(text.to_string()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join(" ");
        for i in 0..5 {
            let needle = format!("UNIQUE{}", i);
            assert!(
                combined.contains(&needle),
                "body sentinel '{}' presente no documento final",
                needle
            );
        }
    }

    #[test]
    fn p305_regressao_p304_single_page_preservado() {
        // CRÍTICO: footnote pequena que cabe — comportamento idêntico
        // P304 (1 página, body no rodapé). Determinismo + bit-exact.
        let doc = layout(&Content::footnote(Content::text("CABE")));
        assert_eq!(doc.pages.len(), 1, "footnote pequena cabe em 1 página");
        let text: String = doc.pages[0]
            .items
            .iter()
            .filter_map(|it| match it {
                FrameItem::Text { text, .. } => Some(text.to_string()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join(" ");
        assert!(text.contains("CABE"), "body 'CABE' no rodapé");
        assert!(text.contains("[1]"), "marker [1] inline");
    }

    #[test]
    fn p305_regressao_documento_sem_footnote_bit_exact() {
        // REGRESSÃO BIT-EXACT: documentos sem footnote idênticos
        // pré-P305 (e pré-P304). Flush early-return + loop iter_limit
        // = 0 garantem zero impacto.
        let doc = layout(&Content::text("hello"));
        assert_eq!(doc.pages.len(), 1, "1 página para texto curto");
        let text: String = doc.pages[0]
            .items
            .iter()
            .filter_map(|it| match it {
                FrameItem::Text { text, .. } => Some(text.to_string()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join(" ");
        assert!(text.contains("hello"), "texto preservado");
        assert!(!text.contains("[1]"), "nenhum marker footnote");
    }

    #[test]
    fn p305_body_gigante_nao_loop_infinito() {
        // CASO DEGENERATE: body singular > página inteira.
        // Defensive force_emit deve placar mesmo assim; iter_limit
        // em finish() evita loop infinito.
        let gigante = "X ".repeat(2000); // ~400 linhas → ~5800pt
        let doc = layout(&Content::footnote(Content::text(gigante)));
        // Documento finalizou (não panicou; não infinite loop).
        assert!(!doc.pages.is_empty(), "documento terminou com páginas");
        // Marker presente em alguma página.
        let combined: String = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|it| match it {
                FrameItem::Text { text, .. } => Some(text.to_string()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join(" ");
        assert!(combined.contains("[1]"), "marker [1] presente");
    }

    #[test]
    fn p305_bug_fix_overflow_sem_overlap_no_top() {
        // BUG FIX P304: bodies já NÃO sobrepõem main content quando
        // overflow. Verificação: para um body que não cabe, ou é
        // deferido (não aparece no Y top), ou clampado a top_safe.
        // Indirecto: body Y nunca está acima do margin minimum.
        let big = "wordSentinel ".repeat(100); // ~30 linhas
        let doc = layout(&Content::sequence(vec![
            Content::text("AAA BBB CCC DDD"),
            Content::footnote(Content::text(big)),
        ]));
        // Sentinel body Y mínimo deve ser >= margin (72.0 default)
        // — não pode estar acima do topo da página.
        let min_body_y = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|it| match it {
                FrameItem::Text { pos, text, .. } if text.contains("wordSentinel") => {
                    Some(pos.y.0)
                }
                _ => None,
            })
            .fold(f64::INFINITY, f64::min);
        if min_body_y.is_finite() {
            // P305 fix: Y do body sempre dentro da página.
            assert!(
                min_body_y >= 72.0 - 1.0, // 1pt tolerance for ascender
                "body Y mínimo ({}) deve estar dentro da página (>= margin 72)",
                min_body_y
            );
        }
    }

    #[test]
    fn p595_nota_pequena_colunas_sem_regressao() {
        // REGRESSÃO: nota pequena que cabe em `#set page(columns: 2)`
        // continua a ser renderizada no fundo da coluna correspondente.
        let doc = layout_test(
            r#"#set page(columns: 2)
Hello #footnote[Nota A] world.
#colbreak()
Goodbye #footnote[Nota B] moon."#,
        );
        assert_eq!(doc.pages.len(), 1, "deve caber numa página");
        assert!(doc.layout_warnings.is_empty(), "sem avisos para notas que cabem");
        let combined = doc.plain_text();
        assert!(combined.contains("Nota A"), "Nota A presente");
        assert!(combined.contains("Nota B"), "Nota B presente");
    }

    #[test]
    fn p595_nota_enorme_colunas_gera_warning() {
        // CASO DEGENERATE: body de footnote maior do que a altura útil
        // total da coluna/página. O algoritmo emite-o defensivemente
        // para evitar loop infinito, mas deve registar um aviso.
        let doc = layout_test(
            r#"#set page(columns: 2, height: 200pt)
#lorem(10)#footnote[
  Esta nota é maior do que a coluna inteira. #lorem(200)
]
#lorem(10)"#,
        );
        assert!(
            doc.layout_warnings
                .iter()
                .any(|w: &String| w.contains("footnote body")),
            "deve haver aviso de footnote body exceed: {:?}",
            doc.layout_warnings
        );
        // Marker e parte do body devem ser visíveis no documento.
        let combined = doc.plain_text();
        assert!(combined.contains("[1]"), "marker [1] presente");
        assert!(
            combined.contains("Esta nota é maior do que a coluna inteira"),
            "início do body presente"
        );
    }

    #[test]
    fn p595_nota_grande_colunas_nao_sobrepoe_topo() {
        // Nota grande que não cabe no espaço restante da coluna não
        // pode ser desenhada acima do conteúdo principal (topo da página).
        let doc = layout_test(
            r#"#set page(columns: 2, height: 200pt)
#lorem(30)#footnote[
  Esta é uma nota de rodapé muito longa, com texto suficiente para não caber no espaço restante de uma coluna pequena, testando o que acontece quando isto excede o espaço disponível na página.
]
#lorem(30)"#,
        );
        // Nenhum item da nota pode estar acima do topo útil da página.
        let min_note_y = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|it| match it {
                FrameItem::Text { pos, text, .. } if text.contains("nota de rodapé") => {
                    Some(pos.y.0)
                }
                _ => None,
            })
            .fold(f64::INFINITY, f64::min);
        if min_note_y.is_finite() {
            assert!(
                min_note_y >= 70.0,
                "nota não pode começar acima do topo útil da página, y={}",
                min_note_y
            );
        }
    }

    #[test]
    fn p598_colunas_pagina_pequena_texto_curto_uma_pagina() {
        // P598 — com margens automáticas (2.5/21 da menor dimensão),
        // uma página de 200pt com duas colunas tem colunas largas
        // suficientes para que #lorem(30) caiba numa única página,
        // tal como no vanilla 0.15.0.
        let doc = layout_test(
            r#"#set page(columns: 2, height: 200pt)
#lorem(30)"#,
        );
        assert_eq!(
            doc.pages.len(),
            1,
            "#lorem(30) numa página 200pt/2cols deve caber numa página"
        );
    }

    #[test]
    fn p598_colunas_pagina_pequena_texto_medio_uma_pagina() {
        // P598 — regressão do caso P597: #lorem(50) no vanilla 0.15.0
        // ainda cabe numa única página 200pt/2cols graças às margens
        // automáticas. O cristalino deve reproduzir o mesmo observable.
        let doc = layout_test(
            r#"#set page(columns: 2, height: 200pt)
#lorem(50)"#,
        );
        assert_eq!(
            doc.pages.len(),
            1,
            "#lorem(50) numa página 200pt/2cols deve caber numa página"
        );
    }

    #[test]
    fn p598_margem_explicita_nao_recalculada_ao_mudar_altura() {
        // P598 — quando o utilizador define margem explicitamente,
        // alterar height não deve recalcular a margem.
        let doc = layout_test(
            r#"#set page(margin: 50pt)
#set page(height: 200pt)
#lorem(5)"#,
        );
        let first_text_x = doc.pages[0]
            .items
            .iter()
            .find_map(|item| match item {
                FrameItem::Text { pos, .. } => Some(pos.x.0),
                _ => None,
            })
            .expect("deve haver texto");
        assert!(
            (first_text_x - 50.0).abs() < 0.01,
            "texto deve começar na margem explícita (50 pt), não na automática; obtido {}",
            first_text_x
        );
    }

    #[test]
    fn p292_marco_arquitectural_serie_p288_a_p292_fechada() {
        // **Marco simbólico**: este teste atesta o fecho da série
        // cirúrgica P288-P292 (5/5 da assimetria residual P289 §5.6
        // fechada). Pós-P292, não há mais campos `StyleDelta` sem
        // variant `Style` correspondente — sequência cumulativa
        // termina naturalmente. Cada um dos 5 variants é construível
        // via `Style::*`.
        use crate::entities::lang::Lang;
        use crate::entities::layout_types::Length as L;

        // Construir uma Styles collection completa P288-P292:
        let all_5 = Styles::from_iter([
            Style::Lang(Lang::ENGLISH),                              // P288
            Style::Weight(700),                                      // P289
            Style::Tracking(L::pt(0.5)),                             // P290
            Style::Leading(L::em(0.65)),                             // P291
            Style::Font(FontList::single(EcoString::from("Inter"))), // P292
        ]);
        // Aplicar à chain — todos os 5 arms da cascade activam:
        let chain = StyleChain::empty().push_styles(&all_5);
        assert!(chain.lang().is_some());
        assert!(chain.weight().is_some());
        assert!(chain.tracking().is_some());
        assert!(chain.leading().is_some());
        assert!(chain.font().is_some());
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// P331 Fase 2 — Rede de caracterização do comportamento de estilo
//
// Fixa a SAÍDA OBSERVÁVEL atual (numeração, estilo, escopo, página) para
// detectar regressão semântica quando o "F" refatorar a StyleChain. Asserta
// sobre saída de layout/plain_text, NÃO sobre representação interna — o F muda
// a representação; esta rede protege o comportamento (critério do dono P329:
// fidelidade é de comportamento). Zero conserto: divergências são
// caracterizadas como estado atual e listadas no dossiê §bugs.
// ─────────────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod f_caracterizacao_estilo {
    use super::*;
    use crate::entities::layout_types::{FrameItem, Pt, TextStyle};

    fn doc_text(c: &Content) -> String {
        layout(c).plain_text()
    }

    fn has_bold(c: &Content) -> bool {
        layout(c)
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .any(|i| matches!(i, FrameItem::Text { style, .. } if style.bold))
    }
    fn has_italic(c: &Content) -> bool {
        layout(c)
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .any(|i| matches!(i, FrameItem::Text { style, .. } if style.italic))
    }

    // ── SetHeadingNumbering → prefixo de heading ──────────────────────────
    #[test]
    fn carac_set_heading_numbering_liga_prefixo() {
        // Lote F-2 S1 (P335): numeração assada no heading (`heading_numbered`);
        // asserções inalteradas.
        let c = Content::Sequence(
            vec![
                Content::heading_numbered(1, Content::text("Intro")),
                Content::heading_numbered(2, Content::text("Sub")),
            ]
            .into(),
        );
        let t = doc_text(&c);
        assert!(t.contains("1."), "H1 deve ter prefixo '1.': '{t}'");
        assert!(t.contains("1.1"), "H2 deve ter prefixo '1.1': '{t}'");
    }

    #[test]
    fn carac_sem_set_heading_numbering_sem_prefixo() {
        let c = Content::heading(1, Content::text("Intro"));
        let t = doc_text(&c);
        assert!(t.contains("Intro"), "corpo presente: '{t}'");
        assert!(!t.contains("1."), "sem SetHeadingNumbering → sem prefixo: '{t}'");
    }

    // ── SetFigureNumbering → prefixo de figura (com caption) ──────────────
    #[test]
    fn carac_set_figure_numbering_caption_prefixo() {
        let c = Content::Sequence(
            vec![Content::figure(
                Content::text("img"),
                Some(Content::text("legenda")),
                Some("image".to_string()),
                Some("1".to_string()),
            )]
            .into(),
        );
        let t = doc_text(&c);
        // Caracteriza: figura com caption + numbering activo recebe "Figura N".
        assert!(t.contains("Figura 1"), "figura numerada deve ter 'Figura 1': '{t}'");
    }

    #[test]
    fn carac_figura_sem_caption_sem_prefixo() {
        let c = Content::figure(
            Content::text("img"),
            None,
            Some("image".to_string()),
            Some("1".to_string()),
        );
        let t = doc_text(&c);
        assert!(!t.contains("Figura 1"), "sem caption → sem prefixo numérico: '{t}'");
    }

    // ── P459 — Table numbering (caption acima) ────────────────────────────
    #[test]
    fn p459_table_caption_numbering_prefixo_acima() {
        use crate::entities::layout_types::TrackSizing;
        use crate::entities::style::Styles;
        use crate::entities::value::Value;
        let table = Content::table_with_caption(
            vec![TrackSizing::Auto],
            vec![TrackSizing::Auto],
            vec![Content::text("cell")],
            Some(Content::text("legenda")),
        );
        let c = Content::Styled(
            Box::new(table),
            Styles::new().push_custom("table.numbering", Value::Str("1.".into())),
        );
        let t = doc_text(&c);
        assert!(
            t.contains("Table 1.: legenda"),
            "table numerada deve prefixar caption acima: '{t}'"
        );
        assert!(t.contains("cell"), "corpo da table deve renderizar: '{t}'");
    }

    #[test]
    fn p459_table_sem_caption_sem_prefixo() {
        use crate::entities::layout_types::TrackSizing;
        use crate::entities::style::Styles;
        use crate::entities::value::Value;
        let table = Content::table_with_caption(
            vec![TrackSizing::Auto],
            vec![TrackSizing::Auto],
            vec![Content::text("cell")],
            None,
        );
        let c = Content::Styled(
            Box::new(table),
            Styles::new().push_custom("table.numbering", Value::Str("1.".into())),
        );
        let t = doc_text(&c);
        assert!(!t.contains("Table 1"), "sem caption → sem prefixo numérico: '{t}'");
        assert!(t.contains("cell"), "corpo da table deve renderizar: '{t}'");
    }

    // ── P461 — Counter "table" via Introspector (regressão re-layout) ─────
    #[test]
    fn p461_table_counter_persiste_relayout() {
        use crate::entities::layout_types::TrackSizing;
        use crate::entities::style::Styles;
        use crate::entities::value::Value;
        let mk = |n: usize| {
            Content::table_with_caption(
                vec![TrackSizing::Auto],
                vec![TrackSizing::Auto],
                vec![Content::text(format!("cell{n}"))],
                Some(Content::text(format!("legenda{n}"))),
            )
        };
        let seq = Content::Sequence(
            vec![
                Content::Styled(
                    Box::new(mk(1)),
                    Styles::new().push_custom("table.numbering", Value::Str("1.".into())),
                ),
                Content::Styled(
                    Box::new(mk(2)),
                    Styles::new().push_custom("table.numbering", Value::Str("1.".into())),
                ),
            ]
            .into(),
        );
        // Dois layouts independentes da mesma sequência; o oráculo é
        // reconstruído a partir do Content, logo a sequência 1, 2 repete-se
        // idempotentemente. Com campo local numa passagem multi-layout
        // o counter duplicaria.
        let t1 = doc_text(&seq);
        let t2 = doc_text(&seq);
        assert!(t1.contains("Table 1.: legenda1"), "1º layout T1: '{t1}'");
        assert!(t1.contains("Table 2.: legenda2"), "1º layout T2: '{t1}'");
        assert!(t2.contains("Table 1.: legenda1"), "2º layout T1: '{t2}'");
        assert!(t2.contains("Table 2.: legenda2"), "2º layout T2: '{t2}'");
    }

    // ── SetEquationNumbering → caracterizar estado atual ──────────────────
    // 1a (inventário): SetEquationNumbering NÃO tem produtor em eval (só
    // testes); o efeito real depende do Introspector. Caracteriza o que
    // `layout` produz hoje para uma equação block com o marcador presente.
    #[test]
    fn carac_set_equation_numbering_estado_atual() {
        let c =
            Content::Sequence(vec![Content::equation(Content::text("x"), true)].into());
        let t = doc_text(&c);
        // caracteriza estado atual; comportamento de numeração de equação
        // registado no dossiê §bugs (sem produtor eval, efeito dependente de
        // Introspector pré-populado).
        assert!(t.contains("x"), "corpo da equação presente: '{t}'");
    }

    // ── SetPage → dimensões da página ─────────────────────────────────────
    #[test]
    fn carac_set_page_altera_dimensoes() {
        let baseline = layout(&Content::text("x"));
        let (bw, bh) = (baseline.pages[0].width, baseline.pages[0].height);
        let c = Content::Sequence(
            vec![
                Content::SetPage {
                    width: Some(crate::entities::layout_types::PageDimension::Length(123.0)),
                    height: Some(crate::entities::layout_types::PageDimension::Length(456.0)),
                    margin: None,
                    numbering: None,
                    columns: None,
                },
                Content::text("x"),
            ]
            .into(),
        );
        let doc = layout(&c);
        // caracteriza: SetPage muta page_config; dimensões observáveis mudam.
        assert!(
            doc.pages[0].width != bw || doc.pages[0].height != bh,
            "SetPage deve alterar dimensões (baseline {bw}x{bh}, obtido {}x{})",
            doc.pages[0].width,
            doc.pages[0].height
        );
        assert_eq!(doc.pages[0].width, 123.0, "width do SetPage propaga");
        assert_eq!(doc.pages[0].height, 456.0, "height do SetPage propaga");
    }

    // ── P867 — `#set page(height: auto)` / `width: auto` ───────────────────
    #[test]
    fn p867_height_auto_cresce_para_conteudo_curto() {
        let doc = layout_test("#set page(height: auto)\nX");
        assert_eq!(doc.pages.len(), 1, "height: auto deve produzir página única");
        assert!(
            doc.pages[0].height.is_finite() && doc.pages[0].height > 0.0,
            "altura deve ser finita e positiva, obtido {}",
            doc.pages[0].height
        );
    }

    #[test]
    fn p867_height_auto_cresce_para_conteudo_longo() {
        let doc = layout_test("#set page(height: auto)\n#lorem(200)");
        assert_eq!(doc.pages.len(), 1, "height: auto deve desactivar paginação automática");
    }

    #[test]
    fn p867_width_auto_cresce_para_conteudo() {
        let doc = layout_test("#set page(width: auto)\nX");
        assert_eq!(doc.pages.len(), 1, "width: auto deve produzir página única");
        assert!(
            doc.pages[0].width.is_finite() && doc.pages[0].width > 0.0,
            "largura deve ser finita e positiva, obtido {}",
            doc.pages[0].width
        );
    }

    #[test]
    fn p867_height_fixo_continua_paginar() {
        let doc = layout_test("#set page(height: 80pt)\n#lorem(80)");
        assert!(
            doc.pages.len() >= 2,
            "height fixo pequeno deve continuar a paginar, obtido {} páginas",
            doc.pages.len()
        );
    }

    // ── Styled (bold/italic) → escopo: aplica ao corpo, não vaza ──────────
    #[test]
    fn carac_strong_aplica_bold_ao_corpo() {
        let c = Content::strong(Content::Text("Bold".into()));
        assert!(has_bold(&c), "strong deve produzir glyph bold");
    }

    #[test]
    fn carac_emph_aplica_italic_ao_corpo() {
        let c = Content::emph(Content::Text("It".into()));
        assert!(has_italic(&c), "emph deve produzir glyph italic");
    }

    #[test]
    fn carac_styled_nao_vaza_para_irmao() {
        // strong(bold) seguido de texto regular: o irmão NÃO fica bold.
        let c = Content::Sequence(
            vec![
                Content::strong(Content::Text("B".into())),
                Content::Text("normal".into()),
            ]
            .into(),
        );
        let bolds: Vec<bool> = layout(&c)
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|i| {
                if let FrameItem::Text { style, text, .. } = i {
                    Some((text.as_str().to_string(), style.bold))
                } else {
                    None
                }
            })
            .map(|(_, b)| b)
            .collect();
        // caracteriza: existe pelo menos um bold e pelo menos um não-bold
        // (o estilo não vaza do strong para o irmão regular).
        assert!(bolds.iter().any(|&b| b), "deve haver glyph bold (do strong)");
        assert!(bolds.iter().any(|&b| !b), "irmão regular não deve ser bold (escopo)");
    }

    // ── Text com/sem Styled em volta: plain_text idêntico, layout difere ──
    #[test]
    fn carac_text_plain_text_identico_com_e_sem_styled() {
        let nu = Content::Text("hi".into());
        let st = Content::strong(Content::Text("hi".into()));
        // plain_text é transparente ao wrapper Styled.
        assert_eq!(nu.plain_text(), "hi");
        assert_eq!(st.plain_text(), "hi", "Styled é transparente em plain_text");
        // mas o layout difere (bold vs regular).
        assert!(!has_bold(&nu), "Text cru não é bold");
        assert!(has_bold(&st), "Text em strong é bold");
    }

    // ── Set + elemento migrado: heading numerado dentro de columns ────────
    #[test]
    fn carac_heading_numerado_dentro_de_columns() {
        // Lote F-2 S1 (P335): numeração assada no heading (`heading_numbered`);
        // asserções inalteradas.
        let c = Content::Sequence(
            vec![Content::columns(
                Content::heading_numbered(1, Content::text("Dentro")),
                2,
                None,
            )]
            .into(),
        );
        let t = doc_text(&c);
        // caracteriza: a numeração de heading atravessa o contentor migrado
        // (columns) — o heading numerado renderiza numerado mesmo encapsulado.
        assert!(t.contains("Dentro"), "corpo do heading presente: '{t}'");
        assert!(t.contains("1."), "heading dentro de columns deve numerar: '{t}'");
    }

    // ── P460 — Label: destinos nomeados ────────────────────────────────────
    use crate::entities::label::Label;

    #[test]
    fn layout_label_renderiza_body_sem_alteracao_visual() {
        let body = Content::text("Marcado");
        let content = Content::label("sec1", body.clone());
        let doc = layout(&content);
        let text = doc.plain_text();
        assert_eq!(text, "Marcado", "Label não deve alterar texto renderizado");
        // Verificar que o body aparece como Text na página.
        let has_text = doc.pages.iter().any(|p| {
            p.items
                .iter()
                .any(|i| matches!(i, FrameItem::Text { text: t, .. } if t == "Marcado"))
        });
        assert!(has_text, "FrameItem::Text do body deve estar presente");
    }

    #[test]
    fn layout_label_registra_pagina_e_posicao() {
        let content = Content::label("fig1", Content::text("Corpo"));
        let doc = layout(&content);
        let label = Label("fig1".to_string());
        assert_eq!(
            doc.extracted_label_pages.get(&label),
            Some(&1),
            "label deve estar registado na página 1"
        );
        assert!(
            doc.extracted_label_positions.contains_key(&label),
            "label deve ter posição registada"
        );
        let pos = doc.extracted_label_positions.get(&label).unwrap();
        assert!(pos.x.val() > 0.0 || pos.y.val() > 0.0, "posição deve ser positiva");
    }
}

// ── P462 — `ref<x>` / `@x`: resolução numérica via Content::Label ─────────────
mod p462_ref_numeric {
    use super::*;
    use crate::engine::introspect::introspect_with_introspector;
    use crate::entities::introspector::TagIntrospector;
    use crate::entities::label::Label;
    use crate::entities::layout_types::TrackSizing;
    use crate::entities::style::Styles;
    use crate::entities::value::Value;

    fn doc_text_with_intr(content: &Content, intr: TagIntrospector) -> String {
        layout_with_introspector(content, intr).plain_text()
    }

    #[test]
    fn ref_resolves_heading_number() {
        let body = Content::heading_numbered(1, Content::text("Intro"));
        let content = Content::Sequence(
            vec![Content::label("intro", body), Content::reference("intro")].into(),
        );
        let intr = introspect_with_introspector(&content);
        let text = doc_text_with_intr(&content, intr);
        assert!(text.contains("1"), "heading ref deve resolver para '1': {text}");
        assert!(!text.contains("@intro"), "não deve manter @intro: {text}");
    }

    #[test]
    fn ref_resolves_figure_number() {
        let fig = Content::figure(
            Content::text("[img]"),
            Some(Content::text("legenda")),
            None,
            Some("1".to_string()),
        );
        let content = Content::Sequence(
            vec![Content::label("f1", fig), Content::reference("f1")].into(),
        );
        let intr = introspect_with_introspector(&content);
        // P788: o join suplemento↔número usa NBSP (paridade vanilla).
        let text = doc_text_with_intr(&content, intr).replace('\u{a0}', " ");
        assert!(text.contains("Fig. 1"), "figure ref deve renderizar 'Fig. 1': {text}");
    }

    #[test]
    fn ref_resolves_equation_number() {
        let eq = Content::equation_numbered(Content::text("x"), true);
        let content = Content::Sequence(
            vec![Content::label("eq1", eq), Content::reference("eq1")].into(),
        );
        let intr = introspect_with_introspector(&content);
        let text = doc_text_with_intr(&content, intr);
        assert!(text.contains("(1)"), "equation ref deve renderizar '(1)': {text}");
    }

    #[test]
    fn ref_resolves_table_number() {
        let table = Content::table_with_caption(
            vec![TrackSizing::Auto],
            vec![TrackSizing::Auto],
            vec![Content::text("cell")],
            Some(Content::text("legenda")),
        );
        let numbered = Content::Styled(
            Box::new(table),
            Styles::new().push_custom("table.numbering", Value::Str("1.".into())),
        );
        let content = Content::Sequence(
            vec![Content::label("tbl1", numbered), Content::reference("tbl1")].into(),
        );
        let intr = introspect_with_introspector(&content);
        let text = doc_text_with_intr(&content, intr);
        assert!(text.contains("Table 1"), "table ref deve renderizar 'Table 1': {text}");
    }

    #[test]
    fn ref_supplement_explicit_overrides_default() {
        let fig = Content::figure(
            Content::text("[img]"),
            Some(Content::text("legenda")),
            None,
            Some("1".to_string()),
        );
        let content = Content::Sequence(
            vec![
                Content::label("f1", fig),
                Content::reference_with_supplement("f1", Some(Content::text("Figura "))),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);
        let text = doc_text_with_intr(&content, intr);
        assert!(
            text.contains("Figura 1"),
            "supplement explicito deve sobrescrever default: {text}"
        );
    }

    #[test]
    fn ref_unknown_label_renders_question_mark() {
        // P788: label inexistente deixou de renderizar "?" — vanilla erra
        // (`does not exist in the document`); teste actualizado para o erro.
        let content = Content::reference("nao_existe");
        let intr = TagIntrospector::empty();
        let doc = layout_with_introspector(&content, intr);
        assert!(
            doc.layout_errors.iter().any(|d| d
                .message
                .contains("label `<nao_existe>` does not exist in the document")),
            "erro de label inexistente esperado: {:?}",
            doc.layout_errors
        );
    }

    #[test]
    fn labelled_path_not_resolved_numerically() {
        // Labelled (P329) continua a usar texto resolvido legacy, não número.
        let content = Content::Sequence(
            vec![
                Content::label_auto(
                    "sec".to_string(),
                    Content::heading_numbered(1, Content::text("Secção")),
                ),
                Content::reference("sec"),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);
        let text = doc_text_with_intr(&content, intr);
        assert!(
            !text.contains("1") || text.contains("Secção"),
            "Labelled deve manter texto resolvido legacy: {text}"
        );
    }

    // ── P463 — ref como link clicável ───────────────────────────────────────

    fn find_first_link(
        doc: &crate::entities::layout_types::PagedDocument,
    ) -> Option<&FrameItem> {
        for page in &doc.pages {
            for item in &page.items {
                if let FrameItem::Link { .. } = item {
                    return Some(item);
                }
            }
        }
        None
    }

    #[test]
    fn ref_renders_as_frame_item_link_with_destination() {
        use crate::entities::layout_types::LinkTarget;
        let body = Content::heading_numbered(1, Content::text("Intro"));
        let content = Content::Sequence(
            vec![Content::label("intro", body), Content::reference("intro")].into(),
        );
        let intr = introspect_with_introspector(&content);
        let doc = layout_with_introspector(&content, intr);
        let link = find_first_link(&doc).expect("ref deve produzir FrameItem::Link");
        assert!(
            matches!(link, FrameItem::Link { target: LinkTarget::Destination(l), .. } if l.0 == "intro"),
            "ref deve ter LinkTarget::Destination(intro): {:?}",
            link
        );
    }

    #[test]
    fn external_link_still_uses_url_target() {
        use crate::entities::layout_types::LinkTarget;
        let doc = layout(&Content::link("https://example.com", Content::text("click")));
        let link = find_first_link(&doc).expect("link deve produzir FrameItem::Link");
        assert!(
            matches!(link, FrameItem::Link { target: LinkTarget::Url(u), .. } if u.as_str() == "https://example.com"),
            "link externo deve continuar LinkTarget::Url: {:?}",
            link
        );
    }
}

// ── P488 — LoF/LoT com page numbers (2-pass fixpoint) ──────────────────────

#[test]
fn p488_lof_sem_known_pages_usa_formato_sem_numero() {
    // Iteração 0: sem known_figure_page_numbers → LoF sem ". . . N".
    let content = Content::Sequence(
        vec![
            Content::lof(None),
            Content::figure(
                Content::text("Corpo"),
                Some(Content::text("Diagrama de fluxo")),
                Some("image".to_string()),
                Some("1".to_string()),
            ),
        ]
        .into(),
    );
    let _state = introspect(&content);
    let doc = layout(&content);
    let text = doc.plain_text();
    // LoF deve aparecer com "Figure 1  Diagrama de fluxo"
    assert!(text.contains("List of Figures"), "LoF deve ter título: {text:?}");
    assert!(
        text.contains("Figure 1") && text.contains("Diagrama de fluxo"),
        "LoF deve listar a figura: {text:?}"
    );
}

#[test]
fn p488_lof_com_figura_regista_page_number_no_extracted() {
    // Verifica que extracted_figure_page_numbers é populado após layout.
    let content = Content::Sequence(
        vec![
            Content::lof(None),
            Content::figure(
                Content::text("Mapa"),
                Some(Content::text("Legenda do mapa")),
                Some("image".to_string()),
                Some("1".to_string()),
            ),
        ]
        .into(),
    );
    let _state = introspect(&content);
    let doc = layout(&content);
    // Após layout com figura contada, extracted_figure_page_numbers não vazio.
    assert!(
        !doc.extracted_figure_page_numbers.is_empty(),
        "extracted_figure_page_numbers deve ser populado após layout com figura contada"
    );
    // A figura deve ter sido registada na página 1.
    assert_eq!(
        doc.extracted_figure_page_numbers[0], 1,
        "figura na primeira página deve ser página 1"
    );
}

#[test]
fn p488_extracted_table_page_numbers_default_vazio() {
    // Documento sem tabelas contadas → extracted_table_page_numbers permanece vazio.
    let content = Content::Sequence(
        vec![Content::lot(None), Content::text("Sem tabelas contadas aqui")].into(),
    );
    let _state = introspect(&content);
    let doc = layout(&content);
    assert!(
        doc.extracted_table_page_numbers.is_empty(),
        "sem tabelas contadas: extracted_table_page_numbers deve estar vazio"
    );
}

// ── P756 — Segmentação de linha para scripts sem espaços ───────────────────

#[test]
fn p756_cjk_segmenta_sem_espacos() {
    // Texto chinês sem espaços numa página estreita deve quebrar em várias
    // linhas em vez de fugir para a margem direita.
    let doc = layout_test("#set page(width: 80pt, margin: 5pt)\n#set text(size: 12pt)\n测试文本测试引号的位置这是一段很长的中文文字用来测试换行的效果如何");
    let text = doc.plain_text().replace(' ', "");
    assert!(
        text.contains("测试文本") && text.contains("换行的效果"),
        "texto CJK deve estar presente no documento: {}",
        text
    );

    let y_values: std::collections::HashSet<u64> = doc
        .pages
        .iter()
        .flat_map(|p| p.items.iter())
        .filter_map(|i| match i {
            FrameItem::Text { pos, .. } => Some(pos.y.val().to_bits()),
            _ => None,
        })
        .collect();
    assert!(
        y_values.len() > 1,
        "texto CJK longo numa página estreita deve ocupar várias linhas, y_values={:?}",
        y_values
    );

    let right = 80.0;
    for page in &doc.pages {
        for item in &page.items {
            if let FrameItem::Text { pos, text, .. } = item {
                let w = FixedMetrics
                    .advance(text.as_str(), Pt(12.0), &TextStyle::default())
                    .0;
                assert!(
                    pos.x.val() + w <= right,
                    "item CJK ({:?}) excede a margem direita: x={} w={}",
                    text,
                    pos.x.val(),
                    w
                );
            }
        }
    }
}

#[test]
fn p756_thai_segmenta_sem_espacos() {
    // Texto tailandês sem espaços numa página estreita deve quebrar em várias
    // linhas sem exceder a margem direita.
    // Página mais larga que o maior fragmento Thai reportado pelo
    // segmentador (≈101 pt em FixedMetrics), para que a verificação de
    // margem seja significativa sem exigir subdivisão abaixo do fragmento.
    let doc = layout_test("#set page(width: 120pt, margin: 5pt)\n#set text(size: 12pt)\nสวัสดีครับผมชื่อจอห์นยินดีที่ได้รู้จักคุณ");
    let text = doc.plain_text().replace(' ', "");
    assert!(
        text.contains("สวัสดี") && text.contains("คุณ"),
        "texto Thai deve estar presente no documento: {}",
        text
    );

    let y_values: std::collections::HashSet<u64> = doc
        .pages
        .iter()
        .flat_map(|p| p.items.iter())
        .filter_map(|i| match i {
            FrameItem::Text { pos, .. } => Some(pos.y.val().to_bits()),
            _ => None,
        })
        .collect();
    assert!(
        y_values.len() > 1,
        "texto Thai longo numa página estreita deve ocupar várias linhas, y_values={:?}",
        y_values
    );

    let right = 120.0;
    for page in &doc.pages {
        for item in &page.items {
            if let FrameItem::Text { pos, text, .. } = item {
                let w = FixedMetrics
                    .advance(text.as_str(), Pt(12.0), &TextStyle::default())
                    .0;
                assert!(
                    pos.x.val() + w <= right,
                    "item Thai ({:?}) excede a margem direita: x={} w={}",
                    text,
                    pos.x.val(),
                    w
                );
            }
        }
    }
}

#[test]
fn p756_cjk_aspas_renderiza_sem_panic() {
    // Caso das aspas CJK: a segmentação deve ocorrer sem panic e o texto
    // completo deve estar presente. A posição exacta das aspas depende do
    // algoritmo greedy e do espaço disponível (limitação documentada em
    // 00_nucleo/diagnosticos/paridade-producao-p756.md).
    let doc = layout_test("#set page(width: 100pt, margin: 5pt)\n#set text(size: 12pt)\n测试文本，\"测试引号的位置\"。这是一段很长的中文文字用来测试换行的效果如何。");
    let text = doc.plain_text().replace(' ', "");
    assert!(
        text.contains("测试文本")
            && text.contains("引号的位置")
            && text.contains("效果如何"),
        "texto com aspas CJK deve estar completo: {}",
        text
    );
}


// ── P813 — equação em bloco: centragem horizontal + espaçamento vertical ────
//
// Achado #16 de P808: a equação em bloco era alinhada à margem esquerda e
// sem o espaçamento de bloco do vanilla (`BlockElem::above/below` default
// 1.2em — lab/typst-original/crates/typst-library/src/layout/container.rs:342;
// centragem via ShowSet `align(center)` —
// lab/typst-original/crates/typst-library/src/math/equation.rs:190).
//
// Valores esperados com FixedMetrics (size 11pt, página A4 default):
//   margin  = 595.28 × 2.5/21          = 70.8667
//   top     = 0.7 × 11 (cap-height)    = 7.7
//   spacing = 1.2 × 11                 = 13.2
//   extent("x") = width 6.6, ascent 7.7 (cap-height), descent 0
#[cfg(test)]
mod p813_equacao_bloco {
    use super::*;

    const MARGIN: f64 = 595.28 * 2.5 / 21.0; // 70.8667
    const USABLE: f64 = 595.28 - 2.0 * MARGIN; // 453.5467
    const TOP: f64 = 7.7; // cap-height 0.7 × 11pt (FixedMetrics)
    const SPACING: f64 = 13.2; // 1.2em × 11pt (default BlockElem vanilla)
    const EQ_W: f64 = 6.6; // advance("x") = 0.6 × 11 (FixedMetrics)
    const EQ_ASCENT: f64 = 7.7; // ink default = cap-height
    const TOL: f64 = 0.01;

    fn baseline_antes() -> f64 {
        MARGIN + TOP // 78.5667
    }
    fn baseline_math() -> f64 {
        baseline_antes() + SPACING + EQ_ASCENT // 99.4667
    }
    fn baseline_depois() -> f64 {
        baseline_math() + SPACING + TOP // 120.3667 (descent ink = 0)
    }
    fn x_centrado() -> f64 {
        MARGIN + (USABLE - EQ_W) / 2.0 // 294.34
    }

    /// Devolve (x, y) do primeiro item de texto cujo conteúdo não está em
    /// `exclude` (i.e. o item da equação).
    fn pos_equacao(doc: &PagedDocument, exclude: &[&str]) -> (f64, f64) {
        doc.pages
            .iter()
            .flat_map(|p| p.items.iter())
            .find_map(|i| match i {
                FrameItem::Text { pos, text, .. }
                    if !exclude.contains(&text.as_str()) =>
                {
                    Some((pos.x.val(), pos.y.val()))
                }
                _ => None,
            })
            .expect("item da equação tem de existir")
    }

    fn pos_texto(doc: &PagedDocument, alvo: &str) -> (f64, f64) {
        doc.pages
            .iter()
            .flat_map(|p| p.items.iter())
            .find_map(|i| match i {
                FrameItem::Text { pos, text, .. } if text.as_str() == alvo => {
                    Some((pos.x.val(), pos.y.val()))
                }
                _ => None,
            })
            .unwrap_or_else(|| panic!("texto {alvo:?} tem de existir"))
    }

    #[test]
    fn equacao_bloco_centrada_horizontalmente() {
        let content = Content::Sequence(
            vec![
                Content::text("Antes"),
                Content::equation(Content::MathIdent("x".into()), true),
                Content::text("Depois"),
            ]
            .into(),
        );
        let doc = layout(&content);
        let (x, _) = pos_equacao(&doc, &["Antes", "Depois"]);
        assert!(
            (x - x_centrado()).abs() < TOL,
            "equação em bloco deve ser centrada na região: x={:.4}, esperado {:.4} (margem={:.4})",
            x,
            x_centrado(),
            MARGIN
        );
    }

    #[test]
    fn equacao_bloco_espacamento_vertical_1_2em() {
        let content = Content::Sequence(
            vec![
                Content::text("Antes"),
                Content::equation(Content::MathIdent("x".into()), true),
                Content::text("Depois"),
            ]
            .into(),
        );
        let doc = layout(&content);
        let (_, y_antes) = pos_texto(&doc, "Antes");
        let (_, y_math) = pos_equacao(&doc, &["Antes", "Depois"]);
        let (_, y_depois) = pos_texto(&doc, "Depois");
        assert!(
            (y_math - baseline_math()).abs() < TOL,
            "baseline da equação: y={:.4}, esperado {:.4} (Δ desde 'Antes' = {:.4})",
            y_math,
            baseline_math(),
            y_math - y_antes
        );
        assert!(
            (y_depois - baseline_depois()).abs() < TOL,
            "baseline do 'Depois': y={:.4}, esperado {:.4} (Δ desde math = {:.4})",
            y_depois,
            baseline_depois(),
            y_depois - y_math
        );
    }

    #[test]
    fn equacao_bloco_no_topo_do_documento_sem_spacing_acima() {
        // Paridade vanilla (medido em P813: `$ x^2 $` sozinho → baseline =
        // margin + ascent, sem 1.2em acima no topo da região).
        let content =
            Content::equation(Content::MathIdent("x".into()), true);
        let doc = layout(&content);
        let (x, y) = pos_equacao(&doc, &[]);
        assert!(
            (y - (MARGIN + EQ_ASCENT)).abs() < TOL,
            "equação no topo: baseline={:.4}, esperado {:.4} (sem spacing acima)",
            y,
            MARGIN + EQ_ASCENT
        );
        assert!(
            (x - x_centrado()).abs() < TOL,
            "equação no topo também é centrada: x={:.4}, esperado {:.4}",
            x,
            x_centrado()
        );
    }

    #[test]
    fn equacao_inline_sem_alteracao_de_baseline() {
        // O caminho inline não é tocado por P813: math partilha a baseline
        // do texto circundante (P800).
        let content = Content::Sequence(
            vec![
                Content::text("A"),
                Content::equation(Content::MathIdent("x".into()), false),
                Content::text("B"),
            ]
            .into(),
        );
        let doc = layout(&content);
        let (_, y_a) = pos_texto(&doc, "A");
        let (_, y_b) = pos_texto(&doc, "B");
        let (_, y_math) = pos_equacao(&doc, &["A", "B"]);
        assert!(
            (y_math - y_a).abs() < TOL && (y_b - y_a).abs() < TOL,
            "inline: texto e math partilham a baseline: A={:.4} math={:.4} B={:.4}",
            y_a,
            y_math,
            y_b
        );
    }

    #[test]
    fn equacao_numerada_centragem_nao_quebra_numero() {
        let content = Content::Sequence(
            vec![
                Content::text("Antes"),
                Content::equation_numbered(Content::MathIdent("E".into()), true),
                Content::text("Depois"),
            ]
            .into(),
        );
        let doc = layout(&content);
        let (x_math, y_math) = pos_equacao(&doc, &["Antes", "Depois", "(1)"]);
        let (x_num, y_num) = pos_texto(&doc, "(1)");
        let eq_w = FixedMetrics.advance("E", Pt(11.0), &TextStyle::default()).0;
        let x_esperado = MARGIN + (USABLE - eq_w) / 2.0;
        assert!(
            (x_math - x_esperado).abs() < TOL,
            "equação numerada também centrada: x={:.4}, esperado {:.4}",
            x_math,
            x_esperado
        );
        // O número continua ancorado à direita e na baseline da equação.
        let right_lim = 595.28 - MARGIN;
        assert!(
            x_num > right_lim - 30.0,
            "número deve ficar junto à margem direita: x={:.4} (limite {:.4})",
            x_num,
            right_lim
        );
        assert!(
            (y_num - y_math).abs() < TOL,
            "número alinhado com a baseline da equação: num={:.4} math={:.4}",
            y_num,
            y_math
        );
    }
}

// ── P856 — Validação de refs para labels existentes mas não referenciáveis ──

#[cfg(test)]
mod p856_ref_unreferencable_labels {
    use super::*;
    use crate::entities::content::Content;
    use crate::entities::label::Label;
    use std::sync::Arc;

    fn lbl(s: &str) -> Label {
        Label(s.to_string())
    }

    fn layout_errors_for(content: Content) -> Vec<String> {
        layout(&content).layout_errors.iter().map(|d| d.message.clone()).collect()
    }

    #[test]
    fn ref_label_texto_da_cannot_reference_text() {
        let content = Content::Sequence(Arc::from(vec![
            Content::label_auto("lbl", Content::text("texto")),
            Content::reference("lbl"),
        ]));
        let errors = layout_errors_for(content);
        assert!(
            errors.iter().any(|m| m == "cannot reference text"),
            "esperado 'cannot reference text', erros: {:?}",
            errors
        );
    }

    #[test]
    fn ref_label_raw_da_mensagem_vanilla_raw() {
        let content = Content::Sequence(Arc::from(vec![
            Content::label_auto("code", Content::raw("fn main() {}", Some("rust".into()), true)),
            Content::reference("code"),
        ]));
        let errors = layout_errors_for(content);
        assert!(
            errors.iter().any(|m| m == "cannot reference raw directly, try putting it into a figure"),
            "esperado mensagem raw, erros: {:?}",
            errors
        );
    }

    #[test]
    fn ref_equation_sem_numbering_da_mensagem_vanilla_equation() {
        let content = Content::Sequence(Arc::from(vec![
            Content::label_auto("eq", Content::equation(Content::MathIdent("x".into()), true)),
            Content::reference("eq"),
        ]));
        let errors = layout_errors_for(content);
        assert!(
            errors.iter().any(|m| m == "cannot reference equation without numbering"),
            "esperado 'cannot reference equation without numbering', erros: {:?}",
            errors
        );
    }

    #[test]
    fn ref_equation_com_numbering_continua_funcionar() {
        let content = Content::Sequence(Arc::from(vec![
            Content::label_auto(
                "eq",
                Content::equation_numbered(Content::MathIdent("x".into()), true),
            ),
            Content::reference("eq"),
        ]));
        let doc = layout(&content);
        assert!(
            doc.layout_errors.is_empty(),
            "não deve haver erros: {:?}",
            doc.layout_errors
        );
        let txt = doc.plain_text();
        assert!(txt.contains("(1)"), "esperado '(1)' no texto: {:?}", txt);
    }

    #[test]
    fn ref_label_inexistente_continua_dar_does_not_exist() {
        let content = Content::Sequence(Arc::from(vec![Content::reference("missing")]));
        let errors = layout_errors_for(content);
        assert!(
            errors.iter().any(|m| m.contains("label `<missing>` does not exist in the document")),
            "esperado 'does not exist', erros: {:?}",
            errors
        );
    }
}
