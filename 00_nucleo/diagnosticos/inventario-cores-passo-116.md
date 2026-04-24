# Passo 116.A — Inventário de cores ANSI

**Data**: 2026-04-23

---

## Parte 1 — MSRV

- `Cargo.toml` raiz: **sem `rust-version`** declarado. Apenas
  `edition = "2021"` (requer 1.56+).
- Sem `rust-toolchain.toml` nem `rust-toolchain` no repo.
- Toolchain instalada localmente: `rustc 1.92.0`.

### `std::io::IsTerminal` disponibilidade

Estabilizado em **Rust 1.70** (2023-06). Como não há MSRV
declarado e o ecosistema moderno está em 1.70+, `IsTerminal` é
usável sem gate adicional.

### Decisão

- **Usar `std::io::IsTerminal`** directamente.
- Não adicionar `rust-version` ao workspace neste passo (fora do
  escopo; se algum utilizador actual tiver rustc < 1.70, outro
  passo trataria).
- Nenhuma dep nova (`is-terminal` crate rejeitado).

**Gate 116.A não dispara.**

---

## Parte 2 — Paleta ANSI

Sequências escolhidas (padrão terminal ANSI/ECMA-48):

| Nome | Escape | Significado |
|------|--------|-------------|
| `ANSI_RED_BOLD` | `\x1b[1;31m` | Vermelho bold — `error:` |
| `ANSI_YELLOW_BOLD` | `\x1b[1;33m` | Amarelo bold — `warning:` |
| `ANSI_CYAN_BOLD` | `\x1b[1;36m` | Ciano bold — `hint:` |
| `ANSI_DIM` | `\x1b[2m` | Dim — `path:linha:coluna` |
| `ANSI_BOLD` | `\x1b[1m` | Bold — mensagem |
| `ANSI_RESET` | `\x1b[0m` | Reset todos os atributos |

Alinhamento com rustc/clang (vermelho bold para error, amarelo
bold para warning). Ciano para hint é convenção rustc.

### Limitações aceites

- **Windows legacy**: Windows 7/8 consoles sem `ENABLE_VIRTUAL_TERMINAL_PROCESSING`
  mostram escapes literais. Windows 10+ (2016+) renderiza. Aceitar.
- **Paleta fixa**: sem `--color=256`, sem themes custom.
- **Daltonismo**: utilizador pode `--color=never` ou `NO_COLOR=1`.

---

## Parte 3 — `NO_COLOR` convenção

Fonte: <https://no-color.org/>.

Regra: "A command-line software should check for a `NO_COLOR`
environment variable that, **when present and not an empty
string** (regardless of its value), prevents the addition of ANSI
color."

**Revisão 2024** da convenção: "when present regardless of value".
Diferença em `NO_COLOR=` (empty string) — há ambiguidade
histórica. Maioria dos consumidores ignora valor.

### Lógica Rust adoptada

```rust
std::env::var_os("NO_COLOR").is_some()
```

Trata presença (qualquer valor, incluindo vazio) como "no color".
Consistente com 2024 revision e com rustc/cargo.

---

## Parte 4 — Vanilla `--color`

Grep em `lab/typst-original/crates/typst-cli/src/args.rs`:

```rust
/// Whether to use color. When set to `auto` if the terminal to supports it.
#[clap(long, default_value_t = ColorChoice::Auto, default_missing_value = "always")]
pub color: ColorChoice,
```

- **Usa `clap::ColorChoice`** (built-in em clap).
- Variantes: `Auto`, `Always`, `Never`.
- Default: `Auto`.
- `default_missing_value = "always"` — `typst --color` (sem valor)
  equivale a `--color=always`.

### Decisão para cristalino

**Definir `ColorWhen` custom** em `04_wiring/src/main.rs` em vez
de reusar `clap::ColorChoice`. Razões:

1. **`clap::ColorChoice` controla cor do próprio clap** (help
   output), **não** do nosso formatter. Semanticamente confuso
   reusar.
2. `ColorWhen` custom é trivial (3 variantes + `ValueEnum` derive).
3. Explicita a intenção no código — `ColorWhen::Auto` é sobre
   diagnostics do compilador, não sobre help do clap.

### Decisão sobre `default_missing_value`

**Não adoptar** neste passo. Comportamento esperado: `typst
--color` sem valor deveria dar erro de clap (arg requires value).
Utilizadores que queiram cores forçadas usam `--color=always`.
Mais explícito. Adopção do `default_missing_value` fica para passo
dedicado se surgir procura.

---

## Conclusões 116.A

| Decisão | Escolha | Razão |
|---------|---------|-------|
| MSRV / IsTerminal | Usar `std::io::IsTerminal` directo | rustc 1.92 disponível; `IsTerminal` estável desde 1.70. |
| Paleta | 6 constantes ANSI literais | Sem deps (`anstyle`, `termcolor` rejeitados). Alinha com rustc. |
| NO_COLOR | `env::var_os("NO_COLOR").is_some()` | Convenção 2024. |
| `ColorWhen` | Custom enum L4 | Evita confusão com `clap::ColorChoice` (que controla output do clap). |
| `default_missing_value` | Não usar | Exige valor explícito; mais previsível. |

**Pronto para 116.B (ADR) e 116.C (implementação).**
