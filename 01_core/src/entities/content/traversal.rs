//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/content/traversal.md
//! @prompt-hash e600e78b
//! @layer L1
//! @updated 2026-10-10

use super::*;
use crate::entities::source_result::SourceResult;

impl Content {
    /// Borrow static children; false prunes descendants. Dynamic stays opaque.
    pub(crate) fn visit_content_static<F>(&self, visit: &mut F)
    where
        F: FnMut(&Content) -> bool,
    {
        if let Content::Source(e) = self {
            e.body.visit_content_static(visit);
            return;
        }
        if let Content::ShowRecipe(e) = self {
            e.body.visit_content_static(visit);
            return;
        }
        if let Content::Located(e) = self {
            e.body.visit_content_static(visit);
            return;
        }
        if let Content::Realized(e) = self {
            e.visual.visit_content_static(visit);
            return;
        }
        if !visit(self) {
            return;
        }
        match self {
            Content::Source(_)
            | Content::ShowRecipe(_)
            | Content::Located(_)
            | Content::Realized(_) => {
                unreachable!("transparent carriers dispatched above")
            }
            Content::Sequence(parts) | Content::MathSequence(parts) => {
                for child in parts.iter() {
                    child.visit_content_static(visit);
                }
            }
            Content::Par { body } | Content::SmallCaps { body } => {
                body.visit_content_static(visit)
            }
            Content::HtmlElem(e) => {
                if let crate::entities::html::HtmlBody::Content(body) = &e.body {
                    body.visit_content_static(visit);
                }
            }
            Content::PageRun(e) => {
                for marginal in [&e.header, &e.footer] {
                    if let Some(crate::entities::page_running::PageMarginal::Content(c)) =
                        marginal
                    {
                        c.visit_content_static(visit);
                    }
                }
                if let Some(crate::entities::page_supplement::PageSupplement::Content(
                    c,
                )) = &e.supplement
                {
                    c.visit_content_static(visit);
                }
                for layer in [&e.background, &e.foreground] {
                    if let Some(Some(c)) = layer {
                        c.visit_content_static(visit);
                    }
                }
                e.body.visit_content_static(visit);
            }
            Content::HtmlFrame(e) => e.body.visit_content_static(visit),
            Content::Heading(e) => e.body.visit_content_static(visit),
            Content::Title(e) => e.body.visit_content_static(visit),
            Content::Strong(e) => e.body.visit_content_static(visit),
            Content::Emph(e) => e.body.visit_content_static(visit),
            Content::ListItem(e) => e.body.visit_content_static(visit),
            Content::EnumItem(e) => e.body.visit_content_static(visit),
            Content::Link(e) => e.body.visit_content_static(visit),
            Content::Label(e) => e.body.visit_content_static(visit),
            Content::Equation(e) => e.body.visit_content_static(visit),
            Content::MathStretch(e) => e.body.visit_content_static(visit),
            Content::MathDelimited(e) => e.body.visit_content_static(visit),
            Content::MathLr(e) => e.body.visit_content_static(visit),
            Content::MathMid(e) => e.body.visit_content_static(visit),
            Content::MathCancel(e) => e.body.visit_content_static(visit),
            Content::MathUnderline(e) => e.body.visit_content_static(visit),
            Content::MathOverline(e) => e.body.visit_content_static(visit),
            Content::MathClassOverride(e) => e.body.visit_content_static(visit),
            Content::MathLimitsOverride(e) => e.body.visit_content_static(visit),
            Content::MathStyled(e) => e.body.visit_content_static(visit),
            Content::PdfArtifact(e) => e.body.visit_content_static(visit),
            Content::Underline(e) => e.body.visit_content_static(visit),
            Content::Strike(e) => e.body.visit_content_static(visit),
            Content::Overline(e) => e.body.visit_content_static(visit),
            Content::Pad(e) => e.body.visit_content_static(visit),
            Content::Hide(e) => e.body.visit_content_static(visit),
            Content::Block(e) => e.body.visit_content_static(visit),
            Content::Boxed(e) => e.body.visit_content_static(visit),
            Content::Repeat(e) => e.body.visit_content_static(visit),
            Content::Columns(e) => e.body.visit_content_static(visit),
            Content::Transform(e) => e.body.visit_content_static(visit),
            Content::GridHeader(e) => e.body.visit_content_static(visit),
            Content::GridFooter(e) => e.body.visit_content_static(visit),
            Content::GridCell(e) => e.body.visit_content_static(visit),
            Content::TableHeader(e) => e.body.visit_content_static(visit),
            Content::TableFooter(e) => e.body.visit_content_static(visit),
            Content::Footnote(e) => e.body.visit_content_static(visit),
            Content::Align(e) => e.body.visit_content_static(visit),
            Content::Place(e) => e.body.visit_content_static(visit),
            Content::Styled(e) => e.body.visit_content_static(visit),
            Content::Figure(e) => {
                e.body.visit_content_static(visit);
                if let Some(c) = &e.caption {
                    c.visit_content_static(visit);
                }
            }
            Content::FigureCaption(e) => {
                e.body.visit_content_static(visit);
                if let Some(c) = &e.separator {
                    c.visit_content_static(visit);
                }
            }
            Content::MathFrac(e) => {
                e.num.visit_content_static(visit);
                e.den.visit_content_static(visit);
            }
            Content::MathAttach(e) => {
                e.base.visit_content_static(visit);
                for slot in [&e.t, &e.b, &e.tl, &e.bl, &e.tr, &e.br] {
                    if let crate::entities::elements::math_attach::MathAttachSlot::Present(c) = slot { c.visit_content_static(visit); }
                }
            }
            Content::MathRoot(e) => {
                if let Some(c) = &e.index {
                    c.visit_content_static(visit);
                }
                e.radicand.visit_content_static(visit);
            }
            Content::MathOverbrace(e) => {
                e.body.visit_content_static(visit);
                if let Some(c) = &e.annotation {
                    c.visit_content_static(visit);
                }
            }
            Content::MathUnderbrace(e) => {
                e.body.visit_content_static(visit);
                if let Some(c) = &e.annotation {
                    c.visit_content_static(visit);
                }
            }
            Content::MathUnderbracket(e) => {
                e.body.visit_content_static(visit);
                if let Some(c) = &e.annotation {
                    c.visit_content_static(visit);
                }
            }
            Content::MathUnderparen(e) => {
                e.body.visit_content_static(visit);
                if let Some(c) = &e.annotation {
                    c.visit_content_static(visit);
                }
            }
            Content::MathUndershell(e) => {
                e.body.visit_content_static(visit);
                if let Some(c) = &e.annotation {
                    c.visit_content_static(visit);
                }
            }
            Content::MathOverbracket(e) => {
                e.body.visit_content_static(visit);
                if let Some(c) = &e.annotation {
                    c.visit_content_static(visit);
                }
            }
            Content::MathOverparen(e) => {
                e.body.visit_content_static(visit);
                if let Some(c) = &e.annotation {
                    c.visit_content_static(visit);
                }
            }
            Content::MathOvershell(e) => {
                e.body.visit_content_static(visit);
                if let Some(c) = &e.annotation {
                    c.visit_content_static(visit);
                }
            }
            Content::MathMatrix(e) => {
                for row in &e.rows {
                    for c in row {
                        c.visit_content_static(visit);
                    }
                }
            }
            Content::MathCases(e) => {
                for c in &e.children {
                    c.visit_content_static(visit);
                }
            }
            Content::MathVec(e) => {
                for c in &e.children {
                    c.visit_content_static(visit);
                }
            }
            Content::MathAccent(e) => e.base.visit_content_static(visit),
            Content::MathUnderover(e) => {
                e.base.visit_content_static(visit);
                for c in [&e.under, &e.over].into_iter().flatten() {
                    c.visit_content_static(visit);
                }
            }
            Content::MathOp(e) => e.text.visit_content_static(visit),
            Content::Terms(e) => {
                for c in &e.items {
                    c.visit_content_static(visit);
                }
            }
            Content::TermItem(e) => {
                e.term.visit_content_static(visit);
                e.description.visit_content_static(visit);
            }
            Content::Quote(e) => {
                e.body.visit_content_static(visit);
                if let Some(c) = &e.attribution {
                    c.visit_content_static(visit);
                }
            }
            Content::Stack(e) => {
                for c in e.children.iter() {
                    c.visit_content_static(visit);
                }
            }
            Content::Outline(e) => {
                if let Some(c) = &e.title {
                    c.visit_content_static(visit);
                }
                if let Some(c) = e.realized_body() {
                    c.visit_content_static(visit);
                }
            }
            Content::OutlineEntry(e) => {
                e.element.visit_content_static(visit);
                if let Some(c) = &e.fill {
                    c.visit_content_static(visit);
                }
                if let Some(c) = e.prefix_ref() {
                    c.visit_content_static(visit);
                }
            }
            Content::Grid(e) => {
                for c in &e.cells {
                    c.visit_content_static(visit);
                }
                for c in [&e.header, &e.footer].into_iter().flatten() {
                    c.visit_content_static(visit);
                }
            }
            Content::Table(e) => {
                if let Some(plan) = &e.resolved {
                    for c in &plan.cells {
                        c.visit_content_static(visit);
                    }
                } else {
                    for c in &e.children {
                        c.visit_content_static(visit);
                    }
                    for c in [&e.header, &e.footer].into_iter().flatten() {
                        c.visit_content_static(visit);
                    }
                }
                if let Some(c) = &e.caption {
                    c.visit_content_static(visit);
                }
            }
            Content::TableCell(e) => e.visual().visit_content_static(visit),
            Content::Bibliography(e) => {
                if let Some(c) = &e.title {
                    c.visit_content_static(visit);
                }
            }
            Content::Cite(e) => {
                if let Some(c) = &e.supplement {
                    c.visit_content_static(visit);
                }
            }
            Content::FootnoteEntry(e) => {
                e.note.visit_content_static(visit);
                if let Some(c) = &e.separator {
                    c.visit_content_static(visit);
                }
            }
            Content::Document { body, title, .. } => {
                body.visit_content_static(visit);
                if let Some(c) = title {
                    c.visit_content_static(visit);
                }
            }
            Content::Empty
            | Content::Text(..)
            | Content::Space
            | Content::SpaceFromNewline
            | Content::Parbreak
            | Content::Raw(..)
            | Content::MathIdent(..)
            | Content::MathText(..)
            | Content::MathPrimes(..)
            | Content::MathAlignPoint(..)
            | Content::Linebreak(..)
            | Content::Ref(..)
            | Content::CounterDisplay(..)
            | Content::CounterUpdate(..)
            | Content::Image(..)
            | Content::Shape(..)
            | Content::Curve(..)
            | Content::SetPage(..)
            | Content::Flush(..)
            | Content::Divider(..)
            | Content::Asset { .. }
            | Content::SmartQuote(..)
            | Content::PdfAttach(..)
            | Content::HSpace(..)
            | Content::VSpace(..)
            | Content::Pagebreak(..)
            | Content::Colbreak(..)
            | Content::GridHLine(..)
            | Content::GridVLine(..)
            | Content::TableHLine(..)
            | Content::TableVLine(..)
            | Content::Metadata(..)
            | Content::State(..)
            | Content::StateUpdate(..)
            | Content::StateDisplay(..)
            | Content::CounterDisplayCallback(..)
            | Content::ContextBlock(..)
            | Content::Dynamic(..) => {}
        }
    }

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
        Ok(self
            .map_content_inner(transform, true)?
            .unwrap_or_else(|| self.clone()))
    }

    /// None proves that neither descendants nor the root were replaced.
    pub(crate) fn map_content_if_changed<F>(
        &self,
        transform: &mut F,
    ) -> SourceResult<Option<Self>>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        self.map_content_inner(transform, true)
    }

    /// Allocate only after the first changed child; callbacks run once in order.
    pub(crate) fn map_children_if_changed<F>(
        parts: &[Content],
        transform: &mut F,
    ) -> SourceResult<Option<Vec<Content>>>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        let mut output: Option<Vec<Content>> = None;
        for (index, child) in parts.iter().enumerate() {
            match child.map_content_if_changed(transform)? {
                Some(replacement) => {
                    let mapped = output.get_or_insert_with(|| {
                        let mut mapped = Vec::with_capacity(parts.len());
                        mapped.extend_from_slice(&parts[..index]);
                        mapped
                    });
                    mapped.push(replacement);
                }
                None => {
                    if let Some(mapped) = &mut output {
                        mapped.push(child.clone());
                    }
                }
            }
        }
        Ok(output)
    }

    fn map_content_inner<F>(
        &self,
        transform: &mut F,
        transform_root: bool,
    ) -> crate::entities::source_result::SourceResult<Option<Self>>
    where
        F: FnMut(
            &Content,
        )
            -> crate::entities::source_result::SourceResult<Option<Content>>,
    {
        // Passo 1: processar os filhos (bottom-up) para obter o nó com filhos transformados.
        if let Content::Source(e) = self {
            let body = e.body.map_content_inner(transform, transform_root)?;
            return Ok(match body {
                Some(body) => Some(body.with_source_span(e.span)),
                None => {
                    if !e.span.is_detached() && e.body.source_span().is_detached() {
                        None
                    } else {
                        Some(e.body.clone().with_source_span(e.span))
                    }
                }
            });
        }
        #[inline(never)]
        fn rebuild_bottom_up_node(
            action: impl FnOnce() -> crate::entities::source_result::SourceResult<
                Option<Content>,
            >,
        ) -> crate::entities::source_result::SourceResult<Option<Content>> {
            action()
        }
        let processed = match self {
            Content::Source(_) => unreachable!("source dispatched above"),
            Content::ShowRecipe(e) => rebuild_bottom_up_node(|| Ok(e.body.map_content_if_changed(transform)?.map(|body|
                Content::ShowRecipe(Arc::new(ShowRecipeContent { body, ..e.as_ref().clone() }))))),
            Content::Located(e) => rebuild_bottom_up_node(|| Ok(e.body.map_content_inner(transform, false)?.map(|body|
                Content::Located(Arc::new(LocatedContent { body, location: e.location }))))),
            Content::Realized(e) => rebuild_bottom_up_node(|| Ok(e.visual.map_content_if_changed(transform)?.map(|visual|
                Content::Realized(Arc::new(RealizedContent { original: e.original.clone(), visual }))))),
            // ── Containers: propagar recursivamente ─────────────────────────
            Content::Sequence(seq) => Ok(Self::map_children_if_changed(seq, transform)?
                .map(|parts| Content::Sequence(Arc::from(parts)))),
            Content::HtmlElem(e) => rebuild_bottom_up_node(|| Ok(Some(Content::HtmlElem(Arc::new(
                crate::entities::html::HtmlElem::new(
                    e.tag.clone(),
                    e.attrs.clone(),
                    match &e.body {
                        crate::entities::html::HtmlBody::Unset => crate::entities::html::HtmlBody::Unset,
                        crate::entities::html::HtmlBody::None => crate::entities::html::HtmlBody::None,
                        crate::entities::html::HtmlBody::Content(body) => crate::entities::html::HtmlBody::Content(Box::new(body.map_content(transform)?)),
                    },
                ),
            ))))),
            Content::HtmlFrame(e) => rebuild_bottom_up_node(|| Ok(Some(Content::HtmlFrame(Arc::new(
                crate::entities::html::HtmlFrameElem {
                    body: e.body.map_content(transform)?,
                },
            ))))),
            // P863: parágrafo é container transparente.
            Content::Par { body } => rebuild_bottom_up_node(|| Ok(body.map_content_if_changed(transform)?
                .map(|body| Content::Par { body: Box::new(body) }))),
            Content::PageRun(e) => rebuild_bottom_up_node(|| Ok(Some(e.map_content(transform)?))),
            // Strong e Emph conservam suas variantes semânticas próprias.
            // A prova de ausência de substituição vem dos owners abaixo.
            // Modelo D (P316): Heading delega ao elemento.
            Content::Heading(h) => rebuild_bottom_up_node(|| Ok(Some(h.map_content(transform)?))),
            Content::Title(t) => rebuild_bottom_up_node(|| Ok(Some(t.map_content(transform)?))),
            Content::Strong(e) => rebuild_bottom_up_node(|| e.map_content_if_changed(transform)),
            Content::Emph(e)   => rebuild_bottom_up_node(|| e.map_content_if_changed(transform)),
            // Modelo D (Lote 3 P318): família lista/termos delega ao elemento.
            Content::ListItem(e) => rebuild_bottom_up_node(|| Ok(Some(e.map_content(transform)?))),
            Content::EnumItem(e) => rebuild_bottom_up_node(|| Ok(Some(e.map_content(transform)?))),
            Content::Link(e)     => rebuild_bottom_up_node(|| Ok(Some(e.map_content(transform)?))),
            Content::Label(e)    => rebuild_bottom_up_node(|| Ok(Some(e.map_content(transform)?))),
            // Modelo D (Lote 13 P328): Figure container delega ao elemento.
            Content::Figure(e) => rebuild_bottom_up_node(|| Ok(Some(e.map_content(transform)?))),
            // P1473 — FigureCaption delega ao elemento.
            Content::FigureCaption(e) => rebuild_bottom_up_node(|| Ok(Some(e.map_content(transform)?))),
            // Equation é um contêiner com body Content.
            // Modelo D (Lote 10 P325): Equation container delega ao elemento.
            Content::Equation(e) => rebuild_bottom_up_node(|| e.map_content_if_changed(transform)),
            Content::MathSequence(seq) => Ok(Self::map_children_if_changed(seq, transform)?
                .map(|parts| Content::MathSequence(Arc::from(parts)))),
            // Modelo D (Lote 2 P317): família math delega ao elemento
            // (cada owner recursa e só materializa após substituição).
            Content::MathFrac(e)      => rebuild_bottom_up_node(|| e.map_content_if_changed(transform)),
            Content::MathAttach(e)    => rebuild_bottom_up_node(|| e.map_content_if_changed(transform)),
            Content::MathPrimes(e)    => rebuild_bottom_up_node(|| e.map_content_if_changed(transform)),
            Content::MathRoot(e)      => rebuild_bottom_up_node(|| e.map_content_if_changed(transform)),
            Content::MathStretch(e)   => rebuild_bottom_up_node(|| e.map_content_if_changed(transform)),
            Content::MathDelimited(e) => rebuild_bottom_up_node(|| e.map_content_if_changed(transform)),
            Content::MathLr(e) => rebuild_bottom_up_node(|| e.map_content_if_changed(transform)),
            Content::MathMid(e) => rebuild_bottom_up_node(|| e.map_content_if_changed(transform)),
            Content::MathOverbrace(e) => rebuild_bottom_up_node(|| e.map_content_if_changed(transform)),
            Content::MathUnderbrace(e) => rebuild_bottom_up_node(|| e.map_content_if_changed(transform)),
            Content::MathUnderbracket(e) => rebuild_bottom_up_node(|| e.map_content_if_changed(transform)),
            Content::MathUnderparen(e) => rebuild_bottom_up_node(|| e.map_content_if_changed(transform)),
            Content::MathUndershell(e) => rebuild_bottom_up_node(|| e.map_content_if_changed(transform)),
            Content::MathOverbracket(e) => rebuild_bottom_up_node(|| e.map_content_if_changed(transform)),
            Content::MathOverparen(e) => rebuild_bottom_up_node(|| e.map_content_if_changed(transform)),
            Content::MathOvershell(e) => rebuild_bottom_up_node(|| e.map_content_if_changed(transform)),
            Content::MathMatrix(e)    => rebuild_bottom_up_node(|| e.map_content_if_changed(transform)),
            Content::MathCases(e)     => rebuild_bottom_up_node(|| e.map_content_if_changed(transform)),
            Content::MathAccent(e)    => rebuild_bottom_up_node(|| e.map_content_if_changed(transform)),
            Content::MathCancel(e)    => rebuild_bottom_up_node(|| e.map_content_if_changed(transform)),
            Content::MathUnderline(e) => rebuild_bottom_up_node(|| e.map_content_if_changed(transform)),
            Content::MathOverline(e) => rebuild_bottom_up_node(|| e.map_content_if_changed(transform)),
            Content::MathVec(e) => rebuild_bottom_up_node(|| e.map_content_if_changed(transform)),
            Content::MathClassOverride(e) => rebuild_bottom_up_node(|| e.map_content_if_changed(transform)),
            Content::MathLimitsOverride(e) => rebuild_bottom_up_node(|| e.map_content_if_changed(transform)),
            Content::MathUnderover(e) => rebuild_bottom_up_node(|| e.map_content_if_changed(transform)),
            Content::MathOp(e)        => rebuild_bottom_up_node(|| e.map_content_if_changed(transform)),
            Content::MathAlignPoint(e) => rebuild_bottom_up_node(|| e.map_content_if_changed(transform)),
            // P311b.2 — MathStyled map_content recursivo (Modelo D P316: delega).
            Content::MathStyled(m) => rebuild_bottom_up_node(|| m.map_content_if_changed(transform)),

            // Modelo D (Lote 3 P318): Terms/TermItem delegam ao elemento.
            Content::Terms(e)    => rebuild_bottom_up_node(|| Ok(Some(e.map_content(transform)?))),
            Content::TermItem(e) => rebuild_bottom_up_node(|| Ok(Some(e.map_content(transform)?))),

            // Modelo D (Lote 8 P323): Quote container delega ao elemento.
            Content::Quote(e) => rebuild_bottom_up_node(|| Ok(Some(e.map_content(transform)?))),
            Content::PdfArtifact(e) => rebuild_bottom_up_node(|| Ok(Some(e.map_content(transform)?))),

            // P284 — text decoration containers — recurse em body;
            // atributos cosméticos são Copy primitivos.
            // Modelo D (Lote 4 P319): decorações delegam ao elemento.
            Content::Underline(e) => rebuild_bottom_up_node(|| Ok(Some(e.map_content(transform)?))),
            Content::Strike(e)    => rebuild_bottom_up_node(|| Ok(Some(e.map_content(transform)?))),
            Content::Overline(e)  => rebuild_bottom_up_node(|| Ok(Some(e.map_content(transform)?))),

            // P408: smallcaps container — recurse em body (stub transparente).
            Content::SmallCaps { body } => rebuild_bottom_up_node(|| Ok(body.map_content_if_changed(transform)?
                .map(|body| Content::SmallCaps { body: Box::new(body) }))),

            // Passo 156C / 156L: Pad / Hide containers — recurse em body;
            // sides é Copy primitivo (Sides<Option<Length>>).
            // Modelo D (Lote 10 P325): Pad container delega ao elemento.
            Content::Pad(e) => rebuild_bottom_up_node(|| Ok(Some(e.map_content(transform)?))),
            // Modelo D (Lote 7 P322): Hide delega ao elemento.
            Content::Hide(e) => rebuild_bottom_up_node(|| Ok(Some(e.map_content(transform)?))),

            // Passo 156G + P231 + P247 + P250: Block container — recurse em
            // body; preserva 9 cosméticos (Sides/Corners/bool/Color Copy;
            // Stroke Clone; Option<Length> Copy; bool Copy).
            Content::Block(e) => rebuild_bottom_up_node(|| e.map_content_if_changed(transform)),

            // Passo 156H + P231 + P247: Boxed (Box inline) — recurse análogo a Block; preserva 5 cosméticos.
            Content::Boxed(e) => rebuild_bottom_up_node(|| e.map_content_if_changed(transform)),

            // Modelo D (Lote 9 P324): Stack container delega ao elemento.
            Content::Stack(e) => rebuild_bottom_up_node(|| Ok(Some(e.map_content(transform)?))),

            // Passo 156J: Repeat container — recurse em body; gap e
            // justify são Copy primitivos (Option<Length>, bool).
            Content::Repeat(e) => rebuild_bottom_up_node(|| Ok(Some(e.map_content(transform)?))),

            // Modelo D (Lote 8 P323): Columns container delega ao elemento.
            Content::Columns(e) => rebuild_bottom_up_node(|| Ok(Some(e.map_content(transform)?))),
            Content::Outline(e) => rebuild_bottom_up_node(|| Ok(Some(e.map_content(transform)?))),
            Content::OutlineEntry(e) => rebuild_bottom_up_node(|| Ok(Some(e.map_content(transform)?))),

            // ── Terminais: emprestar até o callback pedir substituição ───────
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
            | Content::CounterDisplayCallback(_) => Ok(None),
            // Modelo D (Lote 9 P324): Transform container delega ao elemento.
            Content::Transform(e) => rebuild_bottom_up_node(|| Ok(Some(e.map_content(transform)?))),
            // P224+P227+P228 — Grid refino +7 fields (gutter/align/inset/header/footer/stroke/fill).
            // Modelo D (Lote 12 P327): Grid container delega ao elemento.
            Content::Grid(e) => rebuild_bottom_up_node(|| Ok(Some(e.map_content(transform)?))),
            // Modelo D (Lote 5 P320): GridHeader/GridFooter delegam ao elemento.
            Content::GridHeader(e) => rebuild_bottom_up_node(|| Ok(Some(e.map_content(transform)?))),
            Content::GridFooter(e) => rebuild_bottom_up_node(|| Ok(Some(e.map_content(transform)?))),
            // Modelo D (Lote 12 P327): GridCell container delega ao elemento.
            Content::GridCell(e) => rebuild_bottom_up_node(|| Ok(Some(e.map_content(transform)?))),
            // Passo 512 — linhas em grid/table são terminais.
            Content::GridHLine(_) | Content::GridVLine(_) => Ok(None),
            Content::TableHLine(_) | Content::TableVLine(_) => Ok(None),
            // Passo 157A + P227 + P228: Table — mapear children; preservar stroke + fill.
            // Modelo D (Lote 12 P327): Table container delega ao elemento.
            Content::Table(e) => rebuild_bottom_up_node(|| e.map_content_if_changed(transform)),
            // Modelo D (Lote 12 P327): TableCell container delega ao elemento.
            Content::TableCell(e) => rebuild_bottom_up_node(|| e.map_content_if_changed(transform)),
            // Modelo D (Lote 5 P320): TableHeader/TableFooter delegam ao elemento.
            Content::TableHeader(e) => rebuild_bottom_up_node(|| Ok(Some(e.map_content(transform)?))),
            Content::TableFooter(e) => rebuild_bottom_up_node(|| Ok(Some(e.map_content(transform)?))),
            // Passo 159A: Bibliography recurse em title; preserva
            // entries (BibEntry é dados puros, sem Content recursivo).
            // Cite recurse em supplement; preserva key.
            // Modelo D (Lote 10 P325): Bibliography delega ao elemento (recurse title).
            Content::Bibliography(e) => rebuild_bottom_up_node(|| Ok(Some(e.map_content(transform)?))),
            // Modelo D (Lote 9 P324): Cite delega ao elemento (recurse supplement).
            Content::Cite(e) => rebuild_bottom_up_node(|| Ok(Some(e.map_content(transform)?))),
            // Modelo D (Lote 11 P326): Footnote container delega ao elemento.
            Content::Footnote(e) => rebuild_bottom_up_node(|| Ok(Some(e.map_content(transform)?))),
            // P1473 — FootnoteEntry delega ao elemento.
            Content::FootnoteEntry(e) => rebuild_bottom_up_node(|| Ok(Some(e.map_content(transform)?))),
            Content::Align(e) => rebuild_bottom_up_node(|| Ok(Some(e.map_content(transform)?))),
            // Modelo D (Lote 9 P324): Place container delega ao elemento.
            Content::Place(e) => rebuild_bottom_up_node(|| Ok(Some(e.map_content(transform)?))),
            Content::Flush(e) => rebuild_bottom_up_node(|| Ok(Some(e.map_content(transform)?))),
            Content::Styled(styled) => rebuild_bottom_up_node(|| Ok(styled.body.map_content_if_changed(transform)?
                .map(|body| Content::styled(body, styled.styles.clone())))),
            // P397: Document recursa no título (único Content aninhado);
            // Asset é terminal (sem Content aninhado).
            Content::Document { path, body, format, title, author, date, keywords } => rebuild_bottom_up_node(|| Ok(Some(Content::Document {
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
            }))),
            Content::Asset { .. } => Ok(None),
            // P506: ContextBlock é terminal em map_content (o corpo é closure,
            // não content; expansão acontece pós-introspecção).
            Content::ContextBlock(_) => Ok(None),
            // Lote F-1 (P334): a fronteira dinâmica recursa nos filhos via o
            // elemento (mesmo contrato dos 65: devolve o nó com filhos
            // transformados; o hub aplica `transform` ao nó abaixo).
            Content::Dynamic(e) => rebuild_bottom_up_node(|| Ok(Some(e.dyn_map_content(transform)?))),
        }?;

        if !transform_root {
            return Ok(processed);
        }
        // Passo 2: aplicar a transformação ao nó já processado.
        match transform(processed.as_ref().unwrap_or(self))? {
            Some(new_content) => Ok(Some(new_content)),
            None => Ok(processed),
        }
    }

    /// Private pre-order traversal. Some replaces an entire subtree.
    pub(crate) fn map_content_pre<F>(
        &self,
        transform: &mut F,
    ) -> crate::entities::source_result::SourceResult<Self>
    where
        F: FnMut(
            &Content,
        )
            -> crate::entities::source_result::SourceResult<Option<Content>>,
    {
        if let Content::Source(e) = self {
            return Ok(e.body.map_content_pre(transform)?.with_source_span(e.span));
        }
        if let Some(replacement) = transform(self)? {
            return Ok(replacement);
        }
        fn map_slot<F>(
            slot: &crate::entities::elements::math_attach::MathAttachSlot,
            transform: &mut F,
        ) -> crate::entities::source_result::SourceResult<
            crate::entities::elements::math_attach::MathAttachSlot,
        >
        where
            F: FnMut(
                &Content,
            )
                -> crate::entities::source_result::SourceResult<Option<Content>>,
        {
            use crate::entities::elements::math_attach::MathAttachSlot;
            Ok(match slot {
                MathAttachSlot::Omitted => MathAttachSlot::Omitted,
                MathAttachSlot::ExplicitNone => MathAttachSlot::ExplicitNone,
                MathAttachSlot::Present(c) => {
                    MathAttachSlot::Present(c.map_content_pre(transform)?)
                }
            })
        }
        if let Content::Source(e) = self {
            return Ok(e.body.map_content_pre(transform)?.with_source_span(e.span));
        }
        // As reconstruções ficam em frames separados do dispatcher recursivo.
        #[inline(never)]
        fn rebuild_pre_node(
            action: impl FnOnce() -> crate::entities::source_result::SourceResult<Content>,
        ) -> crate::entities::source_result::SourceResult<Content> {
            action()
        }
        match self {
            Content::Source(_) => unreachable!("source dispatched above"),
            Content::ShowRecipe(e) => rebuild_pre_node(|| Ok(Content::ShowRecipe(Arc::new(ShowRecipeContent {
                body: e.body.map_content_pre(transform)?, ..e.as_ref().clone()
            })))),
            Content::Located(e) => rebuild_pre_node(|| Ok(Content::Located(Arc::new(LocatedContent {
                body: e.body.map_content_pre(transform)?, location: e.location,
            })))),
            Content::Realized(e) => rebuild_pre_node(|| Ok(Content::Realized(Arc::new(RealizedContent {
                original: e.original.clone(), visual: e.visual.map_content_pre(transform)?,
            })))),
            // ── Containers: propagar recursivamente ─────────────────────────
            Content::Sequence(seq) => rebuild_pre_node(|| Ok({
                let new_seq: crate::entities::source_result::SourceResult<Vec<Content>> =
                    seq.iter().map(|c| c.map_content_pre(transform)).collect();
                Content::Sequence(Arc::from(new_seq?))
            })),
            Content::HtmlElem(e) => rebuild_pre_node(|| Ok(Content::HtmlElem(Arc::new(
                crate::entities::html::HtmlElem::new(
                    e.tag.clone(),
                    e.attrs.clone(),
                    match &e.body {
                        crate::entities::html::HtmlBody::Unset => crate::entities::html::HtmlBody::Unset,
                        crate::entities::html::HtmlBody::None => crate::entities::html::HtmlBody::None,
                        crate::entities::html::HtmlBody::Content(body) => crate::entities::html::HtmlBody::Content(Box::new(body.map_content_pre(transform)?)),
                    },
                ),
            )))),
            Content::HtmlFrame(e) => rebuild_pre_node(|| Ok(Content::HtmlFrame(Arc::new(
                crate::entities::html::HtmlFrameElem {
                    body: e.body.map_content_pre(transform)?,
                },
            )))),
            // P863: parágrafo é container transparente.
            Content::Par { body } => rebuild_pre_node(|| Ok(Content::Par {
                body: Box::new(body.map_content_pre(transform)?),
            })),
            Content::PageRun(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::PageRun(Arc::new(PageRunElem {
            paper: e.paper,
            flipped: e.flipped,
            binding: e.binding,
            width: e.width,
            height: e.height,
            margin: e.margin,
            numbering: e.numbering.clone(),
            number_align: e.number_align,
            header: e
                .header
                .as_ref()
                .map(|value| match value {
                    crate::entities::page_running::PageMarginal::Content(content) => {
                        content
                            .map_content_pre(transform)
                            .map(Arc::new)
                            .map(crate::entities::page_running::PageMarginal::Content)
                    }
                    other => Ok(other.clone()),
                })
                .transpose()?,
            header_ascent: e.header_ascent,
            footer: e
                .footer
                .as_ref()
                .map(|value| match value {
                    crate::entities::page_running::PageMarginal::Content(content) => {
                        content
                            .map_content_pre(transform)
                            .map(Arc::new)
                            .map(crate::entities::page_running::PageMarginal::Content)
                    }
                    other => Ok(other.clone()),
                })
                .transpose()?,
            footer_descent: e.footer_descent,
            supplement: e
                .supplement
                .as_ref()
                .map(|value| match value {
                    crate::entities::page_supplement::PageSupplement::Content(
                        content,
                    ) => content
                        .map_content_pre(transform)
                        .map(Arc::new)
                        .map(crate::entities::page_supplement::PageSupplement::Content),
                    other => Ok(other.clone()),
                })
                .transpose()?,
            columns: e.columns,
            bleed: e.bleed,
            fill: e.fill.clone(),
            background: e
                .background
                .as_ref()
                .map(|layer| {
                    layer
                        .as_ref()
                        .map(|content| content.map_content_pre(transform))
                        .transpose()
                })
                .transpose()?,
            foreground: e
                .foreground
                .as_ref()
                .map(|layer| {
                    layer
                        .as_ref()
                        .map(|content| content.map_content_pre(transform))
                        .transpose()
                })
                .transpose()?,
            body: e.body.map_content_pre(transform)?,
        })))),
            // Passo 101: Content::Strong/Emph removidos — cobertos pelo
            // arm Content::Styled abaixo (que já propaga transform recursivamente).
            // Modelo D (P316): Heading delega ao elemento.
            Content::Heading(h) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::Heading(Arc::new(h.with_body(h.body.map_content_pre(transform)?))))),
            Content::Title(t) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::Title(Arc::new(TitleElem {
            body: t.body.map_content_pre(transform)?,
        })))),
            Content::Strong(e) => rebuild_pre_node(|| Ok({ let mut mapped = e.as_ref().clone(); mapped.body = mapped.body.map_content_pre(transform)?; Content::Strong(Arc::new(mapped)) })),
            Content::Emph(e) => rebuild_pre_node(|| Ok({ let mut mapped = e.as_ref().clone(); mapped.body = mapped.body.map_content_pre(transform)?; Content::Emph(Arc::new(mapped)) })),
            // Modelo D (Lote 3 P318): família lista/termos delega ao elemento.
            Content::ListItem(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::ListItem(Arc::new(ListItemElem {
            body: e.body.map_content_pre(transform)?,
            marker: e.marker.clone(),
            marker_align: e.marker_align,
            indent: e.indent,
            body_indent: e.body_indent,
            tight: e.tight,
        })))),
            Content::EnumItem(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::EnumItem(Arc::new(EnumItemElem {
            number: e.number,
            body: e.body.map_content_pre(transform)?,
            numbering: e.numbering.clone(),
            indent: e.indent,
            body_indent: e.body_indent,
            tight: e.tight,
        })))),
            Content::Link(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::Link(Arc::new(LinkElem {
            url: e.url.clone(),
            body: e.body.map_content_pre(transform)?,
        })))),
            Content::Label(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::Label(Arc::new(LabelElem {
            body: e.body.map_content_pre(transform)?,
            name: e.name.clone(),
            auto: e.auto,
        })))),
            // Modelo D (Lote 13 P328): Figure container delega ao elemento.
            Content::Figure(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::Figure(Arc::new(FigureElem {
            body: e.body.map_content_pre(transform)?,
            caption: e
                .caption
                .as_ref()
                .map(|c| c.map_content_pre(transform))
                .transpose()?,
            kind: e.kind.clone(),
        })))),
            // P1473 — FigureCaption delega ao elemento.
            Content::FigureCaption(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::FigureCaption(Arc::new(FigureCaptionElem {
            position: e.position,
            separator: e
                .separator
                .as_ref()
                .map(|s| s.map_content_pre(transform))
                .transpose()?,
            body: e.body.map_content_pre(transform)?,
        })))),
            // Content::Equation tem body: Box<Content> → container.
            // Modelo D (Lote 10 P325): Equation container delega ao elemento.
            Content::Equation(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::Equation(Arc::new(EquationElem {
            body: e.body.map_content_pre(transform)?,
            block: e.block,
        })))),
            Content::MathSequence(seq) => rebuild_pre_node(|| Ok({
                let new_seq: crate::entities::source_result::SourceResult<Vec<Content>> =
                    seq.iter().map(|c| c.map_content_pre(transform)).collect();
                Content::MathSequence(Arc::from(new_seq?))
            })),
            // Modelo D (Lote 2 P317): família math delega ao elemento
            // (cada `…Elem::map_content` recurse nos filhos e re-embrulha).
            Content::MathFrac(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::MathFrac(Arc::new(MathFracElem {
            num: e.num.map_content_pre(transform)?,
            den: e.den.map_content_pre(transform)?,
            style: e.style,
            num_deparenthesized: e.num_deparenthesized,
            denom_deparenthesized: e.denom_deparenthesized,
            line: e.line,
        })))),
            Content::MathAttach(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::MathAttach(Arc::new(MathAttachElem {
            base: e.base.map_content_pre(transform)?,
            t: map_slot(&e.t, transform)?,
            b: map_slot(&e.b, transform)?,
            tl: map_slot(&e.tl, transform)?,
            bl: map_slot(&e.bl, transform)?,
            tr: map_slot(&e.tr, transform)?,
            br: map_slot(&e.br, transform)?,
        })))),
            Content::MathPrimes(e) => Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::MathPrimes(Arc::new(e.as_ref().clone()))),
            Content::MathRoot(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::MathRoot(Arc::new(MathRootElem {
            index: e.index.as_ref().map(|c| c.map_content_pre(transform)).transpose()?,
            radicand: e.radicand.map_content_pre(transform)?,
            index_explicit: e.index_explicit,
        })))),
            Content::MathStretch(e) => rebuild_pre_node(|| Ok({ let mut mapped = e.as_ref().clone(); mapped.body = mapped.body.map_content_pre(transform)?; Content::MathStretch(Arc::new(mapped)) })),
            Content::MathDelimited(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::MathDelimited(Arc::new(MathDelimitedElem {
            open: e.open,
            body: e.body.map_content_pre(transform)?,
            close: e.close,
        })))),
            Content::MathLr(e) => rebuild_pre_node(|| Ok({ let mut mapped = e.as_ref().clone(); mapped.body = mapped.body.map_content_pre(transform)?; Content::MathLr(Arc::new(mapped)) })),
            Content::MathMid(e) => rebuild_pre_node(|| Ok({ let mut mapped = e.as_ref().clone(); mapped.body = mapped.body.map_content_pre(transform)?; Content::MathMid(Arc::new(mapped)) })),
            Content::MathOverbrace(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::MathOverbrace(Arc::new(MathOverbraceElem {
            body: e.body.map_content_pre(transform)?,
            annotation: e
                .annotation
                .as_ref()
                .map(|content| content.map_content_pre(transform))
                .transpose()?,
            annotation_explicit: e.annotation_explicit,
        })))),
            Content::MathUnderbrace(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::MathUnderbrace(Arc::new(MathUnderbraceElem {
            body: e.body.map_content_pre(transform)?,
            annotation: e
                .annotation
                .as_ref()
                .map(|content| content.map_content_pre(transform))
                .transpose()?,
            annotation_explicit: e.annotation_explicit,
        })))),
            Content::MathUnderbracket(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::MathUnderbracket(Arc::new(MathUnderbracketElem {
            body: e.body.map_content_pre(transform)?,
            annotation: e
                .annotation
                .as_ref()
                .map(|content| content.map_content_pre(transform))
                .transpose()?,
            annotation_explicit: e.annotation_explicit,
        })))),
            Content::MathUnderparen(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::MathUnderparen(Arc::new(MathUnderparenElem {
            body: e.body.map_content_pre(transform)?,
            annotation: e
                .annotation
                .as_ref()
                .map(|content| content.map_content_pre(transform))
                .transpose()?,
            annotation_explicit: e.annotation_explicit,
        })))),
            Content::MathUndershell(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::MathUndershell(Arc::new(MathUndershellElem {
            body: e.body.map_content_pre(transform)?,
            annotation: e
                .annotation
                .as_ref()
                .map(|content| content.map_content_pre(transform))
                .transpose()?,
            annotation_explicit: e.annotation_explicit,
        })))),
            Content::MathOverbracket(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::MathOverbracket(Arc::new(MathOverbracketElem {
            body: e.body.map_content_pre(transform)?,
            annotation: e
                .annotation
                .as_ref()
                .map(|content| content.map_content_pre(transform))
                .transpose()?,
            annotation_explicit: e.annotation_explicit,
        })))),
            Content::MathOverparen(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::MathOverparen(Arc::new(MathOverparenElem {
            body: e.body.map_content_pre(transform)?,
            annotation: e
                .annotation
                .as_ref()
                .map(|content| content.map_content_pre(transform))
                .transpose()?,
            annotation_explicit: e.annotation_explicit,
        })))),
            Content::MathOvershell(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::MathOvershell(Arc::new(MathOvershellElem {
            body: e.body.map_content_pre(transform)?,
            annotation: e
                .annotation
                .as_ref()
                .map(|content| content.map_content_pre(transform))
                .transpose()?,
            annotation_explicit: e.annotation_explicit,
        })))),
            Content::MathMatrix(e) => rebuild_pre_node(|| Ok({ let rows = e
            .rows
            .iter()
            .map(|row| row.iter().map(|cell| cell.map_content_pre(transform)).collect())
            .collect::<SourceResult<Vec<Vec<_>>>>()?;
        Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::MathMatrix(Arc::new(MathMatrixElem {
            rows,
            delim: e.delim,
            align: e.align,
            augment: e.augment.clone(),
            row_gap: e.row_gap,
            column_gap: e.column_gap,
            explicit: e.explicit,
        })))? })),
            Content::MathCases(e) => rebuild_pre_node(|| Ok({ let children = e
            .children
            .iter()
            .map(|child| child.map_content_pre(transform))
            .collect::<SourceResult<Vec<_>>>()?;
        Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::MathCases(Arc::new(MathCasesElem {
            children,
            delim: e.delim,
            reverse: e.reverse,
            gap: e.gap,
            explicit: e.explicit,
        })))? })),
            Content::MathAccent(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::MathAccent(Arc::new(MathAccentElem {
            base: e.base.map_content_pre(transform)?,
            accent: e.accent,
            size: e.size,
            dotless: e.dotless,
            explicit: e.explicit,
        })))),
            Content::MathCancel(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::MathCancel(Arc::new(MathCancelElem {
            body: e.body.map_content_pre(transform)?,
            length: e.length,
            inverted: e.inverted,
            cross: e.cross,
            angle: e.angle.clone(),
            stroke: e.stroke.clone(),
            background: e.background,
            span: e.span,
            explicit: e.explicit,
        })))),
            Content::MathUnderline(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::MathUnderline(Arc::new(MathUnderlineElem {
            body: e.body.map_content_pre(transform)?,
        })))),
            Content::MathOverline(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::MathOverline(Arc::new(MathOverlineElem {
            body: e.body.map_content_pre(transform)?,
        })))),
            Content::MathVec(e) => rebuild_pre_node(|| Ok({ let children = e
            .children
            .iter()
            .map(|child| child.map_content_pre(transform))
            .collect::<SourceResult<Vec<_>>>()?;
        Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::MathVec(Arc::new(MathVecElem {
            children,
            delim: e.delim,
            align: e.align,
            gap: e.gap,
            explicit: e.explicit,
        })))? })),
            Content::MathClassOverride(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::MathClassOverride(Arc::new(MathClassOverrideElem {
            class: e.class,
            body: e.body.map_content_pre(transform)?,
        })))),
            Content::MathLimitsOverride(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::MathLimitsOverride(Arc::new(MathLimitsOverrideElem {
            body: e.body.map_content_pre(transform)?,
            limits: e.limits,
            inline: e.inline,
            inline_explicit: e.inline_explicit,
        })))),
            Content::MathUnderover(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::MathUnderover(Arc::new(MathUnderoverElem {
            base: e.base.map_content_pre(transform)?,
            under: e.under.as_ref().map(|c| c.map_content_pre(transform)).transpose()?,
            over: e.over.as_ref().map(|c| c.map_content_pre(transform)).transpose()?,
        })))),
            Content::MathOp(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::MathOp(Arc::new(MathOpElem {
            text: e.text.map_content_pre(transform)?,
            limits: e.limits,
        })))),
            Content::MathAlignPoint(e) => Ok({ // Terminal (marcador unit; sem filhos).
        Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::MathAlignPoint(Arc::new(e.as_ref().clone())))? }),
            // P311b.2 — MathStyled map_content recursivo (Modelo D P316: delega).
            Content::MathStyled(m) => rebuild_pre_node(|| Ok({ // Recursivo no body; preserva kind/flags (content.rs:2145).
        Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::MathStyled(Arc::new(MathStyledElem {
            kind: m.kind,
            bold: m.bold,
            italic: m.italic,
            body: m.body.map_content_pre(transform)?,
            cramped: m.cramped,
        })))? })),

            // Modelo D (Lote 3 P318): Terms/TermItem delegam ao elemento.
            Content::Terms(e) => rebuild_pre_node(|| Ok({ let items: SourceResult<Vec<Content>> =
            e.items.iter().map(|c| c.map_content_pre(transform)).collect();
        Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::Terms(Arc::new(TermsElem { items: items? })))? })),
            Content::TermItem(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::TermItem(Arc::new(TermItemElem {
            term: e.term.map_content_pre(transform)?,
            description: e.description.map_content_pre(transform)?,
        })))),

            // Modelo D (Lote 8 P323): Quote container delega ao elemento.
            Content::Quote(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::Quote(Arc::new(QuoteElem {
            body: e.body.map_content_pre(transform)?,
            attribution: e
                .attribution
                .as_ref()
                .map(|c| c.map_content_pre(transform))
                .transpose()?,
            block: e.block,
            quotes: e.quotes,
        })))),
            Content::PdfArtifact(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::PdfArtifact(Arc::new(PdfArtifactElem {
            kind: e.kind,
            body: e.body.map_content_pre(transform)?,
        })))),

            // P284 — text decoration containers — recurse em body;
            // atributos cosméticos são Copy primitivos.
            // Modelo D (Lote 4 P319): decorações delegam ao elemento.
            Content::Underline(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::Underline(Arc::new(UnderlineElem {
            body: e.body.map_content_pre(transform)?,
            stroke: e.stroke.clone(),
            offset: e.offset,
            extent: e.extent,
        })))),
            Content::Strike(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::Strike(Arc::new(StrikeElem {
            body: e.body.map_content_pre(transform)?,
            stroke: e.stroke.clone(),
            offset: e.offset,
            extent: e.extent,
        })))),
            Content::Overline(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::Overline(Arc::new(OverlineElem {
            body: e.body.map_content_pre(transform)?,
            stroke: e.stroke.clone(),
            offset: e.offset,
            extent: e.extent,
        })))),

            // P408: smallcaps container — recurse em body (stub transparente).
            Content::SmallCaps { body } => rebuild_pre_node(|| Ok(Content::SmallCaps {
                body: Box::new(body.map_content_pre(transform)?),
            })),

            // Passo 156C / 156L: Pad / Hide containers — recurse em body;
            // sides é Copy primitivo (Sides<Option<Length>>).
            // Modelo D (Lote 10 P325): Pad container delega ao elemento.
            Content::Pad(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::Pad(Arc::new(PadElem {
            body: e.body.map_content_pre(transform)?,
            sides: e.sides,
        })))),
            // Modelo D (Lote 7 P322): Hide delega ao elemento.
            Content::Hide(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::Hide(Arc::new(HideElem { body: e.body.map_content_pre(transform)? })))),

            // Passo 156G + P231 + P247 + P250: Block container — recurse em
            // body; preserva 9 cosméticos (Sides/Corners/bool/Color Copy;
            // Stroke Clone; Option<Length> Copy; bool Copy).
            Content::Block(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::Block(Arc::new(BlockElem {
            body: e.body.map_content_pre(transform)?,
            ..e.as_ref().clone()
        })))),

            // Passo 156H + P231 + P247: Boxed (Box inline) — recurse análogo a Block; preserva 5 cosméticos.
            Content::Boxed(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::Boxed(Arc::new(BoxedElem {
            body: e.body.map_content_pre(transform)?,
            width: e.width,
            width_fraction: e.width_fraction,
            height: e.height,
            inset: e.inset,
            baseline: e.baseline,
            outset: e.outset,
            radius: e.radius,
            clip: e.clip,
            fill: e.fill.clone(),
            stroke: e.stroke.clone(),
        })))),

            // Modelo D (Lote 9 P324): Stack container delega ao elemento.
            Content::Stack(e) => rebuild_pre_node(|| Ok({ let new_children: SourceResult<Vec<Content>> =
            e.children.iter().map(|c| c.map_content_pre(transform)).collect();
        Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::Stack(Arc::new(StackElem {
            children: Arc::from(new_children?),
            dir: e.dir,
            spacing: e.spacing,
        })))? })),

            // Passo 156J: Repeat container — recurse em body; gap e
            // justify são Copy primitivos (Option<Length>, bool).
            Content::Repeat(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::Repeat(Arc::new(RepeatElem {
            body: e.body.map_content_pre(transform)?,
            gap: e.gap,
            justify: e.justify,
        })))),

            // Modelo D (Lote 8 P323): Columns container delega ao elemento.
            Content::Columns(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::Columns(Arc::new(ColumnsElem {
            count: e.count,
            gutter: e.gutter,
            body: e.body.map_content_pre(transform)?,
            page_columns: e.page_columns,
        })))),
            Content::Outline(e) => rebuild_pre_node(|| Ok({ let mut mapped = e.as_ref().clone(); mapped.title = e.title.as_ref().map(|c| c.map_content_pre(transform)).transpose()?; Content::Outline(Arc::new(mapped)) })),
            Content::OutlineEntry(e) => rebuild_pre_node(|| Ok(Content::OutlineEntry(Arc::new(e.remapped(e.element.map_content_pre(transform)?, e.fill.as_ref().map(|c| c.map_content_pre(transform)).transpose()?, e.prefix().as_ref().map(|c| c.map_content_pre(transform)).transpose()?))))),

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
            | Content::CounterDisplayCallback(_) => Ok(self.clone()),
            // Modelo D (Lote 9 P324): Transform container delega ao elemento.
            Content::Transform(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::Transform(Arc::new(TransformElem {
            operation: e.operation,
            body: e.body.map_content_pre(transform)?,
        })))),
            // P224+P227+P228 — Grid refino +7 fields (gutter/align/inset/header/footer/stroke/fill).
            // Modelo D (Lote 12 P327): Grid container delega ao elemento.
            Content::Grid(e) => rebuild_pre_node(|| Ok({ let new_cells: SourceResult<Vec<Content>> =
            e.cells.iter().map(|c| c.map_content_pre(transform)).collect();
        Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::Grid(Arc::new(GridElem {
            columns: e.columns.clone(),
            rows: e.rows.clone(),
            cells: new_cells?,
            hlines: e.hlines.clone(),
            vlines: e.vlines.clone(),
            gutter: e.gutter,
            align: e.align,
            inset: e.inset,
            header: e.header.as_ref().map(|h| h.map_content_pre(transform)).transpose()?,
            footer: e.footer.as_ref().map(|f| f.map_content_pre(transform)).transpose()?,
            stroke: e.stroke.clone(),
            fill: e.fill.clone(),
        })))? })),
            // Modelo D (Lote 5 P320): GridHeader/GridFooter delegam ao elemento.
            Content::GridHeader(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::GridHeader(Arc::new(GridHeaderElem {
            body: e.body.map_content_pre(transform)?,
            repeat: e.repeat,
        })))),
            Content::GridFooter(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::GridFooter(Arc::new(GridFooterElem {
            body: e.body.map_content_pre(transform)?,
            repeat: e.repeat,
        })))),
            // Modelo D (Lote 12 P327): GridCell container delega ao elemento.
            Content::GridCell(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::GridCell(Arc::new(GridCellElem {
            body: e.body.map_content_pre(transform)?,
            x: e.x,
            y: e.y,
            colspan: e.colspan,
            rowspan: e.rowspan,
            stroke: e.stroke.clone(),
            fill: e.fill.clone(),
            align: e.align,
            inset: e.inset,
            breakable: e.breakable,
        })))),
            // Passo 512 — linhas em grid/table são terminais.
            Content::GridHLine(_) | Content::GridVLine(_) => Ok(self.clone()),
            Content::TableHLine(_) | Content::TableVLine(_) => Ok(self.clone()),
            // Passo 157A + P227 + P228: Table — mapear children; preservar stroke + fill.
            // Modelo D (Lote 12 P327): Table container delega ao elemento.
            Content::Table(e) if e.resolved.is_some() => rebuild_pre_node(|| Ok({
                let mut t=e.as_ref().clone();let mut r=t.resolved.as_ref().unwrap().as_ref().clone();
                r.cells=r.cells.iter().map(|c|c.map_content_pre(transform)).collect::<crate::entities::source_result::SourceResult<_>>()?;
                t.resolved=Some(Arc::new(r));t.caption=t.caption.as_ref().map(|c|c.map_content_pre(transform)).transpose()?;
                Content::Table(Arc::new(t))
            })),
            Content::TableCell(e) if e.standalone.is_some() => rebuild_pre_node(|| Ok({
                let mut c=e.as_ref().clone();let mut r=c.standalone.as_ref().unwrap().as_ref().clone();
                r.visual=r.visual.map_content_pre(transform)?;c.standalone=Some(Arc::new(r));Content::TableCell(Arc::new(c))
            })),
            Content::TableCell(e) if e.resolved.is_some() => rebuild_pre_node(|| Ok({
                let mut c=e.as_ref().clone();let mut r=c.resolved.as_ref().unwrap().as_ref().clone();
                r.visual=r.visual.map_content_pre(transform)?;c.resolved=Some(Arc::new(r));Content::TableCell(Arc::new(c))
            })),
            Content::Table(e) => rebuild_pre_node(|| Ok({ let new_children: SourceResult<Vec<Content>> =
            e.children.iter().map(|c| c.map_content_pre(transform)).collect();
        Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::Table(Arc::new(TableElem {
            source_span: e.source_span,
            resolved: e.resolved.clone(),
            rows_explicit: e.rows_explicit,
            columns: e.columns.clone(),
            rows: e.rows.clone(),
            children: new_children?,
            hlines: e.hlines.clone(),
            vlines: e.vlines.clone(),
            header: e.header.as_ref().map(|h| h.map_content_pre(transform)).transpose()?,
            footer: e.footer.as_ref().map(|f| f.map_content_pre(transform)).transpose()?,
            stroke: e.stroke.clone(),
            fill: e.fill.clone(),
            caption: e
                .caption
                .as_ref()
                .map(|c| c.map_content_pre(transform))
                .transpose()?,
            summary: e.summary.clone(),
            inset: e.inset,
            align: e.align,
        })))? })),
            // Modelo D (Lote 12 P327): TableCell container delega ao elemento.
            Content::TableCell(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::TableCell(Arc::new(TableCellElem {
            source_span: e.source_span,
            resolved: e.resolved.clone(),
            standalone: e.standalone.clone(),
            presence: e.presence,
            body: e.body.map_content_pre(transform)?,
            x: e.x,
            y: e.y,
            colspan: e.colspan,
            rowspan: e.rowspan,
            stroke: e.stroke.clone(),
            fill: e.fill.clone(),
            align: e.align,
            inset: e.inset,
            breakable: e.breakable,
            kind: e.kind,
        })))),
            // Modelo D (Lote 5 P320): TableHeader/TableFooter delegam ao elemento.
            Content::TableHeader(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::TableHeader(Arc::new(TableHeaderElem {
            body: e.body.map_content_pre(transform)?,
            repeat: e.repeat,
        })))),
            Content::TableFooter(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::TableFooter(Arc::new(TableFooterElem {
            body: e.body.map_content_pre(transform)?,
            repeat: e.repeat,
        })))),
            // Passo 159A: Bibliography recurse em title; preserva
            // entries (BibEntry é dados puros, sem Content recursivo).
            // Cite recurse em supplement; preserva key.
            // Modelo D (Lote 10 P325): Bibliography delega ao elemento (recurse title).
            Content::Bibliography(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::Bibliography(Arc::new(BibliographyElem {
            entries: e.entries.clone(),
            path: e.path.clone(),
            title: e.title.as_ref().map(|t| t.map_content_pre(transform)).transpose()?,
            style: e.style.clone(),
            locale: e.locale.clone(),
        })))),
            // Modelo D (Lote 9 P324): Cite delega ao elemento (recurse supplement).
            Content::Cite(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::Cite(Arc::new(CiteElem {
            key: e.key.clone(),
            supplement: e
                .supplement
                .as_ref()
                .map(|s| s.map_content_pre(transform))
                .transpose()?,
            form: e.form,
            style: e.style,
        })))),
            // Modelo D (Lote 11 P326): Footnote container delega ao elemento.
            Content::Footnote(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::Footnote(Arc::new(FootnoteElem {
            body: e.body.map_content_pre(transform)?,
            numbering: e.numbering.clone(),
        })))),
            // P1473 — FootnoteEntry delega ao elemento.
            Content::FootnoteEntry(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::FootnoteEntry(Arc::new(FootnoteEntryElem {
            note: e.note.map_content_pre(transform)?,
            separator: e
                .separator
                .as_ref()
                .map(|s| s.map_content_pre(transform))
                .transpose()?,
            clearance: e.clearance,
            gap: e.gap,
            indent: e.indent,
        })))),
            Content::Align(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::Align(Arc::new(AlignElem {
            alignment: e.alignment,
            body: e.body.map_content_pre(transform)?,
        })))),
            // Modelo D (Lote 9 P324): Place container delega ao elemento.
            Content::Place(e) => rebuild_pre_node(|| Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::Place(Arc::new(PlaceElem {
            alignment: e.alignment,
            dx: e.dx,
            dy: e.dy,
            scope: e.scope,
            float: e.float,
            clearance: e.clearance,
            body: e.body.map_content_pre(transform)?,
        })))),
            Content::Flush(e) => Ok::<Content, Vec<crate::entities::source_result::SourceDiagnostic>>(Content::Flush(Arc::new(e.as_ref().clone()))),
            Content::Styled(styled) => rebuild_pre_node(|| Ok(Content::styled(
                styled.body.map_content_pre(transform)?,
                styled.styles.clone(),
            ))),
            // P397: Document recursa no título (único Content aninhado);
            // Asset é terminal (sem Content aninhado).
            Content::Document { path, body, format, title, author, date, keywords } => rebuild_pre_node(|| Ok(Content::Document {
                path: path.clone(),
                body: Box::new(body.map_content_pre(transform)?),
                format: format.clone(),
                title: match title {
                    Some(body) => Some(Box::new(body.map_content_pre(transform)?)),
                    None       => None,
                },
                author: author.clone(),
                date:   *date,
                keywords: keywords.clone(),
            })),
            Content::Asset { .. } => Ok(self.clone()),
            // P506: ContextBlock é terminal em map_content (o corpo é closure,
            // não content; expansão acontece pós-introspecção).
            Content::ContextBlock(_) => Ok(self.clone()),
            // Lote F-1 (P334): a fronteira dinâmica recursa nos filhos via o
            // elemento (mesmo contrato dos 65: devolve o nó com filhos
            // transformados; o hub aplica `transform` ao nó abaixo).
            Content::Dynamic(_) => Ok(self.clone()),
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
            Content::Source(e) => e.body.map_text(transform).with_source_span(e.span),
            Content::ShowRecipe(e) => Content::ShowRecipe(Arc::new(ShowRecipeContent {
                body: e.body.map_text(transform), ..e.as_ref().clone()
            })),
            Content::Located(e) => Content::Located(Arc::new(LocatedContent {
                body: e.body.map_text(transform), location: e.location,
            })),
            Content::Realized(e) => Content::Realized(Arc::new(RealizedContent {
                original: e.original.clone(), visual: e.visual.map_text(transform),
            })),
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
        let mut uppercase = |text: &str| text.to_uppercase();
        assert_eq!(Content::text("a").map_text(&mut uppercase), Content::text("A"));
    }
}

