//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/stream/draw.md
//! @prompt-hash 39c72505
//! @layer L3
//! @updated 2026-09-28

use super::{
    emit_fill_paint, emit_glyph_pdf, emit_glyph_pdf_marked,
    emit_glyph_pdf_verbose, emit_glyph_pdf_verbose_marked, emit_rounded_rect_ops,
    emit_shape_path_local, emit_shaped_pdf, emit_shaped_pdf_verbose, emit_stroke_paint,
    emit_text_pdf, emit_text_pdf_verbose, emit_text_pdf_deferred, emit_text_pdf_verbose_deferred,
    emit_shaped_pdf_deferred, emit_shaped_pdf_verbose_deferred, group_bbox_from_fields, line_cap_prefix,
    line_rg_prefix, paint_effective_bbox, pdf_defaults, PageContext, PdfTags, StreamMode,
};
use std::sync::Arc;
use typst_core::entities::elements::pdf_artifact::ArtifactKind;
use typst_core::entities::layout_types::FrameItem;
use typst_core::entities::layout_types::TransformMatrix;

/// **P776** — calcula a matriz `cm` para um `FrameItem::Image`, compondo a
/// transformação EXIF (1-8) com a escala e posição de layout.
///
/// Replica as matrizes `cm` do vanilla `exif_transform` em
/// `crates/typst-pdf/src/image.rs`: JPEGs não são recodificados; a orientação
/// é aplicada directamente na matriz PDF. As dimensões `width`/`height` são as
/// dimensões de layout finais (L1 já trocou `width`/`height` para orientações
/// 5-8).
pub(super) fn image_exif_matrix(
    orientation: u32,
    width: f64,
    height: f64,
    pos_x: f64,
    pdf_y: f64,
) -> TransformMatrix {
    let (a, b, c, d, tx, ty) = match orientation {
        2 => (-width, 0.0, 0.0, height, width, 0.0),
        3 => (-width, 0.0, 0.0, -height, width, height),
        4 => (width, 0.0, 0.0, -height, 0.0, height),
        5 => (0.0, -height, -width, 0.0, width, height),
        6 => (0.0, -height, width, 0.0, 0.0, height),
        7 => (0.0, height, width, 0.0, 0.0, 0.0),
        8 => (0.0, height, -width, 0.0, width, 0.0),
        _ => (width, 0.0, 0.0, height, 0.0, 0.0),
    };

    TransformMatrix { a, b, c, d, tx: tx + pos_x, ty: ty + pdf_y }
}

pub(super) fn semantic_opening(
    item: &FrameItem,
    kind: typst_core::entities::layout_types::SemanticKind,
    ctx: &PageContext,
) -> Option<String> {
    use typst_core::entities::layout_types::SemanticKind;

    if ctx.tags != PdfTags::Enabled {
        return None;
    }
    match kind {
        SemanticKind::Formula => {
            let key = item as *const FrameItem as usize;
            ctx.semantic_mcids
                .get(&key)
                .map(|mcid| format!("/Formula << /MCID {mcid} >> BDC\n"))
        }
        SemanticKind::Label => ctx
            .semantic_mcids
            .get(&(item as *const FrameItem as usize))
            .map(|mcid| format!("/Lbl << /MCID {mcid} >> BDC\n")),
        SemanticKind::Artifact(ArtifactKind::Header) => Some(
            "/Artifact << /Attached [/Top] /Subtype /Header /Type /Pagination >> BDC\n"
                .into(),
        ),
        SemanticKind::Artifact(ArtifactKind::Background) => {
            Some("/Artifact << /Type /Background >> BDC\n".into())
        }
        SemanticKind::Artifact(_) => Some("/Artifact BMC\n".into()),
        SemanticKind::TableHeaderCell { .. } => ctx
            .semantic_mcids
            .get(&(item as *const FrameItem as usize))
            .map(|mcid| format!("/TH << /MCID {mcid} >> BDC\n")),
        SemanticKind::TableDataCell { .. } => ctx
            .semantic_mcids
            .get(&(item as *const FrameItem as usize))
            .map(|mcid| format!("/TD << /MCID {mcid} >> BDC\n")),
        SemanticKind::Table { .. }
        | SemanticKind::TableHead { .. }
        | SemanticKind::TableBody { .. }
        | SemanticKind::TableRow { .. }
        | SemanticKind::ExplicitLinebreakBoundary
        | SemanticKind::ParbreakBoundary => None,
    }
}

/// P1481: o role do stream corresponde ao leaf no StructTreeRoot.
pub(super) fn ua1_leaf_opening(item: &FrameItem, ctx: &PageContext) -> Option<String> {
    if !ctx.ua1 {
        return None;
    }
    let mcid = ctx.semantic_mcids.get(&(item as *const FrameItem as usize))?;
    let role = match item {
        FrameItem::TextSeparator { .. } | FrameItem::Text { .. }
        | FrameItem::TextShaped { .. }
        | FrameItem::Glyph { .. } => "Span",
        FrameItem::Image { .. } => "Figure",
        _ => return None,
    };
    Some(format!("/{role} << /MCID {mcid} >> BDC\n"))
}

