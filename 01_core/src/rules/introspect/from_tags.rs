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

/// **P240 (M9d/M7+1)** — slim post-pass para `Content::StateDisplay`
/// pre-render via Opção γ (per ADR-0081 PROPOSTO + P239 audit §3.2).
///
/// Paralelo absoluto a `apply_state_funcs`:
/// - Walk fn emite `Tag::Start(loc, ElementPayload::StateDisplay {
///   key, callback })` via `extract_payload`.
/// - Esta função processa Tags pós-walk + pós-`apply_state_funcs` (que
///   já materializou state values cumulativos), com Engine+ctx
///   disponíveis.
/// - Para cada `(key, loc)`: lookup `intr.state.value_at(key, loc)` →
///   se `callback.is_some()`, chama `apply_func(callback, [value],
///   ctx, engine)` → resultado convertido para `Content` →
///   armazenado em `intr.state_displays[(key, loc)]`.
/// - Layout arm `Content::StateDisplay` consome via
///   `Introspector::state_display_value` — Layouter permanece puro
///   (sem Engine+ctx em signature; paridade arquitectural Opção γ).
///
/// **Caller**: `fixpoint::run_fixpoint` (após `apply_state_funcs`).
/// **Conversão Value→Content** inline: `Value::Content(c) => c;
/// Value::Str(s) => Content::text(s); _ => Content::Empty` (paridade
/// `figure_image.rs:59-72` + `eval/rules.rs:134-146`).
///
/// **Err em apply_func**: defensive ignore (paridade P191B
/// `apply_state_funcs`). Refino futuro pode propagar via Sink.
pub fn apply_state_displays(
    tags:   &[Tag],
    intr:   &mut TagIntrospector,
    engine: &mut Engine<'_>,
    ctx:    &mut EvalContext,
) {
    use crate::entities::content::Content;
    use crate::entities::value::Value;
    for tag in tags {
        if let Tag::Start(loc, info) = tag {
            if let ElementPayload::StateDisplay { key, callback } = &info.payload {
                let value = intr.state.value_at(key, *loc).cloned()
                    .unwrap_or(Value::None);
                let pre_rendered = match callback {
                    Some(func) => {
                        let args = Args::positional(vec![value]);
                        match apply_func(func.clone(), args, ctx, engine) {
                            Ok(Value::Content(c))  => c,
                            Ok(Value::Str(s))      => Content::text(s.as_str()),
                            Ok(_)                  => Content::Empty,
                            Err(_)                 => Content::Empty,
                        }
                    }
                    None => match value {
                        Value::Content(c) => c,
                        Value::Str(s)     => Content::text(s.as_str()),
                        _                 => Content::Empty,
                    },
                };
                intr.state_displays
                    .insert((key.clone(), *loc), pre_rendered);
            }
        }
    }
}

