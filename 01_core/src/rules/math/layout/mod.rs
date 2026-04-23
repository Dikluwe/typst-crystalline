//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/math/layout.md
//! @prompt-hash 38b51727
//! @layer L1
//! @updated 2026-04-11

use ecow::EcoString;

use crate::entities::{
    content::Content,
    layout_types::{FrameItem, Point, Pt, TextStyle},
    math_constants::MathConstants,
};
use crate::rules::layout::FontMetrics;
use super::symbols;

// Sub-métodos do layout matemático extraídos por fase (Passo 96.8, ADR-0037).
mod attach;
mod root;
mod frac;
mod matrix;
mod cases;
mod delimited;
mod stretchy;
mod assembly;

/// Caixa tipográfica de um nó matemático.
/// Todas as medidas são em pontos, relativas à baseline da equação.
/// `ascent` > 0 (acima da baseline), `descent` > 0 (abaixo da baseline).
//
// Visibilidade `pub(super)` nos campos (Passo 96.8): os submódulos
// `attach.rs`, `root.rs`, `frac.rs`, `stretchy.rs`, `grid_layout.rs`
// constroem e compõem `MathBox` directamente — os acessos são quase
// todos em construção (`MathBox { .. }`) ou leitura simples. A ADR-0037
// Regra 3 autoriza `pub(super)` quando métodos não agregam invariante.
#[derive(Debug, Clone)]
pub(super) struct MathBox {
    pub(super) width:   f64,
    pub(super) ascent:  f64,
    pub(super) descent: f64,
    /// Items com posições relativas ao topo esquerdo deste MathBox.
    pub(super) items: Vec<FrameItem>,
}

impl MathBox {
    fn height(&self) -> f64 {
        self.ascent + self.descent
    }

    /// Converte para FrameItems com posição no frame pai.
    ///
    /// `x_origin`: deslocamento horizontal no frame pai.
    /// `baseline_y`: posição da baseline no frame pai (y cresce para baixo).
    ///
    /// Conversão: `parent_y = baseline_y - ascent + local_y`
    ///   - `local_y = 0` → topo do box (acima da baseline)
    ///   - `local_y = ascent` → na baseline
    fn place(self, x_origin: f64, baseline_y: f64) -> Vec<FrameItem> {
        self.items.into_iter().map(|mut item| {
            match item {
                FrameItem::Text { ref mut pos, .. } => {
                    pos.x = Pt(pos.x.val() + x_origin);
                    pos.y = Pt(baseline_y - self.ascent + pos.y.val());
                }
                FrameItem::Line { ref mut start, ref mut end, .. } => {
                    start.x = Pt(start.x.val() + x_origin);
                    end.x   = Pt(end.x.val() + x_origin);
                    start.y = Pt(baseline_y - self.ascent + start.y.val());
                    end.y   = Pt(baseline_y - self.ascent + end.y.val());
                }
                FrameItem::Glyph { ref mut pos, .. } => {
                    pos.x = Pt(pos.x.val() + x_origin);
                    pos.y = Pt(baseline_y - self.ascent + pos.y.val());
                }
                FrameItem::Image { .. } => {}   // imagens não ocorrem em contexto math
                FrameItem::Shape { .. } => {}   // formas não ocorrem em contexto math
                FrameItem::Group { .. } => {}   // grupos não ocorrem em contexto math
            }
            item
        }).collect()
    }
}

/// Desloca um `FrameItem` por `(dx, dy)`.
pub(super) fn offset_item(item: FrameItem, dx: Pt, dy: Pt) -> FrameItem {
    match item {
        FrameItem::Text { pos, text, style } => FrameItem::Text {
            pos: Point { x: Pt(pos.x.val() + dx.val()), y: Pt(pos.y.val() + dy.val()) },
            text,
            style,
        },
        FrameItem::Line { start, end, thickness } => FrameItem::Line {
            start: Point { x: Pt(start.x.val() + dx.val()), y: Pt(start.y.val() + dy.val()) },
            end:   Point { x: Pt(end.x.val()   + dx.val()), y: Pt(end.y.val()   + dy.val()) },
            thickness,
        },
        FrameItem::Glyph { pos, glyph_id, x_advance, size } => FrameItem::Glyph {
            pos: Point { x: Pt(pos.x.val() + dx.val()), y: Pt(pos.y.val() + dy.val()) },
            glyph_id,
            x_advance,
            size,
        },
        FrameItem::Image { pos, data, width, height, intrinsic_width, intrinsic_height } =>
            FrameItem::Image {
                pos: Point { x: Pt(pos.x.val() + dx.val()), y: Pt(pos.y.val() + dy.val()) },
                data,
                width,
                height,
                intrinsic_width,
                intrinsic_height,
            },
        FrameItem::Shape { pos, kind, width, height, fill, stroke } =>
            FrameItem::Shape {
                pos: Point { x: Pt(pos.x.val() + dx.val()), y: Pt(pos.y.val() + dy.val()) },
                kind,
                width,
                height,
                fill,
                stroke,
            },
        FrameItem::Group { pos, matrix, clip_mask, inner_width, inner_height, items } =>
            FrameItem::Group {
                pos: Point { x: Pt(pos.x.val() + dx.val()), y: Pt(pos.y.val() + dy.val()) },
                matrix,
                clip_mask,
                inner_width,
                inner_height,
                items,
            },
    }
}

