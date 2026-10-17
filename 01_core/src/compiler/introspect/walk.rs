//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/introspect/walk.md
//! @prompt-hash 97dd2584
//! @layer L1
//! @updated 2026-10-17

use super::*;

/// Sub-stores populated directamente durante walk via
/// `populate_intr_from_tag_start` no momento de cada `Tag::Start`
/// emission. Pipeline simplificado: walk → return; eliminado etapa
/// `from_tags::from_tags` post-walk (excepto Funcs deferred).
///
/// **P190G (M6 categoria Labels & TOC)**: walk fn ganha
/// `auto_label_counter: &mut usize` parameter (Opção α `.D`).
/// Substitui field `CounterStateLegacy::auto_label_counter`
/// eliminado em P190G. Walk-internal counter para gerar IDs únicas
/// auto-toc-{n} per Heading; incrementado por walk arm Heading;
/// lido por helpers `compute_heading_auto_toc` (P191B) +
/// `compute_heading_for_toc` (P200B).
///
/// **P190I (M6 fechado)**: walk fn drop parameter `state: &mut
/// CounterStateLegacy` — struct eliminada. Walk fn ganha parameter
/// `lang: Option<&Lang>` (substitui `state.lang` lido por walk arm
/// Labelled per P191C Opção β). Net signature: 7 parameters
/// (mantém-se).
pub(crate) fn reserve_locations(content: &Content, locator: &mut Locator) {
    match content {
        Content::Source(e) => return reserve_locations(&e.body, locator),
        Content::Located(e) => {
            reserve_locations(&e.body, locator);
            locator.observe(e.location);
            return;
        }
        Content::Realized(e) => {
            reserve_locations(&e.visual, locator);
            reserve_locations(&e.original, locator);
            return;
        }
        Content::ShowRecipe(e) => return reserve_locations(&e.body, locator),
        Content::Styled(e) => return reserve_locations(&e.body, locator),
        Content::Sequence(parts) => {
            for part in parts.iter() {
                reserve_locations(part, locator);
            }
            return;
        }
        Content::Table(e) => {
            if let Some(resolved) = &e.resolved {
                for cell in &resolved.cells {
                    reserve_locations(cell, locator);
                }
            } else {
                for child in &e.children {
                    reserve_locations(child, locator);
                }
                if let Some(header) = &e.header {
                    reserve_locations(header, locator);
                }
                if let Some(footer) = &e.footer {
                    reserve_locations(footer, locator);
                }
            }
            if let Some(caption) = &e.caption {
                reserve_locations(caption, locator);
            }
            return;
        }
        Content::TableCell(e) => {
            let visual = e
                .standalone
                .as_ref()
                .map(|r| &r.visual)
                .or_else(|| e.resolved.as_ref().map(|r| &r.visual))
                .unwrap_or(&e.body);
            return reserve_locations(visual, locator);
        }
        Content::Empty
        | Content::Text(_)
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
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
        | Content::Repeat(_)
        | Content::Columns(_)
        | Content::Metadata(_)
        | Content::State(_)
        | Content::StateUpdate(_)
        | Content::StateDisplay(_)
        | Content::CounterDisplayCallback(_)
        | Content::ContextBlock(_)
        | Content::Dynamic(_) => {}
    }
    let _ = content.map_content(&mut |node| {
        match node {
            Content::Located(e) => locator.observe(e.location),
            Content::Source(e) => reserve_locations(&e.body, locator),
            Content::Realized(e) => reserve_locations(&e.original, locator),
            Content::ShowRecipe(_)
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
            | Content::Dynamic(_) => {}
        }
        Ok(None)
    });
}

#[expect(
    clippy::too_many_arguments,
    reason = "C007: Travessia pura recebe os acumuladores mutáveis da compilação e o contexto lexical por empréstimo"
)]
pub(crate) fn walk(
    content: &Content,
    locator: &mut Locator,
    tags: &mut Vec<Tag>,
    intr: &mut TagIntrospector,
    auto_label_counter: &mut usize,
    lang: Option<&crate::entities::lang::Lang>,
    // P363 (F-5a, introspect-chain): a chain léxica threaded no walk. Empurrada
    // ao descer num `Content::Styled` (espelho de `layout/mod.rs:1248`), carrega o
    // gate de numbering (`X.numbering`) para o introspect ler de UMA fonte só (a
    // chain) no de-bake (P364). **Aditivo aqui** — usada pelo push em `Styled` e
    // pela probe `#[cfg(test)]` no arm Heading; os reads de gate continuam no campo
    // assado até o P364. Separação gate-vs-número: NÃO toca o contador (P335).
    chain: &StyleChain,
    label_from_parent: Option<&Label>,
) {
    walk_at(
        content,
        locator,
        tags,
        intr,
        auto_label_counter,
        lang,
        chain,
        label_from_parent,
        None,
        false,
    );
}

