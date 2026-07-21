//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/stdlib/primitives-constructors.md
//! @prompt-hash 00abed18
//! @layer L1
//! @updated 2026-06-22
//!
//! Constructors stdlib para tipos primitivos L1: `decimal`, `duration`, `version`.
//! Passo 403: zero tipo novo; zero variant novo; zero I/O.
//! Passo 405: `duration()` estendido para named args vanilla (mantém string P403).

use std::sync::Arc;

use crate::engine::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::decimal::Decimal;
use crate::entities::duration::Duration;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;
use crate::entities::version::Version;

use super::{err, expect_no_named};

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
///
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
    fn extract_nonneg(args: &Args, name: &str) -> Result<u64, String> {
        match args.named.get(name) {
            Some(Value::Int(v)) if *v < 0 => {
                Err(format!("duration(): '{}' não pode ser negativo", name))
            }
            Some(Value::Int(v)) => Ok(*v as u64),
            Some(other) => Err(format!(
                "duration(): '{}' espera Int, recebeu {}",
                name,
                other.type_name()
            )),
            None => Ok(0),
        }
    }

    let days = extract_nonneg(args, "days")
        .map_err(|msg| vec![SourceDiagnostic::error(Span::detached(), msg)])?;
    let hours = extract_nonneg(args, "hours")
        .map_err(|msg| vec![SourceDiagnostic::error(Span::detached(), msg)])?;
    let minutes = extract_nonneg(args, "minutes")
        .map_err(|msg| vec![SourceDiagnostic::error(Span::detached(), msg)])?;
    let seconds = extract_nonneg(args, "seconds")
        .map_err(|msg| vec![SourceDiagnostic::error(Span::detached(), msg)])?;
    let milliseconds = extract_nonneg(args, "milliseconds")
        .map_err(|msg| vec![SourceDiagnostic::error(Span::detached(), msg)])?;
    let microseconds = extract_nonneg(args, "microseconds")
        .map_err(|msg| vec![SourceDiagnostic::error(Span::detached(), msg)])?;
    let nanoseconds = extract_nonneg(args, "nanoseconds")
        .map_err(|msg| vec![SourceDiagnostic::error(Span::detached(), msg)])?;

    const SECOND_NANOS: u128 = 1_000_000_000;
    const MINUTE_NANOS: u128 = 60 * SECOND_NANOS;
    const HOUR_NANOS: u128 = 60 * MINUTE_NANOS;
    const DAY_NANOS: u128 = 24 * HOUR_NANOS;

    let total = days as u128 * DAY_NANOS
        + hours as u128 * HOUR_NANOS
        + minutes as u128 * MINUTE_NANOS
        + seconds as u128 * SECOND_NANOS
        + milliseconds as u128 * 1_000_000
        + microseconds as u128 * 1_000
        + nanoseconds as u128;

    if total > u64::MAX as u128 {
        return err("duration(): excede o máximo suportado".to_string());
    }

    Ok(Value::Duration(Duration::from_nanos(total as u64)))
}

