//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/content/equality.md
//! @prompt-hash 2b27a2b7
//! @layer L1
//! @updated 2026-10-07

use super::*;

impl PartialEq for Content {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Source(e), b) => e.body.eq(b),
            (a, Self::Source(e)) => a.eq(&e.body),
            (Self::ShowRecipe(e), b) => e.body.eq(b),
            (Self::Located(e), b) => e.body.eq(b),
            (Self::Realized(e), b) => e.visual.eq(b),
            (a, Self::ShowRecipe(e)) => a.eq(&e.body),
            (a, Self::Located(e)) => a.eq(&e.body),
            (a, Self::Realized(e)) => a.eq(&e.visual),
            (Self::Empty, Self::Empty) => true,
            (Self::Text(a), Self::Text(b)) => a == b,
            (
                Self::Space | Self::SpaceFromNewline,
                Self::Space | Self::SpaceFromNewline,
            ) => true,
            (Self::Parbreak, Self::Parbreak) => true,
            (Self::Sequence(a), Self::Sequence(b)) => a.as_ref() == b.as_ref(),
            (Self::Par { body: a }, Self::Par { body: b }) => a == b,
            // Passo 101: Content::Strong/Emph removidos — Content::Styled cobre.
            (Self::Heading(a), Self::Heading(b)) => a == b,
            (Self::HtmlElem(a), Self::HtmlElem(b)) => {
                a.tag == b.tag
                    && match (&a.attrs, &b.attrs) {
                        (None, None) => true,
                        (Some(a), Some(b)) => {
                            a.len() == b.len()
                                && a.iter()
                                    .zip(b.iter())
                                    .all(|((ak, av), (bk, bv))| ak == bk && av == bv)
                        }
                        (None, Some(_)) | (Some(_), None) => false,
                    }
                    && a.body == b.body
            }
            (Self::HtmlFrame(a), Self::HtmlFrame(b)) => a == b,
            (Self::Title(a), Self::Title(b)) => a == b,
            // F-5b fatia 1 (P371): strong/emph variantes próprias — `==` por tipo
            // (distinto de `Styled[Bold]` → `strong ≠ #set text`, fidelidade 0107).
            (Self::Strong(a), Self::Strong(b)) => a == b,
            (Self::Emph(a), Self::Emph(b)) => a == b,
            // Modelo D (Lote 7 P322): Raw delega ao `Arc<…Elem>`.
            (Self::Raw(a), Self::Raw(b)) => a == b,
            (Self::ListItem(a), Self::ListItem(b)) => a == b,
            (Self::EnumItem(a), Self::EnumItem(b)) => a == b,
            (Self::Link(a), Self::Link(b)) => a == b,
            // Modelo D (Lote 10 P325): Equation delega ao `Arc<…Elem>`.
            (Self::Equation(a), Self::Equation(b)) => a == b,
            (Self::MathSequence(a), Self::MathSequence(b)) => a.as_ref() == b.as_ref(),
            (Self::Symbol(a), Self::Symbol(b)) => a == b,
            (Self::MathIdent(a), Self::MathIdent(b)) => a == b,
            (Self::MathText(a), Self::MathText(b)) => a == b,
            // Modelo D (Lote 2 P317): família math delega ao `Arc<…Elem>`
            // (PartialEq estrutural derivado em cada `…Elem`).
            (Self::MathFrac(a), Self::MathFrac(b)) => a == b,
            (Self::MathAttach(a), Self::MathAttach(b)) => a == b,
            (Self::MathPrimes(a), Self::MathPrimes(b)) => a == b,
            (Self::MathRoot(a), Self::MathRoot(b)) => a == b,
            (Self::MathStretch(a), Self::MathStretch(b)) => a == b,
            (Self::MathDelimited(a), Self::MathDelimited(b)) => a == b,
            (Self::MathLr(a), Self::MathLr(b)) => a == b,
            (Self::MathMid(a), Self::MathMid(b)) => a == b,
            (Self::MathOverbrace(a), Self::MathOverbrace(b)) => a == b,
            (Self::MathUnderbrace(a), Self::MathUnderbrace(b)) => a == b,
            (Self::MathUnderbracket(a), Self::MathUnderbracket(b)) => a == b,
            (Self::MathUnderparen(a), Self::MathUnderparen(b)) => a == b,
            (Self::MathUndershell(a), Self::MathUndershell(b)) => a == b,
            (Self::MathOverbracket(a), Self::MathOverbracket(b)) => a == b,
            (Self::MathOverparen(a), Self::MathOverparen(b)) => a == b,
            (Self::MathOvershell(a), Self::MathOvershell(b)) => a == b,
            (Self::MathAlignPoint(a), Self::MathAlignPoint(b)) => a == b,
            (Self::Linebreak(a), Self::Linebreak(b)) => a == b,
            (Self::MathMatrix(a), Self::MathMatrix(b)) => a == b,
            (Self::MathCases(a), Self::MathCases(b)) => a == b,
            (Self::MathAccent(a), Self::MathAccent(b)) => a == b,
            (Self::MathCancel(a), Self::MathCancel(b)) => a == b,
            (Self::MathUnderline(a), Self::MathUnderline(b)) => a == b,
            (Self::MathOverline(a), Self::MathOverline(b)) => a == b,
            (Self::MathVec(a), Self::MathVec(b)) => a == b,
            (Self::MathClassOverride(a), Self::MathClassOverride(b)) => a == b,
            (Self::MathLimitsOverride(a), Self::MathLimitsOverride(b)) => a == b,
            (Self::MathUnderover(a), Self::MathUnderover(b)) => a == b,
            (Self::MathOp(a), Self::MathOp(b)) => a == b,
            // MathStyled PartialEq estrutural (Modelo D P316: delega ao Arc<Elem>).
            (Self::MathStyled(a), Self::MathStyled(b)) => a == b,
            // P464: Label delega ao `Arc<LabelElem>` (único tipo de label).
            (Self::Label(a), Self::Label(b)) => a == b,
            (Self::Ref(a), Self::Ref(b)) => a == b,
            // Modelo D (Lote 6 P321): delegam ao `Arc<…Elem>`.
            (Self::CounterDisplay(a), Self::CounterDisplay(b)) => a == b,
            (Self::CounterUpdate(a), Self::CounterUpdate(b)) => a == b,
            (Self::Outline(a), Self::Outline(b)) => a == b,
            (Self::OutlineEntry(a), Self::OutlineEntry(b)) => a == b,
            // Modelo D (Lote 13 P328): Figure delega ao `Arc<…Elem>`.
            (Self::Figure(a), Self::Figure(b)) => a == b,
            // P1473 — FigureCaption delega ao `Arc<…Elem>`.
            (Self::FigureCaption(a), Self::FigureCaption(b)) => a == b,
            // Modelo D (Lote 7 P322): Image delega ao `Arc<…Elem>`.
            (Self::Image(a), Self::Image(b)) => a == b,
            // Modelo D (Lote 11 P326): Shape delega ao `Arc<…Elem>`.
            (Self::Shape(a), Self::Shape(b)) => a == b,
            // Passo 513: Curve delega ao `Arc<CurveElem>`.
            (Self::Curve(a), Self::Curve(b)) => a == b,
            // Modelo D (Lote 9 P324): Transform delega ao `Arc<…Elem>`.
            (Self::Transform(a), Self::Transform(b)) => a == b,
            // P224+P227+P228 — Grid refino +7 fields (gutter/align/inset/header/footer/stroke/fill).
            // Modelo D (Lote 12 P327): Grid delega ao `Arc<…Elem>`.
            (Self::Grid(a), Self::Grid(b)) => a == b,
            // P224.B — GridHeader / GridFooter (paridade P157C literal).
            // Modelo D (Lote 5 P320): delegam ao `Arc<…Elem>`.
            (Self::GridHeader(a), Self::GridHeader(b)) => a == b,
            (Self::GridFooter(a), Self::GridFooter(b)) => a == b,
            // P224.C + P230 + P235 — GridCell +5 fields cumulativos.
            // Modelo D (Lote 12 P327): GridCell delega ao `Arc<…Elem>`.
            (Self::GridCell(a), Self::GridCell(b)) => a == b,
            // Passo 512 — linhas em grid/table.
            (Self::GridHLine(a), Self::GridHLine(b)) => a == b,
            (Self::GridVLine(a), Self::GridVLine(b)) => a == b,
            (Self::TableHLine(a), Self::TableHLine(b)) => a == b,
            (Self::TableVLine(a), Self::TableVLine(b)) => a == b,
            // Modelo D (Lote 12 P327): Table delega ao `Arc<…Elem>`.
            (Self::Table(a), Self::Table(b)) => a == b,
            // Passo 157B + P230 + P235 — TableCell +5 fields cumulativos.
            // Modelo D (Lote 12 P327): TableCell delega ao `Arc<…Elem>`.
            (Self::TableCell(a), Self::TableCell(b)) => a == b,
            // Modelo D (Lote 5 P320): par simétrico TableHeader/TableFooter.
            (Self::TableHeader(a), Self::TableHeader(b)) => a == b,
            (Self::TableFooter(a), Self::TableFooter(b)) => a == b,
            // Passo 159A — par acoplado Bibliography + Cite.
            // Modelo D (Lote 10 P325): Bibliography delega ao `Arc<…Elem>`.
            (Self::Bibliography(a), Self::Bibliography(b)) => a == b,
            // Modelo D (Lote 9 P324): Cite delega ao `Arc<…Elem>`.
            (Self::Cite(a), Self::Cite(b)) => a == b,
            // P295 — Footnote PartialEq: body == body.
            // Modelo D (Lote 11 P326): Footnote delega ao `Arc<…Elem>`.
            (Self::Footnote(a), Self::Footnote(b)) => a == b,
            // P1473 — FootnoteEntry delega ao `Arc<…Elem>`.
            (Self::FootnoteEntry(a), Self::FootnoteEntry(b)) => a == b,
            (Self::PageRun(a), Self::PageRun(b)) => a == b,
            (Self::SetPage(a), Self::SetPage(b)) => a == b,
            // Modelo D (Lote 7 P322): Align delega ao `Arc<…Elem>`.
            (Self::Align(a), Self::Align(b)) => a == b,
            // Modelo D (Lote 9 P324): Place delega ao `Arc<…Elem>`.
            (Self::Place(a), Self::Place(b)) => a == b,
            (Self::Flush(a), Self::Flush(b)) => a == b,
            (Self::Styled(a), Self::Styled(b)) => a == b,
            // Passo 154B — terms + divider.
            (Self::Divider(a), Self::Divider(b)) => a == b,
            (Self::Terms(a), Self::Terms(b)) => a == b,
            (Self::TermItem(a), Self::TermItem(b)) => a == b,
            // Modelo D (Lote 8 P323): Quote delega ao `Arc<…Elem>`.
            (Self::Quote(a), Self::Quote(b)) => a == b,
            // P397: Document/Asset comparam estruturalmente por campos.
            (
                Self::Document {
                    path: p,
                    body: a,
                    format: q,
                    title: b,
                    author: c,
                    date: d,
                    keywords: e,
                },
                Self::Document {
                    path: r,
                    body: f,
                    format: s,
                    title: g,
                    author: h,
                    date: i,
                    keywords: j,
                },
            ) => p == r && a == f && q == s && b == g && c == h && d == i && e == j,
            (Self::Asset { path: a, data: b }, Self::Asset { path: c, data: d }) => {
                a == c && b == d
            }
            // Modelo D (Lote 4 P319): decorações delegam ao `Arc<…Elem>`.
            (Self::Underline(a), Self::Underline(b)) => a == b,
            (Self::Strike(a), Self::Strike(b)) => a == b,
            (Self::Overline(a), Self::Overline(b)) => a == b,
            // P408: smallcaps compara pelo body.
            (Self::SmallCaps { body: a }, Self::SmallCaps { body: b }) => a == b,
            // Modelo D (Lote 9 P324): SmartQuote delega ao `Arc<…Elem>`.
            (Self::SmartQuote(a), Self::SmartQuote(b)) => a == b,
            (Self::PdfAttach(a), Self::PdfAttach(b)) => a == b,
            (Self::PdfArtifact(a), Self::PdfArtifact(b)) => a == b,
            // Passo 156C / 156L — Pad / Hide.
            // Modelo D (Lote 10 P325): Pad delega ao `Arc<…Elem>`.
            (Self::Pad(a), Self::Pad(b)) => a == b,
            (Self::Hide(a), Self::Hide(b)) => a == b,
            // Modelo D (Lote 5 P320): espaços/breaks delegam ao `Arc<…Elem>`.
            (Self::HSpace(a), Self::HSpace(b)) => a == b,
            (Self::VSpace(a), Self::VSpace(b)) => a == b,
            (Self::Pagebreak(a), Self::Pagebreak(b)) => a == b,
            (Self::Colbreak(a), Self::Colbreak(b)) => a == b,
            // Passo 156G + P231 + P247 + P250 — Block +9 cosméticos
            // (outset/radius/clip/fill/stroke/spacing/above/below/sticky).
            // Modelo D (Lote 15 P330): Block delega ao `Arc<…Elem>`.
            (Self::Block(a), Self::Block(b)) => a == b,
            // Passo 156H + P231 + P247 — Boxed +5 cosméticos paralelo Block.
            // Modelo D (Lote 14 P329): Boxed delega ao `Arc<…Elem>`.
            (Self::Boxed(a), Self::Boxed(b)) => a == b,
            // Modelo D (Lote 9 P324): Stack delega ao `Arc<…Elem>`.
            (Self::Stack(a), Self::Stack(b)) => a == b,
            // Modelo D (Lote 7 P322): Repeat delega ao `Arc<…Elem>`.
            (Self::Repeat(a), Self::Repeat(b)) => a == b,
            // Modelo D (Lote 8 P323): Columns delega ao `Arc<…Elem>`.
            (Self::Columns(a), Self::Columns(b)) => a == b,
            // Modelo D (Lote 6 P321): StateDisplay/CounterDisplayCallback delegam.
            (Self::StateDisplay(a), Self::StateDisplay(b)) => a == b,
            (Self::CounterDisplayCallback(a), Self::CounterDisplayCallback(b)) => a == b,
            // Preserve the three documented mechanical always-unequal cases.
            (Self::Metadata(_), Self::Metadata(_))
            | (Self::State(_), Self::State(_))
            | (Self::StateUpdate(_), Self::StateUpdate(_)) => false,
            (Self::ContextBlock(a), Self::ContextBlock(b)) => a == b,
            // Lote F-1 (P334): a fronteira dinâmica compara por downcast
            // estrutural (`dyn_eq`); kinds diferentes ⇒ `false`.
            (Self::Dynamic(a), Self::Dynamic(b)) => a.dyn_eq(b.as_ref()),
            // Every same-kind policy is above; this closed product retains
            // heterogeneous inequality and forces review on either new axis.
            (
                Self::Empty
                | Self::Text(_)
                | Self::Space
                | Self::SpaceFromNewline
                | Self::Parbreak
                | Self::Sequence(_)
                | Self::HtmlElem(_)
                | Self::HtmlFrame(_)
                | Self::Par { .. }
                | Self::Heading(_)
                | Self::Title(_)
                | Self::Strong(_)
                | Self::Emph(_)
                | Self::Raw(_)
                | Self::ListItem(_)
                | Self::EnumItem(_)
                | Self::Link(_)
                | Self::Equation(_)
                | Self::MathSequence(_)
                | Self::Symbol(_)
                | Self::MathIdent(_)
                | Self::MathText(_)
                | Self::MathFrac(_)
                | Self::MathAttach(_)
                | Self::MathPrimes(_)
                | Self::MathRoot(_)
                | Self::MathStretch(_)
                | Self::MathDelimited(_)
                | Self::MathLr(_)
                | Self::MathMid(_)
                | Self::MathOverbrace(_)
                | Self::MathUnderbrace(_)
                | Self::MathUnderbracket(_)
                | Self::MathUnderparen(_)
                | Self::MathUndershell(_)
                | Self::MathOverbracket(_)
                | Self::MathOverparen(_)
                | Self::MathOvershell(_)
                | Self::MathAlignPoint(_)
                | Self::Linebreak(_)
                | Self::MathMatrix(_)
                | Self::MathCases(_)
                | Self::MathAccent(_)
                | Self::MathCancel(_)
                | Self::MathUnderline(_)
                | Self::MathOverline(_)
                | Self::MathVec(_)
                | Self::MathClassOverride(_)
                | Self::MathLimitsOverride(_)
                | Self::MathUnderover(_)
                | Self::MathOp(_)
                | Self::MathStyled(_)
                | Self::Label(_)
                | Self::Ref(_)
                | Self::CounterDisplay(_)
                | Self::CounterUpdate(_)
                | Self::Outline(_)
                | Self::OutlineEntry(_)
                | Self::Figure(_)
                | Self::FigureCaption(_)
                | Self::Image(_)
                | Self::Shape(_)
                | Self::Curve(_)
                | Self::Transform(_)
                | Self::Grid(_)
                | Self::GridHeader(_)
                | Self::GridFooter(_)
                | Self::GridCell(_)
                | Self::SetPage(_)
                | Self::PageRun(_)
                | Self::Align(_)
                | Self::Place(_)
                | Self::Flush(_)
                | Self::Styled(_)
                | Self::Divider(_)
                | Self::Terms(_)
                | Self::TermItem(_)
                | Self::Quote(_)
                | Self::Document { .. }
                | Self::Asset { .. }
                | Self::SmartQuote(_)
                | Self::PdfAttach(_)
                | Self::PdfArtifact(_)
                | Self::Underline(_)
                | Self::Strike(_)
                | Self::Overline(_)
                | Self::SmallCaps { .. }
                | Self::Pad(_)
                | Self::Hide(_)
                | Self::HSpace(_)
                | Self::VSpace(_)
                | Self::Pagebreak(_)
                | Self::Colbreak(_)
                | Self::Stack(_)
                | Self::Boxed(_)
                | Self::Block(_)
                | Self::TableCell(_)
                | Self::Bibliography(_)
                | Self::Cite(_)
                | Self::Footnote(_)
                | Self::FootnoteEntry(_)
                | Self::TableHeader(_)
                | Self::TableFooter(_)
                | Self::GridHLine(_)
                | Self::GridVLine(_)
                | Self::TableHLine(_)
                | Self::TableVLine(_)
                | Self::Table(_)
                | Self::Repeat(_)
                | Self::Columns(_)
                | Self::Metadata(_)
                | Self::State(_)
                | Self::StateUpdate(_)
                | Self::StateDisplay(_)
                | Self::CounterDisplayCallback(_)
                | Self::ContextBlock(_)
                | Self::Dynamic(_),
                Self::Empty
                | Self::Text(_)
                | Self::Space
                | Self::SpaceFromNewline
                | Self::Parbreak
                | Self::Sequence(_)
                | Self::HtmlElem(_)
                | Self::HtmlFrame(_)
                | Self::Par { .. }
                | Self::Heading(_)
                | Self::Title(_)
                | Self::Strong(_)
                | Self::Emph(_)
                | Self::Raw(_)
                | Self::ListItem(_)
                | Self::EnumItem(_)
                | Self::Link(_)
                | Self::Equation(_)
                | Self::MathSequence(_)
                | Self::Symbol(_)
                | Self::MathIdent(_)
                | Self::MathText(_)
                | Self::MathFrac(_)
                | Self::MathAttach(_)
                | Self::MathPrimes(_)
                | Self::MathRoot(_)
                | Self::MathStretch(_)
                | Self::MathDelimited(_)
                | Self::MathLr(_)
                | Self::MathMid(_)
                | Self::MathOverbrace(_)
                | Self::MathUnderbrace(_)
                | Self::MathUnderbracket(_)
                | Self::MathUnderparen(_)
                | Self::MathUndershell(_)
                | Self::MathOverbracket(_)
                | Self::MathOverparen(_)
                | Self::MathOvershell(_)
                | Self::MathAlignPoint(_)
                | Self::Linebreak(_)
                | Self::MathMatrix(_)
                | Self::MathCases(_)
                | Self::MathAccent(_)
                | Self::MathCancel(_)
                | Self::MathUnderline(_)
                | Self::MathOverline(_)
                | Self::MathVec(_)
                | Self::MathClassOverride(_)
                | Self::MathLimitsOverride(_)
                | Self::MathUnderover(_)
                | Self::MathOp(_)
                | Self::MathStyled(_)
                | Self::Label(_)
                | Self::Ref(_)
                | Self::CounterDisplay(_)
                | Self::CounterUpdate(_)
                | Self::Outline(_)
                | Self::OutlineEntry(_)
                | Self::Figure(_)
                | Self::FigureCaption(_)
                | Self::Image(_)
                | Self::Shape(_)
                | Self::Curve(_)
                | Self::Transform(_)
                | Self::Grid(_)
                | Self::GridHeader(_)
                | Self::GridFooter(_)
                | Self::GridCell(_)
                | Self::SetPage(_)
                | Self::PageRun(_)
                | Self::Align(_)
                | Self::Place(_)
                | Self::Flush(_)
                | Self::Styled(_)
                | Self::Divider(_)
                | Self::Terms(_)
                | Self::TermItem(_)
                | Self::Quote(_)
                | Self::Document { .. }
                | Self::Asset { .. }
                | Self::SmartQuote(_)
                | Self::PdfAttach(_)
                | Self::PdfArtifact(_)
                | Self::Underline(_)
                | Self::Strike(_)
                | Self::Overline(_)
                | Self::SmallCaps { .. }
                | Self::Pad(_)
                | Self::Hide(_)
                | Self::HSpace(_)
                | Self::VSpace(_)
                | Self::Pagebreak(_)
                | Self::Colbreak(_)
                | Self::Stack(_)
                | Self::Boxed(_)
                | Self::Block(_)
                | Self::TableCell(_)
                | Self::Bibliography(_)
                | Self::Cite(_)
                | Self::Footnote(_)
                | Self::FootnoteEntry(_)
                | Self::TableHeader(_)
                | Self::TableFooter(_)
                | Self::GridHLine(_)
                | Self::GridVLine(_)
                | Self::TableHLine(_)
                | Self::TableVLine(_)
                | Self::Table(_)
                | Self::Repeat(_)
                | Self::Columns(_)
                | Self::Metadata(_)
                | Self::State(_)
                | Self::StateUpdate(_)
                | Self::StateDisplay(_)
                | Self::CounterDisplayCallback(_)
                | Self::ContextBlock(_)
                | Self::Dynamic(_),
            ) => false,
        }
    }
}

