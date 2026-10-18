//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/line_commit.md
//! @prompt-hash eb5e791e
//! @layer L1
//! @updated 2026-10-17
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
        FrameItem::TextSeparator { mut pos, advance, style, origin, bidi } => {
            let start_shift = shift_at(pos.x.0);
            let end_shift = shift_at(pos.x.0 + advance.0);
            pos.x = Pt(pos.x.0 + start_shift);
            FrameItem::TextSeparator {
                pos,
                advance: Pt(advance.0 + end_shift - start_shift),
                style,
                origin,
                bidi,
            }
        }
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
        FrameItem::TextReplacement { text, items } => FrameItem::TextReplacement {
            text,
            items: items
                .into_iter()
                .map(|child| justify_frame_item(child, opportunities, share, metrics))
                .collect(),
        },
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

fn literal_line_right(items: &[FrameItem]) -> f64 {
    items
        .iter()
        .map(|item| match item {
            FrameItem::TextSeparator { pos, advance, origin, .. }
                if *origin
                    == crate::entities::layout_types::TextSeparatorOrigin::Literal =>
            {
                pos.x.0 + advance.0
            }
            FrameItem::Link { items, .. } => literal_line_right(items),
            FrameItem::Semantic { items, .. } => literal_line_right(items),
            FrameItem::TextReplacement { items, .. } => literal_line_right(items),
            // These carriers have no terminal literal-space advance.
            FrameItem::TextSeparator { .. } => 0.0,
            FrameItem::Text { .. } => 0.0,
            FrameItem::TextShaped { .. } => 0.0,
            FrameItem::Line { .. } => 0.0,
            FrameItem::Glyph { .. } => 0.0,
            FrameItem::Image { .. } => 0.0,
            FrameItem::Shape { .. } => 0.0,
            FrameItem::Group { .. } => 0.0,
        })
        .fold(0.0_f64, f64::max)
}

/// Transparent visual envelopes share line coordinates; transformed groups do not.
fn visit_text_styles(items: &[FrameItem], visit: &mut impl FnMut(&TextStyle)) {
    for item in items {
        if super::html_frame::is_context_marker(item) {
            continue;
        }
        match item {
            FrameItem::Text { style, .. }
            | FrameItem::TextShaped { style, .. }
            | FrameItem::TextSeparator {
                style,
                origin: crate::entities::layout_types::TextSeparatorOrigin::Literal,
                ..
            } => visit(style),
            FrameItem::Semantic {
                kind:
                    crate::entities::layout_types::SemanticKind::ExplicitLinebreakBoundary
                    | crate::entities::layout_types::SemanticKind::ParbreakBoundary,
                ..
            } => {}
            FrameItem::Link { items, .. }
            | FrameItem::Semantic { items, .. }
            | FrameItem::TextReplacement { items, .. } => visit_text_styles(items, visit),
            FrameItem::TextSeparator { .. }
            | FrameItem::Group { .. }
            | FrameItem::Glyph { .. }
            | FrameItem::Line { .. }
            | FrameItem::Image { .. }
            | FrameItem::Shape { .. } => {}
        }
    }
}

impl<'a, M: FontMetrics, S: ImageSizer> super::Layouter<'a, M, S> {
    pub(super) fn record_pending_line_width(&mut self) {
        self.record_line_width(true);
    }

