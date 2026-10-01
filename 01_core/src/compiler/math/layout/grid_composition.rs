//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/math/layout/grid_composition.md
//! @prompt-hash 4cab5c5f
//! @layer L1
//! @updated 2026-10-01

use super::*;

impl<'a, M: FontMetrics> MathLayouter<'a, M> {
    /// Grelha 2D generalizada — usada por `layout_grid` e `MathMatrix`.
    ///
    /// Duas passagens:
    ///   1. Mede todas as células → largura máxima por coluna.
    ///   2. Posiciona com essas larguras + `column_gap` entre colunas.
    pub(super) fn layout_grid_rows(
        &self,
        rows: &[Vec<Content>],
        align: GridAlign,
        column_gap: Pt,
        row_gap: Pt,
        style: &TextStyle,
    ) -> MathBox {
        let n_cols = rows.iter().map(|row| row.len()).max().unwrap_or(0);
        // ── Passagem 1: medir todas as células ────────────────────────────
        let grid_boxes: Vec<Vec<MathBox>> = rows
            .iter()
            .map(|row| row.iter().map(|cell| self.layout_node(cell, style)).collect())
            .collect();
        let _ = n_cols;
        self.layout_grid_boxes(grid_boxes, align, column_gap, row_gap, &[], style)
    }

