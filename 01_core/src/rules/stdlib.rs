//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/eval.md
//! @prompt-hash f883240f
//! @layer L1
//! @updated 2026-03-28

//! Stdlib nativa mínima — Passo 17.
//!
//! Interface `fn(&[Value]) -> SourceResult<Value>`: sem moves, testável
//! directamente sem world nem eval_for_test.

use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

fn err(msg: impl Into<String>) -> SourceResult<Value> {
    Err(vec![SourceDiagnostic::error(Span::detached(), msg.into())])
}

/// `type(v)` → nome do tipo como string Typst.
pub fn native_type(args: &[Value]) -> SourceResult<Value> {
    match args {
        [v] => Ok(Value::Str(v.type_name().into())),
        _   => err(format!("type() requer 1 argumento, recebeu {}", args.len())),
    }
}

/// `len(v)` → comprimento de Str, Array ou Dict.
pub fn native_len(args: &[Value]) -> SourceResult<Value> {
    match args {
        [Value::Str(s)]   => Ok(Value::Int(s.chars().count() as i64)),
        [Value::Array(a)] => Ok(Value::Int(a.len() as i64)),
        [Value::Dict(d)]  => Ok(Value::Int(d.len() as i64)),
        [other]           => err(format!("len() não suporta {}", other.type_name())),
        _                 => err(format!("len() requer 1 argumento, recebeu {}", args.len())),
    }
}

/// `range(n)` → Array de 0..n; `range(start, end)` → Array de start..end.
pub fn native_range(args: &[Value]) -> SourceResult<Value> {
    match args {
        [Value::Int(n)] => {
            if *n < 0 {
                return err("range() requer argumento não-negativo");
            }
            Ok(Value::Array((0..*n).map(Value::Int).collect()))
        }
        [Value::Int(start), Value::Int(end)] => {
            let items = if start <= end {
                (*start..*end).map(Value::Int).collect()
            } else {
                vec![]
            };
            Ok(Value::Array(items))
        }
        _ => err(format!("range() requer 1 ou 2 Int, recebeu {} args", args.len())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_type_directo() {
        // SourceDiagnostic não implementa PartialEq — usar .unwrap() em vez de assert_eq! com Ok(...)
        assert_eq!(native_type(&[Value::Int(1)]).unwrap(),     Value::Str("int".into()));
        assert_eq!(native_type(&[Value::Bool(true)]).unwrap(), Value::Str("bool".into()));
        assert_eq!(native_type(&[Value::None]).unwrap(),       Value::Str("none".into()));
        assert!(native_type(&[]).is_err());
        assert!(native_type(&[Value::Int(1), Value::Int(2)]).is_err());
    }

    #[test]
    fn native_len_directo() {
        assert_eq!(native_len(&[Value::Str("abc".into())]).unwrap(),
                   Value::Int(3));
        assert_eq!(native_len(&[Value::Array(vec![Value::Int(1), Value::Int(2)])]).unwrap(),
                   Value::Int(2));
        assert!(native_len(&[Value::Int(1)]).is_err());
        assert!(native_len(&[]).is_err());
    }

    #[test]
    fn native_range_directo() {
        assert_eq!(native_range(&[Value::Int(3)]).unwrap(),
                   Value::Array(vec![Value::Int(0), Value::Int(1), Value::Int(2)]));
        assert_eq!(native_range(&[Value::Int(2), Value::Int(5)]).unwrap(),
                   Value::Array(vec![Value::Int(2), Value::Int(3), Value::Int(4)]));
        assert_eq!(native_range(&[Value::Int(3), Value::Int(3)]).unwrap(),
                   Value::Array(vec![]));
        assert!(native_range(&[Value::Int(-1)]).is_err());
    }
}
