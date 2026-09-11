//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/foundations/float.md
//! @prompt-hash 497e6d31
//! @layer L1
//! @updated 2026-09-01
//!
//! Superfície e semântica numérica dos predicados públicos de `float`.

use crate::compiler::eval::operators::error_formatting::vanilla_type_name;
use crate::compiler::eval::EvalContext;
use crate::contracts::world::World;
use crate::entities::args::{ArgOccurrence, Args};
use crate::entities::bytes::Bytes;
use crate::entities::file_id::FileId;
use crate::entities::func::Func;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

fn native_is_infinite(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if let Some((name, _)) = args.named.first() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }

    let value = match args.items.as_slice() {
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: self".to_string(),
            )])
        }
        [Value::Float(value)] => *value,
        [Value::Int(value)] => *value as f64,
        [other] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected float, found {}", vanilla_type_name(other)),
            )])
        }
        [_, ..] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "unexpected argument".to_string(),
            )])
        }
    };

    Ok(Value::Bool(value.is_infinite()))
}

fn native_is_nan(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if let Some((name, _)) = args.named.first() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }

    let value = match args.items.as_slice() {
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: self".to_string(),
            )])
        }
        [Value::Float(value)] => *value,
        [Value::Int(value)] => *value as f64,
        [other] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected float, found {}", vanilla_type_name(other)),
            )])
        }
        [_, ..] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "unexpected argument".to_string(),
            )])
        }
    };

    Ok(Value::Bool(value.is_nan()))
}

/// Argument occurrences of the P1339 natives, retained until their owner
/// consumes them. The aggregate span is the whole call, including through With.
struct FloatCallArgs {
    remaining: Vec<ArgOccurrence>,
    span: Span,
}

impl FloatCallArgs {
    fn new(args: &Args) -> Self {
        Self {
            remaining: args.occurrence_sequence(),
            span: args.span,
        }
    }

    fn anchor(&self, span: Span) -> Span {
        if span.is_detached() {
            self.span
        } else {
            span
        }
    }

    fn required(&mut self, name: &str) -> SourceResult<ArgOccurrence> {
        if let Some(index) = self.remaining.iter().position(|arg| arg.name.is_none()) {
            return Ok(self.remaining.remove(index));
        }
        if let Some(arg) =
            self.remaining.iter().find(|arg| arg.name.as_deref() == Some(name))
        {
            return Err(vec![SourceDiagnostic::error(
                self.anchor(arg.span),
                format!("the argument `{name}` is positional"),
            )
            .with_hint(format!("try removing `{name}:`"))]);
        }
        Err(vec![SourceDiagnostic::error(self.span, format!("missing argument: {name}"))])
    }

    fn named<T>(
        &mut self,
        name: &str,
        mut value: T,
        cast: impl Fn(Value, Span) -> SourceResult<T>,
    ) -> SourceResult<T> {
        while let Some(index) = self
            .remaining
            .iter()
            .position(|arg| arg.name.as_deref() == Some(name))
        {
            let arg = self.remaining.remove(index);
            value = cast(arg.value, self.anchor(arg.value_span))?;
        }
        Ok(value)
    }

    fn finish(&self) -> SourceResult<()> {
        if let Some(arg) = self.remaining.first() {
            let message = match &arg.name {
                Some(name) => format!("unexpected argument: {name}"),
                None => "unexpected argument".into(),
            };
            return Err(vec![SourceDiagnostic::error(self.anchor(arg.span), message)]);
        }
        Ok(())
    }

    fn float(&mut self) -> SourceResult<f64> {
        let arg = self.required("self")?;
        match arg.value {
            Value::Float(value) => Ok(value),
            Value::Int(value) => Ok(value as f64),
            other => Err(vec![SourceDiagnostic::error(
                self.anchor(arg.value_span),
                format!("expected float, found {}", vanilla_type_name(&other)),
            )]),
        }
    }
}

fn float_endian(value: Value, span: Span) -> SourceResult<bool> {
    match value {
        Value::Str(value) if value == "little" => Ok(false),
        Value::Str(value) if value == "big" => Ok(true),
        other => {
            let message = if matches!(other, Value::Str(_)) {
                "expected \"big\" or \"little\"".to_string()
            } else {
                format!(
                    "expected \"big\" or \"little\", found {}",
                    vanilla_type_name(&other)
                )
            };
            Err(vec![SourceDiagnostic::error(span, message)])
        }
    }
}

