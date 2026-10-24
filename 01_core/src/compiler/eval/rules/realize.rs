//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/rules/realize.md
//! @prompt-hash f4c6bd24
//! @layer L1
//! @updated 2026-10-07
//!
//! Comportamento de `eval/rules::realize`.

use super::*;

macro_rules! realize_body {
    ($e:expr, $variant:path) => {{
        let mut elem = Arc::unwrap_or_clone(Arc::clone($e));
        elem.body = realize_node(&elem.body);
        $variant(Arc::new(elem))
    }};
}

/// **P863** — Realiza parágrafos antes de aplicar show rules.
///
/// Agrupa, em cada contexto de fluxo (`Sequence`), o conteúdo entre
/// `Content::Parbreak`s em nós `Content::Par`. Sub-árvores matemáticas
/// (`Equation`, `MathSequence` e família `Math*`) são preservadas sem wrapping.
pub(crate) fn realize_paragraphs(content: Content) -> Content {
    realize_scoped_flow(&content)
}

/// Retorna os itens de um fluxo já realizados. `Sequence` é achatada e os
/// trechos entre `Parbreak` são embrulhados em `Content::Par`.
pub(crate) fn is_flow_carrier(content: &Content) -> bool {
    matches!(
        content.source_body(),
        Content::Empty | Content::Space | Content::SpaceFromNewline | Content::SetPage(_)
    )
}

fn flow_boundary_body(mut content: &Content) -> &Content {
    loop {
        content = match content {
            Content::Source(source) => &source.body,
            Content::ShowRecipe(recipe) => &recipe.body,
            Content::Realized(_)
            | Content::Located(_)
            | Content::Empty
            | Content::Text(_)
            | Content::Space
            | Content::SpaceFromNewline
            | Content::Parbreak
            | Content::Sequence(_)
            | Content::HtmlElem(_)
            | Content::HtmlFrame(_)
            | Content::Par { .. }
            | Content::Heading(_)
            | Content::Title(_)
            | Content::Strong(_)
            | Content::Emph(_)
            | Content::Raw(_)
            | Content::ListItem(_)
            | Content::EnumItem(_)
            | Content::Link(_)
            | Content::Equation(_)
            | Content::MathSequence(_)
            | Content::Symbol(_)
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
            | Content::Outline(_)
            | Content::OutlineEntry(_)
            | Content::Figure(_)
            | Content::FigureCaption(_)
            | Content::Image(_)
            | Content::Shape(_)
            | Content::Curve(_)
            | Content::Transform(_)
            | Content::Grid(_)
            | Content::GridHeader(_)
            | Content::GridFooter(_)
            | Content::GridCell(_)
            | Content::SetPage(_)
            | Content::PageRun(_)
            | Content::Align(_)
            | Content::Place(_)
            | Content::Flush(_)
            | Content::Styled(_)
            | Content::Divider(_)
            | Content::Terms(_)
            | Content::TermItem(_)
            | Content::Quote(_)
            | Content::Document { .. }
            | Content::Asset { .. }
            | Content::SmartQuote(_)
            | Content::PdfAttach(_)
            | Content::PdfArtifact(_)
            | Content::Underline(_)
            | Content::Strike(_)
            | Content::Overline(_)
            | Content::SmallCaps { .. }
            | Content::Pad(_)
            | Content::Hide(_)
            | Content::HSpace(_)
            | Content::VSpace(_)
            | Content::Pagebreak(_)
            | Content::Colbreak(_)
            | Content::Stack(_)
            | Content::Boxed(_)
            | Content::Block(_)
            | Content::TableCell(_)
            | Content::Bibliography(_)
            | Content::Cite(_)
            | Content::Footnote(_)
            | Content::FootnoteEntry(_)
            | Content::TableHeader(_)
            | Content::TableFooter(_)
            | Content::GridHLine(_)
            | Content::GridVLine(_)
            | Content::TableHLine(_)
            | Content::TableVLine(_)
            | Content::Table(_)
            | Content::Repeat(_)
            | Content::Columns(_)
            | Content::Metadata(_)
            | Content::State(_)
            | Content::StateUpdate(_)
            | Content::StateDisplay(_)
            | Content::CounterDisplayCallback(_)
            | Content::ContextBlock(_)
            | Content::Dynamic(_) => return content,
        };
    }
}