/// `version(...)` → `Value::Version`.
///
/// Formas suportadas (paridade vanilla 0.15.0):
/// 1. String posicional (compatibilidade P403): `version("1.2.3")` — só inteiros.
/// 2. Componentes posicionais, qualquer número: `version(1, 2, 3)`,
///    `version(1, 2, 3, 4, 5)`.
/// 3. Array de componentes (P682): `version((1, 2, 3))`, `version((1, 2, 3, 4, 5))`.
///
/// Sem `pre`/`build` (não existem no Typst — ver P684); argumentos nomeados são erro.
pub fn native_version(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    fn as_nonneg_int(v: &Value, name: &str) -> Result<u64, String> {
        match v {
            Value::Int(i) if *i < 0 => {
                Err(format!("version(): '{}' não pode ser negativo", name))
            }
            Value::Int(i) => Ok(*i as u64),
            other => Err(format!(
                "version(): '{}' espera Int, recebeu {}",
                name,
                other.type_name()
            )),
        }
    }

    // Forma string (P403): único arg posicional Str (sem named args).
    if args.items.len() == 1
        && matches!(args.items[0], Value::Str(_))
        && args.named.is_empty()
    {
        return match args.items.as_slice() {
            [Value::Str(s)] => match Version::from_str(s) {
                Some(v) => Ok(Value::Version(Arc::new(v))),
                None => err(format!("version(): string inválida: '{}'", s)),
            },
            _ => unreachable!(),
        };
    }

    // Mistura inválida: string posicional com named args.
    if args.items.len() == 1
        && matches!(args.items[0], Value::Str(_))
        && !args.named.is_empty()
    {
        return err(
            "version(): não pode misturar string posicional com argumentos nomeados"
                .to_string(),
        );
    }

    // Nas formas de componentes (posicional ou array) o vanilla não aceita
    // argumentos nomeados (`pre`/`build` não existem — P684).
    if !args.named.is_empty() {
        return err("version(): não aceita argumentos nomeados".to_string());
    }

    // Forma array (P682): único posicional `Array` → componentes; caso contrário
    // os próprios posicionais são os componentes. Qualquer número (≥ 0) é válido.
    let components_values: Vec<&Value> = if args.items.len() == 1 {
        match &args.items[0] {
            Value::Array(a) => a.iter().collect(),
            _ => args.items.iter().collect(),
        }
    } else {
        args.items.iter().collect()
    };

    let mut components = Vec::with_capacity(components_values.len());
    for (i, v) in components_values.iter().enumerate() {
        let name = match i {
            0 => "major",
            1 => "minor",
            2 => "patch",
            _ => "componente",
        };
        let n = as_nonneg_int(v, name)
            .map_err(|msg| vec![SourceDiagnostic::error(Span::detached(), msg)])?;
        components.push(n);
    }

    Ok(Value::Version(Arc::new(Version::from_components(components))))
}

