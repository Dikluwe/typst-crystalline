// Mechanical declaration expansion: exhaustive matches and all-field destructuring.
impl ObservationEq for crate::entities::content::Content { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Empty => match other { Self::Empty => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Text(a0) => match other { Self::Text(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Space => match other { Self::Space => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Parbreak => match other { Self::Parbreak => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Sequence(a0) => match other { Self::Sequence(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::HtmlElem(a0) => match other { Self::HtmlElem(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Par { body: a0 } => match other { Self::Par { body: b0 } => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Heading(a0) => match other { Self::Heading(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Title(a0) => match other { Self::Title(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Strong(a0) => match other { Self::Strong(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Emph(a0) => match other { Self::Emph(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Raw(a0) => match other { Self::Raw(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::ListItem(a0) => match other { Self::ListItem(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::EnumItem(a0) => match other { Self::EnumItem(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Link(a0) => match other { Self::Link(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Equation(a0) => match other { Self::Equation(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::MathSequence(a0) => match other { Self::MathSequence(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::MathIdent(a0) => match other { Self::MathIdent(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::MathText(a0) => match other { Self::MathText(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::MathFrac(a0) => match other { Self::MathFrac(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::MathAttach(a0) => match other { Self::MathAttach(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::MathRoot(a0) => match other { Self::MathRoot(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::MathDelimited(a0) => match other { Self::MathDelimited(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::MathAlignPoint(a0) => match other { Self::MathAlignPoint(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Linebreak(a0) => match other { Self::Linebreak(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::MathMatrix(a0) => match other { Self::MathMatrix(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::MathCases(a0) => match other { Self::MathCases(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::MathAccent(a0) => match other { Self::MathAccent(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::MathCancel(a0) => match other { Self::MathCancel(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::MathUnderline(a0) => match other { Self::MathUnderline(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::MathVec(a0) => match other { Self::MathVec(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::MathClassOverride(a0) => match other { Self::MathClassOverride(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::MathLimitsOverride(a0) => match other { Self::MathLimitsOverride(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::MathUnderover(a0) => match other { Self::MathUnderover(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::MathOp(a0) => match other { Self::MathOp(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::MathStyled(a0) => match other { Self::MathStyled(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Label(a0) => match other { Self::Label(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Ref(a0) => match other { Self::Ref(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::CounterDisplay(a0) => match other { Self::CounterDisplay(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::CounterUpdate(a0) => match other { Self::CounterUpdate(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Outline(a0) => match other { Self::Outline(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Figure(a0) => match other { Self::Figure(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Image(a0) => match other { Self::Image(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Shape(a0) => match other { Self::Shape(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Curve(a0) => match other { Self::Curve(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Transform(a0) => match other { Self::Transform(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Grid(a0) => match other { Self::Grid(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::GridHeader(a0) => match other { Self::GridHeader(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::GridFooter(a0) => match other { Self::GridFooter(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::GridCell(a0) => match other { Self::GridCell(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::SetPage { paper: a0, flipped: a1, binding: a2, width: a3, height: a4, margin: a5, numbering: a6, number_align: a7, header: a8, header_ascent: a9, footer: a10, footer_descent: a11, supplement: a12, columns: a13, bleed: a14, fill: a15, background: a16, foreground: a17 } => match other { Self::SetPage { paper: b0, flipped: b1, binding: b2, width: b3, height: b4, margin: b5, numbering: b6, number_align: b7, header: b8, header_ascent: b9, footer: b10, footer_descent: b11, supplement: b12, columns: b13, bleed: b14, fill: b15, background: b16, foreground: b17 } => ObservationRelation::Same.and(a0.observation_relation(b0)).and(a1.observation_relation(b1)).and(a2.observation_relation(b2)).and(a3.observation_relation(b3)).and(a4.observation_relation(b4)).and(a5.observation_relation(b5)).and(a6.observation_relation(b6)).and(a7.observation_relation(b7)).and(a8.observation_relation(b8)).and(a9.observation_relation(b9)).and(a10.observation_relation(b10)).and(a11.observation_relation(b11)).and(a12.observation_relation(b12)).and(a13.observation_relation(b13)).and(a14.observation_relation(b14)).and(a15.observation_relation(b15)).and(a16.observation_relation(b16)).and(a17.observation_relation(b17)), _ => ObservationRelation::Different },
Self::PageRun(a0) => match other { Self::PageRun(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Align(a0) => match other { Self::Align(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Place(a0) => match other { Self::Place(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Flush(a0) => match other { Self::Flush(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Styled(a0, a1) => match other { Self::Styled(b0, b1) => ObservationRelation::Same.and(a0.observation_relation(b0)).and(a1.observation_relation(b1)), _ => ObservationRelation::Different },
Self::Divider(a0) => match other { Self::Divider(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Terms(a0) => match other { Self::Terms(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::TermItem(a0) => match other { Self::TermItem(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Quote(a0) => match other { Self::Quote(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Document { title: a0, author: a1, date: a2, keywords: a3 } => match other { Self::Document { title: b0, author: b1, date: b2, keywords: b3 } => ObservationRelation::Same.and(a0.observation_relation(b0)).and(a1.observation_relation(b1)).and(a2.observation_relation(b2)).and(a3.observation_relation(b3)), _ => ObservationRelation::Different },
Self::Asset { path: a0, kind: a1 } => match other { Self::Asset { path: b0, kind: b1 } => ObservationRelation::Same.and(a0.observation_relation(b0)).and(a1.observation_relation(b1)), _ => ObservationRelation::Different },
Self::SmartQuote(a0) => match other { Self::SmartQuote(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::PdfAttach(a0) => match other { Self::PdfAttach(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::PdfArtifact(a0) => match other { Self::PdfArtifact(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Underline(a0) => match other { Self::Underline(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Strike(a0) => match other { Self::Strike(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Overline(a0) => match other { Self::Overline(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::SmallCaps { body: a0 } => match other { Self::SmallCaps { body: b0 } => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Pad(a0) => match other { Self::Pad(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Hide(a0) => match other { Self::Hide(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::HSpace(a0) => match other { Self::HSpace(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::VSpace(a0) => match other { Self::VSpace(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Pagebreak(a0) => match other { Self::Pagebreak(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Colbreak(a0) => match other { Self::Colbreak(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Stack(a0) => match other { Self::Stack(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Boxed(a0) => match other { Self::Boxed(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Block(a0) => match other { Self::Block(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::TableCell(a0) => match other { Self::TableCell(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Bibliography(a0) => match other { Self::Bibliography(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Cite(a0) => match other { Self::Cite(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Footnote(a0) => match other { Self::Footnote(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::TableHeader(a0) => match other { Self::TableHeader(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::TableFooter(a0) => match other { Self::TableFooter(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::GridHLine(a0) => match other { Self::GridHLine(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::GridVLine(a0) => match other { Self::GridVLine(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::TableHLine(a0) => match other { Self::TableHLine(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::TableVLine(a0) => match other { Self::TableVLine(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Table(a0) => match other { Self::Table(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Repeat(a0) => match other { Self::Repeat(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Columns(a0) => match other { Self::Columns(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Metadata(a0) => match other { Self::Metadata(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::State(a0) => match other { Self::State(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::StateUpdate(a0) => match other { Self::StateUpdate(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::StateDisplay(a0) => match other { Self::StateDisplay(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::CounterDisplayCallback(a0) => match other { Self::CounterDisplayCallback(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::ContextBlock(a0) => match other { Self::ContextBlock(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Dynamic(a0) => match other { Self::Dynamic(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::elements::align::AlignElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { alignment: _, body: _ } = self; ObservationRelation::Same.and(self.alignment.observation_relation(&other.alignment)).and(self.body.observation_relation(&other.body)) } }
impl ObservationEq for crate::entities::layout_types::Align2D { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { h: _, v: _ } = self; ObservationRelation::Same.and(self.h.observation_relation(&other.h)).and(self.v.observation_relation(&other.v)) } }
impl ObservationEq for crate::entities::layout_types::HAlign { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Left => match other { Self::Left => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Center => match other { Self::Center => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Right => match other { Self::Right => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Start => match other { Self::Start => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::End => match other { Self::End => ObservationRelation::Same, _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::layout_types::VAlign { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Top => match other { Self::Top => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Horizon => match other { Self::Horizon => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Bottom => match other { Self::Bottom => ObservationRelation::Same, _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::elements::math_styled::MathStyledElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { kind: _, bold: _, italic: _, body: _, cramped: _ } = self; ObservationRelation::Same.and(self.kind.observation_relation(&other.kind)).and(self.bold.observation_relation(&other.bold)).and(self.italic.observation_relation(&other.italic)).and(self.body.observation_relation(&other.body)).and(self.cramped.observation_relation(&other.cramped)) } }
impl ObservationEq for crate::entities::math_style::MathStyleKind { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Plain => match other { Self::Plain => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::SansSerif => match other { Self::SansSerif => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Chancery => match other { Self::Chancery => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Roundhand => match other { Self::Roundhand => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Fraktur => match other { Self::Fraktur => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Monospace => match other { Self::Monospace => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::DoubleStruck => match other { Self::DoubleStruck => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Script => match other { Self::Script => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::SScript => match other { Self::SScript => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Display => match other { Self::Display => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Inline => match other { Self::Inline => ObservationRelation::Same, _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::style_chain::StyleDelta { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { bold: _, bold_from_strong: _, italic: _, italic_from_emph: _, size: _, fill: _, heading_level: _, weight: _, tracking: _, leading: _, top_edge: _, bottom_edge: _, lang: _, font: _, subscript: _, superscript: _, highlight: _, highlight_radius: _, highlight_extent: _, subscript_size: _, superscript_size: _, custom: _ } = self; ObservationRelation::Same.and(self.bold.observation_relation(&other.bold)).and(self.bold_from_strong.observation_relation(&other.bold_from_strong)).and(self.italic.observation_relation(&other.italic)).and(self.italic_from_emph.observation_relation(&other.italic_from_emph)).and(self.size.observation_relation(&other.size)).and(self.fill.observation_relation(&other.fill)).and(self.heading_level.observation_relation(&other.heading_level)).and(self.weight.observation_relation(&other.weight)).and(self.tracking.observation_relation(&other.tracking)).and(self.leading.observation_relation(&other.leading)).and(self.top_edge.observation_relation(&other.top_edge)).and(self.bottom_edge.observation_relation(&other.bottom_edge)).and(self.lang.observation_relation(&other.lang)).and(self.font.observation_relation(&other.font)).and(self.subscript.observation_relation(&other.subscript)).and(self.superscript.observation_relation(&other.superscript)).and(self.highlight.observation_relation(&other.highlight)).and(self.highlight_radius.observation_relation(&other.highlight_radius)).and(self.highlight_extent.observation_relation(&other.highlight_extent)).and(self.subscript_size.observation_relation(&other.subscript_size)).and(self.superscript_size.observation_relation(&other.superscript_size)).and(self.custom.observation_relation(&other.custom)) } }
impl ObservationEq for crate::entities::value::Value { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::None => match other { Self::None => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Bool(a0) => match other { Self::Bool(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Int(a0) => match other { Self::Int(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Float(a0) => match other { Self::Float(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Str(a0) => match other { Self::Str(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Array(a0) => match other { Self::Array(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Dict(a0) => match other { Self::Dict(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Module(a0) => match other { Self::Module(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Datetime(a0) => match other { Self::Datetime(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Func(a0) => match other { Self::Func(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Content(a0) => match other { Self::Content(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::LocatedContent(a0, a1) => match other { Self::LocatedContent(b0, b1) => ObservationRelation::Same.and(a0.observation_relation(b0)).and(a1.observation_relation(b1)), _ => ObservationRelation::Different },
Self::Auto => match other { Self::Auto => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Length(a0) => match other { Self::Length(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Relative(a0) => match other { Self::Relative(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Ratio(a0) => match other { Self::Ratio(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Angle(a0) => match other { Self::Angle(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Color(a0) => match other { Self::Color(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Stroke(a0) => match other { Self::Stroke(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Fraction(a0) => match other { Self::Fraction(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Align(a0) => match other { Self::Align(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Location(a0) => match other { Self::Location(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Gradient(a0) => match other { Self::Gradient(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Regex(a0) => match other { Self::Regex(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Tiling(a0) => match other { Self::Tiling(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Bytes(a0) => match other { Self::Bytes(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Decimal(a0) => match other { Self::Decimal(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Duration(a0) => match other { Self::Duration(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Version(a0) => match other { Self::Version(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Selector(a0) => match other { Self::Selector(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Symbol(a0) => match other { Self::Symbol(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Args(a0) => match other { Self::Args(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::State(a0) => match other { Self::State(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Counter(a0) => match other { Self::Counter(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Label(a0) => match other { Self::Label(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Dir(a0) => match other { Self::Dir(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Path(a0) => match other { Self::Path(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Type(a0) => match other { Self::Type(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::path::VirtualRoot { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Project => match other { Self::Project => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Package(a0) => match other { Self::Package(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::package_spec::PackageSpec { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { namespace: _, name: _, version: _ } = self; ObservationRelation::Same.and(self.namespace.observation_relation(&other.namespace)).and(self.name.observation_relation(&other.name)).and(self.version.observation_relation(&other.version)) } }
impl ObservationEq for crate::entities::package_spec::PackageVersion { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { major: _, minor: _, patch: _ } = self; ObservationRelation::Same.and(self.major.observation_relation(&other.major)).and(self.minor.observation_relation(&other.minor)).and(self.patch.observation_relation(&other.patch)) } }
impl<T: ObservationEq> ObservationEq for crate::entities::rel::Rel<T> { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { rel: _, abs: _ } = self; ObservationRelation::Same.and(self.rel.observation_relation(&other.rel)).and(self.abs.observation_relation(&other.abs)) } }
impl ObservationEq for crate::entities::layout_types::Length { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { abs: _, em: _ } = self; ObservationRelation::Same.and(self.abs.observation_relation(&other.abs)).and(self.em.observation_relation(&other.em)) } }
impl ObservationEq for crate::entities::layout_types::Abs { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self(_) = self; ObservationRelation::Same.and(self.0.observation_relation(&other.0)) } }
impl ObservationEq for crate::entities::layout_types::TextEdge { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Metric(a0) => match other { Self::Metric(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Length(a0) => match other { Self::Length(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::color::Color { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Srgb { r: a0, g: a1, b: a2, a: a3 } => match other { Self::Srgb { r: b0, g: b1, b: b2, a: b3 } => ObservationRelation::Same.and(a0.observation_relation(b0)).and(a1.observation_relation(b1)).and(a2.observation_relation(b2)).and(a3.observation_relation(b3)), _ => ObservationRelation::Different },
Self::Luma { l: a0, a: a1 } => match other { Self::Luma { l: b0, a: b1 } => ObservationRelation::Same.and(a0.observation_relation(b0)).and(a1.observation_relation(b1)), _ => ObservationRelation::Different },
Self::LinearRgb { r: a0, g: a1, b: a2, a: a3 } => match other { Self::LinearRgb { r: b0, g: b1, b: b2, a: b3 } => ObservationRelation::Same.and(a0.observation_relation(b0)).and(a1.observation_relation(b1)).and(a2.observation_relation(b2)).and(a3.observation_relation(b3)), _ => ObservationRelation::Different },
Self::Oklab { l: a0, a: a1, b: a2, alpha: a3 } => match other { Self::Oklab { l: b0, a: b1, b: b2, alpha: b3 } => ObservationRelation::Same.and(a0.observation_relation(b0)).and(a1.observation_relation(b1)).and(a2.observation_relation(b2)).and(a3.observation_relation(b3)), _ => ObservationRelation::Different },
Self::Oklch { l: a0, c: a1, h: a2, alpha: a3 } => match other { Self::Oklch { l: b0, c: b1, h: b2, alpha: b3 } => ObservationRelation::Same.and(a0.observation_relation(b0)).and(a1.observation_relation(b1)).and(a2.observation_relation(b2)).and(a3.observation_relation(b3)), _ => ObservationRelation::Different },
Self::Hsl { h: a0, s: a1, l: a2, a: a3 } => match other { Self::Hsl { h: b0, s: b1, l: b2, a: b3 } => ObservationRelation::Same.and(a0.observation_relation(b0)).and(a1.observation_relation(b1)).and(a2.observation_relation(b2)).and(a3.observation_relation(b3)), _ => ObservationRelation::Different },
Self::Hsv { h: a0, s: a1, v: a2, a: a3 } => match other { Self::Hsv { h: b0, s: b1, v: b2, a: b3 } => ObservationRelation::Same.and(a0.observation_relation(b0)).and(a1.observation_relation(b1)).and(a2.observation_relation(b2)).and(a3.observation_relation(b3)), _ => ObservationRelation::Different },
Self::Cmyk { c: a0, m: a1, y: a2, k: a3 } => match other { Self::Cmyk { c: b0, m: b1, y: b2, k: b3 } => ObservationRelation::Same.and(a0.observation_relation(b0)).and(a1.observation_relation(b1)).and(a2.observation_relation(b2)).and(a3.observation_relation(b3)), _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::font_list::FontFamily { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { name: _, variants: _, variant: _, weight: _, style: _, covers: _ } = self; ObservationRelation::Same.and(self.name.observation_relation(&other.name)).and(self.variants.observation_relation(&other.variants)).and(self.variant.observation_relation(&other.variant)).and(self.weight.observation_relation(&other.weight)).and(self.style.observation_relation(&other.style)).and(self.covers.observation_relation(&other.covers)) } }
impl ObservationEq for crate::entities::font_list::FontNamePattern { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Literal(a0) => match other { Self::Literal(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Regex(a0) => match other { Self::Regex(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::font_list::Covers { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {

} } }
impl ObservationEq for crate::entities::elements::underline::UnderlineElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { body: _, stroke: _, offset: _, extent: _ } = self; ObservationRelation::Same.and(self.body.observation_relation(&other.body)).and(self.stroke.observation_relation(&other.stroke)).and(self.offset.observation_relation(&other.offset)).and(self.extent.observation_relation(&other.extent)) } }
impl ObservationEq for crate::entities::elements::v_space::VSpaceElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { amount: _, weak: _, weak_explicit: _ } = self; ObservationRelation::Same.and(self.amount.observation_relation(&other.amount)).and(self.weak.observation_relation(&other.weak)).and(self.weak_explicit.observation_relation(&other.weak_explicit)) } }
impl ObservationEq for crate::entities::elements::enum_item::EnumItemElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { number: _, body: _, numbering: _, indent: _, body_indent: _, tight: _ } = self; ObservationRelation::Same.and(self.number.observation_relation(&other.number)).and(self.body.observation_relation(&other.body)).and(self.numbering.observation_relation(&other.numbering)).and(self.indent.observation_relation(&other.indent)).and(self.body_indent.observation_relation(&other.body_indent)).and(self.tight.observation_relation(&other.tight)) } }
impl ObservationEq for crate::entities::enum_numbering::EnumNumbering { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Decimal => match other { Self::Decimal => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::LowerAlpha => match other { Self::LowerAlpha => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::UpperAlpha => match other { Self::UpperAlpha => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::LowerRoman => match other { Self::LowerRoman => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Custom(a0) => match other { Self::Custom(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::elements::table_footer::TableFooterElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { body: _, repeat: _ } = self; ObservationRelation::Same.and(self.body.observation_relation(&other.body)).and(self.repeat.observation_relation(&other.repeat)) } }
impl ObservationEq for crate::entities::elements::footnote::FootnoteElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { body: _, numbering: _ } = self; ObservationRelation::Same.and(self.body.observation_relation(&other.body)).and(self.numbering.observation_relation(&other.numbering)) } }
impl ObservationEq for crate::entities::elements::shape::ShapeElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { kind: _, width: _, height: _, fill: _, stroke: _, fill_rule: _ } = self; ObservationRelation::Same.and(self.kind.observation_relation(&other.kind)).and(self.width.observation_relation(&other.width)).and(self.height.observation_relation(&other.height)).and(self.fill.observation_relation(&other.fill)).and(self.stroke.observation_relation(&other.stroke)).and(self.fill_rule.observation_relation(&other.fill_rule)) } }
impl ObservationEq for crate::entities::paint::Paint { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Solid(a0) => match other { Self::Solid(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Gradient(a0) => match other { Self::Gradient(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Tiling(a0) => match other { Self::Tiling(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::geometry::Stroke { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { paint: _, thickness: _, cap: _, join: _, dash: _, miter_limit: _, specified: _, overhang: _ } = self; ObservationRelation::Same.and(self.paint.observation_relation(&other.paint)).and(self.thickness.observation_relation(&other.thickness)).and(self.cap.observation_relation(&other.cap)).and(self.join.observation_relation(&other.join)).and(self.dash.observation_relation(&other.dash)).and(self.miter_limit.observation_relation(&other.miter_limit)).and(self.specified.observation_relation(&other.specified)).and(self.overhang.observation_relation(&other.overhang)) } }
impl ObservationEq for crate::entities::geometry::DashPattern { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { array: _, phase: _ } = self; ObservationRelation::Same.and(self.array.observation_relation(&other.array)).and(self.phase.observation_relation(&other.phase)) } }
impl ObservationEq for crate::entities::geometry::DashLength { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Length(a0) => match other { Self::Length(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::LineWidth => match other { Self::LineWidth => ObservationRelation::Same, _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::geometry::LineJoin { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Miter => match other { Self::Miter => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Round => match other { Self::Round => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Bevel => match other { Self::Bevel => ObservationRelation::Same, _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::geometry::StrokeFields { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { paint: _, thickness: _, cap: _, join: _, dash: _, miter_limit: _ } = self; ObservationRelation::Same.and(self.paint.observation_relation(&other.paint)).and(self.thickness.observation_relation(&other.thickness)).and(self.cap.observation_relation(&other.cap)).and(self.join.observation_relation(&other.join)).and(self.dash.observation_relation(&other.dash)).and(self.miter_limit.observation_relation(&other.miter_limit)) } }
impl ObservationEq for crate::entities::geometry::LineCap { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Butt => match other { Self::Butt => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Round => match other { Self::Round => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Square => match other { Self::Square => ObservationRelation::Same, _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::geometry::FillRule { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::NonZero => match other { Self::NonZero => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::EvenOdd => match other { Self::EvenOdd => ObservationRelation::Same, _ => ObservationRelation::Different },
} } }
impl<T: ObservationEq> ObservationEq for crate::entities::geometry::ShapeKind<T> { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Rect => match other { Self::Rect => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::RoundedRect { radii: a0 } => match other { Self::RoundedRect { radii: b0 } => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Ellipse => match other { Self::Ellipse => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Line { dx: a0, dy: a1 } => match other { Self::Line { dx: b0, dy: b1 } => ObservationRelation::Same.and(a0.observation_relation(b0)).and(a1.observation_relation(b1)), _ => ObservationRelation::Different },
Self::Path(a0) => match other { Self::Path(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::geometry::PathItem { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::MoveTo(a0) => match other { Self::MoveTo(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::LineTo(a0) => match other { Self::LineTo(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::CubicTo(a0, a1, a2) => match other { Self::CubicTo(b0, b1, b2) => ObservationRelation::Same.and(a0.observation_relation(b0)).and(a1.observation_relation(b1)).and(a2.observation_relation(b2)), _ => ObservationRelation::Different },
Self::ClosePath => match other { Self::ClosePath => ObservationRelation::Same, _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::layout_types::Point { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { x: _, y: _ } = self; ObservationRelation::Same.and(self.x.observation_relation(&other.x)).and(self.y.observation_relation(&other.y)) } }
impl ObservationEq for crate::entities::layout_types::Pt { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self(_) = self; ObservationRelation::Same.and(self.0.observation_relation(&other.0)) } }
impl<T: ObservationEq> ObservationEq for crate::entities::corners::Corners<T> { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { top_left: _, top_right: _, bottom_right: _, bottom_left: _ } = self; ObservationRelation::Same.and(self.top_left.observation_relation(&other.top_left)).and(self.top_right.observation_relation(&other.top_right)).and(self.bottom_right.observation_relation(&other.bottom_right)).and(self.bottom_left.observation_relation(&other.bottom_left)) } }
impl ObservationEq for crate::entities::elements::math_matrix::MathMatrixElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { rows: _, delim: _, row_gap: _, column_gap: _, gap: _, augment: _ } = self; ObservationRelation::Same.and(self.rows.observation_relation(&other.rows)).and(self.delim.observation_relation(&other.delim)).and(self.row_gap.observation_relation(&other.row_gap)).and(self.column_gap.observation_relation(&other.column_gap)).and(self.gap.observation_relation(&other.gap)).and(self.augment.observation_relation(&other.augment)) } }
impl ObservationEq for crate::entities::elements::grid::GridElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { columns: _, rows: _, cells: _, hlines: _, vlines: _, gutter: _, align: _, inset: _, header: _, footer: _, stroke: _, fill: _ } = self; ObservationRelation::Same.and(self.columns.observation_relation(&other.columns)).and(self.rows.observation_relation(&other.rows)).and(self.cells.observation_relation(&other.cells)).and(self.hlines.observation_relation(&other.hlines)).and(self.vlines.observation_relation(&other.vlines)).and(self.gutter.observation_relation(&other.gutter)).and(self.align.observation_relation(&other.align)).and(self.inset.observation_relation(&other.inset)).and(self.header.observation_relation(&other.header)).and(self.footer.observation_relation(&other.footer)).and(self.stroke.observation_relation(&other.stroke)).and(self.fill.observation_relation(&other.fill)) } }
impl<T: ObservationEq> ObservationEq for crate::entities::sides::Sides<T> { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { left: _, top: _, right: _, bottom: _ } = self; ObservationRelation::Same.and(self.left.observation_relation(&other.left)).and(self.top.observation_relation(&other.top)).and(self.right.observation_relation(&other.right)).and(self.bottom.observation_relation(&other.bottom)) } }
impl ObservationEq for crate::entities::elements::grid_hline::GridHLineElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { start: _, end: _, row: _, stroke: _, position: _ } = self; ObservationRelation::Same.and(self.start.observation_relation(&other.start)).and(self.end.observation_relation(&other.end)).and(self.row.observation_relation(&other.row)).and(self.stroke.observation_relation(&other.stroke)).and(self.position.observation_relation(&other.position)) } }
impl ObservationEq for crate::entities::layout_types::TrackSizing { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Fixed(a0) => match other { Self::Fixed(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Auto => match other { Self::Auto => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Fraction(a0) => match other { Self::Fraction(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::elements::grid_vline::GridVLineElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { start: _, end: _, col: _, stroke: _, position: _ } = self; ObservationRelation::Same.and(self.start.observation_relation(&other.start)).and(self.end.observation_relation(&other.end)).and(self.col.observation_relation(&other.col)).and(self.stroke.observation_relation(&other.stroke)).and(self.position.observation_relation(&other.position)) } }
impl ObservationEq for crate::entities::elements::counter_update::CounterUpdateElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { key: _, action: _ } = self; ObservationRelation::Same.and(self.key.observation_relation(&other.key)).and(self.action.observation_relation(&other.action)) } }
impl ObservationEq for crate::entities::counter::CounterKey { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Page => match other { Self::Page => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Selector(a0) => match other { Self::Selector(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Str(a0) => match other { Self::Str(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::elements::label::LabelElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { name: _, body: _, auto: _ } = self; ObservationRelation::Same.and(self.name.observation_relation(&other.name)).and(self.body.observation_relation(&other.body)).and(self.auto.observation_relation(&other.auto)) } }
impl ObservationEq for crate::entities::elements::pdf_artifact::PdfArtifactElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { kind: _, body: _ } = self; ObservationRelation::Same.and(self.kind.observation_relation(&other.kind)).and(self.body.observation_relation(&other.body)) } }
impl ObservationEq for crate::entities::elements::pdf_artifact::ArtifactKind { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Header => match other { Self::Header => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Footer => match other { Self::Footer => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Watermark => match other { Self::Watermark => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::PageNumber => match other { Self::PageNumber => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::LineNumber => match other { Self::LineNumber => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Redaction => match other { Self::Redaction => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Bates => match other { Self::Bates => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Page => match other { Self::Page => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::PaginationOther => match other { Self::PaginationOther => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Layout => match other { Self::Layout => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Background => match other { Self::Background => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Other => match other { Self::Other => ObservationRelation::Same, _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::elements::smartquote::SmartQuoteElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { double: _, alternative: _, quotes: _ } = self; ObservationRelation::Same.and(self.double.observation_relation(&other.double)).and(self.alternative.observation_relation(&other.alternative)).and(self.quotes.observation_relation(&other.quotes)) } }
impl ObservationEq for crate::entities::elements::smartquote::SmartQuoteQuotes { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Auto => match other { Self::Auto => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Custom(a0) => match other { Self::Custom(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::elements::smartquote::SmartQuoteOverrides { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { single: _, double: _ } = self; ObservationRelation::Same.and(self.single.observation_relation(&other.single)).and(self.double.observation_relation(&other.double)) } }
impl ObservationEq for crate::entities::elements::smartquote::SmartQuotePair { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { open: _, close: _ } = self; ObservationRelation::Same.and(self.open.observation_relation(&other.open)).and(self.close.observation_relation(&other.close)) } }
impl ObservationEq for crate::entities::elements::math_limits_override::MathLimitsOverrideElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { body: _, limits: _, inline: _ } = self; ObservationRelation::Same.and(self.body.observation_relation(&other.body)).and(self.limits.observation_relation(&other.limits)).and(self.inline.observation_relation(&other.inline)) } }
impl ObservationEq for crate::entities::elements::ref::RefElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { name: _, supplement: _, form: _ } = self; ObservationRelation::Same.and(self.name.observation_relation(&other.name)).and(self.supplement.observation_relation(&other.supplement)).and(self.form.observation_relation(&other.form)) } }
impl ObservationEq for crate::entities::elements::ref::RefForm { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Normal => match other { Self::Normal => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Page => match other { Self::Page => ObservationRelation::Same, _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::elements::page_run::PageRunElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { paper: _, flipped: _, binding: _, width: _, height: _, margin: _, numbering: _, number_align: _, header: _, header_ascent: _, footer: _, footer_descent: _, supplement: _, columns: _, bleed: _, fill: _, background: _, foreground: _, body: _ } = self; ObservationRelation::Same.and(self.paper.observation_relation(&other.paper)).and(self.flipped.observation_relation(&other.flipped)).and(self.binding.observation_relation(&other.binding)).and(self.width.observation_relation(&other.width)).and(self.height.observation_relation(&other.height)).and(self.margin.observation_relation(&other.margin)).and(self.numbering.observation_relation(&other.numbering)).and(self.number_align.observation_relation(&other.number_align)).and(self.header.observation_relation(&other.header)).and(self.header_ascent.observation_relation(&other.header_ascent)).and(self.footer.observation_relation(&other.footer)).and(self.footer_descent.observation_relation(&other.footer_descent)).and(self.supplement.observation_relation(&other.supplement)).and(self.columns.observation_relation(&other.columns)).and(self.bleed.observation_relation(&other.bleed)).and(self.fill.observation_relation(&other.fill)).and(self.background.observation_relation(&other.background)).and(self.foreground.observation_relation(&other.foreground)).and(self.body.observation_relation(&other.body)) } }
impl ObservationEq for crate::entities::page_canvas::PageBleedSpec { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { left: _, right: _, top: _, bottom: _, two_sided: _ } = self; ObservationRelation::Same.and(self.left.observation_relation(&other.left)).and(self.right.observation_relation(&other.right)).and(self.top.observation_relation(&other.top)).and(self.bottom.observation_relation(&other.bottom)).and(self.two_sided.observation_relation(&other.two_sided)) } }
impl ObservationEq for crate::entities::page_running::PageNumberAlign { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { horizontal: _, vertical: _ } = self; ObservationRelation::Same.and(self.horizontal.observation_relation(&other.horizontal)).and(self.vertical.observation_relation(&other.vertical)) } }
impl ObservationEq for crate::entities::page_running::PageNumberVAlign { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Top => match other { Self::Top => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Bottom => match other { Self::Bottom => ObservationRelation::Same, _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::numbering::Numbering { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Pattern(a0) => match other { Self::Pattern(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Func(a0) => match other { Self::Func(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::page_geometry::PageBinding { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Auto => match other { Self::Auto => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Left => match other { Self::Left => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Right => match other { Self::Right => ObservationRelation::Same, _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::page_supplement::PageSupplement { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Auto => match other { Self::Auto => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::None => match other { Self::None => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Content(a0) => match other { Self::Content(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::layout_types::PageMarginSpec { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { left: _, right: _, top: _, bottom: _, two_sided: _ } = self; ObservationRelation::Same.and(self.left.observation_relation(&other.left)).and(self.right.observation_relation(&other.right)).and(self.top.observation_relation(&other.top)).and(self.bottom.observation_relation(&other.bottom)).and(self.two_sided.observation_relation(&other.two_sided)) } }
impl ObservationEq for crate::entities::layout_types::PageDimension { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Auto => match other { Self::Auto => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Length(a0) => match other { Self::Length(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::page_running::PageMarginal { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Auto => match other { Self::Auto => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::None => match other { Self::None => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Content(a0) => match other { Self::Content(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::page_canvas::PageFill { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Auto => match other { Self::Auto => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::None => match other { Self::None => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Paint(a0) => match other { Self::Paint(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::elements::columns::ColumnsElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { count: _, gutter: _, body: _, page_columns: _ } = self; ObservationRelation::Same.and(self.count.observation_relation(&other.count)).and(self.gutter.observation_relation(&other.gutter)).and(self.body.observation_relation(&other.body)).and(self.page_columns.observation_relation(&other.page_columns)) } }
impl ObservationEq for crate::entities::elements::strong::StrongElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { body: _ } = self; ObservationRelation::Same.and(self.body.observation_relation(&other.body)) } }
impl ObservationEq for crate::entities::elements::math_delimited::MathDelimitedElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { open: _, body: _, close: _ } = self; ObservationRelation::Same.and(self.open.observation_relation(&other.open)).and(self.body.observation_relation(&other.body)).and(self.close.observation_relation(&other.close)) } }
impl ObservationEq for crate::entities::elements::raw::RawElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { text: _, lang: _, block: _ } = self; ObservationRelation::Same.and(self.text.observation_relation(&other.text)).and(self.lang.observation_relation(&other.lang)).and(self.block.observation_relation(&other.block)) } }
impl ObservationEq for crate::entities::elements::bibliography::BibliographyElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { entries: _, path: _, title: _, style: _, locale: _ } = self; ObservationRelation::Same.and(self.entries.observation_relation(&other.entries)).and(self.path.observation_relation(&other.path)).and(self.title.observation_relation(&other.title)).and(self.style.observation_relation(&other.style)).and(self.locale.observation_relation(&other.locale)) } }
impl ObservationEq for crate::entities::bib_entry::BibEntry { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { key: _, author: _, title: _, year: _, volume: _, pages: _, journal: _, publisher: _, url: _, doi: _, editor: _, series: _, note: _, isbn: _, location: _, organization: _ } = self; ObservationRelation::Same.and(self.key.observation_relation(&other.key)).and(self.author.observation_relation(&other.author)).and(self.title.observation_relation(&other.title)).and(self.year.observation_relation(&other.year)).and(self.volume.observation_relation(&other.volume)).and(self.pages.observation_relation(&other.pages)).and(self.journal.observation_relation(&other.journal)).and(self.publisher.observation_relation(&other.publisher)).and(self.url.observation_relation(&other.url)).and(self.doi.observation_relation(&other.doi)).and(self.editor.observation_relation(&other.editor)).and(self.series.observation_relation(&other.series)).and(self.note.observation_relation(&other.note)).and(self.isbn.observation_relation(&other.isbn)).and(self.location.observation_relation(&other.location)).and(self.organization.observation_relation(&other.organization)) } }
impl ObservationEq for crate::entities::elements::grid_header::GridHeaderElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { body: _, repeat: _ } = self; ObservationRelation::Same.and(self.body.observation_relation(&other.body)).and(self.repeat.observation_relation(&other.repeat)) } }
impl ObservationEq for crate::entities::elements::pdf_attach::PdfAttachElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { path: _, data: _, relationship: _, mime_type: _, description: _ } = self; ObservationRelation::Same.and(self.path.observation_relation(&other.path)).and(self.data.observation_relation(&other.data)).and(self.relationship.observation_relation(&other.relationship)).and(self.mime_type.observation_relation(&other.mime_type)).and(self.description.observation_relation(&other.description)) } }
impl ObservationEq for crate::entities::elements::pdf_attach::AttachedFileRelationship { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Source => match other { Self::Source => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Data => match other { Self::Data => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Alternative => match other { Self::Alternative => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Supplement => match other { Self::Supplement => ObservationRelation::Same, _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::elements::math_op::MathOpElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { text: _, limits: _ } = self; ObservationRelation::Same.and(self.text.observation_relation(&other.text)).and(self.limits.observation_relation(&other.limits)) } }
impl ObservationEq for crate::entities::elements::equation::EquationElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { body: _, block: _ } = self; ObservationRelation::Same.and(self.body.observation_relation(&other.body)).and(self.block.observation_relation(&other.block)) } }
impl ObservationEq for crate::entities::elements::table::TableElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { columns: _, rows: _, children: _, hlines: _, vlines: _, header: _, footer: _, stroke: _, fill: _, caption: _, summary: _, inset: _, align: _ } = self; ObservationRelation::Same.and(self.columns.observation_relation(&other.columns)).and(self.rows.observation_relation(&other.rows)).and(self.children.observation_relation(&other.children)).and(self.hlines.observation_relation(&other.hlines)).and(self.vlines.observation_relation(&other.vlines)).and(self.header.observation_relation(&other.header)).and(self.footer.observation_relation(&other.footer)).and(self.stroke.observation_relation(&other.stroke)).and(self.fill.observation_relation(&other.fill)).and(self.caption.observation_relation(&other.caption)).and(self.summary.observation_relation(&other.summary)).and(self.inset.observation_relation(&other.inset)).and(self.align.observation_relation(&other.align)) } }
impl ObservationEq for crate::entities::elements::table_vline::TableVLineElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { start: _, end: _, col: _, stroke: _, position: _ } = self; ObservationRelation::Same.and(self.start.observation_relation(&other.start)).and(self.end.observation_relation(&other.end)).and(self.col.observation_relation(&other.col)).and(self.stroke.observation_relation(&other.stroke)).and(self.position.observation_relation(&other.position)) } }
impl ObservationEq for crate::entities::elements::table_hline::TableHLineElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { start: _, end: _, row: _, stroke: _, position: _ } = self; ObservationRelation::Same.and(self.start.observation_relation(&other.start)).and(self.end.observation_relation(&other.end)).and(self.row.observation_relation(&other.row)).and(self.stroke.observation_relation(&other.stroke)).and(self.position.observation_relation(&other.position)) } }
impl ObservationEq for crate::entities::elements::math_frac::MathFracElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { num: _, den: _, line: _ } = self; ObservationRelation::Same.and(self.num.observation_relation(&other.num)).and(self.den.observation_relation(&other.den)).and(self.line.observation_relation(&other.line)) } }
impl ObservationEq for crate::entities::elements::h_space::HSpaceElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { amount: _, weak: _, weak_explicit: _ } = self; ObservationRelation::Same.and(self.amount.observation_relation(&other.amount)).and(self.weak.observation_relation(&other.weak)).and(self.weak_explicit.observation_relation(&other.weak_explicit)) } }
impl ObservationEq for crate::entities::elements::h_space::Spacing { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Absolute(a0) => match other { Self::Absolute(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Fractional(a0) => match other { Self::Fractional(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::elements::math_vec::MathVecElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { children: _, delim: _, align: _, gap: _, explicit: _ } = self; ObservationRelation::Same.and(self.children.observation_relation(&other.children)).and(self.delim.observation_relation(&other.delim)).and(self.align.observation_relation(&other.align)).and(self.gap.observation_relation(&other.gap)).and(self.explicit.observation_relation(&other.explicit)) } }
impl ObservationEq for crate::entities::elements::math_vec::MathVecExplicit { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { delim: _, align: _, gap: _ } = self; ObservationRelation::Same.and(self.delim.observation_relation(&other.delim)).and(self.align.observation_relation(&other.align)).and(self.gap.observation_relation(&other.gap)) } }
impl ObservationEq for crate::entities::elements::math_underline::MathUnderlineElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { body: _ } = self; ObservationRelation::Same.and(self.body.observation_relation(&other.body)) } }
impl ObservationEq for crate::entities::elements::context_block::ContextBlockElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { id: _, closure: _ } = self; ObservationRelation::Same.and(self.id.observation_relation(&other.id)).and(self.closure.observation_relation(&other.closure)) } }
impl ObservationEq for crate::entities::source_result::SourceDiagnostic { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { severity: _, span: _, message: _, hints: _, trace: _ } = self; ObservationRelation::Same.and(self.severity.observation_relation(&other.severity)).and(self.span.observation_relation(&other.span)).and(self.message.observation_relation(&other.message)).and(self.hints.observation_relation(&other.hints)).and(self.trace.observation_relation(&other.trace)) } }
impl ObservationEq for crate::entities::source_result::Severity { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Error => match other { Self::Error => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Warning => match other { Self::Warning => ObservationRelation::Same, _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::source_result::Tracepoint { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Call(a0) => match other { Self::Call(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Show(a0) => match other { Self::Show(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Import(a0) => match other { Self::Import(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Include(a0) => match other { Self::Include(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
} } }
impl<T: ObservationEq> ObservationEq for crate::entities::span::Spanned<T> { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { v: _, span: _ } = self; ObservationRelation::Same.and(self.v.observation_relation(&other.v)).and(self.span.observation_relation(&other.span)) } }
impl ObservationEq for crate::entities::show::ShowRule { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { id: _, selector: _, transform: _ } = self; ObservationRelation::Same.and(self.id.observation_relation(&other.id)).and(self.selector.observation_relation(&other.selector)).and(self.transform.observation_relation(&other.transform)) } }
impl ObservationEq for crate::entities::selector::Selector { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Kind(a0) => match other { Self::Kind(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Label(a0) => match other { Self::Label(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Location(a0) => match other { Self::Location(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::And(a0) => match other { Self::And(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Or(a0) => match other { Self::Or(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Regex(a0) => match other { Self::Regex(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Where { base: a0, field: a1, value: a2 } => match other { Self::Where { base: b0, field: b1, value: b2 } => ObservationRelation::Same.and(a0.observation_relation(b0)).and(a1.observation_relation(b1)).and(a2.observation_relation(b2)), _ => ObservationRelation::Different },
Self::Within { base: a0, ancestor: a1 } => match other { Self::Within { base: b0, ancestor: b1 } => ObservationRelation::Same.and(a0.observation_relation(b0)).and(a1.observation_relation(b1)), _ => ObservationRelation::Different },
Self::Element { function: a0, fields: a1 } => match other { Self::Element { function: b0, fields: b1 } => ObservationRelation::Same.and(a0.observation_relation(b0)).and(a1.observation_relation(b1)), _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::show::Selector { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Text(a0) => match other { Self::Text(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::NodeKind(a0) => match other { Self::NodeKind(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::DynKind(a0) => match other { Self::DynKind(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Regex(a0) => match other { Self::Regex(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Where { base: a0, field: a1, value: a2 } => match other { Self::Where { base: b0, field: b1, value: b2 } => ObservationRelation::Same.and(a0.observation_relation(b0)).and(a1.observation_relation(b1)).and(a2.observation_relation(b2)), _ => ObservationRelation::Different },
Self::And(a0) => match other { Self::And(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Or(a0) => match other { Self::Or(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Label(a0) => match other { Self::Label(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::element_kind::ElementKind { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Heading => match other { Self::Heading => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Figure => match other { Self::Figure => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Citation => match other { Self::Citation => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Metadata => match other { Self::Metadata => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::State => match other { Self::State => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::StateUpdate => match other { Self::StateUpdate => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Outline => match other { Self::Outline => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Bibliography => match other { Self::Bibliography => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Equation => match other { Self::Equation => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::CounterUpdate => match other { Self::CounterUpdate => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Table => match other { Self::Table => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::List => match other { Self::List => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Enum => match other { Self::Enum => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Par => match other { Self::Par => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Link => match other { Self::Link => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Raw => match other { Self::Raw => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Quote => match other { Self::Quote => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Footnote => match other { Self::Footnote => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::StateDisplay => match other { Self::StateDisplay => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::CounterDisplay => match other { Self::CounterDisplay => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::ContextBlock => match other { Self::ContextBlock => ObservationRelation::Same, _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::show::Transformation { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Func(a0) => match other { Self::Func(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Content(a0) => match other { Self::Content(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Str(a0) => match other { Self::Str(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Style(a0) => match other { Self::Style(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::args::Args { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { items: _, named: _, span: _, occurrences: _ } = self; ObservationRelation::Same.and(self.items.observation_relation(&other.items)).and(self.named.observation_relation(&other.named)).and(self.span.observation_relation(&other.span)).and(self.occurrences.observation_relation(&other.occurrences)) } }
impl ObservationEq for crate::entities::args::ArgOccurrence { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { name: _, value: _, span: _, value_span: _ } = self; ObservationRelation::Same.and(self.name.observation_relation(&other.name)).and(self.value.observation_relation(&other.value)).and(self.span.observation_relation(&other.span)).and(self.value_span.observation_relation(&other.value_span)) } }
impl ObservationEq for crate::entities::show::NodeKind { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Heading => match other { Self::Heading => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Figure => match other { Self::Figure => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Strong => match other { Self::Strong => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Emph => match other { Self::Emph => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Raw => match other { Self::Raw => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Equation => match other { Self::Equation => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::ListItem => match other { Self::ListItem => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Underline => match other { Self::Underline => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Strike => match other { Self::Strike => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Overline => match other { Self::Overline => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Smallcaps => match other { Self::Smallcaps => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Subscript => match other { Self::Subscript => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Superscript => match other { Self::Superscript => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Highlight => match other { Self::Highlight => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Link => match other { Self::Link => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Quote => match other { Self::Quote => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Footnote => match other { Self::Footnote => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::List => match other { Self::List => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Enum => match other { Self::Enum => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Par => match other { Self::Par => ObservationRelation::Same, _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::syntax_kind::SyntaxKind { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::End => match other { Self::End => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Error => match other { Self::Error => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Shebang => match other { Self::Shebang => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::LineComment => match other { Self::LineComment => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::BlockComment => match other { Self::BlockComment => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Markup => match other { Self::Markup => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Text => match other { Self::Text => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Space => match other { Self::Space => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Linebreak => match other { Self::Linebreak => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Parbreak => match other { Self::Parbreak => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Escape => match other { Self::Escape => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Shorthand => match other { Self::Shorthand => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::SmartQuote => match other { Self::SmartQuote => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Strong => match other { Self::Strong => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Emph => match other { Self::Emph => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Raw => match other { Self::Raw => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::RawLang => match other { Self::RawLang => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::RawDelim => match other { Self::RawDelim => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::RawTrimmed => match other { Self::RawTrimmed => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Link => match other { Self::Link => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Label => match other { Self::Label => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Ref => match other { Self::Ref => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::RefMarker => match other { Self::RefMarker => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Heading => match other { Self::Heading => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::HeadingMarker => match other { Self::HeadingMarker => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::ListItem => match other { Self::ListItem => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::ListMarker => match other { Self::ListMarker => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::EnumItem => match other { Self::EnumItem => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::EnumMarker => match other { Self::EnumMarker => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::TermItem => match other { Self::TermItem => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::TermMarker => match other { Self::TermMarker => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Equation => match other { Self::Equation => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Math => match other { Self::Math => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::MathText => match other { Self::MathText => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::MathIdent => match other { Self::MathIdent => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::MathShorthand => match other { Self::MathShorthand => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::MathAlignPoint => match other { Self::MathAlignPoint => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::MathDelimited => match other { Self::MathDelimited => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::MathAttach => match other { Self::MathAttach => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::MathPrimes => match other { Self::MathPrimes => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::MathFrac => match other { Self::MathFrac => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::MathRoot => match other { Self::MathRoot => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Hash => match other { Self::Hash => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::LeftBrace => match other { Self::LeftBrace => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::RightBrace => match other { Self::RightBrace => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::LeftBracket => match other { Self::LeftBracket => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::RightBracket => match other { Self::RightBracket => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::LeftParen => match other { Self::LeftParen => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::RightParen => match other { Self::RightParen => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Comma => match other { Self::Comma => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Semicolon => match other { Self::Semicolon => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Colon => match other { Self::Colon => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Star => match other { Self::Star => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Underscore => match other { Self::Underscore => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Dollar => match other { Self::Dollar => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Plus => match other { Self::Plus => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Minus => match other { Self::Minus => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Slash => match other { Self::Slash => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Hat => match other { Self::Hat => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Dot => match other { Self::Dot => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Eq => match other { Self::Eq => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::EqEq => match other { Self::EqEq => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::ExclEq => match other { Self::ExclEq => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Lt => match other { Self::Lt => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::LtEq => match other { Self::LtEq => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Gt => match other { Self::Gt => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::GtEq => match other { Self::GtEq => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::PlusEq => match other { Self::PlusEq => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::HyphEq => match other { Self::HyphEq => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::StarEq => match other { Self::StarEq => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::SlashEq => match other { Self::SlashEq => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Dots => match other { Self::Dots => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Arrow => match other { Self::Arrow => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Root => match other { Self::Root => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Bang => match other { Self::Bang => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Not => match other { Self::Not => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::And => match other { Self::And => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Or => match other { Self::Or => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::None => match other { Self::None => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Auto => match other { Self::Auto => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Let => match other { Self::Let => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Set => match other { Self::Set => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Show => match other { Self::Show => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Context => match other { Self::Context => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::If => match other { Self::If => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Else => match other { Self::Else => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::For => match other { Self::For => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::In => match other { Self::In => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::While => match other { Self::While => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Break => match other { Self::Break => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Continue => match other { Self::Continue => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Return => match other { Self::Return => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Import => match other { Self::Import => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Include => match other { Self::Include => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::As => match other { Self::As => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Code => match other { Self::Code => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Ident => match other { Self::Ident => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Bool => match other { Self::Bool => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Int => match other { Self::Int => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Float => match other { Self::Float => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Numeric => match other { Self::Numeric => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Str => match other { Self::Str => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::CodeBlock => match other { Self::CodeBlock => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::ContentBlock => match other { Self::ContentBlock => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Parenthesized => match other { Self::Parenthesized => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Array => match other { Self::Array => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Dict => match other { Self::Dict => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Named => match other { Self::Named => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Keyed => match other { Self::Keyed => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Unary => match other { Self::Unary => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Binary => match other { Self::Binary => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::FieldAccess => match other { Self::FieldAccess => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::FuncCall => match other { Self::FuncCall => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Args => match other { Self::Args => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Spread => match other { Self::Spread => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Closure => match other { Self::Closure => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Params => match other { Self::Params => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::LetBinding => match other { Self::LetBinding => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::SetRule => match other { Self::SetRule => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::ShowRule => match other { Self::ShowRule => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Contextual => match other { Self::Contextual => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Conditional => match other { Self::Conditional => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::WhileLoop => match other { Self::WhileLoop => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::ForLoop => match other { Self::ForLoop => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::ModuleImport => match other { Self::ModuleImport => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::ImportItems => match other { Self::ImportItems => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::ImportItemPath => match other { Self::ImportItemPath => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::RenamedImportItem => match other { Self::RenamedImportItem => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::ModuleInclude => match other { Self::ModuleInclude => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::LoopBreak => match other { Self::LoopBreak => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::LoopContinue => match other { Self::LoopContinue => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::FuncReturn => match other { Self::FuncReturn => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Destructuring => match other { Self::Destructuring => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::DestructAssignment => match other { Self::DestructAssignment => ObservationRelation::Same, _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::scope::Capturer { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Function => match other { Self::Function => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Context => match other { Self::Context => ObservationRelation::Same, _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::elements::image::ImageElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { path: _, data: _, width: _, height: _, fit: _ } = self; ObservationRelation::Same.and(self.path.observation_relation(&other.path)).and(self.data.observation_relation(&other.data)).and(self.width.observation_relation(&other.width)).and(self.height.observation_relation(&other.height)).and(self.fit.observation_relation(&other.fit)) } }
impl<T: ObservationEq> ObservationEq for crate::entities::ptr_eq_arc::PtrEqArc<T> { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self(_) = self; ObservationRelation::Same.and(self.0.observation_relation(&other.0)) } }
impl ObservationEq for crate::entities::elements::math_cases::MathCasesElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { rows: _, delim: _, reverse: _, gap: _ } = self; ObservationRelation::Same.and(self.rows.observation_relation(&other.rows)).and(self.delim.observation_relation(&other.delim)).and(self.reverse.observation_relation(&other.reverse)).and(self.gap.observation_relation(&other.gap)) } }
impl ObservationEq for crate::entities::elements::math_class_override::MathClassOverrideElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { class: _, body: _ } = self; ObservationRelation::Same.and(self.class.observation_relation(&other.class)).and(self.body.observation_relation(&other.body)) } }
impl ObservationEq for crate::entities::math_class::MathClass { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Normal => match other { Self::Normal => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Alphabetic => match other { Self::Alphabetic => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Binary => match other { Self::Binary => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Closing => match other { Self::Closing => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Diacritic => match other { Self::Diacritic => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Fence => match other { Self::Fence => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::GlyphPart => match other { Self::GlyphPart => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Large => match other { Self::Large => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Opening => match other { Self::Opening => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Punctuation => match other { Self::Punctuation => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Relation => match other { Self::Relation => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Space => match other { Self::Space => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Unary => match other { Self::Unary => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Vary => match other { Self::Vary => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Special => match other { Self::Special => ObservationRelation::Same, _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::elements::block::BlockElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { body: _, width: _, height: _, inset: _, breakable: _, outset: _, radius: _, clip: _, fill: _, stroke: _, spacing: _, above: _, below: _, sticky: _ } = self; ObservationRelation::Same.and(self.body.observation_relation(&other.body)).and(self.width.observation_relation(&other.width)).and(self.height.observation_relation(&other.height)).and(self.inset.observation_relation(&other.inset)).and(self.breakable.observation_relation(&other.breakable)).and(self.outset.observation_relation(&other.outset)).and(self.radius.observation_relation(&other.radius)).and(self.clip.observation_relation(&other.clip)).and(self.fill.observation_relation(&other.fill)).and(self.stroke.observation_relation(&other.stroke)).and(self.spacing.observation_relation(&other.spacing)).and(self.above.observation_relation(&other.above)).and(self.below.observation_relation(&other.below)).and(self.sticky.observation_relation(&other.sticky)) } }
impl ObservationEq for crate::entities::elements::repeat::RepeatElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { body: _, gap: _, justify: _ } = self; ObservationRelation::Same.and(self.body.observation_relation(&other.body)).and(self.gap.observation_relation(&other.gap)).and(self.justify.observation_relation(&other.justify)) } }
impl ObservationEq for crate::entities::elements::state_update::StateUpdateElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { key: _, update: _ } = self; ObservationRelation::Same.and(self.key.observation_relation(&other.key)).and(self.update.observation_relation(&other.update)) } }
impl ObservationEq for crate::entities::state_update::StateUpdate { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Set(a0) => match other { Self::Set(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Func(a0) => match other { Self::Func(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::elements::link::LinkElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { url: _, body: _ } = self; ObservationRelation::Same.and(self.url.observation_relation(&other.url)).and(self.body.observation_relation(&other.body)) } }
impl ObservationEq for crate::entities::elements::place::PlaceElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { alignment: _, dx: _, dy: _, scope: _, float: _, clearance: _, body: _ } = self; ObservationRelation::Same.and(self.alignment.observation_relation(&other.alignment)).and(self.dx.observation_relation(&other.dx)).and(self.dy.observation_relation(&other.dy)).and(self.scope.observation_relation(&other.scope)).and(self.float.observation_relation(&other.float)).and(self.clearance.observation_relation(&other.clearance)).and(self.body.observation_relation(&other.body)) } }
impl ObservationEq for crate::entities::layout_types::PlaceScope { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Column => match other { Self::Column => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Parent => match other { Self::Parent => ObservationRelation::Same, _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::elements::counter_display_callback::CounterDisplayCallbackElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { key: _, callback: _ } = self; ObservationRelation::Same.and(self.key.observation_relation(&other.key)).and(self.callback.observation_relation(&other.callback)) } }
impl ObservationEq for crate::entities::elements::table_header::TableHeaderElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { body: _, repeat: _ } = self; ObservationRelation::Same.and(self.body.observation_relation(&other.body)).and(self.repeat.observation_relation(&other.repeat)) } }
impl ObservationEq for crate::entities::elements::title::TitleElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { body: _ } = self; ObservationRelation::Same.and(self.body.observation_relation(&other.body)) } }
impl ObservationEq for crate::entities::elements::emph::EmphElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { body: _ } = self; ObservationRelation::Same.and(self.body.observation_relation(&other.body)) } }
impl ObservationEq for crate::entities::elements::quote::QuoteElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { body: _, attribution: _, block: _, quotes: _ } = self; ObservationRelation::Same.and(self.body.observation_relation(&other.body)).and(self.attribution.observation_relation(&other.attribution)).and(self.block.observation_relation(&other.block)).and(self.quotes.observation_relation(&other.quotes)) } }
impl ObservationEq for crate::entities::elements::flush::FlushElem { fn observation_relation(&self, _: &Self) -> ObservationRelation { let Self = self; ObservationRelation::Same } }
impl ObservationEq for crate::entities::elements::stack::StackElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { children: _, dir: _, spacing: _ } = self; ObservationRelation::Same.and(self.children.observation_relation(&other.children)).and(self.dir.observation_relation(&other.dir)).and(self.spacing.observation_relation(&other.spacing)) } }
impl ObservationEq for crate::entities::dir::Dir { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::LTR => match other { Self::LTR => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::RTL => match other { Self::RTL => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::TTB => match other { Self::TTB => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::BTT => match other { Self::BTT => ObservationRelation::Same, _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::elements::outline::OutlineElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { title: _, depth: _, indent: _, target: _ } = self; ObservationRelation::Same.and(self.title.observation_relation(&other.title)).and(self.depth.observation_relation(&other.depth)).and(self.indent.observation_relation(&other.indent)).and(self.target.observation_relation(&other.target)) } }
impl ObservationEq for crate::entities::elements::outline::OutlineTarget { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Headings => match other { Self::Headings => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Figures => match other { Self::Figures => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Tables => match other { Self::Tables => ObservationRelation::Same, _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::elements::outline::OutlineIndent { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Auto => match other { Self::Auto => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Bool(a0) => match other { Self::Bool(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Length(a0) => match other { Self::Length(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Function(a0) => match other { Self::Function(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::elements::transform::TransformElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { matrix: _, body: _ } = self; ObservationRelation::Same.and(self.matrix.observation_relation(&other.matrix)).and(self.body.observation_relation(&other.body)) } }
impl ObservationEq for crate::entities::layout_types::TransformMatrix { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { a: _, b: _, c: _, d: _, tx: _, ty: _ } = self; ObservationRelation::Same.and(self.a.observation_relation(&other.a)).and(self.b.observation_relation(&other.b)).and(self.c.observation_relation(&other.c)).and(self.d.observation_relation(&other.d)).and(self.tx.observation_relation(&other.tx)).and(self.ty.observation_relation(&other.ty)) } }
impl ObservationEq for crate::entities::elements::math_underover::MathUnderoverElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { base: _, under: _, over: _ } = self; ObservationRelation::Same.and(self.base.observation_relation(&other.base)).and(self.under.observation_relation(&other.under)).and(self.over.observation_relation(&other.over)) } }
impl ObservationEq for crate::entities::elements::list_item::ListItemElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { body: _, marker: _, marker_align: _, indent: _, body_indent: _, tight: _ } = self; ObservationRelation::Same.and(self.body.observation_relation(&other.body)).and(self.marker.observation_relation(&other.marker)).and(self.marker_align.observation_relation(&other.marker_align)).and(self.indent.observation_relation(&other.indent)).and(self.body_indent.observation_relation(&other.body_indent)).and(self.tight.observation_relation(&other.tight)) } }
impl ObservationEq for crate::entities::list_marker::ListMarker { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Default => match other { Self::Default => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Custom(a0) => match other { Self::Custom(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Array(a0) => match other { Self::Array(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::elements::boxed::BoxedElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { body: _, width: _, height: _, inset: _, baseline: _, outset: _, radius: _, clip: _, fill: _, stroke: _ } = self; ObservationRelation::Same.and(self.body.observation_relation(&other.body)).and(self.width.observation_relation(&other.width)).and(self.height.observation_relation(&other.height)).and(self.inset.observation_relation(&other.inset)).and(self.baseline.observation_relation(&other.baseline)).and(self.outset.observation_relation(&other.outset)).and(self.radius.observation_relation(&other.radius)).and(self.clip.observation_relation(&other.clip)).and(self.fill.observation_relation(&other.fill)).and(self.stroke.observation_relation(&other.stroke)) } }
impl ObservationEq for crate::entities::elements::cite::CiteElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { key: _, supplement: _, form: _, style: _ } = self; ObservationRelation::Same.and(self.key.observation_relation(&other.key)).and(self.supplement.observation_relation(&other.supplement)).and(self.form.observation_relation(&other.form)).and(self.style.observation_relation(&other.style)) } }
impl ObservationEq for crate::entities::citation_style::CitationStyle { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::AuthorDate => match other { Self::AuthorDate => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Numeric => match other { Self::Numeric => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Alphabetic => match other { Self::Alphabetic => ObservationRelation::Same, _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::citation_form::CitationForm { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Normal => match other { Self::Normal => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Prose => match other { Self::Prose => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Author => match other { Self::Author => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Year => match other { Self::Year => ObservationRelation::Same, _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::elements::hide::HideElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { body: _ } = self; ObservationRelation::Same.and(self.body.observation_relation(&other.body)) } }
impl ObservationEq for crate::entities::elements::grid_cell::GridCellElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { body: _, x: _, y: _, colspan: _, rowspan: _, stroke: _, fill: _, align: _, inset: _, breakable: _ } = self; ObservationRelation::Same.and(self.body.observation_relation(&other.body)).and(self.x.observation_relation(&other.x)).and(self.y.observation_relation(&other.y)).and(self.colspan.observation_relation(&other.colspan)).and(self.rowspan.observation_relation(&other.rowspan)).and(self.stroke.observation_relation(&other.stroke)).and(self.fill.observation_relation(&other.fill)).and(self.align.observation_relation(&other.align)).and(self.inset.observation_relation(&other.inset)).and(self.breakable.observation_relation(&other.breakable)) } }
impl ObservationEq for crate::entities::elements::math_cancel::MathCancelElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { body: _, length: _, inverted: _, cross: _, angle: _, stroke: _, background: _, span: _, explicit: _ } = self; ObservationRelation::Same.and(self.body.observation_relation(&other.body)).and(self.length.observation_relation(&other.length)).and(self.inverted.observation_relation(&other.inverted)).and(self.cross.observation_relation(&other.cross)).and(self.angle.observation_relation(&other.angle)).and(self.stroke.observation_relation(&other.stroke)).and(self.background.observation_relation(&other.background)).and(self.span.observation_relation(&other.span)).and(self.explicit.observation_relation(&other.explicit)) } }
impl ObservationEq for crate::entities::elements::math_cancel::MathCancelExplicit { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { length: _, inverted: _, cross: _, angle: _, stroke: _, background: _ } = self; ObservationRelation::Same.and(self.length.observation_relation(&other.length)).and(self.inverted.observation_relation(&other.inverted)).and(self.cross.observation_relation(&other.cross)).and(self.angle.observation_relation(&other.angle)).and(self.stroke.observation_relation(&other.stroke)).and(self.background.observation_relation(&other.background)) } }
impl ObservationEq for crate::entities::elements::math_cancel::MathCancelAngle { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Auto => match other { Self::Auto => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Angle(a0) => match other { Self::Angle(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Func(a0) => match other { Self::Func(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::elements::grid_footer::GridFooterElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { body: _, repeat: _ } = self; ObservationRelation::Same.and(self.body.observation_relation(&other.body)).and(self.repeat.observation_relation(&other.repeat)) } }
impl ObservationEq for crate::entities::elements::math_align_point::MathAlignPointElem { fn observation_relation(&self, _: &Self) -> ObservationRelation { let Self = self; ObservationRelation::Same } }
impl ObservationEq for crate::entities::elements::metadata::MetadataElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { value: _ } = self; ObservationRelation::Same.and(self.value.observation_relation(&other.value)) } }
impl ObservationEq for crate::entities::elements::terms::TermsElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { items: _ } = self; ObservationRelation::Same.and(self.items.observation_relation(&other.items)) } }
impl ObservationEq for crate::entities::elements::state_display::StateDisplayElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { key: _, callback: _ } = self; ObservationRelation::Same.and(self.key.observation_relation(&other.key)).and(self.callback.observation_relation(&other.callback)) } }
impl ObservationEq for crate::entities::elements::state::StateElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { key: _, init: _ } = self; ObservationRelation::Same.and(self.key.observation_relation(&other.key)).and(self.init.observation_relation(&other.init)) } }
impl ObservationEq for crate::entities::elements::pagebreak::PagebreakElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { weak: _, weak_explicit: _, to: _ } = self; ObservationRelation::Same.and(self.weak.observation_relation(&other.weak)).and(self.weak_explicit.observation_relation(&other.weak_explicit)).and(self.to.observation_relation(&other.to)) } }
impl ObservationEq for crate::entities::parity::Parity { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Even => match other { Self::Even => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Odd => match other { Self::Odd => ObservationRelation::Same, _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::elements::figure::FigureElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { body: _, caption: _, kind: _ } = self; ObservationRelation::Same.and(self.body.observation_relation(&other.body)).and(self.caption.observation_relation(&other.caption)).and(self.kind.observation_relation(&other.kind)) } }
impl ObservationEq for crate::entities::elements::curve::CurveElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { segments: _ } = self; ObservationRelation::Same.and(self.segments.observation_relation(&other.segments)) } }
impl ObservationEq for crate::entities::elements::curve::CurveSegment { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Move(a0) => match other { Self::Move(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Line(a0) => match other { Self::Line(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Cubic(a0, a1, a2) => match other { Self::Cubic(b0, b1, b2) => ObservationRelation::Same.and(a0.observation_relation(b0)).and(a1.observation_relation(b1)).and(a2.observation_relation(b2)), _ => ObservationRelation::Different },
Self::Quad(a0, a1) => match other { Self::Quad(b0, b1) => ObservationRelation::Same.and(a0.observation_relation(b0)).and(a1.observation_relation(b1)), _ => ObservationRelation::Different },
Self::Close(a0) => match other { Self::Close(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::elements::curve::CurvePoint { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { x: _, y: _ } = self; ObservationRelation::Same.and(self.x.observation_relation(&other.x)).and(self.y.observation_relation(&other.y)) } }
impl ObservationEq for crate::entities::elements::curve::CloseMode { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Smooth => match other { Self::Smooth => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Straight => match other { Self::Straight => ObservationRelation::Same, _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::elements::math_root::MathRootElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { index: _, radicand: _ } = self; ObservationRelation::Same.and(self.index.observation_relation(&other.index)).and(self.radicand.observation_relation(&other.radicand)) } }
impl ObservationEq for crate::entities::elements::math_accent::MathAccentElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { base: _, accent: _ } = self; ObservationRelation::Same.and(self.base.observation_relation(&other.base)).and(self.accent.observation_relation(&other.accent)) } }
impl ObservationEq for crate::entities::elements::math_attach::MathAttachElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { base: _, t: _, b: _, tl: _, bl: _, tr: _, br: _ } = self; ObservationRelation::Same.and(self.base.observation_relation(&other.base)).and(self.t.observation_relation(&other.t)).and(self.b.observation_relation(&other.b)).and(self.tl.observation_relation(&other.tl)).and(self.bl.observation_relation(&other.bl)).and(self.tr.observation_relation(&other.tr)).and(self.br.observation_relation(&other.br)) } }
impl ObservationEq for crate::entities::elements::math_attach::MathAttachSlot { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Omitted => match other { Self::Omitted => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::ExplicitNone => match other { Self::ExplicitNone => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Present(a0) => match other { Self::Present(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::elements::strike::StrikeElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { body: _, stroke: _, offset: _, extent: _ } = self; ObservationRelation::Same.and(self.body.observation_relation(&other.body)).and(self.stroke.observation_relation(&other.stroke)).and(self.offset.observation_relation(&other.offset)).and(self.extent.observation_relation(&other.extent)) } }
impl ObservationEq for crate::entities::elements::divider::DividerElem { fn observation_relation(&self, _: &Self) -> ObservationRelation { let Self = self; ObservationRelation::Same } }
impl ObservationEq for crate::entities::elements::linebreak::LinebreakElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { justify: _, justify_explicit: _ } = self; ObservationRelation::Same.and(self.justify.observation_relation(&other.justify)).and(self.justify_explicit.observation_relation(&other.justify_explicit)) } }
impl ObservationEq for crate::entities::elements::pad::PadElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { body: _, sides: _ } = self; ObservationRelation::Same.and(self.body.observation_relation(&other.body)).and(self.sides.observation_relation(&other.sides)) } }
impl ObservationEq for crate::entities::elements::heading::HeadingElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { level: _, body: _, outlined: _, bookmarked: _, set_fields: _ } = self; ObservationRelation::Same.and(self.level.observation_relation(&other.level)).and(self.body.observation_relation(&other.body)).and(self.outlined.observation_relation(&other.outlined)).and(self.bookmarked.observation_relation(&other.bookmarked)).and(self.set_fields.observation_relation(&other.set_fields)) } }
impl ObservationEq for crate::entities::elements::counter_display::CounterDisplayElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { kind: _ } = self; ObservationRelation::Same.and(self.kind.observation_relation(&other.kind)) } }
impl ObservationEq for crate::entities::elements::table_cell::TableCellElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { body: _, x: _, y: _, colspan: _, rowspan: _, stroke: _, fill: _, align: _, inset: _, breakable: _, kind: _ } = self; ObservationRelation::Same.and(self.body.observation_relation(&other.body)).and(self.x.observation_relation(&other.x)).and(self.y.observation_relation(&other.y)).and(self.colspan.observation_relation(&other.colspan)).and(self.rowspan.observation_relation(&other.rowspan)).and(self.stroke.observation_relation(&other.stroke)).and(self.fill.observation_relation(&other.fill)).and(self.align.observation_relation(&other.align)).and(self.inset.observation_relation(&other.inset)).and(self.breakable.observation_relation(&other.breakable)).and(self.kind.observation_relation(&other.kind)) } }
impl ObservationEq for crate::entities::elements::table_cell::TableCellKind { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Auto => match other { Self::Auto => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Header { level: a0, scope: a1 } => match other { Self::Header { level: b0, scope: b1 } => ObservationRelation::Same.and(a0.observation_relation(b0)).and(a1.observation_relation(b1)), _ => ObservationRelation::Different },
Self::Data => match other { Self::Data => ObservationRelation::Same, _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::elements::table_cell::TableHeaderScope { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Both => match other { Self::Both => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Column => match other { Self::Column => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Row => match other { Self::Row => ObservationRelation::Same, _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::elements::overline::OverlineElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { body: _, stroke: _, offset: _, extent: _ } = self; ObservationRelation::Same.and(self.body.observation_relation(&other.body)).and(self.stroke.observation_relation(&other.stroke)).and(self.offset.observation_relation(&other.offset)).and(self.extent.observation_relation(&other.extent)) } }
impl ObservationEq for crate::entities::elements::term_item::TermItemElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { term: _, description: _ } = self; ObservationRelation::Same.and(self.term.observation_relation(&other.term)).and(self.description.observation_relation(&other.description)) } }
impl ObservationEq for crate::entities::elements::colbreak::ColbreakElem { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { weak: _, weak_explicit: _ } = self; ObservationRelation::Same.and(self.weak.observation_relation(&other.weak)).and(self.weak_explicit.observation_relation(&other.weak_explicit)) } }
impl ObservationEq for crate::entities::document_info::DocumentInfo { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { title: _, author: _, keywords: _ } = self; ObservationRelation::Same.and(self.title.observation_relation(&other.title)).and(self.author.observation_relation(&other.author)).and(self.keywords.observation_relation(&other.keywords)) } }
impl ObservationEq for crate::entities::counter::Counter { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { key: _ } = self; ObservationRelation::Same.and(self.key.observation_relation(&other.key)) } }
impl ObservationEq for crate::entities::state::State { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { key: _, init: _ } = self; ObservationRelation::Same.and(self.key.observation_relation(&other.key)).and(self.init.observation_relation(&other.init)) } }
impl ObservationEq for crate::entities::value::Type { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::None => match other { Self::None => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Auto => match other { Self::Auto => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Bool => match other { Self::Bool => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Int => match other { Self::Int => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Float => match other { Self::Float => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Str => match other { Self::Str => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Array => match other { Self::Array => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Dictionary => match other { Self::Dictionary => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Module => match other { Self::Module => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Datetime => match other { Self::Datetime => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Function => match other { Self::Function => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Content => match other { Self::Content => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Length => match other { Self::Length => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Ratio => match other { Self::Ratio => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Relative => match other { Self::Relative => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Angle => match other { Self::Angle => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Color => match other { Self::Color => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Stroke => match other { Self::Stroke => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Fraction => match other { Self::Fraction => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Alignment => match other { Self::Alignment => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Location => match other { Self::Location => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Gradient => match other { Self::Gradient => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Regex => match other { Self::Regex => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Tiling => match other { Self::Tiling => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Bytes => match other { Self::Bytes => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Decimal => match other { Self::Decimal => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Duration => match other { Self::Duration => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Version => match other { Self::Version => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Selector => match other { Self::Selector => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Symbol => match other { Self::Symbol => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Arguments => match other { Self::Arguments => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::State => match other { Self::State => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Counter => match other { Self::Counter => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Label => match other { Self::Label => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Direction => match other { Self::Direction => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Path => match other { Self::Path => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Type => match other { Self::Type => ObservationRelation::Same, _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::gradient::GradientStop { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { color: _, offset: _ } = self; ObservationRelation::Same.and(self.color.observation_relation(&other.color)).and(self.offset.observation_relation(&other.offset)) } }
impl ObservationEq for crate::entities::gradient::RelativeTo { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Self_ => match other { Self::Self_ => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Parent => match other { Self::Parent => ObservationRelation::Same, _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::gradient::Linear { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { stops: _, angle: _, space: _, relative: _, anti_alias: _ } = self; ObservationRelation::Same.and(self.stops.observation_relation(&other.stops)).and(self.angle.observation_relation(&other.angle)).and(self.space.observation_relation(&other.space)).and(self.relative.observation_relation(&other.relative)).and(self.anti_alias.observation_relation(&other.anti_alias)) } }
impl ObservationEq for crate::entities::gradient::Radial { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { stops: _, center: _, radius: _, focal_center: _, focal_radius: _, space: _, relative: _, anti_alias: _ } = self; ObservationRelation::Same.and(self.stops.observation_relation(&other.stops)).and(self.center.observation_relation(&other.center)).and(self.radius.observation_relation(&other.radius)).and(self.focal_center.observation_relation(&other.focal_center)).and(self.focal_radius.observation_relation(&other.focal_radius)).and(self.space.observation_relation(&other.space)).and(self.relative.observation_relation(&other.relative)).and(self.anti_alias.observation_relation(&other.anti_alias)) } }
impl ObservationEq for crate::entities::gradient::Conic { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { stops: _, center: _, angle: _, space: _, relative: _, anti_alias: _ } = self; ObservationRelation::Same.and(self.stops.observation_relation(&other.stops)).and(self.center.observation_relation(&other.center)).and(self.angle.observation_relation(&other.angle)).and(self.space.observation_relation(&other.space)).and(self.relative.observation_relation(&other.relative)).and(self.anti_alias.observation_relation(&other.anti_alias)) } }
impl ObservationEq for crate::entities::gradient::Gradient { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Linear(a0) => match other { Self::Linear(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Radial(a0) => match other { Self::Radial(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Conic(a0) => match other { Self::Conic(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::tiling::Tiling { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { body: _, size: _, relative: _, spacing: _, offset: _, angle: _ } = self; ObservationRelation::Same.and(self.body.observation_relation(&other.body)).and(self.size.observation_relation(&other.size)).and(self.relative.observation_relation(&other.relative)).and(self.spacing.observation_relation(&other.spacing)).and(self.offset.observation_relation(&other.offset)).and(self.angle.observation_relation(&other.angle)) } }
impl ObservationEq for crate::entities::tiling::TilingBody { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Content(a0) => match other { Self::Content(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Image(a0) => match other { Self::Image(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Gradient(a0) => match other { Self::Gradient(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Color(a0) => match other { Self::Color(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::tiling::TilingRelative { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Auto => match other { Self::Auto => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Itself => match other { Self::Itself => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Parent => match other { Self::Parent => ObservationRelation::Same, _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::counter_update::CounterUpdate { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Set(a0) => match other { Self::Set(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Step(a0) => match other { Self::Step(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
Self::Func(a0) => match other { Self::Func(b0) => ObservationRelation::Same.and(a0.observation_relation(b0)), _ => ObservationRelation::Different },
} } }
impl ObservationEq for crate::entities::duration::Duration { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { nanos: _ } = self; ObservationRelation::Same.and(self.nanos.observation_relation(&other.nanos)) } }
impl ObservationEq for crate::entities::symbol::Symbol { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { value: _, name: _, variants: _, applied: _ } = self; ObservationRelation::Same.and(self.value.observation_relation(&other.value)).and(self.name.observation_relation(&other.name)).and(self.variants.observation_relation(&other.variants)).and(self.applied.observation_relation(&other.applied)) } }
impl ObservationEq for crate::entities::layout_types::Ratio { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self(_) = self; ObservationRelation::Same.and(self.0.observation_relation(&other.0)) } }
impl ObservationEq for crate::entities::color::ColorSpace { fn observation_relation(&self, other: &Self) -> ObservationRelation { match self {
Self::Oklab => match other { Self::Oklab => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Oklch => match other { Self::Oklch => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Srgb => match other { Self::Srgb => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Luma => match other { Self::Luma => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::LinearRgb => match other { Self::LinearRgb => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Hsl => match other { Self::Hsl => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Hsv => match other { Self::Hsv => ObservationRelation::Same, _ => ObservationRelation::Different },
Self::Cmyk => match other { Self::Cmyk => ObservationRelation::Same, _ => ObservationRelation::Different },
} } }
impl<T: ObservationEq> ObservationEq for crate::entities::axes::Axes<T> { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { x: _, y: _ } = self; ObservationRelation::Same.and(self.x.observation_relation(&other.x)).and(self.y.observation_relation(&other.y)) } }
impl ObservationEq for crate::entities::layout_types::Size { fn observation_relation(&self, other: &Self) -> ObservationRelation { let Self { width: _, height: _ } = self; ObservationRelation::Same.and(self.width.observation_relation(&other.width)).and(self.height.observation_relation(&other.height)) } }
