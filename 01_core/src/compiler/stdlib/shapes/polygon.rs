//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/shapes/polygon.md
//! @prompt-hash c8532bca
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// `polygon(pt1, pt2, ...; fill?, stroke?)` → `Content::Shape { kind: Path, ... }`.
///
/// Cada argumento posicional é um array `[x, y]` de `Length` — desde o
/// Passo 734 só `Length` é aceite (paridade vanilla: "expected relative
/// length, found integer/float"; ver `extract_vertex`).
/// Bbox calculada via `geometry::path_bbox` (analítica para CubicTo;
/// equivalente a min/max para LineTo-only — P277 consolidação DEBT-33).
pub fn native_polygon(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let mut path_items: Vec<PathItem> = Vec::new();

    for (i, val) in args.items.iter().enumerate() {
        // **P734** — `extract_vertex`: só `Length` (paridade vanilla);
        // números nus rejeitados com a mensagem do vanilla.
        let (x, y) = extract_vertex(val, i)?;

        if i == 0 {
            path_items.push(PathItem::MoveTo(Point { x: Pt(x), y: Pt(y) }));
        } else {
            path_items.push(PathItem::LineTo(Point { x: Pt(x), y: Pt(y) }));
        }
    }

    if path_items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "polygon() requer pelo menos um ponto".to_string(),
        )]);
    }

    path_items.push(PathItem::ClosePath);

    let fill = args
        .named
        .get("fill")
        .map(|value| parse_paint(value, args.span))
        .transpose()?
        .flatten();
    let fill_rule = match args.named.get("fill-rule") {
        None => FillRule::NonZero,
        Some(Value::Str(value)) if value.as_str() == "non-zero" => FillRule::NonZero,
        Some(Value::Str(value)) if value.as_str() == "even-odd" => FillRule::EvenOdd,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("polygon(fill-rule): valor inesperado {}", other.type_name()),
            )]);
        }
    };
    let parsed_stroke = args
        .named
        .get("stroke")
        .map(|value| parse_shape_stroke(value, "polygon"))
        .transpose()?;

    // P732 — fallback determinístico (paridade vanilla `Smart::Auto`,
    // lab/typst-original/crates/typst-layout/src/shapes.rs:336-339),
    // idêntico ao de `native_curve` (P727): sem fill nem stroke → stroke
    // preta de 1pt; com fill sem stroke → sem stroke. Sem ele o polígono
    // renderiza página em branco (path no PDF sem operador de pintura —
    // medido: 0 px não-brancos no cristalino vs 898 px no vanilla).
    let stroke = if fill.is_none() && parsed_stroke.is_none() {
        Some(Stroke {
            paint: Paint::Solid(Color::rgb(0, 0, 0)),
            thickness: 1.0,
            overhang: false,
            ..Stroke::default()
        })
    } else {
        parsed_stroke
    };

    // P277 — DEBT-33 CLOSED: usar geometry::path_bbox para bbox.
    // Para LineTo-only paths preserva bit-exact min/max behavior.
    let (min_x, min_y, max_x, max_y) = crate::entities::geometry::path_bbox(&path_items);
    let width =
        if max_x > min_x { Some(Box::new(Value::Float(max_x - min_x))) } else { None };
    let height =
        if max_y > min_y { Some(Box::new(Value::Float(max_y - min_y))) } else { None };

    Ok(Value::Content(Content::Shape(Arc::new(ShapeElem {
        kind: ShapeKind::Path(path_items),
        width,
        height,
        fill,
        stroke,
        fill_rule,
    }))))
}

// ── P1473 — `polygon.regular(fill?, stroke?, size?, vertices?)` ─────────────

