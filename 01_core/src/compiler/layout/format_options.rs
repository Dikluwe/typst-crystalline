//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/format_options.md
//! @prompt-hash 8e600c11
//! @layer L1
//! @updated 2026-10-07

use std::num::NonZeroUsize;

use crate::entities::content::Content;
use crate::entities::document_format_options::{
    pdf_standard_from_name, DocumentFormatOptions, PageRange, PageRanges, PdfStandards,
    PdfTagged,
};
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::style::Styles;
use crate::entities::value::Value;

const SVG_PRETTY_KEY: &str = "format.svg.pretty";
const PNG_PPI_KEY: &str = "format.png.ppi";
const HTML_PRETTY_KEY: &str = "format.html.pretty";
const PDF_TAGGED_KEY: &str = "format.pdf.tagged";
const PDF_PRETTY_KEY: &str = "format.pdf.pretty";
const PDF_PAGES_KEY: &str = "format.pdf.pages";
const PDF_STANDARD_KEY: &str = "format.pdf.standard";
const NESTED_FORMAT_RULE_ERROR: &str =
    "format set rules are not allowed inside of containers";

fn typst_type_name(value: &Value) -> &'static str {
    match value {
        Value::Bool(_) => "boolean",
        Value::Int(_) => "integer",
        Value::Str(_) => "string",
        Value::None
        | Value::Float(_)
        | Value::Array(_)
        | Value::Dict(_)
        | Value::Module(_)
        | Value::Datetime(_)
        | Value::Func(_)
        | Value::Content(_)
        | Value::LocatedContent(_, _)
        | Value::Auto
        | Value::Length(_)
        | Value::Relative(_)
        | Value::Ratio(_)
        | Value::Angle(_)
        | Value::Color(_)
        | Value::Stroke(_)
        | Value::Fraction(_)
        | Value::Align(_)
        | Value::Location(_)
        | Value::Gradient(_)
        | Value::Regex(_)
        | Value::Tiling(_)
        | Value::Bytes(_)
        | Value::Decimal(_)
        | Value::Duration(_)
        | Value::Version(_)
        | Value::Selector(_)
        | Value::Symbol(_)
        | Value::Args(_)
        | Value::State(_)
        | Value::Counter(_)
        | Value::Label(_)
        | Value::Dir(_)
        | Value::Path(_)
        | Value::Type(_)
        | Value::SpotColorant(_) => value.type_name(),
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

fn explicit_pdf_tagged_values(styles: &Styles) -> impl Iterator<Item = &Value> {
    styles
        .delta()
        .custom
        .iter()
        .filter_map(|(key, value)| (key.as_str() == PDF_TAGGED_KEY).then_some(value))
}

fn explicit_pdf_pretty_values(styles: &Styles) -> impl Iterator<Item = &Value> {
    styles
        .delta()
        .custom
        .iter()
        .filter_map(|(key, value)| (key.as_str() == PDF_PRETTY_KEY).then_some(value))
}

fn explicit_pdf_pages_values(styles: &Styles) -> impl Iterator<Item = &Value> {
    styles
        .delta()
        .custom
        .iter()
        .filter_map(|(key, value)| (key.as_str() == PDF_PAGES_KEY).then_some(value))
}

fn explicit_pdf_standard_values(styles: &Styles) -> impl Iterator<Item = &Value> {
    styles
        .delta()
        .custom
        .iter()
        .filter_map(|(key, value)| (key.as_str() == PDF_STANDARD_KEY).then_some(value))
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

fn validate_pdf_tagged(value: &Value) -> SourceResult<PdfTagged> {
    match value {
        Value::Auto => Ok(PdfTagged::Auto),
        Value::Bool(value) => Ok(PdfTagged::Explicit(*value)),
        other => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!(
                "expected boolean or auto, found {} in internal {PDF_TAGGED_KEY}",
                typst_type_name(other)
            ),
        )]),
    }
}

fn validate_pdf_pretty(value: &Value) -> SourceResult<bool> {
    match value {
        Value::Bool(value) => Ok(*value),
        other => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!(
                "expected boolean, found {} in internal {PDF_PRETTY_KEY}",
                typst_type_name(other)
            ),
        )]),
    }
}

