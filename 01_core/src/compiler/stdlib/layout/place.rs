//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/layout/place.md
//! @prompt-hash 1d1e518a
//! @layer L1
//! @updated 2026-09-30
//!
//! Comportamento de `stdlib/layout::place`.

use super::*;

/// `place(alignment, dx?, dy?, scope?, body)` → `Content::Place`.
///
/// `dx`/`dy` em pt deslocam o conteúdo a partir da posição alinhada.
/// `scope` (Passo 84.6, encerra DEBT-37): `"column"` (default — ancora à
/// célula activa de Grid, ou à página fora de Grid) ou `"parent"` (ancora
/// sempre à página). Aceita string ou omissão.
pub fn native_place(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    for key in args.named.keys() {
        // P223 — accept "float" e "clearance" novos named args.
        if !["dx", "dy", "scope", "float", "clearance"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("argumento nomeado inesperado em place(): '{}'", key),
            )]);
        }
    }

    fn extract_pt(val: &Value, span: Span) -> SourceResult<f64> {
        use crate::compiler::eval::operators::error_formatting::vanilla_type_name;
        match val {
            Value::Float(f) => Ok(*f),
            Value::Int(i) => Ok(*i as f64),
            Value::Length(l) => Ok(l.abs.to_pt()),
            // RM1: existing unresolved relative projections remain explicit;
            // malformed non-dimensional arguments must not use this path.
            Value::Ratio(_) | Value::Relative(_) => Ok(0.0),
            Value::None
            | Value::Bool(_)
            | Value::Str(_)
            | Value::Array(_)
            | Value::Dict(_)
            | Value::Module(_)
            | Value::Datetime(_)
            | Value::Func(_)
            | Value::Content(_)
            | Value::LocatedContent(_, _)
            | Value::Auto
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
            | Value::Decimal(_)
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
            | Value::SpotColorant(_) => Err(vec![SourceDiagnostic::error(
                span,
                format!("expected relative length, found {}", vanilla_type_name(val)),
            )]),
        }
    }
    let value_span = |name: &str| {
        args.occurrences
            .as_ref()
            .and_then(|occurrences| {
                occurrences.iter().rev().find(|arg| arg.name.as_deref() == Some(name))
            })
            .map_or(args.span, |arg| arg.value_span.or(arg.span).or(args.span))
    };

    // Preserve an omitted vertical axis: it follows flow, unlike explicit Top.
    let alignment = extract_alignment(
        args,
        Align2D {
            h: Some(crate::entities::layout_types::HAlign::Left),
            v: None,
        },
    );

    let dx = args
        .named
        .get("dx")
        .map(|value| extract_pt(value, value_span("dx")))
        .transpose()?
        .unwrap_or(0.0);
    let dy = args
        .named
        .get("dy")
        .map(|value| extract_pt(value, value_span("dy")))
        .transpose()?
        .unwrap_or(0.0);

    let scope = match args.named.get("scope") {
        Some(Value::Str(s)) => match s.as_str() {
            "column" => crate::entities::layout_types::PlaceScope::Column,
            "parent" => crate::entities::layout_types::PlaceScope::Parent,
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!(
                    "place(): scope deve ser \"column\" ou \"parent\", recebeu \"{}\"",
                    other
                ),
                )])
            }
        },
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("place(): scope deve ser string, recebeu {}", other.type_name()),
            )])
        }
        None => crate::entities::layout_types::PlaceScope::default(),
    };

    // P223 — extract `float` (default false). Semantic real adiada per
    // ADR-0054 graded; precedente N=4 cumulativo weak/breakable/float.
    let float = match args.named.get("float") {
        Some(Value::Bool(b)) => *b,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("place(float): espera Bool, recebeu {}", other.type_name()),
            )])
        }
        None => false,
    };

    // P1292-D v10 — o default permanece relativo até ao layout, onde é
    // resolvido contra o estilo efectivo da ocorrência. Valores explícitos,
    // inclusive zero, continuam soberanos.
    let clearance = match args.named.get("clearance") {
        Some(val) => {
            let len = extract_length(val).ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    args.span,
                    format!(
                        "place(clearance): espera length, recebeu {}",
                        val.type_name()
                    ),
                )]
            })?;
            if len.abs.0 < 0.0 || len.em < 0.0 {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    "place(clearance): valor negativo não suportado".to_string(),
                )]);
            }
            Some(len)
        }
        None => Some(Length::em(1.5)),
    };

    // P223 — DEBT-37 §"Divergência" restaurada (Decisão 3 Opção α):
    // vanilla `place` com `scope: "parent"` exige `float: true`.
    if matches!(scope, crate::entities::layout_types::PlaceScope::Parent) && !float {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "place: scope \"parent\" requer float: true (paridade vanilla; sem float, scope \"parent\" não tem semantic flutuante; fecha divergência DEBT-37 documentada)".to_string(),
        )]);
    }

    let body = args
        .items
        .iter()
        .find_map(|v| if let Value::Content(c) = v { Some(c.clone()) } else { None })
        .ok_or_else(|| {
            vec![SourceDiagnostic::error(
                args.span,
                "place() exige um bloco de conteúdo".to_string(),
            )]
        })?;

    Ok(Value::Content(Content::place(alignment, dx, dy, scope, float, clearance, body)))
}