#[cfg(test)]
mod change_aware_tests {
    use super::*;
    use crate::entities::elements::table::TableGridResolved;
    use crate::entities::location::Location;
    use crate::entities::source_result::SourceDiagnostic;
    use crate::entities::span::Span;

    fn table() -> Content {
        Content::table(
            vec![],
            vec![],
            vec![
                Content::table_cell(
                    Content::styled(
                        Content::sequence(vec![Content::text("a"), Content::text("b")]),
                        crate::entities::style::Styles::new(),
                    ),
                    None,
                    None,
                    None,
                    None,
                ),
                Content::table_cell(Content::text("c"), None, None, None, None),
            ],
        )
    }

    #[test]
    fn change_aware_no_replacement_preserves_table_arc_and_visits_once() {
        let input = table();
        let mut seen = Vec::new();
        let output = input
            .map_content(&mut |node| {
                if let Content::Text(text) = node {
                    seen.push(text.to_string());
                }
                Ok(None)
            })
            .unwrap();
        assert_eq!(seen, ["a", "b", "c"]);
        let (Content::Table(before), Content::Table(after)) = (&input, &output) else {
            panic!("table")
        };
        assert!(Arc::ptr_eq(before, after));
    }

    #[test]
    fn change_aware_rebuilds_only_changed_branches_and_parent_sees_new_child() {
        let input = table();
        let mut parent_text = Vec::new();
        let output = input
            .map_content(&mut |node| {
                if matches!(node, Content::Text(text) if text.as_str() == "b") {
                    return Ok(Some(Content::text("B")));
                }
                if let Content::TableCell(cell) = node {
                    parent_text.push(cell.body.plain_text());
                }
                Ok(None)
            })
            .unwrap();
        assert_eq!(parent_text, ["aB", "c"]);
        let (Content::Table(before), Content::Table(after)) = (&input, &output) else {
            panic!("table")
        };
        assert!(!Arc::ptr_eq(before, after));
        let (Content::TableCell(unchanged), Content::TableCell(reused)) =
            (&before.children[1], &after.children[1])
        else {
            panic!("cell")
        };
        assert!(Arc::ptr_eq(unchanged, reused));
        assert_eq!(before.children[0].plain_text(), "ab");
        assert_eq!(after.children[0].plain_text(), "aB");
    }

