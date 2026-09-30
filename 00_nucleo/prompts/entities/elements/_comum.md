# L0 — Element trait and module facade
Hash do Código: aa8fa149

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
`00_nucleo/prompts/entities/elements/dynamic.md`; construction by name belongs
to `00_nucleo/prompts/entities/element_registry.md`. Existing nuclei carry
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
owned by `00_nucleo/prompts/entities/content_hash.md`.

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
