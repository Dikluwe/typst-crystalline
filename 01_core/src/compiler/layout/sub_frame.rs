//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/sub_frame.md
//! @prompt-hash 9c03601f
//! @layer L1
//! @updated 2026-10-07
//!
//! Sub-layout de conteúdo numa região isolada.
//! Extraído de `layout/mod.rs` no Passo 629 (sub-passo A) e refactorizado
//! para `SubLayoutRegion` no sub-passo B.

use crate::entities::content::Content;
use crate::entities::layout_types::{FrameItem, Pt};

use super::{
    DecoSegment, FontMetrics, ImageSizer, Layouter, PendingAlignXEntry,
    PendingAlignYEntry,
};

/// Região onde um sub-layout é executado.
///
/// Define a origem horizontal, a largura útil, a altura útil (se houver),
/// se o alinhamento RTL deve ser aplicado à última linha, e se a altura
/// é ilimitada (afecta o ancoramento de `Content::Align`).
///
/// **P994** — `pub(crate)` (era `pub(super)`): o consumidor novo é
/// `math/layout/mod.rs::layout_external` (`compiler::math` é módulo-irmão
/// de `compiler::layout`, não descendente). Visibilidade interna — não é
/// contrato público. Re-export em `compiler/layout/mod.rs`.
pub(crate) struct SubLayoutRegion {
    /// Origem x dentro do frame pai.
    pub origin_x: f64,
    /// Largura útil disponível para o conteúdo.
    pub width: f64,
    /// Altura útil disponível. `None` significa "sem limite".
    pub height: Option<f64>,
    /// Se a última linha deve ser alinhada à direita quando o estilo for RTL.
    pub align_rtl: bool,
    /// Se a altura é ilimitada (decai `VAlign::Bottom`/`VAlign::Horizon`).
    pub unconstrained_height: bool,
}

impl<'a, M: FontMetrics, S: ImageSizer> Layouter<'a, M, S> {
    /// Layout de conteúdo numa região isolada.
    ///
    /// Salva o estado completo do layouter, cria um frame temporário com
    /// cursor em (`origin_x`, ascender), executa o layout e restaura o estado.
    /// Retorna `(height, items, deco_segments)` com posições locais ao frame
    /// temporário — `deco_segments` (P772x) só é não-vazio quando havia um
    /// `decoration_lines_collector` ambiente activo (chamada aninhada dentro
    /// de `Underline`/`Strike`/`Overline`); o caller é responsável por
    /// traduzir estes segmentos para o referencial do frame pai com a MESMA
    /// translação já aplicada aos `FrameItem`s devolvidos, e por os
    /// re-inserir em `self.decoration_lines_collector` (ver
    /// `00_nucleo/prompts/compiler/layout.md` §"Decoração através de
    /// `layout_sub_frame`").
    ///
    /// **P994** — `pub(in crate::compiler)` (era `pub(super)`): consumidor
    /// novo `math/layout/mod.rs::layout_external` (módulo-irmão, não
    /// descendente de `compiler::layout`). `pub(in crate::compiler)` em vez de
    /// `pub(crate)`: é o mínimo que alcança `compiler::math` E casa com a
    /// visibilidade dos tipos da assinatura (`DecoSegment`,
    /// `PendingAlignXEntry`/`PendingAlignYEntry`, `pub(super)` em
    /// `compiler/layout/mod.rs`) — `pub(crate)` aqui dispararia
    /// `private_interfaces` e exigiria alargar mais três tipos.
    pub(in crate::compiler) fn layout_sub_frame(
        &mut self,
        content: &Content,
        region: SubLayoutRegion,
    ) -> (
        f64,
        Vec<FrameItem>,
        Vec<DecoSegment>,
        Vec<PendingAlignXEntry>,
        Vec<PendingAlignYEntry>,
    ) {
        self.layout_sub_frame_with_origin(content, region, false)
    }

    pub(super) fn layout_sub_frame_at_geometric_origin(
        &mut self,
        content: &Content,
        region: SubLayoutRegion,
    ) -> (
        f64,
        Vec<FrameItem>,
        Vec<DecoSegment>,
        Vec<PendingAlignXEntry>,
        Vec<PendingAlignYEntry>,
    ) {
        self.layout_sub_frame_with_origin(content, region, true)
    }