    #[test]
    fn change_aware_equal_explicit_replacement_is_still_materialized() {
        let input =
            Content::Sequence(Arc::from([Content::text("same"), Content::text("other")]));
        let output = input
            .map_content(&mut |node| {
                Ok(if matches!(node, Content::Text(text) if text.as_str() == "same") {
                    Some(node.clone())
                } else {
                    None
                })
            })
            .unwrap();
        assert_eq!(input, output);
        let (Content::Sequence(before), Content::Sequence(after)) = (&input, &output)
        else {
            panic!("sequence")
        };
        assert!(!Arc::ptr_eq(before, after));
    }

    #[test]
    fn change_aware_error_stops_before_later_sibling_and_parent() {
        let input = table();
        let mut seen = Vec::new();
        let result = input.map_content(&mut |node| {
            if let Content::Text(text) = node {
                seen.push(text.to_string());
                if text.as_str() == "b" {
                    return Err(vec![SourceDiagnostic::error(Span::detached(), "stop")]);
                }
            }
            Ok(None)
        });
        assert_eq!(seen, ["a", "b"]);
        assert_eq!(result.unwrap_err()[0].message, "stop");
    }

    #[test]
    fn change_aware_prepared_table_visits_visual_and_preserves_raw() {
        let Content::Table(raw) = table() else { panic!("table") };
        let mut prepared = raw.as_ref().clone();
        prepared.resolved = Some(Arc::new(TableGridResolved {
            cells: vec![Content::table_cell(
                Content::text("visual"),
                None,
                None,
                None,
                None,
            )],
            num_cols: 1,
            num_rows: 1,
            header_rows: 0,
            footer_start: 1,
            complete: true,
        }));
        prepared.caption = Some(Content::text("caption"));
        let input = Content::Table(Arc::new(prepared));
        let mut seen = Vec::new();
        let output = input
            .map_content(&mut |node| {
                if let Content::Text(text) = node {
                    seen.push(text.to_string());
                    if text.as_str() == "visual" {
                        return Ok(Some(Content::text("new")));
                    }
                }
                Ok(None)
            })
            .unwrap();
        assert_eq!(seen, ["visual", "caption"]);
        let Content::Table(after) = output else { panic!("table") };
        assert_eq!(after.children, raw.children);
        assert_eq!(after.resolved.as_ref().unwrap().cells[0].plain_text(), "new");
    }

