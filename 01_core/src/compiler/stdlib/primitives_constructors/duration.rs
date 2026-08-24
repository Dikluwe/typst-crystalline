//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/primitives-constructors.md
//! @prompt-hash 5b6064d7
//! @layer L1
//! @updated 2026-08-23

use super::super::err;
use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::duration::Duration;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

/// Mistura das duas formas → erro. Todos os named args são `Int` ≥ 0.
pub fn native_duration(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let has_named = !args.named.is_empty();
    let has_positional = !args.items.is_empty();

    if has_named && has_positional {
        return err(
            "duration(): não pode misturar argumentos posicionais e nomeados".to_string()
        );
    }

    if has_positional {
        // Forma legada P403: string canónica.
        return match args.items.as_slice() {
            [Value::Str(s)] => match parse_duration(s) {
                Some(d) => Ok(Value::Duration(d)),
                None => err(format!("duration(): string inválida: '{}'", s)),
            },
            [other] => {
                err(format!("duration(): espera Str, recebeu {}", other.type_name()))
            }
            _ => err(format!(
                "duration(): requer 1 argumento, recebeu {}",
                args.items.len()
            )),
        };
    }

    // Forma vanilla: named args.
    // P843 (F1) — `weeks` adicionado: o vanilla aceita-o
    // (`duration(weeks: 1, days: 1)`, medido em `temp/p843/f1_duration.typ`)
    // e o repr nomeado inclui a componente `weeks:`; antes era silenciosamente
    // ignorado (valor errado sem erro).
    // P850 — componentes negativos suportados.
    fn extract_signed(args: &Args, name: &str) -> Result<i64, String> {
        match args.named.get(name) {
            Some(Value::Int(v)) => Ok(*v),
            Some(other) => Err(format!(
                "duration(): '{}' espera Int, recebeu {}",
                name,
                other.type_name()
            )),
            None => Ok(0),
        }
    }

    let weeks = extract_signed(args, "weeks")
        .map_err(|msg| vec![SourceDiagnostic::error(Span::detached(), msg)])?;
    let days = extract_signed(args, "days")
        .map_err(|msg| vec![SourceDiagnostic::error(Span::detached(), msg)])?;
    let hours = extract_signed(args, "hours")
        .map_err(|msg| vec![SourceDiagnostic::error(Span::detached(), msg)])?;
    let minutes = extract_signed(args, "minutes")
        .map_err(|msg| vec![SourceDiagnostic::error(Span::detached(), msg)])?;
    let seconds = extract_signed(args, "seconds")
        .map_err(|msg| vec![SourceDiagnostic::error(Span::detached(), msg)])?;
    let milliseconds = extract_signed(args, "milliseconds")
        .map_err(|msg| vec![SourceDiagnostic::error(Span::detached(), msg)])?;
    let microseconds = extract_signed(args, "microseconds")
        .map_err(|msg| vec![SourceDiagnostic::error(Span::detached(), msg)])?;
    let nanoseconds = extract_signed(args, "nanoseconds")
        .map_err(|msg| vec![SourceDiagnostic::error(Span::detached(), msg)])?;

    const SECOND_NANOS: i128 = 1_000_000_000;
    const MINUTE_NANOS: i128 = 60 * SECOND_NANOS;
    const HOUR_NANOS: i128 = 60 * MINUTE_NANOS;
    const DAY_NANOS: i128 = 24 * HOUR_NANOS;
    const WEEK_NANOS: i128 = 7 * DAY_NANOS;
    const MILLI_NANOS: i128 = 1_000_000;
    const MICRO_NANOS: i128 = 1_000;

    let total = weeks as i128 * WEEK_NANOS
        + days as i128 * DAY_NANOS
        + hours as i128 * HOUR_NANOS
        + minutes as i128 * MINUTE_NANOS
        + seconds as i128 * SECOND_NANOS
        + milliseconds as i128 * MILLI_NANOS
        + microseconds as i128 * MICRO_NANOS
        + nanoseconds as i128;

    if total > i128::MAX || total < i128::MIN {
        return err("duration(): excede o máximo suportado".to_string());
    }

    Ok(Value::Duration(Duration::from_nanos(total)))
}

