//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/layout.md
//! @prompt-hash 3127a867
//! @layer L1
//! @updated 2026-04-23
//!
//! Braços `Content::Align` e `Content::Place` do `layout_content`.
//! Extraídos de `layout/mod.rs` no Passo 96.7 conforme ADR-0037.

use crate::entities::{
    content::Content,
    image_sizer::ImageSizer,
    layout_types::{Align2D, PlaceScope, Pt, VAlign},
};

use super::metrics::FontMetrics;
use super::{item_pos, measure_content, translate_frame_item};

impl<'a, M: FontMetrics, S: ImageSizer> super::Layouter<'a, M, S> {
    /// Layout de `Content::Align { alignment, body }`.
    pub(super) fn layout_align(&mut self, alignment: Align2D, body: &Content) {
        // Garantir que não há texto inline pendente antes de posicionar o bloco.
        // flush_line usa line_start_x (Passo 81.5).
        self.flush_line();

        let avail_w = self.available_width();

        // P772g (encerra achado B de P772f) — origin_x real (não 0.0):
        // `layout_align` é um "consumidor absoluto" de `layout_sub_frame`
        // (ver 00_nucleo/prompts/engine/layout.md §"Contrato de composição
        // de coordenadas"). Um `Content::Place` aninhado no corpo emite
        // coordenadas absolutas usando `regions.cell`/`cell_origin_x`; se
        // este sub-frame usasse origin_x=0.0 e depois somasse o `target_x`
        // inteiro (como acontecia antes), a origem da célula seria somada
        // duas vezes. Ao inicializar o sub-frame já na origem real, o
        // conteúdo normal (texto/formas) fica com posições absolutas desde
        // o início; só falta somar o deslocamento *incremental* do
        // alinhamento (`delta_x` abaixo), não o `target_x` inteiro.
        let origin_x_abs = self.regions.current.line_start_x.0;
        let (sub_h, sub_items, sub_deco) = self.layout_sub_frame(
            body,
            super::sub_frame::SubLayoutRegion {
                origin_x: origin_x_abs,
                width: avail_w,
                height: None,
                align_rtl: false,
                unconstrained_height: true,
            },
        );

        // Origem vertical local do sub-frame (ascender). Necessária para
        // rebaser as coordenadas Y ao colocar no frame pai.
        let (ascender_local, _) =
            self.metrics.vertical_metrics(self.style.size, &self.style);
        let sub_origin_y = ascender_local.0;

        // P772j — largura do conteúdo medida a partir dos `sub_items` já
        // layoutados (via `FontMetrics::line_content_right`, o mesmo
        // mecanismo usado por `measure_content_real`), não do helper
        // `measure_content` sem acesso a métricas reais. `measure_content`
        // (`helpers.rs`) só tem braços para `Content::Shape`/
        // `Content::Sequence` — para `Content::Text` (o caso comum de
        // `align(center, [texto])`) devolvia sempre `(0.0, 0.0)`, fazendo
        // `resolve_alignment` centrar como se o conteúdo tivesse largura
        // zero (deslocamento de `avail_w/2` em vez de
        // `(avail_w - largura_real)/2`). Bug confirmado por instrumentação
        // directa durante P772j — pré-existe a este passo (afecta qualquer
        // `align()` com texto, não só alinhamento de célula de grid) mas
        // bloqueia directamente a validação do achado deste passo;
        // corrigido aqui por ser pequeno e usar um mecanismo já existente,
        // não uma nova aproximação. `measure_content` fica inalterado —
        // continua a servir `Content::Place`/`Content::Transform` (fora do
        // âmbito de P772j; ver 00_nucleo/prompts/engine/layout.md).
        let sub_item_refs: Vec<&crate::entities::layout_types::FrameItem> =
            sub_items.iter().collect();
        let content_right_abs = self.metrics.line_content_right(&sub_item_refs);
        let content_w = (content_right_abs - origin_x_abs).max(0.0);

        // Verificar quebra de página com a altura do sub-frame.
        if self.regions.current.cursor_y.0 + sub_h > self.page_bottom_limit() {
            self.new_page();
        }

        // Selecção de remaining_h e VAlign efectivo (Passo 83).
        //
        // Prioridade: cell_region > is_height_unconstrained > página.
        // P246 — `cell_available_h` migrado para `regions.cell.height`.
        // Dentro de uma célula de Grid (`regions.cell = Some(_)`), a
        // altura é conhecida — Bottom e Horizon ancoram à célula.
        // No fluxo livre (sub_frame sem cell), Bottom/Horizon decaem
        // para Top (sem "fundo" para ancorar).
        // No fluxo normal da página, usar o espaço restante até à margem.
        let (remaining_h, effective_v) = if let Some(cell) = self.regions.cell.as_ref() {
            (cell.height, alignment.v)
        } else if self.is_height_unconstrained {
            (sub_h, None)
        } else {
            let space =
                f64::max(0.0, self.page_bottom_limit() - self.regions.current.cursor_y.0);
            (space, alignment.v)
        };

        let effective_align = Align2D { h: alignment.h, v: effective_v };

        // origin_x = line_start_x (não page_config.margin). Dentro de uma
        // célula de grid, line_start_x é cell_x, não a margem da página.
        let (target_x, target_y) = self.resolve_alignment(
            effective_align,
            content_w,
            sub_h,
            avail_w,
            remaining_h,
            self.regions.current.line_start_x.0,
            self.regions.current.cursor_y.0,
        );

        // Transferir items: P772g — sub-frame já iniciado em origin_x_abs
        // (não 0), logo os itens "normais" já vêm absolutos; soma-se só o
        // deslocamento incremental do alinhamento (`delta_x`), nunca o
        // `target_x` inteiro (evita duplicar a origem em itens de
        // `Content::Place` aninhado, que já emitem coordenadas absolutas).
        // sub_origin_y = ascender_local (compensar a origem vertical, Y
        // inalterado por P772g — ver nota no L0).
        let delta_x = target_x - origin_x_abs;
        for item in sub_items {
            let (ix, iy) = item_pos(&item);
            let new_x = Pt(ix + delta_x);
            let new_y = Pt(target_y + iy - sub_origin_y);
            self.regions
                .current
                .current_items
                .push(translate_frame_item(item, new_x, new_y));
        }
        // **P772x** — mesma translação para segmentos de decoração do body.
        if let Some(coll) = self.decoration_lines_collector.as_mut() {
            for seg in &sub_deco {
                coll.push(super::DecoSegment {
                    start_x: Pt(seg.start_x.val() + delta_x),
                    end_x: Pt(seg.end_x.val() + delta_x),
                    baseline_y: Pt(target_y + seg.baseline_y.val() - sub_origin_y),
                });
            }
        }

        // Avançar cursor Y.
        //
        // Dentro de uma célula (cell_available_h = Some), o avanço é
        // governado pelo Grid (que repõe cursor_y após emitir cada
        // linha). Manter o cursor próximo do conteúdo (target_y + sub_h)
        // permite que conteúdo subsequente da mesma célula siga abaixo
        // sem saltar para o fundo.
        //
        // No fluxo de página, VAlign::Horizon/Bottom consomem o resto.
        // P246 — `cell_available_h.is_some()` migrado para
        // `regions.cell.is_some()`.
        match (self.regions.cell.is_some(), effective_v) {
            (false, Some(VAlign::Horizon)) | (false, Some(VAlign::Bottom)) => {
                self.regions.current.cursor_y = Pt(self.page_bottom_limit());
            }
            _ => {
                self.regions.current.cursor_y = Pt(target_y + sub_h);
            }
        }
    }

