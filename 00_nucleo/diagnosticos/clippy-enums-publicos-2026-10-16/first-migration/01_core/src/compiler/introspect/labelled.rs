//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/introspect/labelled.md
//! @prompt-hash b2276020
//! @layer L1
//! @updated 2026-08-18
//!
//! Atomização (ADR-0109, P383): a lógica de introspeção por-elemento de
//! `Label` (`compute_labelled` e braço completo do walk) vive neste arquivo
//! da unidade, na camada de introspecção. O match no tronco delega
//! estaticamente, preservando Location, recursão e ordem das tags.
//! **P1018** — chaves de counter migradas para `CounterKey`.

use crate::compiler::lang::heading_supplement_for_lang;
use crate::entities::content::Content;
use crate::entities::counter::CounterKey;
use crate::entities::element_info::ElementInfo;
use crate::entities::element_kind::ElementKind;
use crate::entities::element_payload::ElementPayload;
use crate::entities::elements::label::LabelElem;
use crate::entities::introspector::Introspector;
use crate::entities::introspector::TagIntrospector;
use crate::entities::label::Label;
use crate::entities::lang::Lang;
use crate::entities::location::Location;
use crate::entities::locator::Locator;
use crate::entities::selector::Selector;
use crate::entities::style_chain::StyleChain;
use crate::entities::tag::Tag;

use super::{classify_unreferencable_body, populate_intr_from_tag_start, walk};

/// Computa `(resolved_text, figure_number)` para uma `Label` apontando a um
/// `target` locatável (Heading/Equation/Figure), via o `Introspector`
/// location-aware. Função pura — sem mutação. (ADR-0069/P195D/P191C.)
pub(super) fn compute_labelled<I: Introspector>(
    intr: &I,
    location: Location,
    target: &Content,
    lang: Option<&crate::entities::lang::Lang>,
) -> (Option<String>, Option<usize>) {
    match target {
        Content::Heading(_) => {
            let heading_key = CounterKey::Selector(Selector::Kind(ElementKind::Heading));
            let supp = heading_supplement_for_lang(lang);
            (
                intr.formatted_counter_at(&heading_key, location)
                    .map(|n| format!("{} {}", supp, n)),
                None,
            )
        }
        Content::Equation(e) if e.block => {
            let equation_key =
                CounterKey::Selector(Selector::Kind(ElementKind::Equation));
            let n = intr.flat_counter_at(&equation_key, location).unwrap_or(0);
            if n > 0 {
                (Some(format!("Equação ({})", n)), None)
            } else {
                (None, None)
            }
        }
        Content::Figure(e) => {
            // F-5a de-bake (P365): o gate (padrão + caption) não é mais lido de um
            // campo — vive no **contador** `figure:{kind}`, que só é avançado quando
            // `is_counted` (padrão-da-chain && caption) na emissão (walk top). Logo
            // `flat_counter_at` location-aware devolve >0 só para figuras numeradas
            // (espelho do arm Equation, que confia no contador). Sem leitura de campo.
            let kind_key = e.kind.as_deref().unwrap_or("image");
            let counter_key = CounterKey::Str(format!("figure:{}", kind_key).into());
            let n = intr.flat_counter_at(&counter_key, location).unwrap_or(0);
            if n > 0 {
                let supplement =
                    crate::compiler::lang::figure_supplement::figure_supplement_for_lang(
                        kind_key, lang,
                    );
                (Some(format!("{} {}", supplement, n)), Some(n))
            } else {
                (Some(String::new()), None)
            }
        }
        _ => (None, None), // neutro: N16[γ] — Content sem suporte de labelling retorna (None, None) (conservador: novos nós rotuláveis exigirão arm)
    }
}

/// Percorre o alvo de Label e publica os registros derivados quando aplicáveis.
pub(super) fn walk_label(
    e: &LabelElem,
    locator: &mut Locator,
    tags: &mut Vec<Tag>,
    intr: &mut TagIntrospector,
    auto_label_counter: &mut usize,
    lang: Option<&Lang>,
    chain: &StyleChain,
) {
    let label = Label(e.name.to_string());
    if e.auto {
        // P464: Label auto-gerado (ex-sintaxe `<label>`). Mantém
        // comportamento legado de `Content::Labelled`: emite Tag
        // pós-recursão com texto resolvido / número de figura.
        let target = &e.body;
        let target_lang = chain.lang();
        let tags_len_before = tags.len();
        walk(target, locator, tags, intr, auto_label_counter, lang, chain, Some(&label));
        // P788: o `label_to_counter_key.remove(&label)` que aqui
        // existia forçava o caminho legacy — removido (precedência
        // numérica; legacy fica como fallback).

        let target_loc = tags[tags_len_before..].iter().find_map(|t| {
            if let Tag::Start(l, Box::new(_)) = t {
                Some(*l)
            } else {
                None
            }
        });

        let (resolved_text, figure_number) = match target_loc {
            Some(loc) => {
                compute_labelled(&*intr, loc, target, target_lang.as_ref().or(lang))
            }
            None => (None, None),
        };

        let _ = figure_number;

        if resolved_text.is_some() || figure_number.is_some() {
            if let Some(loc) = target_loc {
                let info = ElementInfo::new(ElementPayload::Labelled {
                    label,
                    resolved_text,
                    figure_number,
                });
                populate_intr_from_tag_start(intr, &info, loc);
                tags.push(Tag::Start(loc, Box::new(info)));
                tags.push(Tag::End(loc, 0));
            }
        } else if target_loc.is_none() {
            // P856 — label auto anexado a conteúdo não locatable
            // (texto, raw, etc.). O target não emitiu Tag, logo o
            // populate não teve oportunidade de registar a label.
            let kind = classify_unreferencable_body(target);
            intr.unreferencable_labels.insert(label, kind);
        }
    } else {
        // P460/P464: Label criado pelo utilizador. Propaga a label
        // para o body, permitindo que Ref resolva labels em
        // headings/figures/equations. Não emite Tag próprio quando
        // o body não é locatable — o destino PDF ainda é gerado pelo
        // layout independentemente.
        let known_before = intr.label_to_counter_key.contains_key(&label)
            || intr.figure_label_numbers.contains_key(&label)
            || intr.resolved_labels.get(&label).is_some()
            || intr.labels.lookup(&label).is_some()
            || intr.unreferencable_labels.contains_key(&label);
        walk(&e.body, locator, tags, intr, auto_label_counter, lang, chain, Some(&label));
        let known_after = intr.label_to_counter_key.contains_key(&label)
            || intr.figure_label_numbers.contains_key(&label)
            || intr.resolved_labels.get(&label).is_some()
            || intr.labels.lookup(&label).is_some()
            || intr.unreferencable_labels.contains_key(&label);
        // P856 — se o body não registou a label em nenhum caminho
        // referenciável, regista-a como não referenciável.
        if !known_before && !known_after {
            let kind = classify_unreferencable_body(&e.body);
            intr.unreferencable_labels.insert(label, kind);
        }
    }
}
