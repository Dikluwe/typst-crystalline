//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/gradients/mod.md
//! @prompt-hash dd9d0afb
//! @layer L3
//! @updated 2026-09-28
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

use typst_core::entities::layout_types::{
    FrameItem, Page, Point, Pt, Rect, TransformMatrix,
};

// ── Submódulos extraídos em P307b.2 (ADR-0100 / diagnóstico P307a §5) ──────

mod adaptive;
mod cmyk;
mod conic;
mod function_dict;
mod linear;
mod radial;
mod relative;

#[cfg(test)]
pub(super) use self::adaptive::perceptual_distance_in_space;
pub(super) use self::adaptive::{adaptive_n_for_stops, svg_adaptive_stops};
#[cfg(test)]
pub(super) use self::cmyk::rgb_to_cmyk;
pub(super) use self::cmyk::{
    multispace_sample_stops_linear_cmyk, multispace_sample_stops_radial_cmyk,
};
pub(super) use self::conic::emit_conic_mesh;
use self::conic::ConicDomain;
#[cfg(test)]
pub(super) use self::conic::{
    bezier_control_points_for_arc, compute_coons_patches_n_stops,
    compute_coons_patches_n_stops_extended, multispace_sample_stops_conic,
};
#[cfg(test)]
pub(super) use self::conic::{emit_conic_coons_stream_cmyk, emit_conic_coons_stream_rgb};
pub(super) use self::function_dict::{
    emit_function_dict, emit_function_dict_cmyk, emit_function_dict_spot,
};
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
    pub(super) parent_bbox_at_emit: Option<typst_core::entities::layout_types::Rect>,
    /// Caixa do próprio shape para posicionar shadings spot.
    pub(super) paint_bbox_at_emit: Option<typst_core::entities::layout_types::Rect>,
    pub(super) page_height_at_emit: f64,
    pub(super) conic_matrix: Option<TransformMatrix>,
    pub(super) conic_domain: ConicDomain,
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
/// effective distintos → DedupKeys distintos → PDF patterns distintos. Conic
/// usa Arc e identidade da ocorrência, pois sua matriz depende de todo o percurso.
/// (fecha limitação P273.6 §9 quarto bullet).
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) struct DedupKey {
    arc_ptr: usize,
    bbox: Option<RectKey>,
    occurrence: Option<usize>,
}

/// Bbox legado de spot; Conic usa seu domínio e identidade de ocorrência.
pub(super) fn spot_bbox_for_shape(
    g: &typst_core::entities::gradient::Gradient,
    pos: typst_core::entities::layout_types::Point,
    width: f64,
    height: f64,
) -> Option<typst_core::entities::layout_types::Rect> {
    use typst_core::entities::gradient::Gradient;
    use typst_core::entities::layout_types::{ColorSpace, Pt, Rect};
    let space = match g {
        Gradient::Linear(inner) => &inner.space,
        Gradient::Radial(inner) => &inner.space,
        Gradient::Conic(inner) => &inner.space,
    };
    if !matches!(space, ColorSpace::Spot(_)) {
        return None;
    }
    Some(Rect { x: pos.x, y: pos.y, w: Pt(width), h: Pt(height) })
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
    DedupKey {
        arc_ptr,
        bbox: effective_bbox.map(rect_to_key),
        occurrence: None,
    }
}

/// Conic patterns are bound to a borrowed Shape occurrence for this export.
/// The document owns its child Vecs and remains immutable through all passes.
pub(super) fn dedup_key_for_shape(
    g: &typst_core::entities::gradient::Gradient,
    effective_bbox: Option<Rect>,
    item: &FrameItem,
) -> DedupKey {
    let mut key = dedup_key_for(g, effective_bbox);
    if matches!(g, typst_core::entities::gradient::Gradient::Conic(_)) {
        key.bbox = None;
        key.occurrence = Some(item as *const FrameItem as usize);
    }
    key
}

/// Local Y-down coordinates to the initial PDF stream space. This is separate
/// from the legacy linear/radial bbox override, which is not an affine carrier.
#[derive(Clone, Copy)]
struct ConicPlacement {
    to_pdf: TransformMatrix,
    parent: Rect,
    canvas: (f64, f64),
}

