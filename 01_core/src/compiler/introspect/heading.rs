//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/introspect/heading.md
//! @prompt-hash 08e43dbf
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P383): a lógica de introspeção por-elemento de
//! `Heading` (snapshot, projeções TOC e braço completo do walk) vive neste
//! arquivo da unidade, na camada de introspecção. O match no tronco delega
//! estaticamente, preservando Location, recursão e ordem das tags.

use crate::compiler::lang::heading_supplement_for_lang;
use crate::entities::content::Content;
use crate::entities::counter::CounterKey;
use crate::entities::element_info::ElementInfo;
use crate::entities::element_kind::ElementKind;
use crate::entities::element_payload::ElementPayload;
use crate::entities::elements::heading::HeadingElem;
use crate::entities::introspector::Introspector;
use crate::entities::introspector::TagIntrospector;
use crate::entities::label::Label;
use crate::entities::lang::Lang;
use crate::entities::location::Location;
use crate::entities::locator::Locator;
use crate::entities::selector::Selector;
use crate::entities::style_chain::StyleChain;
use crate::entities::tag::Tag;
use crate::entities::value::Value;

use super::{materialize_time, populate_intr_from_tag_start, walk};

/// Capture the complete modeled language fields while the causal chain exists.
pub(super) fn snapshot_fields(
    heading: &crate::entities::elements::heading::HeadingElem,
    chain: &crate::entities::style_chain::StyleChain,
    label: Option<&Label>,
) -> indexmap::IndexMap<
    ecow::EcoString,
    crate::entities::value::Value,
    rustc_hash::FxBuildHasher,
> {
    use crate::entities::elements::heading::HEADING_SET_DEPTH;
    use crate::entities::value::Value;

    let numbering =
        if matches!(chain.custom("heading.numbering"), Some(Value::Bool(true))) {
            match chain.custom("heading.numbering.pattern") {
                Some(Value::Str(pattern)) => Value::Str(pattern.clone()),
                _ => Value::None,
            }
        } else {
            Value::None
        };
    let lang = chain.lang().unwrap_or(crate::entities::lang::Lang::ENGLISH);
    let supplement = heading_supplement_for_lang(Some(&lang));
    let mut fields = indexmap::IndexMap::default();
    fields.insert("level".into(), Value::Int(heading.level as i64));
    fields.insert(
        "depth".into(),
        Value::Int(if heading.set_fields & HEADING_SET_DEPTH != 0 {
            heading.level as i64
        } else {
            1
        }),
    );
    fields.insert("offset".into(), Value::Int(0));
    fields.insert("numbering".into(), numbering);
    fields.insert("supplement".into(), Value::Content(Content::text(supplement)));
    fields.insert("outlined".into(), Value::Bool(heading.outlined));
    fields.insert(
        "bookmarked".into(),
        heading.bookmarked.map(Value::Bool).unwrap_or(Value::Auto),
    );
    fields.insert("hanging-indent".into(), Value::Auto);
    fields.insert("body".into(), Value::Content(heading.body.clone()));
    if let Some(label) = label {
        fields.insert("label".into(), Value::Label(label.clone()));
    }
    fields
}

/// Formata o valor hierárquico de um counter como string terminada em ponto.
///
/// Usado para o número do outline (DEBT-60b / P428): emite `"1."`, `"1.1."`,
/// etc., sem supplement. Retorna `None` se a numeração estiver inactiva ou o
/// counter não tiver valor no momento da `location`.
fn format_heading_number<I: Introspector>(
    intr: &I,
    location: Location,
    numbering_active: bool,
) -> Option<String> {
    if !numbering_active {
        return None;
    }
    let heading_key = CounterKey::Selector(Selector::Kind(ElementKind::Heading));
    intr.formatted_counter_at(&heading_key, location)
        .map(|n| format!("{}.", n))
}

