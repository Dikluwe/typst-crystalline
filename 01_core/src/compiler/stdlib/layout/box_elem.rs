//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/layout/box_elem.md
//! @prompt-hash dda91728
//! @layer L1
//! @updated 2026-09-30
//!
//! Comportamento de `stdlib/layout::box_elem`.

use super::*;
use crate::compiler::stdlib::shapes::frame::{
    fold_sides, fold_stroke, merge, partial_sides,
};
use crate::entities::elements::boxed::{BoxExtra, BoxedElem};
use crate::entities::rel::Rel;
use crate::entities::style_chain::StyleChain;

/// `box(body, width: ?, height: ?, inset: ?, baseline: ?)` →
/// `Content::Boxed`.
///
/// **Atributos** (subset Fase 2 per ADR-0054 graded; padrão variant
/// rico reusado de `block` em P156G):
/// - `body` posicional opcional (Content ou Str; ausente → Empty).
/// - `width: Length`/`Float`/`Int` (em pt). Ausente == content-based.
/// - `height` análogo. Ausente == auto.
/// - `inset: Length` uniforme (refino futuro para Sides via dict).
/// - `baseline: Length` ajuste vertical (default zero).
///
/// **Scope-out** (refino futuro): outset, fill, stroke, radius, clip,
/// stroke-overhang.
///
/// Distinção material face a `block`: posicionamento **inline** vs
/// structural.
pub fn native_box(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    Ok(Value::Content(Content::Boxed(std::sync::Arc::new(parse_box(args)?))))
}

const PROPERTIES: &[&str] = &[
    "width", "height", "baseline", "fill", "stroke", "radius", "inset", "outset", "clip",
];

/// Argumentos de `box`, partilhados por construtor e declarações.
fn parse_box(args: &Args) -> SourceResult<BoxedElem> {
    let mut extra = BoxExtra {
        named: Some(std::sync::Arc::new(args.named.clone())),
        ..BoxExtra::default()
    };
    let mut args = args.clone();
    // Partes relativas ficam para a região; o resto segue o caminho absoluto.
    for name in ["width", "height"] {
        let value = args.named.get(name).cloned();
        if matches!(value, Some(Value::Auto)) {
            args.named.shift_remove(name);
        } else if let Some(rel) = value.as_ref().and_then(relative_only) {
            args.named.shift_remove(name);
            if name == "width" {
                extra.width = Some(rel);
            } else {
                extra.height = Some(rel);
            }
        }
    }
    if let Some(value) = args.named.get("baseline").cloned() {
        if matches!(value, Value::Auto) {
            args.named.shift_remove("baseline");
        } else if let Value::Align(align) = &value {
            let (None, Some(vertical)) = (align.h, align.v) else {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!(
                        "expected `top`, `horizon`, or `bottom`, found {}",
                        crate::compiler::eval::repr::repr_value(&value)
                    ),
                )]);
            };
            extra.align = Some(vertical);
            args.named.shift_remove("baseline");
        } else if let Some(rel) = relative_only(&value) {
            extra.baseline = Some(rel);
            args.named.shift_remove("baseline");
        }
    }
    for name in ["inset", "outset"] {
        let Some(partial) = args.named.get(name).and_then(partial_sides) else { continue };
        let zero = Rel::default;
        let sides = Sides::new(
            partial.left.unwrap_or_else(zero),
            partial.top.unwrap_or_else(zero),
            partial.right.unwrap_or_else(zero),
            partial.bottom.unwrap_or_else(zero),
        );
        let relative = [sides.left, sides.top, sides.right, sides.bottom]
            .iter()
            .any(|side| side.rel != 0.0);
        if relative {
            args.named.shift_remove(name);
            if name == "inset" {
                extra.inset = Some(sides);
            } else {
                extra.outset = Some(sides);
            }
        }
    }
    let stroke = args.named.get("stroke").cloned();
    if let Some(value @ Value::Dict(_)) = &stroke {
        let detached = Span::detached();
        extra.strokes =
            crate::compiler::stdlib::shapes::box_strokes(Some(value), detached)?;
        if extra.strokes.is_some() {
            args.named.shift_remove("stroke");
        } else {
            let stroke = crate::compiler::stdlib::layout::positional_stroke(value, detached)?;
            args.named.insert("stroke".into(), Value::from(stroke));
        }
    }
    let mut elem = parse_absolute_box(&args)?;
    elem.extra = extra;
    Ok(elem)
}

/// Valor com parte relativa.
fn relative_only(value: &Value) -> Option<Rel<Length>> {
    if let Value::Ratio(ratio) = value {
        Some(Rel { rel: ratio.0, abs: Length::ZERO })
    } else if let Value::Relative(rel) = value {
        Some(*rel)
    } else {
        None
    }
}

/// Valor a transportar por `#set box`, ou `None` se ainda não tem efeito.
pub(crate) fn declare_box(
    name: &str,
    value: &Value,
    inherited: Option<&Value>,
) -> Option<Value> {
    if matches!(value, Value::Fraction(_)) {
        return None;
    }
    let mut args = Args::positional(Vec::new());
    args.named.insert(name.into(), value.clone());
    parse_box(&args).ok()?;
    Some(match (name, inherited) {
        ("stroke", Some(base)) => fold_stroke(value, base, "box"),
        ("inset" | "outset", Some(base)) => {
            fold_sides(value, base).unwrap_or_else(|| value.clone())
        }
        _ => value.clone(),
    })
}

