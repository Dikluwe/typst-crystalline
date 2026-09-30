//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/rules/textual/space.md
//! @prompt-hash 85bebe40
//! @layer L1
//! @updated 2026-09-30
//!
//! Normalização de espaços no fluxo textual realizado.

use crate::entities::content::Content;

fn space_origin(content: &Content) -> Option<bool> {
    match content {
        Content::Space => Some(false),
        Content::SpaceFromNewline => Some(true),
        Content::Styled(styled) => space_origin(&styled.body),
        _ => None,
    }
}

fn space_with_origin(original: &Content, had_newline: bool) -> Content {
    match original {
        Content::Styled(styled) => Content::styled(
            space_with_origin(&styled.body, had_newline),
            styled.styles.clone(),
        ),
        _ if had_newline => Content::SpaceFromNewline,
        _ => Content::Space,
    }
}

fn previous_visible(parts: &[Content], before: usize) -> Option<usize> {
    (0..before)
        .rev()
        .find(|&index| !matches!(&parts[index], Content::Empty))
}

fn next_visible(parts: &[Content], from: usize) -> Option<usize> {
    (from..parts.len()).find(|&index| !matches!(&parts[index], Content::Empty))
}

/// Texto de um wrapper de estilo pode informar o descarte do espaço exterior,
/// embora o wrapper continue a interromper matching literal/regex.
fn supportive_text(content: &Content) -> String {
    match content {
        Content::Text(text) => text.to_string(),
        Content::Styled(..)
        | Content::Strong(_)
        | Content::Emph(_)
        | Content::Underline(_)
        | Content::Strike(_)
        | Content::Overline(_)
        | Content::SmallCaps { .. } => content.plain_text(),
        _ => String::new(),
    }
}

pub(super) fn normalize_markup_spaces(parts: &mut [Content]) {
    let mut start = 0;
    while start < parts.len() {
        let Some(mut had_newline) = space_origin(&parts[start]) else {
            start += 1;
            continue;
        };
        let mut end = start + 1;
        while end < parts.len() {
            match space_origin(&parts[end]) {
                Some(origin) => had_newline |= origin,
                None if matches!(&parts[end], Content::Empty) => {}
                None => break,
            }
            end += 1;
        }

        let before = previous_visible(parts, start);
        let after = next_visible(parts, end);
        let destructive = before
            .is_some_and(|index| matches!(&parts[index], Content::Linebreak(_)))
            || after.is_some_and(|index| matches!(&parts[index], Content::Linebreak(_)));
        // Não decidir uma borda ainda sem vizinhos: [newline] em #let deve
        // conservar a origem até ser inserido noutro corpo de markup.
        let discard = had_newline
            && before.is_some()
            && after.is_some()
            && discard_space_between(
                &supportive_text(&parts[before.unwrap()]),
                &supportive_text(&parts[after.unwrap()]),
            );
        if destructive || discard {
            parts[start] = Content::Empty;
        } else {
            let normalized = space_with_origin(&parts[start], had_newline);
            parts[start] = normalized;
        }
        for part in &mut parts[start + 1..end] {
            if space_origin(part).is_some() {
                *part = Content::Empty;
            }
        }
        start = end;
    }
}

/// Predicado Unicode puro equivalente à superfície do pin vanilla:
/// Han/Bopomofo/Hiragana/Katakana/Yi e pontuação ideográfica não recebem
/// espaços inferidos; coreano, aspas ambíguas e emoji conservam-nos.
fn discard_space_between(left: &str, right: &str) -> bool {
    use icu_properties::props::{EastAsianWidth, Emoji, Script};
    use icu_properties::{CodePointMapDataBorrowed, CodePointSetDataBorrowed};

    fn spacingless(c: char) -> bool {
        const SCRIPTS: CodePointMapDataBorrowed<Script> = CodePointMapDataBorrowed::new();
        const WIDTHS: CodePointMapDataBorrowed<EastAsianWidth> =
            CodePointMapDataBorrowed::new();
        const EMOJI: CodePointSetDataBorrowed = CodePointSetDataBorrowed::new::<Emoji>();

        match SCRIPTS.get(c) {
            Script::Han
            | Script::Bopomofo
            | Script::Hiragana
            | Script::Katakana
            | Script::Yi => true,
            Script::Inherited if matches!(c, '\u{302A}'..='\u{302D}' | '\u{3099}'..='\u{309A}') => {
                true
            }
            Script::Common => match WIDTHS.get(c) {
                EastAsianWidth::Fullwidth => true,
                EastAsianWidth::Halfwidth if c != '₩' => true,
                EastAsianWidth::Wide if !EMOJI.contains(c) => true,
                EastAsianWidth::Ambiguous if matches!(c, '㉈'..='㉏') => true,
                _ => false,
            },
            _ => false,
        }
    }

    left.chars().next_back().is_some_and(spacingless)
        || right.chars().next().is_some_and(spacingless)
}

#[cfg(test)]
mod p1494_space_tests {
    use super::{discard_space_between, normalize_markup_spaces};
    use crate::entities::content::Content;
    use crate::entities::style::Styles;

    #[test]
    fn pin_unicode_spacing_controls() {
        for (left, right, discard) in [
            ("水", "果", true),
            ("换行", "newline", true),
            ("newline", "换行", true),
            ("쉼표，", "줄", true),
            ("줄", "바꿈", false),
            ("new", "line", false),
            ("“你好”", "“你好”", false),
            ("🎃", "🎃", false),
            ("A", "（B", true),
            ("A", "₩", false),
        ] {
            assert_eq!(discard_space_between(left, right), discard, "{left:?}/{right:?}");
        }
    }

    #[test]
    fn styled_space_preserva_origem_ate_observar_vizinhos() {
        let styled =
            Content::styled(Box::new(Content::SpaceFromNewline), Styles::default());
        let mut isolated = [styled.clone()];
        normalize_markup_spaces(&mut isolated);
        assert!(matches!(
            &isolated[0],
            Content::Styled(styled) if matches!(&styled.body, Content::SpaceFromNewline)
        ));

        let mut between_han = [Content::text("水"), styled, Content::text("果")];
        normalize_markup_spaces(&mut between_han);
        assert!(matches!(&between_han[1], Content::Empty));
    }
}
