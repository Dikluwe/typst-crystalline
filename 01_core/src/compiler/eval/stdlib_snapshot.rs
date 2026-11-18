//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/stdlib_snapshot.md
//! @prompt-hash b3955a06
//! @layer L1
//! @updated 2026-11-08
//!
//! Caller-owned immutable builtin library graph.

use crate::contracts::world::SysInputs;
use crate::entities::compiler_features::Features;
use crate::entities::func::Func;
use crate::entities::module::Module;
use crate::entities::scope::Scope;
use crate::entities::value::Value;
use std::cell::RefCell;
use std::sync::Arc;

pub(super) struct StdlibSnapshot {
    inputs: SysInputs,
    features: Features,
    scope: Scope,
    global: Scope,
}
impl StdlibSnapshot {
    fn new(inputs: SysInputs, features: Features, scope: Scope) -> Self {
        let mut global = Scope::new();
        global.define("std", Value::Module(Module::stdlib(scope.clone())));
        for (name, binding) in scope.iter() {
            global.define(name, binding.value().clone());
        }
        for (name, value) in crate::compiler::stdlib::predefined_color_bindings() {
            global.define(name.as_str(), value);
        }
        global.define(
            "text",
            Value::Func(Func::native("text", crate::compiler::stdlib::native_text)),
        );
        Self { inputs, features, scope, global }
    }
    fn scope(&self) -> &Scope {
        &self.scope
    }
    pub(super) fn global(&self) -> Scope {
        self.global.clone()
    }
    fn matches(&self, inputs: &SysInputs, features: Features) -> bool {
        self.inputs.len() == inputs.len()
            && self.inputs.iter().eq(inputs.iter())
            && self.features == features
    }
}

/// Inputs are queried at the current file boundary, before this lookup.
/// Mismatch creates an isolated subtree without replacing its parent seed.
pub(super) fn resolve(
    slot: &RefCell<Option<Arc<StdlibSnapshot>>>,
    inputs: &SysInputs,
    features: Features,
) -> (Scope, Option<Arc<StdlibSnapshot>>) {
    if let Some(seed) = slot.borrow().as_ref().filter(|s| s.matches(inputs, features)) {
        return (seed.scope().clone(), Some(Arc::clone(seed)));
    }
    let seed = Arc::new(StdlibSnapshot::new(
        inputs.clone(),
        features,
        super::make_stdlib_with_features(inputs, features),
    ));
    if slot.borrow().is_none() {
        *slot.borrow_mut() = Some(Arc::clone(&seed));
    }
    (seed.scope().clone(), Some(seed))
}

pub(super) fn root_resolve(
    slot: &RefCell<Option<Arc<StdlibSnapshot>>>,
    inputs: &SysInputs,
    features: Features,
) -> Arc<StdlibSnapshot> {
    root_graph_with(slot, inputs, features, || {
        super::make_stdlib_with_features(inputs, features)
    })
}
fn root_graph_with(
    slot: &RefCell<Option<Arc<StdlibSnapshot>>>,
    inputs: &SysInputs,
    features: Features,
    build: impl FnOnce() -> Scope,
) -> Arc<StdlibSnapshot> {
    let bytes = inputs.iter().try_fold(0usize, |size, (key, value)| {
        size.checked_add(key.len())?.checked_add(value.len())
    });
    if inputs.len() > 256 || bytes.is_none_or(|size| size > 131072) {
        *slot.borrow_mut() = None;
        return Arc::new(StdlibSnapshot::new(inputs.clone(), features, build()));
    }
    if let Some(seed) = slot.borrow().as_ref().filter(|s| s.matches(inputs, features)) {
        return Arc::clone(seed);
    }
    let seed = Arc::new(StdlibSnapshot::new(inputs.clone(), features, build()));
    *slot.borrow_mut() = Some(Arc::clone(&seed));
    seed
}
#[cfg(test)]
fn root_with(
    slot: &RefCell<Option<Arc<StdlibSnapshot>>>,
    inputs: &SysInputs,
    features: Features,
    build: impl FnOnce() -> Scope,
) -> Scope {
    root_graph_with(slot, inputs, features, build).scope().clone()
}

mod tests;
#[cfg(test)]
mod context_creation_tests {
    #[test]
    fn fresh_eval_context_keeps_stdlib_snapshot_unallocated_until_composition() {
        let context = super::super::EvalContext::new();
        assert!(context.stdlib_snapshot.borrow().is_none());
    }
}
