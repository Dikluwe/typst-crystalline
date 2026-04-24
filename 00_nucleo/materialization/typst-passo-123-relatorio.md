# Passo 123 — Relatório (env vars `TYPST_ROOT` + `TYPST_FONT_PATHS`)

**Data**: 2026-04-24
**Precondição**: Passo 122 encerrado; 1032 total tests; zero
violations; 51 ADRs activas.
**Natureza**: consolidação. Zero funcionalidade nova em pipeline —
paridade com vanilla em mecanismo de configuração.
**ADR tocada**: **ADR-0051** — sem anotação (P1 só ganha
atributos; P2–P6 inalterados).

---

## Sumário

CLI ganha suporte a env vars `TYPST_ROOT` e `TYPST_FONT_PATHS`
via feature `env` do clap. `--font-path` também passa a aceitar
`value_delimiter = ENV_PATH_SEP` (`:` Unix / `;` Windows) para
suportar formato `PATH`-like.

**Precedência clap standard**: flag > env > default.

**Sem mudança em L1, L3 ou L4** — clap preenche `args.root` e
`args.font_paths` transparentemente; `resolve_root_with` e o
move de `font_paths` para `RunIntent` continuam a funcionar sem
saber da origem.

**Descoberta colateral**: `--help` ganha `[env: TYPST_ROOT=]` e
`[env: TYPST_FONT_PATHS=]` automaticamente — documentação
grátis.

**811 L1 + 24 L2 + 186 L3 + 14 L4 (+3)** + 6 ignorados =
**1035 total** (+3 novos testes). Zero violations. **51 ADRs
activas** (+0).

---

## 123.A — Inventário

Completo em
`00_nucleo/diagnosticos/inventario-env-vars-passo-123.md`.

### Vanilla

```rust
// args.rs:23
const ENV_PATH_SEP: char = if cfg!(windows) { ';' } else { ':' };

// args.rs:393 (--root)
#[clap(long = "root", env = "TYPST_ROOT", value_name = "DIR")]
pub root: Option<PathBuf>,

// args.rs:466-472 (--font-path)
#[clap(
    long = "font-path",
    env = "TYPST_FONT_PATHS",
    value_name = "DIR",
    value_delimiter = ENV_PATH_SEP,
)]
pub font_paths: Vec<PathBuf>,
```

### Decisões-chave

| Dimensão | Escolha | Razão |
|----------|---------|-------|
| Feature clap | `env` | Standard; gera `[env: VAR]` em --help |
| `ENV_PATH_SEP` | `const if cfg!(windows)` | 1 linha, vanilla-style |
| Local const | `02_shell/src/cli.rs` privado | Só L2 precisa |
| `--root` | + `env = "TYPST_ROOT"` | Paridade vanilla |
| `--font-path` | + `env` + `value_delimiter` | Paridade; mantém `ArgAction::Append` do 122 |
| L1/L3/L4 | Intacto | Clap preenche transparentemente |

---

## 123.B — ADR-0051

**Sem anotação.** P1 só ganha atributos `env = "..."` e
`value_delimiter` em campos existentes. P2–P6 inalterados:
`resolve_root_with` continua puro, passagem directa de
`font_paths` continua. Precedência `flag > env > default` é
propriedade do clap, não das funções L2.

---

## 123.C — Implementação

### Diff Cargo.toml raiz

```toml
# ANTES
clap = { version = "4", features = ["derive"] }

# DEPOIS
clap = { version = "4", features = ["derive", "env"] }
```

### Diff `02_shell/src/cli.rs`

```rust
// +
const ENV_PATH_SEP: char = if cfg!(windows) { ';' } else { ':' };

// --root: + env
#[arg(long = "root", env = "TYPST_ROOT", value_name = "DIR")]
root: Option<PathBuf>,

// --font-path: + env + value_delimiter
#[arg(
    long = "font-path",
    env = "TYPST_FONT_PATHS",
    value_name = "DIR",
    value_delimiter = ENV_PATH_SEP,
    action = clap::ArgAction::Append,
)]
font_paths: Vec<PathBuf>,
```

Total: +1 const (4 linhas doc+1 linha code), +2 tokens em `--root`,
+2 tokens em `--font-path`.

### L3/L4 e `RunIntent`: intactos

- `resolve_root_with(args.root.as_ref(), &args.input)` continua
  idêntico — `args.root` já chega preenchido por clap (flag
  ou env).
- `args.font_paths` move directo para `RunIntent.font_paths`.
- L4 `main.rs` sem mudança.

---

## 123.D — Testes L4

### 3 testes novos

**`cli_env_typst_root`**: Passa `file_name` (sem directório) +
`TYPST_ROOT=<tempdir>`. Sem env, `SystemWorld` leria `./file_name.typ`
e falharia. Com env, lê `<tempdir>/file_name.typ` — sucesso.

**`cli_flag_root_vence_env`**: Passa `TYPST_ROOT=/inexistente` +
flag `--root <tempdir>`. Se flag vencer (expectativa), compila
OK. Se env vencer, `SystemWorld::new` falha. **Flag venceu** —
comportamento clap confirmado empiricamente.

**`cli_env_typst_font_paths_delimiter`**: Passa
`TYPST_FONT_PATHS=<dir><sep><dir>` (onde `<sep>` é `:` ou `;`
conforme OS). Clap expande em `Vec::<PathBuf>` com 2 elementos.
Binário compila.

### `cargo test --workspace`

