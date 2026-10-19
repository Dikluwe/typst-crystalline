# Passo 120.A — Inventário e decisão de forma

**Data**: 2026-04-23

---

## Parte 1 — Vanilla

`lab/typst-original/crates/typst-cli/src/args.rs:293-312`:

```rust
#[derive(Debug, Clone, Args)]
pub struct CompileArgs {
    /// Path to input Typst file. Use `-` to read input from stdin.
    #[clap(value_parser = input_value_parser(), value_hint = ValueHint::FilePath)]
    pub input: Input,

    /// Path to output file (PDF, PNG, SVG, or HTML). Use `-` to write output to stdout.
    #[clap(
         required_if_eq("input", "-"),
         value_parser = output_value_parser(),
         value_hint = ValueHint::FilePath,
     )]
    pub output: Option<Output>,

    // ...
}
```

### Observações

- **`input`**: positional, required.
- **`output`**: positional, `Option<Output>`.
- **Regra de default**: `required_if_eq("input", "-")` — obrigatório
  apenas se input é stdin; caso contrário, opcional.
- **Sem `-o/--output` flag** — vanilla usa exclusivamente positional.
- **Default** (quando output omitido): derivado do input (no
  pipeline de compile, via `output_value_parser`).

---

## Parte 2 — Tests 114

`04_wiring/tests/cli.rs` (5 testes):

```rust
Command::new(BIN)
    .arg(&input)
    .arg(&output)   // positional, sempre ambos passados
```

Todos os 5 testes passam **positional**: `<input> <output>`. Nenhum
usa `-o`.

Se forma (a): 5 testes migrariam (cada um ganha `.arg("-o")` antes
do `&output`).
Se forma (c): 0 testes migram — positional continua a funcionar.

---

## Parte 3 — Decisão de forma

### Opções avaliadas

| Opção | Descrição | Alinha vanilla? | Tests 114 |
|-------|-----------|:---:|:---:|
| (a) Só flag | `typst input.typ -o output.pdf` | Não | Migram (5) |
| (b) Dupla aceitação | positional OU flag | Parcial | Intactos |
| (c) Positional + `-o` sinónimo | Ambos funcionam | Parcial | Intactos |

### Escolha: **(c) Positional + `-o` sinónimo + default derivado**

Razões:

1. **Vanilla usa positional** — manter compat preserva paridade
   funcional (ADR-0033).
2. **`-o` é convenção moderna** — gcc, clang, rustc, cargo-rustc
   usam `-o`. Adicionar como alternativa dá UX familiar sem
   perder positional.
3. **Default derivado alinha com vanilla** — quando output omitido,
   `input.with_extension("pdf")` (vanilla faz algo equivalente via
   parser).
4. **Tests 114 intactos** — zero fricção de migração.
5. **Regra de precedência clara**: se ambos positional e flag
   passados, **flag vence**. Explicit > implicit.

### Especificação

| Input | Output positional | `-o` flag | Output resolvido |
|-------|------------------|-----------|------------------|
| `in.typ` | — | — | `in.pdf` (default derivado) |
| `in.typ` | `out.pdf` | — | `out.pdf` (positional) |
| `in.typ` | — | `out.pdf` | `out.pdf` (flag) |
| `in.typ` | `pos.pdf` | `flag.pdf` | `flag.pdf` (flag vence) |

---

## Parte 4 — Estrutura `Args` prevista

```rust
struct Args {
    /// Input .typ file.
    input: PathBuf,

    /// Output PDF file (positional). Defaults to input with .pdf
    /// extension if omitted; `-o/--output` flag takes precedence.
    output: Option<PathBuf>,

    /// Output PDF file (flag alternative). Wins over positional if
    /// both are provided.
    #[arg(short = 'o', long = "output")]
    output_flag: Option<PathBuf>,

    #[arg(long = "color", value_enum, default_value_t = ColorWhen::Auto)]
    color: ColorWhen,
}
```

`parse()`:

```rust
let output = resolve_output(&args);

fn resolve_output(args: &Args) -> PathBuf {
    args.output_flag
        .clone()
        .or_else(|| args.output.clone())
        .unwrap_or_else(|| args.input.with_extension("pdf"))
}
```

### Clap viabilidade

`input: PathBuf` required + `output: Option<PathBuf>` optional
positional + `output_flag: Option<PathBuf>` flag — esta combinação
é suportada por clap. Positional `Option<T>` significa "opcional,
até 1 valor", não conflita com flag de mesmo nome subjacente.

**Conflito de nomes em clap**: `output` positional e flag `--output`
podem colidir no derive. Renomear flag para `output_flag` no campo
e usar `long = "output"` resolve — o help mostra `--output`, o
campo interno é `output_flag`. Permite separação.

---

## Conclusões

| Decisão | Escolha |
|---------|---------|
| Forma | **(c) Positional + `-o/--output` sinónimo** |
| Default | `input.with_extension("pdf")` quando ambos omitidos |
| Precedência | flag > positional > default derivado |
| Tests 114 | **Intactos** (positional continua a funcionar) |
| Nome do campo | `output_flag` (para evitar conflito com campo positional) |

Gate 120.A não dispara. **Pronto para 120.B (ADR) e 120.C (implementação)**.
