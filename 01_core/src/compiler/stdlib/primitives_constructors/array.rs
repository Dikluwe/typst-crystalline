//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/primitives-constructors/array.md
//! @prompt-hash 15391e08
//! @layer L1
//! @updated 2026-09-10

use crate::compiler::eval::EvalContext;
use crate::contracts::world::World;
use crate::entities::args::Args;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::value::Value;

/// Narrow Bytes conversion; the dispatcher retains all other Array routes.
pub(crate) fn native_array_bytes(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let mut remaining = args.clone();
    let Some(Value::Bytes(bytes)) = remaining.remove_positional(0) else {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "type array does not have a constructor",
        )]);
    };
    if let Some(arg) = remaining.occurrence_sequence().first() {
        let message = match &arg.name {
            Some(name) => format!("unexpected argument: {name}"),
            None => "unexpected argument".into(),
        };
        let span = if arg.span.is_detached() { args.span } else { arg.span };
        return Err(vec![SourceDiagnostic::error(span, message)]);
    }
    Ok(Value::Array(
        bytes
            .as_slice()
            .iter()
            .map(|byte| Value::Int(i64::from(*byte)))
            .collect(),
    ))
}

#[cfg(test)]
mod tests {
    use super::super::test_support::null_world;
    use crate::compiler::eval::eval_expression;
    use crate::entities::value::Value;

    #[cfg(p1339_observation)]
    mod independent_owner {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../00_nucleo/diagnosticos/p1339-contract-array-owner-harness-r2.rs"
        ));
    }

    #[cfg(p1339_observation)]
    #[test]
    fn p1339_array_frozen_owner_oracle() {
        let world = null_world();
        let mut ctx = crate::compiler::eval::EvalContext::new();
        let file = super::super::test_support::test_file_id();
        independent_owner::assert_array_owner(file, |args| {
            super::native_array_bytes(&mut ctx, args, &world, file)
        });
    }

    #[test]
    fn p1339_array_bytes_preserves_unsigned_order_and_empty() {
        let world = null_world();
        for (expression, expected) in [
            ("array(bytes(()))", vec![]),
            ("array(bytes((255, 0, 128, 127, 255)))", vec![255, 0, 128, 127, 255]),
            ("{ let convert = array; convert(bytes((1, 255))) }", vec![1, 255]),
            ("array(..(bytes((0, 255)),))", vec![0, 255]),
        ] {
            let (result, _) = eval_expression(&world, expression);
            let Value::Array(values) =
                result.unwrap_or_else(|e| panic!("{expression}: {e:?}"))
            else {
                panic!("{expression}: expected Array");
            };
            assert_eq!(values.len(), expected.len(), "{expression}");
            for (actual, expected) in values.iter().zip(expected) {
                assert!(
                    matches!(actual, Value::Int(value) if *value == expected),
                    "{expression}: {actual:?}"
                );
            }
        }
    }

    #[test]
    fn p1339_array_bytes_rejects_surplus() {
        let world = null_world();
        for (expression, message) in [
            ("array(bytes((1,)), 2)", "unexpected argument"),
            ("array(bytes((1,)), extra: 2)", "unexpected argument: extra"),
        ] {
            let (result, _) = eval_expression(&world, expression);
            let errors = result.expect_err(expression);
            assert_eq!(errors[0].message.as_str(), message, "{expression}");
        }
    }

    #[test]
    fn p1339_array_spread_error_preserves_call_trace() {
        use crate::entities::source_result::Tracepoint;
        let world = null_world();
        let (result, _) = eval_expression(
            &world,
            "{ let a = arguments(bytes((1,)), other: 2); array(..a) }",
        );
        let errors = result.expect_err("spread surplus must fail");
        assert_eq!(errors[0].message, "unexpected argument: other");
        assert_eq!(errors[0].trace.len(), 1);
        assert_eq!(errors[0].trace[0].v, Tracepoint::Call(Some("array".into())));
    }

    #[test]
    fn p1339_array_non_bytes_keeps_legacy_route() {
        let world = null_world();
        for expression in [
            "array()",
            "array((1, 2))",
            "array(version(1, 2))",
            "array(value: bytes((1,)))",
            "array(1)",
        ] {
            let (result, _) = eval_expression(&world, expression);
            let errors = result.expect_err(expression);
            assert_eq!(
                errors[0].message.as_str(),
                "type array does not have a constructor",
                "{expression}"
            );
        }
    }
}
