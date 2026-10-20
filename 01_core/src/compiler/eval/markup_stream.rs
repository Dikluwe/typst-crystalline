//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/markup_stream.md
//! @prompt-hash 32fd9b29
//! @layer L1
//! @updated 2026-10-07

use super::*;

pub(crate) fn eval_markup(
    node: &SyntaxNode,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    eval_markup_impl(node, scopes, ctx, engine, true)
}

/// Um ContentBlock ainda é um valor: conserva os espaços brutos até entrar
/// num fluxo de markup. Os demais corpos são realizados no próprio escopo.
pub(super) fn eval_markup_impl(
    node: &SyntaxNode,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    normalize_spaces: bool,
) -> SourceResult<Value> {
    let frame = rules::textual::enter_scope(ctx, engine);
    let result = eval_markup_in_scope(node, scopes, ctx, engine, normalize_spaces);
    rules::textual::leave_scope(ctx, frame);
    result
}

fn eval_markup_in_scope(
    node: &SyntaxNode,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    normalize_spaces: bool,
) -> SourceResult<Value> {
    let mut parts: Vec<Content> = Vec::new();
    let mut sequence_scopes = Vec::new();
    if ctx.apply_show_rules {
        if let Some(scope) = crate::compiler::realize::sequences::snapshot(engine) {
            sequence_scopes.push((0, scope));
        }
    }

    // Set/show iniciam uma cauda lexical, mesmo quando não mudam estilos.
    let mut label_scope_start = 0;
    // P1494: snapshots por ocorrência, sem retroatividade de regras textuais.
    let mut textual_snapshots: Vec<rules::textual::TextualSnapshot> = Vec::new();

    // β1 fatia 1 (P339, §3a.8) + F-item3 (P368, §3a.11): snapshot do **delta
    // completo** à entrada deste corpo, para detectar um `#set` local (numbering,
    // prop de elemento de usuário, ou estilo tipado como `text.bold`) e embrulhar
    // o resto do escopo léxico num `Content::Styled` (transporte aditivo
    // `StyledElem`-scoped). Generaliza o antigo snapshot só-de-custom.
    // P373 (§3a.14): correção do transporte — **wraps aninhados por escopo léxico**
    // (espelho do `styled_with_map` por-`#set` do vanilla, recursivo). Rastreia cada
    // **fronteira de `#set`** (mudança do delta vs o estado corrente) com o delta
    // QUE ESTE `#set` introduziu; no fim, fold de dentro para fora produz o
    // aninhamento. Conserta o bug single-wrap-final-collapse (P372): `#set`
    // sequenciais da mesma chave deixam de colapsar para o valor final.
    // P431 (DEBT-50): o delta agora inclui origem (`from_strong`/`from_emph`),
    // propagada pelo `StyleDelta` e embrulhada via `diff_styles`.
    let snap_delta = engine.styles.collapse();
    let mut running_delta = snap_delta.clone();
    let mut boundaries: Vec<(usize, crate::entities::style::Styles)> = Vec::new();

    let content_block_origin = node
        .span()
        .id()
        .and_then(|id| engine.world.source(id).ok())
        .and_then(|source| {
            crate::entities::syntax_node::LinkedNode::new(source.root())
                .find(node.span())
                .and_then(|node| {
                    node.parent()
                        .filter(|parent| parent.kind() == SyntaxKind::ContentBlock)
                        .map(|parent| parent.span())
                })
        });
    let capture_origin = content_block_origin.is_some();
    let sequence_origin = content_block_origin.unwrap_or(node.span());
    for child in node.children() {
        let snapshot = rules::textual::TextualSnapshot::capture(engine, ctx);
        match child.kind() {
            SyntaxKind::Text => {
                // F-5b fatia 2 (P373): o render não é mais assado no node — viaja
                // na chain (custom), resolvido no layout.
                let text_node = Content::Text(child.text().as_str().into());
                append_markup_sequence(
                    &mut parts,
                    {
                        let text = rules::textual::intercept_text_element(
                            text_node, ctx, engine,
                        )?;
                        if capture_origin {
                            text.with_source_span(child.span())
                        } else {
                            text
                        }
                    },
                    capture_origin,
                );
            }
            SyntaxKind::SmartQuote => {
                let quote = Content::smartquote(child.text().as_str() == "\"");
                append_markup_sequence(
                    &mut parts,
                    rules::intercept_content(quote, ctx, engine)?,
                    capture_origin,
                );
            }
            SyntaxKind::Space => parts.push(if child.text().as_str().contains('\n') {
                Content::SpaceFromNewline
            } else {
                Content::Space
            }),
            // P622: quebra de parágrafo semântica — distinta de Space.
            SyntaxKind::Parbreak => parts.push(Content::Parbreak),
            k if k.is_trivia() => continue,
            SyntaxKind::Label => {
                if let Some(label_ast) = child.cast::<AstLabel<'_>>() {
                    attach_label(
                        &mut parts[label_scope_start..],
                        label_ast.get().to_string(),
                        child.span(),
                        ctx,
                        engine,
                    )?;
                }
            }
            _ => {
                if let Some(expr) = Expr::from_untyped(child) {
                    let value = eval_expr(expr, scopes, ctx, engine)?;
                    if let Value::Label(label) = value {
                        attach_label(
                            &mut parts[label_scope_start..],
                            label.0,
                            child.span(),
                            ctx,
                            engine,
                        )?;
                    } else {
                        let boolean_display = matches!(value, Value::Bool(_));
                        let display = match value {
                            Value::None => (!matches!(
                                child.kind(),
                                SyntaxKind::SetRule | SyntaxKind::ShowRule
                            ))
                            .then(|| Content::Empty.with_source_span(child.span())),
                            Value::Bool(..)
                            | Value::Int(..)
                            | Value::Float(..)
                            | Value::Str(..)
                            | Value::Array(..)
                            | Value::Dict(..)
                            | Value::Module(..)
                            | Value::Datetime(..)
                            | Value::Func(..)
                            | Value::Content(..)
                            | Value::LocatedContent(..)
                            | Value::Auto
                            | Value::Length(..)
                            | Value::Relative(..)
                            | Value::Ratio(..)
                            | Value::Angle(..)
                            | Value::Color(..)
                            | Value::Stroke(..)
                            | Value::Fraction(..)
                            | Value::Align(..)
                            | Value::Location(..)
                            | Value::Gradient(..)
                            | Value::Regex(..)
                            | Value::Tiling(..)
                            | Value::Bytes(..)
                            | Value::Decimal(..)
                            | Value::Duration(..)
                            | Value::Version(..)
                            | Value::Selector(..)
                            | Value::Symbol(..)
                            | Value::Args(..)
                            | Value::State(..)
                            | Value::Counter(..)
                            | Value::Label(..)
                            | Value::Dir(..)
                            | Value::Path(..)
                            | Value::Type(..)
                            | Value::SpotColorant(..) => value_to_display_content(value),
                        };
                        if let Some(content) = display {
                            let content = if boolean_display {
                                rules::intercept_content(content, ctx, engine)?
                            } else {
                                content
                            };
                            let content =
                                crate::compiler::realize::capture_table_insertion(
                                    content, ctx, engine,
                                )?;
                            let content =
                                if ctx.realizing_show_rules && !ctx.defer_show_rules {
                                    rules::apply::realize_inserted_occurrences(
                                        content, ctx, engine,
                                    )
                                    .map_err(
                                        |mut errors| {
                                            for error in &mut errors {
                                                if error.span.is_detached() {
                                                    error.span = child.span();
                                                }
                                            }
                                            errors
                                        },
                                    )?
                                } else {
                                    content
                                };
                            let content = crate::compiler::realize::sequences::capture_at(
                                content,
                                child.span(),
                                ctx,
                                engine,
                            );
                            let content = if ctx.defer_show_rules {
                                rules::textual::capture_local_output(content, ctx, engine)
                            } else {
                                content
                            };
                            append_markup_sequence(&mut parts, content, capture_origin);
                        }
                    }
                }
            }
        }
        if matches!(child.kind(), SyntaxKind::SetRule | SyntaxKind::ShowRule) {
            label_scope_start = parts.len();
            if ctx.apply_show_rules {
                if let Some(scope) = crate::compiler::realize::sequences::snapshot(engine)
                {
                    sequence_scopes.push((parts.len(), scope));
                }
            }
        }
        textual_snapshots.resize(parts.len(), snapshot);

        // P373/P431: fronteira de `#set` — o delta completo mudou vs o estado
        // corrente. Regista `(parts.len(), Styles do delta introduzido)` e actualiza
        // o corrente. (Só o `#set` muta `engine.styles` neste loop; o `#set` não
        // produz `part`, logo `parts.len()` é o início da cauda que este `#set`
        // escopa.)
        let cur = engine.styles.collapse();
        if cur != running_delta {
            let styles = cur.diff_styles(&running_delta);
            boundaries.push((parts.len(), styles));
            running_delta = cur;
        }
    }

    // P373: fold de dentro para fora — cada fronteira embrulha a sua cauda
    // (`parts[idx..]`) no delta do seu `#set`, produzindo o aninhamento
    // `Styled(A, [X, Styled(B, [Y])])` (escopo léxico, fiel ao vanilla). `#set`
    // único → 1 fronteira → 1 wrap = comportamento de antes (content-preserving).
    let mut consumed = rules::textual::realize_textual_parts(
        &mut parts,
        &textual_snapshots,
        &boundaries,
        ctx,
        engine,
        normalize_spaces,
    )?;
    let mut styles = boundaries.into_iter().rev().peekable();
    let mut sequences = sequence_scopes.into_iter().rev().peekable();
    while styles.peek().is_some() || sequences.peek().is_some() {
        let take_sequence = sequences.peek().is_some_and(|(index, _)| {
            styles.peek().is_none_or(|(style_index, _)| index >= style_index)
        });
        if take_sequence {
            let (index, scope) = sequences.next().unwrap();
            let tail = Content::sequence(
                parts
                    .split_off(index)
                    .into_iter()
                    .zip(consumed.split_off(index))
                    .filter_map(|(part, consumed)| (!consumed).then_some(part))
                    .filter(|part| !matches!(part, Content::Empty))
                    .collect(),
            );
            parts.push(crate::compiler::realize::sequences::capture_scope(
                tail,
                scope,
                sequence_origin,
                ctx,
            ));
            consumed.push(false);
        } else {
            let (index, delta) = styles.next().unwrap();
            let tail = parts
                .split_off(index)
                .into_iter()
                .zip(consumed.split_off(index))
                .filter_map(|(part, consumed)| (!consumed).then_some(part))
                .collect();
            let delta = delta.push_custom("eval.flow.scope", Value::Bool(true));
            parts.push(Content::styled(Box::new(Content::sequence(tail)), delta));
            consumed.push(false);
        }
    }

    // P863: as show rules de elemento (NodeKind) são interceptadas nos pontos
    // de construção do conteúdo, mas os parágrafos são sintetizados a partir de
    // vários nós de texto/`Space`/`Parbreak`. Aplica-se apenas as regras
    // `NodeKind::Par` ao fluxo montado, para que `#show par: ...` possa realizar
    // e transformar os parágrafos sem re-disparar regras textuais já aplicadas
    // ao grupo normalizado acima.
    Ok(Value::Content(rules::intercept_paragraphs(
        Content::sequence(
            parts
                .into_iter()
                .zip(consumed)
                .filter_map(|(part, consumed)| (!consumed).then_some(part))
                .collect(),
        ),
        ctx,
        engine,
    )?))
}

