//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/foundations/float.md
//! @prompt-hash 7f4ef0b6
//! @layer L1
//! @updated 2026-09-01
//!
//! Superfície e semântica numérica dos predicados públicos de `float`.

use crate::compiler::eval::operators::error_formatting::vanilla_type_name;
use crate::compiler::eval::EvalContext;
use crate::contracts::world::World;
use crate::entities::args::Args;
use crate::entities::file_id::FileId;
use crate::entities::func::Func;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
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

pub(crate) fn float_type_field(field: &str) -> Option<Value> {
    match field {
        "is-infinite" => {
            Some(Value::Func(Func::native("is-infinite", native_is_infinite)))
        }
        "is-nan" => Some(Value::Func(Func::native("is-nan", native_is_nan))),
        _ => None,
    }
}

pub(crate) fn is_float_instance_method(method: &str) -> bool {
    matches!(method, "is-infinite" | "is-nan")
}

pub(crate) fn dispatch_float_method(
    receiver: f64,
    method: &str,
    mut args: Args,
    ctx: &mut EvalContext,
    world: &dyn World,
    current_file: FileId,
) -> SourceResult<Value> {
    match method {
        "is-infinite" => {
            args.items.insert(0, Value::Float(receiver));
            native_is_infinite(ctx, &args, world, current_file)
        }
        "is-nan" => {
            args.items.insert(0, Value::Float(receiver));
            native_is_nan(ctx, &args, world, current_file)
        }
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
