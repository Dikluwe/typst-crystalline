//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/shaper/traversal.md
//! @prompt-hash b8e082d0
//! @layer L3
//! @updated 2026-09-28

use super::cache::{FaceCache, ShapeCache};
use super::shaping::try_shape_context;
use typst_core::contracts::world::World;
use typst_core::entities::layout_types::{FrameItem, Page, PagedDocument};

/// Converte todos os `FrameItem::Text` de um `PagedDocument` em
/// `FrameItem::TextShaped` via rustybuzz.
///
/// Itens sem fonte resolvida (`style.font == None` ou lookup falha)
/// são preservados como `FrameItem::Text` (fallback Helvetica).
pub fn shape_document(world: &dyn World, doc: PagedDocument) -> PagedDocument {
    let registry = crate::bidi_context::Registry::new(&doc);
    let resolved = registry.resolve();
    shape_document_with_context(world, doc, &resolved)
}
pub(crate) fn shape_document_with_context(
    world: &dyn World,
    mut doc: PagedDocument,
    resolved: &crate::bidi_context::Resolved<'_>,
) -> PagedDocument {
    // P657 — cache local de resultados de shaping. Criada por documento para
    // evitar invalidação complexa entre mundos; reaproveita sub-runs idênticos
    // (mesmo texto, face, direção, variações e tracking) dentro do mesmo
    // documento, o que é comum em documentos com conteúdo repetido.
    let mut cache = ShapeCache::new();
    // P672 — cache local de faces ttf-parser. Evita re-parsear a mesma fonte
    // milhares de vezes durante a segmentação por fonte.
    let mut face_cache = FaceCache::new();

    for page in &mut doc.pages {
        shape_page(world, page, &mut cache, &mut face_cache, resolved);
    }

    doc
}

fn shape_page(
    world: &dyn World,
    page: &mut Page,
    cache: &mut ShapeCache,
    face_cache: &mut FaceCache,
    resolved: &crate::bidi_context::Resolved<'_>,
) {
    fn shape_layer(
        world: &dyn World,
        items: &mut Vec<FrameItem>,
        cache: &mut ShapeCache,
        face_cache: &mut FaceCache,
        resolved: &crate::bidi_context::Resolved<'_>,
    ) {
        let mut shaped = Vec::with_capacity(items.len());
        for item in items.drain(..) {
            shaped.extend(shape_item_inner(
                world,
                item,
                cache,
                face_cache,
                false,
                Some(resolved),
            ));
        }
        *items = shaped;
    }
    shape_layer(world, &mut page.background, cache, face_cache, resolved);
    shape_layer(world, &mut page.items, cache, face_cache, resolved);
    shape_layer(world, &mut page.foreground, cache, face_cache, resolved);
}

/// Processa um `FrameItem`, devolvendo 1 ou mais itens (fallback por
/// caractere pode expandir um `Text` em vários `TextShaped` consecutivos).
#[cfg(test)]
pub(super) fn shape_item(
    world: &dyn World,
    item: FrameItem,
    cache: &mut ShapeCache,
    face_cache: &mut FaceCache,
) -> Vec<FrameItem> {
    shape_item_inner(world, item, cache, face_cache, false, None)
}

fn shape_item_inner(
    world: &dyn World,
    mut item: FrameItem,
    cache: &mut ShapeCache,
    face_cache: &mut FaceCache,
    inside_replacement: bool,
    resolved: Option<&crate::bidi_context::Resolved<'_>>,
) -> Vec<FrameItem> {
    match &mut item {
        FrameItem::Text { pos, text, style, bidi } if style.font.is_some() => {
            if let Some(shaped) = try_shape_context(
                world,
                pos,
                text,
                style,
                cache,
                face_cache,
                bidi.as_ref(),
                resolved,
            ) {
                return if inside_replacement {
                    shaped
                } else {
                    super::text_identity::protect_authored_identity(shaped)
                };
            }
        }
        FrameItem::Group { items, .. } => {
            let mut new_children = Vec::with_capacity(items.len());
            for child in items.drain(..) {
                new_children.extend(shape_item_inner(
                    world,
                    child,
                    cache,
                    face_cache,
                    inside_replacement,
                    resolved,
                ));
            }
            *items = new_children;
        }
        FrameItem::Link { items, .. } => {
            let mut new_children = Vec::with_capacity(items.len());
            for child in items.drain(..) {
                new_children.extend(shape_item_inner(
                    world,
                    child,
                    cache,
                    face_cache,
                    inside_replacement,
                    resolved,
                ));
            }
            *items = new_children;
        }
        FrameItem::TextReplacement { items, .. } => {
            let mut new_children = Vec::with_capacity(items.len());
            for child in items.drain(..) {
                new_children.extend(shape_item_inner(
                    world, child, cache, face_cache, true, resolved,
                ));
            }
            *items = new_children;
        }
        FrameItem::Semantic { items, .. } => {
            let mut new_children = Vec::with_capacity(items.len());
            for child in items.drain(..) {
                new_children.extend(shape_item_inner(
                    world,
                    child,
                    cache,
                    face_cache,
                    inside_replacement,
                    resolved,
                ));
            }
            *items = new_children;
        }
        FrameItem::Shape { fill, stroke, .. } => {
            for paint in fill.iter_mut().chain(stroke.iter_mut().map(|s| &mut s.paint)) {
                if let typst_core::entities::frame_paint::FramePaint::Tiling(cell) = paint
                {
                    let cell = std::sync::Arc::make_mut(cell);
                    let mut shaped = Vec::new();
                    for child in cell.frame.items.drain(..) {
                        shaped.extend(shape_item_inner(
                            world, child, cache, face_cache, false, None,
                        ));
                    }
                    cell.frame.items = shaped;
                }
            }
        }
        // intencional: nós já shaped, linhas, glifos, imagens, formas e Text sem fonte resolvida
        FrameItem::TextSeparator { .. }
        | FrameItem::Text { .. }
        | FrameItem::TextShaped { .. }
        | FrameItem::Line { .. }
        | FrameItem::Glyph { .. }
        | FrameItem::Image { .. } => {}
    }
    vec![item]
}
