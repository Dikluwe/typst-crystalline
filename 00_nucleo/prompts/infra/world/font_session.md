# L0 — infra/system-world/font_session
Hash do Código: c2e6253e

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/watch-font-generation.toml sha256:0f2985807cc3c7b4aeed9a5a26b77ad66dbbdb67ccfb43ba8336dbc8e8eea1a5

**Layer:** L3
**Exclusive consumer:** `03_infra/src/world/font_session.rs`


## Font retention evidence and approval

`lab/results/construction/diagnostics/watch-retencao-2026-10-07.md` retains
HEAD/tree/binaries/UTC, actual edits/recovery and dynamic per-cycle acquisition
counts. `watch-validacao-2026-10-07/measurement.json` compares discovery validation
and complete acquisition at the same state; controls.json covers additions/removals/
replacement/restored mtime/links/configuration. These are isolated costs, no CLI
gain or upstream-intent proof. FontBook/slots/coverage must preserve language
selection/content/diagnostics/render; Arc/duration are mechanics. Cost, stale
resources, regression or unbounded retention refute possible benefit. Fixtures/
provenance/limits remain in the reports.

Owner `Confirmo` ratified
`lab/history/construction/records/00_nucleo/materialization/watch-catalogo-2026-10-07/manifest.json`
under ADR-0127 §2.1/§2.2; receipt
`lab/results/construction/diagnostics/watch-catalogo-execucao-2026-10-07/ratification.json`.
Each owner remains 1:1.

## Public construction and private generation ownership

Opaque public FontCatalogSession has no public fields, with public new() -> Self
and Default delegating to Self::new(). new does no I/O; instance Mutex holds
Option of one generation with key/Arc paired slots-book. No global/persistence/
new trait beyond ratified Default/unsafe; catalog operation is pub(super) only
for World descendants.

Under session serialization obtain eligible current proof or prepare current
discovery through `00_nucleo/prompts/infra/fontdb/generation.md`. Matching key plus
identity recheck clones generation Arc without metadata/coverage/slot reconstruction.
Changed key builds from prepared discovery and revalidates before storing; never
store unstable generation. Refusal/error/poison discards/ignores old generation
and runs fresh combined acquisition from world/fonts, without caching fallback,
new user error or old negative reuse.

Keep only one generation for future reuse; active Worlds' Arc clones remain valid
by Rust ownership, with no index changes/slot clearing. Sources/maps/other services
remain new per World. generation owns guards/discovery, world/fonts composition.

Only under generation's strict eligible proof may session compare full current
proof to stored proof without fresh font read_dir. Validated hit returns paired
Arc without prepare; any change/refusal/instability uses fresh path above. On prepare
collect proof before discovery, revalidate after catalog construction, retain only
stable proof. Never store faces/slots separately, enlarge lifetime/public API or
change lazy demand/next-recompile resource recognition.

## Construction approval and acceptance

font_session.rs:23 no-argument new triggered new_without_default in
field-reassign-2026-10-15/clippy-inventory.json; clippy-default-2026-10-16 retains
source/L0/hashes. new is empty lazy Mutex::new(None), no discovery/font reads.
Owner approved manifesto
`11f3e138f5ba006d04ed4dd1e829e27c004778e691f4a1df63127b1644146d03`
on 2026-10-16, receipt
`lab/history/construction/records/00_nucleo/materialization/clippy-default-2026-10-16/approval-receipt.json`.
This supersedes prior pending gate/API bans only for its three specified Default
impls and D1–D5 tests. Keep new visibility/fields/values/other methods/semantics;
no mode/flag/global/constructor I/O/phase. Existing callers need no migration;
new consumers may use explicit Default or T: Default. Future removal breaks this
additive public contract. Bound tests fail before impl/pass after; verify fresh
session/context values/isolation/laziness/existing controls; no changed default
values or new_without_default suppression; workspace suite/all-target Clippy/build/
Rustdoc/fmt/lineage/scoped seals.

Session RED→GREEN checks same generation avoids rebuild via private test counter,
changed/refused key/postcheck failure rejects reuse, concurrent configs/Worlds keep
correct paired fonts/book, file add/remove/change and negative errors inherit no
Source/read_cache. Arc identity is internal reuse observation, never language
parity gate; product selection/text/diagnostics/render define acceptance.

## Local callable and test-only adapters

`ci-simplificacoes-2026-10-14/before.json.gz` retains source/owner/spans/HEAD/dirty
state/UTC/diffstat at :55, 73, 79; red.log captures selected errors/types checked at
1.92.0. catalog_with_fast passes verify callable directly to Option predicates,
borrowing &verify on early uses/consuming only final use. Preserve call/conditional
order/short-circuit/lock/postcheck/invalidation/fallback/one generation. Removing
identity closures permits neither early verify nor memoizing its result.

`ci-helpers-testes-2026-10-14/before.json.gz`/references-before.json.gz retains
source/owner/HEAD/tree/UTC/diffstat/calls/imports/modules/reference/test inventory,
hash-checked suite state at :35. Audited private usage/compilation, not dead_code
alone, proves catalog_with is cfg(all(test, unix))-only. Apply that same cfg to its
definition without body/fallback/poison/concurrency changes. catalog/catalog_with_fast
remain productive; helper is no public API. Keep test inventory/expectations/full
suite/lib-all-target build/Rustdoc/fmt/lineage/full inventory; scoped resealing,
selected RED→GREEN no public/default/phase/dependency/suppression, no cfg/binding
mirror tests.

`clippy-expressions-2026-10-16/before.json.gz` retains sources/L0/HEAD/tree/status/
diffstat/UTC and initial-diagnostics.json suggestions/spans/types/uses at :240
(clippy::len_zero). Lazy-acquisition test uses !world.book().is_empty() instead of
len>0, preserving query/assertion/cache order and new/Default/acquisition/session
policy. Internal representation, no new parity/performance. Continuous ADR-0127:
no public/default/phase/dependency/suppression; L0 first/scoped resealing, exact
control inputs/results/tolerances, selected RED→GREEN/workspace suite/build/
all-target Clippy/fmt/Rustdoc/lineage; no syntax-mirror tests.
