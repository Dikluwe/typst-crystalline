//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/sequence/fractional_preflight.md
//! @prompt-hash 3011ef11
//! @layer L1
//! @updated 2026-10-07
//!
//! Pré-cálculo de blocos fracionários.

use super::inline_prefix::{
    p1380_inline_prefix, paragraph_block_boundary, InlinePrefixRuntime,
};
use super::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum FractionalRegionBasis {
    Inherited,
    CursorRemaining,
}

pub(super) fn fractional_share(remaining: f64, weight: f64, total: f64) -> Option<f64> {
    if !remaining.is_finite() || remaining < 0.0 || !weight.is_finite() || weight < 0.0 {
        return None;
    }
    if total == 0.0 {
        return (weight == 0.0).then_some(0.0);
    }
    (total.is_finite() && total > 0.0).then_some(remaining * weight / total)
}

/// Encontra um único Block fracionário através dos wrappers estruturais que
/// não introduzem siblings de flow. A descida só empresta nós existentes.
pub(super) fn fractional_block(content: &Content) -> Option<&BlockElem> {
    let mut current = content;
    loop {
        current = current.source_body();
        if let Content::Block(block) = current {
            return matches!(block.height, Some(BlockHeight::Fractional(_)))
                .then_some(block);
        }
        if let Content::Styled(styled) = current {
            current = &styled.body;
            continue;
        }
        if let Content::Sequence(parts) = current {
            let mut substantive = parts.iter().filter(|part| {
                !matches!(
                    part.source_body(),
                    Content::Space | Content::SpaceFromNewline | Content::Empty
                )
            });
            let only = substantive.next()?;
            if substantive.next().is_some() {
                return None;
            }
            current = only;
            continue;
        }
        return None;
    }
}

fn block_for_spacing(content: &Content) -> Option<&BlockElem> {
    if let Content::Block(block) = content {
        Some(block)
    } else {
        fractional_block(content)
    }
}

pub(super) fn fractional_region_remaining(
    available_height: f64,
    page_bottom_limit: f64,
    cursor_y: f64,
    continuous_columns: bool,
) -> f64 {
    if continuous_columns {
        // P1379 — recompor parcela + itens fixos não pode ultrapassar o
        // fundo por um ULP e paginar a cauda. A guarda acompanha a escala
        // das coordenadas e permanece muito abaixo da tolerância visual.
        let scale = page_bottom_limit
            .abs()
            .max(cursor_y.abs())
            .max(available_height.abs())
            .max(1.0);
        let roundoff_guard = 8.0 * f64::EPSILON * scale;
        (page_bottom_limit - cursor_y - roundoff_guard).max(0.0)
    } else {
        available_height
    }
}

