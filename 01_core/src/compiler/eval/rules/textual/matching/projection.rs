//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/rules/textual/matching/projection.md
//! @prompt-hash c70d495e
//! @layer L1
//! @updated 2026-09-30
//!
//! Projeção de faixas originais e achatamento de substituições.

use crate::entities::content::Content;

use super::textual_piece;

/// Reconstitui uma fatia não casada nos nós que a originaram. Espaços
/// explícitos e inferidos fora do match conservam a variante e o estilo;
/// somente um Text parcialmente atravessado vira Text fatiado.
pub(super) fn project_original_range(
    source: &[Content],
    from: usize,
    until: usize,
    out: &mut Vec<Content>,
) {
    if from >= until {
        return;
    }
    let total: usize = source.iter().filter_map(textual_piece).map(|s| s.len()).sum();
    let mut cursor = 0;
    for node in source {
        let Some(piece) = textual_piece(node) else { continue };
        let end = cursor + piece.len();
        if piece.is_empty()
            && from <= cursor
            && (cursor < until || (cursor == total && until == total))
        {
            out.push(node.clone());
        }
        let lower = from.max(cursor);
        let upper = until.min(end);
        if lower < upper {
            if lower == cursor && upper == end {
                out.push(node.clone());
            } else {
                out.push(slice_node(node, lower - cursor, upper - cursor));
            }
        }
        cursor = end;
    }
}

fn slice_node(node: &Content, from: usize, until: usize) -> Content {
    use std::sync::Arc;
    match node {
        Content::Text(text) => Content::text(&text[from..until]),
        Content::Source(e) => slice_node(&e.body, from, until).with_source_span(e.span),
        Content::Located(e) => {
            Content::Located(Arc::new(crate::entities::content::LocatedContent {
                body: slice_node(&e.body, from, until),
                location: e.location,
            }))
        }
        Content::Realized(e) => {
            Content::Realized(Arc::new(crate::entities::content::RealizedContent {
                original: e.original.clone(),
                visual: slice_node(&e.visual, from, until),
            }))
        }
        Content::Sequence(children) => {
            let mut out = Vec::new();
            project_original_range(children, from, until, &mut out);
            Content::Sequence(Arc::from(out))
        }
        Content::Empty | Content::Space | Content::SpaceFromNewline => node.clone(),
        Content::ShowRecipe(_)
        | Content::Parbreak
        | Content::HtmlElem(_)
        | Content::HtmlFrame(_)
        | Content::Par { .. }
        | Content::Heading(_)
        | Content::Title(_)
        | Content::Strong(_)
        | Content::Emph(_)
        | Content::Raw(_)
        | Content::ListItem(_)
        | Content::EnumItem(_)
        | Content::Link(_)
        | Content::Equation(_)
        | Content::MathSequence(_)
        | Content::MathIdent(_)
        | Content::MathText(_)
        | Content::MathFrac(_)
        | Content::MathAttach(_)
        | Content::MathPrimes(_)
        | Content::MathRoot(_)
        | Content::MathStretch(_)
        | Content::MathDelimited(_)
        | Content::MathLr(_)
        | Content::MathMid(_)
        | Content::MathOverbrace(_)
        | Content::MathUnderbrace(_)
        | Content::MathUnderbracket(_)
        | Content::MathUnderparen(_)
        | Content::MathUndershell(_)
        | Content::MathOverbracket(_)
        | Content::MathOverparen(_)
        | Content::MathOvershell(_)
        | Content::MathAlignPoint(_)
        | Content::Linebreak(_)
        | Content::MathMatrix(_)
        | Content::MathCases(_)
        | Content::MathAccent(_)
        | Content::MathCancel(_)
        | Content::MathUnderline(_)
        | Content::MathOverline(_)
        | Content::MathVec(_)
        | Content::MathClassOverride(_)
        | Content::MathLimitsOverride(_)
        | Content::MathUnderover(_)
        | Content::MathOp(_)
        | Content::MathStyled(_)
        | Content::Label(_)
        | Content::Ref(_)
        | Content::CounterDisplay(_)
        | Content::CounterUpdate(_)
        | Content::Outline(_)
        | Content::OutlineEntry(_)
        | Content::Figure(_)
        | Content::FigureCaption(_)
        | Content::Image(_)
        | Content::Shape(_)
        | Content::Curve(_)
        | Content::Transform(_)
        | Content::Grid(_)
        | Content::GridHeader(_)
        | Content::GridFooter(_)
        | Content::GridCell(_)
        | Content::SetPage(_)
        | Content::PageRun(_)
        | Content::Align(_)
        | Content::Place(_)
        | Content::Flush(_)
        | Content::Styled(_)
        | Content::Divider(_)
        | Content::Terms(_)
        | Content::TermItem(_)
        | Content::Quote(_)
        | Content::Document { .. }
        | Content::Asset { .. }
        | Content::SmartQuote(_)
        | Content::PdfAttach(_)
        | Content::PdfArtifact(_)
        | Content::Underline(_)
        | Content::Strike(_)
        | Content::Overline(_)
        | Content::SmallCaps { .. }
        | Content::Pad(_)
        | Content::Hide(_)
        | Content::HSpace(_)
        | Content::VSpace(_)
        | Content::Pagebreak(_)
        | Content::Colbreak(_)
        | Content::Stack(_)
        | Content::Boxed(_)
        | Content::Block(_)
        | Content::TableCell(_)
        | Content::Bibliography(_)
        | Content::Cite(_)
        | Content::Footnote(_)
        | Content::FootnoteEntry(_)
        | Content::TableHeader(_)
        | Content::TableFooter(_)
        | Content::GridHLine(_)
        | Content::GridVLine(_)
        | Content::TableHLine(_)
        | Content::TableVLine(_)
        | Content::Table(_)
        | Content::Repeat(_)
        | Content::Columns(_)
        | Content::Metadata(_)
        | Content::State(_)
        | Content::StateUpdate(_)
        | Content::StateDisplay(_)
        | Content::CounterDisplayCallback(_)
        | Content::ContextBlock(_)
        | Content::Dynamic(_) => {
            unreachable!("a nontextual node cannot enter a certified textual slice")
        }
    }
}

#[cfg(test)]
mod p1494_matching_tests {
    use super::project_original_range;
    use crate::entities::content::Content;

    #[test]
    fn splice_preserva_espacos_nao_casados_e_fatia_texto() {
        let source = [
            Content::text("ab"),
            Content::SpaceFromNewline,
            Content::text("cd"),
            Content::Space,
            Content::text("ef"),
        ];
        let mut prefix = Vec::new();
        project_original_range(&source, 0, 4, &mut prefix);
        assert!(matches!(&prefix[1], Content::SpaceFromNewline));
        assert_eq!(prefix[2].plain_text(), "c");

        let mut suffix = Vec::new();
        project_original_range(&source, 4, 8, &mut suffix);
        assert_eq!(suffix[0].plain_text(), "d");
        assert!(matches!(&suffix[1], Content::Space));
        assert_eq!(suffix[2].plain_text(), "ef");
    }
}
