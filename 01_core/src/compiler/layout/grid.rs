//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout.md
//! @prompt-hash 28c2110a
//! @layer L1
//! @updated 2026-07-24
//!
//! Braço `Content::Grid` do `layout_content`. Extraído de `layout/mod.rs`
//! no Passo 96.7 conforme ADR-0037.

use crate::entities::elements::grid_hline::GridHLineElem;
use crate::entities::elements::grid_vline::GridVLineElem;
use crate::entities::elements::table_hline::TableHLineElem;
use crate::entities::elements::table_vline::TableVLineElem;
use super::PendingAlignYEntry;
use crate::entities::{
    content::Content,
    elements::grid::GridElem,
    geometry::{ShapeKind, Stroke},
    image_sizer::ImageSizer,
    layout_types::{Align2D, Color, FrameItem, Length, Point, Pt, TrackSizing},
    sides::Sides,
};

use super::grid_placement::{place_cells, PlacedCell};
use super::metrics::FontMetrics;
use super::{item_pos, translate_frame_item};

/// Trait privado para desenhar hlines tanto de grid como de table com o
/// mesmo algoritmo (`layout_grid`).
pub(super) trait LayoutHLine {
    fn start(&self) -> usize;
    fn end(&self) -> Option<usize>;
    fn row(&self) -> usize;
    /// **P739A** — `None` = `stroke: none` (linha não desenhada; paridade
    /// vanilla, medido).
    fn stroke(&self) -> Option<&Stroke>;
    fn position(&self) -> &str;
}

impl LayoutHLine for GridHLineElem {
    fn start(&self) -> usize {
        self.start
    }
    fn end(&self) -> Option<usize> {
        self.end
    }
    fn row(&self) -> usize {
        self.row
    }
    fn stroke(&self) -> Option<&Stroke> {
        self.stroke.as_ref()
    }
    fn position(&self) -> &str {
        self.position.as_str()
    }
}

impl LayoutHLine for TableHLineElem {
    fn start(&self) -> usize {
        self.start
    }
    fn end(&self) -> Option<usize> {
        self.end
    }
    fn row(&self) -> usize {
        self.row
    }
    fn stroke(&self) -> Option<&Stroke> {
        self.stroke.as_ref()
    }
    fn position(&self) -> &str {
        self.position.as_str()
    }
}

/// Trait privado para desenhar vlines tanto de grid como de table com o
/// mesmo algoritmo (`layout_grid`).
pub(super) trait LayoutVLine {
    fn start(&self) -> usize;
    fn end(&self) -> Option<usize>;
    fn col(&self) -> usize;
    fn stroke(&self) -> Option<&Stroke>;
    fn position(&self) -> &str;
}

impl LayoutVLine for GridVLineElem {
    fn start(&self) -> usize {
        self.start
    }
    fn end(&self) -> Option<usize> {
        self.end
    }
    fn col(&self) -> usize {
        self.col
    }
    fn stroke(&self) -> Option<&Stroke> {
        self.stroke.as_ref()
    }
    fn position(&self) -> &str {
        self.position.as_str()
    }
}

impl LayoutVLine for TableVLineElem {
    fn start(&self) -> usize {
        self.start
    }
    fn end(&self) -> Option<usize> {
        self.end
    }
    fn col(&self) -> usize {
        self.col
    }
    fn stroke(&self) -> Option<&Stroke> {
        self.stroke.as_ref()
    }
    fn position(&self) -> &str {
        self.position.as_str()
    }
}

/// Layout de `grid(...)` (atomização ADR-0109 P380): delega ao motor
/// `layout_grid` (cluster Grid+Table). Content-preserving — era inline no
/// `layout_content`.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut super::Layouter<M, S>,
    e: &GridElem,
) {
    layouter.layout_grid(
        &e.columns,
        &e.rows,
        &e.cells,
        &e.hlines,
        &e.vlines,
        e.gutter,
        e.align,
        e.inset,
        e.header.as_ref(),
        e.footer.as_ref(),
        e.stroke.as_ref(),
        e.fill.as_ref(),
    );
}