/// Caixa com declarações aplicadas e partes relativas resolvidas.
pub(crate) fn declared_box(
    e: &BoxedElem,
    chain: &StyleChain,
    font: f64,
    region: (f64, f64),
) -> Option<BoxedElem> {
    let lexical = e.extra.named.as_ref()?;
    let merged = merge("box", lexical, PROPERTIES, chain);
    let relative = e.extra.width.is_some()
        || e.extra.height.is_some()
        || e.extra.inset.is_some()
        || e.extra.outset.is_some()
        || e.extra.baseline.is_some()
        || e.extra.align.is_some()
        || e.extra.strokes.is_some();
    if merged.is_none() && !relative {
        return None;
    }
    let mut out = match merged {
        Some(named) => {
            let mut args = Args::positional(Vec::new());
            args.named = named;
            let parsed = parse_box(&args).ok()?;
            BoxedElem { body: e.body.clone(), ..parsed }
        }
        None => e.clone(),
    };
    let finite = |value: f64| if value.is_finite() { value } else { 0.0 };
    let resolve = |rel: Rel<Length>, reference: f64| {
        rel.abs.resolve_pt(font) + finite(rel.rel * reference)
    };
    let points = |value: f64| Some(Length::pt(value.max(0.0)));
    if let Some(width) = out.extra.width {
        out.width = points(resolve(width, region.0));
    }
    if let Some(height) = out.extra.height {
        out.height = points(resolve(height, region.1));
    }
    Some(out)
}

fn parse_absolute_box(args: &Args) -> SourceResult<BoxedElem> {
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "box() espera content ou string como primeiro argumento, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => Content::Empty, // body opcional (vanilla aceita)
    };

    let mut width: Option<Length> = None;
    let mut width_fraction: Option<f64> = None;
    let mut height: Option<Length> = None;
    let mut inset_val: Option<&Value> = None;
    let mut baseline: Length = Length::ZERO;

    for (key, value) in args.named.iter() {
        match key.as_str() {
            "width" => {
                let (absolute, fractional) = extract_box_width(value).ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        args.span,
                        format!(
                            "box(width:) espera length ou fraction, recebeu {}",
                            value.type_name()
                        ),
                    )]
                })?;
                width = absolute;
                width_fraction = fractional;
            }
            "height" => {
                height = Some(extract_length(value).ok_or_else(|| vec![SourceDiagnostic::error(
                    args.span,
                    format!("box(height:) espera length, recebeu {}", value.type_name()),
                )])?);
            }
            // **P475** — inset: Length | Relative | Dict per-side.
            "inset" => { inset_val = Some(value); }
            "baseline" => {
                baseline = extract_length(value).ok_or_else(|| vec![SourceDiagnostic::error(
                    args.span,
                    format!("box(baseline:) espera length, recebeu {}", value.type_name()),
                )])?;
            }
            // P231 — aceitar outset/radius/clip (parsing pós-loop paridade block).
            // P247 — aceitar fill/stroke paralelo block.
            "outset" | "radius" | "clip" | "fill" | "stroke" => {},
            other => return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("box(): argumento nomeado inesperado '{}' (atributos avançados scope-out per ADR-0054 graded — refino futuro)", other),
            )]),
        }
    }

    // Validação: width/height negativos rejeitados (baseline negativo ACEITE).
    for (label, len) in [("width", width), ("height", height)]
        .iter()
        .filter_map(|(l, opt)| opt.map(|len| (*l, len)))
        .collect::<Vec<_>>()
    {
        if len.abs.0 < 0.0 || len.em < 0.0 {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "box({}:): valor negativo não suportado neste passo (P156H)",
                    label
                ),
            )]);
        }
    }

    // **P475** — inset: Length | Relative (abs) | Dict per-side.
    let inset = match inset_val {
        Some(val) => extract_sides_from_value(val, "box", "inset")?,
        None => Sides::uniform(Length::ZERO),
    };

    // P231 — outset. **P475** — aceita Dict per-side além de Length uniforme.
    let outset = match args.named.get("outset") {
        Some(val) => extract_sides_from_value(val, "box", "outset")?,
        None => Sides::uniform(Length::ZERO),
    };
    // P242 — radius `Corners<Length>` paralelo block. Aceita Length
    // uniforme OR Dict por canto via helper centralizado.
    let radius = match args.named.get("radius") {
        Some(val) => extract_corners_length_value(val, "box")?,
        None => crate::entities::corners::Corners::uniform(Length::ZERO),
    };
    let clip = match args.named.get("clip") {
        Some(Value::Bool(b)) => *b,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("box(clip): espera Bool, recebeu {}", other.type_name()),
            )])
        }
        None => false,
    };

    // P247 — fill paralelo block.
    // P726 — `fill: none` aceite (= sem preenchimento, paridade vanilla).
    let fill = match args.named.get("fill") {
        Some(Value::Color(c)) => Some(c.clone()),
        Some(Value::None) | None => None,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("box(fill): espera Color, recebeu {}", other.type_name()),
            )])
        }
    };

    // P247 — stroke reusa `extract_stroke` paralelo block.
    // P726 — `stroke: none` aceite (= sem traço, paridade vanilla).
    let stroke = match args.named.get("stroke") {
        Some(Value::None) | None => None,
        Some(val) => Some(extract_stroke(val, "box", "stroke")?),
    };

    Ok(BoxedElem {
        body,
        width,
        width_fraction,
        height,
        inset,
        baseline,
        outset,
        radius,
        clip,
        fill,
        stroke,
        extra: BoxExtra::default(),
    })
}
