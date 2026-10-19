# L0 — introspect/convergence
Hash do Código: d7911794

## Ownership and evidence

Layer: L1. Sole consumer: `01_core/src/compiler/introspect/convergence.rs`.
The body of `compute_tags_hash` at `:29` hashes the complete tag slice with
Rust DefaultHasher. `01_core/src/compiler/introspect/fixpoint.rs:129` compares
successive hashes. This is an internal structural change detector, not an
implementation of the production pipeline's semantic convergence policy.

## Contract

Keep `pub fn compute_tags_hash(tags: &[Tag]) -> u64` pure and stateless.
Hash the ordered slice through its Hash implementation, including payloads,
Locations and End hashes. Equal input in the same implementation yields the
same digest, including empty input. Order is significant.

No I/O, memoization, global cache or new dependency is required.
Do not promise a collision-free comparison, a stable persisted digest across
Rust versions, or a fixed named hashing algorithm behind DefaultHasher.
A changed payload, Location, End hash or ordering is a relevant change input;
representative differing inputs must be tested, not elevated to a mathematical
guarantee that all unequal inputs have unequal hashes.

## Acceptance

Cover repeatability, empty input, equivalent slices and representative changes
in payload, Location, End hash and order. The bounded loop and its errors belong
only to `00_nucleo/prompts/core/compiler/introspect/fixpoint.md`.
Language convergence and upstream parity need their own observable evidence.


## Ratified compact carrier fixture adaptation

### Evidence before decision

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree;
`lab/results/construction/diagnostics/clippy-enums-publicos-2026-10-16/before.json.gz`
retains UTC/sources/L0/hashes/status/git diff HEAD --stat. Existing owners were
read before proposing. probe-details.json retains rustc 1.92/argv/rlib/binary/
probe SHA-256/output; probe-result.json records 12 GiB/zero swap/one job/events.
This synthetic carrier/register-copy probe is not compile/layout/RSS benchmarking
or a real workload distribution. Candidates mirror real domain types; inline
uses productive Tag/ShowRule.

Tag is 416 bytes, boxed/Arc Start 48. For 4,096 Start/End pairs with heap-free
Heading, requested bytes/allocations are inline 3,407,872/1, Box 2,031,616/4,097,
Arc 2,097,152/4,097. Cloning boxed Tag allocates per Start; inspected productive
fixpoint.rs:86–136 and entrypoints.rs:44–66 use temporary Vec<Tag>/borrowed reads,
without cloning the vector. Walk:325–356 populates the index by reference before
moving info to Start. Synthetic clone frequency is not assumed productive.

Transformation is 328 and ShowRule 376 bytes; Box/Arc<Styles> candidates are 120
and 168. rules/show.rs:464–470 copies to_vec before every registration;
apply.rs:108–112,349–352 filters/clones rules. Across 128 registrations of simple
100% show-set without retained old snapshots, requested bytes/allocations are
inline 12,276,960/384, Box 8,194,560/8,640, Arc 5,530,624/512. With retained
snapshots, requested peaks are 3,203,856, 4,141,744 and 1,477,808. A custom-channel
control copying 4,096 rules requests 4,107 inline, 8,203 Box and 11 Arc
allocations, including Vec growth. apply/intercept/outline/table_cells borrow
styles; application still clones a delta when needed.

Mandatory counterexample: building one simple 100% show-set collection of 4,096
rules requests 1,540,096 bytes inline, 2,031,616 Box and 2,097,152 Arc. Arc may
retain excess custom-channel capacity; the rich 128-registration control without
old snapshots ends with 78,064 live bytes inline and 156,432 Arc. Claim no
universal gain or ignore atomic/indirection cost. This is ADR-0030/0107 storage
mechanics, not language parity or measured AI benefit. Real relevant memory/time
regression refutes the favorable copy-cycle inference; retain those controls.

### Ratification and exclusive responsibility

The human approved manifest
`282da3f8f79080a4831e1898879775e32a62caa0b2ffa941b8fca9198136b950`;
`lab/results/construction/diagnostics/clippy-enums-publicos-2026-10-16/approval-receipt.json`
records approval. The former ADR-0127 public-payload/compatibility proposal gate
is satisfied; this contract, including T1–T6 and no release, is in force.

Migrate only this consumer's fixture Tag::Start constructors to Box::new of the
same ElementInfo. Preserve Locations/payloads/labels/End hashes/calls/assertions
and every productive body. Borrowed reads/value hashes retain their contracts;
add no hash algorithm/error and change no expectations to hide a difference.
The public payload belongs only to `00_nucleo/prompts/core/entities/tag.md`.

### Acceptance and owner boundaries

Ratified L0 precedes tests and real RED; reseal strictly changed owners. No
large_enum_variant suppression, threshold/dependency/flag/default/phase change.

- T1: construction Box<ElementInfo>/Arc<Styles> and relative budgets (Tag at most
  three Location; Transformation at most Content plus alignment) fail before
  and pass after without freezing a universal ABI. Those payloads stay owned by
  their entity prompts.
- T2: preserve Start/End/Location/payload/label, value equality/structural hashes,
  independent Box clone and mutation isolation; hashes are not persistence format.
- T3: existing introspection/convergence/nesting/heading/label/state/counter/query/
  metadata tests retain expectations/errors.
- T4: Transformation::Style clone shares Arc; Arc::make_mut on a copy preserves
  original styles including custom; Func/Content/Str variants remain unchanged.
  This is a dependent entity boundary, not implementation in convergence.
- T5: show-set/func, precedence/scope/labels/outline/table.cell/textual/regex
  controls preserve semantics/morphology/errors with no extra execution.
- T6: full workspace/build/all-targets Clippy with prior expectations/lints as
  errors, rustdoc -D warnings with four indexes, fmt/diff/strict/reverse lineage
  and no Tekt increase, under 12 GiB/zero swap/one job. Record HEAD/working tree/
  hashes/diffstat/UTC and distinguish requested sizes/bytes from RSS.

No release. Any signature/field/phase change beyond these ratified payloads
requires a separate proposal. This owner adapts only its fixtures and keeps
compute_tags_hash's independent ordered-value contract unchanged.
