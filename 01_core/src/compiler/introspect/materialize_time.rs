//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/introspect/materialize_time.md
//! @prompt-hash 51b66794
//! @layer L1
//! @updated 2026-10-07

use super::*;

/// "Congela" o AST substituindo nós dependentes de contexto (como CounterDisplay)
/// pelos seus valores em texto estático no momento exacto da introspecção (Passo 66, DEBT-18).
///
/// Resolve DEBT-18: sem esta função, a TOC mostraria os valores dos contadores
/// no início do documento, não o valor que cada contador tinha quando o título ocorreu.
///
/// Dois braços explícitos — sem wildcard para manter verificação de exaustividade:
/// - Containers: propagam recursivamente.
/// - Terminais: clonados directamente.
///
/// **P190I (ADR-0070 ACEITE)** — signature migrada para
/// `(content, intr, location)`. Reads `state.display_value(kind)`
/// substituídos por `intr.formatted_counter_at(kind, location)`.
/// Caminho Introspector path location-aware. Walk fn deixou de
/// receber `state: &mut CounterStateLegacy` (struct eliminada em
/// P190I).
pub(super) fn materialize_time(
    content: &Content,
    intr: &TagIntrospector,
    location: Location,
) -> Content {
    match content {
        Content::Source(e) => materialize_time(&e.body, intr, location).with_source_span(e.span),
        Content::ShowRecipe(e) => Content::ShowRecipe(Arc::new(crate::entities::content::ShowRecipeContent {
            body: materialize_time(&e.body, intr, location), ..e.as_ref().clone()
        })),
        Content::Located(e) => Content::Located(Arc::new(crate::entities::content::LocatedContent {
            body: materialize_time(&e.body, intr, e.location), location: e.location,
        })),
        Content::Realized(e) => Content::Realized(Arc::new(crate::entities::content::RealizedContent {
            original: e.original.clone(), visual: materialize_time(&e.visual, intr, location),
        })),
        // O caso crítico: substituir o nó dinâmico pelo valor actual do contador.
        Content::CounterDisplay(e) => {
            Content::text(
                intr.formatted_counter_at(&e.kind, location)
                    .unwrap_or_else(|| "0".to_string()),
            )
        }

        // ── Containers com filhos (propagação recursiva) ──────────────────
        Content::Sequence(seq) => {
            Content::Sequence(
                seq.iter().map(|c| materialize_time(c, intr, location)).collect::<Vec<_>>().into()
            )
        }
        // P863: parágrafo é transparente para materialização de tempo.
        Content::Par { body } => Content::par(materialize_time(body, intr, location)),
        Content::PageRun(e) => Content::page_run_with_geometry(
            e.paper,
            e.flipped,
            e.binding,
            e.width,
            e.height,
            e.margin,
            e.numbering.clone(),
            e.columns,
            materialize_time(&e.body, intr, location),
        ),
        // F-5b fatia 1 (P371): strong/emph voltaram a variantes próprias (o
        // colapso P101 foi superado); reconstroem via ctor, recursando no body.
        Content::Strong(e) => Content::strong(materialize_time(&e.body, intr, location)),
        Content::Emph(e)   => Content::emph(materialize_time(&e.body, intr, location)),
        // P408: smallcaps é container transparente para materialização de tempo
        // (o consumer real de small caps será aplicado no layout; DEBT-53).
        Content::SmallCaps { body } => Content::smallcaps(materialize_time(body, intr, location)),
        // Modelo D (P316): Heading delegado; reconstrói via ctor.
        // P606 — preserva `outlined` e `bookmarked` durante materialização de tempo.
        // P829 — `with_body` preserva também `set_fields` (métodos has/at/fields).
        Content::Heading(h) => Content::Heading(Arc::new(
            h.with_body(materialize_time(&h.body, intr, location)),
        )),
        Content::Title(e) => Content::Title(Arc::new(crate::entities::elements::title::TitleElem {
            body: materialize_time(&e.body, intr, location),
        })),
        // Modelo D (Lote 3 P318): destructure de Arc<Elem> + reconstrução via construtor.
        Content::ListItem(e) => {
            let body = materialize_time(&e.body, intr, location);
            match &e.marker {
                None    => Content::list_item(body),
                Some(m) => Content::list_item_with_marker(body, m.clone()),
            }
        }
        Content::EnumItem(e) => {
            let body = materialize_time(&e.body, intr, location);
            match &e.numbering {
                None    => Content::enum_item(e.number, body),
                Some(n) => Content::enum_item_with_numbering(e.number, body, n.clone()),
            }
        }
        Content::Link(e) => Content::link(e.url.clone(), materialize_time(&e.body, intr, location)),
        // P464: Label — recurse no body via construtor, preservando origem.
        Content::Label(e) => {
            let body = materialize_time(&e.body, intr, location);
            if e.auto {
                Content::label_auto(e.name.clone(), body)
            } else {
                Content::label(e.name.clone(), body)
            }
        }
        // Keep lexical origins and the prepared occurrence snapshot intact.
        Content::Figure(e) => Content::Figure(std::sync::Arc::new(
            crate::entities::elements::figure::FigureElem {
                body: materialize_time(&e.body, intr, location),
                caption: e.caption.as_ref().map(|c| materialize_time(c, intr, location)),
                kind: e.kind.clone(),
                resolved_kind: e.resolved_kind.clone(),
                supplement: if e.resolved_supplement.is_some() {
                    e.supplement.clone()
                } else {
                    match &e.supplement {
                        Some(crate::entities::elements::figure::FigureSupplement::Content(content)) => Some(crate::entities::elements::figure::FigureSupplement::Content(materialize_time(content, intr, location))),
                        Some(crate::entities::elements::figure::FigureSupplement::Auto | crate::entities::elements::figure::FigureSupplement::None | crate::entities::elements::figure::FigureSupplement::Func { .. }) | None => e.supplement.clone(),
                    }
                },
                resolved_supplement: e.resolved_supplement.as_ref().map(|content| content.as_ref().map(|c| materialize_time(c, intr, location))),
            },
        )),

        // P1473 — figure.caption (escopo reduzido): recurse body+separator.
        Content::FigureCaption(e) => Content::figure_caption(
            materialize_time(&e.body, intr, location),
            e.position,
            e.separator.as_ref().map(|c| materialize_time(c, intr, location)),
        ),

        // Modelo D (Lote 3 P318): Terms/TermItem delegam reconstrução ao construtor.
        Content::Terms(e) => Content::terms(
            e.items.iter().map(|c| materialize_time(c, intr, location)).collect(),
        ),
        Content::TermItem(e) => Content::term_item(
            materialize_time(&e.term, intr, location),
            materialize_time(&e.description, intr, location),
        ),

        // Passo 155: Quote — recurse em body e attribution.
        // Modelo D (Lote 8 P323): destrutura o `Arc<QuoteElem>`.
        Content::Quote(e) => Content::quote(
            materialize_time(&e.body, intr, location),
            e.attribution.as_ref().map(|c| materialize_time(c, intr, location)),
            e.block,
            e.quotes,
        ),

        // P295 — Footnote recurse em body (paridade Quote).
        Content::Footnote(e) => Content::footnote(materialize_time(&e.body, intr, location)),

        // P1473 — footnote.entry (escopo reduzido): recurse note+separator.
        Content::FootnoteEntry(e) => Content::footnote_entry(
            materialize_time(&e.note, intr, location),
            e.separator.as_ref().map(|c| materialize_time(c, intr, location)),
            e.clearance,
            e.gap,
            e.indent,
        ),

        // Modelo D (Lote 4 P319): decorações — recurse no body via construtor.
        Content::Underline(e) => Content::underline(
            materialize_time(&e.body, intr, location), e.stroke.clone(), e.offset, e.extent),
        Content::Strike(e) => Content::strike(
            materialize_time(&e.body, intr, location), e.stroke.clone(), e.offset, e.extent),
        Content::Overline(e) => Content::overline(
            materialize_time(&e.body, intr, location), e.stroke.clone(), e.offset, e.extent),

        // ── Terminais — clonar directamente ──────────────────────────────
        // Nós matemáticos (Equation e subtipos) não podem conter CounterDisplay
        // em markup válido — clonados em bloco sem recursão.
        Content::Empty
        | Content::Text(_)
        | Content::Space | Content::SpaceFromNewline
        // P622: Parbreak é leaf estrutural — sem CounterDisplay possível.
        | Content::Parbreak
        | Content::Raw(_)
        | Content::Ref(_)
        | Content::SetPage(_)
        | Content::CounterUpdate(_)
        | Content::Outline(_)
        | Content::Linebreak(_)
        | Content::MathAlignPoint(_)
        | Content::MathIdent(_)
        | Content::MathText(_)
        // P287 — SmartQuote leaf (sem CounterDisplay possível).
        | Content::SmartQuote(_)
        | Content::Equation { .. }
        | Content::MathSequence(_)
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
        // P296 — Math accent/cancel terminais em materialize_time
        // (paralelo MathFrac/MathRoot; sem CounterDisplay no body).
        | Content::MathAccent(_)
        | Content::MathCancel(_)
        | Content::MathUnderline(_)
        | Content::MathOverline(_)
        | Content::MathVec(_)
        | Content::Flush(_)
        // P772y — Math class override terminal (paralelo P296/P298).
        | Content::MathClassOverride(_)
        // P992 — Math limits override terminal (limits()/scripts()).
        | Content::MathLimitsOverride(_)
        // P297 — Math underover terminal (paralelo P296).
        | Content::MathUnderover(_)
        // P298 — Math op terminal (paralelo cluster math).
        | Content::MathOp(_)
        // P311b.2 — MathStyled terminal em materialize_time (math structural).
        | Content::MathStyled(_)
        | Content::Image(_)
        | Content::Divider(_)
        // Passo 156D (ADR-0061 Fase 1 sub-passo 2) — h/v spacing leaves.
        | Content::HSpace(_)
        | Content::VSpace(_)
        // Passo 156E (ADR-0061 Fase 1 sub-passo 3) — pagebreak leaf.
        | Content::Pagebreak(_)
        // Passo 220 (ADR-0078 sub-fase b 4/4) — colbreak leaf.
        | Content::Colbreak(_)
        | Content::Shape { .. }
        // Passo 513 — Curve é leaf não-locatable.
        | Content::Curve(_)
        // P169 (M9): Metadata é terminal — clonar directamente.
        | Content::Metadata(_)
        // P171 (M9): State e StateUpdate são terminais.
        | Content::State(_)
        | Content::StateUpdate(_)
        // P240 (M9d/M7+1): StateDisplay terminal em materialize_time —
        // resolução real via apply_state_displays + layout arm.
        | Content::StateDisplay(_)
        // P241 (M9d/M7+2): CounterDisplayCallback terminal paralelo
        // StateDisplay; resolução real via apply_counter_displays.
        | Content::CounterDisplayCallback(_) => content.clone(),
        // Passo 156C (ADR-0061 Fase 1) — pad / hide containers.
        // Materialize_time desce no body para resolver counters dentro;
        // padding e o invariante "hide" preservam-se.
        Content::Pad(e) => Content::pad(materialize_time(&e.body, intr, location), e.sides),
        Content::Hide(e) => Content::hide(materialize_time(&e.body, intr, location)),
        // Modelo D (Lote 15 P330): Block — recurse no body via struct-update.
        Content::Block(e) => Content::Block(std::sync::Arc::new(
            crate::entities::elements::block::BlockElem {
                body: materialize_time(&e.body, intr, location),
                ..(**e).clone()
            },
        )),
        // Modelo D (Lote 14 P329): Boxed — recurse no body via struct-update.
        Content::Boxed(e) => Content::Boxed(std::sync::Arc::new(
            crate::entities::elements::boxed::BoxedElem {
                body: materialize_time(&e.body, intr, location),
                ..(**e).clone()
            },
        )),
        // Passo 156I (ADR-0061 Fase 2 sub-passo 3) — stack compositivo.
        // Materialize_time em cada child; preservar dir/spacing.
        Content::Stack(e) => {
            let new_children: Vec<Content> = e.children.iter()
                .map(|c| materialize_time(c, intr, location))
                .collect();
            Content::stack(new_children, e.dir, e.spacing)
        },
        // Passo 156J (ADR-0061 Fase 3 sub-passo 1) — repeat.
        // Análogo a Block: descer no body; preservar atributos.
        Content::Repeat(e) => Content::repeat(materialize_time(&e.body, intr, location), e.gap, e.justify),
        // P217 — Columns container: análogo a Repeat/Block; descer
        // no body; count/gutter preservados (Copy primitivos).
        Content::Columns(e) => Content::columns(
            materialize_time(&e.body, intr, location),
            e.count,
            e.gutter,
        ),
        Content::Transform(e) => Content::Transform(std::sync::Arc::new(crate::entities::elements::transform::TransformElem { operation: e.operation, body: materialize_time(&e.body, intr, location) })),
        // Modelo D (Lote 12 P327): Grid — recurse cells+header+footer (struct-update).
        Content::Grid(e) => Content::Grid(std::sync::Arc::new(
            crate::entities::elements::grid::GridElem {
                cells:  e.cells.iter().map(|c| materialize_time(c, intr, location)).collect(),
                header: e.header.as_ref().map(|h| materialize_time(h, intr, location)),
                footer: e.footer.as_ref().map(|f| materialize_time(f, intr, location)),
                ..(**e).clone()
            },
        )),
        // Modelo D (Lote 5 P320): GridHeader/GridFooter via construtor.
        Content::GridHeader(e) => Content::grid_header(materialize_time(&e.body, intr, location), e.repeat),
        Content::GridFooter(e) => Content::grid_footer(materialize_time(&e.body, intr, location), e.repeat),
        // Passo 512 — linhas em grid/table são terminais.
        Content::GridHLine(_) | Content::GridVLine(_) => content.clone(),
        // Modelo D (Lote 12 P327): GridCell — recurse no body via struct-update.
        Content::GridCell(e) => Content::GridCell(std::sync::Arc::new(
            crate::entities::elements::grid_cell::GridCellElem {
                body: materialize_time(&e.body, intr, location),
                ..(**e).clone()
            },
        )),
        // Modelo D (Lote 12 P327): Table — recurse em caption + children.
        Content::Table(e) if e.resolved.is_some() => {
            let mut t=e.as_ref().clone();let mut r=t.resolved.as_ref().unwrap().as_ref().clone();
            r.cells=r.cells.iter().map(|c|materialize_time(c,intr,location)).collect();t.resolved=Some(std::sync::Arc::new(r));
            t.caption=t.caption.as_ref().map(|c|materialize_time(c,intr,location));Content::Table(std::sync::Arc::new(t))
        },
        Content::TableCell(e) if e.standalone.is_some() => {
            let mut c=e.as_ref().clone();let mut r=c.standalone.as_ref().unwrap().as_ref().clone();
            r.visual=materialize_time(&r.visual,intr,location);c.standalone=Some(std::sync::Arc::new(r));Content::TableCell(std::sync::Arc::new(c))
        },
        Content::TableCell(e) if e.resolved.is_some() => {
            let mut c=e.as_ref().clone();let mut r=c.resolved.as_ref().unwrap().as_ref().clone();
            r.visual=materialize_time(&r.visual,intr,location);c.resolved=Some(std::sync::Arc::new(r));Content::TableCell(std::sync::Arc::new(c))
        },
        Content::Table(e) => Content::Table(std::sync::Arc::new(
            crate::entities::elements::table::TableElem {
                caption: e.caption.as_ref().map(|c| materialize_time(c, intr, location)),
                children: e.children.iter().map(|c| materialize_time(c, intr, location)).collect(),
                    hlines: vec![],
                    vlines: vec![],
                ..(**e).clone()
            },
        )),
        // Modelo D (Lote 12 P327): TableCell — recurse no body via struct-update.
        Content::TableCell(e) => Content::TableCell(std::sync::Arc::new(
            crate::entities::elements::table_cell::TableCellElem {
                body: materialize_time(&e.body, intr, location),
                ..(**e).clone()
            },
        )),
        // Modelo D (Lote 5 P320): TableHeader/TableFooter via construtor.
        Content::TableHeader(e) => Content::table_header(materialize_time(&e.body, intr, location), e.repeat),
        Content::TableFooter(e) => Content::table_footer(materialize_time(&e.body, intr, location), e.repeat),
        // Passo 512 — linhas em grid/table são terminais.
        Content::TableHLine(_) | Content::TableVLine(_) => content.clone(),
        // Passo 159A — par acoplado Bibliography + Cite. Recurse em
        // title (Bibliography) ou supplement (Cite); preserva
        // entries/key/style/locale/path (P429).
        Content::Bibliography(e) => Content::Bibliography(Arc::new(
            crate::entities::elements::bibliography::BibliographyElem {
                entries: e.entries.clone(),
                path: e.path.clone(),
                title: e.title.as_ref().map(|t| materialize_time(t, intr, location)),
                style: e.style.clone(),
                locale: e.locale.clone(),
            },
        )),
        Content::Cite(e) => Content::cite_with_style(
            e.key.clone(),
            e.supplement.as_ref().map(|s| materialize_time(s, intr, location)),
            e.form,
            e.style,
        ),
        Content::Align(e) => Content::align(e.alignment, materialize_time(&e.body, intr, location)),
        // P223 — Place refino: preservar float + clearance no materialize_time.
        Content::Place(e) => Content::place(
            e.alignment, e.dx, e.dy, e.scope, e.float, e.clearance,
            materialize_time(&e.body, intr, location),
        ),

        // P397 — Document recursa no title; Asset é terminal.
        Content::Document { path, body, format, title, author, date, keywords } => Content::Document {
            path: path.clone(),
            body: Box::new(materialize_time(body, intr, location)),
            format: format.clone(),
            title: title.as_ref().map(|t| Box::new(materialize_time(t, intr, location))),
            author: author.clone(),
            date: *date,
            keywords: keywords.clone(),
        },
        Content::Asset { .. } => content.clone(),
        Content::PdfAttach(_) => content.clone(),
        Content::PdfArtifact(e) => Content::PdfArtifact(std::sync::Arc::new(
            crate::entities::elements::pdf_artifact::PdfArtifactElem {
                kind: e.kind,
                body: materialize_time(&e.body, intr, location),
            },
        )),

        // Passo 99 (ADR-0038): `Styled` é transparente para materialização de
        // contadores — o body é processado e os estilos preservados.
        Content::Styled(styled) => Content::styled(
            materialize_time(&styled.body, intr, location),
            styled.styles.clone(),
        ),

        // P506: ContextBlock é terminal em materialize_time (o conteúdo real
        // só existe após expansão pós-introspecção).
        Content::ContextBlock(_) => content.clone(),

        Content::OutlineEntry(e) => Content::OutlineEntry(Arc::new(e.remapped(
            materialize_time(&e.element, intr, location),
            e.fill.as_ref().map(|fill| materialize_time(fill, intr, location)),
            e.prefix().map(|prefix| materialize_time(&prefix, intr, location)),
        ))),

        // Lote F-1 (P334): a fronteira dinâmica é **terminal** aqui (clone). A
        // travessia/realização do nó dinâmico (e dos seus filhos) chega em F-2
        // (L0 `f_fronteira_e1.md` §3a.7); em F-1 só existe em fixtures.
        Content::HtmlFrame(e) => Content::HtmlFrame(Arc::new(
            crate::entities::html::HtmlFrameElem {
                body: materialize_time(&e.body, intr, location),
            },
        )),
        Content::Dynamic(_) | Content::HtmlElem(_) => content.clone(),
    }
}

