# L0 — infra/system-world/world_impl
Hash do Código: e6766ab5

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/immutable-metric-epoch.toml sha256:2ae956d6a0693c848353f4edfc70aea0888231adbb30b23765917b2bbb57d60e
- 00_nucleo/prompts/_nuclei/lab-legacy-physical-certificate.toml sha256:286d3ec27c85463e62b086abd2df51219197c0502164bac70a631898d7490868


**Layer:** L3
**Exclusive consumer:** `03_infra/src/world/world_impl.rs`



## Current evidence

- `03_infra/src/world/world_impl.rs:19` — `library`.
- `03_infra/src/world/world_impl.rs:23` — `book`.
- `03_infra/src/world/world_impl.rs:27` — `main`.
- `03_infra/src/world/world_impl.rs:31` — `source`.
- `03_infra/src/world/world_impl.rs:35` — `file`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/world/world_impl.rs`.

## Owned contract

- Own the sole `impl World for SystemWorld` adapter. Delegate cache/font/path/package behavior to the corresponding siblings; obtain book through the font child once-initialized catalog, return facade library/main and clone SysInputs/optional plugin Arc.
- Preserve `00_nucleo/prompts/core/contracts/world.md` signatures and public SystemWorld paths. Do not add a trait/vtable or export private descendants.
- today(None) uses local date; today(Some(offset)) uses UTC plus the exact converted duration. Overflow, clock or local-offset failure returns None without panic. Do not change the default or leak clock access into L1.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Pending selective-query adapter

Draft only, contingent on the explicit public-contract ratification in
00_nucleo/prompts/core/contracts/world.md. Override only select_font and font_info
with that exact signature/semantics, delegating acquisition and immutable-prefix
proof to the independently specified font child. book remains the complete paired
catalog. No source/path/date/plugin behavior changes or inline discovery policy.


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

Exclusive owning obligation: Delegate the two ratified methods to the fonts descendant selective selection/info helpers. Keep book, font, coverage and every other World contract; font delegates its now prefix-aware private helper. No new public methods beyond the two ratified trait additions.


## Laboratory immutable metric epoch — 2026-11-13

Evidence before decision: physical-retain/conclusion.json and its exact release
freeze record six-to-three physical execution reduction but only about5% CPU gain
on the outline. physical-retain-deep-stacks/structural-summary.json measures
11.5% inclusive samples in Journal::validates, with additional metric request
recording/style copying. These are execution mechanics, not language semantics.
Source world/fonts.rs:168-216 routes every query beyond the immutable prefix
through font_catalog(); world.rs stores the full catalog in OnceLock. FontSlot
get at fonts.rs:179-216 keeps embedded bytes in owned Vec and disk fonts may be
Mmap. Fallback metrics and transitive shaper queries use only the font-facing
World methods. Therefore a still-uninitialized full catalog plus the owned
embedded-prefix identity can certify this restricted domain; full-catalog or
arbitrary provider identity alone cannot. Runtime tests must refute expiry,
provider replacement and a falsely complete journal. No performance gain is
assumed. The user explicitly waives approval stops for laboratory work; this
additive experiment supersedes prior no-new-API clauses only for named methods.
Integration remains excluded. Follow the pinned immutable-metric-epoch nucleus.

Exclusive owner obligation: Delegate only the new World::font_query_epoch to the fonts descendant. Preserve every other World operation.

## Laboratory immutable source authority — 2026-11-15

Evidence before decision: the rejected legacy-certificate report and World
forwarding audit bind its ELF to 16/19 forwarded methods. markup_stream.rs:121-134
calls World.source to recover the current content origin. Source is immutable
Arc<SourceInner> (source.rs:28-50); reparse produces another immutable revision.
Source Eq only compares id/hash and is not revision authority. SystemWorld
source slots cache the first result in OnceLock (world/cache.rs:37-96), allocate
monotonic ids and never replace a successfully published slot (cache.rs:98-174).
Generic World has no source stability promise. These are mechanics observations;
language acceptance remains text/content morphology/geometry/diagnostics.

Authorization/classification: user explicitly requested correcting World and
testing immutable source certification in the laboratory, without integration
or commits. The Source owner records the earlier user instruction "Para o
laboratório não precisa autorização". This ratifies the following laboratory
capability addition only; no productive contract or phase/default is changed.

Exact laboratory public trait addition:
    fn immutable_source(&self, id: FileId) -> Option<Source> { None }

Some(source) is an explicit authority promise: source has the requested id,
is an owned immutable revision, and every subsequent source(id) query on THIS
World during its lifetime returns that same successful revision without new
observable source-query effects. No guarantee is inferred from bytes, ids,
hashes, root addresses or ordinary source() success. None is conservative and
permits arbitrary World implementations. Never override another World's source
answer or cache file/date/path/plugin results. SystemWorld may return its
successful OnceLock-backed world_source(id); the pipeline asks only after a
successful actual source(id) query. New Worlds have independent source slots.

LegacyWorldGuard must explicitly forward ALL trait methods, including font
epoch/select/info and immutable_source, preserving owned/reference identity and
overrides. The source method first delegates exactly once and returns its exact
result. Permit only the compilation input id, the exact immutable SourceInner
revision kept alive by the borrowed input, and a matching explicit authority
from the SAME World. Root-address comparison is provenance under that authority,
never a standalone semantic key. Different revisions even with same bytes/id,
other ids/imports, errors, missing authority, date/file/path/plugin/inputs reject.
Record whether source was observed in each retained execution; recheck its
authority before candidate reuse. Replay uses the identical guard. No extra
ordinary source() calls merely to validate, and no queries in unguarded routes.

All existing complete-document pagination/Location/read/structural proofs,
first-layout/snapshot requirements, budgets, warning/error policies and local
certificate lifetimes remain. No Unicode cache, cross-compilation context
result cache or selected/HTML/Bundle changes. Two manifested lab builds may
toggle a private constant CERTIFY_IMMUTABLE_SOURCE: false is forwarding-only,
true enables this capability. This is no product mode/flag/default.

RED before implementation: SystemWorld markup totals in plain/Align/Pad retain
correct text but must drop physical compositions from3 to2; sentinel World
font overrides must survive wrapping. Negatives: generic World without authority,
fresh equal-text revision, mutable external reads/date and imports retain fallback.
Preserve existing tests; run full pipeline tests and lint, permanent regressions,
warm/cold/export/edit controls and two balanced paired nominal campaigns versus
prior Block/Pad/Align and forwarding-only, with an identical-ELF A/A alias.
Record CPU/RSS/HWM and compare all states, including references and context scale.

The exact capability in the latest section supersedes the prior prototype's blanket source rejection and no-trait-change statements, solely in this laboratory tree. Product integration remains excluded.

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
