//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/math/layout/matrix.md
//! @prompt-hash e7135420
//! @layer L1
//! @updated 2026-04-23
//!
//! Método `layout_matrix` de `MathLayouter`. Extraído de `math/layout/mod.rs`
//! no Passo 96.8 conforme ADR-0037.

use crate::compiler::layout::FontMetrics;
use crate::entities::{
    content::Content,
    layout_types::{FrameItem, Pt, TextStyle},
};

use super::GridAlign;
use super::{offset_item, MathBox};

impl<'a, M: FontMetrics> super::MathLayouter<'a, M> {
    pub(super) fn layout_matrix(
        &self,
        rows: &[Vec<Content>],
        delim: (char, char),
        style: &TextStyle,
    ) -> MathBox {
        // **P1053/P1054** — `DEFAULT_COL_GAP = 0.5em` do vanilla lab/typst-original/crates/typst-library/src/math/matrix.rs:16`.
        let col_gap = style.size * 0.5;
        // **P923b/P1054** — `row_gap` resolvido contra o estilo exterior, igual ao
        // `DEFAULT_ROW_GAP = 0.2em` do `MatElem` no vanilla (lab/typst-original/crates/typst-library/src/math/matrix.rs:15`).
        let row_gap = style.size * 0.2;

        // **P923** — células de `mat` renderizadas em estilo de denominador
        // (vanilla `resolve_cells` aplica `style_for_denominator`): `MathSize`
        // desce um nível e `cramped` é forçado a `true`. **P945** — a descida
        // passa a ser **por nível** (`denominator_style`, `_comum.md` §P945:
        // `Display→Text` ×1.0, `Text→Script` ×script_percent,
        // `Script→ScriptScript` ×sscript/script, `ScriptScript` ×1.0), em vez
        // do ×`script_percent_scale_down` incondicional de P923 — medido
        // correcto só em inline (Text→Script), errado em bloco (Display→Text,
        // vanilla `style.rs:343-363`).
        let cell_style = self.denominator_style(style);

        // **P825 (sub-achado D de P810 §12)** — `&` dentro de células de
        // `mat`: parte a célula em colunas de alinhamento
        // (`LeftRightAlternator::Right` do vanilla — pares à direita,
        // ímpares à esquerda). Sem `&` em nenhuma célula, comportamento
        // inalterado (colunas centradas — paridade medida em P810 §12).
        let mut any_align = false;
        let mut split_rows: Vec<Vec<Content>> = Vec::with_capacity(rows.len());
        let mut align_boundaries: Vec<Vec<bool>> = Vec::with_capacity(rows.len());
        for row in rows {
            let mut out_row: Vec<Content> = Vec::new();
            let mut marks: Vec<bool> = Vec::new();
            for cell in row {
                let parts = split_cell_on_align_point(cell);
                let n = parts.len();
                for (i, part) in parts.into_iter().enumerate() {
                    out_row.push(part);
                    // Marca o limite ANTES da célula: `true` só para
                    // limites produzidos por `&` (não por vírgula).
                    marks.push(i > 0);
                }
                if n > 1 {
                    any_align = true;
                }
            }
            split_rows.push(out_row);
            align_boundaries.push(marks);
        }

        let grid_box = if any_align {
            // Medir as células e incorporar o espaçamento de classe do
            // limite `&` na largura da célula par à esquerda (paridade
            // vanilla `run.rs:76-94`: o lspace do item da coluna ímpar —
            // ex.: THICK antes de `=` — é movido para a célula par).
            let mut grid_boxes: Vec<Vec<MathBox>> = split_rows
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|cell| self.layout_node(cell, &cell_style))
                        .collect()
                })
                .collect();
            for (row_idx, row) in split_rows.iter().enumerate() {
                for col_idx in 1..row.len() {
                    if !align_boundaries[row_idx][col_idx] {
                        continue;
                    }
                    let gap = self.align_boundary_spacing(
                        &row[col_idx - 1],
                        &row[col_idx],
                        &cell_style,
                    );
                    if let Some(left_box) =
                        grid_boxes[row_idx].get_mut(col_idx - 1)
                    {
                        left_box.width += gap;
                    }
                }
            }
            self.layout_grid_boxes(
                grid_boxes,
                GridAlign::Alternating,
                col_gap,
                row_gap,
                &align_boundaries,
                &cell_style,
            )
        } else {
            self.layout_grid_rows(rows, GridAlign::Center, col_gap, row_gap, &cell_style)
        };

        // Converter altura da grelha de Pt para Design Units para layout_stretchy_delimiter (P912: margem de 10%).
        let min_height_du = self.grid_delim_target_du(&grid_box, style);

        let left_box = if delim.0 != '\0' {
            self.layout_stretchy_delimiter(delim.0, min_height_du, style)
        } else {
            MathBox { width: 0.0, ascent: 0.0, descent: 0.0, items: Vec::new() }
        };
        let right_box = if delim.1 != '\0' {
            self.layout_stretchy_delimiter(delim.1, min_height_du, style)
        } else {
            MathBox { width: 0.0, ascent: 0.0, descent: 0.0, items: Vec::new() }
        };

        // **P919** — centra APENAS a grelha no eixo matemático (vanilla
        // `table.rs:188`) — os delimitadores já vêm pré-centrados por
        // `layout_stretchy_delimiter` e não devem ser deslocados de novo.
        // `min_height_du` (acima) já usou o `grid_box` ORIGINAL — o offset
        // só acontece depois de já ter dimensionado os delimitadores. Ver
        // `matrix.md` §P919.
        let grid_box = self.apply_axis_offset(grid_box, style.size);

        // **P1042** — delimitadores colocados diretamente adjacentes à grelha
        // sem padding manual redundante (+0.1em em cada lado removido per vanilla).
        let mut items: Vec<FrameItem> = Vec::new();
        let mut x = Pt(0.0);

        if delim.0 != '\0' {
            for item in left_box.items.iter() {
                items.push(offset_item(item.clone(), x, Pt(0.0)));
            }
            x = x + Pt(left_box.width);
        }

        for item in grid_box.items.iter() {
            items.push(offset_item(item.clone(), x, Pt(0.0)));
        }
        x = x + Pt(grid_box.width);

        if delim.1 != '\0' {
            for item in right_box.items.iter() {
                items.push(offset_item(item.clone(), x, Pt(0.0)));
            }
            x = x + Pt(right_box.width);
        }

        MathBox {
            width: x.val(),
            ascent: grid_box.ascent,
            descent: grid_box.descent,
            items,
        }
    }

    /// **P825/P967b** — espaçamento no limite `&` entre duas células (paridade
    /// `alignment_lspace` do vanilla, `run.rs:363-373`: o lspace do primeiro
    /// item da célula seguinte — ex.: THICK antes de uma relação).
    ///
    /// **P967b** — passa a ter a mesma semântica de `compute_gaps` (P903):
    /// se o match de classes cair no catch-all E um dos dois lados da aresta
    /// for `Content::Text` (ou `Fence` como classe — paridade `is_spaced()` do
    /// vanilla, `spacing.rs:211-222`), devolve a largura real de um espaço de
    /// texto (`metrics.advance(" ", …)`) em vez de 0.0. Regras explícitas de
    /// 0.0 (pontuação/abertura/fecho) continuam a ganhar. Método de
    /// `MathLayouter` (precisa de `self.metrics`) — era free fn privada em
    /// P825. Ver `_comum.md` §P967b.
    pub(super) fn align_boundary_spacing(
        &self,
        left: &Content,
        right: &Content,
        style: &TextStyle,
    ) -> f64 {
        let left_edge = edge_node(left, false);
        let right_edge = edge_node(right, true);
        let (_, l_rclass) = super::spacing::node_math_class(left_edge);
        let (r_lclass, _) = super::spacing::node_math_class(right_edge);
        match super::spacing::spacing_between_class(l_rclass, r_lclass, style.size.val()) {
            Some(v) => v,
            None => {
                let is_spaced = matches!(left_edge, Content::Text(_))
                    || matches!(right_edge, Content::Text(_))
                    || l_rclass == crate::entities::math_class::MathClass::Fence
                    || r_lclass == crate::entities::math_class::MathClass::Fence;
                if is_spaced {
                    self.metrics.advance(" ", style.size, style).val()
                } else {
                    0.0
                }
            }
        }
    }
}

