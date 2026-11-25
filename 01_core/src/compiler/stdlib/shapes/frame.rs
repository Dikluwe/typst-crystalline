//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/shapes/frame.md
//! @prompt-hash 39d58b49
//! @layer L1
//! @updated 2026-11-24
//!
//! Admissão, construção e projecção de declarações das formas.

use super::rect::{rect_side_strokes, relative_rect_radius};
use super::*;
use crate::compiler::eval::operators::error_formatting::vanilla_type_name;
use crate::entities::elements::block::BlockElem;
use crate::entities::elements::shape::{ShapeExtra, ShapeOrigin};
use crate::entities::elements::table_cell::TableCellProperties;
use crate::entities::rel::Rel;
use crate::entities::sides::Sides;
use crate::entities::style_chain::StyleChain;
use ecow::EcoString;

pub(crate) type Named = indexmap::IndexMap<EcoString, Value, rustc_hash::FxBuildHasher>;

const SIDES: [(&str, &str); 4] =
    [("left", "x"), ("top", "y"), ("right", "x"), ("bottom", "y")];

/// Nome público do elemento de uma origem.
pub(crate) fn element_name(origin: ShapeOrigin) -> Option<&'static str> {
    match origin {
        ShapeOrigin::Geometry => None,
        ShapeOrigin::Rect => Some("rect"),
        ShapeOrigin::Square => Some("square"),
        ShapeOrigin::Circle => Some("circle"),
        ShapeOrigin::Ellipse => Some("ellipse"),
        ShapeOrigin::Line => Some("line"),
        ShapeOrigin::Polygon => Some("polygon"),
        ShapeOrigin::Curve => Some("curve"),
    }
}

/// Propriedades admitidas por um contorno com corpo.
fn properties(origin: ShapeOrigin) -> &'static [&'static str] {
    match origin {
        ShapeOrigin::Rect => {
            &["width", "height", "fill", "stroke", "radius", "inset", "outset"]
        }
        ShapeOrigin::Square => {
            &["size", "width", "height", "fill", "stroke", "radius", "inset", "outset"]
        }
        ShapeOrigin::Circle => {
            &["radius", "width", "height", "fill", "stroke", "inset", "outset"]
        }
        ShapeOrigin::Ellipse => &["width", "height", "fill", "stroke", "inset", "outset"],
        ShapeOrigin::Geometry
        | ShapeOrigin::Line
        | ShapeOrigin::Polygon
        | ShapeOrigin::Curve => &[],
    }
}

fn value_span(args: &Args, name: &str) -> Span {
    args.occurrence_sequence()
        .iter()
        .rev()
        .find(|arg| arg.name.as_deref() == Some(name))
        .map_or(args.span, |arg| arg.value_span.or(arg.span).or(args.span))
}

/// `rect`, `square`, `circle` e `ellipse`.
pub(super) fn native(origin: ShapeOrigin, args: &Args) -> SourceResult<Value> {
    let element = element_name(origin).unwrap_or_default();
    for key in args.named.keys() {
        if !properties(origin).contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("argumento nomeado inesperado em {element}(): '{key}'"),
            )]);
        }
    }
    let positional: Vec<_> = args
        .occurrence_sequence()
        .into_iter()
        .filter(|arg| arg.name.is_none())
        .collect();
    let mut body = None;
    for (index, arg) in positional.iter().enumerate() {
        let content = if index > 0 {
            None
        } else if let Value::None = &arg.value {
            Some(None)
        } else if matches!(
            arg.value,
            Value::Content(_)
                | Value::LocatedContent(..)
                | Value::Str(_)
                | Value::Symbol(_)
        ) {
            crate::compiler::eval::value_to_display_content(arg.value.clone()).map(Some)
        } else {
            None
        };
        match content {
            Some(content) => body = content,
            None => {
                return Err(vec![SourceDiagnostic::error(
                    arg.span.or(args.span),
                    "unexpected argument",
                )])
            }
        }
    }
    let named = normalize(origin, args)?;
    let shape = Arc::new(build(origin, &named, &|name| value_span(args, name))?);
    Ok(Value::Content(match body {
        Some(body) => Content::Block(Arc::new(BlockElem::outlined(body, shape))),
        None => Content::Shape(shape),
    }))
}

