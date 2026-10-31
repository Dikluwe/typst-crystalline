//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/raw.md
//! @prompt-hash 351a9fad
//! @layer L1
//! @updated 2026-10-31
//!
//! Highlighting em Blocos Raw (P785a): integração de syntect + two-face para
//! tokenização e coloração de código-fonte no layouter de `raw`.

use std::sync::LazyLock;
use syntect::highlighting as synt;
use syntect::parsing::SyntaxSet;

use crate::entities::color::Color;
use crate::entities::elements::raw::RawElem;
use crate::entities::layout_types::{Pt, TextStyle};

use super::{FontMetrics, ImageSizer, Layouter};

// Complete recognition precedes output, so a refused stream cannot partly render.
fn numeric_typc_tokens(
    text: &str,
) -> Option<Vec<(crate::entities::syntax_kind::SyntaxKind, &str)>> {
    use crate::entities::syntax_kind::SyntaxKind as Kind;
    let mut lexer = crate::compiler::lexer::Lexer::new(
        text,
        crate::entities::syntax_mode::SyntaxMode::Code,
    );
    let mut tokens = Vec::new();
    let mut expecting_number = true;
    let mut sign_allowed = true;
    let mut has_number = false;
    loop {
        let start = lexer.cursor();
        let kind = lexer.next().0;
        if kind == Kind::End {
            break;
        }
        let token = &text[start..lexer.cursor()];
        if kind == Kind::Space && token.chars().all(|c| c == ' ') {
            tokens.push((kind, token));
            continue;
        }
        if matches!(kind, Kind::Numeric | Kind::Int | Kind::Float) && expecting_number {
            expecting_number = false;
            has_number = true;
            sign_allowed = true;
        } else if matches!(kind, Kind::Plus | Kind::Minus) {
            if expecting_number {
                if !sign_allowed {
                    return None;
                }
                sign_allowed = false;
            } else {
                expecting_number = true;
                sign_allowed = true;
            }
        } else {
            return None;
        }
        tokens.push((kind, token));
    }
    (has_number && !expecting_number).then_some(tokens)
}

fn item(
    scope: &str,
    color: Option<&str>,
    font_style: Option<synt::FontStyle>,
) -> synt::ThemeItem {
    synt::ThemeItem {
        scope: scope.parse().unwrap(),
        style: synt::StyleModifier {
            foreground: color.map(|s| {
                let hex = s.trim_start_matches('#');
                let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
                let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
                let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
                synt::Color { r, g, b, a: 255 }
            }),
            background: None,
            font_style,
        },
    }
}

fn get_syntaxes() -> &'static SyntaxSet {
    static SYNTAXES: LazyLock<SyntaxSet> =
        LazyLock::new(two_face::syntax::extra_no_newlines);
    &SYNTAXES
}

// Python's escaped physical newline requires the newline-aware grammar.
// Other languages retain the existing syntax set and line input.
fn get_python_syntaxes() -> &'static SyntaxSet {
    static SYNTAXES: LazyLock<SyntaxSet> =
        LazyLock::new(two_face::syntax::extra_newlines);
    &SYNTAXES
}

fn get_theme() -> &'static synt::Theme {
    static THEME: LazyLock<synt::Theme> = LazyLock::new(|| synt::Theme {
        name: Some("Typst Light".into()),
        author: Some("The Typst Project Developers".into()),
        settings: synt::ThemeSettings::default(),
        scopes: vec![
            item("comment", Some("#74747c"), None),
            item("constant.character.escape", Some("#1d6c76"), None),
            item("markup.bold", None, Some(synt::FontStyle::BOLD)),
            item("markup.italic", None, Some(synt::FontStyle::ITALIC)),
            item("markup.underline", None, Some(synt::FontStyle::UNDERLINE)),
            item("markup.raw", Some("#6b6b6f"), None),
            item("string.other.math.typst", None, None),
            item("punctuation.definition.math", Some("#198810"), None),
            item("keyword.operator.math, punctuation.math.typst", Some("#1d6c76"), None),
            item(
                "markup.heading, entity.name.section",
                None,
                Some(synt::FontStyle::BOLD),
            ),
            item(
                "markup.heading.typst",
                None,
                Some(synt::FontStyle::BOLD | synt::FontStyle::UNDERLINE),
            ),
            item("punctuation.definition.list", Some("#8b41b1"), None),
            item("markup.list.term", None, Some(synt::FontStyle::BOLD)),
            item("entity.name.label, markup.other.reference", Some("#1d6c76"), None),
            item("keyword, constant.language, variable.language", Some("#d73948"), None),
            item("storage.type, storage.modifier", Some("#d73948"), None),
            item("constant", Some("#b60157"), None),
            item("string", Some("#198810"), None),
            item("entity.name, variable.function, support", Some("#4b69c6"), None),
            item("support.macro", Some("#16718d"), None),
            item("meta.annotation", Some("#301414"), None),
            item("entity.other, meta.interpolation", Some("#8b41b1"), None),
            item("meta.diff.range", Some("#8b41b1"), None),
            item("markup.inserted, meta.diff.header.to-file", Some("#198810"), None),
            item("markup.deleted, meta.diff.header.from-file", Some("#d73948"), None),
            item(
                "meta.mapping.key.json string.quoted.double.json",
                Some("#4b69c6"),
                None,
            ),
            item(
                "meta.mapping.value.json string.quoted.double.json",
                Some("#198810"),
                None,
            ),
        ],
    });
    &THEME
}

