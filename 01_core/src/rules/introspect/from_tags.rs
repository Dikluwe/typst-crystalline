//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/introspect/from_tags.md
//! @prompt-hash 1164f135
//! @layer L1
//! @updated 2026-04-30
//!
//! `from_tags` — construtor de `TagIntrospector` a partir de `&[Tag]`.
//! P165 sub-passo .E (M3 Introspection).
//!
//! Single pass linear sobre tags. Match exaustivo sobre
//! `ElementPayload` para forçar revisão quando variant novo for
//! adicionado.

use crate::entities::args::Args;
use crate::entities::element_kind::ElementKind;
use crate::entities::element_payload::ElementPayload;
use crate::entities::engine::Engine;
use crate::entities::introspector::TagIntrospector;
use crate::entities::state_update::StateUpdate;
use crate::entities::tag::Tag;
use crate::entities::value::Value;
use crate::rules::eval::closures::apply_func;
use crate::rules::eval::EvalContext;

/// Constrói `TagIntrospector` a partir da sequência de tags emitida
/// pelo walk em `rules/introspect.rs::walk` (P162).
///
/// Para cada `Tag::Start`: actualiza sub-stores (label, counter,
/// kind_index). Para cada `Tag::End`: ignora em M3 (hash será usado
/// em M7+ para detecção de mudança fixpoint).
///
/// **P173 (M9 sub-passo 5)**: aceita `Engine` e `EvalContext`
/// opcionais. Quando ambos `Some`, `StateUpdate::Func` é avaliada
/// via `apply_func(fn, Args::positional(vec![curr_value]), ctx, engine)`.
/// Quando algum `None`, `Func` é silenciosamente ignorada (defensive,
/// coerente com P171). API legacy `from_tags(&tags, None, None)`
/// preservada.
pub fn from_tags(
    tags:       &[Tag],
    mut engine: Option<&mut Engine<'_>>,
    mut ctx:    Option<&mut EvalContext>,
) -> TagIntrospector {
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
                        // P177: variante `_at` regista snapshot na history
                        // para suportar `value_at(key, location)` (counter.at).
                        intr.counters.apply_hierarchical_at(
                            "heading".to_string(),
                            *depth as usize,
                            *loc,
                        );
                    }
                    ElementPayload::Figure { kind, counter_update, is_counted, .. } => {
                        intr.kind_index
                            .entry(ElementKind::Figure)
                            .or_default()
                            .push(*loc);
                        // P184B: chave per-kind `figure:{kind}` (default
                        // `"image"` replicando `introspect.rs:391` e
                        // `mod.rs:431`). Promove convenção documentada
                        // em `element_payload.rs:52` mas não
                        // implementada até aqui. Suporta C3 desbloqueio
                        // (consumer em `mod.rs:435–439`).
                        let kind_key = kind.as_deref().unwrap_or("image");
                        intr.counters.apply_at(
                            format!("figure:{}", kind_key),
                            counter_update.clone(),
                            *loc,
                        );
                        // P177: variante `_at` regista snapshot. Chave
                        // global `"figure"` mantida em paralelo durante
                        // janela compat M6 (P184A cláusula 5; dead code
                        // factual — sem consumers actuais — preservado
                        // por simetria com walk legacy `state.figure_numbers`
                        // que também não é copiado ao Layouter).
                        intr.counters.apply_at(
                            "figure".to_string(),
                            counter_update.clone(),
                            *loc,
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
                    // P171 (M9): State init no StateRegistry.
                    ElementPayload::State { key, init } => {
                        intr.kind_index
                            .entry(ElementKind::State)
                            .or_default()
                            .push(*loc);
                        intr.state.init(key.clone(), (**init).clone(), *loc);
                    }
                    // P178: Outline indexado em kind_index. Sem
                    // counter/registry adicional — feature minimal.
                    ElementPayload::Outline => {
                        intr.kind_index
                            .entry(ElementKind::Outline)
                            .or_default()
                            .push(*loc);
                    }
                    // P181E: indexa em kind_index + popula bib_store
                    // (cláusulas 2/3 P181A — extend + or_insert).
                    // Numeração 1-based contínua sobre `numbers_len()`
                    // do bib_store; replica
                    // `state.bib_numbers.len() as u32 + 1` em walk arm
                    // `Content::Bibliography` (introspect.rs:569).
                    // `numbers_len()` cresce só em keys novas
                    // (semântica `or_insert`); duplicates não
                    // incrementam — paridade com walk arm.
                    ElementPayload::Bibliography { entries } => {
                        intr.kind_index
                            .entry(ElementKind::Bibliography)
                            .or_default()
                            .push(*loc);
                        let entries_owned = entries.clone();
                        for entry in &entries_owned {
                            let next_num = intr.bib_store.numbers_len() as u32 + 1;
                            intr.bib_store
                                .assign_number(entry.key.clone(), next_num);
                        }
                        intr.bib_store.add_bibliography(entries_owned);
                    }
                    // P171 (M9) + P173: StateUpdate aplica Set directamente;
                    // Func é avaliada via apply_func quando Engine+ctx
                    // estão disponíveis, ou silenciosamente ignorada caso
                    // contrário.
                    ElementPayload::StateUpdate { key, update } => {
                        intr.kind_index
                            .entry(ElementKind::StateUpdate)
                            .or_default()
                            .push(*loc);
                        match update {
                            StateUpdate::Set(value) => {
                                // P182C: auto-init na primeira ocorrência.
                                // Suporta state interno (`numbering_active:*`
                                // emitido por `Content::SetHeadingNumbering`)
                                // que não tem `Content::State` antecedente.
                                // Userspace `Content::State` continua a
                                // inicializar via arm dedicado acima — este
                                // ramo só auto-inicializa quando key ainda
                                // não foi vista. Ocorrências subsequentes
                                // seguem o caminho normal `state.update`.
                                if intr.state.value_at(key, *loc).is_none() {
                                    intr.state.init(key.clone(), (**value).clone(), *loc);
                                } else {
                                    intr.state.update(key.clone(), (**value).clone(), *loc);
                                }
                            }
                            StateUpdate::Func(func) => {
                                if let (Some(eng), Some(c)) =
                                    (engine.as_deref_mut(), ctx.as_deref_mut())
                                {
                                    if let Some(curr) =
                                        intr.state.value_at(key, *loc).cloned()
                                    {
                                        let args = Args::positional(vec![curr]);
                                        if let Ok(new_value) =
                                            apply_func(func.clone(), args, c, eng)
                                        {
                                            intr.state.update(
                                                key.clone(),
                                                new_value,
                                                *loc,
                                            );
                                        }
                                        // Err: defensive ignore — refino
                                        // futuro pode propagar via Sink.
                                    }
                                    // value_at == None: defensive ignore
                                    // (P171 padrão "update sem init").
                                }
                                // engine/ctx ausentes: defensive ignore.
                            }
                        }
                    }
                    // P186E — arm Equation completo. P186B introduziu
                    // stub no-op; P186D estendeu com `kind_index`
                    // populate (cláusula gate trivial para passar test
                    // P185D); P186E adiciona counter logic gated por
                    // `block && state.value_at("numbering_active:equation",
                    // loc) == Some(Bool(true))` (location-aware,
                    // Opção B em P186E `.B` por futureproofing).
                    //
                    // **Gate dormente em produção**: `Content::SetEquationNumbering`
                    // ainda não existe em cristalino (P186A §11.2),
                    // logo `state.value_at("numbering_active:equation", _)`
                    // é sempre `None` em runtime real → `apply_at`
                    // nunca dispara → counter `equation` permanece
                    // vazio. Eixo 2 P183C resolvido estruturalmente:
                    // P188 migra C2 com substitution-with-fallback;
                    // fallback legacy carrega até equation set rule
                    // materializar.
                    ElementPayload::Equation { block, counter_update } => {
                        intr.kind_index
                            .entry(ElementKind::Equation)
                            .or_default()
                            .push(*loc);
                        if *block
                            && matches!(
                                intr.state.value_at("numbering_active:equation", *loc),
                                Some(Value::Bool(true)),
                            )
                        {
                            intr.counters.apply_at(
                                "equation".to_string(),
                                counter_update.clone(),
                                *loc,
                            );
                        }
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
        let i = from_tags(&[], None, None);
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
        let i = from_tags(&tags, None, None);
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
        let i = from_tags(&tags, None, None);
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
        let i = from_tags(&tags, None, None);
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
        let i = from_tags(&tags, None, None);
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
        let i = from_tags(&tags, None, None);
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
        let i = from_tags(&tags, None, None);
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
        let i = from_tags(&tags, None, None);
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
        let i = from_tags(&tags, None, None);
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
        let i = from_tags(&tags, None, None);
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
        let i = from_tags(&tags, None, None);
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
        let i = from_tags(&tags, None, None);
        assert_eq!(
            i.metadata.query(),
            &[Value::Int(1), Value::Int(2), Value::Int(3)]
        );
    }

    // ── P173 (M9 sub-passo 5) — eval real de StateUpdate::Func ───────────

    use crate::entities::engine::Engine;
    use crate::entities::func::Func;
    use crate::entities::sink::Sink;
    use crate::entities::state_update::StateUpdate;
    use crate::entities::world_types::{Library, Route};
    use crate::entities::show::{RuleId, ShowRule};
    use crate::entities::style_chain::StyleChain;
    use std::sync::Arc;

    /// MockWorld minimal — paridade com o teste do contracts/world.rs.
    struct MockWorld {
        library: Library,
        book:    crate::entities::font_book::FontBook,
        main_id: crate::entities::file_id::FileId,
    }

    impl crate::contracts::world::World for MockWorld {
        fn library(&self) -> &Library { &self.library }
        fn book(&self)    -> &crate::entities::font_book::FontBook { &self.book }
        fn main(&self)    -> crate::entities::file_id::FileId { self.main_id }
        fn source(&self, _: crate::entities::file_id::FileId)
            -> crate::entities::world_types::FileResult<crate::entities::source::Source>
        { Err(crate::entities::world_types::FileError::NotFound) }
        fn file(&self, _: crate::entities::file_id::FileId)
            -> crate::entities::world_types::FileResult<crate::entities::world_types::Bytes>
        { Err(crate::entities::world_types::FileError::NotFound) }
        fn font(&self, _: usize) -> Option<crate::entities::world_types::Font> { None }
        fn today(&self, _: Option<i64>) -> Option<crate::entities::world_types::Datetime> { None }
    }

    fn make_world() -> MockWorld {
        MockWorld {
            library: Library::new(),
            book:    crate::entities::font_book::FontBook::new(),
            main_id: crate::entities::file_id::FileId::from_raw(
                std::num::NonZeroU16::new(1).unwrap()
            ),
        }
    }

    /// Native `x => x + 1` para Funcs Int.
    fn add_one_native(
        _ctx: &mut crate::rules::eval::EvalContext,
        args: &crate::entities::args::Args,
        _world: &dyn crate::contracts::world::World,
        _current_file: crate::entities::file_id::FileId,
        _figure_numbering: Option<&str>,
    ) -> crate::entities::source_result::SourceResult<crate::entities::value::Value> {
        match args.items.first() {
            Some(crate::entities::value::Value::Int(n)) =>
                Ok(crate::entities::value::Value::Int(n + 1)),
            _ => Ok(crate::entities::value::Value::None),
        }
    }

    /// Native `x => x * 10` para Funcs Int.
    fn times_ten_native(
        _ctx: &mut crate::rules::eval::EvalContext,
        args: &crate::entities::args::Args,
        _world: &dyn crate::contracts::world::World,
        _current_file: crate::entities::file_id::FileId,
        _figure_numbering: Option<&str>,
    ) -> crate::entities::source_result::SourceResult<crate::entities::value::Value> {
        match args.items.first() {
            Some(crate::entities::value::Value::Int(n)) =>
                Ok(crate::entities::value::Value::Int(n * 10)),
            _ => Ok(crate::entities::value::Value::None),
        }
    }

    /// Helper que constrói Engine + EvalContext locais e chama a closure.
    /// Engine não é Send/static, por isso construído inline em cada call.
    macro_rules! with_engine {
        ($world:expr, |$engine:ident, $ctx:ident| $body:block) => {{
            use comemo::Track;
            let world: &dyn crate::contracts::world::World = $world;
            let mut $ctx = crate::rules::eval::EvalContext::new();
            let route = Route::root().with_id(world.main());
            let mut styles = StyleChain::default_chain();
            let mut show_rules: Arc<[ShowRule]> = Arc::from([]);
            let mut active_guards: Vec<RuleId> = Vec::new();
            let current_file = world.main();
            let mut figure_numbering: Option<String> = None;
            let mut sink_local = Sink::new();
            let mut sink = sink_local.track_mut();
            let mut $engine = Engine {
                world,
                route: route.track(),
                styles: &mut styles,
                show_rules: &mut show_rules,
                active_guards: &mut active_guards,
                current_file,
                figure_numbering: &mut figure_numbering,
                sink: &mut sink,
            };
            $body
        }};
    }

    #[test]
    fn func_eval_aplica_callback_com_engine() {
        // P173: state init=0 + Func(add_one) com Engine → final value 1.
        let f = Func::native("add_one", add_one_native);
        let tags = vec![
            Tag::Start(
                loc(10),
                ElementInfo::new(ElementPayload::State {
                    key:  "c".to_string(),
                    init: Box::new(Value::Int(0)),
                }),
            ),
            Tag::End(loc(10), 0),
            Tag::Start(
                loc(20),
                ElementInfo::new(ElementPayload::StateUpdate {
                    key:    "c".to_string(),
                    update: StateUpdate::Func(f),
                }),
            ),
            Tag::End(loc(20), 0),
        ];
        let world = make_world();
        let intr = with_engine!(&world, |engine, ctx| {
            from_tags(&tags, Some(&mut engine), Some(&mut ctx))
        });
        assert_eq!(intr.state_final_value("c"), Some(&Value::Int(1)));
    }

    #[test]
    fn func_eval_sem_init_e_defensive_ignore() {
        // P173: Func update sem init prévio → registry inalterado.
        let f = Func::native("add_one", add_one_native);
        let tags = vec![
            Tag::Start(
                loc(20),
                ElementInfo::new(ElementPayload::StateUpdate {
                    key:    "c".to_string(),
                    update: StateUpdate::Func(f),
                }),
            ),
            Tag::End(loc(20), 0),
        ];
        let world = make_world();
        let intr = with_engine!(&world, |engine, ctx| {
            from_tags(&tags, Some(&mut engine), Some(&mut ctx))
        });
        // Sem init → state vazio para "c".
        assert_eq!(intr.state_final_value("c"), None);
    }

    #[test]
    fn func_eval_sem_engine_e_defensive_ignore() {
        // P173: API legacy `from_tags(_, None, None)` ignora Func e
        // mantém init.
        let f = Func::native("add_one", add_one_native);
        let tags = vec![
            Tag::Start(
                loc(10),
                ElementInfo::new(ElementPayload::State {
                    key:  "c".to_string(),
                    init: Box::new(Value::Int(0)),
                }),
            ),
            Tag::End(loc(10), 0),
            Tag::Start(
                loc(20),
                ElementInfo::new(ElementPayload::StateUpdate {
                    key:    "c".to_string(),
                    update: StateUpdate::Func(f),
                }),
            ),
            Tag::End(loc(20), 0),
        ];
        // Sem engine → Func ignorada, state final == init.
        let intr = from_tags(&tags, None, None);
        assert_eq!(intr.state_final_value("c"), Some(&Value::Int(0)));
    }

    #[test]
    fn func_eval_sequencia_aplica_em_ordem() {
        // P173: state init=0 → +1 → *10 → final value 10 (não 20:
        // (0+1)*10).
        let f1 = Func::native("add_one", add_one_native);
        let f2 = Func::native("times_ten", times_ten_native);
        let tags = vec![
            Tag::Start(
                loc(10),
                ElementInfo::new(ElementPayload::State {
                    key:  "c".to_string(),
                    init: Box::new(Value::Int(0)),
                }),
            ),
            Tag::End(loc(10), 0),
            Tag::Start(
                loc(20),
                ElementInfo::new(ElementPayload::StateUpdate {
                    key:    "c".to_string(),
                    update: StateUpdate::Func(f1),
                }),
            ),
            Tag::End(loc(20), 0),
            Tag::Start(
                loc(30),
                ElementInfo::new(ElementPayload::StateUpdate {
                    key:    "c".to_string(),
                    update: StateUpdate::Func(f2),
                }),
            ),
            Tag::End(loc(30), 0),
        ];
        let world = make_world();
        let intr = with_engine!(&world, |engine, ctx| {
            from_tags(&tags, Some(&mut engine), Some(&mut ctx))
        });
        assert_eq!(intr.state_final_value("c"), Some(&Value::Int(10)));
    }

    // ── P181E — Bibliography arm popula BibStore ────────────────────────

    fn bib_entry(key: &str) -> crate::entities::bib_entry::BibEntry {
        crate::entities::bib_entry::BibEntry {
            key:          key.to_string(),
            author:       String::new(),
            title:        String::new(),
            year:         0,
            volume:       None,
            pages:        None,
            journal:      None,
            publisher:    None,
            url:          None,
            doi:          None,
            editor:       None,
            series:       None,
            note:         None,
            isbn:         None,
            location:     None,
            organization: None,
        }
    }

    #[test]
    fn bibliography_arm_popula_bib_store() {
        let tags = vec![
            Tag::Start(loc(1), ElementInfo::new(ElementPayload::Bibliography {
                entries: vec![bib_entry("a"), bib_entry("b")],
            })),
            Tag::End(loc(1), 0),
        ];
        let intr = from_tags(&tags, None, None);
        assert_eq!(intr.bib_store.len(), 2);
        assert!(intr.bib_store.entry_for_key("a").is_some());
        assert!(intr.bib_store.entry_for_key("b").is_some());
    }

    #[test]
    fn bibliography_arm_atribui_numeros_em_ordem() {
        let tags = vec![
            Tag::Start(loc(1), ElementInfo::new(ElementPayload::Bibliography {
                entries: vec![
                    bib_entry("primeiro"),
                    bib_entry("segundo"),
                    bib_entry("terceiro"),
                ],
            })),
            Tag::End(loc(1), 0),
        ];
        let intr = from_tags(&tags, None, None);
        assert_eq!(intr.bib_store.number_for_key("primeiro"), Some(1));
        assert_eq!(intr.bib_store.number_for_key("segundo"),  Some(2));
        assert_eq!(intr.bib_store.number_for_key("terceiro"), Some(3));
    }

    #[test]
    fn bibliography_multi_extend_replica_legacy() {
        // Cláusula 2 P181A: múltiplas Bibliography concatenam via extend.
        // Cláusula 3 P181A: numbering preserva primeiro número via or_insert.
        let tags = vec![
            Tag::Start(loc(1), ElementInfo::new(ElementPayload::Bibliography {
                entries: vec![bib_entry("a"), bib_entry("b")],
            })),
            Tag::End(loc(1), 0),
            Tag::Start(loc(2), ElementInfo::new(ElementPayload::Bibliography {
                entries: vec![bib_entry("c"), bib_entry("d")],
            })),
            Tag::End(loc(2), 0),
        ];
        let intr = from_tags(&tags, None, None);
        assert_eq!(intr.bib_store.len(), 4);
        assert_eq!(intr.bib_store.number_for_key("a"), Some(1));
        assert_eq!(intr.bib_store.number_for_key("b"), Some(2));
        assert_eq!(intr.bib_store.number_for_key("c"), Some(3));
        assert_eq!(intr.bib_store.number_for_key("d"), Some(4));
    }

    #[test]
    fn bibliography_arm_popula_kind_index() {
        let tags = vec![
            Tag::Start(loc(7), ElementInfo::new(ElementPayload::Bibliography {
                entries: vec![bib_entry("a")],
            })),
            Tag::End(loc(7), 0),
        ];
        let intr = from_tags(&tags, None, None);
        let bib_locations = intr.kind_index
            .get(&ElementKind::Bibliography)
            .expect("kind_index Bibliography deve estar populado");
        assert_eq!(bib_locations.len(), 1);
        assert_eq!(bib_locations[0], loc(7));
    }

    // ── P186E — Equation arm ─────────────────────────────────────────────

    fn equation_payload(block: bool) -> ElementPayload {
        ElementPayload::Equation {
            block,
            counter_update: CounterUpdate::Step,
        }
    }

    #[test]
    fn equation_arm_gate_dispara_block_e_state_active() {
        // Caso central: state activo + block=true → counter populado.
        // Pre-popula state via tag StateUpdate antes da equação.
        let tags = vec![
            Tag::Start(
                loc(1),
                ElementInfo::new(ElementPayload::StateUpdate {
                    key:    "numbering_active:equation".to_string(),
                    update: StateUpdate::Set(Box::new(Value::Bool(true))),
                }),
            ),
            Tag::End(loc(1), 0),
            Tag::Start(loc(10), ElementInfo::new(equation_payload(true))),
            Tag::End(loc(10), 0),
        ];
        let intr = from_tags(&tags, None, None);
        // kind_index populado.
        assert_eq!(
            intr.kind_index.get(&ElementKind::Equation).map(|v| v.len()),
            Some(1),
        );
        // Counter populado em loc(10).
        assert_eq!(intr.counters.value_at("equation", loc(10)), Some(&[1usize][..]));
    }

    #[test]
    fn equation_arm_gate_dorme_inline_mesmo_com_state_active() {
        // State activo + block=false → counter NÃO populado.
        let tags = vec![
            Tag::Start(
                loc(1),
                ElementInfo::new(ElementPayload::StateUpdate {
                    key:    "numbering_active:equation".to_string(),
                    update: StateUpdate::Set(Box::new(Value::Bool(true))),
                }),
            ),
            Tag::End(loc(1), 0),
            Tag::Start(loc(10), ElementInfo::new(equation_payload(false))),
            Tag::End(loc(10), 0),
        ];
        let intr = from_tags(&tags, None, None);
        // kind_index populado mesmo para inline.
        assert_eq!(
            intr.kind_index.get(&ElementKind::Equation).map(|v| v.len()),
            Some(1),
        );
        // Counter NÃO populado — gate bloqueia inline.
        assert_eq!(intr.counters.value_at("equation", loc(10)), None);
    }

    #[test]
    fn equation_arm_gate_dorme_state_ausente_caso_producao() {
        // **Caso central da produção actual**: sem state populado para
        // `numbering_active:equation` (Content::SetEquationNumbering
        // ausente em cristalino — P186A §11.2). Gate bloqueia mesmo
        // para block=true. Counter permanece vazio.
        let tags = vec![
            Tag::Start(loc(10), ElementInfo::new(equation_payload(true))),
            Tag::End(loc(10), 0),
        ];
        let intr = from_tags(&tags, None, None);
        // kind_index populado.
        assert_eq!(
            intr.kind_index.get(&ElementKind::Equation).map(|v| v.len()),
            Some(1),
        );
        // Counter vazio — gate dormente.
        assert_eq!(intr.counters.value_at("equation", loc(10)), None);
    }

    #[test]
    fn equation_arm_multiplas_block_sequencializam_counter() {
        // 3 equations block=true sequenciais, com state activo.
        // Counter avança 1, 2, 3 nas Locations correspondentes.
        let tags = vec![
            Tag::Start(
                loc(1),
                ElementInfo::new(ElementPayload::StateUpdate {
                    key:    "numbering_active:equation".to_string(),
                    update: StateUpdate::Set(Box::new(Value::Bool(true))),
                }),
            ),
            Tag::End(loc(1), 0),
            Tag::Start(loc(10), ElementInfo::new(equation_payload(true))),
            Tag::End(loc(10), 0),
            Tag::Start(loc(20), ElementInfo::new(equation_payload(true))),
            Tag::End(loc(20), 0),
            Tag::Start(loc(30), ElementInfo::new(equation_payload(true))),
            Tag::End(loc(30), 0),
        ];
        let intr = from_tags(&tags, None, None);
        assert_eq!(intr.counters.value_at("equation", loc(10)), Some(&[1usize][..]));
        assert_eq!(intr.counters.value_at("equation", loc(20)), Some(&[2usize][..]));
        assert_eq!(intr.counters.value_at("equation", loc(30)), Some(&[3usize][..]));
    }
}