    fn layout_sub_frame_with_origin(
        &mut self,
        content: &Content,
        region: SubLayoutRegion,
        geometric_origin: bool,
    ) -> (
        f64,
        Vec<FrameItem>,
        Vec<DecoSegment>,
        Vec<PendingAlignXEntry>,
        Vec<PendingAlignYEntry>,
    ) {
        #[cfg(test)]
        super::tests::record_cell_sub_frame();

        // Salvar estado.
        let saved_items = std::mem::take(&mut self.regions.current.current_items);
        let saved_line = std::mem::take(&mut self.regions.current.current_line);
        self.open_link_segments();
        // Pending indices belong to this line, not to the shared layouter.
        let saved_fr = std::mem::take(&mut self.regions.current.pending_fr);
        let saved_fr_widths = std::mem::take(&mut self.pending_fr_natural_widths);
        let saved_repeats = std::mem::take(&mut self.pending_repeats);
        let saved_justify = std::mem::take(&mut self.justify_opportunities);
        let saved_bidi_scope = std::mem::take(&mut self.bidi_scope_active);
        let saved_bidi_cursor = self.bidi_cursor.take();
        let saved_text_bidi = self.text_bidi.take();
        let saved_text_segment = self.text_segment_chain.take();
        let saved_geometric_lines =
            std::mem::replace(&mut self.geometric_text_lines, false);
        let saved_paragraph_dir = self.active_paragraph_dir.take();
        let saved_line_dir = self.pending_line_dir.take();
        let saved_tracking = self.text_tracking_state.clone();
        let saved_x = self.regions.current.cursor_x;
        let saved_y = self.regions.current.cursor_y;
        let saved_line_start_x = self.regions.current.line_start_x;
        let saved_width = self.regions.current.width;
        let saved_height = self.regions.current.height;
        let saved_unconstrained = self.is_height_unconstrained;
        let saved_is_sub_frame = self.is_sub_frame;
        let saved_relative_region = self.relative_region_size;
        let parent_height = self
            .block_place_region
            .filter(|r| r.h.0.is_finite())
            .map(|r| r.h.0)
            .or(saved_relative_region.map(|size| size.1))
            .unwrap_or(
                self.regions.current.height
                    - self.page_config.margin.top
                    - self.page_config.margin.bottom,
            );
        self.relative_region_size =
            Some((region.width, region.height.unwrap_or(parent_height)));
        let saved_sub_frame_origin_y = self.sub_frame_origin_y;
        let saved_flow_placement_end = self.flow_placement_end.take();
        let saved_flow_frame_extent =
            std::mem::replace(&mut self.flow_frame_extent, (region.origin_x, 0.0));
        let saved_block_place_region = self.block_place_region.take();
        let saved_block_auto_width = std::mem::take(&mut self.block_auto_width);
        let saved_pending_bib_prefix = self.pending_bib_prefix.take();
        let saved_block_inline_region = self.block_inline_region.take();
        let saved_block_place_decorations =
            std::mem::take(&mut self.block_place_decorations);
        let saved_cell_bidi_allowed = self.cell_bidi_allowed;
        if saved_is_sub_frame {
            self.cell_bidi_allowed = false;
        }
        let saved_initial_baseline_pending = self.initial_baseline_pending;
        // P813 — o avanço do último flush pertence ao frame pai; dentro do
        // sub-frame recomeça (o sub-frame posiciona a sua própria baseline).
        let saved_last_flush_advance =
            std::mem::replace(&mut self.last_flush_advance, 0.0);
        // **P1120** — o fundo de bloco é local ao sub-frame (o exterior não
        // pode ver o interior, nem o interior herdar o exterior).
        let saved_last_block_descent_y = self.last_block_descent_y.take();
        let saved_prev_eq_descent =
            std::mem::replace(&mut self.prev_block_equation_descent, 0.0);
        let saved_prev_line_baseline =
            std::mem::replace(&mut self.prev_line_baseline, 0.0);
        let saved_prev_block_below =
            std::mem::replace(&mut self.prev_block_below_pending, 0.0);
        let saved_block_chain_active =
            std::mem::replace(&mut self.block_chain_active, false);
        let saved_prev_margin_is_parbreak =
            std::mem::replace(&mut self.prev_margin_is_parbreak, false);
        let saved_line_ascent = std::mem::replace(&mut self.line_inline_ascent, 0.0);
        let saved_line_descent = std::mem::replace(&mut self.line_inline_descent, 0.0);
        let saved_line_assumed_ascent = self.line_assumed_ascent.take();
        if !geometric_origin && super::raw::terminal_block(content) {
            self.line_assumed_ascent =
                Some(self.metrics.text_edges(self.style.size, &self.style).0 .0);
        }
        let saved_wrapped_inline_prefix_line_armed =
            std::mem::take(&mut self.wrapped_inline_prefix_line_armed);
        // **P772x** — swap do `decoration_lines_collector` ambiente por um
        // collector LOCAL (coordenadas relativas ao sub-frame), para que os
        // segmentos colectados durante `self.layout_content(content)` abaixo
        // (incluindo pelo flush manual no fim desta função) não se
        // misturem, sem tradução, com segmentos já em coordenadas absolutas
        // do frame pai. `None` quando não há collector ambiente activo —
        // sem overhead (mesma disciplina de `flush_line`/`decorations.rs`).
        let saved_collector = self.decoration_lines_collector.take();
        self.decoration_lines_collector = saved_collector.is_some().then(Vec::new);
        // Legacy callers install an ascender immediately; geometric Place
        // defers the initial baseline to actual text. Both restore parent state.
        self.initial_baseline_pending = geometric_origin;
        self.is_sub_frame = true;

        // Local x is origin_x; y is geometric zero or the legacy ascender.
        // `line_start_x = origin_x` garante que `flush_line()` dentro do sub-frame
        // reinicia o cursor à origem da região, não à margem global da página.
        self.regions.current.cursor_x = Pt(region.origin_x);
        self.regions.current.line_start_x = Pt(region.origin_x);
        // **P625** — durante o sub-layout o alinhamento RTL deve usar o limite
        // direito do sub-frame (`origin_x + width`), não o da página.
        // `align_current_line_rtl` calcula `right_margin = width - margin`,
        // logo configuramos `width = origin_x + width + margin` para que
        // `flush_line` e o alinhamento final do sub-frame usem o limite
        // correcto. A altura é elevada para evitar quebras de página dentro
        // do sub-frame; a largura/altura são restauradas antes de regressar.
        self.regions.current.width =
            region.origin_x + region.width + self.page_config.margin.right;
        self.regions.current.height = region.height.unwrap_or(1_000_000_000.0);
        self.regions.current.cursor_y = if geometric_origin {
            Pt(0.0)
        } else {
            self.metrics.vertical_metrics(self.style.size, &self.style).0
        };
        let start_y = self.regions.current.cursor_y.0;
        self.sub_frame_origin_y = start_y;
        if !geometric_origin && !super::raw::terminal_block(content) {
            self.line_assumed_ascent = Some(start_y);
        }

        // Contexto sem altura delimitada — Content::Align decai VAlign::Bottom
        // e VAlign::Horizon para Top (não há "fundo" para ancorar). Passo 82.
        self.is_height_unconstrained = region.unconstrained_height;

        // **P904 (Item 2) / P908** — `pending_align_centering`/
        // `pending_align_v_centering` (P897/P898) gravam um `path` cujo
        // primeiro elemento é relativo a `current_items`, que acima de nós
        // foi trocado por uma lista LOCAL vazia deste sub-frame. Se
        // `Content::Align`/`Content::Place` aninhado aqui dentro gravar
        // uma entrada pendente (`width`/`height: auto` da página raiz, ou
        // — desde a correcção do sentinel em `layout_place` — qualquer
        // `Place` aninhado nesta sub-região não-restringida), essa
        // entrada só é válida enquanto os items ficarem na lista local.
        // P904 descartava (truncate) estas entradas para nunca corromper
        // um item não relacionado do pai — mitigação de segurança, sem
        // corrigir a posição. **P908** substitui o descarte por devolução:
        // as entradas ficam correctas relativas à lista `cell_items`
        // devolvida por esta função (cada push directo de
        // `layout_align`/`layout_place`, mesmo invocado recursivamente
        // daqui, usa `current_items.len()` NESSE momento — que é
        // exactamente esta lista local); o CALLER (que já sabe traduzir
        // items normais para o seu próprio referencial) rebaseia estas
        // entradas com a MESMA translação, ou desce um nível de `path` se
        // envolver os items num `Group` — ver
        // `00_nucleo/prompts/compiler/layout.md` §P908. Callers que não
        // sabem compor Align/Place (medição-only, `measure_content_real`)
        // simplesmente ignoram (`_`) os dois vectores devolvidos —
        // equivalente ao descarte de antes.
        let pending_x_before = self.pending_align_centering.len();
        let pending_y_before = self.pending_align_v_centering.len();

        self.layout_content(content);

        let orphaned_align_x = self.pending_align_centering.split_off(pending_x_before);
        let orphaned_align_y = self.pending_align_v_centering.split_off(pending_y_before);

        // The final local line does not pass through flush_line. Resolve its
        // fractional spacing/repeats before alignment and before returning it.
        self.expand_fr_spacings();
        self.record_pending_line_width();

        // **P625** — alinhar linha RTL antes de a drenar, do mesmo modo
        // que `flush_line` e `finish` fazem no fluxo principal.
        if region.align_rtl || self.explicit_line_alignment.is_some() {
            self.align_current_line_rtl();
        }

        // Flush de itens pendentes. Se a line tiver conteúdo, conta a
        // altura da linha no cell_height; caso contrário, conteúdo de uma
        // única linha não-flushed produziria altura 0 (P552).
        // Determinar o tamanho máximo de fonte presente nos items da linha e
        // o estilo do item que o possui (clonado para não manter borrow da
        // current_line durante o drain). Se a linha não tiver items de texto,
        // usa self.style como fallback.
        #[allow(deprecated)]
        let (max_font_size, max_style) = self
            .regions
            .current
            .current_line
            .iter()
            .filter_map(|item| match item {
                crate::entities::layout_types::FrameItem::Text { style, .. }
                | crate::entities::layout_types::FrameItem::TextShaped {
                    style, ..
                } => Some((style.size, style.clone())),
                crate::entities::layout_types::FrameItem::Glyph {
                    size, style, ..
                } => Some((*size, style.clone())),
                crate::entities::layout_types::FrameItem::TextReplacement { .. }
                | crate::entities::layout_types::FrameItem::TextSeparator { .. }
                | crate::entities::layout_types::FrameItem::Semantic { .. }
                | crate::entities::layout_types::FrameItem::Line { .. }
                | crate::entities::layout_types::FrameItem::Image { .. }
                | crate::entities::layout_types::FrameItem::Shape { .. }
                | crate::entities::layout_types::FrameItem::Group { .. }
                | crate::entities::layout_types::FrameItem::Link { .. } => None, // neutro: N16[β] — FrameItem não-textual retorna None na extracção de estilo
            })
            .reduce(
                |max, (size, style)| {
                    if size.0 > max.0 .0 {
                        (size, style)
                    } else {
                        max
                    }
                },
            )
            .unwrap_or_else(|| (self.style.size, self.style.clone()));

        let had_items = self
            .regions
            .current
            .current_line
            .iter()
            .any(|item| !super::html_frame::is_context_marker_only_tree(item));
        let direct_text = self.regions.current.current_line.iter().any(|item| {
            matches!(
                item,
                FrameItem::Text { .. }
                    | FrameItem::TextShaped { .. }
                    | FrameItem::Glyph { .. }
            )
        });
        if had_items {
            let text_top = self.metrics.text_edges(max_font_size, &max_style).0 .0;
            let assumed = self.line_assumed_ascent.unwrap_or(text_top);
            let actual =
                self.line_inline_ascent.max(if direct_text { text_top } else { 0.0 });
            let extra = (actual - assumed).max(0.0);
            self.shift_current_line_y(extra);
            self.regions.current.cursor_y += Pt(extra);
        }

        // **P772x** — mesmo hook de `flush_line` (`cursor.rs`): regista o
        // segmento da última linha do sub-frame (que este flush manual
        // drena sem passar por `flush_line`, daí a decoração nunca ter
        // sido capturada aqui antes desta correcção).
        if had_items {
            if let Some(coll) = self.decoration_lines_collector.as_mut() {
                coll.push(DecoSegment {
                    start_x: self.regions.current.line_start_x,
                    end_x: self.regions.current.cursor_x,
                    baseline_y: self.regions.current.cursor_y,
                });
            }
        }
        self.close_link_line_segments();
        for item in self.regions.current.current_line.drain(..) {
            self.regions.current.current_items.push(item);
        }

        let cell_height = if !had_items && super::transform::terminal_native(content) {
            self.flow_placement_end
                .map_or(0.0, |(end, _)| (end - start_y).max(0.0))
        } else if !had_items
            && super::raw::terminal_block(content)
            && self.last_block_descent_y.is_some()
        {
            let top = if geometric_origin {
                start_y
            } else {
                start_y - self.metrics.text_edges(self.style.size, &self.style).0 .0
            };
            (self.last_block_descent_y.unwrap() - top).max(0.0)
        } else if had_items {
            // P1050 — Altura real do conteúdo no sub-frame: da borda superior da primeira
            // linha ao fundo da última linha (sem adicionar leading espúrio após a última linha).
            let (top, bottom) = self.metrics.text_edges(max_font_size, &max_style);
            let content_top = if geometric_origin { start_y } else { start_y - top.0 };
            let text_descent = if direct_text { -bottom.0 } else { 0.0 };
            let content_bottom = self.regions.current.cursor_y.0
                + text_descent.max(self.line_inline_descent);
            (content_bottom - content_top).max(0.0)
        } else {
            (self.regions.current.cursor_y.0 - start_y).max(0.0)
        };

        // **P1120** — fundo real do sub-frame (`baseline + descent` da última
        // equação de bloco, quando existe) antes de o estado ser restaurado.
        // O chamador (`transform.rs`) usa-o para derivar o descent do frame.
        self.last_sub_frame_flow_width = self.flow_frame_extent.1;
        self.last_sub_frame_width = (self.regions.current.cursor_x.0
            - self.regions.current.line_start_x.0)
            .max(0.0);
        self.last_sub_frame_bottom = self.last_block_descent_y;
        self.last_sub_frame_inline_extents =
            (self.line_inline_ascent, self.line_inline_descent);
        self.last_block_descent_y = saved_last_block_descent_y;

        // Recuperar items do sub-frame e restaurar estado.
        let cell_items =
            std::mem::replace(&mut self.regions.current.current_items, saved_items);
        self.regions.current.cursor_x = saved_x;
        self.regions.current.cursor_y = saved_y;
        self.regions.current.line_start_x = saved_line_start_x;
        self.regions.current.width = saved_width;
        self.regions.current.height = saved_height;
        self.regions.current.current_line = saved_line;
        self.regions.current.pending_fr = saved_fr;
        self.pending_fr_natural_widths = saved_fr_widths;
        self.pending_repeats = saved_repeats;
        self.justify_opportunities = saved_justify;
        self.bidi_scope_active = saved_bidi_scope;
        self.bidi_cursor = saved_bidi_cursor;
        self.text_bidi = saved_text_bidi;
        self.text_segment_chain = saved_text_segment;
        self.geometric_text_lines = saved_geometric_lines;
        self.active_paragraph_dir = saved_paragraph_dir;
        self.pending_line_dir = saved_line_dir;
        self.text_tracking_state = saved_tracking;
        self.is_height_unconstrained = saved_unconstrained;
        self.is_sub_frame = saved_is_sub_frame;
        self.relative_region_size = saved_relative_region;
        self.sub_frame_origin_y = saved_sub_frame_origin_y;
        self.flow_placement_end = saved_flow_placement_end;
        self.flow_frame_extent = saved_flow_frame_extent;
        self.block_place_region = saved_block_place_region;
        self.block_auto_width = saved_block_auto_width;
        self.pending_bib_prefix = saved_pending_bib_prefix;
        self.block_inline_region = saved_block_inline_region;
        self.block_place_decorations = saved_block_place_decorations;
        self.cell_bidi_allowed = saved_cell_bidi_allowed;
        self.initial_baseline_pending = saved_initial_baseline_pending;
        self.last_flush_advance = saved_last_flush_advance;
        self.prev_block_equation_descent = saved_prev_eq_descent;
        self.line_assumed_ascent = saved_line_assumed_ascent;
        self.line_inline_ascent = saved_line_ascent;
        self.line_inline_descent = saved_line_descent;
        self.wrapped_inline_prefix_line_armed = saved_wrapped_inline_prefix_line_armed;
        self.prev_line_baseline = saved_prev_line_baseline;
        self.prev_block_below_pending = saved_prev_block_below;
        self.block_chain_active = saved_block_chain_active;
        self.prev_margin_is_parbreak = saved_prev_margin_is_parbreak;
        // **P772x** — recuperar segmentos locais e restaurar o collector
        // ambiente (LIFO — ver comentário no início da função).
        let deco_segments = self.decoration_lines_collector.take().unwrap_or_default();
        self.decoration_lines_collector = saved_collector;

        (cell_height, cell_items, deco_segments, orphaned_align_x, orphaned_align_y)
    }

