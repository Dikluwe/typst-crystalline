//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/format_options.md
//! @prompt-hash 260a4e0f
//! @layer L1
//! @updated 2026-09-19

use crate::entities::content::Content;
use crate::entities::document_format_options::DocumentFormatOptions;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::style::Styles;
use crate::entities::value::Value;

const SVG_PRETTY_KEY: &str = "format.svg.pretty";
const PNG_PPI_KEY: &str = "format.png.ppi";
const HTML_PRETTY_KEY: &str = "format.html.pretty";
const NESTED_FORMAT_RULE_ERROR: &str =
    "format set rules are not allowed inside of containers";

fn typst_type_name(value: &Value) -> &'static str {
    match value {
        Value::Bool(_) => "boolean",
        Value::Int(_) => "integer",
        Value::Str(_) => "string",
        _ => value.type_name(),
    }
}

fn explicit_svg_pretty_values(styles: &Styles) -> impl Iterator<Item = &Value> {
    styles
        .delta()
        .custom
        .iter()
        .filter_map(|(key, value)| (key.as_str() == SVG_PRETTY_KEY).then_some(value))
}

fn explicit_png_ppi_values(styles: &Styles) -> impl Iterator<Item = &Value> {
    styles
        .delta()
        .custom
        .iter()
        .filter_map(|(key, value)| (key.as_str() == PNG_PPI_KEY).then_some(value))
}

fn explicit_html_pretty_values(styles: &Styles) -> impl Iterator<Item = &Value> {
    styles
        .delta()
        .custom
        .iter()
        .filter_map(|(key, value)| (key.as_str() == HTML_PRETTY_KEY).then_some(value))
}

fn validate_svg_pretty(value: &Value) -> SourceResult<bool> {
    match value {
        Value::Bool(value) => Ok(*value),
        other => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!(
                "expected boolean, found {} in internal {SVG_PRETTY_KEY}",
                typst_type_name(other)
            ),
        )]),
    }
}

fn validate_png_ppi(value: &Value) -> SourceResult<f64> {
    match value {
        Value::Float(value) => Ok(*value),
        other => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!(
                "expected float, found {} in internal {PNG_PPI_KEY}",
                typst_type_name(other)
            ),
        )]),
    }
}

fn validate_html_pretty(value: &Value) -> SourceResult<bool> {
    match value {
        Value::Bool(value) => Ok(*value),
        other => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!(
                "expected boolean, found {} in internal {HTML_PRETTY_KEY}",
                typst_type_name(other)
            ),
        )]),
    }
}

fn collect_root_options(
    content: &Content,
    options: &mut DocumentFormatOptions,
    root_occurrences: &mut usize,
) -> SourceResult<()> {
    match content {
        Content::Sequence(sequence) => {
            for child in sequence.iter() {
                collect_root_options(child, options, root_occurrences)?;
            }
        }
        Content::Styled(body, styles) => {
            for value in explicit_svg_pretty_values(styles) {
                options.svg.pretty = validate_svg_pretty(value)?;
                *root_occurrences += 1;
            }
            for value in explicit_png_ppi_values(styles) {
                options.png.ppi = validate_png_ppi(value)?;
                *root_occurrences += 1;
            }
            for value in explicit_html_pretty_values(styles) {
                options.html.pretty = validate_html_pretty(value)?;
                *root_occurrences += 1;
            }
            collect_root_options(body, options, root_occurrences)?;
        }
        _ => {}
    }
    Ok(())
}

