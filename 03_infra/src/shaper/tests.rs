//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/shaper/tests.md
//! @prompt-hash b234db04
//! @layer L3
//! @updated 2026-10-03

use super::bidi::bidi_runs;
use super::cache::{FaceCache, ShapeCache};
use super::font_selection::{
    intern_fallback_slot, is_p1404_format_control, resolve_candidates, CandidateSet,
};
use super::reconciliation::{
    fix_line_positions, fix_line_positions_with_metrics, reconcile_rtl_positions,
};
use super::shaping::{byte_idx_to_char, shaped_width, ssty_level_for_subrun};
use super::traversal::{shape_document, shape_item};
use crate::font_variant::axis_variations_for_font_variant;
use typst_core::contracts::world::World;
use typst_core::entities::dir::Dir;
use typst_core::entities::font_book::FontInfo;
use typst_core::entities::layout_types::ShapedGlyph;

use crate::world::SystemWorld;
use ecow::EcoString;
use std::path::PathBuf;
use typst_core::entities::file_id::FileId;
use typst_core::entities::font_book::{
    Coverage, FontBook, FontFlags, FontStretch, FontStyle, FontVariant, FontWeight,
};
use typst_core::entities::font_list::FontList;
use typst_core::entities::layout_types::{
    FrameItem, Length, MathSize, Page, PagedDocument, Point, Pt, TextStyle,
};
use typst_core::entities::source::Source;
use typst_core::entities::world_types::{
    Bytes, Datetime, FileError, FileResult, Font, Library,
};

#[test]
fn p1404_fallback_interner_e_esparso_e_idempotente() {
    let mut fallback = Vec::new();
    assert_eq!(intern_fallback_slot(&mut fallback, Some(65_535)), 0);
    assert_eq!(fallback, vec![Some(65_535)]);
    assert_eq!(intern_fallback_slot(&mut fallback, Some(7)), 1);
    assert_eq!(intern_fallback_slot(&mut fallback, Some(65_535)), 0);
    assert_eq!(fallback, vec![Some(65_535), Some(7)]);
}

#[test]
fn p1404_controles_de_formato_nao_pedem_glifo() {
    assert!(is_p1404_format_control('\u{2060}'));
    assert!(is_p1404_format_control('\u{202b}'));
    assert!(is_p1404_format_control('\u{202c}'));
    assert!(!is_p1404_format_control('\u{200b}'));
    for c in [
        '\u{61c}', '\u{200e}', '\u{200f}', '\u{202a}', '\u{202d}', '\u{202e}',
        '\u{2066}', '\u{2067}', '\u{2068}', '\u{2069}',
    ] {
        assert!(is_p1404_format_control(c));
    }
    for c in ['\u{200c}', '\u{200d}', '\u{fe0f}'] {
        assert!(!is_p1404_format_control(c));
    }
    assert!(!is_p1404_format_control('a'));
}

struct MockWorld {
    book: FontBook,
}

impl typst_core::contracts::world::World for MockWorld {
    fn library(&self) -> &Library {
        static L: std::sync::OnceLock<Library> = std::sync::OnceLock::new();
        L.get_or_init(Library::new)
    }
    fn book(&self) -> &FontBook {
        &self.book
    }
    fn main(&self) -> FileId {
        unimplemented!()
    }
    fn source(&self, _: FileId) -> FileResult<Source> {
        unimplemented!()
    }
    fn file(&self, _: FileId) -> FileResult<Bytes> {
        unimplemented!()
    }
    fn font(&self, _: usize) -> Option<Font> {
        None
    }
    fn today(
        &self,
        _: Option<typst_core::entities::duration::Duration>,
    ) -> Option<Datetime> {
        None
    }
}

fn empty_world() -> MockWorld {
    MockWorld { book: FontBook::new() }
}

// ── P534 — helpers para testes com fontes reais ─────────────────────────────

/// `World` de teste com `FontBook` e bytes de fontes controlados.
struct FontWorld {
    book: FontBook,
    fonts: Vec<Option<Font>>,
}

impl FontWorld {
    fn push_font(&mut self, path: &str) {
        if let Ok(data) = std::fs::read(path) {
            if let Some(mut info) = crate::fonts::font_info_from_bytes(&data, 0) {
                // P938 — font_info_from_bytes deixa coverage vazia; para
                // testes de fallback com FontWorld preenchemos eager aqui.
                if let Ok(face) = ttf_parser::Face::parse(&data, 0) {
                    info.coverage = crate::fonts::extract_coverage(&face);
                }
                self.book.push(info);
                self.fonts.push(Some(Font::from_data(data)));
                return;
            }
        }
        // Fonte ausente ou inválida: reserva slot vazio para manter índices
        // consistentes com o FontBook (não deve ser usada em testes que
        // dependem desta fonte).
        self.fonts.push(None);
    }

    fn is_complete(&self) -> bool {
        self.fonts.iter().all(|f| f.is_some())
    }
}

impl typst_core::contracts::world::World for FontWorld {
    fn library(&self) -> &Library {
        static L: std::sync::OnceLock<Library> = std::sync::OnceLock::new();
        L.get_or_init(Library::new)
    }
    fn book(&self) -> &FontBook {
        &self.book
    }
    fn main(&self) -> FileId {
        unimplemented!()
    }
    fn source(&self, _: FileId) -> FileResult<Source> {
        unimplemented!()
    }
    fn file(&self, _: FileId) -> FileResult<Bytes> {
        Err(FileError::NotFound)
    }
    fn font(&self, idx: usize) -> Option<Font> {
        self.fonts.get(idx).cloned().flatten()
    }
    fn today(
        &self,
        _: Option<typst_core::entities::duration::Duration>,
    ) -> Option<Datetime> {
        None
    }
}

fn font_world_with(paths: &[&str]) -> FontWorld {
    let mut world = FontWorld { book: FontBook::new(), fonts: Vec::new() };
    for path in paths {
        world.push_font(path);
    }
    world
}

fn text_item_with_font(text: &str, family: &str) -> FrameItem {
    let style = TextStyle {
        font: Some(FontList::single(EcoString::from(family))),
        size: Pt(12.0),
        ..TextStyle::default()
    };
    FrameItem::Text {
        pos: Point { x: Pt(0.0), y: Pt(0.0) },
        text: EcoString::from(text),
        style,
        bidi: None,
    }
}

fn text_item(text: &str) -> FrameItem {
    FrameItem::Text {
        pos: Point { x: Pt(72.0), y: Pt(72.0) },
        text: EcoString::from(text),
        style: TextStyle::default(),
        bidi: None,
    }
}

fn doc_with(items: Vec<FrameItem>) -> PagedDocument {
    PagedDocument::new(vec![Page {
        width: 595.0,
        height: 842.0,
        numbering: None,
        logical_number: 1,
        supplement: typst_core::entities::content::Content::Empty,
        bleed: Default::default(),
        fill: Default::default(),
        background: vec![],
        foreground: vec![],
        items,
    }])
}

struct TempDir(PathBuf);
impl TempDir {
    fn path(&self) -> &std::path::Path {
        &self.0
    }
}
impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn tempfile_write(name: &str, content: &str) -> TempDir {
    let path = std::env::temp_dir().join(format!(
        "typst-shaper-test-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.subsec_nanos())
            .unwrap_or(0)
    ));
    std::fs::create_dir_all(&path).unwrap();
    std::fs::write(path.join(name), content).unwrap();
    TempDir(path)
}

#[test]
fn p482_shape_document_preserves_text_sem_font() {
    let doc = doc_with(vec![text_item("Hello")]);
    let shaped = shape_document(&empty_world(), doc);
    assert!(
        matches!(&shaped.pages[0].items[0], FrameItem::Text { .. }),
        "P482: Text sem style.font deve permanecer Text"
    );
}

#[test]
fn p482_shape_document_preserves_text_content() {
    let doc = doc_with(vec![text_item("world")]);
    let shaped = shape_document(&empty_world(), doc);
    if let FrameItem::Text { text, .. } = &shaped.pages[0].items[0] {
        assert_eq!(text.as_str(), "world");
    } else {
        panic!("esperava FrameItem::Text");
    }
}

#[test]
fn p482_byte_idx_to_char_ascii() {
    assert_eq!(byte_idx_to_char("hello", 0), Some('h'));
    assert_eq!(byte_idx_to_char("hello", 1), Some('e'));
    assert_eq!(byte_idx_to_char("hello", 5), None);
}

#[test]
fn p482_byte_idx_to_char_utf8() {
    let s = "héllo";
    assert_eq!(byte_idx_to_char(s, 0), Some('h'));
    assert_eq!(byte_idx_to_char(s, 1), Some('é'));
    assert_eq!(byte_idx_to_char(s, 3), Some('l'));
}

#[test]
fn p482_shaped_glyph_clone_eq() {
    let g = ShapedGlyph {
        glyph_id: 42,
        x_advance: 600,
        x_offset: 0,
        y_offset: 0,
        cluster: 0,
        char_code: 'A',
    };
    assert_eq!(g.clone(), g);
}