    /// **P825** — passagem 2 de `layout_grid_rows` (posicionamento de uma
    /// grelha já medida), extraída para que `matrix.rs` possa ajustar as
    /// `MathBox` medidas antes de posicionar (incorporar o espaçamento de
    /// classe do limite `&` na largura da célula par — paridade vanilla
    /// `run.rs:76-94`).
    ///
    /// `align_boundaries[r][i] == true` marca que o limite **antes** da
    /// célula `i` da linha `r` foi produzido por um `&` — esse limite não
    /// leva `column_gap` (o espaçamento já está na largura da célula à
    /// esquerda). Slice vazio = nenhum limite de alinhamento.
    pub(super) fn layout_grid_boxes(
        &self,
        grid_boxes: Vec<Vec<MathBox>>,
        align: GridAlign,
        column_gap: Pt,
        row_gap: Pt,
        align_boundaries: &[Vec<bool>],
        style: &TextStyle,
    ) -> MathBox {
        let n_cols = grid_boxes.iter().map(|row| row.len()).max().unwrap_or(0);
        if n_cols == 0 {
            return MathBox {
                width: 0.0,
                ascent: 0.0,
                descent: 0.0,
                items: vec![],
            };
        }

        let mut col_widths = vec![0.0_f64; n_cols];
        for row in &grid_boxes {
            for (col_idx, cell_box) in row.iter().enumerate() {
                col_widths[col_idx] = col_widths[col_idx].max(cell_box.width);
            }
        }

        // **P921/P923** — piso de altura por linha: cada linha fica no mínimo
        // tão alta quanto um `(` sintético em estilo de denominador (vanilla,
        // `table.rs:67-85`: "pad ascent/descent with the paren's, to ensure
        // that normal matrices are aligned with others unless they are way
        // too big"). O tamanho do `(` é o do estilo de denominador do ambiente
        // exterior. Quando `layout_grid_boxes` é chamado por matrizes/cases
        // (P923), o `style` recebido já é esse estilo de denominador
        // (**P945**: via `denominator_style` — descida por nível MathSize,
        // `_comum.md` §P945), pelo que não se aplica o factor outra vez. Para multiline math
        // (`layout_grid`), o `style` é o exterior e o piso continua a usar o
        // tamanho do denominador — impacto mínimo e consistente com a
        // intenção do piso.
        let paren_style = TextStyle { cramped: true, ..style.clone() };
        let paren_box = self.layout_text_node(&EcoString::from("("), &paren_style);
        let (paren_ascent, paren_descent) = (paren_box.ascent, paren_box.descent);

        // ── Passagem 2: posicionar células ────────────────────────────────
        let mut all_items: Vec<FrameItem> = Vec::new();
        let gap = column_gap.val();

        let total_ascent = grid_boxes
            .first()
            .map(|row| row.iter().map(|b| b.ascent).fold(0.0, f64::max).max(paren_ascent))
            .unwrap_or(0.0);
        let mut total_descent = grid_boxes
            .first()
            .map(|row| {
                row.iter().map(|b| b.descent).fold(0.0, f64::max).max(paren_descent)
            })
            .unwrap_or(0.0);

        let n_rows = grid_boxes.len();
        let mut row_baselines = vec![0.0_f64; n_rows];
        let mut cur_baseline = 0.0_f64;
        for row_idx in 0..n_rows {
            row_baselines[row_idx] = cur_baseline;
            if row_idx + 1 < n_rows {
                let row_descent = grid_boxes[row_idx]
                    .iter()
                    .map(|b| b.descent)
                    .fold(0.0, f64::max)
                    .max(paren_descent);
                let next_row = &grid_boxes[row_idx + 1];
                let next_row_ascent = next_row
                    .iter()
                    .map(|b| b.ascent)
                    .fold(0.0, f64::max)
                    .max(paren_ascent);
                let next_row_descent = next_row
                    .iter()
                    .map(|b| b.descent)
                    .fold(0.0, f64::max)
                    .max(paren_descent);
                let advance = row_descent + row_gap.val() + next_row_ascent;
                cur_baseline += advance;
                total_descent += row_gap.val() + next_row_ascent + next_row_descent;
            }
        }

        let mut col_offsets = vec![0.0_f64; n_cols];
        let mut cur_col_x = 0.0_f64;
        for col_idx in 0..n_cols {
            col_offsets[col_idx] = cur_col_x;
            cur_col_x += col_widths[col_idx];
            if col_idx + 1 < n_cols {
                cur_col_x += gap;
            }
        }
        let max_row_width = cur_col_x;

        if matches!(align, GridAlign::Center) {
            // **P1121**: Para matrizes matemáticas (GridAlign::Center), o vanilla emite
            // os nós por colunas (Coluna 0 inteira top-to-bottom, Coluna 1 top-to-bottom...).
            for col_idx in 0..n_cols {
                let col_w = col_widths[col_idx];
                let cursor_x = col_offsets[col_idx];
                for row_idx in 0..n_rows {
                    if let Some(cell_box) = grid_boxes[row_idx].get(col_idx) {
                        let cell_x = cursor_x + (col_w - cell_box.width) / 2.0;
                        let dy = row_baselines[row_idx];
                        for item in cell_box.items.clone() {
                            all_items.push(offset_item(item, Pt(cell_x), Pt(dy)));
                        }
                    }
                }
            }
        } else {
            let mut baseline_offset = 0.0_f64;
            for (row_idx, row) in grid_boxes.iter().enumerate() {
                let mut cursor_x = 0.0_f64;
                for (col_idx, cell_box) in row.iter().enumerate() {
                    let col_w = if col_idx < n_cols { col_widths[col_idx] } else { 0.0 };
                    let cell_x = match align {
                        GridAlign::Alternating => {
                            if col_idx % 2 == 0 {
                                cursor_x + (col_w - cell_box.width)
                            } else {
                                cursor_x
                            }
                        }
                        GridAlign::Center => cursor_x + (col_w - cell_box.width) / 2.0,
                        GridAlign::Left => cursor_x,
                        GridAlign::Right => cursor_x + (col_w - cell_box.width),
                    };
                    let dy = baseline_offset;
                    for item in cell_box.items.clone() {
                        all_items.push(offset_item(item, Pt(cell_x), Pt(dy)));
                    }
                    cursor_x += col_w;
                    if col_idx + 1 < n_cols {
                        let is_align_boundary = align_boundaries
                            .get(row_idx)
                            .and_then(|marks| marks.get(col_idx + 1))
                            .copied()
                            .unwrap_or(false);
                        if !is_align_boundary {
                            cursor_x += gap;
                        }
                    }
                }
                if row_idx + 1 < grid_boxes.len() {
                    let line_gap = row_gap.val();
                    let next_row = &grid_boxes[row_idx + 1];
                    let row_descent = row
                        .iter()
                        .map(|b| b.descent)
                        .fold(0.0, f64::max)
                        .max(paren_descent);
                    let next_row_ascent = next_row
                        .iter()
                        .map(|b| b.ascent)
                        .fold(0.0, f64::max)
                        .max(paren_ascent);
                    let advance = row_descent + line_gap + next_row_ascent;
                    baseline_offset += advance;
                }
            }
        }

        // P825 — sem limites de alinhamento, a largura é a fórmula
        // clássica (colunas + gap fixo), preservando o comportamento
        // anterior para grelhas irregulares; com limites `&`, é a maior
        // largura de linha real (os gaps variam por limite).
        let total_width = if align_boundaries.is_empty() {
            col_widths.iter().sum::<f64>() + n_cols.saturating_sub(1) as f64 * gap
        } else {
            max_row_width
        };

        MathBox {
            width: total_width,
            ascent: total_ascent,
            descent: total_descent,
            items: all_items,
        }
    }

