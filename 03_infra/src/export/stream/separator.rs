//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/stream/separator.md
//! @prompt-hash 629b00dd
//! @layer L3
//! @updated 2026-09-28

use super::{
    emit_text_pdf, emit_text_pdf_deferred, emit_text_pdf_verbose,
    emit_text_pdf_verbose_deferred, PageContext, StreamMode,
};
use typst_core::entities::{
    bidi_context::{BidiSpan, ParagraphBidiSegment},
    dir::Dir,
    layout_types::{TextSeparatorOrigin, TextStyle},
};

/// A source-certified terminal space closes the PDF's LTR embedding.
fn closes_ltr_embedding(origin: TextSeparatorOrigin, span: Option<&BidiSpan>) -> bool {
    if origin != TextSeparatorOrigin::Literal {
        return false;
    }
    let Some(span) = span else { return false };
    if span.input().base_dir() != Dir::RTL {
        return false;
    }
    let Some(ParagraphBidiSegment::Text { text, dir: Dir::LTR }) =
        span.input().segments().get(span.segment())
    else {
        return false;
    };
    let range = span.range();
    range.start > 0 && range.end == text.len() && text.get(range) == Some(" ")
}

pub(super) fn emit(
    ops: &mut String,
    x: f64,
    pdf_y: f64,
    style: &TextStyle,
    origin: TextSeparatorOrigin,
    span: Option<&BidiSpan>,
    ctx: &PageContext,
) {
    if closes_ltr_embedding(origin, span) {
        // PDF direction formatting stays outside authored carriers and ToUnicode.
        super::text_replacement::opening(ops, " \u{202c}");
        let restore = match ctx.mode {
            StreamMode::Compact => {
                emit_text_pdf_deferred(ops, x, pdf_y, " ", style, &ctx.font_scenario)
            }
            StreamMode::Verbose => emit_text_pdf_verbose_deferred(
                ops,
                x,
                pdf_y,
                " ",
                style,
                &ctx.font_scenario,
            ),
        };
        super::text_replacement::closing(ops);
        if restore {
            ops.push_str("Q\n");
        }
    } else {
        match ctx.mode {
            StreamMode::Compact => {
                emit_text_pdf(ops, x, pdf_y, " ", style, &ctx.font_scenario)
            }
            StreamMode::Verbose => {
                emit_text_pdf_verbose(ops, x, pdf_y, " ", style, &ctx.font_scenario)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::HashMap, sync::Arc};
    use typst_core::entities::{
        bidi_context::{BidiSpan, ParagraphBidiInput, ParagraphBidiSegment},
        dir::Dir,
        layout_types::{Pt, TextSeparatorOrigin},
    };

    #[test]
    fn terminal_literal_space_retains_export_direction_boundary() {
        let input = Arc::new(
            ParagraphBidiInput::new(
                Dir::RTL,
                vec![ParagraphBidiSegment::Text { text: "ABC ".into(), dir: Dir::LTR }],
            )
            .unwrap(),
        );
        let span = BidiSpan::new(input.clone(), 0, 3..4).unwrap();
        let middle = BidiSpan::new(
            Arc::new(
                ParagraphBidiInput::new(
                    Dir::RTL,
                    vec![ParagraphBidiSegment::Text {
                        text: "A B".into(),
                        dir: Dir::LTR,
                    }],
                )
                .unwrap(),
            ),
            0,
            1..2,
        )
        .unwrap();
        let same_dir = BidiSpan::new(
            Arc::new(
                ParagraphBidiInput::new(Dir::LTR, input.segments().to_vec()).unwrap(),
            ),
            0,
            3..4,
        )
        .unwrap();
        for mode in [StreamMode::Compact, StreamMode::Verbose] {
            let a = HashMap::new();
            let b = HashMap::new();
            let ctx = PageContext::type1(&a, &[], &b, &[], mode);
            let mut style = TextStyle::regular(Pt(11.0));
            style.weight = Some(700);
            let mut ops = String::new();
            emit(
                &mut ops,
                10.0,
                20.0,
                &style,
                TextSeparatorOrigin::Literal,
                Some(&span),
                &ctx,
            );
            assert!(ops.contains("/ActualText <FEFF0020202C>"), "{ops}");
            if let Some(q) = ops.rfind("Q\n") {
                assert!(ops.rfind("EMC\n").unwrap() < q, "{ops}");
            }
            for (origin, span) in [
                (TextSeparatorOrigin::Markup, Some(&span)),
                (TextSeparatorOrigin::Literal, None),
                (TextSeparatorOrigin::Literal, Some(&middle)),
                (TextSeparatorOrigin::Literal, Some(&same_dir)),
            ] {
                let mut ops = String::new();
                emit(&mut ops, 10.0, 20.0, &style, origin, span, &ctx);
                assert!(!ops.contains("/ActualText"));
            }
        }
    }
}