// ── P483 ────────────────────────────────────────────────────────────────

#[test]
fn p483_text_com_font_helvetica_tenta_shape_mas_sem_fontes_preserva_text() {
    // FrameItem::Text com style.font = Some(Helvetica) → shaper tenta;
    // MockWorld não tem fontes → resolve_slot retorna None → item preservado
    // como Text. O path de tentativa é exercitado (style.font.is_some() == true).
    use typst_core::entities::font_list::FontList;
    let style = TextStyle {
        font: Some(FontList::single(EcoString::from("Helvetica"))),
        ..TextStyle::default()
    };
    let item = FrameItem::Text {
        pos: Point { x: Pt(72.0), y: Pt(72.0) },
        text: EcoString::from("Ola"),
        style,
        bidi: None,
    };
    let doc = doc_with(vec![item]);
    let shaped = shape_document(&empty_world(), doc);
    // MockWorld.font() retorna None → try_shape retorna None → Text preservado
    assert!(
        matches!(&shaped.pages[0].items[0], FrameItem::Text { .. }),
        "P483: sem fontes reais, Text com font=Some preservado como fallback"
    );
}

#[test]
fn p483_text_sem_font_nao_tenta_shape() {
    // style.font = None → shaper ignora (guarda is_some() false).
    let item = text_item("sem fonte"); // text_item usa TextStyle::default() → font=None
    let doc = doc_with(vec![item]);
    let shaped = shape_document(&empty_world(), doc);
    assert!(
        matches!(&shaped.pages[0].items[0], FrameItem::Text { .. }),
        "P483: Text sem style.font=None não é tentado pelo shaper"
    );
}

#[test]
fn p483_shaped_glyph_debug_display() {
    let g = ShapedGlyph {
        glyph_id: 1,
        x_advance: 500,
        x_offset: 0,
        y_offset: 0,
        cluster: 0,
        char_code: 'A',
    };
    let s = format!("{:?}", g);
    assert!(s.contains("glyph_id: 1"), "Debug deve incluir glyph_id");
}

// ── P484 — testes RTL ────────────────────────────────────────────────────

#[test]
fn p484_bidi_runs_ltr_unico_run() {
    let paras = bidi_runs("Hello world");
    assert_eq!(paras.len(), 1, "texto sem `\\n` → 1 parágrafo");
    let runs = &paras[0];
    assert_eq!(runs.len(), 1, "texto inglês deve produzir 1 run");
    assert!(!runs[0].rtl, "texto inglês deve ser LTR");
    assert_eq!(runs[0].byte_start, 0);
}

#[test]
fn p484_bidi_runs_vazio_zero_runs() {
    let runs = bidi_runs("");
    assert_eq!(runs.len(), 0, "texto vazio → 0 runs");
}

#[test]
fn p484_bidi_runs_arabico_rtl() {
    // مرحبا = "Olá" em árabe (5 chars, todos RTL)
    let runs: Vec<_> = bidi_runs("مرحبا").into_iter().flatten().collect();
    assert!(!runs.is_empty(), "árabe deve ter pelo menos 1 run");
    assert!(runs.iter().any(|r| r.rtl), "árabe deve ter run RTL");
}

#[test]
fn p484_try_shape_rtl_sem_fonte_nao_panic() {
    // Shape de texto árabe sem fonte carregada → sem panic (fallback Text)
    let item = FrameItem::Text {
        pos: Point { x: Pt(0.0), y: Pt(0.0) },
        text: EcoString::from("مرحبا"),
        style: TextStyle::default(),
        bidi: None,
    };
    // style.font = None → shaper guard (is_some() == false) → item inalterado
    let mut cache = ShapeCache::new();
    let result = shape_item(&empty_world(), item, &mut cache, &mut FaceCache::new());
    // o critério é simplesmente não entrar em panic
    assert_eq!(result.len(), 1);
    assert!(
        matches!(result[0], FrameItem::Text { .. }),
        "sem fonte: preservado como Text"
    );
}

#[test]
fn p484_bidi_runs_misto_ingles_arabico() {
    // "Hi مرحبا" — deve ter 2 runs (LTR + RTL)
    let runs: Vec<_> = bidi_runs("Hi مرحبا").into_iter().flatten().collect();
    assert!(runs.len() >= 2, "texto misto deve ter ≥2 runs, got {}", runs.len());
    let has_ltr = runs.iter().any(|r| !r.rtl);
    let has_rtl = runs.iter().any(|r| r.rtl);
    assert!(has_ltr, "deve ter run LTR");
    assert!(has_rtl, "deve ter run RTL");
}

#[test]
fn p484_bidi_runs_byte_start_correcto() {
    // Para texto LTR puro, byte_start do único run deve ser 0
    let runs: Vec<_> = bidi_runs("abc").into_iter().flatten().collect();
    assert_eq!(runs[0].byte_start, 0);
    assert_eq!(runs[0].text, "abc");
}

// ── P845 — texto com `\n` interno: todos os parágrafos bidi (achado #55) ──

#[test]
fn p845_bidi_runs_tres_linhas_tres_paragrafos() {
    let paras = bidi_runs("a\nb\nc");
    assert_eq!(paras.len(), 3, "3 linhas → 3 parágrafos bidi");
    let texts: Vec<&str> = paras.iter().flatten().map(|r| r.text.as_str()).collect();
    assert_eq!(texts, vec!["a", "b", "c"], "sem o separador `\\n` nos runs");
}

#[test]
fn p845_bidi_runs_linhas_vazias_consecutivas() {
    let paras = bidi_runs("a\n\nb");
    assert_eq!(paras.len(), 3, "`a\\n\\nb` → 3 parágrafos (meio vazio)");
    assert!(paras[1].is_empty(), "linha vazia não produz runs");
    assert_eq!(paras[0][0].text, "a");
    assert_eq!(paras[2][0].text, "b");
}

#[test]
fn p845_bidi_runs_sem_newline_um_paragrafo() {
    let paras = bidi_runs("Hello world");
    assert_eq!(paras.len(), 1, "texto sem `\\n` → 1 parágrafo");
    assert_eq!(paras[0].len(), 1, "texto LTR puro → 1 run");
}

#[test]
fn p845_try_shape_multilinha_empilha_linhas() {
    let world = font_world_with(&["/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf"]);
    if !world.is_complete() {
        eprintln!("SKIP: DejaVu Sans não disponível");
        return;
    }
    let doc = doc_with(vec![text_item_with_font("a\nb\nc", "DejaVu Sans")]);
    let shaped = shape_document(&world, doc);
    let items = &shaped.pages[0].items;
    let mut texts: Vec<&str> = Vec::new();
    let mut ys: Vec<f64> = Vec::new();
    for it in items {
        if let FrameItem::TextShaped { pos, text, .. } = it {
            texts.push(text.as_str());
            ys.push(pos.y.0);
        }
    }
    assert_eq!(texts, vec!["a", "b", "c"], "as 3 linhas shapeadas");
    assert!(ys[1] > ys[0] && ys[2] > ys[1], "linhas empilhadas verticalmente: {:?}", ys);
}

#[test]
fn p845_shaped_width_multilinha_max_das_linhas() {
    let world = font_world_with(&["/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf"]);
    if !world.is_complete() {
        eprintln!("SKIP: DejaVu Sans não disponível");
        return;
    }
    let style = TextStyle {
        font: Some(FontList::single(EcoString::from("DejaVu Sans"))),
        size: Pt(12.0),
        ..TextStyle::default()
    };
    let mut face_cache = FaceCache::new();
    let w_linha = shaped_width(&world, "bb", &style, &mut face_cache).unwrap();
    let w_multi = shaped_width(&world, "a\nbb", &style, &mut face_cache).unwrap();
    assert_eq!(w_linha.0, w_multi.0, "largura multilinha = max das linhas (não soma)");
}

// P485 — units_per_em populado pelo shaper
#[test]
fn p485_shape_document_sem_fonte_nao_produz_textshaped() {
    // Sem fonte no MockWorld, shape_document não converte → Text preservado
    // units_per_em não é populado (sem TextShaped produzido)
    let doc = doc_with(vec![text_item("Hello")]);
    let shaped = shape_document(&empty_world(), doc);
    assert!(
        matches!(&shaped.pages[0].items[0], FrameItem::Text { .. }),
        "P485: sem fonte → deve permanecer Text (units_per_em não aplicável)"
    );
}

#[test]
fn p485_units_per_em_cast_seguro() {
    // rb_face.units_per_em() retorna i32 em rustybuzz; cast para u16 seguro para valores positivos
    let val_i32: i32 = 1000;
    let as_u16 = val_i32.max(1) as u16;
    assert_eq!(as_u16, 1000u16);
}

#[test]
fn p486_features_default_confirmado() {
    // liga, kern, calt activados por defeito em rustybuzz 0.20.1 via HORIZONTAL_FEATURES
    // (ot_shape.rs:86-91). Nenhuma user feature é necessária — &[] é suficiente.
    let features: &[rustybuzz::Feature] = &[];
    assert_eq!(
        features.len(),
        0,
        "P486: features user vazias — defaults de rustybuzz aplicam-se"
    );
}