/// Substitui `radius` de círculo e `size` de quadrado por largura e altura.
fn normalize(origin: ShapeOrigin, args: &Args) -> SourceResult<Named> {
    let mut named = args.named.clone();
    let name = match origin {
        ShapeOrigin::Circle => "radius",
        ShapeOrigin::Square => "size",
        ShapeOrigin::Geometry
        | ShapeOrigin::Rect
        | ShapeOrigin::Ellipse
        | ShapeOrigin::Line
        | ShapeOrigin::Polygon
        | ShapeOrigin::Curve => return Ok(named),
    };
    let Some(value) = named.shift_remove(name) else { return Ok(named) };
    let length = if let Value::Length(length) = &value {
        *length
    } else if let Value::Float(number) = &value {
        Length::pt(*number)
    } else if let Value::Int(number) = &value {
        Length::pt(*number as f64)
    } else if matches!(value, Value::Auto) {
        return Ok(named);
    } else {
        return Err(vec![SourceDiagnostic::error(
            value_span(args, name),
            format!("expected length or auto, found {}", vanilla_type_name(&value)),
        )]);
    };
    for other in ["width", "height"] {
        if named.contains_key(other) {
            let span = args
                .occurrence_sequence()
                .iter()
                .find(|arg| arg.name.as_deref() == Some(other))
                .map_or(args.span, |arg| arg.span.or(args.span));
            return Err(vec![SourceDiagnostic::error(
                span,
                format!("unexpected argument: {other}"),
            )]);
        }
    }
    let side = if origin == ShapeOrigin::Circle { length * 2.0 } else { length };
    named.insert("width".into(), Value::Length(side));
    named.insert("height".into(), Value::Length(side));
    Ok(named)
}

/// Constrói o contorno a partir de argumentos já normalizados.
fn build(
    origin: ShapeOrigin,
    named: &Named,
    span: &dyn Fn(&str) -> Span,
) -> SourceResult<ShapeElem> {
    let element = element_name(origin).unwrap_or_default();
    let round = matches!(origin, ShapeOrigin::Circle | ShapeOrigin::Ellipse);
    let size = |name: &str, fraction: bool| -> SourceResult<Option<Box<Value>>> {
        let Some(value) = named.get(name) else { return Ok(None) };
        if matches!(value, Value::Auto) {
            return Ok(None);
        }
        let valid = matches!(
            value,
            Value::Length(_)
                | Value::Ratio(_)
                | Value::Relative(_)
                | Value::Float(_)
                | Value::Int(_)
        ) || (fraction && matches!(value, Value::Fraction(_)));
        if !valid {
            let types = if fraction {
                "auto, relative length, or fraction"
            } else {
                "relative length or auto"
            };
            return Err(vec![SourceDiagnostic::error(
                span(name),
                format!("expected {types}, found {}", vanilla_type_name(value)),
            )]);
        }
        Ok(Some(Box::new(value.clone())))
    };
    let width = size("width", false)?;
    let height = size("height", true)?;
    let fill = named
        .get("fill")
        .map(|value| parse_paint(value, span("fill")))
        .transpose()?
        .flatten();
    let sided = if round {
        None
    } else {
        rect_side_strokes(named.get("stroke"), span("stroke"))?
    };
    let stroke = if sided.is_some() {
        None
    } else {
        stroke_or_default(named.get("stroke"), fill.is_some(), element)?
    };
    let mut kind = if round { ShapeKind::Ellipse } else { ShapeKind::Rect };
    let mut rect_radius = None;
    if !round {
        rect_radius = relative_rect_radius(named.get("radius"), span("radius"))?;
        if let (None, Some(value)) = (&rect_radius, named.get("radius")) {
            let radii = crate::compiler::stdlib::layout::extract_corners_length_value(
                value, element,
            )?;
            if radii != Corners::uniform(Length::ZERO) {
                kind = ShapeKind::RoundedRect { radii };
            }
        }
    }
    Ok(ShapeElem {
        origin,
        kind,
        relative_line: None,
        rect_radius,
        rect_stroke: sided,
        width,
        height,
        fill,
        stroke,
        fill_rule: FillRule::NonZero,
        extra: ShapeExtra {
            inset: sides(
                named.get("inset"),
                Rel { rel: 0.0, abs: Length::pt(5.0) },
                span("inset"),
            )?,
            outset: sides(named.get("outset"), Rel::default(), span("outset"))?,
            named: Some(Arc::new(named.clone())),
        },
    })
}

