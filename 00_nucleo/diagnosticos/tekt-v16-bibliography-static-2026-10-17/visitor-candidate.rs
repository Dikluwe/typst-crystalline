    /// Borrow static children in order. False prunes without evaluating callbacks.
    pub(crate) fn visit_content_static<F>(&self, visit: &mut F)
    where
        F: FnMut(&Content) -> bool,
    {
        if let Content::Source(e) = self { e.body.visit_content_static(visit); return; }
        if let Content::ShowRecipe(e) = self { e.body.visit_content_static(visit); return; }
        if let Content::Located(e) = self { e.body.visit_content_static(visit); return; }
        if let Content::Realized(e) = self { e.visual.visit_content_static(visit); return; }
        if !visit(self) { return; }
        match self {
            Content::Source(_) | Content::ShowRecipe(_) | Content::Located(_) | Content::Realized(_) => unreachable!("transparent carriers dispatched above"),
            Content::Sequence(parts) | Content::MathSequence(parts) => {
                for child in parts.iter() { child.visit_content_static(visit); }
            }
            Content::Par { body } | Content::SmallCaps { body } => body.visit_content_static(visit),
            Content::HtmlElem(e) => {
                if let crate::entities::html::HtmlBody::Content(body) = &e.body { body.visit_content_static(visit); }
            }
            Content::PageRun(e) => {
                for marginal in [&e.header, &e.footer] {
                    if let Some(crate::entities::page_running::PageMarginal::Content(c)) = marginal { c.visit_content_static(visit); }
                }
                if let Some(crate::entities::page_supplement::PageSupplement::Content(c)) = &e.supplement { c.visit_content_static(visit); }
                for layer in [&e.background, &e.foreground] {
                    if let Some(Some(c)) = layer { c.visit_content_static(visit); }
                }
                e.body.visit_content_static(visit);
            }
            Content::HtmlFrame(e) => e.body.visit_content_static(visit),
            Content::Heading(e) => e.body.visit_content_static(visit),
            Content::Title(e) => e.body.visit_content_static(visit),
            Content::Strong(e) => e.body.visit_content_static(visit),
            Content::Emph(e) => e.body.visit_content_static(visit),
            Content::ListItem(e) => e.body.visit_content_static(visit),
            Content::EnumItem(e) => e.body.visit_content_static(visit),
            Content::Link(e) => e.body.visit_content_static(visit),
            Content::Label(e) => e.body.visit_content_static(visit),
            Content::Equation(e) => e.body.visit_content_static(visit),
            Content::MathStretch(e) => e.body.visit_content_static(visit),
            Content::MathDelimited(e) => e.body.visit_content_static(visit),
            Content::MathLr(e) => e.body.visit_content_static(visit),
            Content::MathMid(e) => e.body.visit_content_static(visit),
            Content::MathCancel(e) => e.body.visit_content_static(visit),
            Content::MathUnderline(e) => e.body.visit_content_static(visit),
            Content::MathOverline(e) => e.body.visit_content_static(visit),
            Content::MathClassOverride(e) => e.body.visit_content_static(visit),
            Content::MathLimitsOverride(e) => e.body.visit_content_static(visit),
            Content::MathStyled(e) => e.body.visit_content_static(visit),
            Content::PdfArtifact(e) => e.body.visit_content_static(visit),
            Content::Underline(e) => e.body.visit_content_static(visit),
            Content::Strike(e) => e.body.visit_content_static(visit),
            Content::Overline(e) => e.body.visit_content_static(visit),
            Content::Pad(e) => e.body.visit_content_static(visit),
            Content::Hide(e) => e.body.visit_content_static(visit),
            Content::Block(e) => e.body.visit_content_static(visit),
            Content::Boxed(e) => e.body.visit_content_static(visit),
            Content::Repeat(e) => e.body.visit_content_static(visit),
            Content::Columns(e) => e.body.visit_content_static(visit),
            Content::Transform(e) => e.body.visit_content_static(visit),
            Content::GridHeader(e) => e.body.visit_content_static(visit),
            Content::GridFooter(e) => e.body.visit_content_static(visit),
            Content::GridCell(e) => e.body.visit_content_static(visit),
            Content::TableHeader(e) => e.body.visit_content_static(visit),
            Content::TableFooter(e) => e.body.visit_content_static(visit),
            Content::Footnote(e) => e.body.visit_content_static(visit),
            Content::Align(e) => e.body.visit_content_static(visit),
            Content::Place(e) => e.body.visit_content_static(visit),
            Content::Styled(e) => e.body.visit_content_static(visit),
            Content::Figure(e) => {
                e.body.visit_content_static(visit);
                if let Some(c) = &e.caption { c.visit_content_static(visit); }
            }
            Content::FigureCaption(e) => {
                e.body.visit_content_static(visit);
                if let Some(c) = &e.separator { c.visit_content_static(visit); }
            }
            Content::MathFrac(e) => { e.num.visit_content_static(visit); e.den.visit_content_static(visit); }
            Content::MathAttach(e) => {
                e.base.visit_content_static(visit);
                for slot in [&e.t, &e.b, &e.tl, &e.bl, &e.tr, &e.br] {
                    if let crate::entities::elements::math_attach::MathAttachSlot::Present(c) = slot { c.visit_content_static(visit); }
                }
            }
            Content::MathRoot(e) => {
                if let Some(c) = &e.index { c.visit_content_static(visit); }
                e.radicand.visit_content_static(visit);
            }
            Content::MathOverbrace(e) => {
                e.body.visit_content_static(visit);
                if let Some(c) = &e.annotation { c.visit_content_static(visit); }
            }
            Content::MathUnderbrace(e) => {
                e.body.visit_content_static(visit);
                if let Some(c) = &e.annotation { c.visit_content_static(visit); }
            }
            Content::MathUnderbracket(e) => {
                e.body.visit_content_static(visit);
                if let Some(c) = &e.annotation { c.visit_content_static(visit); }
            }
            Content::MathUnderparen(e) => {
                e.body.visit_content_static(visit);
                if let Some(c) = &e.annotation { c.visit_content_static(visit); }
            }
            Content::MathUndershell(e) => {
                e.body.visit_content_static(visit);
                if let Some(c) = &e.annotation { c.visit_content_static(visit); }
            }
            Content::MathOverbracket(e) => {
                e.body.visit_content_static(visit);
                if let Some(c) = &e.annotation { c.visit_content_static(visit); }
            }
            Content::MathOverparen(e) => {
                e.body.visit_content_static(visit);
                if let Some(c) = &e.annotation { c.visit_content_static(visit); }
            }
            Content::MathOvershell(e) => {
                e.body.visit_content_static(visit);
                if let Some(c) = &e.annotation { c.visit_content_static(visit); }
            }
            Content::MathMatrix(e) => {
                for row in &e.rows { for c in row { c.visit_content_static(visit); } }
            }
            Content::MathCases(e) => { for c in &e.children { c.visit_content_static(visit); } }
            Content::MathVec(e) => { for c in &e.children { c.visit_content_static(visit); } }
            Content::MathAccent(e) => e.base.visit_content_static(visit),
            Content::MathUnderover(e) => {
                e.base.visit_content_static(visit);
                for c in [&e.under, &e.over].into_iter().flatten() { c.visit_content_static(visit); }
            }
            Content::MathOp(e) => e.text.visit_content_static(visit),
            Content::Terms(e) => { for c in &e.items { c.visit_content_static(visit); } }
            Content::TermItem(e) => { e.term.visit_content_static(visit); e.description.visit_content_static(visit); }
            Content::Quote(e) => {
                e.body.visit_content_static(visit);
                if let Some(c) = &e.attribution { c.visit_content_static(visit); }
            }
            Content::Stack(e) => { for c in e.children.iter() { c.visit_content_static(visit); } }
            Content::Outline(e) => {
                if let Some(c) = &e.title { c.visit_content_static(visit); }
                if let Some(c) = e.realized_body() { c.visit_content_static(visit); }
            }
            Content::OutlineEntry(e) => {
                e.element.visit_content_static(visit);
                if let Some(c) = &e.fill { c.visit_content_static(visit); }
                if let Some(c) = e.prefix_ref() { c.visit_content_static(visit); }
            }
            Content::Grid(e) => {
                for c in &e.cells { c.visit_content_static(visit); }
                for c in [&e.header, &e.footer].into_iter().flatten() { c.visit_content_static(visit); }
            }
            Content::Table(e) => {
                if let Some(plan) = &e.resolved {
                    for c in &plan.cells { c.visit_content_static(visit); }
                } else {
                    for c in &e.children { c.visit_content_static(visit); }
                    for c in [&e.header, &e.footer].into_iter().flatten() { c.visit_content_static(visit); }
                }
                if let Some(c) = &e.caption { c.visit_content_static(visit); }
            }
            Content::TableCell(e) => e.visual().visit_content_static(visit),
            Content::Bibliography(e) => {
                if let Some(c) = &e.title { c.visit_content_static(visit); }
            }
            Content::Cite(e) => {
                if let Some(c) = &e.supplement { c.visit_content_static(visit); }
            }
            Content::FootnoteEntry(e) => {
                e.note.visit_content_static(visit);
                if let Some(c) = &e.separator { c.visit_content_static(visit); }
            }
            Content::Document { body, title, .. } => {
                body.visit_content_static(visit);
                if let Some(c) = title { c.visit_content_static(visit); }
            }
            Content::Empty
            | Content::Text(..)
            | Content::Space
            | Content::SpaceFromNewline
            | Content::Parbreak
            | Content::Raw(..)
            | Content::MathIdent(..)
            | Content::MathText(..)
            | Content::MathPrimes(..)
            | Content::MathAlignPoint(..)
            | Content::Linebreak(..)
            | Content::Ref(..)
            | Content::CounterDisplay(..)
            | Content::CounterUpdate(..)
            | Content::Image(..)
            | Content::Shape(..)
            | Content::Curve(..)
            | Content::SetPage(..)
            | Content::Flush(..)
            | Content::Divider(..)
            | Content::Asset { .. }
            | Content::SmartQuote(..)
            | Content::PdfAttach(..)
            | Content::HSpace(..)
            | Content::VSpace(..)
            | Content::Pagebreak(..)
            | Content::Colbreak(..)
            | Content::GridHLine(..)
            | Content::GridVLine(..)
            | Content::TableHLine(..)
            | Content::TableVLine(..)
            | Content::Metadata(..)
            | Content::State(..)
            | Content::StateUpdate(..)
            | Content::StateDisplay(..)
            | Content::CounterDisplayCallback(..)
            | Content::ContextBlock(..)
            | Content::Dynamic(..) => {}
        }
    }
