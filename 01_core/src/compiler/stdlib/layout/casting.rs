//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/layout/casting.md
//! @prompt-hash 933e041f
//! @layer L1
//! @updated 2026-10-08
//!
//! Comportamento de `stdlib/layout::casting`.

use super::*;

pub(crate) fn page_type_error(
    span: Span,
    expected: &str,
    found: &Value,
) -> SourceDiagnostic {
    SourceDiagnostic::error(
        span,
        format!("expected {expected}, found {}", found.type_name()),
    )
}

/// Coage `Value` para `Length`. Aceita `Length`, `Float` (interpretado em pt),
/// `Int` (idem). Retorna `None` para outros tipos.
pub(crate) fn extract_length(val: &Value) -> Option<Length> {
    match val {
        Value::Length(l) => Some(*l),
        Value::Float(f) => Some(Length { abs: Abs(*f), em: 0.0 }),
        Value::Int(i) => Some(Length { abs: Abs(*i as f64), em: 0.0 }),
        // **P475** — `Rel<Length>` aceite: parte relativa (`rel`) truncada para zero
        // (resolução percentual requer contexto de layout — scope-out P475).
        Value::Relative(r) => Some(r.abs),
        // **P842 (#32)** — o literal percentual é agora `Value::Ratio`;
        // mantém o scope-out P475 (parte relativa truncada para zero) para
        // não regredir `h(50%)`/`pad(50%)` etc. face ao comportamento
        // pré-P842 (que truncava `Relative.rel`).
        Value::Ratio(_) => Some(Length::ZERO),
        Value::None
        | Value::Bool(..)
        | Value::Str(..)
        | Value::Array(..)
        | Value::Dict(..)
        | Value::Module(..)
        | Value::Datetime(..)
        | Value::Func(..)
        | Value::Content(..)
        | Value::LocatedContent(..)
        | Value::Auto
        | Value::Angle(..)
        | Value::Color(..)
        | Value::Stroke(..)
        | Value::Fraction(..)
        | Value::Align(..)
        | Value::Location(..)
        | Value::Gradient(..)
        | Value::Regex(..)
        | Value::Tiling(..)
        | Value::Bytes(..)
        | Value::Decimal(..)
        | Value::Duration(..)
        | Value::Version(..)
        | Value::Selector(..)
        | Value::Symbol(..)
        | Value::Args(..)
        | Value::State(..)
        | Value::Counter(..)
        | Value::Label(..)
        | Value::Dir(..)
        | Value::Path(..)
        | Value::Type(..)
        | Value::SpotColorant(..) => None,
    }
}

/// P1374 — conserva percentagem/offset de `block(width:)` até o layout.
pub(crate) fn extract_block_width(
    val: &Value,
) -> Option<crate::entities::rel::Rel<Length>> {
    use crate::entities::rel::Rel;
    match val {
        Value::Relative(value) => Some(*value),
        Value::Ratio(value) => Some(Rel { rel: value.get(), abs: Length::ZERO }),
        Value::Length(value) => Some(Rel { rel: 0.0, abs: *value }),
        Value::Float(value) => Some(Rel { rel: 0.0, abs: Length::pt(*value) }),
        Value::Int(value) => Some(Rel { rel: 0.0, abs: Length::pt(*value as f64) }),
        Value::None
        | Value::Bool(..)
        | Value::Str(..)
        | Value::Array(..)
        | Value::Dict(..)
        | Value::Module(..)
        | Value::Datetime(..)
        | Value::Func(..)
        | Value::Content(..)
        | Value::LocatedContent(..)
        | Value::Auto
        | Value::Angle(..)
        | Value::Color(..)
        | Value::Stroke(..)
        | Value::Fraction(..)
        | Value::Align(..)
        | Value::Location(..)
        | Value::Gradient(..)
        | Value::Regex(..)
        | Value::Tiling(..)
        | Value::Bytes(..)
        | Value::Decimal(..)
        | Value::Duration(..)
        | Value::Version(..)
        | Value::Selector(..)
        | Value::Symbol(..)
        | Value::Args(..)
        | Value::State(..)
        | Value::Counter(..)
        | Value::Label(..)
        | Value::Dir(..)
        | Value::Path(..)
        | Value::Type(..)
        | Value::SpotColorant(..) => None,
    }
}

