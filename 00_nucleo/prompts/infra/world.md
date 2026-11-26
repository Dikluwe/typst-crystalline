# L0 — infra/world
Hash do Código: 9df0a33c

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/expression-source-identity.toml sha256:a54eea6def2bac71520060ddf084fc434983bb1ca20eb59dde942a0778a130cb
- 00_nucleo/prompts/_nuclei/watch-font-generation.toml sha256:0132d0e50e81227203f577144758a78792358faee644dc236f74e2d6d8b80fa9
- 00_nucleo/prompts/_nuclei/watch-source-generation.toml sha256:c7f9b3d18b112b1d725e082b7da75b886bf7120346ffbca911c236ea65c52986




**Layer:** L3
**Exclusive consumer:** `03_infra/src/world.rs`

## Current evidence

- `03_infra/src/world.rs:35` — `SystemWorldError`.
- `03_infra/src/world.rs:43` — `fmt`.
- `03_infra/src/world.rs:58` — `SystemWorld`.
- `03_infra/src/world.rs:97` — `for_eval`.
- `03_infra/src/world.rs:132` — `new`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/world.rs`.

## Owned contract

- Own SystemWorld state, SystemWorldError/Display, new/for_eval and input/plugin builders only. Cache, fonts, paths, packages and World implementation have independent descendant owners.
- new accepts the existing root/main inputs, canonicalizes the physical main, eagerly reads it, registers FileId 1 and starts subsequent ids at 2. Preserve main-not-found and I/O errors.
- for_eval creates a preloaded synthetic source at root/<input-expression> without file reads; imports still resolve against the supplied root.
- Initialize the current per-world maps/locks, library, empty SysInputs, absent plugin host and default package downloader. Font state consists of a once-initialized Arc of a paired catalog and a private discovery closure returning that Arc; an unconfigured world has an empty catalog. The font child owns discovery and replacement. with_inputs stores strings and with_plugin_host stores the supplied Arc; no I/O enters the L1 contracts.
- Preserve public paths/builders and private descendant access. Do not add a new trait or global cache. The ratified session extension below is the only new export; child-to-child helpers remain private to L3 descendants.

## Child owners

- `00_nucleo/prompts/infra/world/cache.md` owns only `03_infra/src/world/cache.rs`.
- `00_nucleo/prompts/infra/world/fonts.md` owns only `03_infra/src/world/fonts.rs`.
- `00_nucleo/prompts/infra/world/packages.md` owns only `03_infra/src/world/packages.rs`.
- `00_nucleo/prompts/infra/world/paths.md` owns only `03_infra/src/world/paths.rs`.
- `00_nucleo/prompts/infra/world/tests.md` owns only `03_infra/src/world/tests.rs`.
- `00_nucleo/prompts/infra/world/world_impl.md` owns only `03_infra/src/world/world_impl.rs`.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Source identity evidence and approval

### Evidence and approval

`lab/results/construction/diagnostics/diagnostic-paths-2026-10-06/identity-next-audit.json`
retains commands/sources/UTC/HEAD/changed tree and Tekt/vanilla binary hashes at
historical pin `9f2b6e8715237cb086899a42873660fe744622e8`. Separate document
error/warning and expression-error controls showed the document losing its source
and the expression receiving the document's name. `compiler/eval/mod.rs:609–615`
used world.main() for Source/base; `wiring/eval.rs:66–95` repeated that id for the
formatter's primary source. `world/cache.rs:58–78` registers files/recycles the
counter; upstream `typst-cli/src/eval.rs:159–179` and `world.rs:176–190` distinguish
expression, stdin and absent-input identity. Integer equality/global upstream
interning is mechanics, while displayed origin/source/position/relative resolution
are language observables (ADR-0107). No new comparison with the current pin is
claimed.

Human `confirmo` on 2026-10-06 approved ADR-0127 gate, recorded in
`lab/history/construction/records/00_nucleo/materialization/origem-fontes-expressao-aprovacao-2026-10-06.json`.

## Ratified transient-source constructor

```rust
pub fn for_stdin(root: impl Into<PathBuf>, text: String) -> Self;
```

Create main FileId 1 with preloaded markup Source at root/<stdin>, matching UTF-8
bytes and the same initial configuration as for_eval. Caller supplies text: do
not read stdin. Imports/read use supplied root and world.source(main()) is the
true stdin document. No transient entry in path_to_id. for_eval keeps signature/
base/empty Source but no longer registers its marker as a physical file, permitting
a real same-named file without Source aliasing.

Private next_id can represent exhaustion beyond 65535, initial 2/main 1, maps per
World. Cache owner allocates/publishes, no globals. This constructor alone neither
enables compile stdin nor changes flags/defaults.

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

## Exclusive font-session state and export

Declare font_session and reexport FontCatalogSession at
`typst_infra::world::FontCatalogSession`; definition/methods belong only to
`00_nucleo/prompts/infra/world/font_session.md`. font_catalog is
OnceLock<Arc<(Vec<FontSlot>, FontBook)>>, font_loader returns that Arc. Constructors
remain empty without font acquisition. Sources/bytes/maps/counter/inputs/plugins/
downloader/coverage_cache keep per-World initialization/lifetime. No reset/new
World trait method/identity change; session builder belongs to world/fonts owner.

## Source reuse measurement and acceptance

`lab/results/construction/diagnostics/incremental-prioridade6-2026-10-08/{before.json.gz,measurement.json.gz,control-measurement.json.gz,count.json}`
retains tree/HEAD/UTC/commands/releases. Source::new rebuilds CST; fresh World
rebuilds main/import Source per edit. Minimal import also consumes eval_ms, so it
is not purely parsing cost. Existing owners define causal Source/id/path identity;
reuse mechanics must preserve language observations. Cost/diagnostic/effect/stale
source/geometry/identity divergence refutes benefit; no upstream intent inferred.

Require real RED→GREEN, production counts apart from timing, three paired CLI
rounds against archived prior release, startup/RSS and invalidation/identity/effects/
Unicode/diagnostic/assets/fonts/geometry controls. Owner-only resealing, relevant
build/suites/lint zero errors; no full-incrementality completion claim.

## Ratified source-session composition

Export SourceSession and provide
new_with_source_session(root: impl Into<PathBuf>, main_path: impl AsRef<Path>,
session: Arc<SourceSession>) -> Result<Self, SystemWorldError>.
Existing new/transients retain signature/identity/errors. Configure session before
first eager read, load main normally, retain only a private optional Arc and pass
it into physical slots. All remaining World services are fresh. Do not merge
FontCatalogSession into SourceSession or alter its contract. Child definition is
`00_nucleo/prompts/infra/world/source_session.md`, not shared productive ownership.

## Pending selective-query scope

Draft only: the public gate in core/contracts/world.md proposes select_font and
font_info. If ratified, private facade state may distinguish an immutable leading
text-font prefix from the complete current catalog; constructor and source state
remain per World. This narrowly supersedes the earlier no-new-World-method clause.
The font child must receive a current owner specification before implementation.


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

Exclusive owning obligation: Own a private optional Arc to the fonts descendant immutable prefix. Empty/transient/source constructors start without this policy. No new public field or constructor behavior; all prefix acquisition remains lazy in descendants.

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

## Paths package

`SystemWorld::new` canonicalizes the root, so a relative or symlinked root and a relative input resolve the same files as absolute ones. A missing root reports `root directory not found (searched at <root as given>)`, and a main file outside the canonical root reports `source file must be contained in project root`. The transient eval and stdin worlds canonicalize their root when it exists.

Approved on 2026-11-26 (“Aprovo A–C”): `00_nucleo/diagnostics/paths-package-2026-11-26.pt-BR.md`.
