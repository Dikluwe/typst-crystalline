#![allow(dead_code,unused_variables,unreachable_patterns)]
enum Content{Source(()),ShowRecipe(()),Realized(()),Located(()),Empty,Text(()),Space,SpaceFromNewline,Parbreak,Sequence(()),HtmlElem(()),HtmlFrame(()),Par{body:()},Heading(()),Title(()),Strong(()),Emph(()),Raw(()),ListItem(()),EnumItem(()),Link(()),Equation(()),MathSequence(()),MathIdent(()),MathText(()),MathFrac(()),MathAttach(()),MathPrimes(()),MathRoot(()),MathStretch(()),MathDelimited(()),MathLr(()),MathMid(()),MathOverbrace(()),MathUnderbrace(()),MathUnderbracket(()),MathUnderparen(()),MathUndershell(()),MathOverbracket(()),MathOverparen(()),MathOvershell(()),MathAlignPoint(()),Linebreak(()),MathMatrix(()),MathCases(()),MathAccent(()),MathCancel(()),MathUnderline(()),MathOverline(()),MathVec(()),MathClassOverride(()),MathLimitsOverride(()),MathUnderover(()),MathOp(()),MathStyled(()),Label(()),Ref(()),CounterDisplay(()),CounterUpdate(()),Outline(()),OutlineEntry(()),Figure(()),FigureCaption(()),Image(()),Shape(()),Curve(()),Transform(()),Grid(()),GridHeader(()),GridFooter(()),GridCell(()),SetPage(()),PageRun(()),Align(()),Place(()),Flush(()),Styled(()),Divider(()),Terms(()),TermItem(()),Quote(()),Document{path:(),body:(),format:(),title:(),author:(),date:(),keywords:()},Asset{path:(),data:()},SmartQuote(()),PdfAttach(()),PdfArtifact(()),Underline(()),Strike(()),Overline(()),SmallCaps{body:()},Pad(()),Hide(()),HSpace(()),VSpace(()),Pagebreak(()),Colbreak(()),Stack(()),Boxed(()),Block(()),TableCell(()),Bibliography(()),Cite(()),Footnote(()),FootnoteEntry(()),TableHeader(()),TableFooter(()),GridHLine(()),GridVLine(()),TableHLine(()),TableVLine(()),Table(()),Repeat(()),Columns(()),Metadata(()),State(()),StateUpdate(()),StateDisplay(()),CounterDisplayCallback(()),ContextBlock(()),Dynamic(()),FutureBibliography}
fn classify(input:&Content){match input{Content::Source(_)
            | Content::ShowRecipe(_)
            | Content::Located(_)
            | Content::Realized(_)=>(),Content::Sequence(parts) | Content::MathSequence(parts)=>(),Content::Par { body } | Content::SmallCaps { body }=>(),Content::HtmlElem(e)=>(),Content::PageRun(e)=>(),Content::HtmlFrame(e)=>(),Content::Heading(e)=>(),Content::Title(e)=>(),Content::Strong(e)=>(),Content::Emph(e)=>(),Content::ListItem(e)=>(),Content::EnumItem(e)=>(),Content::Link(e)=>(),Content::Label(e)=>(),Content::Equation(e)=>(),Content::MathStretch(e)=>(),Content::MathDelimited(e)=>(),Content::MathLr(e)=>(),Content::MathMid(e)=>(),Content::MathCancel(e)=>(),Content::MathUnderline(e)=>(),Content::MathOverline(e)=>(),Content::MathClassOverride(e)=>(),Content::MathLimitsOverride(e)=>(),Content::MathStyled(e)=>(),Content::PdfArtifact(e)=>(),Content::Underline(e)=>(),Content::Strike(e)=>(),Content::Overline(e)=>(),Content::Pad(e)=>(),Content::Hide(e)=>(),Content::Block(e)=>(),Content::Boxed(e)=>(),Content::Repeat(e)=>(),Content::Columns(e)=>(),Content::Transform(e)=>(),Content::GridHeader(e)=>(),Content::GridFooter(e)=>(),Content::GridCell(e)=>(),Content::TableHeader(e)=>(),Content::TableFooter(e)=>(),Content::Footnote(e)=>(),Content::Align(e)=>(),Content::Place(e)=>(),Content::Styled(e)=>(),Content::Figure(e)=>(),Content::FigureCaption(e)=>(),Content::MathFrac(e)=>(),Content::MathAttach(e)=>(),Content::MathRoot(e)=>(),Content::MathOverbrace(e)=>(),Content::MathUnderbrace(e)=>(),Content::MathUnderbracket(e)=>(),Content::MathUnderparen(e)=>(),Content::MathUndershell(e)=>(),Content::MathOverbracket(e)=>(),Content::MathOverparen(e)=>(),Content::MathOvershell(e)=>(),Content::MathMatrix(e)=>(),Content::MathCases(e)=>(),Content::MathVec(e)=>(),Content::MathAccent(e)=>(),Content::MathUnderover(e)=>(),Content::MathOp(e)=>(),Content::Terms(e)=>(),Content::TermItem(e)=>(),Content::Quote(e)=>(),Content::Stack(e)=>(),Content::Outline(e)=>(),Content::OutlineEntry(e)=>(),Content::Grid(e)=>(),Content::Table(e)=>(),Content::TableCell(e)=>(),Content::Bibliography(e)=>(),Content::Cite(e)=>(),Content::FootnoteEntry(e)=>(),Content::Document { body, title, .. }=>(),Content::Empty
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
            | Content::Dynamic(..)=>(),}}
fn main(){}
