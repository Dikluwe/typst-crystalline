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

- Own only binary composition, the closed exhaustive RunIntent dispatcher and the private import of the L3 Published carrier (Html string or Bundle files). Preserve all command arms and static delegation to private child handlers.
- Compose L2 intents, L3 operations and L1 values; no CLI parser, serializer, language policy, rendering, TLS, polling or watcher algorithm belongs here.
- Handler bodies, diagnostic drain and eviction belong to their respective owners. The import remains private in L4; the transient carrier is owned by 00_nucleo/prompts/shell/watch.md; no new public API, flag, default or phase is authorized.

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


## Transporte de publicação em L3 — V12, 2026-10-16

### Medição anterior à decisão

O snapshot before.json.gz em 00_nucleo/diagnosticos/tekt-v12-2026-10-16 conserva HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, árvore não commitada, UTC, fontes, L0, hashes, status e git diff HEAD --stat. O inventário anterior, conferido por hashes, emite 19 V12: um enum produtivo em 04_wiring/src/main.rs:22 e 18 declarações auxiliares de testes. O enum privado Published tem exatamente Html(String) e Bundle(Vec<BundleFile>); compile.rs:90/99/363/539/541 o produz e watch.rs:103/104 despacha os dois braços. O proprietário atual é wiring.md, que autoriza o carrier privado em L4: esta proposta atualiza explicitamente essa decisão, em vez de contrariá-la sem registro. É propriedade/representação Rust, sem nova alegação de paridade ou intenção upstream.

### Verificação comum

T1: preservar os controles reais de watch HTML e bundle HTTP/reload, sucesso/erro, no-serve/no-reload e último snapshot válido; testes p1485_watch_bundle_http_reload_e_preserva_snapshot_apos_erro e p1485_watch_flags_no_reload_no_serve_e_porta_ocupada em 04_wiring/tests/cli.rs, mais controles HTTP de 03_infra/src/watch.rs. T2: preservar compile de HTML/bundle, árvore completa, assets byte-exatos, stdout, warnings, exit e ausência de publicação em erro nos testes CLI existentes, sem editar esperados. T3: V12 RED real antes e zero produtivo depois; conferir por diff que só houve transferência do enum e ajuste do import, sem inferir modalidade por extensão. Verificar build, Clippy workspace/all-targets -D warnings e -D unfulfilled_lint_expectations, suíte workspace, formato, Rustdoc -D warnings e quatro índices, linhagem V1/V5/V15/V26 e reversa, SARIF e diff --check. Não escrever testes que apenas espelhem a sintaxe da mudança de owner. Execuções pesadas sequenciais sob MemoryMax=12 GiB, MemorySwapMax=0 e CARGO_BUILD_JOBS=1. Não publicar release.

### Responsabilidade exclusiva

Remover a declaração local de Published e o import de BundleFile que só a alimentava. Importar privadamente typst_infra::watch::Published. A raiz continua apenas com composição e dispatcher fechado, sem novo tipo local ou API L4. Os child handlers podem conservar super::Published.
