//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/content/identity.md
//! @prompt-hash e05e8c67
//! @layer L1
//! @updated 2026-10-07

use std::fmt;

use super::Content;

impl Content {
    /// Return the stable element name used by diagnostics and field errors.
    pub fn elem_name(&self) -> &'static str {
        if self.constructor_heading().is_some() {
            return "heading";
        }
        match self {
            Self::Source(e) => e.body.elem_name(),
            Self::ShowRecipe(e) => e.body.elem_name(),
            Self::Located(e) => e.body.elem_name(),
            Self::Realized(e) => e.visual.elem_name(),
            Self::Text(_) => "text",
            Self::Space | Self::SpaceFromNewline => "space",
            Self::Empty => "sequence",
            Self::Sequence(_) => "sequence",
            Self::HtmlElem(_) => "elem",
            Self::HtmlFrame(_) => "frame",
            Self::Par { .. } => "par",
            Self::Styled(..) => "styled",
            Self::PageRun(_) => "page.run",
            Self::Heading(_) => "heading",
            Self::Title(_) => "title",
            Self::Strong(_) => "strong",
            Self::Emph(_) => "emph",
            Self::Raw(_) => "raw",
            Self::ListItem(_) => "list.item",
            Self::EnumItem(_) => "enum.item",
            Self::Link(_) => "link",
            Self::Equation(_) => "equation",
            Self::MathSequence(_) => "math.sequence",
            Self::MathIdent(_) => "math.ident",
            Self::MathText(_) => "math.text",
            Self::MathFrac(_) => "math.frac",
            Self::MathAttach(_) => "math.attach",
            Self::MathPrimes(_) => "primes",
            Self::MathRoot(_) => "math.root",
            Self::MathStretch(_) => "stretch",
            Self::MathDelimited(_) => "math.delimited",
            Self::MathLr(_) => "lr",
            Self::MathMid(_) => "mid",
            Self::MathOverbrace(_) => "overbrace",
            Self::MathUnderbrace(_) => "underbrace",
            Self::MathUnderbracket(_) => "underbracket",
            Self::MathUnderparen(_) => "underparen",
            Self::MathUndershell(_) => "undershell",
            Self::MathOverbracket(_) => "overbracket",
            Self::MathOverparen(_) => "overparen",
            Self::MathOvershell(_) => "overshell",
            Self::MathAlignPoint(_) => "math.align.point",
            Self::Linebreak(_) => "linebreak",
            Self::MathMatrix(_) => "mat",
            Self::MathCases(_) => "math.cases",
            Self::MathAccent(_) => "math.accent",
            Self::MathCancel(_) => "math.cancel",
            Self::MathUnderline(_) => "math.underline",
            Self::MathOverline(_) => "math.overline",
            Self::MathVec(_) => "math.vec",
            Self::MathClassOverride(_) => "math.class",
            Self::MathLimitsOverride(_) => "math.limits",
            Self::MathUnderover(_) => "math.underover",
            Self::MathOp(_) => "math.op",
            Self::MathStyled(_) => "math.styled",
            Self::Label(_) => "label",
            Self::Ref(_) => "ref",
            Self::CounterDisplay(_) => "counter.display",
            Self::CounterUpdate(_) => "counter.update",
            Self::Outline(_) => "outline",
            Self::OutlineEntry(_) => "entry",
            Self::Figure(_) => "figure",
            Self::FigureCaption(_) => "figure.caption",
            Self::Image(_) => "image",
            Self::Shape(_) => "shape",
            Self::Curve(_) => "curve",
            Self::Transform(e) => match e.operation {
                crate::entities::elements::transform::TransformOperation::Matrix(_) => {
                    "transform"
                }
                crate::entities::elements::transform::TransformOperation::Move {
                    ..
                } => "move",
                crate::entities::elements::transform::TransformOperation::Rotate {
                    ..
                } => "rotate",
                crate::entities::elements::transform::TransformOperation::Scale {
                    ..
                } => "scale",
                crate::entities::elements::transform::TransformOperation::Skew {
                    ..
                } => "skew",
            },
            Self::Grid(_) => "grid",
            Self::GridHeader(_) => "grid.header",
            Self::GridFooter(_) => "grid.footer",
            Self::GridCell(_) => "grid.cell",
            Self::GridHLine(_) => "grid.hline",
            Self::GridVLine(_) => "grid.vline",
            Self::SetPage(_) => "set.page",
            Self::Align(_) => "align",
            Self::Place(_) => "place",
            Self::Flush(_) => "flush",
            Self::Divider(_) => "divider",
            Self::Terms(_) => "terms",
            Self::TermItem(_) => "term.item",
            Self::Quote(_) => "quote",
            Self::SmartQuote(_) => "smartquote",
            Self::PdfAttach(_) => "pdf.attach",
            Self::PdfArtifact(_) => "pdf.artifact",
            Self::Underline(_) => "underline",
            Self::Strike(_) => "strike",
            Self::Overline(_) => "overline",
            Self::SmallCaps { .. } => "small.caps",
            Self::Pad(_) => "pad",
            Self::Columns(_) => "columns",
            Self::Boxed(_) => "box",
            Self::Block(_) => "block",
            Self::Table(_) => "table",
            Self::TableCell(_) => "cell",
            Self::TableHeader(_) => "table.header",
            Self::TableFooter(_) => "table.footer",
            Self::TableHLine(_) => "table.hline",
            Self::TableVLine(_) => "table.vline",
            Self::Bibliography(_) => "bibliography",
            Self::Cite(_) => "cite",
            Self::Footnote(_) => "footnote",
            Self::FootnoteEntry(_) => "footnote.entry",
            Self::Metadata(_) => "metadata",
            Self::State(_) => "state",
            Self::StateUpdate(_) => "state.update",
            Self::StateDisplay(_) => "state.display",
            Self::CounterDisplayCallback(_) => "counter.display.callback",
            Self::Dynamic(_) => "dynamic",
            Self::Document { .. } => "document",
            Self::Asset { .. } => "asset",
            Self::ContextBlock(_) => "context",
            Self::Colbreak(_) => "colbreak",
            Self::HSpace(_) => "hspace",
            Self::Hide(_) => "hide",
            Self::Pagebreak(_) => "pagebreak",
            Self::Parbreak => "parbreak",
            Self::Repeat(_) => "repeat",
            Self::Stack(_) => "stack",
            Self::VSpace(_) => "vspace",
        }
    }
}

