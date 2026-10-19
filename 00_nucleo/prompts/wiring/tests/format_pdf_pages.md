# L0 — wiring/tests/format_pdf_pages
Hash do Código: 7e659d14

**Layer:** L4
**Exclusive consumer:** `04_wiring/tests/format_pdf_pages.rs`

## Current evidence

- `04_wiring/tests/format_pdf_pages.rs:30` — `ProcessResult`.
- `04_wiring/tests/format_pdf_pages.rs:37` — `success`.
- `04_wiring/tests/format_pdf_pages.rs:42` — `typst_binary`.
- `04_wiring/tests/format_pdf_pages.rs:48` — `shm_root`.
- `04_wiring/tests/format_pdf_pages.rs:58` — `fixture_paths`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/tests/format_pdf_pages.rs`.

## Owned contract

- Own real-PDF page-selection integration: documentary integer/range, CLI list and explicit precedence, filtered page count and interactions with tagging. A beyond-end selection must not die by signal; its count is not fixed by this contract.
- Documentary explicit true with pages fails with `cannot enable tagged PDF and export a page range`; CLI true plus pages fails before compilation with distinct `cannot enable PDF tags when exporting a page range`, exit 1.
- Documentary Auto plus pages emits `exporting a page range disables PDF tagging` and the two canonical inaccessibility/silencing hints, producing no struct tree. Explicit false remains silent.
- Included headings survive native outline filtering; excluded headings disappear and included descendants reparent to the nearest included ancestor. An included outline must not become empty merely because pages are selected. Links to removed pages lose their annotation instead of redirecting; keep no-selection control.
- Inspect parsed counts/tree/titles/annotations, not full PDF bytes. Standard/conformity/PageLabels and PNG/SVG multi-output are outside this suite. Use explicit /dev/shm TMPDIR, no disk fallback, serial execution and bounded subprocesses.

## Verification and boundary

Run `cargo test -p typst-wiring --test format_pdf_pages -- --test-threads=1` with the harness prerequisites above.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.


## Manutenção local e observação de testes — 2026-10-16

### Medição anterior à decisão

`04_wiring/tests/format_pdf_pages.rs` nas linhas 32 contém os avisos registrados em `lab/results/construction/diagnostics/clippy-locais-2026-10-16/before.json.gz`. O snapshot conserva fontes/L0, hashes, HEAD, working tree não commitado, UTC e git diff HEAD --stat. A inspeção dos consumidores e usos precedeu esta decisão; o gate Clippy selecionado deve falhar antes do código. Manutenção de representação/observação Rust, sem nova medição de intenção upstream ou paridade.

### Responsabilidade e aceitação

Preservar ProcessResult.stdout e sua captura; incluir stdout e stderr nos diagnósticos das assertions de sucesso existentes. Não usar Debug derivado como pretexto de leitura e não trocar verificações canônicas de stderr, status, PDF/precedência/fixtures, subprocessos ou timeouts. Esta leitura útil resolve dead_code sem supressão ou perda de informação de falha.

Fluxo contínuo ADR-0127: sem contrato público, default de produto ou fase novos. L0 precede código e resselo restrito. Nenhuma supressão de lint nova. Conferir redução exata do inventário e ausência de avisos novos, gate Clippy RED→GREEN e unfulfilled_lint_expectations como erro; executar suíte workspace, build, formato, Rustdoc e linhagem sob 12 GiB, swap zero e um job. Não criar testes que apenas espelhem derive, Entry ou posicionamento de módulos. O caso de bbox exige observação real conforme especificado; os demais controles existentes mantêm fixtures e expectativas.


## Fronteira do harness — V12, 2026-10-16

### Medição anterior à decisão

O snapshot before.json.gz de lab/results/construction/diagnostics/tekt-v12-2026-10-16 conserva HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, árvore não commitada, UTC, fontes/L0/configuração/lock, hashes, status e git diff HEAD --stat. Os 2077 hashes coincidem com o checkpoint final da etapa FrameItem: a suíte anterior passou com 8622 testes, zero falhas e sete ignorados nesse mesmo estado. gate-baseline-result.json confirma RED dos 19 V12 antes de qualquer mudança de L0 ou código. items.json contém os itens AST completos e classification.json registra a separação por unidade. Cláusulas pertinentes dos owners foram lidas junto dos helpers; não há nova alegação de paridade ou intenção upstream.

O linter identifica enum e impl inerente de topo em L4, inclusive suporte de teste. A separação aqui é de responsabilidade: cenários, fixtures de linguagem e asserções de produto ficam em L4; helpers independentes de subprocesso/filesystem/observação passam a uma unidade de suporte L3. Não mover a suíte completa, encaixar declarações em módulo L4 para explorar a leitura rasa do parser, excluir testes do lint, converter impl em funções só para escapar da regra ou alterar severidade.

### Responsabilidade exclusiva

Transferir os tipos ProcessResult, seus impls e Drop existentes para `03_infra/tests/support/wiring/format_pdf_pages.rs`, owner `00_nucleo/prompts/infra/tests/support/wiring/format_pdf_pages.md`. Conservar em `04_wiring/tests/format_pdf_pages.rs` todos os cenários e observações finais. Importar somente ProcessResult por módulo harness com #[path] estático para L3. Não usar esse módulo na biblioteca de produção. Os corpos anteriores desta transferência estão medidos em: 04_wiring/tests/format_pdf_pages.rs:30 struct_item ProcessResult; 04_wiring/tests/format_pdf_pages.rs:36 impl_item ProcessResult.

### Aceitação e limites

Esta separação de harness é interna ADR-0127: nenhum código de suporte integra a biblioteca L3 ou a CLI de produção e pub(super) só permite acesso no crate de teste. Nenhuma API pública de produto, dependência, flag, default ou fase muda. O contrato público Published é proposta separada, já ratificada, e não autoriza outros contratos. Não adicionar derives, supressões, hooks, nova política de cleanup, tolerância ou fallback de verificação.

Preservar todos os corpos dos itens movidos, salvo a visibilidade mínima ao módulo de teste pai. Campos de Checks permanecem privados. Imports std devem ser próprios de L3, sem use super::* ou dependência de lógica L4. BIN, invocação dos subprocessos, opções, cenários, esperados e #[test] permanecem em L4; o helper não lê CARGO_BIN_EXE_typst. Não consolidar diferenças legítimas entre harnesses (prefixos, mensagens, atomicidade de criação e cleanup).

Conferir replay restrito das transferências, funções de teste idênticas, atributos/expectativas sem mudanças, grafo 1:1 válido, ausência do suporte na biblioteca produtiva, V12 GREEN zero e SARIF completo sem avisos novos. Revalidar suíte workspace sob TMPDIR explícito em /dev/shm e oráculo vanilla pinado 9f2b6e8715237cb086899a42873660fe744622e8 com binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea; cargo build, Clippy workspace/all-targets -D warnings e -D unfulfilled_lint_expectations, formato, Rustdoc -D warnings com quatro índices, V1/V5/V15/V26 estrito e linhagem reversa, diff --check. L0 primeiro; resselo restrito aos owners e código autorizado. Jobs pesados sequenciais: MemoryMax=12 GiB, MemorySwapMax=0, CARGO_BUILD_JOBS=1. Fontes/L0/configuração congelados durante comandos. Não escrever testes de espelho da sintaxe de import nem alegar ganho de RAM/IA não medido. Sem release.
