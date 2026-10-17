//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/content/emptiness.md
//! @prompt-hash 1f542697
//! @layer L1
//! @updated 2026-10-07

use super::*;

impl Content {
    /// Return whether this content carries no visible information.
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Source(e) => e.body.is_empty(),
            Self::ShowRecipe(e) => e.body.is_empty(),
            Self::Located(e) => e.body.is_empty(),
            Self::Realized(e) => e.visual.is_empty(),
            Self::Empty => true,
            // P622: Parbreak é marker estrutural — nunca vazio.
            Self::Parbreak => false,
            // P863: parágrafo é vazio sse o body for vazio.
            Self::Par { body } => body.is_empty(),
            Self::PageRun(e) => e.is_empty(),
            Self::Sequence(v) => v.is_empty(),
            Self::Label(e) => e.is_empty(),
            // Figura: não está vazia se tiver body OU caption com conteúdo.
            Self::Figure(e) => e.is_empty(),
            // P1473 — figure.caption vazia se body for.
            Self::FigureCaption(e) => e.is_empty(),
            Self::Grid(e) => e.is_empty(),
            // P224.B — GridHeader/GridFooter vazio se body for (paridade P157C).
            // Modelo D (Lote 5 P320): grid/table header/footer delegam ao elemento.
            Self::GridHeader(e) => e.is_empty(),
            Self::GridFooter(e) => e.is_empty(),
            // P224.C — GridCell vazio se body for (paridade P157B TableCell).
            Self::GridCell(e) => e.is_empty(),
            // Passo 512 — linhas em grid/table são sempre visíveis.
            Self::GridHLine(_) | Self::GridVLine(_) => false,
            Self::TableHLine(_) | Self::TableVLine(_) => false,
            // Passo 157A (ADR-0060 Fase 2): Table é vazio se children
            // for vazio (paridade com Grid; cells / children indistintos
            // semanticamente para is_empty).
            Self::Table(e) => e.is_empty(),
            // Passo 157B (ADR-0060 Fase 2 sub-passo 2): TableCell vazio
            // se body for (atributos x/y/colspan/rowspan não tornam o
            // container não-vazio — paridade Block/Boxed).
            Self::TableCell(e) => e.is_empty(),
            // Passo 157C (ADR-0060 Fase 2 sub-passo 3): par simétrico
            // TableHeader/TableFooter vazio se body for (atributo
            // repeat não torna o container não-vazio — paridade Block/Boxed).
            Self::TableHeader(e) => e.is_empty(),
            Self::TableFooter(e) => e.is_empty(),
            // Passo 159A (ADR-0060 Fase 2 — Bibliography + Cite par
            // acoplado). Bibliography vazio se entries vazias E title
            // None. Cite nunca vazio (key sempre presente; placeholder
            // `[key]` é sempre observable).
            Self::Bibliography(e) => e.is_empty(),
            Self::Cite(e) => e.is_empty(),
            // P295 — Footnote nunca vazio (marker `[N]` é sempre observable).
            Self::Footnote(e) => e.is_empty(),
            // P1473 — footnote.entry vazia se note for.
            Self::FootnoteEntry(e) => e.is_empty(),
            // Passo 154B: Divider é singleton estrutural, nunca vazio.
            // Terms vazio (sem items) é considerado vazio; TermItem vazio
            // se ambos os lados forem vazios.
            Self::Divider(d) => d.is_empty(),
            // Modelo D (Lote 3 P318): Terms/TermItem delegam ao elemento.
            Self::Terms(e) => e.is_empty(),
            Self::TermItem(e) => e.is_empty(),
            // Passo 155: Quote vazio se body for vazio.
            Self::Quote(e) => e.is_empty(),
            // P397: Document/Asset são metadata/resources; não produzem
            // conteúdo observável no layout.
            Self::Document { .. } => true,
            Self::Asset { .. } => true,
            // P284: decoração vazia se o body for vazio (cosméticos não
            // criam observable se não há conteúdo).
            // Modelo D (Lote 4 P319): decorações delegam ao elemento.
            Self::Underline(e) => e.is_empty(),
            Self::Strike(e) => e.is_empty(),
            Self::Overline(e) => e.is_empty(),
            // P408: smallcaps é vazio sse o body for vazio (stub transparente).
            Self::SmallCaps { body } => body.is_empty(),
            // P287 — SmartQuote: nunca vazio (sempre emite 1 glyph).
            Self::SmartQuote(e) => e.is_empty(),
            Self::PdfAttach(e) => e.is_empty(),
            Self::PdfArtifact(e) => e.is_empty(),
            // Passo 156C (ADR-0061 Fase 1): Pad/Hide vazios se o body for.
            Self::Pad(e) => e.is_empty(),
            Self::Hide(e) => e.is_empty(),
            // Modelo D (Lote 5 P320): espaços/breaks delegam ao elemento
            // (HSpace/VSpace = amount.is_zero(); Pagebreak/Colbreak = false).
            Self::HSpace(e) => e.is_empty(),
            Self::VSpace(e) => e.is_empty(),
            Self::Pagebreak(e) => e.is_empty(),
            Self::Colbreak(e) => e.is_empty(),
            // Passo 156G: Block é vazio se o body for (atributos de
            // dimensão/inset não fazem o container deixar de ser vazio
            // semanticamente — análogo a Pad em P156C).
            Self::Block(e) => e.is_empty(),
            // Passo 156H: Boxed (Box inline) — proxy análogo a Block.
            Self::Boxed(e) => e.is_empty(),
            // Passo 156I: Stack é vazio se TODOS os children forem vazios
            // (consistente com Sequence; stack vazio é semanticamente
            // sem conteúdo).
            Self::Stack(e) => e.is_empty(),
            // Passo 156J: Repeat é vazio se body for (atributos não
            // tornam o container não-vazio — análogo a Block/Boxed).
            Self::Repeat(e) => e.is_empty(),
            // P217: Columns é vazio se body for (count/gutter não tornam
            // não-vazio — análogo a Block/Boxed/Repeat).
            Self::Columns(e) => e.is_empty(),
            // Lote F-1 (P334): a fronteira dinâmica delega ao elemento.
            Self::Dynamic(e) => e.dyn_is_empty(),
            // Lote F-4 S1 (P338) — fecha **B2**: um nó estilizado é vazio sse o
            // body for (o estilo não adiciona observable — paridade com
            // Block/Pad/Boxed). Antes caía em `_ => false` (styled-de-vazio
            // reportava não-vazio incorretamente).
            Self::Styled(styled) => styled.body.is_empty(),
            // F-5b fatia 1 (P371): strong/emph vazios se o body for (como Styled).
            Self::Strong(e) => e.is_empty(),
            Self::Emph(e) => e.is_empty(),
            // P1425: o wrapper de limites é semanticamente transparente
            // para vacuidade, tal como já é para texto plano.
            Self::MathLimitsOverride(e) => e.is_empty(),
            Self::Text(..)
            | Self::Space
            | Self::SpaceFromNewline
            | Self::HtmlElem(..)
            | Self::HtmlFrame(..)
            | Self::Heading(..)
            | Self::Title(..)
            | Self::Raw(..)
            | Self::ListItem(..)
            | Self::EnumItem(..)
            | Self::Link(..)
            | Self::Equation(..)
            | Self::MathSequence(..)
            | Self::MathIdent(..)
            | Self::MathText(..)
            | Self::MathFrac(..)
            | Self::MathAttach(..)
            | Self::MathPrimes(..)
            | Self::MathRoot(..)
            | Self::MathStretch(..)
            | Self::MathDelimited(..)
            | Self::MathLr(..)
            | Self::MathMid(..)
            | Self::MathOverbrace(..)
            | Self::MathUnderbrace(..)
            | Self::MathUnderbracket(..)
            | Self::MathUnderparen(..)
            | Self::MathUndershell(..)
            | Self::MathOverbracket(..)
            | Self::MathOverparen(..)
            | Self::MathOvershell(..)
            | Self::MathAlignPoint(..)
            | Self::Linebreak(..)
            | Self::MathMatrix(..)
            | Self::MathCases(..)
            | Self::MathAccent(..)
            | Self::MathCancel(..)
            | Self::MathUnderline(..)
            | Self::MathOverline(..)
            | Self::MathVec(..)
            | Self::MathClassOverride(..)
            | Self::MathUnderover(..)
            | Self::MathOp(..)
            | Self::MathStyled(..)
            | Self::Ref(..)
            | Self::CounterDisplay(..)
            | Self::CounterUpdate(..)
            | Self::Outline(..)
            | Self::OutlineEntry(..)
            | Self::Image(..)
            | Self::Shape(..)
            | Self::Curve(..)
            | Self::Transform(..)
            | Self::SetPage(..)
            | Self::Align(..)
            | Self::Place(..)
            | Self::Flush(..)
            | Self::Metadata(..)
            | Self::State(..)
            | Self::StateUpdate(..)
            | Self::StateDisplay(..)
            | Self::CounterDisplayCallback(..)
            | Self::ContextBlock(..) => false, // neutro: N16[β] — Content estrutural não-vazio (variantes sem arm dedicado) retorna false em is_empty
        }
    }
}

#[cfg(test)]
mod emptiness_tests {
    use super::Content;

    #[test]
    fn empty_variant_is_semantically_empty() {
        assert!(Content::Empty.is_empty());
        assert!(!Content::Space.is_empty());
    }
}