fn float_byte_size(value: Value, span: Span) -> SourceResult<u32> {
    let message = match value {
        Value::Int(value) if value < 0 => "number must be at least zero".into(),
        Value::Int(value) => {
            return u32::try_from(value)
                .map_err(|_| vec![SourceDiagnostic::error(span, "number too large")])
        }
        other => format!("expected integer, found {}", vanilla_type_name(&other)),
    };
    Err(vec![SourceDiagnostic::error(span, message)])
}

fn native_signum(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let mut call = FloatCallArgs::new(args);
    let value = call.float()?;
    call.finish()?;
    Ok(Value::Float(value.signum()))
}

fn native_from_bytes(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let mut call = FloatCallArgs::new(args);
    let arg = call.required("bytes")?;
    let Value::Bytes(bytes) = arg.value else {
        return Err(vec![SourceDiagnostic::error(
            call.anchor(arg.value_span),
            format!("expected bytes, found {}", vanilla_type_name(&arg.value)),
        )]);
    };
    let big = call.named("endian", false, float_endian)?;
    call.finish()?;
    let value = match bytes.as_slice() {
        [a, b, c, d] => {
            let buffer = [*a, *b, *c, *d];
            (if big { f32::from_be_bytes(buffer) } else { f32::from_le_bytes(buffer) })
                as f64
        }
        [a, b, c, d, e, f, g, h] => {
            let buffer = [*a, *b, *c, *d, *e, *f, *g, *h];
            if big {
                f64::from_be_bytes(buffer)
            } else {
                f64::from_le_bytes(buffer)
            }
        }
        _ => {
            return Err(vec![SourceDiagnostic::error(
                call.span,
                "bytes must have a length of 4 or 8",
            )])
        }
    };
    Ok(Value::Float(value))
}

fn native_to_bytes(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let mut call = FloatCallArgs::new(args);
    let value = call.float()?;
    let big = call.named("endian", false, float_endian)?;
    let size = call.named("size", 8, float_byte_size)?;
    call.finish()?;
    let bytes = match size {
        4 => {
            let value = value as f32;
            if big {
                value.to_be_bytes().to_vec()
            } else {
                value.to_le_bytes().to_vec()
            }
        }
        8 => {
            if big {
                value.to_be_bytes().to_vec()
            } else {
                value.to_le_bytes().to_vec()
            }
        }
        _ => {
            return Err(vec![SourceDiagnostic::error(
                call.span,
                "size must be either 4 or 8",
            )])
        }
    };
    Ok(Value::Bytes(Bytes::new(bytes)))
}

pub(crate) fn float_type_field(field: &str) -> Option<Value> {
    match field {
        "inf" => Some(Value::Float(f64::INFINITY)),
        "nan" => Some(Value::Float(f64::NAN)),
        "signum" => Some(Value::Func(Func::native("signum", native_signum))),
        "from-bytes" => Some(Value::Func(Func::native("from-bytes", native_from_bytes))),
        "to-bytes" => Some(Value::Func(Func::native("to-bytes", native_to_bytes))),
        "is-infinite" => {
            Some(Value::Func(Func::native("is-infinite", native_is_infinite)))
        }
        "is-nan" => Some(Value::Func(Func::native("is-nan", native_is_nan))),
        _ => None,
    }
}

pub(crate) fn is_float_instance_method(method: &str) -> bool {
    matches!(method, "is-infinite" | "is-nan" | "signum" | "from-bytes" | "to-bytes")
}

pub(crate) fn dispatch_float_method(
    receiver: f64,
    method: &str,
    args: Args,
    ctx: &mut EvalContext,
    world: &dyn World,
    current_file: FileId,
) -> SourceResult<Value> {
    dispatch_float_method_spanned(
        receiver,
        Span::detached(),
        method,
        args,
        ctx,
        world,
        current_file,
    )
}

