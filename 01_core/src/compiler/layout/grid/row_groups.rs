//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/grid/row_groups.md
//! @prompt-hash 360fbf3e
//! @layer L1
//! @updated 2026-10-17

use super::*;

/// Concatena grupos de linhas e sinaliza conflito explícito com o header.
pub(super) fn combine_cells(
    cells: &[Content],
    header: Option<&Content>,
    footer: Option<&Content>,
    num_cols: usize,
) -> Result<Vec<Content>, usize> {
    // P772i — header/footer como row-group real: extrair as células do
    // corpo de `Content::GridHeader`/`GridFooter` (um `Content::Sequence`
    // se houver mais que uma célula, ou o `Content` da própria célula se
    // só houver uma — `Content::sequence` colapsa Vec de 1 elemento) e
    // colar antes/depois das células normais. Cada grupo é preenchido
    // com `Content::Empty` até múltiplo de `num_cols` — headers/footers
    // ocupam linhas inteiras (paridade vanilla, `resolve.rs` §"row
    // group"). **Scope-out explícito** (não silencioso — ver
    // lab/results/construction/diagnostics/paridade-producao-p772i.md): renderiza uma
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
            let y = match cell {
                Content::GridCell(e) => e.y,
                Content::TableCell(e) => e.placement().1,
                Content::Source(_)
                | Content::ShowRecipe(_)
                | Content::Realized(_)
                | Content::Located(_)
                | Content::Empty
                | Content::Text(_)
                | Content::Space
                | Content::SpaceFromNewline
                | Content::Parbreak
                | Content::Sequence(_)
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
                | Content::Symbol(_)
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
                | Content::Dynamic(_) => None, // Content sem posição explícita.
            }?;
            // Linhas do header são contíguas a partir de 0 no modelo
            // splice — a primeira linha do range dentro do header é `y`.
            (y < header_rows).then_some(y)
        });
        if let Some(row) = conflict {
            return Err(row);
        }
    }
    let combined_cells: Vec<Content> =
        if header_cells.is_empty() && footer_cells.is_empty() {
            cells.to_vec()
        } else {
            let mut combined =
                Vec::with_capacity(header_cells.len() + cells.len() + footer_cells.len());
            combined.extend(header_cells);
            combined.extend_from_slice(cells);
            combined.extend(footer_cells);
            combined
        };
    Ok(combined_cells)
}

/// Parallel source addresses for the exact existing row-group projection.
pub(super) fn planned_sources(
    root: Option<super::super::page_running_requests::Cursor>,
    cells: &[Content],
    header: Option<&Content>,
    footer: Option<&Content>,
    num_cols: usize,
) -> crate::entities::source_result::SourceResult<
    Option<Vec<Option<super::super::page_running_requests::Cursor>>>,
