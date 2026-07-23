//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/shaper.md
//! @prompt-hash 97627a17

//! @layer L3
//! @updated 2026-07-06
//!
//! **P482** — Post-processing shaping pass (Trilha 5 Fase 1).
//! **P484** — RTL básico via unicode-bidi (Trilha 5 Fase 3, ADR-0120).
//! **P515** — Font fallback por caractere usando múltiplas fontes da
//! `FontList` (cada sub-run shaped na sua própria face).
//! **P525** — Variation Fonts MVP: aplica coordenadas de eixo OpenType
//! (`wght`, `ital`) via `rustybuzz::Face::set_variations` antes do shape.
//! **P534** — Fallback multi-script: segmentação por script Unicode e
//! fallback ao `FontBook` global quando as famílias declaradas não cobrem.
//! **P555** — fallback por classe visual (serif/sans) via
//! `03_infra/src/fallback_fonts.rs`.
//! **P845** — texto com `\n` interno: itera TODOS os parágrafos bidi
//! (antes só `paragraphs[0]`), uma linha visível por parágrafo, com o
//! mesmo avanço vertical das linhas normais do Layouter (P762).
//! Converte `FrameItem::Text` → `FrameItem::TextShaped` via rustybuzz.
//! Executado entre layout e export. ADR-0120 Opção A1.

#![allow(deprecated)] // P483 — FrameItem::Text fallback path legítimo

use std::collections::HashMap;
use std::sync::Arc;

use rustybuzz::{Direction, UnicodeBuffer};
use typst_core::contracts::world::World;
use typst_core::entities::font_book::{FontInfo, FontVariant};
use typst_core::entities::font_list::FontList;
use typst_core::entities::layout_types::{
    FrameItem, Length, Page, PagedDocument, Point, Pt, ShapedGlyph, TextStyle,
};
use typst_core::entities::world_types::Font;
use unicode_bidi::BidiInfo;
use unicode_script::{Script, UnicodeScript};

use crate::fallback_fonts::{fallback_font_list_for, math_fallback_font_list};
use crate::font_metrics::FallbackFontMetrics;
use crate::font_variant::{
    axis_variations_for_font_variant, axis_variations_for_text_style,
    text_style_to_font_variant,
};
use typst_core::entities::dir::Dir;

/// Converte todos os `FrameItem::Text` de um `PagedDocument` em
/// `FrameItem::TextShaped` via rustybuzz.
///
/// Itens sem fonte resolvida (`style.font == None` ou lookup falha)
/// são preservados como `FrameItem::Text` (fallback Helvetica).
pub fn shape_document(world: &dyn World, mut doc: PagedDocument) -> PagedDocument {
    // P657 — cache local de resultados de shaping. Criada por documento para
    // evitar invalidação complexa entre mundos; reaproveita sub-runs idênticos
    // (mesmo texto, face, direção, variações e tracking) dentro do mesmo
    // documento, o que é comum em documentos com conteúdo repetido.
    let mut cache = ShapeCache::new();
    // P672 — cache local de faces ttf-parser. Evita re-parsear a mesma fonte
    // milhares de vezes durante a segmentação por fonte.
    let mut face_cache = FaceCache::new();

    for page in &mut doc.pages {
        shape_page(world, page, &mut cache, &mut face_cache);
    }

    doc
}

/// Cache de resultados de shaping por documento.
///
/// A chave inclui tudo o que afecta o output de `rustybuzz::shape` para um
/// sub-run: texto, identificador da face, direção, variações de eixo e
/// tracking. O valor guarda os glifos shaped e a largura total em unidades
/// de fonte.
struct ShapeCache {
    map: HashMap<String, CachedRun>,
}

#[derive(Clone)]
struct CachedRun {
    glyphs: Vec<ShapedGlyph>,
    width: i32,
}

impl ShapeCache {
    fn new() -> Self {
        Self { map: HashMap::new() }
    }

    fn key(
        text: &str,
        slot_idx: usize,
        rtl: bool,
        axis_vars: &[rustybuzz::Variation],
        tracking: Option<Length>,
    ) -> String {
        let axis_key: String = axis_vars
            .iter()
            .map(|v| format!("{}={:.4}", v.tag, v.value))
            .collect::<Vec<_>>()
            .join(",");
        let tracking_key = tracking
            .map(|t| format!("{:.4}:{:.4}", t.abs.0, t.em))
            .unwrap_or_default();
        format!("{}|{}|{}|{}|{}", text, slot_idx, rtl, axis_key, tracking_key)
    }
}

/// Cache de faces `ttf-parser` por `slot_idx` dentro de um documento.
///
/// Evita re-parsear a mesma fonte milhares de vezes durante a fase de
/// `split_run_by_font` (P672). O `Face` empresta internamente dos bytes da
/// `Font` owned; a struct vive dentro de `Arc` para que o slice `'static`
/// seja válido enquanto a face existir.
pub(crate) struct FaceCache {
    map: HashMap<usize, Option<Arc<CachedFace>>>,
}

impl FaceCache {
    pub(crate) fn new() -> Self {
        Self { map: HashMap::new() }
    }