    #[test]
    fn change_aware_carriers_preserve_callback_boundaries() {
        let input = Content::Realized(Arc::new(RealizedContent {
            original: Content::text("hidden"),
            visual: Content::Located(Arc::new(LocatedContent {
                location: Location::from_raw(7),
                body: Content::Sequence(Arc::from([
                    Content::text("a"),
                    Content::text("b"),
                ])),
            })),
        }));
        let mut text = Vec::new();
        let mut sequences = 0;
        let output = input
            .map_content(&mut |node| {
                if let Content::Text(value) = node {
                    text.push(value.to_string());
                }
                if matches!(node, Content::Sequence(_)) {
                    sequences += 1;
                }
                Ok(None)
            })
            .unwrap();
        assert_eq!(text, ["a", "b"]);
        assert_eq!(sequences, 0);
        let (Content::Realized(before), Content::Realized(after)) = (&input, &output)
        else {
            panic!("realized")
        };
        assert!(Arc::ptr_eq(before, after));
    }

    #[test]
    fn change_aware_prepared_cell_preserves_properties_guards_and_hidden_body() {
        use crate::entities::elements::table_cell::{
            TableCellResolved, TableCellStage, TableCellStandaloneResolved,
        };
        let Content::TableCell(raw) =
            Content::table_cell(Content::text("hidden"), None, None, None, None)
        else {
            panic!("cell")
        };
        for standalone in [false, true] {
            let mut cell = raw.as_ref().clone();
            cell.resolved = Some(Arc::new(TableCellResolved {
                x: 2,
                y: 3,
                colspan: 2,
                rowspan: 1,
                fill: None,
                stroke: None,
                align: None,
                inset: Default::default(),
                breakable: false,
                visual: Content::text("visual"),
                stage: TableCellStage::Complete,
                guards: vec![11],
            }));
            if standalone {
                cell.standalone = Some(Arc::new(TableCellStandaloneResolved {
                    properties: Some(raw.properties()),
                    visual: Content::text("standalone"),
                    stage: TableCellStage::RecipeApplied,
                    guards: vec![13],
                }));
            }
            let input = Content::TableCell(Arc::new(cell));
            let unchanged = input.map_content(&mut |_| Ok(None)).unwrap();
            let (Content::TableCell(before), Content::TableCell(reused)) =
                (&input, unchanged)
            else {
                panic!("cell")
            };
            assert!(Arc::ptr_eq(before, &reused));
            let mut seen = Vec::new();
            let output = input
                .map_content(&mut |node| {
                    if let Content::Text(text) = node {
                        seen.push(text.to_string());
                        return Ok(Some(Content::text("new")));
                    }
                    Ok(None)
                })
                .unwrap();
            assert_eq!(seen, [if standalone { "standalone" } else { "visual" }]);
            let Content::TableCell(after) = output else { panic!("cell") };
            assert_eq!(after.body, before.body);
            assert_eq!(after.presence, before.presence);
            if standalone {
                let old = before.standalone.as_ref().unwrap();
                let new = after.standalone.as_ref().unwrap();
                assert_eq!(new.properties, old.properties);
                assert_eq!(new.stage, old.stage);
                assert_eq!(new.guards, old.guards);
                assert_eq!(new.visual.plain_text(), "new");
                assert!(Arc::ptr_eq(
                    before.resolved.as_ref().unwrap(),
                    after.resolved.as_ref().unwrap()
                ));
            } else {
                let plan = after.resolved.as_ref().unwrap();
                assert_eq!((plan.x, plan.y, plan.colspan, plan.rowspan), (2, 3, 2, 1));
                assert_eq!(plan.stage, TableCellStage::Complete);
                assert_eq!(plan.guards, [11]);
                assert_eq!(plan.visual.plain_text(), "new");
            }
        }
    }

