//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/text.md
//! @prompt-hash d9a75b4e
//! @layer L1
//! @updated 2026-09-30
//!
//! Atomização (ADR-0109, P381): o layout de `Text` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Folha de render —
//! decodifica o `#set text`/`#set par` da chain, faz merge top-wins com o
//! estilo tipado, e dispõe cada palavra via `layout_word`. **Não re-entra
//! `layout_content`** (não orquestra). Caminho quente — content-preserving.

use crate::entities::layout_types::{Pt, TextStyle};

mod style;
pub(super) fn effective_style<M: FontMetrics, S: ImageSizer>(
    layouter: &Layouter<M, S>,
) -> TextStyle {
    style::effective_style(layouter)
}

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de um nó `Text`: resolve o estilo efectivo (chain tipada vence o
/// render `#set` decodificado da chain `custom`) e dispõe as palavras.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    text: &str,
) {
    let original = layouter.bidi_cursor.as_mut().and_then(|c| c.next());
    super::paragraph_indent::start(layouter);
    let previous_span = layouter.text_bidi.take();
    if text.is_empty() {
        let effective = effective_style(layouter);
        let prev_style = std::mem::replace(&mut layouter.style, effective);
        super::text_baseline::admit(layouter);
        super::bibliography::emit_pending_prefix(layouter);
        layouter.regions.current.current_line.push(
            crate::entities::layout_types::FrameItem::Text {
                pos: crate::entities::layout_types::Point {
                    x: layouter.regions.current.cursor_x,
                    y: layouter.baseline_y(),
                },
                text: "".into(),
                style: layouter.style.clone(),
                bidi: original,
            },
        );
        layouter.style = prev_style;
        layouter.text_bidi = previous_span;
        return;
    }
    if original.is_none()
        && !layouter.paragraph_planning_suppressed
        && super::paragraph_plan::try_layout(
            layouter,
            &crate::entities::content::Content::text(text),
        )
    {
        return;
    }
    let effective = effective_style(layouter);

    let prev_style = layouter.style.clone();
    layouter.style = effective;

    // **P544/P547** — preservar espaços entre palavras dentro de um nó Text,
    // incluindo espaços iniciais/finais e nós Text compostos só por espaços.
    // `split_whitespace()` perdia esses separadores, juntando palavras e
    // removendo o espaço antes de pontuação. `split(' ')` mantém cada
    // posição de espaço como um segmento vazio; adicionamos `space_width()`
    // entre segmentos e para cada segmento vazio.
    let mut offset = 0usize;
    let mut parts = text.split(' ').peekable();
    while let Some(part) = parts.next() {
        layouter.text_bidi =
            original.as_ref().and_then(|s| s.subspan(offset..offset + part.len()));
        if !part.is_empty() {
            if layouter.smallcaps {
                const SCALE: f64 = 0.8;
                let chars: Vec<char> = part.chars().collect();
                let mut i = 0;
                while i < chars.len() {
                    let is_lower = chars[i].is_lowercase();
                    let mut j = i;
                    while j < chars.len() && chars[j].is_lowercase() == is_lower {
                        j += 1;
                    }
                    let run: String = chars[i..j].iter().collect();
                    if is_lower {
                        let upper = run.to_uppercase();
                        let base_size = layouter.style.size;
                        layouter.style.size = Pt(base_size.0 * SCALE);
                        layouter.layout_chunk(&upper);
                        layouter.style.size = base_size;
                    } else {
                        layouter.layout_chunk(&run);
                    }
                    i = j;
                }
            } else {
                layouter.layout_word(part);
            }
        }
        offset += part.len();
        if parts.peek().is_some() {
            layouter.text_bidi =
                original.as_ref().and_then(|s| s.subspan(offset..offset + 1));
            offset += 1;
            super::space::advance(
                layouter,
                crate::entities::layout_types::TextSeparatorOrigin::Literal,
            );
        }
    }
    layouter.style = prev_style;
    layouter.text_bidi = previous_span;
}

#[cfg(test)]
mod empty_text_geometry_tests {
    use super::super::{FixedMetrics, FontMetrics};
    use super::*;
    use crate::entities::{
        content::Content,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{PageMargins, Pt},
    };
    use comemo::Track;

    #[test]
    fn explicit_empty_text_establishes_typographic_height_without_ink() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            12.0,
            (&intr as &dyn Introspector).track(),
        );
        l.page_config.width = f64::INFINITY;
        l.page_config.height = f64::INFINITY;
        l.page_config.margin = PageMargins::uniform(0.0);
        l.regions.current.width = f64::INFINITY;
        l.regions.current.cursor_x = Pt(0.0);
        l.regions.current.line_start_x = Pt(0.0);
        l.regions.current.cursor_y = Pt(0.0);
        let style = effective_style(&l);
        let top = l.metrics.text_edges(style.size, &style).0 .0;
        l.layout_content(&Content::text(""));
        let page = &l.finish().pages[0];
        assert!((page.height - top).abs() < 1e-9, "height={} top={}", page.height, top);
        assert!(page.plain_text().is_empty());
    }
}
