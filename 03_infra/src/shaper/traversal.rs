//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/shaper/traversal.md
//! @prompt-hash b34edc3d
//! @layer L3
//! @updated 2026-09-26

use super::cache::{FaceCache, ShapeCache};
use super::shaping::try_shape;
use typst_core::contracts::world::World;
use typst_core::entities::layout_types::{FrameItem, Page, PagedDocument};

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

fn shape_page(
    world: &dyn World,
    page: &mut Page,
    cache: &mut ShapeCache,
    face_cache: &mut FaceCache,
) {
    fn shape_layer(
        world: &dyn World,
        items: &mut Vec<FrameItem>,
        cache: &mut ShapeCache,
        face_cache: &mut FaceCache,
    ) {
        let mut shaped = Vec::with_capacity(items.len());
        for item in items.drain(..) {
            shaped.extend(shape_item(world, item, cache, face_cache));
        }
        *items = shaped;
    }
    shape_layer(world, &mut page.background, cache, face_cache);
    shape_layer(world, &mut page.items, cache, face_cache);
    shape_layer(world, &mut page.foreground, cache, face_cache);
}

/// Processa um `FrameItem`, devolvendo 1 ou mais itens (fallback por
/// caractere pode expandir um `Text` em vários `TextShaped` consecutivos).
pub(super) fn shape_item(
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
        FrameItem::Semantic { items, .. } => {
            let mut new_children = Vec::with_capacity(items.len());
            for child in items.drain(..) {
                new_children.extend(shape_item(world, child, cache, face_cache));
            }
            *items = new_children;
        }
        // intencional: nós já shaped, linhas, glifos, imagens, formas e Text sem fonte resolvida
        FrameItem::Text { .. }
        | FrameItem::TextShaped { .. }
        | FrameItem::Line { .. }
        | FrameItem::Glyph { .. }
        | FrameItem::Image { .. }
        | FrameItem::Shape { .. } => {}
    }
    vec![item]
}