    /// Layout inline de conteúdo numa linha do pai.
    ///
    /// O conteúdo continua na linha horizontal actual (avança `cursor_x`,
    /// não `cursor_y`). Os itens produzidos são devolvidos ao caller em vez
    /// de serem injectados directamente no frame pai. Aplica
    /// `align_current_line_rtl()` apenas aos itens que o body adicionou à
    /// `current_line` do pai, sem afectar os itens que já lá estavam.
    ///
    /// O caller é responsável por configurar `regions.current.width` antes
    /// de chamar (por exemplo, `boxed.rs` clampa a largura ao width do box).
    pub(super) fn layout_sub_frame_inline(
        &mut self,
        content: &Content,
        region: SubLayoutRegion,
    ) -> (f64, Vec<FrameItem>) {
        // A variante inline opera sobre a linha horizontal do pai e não usa
        // região geométrica própria. Se estes campos vierem preenchidos, o
        // caller provavelmente queria `layout_sub_frame`.
        debug_assert!(
            region.origin_x == 0.0 && region.width == 0.0,
            "layout_sub_frame_inline ignora `origin_x` e `width`; \
             valores diferentes de 0.0 indicam uso incorreto"
        );

        // Guardar comprimento da linha do pai antes de layoutar o body,
        // para podermos isolar apenas os itens produzidos pelo body (P625).
        let parent_line_len_before = self.regions.current.current_line.len();

        let saved_cell_bidi_allowed =
            std::mem::replace(&mut self.cell_bidi_allowed, false);
        self.layout_content(content);
        self.cell_bidi_allowed = saved_cell_bidi_allowed;

        // Isolar a cauda da `current_line` que foi adicionada pelo body.
        let body_tail: Vec<FrameItem> = self
            .regions
            .current
            .current_line
            .drain(parent_line_len_before..)
            .collect();

        // Aplicar RTL apenas sobre os itens do body, numa linha temporária,
        // preservando a `current_line` do pai.
        let saved_line = std::mem::take(&mut self.regions.current.current_line);
        let saved_bidi_scope = std::mem::take(&mut self.bidi_scope_active);
        let saved_bidi_cursor = self.bidi_cursor.take();
        let saved_text_bidi = self.text_bidi.take();
        let saved_paragraph_dir = self.active_paragraph_dir.take();
        let saved_line_dir = self.pending_line_dir.take();
        self.regions.current.current_line = body_tail;
        if region.align_rtl {
            self.align_current_line_rtl();
        }
        let aligned_body_tail = std::mem::take(&mut self.regions.current.current_line);
        self.regions.current.current_line = saved_line;
        self.bidi_scope_active = saved_bidi_scope;
        self.bidi_cursor = saved_bidi_cursor;
        self.text_bidi = saved_text_bidi;
        self.active_paragraph_dir = saved_paragraph_dir;
        self.pending_line_dir = saved_line_dir;

        // Altura: altura da linha dos itens de texto produzidos; fallback ao
        // line_height do estilo activo se o body não produziu texto.
        let (_, line_h) = self.metrics.vertical_metrics(self.style.size, &self.style);
        let mut height = 0.0_f64;
        for item in &aligned_body_tail {
            match item {
                FrameItem::Text { .. } | FrameItem::TextShaped { .. } => {
                    height = f64::max(height, line_h.0);
                }
                _ => {}
            }
        }
        let has_visual_items = aligned_body_tail
            .iter()
            .any(|item| !super::html_frame::is_context_marker_only_tree(item));
        if height == 0.0 && has_visual_items {
            height = line_h.0;
        }

        (height, aligned_body_tail)
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {
        // V2 smoke test — módulo extraído do monólito no Passo 629.
        // A cobertura funcional vive em `layout/tests.rs` e nos testes
        // específicos de grid/placement/cursor.
    }
}

#[cfg(test)]
mod overflow_isolation {
    use super::*;
    use crate::entities::{
        elements::shape::ShapeElem,
        geometry::ShapeKind,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{Length, PageMargins},
        value::Value,
    };
    use comemo::Track;
    use std::sync::Arc;

