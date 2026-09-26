//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/word_segment.md
//! @prompt-hash 86037726
//! @layer L1
//! @updated 2026-09-25
//!
//! Segmentação ICU e tailoring CJK antes de dispor chunks no Cursor.

use super::metrics::FontMetrics;
use crate::entities::image_sizer::ImageSizer;
use icu_segmenter::{options::LineBreakOptions, LineSegmenter, LineSegmenterBorrowed};
use unicode_script::{Script, UnicodeScript};

// ── P756 — segmentação de linha para scripts sem espaços ────────────────────

/// Devolve um segmentador de linha LSTM configurado com os dados compilados.
///
/// `icu_segmenter` com `compiled_data` embute as tabelas Unicode e o modelo
/// LSTM como constantes estáticas em tempo de compilação; não há I/O em
/// runtime. Por isso pode residir em L1. O segmentador é reconstruído a
/// cada invocação para evitar estado global mutável (V13).
fn line_segmenter() -> LineSegmenterBorrowed<'static> {
    LineSegmenter::new_lstm(LineBreakOptions::default())
}

/// Verifica se uma palavra contém caracteres de scripts sem espaços que
/// exigem segmentação de linha (CJK, Thai, Lao, Myanmar, Khmer).
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

/// Verifica se o texto (ou a linguagem activa) justifica o tailoring de
/// aspas para chinês/japonês: `U+201C` (`“`) não inicia linha e `U+201D`
/// (`”`) não termina linha.
fn needs_cjk_quote_tailoring(
    word: &str,
    lang: Option<&crate::entities::lang::Lang>,
) -> bool {
    let is_cjk_lang = lang.map_or(false, |l| {
        let s = l.as_str();
        s == "zh" || s == "ja"
    });
    let has_cjk_script = word.chars().any(is_cjk_context_char);
    is_cjk_lang || has_cjk_script
}

/// Verifica se um caractere pertence a um contexto CJK: ideogramas,
/// hiragana, katakana, hangul ou pontuação/formas CJK comuns.
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

/// Remove oportunidades de quebra que colocariam `“` no início de linha ou
/// `”` no fim de linha, replicando o efeito do `CJ_SEGMENTER` do vanilla.
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
        // Não quebrar antes de aspas de abertura.
        if idx < chars.len() && chars[idx] == OPEN {
            return false;
        }
        // Não quebrar depois de aspas de fecho.
        if idx > 0 && chars[idx - 1] == CLOSE {
            return false;
        }
        true
    });
    breakpoints
}

/// Fragmenta uma palavra nos breakpoints do segmentador e emite cada
/// fragmento via `layout_chunk`. Usado quando `layout_word` detecta um run
/// de script sem espaços.
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
