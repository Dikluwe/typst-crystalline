//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/introspect/entrypoints.md
//! @prompt-hash 34c5b898
//! @layer L1
//! @updated 2026-10-05

use super::*;
use crate::compiler::eval::EvalContext;
use crate::entities::engine::Engine;
use crate::entities::source_result::SourceResult;

/// Entry point novo (M4 / P166): produz `CounterStateLegacy` E
/// `TagIntrospector` num único walk. Consumers que precisem do
/// introspector usam este entry point; consumers legacy continuam a
/// usar `introspect()`.
///
/// **Walk único subjacente**: state + introspector vêm da mesma
/// passagem — não há duplicação. `introspect()` é wrapper que descarta
/// o introspector.
///
/// Walk puro: registra ações de State sem executar callbacks. Leitores modernos
/// resolvem a chave com seu próprio init, usando Engine e EvalContext na leitura.
/// Os pós-processadores runtime permanecem em seus entrypoints específicos.
pub fn introspect_with_introspector(content: &Content) -> TagIntrospector {
    // **P533** — garantir que `@key` bibliográficos são vistos como
    // `Content::Cite` durante o walk, para que `citation_order` e
    // `back_refs` fiquem correctos.
    let content = convert_bib_refs_to_cites(content.clone());
    let mut locator = Locator::new();
    super::walk::reserve_locations(&content, &mut locator);
    let mut tags: Vec<Tag> = Vec::new();
    let mut intr = TagIntrospector::empty();
    let mut auto_label_counter: usize = 0;
    // P363: chain raiz = default_chain (espelha o root do layout, `layout/mod.rs`).
    let root_chain = StyleChain::default_chain();
    // **P1034** — `lang` da chain raiz (normalmente `None`; ver
    // `StyleChain::lang`). O default `en` para o texto gerado é aplicado por
    // `lang::figure_supplement`, não aqui.
    let root_lang = root_chain.lang();
    walk(
        &content,
        &mut locator,
        &mut tags,
        &mut intr,
        &mut auto_label_counter,
        root_lang.as_ref(),
        &root_chain,
        None,
    );
    materialize_outline_entries(&mut locator, &mut intr);
    intr.parent_locations = build_parent_index(&tags);
    intr
}

/// Percorre os documentos do bundle numa única sequência causal e registra
/// o início de cada body antes de nele entrar.
pub fn introspect_bundle_documents_with_starts(
    bodies: &[Content],
) -> (TagIntrospector, Vec<LocatorSeed>) {
    let keys = bodies
        .iter()
        .fold(std::collections::HashSet::new(), |mut keys, body| {
            keys.extend(collect_bib_keys(body));
            keys
        });
    let mut locator = Locator::new();
    for body in bodies {
        super::walk::reserve_locations(body, &mut locator);
    }
    let mut starts = Vec::with_capacity(bodies.len());
    let mut tags = Vec::new();
    let mut intr = TagIntrospector::empty();
    let mut auto_label_counter = 0;
    let root_chain = StyleChain::default_chain();
    let root_lang = root_chain.lang();
    for body in bodies {
        let seed = locator.checkpoint();
        let boundary = Locator::from_seed(seed).next();
        starts.push(seed);
        // Fronteira lógica: o próximo Location é observado, não consumido.
        intr.counters.observe_location(boundary);
        intr.counters
            .apply_at(CounterKey::Page, CounterUpdate::Set(vec![1]), boundary);
        let first_tag = tags.len();
        let converted = if keys.is_empty() {
            body.clone()
        } else {
            convert_refs(body.clone(), &keys)
        };
        walk(
            &converted,
            &mut locator,
            &mut tags,
            &mut intr,
            &mut auto_label_counter,
            root_lang.as_ref(),
            &root_chain,
            None,
        );
        intr.counters.record_document_locations(
            tags[first_tag..]
                .iter()
                .filter_map(|tag| {
                    if let Tag::Start(location, _) = tag {
                        Some(*location)
                    } else {
                        None
                    }
                })
                .collect(),
        );
    }
    materialize_outline_entries(&mut locator, &mut intr);
    intr.parent_locations = build_parent_index(&tags);
    (intr, starts)
}

/// Introspecta todos os bodies do bundle com os mesmos inícios causais do
/// caminho puro e resolve callbacks Page na faixa do próprio documento.
pub fn introspect_bundle_documents_with_runtime_and_starts(
    bodies: &[Content],
    engine: &mut crate::entities::engine::Engine<'_>,
    ctx: &mut crate::compiler::eval::EvalContext,
) -> crate::entities::source_result::SourceResult<(TagIntrospector, Vec<LocatorSeed>)> {
    let targets = vec![crate::compiler::eval::EvalTarget::Paged; bodies.len()];
    introspect_bundle_documents_with_runtime_targets_and_starts(
        bodies,
        &targets,
        ctx.features,
        engine,
        ctx,
    )
}

