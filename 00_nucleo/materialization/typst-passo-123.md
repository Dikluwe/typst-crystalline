# Passo 123 — Env vars `TYPST_ROOT` + `TYPST_FONT_PATHS` + delimiter

**Série**: 123 (passo pequeno; fecha pendência 121/122).
**Precondição**: Passo 122 encerrado; 1032 total tests; zero
violations; 51 ADRs activas.
**ADRs aplicáveis**: ADR-0051 (flags funcionais).
**ADR nova**: **não por default**. Se anotação em ADR-0051 for
necessária (por causa de P1: `#[arg(...)]` ganha `env = "..."`
que modifica ligeiramente o pattern), anotar.

---

## Objectivo

Adicionar suporte a variáveis de ambiente `TYPST_ROOT` e
`TYPST_FONT_PATHS`, alinhando com vanilla. Adicionar
`value_delimiter` (`:` Unix / `;` Windows) ao `--font-path` para
suportar formato `PATH`-like comum em env vars.

Ao fim do passo:

1. `clap` no workspace tem feature `env` activa.
2. `Args.root` declara `env = "TYPST_ROOT"`.
3. `Args.font_paths` declara `env = "TYPST_FONT_PATHS"` + 
   `value_delimiter = ENV_PATH_SEP`.
4. Precedência natural do clap: **flag > env > default**.
5. `resolve_root_with` e `resolve_font_paths_with` (ausente)
   **não mudam** — clap preenche `args.root` / `args.font_paths`
   transparentemente.
6. Testes L4 novos cobrem env var sozinha e env+flag.

Este passo **não**:
- Muda L1 ou L3.
- Cria ADR nova.
- Adiciona novas flags.
- Revê decisões anteriores da ADR-0051.

---

## Decisões já tomadas

1. **Feature `env` em clap workspace** — 1 linha no Cargo.toml
   raiz.
2. **Precedência**: **flag > env > default** (standard clap).
3. **`--font-path` ganha `value_delimiter`** — mantém Append
   mas aceita `:` ou `;` dentro de um valor único (env var
   típica). Clap permite combinar ambos.
4. **Nenhuma mudança em `resolve_root_with`** — clap preenche
   `args.root` com valor de env var se flag ausente.
5. **Constante `ENV_PATH_SEP`** em L2 (ou em `typst-shell::cli`):
   `':'` em Unix, `';'` em Windows. Como vanilla faz.

---

## Escopo

**Dentro**:
- `Cargo.toml` raiz — `clap` feature `env`.
- `02_shell/src/cli.rs` — atributos `env` + `value_delimiter`;
  constante `ENV_PATH_SEP`.
- `04_wiring/tests/cli.rs` — 3-4 testes novos (env sozinha,
  env+flag, delimiter).
- Prompts L0: `shell/cli.md` descreve env vars.

**Fora**:
- Documentação de env vars no `--help` (clap gera
  automaticamente se `env` atributo é declarado — ganha
  grátis).
- Validação de env var values em L2 (P5 ADR-0051).
- Novas flags.
- Mudança em L1 ou L3.

---

## Sub-passos

### 123.A — Inventário

**Parte 1 — clap `env` feature**:

1. Verificar documentação/crate info de clap 4: feature `env`
   existe? Nome exacto?
   - Se sim, feature `env` (confirmar).
2. `view` em `Cargo.toml` raiz. Ver declaração actual:
   ```toml
   clap = { version = "4", features = ["derive"] }
   ```
3. Registar.

**Parte 2 — Vanilla delimiter + env**:

1. `view` em `lab/typst-original/crates/typst-cli/src/args.rs`
   para `--root`:
   - Tem `env = "TYPST_ROOT"`?
2. Para `--font-path`:
   - Tem `env = "TYPST_FONT_PATHS"`?
   - Tem `value_delimiter`? Constante usada?
   - `ENV_PATH_SEP` é constante global ou inline?
3. Registar literalmente.

**Parte 3 — Sintaxe clap para env + delimiter simultâneo**:

Se possível consultar exemplo:

```rust
#[arg(
    long = "font-path",
    env = "TYPST_FONT_PATHS",
    value_name = "DIR",
    value_delimiter = ':',   // ou Unix/Windows
    action = clap::ArgAction::Append,
)]
font_paths: Vec<PathBuf>,
```

Verificar em clap docs/source que `env` + `value_delimiter` +
`Append` combinam sem conflito.

**Parte 4 — `ENV_PATH_SEP` em Rust**:

Opções:
- Constante manual: `#[cfg(unix)] const ENV_PATH_SEP: char = ':';`
- Const `std::path::MAIN_SEPARATOR` — é `/` ou `\`, **não** o
  separador de env. Não serve.
- Vanilla provavelmente tem constante própria. Alinhar ou
  duplicar.

Decidir em 123.A: onde vive a constante em L2.

**Escrever** em `00_nucleo/diagnosticos/inventario-env-vars-passo-123.md`.

**Gate 123.A**: se clap 4 não tem feature `env` (improvável), o
passo muda forma (implementar leitura manual de env vars em L2).
Preferência: feature clap; se ausente, registar e reportar.

### 123.B — ADR

Se a mudança for só "acrescentar `env = "..."`" a campos
existentes + feature flag, **nenhuma ADR nova, nenhuma
anotação**. Pattern P1-P6 aguenta.

Se `value_delimiter` criar divergência interessante da ADR-0051
(ex: muda comportamento P3: `RunIntent.font_paths` pode ter paths
derivados de split de env vs flags separadas — semântica idêntica
mas via diferente), **anotar ADR-0051** com nota factual:

```
Nota Passo 123 — env vars + delimiter:
- --font-path passa a aceitar `value_delimiter = ENV_PATH_SEP`
  para suportar formato TYPST_FONT_PATHS=/a:/b.
- Precedência continua: flag > env > default (clap standard).
- P5 preservado: L2 ainda não valida paths.
```

### 123.C — Implementação

**123.C.1 — Workspace Cargo.toml**:

```toml
[workspace.dependencies]
clap = { version = "4", features = ["derive", "env"] }
```

Confirmar nome exacto da feature em 123.A.1.

**123.C.2 — `02_shell/src/cli.rs`**:

Adicionar constante:

```rust
/// Separador de paths em env vars (estilo PATH).
#[cfg(unix)]
const ENV_PATH_SEP: char = ':';
#[cfg(windows)]
const ENV_PATH_SEP: char = ';';
```

Ou, se vanilla usa `std::env::SplitPaths`, alinhar.

Actualizar atributos:

```rust
/// Project root directory. ...
#[arg(long = "root", env = "TYPST_ROOT", value_name = "DIR")]
root: Option<PathBuf>,

/// Additional directories to search for fonts. May be repeated.
/// Also accepts a single value with `:` (Unix) / `;` (Windows)
/// separator, e.g. via TYPST_FONT_PATHS env var.
#[arg(
    long = "font-path",
    env = "TYPST_FONT_PATHS",
    value_name = "DIR",
    value_delimiter = ENV_PATH_SEP,
    action = clap::ArgAction::Append,
)]
font_paths: Vec<PathBuf>,
```

**123.C.3 — L4 inalterado**:

`RunIntent` não muda. L4 recebe `args.root` e `args.font_paths`
preenchidos por clap (de flag ou env, transparentemente).

**123.C.4 — Prompts L0**:

`00_nucleo/prompts/shell/cli.md`:
- Menciona env vars: `TYPST_ROOT` mapeia para `--root`;
  `TYPST_FONT_PATHS` mapeia para `--font-path` com delimiter
  de sistema.
- Precedência documentada (flag > env > default).

`crystalline-lint --fix-hashes .`.

### 123.D — Testes L4

**123.D.1 — `TYPST_ROOT` sozinho**:

```rust
#[test]
fn cli_env_typst_root() {
    let input = temp_typ("env_root", "Olá");
    let output = temp_pdf("env_root");
    let root = std::env::temp_dir();
    
    let result = Command::new(BIN)
        .env("TYPST_ROOT", &root)
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar");
    
    assert_eq!(result.status.code(), Some(0));
    assert!(output.exists());
    
    cleanup(&[&input, &output]);
}
```

**123.D.2 — `TYPST_ROOT` + `--root` (flag vence)**:

```rust
#[test]
fn cli_flag_root_vence_env() {
    let input = temp_typ("root_prec", "Olá");
    let output = temp_pdf("root_prec");
    let flag_root = std::env::temp_dir();
    let env_root = PathBuf::from("/path/inexistente/xyz");
    
    let result = Command::new(BIN)
        .env("TYPST_ROOT", &env_root)
        .arg(&input)
        .arg("--root")
        .arg(&flag_root)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar");
    
    // flag (válida) vence env (inválida) — compila OK
    assert_eq!(result.status.code(), Some(0));
    assert!(output.exists());
    
    cleanup(&[&input, &output]);
}
```

**123.D.3 — `TYPST_FONT_PATHS` com delimiter**:

```rust
#[test]
fn cli_env_typst_font_paths_delimiter() {
    let input = temp_typ("env_fonts", "Olá");
    let output = temp_pdf("env_fonts");
    let dir = std::env::temp_dir().display().to_string();
    let env_value = format!("{}{}{}", &dir, ENV_PATH_SEP_CHAR, &dir);
    // Onde ENV_PATH_SEP_CHAR é : ou ; conforme sistema
    
    let result = Command::new(BIN)
        .env("TYPST_FONT_PATHS", &env_value)
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .output()
        .expect("executar");
    
    assert_eq!(result.status.code(), Some(0));
    assert!(output.exists());
    
    cleanup(&[&input, &output]);
}
```

Se constante `ENV_PATH_SEP` não é re-exportada de L2 para tests,
usar literal `':'` em Unix; adicionar `#[cfg(unix)]` ao teste se
for só validado em Unix. Windows pode ter teste separado ou o
teste é cross-platform via constante.