#[cfg(test)]
mod equality_tests {
    use super::Content;

    #[test]
    fn causal_space_origin_is_not_structural_identity() {
        assert_eq!(Content::Space, Content::SpaceFromNewline);
    }
}

#[cfg(test)]
mod batch_g_tests {
    use super::Content;
    use crate::entities::content::{LocatedContent, RealizedContent, SourceContent};
    use crate::entities::elements::context_block::ContextBlockElem;
    use crate::entities::func::Func;
    use crate::entities::html::{HtmlAttrs, HtmlBody, HtmlElem, HtmlFrameElem};
    use crate::entities::location::Location;
    use crate::entities::span::Span;
    use std::hash::{DefaultHasher, Hash, Hasher};
    use std::sync::Arc;

    fn html(attrs: Option<HtmlAttrs>, body: HtmlBody) -> Content {
        Content::HtmlElem(Arc::new(HtmlElem::new("div".into(), attrs, body)))
    }
    fn hash(content: &Content) -> u64 {
        let mut h = DefaultHasher::new();
        content.hash(&mut h);
        h.finish()
    }

    #[test]
    fn batch_g_title_same_kind_compares_body() {
        assert_eq!(
            Content::title(Content::text("A")),
            Content::title(Content::text("A"))
        );
        assert_ne!(
            Content::title(Content::text("A")),
            Content::title(Content::text("B"))
        );
    }

