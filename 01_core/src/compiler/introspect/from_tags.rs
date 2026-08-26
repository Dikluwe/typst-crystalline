//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/introspect/from_tags.md
//! @prompt-hash e4e2bfb5
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
    tags: &[Tag],
    intr: &mut TagIntrospector,
    engine: &mut Engine<'_>,
    ctx: &mut EvalContext,
) -> SourceResult<()> {
    // P394: callbacks de state não têm acesso ao scope de eval; usam scope
    // vazio (closures trazem o seu captured scope como parent).
    let mut scopes = Scopes::new(None);
    for tag in tags {
        if let Tag::Start(loc, info) = tag {
            if let ElementPayload::StateUpdate { key, update } = &info.payload {
                if let StateUpdate::Func(func) = update {
                    if let Some(curr) = intr.state.value_at(key, *loc).cloned() {
                        let args = Args::positional(vec![curr]);
                        match apply_func(func.clone(), args, &mut scopes, ctx, engine) {
                            Ok(new_value) => {
                                intr.state.update(key.clone(), new_value, *loc);
                            }
                            Err(diagnostics) => return Err(diagnostics),
                        }
                    }
                    // value_at == None: defensive ignore (P171 padrão
                    // "update sem init").
                }
            }
        }
    }
    Ok(())
}

