//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/columns.md
//! @prompt-hash 04a61bd1
//! @layer L1
//! @updated 2026-09-29
//!
//! Atomização (ADR-0109, P377): o layout de `Columns` movido do monólito
//! `layout_content` para o arquivo da feature (forma B — free function na
//! camada de render).
//!
//! **P537** — implementação real de múltiplas colunas lado a lado para
//! notas de rodapé por coluna. O body é dividido pelos `colbreak()` e cada
//! segmento é renderizado sequencialmente no mesmo `Layouter`, reutilizando
//! o estado da página actual. As notas de rodapé pendentes são flushed no
//! fim de cada coluna através do modo `column_mode` do `Layouter`.
//!
//! **P553** — correção geométrica: `column_width` passa a ser calculado a
//! partir da largura útil da página (`page_width - 2×margin`), e cada coluna
//! é renderizada como uma mini-página de largura `column_width + 2×margin`,
//! preenchendo toda a largura útil disponível.
#![allow(deprecated)] // P1417 — fallback textual igual ao caminho P1414 do Cursor.

use crate::entities::content::Content;
use crate::entities::counter_format::format_counter;
use crate::entities::dir::Dir;
use crate::entities::elements::columns::ColumnsElem;
use crate::entities::layout_types::{Align2D, FrameItem, HAlign, Point, Pt, TextStyle};
use crate::entities::style::Styles;
use crate::entities::value::Value;

use super::helpers::{item_pos, translate_frame_item};

mod flow;
mod segmented;
mod transitions;
use self::flow::{enter_flow_block, layout_flow, leave_flow_block};
#[cfg(test)]
use self::segmented::has_visible_content;
use self::segmented::{body_dir, layout_segmented, split_by_colbreak};
use self::transitions::translate_item_x;
use super::{FontMetrics, ImageSizer, Layouter, SegmentedParLineOverlay};

/// P1417 — materializa patterns depois de ordenar as baselines das regiões
/// irmãs. `#columns()` respeita a margem configurada no contentor.
pub(super) fn realize_segmented_par_line_patterns<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    start: usize,
    number_style: &TextStyle,
    region_width: f64,
    line_start_x: f64,
    force_end_margin: bool,
) -> Vec<FrameItem> {
    let mut candidates: Vec<_> =
        layouter.pending_segmented_par_line_patterns.drain(start..).collect();
    candidates.sort_by(|left, right| left.baseline.0.total_cmp(&right.baseline.0));

    let (top, bottom) = layouter.metrics.text_edges(number_style.size, number_style);
    let number_height = (top.0 + bottom.0.abs()).max(1.0);
    let mut previous_bottom: Option<f64> = None;
    let mut realized = Vec::new();

    for candidate in candidates {
        if previous_bottom.is_some_and(|limit| candidate.baseline.0 < limit) {
            continue;
        }

        let page_scoped = matches!(
            candidate.styles.custom("par.line.numbering-scope"),
            Some(Value::Str(scope)) if scope.as_str() == "page"
        );
        if page_scoped && layouter.par_line_page != Some(candidate.page) {
            layouter.par_line_counter = 0;
        }
        layouter.par_line_page = Some(candidate.page);
        layouter.par_line_counter += 1;

        let Some(text) =
            format_counter(&[layouter.par_line_counter], candidate.pattern.as_str())
        else {
            continue;
        };
        let width = layouter.metrics.advance(&text, number_style.size, number_style).0;
        let default_clearance = (0.026 * layouter.page_config.width)
            .clamp(0.75 * number_style.size.0, 2.5 * number_style.size.0);
        let clearance = match candidate.styles.custom("par.line.number-clearance") {
            Some(Value::Length(length)) => length.resolve_pt(number_style.size.0),
            _ => default_clearance,
        };
        let configured_margin = match candidate.styles.custom("par.line.number-margin") {
            Some(Value::Align(Align2D {
                h: Some(HAlign::Right | HAlign::End),
                v: None,
            })) => HAlign::Right,
            _ => HAlign::Left,
        };
        let margin = if force_end_margin { HAlign::Right } else { configured_margin };
        let align = match candidate.styles.custom("par.line.number-align") {
            Some(Value::Align(Align2D { h: Some(h), v: None })) => *h,
            _ if margin == HAlign::Left => HAlign::Right,
            _ => HAlign::Left,
        };

        previous_bottom = Some(candidate.baseline.0 + number_height);
        realized.push((candidate.baseline, text, width, margin, align, clearance));
    }

    let max_width = realized
        .iter()
        .map(|(_, _, width, _, _, _)| *width)
        .fold(0.0, f64::max);
    realized
        .into_iter()
        .map(|(baseline, text, width, margin, align, clearance)| {
            let slot_start = if margin == HAlign::Right {
                region_width - layouter.page_config.margin.right + clearance
            } else {
                line_start_x - clearance - max_width
            };
            let free = max_width - width;
            let offset = match align {
                HAlign::Left | HAlign::Start => 0.0,
                HAlign::Center => free / 2.0,
                HAlign::Right | HAlign::End => free,
            };
            FrameItem::Text {
                pos: Point { x: Pt(slot_start + offset), y: baseline },
                text: text.into(),
                style: number_style.clone(),
                bidi: None,
            }
        })
        .collect()
}

