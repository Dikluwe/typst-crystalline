# L0 — infra/system-world/cache
Hash do Código: 5dbfb07a

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/expression-source-identity.toml sha256:a54eea6def2bac71520060ddf084fc434983bb1ca20eb59dde942a0778a130cb
- 00_nucleo/prompts/_nuclei/watch-source-generation.toml sha256:c7f9b3d18b112b1d725e082b7da75b886bf7120346ffbca911c236ea65c52986

**Layer:** L3
**Exclusive consumer:** `03_infra/src/world/cache.rs`

## Current evidence

- `03_infra/src/world/cache.rs:20` — `SourceSlot`.
- `03_infra/src/world/cache.rs:27` — `new`.
- `03_infra/src/world/cache.rs:31` — `synthetic`.
- `03_infra/src/world/cache.rs:40` — `get`.
- `03_infra/src/world/cache.rs:58` — `register_file`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/world/cache.rs`.

## Owned contract

- Own SourceSlot/OnceLock, file registration, dependency enumeration and byte/source cache helpers; per-world storage remains in the facade.
- Synthetic sources are already loaded; physical sources load lazily except main. OnceLock memoizes the first Source result, including errors. Canonicalize registration when possible, otherwise retain the original path; allocation follows the checked identity contract below. Registration must atomically associate one canonical path with its FileId and initialized slot under concurrent calls; keep the existing fast lookup and serialize the publication under next_id → path_to_id → slots, rechecking identity after acquiring the allocation lock.
- Successful byte reads share Arc by canonical FileId across path/id access. Do not cache failed byte reads, so they may retry; failed Source reads remain memoized. Preserve FileError/Bytes conversion behavior.
- Dependencies are sorted and unique and include main, sources, assets and failed attempts, not system fonts. Equivalent physical paths share identity without an independent cache.

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

## Expression Source factory and checked identity

```rust
pub fn source_for_expression(&self, text: String) -> FileResult<Source>;
```

Allocate a new id under next_id, build one Source::new_with_parser using parse_code,
publish preloaded slots entry at root/<input-expression> and UTF-8 bytes in
read_cache. Return the same World::source-resolvable Source; World::file reads
RAM. Never register/canonicalize/read the marker in path_to_id/on disk. Distinct
calls have distinct ids even for equal text. Stdin/empty main also preload bytes;
private SourceSlot helpers may accept a prepared Source without public Source API
changes.

The approved extreme-exhaustion behavior supersedes id wrapping/reuse. One checked
physical/transient counter retains NonZeroU16 FileId. source_for_expression returns
FileError::Other("FileId counter exhausted") before publishing any slot.
register_file keeps signature/known-id lookup even exhausted; new physical identity
without capacity panics exactly "FileId counter exhausted", never overwriting old
Source/caches. Preserve fast lookup and physical next_id→path_to_id→slots order;
transients next_id→read_cache→slots. No path takes next_id after another such lock.
Detect exhaustion/drop guards before panic, avoiding lock poisoning in tests.
Physical dependencies/Source error+success memoization retain prior contract.

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

## Physical syntax-session boundary

Physical SourceSlot may carry private optional Arc session. After ordinary
read_to_string pass full canonical path/id/text to
`00_nucleo/prompts/infra/world/source_session.md`; absent session uses existing
Source::new. Register ids/dependencies normally; each World's OnceLock keeps its
own snapshot/error and no read error enters session. Synthetic/from_source/code/
expression stay outside cross-World cache. Keep private interfaces where possible
to avoid external repairs.