Teste de **precedência** para font-paths (flag repetida + env)
— opcional neste passo. Pode ficar como candidato futuro se
adicionar complexidade.

**Contagem esperada**:

- L1: 811 (inalterado).
- L2: 24 (inalterado — clap preenche, `resolve_root_with` testes
  existentes continuam válidos).
- L3: 186 (inalterado).
- L4: 11 → **14** (+3 testes novos).
- Total: 1032 → **1035** (+3).

### 123.E — Encerramento

1. `cargo build --release` passa.
2. `cargo test --workspace` passa (1035).
3. `crystalline-lint` zero violations.
4. Validação manual:
   - `TYPST_ROOT=/tmp typst /tmp/f.typ -o out.pdf` → compila.
   - `typst /tmp/f.typ --root /real -o out.pdf` (sem env) →
     compila com /real.
   - `TYPST_FONT_PATHS=/a:/b typst f.typ -o out.pdf` → compila,
     ambos paths usados.
   - `typst --help | grep -A2 TYPST` → clap mostra env vars
     associadas a cada flag (feature `env` gera help
     automático).
5. ADR-0051 anotada (se divergência) ou inalterada.
6. Relatório `typst-passo-123-relatorio.md`:
   - Sintaxe clap exacta usada.
   - `ENV_PATH_SEP` literal por OS.
   - Output `--help` actualizado.
   - Diff de Cargo.toml.
   - Limitações aceites.

---

## Critério de conclusão

1. Inventário 123.A escrito.
2. Cargo.toml raiz: clap feature `env`.
3. `Args.root` com `env = "TYPST_ROOT"`.
4. `Args.font_paths` com `env = "TYPST_FONT_PATHS"` +
   `value_delimiter`.
5. `resolve_root_with` e lógica de `font_paths` inalteradas.
6. 3+ testes L4 novos.
7. Tests anteriores passam.
8. `cargo test --workspace` passa.
9. `crystalline-lint` zero violations.
10. Validação manual passa, incluindo `--help` mostrar env vars.
11. ADR-0051 anotada ou não tocada.
12. Relatório 123.E escrito.

---

## O que pode sair errado

- **Feature `env` do clap tem outro nome**: improvável. Gate 
  123.A.1.
- **`value_delimiter = ENV_PATH_SEP` (const) não aceita expressão
  não-literal em macro attribute**: clap derive aceita literais
  e `const`-like expressions. Se `ENV_PATH_SEP` via `#[cfg]`
  dá erro, alternativa é inline duplicado:
  ```rust
  #[cfg_attr(unix, arg(value_delimiter = ':'))]
  #[cfg_attr(windows, arg(value_delimiter = ';'))]
  ```
  Registar no 123.C.2.
- **Testes env var em paralelo**: `cargo test` corre testes em
  paralelo por default. Dois testes que setam `TYPST_ROOT` com
  `.env(...)` no `Command` não colidem (cada processo tem env
  próprio). Mas se algum teste **modificasse `std::env::set_var`
  no processo do teste**, colidiria. `.env()` em `Command` é
  isolado — OK.
- **Precedência inesperada**: clap "standard" é flag > env >
  default. Confirmar empíricamente em 123.D.2.
- **`--help` output muda significativamente**: clap com feature
  `env` anexa `[env: VAR_NAME]` automaticamente em cada flag.
  Se algum teste atualmente asserta `--help` output literal,
  falha. Tests 114+ não fazem isto. OK.
- **Windows CI**: se projecto correr em Windows, `';'` vs `':'`
  tem de funcionar. Teste D.3 com constante `ENV_PATH_SEP`
  pode precisar de `#[cfg(unix)]` para simplificar. Aceitar
  cobertura parcial e documentar.

---

## Notas operacionais

- Este é passo de **consolidação**. Zero funcionalidade nova em
  termos de pipeline; só paridade com vanilla em mecanismo de
  configuração.
- Testes env var em `Command` usam `.env("KEY", "VAL")` —
  isolado do processo que corre `cargo test`. Determinístico.
- Se alguma div surgir na ADR-0051 (ex: precedência documentada
  hoje silenciosamente assumia "flag > default" sem mencionar
  env), a anotação actualiza. P4 (default em L2) continua
  válido porque clap aplica default se env e flag ausentes; L2
  não muda.
- `--help` ganha `[env: TYPST_ROOT]` e `[env: TYPST_FONT_PATHS]`
  automaticamente — feature clap grátis.
- O passo pode desbloquear scripts Typst legacy que assumem env
  vars. Valor UX concreto apesar de ser trivial em implementação.
- Próximo passo natural: **warning amigável para path inválido**
  (do plano 122), ou quebrar linha para L1/L3.