/// Traço explícito, ou o padrão de `auto` conforme o preenchimento.
pub(super) fn stroke_or_default(
    value: Option<&Value>,
    filled: bool,
    element: &str,
) -> SourceResult<Option<Stroke>> {
    let Some(value) = value.filter(|value| !matches!(value, Value::Auto)) else {
        return Ok((!filled).then(default_stroke));
    };
    if super::rect::absent_stroke(value) {
        return Ok(None);
    }
    specified(value, element).map(Some)
}

fn default_stroke() -> Stroke {
    Stroke {
        paint: Paint::Solid(Color::rgb(0, 0, 0)),
        thickness: Length::pt(1.0),
        overhang: false,
        ..Stroke::default()
    }
}

/// Traço que conserva quais componentes o atalho especificou.
fn specified(value: &Value, element: &str) -> SourceResult<Stroke> {
    let mut stroke = parse_shape_stroke(value, element)?;
    if matches!(value, Value::Length(_)) {
        stroke.specified = Default::default();
        stroke.specified.thickness = true;
    } else if matches!(value, Value::Color(_) | Value::Gradient(_) | Value::Tiling(_)) {
        stroke.specified = Default::default();
        stroke.specified.paint = true;
    }
    Ok(stroke)
}

fn relative(value: &Value) -> Option<Rel<Length>> {
    if let Value::Length(abs) = value {
        Some(Rel { rel: 0.0, abs: *abs })
    } else if let Value::Ratio(ratio) = value {
        Some(Rel { rel: ratio.0, abs: Length::ZERO })
    } else if let Value::Relative(rel) = value {
        Some(*rel)
    } else {
        None
    }
}

/// Lados relativos dados por um valor ou dicionário.
pub(crate) fn partial_sides(value: &Value) -> Option<Sides<Option<Rel<Length>>>> {
    if let Some(rel) = relative(value) {
        return Some(Sides::uniform(Some(rel)));
    }
    let Value::Dict(dict) = value else { return None };
    let known = ["left", "top", "right", "bottom", "x", "y", "rest"];
    if dict
        .iter()
        .any(|(key, value)| !known.contains(&key.as_str()) || relative(value).is_none())
    {
        return None;
    }
    let side = |(name, axis): (&str, &str)| {
        [name, axis, "rest"]
            .into_iter()
            .find_map(|key| dict.get(key))
            .and_then(relative)
    };
    let [left, top, right, bottom] = SIDES.map(side);
    Some(Sides::new(left, top, right, bottom))
}

fn sides(
    value: Option<&Value>,
    default: Rel<Length>,
    span: Span,
) -> SourceResult<Sides<Rel<Length>>> {
    let Some(value) = value else { return Ok(Sides::uniform(default)) };
    let partial = partial_sides(value).ok_or_else(|| {
        vec![SourceDiagnostic::error(
            span,
            format!(
                "expected relative length or dictionary, found {}",
                vanilla_type_name(value)
            ),
        )]
    })?;
    Ok(Sides::new(
        partial.left.unwrap_or(default),
        partial.top.unwrap_or(default),
        partial.right.unwrap_or(default),
        partial.bottom.unwrap_or(default),
    ))
}

/// Combina `new` sobre `base`, por lado.
pub(crate) fn fold_sides(new: &Value, base: &Value) -> Option<Value> {
    let (new, base) = (partial_sides(new)?, partial_sides(base)?);
    let mut dict = Named::default();
    for ((name, _), rel) in SIDES.into_iter().zip([
        new.left.or(base.left),
        new.top.or(base.top),
        new.right.or(base.right),
        new.bottom.or(base.bottom),
    ]) {
        if let Some(rel) = rel {
            dict.insert(name.into(), Value::Relative(rel));
        }
    }
    Some(Value::Dict(dict))
}

fn sided_stroke(value: &Value) -> bool {
    matches!(value, Value::Dict(dict) if dict.keys().any(|key| {
        ["left", "top", "right", "bottom", "x", "y", "rest"].contains(&key.as_str())
    }))
}

