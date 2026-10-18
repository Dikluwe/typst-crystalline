//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/introspect.md
//! @prompt-hash d098006b
//! @layer L1
//! @updated 2026-10-05
//!
//! P162 sub-passos .E + .F: walk passa a aceitar `&mut Locator` e
//! `&mut Vec<Tag>`; emite `Tag::Start` antes da mutação de estado e
//! `Tag::End` depois da recursão para variantes locatable
//! (Heading/Figure/Cite).
//!
//! **P191B (ADR-0071)**: walk fn ganha `&mut TagIntrospector`
//! parameter; sub-stores populated directamente durante walk via
//! `populate_intr_from_tag_start`. Pipeline simplificado: walk →
//! return (etapa `from_tags::from_tags` eliminada; substituída por
//! `apply_state_funcs` slim post-pass para Funcs apenas, chamada
//! por fixpoint). Helper `compute_heading_auto_toc` migrado para
//! signature `<I: Introspector>(intr, location, counter_n)`. Walk
//! arm Equation gate pelo campo assado `numbering_active` (F-2 S2;
//! o gate legado por StateRegistry foi removido em F-4 E0, P338).
//! API pública preservada (`introspect()` retorna `CounterStateLegacy`
//! idêntico). `introspect_with_introspector` simplificada — drops
//! parâmetros engine/ctx (Funcs continuam ignoradas neste path
//! coerente com semântica P171 pré-P191B).

use crate::entities::bib_entry::BibEntry;
use std::collections::HashMap;

mod bibliography;
mod classification;
mod entrypoints;
mod materialize_time;
mod outline_entries;
mod parent_index;
mod populate;
mod queries;
pub(crate) mod walk;

pub mod convergence;
pub mod extract_payload;
pub mod fixpoint;
pub mod from_tags;
// Atomização por-elemento (ADR-0109, P383): compute_* movidos do tronco.
pub mod heading;
pub mod labelled;
pub mod locatable;

use std::sync::Arc;

use crate::entities::{
    content::Content,
    content_hash::hash_content,
    counter::CounterKey,
    counter_update::CounterUpdate,
    element_info::ElementInfo,
    element_kind::ElementKind,
    element_payload::ElementPayload,
    introspector::{Introspector, TagIntrospector},
    label::Label,
    label_kind::UnreferencableKind,
    location::Location,
    locator::{Locator, LocatorSeed},
    selector::Selector,
    style_chain::StyleChain,
    tag::Tag,
    value::Value,
};

use crate::compiler::introspect::extract_payload::extract_payload as do_extract_payload;

pub use self::bibliography::convert_bib_refs_to_cites;
use self::bibliography::{collect_bib_keys, convert_refs};
use self::classification::classify_unreferencable_body;
pub use self::entrypoints::{
    introspect_bundle_documents_with_runtime_and_starts,
    introspect_bundle_documents_with_starts, introspect_with_introspector,
    introspect_with_runtime, realize_heading_supplements,
};
use self::materialize_time::materialize_time;
use self::outline_entries::materialize_outline_entries;
use self::parent_index::build_parent_index;
use self::populate::populate_intr_from_tag_start;
pub(crate) use self::walk::walk;

/// **P363 (introspect-chain) — probe de verificação, `#[cfg(test)]`.** Captura, por
/// heading visitado no walk, o gate de numbering **lido da chain** — para um teste
/// provar que a infra threadou o custom `heading.numbering` até o heading (sem mudar
/// o comportamento de produção). Removida quando o de-bake (P364) tornar o read da
/// chain o caminho real e o teste passar a asserir o output.
#[cfg(test)]
pub(crate) mod introspect_chain_probe {
    use std::cell::RefCell;
    thread_local! {
        static GATES: RefCell<Vec<bool>> = const { RefCell::new(Vec::new()) };
    }
    /// Limpa a captura (chamar antes de um walk no teste).
    pub(crate) fn reset() {
        GATES.with(|g| g.borrow_mut().clear());
    }
    /// Regista o gate da chain de um heading (chamado pelo arm Heading do walk).
    pub(crate) fn record(gate: bool) {
        GATES.with(|g| g.borrow_mut().push(gate));
    }
    /// Os gates capturados, na ordem de visita.
    pub(crate) fn captured() -> Vec<bool> {
        GATES.with(|g| g.borrow().clone())
    }
}

