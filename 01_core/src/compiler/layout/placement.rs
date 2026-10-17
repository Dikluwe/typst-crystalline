//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/placement.md
//! @prompt-hash 8d3c2f6b
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
use super::{item_pos, translate_frame_item};
use crate::entities::value::Value;

/// Closed certificate for textual line alignment; geometric consumers keep
/// the established envelope path and its deferred coordinate protocol.
pub(super) fn text_alignment_body(content: &Content) -> bool {
    if matches!(content, Content::Empty) {
        return true;
    }
    if matches!(content, Content::Text(_)) {
        return true;
    }
    if matches!(content, Content::Space) {
        return true;
    }
    if matches!(content, Content::SpaceFromNewline) {
        return true;
    }
    if matches!(content, Content::Linebreak(_)) {
        return true;
    }
    if let Content::Sequence(parts) = content {
        return parts.iter().all(text_alignment_body);
    }
    if let Content::Styled(e) = content {
        return text_alignment_body(&e.body);
    }
    if let Content::Strong(e) = content {
        return text_alignment_body(&e.body);
    }
    if let Content::Emph(e) = content {
        return text_alignment_body(&e.body);
    }
    if let Content::Par { body } = content {
        return text_alignment_body(body);
    }
    if let Content::Align(e) = content {
        return text_alignment_body(&e.body);
    }
    if let Content::Block(e) = content {
        return !e.clip
            && e.fill.is_none()
            && e.stroke.is_none()
            && e.inset
                == crate::entities::sides::Sides::uniform(
                    crate::entities::layout_types::Length::ZERO,
                )
            && e.outset
                == crate::entities::sides::Sides::uniform(
                    crate::entities::layout_types::Length::ZERO,
                )
            && text_alignment_body(&e.body);
    }
    // Absence of positive proof keeps this carrier on its existing layout path.
    false
}

/// A complete grid frame owns its horizontal placement inside the flow region.
fn grid_alignment_body(content: &Content) -> bool {
    if matches!(content, Content::Grid(_) | Content::Table(_)) {
        return true;
    }
    if let Content::Sequence(parts) = content {
        return parts.len() == 1 && grid_alignment_body(&parts[0]);
    }
    if let Content::Styled(e) = content {
        return grid_alignment_body(&e.body);
    }
    if let Content::Located(e) = content {
        return grid_alignment_body(&e.body);
    }
    if let Content::Realized(e) = content {
        return grid_alignment_body(&e.visual);
    }
    false
}

