# L0 — compiler/eval/operators/join
Hash do Código: 19865c14

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
00_nucleo/prompts/core/compiler/eval/call_dispatch.md, which preserves all prearguments.

Code-block/loop accumulation belongs to its evaluating consumer; this helper
does not evaluate callbacks, mutate scopes or add context. Canonical long type
names delegate to the error_formatting owner.


## Content concatenation evidence and stack boundary

HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted tree;
`lab/results/construction/diagnostics/state-stack-2026-10-17/before.json.gz`
retains sources/UTC/status/diffstat. morphology-before.json.gz and
morphology-measured.json.gz measure pin
`bd5ebec285505889e8ff1d4f68804ab147a699f3`: for/while with four [X] bodies
and repeated + yield nested Tekt Sequence versus flat vanilla children. Sequence
aliases and both-Sequence operands also differ. Empty+content keeps one-child
Sequence in vanilla; Empty+Empty empty. Empty-string Text is real child, never
filter it. Upstream foundations/ops.rs:24–45 delegates Content::add;
content/mod.rs:645–665 appends first-level Sequence children, preserving selected
Sequence identity/metadata without recursing into inner children.

GDB debug-trace/debug-root.log finds SIGSEGV in map_content_pre called by
table_cells::needs_preparation:64-update chain creates artificial recursion;
consecutive mapper frames differ104664 bytes. Large reader experiment (64 array
updates with2048 values) aborts both Tekt debug binaries, vanilla release accepts.
This proves neither State defect nor Tekt release result. Observable repr nesting
is language; Arc/Vec/frame size mechanics. Infer no intent.

## Exclusive immediate-operand concatenation

Repair producer, without configured-stack increase or indiscriminate public-tree
flattening. Keep Content::sequence normalization unchanged. Remove only immediate
Sequence layer per operand, recognizing transparent Source. Styled/Located/
Realized/ShowRecipe/body-bearing elements remain opaque children. Do not recursively
find/flatten inner Sequence children.

Keep order/immutable aliases. Result Source comes from left operand when Sequence/
Empty, otherwise right when Sequence/Empty; without Sequence create new parent
retaining both child origins. Use existing source_span/with_source_span without
redefining general origin priority. Empty represents empty Sequence here: add no
Empty child, but preserve one-child Sequence when sum involves sequence. Never
confuse Text("") with Empty or filter via is_empty/plain_text. Value::None is prior
identity without reconstruction. Existing Str/Symbol coercions stay; measured
Symbol/styled morphology and empty-Sequence children/func/fields reflection are
separate debts, no parity proof here.

Own sequential Value combination and private
pub(super) concat_content(Content, Content)->Content. Five Content-with-Content/
Str/Symbol arms delegate after existing owned coercions. Helper shares cloned
children, mutates no alias Arc and changes no Rust equality. Classification is
nominal Empty/Sequence after source_body; rest remain complete opaque values.
Empty|Sequence is one sequential-operand policy, no V19 suppression.

## Acceptance and limits

Internal/parity continuous ADR-0127, no public API/trait/field/product mode/flag/
default/phase. L0 before tests/code, functional shape/order/origin RED, GREEN and
revalidation. Heavy tasks sequential, 12 GiB/High 8 GiB/swap0/jobs1. Confirm original
full experiment, not smaller substitute; larger for/while/add/State/callback/
source/visual-product controls. Promise no linear append on Arc<[Content]>;
measure time/RSS and retain prefix-copy limitation if current representation
requires it. Local tests cover both operand sides, Empty/empty Text/None/aliases/
spans/wrappers/internal Sequence child/repeated accumulation.