> {
    use super::super::page_running_requests::Cursor;
    use crate::compiler::introspect::page_running::PlanEdge;
    fn invalid() -> Vec<crate::entities::source_result::SourceDiagnostic> {
        vec![crate::entities::source_result::SourceDiagnostic::error(
            crate::entities::span::Span::detached(),
            "invalid source-owned grid projection",
        )]
    }
    fn child(
        cursor: &Cursor,
        edge: PlanEdge,
    ) -> crate::entities::source_result::SourceResult<Cursor> {
        let packet = cursor.plan.child(cursor.packet, edge).ok_or_else(invalid)?;
        let mut result = cursor.clone();
        result.packet = packet;
        Ok(result)
    }
    fn group(
        content: &Content,
        mut cursor: Cursor,
        num_cols: usize,
        header: bool,
    ) -> crate::entities::source_result::SourceResult<Vec<Option<Cursor>>> {
        let mut body = content;
        match content {
            Content::GridHeader(e) => {
                if header {
                    body = &e.body;
                    cursor = child(&cursor, PlanEdge::GridHeaderBody)?;
                }
            }
            Content::TableHeader(e) => {
                if header {
                    body = &e.body;
                    cursor = child(&cursor, PlanEdge::TableHeaderBody)?;
                }
            }
            Content::GridFooter(e) => {
                if !header {
                    body = &e.body;
                    cursor = child(&cursor, PlanEdge::GridFooterBody)?;
                }
            }
            Content::TableFooter(e) => {
                if !header {
                    body = &e.body;
                    cursor = child(&cursor, PlanEdge::TableFooterBody)?;
                }
            }
            Content::Source(..)
            | Content::ShowRecipe(..)
            | Content::Realized(..)
            | Content::Located(..)
            | Content::Empty
            | Content::Text(..)
            | Content::Space
            | Content::SpaceFromNewline
            | Content::Parbreak
            | Content::Sequence(..)
            | Content::HtmlElem(..)
            | Content::HtmlFrame(..)
            | Content::Par { .. }
            | Content::Heading(..)
            | Content::Title(..)
            | Content::Strong(..)
            | Content::Emph(..)
            | Content::Raw(..)
            | Content::ListItem(..)
            | Content::EnumItem(..)
            | Content::Link(..)
            | Content::Equation(..)
            | Content::MathSequence(..)
            | Content::Symbol(..)
            | Content::MathIdent(..)
            | Content::MathText(..)
            | Content::MathFrac(..)
            | Content::MathAttach(..)
            | Content::MathPrimes(..)
            | Content::MathRoot(..)
            | Content::MathStretch(..)
            | Content::MathDelimited(..)
            | Content::MathLr(..)
            | Content::MathMid(..)
            | Content::MathOverbrace(..)
            | Content::MathUnderbrace(..)
            | Content::MathUnderbracket(..)
            | Content::MathUnderparen(..)
            | Content::MathUndershell(..)
            | Content::MathOverbracket(..)
            | Content::MathOverparen(..)
            | Content::MathOvershell(..)
            | Content::MathAlignPoint(..)
            | Content::Linebreak(..)
            | Content::MathMatrix(..)
            | Content::MathCases(..)
            | Content::MathAccent(..)
            | Content::MathCancel(..)
            | Content::MathUnderline(..)
            | Content::MathOverline(..)
            | Content::MathVec(..)
            | Content::MathClassOverride(..)
            | Content::MathLimitsOverride(..)
            | Content::MathUnderover(..)
            | Content::MathOp(..)
            | Content::MathStyled(..)
            | Content::Label(..)
            | Content::Ref(..)
            | Content::CounterDisplay(..)
            | Content::CounterUpdate(..)
            | Content::Outline(..)
            | Content::OutlineEntry(..)
            | Content::Figure(..)
            | Content::FigureCaption(..)
            | Content::Image(..)
            | Content::Shape(..)
            | Content::Curve(..)
            | Content::Transform(..)
            | Content::Grid(..)
            | Content::GridCell(..)
            | Content::SetPage(..)
            | Content::PageRun(..)
            | Content::Align(..)
            | Content::Place(..)
            | Content::Flush(..)
            | Content::Styled(..)
            | Content::Divider(..)
            | Content::Terms(..)
            | Content::TermItem(..)
            | Content::Quote(..)
            | Content::Document { .. }
            | Content::Asset { .. }
            | Content::SmartQuote(..)
            | Content::PdfAttach(..)
            | Content::PdfArtifact(..)
            | Content::Underline(..)
            | Content::Strike(..)
            | Content::Overline(..)
            | Content::SmallCaps { .. }
            | Content::Pad(..)
            | Content::Hide(..)
            | Content::HSpace(..)
            | Content::VSpace(..)
            | Content::Pagebreak(..)
            | Content::Colbreak(..)
            | Content::Stack(..)
            | Content::Boxed(..)
            | Content::Block(..)
            | Content::TableCell(..)
            | Content::Bibliography(..)
            | Content::Cite(..)
            | Content::Footnote(..)
            | Content::FootnoteEntry(..)
            | Content::GridHLine(..)
            | Content::GridVLine(..)
            | Content::TableHLine(..)
            | Content::TableVLine(..)
            | Content::Table(..)
            | Content::Repeat(..)
            | Content::Columns(..)
            | Content::Metadata(..)
            | Content::State(..)
            | Content::StateUpdate(..)
            | Content::StateDisplay(..)
            | Content::CounterDisplayCallback(..)
            | Content::ContextBlock(..)
            | Content::Dynamic(..) => {}
        }
        let mut routes = if let Content::Sequence(items) = body {
            (0..items.len())
                .map(|index| child(&cursor, PlanEdge::SequencePart(index)).map(Some))
                .collect::<crate::entities::source_result::SourceResult<Vec<_>>>()?
        } else {
            vec![Some(cursor)]
        };
        let remainder = routes.len() % num_cols;
        if remainder != 0 {
            routes.resize(routes.len() + num_cols - remainder, None);
        }
        Ok(routes)
    }
    let Some(root) = root else { return Ok(None) };
    let node = root.plan.node(root.packet).ok_or_else(invalid)?;
    let (body_edge, header_edge, footer_edge): (
        fn(usize) -> PlanEdge,
        PlanEdge,
        PlanEdge,
    ) = match &node.content {
        Content::Grid(_) => {
            (PlanEdge::GridCell, PlanEdge::GridHeader, PlanEdge::GridFooter)
        }
        Content::Table(table) => (
            if table.resolved.is_some() {
                PlanEdge::TableResolvedCell
            } else {
                PlanEdge::TableRawChild
            },
            PlanEdge::TableHeader,
            PlanEdge::TableFooter,
        ),
        Content::Source(..)
        | Content::ShowRecipe(..)
        | Content::Realized(..)
        | Content::Located(..)
        | Content::Empty
        | Content::Text(..)
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
        | Content::Sequence(..)
        | Content::HtmlElem(..)
        | Content::HtmlFrame(..)
        | Content::Par { .. }
        | Content::Heading(..)
        | Content::Title(..)
        | Content::Strong(..)
        | Content::Emph(..)
        | Content::Raw(..)
        | Content::ListItem(..)
        | Content::EnumItem(..)
        | Content::Link(..)
        | Content::Equation(..)
        | Content::MathSequence(..)
        | Content::Symbol(..)
        | Content::MathIdent(..)
        | Content::MathText(..)
        | Content::MathFrac(..)
        | Content::MathAttach(..)
        | Content::MathPrimes(..)
        | Content::MathRoot(..)
        | Content::MathStretch(..)
        | Content::MathDelimited(..)
        | Content::MathLr(..)
        | Content::MathMid(..)
        | Content::MathOverbrace(..)
        | Content::MathUnderbrace(..)
        | Content::MathUnderbracket(..)
        | Content::MathUnderparen(..)
        | Content::MathUndershell(..)
        | Content::MathOverbracket(..)
        | Content::MathOverparen(..)
        | Content::MathOvershell(..)
        | Content::MathAlignPoint(..)
        | Content::Linebreak(..)
        | Content::MathMatrix(..)
        | Content::MathCases(..)
        | Content::MathAccent(..)
        | Content::MathCancel(..)
        | Content::MathUnderline(..)
        | Content::MathOverline(..)
        | Content::MathVec(..)
        | Content::MathClassOverride(..)
        | Content::MathLimitsOverride(..)
        | Content::MathUnderover(..)
        | Content::MathOp(..)
        | Content::MathStyled(..)
        | Content::Label(..)
        | Content::Ref(..)
        | Content::CounterDisplay(..)
        | Content::CounterUpdate(..)
        | Content::Outline(..)
        | Content::OutlineEntry(..)
        | Content::Figure(..)
        | Content::FigureCaption(..)
        | Content::Image(..)
        | Content::Shape(..)
        | Content::Curve(..)
        | Content::Transform(..)
        | Content::GridHeader(..)
        | Content::GridFooter(..)
        | Content::GridCell(..)
        | Content::SetPage(..)
        | Content::PageRun(..)
        | Content::Align(..)
        | Content::Place(..)
        | Content::Flush(..)
        | Content::Styled(..)
        | Content::Divider(..)
        | Content::Terms(..)
        | Content::TermItem(..)
        | Content::Quote(..)
        | Content::Document { .. }
        | Content::Asset { .. }
        | Content::SmartQuote(..)
        | Content::PdfAttach(..)
        | Content::PdfArtifact(..)
        | Content::Underline(..)
        | Content::Strike(..)
        | Content::Overline(..)
        | Content::SmallCaps { .. }
        | Content::Pad(..)
        | Content::Hide(..)
        | Content::HSpace(..)
        | Content::VSpace(..)
        | Content::Pagebreak(..)
        | Content::Colbreak(..)
        | Content::Stack(..)
        | Content::Boxed(..)
        | Content::Block(..)
        | Content::TableCell(..)
        | Content::Bibliography(..)
        | Content::Cite(..)
        | Content::Footnote(..)
        | Content::FootnoteEntry(..)
        | Content::TableHeader(..)
        | Content::TableFooter(..)
        | Content::GridHLine(..)
        | Content::GridVLine(..)
        | Content::TableHLine(..)
        | Content::TableVLine(..)
        | Content::Repeat(..)
        | Content::Columns(..)
        | Content::Metadata(..)
        | Content::State(..)
        | Content::StateUpdate(..)
        | Content::StateDisplay(..)
        | Content::CounterDisplayCallback(..)
        | Content::ContextBlock(..)
        | Content::Dynamic(..) => return Err(invalid()),
    };
    let mut routes = Vec::new();
    if let Some(header) = header {
        routes.extend(group(header, child(&root, header_edge)?, num_cols, true)?);
    }
    for index in 0..cells.len() {
        routes.push(Some(child(&root, body_edge(index))?));
    }
    if let Some(footer) = footer {
        routes.extend(group(footer, child(&root, footer_edge)?, num_cols, false)?);
    }
    Ok(Some(routes))
}