    #[test]
    fn batch_g_html_frame_same_kind_compares_body() {
        let a = Content::HtmlFrame(Arc::new(HtmlFrameElem { body: Content::text("A") }));
        assert_eq!(a, a.clone());
        assert_ne!(
            a,
            Content::HtmlFrame(Arc::new(HtmlFrameElem { body: Content::text("B") }))
        );
    }

    #[test]
    fn batch_g_html_same_kind_preserves_attrs_order_presence_and_body_state() {
        let first: HtmlAttrs = [("id".into(), "a".into()), ("class".into(), "b".into())]
            .into_iter()
            .collect();
        let reverse: HtmlAttrs =
            [("class".into(), "b".into()), ("id".into(), "a".into())]
                .into_iter()
                .collect();
        let a =
            html(Some(first.clone()), HtmlBody::Content(Box::new(Content::text("A"))));
        let same = html(Some(first), HtmlBody::Content(Box::new(Content::text("A"))));
        let reordered =
            html(Some(reverse), HtmlBody::Content(Box::new(Content::text("A"))));
        assert_eq!(a, same);
        assert_ne!(a, reordered);
        assert_ne!(html(None, HtmlBody::Unset), html(None, HtmlBody::None));
        assert_ne!(
            html(None, HtmlBody::None),
            html(None, HtmlBody::Content(Box::new(Content::Empty)))
        );
        assert_ne!(
            html(None, HtmlBody::Unset),
            html(Some(HtmlAttrs::default()), HtmlBody::Unset)
        );
        for content in [
            a.clone(),
            Content::sequence(vec![Content::text("x"), a.clone()]),
            Content::title(a.clone()),
            Content::HtmlFrame(Arc::new(HtmlFrameElem { body: a.clone() })),
        ] {
            assert_eq!(content, content.clone());
            assert_eq!(hash(&content), hash(&content.clone()));
            assert_eq!(
                crate::entities::content_hash::hash_content(&content),
                crate::entities::content_hash::hash_content(&content.clone())
            );
        }
        assert_eq!(hash(&a), hash(&same));
    }