// Atomização (ADR-0109, P383): `compute_labelled` movido para
// `introspect/labelled.rs`; `compute_heading_auto_toc` + `compute_heading_for_toc`
// para `introspect/heading.rs`. O walk chama via `labelled::`/`heading::`.

// P197B helper `compute_figure` ELIMINADO em P190H (M6 categoria
// Figures eliminada). Walk arm Figure ficou puro — sem necessidade
// de calcular figure_number durante walk porque populate_intr arm
// Figure (P191C, gated por is_counted) popula
// `intr.counters["figure:{kind}"]` directamente. Consumers
// (`compute_labelled` Figure arm via `intr.flat_counter_at`,
// Layouter C3 via `figure_number_at_index`) consomem do intr.

#[cfg(test)]
mod figure_grid_carrier_tests {
    use super::*;
    use crate::entities::elements::grid::{GridChildOrder, GridElem};
    use crate::entities::elements::grid_hline::GridHLineElem;
    use crate::entities::elements::grid_vline::GridVLineElem;
    use crate::entities::layout_types::Length;
    use crate::entities::sides::Sides;

    #[test]
    fn figure_v16_temporal_grid_preserves_line_vectors_and_source_descriptors() {
        let horizontal = GridHLineElem {
            start: 0,
            end: Some(2),
            row: 0,
            stroke: None,
            position: "top".into(),
        };
        let vertical = GridVLineElem {
            start: 0,
            end: Some(1),
            col: 0,
            stroke: None,
            position: "left".into(),
        };
        let grid = GridElem {
            columns: vec![],
            rows: vec![],
            cells: vec![Content::text("cell")],
            hlines: vec![horizontal.clone()],
            vlines: vec![vertical.clone()],
            gutter: None,
            align: None,
            inset: Sides::uniform(Length::ZERO),
            header: None,
            footer: None,
            stroke: None,
            fill: None,
            source_order: Some(vec![
                GridChildOrder::HLine(0),
                GridChildOrder::Cell(0),
                GridChildOrder::VLine(0),
            ]),
        };
        let content = Content::Grid(Arc::new(grid.clone()));
        let result = materialize_time(
            &content,
            &TagIntrospector::default(),
            Location::from_raw(1),
        );
        let Content::Grid(result) = result else { panic!("missing Grid") };
        assert_eq!(result.hlines, [horizontal]);
        assert_eq!(result.vlines, [vertical]);
        assert_eq!(result.source_order, grid.source_order);
        assert_eq!(result.cells[0].plain_text(), "cell");
    }
}