impl fmt::Debug for Content {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        thread_local! {
            static DEPTH: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
        }
        const MAX_DEPTH: usize = 8;

        let depth = DEPTH.get();
        if depth >= MAX_DEPTH {
            return write!(f, "...");
        }
        DEPTH.set(depth + 1);
        let result = fmt_content(self, f);
        DEPTH.set(depth);
        result
    }
}

fn fmt_content(c: &Content, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match c {
        Content::Source(e) => write!(f, "{:?}", e.body),
        Content::ShowRecipe(e) => write!(f, "{:?}", e.body),
        Content::Located(e) => write!(f, "{:?}", e.body),
        Content::Realized(e) => write!(f, "{:?}", e.visual),
        Content::Empty => write!(f, "empty"),
        Content::Text(t) => write!(f, "text({:?})", t),
        Content::Space | Content::SpaceFromNewline => write!(f, "space"),
        Content::Parbreak => write!(f, "parbreak"),
        Content::Par { body } => write!(f, "par({:?})", body),
        Content::Sequence(seq) => f.debug_tuple("sequence").field(&seq.as_ref()).finish(),
        Content::HtmlElem(e) => write!(f, "html.elem({:?})", e),
        Content::HtmlFrame(e) => write!(f, "html.frame({:?})", e),
        Content::Heading(h) => write!(f, "heading({:?})", h),
        Content::Title(t) => write!(f, "title({:?})", t),
        Content::Strong(s) => write!(f, "strong({:?})", s),
        Content::Emph(e) => write!(f, "emph({:?})", e),
        Content::Raw(r) => write!(f, "raw({:?})", r),
        Content::ListItem(li) => write!(f, "list.item({:?})", li),
        Content::EnumItem(ei) => write!(f, "enum.item({:?})", ei),
        Content::Link(l) => write!(f, "link({:?})", l),
        Content::Equation(e) => write!(f, "equation({:?})", e),
        Content::MathSequence(seq) => {
            f.debug_tuple("math.sequence").field(&seq.as_ref()).finish()
        }
        Content::MathIdent(s) => write!(f, "math.ident({:?})", s),
        Content::MathText(s) => write!(f, "math.text({:?})", s),
        Content::MathFrac(fr) => write!(f, "math.frac({:?})", fr),
        Content::MathAttach(a) => write!(f, "math.attach({:?})", a),
        Content::MathPrimes(p) => write!(f, "math.primes({:?})", p),
        Content::MathRoot(r) => write!(f, "math.root({:?})", r),
        Content::MathStretch(s) => write!(f, "math.stretch({:?})", s),
        Content::MathDelimited(d) => write!(f, "math.delimited({:?})", d),
        Content::MathLr(lr) => write!(f, "math.lr({:?})", lr),
        Content::MathMid(mid) => write!(f, "math.mid({:?})", mid),
        Content::MathOverbrace(overbrace) => write!(f, "math.overbrace({:?})", overbrace),
        Content::MathUnderbrace(underbrace) => {
            write!(f, "math.underbrace({:?})", underbrace)
        }
        Content::MathUnderbracket(underbracket) => {
            write!(f, "math.underbracket({:?})", underbracket)
        }
        Content::MathUnderparen(underparen) => {
            write!(f, "math.underparen({:?})", underparen)
        }
        Content::MathUndershell(undershell) => {
            write!(f, "math.undershell({:?})", undershell)
        }
        Content::MathOverbracket(overbracket) => {
            write!(f, "math.overbracket({:?})", overbracket)
        }
        Content::MathOverparen(overparen) => write!(f, "math.overparen({:?})", overparen),
        Content::MathOvershell(overshell) => write!(f, "math.overshell({:?})", overshell),
        Content::MathAlignPoint(_) => write!(f, "math.align.point"),
        Content::Linebreak(_) => write!(f, "linebreak"),
        Content::MathMatrix(_) => write!(f, "math.matrix"),
        Content::MathCases(_) => write!(f, "math.cases"),
        Content::MathAccent(a) => write!(f, "math.accent({:?})", a),
        Content::MathCancel(c) => write!(f, "math.cancel({:?})", c),
        Content::MathUnderline(u) => write!(f, "math.underline({:?})", u),
        Content::MathOverline(o) => write!(f, "math.overline({:?})", o),
        Content::MathVec(v) => write!(f, "math.vec({:?})", v),
        Content::MathClassOverride(c) => write!(f, "math.class({:?})", c),
        Content::MathLimitsOverride(o) => write!(f, "math.limits({:?})", o),
        Content::MathUnderover(u) => write!(f, "math.underover({:?})", u),
        Content::MathOp(o) => write!(f, "math.op({:?})", o),
        Content::MathStyled(s) => write!(f, "math.styled({:?})", s),
        Content::Label(l) => write!(f, "label({:?})", l),
        Content::Ref(r) => write!(f, "ref({:?})", r),
        Content::CounterDisplay(_) => write!(f, "counter.display"),
        Content::CounterUpdate(_) => write!(f, "counter.update"),
        Content::Outline(o) => write!(f, "outline({:?})", o),
        Content::OutlineEntry(e) => write!(f, "entry({:?})", e),
        Content::Figure(fig) => write!(f, "figure({:?})", fig),
        Content::FigureCaption(fc) => write!(f, "figure.caption({:?})", fc),
        Content::Image(i) => write!(f, "image({:?})", i),
        Content::Shape(_) => write!(f, "shape"),
        Content::Curve(_) => write!(f, "curve"),
        Content::Transform(t) => write!(f, "transform({:?})", t),
        Content::Grid(_) => write!(f, "grid"),
        Content::GridHeader(_) => write!(f, "grid.header"),
        Content::GridFooter(_) => write!(f, "grid.footer"),
        Content::GridCell(_) => write!(f, "grid.cell"),
        Content::GridHLine(_) => write!(f, "grid.hline"),
        Content::GridVLine(_) => write!(f, "grid.vline"),
        Content::SetPage(_) => write!(f, "set.page"),
        Content::Align(a) => write!(f, "align({:?})", a),
        Content::Place(p) => write!(f, "place({:?})", p),
        Content::Flush(_) => write!(f, "place.flush"),
        Content::Styled(styled) => {
            write!(f, "styled({:?}, {:?})", styled.body, styled.styles)
        }
        Content::Divider(_) => write!(f, "divider"),
        Content::Terms(_) => write!(f, "terms"),
        Content::TermItem(ti) => write!(f, "term.item({:?})", ti),
        Content::Quote(q) => write!(f, "quote({:?})", q),
        Content::SmartQuote(_) => write!(f, "smart.quote"),
        Content::PdfAttach(_) => write!(f, "pdf.attach"),
        Content::PdfArtifact(a) => write!(f, "pdf.artifact({:?})", a),
        Content::Underline(u) => write!(f, "underline({:?})", u),
        Content::Strike(s) => write!(f, "strike({:?})", s),
        Content::Overline(o) => write!(f, "overline({:?})", o),
        Content::SmallCaps { body } => write!(f, "small.caps({:?})", body),
        Content::Pad(p) => write!(f, "pad({:?})", p),
        Content::Hide(h) => write!(f, "hide({:?})", h),
        Content::HSpace(_) => write!(f, "hspace"),
        Content::VSpace(_) => write!(f, "vspace"),
        Content::Pagebreak(_) => write!(f, "pagebreak"),
        Content::Colbreak(_) => write!(f, "colbreak"),
        Content::Stack(_) => write!(f, "stack"),
        Content::Boxed(b) => write!(f, "box({:?})", b),
        Content::Block(b) => write!(f, "block({:?})", b),
        Content::TableCell(_) => write!(f, "table.cell"),
        Content::Bibliography(b) => write!(f, "bibliography({:?})", b),
        Content::Cite(c) => write!(f, "cite({:?})", c),
        Content::Footnote(foot) => write!(f, "footnote({:?})", foot),
        Content::FootnoteEntry(fe) => write!(f, "footnote.entry({:?})", fe),
        Content::TableHeader(_) => write!(f, "table.header"),
        Content::TableFooter(_) => write!(f, "table.footer"),
        Content::TableHLine(_) => write!(f, "table.hline"),
        Content::TableVLine(_) => write!(f, "table.vline"),
        Content::Table(_) => write!(f, "table"),
        Content::Repeat(_) => write!(f, "repeat"),
        Content::Columns(_) => write!(f, "columns"),
        Content::Metadata(_) => write!(f, "metadata"),
        Content::State(_) => write!(f, "state"),
        Content::StateUpdate(_) => write!(f, "state.update"),
        Content::StateDisplay(_) => write!(f, "state.display"),
        Content::CounterDisplayCallback(_) => write!(f, "counter.display.callback"),
        Content::Dynamic(_) => write!(f, "dynamic"),
        Content::Document { path, body, .. } => {
            write!(f, "document(path: {:?}, body: {:?})", path, body)
        }
        Content::Asset { path, data } => {
            write!(f, "asset(path: {:?}, data: bytes({}))", path, data.len())
        }
        Content::ContextBlock(e) => write!(f, "context({:?})", e),
        Content::PageRun(e) => write!(f, "page.run({:?})", e),
    }
}

#[cfg(test)]
mod tests {
    use super::Content;

    #[test]
    fn debug_and_name_preserve_public_identity() {
        assert_eq!(Content::Space.elem_name(), "space");
        assert_eq!(Content::SpaceFromNewline.elem_name(), "space");
        assert_eq!(format!("{:?}", Content::SpaceFromNewline), "space");
    }
}
