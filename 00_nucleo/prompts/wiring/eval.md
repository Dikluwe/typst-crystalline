# L0 — wiring/eval
Hash do Código: 70f3bb17

**Layer:** L4
**Exclusive consumer:** `04_wiring/src/eval.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:09b32666fb1b1fa7ba8f2ef7c979c60cbc64cc515b8d9c46818bec19d7471fb1
- 00_nucleo/prompts/_nuclei/expression-source-identity.toml sha256:a54eea6def2bac71520060ddf084fc434983bb1ca20eb59dde942a0778a130cb
- 00_nucleo/prompts/_nuclei/network/custom-ca-cert.toml sha256:4b9f69ff41790a55a5b2d6a7b9982da858092e4bb2f0c503d6332a14a510e3ec
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:e9d6623a18ffc35f36051b280d96a75a0e4c1bd1768361a3a50c98587c568aef
```

## Current evidence

- `04_wiring/src/eval.rs:16` — `run_eval`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/src/eval.rs`.

## Owned contract

- Own run_eval composition. No input uses minimal World; `-` uses stdin/transient Source; physical input uses current root/main/font/CA setup. Materialize the expression Source for accurate diagnostics.
- With a document call contextual L3 evaluation, otherwise isolated features/sink evaluation. Error diagnostics drain before warnings on Err; on Ok warnings precede L2 serialization/stdout. Preserve each diagnostic once and its origin.
- Semantic/serialization failures exit 1, input/output/argument failures exit 2 and success exits 0. Do not unify this path with query or move evaluation/serialization into wiring.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Document and expression identity

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

### Contract and acceptance boundary

Compose for_stdin for a stdin document and source_for_expression for every eval
expression mode, using the Source-based L3 facades. L4 must not parse expressions,
allocate ids or implement eval. A Source-factory failure is acquisition failure,
exit 2 with an explicit message.

The formatter's transient primary source is exclusively the expression, displayed
as `<input-expression>`. World resolves physical document, stdin, imports and
callbacks by distinct ids. Preserve Source and bytes throughout execution and
diagnostic drain; never remap all errors through input_path. Preserve message
order, hints/traces, exit codes and format/pretty. This package does not modify
query, compile, stdin outside this composition, flags or phases.

For stdin, supply absolute cwd acquired through `std::env::current_dir`: diagnostic
name remains `<stdin>` and the physical import/read base remains cwd. Acquisition
failure exits 2; do not guess another root. Preserve exclusive ownership; the code
hash follows materialization. Source identity/presentation controls and existing
eval diagnostics remain the acceptance boundary.


## Approved diagnostic resource failure composition — approved 2026-10-22

Integrate the diagnostics owner's private fallible resource preparation before
successful serialization/stdout. Preserve original expression/physical/primary
identities, all original errors and warnings, source acquisition and ordering.
Diagnostic-path failure is operational exit 2, retaining original diagnostics
and reporting the real acquisition error without successful stdout. Healthy
semantics, warnings, serialization and statuses stay identical. No new public
signature, flag, source identity or evaluation phase is introduced.


Approval of this exact metadata correction is retained in `lab/results/world-metadata-errors-2026-10-22/approval.json`, review SHA-256 `31d632a54d40ca7df4139081c5297af774970a93b08cd605986b53e75e0ccc62`. The approved clauses supersede prior infallible signature/return preservation only for the two public APIs and the described private composition interfaces.

Replace the preloaded-stdin Source expect with the existing source-acquisition Result/exit2 handling; construction failures are not assumed impossible. This is the already specified Source failure channel, not a new public signature.


## Checked stderr composition — authorized 2026-10-22

### Measured evidence

`lab/results/stderr-watch-errors-2026-10-22/agents/measurement/baseline.json` (SHA-256
`9c6da7587761dd242cddf07abcf512169ad45d4bde63e3040dc1577e02e67b15`)
retains exact HEAD, uncommitted source/archive, UTC, dirty stat and binary identity.
`lab/results/stderr-watch-errors-2026-10-22/agents/measurement/before/receipt.json` (SHA-256
`e49aa4458f4cb413995af35f8297ecf4fcafd20141cbdab41067fc7ed5b3895c`)
retains the real failed-sink and healthy controls before this correction. The
source map is `lab/results/stderr-watch-errors-2026-10-22/agents/stderr/audit.md`.
These observations precede the narrowly owned correction below; a panic is not
required behavior or evidence of upstream intent.

### Owned correction and acceptance

Change only private run_eval composition to `io::Result<ExitCode>`. Send each
existing input, original error/warning, metadata and serialization/output error
text through the diagnostics owner's checked writer and propagate actual stderr
write/flush errors with `?`. Preserve error-before-warning order, warning-before-
serialization/stdout order and ordinary exits, identities and feature dispatch.
Empty original diagnostic text does not touch stderr. Failure to emit before
stdout prevents that later stdout operation. Do not alter serialization, source
acquisition or diagnostic resource semantics.

The existing pinned CLI-observables Nucleus governs shared output/exit obligations;
this owner does not redefine it. The user's explicit request authorizes replacing
stderr panic with exceptional operational exit 2. This changes private composition
only, without public signature/flag/default mode or eval/layout phase change
(ADR-0127). Retain healthy message bytes/order and all existing alerts/contracts.
Require measured RED→GREEN, healthy parity, tests, hashes and direct/reverse lineage;
no suppression or weakened assertion is authorized.
