# L0 — infra/tests/support/wiring/format_pdf_pages
Hash do Código: ad0979cb

**Layer:** L3, test-only.
**Exclusive consumer:** `03_infra/tests/support/wiring/format_pdf_pages.rs`.
**Governança:** ADR-0107/0108/0127/0129/0130.


## Fronteira do harness — V12, 2026-10-16

### Medição anterior à decisão

O snapshot before.json.gz de lab/results/construction/diagnostics/tekt-v12-2026-10-16 conserva HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, árvore não commitada, UTC, fontes/L0/configuração/lock, hashes, status e git diff HEAD --stat. Os 2077 hashes coincidem com o checkpoint final da etapa FrameItem: a suíte anterior passou com 8622 testes, zero falhas e sete ignorados nesse mesmo estado. gate-baseline-result.json confirma RED dos 19 V12 antes de qualquer mudança de L0 ou código. items.json contém os itens AST completos e classification.json registra a separação por unidade. Cláusulas pertinentes dos owners foram lidas junto dos helpers; não há nova alegação de paridade ou intenção upstream.

O linter identifica enum e impl inerente de topo em L4, inclusive suporte de teste. A separação aqui é de responsabilidade: cenários, fixtures de linguagem e asserções de produto ficam em L4; helpers independentes de subprocesso/filesystem/observação passam a uma unidade de suporte L3. Não mover a suíte completa, encaixar declarações em módulo L4 para explorar a leitura rasa do parser, excluir testes do lint, converter impl em funções só para escapar da regra ou alterar severidade.

### Responsabilidade exclusiva

Transferir os tipos ProcessResult, seus impls e Drop existentes para `03_infra/tests/support/wiring/format_pdf_pages.rs`, owner `00_nucleo/prompts/infra/tests/support/wiring/format_pdf_pages.md`. Conservar em `04_wiring/tests/format_pdf_pages.rs` todos os cenários e observações finais. Importar somente ProcessResult por módulo harness com #[path] estático para L3. Não usar esse módulo na biblioteca de produção. Os corpos anteriores desta transferência estão medidos em: 04_wiring/tests/format_pdf_pages.rs:30 struct_item ProcessResult; 04_wiring/tests/format_pdf_pages.rs:36 impl_item ProcessResult.

Apenas este prompt é proprietário do novo consumer. O owner L4 citado governa o cenário chamador, sem ownership compartilhado. Tipos, campos necessários ao pai e membros inerentes usados por ele podem ser pub(super); funções internas, estado de Checks e statics ficam privados. Nenhum re-export no crate typst_infra.

### Aceitação e limites

Esta separação de harness é interna ADR-0127: nenhum código de suporte integra a biblioteca L3 ou a CLI de produção e pub(super) só permite acesso no crate de teste. Nenhuma API pública de produto, dependência, flag, default ou fase muda. O contrato público Published é proposta separada, já ratificada, e não autoriza outros contratos. Não adicionar derives, supressões, hooks, nova política de cleanup, tolerância ou fallback de verificação.

Preservar todos os corpos dos itens movidos, salvo a visibilidade mínima ao módulo de teste pai. Campos de Checks permanecem privados. Imports std devem ser próprios de L3, sem use super::* ou dependência de lógica L4. BIN, invocação dos subprocessos, opções, cenários, esperados e #[test] permanecem em L4; o helper não lê CARGO_BIN_EXE_typst. Não consolidar diferenças legítimas entre harnesses (prefixos, mensagens, atomicidade de criação e cleanup).

Conferir replay restrito das transferências, funções de teste idênticas, atributos/expectativas sem mudanças, grafo 1:1 válido, ausência do suporte na biblioteca produtiva, V12 GREEN zero e SARIF completo sem avisos novos. Revalidar suíte workspace sob TMPDIR explícito em /dev/shm e oráculo vanilla pinado 9f2b6e8715237cb086899a42873660fe744622e8 com binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea; cargo build, Clippy workspace/all-targets -D warnings e -D unfulfilled_lint_expectations, formato, Rustdoc -D warnings com quatro índices, V1/V5/V15/V26 estrito e linhagem reversa, diff --check. L0 primeiro; resselo restrito aos owners e código autorizado. Jobs pesados sequenciais: MemoryMax=12 GiB, MemorySwapMax=0, CARGO_BUILD_JOBS=1. Fontes/L0/configuração congelados durante comandos. Não escrever testes de espelho da sintaxe de import nem alegar ganho de RAM/IA não medido. Sem release.

### Operações desta unidade

Conservar code Option<i32>, stdout e stderr String e os derives já existentes. success continua code == Some(0); preservar sinal/None como falha e a leitura útil dos dois canais nos diagnósticos. A captura e o limite temporal do subprocesso ficam no cenário L4 sem mudanças.
