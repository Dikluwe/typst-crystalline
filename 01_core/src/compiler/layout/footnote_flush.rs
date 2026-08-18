//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/footnote_overflow_columns.md
//! @layer L1
//! @updated 2026-07-23
//!
//! Flush dos footnote bodies pendentes (`flush_pending_footnote_bodies`):
//! greedy measure-then-fit, defer cross-page, aviso quando o body excede
//! a página/coluna. Extraído de `layout/cursor.rs` no P847 (um ficheiro,
//! um prompt).
#![allow(deprecated)] // P483 — FrameItem::Text fallback path legítimo
use crate::entities::{
    image_sizer::ImageSizer,
    layout_types::{FrameItem, Point, Pt},
};

use super::helpers::{item_pos, translate_frame_item};
use super::metrics::FontMetrics;

impl<'a, M: FontMetrics, S: ImageSizer> super::Layouter<'a, M, S> {
    /// **P304 (P295.1)** — flush dos footnote bodies pendentes no
    /// rodapé da página actual. Cada body é layoutado num sub-frame
    /// (`layout_sub_frame`) e posicionado em Y absoluto
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
        let margin = self.page_config.margin;
        let page_w = self.regions.current.width;
        let page_h = self.regions.current.height;
        // P537 — em modo coluna as notas são posicionadas em coordenadas
        // "locais da coluna" (origem no canto superior-esquerdo útil da
        // coluna, i.e. `margin` de offset). O arquivo `columns.rs` depois
        // translada todos os items da coluna para `column_origin_x`.
        let (avail_w, left_x) = if self.column_mode {
            // rationale: PageConfig::margin é escalar único (f64) — left=right=top=bottom por definição do tipo (entities/layout_types.rs). 2.0 * margin é verdade algébrica estrutural. P1066.
            (self.column_width - 2.0 * margin, margin)
        } else {
            // rationale: PageConfig::margin é escalar único (f64) — left=right=top=bottom por definição do tipo (entities/layout_types.rs). 2.0 * margin é verdade algébrica estrutural. P1066.
            (page_w - 2.0 * margin, margin)
        };
        let area_bot = bottom_y.unwrap_or(page_h - margin);