/// Complete global snapshot with actual per-body semantic targets.
pub fn introspect_bundle_documents_with_runtime_targets_and_starts(
    bodies: &[Content],
    targets: &[crate::compiler::eval::EvalTarget],
    features: crate::entities::compiler_features::Features,
    engine: &mut Engine<'_>,
    ctx: &mut EvalContext,
) -> SourceResult<(TagIntrospector, Vec<LocatorSeed>)> {
    if bodies.len() != targets.len() {
        return Err(vec![crate::entities::source_result::SourceDiagnostic::error(
            crate::entities::span::Span::detached(),
            "bundle body target metadata does not match body count",
        )]);
    }
    let keys = bodies
        .iter()
        .fold(std::collections::HashSet::new(), |mut keys, body| {
            keys.extend(collect_bib_keys(body));
            keys
        });
    let mut locator = Locator::new();
    for body in bodies {
        super::walk::reserve_locations(body, &mut locator);
    }
    let mut starts = Vec::with_capacity(bodies.len());
    let mut body_tags = Vec::with_capacity(bodies.len());
    let mut tags = Vec::new();
    let mut intr = TagIntrospector::empty();
    let mut auto_label_counter = 0;
    let root_chain = StyleChain::default_chain();
    let root_lang = root_chain.lang();
    for body in bodies {
        let seed = locator.checkpoint();
        let boundary = Locator::from_seed(seed).next();
        starts.push(seed);
        intr.counters.observe_location(boundary);
        intr.counters
            .apply_at(CounterKey::Page, CounterUpdate::Set(vec![1]), boundary);
        let first_tag = tags.len();
        let converted = if keys.is_empty() {
            body.clone()
        } else {
            convert_refs(body.clone(), &keys)
        };
        walk(
            &converted,
            &mut locator,
            &mut tags,
            &mut intr,
            &mut auto_label_counter,
            root_lang.as_ref(),
            &root_chain,
            None,
        );
        intr.counters.record_document_locations(
            tags[first_tag..]
                .iter()
                .filter_map(|tag| {
                    if let Tag::Start(location, _) = tag {
                        Some(*location)
                    } else {
                        None
                    }
                })
                .collect(),
        );
        body_tags.push((boundary, first_tag..tags.len()));
    }
    materialize_outline_entries(&mut locator, &mut intr);
    intr.parent_locations = build_parent_index(&tags);
    from_tags::apply_bundle_counter_funcs(&tags, &body_tags, &mut intr, engine, ctx)?;
    from_tags::apply_state_displays_checked(&tags, &mut intr, engine, ctx)?;
    from_tags::apply_counter_displays(&tags, &mut intr, engine, ctx);
    from_tags::apply_equation_numberings(&tags, &mut intr, engine, ctx)?;
    from_tags::apply_equation_supplements(&tags, &mut intr, engine, ctx)?;
    from_tags::apply_heading_supplements(&mut intr, engine, ctx)?;
    from_tags::realize_equation_elements(&tags, &mut intr);
    let paged: Vec<_> = bodies
        .iter()
        .zip(targets)
        .filter_map(|(body, target)| {
            (*target == crate::compiler::eval::EvalTarget::Paged).then_some(body)
        })
        .collect();
    let mut observer = EvalContext::new();
    observer.target = crate::compiler::eval::EvalTarget::Paged;
    observer.features = features;
    super::figure_counters::prepare_figure_counters_for_bodies(
        &paged, &mut intr, engine, &observer,
    )?;
    Ok((intr, starts))
}

/// Introspecção sobre `Content` final com os pós-processadores que requerem
/// runtime de avaliação. Não reavalia o documento.
pub fn introspect_with_runtime(
    content: &Content,
    engine: &mut crate::entities::engine::Engine<'_>,
    ctx: &mut crate::compiler::eval::EvalContext,
) -> crate::entities::source_result::SourceResult<TagIntrospector> {
    let content = convert_bib_refs_to_cites(content.clone());
    let mut locator = Locator::new();
    super::walk::reserve_locations(&content, &mut locator);
    let mut tags: Vec<Tag> = Vec::new();
    let mut intr = TagIntrospector::empty();
    let mut auto_label_counter = 0usize;
    let root_chain = StyleChain::default_chain();
    let root_lang = root_chain.lang();
    walk(
        &content,
        &mut locator,
        &mut tags,
        &mut intr,
        &mut auto_label_counter,
        root_lang.as_ref(),
        &root_chain,
        None,
    );
    materialize_outline_entries(&mut locator, &mut intr);
    intr.parent_locations = build_parent_index(&tags);
    from_tags::apply_counter_funcs(&tags, &mut intr, engine, ctx)?;
    from_tags::apply_state_displays_checked(&tags, &mut intr, engine, ctx)?;
    from_tags::apply_counter_displays(&tags, &mut intr, engine, ctx);
    from_tags::apply_equation_numberings(&tags, &mut intr, engine, ctx)?;
    from_tags::apply_equation_supplements(&tags, &mut intr, engine, ctx)?;
    from_tags::apply_heading_supplements(&mut intr, engine, ctx)?;
    from_tags::realize_equation_elements(&tags, &mut intr);
    super::figure_counters::prepare_figure_counters(&content, &mut intr, engine, ctx)?;
    Ok(intr)
}

/// Realize pending heading supplements without applying other runtime callbacks.
pub fn realize_heading_supplements(
    intr: &mut TagIntrospector,
    engine: &mut Engine<'_>,
    ctx: &mut EvalContext,
) -> SourceResult<()> {
    from_tags::apply_heading_supplements(intr, engine, ctx)
}
