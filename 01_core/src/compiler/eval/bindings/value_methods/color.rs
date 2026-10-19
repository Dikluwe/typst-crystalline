//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/bindings/value_methods/color.md
//! @prompt-hash c6ae829d
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// **P742** — Despacha métodos de `Value::Color` (padrão P506).
///
/// Sintetiza `Args` com a cor como primeiro posicional e delega nas
/// nativas estáticas de `rules/stdlib/color.rs` — validação, semântica e
/// mensagens idênticas nos dois caminhos (medido: `red.lighten(20%)` ≡
/// `color.lighten(red, 20%)`). Só é chamado para os 9 métodos de
/// `is_color_instance_method`; o braço em `closures.rs` filtra os restantes
/// (que caem no caminho genérico de field access).
pub(in crate::compiler::eval) fn eval_color_method(
    color: &crate::entities::layout_types::Color,
    method: &str,
    args: crate::entities::ast::expr::Args<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    use crate::compiler::eval::call_dispatch::eval_args;
    use crate::compiler::stdlib::color as color_rules;
    let span = args.span();
    let args = eval_args(args, scopes, ctx, engine)?;
    let synth = if args.occurrences.is_some() {
        let mut occurrences = args.occurrence_sequence();
        occurrences.insert(
            0,
            ArgOccurrence {
                name: None,
                value: Value::Color(color.clone()),
                span: Span::detached(),
                value_span: Span::detached(),
            },
        );
        Args::from_occurrences(args.span, occurrences)
    } else {
        let mut items = args.items;
        items.insert(0, Value::Color(color.clone()));
        Args::from_parts(items, args.named, args.span)
    };
    let world = engine.world;
    let current_file = engine.current_file;
    match method {
        "lighten" => color_rules::native_color_lighten(ctx, &synth, world, current_file),
        "darken" => color_rules::native_color_darken(ctx, &synth, world, current_file),
        "mix" => {
            // **P744** — o método de instância `red.mix(blue)` não aceita
            // `weight:` (só `space:`); o peso é sempre 0.5. A estática
            // `color.mix(red, blue, weight: ...)` mantém o argumento.
            if synth.named.contains_key("weight") {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    "unexpected argument: weight".to_string(),
                )]);
            }
            color_rules::native_color_mix(ctx, &synth, world, current_file)
        }
        "negate" => color_rules::native_color_negate(ctx, &synth, world, current_file),
        "saturate" => {
            color_rules::native_color_saturate(ctx, &synth, world, current_file)
        }
        "desaturate" => {
            color_rules::native_color_desaturate(ctx, &synth, world, current_file)
        }
        "rotate" => color_rules::native_color_rotate(ctx, &synth, world, current_file),
        "components" => {
            color_rules::native_color_components(ctx, &synth, world, current_file)
        }
        "space" => color_rules::native_color_space(ctx, &synth, world, current_file),
        "to-hex" => color_rules::native_color_to_hex(ctx, &synth, world, current_file),
        "transparentize" => {
            color_rules::native_color_transparentize(ctx, &synth, world, current_file)
        }
        "opacify" => color_rules::native_color_opacify(ctx, &synth, world, current_file),
        _ => Err(vec![SourceDiagnostic::error(
            span,
            format!("color não tem método '{method}'"),
        )]),
    }
}
