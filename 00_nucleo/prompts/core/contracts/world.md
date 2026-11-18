# L0 — Compilation environment contract
Hash do Código: f3dcbaa9

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/immutable-metric-epoch.toml sha256:2ae956d6a0693c848353f4edfc70aea0888231adbb30b23765917b2bbb57d60e
- 00_nucleo/prompts/_nuclei/lab-legacy-physical-certificate.toml sha256:286d3ec27c85463e62b086abd2df51219197c0502164bac70a631898d7490868


**Layer:** L1
**Exclusive consumer:** `01_core/src/contracts/world.rs`



## Evidence

`01_core/src/contracts/world.rs` declares the pure `World: Send + Sync` trait,
rooted-path methods and a duration-based `today`. Retained upstream evidence:
`lab/typst-original/crates/typst-library/src/lib.rs:92-99` for duration offsets,
and `foundations/path.rs:186-214` under that tree for captured path resolution.
These historical coordinates support the decisions, not fresh parity claims.

## Boundary and public interface

`World` declares what the pure core needs; L3 obtains it from disk, network,
memory or mocks. L1 never performs direct system I/O. This owner defines
`World`, `SysInputs` and local mock tests, not the facade or tracking adapter.

```rust
pub type SysInputs = IndexMap<EcoString, EcoString, FxBuildHasher>;
pub trait World: Send + Sync {
    fn library(&self) -> &Library;
    fn book(&self) -> &FontBook;
    fn main(&self) -> FileId;
    fn source(&self, id: FileId) -> FileResult<Source>;
    fn file(&self, id: FileId) -> FileResult<Bytes>;
    fn resolve_path(&self, current_file: FileId, path: &str) -> Result<RootedPath, String>;
    fn read_path(&self, path: &RootedPath) -> Result<Arc<Vec<u8>>, String>;
    fn include_path(&self, path: &RootedPath) -> Result<Source, String>;
    fn read_bytes(&self, current_file: FileId, path: &str) -> Result<Arc<Vec<u8>>, String>;
    fn include_source(&self, current_file: FileId, path: &str) -> Result<Source, String>;
    fn resolve_package(&self, spec: &PackageSpec) -> Result<Source, String>;
    fn inputs(&self) -> SysInputs;
    fn plugin_host(&self) -> Option<Arc<dyn PluginHost>>;
    fn font(&self, index: usize) -> Option<Font>;
    fn candidates_for_char(&self, c: char) -> Vec<usize>;
    fn today(&self, offset: Option<Duration>) -> Option<Datetime>;
}
```

The excerpt omits default bodies, not their obligations:

- `source`/`file` may fail; callers must handle `FileResult`. Unavailable font
  slots return `None`; font consumers retain fallback.
- `resolve_path` performs no I/O and captures `VirtualRoot` and the originating
  file's base. Absolute paths resolve within the current project/package root;
  relative strings resolve against the originating directory. `Value::Path`
  supplies its captured identity instead of being rebound in another consumer.
- `read_path`/`include_path` delegate materialization to L3, which validates root
  containment and performs I/O/cache operations. Their defaults, and the default
  resolution method, report unsupported operations so mocks need no filesystem.
- Legacy `read_bytes`/`include_source` retain two-argument signatures and
  unsupported-operation error defaults. Supporting compatibility adapters must
  compose rooted resolution plus the corresponding rooted operation; they must
  not become a parallel normalization authority.
- `resolve_package` returns the entrypoint source or an error; the default
  reports unsupported package resolution. Cache lookup, manifests and physical
  directory conventions belong to L3.
- `inputs` defaults to an empty insertion-ordered map. Values remain strings:
  `--input n=42` exposes `"42"`, not an integer.
- `plugin_host` defaults to `None`; the capability interface belongs to
  `00_nucleo/prompts/core/contracts/plugin_host.md`.
- `candidates_for_char` collects `FontBook` candidates by default. Coverage is
  approximate by blocks of 256 codepoints; L3 confirms actual glyph availability.
  Overrides may calculate coverage lazily.
- `today(None)` requests the local date; `Some(offset)` requests UTC plus the
  exact duration, including sub-hour precision. `None` as the result denotes
  unavailable date information.

