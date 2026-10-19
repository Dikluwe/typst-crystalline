//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/rules.md
//! @prompt-hash 291b80a4
//! @layer L1
//! @updated 2026-10-05
//!
//! Diagnósticos compartilhados e hub estático de set/show rules.
//! Os algoritmos residem nos módulos filhos por comportamento.

pub(crate) mod textual;

use std::sync::Arc;

use std::str::FromStr;

use crate::compiler::scopes::Scopes;
use crate::entities::args::Args;
use crate::entities::ast::code::{SetRule, ShowRule as ShowRuleNode};
use crate::entities::ast::expr::{Arg, ArrayItem, DictItem, Expr};
use crate::entities::ast::AstNode;
use crate::entities::content::Content;
use crate::entities::document_format_options::{
    pdf_standard_from_name, PdfStandard, PdfStandards, PdfStandardsConflict,
    PdfValidator, PdfVersion, PDF_STANDARD_NAMES,
};
use crate::entities::engine::Engine;
use crate::entities::font_book::FontWeight;
use crate::entities::lang::Lang;
use crate::entities::show::{NodeKind, Selector, ShowRule, Transformation};
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::style::Styles;
use crate::entities::style_chain::StyleChain;
use crate::entities::value::Value;
use crate::entities::world_types::check_show_depth as route_check_show_depth;
use ecow::EcoString;

use super::{
    call_dispatch, eval_expr, font_dict, selector_matching, show_rule_termination,
    EvalContext,
};

pub(crate) mod apply;
mod intercept;
mod math_set;
mod outline;
mod pdf;
mod realize;
mod set;
mod show;

pub(crate) use self::apply::apply_show_rules;
pub(crate) use self::intercept::intercept_content;
pub(crate) use self::intercept::intercept_labelled;
pub(crate) use self::intercept::intercept_paragraphs;
use self::math_set::eval_set_math_rule;
use self::math_set::unsupported_property_warn;
use self::math_set::unsupported_target_warn;
pub use self::outline::realize_outlines;
use self::pdf::pdf_pages_found_name;
use self::pdf::pdf_standard_expected_list;
pub(crate) use self::pdf::pdf_standards_conflict_diagnostic;
use self::pdf::validate_pdf_page_range_element;
pub(crate) use self::realize::realize_paragraphs;
pub(super) use self::set::eval_set_rule;
pub(super) use self::show::eval_show_rule;

/// P636 — mensagem de mismatch de tipo no formato do vanilla
/// (`foundations/cast.rs:325-335`): "expected {expected}, found {actual}".
pub(crate) fn type_mismatch(
    expected: &str,
    found: &Value,
    span: Span,
) -> SourceDiagnostic {
    SourceDiagnostic::error(
        span,
        format!("expected {}, found {}", expected, found.type_name()),
    )
}

/// **P816** (achado #3c de P810) — erro de cast para `Length` no formato
/// do vanilla (`foundations/cast.rs:325-343`): `expected length, found
/// {type}` com hint `a length needs a unit - did you mean {i}pt?` quando o
/// valor recebido é `Int` (hint medido no vanilla, `text(mod.rs)` via
/// `cast.rs:341-343`).
pub(crate) fn expected_length_error(found: &Value, span: Span) -> SourceDiagnostic {
    // Nome do tipo no vocabulário do vanilla (`foundations/cast.rs`,
    // `Value::ty`): o `type_name()` cristalino diverge em `int`/`str`/`bool`
    // (P636); o vanilla diz `integer`/`string`/`boolean` — medido em
    // `#set text(size: 12)` → "expected length, found integer".
    let found_name = match found.type_name() {
        "int" => "integer",
        "str" => "string",
        "bool" => "boolean",
        other => other,
    };
    let mut diag =
        SourceDiagnostic::error(span, format!("expected length, found {found_name}"));
    if let Value::Int(i) = found {
        diag = diag.with_hint(format!("a length needs a unit - did you mean {i}pt?"));
    }
    diag
}

/// **P837** (achado #22 de P831) — erro de cast para `top-edge`/
/// `bottom-edge` no formato verbatim do vanilla 0.15.0 (medido:
/// `error: expected "ascender", "cap-height", "x-height", "baseline",
/// "bounds", or length` para top; `expected "baseline", "descender",
/// "bounds", or length` para bottom; exit 1). String fora do domínio
/// enumerado falha o cast de `TopEdgeMetric`/`BottomEdgeMetric` sem
/// sufixo `found`; outros tipos levam `, found {type}` e, para `Int`,
/// o hint `a length needs a unit - did you mean {i}pt?` (mesmo hint de
/// `expected_length_error`, P816, `foundations/cast.rs:341-343`).
pub(crate) fn edge_cast_error(
    is_top: bool,
    found: &Value,
    span: Span,
) -> SourceDiagnostic {
    let expected = if is_top {
        "\"ascender\", \"cap-height\", \"x-height\", \"baseline\", \"bounds\", or length"
    } else {
        "\"baseline\", \"descender\", \"bounds\", or length"
    };
    if let Value::Str(_) = found {
        return SourceDiagnostic::error(span, format!("expected {expected}"));
    }
    // Nome do tipo no vocabulário do vanilla (mesmo mapeamento de
    // `expected_length_error`, P816).
    let found_name = match found.type_name() {
        "int" => "integer",
        "str" => "string",
        "bool" => "boolean",
        other => other,
    };
    let mut diag =
        SourceDiagnostic::error(span, format!("expected {expected}, found {found_name}"));
    if let Value::Int(i) = found {
        diag = diag.with_hint(format!("a length needs a unit - did you mean {i}pt?"));
    }
    diag
}