/// `polygon.regular(fill?, stroke?, size?, vertices?)` — constructor
/// qualificado que gera um polígono regular circunscrito num círculo de
/// diâmetro `size` (default `1em`), com `vertices` lados (default `3`, `u64`).
///
/// Algoritmo byte-a-byte da fonte pinada
/// (`lab/typst-original/crates/typst-library/src/visualize/polygon.rs:53-88`):
/// mesma fórmula de ângulo (offset `PI*(1/2 − 1/vertices)` + rotação
/// `(i/vertices)·2π`, equivalente a `Angle::from_ratio` = `ratio·TAU`), mesma
/// normalização de bbox por `horizontal_offset`/`vertical_offset` (fold com
/// acumulador inicial `(radius, radius)`, **não** o primeiro ponto). O loop
/// `0..=vertices` (N+1, inclusive) fecha a forma repetindo o primeiro vértice
/// no fim — mesmo comportamento do vanilla (medido: `vertices: 3` devolve 4
/// pares). Reutiliza `PathItem::MoveTo`/`LineTo`/`ClosePath`, `parse_paint`,
/// `parse_shape_stroke`, `path_bbox` e o fallback determinístico P732 já
/// usados por `native_polygon` (mesmo `Content::Shape(ShapeKind::Path(...))`),
/// sem criar `PolygonRegularElem` nem variante nova de `Content`.
///
/// **Decisão de implementação (P1473, Agente A):** `size` default `1em` não é
/// resolvível em tempo de eval — esta função, como `native_polygon`/
/// `native_curve`, não tem acesso a `engine.styles` (assinatura nativa de 4
/// argumentos, sem `Engine`). Resolve o default contra o font-size root do
/// crate (`entities::style_chain::StyleChain::default_chain()`: `size:
/// Some(11.0)`), mesmo valor usado como default em todo o cristalino quando
/// não há `engine` disponível. `size:` explícito aceita `Length`/`Float`/
/// `Int`; só a componente `.abs` de `Length` é usada (paridade com
/// `vertex_component`/`native_rect` — `em` explícito não resolve em tempo de
/// eval, mesma limitação pré-existente).
pub fn native_polygon_regular(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    // Fonte pinada: `regular` não tem posicionais — qualquer positional é erro.
    if !args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "unexpected argument".to_string(),
        )]);
    }
    for key in args.named.keys() {
        if !matches!(key.as_str(), "fill" | "stroke" | "size" | "vertices") {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("unexpected argument: {key}"),
            )]);
        }
    }

    // Default root font-size do crate quando `engine.styles` não está
    // disponível (ver nota de decisão acima) — `entities/style_chain.rs`
    // `StyleChain::default_chain()`.
    const DEFAULT_ROOT_SIZE_PT: f64 = 11.0;

    let size_pt = match args.named.get("size") {
        None => DEFAULT_ROOT_SIZE_PT,
        Some(Value::Length(l)) => l.abs.to_pt(),
        Some(Value::Float(f)) => *f,
        Some(Value::Int(i)) => *i as f64,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("expected length, found {}", other.type_name()),
            )]);
        }
    };

    let vertices: u64 = match args.named.get("vertices") {
        None => 3,
        Some(Value::Int(n)) if *n < 0 => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "number must be at least zero".to_string(),
            )]);
        }
        Some(Value::Int(n)) => *n as u64,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("expected integer, found {}", other.type_name()),
            )]);
        }
    };

    let radius = size_pt / 2.0;
    let angle = |i: f64| -> f64 {
        let offset = std::f64::consts::PI * (0.5 - 1.0 / vertices as f64);
        let rotation = (i / vertices as f64) * std::f64::consts::TAU;
        offset + rotation
    };

    let (horizontal_offset, vertical_offset) = (0..=vertices)
        .map(|v| {
            let a = angle(v as f64);
            ((radius * libm::cos(a)) + radius, (radius * libm::sin(a)) + radius)
        })
        .fold((radius, radius), |(min_x, min_y), (v_x, v_y)| {
            (min_x.min(v_x), min_y.min(v_y))
        });

    let points: Vec<(f64, f64)> = (0..=vertices)
        .map(|v| {
            let a = angle(v as f64);
            let x = (radius * libm::cos(a)) + radius - horizontal_offset;
            let y = (radius * libm::sin(a)) + radius - vertical_offset;
            (x, y)
        })
        .collect();

    let mut path_items: Vec<PathItem> = Vec::with_capacity(points.len() + 1);
    for (i, (x, y)) in points.iter().enumerate() {
        if i == 0 {
            path_items.push(PathItem::MoveTo(Point { x: Pt(*x), y: Pt(*y) }));
        } else {
            path_items.push(PathItem::LineTo(Point { x: Pt(*x), y: Pt(*y) }));
        }
    }
    path_items.push(PathItem::ClosePath);

    let fill = args
        .named
        .get("fill")
        .map(|value| parse_paint(value, args.span))
        .transpose()?
        .flatten();
    let parsed_stroke = args
        .named
        .get("stroke")
        .map(|value| parse_shape_stroke(value, "polygon.regular"))
        .transpose()?;

    // P732 — mesmo fallback determinístico de `native_polygon`: sem fill nem
    // stroke → stroke preta de 1pt; com fill sem stroke → sem stroke.
    let stroke = if fill.is_none() && parsed_stroke.is_none() {
        Some(Stroke {
            paint: Paint::Solid(Color::rgb(0, 0, 0)),
            thickness: 1.0,
            overhang: false,
            ..Stroke::default()
        })
    } else {
        parsed_stroke
    };

    let (min_x, min_y, max_x, max_y) = crate::entities::geometry::path_bbox(&path_items);
    let width =
        if max_x > min_x { Some(Box::new(Value::Float(max_x - min_x))) } else { None };
    let height =
        if max_y > min_y { Some(Box::new(Value::Float(max_y - min_y))) } else { None };

    Ok(Value::Content(Content::Shape(Arc::new(ShapeElem {
        kind: ShapeKind::Path(path_items),
        width,
        height,
        fill,
        stroke,
        fill_rule: FillRule::NonZero,
    }))))
}

#[cfg(test)]
mod tests {
    use super::super::atomization_test_support::call;
    use super::*;

    #[test]
    fn preserves_shape_behavior() {
        let vertex = |x, y| {
            Value::Array(vec![Value::Length(Length::pt(x)), Value::Length(Length::pt(y))])
        };
        let args = Args::positional(vec![
            vertex(0.0, 0.0),
            vertex(10.0, 0.0),
            vertex(0.0, 10.0),
        ]);
        assert!(matches!(
            call(native_polygon, args).unwrap(),
            Value::Content(Content::Shape(_))
        ));
    }
}
