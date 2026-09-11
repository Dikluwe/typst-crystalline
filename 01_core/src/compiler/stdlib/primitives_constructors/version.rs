//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/primitives-constructors/version.md
//! @prompt-hash 7d710322
//! @layer L1
//! @updated 2026-08-23

use super::super::err;
use crate::compiler::eval::EvalContext;
use crate::entities::args::{ArgOccurrence, Args};
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;
use crate::entities::version::Version;
use std::sync::Arc;

fn version_required(args: &mut Args, name: &str) -> SourceResult<ArgOccurrence> {
    let occurrences = args.occurrence_sequence();
    if let Some(arg) = occurrences.iter().find(|arg| arg.name.is_none()) {
        let arg = arg.clone();
        args.remove_positional(0);
        return Ok(arg);
    }
    if let Some(arg) = occurrences.iter().find(|arg| arg.name.as_deref() == Some(name)) {
        return Err(vec![SourceDiagnostic::error(
            version_anchor(arg.span, args.span),
            format!("the argument `{name}` is positional"),
        )
        .with_hint(format!("try removing `{name}:`"))]);
    }
    Err(vec![SourceDiagnostic::error(args.span, format!("missing argument: {name}"))])
}

fn version_anchor(span: Span, fallback: Span) -> Span {
    if span.is_detached() {
        fallback
    } else {
        span
    }
}

fn native_version_at(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use crate::compiler::eval::operators::error_formatting::vanilla_type_name;
    let mut args = args.clone();
    let receiver = version_required(&mut args, "self")?;
    let Value::Version(version) = receiver.value else {
        return Err(vec![SourceDiagnostic::error(
            version_anchor(receiver.value_span, args.span),
            format!("expected version, found {}", vanilla_type_name(&receiver.value)),
        )]);
    };
    let arg = version_required(&mut args, "index")?;
    let Value::Int(index) = arg.value else {
        return Err(vec![SourceDiagnostic::error(
            version_anchor(arg.value_span, args.span),
            format!("expected integer, found {}", vanilla_type_name(&arg.value)),
        )]);
    };
    if let Some(arg) = args.occurrence_sequence().first() {
        let message = match &arg.name {
            Some(name) => format!("unexpected argument: {name}"),
            None => "unexpected argument".into(),
        };
        return Err(vec![SourceDiagnostic::error(
            version_anchor(arg.span, args.span),
            message,
        )]);
    }
    version
        .at(index)
        .map(Value::Int)
        .map_err(|message| vec![SourceDiagnostic::error(args.span, message)])
}

pub(crate) fn version_type_field(field: &str) -> Option<Value> {
    (field == "at").then(|| {
        Value::Func(crate::entities::func::Func::native("at", native_version_at))
    })
}

pub(crate) fn dispatch_version_method(
    receiver: &Version,
    method: &str,
    args: Args,
    ctx: &mut EvalContext,
    world: &dyn crate::contracts::world::World,
    current_file: FileId,
) -> SourceResult<Value> {
    let mut occurrences = args.occurrence_sequence();
    occurrences.insert(
        0,
        ArgOccurrence {
            name: None,
            value: Value::Version(Arc::new(receiver.clone())),
            span: Span::detached(),
            value_span: Span::detached(),
        },
    );
    let args = Args::from_occurrences(args.span, occurrences);
    match method {
        "at" => native_version_at(ctx, &args, world, current_file),
        _ => unreachable!("dispatch_version_method called for unknown method"),
    }
}

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
    fn p1339_version_static_at_and_closed_bound_arguments() {
        use crate::compiler::eval::eval_expression;
        let world = null_world();
        for (expression, expected) in [
            ("version.at(version(1,2,0),-1)", Value::Int(0)),
            ("version.at(version(1,2,3),-3)", Value::Int(1)),
            ("version.at(version(),9223372036854775807)", Value::Int(0)),
        ] {
            let (result, _) = eval_expression(&world, expression);
            assert_eq!(result.unwrap(), expected, "{expression}");
        }
        for (expression, message) in [
            ("version.at(version())", "missing argument: index"),
            (
                "version.at(version(),-1)",
                "component index out of bounds (index: -1, len: 0)",
            ),
            ("version.at(version(1),index:0)", "the argument `index` is positional"),
            ("version(1).at(0,other:1)", "unexpected argument: other"),
            ("version.at(version(1),0.0)", "expected integer, found float"),
        ] {
            let (result, _) = eval_expression(&world, expression);
            assert_eq!(result.unwrap_err()[0].message, message, "{expression}");
        }
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
