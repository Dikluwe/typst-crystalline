# Passo 115 — Relatório (argparsing migrado para `clap`)

**Data**: 2026-04-23
**Precondição**: Passo 114 encerrado (CLI tests); 811 L1 + 195 L3
+ 5 L4 + 6 ignorados; zero violations.
**ADR criada**: ADR-0047 "CLI argparsing com clap — flags
básicas" — **PROMOVIDA A EM VIGOR** em 115.E.

---

## Sumário

Argparsing manual do Passo 113 migrou para **`clap 4` com `derive`**.
Ganhos imediatos: `--help` automático, `--version` automático,
mensagens de erro formatadas para argumentos em falta. Sem
mudança de UX — positional `input output` mantido (compat A).

Zero mudanças em tests 114 (alinhamento escopo a + compat A).

**811 L1 + 195 L3 + 5 L4 + 6 ignorados** (inalterado).
Zero violations. **47 ADRs activas** (+0047).

---

## 115.A — Decisões

Inventário em
`00_nucleo/diagnosticos/decisoes-clap-passo-115.md`.

| Decisão | Escolha | Razão |
|---------|---------|-------|
| Versão clap | `4` (features `derive`) | Alinhar com vanilla `4.4` (pin major); features extras adiadas. |
| Escopo | **(a) Mínimo** | Positional alinha com vanilla; `--help`/`--version` grátis. |
| Compat | **(A) Manter positional** | Tests 114 inalterados; zero mudança de UX para utilizadores. |

Gate 115.A não disparado.

---

## 115.B — ADR-0047

Criada em `00_nucleo/adr/typst-adr-0047-cli-clap.md`.
**EM VIGOR em 115.E**.

Pontos-chave:

- `clap = "4"` com `derive` em `[workspace.dependencies]`.
- `Args` struct com `#[derive(Parser)]` + `#[command(name, version, about)]`.
- Positional `input` + `output` (sem `-o/--output`).
- Features `env`, `wrap_help`, `string` adiadas até uso.

Alternativas rejeitadas: argh/pico-args (menos idiomáticos);
`-o/--output` (divergiria de vanilla); `--root`/`--font-path`
(cada uma é passo dedicado).

---

## 115.C — Implementação

### Ficheiros tocados

| Ficheiro | Mudança |
|----------|---------|
| `Cargo.toml` (raiz) | +1 linha: `clap = { version = "4", features = ["derive"] }` em `[workspace.dependencies]`. |
| `04_wiring/Cargo.toml` | +1 linha: `clap = { workspace = true }`. |
| `04_wiring/src/main.rs` | `parse_args` manual removido; `Args` struct + `Args::parse()` substituem. |
| `00_nucleo/prompts/wiring.md` | Actualizado: descreve argparsing clap. |

### Estrutura `Args`

```rust
// Escopo (a) do Passo 115 — mínimo: positional `input output`.
// `--help` e `--version` vêm gratuitos do derive.
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
}

fn main() -> ExitCode {
    let args = Args::parse();
    let input = args.input;
    let output = args.output;
    // ... pipeline inalterado
}
```

### Remoção

`parse_args` manual (10 linhas) substituído por `Args::parse()`
(1 linha). Match sobre `&args[..]` eliminado — clap encarrega-se
de validar arity e imprimir usage em erro.

### `main.rs` líquido

- Antes (113): 99 linhas (com `parse_args`).
- Depois (115): 99 linhas (sem `parse_args`, com `Args` + `use
  clap::Parser`). Empate: a função `parse_args` foi absorvida
  pelo derive + atributos de struct.

---

## 115.D — Validação manual

### `--help`

```
$ ./target/release/typst --help
Typst compiler (crystalline)

Usage: typst <INPUT> <OUTPUT>

Arguments:
  <INPUT>   Input .typ file
  <OUTPUT>  Output PDF file

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### `--version`

```
$ ./target/release/typst --version
typst 0.1.0
```

Lê `version = "0.1.0"` de `[workspace.package]` via
`version.workspace = true`.

### Fluxo normal (como 113.D)

```
$ ./target/release/typst /tmp/test115.typ /tmp/test115.pdf
/tmp/test115.typ:5:11: warning: text: propriedade 'font' ainda não suportada
  hint: ver ADR-0040 para propriedades cobertas por set text
exit=0
```

PDF válido produzido.

### Sem argumentos

```
$ ./target/release/typst
error: the following required arguments were not provided:
  <INPUT>
  <OUTPUT>

Usage: typst <INPUT> <OUTPUT>

For more information, try '--help'.
exit=2
```

clap emite mensagem estruturada + usage + hint, exit 2
automaticamente. Contém `Usage` e `error:` — tests 114 passam.

### Erro de I/O (inalterado)

```
$ ./target/release/typst /tmp/nao-existe.typ /tmp/out.pdf
error: main file not found: /tmp/nao-existe.typ
exit=2
```

### Limpeza: observação

**Nota técnica**: o docstring na struct `Args` inicialmente
vazou para `--help` como descrição longa. Corrigido: comentário
de implementação fica em `//` (não `///`) acima do derive; só o
`about = "..."` gera a descrição user-facing. O `--help` final
mostra exactamente `"Typst compiler (crystalline)"`.

