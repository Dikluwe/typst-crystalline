# Passo 116 — Cores ANSI nos diagnósticos (`--color` + `NO_COLOR`)

**Série**: 116 (passo médio; L3 + L4).
**Precondição**: Passo 115 encerrado (clap migração); 811 L1 + 195
L3 + 5 L4 + 6 ignorados; zero violations.
**ADRs aplicáveis**: ADR-0045 (formato de diagnósticos), ADR-0046
(CLI mínima), ADR-0047 (clap).
**ADR nova**: ADR-00NN "Cores ANSI em diagnósticos — L3 formata
com bool, L4 decide" — `PROPOSTO` em 116.B, `EM VIGOR` em 116.E.

---

## Objectivo

Formatter de diagnósticos em L3 passa a aceitar `colored: bool`.
Quando `true`, aplica ANSI escapes para:

- `error:` — vermelho bold.
- `warning:` — amarelo bold.
- `path:linha:coluna:` — cinzento (ou dim).
- message — bold.
- `hint:` — ciano bold.

CLI em L4 decide o bool conforme:

1. `--color=never` (explícito) → `false`.
2. `--color=always` (explícito) → `true`.
3. `NO_COLOR` env var presente (e sem flag explícita) → `false`.
4. `--color=auto` (default) → `IsTerminal::is_terminal(&stderr)`.

Este passo **não**:
- Toca L1.
- Adiciona outras flags (`--verbose`, `--quiet`).
- Altera o conteúdo do output (só aplica cores em cima do texto
  existente do Passo 111).
- Muda pipeline.

---

## Decisões já tomadas

1. **Lógica em L3**: `format_diagnostic(diag, source, path, colored: bool)`.
   Wrapper function ou parâmetro extra — decisão em 116.C.1.
2. **ANSI manual**: sem `anstyle`, `termcolor`, `colored`. Código
   directo com literais `"\x1b[31m..."`.
3. **Paleta**: error vermelho bold, warning amarelo bold, path
   dim, message bold, hint ciano bold.
4. **Flag clap**: `--color` com valor `auto|always|never`; default
   `auto`.
5. **`NO_COLOR`**: respeitado quando flag está em `auto`.
6. **isatty**: via `std::io::IsTerminal` (Rust 1.70+).

---

## Escopo

**Dentro**:
- `03_infra/src/diagnostic_format.rs` — formatter ganha parâmetro
  `colored: bool`. Constantes com escapes ANSI. Testes unitários.
- `03_infra/src/diagnostic_format.rs` — `drain_diagnostics_to_stderr`
  ganha parâmetro `colored: bool`.
- `04_wiring/src/main.rs` — flag `--color` + lógica
  auto/always/never + `NO_COLOR` env var + isatty + propagar
  bool.
- Testes 114 — actualizar se assinatura das funções mudou (sim,
  mudou — 1 parâmetro extra).

**Fora**:
- L1.
- Outras flags.
- Cores em outros outputs (PDF, stdout — que é sempre vazio).
- Suporte a terminais Windows legacy sem ANSI (Windows 10+ tem
  ANSI nativo desde 2016 via `ENABLE_VIRTUAL_TERMINAL_PROCESSING`;
  aceitar e não escopar).

---

## Sub-passos

### 116.A — Inventário

**Parte 1 — MSRV (Minimum Supported Rust Version)**:

1. `grep` por `rust-version` em `Cargo.toml` raiz e nos crates.
2. Confirmar que é ≥ 1.70 para usar `std::io::IsTerminal`.
3. Se MSRV < 1.70, decidir:
   - Subir MSRV (decisão arquitectural simples, típica).
   - Usar `is-terminal` crate (backport).
4. Registar decisão.

**Parte 2 — Convenção ANSI no ecosystem Rust**:

1. Consultar os escapes ANSI padrão:
   - `\x1b[31m` — vermelho.
   - `\x1b[33m` — amarelo.
   - `\x1b[36m` — ciano.
   - `\x1b[2m` — dim.
   - `\x1b[1m` — bold.
   - `\x1b[0m` — reset.
2. Combinar: `\x1b[1;31m` é vermelho bold; `\x1b[1;31m<text>\x1b[0m`.
3. Registar paleta escolhida com escapes literais:

```
ERROR:   "\x1b[1;31m"  (red bold)
WARN:    "\x1b[1;33m"  (yellow bold)
HINT:    "\x1b[1;36m"  (cyan bold)
PATH:    "\x1b[2m"     (dim)
BOLD:    "\x1b[1m"     (bold)
RESET:   "\x1b[0m"
```

**Parte 3 — Convenção `NO_COLOR`**:

