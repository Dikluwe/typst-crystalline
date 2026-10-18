//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/sequence/inline_prefix.md
//! @prompt-hash 2352a285
//! @layer L1
//! @updated 2026-10-07
//!
//! Certificação de prefixos inline.

use super::*;

pub(super) fn paragraph_block_boundary<M: FontMetrics, S: ImageSizer>(
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
            .unwrap_or(font * super::super::vanilla_defaults::PAR_LEADING),
    };
    (spacing - leading).max(0.0)
}

fn paragraph_spacing<M: FontMetrics, S: ImageSizer>(
    layouter: &Layouter<'_, M, S>,
) -> f64 {
    let font = layouter.style.size.val();
    match layouter.chain.custom("par.spacing") {
        Some(crate::entities::value::Value::Length(length)) => length.resolve_pt(font),
        _ => font * super::super::vanilla_defaults::PAR_SPACING,
    }
}

#[derive(Clone, Copy)]
pub(super) struct InlinePrefix {
    pub(super) end: usize,
    pub(super) fixed_height: f64,
    pub(super) closes_line: bool,
    pub(super) block_boundary: f64,
    pub(super) wrap_before: Option<usize>,
    pub(super) first_fragment_overlong: bool,
    pub(super) second_smaller_baseline_adjustment: f64,
}

#[derive(Clone, Copy)]
pub(super) struct InlinePrefixRuntime {
    pub(super) geometric: bool,
    pub(super) multiline: bool,
    pub(super) block_index: usize,
    pub(super) block_boundary: f64,
    pub(super) wrap_before: Option<usize>,
    pub(super) first_fragment_overlong: bool,
    pub(super) second_smaller_baseline_adjustment: f64,
}

