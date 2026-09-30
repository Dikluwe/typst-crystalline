# L0 — wiring/tests/cli
Hash do Código: 41880714

**Layer:** L4
**Exclusive consumer:** `04_wiring/tests/cli.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:0e59744a924f0f6acbe7c4d20db9efc7caed4783c3b8d6b2c2c5a9cace340ece
```

## Current evidence

- `04_wiring/tests/cli.rs:38` — `temp_typ`.
- `04_wiring/tests/cli.rs:46` — `temp_pdf`.
- `04_wiring/tests/cli.rs:54` — `cleanup`.
- `04_wiring/tests/cli.rs:60` — `WatchedChild`.
- `04_wiring/tests/cli.rs:63` — `assert_alive`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/tests/cli.rs`.

## Owned contract

- Own the external CARGO_BIN_EXE_typst integration suite only. Observe exit status, stdout/stderr and artifacts, never import private main modules or implement parsing/compilation/formatting.
- Cover commands/aliases/help/rejections, root/font/input/certificate/document id, eval/query/fonts/info/completions, formatted cross-file/plugin spans, transactional init, formats/features/casts and semantic HTML morphology.
- Success with no warning preserves empty stderr; file output leaves stdout empty. Warnings precede errors and retain success; semantic errors exit 1 and argument/I/O errors 2. Check format signatures and content/DOM rather than complete byte parity.
- Use isolated temporary directories and bounded processes. Watch guards kill and wait on every path; each relevant mutation occurs once. Exercise initial publish, asset change, irrelevant-file filtering, temporary error with last artifact retained and recovery while the child remains alive. Sleeps/retries do not prove internal causal ordering; ArmedWatch integration owns arm/publish/wait.
- Nushell completions run twice deterministically with nonempty script/module/exported command structure and empty stderr in isolated HOME/XDG, producing no files. Preserve Bash/invalid-shell controls and public w alias; accept only the existing singular/plural help annotation spellings.
- Preserve canonical repr fields/order and semantic HTML feature/target separation, void/nested/escaped/whitespace cases. Default PNG must finish within memory/time bounds and produce A4 IHDR 1191 by 1684 at 144 PPI, never pass by OOM or absent output.
- Bundle cases cover binary assets, nested mixed-format documents, normalized duplicates, invalid paths, loose content and feature absence. Controlled watch/HTTP tests cover no_serve/no_reload/port, 200/301/404, MIME, raw asset bytes, response-only HTML reload script, SSE and last-snapshot preservation. Leave no daemon or child process.

## Verification and boundary

Run `cargo test -p typst --test cli -- --test-threads=1` with the harness prerequisites above.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