    #[test]
    fn change_aware_source_retains_canonical_origin_and_normalizes_nested_origin() {
        let file = crate::entities::file_id::FileId::from_raw(
            std::num::NonZeroU16::new(1).unwrap(),
        );
        let inner_span = Span::from_range(file, 1..2);
        let canonical = Content::text("a").with_source_span(inner_span);
        let unchanged = canonical.map_content(&mut |_| Ok(None)).unwrap();
        let (Content::Source(before), Content::Source(after)) = (&canonical, unchanged)
        else {
            panic!("source")
        };
        assert!(Arc::ptr_eq(before, &after));
        let nested = Content::Source(Arc::new(SourceContent {
            body: canonical.clone(),
            span: Span::from_range(file, 3..4),
        }));
        let mapped = nested.map_content(&mut |_| Ok(None)).unwrap();
        assert_eq!(mapped.source_span(), inner_span);
        let changed = nested
            .map_content(&mut |node| {
                Ok(if matches!(node, Content::Text(_)) {
                    Some(Content::text("b"))
                } else {
                    None
                })
            })
            .unwrap();
        assert_eq!(changed.source_span(), inner_span);
        assert_eq!(changed.plain_text(), "b");
        let detached = Content::Source(Arc::new(SourceContent {
            body: Content::text("a"),
            span: Span::detached(),
        }));
        assert!(matches!(
            detached.map_content(&mut |_| Ok(None)).unwrap(),
            Content::Text(_)
        ));
    }
}