impl<'a, M: FontMetrics, S: ImageSizer> super::Layouter<'a, M, S> {
    /// Layout de `Content::Grid` — algoritmo de tracks (Passo 80, 83, 84.2,
    /// 84.6). Extraído no Passo 96.7. **P224 refino** — signature expandida
    /// com 5 fields (`gutter`/`align`/`inset`/`header`/`footer`); semantic
    /// real adiada para gutter/align/inset (paridade ADR-0054 graded;
    /// algoritmo placement via `grid_placement::place_cells` em sub-fase
    /// futura quando integração for substantiva). `_align`/`_inset`
    /// armazenados mas ignorados nesta versão — `gutter` aplicado
    /// horizontalmente como soma a col_starts (graded).
    pub(super) fn layout_grid<H: LayoutHLine, V: LayoutVLine>(
        &mut self,
        columns: &[TrackSizing],
        rows: &[TrackSizing],
        cells: &[Content],
        hlines: &[H],
        vlines: &[V],
        _gutter: Option<Length>,
        align: Option<Align2D>, // P232 — Grid-level align disponível para Place herdar
        inset: Sides<Length>,   // P235 — Grid-level inset (default per-cell)
        header: Option<&Content>,
        footer: Option<&Content>,
        stroke: Option<&Stroke>, // P227 — borders cell render Opção β
        fill: Option<&Color>,    // P228 — fill cell render Z-order correcto
    ) {
        // P232 — save/restore cell_align Grid-level para Place
        // herdar via `.or()` per eixo no arm Content::Place. Paridade
        // pattern cell_origin_* P84.6 mas com scope Grid-level (não
        // per-cell — align uniforme aplica-se a todas as cells do Grid).
        let saved_cell_align = self.cell_align;
        self.cell_align = align;
        let available_width = self.available_width();

        // Guarda Passo 83 — colunas vazias caem em [Auto].
        let cols: Vec<TrackSizing> =
            if columns.is_empty() { vec![TrackSizing::Auto] } else { columns.to_vec() };
        let num_cols = cols.len();

        // P772i — header/footer como row-group real: extrair as células do
        // corpo de `Content::GridHeader`/`GridFooter` (um `Content::Sequence`
        // se houver mais que uma célula, ou o `Content` da própria célula se
        // só houver uma — `Content::sequence` colapsa Vec de 1 elemento) e
        // colar antes/depois das células normais. Cada grupo é preenchido
        // com `Content::Empty` até múltiplo de `num_cols` — headers/footers
        // ocupam linhas inteiras (paridade vanilla, `resolve.rs` §"row
        // group"). **Scope-out explícito** (não silencioso — ver
        // 00_nucleo/diagnosticos/paridade-producao-p772i.md): renderiza uma
        // única vez, não repete em quebras de página (equivalente vanilla de
        // `Header`/`Footer`/`Repeatable<T>` com `range`/`level` não
        // implementado).
        fn row_group_cells(content: &Content, num_cols: usize) -> Vec<Content> {
            let mut group_cells = match content {
                Content::Sequence(items) => items.to_vec(),
                other => vec![other.clone()],
            };
            let remainder = group_cells.len() % num_cols;
            if remainder != 0 {
                group_cells
                    .resize(group_cells.len() + (num_cols - remainder), Content::Empty);
            }
            group_cells
        }
        // P772v — `layout_grid` é partilhado por `Content::Grid` e
        // `Content::Table` (cluster P379); os braços `TableHeader`/
        // `TableFooter` estendem o mesmo mecanismo de row-group de P772i
        // para `table()`, extraindo `.body` da mesma forma.
        let header_cells: Vec<Content> = header
            .map(|h| match h {
                Content::GridHeader(e) => row_group_cells(&e.body, num_cols),
                Content::TableHeader(e) => row_group_cells(&e.body, num_cols),
                other => row_group_cells(other, num_cols),
            })
            .unwrap_or_default();
        let footer_cells: Vec<Content> = footer
            .map(|f| match f {
                Content::GridFooter(e) => row_group_cells(&e.body, num_cols),
                Content::TableFooter(e) => row_group_cells(&e.body, num_cols),
                other => row_group_cells(other, num_cols),
            })
            .unwrap_or_default();
        // P789 — detecção de conflito célula↔header (paridade vanilla
        // `check_for_conflicting_cell_row`, resolve.rs:2112): célula do
        // corpo com `y` explícito cujo range `y..y+rowspan` intersecta as
        // linhas do header (0..header_rows) é erro, não sobreposição
        // silenciosa. Mensagem e hint idênticos ao vanilla (observável ao
        // nível da língua — ADR-0107). Só células do corpo são verificadas
        // (equivalente ao `!in_row_group` do vanilla); células com
        // `y: None` são auto-posicionadas e contornam o header. Conflito
        // célula↔footer é scope-out documentado no L0 (layout.md §P789):
        // o range absoluto do footer só existe pós-placement no modelo
        // splice. `Span::detached()` — mesmo trade-off de P647 (elementos
        // não carregam span).
        let header_rows = header_cells.len() / num_cols;
        if header_rows > 0 {
            let conflict = cells.iter().find_map(|cell| {
                let (y, rowspan) = match cell {
                    Content::GridCell(e) => (e.y, e.rowspan),
                    Content::TableCell(e) => (e.y, e.rowspan),
                    _ => (None, None),
                };
                let y = y?;
                let rowspan = rowspan.unwrap_or(1).max(1);
                // Linhas do header são contíguas a partir de 0 no modelo
                // splice — a primeira linha do range dentro do header é `y`.
                (y < header_rows).then_some(y)
            });
            if let Some(row) = conflict {
                self.layout_errors.push(
                    crate::entities::source_result::SourceDiagnostic::error(
                        crate::entities::span::Span::detached(),
                        format!(
                            "cell would conflict with header also spanning row {row}"
                        ),
                    )
                    .with_hint("try moving the cell or the header"),
                );
                self.cell_align = saved_cell_align;
                return;
            }
        }
        let combined_cells: Vec<Content> = if header_cells.is_empty()
            && footer_cells.is_empty()
        {
            cells.to_vec()
        } else {
            let mut combined =
                Vec::with_capacity(header_cells.len() + cells.len() + footer_cells.len());
            combined.extend(header_cells);
            combined.extend_from_slice(cells);
            combined.extend(footer_cells);
            combined
        };
        let cells: &[Content] = &combined_cells;

        // Guarda Passo 83 — rows vazias caem em [Auto] para evitar
        // panic por divisão por zero em N % rows.len() quando o AST
        // é construído manualmente (testes que ignoram a stdlib).
        let row_tracks: Vec<TrackSizing> =
            if rows.is_empty() { vec![TrackSizing::Auto] } else { rows.to_vec() };

        // ── Resolução de larguras (Passo 80, inalterado) ──────
        let mut cols_cells: Vec<Vec<usize>> = vec![vec![]; num_cols];
        for (idx, _) in cells.iter().enumerate() {
            cols_cells[idx % num_cols].push(idx);
        }

        let mut resolved_widths = vec![0.0_f64; num_cols];
        let mut total_fixed_w = 0.0_f64;
        let mut total_fr_w = 0.0_f64;

        for (i, sizing) in cols.iter().enumerate() {
            match sizing {
                TrackSizing::Fixed(w) => {
                    resolved_widths[i] = *w;
                    total_fixed_w += *w;
                }
                TrackSizing::Auto => {
                    // P233 — DEBT-34d fix: capar `safe` quando há fr
                    // tracks presentes para Auto NÃO consumir todo o
                    // remaining (deixando 0pt para fr). Sem fr presente,
                    // comportamento baseline preservado (P80).
                    //
                    // Estratégia subset minimal: dividir `safe_total`
                    // proporcionalmente entre auto + fr (split igualitário
                    // simples). Auto que precisar mais que `safe_capped`
                    // será truncado; fr recebe pelo menos `safe_total /
                    // (num_auto + num_fr) * num_fr_remaining`.
                    //
                    // Two-pass measure→place inaugurado P233 (pattern N=1).
                    // Resolução completa min-content/max-content
                    // negotiation continua DEBT-34d-rest se necessária
                    // (atomização ADR-0036).
                    let has_fr =
                        cols.iter().any(|t| matches!(t, TrackSizing::Fraction(_)));
                    let safe = if has_fr {
                        let num_auto_cols = cols
                            .iter()
                            .filter(|t| matches!(t, TrackSizing::Auto))
                            .count();
                        let num_fr_cols = cols
                            .iter()
                            .filter(|t| matches!(t, TrackSizing::Fraction(_)))
                            .count();
                        let safe_total = (available_width - total_fixed_w).max(0.0);
                        let total_tracks_concorrentes =
                            (num_auto_cols + num_fr_cols).max(1) as f64;
                        safe_total / total_tracks_concorrentes
                    } else {
                        (available_width - total_fixed_w).max(0.0)
                    };
                    let mut max_w = 0.0_f64;
                    for &ci in &cols_cells[i] {
                        let (w, _) = self.measure_content_constrained(&cells[ci], safe);
                        max_w = max_w.max(w);
                    }
                    resolved_widths[i] = max_w;
                    total_fixed_w += max_w;
                }
                TrackSizing::Fraction(fr) => {
                    total_fr_w += fr;
                }
            }
        }

        let remaining_w = (available_width - total_fixed_w).max(0.0);
        if total_fr_w > 0.0 {
            let per_fr = remaining_w / total_fr_w;
            for (i, sizing) in cols.iter().enumerate() {
                if let TrackSizing::Fraction(fr) = sizing {
                    resolved_widths[i] = fr * per_fr;
                }
            }
        }

        // X de cada coluna.
        let mut col_starts = vec![0.0_f64; num_cols];
        {
            let mut x = self.page_config.margin;
            for i in 0..num_cols {
                col_starts[i] = x;
                x += resolved_widths[i];
            }
        }

        // ── Particionar items em linhas ──────────────────────
        let rows_of_items: Vec<&[Content]> = cells.chunks(num_cols).collect();
        let num_rows_produced = rows_of_items.len();

        // ── Resolução de alturas (Passo 83): 3 passagens ─────
        // Fase 1 — Fixed e Auto numa travessia. Auto mede via
        // layout_sub_frame.
        //
        // P234 (B.2 consumer geometric): cache `cell_cache` removido
        // porque emissão pós-P234 itera `placed_cells` (não
        // `rows_of_items` direct) — placed cells reordenam input
        // (explicit Pass 1 + auto Pass 2). Cells re-medidas durante
        // emissão (custo perf ~2× aceitável MVP; cache reintegrável
        // futuro candidato indexado por input_idx).
        let mut row_heights: Vec<f64> = vec![0.0; num_rows_produced];
        let mut total_fixed_and_auto: f64 = 0.0;
        let mut fraction_indices: Vec<(usize, f64)> = Vec::new();

        for (row_idx, row_items) in rows_of_items.iter().enumerate() {
            let track = &row_tracks[row_idx % row_tracks.len()];
            match track {
                TrackSizing::Fixed(pt) => {
                    row_heights[row_idx] = *pt;
                    total_fixed_and_auto += *pt;
                }
                TrackSizing::Auto => {
                    let mut max_h = 0.0_f64;
                    for (col_idx, item) in row_items.iter().enumerate() {
                        if col_idx >= num_cols {
                            break;
                        }
                        let cell_w = resolved_widths[col_idx];
                        let cell_x = col_starts[col_idx];
                        // P772x — Fase 1 é medição pura (altura de linha);
                        // `_sub_items`/`_deco` descartados — a emissão real
                        // (com decoração, se aplicável) acontece na Fase 2
                        // (abaixo, `layout_sub_frame` em `cell_to_layout`).
                        // **P908** — `_orphaned_x`/`_orphaned_y` também
                        // descartados: os items desta passagem já são
                        // descartados, não há para onde propagar a
                        // correcção diferida (a Fase 2 repete a chamada e
                        // captura-a correctamente aí).
                        let (sub_h, _sub_items, _deco, _orphaned_x, _orphaned_y) = self
                            .layout_sub_frame(
                                item,
                                super::sub_frame::SubLayoutRegion {
                                    origin_x: cell_x,
                                    width: cell_w,
                                    height: None,
                                    align_rtl: true,
                                    unconstrained_height: true,
                                },
                            );
                        if sub_h > max_h {
                            max_h = sub_h;
                        }
                    }
                    row_heights[row_idx] = max_h;
                    total_fixed_and_auto += max_h;
                }
                TrackSizing::Fraction(fr) => {
                    fraction_indices.push((row_idx, *fr));
                }
            }
        }

        // Garantir linha limpa antes do Grid.
        self.flush_line();

        // P750/P761/P762 — quando a baseline inicial ainda está pendente,
        // `cursor_y` representa o topo da área disponível (margem) e
        // `ensure_initial_baseline()` ainda vai adicionar o offset do top-edge.
        // Nesse caso o Grid alinha-se directamente com esse topo. Quando
        // a baseline já foi fixada, `cursor_y` representa a baseline e o
        // topo do Grid é baseline − top-edge.
        if !self.initial_baseline_pending {
            let (top, _) = self.metrics.text_edges(self.style.size, &self.style);
            self.regions.current.cursor_y = Pt(self.regions.current.cursor_y.0 - top.0);
        }

        // Fase 1.5 — paginação ANTES da fase 2 de Fraction.
        // Se Fixed+Auto não cabe no resto da página actual mas cabe
        // numa página vazia, quebrar agora — assim cursor_y fica
        // estabilizado e a fase 2 calcula `fr` com o available_below
        // correcto. Se o Grid é maior que uma página inteira, aceita
        // overflow (não chama new_page() em loop).
        let space_left =
            f64::max(0.0, self.page_bottom_limit() - self.regions.current.cursor_y.0);
        if total_fixed_and_auto > space_left {
            let page_usable_height =
                self.regions.current.height - 2.0 * self.page_config.margin;
            if total_fixed_and_auto <= page_usable_height {
                self.new_page();
            }
        }

        // Fase 2 — resolver Fraction com cursor_y estabilizado.
        if !fraction_indices.is_empty() {
            let grid_top_y = self.regions.current.cursor_y.0;
            // **P904 (Item 1)** — sob `height: auto`, `page_bottom_limit()`
            // é `f64::INFINITY` (mesmo sentinel de P867 usado por outros
            // achados desta frente, P895-903); sem esta guarda,
            // `available_below`/`remaining_v` ficavam infinitos e cada
            // linha `fr` recebia `f64::INFINITY` de altura — não só
            // "diverge do vanilla", produzia uma `MediaBox` malformada
            // (`0 0 W inf`), mesma classe do crash original de P894/895.
            // Paridade vanilla confirmada por compilação directa
            // (`lab/typst-original`): `fr` numa página `auto` degenera a 0pt
            // — não há espaço "restante" para distribuir quando a altura é
            // indefinida. `f64::MAX` no lugar de `INFINITY` faz o ramo
            // `total_fixed_and_auto > available_below` abaixo nunca
            // disparar por engano (comparação com infinito real seria
            // sempre falsa para qualquer largura finita, o que já bastava,
            // mas usar um tecto finito explícito é mais robusto a somas
            // subsequentes que poderiam produzir `NaN` a partir de
            // `INFINITY - INFINITY` em casos futuros).
            let page_bottom = self.page_bottom_limit();
            let available_below = if page_bottom.is_finite() {
                f64::max(0.0, page_bottom - grid_top_y)
            } else {
                total_fixed_and_auto
            };
            if total_fixed_and_auto > available_below {
                // Caso patológico residual (Grid > página inteira):
                // não distribuir espaço negativo, atribuir 0pt aos fr.
                for (row_idx, _fr) in &fraction_indices {
                    row_heights[*row_idx] = 0.0;
                }
            } else {
                let remaining_v = available_below - total_fixed_and_auto;
                let total_fr: f64 = fraction_indices.iter().map(|(_, fr)| fr).sum();
                if total_fr > 0.0 {
                    for (row_idx, fr) in &fraction_indices {
                        row_heights[*row_idx] = remaining_v * (fr / total_fr);
                    }
                }
            }
        }

        // ── P234 — Placement algorítmico via place_cells (B.2). ───
        // `layout_grid` consumer geometric integration: itera
        // `Vec<PlacedCell>` em vez de `rows_of_items` chunks direct.
        // Cells com `colspan: None / rowspan: None` resolved como
        // `colspan: 1, rowspan: 1` em `PlacedCell` → comportamento
        // sequencial preservado paridade pré-P234. Cells com
        // colspan/rowspan > 1 ocupam bounds reais.
        //
        // Error path: place_cells retorna Err em conflito explicit/
        // explicit, colspan a exceder as colunas disponíveis ou coluna
        // inválida (mensagens vanilla, P822). P647: propagar como erro
        // de layout em vez de renderizar grid vazia em silêncio.
        let placed_cells: Vec<PlacedCell> = match place_cells(cells, num_cols) {
            Ok(placed) => placed,
            Err(diagnostics) => {
                self.layout_errors.extend(diagnostics);
                self.cell_align = saved_cell_align;
                return;
            }
        };

        // Derive num_rows_produced_final do placed (pode estender
        // além de rows_of_items.len() para cells explicit com y > N).
        let num_rows_from_placed =
            placed_cells.iter().map(|p| p.row + p.rowspan).max().unwrap_or(0);
        let num_rows_produced_final = num_rows_from_placed.max(num_rows_produced).max(1);

        // Pad row_heights se placed estende além de chunks-derived
        // (cells explicit com y maior ou cells com rowspan estendem
        // para rows não-covered por chunks). Extra rows: Fixed
        // resolved literal de row_tracks; Auto/Fraction = 0pt
        // (sem cells a medir → refino futuro candidato).
        while row_heights.len() < num_rows_produced_final {
            let row_idx = row_heights.len();
            let track = &row_tracks[row_idx % row_tracks.len()];
            let h = match track {
                TrackSizing::Fixed(pt) => *pt,
                _ => 0.0,
            };
            row_heights.push(h);
        }

        // Group placed cells por start row para per-row pagination
        // preservada (cells starting in row r emitidas conjunto
        // após pagination check de row r).
        let mut cells_per_row: Vec<Vec<usize>> = vec![vec![]; num_rows_produced_final];
        for (i, p) in placed_cells.iter().enumerate() {
            if p.row < num_rows_produced_final {
                cells_per_row[p.row].push(i);
            }
        }

        // ── Emissão linha a linha (P234 — placed cells iteration) ──
        let local_start_y = {
            let (ascender, _) =
                self.metrics.vertical_metrics(self.style.size, &self.style);
            ascender.0
        };

        // P512 — guarda o Y real de cada linha emitida (afectado por
        // paginação) para posicionar hlines/vlines depois das células.
        let mut emitted_row_starts = vec![0.0_f64; num_rows_produced_final];

        // P888 (achado 3 de P885, secção 7 de P887) — mapa (linha, coluna)
        // → índice em `placed_cells`, cobrindo colspan/rowspan (uma célula
        // ocupa todas as posições no seu range, não só a de início).
        // Permite resolver, para qualquer fronteira de linha vertical
        // (mesmo em linhas cobertas por rowspan, que não aparecem em
        // `cells_per_row`), qual célula está de cada lado — necessário
        // para a fusão de segmentos de borda em `emit_row_borders`.
        let mut grid_owner: Vec<Vec<Option<usize>>> =
            vec![vec![None; num_cols]; num_rows_produced_final];
        for (i, p) in placed_cells.iter().enumerate() {
            for r in p.row..(p.row + p.rowspan).min(num_rows_produced_final) {
                for c in p.col..(p.col + p.colspan).min(num_cols) {
                    grid_owner[r][c] = Some(i);
                }
            }
        }

        // P888 — segmentos verticais "abertos" (a acumular enquanto o
        // stroke resolvido numa fronteira de coluna se mantém igual ao
        // longo de linhas consecutivas). `None` = fronteira sem stroke
        // nesta linha ou ainda por iniciar. Descarregado (emitido) quando
        // o stroke muda, há quebra de página, ou no fim da função —
        // ver `emit_row_borders`/`flush_all_vsegments`.
        let mut open_vsegments: Vec<Option<(f64, f64, Stroke)>> = vec![None; num_cols + 1];

        for row_idx in 0..num_rows_produced_final {
            let row_h = row_heights[row_idx];

            // Quebra durante a emissão se a linha individual não cabe
            // (caso conservador: começar a linha no topo da página seguinte).
            // Se a linha for maior que uma página inteira, aceitar overflow.
            // P234 nota: cells com rowspan > 1 cruzando pagination =
            // out-of-scope (Categoria C.2 multi-region span futura).
            if self.regions.current.cursor_y.0 + row_h > self.page_bottom_limit() {
                let page_usable_height =
                    self.regions.current.height - 2.0 * self.page_config.margin;
                if row_h <= page_usable_height {
                    // P888 — uma linha vertical não pode atravessar uma
                    // quebra de página; descarregar tudo o que está aberto
                    // antes de mudar de página (fica na página que está a
                    // fechar), para a linha seguinte reabrir do zero.
                    flush_all_vsegments(
                        &mut open_vsegments,
                        &col_starts,
                        &resolved_widths,
                        num_cols,
                        &mut self.regions.current.current_items,
                    );
                    self.new_page();
                }
            }

            let row_start_y = self.regions.current.cursor_y.0;
            emitted_row_starts[row_idx] = row_start_y;

            for &placed_idx in &cells_per_row[row_idx] {
                let placed = &placed_cells[placed_idx];
                let cell = &placed.body;

                // P234 — bounds reais usando placed.col/colspan ×
                // resolved_widths + placed.row/rowspan × row_heights.
                let (cell_x, cell_y, cell_w, cell_h) = cell_bounds(
                    placed,
                    &col_starts,
                    &resolved_widths,
                    &row_heights,
                    row_start_y,
                );

                // P230 + P235 — extrair per-cell 5 fields (stroke + fill
                // cosméticos P230; align + inset + breakable algorítmicos
                // P235). Match em `placed.body` preserva GridCell wrapper
                // P234.
                let (cell_stroke, cell_fill, cell_align, cell_inset, _cell_breakable) =
                    match cell {
                        // Modelo D (Lote 12 P327): arm `|`-combinado GridCell+TableCell
                        // dividido — TableCell já é `Arc<…Elem>`, GridCell ainda struct
                        // (migra a seguir). Bindings com tipos distintos não partilham
                        // or-pattern; split permanente.
                        Content::TableCell(e) => (
                            e.stroke.as_ref(),
                            e.fill.as_ref(),
                            e.align.as_ref().copied(),
                            e.inset.as_ref(),
                            e.breakable.as_ref().copied(),
                        ),
                        Content::GridCell(e) => (
                            e.stroke.as_ref(),
                            e.fill.as_ref(),
                            e.align.as_ref().copied(),
                            e.inset.as_ref(),
                            e.breakable.as_ref().copied(),
                        ),
                        _ => (None, None, None, None, None),
                    };

                // Precedência `.or()` uniforme P230 + P232 + P235.
                let effective_stroke: Option<&Stroke> = cell_stroke.or(stroke);
                let effective_fill: Option<&Color> = cell_fill.or(fill);
                // P772j — fold por eixo (não `.or()` do Align2D inteiro):
                // confirmado contra o vanilla (`show_cell`/`resolve_cell` em
                // rules.rs/resolve.rs) e por medição directa
                // (`grid(align: horizon, grid.cell(align: left)[..])` — o
                // vanilla preserva o V herdado do grid quando a célula só
                // especifica H). `.or()` do Align2D inteiro descartava o
                // eixo do grid sempre que a célula especificava qualquer
                // eixo — divergência real, não só mecânica. Ver
                // 00_nucleo/prompts/compiler/layout.md §"Alinhamento efectivo
                // per-célula".
                let effective_align = match (cell_align, self.cell_align) {
                    (None, None) => None,
                    (Some(c), None) => Some(c),
                    (None, Some(g)) => Some(g),
                    (Some(c), Some(g)) => {
                        Some(Align2D { h: c.h.or(g.h), v: c.v.or(g.v) })
                    }
                };
                // P235 — inset per-cell override Grid-level; default Grid inset.
                let effective_inset: Sides<Length> = cell_inset.cloned().unwrap_or(inset);
                // P235 — breakable per-cell semantic adiada graded
                // (pattern "Field armazenado semantic adiada" N=7 → 8).

                // P235 — Cell-level cell_align save/restore (extensão P232
                // per-cell granularidade). Pattern emergente N=1 inaugurado.
                let saved_cell_align_inner = self.cell_align;
                self.cell_align = effective_align;

                // P235 — Inset bounds reduction: layout body em área
                // reduzida (left/top/right/bottom). Clamp a 0 para evitar
                // bounds negativos.
                let inset_l = effective_inset.left.abs.to_pt();
                let inset_t = effective_inset.top.abs.to_pt();
                let inset_r = effective_inset.right.abs.to_pt();
                let inset_b = effective_inset.bottom.abs.to_pt();
                let body_x = cell_x + inset_l;
                let body_y = cell_y + inset_t;
                let body_w = (cell_w - inset_l - inset_r).max(0.0);
                let body_h = (cell_h - inset_t - inset_b).max(0.0);

                // Definir o contexto de altura/origem da célula para
                // Content::Align (Passo 83) e Content::Place (Passo 84.6).
                // P235 — set ao body bounds reduzidos por inset.
                // P246 — cell_available_h + cell_origin_w migrados a
                // regions.cell (entity-side); cell_origin_x/y preservados
                // como Layouter fields legacy.
                let saved_cell_ox = self.cell_origin_x;
                let saved_cell_oy = self.cell_origin_y;
                let saved_cell_region = self
                    .regions
                    .enter_cell(crate::entities::region::Region::new(body_w, body_h));
                self.cell_origin_x = Some(body_x);
                self.cell_origin_y = Some(body_y);

                // P273.9 — save/restore parent_bbox paralelo a cell_origin_*
                // (Decisão 2α: bbox exacto cell body). Defaults rigorosos:
                // popular apenas se body_w/h > 0. Pattern DEBT-37 reused N=4.
                let saved_parent_bbox_p273_9 = self.parent_bbox;
                if body_w > 0.0 && body_h > 0.0 {
                    self.parent_bbox = Some(crate::entities::layout_types::Rect {
                        x: Pt(body_x),
                        y: Pt(body_y),
                        w: Pt(body_w),
                        h: Pt(body_h),
                    });
                }

                // P772j — alinhamento efectivo aplicado envolvendo o corpo
                // em `Content::Align` (nunca `Content::Place` — mecanismo
                // confirmado contra o vanilla, `show_cell` em
                // typst-layout/src/engine.rs, que faz `body.aligned(align)`
                // quando `align` é `Smart::Custom`, nunca `body.placed(..)`).
                // Reutiliza a disciplina de "consumidor absoluto" de P772g:
                // `layout_align`, invocado a partir daqui, lê
                // `self.regions.current.line_start_x` (=body_x, já definido
                // pela chamada de `layout_sub_frame` abaixo) e
                // `self.regions.cell` (Some, já definido por `enter_cell`
                // acima) — nenhuma alteração adicional a `placement.rs`.
                let cell_to_layout: Content = match effective_align {
                    Some(alignment) => Content::Align(std::sync::Arc::new(
                        crate::entities::elements::align::AlignElem {
                            alignment,
                            body: cell.clone(),
                        },
                    )),
                    None => cell.clone(),
                };

                // P234 — sem cache; re-medir cell (custo perf ~2× aceitável).
                // P235 — layout em body_x/body_w reduzidos por inset.
                let saved_cursor_x = self.regions.current.cursor_x;
                let saved_cursor_y = self.regions.current.cursor_y;
                let (cell_h_measured, cell_items, cell_deco, orphaned_align_x, orphaned_align_y) =
                    self.layout_sub_frame(
                        &cell_to_layout,
                        super::sub_frame::SubLayoutRegion {
                            origin_x: body_x,
                            width: body_w,
                            height: None,
                            align_rtl: true,
                            unconstrained_height: true,
                        },
                    );
                self.regions.current.cursor_x = saved_cursor_x;
                self.regions.current.cursor_y = saved_cursor_y;
                // **P772x** — traduzir segmentos de decoração da célula com a
                // MESMA translação aplicada a `cell_items` abaixo (x já
                // absoluto — `origin_x: body_x` — só y é rebaseado) e
                // reinserir no collector ambiente, se activo.
                if let Some(coll) = self.decoration_lines_collector.as_mut() {
                    for seg in &cell_deco {
                        coll.push(super::DecoSegment {
                            start_x: seg.start_x,
                            end_x: seg.end_x,
                            baseline_y: Pt(
                                body_y + (seg.baseline_y.val() - local_start_y)
                            ),
                        });
                    }
                }

                // P273.9 — restore parent_bbox (LIFO).
                self.parent_bbox = saved_parent_bbox_p273_9;

                // P246 — sair célula; restaurar legacy fields.
                self.regions.exit_cell(saved_cell_region);
                self.cell_origin_x = saved_cell_ox;
                self.cell_origin_y = saved_cell_oy;
                self.cell_align = saved_cell_align_inner;

                // P248 — TableCell overflow clip implícito: se cell body
                // ultrapassa o limite da célula (`body_h` populado via
                // `regions.cell.height` P246), emite items dentro de
                // `FrameItem::Group` com `clip_mask: Rect` (paridade
                // mecanismo P242). Row break real diferido per
                // Decisão 3 (refino futuro; DEBT-34e preservado aberto).
                let cell_overflow = cell_h_measured > body_h;

                // P228 + P230 + P234 — Z-order step 1: fill efectivo
                // emite primeiro (atrás do conteúdo cell + stroke).
                // Bounds reais cell_w/cell_h (cobrem colspan/rowspan).
                if let Some(c) = effective_fill {
                    self.regions.current.current_items.push(FrameItem::Shape {
                        pos: Point { x: Pt(cell_x), y: Pt(cell_y) },
                        kind: ShapeKind::Rect,
                        width: cell_w,
                        height: cell_h,
                        fill: Some(*c),
                        stroke: None,
                        parent_bbox_at_emit: None,
                    });
                }

                // Z-order step 2: conteúdo cell (existing P82-84.6 lógica).
                // Transferir items com posições absolutas (Y rebaseado
                // a body_y reduzido por inset P235, compensando o ascender_local).
                let translated_items: Vec<FrameItem> = cell_items
                    .into_iter()
                    .map(|item| {
                        let (lx, ly) = item_pos(&item);
                        let abs_pos =
                            Point { x: Pt(lx), y: Pt(body_y + (ly - local_start_y)) };
                        translate_frame_item(item, abs_pos.x, abs_pos.y)
                    })
                    .collect();
                // **P908** — rebasear as entradas órfãs devolvidas pelo
                // `layout_sub_frame` da célula (`Content::Align`/
                // `Content::Place` aninhado sob `width`/`height: auto`)
                // para ficarem relativas a `translated_items` (mesma
                // ordem/contagem de `cell_items` — só a posição de cada
                // item mudou, não o índice). X já vem absoluto
                // (`origin_x: body_x` na chamada acima) — sem deslocamento
                // extra. **Distinção física vs lógica (eixo Y)** — mesma
                // nota de `layout_align`: `origin_y`/`applied_y` são
                // quantidades LÓGICAS (sem ascender embutido), ao
                // contrário dos `FrameItem`s reais (cujo `ly` já é
                // baseline, por isso a translação real usa `body_y + (ly
                // - local_start_y)`); a translação lógica usa `body_y`
                // directamente — NÃO `body_y - local_start_y`. Ver
                // `00_nucleo/prompts/compiler/layout.md` §P908.
                let orphaned_align_y: Vec<_> = orphaned_align_y
                    .into_iter()
                    .map(|(path, count, align, content_h, origin_y, dy, applied_y)| {
                        (
                            path,
                            count,
                            align,
                            content_h,
                            origin_y + body_y,
                            dy,
                            applied_y + body_y,
                        )
                    })
                    .collect();
                if cell_overflow {
                    // P251 — row break TableCell cell-level γ-Items
                    // (slice items por body_h threshold + push tail
                    // ao buffer pending_cell_tails; flush no topo da
                    // próxima página via new_page).
                    //
                    // Preservação P248 clip implícito para rows
                    // TrackSizing::Fixed (paridade vanilla "Fixed
                    // rows clip overflow"); só Auto/Fraction usam
                    // row break.
                    let row_track = &row_tracks[placed.row % row_tracks.len()];
                    let is_fixed_row = matches!(row_track, TrackSizing::Fixed(_));
                    if is_fixed_row {
                        // **P908** — família "envolvimento em `Group`":
                        // `translated_items` (já absolutos) tornam-se
                        // `Group.items` sem alteração adicional — só
                        // `path` desce um nível (índice do `Group` na
                        // lista onde vai ser inserido, capturado ANTES do
                        // `push` abaixo).
                        let group_idx = self.regions.current.current_items.len();
                        for (mut path, count, align, content_w, origin_x, applied_x) in
                            orphaned_align_x
                        {
                            path.insert(0, group_idx);
                            self.pending_align_centering.push((
                                path, count, align, content_w, origin_x, applied_x,
                            ));
                        }
                        for (mut path, count, align, content_h, origin_y, dy, applied_y) in
                            orphaned_align_y
                        {
                            path.insert(0, group_idx);
                            self.pending_align_v_centering.push((
                                path, count, align, content_h, origin_y, dy, applied_y,
                            ));
                        }
                        // P248 preservado para Fixed rows.
                        self.regions.current.current_items.push(FrameItem::Group {
                            pos:          Point { x: Pt(body_x), y: Pt(body_y) },
                            matrix:       crate::entities::layout_types::TransformMatrix::identity(),
                            clip_mask:    Some(ShapeKind::Rect),
                            inner_width:  body_w,
                            inner_height: body_h,
                            items:        translated_items,
                        });
                    } else {
                        // P251 — γ-Items slice por threshold = body_y + body_h
                        // (em coordenadas absolutas; items já translated_).
                        let threshold = body_y + body_h;
                        // **P908** — família "diferido-cruzado": calcular a
                        // MESMA decisão head/tail de
                        // `slice_frame_items_at_height` por índice
                        // ORIGINAL, antes de consumir `translated_items`,
                        // para poder remapear os índices das entradas
                        // órfãs para a lista compactada correspondente
                        // (head OU tail — nunca as duas, cada partição tem
                        // a sua própria indexação a partir de 0).
                        let is_tail: Vec<bool> = translated_items
                            .iter()
                            .map(|item| {
                                super::slicing::item_y_start(item) >= threshold
                            })
                            .collect();
                        let compact_idx = |orig_idx: usize, want_tail: bool| -> usize {
                            is_tail[..orig_idx].iter().filter(|&&t| t == want_tail).count()
                        };
                        // Entrada cujo intervalo `[start, start+count)`
                        // atravessa a fronteira head/tail é descartada
                        // (degradação graciosa, caso raro — ver doc de
                        // `DeferredCellTail::orphaned_align_x`).
                        let split_orphaned = |entries: Vec<(
                            Vec<usize>,
                            usize,
                            crate::entities::layout_types::Align2D,
                            f64,
                            f64,
                            f64,
                        )>| {
                            let mut to_head = Vec::new();
                            let mut to_tail = Vec::new();
                            for (mut path, count, align, dim, origin, applied) in entries {
                                let start = path[0];
                                let range = start..start + count;
                                if range.end > is_tail.len() {
                                    continue;
                                }
                                let all_tail = is_tail[range.clone()].iter().all(|&t| t);
                                let all_head = is_tail[range].iter().all(|&t| !t);
                                if all_head {
                                    path[0] = compact_idx(start, false);
                                    to_head.push((path, count, align, dim, origin, applied));
                                } else if all_tail {
                                    path[0] = compact_idx(start, true);
                                    to_tail.push((
                                        path,
                                        count,
                                        align,
                                        dim,
                                        origin - threshold,
                                        applied - threshold,
                                    ));
                                }
                                // misto: descartado.
                            }
                            (to_head, to_tail)
                        };
                        let (head_align_x, tail_align_x) = split_orphaned(orphaned_align_x);
                        // eixo Y tem um campo extra (`dy`) — mesma lógica,
                        // duplicada por causa da forma da tupla.
                        let (head_align_y, tail_align_y): (
                            Vec<PendingAlignYEntry>,
                            Vec<PendingAlignYEntry>,
                        ) = {
                            let mut to_head = Vec::new();
                            let mut to_tail = Vec::new();
                            for (mut path, count, align, content_h, origin_y, dy, applied_y) in
                                orphaned_align_y
                            {
                                let start = path[0];
                                let range = start..start + count;
                                if range.end > is_tail.len() {
                                    continue;
                                }
                                let all_tail = is_tail[range.clone()].iter().all(|&t| t);
                                let all_head = is_tail[range].iter().all(|&t| !t);
                                if all_head {
                                    path[0] = compact_idx(start, false);
                                    to_head.push((
                                        path, count, align, content_h, origin_y, dy, applied_y,
                                    ));
                                } else if all_tail {
                                    path[0] = compact_idx(start, true);
                                    to_tail.push((
                                        path,
                                        count,
                                        align,
                                        content_h,
                                        origin_y - threshold,
                                        dy,
                                        applied_y - threshold,
                                    ));
                                }
                            }
                            (to_head, to_tail)
                        };

                        let (head_items, tail_items) =
                            crate::compiler::layout::slicing::slice_frame_items_at_height(
                                translated_items,
                                threshold,
                            );
                        let insertion_base = self.regions.current.current_items.len();
                        for (mut path, count, align, content_w, origin_x, applied_x) in
                            head_align_x
                        {
                            path[0] += insertion_base;
                            self.pending_align_centering.push((
                                path, count, align, content_w, origin_x, applied_x,
                            ));
                        }
                        for (mut path, count, align, content_h, origin_y, dy, applied_y) in
                            head_align_y
                        {
                            path[0] += insertion_base;
                            self.pending_align_v_centering.push((
                                path, count, align, content_h, origin_y, dy, applied_y,
                            ));
                        }
                        for item in head_items {
                            self.regions.current.current_items.push(item);
                        }
                        if !tail_items.is_empty() {
                            self.pending_cell_tails.push(
                                crate::compiler::layout::DeferredCellTail {
                                    items: tail_items,
                                    origin_x: body_x,
                                    width: body_w,
                                    fill: effective_fill.copied(),
                                    stroke: effective_stroke.cloned(),
                                    forwarded_count: 0,
                                    orphaned_align_x: tail_align_x,
                                    orphaned_align_y: tail_align_y,
                                },
                            );
                        }
                    }
                } else {
                    let insertion_base = self.regions.current.current_items.len();
                    for (mut path, count, align, content_w, origin_x, applied_x) in
                        orphaned_align_x
                    {
                        path[0] += insertion_base;
                        self.pending_align_centering.push((
                            path, count, align, content_w, origin_x, applied_x,
                        ));
                    }
                    for (mut path, count, align, content_h, origin_y, dy, applied_y) in
                        orphaned_align_y
                    {
                        path[0] += insertion_base;
                        self.pending_align_v_centering.push((
                            path, count, align, content_h, origin_y, dy, applied_y,
                        ));
                    }
                    for item in translated_items {
                        self.regions.current.current_items.push(item);
                    }
                }

            }

            // P888 (achado 3 de P885, secção 7 de P887) — substitui a
            // antiga "Opção β simplificada" (4 bordas por célula, sempre)
            // por fusão de segmentos partilhados: horizontal fundido
            // dentro da linha, vertical fundido entre linhas via
            // `open_vsegments`. Ver `emit_row_borders` (definição acima,
            // fora do `impl`) para a lógica completa e a nota de segurança
            // sobre `stroke` divergente por célula.
            {
                let mut row_cells_sorted: Vec<usize> = cells_per_row[row_idx].clone();
                row_cells_sorted.sort_by_key(|&idx| placed_cells[idx].col);
                emit_row_borders(
                    &mut self.regions.current.current_items,
                    row_idx,
                    row_start_y,
                    row_h,
                    &row_cells_sorted,
                    &placed_cells,
                    &grid_owner,
                    &mut open_vsegments,
                    &col_starts,
                    &resolved_widths,
                    num_cols,
                    stroke,
                );
            }

            // Avançar cursor para o fim da linha (altura conhecida).
            self.regions.current.cursor_y = Pt(row_start_y + row_h);
        }

        // P888 — descarregar quaisquer segmentos verticais ainda abertos
        // no fim da última linha (nenhuma quebra de página os fechou).
        flush_all_vsegments(
            &mut open_vsegments,
            &col_starts,
            &resolved_widths,
            num_cols,
            &mut self.regions.current.current_items,
        );

        // P512 — desenhar hlines/vlines por cima das células, usando os
        // Y reais emitidos (já consideraram paginação).
        if !hlines.is_empty() || !vlines.is_empty() {
            let grid_left = col_starts.first().copied().unwrap_or(0.0);
            let row_bottom = |r: usize| {
                emitted_row_starts.get(r).copied().unwrap_or(0.0)
                    + row_heights.get(r).copied().unwrap_or(0.0)
            };
            let col_right = |c: usize| {
                col_starts.get(c).copied().unwrap_or(0.0)
                    + resolved_widths.get(c).copied().unwrap_or(0.0)
            };

            for h in hlines {
                // **P739A** — `stroke: none` → linha não desenhada (paridade
                /// vanilla, medido; zero-thickness seria hairline em PDF).
                let Some(stroke) = h.stroke() else {
                    continue;
                };
                let row = h.row().min(num_rows_produced_final.saturating_sub(1));
                let y = match h.position() {
                    "bottom" => row_bottom(row),
                    _ => emitted_row_starts.get(row).copied().unwrap_or(0.0),
                };
                let start = h.start().min(num_cols);
                let end = h.end().map(|e| e.min(num_cols)).unwrap_or(num_cols);
                if start >= end {
                    continue;
                }
                let x0 = col_starts.get(start).copied().unwrap_or(grid_left);
                let x1 = col_right(end.saturating_sub(1));
                // P887 (achado 3 de P885, extensão) — `width` = `dx.abs()`,
                // não `0.0` (ver comentário equivalente nas bordas de
                // célula acima); sem isto o traço tem comprimento zero.
                self.regions.current.current_items.push(FrameItem::Shape {
                    pos: Point { x: Pt(x0), y: Pt(y) },
                    kind: ShapeKind::Line { dx: x1 - x0, dy: 0.0 },
                    width: (x1 - x0).abs(),
                    height: 0.0,
                    fill: None,
                    stroke: Some(stroke.clone()),
                    parent_bbox_at_emit: None,
                });
            }

            for v in vlines {
                let Some(stroke) = v.stroke() else { continue };
                let col = v.col().min(num_cols.saturating_sub(1));
                let x = match v.position() {
                    "right" => col_right(col),
                    _ => col_starts.get(col).copied().unwrap_or(grid_left),
                };
                let start = v.start().min(num_rows_produced_final);
                let end = v
                    .end()
                    .map(|e| e.min(num_rows_produced_final))
                    .unwrap_or(num_rows_produced_final);
                if start >= end {
                    continue;
                }
                let y0 = emitted_row_starts.get(start).copied().unwrap_or(0.0);
                let y1 = row_bottom(end.saturating_sub(1));
                // P887 (achado 3 de P885, extensão) — `height` = `dy.abs()`,
                // não `0.0` (mesma correcção do hline acima).
                self.regions.current.current_items.push(FrameItem::Shape {
                    pos: Point { x: Pt(x), y: Pt(y0) },
                    kind: ShapeKind::Line { dx: 0.0, dy: y1 - y0 },
                    width: 0.0,
                    height: (y1 - y0).abs(),
                    fill: None,
                    stroke: Some(stroke.clone()),
                    parent_bbox_at_emit: None,
                });
            }
        }

        // P232 — restore cell_align ao sair de Grid context (paridade
        // cell_origin_* save/restore pattern P84.6).
        self.cell_align = saved_cell_align;
    }
}

