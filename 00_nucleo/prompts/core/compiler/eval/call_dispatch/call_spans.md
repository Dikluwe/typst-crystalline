# L0 — compiler/eval/call_dispatch/call_spans
Hash do Código: e07b0bf6

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/eval/call_dispatch/call_spans.rs`. At HEAD `d5c563dfab39f789e8e2f72c9a9364f5a761e3ca` with a clean tree, `01_core/src/compiler/eval/call_dispatch.rs:69-389` contains the extracted behavior. SHA-256, time and initial state are recorded in `lab/results/construction/diagnostics/atomizacao-call-dispatch-2026-09-30-before.json.gz`.

## Contract

Capture syntax anchors for float, math and html call diagnostics, preserving native identity and span-selection order. Preserve public signatures, diagnostics, interception priorities, traces and argument semantics from `00_nucleo/prompts/core/compiler/eval/call_dispatch.md`. Extraction remains in L1 with static dispatch, without I/O or mutable global state.

## Acceptance

Preserve the L1 suite, call_dispatch tests and `crystalline-lint .` with correct ownership and hashes. Do not infer performance gains.


## Cardinality decisions and diagnostic priority

### Evidence before decision

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree.
`lab/results/construction/diagnostics/tekt-ranges-2026-10-16/before.json.gz`
retains UTC, sources/L0, hashes, Cargo manifests/lock/toolchain/config, status
and git diff HEAD --stat. Prior validation, hash-checked before the audit,
recorded six V18: four at call_spans.rs:52/161/206/226 and two at watch.rs:287/288.
Current L0 and branches were read before deciding. This is Rust decision
structure; messages/spans/routes remain observables to preserve, without a new
global parity measurement.

The isolated companion probe extracts exactly decode_url_path and replaces only
its digit closure in the candidate. probe-details.json records argv/rustc
1.92/SHA-256/UTC/returns; probe-result.json records 12 GiB, zero swap, one job and
events. Across all 256 bytes and 65,536 pairs, the same 22 ASCII hex bytes and
484 pairs are accepted. Complete functions are compared on pair strings and
13 controls with required UTF-8/query/dot-segment/truncated/invalid/NUL/backslash
outputs. These counts describe that probe, not HTTP/RSS performance or a moving
vanilla main. Decoder ownership remains with the watch owner.

### Exclusive responsibility

At the four flagged matches inspect args.items.as_slice() directly: [] retains
zero, [_] one and [_, _, ..] two-or-more. FloatIsNanCallSpans::anchor preserves
!has_spread guards on nonempty cases and args.span fallback; named/bound branches
and order remain. P1293MathCallSpans::anchor changes only Attach/Mono/Script
cardinality matches: [] returns call, [_, _, ..] returns the second anchor/fallback
and [_] continues. Do not move adjacent validation, bring Attach named validation
forward or postpone Mono/Script named validation. Never access a nonexistent
position, change Args capture/snapshot or touch Binom/HTML. Preserve result
bodies/signatures. The match declares collection shape rather than numeric domain
ranges; diagnostic logic remains exclusive to this owner.

### Acceptance and limits

ADR-0127 continuous flow corrects internal decision representation only: no new
public signature/field/trait, default behavior, dependency, phase or external
compatibility. L0 first, functional tests before implementation and real V18
RED. New tests pass on baseline to prove preservation, not fabricate a product
failure from a structural warning. RED is linter --checks v18 --fail-on warning;
GREEN requires zero V18, exactly six fewer Tekt warnings and no new warning by
rule across this owner and its companion correction.

- T1: float anchors for zero/one/two/more, bound/spread, named and missing captured
  positions. named wins; unbound empty anchors call even with spread; bound
  empty/spread uses args.span.
- T2: Attach/Mono/Script retain relative arity/type/unknown-name/cramped priority
  with/without spread. Binom and HTML remain unchanged.
- T3: the decoder's separate owner preserves every valid percent-encoded ASCII
  octet, NUL/backslash/invalid-UTF-8 rejection, hex case, query, dot segments,
  root and trailing slash. This is integration acceptance, not local ownership.
- T4: existing real P1293 diagnostics and HTTP/CLI controls preserve spans,
  messages, 200/301/404 and output.
- T5: build, workspace/all-targets Clippy -D warnings with
  unfulfilled_lint_expectations as error, fmt, workspace suite, Rustdoc -D warnings
  with four indexes, diff --check and strict/reverse lineage; 12 GiB, zero swap,
  one job. Freeze sources/L0 during commands and reseal only the two owner pairs.

Do not rename modules to evade the rule, add expectations/suppressions, weaken
the linter or change expected results to hide divergence. Claim no unmeasured
performance or AI-reading benefit. No release. The companion decoder's algorithm
is not authorized as a second consumer of this prompt.

## Closed-domain decision and preservation

### Evidence before decision

At checkpoint `370d78355bd646d1abead5bcacef215288cc8834`,
`01_core/src/compiler/eval/call_dispatch/call_spans.rs:302–327`
and the caller/producer evidence recorded in
`lab/results/v16-corrections-2026-10-19/batch-c/agents/structural/l0-audit.json`
show the current decision and its refusal path. That receipt records UTC,
working-tree diffstat and exact source/owner SHA-256 before this amendment.
The existing clauses above establish the behavior to preserve; an open Rust
match is structural evidence, not proof of additional language admission.

### Exclusive policy

After the early has_spread return: Void with no positionals, Normal with no positionals,
and Normal with one Content positional continue to named selection. Void nonempty,
Normal length>1, and Normal first non-Content return their positional anchors. A missing
captured anchor falls back to args.span within those early branches.

Use separate exhaustive Void and Normal arms, with positional shape and Content payload
checks inside Normal. [] and [Content] continue to named selection; a non-Content
singleton selects the first position and two-or-more selects the second before payload
checks. Void nonempty selects first. Spread always wins. Test those cases, missing
captures, unknown named full spans and ordinary named value spans.

Both current HTML native variants must be named without a wildcard; guard refusal stays inside its variant.

### Acceptance and refutation

Preservation controls run before and after the repair. A real temporary enum
variant in a disposable complete workspace must fail with E0004 at this exact
decision after repair; failures at other decisions alone are insufficient.
Exercise nested closed domains separately where present. The structural RED is
the original V16/open evolution boundary, not a fabricated runtime defect.
Any changed result, message, span, callback count, argument evaluation or refusal
route refutes preservation. Retain informational V19 groups with their rationale;
add no suppression, hidden catchall, linter/configuration weakening or new policy.

Internal continuous ADR-0127: no public field/trait/signature, product default,
phase or compatibility change. L0 precedes code; reseal only valid owner pairs
after V15/V26. Require focused and full workspace tests, build/all-targets, fmt,
strict Clippy/Rustdoc, path checks, direct/reverse lineage and differential SARIF
without new errors or warnings outside the selected repairs. Heavy checks are
serialized under one job, 12 GiB maximum, 8 GiB high and zero swap. Historical
parity claims and unresolved scope exclusions above remain bounded.
