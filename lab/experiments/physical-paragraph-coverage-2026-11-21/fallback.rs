
#[inline(never)]
fn measure_paragraph_ascii(face: &Face<'_>) -> u128 {
    (0_u8..=127).fold(0, |mask, byte| {
        if face.glyph_index(char::from(byte)).is_some_and(|glyph| glyph.0 != 0) {
            mask | (1_u128 << byte)
        } else {
            mask
        }
    })
}

fn paragraph_has_coverage(
    session: Option<&crate::shaper::ShapeSession>,
    font: &typst_core::entities::world_types::Font,
    face: &Face<'_>,
    text: &str,
) -> bool {
    use typst_core::entities::world_types::Font;
    if let (Some(session), Font::Vec(owner)) = (session, font) {
        if text.is_ascii() {
            let mask = session.paragraph_ascii(owner, || measure_paragraph_ascii(face));
            return text.bytes().all(|byte| mask & (1_u128 << byte) != 0);
        }
    }
    text.chars().all(|c| face.glyph_index(c).is_some_and(|glyph| glyph.0 != 0))
}

#[cfg(test)]
mod paragraph_ascii_coverage_tests {
    use super::*;
    use typst_core::entities::world_types::Font;

    #[test]
    fn paragraph_ascii_matches_positive_negative_and_unicode_cmap() {
        let font = Font::from_data(
            include_bytes!("../../fixtures/fonts/NimbusSans-Regular.otf").to_vec(),
        );
        let face = Face::parse(font.as_slice(), 0).unwrap();
        let session = crate::shaper::ShapeSession::new();
        for byte in 0..=127_u8 {
            let text = char::from(byte).to_string();
            assert_eq!(
                paragraph_has_coverage(Some(&session), &font, &face, &text),
                paragraph_has_coverage(None, &font, &face, &text),
                "ASCII byte {byte}",
            );
        }
        for text in ["words -", "á ß", "\0", "\u{10ffff}"] {
            assert_eq!(
                paragraph_has_coverage(Some(&session), &font, &face, text),
                paragraph_has_coverage(None, &font, &face, text),
            );
        }
        assert!(paragraph_has_coverage(Some(&session), &font, &face, "words -"));
        assert!(!paragraph_has_coverage(Some(&session), &font, &face, "\0"));
    }
}
