//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/finish.md
//! @prompt-hash 93f09429
//! @layer L1
//! @updated 2026-09-30

use super::*;

impl<'a, M: FontMetrics, S: ImageSizer> Layouter<'a, M, S> {
    pub fn finish(mut self) -> PagedDocument {
        // P842 (#38) — a última linha também expande h(Nfr) pendentes
        // (paridade vanilla: fr consome o espaço restante mesmo na linha
        // final do documento — medido em `temp/p842/l7_h_1fr.typ`).
        // **P1119** — o flush da última linha passa pelo caminho normal
        // (drena, alinha e aplica a descida de baseline de P1120).
        let had_last_line =
            html_frame::items_have_causal_line(&self.regions.current.current_line);
        let had_terminal_paragraph = self.terminal_paragraph_line;
        self.flush_line();
        let had_causal_last_line = had_last_line || had_terminal_paragraph;
        // Terminal geometry uses the last causal baseline; note fitting keeps
        // the conservative clearance that the following baseline represented.
        let terminal_note_clearance = had_causal_last_line.then_some((
            (self.pages.len(), self.current_column),
            self.regions.current.cursor_y,
        ));
        if had_causal_last_line {
            self.regions.current.cursor_y = Pt(self.prev_line_baseline);
            if self.last_block_descent_y.is_none() {
                self.last_block_descent_y = Some(self.prev_line_baseline);
            }
        }

        if self.initial_baseline_pending
            && self.last_block_descent_y.is_none()
            && !html_frame::items_have_material(&self.regions.current.current_items)
        {
            self.regions.current.cursor_y += Pt(-self.empty_paragraph_weak_spacing);
        }

        // **P867** — dimensões finais quando `width: auto` / `height: auto`.
        // Calculadas antes de flush de floats/footnotes para servir de
        // referencial aos mecanismos de rodapé.
        let page_width = if self.page_config.width.is_infinite() {
            self.compute_page_width()
        } else {
            self.page_config.width
        };
        let page_height = if self.page_config.height.is_infinite() {
            self.compute_page_height()
        } else {
            self.page_config.height
        };
        let footnote_bottom_y = if self.page_config.height.is_infinite() {
            Some(page_height - self.page_config.margin.bottom)
        } else {
            None
        };

        // P245 (M9d / M7+4) — flush floats pendentes da última página
        // antes de comitar a Page final.
        self.flush_pending_floats();
        let note_region = (self.pages.len(), self.current_column);
        let note_causal_cursor = self.regions.current.cursor_y;
        if !self.pending_footnote_bodies.is_empty() {
            if let Some((region, clearance)) = terminal_note_clearance {
                if region == note_region {
                    self.regions.current.cursor_y =
                        Pt(self.regions.current.cursor_y.0.max(clearance.0));
                }
            }
        }
        // P304 (P295.1) — flush footnote bodies pendentes da última
        // página antes de comitar a Page final. Subpadrão DeferredX
        // N=3 paralelo a P245/P251.
        self.flush_pending_footnote_bodies(footnote_bottom_y);
        // P305 (P295.2) — overflow: se bodies sobraram no buffer,
        // criar páginas adicionais (`new_page()`) até buffer vazio.
        // Cada iteração: new_page() saves current page + flush
        // next batch. Iter limit defensivo (paralelo P251
        // forwarded_count limit) — assume cada iteração emite ≥1
        // body via defensive first-body fallback.
        let mut iter_limit = self.pending_footnote_bodies.len() + 1;
        while !self.pending_footnote_bodies.is_empty() && iter_limit > 0 {
            self.new_page();
            self.flush_pending_footnote_bodies(footnote_bottom_y);
            iter_limit -= 1;
        }
        if (self.pages.len(), self.current_column) == note_region {
            self.regions.current.cursor_y = note_causal_cursor;
        }
        if self.pages.is_empty()
            || html_frame::items_have_material(&self.regions.current.current_items)
            || !self.page_run_boundary_empty
        {
            let page_numbering = self.page_config.numbering.clone();
            let page_number = self.pages.len() + 1;
            let page = std::num::NonZeroUsize::new(page_number).unwrap();
            let margin_number = self
                .runtime
                .known_page_store
                .logical_number_for_page(page)
                .unwrap_or(page_number);
            let mut items = std::mem::take(&mut self.regions.current.current_items);
            self.apply_pending_par_line_fixups(&mut items);
            // **P896** — resolve centragem/numeração de equação de bloco
            // adiadas por `width: auto` (ver `equation.rs`), agora que
            // `page_width`/`page_height` já estão finitos.
            self.apply_pending_equation_fixups(&mut items, page_width);
            // **P897** — mesma resolução para `Content::Align`/`Content::Place`
            // adiados por `width: auto` (ver `placement.rs`).
            self.apply_pending_align_fixups(&mut items, page_width);
            // **P898** — simétrico de P897, eixo vertical (`height: auto`).
            self.apply_pending_align_v_fixups(&mut items, page_height);
            let mut marginal_items =
                page_running::explicit_layers(&mut self, page_width, page_height);

            // **P532** — numeração automática na última página.
            // **P538d** — o texto de numeração deve usar o estilo activo da
            // página (StyleChain), não `TextStyle::regular`, para que `font`
            // esteja definida. Mesma correção de P483 para texto normal.
            // **P541** — padrões compostos (≥2 tokens de numeração) adiam-se
            // porque precisam do total de páginas, só conhecido no final.
            let number_marginal = match self.page_config.number_align.vertical {
                crate::entities::page_running::PageNumberVAlign::Top => {
                    &self.page_config.header
                }
                crate::entities::page_running::PageNumberVAlign::Bottom => {
                    &self.page_config.footer
                }
            };
            if page_running::numbering_enabled(number_marginal) {
                if let Some(crate::entities::numbering::Numbering::Pattern(pattern)) =
                    &page_numbering
                {
                    if count_numbering_tokens(pattern) >= 2 {
                        let ha = page_running::resolve_offset(
                            self.page_config.header_ascent,
                            self.page_config.margin.top,
                            self.style.size.0,
                        );
                        let fd = page_running::resolve_offset(
                            self.page_config.footer_descent,
                            self.page_config.margin.bottom,
                            self.style.size.0,
                        );
                        self.pending_page_numbering.push((
                            self.pages.len(),
                            margin_number,
                            pattern.clone(),
                            self.page_config.number_align,
                            self.page_config.margin,
                            ha,
                            fd,
                        ));
                    } else if let Some(text) =
                        crate::entities::counter_format::format_counter(
                            &[margin_number],
                            pattern.as_str(),
                        )
                    {
                        let style = TextStyle::from(&self.chain);
                        let text_width =
                            self.metrics.advance(&text, style.size, &style).0;
                        let ha = page_running::resolve_offset(
                            self.page_config.header_ascent,
                            self.page_config.margin.top,
                            self.style.size.0,
                        );
                        let fd = page_running::resolve_offset(
                            self.page_config.footer_descent,
                            self.page_config.margin.bottom,
                            self.style.size.0,
                        );
                        marginal_items.push(FrameItem::Text {
                            pos: page_running::number_position(
                                self.page_config.number_align,
                                page_width,
                                page_height,
                                self.page_config.margin,
                                text_width,
                                ha,
                                fd,
                            ),
                            text: text.into(),
                            style,
                            bidi: None,
                        });
                    }
                }
                if matches!(
                    page_numbering,
                    Some(crate::entities::numbering::Numbering::Func(_))
                ) {
                    if let Some(content) = self
                        .runtime
                        .known_page_store
                        .visible_numbering_for_page(page)
                        .cloned()
                    {
                        marginal_items.extend(page_running::realized_numbering_layer(
                            &mut self,
                            &content,
                            page_width,
                            page_height,
                        ));
                    }
                }
            }

            let bleed = crate::entities::page_canvas::PageBleed::resolve(
                self.page_config.bleed_spec,
                page_width,
                page_height,
                self.page_config.binding.resolve(self.page_config.page_dir),
                std::num::NonZeroUsize::new(self.pages.len() + 1).unwrap(),
                self.style.size.0,
            );
            let background_content = self.page_config.background.clone();
            let foreground_content = self.page_config.foreground.clone();
            let background = page_canvas::layout_layer(
                &mut self,
                background_content.as_ref(),
                bleed,
                page_width,
                page_height,
            );
            let mut foreground = page_canvas::layout_layer(
                &mut self,
                foreground_content.as_ref(),
                bleed,
                page_width,
                page_height,
            );
            foreground.extend(marginal_items);
            let page = Page {
                width: page_width,
                height: page_height,
                numbering: page_numbering,
                logical_number: page_number,
                supplement: self.page_config.supplement.resolve(self.chain.lang()),
                bleed,
                fill: self.page_config.fill.clone(),
                background,
                foreground,
                items,
            };
            self.pages.push(page);
        }

        // **P541** — aplicar numerações de página adiadas agora que o total
        // de páginas é conhecido. Cada entrada pendente contém o índice da
        // página, o número da página e o pattern composto.
        let total_pages = self.pages.len();
        let logical_total = std::num::NonZeroUsize::new(total_pages)
            .and_then(|page| self.runtime.known_page_store.logical_number_for_page(page))
            .unwrap_or(total_pages);
        for (page_idx, page_number, pattern, align, margins, ha, fd) in
            std::mem::take(&mut self.pending_page_numbering)
        {
            if let Some(text) = crate::entities::counter_format::format_counter(
                &[page_number, logical_total],
                pattern.as_str(),
            ) {
                if let Some(page) = self.pages.get_mut(page_idx) {
                    let style = TextStyle::from(&self.chain);
                    let text_width = self.metrics.advance(&text, style.size, &style).0;
                    page.foreground.push(FrameItem::Text {
                        pos: page_running::number_position(
                            align,
                            page.width,
                            page.height,
                            margins,
                            text_width,
                            ha,
                            fd,
                        ),
                        text: text.into(),
                        style,
                        bidi: None,
                    });
                }
            }
        }

        self.resolve_links();
        self.resolve_position_markers();
        let mut doc = PagedDocument::new(self.pages);
        doc.document_language = self.document_language;
        doc.attachments = self.attachments;
        // Expor o mapa de páginas sem mudar a assinatura de layout() (Passo 63).
        // P190C (M6 categoria Page tracking): label_pages movido para
        // LayouterRuntimeState.
        doc.extracted_label_pages = self.runtime.label_pages;
        // P460 — expor posições dos labels para /Dests no export PDF.
        doc.extracted_label_positions = self.runtime.label_positions;
        // P205B (F3): sealing point — extrai runtime.positions para
        // sub-store sealed `SealedPositions` per ADR-0074. Tracked
        // via comemo; consumer migration em P205C.
        doc.extracted_positions =
            crate::entities::sealed_positions::SealedPositions::from_runtime(
                self.runtime.positions,
            );
        // P488 — expor páginas de figuras/tabelas para fixpoint carry-forward (LoF/LoT).
        doc.extracted_figure_page_numbers = self.runtime.figure_page_numbers;
        doc.extracted_table_page_numbers = self.runtime.table_page_numbers;
        // **P595** — exportar avisos de layout acumulados no Layouter.
        doc.layout_warnings = self.layout_warnings;
        doc.layout_errors = self.layout_errors;
        doc
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn modulo_compila_e_carrega() {}
}
