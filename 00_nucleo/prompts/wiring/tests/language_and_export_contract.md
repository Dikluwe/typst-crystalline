# L0 — wiring/tests/language_and_export_contract
Hash do Código: 2d6ea4e4

**Layer:** L4
**Exclusive consumer:** `04_wiring/tests/language_and_export_contract.rs`

## Current evidence

- `04_wiring/tests/language_and_export_contract.rs:24` — `Transcript`.
- `04_wiring/tests/language_and_export_contract.rs:31` — `from_output`.
- `04_wiring/tests/language_and_export_contract.rs:39` — `success`.
- `04_wiring/tests/language_and_export_contract.rs:44` — `trim_transport`.
- `04_wiring/tests/language_and_export_contract.rs:50` — `run`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/tests/language_and_export_contract.rs`.

## Owned contract

- Own only black-box language/export integration: float.is-nan identity/bound calls/casts/diagnostics; math attach/binom/mono/script morphology and geometry (no binom bar); HTML fields/DOM/attributes/escaping/void/nesting/feature-target separation; grid/table short names/public aliases/with/arity and absent nonpublic aliases.
- Compare public language observations, not implementation hashes, administrative catalogs, authorship or execution ordering. Any upstream comparison requires the independently verified current pin; archived earlier runs are not proof against it.
- Use isolated temporary fixtures and finite process limits. Default PNG requires safe-memory success and correct IHDR at 144 PPI (2 pixels/pt); missing output, signal or timeout is failure, not a tolerated result.

## Verification and boundary

Run `cargo test -p typst-wiring --test language_and_export_contract -- --test-threads=1` with the harness prerequisites above.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Oracle provenance and current reference

### Evidence

The 2026-10-13 audit at language_and_export_contract.rs:21 found a hardcoded
/usr/local/bin/typst. The historical provenance report showed it still referred
to an older pin while that audit targeted
`9f2b6e8715237cb086899a42873660fe744622e8`, tree
`6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`; a fresh CI runner lacks that local
installation. This is harness provenance, not a language change or permission
to weaken observations.

At checkpoint `e2d7f6cdc93c5f6b948ca80167411d390ccc714d`, the 2026-10-17 audit of
`tools/ci/build_oracle.py:12–13` and `.github/workflows/ci.yml:37`, and official
`git ls-remote upstream HEAD refs/heads/main`/fetch retained in
`lab/results/construction/diagnostics/vanilla-resync-2026-10-17/`, identify current
commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`. The missing-configuration message at
language_and_export_contract.rs:28 still named the earlier pin; required variable
and comparators did not change.

### Contract and acceptance

Require TYPST_VANILLA for the executable. Before use, independently verify the
current commit/tree/build recipe/executable SHA-256. CI builds a clean pinned
checkout and retains the receipt. Historical permission to use the earlier
ratified lab binary applies only to that historical snapshot; it does not replace
the current resync requirement. Missing configuration fails explicitly, with no
PATH, /usr/local/bin/typst or Crystalline fallback. Update only the configuration
message to the explicit current pin. Rerun against it without relaxed language/
diagnostic/morphology expectations or tolerances. Historical measurements do
not certify the new pin. This is continuous ADR-0127 test infrastructure, no
product public/default/compatibility/phase change; L0 first, owner-only resealing.

## Internal observation expressions

`clippy-expressions-2026-10-16/before.json.gz` retains sources/L0/HEAD/uncommitted
tree/status/diffstat/UTC, and initial-diagnostics.json retains suggestions/spans:
language_and_export_contract.rs:375 clippy::needless_range_loop and :580
clippy::ptr_arg. Uses/types were inspected before deciding; no new parity or
performance claim follows from internal Rust representation.

For viewBox iterate expected.iter().enumerate() over the four-coordinate array,
preserving indices, summary.0[index] comparisons, tolerance/messages; no zip that
silently truncates a short observation. In push_dom_node only stack changes
&mut Vec<DomNode>→&mut [DomNode], since only last_mut is used; roots stays Vec
for push. Preserve private callers/DOM/children. Continuous ADR-0127: no public/
default/phase/dependency/suppression change. L0 first, scoped resealing, unchanged
inputs/results/tolerances; selected-lint RED→GREEN then workspace suite/build/
all-target Clippy/fmt/Rustdoc/lineage; no syntax-mirror tests.

## Harness boundary

### Evidence

