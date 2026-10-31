//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/math/layout/sequence_composition.md
//! @prompt-hash 48b55688
//! @layer L1
//! @updated 2026-10-31

use super::*;

impl<'a, M: FontMetrics> MathLayouter<'a, M> {
    pub(super) fn layout_symbol(&self, text: &EcoString, style: &TextStyle) -> MathBox {
        use unicode_segmentation::UnicodeSegmentation;
        let glyphs: Vec<Content> = text
            .graphemes(true)
            .map(|cluster| Content::MathIdent(cluster.into()))
            .collect();
        self.layout_sequence(&glyphs, style)
    }

    pub(super) fn layout_sequence(
        &self,
        nodes: &[Content],
        style: &TextStyle,
    ) -> MathBox {
        let expanded = self.lr_multiline_nodes(nodes, style);
        let nodes = expanded.as_deref().unwrap_or(nodes);
        if self.block && needs_grid_layout(nodes) {
            self.layout_grid(nodes, style)
        } else {
            let mut flat = Vec::new();
            flatten_math_sequence_nodes(nodes, &mut flat);

            let mut filtered: Vec<Content> = flat
                .into_iter()
                .filter(|n| {
                    !matches!(
                        n.source_body(),
                        Content::MathAlignPoint(_) | Content::Linebreak(_)
                    )
                })
                .collect();
            // **P1132m** — `HElem(..., weak:true)` colapsa nas bordas do
            // run. `dif` usa exactamente esse prefixo fino: preserva-o entre
            // dois itens, mas não cria margem em `$dif x$`.
            let is_weak_hspace =
                |n: &Content| matches!(n.source_body(), Content::HSpace(e) if e.weak);
            let first_material = filtered.iter().position(|n| !is_weak_hspace(n));
            let last_material = filtered.iter().rposition(|n| !is_weak_hspace(n));
            if let (Some(first), Some(last)) = (first_material, last_material) {
                filtered = filtered
                    .into_iter()
                    .enumerate()
                    .filter_map(|(i, n)| {
                        (!is_weak_hspace(&n) || (i > first && i < last)).then_some(n)
                    })
                    .collect();
            } else {
                filtered.clear();
            }
            // P1293 — vista transitória de layout: um separador
            // pontuacional `MathText` com whitespace terminal e irmão
            // material seguinte conserva a pontuação, mas não materializa o
            // mesmo limite duas vezes (avanço textual + gap de classe). O
            // payload original permanece intocado.
            let normalized: Vec<(Content, bool)> = filtered
                .iter()
                .enumerate()
                .map(|(index, node)| {
                    let has_next_material =
                        filtered[index + 1..].iter().any(
                            |next| !match next.source_body() {
                                Content::Empty => true,
                                Content::HSpace(e) if e.weak => true,
                                Content::Source(_)
                                | Content::ShowRecipe(_)
                                | Content::Realized(_)
                                | Content::Located(_)
                                | Content::Text(_)
                                | Content::Space
                                | Content::SpaceFromNewline
                                | Content::Parbreak
                                | Content::Sequence(_)
                                | Content::HtmlElem(_)
                                | Content::HtmlFrame(_)
                                | Content::Par { .. }
                                | Content::Heading(_)
                                | Content::Title(_)
                                | Content::Strong(_)
                                | Content::Emph(_)
                                | Content::Raw(_)
                                | Content::ListItem(_)
                                | Content::EnumItem(_)
                                | Content::Link(_)
                                | Content::Equation(_)
                                | Content::MathSequence(_)
                                | Content::Symbol(_)
                                | Content::MathIdent(_)
                                | Content::MathText(_)
                                | Content::MathFrac(_)
                                | Content::MathAttach(_)
                                | Content::MathPrimes(_)
                                | Content::MathRoot(_)
                                | Content::MathStretch(_)
                                | Content::MathDelimited(_)
                                | Content::MathLr(_)
                                | Content::MathMid(_)
                                | Content::MathOverbrace(_)
                                | Content::MathUnderbrace(_)
                                | Content::MathUnderbracket(_)
                                | Content::MathUnderparen(_)
                                | Content::MathUndershell(_)
                                | Content::MathOverbracket(_)
                                | Content::MathOverparen(_)
                                | Content::MathOvershell(_)
                                | Content::MathAlignPoint(_)
                                | Content::Linebreak(_)
                                | Content::MathMatrix(_)
                                | Content::MathCases(_)
                                | Content::MathAccent(_)
                                | Content::MathCancel(_)
                                | Content::MathUnderline(_)
                                | Content::MathOverline(_)
                                | Content::MathVec(_)
                                | Content::MathClassOverride(_)
                                | Content::MathLimitsOverride(_)
                                | Content::MathUnderover(_)
                                | Content::MathOp(_)
                                | Content::MathStyled(_)
                                | Content::Label(_)
                                | Content::Ref(_)
                                | Content::CounterDisplay(_)
                                | Content::CounterUpdate(_)
                                | Content::Outline(_)
                                | Content::OutlineEntry(_)
                                | Content::Figure(_)
                                | Content::FigureCaption(_)
                                | Content::Image(_)
                                | Content::Shape(_)
                                | Content::Curve(_)
                                | Content::Transform(_)
                                | Content::Grid(_)
                                | Content::GridHeader(_)
                                | Content::GridFooter(_)
                                | Content::GridCell(_)
                                | Content::SetPage(_)
                                | Content::PageRun(_)
                                | Content::Align(_)
                                | Content::Place(_)
                                | Content::Flush(_)
                                | Content::Styled(_)
                                | Content::Divider(_)
                                | Content::Terms(_)
                                | Content::TermItem(_)
                                | Content::Quote(_)
                                | Content::Document { .. }
                                | Content::Asset { .. }
                                | Content::SmartQuote(_)
                                | Content::PdfAttach(_)
                                | Content::PdfArtifact(_)
                                | Content::Underline(_)
                                | Content::Strike(_)
                                | Content::Overline(_)
                                | Content::SmallCaps { .. }
                                | Content::Pad(_)
                                | Content::Hide(_)
                                | Content::HSpace(_)
                                | Content::VSpace(_)
                                | Content::Pagebreak(_)
                                | Content::Colbreak(_)
                                | Content::Stack(_)
                                | Content::Boxed(_)
                                | Content::Block(_)
                                | Content::TableCell(_)
                                | Content::Bibliography(_)
                                | Content::Cite(_)
                                | Content::Footnote(_)
                                | Content::FootnoteEntry(_)
                                | Content::TableHeader(_)
                                | Content::TableFooter(_)
                                | Content::GridHLine(_)
                                | Content::GridVLine(_)
                                | Content::TableHLine(_)
                                | Content::TableVLine(_)
                                | Content::Table(_)
                                | Content::Repeat(_)
                                | Content::Columns(_)
                                | Content::Metadata(_)
                                | Content::State(_)
                                | Content::StateUpdate(_)
                                | Content::StateDisplay(_)
                                | Content::CounterDisplayCallback(_)
                                | Content::ContextBlock(_)
                                | Content::Dynamic(_) => false,
                            },
                        );
                    let Content::MathText(text) = node.source_body() else {
                        return (node.clone(), false);
                    };
                    let trimmed = text.trim_end_matches(char::is_whitespace);
                    let is_punctuation = !trimmed.is_empty()
                        && trimmed.chars().all(|c| {
                            matches!(
                                crate::entities::math_class::default_math_class(c),
                                Some(crate::entities::math_class::MathClass::Punctuation)
                            )
                        });
                    if has_next_material && trimmed.len() != text.len() && is_punctuation
                    {
                        (
                            Content::MathText(trimmed.into())
                                .with_source_span(node.source_span()),
                            true,
                        )
                    } else {
                        (node.clone(), false)
                    }
                })
                .collect();
            let layout_view: Vec<Content> =
                normalized.iter().map(|(node, _)| node.clone()).collect();
            // A vista de classes impede que o fallback de item textual
            // reintroduza o whitespace que acabou de ser normalizado. A
            // caixa desenhada continua a ser a original em `layout_view`.
            let mut spacing_view = layout_view.clone();
            for (index, (_, was_normalized)) in normalized.iter().enumerate() {
                if !was_normalized {
                    continue;
                }
                if let Some(next) = spacing_view.get(index + 1) {
                    if let Content::Text(text) = next.source_body() {
                        let replacement = Content::MathText(text.clone())
                            .with_source_span(next.source_span());
                        spacing_view[index + 1] = replacement;
                    }
                }
            }
            let text_space_pt = self.metrics.advance(" ", style.size, style).val();
            let gaps = spacing::compute_gaps(
                &spacing_view,
                style.size.val(),
                style.math_script
                    || matches!(
                        style.math_size,
                        MathSize::Script | MathSize::ScriptScript
                    ),
                text_space_pt,
            );
            let mut boxes: Vec<MathBox> =
                layout_view.iter().map(|n| self.layout_node(n, style)).collect();
            for (i, (node, text_box)) in
                layout_view.iter().zip(boxes.iter_mut()).enumerate()
            {
                if should_tighten_text_frame(node, layout_view.get(i + 1)) {
                    text_box.width = (text_box.width - text_space_pt).max(0.0);
                }
            }
            self.hconcat_spaced(boxes, &gaps)
        }
    }

