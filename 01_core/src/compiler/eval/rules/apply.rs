//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/rules/apply.md
//! @prompt-hash 695c9870
//! @layer L1
//! @updated 2026-10-07
//!
//! Comportamento de `eval/rules::apply`.

use super::*;

/// Aplica as show rules activas ao Content (Passo 70 — DEBT-23 encerrado).
///
/// NodeKind rules: única travessia `map_content` para todas as regras (O(N)).
/// Dentro da closure, itera o snapshot de regras e salta as que estão em
/// `active_guards` (anti-recursão por rule ID — DEBT-20 encerrado).
///
/// Text rules: aplicadas separadamente via `map_text` após a travessia principal.
pub(crate) fn apply_show_rules(
    content: Content,
    rules: &[ShowRule],
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    apply_show_rules_inner(content, rules, ctx, engine, false, &mut false)
}

pub(crate) fn apply_show_rules_root(
    content: Content,
    rules: &[ShowRule],
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    apply_show_rules_inner(content, rules, ctx, engine, true, &mut false)
}

pub(crate) fn apply_cell_rules_root(
    content: Content,
    rules: &[ShowRule],
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<(Content, bool)> {
    let mut applied = false;
    let product =
        apply_show_rules_inner(content, rules, ctx, engine, true, &mut applied)?;
    Ok((product, applied))
}

fn apply_show_rules_inner(
    mut content: Content,
    rules: &[ShowRule],
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    root_only: bool,
    cell_applied: &mut bool,
) -> SourceResult<Content> {
    if rules.is_empty() {
        return Ok(content);
    }

    if ctx.defer_show_rules {
        ctx.has_deferred_show_rules = true;
        return Ok(Content::ShowRecipe(Arc::new(
            crate::entities::content::ShowRecipeContent {
                body: content,
                textual: false,
                context_id_floor: ctx.next_context_id,
                rules: Arc::from(rules),
                styles: engine.styles.clone(),
                guards: engine.active_guards.clone(),
                file: engine.current_file,
            },
        )));
    }

    if ctx.realizing_show_rules {
        for rule in rules {
            ctx.next_rule_id = ctx.next_rule_id.max(rule.id.saturating_add(1));
        }
    }

    // P394: scope vazio para apply_func; closures de show-rule usam captured
    // scope como parent, e nativas comuns não usam scopes.
    let mut scopes = Scopes::new(None);

    // Limite de aninhamento vanilla (MAX_SHOW_RULE_DEPTH = 64) —
    // paridade com `typst-realize/src/lib.rs:402` (ADR-0033).
    // Pago parcial do DEBT-45 no Passo 93.
    route_check_show_depth(engine.route)?;

    // P863: se há regras sobre parágrafos, realizar os parágrafos antes de
    // aplicar as show rules. A geometria continua no consumer do parágrafo.
    let has_par_rule = rules
        .iter()
        .any(|r| super::selector_matching::is_par_rule(&r.selector));
    if has_par_rule {
        content = realize_paragraphs(content);
    }

    // Separar regras por tipo para travessias distintas. Lote F-3 inc-2: as
    // regras de **kind dinâmico** (`#show callout:`) viajam pela MESMA travessia
    // que as NodeKind — mesmo `apply_all`, mesma ordem, mesmo guard por `RuleId`.
    let has_node_rules =
        rules.iter().any(|r| selector_matching::is_node_rule(&r.selector));

    if has_node_rules {
        // Única travessia para todas as NodeKind + DynKind rules, incluindo
        // Where com base node-like (P417).
        let node_rules: Vec<ShowRule> = rules
            .iter()
            .filter(|r| selector_matching::is_node_rule(&r.selector))
            .cloned()
            .collect();

        let completed_visuals = if ctx.realizing_show_rules {
            completed_visual_locations(&content)
        } else {
            std::collections::HashSet::new()
        };
        let mut apply_all = |node: &Content| -> SourceResult<Option<Content>> {
            if !root_only
                && matches!(node,Content::TableCell(c) if c.resolved.is_some() || c.standalone.is_some())
            {
                return Ok(None);
            }
            if ctx.realizing_show_rules {
                if matches!(node, Content::Realized(_))
                    || matches!(node, Content::Located(e) if completed_visuals.contains(&e.location))
                {
                    return Ok(None);
                }
                if crate::compiler::introspect::extract_payload::extract_payload(node)
                    .is_some()
                {
                    return realize_occurrence(node, &node_rules, ctx, engine, root_only);
                }
            }
            // P348 (modelo α, ADR-0107): a element rule cujo output **re-casa** é
            // **revisitada** até **ponto-fixo morfológico** — o loop re-alimenta o
            // output no mesmo conjunto de regras e para quando a regra é um **no-op
            // morfológico** (`morph_canon(out) == morph_canon(work)`, o `==` do P345).
            // Recursão **não-convergente** (ciclo / divergente) é cortada pelo **teto**
            // backstop (`MAX_SHOW_RULE_DEPTH`, mecânica) e vira erro com a mensagem base
            // do vanilla. O `active_guards` continua a impedir a recursão **durante** a
            // chamada do recipe (criação aninhada); a revisitação é este loop, após o
            // recipe devolver. Divergência consciente vs vanilla (P347d/ADR-0107):
            // `#show heading: it => [= Z]` **converge para "Z"** (ponto-fixo) onde o
            // vanilla erra — o vanilla termina por identidade de instância (mecânica,
            // GEROU em P347b/c), o cristalino por morfologia.
            // P348 (modelo α, ADR-0107): a element rule cujo output **re-casa** é
            // **revisitada** até **ponto-fixo morfológico**. O loop α vive agora em
            // `compiler/eval/show_rule_termination.rs` (Passo 1009), que acrescenta
            // detecção de ciclos por histórico de formas canónicas.
            let owned;
            let node = if ctx.realizing_show_rules
                && crate::compiler::introspect::extract_payload::extract_payload(node)
                    .is_some()
            {
                owned = crate::compiler::realize::locate(node.clone(), ctx);
                &owned
            } else {
                node
            };
            let full_error = ctx.full_error;
            let (mut work, applied) = show_rule_termination::run_show_rule_loop(
                node.clone(),
                |work| {
                    for rule in node_rules.iter().rev() {
                        // Saltar se esta regra está em execução (anti-recursão na criação
                        // aninhada — Lote F-3 inc-2; guard por `RuleId`, vale p/ DynKind).
                        let guarded = if let Content::TableCell(c) = work {
                            if c.stage().is_some() {
                                c.guards().contains(&rule.id)
                            } else {
                                engine.active_guards.contains(&rule.id)
                            }
                        } else {
                            engine.active_guards.contains(&rule.id)
                        };
                        if guarded {
                            continue;
                        }

                        // Show-set (P352): NÃO consome o passe de func — é tratado após o
                        // loop α (embrulha o nó em `Content::Styled`). Saltado aqui.
                        if matches!(rule.transform, Transformation::Style(_)) {
                            continue;
                        }

                        if !selector_matching::selector_matches(work, &rule.selector) {
                            continue;
                        }

                        match &rule.transform {
                            Transformation::Func(func) => {
                                let argument=if let Content::TableCell(c)=work {
                                    if c.stage().is_some() {Content::TableCell(Arc::new(c.guarded(rule.id)))} else {work.clone()}
                                } else {work.clone()};
                                let args = Args::positional(vec![Value::Content(argument)]);
                                engine.active_guards.push(rule.id);
                                let old_defer=ctx.defer_show_rules;
                                if matches!(work,Content::TableCell(c) if c.stage().is_some()) {ctx.defer_show_rules=true;}
                                let call_result = call_dispatch::apply_func(
                                    func.clone(),
                                    args,
                                    &mut scopes,
                                    ctx,
                                    engine,
                                );
                                ctx.defer_show_rules=old_defer;
                                engine.active_guards.pop();
                                let produced = match call_result? {
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
                                        )])
                                    }
                                };
                                return Ok(Some((produced, rule.id)));
                            }
                            Transformation::Content(c) => {
                                return Ok(Some((c.clone(), rule.id)));
                            }
                            // `Str` só é válida sobre `Selector::Text` (tratada no loop
                            // de texto). Sobre NodeKind/DynKind é erro — paridade com o
                            // comportamento anterior ("recebeu str").
                            Transformation::Str(_) => {
                                return Err(vec![SourceDiagnostic::error(
                                    Span::detached(),
                                    "show rule com selector de tipo requer função ou Content, \
                                     recebeu str"
                                        .to_string(),
                                )])
                            }
                            // Saltado acima; inalcançável.
                            Transformation::Style(_) => continue,
                        }
                    }
                    Ok(None)
                },
                crate::entities::world_types::Route::MAX_SHOW_RULE_DEPTH,
                full_error,
            )?;

            *cell_applied |= applied;

            // Show-set (P352/P356, S5): embrulha o output no `Styles` das regras
            // **show-set** que casam o **ELEMENTO** — o nó **original** que entrou na
            // realização (`node`), NÃO o `work` pós-func. **NÃO consome o passe** —
            // espelha `map.apply(transform); continue` do vanilla
            // (`typst-realize/src/lib.rs:458-464` / `styles.rs:504`).
            //
            // **Ordem show-set-vs-func (P356, paridade `lib.rs:341,357`).** O vanilla
            // dobra a show-set na chain (`map`) e aplica a func **sob** `chained =
            // styles.chain(&map)`. Casar contra `node` (o elemento) e embrulhar o
            // `work` (o output da func) reproduz isso: a show-set fica ativa quando a
            // func realiza. Casar contra `work` (pós-func) — o bug que isto conserta —
            // perdia a show-set (o output da func não casa o seletor do elemento).
            // Sem func, `work == node` → idêntico ao P352 (show-set-só e múltiplos
            // show-set por `collapse`). O confinamento vem do `Content::Styled`
            // (fatia 1, `f_fronteira_e1.md §3a.8`). `collapse` resolve top-wins.
            let mut set_chain = StyleChain::empty();
            let mut any_set = false;
            for rule in &node_rules {
                if let Transformation::Style(styles) = &rule.transform {
                    if engine.active_guards.contains(&rule.id) {
                        continue;
                    }
                    if selector_matching::selector_matches(node, &rule.selector) {
                        set_chain = crate::entities::elements::table_cell::TableCellProperties::chain_styles(&set_chain, styles);
                        any_set = true;
                    }
                }
            }
            let mut wrapped = false;
            let mut raw_overrides = Styles::default();
            if any_set {
                let mut delta = set_chain.collapse();
                mark_heading_show_set(&mut delta, node, &work);
                for (key, value) in &delta.custom {
                    if key.as_str() == "text.size" {
                        raw_overrides =
                            raw_overrides.push_custom("raw.block.size", value.clone());
                    } else if matches!(
                        key.as_str(),
                        "block.above" | "block.below" | "block.spacing"
                    ) {
                        raw_overrides =
                            raw_overrides.push_custom(key.clone(), value.clone());
                    }
                }
                if let Some(size) = delta.size {
                    raw_overrides =
                        raw_overrides.push_custom("raw.block.size", Value::Length(size));
                }
                if !delta.is_empty() {
                    work = Content::styled(Box::new(work), Styles::from_delta(delta));
                    wrapped = true;
                }
            }

            if applied || wrapped {
                if let Content::Raw(raw) = node {
                    let defaults = raw_overrides
                        .push_custom("raw.defaults", Value::Bool(true))
                        .push_custom("raw.block", Value::Bool(raw.block));
                    work = Content::styled(Box::new(work), defaults);
                }
                if ctx.realizing_show_rules
                    && crate::compiler::introspect::extract_payload::extract_payload(node)
                        .is_some()
                {
                    Ok(Some(Content::Realized(Arc::new(
                        crate::entities::content::RealizedContent {
                            original: node.clone(),
                            visual: work,
                        },
                    ))))
                } else {
                    Ok(Some(work))
                }
            } else {
                Ok(None)
            }
        };

        content = if root_only {
            apply_all(&content)?.unwrap_or(content)
        } else {
            content.map_content(&mut apply_all)?
        };
    }

    if root_only {
        return Ok(content);
    }

    // Regex rules (P393) — aplicadas a nós de texto que casam.
    // Última-declarada vence (consistente com NodeKind). Transformações
    // Func/Content/Str são suportadas; Style foi rejeitado em parse.
    let regex_rules: Vec<ShowRule> = rules
        .iter()
        .filter(|r| matches!(r.selector, Selector::Regex(_)))
        .cloned()
        .collect();

    if !regex_rules.is_empty() {
        let mut apply_regex = |node: &Content| -> SourceResult<Option<Content>> {
            let Content::Text(text) = node else {
                return Ok(None);
            };
            for rule in regex_rules.iter().rev() {
                if engine.active_guards.contains(&rule.id) {
                    continue;
                }
                let Selector::Regex(re) = &rule.selector else { continue };
                let produced = match &rule.transform {
                    Transformation::Func(func) => {
                        selector_matching::splice_regex_rule_matches(
                            text.as_str(),
                            re,
                            |matched| {
                                let args = Args::positional(vec![Value::Content(
                                    Content::text(matched),
                                )]);
                                engine.active_guards.push(rule.id);
                                let call_result = call_dispatch::apply_func(
                                    func.clone(),
                                    args,
                                    &mut scopes,
                                    ctx,
                                    engine,
                                );
                                engine.active_guards.pop();
                                match call_result? {
                                    Value::Content(c) => Ok(c),
                                    Value::Str(s) => Ok(Content::text(s.as_str())),
                                    other => Err(vec![SourceDiagnostic::error(
                                        Span::detached(),
                                        format!(
                                            "show rule regex deve retornar Content ou String, \
                                             recebeu {}",
                                            other.type_name()
                                        ),
                                    )]),
                                }
                            },
                        )?
                    }
                    Transformation::Content(c) => {
                        selector_matching::splice_regex_rule_matches(
                            text.as_str(),
                            re,
                            |_| Ok(c.clone()),
                        )?
                    }
                    Transformation::Str(s) => {
                        selector_matching::splice_regex_rule_matches(
                            text.as_str(),
                            re,
                            |_| Ok(Content::text(s.as_str())),
                        )?
                    }
                    Transformation::Style(_) => continue,
                };
                if produced.is_some() {
                    return Ok(produced);
                }
            }
            Ok(None)
        };
        content = content.map_content(&mut apply_regex)?;
    }

    // Text rules — P790: `Str` via `map_text` (comportamento pré-existente);
    // `Content`/`Func` por splice em `map_content` — o `Content::Text` é
    // fatiado nas ocorrências do padrão e o replacement é emendado entre as
    // fatias (paridade vanilla `visit_regex_match`; `map_content` não
    // reentra no nó substituído, logo o output não é re-varrido pela mesma
    // regra — equivalente à `Revocation` do vanilla). Match por nó de texto
    // individual (cross-node: scope-out registado no L0, eval.md §P790).
    for rule in rules {
        let Selector::Text(pattern) = &rule.selector else { continue };
        match &rule.transform {
            Transformation::Str(s) => {
                let replacement = s.to_string();
                let mut do_replace =
                    |text: &str| text.replace(pattern.as_str(), &replacement);
                content = content.map_text(&mut do_replace);
            }
            Transformation::Content(replacement) => {
                let mut apply_text = |node: &Content| -> SourceResult<Option<Content>> {
                    let Content::Text(text) = node else { return Ok(None) };
                    selector_matching::splice_text_rule_matches(
                        text.as_str(),
                        pattern,
                        |_| Ok(replacement.clone()),
                    )
                };
                content = content.map_content(&mut apply_text)?;
            }
            Transformation::Func(func) => {
                let mut apply_text = |node: &Content| -> SourceResult<Option<Content>> {
                    let Content::Text(text) = node else { return Ok(None) };
                    selector_matching::splice_text_rule_matches(
                        text.as_str(),
                        pattern,
                        |matched| {
                            let args = Args::positional(vec![Value::Content(
                                Content::text(matched),
                            )]);
                            engine.active_guards.push(rule.id);
                            let call_result = call_dispatch::apply_func(
                                func.clone(),
                                args,
                                &mut scopes,
                                ctx,
                                engine,
                            );
                            engine.active_guards.pop();
                            match call_result? {
                                Value::Content(c) => Ok(c),
                                Value::Str(s) => Ok(Content::text(s.as_str())),
                                other => Err(vec![SourceDiagnostic::error(
                                    Span::detached(),
                                    format!(
                                        "show rule deve retornar Content ou String, \
                                     recebeu {}",
                                        other.type_name()
                                    ),
                                )]),
                            }
                        },
                    )
                };
                content = content.map_content(&mut apply_text)?;
            }
            Transformation::Style(_) => {
                // Rejeitado em `eval_show_rule` para `Selector::Text` —
                // inalcançável (show-set é sobre elementos).
            }
        }
    }

    Ok(content)
}

