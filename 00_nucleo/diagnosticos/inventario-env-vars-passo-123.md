# Passo 123.A — Inventário env vars + delimiter

**Data**: 2026-04-24

---

## Parte 1 — clap feature `env`

clap 4 tem feature **`env`** standard. Actualmente o workspace
declara:

```toml
clap = { version = "4", features = ["derive"] }
```

Precisa ganhar `"env"`:

```toml
clap = { version = "4", features = ["derive", "env"] }
```

Quando feature `env` está activa, `#[arg(..., env = "VAR")]`
passa a ler da variável de ambiente se a flag não for passada,
e `--help` mostra `[env: VAR]` automaticamente.

---

## Parte 2 — Vanilla (`lab/.../typst-cli/src/args.rs`)

### Constante (linha 23)

```rust
const ENV_PATH_SEP: char = if cfg!(windows) { ';' } else { ':' };
```

**Expressão const com `if cfg!(...)`** — compila uma vez por
target, sem `#[cfg(...)]` duplicado. Mais compacto que a
alternativa `#[cfg(unix)] const ... = ':'; #[cfg(windows)] const ... = ';';`.

### `--root` (linha 393)

```rust
#[clap(long = "root", env = "TYPST_ROOT", value_name = "DIR")]
pub root: Option<PathBuf>,
```

Apenas `env` adicionado ao existente.

### `--font-path` (linhas 466-472)

```rust
#[clap(
    long = "font-path",
    env = "TYPST_FONT_PATHS",
    value_name = "DIR",
    value_delimiter = ENV_PATH_SEP,
)]
pub font_paths: Vec<PathBuf>,
```

- `env` + `value_delimiter` combinam.
- **Sem `action = Append`** na vanilla — com `Vec<T>` + `value_delimiter`,
  clap já aceita múltiplas ocorrências da flag.
- No cristalino mantemos `ArgAction::Append` do Passo 122 para
  preservar UX repetível — testar que combina com `value_delimiter`.

### Outros env vars em vanilla (referência)

`TYPST_CERT`, `TYPST_UPDATE_BACKUP_PATH`, `SOURCE_DATE_EPOCH`,
`TYPST_FEATURES`, `TYPST_PACKAGE_PATH`, `TYPST_PACKAGE_CACHE_PATH`,
`TYPST_IGNORE_SYSTEM_FONTS`, `TYPST_IGNORE_EMBEDDED_FONTS`. Este
passo só cobre `TYPST_ROOT` + `TYPST_FONT_PATHS` (flags já
existentes em cristalino).

---

## Parte 3 — Sintaxe clap: `env` + `value_delimiter` + `Append`

Compatíveis. Clap derive aceita os três atributos juntos:

```rust
#[arg(
    long = "font-path",
    env = "TYPST_FONT_PATHS",
    value_name = "DIR",
    value_delimiter = ENV_PATH_SEP,
    action = clap::ArgAction::Append,
)]
font_paths: Vec<PathBuf>,
```

Semântica combinada:
- **Flag repetível**: `--font-path /a --font-path /b` → `[/a, /b]`.
- **Env var com delimiter**: `TYPST_FONT_PATHS=/a:/b` → `[/a, /b]`.
- **Precedência clap padrão**: flag > env > default.
- **Combinar**: flag explícita overrides env inteiramente (não
  concatena). Comportamento clap standard.

---

## Parte 4 — `ENV_PATH_SEP` em cristalino

Duas opções:

**(a)** **Expressão const `if cfg!(windows)`** (vanilla):
```rust
const ENV_PATH_SEP: char = if cfg!(windows) { ';' } else { ':' };
```

**(b)** **`#[cfg]` duplicado**:
```rust
#[cfg(unix)]    const ENV_PATH_SEP: char = ':';
#[cfg(windows)] const ENV_PATH_SEP: char = ';';
```

**Decisão**: **(a)** — alinhada com vanilla, 1 linha, zero
duplicação. Vive em `02_shell/src/cli.rs` (privada ao módulo).

Nota: clap derive aceita `value_delimiter = ENV_PATH_SEP`
(identificador const), não só literal — confirmado pelo uso
vanilla.

---

## Parte 5 — Testes

Testes existentes em `04_wiring/tests/cli.rs` não usam
`.env(...)` em `Command` — isolados por processo.

Testes novos podem usar `Command::env("TYPST_ROOT", ...)` —
variável só visível ao subprocess, não polui o runner `cargo
test`. Cross-test paralelismo OK.

`.env("TYPST_FONT_PATHS", "/a:/b")` em Unix, com literal `':'`
no string construído pelo test. Para cross-platform usar
`std::env::consts` ou inline `cfg!`:
```rust
let sep = if cfg!(windows) { ';' } else { ':' };
```

---

## Decisões finais

| Dimensão | Escolha |
|----------|---------|
| Feature clap | **`env`** (nome standard) |
| `ENV_PATH_SEP` | **const inline `if cfg!`** (vanilla-style) |
| Local do const | `02_shell/src/cli.rs` (privada) |
| `--root` | `env = "TYPST_ROOT"` |
| `--font-path` | `env = "TYPST_FONT_PATHS"` + `value_delimiter = ENV_PATH_SEP` + manter `ArgAction::Append` |
| ADR-0051 | **Sem anotação** — P1 só ganha atributos; P2–P6 inalterados |
| L1/L3/L4 | Inalterados |
| `RunIntent` | Inalterado (clap preenche `args.*` transparentemente) |

---

## Tamanho estimado

**Pequeno**:
- Cargo.toml: +1 token (`"env"`).
- cli.rs: +1 const, +1 linha em `root`, +2 linhas em `font_paths`.
- Tests L4: +3 testes.
- Prompt: pequena secção sobre env vars.