/// `version(...)` → `Value::Version`.
///
/// Formas suportadas (paridade vanilla 0.15.0):
/// 1. String posicional (compatibilidade P403): `version("1.2.3")` — só inteiros.
/// 2. Componentes posicionais, qualquer número: `version(1, 2, 3)`,
///    `version(1, 2, 3, 4, 5)`.
/// 3. Array de componentes (P682): `version((1, 2, 3))`, `version((1, 2, 3, 4, 5))`.

/// - `s` aceita `f64` não negativo (com ou sem fração).
/// - Componentes ausentes contribuem 0; string vazia ou mal formada → `None`.
fn parse_duration(s: &str) -> Option<Duration> {
    if s.is_empty() {
        return None;
    }

    let mut rest = s;
    let mut nanos: i128 = 0;
    let mut seen = [false; 4]; // d, h, m, s

    // P850 — sinal inicial opcional (aplica-se a toda a duração).
    let negative = if rest.starts_with('-') {
        rest = &rest[1..];
        true
    } else if rest.starts_with('+') {
        rest = &rest[1..];
        false
    } else {
        false
    };

    if rest.is_empty() {
        return None;
    }

    // Processamos sufixos na ordem canónica.
    let suffixes = [('d', 0), ('h', 1), ('m', 2), ('s', 3)];

    for (suffix, idx) in suffixes {
        if let Some(pos) = rest.find(suffix) {
            if seen[idx] {
                return None;
            }
            // Verificar que nenhum sufixo posterior aparece antes deste.
            for &(later, _later_idx) in &suffixes[(idx + 1)..] {
                if let Some(later_pos) = rest.find(later) {
                    if later_pos < pos {
                        return None;
                    }
                }
            }

            let num_str = &rest[..pos];
            if num_str.is_empty() {
                return None;
            }

            let value: i128 = if suffix == 's' {
                let seconds = num_str.parse::<f64>().ok()?;
                if !seconds.is_finite() {
                    return None;
                }
                let whole = seconds.trunc() as i128;
                let frac = seconds.fract();
                let frac_nanos = (frac * 1_000_000_000.0).round() as i128;
                whole.checked_mul(1_000_000_000)?.checked_add(frac_nanos)?
            } else {
                let whole = num_str.parse::<i64>().ok()? as i128;
                let multiplier = match suffix {
                    'd' => 86_400i128 * 1_000_000_000,
                    'h' => 3_600 * 1_000_000_000,
                    'm' => 60 * 1_000_000_000,
                    _ => unreachable!(),
                };
                whole.checked_mul(multiplier)?
            };

            nanos = nanos.checked_add(value)?;
            seen[idx] = true;
            rest = &rest[pos + 1..];
        }
    }

    // Se sobrou texto não consumido, a string é inválida.
    if !rest.is_empty() {
        return None;
    }

    // Pelo menos um componente deve ter sido parseado.
    if !seen.iter().any(|&x| x) {
        return None;
    }

    if negative {
        nanos = -nanos;
    }

    Some(Duration::from_nanos(nanos))
}