impl<'a, M: FontMetrics, S: ImageSizer> super::Layouter<'a, M, S> {
    /// Layout de `Content::Align { alignment, body }`.
    pub(super) fn layout_align(&mut self, alignment: Align2D, body: &Content) {
        // Garantir que não há texto inline pendente antes de posicionar o bloco.
        // flush_line usa line_start_x (Passo 81.5).
        let had_pending_line =
            super::html_frame::items_have_causal_line(&self.regions.current.current_line);
        let incoming_page = self.pages.len();
        self.flush_line();

        let avail_w = self.available_width();

        // P772g (encerra achado B de P772f) — origin_x real (não 0.0):
        // `layout_align` é um "consumidor absoluto" de `layout_sub_frame`
        // (ver 00_nucleo/prompts/compiler/layout.md §"Contrato de composição
        // de coordenadas"). Um `Content::Place` aninhado no corpo emite
        // coordenadas absolutas usando `regions.cell`/`cell_origin_x`; se
        // este sub-frame usasse origin_x=0.0 e depois somasse o `target_x`
        // inteiro (como acontecia antes), a origem da célula seria somada
        // duas vezes. Ao inicializar o sub-frame já na origem real, o
        // conteúdo normal (texto/formas) fica com posições absolutas desde
        // o início; só falta somar o deslocamento *incremental* do
        // alinhamento (`delta_x` abaixo), não o `target_x` inteiro.
        let origin_x_abs = self.regions.current.line_start_x.0;
        let textual = avail_w.is_finite()
            && matches!(alignment.v, None | Some(VAlign::Top))
            && text_alignment_body(body);
        let grid_aligned = avail_w.is_finite()
            && matches!(alignment.v, None | Some(VAlign::Top))
            && grid_alignment_body(body);
        let flow_aligned = textual || grid_aligned;
        let saved_alignment = self.explicit_line_alignment;
        if flow_aligned {
            let rtl = self.chain.custom("text.dir")
                == Some(&crate::entities::value::Value::Dir(
                    crate::entities::dir::Dir::RTL,
                ));
            self.explicit_line_alignment = alignment
                .h
                .map(|h| match h {
                    HAlign::Start => {
                        if rtl {
                            HAlign::Right
                        } else {
                            HAlign::Left
                        }
                    }
                    HAlign::End => {
                        if rtl {
                            HAlign::Left
                        } else {
                            HAlign::Right
                        }
                    }
                    h => h,
                })
                .or(saved_alignment);
        }
        let region = super::sub_frame::SubLayoutRegion {
            origin_x: origin_x_abs,
            width: avail_w,
            height: None,
            align_rtl: false,
            unconstrained_height: true,
        };
        let (sub_h, sub_items, sub_deco, orphaned_align_x, orphaned_align_y) =
            self.layout_sub_frame_at_geometric_origin(body, region);
        let flow_width = self.last_sub_frame_flow_width;
        self.explicit_line_alignment = saved_alignment;
        let sub_origin_y = 0.0;
        let occupies_flow = sub_h > 0.0;
        if occupies_flow && self.pages.len() == incoming_page {
            if self.block_chain_active && !self.initial_baseline_pending {
                self.regions.current.cursor_y = Pt(self.prev_line_baseline
                    + self.prev_block_equation_descent
                    + self.prev_block_below_pending);
            } else if had_pending_line
                || (!self.initial_baseline_pending && self.flow_placement_y().is_some())
            {
                let spacing = match self.chain.custom("par.spacing") {
                    Some(Value::Length(length)) => length.resolve_pt(self.style.size.0),
                    _ => self.style.size.0 * super::vanilla_defaults::PAR_SPACING,
                };
                let end = self.flow_placement_y().unwrap_or(self.prev_line_baseline);
                self.regions.current.cursor_y = Pt(end + spacing);
            } else if !self.is_sub_frame && !self.initial_baseline_pending {
                let (top, _) = self.metrics.text_edges(self.style.size, &self.style);
                self.regions.current.cursor_y = self.regions.current.cursor_y - top;
            }
        }

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
        // âmbito de P772j; ver 00_nucleo/prompts/compiler/layout.md).
        let sub_item_refs: Vec<&crate::entities::layout_types::FrameItem> =
            sub_items.iter().collect();
        let content_right_abs = self.metrics.line_content_right(&sub_item_refs);
        let content_w = if flow_aligned {
            avail_w
        } else {
            (content_right_abs - origin_x_abs).max(0.0)
        };

        // Verificar quebra de página com a altura do sub-frame.
        if self.regions.current.cursor_y.0 + sub_h > self.page_bottom_limit() {
            self.new_page();
            if !self.initial_baseline_pending {
                let (top, _) = self.metrics.text_edges(self.style.size, &self.style);
                self.regions.current.cursor_y = self.regions.current.cursor_y - top;
            }
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

        // **P897** — sob `width: auto`, `avail_w` (`available_width()`) pode
        // ser infinito; `resolve_alignment` com `available_w` infinito produz
        // `target_x` infinito para Center/Right. Fallback: substituir
        // `avail_w` por `content_w` na chamada — a fórmula degenera para
        // `target_x == origin_x` (mesmo efeito de um alinhamento Left, igual
        // ao fallback já usado em P896 para `equation.rs`). Registar a
        // correcção diferida (`pending_align_centering`) para reaplicar
        // quando a largura final da página for conhecida
        // (`apply_pending_align_fixups`, chamado de `finish()`/`new_page()`).
        let defer_block_x = self.block_auto_width && !flow_aligned;
        let avail_w_is_finite = avail_w.is_finite() && !defer_block_x;
        let avail_w_for_resolve = if avail_w_is_finite { avail_w } else { content_w };

        // **P898** — mesma vulnerabilidade no eixo Y: sob `height: auto` e
        // fora de uma célula/sub-frame de altura conhecida, `remaining_h`
        // vem de `page_bottom_limit() - cursor_y` (`f64::INFINITY` quando
        // `page_bottom_limit()` é infinito). Mesmo fallback: `content_h`
        // (`sub_h`, a altura própria do bloco) no lugar do valor infinito —
        // a fórmula degenera para "encostado a `origin_y`". Corrigido depois
        // por `apply_pending_align_v_fixups`.
        let remaining_h_is_finite = remaining_h.is_finite();
        let remaining_h_for_resolve =
            if remaining_h_is_finite { remaining_h } else { sub_h };

        // origin_x = line_start_x (não page_config.margin). Dentro de uma
        // célula de grid, line_start_x é cell_x, não a margem da página.
        let (target_x, target_y) = self.resolve_alignment(
            effective_align,
            content_w,
            sub_h,
            avail_w_for_resolve,
            remaining_h_for_resolve,
            self.regions.current.line_start_x.0,
            self.regions.current.cursor_y.0,
        );

        if !avail_w_is_finite {
            let start_idx = self.regions.current.current_items.len();
            let count = sub_items.len();
            self.pending_align_centering.push((
                vec![start_idx],
                count,
                effective_align,
                content_w,
                self.regions.current.line_start_x.0,
                target_x,
            ));
        }

        if !remaining_h_is_finite && !occupies_flow {
            let start_idx = self.regions.current.current_items.len();
            let count = sub_items.len();
            self.pending_align_v_centering.push((
                vec![start_idx],
                count,
                effective_align,
                sub_h,
                self.regions.current.cursor_y.0,
                0.0, // layout_align não tem dy (só Content::Place tem)
                target_y,
            ));
        }

        // Transferir items: P772g — sub-frame já iniciado em origin_x_abs
        // (não 0), logo os itens "normais" já vêm absolutos; soma-se só o
        // deslocamento incremental do alinhamento (`delta_x`), nunca o
        // `target_x` inteiro (evita duplicar a origem em itens de
        // `Content::Place` aninhado, que já emitem coordenadas absolutas).
        // sub_origin_y = ascender_local (compensar a origem vertical, Y
        // inalterado por P772g — ver nota no L0).
        let delta_x = target_x - origin_x_abs;
        // **P908** — rebasear as entradas órfãs devolvidas por
        // `layout_sub_frame` (um `Content::Align`/`Content::Place`
        // aninhado dentro deste corpo, sob `width`/`height: auto`) com a
        // MESMA translação aplicada aos `FrameItem`s reais abaixo:
        // `path[0]` (índice na lista local devolvida) passa a apontar para
        // a posição real em `current_items` (soma o comprimento actual,
        // capturado ANTES do merge). **Distinção física vs lógica (eixo
        // Y)**: `origin_y`/`applied_y` são quantidades LÓGICAS (o "topo"
        // da caixa de alinhamento, sem ascender embutido — mesma
        // convenção de `layout_place`, onde `origin_y = page_config.
        // margin` directamente, nunca `margin + ascender`), ao contrário
        // dos `FrameItem`s reais (cujo `pos.y` de texto É a baseline,
        // ascender já embutido). Por isso a translação lógica usa
        // `target_y` directamente — NÃO `target_y - sub_origin_y` (essa
        // subtracção só faz sentido para compensar o ascender já presente
        // em `iy`, o `pos.y` de um item real). Usar o delta dos items para
        // as quantidades lógicas sub-desloca por um ascender inteiro —
        // confirmado por teste (`p908_place_aninhado_em_align_sob_height_
        // auto_...`, erro de ~8.8pt = ascender de `DEFAULT_FONT_SIZE`).
        // Ver `00_nucleo/prompts/compiler/layout.md` §P908.
        let insertion_base = self.regions.current.current_items.len();
        for (mut path, count, align, content_w, origin_x, applied_x) in orphaned_align_x {
            path[0] += insertion_base;
            self.pending_align_centering.push((
                path,
                count,
                align,
                content_w,
                origin_x + delta_x,
                applied_x + delta_x,
            ));
        }
        for (mut path, count, align, content_h, origin_y, dy, applied_y) in
            orphaned_align_y
        {
            path[0] += insertion_base;
            self.pending_align_v_centering.push((
                path,
                count,
                align,
                content_h,
                origin_y + target_y,
                dy,
                applied_y + target_y,
            ));
        }
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
        //
        // **P898** — sob `height: auto`, `page_bottom_limit()` pode ser
        // infinito; gravar `cursor_y = Pt(f64::INFINITY)` corromperia não só
        // este item mas todo o estado subsequente do Layouter (avanço de
        // conteúdo posterior, e `compute_page_height()` em `finish()`/
        // `new_page()`, que soma `cursor_y` directamente — achado registado
        // em `typst-passo-898.md`). Guarda adicional: só usar
        // `page_bottom_limit()` quando finito; caso contrário cair no mesmo
        // ramo `_` (avançar para `target_y + sub_h`, já finito porque
        // `target_y` foi resolvido com o fallback `remaining_h_for_resolve`
        // acima).
        match (self.regions.cell.is_some(), effective_v) {
            (false, Some(VAlign::Horizon)) | (false, Some(VAlign::Bottom))
                if self.page_bottom_limit().is_finite() =>
            {
                self.regions.current.cursor_y = Pt(self.page_bottom_limit());
            }
            _ => {
                self.regions.current.cursor_y = Pt(target_y + sub_h);
            }
        }
        self.record_flow_placement_end(self.regions.current.cursor_y.0);
        self.record_flow_width_at(origin_x_abs, flow_width);
        if occupies_flow {
            let frame_end = self.regions.current.cursor_y.0;
            self.last_block_descent_y = Some(frame_end);
            self.prev_line_baseline = frame_end;
            self.prev_block_equation_descent = 0.0;
            self.prev_block_below_pending = match self.chain.custom("par.spacing") {
                Some(Value::Length(length)) => length.resolve_pt(self.style.size.0),
                _ => self.style.size.0 * super::vanilla_defaults::PAR_SPACING,
            };
            self.block_chain_active = true;
            self.prev_margin_is_parbreak = false;
            self.initial_baseline_pending = false;
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
        flow_y: Option<f64>,
    ) {
        // Place NÃO chama flush_line e NÃO modifica cursor_x nem cursor_y.
        let block_region = self.block_place_region;
        let avail_w_page = block_region.map_or_else(|| self.available_width(), |r| r.w.0);
        // **P908** — `available_height()` lê `regions.current.height`, que
        // `layout_sub_frame` substitui por um sentinel interno
        // (`1_000_000_000.0`, finito) sempre que a região é chamada com
        // `height: None` (Align/Place/Transform/grid-cell, todos
        // `unconstrained_height: true`). Quando `self.is_height_
        // unconstrained` é verdadeiro (estamos dentro do corpo de um
        // desses sub-frames — inclui aninhamento a qualquer profundidade,
        // `is_height_unconstrained` é guardado/restaurado por
        // `layout_sub_frame`), o sentinel NÃO representa altura disponível
        // real nenhuma — sem esta guarda, `avail_h_page` ficava finito
        // (o sentinel passa `is_finite()`) e o desvio de P897/898 ("se
        // infinito, diferir via `pending_align_v_centering`") nunca
        // disparava: `Place` com alinhamento `Bottom`/`Horizon` aninhado
        // dentro de QUALQUER sub-frame não-restringido resolvia de
        // imediato contra ~1 mil milhões de pt, mesmo em página de altura
        // fixa (bug anterior a P908, confirmado por
        // `p908_place_aninhado_em_align_sob_height_auto_...`, que usa esta
        // corrida como controlo). `layout_align` já tinha esta guarda
        // (`else if self.is_height_unconstrained`, antes de chamar
        // `page_bottom_limit()`); `layout_place` não tinha.
        let avail_h_page = if let Some(region) = block_region {
            region.h.0
        } else if self.is_height_unconstrained {
            f64::INFINITY
        } else {
            self.available_height()
        };

        let region = super::sub_frame::SubLayoutRegion {
            origin_x: 0.0,
            width: avail_w_page,
            height: None,
            align_rtl: false,
            unconstrained_height: true,
        };
        let (sub_h, sub_items, sub_deco, orphaned_align_x, orphaned_align_y) =
            self.layout_sub_frame_at_geometric_origin(body, region);

        // **P904 (Item 3)** — `measure_content` (`helpers.rs`) só tem
        // braços para `Content::Shape`/`Content::Sequence`; para
        // `Content::Text` (o caso comum de `place(right)[texto]`) devolvia
        // sempre `(0.0, 0.0)`, fazendo `resolve_alignment` posicionar como
        // se o conteúdo tivesse largura zero (P772j catalogou o mesmo bug
        // para `Content::Align` e corrigiu-o assim — não estendendo
        // `measure_content`, mas medindo a partir dos `sub_items` já
        // layoutados via `FontMetrics::line_content_right`, o mesmo
        // mecanismo já usado por `measure_content_real`; `Content::Place`
        // ficou fora de âmbito nesse passo, explicitamente — ver
        // `paridade-producao-p772j.md`). Mesmo mecanismo reaproveitado
        // aqui: `origin_x` do sub-frame é sempre `0.0` para `Place`, logo
        // `content_w = line_content_right(sub_items) - 0.0` directamente.
        let sub_item_refs: Vec<&crate::entities::layout_types::FrameItem> =
            sub_items.iter().collect();
        let content_w = self.metrics.line_content_right(&sub_item_refs).max(0.0);

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

        let local_origin = (
            self.regions.current.line_start_x.0,
            self.sub_frame_origin_y,
            avail_w_page,
            avail_h_page,
        );
        let (origin_x, origin_y, avail_w, avail_h) = if let Some(region) = block_region {
            (region.x.0, region.y.0, region.w.0, region.h.0)
        } else {
            match scope {
                PlaceScope::Column => {
                    match (
                        self.cell_origin_x,
                        self.cell_origin_y,
                        self.regions.cell.as_ref(),
                    ) {
                        (Some(cx), Some(cy), Some(cell)) => (
                            cx,
                            if in_sub_frame { self.sub_frame_origin_y } else { cy },
                            cell.width,
                            cell.height,
                        ),
                        _ if in_sub_frame => local_origin,
                        _ => (
                            self.regions.current.line_start_x.0,
                            self.page_config.margin.top,
                            avail_w_page,
                            avail_h_page,
                        ),
                    }
                }
                PlaceScope::Parent if in_sub_frame => local_origin,
                PlaceScope::Parent => (
                    self.page_config.margin.left,
                    self.page_config.margin.top,
                    avail_w_page,
                    avail_h_page,
                ),
            }
        };
        let origin_y =
            if alignment.v.is_none() { flow_y.unwrap_or(origin_y) } else { origin_y };

        // **P897** — mesma vulnerabilidade de `layout_align`: `avail_w` (aqui
        // vindo de `avail_w_page` nos ramos `Parent`/`Column`-sem-célula
        // activa) pode ser infinito sob `width: auto`. Ramos com célula
        // activa (`PlaceScope::Column` com `regions.cell`) usam
        // `cell.width`, sempre finito — não vulneráveis, não alterados aqui.
        let avail_w_is_finite = avail_w.is_finite();
        let avail_w_for_resolve = if avail_w_is_finite { avail_w } else { content_w };

        // **P898** — mesma vulnerabilidade no eixo Y: `avail_h` (`avail_h_page`
        // nos mesmos ramos sem célula activa) pode ser infinito sob `height:
        // auto`. Mesmo fallback: `sub_h` (altura própria do body) no lugar do
        // valor infinito.
        let avail_h_is_finite = avail_h.is_finite();
        let avail_h_for_resolve = if avail_h_is_finite { avail_h } else { sub_h };

        let (base_x, base_y) = self.resolve_alignment(
            alignment,
            content_w,
            sub_h,
            avail_w_for_resolve,
            avail_h_for_resolve,
            origin_x,
            origin_y,
        );

        let target_x = base_x + dx;
        let target_y = base_y + dy;

        if !avail_w_is_finite {
            // `resolve_alignment` é linear em `origin_x` (soma-o directamente
            // ao resultado, sem interagir com `avail_w`/`content_w`); gravar
            // `origin_x + dx` em vez de `origin_x` faz o `apply_pending_align_fixups`
            // recompor directamente `target_x` (que já inclui `dx`), sem
            // precisar de somar `dx` outra vez fora de `resolve_alignment`.
            let start_idx = self.regions.current.current_items.len();
            let count = sub_items.len();
            self.pending_align_centering.push((
                vec![start_idx],
                count,
                alignment,
                content_w,
                origin_x + dx,
                target_x,
            ));
        }

        let deferred_y_index = self.pending_align_v_centering.len();
        if !avail_h_is_finite {
            let start_idx = self.regions.current.current_items.len();
            let count = sub_items.len();
            // **P898 (correcção pós-Agente B)** — ao contrário do truque de
            // `origin_x + dx` usado no eixo X (seguro porque `final_avail_w`
            // é uma constante da página, independente de `origin_x`), aqui
            // `final_avail_h` DEPENDE de `origin_y`
            // (`page_height - margin - origin_y`, ver `apply_pending_align_v_fixups`).
            // Gravar `origin_y + dy` nesse campo faria `dy` contaminar o
            // cálculo de `final_avail_h` — confirmado por teste exploratório
            // que devolvia a posição sem `dy` nenhum aplicado. Por isso
            // `origin_y` puro e `dy` são gravados em campos separados.
            self.pending_align_v_centering.push((
                vec![start_idx],
                count,
                alignment,
                sub_h,
                origin_y,
                dy,
                target_y,
            ));
        }

        // **P908** — rebasear as entradas órfãs devolvidas por
        // `layout_sub_frame` (`Content::Align`/`Content::Place` aninhado
        // dentro deste `Place`, sob `width`/`height: auto`) com a MESMA
        // translação aplicada aos `FrameItem`s reais abaixo: `origin_x`
        // do sub-frame de `Place` é sempre `0.0`, logo `delta_x =
        // target_x` directamente (paralelo `content_w` em P904 Item 3).
        // `path[0]` passa a apontar para a posição real em `current_items`
        // (soma o comprimento actual, capturado ANTES do merge).
        // Body items and orphaned fixups share geometric zero. Translate both
        // by the same target_y; no font ascender is embedded in that origin.
        let insertion_base = self.regions.current.current_items.len();
        let delta_x = target_x;
        for (mut path, count, align, content_w, origin_x, applied_x) in orphaned_align_x {
            path[0] += insertion_base;
            self.pending_align_centering.push((
                path,
                count,
                align,
                content_w,
                origin_x + delta_x,
                applied_x + delta_x,
            ));
        }
        for (mut path, count, align, content_h, origin_y, dy, applied_y) in
            orphaned_align_y
        {
            path[0] += insertion_base;
            self.pending_align_v_centering.push((
                path,
                count,
                align,
                content_h,
                origin_y + target_y,
                dy,
                applied_y + target_y,
            ));
        }

        for item in sub_items {
            let (ix, iy) = item_pos(&item);
            let new_x = Pt(target_x + ix);
            let new_y = Pt(target_y + iy);
            self.regions
                .current
                .current_items
                .push(translate_frame_item(item, new_x, new_y));
        }
        // **P772x** — mesma translação para segmentos de decoração do body
        // (repro original de P772w: `#underline[.. #place(..)[explanation]]`).
        if let Some(coll) = self.decoration_lines_collector.as_mut() {
            let start = coll.len();
            for seg in &sub_deco {
                coll.push(super::DecoSegment {
                    start_x: Pt(target_x + seg.start_x.val()),
                    end_x: Pt(target_x + seg.end_x.val()),
                    baseline_y: Pt(target_y + seg.baseline_y.val()),
                });
            }
            if block_region.is_some() && !avail_h_is_finite && start < coll.len() {
                self.block_place_decorations
                    .push((deferred_y_index, start, coll.len()));
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

use crate::entities::layout_types::{FrameItem, HAlign};

use super::helpers;

impl<'a, M: FontMetrics, S: ImageSizer> super::Layouter<'a, M, S> {
    /// **P897** — mesmo mecanismo de `apply_pending_equation_fixups`,
    /// generalizado a `Content::Align`/`Content::Place` (`placement.rs`)
    /// quando `resolve_alignment` foi chamado com `available_w` infinito
    /// (`width: auto`, ainda por resolver no momento do posicionamento).
    /// Em vez de reimplementar a fórmula de centragem/alinhamento à direita,
    /// reaproveita `resolve_alignment` directamente com a largura disponível
    /// final (`page_width - 2*margin`) já finita — só o componente `x` do
    /// resultado é usado; `content_h`/`available_h`/`origin_y` são
    /// dummies (`0.0`) porque este método só corrige o eixo horizontal — o
    /// vertical usa `apply_pending_align_v_fixups` (**P898**), abaixo.
    pub(super) fn apply_pending_align_fixups(
        &mut self,
        items: &mut [FrameItem],
        page_width: f64,
    ) {
        let final_avail_w = page_width - self.page_config.margin.horizontal();
        for (path, count, align, content_w, origin_x, applied_x) in
            std::mem::take(&mut self.pending_align_centering)
        {
            let (correct_x, _) = self.resolve_alignment(
                align,
                content_w,
                0.0,
                final_avail_w,
                0.0,
                origin_x,
                0.0,
            );
            let delta = correct_x - applied_x;
            if delta == 0.0 {
                continue;
            }
            if let Some(slice) = helpers::resolve_path_slice(items, &path, count) {
                for item in slice {
                    helpers::shift_frame_item_x(item, delta);
                }
            }
        }
    }

    /// **P898** — simétrico de `apply_pending_align_fixups`, eixo vertical.
    /// Corrige `Content::Align`/`Content::Place` (`placement.rs`) adiados
    /// quando `remaining_h`/`avail_h` (derivados de `available_height()`/
    /// `page_bottom_limit()`) estavam infinitos no momento do
    /// posicionamento (`height: auto`, ainda por resolver). Reaproveita
    /// `resolve_alignment` com a altura restante final calculada a partir
    /// de `page_height` (já finito neste ponto) e do `origin_y` gravado em
    /// cada entrada — ao contrário do eixo X (onde `available_width()` é
    /// invariante em toda a página), a altura restante depende da posição
    /// vertical onde o item foi originalmente posicionado
    /// (`page_height - margin - origin_y`), por isso o cálculo é repetido
    /// por entrada em vez de uma única constante `final_avail_h` partilhada.
    /// Só o componente `y` do resultado é usado; `content_w`/`available_w`/
    /// `origin_x` são dummies (`0.0`).
    pub(super) fn apply_pending_align_v_fixups(
        &mut self,
        items: &mut [FrameItem],
        page_height: f64,
    ) {
        for (path, count, align, content_h, origin_y, dy, applied_y) in
            std::mem::take(&mut self.pending_align_v_centering)
        {
            let final_avail_h =
                f64::max(0.0, page_height - self.page_config.margin.bottom - origin_y);
            let (_, correct_base_y) = self.resolve_alignment(
                align,
                0.0,
                content_h,
                0.0,
                final_avail_h,
                0.0,
                origin_y,
            );
            let correct_y = correct_base_y + dy;
            let delta = correct_y - applied_y;
            if delta == 0.0 {
                continue;
            }
            if let Some(slice) = helpers::resolve_path_slice(items, &path, count) {
                for item in slice {
                    helpers::shift_frame_item_y(item, delta);
                }
            }
        }
    }

    /// Calcula a coordenada `(x, y)` do canto superior esquerdo de um item
    /// dado o alinhamento, as dimensões do conteúdo, e a área disponível.
    /// Passo 82.
    ///
    /// `origin_x` e `origin_y` definem o canto superior esquerdo da área
    /// de referência (`line_start_x` para Align; `line_start_x`/`margin` para Place).
    #[allow(clippy::too_many_arguments)]
    pub(super) fn resolve_alignment(
        &self,
        align: Align2D,
        content_w: f64,
        content_h: f64,
        available_w: f64,
        available_h: f64,
        origin_x: f64,
        origin_y: f64,
    ) -> (f64, f64) {
        let x = match align.h.unwrap_or(HAlign::Left) {
            HAlign::Left | HAlign::Start => origin_x,
            // rationale: P1064 Classe 1A — alinhamento central horizontal ((available_w - content_w) / 2.0)
            HAlign::Center => origin_x + (available_w - content_w) / 2.0,
            HAlign::Right | HAlign::End => origin_x + (available_w - content_w),
        };

        let y = match align.v.unwrap_or(VAlign::Top) {
            VAlign::Top => origin_y,
            // rationale: P1064 Classe 1A — alinhamento central vertical ((available_h - content_h) / 2.0)
            VAlign::Horizon => origin_y + (available_h - content_h) / 2.0,
            VAlign::Bottom => origin_y + (available_h - content_h),
        };

        (x, y)
    }
}

#[cfg(test)]
mod block_align_regressions {
    use crate::compiler::layout::{FixedMetrics, Layouter};
    use crate::entities::{
        content::Content,
        dir::Dir,
        image_sizer::NullImageSizer,
        introspector::TagIntrospector,
        layout_types::{Align2D, FrameItem, HAlign, TextStyle},
        style::Styles,
        value::Value,
    };
    use comemo::Track;

    #[test]
    fn alignment_preserves_spacing_after_the_previous_paragraph() {
        let intr = TagIntrospector::empty();
        let make = || {
            Layouter::new(
                FixedMetrics,
                NullImageSizer,
                11.0,
                (&intr as &dyn crate::entities::introspector::Introspector).track(),
            )
        };
        let mut normal = make();
        normal.layout_content(&Content::text("A"));
        normal.layout_content(&Content::Parbreak);
        normal.layout_content(&Content::text("B"));
        let expected = normal.finish();
        let mut aligned = make();
        aligned.layout_content(&Content::text("A"));
        aligned.layout_align(
            Align2D { h: Some(HAlign::Left), v: None },
            &Content::text("B"),
        );
        let actual = aligned.finish();
        let y = |d: &crate::entities::layout_types::PagedDocument| {
            d.pages[0]
                .items
                .iter()
                .find_map(|it| {
                    if let FrameItem::Text { pos, text, .. } = it {
                        (text.as_str() == "B").then_some(pos.y.0)
                    } else {
                        None
                    }
                })
                .unwrap()
        };
        assert!(
            (y(&actual) - y(&expected)).abs() < 1e-6,
            "align={} normal={}",
            y(&actual),
            y(&expected)
        );
    }

    #[test]
    fn checkpoint_restores_alignment_before_emitting_a_line() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn crate::entities::introspector::Introspector).track(),
        );
        let right = l.regions.current.width - l.page_config.margin.right;
        l.explicit_line_alignment = Some(HAlign::Right);
        let checkpoint = l.capture_flow_suffix_checkpoint();
        l.explicit_line_alignment = Some(HAlign::Left);
        l.restore_flow_suffix_checkpoint(checkpoint);
        l.layout_content(&Content::text("A"));
        let doc = l.finish();
        let x = doc.pages[0]
            .items
            .iter()
            .find_map(|it| {
                if let FrameItem::Text { pos, .. } = it {
                    Some(pos.x.0)
                } else {
                    None
                }
            })
            .unwrap();
        assert!((x - (right - 6.6)).abs() < 1e-6, "restored line x={x}");
    }

    #[test]
    fn alignment_scope_does_not_override_the_following_rtl_paragraph() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn crate::entities::introspector::Introspector).track(),
        );
        l.chain = l.chain.push_custom("text.dir", Value::Dir(Dir::RTL));
        l.style = TextStyle::from(&l.chain);
        let right = l.regions.current.width - l.page_config.margin.right;
        l.layout_align(Align2D { h: Some(HAlign::Left), v: None }, &Content::text("A"));
        l.layout_content(&Content::Parbreak);
        l.layout_content(&Content::text("BBB"));
        let doc = l.finish();
        let x = doc.pages[0]
            .items
            .iter()
            .find_map(|it| {
                if let FrameItem::Text { pos, text, .. } = it {
                    (text.as_str() == "BBB").then_some(pos.x.0)
                } else {
                    None
                }
            })
            .unwrap();
        assert!((x - (right - 19.8)).abs() < 1e-6, "outer paragraph x={x}");
    }

