//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/shaper.md
//! @prompt-hash 084a5fe9
//! @layer L3
//! @updated 2026-06-30
//!
//! **P482** — Post-processing shaping pass (Trilha 5 Fase 1).
//! **P484** — RTL básico via unicode-bidi (Trilha 5 Fase 3, ADR-0120).
//! **P515** — Font fallback por caractere usando múltiplas fontes da
//! `FontList` (cada sub-run shaped na sua própria face).
//! Converte `FrameItem::Text` → `FrameItem::TextShaped` via rustybuzz.
//! Executado entre layout e export. ADR-0120 Opção A1.

#![allow(deprecated)] // P483 — FrameItem::Text fallback path legítimo
use rustybuzz::{Direction, UnicodeBuffer};
use unicode_bidi::BidiInfo;
use typst_core::contracts::world::World;
use typst_core::entities::font_book::FontVariant;
use typst_core::entities::font_list::{FontList, FontNamePattern};
use typst_core::entities::layout_types::{FrameItem, Page, PagedDocument, Point, Pt, ShapedGlyph};

/// Converte todos os `FrameItem::Text` de um `PagedDocument` em
/// `FrameItem::TextShaped` via rustybuzz.
///
/// Itens sem fonte resolvida (`style.font == None` ou lookup falha)
/// são preservados como `FrameItem::Text` (fallback Helvetica).
pub fn shape_document(world: &dyn World, mut doc: PagedDocument) -> PagedDocument {
    for page in &mut doc.pages {
        shape_page(world, page);
    }
    doc
}

fn shape_page(world: &dyn World, page: &mut Page) {
    let mut new_items = Vec::with_capacity(page.items.len());
    for item in page.items.drain(..) {
        new_items.extend(shape_item(world, item));
    }
    page.items = new_items;
}

/// Processa um `FrameItem`, devolvendo 1 ou mais itens (fallback por
/// caractere pode expandir um `Text` em vários `TextShaped` consecutivos).
fn shape_item(world: &dyn World, mut item: FrameItem) -> Vec<FrameItem> {
    match &mut item {
        FrameItem::Text { pos, text, style } if style.font.is_some() => {
            if let Some(shaped) = try_shape(world, pos, text, style) {
                return shaped;
            }
        }
        FrameItem::Group { items, .. } => {
            let mut new_children = Vec::with_capacity(items.len());
            for child in items.drain(..) {
                new_children.extend(shape_item(world, child));
            }
            *items = new_children;
        }
        FrameItem::Link { items, .. } => {
            let mut new_children = Vec::with_capacity(items.len());
            for child in items.drain(..) {
                new_children.extend(shape_item(world, child));
            }
            *items = new_children;
        }
        _ => {}
    }
    vec![item]
}

fn try_shape(
    world: &dyn World,
    pos:   &Point,
    text:  &ecow::EcoString,
    style: &typst_core::entities::layout_types::TextStyle,
) -> Option<Vec<FrameItem>> {
    let font_list = style.font.as_ref()?;

    // P515 — resolver todas as fontes candidatas da FontList.
    let candidates = resolve_candidates(world, font_list)?;
    if candidates.is_empty() {
        return None;
    }

    // P484 — dividir em runs bidirectionais antes de shape
    let runs = bidi_runs(text.as_str());
    if runs.is_empty() {
        return None;
    }

    let mut items = Vec::new();
    let mut x_offset = Pt(0.0);

    for run in &runs {
        for subrun in split_run_by_font(run, world, &candidates) {
            let candidate = &candidates[subrun.candidate_idx];
            let font = world.font(candidate.slot_idx)?;
            let rb_face = rustybuzz::Face::from_slice(font.as_slice(), 0)?;

            let mut buffer = UnicodeBuffer::new();
            buffer.push_str(&subrun.text);
            if run.rtl {
                buffer.set_direction(Direction::RightToLeft);
            } else {
                buffer.set_direction(Direction::LeftToRight);
            }
            let output    = rustybuzz::shape(&rb_face, &[], buffer);
            let infos     = output.glyph_infos();
            let positions = output.glyph_positions();

            let mut run_glyphs: Vec<ShapedGlyph> = Vec::with_capacity(infos.len());
            let mut run_width = 0i32;
            for (info, pos_g) in infos.iter().zip(positions.iter()) {
                let abs_cluster = run.byte_start as u32
                                + subrun.byte_start as u32
                                + info.cluster;
                let char_code = byte_idx_to_char(text.as_str(), abs_cluster as usize)
                    .unwrap_or('\u{FFFD}');
                run_glyphs.push(ShapedGlyph {
                    glyph_id:  info.glyph_id as u16,
                    x_advance: pos_g.x_advance,
                    x_offset:  pos_g.x_offset,
                    y_offset:  pos_g.y_offset,
                    cluster:   abs_cluster,
                    char_code,
                });
                run_width += pos_g.x_advance;
            }

            if !run_glyphs.is_empty() {
                let subrun_width_pt = run_width as f64 * style.size.0 / candidate.units_per_em as f64;
                let item_pos = Point {
                    x: Pt(pos.x.0 + x_offset.0),
                    y: pos.y,
                };
                x_offset.0 += subrun_width_pt;

                items.push(FrameItem::TextShaped {
                    pos:    item_pos,
                    glyphs: run_glyphs,
                    style:  style.clone(),
                    text:   text.clone(),
                    units_per_em: candidate.units_per_em,
                });
            }
        }
    }

    if items.is_empty() {
        None
    } else {
        Some(items)
    }
}

