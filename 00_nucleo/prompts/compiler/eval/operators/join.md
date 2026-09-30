# L0 — compiler/eval/operators/join
Hash do Código: bc05bdad

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/operators/join.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/operators/join.rs:27` — `fn join`.

Prior source readings supporting these clauses (snapshot provenance is
retained in the assigned history diagnostic, not asserted as a fresh measurement):

- `foundations/func.rs:372-374`.
- `foundations/ops.rs:24-45`.
- `lab/typst-original/crates/typst-library/src/foundations/args.rs:468-482`.
- `typst-eval/code.rs:57`.
- `typst-eval/flow.rs:86,132`.

## Contract

Own pure sequential value combination. None is identity on either side,
preserving the other value and all its metadata. Strings and symbols concatenate
complete grapheme sequences; content/string/symbol combinations construct the
existing Content sequence/text form. Arrays concatenate in order. Dictionary
merge keeps the first key position and right-hand value on collisions.
Unsupported combinations preserve the existing long-type-name diagnostic.

Args+Args removes every left named occurrence whose name appears anywhere on the
right, preserves all left positional and remaining named occurrences in order,
then appends every right occurrence including duplicates. Construct through
Args::from_occurrences with a detached aggregate span; retain each survivor's
argument/value origin and coherent views. Do not merge collapsed named views.
This differs deliberately from With concatenation owned by
00_nucleo/prompts/compiler/eval/call_dispatch.md, which preserves all prearguments.

Code-block/loop accumulation belongs to its evaluating consumer; this helper
does not evaluate callbacks, mutate scopes or add context. Canonical long type
names delegate to the error_formatting owner.
