# L0 — Element trait and module facade
Hash do Código: ea4377c5

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/mod.rs`

## Evidence and ownership

The consumer declares `Element`, the static element module index, test-only
suites and the `DynElement` reexport. ADR-0026/0105/0106 define the boundaries;
rendering locality follows ADR-0109.

This owner defines trait signatures/defaults and this consumer's topology, not
each child's fields, constructors or behavior. The facade declares distinct
`math_underline`, `math_vec`, `flush`, `outline_entry` and `math_stretch` modules
without embedding their logic. The bridge belongs to
`00_nucleo/prompts/core/entities/elements/dynamic.md`; construction by name belongs
to `00_nucleo/prompts/core/entities/element_registry.md`. Existing nuclei carry
shared obligations; prose delegation does not create shared productive ownership.

## Public trait

```rust
pub trait Element: Clone + PartialEq + std::hash::Hash + std::fmt::Debug {
    fn plain_text(&self) -> String;
    fn is_empty(&self) -> bool { false }
    fn map_content<F>(&self, f: &mut F) -> SourceResult<Content>
    where F: FnMut(&Content) -> SourceResult<Option<Content>>;
    fn map_text<F>(&self, f: &mut F) -> Content
    where F: FnMut(&str) -> String;
    fn get_field(&self, _field: &str) -> Option<Value> { None }
    fn element_kind(&self) -> Option<ElementKind> { None }
    fn to_payload(&self) -> Option<ElementPayload> { None }
    fn dyn_kind_name(&self) -> &'static str { "" }
    fn resolve_settable(&self, _get: &dyn Fn(&str) -> Option<Value>) -> Content
    where Self: Sized + Send + Sync + 'static {
        Content::dynamic(self.clone())
    }
}
```

Text and structural emptiness, child/text maps, named field access and optional
kind/payload identity retain their existing meanings. Maps rebuild the same
variant with transformed children, preserving traversal order and errors.
The empty kind-name default leaves native static dispatch unchanged; dynamic
elements override it with a stable name. The existing `resolve_settable` hook
defaults to preserving the element without applying properties; overrides retain
explicit construction > chain > default precedence.

Generic maps and supertraits deliberately make `Element` non-object-safe.
The blanket bridge supplies object-safe dispatch. Equality/hash are not extra
trait methods. Structural equality compares pointed-to values, not Arc addresses;
it must not replace separately owned language-morphology equality. Hashing is
owned by `00_nucleo/prompts/core/entities/content_hash.md`.

## Boundary and terminology

No rendering methods or layouter dependency enter the domain trait. Rendering
remains in compiler-layer owners; do not move it into entity structs or widen
layouter visibility.

“Non-locatable” means no positional identity: kind and payload are both `None`,
with no locator slot/recorded position or query/counter/state anchor. Confirm
classification with introspection owners rather than inferring it from names.
Historical evidence `01_core/src/compiler/introspect/locatable.rs:217`,
`:10-11` and `01_core/src/compiler/layout/mod.rs:993-994` records exhaustive
classification and consequences at the measured snapshot, not current coordinates.

“Hub arm” means the corresponding exhaustive Content dispatch arm. Equivalence
to a former inline arm preserves results, traversal and diagnostics, not the
former code location. Preserve trait defaults, static module surface, test gating
and native/bridge projection behavior without weakening language acceptance tests.

## Conditional test module integration

### Evidence before specification

At HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, with an uncommitted tree,
cfg(test) was duplicated at math_accent_tests.rs:5 / parent:46,
math_cases_tests.rs:5 / parent:48, and math_matrix_tests.rs:5 / parent:51.
Sources/L0/UTC/status/diffstat are in
`lab/results/construction/diagnostics/cfg-test-ci-2026-10-13/before.json.gz`
and refinement-before.json.gz. Pinned Clippy reproduces duplicate attributes.
Removing the inner condition kept Rust tests but the pinned architectural
linter did not inherit the parent's condition and emitted V2 on helpers.

### Contract and acceptance

Remove only outer cfg(test) on those three declarations. Keep module paths,
names, visibility and path attributes. Each file's inner #![cfg(test)] excludes
its entire module, imports and helpers from production. Test consumers and
owners remain independent and unchanged. Add no dependency, logic, API,
default mode or phase. Verify the same test inventory, core suite, production
build without cfg(test), Clippy and architectural checks.

## Owned test module declarations

This consumer declares the following separately owned test modules: `math_accent_tests`, `math_cases_tests`, `math_matrix_tests`. Their names describe their test responsibilities; each test module retains its own L0 owner. This module delegates the test bodies and does not absorb their ownership.
