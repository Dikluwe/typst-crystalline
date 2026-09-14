# Passo 1367 — flush explícito dos blocos de status

## Objetivo

Garantir que cada bloco `watch` seja escrito integralmente e descarregado em
stderr antes da continuação do ciclo.

## Contrato privado

L4 acrescenta:

```rust
fn write_watch_block(
    out: &mut impl std::io::Write,
    block: &str,
) -> std::io::Result<()>;

fn emit_watch_block(block: &str);
```

`write_watch_block` chama `write_all` exatamente uma vez com os bytes integrais
do bloco e depois `flush` exatamente uma vez. Se a escrita falhar, devolve o
erro sem chamar flush; se o flush falhar, devolve esse erro. Não acrescenta,
remove ou transforma bytes.

`emit_watch_block` bloqueia `stderr`, chama o helper e falha de forma explícita
se o stream rejeitar escrita/flush. Os dois `eprint!` exclusivos de status em
`run_watch`/callback P1366 são substituídos pelo wrapper; nenhum diagnóstico
usa essa fronteira.

## Teste

Um único teste exato usa writers falsos in-memory para provar bytes, ordem
`write → flush`, cardinalidade, ausência de flush após erro de escrita e
propagação de erro de flush. Não usa relógio, filesystem, stderr real, processo
ou watcher.

## L0 e ADR-0127

Owner exclusivo: `00_nucleo/prompts/wiring.md` →
`04_wiring/src/main.rs`. É correção interna da entrega temporal do status, sem
API pública, default, fase ou incompatibilidade; fluxo contínuo.

## Scope-out

- cor, fullscreen, cursor, limpeza e detecção TTY;
- buffering interno do sistema operacional depois de `flush`;
- conteúdo e flush dos diagnósticos não-watch;
- polling, staging, publish, abandon, wait e eviction;
- execução externa de `watch`/`w`.

## Regime e memória

Protocolo completo, executado sem atestação de isolamento. Proibido executar
`watch`, `w`, suíte externa de wiring ou suíte ampla. Um teste exato e um
build, sequenciais, `CARGO_BUILD_JOBS=1`, timeout máximo 120 s, 4 MiB por
stream e 16 MiB agregado.