#[expect(
    clippy::too_many_arguments,
    reason = "C008: A recursão adiciona forced_location e shallow_table à mesma travessia"
)]
fn walk_at(
    content: &Content,
    locator: &mut Locator,
    tags: &mut Vec<Tag>,
    intr: &mut TagIntrospector,
    auto_label_counter: &mut usize,
    lang: Option<&crate::entities::lang::Lang>,
    // P363 (F-5a, introspect-chain): a chain léxica threaded no walk. Empurrada
    // ao descer num `Content::Styled` (espelho de `layout/mod.rs:1248`), carrega o
    // gate de numbering (`X.numbering`) para o introspect ler de UMA fonte só (a
    // chain) no de-bake (P364). **Aditivo aqui** — usada pelo push em `Styled` e
    // pela probe `#[cfg(test)]` no arm Heading; os reads de gate continuam no campo
    // assado até o P364. Separação gate-vs-número: NÃO toca o contador (P335).
    chain: &StyleChain,
    label_from_parent: Option<&Label>,
    forced_location: Option<Location>,
    shallow_table: bool,
) {
    match content {
        Content::Located(e) => {
            locator.observe(e.location);
            if !intr.elements.contains_key(&e.location) {
                walk_at(
                    &e.body,
                    locator,
                    tags,
                    intr,
                    auto_label_counter,
                    lang,
                    chain,
                    label_from_parent,
                    Some(e.location),
                    shallow_table,
                );
            } else if let Content::Table(t) = &e.body {
                if t.resolved.is_some() && !shallow_table {
                    walk_table_visual(
                        t,
                        locator,
                        tags,
                        intr,
                        auto_label_counter,
                        lang,
                        chain,
                    );
                }
            }
            return;
        }
        Content::Realized(e) => {
            fn table_root(c: &Content) -> bool {
                match c.source_body() {
                    Content::Table(_) => true,
                    Content::Located(c) => table_root(&c.body),
                    Content::ShowRecipe(c) => table_root(&c.body),
                    Content::Source(_)
                    | Content::Realized(_)
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
            if table_root(&e.original) {
                walk_at(
                    &e.original,
                    locator,
                    tags,
                    intr,
                    auto_label_counter,
                    lang,
                    chain,
                    label_from_parent,
                    None,
                    true,
                );
            } else {
                walk(
                    &e.original,
                    locator,
                    tags,
                    intr,
                    auto_label_counter,
                    lang,
                    chain,
                    label_from_parent,
                );
            }
            walk(
                &e.visual,
                locator,
                tags,
                intr,
                auto_label_counter,
                lang,
                chain,
                label_from_parent,
            );
            return;
        }
        Content::Source(e) => {
            walk_at(
                &e.body,
                locator,
                tags,
                intr,
                auto_label_counter,
                lang,
                chain,
                label_from_parent,
                forced_location,
                shallow_table,
            );
            return;
        }
        Content::ShowRecipe(e) => {
            walk_at(
                &e.body,
                locator,
                tags,
                intr,
                auto_label_counter,
                lang,
                chain,
                label_from_parent,
                forced_location,
                shallow_table,
            );
            return;
        }
        Content::Empty
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
        | Content::Dynamic(_) => {}
    }
    // P162 .E + P191B: emissão Tag::Start em paralelo, antes da mutação
    // de estado. populate_intr_from_tag_start popula sub-stores intr no
    // momento da emissão (ADR-0071).
    let emitted_loc = if let Some(mut payload) = do_extract_payload(content) {
        let loc = forced_location.unwrap_or_else(|| locator.next());
        // F-5a de-bake (P364, §3a.9): a equação **não baka** mais o gate. O
        // payload tira `numbering_active` da **chain** aqui (introspect-chain
        // P363), no momento da emissão — a consumição posterior (`from_tags` /
        // `populate_intr_from_tag_start`) não tem chain. Fonte única. (`block`
        // segue no payload; o gate efetivo é `block && numbering` no consumidor.)
        if let ElementPayload::Equation {
            numbering_active,
            numbering_pattern,
            numbering_callback,
            supplement,
            supplement_lang,
            number_align,
            alt,
            ..
        } = &mut payload
        {
            match chain.custom("equation.numbering") {
                Some(Value::Str(s)) => {
                    *numbering_active = true;
                    *numbering_pattern = Some(s.clone());
                    *numbering_callback = None;
                }
                Some(Value::Func(f)) => {
                    *numbering_active = true;
                    *numbering_pattern = None;
                    *numbering_callback = Some(f.clone());
                }
                _ => {
                    *numbering_active = false;
                    *numbering_pattern = None;
                    *numbering_callback = None;
                }
            }
            *supplement =
                chain.custom("equation.supplement").cloned().unwrap_or(Value::Auto);
            *supplement_lang = chain.lang();
            *number_align = match chain.custom("equation.number-align") {
                Some(Value::Align(align)) => *align,
                _ => crate::entities::layout_types::Align2D {
                    h: Some(crate::entities::layout_types::HAlign::End),
                    v: Some(crate::entities::layout_types::VAlign::Horizon),
                },
            };
            *alt = chain.custom("equation.alt").cloned().unwrap_or(Value::None);
        }
        // P788: bake da flag de numbering do Heading a partir da chain
        // (mesmo padrão dos bakes de Equation/Figure/Table abaixo). A fonte
        // é `custom("heading.numbering") == Bool(true)` (canal `rules.rs`).
        if let ElementPayload::Heading { numbering_active, .. } = &mut payload {
            *numbering_active =
                matches!(chain.custom("heading.numbering"), Some(Value::Bool(true)));
        }
        // F-5a de-bake (P365): a figura não baka mais o padrão. `is_counted` é o
        // placeholder (`caption.is_some()`, de `to_payload`) **ANDado** com o gate
        // do padrão lido da chain aqui. **P1034** — default da linguagem é "1",
        // logo ausente conta; só `Some(Value::None)` desactiva explicitamente.
        if let ElementPayload::Figure { is_counted, .. } = &mut payload {
            *is_counted &= !matches!(chain.custom("figure.numbering"), Some(Value::None));
        }
        // P461: a table não baka o padrão. `is_counted` placeholder
        // (`caption.is_some()`, de `to_payload`) **ANDado** com o gate
        // `table.numbering` lido da chain. Fonte única.
        if let ElementPayload::Table { is_counted, .. } = &mut payload {
            *is_counted &= matches!(chain.custom("table.numbering"), Some(Value::Str(_)));
        }
        intr.counters.observe_location(loc);
        intr.state.observe_location(loc);
        let info = ElementInfo { payload, label: label_from_parent.cloned() };
        populate_intr_from_tag_start(intr, &info, loc);
        // P844 (achado #47 de P831): regista o Content do elemento para
        // `Introspector::element_at` — `query()` devolve o elemento
        // (paridade vanilla), não só a Location.
        let fields = match content {
            Content::Strong(e) => {
                let mut fields = indexmap::IndexMap::default();
                fields.insert("delta".into(), Value::Int(300));
                fields.insert("body".into(), Value::Content(e.body.clone()));
                if let Some(label) = label_from_parent {
                    fields.insert("label".into(), Value::Label(label.clone()));
                }
                Some(fields)
            }
            Content::Emph(e) => {
                let mut fields = indexmap::IndexMap::default();
                fields.insert("body".into(), Value::Content(e.body.clone()));
                if let Some(label) = label_from_parent {
                    fields.insert("label".into(), Value::Label(label.clone()));
                }
                Some(fields)
            }
            Content::Heading(heading) => {
                Some(heading::snapshot_fields(heading, chain, label_from_parent))
            }
            Content::Source(_)
            | Content::ShowRecipe(_)
            | Content::Realized(_)
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
            | Content::Title(_)
            | Content::Raw(_)
            | Content::ListItem(_)
            | Content::EnumItem(_)
            | Content::Link(_)
            | Content::Equation(_)
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
            | Content::Dynamic(_) => None,
        };
        intr.elements.insert(
            loc,
            crate::entities::value::IntrospectedContent::new(content.clone(), fields),
        );
        tags.push(Tag::Start(loc, Box::new(info)));
        Some(loc)
    } else {
        None
    };

    if shallow_table && matches!(content, Content::Table(_)) {
        if let Some(loc) = emitted_loc {
            tags.push(Tag::End(loc, hash_content(content)));
        }
        return;
    }
    match content {
        Content::Source(_) | Content::Located(_) | Content::Realized(_) | Content::ShowRecipe(_) => unreachable!("carriers dispatched above"),
        Content::Sequence(seq) => {
            for item in seq.iter() {
                walk(item, locator, tags, intr, auto_label_counter, lang, chain, None);
            }
        }

        // P863: parágrafo é transparente ao walk.
        Content::Par { body } => walk(
            body, locator, tags, intr, auto_label_counter, lang, chain, label_from_parent,
        ),
        Content::PdfArtifact(e) => walk(
            &e.body,
            locator,
            tags,
            intr,
            auto_label_counter,
            lang,
            chain,
            label_from_parent,
        ),
        Content::PageRun(e) => walk(
            &e.body,
            locator,
            tags,
            intr,
            auto_label_counter,
            lang,
            chain,
            label_from_parent,
        ),
        Content::HtmlFrame(e) => walk(
            &e.body,
            locator,
            tags,
            intr,
            auto_label_counter,
            lang,
            chain,
            label_from_parent,
        ),

        // F-5b fatia 1 (P371): strong/emph são transparentes ao walk (morfologia,
        // sem custom/locatável) — descem no body, como o arm `Styled`.
        Content::Strong(e) => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),
        Content::Emph(e)   => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),

        // P408: smallcaps é transparente ao walk (morfologia, não locatável).
        Content::SmallCaps { body } => walk(
            body, locator, tags, intr, auto_label_counter, lang, chain, None,
        ),

        // P856 — ListItem/EnumItem são transparentes ao walk para que labels
        // anexadas ao conteúdo de um item sejam processadas (paridade vanilla
        // `cannot reference text`).
        Content::ListItem(e) => walk(
            &e.body, locator, tags, intr, auto_label_counter, lang, chain, label_from_parent,
        ),
        Content::EnumItem(e) => walk(
            &e.body, locator, tags, intr, auto_label_counter, lang, chain, label_from_parent,
        ),

        Content::Heading(h) => heading::walk_heading(
            h, emitted_loc, locator, tags, intr, auto_label_counter, lang, chain,
        ),

        Content::Title(e) => {
            walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None);
        }

        Content::Equation(e) => {
            // E1 fechada — Reserva 1 materializada em P199B (cenário α
            // por construção): SetEquationNumbering popula intr.state;
            // populate_intr arm Equation aplica counter gated por
            // `block && intr.state.value_at("numbering_active:equation",
            // loc) == Some(Bool(true))`.
            //
            // P190I (M6 fechado): mutação `state.step_flat("equation")`
            // ELIMINADA — populate_intr arm Equation já aplica counter
            // a `intr.counters["equation"]` no momento da emission. Walk
            // arm Equation puro — apenas desce em body.
            walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None);
        }

        Content::Figure(e) => {
            let (body, caption) = (&e.body, &e.caption);
            // P197B (cenário α) — caminho Introspector activo desde
            // P184 (variant ElementPayload::Figure + populate_intr arm
            // Figure + sub-store CounterRegistry chave `figure:{kind}`
            // + consumer C3 P184D `figure_number_at_index`).
            //
            // P190H (M6 categoria Figures eliminada): mutações walk
            // arm Figure ELIMINADAS (Opção α `.D`):
            // - `state.local_figure_counters.entry(...) += 1`.
            // - `state.figure_numbers.entry(...).push(...)`.
            // - Helper `compute_figure` removido (orphan após
            //   eliminação dos consumers walk-side).
            //
            // populate_intr Figure arm (P191C) já popula
            // `intr.counters["figure:{kind}"]` no momento da Tag::Start
            // emission (gated por `is_counted`). compute_labelled
            // (P191C migrated) consume via `intr.flat_counter_at`.
            // Layouter consumer C3 consume via `figure_number_at_index`
            // (P184D). Walk arm Figure agora puro — apenas desce em
            // body + caption.
            walk(body, locator, tags, intr, auto_label_counter, lang, chain, None);
            if let Some(cap) = caption {
                walk(cap, locator, tags, intr, auto_label_counter, lang, chain, None);
            }
        }

        // P1473 — figure.caption (escopo reduzido): não-locatável nesta
        // fatia (ver `introspect/locatable.rs`); walk puro em body+separator.
        Content::FigureCaption(e) => {
            walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None);
            if let Some(sep) = &e.separator {
                walk(sep, locator, tags, intr, auto_label_counter, lang, chain, None);
            }
        }

        Content::Label(e) => labelled::walk_label(
            e, locator, tags, intr, auto_label_counter, lang, chain,
        ),

        // Lote F-2 S5 (P335): arms populate Set*Numbering removidos com as variantes.

        Content::CounterUpdate(_) => {
            // P198C — E6 fechada estruturalmente (cenário β-promote
            // ADR-0069). Caminho Introspector activo:
            // extract_payload arm emite ElementPayload::CounterUpdate
            // pré-recursão; populate_intr arm CounterUpdate popula
            // CounterRegistry via apply_at (flat) ou
            // apply_hierarchical_at (key="heading").
            //
            // P190I (M6 fechado): mutação legacy
            // `state.step_*`/`state.update_flat` ELIMINADA — `compute_*`
            // helpers migrados para Introspector path location-aware
            // (P191B/C). populate_intr é única fonte da verdade. Walk
            // arm CounterUpdate puro — sem recursão, sem mutação.
        }

        // Passo 101: `Content::Strong`/`Content::Emph` removidos; o caso
        // equivalente de filhos com Labels/contadores dentro de um bloco
        // estilizado está agora coberto pelo arm `Content::Styled` abaixo.

        // Terminais e nós sem efeito em contadores — cobertos explicitamente
        // para que o compilador detecte variantes em falta (sem wildcard silencioso).

        Content::Empty
        | Content::Text(_)
        | Content::Space | Content::SpaceFromNewline
        // P622: Parbreak é leaf estrutural — sem efeito em counters.
        | Content::Parbreak
        | Content::Ref(_)
        | Content::CounterDisplay(_)
        | Content::Raw(_)
        | Content::Link(_)
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
        | Content::MathMatrix(_)
        | Content::MathCases(_)
        // P296 — Math accent/cancel sem children non-math em walk
        // (paralelo MathFrac/MathRoot: math layout interno não emite
        // tags introspecção).
        | Content::MathAccent(_)
        | Content::MathCancel(_)
        | Content::MathUnderline(_)
        | Content::MathOverline(_)
        | Content::MathVec(_)
        | Content::Flush(_)
        // P772y — Math class override terminal em walk (paralelo P296/P298).
        | Content::MathClassOverride(_)
        // P992 — Math limits override terminal em walk (limits()/scripts()).
        | Content::MathLimitsOverride(_)
        // P297 — Math underover terminal em walk (paralelo P296).
        | Content::MathUnderover(_)
        // P298 — Math op terminal em walk.
        | Content::MathOp(_)
        // P311b.2 — MathStyled terminal em walk (math structural).
        | Content::MathStyled(_)
        | Content::MathAlignPoint(_)
        | Content::Linebreak(_)
        // P287 — SmartQuote leaf (não-locatable; sem counters).
        | Content::SmartQuote(_)
        | Content::Image(_)
        | Content::SetPage(_)
        | Content::Divider(_)
        // Passo 156D — h/v spacing leaves; sem effect em counters.
        | Content::HSpace(_)
        | Content::VSpace(_)
        // Passo 156E — pagebreak leaf; sem effect em counters.
        | Content::Pagebreak(_)
        // Passo 220 — colbreak leaf; sem effect em counters.
        | Content::Colbreak(_)
        | Content::Shape { .. }
        // P169 (M9): Metadata é terminal — sem efeito em counters.
        // Tag::Start/End já é emitido no topo de walk via extract_payload
        // (que produz `Some(ElementPayload::Metadata)`).
        | Content::Metadata(_)
        // P171 (M9): State e StateUpdate são terminais. Tag emitido
        // no topo via extract_payload.
        | Content::State(_)
        | Content::StateUpdate(_)
        // P240 (M9d/M7+1): StateDisplay terminal. Tag emitido no topo
        // via extract_payload; valor pre-rendered em apply_state_displays.
        | Content::StateDisplay(_)
        // P241 (M9d/M7+2): CounterDisplayCallback terminal paralelo
        // StateDisplay; tag emitido no topo via extract_payload; valor
        // pre-rendered em apply_counter_displays.
        | Content::CounterDisplayCallback(_) => {}

        // Lote F-1 (P334): a fronteira dinâmica é **leaf** no walk — o tag de
        // início (se locatável) já foi emitido acima via `extract_payload`; a
        // travessia dos filhos do nó dinâmico chega com a realização em F-2
        // (L0 `f_fronteira_e1.md` §3a.7). Em F-1 só existe em fixtures.
        Content::Dynamic(_) => {}

        // Passo 154B — Terms / TermItem: descem em items para que filhos
        // com contadores ou labels sejam processados.
        Content::Terms(e) => {
            for item in e.items.iter() { walk(item, locator, tags, intr, auto_label_counter, lang, chain, None); }
        }
        Content::TermItem(e) => {
            walk(&e.term, locator, tags, intr, auto_label_counter, lang, chain, None);
            walk(&e.description, locator, tags, intr, auto_label_counter, lang, chain, None);
        }

        // Passo 155 — Quote: walk em body + attribution.
        Content::Quote(e) => {
            walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None);
            if let Some(a) = &e.attribution {
                walk(a, locator, tags, intr, auto_label_counter, lang, chain, None);
            }
        }

        // Modelo D (Lote 4 P319): decorações — walk no body (não-locatable).
        Content::Underline(e) => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),
        Content::Strike(e)    => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),
        Content::Overline(e)  => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),

        Content::Transform(e) => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),

        Content::Grid(e) => {
            // P224 — Grid refino: walk em header (se houver) + cells + footer.
            if let Some(h) = &e.header { walk(h, locator, tags, intr, auto_label_counter, lang, chain, None); }
            for cell in &e.cells { walk(cell, locator, tags, intr, auto_label_counter, lang, chain, None); }
            if let Some(f) = &e.footer { walk(f, locator, tags, intr, auto_label_counter, lang, chain, None); }
        }

        // P224.B — GridHeader / GridFooter (recurse no body; paridade P157C).
        Content::GridHeader(e) => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),
        Content::GridFooter(e) => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),

        // P224.C — GridCell (recurse no body; paridade P157B TableCell).
        Content::GridCell(e) => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),

        // Passo 512 — linhas em grid/table não têm body recursivo.
        Content::GridHLine(_) | Content::GridVLine(_) => {}
        Content::TableHLine(_) | Content::TableVLine(_) => {}

        // P461 — Table locatable. Tag emitido no walk top; aqui recursa
        // em caption (se houver) + children, espelhando Figure.
        Content::Table(e) => walk_table_visual(e,locator,tags,intr,auto_label_counter,lang,chain),

        // Passo 157B — TableCell (recurse no body).
        Content::TableCell(e) => walk(e.visual(), locator, tags, intr, auto_label_counter, lang, chain, None),

        // Passo 157C — par simétrico TableHeader/TableFooter
        // (recurse no body).
        Content::TableHeader(e) => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),
        Content::TableFooter(e) => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),

        // P181H: walk arm puro (P163 invariante restaurada para bib).
        // Pré-P181H (P159C/F): walk mutava `state.bib_entries.extend(...)`
        // e `state.bib_numbers.entry(key).or_insert(...)` directamente.
        // Pós-P181H: tag emitida no topo via `extract_payload` (P181D);
        // BibStore populado por `from_tags` arm Bibliography (P181E).
        // Apenas descida no `title` permanece — `entries` são dados
        // opacos consumidos pelo `extract_payload` que constrói
        // `ElementPayload::Bibliography { entries }`. Cite walk em
        // supplement; sem validação cross-reference (ADR-0017
        // Introspection runtime adiada).
        Content::Bibliography(e) => {
            if let Some(t) = &e.title {
                walk(t, locator, tags, intr, auto_label_counter, lang, chain, None);
            }
        }
        Content::Cite(e) => {
            if let Some(s) = &e.supplement { walk(s, locator, tags, intr, auto_label_counter, lang, chain, None); }
        }

        // P295 — Footnote walk em body (locatable infrastructure não
        // aplicada em Fase 1; body recurse preserva counters/labels
        // dentro para passes futuros).
        Content::Footnote(e) => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),

        // P1473 — footnote.entry (escopo reduzido): não-locatável nesta
        // fatia (ver `introspect/locatable.rs`); walk puro em note+separator.
        Content::FootnoteEntry(e) => {
            walk(&e.note, locator, tags, intr, auto_label_counter, lang, chain, None);
            if let Some(sep) = &e.separator {
                walk(sep, locator, tags, intr, auto_label_counter, lang, chain, None);
            }
        }

        Content::Align(e) => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),

        Content::Place(e) => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),

        // Passo 156C (ADR-0061 Fase 1) — pad / hide são containers
        // estruturais; descer no body para que counters/labels dentro sejam
        // processados. `Hide` mesmo "ocultando visualmente" mantém a
        // semântica de presence (label/ref dentro de hide ainda resolvem).
        Content::Pad(e) => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),
        Content::Hide(e)           => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),

        // Passo 156G (ADR-0061 Fase 2) — block container; descer no body.
        Content::Block(e) => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),

        // Passo 156H (ADR-0061 Fase 2 sub-passo 2) — box inline container.
        Content::Boxed(e) => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),

        // Passo 156I (ADR-0061 Fase 2 sub-passo 3) — stack compositivo.
        // Walk em cada child em ordem (counters/labels resolvem).
        Content::Stack(e) => {
            for c in e.children.iter() {
                walk(c, locator, tags, intr, auto_label_counter, lang, chain, None);
            }
        },

        // Passo 156J (ADR-0061 Fase 3 sub-passo 1) — repeat container.
        // Walk no body uma vez (counters/labels dentro de body
        // resolvem; semântica de repetição é runtime-only e não
        // multiplica state — vanilla repeat também só conta uma vez).
        Content::Repeat(e) => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),
        // P217 (DEBT-56 sub-fase b) — Columns container.
        // Walk no body (counters/labels dentro contam normalmente);
        // sem Tag::Start/End próprio (columns não é locatable).
        // Consumer multi-region em P219.
        Content::Columns(e) => walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, None),

        // Passo 99 (ADR-0038): `Styled` é transparente — desce no body.
        // P363 (introspect-chain): empurra os styles na chain ao descer (espelho de
        // `layout/mod.rs:1248` `push_styles`), para que um heading/equation/figure
        // dentro do `Content::Styled` que o `#set …(numbering:)` embrulha tenha o
        // gate `X.numbering` na chain. Escopado à subárvore (o `pushed` só vale na
        // recursão do `body`).
        Content::Styled(styled) => {
            let body = &styled.body;
            let styles = &styled.styles;
            for (key, value) in &styles.delta().custom {
                if key.as_str() != "outline.prefix-info" {
                    continue;
                }
                let Value::Array(parts) = value else { continue };
                let [Value::Int(parent_key), Value::Int(level), Value::Length(inset)] =
                    parts.as_slice()
                else {
                    continue;
                };
                if let Ok(level) = usize::try_from(*level) {
                    intr.record_outline_prefix_width(
                        *parent_key,
                        level,
                        inset.resolve_pt(0.0),
                    );
                }
            }
            let pushed = chain.push_styles(styles);
            walk(body, locator, tags, intr, auto_label_counter, lang, &pushed, label_from_parent);
        }

        Content::Outline(outline) => {
            // P1406: o corpo realizado é privado e só aparece no candidato
            // pós-introspecção. Descer nele permite recolher `prefix-info`;
            // a store consultável final continua vindo do documento original.
            if let Some(body) = outline.realized_body() {
                walk(
                    body,
                    locator,
                    tags,
                    intr,
                    auto_label_counter,
                    lang,
                    chain,
                    label_from_parent,
                );
            }
            // P189B (M5): walk puro para Outline.
            // Mutação `state.has_outline = true` removida; flag obtida
            // via `intr.kind_index.contains_key(&ElementKind::Outline)`
            // (populado por `from_tags` arm Outline P178). Consumer
            // migrado em `mod.rs:1470` (`layout_with_introspector`).
            // `Content::Outline` continua a ser locatable e emite
            // Tag::Start no topo da `walk` fn — apenas a mutação
            // directa em state foi removida.
            //
            // P480 — registo sintético de heading de título em kind_index
            // para paridade de query de count com vanilla (vanilla conta
            // o heading de título do outline via pós-layout; aqui registo
            // pré-layout como entry sintética). headings_for_toc NÃO
            // actualizado — evita TOC auto-referente. Counter NÃO
            // aplicado — título do outline não é secção numerada.
            // Abordagem directa ao kind_index adoptada em vez da Sequence
            // em native_outline (spec P480 §A.1) porque a Sequence faria
            // walk normal do Heading → headings_for_toc receberia o
            // título → layout_outline listaria o próprio título como
            // entrada na TOC (auto-referência). ADR-0108 §6: intenção
            // = count parity; comportamento = registo sintético.
            if !outline.title_was_set() || outline.title.is_some() {
                let title_loc = locator.next();
                intr.kind_index
                    .entry(ElementKind::Heading)
                    .or_default()
                    .push(title_loc);
            }
            intr.outline_elements.push((**outline).clone());
        }

        Content::OutlineEntry(_) => {
            if let Some(location) = emitted_loc {
                intr.kind_index.entry(ElementKind::OutlineEntry).or_default().push(location);
            }
        }

        // P506 — ContextBlock é terminal no walk (a tag já foi emitida no
        // topo via extract_payload; o corpo é uma closure avaliada na
        // fase de expansão pós-introspecção).
        Content::ContextBlock(_) => {}

        // P397 — Document/Asset são metadata/resources; não entram no walk
        // de conteúdo renderizável.
        Content::Document { .. } => {}
        Content::Asset { .. }    => {}

        // Passo 513 — Curve é leaf não-locatable; sem descendência.
        Content::Curve(_) | Content::PdfAttach(_) | Content::HtmlElem(_) => {}
    }

    // P162 .E: emissão Tag::End após recursão. Usa o mesmo Location
    // que o Tag::Start emitido no topo, e o hash determinístico do
    // conteúdo via hash_content.
    if let Some(loc) = emitted_loc {
        tags.push(Tag::End(loc, hash_content(content)));
    }
}