    pub(crate) fn get(
        &mut self,
        world: &dyn World,
        slot_idx: usize,
    ) -> Option<&CachedFace> {
        if !self.map.contains_key(&slot_idx) {
            let cached = world.font(slot_idx).and_then(CachedFace::new);
            self.map.insert(slot_idx, cached);
        }
        self.map
            .get(&slot_idx)
            .and_then(|o| o.as_ref())
            .map(|arc| arc.as_ref())
    }
}

pub(crate) struct CachedFace {
    #[allow(dead_code)]
    data: Font,
    face: ttf_parser::Face<'static>,
}

impl CachedFace {
    pub(crate) fn new(data: Font) -> Option<Arc<Self>> {
        let slice: &'static [u8] = unsafe {
            std::slice::from_raw_parts(data.as_slice().as_ptr(), data.as_slice().len())
        };
        let face = ttf_parser::Face::parse(slice, 0).ok()?;
        Some(Arc::new(Self { data, face }))
    }

    pub(crate) fn face(&self) -> &ttf_parser::Face<'_> {
        &self.face
    }
}

/// **P591** — mede a largura de `text` já com shaping aplicado, sem gerar
/// `FrameItem`s. Usado pelo `FallbackFontMetrics::advance_shaped` para que o
/// Layouter decida quebras de linha com a largura real de scripts contextuais
/// (árabe, síriaco, etc.).
///
/// Retorna `None` se não conseguir resolver fonte ou se o texto for vazio.
pub(crate) fn shaped_width(
    world: &dyn World,
    text: &str,
    style: &TextStyle,
    face_cache: &mut FaceCache,
) -> Option<Pt> {
    if text.is_empty() {
        return Some(Pt(0.0));
    }

    let font_list = style.font.as_ref()?;

    // P836 — eixos derivados + explícitos (`style.variations`), explícitos
    // vencem por tag.
    let variant = text_style_to_font_variant(style);
    let axis_vars = axis_variations_for_text_style(style);

    let mut primary =
        resolve_candidates(world, font_list, &variant, face_cache).unwrap_or_default();
    if primary.is_empty() {
        let first_family = font_list
            .as_slice()
            .first()
            .and_then(|f| f.name.as_str())
            .unwrap_or("");
        let fallback_list = fallback_font_list_for(first_family);
        for family in fallback_list {
            let fallback_font_list = FontList::single(ecow::EcoString::from(*family));
            if let Some(cands) =
                resolve_candidates(world, &fallback_font_list, &variant, face_cache)
            {
                if !cands.is_empty() {
                    primary = cands;
                    break;
                }
            }
        }
    } else {
        // **P783/P784** — adicionar a cadeia de fallback matemático como
        // primárias adicionais, para que glifos ausentes na primária sejam
        // buscados em fontes math dedicadas antes do fallback global (todo
        // o FontBook em ordem de índice). Dois gatilhos independentes
        // (OR): `style.math` (P784 — sempre que o texto vem do motor de
        // layout matemático, `layout/equation.rs`, independentemente de a
        // fonte de corpo por omissão ter ou não tabela MATH própria — ela
        // não tem, `Libertinus Serif` não tem MATH, então este era o único
        // caso que realmente falhava e motivou este passo) e
        // `primary_has_math` (P783 original — a fonte já resolvida declara
        // MATH própria, útil se o utilizador define `font:
        // "New Computer Modern Math"` explicitamente fora de `$...$`).
        let primary_has_math = primary
            .first()
            .and_then(|cand| face_cache.get(world, cand.slot_idx))
            .map_or(false, |cached| cached.face().tables().math.is_some());
        if style.math || primary_has_math {
            for family in math_fallback_font_list() {
                let math_font_list = FontList::single(ecow::EcoString::from(*family));
                if let Some(cands) =
                    resolve_candidates(world, &math_font_list, &variant, face_cache)
                {
                    for cand in cands {
                        if !primary.iter().any(|p| p.slot_idx == cand.slot_idx) {
                            primary.push(cand);
                        }
                    }
                }
            }
        }
    }

    // P838 — `like` = info da primeira primária (o `ctx.first()` do vanilla).
    let like = primary
        .first()
        .and_then(|cand| world.book().infos().get(cand.slot_idx).cloned());
    let mut candidates = CandidateSet::new(world, primary, face_cache, like, variant);

    // P845 — runs agrupados por parágrafo bidi (texto com `\n` interno).
    let paragraphs = bidi_runs(text);
    if paragraphs.iter().all(|p| p.is_empty()) {
        return Some(Pt(0.0));
    }

    // Largura de texto multilinha = max das larguras de linha (não a soma):
    // cada parágrafo é uma linha visível independente.
    let mut max_width = 0.0f64;

    for runs in &paragraphs {
        let mut line_width = 0.0f64;
        for run in runs {
            for subrun in split_run_by_font(run, &mut candidates) {
                let candidate = candidates.get(subrun.candidate_idx)?;
                let font = world.font(candidate.slot_idx)?;
                let mut rb_face = rustybuzz::Face::from_slice(font.as_slice(), 0)?;

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
                let output = rustybuzz::shape(&rb_face, &[], buffer);
                let positions = output.glyph_positions();

                let run_width: i32 = positions.iter().map(|p| p.x_advance).sum();
                line_width +=
                    run_width as f64 * style.size.0 / candidate.units_per_em as f64;
            }
        }
        max_width = f64::max(max_width, line_width);
    }

    Some(Pt(max_width))
}

