//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/shaper/traversal.md
//! @prompt-hash 0894668d
//! @layer L3
//! @updated 2026-11-06

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
            shape_item_into(
                world,
                item,
                cache,
                face_cache,
                false,
                Some(resolved),
                &mut shaped,
            );
        }
        *items = shaped;
    }
    shape_layer(world, &mut page.background, cache, face_cache, resolved);
    shape_layer(world, &mut page.items, cache, face_cache, resolved);
    shape_layer(world, &mut page.foreground, cache, face_cache, resolved);
}

/// Adaptador de teste que coleta os itens emitidos pelo walker produtivo.
/// Um `Text` pode gerar zero, um ou vários itens consecutivos.
#[cfg(test)]
pub(super) fn shape_item(
    world: &dyn World,
    item: FrameItem,
    cache: &mut ShapeCache,
    face_cache: &mut FaceCache,
) -> Vec<FrameItem> {
    let mut output = Vec::new();
    shape_item_into(world, item, cache, face_cache, false, None, &mut output);
    output
}

/// Consome um item e acrescenta seus resultados em ordem ao destino do caller.
/// A capacidade de extensão não permite consultar ou alterar o prefixo aceito.
/// Contêineres reconstroem seus próprios filhos antes de emitir o wrapper.
fn shape_item_into(
    world: &dyn World,
    mut item: FrameItem,
    cache: &mut ShapeCache,
    face_cache: &mut FaceCache,
    inside_replacement: bool,
    resolved: Option<&crate::bidi_context::Resolved<'_>>,
    output: &mut impl Extend<FrameItem>,
) {
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
                if inside_replacement {
                    output.extend(shaped);
                } else {
                    output
                        .extend(super::text_identity::protect_authored_identity(shaped));
                }
                // Some(empty) também conclui a transação, sem fallback.
                return;
            }
        }
        FrameItem::Group { items, .. } => {
            let mut new_children = Vec::with_capacity(items.len());
            for child in items.drain(..) {
                shape_item_into(
                    world,
                    child,
                    cache,
                    face_cache,
                    inside_replacement,
                    resolved,
                    &mut new_children,
                );
            }
            *items = new_children;
        }
        FrameItem::Link { items, .. } => {
            let mut new_children = Vec::with_capacity(items.len());
            for child in items.drain(..) {
                shape_item_into(
                    world,
                    child,
                    cache,
                    face_cache,
                    inside_replacement,
                    resolved,
                    &mut new_children,
                );
            }
            *items = new_children;
        }
        FrameItem::TextReplacement { items, .. } => {
            let mut new_children = Vec::with_capacity(items.len());
            for child in items.drain(..) {
                shape_item_into(
                    world,
                    child,
                    cache,
                    face_cache,
                    true,
                    resolved,
                    &mut new_children,
                );
            }
            *items = new_children;
        }
        FrameItem::Semantic { items, .. } => {
            let mut new_children = Vec::with_capacity(items.len());
            for child in items.drain(..) {
                shape_item_into(
                    world,
                    child,
                    cache,
                    face_cache,
                    inside_replacement,
                    resolved,
                    &mut new_children,
                );
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
                        shape_item_into(
                            world,
                            child,
                            cache,
                            face_cache,
                            false,
                            None,
                            &mut shaped,
                        );
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
    output.extend(std::iter::once(item));
}
