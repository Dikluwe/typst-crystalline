# ⚖️ ADR-0049: CLI vive em L2 (correcção de ADRs 0046/0047/0048)

**Status**: `EM VIGOR`
**Revoga**: nenhuma (correcção parcial, não revogação total).
**Nota Passo 119 (ADR-0050)**: migração completada.
`format_diagnostic` + paleta ANSI movidos para L2
(`02_shell/src/diagnostic.rs`); `drain_diagnostics_to_stderr`
eliminado (inline em L4 como helper local). L3 perde
`diagnostic_format.rs` completamente.
**Validado**: Passo 117.E.
**Data**: 2026-04-23
**Autor**: Passo 117
**Corrige**: ADR-0046 (CLI mínima), ADR-0047 (clap), ADR-0048
(cores ANSI) — especificamente a **camada** onde a CLI vive.
**Complementa**: decisões funcionais dessas ADRs (clap, cores,
formato gcc/clang) **mantêm-se**.

---

## Contexto

A definição fundacional do projecto em `typst-migracao-estado.md`
estabelece que **L2 (`02_shell/`)** é a camada de "CLI — interface
com utilizador". Esta definição foi **ignorada** por 4 passos
consecutivos:

- **Passo 113 (ADR-0046)**: CLI criada em `04_wiring/src/main.rs`
  com argparsing manual. L2 não foi considerado.