/// Layout real de `columns(count, body, gutter:)`.
///
/// Se o body contiver `colbreak()`, cada segmento é renderizado numa
/// coluna da mesma página (modo segmentado, P537). Se não houver
/// `colbreak()`, o body é tratado como fluxo contínuo que preenche as
/// colunas sequencialmente e só cria nova página quando todas as colunas
/// da página actual estiverem cheias (P538c).
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &ColumnsElem,
) {
    // 1. Columns são realizados como Block no flow vanila. P1377 incorpora
    //    a fronteira vertical antes de configurar a mini-região.
    enter_flow_block(layouter);

    // P1415 — a lista de fixups usa índices relativos ao buffer da região.
    // Antes de trocar `current_items` por buffers de coluna, concluir a região
    // exterior para que seus índices nunca sejam consumidos numa coluna irmã.
    let mut outer_items = std::mem::take(&mut layouter.regions.current.current_items);
    layouter.apply_pending_par_line_fixups(&mut outer_items);
    layouter.regions.current.current_items = outer_items;

    let page_width = layouter.regions.current.width;
    let margins = layouter.page_config.margin;
    // rationale: PageConfig::margin é escalar único (f64) — left=right=top=bottom por definição do tipo (entities/layout_types.rs). 2.0 * margin é verdade algébrica estrutural. P1066.
    // rationale: PageConfig::margin é escalar único (f64) — left=right=top=bottom por definição do tipo (entities/layout_types.rs). 2.0 * margin é verdade algébrica estrutural. P1066.
    // rationale: PageConfig::margin é escalar único (f64) — left=right=top=bottom por definição do tipo (entities/layout_types.rs). 2.0 * margin é verdade algébrica estrutural. P1066.
    let usable_width = page_width - margins.horizontal();
    layouter.record_flow_width_at(layouter.regions.current.line_start_x.0, usable_width);
    let count = e.count.max(1);
    let count_f = count as f64;

    // 2. Resolver gutter (Length → f64 Pt; default ~4% page width).
    let gutter_pt = match e.gutter {
        Some(g) => g.resolve_pt(layouter.style.size.0),
        None => page_width * super::COLUMNS_DEFAULT_GUTTER_RATIO,
    };

    // 3. column_width = (usable_width - (count-1)*gutter) / count.
    //    A largura útil da página é page_width - 2*margin (P553).
    let column_width = (usable_width - (count_f - 1.0) * gutter_pt) / count_f;
    // Largura da região de trabalho de cada coluna: mini-página com
    // margens internas, de modo que o layout preencha toda a largura útil.
    // rationale: PageConfig::margin é escalar único (f64) — left=right=top=bottom por definição do tipo (entities/layout_types.rs). 2.0 * margin é verdade algébrica estrutural. P1066.
    // rationale: PageConfig::margin é escalar único (f64) — left=right=top=bottom por definição do tipo (entities/layout_types.rs). 2.0 * margin é verdade algébrica estrutural. P1066.
    // rationale: PageConfig::margin é escalar único (f64) — left=right=top=bottom por definição do tipo (entities/layout_types.rs). 2.0 * margin é verdade algébrica estrutural. P1066.
    let column_region_width = column_width + margins.horizontal();

    // 4. Dividir body pelos colbreaks e detectar se há colbreaks reais.
    let (segments, had_colbreak) = split_by_colbreak(&e.body, count);

    // 5. Posições horizontais das colunas (origem x absoluta na página).
    let column_x_offsets: Vec<f64> = (0..count)
        .map(|i| margins.left + i as f64 * (column_width + gutter_pt))
        .collect();

    // **P626** — em RTL a ordem de leitura é da direita para a esquerda,
    // logo a primeira coluna a encher é a mais à direita. Invertemos os
    // offsets para que o índice 0 corresponda à coluna mais à direita e
    // `start_next_column` avance para a esquerda.
    // A direcção do texto é lida do `Content::Styled` que envolve o body,
    // porque a chain activa do `Layouter` ainda não incluiu esses estilos.
    let effective_dir = body_dir(&e.body).or_else(|| {
        layouter.chain.custom("text.dir").and_then(|value| match value {
            Value::Dir(dir) => Some(*dir),
            _ => None,
        })
    });
    let is_rtl = effective_dir == Some(Dir::RTL);
    let column_x_offsets: Vec<f64> = if is_rtl {
        column_x_offsets.into_iter().rev().collect()
    } else {
        column_x_offsets
    };

    if had_colbreak {
        layout_segmented(
            layouter,
            &segments,
            &column_x_offsets,
            column_region_width,
            margins.left,
            e.page_columns,
        );
    } else {
        layout_flow(
            layouter,
            &segments[0],
            count,
            &column_x_offsets,
            column_region_width,
            margins.left,
        );
    }

    leave_flow_block(layouter);
}

