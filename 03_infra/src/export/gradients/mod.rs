//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/gradients/mod.md
//! @prompt-hash e4d072c4
//! @layer L3
//! @updated 2026-05-19
//!
//! Gradient cluster — Linear (P263) + Radial + CMYK (P270.2) +
//! relative (P273) + Adaptive N (P274) + Conic Coons (P272).
//!
//! Extraído de `export.rs` em P307b.1 (ADR-0100 / diagnóstico
//! P307a §5). Sub-divisão interna em `gradients/{linear,radial,
//! cmyk,relative,adaptive,conic,function_dict}.rs` é objectivo
//! de P307b.2 (refino opcional).
//!
//! Conteúdo bit-exact pré e pós migração.

use std::collections::{BTreeSet, HashMap};
use std::sync::Arc;

use typst_core::entities::layout_types::{FrameItem, Page, PagedDocument};

// ── Submódulos extraídos em P307b.2 (ADR-0100 / diagnóstico P307a §5) ──────

mod adaptive;
mod cmyk;
mod conic;
mod function_dict;
mod linear;
mod radial;
mod relative;

pub(super) use self::adaptive::{adaptive_n_for_stops, perceptual_distance_in_space};
pub(super) use self::cmyk::{
    multispace_sample_stops_linear_cmyk, multispace_sample_stops_radial_cmyk, rgb_to_cmyk,
};
pub(super) use self::conic::{
    bezier_control_points_for_arc, compute_coons_patches_n_stops,
    compute_coons_patches_n_stops_extended, emit_conic_coons_stream_cmyk,
    emit_conic_coons_stream_rgb, multispace_sample_stops_conic,
};
pub(super) use self::function_dict::{emit_function_dict, emit_function_dict_cmyk};
pub(super) use self::linear::{compute_axial_coords, multispace_sample_stops};
pub(super) use self::radial::{compute_radial_coords, multispace_sample_stops_radial};
pub(super) use self::relative::{apply_parent_transform, resolve_relative};

// ── P263: Gradient Linear → PDF Shading Patterns (ADR-0087) ────────────────

/// Metadados de gradient para resource dict e page streams.
pub(crate) struct PatternRef {
    pub(super) pattern_obj_id: usize,
    pub(super) name: String,
}

/// P265 + P268 — variant para distinguir Linear / Radial / Conic em emit.
pub(super) enum GradientObjectKind {
    Linear(std::sync::Arc<typst_core::entities::gradient::Linear>),
    Radial(std::sync::Arc<typst_core::entities::gradient::Radial>),
    Conic(std::sync::Arc<typst_core::entities::gradient::Conic>),
}

/// Dados internos para emit Function/Shading/Pattern object dicts.
pub(super) struct GradientObject {
    pub(super) kind: GradientObjectKind,
    pub(super) function_id: usize,
    pub(super) shading_id: usize,
    pub(super) pattern_id: usize,
    /// **P273.6** — bbox do contentor imediato capturado no momento do
    /// emit do FrameItem::Shape (3γ.2 materializada). `Some(rect)` quando
    /// shape estava dentro de Content::Block com dimensions literais;
    /// `None` quando top-level (cai no fallback page_bbox L3 P273.5).
    ///
    /// **Limitação dedup**: gradients são deduplicados por Arc pointer;
    /// quando o mesmo gradient é usado por shapes em contextos distintos
    /// (e.g., dentro e fora de Block), apenas a primeira occurrence
    /// captura o bbox. Refino futuro para dedup bbox-aware fica fora de
    /// escopo P273.6 per ADR-0054 graded.
    pub(super) parent_bbox_at_emit: Option<typst_core::entities::layout_types::Rect>,
}

/// **P273.12** — Quantização de `Rect` em milipontos para chave HashMap.
/// `f64` não impl `Hash`; quantização `(r * 1000.0).round() as i32` resolve
/// (NaN, precision creep). 1 mpt = 0.001 pt — precisão sub-typográfica.
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub(super) struct RectKey(i32, i32, i32, i32);

pub(super) fn rect_to_key(r: typst_core::entities::layout_types::Rect) -> RectKey {
    RectKey(
        (r.x.0 * 1000.0).round() as i32,
        (r.y.0 * 1000.0).round() as i32,
        (r.w.0 * 1000.0).round() as i32,
        (r.h.0 * 1000.0).round() as i32,
    )
}

/// **P273.12** — Chave de dedup bbox-aware (Decisão 1β + 1γ Fase A).
/// Substitui `usize` (Arc::as_ptr) pré-P273.12. Mesmo Arc + bboxes
/// effective distintos → DedupKeys distintos → PDF patterns distintos
/// (fecha limitação P273.6 §9 quarto bullet).
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) struct DedupKey {
    arc_ptr: usize,
    bbox: Option<RectKey>,
}

