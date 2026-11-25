//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/layout/block.md
//! @prompt-hash 7cbdaa1b
//! @layer L1
//! @updated 2026-09-30
//!
//! Comportamento de `stdlib/layout::block`.

use super::*;
use crate::entities::elements::block::{omitted, BlockElem};
use crate::entities::elements::table_cell::TableCellProperties;
use crate::entities::geometry::Stroke;
use crate::entities::style_chain::StyleChain;

/// Coage `Value::Str` para `Parity` (`"even"` / `"odd"`).
/// Outros tipos ou strings → erro hard.
pub(crate) fn extract_parity(value: &Value) -> SourceResult<Parity> {
    match value {
        Value::Str(s) => match s.as_str() {
            "even" => Ok(Parity::Even),
            "odd" => Ok(Parity::Odd),
            other => Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "pagebreak(to:) deve ser \"even\" ou \"odd\", recebeu \"{}\"",
                    other
                ),
            )]),
        },
        other => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("pagebreak(to:) deve ser string, recebeu {}", other.type_name()),
        )]),
    }
}

/// `block(body, width: ?, height: ?, inset: ?, breakable: true)` →
/// `Content::Block`.
///
/// **Atributos** (subset Fase 1 per ADR-0054 graded):
/// - `body` posicional obrigatório (Content ou Str).
/// - `width: Length` ou `Float`/`Int` (interpretado em pt). Ausente == auto.
/// - `height` análogo.
/// - `inset: Length` (uniforme nos 4 lados; refino futuro para Sides
///   completo via dict).
/// - `breakable: bool` (default `true`).
///
/// **Scope-out** (refino futuro): outset, fill, stroke, radius, clip,
/// spacing, above/below, sticky.
pub fn native_block(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    Ok(Value::Content(Content::Block(std::sync::Arc::new(parse_block(args)?))))
}

