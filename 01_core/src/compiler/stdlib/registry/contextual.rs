//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/registry/contextual.md
//! @prompt-hash 18aace6c
//! @layer L1
//! @updated 2026-09-26

use crate::compiler::stdlib::{
    native_context, native_counter_at, native_counter_display, native_counter_final,
    native_counter_step, native_here, native_locate, native_metadata, native_query,
    native_state_at, native_state_display, native_state_final, native_state_update,
    native_state_update_with, native_target,
};
use crate::entities::func::Func;
use crate::entities::scope::Scope;
use crate::entities::value::{Type, Value};

pub(super) fn register(scope: &mut Scope) {
    // P169 (M9 sub-passo 1): metadata(value) — feature Introspection vanilla.
    scope.define("metadata", Value::Func(Func::native("metadata", native_metadata)));
    // P506: state(key, init) como valor de primeira classe.
    // **P737** — `state` é `Value::Type` (paridade vanilla — medido:
    // `type(state)` → `type`); chamabilidade via despacho P685 em
    // `eval/closures.rs` (`Type::State` → `native_state`).
    scope.define("state", Value::Type(Type::State));
    // P506: counter(selector) como valor de primeira classe.
    // **P737** — idem `state`: `Value::Type(Type::Counter)`.
    scope.define("counter", Value::Type(Type::Counter));
    // P506: context { expr } — delayed evaluation block.
    scope.define("context", Value::Func(Func::native("context", native_context)));
    // P171 (M9 sub-passo 3): state_update(key, value) — mantido como compatibilidade.
    scope.define(
        "state_update",
        Value::Func(Func::native("state_update", native_state_update)),
    );
    // P236 (Fase 5 Layout candidata Categoria D 1/?, refino aditivo
    // pós-P236.div-1): state_final(key) — valor final do state pós-walk.
    // Paralelo a counter_final P176. Reusa Introspector::state_final_value
    // P171. Retorna Value (init se ausente; última update caso contrário).
    scope.define(
        "state_final",
        Value::Func(Func::native_with_engine("state_final", native_state_final)),
    );
    // P237 (Fase 5 Layout candidata Categoria D 1/?, refino estendido):
    // state_at(key, label) — valor do state na Location associada ao label.
    // Paralelo absoluto a counter_at P177. Reusa query_by_label P139+P140
    // + state_value P171. Retorna Value::None se key/label inexistentes
    // ou state nunca actualizado antes da Location.
    scope.define(
        "state_at",
        Value::Func(Func::native_with_engine("state_at", native_state_at)),
    );
    // P172 (M9 sub-passo 4): state_update_with(key, fn) — callback variant.
    // **Stub**: from_tags ignora Func variant até pipeline restructuring.
    scope.define(
        "state_update_with",
        Value::Func(Func::native("state_update_with", native_state_update_with)),
    );
    // P240 (M9d/M7+1; ADR-0081 PROPOSTO P239 Opção γ):
    // state_display(key, [callback]) — render-mediated state display real
    // walk-time. Walk emite `Content::StateDisplay` tag; `apply_state_displays`
    // pós-fixpoint pre-renderiza Content via apply_func(callback, [value],
    // ctx, engine). Layouter consome via Introspector::state_display_value
    // (Layouter permanece puro — Opção γ vs α/β/δ P239 audit).
    scope.define(
        "state_display",
        Value::Func(Func::native("state_display", native_state_display)),
    );
    // P241 (M9d/M7+2; ADR-0081 IMPLEMENTADO parcial M7+2 paralelo P240):
    // counter_display(key, [callback]) — render-mediated counter display
    // real walk-time. Walk emite `Content::CounterDisplayCallback` tag;
    // `apply_counter_displays` pós-fixpoint converte counter slice para
    // Value::Array e aplica callback. Distinto de Content::CounterDisplay
    // { kind } legacy single-pass.
    scope.define(
        "counter_display",
        Value::Func(Func::native("counter_display", native_counter_display)),
    );
    // P175 (M9 sub-passo 5): query(kind_str) — consulta ctx.introspector
    // da iter de fixpoint anterior. Retorna Value::Int(count) — forma
    // minimal sem Value::Location.
    scope.define("query", Value::Func(Func::native("query", native_query)));
    // P504: selector(kind|func) — constrói selector como valor de primeira classe.
    scope.define("selector", Value::Type(Type::Selector));
    // P208B (M9c Bloco IV): here() — retorna Value::Location(loc) onde
    // loc = ctx.current_location. Erro contextual se current_location
    // é None (P208B infra minimal; captura automática deferred).
    scope.define("here", Value::Func(Func::native("here", native_here)));
    // P772w: target() — devolve sempre "paged" (cristalino só exporta PDF
    // via layout paginado; sem pipeline HTML/Bundle). Achado: função
    // ausente do scope global até este passo.
    scope.define("target", Value::Func(Func::native("target", native_target)));
    // P208C (M9c Bloco IV): locate(kind) — retorna primeira Location
    // do kind indicado, ou Value::None se sem matches. Reusa pattern
    // de native_query + Selector::Kind (P175 minimal); locate(<label>)
    // requer P209 (Selector::Label).
    scope.define("locate", Value::Func(Func::native("locate", native_locate)));
    // P210B (M9c Bloco V): counter_step(key) — emite
    // Content::CounterUpdate { key, action: Step } que aplica em
    // layout time. Q1=β subset minimal (counter.display + state.get
    // deferred até walk advance per P210A C3).
    scope.define(
        "counter_step",
        Value::Func(Func::native("counter_step", native_counter_step)),
    );
    // P176 (M9 sub-passo 6): counter_final(key) — formato hierárquico
    // do counter na iter de fixpoint anterior. Reusa
    // Introspector::formatted_counter (P170). Retorna Value::Str.
    scope.define(
        "counter_final",
        Value::Func(Func::native("counter_final", native_counter_final)),
    );
    // P177 (M9 sub-passo 7): counter_at(key, label) — valor do counter
    // na Location associada ao label. Reusa query_by_label +
    // formatted_counter_at. Retorna Value::Str.
    scope
        .define("counter_at", Value::Func(Func::native("counter_at", native_counter_at)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_bindings_keep_their_kinds_and_names() {
        let mut scope = Scope::new();
        register(&mut scope);
        let Value::Func(func) = scope.get("query").unwrap() else {
            panic!("query function");
        };
        assert_eq!(func.name(), Some("query"));
    }
}
