//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/color/spot.md
//! @prompt-hash a1bf5b77
//! @layer L1
//! @updated 2026-09-30

use super::*;

// ── P1473 — `color.spot(name, fallback)` (escopo reduzido, sem `.tint()`) ───

/// `color.spot(name, fallback)` — constructor de `Value::SpotColorant`.
/// Despachado por `Value::Type(Type::SpotColorant) => ...` em
/// `eval/call_dispatch.rs` (mesmo mecanismo de `int(...)`/`str(...)`/
/// `stroke(...)`, sem novo caminho de dispatch). Ambos os argumentos são
/// posicionais (paridade vanilla: `#[func(constructor)] fn construct(name:
/// Option<Str>, fallback: ProcessColor)`, sem `#[named]`).
///
/// `name` (`Str` ou `none`, opcional — default `none`, genuinamente
/// opcional na língua): tipo errado → `` expected "all", string, or none,
/// found <tipo> ``. `fallback` (`Color` de processo, obrigatório): ausente
/// → `missing argument: fallback`; `spot`-de-`spot` → `expected color,
/// found spot colorant` (não há spot-de-spot); outro tipo errado →
/// `expected color, found <tipo>`.
pub(crate) fn native_spot_colorant_construct(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if let Some(key) = args.named.keys().next() {
        return err_typed(format!("unexpected argument: {key}"));
    }
    if args.items.len() > 2 {
        return err_typed("unexpected argument");
    }

    // `name` é genuinamente opcional (positional trailing-omittable); um
    // único positional que já é `Color` é o `fallback` sozinho (`name`
    // omitido), não um `name` malformado.
    let (name_arg, fallback_arg): (Option<&Value>, Option<&Value>) =
        match args.items.len() {
            0 => (None, None),
            1 => match &args.items[0] {
                v @ Value::Color(_) => (None, Some(v)),
                other => (Some(other), None),
            },
            _ => (Some(&args.items[0]), Some(&args.items[1])),
        };

    let name: Option<EcoString> = match name_arg {
        None => None,
        Some(Value::None) => None,
        Some(Value::Str(s)) => Some(s.clone()),
        Some(other) => {
            return err_typed(format!(
                "expected \"all\", string, or none, found {}",
                vanilla_type_name(other)
            ));
        }
    };

    let fallback = match fallback_arg {
        None => return err_typed("missing argument: fallback"),
        Some(Value::Color(Color::Spot(_))) => {
            return err_typed("spot color found where process color is required");
        }
        Some(Value::Color(c)) => c.clone(),
        Some(Value::SpotColorant(_)) => {
            return err_typed("expected color, found spot colorant");
        }
        Some(other) => {
            return err_typed(format!(
                "expected color, found {}",
                vanilla_type_name(other)
            ));
        }
    };

    Ok(Value::SpotColorant(crate::entities::color::SpotColorant { name, fallback }))
}

/// Constrói uma cor spot sem perder a identidade do colorante.
pub(crate) fn native_spot_tint(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if let Some(key) = args.named.keys().next() {
        let message = if key == "value" {
            "the argument `value` is positional".to_string()
        } else {
            format!("unexpected argument: {key}")
        };
        return Err(vec![SourceDiagnostic::error(args.span, message)]);
    }
    let colorant = match args.items.first() {
        Some(Value::SpotColorant(colorant)) => colorant,
        Some(other) => {
            return err_typed(format!(
                "expected spot colorant, found {}",
                vanilla_type_name(other)
            ))
        }
        None => return err_typed("missing argument: self"),
    };
    let tint = match args.items.get(1) {
        Some(Value::Ratio(value)) => *value,
        Some(other) => {
            return err_typed(format!(
                "expected ratio, found {}",
                vanilla_type_name(other)
            ))
        }
        None => return err_typed("missing argument: value"),
    };
    if args.items.len() > 2 {
        return err_typed("unexpected argument");
    }
    if tint.get() < 0.0 {
        return err_typed("spot color tint must be positive");
    }
    if tint.get() > 1.0 {
        return err_typed("spot color tint must not exceed 100%");
    }
    Ok(Value::Color(Color::Spot(SpotColor {
        colorant: Arc::new(colorant.clone()),
        tint,
    })))
}

/// Injeta o receptor de uma chamada `spot.tint(value)` sem alterar a ABI
/// da nativa estática `color.spot.tint(spot, value)`.
pub(crate) fn dispatch_spot_colorant_tint(
    colorant: &SpotColorant,
    args: Args,
    ctx: &mut EvalContext,
    world: &dyn crate::contracts::world::World,
    current_file: FileId,
) -> SourceResult<Value> {
    let synth = if args.occurrences.is_some() {
        let mut occurrences = args.occurrence_sequence();
        occurrences.insert(
            0,
            ArgOccurrence {
                name: None,
                value: Value::SpotColorant(colorant.clone()),
                span: Span::detached(),
                value_span: Span::detached(),
            },
        );
        Args::from_occurrences(args.span, occurrences)
    } else {
        let mut items = args.items;
        items.insert(0, Value::SpotColorant(colorant.clone()));
        Args::from_parts(items, args.named, args.span)
    };
    native_spot_tint(ctx, &synth, world, current_file)
}

#[cfg(test)]
mod tests {
    use super::super::atomization_test_support::call;
    use super::*;

    #[test]
    fn preserves_color_behavior() {
        let args = Args::positional(vec![Value::Color(Color::rgb(255, 0, 0))]);
        assert!(matches!(
            call(native_spot_colorant_construct, args).unwrap(),
            Value::SpotColorant(_)
        ));
    }
}
