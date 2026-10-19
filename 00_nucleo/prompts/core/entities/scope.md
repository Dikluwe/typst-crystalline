# Prompt L0 — entities/scope
Hash do Código: 66d436dc

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/scope.rs`
**References:** ADR-0023, ADR-0017, ADR-0018.

## Domain contract

Scope maps names to Binding in declaration order using
IndexMap<String,Binding,FxBuildHasher>. Preserve order during iteration and
when replacing an existing name. Binding holds only value: Value, with
new(value)->Self, value(&self)->&Value and into_value(self)->Value.
The original Value(()) examples describe the construction-era placeholder,
not a requirement to replace the current Value domain with unit.

Preserve Scope::new()->Self, Default, define(&mut self,name:impl Into<String>,
value:Value), get(&self,&str)->Option<&Value>,
get_binding(&self,&str)->Option<&Binding>,
iter(&self)->impl Iterator<Item=(&str,&Binding)>, len()->usize and is_empty()->bool.

Capturer is a Debug/Clone/Copy/PartialEq/Eq enum with Function and Context.
It records why a scope was captured, enabling the distinct mutation diagnostics
"...outside the function..." and "...outside the context expression...".
It is one value per closure invocation, stored in ClosureRepr.capturer and
propagated to Scopes at call time, not a per-binding BindingKind flag.
See `00_nucleo/prompts/core/entities/func.md` and the compiler scope owner.

## Boundaries and rationale

BindingKind, Span, Option<Category> and Option<Box<Deprecation>> remain omitted
fields under the original ADR-0017 scope; Span availability alone does not
authorize adding it. Their historical unmigrated-type rationale is preserved
in the archive and is not a claim about current migration status. Scopes<'a>
(parent-lifetime stack) belongs to eval mechanics, outside this entity;
consider inclusion only if required for Module through an explicit contract.

IndexMap preserves declaration/import/shadowing iteration semantics and O(1)
lookup. FxBuildHasher follows ADR-0018 for typically short identifiers;
the original faster-than-SipHash rationale is historical, not a fresh benchmark.

## Acceptance

An empty Scope returns None from get, is_empty true and len 0. A defined x
returns its Value. Insertion [z,a,m] iterates [z,a,m]; redefining a after [a,b]
keeps [a,b], not [b,a]. Preserve both Capturer variants and diagnostic distinction.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/scope.md`. Historical observations are not a new parity certification.