fn table_group_view(content: &Content) -> Content {
    match content.source_body() {
        Content::TableHeader(header) => {
            let mut header = header.as_ref().clone();
            header.body = header.body.source_body().clone();
            Content::TableHeader(std::sync::Arc::new(header))
        }
        Content::TableFooter(footer) => {
            let mut footer = footer.as_ref().clone();
            footer.body = footer.body.source_body().clone();
            Content::TableFooter(std::sync::Arc::new(footer))
        }
        Content::Source(_)
        | Content::ShowRecipe(_)
        | Content::Realized(_)
        | Content::Located(_)
        | Content::Empty
        | Content::Text(_)
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
        | Content::Sequence(_)
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
        | Content::Symbol(_)
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
        | Content::Dynamic(_) => content.source_body().clone(),
    }
}

/// Normalize logical table groups in this single row-group owner.
pub(in crate::compiler::layout) fn table_groups(
    e: &crate::entities::elements::table::TableElem,
    cols: usize,
) -> crate::entities::source_result::SourceResult<(Vec<Content>, usize, usize)> {
    let header = e.header.as_ref().map(table_group_view);
    let footer = e.footer.as_ref().map(table_group_view);
    let headers = combine_cells(&[], header.as_ref(), None, cols)
        .unwrap_or_default()
        .len()
        / cols;
    let footers = combine_cells(&[], None, footer.as_ref(), cols)
        .unwrap_or_default()
        .len();
    let mut body = e.children.clone();
    if footers > 0 {
        let rem = body.len() % cols;
        if rem > 0 {
            body.resize(body.len() + cols - rem, Content::Empty);
        }
    }
    combine_cells(&body, header.as_ref(), footer.as_ref(), cols)
        .map(|cells| (cells, headers, footers))
        .map_err(|row| {
            let span = e
                .children
                .iter()
                .find_map(|c| match c {
                    Content::TableCell(c) if c.placement().1 == Some(row) => {
                        Some(c.source_span)
                    }
                    Content::Source(_)
                    | Content::ShowRecipe(_)
                    | Content::Realized(_)
                    | Content::Located(_)
                    | Content::Empty
                    | Content::Text(_)
                    | Content::Space
                    | Content::SpaceFromNewline
                    | Content::Parbreak
                    | Content::Sequence(_)
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
                    | Content::Symbol(_)
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
                    | Content::Dynamic(_) => None,
                })
                .unwrap_or(e.source_span);
            vec![crate::entities::source_result::SourceDiagnostic::error(
                span,
                format!("cell would conflict with header also spanning row {row}"),
            )
            .with_hint("try moving the cell or the header")]
        })
}