    fn text_of(items: &[FrameItem]) -> String {
        let mut text = String::new();
        for item in items {
            match item {
                FrameItem::Text { text: value, .. } => text.push_str(value),
                FrameItem::Group { items, .. }
                | FrameItem::Link { items, .. }
                | FrameItem::TextReplacement { items, .. }
                | FrameItem::Semantic { items, .. } => text.push_str(&text_of(items)),
                FrameItem::TextSeparator { .. }
                | FrameItem::TextShaped { .. }
                | FrameItem::Line { .. }
                | FrameItem::Glyph { .. }
                | FrameItem::Image { .. }
                | FrameItem::Shape { .. } => {}
            }
        }
        text
    }

    fn check_isolation(body: Content, expected_text: &str, shapes: usize) {
        for geometric in [false, true] {
            let intr = TagIntrospector::empty();
            let mut l = Layouter::new(
                super::super::FixedMetrics,
                NullImageSizer,
                11.0,
                (&intr as &dyn Introspector).track(),
            );
            l.page_config.width = 100.0;
            l.page_config.height = 100.0;
            l.page_config.margin = PageMargins::uniform(0.0);
            l.page_config.margin_spec =
                crate::entities::layout_types::PageMarginSpec::uniform(0.0);
            l.regions.current.width = 100.0;
            l.regions.current.height = 100.0;
            l.layout_content(&Content::text("PREFIX"));
            let cursor = (l.regions.current.cursor_x, l.regions.current.cursor_y);
            let dimensions = (l.regions.current.width, l.regions.current.height);
            let prefix = text_of(&l.regions.current.current_line);
            let region = SubLayoutRegion {
                origin_x: 0.0,
                width: 8.0,
                height: Some(6.0),
                align_rtl: false,
                unconstrained_height: false,
            };
            let (height, items, _, _, _) = if geometric {
                l.layout_sub_frame_at_geometric_origin(&body, region)
            } else {
                l.layout_sub_frame(&body, region)
            };
            assert!(
                l.pages.is_empty(),
                "subframe committed {} parent pages; geometric={geometric}",
                l.pages.len()
            );
            assert_eq!((l.regions.current.cursor_x, l.regions.current.cursor_y), cursor);
            assert_eq!((l.regions.current.width, l.regions.current.height), dimensions);
            assert_eq!(text_of(&l.regions.current.current_line), prefix);
            assert!(l.regions.current.current_items.is_empty());
            assert!(height > 6.0, "overflow must remain measurable, got {height}");
            assert_eq!(text_of(&items), expected_text);
            assert_eq!(
                items.iter().filter(|i| matches!(i, FrameItem::Shape { .. })).count(),
                shapes
            );
        }
    }