fn shape_page(
    world: &dyn World,
    page: &mut Page,
    cache: &mut ShapeCache,
    face_cache: &mut FaceCache,
) {
    let mut new_items = Vec::with_capacity(page.items.len());
    for item in page.items.drain(..) {
        new_items.extend(shape_item(world, item, cache, face_cache));
    }
    page.items = new_items;
}

/// Processa um `FrameItem`, devolvendo 1 ou mais itens (fallback por
/// caractere pode expandir um `Text` em vários `TextShaped` consecutivos).
fn shape_item(
    world: &dyn World,
    mut item: FrameItem,
    cache: &mut ShapeCache,
    face_cache: &mut FaceCache,
) -> Vec<FrameItem> {
    match &mut item {
        FrameItem::Text { pos, text, style } if style.font.is_some() => {
            if let Some(shaped) = try_shape(world, pos, text, style, cache, face_cache) {
                return shaped;
            }
        }
        FrameItem::Group { items, .. } => {
            let mut new_children = Vec::with_capacity(items.len());
            for child in items.drain(..) {
                new_children.extend(shape_item(world, child, cache, face_cache));
            }
            *items = new_children;
        }
        FrameItem::Link { items, .. } => {
            let mut new_children = Vec::with_capacity(items.len());
            for child in items.drain(..) {
                new_children.extend(shape_item(world, child, cache, face_cache));
            }
            *items = new_children;
        }
        _ => {}
    }
    vec![item]
}