fn walk_table_visual(
    e: &crate::entities::elements::table::TableElem,
    locator: &mut Locator,
    tags: &mut Vec<Tag>,
    intr: &mut TagIntrospector,
    auto_label_counter: &mut usize,
    lang: Option<&crate::entities::lang::Lang>,
    chain: &StyleChain,
) {
    if let Some(cap) = &e.caption {
        walk(cap, locator, tags, intr, auto_label_counter, lang, chain, None);
    }
    if let Some(plan) = &e.resolved {
        let mut cells = plan.cells.iter().collect::<Vec<_>>();
        cells.sort_by_key(|c| match c {
            Content::TableCell(c) => c
                .resolved
                .as_ref()
                .map(|r| (r.y + r.rowspan - 1, r.x))
                .unwrap_or((0, 0)),
            _ => (0, 0),
        });
        for c in cells {
            walk(c, locator, tags, intr, auto_label_counter, lang, chain, None);
        }
    } else {
        for c in &e.children {
            walk(c, locator, tags, intr, auto_label_counter, lang, chain, None);
        }
    }
}

#[cfg(test)]
mod reserve_tests {
    use super::*;
    use crate::entities::content::{LocatedContent, RealizedContent};
    use crate::entities::style::Styles;
    use std::sync::Arc;