/// P1374 — conserva altura relativa ou fracionária até ao flow layout.
pub(crate) fn extract_block_height(
    val: &Value,
) -> Option<crate::entities::elements::block::BlockHeight> {
    use crate::entities::elements::block::BlockHeight;
    match val {
        Value::Fraction(value) => Some(BlockHeight::Fractional(*value)),
        _ => extract_block_width(val).map(BlockHeight::Relative),
    }
}

/// P1408 — preserva a alternativa entre largura absoluta/relativa histórica
/// e peso fracionário até à construção de `BoxedElem`.
pub(crate) fn extract_box_width(val: &Value) -> Option<(Option<Length>, Option<f64>)> {
    match val {
        Value::Fraction(weight) => Some((None, Some(*weight))),
        _ => extract_length(val).map(|width| (Some(width), None)),
    }
}

/// **P475** — Parse `Value` para `Sides<Length>`: uniforme (Length / Relative / Float / Int)
/// ou dict `{left?, right?, top?, bottom?, x?, y?, rest?}` per-side.
/// Precedência dentro do dict: específico > eixo (`x`/`y`) > `rest`.
/// Lados não declarados num dict ficam `Length::ZERO`.
/// Parte `rel` de `Value::Relative` truncada (scope-out: requer contexto de layout).
pub(crate) fn extract_sides_from_value(
    val: &Value,
    fn_name: &str,
    field: &str,
) -> SourceResult<Sides<Length>> {
    let reject_neg = |l: Length, key: &str| -> SourceResult<Length> {
        if l.abs.0 < 0.0 || l.em < 0.0 {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("{}({}:{}): valor negativo não suportado", fn_name, field, key),
            )]);
        }
        Ok(l)
    };
    // Uniforme: Length, Float, Int, Relative.
    if let Some(l) = extract_length(val) {
        let l = reject_neg(l, "")?;
        return Ok(Sides::uniform(l));
    }
    // Dict per-side: {left, right, top, bottom, x, y, rest}.
    if let Value::Dict(d) = val {
        let mut left: Option<Length> = None;
        let mut right: Option<Length> = None;
        let mut top: Option<Length> = None;
        let mut bottom: Option<Length> = None;
        let mut x_axis: Option<Length> = None;
        let mut y_axis: Option<Length> = None;
        let mut rest: Option<Length> = None;
        for (k, v) in d.iter() {
            let l = extract_length(v).ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "{}({}:) dict: chave '{}' espera length, recebeu {}",
                        fn_name,
                        field,
                        k,
                        v.type_name()
                    ),
                )]
            })?;
            let l = reject_neg(l, k.as_str())?;
            match k.as_str() {
                "left" => left = Some(l),
                "right" => right = Some(l),
                "top" => top = Some(l),
                "bottom" => bottom = Some(l),
                "x" => x_axis = Some(l),
                "y" => y_axis = Some(l),
                "rest" => rest = Some(l),
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        format!(
                            "{}({}:) dict: chave inesperada '{}'",
                            fn_name, field, other
                        ),
                    )])
                }
            }
        }
        return Ok(Sides {
            left: left.or(x_axis).or(rest).unwrap_or(Length::ZERO),
            right: right.or(x_axis).or(rest).unwrap_or(Length::ZERO),
            top: top.or(y_axis).or(rest).unwrap_or(Length::ZERO),
            bottom: bottom.or(y_axis).or(rest).unwrap_or(Length::ZERO),
        });
    }
    Err(vec![SourceDiagnostic::error(
        Span::detached(),
        format!(
            "{}({}:) espera length ou dict, recebeu {}",
            fn_name,
            field,
            val.type_name()
        ),
    )])
}

