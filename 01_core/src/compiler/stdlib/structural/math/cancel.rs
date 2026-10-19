//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/structural/math/cancel.md
//! @prompt-hash 0f62c037
//! @layer L1
//! @updated 2026-09-30
//!
//! Cast e construção semântica de `math.cancel`.

use super::vanilla_type_name_class;
use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::elements::math_cancel::{MathCancelAngle, MathCancelExplicit};
use crate::entities::file_id::FileId;
use crate::entities::layout_types::Length;
use crate::entities::rel::Rel;
use crate::entities::source_result::SourceDiagnostic;
use crate::entities::source_result::SourceResult;
use crate::entities::value::Value;

/// `cancel(body)` — emite `Content::MathCancel { body }`.
/// Body posicional obrigatório (content ou string).
pub fn native_cancel(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let body = match args.items.as_slice() {
        [Value::Content(c)] => c.clone(),
        [Value::Str(s)] => Content::text(s.as_str()),
        [other] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected content, found {}", vanilla_type_name_class(other)),
            )])
        }
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: body".to_string(),
            )])
        }
        _ => return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]),
    };

    let mut length = Rel::from_percent(100.0) + Length::em(0.3);
    let mut inverted = false;
    let mut cross = false;
    let mut angle = MathCancelAngle::Auto;
    let mut stroke = None;
    let mut background = false;
    let mut explicit = MathCancelExplicit::default();
    for (name, value) in &args.named {
        match name.as_str() {
            "length" => {
                explicit.length = true;
                match value {
                    Value::Relative(value) => length = *value,
                    Value::Length(value) => length = Rel { rel: 0.0, abs: *value },
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            args.span,
                            format!(
                                "expected relative length, found {}",
                                vanilla_type_name_class(other)
                            ),
                        )])
                    }
                }
            }
            "inverted" => {
                explicit.inverted = true;
                match value {
                    Value::Bool(value) => inverted = *value,
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            args.span,
                            format!(
                                "expected boolean, found {}",
                                vanilla_type_name_class(other)
                            ),
                        )])
                    }
                }
            }
            "cross" => {
                explicit.cross = true;
                match value {
                    Value::Bool(value) => cross = *value,
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            args.span,
                            format!(
                                "expected boolean, found {}",
                                vanilla_type_name_class(other)
                            ),
                        )])
                    }
                }
            }
            "angle" => {
                explicit.angle = true;
                match value {
                    Value::Auto => angle = MathCancelAngle::Auto,
                    Value::Angle(value) => angle = MathCancelAngle::Angle(*value),
                    Value::Func(value) => angle = MathCancelAngle::Func(value.clone()),
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            args.span,
                            format!(
                                "expected angle, function, or auto, found {}",
                                vanilla_type_name_class(other)
                            ),
                        )])
                    }
                }
            }
            "stroke" => {
                explicit.stroke = true;
                match value {
                    Value::None => stroke = None,
                    other => {
                        stroke = Some(crate::compiler::stdlib::layout::extract_stroke(
                            other, "cancel", "stroke",
                        )?);
                    }
                }
            }
            "background" => {
                explicit.background = true;
                match value {
                    Value::Bool(value) => background = *value,
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            args.span,
                            format!(
                                "expected boolean, found {}",
                                vanilla_type_name_class(other)
                            ),
                        )])
                    }
                }
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("unexpected argument: {other}"),
                )])
            }
        }
    }

    Ok(Value::Content(Content::math_cancel_full(
        body, length, inverted, cross, angle, stroke, background, args.span, explicit,
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::world::World;
    use crate::entities::content::Content;
    use crate::entities::font_book::FontBook;
    use crate::entities::source::Source;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library,
    };
    use std::num::NonZeroU16;

    #[derive(Default)]
    struct NullWorld {
        library: Library,
        book: FontBook,
    }
    impl World for NullWorld {
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

    #[test]
    fn explicit_false_preserves_named_presence() {
        let mut args = Args::positional(vec![Value::Content(Content::text("x"))]);
        args.named.insert("cross".into(), Value::Bool(false));
        let file = FileId::from_raw(NonZeroU16::new(1).unwrap());
        let Value::Content(Content::MathCancel(elem)) =
            native_cancel(&mut EvalContext::new(), &args, &NullWorld::default(), file)
                .unwrap()
        else {
            panic!("math.cancel must produce its semantic carrier")
        };
        assert!(!elem.cross);
        assert!(elem.explicit.cross);
    }
}