/// P1148 — aplica callbacks de `counter.update` na mesma fase pós-walk dos
/// callbacks de state. O argumento é o estado corrente como array; o retorno
/// deve ser inteiro não-negativo ou array de inteiros não-negativos.
pub fn apply_counter_funcs(
    tags: &[Tag],
    intr: &mut TagIntrospector,
    engine: &mut Engine<'_>,
    ctx: &mut EvalContext,
) -> SourceResult<()> {
    use crate::entities::value::Value;

    let mut scopes = Scopes::new(None);
    for tag in tags {
        let Tag::Start(loc, info) = tag else { continue };
        let ElementPayload::CounterUpdate { key, action: CounterUpdate::Func(func) } =
            &info.payload
        else {
            continue;
        };

        let current = intr.counters.value_at(key, *loc).unwrap_or(&[0]);
        let inputs = current.iter().map(|value| Value::Int(*value as i64)).collect();
        let result =
            apply_func(func.clone(), Args::positional(inputs), &mut scopes, ctx, engine)?;
        let state = match result {
            Value::Int(value) if value >= 0 => vec![value as usize],
            Value::Array(values) => values
                .into_iter()
                .map(|value| match value {
                    Value::Int(value) if value >= 0 => Ok(value as usize),
                    other => Err(vec![
                        crate::entities::source_result::SourceDiagnostic::error(
                            crate::entities::span::Span::detached(),
                            format!(
                                "counter update function returned {} instead of integer",
                                other.type_name()
                            ),
                        ),
                    ]),
                })
                .collect::<SourceResult<Vec<_>>>()?,
            other => {
                return Err(vec![
                    crate::entities::source_result::SourceDiagnostic::error(
                        crate::entities::span::Span::detached(),
                        format!(
                        "counter update function returned {} instead of integer or array",
                        other.type_name()
                    ),
                    ),
                ])
            }
        };
        intr.counters.apply_at(key.clone(), CounterUpdate::Set(state), *loc);
    }
    Ok(())
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
    tags: &[Tag],
    intr: &mut TagIntrospector,
    engine: &mut Engine<'_>,
    ctx: &mut EvalContext,
) {
    use crate::entities::content::Content;
    use crate::entities::value::Value;
    // P394: callbacks de state display não têm acesso ao scope de eval.
    let mut scopes = Scopes::new(None);
    for tag in tags {
        if let Tag::Start(loc, info) = tag {
            if let ElementPayload::StateDisplay { key, callback } = &info.payload {
                let value =
                    intr.state.value_at(key, *loc).cloned().unwrap_or(Value::None);
                let pre_rendered = match callback {
                    Some(func) => {
                        let args = Args::positional(vec![value]);
                        match apply_func(func.clone(), args, &mut scopes, ctx, engine) {
                            Ok(Value::Content(c)) => c,
                            Ok(Value::Str(s)) => Content::text(s.as_str()),
                            Ok(_) => Content::Empty,
                            Err(_) => Content::Empty,
                        }
                    }
                    None => match value {
                        Value::Content(c) | Value::LocatedContent(c, _) => c,
                        Value::Str(s) => Content::text(s.as_str()),
                        // intencional: valores que não possuem renderização textual direta em state display sem callback
                        Value::None
                        | Value::Bool(_)
                        | Value::Int(_)
                        | Value::Float(_)
                        | Value::Array(_)
                        | Value::Dict(_)
                        | Value::Module(_)
                        | Value::Datetime(_)
                        | Value::Func(_)
                        | Value::Auto
                        | Value::Length(_)
                        | Value::Relative(_)
                        | Value::Ratio(_)
                        | Value::Angle(_)
                        | Value::Color(_)
                        | Value::Stroke(_)
                        | Value::Fraction(_)
                        | Value::Align(_)
                        | Value::Location(_)
                        | Value::Gradient(_)
                        | Value::Regex(_)
                        | Value::Tiling(_)
                        | Value::Bytes(_)
                        | Value::Decimal(_)
                        | Value::Duration(_)
                        | Value::Version(_)
                        | Value::Selector(_)
                        | Value::Symbol(_)
                        | Value::Args(_)
                        | Value::State(_)
                        | Value::Counter(_)
                        | Value::Label(_)
                        | Value::Dir(_)
                        | Value::Path(_)
                        | Value::Type(_) => Content::Empty,
                    },
                };
                intr.state_displays.insert((key.clone(), *loc), pre_rendered);
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
    tags: &[Tag],
    intr: &mut TagIntrospector,
    engine: &mut Engine<'_>,
    ctx: &mut EvalContext,
) {
    use crate::entities::content::Content;
    use crate::entities::value::Value;
    // P394: callbacks de counter display não têm acesso ao scope de eval.
    let mut scopes = Scopes::new(None);
    for tag in tags {
        if let Tag::Start(loc, info) = tag {
            if let ElementPayload::CounterDisplay { key, callback } = &info.payload {
                let counter_slice_opt = intr.counters.value_at(key, *loc);
                let counter_value: Value = counter_slice_opt
                    .map(|slice| {
                        Value::Array(
                            slice.iter().map(|&n| Value::Int(n as i64)).collect(),
                        )
                    })
                    .unwrap_or(Value::Array(vec![]));
                let pre_rendered = match callback {
                    Some(func) => {
                        let args = Args::positional(vec![counter_value]);
                        match apply_func(func.clone(), args, &mut scopes, ctx, engine) {
                            Ok(Value::Content(c)) => c,
                            Ok(Value::Str(s)) => Content::text(s.as_str()),
                            Ok(_) => Content::Empty,
                            Err(_) => Content::Empty,
                        }
                    }
                    None => match counter_slice_opt {
                        Some(slice) => {
                            let s = slice
                                .iter()
                                .map(|n| n.to_string())
                                .collect::<Vec<_>>()
                                .join(".");
                            Content::text(&s)
                        }
                        None => Content::Empty,
                    },
                };
                intr.counter_displays.insert((key.clone(), *loc), pre_rendered);
            }
        }
    }
}

/// Materializa o número visível de cada equação depois que o contador
/// convergiu. O layouter consome apenas `Content` e permanece puro.
pub fn apply_equation_numberings(
    tags: &[Tag],
    intr: &mut TagIntrospector,
    engine: &mut Engine<'_>,
    ctx: &mut EvalContext,
) -> SourceResult<()> {
    use crate::entities::content::Content;
    use crate::entities::counter::CounterKey;
    use crate::entities::element_kind::ElementKind;
    use crate::entities::selector::Selector;
    use crate::entities::source_result::SourceDiagnostic;
    use crate::entities::span::Span;
    use crate::entities::value::Value;

    let key = CounterKey::Selector(Selector::Kind(ElementKind::Equation));
    let mut scopes = Scopes::new(None);
    for tag in tags {
        let Tag::Start(loc, info) = tag else { continue };
        let ElementPayload::Equation { block, numbering_active, .. } = &info.payload
        else {
            continue;
        };
        if !*block || !*numbering_active {
            continue;
        }
        let n = intr
            .counters
            .value_at(&key, *loc)
            .and_then(|v| v.last())
            .copied()
            .unwrap_or(0);
        let content =
            if let Some(callback) = intr.equation_numbering_callbacks.get(loc).cloned() {
                let args = Args::positional(vec![Value::Int(n as i64)]);
                match apply_func(callback, args, &mut scopes, ctx, engine)? {
                    Value::Content(content) => content,
                    Value::Str(text) => Content::text(text.as_str()),
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            Span::detached(),
                            format!(
                                "expected content or string, found {}",
                                other.type_name()
                            ),
                        )]);
                    }
                }
            } else if let Some(pattern) = intr.equation_numbering_pattern.get(loc) {
                let text = crate::entities::counter_format::format_counter(&[n], pattern)
                    .unwrap_or_else(|| n.to_string());
                Content::text(&text)
            } else {
                continue;
            };
        intr.equation_numbering_contents.insert(*loc, content);
    }
    Ok(())
}