    #[test]
    fn alignment_preserves_spacing_to_the_following_paragraph() {
        let intr = TagIntrospector::empty();
        let mut plain = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn crate::entities::introspector::Introspector).track(),
        );
        plain.layout_content(&Content::text("A"));
        plain.layout_content(&Content::Parbreak);
        plain.layout_content(&Content::text("B"));
        let expected = plain.finish();
        let mut aligned = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn crate::entities::introspector::Introspector).track(),
        );
        aligned.layout_align(
            Align2D { h: Some(HAlign::Left), v: None },
            &Content::text("A"),
        );
        aligned.layout_content(&Content::Parbreak);
        aligned.layout_content(&Content::text("B"));
        let actual = aligned.finish();
        let y = |d: &crate::entities::layout_types::PagedDocument| {
            d.pages[0]
                .items
                .iter()
                .find_map(|it| {
                    if let FrameItem::Text { pos, text, .. } = it {
                        (text.as_str() == "B").then_some(pos.y.0)
                    } else {
                        None
                    }
                })
                .unwrap()
        };
        assert!(
            (y(&actual) - y(&expected)).abs() < 1e-6,
            "align={} plain={}",
            y(&actual),
            y(&expected)
        );
    }

    #[test]
    fn auto_block_positions_the_frame_without_changing_its_ltr_body() {
        use crate::entities::{
            corners::Corners, elements::block::BlockElem, layout_types::Length, rel::Rel,
            sides::Sides,
        };
        for width in [None, Some(Rel { rel: 0.0, abs: Length::pt(100.0) })] {
            let intr = TagIntrospector::empty();
            let mut l = Layouter::new(
                FixedMetrics,
                NullImageSizer,
                11.0,
                (&intr as &dyn crate::entities::introspector::Introspector).track(),
            );
            l.chain = l.chain.push_custom("text.dir", Value::Dir(Dir::RTL));
            l.style = TextStyle::from(&l.chain);
            let right = l.regions.current.width - l.page_config.margin.right;
            let b = BlockElem {
                body: Content::styled(
                    Content::text("ABC"),
                    Styles::new().push_custom("text.dir", Value::Dir(Dir::LTR)),
                ),
                width,
                height: None,
                inset: Sides::uniform(Length::ZERO),
                breakable: true,
                outset: Sides::uniform(Length::ZERO),
                radius: Corners::uniform(Length::ZERO),
                clip: false,
                fill: None,
                stroke: None,
                spacing: None,
                above: None,
                below: None,
                sticky: false,
            };
            super::super::block::layout(&mut l, &b);
            let doc = l.finish();
            let (pos, style) =
                super::super::tests::frame_items_recursive(&doc.pages[0].items)
                    .into_iter()
                    .find_map(|it| {
                        if let FrameItem::Text { pos, style, .. } = it {
                            Some((pos, style))
                        } else {
                            None
                        }
                    })
                    .unwrap();
            assert_eq!(style.dir, Some(Dir::LTR));
            let expected = right - if width.is_some() { 100.0 } else { 19.8 };
            assert!(
                (pos.x.0 - expected).abs() < 1e-6,
                "width={width:?} x={} expected={expected}",
                pos.x.0
            );
        }
    }

    #[test]
    fn explicit_alignment_controls_each_line_without_changing_run_direction() {
        for h in [HAlign::Left, HAlign::Center, HAlign::Right] {
            let intr = TagIntrospector::empty();
            let mut l = Layouter::new(
                FixedMetrics,
                NullImageSizer,
                11.0,
                (&intr as &dyn crate::entities::introspector::Introspector).track(),
            );
            l.chain = l.chain.push_custom("text.dir", Value::Dir(Dir::RTL));
            l.style = TextStyle::from(&l.chain);
            let start = l.regions.current.line_start_x.0;
            let width = l.available_width();
            let body = Content::sequence(vec![
                Content::text("AAAAA"),
                Content::Linebreak(std::sync::Arc::new(
                    crate::entities::elements::linebreak::LinebreakElem {
                        justify: false,
                        justify_explicit: false,
                    },
                )),
                Content::styled(
                    Content::text("BBB"),
                    Styles::new().push_custom("text.dir", Value::Dir(Dir::LTR)),
                ),
            ]);
            l.layout_align(Align2D { h: Some(h), v: None }, &body);
            let doc = l.finish();
            let texts: Vec<_> = doc
                .pages
                .iter()
                .flat_map(|p| &p.items)
                .filter_map(|it| {
                    if let FrameItem::Text { pos, text, style, .. } = it {
                        Some((pos, text, style))
                    } else {
                        None
                    }
                })
                .collect();
            assert_eq!(texts.len(), 2);
            for (pos, text, style) in texts {
                let w = text.len() as f64 * 6.6;
                let x = match h {
                    HAlign::Left => start,
                    HAlign::Center => start + (width - w) / 2.0,
                    HAlign::Right => start + width - w,
                    _ => unreachable!(),
                };
                assert!(
                    (pos.x.0 - x).abs() < 1e-6,
                    "{h:?} {text}: actual={} expected={x}",
                    pos.x.0
                );
                if text.as_str() == "BBB" {
                    assert_eq!(style.dir, Some(Dir::LTR));
                }
            }
        }
    }

    #[test]
    fn align_text_preserves_the_first_text_baseline() {
        let intr = TagIntrospector::empty();
        let mut plain = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn crate::entities::introspector::Introspector).track(),
        );
        plain.layout_content(&Content::text("A"));
        let expected = plain.finish();
        let mut aligned = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn crate::entities::introspector::Introspector).track(),
        );
        aligned.layout_align(
            Align2D { h: Some(HAlign::Left), v: None },
            &Content::text("A"),
        );
        let actual = aligned.finish();
        let y = |d: &crate::entities::layout_types::PagedDocument| {
            d.pages[0]
                .items
                .iter()
                .find_map(|it| {
                    if let FrameItem::Text { pos, .. } = it {
                        Some(pos.y.0)
                    } else {
                        None
                    }
                })
                .unwrap()
        };
        assert!(
            (y(&actual) - y(&expected)).abs() < 1e-6,
            "align={} plain={}",
            y(&actual),
            y(&expected)
        );
    }
}