#[cfg(test)]
mod tests {
    use super::super::test_support::*;
    use super::*;
    #[test]
    fn duration_zero() {
        let v = native_duration(
            &mut ctx(),
            &p(vec![Value::Str("0s".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(v, Value::Duration(Duration::ZERO));
    }

    #[test]
    fn duration_hours_minutes() {
        let v = native_duration(
            &mut ctx(),
            &p(vec![Value::Str("1h30m".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(v, Value::Duration(Duration::from_seconds(5400)));
    }

    #[test]
    fn duration_two_hours_thirty_minutes() {
        let v = native_duration(
            &mut ctx(),
            &p(vec![Value::Str("2h30m".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(v, Value::Duration(Duration::from_seconds(9000)));
    }

    #[test]
    fn duration_full_with_fractional_seconds() {
        let v = native_duration(
            &mut ctx(),
            &p(vec![Value::Str("3d2h30m15.5s".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        let expected = Duration::from_days(3).nanos
            + Duration::from_hours(2).nanos
            + Duration::from_minutes(30).nanos
            + 15 * 1_000_000_000
            + 500_000_000;
        assert_eq!(v, Value::Duration(Duration::from_nanos(expected)));
    }

    #[test]
    fn duration_subsecond_only() {
        let v = native_duration(
            &mut ctx(),
            &p(vec![Value::Str("0.001s".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(v, Value::Duration(Duration::from_nanos(1_000_000)));
    }

    #[test]
    fn duration_invalid_string() {
        assert!(native_duration(
            &mut ctx(),
            &p(vec![Value::Str("abc".into())]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    #[test]
    fn duration_invalid_suffix() {
        assert!(native_duration(
            &mut ctx(),
            &p(vec![Value::Str("1x".into())]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    #[test]
    fn duration_wrong_order() {
        assert!(native_duration(
            &mut ctx(),
            &p(vec![Value::Str("30m1h".into())]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    #[test]
    fn duration_wrong_type() {
        assert!(native_duration(
            &mut ctx(),
            &p(vec![Value::Int(1)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    #[test]
    fn duration_empty() {
        assert!(native_duration(
            &mut ctx(),
            &p(vec![Value::Str("".into())]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    // ── P405 — constructor duration com named args ───────────────────────────

    #[test]
    fn duration_named_zero() {
        let v = native_duration(&mut ctx(), &p(vec![]), &null_world(), test_file_id())
            .unwrap();
        assert_eq!(v, Value::Duration(Duration::ZERO));
    }

    #[test]
    fn duration_named_seconds() {
        let v = native_duration(
            &mut ctx(),
            &pn(vec![], "seconds", Value::Int(90)),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(v, Value::Duration(Duration::from_seconds(90)));
    }

    #[test]
    fn duration_named_mixed() {
        let args = {
            let mut a = Args::positional(vec![]);
            a.named.insert("days".into(), Value::Int(1));
            a.named.insert("hours".into(), Value::Int(2));
            a.named.insert("minutes".into(), Value::Int(3));
            a
        };
        let v =
            native_duration(&mut ctx(), &args, &null_world(), test_file_id()).unwrap();
        let expected = Duration::from_days(1).nanos
            + Duration::from_hours(2).nanos
            + Duration::from_minutes(3).nanos;
        assert_eq!(v, Value::Duration(Duration::from_nanos(expected)));
    }

    #[test]
    fn duration_named_nanos() {
        let v = native_duration(
            &mut ctx(),
            &pn(vec![], "nanoseconds", Value::Int(500)),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(v, Value::Duration(Duration::from_nanos(500)));
    }

    #[test]
    fn duration_named_negative() {
        let v = native_duration(
            &mut ctx(),
            &pn(vec![], "seconds", Value::Int(-1)),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(v, Value::Duration(-Duration::from_seconds(1)));
    }

    #[test]
    fn duration_named_mixed_negative() {
        let args = {
            let mut a = Args::positional(vec![]);
            a.named.insert("days".into(), Value::Int(1));
            a.named.insert("hours".into(), Value::Int(-2));
            a.named.insert("minutes".into(), Value::Int(3));
            a
        };
        let v =
            native_duration(&mut ctx(), &args, &null_world(), test_file_id()).unwrap();
        let expected = Duration::from_days(1).nanos - Duration::from_hours(2).nanos
            + Duration::from_minutes(3).nanos;
        assert_eq!(v, Value::Duration(Duration::from_nanos(expected)));
    }

    #[test]
    fn duration_string_negative() {
        let v = native_duration(
            &mut ctx(),
            &p(vec![Value::Str("-1h30m".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        let expected =
            -(Duration::from_hours(1).nanos + Duration::from_minutes(30).nanos);
        assert_eq!(v, Value::Duration(Duration::from_nanos(expected)));
    }

    #[test]
    fn duration_named_wrong_type() {
        assert!(native_duration(
            &mut ctx(),
            &pn(vec![], "seconds", Value::Float(1.5)),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    #[test]
    fn duration_named_i64_max_ok() {
        // P850 (i128): i64::MAX segundos ainda cabe na representação interna.
        let mut args = Args::positional(vec![]);
        args.named.insert("seconds".into(), Value::Int(i64::MAX));
        assert!(native_duration(&mut ctx(), &args, &null_world(), test_file_id()).is_ok());
    }

    #[test]
    fn duration_named_and_positional_mixed() {
        let mut args = Args::positional(vec![Value::Str("1h".into())]);
        args.named.insert("seconds".into(), Value::Int(1));
        assert!(
            native_duration(&mut ctx(), &args, &null_world(), test_file_id()).is_err()
        );
    }
}