    #[test]
    fn bounded_subframe_retains_oversized_shape_without_parent_page() {
        check_isolation(
            Content::Shape(Arc::new(ShapeElem {
                kind: ShapeKind::Rect,
                width: Some(Box::new(Value::Length(Length::pt(14.0)))),
                height: Some(Box::new(Value::Length(Length::pt(10.0)))),
                fill: None,
                stroke: None,
                fill_rule: Default::default(),
            })),
            "",
            1,
        );
    }

    #[test]
    fn bounded_subframe_retains_overflowing_lines_without_parent_pages() {
        check_isolation(
            Content::sequence(vec![
                Content::text("A"),
                Content::linebreak(),
                Content::text("B"),
                Content::linebreak(),
                Content::text("C"),
            ]),
            "ABC",
            0,
        );
    }

    #[test]
    fn ordinary_flow_still_paginates_overflowing_lines() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.page_config.height = 20.0;
        l.page_config.margin = PageMargins::uniform(0.0);
        l.page_config.margin_spec =
            crate::entities::layout_types::PageMarginSpec::uniform(0.0);
        l.regions.current.height = 20.0;
        l.regions.current.cursor_y = Pt(0.0);
        l.layout_content(&Content::sequence(vec![
            Content::text("A"),
            Content::linebreak(),
            Content::text("B"),
            Content::linebreak(),
            Content::text("C"),
        ]));
        let doc = l.finish();
        assert!(doc.pages.len() > 1);
        assert_eq!(
            doc.pages.iter().map(|p| text_of(&p.items)).collect::<String>(),
            "ABC"
        );
    }
}

