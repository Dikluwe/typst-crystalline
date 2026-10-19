//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/paragraph_indent.md
//! @prompt-hash 62536dad
//! @layer L1
//! @updated 2026-10-18

use super::{FontMetrics, ImageSizer, Layouter};
use crate::entities::{
    content::Content,
    dir::Dir,
    layout_types::{FrameItem, Pt, TextStyle},
    style_chain::StyleChain,
    value::Value,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct Spec {
    first: f64,
    continuation: f64,
    dir: Dir,
}
impl Spec {
    pub(super) fn escape(amount: f64, dir: Dir) -> Self {
        Self { first: -amount, continuation: 0.0, dir }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Context {
    Paragraph,
    Inline,
}
#[derive(Clone, Debug)]
pub(super) struct State {
    context: Context,
    spec: Option<Spec>,
    continued: bool,
    applied: bool,
    started: bool,
}
impl Default for State {
    fn default() -> Self {
        Self {
            context: Context::Paragraph,
            spec: None,
            continued: false,
            applied: false,
            started: false,
        }
    }
}
impl State {
    pub(super) fn inline() -> Self {
        Self { context: Context::Inline, ..Self::default() }
    }
    pub(super) fn fragment(body: &Content) -> Self {
        if requires_paragraph(body) {
            Self::default()
        } else {
            Self::inline()
        }
    }
    fn offset(&self) -> f64 {
        self.spec
            .map_or(0.0, |s| if self.continued { s.continuation } else { s.first })
    }
    fn end(&mut self) {
        self.spec = None;
        self.continued = false;
        self.applied = false;
        self.started = false;
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) struct Shape {
    first: f64,
    continuation: f64,
}
impl Shape {
    pub(super) fn uniform(width: f64) -> Self {
        Self { first: width, continuation: width }
    }
    pub(super) fn width(self, first: bool) -> f64 {
        if first {
            self.first
        } else {
            self.continuation
        }
    }
    pub(super) fn is_uniform(self) -> bool {
        self.first == self.continuation
    }
}

/// Classify existing visual structure without realizing effects or custom bodies.
fn requires_paragraph(c: &Content) -> bool {
    match c {
        Content::Source(e) => requires_paragraph(&e.body),
        Content::Located(e) => requires_paragraph(&e.body),
        Content::Realized(e) => requires_paragraph(&e.visual),
        Content::ShowRecipe(_) => false,
        Content::Sequence(parts) => parts.iter().any(requires_paragraph),
        Content::Styled(e) => requires_paragraph(&e.body),
        Content::Strong(e) => requires_paragraph(&e.body),
        Content::Emph(e) => requires_paragraph(&e.body),
        Content::Link(e) => requires_paragraph(&e.body),
        Content::Underline(e) => requires_paragraph(&e.body),
        Content::Strike(e) => requires_paragraph(&e.body),
        Content::Overline(e) => requires_paragraph(&e.body),
        Content::SmallCaps { body, .. } => requires_paragraph(body),
        Content::Hide(e) => requires_paragraph(&e.body),
        Content::Raw(e) => e.block,
        Content::Equation(e) => e.block,
        Content::Quote(e) => e.block || requires_paragraph(&e.body),
        Content::Par { .. }
        | Content::Parbreak
        | Content::Heading(_)
        | Content::Title(_)
        | Content::ListItem(_)
        | Content::EnumItem(_)
        | Content::Outline(_)
        | Content::OutlineEntry(_)
        | Content::Figure(_)
        | Content::FigureCaption(_)
        | Content::Grid(_)
        | Content::GridHeader(_)
        | Content::GridFooter(_)
        | Content::GridCell(_)
        | Content::SetPage(_)
        | Content::PageRun(_)
        | Content::Align(_)
        | Content::Place(_)
        | Content::Flush(_)
        | Content::Divider(_)
        | Content::Terms(_)
        | Content::TermItem(_)
        | Content::Document { .. }
        | Content::Pad(_)
        | Content::VSpace(_)
        | Content::Pagebreak(_)
        | Content::Colbreak(_)
        | Content::Stack(_)
        | Content::Block(_)
        | Content::Bibliography(_)
        | Content::FootnoteEntry(_)
        | Content::TableCell(_)
        | Content::TableHeader(_)
        | Content::TableFooter(_)
        | Content::GridHLine(_)
        | Content::GridVLine(_)
        | Content::TableHLine(_)
        | Content::TableVLine(_)
        | Content::Table(_)
        | Content::Columns(_) => true,
        Content::Empty
        | Content::Text(_)
        | Content::Space
        | Content::SpaceFromNewline
        | Content::HtmlElem(_)
        | Content::HtmlFrame(_)
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
        | Content::Image(_)
        | Content::Shape(_)
        | Content::Curve(_)
        | Content::Transform(_)
        | Content::Asset { .. }
        | Content::SmartQuote(_)
        | Content::PdfAttach(_)
        | Content::PdfArtifact(_)
        | Content::HSpace(_)
        | Content::Boxed(_)
        | Content::Cite(_)
        | Content::Footnote(_)
        | Content::Repeat(_)
        | Content::Metadata(_)
        | Content::State(_)
        | Content::StateUpdate(_)
        | Content::StateDisplay(_)
        | Content::CounterDisplayCallback(_)
        | Content::ContextBlock(_)
        | Content::Dynamic(_) => false,
    }
}

fn shared_scope(parts: &[Content], chain: &StyleChain) -> StyleChain {
    let mut material =
        parts.iter().map(super::paragraph::prepared_text_body).filter(|c| {
            !matches!(c, Content::Empty | Content::Space | Content::SpaceFromNewline)
        });
    if let Some(first) = material.next() {
        if material.next().is_none() {
            if let Content::Styled(s) = first {
                return shared_scope(
                    std::slice::from_ref(&s.body),
                    &chain.push_styles(&s.styles),
                );
            }
            if let Content::Sequence(parts) = first {
                return shared_scope(parts, chain);
            }
        }
    }
    chain.clone()
}
fn start_chain<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    chain: &StyleChain,
) {
    if l.paragraph_indent.spec.is_some() {
        return;
    }
    let spec = l.pending_bib_escape.unwrap_or_else(|| {
        let style = TextStyle::from(chain);
        let amount = if l.paragraph_indent.context == Context::Paragraph {
            match chain.custom("par.hanging-indent") {
                Some(Value::Length(v)) => v.resolve_pt(style.size.0),
                _ => 0.0,
            }
        } else {
            0.0
        };
        Spec {
            first: 0.0,
            continuation: amount,
            dir: match chain.custom("text.dir") {
                Some(Value::Dir(dir)) => *dir,
                _ => style.dir.unwrap_or(Dir::LTR),
            },
        }
    });
    l.paragraph_indent.spec = Some(spec);
}
pub(super) fn start<M: FontMetrics, S: ImageSizer>(l: &mut Layouter<M, S>) {
    let chain = l.chain.clone();
    start_chain(l, &chain);
}
pub(super) fn start_scope<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    parts: &[Content],
) {
    if l.paragraph_indent.spec.is_none() {
        let chain = shared_scope(parts, &l.chain);
        start_chain(l, &chain);
    }
}
pub(super) fn end<M: FontMetrics, S: ImageSizer>(l: &mut Layouter<M, S>) {
    l.paragraph_indent.end();
}
pub(super) fn enter_fragment<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    body: &Content,
) -> State {
    end(l);
    std::mem::replace(&mut l.paragraph_indent, State::fragment(body))
}
pub(super) fn bounds<M: FontMetrics, S: ImageSizer>(l: &Layouter<M, S>) -> (f64, f64) {
    let left = l.regions.current.line_start_x.0;
    let right = l.regions.current.width - l.page_config.margin.right;
    let Some(spec) = l.paragraph_indent.spec else { return (left, right) };
    let offset = l.paragraph_indent.offset();
    if spec.dir == Dir::RTL {
        (left, right - offset)
    } else {
        (left + offset, right)
    }
}
/// Whether material admission applied the origin of this line.
pub(super) fn origin_applied<M: FontMetrics, S: ImageSizer>(l: &Layouter<M, S>) -> bool {
    l.paragraph_indent.applied
}
/// Read-only origin preview; does not admit material or consume an escape.
pub(super) fn cursor_start<M: FontMetrics, S: ImageSizer>(l: &Layouter<M, S>) -> Pt {
    let state = &l.paragraph_indent;
    let dx = state.spec.or(l.pending_bib_escape).map_or(0.0, |spec| {
        if state.applied || spec.dir == Dir::RTL {
            0.0
        } else if state.continued {
            spec.continuation
        } else {
            spec.first
        }
    });
    l.regions.current.cursor_x + Pt(dx)
}
pub(super) fn shape<M: FontMetrics, S: ImageSizer>(
    l: &Layouter<M, S>,
    width: f64,
) -> Shape {
    match l.paragraph_indent.spec {
        Some(spec) => Shape {
            first: width - l.paragraph_indent.offset(),
            continuation: width - spec.continuation,
        },
        None => Shape::uniform(width),
    }
}
pub(super) fn current_width<M: FontMetrics, S: ImageSizer>(
    l: &Layouter<M, S>,
    fallback: f64,
) -> f64 {
    l.paragraph_indent.spec.map_or(fallback, |spec| {
        if spec.first == 0.0 && spec.continuation == 0.0 {
            fallback
        } else {
            let (left, right) = bounds(l);
            right - left
        }
    })
}
/// Offsets are not glyphs or spacers and never demand vertical progress.
pub(super) fn admit<M: FontMetrics, S: ImageSizer>(l: &mut Layouter<M, S>) {
    let Some(spec) = l.paragraph_indent.spec else { return };
    if l.paragraph_indent.applied {
        return;
    }
    let dx = if spec.dir == Dir::RTL { 0.0 } else { l.paragraph_indent.offset() };
    l.regions.current.cursor_x += Pt(dx);
    if dx != 0.0 {
        for item in &mut l.regions.current.current_line {
            if let FrameItem::TextSeparator { pos, .. } = item {
                pos.x += Pt(dx);
            }
        }
        for x in &mut l.justify_opportunities {
            *x += dx;
        }
    }
    l.paragraph_indent.applied = true;
    l.paragraph_indent.started = true;
    if l.pending_bib_escape == Some(spec) {
        l.pending_bib_escape = None;
    }
}
pub(super) fn commit<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    material: bool,
) {
    if material && l.paragraph_indent.started {
        l.paragraph_indent.continued = true;
        l.paragraph_indent.applied = false;
    }
}
pub(super) fn reopen<M: FontMetrics, S: ImageSizer>(l: &mut Layouter<M, S>) {
    l.paragraph_indent.applied = false;
    if l.paragraph_indent.started {
        admit(l);
    }
}
pub(super) fn before_content<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    c: &Content,
) {
    let inline = matches!(
        c,
        Content::Boxed(_)
            | Content::Image(_)
            | Content::Shape(_)
            | Content::Curve(_)
            | Content::HSpace(_)
            | Content::Cite(_)
            | Content::Ref(_)
            | Content::Footnote(_)
            | Content::CounterDisplay(_)
            | Content::StateDisplay(_)
            | Content::CounterDisplayCallback(_)
            | Content::SmartQuote(_)
            | Content::Repeat(_)
    ) || matches!(c,Content::Raw(e) if !e.block)
        || matches!(c,Content::Equation(e) if !e.block);
    if inline {
        admit(l);
    }
}
pub(super) fn after_content<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    c: &Content,
) {
    if matches!(c, Content::Pagebreak(_) | Content::Colbreak(_))
        || (l.block_chain_active && l.paragraph_indent.started)
    {
        end(l);
    }
}

