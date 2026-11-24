
// Follow the bottom-up mapper's edges for audited nodes. In particular, a
// mapped Located suppresses the body root callback, unlike reserve_locations.
fn try_reserve_mapped(
    content: &Content,
    locator: &mut Locator,
    observe_root: bool,
) -> SourceResult<bool> {
    use crate::entities::page_running::PageMarginal;
    use crate::entities::page_supplement::PageSupplement;

    match content {
        Content::Source(e) => return try_reserve_mapped(&e.body, locator, observe_root),
        Content::Located(e) => {
            if !try_reserve_mapped(&e.body, locator, false)? {
                return Ok(false);
            }
            if observe_root {
                locator.observe(e.location);
            }
        }
        Content::Realized(e) => {
            reserve_mapped_locations(&e.visual, locator)?;
            if observe_root {
                reserve_locations(&e.original, locator);
            }
        }
        Content::Sequence(parts) | Content::MathSequence(parts) => {
            for part in parts.iter() {
                reserve_mapped_locations(part, locator)?;
            }
        }
        Content::ShowRecipe(e) => reserve_mapped_locations(&e.body, locator)?,
        Content::Styled(e) => reserve_mapped_locations(&e.body, locator)?,
        Content::Par { body } | Content::SmallCaps { body } => {
            reserve_mapped_locations(body, locator)?;
        }
        Content::Heading(e) => reserve_mapped_locations(&e.body, locator)?,
        Content::Label(e) => reserve_mapped_locations(&e.body, locator)?,
        Content::Strong(e) => reserve_mapped_locations(&e.body, locator)?,
        Content::Emph(e) => reserve_mapped_locations(&e.body, locator)?,
        Content::Link(e) => reserve_mapped_locations(&e.body, locator)?,
        Content::Block(e) => reserve_mapped_locations(&e.body, locator)?,
        Content::Boxed(e) => reserve_mapped_locations(&e.body, locator)?,
        Content::Outline(e) => {
            for child in [e.title.as_ref(), e.realized_body()].into_iter().flatten() {
                reserve_mapped_locations(child, locator)?;
            }
        }
        Content::OutlineEntry(e) => {
            for child in [Some(&e.element), e.fill.as_ref(), e.prefix_ref()]
                .into_iter()
                .flatten()
            {
                reserve_mapped_locations(child, locator)?;
            }
        }
        Content::PageRun(e) => {
            for marginal in [&e.header, &e.footer] {
                if let Some(PageMarginal::Content(child)) = marginal {
                    reserve_mapped_locations(child, locator)?;
                }
            }
            if let Some(PageSupplement::Content(child)) = &e.supplement {
                reserve_mapped_locations(child, locator)?;
            }
            for layer in [&e.background, &e.foreground] {
                if let Some(Some(child)) = layer {
                    reserve_mapped_locations(child, locator)?;
                }
            }
            reserve_mapped_locations(&e.body, locator)?;
        }
        Content::Text(_)
        | Content::Empty
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
        | Content::Ref(_)
        | Content::SetPage(_)
        | Content::Raw(_)
        | Content::Linebreak(_)
        | Content::CounterDisplay(_)
        | Content::CounterUpdate(_)
        | Content::Symbol(_)
        | Content::MathIdent(_)
        | Content::MathText(_)
        | Content::Image(_)
        | Content::Divider(_)
        | Content::SmartQuote(_)
        | Content::PdfAttach(_)
        | Content::HSpace(_)
        | Content::VSpace(_)
        | Content::Pagebreak(_)
        | Content::Colbreak(_)
        | Content::Shape(_)
        | Content::Curve(_)
        | Content::Metadata(_)
        | Content::State(_)
        | Content::StateUpdate(_)
        | Content::StateDisplay(_)
        | Content::CounterDisplayCallback(_)
        | Content::GridHLine(_)
        | Content::GridVLine(_)
        | Content::TableHLine(_)
        | Content::TableVLine(_)
        | Content::Asset { .. }
        | Content::ContextBlock(_) => {}
        // The legacy mapper owns unaudited child projections and dynamic effects.
        Content::Table(_)
        | Content::TableCell(_)
        | Content::HtmlElem(_)
        | Content::HtmlFrame(_)
        | Content::Title(_)
        | Content::ListItem(_)
        | Content::EnumItem(_)
        | Content::Equation(_)
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
        | Content::Figure(_)
        | Content::FigureCaption(_)
        | Content::Transform(_)
        | Content::Grid(_)
        | Content::GridHeader(_)
        | Content::GridFooter(_)
        | Content::GridCell(_)
        | Content::Align(_)
        | Content::Place(_)
        | Content::Flush(_)
        | Content::Terms(_)
        | Content::TermItem(_)
        | Content::Quote(_)
        | Content::Document { .. }
        | Content::PdfArtifact(_)
        | Content::Underline(_)
        | Content::Strike(_)
        | Content::Overline(_)
        | Content::Pad(_)
        | Content::Hide(_)
        | Content::Stack(_)
        | Content::Bibliography(_)
        | Content::Cite(_)
        | Content::Footnote(_)
        | Content::FootnoteEntry(_)
        | Content::TableHeader(_)
        | Content::TableFooter(_)
        | Content::Repeat(_)
        | Content::Columns(_)
        | Content::Dynamic(_) => return Ok(false),
    }
    Ok(true)
}
