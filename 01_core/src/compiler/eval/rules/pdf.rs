//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/rules/pdf.md
//! @prompt-hash 437abf55
//! @layer L1
//! @updated 2026-09-30
//!
//! Comportamento de `eval/rules::pdf`.

use super::*;

/// **P1468** — nome do tipo no vocabulário do vanilla para as mensagens de
/// `format.pdf(pages: ...)` (mesmo mapeamento de `expected_length_error`/
/// `edge_cast_error`: `int`/`str`/`bool` → `integer`/`string`/`boolean`).
pub(crate) fn pdf_pages_found_name(value: &Value) -> &'static str {
    match value.type_name() {
        "int" => "integer",
        "str" => "string",
        "bool" => "boolean",
        other => other,
    }
}

/// **P1468** — valida um candidato a página única (`Value::Int`) ou
/// intervalo (`Value::Dict` com `from`/`to`), tanto no topo de
/// `format.pdf(pages: ...)` quanto como elemento de `Value::Array` (cast
/// recursivo de um nível, `typst-library/src/layout/page.rs:806-847` do
/// vanilla ratificado). Não constrói o tipo tipado `PageRange` — isso é
/// `compiler/layout/format_options.rs`; aqui só valida e preserva o
/// `Value` original para inserção no `Styles`.
pub(crate) fn validate_pdf_page_range_element(
    value: &Value,
    span: Span,
) -> SourceResult<()> {
    match value {
        Value::Int(nr) => {
            if *nr < 1 {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    "page numbers start at one",
                )]);
            }
            Ok(())
        }
        Value::Dict(dict) => {
            if let Some(key) =
                dict.keys().find(|k| k.as_str() != "from" && k.as_str() != "to")
            {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    format!(
                        "unexpected key \"{key}\", valid keys are \"from\" and \"to\""
                    ),
                )]);
            }
            let from = dict.get("from");
            let to = dict.get("to");
            if from.is_none() && to.is_none() {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    "page range must not be empty",
                )]);
            }
            let parse_bound = |bound: Option<&Value>| -> SourceResult<Option<i64>> {
                match bound {
                    None => Ok(None),
                    Some(Value::Int(nr)) => {
                        if *nr < 1 {
                            return Err(vec![SourceDiagnostic::error(
                                span,
                                "page numbers start at one",
                            )]);
                        }
                        Ok(Some(*nr))
                    }
                    Some(other) => Err(vec![SourceDiagnostic::error(
                        span,
                        format!(
                            "expected integer, found {}",
                            pdf_pages_found_name(other)
                        ),
                    )]),
                }
            };
            let from_nr = parse_bound(from)?;
            let to_nr = parse_bound(to)?;
            if let (Some(f), Some(t)) = (from_nr, to_nr) {
                if f > t {
                    return Err(vec![SourceDiagnostic::error(
                        span,
                        "start of page range cannot be larger than end",
                    )]);
                }
            }
            Ok(())
        }
        other => Err(vec![SourceDiagnostic::error(
            span,
            format!(
                "expected integer or dictionary, found {}",
                pdf_pages_found_name(other)
            ),
        )]),
    }
}

/// **P1469** (escopo reduzido — contrato de linguagem) — lista de opções
/// formatada `"1.4", "1.5", ..., "ua-1"[, or array]`, medida em
/// `entities/document_format_options.md`/`compiler/eval/rules.md` P1469.
/// `include_array` acrescenta a opção `array` (só no topo de
/// `format.pdf(standard: ...)`, nunca dentro de um elemento).
pub(crate) fn pdf_standard_expected_list(include_array: bool) -> String {
    let mut items: Vec<String> = PDF_STANDARD_NAMES
        .iter()
        .map(|(name, _)| format!("\"{name}\""))
        .collect();
    if include_array {
        items.push("array".to_string());
    }
    match items.as_slice() {
        [] => String::new(),
        [only] => only.clone(),
        [rest @ .., last] => format!("{}, or {last}", rest.join(", ")),
    }
}

/// Junta rótulos de validators com "and" (dois itens, único caso alcançável
/// neste recorte — no máximo um archival e uma accessibility), espelhando
/// `typst-pdf/src/util.rs:145-160` (`ValidatorsExt::to_and_list`).
pub(crate) fn pdf_standard_and_list(labels: &[&str]) -> String {
    match labels {
        [] => String::new(),
        [only] => (*only).to_string(),
        [a, b] => format!("{a} and {b}"),
        [rest @ .., last] => format!("{}, and {last}", rest.join(", ")),
    }
}