/// P234 — Bounds reais per `PlacedCell` × tracks resolved.
///
/// `current_row_start_y` é o cursor_y no início da emissão da row
/// `placed.row` (já considerou pagination). Para cells com
/// `rowspan > 1`, height = sum(row_heights[row..row+rowspan])
/// estende para baixo a partir de current_row_start_y.
fn cell_bounds(
    placed: &PlacedCell,
    col_starts: &[f64],
    resolved_widths: &[f64],
    row_heights: &[f64],
    current_row_start_y: f64,
) -> (f64, f64, f64, f64) {
    let x0 = col_starts.get(placed.col).copied().unwrap_or(0.0);
    let y0 = current_row_start_y;
    let cell_w: f64 = (placed.col..placed.col + placed.colspan)
        .map(|i| resolved_widths.get(i).copied().unwrap_or(0.0))
        .sum();
    let cell_h: f64 = (placed.row..placed.row + placed.rowspan)
        .map(|i| row_heights.get(i).copied().unwrap_or(0.0))
        .sum();
    (x0, y0, cell_w, cell_h)
}

/// P888 (achado 3 de P885, secção 7 de P887) — extrai o `stroke` de uma
/// célula (`TableCell`/`GridCell`), com fallback para o `grid_stroke`
/// (default do `table()`/`grid()`). Mesmo padrão de precedência já usado
/// inline no loop principal (`cell_stroke.or(stroke)`), extraído para
/// reuso pela fusão de segmentos de borda (`generate_row_border_segments`/
/// `update_vline_segments`, abaixo).
fn cell_effective_stroke(cell: &Content, grid_stroke: Option<&Stroke>) -> Option<Stroke> {
    let cell_stroke = match cell {
        Content::TableCell(e) => e.stroke.as_ref(),
        Content::GridCell(e) => e.stroke.as_ref(),
        _ => None,
    };
    cell_stroke.or(grid_stroke).cloned()
}

