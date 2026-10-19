//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/rules/intercept.md
//! @prompt-hash 4bc481a2
//! @layer L1
//! @updated 2026-10-07
//!
//! Comportamento de `eval/rules::intercept`.

use super::*;

/// Aplica show rules ao Content produzido por eval (Passo 70 — DEBT-20 encerrado).
///
/// Anti-recursão via `active_guards` (stack de RuleId) em vez de booleano global.
/// Permite composição entre regras distintas; snapshot explícito evita borrow
/// conflict durante a travessia (DEBT-22).
pub(crate) fn intercept_content(
    mut content: Content,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    // P498 — passagem de captura do conteúdo original: não aplicar show-rules,
    // apenas registá-las (feito em `eval_show_rule`). Isto preserva os elementos
    // locatable para a introspecção, espelhando o modelo vanilla.
    if !ctx.apply_show_rules {
        return Ok(content);
    }

    if let Content::Raw(raw) = &content {
        let texts: Vec<&str> = raw.text.split('\n').collect();
        let count = texts.len() as i64;
        let lines: Vec<Content> = texts
            .iter()
            .enumerate()
            .map(|(index, text)| {
                let text_content = Content::text(*text);
                let body = if text.is_empty() {
                    Content::Empty
                } else if texts.len() > 1 && index == 0 {
                    Content::sequence(vec![text_content, Content::Empty])
                } else if texts.len() > 1 && index + 1 == texts.len() {
                    Content::sequence(vec![Content::Empty, text_content])
                } else {
                    text_content
                };
                Content::dynamic(crate::entities::elements::raw::RawLineElem::new(
                    index as i64 + 1,
                    count,
                    (*text).into(),
                    body,
                ))
            })
            .collect();
        let has_raw_line_rule = engine.show_rules.iter().any(|rule| {
            lines
                .iter()
                .any(|line| selector_matching::selector_matches(line, &rule.selector))
        });
        if has_raw_line_rule {
            let rules = Arc::clone(&*engine.show_rules);
            let mut realized = Vec::with_capacity(lines.len().saturating_mul(2));
            let line_count = lines.len();
            for (index, line) in lines.into_iter().enumerate() {
                realized.push(apply_show_rules(line, &rules, ctx, engine)?);
                if index + 1 < line_count {
                    realized.push(Content::linebreak());
                }
            }
            return Ok(Content::sequence(realized));
        }
    }

    // P1406 — o outline é realizado somente depois de a introspecção conhecer
    // as entries. Conservar aqui o snapshot lexical evita que uma regra
    // declarada mais tarde aja retroativamente sobre esta ocorrência.
    if let Content::Outline(elem) = &content {
        let elem = if elem.indent_was_set() {
            elem.as_ref().clone()
        } else {
            let inherited = engine.styles.custom("outline.indent").and_then(|value| {
                use crate::entities::elements::outline::OutlineIndent;
                match value {
                    Value::Auto => Some(OutlineIndent::Auto),
                    Value::Bool(value) => Some(OutlineIndent::Bool(*value)),
                    Value::Length(value) => Some(OutlineIndent::Length(*value)),
                    Value::Func(value) => Some(OutlineIndent::Function(value.clone())),
                    _ => None,
                }
            });
            inherited.map_or_else(
                || elem.as_ref().clone(),
                |indent| elem.with_resolved_indent(indent),
            )
        };
        let captured = elem.with_realization_context(
            Arc::clone(&*engine.show_rules),
            Styles::from_delta(engine.styles.collapse()),
        );
        content = Content::Outline(Arc::new(captured));
    }

    if let Content::OutlineEntry(elem) = &content {
        if !elem.fill_set {
            let inherited = match engine.styles.custom("outline.entry.fill") {
                Some(Value::Content(fill)) => Some(Some(fill.clone())),
                Some(Value::None) => Some(None),
                _ => None,
            };
            if let Some(fill) = inherited {
                content = Content::OutlineEntry(Arc::new(elem.remapped(
                    elem.element.clone(),
                    fill,
                    elem.prefix(),
                )));
            }
        }
    }

    if engine.show_rules.is_empty() {
        return Ok(content);
    }

    // Passo 84.4 (encerra DEBT-22): snapshot Arc::clone — O(1) refcount.
    // O slice partilhado permite iterar sobre uma cópia estável enquanto
    // `engine.show_rules` pode ser reatribuído por nested `#show` rules
    // sem interferência durante a travessia.
    let rules = Arc::clone(&*engine.show_rules);
    apply_show_rules(content, &rules, ctx, engine)
}

