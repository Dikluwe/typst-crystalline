//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/content/page_structure.md
//! @prompt-hash 4b232b80
//! @layer L1
//! @updated 2026-10-10

use super::*;

impl Content {
    /// Divide page and column markers into ordered structural segments.
    fn page_column_segments(body: &Self) -> Vec<Self> {
        match body {
            Self::Source(e) => Self::page_column_segments(&e.body)
                .into_iter()
                .map(|c| c.with_source_span(e.span))
                .collect(),
            Self::ShowRecipe(_) | Self::Located(_) | Self::Realized(_) => {
                vec![body.clone()]
            }
            Self::Pagebreak(_) => vec![body.clone()],
            Self::Sequence(seq) => {
                let mut pieces: Vec<Self> = Vec::new();
                let mut current: Vec<Self> = Vec::new();
                for child in seq.iter() {
                    for piece in Self::page_column_segments(child) {
                        if matches!(piece.source_body(), Self::Pagebreak(_)) {
                            if !current.is_empty() {
                                pieces.push(Self::sequence(current));
                                current = Vec::new();
                            }
                            pieces.push(piece);
                        } else {
                            current.push(piece);
                        }
                    }
                }
                if !current.is_empty() {
                    pieces.push(Self::sequence(current));
                }
                pieces
            }
            Self::Styled(styled) => {
                let inner_pieces = Self::page_column_segments(&styled.body);
                inner_pieces
                    .into_iter()
                    .map(|piece| match piece.source_body() {
                        Self::Pagebreak(_) => piece,
                        _ => Self::styled(piece, styled.styles.clone()),
                    })
                    .collect()
            }
            // intencional: nós atómicos/folhas não contêm sequências internas de pagebreaks e são devolvidos em lote unitário
            Self::Empty
            | Self::Text(_)
            | Self::PageRun(_)
            | Self::Space
            | Self::SpaceFromNewline
            | Self::Parbreak
            | Self::Par { .. }
            | Self::Heading(_)
            | Self::Title(_)
            | Self::Strong(_)
            | Self::Emph(_)
            | Self::Raw(_)
            | Self::ListItem(_)
            | Self::EnumItem(_)
            | Self::Link(_)
            | Self::Equation(_)
            | Self::MathSequence(_)
            | Self::Symbol(_)
            | Self::MathIdent(_)
            | Self::MathText(_)
            | Self::MathFrac(_)
            | Self::MathAttach(_)
            | Self::MathPrimes(_)
            | Self::MathRoot(_)
            | Self::MathStretch(_)
            | Self::MathDelimited(_)
            | Self::MathLr(_)
            | Self::MathMid(_)
            | Self::MathOverbrace(_)
            | Self::MathUnderbrace(_)
            | Self::MathUnderbracket(_)
            | Self::MathUnderparen(_)
            | Self::MathUndershell(_)
            | Self::MathOverbracket(_)
            | Self::MathOverparen(_)
            | Self::MathOvershell(_)
            | Self::MathAlignPoint(_)
            | Self::Linebreak(_)
            | Self::MathMatrix(_)
            | Self::MathCases(_)
            | Self::MathAccent(_)
            | Self::MathCancel(_)
            | Self::MathUnderline(_)
            | Self::MathOverline(_)
            | Self::MathVec(_)
            | Self::MathClassOverride(_)
            | Self::MathLimitsOverride(_)
            | Self::MathUnderover(_)
            | Self::MathOp(_)
            | Self::MathStyled(_)
            | Self::Label(_)
            | Self::Ref(_)
            | Self::CounterDisplay(_)
            | Self::CounterUpdate(_)
            | Self::Outline(_)
            | Self::OutlineEntry(_)
            | Self::Figure(_)
            | Self::FigureCaption(_)
            | Self::Image(_)
            | Self::Shape(_)
            | Self::Curve(_)
            | Self::Transform(_)
            | Self::Grid(_)
            | Self::GridHeader(_)
            | Self::GridFooter(_)
            | Self::GridCell(_)
            | Self::SetPage(_)
            | Self::Align(_)
            | Self::Place(_)
            | Self::Flush(_)
            | Self::Divider(_)
            | Self::Terms(_)
            | Self::TermItem(_)
            | Self::Quote(_)
            | Self::Document { .. }
            | Self::Asset { .. }
            | Self::SmartQuote(_)
            | Self::PdfAttach(_)
            | Self::PdfArtifact(_)
            | Self::Underline(_)
            | Self::Strike(_)
            | Self::Overline(_)
            | Self::SmallCaps { .. }
            | Self::Pad(_)
            | Self::Hide(_)
            | Self::HSpace(_)
            | Self::VSpace(_)
            | Self::Colbreak(_)
            | Self::Stack(_)
            | Self::Boxed(_)
            | Self::Block(_)
            | Self::TableCell(_)
            | Self::Bibliography(_)
            | Self::Cite(_)
            | Self::Footnote(_)
            | Self::FootnoteEntry(_)
            | Self::TableHeader(_)
            | Self::TableFooter(_)
            | Self::GridHLine(_)
            | Self::GridVLine(_)
            | Self::TableHLine(_)
            | Self::TableVLine(_)
            | Self::Table(_)
            | Self::Repeat(_)
            | Self::Columns(_)
            | Self::Metadata(_)
            | Self::State(_)
            | Self::StateUpdate(_)
            | Self::StateDisplay(_)
            | Self::CounterDisplayCallback(_)
            | Self::ContextBlock(_)
            | Self::Dynamic(_)
            | Self::HtmlElem(_)
            | Self::HtmlFrame(_) => vec![body.clone()],
        }
    }