/// P227 — Coage `Value` para `Stroke` aceitando shorthands paridade
/// vanilla (Opção β); **P252** — defaults `overhang: true` (paridade
/// vanilla user-facing; construtor Rust low-level usa `false` per
/// ADR-0054 divergência consciente):
/// - `Value::Length(l)` → `Stroke { paint: Color::BLACK, thickness: l.to_pt(), overhang: true }`.
/// - `Value::Color(c)` → `Stroke { paint: Paint::Solid(c), thickness: 1.0, overhang: true }` (default 1pt).
/// - `Value::from(s)` → `s.clone()` (preserva overhang do user).
/// - Outros tipos: erro hard.
///
/// `thickness <= 0` rejeitado (paridade vanilla).
pub(crate) fn extract_stroke(
    val: &Value,
    fn_name: &str,
    field: &str,
) -> SourceResult<crate::entities::geometry::Stroke> {
    use crate::entities::geometry::Stroke;
    use crate::entities::layout_types::Color;
    use crate::entities::paint::Paint;
    let stroke = match val {
        Value::Length(l) => {
            let thickness = l.abs.to_pt();
            // P252 — vanilla default overhang=true para inputs stdlib
            // (cobertura Length atalho).
            Stroke {
                paint: Paint::Solid(Color::rgb(0, 0, 0)),
                thickness,
                overhang: true,
                ..Stroke::default()
            }
        }
        Value::Color(c) => Stroke {
            paint: Paint::Solid(c.clone()),
            thickness: 1.0,
            overhang: true,
            ..Stroke::default()
        },
        Value::Stroke(s) => s.as_ref().clone(),
        other => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "{}({}): espera Length / Color / Stroke, recebeu {}",
                    fn_name,
                    field,
                    other.type_name()
                ),
            )])
        }
    };
    if stroke.thickness <= 0.0 {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!(
                "{}({}): thickness deve ser > 0 (recebeu {})",
                fn_name, field, stroke.thickness
            ),
        )]);
    }
    Ok(stroke)
}

/// Helper Passo 156L: parse named args left/top/right/bottom + atalhos
/// x/y/rest, retornando `Sides<Option<Length>>`. Precedência vanilla:
/// específico > eixo > rest. Lados não declarados ficam `None`.
///
/// Pré-decisão (per diagnóstico §5.2): helper privado, não-genérico
/// (toma `Length` directamente). Promoção a genérico/público diferida
/// até segundo reuso (padrão N=2 mínimo para promoção).
///
/// Validação: cada lado declarado rejeita negativos (perfil ADR-0054
/// graded — vanilla aceita; cristalino diverge intencionalmente).
/// Named arg desconhecido rejeitado.
pub(crate) fn extract_sides_lengths(
    args: &Args,
    fn_name: &str,
) -> SourceResult<Sides<Option<Length>>> {
    let mut left: Option<Length> = None;
    let mut right: Option<Length> = None;
    let mut top: Option<Length> = None;
    let mut bottom: Option<Length> = None;
    let mut x_axis: Option<Length> = None;
    let mut y_axis: Option<Length> = None;
    let mut rest: Option<Length> = None;

    for (key, value) in args.named.iter() {
        let len = extract_length(value).ok_or_else(|| {
            vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "{}({}:) espera length, recebeu {}",
                    fn_name,
                    key,
                    value.type_name()
                ),
            )]
        })?;
        match key.as_str() {
            "left" => left = Some(len),
            "right" => right = Some(len),
            "top" => top = Some(len),
            "bottom" => bottom = Some(len),
            "x" => x_axis = Some(len),
            "y" => y_axis = Some(len),
            "rest" => rest = Some(len),
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("{}(): argumento nomeado inesperado '{}'", fn_name, other),
                )])
            }
        }
    }

    // Precedência: específico > eixo > rest.
    let resolved_left = left.or(x_axis).or(rest);
    let resolved_right = right.or(x_axis).or(rest);
    let resolved_top = top.or(y_axis).or(rest);
    let resolved_bottom = bottom.or(y_axis).or(rest);

    // Validação: rejeitar negativos em qualquer lado declarado.
    for (label, opt) in [
        ("left", resolved_left),
        ("right", resolved_right),
        ("top", resolved_top),
        ("bottom", resolved_bottom),
    ] {
        if let Some(len) = opt {
            if len.abs.0 < 0.0 || len.em < 0.0 {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!(
                        "{}({}:): padding negativo não suportado neste passo (P156C/L)",
                        fn_name, label
                    ),
                )]);
            }
        }
    }

    Ok(Sides {
        left: resolved_left,
        top: resolved_top,
        right: resolved_right,
        bottom: resolved_bottom,
    })
}

