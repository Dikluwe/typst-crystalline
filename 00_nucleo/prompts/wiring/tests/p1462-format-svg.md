# L0 — wiring/tests/p1462-format-svg
Hash do Código: 573ade30

**Layer:** L4
**Exclusive consumer:** `04_wiring/tests/p1462_format_svg.rs`

## Current evidence

- `04_wiring/tests/p1462_format_svg.rs:20` — `TempDir`.
- `04_wiring/tests/p1462_format_svg.rs:25` — `new`.
- `04_wiring/tests/p1462_format_svg.rs:42` — `drop`.
- `04_wiring/tests/p1462_format_svg.rs:48` — `ProcessResult`.
- `04_wiring/tests/p1462_format_svg.rs:55` — `Compilation`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/tests/p1462_format_svg.rs`.

## Owned contract

- Own format.svg.pretty external-process tests: compact default, true/false/root/empty-rule cascade, explicit caller precedence, rejected containers/unknown named/type/direct construction.
- Check valid XML and unchanged semantic morphology between compact/pretty, without complete SVG hashes. Keep PNG/PDF/HTML/frame neutrality.
- Without html, sibling format.html produces its public feature gate; with html, only existence belongs here and its detailed surface belongs to `00_nucleo/prompts/wiring/tests/p1465-format-html.md`. Use isolated bounded fixtures; no administrative oracles.

## Verification and boundary

Run `cargo test -p typst --test p1462_format_svg -- --test-threads=1` with the harness prerequisites above.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.


## Expressões locais verificadas — Clippy 2026-10-16

### Medição anterior à decisão

04_wiring/tests/p1462_format_svg.rs:454 (clippy::sliced_string_as_bytes); 04_wiring/tests/p1462_format_svg.rs:515 (clippy::question_mark). O snapshot `00_nucleo/diagnosticos/clippy-expressions-2026-10-16/before.json.gz` conserva fontes/L0, HEAD, working tree não commitado, status, diffstat e UTC; initial-diagnostics.json conserva sugestões e spans. Os usos e tipos foram inspecionados antes da decisão. São representações Rust internas; nenhuma nova paridade upstream ou ganho de performance é alegado.

### Responsabilidade exclusiva e aceitação

Usar &source.as_bytes()[start..end] no observador XML: start/end vêm de find de tags ASCII no str UTF-8 e são fronteiras válidas. Preservar limites e erros existentes. Usar ? somente na extração de artifact, depois de require_success, conservando registro de falha e None.

Fluxo contínuo ADR-0127: sem API pública, default de produto, fase, dependência ou supressão novos. L0 precede código e resselo restrito. Preservar inputs e resultados dos controles existentes, sem relaxar tolerâncias. Gate dos lints selecionados RED→GREEN; executar suíte workspace, build, Clippy all-targets, formato, Rustdoc e linhagem. Não criar testes que apenas espelhem a sintaxe Rust.


## Fronteira do harness — V12, 2026-10-16

### Medição anterior à decisão

O snapshot before.json.gz de 00_nucleo/diagnosticos/tekt-v12-2026-10-16 conserva HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, árvore não commitada, UTC, fontes/L0/configuração/lock, hashes, status e git diff HEAD --stat. Os 2077 hashes coincidem com o checkpoint final da etapa FrameItem: a suíte anterior passou com 8622 testes, zero falhas e sete ignorados nesse mesmo estado. gate-baseline-result.json confirma RED dos 19 V12 antes de qualquer mudança de L0 ou código. items.json contém os itens AST completos e classification.json registra a separação por unidade. Cláusulas pertinentes dos owners foram lidas junto dos helpers; não há nova alegação de paridade ou intenção upstream.

O linter identifica enum e impl inerente de topo em L4, inclusive suporte de teste. A separação aqui é de responsabilidade: cenários, fixtures de linguagem e asserções de produto ficam em L4; helpers independentes de subprocesso/filesystem/observação passam a uma unidade de suporte L3. Não mover a suíte completa, encaixar declarações em módulo L4 para explorar a leitura rasa do parser, excluir testes do lint, converter impl em funções só para escapar da regra ou alterar severidade.

### Responsabilidade exclusiva

Transferir os tipos TempDir, Checks, seus impls e Drop existentes para `03_infra/tests/support/wiring/p1462_format_svg.rs`, owner `00_nucleo/prompts/infra/test-support/wiring/p1462_format_svg.md`. Conservar em `04_wiring/tests/p1462_format_svg.rs` todos os cenários e observações finais. Importar somente Checks, TempDir por módulo harness com #[path] estático para L3. Não usar esse módulo na biblioteca de produção. Os corpos anteriores desta transferência estão medidos em: 04_wiring/tests/p1462_format_svg.rs:18 static_item NEXT_TEMP; 04_wiring/tests/p1462_format_svg.rs:20 struct_item TempDir; 04_wiring/tests/p1462_format_svg.rs:24 impl_item TempDir; 04_wiring/tests/p1462_format_svg.rs:41 impl_item TempDir; 04_wiring/tests/p1462_format_svg.rs:63 struct_item Checks; 04_wiring/tests/p1462_format_svg.rs:67 impl_item Checks.

### Aceitação e limites

Esta separação de harness é interna ADR-0127: nenhum código de suporte integra a biblioteca L3 ou a CLI de produção e pub(super) só permite acesso no crate de teste. Nenhuma API pública de produto, dependência, flag, default ou fase muda. O contrato público Published é proposta separada, já ratificada, e não autoriza outros contratos. Não adicionar derives, supressões, hooks, nova política de cleanup, tolerância ou fallback de verificação.

Preservar todos os corpos dos itens movidos, salvo a visibilidade mínima ao módulo de teste pai. Campos de Checks permanecem privados. Imports std devem ser próprios de L3, sem use super::* ou dependência de lógica L4. BIN, invocação dos subprocessos, opções, cenários, esperados e #[test] permanecem em L4; o helper não lê CARGO_BIN_EXE_typst. Não consolidar diferenças legítimas entre harnesses (prefixos, mensagens, atomicidade de criação e cleanup).

Conferir replay restrito das transferências, funções de teste idênticas, atributos/expectativas sem mudanças, grafo 1:1 válido, ausência do suporte na biblioteca produtiva, V12 GREEN zero e SARIF completo sem avisos novos. Revalidar suíte workspace sob TMPDIR explícito em /dev/shm e oráculo vanilla pinado 9f2b6e8715237cb086899a42873660fe744622e8 com binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea; cargo build, Clippy workspace/all-targets -D warnings e -D unfulfilled_lint_expectations, formato, Rustdoc -D warnings com quatro índices, V1/V5/V15/V26 estrito e linhagem reversa, diff --check. L0 primeiro; resselo restrito aos owners e código autorizado. Jobs pesados sequenciais: MemoryMax=12 GiB, MemorySwapMax=0, CARGO_BUILD_JOBS=1. Fontes/L0/configuração congelados durante comandos. Não escrever testes de espelho da sintaxe de import nem alegar ganho de RAM/IA não medido. Sem release.
