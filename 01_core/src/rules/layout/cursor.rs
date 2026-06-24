//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout.md
//! @prompt-hash 12536b5c
//! @layer L1
//! @updated 2026-04-23
//!
//! Gestão do cursor do Layouter: largura de palavra, layout de palavra,
//! `flush_line`, `new_page`, número de página actual.
//! Extraído de `layout/mod.rs` no Passo 96.7 conforme ADR-0037.

use crate::entities::{
    image_sizer::ImageSizer,
    layout_types::{FrameItem, Page, Point, Pt},
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
        let base = self.metrics.advance(word, self.style.size);
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
        self.metrics.advance(" ", self.style.size)
    }

    pub(super) fn layout_word(&mut self, word: &str) {
        let w = self.word_width(word);
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
                            self.regions.current.current_line.push(FrameItem::Text {
                                pos:   Point { x: self.regions.current.cursor_x, y: self.regions.current.cursor_y },
                                text:  prefix_with_hyphen.into(),
                                style: self.style.clone(),
                            });
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
        self.regions.current.current_line.push(FrameItem::Text {
            pos:   Point { x: self.regions.current.cursor_x, y: self.regions.current.cursor_y },
            text:  word.into(),
            style: self.style.clone(),
        });
        self.regions.current.cursor_x += w + self.space_width();
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
        self.regions.current.current_line.push(FrameItem::Text {
            pos:   Point { x: self.regions.current.cursor_x, y: self.regions.current.cursor_y },
            text:  chunk.into(),
            style: self.style.clone(),
        });
        self.regions.current.cursor_x += w;
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

        // Passo 138 (Fase B.2 DEBT-52): consumer leading.
        // `self.style` pode ter sido restaurado ao outer scope antes de
        // flush_line ser chamado (ver arm Content::Text em layout/mod.rs
        // que faz `self.style = prev_style` após layout_word). Em vez
        // disso, peek no último FrameItem::Text da current_line — o seu
        // `.style.leading` é o valor efectivo do baseline.
        //
        // Fórmula (opt soma): `line_height = default + user_leading`.
        let line_leading_pt = self.regions.current.current_line
            .iter()
            .rev()
            .find_map(|item| match item {
                crate::entities::layout_types::FrameItem::Text { style, .. } => {
                    style.leading.map(|l| l.resolve_pt(self.font_size_pt.val()))
                }
                _ => None,
            })
            .unwrap_or(0.0);

        for item in self.regions.current.current_line.drain(..) {
            self.regions.current.current_items.push(item);
        }
        if had_items {
            let (_, line_height) = self.metrics.vertical_metrics(self.font_size_pt);
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
        // P245 (M9d / M7+4) — flush floats pendentes na página actual
        // antes da transição. Top floats emit no topo, bottom no fundo.
        self.flush_pending_floats();

        // P304 (P295.1) — flush footnote bodies pendentes no rodapé
        // antes de saving a Page. Items posicionados em Y absoluto
        // bottom-up; tornam-se parte dos `current_items` da página.
        self.flush_pending_footnote_bodies();

        let page = Page {
            width:  self.regions.current.width,
            height: self.regions.current.height,
            items:  std::mem::take(&mut self.regions.current.current_items),
        };
        self.pages.push(page);
        self.regions.current.cursor_x = Pt(self.page_config.margin);
        self.regions.current.line_start_x = Pt(self.page_config.margin);
        let (ascender, _) = self.metrics.vertical_metrics(self.font_size_pt);
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
        let (ascender, _) = self.metrics.vertical_metrics(self.font_size_pt);
        let target_y = target_y - ascender.0;
        // Calcular X conforme alignment.x.
        let x_offset = match f.alignment.h {
            Some(HAlign::Center) => (avail_w - f.body_width) / 2.0,
            Some(HAlign::Right)  => avail_w - f.body_width,
            _                    => 0.0, // None → Left default.
        };
        let target_x = margin + x_offset.max(0.0);

        // Translate items: cada item ganha offset (target_x, target_y).
        for item in &f.body_items {
            let translated = match item.clone() {
                FrameItem::Text { pos, text, style } => FrameItem::Text {
                    pos: Point { x: pos.x + Pt(target_x), y: pos.y + Pt(target_y) },
                    text, style,
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
                FrameItem::Link { url, items, pos, size } => FrameItem::Link {
                    url,
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
    pub(super) fn flush_pending_footnote_bodies(&mut self) {
        use crate::entities::content::Content;
        if self.pending_footnote_bodies.is_empty() {
            return;
        }
        let bodies: Vec<(u32, Box<Content>)> =
            std::mem::take(&mut self.pending_footnote_bodies);
        let margin   = self.page_config.margin;
        let page_w   = self.regions.current.width;
        let page_h   = self.regions.current.height;
        let avail_w  = page_w - 2.0 * margin;
        let area_bot = page_h - margin;

        // P305 — compute top boundary safe: max Y of current_items
        // (above which bodies would overlap main content). Fallback
        // a `margin` se página vazia.
        let top_safe = self.regions.current.current_items.iter()
            .map(|it| match it {
                FrameItem::Text  { pos, .. } => pos.y.0,
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
        let (ascender, _) = self.metrics.vertical_metrics(self.font_size_pt);
        // P305 — clamp Y inicial ao top_safe para evitar overlap em
        // defensive emit (body > full_avail). Se acc_h ≤ available_h,
        // clamp é no-op (area_bot - acc_h ≥ top_safe por construção).
        let mut y_cursor = (area_bot - acc_h).max(top_safe);
        for (h, items) in measured {
            let target_y = y_cursor - ascender.0;
            let target_x = margin;
            for item in items {
                let translated = match item {
                    FrameItem::Text { pos, text, style } => FrameItem::Text {
                        pos: Point { x: pos.x + Pt(target_x), y: pos.y + Pt(target_y) },
                        text, style,
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
                    FrameItem::Link { url, items, pos, size } => FrameItem::Link {
                        url,
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
                    FrameItem::Text  { pos, .. } => pos.y.0,
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
