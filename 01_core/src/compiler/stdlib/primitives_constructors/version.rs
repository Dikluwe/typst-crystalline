//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/primitives-constructors/version.md
//! @prompt-hash a87eeef2
//! @layer L1
//! @updated 2026-08-23

use super::super::err;
use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;
use crate::entities::version::Version;
use std::sync::Arc;

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

#[cfg(test)]
mod tests {
    use super::super::test_support::*;
    use super::super::{native_decimal, native_duration};
    use super::*;
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
