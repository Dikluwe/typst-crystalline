//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout.md
//! @prompt-hash 5249700d
//! @layer L1
//! @updated 2026-04-23
//!
//! Gestão do cursor do Layouter: largura de palavra, layout de palavra,
//! `flush_line`, `new_page`, número de página actual.
//! Extraído de `layout/mod.rs` no Passo 96.7 conforme ADR-0037.
#![allow(deprecated)] // P483 — FrameItem::Text fallback path legítimo
use crate::entities::{
    corners::Corners,
    counter_format::{count_numbering_tokens, format_counter},
    dir::Dir,
    geometry::ShapeKind,
    image_sizer::ImageSizer,
    layout_types::{FrameItem, Page, Point, Pt, TextStyle},
};

use super::metrics::FontMetrics;
// P245 (M9d / M7+4) — DeferredFloat buffer entry usado por
// flush_pending_floats + emit_deferred_float.
use super::DeferredFloat;
use super::helpers::{item_pos, translate_frame_item};

impl<'a, M: FontMetrics, S: ImageSizer> super::Layouter<'a, M, S> {
    /// Largura de uma palavra em Pt, incluindo tracking entre glyphs
    /// (Passo 137, fase B.1 DEBT-52).
    ///
    /// Se `TextStyle.tracking` é `Some(length)`, acrescenta
    /// `(n - 1) × tracking_pt` onde n é o número de codepoints —
    /// paridade vanilla (entre pares de glyphs, não depois do último).
    fn word_width(&self, word: &str) -> Pt {
        let base = self.metrics.advance(word, self.style.size, &self.style);
        let tracking_extra = self.style.tracking
            .map(|t| {
                let tracking_pt = t.resolve_pt(self.style.size.val());
                let n = word.chars().count();
                tracking_pt * n.saturating_sub(1) as f64
            })
            .unwrap_or(0.0);
        Pt(base.val() + tracking_extra)
    }

    pub(super) fn space_width(&self) -> Pt {
        self.metrics.advance(" ", self.style.size, &self.style)
    }

    /// **P448** — baseline ajustada pelo offset vertical do estilo (subscrito/
    /// sobrescrito). O `cursor_y` mantém-se como baseline principal da linha;
    /// o offset é aplicado só ao posicionamento do glyph.
    fn baseline_y(&self) -> Pt {
        let offset_pt = self.style.baseline_offset.resolve_pt(self.style.size.val());
        self.regions.current.cursor_y + Pt(offset_pt)
    }

    /// **P449/P471** — emite um `FrameItem::Text` precedido, se necessário, por
    /// um rectângulo de highlight. P471 adiciona `radius` (RoundedRect) e
    /// `extent` (extensão horizontal). O shape cobre `ascender → line_height`.
    fn push_text(&mut self, text: ecow::EcoString, width: Pt) {
        if let Some(fill) = self.style.highlight {
            let (ascender, line_height) = self.metrics.vertical_metrics(self.style.size);
            let size_pt = self.style.size.val();
            let extent_pt = self.style.highlight_extent
                .map(|e| e.resolve_pt(size_pt))
                .unwrap_or(0.0);
            let shape_kind = match self.style.highlight_radius {
                Some(r) if r.resolve_pt(size_pt) > 0.0 => {
                    ShapeKind::RoundedRect {
                        radii: Corners::uniform(r),
                    }
                }
                _ => ShapeKind::Rect,
            };
            self.regions.current.current_line.push(FrameItem::Shape {
                pos: Point {
                    x: Pt(self.regions.current.cursor_x.0 - extent_pt),
                    y: self.baseline_y() - ascender,
                },
                kind: shape_kind,
                width: width.0 + 2.0 * extent_pt,
                height: line_height.0,
                fill: Some(fill),
                stroke: None,
                parent_bbox_at_emit: None,
            });
        }
        self.regions.current.current_line.push(FrameItem::Text {
            pos: Point { x: self.regions.current.cursor_x, y: self.baseline_y() },
            text,
            style: self.style.clone(),
        });
    }