    /// **P537b** — Liga `#set page(columns: N)` ao consumer `Content::Columns`.
    ///
    /// Percorre a árvore de `Content` e, sempre que encontra um
    /// `Content::SetPage { columns: Some(n) }` dentro de uma `Sequence`, envolve
    /// os nós subsequentes (até outro `SetPage`, `Pagebreak` ou fim da sequência)
    /// num `Content::Columns { count: n, body: ... }`.
    ///
    /// **P627**: se o body resultante for uma `Sequence` que contém
    /// `Content::Pagebreak` ao seu nível, o body é partido em segmentos;
    /// cada segmento (e cada `Pagebreak` preservado) é emitido como elemento
    /// separado. Isto permite que secções de páginas subsequentes, com
    /// `text.dir` diferentes, obtenham a sua própria direcção de preenchimento.
    ///
    /// O `SetPage` original permanece imediatamente antes do `Columns`, para que
    /// `set_page::layout` actualize `page_config` antes de `columns::layout`
    /// consumir o body.
    ///
    /// Se uma `Sequence` não contiver `SetPage { columns: Some(_) }`, é
    /// preservada exactamente como está (zero reescrita estrutural).
    pub fn wrap_page_columns(self) -> Self {
        self.map_content(&mut |content| {
            if let Self::Sequence(parts) = content {
                // Só reescrever Sequences que efectivamente têm um set-rule de
                // colunas. Isto evita alterar a estrutura AST de documentos que
                // não usam `#set page(columns:)` (preserva snapshots P307b).
                let needs_wrap = parts.iter().any(
                    |p| matches!(p.source_body(), Self::SetPage(delta) if delta.columns.is_some()),
                );
                if !needs_wrap {
                    return Ok(None);
                }

                let parts_vec: Vec<Self> = parts.iter().cloned().collect();
                let mut out = Vec::with_capacity(parts_vec.len());
                let mut i = 0;
                while i < parts_vec.len() {
                    let count = match parts_vec[i].source_body() {
                        Self::SetPage(delta) => delta.columns,
                        _ => None,
                    };
                    if let Some(count) = count {
                        // Encontrar a fronteira do body implícito: próximo SetPage
                        // ou Pagebreak termina o grupo deste set-rule.
                        let mut j = i + 1;
                        while j < parts_vec.len()
                            && !matches!(
                                parts_vec[j].source_body(),
                                Self::SetPage(_) | Self::Pagebreak(_)
                            )
                        {
                            j += 1;
                        }
                        // Preservar o SetPage original (actualiza page_config).
                        out.push(parts_vec[i].clone());
                        // **P627** — partir o body nos Pagebreaks ao nível da
                        // Sequence, para que cada secção de página tenha o seu
                        // próprio ColumnsElem (e portanto a sua própria
                        // direcção de preenchimento).
                        let body = Self::sequence(parts_vec[i + 1..j].to_vec());
                        let segments = Self::page_column_segments(&body);
                        for piece in segments {
                            match piece.source_body() {
                                Self::Pagebreak(_) => out.push(piece),
                                _ => out.push(Self::Columns(Arc::new(ColumnsElem {
                                    count,
                                    gutter: None,
                                    body: piece,
                                    page_columns: true,
                                }))),
                            }
                        }
                        i = j;
                    } else {
                        out.push(parts_vec[i].clone());
                        i += 1;
                    }
                }
                Ok(Some(Self::sequence(out)))
            } else {
                Ok(None)
            }
        })
        .unwrap_or(self)
    }
}