- **Passo 115 (ADR-0047)**: clap adicionado a L4. L2 ainda vazio.
- **Passo 116 (ADR-0048)**: V12 do linter disparou ("L4 não cria
  tipos"); correcção moveu `ColorWhen` para L3 (resistência menor)
  em vez de L2.

**Sintomas actuais** (pré-117):

- `02_shell/` é stub (só header, sem código) desde o Passo 0.
- `03_infra/` depende de `clap` (anti-padrão: L3 é I/O, não argparser).
- `04_wiring/` tem `Args` + argparsing inline (anti-padrão V12
  mascarado — o enum `ColorWhen` foi movido mas `Args` struct
  ficou em L4).
- A definição fundacional de L2 continua não honrada.

**Este passo corrige arquitecturalmente sem alterar funcionalidade.**

---

## Decisão

### L2 (`02_shell/src/cli.rs`) — novo módulo

L2 ganha conteúdo real (primeira vez desde Passo 0):

- **`Args`** com `#[derive(clap::Parser)]` — toda a superfície de
  argparsing.
- **`ColorWhen`** enum com `#[derive(clap::ValueEnum)]`.
- **`RunIntent`** struct pura: `{ input, output, colored }` — o
  output de L2 para L4.
- **`cli::parse() -> RunIntent`** — ponto de entrada público.
  Traduz argumentos + env + isatty em `RunIntent`.
- **`resolve_colored_with(choice, no_color, is_tty) -> bool`** —
  função pura, testável.
- **`resolve_colored(choice)`** — wrapper thin que lê env + isatty.

### L3 (`03_infra/src/diagnostic_format.rs`) — simplifica

- **Mantém**: constantes ANSI, `format_diagnostic(diag, source,
  path, colored: bool)`, `drain_diagnostics_to_stderr`.
- **Remove**: `ColorWhen` enum, `resolve_colored_with`, testes
  relacionados.
- **Remove dep `clap`** de `Cargo.toml` — L3 é I/O puro.

### L4 (`04_wiring/src/main.rs`) — thin

```rust
fn main() -> ExitCode {
    let RunIntent { input, output, colored } = typst_shell::cli::parse();
    // ... pipeline com `colored` propagado ...
}
```

- Desaparece: `Args`, `ColorWhen`, `resolve_colored`.
- Desaparece: `use clap::Parser`.
- **Remove dep `clap`** de `Cargo.toml`.
- L4 fica ~35 linhas de composição pura.

### Workspace deps

`clap` continua em `[workspace.dependencies]` (Passo 115). Muda
**onde é consumido**:

- Antes: L3 + L4.
- Depois: apenas L2.

---

## Relação com ADRs 0046/0047/0048

Este ADR **não revoga** as ADRs anteriores. Corrige especificamente
a decisão de **camada**:

| ADR | Decisão funcional (mantida) | Decisão de camada (corrigida) |
|-----|------------------------------|-------------------------------|
| ADR-0046 | CLI mínima, positional, pipeline SystemWorld → eval → layout → export_pdf | ~~Em L4~~ → **L2 (parse) + L4 (composição)** |
| ADR-0047 | `clap 4` com `derive`, features `derive` apenas | ~~`clap` em L4~~ → **`clap` em L2** |
| ADR-0048 | Paleta ANSI, `--color=auto\|always\|never`, `NO_COLOR`, isatty | ~~`ColorWhen` em L3~~ → **`ColorWhen` em L2** |

As ADRs 0046/0047/0048 ganham nota:

> **Nota do Passo 117 (ADR-0049)**: camada corrigida — CLI e `ColorWhen`
> vivem agora em L2 (`02_shell/`). Decisões funcionais (clap, cores,
> paleta, flags) mantêm-se.

---

## Alternativas rejeitadas

### R-1 — Manter em L3/L4, aceitar V12 como warning

**Rejeitada**. Contradiz definição fundacional de L2. Qualquer
passo futuro que tente respeitar L2 enfrenta a mesma questão.
Tratar V12 como warning permanente normaliza violação.

### R-2 — Revogar ADRs 0046/0047/0048 completamente

**Rejeitada**. Agressivo demais. Decisões base (usar clap, cores
ANSI, paleta, precedência flag > NO_COLOR > isatty) são correctas
e foram tomadas com razões sólidas. Só a camada estava errada.

### R-3 — Deixar `clap` em L4 e expor `Args` via L2

**Rejeitada**. Fragmentação artificial. L2 é quem conhece CLI;
encapsulamento completo significa `clap` também lá.

### R-4 — Criar feature flag `clap` em L3 e L2

**Rejeitada**. Feature flags para algo que não vai alternar é
overhead. L3 simplesmente não precisa de `clap` após a correcção.

---

## Limitações aceites

1. **L4 passa a depender de L2** (`typst-shell`). Já era dep
   transitiva via `typst-wiring/Cargo.toml` — só muda o uso real.
2. **L2 fica com `clap`** — primeira dep "de qualidade de vida"
   em L2. Consistente com propósito "CLI".
3. **`RunIntent` é novo tipo** — criação legítima em L2
   (compare: L2 é onde tipos de CLI vivem naturalmente).

---

## Consequências

### Positivas

1. **Arquitectura honrada**: L2 é o que a definição fundacional
   diz — CLI.
2. **L3 mais puro**: sem `clap`, só I/O e formatação.
3. **L4 thin**: ~35 linhas de composição; adição de futuras flags
   não polui L4.
4. **V12 desaparece naturalmente**: L4 não cria tipos; L2 cria-os
   onde é esperado.
5. **Padrão estabelecido**: próximos flags/subcomandos entram
   em L2, não em L4.

### Negativas

1. **Refactor de 4 ficheiros** (02_shell, 03_infra, 04_wiring,
   Cargo.toml de cada).
2. **ADRs 0046-0048 ganham anotações de correcção** — overhead
   histórico que é necessário para rastreabilidade.
3. **Testes redistribuem** (6 de L3 movem para L2) — contagem
   total inalterada mas por crate muda.

### Neutras

1. **Zero mudança observável** para utilizador externo. `typst
   input.typ output.pdf`, `--color=auto|always|never`, `NO_COLOR`,
   `--help`, `--version` — tudo igual.
2. **Binário final inalterado** em tamanho e comportamento.
3. **Tests 114** continuam a passar sem modificação.

---

## Aplicação

Implementado no Passo 117.C — ver
`00_nucleo/materialization/typst-passo-117-relatorio.md`.

ADR promovida a **EM VIGOR** em 117.E.