Preserve existing diagnostic strings, optional-service defaults, thread bounds,
pure mocks and error propagation. ADR-0005 separates this interface from tracking;
this prompt does not declare `TrackedWorld` or prescribe an unsupported
`TrackedWorld: World` supertrait. Contract/code discrepancies belong to
`lab/results/construction/diagnostics/sanitization-entities-contracts-history.md`, not to
silent weakening of obligations or public API changes.

## Pending laboratory proposal: selective font queries — 2026-11-12T15:39:03.171981+00:00

Status: DRAFT, awaiting the ADR-0127 public-contract gate. No code or integration
is authorized by saving this proposal. Existing Hash do Código identifies the
unchanged preimplementation consumer, not an implementation of this draft.

### Measurement before decision

`lab/results/construction/diagnostics/watch-residual-2026-11-12/structural/analysis.json` binds six-process CPU/resource observations to
`build-receipt.json`, `probe-receipt.json` and the integrated source freeze.
On layout.typ, median full-operation CPU was 34.124 ms; retaining a real font
World while deliberately bypassing later freshness checks gave 19.798 ms.
Preloading the full current book separately cost 14.093 ms. The twelve-pair
repeat2/receipt.json reproduced 30.841 ms full and 17.803 ms resident, paired
median -13.066 ms (12/12 favorable), with preload total 30.894 ms and paired
total delta +0.010 ms. Therefore moving validation alone does not remove its cost. Two full-process
outliers and all raw values are retained: this is a causal ablation, never a safe
candidate or a guaranteed speedup. Native phase elapsed clocks are not CPU.
`lab/results/construction/diagnostics/watch-residual-2026-11-12/structural/fontproof/receipt.json` independently extracts the exact generation
functions from the same source: full unchanged proof 14.343 ms, including
55 configuration files, 1345 root/link/candidate stamps and 1112 font stamps.
`lab/results/construction/diagnostics/watch-residual-2026-11-12/structural/pdf-operators.json` identifies LibertinusSerif-Regular in this fixture.

Source: `03_infra/src/world/font_session.rs:66` requires full fast/key validation;
`03_infra/src/world/fonts.rs:79` initializes the complete paired catalog on demand;
`03_infra/src/font_metrics/resolution.rs:56,95` and
`03_infra/src/shaper/font_selection.rs:111` ask for the whole book to select a
family, and eagerly prepare later fallback families. `01_core/src/entities/
font_book.rs:357,452` defines selection ranking and first-minimum tie behavior.
Embedded text precedes mutable sources at `03_infra/src/world/fonts.rs:200`.

Selection/text/styles/diagnostics/geometry are language observations. Catalog
traversals, slot numbers and cache lifetimes are mechanics. Inference: selective
queries can avoid mutable discovery when an immutable leading face is already
provably the winner. A changed winning face, fallback behavior, stale system font,
extra eager book demand or absence of robust end-to-end gain refutes this route.

### Proposed decision and bounds

Laboratory only. Add the following two default methods to World; no method is
removed or made newly mandatory for implementations:

```rust
fn select_font(&self, pattern: &FontNamePattern, variant: &FontVariant) -> Option<usize>;
fn font_info(&self, index: usize) -> Option<&FontInfo>;
```

Their default bodies delegate exactly to `self.book().select_pattern(pattern,
variant)` and `self.book().infos().get(index)`. This keeps existing implementors,
mocks and their full-catalog semantics. Types come from existing pure L1 entities.
No discovery, clock, filesystem access, new entity or mutable global enters L1.

An override must return the same selected font/metadata as the complete current
catalog would for that query. The index belongs to this World and must address
the same face through font(index), font_info(index), and a later complete book().
book() remains the full catalog; candidates_for_char retains its existing full
candidate semantics. No partial book, speculative None, stale mutable catalog,
cross-World index, or retained mutable font bytes are authorized.

The first SystemWorld experiment may serve only an immutable embedded-text
prefix of the existing combined policy without mutable discovery. Certify an
unbeatable first-ranked candidate, including current family/pattern/variant rules;
being embedded or having the same family is insufficient. Begin conservatively
with literal names and an exact minimum score; regex, absent families, nonminimum
variants, system-only policy and uncertain cases use complete fresh discovery.
Embedded math/code is NOT a prefix: system sources precede it. Do not reorder it.
Builder replacement must reset all private prefix/complete state consistently.
Immutable assets may be shared with a bounded session, never through a global.

