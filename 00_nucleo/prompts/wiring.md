# Wiring — typst-wiring
Hash do Código: fd812420

## Módulo
`04_wiring/src/main.rs`

## Propósito

**Composição pura** do compilador cristalino. L4 consome `RunIntent`
de L2 (`typst_shell::cli::parse()`) e orquestra o pipeline L3.

Passos relevantes:
- **Passo 113** (ADR-0046): CLI mínima.
- **Passo 115** (ADR-0047): `clap` argparsing.
- **Passo 116** (ADR-0048): cores ANSI.
- **Passo 117** (ADR-0049): CLI movida para L2; L4 é composição pura.

## Contrato

### Uso (inalterado desde Passo 116)

```bash
typst <INPUT> <OUTPUT> [--color=auto|always|never]
typst --help
typst --version
```

### Pipeline

1. `typst_shell::cli::parse()` → `RunIntent { input, output, colored }`.
2. `root = input.parent()`, `main_path = input.file_name()`.
3. `SystemWorld::new(root, main_path)` → `World`. Falha → exit 2.
4. `world.source(world.main())` → `Source`.
5. `compile_to_pdf_bytes(&world, &source)` (L3):
   - `eval` → `Module` + warnings.
   - `introspect` → `CounterState`.
   - `layout` → `PagedDocument`.
   - `export_pdf` → `Vec<u8>`.
6. `drain_diagnostics_to_stderr(&warnings, &source, path, colored)`
   — propaga `colored` do RunIntent.
7. Em sucesso: `fs::write(output, pdf_bytes)`. Exit 0.
8. Em erro de eval: drena errors com mesmo `colored`. Exit 1.

### Exit codes

- **0** — sucesso (PDF escrito).
- **1** — erro de compilação (eval gerou errors).
- **2** — argumentos inválidos (via clap em L2) ou erro de I/O.

### Diagnósticos

Formato gcc/clang (ADR-0045); cores ANSI (ADR-0048) via `colored`
do `RunIntent`. Tudo em stderr; stdout nunca usado.

## Separação de camadas (ADR-0049)

- **L2** (`02_shell`): `clap`, `Args`, `ColorWhen`, `resolve_colored_with`,
  `RunIntent`, `parse()`.
- **L3** (`03_infra`): `format_diagnostic(colored: bool)`,
  `drain_diagnostics_to_stderr`, pipeline.
- **L4** (`04_wiring`): `main()` **thin** (~75 linhas incluindo
  header + tratamento de erros I/O). Zero deps directas em `clap`;
  cria tipos? Não — só `PathBuf` locais.

### Guardas

- **V12 do linter**: L4 não cria tipos. Satisfeito — nenhum struct,
  enum ou trait definido em `main.rs`.
- **`clap` não importado em L4**: `use clap::Parser` **não** aparece.
  Se aparecer em passo futuro, é sinal de que lógica escapou para
  cá e deve migrar para L2.

## Escopo futuro

Fora dos passos 113–117:

- Subcomandos (entram em L2).
- Flags funcionais (`--root`, `--font-path`, `-o`, etc.) — entram
  em `Args` de L2, reflectem em `RunIntent`.
- JSON / SARIF — formatters em L3 ou L2.
- Outros exports (PNG, SVG, HTML).