1. Ler [no-color.org](https://no-color.org/) principle: "any
   value" triggers colorless. Presença da var, não valor.
2. Registar: lógica Rust é `std::env::var_os("NO_COLOR").is_some()`.

**Parte 4 — Clap `--color` convenção**:

1. Verificar como vanilla define `--color`. Grep em
   `lab/typst-original/crates/typst-cli/src/args.rs`.
2. Registar enum usado: `ColorChoice`, `ColorWhen`, ou derive
   com `#[arg(value_enum)]`.

**Escrever** em `00_nucleo/diagnosticos/inventario-cores-passo-116.md`:

```
MSRV:
  rust-version = "..."
  IsTerminal disponível? sim/não

Paleta ANSI:
  [tabela com escapes]

NO_COLOR:
  env::var_os("NO_COLOR").is_some()

Vanilla --color:
  enum: [nome]
  variantes: auto/always/never
```

**Gate 116.A**: se MSRV < 1.70 e o projecto não quer subir, o
passo muda forma (usa `is-terminal` crate como dep nova).
Decisão documentada antes de avançar.

### 116.B — ADR nova

Criar `00_nucleo/adr/typst-adr-00NN-diagnosticos-com-cor.md`
com `PROPOSTO`.

Conteúdo:

- **Contexto**: Passo 111 produziu formato gcc/clang legível mas
  monocromático. Utilizadores modernos esperam cores em
  diagnostics (rustc, clang, gcc têm-nas). CLI (Passo 113)
  agora é invocável; hora de tornar o output visualmente
  rico.
- **Decisão**:
  - `format_diagnostic(diag, source, path, colored: bool)` —
    parâmetro adicionado.
  - Escapes ANSI manuais com constantes nomeadas.
  - Paleta: error vermelho bold, warning amarelo bold, path
    dim, message bold, hint ciano bold.
  - `drain_diagnostics_to_stderr` propaga `colored`.
  - Flag clap `--color=auto|always|never` em L4; default `auto`.
  - Ordem de precedência: flag explícita > `NO_COLOR` > isatty
    auto.
- **Alternativas rejeitadas**:
  - **`anstyle`/`termcolor`**: deps adicionais para pouca
    funcionalidade (5 constantes). Manual resolve.
  - **Cores em L4**: duplica formatação; L3 já tem a estrutura
    do output.
  - **Detecção isatty em L3**: L3 é mais puro se não conhece
    contexto do terminal. L4 decide.
- **Limitações documentadas**:
  - Windows legacy (pre-Windows 10) pode não renderizar ANSI.
    Aceitar.
  - Paleta fixa; sem `--color=256` ou themes custom.
  - `NO_COLOR` só olha para **presença** (qualquer valor, mesmo
    "0", conta). Convenção.

Promover em 116.E.

### 116.C — Implementação

**116.C.1 — Assinatura do formatter**:

Em `03_infra/src/diagnostic_format.rs`:

```rust
// Constantes ANSI
const ANSI_RED_BOLD:    &str = "\x1b[1;31m";
const ANSI_YELLOW_BOLD: &str = "\x1b[1;33m";
const ANSI_CYAN_BOLD:   &str = "\x1b[1;36m";
const ANSI_DIM:         &str = "\x1b[2m";
const ANSI_BOLD:        &str = "\x1b[1m";
const ANSI_RESET:       &str = "\x1b[0m";

pub fn format_diagnostic(
    diag: &SourceDiagnostic,
    source: &Source,
    source_path: &str,
    colored: bool,
) -> String {
    let (sev_color, sev_text) = match diag.severity {
        Severity::Error   => (ANSI_RED_BOLD,    "error"),
        Severity::Warning => (ANSI_YELLOW_BOLD, "warning"),
    };
    
    let location = match source.span_to_line_col(diag.span) {
        Some((line, col)) => format!("{source_path}:{line}:{col}"),
        None              => format!("{source_path}:<detached>"),
    };
    
    let mut out = if colored {
        format!(
            "{dim}{location}{reset}: {sev}{sev_text}{reset}: {bold}{msg}{reset}\n",
            dim = ANSI_DIM,
            reset = ANSI_RESET,
            sev = sev_color,
            bold = ANSI_BOLD,
            msg = diag.message,
        )
    } else {
        format!("{location}: {sev_text}: {msg}\n", msg = diag.message)
    };
    
    for hint in &diag.hints {
        if colored {
            out.push_str(&format!(
                "  {cyan}hint{reset}: {hint}\n",
                cyan = ANSI_CYAN_BOLD,
                reset = ANSI_RESET,
            ));
        } else {
            out.push_str(&format!("  hint: {hint}\n"));
        }
    }
    
    out
}

pub fn drain_diagnostics_to_stderr(
    diagnostics: &[SourceDiagnostic],
    source: &Source,
    source_path: &str,
    colored: bool,
) {
    for diag in diagnostics {
        eprint!("{}", format_diagnostic(diag, source, source_path, colored));
    }
}
```

**116.C.2 — Flag clap em `04_wiring/src/main.rs`**:

```rust
#[derive(Debug, Clone, clap::ValueEnum)]
enum ColorWhen {
    Auto,
    Always,
    Never,
}

#[derive(Parser, Debug)]
#[command(
    name = "typst",
    version,
    about = "Typst compiler (crystalline)"
)]
struct Args {
    /// Input .typ file.
    input: PathBuf,
    /// Output PDF file.
    output: PathBuf,
    /// When to use coloured output.
    #[arg(long = "color", value_enum, default_value_t = ColorWhen::Auto)]
    color: ColorWhen,
}
```

**116.C.3 — Lógica de decisão do bool**:

Função pura em `main.rs`:

```rust
use std::io::IsTerminal;

fn resolve_colored(choice: &ColorWhen) -> bool {
    match choice {
        ColorWhen::Never  => false,
        ColorWhen::Always => true,
        ColorWhen::Auto   => {
            if std::env::var_os("NO_COLOR").is_some() {
                false
            } else {
                std::io::stderr().is_terminal()
            }
        }
    }
}
```

**116.C.4 — Integração no `main()`**:

```rust
fn main() -> ExitCode {
    let args = Args::parse();
    let colored = resolve_colored(&args.color);
    // ... resto inalterado, excepto propagar colored
    drain_diagnostics_to_stderr(&warnings, &source, &source_path, colored);
    // ... em errors também
    drain_diagnostics_to_stderr(&errors, &source, &source_path, colored);
}
```

**116.C.5 — Actualizar testes 114**:

Os 5 testes invocam o binário via `Command`. O binário tem
default `--color=auto`, mas `Command::output()` captura stderr
num pipe — `IsTerminal` retorna `false`. Logo, sem `--color=always`
explícito, os testes vêm output sem cores. Bom — asserts sobre
`"warning:"` (palavra literal) continuam a funcionar.

Se algum teste quisesse **verificar** cores, passaria
`--color=always` explicitamente. Mas os testes 114 não o fazem
— ficam intactos.

Verificar especificamente: os asserts usam `.contains("warning:")`.
Com cores, a string seria `"\x1b[1;33mwarning\x1b[0m:"` — **não
contém `"warning:"` literal** porque há escapes entre `warning`
e `:`. Logo, se algum dia um teste queira cores, assert muda.

Hoje, sem cores (default em pipe), os asserts passam.

**116.C.6 — Testes unitários de `format_diagnostic`**:

Adicionar em `diagnostic_format.rs` `#[cfg(test)]`:

1. `format_diagnostic_sem_cores_e_como_passo_111` — bool `false`
   produz output idêntico ao Passo 111 (regressão).
2. `format_diagnostic_com_cores_contem_ansi_escapes` — bool
   `true` produz output com `\x1b[` presente.
3. `format_diagnostic_com_cores_tem_reset_no_fim_de_spans` —
   verifica que cada span aberto fecha com `\x1b[0m`.
4. `format_diagnostic_com_cores_error_vs_warning` — error tem
   `\x1b[1;31m`, warning tem `\x1b[1;33m`.

**116.C.7 — Testes de `resolve_colored` em L4**:

Em `04_wiring/tests/cli.rs` ou teste unitário em `main.rs`
(`#[cfg(test)] mod tests`):

1. `resolve_colored_never_e_false` — `ColorWhen::Never` → false.
2. `resolve_colored_always_e_true` — `ColorWhen::Always` → true.
3. `resolve_colored_auto_com_no_color_env` — com `NO_COLOR=1` →
   false. Truque: setar env var localmente (`std::env::set_var`)
   no teste, limpar no fim.
4. (isatty em teste é sempre false no pipe — difícil de testar
   unitariamente, aceitar).

### 116.D — Validação manual

1. Cores em terminal real:
   ```bash
   $ typst /tmp/test.typ /tmp/out.pdf
   ```
   Output em stderr tem cores: "warning:" amarelo, "error:"
   vermelho, path cinza, hint ciano.

2. Sem cores em pipe:
   ```bash
   $ typst /tmp/test.typ /tmp/out.pdf 2>&1 | cat
   ```
   `cat` é pipe, `isatty(stderr)` é false, sem cores.

3. Forçar cores em pipe:
   ```bash
   $ typst /tmp/test.typ /tmp/out.pdf --color=always 2>&1 | cat
   ```
   Cores presentes mesmo em pipe.

4. Forçar sem cores em terminal:
   ```bash
   $ typst /tmp/test.typ /tmp/out.pdf --color=never
   ```
   Sem cores.

5. `NO_COLOR`:
   ```bash
   $ NO_COLOR=1 typst /tmp/test.typ /tmp/out.pdf
   ```
   Sem cores.

6. `NO_COLOR` com `--color=always`:
   ```bash
   $ NO_COLOR=1 typst /tmp/test.typ /tmp/out.pdf --color=always
   ```
   Com cores. Flag explícita vence env var.

### 116.E — Encerramento

1. `cargo build --release` passa.
2. `cargo test --workspace` passa com testes novos (unitários
   de `format_diagnostic` com/sem cores + `resolve_colored`).
3. `crystalline-lint` zero violations.
4. Validação manual 116.D passa.
5. ADR promovida a `EM VIGOR`.
6. Relatório `typst-passo-116-relatorio.md`:
   - Paleta literal escolhida.
   - Exemplos antes/depois em texto com ANSI visível.
   - Decisão sobre MSRV (se foi preciso subir).
   - Número de testes novos e finais.
   - Limitações aceites (Windows legacy, paleta fixa).

---

## Critério de conclusão

Todas em conjunto:

1. Inventário 116.A escrito.
2. ADR-00NN criada e promovida.
3. `format_diagnostic` e `drain_diagnostics_to_stderr` aceitam
   `colored: bool`.
4. Flag clap `--color` funcional.
5. Lógica `resolve_colored` correcta (flag > NO_COLOR > isatty).
6. Testes novos passam.
7. Testes 114 passam (intactos).
8. `cargo test --workspace` passa.
9. `crystalline-lint` zero violations.
10. Validação manual 116.D passa.
11. Relatório 116.E escrito.

---

## O que pode sair errado

- **MSRV < 1.70**: gate 116.A. Se projecto não quer subir,
  adicionar `is-terminal` crate. Preferência: subir MSRV
  (decisão comum em 2026).
- **Escapes ANSI em Windows legacy**: pré-Windows 10 não
  interpreta ANSI em console por default. Windows 10+ (2016+)
  tem suporte nativo. Aceitar; utilizadores em Windows 7/8
  veriam `\x1b[...` literal. Caso extremo.
- **Pipe detection em shell com `tee`**: `tee` duplica stdout
  mas stderr fica no terminal. `isatty(stderr)` no typst continua
  true. Output colorido no terminal, não na cópia — comportamento
  correcto.
- **`NO_COLOR` com valor vazio `NO_COLOR=`**: lerá var_os
  como `Some("")`. Convenção de no-color.org diz "any value,
  including empty". Logo, `NO_COLOR=` ainda activa no-color.
  `is_some()` em Rust trata isso correctamente. ✓
- **`--color=always` em pipe produz escapes que ninguém
  interpreta**: comportamento desejado (utilizador pediu
  explicitamente). Se redireccionam para ficheiro, vêem `\x1b[`
  lá. Aceitar.
- **Testes unitários de `resolve_colored` com env vars**:
  mutar env var em teste unitário tem race conditions em
  testes paralelos. Dois caminhos:
  - Usar `std::env::remove_var` antes e ainda depois do teste
    (não-determinístico).
  - Isolar o teste com `#[serial]` macro (dep `serial_test`,
    fora do escopo).
  - Aceitar que `resolve_colored_auto_com_no_color_env` é
    teste frágil; remover ou usar sub-função que aceita
    `Option<OsString>` como argumento em vez de ler env.
  Recomendação: **refactor** para `resolve_colored_with(choice,
  no_color_present: bool, is_tty: bool) -> bool` — pura,
  testável. `resolve_colored` passa a ser wrapper thin que
  lê env e isatty, chama a pura.
- **Actualizar assinatura em 4 sítios** pode ter cadeia de
  mudanças nos testes L3 existentes (195 passaram antes). Ver
  `integration_tests.rs` — testes do pipeline podem usar
  `drain_diagnostics_to_stderr` com 3 args; adicionar o 4º
  em todos.

---

## Notas operacionais

- Este é um passo de **polish** — não adiciona funcionalidade
  nova, só qualidade de vida. Utilizadores em terminal moderno
  vão notar instantaneamente; utilizadores em CI (pipe, não-tty)
  não vão notar diferença (sem cores por default).
- Padrão estabelecido aqui (`--color` + `NO_COLOR` + isatty)
  repete-se se houver outras flags visuais (`--progress`,
  `--verbose`, etc.). Documentar em ADR para reuso.
- Paleta fixa é aceitável para V1. Utilizadores com acessibilidade
  específica (daltonismo, por exemplo) podem ver problemas com
  vermelho/amarelo — `--color=never` + `NO_COLOR` dão saída sem
  cor como alternativa.
- Se 116.C.7 usar "refactor" para função pura (recomendado para
  testabilidade), documentar a estrutura. `resolve_colored_with`
  permite tests determinísticos sem mutar env.
- O formatter actual é ~40 linhas; com cores passa a ~60.
  Aceitável. Se chegasse a 100+ linhas com lógica de cor
  espalhada, valeria extrair helper `style(text, escape)`.
