# Passo 1366 — status terminal antes dos diagnósticos

## Objetivo

Alinhar a ordem observável do stderr do `watch`: o bloco terminal deve aparecer
antes de warnings, errors ou mensagem operacional produzida pela tentativa.

## Contrato privado

L4 acrescenta:

```rust
fn report_in_order(
    status: impl FnOnce(),
    diagnostics: impl FnOnce(),
);
```

O helper chama `status` exatamente uma vez e depois `diagnostics` exatamente
uma vez. `run_compile_observed` recebe uma callback privada
`FnMut(ExitCode, bool)` e a encaminha como primeiro estágio de
`report_in_order` em toda saída: preliminar, escrita e resultado compilado.

No `watch`, a callback escolhe `WatchStatus` com o `Instant` P1365 e escreve o
bloco P1364. O status final deixa de ser emitido depois do retorno. Em
`compile`, uma callback vazia mantém zero status de watch. Para resultados do
compilador, warnings permanecem antes de errors dentro do segundo estágio.

O artefato e timings de sucesso devem ser escritos antes de anunciar sucesso.
Em falha de escrita, o status é `Error` e precede warnings/mensagem operacional.

## Teste

Um teste exato in-process fornece duas closures sentinela, prova ordem
`status,diagnostics`, cardinalidade 1/1 e ausência de transformação. Sem
relógio, documento, filesystem, processo ou watcher.

## L0 e ADR-0127

Owner exclusivo: `00_nucleo/prompts/wiring.md` →
`04_wiring/src/main.rs`. É correção interna de ordenação de paridade no mesmo
canal, sem API pública, default, fase ou incompatibilidade; fluxo contínuo.

## Scope-out

- conteúdo/formatação interna dos diagnósticos;
- cor, fullscreen, cursor, limpeza e flush explícito;
- ordem de medições opt-in posteriores ao pipeline;
- polling, dependências, staging, publish, abandon, wait e eviction;
- execução externa de `watch`/`w`.

## Regime e memória

Protocolo completo, executado sem atestação de isolamento. Proibido executar
`watch`, `w`, suíte externa de wiring ou suíte ampla. Um teste exato e um
build, sequenciais, `CARGO_BUILD_JOBS=1`, timeout máximo 120 s, 4 MiB por
stream e 16 MiB agregado.

