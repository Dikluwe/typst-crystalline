# Passo 118.D — Sanity check de L2

**Data**: 2026-04-23
**Objectivo**: confirmar que o L2 materializado no Passo 117 é
apropriado e que não há contaminação inversa.

---

## L2 faz I/O indevido?

### Grep por filesystem, I/O write, print/eprintln

```
grep 'std::fs|std::io::Write|print!|println!|eprint!|eprintln!'
  em 02_shell/src/
```

**Resultado**: **zero matches**.

L2 não lê ficheiros, não escreve em stdout/stderr, não manipula
I/O de escrita.

### Grep por env vars

```
grep 'std::env' em 02_shell/src/
```

**Resultado**: **1 match**

- `02_shell/src/cli.rs:89` — `std::env::var_os("NO_COLOR").is_some()`.

**Legítimo**: ler env var `NO_COLOR` é convenção de CLI; faz
parte de argparsing moderno. ADR-0048 estabelece a precedência
"flag > NO_COLOR > isatty"; o único local que lê `NO_COLOR` é L2
(`resolve_colored`). L4 nem sabe da existência.

Conclusão: **L2 lê env vars (1 caso legítimo), não escreve**.

---

## L2 importa L3?

### Grep por typst_infra

```
grep 'typst_infra' em 02_shell/src/
```

**Resultado**: **zero matches**.

L2 não depende de L3. A dependência correcta é:

```
L4 → L2 → (nada de L3)
L4 → L3
```

L2 fica isolado de I/O concreto. L4 compõe os dois lados.

---

## L2 importa L1?

### Grep por typst_core

```
grep 'typst_core' em 02_shell/src/
```

Verificado: zero matches (confirmado pela ausência em
`02_shell/Cargo.toml` de uso real — a dep `typst-core` é
declarada mas ainda não consumida).

Mesmo com `typst-core` como dep em `Cargo.toml`, L2 **não a
importa**. Isto é aceitável — L2 é argparsing puro hoje; quando
flags como `--root` surgirem, L2 pode importar `FileId` ou `Span`
de L1 para devolver tipos de domínio no `RunIntent`.

---

## `02_shell/Cargo.toml` revisado

```toml
[dependencies]
typst-core = { path = "../01_core" }  # L2 pode usar L1 no futuro.
anyhow     = { workspace = true }       # Erros genéricos.
clap       = { workspace = true }       # ADR-0049 — argparsing.
```

Todas as deps são legítimas:
- `typst-core`: preparada para futuro uso de tipos L1.
- `anyhow`: permitido em qualquer camada excepto L1.
- `clap`: core do role L2 (CLI).

### Não presente (intencional)

- `typst-infra`: **não aparece** — L2 não deve saber de I/O.
  Correcto.

---

## Estrutura de módulos

`02_shell/src/`:
- `lib.rs` (7 linhas, pub mod cli + header).
- `cli.rs` (153 linhas, materializado Passo 117).

Zero outros módulos. Estrutura limpa.

---

## Testes

`02_shell` tem **6 testes** (todos em `cli.rs`):
- `resolve_colored_never_e_false`
- `resolve_colored_always_e_true`
- `resolve_colored_auto_sem_tty_e_false`
- `resolve_colored_auto_com_tty_e_sem_no_color_e_true`
- `resolve_colored_auto_com_no_color_e_false`
- `resolve_colored_always_vence_no_color`

Todos testam `resolve_colored_with` (função pura). Sem env
mutation. Determinísticos.

---

## Conclusão

**L2 pós-Passo 117 está correctamente posicionado.**

- Zero I/O.
- 1 uso legítimo de `std::env::var_os` (NO_COLOR).
- Zero imports de L3 (respeitando isolamento).
- Deps apropriadas (`clap`, `typst-core` preparado, `anyhow`).
- 6 testes determinísticos sobre função pura.

**Nenhum candidato de migração em L2** — contaminação inversa
(código que devia estar em L3/L4 mas escapou para L2) inexistente.
