//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout.md
//! @prompt-hash 20d03fe5
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

impl<M: FontMetrics, S: ImageSizer> super::Layouter<M, S> {
    /// Layout de `Content::Align { alignment, body }`.
    pub(super) fn layout_align(&mut self, alignment: Align2D, body: &Content) {
        // Garantir que não há texto inline pendente antes de posicionar o bloco.
        // flush_line usa line_start_x (Passo 81.5).
        self.flush_line();

        let avail_w = self.available_width();

        // Layoutar o corpo num sub-frame — cell_x=0 para que items internos
        // comecem em x=0. O sub_frame activa is_height_unconstrained=true
        // e restaura ao terminar.
        let (sub_h, sub_items) = self.layout_sub_frame_with_width(body, 0.0, avail_w);

        // Origem vertical local do sub-frame (ascender). Necessária para
        // rebaser as coordenadas Y ao colocar no frame pai.
        let (ascender_local, _) = self.metrics.vertical_metrics(self.font_size_pt);
        let sub_origin_y        = ascender_local.0;

        // Largura do conteúdo — medida independente para centrar/alinhar.
        let (content_w, _) = measure_content(body, avail_w);

        // Verificar quebra de página com a altura do sub-frame.
        if self.cursor_y.0 + sub_h > self.page_bottom_limit() {
            self.new_page();
        }

        // Selecção de remaining_h e VAlign efectivo (Passo 83).
        //
        // Prioridade: cell_available_h > is_height_unconstrained > página.
        // Dentro de uma célula de Grid (`cell_available_h = Some(h)`), a
        // altura é conhecida — Bottom e Horizon ancoram à célula.
        // No fluxo livre (sub_frame sem cell), Bottom/Horizon decaem
        // para Top (sem "fundo" para ancorar).
        // No fluxo normal da página, usar o espaço restante até à margem.
        let (remaining_h, effective_v) = if let Some(cell_h) = self.cell_available_h {
            (cell_h, alignment.v)
        } else if self.is_height_unconstrained {
            (sub_h, None)
        } else {
            let space = f64::max(0.0, self.page_bottom_limit() - self.cursor_y.0);
            (space, alignment.v)
        };

        let effective_align = Align2D {
            h: alignment.h,
            v: effective_v,
        };

        // origin_x = line_start_x (não page_config.margin). Dentro de uma
        // célula de grid, line_start_x é cell_x, não a margem da página.
        let (target_x, target_y) = self.resolve_alignment(
            effective_align,
            content_w,
            sub_h,
            avail_w,
            remaining_h,
            self.line_start_x.0,
            self.cursor_y.0,
        );

        // Transferir items: sub_origin_x = 0 (passámos cell_x=0);
        // sub_origin_y = ascender_local (compensar a origem vertical).
        for item in sub_items {
            let (ix, iy) = item_pos(&item);
            let new_x = Pt(target_x + ix);
            let new_y = Pt(target_y + iy - sub_origin_y);
            self.current_items.push(translate_frame_item(item, new_x, new_y));
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
        match (self.cell_available_h.is_some(), effective_v) {
            (false, Some(VAlign::Horizon)) | (false, Some(VAlign::Bottom)) => {
                self.cursor_y = Pt(self.page_bottom_limit());
            }
            _ => {
                self.cursor_y = Pt(target_y + sub_h);
            }
        }
    }

    /// Layout de `Content::Place { alignment, dx, dy, scope, body }`.
    pub(super) fn layout_place(
        &mut self,
        alignment: Align2D,
        dx:        f64,
        dy:        f64,
        scope:     PlaceScope,
        body:      &Content,
    ) {
        // Place NÃO chama flush_line e NÃO modifica cursor_x nem cursor_y.
        let avail_w_page = self.available_width();
        let avail_h_page = self.available_height();

        let (sub_h, sub_items) = self.layout_sub_frame_with_width(body, 0.0, avail_w_page);

        let (ascender_local, _) = self.metrics.vertical_metrics(self.font_size_pt);
        let sub_origin_y        = ascender_local.0;

        let (content_w, _) = measure_content(body, avail_w_page);

        // Passo 84.6 (encerra DEBT-37): seleccionar área de ancoragem
        // segundo `scope`.
        // - PlaceScope::Column (default): se estamos dentro de uma
        //   célula de Grid (cell_origin_* + cell_available_h todos
        //   Some), ancorar à célula. Caso contrário cair para a página.
        // - PlaceScope::Parent: ancorar sempre à página, mesmo dentro
        //   de Grid (paridade com vanilla — `parent` "spans columns").
        let (origin_x, origin_y, avail_w, avail_h) = match scope {
            PlaceScope::Column => match (
                self.cell_origin_x,
                self.cell_origin_y,
                self.cell_origin_w,
                self.cell_available_h,
            ) {
                (Some(cx), Some(cy), Some(cw), Some(ch)) => (cx, cy, cw, ch),
                _ => (
                    self.line_start_x.0,
                    self.page_config.margin,
                    avail_w_page,
                    avail_h_page,
                ),
            },
            PlaceScope::Parent => (
                self.page_config.margin,
                self.page_config.margin,
                avail_w_page,
                avail_h_page,
            ),
        };

        let (base_x, base_y) = self.resolve_alignment(
            alignment,
            content_w,
            sub_h,
            avail_w,
            avail_h,
            origin_x,
            origin_y,
        );

        let target_x = base_x + dx;
        let target_y = base_y + dy;

        // Compensação Y para o caso de estarmos dentro de um sub_frame
        // de célula de Grid (Passo 84.6). O sub_frame da célula faz
        // `abs_y = row_start_y + (item.y - ascender_local)` ao transferir
        // items. Como `target_y` aqui já é coord absoluta-na-página
        // (origin_y é cell_origin_y para Column-em-célula ou page_margin
        // para Parent), subtrair `cell_origin_y` em vez de `sub_origin_y`
        // anula a translação que o Grid vai aplicar:
        //   final = row_start_y + (target_y + iy - cell_origin_y - ascender)
        //         = target_y + iy - sub_origin_y  (porque row_start_y == cell_origin_y).
        // Quando NÃO estamos em sub_frame de célula, Place push directo
        // no frame da página: padrão Passo 82 (subtrair sub_origin_y).
        let y_offset = self.cell_origin_y.unwrap_or(sub_origin_y);

        for item in sub_items {
            let (ix, iy) = item_pos(&item);
            let new_x = Pt(target_x + ix);
            let new_y = Pt(target_y + iy - y_offset);
            self.current_items.push(translate_frame_item(item, new_x, new_y));
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
