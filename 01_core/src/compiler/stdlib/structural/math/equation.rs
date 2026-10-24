//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/structural/math/equation.md
//! @prompt-hash f7b96e98
//! @layer L1
//! @updated 2026-09-30
//!
//! Casts e construção semântica de `math.equation`.

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::file_id::FileId;
use crate::entities::layout_types::{Align2D, HAlign};
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::style::Styles;
use crate::entities::value::Value;

/// Construtor puro compartilhado pela função pública e pela sintaxe `$...$`.
pub(crate) fn equation_content(body: Content, block: bool) -> Content {
    Content::equation(body, block)
}

/// Cast público de `math.equation.number-align`.
pub(crate) fn equation_number_align(value: &Value, span: Span) -> SourceResult<Align2D> {
    let Value::Align(align) = value else {
        let found = match value.type_name() {
            "int" => "integer",
            "str" => "string",
            "bool" => "boolean",
            other => other,
        };
        return Err(vec![SourceDiagnostic::error(
            span,
            format!("expected alignment, found {found}"),
        )]);
    };
    if matches!(align.h, Some(HAlign::Center)) {
        return Err(vec![SourceDiagnostic::error(
            span,
            "expected `start`, `left`, `right`, or `end`, found center".to_string(),
        )]);
    }
    Ok(*align)
}

pub(crate) fn equation_supplement(value: &Value, span: Span) -> SourceResult<Value> {
    match value {
        Value::Str(text) => Ok(Value::Content(Content::text(text.as_str()))),
        Value::Content(_) | Value::Func(_) | Value::None | Value::Auto => {
            Ok(value.clone())
        }
        other => {
            let found = match other.type_name() {
                "int" => "integer",
                "bool" => "boolean",
                "str" => "string",
                name => name,
            };
            Err(vec![SourceDiagnostic::error(
                span,
                format!("expected content, function, none, or auto, found {found}"),
            )])
        }
    }
}

pub(crate) fn equation_alt(value: &Value, span: Span) -> SourceResult<Value> {
    match value {
        Value::Str(_) | Value::None => Ok(value.clone()),
        other => {
            let found = match other.type_name() {
                "int" => "integer",
                "bool" => "boolean",
                "str" => "string",
                name => name,
            };
            Err(vec![SourceDiagnostic::error(
                span,
                format!("expected string or none, found {found}"),
            )])
        }
    }
}

/// `math.equation(body, block: false)` — primeira fase do elemento público.
pub fn native_math_equation(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let body = match args.items.as_slice() {
        [body] => super::casts::math_lr_content_value(body, args.span)?,
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: body".to_string(),
            )])
        }
        _ => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "unexpected argument".to_string(),
            )])
        }
    };

    let mut block = false;
    let mut numbering: Option<Value> = None;
    let mut number_align: Option<Value> = None;
    let mut supplement: Option<Value> = None;
    let mut alt: Option<Value> = None;
    for (name, value) in &args.named {
        match name.as_str() {
            "block" => match value {
                Value::Bool(value) => block = *value,
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        format!("expected bool, found {}", other.type_name()),
                    )])
                }
            },
            "numbering" => match value {
                Value::Str(_) | Value::Func(_) | Value::None => {
                    numbering = Some(value.clone());
                }
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        format!(
                            "expected string, function, or none, found {}",
                            other.type_name()
                        ),
                    )])
                }
            },
            "number-align" => {
                equation_number_align(value, args.span)?;
                number_align = Some(value.clone());
            }
            "supplement" => {
                supplement = Some(equation_supplement(value, args.span)?);
            }
            "alt" => {
                alt = Some(equation_alt(value, args.span)?);
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("unexpected argument: {other}"),
                )])
            }
        }
    }

    let equation = equation_content(body, block);
    let mut styles = Styles::new();
    if let Some(value) = numbering {
        styles = styles.push_custom("equation.numbering", value);
    }
    if let Some(value) = number_align {
        styles = styles.push_custom("equation.number-align", value);
    }
    if let Some(value) = supplement {
        styles = styles.push_custom("equation.supplement", value);
    }
    if let Some(value) = alt {
        styles = styles.push_custom("equation.alt", value);
    }
    Ok(Value::Content(if styles.delta().custom.is_empty() {
        equation
    } else {
        Content::styled(
            Box::new(equation),
            styles.push_custom("equation.constructor", Value::Bool(true)),
        )
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::content::Content;

    #[test]
    fn supplement_string_cast_differs_from_alt_content_rejection() {
        let supplement =
            equation_supplement(&Value::Str("Eq".into()), Span::detached()).unwrap();
        assert!(matches!(supplement, Value::Content(Content::Text(_))));
        let error = equation_alt(&Value::Content(Content::text("Eq")), Span::detached())
            .unwrap_err();
        assert_eq!(error[0].message, "expected string or none, found content");
    }
}

#[cfg(test)]
mod v16_e_equation_producer_tests {
    use super::*;
    use crate::contracts::world::World;
    use crate::entities::{
        font_book::FontBook,
        func::Func,
        source::Source,
        world_types::{Bytes, Datetime, FileError, FileResult, Font, Library},
    };

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
            FileId::from_raw(std::num::NonZeroU16::new(1).unwrap())
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

    fn invoke(args: &Args) -> Content {
        let world = TestWorld::default();
        let alias = native_math_equation;
        let Value::Content(content) =
            alias(&mut EvalContext::new(), args, &world, world.main()).unwrap()
        else {
            panic!("equation constructor must return content");
        };
        content
    }

    fn callback_must_not_run(
        _: &mut EvalContext,
        _: &Args,
        _: &dyn World,
        _: FileId,
    ) -> SourceResult<Value> {
        panic!("constructor/identity must not execute an option callback");
    }

    #[test]
    fn v16_e_equation_actual_constructor_options_keep_identity() {
        for (name, value) in [
            ("numbering", Value::Str("(1)".into())),
            ("numbering", Value::None),
            ("supplement", Value::Auto),
            ("alt", Value::Str("description".into())),
            (
                "supplement",
                Value::Func(Func::native("test-callback", callback_must_not_run)),
            ),
        ] {
            let mut args = Args::positional(vec![Value::Content(Content::text("x"))]);
            args.named.insert(name.into(), value);
            let content = invoke(&args);
            assert_eq!(content.elem_name(), "equation", "option {name}");
            let Content::Styled(styled) = content else {
                panic!("existing option transport expected")
            };
            assert!(styled
                .styles
                .delta()
                .custom
                .iter()
                .any(|(key, value)| key == "equation.constructor"
                    && matches!(value, Value::Bool(true))));
        }
    }

    #[test]
    fn v16_e_equation_bare_and_block_only_do_not_gain_transport() {
        for block in [None, Some(true), Some(false)] {
            let mut args = Args::positional(vec![Value::Content(Content::text("x"))]);
            if let Some(block) = block {
                args.named.insert("block".into(), Value::Bool(block));
            }
            let content = invoke(&args);
            assert_eq!(content.elem_name(), "equation");
            assert!(matches!(content, Content::Equation(_)));
        }
    }
}