impl ConicPlacement {
    fn page(page: &Page) -> Self {
        Self {
            canvas: (page.canvas_width(), page.canvas_height()),
            to_pdf: TransformMatrix {
                a: 1.0,
                b: 0.0,
                c: 0.0,
                d: -1.0,
                tx: page.bleed.left,
                ty: page.height + page.bleed.bottom,
            },
            parent: Rect {
                x: Pt(0.0),
                y: Pt(0.0),
                w: Pt(page.width),
                h: Pt(page.height),
            },
        }
    }

    fn group(self, pos: Point, matrix: TransformMatrix, width: f64, height: f64) -> Self {
        let local = TransformMatrix::translate(pos.x.0, pos.y.0).concat(&matrix);
        Self {
            canvas: self.canvas,
            to_pdf: self.to_pdf.concat(&local),
            parent: Rect {
                x: Pt(0.0),
                y: Pt(0.0),
                w: Pt(width),
                h: Pt(height),
            },
        }
    }

    fn reference_box(
        self,
        conic: &typst_core::entities::gradient::Conic,
        own: Rect,
        parent: Option<Rect>,
    ) -> Rect {
        use typst_core::entities::gradient::RelativeTo;
        if resolve_relative(conic.relative) == RelativeTo::Parent {
            parent.unwrap_or(self.parent)
        } else {
            own
        }
    }

    fn matrix(
        self,
        conic: &typst_core::entities::gradient::Conic,
        own: Rect,
        parent: Option<Rect>,
    ) -> TransformMatrix {
        let rect = self.reference_box(conic, own, parent);
        self.to_pdf.concat(&TransformMatrix {
            a: rect.w.0,
            b: 0.0,
            c: 0.0,
            d: -rect.h.0,
            tx: rect.x.0,
            ty: rect.y.0 + rect.h.0,
        })
    }

    fn domain(
        self,
        conic: &typst_core::entities::gradient::Conic,
        own: Rect,
        parent: Option<Rect>,
    ) -> ConicDomain {
        let rect = self.reference_box(conic, own, parent);
        let m = self.matrix(conic, own, parent);
        let det = m.a * m.d - m.b * m.c;
        let mut bounds = [0.0_f64, 1.0, 0.0, 1.0];
        if det.is_finite() && det != 0.0 {
            for x in [0.0, self.canvas.0] {
                for y in [0.0, self.canvas.1] {
                    let dx = x - m.tx;
                    let dy = y - m.ty;
                    let px = (m.d * dx - m.c * dy) / det;
                    let py = (-m.b * dx + m.a * dy) / det;
                    if px.is_finite() && py.is_finite() {
                        bounds[0] = bounds[0].min(px);
                        bounds[1] = bounds[1].max(px);
                        bounds[2] = bounds[2].min(py);
                        bounds[3] = bounds[3].max(py);
                    }
                }
            }
        }
        ConicDomain { size: (rect.w.0, rect.h.0), bounds }
    }
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
/// Arc/bbox em linear/radial e Arc/ocorrência em conic. Captura a matriz e
/// o domínio cônico no mesmo referencial da pintura, incluindo grupos e bleed.
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
    use typst_core::entities::frame_paint::FramePaint as Paint;
    use typst_core::entities::gradient::Gradient;
    use typst_core::entities::layout_types::{FrameItem, Rect};

    // P273.10 — helper recursivo: itera items + tratamento FrameItem::Group
    // com `parent_bbox_override: Option<Rect>` (Decisão 1α parameter
    // threading). Inner-wins: Shape's próprio `parent_bbox_at_emit`
    // prevalece sobre `override` via `.or()`.
    struct Collector {
        ptr_to_idx: HashMap<DedupKey, usize>,
        refs: Vec<PatternRef>,
        grad_objs: Vec<GradientObject>,
        next_id: usize,
        counter: usize,
    }

