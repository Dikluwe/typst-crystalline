//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/shapes/coordinates.md
//! @prompt-hash 1ab6ab29
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// Extrai um par de coordenadas (x, y) de um `Value::Array` com dois elementos numéricos.
pub(super) fn extract_coordinate(val: &Value) -> Option<(f64, f64)> {
    match val {
        Value::Array(arr) if arr.len() == 2 => {
            let x = coord_component(&arr[0])?;
            let y = coord_component(&arr[1])?;
            Some((x, y))
        }
        _ => None,
    }
}

/// Componente de coordenada de vértice/segmento.
///
/// **P732** — aceita `Length` (paridade vanilla: os vértices de `polygon`
/// são `Rel<Length>`; a componente `em` e `Ratio` ficam scope-out, mesmo
/// precedente de P513 em `curve`) além de `Float`/`Int` em pt (interface
/// cristalina legada, partilhada com `curve`).
pub(super) fn coord_component(val: &Value) -> Option<f64> {
    match val {
        Value::Length(l) => Some(l.abs.to_pt()),
        _ => val.cast_float(),
    }
}

/// Vértice de `polygon`: só `Length` (paridade vanilla — **P734**).
///
/// O vanilla exige `Rel<Length>` nos vértices e rejeita números nus
/// ("expected relative length, found integer/float", medido). `Ratio`
/// (`50%`) é scope-out: o vanilla aceita (resolve no layout contra o
/// contentor), mas o constructor cristalino corre em tempo de eval, sem
/// dimensão de referência — erro explícito de scope-out. A restrição não
/// se aplica a `curve` (interface por tuples, divergência intencional
/// documentada — o vanilla rejeita tuples: "expected content, found array").
pub(super) fn extract_vertex(val: &Value, index: usize) -> SourceResult<(f64, f64)> {
    let arr = match val {
        Value::Array(a) if a.len() == 2 => a,
        _ => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("polygon(): argumento {} não é uma coordenada válida", index),
            )]);
        }
    };
    let x = vertex_component(&arr[0])?;
    let y = vertex_component(&arr[1])?;
    Ok((x, y))
}

pub(super) fn vertex_component(val: &Value) -> SourceResult<f64> {
    match val {
        Value::Length(l) => Ok(l.abs.to_pt()),
        // Paridade verbatim do vanilla (medido): "integer", não "int".
        Value::Int(_) => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "expected relative length, found integer".to_string(),
        )]),
        Value::Ratio(_) => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "polygon(): coordenada ratio (50%) não é resolvível em tempo de eval — scope-out (o vanilla aceita)".to_string(),
        )]),
        // **P741** — pré-P842 o caminho real do utilizador: `50%` chegava
        // como `Value::Relative` com abs zero. Desde P842 (#32) chega como
        // `Value::Ratio` (braço acima); este braço fica para `Relative`
        // construídos por outras vias. A sonda de P741 mediu o vanilla:
        // o ratio resolve contra o contentor em tempo de layout (x contra a
        // largura, y contra a altura — medido exacto: `50%` ≡ `50pt` num
        // box de 100pt, diff 0.0000%). O cristalino não tem altura de
        // contentor inline disponível no emit do shape (Boxed usa
        // `unconstrained_height`) — scope-out reforçado, custo no
        // relatório do passo. Antes deste braço, `Relative` caía no
        // `other` e produzia a mensagem absurda "expected relative
        // length, found relative length".
        Value::Relative(_) => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "polygon(): coordenada relativa (50%) não é resolvível em tempo de eval — scope-out (o vanilla resolve contra o contentor no layout)".to_string(),
        )]),
        other => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("expected relative length, found {}", other.type_name()),
        )]),
    }
}

/// Extrai um `CurvePoint` de um `Value::Array` de 2 elementos.
///
/// Aceita `Value::Length`, `Value::Float` ou `Value::Int` (este último
/// convertido para pt). `Value::Relative`/`Ratio` ficam fora do scope
/// minimal do Passo 513.
pub(super) fn extract_curve_point(
    val: &Value,
    fn_name: &str,
    arg_name: &str,
) -> SourceResult<CurvePoint> {
    let arr = match val {
        Value::Array(a) if a.len() == 2 => a,
        _ => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("{}: {} deve ser um array de 2 elementos", fn_name, arg_name),
            )]);
        }
    };

    let x = extract_curve_length(&arr[0], fn_name, arg_name, "x")?;
    let y = extract_curve_length(&arr[1], fn_name, arg_name, "y")?;
    Ok(CurvePoint { x, y })
}

pub(super) fn extract_curve_length(
    val: &Value,
    fn_name: &str,
    arg_name: &str,
    coord: &str,
) -> SourceResult<Length> {
    match val {
        Value::Length(l) => Ok(*l),
        Value::Float(f) => Ok(Length::pt(*f)),
        Value::Int(i) => Ok(Length::pt(*i as f64)),
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{}: {}.{} deve ser length, float ou int", fn_name, arg_name, coord),
        )]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_shape_behavior() {
        assert_eq!(
            extract_coordinate(&Value::Array(vec![Value::Int(2), Value::Float(3.0)])),
            Some((2.0, 3.0))
        );
    }
}