/// The Large-operator envelope supplies one causal text for already-positioned
/// glyph parts. A leaf MCID would interleave with this spanning ActualText,
/// so retain the generic path whenever one is present.
fn glyph_replacement_can_mark(items: &[FrameItem], ctx: &PageContext) -> bool {
    !items.is_empty()
        && !matches!(&ctx.font_scenario, super::FontScenario::Type1)
        && items.iter().all(|item| matches!(item, FrameItem::Glyph { .. }))
        && items.iter().all(|item| ua1_leaf_opening(item, ctx).is_none())
}

/// **P788** — emissão top-level (com flip Y-down→PDF: `page_height - pos.y`)
/// de UM `FrameItem`, extraída de `build_page_stream` para que filhos de
/// `FrameItem::Link` ao nível da página sigam o mesmo caminho. Antes eram
/// desenhados por `draw_item_local` (sem flip — assume matriz de Group) e
/// apareciam no fundo da página (bug medido em P786 A9 e reproduzido com
/// `#link` genérico: filho a y≈754 em vez de y≈68).
///
/// Recebe e devolve `ops` por valor para que o braço `Link` possa chamar
/// recursivamente sem conflitos de borrow.
pub(super) fn draw_item_top(
    ops: String,
    item: &FrameItem,
    page_height: f64,
    ctx: &PageContext,
) -> String {
    draw_item_top_inner(ops, item, page_height, ctx, false).0
}