#[cfg(test)]
mod v16_table_tests {

    use crate::contracts::world::World;
    use crate::entities::{
        file_id::FileId,
        font_book::FontBook,
        source::Source,
        world_types::{Bytes, Datetime, FileError, FileResult, Font, Library},
    };
    struct TestWorld {
        library: Library,
        book: FontBook,
        source: Source,
    }
    impl TestWorld {
        fn new(text: &str) -> Self {
            Self {
                library: Library::new(),
                book: FontBook::new(),
                source: Source::new(
                    FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()),
                    text.into(),
                )
                .expect("valid Source test fixture"),
            }
        }
    }
    impl World for TestWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            self.source.id()
        }
        fn source(&self, _: FileId) -> FileResult<Source> {
            Ok(self.source.clone())
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<Font> {
            None
        }
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<Datetime> {
            None
        }
    }

    use crate::compiler::{
        eval::{EvalContext, EvalTarget},
        realize::realize_document,
        stdlib::native_table,
    };
    use crate::entities::{
        args::Args, compiler_features::Features, content::Content,
        element_registry::ElementRegistry, introspector::TagIntrospector, span::Span,
        value::Value,
    };
    fn span() -> Span {
        Span::from_range(TestWorld::new("").main(), 1..2)
    }
    fn table(raw: Vec<Content>) -> Content {
        let w = TestWorld::new("");
        let mut args = Args::positional(raw.into_iter().map(Value::Content).collect());
        args.named.insert("columns".into(), Value::Int(2));
        let Value::Content(t) =
            native_table(&mut EvalContext::new(), &args, &w, w.main()).unwrap()
        else {
            panic!("table")
        };
        t
    }
    fn find_table(c: &Content) -> Option<&crate::entities::elements::table::TableElem> {
        match c {
            Content::Source(s) => find_table(&s.body),
            Content::Located(e) => find_table(&e.body),
            Content::Realized(e) => find_table(&e.visual),
            Content::Table(t) => Some(t),
            Content::Sequence(v) => v.iter().find_map(find_table),
            Content::Empty => None,
            other => panic!("unsupported observation {other:?}"),
        }
    }
    fn realize(
        t: Content,
    ) -> crate::entities::source_result::SourceResult<
        crate::compiler::realize::RealizedDocument,
    > {
        let w = TestWorld::new("logical table");
        realize_document(
            &t,
            &w,
            &w.source,
            &ElementRegistry::new(),
            &TagIntrospector::empty(),
            false,
            EvalTarget::Paged,
            Features::empty(),
        )
        .0
    }
    fn group(header: bool) {
        for on in [false, true] {
            let v =
                Content::Sequence(vec![Content::text("a"), Content::text("b")].into());
            let v = if on { v.with_source_span(span()) } else { v };
            let g = if header {
                Content::table_header(v, true)
            } else {
                Content::table_footer(v, true)
            };
            let doc = realize(table(vec![g])).unwrap();
            let t = find_table(&doc.visual).unwrap();
            let r = t.resolved.as_ref().unwrap();
            let texts = r.cells.iter().map(|c| c.plain_text()).collect::<Vec<_>>();
            assert_eq!(texts, ["a", "b"]);
            assert_eq!(r.num_rows, 1);
        }
    }
    #[test]
    fn v16_table_header_group_source_sequence() {
        group(true);
    }
    #[test]
    fn v16_table_footer_group_source_sequence() {
        group(false);
    }
    #[test]
    fn v16_table_conflict_uses_first_cell_origin() {
        let h = Content::table_header(Content::text("H"), true);
        let c = Content::table_cell(Content::text("body"), None, Some(0), None, None)
            .with_source_span(span());
        let e = realize(table(vec![h, c])).err().expect("header conflict");
        assert_eq!(e[0].span, span());
        assert!(e[0].message.contains("header"));
    }
}

