//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout.md
//! @prompt-hash 5da4bce9
//! @layer L1
//! @updated 2026-07-09
//!
//! Sub-layout de conteúdo numa região isolada.
//! Extraído de `layout/mod.rs` no Passo 629 (sub-passo A) e refactorizado
//! para `SubLayoutRegion` no sub-passo B.

use crate::entities::content::Content;
use crate::entities::layout_types::{FrameItem, Pt};

use super::{DecoSegment, FontMetrics, ImageSizer, Layouter, PendingAlignXEntry, PendingAlignYEntry};

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
    ) -> (f64, Vec<FrameItem>, Vec<DecoSegment>, Vec<PendingAlignXEntry>, Vec<PendingAlignYEntry>) {
        // Salvar estado.
        let saved_items = std::mem::take(&mut self.regions.current.current_items);
        let saved_line = std::mem::take(&mut self.regions.current.current_line);
        let saved_x = self.regions.current.cursor_x;
        let saved_y = self.regions.current.cursor_y;
        let saved_line_start_x = self.regions.current.line_start_x;
        let saved_width = self.regions.current.width;
        let saved_height = self.regions.current.height;
        let saved_unconstrained = self.is_height_unconstrained;
        let saved_is_sub_frame = self.is_sub_frame;
        let saved_initial_baseline_pending = self.initial_baseline_pending;
        // P813 — o avanço do último flush pertence ao frame pai; dentro do
        // sub-frame recomeça (o sub-frame posiciona a sua própria baseline).
        let saved_last_flush_advance =
            std::mem::replace(&mut self.last_flush_advance, 0.0);
        let saved_prev_eq_descent =
            std::mem::replace(&mut self.prev_block_equation_descent, 0.0);
        // **P772x** — swap do `decoration_lines_collector` ambiente por um
        // collector LOCAL (coordenadas relativas ao sub-frame), para que os
        // segmentos colectados durante `self.layout_content(content)` abaixo
        // (incluindo pelo flush manual no fim desta função) não se
        // misturem, sem tradução, com segmentos já em coordenadas absolutas
        // do frame pai. `None` quando não há collector ambiente activo —
        // sem overhead (mesma disciplina de `flush_line`/`decorations.rs`).
        let saved_collector = self.decoration_lines_collector.take();
        self.decoration_lines_collector = saved_collector.is_some().then(Vec::new);
        // P751 — durante um sub-layout isolado a baseline inicial é fixada
        // imediatamente pelo próprio sub-frame (cursor_y = ascender); não
        // queremos que `ensure_initial_baseline` dentro do sub-frame afecte
        // o estado do layout pai.
        self.initial_baseline_pending = false;
        self.is_sub_frame = true;

        // Inicializar cursor local — x = origin_x, y = ascender (como o layout principal).
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
            region.origin_x + region.width + self.page_config.margin;
        self.regions.current.height = region.height.unwrap_or(1_000_000_000.0);
        let (ascender, _) = self.metrics.vertical_metrics(self.style.size, &self.style);
        self.regions.current.cursor_y = ascender;
        let start_y = self.regions.current.cursor_y.0;

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

        // **P625** — alinhar linha RTL antes de a drenar, do mesmo modo
        // que `flush_line` e `finish` fazem no fluxo principal.
        if region.align_rtl {
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
                _ => None, // neutro: FrameItem não-textual retorna None na extracção de estilo
            })
            .fold((self.style.size, self.style.clone()), |max, (size, style)| {
                if size.0 > max.0 .0 {
                    (size, style)
                } else {
                    max
                }
            });

        // **P762** — leading com default 0,65 em do vanilla.
        #[allow(deprecated)]
        let line_leading_pt = self
            .regions
            .current
            .current_line
            .iter()
            .rev()
            .find_map(|item| match item {
                crate::entities::layout_types::FrameItem::Text { style, .. }
                | crate::entities::layout_types::FrameItem::TextShaped {
                    style, ..
                } => Some(
                    style
                        .leading
                        .map(|l| l.resolve_pt(style.size.val()))
                        // PAR_LEADING, ver vanilla_defaults.rs
                        .unwrap_or_else(|| style.size.val() * super::vanilla_defaults::PAR_LEADING),
                ),
                _ => None, // neutro: FrameItem não-textual retorna None na extracção de métricas
            })
            .unwrap_or_else(|| {
                self.style
                    .leading
                    .map(|l| l.resolve_pt(self.style.size.val()))
                    // PAR_LEADING, ver vanilla_defaults.rs
                    .unwrap_or_else(|| self.style.size.val() * super::vanilla_defaults::PAR_LEADING)
            });
        let had_items = !self.regions.current.current_line.is_empty();
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
        for item in self.regions.current.current_line.drain(..) {
            self.regions.current.current_items.push(item);
        }

        let cell_height = if had_items {
            // P1050 — Altura real do conteúdo no sub-frame: da borda superior da primeira
            // linha ao fundo da última linha (sem adicionar leading espúrio após a última linha).
            let (top, bottom) = self.metrics.text_edges(max_font_size, &max_style);
            let content_top = start_y - top.0;
            let content_bottom = self.regions.current.cursor_y.0 - bottom.0;
            (content_bottom - content_top).max(0.0)
        } else {
            (self.regions.current.cursor_y.0 - start_y).max(0.0)
        };

        // Recuperar items do sub-frame e restaurar estado.
        let cell_items =
            std::mem::replace(&mut self.regions.current.current_items, saved_items);
        self.regions.current.cursor_x = saved_x;
        self.regions.current.cursor_y = saved_y;
        self.regions.current.line_start_x = saved_line_start_x;
        self.regions.current.width = saved_width;
        self.regions.current.height = saved_height;
        self.regions.current.current_line = saved_line;
        self.is_height_unconstrained = saved_unconstrained;
        self.is_sub_frame = saved_is_sub_frame;
        self.initial_baseline_pending = saved_initial_baseline_pending;
        self.last_flush_advance = saved_last_flush_advance;
        self.prev_block_equation_descent = saved_prev_eq_descent;
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

        self.layout_content(content);

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
        self.regions.current.current_line = body_tail;
        if region.align_rtl {
            self.align_current_line_rtl();
        }
        let aligned_body_tail = std::mem::take(&mut self.regions.current.current_line);
        self.regions.current.current_line = saved_line;

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
        if height == 0.0 {
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