#[cfg(test)]
mod grid_align_regressions {
    use super::*;
    use crate::compiler::layout::{FixedMetrics, Layouter};
    use crate::entities::{
        dir::Dir,
        elements::grid::GridElem,
        geometry::{ShapeKind, Stroke},
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{FrameItem, HAlign, Length, TextStyle, TrackSizing},
        sides::Sides,
    };
    use comemo::Track;

    fn grid(cols: usize) -> Content {
        Content::Grid(std::sync::Arc::new(GridElem {
            columns: vec![TrackSizing::Fixed(50.0); cols],
            rows: vec![TrackSizing::Fixed(25.0)],
            cells: vec![Content::text("A"); cols],
            hlines: vec![],
            vlines: vec![],
            gutter: None,
            align: None,
            inset: Sides::uniform(Length::ZERO),
            header: None,
            footer: None,
            stroke: Some(Stroke::default()),
            fill: None,
        }))
    }

    fn bounds(items: &[FrameItem]) -> (f64, f64, f64) {
        let mut left = f64::INFINITY;
        let mut right = f64::NEG_INFINITY;
        let mut top = f64::INFINITY;
        for item in items {
            if let FrameItem::Shape {
                pos, kind: ShapeKind::Line { dx, dy: 0.0 }, ..
            } = item
            {
                left = left.min(pos.x.0 + 0.5);
                right = right.max(pos.x.0 + dx - 0.5);
                top = top.min(pos.y.0);
            }
        }
        (left, right, top)
    }