/// Argumentos de `block`, partilhados por construtor e declarações.
fn parse_block(args: &Args) -> SourceResult<BlockElem> {
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                "block() espera content ou string como primeiro argumento, recebeu {}",
                other.type_name()
            ),
            )])
        }
        // Body opcional em vanilla; aceitamos ausência como Empty.
        None => Content::Empty,
    };

    let mut width: Option<crate::entities::rel::Rel<Length>> = None;
    let mut height: Option<crate::entities::elements::block::BlockHeight> = None;
    let mut inset_val: Option<&Value> = None;
    let mut breakable: bool = true;

    for (key, value) in args.named.iter() {
        match key.as_str() {
            "width" => {
                width = Some(extract_block_width(value).ok_or_else(|| vec![SourceDiagnostic::error(
                    args.span,
                    format!("block(width:) espera length, recebeu {}", value.type_name()),
                )])?);
            }
            "height" => {
                height = Some(extract_block_height(value).ok_or_else(|| vec![SourceDiagnostic::error(
                    args.span,
                    format!("block(height:) espera length, recebeu {}", value.type_name()),
                )])?);
            }
            // **P475** — inset: Length | Relative | Dict per-side.
            "inset" => { inset_val = Some(value); }
            "breakable" => match value {
                Value::Bool(b) => breakable = *b,
                other => return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("block(breakable:) espera bool, recebeu {}", other.type_name()),
                )]),
            },
            // P231 — aceitar outset/radius/clip (parsing pós-loop).
            // P247 — aceitar fill/stroke (parsing pós-loop paralelo).
            // P250 — aceitar spacing/above/below/sticky (parsing pós-loop).
            "outset" | "radius" | "clip" | "fill" | "stroke"
                | "spacing" | "above" | "below" | "sticky" => {},
            other => return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("block(): argumento nomeado inesperado '{}' (atributos avançados scope-out per ADR-0054 graded — refino futuro)", other),
            )]),
        }
    }

    // Validação: width/height negativos rejeitados.
    for (label, len) in [("width", width)]
        .into_iter()
        .filter_map(|(l, opt)| opt.map(|len| (l, len)))
    {
        if len.rel < 0.0 || len.abs.abs.0 < 0.0 || len.abs.em < 0.0 {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "block({}:): valor negativo não suportado neste passo (P156G)",
                    label
                ),
            )]);
        }
    }
    if let Some(value) = height {
        let valid = match value {
            crate::entities::elements::block::BlockHeight::Relative(value) => {
                value.rel >= 0.0 && value.abs.abs.0 >= 0.0 && value.abs.em >= 0.0
            }
            crate::entities::elements::block::BlockHeight::Fractional(weight) => {
                weight.is_finite() && weight >= 0.0
            }
        };
        if !valid {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "block(height:): valor negativo não suportado neste passo (P1373)",
            )]);
        }
    }

    // **P475** — inset: Length | Relative (abs) | Dict per-side.
    let inset = match inset_val {
        Some(val) => extract_sides_from_value(val, "block", "inset")?,
        None => Sides::uniform(Length::ZERO),
    };

    // P231 — outset. **P475** — aceita Dict per-side além de Length uniforme.
    let outset = match args.named.get("outset") {
        Some(val) => extract_sides_from_value(val, "block", "outset")?,
        None => Sides::uniform(Length::ZERO),
    };
    // P242 — radius `Corners<Length>` (refino face P231 `Option<Length>`).
    // Aceita Length uniforme OR Dict por canto via helper centralizado.
    let radius = match args.named.get("radius") {
        Some(val) => extract_corners_length_value(val, "block")?,
        None => crate::entities::corners::Corners::uniform(Length::ZERO),
    };
    let clip = match args.named.get("clip") {
        Some(Value::Bool(b)) => *b,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("block(clip): espera Bool, recebeu {}", other.type_name()),
            )])
        }
        None => false,
    };

    // P247 — fill: aceita Value::Color directo (paridade pattern Grid/
    // Table inline). Refino futuro para Paint enum quando ADR dedicada.
    // P726 — `fill: none` aceite (= sem preenchimento, paridade vanilla).
    let fill = match args.named.get("fill") {
        Some(Value::Color(c)) => Some(c.clone()),
        Some(Value::None) | None => None,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("block(fill): espera Color, recebeu {}", other.type_name()),
            )])
        }
    };

    // P247 — stroke: reusa `extract_stroke` (helper pré-existente
    // P227 stdlib/layout.rs:351). Aceita Length/Color/Stroke shorthand.
    // P726 — `stroke: none` aceite (= sem traço, paridade vanilla).
    let stroke = match args.named.get("stroke") {
        Some(Value::None) | None => None,
        Some(val) => Some(specified_stroke(val, "block")?),
    };

    // P250 — 4 named args novos: spacing/above/below (Length opcionais;
    // negativos rejeitados) + sticky (Bool). Defaults None×3 + false
    // preservam output literal pre-P250.
    let extract_block_length = |key: &str| -> SourceResult<Option<Length>> {
        match args.named.get(key) {
            None => Ok(None),
            Some(val) => {
                let len = extract_length(val).ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        args.span,
                        format!(
                            "block({}:) espera length, recebeu {}",
                            key,
                            val.type_name()
                        ),
                    )]
                })?;
                if len.abs.0 < 0.0 || len.em < 0.0 {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        format!("block({}:): valor negativo não suportado (P250)", key),
                    )]);
                }
                Ok(Some(len))
            }
        }
    };
    let spacing = extract_block_length("spacing")?;
    let above = extract_block_length("above")?;
    let below = extract_block_length("below")?;
    let sticky = match args.named.get("sticky") {
        Some(Value::Bool(b)) => *b,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("block(sticky): espera Bool, recebeu {}", other.type_name()),
            )])
        }
        None => false,
    };

    let mut missing = omitted::NATIVE;
    for (name, bit) in PROPERTIES {
        if !args.named.contains_key(*name) {
            missing |= bit;
        }
    }
    for (name, bit) in [("inset", omitted::INSET), ("outset", omitted::OUTSET)] {
        missing |= match args.named.get(name) {
            None => bit * 0b1111,
            Some(value) => match TableCellProperties::style_inset(value) {
                Some(sides) => missing_sides(&sides) * bit,
                None => 0,
            },
        };
    }
    Ok(BlockElem { shape: None,
        body,
        width,
        height,
        inset,
        breakable,
        outset,
        radius,
        clip,
        fill,
        stroke,
        spacing,
        above,
        below,
        sticky,
        omitted: missing,
    })
}

const PROPERTIES: &[(&str, u32)] = &[
    ("width", omitted::WIDTH),
    ("height", omitted::HEIGHT),
    ("radius", omitted::RADIUS),
    ("breakable", omitted::BREAKABLE),
    ("clip", omitted::CLIP),
    ("fill", omitted::FILL),
    ("stroke", omitted::STROKE),
    ("spacing", omitted::SPACING),
    ("above", omitted::ABOVE),
    ("below", omitted::BELOW),
    ("sticky", omitted::STICKY),
];

fn missing_sides(sides: &Sides<Option<Length>>) -> u32 {
    u32::from(sides.left.is_none())
        | u32::from(sides.top.is_none()) << 1
        | u32::from(sides.right.is_none()) << 2
        | u32::from(sides.bottom.is_none()) << 3
}

/// Valor a transportar por `#set block`, ou `None` se ainda não tem efeito.
pub(crate) fn declare_block(
    name: &str,
    value: &Value,
    inherited: Option<&Value>,
) -> Option<Value> {
    let restores_default =
        matches!(name, "width" | "height" | "spacing" | "above" | "below");
    if restores_default && matches!(value, Value::Auto) {
        return Some(Value::Auto);
    }
    // As antecipações de flow só conhecem a fração do construtor.
    if name == "height" && matches!(value, Value::Fraction(_)) {
        return None;
    }
    if matches!(name, "inset" | "outset") {
        return declare_sides(value, inherited);
    }
    if name == "stroke" {
        return declare_stroke(value, inherited, "block");
    }
    parse_block(&single(name, value.clone())).ok().map(|_| value.clone())
}

