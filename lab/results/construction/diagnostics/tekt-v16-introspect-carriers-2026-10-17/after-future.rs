#![allow(dead_code,unused_variables,unreachable_patterns)]
enum Content{Source(()),ShowRecipe(()),Realized(()),Located(()),Empty,Text(()),Space,SpaceFromNewline,Parbreak,Sequence(()),HtmlElem(()),HtmlFrame(()),Par{body:()},Heading(()),Title(()),Strong(()),Emph(()),Raw(()),ListItem(()),EnumItem(()),Link(()),Equation(()),MathSequence(()),MathIdent(()),MathText(()),MathFrac(()),MathAttach(()),MathPrimes(()),MathRoot(()),MathStretch(()),MathDelimited(()),MathLr(()),MathMid(()),MathOverbrace(()),MathUnderbrace(()),MathUnderbracket(()),MathUnderparen(()),MathUndershell(()),MathOverbracket(()),MathOverparen(()),MathOvershell(()),MathAlignPoint(()),Linebreak(()),MathMatrix(()),MathCases(()),MathAccent(()),MathCancel(()),MathUnderline(()),MathOverline(()),MathVec(()),MathClassOverride(()),MathLimitsOverride(()),MathUnderover(()),MathOp(()),MathStyled(()),Label(()),Ref(()),CounterDisplay(()),CounterUpdate(()),Outline(()),OutlineEntry(()),Figure(()),FigureCaption(()),Image(()),Shape(()),Curve(()),Transform(()),Grid(()),GridHeader(()),GridFooter(()),GridCell(()),SetPage(()),PageRun(()),Align(()),Place(()),Flush(()),Styled(()),Divider(()),Terms(()),TermItem(()),Quote(()),Document{path:(),body:(),format:(),title:(),author:(),date:(),keywords:()},Asset{path:(),data:()},SmartQuote(()),PdfAttach(()),PdfArtifact(()),Underline(()),Strike(()),Overline(()),SmallCaps{body:()},Pad(()),Hide(()),HSpace(()),VSpace(()),Pagebreak(()),Colbreak(()),Stack(()),Boxed(()),Block(()),TableCell(()),Bibliography(()),Cite(()),Footnote(()),FootnoteEntry(()),TableHeader(()),TableFooter(()),GridHLine(()),GridVLine(()),TableHLine(()),TableVLine(()),Table(()),Repeat(()),Columns(()),Metadata(()),State(()),StateUpdate(()),StateDisplay(()),CounterDisplayCallback(()),ContextBlock(()),Dynamic(()),TektIntrospectFuture}
enum ElementPayload{NativeElement,Heading{depth:(),body_hash:(),counter_update:(),numbering_active:bool},Figure{kind:(),counter_update:(),is_counted:bool,caption_text:()},Citation{key:()},Metadata{value:()},State{key:(),init:()},StateUpdate{key:(),update:()},StateDisplay{key:(),callback:()},CounterDisplay{key:(),callback:()},Outline,Bibliography{entries:()},Equation{block:bool,counter_update:(),numbering_active:bool,numbering_pattern:(),numbering_callback:(),supplement:(),supplement_lang:(),number_align:(),alt:()},Labelled{label:(),resolved_text:(),figure_number:()},CounterUpdate{key:(),action:()},Table{counter_update:(),is_counted:bool,caption_text:()},Footnote{counter_update:()},HeadingForToc{label:(),number:(),body:(),level:()},HeadingForBookmarks{label:(),number:(),body:(),level:()},ContextBlock{id:()},TektIntrospectFuture}
fn p0(input:&Content,gate:bool)->usize{match input{Content::Source(e)=>20000,Content::Heading(_)=>0,Content::Equation(e) if gate=>1,Content::Figure(e)=>2,Content::ShowRecipe(..)
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
        | Content::Table(..)
        | Content::Repeat(..)
        | Content::Columns(..)
        | Content::Metadata(..)
        | Content::State(..)
        | Content::StateUpdate(..)
        | Content::StateDisplay(..)
        | Content::CounterDisplayCallback(..)
        | Content::ContextBlock(..)
        | Content::Dynamic(..)=>9999,}}