#[cfg(test)]
mod pending_line_isolation {
    use super::*;
    use crate::entities::{
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{PageMarginSpec, PageMargins},
    };
    use comemo::Track;

    fn text_positions(items: &[FrameItem]) -> Vec<(String, f64)> {
        items
            .iter()
            .filter_map(|item| match item {
                FrameItem::Text { text, pos, .. } => Some((text.to_string(), pos.x.0)),
                _ => None,
            })
            .collect()
    }

    fn in_origins(
        mut run: impl FnMut(&mut Layouter<super::super::FixedMetrics, NullImageSizer>, bool),
    ) {
        for geometric in [false, true] {
            let intr = TagIntrospector::empty();
            let mut l = Layouter::new(
                super::super::FixedMetrics,
                NullImageSizer,
                11.0,
                (&intr as &dyn Introspector).track(),
            );
            l.page_config.width = 100.0;
            l.page_config.height = 200.0;
            l.page_config.margin = PageMargins::uniform(0.0);
            l.page_config.margin_spec = PageMarginSpec::uniform(0.0);
            l.regions.current.width = 100.0;
            l.regions.current.height = 200.0;
            l.regions.current.cursor_x = Pt(0.0);
            l.regions.current.line_start_x = Pt(0.0);
            run(&mut l, geometric);
        }
    }