fn draw_item_top_inner(
    mut ops: String,
    item: &FrameItem,
    page_height: f64,
    ctx: &PageContext,
    defer_restore: bool,
) -> (String, bool) {
    let mut deferred = false;
    use typst_core::entities::geometry::ShapeKind;
    let ua1_opening = ua1_leaf_opening(item, ctx);
    let ua1_decoration =
        ctx.ua1 && matches!(item, FrameItem::Shape { .. } | FrameItem::Line { .. });
    if let Some(opening) = &ua1_opening {
        ops.push_str(opening);
    }
    if ua1_decoration {
        ops.push_str("/Artifact << /Type /Layout >> BDC\n");
    }
    match item {
        FrameItem::Text { pos, text, style } if defer_restore => {
            deferred = match ctx.mode {
                StreamMode::Compact => emit_text_pdf_deferred(&mut ops, pos.x.0, page_height - pos.y.0, text, style, &ctx.font_scenario),
                StreamMode::Verbose => emit_text_pdf_verbose_deferred(&mut ops, pos.x.0, page_height - pos.y.0, text, style, &ctx.font_scenario),
            };
        }
        FrameItem::TextShaped { pos, glyphs, style, text, units_per_em } if defer_restore => {
            deferred = match ctx.mode {
                StreamMode::Compact => emit_shaped_pdf_deferred(&mut ops, pos.x.0, page_height - pos.y.0, glyphs, text, style, &ctx.font_scenario, *units_per_em),
                StreamMode::Verbose => emit_shaped_pdf_verbose_deferred(&mut ops, pos.x.0, page_height - pos.y.0, glyphs, text, style, &ctx.font_scenario, *units_per_em),
            };
        }
        FrameItem::TextSeparator { pos, style, .. } => super::separator::emit(&mut ops, pos.x.val(), page_height - pos.y.val(), style, ctx),
        // P483 — path primário: glifos com shaping real.
        // **P956** — dispatch por modo: Compact = formato Passo 20
        // byte-inalterado; Verbose = envelope vanilla-espelhado (mesma
        // coordenada `pdf_y` — derivação F·F=I, stream.md §P956).
        FrameItem::TextShaped { pos, glyphs, style, text, units_per_em } => {
            let pdf_y = page_height - pos.y.val();
            match ctx.mode {
                StreamMode::Compact => emit_shaped_pdf(
                    &mut ops,
                    pos.x.val(),
                    pdf_y,
                    glyphs,
                    text.as_str(),
                    style,
                    &ctx.font_scenario,
                    *units_per_em,
                ),
                StreamMode::Verbose => emit_shaped_pdf_verbose(
                    &mut ops,
                    pos.x.val(),
                    pdf_y,
                    glyphs,
                    text.as_str(),
                    style,
                    &ctx.font_scenario,
                    *units_per_em,
                ),
            }
        }
        // P483 — fallback: fonte não carregada, Type1, ou shaping indisponível.
        FrameItem::Text { pos, text, style } => {
            let pdf_y = page_height - pos.y.val();
            match ctx.mode {
                StreamMode::Compact => emit_text_pdf(
                    &mut ops,
                    pos.x.val(),
                    pdf_y,
                    text.as_str(),
                    style,
                    &ctx.font_scenario,
                ),
                StreamMode::Verbose => emit_text_pdf_verbose(
                    &mut ops,
                    pos.x.val(),
                    pdf_y,
                    text.as_str(),
                    style,
                    &ctx.font_scenario,
                ),
            }
        }
        FrameItem::Line { start, end, thickness, color, cap } => {
            let x1 = start.x.val();
            let y1 = page_height - start.y.val();
            let x2 = end.x.val();
            let y2 = page_height - end.y.val();
            // P285: `color: Some(c)` injecta `r g b RG ` antes de `w`;
            // `color: None` produz string vazia → bit-exact pré-P285
            // (frac/sqrt/line sem stroke explícito preservam bytes
            // exactos vs baseline P284).
            let rg = line_rg_prefix(color);
            let cap = line_cap_prefix(*cap);
            ops.push_str(&format!(
                "q {}{:.3} w {}{:.5} {:.5} m {:.5} {:.5} l S Q\n",
                rg, thickness, cap, x1, y1, x2, y2
            ));
        }
        FrameItem::Glyph { pos, glyph_id, size, style, .. } => {
            let pdf_y = page_height - pos.y.val();
            match ctx.mode {
                StreamMode::Compact => emit_glyph_pdf(
                    &mut ops,
                    pos.x.val(),
                    pdf_y,
                    *glyph_id,
                    *size,
                    style,
                    &ctx.font_scenario,
                ),
                StreamMode::Verbose => emit_glyph_pdf_verbose(
                    &mut ops,
                    pos.x.val(),
                    pdf_y,
                    *glyph_id,
                    *size,
                    style,
                    &ctx.font_scenario,
                ),
            }
        }
        FrameItem::Image {
            pos,
            data,
            width,
            height,
            intrinsic_width,
            intrinsic_height,
            clip_rect,
            orientation,
            ..
        } => {
            let ptr = Arc::as_ptr(data) as usize;
            if let Some(&idx) = ctx.ptr_to_idx.get(&ptr) {
                // pos.y é o TOPO da imagem → canto inferior esquerdo no espaço PDF.
                let pdf_y = page_height - pos.y.val() - height.val();
                let matrix = image_exif_matrix(
                    *orientation,
                    width.val(),
                    height.val(),
                    pos.x.val(),
                    pdf_y,
                );
                ops.push_str("q\n");
                if let Some(clip) = clip_rect {
                    // P771 — clip_path ao rectângulo target, replicando o vanilla.
                    let clip_pdf_y = page_height - clip.y.val() - clip.h.val();
                    ops.push_str(&format!(
                        "{:.3} {:.3} {:.3} {:.3} re W n\n",
                        clip.x.val(),
                        clip_pdf_y,
                        clip.w.val(),
                        clip.h.val()
                    ));
                }
                // **P777** — precisão aumentada para 5 casas decimais,
                // replicando o vanilla e evitando desvios de 1 px nas
                // bordas em orientações EXIF com flip/rotate.
                ops.push_str(&format!(
                    "{:.5} {:.5} {:.5} {:.5} {:.5} {:.5} cm\n/{} Do\nQ\n",
                    matrix.a,
                    matrix.b,
                    matrix.c,
                    matrix.d,
                    matrix.tx,
                    matrix.ty,
                    ctx.img_refs[idx].name
                ));
            }
        }
        FrameItem::Shape {
            pos,
            kind,
            width,
            height,
            fill,
            stroke,
            parent_bbox_at_emit,
            ..
        } => {
            // Inverter eixo Y: layout tem Y crescente para baixo; PDF crescente para cima.
            let pdf_y = page_height - pos.y.val() - height;

            ops.push_str("q\n");

            // Cor de preenchimento (rg — RGB para fills).
            if let Some(c) = fill {
                emit_fill_paint(
                    &mut ops,
                    c,
                    paint_effective_bbox(c, *pos, *width, *height, *parent_bbox_at_emit),
                    ctx.pat_ptr_to_idx,
                    ctx.pat_refs,
                );
            }

            // Cor e espessura do contorno (RG + w).
            if let Some(s) = stroke {
                emit_stroke_paint(
                    &mut ops,
                    &s.paint,
                    s.thickness,
                    paint_effective_bbox(
                        &s.paint,
                        *pos,
                        *width,
                        *height,
                        *parent_bbox_at_emit,
                    ),
                    ctx.pat_ptr_to_idx,
                    ctx.pat_refs,
                );
            }

            // Path — depende do tipo de forma.
            match kind {
                ShapeKind::Rect => {
                    ops.push_str(&format!(
                        "{:.2} {:.2} {:.2} {:.2} re\n",
                        pos.x.val(),
                        pdf_y,
                        width,
                        height
                    ));
                }
                ShapeKind::RoundedRect { radii } => {
                    emit_rounded_rect_ops(
                        &mut ops,
                        pos.x.val(),
                        pdf_y,
                        *width,
                        *height,
                        radii,
                    );
                }
                ShapeKind::Ellipse => {
                    let cx = pos.x.val() + width / 2.0;
                    let cy = pdf_y + height / 2.0;
                    let rx = width / 2.0;
                    let ry = height / 2.0;
                    let ox = rx * pdf_defaults::BEZIER_CIRCLE_KAPPA;
                    let oy = ry * pdf_defaults::BEZIER_CIRCLE_KAPPA;
                    ops.push_str(&format!("{:.3} {:.3} m\n", cx, cy + ry));
                    ops.push_str(&format!(
                        "{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
                        cx + ox,
                        cy + ry,
                        cx + rx,
                        cy + oy,
                        cx + rx,
                        cy
                    ));
                    ops.push_str(&format!(
                        "{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
                        cx + rx,
                        cy - oy,
                        cx + ox,
                        cy - ry,
                        cx,
                        cy - ry
                    ));
                    ops.push_str(&format!(
                        "{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
                        cx - ox,
                        cy - ry,
                        cx - rx,
                        cy - oy,
                        cx - rx,
                        cy
                    ));
                    ops.push_str(&format!(
                        "{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
                        cx - rx,
                        cy + oy,
                        cx - ox,
                        cy + ry,
                        cx,
                        cy + ry
                    ));
                }
                ShapeKind::Line { dx, dy } => {
                    let start_offset_x = if *dx < 0.0 { *width } else { 0.0 };
                    let end_offset_x = if *dx < 0.0 { 0.0 } else { *width };
                    let start_offset_y = if *dy > 0.0 { *height } else { 0.0 };
                    let end_offset_y = if *dy > 0.0 { 0.0 } else { *height };
                    let start_x = pos.x.val() + start_offset_x;
                    let start_y = pdf_y + start_offset_y;
                    let end_x = pos.x.val() + end_offset_x;
                    let end_y = pdf_y + end_offset_y;
                    ops.push_str(&format!("{:.3} {:.3} m\n", start_x, start_y));
                    ops.push_str(&format!("{:.3} {:.3} l\n", end_x, end_y));
                }
                ShapeKind::Path(items) => {
                    use typst_core::entities::geometry::PathItem;
                    for item in items {
                        match item {
                            PathItem::MoveTo(p) => ops.push_str(&format!(
                                "{:.2} {:.2} m\n",
                                pos.x.val() + p.x.0,
                                page_height - (pos.y.val() + p.y.0),
                            )),
                            PathItem::LineTo(p) => ops.push_str(&format!(
                                "{:.2} {:.2} l\n",
                                pos.x.val() + p.x.0,
                                page_height - (pos.y.val() + p.y.0),
                            )),
                            PathItem::CubicTo(p1, p2, p3) => ops.push_str(&format!(
                                "{:.2} {:.2} {:.2} {:.2} {:.2} {:.2} c\n",
                                pos.x.val() + p1.x.0,
                                page_height - (pos.y.val() + p1.y.0),
                                pos.x.val() + p2.x.0,
                                page_height - (pos.y.val() + p2.y.0),
                                pos.x.val() + p3.x.0,
                                page_height - (pos.y.val() + p3.y.0),
                            )),
                            PathItem::ClosePath => ops.push_str("h\n"),
                        }
                    }
                }
            }

            match (fill.is_some(), stroke.is_some()) {
                (true, true) => ops.push_str("B\n"),
                (true, false) => ops.push_str("f\n"),
                (false, true) => ops.push_str("S\n"),
                (false, false) => {}
            }

            ops.push_str("Q\n");
        }
        FrameItem::Group {
            pos,
            matrix,
            clip_mask,
            inner_width,
            inner_height,
            items,
        } => {
            let pdf_y = page_height - pos.y.val();

            // O layouter usa Y-down; o PDF usa Y-up.
            // Inverter d para manter a orientação padrão Y-down do layout dentro do Group.
            ops.push_str("q\n");
            ops.push_str(&format!(
                "{:.6} {:.6} {:.6} {:.6} {:.5} {:.5} cm\n",
                matrix.a,
                -matrix.b,
                matrix.c,
                -matrix.d,
                pos.x.val() + matrix.tx,
                pdf_y - matrix.ty,
            ));

            if let Some(mask) = clip_mask {
                emit_shape_path_local(&mut ops, mask, *inner_width, *inner_height);
                ops.push_str("W n\n");
            }

            // P273.13 / P278 — Group bbox via helper consolidado.
            let group_bbox = group_bbox_from_fields(*pos, *inner_width, *inner_height);
            for child in items {
                // P281 — recurse com mesmo ctx (substitui cascade
                // de 6 params P279).
                draw_item_local(&mut ops, child, Some(group_bbox), ctx);
            }
            ops.push_str("Q\n");
        }
        // **P425-A7**: FrameItem::Link transportado como Group sem annotation
        // URI por enquanto. Emissão de /Annot requer decisão arquitetural sobre
        // bbox/posição do Link.
        // **P788** — filhos seguem o caminho top-level COM flip Y (eram
        // `draw_item_local`, sem flip — apareciam no fundo da página).
        FrameItem::Link { items, .. } => {
            for child in items {
                ops = draw_item_top(ops, child, page_height, ctx);
            }
        }
        FrameItem::TextReplacement { text, items } => {
            if glyph_replacement_can_mark(items, ctx) {
                for (index, child) in items.iter().enumerate() {
                    let FrameItem::Glyph { pos, glyph_id, size, style, .. } = child else { unreachable!() };
                    let open = (index == 0).then_some(text.as_str());
                    let close = index + 1 == items.len();
                    match ctx.mode {
                        StreamMode::Compact => emit_glyph_pdf_marked(&mut ops,
                            pos.x.val(), page_height - pos.y.val(), *glyph_id, *size,
                            style, &ctx.font_scenario, open, close),
                        StreamMode::Verbose => emit_glyph_pdf_verbose_marked(&mut ops,
                            pos.x.val(), page_height - pos.y.val(), *glyph_id, *size,
                            style, &ctx.font_scenario, open, close),
                    }
                }
            } else {
                super::text_replacement::opening(&mut ops, text);
                let mut tail = false;
                for (index, child) in items.iter().enumerate() {
                    let result = draw_item_top_inner(ops, child, page_height, ctx, index + 1 == items.len());
                    ops = result.0;
                    tail = result.1;
                }
                super::text_replacement::closing(&mut ops);
                if tail { ops.push_str("Q\n"); }
            }
        }
        FrameItem::Semantic { kind, items, .. } => {
            if ctx.ua1 && matches!(kind,
                typst_core::entities::layout_types::SemanticKind::ParbreakBoundary
                | typst_core::entities::layout_types::SemanticKind::ExplicitLinebreakBoundary
            ) { return (ops, false); }
            let opening = semantic_opening(item, *kind, ctx);
            if let Some(ref opening) = opening {
                ops.push_str(&opening);
            }
            for child in items {
                ops = draw_item_top(ops, child, page_height, ctx);
            }
            if opening.is_some() {
                ops.push_str("EMC\n");
            }
        }
    }
    if ua1_opening.is_some() || ua1_decoration {
        ops.push_str("EMC\n");
    }
    (ops, deferred)
}

