//! Laboratory-only stdlib template. Installed into an isolated RAM compiler copy.
//! It is not a general cloning API for arbitrary evaluated modules or closures.

use std::sync::Arc;

use crate::contracts::world::SysInputs;
use crate::entities::args::Args;
use crate::entities::compiler_features::Features;
use crate::entities::func::{Func, FuncRepr, NativeFunc};
use crate::entities::module::Module;
use crate::entities::scope::Scope;
use crate::entities::value::Value;

const SHARE_IDENTITIES: bool = false;
const IGNORE_INPUTS: bool = false;
const IGNORE_FEATURES: bool = false;

pub(super) struct StdlibSnapshot {
    inputs: SysInputs,
    features: Features,
    scope: Scope,
}

impl StdlibSnapshot {
    pub(super) fn new(inputs: SysInputs, features: Features, scope: Scope) -> Self {
        Self { inputs, features, scope }
    }

    pub(super) fn scope(&self) -> &Scope {
        &self.scope
    }

    fn matches(&self, inputs: &SysInputs, features: Features) -> bool {
        (IGNORE_INPUTS
            || (self.inputs.len() == inputs.len()
                && self.inputs.iter().eq(inputs.iter())))
            && (IGNORE_FEATURES || self.features == features)
    }

    fn instantiate(&self) -> Scope {
        if SHARE_IDENTITIES {
            return self.scope.clone();
        }
        Instance::default().scope(&self.scope)
    }
}

/// The same guarded path is used by real imports and the diagnostic facade.
pub(super) fn resolve(
    previous: Option<&Arc<StdlibSnapshot>>,
    inputs: &SysInputs,
    features: Features,
) -> (Scope, Arc<StdlibSnapshot>) {
    if let Some(previous) = previous.filter(|snapshot| snapshot.matches(inputs, features))
    {
        return (previous.instantiate(), Arc::clone(previous));
    }
    let fresh = Arc::new(StdlibSnapshot::new(
        inputs.clone(),
        features,
        super::make_stdlib_with_features(inputs, features),
    ));
    // A mismatch has no previously exposed identity. Retain its fresh template
    // for this imported subtree, without replacing the caller's snapshot.
    (fresh.scope().clone(), fresh)
}

#[derive(Default)]
struct Instance {
    // Module Eq is allocation identity. Keep aliases in one instance together,
    // while creating a different graph for every file's base scope.
    modules: Vec<(Module, Module)>,
    // Unlike native functions, Element and With equality is Arc identity.
    // Compare their representations by pointer, never by native name equality.
    functions: Vec<(Func, Func)>,
}

impl Instance {
    fn scope(&mut self, original: &Scope) -> Scope {
        let mut scope = Scope::new();
        for (name, binding) in original.iter() {
            scope.define(name, self.value(binding.value()));
        }
        scope
    }

    fn value(&mut self, original: &Value) -> Value {
        match original {
            Value::Module(module) => Value::Module(self.module(module)),
            Value::Func(function) => Value::Func(self.function(function)),
            Value::Array(values) => {
                Value::Array(values.iter().map(|value| self.value(value)).collect())
            }
            Value::Dict(values) => Value::Dict(
                values
                    .iter()
                    .map(|(name, value)| (name.clone(), self.value(value)))
                    .collect(),
            ),
            Value::Args(args) => Value::Args(self.args(args)),
            // The pinned registry's other values are immutable builtin leaves
            // (symbols, primitive constants, math operator content and types).
            // This experiment does not authorize templates of user-produced
            // closures/plugins/dynamic element constructors or contexts.
            value => value.clone(),
        }
    }

    fn module(&mut self, original: &Module) -> Module {
        if let Some((_, fresh)) = self.modules.iter().find(|(old, _)| old == original) {
            return fresh.clone();
        }
        let scope = self.scope(original.scope());
        let mut fresh = if original.is_std() {
            Module::stdlib(scope)
        } else if original.is_format() {
            Module::format(scope)
        } else {
            Module::new(original.name(), scope)
        };
        // Registry modules currently have no document products. Preserve all
        // existing carriers rather than silently truncating them nonetheless.
        fresh.set_content(original.content().cloned());
        fresh.set_introspection_content(original.introspection_content().cloned());
        fresh.set_bibliography_styles(original.bibliography_styles().clone());
        fresh.set_document_info(original.document_info().clone());
        if let Some(index) = original.realization_introspector() {
            fresh.set_realization_introspector(index.clone());
        }
        self.modules.push((original.clone(), fresh.clone()));
        fresh
    }

    fn function(&mut self, original: &Func) -> Func {
        let identity_bearing =
            matches!(original.repr(), FuncRepr::Element(_) | FuncRepr::With(_));
        if identity_bearing {
            if let Some((_, fresh)) = self
                .functions
                .iter()
                .find(|(old, _)| Arc::ptr_eq(&old.0, &original.0))
            {
                return fresh.clone().with_diagnostic_span(original.diagnostic_span());
            }
        }
        let fresh = match original.repr() {
            FuncRepr::Native(native) => match &native.namespace {
                Some(namespace) => Func::native_with_namespace(
                    native.name,
                    native.call,
                    Arc::new(self.scope(namespace)),
                ),
                None => Func::native(native.name, native.call),
            },
            FuncRepr::Format(kind, native) => Func::native_format(
                *kind,
                NativeFunc {
                    name: native.name,
                    call: native.call,
                    namespace: native
                        .namespace
                        .as_ref()
                        .map(|scope| Arc::new(self.scope(scope))),
                },
            ),
            FuncRepr::NativeWithEngine(native) => match &native.namespace {
                Some(namespace) => Func::native_with_engine_and_namespace(
                    native.name,
                    native.call,
                    Arc::new(self.scope(namespace)),
                ),
                None => Func::native_with_engine(native.name, native.call),
            },
            FuncRepr::Element(element) => {
                // registry/structural.rs constructs raw.line with the
                // stateless RawLineElem::construct pointer. Recreate the Func
                // wrapper whose identity is observable; the ctor is not run.
                assert_eq!(element.name, "raw.line", "unexpected registry element");
                Func::element(element.name.clone(), Arc::clone(&element.ctor))
            }
            FuncRepr::With(bound) => {
                let function = self.function(&bound.0);
                let args = self.args(&bound.1);
                function.with(args)
            }
            // The pinned registration graph has no lexical closure or WASM
            // function. Never silently share those evaluated identities.
            FuncRepr::Closure(_) | FuncRepr::Plugin(_) => {
                panic!("laboratory stdlib template contains a non-native function")
            }
        };
        if identity_bearing {
            self.functions.push((original.clone(), fresh.clone()));
        }
        fresh.with_diagnostic_span(original.diagnostic_span())
    }

    fn args(&mut self, original: &Args) -> Args {
        let mut fresh = original.clone();
        for value in &mut fresh.items {
            *value = self.value(value);
        }
        for value in fresh.named.values_mut() {
            *value = self.value(value);
        }
        if let Some(occurrences) = &mut fresh.occurrences {
            for occurrence in occurrences {
                occurrence.value = self.value(&occurrence.value);
            }
        }
        fresh
    }
}
