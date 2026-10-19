# L0 — infra/system-world/fonts
Hash do Código: 35a9e117

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/watch-font-generation.toml sha256:0f2985807cc3c7b4aeed9a5a26b77ad66dbbdb67ccfb43ba8336dbc8e8eea1a5

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
