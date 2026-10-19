//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/structural/par.md
//! @prompt-hash fbc908ed
//! @layer L1
//! @updated 2026-09-30
//!
//! `par` e `parbreak` — elemento e fronteira de parágrafo no fluxo.
//!
//! Separado de `structural/flow.rs` em 2026-08-13: a fronteira anterior
//! estava sustentada por um cluster de co-mudança que não existia (artefacto
//! de atribuição da ferramenta). Fronteira nova medida no L0 deste nó.

use crate::compiler::eval::rules::expected_length_error;
use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::file_id::FileId;
use crate::entities::layout_types::Length;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

/// `par.line` is a settable element-like value, but cannot be constructed.
///
/// Line numbers are materialized by the physical line closer. Calling this
/// function directly must therefore fail before producing any `Content`.
pub fn native_par_line(
    _ctx: &mut EvalContext,
    _args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    Err(vec![SourceDiagnostic::error(Span::detached(), "cannot be constructed manually")])
}

/// `parbreak()` → marker de quebra de parágrafo já usado pela sintaxe markup.
///
/// P1140.17 restaura somente a superfície chamável ratificada. A realização e
/// o colapso de quebras consecutivas permanecem nos consumers existentes.
pub fn native_parbreak(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if !args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "unexpected argument".to_string(),
        )]);
    }

    if let Some(name) = args.named.keys().next() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }

    Ok(Value::Content(Content::Parbreak))
}

/// Build a paragraph element with validated lexical paragraph options.
/// Global optimized breaking remains incomplete.
pub fn native_par(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if let Some(value) = args.named.get("hanging-indent") {
        if !matches!(value, Value::Length(_)) {
            let span = args
                .occurrences
                .as_ref()
                .and_then(|items| {
                    items
                        .iter()
                        .rev()
                        .find(|item| item.name.as_deref() == Some("hanging-indent"))
                })
                .map_or(args.span, |item| item.value_span);
            return Err(vec![expected_length_error(value, span)]);
        }
    }
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("expected content, found {}", other.type_name()),
            )])
        }
        None => {
            // Literal vanilla (`typst-library/src/model/par.rs` — medido:
            // `#par()` → "missing argument: body").
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "missing argument: body".to_string(),
            )]);
        }
    };

    let mut leading: Option<Length> = None;
    let mut styles = crate::entities::style::Styles::new();
    for (key, value) in args.named.iter() {
        match key.as_str() {
            "leading" => match value {
                Value::Length(l) => leading = Some(*l),
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        format!(
                            "par(leading:) espera length, recebeu {}",
                            other.type_name()
                        ),
                    )])
                }
            },
            "justify" => {
                let span = args
                    .occurrences
                    .as_ref()
                    .and_then(|items| {
                        items
                            .iter()
                            .rev()
                            .find(|item| item.name.as_deref() == Some("justify"))
                    })
                    .map_or(args.span, |item| item.value_span);
                match value {
                    Value::Bool(_) => {
                        styles = styles.push_custom("par.justify", value.clone())
                    }
                    other => {
                        return Err(vec![break_policy_error(
                            "expected boolean",
                            other,
                            span,
                        )])
                    }
                }
            }
            "linebreaks" => {
                let span = args
                    .occurrences
                    .as_ref()
                    .and_then(|items| {
                        items
                            .iter()
                            .rev()
                            .find(|item| item.name.as_deref() == Some("linebreaks"))
                    })
                    .map_or(args.span, |item| item.value_span);
                let expected = "expected \"simple\", \"optimized\", or auto";
                match value {
                    Value::Auto => {}
                    Value::Str(name)
                        if matches!(name.as_str(), "simple" | "optimized") => {}
                    Value::Str(_) => {
                        return Err(vec![SourceDiagnostic::error(span, expected)])
                    }
                    other => return Err(vec![break_policy_error(expected, other, span)]),
                }
                styles = styles.push_custom("par.linebreaks", value.clone());
            }
            "spacing" => {
                let span = args
                    .occurrences
                    .as_ref()
                    .and_then(|items| {
                        items
                            .iter()
                            .rev()
                            .find(|item| item.name.as_deref() == Some("spacing"))
                    })
                    .map_or(args.span, |item| item.value_span);
                match value {
                    Value::Length(length) => {
                        styles =
                            styles.push_custom("par.spacing", Value::Length(*length));
                    }
                    other => return Err(vec![expected_length_error(other, span)]),
                }
            }
            // Known properties awaiting their own consumers.
            "hanging-indent" => {
                if let Value::Length(length) = value {
                    styles =
                        styles.push_custom("par.hanging-indent", Value::Length(*length));
                }
            }
            "first-line-indent" | "justification-limits" => {}
            other => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("argumento nomeado inesperado em par(): '{}'", other),
                )])
            }
        }
    }

    if let Some(l) = leading {
        styles = styles.push_custom("par.leading", Value::Length(l));
    }
    let body = Content::par(body);
    Ok(Value::Content(if styles.is_empty() {
        body
    } else {
        Content::styled(Box::new(body), styles)
    }))
}

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

