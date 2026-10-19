//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/color/light.md
//! @prompt-hash 7994f0d9
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// `color.lighten(col, amount)` — aumenta luminância por `amount` **no
/// espaço da própria cor** (semântica vanilla corrigida em P742; era Oklch).
pub(crate) fn native_color_lighten(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if !args.named.is_empty() {
        return err("color.lighten() não aceita argumentos nomeados");
    }
    match args.items.as_slice() {
        [col, amount] => {
            let c = extract_color_arg(col, "color.lighten", "col")?;
            let a = extract_ratio_arg(amount, "color.lighten", "amount")?;
            Ok(Value::Color(c.lighten(a)))
        }
        _ => err(format!(
            "color.lighten() requer 2 argumentos (col, amount), recebeu {}",
            args.items.len()
        )),
    }
}

/// `color.darken(col, amount)` — diminui luminância por `amount` **no
/// espaço da própria cor** (semântica vanilla corrigida em P742; era Oklch).
pub(crate) fn native_color_darken(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if !args.named.is_empty() {
        return err("color.darken() não aceita argumentos nomeados");
    }
    match args.items.as_slice() {
        [col, amount] => {
            let c = extract_color_arg(col, "color.darken", "col")?;
            let a = extract_ratio_arg(amount, "color.darken", "amount")?;
            Ok(Value::Color(c.darken(a)))
        }
        _ => err(format!(
            "color.darken() requer 2 argumentos (col, amount), recebeu {}",
            args.items.len()
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::super::atomization_test_support::call;
    use super::*;

    #[test]
    fn preserves_color_behavior() {
        let args = Args::positional(vec![
            Value::Color(Color::rgb(255, 0, 0)),
            Value::Float(0.2),
        ]);
        assert!(matches!(call(native_color_lighten, args).unwrap(), Value::Color(_)));
    }
}
