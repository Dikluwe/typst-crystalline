# L0 — wiring/fonts
Hash do Código: a216be89

**Layer:** L4
**Exclusive consumer:** `04_wiring/src/fonts.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:e9d6623a18ffc35f36051b280d96a75a0e4c1bd1768361a3a50c98587c568aef
```

## Current evidence

- `04_wiring/src/fonts.rs:11` — `run_fonts`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/src/fonts.rs`.

## Owned contract

- Own run_fonts composition: call L3 inventory_fonts with requested paths/include-system and map family/path/index/style/weight/stretch/embedded into L2 FontDisplayEntry.
- Serialize with the existing format_fonts variants and write stdout. Preserve discovery/order; do not rescan, resort or implement font policy. Success exits 0 and output failure exits 2.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
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

Change only private run_fonts composition to `io::Result<ExitCode>`. Preserve
L3 inventory and L2 formatting/stdout exactly. If stdout fails, emit the existing
font-list error through the diagnostics owner's checked writer; propagate its
actual stderr write/flush failure. Preserve ordinary success 0/output failure 2.

The existing pinned CLI-observables Nucleus governs shared output/exit obligations;
this owner does not redefine it. The user's explicit request authorizes replacing
stderr panic with exceptional operational exit 2. This changes private composition
only, without public signature/flag/default mode or eval/layout phase change
(ADR-0127). Retain healthy message bytes/order and all existing alerts/contracts.
Require measured RED→GREEN, healthy parity, tests, hashes and direct/reverse lineage;
no suppression or weakened assertion is authorized.
