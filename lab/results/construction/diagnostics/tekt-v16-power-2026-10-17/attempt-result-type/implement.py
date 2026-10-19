from pathlib import Path
import hashlib,json,gzip
out=Path(__file__).resolve().parents[1];p=Path('01_core/src/compiler/stdlib/calc/power.rs');checkpoint=json.load(open(out/'tests-before-code.json'));assert hashlib.sha256(p.read_bytes()).hexdigest()==checkpoint['sha256']
log=(out/'red.log').read_text();assert '3 failed' in log and '5 passed' in log and 'test result: FAILED' in log
s=p.read_text();start=s.index('    let base_zero = match base {');end=s.index('\npub(crate) fn calc_sqrt(')
s=s[:start]+'''    let base = power_base(base, "calc.pow() base", "Int, Float ou Decimal")?;
    let exp = match power_base(exp, "calc.pow() expoente", "Int ou Float")? {
        PowerBase::Int(i) => PowerExponent::Int(i),
        PowerBase::Float(f) => PowerExponent::Float(f),
        PowerBase::Decimal(_) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "expected integer or float, found decimal",
            ).with_hint(
                "if loss of precision is acceptable, explicitly cast the decimal to a float with `float(value)`",
            )]);
        }
    };
    let base_zero = match base {
        PowerBase::Int(i) => i == 0,
        PowerBase::Float(f) => f == 0.0,
        PowerBase::Decimal(d) => d.0.is_zero(),
    };
    let exp_zero = match exp {
        PowerExponent::Int(i) => i == 0,
        PowerExponent::Float(f) => f == 0.0,
    };
    if base_zero && exp_zero {
        return err("calc.pow() zero elevado a zero é indefinido");
    }
    match exp {
        PowerExponent::Int(i) => {
            if i32::try_from(i).is_err() {
                return err("calc.pow() expoente demasiado grande");
            }
        }
        PowerExponent::Float(f) => {
            if !f.is_normal() && f != 0.0 {
                return err("calc.pow() expoente não pode ser infinito, subnormal ou NaN");
            }
        }
    }

    match (base, exp) {
        (PowerBase::Int(a), PowerExponent::Int(b)) if b >= 0 => {
            a.checked_pow(b as u32).map(Value::Int).ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    args.span,
                    "calc.pow() resultado fora do alcance i64".to_string(),
                )]
            })
        }
        (PowerBase::Int(a), PowerExponent::Int(b)) => {
            #[allow(clippy::disallowed_methods)]
            guard_float((a as f64).powi(b as i32))
        }
        (PowerBase::Decimal(d), PowerExponent::Int(b)) => d.checked_powi(b).map(Value::Decimal).ok_or_else(|| {
            vec![SourceDiagnostic::error(
                args.span,
                "calc.pow() resultado fora do alcance decimal".to_string(),
            )]
        }),
        (PowerBase::Decimal(_), PowerExponent::Float(_)) => Err(vec![SourceDiagnostic::error(
            args.span,
            "cannot apply this operation to a decimal and a float".to_string(),
        )
        .with_hint(
            "if loss of precision is acceptable, explicitly cast the decimal to a float with `float(value)`",
        )]),
        (PowerBase::Float(b), PowerExponent::Int(e)) => {
            // Preserve the existing guarded floating path for floating bases.
            #[allow(clippy::disallowed_methods)]
            guard_float(b.powf(e as f64))
        }
        (PowerBase::Float(b), PowerExponent::Float(e)) => {
            #[allow(clippy::disallowed_methods)]
            guard_float(b.powf(e))
        }
        (PowerBase::Int(b), PowerExponent::Float(e)) => {
            #[allow(clippy::disallowed_methods)]
            guard_float((b as f64).powf(e))
        }
    }
}

#[derive(Clone, Copy)]
enum PowerBase {
    Int(i64),
    Float(f64),
    Decimal(crate::entities::decimal::Decimal),
}

#[derive(Clone, Copy)]
enum PowerExponent {
    Int(i64),
    Float(f64),
}

fn power_base(value: &Value, context: &str, expected: &str) -> SourceResult<PowerBase> {
    match value {
        Value::Int(i) => Ok(PowerBase::Int(*i)),
        Value::Float(f) => Ok(PowerBase::Float(*f)),
        Value::Decimal(d) => Ok(PowerBase::Decimal(*d)),
        Value::None
        | Value::Bool(_)
        | Value::Str(_)
        | Value::Array(_)
        | Value::Dict(_)
        | Value::Module(_)
        | Value::Datetime(_)
        | Value::Func(_)
        | Value::Content(_)
        | Value::LocatedContent(..)
        | Value::Auto
        | Value::Length(_)
        | Value::Relative(_)
        | Value::Ratio(_)
        | Value::Angle(_)
        | Value::Color(_)
        | Value::Stroke(_)
        | Value::Fraction(_)
        | Value::Align(_)
        | Value::Location(_)
        | Value::Gradient(_)
        | Value::Regex(_)
        | Value::Tiling(_)
        | Value::Bytes(_)
        | Value::Duration(_)
        | Value::Version(_)
        | Value::Selector(_)
        | Value::Symbol(_)
        | Value::Args(_)
        | Value::State(_)
        | Value::Counter(_)
        | Value::Label(_)
        | Value::Dir(_)
        | Value::Path(_)
        | Value::Type(_)
        | Value::SpotColorant(_) => err(format!(
            "{context}: esperava {expected}, recebeu {}",
            value.type_name()
        )),
    }
}
''' +s[end:]
p.write_text(s)
