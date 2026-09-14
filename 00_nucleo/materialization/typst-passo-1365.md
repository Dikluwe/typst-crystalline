# Passo 1365 — iniciar duração antes do status `Compiling`

## Objetivo

Alinhar a fronteira da duração humana do `watch`: o marcador monotônico deve
ser capturado antes de formar e emitir o bloco inicial `Compiling`.

## Contrato privado

L4 acrescenta um helper genérico privado equivalente a:

```rust
fn prepare_watch_cycle_with<T>(
    capture: impl FnOnce() -> T,
    format_compiling: impl FnOnce() -> String,
) -> (T, String);
```

Ele chama `capture` exatamente uma vez, depois `format_compiling` exatamente
uma vez, e devolve os resultados sem transformação. Um wrapper privado
`prepare_watch_cycle() -> (Instant, String)` fornece `Instant::now` e
`watch_block_now(&WatchStatus::Compiling)`.

`run_watch` chama o wrapper uma vez por ciclo, emite o bloco pronto e passa a
usar o `Instant` retornado para o único `elapsed()`. Não pode capturar outro
marcador antes da compilação.

## Teste

Um teste unitário exato usa closures sentinela e armazenamento em memória para
provar a sequência `capture`, `format`, sem relógio real, sleep, processo, I/O
ou execução de `watch`. Também prova chamada única e preservação dos dois
valores retornados.

## L0 e ADR-0127

Owner exclusivo: `00_nucleo/prompts/wiring.md` →
`04_wiring/src/main.rs`. É correção interna de fórmula/fronteira de paridade,
sem API pública, novo default, fase de pipeline ou incompatibilidade; segue
fluxo contínuo L0-first, resselo e RED→GREEN.

## Scope-out

- valor absoluto ou precisão do relógio;
- tempo gasto depois do status terminal;
- cor, fullscreen, cursor, buffering e flush;
- ordem do status final perante warnings/errors;
- polling, staging, publish, abandon, wait e eviction;
- execução externa de `watch`/`w`.

## Regime e memória

Protocolo completo, executado sem atestação de isolamento no workspace
partilhado. Proibido executar `watch`, `w`, suíte externa de wiring ou suíte
ampla. Apenas um teste exato e um build, sequenciais, `CARGO_BUILD_JOBS=1`,
timeout máximo 120 s, 4 MiB por stream e 16 MiB agregado.