/// **P273.12** — Constrói `DedupKey` a partir de `Arc<g>` + bbox effective.
pub(super) fn dedup_key_for(
    g: &typst_core::entities::gradient::Gradient,
    effective_bbox: Option<typst_core::entities::layout_types::Rect>,
) -> DedupKey {
    use typst_core::entities::gradient::Gradient;
    let arc_ptr = match g {
        Gradient::Linear(l) => std::sync::Arc::as_ptr(l) as usize,
        Gradient::Radial(r) => std::sync::Arc::as_ptr(r) as usize,
        Gradient::Conic(c) => std::sync::Arc::as_ptr(c) as usize,
    };
    DedupKey { arc_ptr, bbox: effective_bbox.map(rect_to_key) }
}

/// **P278 sub-op 2 — Helper extraído**: constrói `Rect` cristalino
/// (Y-down; sem inversion) a partir de Group `pos`/`inner_width`/`inner_height`.
///
/// Consolida 6 sítios replicados: `scan_all_gradients.walk` +
/// `pattern_resources_for_page.walk` + `draw_item_local` arm Group +
/// 3 variants `build_page_stream_*` Group dispatch. Sub-padrão
/// **"Extract helper de replicação inline" N=3 cumulativo** atinge
/// limiar formalização N≥3-4 mas NÃO formalizado per anti-padrão
/// over-formalização P273.17.
pub(super) fn group_bbox_from_fields(
    pos: typst_core::entities::layout_types::Point,
    inner_width: f64,
    inner_height: f64,
) -> typst_core::entities::layout_types::Rect {
    typst_core::entities::layout_types::Rect {
        x: typst_core::entities::layout_types::Pt(pos.x.0),
        y: typst_core::entities::layout_types::Pt(pos.y.0),
        w: typst_core::entities::layout_types::Pt(inner_width),
        h: typst_core::entities::layout_types::Pt(inner_height),
    }
}

/// Varre o documento e pré-processa todos os gradients únicos por
/// `(Arc::as_ptr, parent_bbox_effective_quantizado)` (P273.12 — chave
/// bbox-aware substitui chave Arc-only P262-P273.11).
///
/// Aloca 3 ObjectIDs por gradient único: Function + Shading + Pattern.
///
/// Retorna `(refs, ptr_to_idx, grad_objs)`:
/// - `refs`: metadados name/obj_id por gradient (para resource dict).
/// - `ptr_to_idx`: `DedupKey → índice em refs`.
/// - `grad_objs`: dados para emit (mesma ordem que refs).
pub(super) fn scan_all_gradients(
    doc: &typst_core::entities::layout_types::PagedDocument,
    first_id: usize,
) -> (Vec<PatternRef>, HashMap<DedupKey, usize>, Vec<GradientObject>) {
    use typst_core::entities::geometry::Stroke;
    use typst_core::entities::gradient::Gradient;
    use typst_core::entities::layout_types::{FrameItem, Pt, Rect};
    use typst_core::entities::paint::Paint;

    // P273.10 — helper recursivo: itera items + tratamento FrameItem::Group
    // com `parent_bbox_override: Option<Rect>` (Decisão 1α parameter
    // threading). Inner-wins: Shape's próprio `parent_bbox_at_emit`
    // prevalece sobre `override` via `.or()`.
    fn walk(
        items: &[FrameItem],
        parent_bbox_override: Option<Rect>,
        ptr_to_idx: &mut HashMap<DedupKey, usize>,
        refs: &mut Vec<PatternRef>,
        grad_objs: &mut Vec<GradientObject>,
        next_id: &mut usize,
        counter: &mut usize,
    ) {
        for item in items {
            match item {
                FrameItem::Shape {
                    stroke: Some(Stroke { paint: Paint::Gradient(g), .. }),
                    parent_bbox_at_emit,
                    ..
                } => {
                    // P273.10 — Inner wins: Shape's próprio campo prevalece.
                    let effective_bbox = parent_bbox_at_emit.or(parent_bbox_override);
                    // P273.12 — DedupKey bbox-aware (substitui chave Arc-only).
                    let key = dedup_key_for(g, effective_bbox);
                    if ptr_to_idx.contains_key(&key) {
                        continue;
                    }
                    let kind = match g {
                        Gradient::Linear(l) => {
                            GradientObjectKind::Linear(std::sync::Arc::clone(l))
                        }
                        Gradient::Radial(r) => {
                            GradientObjectKind::Radial(std::sync::Arc::clone(r))
                        }
                        Gradient::Conic(c) => {
                            GradientObjectKind::Conic(std::sync::Arc::clone(c))
                        }
                    };
                    let function_id = *next_id;
                    *next_id += 1;
                    let shading_id = *next_id;
                    *next_id += 1;
                    let pattern_id = *next_id;
                    *next_id += 1;
                    let name = format!("P{}", *counter);
                    *counter += 1;
                    let idx = refs.len();
                    refs.push(PatternRef { pattern_obj_id: pattern_id, name });
                    grad_objs.push(GradientObject {
                        kind,
                        function_id,
                        shading_id,
                        pattern_id,
                        parent_bbox_at_emit: effective_bbox,
                    });
                    ptr_to_idx.insert(key, idx);
                }
                FrameItem::Group { pos, inner_width, inner_height, items, .. } => {
                    // P273.10 — Group bbox L3-only override (Decisão 2α):
                    // geometric exact em coords cristalino (sem Y-inversion).
                    // P278 — helper `group_bbox_from_fields` consolidação 6 sítios.
                    let group_bbox =
                        group_bbox_from_fields(*pos, *inner_width, *inner_height);
                    walk(
                        items,
                        Some(group_bbox),
                        ptr_to_idx,
                        refs,
                        grad_objs,
                        next_id,
                        counter,
                    );
                }
                // **P425-A7**: gradients dentro de Link também devem ser registados.
                FrameItem::Link { items, .. } => {
                    walk(
                        items,
                        parent_bbox_override,
                        ptr_to_idx,
                        refs,
                        grad_objs,
                        next_id,
                        counter,
                    );
                }
                FrameItem::Semantic { items, .. } => {
                    walk(
                        items,
                        parent_bbox_override,
                        ptr_to_idx,
                        refs,
                        grad_objs,
                        next_id,
                        counter,
                    );
                }
                // intencional: texto, linhas, glifos e imagens não contêm gradientes vetoriais de Shape
                FrameItem::Text { .. }
                | FrameItem::TextShaped { .. }
                | FrameItem::Line { .. }
                | FrameItem::Glyph { .. }
                | FrameItem::Image { .. }
                | FrameItem::Shape { .. } => {}
            }
        }
    }

    let mut ptr_to_idx: HashMap<DedupKey, usize> = HashMap::new();
    let mut refs: Vec<PatternRef> = Vec::new();
    let mut grad_objs: Vec<GradientObject> = Vec::new();
    let mut next_id = first_id;
    let mut counter = 1usize;

    for page in &doc.pages {
        // P273.10 — top-level: parent_bbox_override = None (gradient só
        // recebe override se descobrir Group em rota descendente).
        walk(
            &page.items,
            None,
            &mut ptr_to_idx,
            &mut refs,
            &mut grad_objs,
            &mut next_id,
            &mut counter,
        );
    }
    (refs, ptr_to_idx, grad_objs)
}

