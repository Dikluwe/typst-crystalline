//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/line_commit.md
//! @prompt-hash 837e3acc
//! @layer L1
//! @updated 2026-09-25
//!
//! Fechamento de linha: expansão horizontal, justificação, baseline e commit.
#![allow(deprecated)] // FrameItem::Text fallback path legítimo

use crate::entities::{
    counter_format::format_counter,
    dir::Dir,
    image_sizer::ImageSizer,
    layout_types::{FrameItem, Point, Pt, TextStyle},
};

use super::helpers::{item_pos, translate_frame_item};
use super::metrics::FontMetrics;

fn justify_frame_item<M: super::FontMetrics>(
    item: FrameItem,
    opportunities: &[f64],
    share: f64,
    metrics: &M,
) -> FrameItem {
    let shift_at = |x: f64| {
        share * opportunities.iter().filter(|anchor| **anchor <= x).count() as f64
    };
    match item {
        FrameItem::Line { mut start, mut end, thickness, color, cap } => {
            start.x = Pt(start.x.0 + shift_at(start.x.0));
            end.x = Pt(end.x.0 + shift_at(end.x.0));
            FrameItem::Line { start, end, thickness, color, cap }
        }
        FrameItem::Link { target, items, .. } => {
            let items: Vec<_> = items
                .into_iter()
                .map(|child| justify_frame_item(child, opportunities, share, metrics))
                .collect();
            let (pos, size) = super::link::link_bbox(&items, metrics);
            FrameItem::Link { target, items, pos, size }
        }
        FrameItem::Semantic { kind, placement, items, alt } => FrameItem::Semantic {
            kind,
            placement,
            alt,
            items: items
                .into_iter()
                .map(|child| justify_frame_item(child, opportunities, share, metrics))
                .collect(),
        },
        other => {
            let (x, y) = item_pos(&other);
            translate_frame_item(other, Pt(x + shift_at(x)), Pt(y))
        }
    }
}

