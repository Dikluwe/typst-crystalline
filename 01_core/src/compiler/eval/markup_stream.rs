//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/markup_stream.md
//! @prompt-hash 1783659c
//! @layer L1
//! @updated 2026-09-30

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
    // P1494: snapshots por ocorrência, sem retroatividade de regras textuais.
    let mut textual_snapshots: Vec<rules::textual::TextualSnapshot> = Vec::new();

    // Passo 445: smart quotes context-aware em markup.
    // O lexer marca `"` e `'` como `SyntaxKind::SmartQuote`; o eval decide
    // open/close/apostrophe com base nos caracteres adjacentes e no `text.lang`.
    // Aspas duplas e simples são resolvidas independentemente.
    let src = node.clone().into_text();
    let src_str = src.as_str();
    let mut byte_offset = 0_usize;

    fn is_opening_context(c: Option<char>) -> bool {
        c.is_none()
            || c.unwrap().is_whitespace()
            || matches!(c.unwrap(), '(' | '[' | '{' | '<')
    }
    fn is_word_char(c: Option<char>) -> bool {
        c.is_some_and(|c| c.is_alphanumeric())
    }

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

    for child in node.children() {
        let snapshot = rules::textual::TextualSnapshot::capture(engine);
        match child.kind() {
            SyntaxKind::Text => {
                // F-5b fatia 2 (P373): o render não é mais assado no node — viaja
                // na chain (custom), resolvido no layout.
                let text_node = Content::Text(child.text().as_str().into());
                append_markup_sequence(
                    &mut parts,
                    rules::textual::intercept_text_element(text_node, ctx, engine)?,
                );
            }
            // Passo 445 — SmartQuote: emite glyph localizado consoante
            // open/close (contexto adjacente) e `text.lang`/`smartquote` activo.
            SyntaxKind::SmartQuote => {
                let raw = child.text();
                let is_double = raw.as_str() == "\"";
                let lang = engine.styles.lang();

                let enabled = match engine.styles.custom("smartquote.enabled") {
                    Some(Value::Bool(b)) => *b,
                    _ => true,
                };

                let quote_node = if !enabled {
                    Content::Text(raw.as_str().into())
                } else {
                    let off = byte_offset;
                    let prev = src_str[..off].chars().last();
                    let next = src_str[off + child.len()..].chars().next();
                    let alternative = match engine.styles.custom("smartquote.alternative")
                    {
                        Some(Value::Bool(value)) => *value,
                        _ => false,
                    };
                    let configured = engine
                        .styles
                        .custom("smartquote.quotes")
                        .map(|value| {
                            crate::compiler::lang::quotes::parse_smartquote_quotes(
                                value,
                                child.span(),
                            )
                        })
                        .transpose()?;
                    let pair = crate::compiler::lang::quotes::resolve_smartquote_pair(
                        lang.as_ref(),
                        alternative,
                        configured.as_ref(),
                        is_double,
                    );
                    let glyph = if !is_double && is_word_char(prev) && is_word_char(next)
                    {
                        pair.close
                    } else if is_opening_context(prev) {
                        pair.open
                    } else {
                        pair.close
                    };
                    Content::Text(glyph)
                };
                append_markup_sequence(
                    &mut parts,
                    rules::textual::intercept_text_element(quote_node, ctx, engine)?,
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
            // Passo 56 — associação retroactiva: <label> envolve o nó precedente.
            // O parser expõe <label> como nó irmão (não filho) do nó anterior.
            // Entre o nó alvo e a label pode haver Space — salta-os para encontrar
            // o elemento real, re-insere-os a seguir ao Labelled.
            SyntaxKind::Label => {
                if let Some(label_ast) = child.cast::<AstLabel<'_>>() {
                    let name = label_ast.get().to_string();
                    // Recolher espaços finais para re-inserir após o Labelled.
                    let mut trailing: Vec<Content> = Vec::new();
                    while matches!(
                        parts.last(),
                        Some(Content::Space | Content::SpaceFromNewline | Content::Empty)
                    ) {
                        trailing.push(parts.pop().unwrap());
                    }
                    if let Some(last) = parts.pop() {
                        // P791 — o wrapper Label também viaja pela maquinaria
                        // de show rules (apenas regras `Selector::Label` —
                        // o corpo já foi interceptado no seu próprio ponto).
                        let labelled = Content::label_auto(name, last);
                        parts.push(rules::intercept_labelled(labelled, ctx, engine)?);
                        trailing.reverse();
                        parts.extend(trailing);
                    } else {
                        // P802 — label órfã (sem elemento anterior anexável):
                        // warning, paridade vanilla `typst-eval/markup.rs`
                        // ("label `<x>` is not attached to anything"). A label
                        // é descartada (como no vanilla); os espaços recolhidos
                        // são re-inseridos para não se perderem.
                        engine.sink.warn_note(
                            child.span(),
                            &format!("label `<{name}>` is not attached to anything"),
                            "",
                        );
                        trailing.reverse();
                        parts.extend(trailing);
                    }
                }
            }
            _ => {
                if let Some(expr) = Expr::from_untyped(child) {
                    let value = eval_expr(expr, scopes, ctx, engine)?;
                    if let Some(content) = value_to_display_content(value) {
                        append_markup_sequence(&mut parts, content);
                    }
                }
            }
        }
        textual_snapshots.resize(parts.len(), snapshot);

        byte_offset += child.len();

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
    for (idx, styles) in boundaries.into_iter().rev() {
        let tail = parts.split_off(idx);
        let styles = styles.push_custom("eval.flow.scope", Value::Bool(true));
        parts.push(Content::styled(Box::new(Content::sequence(tail)), styles));
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

/// Sequências interpoladas são transparentes no fluxo de markup. Os filhos
/// retêm a sua origem; um espaço inferido isolado só é decidido quando
/// encontra os vizinhos no corpo que o consome.
fn append_markup_sequence(parts: &mut Vec<Content>, content: Content) {
    match content {
        Content::Sequence(children) => {
            for child in children.iter().cloned() {
                append_markup_sequence(parts, child);
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
        // P471 — símbolo Unicode → char como Content::Text.
        Value::Symbol(s) => Some(Content::Text(EcoString::from(s.value))),
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