/// Defaults de render do pin 9f2b6e871, antes dos overrides show-set.
pub(super) fn default_style(
    outer: &TextStyle,
    chain: &crate::entities::style_chain::StyleChain,
) -> TextStyle {
    TextStyle {
        bold: false,
        italic: false,
        // ref: lab/typst-original/crates/typst-library/src/text/raw.rs:663
        size: Pt(outer.size.val() * 0.8),
        font: Some(crate::entities::font_list::FontList::single(
            "DejaVu Sans Mono".into(),
        )),
        fill: match chain.custom("text.fill") {
            Some(crate::entities::value::Value::Color(c)) => Some(c.clone()),
            _ => outer.fill.clone(),
        },
        ..outer.clone()
    }
}

fn block_margin<M: FontMetrics, S: ImageSizer>(
    l: &Layouter<M, S>,
    edge: &str,
    font: f64,
) -> (f64, bool) {
    use crate::entities::value::Value;
    for key in [edge, "block.spacing"] {
        if let Some(Value::Length(v)) = l.chain.custom(key) {
            return (v.resolve_pt(font), true);
        }
        if l.chain.custom(key) == Some(&Value::Auto) {
            break;
        }
    }
    if let Some(Value::Length(v)) = l.chain.custom("par.spacing") {
        return (v.resolve_pt(font), false);
    }
    (font * super::vanilla_defaults::BLOCK_SPACING, false)
}

pub(super) fn begin_block<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    font: f64,
    outer: &TextStyle,
) -> super::paragraph_indent::State {
    let prefix = super::html_frame::items_have_material(&l.regions.current.current_line);
    let previous_paragraph = prefix
        || (!l.block_chain_active
            && super::html_frame::items_have_material(&l.regions.current.current_items));
    if prefix {
        l.flush_line();
    }
    let (above, explicit) = block_margin(l, "block.above", font);
    if previous_paragraph || l.block_chain_active {
        let preceding = if previous_paragraph {
            match l.chain.custom("par.spacing") {
                Some(crate::entities::value::Value::Length(v)) => {
                    v.resolve_pt(outer.size.val())
                }
                _ => outer.size.val() * super::vanilla_defaults::PAR_SPACING,
            }
        } else {
            l.prev_block_below_pending
        };
        let weak = previous_paragraph || l.prev_margin_is_parbreak;
        let gap = if explicit && weak {
            above
        } else if !explicit && !weak {
            preceding
        } else {
            preceding.max(above)
        };
        l.regions.current.cursor_y =
            Pt(l.prev_line_baseline + l.prev_block_equation_descent + gap);
        l.initial_baseline_pending = true;
    } else if l.is_sub_frame
        && l.regions
            .current
            .current_items
            .iter()
            .all(super::html_frame::is_context_marker)
        && !l.initial_baseline_pending
    {
        l.regions.current.cursor_y =
            Pt(l.regions.current.cursor_y.0 - l.line_assumed_ascent.unwrap_or(0.0));
        l.initial_baseline_pending = true;
    }
    l.regions.current.cursor_x = l.regions.current.line_start_x;
    l.prev_block_below_pending = 0.0;
    l.block_chain_active = false;
    l.prev_margin_is_parbreak = false;
    super::paragraph_indent::end(l);
    std::mem::replace(&mut l.paragraph_indent, super::paragraph_indent::State::inline())
}

