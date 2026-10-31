//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/structural/math/frac.md
//! @prompt-hash d0d7febe
//! @layer L1
//! @updated 2026-09-30
//!
//! Cast e construção semântica de `math.frac`.

use super::vanilla_type_name_class;
use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

fn math_frac_style(
    value: &Value,
    span: Span,
) -> SourceResult<crate::entities::elements::math_frac::MathFracStyle> {
    use crate::entities::elements::math_frac::MathFracStyle;
    match value {
        Value::Str(value) if value.as_str() == "vertical" => Ok(MathFracStyle::Vertical),
        Value::Str(value) if value.as_str() == "skewed" => Ok(MathFracStyle::Skewed),
        Value::Str(value) if value.as_str() == "horizontal" => {
            Ok(MathFracStyle::Horizontal)
        }
        Value::Str(_) => Err(vec![SourceDiagnostic::error(
            span,
            "expected \"vertical\", \"skewed\", or \"horizontal\"".to_string(),
        )]),
        other => Err(vec![SourceDiagnostic::error(
            span,
            format!(
                "expected \"vertical\", \"skewed\", or \"horizontal\", found {}",
                vanilla_type_name_class(other)
            ),
        )]),
    }
}

fn math_frac_operand(value: &Value, span: Span) -> SourceResult<Content> {
    super::casts::math_lr_content_value(value, span)
}

/// Parser/constructor compartilhado por `math.frac` e pelo bare `frac`.
pub(crate) fn math_frac_content(args: &Args) -> SourceResult<Content> {
    for positional in ["num", "denom"] {
        if args.named.contains_key(positional) {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("the argument `{positional}` is positional"),
            )]);
        }
    }
    if let Some((name, _)) = args.named.iter().find(|(name, _)| name.as_str() != "style")
    {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }
    let [num, den] = args.items.as_slice() else {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            match args.items.len() {
                0 => "missing argument: num",
                1 => "missing argument: denom",
                _ => "unexpected argument",
            }
            .to_string(),
        )]);
    };
    let mut style = None;
    for occurrence in args.occurrence_sequence() {
        if occurrence.name.as_deref() == Some("style") {
            let span = if occurrence.value_span.is_detached() {
                args.span
            } else {
                occurrence.value_span
            };
            style = Some(math_frac_style(&occurrence.value, span)?);
        }
    }
    Ok(Content::math_frac_full(
        math_frac_operand(num, args.span)?,
        math_frac_operand(den, args.span)?,
        style,
        false,
        false,
        true,
    ))
}

/// `math.frac(num, denom, style: ...)`.
pub fn native_math_frac(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    math_frac_content(args).map(Value::Content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::elements::math_frac::MathFracStyle;
    use crate::entities::value::Value;

    #[test]
    fn explicit_vertical_style_remains_distinct_from_omission() {
        let args = Args::positional(vec![
            Value::Content(Content::text("a")),
            Value::Content(Content::text("b")),
        ]);
        let omitted = math_frac_content(&args).unwrap();
        let mut explicit = args;
        explicit.named.insert("style".into(), Value::Str("vertical".into()));
        let explicit = math_frac_content(&explicit).unwrap();
        let Content::MathFrac(omitted) = omitted else { panic!("expected fraction") };
        let Content::MathFrac(explicit) = explicit else { panic!("expected fraction") };
        assert_eq!(omitted.style, None);
        assert_eq!(explicit.style, Some(MathFracStyle::Vertical));
    }
}

#[cfg(test)]
mod readiness_argument_tests {
    use super::*;
    use crate::contracts::world::World;
    use crate::entities::font_book::FontBook;
    use crate::entities::source::Source;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library,
    };
    struct TestWorld {
        library: Library,
        book: FontBook,
        source: Source,
    }

    impl TestWorld {
        fn new(text: &str) -> Self {
            Self {
                library: Library::new(),
                book: FontBook::new(),
                source: Source::new(
                    FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()),
                    text.into(),
                )
                .expect("valid fraction argument source fixture"),
            }
        }
    }

    impl World for TestWorld {
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

    fn run(text: &str) -> SourceResult<crate::entities::module::Module> {
        let world = TestWorld::new(text);
        crate::compiler::eval::eval_for_test(&world, &world.source)
    }
    fn style(text: &str) -> Option<crate::entities::elements::math_frac::MathFracStyle> {
        let module =
            run(text).unwrap_or_else(|e| panic!("fraction source must evaluate: {e:?}"));
        let mut styles = Vec::new();
        module.content().unwrap().visit_content_static(&mut |content| {
            if let Content::MathFrac(frac) = content {
                styles.push(frac.style);
                return false;
            }
            true
        });
        assert_eq!(styles.len(), 1, "one fraction must survive");
        styles[0]
    }
    #[test]
    fn readiness_frac_bound_style_accepts_call_site_override() {
        assert_eq!(
            style(
                r#"#let frac=math.frac.with(style:"vertical")
$frac(1,2,style:"horizontal")$"#
            ),
            Some(crate::entities::elements::math_frac::MathFracStyle::Horizontal)
        );
    }
    #[test]
    fn readiness_frac_chained_partial_keeps_last_style() {
        assert_eq!(
            style(
                r#"#let frac=math.frac.with(style:"vertical").with(style:"horizontal")
$frac(1,2)$"#
            ),
            Some(crate::entities::elements::math_frac::MathFracStyle::Horizontal)
        );
    }
    #[test]
    fn readiness_frac_invalid_bound_style_keeps_original_origin() {
        let text = r#"#let frac=math.frac.with(style:true)
$frac(1,2,style:"horizontal")$"#;
        let world = TestWorld::new(text);
        let errors = crate::compiler::eval::eval_for_test(&world, &world.source)
            .expect_err("invalid bound style is still cast");
        let error = errors
            .iter()
            .find(|e| {
                e.message
                    == r#"expected "vertical", "skewed", or "horizontal", found boolean"#
            })
            .unwrap_or_else(|| panic!("{errors:?}"));
        let start = text.find("true").unwrap();
        assert_eq!(world.source.span_byte_range(error.span), Some(start..start + 4));
    }
    #[test]
    fn readiness_frac_literal_duplicate_stays_a_source_error() {
        for text in [
            r#"$frac(1,2,style:"vertical",style:"horizontal")$"#,
            r#"#math.frac([A],[B],style:"vertical",style:"horizontal")"#,
        ] {
            let errors = run(text).expect_err("literal duplicate must reject");
            assert!(
                errors.iter().any(|e| e.message.contains("duplicate argument: style")),
                "{errors:?}"
            );
        }
    }
    #[test]
    fn readiness_frac_partial_argument_expressions_execute_once() {
        run(r#"#let n=0
#let bound=math.frac.with(style:{n+=1;"vertical"})
#bound([A],[B],style:{n+=1;"horizontal"})
#assert.eq(n,2)"#)
        .expect("both argument expressions once");
    }
}