/// Materializa o suplemento próprio de cada equação com os styles capturados
/// no alvo. O layout consome somente o `Content` resultante.
pub fn apply_equation_supplements(
    tags: &[Tag],
    intr: &mut TagIntrospector,
    engine: &mut Engine<'_>,
    ctx: &mut EvalContext,
) -> SourceResult<()> {
    use crate::compiler::eval::value_to_display_content;
    use crate::compiler::lang::equation_supplement::equation_supplement_for_lang;
    use crate::entities::content::Content;
    use crate::entities::value::Value;

    let mut scopes = Scopes::new(None);
    for tag in tags {
        let Tag::Start(loc, info) = tag else { continue };
        if !matches!(info.payload, ElementPayload::Equation { .. }) {
            continue;
        }
        let specification =
            intr.equation_supplements.get(loc).cloned().unwrap_or(Value::Auto);
        let content = match specification {
            Value::Auto => {
                let lang =
                    intr.equation_supplement_langs.get(loc).and_then(|v| v.as_ref());
                Content::text(equation_supplement_for_lang(lang))
            }
            Value::None => Content::Empty,
            Value::Func(callback) => {
                let base = intr.element_at(*loc).cloned().unwrap_or(Content::Empty);
                let result = apply_func(
                    callback,
                    Args::positional(vec![Value::Content(base)]),
                    &mut scopes,
                    ctx,
                    engine,
                )?;
                value_to_display_content(result).unwrap_or(Content::Empty)
            }
            other => value_to_display_content(other).unwrap_or(Content::Empty),
        };
        intr.equation_supplement_contents.insert(*loc, content);
    }
    Ok(())
}