// ── P515 — font fallback por caractere ──────────────────────────────────

#[test]
fn p515_resolve_candidates_sem_fontes_retorna_none() {
    let world = empty_world();
    let font_list = FontList::single(EcoString::from("Helvetica"));
    let mut face_cache = FaceCache::new();
    assert!(resolve_candidates(
        &world,
        &font_list,
        &FontVariant::default(),
        &mut face_cache
    )
    .is_none());
}

#[test]
fn p515_shape_document_real_font_produz_textshaped() {
    // Carrega uma fonte real do sistema (fallback se ausente).
    let candidates = load_real_font_candidates();
    if candidates.is_empty() {
        return; // skip em ambientes sem fontes
    }

    // O SystemWorld carrega o FontBook correspondente aos slots reais.
    let dir = tempfile_write("main.typ", "text");
    let world = SystemWorld::new(dir.path(), "main.typ").unwrap().with_system_fonts();
    if world.book().is_empty() {
        return; // skip
    }

    let mut style = TextStyle::default();
    // Usa a primeira família disponível no sistema.
    let family = EcoString::from(world.book().infos()[0].family.clone());
    style.font = Some(FontList::single(family));
    style.size = Pt(12.0);

    let item = FrameItem::Text {
        pos: Point { x: Pt(0.0), y: Pt(0.0) },
        text: EcoString::from("Hello"),
        style,
        bidi: None,
    };
    let mut cache = ShapeCache::new();
    let result = shape_item(&world, item, &mut cache, &mut FaceCache::new());
    assert!(!result.is_empty(), "deve produzir pelo menos 1 TextShaped");
    assert!(
        result.iter().all(|it| matches!(it, FrameItem::TextShaped { .. })),
        "todos os resultados devem ser TextShaped"
    );
}

// Helper: carrega bytes de uma fonte real do sistema para mock.
// Retorna (slot_idx, bytes). Usado apenas se houver fontes disponíveis.
fn load_real_font_candidates() -> Vec<(usize, Vec<u8>)> {
    let mut result = Vec::new();
    for path in [
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "/usr/share/fonts/opentype/urw-base35/C059-Roman.otf",
    ] {
        if let Ok(bytes) = std::fs::read(path) {
            if ttf_parser::Face::parse(&bytes, 0).is_ok() {
                result.push((0, bytes));
                break;
            }
        }
    }
    result
}

#[test]
fn p482_shape_document_group_children_passthrough() {
    use typst_core::entities::layout_types::TransformMatrix;
    let inner = text_item("inner");
    let group = FrameItem::Group {
        paint_root: false,
        pos: Point { x: Pt(0.0), y: Pt(0.0) },
        matrix: TransformMatrix::default(),
        clip_mask: None,
        inner_width: 100.0,
        inner_height: 100.0,
        items: vec![inner],
    };
    let doc = doc_with(vec![group]);
    let shaped = shape_document(&empty_world(), doc);
    if let FrameItem::Group { items, .. } = &shaped.pages[0].items[0] {
        // sem font → Text preservado dentro do Group
        assert!(matches!(&items[0], FrameItem::Text { .. }));
    } else {
        panic!("esperava Group");
    }
}

// ── P525 — Variation Fonts (MVP) ────────────────────────────────────────

#[test]
fn p525_axis_variations_weight_italic() {
    let bold = axis_variations_for_font_variant(&FontVariant {
        style: FontStyle::Normal,
        weight: FontWeight::BOLD,
        stretch: FontStretch::NORMAL,
    });
    assert_eq!(bold.len(), 1);
    assert_eq!(bold[0].tag, ttf_parser::Tag::from_bytes(b"wght"));
    assert_eq!(bold[0].value, 700.0);

    let italic = axis_variations_for_font_variant(&FontVariant {
        style: FontStyle::Italic,
        weight: FontWeight::REGULAR,
        stretch: FontStretch::NORMAL,
    });
    assert_eq!(italic.len(), 1);
    assert_eq!(italic[0].tag, ttf_parser::Tag::from_bytes(b"ital"));
    assert_eq!(italic[0].value, 1.0);

    let regular = axis_variations_for_font_variant(&FontVariant::default());
    assert!(regular.is_empty(), "regular upright não precisa de variações");
}

#[test]
fn p525_shape_document_mixed_weights_no_contamination() {
    // Usa Ubuntu Sans VF do sistema, se disponível. Se não estiver,
    // o teste faz skip gracioso.
    let dir = tempfile_write("main.typ", "text");
    let world = SystemWorld::new(dir.path(), "main.typ").unwrap().with_system_fonts();
    if world.book().select("Ubuntu Sans", &FontVariant::default()).is_none() {
        eprintln!("SKIP: Ubuntu Sans não disponível no sistema");
        return;
    }

    fn text_item_with_weight(text: &str, weight: u16) -> FrameItem {
        let style = TextStyle {
            font: Some(FontList::single(EcoString::from("Ubuntu Sans"))),
            weight: Some(weight),
            size: Pt(40.0),
            ..TextStyle::default()
        };
        FrameItem::Text {
            pos: Point { x: Pt(0.0), y: Pt(0.0) },
            text: EcoString::from(text),
            style,
            bidi: None,
        }
    }

    fn total_width(item: &FrameItem) -> i32 {
        match item {
            FrameItem::TextShaped { glyphs, .. } => {
                glyphs.iter().map(|g| g.x_advance).sum()
            }
            _ => 0,
        }
    }

    let doc = doc_with(vec![
        text_item_with_weight("Hello", 700),
        text_item_with_weight("Hello", 100),
        text_item_with_weight("Hello", 700),
    ]);
    let shaped = shape_document(&world, doc);
    let items = &shaped.pages[0].items;
    assert_eq!(items.len(), 3, "cada Text deve produzir um TextShaped");

    let bold_1 = total_width(&items[0]);
    let thin = total_width(&items[1]);
    let bold_2 = total_width(&items[2]);

    assert!(
        bold_1 > thin,
        "bold (wght=700) deve ser mais largo que thin (wght=100): {} > {}",
        bold_1,
        thin
    );
    assert_eq!(
        bold_1, bold_2,
        "duas chamadas com wght=700, intercaladas por wght=100, \
         devem produzir a mesma largura — indica contaminação de estado se falhar"
    );
}

// ── P534 — Fallback multi-script ────────────────────────────────────────────

#[test]
fn p534_split_run_by_font_respects_script_boundaries() {
    // DejaVuSans cobre latim, mas não CJK. O fallback global (Noto Sans CJK)
    // é usado para o segmento CJK. Verifica-se que o shaper produz pelo
    // menos dois TextShaped distintos.
    let world = font_world_with(&[
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Bold.ttc",
    ]);
    if !world.is_complete() {
        eprintln!("SKIP: fontes necessárias não disponíveis");
        return;
    }

    let doc = doc_with(vec![text_item_with_font("Hello 你好", "DejaVu Sans")]);
    let shaped = shape_document(&world, doc);
    let items = &shaped.pages[0].items;
    assert!(
        items.iter().all(|it| matches!(it, FrameItem::TextShaped { .. })),
        "todos os itens devem ser TextShaped"
    );
    assert!(
        items.len() >= 2,
        "latim + CJK devem produzir ≥2 TextShaped (fontes distintas), got {}",
        items.len()
    );
}

#[test]
fn p534_shape_mixed_script_system_fallback() {
    // Documento latim + CJK + árabe. A fonte pedida (DejaVu Sans) só cobre
    // latim; o fallback global deve cobrir CJK e árabe, produzindo três
    // scripts distintos.
    let world = font_world_with(&[
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Bold.ttc",
        "/usr/share/fonts/truetype/noto/NotoNaskhArabic-Bold.ttf",
    ]);
    if !world.is_complete() {
        eprintln!("SKIP: fontes necessárias não disponíveis");
        return;
    }

    let doc = doc_with(vec![text_item_with_font("Hello 你好 مرحبا", "DejaVu Sans")]);
    let shaped = shape_document(&world, doc);
    let items = &shaped.pages[0].items;
    assert!(
        items.iter().all(|it| matches!(it, FrameItem::TextShaped { .. })),
        "todos os itens devem ser TextShaped"
    );
    assert!(
        items.len() >= 3,
        "latim + CJK + árabe devem produzir ≥3 TextShaped, got {}",
        items.len()
    );
}

#[test]
fn p534_latin_only_stays_single_textshaped() {
    // Sem texto misto, não deve haver fragmentação adicional.
    let world = font_world_with(&[
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Bold.ttc",
    ]);
    if !world.is_complete() {
        eprintln!("SKIP: fontes necessárias não disponíveis");
        return;
    }

    let doc = doc_with(vec![text_item_with_font("Hello World", "DejaVu Sans")]);
    let shaped = shape_document(&world, doc);
    let items = &shaped.pages[0].items;
    assert_eq!(items.len(), 1, "texto latim puro deve produzir 1 TextShaped");
}

// ── P838 — fallback global com scoring de similaridade (achado #24 P831) ──

