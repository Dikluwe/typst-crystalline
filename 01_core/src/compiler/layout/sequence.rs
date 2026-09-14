//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/sequence.md
//! @prompt-hash 332a437b
//! @layer L1
//! @updated 2026-08-20
//!
//! Layout de `Content::Sequence` — iteração peekable com sticky
//! lookahead e spacing collapse. Extraído de `layout/mod.rs` no P425
//! (ADR-0109 forma B).

use crate::entities::elements::block::BlockElem;
use crate::entities::elements::block::BlockHeight;
use crate::entities::{content::Content, image_sizer::ImageSizer, layout_types::Pt};

use super::metrics::FontMetrics;
use super::{ItemGroup, Layouter};

#[derive(Clone, Copy, PartialEq, Eq)]
enum FractionalRegionBasis {
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
fn fractional_block(content: &Content) -> Option<&BlockElem> {
    let mut current = content;
    loop {
        if let Content::Block(block) = current {
            return matches!(block.height, Some(BlockHeight::Fractional(_)))
                .then_some(block);
        }
        if let Content::Styled(body, _) = current {
            current = body;
            continue;
        }
        if let Content::Sequence(parts) = current {
            let mut substantive = parts
                .iter()
                .filter(|part| !matches!(part, Content::Space | Content::Empty));
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

fn paragraph_block_boundary<M: FontMetrics, S: ImageSizer>(
    layouter: &Layouter<'_, M, S>,
) -> f64 {
    let font = layouter.style.size.val();
    let spacing = paragraph_spacing(layouter);
    let leading = match layouter.chain.custom("par.leading") {
        Some(crate::entities::value::Value::Length(length)) => length.resolve_pt(font),
        _ => layouter
            .style
            .leading
            .map(|length| length.resolve_pt(font))
            .unwrap_or(font * super::vanilla_defaults::PAR_LEADING),
    };
    (spacing - leading).max(0.0)
}

fn paragraph_spacing<M: FontMetrics, S: ImageSizer>(
    layouter: &Layouter<'_, M, S>,
) -> f64 {
    let font = layouter.style.size.val();
    match layouter.chain.custom("par.spacing") {
        Some(crate::entities::value::Value::Length(length)) => length.resolve_pt(font),
        _ => font * super::vanilla_defaults::PAR_SPACING,
    }
}

#[derive(Clone, Copy)]
struct InlinePrefix {
    end: usize,
    fixed_height: f64,
    closes_line: bool,
    block_boundary: f64,
    wrap_before: Option<usize>,
    first_fragment_overlong: bool,
}

#[derive(Clone, Copy)]
struct InlinePrefixRuntime {
    block_index: usize,
    block_boundary: f64,
    wrap_before: Option<usize>,
    first_fragment_overlong: bool,
}

fn p1380_inline_safe_styles(styles: &crate::entities::style::Styles) -> bool {
    let delta = styles.delta();
    delta.heading_level.is_none()
        && delta.tracking.is_none()
        && delta.leading.is_none()
        && delta.top_edge.is_none()
        && delta.bottom_edge.is_none()
        && delta.font.is_none()
        && delta.subscript.is_none()
        && delta.superscript.is_none()
        && delta.highlight.is_none()
        && delta.highlight_radius.is_none()
        && delta.highlight_extent.is_none()
        && delta.subscript_size.is_none()
        && delta.superscript_size.is_none()
        && delta.custom.is_empty()
}

/// P1383 — reconhece um delta que altera exclusivamente `text.size`.
/// Comparar com o delta vazio evita uma allowlist parcial que aceitaria
/// silenciosamente um campo novo no futuro.
fn p1383_size_only_styles(styles: &crate::entities::style::Styles) -> bool {
    let delta = styles.delta();
    let mut size_only = crate::entities::style_chain::StyleDelta::empty();
    size_only.size = delta.size;
    delta.size.is_some() && delta == &size_only
}

fn p1380_is_unwrapped(width: f64, max_width: f64) -> bool {
    width.is_finite() && max_width.is_finite() && width <= max_width
}

fn p1389_single_word(text: &str) -> bool {
    !text.is_empty() && text.chars().all(|character| !character.is_whitespace())
}

fn p1380_text_fragment<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<'_, M, S>,
    text: &str,
    styles: Option<&crate::entities::style::Styles>,
) -> Option<(f64, Option<(f64, crate::entities::layout_types::TextStyle)>)> {
    if styles.is_some_and(|styles| !p1380_inline_safe_styles(styles)) {
        return None;
    }

    let previous_chain = layouter.chain.clone();
    let previous_style = layouter.style.clone();
    if let Some(styles) = styles {
        layouter.chain = layouter.chain.push_styles(styles);
        layouter.style = crate::entities::layout_types::TextStyle::from(&layouter.chain);
    }

    let mut width = 0.0;
    let mut has_text = false;
    let mut pieces = text.split(' ').peekable();
    while let Some(piece) = pieces.next() {
        if !piece.is_empty() {
            width += layouter
                .metrics
                .text_width(piece, layouter.style.size, &layouter.style)
                .0;
            has_text = true;
        }
        if pieces.peek().is_some() {
            width += layouter.space_width().0;
        }
    }
    let text_metrics = has_text.then(|| {
        (
            layouter.metrics.text_edges(layouter.style.size, &layouter.style).0 .0,
            layouter.style.clone(),
        )
    });

    layouter.chain = previous_chain;
    layouter.style = previous_style;
    Some((width, text_metrics))
}

/// P1380 — reconhece uma única linha inline anterior ao primeiro `fr` sem
/// reconstruir conteúdo. O único estado proporcional é a própria fatia já
/// existente; wrappers aceitos têm profundidade exactamente um.
fn p1380_inline_prefix<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<'_, M, S>,
    parts: &[Content],
    end: usize,
    max_width: f64,
) -> Option<InlinePrefix> {
    let mut width = 0.0;
    let mut first_top = None;
    let mut has_text = false;
    // P1381 — espelha os acumuladores escalares de `flush_line`: o maior
    // estilo parte do exterior, enquanto o leading pertence ao último texto.
    let mut max_style = layouter.style.clone();
    let mut first_style = None;
    let mut last_style = None;
    let outer_size = layouter.style.size.0;
    let mut text_fragments = 0usize;
    let mut size_changed = false;
    let mut first_size = None;
    let mut last_size = None;
    let mut first_fragment_direct = false;
    let mut first_fragment_size_only = false;
    let mut second_fragment_direct = false;
    let mut second_fragment_size_only = false;
    let mut all_fragments_single_word = true;
    let mut wrap_before = None;
    // P1391 — uma primeira palavra size-only maior pode ser overlong para a
    // coluna e ainda assim formar uma linha válida antes do segundo wrapper.
    // O adiamento só é legitimado no veredito final pela célula exterior/
    // maior; as demais formas continuam Unknown.
    let mut first_fragment_overlong = false;

    for (part_index, part) in parts[..end].iter().enumerate() {
        let (fragment, fragment_single_word) = match part {
            Content::Text(text) => {
                if text_fragments == 0 {
                    first_fragment_direct = true;
                } else if text_fragments == 1 {
                    second_fragment_direct = true;
                }
                (p1380_text_fragment(layouter, text, None), p1389_single_word(text))
            }
            Content::Styled(body, styles) => match body.as_ref() {
                Content::Text(text) => {
                    if text_fragments == 0 && p1383_size_only_styles(styles) {
                        first_fragment_size_only = true;
                    }
                    if text_fragments == 1 && p1383_size_only_styles(styles) {
                        second_fragment_size_only = true;
                    }
                    (
                        p1380_text_fragment(layouter, text, Some(styles)),
                        p1389_single_word(text),
                    )
                }
                _ => return None,
            },
            Content::Strong(strong) => match &strong.body {
                Content::Text(text) => {
                    let styles = crate::entities::style::Styles::from_iter([
                        crate::entities::style::Style::strong(),
                    ]);
                    (
                        p1380_text_fragment(layouter, text, Some(&styles)),
                        p1389_single_word(text),
                    )
                }
                _ => return None,
            },
            Content::Emph(emph) => match &emph.body {
                Content::Text(text) => {
                    let styles = crate::entities::style::Styles::from_iter([
                        crate::entities::style::Style::emph(),
                    ]);
                    (
                        p1380_text_fragment(layouter, text, Some(&styles)),
                        p1389_single_word(text),
                    )
                }
                _ => return None,
            },
            Content::Space => {
                if has_text {
                    width += layouter.space_width().0;
                }
                continue;
            }
            Content::Empty => continue,
            _ => return None,
        };
        let fragment = fragment?;
        let width_before_fragment = width;
        let had_text_before_fragment = has_text;
        width += fragment.0;
        let fragment_has_text = fragment.1.is_some();
        has_text |= fragment_has_text;
        if fragment_has_text {
            all_fragments_single_word &= fragment_single_word;
        }
        if let Some((top, style)) = fragment.1 {
            text_fragments += 1;
            first_size.get_or_insert(style.size.0);
            if first_style.is_none() {
                first_style = Some(style.clone());
            }
            last_size = Some(style.size.0);
            size_changed |= style.size.0 != outer_size;
            if first_top.is_none() {
                first_top = Some(top);
            }
            if style.size.0 > max_style.size.0 {
                max_style = style.clone();
            }
            last_style = Some(style);
        }
        if !p1380_is_unwrapped(width, max_width) {
            let wraps_between_fragments = wrap_before.is_none()
                && had_text_before_fragment
                && fragment_has_text
                && (p1380_is_unwrapped(width_before_fragment, max_width)
                    || first_fragment_overlong)
                && all_fragments_single_word;
            if wraps_between_fragments {
                wrap_before = Some(part_index);
                width = fragment.0;
            } else if text_fragments == 1
                && first_fragment_size_only
                && fragment_single_word
                && first_size.is_some_and(|size| size > outer_size)
            {
                first_fragment_overlong = true;
            } else {
                return None;
            }
        }
    }

    if !has_text {
        return Some(InlinePrefix {
            end,
            fixed_height: 0.0,
            closes_line: false,
            block_boundary: 0.0,
            wrap_before: None,
            first_fragment_overlong: false,
        });
    }
    // P1383 — P1382 tornou decidível somente a forma crescente exacta. Não
    // inferir daqui paridade para ordem inversa, terceiro fragmento ou delta
    // composto: todos continuam no fallback opaco.
    let p1383_outer_then_larger = text_fragments == 2
        && first_fragment_direct
        && second_fragment_size_only
        && first_size == Some(outer_size)
        && last_size.is_some_and(|size| size > outer_size);
    // P1384 — a ordem inversa só é transparente quando o primeiro delta é
    // exclusivamente size e o segundo fragmento retorna directamente ao
    // estilo exterior. Adiar a rejeição do primeiro maior permite observar
    // a forma completa sem admitir o singleton P1381 nem dois wrappers.
    let p1384_larger_then_outer = text_fragments == 2
        && first_fragment_size_only
        && second_fragment_direct
        && first_size.is_some_and(|size| size > outer_size)
        && last_size == Some(outer_size);
    // P1385 — o segundo wrapper só compartilha a fórmula P1384 quando retorna
    // ao tamanho exterior ou abaixo dele. Acima do exterior, a calibração
    // encontrou uma fronteira adicional ainda não modelada e mantém Unknown.
    let p1385_styled_pair_to_outer_or_smaller = text_fragments == 2
        && first_fragment_size_only
        && second_fragment_size_only
        && first_size.is_some_and(|size| size > outer_size)
        && last_size.is_some_and(|size| size <= outer_size);
    // P1386 — quando ambos os wrappers permanecem acima do exterior, a
    // forma só é transparente sob descida estrita. A boundary exterior é
    // consumida pelo runtime, mas não pertence novamente à reserva fixa.
    let p1386_decreasing_pair_above_outer = text_fragments == 2
        && first_fragment_size_only
        && second_fragment_size_only
        && first_size.is_some_and(|size| size > outer_size)
        && last_size.is_some_and(|size| size > outer_size)
        && first_size.zip(last_size).is_some_and(|(first, last)| first > last);
    // P1387 — wrappers uniformes ainda são mistos em relação ao estilo
    // exterior. Cursor usa o exterior para o próximo ascent/leading; a
    // reserva é, portanto, a mesma forma escalar P1386.
    let p1387_equal_pair_above_outer = text_fragments == 2
        && first_fragment_size_only
        && second_fragment_size_only
        && first_size.is_some_and(|size| size > outer_size)
        && first_size == last_size;
    // P1390 — a forma desigual só é adicionada quando o wrap já foi provado
    // na fronteira entre os dois fragmentos. Isso não amplia a célula
    // crescente sem wrap, que continua opaca.
    let p1390_unequal_wrapped_pair_above_outer = wrap_before.is_some()
        && !first_fragment_overlong
        && text_fragments == 2
        && first_fragment_size_only
        && second_fragment_size_only
        && first_size.is_some_and(|size| size > outer_size)
        && last_size.is_some_and(|size| size > outer_size)
        && first_size != last_size;
    // P1391 — fronteira exacta: um wrapper herda numericamente o tamanho
    // exterior e o outro é maior. `<=` aqui alargaria indevidamente a célula
    // para tamanhos menores, que permanecem opacos.
    let p1391_wrapped_outer_and_larger = wrap_before.is_some()
        && text_fragments == 2
        && first_fragment_size_only
        && second_fragment_size_only
        && first_size.zip(last_size).is_some_and(|(first, last)| {
            (first == outer_size && last > outer_size)
                || (first > outer_size && last == outer_size)
        });
    if first_fragment_overlong && !p1391_wrapped_outer_and_larger {
        return None;
    }
    if wrap_before.is_some()
        && (!(p1387_equal_pair_above_outer
            || p1390_unequal_wrapped_pair_above_outer
            || p1391_wrapped_outer_and_larger)
            || !all_fragments_single_word)
    {
        return None;
    }
    if first_size.is_some_and(|size| size > outer_size)
        && !p1384_larger_then_outer
        && !p1385_styled_pair_to_outer_or_smaller
        && !p1386_decreasing_pair_above_outer
        && !p1387_equal_pair_above_outer
        && !p1390_unequal_wrapped_pair_above_outer
        && !p1391_wrapped_outer_and_larger
    {
        return None;
    }
    if size_changed
        && text_fragments != 1
        && !p1383_outer_then_larger
        && !p1384_larger_then_outer
        && !p1385_styled_pair_to_outer_or_smaller
        && !p1386_decreasing_pair_above_outer
        && !p1387_equal_pair_above_outer
        && !p1390_unequal_wrapped_pair_above_outer
        && !p1391_wrapped_outer_and_larger
    {
        return None;
    }

    let (top, bottom) = layouter.metrics.text_edges(max_style.size, &max_style);
    let last_style = last_style.as_ref().unwrap_or(&layouter.style);
    // P1383 — o Cursor P1382 fecha a linha mista usando o estilo activo
    // exterior para prever a linha seguinte. Leading e boundary precisam
    // formar o mesmo par aqui; misturar estilos conserva a soma reservada,
    // mas entrega uma boundary curta ao runtime.
    let leading_style = if p1383_outer_then_larger
        || p1384_larger_then_outer
        || p1385_styled_pair_to_outer_or_smaller
        || p1386_decreasing_pair_above_outer
        || p1387_equal_pair_above_outer
        || p1390_unequal_wrapped_pair_above_outer
        || p1391_wrapped_outer_and_larger
    {
        &layouter.style
    } else {
        last_style
    };
    let leading = leading_style
        .leading
        .map(|length| length.resolve_pt(leading_style.size.val()))
        .unwrap_or_else(|| {
            leading_style.size.val() * super::vanilla_defaults::PAR_LEADING
        });
    // P1384 — o maior ascent já está representado por `top`; repetir aqui o
    // top-edge do primeiro fragmento 18pt contaria esse crescimento duas
    // vezes. O avanço que P1382 prevê para a linha seguinte usa o exterior.
    let initial_top = if layouter.initial_baseline_pending {
        if p1384_larger_then_outer
            || p1385_styled_pair_to_outer_or_smaller
            || p1386_decreasing_pair_above_outer
            || p1387_equal_pair_above_outer
            || p1390_unequal_wrapped_pair_above_outer
            || p1391_wrapped_outer_and_larger
        {
            layouter.metrics.text_edges(layouter.style.size, &layouter.style).0 .0
        } else {
            first_top.unwrap_or(0.0)
        }
    } else {
        0.0
    };
    let block_boundary = (paragraph_spacing(layouter) - leading).max(0.0);
    let fixed_boundary = if p1386_decreasing_pair_above_outer
        || p1387_equal_pair_above_outer
        || p1390_unequal_wrapped_pair_above_outer
        || p1391_wrapped_outer_and_larger
    {
        0.0
    } else {
        block_boundary
    };
    let fixed_height = if wrap_before.is_some() {
        // P1390 — cada singleton tem métricas próprias. P1389 era igual e
        // podia reutilizar o máximo; fazê-lo para tamanhos desiguais
        // superestima uma das linhas e encurta a parcela fracionária.
        let first_style = first_style.as_ref()?;
        let second_style = last_style;
        let (first_line_top, first_line_bottom) =
            layouter.metrics.text_edges(first_style.size, first_style);
        let (second_line_top, second_line_bottom) =
            layouter.metrics.text_edges(second_style.size, second_style);
        initial_top
            + first_line_top.0
            + (-first_line_bottom.0)
            + leading
            + second_line_top.0
            + (-second_line_bottom.0)
            + leading
            + fixed_boundary
    } else {
        initial_top + top.0 + (-bottom.0) + leading + fixed_boundary
    };
    Some(InlinePrefix {
        end,
        fixed_height,
        closes_line: true,
        block_boundary,
        wrap_before,
        first_fragment_overlong,
    })
}

fn fractional_region_remaining(
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
fn fractional_preflight<M: FontMetrics, S: ImageSizer>(
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
    if let Some(prefix) = inline_prefix {
        fixed += prefix.fixed_height;
    }
    for (index, part) in parts.iter().enumerate() {
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
                let only_trailing_transparent = parts[index + 1..]
                    .iter()
                    .all(|tail| matches!(tail, Content::Space | Content::Empty));
                if precedes_fraction {
                    let (top, bottom) =
                        layouter.metrics.text_edges(layouter.style.size, &layouter.style);
                    let leading = layouter
                        .style
                        .leading
                        .map(|length| length.resolve_pt(layouter.style.size.val()))
                        .unwrap_or_else(|| {
                            layouter.style.size.val()
                                * super::vanilla_defaults::PAR_LEADING
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
            .unwrap_or(font * super::vanilla_defaults::PAR_SPACING);
        let gap = match block_for_spacing(&parts[index + 1]) {
            Some(next) => {
                let above = next
                    .above
                    .or(next.spacing)
                    .map(|length| length.resolve_pt(font))
                    .unwrap_or(font * super::vanilla_defaults::PAR_SPACING);
                below.max(above)
            }
            _ => below,
        };
        fixed += gap;
    }
    layouter.fractional_preflight_active = false;

    // A distribuição é relativa à região útil, como o `Region` entregue ao
    // finalizador do flow vanila. O cursor pode já conter leading sintático
    // da fronteira da Sequence; descontá-lo novamente encurta todas as
    // parcelas pelo mesmo valor.
    let region_remaining = fractional_region_remaining(
        layouter.available_height(),
        layouter.page_bottom_limit(),
        layouter.regions.current.cursor_y.0,
        basis == FractionalRegionBasis::CursorRemaining
            || (layouter.column_mode && layouter.page_columns.is_some()),
    );
    if inline_prefix.is_some_and(|prefix| prefix.wrap_before.is_some())
        && fixed > region_remaining
    {
        return None;
    }
    Some((
        (region_remaining - fixed).max(0.0),
        total_weight,
        inline_prefix.filter(|prefix| prefix.closes_line).map(|prefix| {
            InlinePrefixRuntime {
                block_index: prefix.end,
                block_boundary: prefix.block_boundary,
                wrap_before: prefix.wrap_before,
                first_fragment_overlong: prefix.first_fragment_overlong,
            }
        }),
    ))
}

/// Layout de uma sequência de conteúdos.
///
/// P250 — refactor Sequence consumer para peekable + neighbour context.
/// Permite:
/// - Sticky lookahead (Block.sticky=true detecta next; se combined_h >
///   remaining + cabe em página inteira → new_page() antes do block).
/// - Spacing collapse via Layouter fields `prev_block_below_pending` +
///   `block_chain_active`.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<'_, M, S>,
    parts: &[Content],
) {
    layout_parts(layouter, parts, FractionalRegionBasis::Inherited);
}

/// P1379 — entrada exclusiva para uma região irmã criada por `colbreak()`.
/// O próprio content é emprestado: Sequence usa a fatia existente, singleton
/// usa `from_ref` e vazio não aloca.
pub(super) fn layout_segmented_region<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<'_, M, S>,
    content: &Content,
) {
    if let Content::Styled(body, styles) = content {
        let previous_chain = layouter.chain.clone();
        let previous_style = layouter.style.clone();
        layouter.chain = layouter.chain.push_styles(styles);
        layouter.style = crate::entities::layout_types::TextStyle::from(&layouter.chain);
        layout_segmented_unstyled(layouter, body);
        layouter.chain = previous_chain;
        layouter.style = previous_style;
        return;
    }
    layout_segmented_unstyled(layouter, content);
}

fn layout_segmented_unstyled<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<'_, M, S>,
    content: &Content,
) {
    match content {
        Content::Sequence(parts) => {
            layout_parts(layouter, parts, FractionalRegionBasis::CursorRemaining)
        }
        Content::Empty => {
            layout_parts(layouter, &[], FractionalRegionBasis::CursorRemaining)
        }
        other => layout_parts(
            layouter,
            std::slice::from_ref(other),
            FractionalRegionBasis::CursorRemaining,
        ),
    }
}

fn layout_parts<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<'_, M, S>,
    parts: &[Content],
    basis: FractionalRegionBasis,
) {
    let saved_below = layouter.prev_block_below_pending;
    let saved_chain = layouter.block_chain_active;
    let saved_descent = layouter.prev_block_equation_descent;
    let saved_fractional_height = layouter.fractional_block_height;
    let distribution = fractional_preflight(layouter, parts, basis);
    let terminal_flow_index = parts
        .iter()
        .rposition(|part| !matches!(part, Content::Space | Content::Empty));

    let mut iter = parts.iter().peekable();
    let mut part_index = 0usize;
    while let Some(part) = iter.next() {
        // **P864** — detetar `Content::Parbreak` entre itens do mesmo grupo.
        // Se o item anterior (mesmo tipo) foi separado por um Parbreak,
        // aplica o espaçamento de parágrafo entre grupos e reinicia o estado
        // de agrupamento de *espaçamento* (`last_was_loose_item`).
        //
        // **P1016** — o `enum_counter` NÃO é reiniciado aqui. Medido no
        // vanilla: `+ a\n+ b\n\n+ c` numera `1. 2. 3.` — a numeração
        // atravessa a linha em branco. Só conteúdo real termina a lista.
        if let Some(group) = item_group(part) {
            if layouter.parbreak_since_last_item
                && layouter.last_seen_item_group == Some(group)
            {
                layouter.regions.current.cursor_y += paragraph_advance(layouter);
                layouter.last_was_loose_item = false;
            }
            layouter.last_seen_item_group = Some(group);
            layouter.parbreak_since_last_item = false;
        } else if matches!(part, Content::Parbreak) {
            // Marca que um Parbreak ocorreu desde o último item, desde que
            // ainda estejamos num contexto de agrupamento válido.
            if layouter.last_seen_item_group.is_some() {
                layouter.parbreak_since_last_item = true;
            }
        } else if !matches!(part, Content::Space | Content::Empty | Content::Styled(..)) {
            // Qualquer outro conteúdo real quebra a consecutividade.
            layouter.last_seen_item_group = None;
            layouter.parbreak_since_last_item = false;
        }

        // P250 — sticky pre-layout lookahead 1-block.
        if matches!(part, Content::Block(e) if e.sticky) {
            if let Some(next) = iter.peek() {
                let avail_w = layouter.available_width();
                let (_, part_h) = layouter.measure_content_constrained(part, avail_w);
                let (_, next_h) = layouter.measure_content_constrained(next, avail_w);
                let combined = part_h + next_h;
                let remaining =
                    layouter.page_bottom_limit() - layouter.regions.current.cursor_y.0;
                let page_usable = layouter.available_height();
                if combined > remaining && combined <= page_usable {
                    layouter.new_page();
                }
                // else: cabem ambos OU overlong → emit normal.
            }
        }

        let inline_prefix = distribution.and_then(|(_, _, inline_prefix)| inline_prefix);
        if part_index == 0
            && inline_prefix.is_some_and(|prefix| prefix.first_fragment_overlong)
        {
            // P1391 — a palavra overlong fecha-se dentro do próprio layout de
            // texto, antes de Sequence alcançar `wrap_before`. Armar o próximo
            // flush preserva o ascent/leading exterior nesse fechamento único.
            layouter.arm_wrapped_inline_prefix_line();
        }
        if inline_prefix.is_some_and(|prefix| prefix.wrap_before == Some(part_index))
            && !layouter.regions.current.current_line.is_empty()
        {
            layouter.flush_wrapped_inline_prefix_line();
        }
        if inline_prefix.is_some_and(|prefix| {
            prefix.wrap_before.is_some_and(|index| part_index > index)
                && part_index < prefix.block_index
                && matches!(part, Content::Space | Content::Empty)
        }) && !layouter.regions.current.current_line.is_empty()
        {
            layouter.flush_wrapped_inline_prefix_line();
        }
        if inline_prefix.is_some_and(|prefix| prefix.block_index == part_index) {
            // P1388 — o whitespace terminal pode já ter fechado a linha.
            // A boundary pertence ao prefixo certificado, não ao estado
            // incidental de `current_line`, e deve ser aplicada uma vez.
            if !layouter.regions.current.current_line.is_empty() {
                if inline_prefix.is_some_and(|prefix| prefix.wrap_before.is_some()) {
                    layouter.flush_wrapped_inline_prefix_line();
                } else {
                    layouter.flush_line();
                }
            }
            layouter.regions.current.cursor_y +=
                Pt(inline_prefix.map(|prefix| prefix.block_boundary).unwrap_or(0.0));
        }

        layouter.fractional_block_height = match (distribution, fractional_block(part)) {
            (Some((remaining, total, _)), Some(block)) => match block.height {
                Some(BlockHeight::Fractional(weight)) => {
                    fractional_share(remaining, weight, total)
                }
                _ => None,
            },
            (None, _) => saved_fractional_height,
            _ => None,
        };
        if distribution.is_some() && Some(part_index) == terminal_flow_index {
            layouter.fractional_flow_tail_pending = true;
        }
        layouter.layout_content(part);
        layouter.fractional_block_height = saved_fractional_height;

        // **P1107** — preservação da cadeia de colapso de margens:
        // Apenas elementos de bloco (Block, Shape, Parbreak, Heading, Equation)
        // e nós transparentes (Space, Empty quando fora de linha aberta)
        // mantêm o estado de colapso de margens.
        // **P1107/P1108** — Preservação da cadeia de colapso de margens:
        // A cadeia só é interrompida quando uma linha de texto inline aberta é populada.
        if !layouter.regions.current.current_line.is_empty() {
            layouter.block_chain_active = false;
            layouter.prev_block_below_pending = 0.0;
        }

        // P505 — apenas ListItem/EnumItem consecutivos com tight=false
        // partilham o espaçamento de parágrafo; qualquer outro conteúdo
        // reseta o estado.
        if !matches!(part, Content::ListItem(_) | Content::EnumItem(_)) {
            layouter.last_was_loose_item = false;
        }
        if !matches!(
            part,
            Content::EnumItem(_)
                | Content::Space
                | Content::Parbreak
                | Content::Empty
                | Content::Styled(..)
        ) {
            layouter.enum_counter = None;
        }
        part_index += 1;
    }

    // Preservar estado da cadeia anterior se a sequência foi composta exclusivamente por nós transparentes/estilos
    let is_transparent = parts.iter().all(|p| {
        matches!(
            p,
            Content::Empty | Content::Styled(..) | Content::Space | Content::Parbreak
        )
    });
    if is_transparent && saved_chain {
        layouter.prev_block_below_pending = saved_below;
        layouter.block_chain_active = saved_chain;
        layouter.prev_block_equation_descent = saved_descent;
    }
    if distribution.is_some() && !layouter.regions.current.current_line.is_empty() {
        layouter.fractional_flow_tail_pending = true;
    }
    layouter.fractional_block_height = saved_fractional_height;
}

/// **P864** — devolve o grupo de item estrutural reconhecido, se aplicável.
fn item_group(content: &Content) -> Option<ItemGroup> {
    match content {
        Content::ListItem(_) => Some(ItemGroup::List),
        Content::EnumItem(_) => Some(ItemGroup::Enum),
        Content::TermItem(_) => Some(ItemGroup::Terms),
        _ => None, // neutro: N16[β] — Content atómico sem sub-sequência retorna None (projeção de container)
    }
}

/// **P864** — avanço vertical de parágrafo (top-edge + |bottom-edge| +
/// leading default), usado como espaçamento entre grupos separados por
/// `Content::Parbreak`.
fn paragraph_advance<M: FontMetrics, S: ImageSizer>(layouter: &Layouter<M, S>) -> Pt {
    let font_size = layouter.style.size;
    let (top, bottom) = layouter.metrics.text_edges(font_size, &layouter.style);
    let leading = layouter
        .style
        .leading
        .map(|l| l.resolve_pt(font_size.val()))
        // PAR_LEADING, ver vanilla_defaults.rs
        .unwrap_or_else(|| font_size.val() * super::vanilla_defaults::PAR_LEADING);
    top + Pt(-bottom.0) + Pt(leading)
}

#[cfg(test)]
mod p1374_tests {
    use super::*;

    #[test]
    fn p1374_fractional_share_is_proportional_and_zero_safe() {
        assert_eq!(fractional_share(186.0, 1.0, 3.0), Some(62.0));
        assert_eq!(fractional_share(186.0, 2.0, 3.0), Some(124.0));
        assert_eq!(fractional_share(186.0, 0.0, 0.0), Some(0.0));
        assert_eq!(fractional_share(186.0, 1.0, 0.0), None);
    }
}

#[cfg(test)]
mod p1375_tests {
    use std::sync::Arc;

    use crate::entities::{
        content::Content,
        corners::Corners,
        elements::block::{BlockElem, BlockHeight},
        layout_types::Length,
        sides::Sides,
        style::Styles,
    };

    use super::fractional_block;

    fn fractional(weight: f64) -> Content {
        Content::Block(Arc::new(BlockElem {
            body: Content::text("body"),
            width: None,
            height: Some(BlockHeight::Fractional(weight)),
            inset: Sides::uniform(Length::ZERO),
            breakable: true,
            outset: Sides::uniform(Length::ZERO),
            radius: Corners::uniform(Length::ZERO),
            clip: false,
            fill: None,
            stroke: None,
            spacing: None,
            above: None,
            below: None,
            sticky: false,
        }))
    }

    #[test]
    fn p1375_styled_singleton_exposes_fractional_block() {
        let wrapped = Content::Styled(
            Box::new(Content::sequence(vec![Content::Space, fractional(1.0)])),
            Styles::new(),
        );
        assert!(matches!(
            fractional_block(&wrapped).and_then(|block| block.height),
            Some(BlockHeight::Fractional(1.0))
        ));
    }

    #[test]
    fn p1375_styled_ambiguous_sequence_fails_closed() {
        let wrapped = Content::Styled(
            Box::new(Content::sequence(vec![fractional(1.0), fractional(2.0)])),
            Styles::new(),
        );
        assert!(fractional_block(&wrapped).is_none());
    }
}

#[cfg(test)]
mod p1376_tests {
    use super::fractional_region_remaining;

    #[test]
    fn p1376_continuous_column_uses_cursor_relative_region() {
        assert!(
            (fractional_region_remaining(220.0, 230.0, 10.0, true) - 220.0).abs() < 1e-9
        );
        assert!(
            (fractional_region_remaining(220.0, 230.0, 30.438, true) - 199.562).abs()
                < 1e-9
        );
        assert_eq!(fractional_region_remaining(220.0, 230.0, 30.438, false), 220.0);
    }
}

#[cfg(test)]
mod p1380_tests {
    use crate::entities::{
        layout_types::Pt,
        style::{Style, Styles},
    };

    use super::{p1380_inline_safe_styles, p1380_is_unwrapped, p1389_single_word};

    #[test]
    fn p1380_wrap_permanece_opaco_e_p1381_delega_size_ao_prefixo() {
        assert!(p1380_is_unwrapped(85.0, 85.0));
        assert!(!p1380_is_unwrapped(85.001, 85.0));
        assert!(!p1380_is_unwrapped(f64::INFINITY, 85.0));

        assert!(p1380_inline_safe_styles(&Styles::from_iter([Style::strong()])));
        assert!(p1380_inline_safe_styles(&Styles::from_iter([Style::Size(Pt(8.0))])));
    }

    #[test]
    fn p1389_word_unico_rejeita_whitespace_interno() {
        assert!(p1389_single_word("More"));
        assert!(!p1389_single_word("More text"));
        assert!(!p1389_single_word("More\ntext"));
        assert!(!p1389_single_word(""));
    }
}