#[cfg(test)]
mod v16_paragraph_spacing_tests {
    use super::*;
    use crate::contracts::world::World;
    use crate::entities::{
        args::ArgOccurrence,
        font_book::FontBook,
        source::Source,
        world_types::{Bytes, Datetime, FileError, FileResult, Font, Library},
    };
    use std::num::NonZeroU16;

    #[derive(Default)]
    struct TestWorld {
        library: Library,
        book: FontBook,
    }
    impl World for TestWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            FileId::from_raw(NonZeroU16::new(1).unwrap())
        }
        fn source(&self, _: FileId) -> FileResult<Source> {
            Err(FileError::NotFound)
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
    fn invoke(args: &Args) -> SourceResult<Value> {
        let world = TestWorld::default();
        native_par(&mut EvalContext::new(), args, &world, world.main())
    }

    #[test]
    fn v16_paragraph_spacing_rejects_invalid_types() {
        for value in [
            Value::None,
            Value::Auto,
            Value::Bool(true),
            Value::Int(1),
            Value::Float(1.5),
            Value::Str("x".into()),
            Value::Ratio(crate::entities::layout_types::Ratio(0.5)),
        ] {
            let mut args = Args::positional(vec![Value::Content(Content::Empty)]);
            args.named.insert("spacing".into(), value);
            assert!(invoke(&args).unwrap_err()[0]
                .message
                .starts_with("expected length, found"));
        }
    }

    #[test]
    fn v16_paragraph_spacing_integer_hint_and_last_origin() {
        let file = TestWorld::default().main();
        let whole = Span::from_range(file, 0..30);
        let first = Span::from_range(file, 5..9);
        let last = Span::from_range(file, 25..26);
        let args = Args::from_occurrences(
            whole,
            vec![
                ArgOccurrence {
                    name: None,
                    value: Value::Content(Content::Empty),
                    span: whole,
                    value_span: whole,
                },
                ArgOccurrence {
                    name: Some("spacing".into()),
                    value: Value::Length(Length::pt(30.0)),
                    span: first,
                    value_span: first,
                },
                ArgOccurrence {
                    name: Some("spacing".into()),
                    value: Value::Int(1),
                    span: last,
                    value_span: last,
                },
            ],
        );
        let diagnostics = invoke(&args).unwrap_err();
        assert_eq!(diagnostics[0].message, "expected length, found integer");
        assert_eq!(diagnostics[0].span, last);
        assert_eq!(diagnostics[0].hints, ["a length needs a unit - did you mean 1pt?"]);
    }

    #[test]
    fn v16_paragraph_spacing_synthetic_arguments_use_aggregate_span() {
        let mut args = Args::positional(vec![Value::Content(Content::Empty)]);
        args.span = Span::from_range(TestWorld::default().main(), 10..20);
        args.named.insert("spacing".into(), Value::Auto);
        assert_eq!(invoke(&args).unwrap_err()[0].span, args.span);
    }

    #[test]
    fn v16_paragraph_spacing_body_error_precedes_option_error() {
        let mut args = Args::positional(vec![Value::Bool(true)]);
        args.named.insert("spacing".into(), Value::None);
        assert!(invoke(&args).unwrap_err()[0].message.starts_with("expected content"));
    }
}
