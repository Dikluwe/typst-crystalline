//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/stream/draw.md
//! @prompt-hash 06648bd5
//! @layer L3
//! @updated 2026-10-08

use super::{
    emit_fill_paint, emit_glyph_pdf, emit_glyph_pdf_marked, emit_glyph_pdf_verbose,
    emit_glyph_pdf_verbose_marked, emit_rounded_rect_ops, emit_shape_path_local,
    emit_shaped_pdf, emit_shaped_pdf_deferred, emit_shaped_pdf_verbose,
    emit_shaped_pdf_verbose_deferred, emit_stroke_paint, emit_text_pdf,
    emit_text_pdf_deferred, emit_text_pdf_verbose, emit_text_pdf_verbose_deferred,
    group_bbox_from_fields, line_cap_prefix, line_rg_prefix, opacity_operator,
    paint_effective_bbox, pdf_defaults, solid_opacity, PageContext, PdfTags, StreamMode,
};
use std::sync::Arc;
use typst_core::entities::elements::pdf_artifact::ArtifactKind;
use typst_core::entities::layout_types::{FrameItem, Point, Pt, Rect, TransformMatrix};

fn image_clip_needed(clip: &Rect, pos: &Point, width: Pt, height: Pt) -> bool {
    clip.x.0 > pos.x.0
        || clip.y.0 > pos.y.0
        || clip.x.0 + clip.w.0 < pos.x.0 + width.0
        || clip.y.0 + clip.h.0 < pos.y.0 + height.0
}

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
        FrameItem::TextSeparator { .. }
        | FrameItem::Text { .. }
        | FrameItem::TextShaped { .. }
        | FrameItem::Glyph { .. } => "Span",
        FrameItem::Image { .. } => "Figure",
        FrameItem::TextReplacement { .. }
        | FrameItem::Semantic { .. }
        | FrameItem::Line { .. }
        | FrameItem::Shape { .. }
        | FrameItem::Group { .. }
        | FrameItem::Link { .. } => return None,
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
) -> (String, usize) {
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
    let (fill_alpha, stroke_alpha) = solid_opacity(item);
    let opacity = opacity_operator(fill_alpha, stroke_alpha);
    if !opacity.is_empty() {
        ops.push_str("q\n");
        ops.push_str(&opacity);
    }
    match item {
        FrameItem::Text { pos, text, style, .. } if defer_restore => {
            deferred = match ctx.mode {
                StreamMode::Compact => emit_text_pdf_deferred(
                    &mut ops,
                    pos.x.0,
                    page_height - pos.y.0,
                    text,
                    style,
                    &ctx.font_scenario,
                ),
                StreamMode::Verbose => emit_text_pdf_verbose_deferred(
                    &mut ops,
                    pos.x.0,
                    page_height - pos.y.0,
                    text,
                    style,
                    &ctx.font_scenario,
                ),
            };
        }
        FrameItem::TextShaped {
            pos, glyphs, style, text, units_per_em, font, ..
        } if defer_restore => {
            deferred = match ctx.mode {
                StreamMode::Compact => emit_shaped_pdf_deferred(
                    &mut ops,
                    pos.x.0,
                    page_height - pos.y.0,
                    glyphs,
                    text,
                    style,
                    &ctx.font_scenario,
                    *units_per_em,
                    font.as_ref(),
                ),
                StreamMode::Verbose => emit_shaped_pdf_verbose_deferred(
                    &mut ops,
                    pos.x.0,
                    page_height - pos.y.0,
                    glyphs,
                    text,
                    style,
                    &ctx.font_scenario,
                    *units_per_em,
                    font.as_ref(),
                ),
            };
        }
        FrameItem::TextSeparator { pos, style, origin, bidi, .. } => {
            super::separator::emit(
                &mut ops,
                pos.x.val(),
                page_height - pos.y.val(),
                style,
                *origin,
                bidi.as_ref(),
                ctx,
            )
        }
        // P483 — path primário: glifos com shaping real.
        // **P956** — dispatch por modo: Compact = formato Passo 20
        // byte-inalterado; Verbose = envelope vanilla-espelhado (mesma
        // coordenada `pdf_y` — derivação F·F=I, stream.md §P956).
        FrameItem::TextShaped {
            pos, glyphs, style, text, units_per_em, font, ..
        } => {
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
                    font.as_ref(),
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
                    font.as_ref(),
                ),
            }
        }
        // P483 — fallback: fonte não carregada, Type1, ou shaping indisponível.
        FrameItem::Text { pos, text, style, .. } => {
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
            pos, data, width, height, clip_rect, orientation, ..
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
                if let Some(clip) =
                    clip_rect.filter(|clip| image_clip_needed(clip, pos, *width, *height))
                {
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
                    item,
                    c,
                    paint_effective_bbox(c, *pos, *width, *height, *parent_bbox_at_emit),
                    ctx,
                );
            }

            // Cor e espessura do contorno (RG + w).
            if let Some(s) = stroke {
                emit_stroke_paint(
                    &mut ops,
                    item,
                    &s.paint,
                    s.thickness,
                    paint_effective_bbox(
                        &s.paint,
                        *pos,
                        *width,
                        *height,
                        *parent_bbox_at_emit,
                    ),
                    ctx,
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
                // End the unpainted path before another paint or clip can consume it.
                (false, false) => ops.push_str("n\n"),
            }

            ops.push_str("Q\n");
        }
        FrameItem::Group {
            paint_root: _,
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
                    let FrameItem::Glyph { pos, glyph_id, size, style, .. } = child
                    else {
                        unreachable!()
                    };
                    let open = (index == 0).then_some(text.as_str());
                    let close = index + 1 == items.len();
                    let (f, s) = solid_opacity(child);
                    let glyph_opacity = opacity_operator(f, s);
                    if !glyph_opacity.is_empty() {
                        ops.push_str("q\n");
                        ops.push_str(&glyph_opacity);
                    }
                    match ctx.mode {
                        StreamMode::Compact => emit_glyph_pdf_marked(
                            &mut ops,
                            pos.x.val(),
                            page_height - pos.y.val(),
                            *glyph_id,
                            *size,
                            style,
                            &ctx.font_scenario,
                            open,
                            close,
                        ),
                        StreamMode::Verbose => emit_glyph_pdf_verbose_marked(
                            &mut ops,
                            pos.x.val(),
                            page_height - pos.y.val(),
                            *glyph_id,
                            *size,
                            style,
                            &ctx.font_scenario,
                            open,
                            close,
                        ),
                    }
                    if !glyph_opacity.is_empty() {
                        ops.push_str("Q\n");
                    }
                }
            } else {
                super::text_replacement::opening(&mut ops, text);
                let mut tail = 0;
                for (index, child) in items.iter().enumerate() {
                    let result = draw_item_top_inner(
                        ops,
                        child,
                        page_height,
                        ctx,
                        index + 1 == items.len(),
                    );
                    ops = result.0;
                    tail = result.1;
                }
                super::text_replacement::closing(&mut ops);
                for _ in 0..tail {
                    ops.push_str("Q\n");
                }
            }
        }
        FrameItem::Semantic { kind, items, .. } => {
            if ctx.ua1 && matches!(kind,
                typst_core::entities::layout_types::SemanticKind::ParbreakBoundary
                | typst_core::entities::layout_types::SemanticKind::ExplicitLinebreakBoundary
            ) { return (ops, 0); }
            let opening = semantic_opening(item, *kind, ctx);
            if let Some(ref opening) = opening {
                ops.push_str(opening);
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
    let mut restores = usize::from(deferred);
    if !opacity.is_empty() {
        if defer_restore {
            restores += 1;
        } else {
            ops.push_str("Q\n");
        }
    }
    (ops, restores)
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
    let (fill_alpha, stroke_alpha) = solid_opacity(item);
    let opacity = opacity_operator(fill_alpha, stroke_alpha);
    if !opacity.is_empty() {
        ops.push_str("q\n");
        ops.push_str(&opacity);
    }
    match item {
        FrameItem::Text { pos, text, style, .. } if defer_restore => {
            ops.push_str("q\n1 0 0 -1 0 0 cm\n");
            match ctx.mode {
                StreamMode::Compact => {
                    let tail = emit_text_pdf_deferred(
                        ops,
                        pos.x.0,
                        -pos.y.0,
                        text,
                        style,
                        &ctx.font_scenario,
                    );
                    if tail {
                        restores += 1;
                    }
                }
                StreamMode::Verbose => {
                    let tail = emit_text_pdf_verbose_deferred(
                        ops,
                        pos.x.0,
                        -pos.y.0,
                        text,
                        style,
                        &ctx.font_scenario,
                    );
                    if tail {
                        restores += 1;
                    }
                }
            }
            restores += 1;
        }
        FrameItem::TextShaped {
            pos, glyphs, style, text, units_per_em, font, ..
        } if defer_restore => {
            ops.push_str("q\n1 0 0 -1 0 0 cm\n");
            match ctx.mode {
                StreamMode::Compact => {
                    let tail = emit_shaped_pdf_deferred(
                        ops,
                        pos.x.0,
                        -pos.y.0,
                        glyphs,
                        text,
                        style,
                        &ctx.font_scenario,
                        *units_per_em,
                        font.as_ref(),
                    );
                    if tail {
                        restores += 1;
                    }
                }
                StreamMode::Verbose => {
                    let tail = emit_shaped_pdf_verbose_deferred(
                        ops,
                        pos.x.0,
                        -pos.y.0,
                        glyphs,
                        text,
                        style,
                        &ctx.font_scenario,
                        *units_per_em,
                        font.as_ref(),
                    );
                    if tail {
                        restores += 1;
                    }
                }
            }
            restores += 1;
        }
        FrameItem::TextSeparator { pos, style, origin, bidi, .. } => {
            let y_eff = group_text_flip_open(ops, pos.y.0);
            super::separator::emit(
                ops,
                pos.x.0,
                y_eff,
                style,
                *origin,
                bidi.as_ref(),
                ctx,
            );
            group_text_flip_close(ops);
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
            let local_y = pos.y.0;
            ops.push_str("q\n");
            if let Some(c) = fill {
                let parent = parent_bbox_at_emit.or(parent_bbox_override);
                let effective_bbox =
                    paint_effective_bbox(c, *pos, *width, *height, parent);
                emit_fill_paint(ops, item, c, effective_bbox, ctx);
            }
            if let Some(s) = stroke {
                let parent = parent_bbox_at_emit.or(parent_bbox_override);
                let effective_bbox =
                    paint_effective_bbox(&s.paint, *pos, *width, *height, parent);
                emit_stroke_paint(ops, item, &s.paint, s.thickness, effective_bbox, ctx);
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
                // End the unpainted path before another paint or clip can consume it.
                (false, false) => ops.push_str("n\n"),
            }
            ops.push_str("Q\n");
        }
        // P273.13 — arm Group; nested Groups via recursão.
        // P281 — cascade único `&PageContext` (substitui 4 params P279).
        FrameItem::Group {
            paint_root: _,
            pos,
            matrix,
            clip_mask,
            inner_width,
            inner_height,
            items,
        } => {
            // The outer group already established Y-down coordinates. Compose
            // this local affine transform without a second page reflection.
            ops.push_str("q\n");
            ops.push_str(&format!(
                "{:.6} {:.6} {:.6} {:.6} {:.5} {:.5} cm\n",
                matrix.a,
                matrix.b,
                matrix.c,
                matrix.d,
                pos.x.0 + matrix.tx,
                pos.y.0 + matrix.ty,
            ));
            if let Some(mask) = clip_mask {
                emit_shape_path_local(ops, mask, *inner_width, *inner_height);
                ops.push_str("W n\n");
            }
            let group_bbox = group_bbox_from_fields(*pos, *inner_width, *inner_height);
            for child in items {
                draw_item_local(ops, child, Some(group_bbox), ctx);
            }
            ops.push_str("Q\n");
        }
        // P279 — Image em Group emit local.
        FrameItem::Image {
            pos, data, width, height, clip_rect, orientation, ..
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
                ops.push_str("q\n");
                if let Some(clip) =
                    clip_rect.filter(|clip| image_clip_needed(clip, pos, *width, *height))
                {
                    ops.push_str(&format!(
                        "{:.3} {:.3} {:.3} {:.3} re W n\n",
                        clip.x.val(),
                        clip.y.val(),
                        clip.w.val(),
                        clip.h.val()
                    ));
                }
                ops.push_str(&format!(
                    "{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} cm\n/{} Do\nQ\n",
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
        FrameItem::TextShaped {
            pos, glyphs, style, text, units_per_em, font, ..
        } => {
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
                    font.as_ref(),
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
                    font.as_ref(),
                ),
            }
            group_text_flip_close(ops);
        }
        FrameItem::Text { pos, text, style, .. } => {
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
                    let FrameItem::Glyph { pos, glyph_id, size, style, .. } = child
                    else {
                        unreachable!()
                    };
                    let open = (index == 0).then_some(text.as_str());
                    let close = index + 1 == items.len();
                    let (f, s) = solid_opacity(child);
                    let glyph_opacity = opacity_operator(f, s);
                    if !glyph_opacity.is_empty() {
                        ops.push_str("q\n");
                        ops.push_str(&glyph_opacity);
                    }
                    let y_eff = group_text_flip_open(ops, pos.y.val());
                    match ctx.mode {
                        StreamMode::Compact => emit_glyph_pdf_marked(
                            ops,
                            pos.x.val(),
                            y_eff,
                            *glyph_id,
                            *size,
                            style,
                            &ctx.font_scenario,
                            open,
                            close,
                        ),
                        StreamMode::Verbose => emit_glyph_pdf_verbose_marked(
                            ops,
                            pos.x.val(),
                            y_eff,
                            *glyph_id,
                            *size,
                            style,
                            &ctx.font_scenario,
                            open,
                            close,
                        ),
                    }
                    group_text_flip_close(ops);
                    if !glyph_opacity.is_empty() {
                        ops.push_str("Q\n");
                    }
                }
            } else {
                super::text_replacement::opening(ops, text);
                let mut tails = 0;
                for (index, child) in items.iter().enumerate() {
                    tails = draw_item_local_inner(
                        ops,
                        child,
                        parent_bbox_override,
                        ctx,
                        index + 1 == items.len(),
                    );
                }
                super::text_replacement::closing(ops);
                for _ in 0..tails {
                    ops.push_str("Q\n");
                }
            }
        }
        FrameItem::Semantic { kind, items, .. } => {
            if ctx.ua1 && matches!(kind,
                typst_core::entities::layout_types::SemanticKind::ParbreakBoundary
                | typst_core::entities::layout_types::SemanticKind::ExplicitLinebreakBoundary
            ) { return 0; }
            let opening = semantic_opening(item, *kind, ctx);
            if let Some(ref opening) = opening {
                ops.push_str(opening);
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
    if !opacity.is_empty() {
        if defer_restore {
            restores += 1;
        } else {
            ops.push_str("Q\n");
        }
    }
    restores
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image_clip_is_emitted_only_for_real_crop_in_top_and_local_streams() {
        use typst_core::entities::layout_types::{Point, Pt, Rect};
        let data = Arc::new(vec![1]);
        let mut images = std::collections::HashMap::new();
        images.insert(Arc::as_ptr(&data) as usize, 0);
        let refs = [super::super::ImageRef { main_obj_id: 1, name: "Im1".into() }];
        let patterns = std::collections::HashMap::new();
        let ctx = PageContext::type1(&images, &refs, &patterns, &[], StreamMode::Verbose);
        let item = |clip_rect| FrameItem::Image {
            pos: Point { x: Pt(8.0), y: Pt(8.0) },
            data: Arc::clone(&data),
            width: Pt(80.0),
            height: Pt(80.0),
            intrinsic_width: 1,
            intrinsic_height: 1,
            clip_rect,
            orientation: 1,
            alt: None,
        };
        let same = item(Some(Rect { x: Pt(8.0), y: Pt(8.0), w: Pt(80.0), h: Pt(80.0) }));
        let crop = item(Some(Rect { x: Pt(8.0), y: Pt(8.0), w: Pt(60.0), h: Pt(80.0) }));
        assert!(!draw_item_top(String::new(), &same, 128.0, &ctx).contains(" re W n"));
        assert!(draw_item_top(String::new(), &crop, 128.0, &ctx).contains(" re W n"));
        let mut local = String::new();
        draw_item_local(&mut local, &same, None, &ctx);
        assert!(!local.contains(" re W n"));
        local.clear();
        draw_item_local(&mut local, &crop, None, &ctx);
        assert!(local.contains(" re W n"));
    }

    #[test]
    fn exif_rotation_preserves_image_origin() {
        let matrix = image_exif_matrix(6, 30.0, 20.0, 7.0, 11.0);
        assert_eq!((matrix.a, matrix.b, matrix.c, matrix.d), (0.0, -20.0, 30.0, 0.0));
        assert_eq!((matrix.tx, matrix.ty), (7.0, 31.0));
    }
    // Interpret only the geometric operators emitted by these line/group fixtures.
    // Expected coordinates below are independent of TransformMatrix::concat/apply.
    struct ObservedGeometry {
        points: Vec<([f64; 2], bool)>,
        rectangles: Vec<[[f64; 2]; 4]>,
    }

    fn observed_geometry(stream: &str) -> ObservedGeometry {
        let mut matrix = [1.0, 0.0, 0.0, 1.0, 0.0, 0.0];
        let mut clipped = false;
        let mut stack = Vec::new();
        let mut args = Vec::<f64>::new();
        let mut points = Vec::new();
        let mut rectangles = Vec::new();
        let map = |m: [f64; 6], x: f64, y: f64| {
            [m[0] * x + m[2] * y + m[4], m[1] * x + m[3] * y + m[5]]
        };
        for token in stream.split_whitespace() {
            if let Ok(number) = token.parse::<f64>() {
                args.push(number);
                continue;
            }
            match token {
                "q" => stack.push((matrix, clipped)),
                "Q" => (matrix, clipped) = stack.pop().expect("balanced graphics state"),
                "cm" => {
                    assert_eq!(args.len(), 6);
                    let origin = map(matrix, args[4], args[5]);
                    matrix = [
                        matrix[0] * args[0] + matrix[2] * args[1],
                        matrix[1] * args[0] + matrix[3] * args[1],
                        matrix[0] * args[2] + matrix[2] * args[3],
                        matrix[1] * args[2] + matrix[3] * args[3],
                        origin[0],
                        origin[1],
                    ];
                }
                "m" | "l" => {
                    assert_eq!(args.len(), 2);
                    points.push((map(matrix, args[0], args[1]), clipped));
                }
                "re" => {
                    assert_eq!(args.len(), 4);
                    rectangles.push([
                        map(matrix, args[0], args[1]),
                        map(matrix, args[0] + args[2], args[1]),
                        map(matrix, args[0] + args[2], args[1] + args[3]),
                        map(matrix, args[0], args[1] + args[3]),
                    ]);
                }
                "W" => clipped = true,
                "w" | "S" | "n" => {}
                other => panic!("unexpected operator in geometry fixture: {other}"),
            }
            args.clear();
        }
        assert!(args.is_empty() && stack.is_empty());
        ObservedGeometry { points, rectangles }
    }

    fn geometry_line() -> FrameItem {
        FrameItem::Line {
            start: Point { x: Pt(1.0), y: Pt(2.0) },
            end: Point { x: Pt(4.0), y: Pt(6.0) },
            thickness: 1.0,
            color: None,
            cap: typst_core::entities::geometry::LineCap::Butt,
        }
    }

    #[test]
    fn nested_group_composes_position_and_affine_matrix_and_restores_siblings() {
        for mode in [StreamMode::Compact, StreamMode::Verbose] {
            let images = std::collections::HashMap::new();
            let patterns = std::collections::HashMap::new();
            let ctx = PageContext::type1(&images, &[], &patterns, &[], mode);
            let nested = FrameItem::Group {
                paint_root: false,
                pos: Point { x: Pt(7.0), y: Pt(11.0) },
                matrix: TransformMatrix {
                    a: 0.0,
                    b: 1.0,
                    c: -1.0,
                    d: 0.0,
                    tx: 3.0,
                    ty: 5.0,
                },
                clip_mask: None,
                inner_width: 40.0,
                inner_height: 30.0,
                items: vec![geometry_line()],
            };
            let outer = FrameItem::Group {
                paint_root: false,
                pos: Point { x: Pt(10.0), y: Pt(20.0) },
                matrix: TransformMatrix {
                    a: 2.0,
                    b: 0.0,
                    c: 0.0,
                    d: 3.0,
                    tx: 1.0,
                    ty: 2.0,
                },
                clip_mask: None,
                inner_width: 80.0,
                inner_height: 60.0,
                items: vec![nested, geometry_line()],
            };
            let stream = draw_item_top(String::new(), &outer, 200.0, &ctx);
            let stream = draw_item_top(stream, &geometry_line(), 200.0, &ctx);
            let ObservedGeometry { points, .. } = observed_geometry(&stream);
            assert_eq!(
                points,
                vec![
                    ([27.0, 127.0], false),
                    ([19.0, 118.0], false),
                    ([13.0, 172.0], false),
                    ([19.0, 160.0], false),
                    ([1.0, 198.0], false),
                    ([4.0, 194.0], false),
                ],
                "{stream}"
            );
        }
    }

    #[test]
    fn nested_group_clip_has_transformed_bounds_and_does_not_leak() {
        for mode in [StreamMode::Compact, StreamMode::Verbose] {
            let images = std::collections::HashMap::new();
            let patterns = std::collections::HashMap::new();
            let ctx = PageContext::type1(&images, &[], &patterns, &[], mode);
            let nested = FrameItem::Group {
                paint_root: false,
                pos: Point { x: Pt(7.0), y: Pt(11.0) },
                matrix: TransformMatrix::identity(),
                clip_mask: Some(typst_core::entities::geometry::ShapeKind::Rect),
                inner_width: 10.0,
                inner_height: 8.0,
                items: vec![geometry_line()],
            };
            let outer = FrameItem::Group {
                paint_root: false,
                pos: Point { x: Pt(10.0), y: Pt(20.0) },
                matrix: TransformMatrix::identity(),
                clip_mask: None,
                inner_width: 80.0,
                inner_height: 60.0,
                items: vec![nested, geometry_line()],
            };
            let stream = draw_item_top(String::new(), &outer, 200.0, &ctx);
            let ObservedGeometry { points, rectangles } = observed_geometry(&stream);
            assert_eq!(
                rectangles,
                vec![[[17.0, 169.0], [27.0, 169.0], [27.0, 161.0], [17.0, 161.0]]],
                "{stream}"
            );
            assert_eq!(
                points,
                vec![
                    ([18.0, 167.0], true),
                    ([21.0, 163.0], true),
                    ([11.0, 178.0], false),
                    ([14.0, 174.0], false),
                ],
                "{stream}"
            );
        }
    }
    // A PDF path is consumed by painting or n. Graphics state q/Q does not
    // consume it. Observe painted/clip geometry independently of the emitter.
    #[derive(Default)]
    struct ObservedRectPaths {
        painted: Vec<Vec<[f64; 4]>>,
        clipped: Vec<Vec<[f64; 4]>>,
    }

    fn observe_rect_paths(stream: &str) -> ObservedRectPaths {
        let mut observed = ObservedRectPaths::default();
        let mut path = Vec::new();
        let mut numbers = Vec::new();
        for token in stream.split_whitespace() {
            if let Ok(value) = token.parse::<f64>() {
                numbers.push(value);
                continue;
            }
            match token {
                "re" => path.push(numbers.as_slice().try_into().expect("rectangle")),
                "f" | "S" | "B" => observed.painted.push(std::mem::take(&mut path)),
                "W" => observed.clipped.push(path.clone()),
                "n" => path.clear(),
                "q" | "Q" | "cm" | "rg" | "RG" | "w" => {}
                other => panic!("unexpected rectangle-fixture operator: {other}"),
            }
            numbers.clear();
        }
        assert!(numbers.is_empty());
        observed
    }

    fn rectangle_fixture(x: f64, y: f64, painted: bool, stroked: bool) -> FrameItem {
        use typst_core::entities::{
            frame_paint::{FramePaint as Paint, FrameStroke as Stroke},
            geometry::ShapeKind,
            layout_types::Color,
        };
        FrameItem::Shape {
            pos: Point { x: Pt(x), y: Pt(y) },
            kind: ShapeKind::Rect,
            width: 12.0,
            height: 8.0,
            fill: painted.then_some(Paint::Solid(Color::rgb(0, 0, 255))),
            stroke: stroked.then(Stroke::default),
            parent_bbox_at_emit: None,
            fill_rule: Default::default(),
        }
    }

    fn assert_unpainted_path_isolation(local: bool, grouped: bool) {
        for mode in [StreamMode::Compact, StreamMode::Verbose] {
            for (filled, stroked) in [(true, false), (false, true), (true, true)] {
                let images = std::collections::HashMap::new();
                let patterns = std::collections::HashMap::new();
                let ctx = PageContext::type1(&images, &[], &patterns, &[], mode);
                let mut invisible = rectangle_fixture(10.0, 20.0, false, false);
                if grouped {
                    invisible = FrameItem::Group {
                        paint_root: false,
                        pos: Point { x: Pt(3.0), y: Pt(4.0) },
                        matrix: TransformMatrix::identity(),
                        clip_mask: None,
                        inner_width: 50.0,
                        inner_height: 40.0,
                        items: vec![invisible],
                    };
                }
                let visible = rectangle_fixture(60.0, 50.0, filled, stroked);
                let stream = if local {
                    let mut stream = String::new();
                    draw_item_local(&mut stream, &invisible, None, &ctx);
                    draw_item_local(&mut stream, &visible, None, &ctx);
                    stream
                } else {
                    let stream = draw_item_top(String::new(), &invisible, 100.0, &ctx);
                    draw_item_top(stream, &visible, 100.0, &ctx)
                };
                let observed = observe_rect_paths(&stream);
                assert_eq!(
                    observed.painted,
                    vec![vec![[60.0, if local { 50.0 } else { 42.0 }, 12.0, 8.0]]],
                    "invisible geometry entered later painting: {stream}"
                );
                assert!(observed.clipped.is_empty());
            }
        }
    }

    #[test]
    fn unpainted_path_does_not_join_later_top_level_paint() {
        assert_unpainted_path_isolation(false, false);
    }

    #[test]
    fn unpainted_path_does_not_join_later_local_paint() {
        assert_unpainted_path_isolation(true, false);
    }

    #[test]
    fn unpainted_path_does_not_escape_its_group_into_sibling_paint() {
        assert_unpainted_path_isolation(false, true);
        assert_unpainted_path_isolation(true, true);
    }

    fn assert_unpainted_path_does_not_expand_clip(local: bool) {
        for mode in [StreamMode::Compact, StreamMode::Verbose] {
            let images = std::collections::HashMap::new();
            let patterns = std::collections::HashMap::new();
            let ctx = PageContext::type1(&images, &[], &patterns, &[], mode);
            let invisible = rectangle_fixture(30.0, 40.0, false, false);
            let group = FrameItem::Group {
                paint_root: false,
                pos: Point { x: Pt(3.0), y: Pt(4.0) },
                matrix: TransformMatrix::identity(),
                clip_mask: Some(typst_core::entities::geometry::ShapeKind::Rect),
                inner_width: 6.0,
                inner_height: 5.0,
                items: vec![rectangle_fixture(0.0, 0.0, true, false)],
            };
            let stream = if local {
                let mut stream = String::new();
                draw_item_local(&mut stream, &invisible, None, &ctx);
                draw_item_local(&mut stream, &group, None, &ctx);
                stream
            } else {
                let stream = draw_item_top(String::new(), &invisible, 100.0, &ctx);
                draw_item_top(stream, &group, 100.0, &ctx)
            };
            let observed = observe_rect_paths(&stream);
            assert_eq!(
                observed.clipped,
                vec![vec![[0.0, 0.0, 6.0, 5.0]]],
                "invisible geometry expanded the next clip: {stream}"
            );
            assert_eq!(observed.painted, vec![vec![[0.0, 0.0, 12.0, 8.0]]]);
        }
    }

    #[test]
    fn unpainted_path_does_not_expand_next_top_level_clip() {
        assert_unpainted_path_does_not_expand_clip(false);
    }

    #[test]
    fn unpainted_path_does_not_expand_next_local_clip() {
        assert_unpainted_path_does_not_expand_clip(true);
    }
    #[test]
    fn solid_alpha_is_scoped_across_text_replacement_and_next_leaf() {
        use typst_core::entities::layout_types::{Color, TextStyle};
        let map = std::collections::HashMap::new();
        for mode in [StreamMode::Compact, StreamMode::Verbose] {
            let patterns_map = std::collections::HashMap::new();
            let ctx = PageContext::type1(&map, &[], &patterns_map, &[], mode);
            let translucent = FrameItem::Line {
                start: Point::ZERO,
                end: Point { x: Pt(10.0), y: Pt(0.0) },
                thickness: 1.0,
                color: Some(Color::srgb_f32(1.0, 0.0, 0.0, 0.5)),
                cap: typst_core::entities::geometry::LineCap::Butt,
            };
            let text = FrameItem::Text {
                pos: Point { x: Pt(3.0), y: Pt(12.0) },
                text: "A".into(),
                style: TextStyle {
                    fill: Some(Color::srgb_f32(0.0, 0.0, 1.0, 0.25)),
                    ..Default::default()
                },
                bidi: None,
            };
            let replacement =
                FrameItem::TextReplacement { text: "A".into(), items: vec![text] };
            let mut opaque = translucent.clone();
            if let FrameItem::Line { color, .. } = &mut opaque {
                *color = None;
            }
            for local in [false, true] {
                let mut stream = String::new();
                for item in [&translucent, &replacement, &opaque] {
                    if local {
                        draw_item_local(&mut stream, item, None, &ctx);
                    } else {
                        stream = draw_item_top(stream, item, 40.0, &ctx);
                    }
                }
                assert!(stream.contains(" gs"), "alpha must be emitted");
                let mut state = (1.0f32, 1.0f32);
                let mut stack = Vec::new();
                let mut last = "";
                let mut strokes = Vec::new();
                let mut texts = Vec::new();
                for token in stream.split_whitespace() {
                    match token {
                        "q" => stack.push(state),
                        "Q" => state = stack.pop().expect("balanced graphics state"),
                        "gs" => {
                            let hex = last.strip_prefix("/A").unwrap();
                            state = (
                                f32::from_bits(
                                    u32::from_str_radix(&hex[..8], 16).unwrap(),
                                ),
                                f32::from_bits(
                                    u32::from_str_radix(&hex[8..], 16).unwrap(),
                                ),
                            );
                        }
                        "S" => strokes.push(state.1),
                        "Tj" => texts.push(state.0),
                        _ => {}
                    }
                    last = token;
                }
                assert!(
                    stack.is_empty(),
                    "alpha and ActualText scopes balanced: {stream}"
                );
                assert_eq!(state, (1.0, 1.0));
                assert_eq!(
                    strokes,
                    vec![0.5, 1.0],
                    "transparent stroke must not affect following leaf"
                );
                assert_eq!(texts, vec![0.25]);
                assert!(stream.rfind("EMC").unwrap() < stream.rfind("Q").unwrap());
            }
        }
    }
}
