//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/introspect/from_tags.md
//! @prompt-hash b161d9d0
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
//! `apply_state_funcs` permanece API explícita legada; o runtime automático
//! registra ações no walk e aplica callbacks somente quando a chave é lida.
//! Os demais pós-processadores preservam sua fase e responsabilidade próprias.

use crate::compiler::eval::call_dispatch::apply_func;
use crate::compiler::eval::EvalContext;
use crate::compiler::scopes::Scopes;
use crate::entities::args::Args;
use crate::entities::counter_update::CounterUpdate;
use crate::entities::element_payload::ElementPayload;
use crate::entities::engine::Engine;
use crate::entities::introspector::{Introspector, TagIntrospector};
use crate::entities::source_result::SourceResult;
use crate::entities::state_update::StateUpdate;
use crate::entities::tag::Tag;

mod counter_updates;
mod displays;
mod equations;
mod headings;
pub(super) use headings::apply_heading_supplements;
mod state_updates;

pub(crate) use counter_updates::{apply_bundle_counter_funcs, resolve_filtered_counter};
pub use counter_updates::{apply_counter_funcs, realize_page_counter_snapshot};
pub(super) use displays::apply_state_displays_checked;
pub use displays::{apply_counter_displays, apply_state_displays};
pub use equations::{
    apply_equation_numberings, apply_equation_supplements, realize_equation_elements,
};
pub use state_updates::apply_state_funcs;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::layout::FixedMetrics;
    use crate::entities::content::Content;
    use crate::entities::counter::CounterKey;
    use crate::entities::element_info::ElementInfo;
    use crate::entities::location::Location;
    use crate::entities::value::Value;

    fn loc(raw: u128) -> Location {
        Location::from_raw(raw)
    }

    fn ck(s: &str) -> CounterKey {
        CounterKey::Str(s.into())
    }

    // ── P191B (ADR-0071) — apply_state_funcs (slim Func post-pass) ──────

    use crate::entities::engine::Engine;
    use crate::entities::func::Func;
    use crate::entities::show::{RuleId, ShowRule};
    use crate::entities::sink::Sink;
    use crate::entities::style_chain::StyleChain;
    use crate::entities::world_types::{Library, Route};
    use std::sync::Arc;

    /// MockWorld minimal — paridade com o teste do contracts/world.rs.
    struct MockWorld {
        library: Library,
        book: crate::entities::font_book::FontBook,
        main_id: crate::entities::file_id::FileId,
    }

    impl crate::contracts::world::World for MockWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &crate::entities::font_book::FontBook {
            &self.book
        }
        fn main(&self) -> crate::entities::file_id::FileId {
            self.main_id
        }
        fn source(
            &self,
            _: crate::entities::file_id::FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::source::Source>
        {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn file(
            &self,
            _: crate::entities::file_id::FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::world_types::Bytes>
        {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<crate::entities::world_types::Font> {
            None
        }
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<crate::entities::world_types::Datetime> {
            None
        }
    }

    fn make_world() -> MockWorld {
        MockWorld {
            library: Library::new(),
            book: crate::entities::font_book::FontBook::new(),
            main_id: crate::entities::file_id::FileId::from_raw(
                std::num::NonZeroU16::new(1).unwrap(),
            ),
        }
    }

    /// Native `x => x + 1` para Funcs Int.
    fn add_one_native(
        _ctx: &mut crate::compiler::eval::EvalContext,
        args: &crate::entities::args::Args,
        _world: &dyn crate::contracts::world::World,
        _current_file: crate::entities::file_id::FileId,
    ) -> crate::entities::source_result::SourceResult<crate::entities::value::Value> {
        match args.items.first() {
            Some(crate::entities::value::Value::Int(n)) => {
                Ok(crate::entities::value::Value::Int(n + 1))
            }
            _ => Ok(crate::entities::value::Value::None),
        }
    }

    /// Native `x => x * 10` para Funcs Int.
    fn times_ten_native(
        _ctx: &mut crate::compiler::eval::EvalContext,
        args: &crate::entities::args::Args,
        _world: &dyn crate::contracts::world::World,
        _current_file: crate::entities::file_id::FileId,
    ) -> crate::entities::source_result::SourceResult<crate::entities::value::Value> {
        match args.items.first() {
            Some(crate::entities::value::Value::Int(n)) => {
                Ok(crate::entities::value::Value::Int(n * 10))
            }
            _ => Ok(crate::entities::value::Value::None),
        }
    }

    /// Helper que constrói Engine + EvalContext locais e chama a closure.
    /// Engine não é Send/static, por isso construído inline em cada call.
    macro_rules! with_engine {
        ($world:expr, |$engine:ident, $ctx:ident| $body:block) => {{
            use comemo::Track;
            let world: &dyn crate::contracts::world::World = $world;
            let mut $ctx = crate::compiler::eval::EvalContext::new();
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
    fn a1_state_readers_keep_their_own_init() {
        let world = make_world();
        let content = Content::state_update(
            "read",
            StateUpdate::Func(Func::native("add", add_one_native)),
        );
        with_engine!(&world, |engine, ctx| {
            ctx.introspector = crate::compiler::introspect::introspect_with_runtime(
                &content,
                &mut engine,
                &mut ctx,
            )
            .unwrap();
            ctx.in_context = true;
            let first = crate::entities::state::State {
                key: "read".into(),
                init: Box::new(Value::Int(1)),
            };
            let second = crate::entities::state::State {
                key: "read".into(),
                init: Box::new(Value::Int(10)),
            };
            assert_eq!(
                crate::compiler::stdlib::state::state_final(
                    &first,
                    &mut ctx,
                    &mut engine,
                    crate::entities::span::Span::detached()
                )
                .unwrap(),
                Value::Int(2)
            );
            assert_eq!(
                crate::compiler::stdlib::state::state_final(
                    &second,
                    &mut ctx,
                    &mut engine,
                    crate::entities::span::Span::detached()
                )
                .unwrap(),
                Value::Int(11)
            );
        });
    }

    #[test]
    fn a1_state_unused_callback_waits_for_a_read() {
        fn fail(
            _: &mut EvalContext,
            _: &Args,
            _: &dyn crate::contracts::world::World,
            _: crate::entities::file_id::FileId,
        ) -> SourceResult<Value> {
            Err(vec![crate::entities::source_result::SourceDiagnostic::error(
                crate::entities::span::Span::detached(),
                "UNUSED",
            )])
        }
        let world = make_world();
        let content = Content::sequence(vec![
            Content::state("read", Value::Int(0)),
            Content::state_update("read", StateUpdate::Func(Func::native("fail", fail))),
        ]);
        with_engine!(&world, |engine, ctx| {
            assert!(crate::compiler::introspect::introspect_with_runtime(
                &content,
                &mut engine,
                &mut ctx
            )
            .is_ok());
        });
    }

    #[test]
    fn a1_state_update_context_is_absent_and_restored_after_success_and_error() {
        fn check(
            ctx: &mut EvalContext,
            args: &Args,
            _: &dyn crate::contracts::world::World,
            _: crate::entities::file_id::FileId,
        ) -> SourceResult<Value> {
            assert!(!ctx.in_context);
            assert!(ctx.current_location.is_none());
            if args.items[0] == Value::Int(1) {
                Ok(Value::Int(2))
            } else {
                Err(vec![crate::entities::source_result::SourceDiagnostic::error(
                    crate::entities::span::Span::detached(),
                    "FAIL",
                )])
            }
        }
        let world = make_world();
        let content = Content::state_update(
            "read",
            StateUpdate::Func(Func::native("check", check)),
        );
        with_engine!(&world, |engine, ctx| {
            ctx.introspector = crate::compiler::introspect::introspect_with_runtime(
                &content,
                &mut engine,
                &mut ctx,
            )
            .unwrap();
            ctx.in_context = true;
            ctx.current_location = Some(loc(999));
            for (init, succeeds) in [(1, true), (3, false)] {
                let state = crate::entities::state::State {
                    key: "read".into(),
                    init: Box::new(Value::Int(init)),
                };
                let result = crate::compiler::stdlib::state::state_final(
                    &state,
                    &mut ctx,
                    &mut engine,
                    crate::entities::span::Span::detached(),
                );
                assert_eq!(result.is_ok(), succeeds);
                assert!(ctx.in_context);
                assert_eq!(ctx.current_location, Some(loc(999)));
            }
        });
    }

    #[test]
    fn a1_heading_pending_styles_preserve_identity_and_finalize_once() {
        fn count(
            ctx: &mut EvalContext,
            _: &Args,
            _: &dyn crate::contracts::world::World,
            _: crate::entities::file_id::FileId,
        ) -> SourceResult<Value> {
            assert!(!ctx.in_context);
            assert!(ctx.current_location.is_none());
            ctx.next_context_id += 1;
            Ok(Value::Str(ctx.next_context_id.to_string().into()))
        }
        let world = make_world();
        let content = Content::styled(
            Content::heading(1, Content::text("A")),
            crate::entities::style::Styles::new().push_custom(
                "heading.supplement",
                Value::Func(Func::native("count", count)),
            ),
        );
        with_engine!(&world, |engine, ctx| {
            let mut intr =
                crate::compiler::introspect::introspect_with_introspector(&content);
            let location = intr
                .query_by_kind(crate::entities::element_kind::ElementKind::Heading)[0];
            let pending = intr.elements[&location].clone();
            ctx.in_context = true;
            ctx.current_location = Some(location);
            assert!(matches!(pending.content(), Content::Heading(_)));
            crate::compiler::introspect::realize_heading_supplements(
                &mut intr,
                &mut engine,
                &mut ctx,
            )
            .unwrap();
            let realized = intr.elements[&location].clone();
            crate::compiler::introspect::realize_heading_supplements(
                &mut intr,
                &mut engine,
                &mut ctx,
            )
            .unwrap();
            assert_eq!(ctx.next_context_id, 1);
            assert!(ctx.in_context);
            assert_eq!(ctx.current_location, Some(location));
            assert_eq!(
                realized.fields().unwrap().get("supplement"),
                Some(&Value::Content(Content::text("1")))
            );
            assert_eq!(intr.elements[&location], realized);
            assert!(pending.heading_styles().is_some());
            assert!(matches!(
                pending.fields().unwrap().get("supplement"),
                Some(Value::Func(_))
            ));
            assert!(realized.heading_styles().is_none());
        });
    }

    #[test]
    fn a1_heading_supplement_executes_callback_and_publishes_content() {
        fn callback(
            _: &mut EvalContext,
            args: &Args,
            _: &dyn crate::contracts::world::World,
            _: crate::entities::file_id::FileId,
        ) -> SourceResult<Value> {
            let Value::LocatedContent(heading, _) = &args.items[0] else {
                panic!("callback must receive its heading occurrence");
            };
            assert_eq!(heading.fields().unwrap().get("depth"), Some(&Value::Int(2)));
            Ok(Value::Str("Custom".into()))
        }
        let world = make_world();
        let content = Content::styled(
            Content::heading(2, Content::text("Body")),
            crate::entities::style::Styles::from_delta(
                StyleChain::empty()
                    .push_custom(
                        "heading.supplement",
                        Value::Func(Func::native("callback", callback)),
                    )
                    .collapse(),
            ),
        );
        with_engine!(&world, |engine, ctx| {
            let intr = crate::compiler::introspect::introspect_with_runtime(
                &content,
                &mut engine,
                &mut ctx,
            )
            .unwrap();
            let entry = intr
                .elements
                .values()
                .find(|entry| {
                    entry.fields().is_some_and(|fields| fields.contains_key("depth"))
                })
                .unwrap();
            assert_eq!(
                entry.fields().unwrap().get("supplement"),
                Some(&Value::Content(Content::text("Custom")))
            );
        });
    }

    #[test]
    fn a1_heading_supplement_callback_error_is_not_ignored() {
        fn callback(
            _: &mut EvalContext,
            _: &Args,
            _: &dyn crate::contracts::world::World,
            _: crate::entities::file_id::FileId,
        ) -> SourceResult<Value> {
            Err(vec![crate::entities::source_result::SourceDiagnostic::error(
                crate::entities::span::Span::detached(),
                "A1_HEADING",
            )])
        }
        let world = make_world();
        let content = Content::styled(
            Content::heading(1, Content::text("Body")),
            crate::entities::style::Styles::from_delta(
                StyleChain::empty()
                    .push_custom(
                        "heading.supplement",
                        Value::Func(Func::native("callback", callback)),
                    )
                    .collapse(),
            ),
        );
        with_engine!(&world, |engine, ctx| {
            let error = crate::compiler::introspect::introspect_with_runtime(
                &content,
                &mut engine,
                &mut ctx,
            )
            .unwrap_err();
            assert_eq!(error[0].message, "A1_HEADING");
        });
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
                Box::new(ElementInfo::new(ElementPayload::StateUpdate {
                    key: "c".to_string(),
                    update: StateUpdate::Func(f),
                })),
            ),
            Tag::End(loc(20), 0),
        ];
        let world = make_world();
        with_engine!(&world, |engine, ctx| {
            apply_state_funcs(&tags, &mut intr, &mut engine, &mut ctx)
                .expect("apply_state_funcs deve suceder");
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
                Box::new(ElementInfo::new(ElementPayload::StateUpdate {
                    key: "c".to_string(),
                    update: StateUpdate::Func(f),
                })),
            ),
            Tag::End(loc(20), 0),
        ];
        let world = make_world();
        with_engine!(&world, |engine, ctx| {
            apply_state_funcs(&tags, &mut intr, &mut engine, &mut ctx)
                .expect("apply_state_funcs deve suceder");
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
                Box::new(ElementInfo::new(ElementPayload::StateUpdate {
                    key: "c".to_string(),
                    update: StateUpdate::Func(f1),
                })),
            ),
            Tag::End(loc(20), 0),
            Tag::Start(
                loc(30),
                Box::new(ElementInfo::new(ElementPayload::StateUpdate {
                    key: "c".to_string(),
                    update: StateUpdate::Func(f2),
                })),
            ),
            Tag::End(loc(30), 0),
        ];
        let world = make_world();
        with_engine!(&world, |engine, ctx| {
            apply_state_funcs(&tags, &mut intr, &mut engine, &mut ctx)
                .expect("apply_state_funcs deve suceder");
        });
        assert_eq!(intr.state.final_value("c"), Some(&Value::Int(10)));
    }

    #[test]
    fn func_eval_callback_erro_propaga_diagnostics() {
        // P642: callback de state.update que retorna Err deve propagar
        // diagnostics em vez de ser descartado silenciosamente.
        fn err_callback(
            _ctx: &mut crate::compiler::eval::EvalContext,
            _args: &crate::entities::args::Args,
            _world: &dyn crate::contracts::world::World,
            _current_file: crate::entities::file_id::FileId,
        ) -> crate::entities::source_result::SourceResult<crate::entities::value::Value>
        {
            Err(vec![crate::entities::source_result::SourceDiagnostic::error(
                crate::entities::span::Span::detached(),
                "state update callback error",
            )])
        }
        let f = Func::native("err_callback", err_callback);
        let mut intr = TagIntrospector::empty();
        intr.state.init("c".to_string(), Value::Int(0), loc(10));
        let tags = vec![
            Tag::Start(
                loc(20),
                Box::new(ElementInfo::new(ElementPayload::StateUpdate {
                    key: "c".to_string(),
                    update: StateUpdate::Func(f),
                })),
            ),
            Tag::End(loc(20), 0),
        ];
        let world = make_world();
        let result: SourceResult<()> = with_engine!(&world, |engine, ctx| {
            apply_state_funcs(&tags, &mut intr, &mut engine, &mut ctx)
        });
        match result {
            Err(diagnostics) => {
                assert_eq!(diagnostics.len(), 1);
                assert!(diagnostics[0].message.contains("state update callback error"));
            }
            Ok(_) => panic!("esperado Err com diagnostics, recebido Ok"),
        }
        // State não deve ter sido actualizado.
        assert_eq!(intr.state.final_value("c"), Some(&Value::Int(0)));
    }

    // ── Passo 240 (M9d/M7+1; ADR-0081 PROPOSTO P239 Opção γ) —
    //     apply_state_displays paralelo apply_state_funcs ──

    #[test]
    fn p240_apply_state_displays_sem_callback_renderiza_value() {
        // Sem callback: state value renderiza directo (Value::Str via
        // Content::text; outros tipos Content::Empty).
        let mut intr = TagIntrospector::empty();
        intr.state.init("k".to_string(), Value::Str("hello".into()), loc(10));
        let tags = vec![
            Tag::Start(
                loc(20),
                Box::new(ElementInfo::new(ElementPayload::StateDisplay {
                    key: "k".to_string(),
                    callback: None,
                })),
            ),
            Tag::End(loc(20), 0),
        ];
        let world = make_world();
        with_engine!(&world, |engine, ctx| {
            apply_state_displays(&tags, &mut intr, &mut engine, &mut ctx);
        });
        let pre = intr
            .state_displays
            .get(&("k".to_string(), loc(20)))
            .expect("state_displays populated");
        assert_eq!(pre.plain_text(), "hello");
    }

    #[test]
    fn p240_apply_state_displays_com_callback_aplica_func() {
        // Callback add_one aplicado a state value 41 → Content::text("42")
        // (via fallback Value::Int → Content::Empty se não Content/Str;
        // mas add_one retorna Int, então `_ => Content::Empty`).
        // Para teste material, callback retorna Str.
        fn str_callback(
            _ctx: &mut crate::compiler::eval::EvalContext,
            args: &crate::entities::args::Args,
            _world: &dyn crate::contracts::world::World,
            _current_file: crate::entities::file_id::FileId,
        ) -> crate::entities::source_result::SourceResult<crate::entities::value::Value>
        {
            match args.items.first() {
                Some(Value::Int(n)) => Ok(Value::Str(format!("v={}", n).into())),
                _ => Ok(Value::Str("v=?".into())),
            }
        }
        let f = Func::native("str_callback", str_callback);
        let mut intr = TagIntrospector::empty();
        intr.state.init("k".to_string(), Value::Int(42), loc(10));
        let tags = vec![
            Tag::Start(
                loc(20),
                Box::new(ElementInfo::new(ElementPayload::StateDisplay {
                    key: "k".to_string(),
                    callback: Some(f),
                })),
            ),
            Tag::End(loc(20), 0),
        ];
        let world = make_world();
        with_engine!(&world, |engine, ctx| {
            apply_state_displays(&tags, &mut intr, &mut engine, &mut ctx);
        });
        let pre = intr
            .state_displays
            .get(&("k".to_string(), loc(20)))
            .expect("state_displays populated");
        assert_eq!(pre.plain_text(), "v=42");
    }

    #[test]
    fn p240_apply_state_displays_callback_erro_retorna_content_empty() {
        // Func que sempre retorna Err → defensive ignore (Content::Empty).
        fn err_callback(
            _ctx: &mut crate::compiler::eval::EvalContext,
            _args: &crate::entities::args::Args,
            _world: &dyn crate::contracts::world::World,
            _current_file: crate::entities::file_id::FileId,
        ) -> crate::entities::source_result::SourceResult<crate::entities::value::Value>
        {
            Err(vec![crate::entities::source_result::SourceDiagnostic::error(
                crate::entities::span::Span::detached(),
                "test error",
            )])
        }
        let f = Func::native("err_callback", err_callback);
        let mut intr = TagIntrospector::empty();
        intr.state.init("k".to_string(), Value::Int(1), loc(10));
        let tags = vec![
            Tag::Start(
                loc(20),
                Box::new(ElementInfo::new(ElementPayload::StateDisplay {
                    key: "k".to_string(),
                    callback: Some(f),
                })),
            ),
            Tag::End(loc(20), 0),
        ];
        let world = make_world();
        with_engine!(&world, |engine, ctx| {
            apply_state_displays(&tags, &mut intr, &mut engine, &mut ctx);
        });
        let pre = intr
            .state_displays
            .get(&("k".to_string(), loc(20)))
            .expect("state_displays populated mesmo com Err defensive ignore");
        // Content::Empty.plain_text() == ""
        assert_eq!(pre.plain_text(), "");
    }

    #[test]
    fn p240_apply_state_displays_locations_diferentes_valores_diferentes() {
        // 2 updates Set + 2 StateDisplay tags em locations diferentes.
        // Cada display deve ver o valor cumulativo correspondente ao loc.
        let mut intr = TagIntrospector::empty();
        intr.state.init("k".to_string(), Value::Str("init".into()), loc(10));
        intr.state.update("k".to_string(), Value::Str("mid".into()), loc(15));
        intr.state.update("k".to_string(), Value::Str("end".into()), loc(25));
        let tags = vec![
            Tag::Start(
                loc(12),
                Box::new(ElementInfo::new(ElementPayload::StateDisplay {
                    key: "k".to_string(),
                    callback: None,
                })),
            ),
            Tag::End(loc(12), 0),
            Tag::Start(
                loc(20),
                Box::new(ElementInfo::new(ElementPayload::StateDisplay {
                    key: "k".to_string(),
                    callback: None,
                })),
            ),
            Tag::End(loc(20), 0),
            Tag::Start(
                loc(30),
                Box::new(ElementInfo::new(ElementPayload::StateDisplay {
                    key: "k".to_string(),
                    callback: None,
                })),
            ),
            Tag::End(loc(30), 0),
        ];
        let world = make_world();
        with_engine!(&world, |engine, ctx| {
            apply_state_displays(&tags, &mut intr, &mut engine, &mut ctx);
        });
        // loc 12 → init ainda (update mid em loc 15 não-aplicável).
        assert_eq!(
            intr.state_displays
                .get(&("k".to_string(), loc(12)))
                .unwrap()
                .plain_text(),
            "init"
        );
        // loc 20 → mid (update em loc 15 aplicado).
        assert_eq!(
            intr.state_displays
                .get(&("k".to_string(), loc(20)))
                .unwrap()
                .plain_text(),
            "mid"
        );
        // loc 30 → end (todos updates aplicados).
        assert_eq!(
            intr.state_displays
                .get(&("k".to_string(), loc(30)))
                .unwrap()
                .plain_text(),
            "end"
        );
    }

    #[test]
    fn p240_apply_state_displays_state_inexistente_value_none() {
        // State key não inicializada → value_at retorna None →
        // Value::None.unwrap_or fallback → Content::Empty (None não é
        // Content nem Str).
        let mut intr = TagIntrospector::empty();
        let tags = vec![
            Tag::Start(
                loc(20),
                Box::new(ElementInfo::new(ElementPayload::StateDisplay {
                    key: "inexistente".to_string(),
                    callback: None,
                })),
            ),
            Tag::End(loc(20), 0),
        ];
        let world = make_world();
        with_engine!(&world, |engine, ctx| {
            apply_state_displays(&tags, &mut intr, &mut engine, &mut ctx);
        });
        let pre = intr
            .state_displays
            .get(&("inexistente".to_string(), loc(20)))
            .expect("state_displays populated mesmo com key ausente");
        assert_eq!(pre.plain_text(), "");
    }

    // ── Passo 241 (M9d/M7+2; ADR-0081 IMPLEMENTADO parcial paralelo absoluto
    //     P240 M7+1) — apply_counter_displays paralelo apply_state_displays ──

    #[test]
    fn p241_apply_counter_displays_sem_callback_renderiza_formato_default() {
        // Sem callback + counter populado: formato default "1.1"
        // via join "." paridade formatted_counter_at P177.
        // apply_hierarchical_at semantic: (key, level, loc); cada nova
        // depth-step appende [1] hierarchicamente.
        let mut intr = TagIntrospector::empty();
        intr.counters.apply_hierarchical_at(ck("heading"), 1, loc(10)); // [1]
        intr.counters.apply_hierarchical_at(ck("heading"), 2, loc(15)); // [1, 1]
        let tags = vec![
            Tag::Start(
                loc(20),
                Box::new(ElementInfo::new(ElementPayload::CounterDisplay {
                    key: ck("heading"),
                    callback: None,
                })),
            ),
            Tag::End(loc(20), 0),
        ];
        let world = make_world();
        with_engine!(&world, |engine, ctx| {
            apply_counter_displays(&tags, &mut intr, &mut engine, &mut ctx);
        });
        let pre = intr
            .counter_displays
            .get(&(ck("heading"), loc(20)))
            .expect("counter_displays populated");
        // Snapshot final em loc 15 é [1, 1] → "1.1" via join ".".
        assert_eq!(pre.plain_text(), "1.1");
    }

    #[test]
    fn p241_apply_counter_displays_com_callback_aplica_func() {
        // Callback recebe Value::Array(counter_state) e retorna Str
        // formatado custom. apply_hierarchical_at snapshots resultam
        // em [1, 1] em loc 20.
        fn str_callback(
            _ctx: &mut crate::compiler::eval::EvalContext,
            args: &crate::entities::args::Args,
            _world: &dyn crate::contracts::world::World,
            _current_file: crate::entities::file_id::FileId,
        ) -> crate::entities::source_result::SourceResult<crate::entities::value::Value>
        {
            match args.items.first() {
                Some(Value::Array(items)) => {
                    let s = items
                        .iter()
                        .filter_map(|v| {
                            if let Value::Int(n) = v {
                                Some(n.to_string())
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                        .join("-");
                    Ok(Value::Str(format!("[{}]", s).into()))
                }
                _ => Ok(Value::Str("[]".into())),
            }
        }
        let f = Func::native("str_callback", str_callback);
        let mut intr = TagIntrospector::empty();
        intr.counters.apply_hierarchical_at(ck("heading"), 1, loc(10)); // [1]
        intr.counters.apply_hierarchical_at(ck("heading"), 2, loc(15)); // [1, 1]
        let tags = vec![
            Tag::Start(
                loc(20),
                Box::new(ElementInfo::new(ElementPayload::CounterDisplay {
                    key: ck("heading"),
                    callback: Some(f),
                })),
            ),
            Tag::End(loc(20), 0),
        ];
        let world = make_world();
        with_engine!(&world, |engine, ctx| {
            apply_counter_displays(&tags, &mut intr, &mut engine, &mut ctx);
        });
        let pre = intr
            .counter_displays
            .get(&(ck("heading"), loc(20)))
            .expect("counter_displays populated");
        // Callback recebe [1, 1] → formato "[1-1]".
        assert_eq!(pre.plain_text(), "[1-1]");
    }

    #[test]
    fn p241_apply_counter_displays_callback_erro_retorna_content_empty() {
        // Func que sempre retorna Err → defensive ignore (Content::Empty).
        fn err_callback(
            _ctx: &mut crate::compiler::eval::EvalContext,
            _args: &crate::entities::args::Args,
            _world: &dyn crate::contracts::world::World,
            _current_file: crate::entities::file_id::FileId,
        ) -> crate::entities::source_result::SourceResult<crate::entities::value::Value>
        {
            Err(vec![crate::entities::source_result::SourceDiagnostic::error(
                crate::entities::span::Span::detached(),
                "test error",
            )])
        }
        let f = Func::native("err_callback", err_callback);
        let mut intr = TagIntrospector::empty();
        intr.counters.apply_hierarchical_at(ck("heading"), 1, loc(10));
        let tags = vec![
            Tag::Start(
                loc(20),
                Box::new(ElementInfo::new(ElementPayload::CounterDisplay {
                    key: ck("heading"),
                    callback: Some(f),
                })),
            ),
            Tag::End(loc(20), 0),
        ];
        let world = make_world();
        with_engine!(&world, |engine, ctx| {
            apply_counter_displays(&tags, &mut intr, &mut engine, &mut ctx);
        });
        let pre = intr
            .counter_displays
            .get(&(ck("heading"), loc(20)))
            .expect("counter_displays populated mesmo com Err defensive ignore");
        assert_eq!(pre.plain_text(), "");
    }

    #[test]
    fn p241_apply_counter_displays_locations_diferentes_valores_diferentes() {
        // 2 apply_hierarchical_at em locations diferentes; CounterDisplay
        // em locations intermédias deve ver snapshots cumulativos.
        let mut intr = TagIntrospector::empty();
        intr.counters.apply_hierarchical_at(ck("heading"), 1, loc(10));
        intr.counters.apply_hierarchical_at(ck("heading"), 1, loc(20));
        intr.counters.apply_hierarchical_at(ck("heading"), 1, loc(30));
        let tags = vec![
            // loc 15 → ver snapshot loc 10 = [1].
            Tag::Start(
                loc(15),
                Box::new(ElementInfo::new(ElementPayload::CounterDisplay {
                    key: ck("heading"),
                    callback: None,
                })),
            ),
            Tag::End(loc(15), 0),
            // loc 25 → ver snapshot loc 20 = [2] (2º depth-1 step).
            Tag::Start(
                loc(25),
                Box::new(ElementInfo::new(ElementPayload::CounterDisplay {
                    key: ck("heading"),
                    callback: None,
                })),
            ),
            Tag::End(loc(25), 0),
            // loc 35 → ver snapshot loc 30 = [3].
            Tag::Start(
                loc(35),
                Box::new(ElementInfo::new(ElementPayload::CounterDisplay {
                    key: ck("heading"),
                    callback: None,
                })),
            ),
            Tag::End(loc(35), 0),
        ];
        let world = make_world();
        with_engine!(&world, |engine, ctx| {
            apply_counter_displays(&tags, &mut intr, &mut engine, &mut ctx);
        });
        assert_eq!(
            intr.counter_displays
                .get(&(ck("heading"), loc(15)))
                .unwrap()
                .plain_text(),
            "1"
        );
        assert_eq!(
            intr.counter_displays
                .get(&(ck("heading"), loc(25)))
                .unwrap()
                .plain_text(),
            "2"
        );
        assert_eq!(
            intr.counter_displays
                .get(&(ck("heading"), loc(35)))
                .unwrap()
                .plain_text(),
            "3"
        );
    }

    #[test]
    fn p241_apply_counter_displays_counter_inexistente_array_vazio() {
        // Counter key não inicializada → value_at None → callback recebe
        // Value::Array(vec![]) (vector vazio); sem callback → Content::Empty.
        fn array_len_callback(
            _ctx: &mut crate::compiler::eval::EvalContext,
            args: &crate::entities::args::Args,
            _world: &dyn crate::contracts::world::World,
            _current_file: crate::entities::file_id::FileId,
        ) -> crate::entities::source_result::SourceResult<crate::entities::value::Value>
        {
            match args.items.first() {
                Some(Value::Array(items)) => {
                    Ok(Value::Str(format!("len={}", items.len()).into()))
                }
                _ => Ok(Value::Str("?".into())),
            }
        }
        let f = Func::native("array_len_callback", array_len_callback);
        let mut intr = TagIntrospector::empty();
        let tags = vec![
            Tag::Start(
                loc(20),
                Box::new(ElementInfo::new(ElementPayload::CounterDisplay {
                    key: ck("inexistente"),
                    callback: Some(f),
                })),
            ),
            Tag::End(loc(20), 0),
        ];
        let world = make_world();
        with_engine!(&world, |engine, ctx| {
            apply_counter_displays(&tags, &mut intr, &mut engine, &mut ctx);
        });
        let pre = intr
            .counter_displays
            .get(&(ck("inexistente"), loc(20)))
            .expect("counter_displays populated mesmo com key ausente");
        // Callback recebeu Array vazio → "len=0".
        assert_eq!(pre.plain_text(), "len=0");
    }
}
