//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/content/plain_text.md
//! @prompt-hash bababcc0
//! @layer L1
//! @updated 2026-10-07

use super::*;

impl Content {
    /// Extrai texto plano recursivamente — para verificação em testes.
    pub fn plain_text(&self) -> String {
        match self {
            Self::Source(e) => e.body.plain_text(),
            Self::ShowRecipe(e) => e.body.plain_text(),
            Self::Located(e) => e.body.plain_text(),
            Self::Realized(e) => e.visual.plain_text(),
            Self::Empty => String::new(),
            Self::Text(s) => s.to_string(),
            Self::Space | Self::SpaceFromNewline => " ".to_string(),
            // P622: representação textual de separação de parágrafos.
            Self::Parbreak => "\n".to_string(),
            // P863: parágrafo é transparente para texto plano.
            Self::Par { body } => body.plain_text(),
            Self::PageRun(e) => e.plain_text(),
            Self::Sequence(v) => v.iter().map(|c| c.plain_text()).collect(),
            Self::HtmlElem(e) => {
                e.body.content().map_or_else(String::new, Content::plain_text)
            }
            Self::HtmlFrame(e) => e.body.plain_text(),
            // Passo 101: Content::Strong/Emph removidos — cobertos por
            // Content::Styled(body, _) => body.plain_text() no fim do match.
            // Modelo D (Lote 6 P321): família state/counter delega ao elemento (vazio).
            Self::Metadata(e) => e.plain_text(),
            Self::State(e) => e.plain_text(),
            Self::StateUpdate(e) => e.plain_text(),
            Self::StateDisplay(e) => e.plain_text(),
            Self::CounterDisplayCallback(e) => e.plain_text(),
            Self::Heading(h) => h.plain_text(),
            Self::Title(t) => t.plain_text(),
            // F-5b fatia 1 (P371): strong/emph transparentes ao plain_text (só body).
            Self::Strong(e) => e.plain_text(),
            Self::Emph(e) => e.plain_text(),
            Self::Raw(e) => e.plain_text(),
            // Modelo D (Lote 3 P318): família lista/termos delega ao elemento.
            Self::ListItem(e) => e.plain_text(),
            Self::EnumItem(e) => e.plain_text(),
            Self::Link(e) => e.plain_text(),
            // Modelo D (Lote 4 P319): decorações delegam ao elemento
            // (transparente — só body; cosméticos não afetam plain_text).
            Self::Underline(e) => e.plain_text(),
            Self::Strike(e) => e.plain_text(),
            Self::Overline(e) => e.plain_text(),
            // P408: smallcaps é transparente para plain_text (stub).
            Self::SmallCaps { body } => body.plain_text(),
            // P287 — SmartQuote: paridade vanilla `PlainText for
            // Packed<SmartQuoteElem>` — emite fallback ASCII (`"` ou `'`).
            // Layouter resolve lang-aware (consumer pós-P287); plain_text
            // é vista textual sem contexto lang.
            Self::SmartQuote(e) => e.plain_text(),
            Self::PdfAttach(e) => e.plain_text(),
            Self::PdfArtifact(e) => e.plain_text(),
            Self::Equation(e) => e.plain_text(),
            Self::MathSequence(nodes) => nodes.iter().map(|n| n.plain_text()).collect(),
            Self::MathIdent(s) => s.to_string(),
            Self::MathText(s) => s.to_string(),
            // Modelo D (Lote 2 P317): família math delega ao elemento.
            Self::MathFrac(e) => e.plain_text(),
            Self::MathAttach(e) => e.plain_text(),
            Self::MathPrimes(e) => e.plain_text(),
            Self::MathRoot(e) => e.plain_text(),
            Self::MathStretch(e) => e.plain_text(),
            Self::MathDelimited(e) => e.plain_text(),
            Self::MathLr(e) => e.plain_text(),
            Self::MathMid(e) => e.plain_text(),
            Self::MathOverbrace(e) => e.plain_text(),
            Self::MathUnderbrace(e) => e.plain_text(),
            Self::MathUnderbracket(e) => e.plain_text(),
            Self::MathUnderparen(e) => e.plain_text(),
            Self::MathUndershell(e) => e.plain_text(),
            Self::MathOverbracket(e) => e.plain_text(),
            Self::MathOverparen(e) => e.plain_text(),
            Self::MathOvershell(e) => e.plain_text(),
            Self::MathAlignPoint(e) => e.plain_text(),
            Self::Linebreak(e) => e.plain_text(),
            Self::MathMatrix(e) => e.plain_text(),
            Self::MathCases(e) => e.plain_text(),
            Self::MathAccent(e) => e.plain_text(),
            Self::MathCancel(e) => e.plain_text(),
            Self::MathUnderline(e) => e.plain_text(),
            Self::MathOverline(e) => e.plain_text(),
            Self::MathVec(e) => e.plain_text(),
            Self::MathClassOverride(e) => e.plain_text(),
            Self::MathLimitsOverride(e) => e.plain_text(),
            Self::MathUnderover(e) => e.plain_text(),
            Self::MathOp(e) => e.plain_text(),
            // P311b.2 — MathStyled é transparente para plain_text (wraps body).
            Self::MathStyled(m) => m.plain_text(),
            Self::Label(e) => e.plain_text(),
            Self::Ref(e) => e.plain_text(),
            Self::CounterDisplay(e) => e.plain_text(),
            Self::CounterUpdate(e) => e.plain_text(),
            Self::Outline(e) => e.plain_text(),
            Self::OutlineEntry(e) => e.plain_text(),
            Self::Figure(e) => e.plain_text(),
            Self::FigureCaption(e) => e.plain_text(),
            Self::Image(e) => e.plain_text(),
            Self::Shape(e) => e.plain_text(),
            Self::Curve(e) => e.plain_text(),
            Self::Transform(e) => e.plain_text(),
            Self::Grid(e) => e.plain_text(),
            // P224.B — GridHeader/GridFooter transparentes (paridade P157C).
            Self::GridHeader(e) => e.plain_text(),
            Self::GridFooter(e) => e.plain_text(),
            // P224.C — GridCell transparente (paridade P157B TableCell).
            Self::GridCell(e) => e.plain_text(),
            // Passo 512 — linhas em grid/table não contribuem para texto plano.
            Self::GridHLine(_) | Self::GridVLine(_) => String::new(),
            Self::TableHLine(_) | Self::TableVLine(_) => String::new(),
            // Passo 157A: Table concatena children com space (paridade
            // com Grid em plain_text — semântica de "células visíveis
            // em sequência").
            Self::Table(e) => e.plain_text(),
            // Passo 157B: TableCell é transparente para texto plano —
            // recurse no body sem multiplicar por colspan/rowspan
            // (paridade não visível em texto plano; spans são
            // runtime-only e diferidos em DEBT-34e).
            Self::TableCell(e) => e.plain_text(),
            // Passo 157C: par simétrico TableHeader/TableFooter
            // transparente para texto plano — recurse no body sem
            // multiplicar por repeat (semântica de page-break
            // repetição não visível em texto plano; diferida em
            // DEBT-56).
            Self::TableHeader(e) => e.plain_text(),
            Self::TableFooter(e) => e.plain_text(),
            // Passo 159A: Bibliography concatena title (se Some) +
            // entries formatadas. Cite emite `"[{key}]"` placeholder
            // + supplement.
            Self::Bibliography(e) => e.plain_text(),
            Self::Cite(e) => e.plain_text(),
            // P295 — Footnote plain_text: incorporar corpo (paridade
            // semântica de plain_text para search/screen readers).
            // Marker `[N]` real é resolvido em layout-time.
            Self::Footnote(e) => e.plain_text(),
            Self::FootnoteEntry(e) => e.plain_text(),
            Self::SetPage(_) => String::new(),
            Self::Align(e) => e.plain_text(),
            Self::Place(e) => e.plain_text(),
            Self::Flush(e) => e.plain_text(),
            Self::Styled(styled) => styled.body.plain_text(),
            // Passo 154B: Divider é structural sem texto; Terms concatena
            // pares por linha; TermItem produz "term: description".
            Self::Divider(d) => d.plain_text(),
            Self::Terms(e) => e.plain_text(),
            Self::TermItem(e) => e.plain_text(),
            // Passo 155: Quote em texto plain usa ASCII fallback (sem
            // smart-quotes — interaction com lang só vive no layouter).
            // Com attribution: `"body" — attribution`; sem: `"body"`.
            // Passo 156C: Pad é transparente para texto plano (recurse no
            // body sem alterar texto). Hide produz string vazia (não rende).
            Self::Pad(e) => e.plain_text(),
            Self::Hide(e) => e.plain_text(),
            // Modelo D (Lote 5 P320): espaços/breaks delegam ao elemento (vazio).
            Self::HSpace(e) => e.plain_text(),
            Self::VSpace(e) => e.plain_text(),
            Self::Pagebreak(e) => e.plain_text(),
            Self::Colbreak(e) => e.plain_text(),
            // Passo 156G: Block é transparente para texto plano (recurse
            // no body; análogo a Pad em P156C).
            Self::Block(e) => e.plain_text(),
            // Passo 156H: Boxed (Box) — análogo a Block.
            Self::Boxed(e) => e.plain_text(),
            // Passo 156I: Stack concatena plain_text de children
            // (análogo a Sequence; preserva ordem).
            Self::Stack(e) => e.plain_text(),
            // Passo 156J: Repeat é transparente para texto plano —
            // recurse no body sem multiplicar (paridade não visível em
            // texto plano; semântica de repetição é runtime-only).
            Self::Repeat(e) => e.plain_text(),
            // P217: Columns transparente para texto plano (recurse no body).
            Self::Columns(e) => e.plain_text(),
            Self::Quote(e) => e.plain_text(),
            // P397: Document devolve texto plano do título; Asset não tem
            // representação textual.
            Self::Document { title, .. } => {
                title.as_ref().map_or(String::new(), |t| t.plain_text())
            }
            Self::Asset { .. } => String::new(),
            // P506: ContextBlock é terminal para plain_text (conteúdo real só
            // existe após expansão pós-introspecção).
            Self::ContextBlock(_) => String::new(),
            // Lote F-1 (P334): a fronteira dinâmica delega ao elemento.
            Self::Dynamic(e) => e.dyn_plain_text(),
        }
    }
}

#[cfg(test)]
mod plain_text_tests {
    use super::Content;

    #[test]
    fn text_and_space_project_in_order() {
        let content = Content::sequence(vec![Content::text("a"), Content::Space]);
        assert_eq!(content.plain_text(), "a ");
    }
}