    pub(super) fn layout_word(&mut self, word: &str) {
        let w = self.metrics
            .advance_shaped(word, self.style.size, &self.style)
            .unwrap_or_else(|| self.word_width(word));
        let right_margin = self.regions.current.width - self.page_config.margin;
        if self.regions.current.cursor_x.0 + w.0 > right_margin && self.regions.current.cursor_x.0 > self.page_config.margin {
            // Passo 144 (ADR-0057): tentar hyphenation antes do
            // flush. Se `style.lang` define um idioma e `hypher`
            // produz pontos de quebra, escolher o maior prefixo
            // (com hífen literal) que cabe no espaço disponível;
            // emitir prefixo, fazer flush, e recursar com o resto.
            // Sem `lang` ou sem ponto de quebra que caiba: cai no
            // fallback `flush_line` original (palavra inteira para
            // linha seguinte — comportamento pré-144).
            if let Some(lang) = self.style.lang {
                let break_points = super::hyphenation::hyphenate(word, &lang);
                if !break_points.is_empty() {
                    let available = right_margin - self.regions.current.cursor_x.0;
                    for &point in break_points.iter().rev() {
                        let prefix: String = word.chars().take(point).collect();
                        let prefix_with_hyphen = format!("{}-", prefix);
                        let pw = self.word_width(&prefix_with_hyphen);
                        if pw.0 <= available {
                            self.push_text(prefix_with_hyphen.into(), pw);
                            self.regions.current.cursor_x += pw;
                            self.flush_line();
                            let rest: String = word.chars().skip(point).collect();
                            self.layout_word(&rest);
                            return;
                        }
                    }
                }
            }
            self.flush_line();
        }
        self.push_text(word.into(), w);
        // **P545** — o avanço de `space_width()` foi removido de
        // `layout_word`. O espaço entre palavras é responsabilidade do
        // caller: o layout de `Content::Text` (múltiplas palavras num
        // token) ou `Content::Space` (tokens adjacentes no markup).
        // Isto evita inserir espaço automático entre fragmentos de texto
        // produzidos por interpolações `#{expr}` adjacentes.
        self.regions.current.cursor_x += w;
    }

    /// P446 — emite um fragmento de texto sem adicionar o espaço de separação
    /// de palavras. Usado por smallcaps para compor uma palavra a partir de
    /// runs de tamanhos diferentes (minúsculas → maiúsculas a 0.8×).
    pub(super) fn layout_chunk(&mut self, chunk: &str) {
        let w = self.word_width(chunk);
        let right_margin = self.regions.current.width - self.page_config.margin;
        if self.regions.current.cursor_x.0 + w.0 > right_margin
            && self.regions.current.cursor_x.0 > self.page_config.margin
        {
            self.flush_line();
        }
        self.push_text(chunk.into(), w);
        self.regions.current.cursor_x += w;
    }

    /// P576 — desloca a linha actual para a margem direita se o seu
    /// estilo indicar `dir: rtl`. Usado por `flush_line` e por `finish`
    /// (a última linha do documento não passa por `flush_line`).
    pub(super) fn align_current_line_rtl(&mut self) {
        let is_rtl = self.regions.current.current_line
            .iter()
            .find_map(|item| match item {
                FrameItem::Text { style, .. } | FrameItem::TextShaped { style, .. } => style.dir,
                _ => None,
            })
            == Some(Dir::RTL);
        if !is_rtl {
            return;
        }
        let right_margin = self.regions.current.width - self.page_config.margin;
        let offset = right_margin - self.regions.current.cursor_x.0;
        let translated: Vec<FrameItem> = self
            .regions
            .current
            .current_line
            .drain(..)
            .map(|item| {
                let (ix, iy) = super::helpers::item_pos(&item);
                super::helpers::translate_frame_item(item, Pt(ix + offset), Pt(iy))
            })
            .collect();
        self.regions.current.current_line = translated;
    }

    pub(super) fn flush_line(&mut self) {
        // Avançar cursor_y apenas se havia items pendentes na linha actual
        // (Passo 83). Caso contrário, flush_line é um no-op semanticamente
        // — evita acumular line_height em cascata quando Shape/Image/Heading
        // chamam flush_line por segurança antes do seu próprio push.
        let had_items = !self.regions.current.current_line.is_empty();

        // P286 — hook decorações wrap-aware. Se o collector está activo
        // (consumer Underline/Strike/Overline o ligou) e há items na
        // linha que vai ser drenada, regista o segmento `(line_start_x,
        // cursor_x, cursor_y)` antes do advance. Backward-compat
        // estricta: collector None → nenhum overhead.
        if had_items {
            if let Some(coll) = self.decoration_lines_collector.as_mut() {
                coll.push(super::DecoSegment {
                    start_x:    self.regions.current.line_start_x,
                    end_x:      self.regions.current.cursor_x,
                    baseline_y: self.regions.current.cursor_y,
                });
            }
        }

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

        // Passo 138 (Fase B.2 DEBT-52): consumer leading.
        // `self.style` pode ter sido restaurado ao outer scope antes de
        // flush_line ser chamado. Em vez disso, peek no último item da
        // current_line — resolve o leading com base no seu próprio tamanho.
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

        // P576 — alinhamento de parágrafo RTL.
        self.align_current_line_rtl();

        for item in self.regions.current.current_line.drain(..) {
            self.regions.current.current_items.push(item);
        }
        if had_items {
            let (_, line_height) = self.metrics.vertical_metrics(max_font_size);
            self.regions.current.cursor_y += line_height + Pt(line_leading_pt);
        }
        // Reiniciar ao início da linha actual — margem da página, ou cell_x
        // se estivermos dentro de um sub-layout de Grid (Passo 81.5).
        self.regions.current.cursor_x = self.regions.current.line_start_x;

        if self.regions.current.cursor_y.0 > self.regions.current.height - self.page_config.margin {
            self.new_page();
        }
    }

