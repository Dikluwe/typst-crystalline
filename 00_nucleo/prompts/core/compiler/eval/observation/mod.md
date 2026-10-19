# L0 — core/compiler/eval/observation/mod
Hash do Código: 4dfbffd9

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/observation-relation.toml sha256:6f6d86b0ee72d314a84d86b12f9524b2959e075b29f99fcf6d2de9f30a4b338a

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/observation/mod.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/observation/mod.rs:31` — `enum ObservationRelation`.
- `01_core/src/compiler/eval/observation/mod.rs:52` — `trait ObservationEq`.

## Contract

Own ObservationRelation, ObservationEq, generic implementations for references,
boxes, Arc, slices, arrays, vectors, Option, Result, pairs and IndexMap, and
scalar leaves. Declare the content, value, style, geometry, math, page and
elements modules without a dynamic dispatcher or registry. Reexport only the
crate-internal structural test helpers; do not introduce a public API.

Same.and(x) treats Different as dominant evidence and Unproven as lack of
proof. Scalars use exact equality; f32/f64 compare IEEE bits. Containers compare
cardinality, order and every payload.

The cfg(test) Criterion runner is ignored by default and requires an explicit
release-profile invocation. Cover equal values, early/late differences, Content
trees, styles, geometry, math, page and element carriers through the private
trait and black_box. It must not run in production or expose benchmarking APIs.
Timing results are comparative evidence, not semantic requirements.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.