/// Verifica se uma sequência de nós matemáticos precisa de layout em grelha.
///
/// Retorna `true` se houver pelo menos um `MathAlignPoint` ou `Linebreak`.
/// Se `false`, o layout linear existente é usado sem custo adicional.
pub(super) fn needs_grid_layout(nodes: &[Content]) -> bool {
    nodes.iter().any(|c| matches!(c, Content::MathAlignPoint | Content::Linebreak))
}

/// Particiona uma sequência flat em linhas e colunas.
///
/// Retorna `Vec<Vec<Vec<Content>>>`:
///   - dim 0: linhas (separadas por `Linebreak`)
///   - dim 1: colunas (separadas por `MathAlignPoint`)
///   - dim 2: items da célula
///
/// Células e linhas finais vazias são removidas.
pub(super) fn partition_grid(nodes: &[Content]) -> Vec<Vec<Vec<Content>>> {
    let mut lines: Vec<Vec<Vec<Content>>> = vec![vec![vec![]]];

    for node in nodes {
        match node {
            Content::Linebreak => {
                lines.push(vec![vec![]]);
            }
            Content::MathAlignPoint => {
                lines.last_mut().unwrap().push(vec![]);
            }
            other => {
                lines.last_mut().unwrap()
                     .last_mut().unwrap()
                     .push(other.clone());
            }
        }
    }

    // Remover células finais vazias em cada linha
    for line in &mut lines {
        while line.last().map(|c| c.is_empty()).unwrap_or(false) {
            line.pop();
        }
    }

    // Remover linhas finais completamente vazias
    while lines.last().map(|l| l.is_empty()).unwrap_or(false) {
        lines.pop();
    }

    lines
}

/// Motor de layout matemático — stateless.
///
/// Recebe `Content` matemático e produz um `Vec<FrameItem>` com posições
/// relativas à origem `(0, 0)`, prontas para integração no layouter principal.
///
/// **Passo 36**: `MathIdent` e `MathText` → `FrameItem::Text`.
/// **Passo 37**: `MathFrac` (numerador/denominador) e `MathAttach`
///   (sup/sub com posicionamento vertical) implementados via `MathBox`.
/// **Passo 41**: constantes OpenType MATH via `FontMetrics::math_constants()`.
/// Política de alinhamento horizontal das células numa grelha matemática.
pub(super) enum GridAlign {
    /// `MathAlignPoint` (`&`): colunas pares à direita, ímpares à esquerda.
    Alternating,
    /// `MathMatrix` (`mat`): todas as células centradas na sua coluna.
    Center,
    /// `MathCases` (`cases`): todas as colunas alinhadas à esquerda.
    Left,
}

pub struct MathLayouter<'a, M: FontMetrics> {
    pub(super) metrics:   &'a M,
    pub(super) constants: MathConstants,
    /// True se a equação é de bloco (display mode); false se inline.
    /// Controla se operadores grandes usam limites verticais (Passo 50).
    pub(super) block: bool,
}

impl<'a, M: FontMetrics> MathLayouter<'a, M> {
    pub fn new(metrics: &'a M, block: bool) -> Self {
        let constants = metrics.math_constants();
        Self { metrics, constants, block }
    }

    /// Centra um MathBox no eixo matemático ajustando ascent/descent.
    ///
    /// O eixo matemático é `axis_height` (design units) acima da baseline.
    /// Após este ajuste, o centro vertical do box fica no eixo.
    ///
    /// Aplica-se a fracções, delimitadores e raízes — não a elementos inline.
    pub(super) fn apply_axis_offset(&self, mut b: MathBox, size: Pt) -> MathBox {
        let axis_pt = self.constants.to_pt(self.constants.axis_height, size).val();
        let shift   = axis_pt - (b.ascent - b.descent) / 2.0;
        b.ascent  += shift;
        b.descent -= shift;
        b
    }