/// **P837** — domínios enumerados válidos de `top-edge`/`bottom-edge`
/// (cast de `TopEdgeMetric`/`BottomEdgeMetric`, vanilla
/// `text/mod.rs:1180-1248`). `"bounds"` é válido em ambos (resolvido no
/// vanilla via bbox do glyph; no cristalino cai no fallback defensivo de
/// `edge_offset_pt` — limitação conhecida, ver relatório P837).
const TOP_EDGE_METRICS: &[&str] =
    &["ascender", "cap-height", "x-height", "baseline", "bounds"];
const BOTTOM_EDGE_METRICS: &[&str] = &["baseline", "descender", "bounds"];

/// **P816** (achado #3a de P810) — propriedades nomeadas que o `TextElem`
/// do vanilla aceita em `#set text(...)` (campos settable de
/// `text/mod.rs:182-788`, vanilla 0.15.0, em kebab-case). Nomes fora
/// desta lista são **erro hard** `unexpected argument: {name}` (paridade
/// `foundations/args.rs:262`, exit 1); nomes dentro da lista mas ainda
/// não capturados pelo cristalino mantêm o warning de scope-out do
/// Passo 107 (ADR-0040).
pub(crate) const VANILLA_TEXT_SET_PROPS: &[&str] = &[
    "font",
    "fallback",
    "style",
    "weight",
    "stretch",
    "size",
    "fill",
    "stroke",
    "tracking",
    "spacing",
    "cjk-latin-spacing",
    "baseline",
    "overhang",
    "top-edge",
    "bottom-edge",
    "lang",
    "region",
    "script",
    "dir",
    "hyphenate",
    "costs",
    "kerning",
    "alternates",
    "stylistic-set",
    "ligatures",
    "discretionary-ligatures",
    "historical-ligatures",
    "number-type",
    "number-width",
    "slashed-zero",
    "fractions",
    "features",
    // P836 — `#[fold] #[ghost]` settable no vanilla (`text/mod.rs:846-850`).
    "variations",
];

/// **P1030 fatia 1** — os pares que chegam ao construtor. Medição da Fase A:
/// são exactamente aqueles cujo argumento **explícito** já se aplica no
/// cristalino; os restantes precisam da feature antes do `#set` (grupos B/C/D
/// de `eval.md` §P1030).
const MATH_SET_LIGADOS: &[(&str, &str)] = &[
    ("mat", "delim"),
    ("mat", "align"),
    ("mat", "augment"),
    ("mat", "gap"),
    ("mat", "row-gap"),
    ("mat", "column-gap"),
    ("vec", "delim"),
    ("op", "limits"),
    ("cases", "delim"),
    ("cases", "reverse"),
    ("cases", "gap"),
    ("frac", "style"),
    ("accent", "size"),
    ("accent", "dotless"),
    ("stretch", "size"),
];

/// Diagnostic vocabulary differs from the public short names of types.
fn break_policy_error(expected: &str, found: &Value, span: Span) -> SourceDiagnostic {
    let name = match found.type_name() {
        "int" => "integer",
        "str" => "string",
        "bool" => "boolean",
        other => other,
    };
    SourceDiagnostic::error(span, format!("{expected}, found {name}"))
}