    #[test]
    fn borrowed_reservation_matches_previous_visit_for_nested_table() {
        let located = |id, body| {
            Content::Located(Arc::new(LocatedContent {
                location: Location::from_raw(id),
                body,
            }))
        };
        let cell = Content::table_cell(
            located(17, Content::text("cell")),
            None,
            None,
            None,
            None,
        );
        let table = Content::table(
            vec![],
            vec![],
            vec![cell, located(31, Content::text("tail"))],
        );
        let realized = Content::Realized(Arc::new(RealizedContent {
            original: located(83, Content::text("origin")),
            visual: Content::styled(table, Styles::new()),
        }));
        let tree = Content::sequence(vec![located(5, Content::text("head")), realized]);

        fn old_reserve(content: &Content, locator: &mut Locator) {
            let _ = content.map_content(&mut |node| {
                match node {
                    Content::Located(e) => locator.observe(e.location),
                    Content::Source(e) => old_reserve(&e.body, locator),
                    Content::Realized(e) => old_reserve(&e.original, locator),
                    Content::ShowRecipe(_)
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
                    | Content::Dynamic(_) => {}
                }
                Ok(None)
            });
        }
        let mut old = Locator::new();
        old_reserve(&tree, &mut old);
        let mut borrowed = Locator::new();
        reserve_locations(&tree, &mut borrowed);
        assert_eq!(borrowed.next(), old.next());
        assert_eq!(borrowed.next(), Location::from_raw(85));
    }
}

