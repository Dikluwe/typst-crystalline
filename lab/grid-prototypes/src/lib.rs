//! Laboratório independente: estratégias de medição e emissão de células.
//! Não é uma implementação de Typst e não é importado pelas camadas L1–L4.

use rustc_hash::FxHashMap;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Column {
    Auto,
    Fixed(u32),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Align {
    Top,
    Middle,
    Bottom,
}

#[derive(Clone, Debug)]
pub struct Cell {
    pub id: usize,
    pub row: usize,
    pub col: usize,
    pub colspan: usize,
    pub rowspan: usize,
    pub words: Arc<[u16]>,
    pub font_scale: u16,
    pub inset: u16,
    pub align: Align,
    /// Representa conteúdo que observa página/localização; impede reúso.
    pub page_sensitive: bool,
}

#[derive(Clone, Debug)]
pub struct Grid {
    pub columns: Vec<Column>,
    pub cells: Vec<Cell>,
    pub page_width: u32,
    pub page_height: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Options {
    pub reuse_frame: bool,
    pub height_only: bool,
    pub page_window: bool,
    pub width_cache: bool,
}

impl Options {
    pub const BASELINE: Self = Self {
        reuse_frame: false,
        height_only: false,
        page_window: false,
        width_cache: false,
    };

    pub fn from_mask(mask: u8) -> Self {
        Self {
            reuse_frame: mask & 1 != 0,
            height_only: mask & 2 != 0,
            page_window: mask & 4 != 0,
            width_cache: mask & 8 != 0,
        }
    }

    pub fn label(self) -> String {
        if self == Self::BASELINE {
            return "baseline".into();
        }
        [
            (self.reuse_frame, "reuse"),
            (self.height_only, "height"),
            (self.page_window, "window"),
            (self.width_cache, "width"),
        ]
        .into_iter()
        .filter_map(|(enabled, name)| enabled.then_some(name))
        .collect::<Vec<_>>()
        .join("+")
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Stats {
    pub intrinsic_calls: usize,
    pub intrinsic_hits: usize,
    pub full_layouts: usize,
    pub height_layouts: usize,
    pub frame_reuses: usize,
    pub frame_fallbacks: usize,
    pub peak_saved_lines: usize,
    pub measured_rows: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlacedLine {
    pub cell: usize,
    pub page: u32,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub page_mark: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Output {
    pub column_widths: Vec<u32>,
    pub row_heights: Vec<u32>,
    pub lines: Vec<PlacedLine>,
    pub pages: u32,
}

#[derive(Clone, Debug)]
pub struct Run {
    pub output: Output,
    pub stats: Stats,
}

#[derive(Clone, Debug)]
struct Frame {
    line_widths: Vec<u32>,
    height: u32,
}

#[derive(Clone, Debug)]
struct SavedFrame {
    frame: Frame,
    body_width: u32,
    font_scale: u16,
    inset: u16,
}

#[derive(Hash, Eq, PartialEq)]
struct WidthKey {
    address: usize,
    len: usize,
    font_scale: u16,
}

struct Engine<'a> {
    grid: &'a Grid,
    options: Options,
    stats: Stats,
    widths: FxHashMap<WidthKey, (u32, u32)>,
    saved: Vec<Option<SavedFrame>>,
    saved_lines: usize,
}

impl<'a> Engine<'a> {
    fn new(grid: &'a Grid, options: Options) -> Self {
        Self {
            grid,
            options,
            stats: Stats::default(),
            widths: FxHashMap::default(),
            saved: vec![None; grid.cells.len()],
            saved_lines: 0,
        }
    }

    fn intrinsic(&mut self, cell: &Cell) -> (u32, u32) {
        // Identidade de uma alocação já compartilhada: O(1), sem copiar nem
        // percorrer palavras. Conteúdo apenas igual, mas distinto, não acerta.
        let key =
            (self.options.width_cache && Arc::strong_count(&cell.words) > 1).then(|| {
                WidthKey {
                    address: Arc::as_ptr(&cell.words) as *const u16 as usize,
                    len: cell.words.len(),
                    font_scale: cell.font_scale,
                }
            });
        if let Some(key) = key.as_ref() {
            if let Some(&result) = self.widths.get(key) {
                self.stats.intrinsic_hits += 1;
                return result;
            }
        }
        self.stats.intrinsic_calls += 1;
        let scale = u32::from(cell.font_scale);
        let min = cell.words.iter().copied().max().unwrap_or(0) as u32 * scale;
        let max = cell.words.iter().map(|&w| u32::from(w) * scale).sum::<u32>()
            + cell.words.len().saturating_sub(1) as u32 * scale;
        let result = (min, max);
        if let Some(key) = key {
            self.widths.insert(key, result);
        }
        result
    }

    fn columns(&mut self) -> Vec<u32> {
        let count = self.grid.columns.len();
        let fixed = self
            .grid
            .columns
            .iter()
            .map(|c| match c {
                Column::Fixed(w) => *w,
                Column::Auto => 0,
            })
            .sum::<u32>();
        let auto_count =
            self.grid.columns.iter().filter(|c| matches!(c, Column::Auto)).count();
        let share = (self.grid.page_width.saturating_sub(fixed)
            / auto_count.max(1) as u32)
            .max(1);
        let mut widths = self
            .grid
            .columns
            .iter()
            .map(|c| match c {
                Column::Fixed(w) => *w,
                Column::Auto => 1,
            })
            .collect::<Vec<_>>();
        for cell in &self.grid.cells {
            if cell.colspan != 1 || !matches!(self.grid.columns[cell.col], Column::Auto) {
                continue;
            }
            let (min, max) = self.intrinsic(cell);
            let desired = max.min(share).max(min) + 2 * u32::from(cell.inset);
            widths[cell.col] = widths[cell.col].max(desired);
        }
        debug_assert_eq!(widths.len(), count);
        widths
    }

    fn body_width(cell: &Cell, widths: &[u32]) -> u32 {
        widths[cell.col..cell.col + cell.colspan]
            .iter()
            .sum::<u32>()
            .saturating_sub(2 * u32::from(cell.inset))
    }

    /// A mesma quebra de palavras nos caminhos completo e só-altura.
    fn lay_out(&mut self, cell: &Cell, width: u32, frame: bool) -> Frame {
        if frame {
            self.stats.full_layouts += 1;
        } else {
            self.stats.height_layouts += 1;
        }
        let mut lines = if frame { Vec::new() } else { Vec::with_capacity(0) };
        let mut count = 0_u32;
        let mut used = 0_u32;
        let scale = u32::from(cell.font_scale);
        for &word in cell.words.iter() {
            let word_width = u32::from(word) * scale;
            let next = if used == 0 { word_width } else { used + scale + word_width };
            if used != 0 && next > width {
                count += 1;
                if frame {
                    lines.push(used);
                }
                used = word_width;
            } else {
                used = next;
            }
        }
        if used != 0 {
            count += 1;
            if frame {
                lines.push(used);
            }
        }
        Frame { line_widths: lines, height: count * (scale + 2) }
    }

    fn can_reuse(cell: &Cell) -> bool {
        // Escopo intencionalmente conservador: posição/página, alinhamento
        // vertical e spans podem mudar o contexto entre medir e emitir.
        !cell.page_sensitive
            && cell.rowspan == 1
            && cell.colspan == 1
            && cell.align == Align::Top
    }

    fn measure_cell(&mut self, index: usize, widths: &[u32]) -> u32 {
        let cell = &self.grid.cells[index];
        let body_width = Self::body_width(cell, widths);
        let save = self.options.reuse_frame && Self::can_reuse(cell);
        let frame = self.lay_out(cell, body_width, save || !self.options.height_only);
        let height = frame.height + 2 * u32::from(cell.inset);
        if save {
            self.saved_lines += frame.line_widths.len();
            self.stats.peak_saved_lines =
                self.stats.peak_saved_lines.max(self.saved_lines);
            self.saved[index] = Some(SavedFrame {
                frame,
                body_width,
                font_scale: cell.font_scale,
                inset: cell.inset,
            });
        }
        height
    }

    fn emit_cell(
        &mut self,
        index: usize,
        widths: &[u32],
        row_height: u32,
        page: u32,
        row_y: u32,
        x: u32,
        output: &mut Vec<PlacedLine>,
    ) {
        let cell = &self.grid.cells[index];
        let width = Self::body_width(cell, widths);
        let saved = self.saved[index].take();
        let frame = if let Some(saved) = saved {
            self.saved_lines -= saved.frame.line_widths.len();
            if Self::can_reuse(cell)
                && saved.body_width == width
                && saved.font_scale == cell.font_scale
                && saved.inset == cell.inset
            {
                self.stats.frame_reuses += 1;
                saved.frame
            } else {
                self.stats.frame_fallbacks += 1;
                self.lay_out(cell, width, true)
            }
        } else {
            if self.options.reuse_frame {
                self.stats.frame_fallbacks += 1;
            }
            self.lay_out(cell, width, true)
        };
        let body_room = row_height.saturating_sub(2 * u32::from(cell.inset));
        let offset = match cell.align {
            Align::Top => 0,
            Align::Middle => body_room.saturating_sub(frame.height) / 2,
            Align::Bottom => body_room.saturating_sub(frame.height),
        };
        let mark = if cell.page_sensitive { page } else { 0 };
        for (line, width) in frame.line_widths.into_iter().enumerate() {
            output.push(PlacedLine {
                cell: cell.id,
                page,
                x: x + u32::from(cell.inset),
                y: row_y
                    + u32::from(cell.inset)
                    + offset
                    + line as u32 * (u32::from(cell.font_scale) + 2),
                width,
                page_mark: mark,
            });
        }
    }
}

/// Executa os quatro eixos de forma independente ou combinada.
pub fn run(grid: &Grid, options: Options, first_page_only: bool) -> Result<Run, String> {
    if grid.columns.is_empty() || grid.page_height == 0 {
        return Err("grid sem colunas ou altura de página".into());
    }
    for cell in &grid.cells {
        if cell.col >= grid.columns.len()
            || cell.colspan == 0
            || cell.col + cell.colspan > grid.columns.len()
            || cell.rowspan == 0
        {
            return Err(format!("célula {} fora da grade", cell.id));
        }
    }
    let rows = grid.cells.iter().map(|c| c.row + c.rowspan).max().unwrap_or(0);
    let mut by_row = vec![Vec::new(); rows];
    for (index, cell) in grid.cells.iter().enumerate() {
        by_row[cell.row].push(index);
    }
    let mut engine = Engine::new(grid, options);
    let widths = engine.columns();
    let mut heights = vec![0_u32; rows];
    let mut pending_spans: Vec<Vec<u32>> = vec![Vec::new(); rows];
    let mut lines = Vec::new();
    let mut page = 0_u32;
    let mut row_y = 0_u32;
    let mut measured = 0_usize;
    let mut visible_rows = 0_usize;
    let mut measured_height = 0_u32;

    // A janela mede imediatamente antes de emitir; o baseline mede todas
    // as linhas antes. Ambas aplicam a mesma política de paginação.
    if !options.page_window {
        for row in 0..rows {
            measure_row(
                &mut engine,
                row,
                measured_height,
                &by_row,
                &widths,
                &mut heights,
                &mut pending_spans,
            );
            measured_height += heights[row];
            measured += 1;
        }
    }
    let x_starts = widths
        .iter()
        .scan(0_u32, |x, w| {
            let start = *x;
            *x += *w;
            Some(start)
        })
        .collect::<Vec<_>>();
    for row in 0..rows {
        if options.page_window {
            measure_row(
                &mut engine,
                row,
                measured_height,
                &by_row,
                &widths,
                &mut heights,
                &mut pending_spans,
            );
            measured_height += heights[row];
            measured += 1;
        }
        if row_y != 0 && row_y + heights[row] > grid.page_height {
            page += 1;
            row_y = 0;
        }
        if first_page_only && page > 0 {
            break;
        }
        for &index in &by_row[row] {
            engine.emit_cell(
                index,
                &widths,
                heights[row],
                page,
                row_y,
                x_starts[grid.cells[index].col],
                &mut lines,
            );
        }
        visible_rows += 1;
        row_y += heights[row];
    }
    engine.stats.measured_rows = measured;
    if first_page_only {
        heights.truncate(visible_rows);
    }
    Ok(Run {
        output: Output {
            column_widths: widths,
            row_heights: heights,
            lines,
            pages: if first_page_only { 1 } else { page + 1 },
        },
        stats: engine.stats,
    })
}

fn measure_row(
    engine: &mut Engine<'_>,
    row: usize,
    previous_height: u32,
    by_row: &[Vec<usize>],
    widths: &[u32],
    heights: &mut [u32],
    pending_spans: &mut [Vec<u32>],
) {
    let mut height = 0_u32;
    for &index in &by_row[row] {
        let cell = &engine.grid.cells[index];
        let cell_height = engine.measure_cell(index, widths);
        if cell.rowspan == 1 {
            height = height.max(cell_height);
        } else {
            let end = row + cell.rowspan - 1;
            pending_spans[end].push(previous_height + cell_height);
        }
    }
    for &required_bottom in &pending_spans[row] {
        height = height.max(required_bottom.saturating_sub(previous_height));
    }
    heights[row] = height;
}

pub fn fixture(rows: usize, repeated: bool) -> Grid {
    fixture_with_word_repeat(rows, repeated, 1)
}

/// Repete a sequência de palavras antes de acrescentar a identidade única.
/// Permite medir o ponto em que uma chave de cache compensa a conta intrínseca.
pub fn fixture_with_word_repeat(rows: usize, repeated: bool, word_repeat: usize) -> Grid {
    let mut cells = Vec::with_capacity(rows * 4);
    let shared_words = (0..4)
        .map(|col| Arc::<[u16]>::from(word_pattern(col, word_repeat)))
        .collect::<Vec<_>>();
    for row in 0..rows {
        for col in 0..4 {
            let id = row * 4 + col;
            let seed = if repeated { col } else { id };
            let words = if repeated {
                Arc::clone(&shared_words[col])
            } else {
                let mut words = word_pattern(seed, word_repeat);
                // Código de cinco dígitos, suficiente para as fixtures usuais.
                // Evita que o padrão modular acima finja ter conteúdo único.
                let mut identity = id;
                for _ in 0..5 {
                    words.push(3 + (identity % 7) as u16);
                    identity /= 7;
                }
                Arc::from(words)
            };
            cells.push(Cell {
                id,
                row,
                col,
                colspan: 1,
                rowspan: 1,
                words,
                font_scale: 1 + (seed % 2) as u16,
                inset: (col % 3) as u16,
                align: if row % 11 == 0 { Align::Middle } else { Align::Top },
                page_sensitive: row % 17 == 0,
            });
        }
    }
    Grid {
        columns: vec![Column::Auto, Column::Fixed(90), Column::Auto, Column::Fixed(90)],
        cells,
        page_width: 440,
        page_height: 210,
    }
}

fn word_pattern(seed: usize, word_repeat: usize) -> Vec<u16> {
    let base = (0..(4 + seed % 9))
        .map(|i| (3 + (seed * 7 + i * 11) % 18) as u16)
        .collect::<Vec<_>>();
    base.repeat(word_repeat)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_combinations_match_baseline() {
        for repeated in [false, true] {
            let grid = fixture(43, repeated);
            let expected = run(&grid, Options::BASELINE, false).unwrap().output;
            for mask in 0..16 {
                let got = run(&grid, Options::from_mask(mask), false).unwrap();
                assert_eq!(
                    got.output,
                    expected,
                    "{} repeated={repeated}",
                    Options::from_mask(mask).label()
                );
            }
        }
    }

    #[test]
    fn window_measures_only_through_first_page() {
        let grid = fixture(80, true);
        let baseline = run(&grid, Options::BASELINE, true).unwrap();
        let window = run(&grid, Options::from_mask(4), true).unwrap();
        assert_eq!(baseline.output.lines, window.output.lines);
        assert!(window.stats.measured_rows < baseline.stats.measured_rows);
    }

    #[test]
    fn reuse_reduces_full_layouts_and_falls_back_for_context() {
        let grid = fixture(40, true);
        let baseline = run(&grid, Options::BASELINE, false).unwrap();
        let variant = run(&grid, Options::from_mask(1), false).unwrap();
        assert_eq!(baseline.output, variant.output);
        assert!(variant.stats.frame_reuses > 0);
        assert!(variant.stats.frame_fallbacks > 0);
        assert!(variant.stats.full_layouts < baseline.stats.full_layouts);
    }

    #[test]
    fn height_only_and_width_cache_do_less_modeled_work() {
        let grid = fixture(40, true);
        let baseline = run(&grid, Options::BASELINE, false).unwrap();
        let variant = run(&grid, Options::from_mask(2 | 8), false).unwrap();
        assert_eq!(baseline.output, variant.output);
        assert!(variant.stats.height_layouts > 0);
        assert!(variant.stats.intrinsic_hits > 0);
        assert!(variant.stats.intrinsic_calls < baseline.stats.intrinsic_calls);
        let unique = run(&fixture(40, false), Options::from_mask(8), false).unwrap();
        assert_eq!(unique.stats.intrinsic_hits, 0);
    }

    #[test]
    fn width_cache_requires_shared_allocation_and_matching_scale() {
        let shared = fixture(2, true);
        assert!(Arc::ptr_eq(&shared.cells[0].words, &shared.cells[4].words));
        let mut distinct = shared.clone();
        for cell in &mut distinct.cells {
            cell.words = Arc::from(cell.words.as_ref().to_vec());
        }
        let distinct_run = run(&distinct, Options::from_mask(8), false).unwrap();
        assert_eq!(distinct_run.stats.intrinsic_hits, 0);
        assert_eq!(
            distinct_run.output,
            run(&distinct, Options::BASELINE, false).unwrap().output
        );

        let mut styled = shared;
        styled.cells[4].font_scale += 1;
        let styled_run = run(&styled, Options::from_mask(8), false).unwrap();
        assert_eq!(styled_run.stats.intrinsic_calls, 3);
        assert_eq!(styled_run.stats.intrinsic_hits, 1);
        assert_eq!(
            styled_run.output,
            run(&styled, Options::BASELINE, false).unwrap().output
        );
    }

    #[test]
    fn spans_and_page_sensitive_cells_keep_fallback() {
        let mut grid = fixture(20, true);
        grid.cells[0].rowspan = 2;
        grid.cells[1].colspan = 2;
        grid.cells
            .retain(|cell| !matches!((cell.row, cell.col), (0, 2) | (1, 0)));
        let expected = run(&grid, Options::BASELINE, false).unwrap().output;
        for mask in 0..16 {
            assert_eq!(
                run(&grid, Options::from_mask(mask), false).unwrap().output,
                expected
            );
        }
        assert!(
            run(&grid, Options::from_mask(1), false)
                .unwrap()
                .stats
                .frame_fallbacks
                > 0
        );
    }

    #[test]
    fn all_combinations_match_on_first_page() {
        let grid = fixture(80, true);
        let expected = run(&grid, Options::BASELINE, true).unwrap().output;
        for mask in 0..16 {
            assert_eq!(
                run(&grid, Options::from_mask(mask), true).unwrap().output,
                expected
            );
        }
        let all_rows = run(&grid, Options::from_mask(1), false).unwrap();
        let window = run(&grid, Options::from_mask(1 | 4), false).unwrap();
        assert!(window.stats.peak_saved_lines < all_rows.stats.peak_saved_lines);
    }
}
