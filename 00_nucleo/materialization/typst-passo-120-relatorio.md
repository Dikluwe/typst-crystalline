# Passo 120 — Relatório (`-o/--output` flag + default derivado)

**Data**: 2026-04-23
**Precondição**: Passo 119 encerrado; 1017 total tests; zero
violations.
**Natureza**: primeira flag funcional; estabelece pattern.
**ADR criada**: ADR-0051 "Flags funcionais em L2" — **PROMOVIDA A
EM VIGOR** em 120.E.

---

## Sumário

CLI ganha flag `-o/--output` como alternativa ao positional
output. Se ambos omitidos, default é `input.with_extension("pdf")`.
Precedência: **flag > positional > default**.

Zero mudança de UX existente: tests 114 passam sem modificação
(positional continua a funcionar). UX melhorada: `typst input.typ`
agora compila para `input.pdf` sem precisar de path explícito.

ADR-0051 é **abrangente** — estabelece pattern para flags futuras
(`--root`, `--font-path`, etc.).

**811 L1 + 21 L2 (+6) + 186 L3 + 7 L4 (+2)** + 6 ignorados =
**1025 total** (+8 novos testes). Zero violations. **51 ADRs
activas** (+0051).

---

## 120.A — Inventário vanilla + decisão

Inventário em
`00_nucleo/diagnosticos/inventario-output-flag-passo-120.md`.

### Vanilla (lab/typst-original/crates/typst-cli/src/args.rs:293+)

```rust
pub input:  Input,         // positional required
pub output: Option<Output>,// POSITIONAL, required_if_eq("input", "-")
```

Sem `-o/--output` flag — apenas positional.

### Decisão: **(c) Positional + `-o/--output` sinónimo + default derivado**

| Razão | |
|-------|---|
| Alinha vanilla | Positional preservado |
| UX moderna | `-o` como gcc/clang/rustc |
| Tests 114 intactos | Positional continua |
| Default derivado | `typst input.typ` → `input.pdf` |

### Especificação de precedência

1. `-o/--output` flag vence.
2. Positional `output` (se presente).
3. Default: `input.with_extension("pdf")`.

---

## 120.B — ADR-0051

Criada em `00_nucleo/adr/typst-adr-0051-flags-funcionais.md`.
**EM VIGOR em 120.E**.

### Pattern estabelecido (aplicável a flags futuras)

- **P1**: L2 define campos em `Args` com derive clap.
- **P2**: L2 converte raw → resolvido em `parse() -> RunIntent`.
- **P3**: `RunIntent` cresce com campos prontos.
- **P4**: Defaults em L2, não em L4.
- **P5**: Validação profunda em L3/L4 (L2 não faz I/O).
- **P6**: Funções puras `resolve_xxx_with(...)` testáveis.

### Preview (não executados neste passo)

- `--root DIR` — `resolve_root(args) -> PathBuf` (fallback
  `input.parent()`).
- `--font-path DIR` (repetível) — `Vec<PathBuf>` passado raw; L3
  valida.

---

## 120.C — Implementação

### Ficheiros tocados

| Ficheiro | Mudança |
|----------|---------|
| `02_shell/src/cli.rs` | `Args.output` passa a `Option<PathBuf>`; novo campo `output_flag`; nova função `resolve_output_with`; 6 testes unit. |
| `04_wiring/tests/cli.rs` | +2 testes: `cli_output_omitido_deriva_de_input` + `cli_output_via_flag_o`. |
| `00_nucleo/prompts/shell/cli.md` | Documenta novo campo + `resolve_output_with` + 12 testes total. |

### `Args` — estrutura final

```rust
struct Args {
    /// Input .typ file.
    input: PathBuf,

    /// Output PDF file (positional). Defaults to input with `.pdf`
    /// extension if omitted. `-o/--output` flag takes precedence.
    output: Option<PathBuf>,

    /// Output PDF file. Alternative to the positional argument;
    /// wins if both are provided.
    #[arg(short = 'o', long = "output", value_name = "FILE")]
    output_flag: Option<PathBuf>,

    /// When to use coloured diagnostics.
    #[arg(long = "color", value_enum, default_value_t = ColorWhen::Auto)]
    color: ColorWhen,
}
```

**Nota sobre `output_flag`**: nome interno divergente de
`--output` (clap long) para evitar colisão com campo positional
`output`. Help mostra `-o, --output <FILE>`; interno usa
`args.output_flag`.

### `resolve_output_with` — função pura

```rust
pub fn resolve_output_with(
    input: &Path,
    output: Option<&PathBuf>,
    output_flag: Option<&PathBuf>,
) -> PathBuf {
    output_flag
        .cloned()
        .or_else(|| output.cloned())
        .unwrap_or_else(|| input.with_extension("pdf"))
}
```

**6 testes unitários** cobrem todas as combinações:

1. `resolve_output_flag_vence_positional` — ambos passados, flag vence.
2. `resolve_output_positional_usa_quando_sem_flag` — só positional.
3. `resolve_output_flag_usa_sem_positional` — só flag.
4. `resolve_output_ambos_omitidos_usa_default_derivado` — input.pdf.
5. `resolve_output_default_com_path_completo` — `/tmp/sub/file.typ` → `/tmp/sub/file.pdf`.
6. `resolve_output_default_sem_extensao_adiciona_pdf` — `noext` → `noext.pdf`.

### L4 — inalterado

`RunIntent { input, output, colored }` mantém mesma forma.
`output: PathBuf` (sempre resolvido). L4 não muda nada.

---

## 120.D — Validação manual