    #[test]
    fn batch_g_context_hub_equality_uses_owned_id_without_executing_closure() {
        let make = |id, name| {
            Content::ContextBlock(Arc::new(ContextBlockElem {
                id,
                closure: Func::native(name, |_, _, _, _| {
                    panic!("mechanical equality executed closure")
                }),
            }))
        };
        let a = make(7, "a");
        assert_eq!(a, make(7, "different-capability"));
        assert_ne!(a, make(8, "a"));
        assert_eq!(hash(&a), hash(&make(7, "different-capability")));
    }

    #[test]
    fn batch_g_historical_false_cases_and_transparent_heterogeneous_pairs_remain() {
        use crate::entities::elements::{
            metadata::MetadataElem, state::StateElem, state_update::StateUpdateElem,
        };
        use crate::entities::value::Value;
        for content in [
            Content::Metadata(Arc::new(MetadataElem { value: Box::new(Value::Int(1)) })),
            Content::State(Arc::new(StateElem {
                key: "k".into(),
                init: Box::new(Value::Int(1)),
            })),
            Content::StateUpdate(Arc::new(StateUpdateElem {
                key: "k".into(),
                update: crate::entities::state_update::StateUpdate::Set(Box::new(
                    Value::Int(1),
                )),
            })),
        ] {
            assert_ne!(content, content.clone());
        }
        let a = Content::text("a");
        for wrapper in [
            Content::Source(Arc::new(SourceContent {
                body: a.clone(),
                span: Span::detached(),
            })),
            Content::Located(Arc::new(LocatedContent {
                body: a.clone(),
                location: Location::from_raw(4),
            })),
            Content::Realized(Arc::new(RealizedContent {
                original: Content::text("ignored"),
                visual: a.clone(),
            })),
        ] {
            assert_eq!(a, wrapper);
            assert_eq!(wrapper, a);
            assert_ne!(wrapper, Content::Space);
            assert_ne!(Content::Space, wrapper);
        }
    }
}