pub(crate) fn dispatch_float_method_spanned(
    receiver: f64,
    receiver_span: Span,
    method: &str,
    mut args: Args,
    ctx: &mut EvalContext,
    world: &dyn World,
    current_file: FileId,
) -> SourceResult<Value> {
    args = if args.occurrences.is_some() || !receiver_span.is_detached() {
        let mut occurrences = args.occurrence_sequence();
        occurrences.insert(
            0,
            ArgOccurrence {
                name: None,
                value: Value::Float(receiver),
                span: receiver_span,
                value_span: receiver_span,
            },
        );
        Args::from_occurrences(args.span, occurrences)
    } else {
        let mut items = args.items;
        items.insert(0, Value::Float(receiver));
        Args::from_parts(items, args.named, args.span)
    };
    match method {
        "is-infinite" => native_is_infinite(ctx, &args, world, current_file),
        "is-nan" => native_is_nan(ctx, &args, world, current_file),
        "signum" => native_signum(ctx, &args, world, current_file),
        "from-bytes" => native_from_bytes(ctx, &args, world, current_file),
        "to-bytes" => native_to_bytes(ctx, &args, world, current_file),
        _ => unreachable!("dispatch_float_method called with unknown method: {method}"),
    }
}

#[cfg(test)]
mod tests_p1293_is_nan {
    use super::*;
    use crate::compiler::eval::eval_expression;
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
            test_file_id()
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

    fn test_file_id() -> FileId {
        FileId::from_raw(NonZeroU16::new(1).unwrap())
    }

    fn call_static(args: Args) -> SourceResult<Value> {
        native_is_nan(
            &mut EvalContext::new(),
            &args,
            &NullWorld::default(),
            test_file_id(),
        )
    }

    #[test]
    fn p1339_float_public_numeric_values_and_bytes() {
        for (expression, expected) in [
            ("repr((type(float.inf), float.inf, float.nan == float.nan))", "(float, float.inf, false)"),
            ("repr((float.signum(-0.0), float.signum(0.0), float.signum(-3), float.signum(float.nan)))", "(-1.0, 1.0, -1.0, float.nan)"),
            ("repr(float.from-bytes(bytes((0,0,192,63))))", "1.5"),
            ("repr(float.from-bytes(float.to-bytes(0.1,size:4)))", "0.10000000149011612"),
        ] {
            let (result, _) = eval_expression(&NullWorld::default(), expression);
            assert_eq!(result.unwrap(), Value::Str(expected.into()), "{expression}");
        }
        // Bytes are themselves the API result. Inspect the returned value
        // directly, without depending on the unrelated array constructor.
        for (expression, expected) in [
            ("float.to-bytes(1.5)", vec![0, 0, 0, 0, 0, 0, 248, 63]),
            ("float.to-bytes(-0.0, size:4, endian:\"big\")", vec![128, 0, 0, 0]),
        ] {
            let (result, _) = eval_expression(&NullWorld::default(), expression);
            assert_eq!(
                result.unwrap(),
                Value::Bytes(crate::entities::bytes::Bytes::new(expected)),
                "{expression}"
            );
        }
    }

    #[test]
    fn p1339_float_closed_arguments_and_diagnostic_origin() {
        for (expression, message, needle, hint) in [
            (
                "float.signum(self:1.0)",
                "the argument `self` is positional",
                "self:1.0",
                Some("try removing `self:`"),
            ),
            ("float.to-bytes(1.0,size:-1)", "number must be at least zero", "-1", None),
            (
                "float.to-bytes(1.0,size:3)",
                "size must be either 4 or 8",
                "float.to-bytes(1.0,size:3)",
                None,
            ),
            (
                "float.from-bytes(bytes((0,0,0)))",
                "bytes must have a length of 4 or 8",
                "float.from-bytes(bytes((0,0,0)))",
                None,
            ),
            ("float.signum(true)", "expected float, found boolean", "true", None),
            (
                "float.to-bytes(1.0,endian:\"middle\")",
                "expected \"big\" or \"little\"",
                "\"middle\"",
                None,
            ),
        ] {
            let world = NullWorld::default();
            let (result, _) = eval_expression(&world, expression);
            let errors = result.unwrap_err();
            assert_eq!(errors[0].message, message, "{expression}");
            assert_eq!(errors[0].hints, hint.into_iter().collect::<Vec<_>>());
            let source = Source::new_with_parser(
                world.main(),
                expression.into(),
                crate::compiler::parse::parse_code,
            );
            let offset = expression.find(needle).unwrap();
            assert_eq!(
                source.span_byte_range(errors[0].span),
                Some(offset..offset + needle.len()),
                "{expression}"
            );
        }
    }

