//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/math/layout/sequence_composition.md
//! @prompt-hash 7f76dd0c
//! @layer L1
//! @updated 2026-10-01

use super::*;

impl<'a, M: FontMetrics> MathLayouter<'a, M> {
    pub(super) fn layout_sequence(
        &self,
        nodes: &[Content],
        style: &TextStyle,
    ) -> MathBox {
        if self.block && needs_grid_layout(nodes) {
            self.layout_grid(nodes, style)
        } else {
            let mut flat = Vec::new();
            flatten_math_sequence_nodes(nodes, &mut flat);

            let mut filtered: Vec<Content> = flat
                .into_iter()
                .filter(|n| {
                    !matches!(n, Content::MathAlignPoint(_) | Content::Linebreak(_))
                })
                .collect();
            // **P1132m** — `HElem(..., weak:true)` colapsa nas bordas do
            // run. `dif` usa exactamente esse prefixo fino: preserva-o entre
            // dois itens, mas não cria margem em `$dif x$`.
            let is_weak_hspace = |n: &Content| matches!(n, Content::HSpace(e) if e.weak);
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
                        filtered[index + 1..].iter().any(|next| !match next {
                            Content::Empty => true,
                            Content::HSpace(e) if e.weak => true,
                            _ => false,
                        });
                    let Content::MathText(text) = node else {
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
                        (Content::MathText(trimmed.into()), true)
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
                if let Some(Content::Text(text)) = spacing_view.get(index + 1).cloned() {
                    spacing_view[index + 1] = Content::MathText(text);
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