/// Combina traço `new` sobre `base`, por lado e componente.
pub(crate) fn fold_stroke(new: &Value, base: &Value, element: &str) -> Value {
    if matches!(new, Value::Auto | Value::None) || matches!(base, Value::Auto) {
        return new.clone();
    }
    let one = |new: &Value, base: Option<&Value>| -> Value {
        let Some(base) = base else { return new.clone() };
        if matches!(new, Value::None) || matches!(base, Value::None) {
            return new.clone();
        }
        match (specified(new, element), specified(base, element)) {
            (Ok(new), Ok(base)) => {
                Value::from(TableCellProperties::fold_stroke(&new, Some(&base)))
            }
            _ => new.clone(),
        }
    };
    if !sided_stroke(new) && !sided_stroke(base) {
        return one(new, Some(base));
    }
    let side = |value: &Value, (name, axis): (&str, &str)| -> Option<Value> {
        if let (Value::Dict(dict), true) = (value, sided_stroke(value)) {
            [name, axis, "rest"]
                .into_iter()
                .find_map(|key| dict.get(key))
                .cloned()
        } else {
            Some(value.clone())
        }
    };
    let mut dict = Named::default();
    for names in SIDES {
        let base = side(base, names);
        let folded = match side(new, names) {
            Some(new) => Some(one(&new, base.as_ref())),
            None => base,
        };
        if let Some(value) = folded {
            dict.insert(names.0.into(), value);
        }
    }
    Value::Dict(dict)
}

/// Valores a transportar por `#set`, ou `None` se ainda não têm efeito.
pub(crate) fn declare(
    element: &str,
    name: &str,
    value: &Value,
    inherited: &dyn Fn(&str) -> Option<Value>,
) -> Option<Vec<(String, Value)>> {
    let origin = [
        ShapeOrigin::Rect,
        ShapeOrigin::Square,
        ShapeOrigin::Circle,
        ShapeOrigin::Ellipse,
        ShapeOrigin::Line,
        ShapeOrigin::Polygon,
        ShapeOrigin::Curve,
    ]
    .into_iter()
    .find(|origin| element_name(*origin) == Some(element))?;
    if matches!(value, Value::Fraction(_)) {
        return None;
    }
    let mut args = Args::positional(Vec::new());
    args.named.insert(name.into(), value.clone());
    let detached = |_: &str| Span::detached();
    let named = match origin {
        ShapeOrigin::Line => {
            super::line::parse_line(&args).ok()?;
            args.named
        }
        ShapeOrigin::Polygon | ShapeOrigin::Curve => {
            super::paint::path_paint(&args.named, element, &detached).ok()?;
            args.named
        }
        ShapeOrigin::Geometry
        | ShapeOrigin::Rect
        | ShapeOrigin::Square
        | ShapeOrigin::Circle
        | ShapeOrigin::Ellipse => {
            let restores = matches!((name, value), ("radius" | "size", Value::Auto));
            if restores && matches!(origin, ShapeOrigin::Circle | ShapeOrigin::Square) {
                return Some(Vec::new());
            }
            let named = normalize(origin, &args).ok()?;
            build(origin, &named, &detached).ok()?;
            named
        }
    };
    Some(
        named
            .into_iter()
            .map(|(name, value)| {
                let key = format!("{element}.{name}");
                let value = match (name.as_str(), inherited(&key)) {
                    ("stroke", Some(base)) => fold_stroke(&value, &base, element),
                    ("inset" | "outset", Some(base)) => {
                        fold_sides(&value, &base).unwrap_or(value)
                    }
                    _ => value,
                };
                (key, value)
            })
            .collect(),
    )
}

/// Argumentos lexicais completados com as declarações da cadeia.
pub(crate) fn merge(
    element: &str,
    lexical: &Named,
    names: &[&str],
    chain: &StyleChain,
) -> Option<Named> {
    let mut merged = None;
    for name in names {
        let Some(declared) = chain.custom(&format!("{element}.{name}")) else { continue };
        let value = match (*name, lexical.get(*name)) {
            (_, None) => declared.clone(),
            ("stroke", Some(explicit)) => fold_stroke(explicit, declared, element),
            ("inset" | "outset", Some(explicit)) => {
                fold_sides(explicit, declared).unwrap_or_else(|| explicit.clone())
            }
            _ => continue,
        };
        merged
            .get_or_insert_with(|| lexical.clone())
            .insert((*name).into(), value);
    }
    merged
}