/// **P1468** — converte um `Value::Int`/`Dict` já validado por
/// `compiler/eval/rules.rs` (braço `format.pdf`) num `PageRange` tipado.
/// A validação de domínio (inteiro positivo, chaves `from`/`to`, `from <=
/// to`) já ocorreu no `set`; aqui um valor fora do domínio é estado interno
/// impossível, falha fechada, nunca um novo diagnóstico de schema.
fn pdf_page_number_from_i64(nr: i64) -> SourceResult<NonZeroUsize> {
    if nr < 1 {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("expected positive integer, found {nr} in internal {PDF_PAGES_KEY}"),
        )]);
    }
    usize::try_from(nr).ok().and_then(NonZeroUsize::new).ok_or_else(|| {
        vec![SourceDiagnostic::error(
            Span::detached(),
            format!("expected positive integer, found {nr} in internal {PDF_PAGES_KEY}"),
        )]
    })
}

fn pdf_page_range_from_value(value: &Value) -> SourceResult<PageRange> {
    match value {
        Value::Int(nr) => Ok(PageRange::at(pdf_page_number_from_i64(*nr)?)),
        Value::Dict(dict) => {
            let parse_bound = |key: &str| -> SourceResult<Option<NonZeroUsize>> {
                match dict.get(key) {
                    None => Ok(None),
                    Some(Value::Int(nr)) => Ok(Some(pdf_page_number_from_i64(*nr)?)),
                    Some(other) => Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        format!(
                            "expected integer, found {} in internal {PDF_PAGES_KEY}",
                            typst_type_name(other)
                        ),
                    )]),
                }
            };
            let start = parse_bound("from")?;
            let end = parse_bound("to")?;
            Ok(PageRange { start, end })
        }
        other => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!(
                "expected integer or dictionary, found {} in internal {PDF_PAGES_KEY}",
                typst_type_name(other)
            ),
        )]),
    }
}

/// **P1468** — extração root de `format.pdf.pages`: `Value::None` reafirma
/// "todas as páginas"; `Int`/`Dict`/`Array` (incluindo array vazio, seleção
/// vazia explícita — distinta de `None`) convertem para `PageRanges`
/// tipado. Não normaliza, deduplica nem ordena a lista (ver
/// `entities/document_format_options.md` P1468).
fn validate_pdf_pages(value: &Value) -> SourceResult<Option<PageRanges>> {
    match value {
        Value::None => Ok(None),
        Value::Int(_) | Value::Dict(_) => {
            Ok(Some(PageRanges::new(vec![pdf_page_range_from_value(value)?])))
        }
        Value::Array(items) => {
            let mut ranges = Vec::with_capacity(items.len());
            for item in items {
                ranges.push(pdf_page_range_from_value(item)?);
            }
            Ok(Some(PageRanges::new(ranges)))
        }
        other => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!(
                "expected integer, dictionary, array, or none, found {} in internal {PDF_PAGES_KEY}",
                typst_type_name(other)
            ),
        )]),
    }
}

/// **P1469** (escopo reduzido — contrato de linguagem) — reconstrói
/// `PdfStandards` a partir do `Value::Array` já validado em
/// `compiler/eval/rules.rs` (braço `format.pdf`, named `standard`). O array
/// é sempre a única forma inserida (string única já normalizada para array
/// de um elemento, `Value::None` nunca aceito) — qualquer desvio aqui é
/// estado interno impossível, falha fechada com diagnóstico atribuível,
/// nunca retorna a `None` nem ao default (ver `compiler/layout/format_options.md`
/// P1469).
fn validate_pdf_standard(value: &Value) -> SourceResult<PdfStandards> {
    let Value::Array(items) = value else {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!(
                "expected array, found {} in internal {PDF_STANDARD_KEY}",
                typst_type_name(value)
            ),
        )]);
    };

    let mut names = Vec::with_capacity(items.len());
    for item in items {
        let Value::Str(name) = item else {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "expected string, found {} in internal {PDF_STANDARD_KEY}",
                    typst_type_name(item)
                ),
            )]);
        };
        let Some(standard) = pdf_standard_from_name(name.as_str()) else {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("unrecognized PDF standard name in internal {PDF_STANDARD_KEY}"),
            )]);
        };
        names.push(standard);
    }

    PdfStandards::new(names).map_err(|_| {
        vec![SourceDiagnostic::error(
            Span::detached(),
            format!("conflicting PDF standards in internal {PDF_STANDARD_KEY}"),
        )]
    })
}

