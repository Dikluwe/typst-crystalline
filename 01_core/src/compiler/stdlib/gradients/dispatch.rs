//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/gradients/dispatch.md
//! @prompt-hash beae5497
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// Devolve o valor associado a `field` no tipo `gradient` (P736).
///
/// `gradient` é `Value::Type(Type::Gradient)` no scope global (paridade
/// vanilla — medido: `type(gradient)` → `type`); os fields resolvem-se
/// por field access em `Value::Type` (`eval/bindings.rs`), que delega
/// aqui. `None` para campo inexistente — o chamador emite
/// "type gradient does not contain field `<f>`" (verbatim vanilla).
///
/// Histórico: até P736 era `make_gradient_module() -> Value`
/// (`Value::Dict` com os 3 constructors; acesso via field access em Dict).
pub fn gradient_type_field(field: &str) -> Option<Value> {
    Some(match field {
        "linear" => Value::Func(Func::native("linear", native_gradient_linear)),
        // P264 — Radial activa per ADR-0088.
        "radial" => Value::Func(Func::native("radial", native_gradient_radial)),
        // P267 — Conic activa per ADR-0089 (cluster Gradient 3/3 completo).
        "conic" => Value::Func(Func::native("conic", native_gradient_conic)),
        "kind" => Value::Func(Func::native("kind", native_gradient_kind)),
        "stops" => Value::Func(Func::native("stops", native_gradient_stops)),
        "space" => Value::Func(Func::native("space", native_gradient_space)),
        "relative" => Value::Func(Func::native("relative", native_gradient_relative)),
        "angle" => Value::Func(Func::native("angle", native_gradient_angle)),
        "center" => Value::Func(Func::native("center", native_gradient_center)),
        "radius" => Value::Func(Func::native("radius", native_gradient_radius)),
        "focal-center" => {
            Value::Func(Func::native("focal-center", native_gradient_focal_center))
        }
        "focal-radius" => {
            Value::Func(Func::native("focal-radius", native_gradient_focal_radius))
        }
        "sample" => Value::Func(Func::native("sample", native_gradient_sample)),
        "samples" => Value::Func(Func::native("samples", native_gradient_samples)),
        "sharp" => Value::Func(Func::native("sharp", native_gradient_sharp)),
        "repeat" => Value::Func(Func::native("repeat", native_gradient_repeat)),
        _ => return None,
    })
}

pub fn is_gradient_instance_method(method: &str) -> bool {
    matches!(
        method,
        "kind"
            | "stops"
            | "space"
            | "relative"
            | "angle"
            | "center"
            | "radius"
            | "focal-center"
            | "focal-radius"
            | "sample"
            | "samples"
            | "sharp"
            | "repeat"
    )
}

pub(crate) fn dispatch_gradient_method(
    gradient: &Gradient,
    method: &str,
    mut args: Args,
    ctx: &mut EvalContext,
    world: &dyn crate::contracts::world::World,
    current_file: FileId,
) -> SourceResult<Value> {
    args = if args.occurrences.is_some() {
        let mut occurrences = args.occurrence_sequence();
        occurrences.insert(
            0,
            ArgOccurrence {
                name: None,
                value: Value::Gradient(gradient.clone()),
                span: Span::detached(),
                value_span: Span::detached(),
            },
        );
        Args::from_occurrences(args.span, occurrences)
    } else {
        let mut items = args.items;
        items.insert(0, Value::Gradient(gradient.clone()));
        Args::from_parts(items, args.named, args.span)
    };
    match method {
        "kind" => native_gradient_kind(ctx, &args, world, current_file),
        "stops" => native_gradient_stops(ctx, &args, world, current_file),
        "space" => native_gradient_space(ctx, &args, world, current_file),
        "relative" => native_gradient_relative(ctx, &args, world, current_file),
        "angle" => native_gradient_angle(ctx, &args, world, current_file),
        "center" => native_gradient_center(ctx, &args, world, current_file),
        "radius" => native_gradient_radius(ctx, &args, world, current_file),
        "focal-center" => native_gradient_focal_center(ctx, &args, world, current_file),
        "focal-radius" => native_gradient_focal_radius(ctx, &args, world, current_file),
        "sample" => native_gradient_sample(ctx, &args, world, current_file),
        "samples" => native_gradient_samples(ctx, &args, world, current_file),
        "sharp" => native_gradient_sharp(ctx, &args, world, current_file),
        "repeat" => native_gradient_repeat(ctx, &args, world, current_file),
        _ => unreachable!("filtrado por is_gradient_instance_method"),
    }
}