/// Forma com as propriedades declaradas, se alguma se aplica.
pub(crate) fn declared_shape(e: &ShapeElem, chain: &StyleChain) -> Option<ShapeElem> {
    let element = element_name(e.origin)?;
    let lexical = e.extra.named.as_ref()?;
    let detached = |_: &str| Span::detached();
    let names: &[&str] = match e.origin {
        ShapeOrigin::Line => &["length", "angle", "start", "end", "stroke"],
        ShapeOrigin::Polygon | ShapeOrigin::Curve => &["fill", "fill-rule", "stroke"],
        ShapeOrigin::Geometry
        | ShapeOrigin::Rect
        | ShapeOrigin::Square
        | ShapeOrigin::Circle
        | ShapeOrigin::Ellipse => {
            &["width", "height", "fill", "stroke", "radius", "inset", "outset"]
        }
    };
    let names: Vec<_> = names
        .iter()
        .copied()
        .filter(|name| !(e.origin == ShapeOrigin::Circle && *name == "radius"))
        .filter(|name| {
            !(e.origin == ShapeOrigin::Line
                && *name != "stroke"
                && (lexical.contains_key("dx") || lexical.contains_key("dy")))
        })
        .collect();
    let merged = merge(element, lexical, &names, chain)?;
    match e.origin {
        ShapeOrigin::Line => {
            let mut args = Args::positional(Vec::new());
            args.named = merged;
            super::line::parse_line(&args).ok()
        }
        ShapeOrigin::Polygon | ShapeOrigin::Curve => {
            let (fill, fill_rule, stroke) =
                super::paint::path_paint(&merged, element, &detached).ok()?;
            Some(ShapeElem { fill, fill_rule, stroke, ..e.clone() })
        }
        ShapeOrigin::Geometry
        | ShapeOrigin::Rect
        | ShapeOrigin::Square
        | ShapeOrigin::Circle
        | ShapeOrigin::Ellipse => build(e.origin, &merged, &detached).ok(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn named(entries: &[(&str, Value)]) -> Args {
        let mut args = Args::positional(Vec::new());
        for (name, value) in entries {
            args.named.insert((*name).into(), value.clone());
        }
        args
    }

    #[test]
    fn body_is_kept_and_leftovers_fail() {
        for origin in [
            ShapeOrigin::Rect,
            ShapeOrigin::Square,
            ShapeOrigin::Circle,
            ShapeOrigin::Ellipse,
        ] {
            let args = Args::positional(vec![Value::Str("a".into())]);
            let Value::Content(Content::Block(block)) = native(origin, &args).unwrap()
            else {
                panic!("a body needs its container");
            };
            assert_eq!(block.body.plain_text(), "a");
            assert_eq!(block.shape.as_ref().unwrap().origin, origin);
            let empty = native(origin, &Args::positional(Vec::new())).unwrap();
            assert!(matches!(empty, Value::Content(Content::Shape(_))));
            for items in [
                vec![Value::Int(1)],
                vec![Value::Str("a".into()), Value::Str("b".into())],
            ] {
                let errors = native(origin, &Args::positional(items)).unwrap_err();
                assert_eq!(errors[0].message, "unexpected argument");
            }
        }
    }

    #[test]
    fn radius_and_size_are_both_dimensions() {
        let ten = Value::Length(Length::pt(10.0));
        let circle =
            normalize(ShapeOrigin::Circle, &named(&[("radius", ten.clone())])).unwrap();
        assert_eq!(circle["width"], Value::Length(Length::pt(20.0)));
        assert_eq!(circle["height"], Value::Length(Length::pt(20.0)));
        let square =
            normalize(ShapeOrigin::Square, &named(&[("size", ten.clone())])).unwrap();
        assert_eq!(square["height"], ten);
        assert!(normalize(
            ShapeOrigin::Square,
            &named(&[("size", ten.clone()), ("width", ten)])
        )
        .is_err());
    }

    #[test]
    fn strokes_and_sides_fold() {
        let red = Value::Color(Color::rgb(255, 0, 0));
        let thick = Value::Length(Length::pt(3.0));
        let Value::Stroke(stroke) = fold_stroke(&thick, &red, "rect") else {
            panic!("uniform fold")
        };
        assert_eq!(stroke.thickness, Length::pt(3.0));
        assert_eq!(stroke.paint, Paint::Solid(Color::rgb(255, 0, 0)));
        assert_eq!(fold_stroke(&Value::Auto, &red, "rect"), Value::Auto);
        let left = Value::Dict([("left".into(), thick)].into_iter().collect());
        let Value::Dict(dict) = fold_stroke(&left, &red, "rect") else {
            panic!("sided fold")
        };
        assert_eq!(dict.len(), 4);
        assert_eq!(dict["top"], red);
    }
}
