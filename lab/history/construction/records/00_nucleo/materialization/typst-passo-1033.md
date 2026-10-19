# Passo 1033 — Fix: `--version` do binário não reflecte o commit compilado

**Tipo**: Correcção de infra-estrutura, sem gate (não é comportamento de utilizador, é
proveniência de build). Prioridade alta — mina a confiança em qualquer medição anterior
que tenha citado `--version` como prova de que binário foi usado.
**Achado (Passo 1031)**: `02_shell/build.rs` só declara
`rerun-if-env-changed=TYPST_COMMIT_SHA` (cópia fiel do vanilla,
`lab/typst-original/crates/typst-utils/build.rs`) — o `cargo` não reexecuta o script
quando o HEAD muda sem essa env var mudar, logo a string de versão fica presa ao primeiro
build daquele directório `target/`. Reproduzido: build em HEAD `4f64e4e69` imprimiu
`typst 0.15.0 (0f8487b9)` — hash errado.
**Pré-condição**: `git status` limpo.

---

## Fase A — Confirmar o mecanismo exacto

1. Ler `02_shell/build.rs` na íntegra — confirmar a lógica actual de obtenção do hash
   (via `TYPST_COMMIT_SHA` env var, ou fallback a `git rev-parse` nalgum ponto).
2. Confirmar por que o vanilla tem este comportamento e se é aceitável **lá** (talvez o
   fluxo de release deles sempre define a env var explicitamente, nunca dependendo de
   rebuild automático por mudança de HEAD) — isto não invalida que seja um problema
   **aqui**, onde rebuilds frequentes em HEADs diferentes são o normal do fluxo de passos.

## Fase B — Corrigir

Adicionar instrução a `build.rs` para que o cargo reexecute o script sempre que o `HEAD`
do git mudar, não só quando a env var mudar:
```rust
println!("cargo:rerun-if-changed=../.git/HEAD");
println!("cargo:rerun-if-changed=../.git/refs");
```
(ajustar caminhos ao real, `.git` pode estar noutro nível relativo a `02_shell/`).
Confirmar que isto não quebra paridade com o vanilla de forma que importe — é aditivo
(mais gatilhos de rebuild), não muda o output do binário em si, só quando o build.rs
corre.

## Fase C — Validar

1. Build num HEAD, confirmar `--version` bate.
2. Sem tocar em código, avançar HEAD (checkout doutro commit ou commit novo), rebuild,
   confirmar que `--version` **mudou** para o hash novo sem precisar de `cargo clean`.
3. `cargo test --workspace` — zero regressão (mudança só em `build.rs`, não em lógica).

## Nota para relatórios futuros

Enquanto este fix não estiver confirmado e generalizado (pode levar tempo até todos os
binários em uso serem rebuilds limpos), **a proveniência de medições continua a usar HEAD
+ estado da árvore, nunca a string `--version` sozinha** — regra já adoptada
informalmente no P1031, tornada explícita aqui.

---

## Resultado esperado

`--version` reflecte sempre o commit real após rebuild, sem precisar de `cargo clean`.
Nota de proveniência actualizada (`AGENTS.md` e `CLAUDE.md`) a registar esta como a quarta
armadilha de binário, resolvida.

---

## Relatório

O relatório completo está em:

`00_nucleo/diagnosticos/typst-passo-1033-relatorio.md`

## Validação rápida

```text
cargo build --release -p typst-wiring -> ok
./target/release/typst --version      -> typst 0.15.0 (<hash-do-HEAD-actual>)
cargo test --workspace                -> ok (~5855 passed, 0 failed)
crystalline-lint .                    -> 0 violations
```