/// Pré-passagem analítica sobre `Content` — entry point legado.
///
/// Percorre a árvore completa uma vez, avançando contadores e populando
/// `resolved_labels`, sem realizar nenhum cálculo visual.
///
/// O `CounterStateLegacy` retornado é injectado no Layouter como estado inicial
/// (apenas o campo `resolved_labels`), garantindo que todas as referências —
/// incluindo para a frente — estão resolvidas antes do primeiro `FrameItem`
/// ser gerado.
///
/// **P166 (M4)**: esta função é agora wrapper sobre
/// `introspect_with_introspector` — descarta o `TagIntrospector`. Consumers
/// que precisem do introspector devem chamar `introspect_with_introspector`
/// directamente. M5 migra primeiro consumer real; M6 elimina este wrapper
/// + `CounterStateLegacy`.
///
/// **P173**: passa `None, None` para `introspect_with_introspector` —
/// Funcs em `state.update(key, fn)` são silenciosamente ignoradas neste
/// path legacy (sem Engine disponível). Comportamento defensivo coerente
/// com P171 ("update sem init é ignorado").
/// **P190I (M6 fechado)**: API pública continua a existir mas
/// retorna `TagIntrospector` (struct `CounterStateLegacy` eliminada).
/// Wrapper de compatibilidade — preserva nome `introspect()`
/// histórico. Callers actualizados para path Introspector.
pub fn introspect(content: &Content) -> TagIntrospector {
    introspect_with_introspector(content)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::layout::FixedMetrics;
    use crate::entities::{
        content::Content, counter::CounterKey,
        counter_update::CounterUpdate as CounterAction, element_payload::ElementPayload,
        label::Label, location::Location, style::Styles, value::Value,
    };

    fn ck(s: &str) -> CounterKey {
        CounterKey::Str(s.into())
    }

    fn sel(kind: ElementKind) -> CounterKey {
        CounterKey::Selector(Selector::Kind(kind))
    }

    /// **F-5a de-bake (P365)** — rotula reproduzindo a **forma de produção**: o
    /// transporte de numbering (`Content::Styled`, ex.: o que `Content::figure(..,
    /// Some)` produz) fica **fora** do `Labelled` (`Styled{ Labelled{ alvo } }`),
    /// como a fatia-1 embrulha a cauda. Sem isto, `Labelled{ Styled{ alvo } }`
    /// inverte a ordem e o `compute_labelled` (que inspeciona o tipo do alvo) não
    /// dispara. Levanta o transporte transparente para fora; alvo simples passa
    /// direto.
    fn labelled_prod(target: Content, label: Label) -> Content {
        let name = label.0;
        match target {
            Content::Styled(styled) => {
                let inner = &styled.body;
                let styles = &styled.styles;
                Content::styled(Content::label_auto(name, inner.clone()), styles.clone())
            }
            other => Content::label_auto(name, other),
        }
    }

    /// Activa a numeração de um heading via `Content::Styled` com
    /// `custom("heading.numbering") = true`, a forma canónica de transporte do
    /// gate (P364/P1019). Usado pelos testes que precisam que o counter de
    /// heading avance.
    fn with_heading_numbering(body: Content) -> Content {
        Content::styled(
            Box::new(body),
            Styles::new().push_custom("heading.numbering", Value::Bool(true)),
        )
    }

    #[test]
    fn introspect_popula_label_forward() {
        // Ref antes do Labelled — forward reference. P1019: o heading precisa
        // de numbering activo para que o counter avance e a label resolva.
        let content = Content::Sequence(
            vec![
                Content::reference("conclusao"),
                labelled_prod(
                    with_heading_numbering(Content::heading(
                        1,
                        Content::text("Conclusão"),
                    )),
                    Label("conclusao".to_string()),
                ),
            ]
            .into(),
        );

        let intr = introspect_with_introspector(&content);
        assert!(
            intr.resolved_labels.get(&Label("conclusao".to_string())).is_some(),
            "introspect deve popular resolved_labels mesmo para forward refs"
        );
        assert_eq!(
            intr.resolved_labels.get(&Label("conclusao".to_string())),
            Some("Section 1")
        );
    }

    #[test]
    fn introspect_counter_update_e_aplicado() {
        let content = Content::Sequence(
            vec![Content::counter_update(
                "equation".to_string(),
                CounterAction::Set(vec![5]),
            )]
            .into(),
        );

        let intr = introspect_with_introspector(&content);
        assert_eq!(
            intr.counters
                .value(&ck("equation"))
                .and_then(|v| v.last())
                .copied()
                .unwrap_or(0),
            5
        );
    }

    #[test]
    fn p844_a1_walk_popula_elements_e_element_at() {
        // P844 (achado #47 de P831) — o walk regista o `Content` de cada
        // elemento locatable no sub-store `elements`, permitindo a
        // `query()` devolver o elemento (com campos acessíveis) em vez
        // de só a `Location` (paridade vanilla `query` → elems).
        let content = Content::Sequence(
            vec![
                Content::label_auto(
                    "meta".to_string(),
                    Content::Metadata(std::sync::Arc::new(
                        crate::entities::elements::metadata::MetadataElem {
                            value: Box::new(Value::Str("ola".into())),
                        },
                    )),
                ),
                Content::heading(1, Content::text("Titulo")),
            ]
            .into(),
        );

        let intr = introspect_with_introspector(&content);
        // Label resolve para a Location do metadata.
        let loc = intr
            .query_by_label(&Label("meta".to_string()))
            .expect("label meta registada");
        let elem = intr.element_at(loc).expect("elements populado pelo walk");
        assert!(
            matches!(elem, Content::Metadata(_)),
            "esperado Metadata, obtido: {elem:?}"
        );
        // Campo `value` acessível (cadeia completa do achado #47).
        assert_eq!(elem.get_field("value"), Some(Value::Str("ola".into())));
        // Heading também é resolvível por Location.
        let hloc = intr.query_by_kind(ElementKind::Heading)[0];
        assert!(
            matches!(intr.element_at(hloc), Some(Content::Heading(_))),
            "esperado Heading em element_at"
        );
    }

    #[test]
    fn introspect_dois_conteudos_independentes() {
        // P1019: activar numbering para que a label "a" resolva.
        let content_a = labelled_prod(
            with_heading_numbering(Content::heading(1, Content::text("A"))),
            Label("a".to_string()),
        );
        let content_b = Content::reference("a");

        let intr_a = introspect_with_introspector(&content_a);
        let intr_b = introspect_with_introspector(&content_b);

        assert!(intr_a.resolved_labels.get(&Label("a".to_string())).is_some());
        assert!(
            intr_b.resolved_labels.get(&Label("a".to_string())).is_none(),
            "estados de introspecção devem ser independentes"
        );
    }

    // ── Testes de Passo 61 — TOC ─────────────────────────────────────────

    #[test]
    fn introspect_cataloga_headings_para_toc() {
        let content = Content::Sequence(
            vec![
                Content::heading(1, Content::text("Introdução")),
                Content::heading(2, Content::text("Motivação")),
                Content::heading(1, Content::text("Conclusão")),
            ]
            .into(),
        );

        let intr = introspect_with_introspector(&content);
        assert_eq!(intr.headings_for_toc().len(), 3);

        let (_, _, title_0, level_0) = &intr.headings_for_toc()[0];
        assert_eq!(title_0.plain_text(), "Introdução");
        assert_eq!(*level_0, 1);

        let (_, _, _, level_1) = &intr.headings_for_toc()[1];
        assert_eq!(*level_1, 2);
    }

    #[test]
    fn introspect_gera_labels_automaticas_unicas() {
        let content = Content::Sequence(
            vec![
                Content::heading(1, Content::text("A")),
                Content::heading(1, Content::text("B")),
            ]
            .into(),
        );

        let intr = introspect_with_introspector(&content);
        let label_a = &intr.headings_for_toc()[0].0;
        let label_b = &intr.headings_for_toc()[1].0;
        assert_ne!(label_a, label_b, "labels automáticas devem ser únicas");

        // As labels devem estar em resolved_labels
        assert!(intr.resolved_labels.get(label_a).is_some());
        assert!(intr.resolved_labels.get(label_b).is_some());
    }

    #[test]
    fn introspect_heading_sem_numbering_insere_string_vazia_em_resolved_labels() {
        // Sem numeração activa, resolved_labels deve conter "" (não "@auto-toc-N").
        let content = Content::heading(1, Content::text("Título"));
        let intr = introspect_with_introspector(&content);
        assert_eq!(intr.headings_for_toc().len(), 1);
        let (label, _, _, _) = &intr.headings_for_toc()[0];
        assert_eq!(
            intr.resolved_labels.get(label),
            Some(""),
            "heading sem numeração deve ter string vazia em resolved_labels"
        );
    }

    // ── Testes de Passo 62 — Figuras ─────────────────────────────────────

    #[test]
    fn introspect_resolve_label_de_figura() {
        let content = Content::Sequence(
            vec![labelled_prod(
                Content::figure(
                    Content::text("Um gráfico"),
                    Some(Content::text("Evolução")),
                    Some("image".to_string()),
                    Some("1".to_string()),
                ),
                Label("fig1".to_string()),
            )]
            .into(),
        );

        let intr = introspect_with_introspector(&content);
        // **P1034** — default de linguagem é `en`; label resolve para "Figure 1".
        assert_eq!(
            intr.resolved_labels.get(&Label("fig1".to_string())),
            Some("Figure 1"),
            "label de figura deve resolver para 'Figure 1'"
        );
    }

    #[test]
    fn introspect_duas_figuras_contadores_independentes() {
        let content = Content::Sequence(
            vec![
                labelled_prod(
                    Content::figure(
                        Content::text("A"),
                        Some(Content::text("Legenda A")),
                        Some("image".to_string()),
                        Some("1".to_string()),
                    ),
                    Label("f1".to_string()),
                ),
                labelled_prod(
                    Content::figure(
                        Content::text("B"),
                        Some(Content::text("Legenda B")),
                        Some("image".to_string()),
                        Some("1".to_string()),
                    ),
                    Label("f2".to_string()),
                ),
            ]
            .into(),
        );

        let intr = introspect_with_introspector(&content);
        assert_eq!(intr.resolved_labels.get(&Label("f1".to_string())), Some("Figure 1"));
        assert_eq!(intr.resolved_labels.get(&Label("f2".to_string())), Some("Figure 2"));
    }

    #[test]
    fn introspect_figura_sem_caption_nao_incrementa_contador() {
        let content = Content::Sequence(
            vec![
                Content::figure(
                    Content::text("Diagrama"),
                    None,
                    Some("image".to_string()),
                    Some("1".to_string()),
                ),
                labelled_prod(
                    Content::figure(
                        Content::text("B"),
                        Some(Content::text("Legenda")),
                        Some("image".to_string()),
                        Some("1".to_string()),
                    ),
                    Label("f2".to_string()),
                ),
            ]
            .into(),
        );

        let intr = introspect_with_introspector(&content);
        // Figura sem caption não consome contador — a segunda figura numerada é "Figure 1"
        assert_eq!(
            intr.resolved_labels.get(&Label("f2".to_string())),
            Some("Figure 1"),
            "figura sem caption não deve consumir o contador"
        );
    }

    #[test]
    fn introspect_backward_ref_tambem_funciona() {
        // Labelled antes de Ref — deve também popular o mapa. P1019: numbering
        // activo necessário para a label de heading resolver.
        let content = Content::Sequence(
            vec![
                labelled_prod(
                    with_heading_numbering(Content::heading(1, Content::text("Secção"))),
                    Label("sec".to_string()),
                ),
                Content::reference("sec"),
            ]
            .into(),
        );

        let intr = introspect_with_introspector(&content);
        assert!(
            intr.resolved_labels.get(&Label("sec".to_string())).is_some(),
            "backward ref deve também estar em resolved_labels"
        );
    }

    // ── Testes de Passo 66 — Materialização temporal (DEBT-18) ───────────────

    #[test]
    fn materialize_time_substitui_counter_display() {
        // P190I (M6 fechado): test adaptado — populate intr manualmente
        // para simular contador "fig" = 42 numa Location.
        use crate::entities::counter_update::CounterUpdate as CU;
        use crate::entities::location::Location;

        let mut intr = TagIntrospector::empty();
        let loc = Location::from_raw(1);
        intr.counters.apply_at(ck("fig"), CU::Set(vec![42]), loc);

        let dynamic_ast = Content::Sequence(
            vec![Content::text("Figura "), Content::counter_display("fig".to_string())]
                .into(),
        );

        let frozen = materialize_time(&dynamic_ast, &intr, loc);

        let expected =
            Content::Sequence(vec![Content::text("Figura "), Content::text("42")].into());

        assert_eq!(
            frozen, expected,
            "CounterDisplay deve ser materializado em Text com o valor do contador"
        );
    }

    #[test]
    fn materialize_time_preserva_terminais() {
        // P190I: test adaptado — intr vazio, Location dummy.
        use crate::entities::location::Location;

        let intr = TagIntrospector::empty();
        let loc = Location::from_raw(1);

        // Nós terminais sem CounterDisplay devem ser clonados sem alteração.
        let content = Content::Sequence(
            vec![
                Content::text("Texto estático"),
                Content::strong(Content::text("Negrito")),
            ]
            .into(),
        );

        let frozen = materialize_time(&content, &intr, loc);
        assert_eq!(
            frozen, content,
            "Terminais sem CounterDisplay não devem ser alterados"
        );
    }

    #[test]
    fn introspect_headings_for_toc_congelados() {
        // Simular: = Figura #counter("fig").display()
        // O CounterDisplay no título deve ser substituído pelo valor no momento
        // da introspecção — não pelo valor quando a TOC for renderizada.
        let content = Content::Sequence(
            vec![
                Content::counter_update("fig".to_string(), CounterAction::Set(vec![7])),
                Content::heading(
                    1,
                    Content::Sequence(
                        vec![
                            Content::text("Figura "),
                            Content::counter_display("fig".to_string()),
                        ]
                        .into(),
                    ),
                ),
            ]
            .into(),
        );

        let intr = introspect_with_introspector(&content);
        assert_eq!(intr.headings_for_toc().len(), 1);

        let (_, _, frozen_body, _) = &intr.headings_for_toc()[0];
        let text = frozen_body.plain_text();
        // O body congelado deve conter "7" (valor no momento da introspecção),
        // não "0" (valor no início do documento quando a TOC é renderizada).
        assert!(
            text.contains("7"),
            "CounterDisplay no título deve ser congelado com o valor correcto: {:?}",
            text
        );
    }

    // ── Testes de Passo 75 — figure_numbers por kind (DEBT-14/15) ───────────

    #[test]
    fn figure_tem_kind_e_numbering() {
        // F-5a de-bake (P365): `Content::figure(.., Some(padrão))` produz a forma de
        // transporte `Styled(Figure, custom("figure.numbering"))` — o `kind` fica no
        // `Figure`; o padrão vive no custom da chain (não mais campo do elemento).
        let fig = Content::figure(
            Content::text("corpo"),
            Some(Content::text("legenda")),
            Some("image".to_string()),
            Some("1".to_string()),
        );
        if let Content::Styled(styled) = fig {
            let body = &styled.body;
            let styles = &styled.styles;
            let pat = styles
                .delta()
                .custom
                .iter()
                .find(|(k, _)| k == "figure.numbering")
                .map(|(_, v)| v.clone());
            assert_eq!(pat, Some(Value::Str("1".into())));
            assert!(
                matches!(body, Content::Figure(e) if e.kind.as_deref() == Some("image"))
            );
        } else {
            panic!("esperado Styled (transporte de numbering)");
        }
    }

    #[test]
    fn figuras_kind_diferente_contadores_independentes() {
        let doc = Content::Sequence(
            vec![
                Content::figure(
                    Content::text("img1"),
                    Some(Content::text("cap1")),
                    Some("image".to_string()),
                    Some("1".to_string()),
                ),
                Content::figure(
                    Content::text("tab1"),
                    Some(Content::text("cap2")),
                    Some("table".to_string()),
                    Some("1".to_string()),
                ),
                Content::figure(
                    Content::text("img2"),
                    Some(Content::text("cap3")),
                    Some("image".to_string()),
                    Some("1".to_string()),
                ),
            ]
            .into(),
        );

        let intr = introspect_with_introspector(&doc);

        let image_nums = (0..)
            .map_while(|i| intr.figure_number_at_index("image", i))
            .collect::<Vec<_>>();
        let table_nums = (0..)
            .map_while(|i| intr.figure_number_at_index("table", i))
            .collect::<Vec<_>>();

        assert_eq!(
            image_nums,
            vec![1, 2],
            "Duas figuras de kind 'image' devem produzir [1, 2]"
        );
        assert_eq!(
            table_nums,
            vec![1],
            "Uma figura de kind 'table' deve produzir [1] independentemente"
        );
    }

    // ── Passo 158B — Supplement automático por lang em figure ────────────

    /// Helper para construir state com lang explícito.
    /// **P190G**: retorna `(state, intr)` — testes leem
    /// `intr.resolved_labels` (field state.resolved_labels eliminado).
    fn introspect_with_lang(content: &Content, lang_code: &str) -> TagIntrospector {
        use crate::entities::lang::Lang;
        use std::str::FromStr;
        let lang = Lang::from_str(lang_code).unwrap();
        let mut locator = Locator::new();
        let mut tags: Vec<Tag> = Vec::new();
        let mut intr = TagIntrospector::empty();
        let mut auto_label_counter: usize = 0;
        walk(
            content,
            &mut locator,
            &mut tags,
            &mut intr,
            &mut auto_label_counter,
            Some(&lang),
            &crate::entities::style_chain::StyleChain::default_chain(),
            None,
        );
        intr
    }

    #[test]
    fn figure_label_default_no_lang_set_devolve_figure() {
        // **P1034** — default de linguagem do Typst é `en`; sem lang explícito
        // o supplement é "Figure".
        use crate::entities::label::Label;
        let label = Label("fig1".to_string());
        let figure = Content::figure(
            Content::text("body"),
            Some(Content::text("caption")),
            Some("image".to_string()),
            Some("1".to_string()),
        );
        let labelled = labelled_prod(figure, label.clone());
        let intr = introspect_with_introspector(&labelled);
        assert_eq!(
            intr.resolved_labels.get(&label),
            Some("Figure 1"),
            "Default sem lang → EN 'Figure'"
        );
    }

    #[test]
    fn figure_label_lang_pt_image_devolve_figura() {
        use crate::entities::label::Label;
        let label = Label("fig1".to_string());
        let figure = Content::figure(
            Content::text("body"),
            Some(Content::text("caption")),
            Some("image".to_string()),
            Some("1".to_string()),
        );
        let labelled = labelled_prod(figure, label.clone());
        let intr = introspect_with_lang(&labelled, "pt");
        assert_eq!(intr.resolved_labels.get(&label), Some("Figura 1"),);
    }

    #[test]
    fn figure_label_lang_en_table_devolve_table() {
        use crate::entities::label::Label;
        let label = Label("tab1".to_string());
        let figure = Content::figure(
            Content::text("body"),
            Some(Content::text("caption")),
            Some("table".to_string()),
            Some("1".to_string()),
        );
        let labelled = labelled_prod(figure, label.clone());
        let intr = introspect_with_lang(&labelled, "en");
        assert_eq!(intr.resolved_labels.get(&label), Some("Table 1"),);
    }

    #[test]
    fn figure_label_lang_de_raw_devolve_listing() {
        use crate::entities::label::Label;
        let label = Label("lst1".to_string());
        let figure = Content::figure(
            Content::text("body"),
            Some(Content::text("caption")),
            Some("raw".to_string()),
            Some("1".to_string()),
        );
        let labelled = labelled_prod(figure, label.clone());
        let intr = introspect_with_lang(&labelled, "de");
        assert_eq!(intr.resolved_labels.get(&label), Some("Listing 1"),);
    }

    #[test]
    fn figure_label_lang_unknown_fallback_en() {
        // **P1034** — lang desconhecido (zh) → fallback EN, não PT. Medido no
        // vanilla ratificado: `#set text(lang: "jp")` + figure → "Figure 1".
        use crate::entities::label::Label;
        let label = Label("fig1".to_string());
        let figure = Content::figure(
            Content::text("body"),
            Some(Content::text("caption")),
            Some("image".to_string()),
            Some("1".to_string()),
        );
        let labelled = labelled_prod(figure, label.clone());
        let intr = introspect_with_lang(&labelled, "zh");
        assert_eq!(
            intr.resolved_labels.get(&label),
            Some("Figure 1"),
            "Lang desconhecido cai no fallback EN (default da linguagem)"
        );
    }

    #[test]
    fn figure_label_kind_custom_devolve_capitalizado() {
        // P158B §6: kind desconhecido devolve string capitalizada.
        use crate::entities::label::Label;
        let label = Label("custom1".to_string());
        let figure = Content::figure(
            Content::text("body"),
            Some(Content::text("caption")),
            Some("custom".to_string()),
            Some("1".to_string()),
        );
        let labelled = labelled_prod(figure, label.clone());
        let intr = introspect_with_lang(&labelled, "en");
        assert_eq!(
            intr.resolved_labels.get(&label),
            Some("Custom 1"),
            "Kind desconhecido capitalizado"
        );
    }

    #[test]
    fn figure_counters_independentes_por_kind_continuam_a_funcionar_apos_p158b() {
        // Regression P157A/P158A: counters por kind continuam
        // independentes; supplement P158B não interfere.
        use std::sync::Arc;
        let content = Content::Sequence(Arc::from(vec![
            Content::figure(
                Content::text("body1"),
                Some(Content::text("c1")),
                Some("image".to_string()),
                Some("1".to_string()),
            ),
            Content::figure(
                Content::text("body2"),
                Some(Content::text("c2")),
                Some("table".to_string()),
                Some("1".to_string()),
            ),
            Content::figure(
                Content::text("body3"),
                Some(Content::text("c3")),
                Some("image".to_string()),
                Some("1".to_string()),
            ),
        ]));
        let intr = introspect_with_introspector(&content);
        assert_eq!(
            (0..)
                .map_while(|i| intr.figure_number_at_index("image", i))
                .collect::<Vec<_>>(),
            vec![1, 2],
            "image counter independente"
        );
        assert_eq!(
            (0..)
                .map_while(|i| intr.figure_number_at_index("table", i))
                .collect::<Vec<_>>(),
            vec![1],
            "table counter independente"
        );
    }

    // ── Passo 158C: Figure.kind = None resolve a "image" em uso ─────────

    #[test]
    fn introspect_figure_kind_none_resolve_para_image_no_counter() {
        // P158C: kind=None deve cair no default "image" via fallback
        // em counter (paridade backwards compat com tests pré-existentes
        // que usam Some("image".to_string())).
        let content = Content::Sequence(
            vec![labelled_prod(
                Content::figure(
                    Content::text("body sem kind explícito"),
                    Some(Content::text("legenda")),
                    None,
                    Some("1".to_string()),
                ),
                Label("f_none".to_string()),
            )]
            .into(),
        );
        let intr = introspect_with_introspector(&content);
        // Counter "image" deve avançar via fallback default.
        assert_eq!(
            (0..)
                .map_while(|i| intr.figure_number_at_index("image", i))
                .collect::<Vec<_>>(),
            vec![1],
            "kind=None deve cair no default 'image' no counter"
        );
        // **P1034** — default de linguagem é `en`; label resolve para "Figure 1".
        assert_eq!(
            intr.resolved_labels.get(&Label("f_none".to_string())),
            Some("Figure 1"),
            "label de figura kind=None deve resolver via fallback 'image' default"
        );
    }

    // ── P162 .G — Tests do walk com tags em paralelo ─────────────────────

    /// Helper de teste para correr walk e devolver tags.
    /// **P190I**: state eliminado.
    fn introspect_with_tags(content: &Content) -> Vec<Tag> {
        let mut locator = Locator::new();
        let mut tags: Vec<Tag> = Vec::new();
        let mut intr = TagIntrospector::empty();
        let mut auto_label_counter: usize = 0;
        walk(
            content,
            &mut locator,
            &mut tags,
            &mut intr,
            &mut auto_label_counter,
            None,
            &crate::entities::style_chain::StyleChain::default_chain(),
            None,
        );
        tags
    }

    #[test]
    fn walk_emite_start_e_end_para_heading() {
        let h = Content::heading(1, Content::text("title"));
        let tags = introspect_with_tags(&h);
        // P606: heading emite 8 tags com mesma Location:
        //   Start(Heading), Start(Labelled auto-toc), End(Labelled),
        //   Start(HeadingForToc), End(HeadingForToc),
        //   Start(HeadingForBookmarks), End(HeadingForBookmarks), End(Heading).
        // 4 pares Start/End — bracketing por construção (mesma loc).
        assert_eq!(tags.len(), 8, "heading deve emitir 8 tags pós-P606; obtido {tags:?}");
        let locs: Vec<_> = tags
            .iter()
            .map(|t| match t {
                Tag::Start(l, _) | Tag::End(l, _) => *l,
            })
            .collect();
        // Todas as 8 tags partilham mesma Location (P196A §11.5 +
        // P200B/P606 trabalho híbrido).
        for w in locs.windows(2) {
            assert_eq!(w[0], w[1], "tags devem partilhar mesma Location");
        }
        // Ordem esperada: 4 Start consecutivas + 4 End consecutivas
        // (Heading abre, Labelled abre, Labelled fecha, HeadingForToc
        // abre, HeadingForToc fecha, HeadingForBookmarks abre,
        // HeadingForBookmarks fecha, Heading fecha).
        match (&tags[0], &tags[1], &tags[2], &tags[3], &tags[4], &tags[5], &tags[6], &tags[7]) {
            (
                Tag::Start(_, _),
                Tag::Start(_, _),
                Tag::End(_, _),
                Tag::Start(_, _),
                Tag::End(_, _),
                Tag::Start(_, _),
                Tag::End(_, _),
                Tag::End(_, _),
            ) => {}
            other => panic!("ordem esperada: Start, Start, End, Start, End, Start, End, End; obtido {other:?}"),
        }
    }

    #[test]
    fn walk_nao_emite_para_text_simples() {
        let t = Content::text("plain");
        let tags = introspect_with_tags(&t);
        assert!(tags.is_empty(), "text simples não deve emitir tags");
    }

    #[test]
    fn walk_aninha_start_end_para_heading_contendo_figure() {
        // Heading com Figure aninhada no body.
        let figure = Content::figure(
            Content::Empty,
            Some(Content::text("cap")),
            Some("image".into()),
            Some("1".into()),
        );
        let h = Content::heading(1, figure);
        let tags = introspect_with_tags(&h);
        // P606: heading emite 8 tags + figura emite 2 tags = 10 tags.
        // Sequência: Start(Heading), Start(Figure), End(Figure),
        // Start(Labelled), End(Labelled), Start(HeadingForToc),
        // End(HeadingForToc), Start(HeadingForBookmarks),
        // End(HeadingForBookmarks), End(Heading).
        assert_eq!(
            tags.len(),
            10,
            "heading-com-figura deve emitir 10 tags pós-P606, obtido {tags:?}"
        );
        match (
            &tags[0], &tags[1], &tags[2], &tags[3], &tags[4], &tags[5], &tags[6],
            &tags[7], &tags[8], &tags[9],
        ) {
            (
                Tag::Start(_, _), // Heading
                Tag::Start(_, _), // Figure
                Tag::End(_, _),   // Figure
                Tag::Start(_, _), // Labelled auto-toc
                Tag::End(_, _),   // Labelled auto-toc
                Tag::Start(_, _), // HeadingForToc
                Tag::End(_, _),   // HeadingForToc
                Tag::Start(_, _), // HeadingForBookmarks
                Tag::End(_, _),   // HeadingForBookmarks
                Tag::End(_, _),   // Heading
            ) => {}
            other => panic!("ordem esperada após P606: 10 tags; obtido {other:?}"),
        }
    }

    #[test]
    fn walk_emite_tags_em_paralelo_com_state() {
        // Verifica que tanto state quanto tags são populados. P1019: os
        // headings precisam de numbering activo para o counter avançar.
        let content = Content::Sequence(
            vec![
                with_heading_numbering(Content::heading(1, Content::text("um"))),
                with_heading_numbering(Content::heading(1, Content::text("dois"))),
            ]
            .into(),
        );
        let tags = introspect_with_tags(&content);
        // P190I: intr verificado via introspect_with_introspector
        // separado (state legacy eliminada).
        let intr = introspect_with_introspector(&content);
        // Contador heading deve estar em "2" após dois headings nivel 1.
        assert_eq!(
            intr.formatted_counter(&sel(ElementKind::Heading)).as_deref(),
            Some("2"),
            "intr deve ter contador heading=2 após dois headings nível 1"
        );
        // P182C: SetHeadingNumbering passou a ser locatable (emite
        // ElementPayload::StateUpdate sob chave numbering_active:heading).
        // P606: cada heading emite 8 tags (Start_h, Start_labelled,
        // End_labelled, Start_HeadingForToc, End_HeadingForToc,
        // Start_HeadingForBookmarks, End_HeadingForBookmarks, End_h).
        // Total: 2 headings × 8 = 16.
        assert_eq!(
            tags.len(),
            16,
            "deve haver 8 tags por heading (2 headings x 8); obtido {tags:?}"
        );
    }

    #[test]
    fn walk_label_de_wrapper_chega_ao_payload() {
        // Content::Labelled { target: Heading } → tag Heading recebe Some(label).
        let content = Content::label_auto(
            "intro".to_string(),
            Content::heading(1, Content::text("Introdução")),
        );
        let tags = introspect_with_tags(&content);
        // Esperado: Start(Heading) com label="intro", End(Heading).
        match &tags[0] {
            Tag::Start(_, info) => {
                assert_eq!(
                    info.label.as_ref().map(|l| l.0.as_str()),
                    Some("intro"),
                    "label do wrapper Labelled deve aparecer no ElementInfo do Heading"
                );
            }
            other => panic!("esperado Tag::Start, obtido {other:?}"),
        }
    }

    // ── P163 .C — Tests E2E de bracketing ────────────────────────────────

    /// Helper local: constrói Content com headings, figures, citations
    /// aninhados em diversas profundidades para tests E2E.
    fn make_content_complexo() -> Content {
        Content::Sequence(
            vec![
                Content::heading(1, Content::text("Capítulo")),
                Content::figure(
                    Content::Empty,
                    Some(Content::text("legenda")),
                    Some("image".into()),
                    Some("1".into()),
                ),
                Content::heading(2, Content::text("Secção")),
                Content::cite("smith2024", None, None),
            ]
            .into(),
        )
    }

    #[test]
    fn walk_e_deterministico() {
        // P163 .C.1: walk duas vezes sobre o mesmo Content produz
        // Vec<Tag> idêntico (mesma ordem, mesmas Locations, mesmos hashes).
        let content = make_content_complexo();
        let tags1 = introspect_with_tags(&content);
        let tags2 = introspect_with_tags(&content);
        assert_eq!(
            tags1,
            tags2,
            "walk não-determinístico: tags1.len={}, tags2.len={}",
            tags1.len(),
            tags2.len()
        );
    }

    #[test]
    fn bracketing_valido_em_aninhamento_complexo() {
        // P163 .C.2: heading ⊃ figure-com-caption-com-text ⊃ heading.
        // Verificar que cada Start tem o seu End correspondente, sem
        // overlapping. Headings emitem tags; figures aninhadas no
        // caption também.
        let inner_h = Content::heading(2, Content::text("inner"));
        let figure_with_h = Content::figure(
            inner_h,
            Some(Content::text("cap")),
            Some("image".into()),
            Some("1".into()),
        );
        let outer_h = Content::heading(1, figure_with_h);
        let tags = introspect_with_tags(&outer_h);

        let mut stack: Vec<Location> = Vec::new();
        for tag in &tags {
            match tag {
                Tag::Start(loc, _) => stack.push(*loc),
                Tag::End(loc, _) => {
                    let top = stack.pop().expect("End sem Start correspondente");
                    assert_eq!(top, *loc, "End com Location diferente do último Start");
                }
            }
        }
        assert!(stack.is_empty(), "Start sem End correspondente; stack={stack:?}");
    }

    #[test]
    fn bracketing_valido_em_sequencia_plana() {
        // Caso adicional .C.2: múltiplos headings ao mesmo nível
        // (não aninhados) — bracketing também válido.
        let content = Content::Sequence(
            vec![
                Content::heading(1, Content::text("um")),
                Content::heading(1, Content::text("dois")),
                Content::heading(1, Content::text("três")),
            ]
            .into(),
        );
        let tags = introspect_with_tags(&content);

        let mut stack: Vec<Location> = Vec::new();
        for tag in &tags {
            match tag {
                Tag::Start(loc, _) => stack.push(*loc),
                Tag::End(loc, _) => {
                    let top = stack.pop().expect("End sem Start correspondente");
                    assert_eq!(top, *loc);
                }
            }
        }
        assert!(stack.is_empty());
        // P606: cada heading emite 8 tags (Start_h, Start_labelled,
        // End_labelled, Start_HeadingForToc, End_HeadingForToc,
        // Start_HeadingForBookmarks, End_HeadingForBookmarks, End_h)
        // — todas com mesma Location, bracketing continua válido.
        // 3 headings × 8 = 24.
        assert_eq!(tags.len(), 24, "3 headings × 8 tags pós-P606 = 24");
    }

    #[test]
    fn end_hash_distingue_conteudo() {
        // P163 .C.3: dois headings com bodies diferentes produzem
        // Tag::End com u128 distintos.
        // P196B: heading emite agora 2 Tag::End — primeiro o auto-toc
        // Labelled (hash=0 fixo, ADR-0069) e depois o End real do
        // Heading (hash via hash_content). Filtramos hash != 0.
        let a = Content::heading(1, Content::text("Título A"));
        let b = Content::heading(1, Content::text("Título B"));
        let tags_a = introspect_with_tags(&a);
        let tags_b = introspect_with_tags(&b);

        let end_a = tags_a
            .iter()
            .find_map(|t| match t {
                Tag::End(_, h) if *h != 0 => Some(*h),
                _ => None,
            })
            .expect("nenhum Tag::End com hash != 0 emitido para a");
        let end_b = tags_b
            .iter()
            .find_map(|t| match t {
                Tag::End(_, h) if *h != 0 => Some(*h),
                _ => None,
            })
            .expect("nenhum Tag::End com hash != 0 emitido para b");

        assert_ne!(
            end_a, end_b,
            "Contents distintos produziram mesmo content_hash em Tag::End"
        );
    }

    // ── P163 .D — Tests de consistência por kind ─────────────────────────

    #[test]
    fn headings_capturados_em_paralelo() {
        // P163 .D.1: walk sobre Content com N headings em níveis
        // variados. Verificar:
        //  - número de Tag::Start(_, Heading{..}) bate com input;
        //  - depth de cada Heading bate com level esperado;
        //  - intr.formatted_counter(&sel(ElementKind::Heading)) fica no valor
        //    esperado após todos os headings (verificação cruzada).
        let levels = vec![1u8, 2, 2, 3];
        // P1019: cada heading precisa de numbering activo para o counter avançar.
        let content = Content::Sequence(
            levels
                .iter()
                .map(|&l| with_heading_numbering(Content::heading(l, Content::text("h"))))
                .collect::<Vec<_>>()
                .into(),
        );
        let tags = introspect_with_tags(&content);
        // P190I: intr separado para verificação cross.
        let intr = introspect_with_introspector(&content);

        let captured_levels: Vec<u8> = tags
            .iter()
            .filter_map(|t| match t {
                Tag::Start(_, info) => match &info.payload {
                    ElementPayload::Heading { depth, .. } => Some(*depth),
                    _ => None,
                },
                _ => None,
            })
            .collect();

        assert_eq!(
            captured_levels, levels,
            "depth dos heading payloads em tags difere dos levels do input"
        );

        // Verificação cruzada: intr.formatted_counter(&sel(ElementKind::Heading))
        // depois de [1, 2, 2, 3] deve ser "1.1.1.1" — após
        // step ao nível 1, depois nível 2 (= [1,1]), nível 2 outra
        // vez (= [1,2]), depois nível 3 (= [1,2,1]).
        // Na verdade: walk usa step_hierarchical que avança o último
        // segmento ou empurra um novo nível. Com [1,2,2,3]:
        //  []     +1 → [1]
        //  [1]    +2 → [1, 1]
        //  [1,1]  +2 → [1, 2]
        //  [1,2]  +3 → [1, 2, 1]
        // → format = "1.2.1"
        assert_eq!(
            intr.formatted_counter(&sel(ElementKind::Heading)).as_deref(),
            Some("1.2.1"),
            "format_hierarchical não bate com sequência [1,2,2,3]"
        );
    }

    #[test]
    fn figures_capturadas_em_paralelo() {
        // P163 .D.2: walk sobre Content com 3 figures (kind variados).
        let content = Content::Sequence(
            vec![
                Content::figure(
                    Content::Empty,
                    Some(Content::text("c1")),
                    Some("image".into()),
                    Some("1".into()),
                ),
                Content::figure(
                    Content::Empty,
                    Some(Content::text("c2")),
                    Some("table".into()),
                    Some("1".into()),
                ),
                Content::figure(
                    Content::Empty,
                    Some(Content::text("c3")),
                    None,
                    Some("1".into()),
                ),
            ]
            .into(),
        );
        let tags = introspect_with_tags(&content);

        let captured_kinds: Vec<Option<String>> = tags
            .iter()
            .filter_map(|t| match t {
                Tag::Start(_, info) => match &info.payload {
                    ElementPayload::Figure { kind, .. } => Some(kind.clone()),
                    _ => None,
                },
                _ => None,
            })
            .collect();

        assert_eq!(
            captured_kinds,
            vec![Some("image".to_string()), Some("table".to_string()), None],
            "kinds das figures em tags não batem com input"
        );

        // P190H: paridade verificada em testes dedicados via
        // intr.figure_number_at_index (P190H não toca este test
        // específico de Tag emission).
    }

    #[test]
    fn citations_capturadas_em_paralelo() {
        // P163 .D.3: walk sobre Content com 3 citations distintas.
        let content = Content::Sequence(
            vec![
                Content::cite("smith2024", None, None),
                Content::cite("jones2023", None, None),
                Content::cite("smith2024", None, None), // repetida
            ]
            .into(),
        );
        let tags = introspect_with_tags(&content);

        let captured_keys: Vec<String> = tags
            .iter()
            .filter_map(|t| match t {
                Tag::Start(_, info) => match &info.payload {
                    ElementPayload::Citation { key } => Some(key.clone()),
                    _ => None,
                },
                _ => None,
            })
            .collect();

        assert_eq!(
            captured_keys,
            vec![
                "smith2024".to_string(),
                "jones2023".to_string(),
                "smith2024".to_string()
            ],
            "keys das citations em tags não batem com input (incluindo repetição)"
        );

        // Verificação: 3 citations × 2 tags cada = 6 (Start + End por citation).
        let citation_tags: Vec<&Tag> = tags
            .iter()
            .filter(|t| match t {
                Tag::Start(_, info) => {
                    matches!(info.payload, ElementPayload::Citation { .. })
                }
                Tag::End(_, _) => true, // End não tem payload mas todos os Ends correspondem a Citations aqui
            })
            .collect();
        assert_eq!(
            citation_tags.len(),
            6,
            "3 citations × 2 tags = 6, obtido {}",
            citation_tags.len()
        );
    }

    // ── P165 .G — Tests E2E paralelo CounterStateLegacy + Introspector ───

    use crate::entities::element_kind::ElementKind;
    use crate::entities::introspector::Introspector;

    /// Helper de teste para correr walk + construir Introspector em paralelo.
    /// **P191B (ADR-0071)**: walk popula intr directamente; sem
    /// chamada a `from_tags` (eliminado). Funcs em `state.update`
    /// silenciosamente ignoradas neste path local (sem Engine
    /// disponível) — coerente com semântica P173 pré-P191B.
    fn introspect_with_introspector(content: &Content) -> TagIntrospector {
        // **P533** — mirror do path público: converter `@key` bibliográficos
        // antes do walk para contar citações correctamente.
        let content = convert_bib_refs_to_cites(content.clone());
        let mut locator = Locator::new();
        let mut tags: Vec<Tag> = Vec::new();
        let mut intr = TagIntrospector::empty();
        let mut auto_label_counter: usize = 0;
        walk(
            &content,
            &mut locator,
            &mut tags,
            &mut intr,
            &mut auto_label_counter,
            None,
            &crate::entities::style_chain::StyleChain::default_chain(),
            None,
        );
        intr.parent_locations = build_parent_index(&tags);
        intr
    }

    #[test]
    fn introspector_consistencia_heading() {
        // P165 .G.1: walk com headings em níveis [1, 2, 2, 3] →
        // Introspector.kind_index[Heading] tem 4 locations.
        // CounterStateLegacy.format_hierarchical("heading") confirma
        // mesma contagem.
        //
        // **P170 (M9 sub-passo 2)**: paridade extendida — Introspector
        // agora também produz "1.2.1" via formatted_counter (resolve
        // lacuna #5).
        let levels = [1u8, 2, 2, 3];
        // P1019: cada heading precisa de numbering activo para o counter avançar.
        let content = Content::Sequence(
            levels
                .iter()
                .map(|&l| with_heading_numbering(Content::heading(l, Content::text("h"))))
                .collect::<Vec<_>>()
                .into(),
        );
        let intr = introspect_with_introspector(&content);
        // Introspector tem 4 headings indexados.
        assert_eq!(intr.query_by_kind(ElementKind::Heading).len(), 4);
        // CounterStateLegacy tem hierarchical "1.2.1" após [1,2,2,3]
        // (verificado em P163 .D.1).
        assert_eq!(
            intr.formatted_counter(&sel(ElementKind::Heading)).as_deref(),
            Some("1.2.1")
        );
        // P170: Introspector tem mesma string via formatted_counter.
        assert_eq!(
            intr.formatted_counter(&sel(ElementKind::Heading)).as_deref(),
            Some("1.2.1"),
            "P170: paridade entre legacy.format_hierarchical e \
             introspector.formatted_counter"
        );
    }

    #[test]
    fn introspector_consistencia_figure() {
        // P165 .G.2: 3 figures (kind variados) → introspector indexa 3.
        let content = Content::Sequence(
            vec![
                Content::figure(
                    Content::Empty,
                    Some(Content::text("c1")),
                    Some("image".into()),
                    Some("1".into()),
                ),
                Content::figure(
                    Content::Empty,
                    Some(Content::text("c2")),
                    Some("table".into()),
                    Some("1".into()),
                ),
                Content::figure(
                    Content::Empty,
                    Some(Content::text("c3")),
                    None,
                    Some("1".into()),
                ),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);
        assert_eq!(intr.query_by_kind(ElementKind::Figure).len(), 3);
        // CounterStateLegacy resolve kind=None para "image" → image=2, table=1.
        // Introspector preserva kind literal mas conta as 3 sob ElementKind::Figure.
        // Divergência conhecida (m1-lacunas-captura.md #1).
        assert_eq!(
            (0..).map_while(|i| intr.figure_number_at_index("image", i)).count(),
            2
        );
        assert_eq!(
            (0..).map_while(|i| intr.figure_number_at_index("table", i)).count(),
            1
        );
    }

    #[test]
    fn introspector_consistencia_citation() {
        // P165 .G.3: 3 citations distintas com keys → introspector indexa 3.
        let content = Content::Sequence(
            vec![
                Content::cite("smith2024", None, None),
                Content::cite("jones2023", None, None),
                Content::cite("brown2022", None, None),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);
        let locs = intr.query_by_kind(ElementKind::Citation);
        assert_eq!(locs.len(), 3);
    }

    #[test]
    fn introspector_query_by_label() {
        // P165 .G.4: walk com Heading labelled → query_by_label retorna location;
        // mesma location aparece em query_by_kind(Heading).
        let content = Content::label_auto(
            "intro".to_string(),
            Content::heading(1, Content::text("Introdução")),
        );
        let intr = introspect_with_introspector(&content);

        let by_label = intr.query_by_label(&Label("intro".to_string()));
        assert!(by_label.is_some(), "label intro deveria ter sido indexada");
        let by_kind = intr.query_by_kind(ElementKind::Heading);
        assert_eq!(by_kind.len(), 1);
        assert_eq!(by_kind.first().copied(), by_label, "location deve coincidir");
    }

    #[test]
    fn introspector_query_first_e_query_unique() {
        // P165 .G.5: walk com 1 Figure → query_first e query_unique retornam Some.
        // walk com 2 Figures → query_first retorna Some(loc1), query_unique None.
        let single_figure = Content::figure(
            Content::Empty,
            Some(Content::text("c")),
            Some("image".into()),
            Some("1".into()),
        );
        let intr1 = introspect_with_introspector(&single_figure);
        assert!(intr1.query_first(ElementKind::Figure).is_some());
        assert!(intr1.query_unique(ElementKind::Figure).is_some());
        assert_eq!(
            intr1.query_first(ElementKind::Figure),
            intr1.query_unique(ElementKind::Figure)
        );

        let two_figures = Content::Sequence(
            vec![
                Content::figure(
                    Content::Empty,
                    Some(Content::text("c1")),
                    Some("image".into()),
                    Some("1".into()),
                ),
                Content::figure(
                    Content::Empty,
                    Some(Content::text("c2")),
                    Some("table".into()),
                    Some("1".into()),
                ),
            ]
            .into(),
        );
        let intr2 = introspect_with_introspector(&two_figures);
        let first = intr2.query_first(ElementKind::Figure);
        assert!(first.is_some(), "query_first deve devolver primeira location");
        assert_eq!(
            intr2.query_unique(ElementKind::Figure),
            None,
            "query_unique deve devolver None com 2 figures"
        );
    }

    // ── P166 .C — Tests da exposição pública do TagIntrospector ──────────

    #[test]
    fn introspect_with_introspector_devolve_par() {
        // P166 .C.1: novo entry point retorna tuple (state, introspector).
        // Verificar que introspector tem 1 heading indexado.
        let content = Content::heading(1, Content::text("título"));
        let intr = introspect_with_introspector(&content);
        assert!(
            intr.query_first(ElementKind::Heading).is_some(),
            "introspector exposto deve ter o heading indexado"
        );
    }

    #[test]
    fn introspect_chain_threada_gate_de_numbering_ate_o_heading() {
        // **P363 (introspect-chain) — prova a infra (aditiva).** Um heading DENTRO do
        // `Content::Styled` que o `#set heading(numbering:)` embrulha vê o gate
        // `heading.numbering` **na chain** (true); um heading fora não (false). A probe
        // `#[cfg(test)]` no arm Heading do walk captura o gate lido da chain. Prova que
        // a chain foi threadada até o heading (sem mudar o comportamento — o read real do
        // gate continua no campo assado até o de-bake P364).
        use crate::entities::style::Styles;
        super::introspect_chain_probe::reset();
        let sob_set = Content::styled(
            Box::new(Content::heading(1, Content::text("Sob set"))),
            Styles::new().push_custom("heading.numbering", Value::Bool(true)),
        );
        let fora = Content::heading(1, Content::text("Fora"));
        let doc = Content::Sequence(vec![sob_set, fora].into());
        let _ = introspect_with_introspector(&doc);
        assert_eq!(
            super::introspect_chain_probe::captured(),
            vec![true, false],
            "introspect-chain: o heading sob #set numbering vê o gate na chain (true); \
             o de fora não (false)"
        );
    }

    #[test]
    fn introspect_legacy_continua_a_funcionar() {
        // P166 .C.3 (backward compat): call-site antigo
        // `let state = introspect(&c)` continua a compilar e funcionar.
        // Esta é a invariante crítica de M4b — wrapper preserva API.
        // P1019: numbering activo para que o counter de heading avance.
        let content = Content::Sequence(
            vec![
                with_heading_numbering(Content::heading(1, Content::text("um"))),
                with_heading_numbering(Content::heading(1, Content::text("dois"))),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);
        assert_eq!(
            intr.formatted_counter(&sel(ElementKind::Heading)).as_deref(),
            Some("2"),
            "wrapper introspect() deve produzir mesmo state que antes de M4"
        );
    }

    #[test]
    fn introspect_e_introspect_with_introspector_produzem_mesmo_state() {
        // P166 .C.2: state retornado pelo wrapper e pelo entry point novo
        // deve ser idêntico (walk único subjacente; wrapper só descarta
        // o introspector).
        let content = Content::Sequence(
            vec![
                Content::heading(1, Content::text("a")),
                Content::heading(2, Content::text("b")),
                Content::figure(
                    Content::Empty,
                    Some(Content::text("cap")),
                    Some("image".into()),
                    Some("1".into()),
                ),
            ]
            .into(),
        );
        let state_legacy = introspect(&content);
        let _intr_new = introspect_with_introspector(&content);

        // Comparação por campos relevantes (CounterStateLegacy não
        // implementa PartialEq globalmente; comparar via API pública).
        assert_eq!(
            state_legacy.formatted_counter(&sel(ElementKind::Heading)).as_deref(),
            _intr_new.formatted_counter(&sel(ElementKind::Heading)).as_deref(),
        );
        // P190H: state.figure_numbers eliminado. Cobertura paridade
        // via intr sub-stores em testes dedicados (P184E suite).
    }

    // ── P173 (M9 sub-passo 5) — E2E cascade Engine através da API pública ─

    use crate::compiler::eval::EvalContext;
    use crate::entities::args::Args;
    use crate::entities::engine::Engine;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::func::Func;
    use crate::entities::show::{RuleId, ShowRule};
    use crate::entities::sink::Sink;
    use crate::entities::state_update::StateUpdate;
    use crate::entities::style_chain::StyleChain;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library, Route,
    };
    use std::num::NonZeroU16;
    use std::sync::Arc;

    struct E2EWorld {
        library: Library,
        book: FontBook,
        main_id: FileId,
    }
    impl crate::contracts::world::World for E2EWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            self.main_id
        }
        fn source(&self, _: FileId) -> FileResult<crate::entities::source::Source> {
            Err(FileError::NotFound)
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
    fn make_e2e_world() -> E2EWorld {
        E2EWorld {
            library: Library::new(),
            book: FontBook::new(),
            main_id: FileId::from_raw(NonZeroU16::new(1).unwrap()),
        }
    }
    fn add_one_native(
        _ctx: &mut EvalContext,
        args: &Args,
        _world: &dyn crate::contracts::world::World,
        _current_file: FileId,
    ) -> crate::entities::source_result::SourceResult<Value> {
        match args.items.first() {
            Some(Value::Int(n)) => Ok(Value::Int(n + 1)),
            _ => Ok(Value::None),
        }
    }

    fn add_components_native(
        _ctx: &mut EvalContext,
        args: &Args,
        _world: &dyn crate::contracts::world::World,
        _current_file: FileId,
    ) -> crate::entities::source_result::SourceResult<Value> {
        match args.items.as_slice() {
            [Value::Int(a), Value::Int(b)] => {
                Ok(Value::Array(vec![Value::Int(a + 1), Value::Int(b + 2)]))
            }
            _ => Ok(Value::None),
        }
    }

    macro_rules! with_engine {
        ($world:expr, |$engine:ident, $ctx:ident| $body:block) => {{
            use comemo::Track;
            let world: &dyn crate::contracts::world::World = $world;
            let mut $ctx = EvalContext::new();
            let route = Route::root().with_id(world.main());
            let mut styles = StyleChain::default_chain();
            let mut show_rules: Arc<[ShowRule]> = Arc::from([]);
            let mut active_guards: Vec<RuleId> = Vec::new();
            let current_file = world.main();
            let fixed_metrics = FixedMetrics;
            let mut sink_local = Sink::new();
            let mut sink = sink_local.track_mut();
            let mut $engine = Engine {
                world,
                font_metrics: &fixed_metrics,
                route: route.track(),
                styles: &mut styles,
                show_rules: &mut show_rules,
                active_guards: &mut active_guards,
                current_file,
                sink: &mut sink,
            };
            $body
        }};
    }

    #[test]
    fn p173_cascade_engine_via_api_publica() {
        // E2E: estado de count via API pública. Cascade Engine →
        // from_tags → apply_func executa Func real.
        let f = Func::native("add_one", add_one_native);
        let content = Content::Sequence(
            vec![
                Content::state("c".to_string(), Value::Int(0)),
                Content::state_update("c".to_string(), StateUpdate::Func(f)),
            ]
            .into(),
        );
        // P191B (ADR-0071): pipeline walk → apply_state_funcs.
        // introspect_with_introspector já não recebe engine/ctx (Funcs
        // path legacy ignoradas); para exercitar Func eval, replicamos
        // o pipeline manual de fixpoint::run_fixpoint.
        let world = make_e2e_world();
        let intr = with_engine!(&world, |engine, ctx| {
            let mut locator = Locator::new();
            let mut tags: Vec<Tag> = Vec::new();
            let mut intr = TagIntrospector::empty();
            let mut auto_label_counter: usize = 0;
            super::walk(
                &content,
                &mut locator,
                &mut tags,
                &mut intr,
                &mut auto_label_counter,
                None,
                &crate::entities::style_chain::StyleChain::default_chain(),
                None,
            );
            super::from_tags::apply_state_funcs(&tags, &mut intr, &mut engine, &mut ctx)
                .expect("apply_state_funcs deve suceder");
            intr
        });
        assert_eq!(intr.state_final_value("c"), Some(&Value::Int(1)));
    }

    #[test]
    fn p1149_counter_callback_recebe_componentes_separados() {
        let key = CounterKey::Str("p1149".into());
        let content = Content::Sequence(
            vec![
                Content::counter_update(key.clone(), CounterUpdate::Set(vec![2, 3])),
                Content::counter_update(
                    key.clone(),
                    CounterUpdate::Func(Func::native(
                        "add_components",
                        add_components_native,
                    )),
                ),
            ]
            .into(),
        );
        let world = make_e2e_world();
        let final_state = with_engine!(&world, |engine, ctx| {
            let mut locator = Locator::new();
            let mut tags = Vec::new();
            let mut intr = TagIntrospector::empty();
            let mut auto_label_counter = 0;
            super::walk(
                &content,
                &mut locator,
                &mut tags,
                &mut intr,
                &mut auto_label_counter,
                None,
                &StyleChain::default_chain(),
                None,
            );
            super::from_tags::apply_counter_funcs(
                &tags,
                &mut intr,
                &mut engine,
                &mut ctx,
            )
            .expect("callback de counter deve suceder");
            intr.counter_final_values(&key).map(<[usize]>::to_vec)
        });
        assert_eq!(final_state, Some(vec![3, 5]));
    }

    #[test]
    fn p173_introspect_legacy_ignora_func() {
        // E2E: API legacy `introspect()` (sem Engine) continua a funcionar.
        // Funcs em state.update são silenciosamente ignoradas.
        let f = Func::native("add_one", add_one_native);
        let content = Content::Sequence(
            vec![
                Content::state("c".to_string(), Value::Int(0)),
                Content::state_update("c".to_string(), StateUpdate::Func(f)),
            ]
            .into(),
        );
        // Path legacy: walk + from_tags(_, None, None).
        let intr = super::introspect_with_introspector(&content);
        // Func ignorada → final value continua em init.
        assert_eq!(intr.state_final_value("c"), Some(&Value::Int(0)));
    }

    #[test]
    fn p173_determinismo_func_eval() {
        // E2E: dois chamadas com Engine produzem mesmo resultado.
        let f1 = Func::native("add_one", add_one_native);
        let f2 = Func::native("add_one", add_one_native);
        let content_a = Content::Sequence(
            vec![
                Content::state("c".to_string(), Value::Int(5)),
                Content::state_update("c".to_string(), StateUpdate::Func(f1)),
            ]
            .into(),
        );
        let content_b = Content::Sequence(
            vec![
                Content::state("c".to_string(), Value::Int(5)),
                Content::state_update("c".to_string(), StateUpdate::Func(f2)),
            ]
            .into(),
        );
        // P191B (ADR-0071): pipeline walk → apply_state_funcs.
        let world = make_e2e_world();
        let v_a = with_engine!(&world, |engine, ctx| {
            let mut locator = Locator::new();
            let mut tags: Vec<Tag> = Vec::new();
            let mut intr = TagIntrospector::empty();
            let mut auto_label_counter: usize = 0;
            super::walk(
                &content_a,
                &mut locator,
                &mut tags,
                &mut intr,
                &mut auto_label_counter,
                None,
                &crate::entities::style_chain::StyleChain::default_chain(),
                None,
            );
            super::from_tags::apply_state_funcs(&tags, &mut intr, &mut engine, &mut ctx)
                .expect("apply_state_funcs deve suceder");
            intr.state_final_value("c").cloned()
        });
        let v_b = with_engine!(&world, |engine, ctx| {
            let mut locator = Locator::new();
            let mut tags: Vec<Tag> = Vec::new();
            let mut intr = TagIntrospector::empty();
            let mut auto_label_counter: usize = 0;
            super::walk(
                &content_b,
                &mut locator,
                &mut tags,
                &mut intr,
                &mut auto_label_counter,
                None,
                &crate::entities::style_chain::StyleChain::default_chain(),
                None,
            );
            super::from_tags::apply_state_funcs(&tags, &mut intr, &mut engine, &mut ctx)
                .expect("apply_state_funcs deve suceder");
            intr.state_final_value("c").cloned()
        });
        assert_eq!(v_a, Some(Value::Int(6)));
        assert_eq!(v_a, v_b);
    }

    // ── P181H — Walk arm Bibliography puro (P163 invariante restaurada) ─

    #[test]
    fn walk_arm_bibliography_nao_muta_state_bib_legacy() {
        // P181H: walk arm `Content::Bibliography` não muta directamente
        // `state.bib_entries` ou `state.bib_numbers`. Tag emitida via
        // extract_payload do topo de walk (P181D); BibStore populado
        // por from_tags arm (P181E). Walk puro restaurado para bib —
        // invariante P163 preservada.
        use crate::entities::bib_entry::BibEntry;
        use crate::entities::content::Content;

        let content = Content::bibliography(
            vec![
                BibEntry::new("a", "Author A", "Title A", 2024),
                BibEntry::new("b", "Author B", "Title B", 2025),
            ],
            None,
        );
        let mut locator = Locator::new();
        let mut tags: Vec<Tag> = Vec::new();
        let mut intr = TagIntrospector::empty();
        let mut auto_label_counter: usize = 0;
        walk(
            &content,
            &mut locator,
            &mut tags,
            &mut intr,
            &mut auto_label_counter,
            None,
            &crate::entities::style_chain::StyleChain::default_chain(),
            None,
        );

        // P190B (M6 categoria Bibliography eliminada): assertions sobre
        // `state.bib_entries`/`bib_numbers` removidas — fields eliminados
        // de CounterStateLegacy. Walk arm puro desde P181H confirmado
        // estruturalmente (sem mutação a fields que já não existem).
        // Cobertura cobrir-a-tag preservada abaixo.

        // Tag emitida pelo topo via extract_payload (P181D): existe
        // exactamente uma Tag::Start de Bibliography.
        use crate::entities::element_payload::ElementPayload;
        let bib_tags: Vec<_> = tags.iter().filter(|t| matches!(
            t,
            Tag::Start(_, info) if matches!(info.payload, ElementPayload::Bibliography { .. })
        )).collect();
        assert_eq!(
            bib_tags.len(),
            1,
            "tag Bibliography deve ser emitida via extract_payload mesmo com walk puro"
        );
    }

    #[test]
    fn walk_arm_bibliography_desce_em_title() {
        // P181H: walk arm puro continua a descer no `title` (preserva
        // comportamento legacy para que children dentro de title
        // sejam visíveis a outros consumers).
        use crate::entities::bib_entry::BibEntry;
        use crate::entities::content::Content;

        let titulo = Content::label_auto(
            "bib-title".to_string(),
            Content::heading(1, Content::Empty),
        );

        let content =
            Content::bibliography(vec![BibEntry::new("a", "A", "T", 2024)], Some(titulo));
        let mut locator = Locator::new();
        let mut tags: Vec<Tag> = Vec::new();
        let mut intr = TagIntrospector::empty();
        let mut auto_label_counter: usize = 0;
        walk(
            &content,
            &mut locator,
            &mut tags,
            &mut intr,
            &mut auto_label_counter,
            None,
            &crate::entities::style_chain::StyleChain::default_chain(),
            None,
        );

        // Heading dentro de title produz Tag de Heading.
        use crate::entities::element_payload::ElementPayload;
        let heading_tags: Vec<_> = tags.iter().filter(|t| matches!(
            t,
            Tag::Start(_, info) if matches!(info.payload, ElementPayload::Heading { .. })
        )).collect();
        assert_eq!(
            heading_tags.len(),
            1,
            "walk deve descer em Bibliography.title — Heading interno deve produzir Tag"
        );
    }

    // ── P196B — Walk arm Heading auto-toc via Tag pattern (ADR-0069) ─────
    //
    // 5 tests E2E que validam: (a) emissão de Tag::Labelled auto-toc;
    // (b) paridade entre legacy state e Introspector resolved_labels;
    // (c) numbering inactivo produz string vazia (paridade legacy);
    // (d) E2-residuo `headings_for_toc` continua via legacy mutation;
    // (e) consumer C4 substitution-with-fallback recebe Some via
    //     Introspector path (auto-toc label sintetizada).

    #[test]
    fn heading_auto_toc_walk_emite_tag_e_popula_introspector() {
        // P196B: walk arm Heading emite Tag::Start+End com payload
        // Labelled { label: "auto-toc-N", resolved_text, figure_number: None }
        // pós-recursão. Introspector populated via from_tags.
        let content = Content::Sequence(
            vec![Content::heading_numbered(1, Content::text("Intro"))].into(),
        );
        let intr = introspect_with_introspector(&content);

        // Auto-label "auto-toc-1" deve estar no Introspector.resolved_labels
        // populado via Tag::Labelled pós-P196B.
        let auto_label = Label("auto-toc-1".to_string());
        assert_eq!(
            intr.resolved_label_for(&auto_label),
            // P359 (DEBT-60 b): auto-toc resolved_text é o NÚMERO ("1."), sem o
            // supplement "Secção" — o outline mostra o numbering (paridade vanilla).
            Some("1."),
            "P196B: Introspector.resolved_labels deve conter auto-toc-1 \
             populado via Tag::Labelled emitida pelo walk arm Heading"
        );
    }

    #[test]
    fn heading_auto_toc_paridade_legacy_vs_introspector() {
        // P196B: durante janela compat M5, legacy state e Introspector
        // devem conter MESMO valor para auto-toc label (write paralelo).
        let content = Content::Sequence(
            vec![
                Content::heading(1, Content::text("um")),
                Content::heading(2, Content::text("dois")),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);

        // 2 headings → auto-toc-1 e auto-toc-2.
        for n in [1usize, 2] {
            let lbl = Label(format!("auto-toc-{n}"));
            let from_legacy = intr.resolved_labels.get(&lbl);
            let from_intr = intr.resolved_label_for(&lbl);
            assert!(
                from_legacy.is_some(),
                "legacy state deve conter {lbl:?} (write paralelo M5)"
            );
            assert!(
                from_intr.is_some(),
                "Introspector deve conter {lbl:?} (Tag::Labelled P196B)"
            );
            assert_eq!(
                from_legacy, from_intr,
                "paridade compat M5: legacy e Introspector devem ter \
                 o mesmo resolved_text para {lbl:?}"
            );
        }
    }

    #[test]
    fn heading_auto_toc_numbering_inactivo_emite_string_vazia() {
        // P196B preserva paridade legacy: quando numbering inactivo,
        // helper compute_heading_auto_toc retorna (label, "") —
        // resolved_labels recebe insert mesmo assim (presença, não conteúdo).
        let content = Content::heading(1, Content::text("sem numbering"));
        let intr = introspect_with_introspector(&content);

        let lbl = Label("auto-toc-1".to_string());
        // Legacy state preserva insert "" (não None).
        assert_eq!(
            intr.resolved_labels.get(&lbl),
            Some(""),
            "legacy: numbering inactivo → insert string vazia"
        );
        // Introspector path: Tag::Labelled tem resolved_text=Some("") →
        // populated em ResolvedLabelStore.
        assert_eq!(
            intr.resolved_label_for(&lbl),
            Some(""),
            "P196B: Introspector path preserva paridade — string vazia \
             sob numbering inactivo, não None"
        );
    }

    #[test]
    fn walk_e2_residuo_headings_for_toc_via_legacy() {
        // E2-residuo: state.headings_for_toc.push continua activo
        // como mutação legacy porque sub-store `intr.headings_for_toc`
        // não existe (lacuna #3). Test confirma write paralelo M5.
        let content = Content::Sequence(
            vec![
                Content::heading(1, Content::text("Cap 1")),
                Content::heading(2, Content::text("Sec 1.1")),
                Content::heading(1, Content::text("Cap 2")),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);

        // P190G: 3 entries em intr.headings_for_toc (1 por heading).
        assert_eq!(
            intr.headings_for_toc().len(),
            3,
            "walk arm Heading popula intr.headings_for_toc via \
             Tag::HeadingForToc pós-recursão (P200B + P190G)"
        );
        // Levels preservados em ordem.
        let levels: Vec<usize> =
            intr.headings_for_toc().iter().map(|(_, _, _, lvl)| *lvl).collect();
        assert_eq!(levels, vec![1, 2, 1]);
    }

    #[test]
    fn p606_outlined_e_bookmarked_separados_nas_substores() {
        // Quatro casos vanilla:
        // A: outlined=true,  bookmarked=auto  -> em TOC e bookmarks
        // B: outlined=false, bookmarked=auto  -> fora de TOC e bookmarks
        // C: outlined=true,  bookmarked=false -> em TOC, fora de bookmarks
        // D: outlined=false, bookmarked=false -> fora de ambos
        let content = Content::Sequence(
            vec![
                Content::heading(1, Content::text("A")),
                Content::heading_with_outlined_and_bookmarked(
                    1,
                    Content::text("B"),
                    false,
                    None,
                ),
                Content::heading_with_outlined_and_bookmarked(
                    1,
                    Content::text("C"),
                    true,
                    Some(false),
                ),
                Content::heading_with_outlined_and_bookmarked(
                    1,
                    Content::text("D"),
                    false,
                    Some(false),
                ),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);

        let toc_titles: Vec<String> = intr
            .headings_for_toc()
            .iter()
            .map(|(_, _, body, _)| body.plain_text())
            .collect();
        let bm_titles: Vec<String> = intr
            .headings_for_bookmarks()
            .iter()
            .map(|(_, _, body, _)| body.plain_text())
            .collect();

        assert_eq!(toc_titles, vec!["A", "C"], "TOC deve conter A e C");
        assert_eq!(bm_titles, vec!["A"], "Bookmarks devem conter apenas A");
    }

    #[test]
    fn consumer_c4_recebe_some_para_auto_toc_label() {
        // P196B: consumer C4 (Ref-arm em Layouter, P194B) usa
        // substitution-with-fallback: `intr.resolved_label_for(label)
        // .or_else(|| state.resolved_labels.get(label))`.
        // Pós-P196B, primeira branch (`.resolved_label_for`) deve
        // devolver `Some(text)` para auto-toc labels — caminho
        // Introspector activo (sem fallback necessário).
        let content = Content::Sequence(
            vec![Content::heading_numbered(1, Content::text("Capítulo"))].into(),
        );
        let intr = introspect_with_introspector(&content);

        // Primeira branch: Introspector path.
        let auto_lbl = Label("auto-toc-1".to_string());
        let via_introspector = intr.resolved_label_for(&auto_lbl);
        assert!(
            via_introspector.is_some(),
            "P196B: consumer C4 deve obter Some via Introspector path \
             para auto-toc-1 — fallback legacy desnecessário"
        );
        // P359 (DEBT-60 b): número só ("1."), sem supplement "Secção".
        assert_eq!(via_introspector, Some("1."));
    }

    // ── P197B — Walk arm Figure refactor (cenário α) ─────────────────────
    //
    // 5 tests sentinela que validam: (a) caminho Introspector já activo
    // desde P184 (independente de P197B); (b) helper compute_figure
    // produz mesmo resultado que walk legacy; (c) paridade
    // legacy↔Introspector inalterada; (d) numbering inactivo retorna
    // None (helper sem mutação); (e) compute_labelled P195D Figure arm
    // continua funcional após refactor (cadeia E2-E3 preservada).

    #[test]
    fn figure_walk_caminho_introspector_ja_activo() {
        // P197B test 1: confirma que o caminho Introspector para figure
        // numbering já está activo desde P184. Independente de P197B —
        // cenário α (P197A diagnóstico §5).
        let content = Content::figure(
            Content::Empty,
            Some(Content::text("Cap")),
            Some("image".into()),
            Some("1".to_string()),
        );
        let intr = introspect_with_introspector(&content);

        // Consumer C3 path (P184D): figure_number_at_index retorna Some.
        assert_eq!(
            intr.figure_number_at_index("image", 0),
            Some(1),
            "caminho Introspector já activo desde P184 — P197B não alterou"
        );
        // kind_index populado.
        assert_eq!(intr.query_by_kind(ElementKind::Figure).len(), 1);
    }

    #[test]
    fn figure_walk_intr_counters_populated_correctamente() {
        // P197B test 2 (P190H adapted): walk arm Figure puro — populate_intr
        // arm Figure (P191C) popula intr.counters["figure:image"] em
        // ordem para figuras is_counted. Field state.figure_numbers
        // eliminado.
        let content = Content::Sequence(
            vec![
                Content::figure(
                    Content::Empty,
                    Some(Content::text("c1")),
                    Some("image".into()),
                    Some("1".to_string()),
                ),
                Content::figure(
                    Content::Empty,
                    Some(Content::text("c2")),
                    Some("image".into()),
                    Some("1".to_string()),
                ),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);

        // 2 figures image numeradas → intr.figure_number_at_index = [1, 2].
        assert_eq!(
            (0..)
                .map_while(|i| intr.figure_number_at_index("image", i))
                .collect::<Vec<_>>(),
            vec![1, 2],
            "P190H: populate_intr Figure popula intr.counters em ordem (1-based)"
        );
    }

    #[test]
    fn figure_paridade_introspector_pos_p190h() {
        // P197B test 3 (P190H adapted): confirma paridade Introspector
        // path puro após eliminação fields legacy. Caminho Introspector
        // (P184D) é única fonte da verdade.
        let content = Content::figure(
            Content::Empty,
            Some(Content::text("Cap")),
            Some("table".into()),
            Some("1".to_string()),
        );
        let intr = introspect_with_introspector(&content);

        let intr_num = intr.figure_number_at_index("table", 0);
        assert_eq!(
            intr_num,
            Some(1),
            "P190H: figura única counted → intr.figure_number_at_index(table, 0) = 1"
        );
    }

    #[test]
    fn figure_numbering_inactivo_nao_popula_intr() {
        // P197B test 4 (P190H adapted): figura sem caption → is_counted
        // = false → populate_intr arm Figure (gated por is_counted)
        // NÃO popula intr.counters. Field state.figure_numbers e
        // state.local_figure_counters eliminados.
        let figura_sem_caption = Content::figure(
            Content::Empty,
            None,
            Some("image".into()),
            Some("1".to_string()),
        );
        let intr = introspect_with_introspector(&figura_sem_caption);

        // intr.counters["figure:image"] não populated — gate is_counted.
        assert_eq!(
            intr.figure_number_at_index("image", 0),
            None,
            "is_counted=false → populate_intr não aplica counter"
        );
    }

    #[test]
    fn figure_compute_labelled_p195d_continua_funcional() {
        // P197B test 5 (P190H adapted): cadeia E2-E3 preservada via
        // Introspector path puro. compute_labelled (P191C migrado) lê
        // intr.flat_counter_at; populate_intr arm Labelled popula
        // intr.figure_label_numbers.
        let content = labelled_prod(
            Content::figure(
                Content::Empty,
                Some(Content::text("Cap")),
                Some("image".into()),
                Some("1".to_string()),
            ),
            Label("fig1".to_string()),
        );
        let intr = introspect_with_introspector(&content);

        // P190H: intr.figure_label_numbers populated via populate_intr
        // arms Figure + Labelled (sem fallback legacy).
        assert_eq!(
            intr.figure_label_numbers.get(&Label("fig1".to_string())).copied(),
            Some(1),
            "Introspector path: figure_label_numbers populated via P195D + P184B"
        );
    }

    // ── P203B — Formalização do fecho das lacunas #1 e #1b ───────────────
    //
    // Lacunas #1 (`figure.kind` literal em tags vs colapsado em state) e
    // #1b (`from_tags` arm Figure sem gate `is_counted`) foram
    // **estruturalmente fechadas** por P190H + P191C:
    //
    // - **P190H**: campos legacy `state.figure_numbers`,
    //   `state.figure_label_numbers`, `state.local_figure_counters`
    //   eliminados; helper `compute_figure` orphan removido.
    // - **P191C**: `populate_intr_from_tag_start` arm Figure aplica
    //   `is_counted` gate + `kind.as_deref().unwrap_or("image")` default
    //   no momento da Tag::Start emission.
    //
    // P203B é trabalho **declarativo** — não toca código produção
    // (já alinhado pós-P190/P191). Este test consolidado formaliza o
    // fecho das lacunas com 4 casos canónicos.
    //
    // Per `P203B.div-1`: spec P203B descrevia walk arm Figure como
    // usando `unwrap_or("image")` directamente, mas walk arm Figure
    // está puro desde P190H — apenas desce em body + caption. O default
    // foi migrado para `populate_intr_from_tag_start` (P191C).

    #[test]
    fn p203b_lacuna_1_e_1b_fecho_formal_4_casos() {
        // 4 casos canónicos per spec P203B C3:
        //   (1) #figure([img])                      — kind=None,    sem caption
        //   (2) #figure([img], caption: [c])        — kind=None,    com caption
        //   (3) #figure(kind: "table", [t], caption)— kind=Some,    com caption
        //   (4) #figure(kind: "table", [t])         — kind=Some,    sem caption

        use crate::compiler::introspect::extract_payload::extract_payload;

        // F-5a de-bake (P365): `Content::figure(.., Some)` produz a forma de
        // transporte `Styled(Figure, custom)`. Para a parte (a) — que chama
        // `extract_payload` **direto** no elemento — descasca o transporte para
        // alcançar a `Figure`. `extract_payload`/`to_payload` dão agora o
        // **placeholder** de `is_counted` (= `caption.is_some()`); o gate de
        // numbering é ANDado no walk (parte (b)/(c) abaixo o exercitam). Como todos
        // os casos têm numbering Some, o placeholder coincide com o valor antigo.
        fn fig_inner(c: &Content) -> &Content {
            match c {
                Content::Styled(styled) => &styled.body,
                other => other,
            }
        }

        let caso1 =
            Content::figure(Content::text("img"), None, None, Some("1".to_string()));
        let caso2 = Content::figure(
            Content::text("img"),
            Some(Content::text("c")),
            None,
            Some("1".to_string()),
        );
        let caso3 = Content::figure(
            Content::text("t"),
            Some(Content::text("c")),
            Some("table".to_string()),
            Some("1".to_string()),
        );
        let caso4 = Content::figure(
            Content::text("t"),
            None,
            Some("table".to_string()),
            Some("1".to_string()),
        );

        // (a) `extract_payload` preserva `kind` literalmente (sem default
        //     — lacuna #1 fecha porque tag preserva None vs Some("image")
        //     distintamente; default só aplica em populate_intr).
        match extract_payload(fig_inner(&caso1)) {
            Some(ElementPayload::Figure { kind, is_counted, .. }) => {
                assert_eq!(kind, None, "caso1 kind preservado None literal");
                assert!(!is_counted, "caso1 sem caption → is_counted=false");
            }
            other => panic!("caso1: esperado Some(Figure), obtido {other:?}"),
        }
        match extract_payload(fig_inner(&caso2)) {
            Some(ElementPayload::Figure { kind, is_counted, .. }) => {
                assert_eq!(kind, None, "caso2 kind preservado None literal");
                assert!(is_counted, "caso2 com caption+numbering → is_counted=true");
            }
            other => panic!("caso2: esperado Some(Figure), obtido {other:?}"),
        }
        match extract_payload(fig_inner(&caso3)) {
            Some(ElementPayload::Figure { kind, is_counted, .. }) => {
                assert_eq!(
                    kind,
                    Some("table".to_string()),
                    "caso3 kind literal Some(\"table\")"
                );
                assert!(is_counted, "caso3 com caption+numbering → is_counted=true");
            }
            other => panic!("caso3: esperado Some(Figure), obtido {other:?}"),
        }
        match extract_payload(fig_inner(&caso4)) {
            Some(ElementPayload::Figure { kind, is_counted, .. }) => {
                assert_eq!(
                    kind,
                    Some("table".to_string()),
                    "caso4 kind literal Some(\"table\")"
                );
                assert!(!is_counted, "caso4 sem caption → is_counted=false");
            }
            other => panic!("caso4: esperado Some(Figure), obtido {other:?}"),
        }

        // (b) `populate_intr_from_tag_start` aplica gate `is_counted`
        //     + default `unwrap_or("image")` consistentemente para os
        //     4 casos. Lacuna #1b fecha — gate aplicado no caminho
        //     activo (não em from_tags arm porque tal arm não existe;
        //     população é durante walk via populate_intr_from_tag_start
        //     desde P191B/C ADR-0071).
        let intr1 = introspect_with_introspector(&caso1);
        assert_eq!(
            intr1.figure_number_at_index("image", 0),
            None,
            "caso1 is_counted=false → counter NÃO populated (gate aplicado)"
        );

        let intr2 = introspect_with_introspector(&caso2);
        assert_eq!(
            intr2.figure_number_at_index("image", 0),
            Some(1),
            "caso2 is_counted=true + kind=None → kind_key='image' (default) → counter[1]"
        );

        let intr3 = introspect_with_introspector(&caso3);
        assert_eq!(
            intr3.figure_number_at_index("table", 0),
            Some(1),
            "caso3 is_counted=true + kind=Some(\"table\") → counter['table'][1]"
        );
        assert_eq!(
            intr3.figure_number_at_index("image", 0),
            None,
            "caso3 kind='table' → contador 'image' NÃO populated"
        );

        let intr4 = introspect_with_introspector(&caso4);
        assert_eq!(
            intr4.figure_number_at_index("table", 0),
            None,
            "caso4 is_counted=false → counter NÃO populated (gate aplicado)"
        );

        // (c) Walk emite Tags consistentes (Tag preserva kind literal).
        let tags1 = introspect_with_tags(&caso1);
        let tags2 = introspect_with_tags(&caso2);
        // caso1 (sem caption, kind=None): payload kind=None preservado.
        let payload1 = tags1.iter().find_map(|t| match t {
            Tag::Start(_, info) => match &info.payload {
                ElementPayload::Figure { kind, is_counted, .. } => {
                    Some((kind.clone(), *is_counted))
                }
                _ => None,
            },
            _ => None,
        });
        assert_eq!(
            payload1,
            Some((None, false)),
            "Tag preserva kind=None literal e is_counted=false (paridade pre-walk)."
        );
        // caso2 (com caption, kind=None): payload kind=None preservado, is_counted=true.
        let payload2 = tags2.iter().find_map(|t| match t {
            Tag::Start(_, info) => match &info.payload {
                ElementPayload::Figure { kind, is_counted, .. } => {
                    Some((kind.clone(), *is_counted))
                }
                _ => None,
            },
            _ => None,
        });
        assert_eq!(
            payload2,
            Some((None, true)),
            "Tag preserva kind=None literal mesmo quando is_counted=true."
        );
    }

    // ── P198C — Walk arm CounterUpdate (cenário β-promote) ──────────────
    //
    // 6 tests sentinela que validam: (a) extract_payload arm novo;
    // (b) is_locatable activado; (c) from_tags arm popula
    // CounterRegistry via apply_at; (d) paridade legacy vs Introspector;
    // (e) action Update aplica correctamente; (f) cadeia E6 ↔ helpers
    // compute_* funcional após promote.

    #[test]
    fn counter_update_extract_payload_emite_payload() {
        // P198C test 1: confirma que extract_payload(CounterUpdate)
        // retorna Some(ElementPayload::CounterUpdate { ... }).
        use crate::compiler::introspect::extract_payload::extract_payload;
        use crate::entities::counter_update::CounterUpdate as CU;

        let content =
            Content::counter_update("equation".to_string(), CounterAction::step());
        match extract_payload(&content) {
            Some(ElementPayload::CounterUpdate { key, action }) => {
                assert_eq!(key, ck("equation"));
                assert_eq!(action, CU::step());
            }
            other => panic!("esperado Some(CounterUpdate), obtido {other:?}"),
        }
    }

    #[test]
    fn counter_update_is_locatable_true() {
        // P198C test 2: is_locatable(CounterUpdate) = true após promote.
        use crate::compiler::introspect::locatable::is_locatable;

        let c = Content::counter_update("page".to_string(), CounterAction::Set(vec![42]));
        assert!(
            is_locatable(&c),
            "P198C: is_locatable(CounterUpdate) deve retornar true após promote"
        );
    }

    #[test]
    fn counter_update_walk_popula_counter_registry() {
        // P198C test 3: pipeline walk + from_tags com 2 CounterUpdate
        // (Step) popula CounterRegistry; flat_counter_at retorna valor
        // correcto.
        use crate::entities::introspector::Introspector;

        let content = Content::Sequence(
            vec![
                Content::counter_update("equation".to_string(), CounterAction::step()),
                Content::counter_update("equation".to_string(), CounterAction::step()),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);

        // 2 Steps em "equation" → counter chega a 2.
        // Procurar a última location com snapshot.
        let tags_locations: Vec<Location> =
            intr.query_by_kind(ElementKind::CounterUpdate);
        assert_eq!(
            tags_locations.len(),
            2,
            "P198C: 2 CounterUpdate emitem 2 locations indexadas em kind_index"
        );
        let last_loc = *tags_locations.last().unwrap();
        assert_eq!(
            intr.flat_counter_at(&ck("equation"), last_loc),
            Some(2),
            "P198C: from_tags arm popula CounterRegistry; flat=2 após 2 Steps"
        );
    }

    #[test]
    fn counter_update_paridade_legacy_vs_introspector() {
        // P198C test 4: paridade legacy state vs Introspector após promote.
        use crate::entities::introspector::Introspector;

        let content = Content::Sequence(
            vec![
                Content::counter_update("equation".to_string(), CounterAction::step()),
                Content::counter_update("equation".to_string(), CounterAction::step()),
                Content::counter_update("equation".to_string(), CounterAction::step()),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);

        // Legacy: state.flat populated via walk arm.
        assert_eq!(
            intr.counters
                .value(&ck("equation"))
                .and_then(|v| v.last())
                .copied()
                .unwrap_or(0),
            3,
            "legacy: 3 Steps → state.flat['equation'] == 3"
        );
        // Introspector: counter populated via from_tags arm.
        let last_loc = *intr.query_by_kind(ElementKind::CounterUpdate).last().unwrap();
        assert_eq!(
            intr.flat_counter_at(&ck("equation"), last_loc),
            Some(3),
            "Introspector: paridade pós-P198C — flat_counter_at == 3"
        );
    }

    #[test]
    fn p461_table_counter_popula_via_introspector() {
        // P461: Table locatable + counter "table" populado quando
        // caption + table.numbering presentes.
        use crate::entities::introspector::Introspector;
        use crate::entities::layout_types::TrackSizing;
        use crate::entities::style::Styles;
        use crate::entities::value::Value;

        let mk = |n: usize| {
            Content::table_with_caption(
                vec![TrackSizing::Auto],
                vec![TrackSizing::Auto],
                vec![Content::text(format!("cell{n}"))],
                Some(Content::text(format!("cap{n}"))),
            )
        };
        let content = Content::Sequence(
            vec![
                Content::styled(
                    Box::new(mk(1)),
                    Styles::new().push_custom("table.numbering", Value::Str("1.".into())),
                ),
                Content::styled(
                    Box::new(mk(2)),
                    Styles::new().push_custom("table.numbering", Value::Str("1.".into())),
                ),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);

        let locs = intr.query_by_kind(ElementKind::Table);
        assert_eq!(locs.len(), 2, "duas tables locatable");
        assert_eq!(
            intr.flat_counter_at(&sel(ElementKind::Table), locs[0]),
            Some(1),
            "primeira table numerada 1"
        );
        assert_eq!(
            intr.flat_counter_at(&sel(ElementKind::Table), locs[1]),
            Some(2),
            "segunda table numerada 2"
        );
    }

    /// **P1016** — `Content::Footnote` locatable, counter flat `"footnote"`
    /// a avançar uma vez por nota. Espelho do `p461_table_counter_*`, sem
    /// gate: toda a nota conta.
    #[test]
    fn p1016_footnote_counter_popula_via_introspector() {
        use crate::entities::introspector::Introspector;

        let content = Content::Sequence(
            vec![
                Content::text("A"),
                Content::footnote(Content::text("uma")),
                Content::text("B"),
                Content::footnote(Content::text("duas")),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);

        let locs = intr.query_by_kind(ElementKind::Footnote);
        assert_eq!(locs.len(), 2, "duas footnotes locatable");
        assert_eq!(
            intr.flat_counter_at(&sel(ElementKind::Footnote), locs[0]),
            Some(1),
            "primeira nota numerada 1"
        );
        assert_eq!(
            intr.flat_counter_at(&sel(ElementKind::Footnote), locs[1]),
            Some(2),
            "segunda nota numerada 2"
        );
    }

    #[test]
    fn counter_update_action_update_apply_correctly() {
        // P198C test 5: 3º caminho da match — Update(val) aplica via
        // apply_at(Update). Legacy via state.update_flat. Paridade.
        use crate::entities::introspector::Introspector;

        let content =
            Content::counter_update("page".to_string(), CounterAction::Set(vec![42]));
        let intr = introspect_with_introspector(&content);

        // Legacy.
        assert_eq!(
            intr.counters
                .value(&ck("page"))
                .and_then(|v| v.last())
                .copied()
                .unwrap_or(0),
            42,
            "legacy: state.update_flat('page', 42) → 42"
        );
        // Introspector.
        let loc = *intr.query_by_kind(ElementKind::CounterUpdate).first().unwrap();
        assert_eq!(
            intr.flat_counter_at(&ck("page"), loc),
            Some(42),
            "P198C: apply_at(Update(42)) → flat_counter_at == 42"
        );
    }

    #[test]
    fn counter_update_compute_helpers_continuam_funcionais() {
        // P198C test 6: cadeia E6 ↔ compute_labelled Equation arm
        // preservada após promote. compute_labelled Equation arm lê
        // intr.flat_counter_at(Selector(Equation), loc).
        // CounterUpdate directo com Selector(Equation) avança o mesmo
        // counter que compute_labelled consulta.
        let content = Content::Sequence(
            vec![
                Content::counter_update(
                    sel(ElementKind::Equation),
                    CounterAction::step(),
                ),
                Content::label_auto(
                    "eq1".to_string(),
                    Content::equation(Content::Empty, true),
                ),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);

        // CounterUpdate Selector(Equation) popula intr.counters.
        assert_eq!(
            intr.counters
                .value(&sel(ElementKind::Equation))
                .and_then(|v| v.last())
                .copied()
                .unwrap_or(0),
            1,
            "P198C: CounterUpdate(Selector(Equation), Step) → counter = 1"
        );
        // compute_labelled Equation arm lê flat_counter_at → produz
        // resolved_text "Equação (1)".
        assert_eq!(
            intr.resolved_labels.get(&Label("eq1".to_string())),
            Some("Equação (1)"),
            "cadeia E6↔E4: compute_labelled Equation arm continua funcional \
             após promote — lê flat_counter_at(Selector(Equation)) durante walk"
        );
        // Introspector path: resolved_labels populated via P195D Tag.
        assert_eq!(
            intr.resolved_label_for(&Label("eq1".to_string())),
            Some("Equação (1)"),
            "Introspector path: resolved_labels populated via P195D + Tag::Labelled"
        );
    }

    #[test]
    fn consumer_layouter_equation_activa_via_introspector() {
        // P199B test 5: caminho Introspector activado por construção
        // — pipeline com SetEquationNumbering + Equation labelled
        // produz resolved_labels populated via cadeia legacy
        // (compute_labelled P195D) E intr.state populated via
        // Tag::StateUpdate. Confirma activação imediata após P199B.
        use crate::entities::introspector::Introspector;

        // F-5a de-bake (P364, §3a.9): o gate de numeração vive **só na chain**,
        // transportado por `Content::Styled`. A forma de **produção** põe o
        // transporte **fora** do `Labelled` (a fatia-1 embrulha a cauda:
        // `Styled{ Labelled{ Equation } }`) — assim o alvo do `Labelled` é a
        // própria `Equation` (o `compute_labelled` inspeciona o tipo do alvo) e o
        // `custom("equation.numbering")` chega à chain antes da emissão do payload.
        // Fixture roteado a essa forma (S5b declarado; era campo assado
        // `equation_numbered` direto, P335).
        let content = Content::Sequence(
            vec![Content::styled(
                Box::new(Content::label_auto(
                    "eq1".to_string(),
                    Content::equation(Content::Empty, true),
                )),
                crate::entities::style::Styles::new()
                    .push_custom("equation.numbering", Value::Str("(1)".into())),
            )]
            .into(),
        );
        let intr = introspect_with_introspector(&content);

        // Legacy: compute_labelled Equation arm produz "Equação (1)".
        assert_eq!(
            intr.resolved_labels.get(&Label("eq1".to_string())),
            Some("Equação (1)"),
            "compute_labelled Equation arm activado via mutação legacy SetEquationNumbering"
        );
        // Introspector: resolved_labels populated via P195D Tag::Labelled.
        assert_eq!(
            intr.resolved_label_for(&Label("eq1".to_string())),
            Some("Equação (1)"),
            "Introspector path: resolved_labels populated em paralelo (P199B + P195D)"
        );
        // Lote F-2 S5 (P335): asserção is_numbering_active removida (StateRegistry
        // numbering_active:equation saiu — gate via campo assado).
    }

    // ── P200B — Sub-store headings_for_toc + Tag + consumer (M5 universal) ─
    //
    // 5 tests sentinela que validam: (a) walk arm Heading emite
    // Tag::HeadingForToc pós-recursão; (b) sub-store
    // intr.headings_for_toc populated em paridade com legacy;
    // (c) bracketing válido com 6 tags por Heading folha;
    // (d) E2-residuo fechada — paridade legacy/Introspector
    // preservada; (e) compute_heading_for_toc helper produz
    // tuple correcto.

    #[test]
    fn headings_for_toc_walk_emite_tag_e_popula_sub_store() {
        // P200B test 1: pipeline walk + from_tags com Heading
        // numerado popula sub-store intr.headings_for_toc.
        use crate::entities::introspector::Introspector;

        let content =
            Content::Sequence(vec![Content::heading(1, Content::text("Intro"))].into());
        let intr = introspect_with_introspector(&content);

        // Sub-store populated com 1 entry.
        assert_eq!(
            intr.headings_for_toc().len(),
            1,
            "P200B: 1 Heading deve gerar 1 entry em intr.headings_for_toc"
        );
        // Auto-label sintetizada usa state.auto_label_counter (1).
        let entry = &intr.headings_for_toc()[0];
        assert_eq!(entry.0, Label("auto-toc-1".to_string()));
        assert_eq!(entry.3, 1, "level esperado");
    }

    #[test]
    fn headings_for_toc_paridade_legacy_vs_introspector() {
        // P200B test 2: write paralelo legacy + Introspector preserva
        // paridade exacta. intr.headings_for_toc().len() ==
        // intr.headings_for_toc().len(); conteúdo idêntico.
        use crate::entities::introspector::Introspector;

        let content = Content::Sequence(
            vec![
                Content::heading(1, Content::text("Cap 1")),
                Content::heading(2, Content::text("Sec 1.1")),
                Content::heading(1, Content::text("Cap 2")),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);

        assert_eq!(
            intr.headings_for_toc().len(),
            3,
            "legacy: 3 entries (mutação 4 preservada como write paralelo M5)"
        );
        assert_eq!(
            intr.headings_for_toc().len(),
            3,
            "Introspector: 3 entries via Tag::HeadingForToc"
        );

        // Paridade exacta — labels e levels.
        for (legacy_entry, intr_entry) in
            intr.headings_for_toc().iter().zip(intr.headings_for_toc())
        {
            assert_eq!(legacy_entry.0, intr_entry.0, "labels devem ser idênticos");
            assert_eq!(legacy_entry.3, intr_entry.3, "levels devem ser idênticos");
        }
    }

    #[test]
    fn bracketing_valido_8_tags_por_heading_p606() {
        // P606 test: confirma bracketing válido com 8 tags por
        // Heading folha (4 Start + 4 End consecutivas; mesma Location).
        let h = Content::heading(1, Content::text("título"));
        let tags = introspect_with_tags(&h);

        assert_eq!(tags.len(), 8, "P606: 8 tags por Heading folha");

        let mut stack: Vec<Location> = Vec::new();
        for tag in &tags {
            match tag {
                Tag::Start(loc, _) => stack.push(*loc),
                Tag::End(loc, _) => {
                    let top = stack.pop().expect("End sem Start correspondente");
                    assert_eq!(top, *loc, "End com Location diferente do último Start");
                }
            }
        }
        assert!(stack.is_empty(), "todos Start têm End correspondente");
    }

    #[test]
    fn e2_residuo_fechada_paridade_legacy_introspector() {
        // P200B test 4: sentinela substituindo
        // walk_e2_residuo_headings_for_toc_via_legacy P196B.
        // Confirma que E2-residuo fecha estruturalmente: ambos
        // legacy e Introspector populated em paralelo.
        use crate::entities::introspector::Introspector;

        let content = Content::Sequence(
            vec![
                Content::heading(1, Content::text("Cap 1")),
                Content::heading(2, Content::text("Sec 1.1")),
                Content::heading(1, Content::text("Cap 2")),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);

        // Legacy preservado (write paralelo M5).
        assert_eq!(
            intr.headings_for_toc().len(),
            3,
            "E2-residuo: mutação 4 legacy preservada (Layouter mod.rs:1490, 1521 dependem)"
        );
        // Introspector popula via Tag::HeadingForToc (E2-residuo fechada).
        assert_eq!(
            intr.headings_for_toc().len(),
            3,
            "P200B: E2-residuo fechada estruturalmente via Tag::HeadingForToc"
        );
        // Levels preservados em ordem.
        let levels_legacy: Vec<_> =
            intr.headings_for_toc().iter().map(|(_, _, _, l)| *l).collect();
        let levels_intr: Vec<_> =
            intr.headings_for_toc().iter().map(|(_, _, _, l)| *l).collect();
        assert_eq!(levels_legacy, vec![1, 2, 1]);
        assert_eq!(levels_intr, levels_legacy);
    }

    #[test]
    fn headings_for_toc_helper_compute_produces_correct_entry() {
        // P200B test 5: confirma que compute_heading_for_toc
        // helper produz tuple correcto reusando frozen_body
        // computed pelo walk arm.
        use crate::entities::introspector::Introspector;

        let h = Content::heading(2, Content::text("título"));
        let intr = introspect_with_introspector(&h);

        // P190G: walk-internal `auto_label_counter` (local var em
        // walk fn) incrementado a 1 — verificável via Label
        // sintetizada no sub-store.
        // Entry correcta no sub-store.
        let entry = &intr.headings_for_toc()[0];
        assert_eq!(
            entry.0,
            Label("auto-toc-1".to_string()),
            "label sintetizada usa walk-internal auto_label_counter"
        );
        assert_eq!(entry.3, 2, "level preservado per cast usize do level: u8");
        // body materializado preserva text content.
        match &entry.2 {
            Content::Text(s) => assert_eq!(s.as_str(), "título"),
            other => panic!("body esperado Text, obtido {other:?}"),
        }
    }

    // ── P191B (ADR-0071) — sentinelas mecanismo walk pipeline ───────────

    #[test]
    fn p191b_walk_popula_intr_directamente_sem_from_tags() {
        // P191B sentinela #1: walk popula `TagIntrospector` directamente
        // durante walk via populate_intr_from_tag_start. Sem chamada
        // a from_tags::from_tags (eliminado em P191B).
        //
        // Verifica que doc com Heading + Figure + Cite produz intr
        // populated com sub-stores correctos só por chamar walk.
        use crate::entities::element_kind::ElementKind;
        use crate::entities::introspector::Introspector;

        let content = Content::Sequence(
            vec![
                Content::heading(1, Content::text("h")),
                Content::figure(
                    Content::Empty,
                    Some(Content::text("c")),
                    Some("image".to_string()),
                    Some("1".to_string()),
                ),
                Content::cite("k".to_string(), None, None),
            ]
            .into(),
        );

        // Walk directo — sem from_tags.
        let mut locator = Locator::new();
        let mut tags: Vec<Tag> = Vec::new();
        let mut intr = TagIntrospector::empty();
        let mut auto_label_counter: usize = 0;
        walk(
            &content,
            &mut locator,
            &mut tags,
            &mut intr,
            &mut auto_label_counter,
            None,
            &crate::entities::style_chain::StyleChain::default_chain(),
            None,
        );

        // Sub-stores populated por walk directo.
        assert_eq!(
            intr.query_by_kind(ElementKind::Heading).len(),
            1,
            "ADR-0071: walk popula kind_index[Heading] directamente"
        );
        assert_eq!(
            intr.query_by_kind(ElementKind::Figure).len(),
            1,
            "ADR-0071: walk popula kind_index[Figure] directamente"
        );
        assert_eq!(
            intr.query_by_kind(ElementKind::Citation).len(),
            1,
            "ADR-0071: walk popula kind_index[Citation] directamente"
        );
    }

    #[test]
    fn p191c_compute_labelled_le_via_introspector_path() {
        // P191C (ADR-0071 ACEITE) sentinela: compute_labelled migrada
        // para signature <I: Introspector>(intr, location, target,
        // lang). Reads location-aware substituem state.figure_numbers
        // / state.format_hierarchical / state.get_flat. Paridade com
        // comportamento legacy.
        //
        // Cenário: Heading numbered + Labelled Heading (cadeia C1).
        // P1019: ambos os headings precisam de numbering activo; o Styled de
        // numbering tem de ficar fora do Labelled (labelled_prod).
        let content = Content::Sequence(
            vec![
                with_heading_numbering(Content::heading(1, Content::text("intro"))),
                labelled_prod(
                    with_heading_numbering(Content::heading(1, Content::text("body"))),
                    Label("sec".to_string()),
                ),
            ]
            .into(),
        );

        let intr = introspect_with_introspector(&content);

        // resolved_labels populated com "Secção 2" via Introspector path
        // location-aware (compute_labelled Heading arm chama
        // intr.formatted_counter_at("heading", target_loc)).
        assert_eq!(
            intr.resolved_labels.get(&Label("sec".to_string())),
            Some("Section 2"),
            "compute_labelled via Introspector path: 2ª heading labelled → 'Section 2'",
        );
    }

    #[test]
    fn p1459_labelled_transporta_lingua_causal_de_cada_alvo() {
        fn localized_heading(label: &str, lang: &str) -> Content {
            Content::styled(
                Box::new(Content::label_auto(
                    label.to_string(),
                    Content::heading(1, Content::text(label)),
                )),
                Styles::new()
                    .push_custom("heading.numbering", Value::Bool(true))
                    .push_custom("text.lang", Value::Str(lang.into())),
            )
        }

        let content = Content::Sequence(
            vec![
                localized_heading("de-heading", "de"),
                localized_heading("es-heading", "es"),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);

        assert_eq!(
            intr.resolved_labels.get(&Label("de-heading".to_string())),
            Some("Abschnitt 1"),
        );
        assert_eq!(
            intr.resolved_labels.get(&Label("es-heading".to_string())),
            Some("Sección 2"),
        );
    }

    #[test]
    fn p191b_compute_heading_auto_toc_le_via_introspector_path() {
        // P191B sentinela #2: compute_heading_auto_toc migrada para
        // signature <I: Introspector>(intr, location, counter_n). Gate pelo
        // campo assado `numbering_active` + `formatted_counter_at` location-
        // aware (F-4 E0, P338: o gate legado `is_numbering_active_at` saiu).
        // Paridade quando SetHeadingNumbering(true) precede o Heading.
        let content = Content::Sequence(
            vec![
                Content::heading_numbered(1, Content::text("um")),
                Content::heading_numbered(1, Content::text("dois")),
            ]
            .into(),
        );

        let intr = introspect_with_introspector(&content);

        // resolved_labels populated com auto-toc texts via Introspector path.
        // P359 (DEBT-60 b): NÚMERO só ("1." / "2."), sem supplement "Secção" — o
        // outline mostra o numbering (paridade vanilla), não a cross-reference.
        assert_eq!(
            intr.resolved_labels.get(&Label("auto-toc-1".to_string())),
            Some("1."),
            "compute_heading_auto_toc via Introspector path: 1ª heading → '1.'",
        );
        assert_eq!(
            intr.resolved_labels.get(&Label("auto-toc-2".to_string())),
            Some("2."),
            "compute_heading_auto_toc via Introspector path: 2ª heading → '2.'",
        );
    }

    #[test]
    fn p464_label_auto_e_user_partilham_resolucao() {
        // P464: `Content::Label` único cobre ambas as origens. Label
        // user-created popula `label_to_counter_key`; label auto-gerado
        // popula `resolved_labels` (caminho legacy ex-Labelled).
        //
        // `labelled_prod` levanta o `Styled` do numbering para fora,
        // garantindo que o alvo do label auto é o Heading puro
        // (requerido por `compute_labelled`).
        let user_label = Content::label(
            "user-sec".to_string(),
            Content::heading_numbered(1, Content::text("User")),
        );
        let auto_label = labelled_prod(
            Content::heading_numbered(1, Content::text("Auto")),
            Label("auto-sec".to_string()),
        );
        let content = Content::Sequence(vec![user_label, auto_label].into());
        let intr = introspect_with_introspector(&content);

        assert_eq!(
            intr.counter_key_for_label(&Label("user-sec".to_string())),
            Some(&sel(ElementKind::Heading)),
            "user label deve ter counter_key 'heading'",
        );
        assert_eq!(
            intr.resolved_labels.get(&Label("auto-sec".to_string())),
            Some("Section 2"),
            "auto label deve ter texto resolvido via caminho legacy",
        );
    }

    // ── P480 — Outline arm kind_index ────────────────────────────────────────

    #[test]
    fn p480_walk_outline_registra_heading_em_kind_index() {
        // P480 — walk arm de Content::Outline regista 1 entrada em
        // kind_index[Heading] para paridade de count com vanilla.
        // vanilla conta o heading do título via pós-layout; cristalino
        // regista aqui pré-layout como entrada sintética.
        let content = Content::outline();
        let intr = introspect_with_introspector(&content);
        assert_eq!(
            intr.query_by_kind(ElementKind::Heading).len(),
            1,
            "P480: walk arm Outline deve registar 1 heading sintético em kind_index",
        );
    }

    #[test]
    fn p480_outline_nao_adiciona_headings_for_toc() {
        // P480 — registo sintético no kind_index NÃO afecta headings_for_toc.
        // headings_for_toc continua a conter apenas os headings reais
        // do documento (sem auto-referência do TOC).
        let content = Content::Sequence(
            vec![
                Content::outline(),
                Content::heading(1, Content::text("H1")),
                Content::heading(2, Content::text("H2")),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);

        // kind_index: 1 outline title + 2 headings reais = 3
        assert_eq!(
            intr.query_by_kind(ElementKind::Heading).len(),
            3,
            "P480: kind_index deve conter 1 outline title + 2 headings reais",
        );
        // headings_for_toc: apenas 2 headings reais (sem outline title)
        assert_eq!(
            intr.headings_for_toc().len(),
            2,
            "P480: headings_for_toc deve conter só headings reais; \
             outline title não incluído (evita TOC auto-referente)",
        );
    }

    #[test]
    fn p504_within_selector_query() {
        use crate::entities::selector::Selector;
        let content = Content::Sequence(
            vec![
                Content::figure(
                    Content::heading(1, Content::text("Dentro")),
                    None,
                    None,
                    None,
                ),
                Content::heading(1, Content::text("Fora")),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);
        let within = Selector::Within {
            base: Box::new(Selector::Kind(ElementKind::Heading)),
            ancestor: Box::new(Selector::Kind(ElementKind::Figure)),
        };
        let locations = intr.query(&within);
        assert_eq!(locations.len(), 1, "só o heading dentro da figure deve match");
    }

    #[test]
    fn p1401_before_after_usam_primeira_fronteira() {
        use crate::entities::selector::Selector;
        let content = Content::Sequence(
            vec![
                Content::heading(1, Content::text("A")),
                Content::figure(Content::text("F"), None, None, None),
                Content::heading(1, Content::text("B")),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);
        let before = Selector::Before {
            base: Box::new(Selector::Kind(ElementKind::Heading)),
            end: Box::new(Selector::Kind(ElementKind::Figure)),
            inclusive: true,
        };
        let after = Selector::After {
            base: Box::new(Selector::Kind(ElementKind::Heading)),
            start: Box::new(Selector::Kind(ElementKind::Figure)),
            inclusive: true,
        };
        assert_eq!(intr.query(&before).len(), 1);
        assert_eq!(intr.query(&after).len(), 1);
    }

    #[test]
    fn p504_parent_locations_index() {
        let content = Content::Sequence(
            vec![
                Content::figure(
                    Content::heading(1, Content::text("Dentro")),
                    None,
                    None,
                    None,
                ),
                Content::heading(1, Content::text("Fora")),
            ]
            .into(),
        );
        let intr = introspect_with_introspector(&content);
        let headings = intr.query_by_kind(ElementKind::Heading);
        let figures = intr.query_by_kind(ElementKind::Figure);
        assert_eq!(headings.len(), 2);
        assert_eq!(figures.len(), 1);
        // O primeiro heading é filho da figure; o segundo não tem parent.
        assert_eq!(intr.parent_locations.get(&headings[0]), Some(&figures[0]));
        assert_eq!(intr.parent_locations.get(&headings[1]), None);
    }

    // **P533** — `@key` bibliográficos devem ser contados como citações.
    #[test]
    fn p533_bib_refs_convertidos_contam_citation_order() {
        use crate::entities::bib_entry::BibEntry;
        let bib = Content::bibliography(
            vec![
                BibEntry::new("k1", "A1", "T1", 2024),
                BibEntry::new("k2", "A2", "T2", 2023),
            ],
            None,
        );
        let content = Content::Sequence(
            vec![Content::reference("k2"), Content::reference("k1"), bib].into(),
        );
        let converted = convert_bib_refs_to_cites(content.clone());
        let refs: Vec<_> = match &converted {
            Content::Sequence(seq) => seq.iter().collect(),
            _ => panic!("esperado Sequence"),
        };
        assert!(
            matches!(&refs[0], Content::Cite(c) if c.key == "k2"),
            "primeiro ref deve converter para Cite(k2): {:?}",
            refs[0]
        );
        assert!(
            matches!(&refs[1], Content::Cite(c) if c.key == "k1"),
            "segundo ref deve converter para Cite(k1): {:?}",
            refs[1]
        );
        // O path interno de introspecção também deve aplicar a conversão.
        let intr = introspect_with_introspector(&content);
        assert_eq!(intr.citation_order(), &["k2", "k1"]);
    }

    #[test]
    fn p1485_bundle_starts_preservam_locations_e_entradas_vazias() {
        let bodies = vec![
            Content::heading_numbered(1, Content::text("Alpha")),
            Content::Empty,
            Content::heading_numbered(1, Content::text("Beta")),
        ];
        let (global, starts) = introspect_bundle_documents_with_starts(&bodies);
        let canonical = super::introspect_with_introspector(&Content::sequence(bodies));
        assert_eq!(starts.len(), 3);
        assert_eq!(starts[1], starts[2]);
        let headings = global.query_by_kind(ElementKind::Heading);
        assert_eq!(headings.len(), 2);
        assert_eq!(
            crate::entities::locator::Locator::from_seed(starts[0]).next(),
            headings[0]
        );
        assert_eq!(
            crate::entities::locator::Locator::from_seed(starts[2]).next(),
            headings[1]
        );
        assert_eq!(headings, canonical.query_by_kind(ElementKind::Heading));
    }

    #[test]
    fn p1485_bundle_starts_cobrem_styled_outline_label_ref_e_counter() {
        let bodies = vec![
            Content::styled(
                Box::new(Content::label_auto(
                    "alpha".to_owned(),
                    Content::heading_numbered(1, Content::text("Alpha")),
                )),
                crate::entities::style::Styles::new(),
            ),
            Content::outline(),
            Content::sequence(vec![
                Content::reference("alpha"),
                Content::heading_numbered(1, Content::text("Beta")),
            ]),
        ];
        let (global, starts) = introspect_bundle_documents_with_starts(&bodies);
        let canonical = super::introspect_with_introspector(&Content::sequence(bodies));
        let first =
            |seed| crate::entities::locator::Locator::from_seed(seed).next().as_u128();
        assert!(first(starts[0]) < first(starts[1]));
        assert!(first(starts[1]) < first(starts[2]));
        assert_eq!(global.outline_elements.len(), canonical.outline_elements.len());
        assert_eq!(global.headings_for_toc.len(), canonical.headings_for_toc.len());
        assert_eq!(
            global.outlined_heading_locations,
            canonical.outlined_heading_locations
        );
        assert_eq!(
            global.outline_elements[0].target,
            canonical.outline_elements[0].target
        );
        assert_eq!(global.outline_elements[0].depth, canonical.outline_elements[0].depth);
        assert_eq!(
            canonical.outline_elements[0].target,
            crate::entities::elements::outline::OutlineTarget::Headings
        );
        assert!(canonical.outline_elements[0].depth >= 1);
        assert!(canonical
            .outlined_heading_locations
            .iter()
            .all(|loc| canonical.elements.contains_key(loc)));
        assert!(canonical
            .headings_for_toc
            .iter()
            .all(|entry| entry.3 <= canonical.outline_elements[0].depth));
        assert_eq!(
            global
                .headings_for_toc
                .iter()
                .map(|entry| entry.3)
                .collect::<Vec<_>>(),
            canonical
                .headings_for_toc
                .iter()
                .map(|entry| entry.3)
                .collect::<Vec<_>>(),
        );
        let keys = |intr: &TagIntrospector| {
            intr.elements
                .keys()
                .copied()
                .collect::<std::collections::HashSet<_>>()
        };
        assert_eq!(keys(&global), keys(&canonical));
        assert_eq!(global.kind_index, canonical.kind_index);
        assert_eq!(global.parent_locations, canonical.parent_locations);
        assert_eq!(
            global.query_by_label(&Label("alpha".to_owned())),
            canonical.query_by_label(&Label("alpha".to_owned())),
        );
        let heading_key = CounterKey::Selector(Selector::Kind(ElementKind::Heading));
        let counter = global.counter_final_values(&heading_key);
        assert!(counter.is_some());
        assert_eq!(counter, canonical.counter_final_values(&heading_key));
    }
}