pub(crate) fn realize_flow(content: &Content) -> Vec<Content> {
    match content {
        Content::Source(e) => realize_flow(&e.body)
            .into_iter()
            .map(|c| c.with_source_span(e.span))
            .collect(),
        Content::Sequence(seq) => {
            let mut out = Vec::new();
            let mut current = Vec::new();
            for (index, child) in seq.iter().enumerate() {
                let scoped_prefix = matches!(flow_boundary_body(child), Content::Styled(styled)
                    if styled.styles.delta().custom.iter().any(|(key, value)|
                        key.as_str() == "eval.flow.scope" && matches!(value, Value::Bool(true))));
                let whole_style = matches!(flow_boundary_body(child), Content::Styled(_))
                    && seq.iter().skip(index + 1).all(is_flow_carrier);
                if (scoped_prefix || whole_style) && current.iter().all(is_flow_carrier) {
                    out.append(&mut current);
                    out.push(realize_scoped_flow(child));
                    continue;
                }
                for piece in realize_flow(child) {
                    if matches!(piece.source_body(), Content::Parbreak)
                        || is_paragraph_node(&piece)
                    {
                        if !current.is_empty() {
                            out.push(Content::par(Content::sequence(current)));
                            current = Vec::new();
                        }
                        out.push(piece);
                    } else {
                        current.push(piece);
                    }
                }
            }
            if !current.is_empty() {
                out.push(Content::par(Content::sequence(current)));
            }
            out
        }
        Content::Styled(styled) => vec![Content::styled(
            realize_inline_node(&styled.body),
            styled.styles.clone(),
        )],
        other => vec![realize_node(other)],
    }
}

pub(crate) fn realize_scoped_flow(content: &Content) -> Content {
    match content {
        Content::Source(e) => realize_scoped_flow(&e.body).with_source_span(e.span),
        Content::ShowRecipe(recipe) => {
            Content::ShowRecipe(Arc::new(crate::entities::content::ShowRecipeContent {
                body: realize_scoped_flow(&recipe.body),
                ..recipe.as_ref().clone()
            }))
        }
        Content::Styled(styled) => {
            Content::styled(realize_scoped_flow(&styled.body), styled.styles.clone())
        }
        Content::Text(_) => Content::par(realize_inline_node(content)),
        other => realize_node(other),
    }
}

pub(crate) fn is_paragraph_node(content: &Content) -> bool {
    match content {
        Content::Source(source) => is_paragraph_node(&source.body),
        Content::ShowRecipe(recipe) => is_paragraph_node(&recipe.body),
        Content::Par { .. } => true,
        Content::Styled(styled) => is_paragraph_node(&styled.body),
        // This early predicate recognizes prepared nodes, not arbitrary projections.
        // Carrier reentry and broader flow preparation remain separate obligations.
        Content::Realized(_)
        | Content::Located(_)
        | Content::Empty
        | Content::Text(_)
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
        | Content::Sequence(_)
        | Content::HtmlElem(_)
        | Content::HtmlFrame(_)
        | Content::Heading(_)
        | Content::Title(_)
        | Content::Strong(_)
        | Content::Emph(_)
        | Content::Raw(_)
        | Content::ListItem(_)
        | Content::EnumItem(_)
        | Content::Link(_)
        | Content::Equation(_)
        | Content::MathSequence(_)
        | Content::Symbol(_)
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
        | Content::Outline(_)
        | Content::OutlineEntry(_)
        | Content::Figure(_)
        | Content::FigureCaption(_)
        | Content::Image(_)
        | Content::Shape(_)
        | Content::Curve(_)
        | Content::Transform(_)
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
        | Content::Quote(_)
        | Content::Document { .. }
        | Content::Asset { .. }
        | Content::SmartQuote(_)
        | Content::PdfAttach(_)
        | Content::PdfArtifact(_)
        | Content::Underline(_)
        | Content::Strike(_)
        | Content::Overline(_)
        | Content::SmallCaps { .. }
        | Content::Pad(_)
        | Content::Hide(_)
        | Content::HSpace(_)
        | Content::VSpace(_)
        | Content::Pagebreak(_)
        | Content::Colbreak(_)
        | Content::Stack(_)
        | Content::Boxed(_)
        | Content::Block(_)
        | Content::TableCell(_)
        | Content::Bibliography(_)
        | Content::Cite(_)
        | Content::Footnote(_)
        | Content::FootnoteEntry(_)
        | Content::TableHeader(_)
        | Content::TableFooter(_)
        | Content::GridHLine(_)
        | Content::GridVLine(_)
        | Content::TableHLine(_)
        | Content::TableVLine(_)
        | Content::Table(_)
        | Content::Repeat(_)
        | Content::Columns(_)
        | Content::Metadata(_)
        | Content::State(_)
        | Content::StateUpdate(_)
        | Content::StateDisplay(_)
        | Content::CounterDisplayCallback(_)
        | Content::ContextBlock(_)
        | Content::Dynamic(_) => false,
    }
}