pub(super) fn p1380_inline_safe_styles(styles: &crate::entities::style::Styles) -> bool {
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

pub(super) fn p1380_is_unwrapped(width: f64, max_width: f64) -> bool {
    width.is_finite() && max_width.is_finite() && width <= max_width
}

pub(super) fn p1389_single_word(text: &str) -> bool {
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
pub(super) fn p1380_inline_prefix<M: FontMetrics, S: ImageSizer>(
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
        let (fragment, fragment_single_word) =
            match super::super::paragraph::prepared_text_body(part) {
                Content::Text(text) => {
                    if text_fragments == 0 {
                        first_fragment_direct = true;
                    } else if text_fragments == 1 {
                        second_fragment_direct = true;
                    }
                    (p1380_text_fragment(layouter, text, None), p1389_single_word(text))
                }
                Content::Styled(styled) => {
                    match super::super::paragraph::prepared_text_body(&styled.body) {
                        Content::Text(text) => {
                            if text_fragments == 0
                                && p1383_size_only_styles(&styled.styles)
                            {
                                first_fragment_size_only = true;
                            }
                            if text_fragments == 1
                                && p1383_size_only_styles(&styled.styles)
                            {
                                second_fragment_size_only = true;
                            }
                            (
                                p1380_text_fragment(layouter, text, Some(&styled.styles)),
                                p1389_single_word(text),
                            )
                        }
                        _ => return None,
                    }
                }
                Content::Strong(strong) => {
                    match super::super::paragraph::prepared_text_body(&strong.body) {
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
                    }
                }
                Content::Emph(emph) => {
                    match super::super::paragraph::prepared_text_body(&emph.body) {
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
                    }
                }
                Content::Space | Content::SpaceFromNewline => {
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
            second_smaller_baseline_adjustment: 0.0,
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
    // P1392 — os dois wrappers atravessam estritamente o tamanho exterior.
    // Igualdade continua sob P1391; duas pontas no mesmo lado permanecem nos
    // casos confirmados anteriores ou no fallback opaco.
    let p1392_wrapped_below_and_above_outer = wrap_before.is_some()
        && text_fragments == 2
        && first_fragment_size_only
        && second_fragment_size_only
        && first_size.zip(last_size).is_some_and(|(first, last)| {
            (first < outer_size && last > outer_size)
                || (first > outer_size && last < outer_size)
        });
    if first_fragment_overlong
        && !(p1391_wrapped_outer_and_larger || p1392_wrapped_below_and_above_outer)
    {
        return None;
    }
    if wrap_before.is_some()
        && (!(p1387_equal_pair_above_outer
            || p1390_unequal_wrapped_pair_above_outer
            || p1391_wrapped_outer_and_larger
            || p1392_wrapped_below_and_above_outer)
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
        && !p1392_wrapped_below_and_above_outer
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
        && !p1392_wrapped_below_and_above_outer
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
        || p1392_wrapped_below_and_above_outer
    {
        &layouter.style
    } else {
        last_style
    };
    let leading = leading_style
        .leading
        .map(|length| length.resolve_pt(leading_style.size.val()))
        .unwrap_or_else(|| {
            leading_style.size.val() * super::super::vanilla_defaults::PAR_LEADING
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
            || p1392_wrapped_below_and_above_outer
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
        || p1392_wrapped_below_and_above_outer
    {
        0.0
    } else {
        block_boundary
    };
    let mut second_smaller_baseline_adjustment = 0.0;
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
        second_smaller_baseline_adjustment = if p1392_wrapped_below_and_above_outer
            && second_style.size.0 < layouter.style.size.0
        {
            let outer_top =
                layouter.metrics.text_edges(layouter.style.size, &layouter.style).0;
            (outer_top.0 - second_line_top.0).max(0.0)
        } else {
            0.0
        };
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
        second_smaller_baseline_adjustment,
    })
}

#[cfg(test)]
mod smoke {
    #[test]
    fn modulo_compila_e_carrega() {}
}

#[cfg(test)]
mod prepared_carriers_tests {
    use super::*;
    use crate::entities::{
        content::{LocatedContent, RealizedContent, SourceContent},
        introspector::{Introspector, TagIntrospector},
        locator::Locator,
        span::Span,
        style::{Style, Styles},
        style_chain::StyleChain,
    };
    use comemo::Track;
    use std::sync::Arc;

    fn prepared(body: Content) -> Content {
        Content::Source(Arc::new(SourceContent {
            span: Span::detached(),
            body: Content::Realized(Arc::new(RealizedContent {
                original: Content::link("hidden", Content::text("NOT VISUAL")),
                visual: Content::Located(Arc::new(LocatedContent {
                    location: Locator::new().next(),
                    body,
                })),
            })),
        }))
    }

    #[test]
    fn prepared_carriers_preserve_fractional_prefix_certificate() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::super::metrics::FixedMetrics,
            crate::entities::image_sizer::NullImageSizer,
            12.0,
            (&intr as &dyn Introspector).track(),
        );
        let initial_size = l.style.size.0;
        let size = Styles::from_iter([Style::Size(
            crate::entities::layout_types::Length::pt(8.0),
        )]);
        for plain in [
            Content::strong(Content::text("Lead")),
            Content::emph(Content::text("Lead")),
            Content::styled(Box::new(Content::text("Lead")), size.clone()),
            Content::strong(prepared(Content::text("Lead"))),
        ] {
            let baseline =
                p1380_inline_prefix(&mut l, std::slice::from_ref(&plain), 1, 85.0)
                    .expect("raw prefix must be certified");
            let wrapped = prepared(plain);
            let result = p1380_inline_prefix(&mut l, &[wrapped], 1, 85.0)
                .expect("identity/visual carriers must preserve certificate");
            assert!((result.fixed_height - baseline.fixed_height).abs() < 1e-9);
            assert!((result.block_boundary - baseline.block_boundary).abs() < 1e-9);
            assert_eq!(result.wrap_before, baseline.wrap_before);
            assert_eq!(result.end, baseline.end);
        }
        // A larger singleton is outside the legacy scalar certificate.
        let larger = Content::styled(
            Box::new(Content::text("Lead")),
            Styles::from_iter([Style::Size(crate::entities::layout_types::Length::pt(
                16.0,
            ))]),
        );
        assert!(
            p1380_inline_prefix(&mut l, std::slice::from_ref(&larger), 1, 85.0).is_none()
        );
        assert!(p1380_inline_prefix(&mut l, &[prepared(larger)], 1, 85.0).is_none());
        let opaque = prepared(Content::link("url", Content::text("Lead")));
        assert!(p1380_inline_prefix(&mut l, &[opaque], 1, 85.0).is_none());
        assert_eq!(l.style.size.0, initial_size);
    }

    #[test]
    fn prepared_carriers_shared_chain_uses_visual_lexical_scope() {
        let styles = Styles::from_iter([Style::Size(
            crate::entities::layout_types::Length::pt(18.0),
        )]);
        let chain = StyleChain::empty();
        let visual = Content::styled(Box::new(Content::text("Lead")), styles);
        let expected = super::super::super::paragraph::shared_chain(
            std::slice::from_ref(&visual),
            &chain,
        )
        .unwrap();
        let actual =
            super::super::super::paragraph::shared_chain(&[prepared(visual)], &chain)
                .expect("prepared visual must remain a complete textual segment");
        assert!(
            (crate::entities::layout_types::TextStyle::from(&actual).size.0
                - crate::entities::layout_types::TextStyle::from(&expected).size.0)
                .abs()
                < 1e-9
        );
        assert_eq!(actual.custom("par.spacing"), expected.custom("par.spacing"));
        assert!(super::super::super::paragraph::shared_chain(
            &[prepared(Content::link("url", Content::text("Lead")))],
            &chain,
        )
        .is_none());
    }
}
