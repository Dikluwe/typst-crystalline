# L0 — wiring/info
Hash do Código: 32acff47

**Layer:** L4
**Exclusive consumer:** `04_wiring/src/info.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/network/custom-ca-cert.toml sha256:4b9f69ff41790a55a5b2d6a7b9982da858092e4bb2f0c503d6332a14a510e3ec
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:e9d6623a18ffc35f36051b280d96a75a0e4c1bd1768361a3a50c98587c568aef
```

## Current evidence

- `04_wiring/src/info.rs:11` — `run_info`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/src/info.rs`.

### Effective feature evidence

At clean HEAD `3548bf20f95f27ad467dd1a592851bfcd0af9232`, the
[paired CLI probe](../../diagnostics/prompt-review-after-adr-consolidation-2026-11-24.pt-BR.md)
reports `html: false` in crystalline for `TYPST_FEATURES=html`. The pinned
oracle reports true, with false on both sides when the variable is absent.
`lab/typst-original/crates/typst-cli/src/info.rs:429–446` shows comma-separated
case-insensitive feature names, empty-entry skipping, and a reported diagnostic
for each unknown nonempty name while recognized peers still take effect.

## Owned contract

- Own run_info: compose L3 snapshot, L1 PARITY_VERSION, L2 build commit/platform and ordered environment data into L2 InfoData.
- Project effective feature booleans from the existing allowlisted
  `TYPST_FEATURES` snapshot entry under ADR-0128. Accept comma-separated
  `html`, `a11y-extras` and `bundle` case-insensitively, ignore empty
  entries, and retain false for features not selected. An unknown nonempty
  value writes ``error: unknown runtime feature: `{value}``` through the checked
  stderr writer. Report recognized peers and preserve the success status if
  diagnostics and output can be written. Treat an absent snapshot value as
  empty. Keep existing font/package paths and certificate presence reporting.
- Delegate human/JSON-pretty serialization to L2. Serialization/stdout failures exit 2; success exits 0. No TLS configuration or unapproved info fields/defaults are introduced.

## Verification and boundary

Verify absent, individually selected and combined features through the CLI.
Cover case-insensitive matching, empty segments, an unknown token with a known
peer, the human feature projection and the existing JSON field names. This
corrects the existing report in place under ADR-0127. No feature flag, new
public type, schema change or pipeline phase is needed.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.


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

Change only private run_info composition to `io::Result<ExitCode>`. Preserve
snapshot fields, certificate presence privacy, human/JSON serialization and
stdout. Emit existing serialization/stdout error text through the diagnostics
owner's checked writer; propagate actual stderr write/flush failure. Preserve
ordinary successful and failed serialization/output exit codes.

The existing pinned CLI-observables Nucleus governs shared output/exit obligations;
this owner does not redefine it. The user's explicit request authorizes replacing
stderr panic with exceptional operational exit 2. This changes private composition
only, without public signature/flag/default mode or eval/layout phase change
(ADR-0127). Retain healthy message bytes/order and all existing alerts/contracts.
Require measured RED→GREEN, healthy parity, tests, hashes and direct/reverse lineage;
no suppression or weakened assertion is authorized.
