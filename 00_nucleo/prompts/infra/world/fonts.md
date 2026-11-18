# L0 — infra/system-world/fonts
Hash do Código: c8c0f338

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/immutable-metric-epoch.toml sha256:2ae956d6a0693c848353f4edfc70aea0888231adbb30b23765917b2bbb57d60e
- 00_nucleo/prompts/_nuclei/watch-font-generation.toml sha256:0132d0e50e81227203f577144758a78792358faee644dc236f74e2d6d8b80fa9



**Layer:** L3
**Exclusive consumer:** `03_infra/src/world/fonts.rs`

## Current evidence

- `03_infra/src/world/fonts.rs:23` — `with_fonts`.
- `03_infra/src/world/fonts.rs:34` — `with_system_fonts`.
- `03_infra/src/world/fonts.rs:46` — `with_embedded_fonts`.
- `03_infra/src/world/fonts.rs:65` — `with_fonts_and_system`.
- `03_infra/src/world/fonts.rs:121` — `preload_coverage_if_needed`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/world/fonts.rs`.

## Owned contract

- Own font builders, lazy exact coverage and conservative preloading. Invalid FontInfo entries are dropped while book/slot indices remain aligned; absent system fonts are nonfatal.
- Preserve builder order: embedded text before embedded math/code; combined discovery uses embedded text, system, embedded math/code, project sources.
- System and combined builders capture discovery policy and owned project paths without discovering fonts. The first book, font or candidate request initializes the complete paired catalog once, safely under concurrent access, and retains its Arc for the world lifetime. An explicitly installed validated font session may also retain its generation as specified below. Explicit slot and embedded-only builders retain their existing acquisition behavior. Replacing builder configuration replaces its catalog and clears coverage belonging to the previous indices.
- Deferred discovery still obtains exact system coverage through the fontdb owner while temporary face bytes are available. Candidate queries must not map all system FontSlots to reconstruct coverage; preserve the P1410 resource bound.
- Prefer nonempty book coverage without loading font bytes. Only missing coverage triggers lazy cmap parsing/cache by slot; invalid faces yield empty coverage. Preserve exact codepoint filtering, candidate order and locks.
- The existing explicit preload method remains conservative for Text/MathText/RawTrimmed through embedded coverage union and first-uncovered candidate demand; calling it may initialize the catalog. Normal CLI compilation leaves activation to actual font demand. Post-evaluation content follows the same demand path.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.


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

## Session builder contract

```rust
pub fn with_font_catalog_session(
    self,
    session: Arc<FontCatalogSession>,
    font_paths: &[PathBuf],
) -> Self;
```

This is this owner's only new public signature in that approved package. Capture
Arc/copied paths without I/O/discovery. First catalog demand delegates to session;
builder replacement clears coverage_cache and replaces OnceLock/closure like the
existing builders. Do not share coverage_cache: FontBook's precomputed coverage
belongs to the generation, missing coverage is rebuilt in the new World's map.
Old builders remain independent.

Use internal Arc storage without changing private font_catalog return or paired
indices/book. Order stays embedded text→system→embedded math/code→project. Expose
pub(super) helpers for fresh combined acquisition and assembling a catalog from
generation-prepared discovery, consuming already discovered project slots. This
is L3-descendant access, no public API. Do not duplicate metadata/coverage/selection.
`00_nucleo/prompts/infra/fontdb.md` owns book creation from Database;
`00_nucleo/prompts/infra/fonts.md` owns project-slot book creation. No phase change.
Inline builder tests check laziness/replacement/source independence without public
instrumentation.


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

Exclusive owning obligation: Own a private EmbeddedPrefix with one OnceLock holding paired embedded-text slots/book. No discovery in Default/new builders. Combined-system builder installs a fresh empty prefix; session builder shares session prefix. Explicit slots/embedded-only/system-only/replacement policies clear it. Selection, one-info lookup and bytes may use only this stable prefix; indices outside it and uncertified queries use complete catalog. Complete book remains unchanged. No changes to coverage/global fallback/preload behavior in this foundation.


## Ratified selective-font consumers — 2026-11-12T19:19:44.102371+00:00

Measurement before decision: structural/repeat2/receipt.json isolates about 13ms
of catalog freshness work, not a safe optimization. selection-proof/receipt.json
checks actual prefix metadata/bytes/indices and minimum-score winners; impact.json
identifies eager chain and late PDF requests. Source font_metrics/resolution.rs:95
and shaper/font_selection.rs:73 construct unused later families; shaping.rs:302,542
and pipeline/resources.rs:214 demand full book for individual selection/metadata.
The approved World query contract now has default-method GREEN; foundation RED
proved ordinary prefix selection unnecessarily invoked the full loader, and GREEN
removes that demand with stable paired immutable data. No new performance is yet
claimed. User explicitly authorizes this candidate only in RAM, no integration.

Selective queries preserve complete current selection/metadata/bytes. A private
first-primary lookup follows the unchanged ordered declared-family/readability
policy; if none resolves, use the existing class fallback then common list. It
is NOT a complete-chain result and never enters a PrimaryResolution cache.
Existing complete functions/caches remain the fallback and regression oracle.
No family-name/phrase/fixture/paragraph-count special case, negative memoization,
cache lifetime extension, new public method, dependency, unsafe, phase or mode.
This narrowly supersedes previous mandatory eager full-chain query order only
where the result is proven unchanged, not source font/fallback/script semantics.

For ordinary metrics that use only the first readable face (vertical/edge/cap
properties), query that face directly. For direct ordinary text advance, a first
face with nonzero cmap coverage of every authored character already wins the
existing per-character covering_text rule. Math/native scripts/uncertain coverage
keep the full route. This proof concerns the unchanged direct-metric operation;
it is not certification of a raw shaping result.

Shaping may use a one-face primary vector only when non-math, non-superscript,
non-subscript and not a resolved BidiSpan occurrence. Prove full authored-text
nonzero cmap coverage in the first readable face; then run the EXISTING bidi and
script segmentation with that sole candidate and the existing raw owner, actual
ordered axes, language, direction, script=None/features=[] policies. Require
nonempty raw results and no zero glyph. Actual emission/measurement uses the same
unchanged segmentation and raw inputs and can hit the exact current-compilation
raw store. Any failure/uncertainty uses the unchanged full primary chain before
producing output. No partial primary result is cached as complete. No adjusted
glyph/position/width/cache-key changes. This certificate covers each original
sub-run, not a whole-text shape used to bypass script segmentation. Terminal
measurement keeps its current output algorithm; using the raw store for the
additional ordinary certificate does not replace its existing terminal policy.

RED first: complete ordinary PDF compilation must not invoke a full-catalog
loader, including metrics, shaping, metadata and export-resource resolution.
Verify full path on uncertain/default/regex/mutable/missing/longest-run cases;
keep existing snowman, scripts, math, axes, raw identity, missing-face and cache
controls. A successful prefix followed by a fallback request must match a fresh
full result and not poison complete caches. Full source/font-update/PDF/SVG/raster
controls and paired release CPU/allocation measurements remain required.

Exclusive owning obligation: Add a private test-only whole-PDF counter control using a synthetic stdin World and a counted complete-catalog loader. The test must compile real ordinary text through the full PDF facade, not call query helpers alone. Existing foundation/refusal/builder/concurrency tests stay intact; add regex and session/new-World sharing controls.


## Laboratory first-face proof for control-only input, 2026-11-12

### Evidence before decision

`lab/results/construction/diagnostics/watch-residual-2026-11-12/fontdemand-v2-cpu-stacks/analysis.json` records catalog validation below advance_shaped/inline_allocation::Certificate::walk. The independent breakpoint in `loremlocal-catalog-trace2/{receipt.json,trace.log}` confirms the first full loader comes from that same path; the earlier breakpoint selected the watch-only loader during compile and did not fire. Each receipt binds UTC/source/ELF. `core/compiler/stdlib/structural/outline.rs:273` inserts a standalone U+2060 before the page; `shaper/font_selection.rs:474-481` sends the existing is_p1404_format_control class directly to candidate zero without cmap/global fallback. `shaper/shaping.rs:47-52` nevertheless requires cmap coverage before accepting the first face. Mixed text is DIFFERENT: best_covering_run compares actual cmap coverage and a later face can win a longer run across a control. This refutes extending the all-character proof by simply ignoring controls inside mixed text. Code evidence motivates a measured RED, not an unmeasured performance claim.

### Exclusive internal correction

Laboratory only, no integration; ADR-0127 continuous flow. Preserve current public signatures, phases, defaults, capture identity, font selection and authored Unicode. No new control category, cache/lifetime, dependency, unsafe, source rewrite, glyph filtering or special outline/fixture/family path.
Extend only the existing counted test World controls. Require zero full acquisitions for direct ordinary measurement of pinned control-only inputs, and a complete actual outline PDF over certified embedded text. Require full acquisition for a mixed letter/control input with first-face cmap refusal and for a nonminimum variant; do not assert that every mixed control always needs discovery in general. Preserve observed_source's complete loader, mutable suffix adversary, policies, existing tests and all productive code in this owner. Real release PDF/SVG/annotation/query/raster and font-update controls remain separate gates.


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

Exclusive owner obligation: Add an owned epoch token to private EmbeddedPrefix, initialized by Default. Own private world_font_query_epoch: return the prefix token only when font_catalog.get() is None and font_prefix is Some; otherwise None. Do not call either get_or_init, scan fonts, or initialize a catalog. Existing builders already clear/replace prefix and complete state; preserve them. Tests prove lazy prefix support, revocation on complete queries, builder replacement and old token ownership.

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