/// P888 — coordenada X da posição de linha vertical `x_idx` (0..=num_cols).
fn vline_x(x_idx: usize, col_starts: &[f64], resolved_widths: &[f64], num_cols: usize) -> f64 {
    if x_idx < num_cols {
        col_starts.get(x_idx).copied().unwrap_or(0.0)
    } else {
        col_starts.get(num_cols.saturating_sub(1)).copied().unwrap_or(0.0)
            + resolved_widths.get(num_cols.saturating_sub(1)).copied().unwrap_or(0.0)
    }
}

/// P888 — emite um segmento horizontal (borda de topo/fundo de uma
/// célula, ou run fundido de várias) se tiver comprimento > 0.
fn emit_hsegment(items: &mut Vec<FrameItem>, x0: f64, x1: f64, y: f64, stroke: Stroke) {
    if x1 > x0 {
        items.push(FrameItem::Shape {
            pos: Point { x: Pt(x0), y: Pt(y) },
            kind: ShapeKind::Line { dx: x1 - x0, dy: 0.0 },
            width: (x1 - x0).abs(),
            height: 0.0,
            fill: None,
            stroke: Some(stroke),
            parent_bbox_at_emit: None,
        });
    }
}

/// P888 — emite um segmento vertical (borda de coluna, possivelmente
/// fundida ao longo de várias linhas) se tiver comprimento > 0.
fn emit_vsegment(items: &mut Vec<FrameItem>, x: f64, y0: f64, y1: f64, stroke: Stroke) {
    if y1 > y0 {
        items.push(FrameItem::Shape {
            pos: Point { x: Pt(x), y: Pt(y0) },
            kind: ShapeKind::Line { dx: 0.0, dy: y1 - y0 },
            width: 0.0,
            height: (y1 - y0).abs(),
            fill: None,
            stroke: Some(stroke),
            parent_bbox_at_emit: None,
        });
    }
}

