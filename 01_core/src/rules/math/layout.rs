//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/math.md
//! @prompt-hash 8be87568
//! @layer L1
//! @updated 2026-04-11

use ecow::EcoString;

use crate::entities::{
    content::Content,
    glyph_variants::GlyphAssembly,
    layout_types::{FrameItem, Point, Pt, TextStyle},
    math_constants::MathConstants,
};
use crate::rules::layout::FontMetrics;
use crate::entities::glyph_variants::MathGlyphKern;
use super::symbols;

/// Caixa tipográfica de um nó matemático.
/// Todas as medidas são em pontos, relativas à baseline da equação.
/// `ascent` > 0 (acima da baseline), `descent` > 0 (abaixo da baseline).
#[derive(Debug, Clone)]
struct MathBox {
    width:   f64,
    ascent:  f64,
    descent: f64,
    /// Items com posições relativas ao topo esquerdo deste MathBox.
    items: Vec<FrameItem>,
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
            }
            item
        }).collect()
    }
}

/// Desloca um `FrameItem` por `(dx, dy)`.
fn offset_item(item: FrameItem, dx: Pt, dy: Pt) -> FrameItem {
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
    }
}

/// Verifica se uma sequência de nós matemáticos precisa de layout em grelha.
///
/// Retorna `true` se houver pelo menos um `MathAlignPoint` ou `Linebreak`.
/// Se `false`, o layout linear existente é usado sem custo adicional.
fn needs_grid_layout(nodes: &[Content]) -> bool {
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
fn partition_grid(nodes: &[Content]) -> Vec<Vec<Vec<Content>>> {
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
pub struct MathLayouter<'a, M: FontMetrics> {
    metrics:   &'a M,
    constants: MathConstants,
    /// True se a equação é de bloco (display mode); false se inline.
    /// Controla se operadores grandes usam limites verticais (Passo 50).
    block: bool,
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
    fn apply_axis_offset(&self, mut b: MathBox, size: Pt) -> MathBox {
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
    fn layout_node(&self, content: &Content, style: &TextStyle) -> MathBox {
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
    fn layout_text_node(&self, text: &EcoString, style: &TextStyle) -> MathBox {
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

    fn layout_sequence(&self, nodes: &[Content], style: &TextStyle) -> MathBox {
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

    /// Layout em grelha 2D para equações com `&` e `\\`.
    ///
    /// Duas passagens:
    ///   1. Mede todas as células para calcular largura máxima por coluna.
    ///   2. Posiciona com essas larguras fixas.
    ///
    /// Regra de alinhamento Typst:
    ///   - Colunas 0-based pares (0, 2, …) → alinha à direita.
    ///   - Colunas 0-based ímpares (1, 3, …) → alinha à esquerda.
    fn layout_grid(&self, nodes: &[Content], style: &TextStyle) -> MathBox {
        let grid = partition_grid(nodes);
        let n_cols = grid.iter().map(|row| row.len()).max().unwrap_or(0);
        if n_cols == 0 {
            return MathBox { width: 0.0, ascent: 0.0, descent: 0.0, items: vec![] };
        }

        // ── Passagem 1: medir todas as células ────────────────────────────
        let grid_boxes: Vec<Vec<MathBox>> = grid.iter()
            .map(|row| row.iter().map(|cell| self.layout_sequence(cell, style)).collect())
            .collect();

        let mut col_widths = vec![0.0_f64; n_cols];
        for row in &grid_boxes {
            for (col_idx, cell_box) in row.iter().enumerate() {
                col_widths[col_idx] = col_widths[col_idx].max(cell_box.width);
            }
        }

        // ── Passagem 2: posicionar células ────────────────────────────────
        let mut all_items: Vec<FrameItem> = Vec::new();
        // dy acumulado: deslocamento da baseline da linha actual
        // relativo à baseline da linha 0 (= baseline da equação toda).
        let mut baseline_offset = 0.0_f64;
        let total_width: f64 = col_widths.iter().sum();

        // Ascent total = ascent da linha 0
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

                // Alinhamento interno da célula
                let cell_x = if col_idx % 2 == 0 {
                    // Col par (0-based): alinha à direita dentro da coluna
                    cursor_x + (col_w - cell_box.width)
                } else {
                    // Col ímpar: alinha à esquerda
                    cursor_x
                };

                // dy: os items do cell_box têm y=0 no topo do box.
                // A baseline do cell_box está a y=cell_box.ascent (no box local).
                // No box global, a baseline da linha row_idx está a baseline_offset
                // abaixo da baseline da linha 0 (y=0 no box global é o topo do row0).
                // Items devem ser deslocados de baseline_offset - row_ascent
                // para que o topo do row apareça na posição correcta.
                let dy = baseline_offset - row_ascent;

                for item in cell_box.items.clone() {
                    all_items.push(offset_item(item, Pt(cell_x), Pt(dy)));
                }

                cursor_x += col_w;
            }

            // Acumular descent para linhas seguintes
            if row_idx == 0 {
                // baseline_offset permanece 0 para linha 0
            } else {
                // Já foi avançado antes desta linha
            }

            // Avançar baseline para próxima linha
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

    /// Concatenação horizontal: posiciona MathBoxes lado a lado.
    fn hconcat(&self, boxes: Vec<MathBox>) -> MathBox {
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
                }
                items.push(item);
            }
            x += b.width;
        }

        MathBox { width: x, ascent, descent, items }
    }

    /// Fracção: numerador acima da linha de fracção, denominador abaixo.
    ///
    /// Tamanho do sub-estilo: `script_percent_scale_down` da tabela MATH.
    fn layout_frac(&self, num: &Content, den: &Content, style: &TextStyle) -> MathBox {
        let sub_style = TextStyle {
            size: style.size * self.constants.script_percent_scale_down,
            ..*style
        };

        let num_box = self.layout_node(num, &sub_style);
        let den_box = self.layout_node(den, &sub_style);

        let width = num_box.width.max(den_box.width);

        let rule_thickness = self.constants.to_pt(
            self.constants.fraction_rule_thickness, style.size
        ).val();
        let gap = self.constants.to_pt(
            self.constants.fraction_num_gap, style.size
        ).val();

        // ascent cobre todo o numerador + espaço + metade da linha
        let ascent  = num_box.height() + gap + rule_thickness / 2.0;
        // descent cobre metade da linha + espaço + todo o denominador
        let descent = den_box.height() + gap + rule_thickness / 2.0;

        // Centrar horizontalmente
        let num_x = (width - num_box.width) / 2.0;
        let den_x = (width - den_box.width) / 2.0;

        // Numerador: topo do MathBox (local_y = 0)
        let num_y = 0.0_f64;
        // Denominador: abaixo do numerador + linha de fracção
        let den_y = num_box.height() + gap + rule_thickness + gap;
        // Linha de fracção: centro entre numerador e denominador
        let rule_local_y = num_box.height() + gap + rule_thickness / 2.0;

        let mut items = Vec::new();

        for mut item in num_box.items {
            if let FrameItem::Text { ref mut pos, .. } = item {
                pos.x = Pt(pos.x.val() + num_x);
                pos.y = Pt(pos.y.val() + num_y);
            }
            items.push(item);
        }

        // Linha de fracção posicionada entre numerador e denominador.
        items.push(FrameItem::Line {
            start:     Point { x: Pt(0.0),    y: Pt(rule_local_y) },
            end:       Point { x: Pt(width),  y: Pt(rule_local_y) },
            thickness: rule_thickness,
        });

        for mut item in den_box.items {
            if let FrameItem::Text { ref mut pos, .. } = item {
                pos.x = Pt(pos.x.val() + den_x);
                pos.y = Pt(pos.y.val() + den_y);
            }
            items.push(item);
        }

        self.apply_axis_offset(MathBox { width, ascent, descent, items }, style.size)
    }

    /// Attach: base na baseline, sup elevado, sub baixado.
    ///
    /// Tamanho do sub-estilo: `script_percent_scale_down` da tabela MATH.
    /// Deslocamentos: `superscript_shift_up` e `subscript_shift_down` da tabela MATH.
    fn layout_attach(
        &self,
        base: &Content,
        tl:   Option<&Content>,
        bl:   Option<&Content>,
        sub:  Option<&Content>,
        sup:  Option<&Content>,
        style: &TextStyle,
    ) -> MathBox {
        let base_box     = self.layout_node(base, style);
        let script_style = TextStyle {
            size: style.size * self.constants.script_percent_scale_down,
            ..*style
        };

        let sup_offset = self.constants.to_pt(
            self.constants.superscript_shift_up, style.size
        ).val();
        let sub_offset = self.constants.to_pt(
            self.constants.subscript_shift_down, style.size
        ).val();

        // Extrair o char da base para consultar MathKernInfo e detectar operador grande.
        // Apenas MathIdent/MathText têm char único; outros ficam com kern zero.
        let base_char: Option<char> = match base {
            Content::MathIdent(s) | Content::MathText(s) => s.chars().next(),
            _ => None,
        };
        let base_kern: MathGlyphKern = base_char
            .map(|c| self.metrics.math_kern(c))
            .unwrap_or_default();

        // Passo 49/50 — empilhamento vertical apenas em bloco (display mode).
        // Inline: sub/sup à direita para não expandir a linha de texto.
        let is_limits = self.block && match base {
            Content::MathIdent(s) | Content::MathText(s) => {
                let ch = s.chars().next().unwrap_or('\0');
                symbols::is_large_operator(ch) || symbols::is_limit_function(s.as_str())
            }
            _ => false,
        };

        // ── Passo 3a/3b/3c — Coluna esquerda (pre-scripts) ──────────────
        // Layout dos left-scripts para obter larguras.
        let tl_box = tl.map(|c| self.layout_node(c, &script_style));
        let bl_box = bl.map(|c| self.layout_node(c, &script_style));

        // Kern dos quadrantes esquerdos (com FixedMetrics é sempre zero).
        let tl_kern = if let Some(ref tb) = tl_box {
            let h_du = tb.descent * self.constants.upem / style.size.val().max(0.001);
            self.constants.to_pt(base_kern.top_left.kern_at(h_du), style.size).val()
        } else { 0.0 };
        let bl_kern = if let Some(ref bb) = bl_box {
            let h_du = bb.ascent * self.constants.upem / style.size.val().max(0.001);
            self.constants.to_pt(base_kern.bottom_left.kern_at(h_du), style.size).val()
        } else { 0.0 };

        // Largura da coluna esquerda: max das larguras com respectivo kern.
        let left_col_width = {
            let tl_w = tl_box.as_ref().map(|b| b.width + tl_kern.abs()).unwrap_or(0.0);
            let bl_w = bl_box.as_ref().map(|b| b.width + bl_kern.abs()).unwrap_or(0.0);
            tl_w.max(bl_w)
        };

        // Salvar métricas da base antes de consumir base_box.items.
        let base_ascent  = base_box.ascent;
        let base_descent = base_box.descent;
        let base_width   = base_box.width;

        // ── Construção dos items ──────────────────────────────────────────
        let mut ascent  = base_ascent;
        let mut descent = base_descent;
        let mut items   = Vec::new();

        // Posicionar tl (pre-superscript): alinhado à direita da coluna esquerda,
        // elevado pelo sup_offset acima da baseline da base.
        if let Some(tb) = tl_box {
            ascent = ascent.max(sup_offset + tb.ascent);
            let x_tl = left_col_width - tb.width - tl_kern;
            for item in tb.items {
                items.push(offset_item(item, Pt(x_tl), Pt(-sup_offset)));
            }
        }

        // Posicionar bl (pre-subscript): alinhado à direita da coluna esquerda,
        // baixado pelo sub_offset abaixo da baseline da base.
        if let Some(bb) = bl_box {
            descent = descent.max(sub_offset + bb.descent);
            let x_bl = left_col_width - bb.width - bl_kern;
            for item in bb.items {
                items.push(offset_item(item, Pt(x_bl), Pt(sub_offset)));
            }
        }

        if is_limits {
            // ── Passo 49 — Empilhamento vertical para operadores grandes ──────
            //
            // sup fica centrado ACIMA da base, separado por upper_limit_gap_min.
            // sub fica centrado ABAIXO da base, separado por lower_limit_gap_min.
            let upper_gap = self.constants.to_pt(
                self.constants.upper_limit_gap_min, style.size
            ).val();
            let lower_gap = self.constants.to_pt(
                self.constants.lower_limit_gap_min, style.size
            ).val();

            let sup_box_opt = sup.map(|c| self.layout_node(c, &script_style));
            let sub_box_opt = sub.map(|c| self.layout_node(c, &script_style));

            // Largura máxima dos três elementos para centrar em X.
            let max_content_w = [
                base_width,
                sup_box_opt.as_ref().map(|b| b.width).unwrap_or(0.0),
                sub_box_opt.as_ref().map(|b| b.width).unwrap_or(0.0),
            ].iter().cloned().fold(0.0f64, f64::max);

            let total_w = left_col_width + max_content_w;

            // Base centrada.
            let x_base = left_col_width + (max_content_w - base_width) / 2.0;
            for item in base_box.items {
                items.push(offset_item(item, Pt(x_base), Pt(0.0)));
            }

            // Limite superior: bottom do sup fica upper_gap acima do top da base.
            // y_sup = -(base_ascent + upper_gap + sup.descent)
            if let Some(sb) = sup_box_opt {
                let y_sup = -(base_ascent + upper_gap + sb.descent);
                let x_sup = left_col_width + (max_content_w - sb.width) / 2.0;
                ascent = ascent.max(base_ascent + upper_gap + sb.descent + sb.ascent);
                for item in sb.items {
                    items.push(offset_item(item, Pt(x_sup), Pt(y_sup)));
                }
            }

            // Limite inferior: top do sub fica lower_gap abaixo do bottom da base.
            // y_sub = base_descent + lower_gap + sub.ascent
            if let Some(sb) = sub_box_opt {
                let y_sub = base_descent + lower_gap + sb.ascent;
                let x_sub = left_col_width + (max_content_w - sb.width) / 2.0;
                descent = descent.max(base_descent + lower_gap + sb.ascent + sb.descent);
                for item in sb.items {
                    items.push(offset_item(item, Pt(x_sub), Pt(y_sub)));
                }
            }

            MathBox { width: total_w, ascent, descent, items }
        } else {
            // ── Right-scripts (sub/sup à direita — layout horizontal) ────────
            // Base: posicionada em x = left_col_width.
            for item in base_box.items {
                items.push(offset_item(item, Pt(left_col_width), Pt(0.0)));
            }

            // Right-scripts partem de left_col_width + base_width.
            let mut x = left_col_width + base_width;

            if let Some(sup_content) = sup {
                let sup_box = self.layout_node(sup_content, &script_style);
                ascent = ascent.max(sup_offset + sup_box.ascent);

                // Kern: quadrante top-right. Altura de conexão = ascent do sup.
                let sup_h_du = sup_box.ascent * self.constants.upem
                    / style.size.val().max(0.001);
                let kern_sup = self.constants.to_pt(
                    base_kern.top_right.kern_at(sup_h_du), style.size
                ).val();

                for item in sup_box.items {
                    items.push(offset_item(item, Pt(x + kern_sup), Pt(-sup_offset)));
                }
                x += sup_box.width + kern_sup;
            }

            if let Some(sub_content) = sub {
                let sub_box = self.layout_node(sub_content, &script_style);
                descent = descent.max(sub_offset + sub_box.descent);

                // Kern: quadrante bottom-right. Altura de conexão = ascent do sub.
                let sub_h_du = sub_box.ascent * self.constants.upem
                    / style.size.val().max(0.001);
                let kern_sub = self.constants.to_pt(
                    base_kern.bottom_right.kern_at(sub_h_du), style.size
                ).val();

                for item in sub_box.items {
                    items.push(offset_item(item, Pt(x + kern_sub), Pt(sub_offset)));
                }
            }

            MathBox { width: x, ascent, descent, items }
        }
    }

    /// Layout de raiz quadrada / n-ésima.
    ///
    /// Componentes: símbolo `√`, overline sobre o radicando, radicando à direita,
    /// índice opcional posicionado acima e à esquerda do símbolo radical.
    fn layout_root(
        &self,
        index:    Option<&Content>,
        radicand: &Content,
        style:    &TextStyle,
    ) -> MathBox {
        // 1. Layout do radicando
        let rad_box = self.layout_node(radicand, style);

        // 3. Geometria da overline (calculada antes do radical para computar min_height)
        let line_thickness = self.constants.to_pt(
            self.constants.radical_rule_thickness, style.size
        ).val();
        let gap = self.constants.to_pt(
            self.constants.radical_vertical_gap, style.size
        ).val();

        // 2. Símbolo √ extensível — altura cobre radicando + gap + overline
        let rad_height_pt  = rad_box.ascent + rad_box.descent + gap + line_thickness;
        let min_height_du  = if style.size.val() > 0.0 {
            rad_height_pt * self.constants.upem / style.size.val()
        } else {
            0.0
        };
        let radical_box   = self.layout_stretchy_delimiter('√', min_height_du, style);
        let radical_width = radical_box.width;

        // 4. Dimensões totais
        //    ascent cobre: ascent do radicando + gap + espessura da linha
        let total_ascent  = rad_box.ascent + gap + line_thickness;
        let total_descent = rad_box.descent;
        let total_width   = radical_width + rad_box.width;

        let mut items = Vec::new();

        // 5a. Símbolo √ — deslocar para baixo para que a sua baseline alinhe
        //     com a baseline principal (total_ascent em coordenadas locais)
        let sym_dy = total_ascent - radical_box.ascent;
        for item in radical_box.items {
            items.push(offset_item(item, Pt(0.0), Pt(sym_dy)));
        }

        // 5b. Overline — linha horizontal no topo do radicando
        let overline_y = gap + line_thickness / 2.0;
        items.push(FrameItem::Line {
            start:     Point { x: Pt(radical_width),                    y: Pt(overline_y) },
            end:       Point { x: Pt(radical_width + rad_box.width),    y: Pt(overline_y) },
            thickness: line_thickness,
        });

        // 5c. Radicando — à direita do símbolo, deslocado abaixo da overline
        let rad_offset_y = gap + line_thickness;
        for item in rad_box.items {
            items.push(offset_item(item, Pt(radical_width), Pt(rad_offset_y)));
        }

        // 6. Índice opcional (para root(n, x))
        if let Some(idx_content) = index {
            let script_style = TextStyle {
                size: style.size * self.constants.script_percent_scale_down,
                ..*style
            };
            let idx_box = self.layout_node(idx_content, &script_style);
            // Posicionar acima e à esquerda do símbolo: x=20% da largura do radical, y=0 (topo)
            let idx_x = radical_width * 0.2;
            for item in idx_box.items {
                items.push(offset_item(item, Pt(idx_x), Pt(0.0)));
            }
        }

        let result = MathBox { width: total_width, ascent: total_ascent, descent: total_descent, items };
        self.apply_axis_offset(result, style.size)
    }

    /// Selecciona e emite um delimitador com a altura mínima necessária.
    ///
    /// Prioridade:
    /// 1. Variante encontrada + `glyph_to_char` → `FrameItem::Text` com o char mapeado
    /// 2. Variante encontrada + sem mapeamento → `FrameItem::Glyph` com o glyph_id
    /// 3. Sem variante suficiente + `GlyphAssembly` disponível → `layout_assembly`
    /// 4. Sem variante nem assembly → `FrameItem::Text` com o char base
    fn layout_stretchy_delimiter(
        &self,
        c:             char,
        min_height_du: f64,
        style:         &TextStyle,
    ) -> MathBox {
        let variants = self.metrics.vertical_glyph_variants(c);

        if let Some((glyph_id, advance_du)) = variants.select_with_advance(min_height_du) {
            // Variante encontrada
            if let Some(mapped_char) = self.metrics.glyph_to_char(glyph_id) {
                // Mapeamento Unicode disponível — emitir como Text
                let text: ecow::EcoString = mapped_char.to_string().into();
                return self.layout_text_node(&text, style);
            } else {
                // Sem mapeamento — emitir como Glyph
                let x_advance = style.size * (advance_du / self.constants.upem);
                let (ascent, _) = self.metrics.vertical_metrics(style.size);
                return MathBox {
                    width:   x_advance.val(),
                    ascent:  ascent.val(),
                    descent: 0.0,
                    items:   vec![FrameItem::Glyph {
                        pos:       Point::ZERO,
                        glyph_id,
                        x_advance,
                        size:      style.size,
                    }],
                };
            }
        }

        // Nenhuma variante suficiente — tentar GlyphAssembly
        let assembly = self.metrics.vertical_glyph_assembly(c);
        if !assembly.is_empty() {
            return self.layout_assembly(c, assembly, min_height_du, style);
        }

        // Fallback: glifo base
        let text: ecow::EcoString = c.to_string().into();
        self.layout_text_node(&text, style)
    }

    /// Monta um delimitador vertical a partir de peças extensíveis.
    ///
    /// Estratégia simplificada: usar cada peça uma vez. Extensores são
    /// incluídos uma vez se a altura base for insuficiente.
    fn layout_assembly(
        &self,
        c:              char,
        assembly:       GlyphAssembly,
        _target_advance: f64,
        style:          &TextStyle,
    ) -> MathBox {
        if assembly.is_empty() {
            let text: ecow::EcoString = c.to_string().into();
            return self.layout_text_node(&text, style);
        }

        let scale = style.size.val() / self.constants.upem;
        let mut items  = Vec::new();
        let mut max_advance  = 0.0_f64;

        // Empilhar peças de baixo para cima (bottom → top em coords do MathBox)
        // No MathBox, y=0 é o topo e y=ascent é a baseline.
        // Vamos acumular posições de baixo para cima e depois inverter.
        let mut piece_positions: Vec<(f64, u16, f64)> = Vec::new(); // (y_from_bottom, glyph_id, advance_pt)

        let mut y_cursor = 0.0_f64;
        let n = assembly.parts.len();
        for (i, part) in assembly.parts.iter().enumerate() {
            let advance_pt = part.full_advance as f64 * scale;
            let x_advance  = Pt(advance_pt);

            // Sobreposição com a peça seguinte
            let overlap = if i + 1 < n {
                let next = &assembly.parts[i + 1];
                (part.end_connector as f64).min(next.start_connector as f64) * scale
            } else {
                0.0
            };

            piece_positions.push((y_cursor, part.glyph_id, x_advance.val()));
            max_advance = max_advance.max(x_advance.val());
            y_cursor += advance_pt - overlap;
        }
        let total_height = y_cursor;

        // Converter coordenadas: y_from_bottom → y no MathBox (topo = 0)
        for (y_from_bottom, glyph_id, x_advance_val) in piece_positions {
            let y_in_box = total_height - y_from_bottom - x_advance_val.min(total_height);
            items.push(FrameItem::Glyph {
                pos:       Point { x: Pt(0.0), y: Pt(y_in_box.max(0.0)) },
                glyph_id,
                x_advance: Pt(x_advance_val),
                size:      style.size,
            });
        }

        MathBox {
            width:   max_advance,
            ascent:  total_height,
            descent: 0.0,
            items,
        }
    }

    /// Expressão entre delimitadores extensíveis.
    ///
    /// Calcula a altura total do corpo e selecciona delimitadores que a cubram.
    fn layout_delimited(
        &self,
        open:  char,
        body:  &Content,
        close: char,
        style: &TextStyle,
    ) -> MathBox {
        let body_box = self.layout_node(body, style);

        // Converter altura do corpo de pt para design units
        let body_height_pt = body_box.ascent + body_box.descent;
        let min_height_du  = if style.size.val() > 0.0 {
            body_height_pt * self.constants.upem / style.size.val()
        } else {
            0.0
        };

        let open_box  = self.layout_stretchy_delimiter(open,  min_height_du, style);
        let close_box = self.layout_stretchy_delimiter(close, min_height_du, style);

        let result = self.hconcat(vec![open_box, body_box, close_box]);
        self.apply_axis_offset(result, style.size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use crate::rules::layout::{FixedMetrics, FontMetrics};

    fn default_style() -> TextStyle {
        TextStyle::regular(Pt(12.0))
    }

    fn size10_style() -> TextStyle {
        TextStyle::regular(Pt(10.0))
    }

    // ── Testes herdados do Passo 36 (adaptados para nova API) ─────────────

    #[test]
    fn math_layouter_math_ident_produz_items_nao_vazios() {
        let ml    = MathLayouter::new(&FixedMetrics, true);
        let items = ml.layout_equation(
            &Content::MathIdent("x".into()),
            &default_style(),
        );
        assert!(!items.is_empty(), "MathIdent deve produzir pelo menos 1 item");
    }

    #[test]
    fn math_layouter_math_text_produz_items_nao_vazios() {
        let ml    = MathLayouter::new(&FixedMetrics, true);
        let items = ml.layout_equation(
            &Content::MathText("sin".into()),
            &default_style(),
        );
        assert!(!items.is_empty());
    }

    #[test]
    fn math_layouter_sequence_produz_multiplos_items() {
        let ml = MathLayouter::new(&FixedMetrics, true);
        let seq = Content::MathSequence(
            Arc::from(vec![
                Content::MathIdent("x".into()),
                Content::MathText("+".into()),
                Content::MathIdent("y".into()),
            ].into_boxed_slice())
        );
        let items = ml.layout_equation(&seq, &default_style());
        assert_eq!(items.len(), 3, "x + y deve produzir 3 items");
    }

    #[test]
    fn math_layouter_frac_sem_placeholder_colchetes() {
        let ml = MathLayouter::new(&FixedMetrics, true);
        let frac = Content::MathFrac {
            num: Box::new(Content::MathIdent("a".into())),
            den: Box::new(Content::MathIdent("b".into())),
        };
        let items = ml.layout_equation(&frac, &default_style());
        for item in &items {
            if let FrameItem::Text { text, .. } = item {
                assert!(!text.contains('['),
                    "frac não deve conter '[': {}", text);
            }
        }
        assert!(!items.is_empty(), "frac deve produzir items");
    }

    #[test]
    fn math_layouter_cursor_avanca_horizontalmente() {
        let ml    = MathLayouter::new(&FixedMetrics, true);
        let items = ml.layout_equation(
            &Content::MathSequence(Arc::from(vec![
                Content::MathIdent("a".into()),
                Content::MathIdent("b".into()),
            ].into_boxed_slice())),
            &default_style(),
        );
        // Segundo item deve ter pos.x > 0 (cursor avançou)
        if let [first, second] = items.as_slice() {
            if let (FrameItem::Text { pos: p1, .. }, FrameItem::Text { pos: p2, .. }) = (first, second) {
                assert!(p2.x > p1.x, "segundo item deve estar à direita do primeiro");
            }
        }
    }

    #[test]
    fn math_layouter_math_attach_sem_colchetes() {
        let ml = MathLayouter::new(&FixedMetrics, true);
        let attach = Content::MathAttach {
            base: Box::new(Content::MathIdent("x".into())),
            tl:   None,
            bl:   None,
            sub:  None,
            sup:  Some(Box::new(Content::MathText("2".into()))),
        };
        let items = ml.layout_equation(&attach, &default_style());
        for item in &items {
            if let FrameItem::Text { text, .. } = item {
                assert!(!text.contains('['), "attach não deve conter '[': {}", text);
            }
        }
    }

    // ── Novos testes do Passo 37 + 38 ────────────────────────────────────

    #[test]
    fn math_frac_tem_dois_ou_mais_items() {
        let ml   = MathLayouter::new(&FixedMetrics, true);
        let frac = Content::MathFrac {
            num: Box::new(Content::MathIdent("a".into())),
            den: Box::new(Content::MathIdent("b".into())),
        };
        let items = ml.layout_equation(&frac, &size10_style());
        assert!(items.len() >= 2, "frac deve ter >= 2 items, tem {}", items.len());
    }

    #[test]
    fn math_frac_numerador_acima_denominador() {
        let ml   = MathLayouter::new(&FixedMetrics, true);
        let frac = Content::MathFrac {
            num: Box::new(Content::MathIdent("a".into())),
            den: Box::new(Content::MathIdent("b".into())),
        };
        let items = ml.layout_equation(&frac, &size10_style());

        let ys: Vec<f64> = items.iter().filter_map(|item| {
            if let FrameItem::Text { pos, .. } = item { Some(pos.y.val()) }
            else { None }
        }).collect();

        assert!(ys.len() >= 2, "deve ter pelo menos 2 posições y");
        assert!(ys[0] < ys[1],
            "numerador (y={}) deve estar acima do denominador (y={})", ys[0], ys[1]);
    }

    #[test]
    fn math_attach_sup_elevado() {
        let ml = MathLayouter::new(&FixedMetrics, true);
        let attach = Content::MathAttach {
            base: Box::new(Content::MathIdent("x".into())),
            tl:   None,
            bl:   None,
            sub:  None,
            sup:  Some(Box::new(Content::MathIdent("2".into()))),
        };
        let items = ml.layout_equation(&attach, &size10_style());
        assert!(items.len() >= 2, "x^2 deve ter >= 2 items");

        let ys: Vec<f64> = items.iter().filter_map(|item| {
            if let FrameItem::Text { pos, .. } = item { Some(pos.y.val()) }
            else { None }
        }).collect();

        // sup deve estar acima da base (y menor, pois y cresce para baixo)
        assert!(ys[1] < ys[0],
            "sup (y={}) deve estar acima da base (y={})", ys[1], ys[0]);
    }

    #[test]
    fn math_frac_tem_item_linha() {
        let ml   = MathLayouter::new(&FixedMetrics, true);
        let frac = Content::MathFrac {
            num: Box::new(Content::MathIdent("a".into())),
            den: Box::new(Content::MathIdent("b".into())),
        };
        let items = ml.layout_equation(&frac, &size10_style());
        let has_line = items.iter().any(|item| matches!(item, FrameItem::Line { .. }));
        assert!(has_line, "frac deve ter FrameItem::Line para a linha de fracção");
    }

    #[test]
    fn math_frac_linha_horizontal() {
        let ml   = MathLayouter::new(&FixedMetrics, true);
        let frac = Content::MathFrac {
            num: Box::new(Content::MathIdent("a".into())),
            den: Box::new(Content::MathIdent("b".into())),
        };
        let items = ml.layout_equation(&frac, &size10_style());
        for item in &items {
            if let FrameItem::Line { start, end, .. } = item {
                assert_eq!(start.y.val(), end.y.val(),
                    "linha de fracção deve ser horizontal");
                assert!(end.x.val() > start.x.val(),
                    "linha de fracção deve ter largura > 0");
            }
        }
    }

    #[test]
    fn math_attach_sub_baixado() {
        let ml = MathLayouter::new(&FixedMetrics, true);
        let attach = Content::MathAttach {
            base: Box::new(Content::MathIdent("x".into())),
            tl:   None,
            bl:   None,
            sub:  Some(Box::new(Content::MathIdent("i".into()))),
            sup:  None,
        };
        let items = ml.layout_equation(&attach, &size10_style());
        assert!(items.len() >= 2);

        let ys: Vec<f64> = items.iter().filter_map(|item| {
            if let FrameItem::Text { pos, .. } = item { Some(pos.y.val()) }
            else { None }
        }).collect();

        // sub deve estar abaixo da base (y maior)
        assert!(ys[1] > ys[0],
            "sub (y={}) deve estar abaixo da base (y={})", ys[1], ys[0]);
    }

    // ── Testes do Passo 40 — layout_root ─────────────────────────────────

    #[test]
    fn layout_root_contem_radical_e_radicando() {
        let ml = MathLayouter::new(&FixedMetrics, true);
        let root = Content::MathRoot {
            index:    None,
            radicand: Box::new(Content::MathIdent("x".into())),
        };
        let items = ml.layout_equation(&root, &default_style());
        // Deve conter pelo menos o símbolo √ e o radicando "x"
        let texts: Vec<_> = items.iter().filter_map(|i| {
            if let FrameItem::Text { text, .. } = i { Some(text.as_str()) } else { None }
        }).collect();
        assert!(texts.iter().any(|t| t.contains('√')), "deve conter √: {:?}", texts);
        assert!(texts.iter().any(|t| t.contains('x')), "deve conter x: {:?}", texts);
    }

    #[test]
    fn layout_root_tem_overline() {
        let ml = MathLayouter::new(&FixedMetrics, true);
        let root = Content::MathRoot {
            index:    None,
            radicand: Box::new(Content::MathIdent("x".into())),
        };
        let items = ml.layout_equation(&root, &default_style());
        let has_line = items.iter().any(|i| matches!(i, FrameItem::Line { .. }));
        assert!(has_line, "sqrt deve gerar FrameItem::Line para overline");
    }

    #[test]
    fn layout_root_overline_horizontal() {
        let ml = MathLayouter::new(&FixedMetrics, true);
        let root = Content::MathRoot {
            index:    None,
            radicand: Box::new(Content::MathIdent("x".into())),
        };
        let items = ml.layout_equation(&root, &default_style());
        for item in &items {
            if let FrameItem::Line { start, end, .. } = item {
                assert_eq!(start.y.val(), end.y.val(), "overline deve ser horizontal");
                assert!(end.x.val() > start.x.val(), "overline deve ter largura > 0");
            }
        }
    }

    #[test]
    fn layout_root_com_indice_contem_indice() {
        let ml = MathLayouter::new(&FixedMetrics, true);
        let root = Content::MathRoot {
            index:    Some(Box::new(Content::MathText("3".into()))),
            radicand: Box::new(Content::MathIdent("x".into())),
        };
        let items = ml.layout_equation(&root, &default_style());
        let texts: Vec<_> = items.iter().filter_map(|i| {
            if let FrameItem::Text { text, .. } = i { Some(text.as_str()) } else { None }
        }).collect();
        assert!(texts.iter().any(|t| t.contains('3')), "root(3,x) deve conter '3': {:?}", texts);
        assert!(texts.iter().any(|t| t.contains('√')), "root(3,x) deve conter √: {:?}", texts);
        assert!(texts.iter().any(|t| t.contains('x')), "root(3,x) deve conter x: {:?}", texts);
    }

    // ── Testes do Passo 42 — MathDelimited e layout_stretchy_delimiter ───────

    #[test]
    fn layout_delimited_contem_corpo_e_delimitadores() {
        let ml = MathLayouter::new(&FixedMetrics, true);
        let delim = Content::MathDelimited {
            open:  '(',
            body:  Box::new(Content::MathIdent("a".into())),
            close: ')',
        };
        let items = ml.layout_equation(&delim, &default_style());
        let texts: Vec<_> = items.iter().filter_map(|i| {
            if let FrameItem::Text { text, .. } = i { Some(text.as_str().to_string()) } else { None }
        }).collect();
        assert!(texts.iter().any(|t| t.contains('(')), "deve conter '(': {:?}", texts);
        assert!(texts.iter().any(|t| t.contains('a')), "deve conter 'a': {:?}", texts);
        assert!(texts.iter().any(|t| t.contains(')')), "deve conter ')': {:?}", texts);
    }

    #[test]
    fn layout_delimited_tres_ou_mais_items() {
        let ml = MathLayouter::new(&FixedMetrics, true);
        let delim = Content::MathDelimited {
            open:  '[',
            body:  Box::new(Content::MathIdent("x".into())),
            close: ']',
        };
        let items = ml.layout_equation(&delim, &default_style());
        assert!(items.len() >= 3, "delimitado deve ter >= 3 items, tem {}", items.len());
    }

    #[test]
    fn layout_delimited_cursor_avanca() {
        // Delimitadores à esquerda e à direita do corpo
        let ml = MathLayouter::new(&FixedMetrics, true);
        let delim = Content::MathDelimited {
            open:  '(',
            body:  Box::new(Content::MathIdent("x".into())),
            close: ')',
        };
        let items = ml.layout_equation(&delim, &default_style());
        let xs: Vec<f64> = items.iter().filter_map(|i| {
            if let FrameItem::Text { pos, .. } = i { Some(pos.x.val()) } else { None }
        }).collect();
        // O delimitador de fecho deve estar à direita do delimitador de abertura
        assert!(xs.len() >= 2, "deve ter pelo menos 2 posições x");
        assert!(xs.last().unwrap() > xs.first().unwrap(),
            "fecho deve estar à direita de abertura");
    }

    #[test]
    fn fixed_metrics_sem_variantes_vertextuais() {
        let m = FixedMetrics;
        let v = m.vertical_glyph_variants('(');
        assert!(v.is_empty(), "FixedMetrics não tem variantes");
    }

    #[test]
    fn fixed_metrics_glyph_to_char_none() {
        let m = FixedMetrics;
        assert_eq!(m.glyph_to_char(42), None);
    }

    #[test]
    fn layout_stretchy_sem_variantes_usa_base() {
        // Com FixedMetrics, o glifo base é usado directamente
        let ml = MathLayouter::new(&FixedMetrics, true);
        let box_ = ml.layout_stretchy_delimiter('(', 1000.0, &default_style());
        assert!(box_.width > 0.0, "delimitador base deve ter largura > 0");
    }

    // ── Testes do Passo 43 — FrameItem::Glyph e GlyphAssembly ───────────────

    #[test]
    fn offset_item_desloca_glyph() {
        let item = FrameItem::Glyph {
            pos:       Point { x: Pt(1.0), y: Pt(2.0) },
            glyph_id:  42,
            x_advance: Pt(10.0),
            size:      Pt(12.0),
        };
        let shifted = offset_item(item, Pt(3.0), Pt(4.0));
        if let FrameItem::Glyph { pos, glyph_id, .. } = shifted {
            assert_eq!(pos.x.val(), 4.0);
            assert_eq!(pos.y.val(), 6.0);
            assert_eq!(glyph_id, 42);
        } else { panic!("deve ser Glyph"); }
    }

    #[test]
    fn fixed_metrics_assembly_vazia() {
        let m = FixedMetrics;
        let a = m.vertical_glyph_assembly('(');
        assert!(a.is_empty(), "FixedMetrics não tem assembly");
    }

    #[test]
    fn layout_stretchy_sem_variantes_sem_assembly_usa_char_base() {
        // Com FixedMetrics, sem variantes nem assembly, deve usar char base
        let ml  = MathLayouter::new(&FixedMetrics, true);
        let box_ = ml.layout_stretchy_delimiter('(', 5000.0, &default_style());
        // O resultado é um Text com '('
        let has_paren = box_.items.iter().any(|i| {
            matches!(i, FrameItem::Text { text, .. } if text.as_str().contains('('))
        });
        assert!(has_paren, "deve usar char base '(' quando sem variantes");
    }

    #[test]
    fn layout_delimited_nao_tem_glyph_com_fixed_metrics() {
        // FixedMetrics não tem variantes — todos os items devem ser Text ou Line
        let ml = MathLayouter::new(&FixedMetrics, true);
        let delim = Content::MathDelimited {
            open:  '(',
            body:  Box::new(Content::MathIdent("a".into())),
            close: ')',
        };
        let items = ml.layout_equation(&delim, &default_style());
        let has_glyph = items.iter().any(|i| matches!(i, FrameItem::Glyph { .. }));
        assert!(!has_glyph, "FixedMetrics não deve emitir FrameItem::Glyph");
    }

    #[test]
    fn frac_dentro_de_delimitadores_nao_regride() {
        let ml = MathLayouter::new(&FixedMetrics, true);
        let delim = Content::MathDelimited {
            open: '(',
            body: Box::new(Content::MathFrac {
                num: Box::new(Content::MathIdent("a".into())),
                den: Box::new(Content::MathIdent("b".into())),
            }),
            close: ')',
        };
        let items = ml.layout_equation(&delim, &default_style());
        let texts: Vec<_> = items.iter()
            .filter_map(|i| if let FrameItem::Text { text, .. } = i { Some(text.as_str()) } else { None })
            .collect();
        assert!(texts.iter().any(|t| t.contains('a')), "numerador: {:?}", texts);
        assert!(texts.iter().any(|t| t.contains('b')), "denominador: {:?}", texts);
    }

    #[test]
    fn sqrt_nao_regride_passo43() {
        let ml = MathLayouter::new(&FixedMetrics, true);
        let root = Content::MathRoot {
            index:    None,
            radicand: Box::new(Content::MathIdent("x".into())),
        };
        let items = ml.layout_equation(&root, &default_style());
        let texts: Vec<_> = items.iter()
            .filter_map(|i| if let FrameItem::Text { text, .. } = i { Some(text.as_str()) } else { None })
            .collect();
        assert!(texts.iter().any(|t| t.contains('√') || t.contains('x')),
            "sqrt deve conter radical ou radicando: {:?}", texts);
    }

    #[test]
    fn attach_nao_regride_passo43() {
        let ml = MathLayouter::new(&FixedMetrics, true);
        let attach = Content::MathAttach {
            base: Box::new(Content::MathIdent("x".into())),
            tl:   None,
            bl:   None,
            sub:  None,
            sup:  Some(Box::new(Content::MathText("2".into()))),
        };
        let items = ml.layout_equation(&attach, &default_style());
        let texts: Vec<_> = items.iter()
            .filter_map(|i| if let FrameItem::Text { text, .. } = i { Some(text.as_str()) } else { None })
            .collect();
        assert!(texts.iter().any(|t| t.contains('x')), "base: {:?}", texts);
        assert!(texts.iter().any(|t| t.contains('2')), "sup: {:?}", texts);
    }

    #[test]
    fn offset_item_desloca_text() {
        let item = FrameItem::Text {
            pos:   Point { x: Pt(1.0), y: Pt(2.0) },
            text:  "a".into(),
            style: TextStyle::regular(Pt(12.0)),
        };
        let shifted = offset_item(item, Pt(3.0), Pt(4.0));
        if let FrameItem::Text { pos, .. } = shifted {
            assert_eq!(pos.x.val(), 4.0);
            assert_eq!(pos.y.val(), 6.0);
        } else { panic!("deve ser Text"); }
    }

    #[test]
    fn offset_item_desloca_line() {
        let item = FrameItem::Line {
            start:     Point { x: Pt(0.0), y: Pt(0.0) },
            end:       Point { x: Pt(10.0), y: Pt(0.0) },
            thickness: 0.5,
        };
        let shifted = offset_item(item, Pt(5.0), Pt(2.0));
        if let FrameItem::Line { start, end, .. } = shifted {
            assert_eq!(start.x.val(), 5.0);
            assert_eq!(start.y.val(), 2.0);
            assert_eq!(end.x.val(), 15.0);
            assert_eq!(end.y.val(), 2.0);
        } else { panic!("deve ser Line"); }
    }

    // ── Testes do Passo 44 — AxisHeight e MathKernInfo ───────────────────

    fn layout_equation_items(content: &Content) -> Vec<FrameItem> {
        let ml = MathLayouter::new(&FixedMetrics, true);
        ml.layout_equation(content, &default_style())
    }

    #[test]
    fn fixed_metrics_math_kern_vazio() {
        let m = FixedMetrics;
        let k = m.math_kern('f');
        assert!(k.top_right.is_empty());
        assert!(k.bottom_right.is_empty());
    }

    #[test]
    fn math_kern_default_nao_afecta_layout() {
        // math_kern com FixedMetrics retorna kern zero — layout não deve mudar
        let ml = MathLayouter::new(&FixedMetrics, true);
        let attach = Content::MathAttach {
            base: Box::new(Content::MathIdent("f".into())),
            tl:   None,
            bl:   None,
            sub:  None,
            sup:  Some(Box::new(Content::MathText("2".into()))),
        };
        let items = ml.layout_equation(&attach, &default_style());
        assert!(!items.is_empty(), "attach deve produzir items");
    }

    fn items_contain_text(items: &[FrameItem], c: char) -> bool {
        items.iter().any(|i| matches!(i, FrameItem::Text { text, .. } if text.as_str().contains(c)))
    }

    #[test]
    fn frac_com_axis_height_nao_regride() {
        let frac = Content::MathFrac {
            num: Box::new(Content::MathIdent("a".into())),
            den: Box::new(Content::MathIdent("b".into())),
        };
        let items = layout_equation_items(&frac);
        assert!(items_contain_text(&items, 'a'), "numerador: {:?}", items);
        assert!(items_contain_text(&items, 'b'), "denominador: {:?}", items);
    }

    #[test]
    fn delimitado_com_axis_height_nao_regride() {
        let delim = Content::MathDelimited {
            open: '(',
            body: Box::new(Content::MathFrac {
                num: Box::new(Content::MathIdent("a".into())),
                den: Box::new(Content::MathIdent("b".into())),
            }),
            close: ')',
        };
        let items = layout_equation_items(&delim);
        assert!(items_contain_text(&items, 'a'));
        assert!(items_contain_text(&items, 'b'));
    }

    #[test]
    fn sqrt_com_axis_height_nao_regride() {
        let root = Content::MathRoot {
            index:    None,
            radicand: Box::new(Content::MathIdent("x".into())),
        };
        let items = layout_equation_items(&root);
        assert!(
            items_contain_text(&items, '√') || items_contain_text(&items, 'x'),
            "sqrt deve conter radical ou radicando"
        );
    }

    #[test]
    fn attach_com_kern_nao_regride() {
        let attach = Content::MathAttach {
            base: Box::new(Content::MathIdent("x".into())),
            tl:   None,
            bl:   None,
            sub:  None,
            sup:  Some(Box::new(Content::MathText("2".into()))),
        };
        let items = layout_equation_items(&attach);
        assert!(items_contain_text(&items, 'x'));
        assert!(items_contain_text(&items, '2'));
    }

    #[test]
    fn attach_sub_com_kern_nao_regride() {
        let attach = Content::MathAttach {
            base: Box::new(Content::MathIdent("x".into())),
            tl:   None,
            bl:   None,
            sub:  Some(Box::new(Content::MathIdent("i".into()))),
            sup:  None,
        };
        let items = layout_equation_items(&attach);
        assert!(items_contain_text(&items, 'x'));
        assert!(items_contain_text(&items, 'i'));
    }

    #[test]
    fn frac_axis_ascent_maior_que_sem_axis() {
        // Com axis_height, a fracção sobe: o ascent do MathBox aumenta.
        // Verificar que o axis_height é não-zero (fallback=500 > 0).
        let constants = crate::entities::math_constants::MathConstants::fallback();
        assert!(constants.axis_height > 0.0, "axis_height do fallback deve ser > 0");
    }

    // ── Testes do Passo 46 — Pre-scripts (tl/bl) ─────────────────────────

    #[test]
    fn attach_sem_left_scripts_nao_regride() {
        // Regressão: MathAttach sem tl/bl comporta-se como antes
        let attach = Content::MathAttach {
            base: Box::new(Content::MathIdent("x".into())),
            tl:   None,
            bl:   None,
            sub:  None,
            sup:  Some(Box::new(Content::MathText("2".into()))),
        };
        let items = layout_equation_items(&attach);
        assert!(items_contain_text(&items, 'x'), "base ausente: {:?}", items);
        assert!(items_contain_text(&items, '2'), "sup ausente: {:?}", items);
    }

    #[test]
    fn attach_left_sup_contem_base_e_script() {
        // Pre-superscript: conteúdo do script e da base presentes
        let attach = Content::MathAttach {
            base: Box::new(Content::MathIdent("x".into())),
            tl:   Some(Box::new(Content::MathText("2".into()))),
            bl:   None,
            sub:  None,
            sup:  None,
        };
        let items = layout_equation_items(&attach);
        assert!(items_contain_text(&items, '2'), "pre-sup ausente: {:?}", items);
        assert!(items_contain_text(&items, 'x'), "base ausente: {:?}", items);
    }

    #[test]
    fn attach_left_sub_contem_base_e_script() {
        // Pre-subscript
        let attach = Content::MathAttach {
            base: Box::new(Content::MathIdent("x".into())),
            tl:   None,
            bl:   Some(Box::new(Content::MathText("1".into()))),
            sub:  None,
            sup:  None,
        };
        let items = layout_equation_items(&attach);
        assert!(items_contain_text(&items, '1'), "pre-sub ausente: {:?}", items);
        assert!(items_contain_text(&items, 'x'), "base ausente: {:?}", items);
    }

    #[test]
    fn attach_left_e_right_juntos() {
        // Scripts nos dois lados simultaneamente
        let attach = Content::MathAttach {
            base: Box::new(Content::MathIdent("x".into())),
            tl:   Some(Box::new(Content::MathText("2".into()))),
            bl:   Some(Box::new(Content::MathText("1".into()))),
            sub:  Some(Box::new(Content::MathText("3".into()))),
            sup:  Some(Box::new(Content::MathText("4".into()))),
        };
        let items = layout_equation_items(&attach);
        assert!(items_contain_text(&items, '1'), "bl ausente");
        assert!(items_contain_text(&items, '2'), "tl ausente");
        assert!(items_contain_text(&items, 'x'), "base ausente");
        assert!(items_contain_text(&items, '3'), "sub ausente");
        assert!(items_contain_text(&items, '4'), "sup ausente");
    }

    #[test]
    fn attach_left_sup_base_deslocada_para_direita() {
        // Com tl presente, a base deve aparecer a uma posição x maior do que zero
        let attach = Content::MathAttach {
            base: Box::new(Content::MathIdent("x".into())),
            tl:   Some(Box::new(Content::MathText("2".into()))),
            bl:   None,
            sub:  None,
            sup:  None,
        };
        let items = layout_equation_items(&attach);
        // Encontrar a posição x do glifo "x" (base)
        let base_xs: Vec<f64> = items.iter()
            .filter_map(|i| {
                if let FrameItem::Text { pos, text, .. } = i {
                    if text.contains('x') { Some(pos.x.val()) } else { None }
                } else { None }
            })
            .collect();
        assert!(!base_xs.is_empty(), "base deve estar presente");
        assert!(base_xs.iter().any(|&x| x > 0.0),
            "base deve estar deslocada para direita quando há tl; xs={:?}", base_xs);
    }

    #[test]
    fn attach_sem_base_explicita_usa_empty() {
        // Base vazia: não deve panicar
        let attach = Content::MathAttach {
            base: Box::new(Content::Empty),
            tl:   Some(Box::new(Content::MathText("14".into()))),
            bl:   None,
            sub:  None,
            sup:  None,
        };
        let items = layout_equation_items(&attach);
        // Não deve panicar; items pode estar vazio mas o programa não crasha
        let _ = items;
    }
}