/// "PDF 1.4"-style, usado só em texto de diagnóstico — distinto de
/// `PdfVersion::as_str()` (`"1.4"`, usado no header e no cast).
pub(crate) fn pdf_version_display(version: PdfVersion) -> String {
    format!("PDF {}", version.as_str())
}

/// Hint de compatibilidade de versão de um validator, espelhando
/// `typst-pdf/src/format.rs:684-701` (`version_hint`).
pub(crate) fn pdf_standard_version_hint(
    label: &str,
    min: PdfVersion,
    max: PdfVersion,
) -> String {
    if min == max {
        format!("{label} requires version {}", pdf_version_display(min))
    } else {
        format!(
            "{label} requires a version between {} and {}",
            pdf_version_display(min),
            pdf_version_display(max)
        )
    }
}

/// Nome de exibição de um `PdfValidator` (archival ou accessibility),
/// delegando ao `label()` do respetivo tipo L1.
pub(crate) fn pdf_validator_label(validator: PdfValidator) -> &'static str {
    match validator {
        PdfValidator::Archival(a) => a.label(),
        PdfValidator::Accessibility(ua) => ua.label(),
    }
}

/// Range min/max de um `PdfValidator`, delegando ao respetivo tipo L1.
pub(crate) fn pdf_validator_range(validator: PdfValidator) -> (PdfVersion, PdfVersion) {
    match validator {
        PdfValidator::Archival(a) => (a.min_version(), a.max_version()),
        PdfValidator::Accessibility(ua) => (ua.min_version(), ua.max_version()),
    }
}

/// **P1469** (retificado — ver `entities/document_format_options.md` P1469,
/// commit `e14002d55`: `NoOverlappingRange`/`VersionOutOfRange` passaram a
/// carregar `validators: Vec<PdfValidator>` na ordem de citação medida) —
/// formata o texto final de diagnóstico a partir do resultado estruturado
/// de `PdfStandards::new`. Mensagens espelham `krilla::configure::mod.rs:
/// 68-96` e `typst-pdf/src/format.rs:582-701` medidos no baseline do passo,
/// incluindo os dois casos com mais de um validator no mesmo erro
/// ("PDF/A-4 and PDF/UA-1 are mutually incompatible..."; "PDF 1.7 is not
/// compatible with PDF/A-1b and PDF/UA-1"). `pub(crate)` (não
/// módulo-privado): reaproveitado por `compiler/eval/mod.rs`
/// (`pdf_standards_conflict_diagnostic`), que expõe a mesma formatação ao
/// gate CLI-only de `04_wiring/src/main.rs` (`wiring.md` P1469: "mesmo
/// texto ... é a mesma função de validação, reaproveitada").
pub(crate) fn pdf_standards_conflict_diagnostic(
    span: Span,
    conflict: PdfStandardsConflict,
) -> SourceDiagnostic {
    fn with_validator_hints(
        mut diag: SourceDiagnostic,
        validators: &[PdfValidator],
    ) -> SourceDiagnostic {
        for validator in validators {
            let (min, max) = pdf_validator_range(*validator);
            diag = diag.with_hint(pdf_standard_version_hint(
                pdf_validator_label(*validator),
                min,
                max,
            ));
        }
        diag
    }

    match conflict {
        PdfStandardsConflict::ConflictingVersions(prev, new) => SourceDiagnostic::error(
            span,
            format!(
                "PDF cannot conform to {} and {} at the same time",
                pdf_version_display(prev),
                pdf_version_display(new)
            ),
        ),
        PdfStandardsConflict::MultipleArchival => {
            SourceDiagnostic::error(span, "choose at most one PDF/A standard")
        }
        PdfStandardsConflict::MultipleAccessibility => {
            SourceDiagnostic::error(span, "choose at most one PDF/UA standard")
        }
        PdfStandardsConflict::NoOverlappingRange { validators } => {
            let labels: Vec<&str> =
                validators.iter().map(|v| pdf_validator_label(*v)).collect();
            let diag = SourceDiagnostic::error(
                span,
                format!(
                    "{} are mutually incompatible because they do not have any overlapping PDF versions",
                    pdf_standard_and_list(&labels)
                ),
            );
            with_validator_hints(diag, &validators)
        }
        PdfStandardsConflict::VersionOutOfRange { version, validators } => {
            let labels: Vec<&str> =
                validators.iter().map(|v| pdf_validator_label(*v)).collect();
            let diag = SourceDiagnostic::error(
                span,
                format!(
                    "{} is not compatible with {}",
                    pdf_version_display(version),
                    pdf_standard_and_list(&labels)
                ),
            );
            with_validator_hints(diag, &validators)
        }
    }
}