/// **P242 (M9d / M7+5)** — extrai `Corners<Length>` a partir de um
/// `Value` único (named arg `radius:` em `block`/`box`). Aceita:
/// - `Value::Length(L)` (ou coerções via `extract_length`) → uniforme
///   `Corners::uniform(L)`.
/// - `Value::Dict(d)` com keys `top-left` / `top-right` /
///   `bottom-right` / `bottom-left` / `top` / `bottom` / `left` /
///   `right` / `rest`.
///
/// **Precedência** (paridade `extract_sides_lengths` per ADR-0064
/// Caso C): canto específico > eixo (top/bottom/left/right) > `rest`.
/// Cantos omitidos preservam-se em `Length::ZERO`.
///
/// **Validação**: negativos rejeitados (paridade `block(radius)` P231).
pub(crate) fn extract_corners_length_value(
    value: &Value,
    fn_name: &str,
) -> SourceResult<crate::entities::corners::Corners<Length>> {
    use crate::entities::corners::Corners;
    // Caso 1: Length uniforme.
    if let Some(len) = extract_length(value) {
        if len.abs.0 < 0.0 || len.em < 0.0 {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("{}(radius): negativo rejeitado", fn_name),
            )]);
        }
        return Ok(Corners::uniform(len));
    }
    // Caso 2: Dict por canto.
    if let Value::Dict(d) = value {
        let mut tl: Option<Length> = None;
        let mut tr: Option<Length> = None;
        let mut br: Option<Length> = None;
        let mut bl: Option<Length> = None;
        let mut top: Option<Length> = None;
        let mut bottom: Option<Length> = None;
        let mut left: Option<Length> = None;
        let mut right: Option<Length> = None;
        let mut rest: Option<Length> = None;
        for (key, val) in d.iter() {
            let len = extract_length(val).ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "{}(radius.{}:) espera length, recebeu {}",
                        fn_name,
                        key,
                        val.type_name()
                    ),
                )]
            })?;
            if len.abs.0 < 0.0 || len.em < 0.0 {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("{}(radius.{}:) negativo rejeitado", fn_name, key),
                )]);
            }
            match key.as_str() {
                "top-left" => tl = Some(len),
                "top-right" => tr = Some(len),
                "bottom-right" => br = Some(len),
                "bottom-left" => bl = Some(len),
                "top" => top = Some(len),
                "bottom" => bottom = Some(len),
                "left" => left = Some(len),
                "right" => right = Some(len),
                "rest" => rest = Some(len),
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        format!(
                            "{}(radius): chave de canto inesperada '{}'",
                            fn_name, other
                        ),
                    )])
                }
            }
        }
        // Precedência: específico > eixo > rest.
        let zero = Length::ZERO;
        let resolved_tl = tl.or(top).or(left).or(rest).unwrap_or(zero);
        let resolved_tr = tr.or(top).or(right).or(rest).unwrap_or(zero);
        let resolved_br = br.or(bottom).or(right).or(rest).unwrap_or(zero);
        let resolved_bl = bl.or(bottom).or(left).or(rest).unwrap_or(zero);
        return Ok(Corners::new(resolved_tl, resolved_tr, resolved_br, resolved_bl));
    }
    Err(vec![SourceDiagnostic::error(
        Span::detached(),
        format!(
            "{}(radius:) espera length ou dict por canto, recebeu {}",
            fn_name,
            value.type_name()
        ),
    )])
}

#[cfg(test)]
mod domain_tests {
    use super::*;
    #[test]
    fn measure_domain_keeps_affine_and_block_relative_controls() {
        use crate::entities::rel::Rel;
        let length = Length { abs: Abs(3.0), em: 2.0 };
        assert_eq!(extract_length(&Value::Length(length)), Some(length));
        let relative = Rel { rel: 0.5, abs: length };
        assert_eq!(extract_block_width(&Value::Relative(relative)), Some(relative));
        assert!(extract_length(&Value::Bool(false)).is_none());
        assert!(extract_block_width(&Value::Fraction(1.0)).is_none());
    }
}
