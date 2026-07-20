//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/layout/raw_highlight.md
//! @prompt-hash 1a34696e
//! @layer L1
//! @updated 2026-07-20
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
    static SYNTAXES: LazyLock<SyntaxSet> = LazyLock::new(two_face::syntax::extra_no_newlines);
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
            item("markup.heading, entity.name.section", None, Some(synt::FontStyle::BOLD)),
            item("markup.heading.typst", None, Some(synt::FontStyle::BOLD | synt::FontStyle::UNDERLINE)),
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
            item("meta.mapping.key.json string.quoted.double.json", Some("#4b69c6"), None),
            item("meta.mapping.value.json string.quoted.double.json", Some("#198810"), None),
        ],
    });
    &THEME
}

/// Layout de código `raw` (`` `...` ``): estilo a 90% sem bold/italic; em modo
/// bloco força nova linha e indenta. Aplica syntax highlighting quando `lang` é fornecido.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e:        &RawElem,
) {
    let prev = layouter.style.clone();
    let base_style = TextStyle {
        bold: false,
        italic: false,
        size: Pt(prev.size.0 * 0.9),
        fill: None,
        font: prev.font.clone(),
        lang: prev.lang.clone(),
        dir: prev.dir,
        ..prev.clone()
    };

    let lang = e.lang.as_deref().unwrap_or("txt").to_lowercase();
    let syntaxes = get_syntaxes();
    let theme = get_theme();
    let syntax = syntaxes.find_syntax_by_token(&lang);

    if e.block {
        if layouter.regions.current.cursor_x.0 > layouter.page_config.margin {
            layouter.flush_line();
        }
        layouter.regions.current.cursor_x = Pt(layouter.page_config.margin) + prev.size;
    }

    if let Some(syntax) = syntax {
        let mut highlighter = syntect::easy::HighlightLines::new(syntax, theme);
        let lines: Vec<&str> = e.text.lines().collect();

        for (i, line) in lines.iter().enumerate() {
            if i > 0 && e.block {
                layouter.flush_line();
                layouter.regions.current.cursor_x = Pt(layouter.page_config.margin) + prev.size;
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
                    if synt_style.foreground != (synt::Color { r: 0, g: 0, b: 0, a: 255 }) && synt_style.foreground != (synt::Color { r: 0, g: 0, b: 0, a: 0 }) {
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
        for word in e.text.split_whitespace() {
            layouter.layout_word(word);
        }
    }

    if e.block {
        layouter.flush_line();
    }
    layouter.style = prev;
}
