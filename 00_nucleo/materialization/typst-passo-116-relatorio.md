# Passo 116 — Relatório (cores ANSI nos diagnósticos)

**Data**: 2026-04-23
**Precondição**: Passo 115 encerrado; 811 L1 + 195 L3 + 5 L4 + 6
ignorados; zero violations.
**ADR criada**: ADR-0048 "Cores ANSI nos diagnósticos" — **PROMOVIDA
A EM VIGOR** em 116.E.

---

## Sumário

Formatter de diagnósticos (L3) agora aceita `colored: bool` e
aplica escapes ANSI quando `true`. CLI (L4) decide conforme
`--color=auto|always|never` + `NO_COLOR` env + `isatty(stderr)`.

Paleta: vermelho bold (error), amarelo bold (warning), ciano bold
(hint), dim (path), bold (message). Alinhamento rustc/clang.

**Correção arquitectural durante implementação**: a `ColorWhen`
enum foi deslocada de L4 para L3 para respeitar a regra V12 do
linter ("L4 não cria tipos"). L3 ganhou `clap` como dep.

**811 L1 + 207 L3 (+12) + 5 L4 (inalterado)** + 6 ignorados. Zero
violations. **48 ADRs activas** (+0048).

---

## 116.A — Decisões

Inventário em
`00_nucleo/diagnosticos/inventario-cores-passo-116.md`.

| Decisão | Escolha | Razão |
|---------|---------|-------|
| MSRV / `IsTerminal` | Usar directo | rustc 1.92 instalado; `IsTerminal` estável desde 1.70. |
| Paleta | 6 constantes ANSI literais | Sem deps (`anstyle`, `termcolor`, `colored` rejeitados). |
| `NO_COLOR` | `env::var_os("NO_COLOR").is_some()` | Convenção 2024 (qualquer valor, incluindo empty). |
| `ColorWhen` enum | Custom (não `clap::ColorChoice`) | Semântica distinta (diagnostics vs help do clap). |
| Localização `ColorWhen` | **L3** (não L4 como previsto no spec) | Respeita V12 do linter. Decidido durante implementação. |
| `default_missing_value` | Não usar | `typst --color` sem valor dá erro de clap; mais previsível. |

Gate 116.A não disparou (MSRV moderno).

---

## 116.B — ADR-0048

Criada em `00_nucleo/adr/typst-adr-0048-diagnosticos-com-cor.md`.
**EM VIGOR em 116.E**.

Pontos-chave:

- `format_diagnostic` + `drain_diagnostics_to_stderr` ganham 4º
  parâmetro `colored: bool`.
- Paleta ANSI literal com 6 constantes.
- `ColorWhen` + `resolve_colored_with` **em L3** (após correcção
  V12).
- L4 envolve: `resolve_colored(choice) → resolve_colored_with(choice,
  env_no_color, isatty)`.
- Ordem de precedência: flag > `NO_COLOR` > isatty.
- Sem deps novas (ANSI manual).

---

## 116.C — Implementação

### Ficheiros tocados

| Ficheiro | Mudança |
|----------|---------|
| `03_infra/Cargo.toml` | +1 linha: `clap = { workspace = true }`. |
| `03_infra/src/diagnostic_format.rs` | +6 constantes ANSI, +`ColorWhen` enum, +`resolve_colored_with`, +12 testes; assinatura `colored: bool` em 2 funções. |
| `04_wiring/src/main.rs` | Adiciona `#[arg(--color)]` em `Args`; `resolve_colored` wrapper; importa `ColorWhen` + `resolve_colored_with` de L3; propaga `colored` nos 2 `drain_*` calls. |
| `03_infra/src/integration_tests.rs` | 6 call sites `format_diagnostic(..., false)` (sem cores, back-compat Passo 111). |
| `00_nucleo/prompts/wiring.md` | Actualizado com `--color` + precedência. |

### Paleta final (literais ANSI)