    pub(super) fn new_page(&mut self) {
        // **P538c** — se estiver em fluxo contínuo de colunas, fechar a
        // coluna actual e avançar para a seguinte da mesma página, se
        // possível. Só cria página física quando todas as colunas da
        // página actual estiverem cheias.
        if self.page_columns.is_some() {
            self.close_current_column();
            let count = self.page_columns.unwrap_or(1);
            if self.current_column + 1 < count {
                self.start_next_column();
                return;
            }
            // Última coluna da página: juntar items de todas as colunas
            // no current_items e restaurar dimensões da página física antes
            // de a criar.
            self.merge_column_items();
            self.regions.current.width = self.page_config.width;
            self.regions.current.height = self.page_config.height;
        }

        // P245 (M9d / M7+4) — flush floats pendentes na página actual
        // antes da transição. Top floats emit no topo, bottom no fundo.
        self.flush_pending_floats();

        // P304 (P295.1) — flush footnote bodies pendentes no rodapé
        // antes de saving a Page. Items posicionados em Y absoluto
        // bottom-up; tornam-se parte dos `current_items` da página.
        self.flush_pending_footnote_bodies(None);

        // **P532** — guardar snapshot do numbering da página actual antes de
        // a fechar, para que o export PDF saiba se/desenha o número.
        let page_numbering = self.page_config.numbering.clone();
        let page_number = self.pages.len() + 1;

        let mut items = std::mem::take(&mut self.regions.current.current_items);

        // **P532** — se houver numeração automática, desenhar o número no rodapé.
        // **P541** — padrões compostos (≥2 tokens de numeração) precisam do total
        // de páginas, só conhecido no final; adiar para `finish()`.
        if let Some(pattern) = &page_numbering {
            if count_numbering_tokens(pattern) >= 2 {
                self.pending_page_numbering.push((
                    self.pages.len(),
                    page_number,
                    pattern.clone(),
                ));
            } else if let Some(text) = format_counter(&[page_number], pattern.as_str()) {
                let style = TextStyle::from(&self.chain);
                let text_width = self.metrics.advance(&text, style.size, &style).0;
                let x = (self.regions.current.width - text_width) / 2.0;
                // Coordenadas do layout: origem no canto superior-esquerdo,
                // Y cresce para baixo. O PDF inverte Y; posicionar perto do
                // fundo da página requer Y próximo de height - margin/2.
                let y = self.regions.current.height - self.page_config.margin / 2.0;
                items.push(FrameItem::Text {
                    pos: Point { x: Pt(x), y: Pt(y) },
                    text: text.into(),
                    style,
                });
            }
        }

        let page = Page {
            width:  self.regions.current.width,
            height: self.regions.current.height,
            numbering: page_numbering,
            items,
        };
        self.pages.push(page);
        self.regions.current.cursor_x = Pt(self.page_config.margin);
        self.regions.current.line_start_x = Pt(self.page_config.margin);
        let (ascender, _) = self.metrics.vertical_metrics(self.style.size);
        self.regions.current.cursor_y = Pt(self.page_config.margin) + ascender;
        // P245 — reset reservas na nova página.
        self.cursor_y_top_reserve = 0.0;
        self.cursor_y_bottom_reserve = 0.0;

        // P251 (M9d / M7+5; ADR-0079 Categoria C.2 parcial) — flush
        // pending cell tails (row break TableCell cell-level) NO TOPO
        // da nova página. Items rebased pelo `cursor_y` actual (já
        // posicionado pós-margin + ascender). Subpadrão "DeferredX
        // buffer + flush em new_page" N=1 → 2 cumulativo (P245
        // floats + P251 cell tails).
        self.flush_pending_cell_tails();

        // **P538c** — após criar página física, se estiver em modo colunas,
        // preparar a primeira coluna da nova página.
        if self.page_columns.is_some() {
            self.current_column = 0;
            self.column_page_items.clear();
            self.start_column(0);
        }
    }

