//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout_outline.md
//! @prompt-hash f60186de
//! @layer L1
//! @updated 2026-06-26
//!
//! P457: Gera a Tabela de Conteúdos visual respeitando os parâmetros do
//! `OutlineElem` (title, depth, indent).
//!
//! **P472** — `OutlineTarget::Figures` e `OutlineTarget::Tables` geram
//! List of Figures e List of Tables respectivamente.

use crate::entities::content::Content;
use crate::entities::elements::outline::{OutlineElem, OutlineTarget};
use crate::entities::introspector::Introspector;

use super::{FontMetrics, ImageSizer, Layouter};

/// Gera a Tabela de Conteúdos visual.
///
/// Lê `headings_for_toc` e `label_pages` do estado injectado pela introspecção.
/// Usa Content clonado para preservar formatação dos títulos.
///
/// Motor de congelamento (DEBT-13): `is_readonly = true` durante o layout de
/// cada linha impede que CounterUpdate embebido no clone avance contadores.
///
/// Números de página (DEBT-12): lidos de `label_pages` se disponíveis.
/// Na Passagem 2 (draft) estará vazio — TOC sem números.
/// Na Passagem 3 (final) terá os dados reais — TOC com páginas correctas.
pub(super) fn layout_outline<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e:        &OutlineElem,
) {
    match e.target {
        OutlineTarget::Figures => { layout_lof(layouter, e); return; }
        OutlineTarget::Tables  => { layout_lot(layouter, e); return; }
        OutlineTarget::Headings => {}
    }
    // P200B (M5 universal completo) — caminho Introspector activo via
    // Tag::HeadingForToc pós-recursão (3ª Tag emitida pelo walk arm
    // Heading); sub-store `intr.headings_for_toc` populated via
    // populate_intr_from_tag_start arm HeadingForToc.
    //
    // P190G (M6 categoria Labels & TOC): fallback legacy
    // `counter.headings_for_toc` ELIMINADO — field eliminado de
    // CounterStateLegacy. P184D / P194B substitution-with-fallback
    // colapsa em Introspector path puro.
    //
    // Clonar o vector antes do loop para evitar borrow duplo de `layouter`.
    let entries: Vec<(_, _, _, _)> = layouter.introspector.headings_for_toc().to_vec();

    // Título da TOC — default "Índice" se nenhum título fornecido.
    let title_content = e
        .title
        .clone()
        .unwrap_or_else(|| Content::text("Índice"));
    layouter.layout_content(&Content::heading(1, title_content));

    for (label, number, body_content, level) in entries {
        // P457: respeitar profundidade configurada.
        if level > e.depth {
            continue;
        }

        let indent = if e.indent {
            "  ".repeat(level.saturating_sub(1))
        } else {
            String::new()
        };

        // Ler página ANTES de activar is_readonly — evita borrow duplo.
        // Na iteração 0, known_page_numbers está vazio → string vazia.
        // Nas iterações seguintes, known_page_numbers tem os dados → "  N".
        // P190C (M6 categoria Page tracking): known_page_numbers movido
        // para LayouterRuntimeState.
        let page_num = layouter
            .runtime
            .known_page_numbers
            .get(&label)
            .map(|p| format!("  {}", p))
            .unwrap_or_default();

        // P428 (DEBT-60b): o número do outline vem do campo `number`, não de
        // `resolved_labels` (que incluiria o supplement "Secção"). Para
        // headings sem numeração, `number` é `None` e a linha começa
        // directamente com o body.
        let prefix = number.unwrap_or_default();
        let prefix_with_space = if prefix.is_empty() {
            String::new()
        } else {
            format!("{} ", prefix)
        };

        let line = Content::Sequence(
            vec![
                Content::text(indent),
                Content::text(prefix_with_space),
                body_content, // Content clonado — preserva formatação original
                Content::text(page_num),
                Content::linebreak(),
            ]
            .into(),
        );

        // Activar is_readonly antes do layout para bloquear CounterUpdate/step
        // durante a renderização do clone (DEBT-13).
        // P190D (M6 categoria Document metadata): is_readonly movido
        // para LayouterRuntimeState (Layouter-runtime — não derivado
        // de Content pre-pass). Guard movido de
        // CounterStateLegacy::step_* para
        // counters::layout_counter_update.
        layouter.runtime.is_readonly = true;
        layouter.layout_content(&line);
        // Restaurar DEPOIS do layout — a protecção deve cobrir toda a execução.
        layouter.runtime.is_readonly = false;
    }
}

/// **P472/P488** — List of Figures: itera `figures_for_lof` do Introspector e
/// emite uma linha por figura com o número, caption e page number (P488).
/// P488: page numbers via `runtime.known_figure_page_numbers` (carry-forward
/// do fixpoint). Na iteração 0 o Vec está vazio → linha sem ". . . N".
fn layout_lof<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e:        &OutlineElem,
) {
    let title_content = e
        .title
        .clone()
        .unwrap_or_else(|| Content::text("List of Figures"));
    layouter.layout_content(&Content::heading(1, title_content));

    let entries: Vec<(usize, String)> = layouter.introspector.figures_for_lof().to_vec();
    // P488 — match posicional: ambas as fontes seguem ordem de documento.
    let known_pages = layouter.runtime.known_figure_page_numbers.clone();
    for (i, (num, caption)) in entries.iter().enumerate() {
        let page_num = known_pages.get(i).copied().unwrap_or(0);
        let line = if page_num > 0 {
            format!("Figure {}  {} . . . {}", num, caption, page_num)
        } else {
            format!("Figure {}  {}", num, caption)
        };
        layouter.layout_content(&Content::text(line));
        layouter.flush_line();
    }
}

/// **P472/P488** — List of Tables: itera `tables_for_lot` do Introspector e
/// emite uma linha por tabela com o número, caption e page number (P488).
fn layout_lot<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e:        &OutlineElem,
) {
    let title_content = e
        .title
        .clone()
        .unwrap_or_else(|| Content::text("List of Tables"));
    layouter.layout_content(&Content::heading(1, title_content));

    let entries: Vec<(usize, String)> = layouter.introspector.tables_for_lot().to_vec();
    // P488 — match posicional: ambas as fontes seguem ordem de documento.
    let known_pages = layouter.runtime.known_table_page_numbers.clone();
    for (i, (num, caption)) in entries.iter().enumerate() {
        let page_num = known_pages.get(i).copied().unwrap_or(0);
        let line = if page_num > 0 {
            format!("Table {}  {} . . . {}", num, caption, page_num)
        } else {
            format!("Table {}  {}", num, caption)
        };
        layouter.layout_content(&Content::text(line));
        layouter.flush_line();
    }
}
