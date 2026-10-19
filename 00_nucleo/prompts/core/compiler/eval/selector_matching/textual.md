# L0 — Textual selector matching
Hash do Código: e5eff3d7

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/selector_matching/textual.rs`

## Evidence before decision

At HEAD ea7ae25ab (clean tree, 2026-09-30),
compiler/eval/selector_matching.rs:370–481 contains the extracted block.
This identifies its boundary, without performance measurement or a language
parity change.

## Contract

Match textual selectors and produce Text/Regex ranges/substitutions, preserving
UTF-8 offsets, order and exclusion of empty matches. Preserve public contracts,
defaults, pipeline phase, compatibility and L1 purity. Dispatch stays static;
L1 tests/build/crystalline-lint must pass. Infer no performance gain.

## Nominal selector exhaustiveness

### Evidence before decision

is_text_rule at textual.rs:13 is a latent predicate; textual_selector_matches
at :26 and textual_match_ranges at :54 emit V16. entities/show.rs:63–96 contains
Text, NodeKind, NativeElement, DynKind, Regex, Where, And, Or and Label. Current
source/lines/functions were checked against pinned SARIF and prior AST inventory,
confirming literal match presence.
`lab/results/construction/diagnostics/tekt-v16-show-selectors-2026-10-16/before.json.gz`
retains HEAD, uncommitted working tree, UTC, hashes/sources, status and exact
git diff HEAD --stat; classification.json distinguishes emitted warnings from
latent predicates. Patterns/exhaustiveness are mechanics; language selection,
text, effects and errors retain existing contracts. No new parity measurement or
unmeasured reading/performance benefit is inferred.

### Exclusive responsibility

Replace is_text_rule's matches! predicate with explicit match: Text/Regex true,
NodeKind/NativeElement/DynKind/Where/And/Or/Label false. This closes a latent
decision V16 does not emit. textual_selector_matches uses the same nontextual
false group, retaining Text/Regex bodies. textual_match_ranges uses empty group
Text/NodeKind/NativeElement/DynKind/Where/And/Or/Label, including Text because its
previous guard fails on an empty pattern. Preserve guard, offsets, order,
collection and splice functions. S2 tests cover nine variants, UTF-8 text with
two occurrences at [0,3] and [4,7], misses, empty Text and empty-match Regex.
Textual Where/And/Or do not become dedicated textual rules.

Equivalent results retain one body. Explicit groups document current policy and
require a new decision if Selector grows; do not duplicate bodies to zero V19.
V19 remains informational, without a claimed coverage deficit or test multiplier.
Guards/recursion require controls; executing a body does not prove all
alternatives. No allow/expect, exemption, severity adjustment or generic helper
may hide V16.

### Acceptance and limits

ADR-0127 continuous flow adds no public signature/field/trait, product default,
compatibility or phase. Do not reopen historical proposals. L0 before tests/code
and restricted bilateral resealing. Write local controls and confirm baseline
behavior. Structural RED is absent E0004 at six points in a real core copy with
a sentinel added only to the copied enum; after nominalization all six must
require E0004 without stubs or productive-source mutation. Remove exactly five
selected V16 while preserving external diagnostics/V19 notes. Rerun real tests,
build, fmt, strict Clippy/Rustdoc and direct/reverse lineage under 12 GiB, zero
swap and one Cargo job. Global gate stays open while external V16 remain.