fn p1(input:&ElementPayload,gate:bool)->usize{match input{ElementPayload::NativeElement
        | ElementPayload::Footnote { .. }
        | ElementPayload::Citation { .. }=>0,ElementPayload::Heading { depth, numbering_active: true, .. }=>1,ElementPayload::Figure { is_counted: true, counter_update, .. }
        | ElementPayload::Table { is_counted: true, counter_update, .. }=>2,ElementPayload::Equation { block: true, numbering_active: true, .. }=>3,ElementPayload::Metadata { .. }
        | ElementPayload::State { .. }
        | ElementPayload::StateUpdate { .. }
        | ElementPayload::StateDisplay { .. }
        | ElementPayload::CounterDisplay { .. }
        | ElementPayload::Outline
        | ElementPayload::Bibliography { .. }
        | ElementPayload::Labelled { .. }
        | ElementPayload::CounterUpdate { .. }
        | ElementPayload::HeadingForToc { .. }
        | ElementPayload::HeadingForBookmarks { .. }
        | ElementPayload::ContextBlock { .. }
        | ElementPayload::Heading { numbering_active: false, .. }
        | ElementPayload::Figure { is_counted: false, .. }
        | ElementPayload::Table { is_counted: false, .. }
        | ElementPayload::Equation { block: false, .. }
        | ElementPayload::Equation { block: true, numbering_active: false, .. }=>9999,}}
