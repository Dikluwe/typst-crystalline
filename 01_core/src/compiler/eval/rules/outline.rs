//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/rules/outline.md
//! @prompt-hash 95192164
//! @layer L1
//! @updated 2026-10-05
//!
//! Comportamento de `eval/rules::outline`.

use super::*;

fn contains_heading_outline(content: &Content) -> bool {
    use crate::entities::elements::outline::OutlineTarget;
    match content {
        Content::Outline(e) => e.target == OutlineTarget::Headings,
        Content::Source(e) => contains_heading_outline(&e.body),
        Content::Located(e) => contains_heading_outline(&e.body),
        Content::Realized(e) => contains_heading_outline(&e.visual),
        Content::ShowRecipe(e) => contains_heading_outline(&e.body),
        Content::Styled(e) => contains_heading_outline(&e.body),
        Content::Sequence(parts) => parts.iter().any(contains_heading_outline),
        Content::Table(e) => {
            let children = if let Some(resolved) = &e.resolved {
                &resolved.cells
            } else {
                e.items.as_ref().unwrap_or(&e.children)
            };
            children.iter().any(contains_heading_outline)
                || (e.resolved.is_none()
                    && e.items.is_none()
                    && e.header.as_ref().is_some_and(contains_heading_outline))
                || (e.resolved.is_none()
                    && e.items.is_none()
                    && e.footer.as_ref().is_some_and(contains_heading_outline))
                || e.caption.as_ref().is_some_and(contains_heading_outline)
        }
        Content::TableCell(e) => {
            let visual = e
                .standalone
                .as_ref()
                .map(|r| &r.visual)
                .or_else(|| e.resolved.as_ref().map(|r| &r.visual))
                .unwrap_or(&e.body);
            contains_heading_outline(visual)
        }
        other => {
            let mut found = false;
            let traversal = other.map_content(&mut |node| {
                if let Content::Outline(e) = node {
                    found |= e.target == OutlineTarget::Headings;
                } else if let Content::Located(e) = node {
                    if let Content::Outline(outline) = &e.body {
                        found |= outline.target == OutlineTarget::Headings;
                    }
                }
                Ok(None)
            });
            // If the proof cannot traverse a carrier, retain the original
            // transforming path so it can report its own error.
            found || traversal.is_err()
        }
    }
}

/// P1406 — substitui cada outline de headings pela sua sequência realizada
/// depois que a introspecção já conhece entries e páginas. A função permanece
/// em L1: L3 fornece apenas Engine/Introspector e orquestra a fase.
pub fn realize_outlines(
    content: Content,
    introspector: &crate::entities::introspector::TagIntrospector,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    if !contains_heading_outline(&content) {
        return Ok(content);
    }
    let mut changed = false;
    let mut realize = |node: &Content| -> SourceResult<Option<Content>> {
        if let Content::Located(e) = node {
            if let Content::Outline(outline) = &e.body {
                if outline.target
                    == crate::entities::elements::outline::OutlineTarget::Headings
                {
                    changed = true;
                    let body =
                        realize_heading_outline(outline, introspector, ctx, engine)?;
                    return Ok(Some(Content::Located(Arc::new(
                        crate::entities::content::LocatedContent {
                            location: e.location,
                            body,
                        },
                    ))));
                }
            }
            return Ok(None);
        }
        let Content::Outline(outline) = node else {
            return Ok(None);
        };
        if outline.target != crate::entities::elements::outline::OutlineTarget::Headings {
            return Ok(None);
        }
        changed = true;
        realize_heading_outline(outline, introspector, ctx, engine).map(Some)
    };
    let transformed = content.map_content(&mut realize)?;
    Ok(if changed { transformed } else { content })
}

fn direct_entry_alias(
    done: &crate::entities::content::RealizedContent,
) -> Option<&Content> {
    let Content::Located(original) = done.original.source_body() else { return None };
    let Content::Located(visual) = done.visual.source_body() else { return None };
    if original.location != visual.location {
        return None;
    }
    let Content::OutlineEntry(_) = original.body.source_body() else { return None };
    let Content::OutlineEntry(_) = visual.body.source_body() else { return None };
    Some(visual.body.source_body())
}