impl<'a, M: FontMetrics, S: ImageSizer> super::Layouter<'a, M, S> {
    /// **P842 (#38)** — expande os spacings fracionários (`h(Nfr)`)
    /// pendentes na linha actual, distribuindo o espaço restante
    /// proporcionalmente (paridade vanilla `Spacing::Fractional`: o fr
    /// consome o espaço entre o fim do conteúdo e a margem direita; vários
    /// fr partilham na razão dos valores; sem restante positivo, fr = 0).
    /// Chamado por `flush_line` e por `finish` antes do drain da linha.
    pub(super) fn expand_fr_spacings(&mut self) {
        if self.regions.current.pending_fr.is_empty() && self.pending_repeats.is_empty() {
            return;
        }
        let right_margin = self.regions.current.width - self.page_config.margin.right;
        // **P867** — `width: auto` não define espaço restante; fracionários
        // não expandem (limpar pending sem alterar posições).
        if !right_margin.is_finite() {
            self.regions.current.pending_fr.clear();
            self.pending_fr_natural_widths.clear();
            self.pending_repeats.clear();
            return;
        }
        let content_right = {
            let line_refs: Vec<&FrameItem> =
                self.regions.current.current_line.iter().collect();
            self.metrics.line_content_right(&line_refs)
        };
        let remaining = (right_margin - content_right).max(0.0);
        let pending_raw = std::mem::take(&mut self.regions.current.pending_fr);
        let natural_widths = std::mem::take(&mut self.pending_fr_natural_widths);
        let mut pending: Vec<(usize, f64, f64)> = pending_raw
            .into_iter()
            .enumerate()
            .map(|(ordinal, (index, weight))| {
                (index, weight, natural_widths.get(ordinal).copied().unwrap_or(0.0))
            })
            .collect();
        let total_fr: f64 = pending.iter().map(|(_, fr, _)| fr).sum();
        let distributable =
            remaining + pending.iter().map(|(_, _, width)| width).sum::<f64>();
        let share = |weight: f64| {
            let ratio = weight / total_fr;
            if ratio.is_finite() && distributable.is_finite() {
                (ratio * distributable).max(0.0)
            } else {
                0.0
            }
        };
        let total_delta = distributable
            .is_finite()
            .then(|| {
                pending
                    .iter()
                    .map(|(_, weight, natural_width)| share(*weight) - natural_width)
                    .sum::<f64>()
            })
            .unwrap_or(0.0);
        if distributable.is_finite() {
            // Ordenado por índice de inserção. Cada fr translada os items
            // à sua direita APENAS pelo seu próprio share — os shares dos
            // fr anteriores já foram aplicados nas iterações anteriores
            // (o item à direita de dois fr recebe a soma dos dois shares,
            // uma parcela por iteração).
            pending.sort_by_key(|(idx, _, _)| *idx);
            for &(idx, fr, natural_width) in &pending {
                let delta = share(fr) - natural_width;
                if delta == 0.0 {
                    continue;
                }
                let line = std::mem::take(&mut self.regions.current.current_line);
                self.regions.current.current_line = line
                    .into_iter()
                    .enumerate()
                    .map(|(i, item)| {
                        if i >= idx {
                            let (ix, iy) = super::helpers::item_pos(&item);
                            super::helpers::translate_frame_item(
                                item,
                                Pt(ix + delta),
                                Pt(iy),
                            )
                        } else {
                            item
                        }
                    })
                    .collect();
            }
        }

        // P1407 — após os suffixes já estarem nas posições definitivas, a
        // distância até o primeiro item à direita é a largura finita do
        // repeat. Ordem descendente conserva os índices das faixas anteriores.
        let mut repeats = std::mem::take(&mut self.pending_repeats);
        repeats.sort_by_key(|repeat| std::cmp::Reverse(repeat.end));
        for repeat in repeats {
            if repeat.start >= repeat.end
                || repeat.end > self.regions.current.current_line.len()
            {
                continue;
            }
            let piece_start = self.regions.current.current_line[repeat.start..repeat.end]
                .iter()
                .map(super::helpers::item_pos)
                .map(|(x, _)| x)
                .fold(f64::INFINITY, f64::min);
            let fill_end = self
                .regions
                .current
                .current_line
                .get(repeat.end)
                .map(super::helpers::item_pos)
                .map(|(x, _)| x)
                .unwrap_or(right_margin);
            let fractional_fill = pending
                .iter()
                .find(|(index, _, _)| *index == repeat.end)
                .map(|(_, fraction, _)| share(*fraction));
            let fill = fractional_fill.unwrap_or(fill_end - piece_start);
            super::repeat::expand_pending(
                &mut self.regions.current.current_line,
                &repeat,
                fill,
            );
        }

        // O cursor reflete a linha já expandida (decorações e medidas
        // subsequentes vêem o fim real da linha).
        self.regions.current.cursor_x += Pt(total_delta);
    }

    /// P1140.11 — distribui dinamicamente o espaço restante pelos espaços
    /// atravessados na linha. Valores de sondas não entram nesta fórmula.
    pub(super) fn justify_current_line(&mut self) {
        self.expand_fr_spacings();
        let right_margin = self.regions.current.width - self.page_config.margin.right;
        if !right_margin.is_finite() || self.justify_opportunities.is_empty() {
            return;
        }

        let line_refs: Vec<&FrameItem> =
            self.regions.current.current_line.iter().collect();
        let content_right = self.metrics.line_content_right(&line_refs);
        let is_rtl =
            self.regions.current.current_line.iter().find_map(|item| match item {
                FrameItem::Text { style, .. } | FrameItem::TextShaped { style, .. } => {
                    style.dir
                }
                _ => None,
            }) == Some(Dir::RTL);
        let opportunities: Vec<f64> = self
            .justify_opportunities
            .iter()
            .copied()
            .filter(|anchor| *anchor < content_right)
            .collect();
        let remaining = if is_rtl {
            (right_margin - self.regions.current.cursor_x.0).max(0.0)
        } else {
            (right_margin - content_right).max(0.0)
        };
        if opportunities.is_empty() || remaining == 0.0 {
            return;
        }

        let share = remaining / opportunities.len() as f64;
        let line = std::mem::take(&mut self.regions.current.current_line);
        self.regions.current.current_line = line
            .into_iter()
            .map(|item| justify_frame_item(item, &opportunities, share, &self.metrics))
            .collect();
        self.regions.current.cursor_x = Pt(right_margin);
    }