fn collect_root_options(
    content: &Content,
    options: &mut DocumentFormatOptions,
    root_occurrences: &mut usize,
) -> SourceResult<()> {
    match content.source_body() {
        Content::Sequence(sequence) => {
            for child in sequence.iter() {
                collect_root_options(child, options, root_occurrences)?;
            }
        }
        Content::Styled(styled) => {
            let body = &styled.body;
            let styles = &styled.styles;
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
            for value in explicit_pdf_tagged_values(styles) {
                options.pdf.tagged = validate_pdf_tagged(value)?;
                *root_occurrences += 1;
            }
            for value in explicit_pdf_pretty_values(styles) {
                options.pdf.pretty = validate_pdf_pretty(value)?;
                *root_occurrences += 1;
            }
            for value in explicit_pdf_pages_values(styles) {
                options.pdf.pages = validate_pdf_pages(value)?;
                *root_occurrences += 1;
            }
            for value in explicit_pdf_standard_values(styles) {
                options.pdf.standard = Some(validate_pdf_standard(value)?);
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
        if let Content::Styled(styled) = node {
            let styles = &styled.styles;
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
            for value in explicit_pdf_tagged_values(styles) {
                validate_pdf_tagged(value)?;
                all_occurrences += 1;
            }
            for value in explicit_pdf_pretty_values(styles) {
                validate_pdf_pretty(value)?;
                all_occurrences += 1;
            }
            for value in explicit_pdf_pages_values(styles) {
                validate_pdf_pages(value)?;
                all_occurrences += 1;
            }
            for value in explicit_pdf_standard_values(styles) {
                validate_pdf_standard(value)?;
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

    #[test]
    fn v16_value_projection_format_diagnostics_keep_long_aliases() {
        for (value, expected) in [
            (Value::Bool(false), "boolean"),
            (Value::Int(0), "integer"),
            (Value::Str("x".into()), "string"),
            (Value::None, "none"),
            (Value::Auto, "auto"),
            (Value::Float(1.5), "float"),
            (Value::Array(vec![]), "array"),
            (Value::Content(Content::Empty), "content"),
            (Value::Length(crate::entities::layout_types::Length::pt(3.0)), "length"),
            (Value::Type(crate::entities::value::Type::Int), "type"),
        ] {
            assert_eq!(typst_type_name(&value), expected);
        }
    }

    fn svg_pretty(body: Content, value: Value) -> Content {
        Content::styled(Box::new(body), Styles::new().push_custom(SVG_PRETTY_KEY, value))
    }

    fn html_pretty(body: Content, value: Value) -> Content {
        Content::styled(Box::new(body), Styles::new().push_custom(HTML_PRETTY_KEY, value))
    }

    #[test]
    fn p1462_absence_and_unrelated_styles_keep_default_false() {
        let content = Content::styled(
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
        let content = Content::styled(
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

    // ── P1467 (RED — Agente B) — extração root de `format.pdf.tagged`/
    // `format.pdf.pretty`. `DocumentFormatOptions::pdf` e `PdfTagged` ainda
    // não existem em `entities/document_format_options.rs` neste baseline;
    // este módulo falha por erro de compilação até o carrier P1467 (mesmo
    // gate ADR-0127) ser materializado por outro agente em paralelo.
    use crate::entities::document_format_options::PdfTagged;

    const PDF_TAGGED_KEY: &str = "format.pdf.tagged";
    const PDF_PRETTY_KEY: &str = "format.pdf.pretty";

    fn pdf_tagged(body: Content, value: Value) -> Content {
        Content::styled(Box::new(body), Styles::new().push_custom(PDF_TAGGED_KEY, value))
    }

    fn pdf_pretty(body: Content, value: Value) -> Content {
        Content::styled(Box::new(body), Styles::new().push_custom(PDF_PRETTY_KEY, value))
    }

    #[test]
    fn p1467_absence_keeps_default_auto_tagged_and_compact_pretty() {
        let content = Content::styled(
            Box::new(Content::text("body")),
            Styles::new().push_custom("text.example", Value::Bool(true)),
        );

        let options = collect_document_format_options(&content).unwrap();

        assert_eq!(options.pdf.tagged, PdfTagged::Auto);
        assert!(!options.pdf.pretty);
    }

    #[test]
    fn p1467_tagged_root_sequence_and_styled_use_lexical_last_write_wins() {
        let content = pdf_tagged(
            Content::sequence(vec![
                Content::text("before"),
                pdf_tagged(Content::text("after"), Value::Bool(false)),
            ]),
            Value::Bool(true),
        );
        assert_eq!(
            collect_document_format_options(&content).unwrap().pdf.tagged,
            PdfTagged::Explicit(false)
        );

        // Última decisão pode transitar de explícito de volta a `auto`.
        let reversed = Content::sequence(vec![
            pdf_tagged(Content::Empty, Value::Bool(false)),
            pdf_tagged(Content::Empty, Value::Auto),
        ]);
        assert_eq!(
            collect_document_format_options(&reversed).unwrap().pdf.tagged,
            PdfTagged::Auto
        );
    }

    #[test]
    fn p1467_pretty_root_sequence_and_styled_use_lexical_last_write_wins() {
        let content = pdf_pretty(
            Content::sequence(vec![
                Content::text("before"),
                pdf_pretty(Content::text("after"), Value::Bool(false)),
            ]),
            Value::Bool(true),
        );
        assert!(!collect_document_format_options(&content).unwrap().pdf.pretty);

        let reversed = Content::sequence(vec![
            pdf_pretty(Content::Empty, Value::Bool(false)),
            pdf_pretty(Content::Empty, Value::Bool(true)),
        ]);
        assert!(collect_document_format_options(&reversed).unwrap().pdf.pretty);
    }

    #[test]
    fn p1467_styled_without_pdf_keys_is_transparent() {
        let tagged_content = Content::styled(
            Box::new(pdf_tagged(Content::Empty, Value::Bool(false))),
            Styles::new().push_custom("other.key", Value::Bool(true)),
        );
        assert_eq!(
            collect_document_format_options(&tagged_content).unwrap().pdf.tagged,
            PdfTagged::Explicit(false)
        );

        let pretty_content = Content::styled(
            Box::new(pdf_pretty(Content::Empty, Value::Bool(true))),
            Styles::new().push_custom("other.key", Value::Bool(false)),
        );
        assert!(collect_document_format_options(&pretty_content).unwrap().pdf.pretty);
    }

    #[test]
    fn p1467_pdf_rule_in_distinct_containers_fails_closed() {
        for nested in [
            Content::Par {
                body: Box::new(pdf_tagged(Content::Empty, Value::Bool(true))),
            },
            Content::SmallCaps {
                body: Box::new(pdf_pretty(Content::Empty, Value::Bool(false))),
            },
        ] {
            let errors = collect_document_format_options(&nested).unwrap_err();
            assert_eq!(errors[0].message, NESTED_FORMAT_RULE_ERROR);
        }
    }

    #[test]
    fn p1467_tagged_non_auto_non_bool_internal_value_fails_closed() {
        let content = pdf_tagged(Content::Empty, Value::Int(1));

        let errors = collect_document_format_options(&content).unwrap_err();

        assert!(errors[0].message.contains("expected boolean or auto"));
        assert!(errors[0].message.contains("integer"));
        assert!(errors[0].message.contains(PDF_TAGGED_KEY));
    }

    #[test]
    fn p1467_pretty_non_boolean_internal_value_fails_closed() {
        let content = pdf_pretty(Content::Empty, Value::Int(1));

        let errors = collect_document_format_options(&content).unwrap_err();

        assert!(errors[0].message.contains("expected boolean, found integer"));
        assert!(errors[0].message.contains(PDF_PRETTY_KEY));
    }

    #[test]
    fn p1467_pdf_is_orthogonal_to_svg_png_html_in_the_same_tree() {
        let content = Content::sequence(vec![
            svg_pretty(Content::Empty, Value::Bool(true)),
            html_pretty(Content::Empty, Value::Bool(true)),
            Content::styled(
                Box::new(Content::Empty),
                Styles::new().push_custom(PNG_PPI_KEY, Value::Float(300.0)),
            ),
            pdf_tagged(Content::Empty, Value::Bool(false)),
            pdf_pretty(Content::Empty, Value::Bool(true)),
        ]);

        let options = collect_document_format_options(&content).unwrap();
        assert!(options.svg.pretty);
        assert!(options.html.pretty);
        assert_eq!(options.png.ppi, 300.0);
        assert_eq!(options.pdf.tagged, PdfTagged::Explicit(false));
        assert!(options.pdf.pretty);
    }

    #[test]
    fn p1467_pdf_tagged_and_pretty_fields_are_independent() {
        let only_tagged = pdf_tagged(Content::Empty, Value::Bool(false));
        let options = collect_document_format_options(&only_tagged).unwrap();
        assert_eq!(options.pdf.tagged, PdfTagged::Explicit(false));
        assert!(!options.pdf.pretty);

        let only_pretty = pdf_pretty(Content::Empty, Value::Bool(true));
        let options = collect_document_format_options(&only_pretty).unwrap();
        assert_eq!(options.pdf.tagged, PdfTagged::Auto);
        assert!(options.pdf.pretty);

        // Regras distintas (`Styled` aninhados): tagged fora, pretty dentro —
        // cada campo resolve pela sua própria chave, sem se pisarem.
        let both_in_distinct_rules =
            pdf_tagged(pdf_pretty(Content::Empty, Value::Bool(true)), Value::Bool(false));
        let options = collect_document_format_options(&both_in_distinct_rules).unwrap();
        assert_eq!(options.pdf.tagged, PdfTagged::Explicit(false));
        assert!(options.pdf.pretty);
    }

    // ── P1468 (RED — Agente B) — extração root de `format.pdf.pages`.
    // `DocumentFormatOptions::pdf.pages`, `PageRange` e `PageRanges` ainda
    // não existem neste baseline (`0b322a926dd2f86b49aabfeb0d2f287ce06aaace`);
    // este módulo falha por erro de compilação até o carrier P1468 (mesmo
    // gate ADR-0127) ser materializado por outro agente em paralelo.
    use crate::entities::document_format_options::{PageRange, PageRanges};
    use ecow::EcoString;
    use indexmap::IndexMap;
    use rustc_hash::FxBuildHasher;
    use std::num::NonZeroUsize;

    const PDF_PAGES_KEY: &str = "format.pdf.pages";

    fn page(n: usize) -> NonZeroUsize {
        NonZeroUsize::new(n).unwrap()
    }

    fn single_range(n: usize) -> PageRanges {
        PageRanges::new(vec![PageRange { start: Some(page(n)), end: Some(page(n)) }])
    }

    fn dict_value(pairs: &[(&str, i64)]) -> Value {
        let mut map: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
        for (key, value) in pairs {
            map.insert((*key).into(), Value::Int(*value));
        }
        Value::Dict(map)
    }

    fn pdf_pages(body: Content, value: Value) -> Content {
        Content::styled(Box::new(body), Styles::new().push_custom(PDF_PAGES_KEY, value))
    }

    #[test]
    fn p1468_absence_keeps_default_none_pages() {
        let content = Content::styled(
            Box::new(Content::text("body")),
            Styles::new().push_custom("text.example", Value::Bool(true)),
        );

        let options = collect_document_format_options(&content).unwrap();

        assert_eq!(options.pdf.pages, None);
    }

    #[test]
    fn p1468_pages_integer_root_extraction() {
        let content = pdf_pages(Content::Empty, Value::Int(3));
        let options = collect_document_format_options(&content).unwrap();
        assert_eq!(options.pdf.pages, Some(single_range(3)));
    }

    #[test]
    fn p1468_pages_dict_open_and_closed_root_extraction() {
        let open_end = pdf_pages(Content::Empty, dict_value(&[("from", 2)]));
        let options = collect_document_format_options(&open_end).unwrap();
        assert_eq!(
            options.pdf.pages,
            Some(PageRanges::new(vec![PageRange { start: Some(page(2)), end: None }]))
        );

        let open_start = pdf_pages(Content::Empty, dict_value(&[("to", 2)]));
        let options = collect_document_format_options(&open_start).unwrap();
        assert_eq!(
            options.pdf.pages,
            Some(PageRanges::new(vec![PageRange { start: None, end: Some(page(2)) }]))
        );

        let closed = pdf_pages(Content::Empty, dict_value(&[("from", 1), ("to", 3)]));
        let options = collect_document_format_options(&closed).unwrap();
        assert_eq!(
            options.pdf.pages,
            Some(PageRanges::new(vec![PageRange {
                start: Some(page(1)),
                end: Some(page(3))
            }]))
        );
    }

    #[test]
    fn p1468_pages_array_mixed_and_empty_root_extraction() {
        let mixed = pdf_pages(
            Content::Empty,
            Value::Array(vec![Value::Int(1), dict_value(&[("from", 3), ("to", 5)])]),
        );
        let options = collect_document_format_options(&mixed).unwrap();
        assert_eq!(
            options.pdf.pages,
            Some(PageRanges::new(vec![
                PageRange { start: Some(page(1)), end: Some(page(1)) },
                PageRange { start: Some(page(3)), end: Some(page(5)) },
            ]))
        );

        // Array vazio é presente (seleção vazia), distinto de `None`.
        let empty = pdf_pages(Content::Empty, Value::Array(vec![]));
        let options = collect_document_format_options(&empty).unwrap();
        assert_eq!(options.pdf.pages, Some(PageRanges::new(vec![])));
        assert_ne!(options.pdf.pages, None);
    }

    #[test]
    fn p1468_pages_empty_rule_after_previous_decision_preserves_it() {
        // Regra externa define `pages`; regra interna só toca uma chave não
        // relacionada — não deve apagar a decisão de `pages`.
        let content = pdf_pages(
            Content::styled(
                Box::new(Content::text("after")),
                Styles::new().push_custom("other.key", Value::Bool(true)),
            ),
            Value::Int(2),
        );
        let options = collect_document_format_options(&content).unwrap();
        assert_eq!(options.pdf.pages, Some(single_range(2)));
    }

    #[test]
    fn p1468_pages_none_value_reaffirms_none_after_prior_decision() {
        // `Value::None` é uma decisão EXPLÍCITA ("todas as páginas"),
        // distinta de nunca ter existido a chave — e, como qualquer outra
        // decisão root, a mais lexicalmente interna/posterior vence.
        let content = pdf_pages(pdf_pages(Content::Empty, Value::None), Value::Int(2));
        let options = collect_document_format_options(&content).unwrap();
        assert_eq!(options.pdf.pages, None);
    }

    #[test]
    fn p1468_pages_root_sequence_and_styled_use_lexical_last_write_wins() {
        let content = pdf_pages(
            Content::sequence(vec![
                Content::text("before"),
                pdf_pages(Content::text("after"), Value::Int(5)),
            ]),
            Value::Int(2),
        );
        assert_eq!(
            collect_document_format_options(&content).unwrap().pdf.pages,
            Some(single_range(5))
        );

        let reversed = Content::sequence(vec![
            pdf_pages(Content::Empty, Value::Int(5)),
            pdf_pages(Content::Empty, Value::Int(2)),
        ]);
        assert_eq!(
            collect_document_format_options(&reversed).unwrap().pdf.pages,
            Some(single_range(2))
        );
    }

    #[test]
    fn p1468_pdf_pages_rule_in_distinct_containers_fails_closed() {
        for nested in [
            Content::Par {
                body: Box::new(pdf_pages(Content::Empty, Value::Int(1))),
            },
            Content::SmallCaps {
                body: Box::new(pdf_pages(Content::Empty, dict_value(&[("from", 1)]))),
            },
        ] {
            let errors = collect_document_format_options(&nested).unwrap_err();
            assert_eq!(errors[0].message, NESTED_FORMAT_RULE_ERROR);
        }
    }

    #[test]
    fn p1468_pages_composition_with_svg_png_html_tagged_pretty() {
        let content = Content::sequence(vec![
            svg_pretty(Content::Empty, Value::Bool(true)),
            html_pretty(Content::Empty, Value::Bool(true)),
            Content::styled(
                Box::new(Content::Empty),
                Styles::new().push_custom(PNG_PPI_KEY, Value::Float(300.0)),
            ),
            pdf_tagged(Content::Empty, Value::Bool(false)),
            pdf_pretty(Content::Empty, Value::Bool(true)),
            pdf_pages(Content::Empty, dict_value(&[("from", 2), ("to", 4)])),
        ]);

        let options = collect_document_format_options(&content).unwrap();
        assert!(options.svg.pretty);
        assert!(options.html.pretty);
        assert_eq!(options.png.ppi, 300.0);
        assert_eq!(options.pdf.tagged, PdfTagged::Explicit(false));
        assert!(options.pdf.pretty);
        assert_eq!(
            options.pdf.pages,
            Some(PageRanges::new(vec![PageRange {
                start: Some(page(2)),
                end: Some(page(4))
            }]))
        );
    }

    #[test]
    fn p1468_pages_field_is_independent_of_tagged_and_pretty() {
        let only_pages = pdf_pages(Content::Empty, Value::Int(7));
        let options = collect_document_format_options(&only_pages).unwrap();
        assert_eq!(options.pdf.pages, Some(single_range(7)));
        assert_eq!(options.pdf.tagged, PdfTagged::Auto);
        assert!(!options.pdf.pretty);
    }

    #[test]
    fn p1468_pages_internal_type_impossible_fails_closed() {
        // `format.pdf.pages` já foi validado em `compiler/eval/rules.rs`
        // (só Int/Dict/Array/None chegam à StyleChain); um `Value::Bool`
        // interno é estado impossível e deve falhar fechado, nunca cair de
        // volta a `None`. Não fixamos aqui a forma exata da mensagem de
        // topo (isso é `compiler/eval/rules.md`); só que a falha é fechada,
        // atribuível e cita a chave canônica.
        let content = pdf_pages(Content::Empty, Value::Bool(true));

        let errors = collect_document_format_options(&content).unwrap_err();

        assert!(errors[0].message.contains(PDF_PAGES_KEY), "{errors:?}");
        assert!(
            errors[0].message.contains("boolean"),
            "mensagem deve nomear o tipo interno encontrado: {errors:?}"
        );
    }

    // ── P1469 (RED — Agente B) — extração root de `format.pdf.standard`
    // (contrato de linguagem, escopo reduzido — ver
    // `entities/document_format_options.md` P1469 e
    // `compiler/layout/format_options.md` P1469). `PdfStandard`/
    // `PdfStandards` e `DocumentFormatOptions::pdf.standard` ainda não
    // existem neste baseline (`65e68ad45dcbfd8db00aeda78d0d2fbaaadefb27`);
    // este módulo falha por erro de compilação até o carrier P1469 (mesmo
    // gate ADR-0127) ser materializado por outro agente em paralelo.
    use crate::entities::document_format_options::{PdfStandard, PdfStandards};

    const PDF_STANDARD_KEY: &str = "format.pdf.standard";

    /// Mapeamento nome bruto → `PdfStandard`, só para montar valores
    /// esperados nos testes (a reconstrução real é responsabilidade do
    /// owner, não deste módulo de testes).
    fn standard(name: &str) -> PdfStandard {
        match name {
            "1.4" => PdfStandard::V1_4,
            "1.5" => PdfStandard::V1_5,
            "1.6" => PdfStandard::V1_6,
            "1.7" => PdfStandard::V1_7,
            "2.0" => PdfStandard::V2_0,
            "a-1b" => PdfStandard::A1b,
            "a-1a" => PdfStandard::A1a,
            "a-2b" => PdfStandard::A2b,
            "a-2u" => PdfStandard::A2u,
            "a-2a" => PdfStandard::A2a,
            "a-3b" => PdfStandard::A3b,
            "a-3u" => PdfStandard::A3u,
            "a-3a" => PdfStandard::A3a,
            "a-4" => PdfStandard::A4,
            "a-4f" => PdfStandard::A4f,
            "a-4e" => PdfStandard::A4e,
            "ua-1" => PdfStandard::Ua1,
            other => panic!("nome desconhecido no teste: {other}"),
        }
    }

    fn standards_value(names: &[&str]) -> Value {
        Value::Array(names.iter().map(|n| Value::Str((*n).into())).collect())
    }

    fn standards(names: &[&str]) -> PdfStandards {
        PdfStandards::new(names.iter().map(|n| standard(n)))
            .expect("combinação de teste deve ser válida")
    }

    fn pdf_standard(body: Content, value: Value) -> Content {
        Content::styled(
            Box::new(body),
            Styles::new().push_custom(PDF_STANDARD_KEY, value),
        )
    }

    #[test]
    fn p1469_absence_keeps_default_none_standard() {
        let content = Content::styled(
            Box::new(Content::text("body")),
            Styles::new().push_custom("text.example", Value::Bool(true)),
        );

        let options = collect_document_format_options(&content).unwrap();

        assert_eq!(options.pdf.standard, None);
    }

    #[test]
    fn p1469_standard_array_single_name_root_extraction() {
        let content = pdf_standard(Content::Empty, standards_value(&["a-2b"]));
        let options = collect_document_format_options(&content).unwrap();
        assert_eq!(options.pdf.standard, Some(standards(&["a-2b"])));
    }

    #[test]
    fn p1469_standard_array_multiple_names_root_extraction() {
        // versão + archival
        let version_and_archival =
            pdf_standard(Content::Empty, standards_value(&["1.4", "a-1b"]));
        let options = collect_document_format_options(&version_and_archival).unwrap();
        assert_eq!(options.pdf.standard, Some(standards(&["1.4", "a-1b"])));

        // versão + accessibility
        let version_and_accessibility =
            pdf_standard(Content::Empty, standards_value(&["1.7", "ua-1"]));
        let options =
            collect_document_format_options(&version_and_accessibility).unwrap();
        assert_eq!(options.pdf.standard, Some(standards(&["1.7", "ua-1"])));

        // archival + accessibility compatíveis (interseção de range
        // 1.4..=1.7 não vazia entre `a-2b` e `ua-1`)
        let archival_and_accessibility =
            pdf_standard(Content::Empty, standards_value(&["a-2b", "ua-1"]));
        let options =
            collect_document_format_options(&archival_and_accessibility).unwrap();
        assert_eq!(options.pdf.standard, Some(standards(&["a-2b", "ua-1"])));
    }

    #[test]
    fn p1469_standard_empty_array_is_some_default_distinct_from_none() {
        let content = pdf_standard(Content::Empty, Value::Array(vec![]));
        let options = collect_document_format_options(&content).unwrap();
        assert_eq!(options.pdf.standard, Some(PdfStandards::default()));
        assert_ne!(options.pdf.standard, None);
    }

    #[test]
    fn p1469_standard_empty_rule_after_previous_decision_preserves_it() {
        let content = pdf_standard(
            Content::styled(
                Box::new(Content::text("after")),
                Styles::new().push_custom("other.key", Value::Bool(true)),
            ),
            standards_value(&["a-2b"]),
        );
        let options = collect_document_format_options(&content).unwrap();
        assert_eq!(options.pdf.standard, Some(standards(&["a-2b"])));
    }

    #[test]
    fn p1469_standard_root_sequence_and_styled_use_lexical_last_write_wins_and_repeats() {
        let content = pdf_standard(
            Content::sequence(vec![
                Content::text("before"),
                pdf_standard(Content::text("after"), standards_value(&["a-4"])),
            ]),
            standards_value(&["a-2b"]),
        );
        assert_eq!(
            collect_document_format_options(&content).unwrap().pdf.standard,
            Some(standards(&["a-4"]))
        );

        // Inversão lexical: a última decisão vence, seja qual for.
        let reversed = Content::sequence(vec![
            pdf_standard(Content::Empty, standards_value(&["a-4"])),
            pdf_standard(Content::Empty, standards_value(&["a-2b"])),
        ]);
        assert_eq!(
            collect_document_format_options(&reversed).unwrap().pdf.standard,
            Some(standards(&["a-2b"]))
        );

        // Repetição da mesma decisão: continua estável.
        let repeated = Content::sequence(vec![
            pdf_standard(Content::Empty, standards_value(&["a-2b"])),
            pdf_standard(Content::Empty, standards_value(&["a-2b"])),
        ]);
        assert_eq!(
            collect_document_format_options(&repeated).unwrap().pdf.standard,
            Some(standards(&["a-2b"]))
        );
    }

    #[test]
    fn p1469_pdf_standard_rule_in_distinct_containers_fails_closed() {
        for nested in [
            Content::Par {
                body: Box::new(pdf_standard(Content::Empty, standards_value(&["a-2b"]))),
            },
            Content::SmallCaps {
                body: Box::new(pdf_standard(Content::Empty, standards_value(&["1.4"]))),
            },
        ] {
            let errors = collect_document_format_options(&nested).unwrap_err();
            assert_eq!(errors[0].message, NESTED_FORMAT_RULE_ERROR);
        }
    }

    #[test]
    fn p1469_standard_composition_with_svg_png_html_tagged_pretty_pages() {
        let content = Content::sequence(vec![
            svg_pretty(Content::Empty, Value::Bool(true)),
            html_pretty(Content::Empty, Value::Bool(true)),
            Content::styled(
                Box::new(Content::Empty),
                Styles::new().push_custom(PNG_PPI_KEY, Value::Float(300.0)),
            ),
            pdf_tagged(Content::Empty, Value::Bool(false)),
            pdf_pretty(Content::Empty, Value::Bool(true)),
            pdf_pages(Content::Empty, dict_value(&[("from", 2), ("to", 4)])),
            pdf_standard(Content::Empty, standards_value(&["a-2b"])),
        ]);

        let options = collect_document_format_options(&content).unwrap();
        assert!(options.svg.pretty);
        assert!(options.html.pretty);
        assert_eq!(options.png.ppi, 300.0);
        assert_eq!(options.pdf.tagged, PdfTagged::Explicit(false));
        assert!(options.pdf.pretty);
        assert_eq!(
            options.pdf.pages,
            Some(PageRanges::new(vec![PageRange {
                start: Some(page(2)),
                end: Some(page(4))
            }]))
        );
        assert_eq!(options.pdf.standard, Some(standards(&["a-2b"])));
    }

    #[test]
    fn p1469_standard_field_is_independent_of_tagged_pretty_and_pages() {
        let only_standard = pdf_standard(Content::Empty, standards_value(&["ua-1"]));
        let options = collect_document_format_options(&only_standard).unwrap();
        assert_eq!(options.pdf.standard, Some(standards(&["ua-1"])));
        assert_eq!(options.pdf.tagged, PdfTagged::Auto);
        assert!(!options.pdf.pretty);
        assert_eq!(options.pdf.pages, None);
    }

    #[test]
    fn p1469_standard_internal_type_impossible_fails_closed() {
        // `format.pdf.standard` já foi validado em `compiler/eval/rules.rs`
        // (só `Value::Array` de `Value::Str` chega à StyleChain); um
        // elemento `Value::Int` interno é estado impossível e deve falhar
        // fechado, nunca cair de volta a `None`. Não fixamos aqui a forma
        // exata da mensagem de topo (isso é `compiler/eval/rules.md`); só
        // que a falha é fechada, atribuível e cita a chave canônica.
        let content = pdf_standard(Content::Empty, Value::Array(vec![Value::Int(1)]));

        let errors = collect_document_format_options(&content).unwrap_err();

        assert!(errors[0].message.contains(PDF_STANDARD_KEY), "{errors:?}");
        assert!(
            errors[0].message.contains("integer") || errors[0].message.contains("string"),
            "mensagem deve nomear o tipo interno inesperado: {errors:?}"
        );
    }
}
