#![allow(dead_code)]
struct Carrier { body: Content }
enum Content { Source(Box<Carrier>),ShowRecipe(()),Realized(()),Located(()),Empty,Text(String),Space,SpaceFromNewline,Parbreak,Sequence(Vec<Content>),HtmlElem(()),HtmlFrame(()),Par { body: Box<Content> },Heading(()),Title(()),Strong(()),Emph(()),Raw(()),ListItem(()),EnumItem(()),Link(()),Equation(()),MathSequence(()),MathIdent(()),MathText(()),MathFrac(()),MathAttach(()),MathPrimes(()),MathRoot(()),MathStretch(()),MathDelimited(()),MathLr(()),MathMid(()),MathOverbrace(()),MathUnderbrace(()),MathUnderbracket(()),MathUnderparen(()),MathUndershell(()),MathOverbracket(()),MathOverparen(()),MathOvershell(()),MathAlignPoint(()),Linebreak(()),MathMatrix(()),MathCases(()),MathAccent(()),MathCancel(()),MathUnderline(()),MathOverline(()),MathVec(()),MathClassOverride(()),MathLimitsOverride(()),MathUnderover(()),MathOp(()),MathStyled(()),Label(()),Ref(()),CounterDisplay(()),CounterUpdate(()),Outline(()),OutlineEntry(()),Figure(()),FigureCaption(()),Image(()),Shape(()),Curve(()),Transform(()),Grid(()),GridHeader(()),GridFooter(()),GridCell(()),SetPage(()),PageRun(()),Align(()),Place(()),Flush(()),Styled(Box<Carrier>),Divider(()),Terms(()),TermItem(()),Quote(()),Document { marker: () },Asset { marker: () },SmartQuote(()),PdfAttach(()),PdfArtifact(()),Underline(()),Strike(()),Overline(()),SmallCaps { marker: () },Pad(()),Hide(()),HSpace(()),VSpace(()),Pagebreak(()),Colbreak(()),Stack(()),Boxed(()),Block(()),TableCell(()),Bibliography(()),Cite(()),Footnote(()),FootnoteEntry(()),TableHeader(()),TableFooter(()),GridHLine(()),GridVLine(()),TableHLine(()),TableVLine(()),Table(()),Repeat(()),Columns(()),Metadata(()),State(()),StateUpdate(()),StateDisplay(()),CounterDisplayCallback(()),ContextBlock(()),Dynamic(()) }
enum ParagraphBody<'a> {
    Empty,
    Space,
    Text(&'a str),
    Sequence(&'a [Content]),
    Styled(&'a Content),
    Par,
    Opaque,
}

/// One exhaustive boundary owns the admission policy shared by these predicates.
fn paragraph_body(content: &Content) -> ParagraphBody<'_> {
    match content {
        Content::Source(e) => paragraph_body(&e.body),
        Content::Empty => ParagraphBody::Empty,
        Content::Space | Content::SpaceFromNewline => ParagraphBody::Space,
        Content::Text(text) => ParagraphBody::Text(text.as_str()),
        Content::Sequence(parts) => ParagraphBody::Sequence(parts),
        Content::Styled(styled) => ParagraphBody::Styled(&styled.body),
        Content::Par { .. } => ParagraphBody::Par,
        Content::ShowRecipe(..)
        | Content::Realized(..)
        | Content::Located(..)
        | Content::Parbreak
        | Content::HtmlElem(..)
        | Content::HtmlFrame(..)
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
        | Content::GridCell(..)
        | Content::SetPage(..)
        | Content::PageRun(..)
        | Content::Align(..)
        | Content::Place(..)
        | Content::Flush(..)
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
        | Content::Table(..)
        | Content::Repeat(..)
        | Content::Columns(..)
        | Content::Metadata(..)
        | Content::State(..)
        | Content::StateUpdate(..)
        | Content::StateDisplay(..)
        | Content::CounterDisplayCallback(..)
        | Content::ContextBlock(..)
        | Content::Dynamic(..) => ParagraphBody::Opaque,
    }
}

pub(super) fn is_paragraph(content: &Content) -> bool {
    match paragraph_body(content) {
        ParagraphBody::Par => true,
        ParagraphBody::Styled(body) => is_paragraph(body),
        ParagraphBody::Empty
        | ParagraphBody::Space
        | ParagraphBody::Text(_)
        | ParagraphBody::Sequence(_)
        | ParagraphBody::Opaque => false,
    }
}

fn is_empty_body(content: &Content) -> bool {
    match paragraph_body(content) {
        ParagraphBody::Empty | ParagraphBody::Space => true,
        ParagraphBody::Text(text) => text.is_empty(),
        ParagraphBody::Sequence(parts) => parts.iter().all(is_empty_body),
        ParagraphBody::Styled(body) => is_empty_body(body),
        ParagraphBody::Par | ParagraphBody::Opaque => false,
    }
}

fn contains_empty_text(content: &Content) -> bool {
    match paragraph_body(content) {
        ParagraphBody::Text(text) => text.is_empty(),
        ParagraphBody::Sequence(parts) => parts.iter().any(contains_empty_text),
        ParagraphBody::Styled(body) => contains_empty_text(body),
        ParagraphBody::Empty
        | ParagraphBody::Space
        | ParagraphBody::Par
        | ParagraphBody::Opaque => false,
    }
}
