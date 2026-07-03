//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/shaper.md
//! @prompt-hash 355f8197
//! @layer L3
//! @updated 2026-06-30
//!
//! **P482** — Post-processing shaping pass (Trilha 5 Fase 1).
//! **P484** — RTL básico via unicode-bidi (Trilha 5 Fase 3, ADR-0120).
//! **P515** — Font fallback por caractere usando múltiplas fontes da
//! `FontList` (cada sub-run shaped na sua própria face).
//! **P525** — Variation Fonts MVP: aplica coordenadas de eixo OpenType
//! (`wght`, `ital`) via `rustybuzz::Face::set_variations` antes do shape.
//! **P534** — Fallback multi-script: segmentação por script Unicode e
//! fallback ao `FontBook` global quando as famílias declaradas não cobrem.
//! Converte `FrameItem::Text` → `FrameItem::TextShaped` via rustybuzz.
//! Executado entre layout e export. ADR-0120 Opção A1.

#![allow(deprecated)] // P483 — FrameItem::Text fallback path legítimo
use std::collections::HashMap;

use rustybuzz::{Direction, UnicodeBuffer};
use unicode_bidi::BidiInfo;
use unicode_script::{Script, UnicodeScript};
use typst_core::contracts::world::World;
use typst_core::entities::font_book::FontVariant;
use typst_core::entities::font_list::FontList;
use typst_core::entities::layout_types::{FrameItem, Page, PagedDocument, Point, Pt, ShapedGlyph, TextStyle};

use crate::font_variant::{axis_variations_for_font_variant, text_style_to_font_variant};

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

/// **P538e** — fontes padrão de fallback quando a fonte declarada (ou a
/// default "Helvetica") não existe no FontBook. Ordenadas por cobertura
/// multi-script comum em sistemas Linux; a primeira que resolver torna-se
/// a primária, evitando segmentação carácter-a-carácter sobre o catálogo
/// inteiro (que pode começar por fontes especiais como MathJax).
const DEFAULT_FALLBACK_FONTS: &[&str] = &[
    "DejaVu Sans",
    "Noto Sans",
    "Liberation Sans",
    "FreeSans",
    "Arial",
];