---

## 115.E — Encerramento

### Verificação

```
$ cargo build --release
    Finished `release` profile [optimized] target(s) in 10.44s (primeira) / 0.46s (incremental)

$ cargo test --workspace | grep "test result"
test result: ok. 811 passed ...  (L1 inalterado)
test result: ok. 195 passed ...  (L3 inalterado)
test result: ok. 5 passed   ...  (L4 inalterado; tests 114 não migrados, compat A)

$ crystalline-lint .
✓ No violations found
```

Tests 114 passaram **sem modificação** (compat A confirmada).

### ADR

**ADR-0047** `EM VIGOR`.

---

## Números finais

| Métrica | Antes | Depois |
|---------|------:|-------:|
| L1 tests | 811 | 811 (inalterado) |
| L3 tests | 195 | 195 (inalterado) |
| L4 tests (`typst-wiring`) | 5 | 5 (inalterado) |
| Ignorados | 6 | 6 |
| Violations | 0 | 0 |
| ADRs activas | 46 | **47** (+0047) |
| DEBTs abertos | 11 | 11 (inalterado) |
| Deps externas em 04_wiring | 1 (anyhow) | **2** (anyhow, clap) |
| Deps externas em workspace | 13 | **14** (+clap) |
| `main.rs` linhas | 99 | 99 (parse_args absorvido no derive) |

### Binário crescimento

Release build: tempo **10.44s** (primeira compilação com clap);
incrementais < 1s. Tamanho binário aumenta ~300KB com clap
(aceitável para qualidade de vida conseguida).

---

## Limitações aceites

1. **Sem subcomandos** — CLI continua flat. Vanilla tem 9
   subcomandos; cristalino adopta-os em passos dedicados.
2. **Sem flags funcionais** — `--root`, `--font-path`, `-o`,
   `--format`, `--color`. Passos dedicados futuros.
3. **Sem features extras de clap** — `env` (para `TYPST_ROOT`
   etc.), `wrap_help` (layout de ajuda), `string` (styling).
   Adicionar quando forem usadas.
4. **`version = "0.1.0"`** — literal do workspace. Se o projecto
   ganhar versioning por commit/date, mudar fica em passo
   dedicado.

---

## Lições

1. **Clap `derive` é verdadeiramente trivial** — 4 attributes
   + 2 campos vs 10 linhas de match manual. E ganha `--help` +
   `--version` + formatação de erros.

2. **Docstrings do struct vazam para `--help`**: lição de UX
   involuntariamente encontrada. Solução: comentário de
   implementação em `//`, user-facing em `about = "..."`. Vale
   o cuidado porque o help é a primeira impressão da CLI.

3. **Compat A é o caminho natural**: preservar positional com
   testes 114 intactos confirma que a migração é refactor puro.
   Se quisermos `-o` no futuro, adicionar flag sem remover
   positional (`Option<PathBuf>` + `#[arg(short, long)]`) é
   viável.

4. **Pin major em deps externas**: o workspace usa
   `anyhow = "1"`, `thiserror = "2"`. Seguimos o estilo com
   `clap = "4"`. Permite atualizações minor gratuitas; impede
   quebras de major.

5. **Features adiáveis**: `env`, `wrap_help`, `string` são
   "nice-to-have" do vanilla. Adiar até o código as usar evita
   inflar compile times sem ganho.

6. **Exit code 2 do clap alinha com convenção**: tests 114
   esperam `2` para argumentos inválidos; clap faz isso por
   default. Alinhamento gratuito.

---

## Estado pós-Passo 115

### CLI final

```
$ typst --help                    # ajuda automática
$ typst --version                 # versão automática (0.1.0)
$ typst input.typ output.pdf      # compilação (inalterada)
$ typst                           # erro de args (exit 2 via clap)
$ typst inexistente.typ o.pdf     # erro I/O (exit 2)
```

Diagnostic output (stderr) inalterado — formato ADR-0045.
Pipeline (eval → layout → export_pdf) inalterado.

### Trabalho futuro identificado

1. **`-o/--output` com default derivado** (escopo b). Útil
   quando `--format` aparecer.
2. **`--root`, `--font-path`** (escopo c). Cada passo dedicado
   com integração `SystemWorld::new(root, main)` /
   `SystemWorld::with_fonts(...)`.
3. **`--color`** — requer detecção isatty + `NO_COLOR` env var.
4. **`-f/--format`** para PNG, SVG, HTML (quando exports
   existirem).
5. **Subcomandos** — começar por `compile` + `watch`.
6. **Features extra de clap** quando forem necessárias (`env`
   para `TYPST_ROOT`, `wrap_help`, `string`).