    /// Ponto de entrada: recebe o body de uma equação e produz `Vec<FrameItem>`.
    ///
    /// Os items retornados têm posições relativas à origem — o layouter principal
    /// é responsável por ajustar para posição absoluta na página.
    pub fn layout_equation(
        &self,
        body:  &Content,
        style: &TextStyle,
    ) -> Vec<FrameItem> {
        let math_box = self.layout_node(body, style);
        // Baseline no topo do box (simplificado: Passo 38+ alinhará com x-height)
        let baseline_y = math_box.ascent;
        math_box.place(0.0, baseline_y)
    }

    /// Percorre a árvore de Content matemático recursivamente, produzindo um `MathBox`.
    pub(super) fn layout_node(&self, content: &Content, style: &TextStyle) -> MathBox {
        match content {
            Content::MathIdent(name) => {
                // Variáveis de uma letra → itálico; funções conhecidas → não-itálico
                let is_var  = symbols::is_single_letter_var(name)
                              && symbols::ident_to_unicode(name).is_none();
                let is_func = symbols::is_math_function(name);
                let math_style = if is_var && !is_func {
                    TextStyle { italic: true, ..*style }
                } else {
                    TextStyle { italic: false, ..*style }
                };
                self.layout_text_node(name, &math_style)
            }
            Content::MathText(text) => {
                self.layout_text_node(text, style)
            }

            Content::MathSequence(nodes) => {
                self.layout_sequence(nodes, style)
            }

            Content::MathFrac { num, den } => {
                self.layout_frac(num, den, style)
            }

            Content::MathAttach { base, tl, bl, sub, sup } => {
                self.layout_attach(
                    base,
                    tl.as_deref(), bl.as_deref(),
                    sub.as_deref(), sup.as_deref(),
                    style,
                )
            }

            Content::MathRoot { index, radicand } => {
                self.layout_root(index.as_deref(), radicand, style)
            }

            Content::MathDelimited { open, body, close } => {
                self.layout_delimited(*open, body, *close, style)
            }

            Content::MathMatrix { rows, delim } => {
                self.layout_matrix(rows, *delim, style)
            }

            Content::MathCases { rows } => {
                self.layout_cases(rows, style)
            }

            other => {
                let text: EcoString = other.plain_text().into();
                if text.trim().is_empty() {
                    MathBox { width: 0.0, ascent: 0.0, descent: 0.0, items: vec![] }
                } else {
                    self.layout_text_node(&text, style)
                }
            }
        }
    }

    /// Nó folha: texto com métricas tipográficas.
    ///
    /// Posição do item dentro do MathBox: `(0, 0)` — relativo ao topo esquerdo.
    pub(super) fn layout_text_node(&self, text: &EcoString, style: &TextStyle) -> MathBox {
        if text.is_empty() {
            return MathBox { width: 0.0, ascent: 0.0, descent: 0.0, items: vec![] };
        }
        let width  = self.metrics.advance(text, style.size).val();
        let vm     = self.metrics.vertical_metrics(style.size);
        let ascent  = vm.0.val();
        let descent = (vm.1 - vm.0).val();
        MathBox {
            width,
            ascent,
            descent,
            items: vec![FrameItem::Text {
                pos:   Point { x: Pt(0.0), y: Pt(0.0) },
                text:  text.clone(),
                style: *style,
            }],
        }
    }

    pub(super) fn layout_sequence(&self, nodes: &[Content], style: &TextStyle) -> MathBox {
        if self.block && needs_grid_layout(nodes) {
            self.layout_grid(nodes, style)
        } else {
            let boxes: Vec<MathBox> = nodes.iter()
                .filter(|n| !matches!(n, Content::MathAlignPoint | Content::Linebreak))
                .map(|n| self.layout_node(n, style))
                .collect();
            self.hconcat(boxes)
        }
    }