#[cfg(test)]
mod v16_walk_tests {
    use super::*;
    use crate::entities::{
        content::LocatedContent, file_id::FileId, func::Func, span::Span,
    };

    fn entry_selector() -> Selector {
        Selector::NativeElement(Func::native(
            "outline.entry",
            crate::compiler::stdlib::native_outline_entry,
        ))
    }

    fn source_entry() -> Content {
        Content::outline_entry(1, Content::heading(1, Content::text("reference")), None)
            .with_source_span(Span::from_range(
                FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()),
                2..5,
            ))
    }

    #[test]
    fn v16_walk_entry_emits_own_labeled_tags_without_indexing_element_field() {
        let mut locator = Locator::new();
        let mut tags = Vec::new();
        let mut intr = TagIntrospector::empty();
        let mut auto = 0;
        let label = Label("entry".into());
        walk(
            &source_entry(),
            &mut locator,
            &mut tags,
            &mut intr,
            &mut auto,
            None,
            &StyleChain::default_chain(),
            Some(&label),
        );
        assert_eq!(tags.len(), 2);
        let Tag::Start(location, info) = &tags[0] else { panic!("missing Start") };
        assert_eq!(info.payload, ElementPayload::NativeElement);
        assert_eq!(info.label.as_ref(), Some(&label));
        assert!(matches!(&tags[1], Tag::End(end, _) if end == location));
        assert_eq!(intr.query(&entry_selector()), vec![*location]);
        assert_eq!(
            intr.query(&Selector::Kind(ElementKind::OutlineEntry)),
            vec![*location]
        );
        assert_eq!(intr.query(&Selector::Label(label)), vec![*location]);
        assert!(!intr.kind_index.contains_key(&ElementKind::Heading));
        assert_eq!(auto, 0);
    }

    #[test]
    fn v16_walk_entry_direct_and_synthesized_are_distinct_without_heading_ghosts() {
        let tree = Content::sequence(vec![
            Content::outline(),
            Content::heading(1, Content::text("real")),
            source_entry(),
        ]);
        let intr = crate::compiler::introspect::introspect_with_introspector(&tree);
        let entries = intr.query(&entry_selector());
        assert_eq!(entries.len(), 2);
        assert_eq!(intr.query(&Selector::Kind(ElementKind::OutlineEntry)).len(), 2);
        assert_ne!(entries[0], entries[1]);
        let heading = Selector::NativeElement(Func::native(
            "heading",
            crate::compiler::stdlib::native_heading,
        ));
        assert_eq!(intr.query(&heading).len(), 1);
        let bodies: Vec<_> = entries
            .iter()
            .map(|location| {
                let Content::OutlineEntry(entry) =
                    intr.elements[location].content().source_body()
                else {
                    panic!("entry lost")
                };
                entry.body().unwrap().plain_text()
            })
            .collect();
        assert!(bodies.contains(&"reference".to_string()));
        assert!(bodies.contains(&"real".to_string()));
    }

    #[test]
    fn v16_walk_entry_located_alias_is_indexed_once() {
        let location = Locator::new().next();
        let entry =
            Content::Located(Arc::new(LocatedContent { location, body: source_entry() }));
        let intr = crate::compiler::introspect::introspect_with_introspector(
            &Content::sequence(vec![entry.clone(), entry]),
        );
        assert_eq!(intr.query(&entry_selector()), vec![location]);
        assert_eq!(intr.elements.len(), 1);
    }
}