#[cfg(test)]
mod preorder_tests {
    use super::*;

    #[test]
    fn bottom_up_bounded_stack_preserves_change_proof_and_order() {
        let mut original = Content::text("X");
        for _ in 0..128 {
            original = Content::strong(original);
        }
        let mut seen = Vec::new();
        let unchanged = original
            .map_content(&mut |node| {
                if matches!(node, Content::Text(_)) {
                    seen.push("text");
                }
                if matches!(node, Content::Strong(_)) {
                    seen.push("strong");
                }
                Ok(None)
            })
            .unwrap();
        assert_eq!(seen.len(), 129);
        assert_eq!(seen[0], "text");
        assert!(seen[1..].iter().all(|kind| *kind == "strong"));
        let (Content::Strong(before), Content::Strong(after)) = (&original, &unchanged)
        else {
            panic!("expected Strong")
        };
        assert!(Arc::ptr_eq(before, after));
        let changed = original
            .map_content(&mut |node| {
                Ok(if matches!(node, Content::Text(_)) {
                    Some(node.clone())
                } else {
                    None
                })
            })
            .unwrap();
        let Content::Strong(after) = &changed else { panic!("expected Strong") };
        assert!(!Arc::ptr_eq(before, after));
        assert_eq!(changed.plain_text(), "X");
    }

