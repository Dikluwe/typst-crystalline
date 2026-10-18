//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/markup_stream.md
//! @prompt-hash ef8085d5
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
        let snapshot = rules::textual::TextualSnapshot::capture(engine);
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
                        if let Some(content) = value_to_display_content(value) {
                            let content = if boolean_display {
                                rules::intercept_content(content, ctx, engine)?
                            } else {
                                content
                            };
                            let content =
                                crate::compiler::realize::capture_table_insertion(
                                    content, ctx, engine,
                                )?;
                            let content = if ctx.realizing_show_rules {
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
    rules::textual::realize_textual_parts(
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
                    .filter(|part| !matches!(part, Content::Empty))
                    .collect(),
            );
            parts.push(crate::compiler::realize::sequences::capture_scope(
                tail,
                scope,
                sequence_origin,
                ctx,
            ));
        } else {
            let (index, delta) = styles.next().unwrap();
            let tail = parts.split_off(index);
            let delta = delta.push_custom("eval.flow.scope", Value::Bool(true));
            parts.push(Content::styled(Box::new(Content::sequence(tail)), delta));
        }
    }

    // P863: as show rules de elemento (NodeKind) são interceptadas nos pontos
    // de construção do conteúdo, mas os parágrafos são sintetizados a partir de
    // vários nós de texto/`Space`/`Parbreak`. Aplica-se apenas as regras
    // `NodeKind::Par` ao fluxo montado, para que `#show par: ...` possa realizar
    // e transformar os parágrafos sem re-disparar regras textuais já aplicadas
    // ao grupo normalizado acima.
    Ok(Value::Content(rules::intercept_paragraphs(
        Content::sequence(parts),
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

/// Sequências interpoladas são transparentes no fluxo de markup. Os filhos
/// retêm a sua origem; um espaço inferido isolado só é decidido quando
/// encontra os vizinhos no corpo que o consome.
fn append_markup_sequence(
    parts: &mut Vec<Content>,
    content: Content,
    preserve_source: bool,
) {
    match content {
        Content::Source(e) if !preserve_source => {
            append_markup_sequence(parts, e.body.clone(), false)
        }
        Content::Source(e) if matches!(e.body.source_body(), Content::Sequence(_)) => {
            if let Content::Sequence(children) = e.body.source_body() {
                for child in children.iter().cloned() {
                    append_markup_sequence(parts, child.with_source_span(e.span), true);
                }
            }
        }
        Content::Sequence(children) => {
            for child in children.iter().cloned() {
                append_markup_sequence(parts, child, preserve_source);
            }
        }
        other => parts.push(other),
    }
}

/// **P545** (extraído em **P780**) — conversão genérica `Value` → `Content`
/// para exibição (interpolação `#{expr}` em markup, e resolução de
/// identificador de variável em modo math — `compiler/eval/math.rs`). `None`
/// = nada a mostrar (`Value::None`, `Value::Counter`, texto vazio) —
/// distinto de `Some(Content::Empty)`, que só ocorre se o próprio
/// `Value::Content` embrulhado já for `Content::Empty` (preserva-se, não
/// se filtra).
///
/// Paridade conceptual com `Value::display()` do vanilla
/// (`typst-eval/src/math.rs::ExprExt::eval_display` chama `.display()`
/// genericamente) — cristalino não tem esse método unificado; esta função
/// é o equivalente inlined.
pub(crate) fn value_to_display_content(value: Value) -> Option<Content> {
    match value {
        Value::Content(c) => Some(c),
        Value::Str(s) => Some(Content::Text(s)),
        Value::Auto => Some(Content::raw("auto", Some("typc".into()), false)),
        Value::Bool(b) => Some(Content::raw(
            if b { "true" } else { "false" },
            Some("typc".into()),
            false,
        )),
        // P471 — símbolo Unicode → char como Content::Text.
        Value::Symbol(s) => Some(Content::Text(s.value)),
        // P506 — state(key, init) → Content::State locatável.
        Value::State(s) => {
            Some(Content::state(s.key.to_string(), s.init.as_ref().clone()))
        }
        // P506 — counter(selector) → nada visível directamente (só via
        // .update()/.step()/.display()).
        Value::Counter(_) => None,
        Value::None => None,
        // P796 — Version usa o `Display` (`to_string()`), não o `repr()`:
        // `#sys.version` → "0.15.0", não "version(0, 15, 0)" (paridade
        // vanilla `Value::display`, `entities/version.md` §8b).
        Value::Version(v) => {
            let text = v.to_string();
            if text.is_empty() {
                None
            } else {
                Some(Content::Text(text.into()))
            }
        }
        // Valores primitivos convertem-se para texto. Int, Float, Bool,
        // Array, Dict, Length, Datetime, etc. usam repr_value.
        other => {
            // **P739C** — display de Float: paridade vanilla (Display de
            // f64 — inteiros exactos sem `.0`: `#(1.0)` → "1", `#(4/2)` →
            // "2", medido). `repr` mantém "1.0" (repr_value inalterado).
            // **P817-D** — display de Decimal: paridade vanilla
            // (`#decimal("1.50")` → "1.50", não `decimal("1.50")` — o repr
            // constructor-form é só para `repr()`, medido nos dois binários).
            let text = match &other {
                Value::Float(f) => format!("{f}"),
                Value::Decimal(d) => d.to_string(),
                _ => crate::compiler::eval::repr::repr_value(&other),
            };
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
