# L0 — Compiler feature set
Hash do Código: 3ed1ce25

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/compiler_features.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9

## Evidence

The consumer defines Feature/Features outside the HTML entity owner.
Historical upstream evidence is
`lab/typst-original/crates/typst-library/src/lib.rs:272-305,283-306` at
`586e1bd43`, not a measurement against the newer pinned upstream reference.

## Contract

`Feature` is the closed public enum Html, Bundle, A11yExtras; the set's internal
representation is private. Default and empty represent the empty set.
`contains` queries membership; `enable` adds idempotently. Repetition and order
do not affect the set; all three features are independent. Neither another
feature nor an output target/format implicitly activates Bundle.
No constructor activates features by default. The explicit existing `html()`
helper and the earlier blanket constructor wording are recorded in the diagnostic
so default construction is not confused with an explicit enable request.

The canonical public type path is `entities::compiler_features::{Feature, Features}`.
The older HTML path remains a compatibility reexport of the same identities,
not a copied type, wrapper or second set. That reexport is owned by
`00_nucleo/prompts/entities/html.md`; this consumer owns only the feature data.
Keep L1 pure: no I/O, env, target, exporter, diagnostic or global mutation.
Verify empty defaults, independent features, idempotence and compatibility identity.
