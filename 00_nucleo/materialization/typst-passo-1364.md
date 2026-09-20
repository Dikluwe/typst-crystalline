# Passo 1364 — linha vazia após status terminal do `watch`

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


## Objetivo

Corrigir o espaçamento não-fullscreen do envelope P1363: `Compiling` termina
com uma quebra; `Success`, `PartialSuccess` e `Error` terminam com duas.

## Contrato

L4 mantém `watch_line_at` como linha sem terminador e acrescenta helpers
privados equivalentes a:

```rust
fn watch_block_at(status: &WatchStatus, time: time::Time) -> String;
fn watch_block_now(status: &WatchStatus) -> String;
```

Para `03:04:05`, o resultado é exatamente:

```text
Compiling             -> "[03:04:05] compiling ...\n"
Success(2.8492 s)     -> "[03:04:05] compiled successfully in 2.85 s\n\n"
PartialSuccess(...)   -> "[03:04:05] compiled with warnings in 2.85 s\n\n"
Error                 -> "[03:04:05] compiled with errors\n\n"
```

`run_watch` emite somente os blocos prontos; não acrescenta terminador fora
do helper. Nenhuma API pública muda.

## Scope-out

- cor, fullscreen, limpeza e cursor;
- ordem entre status e diagnósticos, buffering e flush explícito;
- polling, dependências, staging, publish, abandon, wait e eviction;
- execução externa de `watch`/`w`;
- timestamp real, timezone e precisão subsegundo.

## L0 e gate ADR-0127

Owner exclusivo: `00_nucleo/prompts/wiring.md` →
`04_wiring/src/main.rs`. É correção interna de paridade morfológica, sem
contrato público, novo modo, mudança de fase ou quebra de compatibilidade;
segue fluxo contínuo L0-first + resselo + RED→GREEN.

## Regime segregado

Protocolo completo: intenção/L0 → contrato → oráculos e mutantes → selo 1.0 →
teste RED in-process → implementação → GREEN e gates → veredito independente.
Como o workspace é partilhado, o resultado só pode dizer “executado sem
atestação de isolamento”.

## Contenção de memória

- proibido executar `watch`, `w`, suíte externa de wiring ou suíte ampla;
- um teste exato in-process e um build, sequenciais, `CARGO_BUILD_JOBS=1`;
- timeout máximo de 120 s por comando dinâmico;
- stdout/stderr em recibos, máximo de 4 MiB por stream e 16 MiB no total;
- zero testes, timeout, OOM ou `Unknown` inesperado impedem PASS.