/// P888 — descarrega (emite) todos os segmentos verticais abertos em
/// `open`. Chamado antes de uma quebra de página (uma linha vertical não
/// pode atravessar páginas) e no fim da emissão do grid/table.
fn flush_all_vsegments(
    open: &mut [Option<(f64, f64, Stroke)>],
    col_starts: &[f64],
    resolved_widths: &[f64],
    num_cols: usize,
    items: &mut Vec<FrameItem>,
) {
    for (x_idx, slot) in open.iter_mut().enumerate() {
        if let Some((y0, y1, stroke)) = slot.take() {
            let x = vline_x(x_idx, col_starts, resolved_widths, num_cols);
            emit_vsegment(items, x, y0, y1, stroke);
        }
    }
}

/// P888 — para a linha `row_idx`, emite as bordas de topo/fundo fundidas
/// (runs contíguos de células com o mesmo `effective_stroke`) e actualiza
/// `open_vsegments` com a continuação/quebra dos segmentos verticais.
///
/// Substitui a antiga "Opção β simplificada" (4 `FrameItem::Shape::Line`
/// por célula, sempre) por fusão: bordas horizontais fundem dentro da
/// própria linha (sem estado entre linhas — cada linha é uma unidade
/// fechada); bordas verticais fundem entre linhas via `open_vsegments`,
/// que persiste ao longo da chamada a `layout_grid` e é descarregado em
/// quebras de stroke, quebras de página, ou no fim da função.
///
/// **Segurança com `stroke` por célula divergente** (`table.cell(stroke:
/// ..)`/`grid.cell(stroke: ..)`, confirmado activo em `grid.rs` — ver
/// `typst-passo-888-relatorio.md` secção 1): quando os dois lados de uma
/// fronteira (horizontal ou vertical) têm `effective_stroke` diferentes,
/// **não funde** — em vez de tentar decidir um vencedor (como o sistema de
/// prioridade do vanilla, `lines.rs::StrokePriority`, fora de âmbito desta
/// correcção per ADR-0107), desenha os dois lados separadamente, tal como
/// o comportamento anterior a este passo — sem risco de regressão visual.
#[allow(clippy::too_many_arguments)]
fn emit_row_borders(
    items: &mut Vec<FrameItem>,
    row_idx: usize,
    row_start_y: f64,
    row_h: f64,
    row_cells_sorted: &[usize],
    placed_cells: &[PlacedCell],
    grid_owner: &[Vec<Option<usize>>],
    open_vsegments: &mut [Option<(f64, f64, Stroke)>],
    col_starts: &[f64],
    resolved_widths: &[f64],
    num_cols: usize,
    grid_stroke: Option<&Stroke>,
) {
    // ── Bordas horizontais (topo + fundo), fundidas dentro da linha ──
    let mut top_run: Option<(f64, f64, Stroke)> = None; // (x0, x1, stroke)
    let mut bottom_run: Option<(f64, f64, Stroke)> = None;
    for &placed_idx in row_cells_sorted {
        let placed = &placed_cells[placed_idx];
        // Largura real (cobre colspan) directamente de `col_starts`/
        // `resolved_widths` — não usa `cell_bounds` aqui porque a altura
        // desta linha (`row_h`) já vem resolvida do caller (que soma
        // `row_heights[row..row+rowspan]` para linhas com rowspan); só a
        // posição/largura X é necessária para as bordas horizontais.
        let cx = col_starts.get(placed.col).copied().unwrap_or(0.0);
        let cw: f64 = (placed.col..placed.col + placed.colspan)
            .map(|i| resolved_widths.get(i).copied().unwrap_or(0.0))
            .sum();
        let stroke = cell_effective_stroke(&placed.body, grid_stroke);

        match (&top_run, &stroke) {
            (Some((rx0, rx1, rs)), Some(s)) if rs == s && (*rx1 - cx).abs() < 1e-6 => {
                top_run = Some((*rx0, cx + cw, rs.clone()));
            }
            _ => {
                if let Some((rx0, rx1, rs)) = top_run.take() {
                    emit_hsegment(items, rx0, rx1, row_start_y, rs);
                }
                top_run = stroke.clone().map(|s| (cx, cx + cw, s));
            }
        }
        match (&bottom_run, &stroke) {
            (Some((rx0, rx1, rs)), Some(s)) if rs == s && (*rx1 - cx).abs() < 1e-6 => {
                bottom_run = Some((*rx0, cx + cw, rs.clone()));
            }
            _ => {
                if let Some((rx0, rx1, rs)) = bottom_run.take() {
                    emit_hsegment(items, rx0, rx1, row_start_y + row_h, rs);
                }
                bottom_run = stroke.map(|s| (cx, cx + cw, s));
            }
        }
    }
    if let Some((rx0, rx1, rs)) = top_run {
        emit_hsegment(items, rx0, rx1, row_start_y, rs);
    }
    if let Some((rx0, rx1, rs)) = bottom_run {
        emit_hsegment(items, rx0, rx1, row_start_y + row_h, rs);
    }

    // ── Bordas verticais, fundidas entre linhas via `open_vsegments` ──
    let row_end_y = row_start_y + row_h;
    for x_idx in 0..=num_cols {
        let left_owner = if x_idx > 0 { grid_owner[row_idx][x_idx - 1] } else { None };
        let right_owner = if x_idx < num_cols { grid_owner[row_idx][x_idx] } else { None };

        // Interior de colspan (as duas posições pertencem à mesma célula):
        // não desenhar linha aqui (paridade vanilla, `lines.rs::
        // vline_stroke_at_row`, "returns None" quando cruza colspan).
        let resolved: Option<Stroke> = if left_owner.is_some() && left_owner == right_owner {
            None
        } else {
            let left_stroke =
                left_owner.and_then(|i| cell_effective_stroke(&placed_cells[i].body, grid_stroke));
            let right_stroke = right_owner
                .and_then(|i| cell_effective_stroke(&placed_cells[i].body, grid_stroke));
            match (left_stroke, right_stroke) {
                (Some(l), Some(r)) if l == r => Some(l),
                (Some(l), Some(r)) => {
                    // Strokes divergentes dos dois lados: não funde — desenha
                    // os dois directamente para esta linha (comportamento
                    // seguro, idêntico ao pré-P888 nesta posição específica).
                    let x = vline_x(x_idx, col_starts, resolved_widths, num_cols);
                    emit_vsegment(items, x, row_start_y, row_end_y, l);
                    emit_vsegment(items, x, row_start_y, row_end_y, r);
                    None
                }
                (Some(l), None) => Some(l),
                (None, Some(r)) => Some(r),
                (None, None) => None,
            }
        };

        let continues = matches!(
            (&open_vsegments[x_idx], &resolved),
            (Some((_, _, open_stroke)), Some(r)) if open_stroke == r
        );
        if continues {
            if let Some(slot) = open_vsegments[x_idx].as_mut() {
                slot.1 = row_end_y;
            }
        } else {
            if let Some((y0, y1, s)) = open_vsegments[x_idx].take() {
                let x = vline_x(x_idx, col_starts, resolved_widths, num_cols);
                emit_vsegment(items, x, y0, y1, s);
            }
            open_vsegments[x_idx] = resolved.map(|s| (row_start_y, row_end_y, s));
        }
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
