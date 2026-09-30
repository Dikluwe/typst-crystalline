//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/content/traversal.md
//! @prompt-hash 33b93407
//! @layer L1
//! @updated 2026-09-26

use super::*;

impl Content {
    /// Traverse bottom-up and transform each node after its children.
    pub fn map_content<F>(
        &self,
        transform: &mut F,
    ) -> crate::entities::source_result::SourceResult<Self>
    where
        F: FnMut(
            &Content,
        )
            -> crate::entities::source_result::SourceResult<Option<Content>>,
    {
        // Passo 1: processar os filhos (bottom-up) para obter o nó com filhos transformados.
        let processed = match self {
            // ── Containers: propagar recursivamente ─────────────────────────
            Content::Sequence(seq) => {
                let new_seq: crate::entities::source_result::SourceResult<Vec<Content>> =
                    seq.iter().map(|c| c.map_content(transform)).collect();
                Content::Sequence(Arc::from(new_seq?))
            },
            Content::HtmlElem(e) => Content::HtmlElem(Arc::new(
                crate::entities::html::HtmlElem::new(
                    e.tag.clone(),
                    e.attrs.clone(),
                    match &e.body {
                        crate::entities::html::HtmlBody::Unset => crate::entities::html::HtmlBody::Unset,
                        crate::entities::html::HtmlBody::None => crate::entities::html::HtmlBody::None,
                        crate::entities::html::HtmlBody::Content(body) => crate::entities::html::HtmlBody::Content(Box::new(body.map_content(transform)?)),
                    },
                ),
            )),
            Content::HtmlFrame(e) => Content::HtmlFrame(Arc::new(
                crate::entities::html::HtmlFrameElem {
                    body: e.body.map_content(transform)?,
                },
            )),
            // P863: parágrafo é container transparente.
            Content::Par { body } => Content::Par {
                body: Box::new(body.map_content(transform)?),
            },
            Content::PageRun(e) => e.map_content(transform)?,
            // Passo 101: Content::Strong/Emph removidos — cobertos pelo
            // arm Content::Styled abaixo (que já propaga transform recursivamente).
            // Modelo D (P316): Heading delega ao elemento.
            Content::Heading(h) => h.map_content(transform)?,
            Content::Title(t) => t.map_content(transform)?,
            Content::Strong(e) => e.map_content(transform)?,
            Content::Emph(e)   => e.map_content(transform)?,
            // Modelo D (Lote 3 P318): família lista/termos delega ao elemento.
            Content::ListItem(e) => e.map_content(transform)?,
            Content::EnumItem(e) => e.map_content(transform)?,
            Content::Link(e)     => e.map_content(transform)?,
            Content::Label(e)    => e.map_content(transform)?,
            // Modelo D (Lote 13 P328): Figure container delega ao elemento.
            Content::Figure(e) => e.map_content(transform)?,
            // P1473 — FigureCaption delega ao elemento.
            Content::FigureCaption(e) => e.map_content(transform)?,
            // Content::Equation tem body: Box<Content> → container.
            // Modelo D (Lote 10 P325): Equation container delega ao elemento.
            Content::Equation(e) => e.map_content(transform)?,
            Content::MathSequence(seq) => {
                let new_seq: crate::entities::source_result::SourceResult<Vec<Content>> =
                    seq.iter().map(|c| c.map_content(transform)).collect();
                Content::MathSequence(Arc::from(new_seq?))
            },
            // Modelo D (Lote 2 P317): família math delega ao elemento
            // (cada `…Elem::map_content` recurse nos filhos e re-embrulha).
            Content::MathFrac(e)      => e.map_content(transform)?,
            Content::MathAttach(e)    => e.map_content(transform)?,
            Content::MathPrimes(e)    => e.map_content(transform)?,
            Content::MathRoot(e)      => e.map_content(transform)?,
            Content::MathStretch(e)   => e.map_content(transform)?,
            Content::MathDelimited(e) => e.map_content(transform)?,
            Content::MathLr(e) => e.map_content(transform)?,
            Content::MathMid(e) => e.map_content(transform)?,
            Content::MathOverbrace(e) => e.map_content(transform)?,
            Content::MathUnderbrace(e) => e.map_content(transform)?,
            Content::MathUnderbracket(e) => e.map_content(transform)?,
            Content::MathUnderparen(e) => e.map_content(transform)?,
            Content::MathUndershell(e) => e.map_content(transform)?,
            Content::MathOverbracket(e) => e.map_content(transform)?,
            Content::MathOverparen(e) => e.map_content(transform)?,
            Content::MathOvershell(e) => e.map_content(transform)?,
            Content::MathMatrix(e)    => e.map_content(transform)?,
            Content::MathCases(e)     => e.map_content(transform)?,
            Content::MathAccent(e)    => e.map_content(transform)?,
            Content::MathCancel(e)    => e.map_content(transform)?,
            Content::MathUnderline(e) => e.map_content(transform)?,
            Content::MathOverline(e) => e.map_content(transform)?,
            Content::MathVec(e) => e.map_content(transform)?,
            Content::MathClassOverride(e) => e.map_content(transform)?,
            Content::MathLimitsOverride(e) => e.map_content(transform)?,
            Content::MathUnderover(e) => e.map_content(transform)?,
            Content::MathOp(e)        => e.map_content(transform)?,
            Content::MathAlignPoint(e) => e.map_content(transform)?,
            // P311b.2 — MathStyled map_content recursivo (Modelo D P316: delega).
            Content::MathStyled(m) => m.map_content(transform)?,

            // Modelo D (Lote 3 P318): Terms/TermItem delegam ao elemento.
            Content::Terms(e)    => e.map_content(transform)?,
            Content::TermItem(e) => e.map_content(transform)?,

            // Modelo D (Lote 8 P323): Quote container delega ao elemento.
            Content::Quote(e) => e.map_content(transform)?,
            Content::PdfArtifact(e) => e.map_content(transform)?,

            // P284 — text decoration containers — recurse em body;
            // atributos cosméticos são Copy primitivos.
            // Modelo D (Lote 4 P319): decorações delegam ao elemento.
            Content::Underline(e) => e.map_content(transform)?,
            Content::Strike(e)    => e.map_content(transform)?,
            Content::Overline(e)  => e.map_content(transform)?,

            // P408: smallcaps container — recurse em body (stub transparente).
            Content::SmallCaps { body } => Content::SmallCaps {
                body: Box::new(body.map_content(transform)?),
            },

            // Passo 156C / 156L: Pad / Hide containers — recurse em body;
            // sides é Copy primitivo (Sides<Option<Length>>).
            // Modelo D (Lote 10 P325): Pad container delega ao elemento.
            Content::Pad(e) => e.map_content(transform)?,
            // Modelo D (Lote 7 P322): Hide delega ao elemento.
            Content::Hide(e) => e.map_content(transform)?,

            // Passo 156G + P231 + P247 + P250: Block container — recurse em
            // body; preserva 9 cosméticos (Sides/Corners/bool/Color Copy;
            // Stroke Clone; Option<Length> Copy; bool Copy).
            Content::Block(e) => e.map_content(transform)?,

            // Passo 156H + P231 + P247: Boxed (Box inline) — recurse análogo a Block; preserva 5 cosméticos.
            Content::Boxed(e) => e.map_content(transform)?,

            // Modelo D (Lote 9 P324): Stack container delega ao elemento.
            Content::Stack(e) => e.map_content(transform)?,

            // Passo 156J: Repeat container — recurse em body; gap e
            // justify são Copy primitivos (Option<Length>, bool).
            Content::Repeat(e) => e.map_content(transform)?,

            // Modelo D (Lote 8 P323): Columns container delega ao elemento.
            Content::Columns(e) => e.map_content(transform)?,
            Content::Outline(e) => e.map_content(transform)?,
            Content::OutlineEntry(e) => e.map_content(transform)?,

            // ── Terminais: clonar directamente ──────────────────────────────
            // Listados explicitamente — variantes novas não passam em silêncio.
            // Passo 156D: HSpace/VSpace são leaves (sem body), terminais.
            // Passo 156E: Pagebreak é leaf (event sem body), terminal.
            // P622: Parbreak é leaf estrutural — terminal.
            Content::Text(_)
            | Content::Space
            | Content::Parbreak
            | Content::SpaceFromNewline
            | Content::Empty
            | Content::Linebreak(_)
            | Content::Raw(_)
            | Content::Ref(_)
            | Content::SetPage(_)
            | Content::CounterUpdate(_)
            | Content::CounterDisplay(_)
            | Content::MathIdent(_)
            | Content::MathText(_)
            | Content::Image(_)
            | Content::Divider(_)
            // P287 — SmartQuote leaf (sem body — terminal).
            | Content::SmartQuote(_)
            | Content::PdfAttach(_)
            | Content::HSpace(_)
            | Content::VSpace(_)
            | Content::Pagebreak(_)
            // P220: Colbreak é leaf (event sem body), terminal.
            | Content::Colbreak(_)
            | Content::Shape(_)
            // Passo 513: Curve é leaf — terminal.
            | Content::Curve(_)
            // P169 (M9): Metadata é terminal — clonar directamente.
            | Content::Metadata(_)
            // P171 (M9): State e StateUpdate são terminais.
            | Content::State(_)
            | Content::StateUpdate(_)
            // P240 (M9d/M7+1): StateDisplay é terminal (callback opcional
            // mas não atravessa Content; resolvido pós-fixpoint).
            | Content::StateDisplay(_)
            // P241 (M9d/M7+2): CounterDisplayCallback terminal paralelo
            // StateDisplay; resolvido pós-fixpoint via apply_counter_displays.
            | Content::CounterDisplayCallback(_) => self.clone(),
            // Modelo D (Lote 9 P324): Transform container delega ao elemento.
            Content::Transform(e) => e.map_content(transform)?,
            // P224+P227+P228 — Grid refino +7 fields (gutter/align/inset/header/footer/stroke/fill).
            // Modelo D (Lote 12 P327): Grid container delega ao elemento.
            Content::Grid(e) => e.map_content(transform)?,
            // Modelo D (Lote 5 P320): GridHeader/GridFooter delegam ao elemento.
            Content::GridHeader(e) => e.map_content(transform)?,
            Content::GridFooter(e) => e.map_content(transform)?,
            // Modelo D (Lote 12 P327): GridCell container delega ao elemento.
            Content::GridCell(e) => e.map_content(transform)?,
            // Passo 512 — linhas em grid/table são terminais.
            Content::GridHLine(_) | Content::GridVLine(_) => self.clone(),
            Content::TableHLine(_) | Content::TableVLine(_) => self.clone(),
            // Passo 157A + P227 + P228: Table — mapear children; preservar stroke + fill.
            // Modelo D (Lote 12 P327): Table container delega ao elemento.
            Content::Table(e) => e.map_content(transform)?,
            // Modelo D (Lote 12 P327): TableCell container delega ao elemento.
            Content::TableCell(e) => e.map_content(transform)?,
            // Modelo D (Lote 5 P320): TableHeader/TableFooter delegam ao elemento.
            Content::TableHeader(e) => e.map_content(transform)?,
            Content::TableFooter(e) => e.map_content(transform)?,
            // Passo 159A: Bibliography recurse em title; preserva
            // entries (BibEntry é dados puros, sem Content recursivo).
            // Cite recurse em supplement; preserva key.
            // Modelo D (Lote 10 P325): Bibliography delega ao elemento (recurse title).
            Content::Bibliography(e) => e.map_content(transform)?,
            // Modelo D (Lote 9 P324): Cite delega ao elemento (recurse supplement).
            Content::Cite(e) => e.map_content(transform)?,
            // Modelo D (Lote 11 P326): Footnote container delega ao elemento.
            Content::Footnote(e) => e.map_content(transform)?,
            // P1473 — FootnoteEntry delega ao elemento.
            Content::FootnoteEntry(e) => e.map_content(transform)?,
            Content::Align(e) => e.map_content(transform)?,
            // Modelo D (Lote 9 P324): Place container delega ao elemento.
            Content::Place(e) => e.map_content(transform)?,
            Content::Flush(e) => e.map_content(transform)?,
            Content::Styled(styled) => Content::styled(
                styled.body.map_content(transform)?,
                styled.styles.clone(),
            ),
            // P397: Document recursa no título (único Content aninhado);
            // Asset é terminal (sem Content aninhado).
            Content::Document { path, body, format, title, author, date, keywords } => Content::Document {
                path: path.clone(),
                body: Box::new(body.map_content(transform)?),
                format: format.clone(),
                title: match title {
                    Some(body) => Some(Box::new(body.map_content(transform)?)),
                    None       => None,
                },
                author: author.clone(),
                date:   *date,
                keywords: keywords.clone(),
            },
            Content::Asset { .. } => self.clone(),
            // P506: ContextBlock é terminal em map_content (o corpo é closure,
            // não content; expansão acontece pós-introspecção).
            Content::ContextBlock(_) => self.clone(),
            // Lote F-1 (P334): a fronteira dinâmica recursa nos filhos via o
            // elemento (mesmo contrato dos 65: devolve o nó com filhos
            // transformados; o hub aplica `transform` ao nó abaixo).
            Content::Dynamic(e) => e.dyn_map_content(transform)?,
        };

        // Passo 2: aplicar a transformação ao nó já processado.
        match transform(&processed)? {
            Some(new_content) => Ok(new_content),
            None => Ok(processed),
        }
    }

