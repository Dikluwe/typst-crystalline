//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/word_segment.md
//! @prompt-hash 12e336f6
//! @layer L1
//! @updated 2026-09-25
//!
//! ICU segmentation and CJK tailoring before laying out chunks in Cursor.

use super::metrics::FontMetrics;
use crate::entities::image_sizer::ImageSizer;
use icu_segmenter::{options::LineBreakOptions, LineSegmenter, LineSegmenterBorrowed};
use unicode_script::{Script, UnicodeScript};

// ── P756 — line segmentation for scripts without spaces ────────────────────

/// Returns an LSTM line segmenter configured with compiled data.
///
/// `icu_segmenter` with `compiled_data` embeds Unicode tables and the LSTM
/// model as static constants at compile time; there is no runtime I/O.
/// It can therefore reside in L1. The segmenter is reconstructed on each
/// invocation to avoid mutable global state (V13).
fn line_segmenter() -> LineSegmenterBorrowed<'static> {
    LineSegmenter::new_lstm(LineBreakOptions::default())
}

/// Checks whether a word contains characters from scripts without spaces
/// that require line segmentation (CJK, Thai, Lao, Myanmar, Khmer).
pub(super) fn word_needs_line_segmentation(word: &str) -> bool {
    word.chars().any(|c| {
        matches!(
            c.script(),
            Script::Han
                | Script::Hiragana
                | Script::Katakana
                | Script::Thai
                | Script::Lao
                | Script::Myanmar
                | Script::Khmer
        )
    })
}

/// Checks whether the text (or active language) calls for Chinese/Japanese
/// quotation-mark tailoring: `U+201C` (`“`) does not start a line and
/// `U+201D` (`”`) does not end a line.
fn needs_cjk_quote_tailoring(
    word: &str,
    lang: Option<&crate::entities::lang::Lang>,
) -> bool {
    let is_cjk_lang = lang.is_some_and(|l| {
        let s = l.as_str();
        s == "zh" || s == "ja"
    });
    let has_cjk_script = word.chars().any(is_cjk_context_char);
    is_cjk_lang || has_cjk_script
}

/// Checks whether a character belongs to a CJK context: ideographs,
/// hiragana, katakana, hangul, or common CJK punctuation/forms.
fn is_cjk_context_char(c: char) -> bool {
    matches!(
        c.script(),
        Script::Han
            | Script::Hiragana
            | Script::Katakana
            | Script::Hangul
            | Script::Bopomofo
    ) || matches!(c as u32,
        0x3000..=0x303F      // CJK Symbols and Punctuation
        | 0xFF00..=0xFFEF    // Halfwidth and Fullwidth Forms
        | 0xFE10..=0xFE1F    // Vertical Forms
        | 0xFE30..=0xFE4F    // CJK Compatibility Forms
    )
}

/// Removes break opportunities that would place `“` at the start of a line
/// or `”` at the end, reproducing the effect of vanilla's `CJ_SEGMENTER`.
fn filter_cjk_quote_breakpoints(text: &str, mut breakpoints: Vec<usize>) -> Vec<usize> {
    const OPEN: char = '\u{201C}';
    const CLOSE: char = '\u{201D}';

    let chars: Vec<char> = text.chars().collect();
    let mut byte_to_char = vec![0usize; text.len() + 1];
    let mut byte_pos = 0;
    for (i, c) in chars.iter().enumerate() {
        byte_to_char[byte_pos] = i;
        byte_pos += c.len_utf8();
    }
    byte_to_char[byte_pos] = chars.len();

    breakpoints.retain(|&bp| {
        let idx = byte_to_char[bp];
        // Do not break before an opening quotation mark.
        if idx < chars.len() && chars[idx] == OPEN {
            return false;
        }
        // Do not break after a closing quotation mark.
        if idx > 0 && chars[idx - 1] == CLOSE {
            return false;
        }
        true
    });
    breakpoints
}

/// Splits a word at the segmenter's breakpoints and emits each fragment
/// through `layout_chunk`. Used when `layout_word` detects a run from a
/// script without spaces.
pub(super) fn layout_segmented_word<M: FontMetrics, S: ImageSizer>(
    layouter: &mut super::Layouter<'_, M, S>,
    word: &str,
) {
    let mut breakpoints: Vec<usize> = line_segmenter().segment_str(word).collect();

    if needs_cjk_quote_tailoring(word, layouter.style.lang.as_ref()) {
        breakpoints = filter_cjk_quote_breakpoints(word, breakpoints);
    }

    for w in breakpoints.windows(2) {
        let fragment = &word[w[0]..w[1]];
        if !fragment.is_empty() {
            layouter.layout_chunk(fragment);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::filter_cjk_quote_breakpoints;

    #[test]
    fn p1504_aspas_cjk_nao_ficam_na_borda_errada() {
        let text = "漢“字”漢";
        assert_eq!(
            filter_cjk_quote_breakpoints(text, vec![0, 3, 6, 9, 12, 15]),
            vec![0, 6, 9, 15]
        );
    }
}