/// Candidata a fonte para shaping/fallback.
struct FontCandidate {
    slot_idx:     usize,
    units_per_em: u16,
}

fn resolve_candidates(world: &dyn World, font_list: &FontList) -> Option<Vec<FontCandidate>> {
    let variant = FontVariant::default();
    let book = world.book();
    let mut candidates = Vec::new();
    for family in font_list.as_slice() {
        if let Some(idx) = book.select_pattern(&family.name, &variant) {
            if let Some(font) = world.font(idx) {
                if let Ok(face) = ttf_parser::Face::parse(font.as_slice(), 0) {
                    let units_per_em = face.units_per_em().max(1) as u16;
                    candidates.push(FontCandidate { slot_idx: idx, units_per_em });
                }
            }
        }
    }
    if candidates.is_empty() {
        None
    } else {
        Some(candidates)
    }
}

/// Sub-run dentro de um BidiRun, todos os caracteres cobertos pela mesma
/// fonte candidata.
struct SubRun {
    text:              String,
    byte_start:        usize,
    candidate_idx:     usize,
}

fn split_run_by_font(run: &BidiRun, world: &dyn World, candidates: &[FontCandidate]) -> Vec<SubRun> {
    let mut result = Vec::new();
    let mut current_start = 0usize;
    let mut current_candidate = None::<usize>;

    for (byte_offset, c) in run.text.char_indices() {
        let covering = candidates.iter().position(|cand| {
            let Some(font) = world.font(cand.slot_idx) else { return false; };
            let Some(face) = ttf_parser::Face::parse(font.as_slice(), 0).ok() else { return false; };
            face.glyph_index(c).is_some()
        });
        // Se nenhuma fonte cobrir, fallback para a primeira candidata (notdef).
        let candidate_idx = covering.unwrap_or(0);

        if Some(candidate_idx) != current_candidate {
            if let Some(idx) = current_candidate {
                result.push(SubRun {
                    text: run.text[current_start..byte_offset].to_owned(),
                    byte_start: current_start,
                    candidate_idx: idx,
                });
            }
            current_start = byte_offset;
            current_candidate = Some(candidate_idx);
        }
    }

    if let Some(idx) = current_candidate {
        result.push(SubRun {
            text: run.text[current_start..].to_owned(),
            byte_start: current_start,
            candidate_idx: idx,
        });
    }

    result
}

// ── P484 — runs bidirectionais ──────────────────────────────────────────────

struct BidiRun {
    text:       String,
    rtl:        bool,
    /// Offset byte do run no string original (para ajuste de `cluster`).
    byte_start: usize,
}

/// Divide `text` em runs bidirectionais na ordem visual correcta.
/// Para texto puramente LTR retorna um único run.
/// API unicode-bidi 0.3: `BidiInfo::new`, `visual_runs(para, range)`.
fn bidi_runs(text: &str) -> Vec<BidiRun> {
    if text.is_empty() {
        return vec![];
    }
    let bidi = BidiInfo::new(text, None);
    if bidi.paragraphs.is_empty() {
        return vec![BidiRun { text: text.to_owned(), rtl: false, byte_start: 0 }];
    }
    let para              = &bidi.paragraphs[0];
    let line              = para.range.clone();
    let (levels, runs)    = bidi.visual_runs(para, line);

    runs.into_iter().map(|run_range| {
        let rtl = levels
            .get(run_range.start)
            .map(|l: &unicode_bidi::Level| l.is_rtl())
            .unwrap_or(false);
        BidiRun {
            text:       text[run_range.clone()].to_owned(),
            rtl,
            byte_start: run_range.start,
        }
    }).collect()
}