#[cfg(test)]
mod page_structure_tests {
    use super::Content;
    use std::sync::Arc;

    #[test]
    fn remaining_parity_root_page_columns_classify_sourced_markers() {
        use crate::entities::{file_id::FileId, span::Span, style::Styles};
        let file = FileId::from_raw(std::num::NonZeroU16::new(1).unwrap());
        let first_span = Span::from_range(file, 0..10);
        let break_span = Span::from_range(file, 20..30);
        let second_span = Span::from_range(file, 40..50);
        let first = Content::SetPage(Arc::new(super::SetPageDelta {
            columns: Some(2),
            ..Default::default()
        }))
        .with_source_span(first_span);
        let second = Content::SetPage(Arc::new(super::SetPageDelta {
            columns: Some(3),
            ..Default::default()
        }))
        .with_source_span(second_span);
        let page_break = Content::pagebreak(false, None).with_source_span(break_span);
        let wrapped = Content::sequence(vec![
            first.clone(),
            Content::styled(
                Content::sequence(vec![
                    Content::text("A"),
                    page_break.clone(),
                    Content::text("B"),
                ]),
                Styles::default(),
            ),
            second,
            Content::text("C"),
        ])
        .wrap_page_columns();
        let Content::Sequence(parts) = wrapped else { panic!("ordered page segments") };
        assert_eq!(
            parts.len(),
            6,
            "sourced markers retain their original boundaries: {parts:?}"
        );
        assert_eq!(parts[0].source_span(), first_span);
        assert!(
            matches!(parts[0].source_body(), Content::SetPage(delta) if delta.columns == Some(2))
        );
        for (index, count, text) in [(1, 2, "A"), (3, 2, "B"), (5, 3, "C")] {
            let Content::Columns(columns) = parts[index].source_body() else {
                panic!("implicit column section {index}")
            };
            assert_eq!(columns.count, count);
            assert!(columns.page_columns);
            assert_eq!(columns.body.plain_text(), text);
        }
        assert_eq!(parts[2].source_span(), break_span);
        assert!(matches!(parts[2].source_body(), Content::Pagebreak(_)));
        assert_eq!(parts[4].source_span(), second_span);
        assert!(
            matches!(parts[4].source_body(), Content::SetPage(delta) if delta.columns == Some(3))
        );

        // A marker directly in this Sequence ends its existing implicit group.
        let flat = Content::sequence(vec![
            first,
            Content::text("A"),
            page_break,
            Content::text("B"),
        ])
        .wrap_page_columns();
        let Content::Sequence(parts) = flat else { panic!("flat marker boundary") };
        assert_eq!(parts.len(), 4);
        assert_eq!(parts[0].source_span(), first_span);
        assert!(matches!(parts[1].source_body(), Content::Columns(columns)
            if columns.count == 2 && columns.body.plain_text() == "A"));
        assert_eq!(parts[2].source_span(), break_span);
        assert!(matches!(parts[2].source_body(), Content::Pagebreak(_)));
        assert!(matches!(parts[3].source_body(), Content::Text(text) if text == "B"));
    }

    #[test]
    fn wrapping_plain_content_preserves_it() {
        let content = Content::text("body");
        assert_eq!(content.clone().wrap_page_columns(), content);
    }

    #[test]
    fn page_columns_preserve_tables_and_transform_markers() {
        let table = Content::table(
            vec![],
            vec![],
            vec![Content::table_cell(Content::text("cell"), None, None, None, None)],
        );
        let wrapped = table.clone().wrap_page_columns();
        assert_eq!(wrapped, table);
        if let (Content::Table(before), Content::Table(after)) = (&table, &wrapped) {
            assert!(Arc::ptr_eq(before, after));
        } else {
            panic!("expected table");
        }

        let marker = Content::SetPage(Arc::new(super::SetPageDelta {
            columns: Some(2),
            ..Default::default()
        }));
        let with_columns = Content::sequence(vec![marker, Content::text("body")]);
        let wrapped = with_columns.wrap_page_columns();
        let Content::Sequence(parts) = wrapped else { panic!("expected sequence") };
        assert!(parts.iter().any(|part| matches!(part, Content::Columns(_))));
    }
}