    pub(super) fn flush_line(&mut self) {
        let force_outer_singleton =
            std::mem::take(&mut self.wrapped_inline_prefix_line_armed);
        self.flush_line_with_outer_singleton(force_outer_singleton);
    }

    /// P1391 — o texto overlong pode fechar a linha antes de Sequence voltar
    /// a controlar o fluxo. O arm é consumido pelo primeiro `flush_line`.
    pub(super) fn arm_wrapped_inline_prefix_line(&mut self) {
        self.wrapped_inline_prefix_line_armed = true;
    }

    /// P1389 — fecha uma linha singleton que pertence ao prefixo de wrap
    /// certificado. O máximo/descent continuam no item; apenas o próximo
    /// ascent e o leading passam a seguir o estilo exterior activo.
    pub(super) fn flush_wrapped_inline_prefix_line(&mut self) {
        self.wrapped_inline_prefix_line_armed = false;
        self.flush_line_with_outer_singleton(true);
    }

    fn flush_line_with_outer_singleton(&mut self, force_outer_singleton: bool) {
        // **P842 (#38)** — expandir h(Nfr) pendentes antes de qualquer
        // medição da linha (collector de decorações, leading, RTL align).
        self.expand_fr_spacings();
        // Avançar cursor_y apenas se havia items pendentes na linha actual
        // (Passo 83). Caso contrário, flush_line é um no-op semanticamente
        // — evita acumular line_height em cascata quando Shape/Image/Heading
        // chamam flush_line por segurança antes do seu próprio push.
        let had_items =
            super::html_frame::items_have_material(&self.regions.current.current_line);

        // Determinar o tamanho máximo de fonte presente nos items da linha e
        // o estilo do item que o possui (clonado para não manter borrow da
        // current_line durante o drain). Se a linha não tiver items de texto,
        // usa self.style como fallback.
        let outer_font_size = self.style.size;
        #[allow(deprecated)]
        let (max_font_size, max_style, first_text_size, mut mixed_text_sizes, text_count) =
            self.regions
                .current
                .current_line
                .iter()
                .filter_map(|item| match item {
                    crate::entities::layout_types::FrameItem::Text { style, .. }
                    | crate::entities::layout_types::FrameItem::TextShaped {
                        style,
                        ..
                    } => Some((style.size, style.clone())),
                    _ => None, // neutro: N16[β] — itens não-textuais não contribuem métricas de tamanho para cálculo de linha
                })
                .fold(
                    (self.style.size, self.style.clone(), None::<f64>, false, 0usize),
                    |(max_size, max_style, first_size, mixed, text_count),
                     (size, style)| {
                        let mixed = mixed
                            || (text_count > 0 && size.0 != outer_font_size.0)
                            || first_size.is_some_and(|first| size.0 != first);
                        let first_size = first_size.or(Some(size.0));
                        if size.0 > max_size.0 {
                            (size, style, first_size, mixed, text_count + 1)
                        } else {
                            (max_size, max_style, first_size, mixed, text_count + 1)
                        }
                    },
                );
        let forced_smaller_singleton = force_outer_singleton
            && text_count == 1
            && first_text_size.is_some_and(|size| size < outer_font_size.0);
        if force_outer_singleton
            && text_count == 1
            && !forced_smaller_singleton
            && first_text_size.is_some_and(|size| size != outer_font_size.0)
        {
            mixed_text_sizes = true;
        }

        // **P1382/P762** — numa linha realmente mista, o leading pertence ao
        // estilo activo do parágrafo. Linhas uniformes preservam a regra
        // P762/P1381 já selada, baseada no último fragmento textual.
        #[allow(deprecated)]
        let line_leading_pt = if mixed_text_sizes || forced_smaller_singleton {
            self.style
                .leading
                .map(|l| l.resolve_pt(self.style.size.val()))
                .unwrap_or_else(|| {
                    self.style.size.val() * super::vanilla_defaults::PAR_LEADING
                })
        } else {
            self.regions
                .current
                .current_line
                .iter()
                .rev()
                .find_map(|item| match item {
                    crate::entities::layout_types::FrameItem::Text { style, .. }
                    | crate::entities::layout_types::FrameItem::TextShaped {
                        style,
                        ..
                    } => Some(
                        style
                            .leading
                            .map(|l| l.resolve_pt(style.size.val()))
                            .unwrap_or_else(|| {
                                style.size.val() * super::vanilla_defaults::PAR_LEADING
                            }),
                    ),
                    _ => None,
                })
                .unwrap_or_else(|| {
                    self.style
                        .leading
                        .map(|l| l.resolve_pt(self.style.size.val()))
                        .unwrap_or_else(|| {
                            self.style.size.val() * super::vanilla_defaults::PAR_LEADING
                        })
                })
        };

        // **P1120** — extensões inline da linha (caixas com altura própria).
        let inline_ascent = self.line_inline_ascent;
        let inline_descent = self.line_inline_descent;
        self.line_inline_ascent = 0.0;
        self.line_inline_descent = 0.0;

        // **P1382** — a baseline corrente usa o maior ascent dos frames já
        // produzidos. O avanço, porém, deve prever o ascent do estilo activo
        // da linha seguinte; reutilizar o maior fragmento corrente alongava
        // a distância quando um wrapper inline mudava o tamanho da fonte.
        let (current_text_top, current_text_bottom) =
            self.metrics.text_edges(max_font_size, &max_style);
        let next_text_top = if mixed_text_sizes {
            self.metrics.text_edges(self.style.size, &self.style).0
        } else {
            current_text_top
        };
        let assumed_ascent = if self.line_assumed_ascent > 0.0 {
            self.line_assumed_ascent
        } else {
            next_text_top.0
        };
        let line_extra_ascent = if mixed_text_sizes {
            inline_ascent.max(current_text_top.0) - assumed_ascent
        } else {
            inline_ascent - assumed_ascent
        }
        .max(0.0);

        // P1292-D v10 — uma inserção bottom reduz a geometria efectiva
        // antes de o próximo child estabilizar. Se a tinta da linha corrente
        // não cabe nessa região, migra a linha ainda não comitada pelo
        // caminho normal de avanço; não é preciso conhecer seu Content.
        let mut effective_line_committed = false;
        if had_items
            && self.page_config.height.is_finite()
            && self.regions.current.height + f64::EPSILON < self.page_config.height
        {
            let ink_bottom = self.regions.current.cursor_y.0
                + line_extra_ascent
                + (-current_text_bottom.0).max(inline_descent);
            if ink_bottom > self.page_bottom_limit()
                && (super::html_frame::items_have_material(
                    &self.regions.current.current_items,
                ) || !self.pages.is_empty())
            {
                // P1292-D v11 — a unidade posterior ao marker foi rejeitada
                // pela geometria efectiva. `new_page` pode materializar
                // buffers durante este ensaio, mas o owner Layouter possui o
                // snapshot completo e o Cursor fará rollback antes do avanço
                // definitivo. Pagebreaks por outros motivos não marcam isto.
                if self.flow_suffix_transaction_active {
                    self.flow_suffix_transaction_rejected = true;
                } else {
                    let old_baseline = self.regions.current.cursor_y.0;
                    self.new_page();
                    let delta = self.regions.current.cursor_y.0 - old_baseline;
                    self.shift_current_line_y(delta);
                }
            } else {
                // A tinta cabe; o avanço para a baseline seguinte pode
                // ultrapassar a reserva sem expulsar retroactivamente esta
                // linha. Um próximo child fará o progresso normal.
                effective_line_committed = true;
            }
        }

        // P1374 — uma distribuição fracionária ocupa a região até à
        // bottom-edge da última linha. O avanço de baseline calculado abaixo
        // pertence apenas a uma linha hipotética seguinte e não cria página
        // vazia quando a tinta desta linha ainda cabe.
        if had_items && self.fractional_flow_tail_pending {
            let ink_bottom = self.regions.current.cursor_y.0
                + line_extra_ascent
                + (-current_text_bottom.0).max(inline_descent);
            effective_line_committed |= ink_bottom <= self.page_bottom_limit();
            self.fractional_flow_tail_pending = false;
        }

        // P286 — regista a linha somente depois da eventual migração,
        // para que baseline e collector pertençam à região estabilizada.
        if had_items {
            self.prev_line_baseline = self.regions.current.cursor_y.0;
            if let Some(coll) = self.decoration_lines_collector.as_mut() {
                coll.push(super::DecoSegment {
                    start_x: self.regions.current.line_start_x,
                    end_x: self.regions.current.cursor_x,
                    baseline_y: self.regions.current.cursor_y,
                });
            }
        }

        // **P1120/P1382** — a baseline desta linha foi fixada quando a linha
        // anterior fechou (ou por `ensure_initial_baseline`), assumindo o
        // ascent do texto. Se um item inline pede mais ascent (caixa mais
        // alta que o texto), o topo da linha mantém-se e é a baseline que
        // desce — o vanilla alinha os items inline pela baseline. A linha
        // inteira desce a diferença, antes de ser drenada.
        if had_items && line_extra_ascent > 0.0 {
            self.shift_current_line_y(line_extra_ascent);
            self.regions.current.cursor_y += Pt(line_extra_ascent);
        }

        // P576 — alinhamento de parágrafo RTL.
        self.align_current_line_rtl();

        // P1414/P1415 — a ocorrência nasce somente depois de estabilizar
        // wrap, paginação e baseline. Flushes vazios e subframes não consomem
        // o contador privado do fluxo raiz.
        if had_items && !self.is_sub_frame {
            let mut numbering = self.chain.custom("par.line.numbering").cloned();
            if self.defer_segmented_par_line_patterns {
                if let Some(crate::entities::value::Value::Str(pattern)) = &numbering {
                    self.pending_segmented_par_line_patterns.push(
                        super::SegmentedParLinePatternCandidate {
                            baseline: self.regions.current.cursor_y,
                            page: self.current_page_number(),
                            pattern: pattern.clone(),
                            styles: self.chain.clone(),
                        },
                    );
                    numbering = None;
                }
            }
            if matches!(
                numbering,
                Some(
                    crate::entities::value::Value::Str(_)
                        | crate::entities::value::Value::Func(_)
                )
            ) {
                let page = self.current_page_number();
                let page_scoped = matches!(
                    self.chain.custom("par.line.numbering-scope"),
                    Some(crate::entities::value::Value::Str(scope))
                        if scope.as_str() == "page"
                );
                if page_scoped && self.par_line_page != Some(page) {
                    self.par_line_counter = 0;
                }
                self.par_line_page = Some(page);
                self.par_line_counter += 1;
                if self.segmented_par_line_overlay.is_some_and(|overlay| {
                    self.par_line_counter <= overlay.suppress_through
                }) {
                    numbering = None;
                }

                enum RealizedLineNumber {
                    Pattern(String),
                    Callback(crate::entities::content::Content),
                }

                let realized = match numbering {
                    Some(crate::entities::value::Value::Str(pattern)) => {
                        format_counter(&[self.par_line_counter], pattern.as_str())
                            .map(RealizedLineNumber::Pattern)
                    }
                    Some(crate::entities::value::Value::Func(func)) => {
                        let request = super::ParLineCallbackRequest {
                            span: func.diagnostic_span(),
                            func,
                            number: self.par_line_counter,
                            styles: self.chain.clone(),
                        };
                        self.par_line_callback_pass
                            .and_then(|pass| pass.resolve_or_record(request))
                            .map(RealizedLineNumber::Callback)
                    }
                    _ => None,
                };

                if let Some(realized) = realized {
                    use crate::entities::layout_types::{Align2D, HAlign};
                    use crate::entities::value::Value;

                    let style = TextStyle::from(&self.chain);
                    let (mut number_items, width) = match realized {
                        RealizedLineNumber::Pattern(number) => {
                            let width =
                                self.metrics.advance(&number, style.size, &style).0;
                            (
                                vec![FrameItem::Text {
                                    pos: Point {
                                        x: Pt(0.0),
                                        y: self.regions.current.cursor_y,
                                    },
                                    text: number.into(),
                                    style: style.clone(),
                                }],
                                width,
                            )
                        }
                        RealizedLineNumber::Callback(content) => {
                            let (_, items, _, _, _) = self.layout_sub_frame(
                                &content,
                                super::SubLayoutRegion {
                                    origin_x: 0.0,
                                    width: f64::INFINITY,
                                    height: None,
                                    align_rtl: false,
                                    unconstrained_height: true,
                                },
                            );
                            let refs: Vec<&FrameItem> = items.iter().collect();
                            let width = if refs.is_empty() {
                                0.0
                            } else {
                                self.metrics.line_content_right(&refs)
                            };
                            let local_baseline = items
                                .iter()
                                .find_map(|item| match item {
                                    FrameItem::Text { pos, .. }
                                    | FrameItem::TextShaped { pos, .. }
                                    | FrameItem::Glyph { pos, .. } => Some(pos.y.0),
                                    _ => None,
                                })
                                .unwrap_or(0.0);
                            let dy = self.regions.current.cursor_y.0 - local_baseline;
                            let items = items
                                .into_iter()
                                .map(|item| {
                                    let (x, y) = item_pos(&item);
                                    translate_frame_item(item, Pt(x), Pt(y + dy))
                                })
                                .collect();
                            (items, width)
                        }
                    };
                    let default_clearance = (0.026 * self.page_config.width)
                        .clamp(0.75 * style.size.0, 2.5 * style.size.0);
                    let clearance = match self.chain.custom("par.line.number-clearance") {
                        Some(Value::Length(length)) => length.resolve_pt(style.size.0),
                        _ => default_clearance,
                    };
                    let configured_margin =
                        match self.chain.custom("par.line.number-margin") {
                            Some(Value::Align(Align2D {
                                h: Some(HAlign::Right | HAlign::End),
                                v: None,
                            })) => HAlign::Right,
                            _ => HAlign::Left,
                        };
                    let last_column = self
                        .page_columns
                        .is_some_and(|count| self.current_column + 1 == count)
                        || self
                            .segmented_par_line_column
                            .is_some_and(|(index, count)| index + 1 == count);
                    let margin =
                        if last_column { HAlign::Right } else { configured_margin };
                    let align = match self.chain.custom("par.line.number-align") {
                        Some(Value::Align(Align2D { h: Some(h), v: None })) => *h,
                        _ if margin == HAlign::Left => HAlign::Right,
                        _ => HAlign::Left,
                    };
                    let slot_start = match margin {
                        HAlign::Right => {
                            self.regions.current.width - self.page_config.margin.right
                                + clearance
                        }
                        _ => self.regions.current.line_start_x.0 - clearance - width,
                    };
                    let mut x = match (margin, align) {
                        (HAlign::Right, HAlign::Right | HAlign::End) => {
                            slot_start - width
                        }
                        _ => slot_start,
                    };
                    if let Some(overlay) = self.segmented_par_line_overlay {
                        x += if margin == HAlign::Right {
                            overlay.end_x_shift
                        } else {
                            overlay.start_x_shift
                        };
                    }
                    let index = self.regions.current.current_items.len()
                        + self.regions.current.current_line.len();
                    let count = number_items.len();
                    if count > 0 {
                        self.pending_par_line_numbering
                            .push((index, count, width, margin, align));
                        for item in &mut number_items {
                            let (local_x, y) = item_pos(item);
                            *item = translate_frame_item(
                                item.clone(),
                                Pt(x + local_x),
                                Pt(y),
                            );
                        }
                        self.regions.current.current_line.extend(number_items);
                    }
                }
            }
        }

        for item in self.regions.current.current_line.drain(..) {
            self.regions.current.current_items.push(item);
        }
        self.justify_opportunities.clear();
        if had_items {
            // **P1382** — a próxima baseline nasce com o ascent do estilo
            // activo, não com o maior ascent transitório da linha corrente.
            self.line_assumed_ascent = next_text_top.0;

            // **P1120** — descida da linha = a maior entre a aresta inferior
            // do texto e o descent dos items inline (`h − ascent` da caixa).
            let line_descent = (-current_text_bottom.0).max(inline_descent);
            let advance = next_text_top + Pt(line_descent) + Pt(line_leading_pt);
            // **P813** — registar o avanço aplicado, para que consumidores
            // posteriores (equações de bloco) recuperem a baseline da linha
            // anterior (`cursor_y - last_flush_advance`).
            self.last_flush_advance = advance.0;
            // P952 — mantido pelo protocolo de colapso de bloco
            let baseline_y = self.regions.current.cursor_y.0;
            self.prev_line_baseline = baseline_y;
            self.regions.current.cursor_y += advance;
            // **P1131** — uma linha substitui o fundo vertical anterior
            // quando ele existe. Texto comum fecha na baseline (aresta
            // inferior padrão do vanilla para `height: auto`); uma caixa
            // inline com extensão própria preserva P1120 e acrescenta o
            // descent real. Deixar o campo intocado reutilizava o fundo de
            // uma equação/bloco anterior quando a linha já tinha sido
            // drenada antes de `finish()`.
            if inline_descent > 0.0 {
                self.last_block_descent_y = Some(baseline_y + line_descent);
            } else if self.last_block_descent_y.is_some() {
                // **P1131** — texto comum posterior substitui o fundo de
                // bloco anterior pela sua própria baseline. Quando não há
                // fundo anterior (`None`), preservar o caminho normal pelo
                // cursor: Align/Place e texto puro dependem dessa distinção.
                self.last_block_descent_y = Some(baseline_y);
            }
        }
        // Reiniciar ao início da linha actual — margem da página, ou cell_x
        // se estivermos dentro de um sub-layout de Grid (Passo 81.5).
        self.regions.current.cursor_x = self.regions.current.line_start_x;

        if had_items
            && self.regions.current.cursor_y.0 > self.page_bottom_limit()
            && !effective_line_committed
        {
            if self.flow_suffix_transaction_active {
                self.flow_suffix_transaction_rejected = true;
            } else {
                self.new_page();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::FixedMetrics;
    use super::*;
    use crate::entities::geometry::LineCap;

    #[test]
    fn p1503_justificacao_translada_aresta_sem_mover_baseline() {
        let line = FrameItem::Line {
            start: Point { x: Pt(0.0), y: Pt(3.0) },
            end: Point { x: Pt(10.0), y: Pt(3.0) },
            thickness: 1.0,
            color: None,
            cap: LineCap::Butt,
        };
        let shifted = justify_frame_item(line, &[4.0, 8.0], 2.0, &FixedMetrics);
        let FrameItem::Line { start, end, .. } = shifted else {
            panic!("a justificação deve preservar a variante Line");
        };
        assert_eq!(start.x, Pt(0.0));
        assert_eq!(end.x, Pt(14.0));
        assert_eq!(start.y, Pt(3.0));
        assert_eq!(end.y, Pt(3.0));
    }
}
