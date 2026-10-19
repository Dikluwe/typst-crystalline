//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/color/alpha.md
//! @prompt-hash 8c55d959
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// **P744** — `color.transparentize(col, factor)` → Color com alpha reduzido.
pub(crate) fn native_color_transparentize(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if !args.named.is_empty() {
        return err("color.transparentize() não aceita argumentos nomeados");
    }
    match args.items.as_slice() {
        [col, factor] => {
            let c = extract_color_arg(col, "color.transparentize", "col")?;
            let f = extract_ratio_arg(factor, "color.transparentize", "factor")?;
            if matches!(&c, Color::Spot(_)) {
                return err_typed("spot colors do not have an alpha component");
            }
            match c.transparentize(f) {
                Some(t) => Ok(Value::Color(t)),
                None => Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    "CMYK does not have an alpha component",
                )]),
            }
        }
        _ => err(format!(
            "color.transparentize() requer 2 argumentos (col, factor), recebeu {}",
            args.items.len()
        )),
    }
}

/// **P744** — `color.opacify(col, factor)` → Color com alpha aumentado.
pub(crate) fn native_color_opacify(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if !args.named.is_empty() {
        return err("color.opacify() não aceita argumentos nomeados");
    }
    match args.items.as_slice() {
        [col, factor] => {
            let c = extract_color_arg(col, "color.opacify", "col")?;
            let f = extract_ratio_arg(factor, "color.opacify", "factor")?;
            if matches!(&c, Color::Spot(_)) {
                return err_typed("spot colors do not have an alpha component");
            }
            match c.opacify(f) {
                Some(o) => Ok(Value::Color(o)),
                None => Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    "CMYK does not have an alpha component",
                )]),
            }
        }
        _ => err(format!(
            "color.opacify() requer 2 argumentos (col, factor), recebeu {}",
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
        assert!(matches!(
            call(native_color_transparentize, args).unwrap(),
            Value::Color(_)
        ));
    }
}