`lab/results/construction/diagnostics/tekt-v12-2026-10-16/before.json.gz`
retains HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC,
sources/L0/configuration/lock/hashes/status/diffstat. Its 2077 hashes match the
final FrameItem checkpoint: the prior suite ran 8622 tests with zero failures
and seven ignored at that same state. gate-baseline-result.json records real RED
for 19 V12 before L0/code changes. items.json contains complete AST items and
classification.json records responsibility per unit. These are historical
measurements, not new parity or upstream-intent claims.

The linter identifies top-level enums/inherent impls in L4, including test support.
Scenarios, language fixtures and final product assertions stay in L4; independent
subprocess/filesystem/observation helpers belong to L3 test support. Do not move
the whole suite, hide declarations inside an L4 module to exploit shallow parsing,
exclude tests from lint, convert impls solely to evade the rule or change severity.

### Exclusive responsibility

Use `03_infra/tests/support/wiring/language_and_export_contract.rs`, owned exclusively by
`00_nucleo/prompts/infra/tests/support/wiring/language_and_export_contract.md`, for Transcript, Scratch, DomNode, ParsedDom, their existing impls and Drop.
Also transfer generic observation helpers trim_transport, decode_entities, tag_end, parse_open_tag, push_dom_node, parse_dom and visit_elements without algorithm changes.
Keep all scenarios and final observations in this L4 consumer. Import only
ParsedDom, Scratch, Transcript, parse_dom, visit_elements through a harness module with static #[path] to L3;
never use that module in the production library.

Recorded pre-transfer item anchors (historical, not current declaration locations):
04_wiring/tests/language_and_export_contract.rs:28 struct_item Transcript; 04_wiring/tests/language_and_export_contract.rs:34 impl_item Transcript; 04_wiring/tests/language_and_export_contract.rs:48 function_item trim_transport; 04_wiring/tests/language_and_export_contract.rs:126 struct_item Scratch; 04_wiring/tests/language_and_export_contract.rs:128 impl_item Scratch; 04_wiring/tests/language_and_export_contract.rs:141 impl_item Scratch; 04_wiring/tests/language_and_export_contract.rs:519 enum_item DomNode; 04_wiring/tests/language_and_export_contract.rs:525 struct_item ParsedDom; 04_wiring/tests/language_and_export_contract.rs:530 function_item decode_entities; 04_wiring/tests/language_and_export_contract.rs:538 function_item tag_end; 04_wiring/tests/language_and_export_contract.rs:550 function_item parse_open_tag; 04_wiring/tests/language_and_export_contract.rs:580 function_item push_dom_node; 04_wiring/tests/language_and_export_contract.rs:588 function_item parse_dom; 04_wiring/tests/language_and_export_contract.rs:650 function_item visit_elements.

### Acceptance and limits

This internal ADR-0127 harness separation does not put support into the L3
production library or CLI. pub(super) grants access only inside the test crate.
No public product API/dependency/flag/default/phase changes. The separately
ratified Published contract grants no other contracts. No new derives,
suppressions, hooks, cleanup policy, tolerance or verification fallback.

Preserve moved item bodies except minimum visibility for the parent test module;
Checks fields remain private where present. L3 owns its std imports, with no
use super::* or dependency on L4 logic. BIN, subprocess invocation/options,
scenarios/expectations and #[test] remain in L4; support does not read
CARGO_BIN_EXE_typst. Retain legitimate per-harness differences in prefixes,
messages, creation atomicity and cleanup.

Check restricted transfer replay, identical test functions/attributes/expectations,
valid 1:1 graph, no support in the productive library, V12 GREEN zero and complete
SARIF without new warnings. The original batch required workspace suite with
explicit /dev/shm TMPDIR and historical vanilla pin
`9f2b6e8715237cb086899a42873660fe744622e8`, binary SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
That historical receipt does not certify the current pin; subsequent runs use the
ratified current oracle after independent recipe/source/tree/SHA verification.
Preserve all expectations/tolerances. Validate cargo build, workspace/all-target
Clippy -D warnings/-D unfulfilled_lint_expectations, fmt, Rustdoc -D warnings/four
indexes, strict V1/V5/V15/V26/reverse lineage and diff --check. L0 first; resealing
limited to authorized owners/code. Heavy jobs sequential under MemoryMax=12 GiB,
MemorySwapMax=0, CARGO_BUILD_JOBS=1; freeze sources/L0/configuration during commands.
No import-syntax mirror tests, unmeasured RAM/AI-gain claims or release publication.