    /// Layout de `Content::Place { alignment, dx, dy, scope, body }`.
    pub(super) fn layout_place(
        &mut self,
        alignment: Align2D,
        dx: f64,
        dy: f64,
        scope: PlaceScope,
        body: &Content,
    ) {
        // Place NÃO chama flush_line e NÃO modifica cursor_x nem cursor_y.
        let avail_w_page = self.available_width();
        let avail_h_page = self.available_height();

        let (sub_h, sub_items, sub_deco) = self.layout_sub_frame(
            body,
            super::sub_frame::SubLayoutRegion {
                origin_x: 0.0,
                width: avail_w_page,
                height: None,
                align_rtl: false,
                unconstrained_height: true,
            },
        );

        let (ascender_local, _) =
            self.metrics.vertical_metrics(self.style.size, &self.style);
        let sub_origin_y = ascender_local.0;

        let (content_w, _) = measure_content(body, avail_w_page);

        // Passo 84.6 (encerra DEBT-37): seleccionar área de ancoragem
        // segundo `scope`.
        // - PlaceScope::Column (default): se estamos dentro de uma
        //   célula de Grid (cell_origin_x/y + regions.cell todos Some),
        //   ancorar à célula. Caso contrário cair para a página.
        // - PlaceScope::Parent: ancorar sempre à página, mesmo dentro
        //   de Grid (paridade com vanilla — `parent` "spans columns").
        // P246 — `cell_origin_w` + `cell_available_h` migrados para
        // `regions.cell.width/height`; `cell_origin_x/y` preservados
        // como Layouter fields legacy.
        // P763f — quando `place` é executado dentro de um sub-frame (ex:
        // `align(top, place(...))` dentro de um `block`), as coordenadas de
        // ancoragem devem ser relativas ao próprio sub-frame, não à página.
        // Caso contrário o `origin_y = page_config.margin` desloca o conteúdo
        // para baixo pelo valor da margem dentro do referencial local.
        let in_sub_frame = self.is_sub_frame;

        let (origin_x, origin_y, avail_w, avail_h, y_offset) = match scope {
            PlaceScope::Column => {
                match (self.cell_origin_x, self.cell_origin_y, self.regions.cell.as_ref())
                {
                    (Some(cx), Some(cy), Some(cell)) => {
                        (cx, cy, cell.width, cell.height, cy)
                    }
                    _ => {
                        if in_sub_frame {
                            (0.0, 0.0, avail_w_page, avail_h_page, 0.0)
                        } else {
                            (
                                self.regions.current.line_start_x.0,
                                self.page_config.margin,
                                avail_w_page,
                                avail_h_page,
                                sub_origin_y,
                            )
                        }
                    }
                }
            }
            PlaceScope::Parent => {
                if in_sub_frame {
                    (0.0, 0.0, avail_w_page, avail_h_page, 0.0)
                } else {
                    (
                        self.page_config.margin,
                        self.page_config.margin,
                        avail_w_page,
                        avail_h_page,
                        sub_origin_y,
                    )
                }
            }
        };

        let (base_x, base_y) = self.resolve_alignment(
            alignment, content_w, sub_h, avail_w, avail_h, origin_x, origin_y,
        );

        let target_x = base_x + dx;
        let target_y = base_y + dy;

        for item in sub_items {
            let (ix, iy) = item_pos(&item);
            let new_x = Pt(target_x + ix);
            let new_y = Pt(target_y + iy - y_offset);
            self.regions
                .current
                .current_items
                .push(translate_frame_item(item, new_x, new_y));
        }
        // **P772x** — mesma translação para segmentos de decoração do body
        // (repro original de P772w: `#underline[.. #place(..)[explanation]]`).
        if let Some(coll) = self.decoration_lines_collector.as_mut() {
            for seg in &sub_deco {
                coll.push(super::DecoSegment {
                    start_x: Pt(target_x + seg.start_x.val()),
                    end_x: Pt(target_x + seg.end_x.val()),
                    baseline_y: Pt(target_y + seg.baseline_y.val() - y_offset),
                });
            }
        }
        // cursor_y e cursor_x ficam intocados — Place não consome espaço.
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
