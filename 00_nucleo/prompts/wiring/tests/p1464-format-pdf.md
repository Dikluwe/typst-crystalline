# L0 — wiring/tests/p1464-format-pdf
Hash do Código: 883e0ee4

**Layer:** L4
**Exclusive consumer:** `04_wiring/tests/p1464_format_pdf.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9
```

## Current evidence

- `04_wiring/tests/p1464_format_pdf.rs:16` — `Profile`.
- `04_wiring/tests/p1464_format_pdf.rs:38` — `ProcessResult`.
- `04_wiring/tests/p1464_format_pdf.rs:45` — `success`.
- `04_wiring/tests/p1464_format_pdf.rs:51` — `Checks`.
- `04_wiring/tests/p1464_format_pdf.rs:56` — `require`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/tests/p1464_format_pdf.rs`.

## Owned contract

- Own public format.pdf namespace integration only, across default, a11y-extras, html and combined profiles. Verify type function, repr pdf, defaults pages none/standard ("1.7",)/tagged auto/pretty false, rejected manual construction and alias/std identity.
- attach/artifact are the same short-named functions as pdf.attach/pdf.artifact. The accessibility trio table-summary/header-cell/data-cell is jointly gated by a11y-extras, with exact disabled-feature message, ordered hints, field-only span and empty stdout; direct/with/alias/std paths preserve the gate.
- Enabled members remain discoverable through with. A same-named user closure does not receive special feature diagnostics. Global pdf is function with repr pdf, not the obsolete module morphology, retaining its members/gates.
- Keep svg/png defaults and html sibling gate/existence controls; do not test documentary export, PDF/A/UA conformity, pages/tagging/pretty export here. Private module/foreign-function pointer identity and insertion order are not oracles.
- Use finite subprocess limits and isolated task fixtures under /dev/shm without disk fallback. Missing capacity is an explicit inability to verify, never success.

## Verification and boundary

Run `cargo test -p typst --test p1464_format_pdf -- --test-threads=1` with the harness prerequisites above.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.


## Fronteira do harness — V12, 2026-10-16

### Medição anterior à decisão

O snapshot before.json.gz de 00_nucleo/diagnosticos/tekt-v12-2026-10-16 conserva HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, árvore não commitada, UTC, fontes/L0/configuração/lock, hashes, status e git diff HEAD --stat. Os 2077 hashes coincidem com o checkpoint final da etapa FrameItem: a suíte anterior passou com 8622 testes, zero falhas e sete ignorados nesse mesmo estado. gate-baseline-result.json confirma RED dos 19 V12 antes de qualquer mudança de L0 ou código. items.json contém os itens AST completos e classification.json registra a separação por unidade. Cláusulas pertinentes dos owners foram lidas junto dos helpers; não há nova alegação de paridade ou intenção upstream.

O linter identifica enum e impl inerente de topo em L4, inclusive suporte de teste. A separação aqui é de responsabilidade: cenários, fixtures de linguagem e asserções de produto ficam em L4; helpers independentes de subprocesso/filesystem/observação passam a uma unidade de suporte L3. Não mover a suíte completa, encaixar declarações em módulo L4 para explorar a leitura rasa do parser, excluir testes do lint, converter impl em funções só para escapar da regra ou alterar severidade.

### Responsabilidade exclusiva

Transferir os tipos ProcessResult, Checks, seus impls e Drop existentes para `03_infra/tests/support/wiring/p1464_format_pdf.rs`, owner `00_nucleo/prompts/infra/test-support/wiring/p1464_format_pdf.md`. Conservar em `04_wiring/tests/p1464_format_pdf.rs` todos os cenários e observações finais. Importar somente Checks, ProcessResult por módulo harness com #[path] estático para L3. Não usar esse módulo na biblioteca de produção. Os corpos anteriores desta transferência estão medidos em: 04_wiring/tests/p1464_format_pdf.rs:38 struct_item ProcessResult; 04_wiring/tests/p1464_format_pdf.rs:44 impl_item ProcessResult; 04_wiring/tests/p1464_format_pdf.rs:51 struct_item Checks; 04_wiring/tests/p1464_format_pdf.rs:55 impl_item Checks.

### Aceitação e limites

Esta separação de harness é interna ADR-0127: nenhum código de suporte integra a biblioteca L3 ou a CLI de produção e pub(super) só permite acesso no crate de teste. Nenhuma API pública de produto, dependência, flag, default ou fase muda. O contrato público Published é proposta separada, já ratificada, e não autoriza outros contratos. Não adicionar derives, supressões, hooks, nova política de cleanup, tolerância ou fallback de verificação.

Preservar todos os corpos dos itens movidos, salvo a visibilidade mínima ao módulo de teste pai. Campos de Checks permanecem privados. Imports std devem ser próprios de L3, sem use super::* ou dependência de lógica L4. BIN, invocação dos subprocessos, opções, cenários, esperados e #[test] permanecem em L4; o helper não lê CARGO_BIN_EXE_typst. Não consolidar diferenças legítimas entre harnesses (prefixos, mensagens, atomicidade de criação e cleanup).

Conferir replay restrito das transferências, funções de teste idênticas, atributos/expectativas sem mudanças, grafo 1:1 válido, ausência do suporte na biblioteca produtiva, V12 GREEN zero e SARIF completo sem avisos novos. Revalidar suíte workspace sob TMPDIR explícito em /dev/shm e oráculo vanilla pinado 9f2b6e8715237cb086899a42873660fe744622e8 com binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea; cargo build, Clippy workspace/all-targets -D warnings e -D unfulfilled_lint_expectations, formato, Rustdoc -D warnings com quatro índices, V1/V5/V15/V26 estrito e linhagem reversa, diff --check. L0 primeiro; resselo restrito aos owners e código autorizado. Jobs pesados sequenciais: MemoryMax=12 GiB, MemorySwapMax=0, CARGO_BUILD_JOBS=1. Fontes/L0/configuração congelados durante comandos. Não escrever testes de espelho da sintaxe de import nem alegar ganho de RAM/IA não medido. Sem release.