/// `place.flush()` → sentinela que realiza os floats já pendentes.
pub fn native_flush(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    validate_flush_args(args)?;
    Ok(Value::Content(Content::flush()))
}

pub(crate) fn validate_flush_args(args: &Args) -> SourceResult<()> {
    if !args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]);
    }

    if let Some(name) = args.named.keys().next() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }

    Ok(())
}

#[cfg(test)]
mod batch_f_tests {
    use super::*;

    use crate::contracts::world::World;
    use crate::entities::{
        font_book::FontBook,
        source::Source,
        world_types::{Bytes, Datetime, FileError, FileResult, Font, Library},
    };
    #[derive(Default)]
    struct TestWorld {
        library: Library,
        book: FontBook,
    }
    impl World for TestWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            FileId::from_raw(std::num::NonZeroU16::new(1).unwrap())
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
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<Datetime> {
            None
        }
    }
    fn call(args: &Args) -> SourceResult<Value> {
        let world = TestWorld::default();
        native_place(&mut EvalContext::new(), args, &world, world.main())
    }

    fn at(start: usize) -> Span {
        Span::from_range(
            FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()),
            start..start + 1,
        )
    }
    fn named(name: &str, value: Value) -> Args {
        use crate::entities::args::ArgOccurrence;
        Args::from_occurrences(
            at(1),
            vec![
                ArgOccurrence {
                    name: None,
                    value: Value::Content(Content::text("X")),
                    span: at(2),
                    value_span: at(3),
                },
                ArgOccurrence {
                    name: Some(name.into()),
                    value,
                    span: at(4),
                    value_span: at(5),
                },
            ],
        )
    }

    #[test]
    fn batch_f_place_rejects_malformed_offsets_at_value_span() {
        for (value, kind) in [
            (Value::None, "none"),
            (Value::Auto, "auto"),
            (Value::Bool(true), "boolean"),
            (Value::Str("x".into()), "string"),
            (Value::Array(vec![]), "array"),
            (Value::Fraction(1.0), "fraction"),
        ] {
            for name in ["dx", "dy"] {
                let errors = call(&named(name, value.clone())).unwrap_err();
                assert_eq!(
                    errors[0].message,
                    format!("expected relative length, found {kind}")
                );
                assert_eq!(errors[0].span, at(5));
            }
        }
    }
    #[test]
    fn batch_f_place_preserves_point_extensions_and_vertical_omission() {
        for (value, expected) in [
            (Value::Float(-2.5), -2.5),
            (Value::Int(0), 0.0),
            (Value::Length(Length::pt(4.0)), 4.0),
        ] {
            let Value::Content(Content::Place(place)) =
                call(&named("dx", value)).unwrap()
            else {
                panic!("place")
            };
            assert_eq!(place.dx, expected);
            assert_eq!(place.dy, 0.0);
            assert_eq!(place.alignment.v, None);
        }
    }
    #[test]
    fn batch_f_place_dx_rejection_precedes_dy_and_scope() {
        let mut args = named("dx", Value::Bool(true));
        args.named.insert("dy".into(), Value::None);
        args.named.insert("scope".into(), Value::None);
        let errors = call(&args).unwrap_err();
        assert_eq!(errors[0].message, "expected relative length, found boolean");
        assert_eq!(errors[0].span, at(5));
    }
}