/// Labels literais e valores Label têm a mesma associação no fluxo local.
/// O corpo já passou pela intercepção própria; só aplicar regras de label.
fn attach_label(
    parts: &mut [Content],
    name: String,
    span: Span,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<()> {
    let target = parts.iter_mut().rev().find(|part| {
        !matches!(
            part.source_body(),
            Content::Space
                | Content::SpaceFromNewline
                | Content::Parbreak
                | Content::Empty
        )
    });
    if let Some(target) = target {
        let body = std::mem::replace(target, Content::Empty);
        let labelled = Content::label_auto(name, body);
        *target = rules::intercept_labelled(labelled, ctx, engine)?;
    } else {
        engine.sink.warn_note(
            span,
            &format!("label `<{name}>` is not attached to anything"),
            "",
        );
    }
    Ok(())
}

/// A value keeps its immediate interpolated children; the realized flow
/// expands sequences only after its causal capture has had access to them.
fn append_markup_sequence(
    parts: &mut Vec<Content>,
    content: Content,
    preserve_source: bool,
) {
    if preserve_source {
        parts.push(content);
        return;
    }
    match content {
        Content::Source(e) if matches!(e.body.source_body(), Content::Ref(_)) => {
            parts.push(Content::Source(e));
        }
        Content::Source(e) => append_markup_sequence(parts, e.body.clone(), false),
        Content::Sequence(children) => {
            for child in children.iter().cloned() {
                append_markup_sequence(parts, child, false);
            }
        }
        Content::ShowRecipe(..)
        | Content::Realized(..)
        | Content::Located(..)
        | Content::Empty
        | Content::Text(..)
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
        | Content::HtmlElem(..)
        | Content::HtmlFrame(..)
        | Content::Par { .. }
        | Content::Heading(..)
        | Content::Title(..)
        | Content::Strong(..)
        | Content::Emph(..)
        | Content::Raw(..)
        | Content::ListItem(..)
        | Content::EnumItem(..)
        | Content::Link(..)
        | Content::Equation(..)
        | Content::MathSequence(..)
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
        | Content::Styled(..)
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
        | Content::Repeat(..)
        | Content::Columns(..)
        | Content::Metadata(..)
        | Content::State(..)
        | Content::StateUpdate(..)
        | Content::StateDisplay(..)
        | Content::CounterDisplayCallback(..)
        | Content::ContextBlock(..)
        | Content::Dynamic(..)
        | Content::TableCell(..)
        | Content::Table(..) => parts.push(content),
    }
}

/// Display a value at markup, lexical code-tail and math insertion boundaries.
/// Markup handles its own None/Label semantics before this shared adapter.
/// Residual repr presentation is explicit pending rich-value display completion.
pub(crate) fn value_to_display_content(value: Value) -> Option<Content> {
    match value {
        Value::Content(content) => Some(content),
        Value::LocatedContent(content, _) => Some(content.into_content()),
        Value::Str(text) => Some(Content::Text(text)),
        Value::Auto => Some(Content::raw("auto", Some("typc".into()), false)),
        Value::Bool(value) => Some(Content::raw(
            if value { "true" } else { "false" },
            Some("typc".into()),
            false,
        )),
        Value::Symbol(symbol) => Some(Content::Text(symbol.value)),
        Value::State(state) => {
            Some(Content::state(state.key.to_string(), state.init.as_ref().clone()))
        }
        Value::Counter(_) | Value::None => None,
        Value::Version(version) => {
            let text = version.to_string();
            if text.is_empty() {
                None
            } else {
                Some(Content::Text(text.into()))
            }
        }
        // Numeric display differs from constructor-form repr.
        Value::Float(value) => Some(Content::Text(format!("{value}").into())),
        Value::Decimal(value) => Some(Content::Text(value.to_string().into())),
        // Preserve this bounded legacy path; it does not certify rich display parity.
        other @ (Value::Int(..)
        | Value::Array(..)
        | Value::Dict(..)
        | Value::Module(..)
        | Value::Datetime(..)
        | Value::Func(..)
        | Value::Length(..)
        | Value::Relative(..)
        | Value::Ratio(..)
        | Value::Angle(..)
        | Value::Color(..)
        | Value::Stroke(..)
        | Value::Fraction(..)
        | Value::Align(..)
        | Value::Location(..)
        | Value::Gradient(..)
        | Value::Regex(..)
        | Value::Tiling(..)
        | Value::Bytes(..)
        | Value::Duration(..)
        | Value::Selector(..)
        | Value::Args(..)
        | Value::Label(..)
        | Value::Dir(..)
        | Value::Path(..)
        | Value::Type(..)
        | Value::SpotColorant(..)) => {
            let text = crate::compiler::eval::repr::repr_value(&other);
            if text.is_empty() {
                None
            } else {
                Some(Content::Text(text.into()))
            }
        }
    }
}

/// Avalia o corpo de um nó de markup como Content.
pub(super) fn eval_markup_body(
    node: &SyntaxNode,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    match eval_markup(node, scopes, ctx, engine)? {
        Value::Content(c) => Ok(c),
        _ => Ok(Content::Empty),
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn modulo_compila_e_carrega() {}
}

#[cfg(test)]
mod sequence_r3_markup_tests {
    use super::*;

    #[test]
    fn sequence_r3_markup_value_keeps_source_sequence_while_flow_expands_it() {
        let id = crate::entities::file_id::FileId::from_raw(
            std::num::NonZeroU16::new(1).unwrap(),
        );
        let span = Span::from_range(id, 10..20);
        let inner = Content::sequence(vec![Content::text("A"), Content::text("B")])
            .with_source_span(span);
        let mut value = Vec::new();
        append_markup_sequence(&mut value, inner.clone(), true);
        assert_eq!(value.len(), 1);
        assert!(matches!(value[0].source_body(), Content::Sequence(_)));
        assert_eq!(value[0].source_span(), span);
        let mut flow = Vec::new();
        append_markup_sequence(&mut flow, inner.clone(), false);
        assert_eq!(flow.len(), 2);
        assert_eq!(flow[0].plain_text(), "A");
        assert_eq!(flow[1].plain_text(), "B");
        assert!(matches!(inner.source_body(), Content::Sequence(_)));
    }
}

#[cfg(test)]
mod v16_c_display_tests {
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
            source: Source::new(id, text.into()),
        }
    }

    fn rich() -> Content {
        Content::heading(1, Content::strong(Content::text("RICH")))
    }
    fn located() -> Value {
        Value::LocatedContent(
            crate::entities::value::IntrospectedContent::new(rich(), None),
            crate::entities::locator::Locator::new().next(),
        )
    }
    fn assert_rich(content: &Content) {
        let mut headings = 0;
        let mut strong = 0;
        content.visit_content_static(&mut |node| {
            if let Content::Heading(h) = node {
                headings += 1;
                assert_eq!(h.body.plain_text(), "RICH");
            }
            if let Content::Strong(s) = node {
                strong += 1;
                assert_eq!(s.body.plain_text(), "RICH");
            }
            true
        });
        assert_eq!((headings, strong), (1, 1), "rich content lost: {content:?}");
    }
    fn inserted(source: &str) -> Content {
        use comemo::Track;
        let world = world(source);
        let library = Library::with_global(super::super::make_stdlib(
            &crate::contracts::world::SysInputs::default(),
        ));
        let mut library = library;
        library.global.define(
            "text",
            Value::Func(crate::entities::func::Func::native(
                "text",
                crate::compiler::stdlib::native_text,
            )),
        );
        let mut scopes = Scopes::new(Some(&library));
        scopes.define("located", located());
        let metrics = crate::compiler::layout::FixedMetrics;
        let route = Route::root().with_id(world.main());
        let mut styles = StyleChain::default_chain();
        let mut rules = Arc::from([]);
        let mut guards = Vec::new();
        let mut sink = Sink::new();
        let mut tracked = sink.track_mut();
        let mut engine = Engine {
            world: &world,
            font_metrics: &metrics,
            route: route.track(),
            styles: &mut styles,
            show_rules: &mut rules,
            active_guards: &mut guards,
            current_file: world.main(),
            sink: &mut tracked,
        };
        let mut ctx = EvalContext::new();
        eval_markup(world.source.root(), &mut scopes, &mut ctx, &mut engine)
            .expect("source-level display route must evaluate");
        let Some(Value::Content(content)) = scopes.get("result") else {
            panic!("expected result content")
        };
        content.clone()
    }
    #[test]
    fn v16_c_located_display_preserves_rich_body() {
        assert_rich(
            &value_to_display_content(located()).expect("content is displayable"),
        );
    }
    #[test]
    fn v16_c_located_display_markup_route() {
        assert_rich(&inserted("#let result = [#located]"));
    }
    #[test]
    fn v16_c_located_display_code_tail_route() {
        assert_rich(&inserted("#let result = {set text(size:12pt);located}"));
    }
    #[test]
    fn v16_c_located_display_math_route() {
        assert_rich(&inserted("#let result = $ #located $"));
    }
    #[test]
    fn v16_c_display_preserves_scalars_raw_and_empty() {
        assert_eq!(
            value_to_display_content(Value::Float(1.0)).unwrap().plain_text(),
            "1"
        );
        assert!(matches!(value_to_display_content(Value::Auto), Some(Content::Raw(_))));
        assert!(matches!(
            value_to_display_content(Value::Bool(false)),
            Some(Content::Raw(_))
        ));
        assert!(value_to_display_content(Value::None).is_none());
        assert!(matches!(
            value_to_display_content(Value::Content(Content::Empty)),
            Some(Content::Empty)
        ));
        assert_rich(&value_to_display_content(Value::Content(rich())).unwrap());
    }
}