#[cfg(test)]
mod p1377_tests {
    use super::super::vanilla_defaults::{PAR_LEADING, PAR_SPACING};

    #[test]
    fn p1377_default_block_boundary_adds_only_spacing_minus_leading() {
        let font = 11.0;
        let extra = (font * PAR_SPACING - font * PAR_LEADING).max(0.0);
        assert!((extra - 6.05).abs() < 1e-9);
    }
}

#[cfg(test)]
mod p1378_tests {
    use crate::entities::content::Content;
    use crate::entities::layout_types::FrameItem;

    fn text_y(
        document: &crate::entities::layout_types::PagedDocument,
        needle: &str,
    ) -> f64 {
        document
            .pages
            .iter()
            .flat_map(|page| {
                super::super::tests::frame_items_recursive(&page.items).into_iter()
            })
            .find_map(|item| match item {
                #[allow(deprecated)]
                FrameItem::Text { text, pos, .. }
                | FrameItem::TextShaped { text, pos, .. }
                    if text.as_str() == needle =>
                {
                    Some(pos.y.0)
                }
                _ => None,
            })
            .unwrap_or_else(|| panic!("texto ausente: {needle}"))
    }

    #[test]
    fn p1378_segments_irmaos_partilham_origem_logica() {
        let columns = Content::columns(
            Content::sequence(vec![
                Content::text("One"),
                Content::colbreak(false),
                Content::text("Two"),
            ]),
            2,
            None,
        );
        let document = super::super::layout(&Content::sequence(vec![
            columns,
            Content::text("Outside"),
        ]));
        let one = text_y(&document, "One");
        let two = text_y(&document, "Two");
        assert!((one - two).abs() < 1e-9, "One={one}, Two={two}");
        assert!(text_y(&document, "Outside") > one);
    }

    #[test]
    fn p1378_saida_segue_segmento_mais_profundo() {
        let columns = Content::columns(
            Content::sequence(vec![
                Content::text("One"),
                Content::linebreak(),
                Content::text("Lower"),
                Content::colbreak(false),
                Content::text("Two"),
            ]),
            2,
            None,
        );
        let document = super::super::layout(&Content::sequence(vec![
            columns,
            Content::text("Outside"),
        ]));
        assert!((text_y(&document, "One") - text_y(&document, "Two")).abs() < 1e-9);
        assert!(text_y(&document, "Outside") > text_y(&document, "Lower"));
    }