/// Boundaries in the normalized input stream, filled with actual row bounds by
/// the placement owner. None header metadata denotes the unique footer.
pub(in crate::compiler) struct OrderedGroup {
    pub(in crate::compiler) first: usize,
    pub(in crate::compiler) last: usize,
    pub(in crate::compiler) header: Option<(u32, bool)>,
    pub(in crate::compiler) start: usize,
    pub(in crate::compiler) end: usize,
}

pub(in crate::compiler) fn ordered_groups(
    items: &[Content],
) -> crate::entities::source_result::SourceResult<(Vec<Content>, Vec<OrderedGroup>)> {
    use crate::entities::source_result::SourceDiagnostic;
    let mut cells = Vec::new();
    let mut groups = Vec::new();
    let mut footer_seen = false;
    for item in items {
        let fail = |message| vec![SourceDiagnostic::error(item.source_span(), message)];
        match item.source_body() {
            Content::TableHeader(header) => {
                if footer_seen {
                    return Err(fail("table header cannot follow footer"));
                }
                if header.level == 0 {
                    return Err(fail("header level must be positive"));
                }
                let first = cells.len();
                if let Content::Sequence(parts) = header.body.source_body() {
                    cells.extend(parts.iter().map(|c| c.source_body().clone()));
                } else {
                    cells.push(header.body.source_body().clone());
                }
                if first == cells.len() {
                    return Err(fail("table header must contain a cell"));
                }
                groups.push(OrderedGroup {
                    first,
                    last: cells.len(),
                    header: Some((header.level, header.repeat)),
                    start: 0,
                    end: 0,
                });
            }
            Content::TableFooter(footer) => {
                if footer_seen {
                    return Err(fail("table: não pode haver mais do que um footer"));
                }
                footer_seen = true;
                let first = cells.len();
                if let Content::Sequence(parts) = footer.body.source_body() {
                    cells.extend(parts.iter().map(|c| c.source_body().clone()));
                } else {
                    cells.push(footer.body.source_body().clone());
                }
                if first == cells.len() {
                    cells.push(Content::Empty);
                }
                groups.push(OrderedGroup {
                    first,
                    last: cells.len(),
                    header: None,
                    start: 0,
                    end: 0,
                });
            }
            Content::TableHLine(_) | Content::TableVLine(_) => {
                return Err(fail(
                    "ordered headers with explicit lines are not supported",
                ));
            }
            Content::GridCell(..)
            | Content::TableCell(..)
            | Content::Source(..)
            | Content::ShowRecipe(..)
            | Content::Realized(..)
            | Content::Located(..)
            | Content::Empty
            | Content::Text(..)
            | Content::Space
            | Content::SpaceFromNewline
            | Content::Parbreak
            | Content::Sequence(..)
            | Content::HtmlElem(..)
            | Content::HtmlFrame(..)
            | Content::Par { .. }
            | Content::Heading(..)
            | Content::Title(..)
            | Content::Strong(..)
            | Content::Emph(..)
            | Content::Raw(..)
            | Content::ListItem(..)
            | Content::EnumItem(..)
            | Content::Link(..)
            | Content::Equation(..)
            | Content::MathSequence(..)
            | Content::Symbol(..)
            | Content::MathIdent(..)
            | Content::MathText(..)
            | Content::MathFrac(..)
            | Content::MathAttach(..)
            | Content::MathPrimes(..)
            | Content::MathRoot(..)
            | Content::MathStretch(..)
            | Content::MathDelimited(..)
            | Content::MathLr(..)
            | Content::MathMid(..)
            | Content::MathOverbrace(..)
            | Content::MathUnderbrace(..)
            | Content::MathUnderbracket(..)
            | Content::MathUnderparen(..)
            | Content::MathUndershell(..)
            | Content::MathOverbracket(..)
            | Content::MathOverparen(..)
            | Content::MathOvershell(..)
            | Content::MathAlignPoint(..)
            | Content::Linebreak(..)
            | Content::MathMatrix(..)
            | Content::MathCases(..)
            | Content::MathAccent(..)
            | Content::MathCancel(..)
            | Content::MathUnderline(..)
            | Content::MathOverline(..)
            | Content::MathVec(..)
            | Content::MathClassOverride(..)
            | Content::MathLimitsOverride(..)
            | Content::MathUnderover(..)
            | Content::MathOp(..)
            | Content::MathStyled(..)
            | Content::Label(..)
            | Content::Ref(..)
            | Content::CounterDisplay(..)
            | Content::CounterUpdate(..)
            | Content::Outline(..)
            | Content::OutlineEntry(..)
            | Content::Figure(..)
            | Content::FigureCaption(..)
            | Content::Image(..)
            | Content::Shape(..)
            | Content::Curve(..)
            | Content::Transform(..)
            | Content::Grid(..)
            | Content::GridHeader(..)
            | Content::GridFooter(..)
            | Content::SetPage(..)
            | Content::PageRun(..)
            | Content::Align(..)
            | Content::Place(..)
            | Content::Flush(..)
            | Content::Styled(..)
            | Content::Divider(..)
            | Content::Terms(..)
            | Content::TermItem(..)
            | Content::Quote(..)
            | Content::Document { .. }
            | Content::Asset { .. }
            | Content::SmartQuote(..)
            | Content::PdfAttach(..)
            | Content::PdfArtifact(..)
            | Content::Underline(..)
            | Content::Strike(..)
            | Content::Overline(..)
            | Content::SmallCaps { .. }
            | Content::Pad(..)
            | Content::Hide(..)
            | Content::HSpace(..)
            | Content::VSpace(..)
            | Content::Pagebreak(..)
            | Content::Colbreak(..)
            | Content::Stack(..)
            | Content::Boxed(..)
            | Content::Block(..)
            | Content::Bibliography(..)
            | Content::Cite(..)
            | Content::Footnote(..)
            | Content::FootnoteEntry(..)
            | Content::GridHLine(..)
            | Content::GridVLine(..)
            | Content::Table(..)
            | Content::Repeat(..)
            | Content::Columns(..)
            | Content::Metadata(..)
            | Content::State(..)
            | Content::StateUpdate(..)
            | Content::StateDisplay(..)
            | Content::CounterDisplayCallback(..)
            | Content::ContextBlock(..)
            | Content::Dynamic(..) => {
                if footer_seen {
                    return Err(fail("footer must end at the last row"));
                }
                cells.push(item.source_body().clone());
            }
        }
    }
    Ok((cells, groups))
}