    /// Forma canônica para o `==` **morfológico** da linguagem (Passo 345,
    /// ADR-0107). Remove o estilo de **render** sem o apagar da árvore real:
    /// - `Content::Text`: o `TextStyle` **assado** (heading bold, `#set text`
    ///   ambiente — render, P343 #1) → `TextStyle::default()`. A morfologia do
    ///   texto é a **string**, não o estilo resolvido.
    /// - `Content::Styled` **semanticamente vazio** (só transporte `custom`,
    ///   ex.: numbering β1) → **transparente** (desce no body). O estilo
    ///   semântico (`*bold*`/`_italic_`, em campos tipados) **permanece** —
    ///   é morfologia (o `#show strong` o vê, ADR-0038).
    /// - `numbering_active`/`numbering` (heading/equation/figure — assados da
    ///   chain `#set …(numbering:)`, P343 #2/#3/#4) → neutro. Medido contra o
    ///   vanilla (P345 N1: `#set` numbering **não** entra na igualdade).
    ///
    /// Comparar duas formas canônicas com o `==` **estrutural** (`PartialEq`)
    /// dá a igualdade **morfológica**. **Não** altera o `#[derive(PartialEq)]`
    /// do Rust — são dois sistemas de propósito (ADR-0025): o derivado serve
    /// testes/coleções; este caminho serve o `==` da linguagem (`eval`).
    /// `it.body == [a]` casa por morfologia **como consequência** (Achado 2,
    /// P342), não como alvo.
    pub fn map_text<F>(&self, transform: &mut F) -> Self
    where
        F: FnMut(&str) -> String,
    {
        match self {
            // O caso alvo: aplicar a transformação (o render vive na chain, F-5b).
            Content::Text(s) => Content::Text(transform(s.as_str()).into()),

            // ── Containers com filhos (propagação recursiva) ──────────────
            // Cada variante listada explicitamente — sem `_ =>` ou `other =>`.
            Content::Sequence(seq) => {
                Content::Sequence(
                    seq.iter().map(|c| c.map_text(transform)).collect::<Vec<_>>().into()
                )
            }
            Content::HtmlElem(e) => Content::HtmlElem(Arc::new(
                crate::entities::html::HtmlElem::new(
                    e.tag.clone(),
                    e.attrs.clone(),
                    e.body.map_content(|body| body.map_text(transform)),
                ),
            )),
            Content::HtmlFrame(e) => Content::HtmlFrame(Arc::new(
                crate::entities::html::HtmlFrameElem {
                    body: e.body.map_text(transform),
                },
            )),
            // P863: parágrafo é container transparente para map_text.
            Content::Par { body } => Content::Par {
                body: Box::new(body.map_text(transform)),
            },
            Content::PageRun(e) => e.map_text(transform),
            // Modelo D (P316): Heading delega ao elemento.
            Content::Heading(h) => h.map_text(transform),
            Content::Title(t) => t.map_text(transform),
            Content::Strong(e) => e.map_text(transform),
            Content::Emph(e)   => e.map_text(transform),
            // Passo 101: Content::Strong/Emph removidos — cobertos pelo
            // arm Content::Styled abaixo (map_text recursivo).
            Content::Label(e)    => e.map_text(transform),
            // Modelo D (Lote 13 P328): Figure container delega ao elemento.
            Content::Figure(e) => e.map_text(transform),
            // P1473 — FigureCaption delega ao elemento.
            Content::FigureCaption(e) => e.map_text(transform),
            // Modelo D (Lote 3 P318): família lista/termos delega ao elemento
            // (contentores de prosa — map_text recurse, precedente Heading).
            Content::ListItem(e) => e.map_text(transform),
            Content::EnumItem(e) => e.map_text(transform),
            Content::Link(e)     => e.map_text(transform),
            Content::Terms(e)    => e.map_text(transform),
            Content::TermItem(e) => e.map_text(transform),

            // Modelo D (Lote 8 P323): Quote container delega ao elemento.
            Content::Quote(e) => e.map_text(transform),
            Content::PdfArtifact(e) => e.map_text(transform),

            // Modelo D (Lote 4 P319): decorações delegam ao elemento
            // (contentores de prosa — map_text recurse no body).
            Content::Underline(e) => e.map_text(transform),
            Content::Strike(e)    => e.map_text(transform),
            Content::Overline(e)  => e.map_text(transform),

            // P408: smallcaps container — map_text recurse no body (stub).
            Content::SmallCaps { body } => Content::SmallCaps {
                body: Box::new(body.map_text(transform)),
            },

            // Passo 156C / 156L: Pad / Hide containers — recurse em body.
            // Modelo D (Lote 10 P325): Pad container delega ao elemento.
            Content::Pad(e) => e.map_text(transform),
            // Modelo D (Lote 7 P322): Hide delega ao elemento.
            Content::Hide(e) => e.map_text(transform),

            // Passo 156G + P247 + P250: Block container — recurse em body;
            // preserva 9 cosméticos (P247 fill/stroke + P250 spacing/above/
            // below/sticky incluídos).
            Content::Block(e) => e.map_text(transform),

            // Passo 156H + P231 + P247: Boxed (Box inline) — recurse análogo a Block; preserva 5 cosméticos.
            Content::Boxed(e) => e.map_text(transform),

            // Modelo D (Lote 9 P324): Stack container delega ao elemento.
            Content::Stack(e) => e.map_text(transform),

            // Passo 156J: Repeat container — map_text no body.
            Content::Repeat(e) => e.map_text(transform),

            // Modelo D (Lote 8 P323): Columns container delega ao elemento.
            Content::Columns(e) => e.map_text(transform),
            Content::OutlineEntry(e) => e.map_text(transform),

            // ── Terminais — clonar directamente ──────────────────────────
            // Nós matemáticos e estruturais sem markup Text — não contêm
            // Content::Text, portanto clonar em bloco é correcto e seguro.
            // Passo 156D: HSpace/VSpace são leaves (sem body), terminais.
            // Passo 156E: Pagebreak é leaf (event sem body), terminal.
            // P622: Parbreak é leaf estrutural — terminal.
            Content::Empty
            | Content::Space
            | Content::Parbreak
            | Content::SpaceFromNewline
            | Content::Linebreak(_)
            | Content::Outline(_)
            | Content::Raw(_)
            | Content::Ref(_)
            | Content::SetPage(_)
            | Content::CounterUpdate(_)
            | Content::CounterDisplay(_)
            | Content::MathIdent(_)
            | Content::MathText(_)
            // P287 — SmartQuote leaf (sem texto interno — map_text não recurse).
            | Content::SmartQuote(_)
            | Content::PdfAttach(_)
            | Content::Equation(_)
            | Content::MathSequence(_)
            // Modelo D (Lote 2 P317): família math é terminal em map_text
            // (math structural; não desce — paralelo MathStyled/Divider). O
            // bloco clona em bloco; `…Elem::map_text` existe pelo contrato.
            | Content::MathAlignPoint(_)
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
            // P311b.2 — MathStyled terminal em map_text (math structural).
            | Content::MathStyled(_)
            | Content::Image(_)
            | Content::Divider(_)
            | Content::HSpace(_)
            | Content::VSpace(_)
            | Content::Pagebreak(_)
            // P220: Colbreak é leaf (event sem body), terminal.
            | Content::Colbreak(_)
            | Content::Shape(_)
            // Passo 513: Curve é leaf — terminal.
            | Content::Curve(_)
            // P169 (M9): Metadata é terminal — clonar directamente.
            | Content::Metadata(_)
            // P171 (M9): State e StateUpdate são terminais.
            | Content::State(_)
            | Content::StateUpdate(_)
            // P240 (M9d/M7+1): StateDisplay é terminal em map_text.
            | Content::StateDisplay(_)
            // P241 (M9d/M7+2): CounterDisplayCallback terminal em map_text.
            | Content::CounterDisplayCallback(_) => self.clone(),
            // Modelo D (Lote 9 P324): Transform container delega ao elemento.
            Content::Transform(e) => e.map_text(transform),
            // P224+P227+P228 — Grid refino +7 fields (map_text).
            // Modelo D (Lote 12 P327): Grid container delega ao elemento.
            Content::Grid(e) => e.map_text(transform),
            // Modelo D (Lote 5 P320): GridHeader/GridFooter delegam ao elemento.
            Content::GridHeader(e) => e.map_text(transform),
            Content::GridFooter(e) => e.map_text(transform),
            // P224.C + P230 + P235 — GridCell recurse no body (map_text);
            // Modelo D (Lote 12 P327): GridCell container delega ao elemento.
            Content::GridCell(e) => e.map_text(transform),
            // Passo 512 — linhas em grid/table são terminais.
            Content::GridHLine(_) | Content::GridVLine(_) => self.clone(),
            Content::TableHLine(_) | Content::TableVLine(_) => self.clone(),
            // Passo 157A + P227 + P228: Table — map_text em children; preservar stroke + fill.
            // Modelo D (Lote 12 P327): Table container delega ao elemento.
            Content::Table(e) => e.map_text(transform),
            // Modelo D (Lote 12 P327): TableCell container delega ao elemento.
            Content::TableCell(e) => e.map_text(transform),
            // Modelo D (Lote 5 P320): TableHeader/TableFooter delegam ao elemento.
            Content::TableHeader(e) => e.map_text(transform),
            Content::TableFooter(e) => e.map_text(transform),
            // Passo 159A: Bibliography map_text em title; entries
            // são dados puros (String fields) — sem map_text recursivo
            // em entries (mapeamento de strings em fields entities é
            // out of scope per ADR-0033 paridade observable).
            // Modelo D (Lote 10 P325): Bibliography delega ao elemento (recurse title).
            Content::Bibliography(e) => e.map_text(transform),
            // Modelo D (Lote 9 P324): Cite delega ao elemento (recurse supplement).
            Content::Cite(e) => e.map_text(transform),
            // Modelo D (Lote 11 P326): Footnote container delega ao elemento.
            Content::Footnote(e) => e.map_text(transform),
            // P1473 — FootnoteEntry delega ao elemento.
            Content::FootnoteEntry(e) => e.map_text(transform),
            Content::Align(e) => e.map_text(transform),
            // Modelo D (Lote 9 P324): Place container delega ao elemento.
            Content::Place(e) => e.map_text(transform),
            Content::Flush(e) => e.map_text(transform),
            Content::Styled(styled) => Content::styled(
                styled.body.map_text(transform),
                styled.styles.clone(),
            ),
            // P397: Document recursa no título; Asset é terminal.
            Content::Document { path, body, format, title, author, date, keywords } => Content::Document {
                path: path.clone(),
                body: Box::new(body.map_text(transform)),
                format: format.clone(),
                title: title.as_ref().map(|body| Box::new(body.map_text(transform))),
                author: author.clone(),
                date:   *date,
                keywords: keywords.clone(),
            },
            Content::Asset { .. } => self.clone(),
            // P506: ContextBlock é terminal em map_text.
            Content::ContextBlock(_) => self.clone(),
            // Lote F-1 (P334): a fronteira dinâmica delega ao elemento.
            Content::Dynamic(e) => e.dyn_map_text(transform),
        }
    }
}

#[cfg(test)]
mod traversal_tests {
    use super::Content;

    #[test]
    fn text_mapping_preserves_the_closed_variant() {
        let mut uppercase = |text: &str| text.to_uppercase().into();
        assert_eq!(Content::text("a").map_text(&mut uppercase), Content::text("A"));
    }
}
