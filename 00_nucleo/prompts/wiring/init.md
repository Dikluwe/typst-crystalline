# L0 — wiring/init
Hash do Código: 182bc360

**Layer:** L4
**Exclusive consumer:** `04_wiring/src/init.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:e9d6623a18ffc35f36051b280d96a75a0e4c1bd1768361a3a50c98587c568aef
```

## Current evidence

- `04_wiring/src/init.rs:10` — `run_init`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/src/init.rs`.

## Owned contract

- Own run_init composition and pass template/destination/certificate inputs to the existing L3 initialize operation.
- On success print the existing destination and entrypoint lines and exit 0; initialization failure exits 1. Validation, download, cleanup and transactional project writes remain in L3.

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

Change only private run_init composition to `io::Result<ExitCode>`. Preserve
L3 initialization and successful destination/entrypoint stdout lines exactly.
On initialization failure send the existing error line through the diagnostics
owner's checked writer; propagate actual stderr write/flush failure. A healthy
stderr keeps ordinary initialization failure exit 1. Existing successful stdout
println panic candidates remain outside this stderr correction and are disclosed.

The existing pinned CLI-observables Nucleus governs shared output/exit obligations;
this owner does not redefine it. The user's explicit request authorizes replacing
stderr panic with exceptional operational exit 2. This changes private composition
only, without public signature/flag/default mode or eval/layout phase change
(ADR-0127). Retain healthy message bytes/order and all existing alerts/contracts.
Require measured RED→GREEN, healthy parity, tests, hashes and direct/reverse lineage;
no suppression or weakened assertion is authorized.


## Checked init success notification — 2026-10-23

### Measurement before decision

`lab/results/init-output-2026-10-23/baseline.json` and its compressed source archive bind HEAD, uncommitted tree/diff stat, all source/L0/config inputs and binary hashes; they verify that batch A inputs remain unchanged. `before/receipt.json` records 28 actual local-template process probes with exact UTC/commands/environment/bytes/files and verified binaries. Tekt success with full or broken stdout exits101 after project publication; healthy success exits0 and uses stdout. Failed stderr on initialization errors already exits2. Pinned vanilla uses stderr for success and therefore exits0 with only stdout failed; it panics or receives SIGPIPE when its used stderr fails. This is a Tekt robustness defect, not a claim that vanilla checks stdout.

Source `04_wiring/src/init.rs:18-19` uses unchecked println for the two success lines; `diagnostics.rs:111-116` shows checked write_all+flush; `main.rs:35-39` already maps private I/O errors to2. `03_infra/src/project_init.rs:86-94` publishes the project before returning its result. The measured channel/bytes/exit/artifact effects are observables; writer injection is a testing mechanism. An error during notification is not failure to create the project.

### Exclusive correction and acceptance

After successful L3 initialization, acquire one stdout lock and write the existing success destination line followed by the existing entrypoint line through a private checked writer helper, then explicitly flush. Propagate each actual write/WriteZero/flush error unchanged through run_init's existing io::Result; the main owner already maps it to operational exit2. Stop after a failed write, without retrying output, falling back to stderr, retrying initialization or deleting the created project. Keep healthy bytes, newline order, stdout channel and exit0. Preserve initialization-failure stderr text and exit1, and propagate failed diagnostic emission as exit2. All package resolution, validation, download, copying/staging/publication/cleanup stay in L3. The helper owns only existing notification composition and accepts borrowed paths plus an injected Write for unit fault tests.

Supersede only the earlier exclusion of successful stdout println panic candidates. The existing CLI-observables Nucleus and main boundary remain unchanged. Require semantic process RED→GREEN on full stdout and closed stdout, both failed channels and retained project files; healthy output and error controls; deterministic writer tests for partial/zero writes, first/second-line failure, BrokenPipe and flush-only failure. A healthy unused failed stderr does not turn successful stdout notification into failure.

This is an expressly requested internal robustness correction under continuous-flow ADR-0127. No public entity field, trait/signature, product mode/flag/default/channel, compatibility break or pipeline phase change. L0 precedes code and resealing; retain RED failures and all existing warnings/contracts. Complete required suites/builds/Clippy/rustdoc/fmt/path/forward/reverse/Nucleus validation.