#[cfg(test)]
mod v16_walk_source_context_tests {
    use super::*;
    use crate::entities::{file_id::FileId, span::Span};

    #[test]
    fn v16_walk_source_table_preserves_forced_location_and_shallow_mode() {
        let table = Content::table(
            vec![],
            vec![],
            vec![Content::heading(1, Content::text("hidden"))],
        )
        .with_source_span(Span::from_range(
            FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()),
            2..5,
        ));
        let forced = Locator::new().next();
        let mut locator = Locator::new();
        let _ = locator.next();
        let _ = locator.next();
        let mut tags = Vec::new();
        let mut intr = TagIntrospector::empty();
        let mut auto = 0;
        walk_at(
            &table,
            &mut locator,
            &mut tags,
            &mut intr,
            &mut auto,
            None,
            &StyleChain::default_chain(),
            None,
            Some(forced),
            true,
        );
        assert_eq!(tags.len(), 2);
        assert!(matches!(&tags[0], Tag::Start(location, _) if *location == forced));
        assert!(matches!(&tags[1], Tag::End(location, _) if *location == forced));
        assert!(!intr.kind_index.contains_key(&ElementKind::Heading));
        assert_eq!(auto, 0);
    }
}

#[cfg(test)]
mod v16_eval_math_table_root_tests {
    use super::*;
    use crate::entities::{
        content::{LocatedContent, RealizedContent, ShowRecipeContent, SourceContent},
        file_id::FileId,
        span::Span,
    };