pub(super) fn end_block<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    font: f64,
) {
    let has_line =
        super::html_frame::items_have_material(&l.regions.current.current_line);
    let bottom = l
        .regions
        .current
        .current_line
        .iter()
        .filter_map(|i| match i {
            crate::entities::layout_types::FrameItem::Text { style, .. }
            | crate::entities::layout_types::FrameItem::TextShaped { style, .. } => {
                Some((-l.metrics.text_edges(style.size, style).1 .0).max(0.0))
            }
            _ => None,
        })
        .fold(0.0_f64, f64::max);
    let empty_origin = l.initial_baseline_pending.then_some(l.regions.current.cursor_y.0);
    if has_line {
        l.flush_line();
    }
    let base = empty_origin.unwrap_or(l.prev_line_baseline);
    let end = base + bottom;
    l.prev_line_baseline = base;
    l.prev_block_equation_descent = bottom;
    l.last_block_descent_y = Some(end);
    l.regions.current.cursor_y = Pt(end);
    l.initial_baseline_pending = false;
    let (below, explicit) = block_margin(l, "block.below", font);
    l.prev_block_below_pending = below;
    l.block_chain_active = true;
    l.prev_margin_is_parbreak = !explicit;
    if explicit && below <= 0.0 {
        l.regions.current.cursor_y = Pt(end + below);
        l.initial_baseline_pending = true;
    }
    l.record_flow_placement_end(end);
}

/// Terminal Raw block products use their real lower edge in sub-frame measurement.
pub(super) fn terminal_block(c: &crate::entities::content::Content) -> bool {
    use crate::entities::{content::Content, value::Value};
    if let Content::Raw(r) = c {
        return r.block;
    }
    if let Content::Styled(s) = c {
        return s
            .styles
            .delta()
            .custom
            .iter()
            .any(|(k, v)| k.as_str() == "raw.block" && v == &Value::Bool(true))
            || terminal_block(&s.body);
    }
    if let Content::Located(s) = c {
        return terminal_block(&s.body);
    }
    if let Content::Realized(s) = c {
        return terminal_block(&s.visual);
    }
    if let Content::ShowRecipe(s) = c {
        return terminal_block(&s.body);
    }
    if let Content::Strong(s) = c {
        return terminal_block(&s.body);
    }
    if let Content::Emph(s) = c {
        return terminal_block(&s.body);
    }
    if let Content::TableCell(s) = c {
        return terminal_block(s.visual());
    }
    if let Content::Sequence(v) = c {
        return v
            .iter()
            .rev()
            .find(|c| {
                !matches!(
                    c,
                    Content::Empty
                        | Content::Space
                        | Content::SpaceFromNewline
                        | Content::Metadata(_)
                )
            })
            .is_some_and(terminal_block);
    }
    false
}

/// Render Raw no escopo corrente, restaurando o estilo exterior.
fn paint_literal_spaces<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    text: &str,
) {
    for (index, part) in text.split(' ').enumerate() {
        if index > 0 {
            super::space::advance(
                layouter,
                crate::entities::layout_types::TextSeparatorOrigin::Literal,
            );
        }
        // Other whitespace retains the bounded pre-existing fallback.
        for word in part.split_whitespace() {
            layouter.layout_word(word);
        }
    }
}

pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &RawElem,
) {
    let prev = layouter.style.clone();
    let owns_block = e.block
        && layouter.chain.custom("raw.block.active")
            != Some(&crate::entities::value::Value::Bool(true));
    let mut base_style = if layouter.chain.custom("raw.defaults.active")
        == Some(&crate::entities::value::Value::Bool(true))
    {
        prev.clone()
    } else {
        default_style(&prev, &layouter.chain)
    };
    // Raw is normal text even when its exterior is mathematical. Its actual
    // text face owns both metric selection and drawing.
    base_style.math = false;
    let block_font = base_style.size.val();
    let raw_indent_state = owns_block.then(|| begin_block(layouter, block_font, &prev));
    if let Some(crate::entities::value::Value::Color(c)) =
        layouter.chain.custom("text.fill")
    {
        base_style.fill = Some(c.clone());
    }
    // A gramática typc não é disponibilizada por todas as bundles two-face.
    // Átomos Auto/Bool conservam a cor constant.language da paleta Typst Light.
    if e.lang.as_deref() == Some("typc")
        && matches!(e.text.as_str(), "true" | "false" | "auto")
    {
        base_style.fill = Some(Color::Srgb {
            r: 0xd7 as f32 / 255.0,
            g: 0x39 as f32 / 255.0,
            b: 0x48 as f32 / 255.0,
            a: 1.0,
        });
    }

    if !e.block && e.lang.as_deref() == Some("typc") {
        if let Some(tokens) = numeric_typc_tokens(&e.text) {
            use crate::entities::syntax_kind::SyntaxKind;
            for (kind, text) in tokens {
                layouter.style = base_style.clone();
                if kind == SyntaxKind::Space {
                    for _ in text.chars() {
                        super::space::advance(
                            layouter,
                            crate::entities::layout_types::TextSeparatorOrigin::Literal,
                        );
                    }
                } else {
                    layouter.style.fill =
                        Some(if matches!(kind, SyntaxKind::Plus | SyntaxKind::Minus) {
                            Color::rgb(0xd7, 0x39, 0x48)
                        } else {
                            Color::rgb(0xb6, 0x01, 0x57)
                        });
                    layouter.layout_word(text);
                }
            }
            layouter.style = prev;
            return;
        }
    }

    let lang = e.lang.as_deref().unwrap_or("txt").to_lowercase();
    let syntaxes = get_syntaxes();
    let theme = get_theme();
    let python = syntaxes
        .find_syntax_by_token(&lang)
        .is_some_and(|syntax| syntax.name == "Python");
    let syntaxes = if python { get_python_syntaxes() } else { syntaxes };
    let syntax = syntaxes.find_syntax_by_token(&lang);

    if let Some(syntax) = syntax {
        let mut highlighter = syntect::easy::HighlightLines::new(syntax, theme);
        let lines: Vec<&str> = if python {
            e.text.split_inclusive('\n').collect()
        } else {
            e.text.lines().collect()
        };

        for (i, line) in lines.iter().enumerate() {
            if i > 0 && e.block {
                layouter.flush_line();
                layouter.regions.current.cursor_x = layouter.regions.current.line_start_x;
            }

            let visible = line
                .strip_suffix("\r\n")
                .or_else(|| line.strip_suffix('\n'))
                .unwrap_or(line);
            // The embedded Python grammar recognizes escaped LF, not CRLF.
            let input = if python && line.ends_with("\r\n") {
                std::borrow::Cow::Owned(format!("{visible}\n"))
            } else {
                std::borrow::Cow::Borrowed(*line)
            };
            if let Ok(ranges) = highlighter.highlight_line(&input, syntaxes) {
                // Terminators drive lexical state, but layout owns physical breaks.
                let mut remaining = visible.len();
                for (synt_style, piece) in ranges {
                    let end = remaining.min(piece.len());
                    remaining -= end;
                    let piece = &piece[..end];
                    if piece.is_empty() {
                        continue;
                    }
                    let mut tok_style = base_style.clone();

                    if synt_style.font_style.contains(synt::FontStyle::BOLD) {
                        tok_style.bold = true;
                    }
                    if synt_style.font_style.contains(synt::FontStyle::ITALIC) {
                        tok_style.italic = true;
                    }
                    if synt_style.foreground != (synt::Color { r: 0, g: 0, b: 0, a: 255 })
                        && synt_style.foreground
                            != (synt::Color { r: 0, g: 0, b: 0, a: 0 })
                    {
                        tok_style.fill = Some(Color::Srgb {
                            r: synt_style.foreground.r as f32 / 255.0,
                            g: synt_style.foreground.g as f32 / 255.0,
                            b: synt_style.foreground.b as f32 / 255.0,
                            a: synt_style.foreground.a as f32 / 255.0,
                        });
                    }

                    layouter.style = tok_style;
                    paint_literal_spaces(layouter, piece);
                }
            } else {
                layouter.style = base_style.clone();
                paint_literal_spaces(layouter, visible);
            }
        }
    } else {
        layouter.style = base_style;
        for (i, line) in e.text.lines().enumerate() {
            if i > 0 && e.block {
                layouter.flush_line();
                layouter.regions.current.cursor_x = layouter.regions.current.line_start_x;
            }
            paint_literal_spaces(layouter, line);
        }
    }

    if let Some(state) = raw_indent_state {
        end_block(layouter, block_font);
        layouter.paragraph_indent = state;
    }
    layouter.style = prev;
}