```bash
$ cat /tmp/test.typ
= Olá
Texto.

# 1. Default derivado (sem output)
$ ./typst /tmp/test.typ
$ ls /tmp/test.pdf           # existe ✓

# 2. Positional (compat tests 114)
$ ./typst /tmp/test.typ /tmp/out.pdf
$ ls /tmp/out.pdf            # existe ✓

# 3. Flag curto -o
$ ./typst /tmp/test.typ -o /tmp/flag.pdf
$ ls /tmp/flag.pdf           # existe ✓

# 4. Flag longo --output
$ ./typst /tmp/test.typ --output /tmp/long.pdf
$ ls /tmp/long.pdf           # existe ✓

# 5. Flag vence positional
$ ./typst /tmp/test.typ /tmp/pos.pdf -o /tmp/winner.pdf
$ ls /tmp/winner.pdf         # existe ✓
$ ls /tmp/pos.pdf            # NÃO existe ✓ (flag venceu)

# 6. --help mostra -o
$ ./typst --help | grep -- "-o,"
  -o, --output <FILE>
          Output PDF file. Alternative to the positional argument; wins if both are provided
```

**Todos os 6 cenários passam.**

---

## 120.E — Encerramento

### Verificação

```
$ cargo build --release
    Finished `release` profile [optimized] target(s) in 0.74s

$ cargo test --workspace | grep "test result"
test result: ok. 811 passed ...   (L1 inalterado)
test result: ok. 186 passed ...   (L3 inalterado)
test result: ok. 21 passed  ...   (L2 +6: resolve_output_*)
test result: ok. 7 passed   ...   (L4 +2: output omitido + flag -o)

$ crystalline-lint .
✓ No violations found
```

### ADR

**ADR-0051** `EM VIGOR`. Pattern para flags funcionais
estabelecido — `--root` e `--font-path` seguem em passos
dedicados.

---

## Números finais

| Métrica | Antes (Passo 119) | Depois |
|---------|------:|-------:|
| L1 tests | 811 | 811 |
| L2 tests | 15 | **21** (+6) |
| L3 tests | 186 | 186 |
| L4 tests | 5 | **7** (+2) |
| **Total** | **1017** | **1025** (+8) |
| Ignorados | 6 | 6 |
| Violations | 0 | 0 |
| ADRs activas | 50 | **51** (+0051) |
| DEBTs abertos | 11 | 11 |

---

## Limitações aceites (ADR-0051)

1. **Sem `-o -` para stdout**: `-` como path literal.
2. **Sem validação de paths em L2**: só falha no `fs::write` em L4.
3. **Overwrite silencioso**: se output existe, sobrescreve. Convenção gcc.
4. **Short `-o` só para `output`**: `-r`, `-f` reservados para
   futuro (subcomandos podem querer).

---

## Lições

1. **Form (c) é idioma intermédio**: não é "só flag" (diverge
   vanilla) nem "só positional" (sem UX moderna). Combina o
   melhor dos dois. Custo: campo interno `output_flag` com nome
   divergente do help (`--output`). Cosmético aceitável.

2. **Default derivado paga-se**: `typst input.typ` agora compila
   para `input.pdf` sem pensar. UX próxima de `rustc foo.rs` que
   compila para `foo`. Um dos ganhos imediatos mais visíveis.

3. **Precedência explícita**: flag > positional > default é
   regra consistente. Documentada em `--help` docstring
   ("wins if both are provided"). Tests cobrem as 4 combinações.

4. **`resolve_output_with` é testável em isolamento**: 6 testes
   unit em ~30 linhas cobrem lógica. Zero env mutation, zero I/O.
   Seguimento natural do padrão `resolve_colored_with` do Passo
   116.

5. **ADR abrangente paga-se depois**: ADR-0051 descreve pattern
   para `--root` e `--font-path` sem os executar. Passos futuros
   (121, 122?) seguem decisões já publicadas. Evita "reinventar"
   estilo a cada flag.

6. **`output_flag` naming dance**: clap derive não permite dois
   campos com mesmo nome; positional `output` + flag `--output`
   colidiriam. Solução: campo interno `output_flag`, clap long
   `"output"`. Funciona mas revela friction do derive macro.

---

## Estado pós-Passo 120

### CLI final

```bash
$ typst --help
Typst compiler (crystalline)

Usage: typst [OPTIONS] <INPUT> [OUTPUT]

Arguments:
  <INPUT>   Input .typ file
  [OUTPUT]  Output PDF file (positional). Defaults to input with `.pdf`
            extension if omitted. `-o/--output` flag takes precedence.

Options:
  -o, --output <FILE>
          Output PDF file. Alternative to the positional argument;
          wins if both are provided
      --color <COLOR>
          When to use coloured diagnostics
          [default: auto]
  -h, --help       Print help
  -V, --version    Print version
```

### Pattern aplicável a futuros passos

Próximas flags seguem o mesmo modelo:

```rust
// Em Args:
#[arg(long = "root", value_name = "DIR")]
root: Option<PathBuf>,

// resolve em L2:
fn resolve_root(args: &Args) -> PathBuf {
    args.root.clone()
        .or_else(|| args.input.parent().map(Path::to_path_buf))
        .unwrap_or_else(|| PathBuf::from("."))
}

// RunIntent ganha campo:
pub struct RunIntent {
    pub root: PathBuf,   // novo
    // ...
}

// L4 consome sem saber de clap:
SystemWorld::new(&intent.root, &main_path)?;
```

### Trabalho futuro identificado

1. **`--root`** — próximo passo. Pattern claro.
2. **`--font-path`** (repetível) — passo dedicado.
3. **`-f/--format`** — quando outros exports (PNG/SVG/HTML) existirem.
4. **Subcomandos** (`compile`, `watch`, `query`) — passos grandes.
5. **`-o -` para stdout** — passo dedicado se surgir procura.