    fn source(body: Content) -> Content {
        Content::Source(Arc::new(SourceContent {
            body,
            span: Span::from_range(
                FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()),
                2..5,
            ),
        }))
    }
    fn realized(original: Content, visual: Content) -> Content {
        Content::Realized(Arc::new(RealizedContent { original, visual }))
    }
    fn table() -> Content {
        Content::table(vec![], vec![], vec![Content::heading(1, Content::text("hidden"))])
    }
    fn recipe(body: Content) -> Content {
        Content::ShowRecipe(Arc::new(ShowRecipeContent {
            body,
            textual: false,
            context_id_floor: 0,
            rules: Arc::from([]),
            styles: StyleChain::default_chain(),
            guards: vec![],
            file: FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()),
        }))
    }
    fn observed(content: &Content) -> (Vec<String>, usize) {
        let intr = crate::compiler::introspect::introspect(content);
        let headings = intr
            .query(&Selector::Kind(ElementKind::Heading))
            .into_iter()
            .map(|loc| intr.elements[&loc].content().plain_text())
            .collect();
        (headings, intr.query(&Selector::Kind(ElementKind::Table)).len())
    }
    #[test]
    fn v16_eval_math_raw_table_origin_remains_shallow() {
        assert_eq!(
            observed(&realized(table(), Content::text("replacement"))),
            (vec![], 1)
        );
    }
    #[test]
    fn v16_eval_math_source_table_carriers_hide_original_heading() {
        let loc = Location::from_raw(13);
        let located =
            |body| Content::Located(Arc::new(LocatedContent { location: loc, body }));
        let wrapped_recipe = recipe(source(table()));
        let cases = [
            ("one", source(table())),
            ("nested", source(source(table()))),
            ("located-source", located(source(table()))),
            ("source-located", source(located(table()))),
            ("recipe-source", wrapped_recipe),
            ("recipe-plain", recipe(table())),
        ];
        let actual: Vec<_> = cases
            .into_iter()
            .map(|(name, original)| {
                (name, observed(&realized(original, Content::text("replacement"))))
            })
            .collect();
        let expected: Vec<_> = [
            "one",
            "nested",
            "located-source",
            "source-located",
            "recipe-source",
            "recipe-plain",
        ]
        .into_iter()
        .map(|name| (name, (Vec::<String>::new(), 1)))
        .collect();
        assert_eq!(actual, expected);
    }
    #[test]
    fn v16_eval_math_source_table_keeps_visual_heading_and_location() {
        let loc = Location::from_raw(13);
        let original = Content::Located(Arc::new(LocatedContent {
            location: loc,
            body: source(table()),
        }));
        let doc = realized(original, Content::heading(1, Content::text("shown")));
        assert_eq!(observed(&doc), (vec!["shown".to_string()], 1));
        let intr = crate::compiler::introspect::introspect(&doc);
        assert_eq!(intr.query(&Selector::Kind(ElementKind::Table)), vec![loc]);
    }
    #[test]
    fn v16_eval_math_non_table_source_retains_origin_and_visual() {
        let doc = realized(
            source(Content::heading(1, Content::text("original"))),
            Content::heading(1, Content::text("shown")),
        );
        assert_eq!(
            observed(&doc),
            (vec!["original".to_string(), "shown".to_string()], 0)
        );
    }
    #[test]
    fn v16_eval_math_show_recipe_preserves_location_and_shallow_context() {
        let loc = Location::from_raw(13);
        let original = Content::Located(Arc::new(LocatedContent {
            location: loc,
            body: recipe(source(table())),
        }));
        let doc = realized(original, Content::heading(1, Content::text("shown")));
        let intr = crate::compiler::introspect::introspect(&doc);
        assert_eq!(
            (observed(&doc), intr.query(&Selector::Kind(ElementKind::Table))),
            ((vec!["shown".to_string()], 1), vec![loc])
        );
    }
}