    #[test]
    fn p1378_segmento_vazio_nao_desloca_o_seguinte() {
        let control = super::super::layout(&Content::text("Control"));
        let empty_first = super::super::layout(&Content::columns(
            Content::sequence(vec![
                Content::Empty,
                Content::colbreak(false),
                Content::text("Two"),
            ]),
            2,
            None,
        ));
        assert!((text_y(&control, "Control") - text_y(&empty_first, "Two")).abs() < 1e-9);
    }
}

#[cfg(test)]
mod p1379_tests {
    use std::sync::Arc;

    use crate::entities::{
        content::Content,
        corners::Corners,
        elements::block::{BlockElem, BlockHeight},
        layout_types::{FrameItem, Length},
        sides::Sides,
    };

    fn fractional(text: &str, weight: f64) -> Content {
        Content::Block(Arc::new(BlockElem {
            body: Content::text(text),
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

    fn text_y(
        document: &crate::entities::layout_types::PagedDocument,
        needle: &str,
    ) -> f64 {
        document
            .pages
            .iter()
            .flat_map(|page| {
                super::super::tests::frame_items_recursive(&page.items).into_iter()
            })
            .find_map(|item| match item {
                #[allow(deprecated)]
                FrameItem::Text { text, pos, .. }
                | FrameItem::TextShaped { text, pos, .. }
                    if text.as_str() == needle =>
                {
                    Some(pos.y.0)
                }
                _ => None,
            })
            .unwrap_or_else(|| panic!("texto ausente: {needle}"))
    }

    #[test]
    fn p1379_cada_segmento_expande_fr_ate_o_mesmo_fundo() {
        let columns = Content::columns(
            Content::sequence(vec![
                fractional("One", 1.0),
                Content::text("After"),
                Content::colbreak(false),
                fractional("Two", 1.0),
                Content::text("Tail"),
            ]),
            2,
            None,
        );
        let document = super::super::layout(&columns);
        let one = text_y(&document, "One");
        let two = text_y(&document, "Two");
        let after = text_y(&document, "After");
        let tail = text_y(&document, "Tail");
        assert!((one - two).abs() < 1e-9);
        assert!((after - tail).abs() < 1e-9, "After={after}, Tail={tail}");
        assert!(after - one > 100.0, "One={one}, After={after}");
    }

    #[test]
    fn p1379_pesos_e_conteudo_fixo_sao_locais_ao_segmento() {
        let columns = Content::columns(
            Content::sequence(vec![
                fractional("One", 1.0),
                fractional("Two", 2.0),
                Content::text("After"),
                Content::colbreak(false),
                fractional("Three", 3.0),
                Content::text("Tail"),
            ]),
            2,
            None,
        );
        let document = super::super::layout(&columns);
        let one = text_y(&document, "One");
        let two = text_y(&document, "Two");
        let after = text_y(&document, "After");
        let tail = text_y(&document, "Tail");
        assert!(two - one > 50.0, "One={one}, Two={two}");
        assert!((after - tail).abs() < 1e-9, "After={after}, Tail={tail}");
    }

    #[test]
    fn p1379_offset_exterior_reduz_a_parcela_sem_mudar_o_fundo() {
        let columns = Content::columns(
            Content::sequence(vec![
                fractional("One", 1.0),
                Content::text("After"),
                Content::colbreak(false),
                fractional("Two", 1.0),
                Content::text("Tail"),
            ]),
            2,
            None,
        );
        let document = super::super::layout(&Content::sequence(vec![
            Content::text("Intro"),
            columns,
        ]));
        let intro = text_y(&document, "Intro");
        let one = text_y(&document, "One");
        let after = text_y(&document, "After");
        let tail = text_y(&document, "Tail");
        assert!(one > intro);
        assert!(after - intro > 150.0, "Intro={intro}, After={after}");
        assert!((after - tail).abs() < 1e-9, "After={after}, Tail={tail}");
    }
}

#[cfg(test)]
mod p1494_tests {
    use super::*;

    #[test]
    fn espaco_inferido_nao_cria_coluna_visivel() {
        assert!(!has_visible_content(&Content::Space));
        assert!(!has_visible_content(&Content::SpaceFromNewline));
        let only_spaces =
            Content::sequence(vec![Content::SpaceFromNewline, Content::Space]);
        assert!(!has_visible_content(&only_spaces));
        let with_text = Content::sequence(vec![only_spaces, Content::text("A")]);
        assert!(has_visible_content(&with_text));
    }
}
