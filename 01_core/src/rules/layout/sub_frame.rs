//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout.md
//! @prompt-hash 9c9b7122
//! @layer L1
//! @updated 2026-07-09
//!
//! Sub-layout de conteúdo numa região isolada.
//! Extraído de `layout/mod.rs` no Passo 629 (sub-passo A).

use crate::entities::content::Content;
use crate::entities::layout_types::{FrameItem, Pt};

use super::{FontMetrics, ImageSizer, Layouter};

impl<'a, M: FontMetrics, S: ImageSizer> Layouter<'a, M, S> {
    /// Layout de conteúdo numa célula de grid isolada.
    ///
    /// Salva o estado completo do layouter, cria um frame temporário com
    /// cursor em (cell_x, ascender), executa o layout e restaura o estado.
    /// Retorna `(height, items)` com posições locais ao frame temporário.
    pub(super) fn layout_sub_frame_with_width(
        &mut self,
        content: &Content,
        cell_x: f64,
        cell_width: f64,
        align_rtl: bool,
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

        // Inicializar cursor local — x = cell_x, y = ascender (como o layout principal).
        // `line_start_x = cell_x` garante que `flush_line()` dentro da célula
        // (chamado por Shape, word-wrap, etc.) reinicia o cursor à coluna
        // da célula, não à margem global da página (Passo 81.5).
        self.regions.current.cursor_x = Pt(cell_x);
        self.regions.current.line_start_x = Pt(cell_x);
        // **P625** — durante o sub-layout o alinhamento RTL deve usar o limite
        // direito do sub-frame (`cell_x + cell_width`), não o da página.
        // `align_current_line_rtl` calcula `right_margin = width - margin`,
        // logo configuramos `width = cell_x + cell_width + margin` para que
        // `flush_line` e o alinhamento final do sub-frame usem o limite
        // correcto. A altura é elevada para evitar quebras de página dentro
        // do sub-frame; a largura/altura são restauradas antes de regressar.
        self.regions.current.width = cell_x + cell_width + self.page_config.margin;
        self.regions.current.height = 1_000_000_000.0;
        let (ascender, _) = self.metrics.vertical_metrics(self.style.size);
        self.regions.current.cursor_y = ascender;
        let start_y = self.regions.current.cursor_y.0;

        // Contexto sem altura delimitada — Content::Align decai VAlign::Bottom
        // e VAlign::Horizon para Top (não há "fundo" para ancorar). Passo 82.
        self.is_height_unconstrained = true;

        self.layout_content(content);

        // **P625** — alinhar linha RTL antes de a drenar, do mesmo modo
        // que `flush_line` e `finish` fazem no fluxo principal.
        if align_rtl {
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

        (cell_height, cell_items)
    }
}