/// **P241 (M9d/M7+2)** — slim post-pass para
/// `Content::CounterDisplayCallback` paralelo absoluto
/// `apply_state_displays` P240.
///
/// Walk emite `Tag::Start(loc, ElementPayload::CounterDisplay { key,
/// callback })` via `extract_payload`. Esta função processa Tags
/// pós-walk + pós-`apply_state_funcs` + pós-`apply_state_displays`,
/// com Engine+ctx disponíveis.
///
/// Algoritmo:
/// - Para cada `(key, loc)`: lookup `intr.counters.value_at(key, loc)`
///   `Option<&[usize]>` → converter para `Value::Array(Vec<Value::Int>)`
///   (counter inexistente → `Value::Array(vec![])`).
/// - Se `callback.is_some()`: `apply_func(callback, [array], ctx,
///   engine)`; resultado convertido para Content (paridade
///   `apply_state_displays`: Content passa-through; Str via
///   Content::text; outros tipos / Err → Content::Empty).
/// - Sem callback: formato default "1.2.3" via join "." se counter
///   existe; `Content::Empty` se inexistente.
/// - Armazenar em `intr.counter_displays[(key, loc)]`.
///
/// **Forma do Value passado ao callback** (Decisão 4 P241): paridade
/// vanilla `CounterState = SmallVec<[u64; 3]>` representado como
/// `Value::Array(Vec<Value::Int>)`. Permite callbacks ricos como
/// `counter("heading").display(nums => nums.map(str).join("."))`.
///
/// **Caller**: `fixpoint::run_fixpoint` após `apply_state_displays`.
pub fn apply_counter_displays(
    tags:   &[Tag],
    intr:   &mut TagIntrospector,
    engine: &mut Engine<'_>,
    ctx:    &mut EvalContext,
) {
    use crate::entities::content::Content;
    use crate::entities::value::Value;
    for tag in tags {
        if let Tag::Start(loc, info) = tag {
            if let ElementPayload::CounterDisplay { key, callback } = &info.payload {
                let counter_slice_opt = intr.counters.value_at(key, *loc);
                let counter_value: Value = counter_slice_opt
                    .map(|slice| Value::Array(
                        slice.iter().map(|&n| Value::Int(n as i64)).collect()
                    ))
                    .unwrap_or(Value::Array(vec![]));
                let pre_rendered = match callback {
                    Some(func) => {
                        let args = Args::positional(vec![counter_value]);
                        match apply_func(func.clone(), args, ctx, engine) {
                            Ok(Value::Content(c))  => c,
                            Ok(Value::Str(s))      => Content::text(s.as_str()),
                            Ok(_)                  => Content::Empty,
                            Err(_)                 => Content::Empty,
                        }
                    }
                    None => match counter_slice_opt {
                        Some(slice) => {
                            let s = slice.iter()
                                .map(|n| n.to_string())
                                .collect::<Vec<_>>()
                                .join(".");
                            Content::text(&s)
                        }
                        None => Content::Empty,
                    },
                };
                intr.counter_displays
                    .insert((key.clone(), *loc), pre_rendered);
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
                ElementInfo::new(ElementPayload::StateDisplay {
                    key:      "k".to_string(),
                    callback: None,
                }),
            ),
            Tag::End(loc(20), 0),
        ];
        let world = make_world();
        with_engine!(&world, |engine, ctx| {
            apply_state_displays(&tags, &mut intr, &mut engine, &mut ctx);
        });
        let pre = intr.state_displays
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
            _ctx: &mut crate::rules::eval::EvalContext,
            args: &crate::entities::args::Args,
            _world: &dyn crate::contracts::world::World,
            _current_file: crate::entities::file_id::FileId,
            _figure_numbering: Option<&str>,
        ) -> crate::entities::source_result::SourceResult<crate::entities::value::Value> {
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
                ElementInfo::new(ElementPayload::StateDisplay {
                    key:      "k".to_string(),
                    callback: Some(f),
                }),
            ),
            Tag::End(loc(20), 0),
        ];
        let world = make_world();
        with_engine!(&world, |engine, ctx| {
            apply_state_displays(&tags, &mut intr, &mut engine, &mut ctx);
        });
        let pre = intr.state_displays
            .get(&("k".to_string(), loc(20)))
            .expect("state_displays populated");
        assert_eq!(pre.plain_text(), "v=42");
    }

    #[test]
    fn p240_apply_state_displays_callback_erro_retorna_content_empty() {
        // Func que sempre retorna Err → defensive ignore (Content::Empty).
        fn err_callback(
            _ctx: &mut crate::rules::eval::EvalContext,
            _args: &crate::entities::args::Args,
            _world: &dyn crate::contracts::world::World,
            _current_file: crate::entities::file_id::FileId,
            _figure_numbering: Option<&str>,
        ) -> crate::entities::source_result::SourceResult<crate::entities::value::Value> {
            Err(vec![
                crate::entities::source_result::SourceDiagnostic::error(
                    crate::entities::span::Span::detached(),
                    "test error",
                ),
            ])
        }
        let f = Func::native("err_callback", err_callback);
        let mut intr = TagIntrospector::empty();
        intr.state.init("k".to_string(), Value::Int(1), loc(10));
        let tags = vec![
            Tag::Start(
                loc(20),
                ElementInfo::new(ElementPayload::StateDisplay {
                    key:      "k".to_string(),
                    callback: Some(f),
                }),
            ),
            Tag::End(loc(20), 0),
        ];
        let world = make_world();
        with_engine!(&world, |engine, ctx| {
            apply_state_displays(&tags, &mut intr, &mut engine, &mut ctx);
        });
        let pre = intr.state_displays
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
                ElementInfo::new(ElementPayload::StateDisplay {
                    key:      "k".to_string(),
                    callback: None,
                }),
            ),
            Tag::End(loc(12), 0),
            Tag::Start(
                loc(20),
                ElementInfo::new(ElementPayload::StateDisplay {
                    key:      "k".to_string(),
                    callback: None,
                }),
            ),
            Tag::End(loc(20), 0),
            Tag::Start(
                loc(30),
                ElementInfo::new(ElementPayload::StateDisplay {
                    key:      "k".to_string(),
                    callback: None,
                }),
            ),
            Tag::End(loc(30), 0),
        ];
        let world = make_world();
        with_engine!(&world, |engine, ctx| {
            apply_state_displays(&tags, &mut intr, &mut engine, &mut ctx);
        });
        // loc 12 → init ainda (update mid em loc 15 não-aplicável).
        assert_eq!(
            intr.state_displays.get(&("k".to_string(), loc(12)))
                .unwrap().plain_text(),
            "init"
        );
        // loc 20 → mid (update em loc 15 aplicado).
        assert_eq!(
            intr.state_displays.get(&("k".to_string(), loc(20)))
                .unwrap().plain_text(),
            "mid"
        );
        // loc 30 → end (todos updates aplicados).
        assert_eq!(
            intr.state_displays.get(&("k".to_string(), loc(30)))
                .unwrap().plain_text(),
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
                ElementInfo::new(ElementPayload::StateDisplay {
                    key:      "inexistente".to_string(),
                    callback: None,
                }),
            ),
            Tag::End(loc(20), 0),
        ];
        let world = make_world();
        with_engine!(&world, |engine, ctx| {
            apply_state_displays(&tags, &mut intr, &mut engine, &mut ctx);
        });
        let pre = intr.state_displays
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
        intr.counters.apply_hierarchical_at("heading".to_string(), 1, loc(10)); // [1]
        intr.counters.apply_hierarchical_at("heading".to_string(), 2, loc(15)); // [1, 1]
        let tags = vec![
            Tag::Start(
                loc(20),
                ElementInfo::new(ElementPayload::CounterDisplay {
                    key:      "heading".to_string(),
                    callback: None,
                }),
            ),
            Tag::End(loc(20), 0),
        ];
        let world = make_world();
        with_engine!(&world, |engine, ctx| {
            apply_counter_displays(&tags, &mut intr, &mut engine, &mut ctx);
        });
        let pre = intr.counter_displays
            .get(&("heading".to_string(), loc(20)))
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
            _ctx: &mut crate::rules::eval::EvalContext,
            args: &crate::entities::args::Args,
            _world: &dyn crate::contracts::world::World,
            _current_file: crate::entities::file_id::FileId,
            _figure_numbering: Option<&str>,
        ) -> crate::entities::source_result::SourceResult<crate::entities::value::Value> {
            match args.items.first() {
                Some(Value::Array(items)) => {
                    let s = items.iter()
                        .filter_map(|v| if let Value::Int(n) = v { Some(n.to_string()) } else { None })
                        .collect::<Vec<_>>()
                        .join("-");
                    Ok(Value::Str(format!("[{}]", s).into()))
                }
                _ => Ok(Value::Str("[]".into())),
            }
        }
        let f = Func::native("str_callback", str_callback);
        let mut intr = TagIntrospector::empty();
        intr.counters.apply_hierarchical_at("heading".to_string(), 1, loc(10)); // [1]
        intr.counters.apply_hierarchical_at("heading".to_string(), 2, loc(15)); // [1, 1]
        let tags = vec![
            Tag::Start(
                loc(20),
                ElementInfo::new(ElementPayload::CounterDisplay {
                    key:      "heading".to_string(),
                    callback: Some(f),
                }),
            ),
            Tag::End(loc(20), 0),
        ];
        let world = make_world();
        with_engine!(&world, |engine, ctx| {
            apply_counter_displays(&tags, &mut intr, &mut engine, &mut ctx);
        });
        let pre = intr.counter_displays
            .get(&("heading".to_string(), loc(20)))
            .expect("counter_displays populated");
        // Callback recebe [1, 1] → formato "[1-1]".
        assert_eq!(pre.plain_text(), "[1-1]");
    }

    #[test]
    fn p241_apply_counter_displays_callback_erro_retorna_content_empty() {
        // Func que sempre retorna Err → defensive ignore (Content::Empty).
        fn err_callback(
            _ctx: &mut crate::rules::eval::EvalContext,
            _args: &crate::entities::args::Args,
            _world: &dyn crate::contracts::world::World,
            _current_file: crate::entities::file_id::FileId,
            _figure_numbering: Option<&str>,
        ) -> crate::entities::source_result::SourceResult<crate::entities::value::Value> {
            Err(vec![
                crate::entities::source_result::SourceDiagnostic::error(
                    crate::entities::span::Span::detached(),
                    "test error",
                ),
            ])
        }
        let f = Func::native("err_callback", err_callback);
        let mut intr = TagIntrospector::empty();
        intr.counters.apply_hierarchical_at("heading".to_string(), 1, loc(10));
        let tags = vec![
            Tag::Start(
                loc(20),
                ElementInfo::new(ElementPayload::CounterDisplay {
                    key:      "heading".to_string(),
                    callback: Some(f),
                }),
            ),
            Tag::End(loc(20), 0),
        ];
        let world = make_world();
        with_engine!(&world, |engine, ctx| {
            apply_counter_displays(&tags, &mut intr, &mut engine, &mut ctx);
        });
        let pre = intr.counter_displays
            .get(&("heading".to_string(), loc(20)))
            .expect("counter_displays populated mesmo com Err defensive ignore");
        assert_eq!(pre.plain_text(), "");
    }

    #[test]
    fn p241_apply_counter_displays_locations_diferentes_valores_diferentes() {
        // 2 apply_hierarchical_at em locations diferentes; CounterDisplay
        // em locations intermédias deve ver snapshots cumulativos.
        let mut intr = TagIntrospector::empty();
        intr.counters.apply_hierarchical_at("heading".to_string(), 1, loc(10));
        intr.counters.apply_hierarchical_at("heading".to_string(), 1, loc(20));
        intr.counters.apply_hierarchical_at("heading".to_string(), 1, loc(30));
        let tags = vec![
            // loc 15 → ver snapshot loc 10 = [1].
            Tag::Start(
                loc(15),
                ElementInfo::new(ElementPayload::CounterDisplay {
                    key:      "heading".to_string(),
                    callback: None,
                }),
            ),
            Tag::End(loc(15), 0),
            // loc 25 → ver snapshot loc 20 = [2] (2º depth-1 step).
            Tag::Start(
                loc(25),
                ElementInfo::new(ElementPayload::CounterDisplay {
                    key:      "heading".to_string(),
                    callback: None,
                }),
            ),
            Tag::End(loc(25), 0),
            // loc 35 → ver snapshot loc 30 = [3].
            Tag::Start(
                loc(35),
                ElementInfo::new(ElementPayload::CounterDisplay {
                    key:      "heading".to_string(),
                    callback: None,
                }),
            ),
            Tag::End(loc(35), 0),
        ];
        let world = make_world();
        with_engine!(&world, |engine, ctx| {
            apply_counter_displays(&tags, &mut intr, &mut engine, &mut ctx);
        });
        assert_eq!(
            intr.counter_displays.get(&("heading".to_string(), loc(15)))
                .unwrap().plain_text(),
            "1"
        );
        assert_eq!(
            intr.counter_displays.get(&("heading".to_string(), loc(25)))
                .unwrap().plain_text(),
            "2"
        );
        assert_eq!(
            intr.counter_displays.get(&("heading".to_string(), loc(35)))
                .unwrap().plain_text(),
            "3"
        );
    }

    #[test]
    fn p241_apply_counter_displays_counter_inexistente_array_vazio() {
        // Counter key não inicializada → value_at None → callback recebe
        // Value::Array(vec![]) (vector vazio); sem callback → Content::Empty.
        fn array_len_callback(
            _ctx: &mut crate::rules::eval::EvalContext,
            args: &crate::entities::args::Args,
            _world: &dyn crate::contracts::world::World,
            _current_file: crate::entities::file_id::FileId,
            _figure_numbering: Option<&str>,
        ) -> crate::entities::source_result::SourceResult<crate::entities::value::Value> {
            match args.items.first() {
                Some(Value::Array(items)) => Ok(Value::Str(format!("len={}", items.len()).into())),
                _ => Ok(Value::Str("?".into())),
            }
        }
        let f = Func::native("array_len_callback", array_len_callback);
        let mut intr = TagIntrospector::empty();
        let tags = vec![
            Tag::Start(
                loc(20),
                ElementInfo::new(ElementPayload::CounterDisplay {
                    key:      "inexistente".to_string(),
                    callback: Some(f),
                }),
            ),
            Tag::End(loc(20), 0),
        ];
        let world = make_world();
        with_engine!(&world, |engine, ctx| {
            apply_counter_displays(&tags, &mut intr, &mut engine, &mut ctx);
        });
        let pre = intr.counter_displays
            .get(&("inexistente".to_string(), loc(20)))
            .expect("counter_displays populated mesmo com key ausente");
        // Callback recebeu Array vazio → "len=0".
        assert_eq!(pre.plain_text(), "len=0");
    }
}