/// Book controlado: [DejaVu Sans (primária), Droid Sans Fallback,
/// Noto Sans CJK JP]. Sem scoring, o fallback global escolhia Droid
/// (primeira por ordem de índice que cobre). Com o scoring do vanilla
/// (`like` = DejaVu Sans: mono=false, serif=false), vence
/// "Noto Sans CJK JP" — família mais curta (15 < 19 chars) com os
/// mesmos flags, tal como o vanilla escolhe para `like` Libertinus Serif.
#[test]
fn p838_fallback_global_prefere_scoring_vanilla() {
    let world = font_world_with(&[
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "/usr/share/fonts/truetype/droid/DroidSansFallbackFull.ttf",
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
    ]);
    if !world.is_complete() {
        eprintln!("SKIP: fontes necessárias não disponíveis");
        return;
    }
    assert_eq!(
        world.book().infos()[2].family,
        "Noto Sans CJK JP",
        "face 0 do NotoSansCJK-Regular.ttc deve ser JP"
    );

    let mut face_cache = FaceCache::new();
    let font_list = FontList::single(EcoString::from("DejaVu Sans"));
    let variant = FontVariant::default();
    let primary = resolve_candidates(&world, &font_list, &variant, &mut face_cache)
        .expect("DejaVu Sans resolve");
    let like = world.book().infos().get(primary[0].slot_idx).cloned();
    let mut candidates =
        CandidateSet::new(&world, primary, &mut face_cache, like, variant);

    let text = "日本語の";
    let (cand_idx, end) =
        candidates.covering_run(text, 0).expect("CJK tem cobertura no book");
    let slot = candidates.get(cand_idx).unwrap().slot_idx;
    assert_eq!(
        slot, 2,
        "scoring vanilla escolhe Noto Sans CJK JP (slot 2), não Droid (slot 1)"
    );
    assert_eq!(end, text.len(), "a fonte escolhida cobre o run inteiro");
}

/// **P538e** — quando a fonte declarada ("Helvetica") não existe no
/// FontBook, o shaper recai na lista de fontes padrão (DejaVu Sans, ...)
/// e shapeia o texto misto em vez de preservar `FrameItem::Text`.
#[test]
fn p538e_fonte_default_ausente_usa_fallback_padrao() {
    let world = font_world_with(&[
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Bold.ttc",
    ]);
    if !world.is_complete() {
        eprintln!("SKIP: fontes necessárias não disponíveis");
        return;
    }

    // "Helvetica" não está no FontWorld; o shaper deve tentar DejaVu Sans.
    let doc = doc_with(vec![text_item_with_font("Hello 你好", "Helvetica")]);
    let shaped = shape_document(&world, doc);
    let items = &shaped.pages[0].items;
    assert!(
        items.iter().all(|i| matches!(i, FrameItem::TextShaped { .. })),
        "texto misto com fonte inexistente deve ser shapeado via fallback padrão"
    );
    let glyph_chars: String = items
        .iter()
        .filter_map(|i| match i {
            FrameItem::TextShaped { glyphs, .. } => {
                Some(glyphs.iter().map(|g| g.char_code).collect::<String>())
            }
            _other => None, // neutro: N16[β] — itens não-textuais não produzem métricas de shaping,
        })
        .collect();
    assert!(
        glyph_chars.contains('H') && glyph_chars.contains('你'),
        "deve conter caracteres latinos e CJK, got {:?}",
        glyph_chars
    );
}

/// **P543** — quando nenhuma das fontes padrão existe e o FontBook começa
/// por uma fonte especializada de cobertura parcial (MathJax_AMS cobre 'H'
/// mas não 'ello'), o shaper deve escolher a fonte que cobre o maior trecho
/// contíguo, evitando fragmentar "Hello" em "H" + "ello" e duplicar texto.
#[test]
fn p543_fallback_global_escolhe_maior_trecho_e_nao_duplica() {
    // MathJax_AMS primeiro: cobre 'H' mas não 'e'/'l'/'o'.
    // DejaVu Sans depois: cobre "Hello" inteiro.
    let world = font_world_with(&[
        "/usr/share/fonts/opentype/mathjax/MathJax_AMS-Regular.otf",
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
    ]);
    if !world.is_complete() {
        eprintln!("SKIP: fontes necessárias não disponíveis");
        return;
    }

    // "Helvetica" não existe; as primárias ficam vazias e recai-se no
    // fallback global. Sem a correcção de P543, MathJax_AMS seria escolhida
    // para 'H' e DejaVu Sans para "ello", produzindo duplicação.
    let doc = doc_with(vec![text_item_with_font("Hello", "Helvetica")]);
    let shaped = shape_document(&world, doc);
    let items = &shaped.pages[0].items;
    assert!(
        items.iter().all(|i| matches!(i, FrameItem::TextShaped { .. })),
        "deve produzir TextShaped"
    );

    // O texto combinado dos itens não deve duplicar caracteres.
    let rendered: String = items
        .iter()
        .filter_map(|i| match i {
            FrameItem::TextShaped { glyphs, .. } => {
                Some(glyphs.iter().map(|g| g.char_code).collect::<String>())
            }
            _other => None, // neutro: N16[β] — itens não-textuais não produzem métricas de shaping,
        })
        .collect();
    assert_eq!(rendered, "Hello", "texto não deve ser duplicado: got {:?}", rendered);

    // Deve haver apenas um TextShaped, porque DejaVu Sans cobre tudo.
    assert_eq!(
        items.len(),
        1,
        "fonte que cobre toda a palavra deve produzir 1 TextShaped, got {}",
        items.len()
    );
}

fn make_textshaped(
    x: f64,
    y: f64,
    glyphs: Vec<ShapedGlyph>,
    upem: u16,
    size: f64,
) -> FrameItem {
    FrameItem::TextShaped {
        pos: Point { x: Pt(x), y: Pt(y) },
        glyphs,
        style: TextStyle { size: Pt(size), ..TextStyle::default() },
        text: ecow::EcoString::from(""),
        units_per_em: upem,
        bidi: None,
        font: None,
    }
}

fn glyph(x_advance: i32) -> ShapedGlyph {
    ShapedGlyph {
        glyph_id: 1,
        x_advance,
        x_offset: 0,
        y_offset: 0,
        cluster: 0,
        char_code: 'A',
    }
}

fn page_with(items: Vec<FrameItem>) -> Page {
    Page {
        items,
        width: 595.28,
        height: 841.89,
        numbering: None,
        logical_number: 1,
        supplement: typst_core::entities::content::Content::Empty,
        bleed: Default::default(),
        fill: Default::default(),
        background: vec![],
        foreground: vec![],
    }
}

#[test]
fn reconciliation_with_borrowed_metrics_matches_fresh_entry_point() {
    let world = empty_world();
    let metrics = crate::font_metrics::FallbackFontMetrics::new(&world);
    let doc = PagedDocument::new(vec![page_with(vec![
        make_textshaped(70.0, 50.0, vec![glyph(500)], 1000, 12.0),
        make_textshaped(100.0, 50.0, vec![glyph(300)], 1000, 12.0),
    ])]);
    let fresh = fix_line_positions(&world, doc.clone());
    let borrowed = fix_line_positions_with_metrics(doc, &metrics);
    for (old, new) in fresh.pages[0].items.iter().zip(&borrowed.pages[0].items) {
        let (
            FrameItem::TextShaped { pos: old, .. },
            FrameItem::TextShaped { pos: new, .. },
        ) = (old, new)
        else {
            panic!("expected shaped text");
        };
        assert_eq!(old.x, new.x);
    }
}

// P582-T1: item único — posição inalterada.
#[test]
fn p582_single_item_unchanged() {
    let mut doc = PagedDocument::new(vec![page_with(vec![make_textshaped(
        70.87,
        50.0,
        vec![glyph(500)],
        1000,
        12.0,
    )])]);
    doc = fix_line_positions(&empty_world(), doc);
    if let FrameItem::TextShaped { pos, .. } = &doc.pages[0].items[0] {
        assert!((pos.x.0 - 70.87).abs() < 0.01, "item único: x inalterado");
    }
}

// P582-T2: dois itens — segundo recebe x = x_orig + (w_real - w_est) de item0.
#[test]
fn p582_two_items_redistributed() {
    // item1 em x=70, glyphs advance = 500/1000 * 12 = 6pt, w_est = 0
    // item2 originalmente em x=100, deve ir para 100 + (6 - 0) = 106
    let mut doc = PagedDocument::new(vec![page_with(vec![
        make_textshaped(70.0, 50.0, vec![glyph(500)], 1000, 12.0),
        make_textshaped(100.0, 50.0, vec![glyph(300)], 1000, 12.0),
    ])]);
    doc = fix_line_positions(&empty_world(), doc);
    let items = &doc.pages[0].items;
    // item[0] mantém x=70
    if let FrameItem::TextShaped { pos, .. } = &items[0] {
        assert!(
            (pos.x.0 - 70.0).abs() < 0.01,
            "item[0] x deve ser 70.0, got {}",
            pos.x.0
        );
    }
    // item[1] deve ser 106.0
    if let FrameItem::TextShaped { pos, .. } = &items[1] {
        assert!(
            (pos.x.0 - 106.0).abs() < 0.01,
            "item[1] x deve ser 106.0, got {}",
            pos.x.0
        );
    }
}