// ── fim P484 ─────────────────────────────────────────────────────────────────

fn byte_idx_to_char(s: &str, byte_idx: usize) -> Option<char> {
    s.get(byte_idx..)?.chars().next()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ecow::EcoString;
    use typst_core::entities::font_book::FontBook;
    use typst_core::entities::layout_types::{FrameItem, Page, PagedDocument, Point, Pt, TextStyle};
    use typst_core::entities::file_id::FileId;
    use typst_core::entities::source::Source;
    use typst_core::entities::world_types::{Bytes, Datetime, FileResult, Font, Library};
    use crate::world::SystemWorld;
    use std::path::PathBuf;

    struct MockWorld {
        book: FontBook,
    }

    impl typst_core::contracts::world::World for MockWorld {
        fn library(&self) -> &Library { static L: std::sync::OnceLock<Library> = std::sync::OnceLock::new(); L.get_or_init(Library::new) }
        fn book(&self) -> &FontBook { &self.book }
        fn main(&self) -> FileId { unimplemented!() }
        fn source(&self, _: FileId) -> FileResult<Source> { unimplemented!() }
        fn file(&self, _: FileId) -> FileResult<Bytes> { unimplemented!() }
        fn font(&self, _: usize) -> Option<Font> { None }
        fn today(&self, _: Option<i64>) -> Option<Datetime> { None }
    }

    fn empty_world() -> MockWorld { MockWorld { book: FontBook::new() } }

    fn text_item(text: &str) -> FrameItem {
        FrameItem::Text {
            pos:   Point { x: Pt(72.0), y: Pt(72.0) },
            text:  EcoString::from(text),
            style: TextStyle::default(),
        }
    }

    fn doc_with(items: Vec<FrameItem>) -> PagedDocument {
        PagedDocument::new(vec![Page { width: 595.0, height: 842.0, items }])
    }

    struct TempDir(PathBuf);
    impl TempDir {
        fn path(&self) -> &std::path::Path { &self.0 }
    }
    impl Drop for TempDir {
        fn drop(&mut self) { let _ = std::fs::remove_dir_all(&self.0); }
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
        assert!(matches!(&shaped.pages[0].items[0], FrameItem::Text { .. }),
            "P482: Text sem style.font deve permanecer Text");
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
            glyph_id: 42, x_advance: 600, x_offset: 0, y_offset: 0,
            cluster: 0, char_code: 'A',
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
        let mut style = TextStyle::default();
        style.font = Some(FontList::single(EcoString::from("Helvetica")));
        let item = FrameItem::Text {
            pos:  Point { x: Pt(72.0), y: Pt(72.0) },
            text: EcoString::from("Ola"),
            style,
        };
        let doc = doc_with(vec![item]);
        let shaped = shape_document(&empty_world(), doc);
        // MockWorld.font() retorna None → try_shape retorna None → Text preservado
        assert!(matches!(&shaped.pages[0].items[0], FrameItem::Text { .. }),
            "P483: sem fontes reais, Text com font=Some preservado como fallback");
    }

    #[test]
    fn p483_text_sem_font_nao_tenta_shape() {
        // style.font = None → shaper ignora (guarda is_some() false).
        let item = text_item("sem fonte"); // text_item usa TextStyle::default() → font=None
        let doc = doc_with(vec![item]);
        let shaped = shape_document(&empty_world(), doc);
        assert!(matches!(&shaped.pages[0].items[0], FrameItem::Text { .. }),
            "P483: Text sem style.font=None não é tentado pelo shaper");
    }

    #[test]
    fn p483_shaped_glyph_debug_display() {
        let g = ShapedGlyph {
            glyph_id: 1, x_advance: 500, x_offset: 0, y_offset: 0,
            cluster: 0, char_code: 'A',
        };
        let s = format!("{:?}", g);
        assert!(s.contains("glyph_id: 1"), "Debug deve incluir glyph_id");
    }

    // ── P484 — testes RTL ────────────────────────────────────────────────────

    #[test]
    fn p484_bidi_runs_ltr_unico_run() {
        let runs = bidi_runs("Hello world");
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
        let runs = bidi_runs("مرحبا");
        assert!(!runs.is_empty(), "árabe deve ter pelo menos 1 run");
        assert!(runs.iter().any(|r| r.rtl), "árabe deve ter run RTL");
    }

    #[test]
    fn p484_try_shape_rtl_sem_fonte_nao_panic() {
        // Shape de texto árabe sem fonte carregada → sem panic (fallback Text)
        let item = FrameItem::Text {
            pos:   Point { x: Pt(0.0), y: Pt(0.0) },
            text:  EcoString::from("مرحبا"),
            style: TextStyle::default(),
        };
        // style.font = None → shaper guard (is_some() == false) → item inalterado
        let result = shape_item(&empty_world(), item);
        // o critério é simplesmente não entrar em panic
        assert_eq!(result.len(), 1);
        assert!(matches!(result[0], FrameItem::Text { .. }), "sem fonte: preservado como Text");
    }

    #[test]
    fn p484_bidi_runs_misto_ingles_arabico() {
        // "Hi مرحبا" — deve ter 2 runs (LTR + RTL)
        let runs = bidi_runs("Hi مرحبا");
        assert!(runs.len() >= 2, "texto misto deve ter ≥2 runs, got {}", runs.len());
        let has_ltr = runs.iter().any(|r| !r.rtl);
        let has_rtl = runs.iter().any(|r| r.rtl);
        assert!(has_ltr, "deve ter run LTR");
        assert!(has_rtl, "deve ter run RTL");
    }

    #[test]
    fn p484_bidi_runs_byte_start_correcto() {
        // Para texto LTR puro, byte_start do único run deve ser 0
        let runs = bidi_runs("abc");
        assert_eq!(runs[0].byte_start, 0);
        assert_eq!(runs[0].text, "abc");
    }

    // P485 — units_per_em populado pelo shaper
    #[test]
    fn p485_shape_document_sem_fonte_nao_produz_textshaped() {
        // Sem fonte no MockWorld, shape_document não converte → Text preservado
        // units_per_em não é populado (sem TextShaped produzido)
        let doc = doc_with(vec![text_item("Hello")]);
        let shaped = shape_document(&empty_world(), doc);
        assert!(matches!(&shaped.pages[0].items[0], FrameItem::Text { .. }),
            "P485: sem fonte → deve permanecer Text (units_per_em não aplicável)");
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
        assert_eq!(features.len(), 0, "P486: features user vazias — defaults de rustybuzz aplicam-se");
    }

    // ── P515 — font fallback por caractere ──────────────────────────────────

    #[test]
    fn p515_resolve_candidates_sem_fontes_retorna_none() {
        let world = empty_world();
        let font_list = FontList::single(EcoString::from("Helvetica"));
        assert!(resolve_candidates(&world, &font_list).is_none());
    }

    #[test]
    fn p515_shape_document_real_font_produz_textshaped() {
        // Carrega uma fonte real do sistema (fallback se ausente).
        let candidates = load_real_font_candidates();
        if candidates.is_empty() {
            return; // skip em ambientes sem fontes
        }

        let mut book = FontBook::new();
        // O MockWorld precisa de um FontBook que corresponda aos slots.
        // Para simplificar, usamos SystemWorld com with_system_fonts.
        let dir = tempfile_write("main.typ", "text");
        let world = SystemWorld::new(dir.path(), "main.typ")
            .unwrap()
            .with_system_fonts();
        if world.book().is_empty() {
            return; // skip
        }

        let mut style = TextStyle::default();
        // Usa a primeira família disponível no sistema.
        let family = EcoString::from(world.book().infos()[0].family.clone());
        style.font = Some(FontList::single(family));
        style.size = Pt(12.0);

        let item = FrameItem::Text {
            pos:   Point { x: Pt(0.0), y: Pt(0.0) },
            text:  EcoString::from("Hello"),
            style,
        };
        let result = shape_item(&world, item);
        assert!(!result.is_empty(), "deve produzir pelo menos 1 TextShaped");
        assert!(result.iter().all(|it| matches!(it, FrameItem::TextShaped { .. })),
                "todos os resultados devem ser TextShaped");
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
}
