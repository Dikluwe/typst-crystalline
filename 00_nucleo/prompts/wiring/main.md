# L0 — wiring/main
Hash do Código: f4a688ee

**Layer:** L4
**Exclusive consumer:** `04_wiring/src/main.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:e9d6623a18ffc35f36051b280d96a75a0e4c1bd1768361a3a50c98587c568aef
```

## Current evidence

- `04_wiring/src/main.rs` — private import of `typst_infra::watch::Published` (the historical local declaration is superseded below).
- `04_wiring/src/main.rs:27` — `main`.
- `04_wiring/src/main.rs:43` — `main_is_only_a_closed_command_dispatcher`.

These are local source anchors, not a new upstream parity measurement.

## Owned contract

- Own only binary composition, the closed exhaustive RunIntent dispatcher and the private import of the L3 Published carrier (Html string or Bundle files). Preserve all command arms and static delegation to private child handlers.
- Compose L2 intents, L3 operations and L1 values; no CLI parser, serializer, language policy, rendering, TLS, polling or watcher algorithm belongs here.
- Handler bodies, diagnostic drain and eviction belong to their respective owners. The import remains private in L4; the transient carrier is owned by 00_nucleo/prompts/infra/watch.md; no new public API, flag, default or phase is authorized.

## Child owners

- `00_nucleo/prompts/wiring/compile.md` owns only `04_wiring/src/compile.rs`.
- `00_nucleo/prompts/wiring/completions.md` owns only `04_wiring/src/completions.rs`.
- `00_nucleo/prompts/wiring/diagnostics.md` owns only `04_wiring/src/diagnostics.rs`.
- `00_nucleo/prompts/wiring/eval.md` owns only `04_wiring/src/eval.rs`.
- `00_nucleo/prompts/wiring/eviction.md` owns only `04_wiring/src/eviction.rs`.
- `00_nucleo/prompts/wiring/fonts.md` owns only `04_wiring/src/fonts.rs`.
- `00_nucleo/prompts/wiring/info.md` owns only `04_wiring/src/info.rs`.
- `00_nucleo/prompts/wiring/init.md` owns only `04_wiring/src/init.rs`.
- `00_nucleo/prompts/wiring/query.md` owns only `04_wiring/src/query.rs`.
- `00_nucleo/prompts/wiring/watch.md` owns only `04_wiring/src/watch.rs`.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve the existing public contracts, defaults and phases; documentation cleanup
does not authorize productive changes or approve archived proposals. Validate lineage with
`crystalline-lint --checks v5,v15,v26 .`.


## Publication carrier ownership and verification

### Evidence

`lab/results/construction/diagnostics/tekt-v12-2026-10-16/before.json.gz` retains
HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC, sources/L0/
hashes/status/diffstat. The hash-checked inventory reported 19 V12: one productive
enum at `04_wiring/src/main.rs:22` and 18 auxiliary test declarations. The private
Published enum had exactly Html(String) and Bundle(Vec<BundleFile>), produced at
compile.rs:90/99/363/539/541 and dispatched at watch.rs:103/104. The former owner
permitted this private L4 carrier; its transfer explicitly supersedes that local
ownership decision. This is Rust ownership/representation, not new language
parity or upstream intent evidence. The complete construction record remains in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/wiring/main.md`.

### Current responsibility and acceptance

Remove the local Published declaration and the BundleFile import used only by it;
privately import `typst_infra::watch::Published`. The root remains composition and
closed dispatch without a new local type/public L4 API. Child handlers may retain
super::Published. The carrier contract belongs exclusively to
`00_nucleo/prompts/infra/watch.md`.

Preserve real watch HTML/bundle HTTP/reload controls, success/error, no-serve/
no-reload and last valid snapshot, including
`p1485_watch_bundle_http_reload_e_preserva_snapshot_apos_erro` and
`p1485_watch_flags_no_reload_no_serve_e_porta_ocupada` in `04_wiring/tests/cli.rs`,
and L3 watch HTTP controls. Preserve existing compile HTML/bundle full-tree,
byte-exact assets, stdout/warnings/exit and no-publication-on-error expectations.
Require real V12 RED before transfer and zero productive V12 afterward; the diff
must show only enum transfer/import adjustments, never extension-based target
inference. Do not add tests merely mirroring owner-transfer syntax.

Validate build, workspace/all-target Clippy with -D warnings and
-D unfulfilled_lint_expectations, workspace suite, formatting, Rustdoc -D warnings
and four indexes, V1/V5/V15/V26/reverse lineage, SARIF and diff --check. Heavy runs
remain sequential under MemoryMax=12 GiB, MemorySwapMax=0 and CARGO_BUILD_JOBS=1.
No release publication is authorized by this transfer.
