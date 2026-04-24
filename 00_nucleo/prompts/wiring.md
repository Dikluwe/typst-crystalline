# Wiring — typst-wiring
Hash do Código: fd812420

## Módulo
`04_wiring/src/main.rs`

## Propósito

CLI mínima do compilador cristalino (Passo 113, ADR-0046;
argparsing migrado para `clap` no Passo 115, ADR-0047;
cores ANSI no Passo 116, ADR-0048).

## Contrato

### Uso

```bash
typst <INPUT> <OUTPUT> [--color=auto|always|never]
typst --help
typst --version
```

Positional. 2 argumentos obrigatórios. `--help`, `--version` e
`--color` via `clap` derive.

### Pipeline

1. `Args::parse()` (clap) → `(input, output, color)` ou imprime
   erro e sai (exit 2).
2. `colored = resolve_colored(&args.color)` — decisão via
   função pura (flag > `NO_COLOR` > isatty).
3. `root = input.parent()`, `main_path = input.file_name()`.
4. `SystemWorld::new(root, main_path)` → `World`. Falha → exit 2.
5. `world.source(world.main())` → `Source`.
6. `compile_to_pdf_bytes(&world, &source)` (L3):
   - `eval` → `Module` + warnings.
   - `introspect` → `CounterState`.
   - `layout` → `PagedDocument`.
   - `export_pdf` → `Vec<u8>`.
7. `drain_diagnostics_to_stderr(&warnings, &source, path, colored)`
   (ADR-0045, cores ADR-0048). Warnings primeiro (convenção
   gcc/clang).
8. Em sucesso: `fs::write(output, pdf_bytes)`. Exit 0.
9. Em erro de eval: `drain_diagnostics_to_stderr(&errors, ..., colored)`.
   Exit 1.

### Exit codes

- **0** — sucesso (PDF escrito).
- **1** — erro de compilação (eval gerou errors).
- **2** — argumentos inválidos (clap decide mensagem e exit) ou
  erro de I/O (`SystemWorld::new`, `fs::write`).

### Diagnósticos (stderr)

Formato gcc/clang via ADR-0045 + cores ADR-0048:

```text
input.typ:3:11: warning: text: propriedade 'font' ainda não suportada
  hint: ver ADR-0040 para propriedades cobertas por set text
```

Com `colored = true`: path em dim, severity em
vermelho/amarelo bold, message em bold, hint em ciano bold.

Mensagens de args errados (clap) também em stderr. Nenhum output
em stdout.

### `--color`

```
--color=auto    (default) Cores se stderr é terminal e NO_COLOR ausente.
--color=always  Cores sempre activas (mesmo em pipe).
--color=never   Cores sempre desactivadas.
```

Ordem de precedência:
1. Flag explícita vence tudo.
2. `NO_COLOR` env var (quando flag é `Auto`) desactiva.
3. `isatty(stderr)` (quando flag é `Auto` e `NO_COLOR` ausente)
   decide.

Lógica em `resolve_colored_with(choice, no_color_present, is_tty)`
— função pura testável.

## Escopo futuro

Explicitamente fora dos passos 113–116:

- Subcomandos (`watch`, `query`, `init`, `eval`, `fonts`, …).
- Flags funcionais (`--root`, `--font-path`, `--format`,
  `-o/--output`, `-` stdin/stdout, `--verbose`, `--quiet`).
- JSON / SARIF diagnostics.
- Outros exports (PNG, SVG, HTML).
- `sys.inputs`.
- Paleta ANSI customizável ou themes.
- Windows legacy ANSI compatibility.

## Argparsing — `clap` derive (Passo 115, ADR-0047)

```rust
#[derive(Parser, Debug)]
#[command(name = "typst", version, about = "Typst compiler (crystalline)")]
struct Args {
    input:  PathBuf,
    output: PathBuf,
    #[arg(long = "color", value_enum, default_value_t = ColorWhen::Auto)]
    color:  ColorWhen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
enum ColorWhen { Auto, Always, Never }
```