```
test result: ok. 811 passed ...      (L1 inalterado)
test result: ok. 186 passed, 6 ignored (L3 inalterado)
test result: ok. 24 passed  ...      (L2 inalterado)
test result: ok. 14 passed  ...      (L4 +3: env_*, flag_root_vence_env)
```

---

## 123.E — Encerramento

### `crystalline-lint .`

```
✓ No violations found
```

### Validação manual

```bash
$ ./typst --help | grep "\[env:"
          [env: TYPST_ROOT=]
          [env: TYPST_FONT_PATHS=]

$ TYPST_ROOT=/tmp/p123 ./typst env_file.typ -o /tmp/p123/out1.pdf
$ ls -la /tmp/p123/out1.pdf   # 977 bytes ✓

$ TYPST_FONT_PATHS="/tmp:/usr/share/fonts" ./typst env_file.typ \
      -o /tmp/p123/out2.pdf
$ ls -la /tmp/p123/out2.pdf   # 977 bytes ✓
```

### Números finais

| Métrica | Antes (Passo 122) | Depois |
|---------|------:|-------:|
| L1 tests | 811 | 811 |
| L2 tests | 24 | 24 |
| L3 tests | 186 | 186 |
| L4 tests | 11 | **14** (+3) |
| **Total** | **1032** | **1035** (+3) |
| Ignorados | 6 | 6 |
| Violations | 0 | 0 |
| ADRs activas | 51 | 51 |
| DEBTs abertos | 11 | 11 |

---

## Limitações aceites

1. **Sem `TYPST_IGNORE_SYSTEM_FONTS` e outras env vars**:
   fora de escopo. Vanilla tem mais — entram quando flags
   correspondentes chegarem.
2. **Flag substitui env inteiramente (não concatena)**: se
   utilizador define `TYPST_FONT_PATHS=/a:/b` e depois
   `--font-path /c`, só `/c` entra em `font_paths`. Clap
   standard. Documentado.
3. **`[env: TYPST_ROOT=]` em --help aparece mesmo sem valor
   definido**: cosmética de clap. Utilizador lê como "aceita
   esta env var". OK.
4. **Windows delimiter não testado em CI**: `cfg!(windows)`
   funciona mas os testes correm em Linux. `cli_env_typst_font_paths_delimiter`
   usa `if cfg!(windows) { ';' } else { ':' }` para cross-platform
   teoricamente; em CI real cobre só Unix.

---

## Lições

1. **Feature flag grátis produz help grátis**: activar `env`
   do clap deu `[env: VAR]` em `--help` sem escrever uma
   linha de docs. O custo foi 4 caracteres no Cargo.toml.

2. **Const `if cfg!()` > `#[cfg]` duplicado**: vanilla fez a
   escolha certa — `const X: char = if cfg!(windows) { ... }`
   lê como código normal, sem ritual de atributos. Alinhar
   paga-se em consistência entre repos.

3. **Zero mudança em L2 puro**: `resolve_root_with` sem
   tocar é sinal de que a API foi bem desenhada no 121 —
   toma `Option<&PathBuf>` agnóstico à origem (flag ou env).
   Quando clap preenche, a função não nota.

4. **Teste de precedência é teste de confiança**: documentação
   clap diz "flag > env > default". `cli_flag_root_vence_env`
   verifica **empiricamente** porque *um dia* alguém pode
   argumentar o contrário. Custo: 20 linhas de teste; valor:
   confiança em bug report.

5. **`TYPST_ROOT` desbloqueia scripts legacy**: utilizadores
   que invocam `typst` dentro de scripts shell com env já
   exportada beneficiam imediatamente. Valor UX concreto
   apesar da trivialidade.

6. **Evitar `env` feature tinha custo escondido**: sem a
   feature, cada env var seria um `std::env::var(...).ok()`
   em L2 + merge manual com flag. Entregar P3 (flag > env >
   default) consistentemente entre flags tornar-se-ia
   responsabilidade da equipa. Com `env` do clap, a regra
   é *propriedade do framework*. Parcimónia.

---

## Estado pós-Passo 123

### CLI final (help fragment)

```
  -o, --output <FILE>   Output PDF file ...
      --root <DIR>      Project root directory ...
                        [env: TYPST_ROOT=]
      --font-path <DIR> Additional directories to search for fonts.
                        May be repeated. Also accepts a single value
                        with `:` (Unix) / `;` (Windows) as separator,
                        e.g. via `TYPST_FONT_PATHS=/a:/b`.
                        [env: TYPST_FONT_PATHS=]
      --color <COLOR>   ... [default: auto]
```

### Estado ADR-0051 (pós-123)

| Flag | Passo | Env | Delimiter |
|------|------:|:---:|:---------:|
| `-o/--output` | 120 | — | — |
| `--root` | 121 | `TYPST_ROOT` (123) | — |
| `--font-path` | 122 | `TYPST_FONT_PATHS` (123) | `:`/`;` (123) |

### Trabalho futuro identificado

1. **`--ignore-system-fonts`** — quando system font discovery
   existir em L3.
2. **System font discovery** em L3 — via `fontdb` ou stdlib;
   passo dedicado.
3. **`-f/--format`** — export PNG/SVG/HTML; novo escopo.
4. **Warning amigável para `--font-path`/`--root` inválidos** —
   Sink de L3 ou linha stderr de L4; pequeno valor UX.
5. **Mais env vars** (`TYPST_PACKAGE_PATH`, `TYPST_CERT`, etc.)
   — entram com as flags correspondentes.