    #[test]
    fn aligned_grid_uses_geometric_top_and_physical_anchor_once() {
        let intr = TagIntrospector::empty();
        for dir in [Dir::LTR, Dir::RTL] {
            for cols in [1, 2] {
                for h in [HAlign::Left, HAlign::Center, HAlign::Right] {
                    let mut l = Layouter::new(
                        FixedMetrics,
                        NullImageSizer,
                        11.0,
                        (&intr as &dyn Introspector).track(),
                    );
                    l.chain = l.chain.push_custom("text.dir", Value::Dir(dir));
                    l.style = TextStyle::from(&l.chain);
                    let origin = l.regions.current.line_start_x.0;
                    let width = l.available_width();
                    let top = l.regions.current.cursor_y.0;
                    let free = width - cols as f64 * 50.0;
                    let x = match h {
                        HAlign::Left => origin,
                        HAlign::Center => origin + free / 2.0,
                        HAlign::Right => origin + free,
                        _ => unreachable!(),
                    };
                    l.layout_align(Align2D { h: Some(h), v: None }, &grid(cols));
                    assert!(l.explicit_line_alignment.is_none());
                    let (left, right, actual_top) =
                        bounds(&l.regions.current.current_items);
                    assert!(
                        (actual_top - top).abs() < 1e-6,
                        "dir={dir:?} cols={cols} h={h:?} top={actual_top} expected={top}"
                    );
                    assert!(
                        (left - x).abs() < 1e-6,
                        "dir={dir:?} cols={cols} h={h:?} x={left} expected={x}"
                    );
                    assert!((right - left - cols as f64 * 50.0).abs() < 1e-6);
                }
            }
        }
    }
}
