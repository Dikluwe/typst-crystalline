# ⚖️ ADR-0048: Cores ANSI nos diagnósticos (L3 formata com `bool`, L4 decide)

**Status**: `EM VIGOR`
**Revoga**: nenhuma.
**Nota Passo 117 (ADR-0049)**: camada corrigida — `ColorWhen` e
`resolve_colored_with` vivem agora em L2 (`02_shell/src/cli.rs`),
não em L3. L3 mantém apenas `format_diagnostic(colored: bool)` +
paleta ANSI. Decisões funcionais deste ADR (paleta, precedência
flag > NO_COLOR > isatty) mantêm-se.
**Validado**: Passo 116.E.
**Data**: 2026-04-23
**Autor**: Passo 116
**Complementa**: ADR-0045 (formato de diagnósticos), ADR-0046
(CLI mínima), ADR-0047 (clap).

---

## Contexto

O Passo 111 (ADR-0045) materializou formato gcc/clang para
diagnósticos:

```
input.typ:3:11: warning: text: propriedade 'font' ainda não suportada
  hint: ver ADR-0040 para propriedades cobertas por set text
```

Monocromático. Utilizadores modernos (rustc, clang, gcc)
esperam cores — severity é a primeira distinção visual.

CLI (Passo 113, ADR-0046) invocável; argparsing via `clap`
(Passo 115, ADR-0047). Hora de tornar o output visualmente
rico.

---

## Decisão

### L3 — formatter aceita `colored: bool`

`format_diagnostic` e `drain_diagnostics_to_stderr` ganham
parâmetro `colored: bool`:

```rust
pub fn format_diagnostic(
    diag: &SourceDiagnostic,
    source: &Source,
    source_path: &str,
    colored: bool,
) -> String;

pub fn drain_diagnostics_to_stderr(
    diagnostics: &[SourceDiagnostic],
    source: &Source,
    source_path: &str,
    colored: bool,
);
```

Em `colored = false`, output idêntico ao Passo 111 (regressão
preservada).

### Paleta (escapes ANSI literais)

| Elemento | Escape | Cor |
|----------|--------|-----|
| `error:` | `\x1b[1;31m` … `\x1b[0m` | Vermelho bold |
| `warning:` | `\x1b[1;33m` … `\x1b[0m` | Amarelo bold |
| `path:linha:coluna` | `\x1b[2m` … `\x1b[0m` | Dim |
| message | `\x1b[1m` … `\x1b[0m` | Bold |
| `hint:` | `\x1b[1;36m` … `\x1b[0m` | Ciano bold |

Escapes manuais com constantes nomeadas. Sem deps externas
(`anstyle`, `termcolor`, `colored` rejeitados — 6 constantes
não justificam dep).

### L3 expõe `ColorWhen` + `resolve_colored_with`; L4 compõe

Por respeito a V12 (linter: L4 não cria tipos), o enum e a função
pura **vivem em L3** (`03_infra/src/diagnostic_format.rs`):

```rust
// L3 — 03_infra/src/diagnostic_format.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum ColorWhen { Auto, Always, Never }

pub fn resolve_colored_with(
    choice: &ColorWhen,
    no_color_present: bool,
    is_tty: bool,
) -> bool {
    match choice {
        ColorWhen::Never  => false,
        ColorWhen::Always => true,
        ColorWhen::Auto   => !no_color_present && is_tty,
    }
}
```

Consequência: `03_infra/Cargo.toml` adiciona `clap = { workspace
= true }` como dep. Aceitável — `ColorWhen` é tipo de dados
relevante para formatting (L3 é I/O + formatters), e reutilizar
`clap::ValueEnum` derive evita duplicação.

L4 consume directamente:

```rust
// L4 — 04_wiring/src/main.rs
use typst_infra::diagnostic_format::{ColorWhen, resolve_colored_with};

#[derive(Parser, Debug)]
struct Args {
    // ...
    #[arg(long = "color", value_enum, default_value_t = ColorWhen::Auto)]
    color: ColorWhen,
}

fn resolve_colored(choice: &ColorWhen) -> bool {
    resolve_colored_with(
        choice,
        std::env::var_os("NO_COLOR").is_some(),
        std::io::stderr().is_terminal(),
    )
}
```