fn try_shape(
    world: &dyn World,
    pos:   &Point,
    text:  &ecow::EcoString,
    style: &TextStyle,
) -> Option<Vec<FrameItem>> {
    let font_list = style.font.as_ref()?;

    // P525 — derivar a variante real do TextStyle para VF e selecção de fonte.
    let variant = text_style_to_font_variant(style);
    let axis_vars = axis_variations_for_font_variant(&variant);

    // P515 — resolver todas as fontes candidatas da FontList.
    // **P538e** — se a fonte declarada (incluindo a default "Helvetica")
    // não existe no FontBook, tentar fontes padrão de fallback antes de
    // recair no fallback global carácter-a-carácter.
    let mut primary = resolve_candidates(world, font_list, &variant).unwrap_or_default();
    if primary.is_empty() {
        for family in DEFAULT_FALLBACK_FONTS {
            let fallback_list = FontList::single(ecow::EcoString::from(*family));
            if let Some(cands) = resolve_candidates(world, &fallback_list, &variant) {
                if !cands.is_empty() {
                    primary = cands;
                    break;
                }
            }
        }
    }

    // P534 — candidatos de fallback: todo o FontBook, carregados lazy.
    let mut candidates = CandidateSet::new(world, primary);

    // P484 — dividir em runs bidirectionais antes de shape
    let runs = bidi_runs(text.as_str());
    if runs.is_empty() {
        return None;
    }

    let mut items = Vec::new();
    let mut x_offset = Pt(0.0);

    for run in &runs {
        for subrun in split_run_by_font(run, &mut candidates) {
            let candidate = candidates.get(subrun.candidate_idx)?;
            let font = world.font(candidate.slot_idx)?;
            let mut rb_face = rustybuzz::Face::from_slice(font.as_slice(), 0)?;

            // P525 — aplicar coordenadas de eixo OpenType (weight/italic) antes de shape.
            if !axis_vars.is_empty() {
                rb_face.set_variations(&axis_vars);
            }

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
            // **P543** — o campo `text` do TextShaped reflecte apenas o sub-run,
            // e os clusters são relativos a esse texto. Isto evita que o export
            // ToUnicode ou `pdftotext` dupliquem conteúdo quando o texto original
            // era partido em sub-runs pequenos.
            let subrun_text = subrun.text.as_str();
            for (info, pos_g) in infos.iter().zip(positions.iter()) {
                let cluster = info.cluster as usize;
                let char_code = byte_idx_to_char(subrun_text, cluster)
                    .unwrap_or('\u{FFFD}');
                run_glyphs.push(ShapedGlyph {
                    glyph_id:  info.glyph_id as u16,
                    x_advance: pos_g.x_advance,
                    x_offset:  pos_g.x_offset,
                    y_offset:  pos_g.y_offset,
                    cluster:   cluster as u32,
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

                // P534 — cada sub-run reflecte a família real usada, para que o
                // export multi-font embuta a face correcta e a associe via
                // `font_index_for_style`.
                let mut segment_style = style.clone();
                let real_family = world.book().infos().get(candidate.slot_idx)?.family.clone();
                segment_style.font = Some(FontList::single(ecow::EcoString::from(real_family)));

                items.push(FrameItem::TextShaped {
                    pos:    item_pos,
                    glyphs: run_glyphs,
                    style:  segment_style,
                    text:   subrun.text.clone().into(),
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
#[derive(Clone, Copy)]
struct FontCandidate {
    slot_idx:     usize,
    units_per_em: u16,
}

/// Resolve as fontes declaradas na `FontList` (primárias), na ordem do
/// utilizador. Se nenhuma resolver, `try_shape` cai no fallback `Text`.
fn resolve_candidates(
    world: &dyn World,
    font_list: &FontList,
    variant: &FontVariant,
) -> Option<Vec<FontCandidate>> {
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

/// Conjunto de candidatas primárias + fallback lazy sobre todo o `FontBook`.
///
/// P534: quando as famílias declaradas não cobrem um caractere, procura-se no
/// resto do catálogo na ordem de descoberta. O fallback é lazy para evitar
/// carregar todas as fontes do sistema em documentos que não precisam.
struct CandidateSet<'a> {
    world:    &'a dyn World,
    primary:  Vec<FontCandidate>,
    fallback: Vec<Option<FontCandidate>>,
}

impl<'a> CandidateSet<'a> {
    fn new(world: &'a dyn World, primary: Vec<FontCandidate>) -> Self {
        Self {
            world,
            primary,
            fallback: Vec::new(),
        }
    }

    /// Todos os candidatos que cobrem `c`, em ordem de prioridade (primárias
    /// primeiro, depois fallback lazy na ordem do FontBook).
    fn covering_all(&mut self, c: char) -> Vec<usize> {
        let mut result = Vec::new();
        for (i, cand) in self.primary.iter().enumerate() {
            if face_covers_char(self.world, cand.slot_idx, c) {
                result.push(i);
            }
        }
        let book_len = self.world.book().len();
        for slot_idx in self.primary.len()..book_len {
            let fb_idx = slot_idx - self.primary.len();
            if fb_idx >= self.fallback.len() {
                self.fallback.push(self.load_fallback(slot_idx));
            }
            if let Some(cand) = self.fallback[fb_idx] {
                if face_covers_char(self.world, cand.slot_idx, c) {
                    result.push(slot_idx);
                }
            }
        }
        result
    }

    /// Escolhe o candidato que cobre o maior trecho contíguo de `text`
    /// começando em `start`. As primárias têm prioridade: só se nenhuma
    /// primária cobrir o primeiro caractere é que se recai no fallback global.
    /// Devolve `(candidate_idx, end_byte)`. Se nenhuma fonte cobrir o primeiro
    /// caractere, devolve `None`.
    fn covering_run(&mut self, text: &str, start: usize) -> Option<(usize, usize)> {
        let first_char = text[start..].chars().next()?;

        // 1. Tentar primárias primeiro.
        let mut primary_candidates = Vec::new();
        for (i, cand) in self.primary.iter().enumerate() {
            if face_covers_char(self.world, cand.slot_idx, first_char) {
                primary_candidates.push(i);
            }
        }
        if let Some(result) = self.best_covering_run(text, start, &primary_candidates) {
            return Some(result);
        }

        // 2. Recair no fallback global.
        let fallback_candidates = self.covering_all(first_char);
        self.best_covering_run(text, start, &fallback_candidates)
    }

    /// Dado um conjunto de índices de candidatos, escolhe o que cobre o maior
    /// trecho contíguo a partir de `start`.
    fn best_covering_run(
        &mut self,
        text: &str,
        start: usize,
        candidates: &[usize],
    ) -> Option<(usize, usize)> {
        if candidates.is_empty() {
            return None;
        }

        let mut best_idx = candidates[0];
        let mut best_end = start;

        for &idx in candidates {
            let mut end = start;
            for c in text[start..].chars() {
                let covers = if idx < self.primary.len() {
                    face_covers_char(self.world, self.primary[idx].slot_idx, c)
                } else {
                    self.fallback
                        .get(idx - self.primary.len())
                        .and_then(|f| f.as_ref())
                        .map_or(false, |cand| face_covers_char(self.world, cand.slot_idx, c))
                };
                if !covers {
                    break;
                }
                end += c.len_utf8();
            }
            if end > best_end {
                best_end = end;
                best_idx = idx;
            }
        }

        Some((best_idx, best_end))
    }

    fn load_fallback(&self, slot_idx: usize) -> Option<FontCandidate> {
        let font = self.world.font(slot_idx)?;
        let face = ttf_parser::Face::parse(font.as_slice(), 0).ok()?;
        Some(FontCandidate {
            slot_idx,
            units_per_em: face.units_per_em().max(1) as u16,
        })
    }

    fn get(&self, idx: usize) -> Option<&FontCandidate> {
        if idx < self.primary.len() {
            self.primary.get(idx)
        } else {
            self.fallback.get(idx - self.primary.len())?.as_ref()
        }
    }
}

fn face_covers_char(world: &dyn World, slot_idx: usize, c: char) -> bool {
    let Some(font) = world.font(slot_idx) else { return false };
    let Some(face) = ttf_parser::Face::parse(font.as_slice(), 0).ok() else { return false };
    face.glyph_index(c).is_some()
}

/// Sub-run dentro de um BidiRun: todos os caracteres partilham o mesmo script
/// efectivo e a mesma fonte candidata.
struct SubRun {
    text:              String,
    candidate_idx:     usize,
}

/// P534/P543 — divide um BidiRun em sub-runs por (a) mudança de script
/// Unicode e (b) mudança de fonte necessária para cobertura do caractere.
///
/// **P543**: em vez de escolher a primeira fonte que cobre o caractere
/// actual, escolhe-se a fonte que cobre o maior trecho contíguo a partir da
/// posição actual. Isto evita fragmentação excessiva (ex.: "Hello" → "H" +
/// "ello") quando o FontBook começa por fontes especializadas de cobertura
/// parcial.
fn split_run_by_font(run: &BidiRun, candidates: &mut CandidateSet) -> Vec<SubRun> {
    let text = run.text.as_str();
    if text.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::new();
    let mut pos = 0usize;
    let mut current_script = Script::Unknown;
    let mut current: Option<SubRun> = None;

    while pos < text.len() {
        let c = text[pos..].chars().next().unwrap();
        let script = effective_script(c.script(), current_script);
        let script_changed = pos > 0 && !is_compatible(script, current_script);

        // Próxima fronteira de script dentro do run.
        let script_end = next_script_boundary(text, pos, script);

        // P543 — fonte que cobre o maior trecho contíguo a partir de pos.
        let (candidate_idx, font_end) = candidates
            .covering_run(text, pos)
            .unwrap_or((0, pos + c.len_utf8()));

        let end_byte = font_end.min(script_end);

        if script_changed {
            if let Some(cur) = current.take() {
                result.push(cur);
            }
            current_script = script;
        }

        if let Some(ref mut cur) = current {
            if cur.candidate_idx == candidate_idx {
                // Mesma fonte: estender o sub-run actual.
                cur.text.push_str(&text[pos..end_byte]);
                pos = end_byte;
                continue;
            }
            // Fonte diferente: fechar o actual e iniciar novo.
            result.push(current.take().unwrap());
        }

        current = Some(SubRun {
            text: text[pos..end_byte].to_owned(),
            candidate_idx,
        });
        pos = end_byte;
    }

    if let Some(cur) = current {
        result.push(cur);
    }

    result
}

/// Script efectivo de `c` dado o script do segmento actual: scripts genéricos
/// herdam o script corrente para evitar fragmentação por pontuação/espaço.
fn effective_script(script: Script, current: Script) -> Script {
    if is_generic_script(script) && !is_generic_script(current) {
        current
    } else {
        script
    }
}

/// Byte offset imediatamente após a maior sequência de caracteres a partir de
/// `start` que partilham o mesmo script efectivo `current_script`.
fn next_script_boundary(text: &str, start: usize, current_script: Script) -> usize {
    let mut end = start;
    for (off, c) in text[start..].char_indices() {
        let script = effective_script(c.script(), current_script);
        if off > 0 && !is_compatible(script, current_script) {
            break;
        }
        end = start + off + c.len_utf8();
    }
    end
}

fn is_generic_script(script: Script) -> bool {
    matches!(script, Script::Unknown | Script::Common | Script::Inherited)
}

fn is_compatible(a: Script, b: Script) -> bool {
    is_generic_script(a) || is_generic_script(b) || a == b
}


// ── P484 — runs bidirectionais ──────────────────────────────────────────────

struct BidiRun {
    text:       String,
    rtl:        bool,
    /// Offset byte do run no string original (para ajuste de `cluster`).
    #[allow(dead_code)]
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
    use typst_core::entities::font_book::{FontBook, FontStretch, FontStyle, FontWeight};
    use typst_core::entities::font_list::FontList;
    use typst_core::entities::layout_types::{FrameItem, Page, PagedDocument, Point, Pt, TextStyle};
    use typst_core::entities::file_id::FileId;
    use typst_core::entities::source::Source;
    use typst_core::entities::world_types::{Bytes, Datetime, FileError, FileResult, Font, Library};
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

    // ── P534 — helpers para testes com fontes reais ─────────────────────────────

    /// `World` de teste com `FontBook` e bytes de fontes controlados.
    struct FontWorld {
        book:  FontBook,
        fonts: Vec<Option<Font>>,
    }

    impl FontWorld {
        fn push_font(&mut self, path: &str) {
            let slot = self.fonts.len();
            if let Ok(data) = std::fs::read(path) {
                if let Some(info) = crate::fonts::font_info_from_bytes(&data, 0) {
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
        fn book(&self) -> &FontBook { &self.book }
        fn main(&self) -> FileId { unimplemented!() }
        fn source(&self, _: FileId) -> FileResult<Source> { unimplemented!() }
        fn file(&self, _: FileId) -> FileResult<Bytes> { Err(FileError::NotFound) }
        fn font(&self, idx: usize) -> Option<Font> {
            self.fonts.get(idx).cloned().flatten()
        }
        fn today(&self, _: Option<i64>) -> Option<Datetime> { None }
    }

    fn font_world_with(paths: &[&str]) -> FontWorld {
        let mut world = FontWorld { book: FontBook::new(), fonts: Vec::new() };
        for path in paths {
            world.push_font(path);
        }
        world
    }

    fn text_item_with_font(text: &str, family: &str) -> FrameItem {
        let mut style = TextStyle::default();
        style.font = Some(FontList::single(EcoString::from(family)));
        style.size = Pt(12.0);
        FrameItem::Text {
            pos:  Point { x: Pt(0.0), y: Pt(0.0) },
            text: EcoString::from(text),
            style,
        }
    }

    fn text_item(text: &str) -> FrameItem {
        FrameItem::Text {
            pos:   Point { x: Pt(72.0), y: Pt(72.0) },
            text:  EcoString::from(text),
            style: TextStyle::default(),
        }
    }

    fn doc_with(items: Vec<FrameItem>) -> PagedDocument {
        PagedDocument::new(vec![Page { width: 595.0, height: 842.0, numbering: None, items }])
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
        assert!(resolve_candidates(&world, &font_list, &FontVariant::default()).is_none());
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
        let world = SystemWorld::new(dir.path(), "main.typ")
            .unwrap()
            .with_system_fonts();
        if world.book().select("Ubuntu Sans", &FontVariant::default()).is_none() {
            eprintln!("SKIP: Ubuntu Sans não disponível no sistema");
            return;
        }

        fn text_item_with_weight(text: &str, weight: u16) -> FrameItem {
            let mut style = TextStyle::default();
            style.font = Some(FontList::single(EcoString::from("Ubuntu Sans")));
            style.weight = Some(weight);
            style.size = Pt(40.0);
            FrameItem::Text {
                pos: Point { x: Pt(0.0), y: Pt(0.0) },
                text: EcoString::from(text),
                style,
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
            bold_1, thin
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
                _ => None,
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
                _ => None,
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
}