/// Substitui a cópia nua guardada por `element_at` pela visão pública e
/// realizada da equação. Isso preserva os valores efetivos usados por query
/// sem acrescentá-los ao `EquationElem` de domínio.
pub fn realize_equation_elements(tags: &[Tag], intr: &mut TagIntrospector) {
    use crate::entities::content::Content;
    use crate::entities::style::Styles;
    use crate::entities::value::Value;

    for tag in tags {
        let Tag::Start(loc, info) = tag else { continue };
        if !matches!(info.payload, ElementPayload::Equation { .. }) {
            continue;
        }
        let Some(base) = intr.elements.get(loc).cloned() else { continue };
        let mut styles = Styles::new()
            .push_custom(
                "equation.numbering",
                intr.equation_numberings.get(loc).cloned().unwrap_or(Value::None),
            )
            .push_custom(
                "equation.number-align",
                Value::Align(intr.equation_number_aligns.get(loc).copied().unwrap_or(
                    crate::entities::layout_types::Align2D {
                        h: Some(crate::entities::layout_types::HAlign::End),
                        v: Some(crate::entities::layout_types::VAlign::Horizon),
                    },
                )),
            )
            .push_custom(
                "equation.supplement",
                Value::Content(
                    intr.equation_supplement_contents
                        .get(loc)
                        .cloned()
                        .unwrap_or(Content::Empty),
                ),
            );
        styles = styles.push_custom(
            "equation.alt",
            intr.equation_alts
                .get(loc)
                .and_then(|alt| alt.clone())
                .map(Value::Str)
                .unwrap_or(Value::None),
        );
        intr.elements.insert(*loc, Content::Styled(Box::new(base), styles));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::layout::FixedMetrics;
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
                    key: "c".to_string(),
                    update: StateUpdate::Func(f),
                }),
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
                ElementInfo::new(ElementPayload::StateUpdate {
                    key: "c".to_string(),
                    update: StateUpdate::Func(f),
                }),
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
                ElementInfo::new(ElementPayload::StateUpdate {
                    key: "c".to_string(),
                    update: StateUpdate::Func(f1),
                }),
            ),
            Tag::End(loc(20), 0),
            Tag::Start(
                loc(30),
                ElementInfo::new(ElementPayload::StateUpdate {
                    key: "c".to_string(),
                    update: StateUpdate::Func(f2),
                }),
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
                ElementInfo::new(ElementPayload::StateUpdate {
                    key: "c".to_string(),
                    update: StateUpdate::Func(f),
                }),
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
                ElementInfo::new(ElementPayload::StateDisplay {
                    key: "k".to_string(),
                    callback: None,
                }),
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
                ElementInfo::new(ElementPayload::StateDisplay {
                    key: "k".to_string(),
                    callback: Some(f),
                }),
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
                ElementInfo::new(ElementPayload::StateDisplay {
                    key: "k".to_string(),
                    callback: Some(f),
                }),
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
                ElementInfo::new(ElementPayload::StateDisplay {
                    key: "k".to_string(),
                    callback: None,
                }),
            ),
            Tag::End(loc(12), 0),
            Tag::Start(
                loc(20),
                ElementInfo::new(ElementPayload::StateDisplay {
                    key: "k".to_string(),
                    callback: None,
                }),
            ),
            Tag::End(loc(20), 0),
            Tag::Start(
                loc(30),
                ElementInfo::new(ElementPayload::StateDisplay {
                    key: "k".to_string(),
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
                ElementInfo::new(ElementPayload::StateDisplay {
                    key: "inexistente".to_string(),
                    callback: None,
                }),
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
                ElementInfo::new(ElementPayload::CounterDisplay {
                    key: ck("heading"),
                    callback: None,
                }),
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
                ElementInfo::new(ElementPayload::CounterDisplay {
                    key: ck("heading"),
                    callback: Some(f),
                }),
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
                ElementInfo::new(ElementPayload::CounterDisplay {
                    key: ck("heading"),
                    callback: Some(f),
                }),
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
                ElementInfo::new(ElementPayload::CounterDisplay {
                    key: ck("heading"),
                    callback: None,
                }),
            ),
            Tag::End(loc(15), 0),
            // loc 25 → ver snapshot loc 20 = [2] (2º depth-1 step).
            Tag::Start(
                loc(25),
                ElementInfo::new(ElementPayload::CounterDisplay {
                    key: ck("heading"),
                    callback: None,
                }),
            ),
            Tag::End(loc(25), 0),
            // loc 35 → ver snapshot loc 30 = [3].
            Tag::Start(
                loc(35),
                ElementInfo::new(ElementPayload::CounterDisplay {
                    key: ck("heading"),
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
                ElementInfo::new(ElementPayload::CounterDisplay {
                    key: ck("inexistente"),
                    callback: Some(f),
                }),
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