Internal consumer work after ratification requires its own current L0 before
code. Metrics/shaper/resource consumers must use selective queries where they
only need selection/one record, and defer unused later families without changing
the winner. The initial fast route is ordinary text only; math, global fallback,
uncertain coverage/shaping, partial failures and unsupported cases use the original
complete path. Successful cmap lookup alone is NOT proof of successful shaping.
Keep all original script/bidi segmentation, missing-glyph recovery, fallback order,
axes and font bytes. Partial resolutions may not enter existing complete-chain
caches. No phrase/fixture/paragraph-count recognition is allowed.

This supersedes prior API-preservation wording solely for these two methods and
their SystemWorld overrides. All other public contracts, flags, defaults and phase
boundaries remain outside this proposal. No cross-compilation shaping/layout/PDF
cache, event-only freshness or oracle stale-font behavior is authorized.

### Acceptance before any proposed integration

Tests first: default-query results on existing mock Worlds; exact embedded winner
without mutable discovery; a better system variant and earlier family forcing the
fresh path; regex, missing/failed face, math/code, mixed scripts and fallback;
prefix indices/metadata/bytes stable before and after complete-book acquisition;
builder replacement, concurrent requests and new Worlds. Preserve current cache
identity/axes controls. A partial hit must not poison complete-chain caches.

Repeat add/remove/in-place/atomic replacement/restored-mtime/symlink/configuration
controls for fonts genuinely demanded in each edited document, including a later
edit that first requests a previously unused mutable font. Retain source/effect/
diagnostic, text/morphology/style, page/glyph geometry and rendered PDF controls.
Mechanical counters demonstrate the intended absence of unrelated catalog work;
they do not replace language acceptance. Use pinned vanilla only for language
comparisons, not for reproducing its observed stale-font defect.

Before writing each implementation, audit/update that consumer's L0, validate
V15/V26, reseal owner hashes and run RED→GREEN. Complete relevant core/infra tests,
build and crystalline-lint with zero violations, then matched-recipe paired CLI
CPU/startup/memory/resource measurements against the integrated base and pinned
vanilla. Do not stack unpromoted candidates or claim the 14 ms ablation as measured
candidate gain. Integration still requires separate user authorization.


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

Exclusive owning obligation: Own only the two approved default query methods and pure mock tests. Import existing FontNamePattern, FontVariant and FontInfo. Defaults are precisely book().select_pattern and book().infos().get, preserving old implementors. No tracking or I/O in this contract.


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

Exclusive owner obligation: Own only optional font_query_epoch(&self)->Option<std::sync::Arc<()>>, default None. Document the temporal certificate in the consumed nucleus; existing implementors remain uncertified and all other methods preserve their behavior.


## Laboratory complete shaping products — 2026-11-13

### Evidence before decision

The frozen fraction-demand release (f34f54de100fc63587f7611d042dc3d836b484f95283a2f5fc029c8afadce65e)
and fraction-demand-work-shift/causal-summary.json record 112 backend executions
per100-paragraph compilation and64 peroutline, unchanged by removing2004/2172
width queries. All three cycle kinds repeat this work. structural-summary.json
attributes18.39% inclusive samples to emitted shaping and5.31% to raw shaping
in100 paragraphs; these groups overlap and are not a speedup prediction.
font_metrics.rs:100-106 shares only raw buffers within a compilation;
shaper/traversal.rs:30-49 reconstructs faces/selection/output caches;
shaping.rs:262-346 rebuilds measured widths and:363-679 rebuilds emitted products.
world/fonts.rs:21,35-39,291 retains one immutable prefix owner across a font
session and revokes before full-catalog exposure. Current PdfCompileSession
at pipeline/pdf.rs:1133 has no shaping products. These are execution mechanics,
not a change to language semantics. The user explicitly authorizes laboratory
implementation/testing without further stops, including experimental APIs;
integration remains prohibited. No performance benefit is presumed.

### Exclusive responsibility

Clarify that a shared font_query_epoch token promises stable equal font-query answers and immutable owners across World instances, not merely immutable storage somewhere. Default None and every method behavior remain unchanged; SystemWorld shared prefix already satisfies this stronger explicit guarantee.

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