    #[test]
    fn pre_order_bounded_stack_keeps_nested_wrappers_and_short_circuit() {
        let mut original = Content::text("X");
        for _ in 0..64 {
            original = Content::strong(original);
        }
        let mut strong = 0;
        let mut text = 0;
        let result = original
            .map_content_pre(&mut |node| {
                if matches!(node, Content::Strong(_)) {
                    strong += 1;
                }
                if matches!(node, Content::Text(_)) {
                    text += 1;
                }
                Ok(None)
            })
            .unwrap();
        assert_eq!(strong, 64);
        assert_eq!(text, 1);
        assert_eq!(result, original);
        let mut visited = 0;
        let replaced = original
            .map_content_pre(&mut |_| {
                visited += 1;
                Ok(Some(Content::text("terminal")))
            })
            .unwrap();
        assert_eq!(visited, 1);
        assert_eq!(replaced.plain_text(), "terminal");
        visited = 0;
        let result = original.map_content_pre(&mut |_| {
            visited += 1;
            if visited == 3 {
                return Err(vec![
                    crate::entities::source_result::SourceDiagnostic::error(
                        crate::entities::span::Span::detached(),
                        "stop",
                    ),
                ]);
            }
            Ok(None)
        });
        assert_eq!(visited, 3);
        assert_eq!(result.unwrap_err()[0].message, "stop");
    }

    #[test]
    fn math_bare_insertion_preorder_subtree_is_terminal_and_public_walk_is_unchanged() {
        let original = Content::sequence(vec![
            Content::equation(Content::text("hidden"), false),
            Content::text("visible"),
        ]);
        let mut seen = Vec::new();
        let mapped = original
            .map_content_pre(&mut |node| {
                if matches!(node, Content::Equation(_)) {
                    return Ok(Some(Content::text("replacement")));
                }
                if let Content::Text(s) = node {
                    seen.push(s.to_string());
                }
                Ok(None)
            })
            .unwrap();
        assert_eq!(seen, vec!["visible"]);
        assert_eq!(mapped.plain_text(), "replacementvisible");
        let mut seen = Vec::new();
        original
            .map_content(&mut |node| {
                if let Content::Text(s) = node {
                    seen.push(s.to_string());
                }
                Ok(None)
            })
            .unwrap();
        assert_eq!(seen, vec!["hidden", "visible"]);
    }
}

#[cfg(test)]
mod container_reuse_tests {
    use super::*;
    use crate::entities::elements::math_attach::MathAttachSlot;
    use crate::entities::layout_types::Length;
    use crate::entities::rel::Rel;
    use crate::entities::sides::Sides;

    fn cases() -> Vec<Content> {
        let c = || {
            Content::sequence(vec![
                Content::text("seed"),
                Content::strong(Content::text("sibling")),
            ])
        };
        vec![
            Content::block(
                c(),
                Some(Length::pt(50.0)),
                Some(Length::pt(20.0)),
                Sides::uniform(Length::pt(2.0)),
                false,
            ),
            Content::boxed_fractional(c(), 2.0),
            Content::strong(c()),
            Content::emph(c()),
            Content::equation(c(), true),
            Content::math_frac_full(
                c(),
                c(),
                Some(crate::entities::elements::math_frac::MathFracStyle::Horizontal),
                true,
                false,
                false,
            ),
            Content::math_attach(
                c(),
                MathAttachSlot::Omitted,
                MathAttachSlot::ExplicitNone,
                MathAttachSlot::Present(c()),
                MathAttachSlot::Omitted,
                MathAttachSlot::Present(c()),
                MathAttachSlot::ExplicitNone,
            ),
            Content::math_primes(3),
            Content::math_root(Some(c()), c(), true),
            Content::math_stretch(c(), Rel::from_percent(130.0), true),
            Content::math_delimited('[', c(), ']'),
            Content::math_lr(c(), Rel::from_percent(120.0), true),
            Content::math_mid(c()),
            Content::math_overbrace(c(), Some(c()), true),
            Content::math_underbrace(c(), None, true),
            Content::math_underbracket(c(), Some(c()), false),
            Content::math_underparen(c(), None, false),
            Content::math_undershell(c(), Some(c()), true),
            Content::math_overbracket(c(), None, true),
            Content::math_overparen(c(), Some(c()), true),
            Content::math_overshell(c(), None, false),
            Content::math_matrix_full(
                vec![vec![c(), c()], vec![c(), c()]],
                ('[', ']'),
                Some(Length::em(0.4)),
                Some(Length::em(0.7)),
                None,
                Some(1),
            ),
            Content::math_cases(
                vec![vec![c()], vec![c()]],
                ('{', '\0'),
                true,
                Some(Length::em(0.6)),
            ),
            Content::math_accent(c(), '^'),
            Content::math_cancel(c()),
            Content::math_underline(c()),
            Content::math_overline(c()),
            Content::math_vec(vec![c(), c()]),
            Content::math_class_override(
                crate::entities::math_class::MathClass::Relation,
                c(),
            ),
            Content::math_limits_override(c(), true, false, true),
            Content::math_underover(c(), Some(c()), Some(c())),
            Content::math_op(c(), true),
            Content::math_align_point(),
            Content::math_styled(None, Some(true), Some(false), c(), Some(true)),
        ]
    }

