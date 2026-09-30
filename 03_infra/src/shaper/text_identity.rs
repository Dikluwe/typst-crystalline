//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/shaper/text_identity.md
//! @prompt-hash 39ca5905
//! @layer L3
//! @updated 2026-09-28

use typst_core::entities::layout_types::FrameItem;

/// Authored U+00AD belongs to an occurrence, not a shared font glyph.
pub(super) fn protect_authored_shy(items: Vec<FrameItem>) -> Vec<FrameItem> {
    items
        .into_iter()
        .map(|item| {
            if let FrameItem::TextShaped { text, .. } = &item {
                if text.contains('\u{ad}') {
                    return FrameItem::TextReplacement {
                        text: text.clone(),
                        items: vec![item],
                    };
                }
            }
            item
        })
        .collect()
}

/// Remove invisible placeholders while retaining the complete authored input.
pub(super) fn configure_authored_shy_buffer(
    buffer: &mut rustybuzz::UnicodeBuffer,
    text: &str,
) {
    if text.contains('\u{ad}') {
        buffer.set_flags(
            buffer.flags() | rustybuzz::BufferFlags::REMOVE_DEFAULT_IGNORABLES,
        );
    }
}