// P1467 (RED — Agente B) — testes do braço `format.pdf` de `eval_set_rule`.
//
// `format.pdf` ainda não tem braço estrutural em `eval_set_rule` neste
// baseline; hoje cai no fallback genérico de target desconhecido
// (`unsupported_target_warn`, medido em `compiler/eval/rules.md` P1467).
// Estes testes fixam o contrato do L0 (§P1467) — falham por asserção em
// runtime (não erro de compilação), porque o campo público
// `format.pdf.tagged`/`format.pdf.pretty` nunca é inserido em
// `Styles.delta().custom` hoje.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::eval::tests::{eval_for_test, eval_for_test_keep_sink};
    use crate::contracts::world::World;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::source::Source;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library,
    };
    use std::num::NonZeroU16;

    struct MockWorld {
        library: Library,
        book: FontBook,
        source: Source,
    }

    impl World for MockWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            self.source.id()
        }
        fn source(&self, _: FileId) -> FileResult<Source> {
            Ok(self.source.clone())
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<Font> {
            None
        }
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<Datetime> {
            None
        }
    }

    fn mock_world(src: &str) -> MockWorld {
        let id = FileId::from_raw(NonZeroU16::new(1).unwrap());
        MockWorld {
            library: Library::new(),
            book: FontBook::new(),
            source: Source::new(id, src.to_string()),
        }
    }

    fn eval_ok_content(src: &str) -> Content {
        let world = mock_world(src);
        let source = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &source).expect("eval deveria ter sucesso");
        module.content().cloned().unwrap_or(Content::Empty)
    }

    fn eval_err(src: &str) -> Vec<SourceDiagnostic> {
        let world = mock_world(src);
        let source = World::source(&world, World::main(&world)).unwrap();
        eval_for_test(&world, &source).expect_err("eval deveria falhar")
    }

    fn eval_warnings(src: &str) -> Vec<SourceDiagnostic> {
        let world = mock_world(src);
        let source = World::source(&world, World::main(&world)).unwrap();
        let (_, sink) = eval_for_test_keep_sink(&world, &source);
        sink.into_diagnostics()
    }

    /// Espelha `styled_custom_bool` de `compiler/eval/tests.rs`, mas
    /// generalizado a qualquer `Value` e a toda a chain (não só um `Styles`
    /// isolado): decisões mais internas (mais próximas do conteúdo real,
    /// logo mais tardias lexicalmente) sobrepõem-se às mais externas.
    fn collect_custom_into(content: &Content, acc: &mut Vec<(EcoString, Value)>) {
        match content {
            Content::Styled(styled) => {
                let inner = &styled.body;
                let styles = &styled.styles;
                for (key, value) in styles.delta().custom.iter() {
                    if let Some(slot) = acc.iter_mut().find(|(k, _)| k == key) {
                        slot.1 = value.clone();
                    } else {
                        acc.push((key.clone(), value.clone()));
                    }
                }
                collect_custom_into(inner, acc);
            }
            Content::Sequence(items) => {
                for item in items.iter() {
                    collect_custom_into(item, acc);
                }
            }
            _ => {}
        }
    }

    fn custom_values(src: &str) -> Vec<(EcoString, Value)> {
        let content = eval_ok_content(src);
        let mut acc = Vec::new();
        collect_custom_into(&content, &mut acc);
        acc
    }

    fn custom_value<'a>(
        values: &'a [(EcoString, Value)],
        key: &str,
    ) -> Option<&'a Value> {
        values.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    #[test]
    fn p1467_tagged_auto_inserts_value_auto() {
        let values = custom_values("#set format.pdf(tagged: auto)\nHello");
        assert_eq!(custom_value(&values, "format.pdf.tagged"), Some(&Value::Auto));
        assert_eq!(custom_value(&values, "format.pdf.pretty"), None);
    }

    #[test]
    fn p1467_tagged_true_inserts_value_bool_true() {
        let values = custom_values("#set format.pdf(tagged: true)\nHello");
        assert_eq!(custom_value(&values, "format.pdf.tagged"), Some(&Value::Bool(true)));
    }

    #[test]
    fn p1467_tagged_false_inserts_value_bool_false() {
        let values = custom_values("#set format.pdf(tagged: false)\nHello");
        assert_eq!(custom_value(&values, "format.pdf.tagged"), Some(&Value::Bool(false)));
    }

    #[test]
    fn p1467_pretty_true_and_false_insert_bool() {
        let true_values = custom_values("#set format.pdf(pretty: true)\nHello");
        assert_eq!(
            custom_value(&true_values, "format.pdf.pretty"),
            Some(&Value::Bool(true))
        );

        let false_values = custom_values("#set format.pdf(pretty: false)\nHello");
        assert_eq!(
            custom_value(&false_values, "format.pdf.pretty"),
            Some(&Value::Bool(false))
        );
    }

    #[test]
    fn p1467_tagged_and_pretty_together_use_independent_keys() {
        let values = custom_values("#set format.pdf(tagged: true, pretty: true)\nHello");
        assert_eq!(custom_value(&values, "format.pdf.tagged"), Some(&Value::Bool(true)));
        assert_eq!(custom_value(&values, "format.pdf.pretty"), Some(&Value::Bool(true)));
    }

    #[test]
    fn p1467_each_field_defined_alone_does_not_touch_the_other() {
        let only_tagged = custom_values("#set format.pdf(tagged: false)\nHello");
        assert_eq!(
            custom_value(&only_tagged, "format.pdf.tagged"),
            Some(&Value::Bool(false))
        );
        assert_eq!(custom_value(&only_tagged, "format.pdf.pretty"), None);

        let only_pretty = custom_values("#set format.pdf(pretty: true)\nHello");
        assert_eq!(custom_value(&only_pretty, "format.pdf.tagged"), None);
        assert_eq!(
            custom_value(&only_pretty, "format.pdf.pretty"),
            Some(&Value::Bool(true))
        );
    }

    #[test]
    fn p1467_empty_rule_inserts_no_keys() {
        let values = custom_values("#set format.pdf()\nHello");
        assert_eq!(custom_value(&values, "format.pdf.tagged"), None);
        assert_eq!(custom_value(&values, "format.pdf.pretty"), None);
    }

    #[test]
    fn p1467_unknown_named_argument_is_rejected() {
        let errors = eval_err("#set format.pdf(nope: 1)\nHello");
        assert!(
            errors.iter().any(|e| e.message.contains("unexpected argument: nope")),
            "{errors:?}"
        );
    }

    #[test]
    fn p1467_positional_argument_is_rejected() {
        let errors = eval_err("#set format.pdf(true)\nHello");
        assert!(
            errors.iter().any(|e| e.message.contains("unexpected argument")),
            "{errors:?}"
        );
    }

    #[test]
    fn p1467_duplicate_tagged_is_rejected() {
        let errors = eval_err("#set format.pdf(tagged: true, tagged: false)\nHello");
        assert!(
            errors
                .iter()
                .any(|e| e.message.contains("duplicate argument: tagged")),
            "{errors:?}"
        );
    }

    #[test]
    fn p1467_duplicate_pretty_is_rejected() {
        let errors = eval_err("#set format.pdf(pretty: true, pretty: false)\nHello");
        assert!(
            errors
                .iter()
                .any(|e| e.message.contains("duplicate argument: pretty")),
            "{errors:?}"
        );
    }

    #[test]
    fn p1467_tagged_wrong_type_has_auto_aware_message() {
        // Diverge de propósito da mensagem dos irmãos só-booleanos
        // (`expected boolean, found <tipo>`): `tagged` aceita `auto` além de
        // `bool`, logo a forma de dois tipos é sempre errada aqui.
        let errors = eval_err("#set format.pdf(tagged: 1)\nHello");
        assert!(
            errors
                .iter()
                .any(|e| e.message.contains("expected boolean or auto, found integer")),
            "{errors:?}"
        );
    }

    #[test]
    fn p1467_pretty_wrong_type_has_boolean_only_message() {
        let errors = eval_err("#set format.pdf(pretty: 1)\nHello");
        assert!(
            errors
                .iter()
                .any(|e| e.message.contains("expected boolean, found integer")),
            "{errors:?}"
        );
    }

    #[test]
    fn p1467_generic_unsupported_target_warning_is_not_emitted_for_pdf() {
        let warnings = eval_warnings("#set format.pdf(tagged: true)\nHello");
        assert!(
            !warnings.iter().any(|w| w.message.contains("ainda não suportado")),
            "o aviso genérico de target não suportado não deve mais alcançar format.pdf: {warnings:?}"
        );
    }

    #[test]
    fn p1467_svg_png_html_set_rules_are_not_regressed() {
        let svg = custom_values("#set format.svg(pretty: true)\nHello");
        assert_eq!(custom_value(&svg, "format.svg.pretty"), Some(&Value::Bool(true)));

        let png = custom_values("#set format.png(ppi: 300)\nHello");
        assert_eq!(custom_value(&png, "format.png.ppi"), Some(&Value::Float(300.0)));

        // `format.html` continua gated pela feature `html` (sem
        // `Features::Html` neste harness); a regressão a provar é que o
        // gate por si só continua intacto, não uma reintrodução do gate.
        let html_errors = eval_err("#set format.html(pretty: true)\nHello");
        assert!(
            html_errors
                .iter()
                .any(|e| e.message.contains("the `html` feature is not enabled")),
            "{html_errors:?}"
        );
    }

    // ── P1468 (RED — Agente B) — named `pages` no braço `format.pdf`.
    // Neste baseline (`0b322a926dd2f86b49aabfeb0d2f287ce06aaace`), o braço
    // só reconhece `tagged`/`pretty` (P1467); `pages` cai no ramo
    // `unexpected argument: pages`. Estes testes falham em runtime
    // (não erro de compilação) até o terceiro `seen_*` ser acrescentado.
    use indexmap::IndexMap;
    use rustc_hash::FxBuildHasher;

    fn dict_value(pairs: &[(&str, i64)]) -> Value {
        let mut map: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
        for (key, value) in pairs {
            map.insert((*key).into(), Value::Int(*value));
        }
        Value::Dict(map)
    }

    #[test]
    fn p1468_pages_integer_inserts_value_int() {
        let values = custom_values("#set format.pdf(pages: 1)\nHello");
        assert_eq!(custom_value(&values, "format.pdf.pages"), Some(&Value::Int(1)));
    }

    #[test]
    fn p1468_pages_dict_closed_inserts_value_dict() {
        let values = custom_values("#set format.pdf(pages: (from: 1, to: 3))\nHello");
        assert_eq!(
            custom_value(&values, "format.pdf.pages"),
            Some(&dict_value(&[("from", 1), ("to", 3)]))
        );
    }

    #[test]
    fn p1468_pages_dict_open_at_end_inserts_value_dict() {
        // Só `from`: aberto no fim.
        let values = custom_values("#set format.pdf(pages: (from: 2))\nHello");
        assert_eq!(
            custom_value(&values, "format.pdf.pages"),
            Some(&dict_value(&[("from", 2)]))
        );
    }

    #[test]
    fn p1468_pages_dict_open_at_start_inserts_value_dict() {
        // Só `to`: aberto no início.
        let values = custom_values("#set format.pdf(pages: (to: 2))\nHello");
        assert_eq!(
            custom_value(&values, "format.pdf.pages"),
            Some(&dict_value(&[("to", 2)]))
        );
    }

    #[test]
    fn p1468_pages_array_mixed_inserts_value_array() {
        let values =
            custom_values("#set format.pdf(pages: (1, (from: 3, to: 5)))\nHello");
        assert_eq!(
            custom_value(&values, "format.pdf.pages"),
            Some(&Value::Array(vec![
                Value::Int(1),
                dict_value(&[("from", 3), ("to", 5)]),
            ]))
        );
    }

    #[test]
    fn p1468_pages_array_empty_inserts_value_array_empty() {
        // Array vazio é uma seleção vazia PRESENTE, distinta de `none`
        // (todas as páginas) — ver `entities/document_format_options.md`.
        let values = custom_values("#set format.pdf(pages: ())\nHello");
        assert_eq!(
            custom_value(&values, "format.pdf.pages"),
            Some(&Value::Array(vec![]))
        );
    }

    #[test]
    fn p1468_pages_none_inserts_value_none_distinct_from_absent() {
        let none_values = custom_values("#set format.pdf(pages: none)\nHello");
        assert_eq!(custom_value(&none_values, "format.pdf.pages"), Some(&Value::None));

        let absent_values = custom_values("#set format.pdf()\nHello");
        assert_eq!(custom_value(&absent_values, "format.pdf.pages"), None);
    }

    #[test]
    fn p1468_pages_empty_rule_inserts_no_pages_key() {
        let values = custom_values("#set format.pdf()\nHello");
        assert_eq!(custom_value(&values, "format.pdf.pages"), None);
        assert_eq!(custom_value(&values, "format.pdf.tagged"), None);
        assert_eq!(custom_value(&values, "format.pdf.pretty"), None);
    }

    #[test]
    fn p1468_pages_is_independent_of_tagged_and_pretty() {
        let values = custom_values(
            "#set format.pdf(pages: 1, tagged: false, pretty: true)\nHello",
        );
        assert_eq!(custom_value(&values, "format.pdf.pages"), Some(&Value::Int(1)));
        assert_eq!(custom_value(&values, "format.pdf.tagged"), Some(&Value::Bool(false)));
        assert_eq!(custom_value(&values, "format.pdf.pretty"), Some(&Value::Bool(true)));

        let only_pages = custom_values("#set format.pdf(pages: 2)\nHello");
        assert_eq!(custom_value(&only_pages, "format.pdf.tagged"), None);
        assert_eq!(custom_value(&only_pages, "format.pdf.pretty"), None);
    }

    #[test]
    fn p1468_pages_duplicate_is_rejected() {
        let errors = eval_err("#set format.pdf(pages: 1, pages: 2)\nHello");
        assert!(
            errors.iter().any(|e| e.message.contains("duplicate argument: pages")),
            "{errors:?}"
        );
    }

    #[test]
    fn p1468_pages_dict_empty_is_rejected() {
        let errors = eval_err("#set format.pdf(pages: (:))\nHello");
        assert!(
            errors
                .iter()
                .any(|e| e.message.contains("page range must not be empty")),
            "{errors:?}"
        );
    }

    #[test]
    fn p1468_pages_dict_extra_key_is_rejected() {
        let errors = eval_err("#set format.pdf(pages: (from: 1, extra: 2))\nHello");
        assert!(
            errors.iter().any(|e| e.message.contains(
                "unexpected key \"extra\", valid keys are \"from\" and \"to\""
            )),
            "{errors:?}"
        );
    }

    #[test]
    fn p1468_pages_dict_from_greater_than_to_is_rejected() {
        let errors = eval_err("#set format.pdf(pages: (from: 5, to: 2))\nHello");
        assert!(
            errors.iter().any(|e| e
                .message
                .contains("start of page range cannot be larger than end")),
            "{errors:?}"
        );
    }

    #[test]
    fn p1468_pages_zero_is_rejected() {
        let errors = eval_err("#set format.pdf(pages: 0)\nHello");
        assert!(
            errors.iter().any(|e| e.message.contains("page numbers start at one")),
            "{errors:?}"
        );
    }

    #[test]
    fn p1468_pages_negative_is_rejected() {
        let errors = eval_err("#set format.pdf(pages: -1)\nHello");
        assert!(
            errors.iter().any(|e| e.message.contains("page numbers start at one")),
            "{errors:?}"
        );
    }

    #[test]
    fn p1468_pages_float_top_level_type_is_rejected() {
        let errors = eval_err("#set format.pdf(pages: 1.5)\nHello");
        assert!(
            errors.iter().any(|e| e
                .message
                .contains("expected integer, dictionary, array, or none, found float")),
            "{errors:?}"
        );
    }

    #[test]
    fn p1468_pages_string_top_level_type_is_rejected() {
        let errors = eval_err("#set format.pdf(pages: \"1\")\nHello");
        assert!(
            errors.iter().any(|e| e
                .message
                .contains("expected integer, dictionary, array, or none, found string")),
            "{errors:?}"
        );
    }

    #[test]
    fn p1468_pages_auto_top_level_type_is_rejected() {
        let errors = eval_err("#set format.pdf(pages: auto)\nHello");
        assert!(
            errors.iter().any(|e| e
                .message
                .contains("expected integer, dictionary, array, or none, found auto")),
            "{errors:?}"
        );
    }

    #[test]
    fn p1468_pages_array_element_type_message_differs_from_top_level() {
        // A mensagem de elemento de array NÃO menciona `array`/`none`
        // (um elemento não pode ser ele próprio array/none) — distinta da
        // mensagem de topo testada acima.
        let errors = eval_err("#set format.pdf(pages: (1, \"x\"))\nHello");
        assert!(
            errors.iter().any(|e| e
                .message
                .contains("expected integer or dictionary, found string")),
            "{errors:?}"
        );
        assert!(
            !errors.iter().any(|e| e.message.contains("array, or none")),
            "mensagem de elemento não deve reutilizar a forma de topo: {errors:?}"
        );
    }

    #[test]
    fn p1468_svg_png_html_set_rules_are_not_regressed_by_pages() {
        let svg = custom_values("#set format.svg(pretty: true)\nHello");
        assert_eq!(custom_value(&svg, "format.svg.pretty"), Some(&Value::Bool(true)));

        let png = custom_values("#set format.png(ppi: 300)\nHello");
        assert_eq!(custom_value(&png, "format.png.ppi"), Some(&Value::Float(300.0)));

        let html_errors = eval_err("#set format.html(pretty: true)\nHello");
        assert!(
            html_errors
                .iter()
                .any(|e| e.message.contains("the `html` feature is not enabled")),
            "{html_errors:?}"
        );
    }

    // ── P1469 (RED — Agente B) — named `standard` no braço `format.pdf`
    // (contrato de linguagem, escopo reduzido — ver
    // `entities/document_format_options.md` P1469 e
    // `compiler/eval/rules.md` P1469). `PdfStandard`/`PdfStandards`/
    // `PdfStandardsConflict` (entities) ainda não existem neste baseline
    // (`65e68ad45dcbfd8db00aeda78d0d2fbaaadefb27`), nem o named `standard`
    // em `eval_set_rule`; este módulo falha por erro de compilação até o
    // carrier P1469 (mesmo gate ADR-0127) ser materializado por outro
    // agente em paralelo.

    const STANDARD_SEVENTEEN_NAMES: [&str; 17] = [
        "1.4", "1.5", "1.6", "1.7", "2.0", "a-1b", "a-1a", "a-2b", "a-2u", "a-2a",
        "a-3b", "a-3u", "a-3a", "a-4", "a-4f", "a-4e", "ua-1",
    ];

    // Lista das 17 opções tal como aparece no erro de schema de topo
    // (com `array` como opção adicional): medida via `typst eval` em
    // 2026-09-21, mesmo padrão Oxford-comma já usado em
    // `stdlib/html.rs`/`stdlib/pdf.rs` para `cast!` de enums de string.
    const STANDARD_SEVENTEEN_NAMES_LIST: &str = "\"1.4\", \"1.5\", \"1.6\", \"1.7\", \"2.0\", \"a-1b\", \"a-1a\", \"a-2b\", \"a-2u\", \"a-2a\", \"a-3b\", \"a-3u\", \"a-3a\", \"a-4\", \"a-4f\", \"a-4e\", \"ua-1\"";

    // Mesma lista, mas como forma de ELEMENTO de array: sem a opção
    // `array` (um elemento não pode ser array aninhado), logo o último
    // nome (`ua-1`) é que recebe o prefixo "or".
    const STANDARD_SEVENTEEN_NAMES_OR_LIST: &str = "\"1.4\", \"1.5\", \"1.6\", \"1.7\", \"2.0\", \"a-1b\", \"a-1a\", \"a-2b\", \"a-2u\", \"a-2a\", \"a-3b\", \"a-3u\", \"a-3a\", \"a-4\", \"a-4f\", \"a-4e\", or \"ua-1\"";

    #[test]
    fn p1469_standard_each_of_seventeen_names_inserts_value_array_of_one_string() {
        for name in STANDARD_SEVENTEEN_NAMES {
            let src = format!("#set format.pdf(standard: \"{name}\")\nHello");
            let values = custom_values(&src);
            assert_eq!(
                custom_value(&values, "format.pdf.standard"),
                Some(&Value::Array(vec![Value::Str(name.into())])),
                "nome {name:?}: named string único deve ser envolvido num array de \
                 um elemento (mesma forma de `repr(format.pdf.standard) == (\"1.7\",)`)"
            );
        }
    }

    #[test]
    fn p1469_standard_array_with_multiple_compatible_names_inserts_value_array() {
        // `a-1b` (range 1.4..=1.4) + versão explícita "1.4": compatível.
        let values =
            custom_values("#set format.pdf(standard: (\"a-1b\", \"1.4\"))\nHello");
        assert_eq!(
            custom_value(&values, "format.pdf.standard"),
            Some(&Value::Array(vec![
                Value::Str("a-1b".into()),
                Value::Str("1.4".into()),
            ]))
        );
    }

    #[test]
    fn p1469_standard_empty_array_inserts_value_array_empty() {
        // Array vazio é uma seleção válida (sem validators; efetivo 1.7).
        let values = custom_values("#set format.pdf(standard: ())\nHello");
        assert_eq!(
            custom_value(&values, "format.pdf.standard"),
            Some(&Value::Array(vec![]))
        );
    }

    #[test]
    fn p1469_standard_absent_inserts_no_standard_key() {
        // `standard` NÃO aceita `none` como forma explícita de "ausente"
        // (diferente de `pages`) — a única forma é a ausência do named.
        let values = custom_values("#set format.pdf()\nHello");
        assert_eq!(custom_value(&values, "format.pdf.standard"), None);
        assert_eq!(custom_value(&values, "format.pdf.tagged"), None);
        assert_eq!(custom_value(&values, "format.pdf.pretty"), None);
        assert_eq!(custom_value(&values, "format.pdf.pages"), None);
    }

    #[test]
    fn p1469_standard_conflict_two_versions_uses_exact_message() {
        let errors = eval_err("#set format.pdf(standard: (\"1.4\", \"1.7\"))\nHello");
        assert!(
            errors.iter().any(|e| e.message
                == "PDF cannot conform to PDF 1.4 and PDF 1.7 at the same time"),
            "{errors:?}"
        );
    }

    #[test]
    fn p1469_standard_conflict_two_archival_uses_exact_message() {
        let errors = eval_err("#set format.pdf(standard: (\"a-1b\", \"a-2b\"))\nHello");
        assert!(
            errors
                .iter()
                .any(|e| e.message == "choose at most one PDF/A standard"),
            "{errors:?}"
        );
    }

    #[test]
    fn p1469_standard_conflict_two_accessibility_uses_exact_message() {
        // `ua-1` é o único nome de acessibilidade; repeti-lo no array é a
        // única forma de produzir este conflito.
        let errors = eval_err("#set format.pdf(standard: (\"ua-1\", \"ua-1\"))\nHello");
        assert!(
            errors
                .iter()
                .any(|e| e.message == "choose at most one PDF/UA standard"),
            "{errors:?}"
        );
    }

    #[test]
    fn p1469_standard_conflict_no_overlapping_range_between_validators_fails() {
        // `ua-1` (range 1.4..=1.7) e `a-4` (range 2.0..=2.0) não têm
        // interseção. O texto exato desta mensagem NÃO foi medido em
        // nenhum L0 de P1469 (só a estrutura `NoOverlappingRange` foi
        // medida em `entities/document_format_options.md`); por
        // disciplina ADR-0108 (não inventar texto não medido), este teste
        // só prova que a combinação falha, sem fixar a mensagem exata.
        let errors = eval_err("#set format.pdf(standard: (\"ua-1\", \"a-4\"))\nHello");
        assert!(!errors.is_empty(), "{errors:?}");
    }

    #[test]
    fn p1469_standard_conflict_version_out_of_range_uses_exact_message_and_hint() {
        // `ua-1` (range 1.4..=1.7, min != max) -> hint usa "between".
        let errors = eval_err("#set format.pdf(standard: (\"ua-1\", \"2.0\"))\nHello");
        assert!(
            errors
                .iter()
                .any(|e| e.message == "PDF 2.0 is not compatible with PDF/UA-1"),
            "{errors:?}"
        );
        let hints: Vec<_> = errors.iter().flat_map(|e| e.hints.iter()).collect();
        assert!(
            hints.iter().any(|h| h.as_str()
                == "PDF/UA-1 requires a version between PDF 1.4 and PDF 1.7"),
            "{hints:?}"
        );

        // `a-1b` (range 1.4..=1.4, min == max) -> hint SEM "between".
        let errors = eval_err("#set format.pdf(standard: (\"a-1b\", \"1.7\"))\nHello");
        assert!(
            errors
                .iter()
                .any(|e| e.message == "PDF 1.7 is not compatible with PDF/A-1b"),
            "{errors:?}"
        );
        let hints: Vec<_> = errors.iter().flat_map(|e| e.hints.iter()).collect();
        assert!(
            hints
                .iter()
                .any(|h| h.as_str() == "PDF/A-1b requires version PDF 1.4"),
            "{hints:?}"
        );
        assert!(
            !hints.iter().any(|h| h.contains("between")),
            "min==max não deve usar \"between\": {hints:?}"
        );

        // `a-4` (range 2.0..=2.0, min == max) -> mesma forma sem "between".
        let errors = eval_err("#set format.pdf(standard: (\"a-4\", \"1.7\"))\nHello");
        assert!(
            errors
                .iter()
                .any(|e| e.message == "PDF 1.7 is not compatible with PDF/A-4"),
            "{errors:?}"
        );
        let hints: Vec<_> = errors.iter().flat_map(|e| e.hints.iter()).collect();
        assert!(
            hints.iter().any(|h| h.as_str() == "PDF/A-4 requires version PDF 2.0"),
            "{hints:?}"
        );
    }

    #[test]
    fn p1469_standard_none_is_rejected_with_found_none_suffix() {
        let errors = eval_err("#set format.pdf(standard: none)\nHello");
        assert!(
            errors.iter().any(|e| e.message
                == format!(
                    "expected {STANDARD_SEVENTEEN_NAMES_LIST}, or array, found none"
                )),
            "{errors:?}"
        );
    }

    #[test]
    fn p1469_standard_integer_is_rejected_with_found_integer_suffix() {
        let errors = eval_err("#set format.pdf(standard: 17)\nHello");
        assert!(
            errors.iter().any(|e| e.message
                == format!(
                    "expected {STANDARD_SEVENTEEN_NAMES_LIST}, or array, found integer"
                )),
            "{errors:?}"
        );
    }

    #[test]
    fn p1469_standard_boolean_is_rejected_with_found_boolean_suffix() {
        let errors = eval_err("#set format.pdf(standard: true)\nHello");
        assert!(
            errors.iter().any(|e| e.message
                == format!(
                    "expected {STANDARD_SEVENTEEN_NAMES_LIST}, or array, found boolean"
                )),
            "{errors:?}"
        );
    }

    #[test]
    fn p1469_standard_unrecognized_string_is_rejected_without_found_suffix() {
        // Nuance medida: um `Value::Str` reconhecido como string mas que
        // não corresponde a nenhum dos 17 nomes NÃO ganha ", found
        // string" — mesmo padrão de qualquer `cast!` Typst para enum de
        // string literais (contraste com o caso `none`/inteiro/boolean
        // acima, que NÃO são string).
        let errors = eval_err("#set format.pdf(standard: \"3.0\")\nHello");
        assert!(
            errors.iter().any(|e| e.message
                == format!("expected {STANDARD_SEVENTEEN_NAMES_LIST}, or array")),
            "{errors:?}"
        );
        assert!(!errors.iter().any(|e| e.message.contains("found string")), "{errors:?}");
    }

    #[test]
    fn p1469_standard_array_element_invalid_uses_list_without_array_or_found() {
        let errors = eval_err("#set format.pdf(standard: (\"1.7\", \"xyz\"))\nHello");
        assert!(
            errors
                .iter()
                .any(|e| e.message
                    == format!("expected {STANDARD_SEVENTEEN_NAMES_OR_LIST}")),
            "{errors:?}"
        );
        assert!(
            !errors.iter().any(|e| e.message.contains("array")),
            "mensagem de elemento não deve mencionar a opção `array`: {errors:?}"
        );
        assert!(
            !errors.iter().any(|e| e.message.contains("found")),
            "mensagem de elemento não deve usar sufixo `found`: {errors:?}"
        );
    }

    #[test]
    fn p1469_standard_is_independent_of_tagged_pretty_and_pages() {
        let values = custom_values(
            "#set format.pdf(standard: \"a-2b\", tagged: false, pretty: true, pages: 1)\nHello",
        );
        assert_eq!(
            custom_value(&values, "format.pdf.standard"),
            Some(&Value::Array(vec![Value::Str("a-2b".into())]))
        );
        assert_eq!(custom_value(&values, "format.pdf.tagged"), Some(&Value::Bool(false)));
        assert_eq!(custom_value(&values, "format.pdf.pretty"), Some(&Value::Bool(true)));
        assert_eq!(custom_value(&values, "format.pdf.pages"), Some(&Value::Int(1)));

        let only_standard = custom_values("#set format.pdf(standard: \"1.7\")\nHello");
        assert_eq!(custom_value(&only_standard, "format.pdf.tagged"), None);
        assert_eq!(custom_value(&only_standard, "format.pdf.pretty"), None);
        assert_eq!(custom_value(&only_standard, "format.pdf.pages"), None);
    }

    #[test]
    fn p1469_standard_duplicate_named_is_rejected() {
        // Mesma política de named duplicado que `tagged`/`pretty`/`pages`.
        let errors =
            eval_err("#set format.pdf(standard: \"1.4\", standard: \"1.7\")\nHello");
        assert!(
            errors
                .iter()
                .any(|e| e.message.contains("duplicate argument: standard")),
            "{errors:?}"
        );
    }

    #[test]
    fn p1469_svg_png_html_set_rules_are_not_regressed_by_standard() {
        let svg = custom_values("#set format.svg(pretty: true)\nHello");
        assert_eq!(custom_value(&svg, "format.svg.pretty"), Some(&Value::Bool(true)));

        let png = custom_values("#set format.png(ppi: 300)\nHello");
        assert_eq!(custom_value(&png, "format.png.ppi"), Some(&Value::Float(300.0)));

        let html_errors = eval_err("#set format.html(pretty: true)\nHello");
        assert!(
            html_errors
                .iter()
                .any(|e| e.message.contains("the `html` feature is not enabled")),
            "{html_errors:?}"
        );
    }
}