pub(crate) fn realize_inline_node(content: &Content) -> Content {
    match content {
        Content::Source(e) => realize_inline_node(&e.body).with_source_span(e.span),
        Content::Sequence(seq) => {
            Content::sequence(seq.iter().map(realize_inline_node).collect())
        }
        Content::Styled(styled) => {
            Content::styled(realize_inline_node(&styled.body), styled.styles.clone())
        }
        Content::Par { body } => Content::par(realize_inline_node(body)),
        other => realize_node(other),
    }
}

pub(crate) fn realize_node(content: &Content) -> Content {
    match content {
        Content::Source(e) => realize_node(&e.body).with_source_span(e.span),
        Content::Sequence(_) => Content::sequence(realize_flow(content)),
        Content::Par { body } => Content::par(realize_inline_node(body)),
        Content::Styled(styled) => {
            Content::styled(realize_node(&styled.body), styled.styles.clone())
        }
        Content::Document { path, body, format, title, author, date, keywords } => {
            Content::Document {
                path: path.clone(),
                body: Box::new(realize_node(body)),
                format: format.clone(),
                title: title.as_ref().map(|t| Box::new(realize_node(t))),
                author: author.clone(),
                date: *date,
                keywords: keywords.clone(),
            }
        }

        // Containers com um único `body`.
        Content::Block(e) => realize_body!(e, Content::Block),
        Content::Boxed(e) => realize_body!(e, Content::Boxed),
        Content::Pad(e) => realize_body!(e, Content::Pad),
        Content::Align(e) => realize_body!(e, Content::Align),
        Content::Hide(e) => realize_body!(e, Content::Hide),
        Content::Transform(e) => realize_body!(e, Content::Transform),
        Content::Columns(e) => realize_body!(e, Content::Columns),
        Content::Repeat(e) => realize_body!(e, Content::Repeat),
        Content::ListItem(e) => realize_body!(e, Content::ListItem),
        Content::EnumItem(e) => realize_body!(e, Content::EnumItem),
        Content::Footnote(e) => realize_body!(e, Content::Footnote),
        Content::Label(e) => realize_body!(e, Content::Label),
        Content::Strong(e) => realize_body!(e, Content::Strong),
        Content::Emph(e) => realize_body!(e, Content::Emph),
        Content::Underline(e) => realize_body!(e, Content::Underline),
        Content::Strike(e) => realize_body!(e, Content::Strike),
        Content::Overline(e) => realize_body!(e, Content::Overline),
        Content::GridCell(e) => realize_body!(e, Content::GridCell),
        Content::GridHeader(e) => realize_body!(e, Content::GridHeader),
        Content::GridFooter(e) => realize_body!(e, Content::GridFooter),
        Content::TableCell(e) => realize_body!(e, Content::TableCell),
        Content::TableHeader(e) => realize_body!(e, Content::TableHeader),
        Content::TableFooter(e) => realize_body!(e, Content::TableFooter),
        Content::Title(e) => realize_body!(e, Content::Title),
        // `OutlineElem` não tem campo `body`; é folha para realização.
        Content::Outline(_) => content.clone(),
        Content::MathStyled(e) => realize_body!(e, Content::MathStyled),

        // Containers com body explícito inline.
        Content::SmallCaps { body } => Content::smallcaps(realize_node(body)),

        // Containers com múltiplos conteúdos.
        Content::Figure(e) => {
            let mut e = Arc::unwrap_or_clone(Arc::clone(e));
            e.body = realize_node(&e.body);
            e.caption = e.caption.as_ref().map(realize_node);
            Content::Figure(Arc::new(e))
        }
        Content::Quote(e) => {
            let mut e = Arc::unwrap_or_clone(Arc::clone(e));
            e.body = realize_node(&e.body);
            e.attribution = e.attribution.as_ref().map(realize_node);
            Content::Quote(Arc::new(e))
        }
        Content::TermItem(e) => {
            let mut e = Arc::unwrap_or_clone(Arc::clone(e));
            e.term = realize_node(&e.term);
            e.description = realize_node(&e.description);
            Content::TermItem(Arc::new(e))
        }
        Content::Grid(e) => {
            let mut e = Arc::unwrap_or_clone(Arc::clone(e));
            e.cells = e.cells.iter().map(realize_node).collect();
            e.header = e.header.as_ref().map(realize_node);
            e.footer = e.footer.as_ref().map(realize_node);
            Content::Grid(Arc::new(e))
        }
        Content::Table(e) => {
            let mut e = Arc::unwrap_or_clone(Arc::clone(e));
            e.children = e.children.iter().map(realize_node).collect();
            e.caption = e.caption.as_ref().map(realize_node);
            Content::Table(Arc::new(e))
        }
        Content::Stack(e) => {
            let mut e = Arc::unwrap_or_clone(Arc::clone(e));
            e.children =
                Arc::from(e.children.iter().map(realize_node).collect::<Vec<_>>());
            Content::Stack(Arc::new(e))
        }
        Content::Terms(e) => {
            let mut e = Arc::unwrap_or_clone(Arc::clone(e));
            e.items = e.items.iter().map(realize_node).collect();
            Content::Terms(Arc::new(e))
        }

        // Matemática e folhas: preservar sem wrapping.
        other => other.clone(),
    }
}

