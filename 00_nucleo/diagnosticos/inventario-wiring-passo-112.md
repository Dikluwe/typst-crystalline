# Passo 112.A — Inventário do `04_wiring/`

**Data**: 2026-04-23
**Propósito**: caracterizar o estado actual do L4 antes de propor
escopo para a CLI real.

---

## `04_wiring/src/main.rs`

```rust
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring.md
//! @prompt-hash dd8fc37a
//! @layer L4
//! @updated 2026-03-22

fn main() {
    // Composição será implementada durante a migração
    println!("typst cristalino — em migração");
}
```

- **Linhas totais**: 11 (6 header + 1 linha em branco + `fn main()` de 3 linhas).
- **Estado**: **stub**. Sem lógica real.
- **Imports**: nenhum.

## `04_wiring/Cargo.toml`

Binary:

```toml
[[bin]]
name = "typst"
path = "src/main.rs"
```

Deps:

```toml
[dependencies]
typst-core  = { path = "../01_core" }
typst-shell = { path = "../02_shell" }
typst-infra = { path = "../03_infra" }
anyhow      = { workspace = true }
```

- 3 crates internas + `anyhow`.
- **Sem argparsing lib** (clap, argh, structopt ausentes).
- **Sem watch** (notify ausente).

---

## `02_shell/src/lib.rs`

```rust
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/shell.md
//! @prompt-hash 71b747af
//! @layer L2
//! @updated 2026-03-22
```

- **L2 é vazio** (apenas header). Nenhum código.
- Não contribui com nada para a CLI hoje — 04_wiring usa L1 + L3 directamente.

---

## `Cargo.toml` workspace — deps disponíveis

```toml
[workspace.dependencies]
thiserror  = "2"
comemo     = "0.4"
rustc-hash = "2"
ttf-parser = "0.25"
rustybuzz  = "0.20"
time       = { version = "0.3", features = ["macros"] }
indexmap   = { version = "2" }
ecow       = "0.2"
anyhow     = "1"
unicode-ident       = "1"
unicode-math-class  = "0.1"
unicode-script      = "0.5"
unicode-segmentation = "1"
```

**Deps externas disponíveis mas não usadas pelo 04_wiring hoje**:
- `thiserror` (error derive)
- `ecow` (copy-on-write strings)
- Outras (comemo, rustc-hash, ttf-parser, rustybuzz, time, indexmap, unicode-*) — todas usadas por L1 ou L3, não pelo L4 directamente.

**Não disponíveis no workspace** (teriam de ser adicionadas):
- `clap` — argparsing declarativo (vanilla usa).
- `argh` — alternativa mais leve a clap.
- `notify` — file watching para `watch` command.
- `codespan-reporting` — diagnostics ricos com cores/snippets (vanilla usa para formato — o cristalino tem formatter próprio no Passo 111).
- `color-print` — templates de ajuda coloridos.

---

## Conclusões 112.A

1. **Ponto de partida é zero real** — `main.rs` só imprime.
2. **`anyhow` já disponível** — `Result<T, anyhow::Error>` directo no binário.
3. **Argparsing requer decisão**: manual (std::env::args) vs clap (nova dep no workspace).
4. **L2 não existe** — CLI vai invocar L3 e L1 directamente do L4.
