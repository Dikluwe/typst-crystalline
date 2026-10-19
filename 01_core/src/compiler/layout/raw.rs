//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/raw.md
//! @prompt-hash 1a521fdb
//! @layer L1
//! @updated 2026-10-07
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

    let lang = e.lang.as_deref().unwrap_or("txt").to_lowercase();
    let syntaxes = get_syntaxes();
    let theme = get_theme();
    let syntax = syntaxes.find_syntax_by_token(&lang);

    if let Some(syntax) = syntax {
        let mut highlighter = syntect::easy::HighlightLines::new(syntax, theme);
        let lines: Vec<&str> = e.text.lines().collect();

        for (i, line) in lines.iter().enumerate() {
            if i > 0 && e.block {
                layouter.flush_line();
                layouter.regions.current.cursor_x = layouter.regions.current.line_start_x;
            }

            if let Ok(ranges) = highlighter.highlight_line(line, syntaxes) {
                for (synt_style, piece) in ranges {
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
                    for word in piece.split_whitespace() {
                        layouter.layout_word(word);
                    }
                }
            } else {
                layouter.style = base_style.clone();
                for word in line.split_whitespace() {
                    layouter.layout_word(word);
                }
            }
        }
    } else {
        layouter.style = base_style;
        for (i, line) in e.text.lines().enumerate() {
            if i > 0 && e.block {
                layouter.flush_line();
                layouter.regions.current.cursor_x = layouter.regions.current.line_start_x;
            }
            for word in line.split_whitespace() {
                layouter.layout_word(word);
            }
        }
    }

    if let Some(state) = raw_indent_state {
        end_block(layouter, block_font);
        layouter.paragraph_indent = state;
    }
    layouter.style = prev;
}