fn default_entry_visual(
    entry: &Content,
    relevant: &[ShowRule],
    label: &crate::entities::label::Label,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    let Content::OutlineEntry(entry_elem) = entry else { unreachable!() };
    let mut scopes = Scopes::new(None);
    let receiver = Value::Content(entry.clone());
    let args = Args::positional(vec![receiver.clone()]);
    let inner = match crate::compiler::stdlib::native_outline_entry_inner(
        ctx,
        &args,
        engine.world,
        engine.current_file,
        &mut scopes,
        engine,
    )? {
        Value::Content(content) => content,
        _ => unreachable!("outline.entry.inner always returns content"),
    };
    let prefix = entry_elem.prefix().map(Value::Content).unwrap_or(Value::None);
    let args = Args::positional(vec![receiver, prefix, Value::Content(inner)]);
    let body = match crate::compiler::stdlib::native_outline_entry_indented(
        ctx,
        &args,
        engine.world,
        engine.current_file,
        &mut scopes,
        engine,
    )? {
        Value::Content(content) => content,
        _ => unreachable!("outline.entry.indented always returns content"),
    };
    let linked = Content::styled(
        Box::new(body),
        Styles::new()
            .push_custom("outline.link.destination", Value::Str(label.0.clone().into())),
    );
    let mut set_chain = StyleChain::empty();
    for rule in relevant {
        if let Transformation::Style(styles) = &rule.transform {
            set_chain = set_chain.push(styles.delta().clone());
        }
    }
    let delta = set_chain.collapse();
    Ok(if delta.is_empty() {
        linked
    } else {
        Content::styled(Box::new(linked), Styles::from_delta(delta))
    })
}