/// **P825/P1042** — parte o `Content` de uma célula de `mat` ou `cases` nos
/// `MathAlignPoint` (`&`). Sem `&`, devolve a célula inalterada (1 parte).
pub(super) fn split_cell_on_align_point(cell: &Content) -> Vec<Content> {
    if let Content::MathSequence(items) = cell {
        if items.iter().any(|c| matches!(c, Content::MathAlignPoint(_))) {
            let mut parts: Vec<Vec<Content>> = vec![vec![]];
            for item in items.iter() {
                match item {
                    Content::MathAlignPoint(_) => parts.push(vec![]),
                    other => parts.last_mut().unwrap().push(other.clone()),
                }
            }
            return parts
                .into_iter()
                .map(|part| match part.len() {
                    0 => Content::Empty,
                    1 => part.into_iter().next().unwrap(),
                    _ => Content::MathSequence(part.into()),
                })
                .collect();
        }
    }
    vec![cell.clone()]
}

/// **P825/P1042** — primeiro/último nó "real" de uma célula (para a classe do
/// limite). `MathSequence` → primeiro/último item; `Empty`/outros → a
/// própria célula.
pub(super) fn edge_node<'c>(cell: &'c Content, first: bool) -> &'c Content {
    if let Content::MathSequence(items) = cell {
        let edge = if first { items.first() } else { items.last() };
        if let Some(node) = edge {
            return node;
        }
    }
    cell
}


#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {}
}