/// Computa `(auto_label, resolved_text)` para a auto-toc de um `Heading`
/// (ADR-0069/P196B). Função pura sobre `(intr, location, auto_label_n,
/// numbering_active)` — sem mutação. Sempre retorna `(Label, String)`
/// (resolved_text vazio quando numbering inactivo — paridade legacy).
pub(super) fn compute_heading_auto_toc<I: Introspector>(
    intr: &I,
    location: Location,
    auto_label_n: usize,
    numbering_active: bool,
) -> (Label, String) {
    let auto_label = Label(format!("auto-toc-{}", auto_label_n));
    // Lote F-2 S5 (P335): gate pelo `numbering_active` **assado** no
    // `HeadingElem` (escopo léxico via chain).
    let heading_key = CounterKey::Selector(Selector::Kind(ElementKind::Heading));
    let resolved_text = if numbering_active {
        // P359 (DEBT-60 b): o número do heading, **sem** o supplement "Secção" — o
        // outline mostra o numbering (paridade vanilla). Formato `{n}.` espelha o
        // corpo do heading (o nº do outline == o nº do corpo).
        intr.formatted_counter_at(&heading_key, location)
            .map(|n| format!("{}.", n))
            .unwrap_or_default()
    } else {
        String::new()
    };
    (auto_label, resolved_text)
}

/// Projecta a entry de outline para um `Heading` (ADR-0069/P200B). Função pura
/// sobre `(intr, location, auto_label_n, frozen_body, level, numbering_active)`
/// — sem mutação. `frozen_body` já materializado pelo walk arm Heading. O
/// `number` é computado a partir do counter do Introspector, separando-o do
/// body e do supplement (DEBT-60b / P428). Sempre retorna `Some(...)` (paridade
/// com o push incondicional legacy).
pub(super) fn compute_heading_for_toc<I: Introspector>(
    intr: &I,
    location: Location,
    auto_label_n: usize,
    frozen_body: Content,
    level: usize,
    numbering_active: bool,
) -> Option<(Label, Option<String>, Content, usize)> {
    let auto_label = Label(format!("auto-toc-{}", auto_label_n));
    let number = format_heading_number(intr, location, numbering_active);
    Some((auto_label, number, frozen_body, level))
}