#[cfg(test)]
mod v16_c_paragraph_tests {
    use super::*;
    use crate::contracts::world::World;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library,
    };
    use crate::entities::{file_id::FileId, font_book::FontBook, source::Source};
    struct MemoryWorld {
        library: Library,
        book: FontBook,
        source: Source,
    }
    impl World for MemoryWorld {
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
    fn world(text: &str) -> MemoryWorld {
        let id = FileId::from_raw(std::num::NonZeroU16::new(1).unwrap());
        MemoryWorld {
            library: Library::new(),
            book: FontBook::new(),
            source: Source::new(id, text.into()).expect("valid Source test fixture"),
        }
    }

    #[test]
    fn v16_c_paragraph_source_producers_preserve_matched_effects() {
        let sources = [
            "#show par:it=>{metadata(\"P\");it}\n#show heading:it=>par(it.body)\n= A",
            "#show par:it=>{metadata(\"P\");it}\n#show heading:it=>par(it.body)\n#text(fill:red)[#heading[A]]",
            "#show par:it=>{metadata(\"P\");it}\n#show heading:it=>it.body\n= A",
        ];
        for source in sources {
            let world = world(source);
            let (result, _) = crate::compiler::eval::tests::eval_for_test_keep_sink(
                &world,
                &world.source,
            );
            let module = result.expect("measured paragraph producer must evaluate");
            let content = module.content().expect("producer emits content");
            let mut effects = Vec::new();
            content.visit_content_static(&mut |node| {
                if let Content::Metadata(metadata) = node {
                    effects.push(metadata.value.as_ref().clone());
                }
                true
            });
            assert_eq!(effects, vec![Value::Str("P".into())], "{source}");
        }
    }

    #[test]
    fn v16_c_paragraph_recognition_preserves_lexical_carriers() {
        use crate::entities::content::ShowRecipeContent;
        let file = FileId::from_raw(std::num::NonZeroU16::new(1).unwrap());
        let span = crate::entities::span::Span::from_range(file, 10..20);
        let par = Content::par(Content::text("BODY"));
        let sourced = par.clone().with_source_span(span);
        let recipe = Arc::new(ShowRecipeContent {
            body: sourced.clone(),
            textual: false,
            context_id_floor: 17,
            rules: Arc::from([]),
            styles: crate::entities::style_chain::StyleChain::default_chain(),
            guards: vec![23],
            file,
        });
        let wrapped = Content::styled(
            Content::ShowRecipe(recipe.clone()),
            crate::entities::style::Styles::default(),
        );
        for content in [&par, &sourced, &wrapped] {
            assert!(is_paragraph_node(content));
        }
        let sourced_flow = realize_flow(&sourced);
        assert_eq!(sourced_flow.len(), 1);
        assert_eq!(sourced_flow[0].source_span(), span);
        let flow = realize_flow(&Content::sequence(vec![
            Content::text("PREFIX"),
            wrapped,
            Content::text("SUFFIX"),
        ]));
        assert_eq!(flow.len(), 3);
        assert!(flow.iter().all(is_paragraph_node));
        let Content::Styled(styled) = &flow[1] else {
            panic!("lexical style boundary lost")
        };
        let Content::ShowRecipe(retained) = &styled.body else {
            panic!("snapshot was executed or removed")
        };
        assert!(Arc::ptr_eq(&recipe, retained));
        assert_eq!(retained.guards, vec![23]);
        assert_eq!(retained.context_id_floor, 17);
    }

    #[test]
    fn v16_c_paragraph_recognition_stays_at_the_prepared_node() {
        use crate::entities::content::LocatedContent;
        let paragraph = Content::par(Content::text("A"));
        let sequence = Content::sequence(vec![paragraph.clone(), Content::text("B")]);
        assert!(!is_paragraph_node(&sequence));
        assert!(!is_paragraph_node(&Content::text("inline")));
        let heading = Content::Located(Arc::new(LocatedContent {
            location: crate::entities::locator::Locator::new().next(),
            body: Content::heading(1, Content::text("H")),
        }));
        assert!(!is_paragraph_node(&heading));
        let flow = realize_flow(&Content::sequence(vec![
            Content::text("PREFIX"),
            paragraph,
            Content::text("SUFFIX"),
        ]));
        assert_eq!(flow.len(), 3);
        assert_eq!(
            flow.iter().map(Content::plain_text).collect::<Vec<_>>(),
            vec!["PREFIX", "A", "SUFFIX"]
        );
    }
    fn assert_one_frontier_paragraph(content: &Content) {
        let mut paragraphs = 0;
        content.visit_content_static(&mut |node| {
            paragraphs += usize::from(matches!(node, Content::Par { .. }));
            true
        });
        assert_eq!(
            paragraphs, 1,
            "one actual flow root must yield one paragraph: {content:?}"
        );
        assert_eq!(content.plain_text(), "BODY");
    }

    #[test]
    fn remaining_parity_root_paragraph_direct_text() {
        let realized = realize_paragraphs(Content::text("BODY"));
        assert_one_frontier_paragraph(&realized);
        assert!(matches!(realized, Content::Par { .. }));
    }

    #[test]
    fn remaining_parity_root_paragraph_source_span() {
        let file = FileId::from_raw(std::num::NonZeroU16::new(1).unwrap());
        let span = crate::entities::span::Span::from_range(file, 3..7);
        let realized = realize_paragraphs(Content::text("BODY").with_source_span(span));
        assert_one_frontier_paragraph(&realized);
        assert_eq!(realized.source_span(), span);
        assert!(matches!(realized.source_body(), Content::Par { .. }));
    }

    #[test]
    fn remaining_parity_root_paragraph_outer_style() {
        use crate::entities::style::{Style, Styles};
        let styles = Styles::from_iter([Style::bold(true)]);
        let realized =
            realize_paragraphs(Content::styled(Content::text("BODY"), styles.clone()));
        assert_one_frontier_paragraph(&realized);
        let Content::Styled(styled) = realized else {
            panic!("the actual lexical scope must remain outside the paragraph")
        };
        assert_eq!(styled.styles, styles);
        assert!(matches!(&styled.body, Content::Par { .. }));
    }

    #[test]
    fn remaining_parity_root_existing_paragraph_not_nested() {
        use crate::entities::style::{Style, Styles};
        let par = Content::par(Content::text("BODY"));
        let file = FileId::from_raw(std::num::NonZeroU16::new(1).unwrap());
        let span = crate::entities::span::Span::from_range(file, 3..7);
        for root in [
            par.clone(),
            par.clone().with_source_span(span),
            Content::styled(par, Styles::from_iter([Style::bold(true)])),
        ] {
            let once = realize_paragraphs(root);
            assert_one_frontier_paragraph(&once);
            assert_one_frontier_paragraph(&realize_paragraphs(once));
        }
    }
}
