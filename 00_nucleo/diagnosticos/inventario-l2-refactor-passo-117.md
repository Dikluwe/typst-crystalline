# Passo 117.A — Inventário do refactor L2

**Data**: 2026-04-23

---

## Parte 1 — Estado actual de L2

### `02_shell/Cargo.toml`

```toml
[package]
name        = "typst-shell"
description = "CLI and formatters for the Typst compiler"
version.workspace    = true
# ...

[dependencies]
typst-core = { path = "../01_core" }
anyhow     = { workspace = true }
```

- **`typst-core` e `anyhow`** — únicas deps.
- **Sem `clap`** — ainda não declarada.

### `02_shell/src/lib.rs`

```rust
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/shell.md
//! @prompt-hash 71b747af
//! @layer L2
//! @updated 2026-03-22
```

- **Apenas header** (5 linhas). Zero código, zero módulos.
- **Gate 117.A**: não disparado — L2 é literalmente stub.

### Estrutura

```
02_shell/
├── Cargo.toml
└── src/
    └── lib.rs  (só header)
```

---

## Parte 2 — Escopo da migração

### De L3 (`03_infra/src/diagnostic_format.rs`)

**A mover**:
- Enum `ColorWhen` (Auto/Always/Never) com docstrings — ~10 linhas.
- Função pura `resolve_colored_with(choice, no_color, is_tty) -> bool` — ~15 linhas.
- 6 testes `resolve_colored_*` (Never, Always, Auto variantes) — ~40 linhas.
- `use clap::ValueEnum` — remover.
- Dep `clap` em Cargo.toml — remover.

**A manter em L3**:
- Constantes ANSI (`ANSI_RED_BOLD`, etc.).
- `format_diagnostic(diag, source, path, colored: bool) -> String`.
- `drain_diagnostics_to_stderr`.
- 7 testes `format_diagnostic_*` (sem cores + com cores).

### De L4 (`04_wiring/src/main.rs`)

**A mover**:
- Struct `Args` com `#[derive(Parser)]` — ~12 linhas.
- Função wrapper `resolve_colored(choice)` — ~8 linhas (lê env + isatty).
- Uso directo de `clap::Parser` — remover.
- Dep `clap` em Cargo.toml — remover.

**A manter em L4**:
- `main()` thin (~25 linhas estimadas finais).
- Imports de L3 (pipeline, diagnostic_format).
- Imports de L2 (cli).

### Total estimado

~85 linhas movidas de L3+L4 para L2. Plus criar `RunIntent` struct
(~8 linhas nova em L2) e função `parse()` (~10 linhas).

---

## Parte 3 — Tamanho do `cli.rs`

Total esperado em `02_shell/src/cli.rs`:

| Secção | Linhas |
|--------|-------:|
| Header + docstrings | ~10 |
| `ColorWhen` enum | ~10 |
| `Args` struct | ~15 |
| `RunIntent` struct | ~8 |
| `parse()` | ~12 |
| `resolve_colored()` wrapper | ~10 |
| `resolve_colored_with()` pura | ~15 |
| 6 testes `resolve_colored_*` | ~45 |
| **Total** | **~125 linhas** |

---

## Parte 4 — Decisão de localização

**Escolha**: `02_shell/src/cli.rs` dedicado.

**Razões**:
1. **>50 linhas** — spec recomenda ficheiro dedicado acima deste
   limite.
2. **Primeiro módulo de L2** — estabelece padrão "L2 organiza por
   domínio" para módulos futuros (ex: formatters de output diferentes,
   wiring intermediário).
3. **`lib.rs` fica como index** — apenas `pub mod cli;` e header.
   Seguir convenção de `03_infra/src/lib.rs`.

---

## Parte 5 — Impacto nos testes

### Redistribuição

- L1: 811 (inalterado).
- **L2: 0 → 6** (`resolve_colored_*` movem de L3 para L2).
- **L3: 207 → 201** (perde os 6 testes; mantém 6 `format_diagnostic_*` + outros).
- L4: 5 tests/cli.rs (inalterado).

**Total workspace**: 811 + 6 + 201 + 5 = **1023** (igual a antes: 811 + 207 + 5 = 1023).

Redistribuição pura — zero mudança no número total.

---

## Parte 6 — Plano de deps

### Diffs de `[dependencies]`

| Crate | Antes | Depois |
|-------|-------|--------|
| `02_shell/Cargo.toml` | sem `clap` | **+`clap = { workspace = true }`** |
| `03_infra/Cargo.toml` | `clap = { workspace = true }` (desde Passo 116) | **remove `clap`** |
| `04_wiring/Cargo.toml` | `clap = { workspace = true }` (desde Passo 115) | **remove `clap`** (L4 não importa `clap` directamente após refactor) |

`clap` continua em `[workspace.dependencies]` (adicionado no Passo
115) — é dep do workspace. Só muda em que `[dependencies]` é usado.

**Resultado líquido**: clap passa de 2 crates (L3+L4) a 1 crate
(L2). Binário final é o mesmo (transitivamente).

### `04_wiring` deps finais

```toml
[dependencies]
typst-core  = { path = "../01_core" }
typst-shell = { path = "../02_shell" }
typst-infra = { path = "../03_infra" }
anyhow      = { workspace = true }
```

Sem `clap`.

---

## Conclusões 117.A

| Decisão | Escolha |
|---------|---------|
| Localização | `02_shell/src/cli.rs` dedicado |
| `lib.rs` | `pub mod cli;` + header |
| `clap` em L2 | Sim |
| `clap` em L3 | **Remover** |
| `clap` em L4 | **Remover** |
| Linhas estimadas `cli.rs` | ~125 |
| Linhas finais `main.rs` | ~35 |
| Total testes workspace | 1023 (inalterado) |

Gate 117.A **não dispara**. L2 está limpo para receber a migração.