```rust
const ANSI_RED_BOLD:    &str = "\x1b[1;31m";  // error
const ANSI_YELLOW_BOLD: &str = "\x1b[1;33m";  // warning
const ANSI_CYAN_BOLD:   &str = "\x1b[1;36m";  // hint
const ANSI_DIM:         &str = "\x1b[2m";     // path
const ANSI_BOLD:        &str = "\x1b[1m";     // message
const ANSI_RESET:       &str = "\x1b[0m";
```

### Correção V12 (ColorWhen movido L4 → L3)

Primeira tentativa colocou `ColorWhen` em L4 (como spec). Lint
emitiu V12 warning:

```
warning: Lógica no fio: enum 'ColorWhen' declarado em L4. L4 não
cria tipos — mover para L2 ou L3. [V12]
   --> ./04_wiring/src/main.rs:64
```

Solução: `ColorWhen` + `resolve_colored_with` movidos para L3.
`03_infra` ganha `clap` como dep (workspace dep, custo zero).
L4 importa via `typst_infra::diagnostic_format::{ColorWhen,
resolve_colored_with}`.

Consequência positiva: os 6 testes `resolve_colored_*` também vivem
em L3 (próximos da definição). L4 fica só composição pura (regra
V12 honrada).

---

## 116.D — Validação manual

### Pipe (default, auto → sem cores)

```
$ ./target/release/typst /tmp/test.typ /tmp/out.pdf 2>&1 | cat
/tmp/test.typ:5:11: warning: text: propriedade 'font' ainda não suportada
  hint: ver ADR-0040 para propriedades cobertas por set text
```

Sem escapes ANSI — `isatty(stderr)` é `false` no pipe.

### `--color=always` em pipe (cores forçadas)

```
$ ./target/release/typst /tmp/test.typ /tmp/out.pdf --color=always 2>&1 | cat
[2m/tmp/test.typ:5:11[0m: [1;33mwarning[0m: [1mtext: propriedade 'font' ainda não suportada[0m
  [1;36mhint[0m: ver ADR-0040 para propriedades cobertas por set text
```

Todos os escapes presentes:
- `[2m...[0m` — path em dim.
- `[1;33m...[0m` — warning em amarelo bold.
- `[1m...[0m` — message em bold.
- `[1;36m...[0m` — hint em ciano bold.

### `--color=never`

Sem cores.

### `NO_COLOR=1` com `--color=always` (flag vence)

```
$ NO_COLOR=1 ./target/release/typst /tmp/test.typ /tmp/out.pdf --color=always 2>&1 | cat
[2m...[0m ...
```

Cores presentes — flag explícita vence env var. ✓

### `--help`

```
$ ./target/release/typst --help
Typst compiler (crystalline)

Usage: typst [OPTIONS] <INPUT> <OUTPUT>

Arguments:
  <INPUT>   Input .typ file
  <OUTPUT>  Output PDF file

Options:
      --color <COLOR>
          When to use coloured diagnostics

          Possible values:
          - auto:   Cores activas se stderr é terminal e `NO_COLOR` ausente
          - always: Cores sempre activas, mesmo em pipe
          - never:  Cores sempre desactivadas

          [default: auto]

  -h, --help     Print help (see a summary with '-h')
  -V, --version  Print version
```

Flag `--color` aparece com `Possible values` + descrições geradas
pelos docstrings do enum em L3.

---

## 116.E — Encerramento

### Verificação

```
$ cargo test --workspace | grep "test result"
test result: ok. 811 passed ...  (L1 inalterado)
test result: ok. 207 passed ...  (L3 +12: format_diagnostic_* cores + resolve_colored_*)
test result: ok. 5 passed   ...  (L4 integração — inalterado)

$ crystalline-lint .
✓ No violations found
```

Tests 114 passam **sem modificação**: `cargo test` captura stderr
em pipe → `isatty` = false → sem cores por default → asserts
literais (`.contains("warning:")`) continuam válidos.

### ADR

**ADR-0048** `EM VIGOR`.

---

## Números finais

