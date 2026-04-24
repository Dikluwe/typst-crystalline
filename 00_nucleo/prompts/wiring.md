# Wiring — typst-wiring
Hash do Código: fd812420

## Módulo
`04_wiring/src/main.rs`

## Propósito

CLI mínima do compilador cristalino (Passo 113, ADR-0046;
argparsing migrado para `clap` no Passo 115, ADR-0047).

## Contrato

### Uso

```bash
typst <INPUT> <OUTPUT>
typst --help
typst --version
```

Positional. 2 argumentos obrigatórios. `--help` e `--version`
gratuitos via `clap` derive.

### Pipeline

1. `Args::parse()` (clap) → `(input: PathBuf, output: PathBuf)` ou
   imprime erro e sai (exit 2).
2. `root = input.parent()`, `main_path = input.file_name()`.
3. `SystemWorld::new(root, main_path)` → `World`. Falha → exit 2.
4. `world.source(world.main())` → `Source`.
5. `compile_to_pdf_bytes(&world, &source)` (L3):
   - `eval` → `Module` + warnings.
   - `introspect` → `CounterState`.
   - `layout` → `PagedDocument`.
   - `export_pdf` → `Vec<u8>`.
6. `drain_diagnostics_to_stderr(&warnings, &source, path)`
   (ADR-0045). Warnings primeiro (convenção gcc/clang).
7. Em sucesso: `fs::write(output, pdf_bytes)`. Exit 0.
8. Em erro de eval: `drain_diagnostics_to_stderr(&errors, ...)`.
   Exit 1.

### Exit codes

- **0** — sucesso (PDF escrito).
- **1** — erro de compilação (eval gerou errors).
- **2** — argumentos inválidos (clap decide mensagem e exit) ou
  erro de I/O (`SystemWorld::new`, `fs::write`).

### Diagnósticos (stderr)

Formato gcc/clang via ADR-0045:

```text
input.typ:3:11: warning: text: propriedade 'font' ainda não suportada
  hint: ver ADR-0040 para propriedades cobertas por set text
```

Mensagens de args errados (clap) também em stderr. Nenhum output
em stdout. PDF é escrito directamente para ficheiro.

## Escopo futuro

Explicitamente fora deste passo (ADR-0046 e ADR-0047):

- Subcomandos (`watch`, `query`, `init`, `eval`, `fonts`, …).
- Flags funcionais (`--root`, `--font-path`, `--format`,
  `-o/--output`, `-` stdin/stdout).
- Cores ANSI, JSON, SARIF.
- Outros exports (PNG, SVG, HTML).
- `sys.inputs`.
- Features extra de `clap` (`env`, `wrap_help`, `string`).

## Argparsing — `clap` derive (Passo 115, ADR-0047)

```rust
#[derive(Parser, Debug)]
#[command(name = "typst", version, about = "Typst compiler (crystalline)")]
struct Args {
    /// Input .typ file.
    input: PathBuf,
    /// Output PDF file.
    output: PathBuf,
}

fn main() -> ExitCode {
    let args = Args::parse();
    // ...
}
```

`version` = `true` lê `version` do `Cargo.toml`
(`version.workspace = true` → `"0.1.0"`).

`Args::parse()` em erro de args emite mensagem em stderr e sai
com exit 2 automaticamente — sem handling manual necessário.