pub(crate) fn realize_heading_outline(
    outline: &crate::entities::elements::outline::OutlineElem,
    introspector: &crate::entities::introspector::TagIntrospector,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    use crate::entities::layout_types::Length;

    let previous_styles = engine.styles.clone();
    let previous_rules = Arc::clone(&*engine.show_rules);
    let previous_context = ctx.in_context;
    let previous_intr = std::mem::replace(&mut ctx.introspector, introspector.clone());

    let parent_styles = Styles::new()
        .push_custom(
            "outline.parent.indent",
            match &outline.indent {
                crate::entities::elements::outline::OutlineIndent::Auto => Value::Auto,
                crate::entities::elements::outline::OutlineIndent::Bool(value) => {
                    Value::Bool(*value)
                }
                crate::entities::elements::outline::OutlineIndent::Length(value) => {
                    Value::Length(*value)
                }
                crate::entities::elements::outline::OutlineIndent::Function(value) => {
                    Value::Func(value.clone())
                }
            },
        )
        .push_custom("outline.parent.key", Value::Int(outline.parent_key()))
        .push_custom("heading.numbering", Value::Bool(false))
        .push_custom("par.justify", Value::Bool(false));
    *engine.styles = StyleChain::default_chain()
        .push_styles(outline.captured_styles())
        .push_styles(&parent_styles);
    *engine.show_rules = Arc::from(outline.show_rules().to_vec());
    ctx.in_context = true;
    let previous_read_styles = ctx.replace_context_read_styles(engine.styles.clone());
    let previous_read_rules = ctx.replace_context_read_rules(
        Arc::clone(engine.show_rules),
        engine.active_guards.clone(),
    );
    let previous_read_file = ctx.replace_context_read_file(Some(engine.current_file));

    let result = (|| {
        ctx.observe_context_read(
            crate::compiler::eval::ContextReadRequest::OutlineInputs {
                parent_key: outline.parent_key(),
                depth: outline.depth,
            },
            Span::detached(),
            Ok(crate::compiler::eval::context_reads::outline_inputs(
                introspector,
                outline.parent_key(),
                outline.depth,
            )),
        )?;
        let title_heading = |body| {
            Content::styled(
                Content::Heading(Arc::new(
                    crate::entities::elements::heading::HeadingElem::new_with_outlined(
                        1, body, false,
                    ),
                )),
                parent_styles.clone(),
            )
        };
        let mut parts = Vec::new();
        if !outline.title_was_set() {
            let title = crate::compiler::lang::outline_title::outline_title_for_lang(
                engine.styles.lang().as_ref(),
            );
            parts.push(title_heading(Content::text(title)));
        } else if let Some(title) = &outline.title {
            parts.push(title_heading(title.clone()));
        }

        for (((label, number, _body, level), source_location), source) in introspector
            .headings_for_toc
            .iter()
            .zip(&introspector.outlined_heading_locations)
            .filter_map(|(heading, location)| {
                introspector
                    .elements
                    .get(location)
                    .map(|source| ((heading, location), source))
            })
        {
            if *level > outline.depth {
                continue;
            }
            let fill = match engine.styles.custom("outline.entry.fill") {
                Some(Value::Content(content)) => Some(content.clone()),
                Some(Value::None) => None,
                _ => Some(Content::repeat(
                    Content::text("."),
                    Some(Length::em(0.15)),
                    true,
                )),
            };
            let prefix = number.as_ref().map(|value| Content::text(value.as_str()));
            let entry = Content::realized_outline_entry(
                *level,
                source.content().clone(),
                fill,
                prefix,
                *source_location,
            );
            let relevant: Vec<ShowRule> = outline
                .show_rules()
                .iter()
                .filter(|rule| {
                    selector_matching::is_node_rule(&rule.selector)
                        && selector_matching::selector_matches(&entry, &rule.selector)
                })
                .cloned()
                .collect();
            let has_transform = relevant
                .iter()
                .any(|rule| !matches!(rule.transform, Transformation::Style(_)));
            let realized = if has_transform {
                let previous_realizing = ctx.realizing_show_rules;
                ctx.realizing_show_rules = true;
                let realized = apply_show_rules(entry, &relevant, ctx, engine);
                ctx.realizing_show_rules = previous_realizing;
                let realized = realized?;
                // Only a direct same-occurrence Entry alias needs its native
                // default visual; do not classify arbitrary recipe products.
                let alias = if let Content::Realized(done) = &realized {
                    direct_entry_alias(done)
                        .map(|entry| (done.original.clone(), entry.clone()))
                } else {
                    None
                };
                if let Some((original, entry)) = alias {
                    Content::Realized(Arc::new(
                        crate::entities::content::RealizedContent {
                            original,
                            visual: default_entry_visual(
                                &entry, &relevant, label, ctx, engine,
                            )?,
                        },
                    ))
                } else {
                    realized
                }
            } else {
                let visual = default_entry_visual(&entry, &relevant, label, ctx, engine)?;
                Content::Realized(Arc::new(crate::entities::content::RealizedContent {
                    original: crate::compiler::realize::locate(entry, ctx),
                    visual,
                }))
            };
            parts.push(Content::styled(Box::new(realized), parent_styles.clone()));
        }
        let body = crate::compiler::realize::finish_outline_insertion(
            Content::sequence(parts),
            ctx,
            engine,
        )?;
        Ok(Content::Outline(Arc::new(outline.with_realized_body(body))))
    })();

    ctx.replace_context_read_styles(previous_read_styles);
    ctx.replace_context_read_rules(previous_read_rules.0, previous_read_rules.1);
    ctx.replace_context_read_file(previous_read_file);
    *engine.styles = previous_styles;
    *engine.show_rules = previous_rules;
    ctx.in_context = previous_context;
    ctx.introspector = previous_intr;
    result
}

#[cfg(test)]
mod scan_tests {
    use super::*;
    use crate::entities::content::{LocatedContent, RealizedContent};
    use crate::entities::elements::outline::OutlineTarget;
    use crate::entities::location::Location;

    #[test]
    fn borrowed_scan_detects_only_visible_heading_outlines() {
        let heading = Content::outline();
        let Content::Outline(elem) = &heading else { unreachable!() };
        let mut other = elem.as_ref().clone();
        other.target = OutlineTarget::Figures;
        let other = Content::Outline(Arc::new(other));
        assert!(!contains_heading_outline(&Content::table(
            vec![],
            vec![],
            vec![other.clone()]
        )));
        assert!(contains_heading_outline(&Content::table(
            vec![],
            vec![],
            vec![heading.clone()]
        )));
        let located = Content::Located(Arc::new(LocatedContent {
            location: Location::from_raw(17),
            body: heading.clone(),
        }));
        assert!(contains_heading_outline(&located));
        let hidden = Content::Realized(Arc::new(RealizedContent {
            original: heading,
            visual: other,
        }));
        assert!(!contains_heading_outline(&hidden));
    }