**Função pura `resolve_colored_with`** (L3) é testável sem mutar
env. `resolve_colored` (L4) é wrapper thin que só lê env e tty.

### Ordem de precedência

1. **Flag explícita** (`--color=always` ou `--color=never`) — vence tudo.
2. **`NO_COLOR` env var** (quando flag está em `Auto`) — presença activa no-color.
3. **`isatty(stderr)`** (quando flag está em `Auto` e `NO_COLOR` ausente) — default natural.

### Flag `--color` na CLI (L4)

```rust
#[arg(long = "color", value_enum, default_value_t = ColorWhen::Auto)]
color: ColorWhen,
```

Default `Auto`. Sem `default_missing_value` (cores em `--color`
sem valor dariam erro de clap — mais previsível).

---

## Alternativas rejeitadas

### R-1 — `anstyle` / `termcolor` / `colored` crates

**Rejeitada**. 6 constantes ANSI não justificam dep. Manual é
directo, sem overhead de compilação, e o código é auto-documentado.

### R-2 — Cores em L4, L3 produz só bytes

**Rejeitada**. Duplicaria a formatação (L3 já constrói o layout
`path:linha:coluna: severity: mensagem`). Adicionar cores em
camada externa exigiria re-parsing do output.

### R-3 — Detecção `isatty` em L3

**Rejeitada**. L3 é mais puro se não conhece contexto do
terminal. L4 decide com base em argumentos + env + tty; L3
recebe `bool` e aplica.

### R-4 — `clap::ColorChoice` reutilizado

**Rejeitada**. `clap::ColorChoice` controla **cor do próprio
clap** (help output). Reusar para diagnostics é confuso —
semanticamente são coisas distintas. Enum custom `ColorWhen`
explicita intenção.

### R-5 — `default_missing_value = "always"`

**Rejeitada** (por agora). `typst --color` sem valor daria
cores forçadas — comportamento subtil. Exigir valor explícito
é mais previsível; muda de opinião em passo dedicado se
surgir procura.

### R-6 — Paleta com `--color=256` ou themes custom

**Rejeitada**. Escopo V1. Utilizadores com requisitos específicos
(daltonismo) têm `--color=never` e `NO_COLOR`.

---

## Limitações aceites

1. **Windows legacy (pre-Windows 10)** mostram `\x1b[...` literal
   em console. Windows 10+ tem ANSI nativo. Aceitar.
2. **Paleta fixa**. Sem customização.
3. **`NO_COLOR` trata qualquer valor** (incluindo empty string)
   como no-color. Convenção 2024.
4. **`--color=always` em pipe** produz escapes que ninguém
   interpreta se utilizador redirecciona para ficheiro.
   Comportamento desejado — utilizador pediu.

---

## Consequências

### Positivas

1. Diagnostics visualmente ricos — severity óbvia em 50ms em vez
   de scan de palavras.
2. Padrão rustc/clang preservado (vermelho error, amarelo warning,
   ciano hint).
3. `--color` + `NO_COLOR` + isatty seguem convenções de 2024.
4. `resolve_colored_with` função pura — testável sem env mutation.
5. L3 formatter aceita `bool` — futuros callers (LSP, JSON
   exporter) escolhem livremente.

### Negativas

1. Formatter cresce ~20 linhas (constantes + branches if/else).
2. Call sites actualizam (4º parâmetro) — propagação mecânica.
3. Testes 114 passariam assim-assim — o binário em `Command`
   recebe stderr como pipe → `isatty` é `false` → sem cores →
   asserts literais funcionam. Confirmado em 116.C.

### Neutras

1. ADR-0045 intacta — este complementa; formato base inalterado.
2. Pipeline (eval → layout → export_pdf) intacto.
3. Flag `--color` adiciona 1 enum + 1 campo em `Args`. Clap
   gerencia.

---

## Aplicação

Implementado no Passo 116.C — ver
`00_nucleo/materialization/typst-passo-116-relatorio.md`.

ADR promovida a **EM VIGOR** em 116.E.
