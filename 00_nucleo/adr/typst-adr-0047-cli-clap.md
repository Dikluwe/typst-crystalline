# ⚖️ ADR-0047: Argparsing com `clap` na CLI

**Status**: `EM VIGOR`
**Revoga**: nenhuma.
**Nota Passo 117 (ADR-0049)**: camada corrigida — `clap` consumido
em L2 (`02_shell/`), não em L4. Decisões funcionais deste ADR
(clap 4, derive, version automático) mantêm-se.
**Validado**: Passo 115.E.
**Data**: 2026-04-23
**Autor**: Passo 115
**Complementa**: ADR-0046 (CLI mínima).

---

## Contexto

A CLI nasceu no Passo 113 (ADR-0046) com argparsing manual
(`std::env::args` + `match`). Era a escolha certa no MVP porque
`clap` não estava em `[workspace.dependencies]` e o escopo era
literalmente 2 positional args.

O Passo 114 automatizou as validações manuais em testes de
integração (5 casos em `04_wiring/tests/cli.rs`). Com esse safety
net, migrar para clap torna-se refactor seguro.

**Motivação**:
- Crescer a CLI com argparsing manual é inviável — cada flag nova
  duplica parsing code.
- `clap` é o standard Rust, com `derive` macros idiomáticas.
- Vanilla Typst usa `clap = "4.4"` com `derive` em
  `typst-cli/Cargo.toml` (verificado em 115.A).
- **Benefícios imediatos gratuitos**: `--help` e `--version`
  automáticos; mensagens de erro formatadas.

---

## Decisão

### Dependência

Adicionar ao workspace raiz:

```toml
[workspace.dependencies]
clap = { version = "4", features = ["derive"] }
```

Adicionar a `04_wiring/Cargo.toml`:

```toml
[dependencies]
clap = { workspace = true }
```

### Estrutura Args

```rust
use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "typst",
    version,
    about = "Typst compiler (crystalline)"
)]
struct Args {
    /// Input .typ file
    input: PathBuf,
    /// Output PDF file
    output: PathBuf,
}
```

### Escopo de flags funcionais — **(a) Mínimo** (decidido em 115.A)

- `--help` automático.
- `--version` automático (lê `Cargo.toml` via `version = true`).
- Positional `input` + `output` mantidos exactamente como no 113.
- **Sem** `-o/--output`, `--root`, `--font-path`, `-f/--format`,
  `--color`. Cada uma é passo dedicado futuro.

### Compatibilidade com testes 114 — **(A) Manter positional**

Tests 114 **inalterados** — a CLI continua a aceitar
`typst input.typ output.pdf` como antes. Clap substitui apenas o
parser interno; o contrato externo (exit codes, positional args,
mensagens em stderr) não muda.

### Versão features

- Só `derive` por agora.
- **Não** incluir `env`, `wrap_help`, `string` (usadas por
  vanilla) — adiadas até aparecerem no código.

---

## Alternativas rejeitadas

### R-1 — Manter argparsing manual

Rejeitada. Não escala para flags/subcomandos. Cada flag
duplicaria dezenas de linhas de match.

### R-2 — `argh` / `pico-args`

Ambos mais leves que clap. Rejeitados:
- Menos idiomáticos no ecossistema Rust.
- Vanilla usa clap; alinhamento ADR-0033 (paridade funcional
  preferida quando gratuita).
- Derive macros de clap dão docstrings → help automaticamente.

### R-3 — Escopo (b) com `-o/--output`

Rejeitada em 115.A:
- Divergiria de vanilla (que usa positional output, não flag).
- Dupla aceitação (positional **ou** flag) é confusa em clap.
- Migrar tests 114 (`-o` em vez de positional) por razão
  estética não justifica.

### R-4 — Escopo (c) com `--root` e `--font-path`

Rejeitada em 115.A:
- Cada flag requer integração semântica própria (`SystemWorld::new(root, main)`;
  `discover_fonts` + `SystemWorld::with_fonts`).
- Cresce escopo acima do passo médio.
- Passos dedicados futuros.

### R-5 — Features `env`, `wrap_help`, `string` já no workspace

Rejeitada:
- `env`: usado por vanilla para `TYPST_CERT`, `TYPST_ROOT`, etc.
  Nenhuma flag com `env` no cristalino hoje.
- `wrap_help`: vanilla gerencia output-width dinamicamente. Não
  implementado neste passo.
- `string`: usado em `typst-cli` vanilla para custom styling.
  Não necessário.

Todas adiáveis. Princípio: só adicionar features quando o código
as usa.

---

## Limitações aceites

1. **Sem subcomandos** — CLI continua "flat" (`typst <input>
   <output>`). Vanilla tem 9 subcomandos; cristalino adopta-os
   em passos dedicados.
2. **Sem `--root`, `--font-path`, `-f/--format`, `--color`,
   `-o/--output`** — cada uma passo dedicado.
3. **Sem features extras de clap** (`env`, `wrap_help`, `string`).
   Adicionar quando forem usadas.
4. **`version` vem de `Cargo.toml`** — que hoje é
   `version.workspace = true` → `"0.1.0"`. Sem `--version`
   custom.

---

## Consequências

### Positivas

1. `typst --help` e `typst --version` automáticos — UX básica
   que 113 não tinha.
2. Mensagens de erro de args formatadas por clap (gcc-like para
   missing args).
3. Terreno preparado para adicionar flags em passos futuros — só
   `#[arg(short, long)]` no `Args`.
4. Alinhamento parcial com vanilla (idioma clap + positional).
5. Tests 114 **não tocam** — regressão garantida sem ajustes.

### Negativas

1. Nova dep no workspace (`clap`). Compilação ligeiramente mais
   lenta (clap é dep notório).
2. Binário maior (clap adiciona ~300KB).
3. Primeira dep de "qualidade de vida" pura (não I/O, não dados)
   em L4.

### Neutras

1. ADR-0046 mantém-se válida — este ADR complementa sem revogar.
2. Tests 114 mantêm-se (inalterados — compat A).
3. Pipeline eval → layout → export_pdf intacto.

---

## Aplicação

Implementado no Passo 115.C — ver
`00_nucleo/materialization/typst-passo-115-relatorio.md`.

ADR promovida a **EM VIGOR** em 115.E.