    /// **P538c** — fecha a coluna actual em fluxo contínuo: flush das
    /// notas de rodapé pendentes, translada os items da coluna para a
    /// posição horizontal absoluta correcta e guarda-os no buffer de
    /// colunas da página.
    fn close_current_column(&mut self) {
        // Flush das footnotes no fundo da coluna actual.
        let prev_column_mode = self.column_mode;
        self.column_mode = true;
        self.flush_pending_footnote_bodies(None);
        self.column_mode = prev_column_mode;

        // Transladar items da coluna actual de coordenadas locais (origem
        // na margem) para coordenadas absolutas na página.
        let dx = self.column_origin_x - self.page_config.margin;
        let items = std::mem::take(&mut self.regions.current.current_items);
        let translated = items
            .into_iter()
            .map(|item| {
                let (x, y) = item_pos(&item);
                translate_frame_item(item, Pt(x + dx), Pt(y))
            })
            .collect();
        self.column_page_items.push(translated);
    }

    /// **P538c** — funde os items acumulados de todas as colunas da
    /// página actual no `current_items` da região actual, preparando a
    /// criação de uma nova página física.
    pub(super) fn merge_column_items(&mut self) {
        let mut merged: Vec<FrameItem> = Vec::new();
        for column in std::mem::take(&mut self.column_page_items) {
            merged.extend(column);
        }
        // A coluna actual já foi fechada e está em column_page_items;
        // limpar current_items por segurança e repor merged.
        self.regions.current.current_items = merged;
    }

    /// **P538c** — fecha a coluna actual, funde as colunas da página e
    /// restaura o estado do Layouter para layout de página normal.
    /// Chamado por `columns::layout` no fim do modo fluxo contínuo.
    pub(super) fn finish_columns(&mut self) {
        if self.page_columns.is_none() {
            return;
        }
        self.close_current_column();
        self.merge_column_items();
        // Restaurar estado de página normal.
        self.page_columns = None;
        self.current_column = 0;
        self.column_page_items.clear();
        self.column_x_offsets.clear();
        self.column_mode = false;
        self.column_origin_x = 0.0;
        self.column_width = self.regions.current.width; // já é a largura da página
        self.regions.current.width = self.page_config.width;
        self.regions.current.cursor_x = Pt(self.page_config.margin);
        self.regions.current.line_start_x = Pt(self.page_config.margin);
    }

    /// **P538c** — avança o cursor para a coluna seguinte da mesma página.
    fn start_next_column(&mut self) {
        self.current_column += 1;
        self.start_column(self.current_column);
    }

    /// **P538c** — configura a região actual para a coluna `idx`.
    fn start_column(&mut self, idx: usize) {
        let count = self.page_columns.unwrap_or(1);
        if idx >= count {
            return;
        }
        self.column_origin_x = self.column_x_offsets[idx];
        self.regions.current.width = self.column_width;
        self.regions.current.cursor_x = Pt(self.page_config.margin);
        self.regions.current.line_start_x = Pt(self.page_config.margin);
        let (ascender, _) = self.metrics.vertical_metrics(self.style.size);
        self.regions.current.cursor_y = Pt(self.page_config.margin) + ascender;
        self.regions.current.current_line.clear();
    }

