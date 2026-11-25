//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/calc/numeric.md
//! @prompt-hash e99e2637
//! @layer L1
//! @updated 2026-09-30

use super::*;
use crate::compiler::eval::operators::error_formatting::vanilla_type_name;
use crate::entities::decimal::Decimal;

pub(super) fn coerce_to_f64(v: &Value, ctx: &str) -> SourceResult<f64> {
    match v {
        Value::Int(i) => Ok(*i as f64),
        Value::Float(f) => Ok(*f),
        other => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{ctx}: esperava Int ou Float, recebeu {}", other.type_name()),
        )]),
    }
}

/// Rejeita NaN; o infinito é um resultado válido para quem chama.
pub(super) fn reject_nan(f: f64, span: Span) -> SourceResult<Value> {
    if f.is_nan() {
        Err(not_a_real_number(span))
    } else {
        Ok(Value::Float(f))
    }
}

pub(super) fn not_a_real_number(span: Span) -> Vec<SourceDiagnostic> {
    vec![SourceDiagnostic::error(span, "the result is not a real number")]
}

pub(super) fn too_large(span: Span) -> Vec<SourceDiagnostic> {
    vec![SourceDiagnostic::error(span, "the result is too large")]
}

/// Converte um valor já arredondado. NaN dá 0; fora de i64 falha.
pub(super) fn float_to_int(f: f64, span: Span) -> SourceResult<Value> {
    if f.is_nan() {
        return Ok(Value::Int(0));
    }
    if (-9_223_372_036_854_775_808.0..9_223_372_036_854_775_808.0).contains(&f) {
        Ok(Value::Int(f as i64))
    } else {
        Err(too_large(span))
    }
}

/// Span do valor do n-ésimo argumento posicional, quando há origens.
pub(super) fn positional_span(args: &Args, positional: usize) -> Span {
    args.occurrences
        .as_ref()
        .and_then(|items| items.iter().filter(|arg| arg.name.is_none()).nth(positional))
        .map(|arg| arg.value_span)
        .unwrap_or_else(Span::detached)
}

#[derive(Clone, Copy)]
pub(super) enum DecNum {
    Int(i64),
    Float(f64),
    Decimal(Decimal),
}

/// Dois operandos trazidos a um tipo comum.
pub(super) enum DecNumPair {
    Int(i64, i64),
    Float(f64, f64),
    Decimal(Decimal, Decimal),
}

impl DecNum {
    pub(super) fn from_arg(args: &Args, positional: usize) -> SourceResult<Self> {
        match &args.items[positional] {
            Value::Int(i) => Ok(Self::Int(*i)),
            Value::Float(f) => Ok(Self::Float(*f)),
            Value::Decimal(d) => Ok(Self::Decimal(*d)),
            other => Err(vec![SourceDiagnostic::error(
                positional_span(args, positional),
                format!(
                    "expected integer, float, or decimal, found {}",
                    vanilla_type_name(other)
                ),
            )]),
        }
    }

    pub(super) fn is_zero(self) -> bool {
        match self {
            Self::Int(i) => i == 0,
            Self::Float(f) => f == 0.0,
            Self::Decimal(d) => d.0.is_zero(),
        }
    }

    pub(super) fn is_nan(self) -> bool {
        match self {
            Self::Float(f) => f.is_nan(),
            Self::Int(_) | Self::Decimal(_) => false,
        }
    }

    pub(super) fn decimal(self) -> Option<Decimal> {
        match self {
            Self::Int(i) => Some(Decimal::from_i64(i)),
            Self::Decimal(d) => Some(d),
            Self::Float(_) => None,
        }
    }

    pub(super) fn float(self) -> Option<f64> {
        match self {
            Self::Int(i) => Some(i as f64),
            Self::Float(f) => Some(f),
            Self::Decimal(_) => None,
        }
    }

    /// `None` quando um é Decimal e o outro é Float.
    pub(super) fn pair(self, other: Self) -> Option<DecNumPair> {
        Some(match (self, other) {
            (Self::Int(a), Self::Int(b)) => DecNumPair::Int(a, b),
            (Self::Decimal(a), Self::Decimal(b)) => DecNumPair::Decimal(a, b),
            (Self::Decimal(a), Self::Int(b)) => {
                DecNumPair::Decimal(a, Decimal::from_i64(b))
            }
            (Self::Int(a), Self::Decimal(b)) => {
                DecNumPair::Decimal(Decimal::from_i64(a), b)
            }
            (Self::Float(a), Self::Float(b)) => DecNumPair::Float(a, b),
            (Self::Float(a), Self::Int(b)) => DecNumPair::Float(a, b as f64),
            (Self::Int(a), Self::Float(b)) => DecNumPair::Float(a as f64, b),
            (Self::Decimal(_), Self::Float(_)) | (Self::Float(_), Self::Decimal(_)) => {
                return None
            }
        })
    }
}

pub(super) fn decimal_and_float(span: Span) -> Vec<SourceDiagnostic> {
    vec![SourceDiagnostic::error(
        span,
        "cannot apply this operation to a decimal and a float",
    )
    .with_hint(
        "if loss of precision is acceptable, explicitly cast the decimal to a float with `float(value)`",
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_modeled_result() {
        assert!(reject_nan(f64::NAN, Span::detached()).is_err());
        assert_eq!(reject_nan(1.0, Span::detached()).unwrap(), Value::Float(1.0));
        assert_eq!(
            reject_nan(f64::INFINITY, Span::detached()).unwrap(),
            Value::Float(f64::INFINITY)
        );
    }

    #[test]
    fn float_to_int_rejects_instead_of_saturating() {
        let span = Span::detached();
        assert_eq!(
            float_to_int(-9_223_372_036_854_775_808.0, span).unwrap(),
            Value::Int(i64::MIN)
        );
        assert_eq!(
            float_to_int(9_223_372_036_854_774_784.0, span).unwrap(),
            Value::Int(9_223_372_036_854_774_784)
        );
        assert_eq!(float_to_int(f64::NAN, span).unwrap(), Value::Int(0));
        for value in [
            9_223_372_036_854_775_808.0,
            -9_223_372_036_854_777_856.0,
            f64::INFINITY,
            f64::NEG_INFINITY,
        ] {
            assert!(float_to_int(value, span).is_err());
        }
    }

    #[test]
    fn decimal_and_float_never_pair() {
        let decimal = DecNum::Decimal(Decimal::from_i64(1));
        assert!(decimal.pair(DecNum::Float(1.0)).is_none());
        assert!(DecNum::Float(1.0).pair(decimal).is_none());
        for (a, b) in [(decimal, DecNum::Int(2)), (DecNum::Int(2), decimal)] {
            assert!(matches!(a.pair(b), Some(DecNumPair::Decimal(..))));
        }
        for (a, b) in
            [(DecNum::Float(1.0), DecNum::Int(2)), (DecNum::Int(2), DecNum::Float(1.0))]
        {
            assert!(matches!(a.pair(b), Some(DecNumPair::Float(..))));
        }
    }
}