    /// Concatenação horizontal: posiciona MathBoxes lado a lado, sem
    /// espaçamento extra entre eles (abertura+corpo+fecho de delimitadores
    /// — a regra de classe já dá 0pt aí, ADR-0108 P772y).
    pub(super) fn hconcat(&self, boxes: Vec<MathBox>) -> MathBox {
        self.hconcat_spaced(boxes, &[])
    }

    /// Concatenação horizontal com espaçamento extra entre boxes
    /// consecutivas — `gaps[i]` é o espaço (pt) inserido entre a box `i` e
    /// a box `i + 1`. `gaps` mais curto que `boxes.len() - 1` trata os
    /// gaps em falta como 0 (P772y — tabela de espaçamento por MathClass).
    pub(super) fn hconcat_spaced(&self, boxes: Vec<MathBox>, gaps: &[f64]) -> MathBox {
        let mut x = 0.0_f64;
        let mut ascent = 0.0_f64;
        let mut descent = 0.0_f64;
        let mut items = Vec::new();

        for (i, b) in boxes.into_iter().enumerate() {
            if i > 0 {
                x += gaps.get(i - 1).copied().unwrap_or(0.0);
            }
            ascent = ascent.max(b.ascent);
            descent = descent.max(b.descent);
            for mut item in b.items {
                match item {
                    FrameItem::TextSeparator { ref mut pos, .. }
                    | FrameItem::Text { ref mut pos, .. } => {
                        pos.x = Pt(pos.x.val() + x);
                    }
                    FrameItem::TextShaped { ref mut pos, .. } => {
                        pos.x = Pt(pos.x.val() + x);
                    }
                    FrameItem::Line { ref mut start, ref mut end, .. } => {
                        start.x = Pt(start.x.val() + x);
                        end.x = Pt(end.x.val() + x);
                    }
                    FrameItem::Glyph { ref mut pos, .. } => {
                        pos.x = Pt(pos.x.val() + x);
                    }
                    FrameItem::Image { .. } => {} // imagens não ocorrem em contexto math
                    // **P994** — formas OCORREM desde P994 (borda de
                    // `box()` embutido, achatada por `layout_external`);
                    // deslocar em X como as outras variantes posicionadas.
                    FrameItem::Shape { ref mut pos, .. } => {
                        pos.x = Pt(pos.x.val() + x);
                    }
                    // **P994** — grupos OCORREM desde P994 (conteúdo externo
                    // embutido via `layout_external`); deslocar em X como as
                    // outras variantes posicionadas.
                    FrameItem::Group { ref mut pos, .. } => {
                        pos.x = Pt(pos.x.val() + x);
                    }
                    FrameItem::Link { .. } => {} // links não ocorrem em contexto math
                    FrameItem::TextReplacement { ref mut items, .. }
                    | FrameItem::Semantic { ref mut items, .. } => {
                        for child in items {
                            *child = offset_item(child.clone(), Pt(x), Pt(0.0));
                        }
                    }
                }
                items.push(item);
            }
            x += b.width;
        }

        MathBox { width: x, ascent, descent, items }
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn modulo_compila_e_carrega() {}
}