    /// **P245 (M9d / M7+4)** — flush dos floats pendentes na página
    /// actual. Top floats stack do topo; bottom stack do fundo;
    /// alignment.x aplica-se horizontalmente. `floats_pending.clear()`
    /// após emissão.
    pub(super) fn flush_pending_floats(&mut self) {
        if self.floats_pending.is_empty() {
            return;
        }
        use crate::entities::layout_types::{Align2D, HAlign, VAlign, FrameItem, Point};
        let margin    = self.page_config.margin;
        let page_w    = self.regions.current.width;
        let page_h    = self.regions.current.height;
        let avail_w   = page_w - 2.0 * margin;
        let area_top  = margin;
        let area_bot  = page_h - margin;

        let floats: Vec<DeferredFloat> = std::mem::take(&mut self.floats_pending);

        // Separar top floats (alignment.v == Top) vs outros (default
        // bottom paridade vanilla).
        let (mut top_floats, mut bot_floats): (Vec<_>, Vec<_>) = floats
            .into_iter()
            .partition(|f| matches!(f.alignment.v, Some(VAlign::Top)));

        // Stack top floats do topo para baixo (cursor_y_top start area_top).
        let mut y_top_cursor = area_top;
        for f in top_floats.drain(..) {
            let f_y = y_top_cursor;
            self.emit_deferred_float(&f, f_y, margin, avail_w);
            y_top_cursor += f.body_height + f.clearance;
        }

        // Stack bottom floats do fundo para cima (cursor_y_bot start area_bot).
        // Clearance afasta float do fundo (e do float seguinte stack-up).
        let mut y_bot_cursor = area_bot;
        for f in bot_floats.drain(..) {
            y_bot_cursor -= f.clearance + f.body_height;
            let f_y = y_bot_cursor;
            self.emit_deferred_float(&f, f_y, margin, avail_w);
        }

        let _ = Align2D { h: None::<HAlign>, v: None::<VAlign> }; // marker import use
        let _ = FrameItem::Group { pos: Point { x: Pt(0.0), y: Pt(0.0) }, matrix:
            crate::entities::layout_types::TransformMatrix::identity(),
            clip_mask: None, inner_width: 0.0, inner_height: 0.0, items: Vec::new() }; // marker
    }

    /// **P245 (M9d / M7+4)** — emite um `DeferredFloat` na posição
    /// final calculada. Aplica `alignment.x` para posicionamento
    /// horizontal dentro da largura útil da página. Translada items
    /// locais (origem 0,0 + ascender) para coordenadas finais.
    fn emit_deferred_float(
        &mut self,
        f: &DeferredFloat,
        target_y: f64,
        margin: f64,
        avail_w: f64,
    ) {
        use crate::entities::layout_types::{FrameItem, Point, HAlign};
        // `layout_sub_frame_with_width` posicionou items com ascender
        // offset (cursor_y = ascender inicial). Para alinhar shapes ao
        // target_y final exacto (não baseline), subtrair ascender do
        // offset de translação — paridade pattern `layout_place`
        // (placement.rs).
        let (ascender, _) = self.metrics.vertical_metrics(self.style.size);
        let target_y = target_y - ascender.0;
        // Calcular X conforme alignment.x.
        let x_offset = match f.alignment.h {
            Some(HAlign::Center) => (avail_w - f.body_width) / 2.0,
            Some(HAlign::Right) | Some(HAlign::End) => avail_w - f.body_width,
            _ => 0.0, // None / Left / Start default.
        };
        let target_x = margin + x_offset.max(0.0);

        // Translate items: cada item ganha offset (target_x, target_y).
        for item in &f.body_items {
            let translated = match item.clone() {
                FrameItem::Text { pos, text, style } => FrameItem::Text {
                    pos: Point { x: pos.x + Pt(target_x), y: pos.y + Pt(target_y) },
                    text, style,
                },
                FrameItem::TextShaped { pos, glyphs, style, text, units_per_em } => FrameItem::TextShaped {
                    pos: Point { x: pos.x + Pt(target_x), y: pos.y + Pt(target_y) },
                    glyphs, style, text, units_per_em,
                },
                FrameItem::Shape { pos, kind, width, height, fill, stroke, parent_bbox_at_emit } =>
                    FrameItem::Shape {
                        pos: Point { x: pos.x + Pt(target_x), y: pos.y + Pt(target_y) },
                        kind, width, height, fill, stroke,
                        parent_bbox_at_emit,
                    },
                FrameItem::Group { pos, matrix, clip_mask, inner_width, inner_height, items } =>
                    FrameItem::Group {
                        pos: Point { x: pos.x + Pt(target_x), y: pos.y + Pt(target_y) },
                        matrix, clip_mask, inner_width, inner_height, items,
                    },
                FrameItem::Line { start, end, thickness, color } => FrameItem::Line {
                    start: Point { x: start.x + Pt(target_x), y: start.y + Pt(target_y) },
                    end:   Point { x: end.x   + Pt(target_x), y: end.y   + Pt(target_y) },
                    thickness,
                    // P285: cursor reflector preserva cor (translação).
                    color,
                },
                FrameItem::Glyph { pos, glyph_id, x_advance, size } =>
                    FrameItem::Glyph {
                        pos: Point { x: pos.x + Pt(target_x), y: pos.y + Pt(target_y) },
                        glyph_id, x_advance, size,
                    },
                FrameItem::Image { pos, data, width, height, intrinsic_width, intrinsic_height } =>
                    FrameItem::Image {
                        pos: Point { x: pos.x + Pt(target_x), y: pos.y + Pt(target_y) },
                        data, width, height, intrinsic_width, intrinsic_height,
                    },
                FrameItem::Link { target, items, pos, size } => FrameItem::Link {
                    target,
                    items: items.into_iter().map(|child| {
                        let (ix, iy) = item_pos(&child);
                        translate_frame_item(child, Pt(target_x + ix), Pt(target_y + iy))
                    }).collect(),
                    pos: Point { x: pos.x + Pt(target_x), y: pos.y + Pt(target_y) },
                    size,
                },
            };
            self.regions.current.current_items.push(translated);
        }
    }