/// **P1120** — abre o envelope de reflexão do texto dentro de um `Group`.
///
/// O `cm` do `Group` (§P1119) é a matriz Typst→PDF **completa**
/// (`[a, -b, c, -d]`, com o flip do eixo Y já composto) — a mesma que o
/// vanilla emite por bloco de glifo. Os helpers de emissão de texto são
/// partilhados com o caminho top-level e assumem que o flip ainda **não**
/// foi aplicado: o verbose emite o seu próprio `cm 1 0 0 -1 x y` + `Tm
/// 1 0 0 -1` (duplo flip = glifo direito), o compacto emite `x y Td` sem
/// flip nenhum. Debaixo do `cm` do `Group`, qualquer um dos dois fica com
/// um flip a mais → **glifos espelhados na vertical** (medido em
/// `.typ/sec_36.typ`, P1120; posições certas, orientação invertida).
///
/// A correcção repõe a pré-condição dos helpers: um `cm` de reflexão
/// (`1 0 0 -1 0 0`) antes do bloco e a coordenada reflectida (`-pos.y`)
/// devolvida por esta função. F∘F = I no eixo do glifo (fica direito) e a
/// reflexão da coordenada recoloca a posição y-down local no sítio certo —
/// derivação em `stream.md` §P1120. Fecha com `group_text_flip_close`.
pub(super) fn group_text_flip_open(ops: &mut String, pos_y: f64) -> f64 {
    ops.push_str("q\n1 0 0 -1 0 0 cm\n");
    -pos_y
}

