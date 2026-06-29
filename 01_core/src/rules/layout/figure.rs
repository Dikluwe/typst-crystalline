//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout_figure.md
//! @prompt-hash 129fae12
//! @layer L1
//! @updated 2026-06-26

use crate::entities::content::Content;
use crate::entities::counter_format::format_counter;
use crate::entities::elements::figure::FigureElem;
use crate::entities::introspector::Introspector;
use crate::entities::value::Value;
use crate::rules::lang::figure_supplement::figure_supplement_for_lang;

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de `figure(...)` (atomização ADR-0109 P378): calcula o prefixo de
/// numeração (gate na chain, número via Introspector) e delega o desenho a
/// `layout_figure`. Content-preserving — era inline no `layout_content`.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e:        &FigureElem,
) {
    let (body, caption, kind) = (&e.body, &e.caption, &e.kind);
    // F-5a de-bake (P365, §3a.9): o gate (padrão presente/ausente) vive
    // **só na chain** (`custom("figure.numbering")`, transportado por
    // `Content::Styled`); lido de `layouter.chain`. O pattern define o
    // formato do número (subset "1.", "I.", "(a)", "A." via `format_counter`).
    let numbering_pattern = layouter
        .chain
        .custom("figure.numbering")
        .and_then(|v| match v {
            Value::Str(s) => Some(s.as_str()),
            _ => None,
        });
    // Calcular o prefixo de numeração antes de chamar layout_figure.
    let caption_prefix: Option<String> = if numbering_pattern.is_some() {
        let kind_key = kind.as_deref().unwrap_or("image");
        let progress = layouter.figure_progress.entry(kind_key.to_string()).or_insert(0);
        let idx = *progress;
        *progress += 1;
        // P190H (M6 categoria Figures): fallback legacy
        // `state.figure_numbers` ELIMINADO. Caminho Introspector activo
        // via `figure_number_at_index` (P184C/D); rede de segurança final
        // `unwrap_or(idx + 1)` preservada (heurística para edge cases).
        let figure_number = layouter
            .introspector
            .figure_number_at_index(kind_key, idx)
            .unwrap_or(idx + 1);
        // P488 — regista página actual para LoF carry-forward entre iterações fixpoint.
        let page = layouter.current_page_number();
        layouter.runtime.figure_page_numbers.push(page);
        let formatted = numbering_pattern
            .and_then(|pat| format_counter(&[figure_number], pat))
            .unwrap_or_else(|| figure_number.to_string());
        let supplement = figure_supplement_for_lang(kind_key, layouter.chain.lang().as_ref());
        Some(format!("{} {}: ", supplement, formatted))
    } else {
        None
    };
    layout_figure(layouter, body, caption, caption_prefix);
}

/// Renderiza uma figura com legenda opcional (Passo 62/75, DEBT-14/15).
///
/// `caption_prefix`: prefixo de numeração pré-calculado pelo layouter
/// (ex: "Figura 1: "), ou `None` se a figura não tiver numeração activa.
/// O cálculo acontece em `layout_content` usando `figure_progress` e
/// `counter_state.figure_numbers` — a introspecção pré-computou os números.
pub(super) fn layout_figure<M: FontMetrics, S: ImageSizer>(
    layouter:       &mut Layouter<M, S>,
    body:           &Content,
    caption:        &Option<Content>,
    caption_prefix: Option<String>,
) {
    // 1. Desenhar o corpo da figura.
    layouter.layout_content(body);

    // 2. Desenhar a legenda, se existir.
    if let Some(cap) = caption {
        layouter.layout_content(&Content::linebreak());

        if let Some(prefix) = caption_prefix {
            let caption_block = Content::Sequence(
                vec![
                    Content::text(prefix),
                    cap.clone(),
                ]
                .into(),
            );
            layouter.layout_content(&caption_block);
        } else {
            layouter.layout_content(cap);
        }
    }
}
