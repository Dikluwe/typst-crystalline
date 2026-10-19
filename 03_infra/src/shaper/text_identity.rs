//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/shaper/text_identity.md
//! @prompt-hash e94f1db2
//! @layer L3
//! @updated 2026-09-28

use typst_core::entities::layout_types::FrameItem;

/// Authored identity belongs to an occurrence, not a shared font glyph.
pub(super) fn protect_authored_identity(items: Vec<FrameItem>) -> Vec<FrameItem> {
    items
        .into_iter()
        .map(|item| {
            if let FrameItem::TextShaped { text, glyphs, .. } = &item {
                if text.contains('\u{ad}') || has_mixed_bidi_cluster(text, glyphs) {
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

/// Certify a control only from a complete emitted UTF-8 cluster range.
fn has_mixed_bidi_cluster(
    text: &str,
    glyphs: &[typst_core::entities::layout_types::ShapedGlyph],
) -> bool {
    let is_bidi = |c| matches!(c, '\u{61c}' | '\u{200e}' | '\u{200f}' | '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}');
    // Avoid constructing boundaries for the ordinary text path.
    if !text.chars().any(is_bidi) {
        return false;
    }
    let mut boundaries: Vec<usize> = glyphs.iter().map(|g| g.cluster as usize).collect();
    boundaries.push(text.len());
    boundaries.sort_unstable();
    boundaries.dedup();
    boundaries.windows(2).any(|range| {
        text.get(range[0]..range[1]).is_some_and(|cluster| {
            cluster.chars().any(is_bidi) && cluster.chars().any(|c| !is_bidi(c))
        })
    })
}

/// Remove invisible placeholders while retaining the complete authored input.
pub(super) fn configure_authored_ignorables_buffer(
    buffer: &mut rustybuzz::UnicodeBuffer,
    text: &str,
) {
    if text.chars().any(|c| matches!(c, '\u{ad}' | '\u{61c}' | '\u{200e}' | '\u{200f}' | '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}')) {
        buffer.set_flags(
            buffer.flags() | rustybuzz::BufferFlags::REMOVE_DEFAULT_IGNORABLES,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shaped(text: &str, clusters: &[u32]) -> FrameItem {
        use typst_core::entities::layout_types::{Point, ShapedGlyph, TextStyle};
        FrameItem::TextShaped {
            pos: Point::ZERO,
            text: text.into(),
            style: TextStyle::default(),
            units_per_em: 1000,
            bidi: None,
            font: None,
            glyphs: clusters
                .iter()
                .map(|&cluster| ShapedGlyph {
                    glyph_id: 36,
                    cluster,
                    x_advance: 500,
                    x_offset: 0,
                    y_offset: 0,
                    char_code: 'A',
                })
                .collect(),
        }
    }

    #[test]
    fn authored_bidi_identity_requires_a_mixed_emitted_cluster() {
        for control in ['\u{200e}', '\u{2069}'] {
            let text = format!("{control}A A");
            let original = shaped(&text, &[0, 4, 5]);
            let child = format!("{original:?}");
            let result = protect_authored_identity(vec![original]);
            let FrameItem::TextReplacement { text: identity, items } = &result[0] else {
                panic!("missing occurrence identity")
            };
            assert_eq!(identity.as_str(), text);
            assert_eq!(items.len(), 1);
            assert_eq!(format!("{:?}", items[0]), child);
            // A control outside the emitted range is not a mixed-cluster certificate.
            assert!(matches!(
                &protect_authored_identity(vec![shaped(&text, &[3, 4, 5])])[0],
                FrameItem::TextShaped { .. }
            ));
        }
        for item in [
            shaped("ABC", &[0, 1, 2]),
            shaped("\u{200e}", &[0]),
            shaped("\u{200e}A", &[]),
            shaped("\u{200e}A", &[1]),
            shaped("a\u{200d}", &[0]),
        ] {
            assert!(matches!(
                &protect_authored_identity(vec![item])[0],
                FrameItem::TextShaped { .. }
            ));
        }
    }

    #[test]
    fn authored_bidi_removes_placeholder_only_when_requested() {
        for control in [
            '\u{61c}', '\u{200e}', '\u{200f}', '\u{202a}', '\u{202b}', '\u{202c}',
            '\u{202d}', '\u{202e}', '\u{2066}', '\u{2067}', '\u{2068}', '\u{2069}',
        ] {
            let mut buffer = rustybuzz::UnicodeBuffer::new();
            buffer.set_flags(rustybuzz::BufferFlags::BEGINNING_OF_TEXT);
            configure_authored_ignorables_buffer(&mut buffer, &format!("A{control} B"));
            assert!(buffer
                .flags()
                .contains(rustybuzz::BufferFlags::REMOVE_DEFAULT_IGNORABLES));
            assert!(buffer.flags().contains(rustybuzz::BufferFlags::BEGINNING_OF_TEXT));
        }
        for text in ["ABC", "a\u{200d}", "a\u{200c}", "a\u{fe0f}"] {
            let mut buffer = rustybuzz::UnicodeBuffer::new();
            configure_authored_ignorables_buffer(&mut buffer, text);
            assert!(!buffer
                .flags()
                .contains(rustybuzz::BufferFlags::REMOVE_DEFAULT_IGNORABLES));
        }
    }
}