/// Productive locatables retain their identity across aliases. The legacy
/// morphology loop above remains available for its separate evaluation path.
fn mark_heading_show_set(
    delta: &mut crate::entities::style_chain::StyleDelta,
    original: &Content,
    visual: &Content,
) {
    fn is_heading(mut content: &Content) -> bool {
        loop {
            content = content.source_body();
            if let Content::Located(located) = content {
                content = &located.body;
            } else {
                return matches!(content, Content::Heading(_));
            }
        }
    }
    if is_heading(original)
        && is_heading(visual)
        && (delta.size.is_some()
            || delta.custom.iter().any(|(key, value)| {
                key.as_str() == "text.size" && matches!(value, Value::Length(_))
            }))
    {
        delta.custom.push(("heading.show-set-size".into(), Value::Bool(true)));
    }
}

fn realize_occurrence(
    node: &Content,
    rules: &[ShowRule],
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    root_only: bool,
) -> SourceResult<Option<Content>> {
    let node = crate::compiler::realize::locate(node.clone(), ctx);
    let Content::Located(located) = &node else { unreachable!() };
    let location = located.location;
    let mut set_chain = StyleChain::empty();
    for rule in rules {
        if let Transformation::Style(styles) = &rule.transform {
            if selector_matching::selector_matches(&node, &rule.selector)
                && !ctx.occurrence_show_results.contains_key(&(location, rule.id))
            {
                ctx.occurrence_show_results.insert((location, rule.id), None);
                set_chain = crate::entities::elements::table_cell::TableCellProperties::chain_styles(&set_chain, styles);
            }
        }
    }
    let mut work = node.clone();
    let mut applied = false;
    for rule in rules.iter().rev() {
        if matches!(rule.transform, Transformation::Style(_))
            || !selector_matching::selector_matches(&node, &rule.selector)
        {
            continue;
        }
        let key = (location, rule.id);
        if let Some(cached) = ctx.occurrence_show_results.get(&key) {
            if let Some(product) = cached {
                work = product.clone();
                applied = true;
                break;
            }
            continue;
        }
        if ctx.occurrence_show_depth
            >= crate::entities::world_types::Route::MAX_SHOW_RULE_DEPTH
        {
            let mut error = SourceDiagnostic::error(
                Span::detached(),
                "maximum show rule depth exceeded",
            )
            .with_hint("maybe a show rule matches its own output")
            .with_hint("maybe there are too deeply nested elements");
            if ctx.full_error {
                let canon = node.morph_canon();
                error = error.with_hint(if ctx.occurrence_show_path.contains(&canon) {
                    "erro completo: recursão CÍCLICA — uma forma de conteúdo repetiu-se no caminho de revisitação (a regra de #show reescreve para algo que reaparece)"
                } else {
                    "erro completo: recursão NÃO-CONVERGENTE — passou do limite sem repetir nem estabilizar (verifique se a regra termina, ou se é recursão legítima profunda)"
                });
            }
            return Err(vec![error]);
        }
        ctx.occurrence_show_results.insert(key, None);
        ctx.occurrence_show_depth += 1;
        if ctx.full_error {
            ctx.occurrence_show_path.push(node.morph_canon());
        }
        let result = (|| {
            let produced = match &rule.transform {
                Transformation::Func(func) => {
                    let mut scopes = Scopes::new(None);
                    match call_dispatch::apply_func(
                        func.clone(),
                        Args::positional(vec![Value::Content(node.clone())]),
                        &mut scopes,
                        ctx,
                        engine,
                    )? {
                        Value::Content(content) => content,
                        Value::Str(text) => Content::text(text.as_str()),
                        other => {
                            return Err(vec![SourceDiagnostic::error(
                                Span::detached(),
                                format!(
                                    "show rule deve retornar Content ou String, recebeu {}",
                                    other.type_name()
                                ),
                            )])
                        }
                    }
                }
                Transformation::Content(content) => content.clone(),
                Transformation::Str(_) => {
                    return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        "show rule com selector de tipo requer função ou Content, recebeu str",
                    )])
                }
                Transformation::Style(_) => unreachable!(),
            };
            if root_only {
                Ok(produced)
            } else {
                realize_occurrences(&produced, rules, ctx, engine)
            }
        })();
        ctx.occurrence_show_depth -= 1;
        if ctx.full_error {
            ctx.occurrence_show_path.pop();
        }
        match result {
            Ok(product) => {
                ctx.occurrence_show_results.insert(key, Some(product.clone()));
                work = product;
                applied = true;
            }
            Err(errors) => {
                ctx.occurrence_show_results.remove(&key);
                return Err(errors);
            }
        }
        break;
    }
    let mut delta = set_chain.collapse();
    mark_heading_show_set(&mut delta, &node, &work);
    let wrapped = !delta.is_empty();
    if wrapped {
        work = Content::styled(Box::new(work), Styles::from_delta(delta));
    }
    if applied || wrapped {
        Ok(Some(Content::Realized(Arc::new(crate::entities::content::RealizedContent {
            original: node,
            visual: work,
        }))))
    } else {
        // Even an unmatched fresh product must keep its newly allocated identity.
        Ok(Some(node))
    }
}