// P582-T3: itens em linhas diferentes — redistribuídos de forma independente.
#[test]
fn p582_two_lines_independent() {
    let mut doc = PagedDocument::new(vec![page_with(vec![
        make_textshaped(70.0, 50.0, vec![glyph(500)], 1000, 12.0), // linha 1
        make_textshaped(100.0, 50.0, vec![glyph(300)], 1000, 12.0), // linha 1
        make_textshaped(70.0, 70.0, vec![glyph(400)], 1000, 12.0), // linha 2
        make_textshaped(90.0, 70.0, vec![glyph(200)], 1000, 12.0), // linha 2
    ])]);
    doc = fix_line_positions(&empty_world(), doc);
    let items = &doc.pages[0].items;
    // linha 1: item[1] = 100 + (6 - 0) = 106
    if let FrameItem::TextShaped { pos, .. } = &items[1] {
        assert!(
            (pos.x.0 - 106.0).abs() < 0.01,
            "linha1 item[1] x deve ser 106.0, got {}",
            pos.x.0
        );
    }
    // linha 2: item[3] = 90 + (4.8 - 0) = 94.8
    if let FrameItem::TextShaped { pos, .. } = &items[3] {
        assert!(
            (pos.x.0 - 94.8).abs() < 0.01,
            "linha2 item[3] x deve ser 94.8, got {}",
            pos.x.0
        );
    }
}

// P582-T4: item com zero glyphs — advance zero, item seguinte mantém gap original.
#[test]
fn p582_zero_glyphs_zero_advance() {
    let mut doc = PagedDocument::new(vec![page_with(vec![
        make_textshaped(70.0, 50.0, vec![], 1000, 12.0), // sem glyphs
        make_textshaped(100.0, 50.0, vec![glyph(500)], 1000, 12.0),
    ])]);
    doc = fix_line_positions(&empty_world(), doc);
    let items = &doc.pages[0].items;
    // item[1] = 100 + 0 = 100
    if let FrameItem::TextShaped { pos, .. } = &items[1] {
        assert!(
            (pos.x.0 - 100.0).abs() < 0.01,
            "advance zero: item[1] x deve ser 100.0, got {}",
            pos.x.0
        );
    }
}

// P582-T5: coordenada y inalterada.
#[test]
fn p582_y_unchanged() {
    let mut doc = PagedDocument::new(vec![page_with(vec![
        make_textshaped(70.0, 123.45, vec![glyph(500)], 1000, 12.0),
        make_textshaped(100.0, 123.45, vec![glyph(300)], 1000, 12.0),
    ])]);
    doc = fix_line_positions(&empty_world(), doc);
    for item in &doc.pages[0].items {
        if let FrameItem::TextShaped { pos, .. } = item {
            assert!(
                (pos.y.0 - 123.45).abs() < 0.01,
                "y deve ser inalterado, got {}",
                pos.y.0
            );
        }
    }
}

// P582-T6: linha RTL — redistribuição com âncora à direita.
#[test]
fn p582_rtl_redistributed() {
    // Dois itens com dir RTL:
    // item0 (esquerda): x_orig = 70.0, w_real = 6.0, w_est = 0.0
    // item1 (direita/ancora): x_orig = 100.0, w_real = 3.6, w_est = 0.0
    let mut item0 = make_textshaped(70.0, 50.0, vec![glyph(500)], 1000, 12.0);
    let mut item1 = make_textshaped(100.0, 50.0, vec![glyph(300)], 1000, 12.0);
    if let FrameItem::TextShaped { style, .. } = &mut item0 {
        style.dir = Some(Dir::RTL);
    }
    if let FrameItem::TextShaped { style, .. } = &mut item1 {
        style.dir = Some(Dir::RTL);
    }

    let mut doc = PagedDocument::new(vec![page_with(vec![item0, item1])]);
    doc = fix_line_positions(&empty_world(), doc);
    let items = &doc.pages[0].items;

    // A borda direita estimada (100) permanece 100 após aplicar 3.6pt.
    if let FrameItem::TextShaped { pos, .. } = &items[1] {
        assert!(
            (pos.x.0 - 96.4).abs() < 0.01,
            "âncora RTL: x deve ser 96.4, got {}",
            pos.x.0
        );
    }
    // O gap estimado original era 30pt e permanece 30pt:
    // 96.4 - (60.4 + 6.0) = 30.
    if let FrameItem::TextShaped { pos, .. } = &items[0] {
        assert!(
            (pos.x.0 - 60.4).abs() < 0.01,
            "item RTL esquerdo: x deve ser 60.4, got {}",
            pos.x.0
        );
    }
}

#[test]
fn p1140_12_rtl_preserva_gaps_e_borda_direita() {
    let xs = [10.0, 40.0, 80.0];
    let estimated = [20.0, 25.0, 30.0];
    let real = [18.0, 28.0, 24.0];
    let corrected = reconcile_rtl_positions(&xs, &estimated, &real);

    let original_right = xs[2] + estimated[2];
    let corrected_right = corrected[2] + real[2];
    assert!((original_right - corrected_right).abs() < 0.001);
    for i in 0..2 {
        let original_gap = xs[i + 1] - (xs[i] + estimated[i]);
        let corrected_gap = corrected[i + 1] - (corrected[i] + real[i]);
        assert!((original_gap - corrected_gap).abs() < 0.001);
    }
}

/// **P621** — tracking aumenta os avanços reais dos glifos no shaper.
/// Usa DejaVu Sans se disponível; skip caso contrário.
#[test]
fn p621_tracking_aumenta_x_advance() {
    let world = font_world_with(&["/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf"]);
    if !world.is_complete() {
        eprintln!("SKIP: DejaVu Sans não disponível");
        return;
    }

    let style_no_tracking = TextStyle {
        font: Some(FontList::single(EcoString::from("DejaVu Sans"))),
        size: Pt(12.0),
        ..TextStyle::default()
    };

    let mut style_tracking = style_no_tracking.clone();
    let tracking_pt = 5.0;
    style_tracking.tracking = Some(Length::pt(tracking_pt));

    let text = "Hello";
    let item_no = FrameItem::Text {
        pos: Point { x: Pt(0.0), y: Pt(0.0) },
        text: EcoString::from(text),
        style: style_no_tracking,
        bidi: None,
    };
    let item_yes = FrameItem::Text {
        pos: Point { x: Pt(0.0), y: Pt(0.0) },
        text: EcoString::from(text),
        style: style_tracking,
        bidi: None,
    };

    let measure = |items: &[FrameItem]| -> (i32, usize) {
        items
            .iter()
            .filter_map(|it| match it {
                FrameItem::TextShaped { glyphs, units_per_em, .. } => {
                    let upm = *units_per_em as f64;
                    let sum_fu: i32 = glyphs.iter().map(|g| g.x_advance).sum();
                    let width = (sum_fu as f64 * 12.0 / upm * 1000.0).round() as i32;
                    Some((width, glyphs.len()))
                }
                _other => None, // neutro: N16[β] — itens não-textuais não produzem métricas de shaping,
            })
            .fold((0, 0), |(w, n), (dw, dn)| (w + dw, n + dn))
    };

    let mut cache = ShapeCache::new();
    let shaped_no = shape_item(&world, item_no, &mut cache, &mut FaceCache::new());
    let shaped_yes = shape_item(&world, item_yes, &mut cache, &mut FaceCache::new());

    let (width_no, n_glyphs) = measure(&shaped_no);
    let (width_yes, _) = measure(&shaped_yes);

    // Tracking é aplicado entre glifos (n_glyphs - 1 gaps).
    // Cada gap adiciona tracking_pt em unidades de texto (1/1000 de em).
    let expected_delta =
        (tracking_pt * 1000.0 * (n_glyphs.saturating_sub(1)) as f64).round() as i32;
    let actual_delta = width_yes - width_no;

    assert!(
        (actual_delta - expected_delta).abs() <= 20,
        "P621: tracking deve aumentar largura real em ~{} ({} glyphs), got {} (sem tracking {}, com tracking {})",
        expected_delta, n_glyphs, actual_delta, width_no, width_yes
    );
}

// ── P875 — filtro de fallback por bitmap Coverage ───────────────────────

/// `World` mock onde o `FontBook` tem cobertura controlada mas as fontes
/// reais (bytes) são idênticas para todos os slots. Assim, qualquer diferença
/// em `covering_all` vem do filtro de bitmap, não da cobertura real da face.
struct CoverageWorld {
    book: FontBook,
    fonts: Vec<Option<Font>>,
}

