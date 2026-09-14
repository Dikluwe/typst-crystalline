# Passo 1363 — timestamp `[HH:MM:SS]` no status do `watch`

## Objetivo

Completar o envelope textual mínimo dos status P1362 com hora local em dois
dígitos, mantendo formatação testável por componentes literais e sem executar
`watch` durante a materialização.

## Contrato

L2 acrescenta:

```rust
pub fn format_watch_line(
    hour: u8,
    minute: u8,
    second: u8,
    status: &WatchStatus,
) -> String;
```

Para componentes válidos `0..=23`, `0..=59`, `0..=59`, retorna exatamente:

```text
[03:04:05] compiling ...
[03:04:05] compiled successfully in 2.85 s
[03:04:05] compiled with warnings in 2.85 s
[03:04:05] compiled with errors
```

Valores inválidos são precondição do chamador e ficam fora da API observada;
L4 fornece componentes de `time::Time`, que são válidos por construção.

L4 declara `time = { workspace = true }`, obtém
`OffsetDateTime::now_local()`, usa UTC somente se o offset local estiver
indisponível, e delega toda string a L2. Um helper puro `watch_line_at(status,
Time)` permite teste determinístico; `run_watch` chama apenas o helper `now`.

## Scope-out

- cor, fullscreen e limpeza/movimento de cursor;
- data, timezone, offset, subseguundos e locale;
- posição perante diagnósticos, buffering e flush;
- polling, dependências, staging, arm/publish/abandon/wait/eviction;
- execução externa de `watch`/`w`;
- promessa de qual instante exato é capturado dentro do segundo.

## Owners

- `shell/duration.md` → `02_shell/src/duration.rs`;
- `shell/tests/p1361-duration.md` → `02_shell/tests/p1361_duration.rs`;
- `wiring.md` → `04_wiring/src/main.rs`;
- `04_wiring/Cargo.toml` apenas declara dependência já pinada no workspace.

## Regime e ordem

Protocolo completo, pois altera o envelope padrão de `watch`. Congelar L0 e
digests normativos → contrato → oráculos/mutantes → selo 1.0 → RED in-process
→ implementação → GREEN/gates → veredito. O ambiente partilhado exige a frase
“executado sem atestação de isolamento”. Resselo pode alterar somente a linha
`Hash do Código`; digests normativos excluem exclusivamente essa linha.

## Contenção de memória

- processo `watch`/`w` e suíte externa de wiring: proibidos;
- dois testes exatos sequenciais, `CARGO_BUILD_JOBS=1`, timeout <=120 s;
- stdout/stderr em ficheiros, 4 MiB por stream e 16 MiB total;
- sem suíte ampla, corpus ou relógio real em expected;
- zero testes, timeout, OOM ou Unknown inesperado bloqueiam PASS.

## Gate ADR-0127

O prefixo muda comportamento por defeito. A instrução explícita do dono para
escrever e implementar o próximo passo autoriza apenas este envelope mínimo;
os scope-outs não são promovidos por implicação.
