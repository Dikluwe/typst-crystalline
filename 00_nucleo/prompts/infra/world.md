# L0 — infra/world
Hash do Código: 08bb2e65

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/expression-source-identity.toml sha256:a54eea6def2bac71520060ddf084fc434983bb1ca20eb59dde942a0778a130cb
- 00_nucleo/prompts/_nuclei/watch-font-generation.toml sha256:0f2985807cc3c7b4aeed9a5a26b77ad66dbbdb67ccfb43ba8336dbc8e8eea1a5
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
