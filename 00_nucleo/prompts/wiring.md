# Wiring — typst-wiring
Hash do Código: 145e443b

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
- **Passo 119** (ADR-0050): formatter completamente em L2; drain
  inline em L4 (helper local `drain_to_stderr`).
- **Passo 120** (ADR-0051): `-o/--output` + default derivado em L2.
- **Passo 121** (ADR-0051): `--root` em L2; L4 consome `intent.root`
  directamente (sem `input.parent()` local).
- **Passo 122** (ADR-0051): `--font-path` (repetível) em L2; L4
  invoca `typst_infra::fonts::discover_fonts` + `.with_fonts(...)`.

## Contrato

### Uso

```bash
typst <INPUT> [OUTPUT] [-o FILE] [--root DIR] [--font-path DIR]... \
            [--color=auto|always|never]
typst --help
typst --version
```

### Pipeline

1. `typst_shell::cli::parse()` → `RunIntent { input, output, root,
   font_paths, colored }`.
2. `main_path = input.file_name()` — falha → exit 2.
3. `font_slots = discover_fonts(&font_paths)` (L3).
4. `SystemWorld::new(&root, &main_path).with_fonts(font_slots)` →
   `World`. Falha de `new` → exit 2.
5. `world.source(world.main())` → `Source`.
6. `compile_to_pdf_bytes(&world, &source)` (L3):
   - `eval` → `Module` + warnings.
   - `introspect` → `CounterState`.
   - `layout` → `PagedDocument`.
   - `export_pdf` → `Vec<u8>`.
7. `drain_to_stderr(&warnings, &source, path, colored)` — propaga
   `colored` do RunIntent.
8. Em sucesso: `fs::write(output, pdf_bytes)`. Exit 0.
9. Em erro de eval: drena errors com mesmo `colored`. Exit 1.

### Exit codes

- **0** — sucesso (PDF escrito).
- **1** — erro de compilação (eval gerou errors).
- **2** — argumentos inválidos (via clap em L2) ou erro de I/O.

### Diagnósticos

Formato gcc/clang (ADR-0045); cores ANSI (ADR-0048) via `colored`
do `RunIntent`. Tudo em stderr; stdout nunca usado.

## Separação de camadas (ADR-0049 + ADR-0050)

- **L2** (`02_shell`): `clap`, `Args`, `ColorWhen`, `resolve_colored_with`,
  `RunIntent`, `parse()`, `format_diagnostic`, paleta ANSI.
- **L3** (`03_infra`): pipeline, `SystemWorld`, export. Sem formatação
  user-facing (removida no Passo 119).
- **L4** (`04_wiring`): `main()` **thin**. Helper local
  `drain_to_stderr` (5 linhas) que aplica `format_diagnostic` +
  `eprint!`. Zero deps directas em `clap`; cria tipos? Não — só
  `PathBuf` locais e a função helper.

### Guardas

- **V12 do linter**: L4 não cria tipos. Satisfeito — nenhum struct,
  enum ou trait definido em `main.rs`.
- **`clap` não importado em L4**: `use clap::Parser` **não** aparece.
  Se aparecer em passo futuro, é sinal de que lógica escapou para
  cá e deve migrar para L2.

## Escopo futuro

Fora dos passos 113–122:

- Subcomandos (entram em L2).
- Flags funcionais (`--format`, `--ignore-system-fonts`, env vars,
  etc.) — entram em `Args` de L2, reflectem em `RunIntent`.
- JSON / SARIF — formatters em L3 ou L2.
- Outros exports (PNG, SVG, HTML).
- Virtualização de imports (resolução real contra `root`) — hoje
  `SystemWorld` ignora `root` para imports e usa `directory_of(
  current_file)`.
- Descoberta de system fonts (hoje `discover_fonts` só varre
  paths explícitos; vanilla varre também `fontdb::Database::load_system_fonts`).