    fn child(
        l: &mut Layouter<super::super::FixedMetrics, NullImageSizer>,
        body: &Content,
        geometric: bool,
        width: f64,
    ) -> Vec<FrameItem> {
        let region = SubLayoutRegion {
            origin_x: 0.0,
            width,
            height: None,
            align_rtl: true,
            unconstrained_height: true,
        };
        if geometric {
            l.layout_sub_frame_at_geometric_origin(body, region).1
        } else {
            l.layout_sub_frame(body, region).1
        }
    }

    #[test]
    fn child_fraction_tail_uses_its_own_right_edge() {
        in_origins(|l, geometric| {
            let body = Content::sequence(vec![
                Content::text("A"),
                Content::h_space_fraction(1.0, false),
                Content::text("B"),
            ]);
            let advance = l.metrics.text_width("B", l.style.size, &l.style).0;
            let items = child(l, &body, geometric, 40.0);
            let positions = text_positions(&items);
            assert_eq!(positions.iter().map(|p| p.0.as_str()).collect::<String>(), "AB");
            assert!(
                (positions[1].1 + advance - 40.0).abs() < 1e-9,
                "local B must end at 40pt: {positions:?}"
            );
            assert!(l.regions.current.pending_fr.is_empty());
            assert!(l.pending_fr_natural_widths.is_empty());
        });
    }

