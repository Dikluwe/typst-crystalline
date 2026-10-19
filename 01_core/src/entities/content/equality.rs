//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/content/equality.md
//! @prompt-hash b184b499
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
            // Quirk pré-existente preservado (Lote 6 P321): Metadata/State/
            // StateUpdate NÃO têm arm — caem em `_ => false` (sempre desiguais).
            // Lote F-1 (P334): a fronteira dinâmica compara por downcast
            // estrutural (`dyn_eq`); kinds diferentes ⇒ `false`.
            (Self::Dynamic(a), Self::Dynamic(b)) => a.dyn_eq(b.as_ref()),
            _ => false, // neutro: N16[β] — variantes de Content sem arm de PartialEq retornam false (sempre desiguais)
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