    /// Grelha 2D generalizada — usada por `layout_grid` e `MathMatrix`.
    ///
    /// Duas passagens:
    ///   1. Mede todas as células → largura máxima por coluna.
    ///   2. Posiciona com essas larguras + `column_gap` entre colunas.
    pub(super) fn layout_grid_rows(
        &self,
        rows:       &[Vec<Content>],
        align:      GridAlign,
        column_gap: Pt,
        style:      &TextStyle,
    ) -> MathBox {
        let n_cols = rows.iter().map(|row| row.len()).max().unwrap_or(0);
        if n_cols == 0 {
            return MathBox { width: 0.0, ascent: 0.0, descent: 0.0, items: vec![] };
        }

        // ── Passagem 1: medir todas as células ────────────────────────────
        let grid_boxes: Vec<Vec<MathBox>> = rows.iter()
            .map(|row| row.iter().map(|cell| self.layout_node(cell, style)).collect())
            .collect();

        let mut col_widths = vec![0.0_f64; n_cols];
        for row in &grid_boxes {
            for (col_idx, cell_box) in row.iter().enumerate() {
                col_widths[col_idx] = col_widths[col_idx].max(cell_box.width);
            }
        }

        // ── Passagem 2: posicionar células ────────────────────────────────
        let mut all_items: Vec<FrameItem> = Vec::new();
        let mut baseline_offset = 0.0_f64;
        let gap = column_gap.val();
        let n_gaps = n_cols.saturating_sub(1) as f64;
        let total_width: f64 = col_widths.iter().sum::<f64>() + n_gaps * gap;

        let total_ascent = grid_boxes.first()
            .map(|row| row.iter().map(|b| b.ascent).fold(0.0, f64::max))
            .unwrap_or(0.0);
        let mut total_descent = grid_boxes.first()
            .map(|row| row.iter().map(|b| b.descent).fold(0.0, f64::max))
            .unwrap_or(0.0);

        for (row_idx, row) in grid_boxes.iter().enumerate() {
            let row_ascent  = row.iter().map(|b| b.ascent).fold(0.0, f64::max);
            let row_descent = row.iter().map(|b| b.descent).fold(0.0, f64::max);

            let mut cursor_x = 0.0_f64;
            for (col_idx, cell_box) in row.iter().enumerate() {
                let col_w = if col_idx < n_cols { col_widths[col_idx] } else { 0.0 };

                let cell_x = match align {
                    GridAlign::Alternating => if col_idx % 2 == 0 {
                        cursor_x + (col_w - cell_box.width)   // par: à direita
                    } else {
                        cursor_x                               // ímpar: à esquerda
                    },
                    GridAlign::Center => cursor_x + (col_w - cell_box.width) / 2.0,
                    GridAlign::Left   => cursor_x,
                };

                let dy = baseline_offset - row_ascent;
                for item in cell_box.items.clone() {
                    all_items.push(offset_item(item, Pt(cell_x), Pt(dy)));
                }

                cursor_x += col_w;
                if col_idx + 1 < n_cols { cursor_x += gap; }
            }

            if row_idx + 1 < grid_boxes.len() {
                let line_gap = self.constants.to_pt(self.constants.math_leading, style.size).val();
                let advance = row_descent + line_gap + {
                    let next_row = &grid_boxes[row_idx + 1];
                    next_row.iter().map(|b| b.ascent).fold(0.0, f64::max)
                };
                baseline_offset += advance;
                total_descent += row_descent + line_gap
                    + grid_boxes[row_idx + 1].iter().map(|b| b.ascent + b.descent).fold(0.0, f64::max);
            }
        }

        MathBox {
            width:   total_width,
            ascent:  total_ascent,
            descent: total_descent,
            items:   all_items,
        }
    }

    /// Layout em grelha 2D para equações com `&` e `\\`.
    ///
    /// Chama `layout_grid_rows` com alinhamento alternado (colunas pares à
    /// direita, ímpares à esquerda) e sem espaço entre colunas.
    pub(super) fn layout_grid(&self, nodes: &[Content], style: &TextStyle) -> MathBox {
        let grid = partition_grid(nodes);
        // Cada célula é Vec<Content> — envolver em MathSequence para layout_node.
        let rows: Vec<Vec<Content>> = grid.into_iter()
            .map(|row| row.into_iter()
                .map(|cell_nodes| Content::MathSequence(cell_nodes.into()))
                .collect())
            .collect();
        self.layout_grid_rows(&rows, GridAlign::Alternating, Pt(0.0), style)
    }

    /// Concatenação horizontal: posiciona MathBoxes lado a lado.
    pub(super) fn hconcat(&self, boxes: Vec<MathBox>) -> MathBox {
        let mut x       = 0.0_f64;
        let mut ascent  = 0.0_f64;
        let mut descent = 0.0_f64;
        let mut items   = Vec::new();

        for b in boxes {
            ascent  = ascent.max(b.ascent);
            descent = descent.max(b.descent);
            for mut item in b.items {
                match item {
                    FrameItem::Text { ref mut pos, .. } => {
                        pos.x = Pt(pos.x.val() + x);
                    }
                    FrameItem::Line { ref mut start, ref mut end, .. } => {
                        start.x = Pt(start.x.val() + x);
                        end.x   = Pt(end.x.val() + x);
                    }
                    FrameItem::Glyph { ref mut pos, .. } => {
                        pos.x = Pt(pos.x.val() + x);
                    }
                    FrameItem::Image { .. } => {}   // imagens não ocorrem em contexto math
                    FrameItem::Shape { .. } => {}   // formas não ocorrem em contexto math
                    FrameItem::Group { .. } => {}   // grupos não ocorrem em contexto math
                }
                items.push(item);
            }
            x += b.width;
        }

        MathBox { width: x, ascent, descent, items }
    }
}

#[cfg(test)]
mod tests;