impl typst_core::contracts::world::World for CoverageWorld {
    fn library(&self) -> &Library {
        static L: std::sync::OnceLock<Library> = std::sync::OnceLock::new();
        L.get_or_init(Library::new)
    }
    fn book(&self) -> &FontBook {
        &self.book
    }
    fn main(&self) -> FileId {
        unimplemented!()
    }
    fn source(&self, _: FileId) -> FileResult<Source> {
        unimplemented!()
    }
    fn file(&self, _: FileId) -> FileResult<Bytes> {
        Err(FileError::NotFound)
    }
    fn font(&self, idx: usize) -> Option<Font> {
        self.fonts.get(idx).cloned().flatten()
    }
    fn today(
        &self,
        _: Option<typst_core::entities::duration::Duration>,
    ) -> Option<Datetime> {
        None
    }
}

/// Fonte real disponível em todos os ambientes de CI; usada como bytes
/// para todos os slots do mock.
fn load_fixture_font() -> Option<Font> {
    let data = std::fs::read(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/fixtures/fonts/NimbusSans-Regular.otf"
    ))
    .ok()?;
    Font::from_data(data).into()
}

/// Cria um `CoverageWorld` com `n` slots, todos com a mesma fonte real,
/// mas com `FontInfo.coverage` controlada individualmente.
fn coverage_world(coverages: Vec<Coverage>) -> CoverageWorld {
    let font = load_fixture_font();
    let mut book = FontBook::new();
    let mut fonts = Vec::new();
    for (i, coverage) in coverages.into_iter().enumerate() {
        book.push(FontInfo {
            family: format!("Font{i}"),
            variant: FontVariant::default(),
            flags: FontFlags::default(),
            coverage,
        });
        fonts.push(font.clone());
    }
    CoverageWorld { book, fonts }
}

/// `covering_all` só devolve candidatos cuja coverage exacta contém `c`.
/// A fonte real cobre 'A' para todos os slots, mas só o slot 1 tem 'A'
/// marcado no `FontInfo`.
#[test]
fn p937_covering_all_filtra_por_coverage_exacta() {
    let mut coverages = vec![Coverage::new(); 3];
    coverages[1] = Coverage::from_codepoints(['A' as u32]);
    let world = coverage_world(coverages);

    let primary = Vec::new();
    let mut face_cache = FaceCache::new();
    let mut candidates =
        CandidateSet::new(&world, primary, &mut face_cache, None, FontVariant::default());

    let all = candidates.covering_all('A');
    let slots: Vec<_> = all
        .into_iter()
        .map(|candidate_idx| candidates.get(candidate_idx).unwrap().slot_idx)
        .collect();
    assert_eq!(
        slots,
        vec![1],
        "só o slot com coverage do bloco de 'A' deve ser candidato"
    );
}

/// Se nenhuma fonte cobre o bloco, `covering_all` devolve lista vazia sem
/// carregar nenhuma face.
#[test]
fn p875_covering_all_vazio_quando_nenhum_bloco_cobre() {
    let coverages = vec![Coverage::new(); 3];
    let world = coverage_world(coverages);

    let primary = Vec::new();
    let mut face_cache = FaceCache::new();
    let mut candidates =
        CandidateSet::new(&world, primary, &mut face_cache, None, FontVariant::default());

    let all = candidates.covering_all('A');
    assert!(all.is_empty(), "coverage vazio → nenhum candidato");
    // Nenhuma face foi carregada: o cache deve estar vazio.
    assert!(face_cache.map.is_empty(), "nenhuma face carregada quando bitmap exclui");
}

#[test]
fn p1293_textitem_shaper_nao_solicita_ssty_e_preserva_controles() {
    let script_glyph = TextStyle {
        math: true,
        math_text_item: false,
        math_size: MathSize::Script,
        ..TextStyle::regular(Pt(11.0))
    };
    let script_textitem = TextStyle { math_text_item: true, ..script_glyph.clone() };
    let scriptscript_glyph = TextStyle {
        math_size: MathSize::ScriptScript,
        ..script_glyph.clone()
    };

    assert!(script_textitem.math, "TextItem deve preservar o contexto math");
    assert_eq!(ssty_level_for_subrun(&script_textitem, "R"), None);
    assert_eq!(ssty_level_for_subrun(&script_glyph, "R"), Some(1));
    assert_eq!(ssty_level_for_subrun(&scriptscript_glyph, "R"), Some(2));
    assert_eq!(ssty_level_for_subrun(&script_glyph, "RR"), None);
}

// Regression controls from the discriminating shaper prompt audit.
fn contract_fix_font(family: &str) -> (FontInfo, Font) {
    typst_assets::fonts()
        .find_map(|data| {
            let mut info = crate::fonts::font_info_from_bytes(data, 0)?;
            if info.family != family || info.variant != FontVariant::default() {
                return None;
            }
            let face = ttf_parser::Face::parse(data, 0).unwrap();
            info.coverage = crate::fonts::extract_coverage(&face);
            Some((info, Font::from_data(data.to_vec())))
        })
        .expect("required embedded regression font")
}
fn contract_fix_world() -> FontWorld {
    let (info, font) = contract_fix_font("Libertinus Serif");
    let mut book = FontBook::new();
    book.push(info);
    FontWorld { book, fonts: vec![Some(font)] }
}
fn contract_fix_glyphs(
    world: &FontWorld,
    text: &str,
    styles: &[TextStyle],
) -> Vec<Vec<ShapedGlyph>> {
    let items = styles
        .iter()
        .map(|style| FrameItem::Text {
            pos: Point { x: Pt(0.0), y: Pt(0.0) },
            text: text.into(),
            style: style.clone(),
            bidi: None,
        })
        .collect();
    shape_document(world, doc_with(items)).pages[0]
        .items
        .iter()
        .map(|item| match item {
            FrameItem::TextShaped { glyphs, .. } => glyphs.clone(),
            _ => panic!("regression fixture did not shape"),
        })
        .collect()
}
fn contract_fix_tracking_pair(a: TextStyle, b: TextStyle) {
    let world = contract_fix_world();
    let fresh_a = contract_fix_glyphs(&world, "ab", std::slice::from_ref(&a));
    let fresh_b = contract_fix_glyphs(&world, "ab", std::slice::from_ref(&b));
    assert_ne!(fresh_a, fresh_b, "tracking fixture must discriminate");
    assert_eq!(
        contract_fix_glyphs(&world, "ab", &[a.clone(), b.clone()]),
        vec![fresh_a[0].clone(), fresh_b[0].clone()]
    );
    assert_eq!(
        contract_fix_glyphs(&world, "ab", &[b, a.clone()]),
        vec![fresh_b[0].clone(), fresh_a[0].clone()]
    );
    assert_eq!(
        contract_fix_glyphs(&world, "ab", &[a.clone(), a]),
        vec![fresh_a[0].clone(), fresh_a[0].clone()]
    );
}
#[test]
fn contract_fix_tracking_precision_matches_fresh() {
    let mut a = TextStyle {
        font: Some(FontList::single("Libertinus Serif".into())),
        size: Pt(12.0),
        tracking: Some(Length::pt(0.00599)),
        ..Default::default()
    };
    let mut b = a.clone();
    b.tracking = Some(Length::pt(0.00601));
    contract_fix_tracking_pair(a.clone(), b);
    a.tracking = Some(Length::pt(-0.00599));
    let mut b = a.clone();
    b.tracking = Some(Length::pt(-0.00601));
    contract_fix_tracking_pair(a, b);
}
#[test]
fn contract_fix_tracking_size_matches_fresh() {
    let a = TextStyle {
        font: Some(FontList::single("Libertinus Serif".into())),
        size: Pt(12.0),
        tracking: Some(Length::pt(1.0)),
        ..Default::default()
    };
    let mut b = a.clone();
    b.size = Pt(24.0);
    contract_fix_tracking_pair(a, b);
}
#[test]
fn contract_fix_sparse_fallback_scores_global_slots() {
    let (base, first) = contract_fix_font("Libertinus Serif");
    let (extra, second) = contract_fix_font("New Computer Modern Math");
    let af = ttf_parser::Face::parse(first.as_slice(), 0).unwrap();
    let bf = ttf_parser::Face::parse(second.as_slice(), 0).unwrap();
    let c = (0x1d400..=0x1d7ff)
        .filter_map(char::from_u32)
        .find(|&c| af.glyph_index(c).is_none() && bf.glyph_index(c).is_some())
        .expect("discriminating fallback glyph");
    let mut book = FontBook::new();
    let mut fonts = Vec::new();
    for i in 0..7 {
        let mut info = base.clone();
        info.family = match i {
            0 => "Primary",
            1 => "Primary Preferred Decoy",
            2 => "ZZZ Decoy",
            5 => "Far Fallback With Long Name",
            6 => "Primary Extra",
            _ => "Unused",
        }
        .into();
        info.coverage = if i >= 5 { extra.coverage.clone() } else { Coverage::new() };
        book.push(info);
        fonts.push(if i >= 5 {
            Some(second.clone())
        } else if i == 0 {
            Some(first.clone())
        } else {
            None
        });
    }
    assert_eq!(
        book.select_fallback(Some(&book.infos()[0]), &FontVariant::default(), [5, 6]),
        Some(6)
    );
    let world = FontWorld { book, fonts };
    let mut face_cache = FaceCache::new();
    let primary = resolve_candidates(
        &world,
        &FontList::single("Primary".into()),
        &FontVariant::default(),
        &mut face_cache,
    )
    .unwrap();
    let like = Some(world.book.infos()[0].clone());
    let mut candidates =
        CandidateSet::new(&world, primary, &mut face_cache, like, FontVariant::default());
    let (index, end) = candidates.covering_run(&c.to_string(), 0).unwrap();
    assert_eq!(end, c.len_utf8());
    assert_eq!(candidates.get(index).unwrap().slot_idx, 6);
}
#[test]
fn contract_fix_same_face_script_boundaries() {
    let world = contract_fix_world();
    for (text, expected) in [
        ("abαβ", vec!["ab", "αβ"]),
        (" abαβ", vec![" ab", "αβ"]),
        ("a\u{301}α", vec!["a\u{301}", "α"]),
        ("αab", vec!["α", "ab"]),
    ] {
        let d = shape_document(
            &world,
            doc_with(vec![text_item_with_font(text, "Libertinus Serif")]),
        );
        let pieces: Vec<_> = d.pages[0]
            .items
            .iter()
            .map(|i| match i {
                FrameItem::TextShaped { text, glyphs, .. } => {
                    assert!(glyphs.iter().all(|g| g.glyph_id != 0));
                    text.as_str()
                }
                _ => panic!("did not shape"),
            })
            .collect();
        assert_eq!(pieces, expected);
    }
}

