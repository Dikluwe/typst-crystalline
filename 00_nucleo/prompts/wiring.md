# Wiring — typst-wiring
Hash do Código: ae61905a

## Módulo
`04_wiring/src/main.rs`

## Propósito

CLI mínima do compilador cristalino (Passo 113, ADR-0046).

## Contrato

### Uso

```bash
typst <input.typ> <output.pdf>
```

Positional. 2 argumentos obrigatórios. Sem flags.

### Pipeline

1. `parse_args(&argv)` → `(input: PathBuf, output: PathBuf)` ou
   imprime usage e sai (exit 2).
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
- **2** — argumentos inválidos ou erro de I/O (SystemWorld::new,
  fs::write).

### Diagnósticos (stderr)

Formato gcc/clang via ADR-0045:

```text
input.typ:3:11: warning: text: propriedade 'font' ainda não suportada
  hint: ver ADR-0040 para propriedades cobertas por set text
```

Nenhum output em stdout. PDF é escrito directamente para ficheiro.

## Escopo futuro

Explicitamente fora deste passo (ADR-0046):

- Subcomandos (`watch`, `query`, `init`, `eval`, `fonts`, …).
- Flags (`--root`, `--font-path`, `--format`, `--output`, `-`).
- Cores ANSI, JSON, SARIF.
- Outros exports (PNG, SVG, HTML).
- `sys.inputs`.
- `argparsing` declarativo (clap) — candidato quando flags forem
  adicionadas.

## Argparsing — manual

`std::env::args()` + `match` sobre `&args[..]`. Motivo: `clap`
não está em `[workspace.dependencies]` hoje. Migração para clap
fica para passo que adicione flags (ver ADR-0046).