    /// **P304 (P295.1)** — flush dos footnote bodies pendentes no
    /// rodapé da página actual. Cada body é layoutado num sub-frame
    /// (`layout_sub_frame_with_width`) e posicionado em Y absoluto
    /// bottom-up: a primeira footnote fica imediatamente acima do
    /// limite inferior `page_h - margin`, a segunda abaixo dela, etc.
    /// Marker `[N]: ` prepende cada body para identificação.
    ///
    /// **P305 (P295.2)** — overflow multi-página: bodies que não
    /// cabem no espaço disponível ficam no buffer; próximo
    /// `new_page()` (ou loop em `finish()`) faz flush deles na
    /// próxima página. Bug latente P304 (overlap silencioso quando
    /// `total_h > available_h`) fixado via greedy fit + defer.
    /// Fallback defensivo: se body único > página inteira, emite
    /// na mesma para evitar loop infinito (paralelo P251 `forwarded_count`
    /// limit).
    ///
    /// Subpadrão "DeferredX buffer + flush em new_page" N=3 cumulativo
    /// (P245 floats + P251 cell tails + P304 footnotes; P305 estende
    /// com cross-page partial drain).
    /// Flush das footnotes pendentes.
    ///
    /// `bottom_y` permite forçar a coordenada Y inferior da área de
    /// footnotes (usado por `columns.rs` quando as notas de um contentor
    /// `#columns()` devem ser empilhadas abaixo do conteúdo, não no fundo
    /// da página).
    pub(super) fn flush_pending_footnote_bodies(&mut self, bottom_y: Option<f64>) {
        use crate::entities::content::Content;
        if self.pending_footnote_bodies.is_empty() {
            return;
        }
        let bodies: Vec<(u32, Box<Content>)> =
            std::mem::take(&mut self.pending_footnote_bodies);
        let margin   = self.page_config.margin;
        let page_w   = self.regions.current.width;
        let page_h   = self.regions.current.height;
        // P537 — em modo coluna as notas são posicionadas em coordenadas
        // "locais da coluna" (origem no canto superior-esquerdo útil da
        // coluna, i.e. `margin` de offset). O arquivo `columns.rs` depois
        // translada todos os items da coluna para `column_origin_x`.
        let (avail_w, left_x) = if self.column_mode {
            (self.column_width - 2.0 * margin, margin)
        } else {
            (page_w - 2.0 * margin, margin)
        };
        let area_bot = bottom_y.unwrap_or(page_h - margin);

        // P305 — compute top boundary safe: max Y of current_items
        // (above which bodies would overlap main content). Fallback
        // a `margin` se página vazia.
        let top_safe = self.regions.current.current_items.iter()
            .map(|it| match it {
                FrameItem::Text        { pos, .. } => pos.y.0,
                FrameItem::TextShaped  { pos, .. } => pos.y.0,
                FrameItem::Line  { start, .. } => start.y.0,
                FrameItem::Glyph { pos, .. } => pos.y.0,
                FrameItem::Image { pos, .. } => pos.y.0,
                FrameItem::Shape { pos, .. } => pos.y.0,
                FrameItem::Group { pos, .. } => pos.y.0,
                FrameItem::Link { .. }       => 0.0,
            })
            .fold(margin, f64::max);
        let available_h = (area_bot - top_safe).max(0.0);

        // P305 — greedy measure-then-fit. Bodies que cabem ficam
        // measured (place na pass 2); restantes diferidos para
        // próxima página via `remainder`. Fallback defensivo
        // refinado: emite primeiro body apenas se for maior que
        // a área de conteúdo completa (i.e., não cabe em nenhuma
        // página) — evita loop infinito sem forçar overlap em
        // partial-page overflow normal. Paralelo P251 `forwarded_count`
        // limit.
        let full_avail = (page_h - 2.0 * margin).max(0.0);
        let mut measured: Vec<(f64, Vec<FrameItem>)> = Vec::new();
        let mut acc_h = 0.0_f64;
        let mut remainder: Vec<(u32, Box<Content>)> = Vec::new();
        let mut overflow = false;
        for (n, body) in bodies.into_iter() {
            if overflow {
                remainder.push((n, body));
                continue;
            }
            let combined = Content::sequence(vec![
                Content::text(format!("[{}] ", n)),
                (*body).clone(),
            ]);
            let (h, items) = self.layout_sub_frame_with_width(&combined, 0.0, avail_w);
            let fits = acc_h + h <= available_h;
            // Defensive: primeiro body emite mesmo se > available_h SE
            // body > full_avail (não fits em nenhuma página).
            let force_emit = measured.is_empty() && h > full_avail;
            if fits || force_emit {
                acc_h += h;
                measured.push((h, items));
            } else {
                overflow = true;
                remainder.push((n, body));
            }
        }

        // Pass 2 — place top-down a partir de `area_bot - acc_h`.
        // Primeira footnote no topo da zona; última no fundo.
        // `layout_sub_frame_with_width` posicionou items com ascender
        // offset (cursor_y inicial = ascender). Para alinhar ao
        // target_y absoluto exacto (não baseline), subtrair ascender
        // do offset de translação — paridade pattern `emit_deferred_float`
        // (P245) + `layout_place` (placement.rs).
        let (ascender, _) = self.metrics.vertical_metrics(self.style.size);
        // P305 — clamp Y inicial ao top_safe para evitar overlap em
        // defensive emit (body > full_avail). Se acc_h ≤ available_h,
        // clamp é no-op (area_bot - acc_h ≥ top_safe por construção).
        let mut y_cursor = (area_bot - acc_h).max(top_safe);
        for (h, items) in measured {
            let target_y = y_cursor - ascender.0;
            let target_x = left_x;
            for item in items {
                let translated = match item {
                    FrameItem::Text { pos, text, style } => FrameItem::Text {
                        pos: Point { x: pos.x + Pt(target_x), y: pos.y + Pt(target_y) },
                        text, style,
                    },
                    FrameItem::TextShaped { pos, glyphs, style, text, units_per_em } => FrameItem::TextShaped {
                        pos: Point { x: pos.x + Pt(target_x), y: pos.y + Pt(target_y) },
                        glyphs, style, text, units_per_em,
                    },
                    FrameItem::Shape { pos, kind, width, height, fill, stroke, parent_bbox_at_emit } =>
                        FrameItem::Shape {
                            pos: Point { x: pos.x + Pt(target_x), y: pos.y + Pt(target_y) },
                            kind, width, height, fill, stroke, parent_bbox_at_emit,
                        },
                    FrameItem::Group { pos, matrix, clip_mask, inner_width, inner_height, items } =>
                        FrameItem::Group {
                            pos: Point { x: pos.x + Pt(target_x), y: pos.y + Pt(target_y) },
                            matrix, clip_mask, inner_width, inner_height, items,
                        },
                    FrameItem::Line { start, end, thickness, color } => FrameItem::Line {
                        start: Point { x: start.x + Pt(target_x), y: start.y + Pt(target_y) },
                        end:   Point { x: end.x   + Pt(target_x), y: end.y   + Pt(target_y) },
                        thickness, color,
                    },
                    FrameItem::Glyph { pos, glyph_id, x_advance, size } =>
                        FrameItem::Glyph {
                            pos: Point { x: pos.x + Pt(target_x), y: pos.y + Pt(target_y) },
                            glyph_id, x_advance, size,
                        },
                    FrameItem::Image { pos, data, width, height, intrinsic_width, intrinsic_height } =>
                        FrameItem::Image {
                            pos: Point { x: pos.x + Pt(target_x), y: pos.y + Pt(target_y) },
                            data, width, height, intrinsic_width, intrinsic_height,
                        },
                    FrameItem::Link { target, items, pos, size } => FrameItem::Link {
                        target,
                        items: items.into_iter().map(|child| {
                            let (ix, iy) = item_pos(&child);
                            translate_frame_item(child, Pt(target_x + ix), Pt(target_y + iy))
                        }).collect(),
                        pos: Point { x: pos.x + Pt(target_x), y: pos.y + Pt(target_y) },
                        size,
                    },
                };
                self.regions.current.current_items.push(translated);
            }
            y_cursor += h;
        }

        // P305 — re-inserir remainder no buffer para flush na
        // próxima página. `new_page()` chama este método; iteração
        // automática até buffer vazio. `finish()` itera explicitamente
        // via loop com new_page() se buffer não-vazio pós-flush final.
        self.pending_footnote_bodies = remainder;
    }

