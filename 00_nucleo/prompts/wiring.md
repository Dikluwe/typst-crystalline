# L0 — wiring
Hash do Código: 3918c50f

**Layer:** L4
**Exclusive consumer:** `04_wiring/src/main.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:0e59744a924f0f6acbe7c4d20db9efc7caed4783c3b8d6b2c2c5a9cace340ece
```

## Current evidence

- `04_wiring/src/main.rs:22` — `Published`.
- `04_wiring/src/main.rs:27` — `main`.
- `04_wiring/src/main.rs:43` — `main_is_only_a_closed_command_dispatcher`.

These are local source anchors, not a new upstream parity measurement.

## Owned contract

- Own only binary composition, the closed exhaustive RunIntent dispatcher and private Published carrier (Html string or Bundle files). Preserve all command arms and static delegation to private child handlers.
- Compose L2 intents, L3 operations and L1 values; no CLI parser, serializer, language policy, rendering, TLS, polling or watcher algorithm belongs here.
- Handler bodies, diagnostic drain and eviction belong to their respective owners. Published remains private/transient for compile/watch; no new public API, flag, default or phase is authorized.

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
