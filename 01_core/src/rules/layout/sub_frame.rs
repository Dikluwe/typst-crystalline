//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout.md
//! @prompt-hash a3c1dfd7
//! @layer L1
//! @updated 2026-07-09
//!
//! Sub-layout de conteúdo numa região isolada.
//! Extraído de `layout/mod.rs` no Passo 629 (sub-passo A) e refactorizado
//! para `SubLayoutRegion` no sub-passo B.

use crate::entities::content::Content;
use crate::entities::layout_types::{FrameItem, Pt};

use super::{FontMetrics, ImageSizer, Layouter};

/// Região onde um sub-layout é executado.
///
/// Define a origem horizontal, a largura útil, a altura útil (se houver),
/// se o alinhamento RTL deve ser aplicado à última linha, e se a altura
/// é ilimitada (afecta o ancoramento de `Content::Align`).
pub(super) struct SubLayoutRegion {
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
    /// Retorna `(height, items)` com posições locais ao frame temporário.
    pub(super) fn layout_sub_frame(
        &mut self,
        content: &Content,
        region: SubLayoutRegion,
    ) -> (f64, Vec<FrameItem>) {
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
        self.regions.current.width = region.origin_x + region.width + self.page_config.margin;
        self.regions.current.height = region.height.unwrap_or(1_000_000_000.0);
        let (ascender, _) = self.metrics.vertical_metrics(self.style.size);
        self.regions.current.cursor_y = ascender;
        let start_y = self.regions.current.cursor_y.0;

        // Contexto sem altura delimitada — Content::Align decai VAlign::Bottom
        // e VAlign::Horizon para Top (não há "fundo" para ancorar). Passo 82.
        self.is_height_unconstrained = region.unconstrained_height;

        self.layout_content(content);

        // **P625** — alinhar linha RTL antes de a drenar, do mesmo modo
        // que `flush_line` e `finish` fazem no fluxo principal.
        if region.align_rtl {
            self.align_current_line_rtl();
        }

        // Flush de itens pendentes. Se a line tiver conteúdo, conta a
        // altura da linha no cell_height; caso contrário, conteúdo de uma
        // única linha não-flushed produziria altura 0 (P552).
        // Determinar o tamanho máximo de fonte presente nos items da linha.
        // Se a linha não tiver items de texto, usa self.style.size como fallback.
        #[allow(deprecated)]
        let max_font_size = self.regions.current.current_line
            .iter()
            .map(|item| match item {
                crate::entities::layout_types::FrameItem::Text { style, .. } => style.size,
                crate::entities::layout_types::FrameItem::TextShaped { style, .. } => style.size,
                _ => Pt::ZERO,
            })
            .fold(self.style.size, |max, size| if size.0 > max.0 { size } else { max });

        #[allow(deprecated)]
        let line_leading_pt = self.regions.current.current_line
            .iter()
            .rev()
            .find_map(|item| match item {
                crate::entities::layout_types::FrameItem::Text { style, .. } | crate::entities::layout_types::FrameItem::TextShaped { style, .. } => {
                    style.leading.map(|l| l.resolve_pt(style.size.val()))
                }
                _ => None,
            })
            .unwrap_or(0.0);
        let had_items = !self.regions.current.current_line.is_empty();
        for item in self.regions.current.current_line.drain(..) {
            self.regions.current.current_items.push(item);
        }

        let mut end_y = self.regions.current.cursor_y.0;
        if had_items {
            let (_, line_height) = self.metrics.vertical_metrics(max_font_size);
            end_y += line_height.0 + line_leading_pt;
        }
        let cell_height = (end_y - start_y).max(0.0);

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

        (cell_height, cell_items)
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
        let (_, line_h) = self.metrics.vertical_metrics(self.style.size);
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