/// Extrai opções documentais de formato e rejeita decisões explícitas
/// aninhadas em containers. `Sequence` e `Styled` são os únicos envelopes
/// transparentes de raiz.
pub fn collect_document_format_options(
    content: &Content,
) -> SourceResult<DocumentFormatOptions> {
    let mut all_occurrences = 0usize;
    content.map_content(&mut |node| {
        if let Content::Styled(_, styles) = node {
            for value in explicit_svg_pretty_values(styles) {
                validate_svg_pretty(value)?;
                all_occurrences += 1;
            }
            for value in explicit_png_ppi_values(styles) {
                validate_png_ppi(value)?;
                all_occurrences += 1;
            }
            for value in explicit_html_pretty_values(styles) {
                validate_html_pretty(value)?;
                all_occurrences += 1;
            }
        }
        Ok(None)
    })?;

    let mut options = DocumentFormatOptions::default();
    let mut root_occurrences = 0usize;
    collect_root_options(content, &mut options, &mut root_occurrences)?;

    if all_occurrences > root_occurrences {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            NESTED_FORMAT_RULE_ERROR,
        )]);
    }

    Ok(options)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::style::Styles;

    fn svg_pretty(body: Content, value: Value) -> Content {
        Content::Styled(Box::new(body), Styles::new().push_custom(SVG_PRETTY_KEY, value))
    }

    fn html_pretty(body: Content, value: Value) -> Content {
        Content::Styled(Box::new(body), Styles::new().push_custom(HTML_PRETTY_KEY, value))
    }

    #[test]
    fn p1462_absence_and_unrelated_styles_keep_default_false() {
        let content = Content::Styled(
            Box::new(Content::text("body")),
            Styles::new().push_custom("text.example", Value::Bool(true)),
        );

        let options = collect_document_format_options(&content).unwrap();

        assert!(!options.svg.pretty);
        assert!(!options.html.pretty);
    }

    #[test]
    fn p1462_root_sequence_and_styled_use_lexical_last_write_wins() {
        let content = svg_pretty(
            Content::sequence(vec![
                Content::text("before"),
                svg_pretty(Content::text("after"), Value::Bool(false)),
            ]),
            Value::Bool(true),
        );
        assert!(!collect_document_format_options(&content).unwrap().svg.pretty);

        let reversed = Content::sequence(vec![
            svg_pretty(Content::Empty, Value::Bool(false)),
            svg_pretty(Content::Empty, Value::Bool(true)),
        ]);
        assert!(collect_document_format_options(&reversed).unwrap().svg.pretty);
    }

    #[test]
    fn p1462_styled_without_svg_key_is_transparent() {
        let content = Content::Styled(
            Box::new(svg_pretty(Content::Empty, Value::Bool(true))),
            Styles::new().push_custom("other.key", Value::Bool(false)),
        );

        assert!(collect_document_format_options(&content).unwrap().svg.pretty);
    }

    #[test]
    fn p1462_explicit_rule_in_distinct_containers_fails_closed() {
        for nested in [
            Content::Par {
                body: Box::new(svg_pretty(Content::Empty, Value::Bool(true))),
            },
            Content::SmallCaps {
                body: Box::new(svg_pretty(Content::Empty, Value::Bool(false))),
            },
        ] {
            let errors = collect_document_format_options(&nested).unwrap_err();
            assert_eq!(errors[0].message, NESTED_FORMAT_RULE_ERROR);
        }
    }

    #[test]
    fn p1462_non_boolean_internal_value_fails_closed() {
        let content = svg_pretty(Content::Empty, Value::Int(1));

        let errors = collect_document_format_options(&content).unwrap_err();

        assert!(errors[0].message.contains("expected boolean, found integer"));
    }

    #[test]
    fn p1465_html_root_cascade_is_last_write_wins() {
        let content = html_pretty(
            Content::sequence(vec![
                Content::text("before"),
                html_pretty(Content::text("after"), Value::Bool(false)),
            ]),
            Value::Bool(true),
        );
        assert!(!collect_document_format_options(&content).unwrap().html.pretty);

        let reversed = Content::sequence(vec![
            html_pretty(Content::Empty, Value::Bool(false)),
            html_pretty(Content::Empty, Value::Bool(true)),
        ]);
        assert!(collect_document_format_options(&reversed).unwrap().html.pretty);
    }

    #[test]
    fn p1465_html_nested_and_non_boolean_values_fail_closed() {
        let nested = Content::Par {
            body: Box::new(html_pretty(Content::Empty, Value::Bool(true))),
        };
        let errors = collect_document_format_options(&nested).unwrap_err();
        assert_eq!(errors[0].message, NESTED_FORMAT_RULE_ERROR);

        let invalid = html_pretty(Content::Empty, Value::Int(1));
        let errors = collect_document_format_options(&invalid).unwrap_err();
        assert!(errors[0].message.contains("expected boolean, found integer"));
        assert!(errors[0].message.contains(HTML_PRETTY_KEY));
    }
}