/// Mede somente a cauda fixa da primeira fatia P1374. O helper não despacha
/// conteúdo e, portanto, não emite itens, páginas ou Locations.
pub(super) fn fractional_preflight<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<'_, M, S>,
    parts: &[Content],
    basis: FractionalRegionBasis,
) -> Option<(f64, f64, Option<InlinePrefixRuntime>)> {
    if layouter.fractional_preflight_active
        || layouter.fractional_block_height.is_some()
        || !layouter.page_config.height.is_finite()
        || !layouter.regions.backlog.is_empty()
        || layouter.regions.last.is_some()
        || layouter.regions.cell.is_some()
        || (layouter.column_mode
            && layouter.page_columns.is_none()
            && basis != FractionalRegionBasis::CursorRemaining)
        || layouter.is_sub_frame
    {
        return None;
    }

    let mut has_fraction = false;
    let total_weight = parts.iter().fold(0.0, |sum, part| {
        let weight = fractional_block(part)
            .and_then(|block| match block.height {
                Some(BlockHeight::Fractional(weight)) => Some(weight),
                _ => None,
            })
            .unwrap_or(0.0);
        if fractional_block(part).is_some() {
            has_fraction = true;
        }
        sum + weight
    });
    if !has_fraction || !total_weight.is_finite() || total_weight < 0.0 {
        return None;
    }

    layouter.fractional_preflight_active = true;
    let font = layouter.style.size.val();
    let paragraph_spacing = match layouter.chain.custom("par.spacing") {
        Some(crate::entities::value::Value::Length(length)) => length.resolve_pt(font),
        _ => font * super::super::vanilla_defaults::PAR_SPACING,
    };
    let max_width = layouter.available_width();
    let mut fixed = 0.0_f64;
    let inline_prefix = if basis == FractionalRegionBasis::CursorRemaining {
        parts
            .iter()
            .position(|part| fractional_block(part).is_some())
            .and_then(|end| p1380_inline_prefix(layouter, parts, end, max_width))
    } else {
        None
    };
    let geometric_prefix = if inline_prefix.is_none() {
        parts
            .iter()
            .position(|part| fractional_block(part).is_some())
            .filter(|&end| end > 0)
            .and_then(|end| {
                let (height, spacing, multiline) =
                    super::super::paragraph::measure_segment(
                        layouter,
                        &parts[..end],
                        max_width,
                        true,
                    )
                    .map(|(h, s)| (h, s, false))
                    .or_else(|| {
                        super::super::paragraph::measure_multiline_prefix(
                            layouter,
                            &parts[..end],
                            max_width,
                        )
                        .map(|(h, s)| (h, s, true))
                    })?;
                if height == 0.0 {
                    return None;
                }
                let block = fractional_block(&parts[end])?;
                let gap = block
                    .above
                    .or(block.spacing)
                    .map(|v| v.resolve_pt(font))
                    .unwrap_or_else(|| spacing.max(paragraph_spacing));
                let prepared_top = if layouter.initial_baseline_pending {
                    0.0
                } else {
                    layouter.metrics.text_edges(layouter.style.size, &layouter.style).0 .0
                };
                Some((end, (height + gap - prepared_top).max(0.0), multiline))
            })
    } else {
        None
    };
    let text_tail = parts
        .iter()
        .rposition(|part| fractional_block(part).is_some())
        .map(|index| index + 1)
        .filter(|&start| start < parts.len())
        .and_then(|start| {
            super::super::paragraph::measure_segment(
                layouter,
                &parts[start..],
                max_width,
                false,
            )
            .map(|(height, spacing)| (start, height, spacing))
        });
    if let Some((_, height, _)) = geometric_prefix {
        fixed += height;
    }
    if let Some(prefix) = inline_prefix {
        fixed += prefix.fixed_height;
        if prefix.closes_line {
            // The prefix measured a prepared text baseline. Block consumes
            // a geometric frame origin, so this top belongs to the free
            // region shared by the fractional blocks, not to the prefix.
            fixed -=
                layouter.metrics.text_edges(layouter.style.size, &layouter.style).0 .0;
        }
    }
    let region_remaining = fractional_region_remaining(
        layouter.available_height(),
        layouter.page_bottom_limit(),
        layouter.regions.current.cursor_y.0,
        basis == FractionalRegionBasis::CursorRemaining
            || (layouter.column_mode && layouter.page_columns.is_some()),
    );
    for (index, part) in parts.iter().enumerate() {
        let part = part.source_body();
        if geometric_prefix.is_some_and(|(end, _, _)| index < end) {
            continue;
        }
        if let Some((start, height, _)) = text_tail {
            if index == start {
                let capacity = (region_remaining - fixed).max(0.0);
                let first_height = if height > capacity {
                    super::super::paragraph::measure_first_fragment(
                        layouter,
                        &parts[start..],
                        max_width,
                        capacity,
                    )
                    .unwrap_or(height)
                } else {
                    height
                };
                fixed += first_height;
            }
            if index >= start {
                continue;
            }
        }
        if inline_prefix.is_some_and(|prefix| index < prefix.end) {
            continue;
        }
        match fractional_block(part) {
            Some(block) => {
                fixed += block.inset.top.resolve_pt(font)
                    + block.inset.bottom.resolve_pt(font)
                    + block.outset.top.resolve_pt(font)
                    + block.outset.bottom.resolve_pt(font);
            }
            _ => {
                let (_, mut height) =
                    layouter.measure_content_constrained(part, max_width);
                // P1379 — texto directo antes de um `fr` fecha uma linha
                // real quando o Block estrutural entra. A medição genérica
                // subestima esse avanço; usar a métrica vertical efectiva.
                let precedes_fraction = matches!(part, Content::Text(_))
                    && parts[index + 1..]
                        .iter()
                        .any(|tail| fractional_block(tail).is_some());
                // A última linha fecha na sua bottom-edge (baseline por omissão),
                // não no avanço até à baseline hipotética seguinte.
                let only_trailing_transparent = parts[index + 1..].iter().all(|tail| {
                    matches!(
                        tail.source_body(),
                        Content::Space | Content::SpaceFromNewline | Content::Empty
                    )
                });
                if precedes_fraction {
                    let (top, bottom) =
                        layouter.metrics.text_edges(layouter.style.size, &layouter.style);
                    let leading = layouter
                        .style
                        .leading
                        .map(|length| length.resolve_pt(layouter.style.size.val()))
                        .unwrap_or_else(|| {
                            layouter.style.size.val()
                                * super::super::vanilla_defaults::PAR_LEADING
                        });
                    let initial_top =
                        if layouter.initial_baseline_pending { top.0 } else { 0.0 };
                    height = initial_top
                        + top.0
                        + (-bottom.0)
                        + leading
                        + paragraph_block_boundary(layouter);
                } else if only_trailing_transparent && matches!(part, Content::Text(_)) {
                    let (top, _) =
                        layouter.metrics.text_edges(layouter.style.size, &layouter.style);
                    let (_, line_height) = layouter
                        .metrics
                        .vertical_metrics(layouter.style.size, &layouter.style);
                    height = (height - line_height.0 + top.0).max(0.0);
                }
                fixed += height;
            }
        }

        let Some(block) = block_for_spacing(part) else { continue };
        if index + 1 >= parts.len() {
            continue;
        }
        let below = block
            .below
            .or(block.spacing)
            .map(|length| length.resolve_pt(font))
            .unwrap_or(paragraph_spacing);
        let gap = match block_for_spacing(&parts[index + 1]) {
            Some(next) => {
                let above = next
                    .above
                    .or(next.spacing)
                    .map(|length| length.resolve_pt(font))
                    .unwrap_or(paragraph_spacing);
                below.max(above)
            }
            _ => text_tail
                .filter(|(start, _, _)| *start == index + 1)
                .filter(|_| block.below.is_none() && block.spacing.is_none())
                .map_or(below, |(_, _, spacing)| below.max(spacing)),
        };
        fixed += gap;
    }
    layouter.fractional_preflight_active = false;

    // A distribuição é relativa à região útil, como o `Region` entregue ao
    // finalizador do flow vanila. O cursor pode já conter leading sintático
    // da fronteira da Sequence; descontá-lo novamente encurta todas as
    // parcelas pelo mesmo valor.
    if (inline_prefix.is_some_and(|prefix| prefix.wrap_before.is_some())
        || geometric_prefix.is_some_and(|(_, _, multiline)| multiline))
        && fixed > region_remaining
    {
        return None;
    }
    Some((
        (region_remaining - fixed).max(0.0),
        total_weight,
        inline_prefix
            .filter(|prefix| prefix.closes_line)
            .map(|prefix| InlinePrefixRuntime {
                geometric: false,
                multiline: false,
                block_index: prefix.end,
                block_boundary: prefix.block_boundary,
                wrap_before: prefix.wrap_before,
                first_fragment_overlong: prefix.first_fragment_overlong,
                second_smaller_baseline_adjustment: prefix
                    .second_smaller_baseline_adjustment,
            })
            .or_else(|| {
                geometric_prefix.map(|(end, _, multiline)| InlinePrefixRuntime {
                    geometric: true,
                    multiline,
                    block_index: end,
                    block_boundary: 0.0,
                    wrap_before: None,
                    first_fragment_overlong: false,
                    second_smaller_baseline_adjustment: 0.0,
                })
            }),
    ))
}

#[cfg(test)]
mod smoke {
    #[test]
    fn modulo_compila_e_carrega() {}
}
