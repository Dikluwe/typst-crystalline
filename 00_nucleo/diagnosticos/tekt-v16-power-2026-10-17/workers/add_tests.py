from pathlib import Path
import gzip,json,hashlib,datetime
out=Path(__file__).resolve().parents[1];p=Path('01_core/src/compiler/stdlib/calc/power.rs');b=json.load(gzip.open(out/'before.json.gz'));assert p.read_text()==b['files'][str(p)]
p.write_text(p.read_text()+'''
#[cfg(test)]
mod v16_power_tests {
    use super::super::calc_test_support::call;
    use super::*;

    #[test]
    fn v16_power_invalid_base_before_large_exponent() {
        let diagnostics = call(calc_pow, vec![Value::None, Value::Int(2_147_483_648)]).unwrap_err();
        assert!(diagnostics[0].message.contains("base"));
        assert!(diagnostics[0].message.contains("Decimal"));
    }

    #[test]
    fn v16_power_invalid_base_before_non_normal_exponent() {
        let diagnostics = call(calc_pow, vec![Value::None, Value::Float(f64::NAN)]).unwrap_err();
        assert!(diagnostics[0].message.contains("base"));
    }

    #[test]
    fn v16_power_decimal_exponent_has_conversion_hint() {
        let diagnostics = call(calc_pow, vec![Value::Int(2), Value::Decimal(Decimal::new(0, 0))]).unwrap_err();
        assert_eq!(diagnostics[0].hints, vec!["if loss of precision is acceptable, explicitly cast the decimal to a float with `float(value)`"]);
    }

    #[test]
    fn v16_power_numeric_kinds() {
        for (base, exponent, expected) in [
            (Value::Int(2), Value::Int(3), Value::Int(8)),
            (Value::Int(2), Value::Int(-1), Value::Float(0.5)),
            (Value::Int(2), Value::Float(3.0), Value::Float(8.0)),
            (Value::Float(2.0), Value::Int(3), Value::Float(8.0)),
            (Value::Float(2.0), Value::Float(3.0), Value::Float(8.0)),
            (Value::Decimal(Decimal::new(25, 1)), Value::Int(3), Value::Decimal(Decimal::new(15625, 3))),
            (Value::Decimal(Decimal::new(2, 0)), Value::Int(-1), Value::Decimal(Decimal::new(5, 1))),
        ] {
            assert_eq!(call(calc_pow, vec![base, exponent]).unwrap(), expected);
        }
    }

    #[test]
    fn v16_power_zero_and_overflow() {
        for base in [Value::Int(0), Value::Float(0.0), Value::Decimal(Decimal::new(0, 0))] {
            for exponent in [Value::Int(0), Value::Float(0.0)] {
                let diagnostics = call(calc_pow, vec![base.clone(), exponent]).unwrap_err();
                assert!(diagnostics[0].message.contains("zero"));
            }
        }
        for exponent in [63, 2_147_483_648] {
            assert!(call(calc_pow, vec![Value::Int(2), Value::Int(exponent)]).is_err());
        }
    }

    #[test]
    fn v16_power_decimal_float_diagnostic() {
        let diagnostics = call(calc_pow, vec![Value::Decimal(Decimal::new(2, 0)), Value::Float(2.0)]).unwrap_err();
        assert_eq!(diagnostics[0].message, "cannot apply this operation to a decimal and a float");
        assert_eq!(diagnostics[0].hints.len(), 1);
    }

    #[test]
    fn v16_power_invalid_arguments() {
        assert!(call(calc_pow, vec![Value::Int(2), Value::None]).is_err());
        assert!(call(calc_pow, vec![Value::None, Value::None]).unwrap_err()[0].message.contains("base"));
        assert!(call(calc_pow, vec![Value::Int(2)]).is_err());
    }

    #[test]
    fn v16_power_root_and_sqrt_controls() {
        assert_eq!(call(calc_root, vec![Value::Int(-8), Value::Int(3)]).unwrap(), Value::Float(-2.0));
        assert_eq!(call(calc_sqrt, vec![Value::Int(4)]).unwrap(), Value::Float(2.0));
    }
}
''')
(out/'tests-before-code.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),source=str(p),sha256=hashlib.sha256(p.read_bytes()).hexdigest(),implementation_unchanged=p.read_text().startswith(b['files'][str(p)])),indent=2)+'\n')