/// Parser canónico de duração.
///
/// Formatos aceites: `"0s"`, `"1h30m"`, `"3d2h30m15.5s"`, `"0.001s"`.
/// Regras:
/// - Sufixos `d`, `h`, `m`, `s` em qualquer combinação não vazia.
/// - Ordem fixa d → h → m → s.
/// - Cada sufixo no máximo uma vez.
/// - `d`, `h`, `m` aceitam apenas inteiros decimais não negativos (`u64`).
/// - `s` aceita `f64` não negativo (com ou sem fração).
/// - Componentes ausentes contribuem 0; string vazia ou mal formada → `None`.
fn parse_duration(s: &str) -> Option<Duration> {
    if s.is_empty() {
        return None;
    }

    let mut rest = s;
    let mut nanos: u64 = 0;
    let mut seen = [false; 4]; // d, h, m, s

    // Se a string for apenas "0s" (ou equivalente), permitimos zero explícito.
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

            let value = if suffix == 's' {
                let seconds = num_str.parse::<f64>().ok()?;
                if seconds.is_sign_negative() || !seconds.is_finite() {
                    return None;
                }
                let whole = seconds.trunc() as u64;
                let frac = seconds.fract();
                let frac_nanos = (frac * 1_000_000_000.0).round() as u64;
                whole.checked_mul(1_000_000_000)?.checked_add(frac_nanos)?
            } else {
                let whole = num_str.parse::<u64>().ok()?;
                let multiplier = match suffix {
                    'd' => 86_400u64 * 1_000_000_000,
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

    Some(Duration::from_nanos(nanos))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::world::World;
    use crate::engine::eval::EvalContext;
    use crate::entities::args::Args;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::source::Source;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library,
    };
    use std::collections::HashMap;
    use std::num::NonZeroU16;
    use std::sync::Arc;

    fn p(items: Vec<Value>) -> Args {
        Args::positional(items)
    }

    fn pn(items: Vec<Value>, name: &str, val: Value) -> Args {
        let mut a = Args::positional(items);
        a.named.insert(name.into(), val);
        a
    }

    fn ctx() -> EvalContext {
        EvalContext::new()
    }

    fn test_file_id() -> FileId {
        FileId::from_raw(NonZeroU16::new(1).unwrap())
    }

    #[derive(Default)]
    struct NullWorld {
        library: Library,
        book: FontBook,
        files: HashMap<String, Arc<Vec<u8>>>,
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
        fn today(&self, _: Option<i64>) -> Option<Datetime> {
            None
        }
        fn read_bytes(
            &self,
            _current_file: FileId,
            path: &str,
        ) -> Result<Arc<Vec<u8>>, String> {
            self.files
                .get(path)
                .cloned()
                .ok_or_else(|| format!("ficheiro não encontrado: {}", path))
        }
    }

    fn null_world() -> NullWorld {
        NullWorld::default()
    }

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
        assert!(native_duration(
            &mut ctx(),
            &pn(vec![], "seconds", Value::Int(-1)),
            &null_world(),
            test_file_id()
        )
        .is_err());
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
    fn duration_named_overflow() {
        let mut args = Args::positional(vec![]);
        args.named.insert("seconds".into(), Value::Int(i64::MAX));
        assert!(
            native_duration(&mut ctx(), &args, &null_world(), test_file_id()).is_err()
        );
    }

    #[test]
    fn duration_named_and_positional_mixed() {
        let mut args = Args::positional(vec![Value::Str("1h".into())]);
        args.named.insert("seconds".into(), Value::Int(1));
        assert!(
            native_duration(&mut ctx(), &args, &null_world(), test_file_id()).is_err()
        );
    }

    #[test]
    fn version_valid() {
        let v = native_version(
            &mut ctx(),
            &p(vec![Value::Str("1.2.3".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(v, Value::Version(Arc::new(Version::new(1, 2, 3))));
    }

    #[test]
    fn version_string_arbitrary_components() {
        let v = native_version(
            &mut ctx(),
            &p(vec![Value::Str("1.2.3.4.5".into())]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(
            v,
            Value::Version(Arc::new(Version::from_components(vec![1, 2, 3, 4, 5])))
        );
    }

    #[test]
    fn version_string_rejects_pre_build() {
        // P684 — `pre`/`build` textuais não existem no Typst: string com '-'/'+' é inválida.
        assert!(native_version(
            &mut ctx(),
            &p(vec![Value::Str("1.2.3-alpha.1".into())]),
            &null_world(),
            test_file_id()
        )
        .is_err());
        assert!(native_version(
            &mut ctx(),
            &p(vec![Value::Str("1.2.3+build.2".into())]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    #[test]
    fn version_invalid() {
        assert!(native_version(
            &mut ctx(),
            &p(vec![Value::Str("invalid".into())]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    // ── P406/P684 — constructor version: componentes arbitrários, sem pre/build ─

    #[test]
    fn version_vanilla_basic() {
        let v = native_version(
            &mut ctx(),
            &p(vec![Value::Int(1), Value::Int(2), Value::Int(3)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(v, Value::Version(Arc::new(Version::new(1, 2, 3))));
    }

    #[test]
    fn version_vanilla_zero() {
        let v = native_version(
            &mut ctx(),
            &p(vec![Value::Int(0), Value::Int(0), Value::Int(0)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(v, Value::Version(Arc::new(Version::new(0, 0, 0))));
    }

    #[test]
    fn version_single_component() {
        // P684 — qualquer número (≥ 0) de componentes é válido.
        let v = native_version(
            &mut ctx(),
            &p(vec![Value::Int(1)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(v, Value::Version(Arc::new(Version::from_components(vec![1]))));
    }

    #[test]
    fn version_two_components() {
        let v = native_version(
            &mut ctx(),
            &p(vec![Value::Int(1), Value::Int(2)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(v, Value::Version(Arc::new(Version::from_components(vec![1, 2]))));
    }

    #[test]
    fn version_arbitrary_components() {
        let v = native_version(
            &mut ctx(),
            &p(vec![
                Value::Int(1),
                Value::Int(2),
                Value::Int(3),
                Value::Int(4),
                Value::Int(5),
            ]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(
            v,
            Value::Version(Arc::new(Version::from_components(vec![1, 2, 3, 4, 5])))
        );
    }

    #[test]
    fn version_empty() {
        let v = native_version(&mut ctx(), &p(vec![]), &null_world(), test_file_id())
            .unwrap();
        assert_eq!(v, Value::Version(Arc::new(Version::default())));
    }

    #[test]
    fn version_eq_zero_pad() {
        let a = native_version(
            &mut ctx(),
            &p(vec![Value::Int(1), Value::Int(2), Value::Int(3)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        let b = native_version(
            &mut ctx(),
            &p(vec![Value::Int(1), Value::Int(2), Value::Int(3), Value::Int(0)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn version_named_rejected() {
        // P684 — `version(1, 2, 3, pre: "alpha")` é erro (argumento desconhecido).
        let mut args =
            Args::positional(vec![Value::Int(1), Value::Int(2), Value::Int(3)]);
        args.named.insert("pre".into(), Value::Str("alpha".into()));
        assert!(native_version(&mut ctx(), &args, &null_world(), test_file_id()).is_err());
    }

    #[test]
    fn version_fourth_positional_string_rejected() {
        // P684 — `version(1, 2, 3, "alpha.1")` é erro (4º posicional tem de ser Int).
        assert!(native_version(
            &mut ctx(),
            &p(vec![
                Value::Int(1),
                Value::Int(2),
                Value::Int(3),
                Value::Str("alpha.1".into())
            ]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    #[test]
    fn version_vanilla_negative_major() {
        assert!(native_version(
            &mut ctx(),
            &p(vec![Value::Int(-1), Value::Int(2), Value::Int(3)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    #[test]
    fn version_vanilla_negative_minor() {
        assert!(native_version(
            &mut ctx(),
            &p(vec![Value::Int(1), Value::Int(-2), Value::Int(3)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    #[test]
    fn version_vanilla_negative_patch() {
        assert!(native_version(
            &mut ctx(),
            &p(vec![Value::Int(1), Value::Int(2), Value::Int(-3)]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    // ── P682/P684 — constructor version forma array (componentes arbitrários) ─

    #[test]
    fn version_array_basic() {
        let v = native_version(
            &mut ctx(),
            &p(vec![Value::Array(vec![Value::Int(0), Value::Int(2), Value::Int(2)])]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(v, Value::Version(Arc::new(Version::new(0, 2, 2))));
    }

    #[test]
    fn version_array_equivale_posicional() {
        let pos = native_version(
            &mut ctx(),
            &p(vec![Value::Int(1), Value::Int(2), Value::Int(3)]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        let arr = native_version(
            &mut ctx(),
            &p(vec![Value::Array(vec![Value::Int(1), Value::Int(2), Value::Int(3)])]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(pos, arr);
    }

    #[test]
    fn version_array_arbitrary() {
        let v = native_version(
            &mut ctx(),
            &p(vec![Value::Array(vec![
                Value::Int(1),
                Value::Int(2),
                Value::Int(3),
                Value::Int(4),
                Value::Int(5),
            ])]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert_eq!(
            v,
            Value::Version(Arc::new(Version::from_components(vec![1, 2, 3, 4, 5])))
        );
    }

    #[test]
    fn version_array_non_int() {
        assert!(native_version(
            &mut ctx(),
            &p(vec![Value::Array(vec![
                Value::Int(1),
                Value::Str("x".into()),
                Value::Int(3)
            ])]),
            &null_world(),
            test_file_id()
        )
        .is_err());
    }

    #[test]
    fn version_array_named_rejected() {
        let mut args = Args::positional(vec![Value::Array(vec![
            Value::Int(1),
            Value::Int(2),
            Value::Int(3),
        ])]);
        args.named.insert("pre".into(), Value::Str("alpha".into()));
        assert!(native_version(&mut ctx(), &args, &null_world(), test_file_id()).is_err());
    }

    #[test]
    fn version_string_and_named_mixed() {
        let mut args = Args::positional(vec![Value::Str("1.2.3".into())]);
        args.named.insert("pre".into(), Value::Str("alpha".into()));
        assert!(native_version(&mut ctx(), &args, &null_world(), test_file_id()).is_err());
    }

    #[test]
    fn named_arg_rejected() {
        let mut args = Args::positional(vec![Value::Str("1.5".into())]);
        args.named.insert("extra".into(), Value::Bool(true));
        assert!(native_decimal(&mut ctx(), &args, &null_world(), test_file_id()).is_err());
        assert!(
            native_duration(&mut ctx(), &args, &null_world(), test_file_id()).is_err()
        );
        assert!(native_version(&mut ctx(), &args, &null_world(), test_file_id()).is_err());
    }
}