#[cfg(test)]
mod tests {
    use super::super::{layout, FixedMetrics, Layouter};
    use crate::entities::{
        content::Content,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{
            FrameItem, Length, PageMarginSpec, PageMargins, PagedDocument, Pt, TextStyle,
        },
        style::{Style, Styles},
        value::Value,
    };
    use comemo::Track;
    use std::sync::Arc;

    const WORDS: &str = "AAAA BBBB CCCC DDDD EEEE FFFF GGGG HHHH";
    #[derive(Debug)]
    struct Word {
        page: usize,
        text: String,
        x: f64,
        y: f64,
    }
    fn words(doc: &PagedDocument) -> Vec<Word> {
        doc.pages
            .iter()
            .enumerate()
            .flat_map(|(page, frame)| {
                crate::compiler::layout::tests::frame_items_recursive(&frame.items)
                    .into_iter()
                    .filter_map(move |item| match item {
                        FrameItem::Text { text, pos, .. } if !text.is_empty() => {
                            Some(Word {
                                page,
                                text: text.to_string(),
                                x: pos.x.0,
                                y: pos.y.0,
                            })
                        }
                        _ => None,
                    })
            })
            .collect()
    }
    fn line_starts(rows: &[Word]) -> Vec<&Word> {
        rows.iter()
            .filter(|w| w.text != "NEXT")
            .fold(Vec::new(), |mut lines, w| {
                if lines.last().is_none_or(|prior: &&Word| {
                    prior.page != w.page || (prior.y - w.y).abs() > 1e-6
                }) {
                    lines.push(w);
                }
                lines
            })
    }
    fn assert_origins(rows: &[Word], indent: f64) {
        let lines = line_starts(rows);
        assert!(lines.len() > 1, "{rows:?}");
        assert!(lines[0].x.abs() < 1e-6, "{rows:?}");
        for line in &lines[1..] {
            assert!((line.x - indent).abs() < 1e-6, "expected={indent}: {rows:?}");
        }
    }
    fn configured(intr: &dyn Introspector) -> Layouter<'_, FixedMetrics, NullImageSizer> {
        let mut l = Layouter::new(FixedMetrics, NullImageSizer, 10.0, intr.track());
        l.chain = l
            .chain
            .push_styles(&Styles::from_iter([Style::Size(Length::pt(10.0))]));
        l.style = TextStyle::from(&l.chain);
        l.page_config.width = 100.0;
        l.page_config.height = f64::INFINITY;
        l.page_config.margin = PageMargins::uniform(0.0);
        l.page_config.margin_spec = PageMarginSpec::uniform(0.0);
        l.regions.current.width = 100.0;
        l.regions.current.height = f64::INFINITY;
        l.regions.current.cursor_x = Pt(0.0);
        l.regions.current.line_start_x = Pt(0.0);
        l.regions.current.cursor_y = Pt(0.0);
        l
    }
    fn document(body: &str) -> PagedDocument {
        use crate::{
            compiler::eval::eval_for_test,
            contracts::world::World,
            entities::{
                file_id::FileId,
                font_book::FontBook,
                source::Source,
                world_types::{Bytes, Datetime, FileError, FileResult, Font, Library},
            },
        };
        struct MockWorld {
            library: Library,
            book: FontBook,
            source: Source,
        }
        impl World for MockWorld {
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
        let text=format!("#set page(width:100pt,height:auto,margin:0pt)\n#set text(size:10pt,top-edge:10pt,bottom-edge:0pt,hyphenate:false)\n{body}");
        let world = MockWorld {
            library: Library::new(),
            book: FontBook::new(),
            source: Source::new(
                FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()),
                text,
            ),
        };
        let module = eval_for_test(&world, &world.source).unwrap();
        layout(module.content().unwrap())
    }

    #[test]
    fn hanging_indent_set_uses_common_shape_for_both_strategies_and_signs() {
        for strategy in ["simple", "optimized"] {
            for (value, expected) in
                [("0pt", 0.0), ("20pt", 20.0), ("-20pt", -20.0), ("2pt+1em", 12.0)]
            {
                let doc=document(&format!("#set par(hanging-indent:{value},linebreaks:\"{strategy}\")\n{WORDS}\n\nNEXT"));
                let rows = words(&doc);
                assert_origins(&rows, expected);
                assert_eq!(rows.iter().filter(|w| w.text != "NEXT").count(), 8);
                assert_eq!(rows.iter().find(|w| w.text == "NEXT").unwrap().x, 0.0);
            }
        }
    }
    #[test]
    fn hanging_indent_optimized_keeps_its_paragraph_choice() {
        let body = "AAAAA BBBBB CCCCC DDDDD EEEEE FFFFF GG HH";
        let layout = |strategy| {
            words(&document(&format!(
            "#set par(hanging-indent:10pt,justify:true,linebreaks:\"{strategy}\")\n{body}"
        )))
        };
        let simple = layout("simple");
        let optimized = layout("optimized");
        let first_line = |rows: &[Word]| {
            rows.iter().take_while(|w| (w.y - rows[0].y).abs() < 1e-6).count()
        };
        assert_eq!(first_line(&simple), 2, "{simple:?}");
        assert_eq!(first_line(&optimized), 3, "{optimized:?}");
        assert_eq!(
            simple.iter().map(|w| &w.text).collect::<Vec<_>>(),
            optimized.iter().map(|w| &w.text).collect::<Vec<_>>()
        );
    }
    #[test]
    fn hanging_indent_decoration_origins_follow_real_continuations() {
        for kind in ["underline", "strike", "overline"] {
            for amount in [0.0, 20.0, -10.0] {
                for forced in [false, true] {
                    let prefix = if forced { "PREFIX\\\n" } else { "" };
                    let doc = document(&format!(
                        "#set par(hanging-indent:{amount}pt)\n{prefix}#{kind}[{WORDS}]"
                    ));
                    let lines: Vec<_> = doc
                        .pages
                        .iter()
                        .flat_map(|p| {
                            super::super::tests::frame_items_recursive(&p.items)
                        })
                        .filter_map(|i| match i {
                            FrameItem::Line { start, .. } => Some(start.x.0),
                            _ => None,
                        })
                        .collect();
                    assert!(lines.len() > 1, "{kind}: {lines:?}");
                    for (index, x) in lines.iter().enumerate() {
                        assert_eq!(
                            *x,
                            if forced || index > 0 { amount } else { 0.0 },
                            "{kind}, h={amount}, forced={forced}, lines={lines:?}"
                        );
                    }
                }
            }
        }
    }
    #[test]
    fn hanging_indent_decorations_inside_explicit_box_paragraph_keep_local_origin() {
        let doc = document(&format!("#set par(hanging-indent:20pt)\n#box(width:100pt)[#par()[#overline[{WORDS}]]]"));
        let lines: Vec<_> = doc
            .pages
            .iter()
            .flat_map(|p| super::super::tests::frame_items_recursive(&p.items))
            .filter_map(|i| match i {
                FrameItem::Line { start, .. } => Some(start.x.0),
                _ => None,
            })
            .collect();
        assert!(lines.len() > 1, "{lines:?}");
        for x in &lines[1..] {
            assert_eq!(*x, 20.0, "{lines:?}");
        }
    }
    #[test]
    fn hanging_indent_empty_decoration_does_not_admit_preview() {
        for kind in ["underline", "strike", "overline"] {
            for body in ["", "#metadata(0)", "#par()[]"] {
                let doc = document(&format!(
                    "#set par(hanging-indent:-10pt)\nPREFIX\\\n#{kind}[{body}]"
                ));
                let lines: Vec<_> = doc
                    .pages
                    .iter()
                    .flat_map(|p| super::super::tests::frame_items_recursive(&p.items))
                    .filter(|i| matches!(i, FrameItem::Line { .. }))
                    .collect();
                assert!(lines.is_empty(), "{kind}, body={body}: {lines:?}");
            }
        }
    }
    #[test]
    fn hanging_indent_constructor_transports_length_and_restores_outer_scope() {
        for strategy in ["simple", "optimized"] {
            for (value, expected) in [("20pt", 20.0), ("-20pt", -20.0), ("2pt+1em", 12.0)]
            {
                let doc=document(&format!("#par(hanging-indent:{value},linebreaks:\"{strategy}\")[{WORDS}]\nNEXT"));
                let rows = words(&doc);
                assert_origins(&rows, expected);
                assert_eq!(rows.iter().find(|w| w.text == "NEXT").unwrap().x, 0.0);
            }
        }
    }
    #[test]
    fn hanging_indent_repeated_and_empty_declarations_keep_boundaries() {
        for body in [
            "#set par(hanging-indent:0pt)\nFIRST\n#set par(hanging-indent:0pt)\nLAST",
            "FIRST #[#set par(hanging-indent:0pt)] LAST",
            "#set par(hanging-indent:20pt)\nFIRST #[#set par(hanging-indent:20pt)\nMIDDLE] LAST",
        ] {
            let rows=words(&document(body));
            let first=rows.iter().find(|w| w.text=="FIRST").unwrap();
            let last=rows.iter().find(|w| w.text=="LAST").unwrap();
            assert!(last.y>first.y,"{body}: {rows:?}");
            assert_eq!(last.x,0.0);
            if let Some(middle)=rows.iter().find(|w| w.text=="MIDDLE") {
                assert!(middle.y>first.y && middle.y<last.y,"{rows:?}");
            }
        }
    }
    #[test]
    fn hanging_indent_em_uses_shared_scope_before_partial_font_run() {
        let rows=words(&document("#set par(hanging-indent:1em)\n#text(size:20pt)[AAAA] BBBB CCCC DDDD EEEE FFFF GGGG HHHH"));
        assert_origins(&rows, 10.0);
    }
    #[test]
    fn hanging_indent_inline_fragments_ignore_generic_length() {
        for body in [
            format!("#block[{WORDS}]"),
            format!("#box(width:100pt)[{WORDS}]"),
            format!("= {WORDS}"),
        ] {
            let rows =
                words(&document(&format!("#set par(hanging-indent:20pt)\n{body}")));
            assert_origins(&rows, 0.0);
        }
    }
    #[test]
    fn hanging_indent_explicit_paragraph_in_container_is_active() {
        for body in [
            format!("#block[#par()[{WORDS}]]"),
            format!("#box(width:100pt)[#par()[{WORDS}]]"),
        ] {
            let rows =
                words(&document(&format!("#set par(hanging-indent:20pt)\n{body}")));
            assert_origins(&rows, 20.0);
        }
    }
    #[test]
    fn hanging_indent_multiline_block_restarts_for_each_paragraph() {
        let rows = words(&document(&format!(
            "#set par(hanging-indent:20pt)\n#block[{WORDS}\n\n{WORDS}]"
        )));
        let mut first = true;
        for row in line_starts(&rows) {
            if row.text == "AAAA" {
                first = true;
            }
            assert_eq!(row.x, if first { 0.0 } else { 20.0 }, "{rows:?}");
            first = false;
        }
        assert_eq!(rows.iter().filter(|w| w.text == "AAAA").count(), 2);
    }
    #[test]
    fn hanging_indent_rtl_continuation_moves_the_initial_edge() {
        for strategy in ["simple", "optimized"] {
            let doc=document(&format!("#set text(dir:rtl)\n#set par(hanging-indent:20pt,linebreaks:\"{strategy}\")\n{WORDS}"));
            let rows = words(&doc);
            let starts = line_starts(&rows);
            for (index, start) in starts.iter().enumerate() {
                let right = rows
                    .iter()
                    .filter(|w| w.page == start.page && (w.y - start.y).abs() < 1e-6)
                    .map(|w| w.x + 24.0)
                    .fold(f64::NEG_INFINITY, f64::max);
                assert!(
                    (right - if index == 0 { 100.0 } else { 80.0 }).abs() < 1e-6,
                    "{strategy}: {rows:?}"
                );
            }
        }
    }
    #[test]
    fn hanging_indent_page_continuation_keeps_the_same_paragraph() {
        let doc = document(&format!(
            "#set page(height:60pt)\n#set par(hanging-indent:20pt)\n{}",
            [WORDS; 3].join(" ")
        ));
        assert!(doc.pages.len() > 1);
        let rows = words(&doc);
        assert_origins(&rows, 20.0);
        assert_eq!(rows.len(), 24);
    }
    #[test]
    fn hanging_indent_checkpoint_restores_actual_continuation() {
        let intr = TagIntrospector::empty();
        let mut l = configured(&intr);
        l.chain = l
            .chain
            .push_custom("par.hanging-indent", Value::Length(Length::pt(20.0)));
        l.layout_content(&Content::text("AAAA BBBB CCCC DDDD"));
        l.flush_line();
        let checkpoint = l.capture_flow_suffix_checkpoint();
        l.layout_content(&Content::text("EEEE FFFF"));
        l.restore_flow_suffix_checkpoint(checkpoint);
        l.layout_content(&Content::text("GGGG HHHH"));
        let rows = words(&l.finish());
        assert_eq!(rows.iter().find(|w| w.text == "DDDD").unwrap().x, 20.0, "{rows:?}");
        assert!(rows.iter().all(|w| w.text != "EEEE" && w.text != "FFFF"));
        assert_eq!(
            rows.iter().map(|w| w.text.as_str()).collect::<Vec<_>>(),
            ["AAAA", "BBBB", "CCCC", "DDDD", "GGGG", "HHHH"]
        );
        assert_eq!(rows.iter().find(|w| w.text == "GGGG").unwrap().x, 20.0);
    }
    #[test]
    fn hanging_indent_subframe_does_not_steal_outer_continuation() {
        let rows=words(&document("#set par(hanging-indent:20pt)\nAAAA BBBB CCCC\\\n#box(width:30pt)[X] DDDD EEEE FFFF GGGG HHHH"));
        let x = rows.iter().find(|w| w.text == "X").unwrap();
        assert_eq!(x.x, 20.0, "{rows:?}");
        for row in line_starts(&rows).into_iter().skip(1) {
            assert_eq!(row.x, 20.0, "{rows:?}");
        }
    }
    #[test]
    fn hanging_indent_extreme_values_keep_all_logical_words_and_progress() {
        for value in ["200pt", "-200pt"] {
            let rows =
                words(&document(&format!("#set par(hanging-indent:{value})\n{WORDS}")));
            assert_eq!(
                rows.iter().map(|w| w.text.as_str()).collect::<Vec<_>>(),
                WORDS.split_whitespace().collect::<Vec<_>>()
            );
            assert!(rows.iter().all(|w| w.x.is_finite() && w.y.is_finite()));
        }
    }
    fn bibliography_rows(
        plan: super::super::bib_csl::plan::BibliographyPlan,
        override_value: Option<f64>,
    ) -> Vec<Word> {
        use crate::entities::{
            bib_entry::BibEntry, elements::bibliography::BibliographyElem,
        };
        let intr = TagIntrospector::empty();
        let mut l = configured(&intr);
        l.chain = l
            .chain
            .push_custom("par.hanging-indent", Value::Length(Length::pt(30.0)));
        if let Some(value) = override_value {
            l.chain = l.chain.push_custom(
                "bibliography.hanging-override",
                Value::Length(Length::pt(value)),
            );
        }
        l.bib_layout = Some(Arc::new(plan));
        let b = Content::Bibliography(Arc::new(BibliographyElem {
            entries: vec![BibEntry::new("a", "Author", WORDS, 2020)],
            path: None,
            title: None,
            style: Some("mla".into()),
            locale: None,
        }));
        l.layout_content(&b);
        words(&l.finish())
    }
    #[test]
    fn hanging_indent_bibliography_default_and_explicit_override_are_causal() {
        use super::super::bib_csl::plan::{BibliographyPlan, Entry};
        for (value, expected) in
            [(None, 15.0), (Some(0.0), 0.0), (Some(30.0), 30.0), (Some(-5.0), -5.0)]
        {
            let rows = bibliography_rows(
                BibliographyPlan::Hanging(vec![Entry {
                    prefix: None,
                    body: Content::text(WORDS),
                }]),
                value,
            );
            assert_origins(&rows, expected);
        }
    }
    #[test]
    fn hanging_indent_plain_and_columns_keep_their_own_geometry() {
        use super::super::bib_csl::plan::{BibliographyPlan, Entry};
        let rows = bibliography_rows(
            BibliographyPlan::Plain(vec![Entry {
                prefix: None,
                body: Content::text(WORDS),
            }]),
            Some(30.0),
        );
        assert_origins(&rows, 0.0);
        let rows = bibliography_rows(
            BibliographyPlan::Columns(vec![Entry {
                prefix: Some(Content::text("[1]")),
                body: Content::text(WORDS),
            }]),
            Some(30.0),
        );
        for row in rows.iter().filter(|w| w.text != "[1]") {
            assert!(row.x >= 24.5 - 1e-6, "{rows:?}");
        }
    }
}