    // Representation checks here qualify reuse; language preservation is also
    // checked separately by fields, effects and document qualification.
    fn same_storage(a: &Content, b: &Content) -> bool {
        match (a, b) {
            (Content::Block(a), Content::Block(b)) => Arc::ptr_eq(a, b),
            (Content::Boxed(a), Content::Boxed(b)) => Arc::ptr_eq(a, b),
            (Content::Strong(a), Content::Strong(b)) => Arc::ptr_eq(a, b),
            (Content::Emph(a), Content::Emph(b)) => Arc::ptr_eq(a, b),
            (Content::Equation(a), Content::Equation(b)) => Arc::ptr_eq(a, b),
            (Content::MathAccent(a), Content::MathAccent(b)) => Arc::ptr_eq(a, b),
            (Content::MathAlignPoint(a), Content::MathAlignPoint(b)) => Arc::ptr_eq(a, b),
            (Content::MathPrimes(a), Content::MathPrimes(b)) => Arc::ptr_eq(a, b),
            (Content::MathAttach(a), Content::MathAttach(b)) => Arc::ptr_eq(a, b),
            (Content::MathCancel(a), Content::MathCancel(b)) => Arc::ptr_eq(a, b),
            (Content::MathCases(a), Content::MathCases(b)) => Arc::ptr_eq(a, b),
            (Content::MathClassOverride(a), Content::MathClassOverride(b)) => {
                Arc::ptr_eq(a, b)
            }
            (Content::MathDelimited(a), Content::MathDelimited(b)) => Arc::ptr_eq(a, b),
            (Content::MathLr(a), Content::MathLr(b)) => Arc::ptr_eq(a, b),
            (Content::MathMid(a), Content::MathMid(b)) => Arc::ptr_eq(a, b),
            (Content::MathFrac(a), Content::MathFrac(b)) => Arc::ptr_eq(a, b),
            (Content::MathLimitsOverride(a), Content::MathLimitsOverride(b)) => {
                Arc::ptr_eq(a, b)
            }
            (Content::MathMatrix(a), Content::MathMatrix(b)) => Arc::ptr_eq(a, b),
            (Content::MathOp(a), Content::MathOp(b)) => Arc::ptr_eq(a, b),
            (Content::MathRoot(a), Content::MathRoot(b)) => Arc::ptr_eq(a, b),
            (Content::MathStretch(a), Content::MathStretch(b)) => Arc::ptr_eq(a, b),
            (Content::MathStyled(a), Content::MathStyled(b)) => Arc::ptr_eq(a, b),
            (Content::MathUnderline(a), Content::MathUnderline(b)) => Arc::ptr_eq(a, b),
            (Content::MathOverline(a), Content::MathOverline(b)) => Arc::ptr_eq(a, b),
            (Content::MathOvershell(a), Content::MathOvershell(b)) => Arc::ptr_eq(a, b),
            (Content::MathOverparen(a), Content::MathOverparen(b)) => Arc::ptr_eq(a, b),
            (Content::MathOverbracket(a), Content::MathOverbracket(b)) => {
                Arc::ptr_eq(a, b)
            }
            (Content::MathOverbrace(a), Content::MathOverbrace(b)) => Arc::ptr_eq(a, b),
            (Content::MathUnderbrace(a), Content::MathUnderbrace(b)) => Arc::ptr_eq(a, b),
            (Content::MathUnderbracket(a), Content::MathUnderbracket(b)) => {
                Arc::ptr_eq(a, b)
            }
            (Content::MathUnderparen(a), Content::MathUnderparen(b)) => Arc::ptr_eq(a, b),
            (Content::MathUndershell(a), Content::MathUndershell(b)) => Arc::ptr_eq(a, b),
            (Content::MathUnderover(a), Content::MathUnderover(b)) => Arc::ptr_eq(a, b),
            (Content::MathVec(a), Content::MathVec(b)) => Arc::ptr_eq(a, b),
            _ => false,
        }
    }

    #[test]
    fn extended_reuse_all_containers_keep_storage_without_substitution() {
        let mut rebuilt = Vec::new();
        for input in cases() {
            let output = input.map_content(&mut |_| Ok(None)).unwrap();
            if !same_storage(&input, &output) {
                rebuilt.push(input.elem_name());
            }
        }
        assert!(rebuilt.is_empty(), "rebuilt: {rebuilt:?}");
    }

    #[test]
    fn extended_reuse_changes_keep_kind_metadata_and_explicit_equal_substitution() {
        for input in cases() {
            let output = input
                .map_content(&mut |node| {
                    Ok(match node {
                        Content::Text(t) if t.as_str() == "seed" => {
                            Some(Content::text("replaced"))
                        }
                        _ => None,
                    })
                })
                .unwrap();
            let expected = input
                .map_content_pre(&mut |node| {
                    Ok(match node {
                        Content::Text(t) if t.as_str() == "seed" => {
                            Some(Content::text("replaced"))
                        }
                        _ => None,
                    })
                })
                .unwrap();
            assert_eq!(output, expected, "{} fields", input.elem_name());
            let explicit = input
                .map_content_if_changed(&mut |node| Ok(Some(node.clone())))
                .unwrap();
            assert!(explicit.is_some(), "{} explicit replacement", input.elem_name());
        }
        let input = Content::strong(Content::text("equal"));
        let output = input
            .map_content(&mut |node| {
                Ok(if matches!(node, Content::Text(_)) {
                    Some(node.clone())
                } else {
                    None
                })
            })
            .unwrap();
        assert!(!same_storage(&input, &output));
        assert_eq!(input.plain_text(), output.plain_text());
    }

    #[test]
    fn extended_reuse_attach_visits_slots_in_order_and_keeps_presence() {
        let input = Content::math_attach(
            Content::text("base"),
            MathAttachSlot::ExplicitNone,
            MathAttachSlot::Present(Content::text("b")),
            MathAttachSlot::Omitted,
            MathAttachSlot::Present(Content::text("bl")),
            MathAttachSlot::Present(Content::text("tr")),
            MathAttachSlot::Omitted,
        );
        let mut seen = Vec::new();
        let output = input
            .map_content(&mut |node| {
                if let Content::Text(t) = node {
                    seen.push(t.to_string());
                }
                Ok(if matches!(node, Content::Text(t) if t.as_str() == "bl") {
                    Some(Content::emph(Content::text("changed")))
                } else {
                    None
                })
            })
            .unwrap();
        assert_eq!(seen, ["base", "b", "bl", "tr"]);
        let Content::MathAttach(e) = output else { panic!("attach") };
        assert!(matches!(e.t, MathAttachSlot::ExplicitNone));
        assert!(matches!(e.tl, MathAttachSlot::Omitted));
        assert!(matches!(e.br, MathAttachSlot::Omitted));
        assert!(matches!(&e.bl, MathAttachSlot::Present(Content::Emph(_))));
    }

    #[test]
    fn extended_reuse_matrix_change_keeps_siblings_augment_and_row_order() {
        let unchanged = Content::strong(Content::text("stable"));
        let input = Content::math_matrix_full(
            vec![
                vec![unchanged.clone(), Content::text("target")],
                vec![unchanged.clone(), Content::text("last")],
            ],
            ('[', ']'),
            Some(Length::em(0.7)),
            Some(Length::em(0.9)),
            None,
            Some(1),
        );
        let mut seen = Vec::new();
        let output = input
            .map_content(&mut |node| {
                if let Content::Text(t) = node {
                    seen.push(t.to_string());
                }
                Ok(if matches!(node, Content::Text(t) if t.as_str() == "target") {
                    Some(Content::text("changed"))
                } else {
                    None
                })
            })
            .unwrap();
        assert_eq!(seen, ["stable", "target", "stable", "last"]);
        let (Content::MathMatrix(a), Content::MathMatrix(b)) = (&input, output) else {
            panic!("matrix")
        };
        assert_eq!(a.augment, b.augment);
        assert_eq!(a.explicit, b.explicit);
        assert_eq!(
            (a.delim, a.align, a.row_gap, a.column_gap),
            (b.delim, b.align, b.row_gap, b.column_gap)
        );
        assert!(same_storage(&unchanged, &b.rows[0][0]));
        assert!(same_storage(&unchanged, &b.rows[1][0]));
        assert_eq!(b.rows[0][1].plain_text(), "changed");
    }

    #[test]
    fn extended_reuse_error_stops_before_later_children_and_parent() {
        let inputs = [
            Content::math_root(Some(Content::text("fail")), Content::text("later"), true),
            Content::math_underover(
                Content::text("first"),
                Some(Content::text("fail")),
                Some(Content::text("later")),
            ),
            Content::math_matrix(
                vec![
                    vec![Content::text("first"), Content::text("fail")],
                    vec![Content::text("later")],
                ],
                ('(', ')'),
            ),
        ];
        for input in inputs {
            let mut seen = Vec::new();
            let result = input.map_content(&mut |node| {
                if let Content::Text(t) = node {
                    seen.push(t.to_string());
                    if t.as_str() == "fail" {
                        return Err(vec![]);
                    }
                } else {
                    panic!("parent visited after error");
                }
                Ok(None)
            });
            assert!(result.is_err());
            assert_eq!(seen.last().map(String::as_str), Some("fail"));
            assert!(!seen.iter().any(|s| s == "later"));
        }
    }

    #[test]
    fn bibliography_static_visit_agrees_with_existing_container_fields() {
        for input in cases() {
            let mut expected = Vec::new();
            input
                .map_content(&mut |c| {
                    if let Content::Text(t) = c {
                        expected.push(t.to_string());
                    }
                    Ok(None)
                })
                .unwrap();
            let mut actual = Vec::new();
            input.visit_content_static(&mut |c| {
                if let Content::Text(t) = c {
                    actual.push(t.to_string());
                }
                true
            });
            assert_eq!(actual, expected, "{}", input.elem_name());
        }
        let input = Content::Sequence(
            vec![
                Content::text("first"),
                Content::Par { body: Box::new(Content::text("pruned")) },
                Content::Realized(Arc::new(RealizedContent {
                    original: Content::text("hidden"),
                    visual: Content::text("visible"),
                })),
                Content::text("last"),
            ]
            .into(),
        );
        let mut seen = Vec::new();
        input.visit_content_static(&mut |c| {
            if let Content::Text(t) = c {
                seen.push(t.to_string());
            }
            !matches!(c, Content::Par { .. })
        });
        assert_eq!(seen, ["first", "visible", "last"]);
    }
}
