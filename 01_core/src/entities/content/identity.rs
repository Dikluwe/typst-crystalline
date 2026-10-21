//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/content/identity.md
//! @prompt-hash 4e1b9ce7
//! @layer L1
//! @updated 2026-10-07

use std::fmt;

use super::Content;
use crate::entities::value::Value;

fn is_equation_constructor_content(content: &Content) -> bool {
    match content {
        Content::Equation(_) => true,
        Content::Source(e) => is_equation_constructor_content(&e.body),
        Content::ShowRecipe(e) => is_equation_constructor_content(&e.body),
        Content::Located(e) => is_equation_constructor_content(&e.body),
        Content::Realized(e) => is_equation_constructor_content(&e.visual),
        Content::Styled(styled) => {
            let marker = styled
                .styles
                .delta()
                .custom
                .iter()
                .rev()
                .find(|(key, _)| key.as_str() == "equation.constructor");
            matches!(marker, Some((_, Value::Bool(true))))
                && is_equation_constructor_content(&styled.body)
        }
        Content::Empty
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
        | Content::Dynamic(..) => false,
    }
}

impl Content {
    /// Return the stable element name used by diagnostics and field errors.
    pub fn elem_name(&self) -> &'static str {
        if self.constructor_heading().is_some() {
            return "heading";
        }
        if is_equation_constructor_content(self) {
            return "equation";
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

        let mut content = self;
        loop {
            match content {
                Content::Source(e) => content = &e.body,
                Content::ShowRecipe(e) => content = &e.body,
                Content::Located(e) => content = &e.body,
                Content::Realized(e) => content = &e.visual,
                Content::Empty
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
                | Content::Dynamic(..) => break,
            }
        }

        let depth = DEPTH.get();
        if depth >= MAX_DEPTH {
            return write!(f, "...");
        }
        DEPTH.set(depth + 1);
        let result = fmt_content(content, f);
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

#[cfg(test)]
mod v16_e_equation_identity_tests {
    use super::Content;
    use crate::entities::{
        content::{RealizedContent, SourceContent},
        span::Span,
        style::Styles,
        value::Value,
    };
    use std::sync::Arc;

    fn marked(body: Content) -> Content {
        Content::styled(
            body,
            Styles::new().push_custom("equation.constructor", Value::Bool(true)),
        )
    }

    #[test]
    fn v16_e_equation_marked_identity_follows_causal_visual_and_nested_transport() {
        let equation = || Content::equation(Content::text("x"), false);
        for body in [
            equation(),
            Content::Source(Arc::new(SourceContent {
                body: equation(),
                span: Span::detached(),
            })),
            Content::Realized(Arc::new(RealizedContent {
                original: Content::text("not-equation"),
                visual: equation(),
            })),
            marked(equation()),
        ] {
            assert_eq!(marked(body).elem_name(), "equation");
        }
    }

    #[test]
    fn v16_e_equation_identity_rejects_unmarked_and_invalid_transports() {
        let equation = || Content::equation(Content::text("x"), false);
        for content in [
            Content::styled(
                equation(),
                Styles::new().push_custom("equation.numbering", Value::None),
            ),
            Content::styled(marked(equation()), Styles::new()),
            Content::styled(
                equation(),
                Styles::new().push_custom("equation.constructor", Value::Bool(false)),
            ),
            Content::styled(
                equation(),
                Styles::new()
                    .push_custom("equation.constructor", Value::Str("true".into())),
            ),
            marked(Content::text("not-equation")),
            marked(Content::Realized(Arc::new(RealizedContent {
                original: equation(),
                visual: Content::text("visual"),
            }))),
        ] {
            assert_eq!(content.elem_name(), "styled");
        }
    }
}

#[cfg(test)]
mod batch_g_carrier_depth_tests {
    use super::Content;
    use crate::entities::content::{
        LocatedContent, RealizedContent, ShowRecipeContent, SourceContent,
    };
    use crate::entities::elements::context_block::ContextBlockElem;
    use crate::entities::file_id::FileId;
    use crate::entities::func::Func;
    use crate::entities::html::{HtmlAttrs, HtmlBody, HtmlElem, HtmlFrameElem};
    use crate::entities::locator::Locator;
    use crate::entities::show::{NodeKind, Selector, ShowRule, Transformation};
    use crate::entities::span::Span;
    use crate::entities::style_chain::StyleChain;
    use std::hash::{DefaultHasher, Hash, Hasher};
    use std::num::NonZeroU16;
    use std::sync::Arc;

    fn strong(mut body: Content, depth: usize) -> Content {
        for _ in 0..depth {
            body = Content::strong(body);
        }
        body
    }
    fn source(body: Content) -> Content {
        Content::Source(Arc::new(SourceContent {
            body,
            span: Span::from_range(FileId::from_raw(NonZeroU16::new(1).unwrap()), 2..3),
        }))
    }
    fn context() -> Content {
        Content::ContextBlock(Arc::new(ContextBlockElem {
            id: 91,
            closure: Func::native("must-not-execute", |_, _, _, _| {
                panic!("Debug executed Context")
            }),
        }))
    }
    fn carrier(body: Content, kind: usize, locator: &mut Locator) -> Content {
        match kind {
            0 => source(body),
            1 => Content::Located(Arc::new(LocatedContent {
                body,
                location: locator.next(),
            })),
            2 => Content::ShowRecipe(Arc::new(ShowRecipeContent {
                body,
                textual: true,
                context_id_floor: 73,
                guards: vec![31],
                styles: StyleChain::default_chain(),
                rules: Arc::from([ShowRule {
                    id: 31,
                    selector: Selector::NodeKind(NodeKind::Heading),
                    transform: Transformation::Func(Func::native(
                        "never-run-recipe",
                        |_, _, _, _| panic!("Debug executed recipe"),
                    )),
                }]),
                file: FileId::from_raw(NonZeroU16::new(1).unwrap()),
            })),
            3 => Content::Realized(Arc::new(RealizedContent {
                original: Content::sequence(vec![
                    Content::text("ORIGINAL_MUST_NOT_APPEAR"),
                    context(),
                ]),
                visual: body,
            })),
            _ => panic!("unknown test carrier"),
        }
    }
    fn family(name: usize, body: Content) -> Content {
        // All leaves remain at the visible-depth boundary, never beyond it.
        match name {
            0 => strong(body, 7),
            1 => Content::Sequence(vec![Content::text("prefix"), strong(body, 6)].into()),
            2 => Content::title(strong(body, 6)),
            3 => Content::HtmlFrame(Arc::new(HtmlFrameElem { body: strong(body, 6) })),
            4 => Content::HtmlElem(Arc::new(HtmlElem::new(
                "div".into(),
                None,
                HtmlBody::Content(Box::new(strong(body, 6))),
            ))),
            5 => strong(body, 7), // Same-id Context mechanics, not its language equality.
            _ => panic!("unknown test family"),
        }
    }
    fn std_hash(content: &Content) -> u64 {
        let mut state = DefaultHasher::new();
        content.hash(&mut state);
        state.finish()
    }

    #[test]
    fn carrier_debug_transparency_reaches_the_same_visible_depth_boundary() {
        let mut locator = Locator::new();
        for family_id in 0..=5 {
            let leaf = if family_id == 5 { context() } else { Content::text("VISIBLE") };
            let ordinary = family(family_id, leaf.clone());
            let expected = format!("{ordinary:?}");
            assert!(!expected.contains("..."));
            for kind in 0..4 {
                let transported =
                    family(family_id, carrier(leaf.clone(), kind, &mut locator));
                assert_eq!(
                    ordinary, transported,
                    "mechanical equality is unchanged by transport"
                );
                assert_eq!(
                    format!("{transported:?}"),
                    expected,
                    "carrier must not consume visible depth"
                );
            }
        }
    }

    #[test]
    fn carrier_hash_compatibility_covers_selected_old_and_new_content_families() {
        let mut locator = Locator::new();
        for family_id in 0..=5 {
            let leaf = if family_id == 5 { context() } else { Content::text("VISIBLE") };
            let ordinary = family(family_id, leaf.clone());
            for kind in 0..4 {
                let mut transported_leaf = leaf.clone();
                for _ in 0..12 {
                    transported_leaf = carrier(transported_leaf, kind, &mut locator);
                }
                let transported = family(family_id, transported_leaf);
                assert_eq!(ordinary, transported);
                assert_eq!(
                    crate::entities::content_hash::hash_content(&ordinary),
                    crate::entities::content_hash::hash_content(&transported)
                );
                assert_eq!(std_hash(&ordinary), std_hash(&transported));
            }
        }
        // Ordered attributes are identical here; no order-insensitive Hash promise.
        let attrs: HtmlAttrs = [("id".into(), "a".into()), ("class".into(), "b".into())]
            .into_iter()
            .collect();
        let html = Content::HtmlElem(Arc::new(HtmlElem::new(
            "div".into(),
            Some(attrs),
            HtmlBody::Content(Box::new(Content::text("A"))),
        )));
        assert_eq!(html, html.clone());
        assert_eq!(
            crate::entities::content_hash::hash_content(&html),
            crate::entities::content_hash::hash_content(&html.clone())
        );
        assert_eq!(std_hash(&html), std_hash(&html.clone()));
        assert_eq!(Content::Space, Content::SpaceFromNewline);
        assert_eq!(std_hash(&Content::Space), std_hash(&Content::SpaceFromNewline));
    }

    #[test]
    fn debug_retains_visible_max_depth_and_iterates_carriers_without_consuming_originals()
    {
        let visible = strong(Content::text("VISIBLE"), 7);
        let too_deep = strong(Content::text("VISIBLE"), 8);
        assert!(format!("{visible:?}").contains("VISIBLE"));
        let truncated = format!("{too_deep:?}");
        assert!(truncated.contains("..."));
        assert!(
            !truncated.contains("VISIBLE"),
            "visible structures still obey MAX_DEPTH8"
        );
        let realized = Arc::new(RealizedContent {
            original: Content::sequence(vec![
                Content::text("ORIGINAL_MUST_NOT_APPEAR"),
                context(),
            ]),
            visual: visible.clone(),
        });
        let prepared = Arc::new(LocatedContent {
            location: Locator::new().next(),
            body: Content::Realized(Arc::clone(&realized)),
        });
        let mut tree = Content::Located(Arc::clone(&prepared));
        let mut locator = Locator::new();
        for depth in 0..512 {
            tree = carrier(tree, depth % 4, &mut locator);
        }
        let debug = format!("{tree:?}");
        assert_eq!(debug, format!("{visible:?}"));
        assert!(!debug.contains("ORIGINAL_MUST_NOT_APPEAR"));
        assert!(
            matches!(&prepared.body,Content::Realized(e) if Arc::ptr_eq(e,&realized))
        );
        assert_eq!(realized.visual, visible);
        assert!(realized.original.plain_text().contains("ORIGINAL_MUST_NOT_APPEAR"));
        // All stored Context/recipe closures panic if executed; Debug must read data only.
    }
}