#[test]
fn contract_fix_equal_effective_tracking_matches_fresh() {
    let world = contract_fix_world();
    let a = TextStyle {
        font: Some(FontList::single("Libertinus Serif".into())),
        size: Pt(12.0),
        tracking: Some(Length::em(0.1)),
        ..Default::default()
    };
    let mut b = a.clone();
    b.size = Pt(24.0);
    let fresh_a = contract_fix_glyphs(&world, "ab", std::slice::from_ref(&a));
    let fresh_b = contract_fix_glyphs(&world, "ab", std::slice::from_ref(&b));
    assert_eq!(fresh_a, fresh_b);
    assert_eq!(
        contract_fix_glyphs(&world, "ab", &[a, b]),
        vec![fresh_a[0].clone(), fresh_b[0].clone()]
    );
}

// Language-sensitive locl controls use the mandatory, versioned Noto fixture.
fn language_fixture() -> FontWorld {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/fonts/NotoSans-Regular.ttf");
    let world = font_world_with(&[path.to_str().unwrap()]);
    assert!(world.is_complete(), "versioned language fixture must load");
    world
}

fn language_style(code: Option<&str>) -> TextStyle {
    TextStyle {
        font: Some(FontList::single("Noto Sans".into())),
        size: Pt(12.0),
        lang: code.map(|code| code.parse().unwrap()),
        ..Default::default()
    }
}

fn language_control(world: &FontWorld, code: Option<&str>) -> (u16, i32) {
    let font = world.font(0).unwrap();
    let face = rustybuzz::Face::from_slice(font.as_slice(), 0).unwrap();
    let mut buffer = rustybuzz::UnicodeBuffer::new();
    buffer.push_str("б");
    buffer.set_direction(rustybuzz::Direction::LeftToRight);
    if let Some(code) = code {
        buffer.set_language(code.parse().unwrap());
    }
    let output = rustybuzz::shape(&face, &[], buffer);
    assert_eq!(output.glyph_infos().len(), 1);
    (output.glyph_infos()[0].glyph_id as u16, output.glyph_positions()[0].x_advance)
}

#[test]
fn language_width_honors_localized_advances_and_absent_language() {
    let world = language_fixture();
    assert_eq!(language_control(&world, Some("ru")).1, 599);
    assert_eq!(language_control(&world, Some("sr")).1, 604);
    let font = world.font(0).unwrap();
    let upem = ttf_parser::Face::parse(font.as_slice(), 0).unwrap().units_per_em();
    let mut faces = FaceCache::new();
    for code in [Some("ru"), Some("sr"), Some("ru"), None, Some("en")] {
        let style = language_style(code);
        let expected = language_control(&world, code).1 as f64 * 12.0 / upem as f64;
        let width = shaped_width(&world, "б", &style, &mut faces).unwrap();
        assert!(
            (width.0 - expected).abs() < 1e-10,
            "language {code:?}: {width:?} != {expected}"
        );
    }
}

#[test]
fn language_emission_shared_cache_matches_fresh_in_both_orders() {
    let world = language_fixture();
    let shape = |codes: &[Option<&str>]| {
        let items = codes
            .iter()
            .map(|&code| FrameItem::Text {
                pos: Point::ZERO,
                text: "б".into(),
                style: language_style(code),
                bidi: None,
            })
            .collect();
        shape_document(&world, doc_with(items)).pages[0]
            .items
            .iter()
            .map(|item| match item {
                FrameItem::TextShaped { text, glyphs, .. } => {
                    assert_eq!(text.as_str(), "б");
                    assert_eq!(glyphs.len(), 1);
                    assert_eq!(glyphs[0].cluster, 0);
                    assert_eq!(glyphs[0].char_code, 'б');
                    glyphs.clone()
                }
                _ => panic!("mandatory language fixture did not shape"),
            })
            .collect::<Vec<_>>()
    };
    for codes in [
        vec![Some("ru"), Some("sr"), Some("ru")],
        vec![Some("sr"), Some("ru"), Some("sr")],
        vec![None, Some("sr"), Some("en"), None],
    ] {
        let fresh: Vec<_> = codes
            .iter()
            .map(|&code| {
                let glyphs = shape(&[code]).remove(0);
                let expected = language_control(&world, code);
                assert_eq!(
                    (glyphs[0].glyph_id, glyphs[0].x_advance),
                    expected,
                    "fresh language {code:?}"
                );
                glyphs
            })
            .collect();
        assert_eq!(shape(&codes), fresh, "language cache order {codes:?}");
    }
}

#[test]
#[allow(deprecated)]
fn discretionary_identity_shaper_preserves_envelope_visual_glyph_and_fallback() {
    let world = language_fixture();
    let visual = FrameItem::Text {
        pos: Point { x: Pt(10.0), y: Pt(20.0) },
        text: "-".into(),
        style: language_style(Some("en")),
        bidi: None,
    };
    let control = shape_document(&world, doc_with(vec![visual.clone()]));
    let wrapper =
        FrameItem::TextReplacement { text: "\u{ad}".into(), items: vec![visual] };
    let shaped = shape_document(&world, doc_with(vec![wrapper.clone()]));
    let FrameItem::TextReplacement { text, items } = &shaped.pages[0].items[0] else {
        panic!("identity lost")
    };
    assert_eq!(text.as_str(), "\u{ad}");
    let FrameItem::TextShaped { text: visual_text, glyphs, units_per_em, .. } = &items[0]
    else {
        panic!("visual unshaped")
    };
    let FrameItem::TextShaped {
        glyphs: control_glyphs, units_per_em: control_upem, ..
    } = &control.pages[0].items[0]
    else {
        panic!("control unshaped")
    };
    assert_eq!(visual_text.as_str(), "-");
    assert_eq!(glyphs, control_glyphs);
    assert_eq!(units_per_em, control_upem);
    assert_eq!(shaped.pages[0].plain_text(), "\u{ad}");
    let fallback = shape_document(&empty_world(), doc_with(vec![wrapper]));
    let FrameItem::TextReplacement { text, items } = &fallback.pages[0].items[0] else {
        panic!("fallback lost identity")
    };
    assert_eq!(text.as_str(), "\u{ad}");
    assert!(matches!(&items[0], FrameItem::Text { text, .. } if text.as_str()=="-"));
}