/// Percorre um Heading após o Start real ter sido emitido pelo walk pai.
#[expect(
    clippy::too_many_arguments,
    reason = "C009: Free function descendente da feature Heading sob ADR-0109: conserva os acumuladores emprestados e a chain causal, delegando a recursão ao walk"
)]
pub(super) fn walk_heading(
    h: &HeadingElem,
    emitted_loc: Option<Location>,
    locator: &mut Locator,
    tags: &mut Vec<Tag>,
    intr: &mut TagIntrospector,
    auto_label_counter: &mut usize,
    lang: Option<&Lang>,
    chain: &StyleChain,
) {
    // Modelo D (P316): Heading delegado; re-bind dos campos.
    let level = &h.level;
    let body = &h.body;
    let _ = level;
    // P363 (introspect-chain, PROBE de verificação): o gate de numbering
    // lido **da chain** neste heading. **Aditivo** — não muda o read do
    // gate (continua `h.numbering_active` até o de-bake P364). Só captura,
    // sob `#[cfg(test)]`, o que a chain entrega aqui, para o teste provar que
    // a infra threadou o custom até o heading (um heading sob o `Content::
    // Styled` de `#set heading(numbering:)` vê `Some(Bool(true))`).
    #[cfg(test)]
    super::introspect_chain_probe::record(matches!(
        chain.custom("heading.numbering"),
        Some(Value::Bool(true))
    ));
    // P200B (M5 universal completo) — walk arm Heading
    // E2-residuo fechada estruturalmente. Trabalho híbrido
    // combinando 3 padrões testados:
    // - sub-store novo `intr.headings_for_toc` (P193B-style),
    // - Tag pós-recursão `ElementPayload::HeadingForToc`
    //   (variante P196B locatable + emitted_loc directo),
    // - consumer outline.rs:24 substitution-with-fallback
    //   (P184D / P194B style).
    //
    // P196B (referência histórica): 3 das 4 mutações E2
    // migraram estruturalmente via Tag::Labelled auto-toc;
    // a 4ª (`state.headings_for_toc.push`) era residual.
    // P200B fecha esse residuo via Tag::HeadingForToc
    // (3ª Tag pós-recursão, mesma `emitted_loc`).
    //
    // Mutação 4 legacy preservada como write paralelo M5
    // porque Layouter assignments (`mod.rs:1490, 1521`)
    // dependem de `state.headings_for_toc`. Cleanup
    // orgânico em M6.
    // P190I (M6 fechado): mutação `state.step_hierarchical`
    // ELIMINADA — caminho Introspector activo via populate_intr
    // arm Heading (P191B `apply_hierarchical_at`). intr.counters
    // populated antes desta arm via walk top emission. State
    // legacy struct eliminada.

    // P190G (M6 categoria Labels & TOC): `state.auto_label_counter`
    // eliminado — substituído por local var threaded via
    // `auto_label_counter: &mut usize` parameter (Opção α).
    *auto_label_counter += 1;
    let current_auto_label = *auto_label_counter;
    // P191B (ADR-0071): compute_heading_auto_toc lê via
    // Introspector path location-aware. `emitted_loc` é
    // `Some(loc)` para Heading (locatable; populate_intr no
    // walk top já registou Heading kind + counter no intr).
    let auto_loc = emitted_loc.expect("Heading é locatable — emitted_loc deve ser Some");
    // F-5a de-bake (P364, §3a.9): o gate vive **só na chain** (lido aqui
    // sobre o introspect-chain do P363). O campo assado `numbering_active`
    // foi removido. O **valor** do contador (`apply_hierarchical_at`,
    // incondicional P335) e `formatted_counter_at` ficam intactos.
    let numbering_active =
        matches!(chain.custom("heading.numbering"), Some(Value::Bool(true)));
    let (auto_label, resolved_text) =
        compute_heading_auto_toc(&*intr, auto_loc, current_auto_label, numbering_active);
    // P190G: mutação `state.resolved_labels.insert` ELIMINADA
    // — caminho Introspector activo via Tag::Labelled
    // pós-recursão (populate_intr_from_tag_start arm Labelled
    // popula intr.resolved_labels). Layouter consumer
    // `references.rs:64` migrado para Introspector path puro
    // (sem fallback legacy).

    // P190G: mutação `state.headings_for_toc.push` ELIMINADA
    // — caminho Introspector activo via Tag::HeadingForToc
    // pós-recursão (populate_intr_from_tag_start arm
    // HeadingForToc popula intr.headings_for_toc). Layouter
    // consumer `outline.rs:38` migrado para Introspector path
    // puro (sem fallback legacy).
    //
    // `frozen_body` ainda computed aqui — reusado na Tag
    // HeadingForToc pós-recursão (P200B).
    let frozen_body = materialize_time(body, &*intr, auto_loc);

    walk(body, locator, tags, intr, auto_label_counter, lang, chain, None);

    // P196B: emit Tag auto-toc pós-recursão (ADR-0069).
    // Reusa Location alocada para Heading (locatable; walk
    // top emitiu Tag::Start). `emitted_loc` é `Some(loc)` —
    // mais simples que P195D (Labelled não-locatable
    // exigiu snapshot+find_map).
    //
    // P191B (ADR-0071): popula intr directamente via
    // populate_intr_from_tag_start no momento da emissão.
    if let Some(loc) = emitted_loc {
        let info = ElementInfo::new(ElementPayload::Labelled {
            label: auto_label.clone(),
            resolved_text: Some(resolved_text),
            figure_number: None,
        });
        populate_intr_from_tag_start(intr, &info, loc);
        tags.push(Tag::Start(loc, Box::new(info)));
        tags.push(Tag::End(loc, 0));
    }

    // P200B/P606: emit Tag::HeadingForToc e/ou Tag::HeadingForBookmarks
    // pós-recursão. `outlined` controla o índice do documento;
    // `bookmarked` (explicitamente ou via `outlined`) controla a árvore
    // `/Outlines` do PDF. Ambas partilham o mesmo cálculo de entrada.
    if let Some(loc) = emitted_loc {
        if let Some((label, number, body_for_toc, lvl)) = compute_heading_for_toc(
            &*intr,
            loc,
            current_auto_label,
            frozen_body,
            *level as usize,
            numbering_active,
        ) {
            if h.outlined {
                intr.outlined_heading_locations.push(loc);
                let info = ElementInfo::new(ElementPayload::HeadingForToc {
                    label: label.clone(),
                    number: number.clone(),
                    body: body_for_toc.clone(),
                    level: lvl,
                });
                populate_intr_from_tag_start(intr, &info, loc);
                tags.push(Tag::Start(loc, Box::new(info)));
                tags.push(Tag::End(loc, 0));
            }
            if h.is_bookmarked() {
                let info = ElementInfo::new(ElementPayload::HeadingForBookmarks {
                    label,
                    number,
                    body: body_for_toc,
                    level: lvl,
                });
                populate_intr_from_tag_start(intr, &info, loc);
                tags.push(Tag::Start(loc, Box::new(info)));
                tags.push(Tag::End(loc, 0));
            }
        }
    }
}
