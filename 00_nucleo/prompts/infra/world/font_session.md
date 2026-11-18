# L0 — infra/system-world/font_session
Hash do Código: bbd489b3

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/watch-font-generation.toml sha256:0132d0e50e81227203f577144758a78792358faee644dc236f74e2d6d8b80fa9



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


## Ratified laboratory selective-font foundation — 2026-11-12T19:11:20.034402+00:00

The user explicitly authorized implementing and testing World::select_font and
World::font_info only in the laboratory, without integration. The exact approved
proposal patch is 281b8289424b9c24fe61b8890cd30a12e8d9331165afc4b81590a6e80e85cc30;
fontdemand-validation/approval.json preserves the wording and integrated base.
This changes the earlier DRAFT gate status to ratified solely for that scope.

Evidence before decision: structural/selection-proof/receipt.json binds real
selection/bytes/metadata observations to unchanged font modules and verified
libraries. Its 2142 query matrix has 60 minimum-weight/style-distance prefix
winners, all equal to the complete catalog; later equal-score entries do not
displace them. Better suffix variants and absent-prefix families refute name-only
selection or negative proof. Embedded math/code follows mutable system entries
and cannot provide a stable leading index. structural/repeat2/receipt.json
locates about 13ms by deliberately omitting catalog validation, not by a safe
candidate; impact.json shows byte/metadata/export queries must also stay selective.
This foundation alone makes no end-to-end speed claim.

No partial book, changed ordering, stale system font, negative answer without
proof, global cache, extra public method, unsafe or changed demand phase. Default
methods on arbitrary World delegate to full current book. Only combined policy
may expose the immutable leading text group. Literal query certification requires
existing select_pattern winner with minimum weight/style distances; regex, absent
family and nonminimum score use existing full acquisition. Current select_pattern
ranking is observed mechanics, not a new stretch/language parity decision.

Tests precede implementation: defaults equal current full selection/metadata;
exact prefix queries do not call the full loader; bytes/metadata/indices remain
stable after full-book acquisition; better variants, absent/regex/math-code
requests force full path; replacing builders clears old prefix; concurrent
requests/new Worlds share only immutable data, bounded by the session. Keep
existing generation freshness controls and original fixture tolerances.

Exclusive owning obligation: Own one shared Arc of the private lazy immutable prefix alongside the existing single mutable-generation cache. Session new allocates empty state only, no font discovery. Keep existing catalog proof/rebuild/refusal/poison/concurrency behavior exactly. Only immutable built-in data is shared by fresh Worlds; no metrics/raw-shaping/layout retention.

## Productive adoption scope — 2026-11-18

The user explicitly authorizes the cumulative packages with measured gain. This
owner adopts only its finalized implementation in the preserved capture candidate
29115606, qualified together with its required dependency owners; earlier
laboratory-only exclusions are superseded only for this specific implementation.
Existing current project ADR scope clarifications and conservative fallbacks
remain in force. No additional public method, default or phase is invented here;
already specified optional capabilities keep their explicit authority/None paths.
Diagnostic census, no-retain ablations and blocking watch markers are not product
features. Current adoption uses disk-backed task targets, one Cargo job,12GiB AS,
8GiB tree RSS and no swap; prior task-specific temporary-target paths are scoped
to their original correction. Require real capture RED→GREEN, complete suites,
lineage/ownership, source/effect/invalidation controls and measured before/after
and pinned vanilla comparison. No commit is authorized by this adoption.