    struct OutlineWorld {
        source: crate::entities::source::Source,
        library: crate::entities::world_types::Library,
        book: crate::entities::font_book::FontBook,
    }
    impl crate::contracts::world::World for OutlineWorld {
        fn library(&self) -> &crate::entities::world_types::Library {
            &self.library
        }
        fn book(&self) -> &crate::entities::font_book::FontBook {
            &self.book
        }
        fn main(&self) -> crate::entities::file_id::FileId {
            self.source.id()
        }
        fn source(
            &self,
            _: crate::entities::file_id::FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::source::Source>
        {
            Ok(self.source.clone())
        }
        fn file(
            &self,
            _: crate::entities::file_id::FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::world_types::Bytes>
        {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<crate::entities::world_types::Font> {
            None
        }
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<crate::entities::world_types::Datetime> {
            None
        }
    }
    fn with_outline_engine(run: impl FnOnce(&mut EvalContext, &mut Engine<'_>)) {
        use crate::contracts::world::World;
        use comemo::Track;
        let world = OutlineWorld {
            source: crate::entities::source::Source::detached("")
                .expect("valid Source test fixture"),
            library: crate::entities::world_types::Library::new(),
            book: crate::entities::font_book::FontBook::new(),
        };
        let route = crate::entities::world_types::Route::root();
        let mut styles = StyleChain::default_chain();
        let mut rules = Arc::from([]);
        let mut guards = Vec::new();
        let mut sink = crate::entities::sink::Sink::new();
        let mut tracked = sink.track_mut();
        let mut engine = Engine {
            world: &world,
            font_metrics: &crate::compiler::layout::FixedMetrics,
            route: route.track(),
            styles: &mut styles,
            show_rules: &mut rules,
            active_guards: &mut guards,
            current_file: world.main(),
            sink: &mut tracked,
        };
        run(&mut EvalContext::new(), &mut engine);
    }
    fn title_headings(
        body: &Content,
        chain: &StyleChain,
        result: &mut Vec<(
            Arc<crate::entities::elements::heading::HeadingElem>,
            StyleChain,
        )>,
    ) {
        match body {
            Content::Heading(heading) => result.push((heading.clone(), chain.clone())),
            Content::Styled(styled) => {
                title_headings(&styled.body, &chain.push_styles(&styled.styles), result)
            }
            Content::Located(located) => title_headings(&located.body, chain, result),
            Content::Source(source) => title_headings(&source.body, chain, result),
            Content::Realized(realized) => {
                title_headings(&realized.original, chain, result)
            }
            Content::Sequence(parts) => {
                for part in parts.iter() {
                    title_headings(part, chain, result);
                }
            }
            // Only the source spine above contributes synthesized title Headings.
            // Other domain variants are outside this narrow observer. Keep the
            // partition exhaustive so future variants require a decision.
            Content::ShowRecipe(..)
            | Content::Empty
            | Content::Text(..)
            | Content::Space
            | Content::SpaceFromNewline
            | Content::Parbreak
            | Content::HtmlElem(..)
            | Content::HtmlFrame(..)
            | Content::Par { .. }
            | Content::Title(..)
            | Content::Strong(..)
            | Content::Emph(..)
            | Content::Raw(..)
            | Content::ListItem(..)
            | Content::EnumItem(..)
            | Content::Link(..)
            | Content::Equation(..)
            | Content::MathSequence(..)
            | Content::Symbol(..)
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
            | Content::Dynamic(..) => {}
        }
    }
    fn actual_outline_body(content: &Content) -> &Content {
        let Content::Outline(outline) = content else { panic!("realized Outline") };
        outline.realized_body().expect("actual synthesized body")
    }

    #[test]
    fn physical_page_outline_title_is_heading_without_self_entry() {
        use crate::entities::elements::outline::{OutlineElem, OutlineIndent};
        with_outline_engine(|ctx, engine| {
            let rich = Content::strong(Content::text("Custom contents"));
            for title in [None, Some(rich.clone())] {
                let outline = OutlineElem::with_target_and_title_presence(
                    title.clone(),
                    title.is_some(),
                    3,
                    OutlineIndent::Auto,
                    OutlineTarget::Headings,
                );
                let realized = realize_heading_outline(
                    &outline,
                    &crate::entities::introspector::TagIntrospector::empty(),
                    ctx,
                    engine,
                )
                .unwrap();
                let body = actual_outline_body(&realized);
                let mut headings = Vec::new();
                title_headings(body, engine.styles, &mut headings);
                assert_eq!(headings.len(), 1, "one actual synthesized title");
                assert_eq!(headings[0].0.level, 1);
                if let Some(title) = &title {
                    let Content::Strong(expected) = title else { unreachable!() };
                    let Content::Strong(actual) = &headings[0].0.body else {
                        panic!("explicit title lost Strong morphology")
                    };
                    assert!(Arc::ptr_eq(actual, expected), "title body is preserved");
                }
                assert!(!headings[0].0.outlined, "title must not include itself");
                assert!(!headings[0].0.is_bookmarked());
                let indexed = crate::compiler::introspect::introspect(body);
                assert!(indexed.headings_for_toc.is_empty());
                assert!(indexed.outlined_heading_locations.is_empty());
            }
        });
    }

    #[test]
    fn physical_page_outline_title_disables_inherited_heading_numbering() {
        use crate::entities::elements::outline::{OutlineElem, OutlineIndent};
        with_outline_engine(|ctx, engine| {
            let numbered = Styles::new()
                .push_custom("heading.numbering", Value::Bool(true))
                .push_custom("heading.numbering.pattern", Value::Str("I.".into()));
            *engine.styles = engine.styles.push_styles(&numbered);
            let original_styles = engine.styles.clone();
            let outline = OutlineElem::new(None, 3, OutlineIndent::Auto)
                .with_realization_context(Arc::from([]), numbered);
            let realized = realize_heading_outline(
                &outline,
                &crate::entities::introspector::TagIntrospector::empty(),
                ctx,
                engine,
            )
            .unwrap();
            let mut headings = Vec::new();
            title_headings(actual_outline_body(&realized), engine.styles, &mut headings);
            assert_eq!(headings.len(), 1);
            assert!(
                matches!(
                    headings[0].1.custom("heading.numbering"),
                    Some(Value::Bool(false))
                ),
                "synthesized title must suppress inherited Heading numbering"
            );
            assert!(matches!(
                engine.styles.custom("heading.numbering"),
                Some(Value::Bool(true))
            ));
            assert!(std::ptr::eq(
                engine.styles.raw_deltas().next().unwrap(),
                original_styles.raw_deltas().next().unwrap()
            ));
        });
    }

    #[test]
    fn readiness_outline_justification_parent_is_nonjustified_and_restores_caller() {
        use crate::entities::elements::outline::{OutlineElem, OutlineIndent};
        with_outline_engine(|ctx, engine| {
            for justified in [true, false] {
                let captured =
                    Styles::new().push_custom("par.justify", Value::Bool(justified));
                *engine.styles = engine.styles.push_styles(&captured);
                let previous = engine.styles.clone();
                let intr = crate::compiler::introspect::introspect(&Content::heading(
                    1,
                    Content::text("Entry"),
                ));
                assert_eq!(intr.headings_for_toc.len(), 1);
                let outline = OutlineElem::new(None, 3, OutlineIndent::Auto)
                    .with_realization_context(Arc::from([]), captured);
                let realized = realize_heading_outline(&outline, &intr, ctx, engine)
                    .expect("the actual synthesized title and entry");
                let Content::Sequence(parts) = actual_outline_body(&realized) else {
                    panic!("title plus actual source entry must remain a sequence");
                };
                assert_eq!(parts.len(), 2, "one title and one original source entry");
                for part in parts.iter() {
                    let Content::Styled(styled) = part else {
                        panic!("each actual part carries the parent defaults");
                    };
                    let effective = previous.push_styles(&styled.styles);
                    assert_eq!(effective.custom("par.justify"), Some(&Value::Bool(false)),
                        "an Outline part inherited the surrounding paragraph justification");
                }
                assert_eq!(
                    engine.styles.custom("par.justify"),
                    Some(&Value::Bool(justified))
                );
                assert!(std::ptr::eq(
                    engine.styles.raw_deltas().next().unwrap(),
                    previous.raw_deltas().next().unwrap(),
                ));
            }
        });
    }

    #[test]
    fn readiness_outline_own_show_keeps_original_entry_source() {
        use crate::entities::elements::outline::{OutlineElem, OutlineIndent};
        with_outline_engine(|ctx, engine| {
            for previous_realizing in [false, true] {
                ctx.realizing_show_rules = previous_realizing;
                let rule = ShowRule {
                    id: 0,
                    selector: Selector::NativeElement(
                        crate::entities::func::Func::native(
                            "outline.entry",
                            crate::compiler::stdlib::native_outline_entry,
                        ),
                    ),
                    transform: Transformation::Func(crate::entities::func::Func::native(
                        "entry-source-proof",
                        |_, _, _, _| Ok(Value::Content(Content::text("ENTRY"))),
                    )),
                };
                let outline = OutlineElem::with_target_and_title_presence(
                    None,
                    true,
                    3,
                    OutlineIndent::Auto,
                    OutlineTarget::Headings,
                )
                .with_realization_context(Arc::from([rule]), Styles::new());
                let input = crate::compiler::introspect::introspect(&Content::heading(
                    1,
                    Content::text("Alpha"),
                ));
                let realized = realize_heading_outline(&outline, &input, ctx, engine)
                    .expect("actual Entry own show");
                let body = actual_outline_body(&realized);
                assert_eq!(body.plain_text(), "ENTRY");
                let mut originals = 0;
                body.map_content(&mut |node| {
                    if let Content::Realized(done) = node {
                        let mut original = done.original.source_body();
                        while let Content::Located(located) = original {
                            original = located.body.source_body();
                        }
                        if let Content::OutlineEntry(entry) = original {
                            originals += 1;
                            assert_eq!(entry.body().unwrap().plain_text(), "Alpha");
                        }
                    }
                    Ok(None)
                })
                .expect("inspect retained carrier originals");
                assert_eq!(
                    originals, 1,
                    "actual original Entry must reach source planning"
                );
                assert_eq!(
                    ctx.realizing_show_rules, previous_realizing,
                    "restore caller realization state"
                );
            }
        });
    }

    #[test]
    fn physical_page_outline_explicit_none_has_no_title_and_restores_environment() {
        use crate::entities::elements::outline::{OutlineElem, OutlineIndent};
        with_outline_engine(|ctx, engine| {
            *engine.styles = engine.styles.push_styles(
                &Styles::new().push_custom("outline.test.caller", Value::Int(29)),
            );
            let previous_styles = engine.styles.clone();
            let previous_rules = engine.show_rules.clone();
            ctx.in_context = false;
            let outline = OutlineElem::with_target_and_title_presence(
                None,
                true,
                3,
                OutlineIndent::Auto,
                OutlineTarget::Headings,
            );
            let realized = realize_heading_outline(
                &outline,
                &crate::entities::introspector::TagIntrospector::empty(),
                ctx,
                engine,
            )
            .unwrap();
            assert!(matches!(actual_outline_body(&realized), Content::Empty));
            assert!(std::ptr::eq(
                engine.styles.raw_deltas().next().unwrap(),
                previous_styles.raw_deltas().next().unwrap()
            ));
            assert!(Arc::ptr_eq(engine.show_rules, &previous_rules));
            assert!(!ctx.in_context);
        });
    }

    #[test]
    fn physical_page_outline_empty_inputs_record_actual_dependency() {
        use crate::entities::elements::outline::{OutlineElem, OutlineIndent};
        with_outline_engine(|ctx, engine| {
            let empty = crate::entities::introspector::TagIntrospector::empty();
            let outline = OutlineElem::with_target_and_title_presence(
                None,
                true,
                3,
                OutlineIndent::Auto,
                OutlineTarget::Headings,
            );
            realize_heading_outline(&outline, &empty, ctx, engine).unwrap();
            assert!(
                ctx.context_reads_valid_for(&empty, engine).unwrap(),
                "the exact empty input must remain valid"
            );
            let changed = crate::compiler::introspect::introspect(&Content::heading(
                1,
                Content::text("New section"),
            ));
            assert_eq!(changed.headings_for_toc.len(), 1, "actual changed input");
            assert!(
                !ctx.context_reads_valid_for(&changed, engine).unwrap(),
                "an empty Outline still depends on the actual source-heading input"
            );
        });
    }
}