    fn walk(
        items: &[FrameItem],
        parent_bbox_override: Option<Rect>,
        page_height: f64,
        placement: ConicPlacement,
        collector: &mut Collector,
    ) {
        for item in items {
            match item {
                FrameItem::Shape {
                    pos,
                    width,
                    height,
                    fill,
                    stroke,
                    parent_bbox_at_emit,
                    ..
                } => {
                    for paint in fill.iter().chain(stroke.iter().map(|s| &s.paint)) {
                        if let Paint::Tiling(cell) = paint {
                            let w = cell.frame.size.width.0;
                            let h = cell.frame.size.height.0;
                            let cell_placement = ConicPlacement {
                                to_pdf: TransformMatrix::identity(),
                                parent: Rect {
                                    x: Pt(0.0),
                                    y: Pt(0.0),
                                    w: Pt(w),
                                    h: Pt(h),
                                },
                                canvas: (w, h),
                            };
                            walk(&cell.frame.items, None, h, cell_placement, collector);
                        }
                    }
                    for g in fill
                        .iter()
                        .chain(stroke.iter().map(|s| &s.paint))
                        .filter_map(|p| {
                            if let Paint::Gradient(g) = p {
                                Some(g)
                            } else {
                                None
                            }
                        })
                    {
                        // P273.10 — Inner wins: Shape's próprio campo prevalece.
                        let effective_bbox = parent_bbox_at_emit.or(parent_bbox_override);
                        let paint_bbox = spot_bbox_for_shape(g, *pos, *width, *height);
                        // Spot precisa da posição do próprio shape na chave.
                        let key =
                            dedup_key_for_shape(g, paint_bbox.or(effective_bbox), item);
                        if collector.ptr_to_idx.contains_key(&key) {
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
                        let function_id = collector.next_id;
                        collector.next_id += 1;
                        let shading_id = collector.next_id;
                        collector.next_id += 1;
                        let pattern_id = collector.next_id;
                        collector.next_id += 1;
                        let name = format!("P{}", collector.counter);
                        collector.counter += 1;
                        let idx = collector.refs.len();
                        collector
                            .refs
                            .push(PatternRef { pattern_obj_id: pattern_id, name });
                        collector.grad_objs.push(GradientObject {
                            kind,
                            function_id,
                            shading_id,
                            pattern_id,
                            parent_bbox_at_emit: effective_bbox,
                            paint_bbox_at_emit: paint_bbox,
                            page_height_at_emit: page_height,
                            conic_domain: if let Gradient::Conic(conic) = g {
                                placement.domain(
                                    conic,
                                    Rect {
                                        x: pos.x,
                                        y: pos.y,
                                        w: Pt(*width),
                                        h: Pt(*height),
                                    },
                                    *parent_bbox_at_emit,
                                )
                            } else {
                                ConicDomain::default()
                            },
                            conic_matrix: if let Gradient::Conic(conic) = g {
                                Some(placement.matrix(
                                    conic,
                                    Rect {
                                        x: pos.x,
                                        y: pos.y,
                                        w: Pt(*width),
                                        h: Pt(*height),
                                    },
                                    *parent_bbox_at_emit,
                                ))
                            } else {
                                None
                            },
                        });
                        collector.ptr_to_idx.insert(key, idx);
                    }
                }
                FrameItem::Group {
                    pos,
                    matrix,
                    inner_width,
                    inner_height,
                    items,
                    ..
                } => {
                    // P273.10 — Group bbox L3-only override (Decisão 2α):
                    // geometric exact em coords cristalino (sem Y-inversion).
                    // P278 — helper `group_bbox_from_fields` consolidação 6 sítios.
                    let group_bbox =
                        group_bbox_from_fields(*pos, *inner_width, *inner_height);
                    walk(
                        items,
                        Some(group_bbox),
                        page_height,
                        placement.group(*pos, *matrix, *inner_width, *inner_height),
                        collector,
                    );
                }
                // **P425-A7**: gradients dentro de Link também devem ser registados.
                FrameItem::Link { items, .. } => {
                    walk(items, parent_bbox_override, page_height, placement, collector);
                }
                FrameItem::TextReplacement { items, .. }
                | FrameItem::Semantic { items, .. } => {
                    walk(items, parent_bbox_override, page_height, placement, collector);
                }
                // intencional: texto, linhas, glifos e imagens não contêm gradientes vetoriais de Shape
                FrameItem::TextSeparator { .. }
                | FrameItem::Text { .. }
                | FrameItem::TextShaped { .. }
                | FrameItem::Line { .. }
                | FrameItem::Glyph { .. }
                | FrameItem::Image { .. } => {}
            }
        }
    }

    let mut collector = Collector {
        ptr_to_idx: HashMap::new(),
        refs: Vec::new(),
        grad_objs: Vec::new(),
        next_id: first_id,
        counter: 1,
    };

    for page in &doc.pages {
        // P273.10 — top-level: parent_bbox_override = None (gradient só
        // recebe override se descobrir Group em rota descendente).
        for items in [&page.background, &page.items, &page.foreground] {
            walk(items, None, page.height, ConicPlacement::page(page), &mut collector);
        }
    }
    (collector.refs, collector.ptr_to_idx, collector.grad_objs)
}

/// Constrói o fragmento `/Pattern << /P1 X 0 R ... >>` para os recursos
/// de página. Retorna string vazia se não houver gradients na página.
pub(super) fn pattern_resources_for_page(
    page: &Page,
    ptr_to_idx: &HashMap<DedupKey, usize>,
    refs: &[PatternRef],
) -> String {
    use typst_core::entities::frame_paint::FramePaint as Paint;
    use typst_core::entities::layout_types::{FrameItem, Rect};

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
                    pos,
                    width,
                    height,
                    fill,
                    stroke,
                    parent_bbox_at_emit,
                    ..
                } => {
                    for paint in fill.iter().chain(stroke.iter().map(|s| &s.paint)) {
                        if let Paint::Tiling(cell) = paint {
                            walk(
                                &cell.frame.items,
                                None,
                                ptr_to_idx,
                                refs,
                                entries,
                                seen,
                            );
                        }
                    }
                    for g in fill
                        .iter()
                        .chain(stroke.iter().map(|s| &s.paint))
                        .filter_map(|p| {
                            if let Paint::Gradient(g) = p {
                                Some(g)
                            } else {
                                None
                            }
                        })
                    {
                        let effective_bbox = parent_bbox_at_emit.or(parent_bbox_override);
                        let paint_bbox = spot_bbox_for_shape(g, *pos, *width, *height);
                        let key =
                            dedup_key_for_shape(g, paint_bbox.or(effective_bbox), item);
                        if let Some(&idx) = ptr_to_idx.get(&key) {
                            if seen.insert(idx) {
                                let r = &refs[idx];
                                entries.push(format!(
                                    "/{} {} 0 R",
                                    r.name, r.pattern_obj_id
                                ));
                            }
                        }
                    }
                }
                FrameItem::Group { pos, inner_width, inner_height, items, .. } => {
                    // P278 — helper `group_bbox_from_fields` consolidação 6 sítios.
                    let group_bbox =
                        group_bbox_from_fields(*pos, *inner_width, *inner_height);
                    walk(items, Some(group_bbox), ptr_to_idx, refs, entries, seen);
                }
                FrameItem::TextReplacement { items, .. }
                | FrameItem::Semantic { items, .. }
                | FrameItem::Link { items, .. } => {
                    walk(items, parent_bbox_override, ptr_to_idx, refs, entries, seen);
                }
                // intencional: texto, linhas, glifos e imagens não definem recursos /Pattern
                FrameItem::TextSeparator { .. }
                | FrameItem::Text { .. }
                | FrameItem::TextShaped { .. }
                | FrameItem::Line { .. }
                | FrameItem::Glyph { .. }
                | FrameItem::Image { .. } => {}
            }
        }
    }

    let mut entries: Vec<String> = Vec::new();
    let mut seen: BTreeSet<usize> = Default::default();
    for items in [&page.background, &page.items, &page.foreground] {
        walk(items, None, ptr_to_idx, refs, &mut entries, &mut seen);
    }
    if entries.is_empty() {
        return String::new();
    }
    format!("/Pattern << {} >>", entries.join(" "))
}

