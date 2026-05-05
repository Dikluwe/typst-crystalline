//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/introspect/from_tags.md
//! @prompt-hash e7647593
//! @layer L1
//! @updated 2026-05-05
//!
//! **P191B (ADR-0071)** — pipeline redesign.
//!
//! `from_tags::from_tags` (P165 .E) **eliminado**. Walk fn em
//! `rules/introspect.rs::walk` agora popula `TagIntrospector`
//! directamente durante walk via `populate_intr_from_tag_start`
//! (todos os 12 ElementPayload variants). Pipeline simplificado:
//! `walk → return` (sem etapa post-walk de construção).
//!
//! Função residual `apply_state_funcs` faz **slim post-pass** para
//! `StateUpdate::Func` apenas — Funcs requerem `Engine + EvalContext`
//! para `apply_func` (não disponíveis em walk). Caller único:
//! `fixpoint::run_fixpoint` que tem ambos. Walk path legacy
//! (`introspect()`, `introspect_with_introspector()`) silenciosamente
//! ignora Funcs — coerente com semântica P171/P173 pré-P191B (sem
//! Engine = defensive ignore).

use crate::entities::args::Args;
use crate::entities::element_payload::ElementPayload;
use crate::entities::engine::Engine;
use crate::entities::introspector::TagIntrospector;
use crate::entities::state_update::StateUpdate;
use crate::entities::tag::Tag;
use crate::rules::eval::closures::apply_func;
use crate::rules::eval::EvalContext;

/// **P191B (ADR-0071)** — slim post-pass para `StateUpdate::Func`.
///
/// Walk fn popula `intr` directamente para todos os payloads excepto
/// `StateUpdate::Func` — Funcs requerem `Engine + EvalContext` para
/// `apply_func`. Esta função processa apenas tags `Tag::Start` com
/// `ElementPayload::StateUpdate { update: StateUpdate::Func(_) }`.
///
/// **Ordem location-monotónica**: walk emite tags em ordem de
/// Locator (counter incrementado por `next()`). Func updates aqui
/// processados na mesma ordem, anexados a `intr.state` history.
/// Para keys onde walk apenas inseriu Sets em locations anteriores
/// (caso comum), value_at(loc_func) devolve o valor correcto antes
/// da aplicação da Func.
///
/// Caller: `fixpoint::run_fixpoint` (vide P174). Path legacy não
/// chama esta função — Funcs ignoradas por design.
pub fn apply_state_funcs(
    tags:   &[Tag],
    intr:   &mut TagIntrospector,
    engine: &mut Engine<'_>,
    ctx:    &mut EvalContext,
) {
    for tag in tags {
        if let Tag::Start(loc, info) = tag {
            if let ElementPayload::StateUpdate { key, update } = &info.payload {
                if let StateUpdate::Func(func) = update {
                    if let Some(curr) = intr.state.value_at(key, *loc).cloned() {
                        let args = Args::positional(vec![curr]);
                        if let Ok(new_value) =
                            apply_func(func.clone(), args, ctx, engine)
                        {
                            intr.state.update(
                                key.clone(),
                                new_value,
                                *loc,
                            );
                        }
                        // Err: defensive ignore — refino futuro pode
                        // propagar via Sink.
                    }
                    // value_at == None: defensive ignore (P171 padrão
                    // "update sem init").
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::element_info::ElementInfo;
    use crate::entities::location::Location;
    use crate::entities::value::Value;

    fn loc(raw: u128) -> Location {
        Location::from_raw(raw)
    }

    // ── P191B (ADR-0071) — apply_state_funcs (slim Func post-pass) ──────

    use crate::entities::engine::Engine;
    use crate::entities::func::Func;
    use crate::entities::sink::Sink;
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
        // P191B: state init=0 + Func(add_one) com Engine → final value 1.
        // Pre-popula intr via simulação directa (sub-store init); Func
        // tag é processada por apply_state_funcs.
        let f = Func::native("add_one", add_one_native);
        let mut intr = TagIntrospector::empty();
        // Simula walk: state.init em loc 10 (do `Content::State`).
        intr.state.init("c".to_string(), Value::Int(0), loc(10));
        // Tag Func emitida pelo walk em loc 20.
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
        with_engine!(&world, |engine, ctx| {
            apply_state_funcs(&tags, &mut intr, &mut engine, &mut ctx);
        });
        assert_eq!(intr.state.final_value("c"), Some(&Value::Int(1)));
    }

    #[test]
    fn func_eval_sem_init_e_defensive_ignore() {
        // P191B: Func update sem init prévio → registry inalterado.
        let f = Func::native("add_one", add_one_native);
        let mut intr = TagIntrospector::empty();
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
        with_engine!(&world, |engine, ctx| {
            apply_state_funcs(&tags, &mut intr, &mut engine, &mut ctx);
        });
        // Sem init → state vazio para "c".
        assert_eq!(intr.state.final_value("c"), None);
    }

    #[test]
    fn func_eval_sequencia_aplica_em_ordem() {
        // P191B: state init=0 → +1 → *10 → final value 10 (não 20:
        // (0+1)*10).
        let f1 = Func::native("add_one", add_one_native);
        let f2 = Func::native("times_ten", times_ten_native);
        let mut intr = TagIntrospector::empty();
        intr.state.init("c".to_string(), Value::Int(0), loc(10));
        let tags = vec![
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
        with_engine!(&world, |engine, ctx| {
            apply_state_funcs(&tags, &mut intr, &mut engine, &mut ctx);
        });
        assert_eq!(intr.state.final_value("c"), Some(&Value::Int(10)));
    }
}