/// Constrói o fragmento `/Pattern << /P1 X 0 R ... >>` para os recursos
/// de página. Retorna string vazia se não houver gradients na página.
pub(super) fn pattern_resources_for_page(
    page: &Page,
    ptr_to_idx: &HashMap<DedupKey, usize>,
    refs: &[PatternRef],
) -> String {
    use typst_core::entities::geometry::Stroke;
    use typst_core::entities::layout_types::{FrameItem, Pt, Rect};
    use typst_core::entities::paint::Paint;

    // P273.10 — helper recursivo symmetric a scan_all_gradients.
    // P273.12 — DedupKey bbox-aware lookup; threading
    // `parent_bbox_override` paralelo ao scan walk.
    fn walk(
        items: &[FrameItem],
        parent_bbox_override: Option<Rect>,
        ptr_to_idx: &HashMap<DedupKey, usize>,
        refs: &[PatternRef],
        entries: &mut Vec<String>,
        seen: &mut BTreeSet<usize>,
    ) {
        for item in items {
            match item {
                FrameItem::Shape {
                    stroke: Some(Stroke { paint: Paint::Gradient(g), .. }),
                    parent_bbox_at_emit,
                    ..
                } => {
                    let effective_bbox = parent_bbox_at_emit.or(parent_bbox_override);
                    let key = dedup_key_for(g, effective_bbox);
                    if let Some(&idx) = ptr_to_idx.get(&key) {
                        if seen.insert(idx) {
                            let r = &refs[idx];
                            entries.push(format!("/{} {} 0 R", r.name, r.pattern_obj_id));
                        }
                    }
                }
                FrameItem::Group { pos, inner_width, inner_height, items, .. } => {
                    // P278 — helper `group_bbox_from_fields` consolidação 6 sítios.
                    let group_bbox =
                        group_bbox_from_fields(*pos, *inner_width, *inner_height);
                    walk(items, Some(group_bbox), ptr_to_idx, refs, entries, seen);
                }
                FrameItem::Semantic { items, .. } => {
                    walk(items, parent_bbox_override, ptr_to_idx, refs, entries, seen);
                }
                // intencional: texto, linhas, glifos e imagens não definem recursos /Pattern
                FrameItem::Text { .. }
                | FrameItem::TextShaped { .. }
                | FrameItem::Line { .. }
                | FrameItem::Glyph { .. }
                | FrameItem::Image { .. }
                | FrameItem::Link { .. }
                | FrameItem::Shape { .. } => {}
            }
        }
    }

    let mut entries: Vec<String> = Vec::new();
    let mut seen: BTreeSet<usize> = Default::default();
    walk(&page.items, None, ptr_to_idx, refs, &mut entries, &mut seen);
    if entries.is_empty() {
        return String::new();
    }
    format!("/Pattern << {} >>", entries.join(" "))
}