#[cfg(test)]
mod remaining_typc_length_tests {
    use super::*;
    use crate::entities::content::Content;
    use crate::entities::image_sizer::NullImageSizer;
    use crate::entities::introspector::{Introspector, TagIntrospector};
    use crate::entities::layout_types::FrameItem;
    use comemo::Track;

    #[test]
    fn remaining_typc_numeric_raw_uses_numeric_paint() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.layout_content(&Content::raw("158.33pt", Some("typc".into()), false));
        let doc = l.finish();
        let expected = Color::Srgb {
            r: 0xb6 as f32 / 255.0,
            g: 0x01 as f32 / 255.0,
            b: 0x57 as f32 / 255.0,
            a: 1.0,
        };
        let fills: Vec<_> = doc
            .pages
            .iter()
            .flat_map(|p| &p.items)
            .filter_map(|item| {
                if let FrameItem::Text { style, .. }
                | FrameItem::TextShaped { style, .. } = item
                {
                    Some(style.fill.clone())
                } else {
                    None
                }
            })
            .collect();
        assert!(!fills.is_empty());
        assert!(fills.iter().all(|fill| fill.as_ref() == Some(&expected)), "{fills:?}");
    }

    #[test]
    fn remaining_typc_numeric_raw_retains_operator_spaces() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.layout_content(&Content::raw("12pt + 0.5em", Some("typc".into()), false));
        let doc = l.finish();
        assert_eq!(doc.plain_text().trim(), "12pt + 0.5em");
        let separators: Vec<_> = doc
            .pages
            .iter()
            .flat_map(|page| &page.items)
            .filter_map(|item| {
                if let FrameItem::TextSeparator { origin, advance, style, .. } = item {
                    Some((*origin, *advance, style.size))
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(separators.len(), 2, "actual literal Raw spaces: {separators:?}");
        for (origin, advance, size) in separators {
            assert_eq!(
                origin,
                crate::entities::layout_types::TextSeparatorOrigin::Literal
            );
            assert!(advance.val().is_finite() && advance.val() > 0.0);
            assert_eq!(size.val(), 11.0 * 0.8);
        }
    }
}

#[cfg(test)]
mod readiness_flow_tests {
    use super::*;
    use crate::entities::{
        content::Content,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{FrameItem, TextSeparatorOrigin},
    };
    use comemo::Track;

    #[test]
    fn readiness_raw_keeps_literal_spaces_and_indent() {
        let intr = TagIntrospector::empty();
        for lang in [None, Some("rust"), Some("tekt-unknown")] {
            for block in [false, true] {
                let mut l = Layouter::new(
                    super::super::FixedMetrics,
                    NullImageSizer,
                    11.0,
                    (&intr as &dyn Introspector).track(),
                );
                let origin = l.regions.current.line_start_x.0;
                l.layout_content(&Content::raw(
                    "  alpha  beta",
                    lang.map(Into::into),
                    block,
                ));
                let doc = l.finish();
                let items: Vec<_> = doc
                    .pages
                    .iter()
                    .flat_map(|p| {
                        crate::compiler::layout::tests::frame_items_recursive(&p.items)
                    })
                    .collect();
                let text_pos = |name: &str| {
                    items
                        .iter()
                        .find_map(|i| match i {
                            FrameItem::Text { text, pos, style, .. } if text == name => {
                                Some((*pos, style.clone()))
                            }
                            _ => None,
                        })
                        .unwrap()
                };
                let (alpha, style) = text_pos("alpha");
                let (beta, _) = text_pos("beta");
                let space = super::super::FixedMetrics.advance(" ", style.size, &style).0;
                assert!(
                    (alpha.x.0 - origin - 2.0 * space).abs() < 1e-8,
                    "leading Raw spaces: lang={lang:?}, block={block}"
                );
                assert!(
                    (beta.x.0 - alpha.x.0 - 7.0 * space).abs() < 1e-8,
                    "inter-token Raw spaces: lang={lang:?}, block={block}"
                );
                let count = items
                    .iter()
                    .filter(|i| {
                        matches!(
                            i,
                            FrameItem::TextSeparator {
                                origin: TextSeparatorOrigin::Literal,
                                ..
                            }
                        )
                    })
                    .count();
                assert_eq!(count, 4, "actual literal space occurrences");
            }
        }
    }
}

#[cfg(test)]
mod issue_9015_tests {
    use super::*;
    use crate::entities::{
        content::Content,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::FrameItem,
    };
    use comemo::Track;

    fn rendered(text: &str, lang: &str, block: bool) -> Vec<(char, Color, f64)> {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        let before = l.style.clone();
        l.layout_content(&Content::raw(text, Some(lang.into()), block));
        assert_eq!(l.style.fill, before.fill, "exterior paint restored");
        assert_eq!(l.style.size, before.size, "exterior size restored");
        let doc = l.finish();
        let mut result = Vec::new();
        for page in &doc.pages {
            for item in super::super::tests::frame_items_recursive(&page.items) {
                if let FrameItem::Text { text, style, pos, .. } = item {
                    result.extend(text.chars().map(|c| {
                        (c, style.fill.clone().unwrap_or(Color::rgb(0, 0, 0)), pos.y.0)
                    }));
                }
            }
        }
        result
    }

    fn paint_of(items: &[(char, Color, f64)], needle: &str, expected: Color) {
        let text: String = items.iter().map(|(c, _, _)| *c).collect();
        let start = text.find(needle).unwrap_or_else(|| panic!("{needle:?} in {text:?}"));
        let start = text[..start].chars().count();
        for (c, color, _) in &items[start..start + needle.chars().count()] {
            assert_eq!(*color, expected, "paint of {c:?} in {needle:?}: {text:?}");
        }
    }

    #[test]
    fn issue_9015_python_continued_string_and_following_code() {
        for lang in ["py", "python", "PY", "Python"] {
            for quote in ['\'', '"'] {
                for newline in ["\n", "\r\n"] {
                    for prefix in ["", "r", "f"] {
                        let source = format!("{prefix}{quote}Hello\\{newline}world{quote} + 123 # tail{newline}print(42)");
                        let items = rendered(&source, lang, true);
                        paint_of(&items, "world", Color::rgb(0x19, 0x88, 0x10));
                        paint_of(&items, "123", Color::rgb(0xb6, 0x01, 0x57));
                        paint_of(&items, "#tail", Color::rgb(0x74, 0x74, 0x7c));
                        paint_of(&items, "42", Color::rgb(0xb6, 0x01, 0x57));
                        let hello = items.iter().position(|(c, _, _)| *c == 'H').unwrap();
                        let world = items.iter().position(|(c, _, _)| *c == 'w').unwrap();
                        assert!(
                            items[world].2 > items[hello].2,
                            "physical line retained"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn issue_9015_python_line_boundaries_and_string_controls() {
        for (source, needle, color) in [
            ("\"one\\\ntwo\\\nthree\" + 123", "three", Color::rgb(0x19, 0x88, 0x10)),
            ("\"one\ntwo\"", "two", Color::rgb(0, 0, 0)),
            ("\"one\\\\\ntwo\"", "two", Color::rgb(0, 0, 0)),
            ("\"\"\"one\ntwo\"\"\" + 123", "two", Color::rgb(0x74, 0x74, 0x7c)),
            ("\"one\\\ntwo\"\n\"fresh\"", "fresh", Color::rgb(0x19, 0x88, 0x10)),
            ("value = 123", "123", Color::rgb(0xb6, 0x01, 0x57)),
            ("\"one\\\ntwo\"\n", "two", Color::rgb(0x19, 0x88, 0x10)),
        ] {
            paint_of(&rendered(source, "py", true), needle, color);
        }
        paint_of(
            &rendered("\"one\\\ntwo\" + 123", "py", false),
            "two",
            Color::rgb(0x19, 0x88, 0x10),
        );
    }

    #[test]
    fn issue_9015_other_languages_keep_existing_highlighting() {
        for (lang, source) in [
            ("rust", "/* comment\nstill */ let value = \"hello\";"),
            ("js", "/* comment\nstill */ const value = \"hello\";"),
            ("json", "{\"key\":\n\"value\"}"),
            ("unknown-9015", "plain\nwords"),
        ] {
            let actual = rendered(source, lang, true);
            let mut expected = Vec::new();
            if let Some(syntax) = get_syntaxes().find_syntax_by_token(lang) {
                let mut h = syntect::easy::HighlightLines::new(syntax, get_theme());
                for line in source.lines() {
                    for (style, text) in h.highlight_line(line, get_syntaxes()).unwrap() {
                        let c = style.foreground;
                        expected.extend(
                            text.chars()
                                .filter(|c| !c.is_whitespace())
                                .map(|ch| (ch, Color::rgba(c.r, c.g, c.b, c.a))),
                        );
                    }
                }
            } else {
                expected.extend(
                    source
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .map(|c| (c, Color::rgb(0, 0, 0))),
                );
            }
            let actual: Vec<_> =
                actual.into_iter().map(|(c, color, _)| (c, color)).collect();
            assert_eq!(actual, expected, "{lang}");
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    struct Observation {
        ch: char,
        pos: crate::entities::layout_types::Point,
        advance: Pt,
        style: TextStyle,
        literal_space: bool,
    }

    fn observed_layout(source: &str, lang: &str, block: bool) -> Vec<Observation> {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.style.fill = Some(Color::rgb(0x80, 0x20, 0x60));
        l.style.bold = true;
        l.style.italic = true;
        let exterior = l.style.clone();
        l.layout_content(&Content::raw(source, Some(lang.into()), block));
        assert_eq!(l.style, exterior, "restore complete nondefault exterior style");
        let doc = l.finish();
        let mut observations = Vec::new();
        for page in &doc.pages {
            for item in super::super::tests::frame_items_recursive(&page.items) {
                if let FrameItem::Text { text, style, pos, .. } = item {
                    let mut x = pos.x;
                    for ch in text.chars() {
                        let advance = super::super::FixedMetrics.advance(
                            &ch.to_string(),
                            style.size,
                            style,
                        );
                        observations.push(Observation {
                            ch,
                            pos: crate::entities::layout_types::Point { x, y: pos.y },
                            advance,
                            style: style.clone(),
                            literal_space: false,
                        });
                        x += advance;
                    }
                } else if let FrameItem::TextSeparator {
                    pos,
                    advance,
                    style,
                    origin,
                    ..
                } = item
                {
                    observations.push(Observation {
                        ch: ' ',
                        pos: *pos,
                        advance: *advance,
                        style: style.clone(),
                        literal_space: *origin == crate::entities::layout_types::TextSeparatorOrigin::Literal,
                    });
                }
            }
        }
        observations
    }

    fn observations_match(actual: &[Observation], expected: &[Observation]) -> bool {
        actual.len() == expected.len()
            && actual.iter().zip(expected).all(|(a, e)| {
                a.ch == e.ch
                    && a.literal_space == e.literal_space
                    && a.style == e.style
                    && (a.pos.x.0 - e.pos.x.0).abs() < 1e-8
                    && (a.pos.y.0 - e.pos.y.0).abs() < 1e-8
                    && (a.advance.0 - e.advance.0).abs() < 1e-8
            })
    }

    fn legacy_expected(source: &str, lang: &str, block: bool) -> Vec<Observation> {
        // An unchanged plain Raw route supplies font, size and physical geometry.
        let mut expected = observed_layout(source, "unknown-9015-geometry", block);
        let visible: String = source.lines().collect();
        assert_eq!(
            expected.iter().map(|s| s.ch).collect::<String>(),
            visible,
            "the observer must retain actual source spaces"
        );
        if let Some(syntax) = get_syntaxes().find_syntax_by_token(lang) {
            let mut h = syntect::easy::HighlightLines::new(syntax, get_theme());
            let mut index = 0;
            for line in source.lines() {
                for (style, text) in h.highlight_line(line, get_syntaxes()).unwrap() {
                    for ch in text.chars() {
                        let sample = &mut expected[index];
                        assert_eq!(sample.ch, ch);
                        sample.style.bold =
                            style.font_style.contains(synt::FontStyle::BOLD);
                        sample.style.italic =
                            style.font_style.contains(synt::FontStyle::ITALIC);
                        let c = style.foreground;
                        if c != (synt::Color { r: 0, g: 0, b: 0, a: 255 })
                            && c != (synt::Color { r: 0, g: 0, b: 0, a: 0 })
                        {
                            sample.style.fill = Some(Color::rgba(c.r, c.g, c.b, c.a));
                        }
                        index += 1;
                    }
                }
            }
            assert_eq!(index, expected.len());
        }
        expected
    }

    fn check_literal_geometry(source: &str, block: bool, samples: &[Observation]) {
        assert_eq!(
            samples.iter().map(|s| s.ch).collect::<String>(),
            source.lines().collect::<String>()
        );
        assert_eq!(
            samples.iter().filter(|s| s.literal_space).count(),
            source.chars().filter(|c| *c == ' ').count()
        );
        let cell = super::super::FixedMetrics
            .advance(" ", Pt(11.0 * 0.8), &TextStyle::default())
            .0;
        let origin = samples[0].pos.x.0;
        let first_y = samples[0].pos.y.0;
        let mut index = 0;
        let mut column = 0;
        let mut prior_row_y = first_y;
        for (row, line) in source.lines().enumerate() {
            if block {
                column = 0;
            }
            let row_y = samples[index].pos.y.0;
            if row > 0 && block {
                assert!(row_y > prior_row_y, "physical line baseline must advance");
            }
            if !block {
                assert!(
                    (row_y - first_y).abs() < 1e-8,
                    "inline Raw must not add a physical break"
                );
            }
            for ch in line.chars() {
                let sample = &samples[index];
                assert_eq!(sample.ch, ch);
                assert_eq!(sample.literal_space, ch == ' ');
                assert!((sample.advance.0 - cell).abs() < 1e-8);
                assert!(
                    (sample.pos.x.0 - origin - column as f64 * cell).abs() < 1e-8,
                    "source column {column}"
                );
                assert!((sample.pos.y.0 - row_y).abs() < 1e-8);
                assert_eq!(sample.style.size, Pt(11.0 * 0.8));
                column += 1;
                index += 1;
            }
            prior_row_y = row_y;
        }
    }

    #[test]
    fn issue_9015_other_languages_preserve_spaces_geometry_and_font_flags() {
        for (lang, lf) in [
            ("rust", "  /* comment\n still */ let  value = \"hello\";  "),
            ("js", "  /* comment\n still */ const  value = \"hello\";  "),
            ("json", "  {\"key\":  \n   \"value\"}  "),
            ("unknown-9015", "  alpha  beta  \n   gamma delta "),
            ("md", "**bold**  *italic* plain  \n**next**  _more_ "),
            ("rst", "**bold**  *italic* plain  \n**next**  *more* "),
        ] {
            for source in [lf.to_string(), lf.replace('\n', "\r\n")] {
                for block in [false, true] {
                    let actual = observed_layout(&source, lang, block);
                    let expected = legacy_expected(&source, lang, block);
                    check_literal_geometry(&source, block, &actual);
                    assert!(
                        observations_match(&actual, &expected),
                        "{lang} block={block}:\nactual={actual:?}\nexpected={expected:?}"
                    );
                    if matches!(lang, "md" | "rst") {
                        assert!(actual.iter().any(|s| s.style.bold));
                        assert!(actual.iter().any(|s| s.style.italic));
                    }
                    assert!(actual.iter().any(|s| !s.style.bold && !s.style.italic));
                }
            }
        }
    }

    #[test]
    fn issue_9015_full_observer_rejects_regressions_hidden_by_old_projection() {
        let source = "**bold**  *italic* plain  \n**next**  _more_ ";
        let baseline = observed_layout(source, "md", true);
        let expected = legacy_expected(source, "md", true);
        assert!(observations_match(&baseline, &expected));
        let space = baseline.iter().position(|s| s.ch == ' ').unwrap();
        let bold = baseline.iter().position(|s| s.ch == 'b' && s.style.bold).unwrap();
        let italic = baseline.iter().position(|s| s.ch == 'i' && s.style.italic).unwrap();
        let old_projection = |items: &[Observation]| {
            items
                .iter()
                .filter(|s| s.ch != ' ')
                .map(|s| (s.ch, s.style.fill.clone()))
                .collect::<Vec<_>>()
        };
        for mutation in [
            "missing-space",
            "extra-space",
            "x",
            "y",
            "advance",
            "bold",
            "italic",
            "literal-origin",
        ] {
            let mut altered = baseline.clone();
            match mutation {
                "missing-space" => {
                    altered.remove(space);
                }
                "extra-space" => {
                    altered.insert(space, baseline[space].clone());
                }
                "x" => altered[bold].pos.x += Pt(1.0),
                "y" => altered[bold].pos.y += Pt(1.0),
                "advance" => altered[space].advance += Pt(1.0),
                "bold" => altered[bold].style.bold = false,
                "italic" => altered[italic].style.italic = false,
                "literal-origin" => altered[space].literal_space = false,
                _ => unreachable!("closed mutation inventory"),
            }
            assert_eq!(
                old_projection(&altered),
                old_projection(&baseline),
                "old projection: {mutation}"
            );
            assert!(
                !observations_match(&altered, &expected),
                "missed regression: {mutation}"
            );
        }
    }
}