    fn public_error_range(expression: &str) -> std::ops::Range<usize> {
        let world = NullWorld::default();
        let (result, _) = eval_expression(&world, expression);
        let errors = result.expect_err("a expressão pública deve falhar");
        let source = Source::new_with_parser(
            world.main(),
            expression.to_string(),
            crate::compiler::parse::parse_code,
        );
        source
            .span_byte_range(errors[0].span)
            .expect("o span deve resolver contra a expressão pública")
    }

    #[test]
    fn p1293_is_nan_tem_nome_publico_curto_e_forma_ligada() {
        let Value::Func(func) = float_type_field("is-nan")
            .expect("float.is-nan deve existir como função estática")
        else {
            panic!("float.is-nan deve ser função");
        };

        assert_eq!(func.name(), Some("is-nan"));
        assert!(is_float_instance_method("is-nan"));
    }

    #[test]
    fn p1293_is_nan_so_reconhece_nan_e_coage_inteiro_na_forma_estatica() {
        for value in [
            Value::Float(0.0),
            Value::Float(-42.5),
            Value::Float(f64::INFINITY),
            Value::Float(f64::NEG_INFINITY),
            Value::Int(42),
        ] {
            assert_eq!(
                call_static(Args::positional(vec![value])).unwrap(),
                Value::Bool(false)
            );
        }

        assert_eq!(
            call_static(Args::positional(vec![Value::Float(f64::NAN)])).unwrap(),
            Value::Bool(true)
        );
    }

    #[test]
    fn p1293_is_nan_forma_ligada_delega_a_mesma_nativa() {
        assert_eq!(
            dispatch_float_method(
                f64::NAN,
                "is-nan",
                Args::positional(vec![]),
                &mut EvalContext::new(),
                &NullWorld::default(),
                test_file_id(),
            )
            .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            dispatch_float_method(
                f64::INFINITY,
                "is-nan",
                Args::positional(vec![]),
                &mut EvalContext::new(),
                &NullWorld::default(),
                test_file_id(),
            )
            .unwrap(),
            Value::Bool(false)
        );
    }

    #[test]
    fn p1293_is_nan_fecha_aridade_named_e_tipo() {
        let missing = call_static(Args::positional(vec![])).unwrap_err();
        assert_eq!(missing[0].message, "missing argument: self");

        let extra =
            call_static(Args::positional(vec![Value::Float(0.0), Value::Float(1.0)]))
                .unwrap_err();
        assert_eq!(extra[0].message, "unexpected argument");

        let mut named = Args::positional(vec![Value::Float(0.0)]);
        named.named.insert("other".into(), Value::Bool(true));
        let named = call_static(named).unwrap_err();
        assert_eq!(named[0].message, "unexpected argument: other");

        let invalid = call_static(Args::positional(vec![Value::Bool(true)])).unwrap_err();
        assert_eq!(invalid[0].message, "expected float, found boolean");
    }

    #[test]
    fn p1293_is_nan_span_missing_ancora_chamada_estatica() {
        assert_eq!(public_error_range("float.is-nan()"), 0..14);
    }

    #[test]
    fn p1293_is_nan_span_extra_ancora_posicional_excedente() {
        assert_eq!(public_error_range("float.is-nan(0.0, 1.0)"), 18..21);
    }

    #[test]
    fn p1293_is_nan_span_named_bound_ancora_named_completo() {
        assert_eq!(public_error_range("float(\"NaN\").is-nan(other: true)"), 20..31);
    }

    #[test]
    fn p1293_is_nan_span_cast_ancora_primeiro_posicional() {
        assert_eq!(public_error_range("float.is-nan(\"x\")"), 13..16);
    }

    #[test]
    fn p1293_is_nan_span_acesso_sem_chamada_ancora_campo() {
        assert_eq!(public_error_range("float(\"NaN\").is-nan"), 13..19);
    }
}