/// A completed visual already carries its recipe result. Visiting its aliases
/// again must not substitute that complete result into each emitted alias.
/// Collect only visual locations: consumed originals remain query provenance.
fn completed_visual_locations(
    content: &Content,
) -> std::collections::HashSet<crate::entities::location::Location> {
    let mut locations = std::collections::HashSet::new();
    let _ = content.map_content(&mut |node| {
        if let Content::Realized(realized) = node {
            let _ = realized.visual.map_content(&mut |visual| {
                if let Content::Located(located) = visual {
                    locations.insert(located.location);
                }
                Ok(None)
            });
        }
        Ok(None)
    });
    locations
}

/// Preserve the lexical rule snapshot at insertion of an alias into markup.
/// Text rules and paragraph preparation keep their separate coordinating path.
pub(crate) fn realize_inserted_occurrences(
    content: Content,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    let rules: Vec<ShowRule> = engine
        .show_rules
        .iter()
        .filter(|rule| selector_matching::is_node_rule(&rule.selector))
        .cloned()
        .collect();
    if rules.is_empty() {
        Ok(content)
    } else {
        realize_occurrences(&content, &rules, ctx, engine)
    }
}

fn realize_occurrences(
    content: &Content,
    rules: &[ShowRule],
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    let content =
        crate::compiler::realize::realize_table_insertions(content.clone(), ctx, engine)?;
    let content = crate::compiler::realize::bare_math::prepare(&content, engine.styles)?;
    let completed_visuals = completed_visual_locations(&content);
    content.map_content(&mut |child| {
        if matches!(child, Content::Realized(_))
            || matches!(child, Content::Located(e) if completed_visuals.contains(&e.location))
        {
            return Ok(None);
        }
        if crate::compiler::introspect::extract_payload::extract_payload(child).is_some() {
            realize_occurrence(child, rules, ctx, engine, false)
        } else {
            Ok(None)
        }
    })
}