/// **P1120** — fecha o envelope aberto por `group_text_flip_open`.
pub(super) fn group_text_flip_close(ops: &mut String) {
    ops.push_str("Q\n");
}

/// Desenha um `FrameItem` em espaço LOCAL (após `cm`).
///
/// Diferença de `draw_item_global`: não subtrai `page_height` — a matriz `cm`
/// já aplicou a transformação e a inversão Y. Os filhos usam `pos.y.0` directamente.
pub(in crate::export) fn draw_item_local(
    ops: &mut String,
    item: &FrameItem,
    // P273.13 — parameter threading paralelo P273.10 scan_all_gradients.walk
    // (Decisão 1α Fase A). Inner-wins via .or().
    parent_bbox_override: Option<typst_core::entities::layout_types::Rect>,
    // P281 — substitui cascade P279 (ptr_to_idx, img_refs, pat_ptr_to_idx,
    // pat_refs) por agregador unificado `&PageContext`. Text/Glyph arm
    // dispatch via `ctx.font_scenario`.
    ctx: &PageContext,
) {
    let _ = draw_item_local_inner(ops, item, parent_bbox_override, ctx, false);
}

fn draw_item_local_inner(
    ops: &mut String,
    item: &FrameItem,
    parent_bbox_override: Option<typst_core::entities::layout_types::Rect>,
    ctx: &PageContext,
    defer_restore: bool,
) -> usize {
    let mut restores = 0usize;
    use typst_core::entities::geometry::ShapeKind;
    use typst_core::entities::layout_types::FrameItem;
    let ua1_opening = ua1_leaf_opening(item, ctx);
    let ua1_decoration =
        ctx.ua1 && matches!(item, FrameItem::Shape { .. } | FrameItem::Line { .. });
    if let Some(opening) = &ua1_opening {
        ops.push_str(opening);
    }
    if ua1_decoration {
        ops.push_str("/Artifact << /Type /Layout >> BDC\n");
    }
    match item {
        FrameItem::Text { pos, text, style } if defer_restore => {
            ops.push_str("q\n1 0 0 -1 0 0 cm\n");
            match ctx.mode {
                StreamMode::Compact => { let tail = emit_text_pdf_deferred(ops, pos.x.0, -pos.y.0, text, style, &ctx.font_scenario); if tail { restores += 1; } },
                StreamMode::Verbose => { let tail = emit_text_pdf_verbose_deferred(ops, pos.x.0, -pos.y.0, text, style, &ctx.font_scenario); if tail { restores += 1; } },
            }
            restores += 1;
        }
        FrameItem::TextShaped { pos, glyphs, style, text, units_per_em } if defer_restore => {
            ops.push_str("q\n1 0 0 -1 0 0 cm\n");
            match ctx.mode {
                StreamMode::Compact => { let tail = emit_shaped_pdf_deferred(ops, pos.x.0, -pos.y.0, glyphs, text, style, &ctx.font_scenario, *units_per_em); if tail { restores += 1; } },
                StreamMode::Verbose => { let tail = emit_shaped_pdf_verbose_deferred(ops, pos.x.0, -pos.y.0, glyphs, text, style, &ctx.font_scenario, *units_per_em); if tail { restores += 1; } },
            }
            restores += 1;
        }
        FrameItem::TextSeparator { pos, style, .. } => {
            let y_eff = group_text_flip_open(ops, pos.y.0);
            super::separator::emit(ops, pos.x.0, y_eff, style, ctx);
            group_text_flip_close(ops);
        },
        FrameItem::Shape {
            pos,
            kind,
            width,
            height,
            fill,
            stroke,
            parent_bbox_at_emit,
            ..
        } => {
            let local_y = pos.y.0;
            ops.push_str("q\n");
            if let Some(c) = fill {
                let parent = parent_bbox_at_emit.or(parent_bbox_override);
                let effective_bbox =
                    paint_effective_bbox(c, *pos, *width, *height, parent);
                emit_fill_paint(ops, c, effective_bbox, ctx.pat_ptr_to_idx, ctx.pat_refs);
            }
            if let Some(s) = stroke {
                let parent = parent_bbox_at_emit.or(parent_bbox_override);
                let effective_bbox =
                    paint_effective_bbox(&s.paint, *pos, *width, *height, parent);
                emit_stroke_paint(
                    ops,
                    &s.paint,
                    s.thickness,
                    effective_bbox,
                    ctx.pat_ptr_to_idx,
                    ctx.pat_refs,
                );
            }
            match kind {
                ShapeKind::Rect => {
                    ops.push_str(&format!(
                        "{:.2} {:.2} {:.2} {:.2} re\n",
                        pos.x.0, local_y, width, height
                    ));
                }
                ShapeKind::RoundedRect { radii } => {
                    // P242 — Bezier 4 corners em espaço local.
                    emit_rounded_rect_ops(
                        &mut *ops, pos.x.0, local_y, *width, *height, radii,
                    );
                }
                ShapeKind::Ellipse => {
                    let cx = pos.x.0 + width / 2.0;
                    let cy = local_y + height / 2.0;
                    let rx = width / 2.0;
                    let ry = height / 2.0;
                    let ox = rx * pdf_defaults::BEZIER_CIRCLE_KAPPA;
                    let oy = ry * pdf_defaults::BEZIER_CIRCLE_KAPPA;
                    ops.push_str(&format!("{:.3} {:.3} m\n", cx, cy + ry));
                    ops.push_str(&format!(
                        "{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
                        cx + ox,
                        cy + ry,
                        cx + rx,
                        cy + oy,
                        cx + rx,
                        cy
                    ));
                    ops.push_str(&format!(
                        "{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
                        cx + rx,
                        cy - oy,
                        cx + ox,
                        cy - ry,
                        cx,
                        cy - ry
                    ));
                    ops.push_str(&format!(
                        "{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
                        cx - ox,
                        cy - ry,
                        cx - rx,
                        cy - oy,
                        cx - rx,
                        cy
                    ));
                    ops.push_str(&format!(
                        "{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
                        cx - rx,
                        cy + oy,
                        cx - ox,
                        cy + ry,
                        cx,
                        cy + ry
                    ));
                }
                ShapeKind::Line { dx, dy } => {
                    let start_offset_x = if *dx < 0.0 { *width } else { 0.0 };
                    let end_offset_x = if *dx < 0.0 { 0.0 } else { *width };
                    let start_offset_y = if *dy > 0.0 { *height } else { 0.0 };
                    let end_offset_y = if *dy > 0.0 { 0.0 } else { *height };
                    ops.push_str(&format!(
                        "{:.3} {:.3} m\n",
                        pos.x.0 + start_offset_x,
                        local_y + start_offset_y
                    ));
                    ops.push_str(&format!(
                        "{:.3} {:.3} l\n",
                        pos.x.0 + end_offset_x,
                        local_y + end_offset_y
                    ));
                }
                ShapeKind::Path(items) => {
                    use typst_core::entities::geometry::PathItem;
                    for item in items {
                        match item {
                            PathItem::MoveTo(p) => ops.push_str(&format!(
                                "{:.2} {:.2} m\n",
                                pos.x.0 + p.x.0,
                                -(local_y + p.y.0),
                            )),
                            PathItem::LineTo(p) => ops.push_str(&format!(
                                "{:.2} {:.2} l\n",
                                pos.x.0 + p.x.0,
                                -(local_y + p.y.0),
                            )),
                            PathItem::CubicTo(p1, p2, p3) => ops.push_str(&format!(
                                "{:.2} {:.2} {:.2} {:.2} {:.2} {:.2} c\n",
                                pos.x.0 + p1.x.0,
                                -(local_y + p1.y.0),
                                pos.x.0 + p2.x.0,
                                -(local_y + p2.y.0),
                                pos.x.0 + p3.x.0,
                                -(local_y + p3.y.0),
                            )),
                            PathItem::ClosePath => ops.push_str("h\n"),
                        }
                    }
                }
            }
            match (fill.is_some(), stroke.is_some()) {
                (true, true) => ops.push_str("B\n"),
                (true, false) => ops.push_str("f\n"),
                (false, true) => ops.push_str("S\n"),
                (false, false) => {}
            }
            ops.push_str("Q\n");
        }
        // P273.13 — arm Group; nested Groups via recursão.
        // P281 — cascade único `&PageContext` (substitui 4 params P279).
        FrameItem::Group { pos, inner_width, inner_height, items, .. } => {
            let group_bbox = group_bbox_from_fields(*pos, *inner_width, *inner_height);
            for child in items {
                draw_item_local(ops, child, Some(group_bbox), ctx);
            }
        }
        // P279 — Image em Group emit local.
        FrameItem::Image {
            pos,
            data,
            width,
            height,
            intrinsic_width,
            intrinsic_height,
            orientation,
            ..
        } => {
            let ptr = std::sync::Arc::as_ptr(data) as usize;
            if let Some(&idx) = ctx.ptr_to_idx.get(&ptr) {
                // Local emit: pos.x/pos.y são coords locais (após Group cm).
                let matrix = image_exif_matrix(
                    *orientation,
                    width.val(),
                    height.val(),
                    pos.x.0,
                    pos.y.0,
                );
                ops.push_str(&format!(
                    "q\n{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} cm\n/{} Do\nQ\n",
                    matrix.a,
                    matrix.b,
                    matrix.c,
                    matrix.d,
                    matrix.tx,
                    matrix.ty,
                    ctx.img_refs[idx].name
                ));
            }
        }
        // **P281** — Text/Glyph/Line arms real (substituem stubs P278/P279).
        // P483 — path primário TextShaped; Text = fallback.
        // **P1120** — envelope de reflexão: ver `group_text_flip_open`. A
        // coordenada passada aos helpers é `-pos.y` (a reflexão do `cm` de
        // abertura devolve-a à posição y-down local).
        FrameItem::TextShaped { pos, glyphs, style, text, units_per_em } => {
            let y_eff = group_text_flip_open(ops, pos.y.0);
            match ctx.mode {
                StreamMode::Compact => emit_shaped_pdf(
                    ops,
                    pos.x.0,
                    y_eff,
                    glyphs,
                    text.as_str(),
                    style,
                    &ctx.font_scenario,
                    *units_per_em,
                ),
                StreamMode::Verbose => emit_shaped_pdf_verbose(
                    ops,
                    pos.x.0,
                    y_eff,
                    glyphs,
                    text.as_str(),
                    style,
                    &ctx.font_scenario,
                    *units_per_em,
                ),
            }
            group_text_flip_close(ops);
        }
        FrameItem::Text { pos, text, style } => {
            let y_eff = group_text_flip_open(ops, pos.y.0);
            match ctx.mode {
                StreamMode::Compact => emit_text_pdf(
                    ops,
                    pos.x.0,
                    y_eff,
                    text.as_str(),
                    style,
                    &ctx.font_scenario,
                ),
                StreamMode::Verbose => emit_text_pdf_verbose(
                    ops,
                    pos.x.0,
                    y_eff,
                    text.as_str(),
                    style,
                    &ctx.font_scenario,
                ),
            }
            group_text_flip_close(ops);
        }
        FrameItem::Glyph { pos, glyph_id, size, style, .. } => {
            let y_eff = group_text_flip_open(ops, pos.y.0);
            match ctx.mode {
                StreamMode::Compact => emit_glyph_pdf(
                    ops,
                    pos.x.0,
                    y_eff,
                    *glyph_id,
                    *size,
                    style,
                    &ctx.font_scenario,
                ),
                StreamMode::Verbose => emit_glyph_pdf_verbose(
                    ops,
                    pos.x.0,
                    y_eff,
                    *glyph_id,
                    *size,
                    style,
                    &ctx.font_scenario,
                ),
            }
            group_text_flip_close(ops);
        }
        FrameItem::Line { start, end, thickness, color, cap } => {
            // Local emit: coords locais (após Group `cm`). Y já invertido
            // pela matriz cm → preserved bit-exact com top-level emit
            // estructura (`q w m l S Q`) mas sem `page_height -` subtract.
            // P285: paridade simétrica com top-level — `color: Some(c)`
            // injecta `r g b RG ` antes de `w`; `color: None` preserva
            // bit-exact (Win arquitectural P281 — helpers unificados
            // garantem que mudança em ambos os emit ocorre simetricamente).
            let rg = line_rg_prefix(color);
            let cap = line_cap_prefix(*cap);
            ops.push_str(&format!(
                "q {}{:.3} w {}{:.1} {:.1} m {:.1} {:.1} l S Q\n",
                rg, thickness, cap, start.x.0, start.y.0, end.x.0, end.y.0
            ));
        }
        // **P425-A7**: transporte recursivo de Link no espaço local do Group.
        FrameItem::Link { items, .. } => {
            for child in items {
                draw_item_local(ops, child, parent_bbox_override, ctx);
            }
        }
        FrameItem::TextReplacement { text, items } => {
            if glyph_replacement_can_mark(items, ctx) {
                for (index, child) in items.iter().enumerate() {
                    let FrameItem::Glyph { pos, glyph_id, size, style, .. } = child else { unreachable!() };
                    let open = (index == 0).then_some(text.as_str());
                    let close = index + 1 == items.len();
                    let y_eff = group_text_flip_open(ops, pos.y.val());
                    match ctx.mode {
                        StreamMode::Compact => emit_glyph_pdf_marked(ops,
                            pos.x.val(), y_eff, *glyph_id, *size, style,
                            &ctx.font_scenario, open, close),
                        StreamMode::Verbose => emit_glyph_pdf_verbose_marked(ops,
                            pos.x.val(), y_eff, *glyph_id, *size, style,
                            &ctx.font_scenario, open, close),
                    }
                    group_text_flip_close(ops);
                }
            } else {
                super::text_replacement::opening(ops, text);
                let mut tails = 0;
                for (index, child) in items.iter().enumerate() {
                    tails = draw_item_local_inner(ops, child, parent_bbox_override, ctx, index + 1 == items.len());
                }
                super::text_replacement::closing(ops);
                for _ in 0..tails { ops.push_str("Q\n"); }
            }
        }
        FrameItem::Semantic { kind, items, .. } => {
            if ctx.ua1 && matches!(kind,
                typst_core::entities::layout_types::SemanticKind::ParbreakBoundary
                | typst_core::entities::layout_types::SemanticKind::ExplicitLinebreakBoundary
            ) { return 0; }
            let opening = semantic_opening(item, *kind, ctx);
            if let Some(ref opening) = opening {
                ops.push_str(&opening);
            }
            for child in items {
                draw_item_local(ops, child, parent_bbox_override, ctx);
            }
            if opening.is_some() {
                ops.push_str("EMC\n");
            }
        }
    }
    if ua1_opening.is_some() || ua1_decoration {
        ops.push_str("EMC\n");
    }
    restores
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exif_rotation_preserves_image_origin() {
        let matrix = image_exif_matrix(6, 30.0, 20.0, 7.0, 11.0);
        assert_eq!((matrix.a, matrix.b, matrix.c, matrix.d), (0.0, -20.0, 30.0, 0.0));
        assert_eq!((matrix.tx, matrix.ty), (7.0, 31.0));
    }
}