    /// Layout em grelha 2D para equações com `&` e/ou `\\`.
    ///
    /// **P991** — o alinhamento depende de existir algum `&` nos nós de
    /// origem (`n_cols > 1`, o mesmo valor usado para `align_boundaries`):
    /// com `&`, alinhamento alternado (colunas pares à direita, ímpares à
    /// esquerda), paridade `run.rs::stack_rows` quando `has_alignment`;
    /// sem `&` (só `\\`, ex.: `(n \\ k)`), `GridAlign::Center` — paridade
    /// com o default `AlignElem::alignment = CENTER` de `equation.rs`,
    /// usado por `stack_rows` quando `!has_alignment`. Ver `_comum.md`
    /// §P991.
    ///
    /// Sem espaço extra entre colunas (o espaçamento de limite `&`, quando
    /// aplicável, é incorporado na largura da célula esquerda — ver
    /// abaixo). **P967b** — todos os limites produzidos por
    /// `partition_grid` são limites `&` por construção: o espaçamento do
    /// limite (`align_boundary_spacing`, com o fallback de item espaçado de
    /// P903 — ex.: espaço de texto antes de uma anotação entre aspas) é
    /// incorporado na largura da célula esquerda, mesmo modelo de P825 para
    /// matrizes. Antes de P967b passava `&[]` (boundary gap = 0) e as
    /// anotações colavam ao fim da célula ("9dado"). Ver `_comum.md` §P967b.
    pub(super) fn layout_grid(&self, nodes: &[Content], style: &TextStyle) -> MathBox {
        self.layout_grid_with_baselines(nodes, style).0
    }

    pub(super) fn layout_grid_with_baselines(
        &self,
        nodes: &[Content],
        style: &TextStyle,
    ) -> (MathBox, Vec<f64>) {
        let grid = partition_grid(nodes);
        let n_cols = grid.iter().map(|row| row.len()).max().unwrap_or(0);
        // Cada célula é Vec<Content> — envolver em MathSequence para layout_node.
        let rows: Vec<Vec<Content>> = grid
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|cell_nodes| Content::MathSequence(cell_nodes.into()))
                    .collect()
            })
            .collect();
        // **P1134** — o leading de math multilinha é propriedade do estilo
        // de parágrafo no vanilla (`run.rs:49-53`), não a MathConstant da
        // fonte. Em scripts, o vanilla troca para TIGHT_LEADING = 0.25em.
        // Matrizes e `cases` passam o seu próprio row_gap por caminho distinto.
        let row_gap = match style.math_size {
            MathSize::Display | MathSize::Text => Pt(style
                .leading
                .map(|leading| leading.resolve_pt(style.size.val()))
                .unwrap_or(PAR_LEADING * style.size.val())),
            // ref: lab/typst-original/crates/typst-layout/src/math/run.rs:15
            MathSize::Script | MathSize::ScriptScript => Pt(0.25 * style.size.val()),
        };

        // Medir as células e incorporar o espaçamento do limite `&` na
        // largura da célula à esquerda (paridade vanilla `run.rs:76-94`).
        let mut grid_boxes: Vec<Vec<MathBox>> = rows
            .iter()
            .map(|row| row.iter().map(|cell| self.layout_node(cell, style)).collect())
            .collect();
        for (row_idx, row) in rows.iter().enumerate() {
            for col_idx in 1..row.len() {
                let gap =
                    self.align_boundary_spacing(&row[col_idx - 1], &row[col_idx], style);
                if let Some(left_box) = grid_boxes[row_idx].get_mut(col_idx - 1) {
                    left_box.width += gap;
                }
            }
        }
        // Todos os limites internos são `&` (ver doc do método).
        let align_boundaries: Vec<Vec<bool>> = rows
            .iter()
            .map(|row| (0..row.len()).map(|i| i > 0).collect())
            .collect();
        // **P991** — sem `&` (n_cols <= 1), centrar; com `&`, alternar.
        let align = if n_cols > 1 { GridAlign::Alternating } else { GridAlign::Center };
        let paren_style = TextStyle { cramped: true, ..style.clone() };
        let paren_box = self.layout_text_node(&EcoString::from("("), &paren_style);
        let mut baselines = Vec::with_capacity(grid_boxes.len());
        let mut baseline = 0.0;
        for (row_idx, row) in grid_boxes.iter().enumerate() {
            baselines.push(baseline);
            if let Some(next) = grid_boxes.get(row_idx + 1) {
                let descent = row
                    .iter()
                    .map(|b| b.descent)
                    .fold(0.0, f64::max)
                    .max(paren_box.descent);
                let next_ascent = next
                    .iter()
                    .map(|b| b.ascent)
                    .fold(0.0, f64::max)
                    .max(paren_box.ascent);
                baseline += descent + row_gap.val() + next_ascent;
            }
        }
        let math_box = self.layout_grid_boxes(
            grid_boxes,
            align,
            Pt(0.0),
            row_gap,
            &align_boundaries,
            style,
        );
        (math_box, baselines)
    }
}