fn main(){for(name,c)in [("Source",Content::Source(())),("ShowRecipe",Content::ShowRecipe(())),("Realized",Content::Realized(())),("Located",Content::Located(())),("Empty",Content::Empty),("Text",Content::Text(())),("Space",Content::Space),("SpaceFromNewline",Content::SpaceFromNewline),("Parbreak",Content::Parbreak),("Sequence",Content::Sequence(())),("HtmlElem",Content::HtmlElem(())),("HtmlFrame",Content::HtmlFrame(())),("Par",Content::Par{body:()}),("Heading",Content::Heading(())),("Title",Content::Title(())),("Strong",Content::Strong(())),("Emph",Content::Emph(())),("Raw",Content::Raw(())),("ListItem",Content::ListItem(())),("EnumItem",Content::EnumItem(())),("Link",Content::Link(())),("Equation",Content::Equation(())),("MathSequence",Content::MathSequence(())),("MathIdent",Content::MathIdent(())),("MathText",Content::MathText(())),("MathFrac",Content::MathFrac(())),("MathAttach",Content::MathAttach(())),("MathPrimes",Content::MathPrimes(())),("MathRoot",Content::MathRoot(())),("MathStretch",Content::MathStretch(())),("MathDelimited",Content::MathDelimited(())),("MathLr",Content::MathLr(())),("MathMid",Content::MathMid(())),("MathOverbrace",Content::MathOverbrace(())),("MathUnderbrace",Content::MathUnderbrace(())),("MathUnderbracket",Content::MathUnderbracket(())),("MathUnderparen",Content::MathUnderparen(())),("MathUndershell",Content::MathUndershell(())),("MathOverbracket",Content::MathOverbracket(())),("MathOverparen",Content::MathOverparen(())),("MathOvershell",Content::MathOvershell(())),("MathAlignPoint",Content::MathAlignPoint(())),("Linebreak",Content::Linebreak(())),("MathMatrix",Content::MathMatrix(())),("MathCases",Content::MathCases(())),("MathAccent",Content::MathAccent(())),("MathCancel",Content::MathCancel(())),("MathUnderline",Content::MathUnderline(())),("MathOverline",Content::MathOverline(())),("MathVec",Content::MathVec(())),("MathClassOverride",Content::MathClassOverride(())),("MathLimitsOverride",Content::MathLimitsOverride(())),("MathUnderover",Content::MathUnderover(())),("MathOp",Content::MathOp(())),("MathStyled",Content::MathStyled(())),("Label",Content::Label(())),("Ref",Content::Ref(())),("CounterDisplay",Content::CounterDisplay(())),("CounterUpdate",Content::CounterUpdate(())),("Outline",Content::Outline(())),("OutlineEntry",Content::OutlineEntry(())),("Figure",Content::Figure(())),("FigureCaption",Content::FigureCaption(())),("Image",Content::Image(())),("Shape",Content::Shape(())),("Curve",Content::Curve(())),("Transform",Content::Transform(())),("Grid",Content::Grid(())),("GridHeader",Content::GridHeader(())),("GridFooter",Content::GridFooter(())),("GridCell",Content::GridCell(())),("SetPage",Content::SetPage(())),("PageRun",Content::PageRun(())),("Align",Content::Align(())),("Place",Content::Place(())),("Flush",Content::Flush(())),("Styled",Content::Styled(())),("Divider",Content::Divider(())),("Terms",Content::Terms(())),("TermItem",Content::TermItem(())),("Quote",Content::Quote(())),("Document",Content::Document{path:(),body:(),format:(),title:(),author:(),date:(),keywords:()}),("Asset",Content::Asset{path:(),data:()}),("SmartQuote",Content::SmartQuote(())),("PdfAttach",Content::PdfAttach(())),("PdfArtifact",Content::PdfArtifact(())),("Underline",Content::Underline(())),("Strike",Content::Strike(())),("Overline",Content::Overline(())),("SmallCaps",Content::SmallCaps{body:()}),("Pad",Content::Pad(())),("Hide",Content::Hide(())),("HSpace",Content::HSpace(())),("VSpace",Content::VSpace(())),("Pagebreak",Content::Pagebreak(())),("Colbreak",Content::Colbreak(())),("Stack",Content::Stack(())),("Boxed",Content::Boxed(())),("Block",Content::Block(())),("TableCell",Content::TableCell(())),("Bibliography",Content::Bibliography(())),("Cite",Content::Cite(())),("Footnote",Content::Footnote(())),("FootnoteEntry",Content::FootnoteEntry(())),("TableHeader",Content::TableHeader(())),("TableFooter",Content::TableFooter(())),("GridHLine",Content::GridHLine(())),("GridVLine",Content::GridVLine(())),("TableHLine",Content::TableHLine(())),("TableVLine",Content::TableVLine(())),("Table",Content::Table(())),("Repeat",Content::Repeat(())),("Columns",Content::Columns(())),("Metadata",Content::Metadata(())),("State",Content::State(())),("StateUpdate",Content::StateUpdate(())),("StateDisplay",Content::StateDisplay(())),("CounterDisplayCallback",Content::CounterDisplayCallback(())),("ContextBlock",Content::ContextBlock(())),("Dynamic",Content::Dynamic(()))]{for gate in [false,true]{println!("0/{name}/{gate}: {}",p0(&c,gate));}}for(name,c)in [("NativeElement",ElementPayload::NativeElement),("Heading;numbering_active=0",ElementPayload::Heading{depth:(),body_hash:(),counter_update:(),numbering_active:false}),("Heading;numbering_active=1",ElementPayload::Heading{depth:(),body_hash:(),counter_update:(),numbering_active:true}),("Figure;is_counted=0",ElementPayload::Figure{kind:(),counter_update:(),is_counted:false,caption_text:()}),("Figure;is_counted=1",ElementPayload::Figure{kind:(),counter_update:(),is_counted:true,caption_text:()}),("Citation",ElementPayload::Citation{key:()}),("Metadata",ElementPayload::Metadata{value:()}),("State",ElementPayload::State{key:(),init:()}),("StateUpdate",ElementPayload::StateUpdate{key:(),update:()}),("StateDisplay",ElementPayload::StateDisplay{key:(),callback:()}),("CounterDisplay",ElementPayload::CounterDisplay{key:(),callback:()}),("Outline",ElementPayload::Outline),("Bibliography",ElementPayload::Bibliography{entries:()}),("Equation;block=0;numbering_active=0",ElementPayload::Equation{block:false,counter_update:(),numbering_active:false,numbering_pattern:(),numbering_callback:(),supplement:(),supplement_lang:(),number_align:(),alt:()}),("Equation;block=0;numbering_active=1",ElementPayload::Equation{block:false,counter_update:(),numbering_active:true,numbering_pattern:(),numbering_callback:(),supplement:(),supplement_lang:(),number_align:(),alt:()}),("Equation;block=1;numbering_active=0",ElementPayload::Equation{block:true,counter_update:(),numbering_active:false,numbering_pattern:(),numbering_callback:(),supplement:(),supplement_lang:(),number_align:(),alt:()}),("Equation;block=1;numbering_active=1",ElementPayload::Equation{block:true,counter_update:(),numbering_active:true,numbering_pattern:(),numbering_callback:(),supplement:(),supplement_lang:(),number_align:(),alt:()}),("Labelled",ElementPayload::Labelled{label:(),resolved_text:(),figure_number:()}),("CounterUpdate",ElementPayload::CounterUpdate{key:(),action:()}),("Table;is_counted=0",ElementPayload::Table{counter_update:(),is_counted:false,caption_text:()}),("Table;is_counted=1",ElementPayload::Table{counter_update:(),is_counted:true,caption_text:()}),("Footnote",ElementPayload::Footnote{counter_update:()}),("HeadingForToc",ElementPayload::HeadingForToc{label:(),number:(),body:(),level:()}),("HeadingForBookmarks",ElementPayload::HeadingForBookmarks{label:(),number:(),body:(),level:()}),("ContextBlock",ElementPayload::ContextBlock{id:()})]{for gate in [false,true]{println!("1/{name}/{gate}: {}",p1(&c,gate));}}}