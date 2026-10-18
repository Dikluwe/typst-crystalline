//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/introspect/from_tags/displays.md
//! @prompt-hash 7b604f69
//! @layer L1
//! @updated 2026-09-30

use super::*;

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
    let _ = apply_state_displays_checked(tags, intr, engine, ctx);
}

pub(in crate::compiler::introspect) fn apply_state_displays_checked(
    tags: &[Tag],
    intr: &mut TagIntrospector,
    engine: &mut Engine<'_>,
    ctx: &mut EvalContext,
) -> SourceResult<()> {
    use crate::entities::content::Content;
    use crate::entities::value::Value;
    // P394: callbacks de state display não têm acesso ao scope de eval.
    let mut scopes = Scopes::new(None);
    for tag in tags {
        if let Tag::Start(loc, info) = tag {
            if let ElementPayload::StateDisplay { key, callback } = &info.payload {
                let value = crate::compiler::stdlib::state::read_legacy_state(
                    &intr.state,
                    key,
                    Some(*loc),
                    ctx,
                    engine,
                )?;
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
                        Value::Content(c) => c,
                        Value::LocatedContent(c, _) => c.into_content(),
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
                        | Value::Type(_)
                        | Value::SpotColorant(_) => Content::Empty,
                    },
                };
                intr.state_displays.insert((key.clone(), *loc), pre_rendered);
            }
        }
    }
    Ok(())
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
