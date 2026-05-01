//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/introspect/from_tags.md
//! @prompt-hash 9254a648
//! @layer L1
//! @updated 2026-04-30
//!
//! `from_tags` — construtor de `TagIntrospector` a partir de `&[Tag]`.
//! P165 sub-passo .E (M3 Introspection).
//!
//! Single pass linear sobre tags. Match exaustivo sobre
//! `ElementPayload` para forçar revisão quando variant novo for
//! adicionado.

use crate::entities::element_kind::ElementKind;
use crate::entities::element_payload::ElementPayload;
use crate::entities::introspector::TagIntrospector;
use crate::entities::tag::Tag;

/// Constrói `TagIntrospector` a partir da sequência de tags emitida
/// pelo walk em `rules/introspect.rs::walk` (P162).
///
/// Para cada `Tag::Start`: actualiza sub-stores (label, counter,
/// kind_index). Para cada `Tag::End`: ignora em M3 (hash será usado
/// em M7+ para detecção de mudança fixpoint).
pub fn from_tags(tags: &[Tag]) -> TagIntrospector {
    let mut intr = TagIntrospector::empty();

    for tag in tags {
        match tag {
            Tag::Start(loc, info) => {
                if let Some(label) = &info.label {
                    intr.labels.add(label.clone(), *loc);
                }
                match &info.payload {
                    ElementPayload::Heading { depth, counter_update: _, .. } => {
                        intr.kind_index
                            .entry(ElementKind::Heading)
                            .or_default()
                            .push(*loc);
                        // P170 (M9 sub-passo 2): apply_hierarchical em vez
                        // de apply flat — paridade com walk arm
                        // `Content::Heading` em introspect.rs:279 que faz
                        // `state.step_hierarchical("heading", *level as usize)`.
                        // Resolve lacuna #5 (format_hierarchical hierárquico).
                        // counter_update é ignorado para Heading — depth é a
                        // fonte autoritativa para hierarquia.
                        intr.counters.apply_hierarchical(
                            "heading".to_string(),
                            *depth as usize,
                        );
                    }
                    ElementPayload::Figure { counter_update, is_counted, .. } => {
                        intr.kind_index
                            .entry(ElementKind::Figure)
                            .or_default()
                            .push(*loc);
                        intr.counters.apply(
                            "figure".to_string(),
                            counter_update.clone(),
                        );
                        // P168 (M5): se figura é numerada+captioned E
                        // tem label associada, indexar em
                        // figure_label_numbers com número 1-based.
                        // Paridade com walk arm `Content::Labelled` em
                        // introspect.rs:362+ que aplica mesmo filtro.
                        if *is_counted {
                            if let Some(label) = &info.label {
                                let next_num = intr.figure_label_numbers.len() + 1;
                                intr.figure_label_numbers
                                    .entry(label.clone())
                                    .or_insert(next_num);
                            }
                        }
                    }
                    ElementPayload::Citation { .. } => {
                        intr.kind_index
                            .entry(ElementKind::Citation)
                            .or_default()
                            .push(*loc);
                        // Citation não tem campo counter_update —
                        // sem update no CounterRegistry.
                    }
                    // P169 (M9): Metadata acumula em MetadataStore.
                    ElementPayload::Metadata { value } => {
                        intr.kind_index
                            .entry(ElementKind::Metadata)
                            .or_default()
                            .push(*loc);
                        intr.metadata.add((**value).clone());
                    }
                }
            }
            Tag::End(_, _) => {
                // M3: hash não usado. M7+ fixpoint utilizará para
                // detecção de mudança cross-iteration.
            }
        }
    }

    intr
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::counter_update::CounterUpdate;
    use crate::entities::element_info::ElementInfo;
    use crate::entities::introspector::Introspector;
    use crate::entities::label::Label;
    use crate::entities::location::Location;

    fn loc(raw: u128) -> Location {
        Location::from_raw(raw)
    }

    fn lbl(s: &str) -> Label {
        Label(s.to_string())
    }

    fn heading_payload() -> ElementPayload {
        ElementPayload::Heading {
            depth:          1,
            body_hash:      0,
            counter_update: CounterUpdate::Step,
        }
    }

    fn figure_payload() -> ElementPayload {
        ElementPayload::Figure {
            kind:           Some("image".into()),
            counter_update: CounterUpdate::Step,
            is_counted:     true,
        }
    }

    fn citation_payload(key: &str) -> ElementPayload {
        ElementPayload::Citation { key: key.to_string() }
    }

    #[test]
    fn vazio_produz_introspector_vazio() {
        let i = from_tags(&[]);
        assert_eq!(i.query_by_kind(ElementKind::Heading), Vec::<Location>::new());
        assert!(i.labels.is_empty());
        assert!(i.counters.is_empty());
    }

    #[test]
    fn um_heading_popula_kind_index_e_counter() {
        let tags = vec![
            Tag::Start(loc(1), ElementInfo::new(heading_payload())),
            Tag::End(loc(1), 0xdead),
        ];
        let i = from_tags(&tags);
        assert_eq!(i.query_by_kind(ElementKind::Heading), vec![loc(1)]);
        assert_eq!(i.counters.value("heading"), Some(&[1usize][..]));
    }

    #[test]
    fn heading_com_label_popula_label_registry() {
        let tags = vec![
            Tag::Start(
                loc(2),
                ElementInfo::with_label(heading_payload(), lbl("intro")),
            ),
            Tag::End(loc(2), 0),
        ];
        let i = from_tags(&tags);
        assert_eq!(i.query_by_label(&lbl("intro")), Some(loc(2)));
    }

    #[test]
    fn tres_headings_produzem_counter_em_tres() {
        let tags = vec![
            Tag::Start(loc(1), ElementInfo::new(heading_payload())),
            Tag::End(loc(1), 0),
            Tag::Start(loc(2), ElementInfo::new(heading_payload())),
            Tag::End(loc(2), 0),
            Tag::Start(loc(3), ElementInfo::new(heading_payload())),
            Tag::End(loc(3), 0),
        ];
        let i = from_tags(&tags);
        assert_eq!(i.query_by_kind(ElementKind::Heading), vec![loc(1), loc(2), loc(3)]);
        assert_eq!(i.counters.value("heading"), Some(&[3usize][..]));
    }

    #[test]
    fn sequencia_mista_isola_por_kind() {
        let tags = vec![
            Tag::Start(loc(1), ElementInfo::new(heading_payload())),
            Tag::End(loc(1), 0),
            Tag::Start(loc(2), ElementInfo::new(figure_payload())),
            Tag::End(loc(2), 0),
            Tag::Start(loc(3), ElementInfo::new(citation_payload("k"))),
            Tag::End(loc(3), 0),
        ];
        let i = from_tags(&tags);
        assert_eq!(i.query_by_kind(ElementKind::Heading), vec![loc(1)]);
        assert_eq!(i.query_by_kind(ElementKind::Figure), vec![loc(2)]);
        assert_eq!(i.query_by_kind(ElementKind::Citation), vec![loc(3)]);
        assert_eq!(i.counters.value("heading"), Some(&[1usize][..]));
        assert_eq!(i.counters.value("figure"), Some(&[1usize][..]));
        // Citation não actualiza counter.
        assert_eq!(i.counters.value("cite"), None);
    }

    #[test]
    fn end_tags_sao_ignoradas_em_m3() {
        // Apenas End — sem Start — produz introspector vazio.
        let tags = vec![Tag::End(loc(1), 0xbeef), Tag::End(loc(2), 0xcafe)];
        let i = from_tags(&tags);
        assert!(i.kind_index.is_empty());
        assert!(i.labels.is_empty());
        assert!(i.counters.is_empty());
    }

    // ── P168 (M5 sub-passo 2) — figure_label_numbers ─────────────────────

    fn figure_payload_counted(is_counted: bool) -> ElementPayload {
        ElementPayload::Figure {
            kind:           Some("image".into()),
            counter_update: CounterUpdate::Step,
            is_counted,
        }
    }

    #[test]
    fn figura_numerada_com_label_popula_figure_label_numbers() {
        let tags = vec![
            Tag::Start(
                loc(10),
                ElementInfo::with_label(figure_payload_counted(true), lbl("fig1")),
            ),
            Tag::End(loc(10), 0),
        ];
        let i = from_tags(&tags);
        assert_eq!(i.figure_number_for_label(&lbl("fig1")), Some(1));
    }

    #[test]
    fn figura_nao_numerada_nao_popula_figure_label_numbers() {
        // is_counted = false (sem numbering ou sem caption) → não indexa.
        let tags = vec![
            Tag::Start(
                loc(11),
                ElementInfo::with_label(figure_payload_counted(false), lbl("nofig")),
            ),
            Tag::End(loc(11), 0),
        ];
        let i = from_tags(&tags);
        assert_eq!(i.figure_number_for_label(&lbl("nofig")), None);
        // Mas a figura ainda aparece em kind_index (todas as figuras são indexadas).
        assert_eq!(i.query_by_kind(ElementKind::Figure), vec![loc(11)]);
    }

    #[test]
    fn figuras_numeradas_recebem_numeros_sequenciais() {
        // 3 figuras numeradas+labelled produzem 1, 2, 3.
        let tags = vec![
            Tag::Start(loc(20), ElementInfo::with_label(figure_payload_counted(true), lbl("a"))),
            Tag::End(loc(20), 0),
            Tag::Start(loc(21), ElementInfo::with_label(figure_payload_counted(true), lbl("b"))),
            Tag::End(loc(21), 0),
            Tag::Start(loc(22), ElementInfo::with_label(figure_payload_counted(true), lbl("c"))),
            Tag::End(loc(22), 0),
        ];
        let i = from_tags(&tags);
        assert_eq!(i.figure_number_for_label(&lbl("a")), Some(1));
        assert_eq!(i.figure_number_for_label(&lbl("b")), Some(2));
        assert_eq!(i.figure_number_for_label(&lbl("c")), Some(3));
    }

    #[test]
    fn figura_sem_label_nao_aparece_em_figure_label_numbers() {
        // is_counted = true mas sem label → nada para indexar por label.
        let tags = vec![
            Tag::Start(loc(30), ElementInfo::new(figure_payload_counted(true))),
            Tag::End(loc(30), 0),
        ];
        let i = from_tags(&tags);
        assert!(i.figure_label_numbers.is_empty());
        // kind_index ainda contém a figura.
        assert_eq!(i.query_by_kind(ElementKind::Figure), vec![loc(30)]);
    }

    // ── P169 (M9 sub-passo 1) — Metadata feature ─────────────────────────

    use crate::entities::value::Value;

    #[test]
    fn metadata_popula_store_e_kind_index() {
        let tags = vec![
            Tag::Start(
                loc(40),
                ElementInfo::new(ElementPayload::Metadata {
                    value: Box::new(Value::Int(42)),
                }),
            ),
            Tag::End(loc(40), 0),
        ];
        let i = from_tags(&tags);
        // Store populado.
        assert_eq!(i.metadata.query(), &[Value::Int(42)]);
        // kind_index recebe Metadata kind.
        assert_eq!(i.query_by_kind(ElementKind::Metadata), vec![loc(40)]);
        // Counter NÃO é tocado por Metadata.
        assert_eq!(i.counters.value("metadata"), None);
    }

    #[test]
    fn multiplos_metadata_preservam_ordem() {
        let tags = vec![
            Tag::Start(
                loc(50),
                ElementInfo::new(ElementPayload::Metadata {
                    value: Box::new(Value::Int(1)),
                }),
            ),
            Tag::End(loc(50), 0),
            Tag::Start(
                loc(51),
                ElementInfo::new(ElementPayload::Metadata {
                    value: Box::new(Value::Int(2)),
                }),
            ),
            Tag::End(loc(51), 0),
            Tag::Start(
                loc(52),
                ElementInfo::new(ElementPayload::Metadata {
                    value: Box::new(Value::Int(3)),
                }),
            ),
            Tag::End(loc(52), 0),
        ];
        let i = from_tags(&tags);
        assert_eq!(
            i.metadata.query(),
            &[Value::Int(1), Value::Int(2), Value::Int(3)]
        );
    }
}