fn single(name: &str, value: Value) -> Args {
    let mut args = Args::positional(Vec::new());
    args.named.insert(name.into(), value);
    args
}

/// Combina lados declarados; o resultado conserva os lados não declarados.
pub(crate) fn declare_sides(value: &Value, inherited: Option<&Value>) -> Option<Value> {
    let sides = TableCellProperties::style_inset(value)?;
    let base = inherited.and_then(TableCellProperties::style_inset);
    let side = |new: Option<Length>, pick: fn(&Sides<Option<Length>>) -> Option<Length>| {
        new.or_else(|| base.as_ref().and_then(pick))
    };
    let mut dict = indexmap::IndexMap::default();
    for (name, length) in [
        ("left", side(sides.left, |s| s.left)),
        ("top", side(sides.top, |s| s.top)),
        ("right", side(sides.right, |s| s.right)),
        ("bottom", side(sides.bottom, |s| s.bottom)),
    ] {
        if let Some(length) = length {
            dict.insert(name.into(), Value::Length(length));
        }
    }
    Some(Value::Dict(dict.into()))
}

/// Combina componentes de traço declarados.
pub(crate) fn declare_stroke(
    value: &Value,
    inherited: Option<&Value>,
    element: &str,
) -> Option<Value> {
    if matches!(value, Value::None) {
        return Some(Value::None);
    }
    let stroke = specified_stroke(value, element).ok()?;
    let base = match inherited {
        Some(Value::Stroke(base)) => Some(base.as_ref()),
        _ => None,
    };
    Some(Value::from(TableCellProperties::fold_stroke(&stroke, base)))
}

/// Traço que conserva quais componentes o atalho especificou.
pub(crate) fn specified_stroke(value: &Value, element: &str) -> SourceResult<Stroke> {
    let mut stroke = extract_stroke(value, element, "stroke")?;
    if matches!(value, Value::Length(_)) {
        stroke.specified = Default::default();
        stroke.specified.thickness = true;
    } else if matches!(value, Value::Color(_)) {
        stroke.specified = Default::default();
        stroke.specified.paint = true;
    }
    Ok(stroke)
}

/// Preenche as propriedades omitidas com as declaradas na cadeia.
pub(crate) fn declared_block(e: &BlockElem, chain: &StyleChain) -> Option<BlockElem> {
    if e.omitted == 0 {
        return None;
    }
    let declared = |name: &str| {
        chain
            .custom(&format!("block.{name}"))
            .filter(|value| !matches!(value, Value::Auto))
    };
    let mut out = None;
    for (name, bit) in PROPERTIES {
        if e.omitted & bit == 0 && !(*name == "stroke" && e.stroke.is_some()) {
            continue;
        }
        let Some(value) = declared(name) else { continue };
        let out = out.get_or_insert_with(|| e.clone());
        if *name == "stroke" && e.omitted & bit == 0 {
            if let (Some(stroke), Value::Stroke(base)) = (&e.stroke, value) {
                out.stroke =
                    Some(TableCellProperties::fold_stroke(stroke, Some(base.as_ref())));
            }
            continue;
        }
        let Ok(parsed) = parse_block(&single(name, value.clone())) else { continue };
        match *name {
            "width" => out.width = parsed.width,
            "height" => out.height = parsed.height,
            "radius" => out.radius = parsed.radius,
            "breakable" => out.breakable = parsed.breakable,
            "clip" => out.clip = parsed.clip,
            "fill" => out.fill = parsed.fill,
            "stroke" => out.stroke = parsed.stroke,
            "above" if e.omitted & omitted::SPACING != 0 => out.above = parsed.above,
            "below" if e.omitted & omitted::SPACING != 0 => out.below = parsed.below,
            "sticky" => out.sticky = parsed.sticky,
            _ => {}
        }
    }
    for (name, bit) in [("inset", omitted::INSET), ("outset", omitted::OUTSET)] {
        let missing = e.omitted / bit & 0b1111;
        if missing == 0 {
            continue;
        }
        let Some(sides) = declared(name).and_then(TableCellProperties::style_inset)
        else {
            continue;
        };
        let out = out.get_or_insert_with(|| e.clone());
        let target = if name == "inset" { &mut out.inset } else { &mut out.outset };
        for (index, (slot, side)) in [
            (&mut target.left, sides.left),
            (&mut target.top, sides.top),
            (&mut target.right, sides.right),
            (&mut target.bottom, sides.bottom),
        ]
        .into_iter()
        .enumerate()
        {
            if missing & (1 << index) != 0 {
                *slot = side.unwrap_or(*slot);
            }
        }
    }
    out
}