/// **P863** — Intercepção específica de parágrafos no fluxo montado pelo markup.
///
/// Parágrafos são sintetizados a partir de vários nós de texto/`Space`/`Parbreak`,
/// pelo que não têm um único ponto de construção onde `intercept_content` possa
/// atuar. Esta função realiza os parágrafos e aplica **apenas** as regras
/// `NodeKind::Par`, deixando as regras textuais (já aplicadas ao grupo
/// normalizado) e as restantes regras de elemento (já aplicadas nos seus pontos de
/// construção) intactas.
pub(crate) fn intercept_paragraphs(
    content: Content,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    if !ctx.apply_show_rules {
        return Ok(content);
    }

    let par_rules: Vec<ShowRule> = engine
        .show_rules
        .iter()
        .filter(|r| super::selector_matching::is_par_rule(&r.selector))
        .cloned()
        .collect();

    if par_rules.is_empty() {
        return Ok(content);
    }

    apply_show_rules(content, &par_rules, ctx, engine)
}

/// **P791** — Intercepção de wrappers `Content::Label` para regras
/// `Selector::Label` (`#show <sp>: …`). Chamada no ponto de associação
/// retroactiva de `<label>` em markup (Passo 56, `eval/mod.rs`): o wrapper é
/// criado **depois** do corpo já ter viajado pela maquinaria geral de show
/// rules, logo aqui só regras de label casam — as outras já aplicaram nos
/// seus pontos próprios (sem dupla aplicação).
///
/// Semântica (paridade vanilla, `target.label()` — o label é metadado do
/// elemento, não um nó): `it` = o **corpo** rotulado; a saída substitui o
/// wrapper inteiro (label consumido). Aplicação **única**,
/// última-declarada primeiro (innermost-first, P358) — sem revisitação
/// (equivalente à `Revocation` do vanilla). `Content` substitui; `Str` é
/// erro (consistente com `NodeKind`); show-set (`Style`) **não consome o
/// passe** — as que casam são dobradas e embrulham o wrapper em
/// `Content::Styled` (paridade P352).
pub(crate) fn intercept_labelled(
    labelled: Content,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    if !ctx.apply_show_rules || engine.show_rules.is_empty() {
        return Ok(labelled);
    }
    let Content::Label(e) = &labelled else {
        return Ok(labelled);
    };
    // Snapshot (Passo 84.4): slice estável enquanto closures podem mutar
    // `engine.show_rules`.
    let rules = Arc::clone(&*engine.show_rules);
    let mut scopes = Scopes::new(None);

    // Show-set: dobrar todas as regras de label que casam (ordem de
    // declaração, como no loop α) e embrulhar uma vez (collapse = top-wins).
    let mut set_chain = StyleChain::empty();
    let mut any_set = false;
    for rule in rules.iter() {
        if engine.active_guards.contains(&rule.id) {
            continue;
        }
        if let (Selector::Label(l), Transformation::Style(styles)) =
            (&rule.selector, &rule.transform)
        {
            if e.name.as_str() == l.0.as_str() {
                set_chain = set_chain.push(styles.delta().clone());
                any_set = true;
            }
        }
    }

    // Func/Content: primeira regra que casa (última-declarada primeiro),
    // uma única aplicação.
    for rule in rules.iter().rev() {
        if engine.active_guards.contains(&rule.id) {
            continue;
        }
        let Selector::Label(l) = &rule.selector else { continue };
        if e.name.as_str() != l.0.as_str() {
            continue;
        }
        let produced: Option<Content> = match &rule.transform {
            Transformation::Func(func) => {
                let args = Args::positional(vec![Value::Content(e.body.clone())]);
                engine.active_guards.push(rule.id);
                let call_result = call_dispatch::apply_func(
                    func.clone(),
                    args,
                    &mut scopes,
                    ctx,
                    engine,
                );
                engine.active_guards.pop();
                Some(match call_result? {
                    Value::Content(c) => c,
                    Value::Str(s) => Content::text(s.as_str()),
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            Span::detached(),
                            format!(
                                "show rule deve retornar Content ou String, \
                                 recebeu {}",
                                other.type_name()
                            ),
                        )]);
                    }
                })
            }
            Transformation::Content(c) => Some(c.clone()),
            // `Str` sobre label é erro — paridade com o comportamento sobre
            // `NodeKind` ("requer função ou Content, recebeu str").
            Transformation::Str(_) => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    "show rule com selector de label requer função ou Content, \
                     recebeu str"
                        .to_string(),
                )]);
            }
            // Show-set: tratada acima (não consome o passe).
            Transformation::Style(_) => None,
        };
        if let Some(out) = produced {
            return Ok(out);
        }
    }

    if any_set {
        let delta = set_chain.collapse();
        if !delta.is_empty() {
            return Ok(Content::styled(Box::new(labelled), Styles::from_delta(delta)));
        }
    }
    Ok(labelled)
}