    #[test]
    fn child_repeat_tail_is_expanded_before_return() {
        in_origins(|l, geometric| {
            let advance = l.metrics.text_width("X", l.style.size, &l.style).0;
            let body = Content::repeat(Content::text("X"), None, false);
            let items = child(l, &body, geometric, advance * 8.0);
            let positions = text_positions(&items);
            assert_eq!(positions.len(), 8, "repeat must fill its own region");
            for (index, (text, x)) in positions.iter().enumerate() {
                assert_eq!(text, "X");
                assert!((*x - index as f64 * advance).abs() < 1e-9);
            }
            assert!(l.pending_repeats.is_empty());
        });
    }

    fn parent_survives(repeat: bool) {
        in_origins(|l, geometric| {
            let first = if repeat {
                Content::repeat(Content::text("P"), None, false)
            } else {
                Content::text("P")
            };
            l.layout_content(&Content::sequence(vec![
                first,
                Content::h_space_fraction(1.0, false),
                Content::text("Q"),
            ]));
            let pending = l.regions.current.pending_fr.clone();
            let natural = l.pending_fr_natural_widths.clone();
            let repeats = l.pending_repeats.clone();
            let opportunities = l.justify_opportunities.clone();
            let prefix = text_positions(&l.regions.current.current_line);
            let cursor = (l.regions.current.cursor_x, l.regions.current.cursor_y);
            // Control: finalize the same parent without entering any subframe.
            let saved_line = l.regions.current.current_line.clone();
            l.expand_fr_spacings();
            let control = text_positions(&l.regions.current.current_line);
            l.regions.current.current_line = saved_line;
            l.regions.current.pending_fr = pending.clone();
            l.pending_fr_natural_widths = natural.clone();
            l.pending_repeats = repeats.clone();
            l.regions.current.cursor_x = cursor.0;
            let items = child(
                l,
                &Content::sequence(vec![
                    Content::text("A"),
                    Content::linebreak(),
                    Content::text("B"),
                ]),
                geometric,
                40.0,
            );
            assert_eq!(
                text_positions(&items),
                vec![("A".into(), 0.0), ("B".into(), 0.0)]
            );
            assert_eq!(l.regions.current.pending_fr, pending);
            assert_eq!(l.pending_fr_natural_widths, natural);
            assert_eq!(format!("{:?}", l.pending_repeats), format!("{repeats:?}"));
            assert_eq!(l.justify_opportunities, opportunities);
            assert_eq!(text_positions(&l.regions.current.current_line), prefix);
            assert_eq!((l.regions.current.cursor_x, l.regions.current.cursor_y), cursor);
            l.expand_fr_spacings();
            let positions = text_positions(&l.regions.current.current_line);
            let q = positions.iter().find(|p| p.0 == "Q").unwrap();
            let advance = l.metrics.text_width("Q", l.style.size, &l.style).0;
            assert!((q.1 + advance - 100.0).abs() < 1e-9);
            assert_eq!(positions, control, "child must not alter the outer line");
        });
    }

    #[test]
    fn parent_fraction_survives_a_child_line_flush() {
        parent_survives(false);
    }

    #[test]
    fn parent_repeat_survives_a_child_line_flush() {
        parent_survives(true);
    }
}