    /// Número da página actual (1-indexed).
    ///
    /// Abordagem A: `self.pages.len() + 1` — a página actual ainda não foi
    /// finalizada (não foi empurrada para `self.pages`), por isso a contagem
    /// de páginas finalizadas + 1 dá o número da página em curso.
    pub(super) fn current_page_number(&self) -> usize {
        self.pages.len() + 1
    }

    /// **P251 (M9d / M7+5; ADR-0079 Categoria C.2 parcial; cita
    /// ADR-0082 PROPOSTO N=2 segunda aplicação citante)** — flush
    /// dos cell tails pendentes no **topo** da página actual (chamado
    /// por `new_page` após cursor_y setup). Items rebased pelo
    /// `cursor_y` actual; cursor_y avança pela altura do tail emitido.
    ///
    /// Z-order paridade P248: fill atrás → items → stroke à frente.
    /// Bounds do fill/stroke usam tail extent (não cell original).
    ///
    /// Limit 3 forwardings consecutivos (paridade vanilla heurística;
    /// `forwarded_count >= 3` descarta silenciosamente — mitigação
    /// loop infinito caso tail recursivo).
    pub(super) fn flush_pending_cell_tails(&mut self) {
        use crate::entities::geometry::ShapeKind;
        use crate::entities::layout_types::{FrameItem, Point};
        use crate::rules::layout::slicing::rebase_item_y;
        if self.pending_cell_tails.is_empty() {
            return;
        }
        let tails: Vec<crate::rules::layout::DeferredCellTail> =
            std::mem::take(&mut self.pending_cell_tails);
        let cursor_top = self.regions.current.cursor_y.0;
        let mut max_y_after = cursor_top;
        for tail in tails {
            if tail.forwarded_count >= 3 {
                // P251 — limit forwarding; descarta silenciosamente
                // (paridade vanilla heurística max-iter).
                continue;
            }
            // Calcula altura do tail (max pos.y dos items locais).
            let mut tail_h = 0.0_f64;
            for item in tail.items.iter() {
                let y = match item {
                    FrameItem::Text        { pos, .. } => pos.y.0,
                    FrameItem::TextShaped  { pos, .. } => pos.y.0,
                    FrameItem::Line  { start, .. } => start.y.0,
                    FrameItem::Glyph { pos, .. } => pos.y.0,
                    FrameItem::Image { pos, .. } => pos.y.0,
                    FrameItem::Shape { pos, .. } => pos.y.0,
                    FrameItem::Group { pos, .. } => pos.y.0,
                    FrameItem::Link { .. }       => 0.0,
                };
                tail_h = tail_h.max(y);
            }
            // Z-order step 1: fill atrás.
            if let Some(c) = tail.fill {
                self.regions.current.current_items.push(FrameItem::Shape {
                    pos:    Point { x: Pt(tail.origin_x), y: Pt(cursor_top) },
                    kind:   ShapeKind::Rect,
                    width:  tail.width,
                    height: tail_h,
                    fill:   Some(c),
                    stroke: None,
                    parent_bbox_at_emit: None,
                });
            }
            // Z-order step 2: items rebased.
            for item in tail.items {
                let final_item = rebase_item_y(item, cursor_top);
                self.regions.current.current_items.push(final_item);
            }
            // Z-order step 3: stroke à frente.
            if let Some(s) = tail.stroke {
                self.regions.current.current_items.push(FrameItem::Shape {
                    pos:    Point { x: Pt(tail.origin_x), y: Pt(cursor_top) },
                    kind:   ShapeKind::Rect,
                    width:  tail.width,
                    height: tail_h,
                    fill:   None,
                    stroke: Some(s),
                    parent_bbox_at_emit: None,
                });
            }
            max_y_after = max_y_after.max(cursor_top + tail_h);
        }
        // Avança cursor_y para depois dos tails emitidos.
        self.regions.current.cursor_y = Pt(max_y_after);
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {
        // V2 smoke test — submódulo extraído no Passo 96.7 (ADR-0037).
        // A cobertura funcional vive em `layout/tests.rs`.
    }
}