    fn record_line_width(&mut self, preserve_terminal_literal: bool) {
        if super::html_frame::items_have_causal_line(&self.regions.current.current_line) {
            let right = super::helpers::line_content_right(
                &self.regions.current.current_line,
                &self.metrics,
            );
            let right = if preserve_terminal_literal {
                right.max(literal_line_right(&self.regions.current.current_line))
            } else {
                right
            };
            self.record_flow_width_at(right, 0.0);
        }
    }

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
        let right_margin = super::paragraph_indent::bounds(self).1;
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
        let total_delta = if distributable.is_finite() {
            pending
                .iter()
                .map(|(_, weight, natural_width)| share(*weight) - natural_width)
                .sum::<f64>()
        } else {
            0.0
        };
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
        if !pending.is_empty() {
            self.record_flow_width_at(content_right + total_delta, 0.0);
        }
    }

    /// P1140.11 — distribui dinamicamente o espaço restante pelos espaços
    /// atravessados na linha. Valores de sondas não entram nesta fórmula.
    pub(super) fn justify_current_line(&mut self) {
        self.expand_fr_spacings();
        let right_margin = super::paragraph_indent::bounds(self).1;
        if !right_margin.is_finite() || self.justify_opportunities.is_empty() {
            return;
        }

        let line_refs: Vec<&FrameItem> =
            self.regions.current.current_line.iter().collect();
        let content_right = self.metrics.line_content_right(&line_refs);
        let is_rtl = super::paragraph::line_direction(self) == Some(Dir::RTL);
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

    pub(super) fn flush_continued_line(&mut self) {
        let justified = matches!(
            self.chain.custom("par.justify"),
            Some(crate::entities::value::Value::Bool(true))
        );
        if justified {
            self.justify_current_line();
        }
        let force_outer_singleton =
            std::mem::take(&mut self.wrapped_inline_prefix_line_armed);
        self.flush_line_with_outer_singleton(force_outer_singleton, !justified);
    }

    pub(super) fn flush_line(&mut self) {
        let force_outer_singleton =
            std::mem::take(&mut self.wrapped_inline_prefix_line_armed);
        self.flush_line_with_outer_singleton(force_outer_singleton, false);
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
        self.flush_line_with_outer_singleton(true, false);
    }

    fn flush_line_with_outer_singleton(
        &mut self,
        force_outer_singleton: bool,
        continued: bool,
    ) {
        self.terminal_paragraph_line = false;
        // **P842 (#38)** — expandir h(Nfr) pendentes antes de qualquer
        // medição da linha (collector de decorações, leading, RTL align).
        self.expand_fr_spacings();
        // Avançar cursor_y apenas se havia items pendentes na linha actual
        // (Passo 83). Caso contrário, flush_line é um no-op semanticamente
        // — evita acumular line_height em cascata quando Shape/Image/Heading
        // chamam flush_line por segurança antes do seu próprio push.
        let had_items =
            super::html_frame::items_have_causal_line(&self.regions.current.current_line);
        if had_items
            && !super::html_frame::items_have_material(&self.regions.current.current_line)
        {
            super::text_baseline::admit_literal_line(self);
        }

        // A soft wrap consumes terminal spaces without allocating their advance.
        // Keep their carriers for text extraction; explicit final spaces remain.
        self.record_line_width(!continued);

        // Determinar o tamanho máximo de fonte presente nos items da linha e
        // o estilo do item que o possui (clonado para não manter borrow da
        // current_line durante o drain). Se a linha não tiver items de texto,
        // usa self.style como fallback.
        let outer_font_size = self.style.size;
        let mut max_text = None::<(Pt, TextStyle)>;
        let mut first_text_size = None::<f64>;
        let mut mixed_text_sizes = false;
        let mut text_count = 0usize;
        let mut actual_text_top = 0.0_f64;
        let mut actual_text_bottom = 0.0_f64;
        visit_text_styles(&self.regions.current.current_line, &mut |style| {
            let size = style.size;
            mixed_text_sizes |= (text_count > 0 && size.0 != outer_font_size.0)
                || first_text_size.is_some_and(|first| size.0 != first);
            first_text_size = first_text_size.or(Some(size.0));
            if max_text.as_ref().is_none_or(|(max, _)| size.0 > max.0) {
                max_text = Some((size, style.clone()));
            }
            text_count += 1;
            let (top, bottom) = self.metrics.text_edges(size, style);
            actual_text_top = actual_text_top.max(top.0);
            actual_text_bottom = actual_text_bottom.min(bottom.0);
        });
        let forced_smaller_singleton = force_outer_singleton
            && text_count == 1
            && first_text_size.is_some_and(|size| size < outer_font_size.0);
        let (max_font_size, max_style) = if !self.geometric_text_lines
            && (forced_smaller_singleton || self.column_mode)
        {
            match max_text {
                Some((size, style)) if size.0 > self.style.size.0 => (size, style),
                _ => (self.style.size, self.style.clone()),
            }
        } else {
            max_text.unwrap_or_else(|| (self.style.size, self.style.clone()))
        };
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
        let common_style = self.geometric_text_lines.then(|| {
            TextStyle::from(self.text_segment_chain.as_ref().unwrap_or(&self.chain))
        });
        let line_leading_pt = if let Some(style) = &common_style {
            style
                .leading
                .map(|l| l.resolve_pt(style.size.0))
                .unwrap_or(style.size.0 * super::vanilla_defaults::PAR_LEADING)
        } else if mixed_text_sizes || forced_smaller_singleton {
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
        let legacy_edge_fallback =
            !self.geometric_text_lines && (forced_smaller_singleton || self.column_mode);
        let (current_text_top, current_text_bottom) = if !had_items {
            (Pt(0.0), Pt(0.0))
        } else if text_count == 0 {
            self.metrics.text_edges(max_font_size, &max_style)
        } else if legacy_edge_fallback {
            let (top, bottom) = self.metrics.text_edges(max_font_size, &max_style);
            (Pt(actual_text_top.max(top.0)), Pt(actual_text_bottom.min(bottom.0)))
        } else {
            (Pt(actual_text_top), Pt(actual_text_bottom))
        };
        let next_text_top =
            if let Some(style) = common_style.as_ref().filter(|_| had_items) {
                self.metrics.text_edges(style.size, style).0
            } else if had_items && mixed_text_sizes {
                self.metrics.text_edges(self.style.size, &self.style).0
            } else {
                current_text_top
            };
        let assumed_ascent = self.line_assumed_ascent.unwrap_or(next_text_top.0);
        let reconcile_text_top = self.geometric_text_lines
            || (text_count > 0 && !self.column_mode && !force_outer_singleton);
        let line_extra_ascent = if reconcile_text_top {
            inline_ascent.max(actual_text_top) - assumed_ascent
        } else if mixed_text_sizes {
            inline_ascent.max(current_text_top.0) - assumed_ascent
        } else {
            inline_ascent.max(actual_text_top) - assumed_ascent
        };
        let mut line_extra_ascent = if reconcile_text_top {
            line_extra_ascent
        } else {
            line_extra_ascent.max(0.0)
        };

        // P1292-D v10 — uma inserção bottom reduz a geometria efectiva
        // antes de o próximo child estabilizar. Se a tinta da linha corrente
        // não cabe nessa região, migra a linha ainda não comitada pelo
        // caminho normal de avanço; não é preciso conhecer seu Content.
        let mut effective_line_committed = false;
        if had_items
            && !self.is_sub_frame
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

        // Ordinary root pages admit the entire settled line before commit.
        // Reserved regions and suffix transactions retain their own protocol.
        let ordinary_root = had_items
            && !self.is_sub_frame
            && self.page_config.height.is_finite()
            && self.regions.current.height == self.page_config.height
            && !self.column_mode
            && self.page_columns.is_none()
            && self.floats_pending.is_empty()
            && self.cursor_y_top_reserve == 0.0
            && self.cursor_y_bottom_reserve == 0.0
            && !self.flow_suffix_checkpoint_pending
            && !self.flow_suffix_transaction_active
            && self.flow_suffix_checkpoint.is_none()
            && self.decoration_lines_collector.is_none();
        if ordinary_root {
            let descent = if text_count == 0 {
                inline_descent
            } else {
                (-current_text_bottom.0).max(inline_descent)
            };
            let ink_bottom =
                self.regions.current.cursor_y.0 + line_extra_ascent + descent;
            if ink_bottom > self.page_bottom_limit() {
                let has_previous_unit = super::html_frame::items_have_material(
                    &self.regions.current.current_items,
                ) || self.flow_placement_end.is_some();
                if has_previous_unit {
                    let old_baseline = self.regions.current.cursor_y.0;
                    let old_left = self.page_config.margin.left;
                    let old_cursor = self.regions.current.cursor_x.0;
                    let old_start = self.regions.current.line_start_x.0;
                    let ascent = assumed_ascent + line_extra_ascent;
                    // Ask the region owner for its geometric top, without a
                    // font strut that could differ from this line's real ascent.
                    self.initial_baseline_pending = true;
                    self.new_page();
                    self.regions.current.cursor_y += Pt(ascent);
                    self.initial_baseline_pending = false;
                    self.line_assumed_ascent = Some(ascent);
                    let dx = self.page_config.margin.left - old_left;
                    let dy = self.regions.current.cursor_y.0 - old_baseline;
                    for item in &mut self.regions.current.current_line {
                        super::helpers::offset_frame_item(item, dx, dy);
                    }
                    self.regions.current.cursor_x = Pt(old_cursor + dx);
                    self.regions.current.line_start_x = Pt(old_start + dx);
                    line_extra_ascent = 0.0;
                }
                // An oversized first unit is committed once, even if its ink
                // exceeds an empty region. Its predicted next baseline is not
                // a reason to manufacture a terminal blank page.
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
            let decoration_start = Pt(super::paragraph_indent::bounds(self).0);
            if let Some(coll) = self.decoration_lines_collector.as_mut() {
                coll.push(super::DecoSegment {
                    start_x: decoration_start,
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
        if had_items && line_extra_ascent != 0.0 {
            self.shift_current_line_y(line_extra_ascent);
            self.regions.current.cursor_y += Pt(line_extra_ascent);
        }

        // P576 — alinhamento de parágrafo RTL.
        self.align_current_line_rtl_with_continuation(continued);

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
                                    bidi: None,
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

        // Real ink has stabilized. A fitting line does not require another
        // region solely because its predicted following baseline exceeds it.
        if had_items {
            let ink_bottom = self.regions.current.cursor_y.0
                + (-current_text_bottom.0).max(inline_descent);
            effective_line_committed |= ink_bottom <= self.page_bottom_limit();
        }
        self.pending_line_dir = None;
        self.close_link_line_segments();
        for item in self.regions.current.current_line.drain(..) {
            self.regions.current.current_items.push(item);
        }
        self.open_link_segments();
        self.justify_opportunities.clear();
        if had_items {
            // **P1382** — a próxima baseline nasce com o ascent do estilo
            // activo, não com o maior ascent transitório da linha corrente.
            self.line_assumed_ascent = Some(next_text_top.0);

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
            self.record_flow_placement_end(baseline_y + line_descent);
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

        super::paragraph_indent::commit(self, had_items);
        super::region_progress::post_line(self, had_items, effective_line_committed);
    }
}

#[cfg(test)]
mod tests {
    use super::super::FixedMetrics;
    use super::*;
    use crate::entities::geometry::LineCap;

    #[test]
    fn two_smaller_fragments_do_not_take_outer_ascent_for_current_line() {
        use crate::entities::{
            image_sizer::NullImageSizer,
            introspector::{Introspector, TagIntrospector},
            layout_types::TextStyle,
        };
        use comemo::Track;
        let intr = TagIntrospector::empty();
        let mut l = super::super::Layouter::new(
            FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        let style = TextStyle::regular(Pt(10.0));
        l.initial_baseline_pending = false;
        l.regions.current.cursor_y = Pt(7.0);
        l.line_assumed_ascent = Some(7.0);
        for (x, text) in [(0.0, "A"), (6.0, "B")] {
            l.regions.current.current_line.push(FrameItem::Text {
                pos: Point { x: Pt(x), y: Pt(7.0) },
                text: text.into(),
                style: style.clone(),
                bidi: None,
            });
        }
        l.flush_line();
        let ys: Vec<_> = l
            .regions
            .current
            .current_items
            .iter()
            .filter_map(|item| {
                if let FrameItem::Text { pos, .. } = item {
                    Some(pos.y.0)
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(ys, vec![7.0, 7.0]);
    }

    #[test]
    fn rtl_continued_line_reserves_signed_terminal_tracking() {
        use crate::entities::{
            dir::Dir,
            image_sizer::NullImageSizer,
            introspector::{Introspector, TagIntrospector},
            layout_types::{Length, TextStyle},
        };
        use comemo::Track;
        let intr = TagIntrospector::empty();
        for t in [0.2, -0.2, 0.0] {
            let mut l = super::super::Layouter::new(
                FixedMetrics,
                NullImageSizer,
                80.0,
                (&intr as &dyn Introspector).track(),
            );
            let mut style = TextStyle::regular(Pt(12.0));
            style.dir = Some(Dir::RTL);
            style.tracking = Some(Length::pt(t));
            l.style = style.clone();
            let width = super::super::text_measure::word(&l.metrics, "abc", &style);
            l.regions.current.current_line.push(FrameItem::Text {
                pos: Point { x: Pt(0.0), y: Pt(0.0) },
                text: "abc".into(),
                style,
                bidi: None,
            });
            l.regions.current.cursor_x = width;
            let right = l.regions.current.width - l.page_config.margin.right;
            l.flush_continued_line();
            let item = l
                .regions
                .current
                .current_items
                .iter()
                .find(|item| matches!(item, FrameItem::Text { .. }))
                .unwrap();
            let FrameItem::Text { pos, .. } = item else { panic!() };
            assert!(
                (pos.x.0 - (right - width.0 - t)).abs() < 1e-9,
                "continued RTL line must reserve its terminal tracking interval"
            );
        }
    }

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

#[cfg(test)]
mod face_edge_regressions {
    use super::super::FixedMetrics;
    use super::*;
    use crate::entities::{
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{
            Length, LinkTarget, PageMarginSpec, PageMargins, SemanticKind,
            SemanticPlacement, Size, TextEdge, TextSeparatorOrigin, TransformMatrix,
        },
        style::{Style, Styles},
    };
    use comemo::Track;

    struct Faces;
    impl FontMetrics for Faces {
        fn advance(&self, text: &str, size: Pt, style: &TextStyle) -> Pt {
            FixedMetrics.advance(text, size, style)
        }
        fn vertical_metrics(&self, size: Pt, _: &TextStyle) -> (Pt, Pt) {
            (size * 1.2, size * 1.5)
        }
        fn cap_height(&self, size: Pt, style: &TextStyle) -> Pt {
            size * if style.bold {
                1.0
            } else if style.italic {
                0.6
            } else {
                0.7
            }
        }
        fn text_edges(&self, size: Pt, style: &TextStyle) -> (Pt, Pt) {
            let (top, bottom) = FixedMetrics.text_edges(size, style);
            (
                if style.top_edge.is_some() { top } else { self.cap_height(size, style) },
                bottom,
            )
        }
    }
    fn configured(
        intr: &dyn Introspector,
        baseline: f64,
        assumed: f64,
    ) -> super::super::Layouter<'_, Faces, NullImageSizer> {
        let mut l =
            super::super::Layouter::new(Faces, NullImageSizer, 10.0, intr.track());
        l.chain = l
            .chain
            .push_styles(&Styles::from_iter([Style::Size(Length::pt(10.0))]));
        l.style = TextStyle::from(&l.chain);
        l.page_config.width = 100.0;
        l.page_config.height = f64::INFINITY;
        l.page_config.margin = PageMargins::uniform(0.0);
        l.page_config.margin_spec = PageMarginSpec::uniform(0.0);
        l.regions.current.width = 100.0;
        l.regions.current.height = f64::INFINITY;
        l.regions.current.line_start_x = Pt(0.0);
        l.regions.current.cursor_x = Pt(20.0);
        l.initial_baseline_pending = false;
        l.regions.current.cursor_y = Pt(baseline);
        l.line_assumed_ascent = Some(assumed);
        l
    }
    fn text(style: TextStyle, y: f64) -> FrameItem {
        FrameItem::Text {
            pos: Point { x: Pt(0.0), y: Pt(y) },
            text: "X".into(),
            style,
            bidi: None,
        }
    }
    fn ys(l: &super::super::Layouter<Faces, NullImageSizer>) -> Vec<f64> {
        super::super::tests::frame_items_recursive(&l.regions.current.current_items)
            .iter()
            .filter_map(|i| match i {
                FrameItem::Text { pos, .. } => Some(pos.y.0),
                _ => None,
            })
            .collect()
    }
    #[test]
    fn face_edges_equal_sizes_use_actual_top_in_both_orders() {
        let intr = TagIntrospector::empty();
        for italic_first in [true, false] {
            let normal = TextStyle::regular(Pt(10.0));
            let mut italic = normal.clone();
            italic.italic = true;
            let first = if italic_first { 6.0 } else { 7.0 };
            let mut l = configured(&intr, first, first);
            let styles = if italic_first { [italic, normal] } else { [normal, italic] };
            l.regions
                .current
                .current_line
                .extend(styles.into_iter().map(|s| text(s, first)));
            l.flush_line();
            assert_eq!(ys(&l), vec![7.0, 7.0]);
        }
    }
    #[test]
    fn face_edges_smaller_face_can_dominate_top() {
        let intr = TagIntrospector::empty();
        let mut l = configured(&intr, 7.0, 7.0);
        let normal = TextStyle::regular(Pt(10.0));
        let mut tall = TextStyle::regular(Pt(8.0));
        tall.bold = true;
        l.regions
            .current
            .current_line
            .extend([text(normal, 7.0), text(tall, 7.0)]);
        l.flush_line();
        assert_eq!(ys(&l), vec![8.0, 8.0]);
    }
    #[test]
    fn face_edges_top_and_bottom_have_independent_owners() {
        let intr = TagIntrospector::empty();
        let mut l = configured(&intr, 14.0, 14.0);
        let top = TextStyle::regular(Pt(20.0));
        let mut bottom = TextStyle::regular(Pt(10.0));
        bottom.italic = true;
        bottom.bottom_edge = Some(TextEdge::Length(Length::pt(-2.0)));
        l.regions
            .current
            .current_line
            .extend([text(top, 14.0), text(bottom, 14.0)]);
        l.flush_line();
        assert_eq!(ys(&l), vec![14.0, 14.0]);
        assert_eq!(l.regions.current.cursor_y.0, 29.5);
    }
    #[test]
    fn face_edges_continuation_reconciles_smaller_real_top() {
        let intr = TagIntrospector::empty();
        let mut l = configured(&intr, 20.5, 7.0);
        let mut italic = TextStyle::regular(Pt(10.0));
        italic.italic = true;
        l.regions.current.current_line.push(text(italic, 20.5));
        l.flush_line();
        assert_eq!(ys(&l), vec![19.5]);
    }
    #[test]
    fn face_edges_transparent_envelopes_contribute_once() {
        let intr = TagIntrospector::empty();
        let mut l = configured(&intr, 7.0, 7.0);
        let mut tall = TextStyle::regular(Pt(8.0));
        tall.bold = true;
        let wrapped = FrameItem::Semantic {
            kind: SemanticKind::Label,
            placement: SemanticPlacement::Inline,
            alt: None,
            items: vec![FrameItem::Link {
                target: LinkTarget::Url("https://example.invalid".into()),
                pos: Point { x: Pt(0.0), y: Pt(7.0) },
                size: Size { width: Pt(8.0), height: Pt(8.0) },
                items: vec![FrameItem::TextReplacement {
                    text: "X".into(),
                    items: vec![text(tall, 7.0)],
                }],
            }],
        };
        l.regions
            .current
            .current_line
            .extend([wrapped, text(TextStyle::regular(Pt(10.0)), 7.0)]);
        l.flush_line();
        assert_eq!(ys(&l), vec![8.0, 8.0]);
    }
    #[test]
    fn face_edges_group_uses_inline_extent_without_remeasuring_children() {
        let intr = TagIntrospector::empty();
        let mut l = configured(&intr, 3.0, 3.0);
        let mut tall = TextStyle::regular(Pt(30.0));
        tall.bold = true;
        l.line_inline_ascent = 3.0;
        l.regions.current.current_line.push(FrameItem::Group {
            paint_root: true,
            pos: Point { x: Pt(0.0), y: Pt(3.0) },
            matrix: TransformMatrix::identity(),
            clip_mask: None,
            inner_width: 3.0,
            inner_height: 3.0,
            items: vec![text(tall, 0.0)],
        });
        l.flush_line();
        let FrameItem::Group { pos, .. } = &l.regions.current.current_items[0] else {
            panic!("group preserved")
        };
        assert_eq!(pos.y.0, 3.0);
    }
    #[test]
    fn face_edges_break_boundaries_do_not_supply_font_metrics() {
        let intr = TagIntrospector::empty();
        for kind in
            [SemanticKind::ExplicitLinebreakBoundary, SemanticKind::ParbreakBoundary]
        {
            let mut l = configured(&intr, 20.5, 7.0);
            let mut italic = TextStyle::regular(Pt(10.0));
            italic.italic = true;
            let mut marker_text = text(TextStyle::regular(Pt(10.0)), 20.5);
            if let FrameItem::Text { text, .. } = &mut marker_text {
                *text = "".into();
            }
            l.regions.current.current_line.extend([
                text(italic, 20.5),
                FrameItem::Semantic {
                    kind,
                    placement: SemanticPlacement::Inline,
                    alt: None,
                    items: vec![marker_text],
                },
            ]);
            l.flush_line();
            assert_eq!(ys(&l), vec![19.5, 19.5]);
        }
    }
    #[test]
    fn face_edges_weak_separator_does_not_define_text_top() {
        let intr = TagIntrospector::empty();
        let mut l = configured(&intr, 7.0, 7.0);
        let mut tall = TextStyle::regular(Pt(30.0));
        tall.bold = true;
        l.regions.current.current_line.push(FrameItem::TextSeparator {
            pos: Point { x: Pt(0.0), y: Pt(7.0) },
            advance: Pt(0.0),
            style: tall,
            origin: TextSeparatorOrigin::Markup,
            bidi: None,
        });
        l.regions
            .current
            .current_line
            .push(text(TextStyle::regular(Pt(10.0)), 7.0));
        l.flush_line();
        assert_eq!(ys(&l), vec![7.0]);
    }
}
