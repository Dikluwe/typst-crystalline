//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/primitives-constructors.md
//! @prompt-hash 5b6064d7
//! @layer L1
//! @updated 2026-08-23

use super::super::{err, expect_no_named};
use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::decimal::Decimal;
use crate::entities::file_id::FileId;
use crate::entities::source_result::SourceResult;
use crate::entities::value::Value;

/// `decimal(s)` → `Value::Decimal`.
///
/// Parse a string decimal via `Decimal::from_str`. Rejeita argumentos nomeados
/// e qualquer tipo que não seja `Str`.
pub fn native_decimal(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(s)] => match Decimal::from_str(s) {
            Some(d) => Ok(Value::Decimal(d)),
            None => err(format!("decimal(): string inválida: '{}'", s)),
        },
        [other] => err(format!("decimal(): espera Str, recebeu {}", other.type_name())),
        _ => err(format!("decimal(): requer 1 argumento, recebeu {}", args.items.len())),
    }
}

/// `duration(...)` → `Value::Duration`.
///
/// Duas formas suportadas:
/// 1. Named args vanilla: `duration(days: 3, hours: 2, minutes: 30)`.
/// 2. String posicional (compatibilidade P403): `duration("1h30m")`.

#[cfg(test)]
mod tests {
    use super::super::test_support::*;
    use super::*;
    #[test]
    fn decimal_valid() {
        let v = native_decimal(
            &mut ctx(),
            &p(vec![Value::Str("1.23".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(v, Value::Decimal(Decimal::from_str("1.23").unwrap()));
    }

    #[test]
    fn decimal_invalid_string() {
        assert!(native_decimal(
            &mut ctx(),
            &p(vec![Value::Str("abc".into())]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    #[test]
    fn decimal_wrong_type() {
        assert!(native_decimal(
            &mut ctx(),
            &p(vec![Value::Int(1)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    #[test]
    fn decimal_too_many_args() {
        assert!(native_decimal(
            &mut ctx(),
            &p(vec![Value::Str("1.5".into()), Value::Str("2.0".into())]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }
}