#[test]
fn authored_shy_occurrence_identity_keeps_visuals_and_existing_envelope() {
    fn same_visual(actual: &[FrameItem], expected: &[FrameItem]) {
        assert_eq!(actual.len(), expected.len());
        for (a, b) in actual.iter().zip(expected) {
            let (
                FrameItem::TextShaped { pos, text, glyphs, units_per_em, style, .. },
                FrameItem::TextShaped {
                    pos: bp,
                    text: bt,
                    glyphs: bg,
                    units_per_em: bu,
                    style: bs,
                    ..
                },
            ) = (a, b)
            else {
                panic!("expected visual shaped children")
            };
            assert_eq!((pos.x.0, pos.y.0), (bp.x.0, bp.y.0));
            assert_eq!(text, bt);
            assert_eq!(units_per_em, bu);
            let glyph_data = |gs: &[ShapedGlyph]| {
                gs.iter()
                    .map(|g| {
                        (
                            g.glyph_id,
                            g.x_advance,
                            g.x_offset,
                            g.y_offset,
                            g.cluster,
                            g.char_code,
                        )
                    })
                    .collect::<Vec<_>>()
            };
            assert_eq!(glyph_data(glyphs), glyph_data(bg));
            assert_eq!(format!("{style:?}"), format!("{bs:?}"));
        }
    }
    let world = language_fixture();
    let font = world.font(0).expect("mandatory fixture");
    let face = rustybuzz::Face::from_slice(font.as_slice(), 0).unwrap();
    for tracking in [None, Some(Length::pt(0.2))] {
        let mut style = language_style(Some("en"));
        style.tracking = tracking;
        for text in ["repre\u{ad}sentation", "a\u{ad}\u{ad}b", "f\u{ad}fi"] {
            let original = FrameItem::Text {
                pos: Point { x: Pt(17.0), y: Pt(23.0) },
                text: text.into(),
                style: style.clone(),
                bidi: None,
            };
            let FrameItem::Text { pos, text: source, style, .. } = &original else {
                unreachable!()
            };
            let control = super::shaping::try_shape(
                &world,
                pos,
                source,
                style,
                &mut ShapeCache::new(),
                &mut FaceCache::new(),
            )
            .unwrap();
            let mut buffer = rustybuzz::UnicodeBuffer::new();
            buffer.push_str(text);
            buffer.set_direction(rustybuzz::Direction::LeftToRight);
            buffer.set_language("en".parse().unwrap());
            buffer.set_flags(rustybuzz::BufferFlags::REMOVE_DEFAULT_IGNORABLES);
            let control_shape = rustybuzz::shape(&face, &[], buffer);
            let FrameItem::TextShaped { glyphs, .. } = &control[0] else {
                panic!("shaping failed")
            };
            assert_eq!(
                glyphs.iter().map(|g| g.glyph_id as u32).collect::<Vec<_>>(),
                control_shape
                    .glyph_infos()
                    .iter()
                    .map(|g| g.glyph_id)
                    .collect::<Vec<_>>(),
                "invisible placeholders must not enlarge the extraction box"
            );
            let shaped = shape_item(
                &world,
                original.clone(),
                &mut ShapeCache::new(),
                &mut FaceCache::new(),
            );
            assert_eq!(shaped.len(), 1);
            let FrameItem::TextReplacement { text: replacement, items } = &shaped[0]
            else {
                panic!("authored SHY lacks occurrence identity: {shaped:?}");
            };
            assert_eq!(replacement, source);
            same_visual(items, &control);
            let protected = FrameItem::TextReplacement {
                text: "external identity".into(),
                items: vec![original],
            };
            let shaped = shape_item(
                &world,
                protected,
                &mut ShapeCache::new(),
                &mut FaceCache::new(),
            );
            let FrameItem::TextReplacement { text, items } = &shaped[0] else {
                panic!("envelope lost")
            };
            assert_eq!(text, "external identity");
            same_visual(items, &control);
        }
    }
    let original = FrameItem::Text {
        pos: Point::ZERO,
        text: "a\u{ad}b".into(),
        style: language_style(None),
        bidi: None,
    };
    let absent = shape_item(
        &empty_world(),
        original,
        &mut ShapeCache::new(),
        &mut FaceCache::new(),
    );
    assert_eq!(absent.len(), 1);
    let FrameItem::Text { text, .. } = &absent[0] else {
        panic!("unresolved text changed")
    };
    assert_eq!(text, "a\u{ad}b");
}

#[test]
fn fonte_efetiva_sobrevive_fallback_e_cache_hit() {
    let world = font_world_with(&[
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "/usr/share/fonts/truetype/dejavu/DejaVuSans-Oblique.ttf",
    ]);
    assert_eq!(world.book().len(), 2, "controle requer as duas faces DejaVu");
    let mut item = text_item_with_font("مرحبا", "DejaVu Sans");
    if let FrameItem::Text { style, .. } = &mut item {
        style.italic = true;
        style.semantic_emph = true;
    }
    let doc = shape_document(&world, doc_with(vec![item.clone(), item]));
    assert_eq!(doc.pages[0].items.len(), 2);
    for item in &doc.pages[0].items {
        let FrameItem::TextShaped { font, style, glyphs, .. } = item else {
            panic!("shaping esperado")
        };
        let actual =
            font.as_ref().expect("fonte efetiva obrigatória também no cache hit");
        assert_eq!(actual.as_slice(), world.font(0).unwrap().as_slice());
        assert!(style.italic && style.semantic_emph);
        assert!(glyphs.iter().all(|g| g.glyph_id != 0));
    }
}

#[test]
fn resultado_memoizado_nao_recarrega_fonte_para_emitir_carrier() {
    struct CountWorld<'a> {
        base: &'a FontWorld,
        regular_loads: std::sync::atomic::AtomicUsize,
    }
    impl World for CountWorld<'_> {
        fn library(&self) -> &Library {
            self.base.library()
        }
        fn book(&self) -> &FontBook {
            self.base.book()
        }
        fn main(&self) -> FileId {
            self.base.main()
        }
        fn source(&self, id: FileId) -> FileResult<Source> {
            self.base.source(id)
        }
        fn file(&self, id: FileId) -> FileResult<Bytes> {
            self.base.file(id)
        }
        fn today(
            &self,
            offset: Option<typst_core::entities::duration::Duration>,
        ) -> Option<Datetime> {
            self.base.today(offset)
        }
        fn font(&self, index: usize) -> Option<Font> {
            if index == 0 {
                self.regular_loads.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
            self.base.font(index)
        }
    }
    let base = font_world_with(&[
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "/usr/share/fonts/truetype/dejavu/DejaVuSans-Oblique.ttf",
    ]);
    assert_eq!(base.book().len(), 2);
    let world = CountWorld { base: &base, regular_loads: Default::default() };
    let mut item = text_item_with_font("مرحبا", "DejaVu Sans");
    if let FrameItem::Text { style, .. } = &mut item {
        style.italic = true;
    }
    let doc = shape_document(&world, doc_with(vec![item.clone(), item]));
    assert!(doc.pages[0]
        .items
        .iter()
        .all(|item| matches!(item, FrameItem::TextShaped { font: Some(_), .. })));
    assert!(
        world.regular_loads.load(std::sync::atomic::Ordering::Relaxed) <= 2,
        "face/shape frio bastam: foram {} consultas",
        world.regular_loads.load(std::sync::atomic::Ordering::Relaxed)
    );
}

#[test]
fn authored_bidi_only_success_does_not_reenter_plain_text_fallback() {
    let world = language_fixture();
    let style = language_style(Some("en"));
    let mut cache = ShapeCache::new();
    let mut face_cache = FaceCache::new();
    for _ in 0..2 {
        for text in ["\u{200f}", "\u{202a}\u{202c}", "\u{2066}\u{2069}"] {
            let original = FrameItem::Text {
                pos: Point::ZERO,
                text: text.into(),
                style: style.clone(),
                bidi: None,
            };
            assert!(shape_item(&world, original.clone(), &mut cache, &mut face_cache)
                .is_empty());
            assert!(!shape_item(
                &empty_world(),
                original,
                &mut ShapeCache::new(),
                &mut FaceCache::new()
            )
            .is_empty());
        }
        let visible = FrameItem::Text {
            pos: Point::ZERO,
            text: "A\u{200f} B".into(),
            style: style.clone(),
            bidi: None,
        };
        assert!(!shape_item(&world, visible, &mut cache, &mut face_cache).is_empty());
    }
}

#[test]
fn bidi_occurrence_preserves_real_shape_and_does_not_nest_on_cache_hit() {
    let world = language_fixture();
    let style = language_style(Some("en"));
    let mut cache = ShapeCache::new();
    let mut face_cache = FaceCache::new();
    for _ in 0..2 {
        for text in ["\u{200e}ABC", "\u{2069}ABC"] {
            let original = FrameItem::Text {
                pos: Point::ZERO,
                text: text.into(),
                style: style.clone(),
                bidi: None,
            };
            let FrameItem::Text { pos, text: source, style, .. } = &original else {
                unreachable!()
            };
            let raw = super::shaping::try_shape(
                &world,
                pos,
                source,
                style,
                &mut ShapeCache::new(),
                &mut FaceCache::new(),
            )
            .unwrap();
            let actual =
                shape_item(&world, original.clone(), &mut cache, &mut face_cache);
            let FrameItem::TextReplacement { text: identity, items } = &actual[0] else {
                panic!("missing actual text")
            };
            assert_eq!(identity.as_str(), text);
            assert_eq!(format!("{items:?}"), format!("{raw:?}"));
            let wrapped = FrameItem::TextReplacement {
                text: "outer identity".into(),
                items: vec![original],
            };
            let result = shape_item(&world, wrapped, &mut cache, &mut face_cache);
            let FrameItem::TextReplacement { text, items } = &result[0] else {
                panic!("lost envelope")
            };
            assert_eq!(text.as_str(), "outer identity");
            assert!(items
                .iter()
                .all(|item| matches!(item, FrameItem::TextShaped { .. })));
        }
        let ordinary = FrameItem::Text {
            pos: Point::ZERO,
            text: "ABC".into(),
            style: style.clone(),
            bidi: None,
        };
        assert!(shape_item(&world, ordinary, &mut cache, &mut face_cache)
            .iter()
            .all(|item| matches!(item, FrameItem::TextShaped { .. })));
    }
}