| Métrica | Antes | Depois |
|---------|------:|-------:|
| L1 tests | 811 | 811 (inalterado) |
| L3 tests | 195 | **207** (+12) |
| L4 integration tests | 5 | 5 (inalterado) |
| L4 unit tests (`main.rs`) | 0 | 0 (movidos para L3) |
| Ignorados | 6 | 6 |
| Violations | 0 | 0 |
| ADRs activas | 47 | **48** (+0048) |
| DEBTs abertos | 11 | 11 (inalterado) |
| Deps externas em 03_infra | 3 | **4** (+clap) |
| Deps externas em workspace | 14 | 14 (inalterado) |
| Linhas ANSI constants (L3) | 0 | 6 |
| Linhas `resolve_colored_with` + `ColorWhen` (L3) | 0 | ~18 |
| Linhas `main.rs` (L4) | 110 | 99 (helpers movidos) |

---

## Limitações aceites

1. **Windows legacy (pre-Windows 10)** mostram `\x1b[...` literal.
   Aceitável — Windows 10+ tem ANSI nativo desde 2016.
2. **Paleta fixa** — sem `--color=256`, themes.
3. **Daltonismo** — utilizador tem `--color=never` / `NO_COLOR=1`.
4. **`NO_COLOR=...valor...`** — presença conta, valor ignorado
   (convenção 2024).
5. **Binário real `isatty` em `cargo test`** — tests 114 não
   forçam `--color=always`, então vêm sem cores; assert literais
   funcionam. Comportamento documentado em
   `04_wiring/tests/cli.rs`.

---

## Lições

1. **V12 dispara warning, não error**: mas trata-se como
   qualidade de código. Mover `ColorWhen` para L3 foi correcção
   orgânica — o enum é tipo de dados relacionado a formatting
   (L3), não orquestração (L4). Spec preliminar colocou-o em L4;
   o lint confirmou que L3 é mais adequado.

2. **`clap::ValueEnum` em L3 é aceitável**: L3 já tem outras
   deps (ttf-parser, rustybuzz, time, etc.). `clap` entra como
   dep de **dados** (ValueEnum derive) mais do que argparsing.
   L4 continua a ser o único sítio que chama `clap::Parser::parse()`.

3. **Docstrings nos variants do enum → `--help` bonitinho**:
   `clap::ValueEnum` usa os docstrings `///` como descrições
   para `Possible values`. UX gratuita.

4. **Função pura é ouro**: `resolve_colored_with(choice, bool,
   bool)` é 100% testável sem env mutation. 6 tests unitários
   cobrem 3³ = 27 combinações conceptuais (comprimidas em 6
   testes). Zero `std::env::set_var` em testes.

5. **Paleta mínima paga-se**: 6 constantes + 3 branches if/else
   adicionam ~20 linhas ao formatter. Zero deps, zero complexidade
   de runtime. `anstyle`/`termcolor` seriam overkill para esta
   escala.

6. **NO_COLOR precedence = clap feature + convention**: Rust não
   tem padrão built-in para `NO_COLOR`. Criar `resolve_colored_with`
   explicita a ordem de precedência e permite testar cada caminho.
   Se a lógica vivesse inline em `main()`, seria impossível
   testar.

---

## Estado pós-Passo 116

### UX final

```bash
$ typst input.typ output.pdf                    # auto: cores em tty, sem em pipe
$ typst input.typ output.pdf --color=always     # sempre com cores
$ typst input.typ output.pdf --color=never      # sempre sem cores
$ NO_COLOR=1 typst input.typ output.pdf         # env desactiva em auto
$ NO_COLOR=1 typst input.typ output.pdf --color=always  # flag vence
```

Diagnostics coloridos:
- Vermelho bold para `error:`.
- Amarelo bold para `warning:`.
- Dim para `path:linha:coluna`.
- Bold para mensagem.
- Ciano bold para `hint:`.

### Trabalho futuro identificado

1. **Paleta customizável** via tema ou env — adiada até surgir
   procura.
2. **Windows legacy** — se algum utilizador reportar consoles
   sem ANSI, passo dedicado com `ENABLE_VIRTUAL_TERMINAL_PROCESSING`
   setup.
3. **`-f/--format` (PDF/PNG/SVG/HTML)** — quando outros exports
   existirem.
4. **`--root`, `--font-path`** — passos dedicados com integração
   `SystemWorld`.
5. **JSON/SARIF diagnostics** — passos dedicados.