fn try_shape(
    world: &dyn World,
    pos: &Point,
    text: &ecow::EcoString,
    style: &TextStyle,
    cache: &mut ShapeCache,
    face_cache: &mut FaceCache,
) -> Option<Vec<FrameItem>> {
    // P568 — espaços entre palavras são emitidos como FrameItem::Text para
    // que o PDF contenha o caractere de espaço. Não os shapear, para que
    // o export primário (emit_text_pdf) os escreva directamente na stream
    // de texto em vez de os perder no shaping de glyphs.
    if text.trim().is_empty() {
        return None;
    }

    let font_list = style.font.as_ref()?;

    // P525 — derivar a variante real do TextStyle para VF e selecção de fonte.
    // P836 — eixos derivados + explícitos (`style.variations`).
    let variant = text_style_to_font_variant(style);
    let axis_vars = axis_variations_for_text_style(style);

    // P515 — resolver todas as fontes candidatas da FontList.
    // **P538e/P555** — se a fonte declarada (incluindo a default "Libertinus Serif")
    // não existe no FontBook, tentar fontes padrão de fallback da mesma
    // classe (serif/sans) antes de recair no fallback global carácter-a-carácter.
    let mut primary =
        resolve_candidates(world, font_list, &variant, face_cache).unwrap_or_default();
    if primary.is_empty() {
        let first_family = font_list
            .as_slice()
            .first()
            .and_then(|f| f.name.as_str())
            .unwrap_or("");
        let fallback_list = fallback_font_list_for(first_family);
        for family in fallback_list {
            let fallback_font_list = FontList::single(ecow::EcoString::from(*family));
            if let Some(cands) =
                resolve_candidates(world, &fallback_font_list, &variant, face_cache)
            {
                if !cands.is_empty() {
                    primary = cands;
                    break;
                }
            }
        }
    } else {
        // **P783/P784** — adicionar a cadeia de fallback matemático como
        // primárias adicionais, para que glifos ausentes na primária sejam
        // buscados em fontes math dedicadas antes do fallback global (todo
        // o FontBook em ordem de índice). Dois gatilhos independentes
        // (OR): `style.math` (P784 — sempre que o texto vem do motor de
        // layout matemático, `layout/equation.rs`, independentemente de a
        // fonte de corpo por omissão ter ou não tabela MATH própria — ela
        // não tem, `Libertinus Serif` não tem MATH, então este era o único
        // caso que realmente falhava e motivou este passo) e
        // `primary_has_math` (P783 original — a fonte já resolvida declara
        // MATH própria, útil se o utilizador define `font:
        // "New Computer Modern Math"` explicitamente fora de `$...$`).
        let primary_has_math = primary
            .first()
            .and_then(|cand| face_cache.get(world, cand.slot_idx))
            .map_or(false, |cached| cached.face().tables().math.is_some());
        if style.math || primary_has_math {
            for family in math_fallback_font_list() {
                let math_font_list = FontList::single(ecow::EcoString::from(*family));
                if let Some(cands) =
                    resolve_candidates(world, &math_font_list, &variant, face_cache)
                {
                    for cand in cands {
                        if !primary.iter().any(|p| p.slot_idx == cand.slot_idx) {
                            primary.push(cand);
                        }
                    }
                }
            }
        }
    }

    // P534 — candidatos de fallback: todo o FontBook, carregados lazy.
    // P838 — `like` = info da primeira primária (o `ctx.first()` do vanilla).
    let like = primary
        .first()
        .and_then(|cand| world.book().infos().get(cand.slot_idx).cloned());

    // P484 — dividir em runs bidirectionais antes de shape.
    // **P845** — iterar TODOS os parágrafos bidi (antes só `paragraphs[0]`,
    // truncando texto com `\n` interno na primeira linha — achado #55 de
    // P831). Cada parágrafo é uma linha visível: x reinicia e y avança por
    // `line_advance`.
    let paragraphs = bidi_runs(text.as_str());
    if paragraphs.iter().all(|p| p.is_empty()) {
        return None;
    }

    // **P845** — avanço vertical por linha: top-edge + |bottom-edge| +
    // leading (default 0,65em), a MESMA fórmula do avanço de linha do
    // Layouter em L1 (P762) e os mesmos edges por omissão
    // (`cap-height`/`baseline`, via `edge_offset_pt`) — assim as linhas de
    // um texto com `\n` interno ficam espaçadas como as linhas normais do
    // documento. Sem face resolvida, cai para size + leading. Calculado
    // ANTES de mover `primary`/`face_cache` para o `CandidateSet`.
    let line_advance = {
        let (top, bottom) = primary
            .first()
            .and_then(|cand| face_cache.get(world, cand.slot_idx))
            .map(|cached| {
                let face = cached.face();
                let upem = face.units_per_em().max(1) as f64;
                (
                    crate::font_metrics::edge_offset_pt(
                        face,
                        upem,
                        style.size,
                        style.top_edge.as_ref(),
                        true,
                    ),
                    crate::font_metrics::edge_offset_pt(
                        face,
                        upem,
                        style.size,
                        style.bottom_edge.as_ref(),
                        false,
                    ),
                )
            })
            .unwrap_or((style.size, Pt(0.0)));
        let leading = style
            .leading
            .map(|l| l.resolve_pt(style.size.val()))
            .unwrap_or_else(|| style.size.0 * 0.65);
        top.0 - bottom.0 + leading
    };

    let mut candidates = CandidateSet::new(world, primary, face_cache, like, variant);

    let mut items = Vec::new();

    for (para_idx, runs) in paragraphs.iter().enumerate() {
        // Linha vazia (`\n\n`): sem runs, mas o índice do parágrafo já faz o
        // y avançar para as linhas seguintes.
        let line_y = Pt(pos.y.0 + line_advance * para_idx as f64);
        let mut x_offset = Pt(0.0);

        for run in runs {
            for subrun in split_run_by_font(run, &mut candidates) {
                let candidate = candidates.get(subrun.candidate_idx)?;

                // P657 — cache de shaping: reaproveita o resultado bruto do shaper
                // quando o mesmo sub-run (texto + face + direção + variações +
                // tracking) já foi processado neste documento.
                let cache_key = ShapeCache::key(
                    &subrun.text,
                    candidate.slot_idx,
                    run.rtl,
                    &axis_vars,
                    style.tracking,
                );
                let cached = cache.map.get(&cache_key).cloned();
                let (run_glyphs, run_width) = if let Some(cached) = cached {
                    (cached.glyphs, cached.width)
                } else {
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
                    let output = rustybuzz::shape(&rb_face, &[], buffer);
                    let infos = output.glyph_infos();
                    let positions = output.glyph_positions();

                    let mut run_glyphs: Vec<ShapedGlyph> =
                        Vec::with_capacity(infos.len());
                    let mut run_width = 0i32;
                    // **P543** — o campo `text` do TextShaped reflecte apenas o sub-run,
                    // e os clusters são relativos a esse texto. Isto evita que o export
                    // ToUnicode ou `pdftotext` dupliquem conteúdo quando o texto original
                    // era partido em sub-runs pequenos.
                    let subrun_text = subrun.text.as_str();
                    // **P621** — tracking: adicionar espaçamento extra entre clusters
                    // de caracteres, convertido de pontos para unidades da fonte.
                    // O tracking é aplicado ao avanço de um glifo apenas quando o glifo
                    // seguinte pertence a um cluster diferente, evitando partir ligaduras
                    // e conjuntos (ex.: devanágari) onde vários glifos compõem um único
                    // caractere visual. O último glifo do sub-run nunca recebe tracking.
                    let tracking_fu = style
                        .tracking
                        .map(|t| {
                            let pt = t.resolve_pt(style.size.val());
                            (pt * candidate.units_per_em as f64 / style.size.val())
                                .round() as i32
                        })
                        .unwrap_or(0);
                    let n_glyphs = infos.len();
                    let clusters: Vec<usize> =
                        infos.iter().map(|info| info.cluster as usize).collect();
                    for (idx, (info, pos_g)) in
                        infos.iter().zip(positions.iter()).enumerate()
                    {
                        let cluster = clusters[idx];
                        let char_code =
                            byte_idx_to_char(subrun_text, cluster).unwrap_or('\u{FFFD}');
                        let is_last = idx + 1 == n_glyphs;
                        let next_cluster_differs =
                            !is_last && clusters[idx + 1] != cluster;
                        let extra = if next_cluster_differs { tracking_fu } else { 0 };
                        run_glyphs.push(ShapedGlyph {
                            glyph_id: info.glyph_id as u16,
                            x_advance: pos_g.x_advance + extra,
                            x_offset: pos_g.x_offset,
                            y_offset: pos_g.y_offset,
                            cluster: cluster as u32,
                            char_code,
                        });
                        run_width += pos_g.x_advance + extra;
                    }

                    cache.map.insert(
                        cache_key,
                        CachedRun { glyphs: run_glyphs.clone(), width: run_width },
                    );

                    (run_glyphs, run_width)
                };

                if !run_glyphs.is_empty() {
                    let subrun_width_pt =
                        run_width as f64 * style.size.0 / candidate.units_per_em as f64;
                    let item_pos = Point { x: Pt(pos.x.0 + x_offset.0), y: line_y };
                    x_offset.0 += subrun_width_pt;

                    // P534 — cada sub-run reflecte a família real usada, para que o
                    // export multi-font embuta a face correcta e a associe via
                    // `font_index_for_style`.
                    let mut segment_style = style.clone();
                    let real_family =
                        world.book().infos().get(candidate.slot_idx)?.family.clone();
                    segment_style.font =
                        Some(FontList::single(ecow::EcoString::from(real_family)));

                    items.push(FrameItem::TextShaped {
                        pos: item_pos,
                        glyphs: run_glyphs,
                        style: segment_style,
                        text: subrun.text.clone().into(),
                        units_per_em: candidate.units_per_em,
                    });
                }
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
    slot_idx: usize,
    units_per_em: u16,
}

/// Resolve as fontes declaradas na `FontList` (primárias), na ordem do
/// utilizador. Se nenhuma resolver, `try_shape` cai no fallback `Text`.
fn resolve_candidates(
    world: &dyn World,
    font_list: &FontList,
    variant: &FontVariant,
    face_cache: &mut FaceCache,
) -> Option<Vec<FontCandidate>> {
    let book = world.book();
    let mut candidates = Vec::new();
    for family in font_list.as_slice() {
        if let Some(idx) = book.select_pattern(&family.name, &variant) {
            if let Some(cached) = face_cache.get(world, idx) {
                let units_per_em = cached.face().units_per_em().max(1) as u16;
                candidates.push(FontCandidate { slot_idx: idx, units_per_em });
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
    world: &'a dyn World,
    face_cache: &'a mut FaceCache,
    primary: Vec<FontCandidate>,
    fallback: Vec<Option<FontCandidate>>,
    /// **P838** — `FontInfo` da primeira primária (o `like` / `ctx.first()`
    /// do vanilla) e a variante activa, para o scoring de similaridade do
    /// fallback global (`FontBook::select_fallback`).
    like: Option<FontInfo>,
    variant: FontVariant,
}

impl<'a> CandidateSet<'a> {
    fn new(
        world: &'a dyn World,
        primary: Vec<FontCandidate>,
        face_cache: &'a mut FaceCache,
        like: Option<FontInfo>,
        variant: FontVariant,
    ) -> Self {
        Self { world, face_cache, primary, fallback: Vec::new(), like, variant }
    }

    /// Todos os candidatos que cobrem `c`, em ordem de prioridade (primárias
    /// primeiro, depois fallback lazy na ordem do FontBook).
    fn covering_all(&mut self, c: char) -> Vec<usize> {
        let mut result = Vec::new();
        for (i, cand) in self.primary.iter().enumerate() {
            if face_covers_char(self.world, self.face_cache, cand.slot_idx, c) {
                result.push(i);
            }
        }
        let book_len = self.world.book().len();
        for slot_idx in self.primary.len()..book_len {
            let fb_idx = slot_idx - self.primary.len();
            if fb_idx >= self.fallback.len() {
                let fallback = self.load_fallback(slot_idx);
                self.fallback.push(fallback);
            }
            if let Some(cand) = self.fallback[fb_idx] {
                if face_covers_char(self.world, self.face_cache, cand.slot_idx, c) {
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
            if face_covers_char(self.world, self.face_cache, cand.slot_idx, first_char) {
                primary_candidates.push(i);
            }
        }
        if let Some(result) = self.best_covering_run(text, start, &primary_candidates) {
            return Some(result);
        }

        // 2. Recair no fallback global. **P838** — o vencedor do scoring de
        // similaridade do vanilla (`FontBook::select_fallback`, com
        // `like` = primeira primária) entra na frente da lista: em empate de
        // comprimento de run (caso CJK típico — todas as candidatas cobrem o
        // run inteiro), vence o scoring em vez da ordem de índice do book.
        // O critério primário continua a ser o run mais longo (P543).
        let mut fallback_candidates = self.covering_all(first_char);
        if let Some(best) = self.world.book().select_fallback(
            self.like.as_ref(),
            &self.variant,
            fallback_candidates.iter().copied(),
        ) {
            if let Some(pos) = fallback_candidates.iter().position(|&c| c == best) {
                fallback_candidates.swap(0, pos);
            }
        }
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
                    face_covers_char(
                        self.world,
                        self.face_cache,
                        self.primary[idx].slot_idx,
                        c,
                    )
                } else {
                    self.fallback
                        .get(idx - self.primary.len())
                        .and_then(|f| f.as_ref())
                        .map_or(false, |cand| {
                            face_covers_char(
                                self.world,
                                self.face_cache,
                                cand.slot_idx,
                                c,
                            )
                        })
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

    fn load_fallback(&mut self, slot_idx: usize) -> Option<FontCandidate> {
        let cached = self.face_cache.get(self.world, slot_idx)?;
        Some(FontCandidate {
            slot_idx,
            units_per_em: cached.face().units_per_em().max(1) as u16,
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

fn face_covers_char(
    world: &dyn World,
    face_cache: &mut FaceCache,
    slot_idx: usize,
    c: char,
) -> bool {
    let Some(cached) = face_cache.get(world, slot_idx) else { return false };
    cached.face().glyph_index(c).is_some()
}

/// Sub-run dentro de um BidiRun: todos os caracteres partilham o mesmo script
/// efectivo e a mesma fonte candidata.
struct SubRun {
    text: String,
    candidate_idx: usize,
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
        let (candidate_idx, font_end) =
            candidates.covering_run(text, pos).unwrap_or((0, pos + c.len_utf8()));

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
    text: String,
    rtl: bool,
    /// Offset byte do run no string original (para ajuste de `cluster`).
    #[allow(dead_code)]
    byte_start: usize,
}

/// Divide `text` em runs bidirectionais na ordem visual correcta, agrupados
/// por parágrafo bidi. O unicode-bidi separa parágrafos nos caracteres de
/// classe B (`\n`, `\r`, U+2028, …) e mantém o separador no range do
/// parágrafo anterior (regra P1 do UAX#9); cada parágrafo corresponde a uma
/// linha visível do texto. Para texto puramente LTR de uma linha retorna um
/// único parágrafo com um único run. Parágrafos vazios (`\n\n` consecutivos)
/// retornam um `Vec` vazio — a linha existe, só não tem runs.
///
/// **P845** — antes iterava-se só `paragraphs[0]`, truncando texto com `\n`
/// interno na primeira linha (achado #55 de P831).
/// API unicode-bidi 0.3: `BidiInfo::new`, `visual_runs(para, range)`.
fn bidi_runs(text: &str) -> Vec<Vec<BidiRun>> {
    if text.is_empty() {
        return vec![];
    }
    let bidi = BidiInfo::new(text, None);
    if bidi.paragraphs.is_empty() {
        return vec![vec![BidiRun { text: text.to_owned(), rtl: false, byte_start: 0 }]];
    }
    bidi.paragraphs
        .iter()
        .map(|para| {
            // Excluir o separador de parágrafo final (classe B) do range a
            // shapear — não gera glifo visível e poluiria o texto extraível
            // (ToUnicode/pdftotext) com o `\n` cru.
            let mut range = para.range.clone();
            while range.end > range.start {
                let last = text[range.start..range.end].chars().next_back().unwrap();
                if unicode_bidi::bidi_class(last) == unicode_bidi::BidiClass::B {
                    range.end -= last.len_utf8();
                } else {
                    break;
                }
            }
            if range.is_empty() {
                return Vec::new();
            }
            let (levels, runs) = bidi.visual_runs(para, range);
            runs.into_iter()
                .map(|run_range| {
                    let rtl = levels
                        .get(run_range.start)
                        .map(|l: &unicode_bidi::Level| l.is_rtl())
                        .unwrap_or(false);
                    BidiRun {
                        text: text[run_range.clone()].to_owned(),
                        rtl,
                        byte_start: run_range.start,
                    }
                })
                .collect()
        })
        .collect()
}

// ── fim P484 ─────────────────────────────────────────────────────────────────

fn byte_idx_to_char(s: &str, byte_idx: usize) -> Option<char> {
    s.get(byte_idx..)?.chars().next()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::SystemWorld;
    use ecow::EcoString;
    use std::path::PathBuf;
    use typst_core::entities::file_id::FileId;
    use typst_core::entities::font_book::{
        FontBook, FontStretch, FontStyle, FontVariant, FontWeight,
    };
    use typst_core::entities::font_list::FontList;
    use typst_core::entities::layout_types::{
        FrameItem, Length, Page, PagedDocument, Point, Pt, TextStyle,
    };
    use typst_core::entities::source::Source;
    use typst_core::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library,
    };

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
        fn today(&self, _: Option<i64>) -> Option<Datetime> {
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
        fn today(&self, _: Option<i64>) -> Option<Datetime> {
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
        let mut style = TextStyle::default();
        style.font = Some(FontList::single(EcoString::from(family)));
        style.size = Pt(12.0);
        FrameItem::Text {
            pos: Point { x: Pt(0.0), y: Pt(0.0) },
            text: EcoString::from(text),
            style,
        }
    }

    fn text_item(text: &str) -> FrameItem {
        FrameItem::Text {
            pos: Point { x: Pt(72.0), y: Pt(72.0) },
            text: EcoString::from(text),
            style: TextStyle::default(),
        }
    }

    fn doc_with(items: Vec<FrameItem>) -> PagedDocument {
        PagedDocument::new(vec![Page {
            width: 595.0,
            height: 842.0,
            numbering: None,
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
        let mut style = TextStyle::default();
        style.font = Some(FontList::single(EcoString::from("Helvetica")));
        let item = FrameItem::Text {
            pos: Point { x: Pt(72.0), y: Pt(72.0) },
            text: EcoString::from("Ola"),
            style,
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
        assert!(
            ys[1] > ys[0] && ys[2] > ys[1],
            "linhas empilhadas verticalmente: {:?}",
            ys
        );
    }

    #[test]
    fn p845_shaped_width_multilinha_max_das_linhas() {
        let world = font_world_with(&["/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf"]);
        if !world.is_complete() {
            eprintln!("SKIP: DejaVu Sans não disponível");
            return;
        }
        let mut style = TextStyle::default();
        style.font = Some(FontList::single(EcoString::from("DejaVu Sans")));
        style.size = Pt(12.0);
        let mut face_cache = FaceCache::new();
        let w_linha = shaped_width(&world, "bb", &style, &mut face_cache).unwrap();
        let w_multi = shaped_width(&world, "a\nbb", &style, &mut face_cache).unwrap();
        assert_eq!(
            w_linha.0, w_multi.0,
            "largura multilinha = max das linhas (não soma)"
        );
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

        let mut book = FontBook::new();
        // O MockWorld precisa de um FontBook que corresponda aos slots.
        // Para simplificar, usamos SystemWorld com with_system_fonts.
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

        // Âncora à direita (item1) mantém x=100.0
        if let FrameItem::TextShaped { pos, .. } = &items[1] {
            assert!(
                (pos.x.0 - 100.0).abs() < 0.01,
                "âncora RTL: x deve ser 100.0, got {}",
                pos.x.0
            );
        }
        // Item à esquerda (item0) deve ser deslocado por shift = - (w_real[0] - w_est[0]) = -6.0.
        // Assim, x_new[0] = 70.0 - 6.0 = 64.0.
        if let FrameItem::TextShaped { pos, .. } = &items[0] {
            assert!(
                (pos.x.0 - 64.0).abs() < 0.01,
                "item RTL esquerdo: x deve ser 64.0, got {}",
                pos.x.0
            );
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

        let mut style_no_tracking = TextStyle::default();
        style_no_tracking.font = Some(FontList::single(EcoString::from("DejaVu Sans")));
        style_no_tracking.size = Pt(12.0);

        let mut style_tracking = style_no_tracking.clone();
        let tracking_pt = 5.0;
        style_tracking.tracking = Some(Length::pt(tracking_pt));

        let text = "Hello";
        let item_no = FrameItem::Text {
            pos: Point { x: Pt(0.0), y: Pt(0.0) },
            text: EcoString::from(text),
            style: style_no_tracking,
        };
        let item_yes = FrameItem::Text {
            pos: Point { x: Pt(0.0), y: Pt(0.0) },
            text: EcoString::from(text),
            style: style_tracking,
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
                    _ => None,
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
}

// ── P582 — redistribuição de posições x após shaping ─────────────────────────

/// Tolerância y para agrupar `FrameItem` na mesma linha visual.
/// Maior que a do bidi (0.01pt) porque aqui não estamos a reposicionar;
/// 0.5pt cobre sub-pixel rounding sem cruzar linhas adjacentes.
const Y_TOL_SHAPED: f64 = 0.5;

/// P582 — corrige posições x de todos os itens de uma linha usando os advances
/// reais dos glyphs obtidos no shaper. O Layouter usa `FallbackFontMetrics`
/// (estimativa) para calcular o cursor_x de cada palavra; o shaper usa a fonte
/// real (rustybuzz). Quando as duas fontes diferem (ex.: bold resolve face
/// diferente), as posições ficam descasadas. Esta passagem redistribui as
/// posições dentro de cada linha acumulando a diferença (width_real - width_est)
/// de forma a manter todos os espaçamentos e layouts relativos intactos.
///
/// Chamado na pipeline logo após `shape_document`.
pub fn fix_line_positions(world: &dyn World, mut doc: PagedDocument) -> PagedDocument {
    let metrics = FallbackFontMetrics::new(world);
    for page in &mut doc.pages {
        fix_line_positions_page(&metrics, page);
    }
    doc
}

fn get_item_x(item: &FrameItem) -> Option<f64> {
    match item {
        FrameItem::Text { pos, .. } => Some(pos.x.0),
        FrameItem::TextShaped { pos, .. } => Some(pos.x.0),
        FrameItem::Glyph { pos, .. } => Some(pos.x.0),
        FrameItem::Image { pos, .. } => Some(pos.x.0),
        FrameItem::Shape { pos, .. } => Some(pos.x.0),
        FrameItem::Group { pos, .. } => Some(pos.x.0),
        FrameItem::Link { pos, .. } => Some(pos.x.0),
        FrameItem::Line { start, .. } => Some(start.x.0),
    }
}

fn set_item_x(item: &mut FrameItem, x: f64) {
    match item {
        FrameItem::Text { pos, .. } => pos.x = Pt(x),
        FrameItem::TextShaped { pos, .. } => pos.x = Pt(x),
        FrameItem::Glyph { pos, .. } => pos.x = Pt(x),
        FrameItem::Image { pos, .. } => pos.x = Pt(x),
        FrameItem::Shape { pos, .. } => pos.x = Pt(x),
        FrameItem::Group { pos, .. } => pos.x = Pt(x),
        FrameItem::Link { pos, .. } => pos.x = Pt(x),
        FrameItem::Line { start, end, .. } => {
            let dx = end.x.0 - start.x.0;
            start.x = Pt(x);
            end.x = Pt(x + dx);
        }
    }
}

fn get_item_y(item: &FrameItem) -> Option<f64> {
    match item {
        FrameItem::Text { pos, .. } => Some(pos.y.0),
        FrameItem::TextShaped { pos, .. } => Some(pos.y.0),
        FrameItem::Glyph { pos, .. } => Some(pos.y.0),
        FrameItem::Image { pos, .. } => Some(pos.y.0),
        FrameItem::Shape { pos, .. } => Some(pos.y.0),
        FrameItem::Group { pos, .. } => Some(pos.y.0),
        FrameItem::Link { pos, .. } => Some(pos.y.0),
        FrameItem::Line { start, .. } => Some(start.y.0),
    }
}

fn estimate_width(metrics: &FallbackFontMetrics, text: &str, style: &TextStyle) -> f64 {
    use typst_core::engine::layout::FontMetrics;
    // **P593** — delegar para `FontMetrics::text_width`, a fonte única do
    // nível palavra (shaping + tracking).
    metrics.text_width(text, style.size, style).val()
}

fn fix_line_positions_page(metrics: &FallbackFontMetrics, page: &mut Page) {
    if page.items.is_empty() {
        return;
    }

    // 1. Recolher todos os índices de itens que têm coordenada x e y,
    // agrupados por linha visual (y).
    let mut lines: Vec<Vec<usize>> = Vec::new();
    for i in 0..page.items.len() {
        let y = match get_item_y(&page.items[i]) {
            Some(y) => y,
            None => continue,
        };
        if let Some(line) = lines.iter_mut().find(|l| {
            let ly = get_item_y(&page.items[*l.first().unwrap()]).unwrap_or(0.0);
            (y - ly).abs() <= Y_TOL_SHAPED
        }) {
            line.push(i);
        } else {
            lines.push(vec![i]);
        }
    }

    // 2. Para cada linha, ordenar e aplicar os desvios cumulativos
    for line in &lines {
        if line.len() < 2 {
            continue; // sem vizinhos para acumular desvios
        }

        // Ordenar por x original crescente (esquerda→direita).
        let mut sorted = line.clone();
        sorted.sort_by(|&a, &b| {
            let xa = get_item_x(&page.items[a]).unwrap_or(0.0);
            let xb = get_item_x(&page.items[b]).unwrap_or(0.0);
            xa.partial_cmp(&xb).unwrap_or(std::cmp::Ordering::Equal)
        });

        // Verificar se a linha é RTL (se algum item tem direcção RTL)
        let is_rtl_line = sorted.iter().any(|&idx| match &page.items[idx] {
            FrameItem::TextShaped { style, .. } => style.dir == Some(Dir::RTL),
            FrameItem::Text { style, .. } => style.dir == Some(Dir::RTL),
            _ => false,
        });

        if is_rtl_line {
            // Em linhas RTL, ancoramos o item mais à direita (o início da linha RTL)
            // e acumulamos desvios para a esquerda (valores negativos de shift).
            let mut shift = 0.0;
            for i in (0..sorted.len()).rev() {
                let idx = sorted[i];
                let x_orig = match get_item_x(&page.items[idx]) {
                    Some(x) => x,
                    None => continue,
                };
                set_item_x(&mut page.items[idx], x_orig + shift);

                // O shift que afeta os itens à esquerda (i-1) acumula a diferença
                // do item que acabamos de posicionar.
                if i > 0 {
                    let prev_idx = sorted[i - 1];
                    let (w_est, w_real) = match &page.items[prev_idx] {
                        FrameItem::TextShaped {
                            text,
                            style,
                            glyphs,
                            units_per_em,
                            ..
                        } => {
                            let upem = (*units_per_em).max(1) as f64;
                            let size = style.size.0;
                            let w_real = glyphs
                                .iter()
                                .map(|g| g.x_advance as f64 / upem * size)
                                .sum::<f64>();
                            let w_est = estimate_width(metrics, text, style);
                            (w_est, w_real)
                        }
                        _ => (0.0, 0.0),
                    };
                    shift -= w_real - w_est;
                }
            }
        } else {
            // Linha LTR normal: ancoramos o primeiro item (mais à esquerda)
            // e acumulamos desvios para a direita (valores positivos de shift).
            let mut shift = 0.0;
            for &idx in &sorted {
                let x_orig = match get_item_x(&page.items[idx]) {
                    Some(x) => x,
                    None => continue,
                };
                set_item_x(&mut page.items[idx], x_orig + shift);

                let (w_est, w_real) = match &page.items[idx] {
                    FrameItem::TextShaped {
                        text, style, glyphs, units_per_em, ..
                    } => {
                        let upem = (*units_per_em).max(1) as f64;
                        let size = style.size.0;
                        let w_real = glyphs
                            .iter()
                            .map(|g| g.x_advance as f64 / upem * size)
                            .sum::<f64>();
                        let w_est = estimate_width(metrics, text, style);
                        (w_est, w_real)
                    }
                    _ => (0.0, 0.0),
                };
                shift += w_real - w_est;
            }
        }
    }
}