        // P305 — compute top boundary safe: max Y of current_items
        // (above which bodies would overlap main content). Fallback
        // a `margin` se página vazia.
        // **P595** — incluir cursor_y e current_line no cálculo, porque
        // o texto ainda não flushado para current_items também ocupa
        // espaço; ignorá-lo faz a nota sobrepor o conteúdo principal
        // (especialmente em colunas).
        let item_top_y = |it: &FrameItem| match it {
            FrameItem::Text { pos, .. } => pos.y.0,
            FrameItem::TextShaped { pos, .. } => pos.y.0,
            FrameItem::Line { start, .. } => start.y.0,
            FrameItem::Glyph { pos, .. } => pos.y.0,
            FrameItem::Image { pos, .. } => pos.y.0,
            FrameItem::Shape { pos, .. } => pos.y.0,
            FrameItem::Group { pos, .. } => pos.y.0,
            FrameItem::Link { .. } => 0.0,
        };
        let top_safe = self
            .regions
            .current
            .current_items
            .iter()
            .map(item_top_y)
            .chain(self.regions.current.current_line.iter().map(item_top_y))
            .fold(margin, f64::max)
            .max(self.regions.current.cursor_y.0);
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
        #[allow(clippy::type_complexity)]
        let mut measured: Vec<(
            f64,
            Vec<FrameItem>,
            Vec<super::DecoSegment>,
            Vec<super::PendingAlignXEntry>,
            Vec<super::PendingAlignYEntry>,
        )> = Vec::new();
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
            // P772g (encerra achado B de P772f) — origin_x real (`left_x`,
            // não 0.0): esta emissão é um "consumidor absoluto" de
            // `layout_sub_frame` (ver 00_nucleo/prompts/compiler/layout.md
            // §"Contrato de composição de coordenadas"). Um `Content::Place`
            // aninhado no body da nota emite coordenadas absolutas via
            // `regions.cell`/`cell_origin_x`; se este sub-frame usasse
            // origin_x=0.0 e depois somasse `target_x=left_x` inteiro a
            // todos os itens (como acontecia antes), a origem seria somada
            // duas vezes para o `Place` aninhado.
            let (h, items, deco, orphaned_x, orphaned_y) = self.layout_sub_frame(
                &combined,
                super::sub_frame::SubLayoutRegion {
                    origin_x: left_x,
                    width: avail_w,
                    height: None,
                    align_rtl: true,
                    unconstrained_height: true,
                },
            );
            let fits = acc_h + h <= available_h;
            // Defensive: primeiro body emite mesmo se > available_h SE
            // body > full_avail (não fits em nenhuma página).
            let force_emit = measured.is_empty() && h > full_avail;
            if force_emit {
                // **P595** — body maior do que a página/coluna inteira:
                // emite-se para evitar loop infinito, mas o utilizador
                // deve ser avisado de que o conteúdo pode ser truncado
                // ou sobreposto.
                self.layout_warnings.push(format!(
                    "footnote body [{}] exceeds available column/page height and may be truncated or overlap content",
                    n
                ));
            }
            if fits || force_emit {
                acc_h += h;
                measured.push((h, items, deco, orphaned_x, orphaned_y));
            } else {
                overflow = true;
                remainder.push((n, body));
            }
        }

        // Pass 2 — place top-down a partir de `area_bot - acc_h`.
        // Primeira footnote no topo da zona; última no fundo.
        // `layout_sub_frame` posicionou items com ascender
        // offset (cursor_y inicial = ascender). Para alinhar ao
        // target_y absoluto exacto (não baseline), subtrair ascender
        // do offset de translação — paridade pattern `emit_deferred_float`
        // (P245) + `layout_place` (placement.rs).
        let (ascender, _) = self.metrics.vertical_metrics(self.style.size, &self.style);
        // P305 — clamp Y inicial ao top_safe para evitar overlap em
        // defensive emit (body > full_avail). Se acc_h ≤ available_h,
        // clamp é no-op (area_bot - acc_h ≥ top_safe por construção).
        let mut y_cursor = (area_bot - acc_h).max(top_safe);
        for (h, items, deco, orphaned_x, orphaned_y) in measured {
            // **P908** — `target_y_logical` (pré-ascender) rebaseia
            // `origin_y`/`applied_y` das entradas órfãs (quantidades sem
            // ascender embutido — mesma distinção física-vs-lógica de
            // `layout_align`/`layout_place`/`emit_deferred_float`,
            // `00_nucleo/prompts/compiler/layout.md` §P908); `target_y`
            // (pós-subtracção) só é correcto para os `FrameItem`s reais
            // abaixo, cujo `pos.y` já é baseline.
            let target_y_logical = y_cursor;
            let target_y = y_cursor - ascender.0;
            // P772g — sub-frame já iniciado em `left_x` (não 0.0, ver acima),
            // logo os itens "normais" já vêm absolutos em x; footnotes não
            // têm alinhamento horizontal próprio (sempre ancoradas a
            // `left_x`), logo o deslocamento incremental em x é 0 — mantém-se
            // a variável (agora delta, não valor absoluto) para minimizar o
            // diff no resto do bloco de tradução abaixo.
            let target_x = 0.0;
            // **P908** — rebasear as entradas órfãs devolvidas pelo
            // `layout_sub_frame` deste body (`Content::Align`/
            // `Content::Place` aninhado sob `width`/`height: auto`) com a
            // MESMA translação `(target_x, target_y)` aplicada aos items
            // reais abaixo. `path[0]` passa a apontar para a posição real
            // em `current_items` (soma o comprimento actual, capturado
            // ANTES do merge). Ver
            // `00_nucleo/prompts/compiler/layout.md` §P908.
            let insertion_base = self.regions.current.current_items.len();
            for (mut path, count, align, content_w, origin_x, applied_x) in orphaned_x {
                path[0] += insertion_base;
                self.pending_align_centering.push((
                    path,
                    count,
                    align,
                    content_w,
                    origin_x + target_x,
                    applied_x + target_x,
                ));
            }
            for (mut path, count, align, content_h, origin_y, dy, applied_y) in orphaned_y {
                path[0] += insertion_base;
                self.pending_align_v_centering.push((
                    path,
                    count,
                    align,
                    content_h,
                    origin_y + target_y_logical,
                    dy,
                    applied_y + target_y_logical,
                ));
            }
            for item in items {
                let translated = match item {
                    FrameItem::Text { pos, text, style } => FrameItem::Text {
                        pos: Point { x: pos.x + Pt(target_x), y: pos.y + Pt(target_y) },
                        text,
                        style,
                    },
                    FrameItem::TextShaped { pos, glyphs, style, text, units_per_em } => {
                        FrameItem::TextShaped {
                            pos: Point {
                                x: pos.x + Pt(target_x),
                                y: pos.y + Pt(target_y),
                            },
                            glyphs,
                            style,
                            text,
                            units_per_em,
                        }
                    }
                    FrameItem::Shape {
                        pos,
                        kind,
                        width,
                        height,
                        fill,
                        stroke,
                        parent_bbox_at_emit,
                    } => FrameItem::Shape {
                        pos: Point { x: pos.x + Pt(target_x), y: pos.y + Pt(target_y) },
                        kind,
                        width,
                        height,
                        fill,
                        stroke,
                        parent_bbox_at_emit,
                    },
                    FrameItem::Group {
                        pos,
                        matrix,
                        clip_mask,
                        inner_width,
                        inner_height,
                        items,
                    } => FrameItem::Group {
                        pos: Point { x: pos.x + Pt(target_x), y: pos.y + Pt(target_y) },
                        matrix,
                        clip_mask,
                        inner_width,
                        inner_height,
                        items,
                    },
                    FrameItem::Line { start, end, thickness, color } => FrameItem::Line {
                        start: Point {
                            x: start.x + Pt(target_x),
                            y: start.y + Pt(target_y),
                        },
                        end: Point { x: end.x + Pt(target_x), y: end.y + Pt(target_y) },
                        thickness,
                        color,
                    },
                    FrameItem::Glyph { pos, glyph_id, x_advance, size, style, base_char } => {
                        FrameItem::Glyph {
                            pos: Point {
                                x: pos.x + Pt(target_x),
                                y: pos.y + Pt(target_y),
                            },
                            glyph_id,
                            x_advance,
                            size,
                            style,
                            base_char,
                        }
                    }
                    FrameItem::Image {
                        pos,
                        data,
                        width,
                        height,
                        intrinsic_width,
                        intrinsic_height,
                        orientation,
                        ..
                    } => FrameItem::Image {
                        pos: Point { x: pos.x + Pt(target_x), y: pos.y + Pt(target_y) },
                        data,
                        width,
                        height,
                        intrinsic_width,
                        intrinsic_height,
                        clip_rect: None,
                        orientation,
                    },
                    FrameItem::Link { target, items, pos, size } => FrameItem::Link {
                        target,
                        items: items
                            .into_iter()
                            .map(|child| {
                                let (ix, iy) = item_pos(&child);
                                translate_frame_item(
                                    child,
                                    Pt(target_x + ix),
                                    Pt(target_y + iy),
                                )
                            })
                            .collect(),
                        pos: Point { x: pos.x + Pt(target_x), y: pos.y + Pt(target_y) },
                        size,
                    },
                };
                self.regions.current.current_items.push(translated);
            }
            // **P772x** — mesma translação (target_x=0.0, target_y desta
            // iteração) para segmentos de decoração do body da footnote.
            if let Some(coll) = self.decoration_lines_collector.as_mut() {
                for seg in &deco {
                    coll.push(super::DecoSegment {
                        start_x: Pt(target_x + seg.start_x.val()),
                        end_x: Pt(target_x + seg.end_x.val()),
                        baseline_y: Pt(target_y + seg.baseline_y.val()),
                    });
                }
            }
            y_cursor += h;
        }

        // P305 — re-inserir remainder no buffer para flush na
        // próxima página. `new_page()` chama este método; iteração
        // automática até buffer vazio. `finish()` itera explicitamente
        // via loop com new_page() se buffer não-vazio pós-flush final.
        self.pending_footnote_bodies = remainder;
    }
}


// ── Testes ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {
        // V2 smoke test — módulo extraído de `cursor.rs` em P847 (um
        // ficheiro, um prompt: `compiler/footnote_overflow_columns.md`).
        // A cobertura funcional de footnote vive nos testes de layout e
        // nos goldens de colunas (P595+).
    }
}