#[cfg(test)]
mod placement_tests {
    use super::*;
    use crate::export::{export_pdf, StreamMode};
    use typst_core::entities::axes::Axes;
    use typst_core::entities::content::Content;
    use typst_core::entities::frame_paint::FramePaint as Paint;
    use typst_core::entities::frame_paint::FrameStroke as Stroke;
    use typst_core::entities::geometry::ShapeKind;
    use typst_core::entities::gradient::{Gradient, GradientStop, RelativeTo};
    use typst_core::entities::layout_types::{
        Angle, Color, ColorSpace, FrameItem, Page, PagedDocument, Point, Pt, Ratio, Rect,
        TransformMatrix,
    };

    fn gradient(space: ColorSpace, parent: bool) -> Gradient {
        let mut g = Gradient::conic_with_space(
            vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ],
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Angle::deg(0.0),
            space,
        );
        if let Gradient::Conic(c) = &mut g {
            std::sync::Arc::make_mut(c).relative =
                Some(if parent { RelativeTo::Parent } else { RelativeTo::Self_ });
        }
        g
    }

    fn shape(g: Gradient, x: f64, y: f64) -> FrameItem {
        FrameItem::Shape {
            pos: Point { x: Pt(x), y: Pt(y) },
            kind: ShapeKind::Rect,
            width: 20.0,
            height: 10.0,
            fill: Some(Paint::Gradient(g)),
            stroke: None,
            fill_rule: Default::default(),
            parent_bbox_at_emit: None,
        }
    }

    fn page(height: f64, items: Vec<FrameItem>) -> Page {
        Page {
            width: 100.0,
            height,
            items,
            background: vec![],
            foreground: vec![],
            numbering: None,
            logical_number: 1,
            supplement: Content::Empty,
            bleed: Default::default(),
            fill: Default::default(),
        }
    }

    fn group(
        pos: (f64, f64),
        matrix: TransformMatrix,
        items: Vec<FrameItem>,
    ) -> FrameItem {
        FrameItem::Group {
            paint_root: false,
            pos: Point { x: Pt(pos.0), y: Pt(pos.1) },
            matrix,
            items,
            clip_mask: None,
            inner_width: 70.0,
            inner_height: 50.0,
        }
    }

    fn matrices(doc: &PagedDocument, mode: StreamMode) -> Vec<Vec<f64>> {
        let pdf = export_pdf(doc, mode);
        let text = String::from_utf8_lossy(&pdf);
        text.split("/PatternType 2")
            .skip(1)
            .map(|s| {
                s.split_once("/Matrix [")
                    .unwrap()
                    .1
                    .split_once(']')
                    .unwrap()
                    .0
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect()
            })
            .collect()
    }

    #[test]
    fn conic_placement_process_self_uses_shape_extent() {
        for space in [ColorSpace::Srgb, ColorSpace::Cmyk] {
            let doc = PagedDocument::new(vec![page(
                120.0,
                vec![shape(gradient(space, false), 12.0, 9.0)],
            )]);
            for mode in [StreamMode::Compact, StreamMode::Verbose] {
                assert_eq!(
                    matrices(&doc, mode),
                    vec![vec![20.0, 0.0, 0.0, 10.0, 12.0, 101.0]]
                );
            }
        }
    }

    #[test]
    fn conic_placement_same_arc_retains_occurrences_and_page_heights() {
        let g = gradient(ColorSpace::Srgb, false);
        let doc = PagedDocument::new(vec![
            page(120.0, vec![shape(g.clone(), 10.0, 20.0), shape(g.clone(), 40.0, 30.0)]),
            page(240.0, vec![shape(g, 10.0, 20.0)]),
        ]);
        assert_eq!(
            matrices(&doc, StreamMode::Compact),
            vec![
                vec![20.0, 0.0, 0.0, 10.0, 10.0, 90.0],
                vec![20.0, 0.0, 0.0, 10.0, 40.0, 80.0],
                vec![20.0, 0.0, 0.0, 10.0, 10.0, 210.0],
            ]
        );
    }

    #[test]
    fn conic_placement_composes_nested_affine_groups_and_bleed() {
        let inner = group(
            (4.0, 6.0),
            TransformMatrix { a: 1.0, b: 0.25, c: 0.5, d: 1.0, tx: 2.0, ty: 3.0 },
            vec![shape(gradient(ColorSpace::Srgb, false), 2.0, 3.0)],
        );
        let outer = group(
            (100.0, 60.0),
            TransformMatrix { a: 0.0, b: 2.0, c: -3.0, d: 0.0, tx: 5.0, ty: 7.0 },
            vec![inner],
        );
        let mut p = page(200.0, vec![outer]);
        p.bleed.left = 3.0;
        p.bleed.bottom = 5.0;
        let doc = PagedDocument::new(vec![p]);
        for mode in [StreamMode::Compact, StreamMode::Verbose] {
            assert_eq!(
                matrices(&doc, mode),
                vec![vec![-15.0, -40.0, 30.0, 10.0, 40.5, 109.0]]
            );
        }
    }

    #[test]
    fn conic_placement_parent_uses_local_group_or_page_and_explicit_bbox() {
        let g = gradient(ColorSpace::Srgb, true);
        let local = group(
            (10.0, 20.0),
            TransformMatrix::identity(),
            vec![shape(g.clone(), 2.0, 3.0)],
        );
        let mut explicit = shape(g.clone(), 2.0, 3.0);
        if let FrameItem::Shape { parent_bbox_at_emit, .. } = &mut explicit {
            *parent_bbox_at_emit =
                Some(Rect { x: Pt(5.0), y: Pt(6.0), w: Pt(30.0), h: Pt(40.0) });
        }
        let doc = PagedDocument::new(vec![page(
            200.0,
            vec![
                shape(g, 2.0, 3.0),
                local,
                group((10.0, 20.0), TransformMatrix::identity(), vec![explicit]),
            ],
        )]);
        assert_eq!(
            matrices(&doc, StreamMode::Compact),
            vec![
                vec![100.0, 0.0, 0.0, 200.0, 0.0, 0.0],
                vec![70.0, 0.0, 0.0, 50.0, 10.0, 130.0],
                vec![30.0, 0.0, 0.0, 40.0, 15.0, 134.0],
            ]
        );
    }

    #[test]
    fn conic_placement_resource_and_paint_lookup_share_fill_stroke_occurrence() {
        let g = gradient(ColorSpace::Srgb, false);
        let mut s = shape(g.clone(), 10.0, 20.0);
        if let FrameItem::Shape { stroke, .. } = &mut s {
            *stroke = Some(Stroke {
                paint: Paint::Gradient(g.clone()),
                ..Stroke::default()
            });
        }
        let mut p = page(200.0, vec![s]);
        p.background = vec![shape(g.clone(), 30.0, 40.0)];
        p.foreground = vec![group(
            (10.0, 20.0),
            TransformMatrix::identity(),
            vec![shape(g, 1.0, 2.0)],
        )];
        let doc = PagedDocument::new(vec![p]);
        let (refs, map, objects) = scan_all_gradients(&doc, 10);
        assert_eq!(objects.len(), 3, "separate placements; same fill/stroke shares");
        let resources = pattern_resources_for_page(&doc.pages[0], &map, &refs);
        for r in &refs {
            assert!(resources.contains(&format!("/{} ", r.name)));
        }
        let images = HashMap::new();
        let ctx = crate::export::PageContext::type1(
            &images,
            &[],
            &map,
            &refs,
            StreamMode::Compact,
        );
        let stream =
            String::from_utf8(crate::export::build_page_stream(&doc.pages[0], &ctx))
                .unwrap();
        assert_eq!(stream.matches("/Pattern cs").count(), 3, "{stream}");
        assert_eq!(stream.matches("/Pattern CS").count(), 1, "{stream}");
        for r in &refs {
            assert!(stream.contains(&format!("/{} scn", r.name)), "{stream}");
        }
    }
}
