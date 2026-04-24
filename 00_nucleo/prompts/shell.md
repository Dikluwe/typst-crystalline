# Shell — typst-shell
Hash do Código: 1ced4816

## Propósito

L2 — "CLI — interface com utilizador". Camada de shell que
conhece `clap`, argparsing, modo de coloração, formatação de
diagnostics para terminal e outras primitivas de apresentação
de alto nível ao utilizador.

Materializada no Passo 117 (ADR-0049) com `cli`. Completada no
Passo 119 (ADR-0050) com `diagnostic`. Antes disso, o crate
era stub (só header).

## Submódulos

- **`cli`** (`02_shell/src/cli.rs`) — argparsing, `ColorWhen`,
  `RunIntent`, `parse()`, `resolve_colored_with`. Ver prompt
  `00_nucleo/prompts/shell/cli.md`.
- **`diagnostic`** (`02_shell/src/diagnostic.rs`) — formatter
  gcc/clang para `SourceDiagnostic`, com suporte a cores ANSI.
  Ver prompt `00_nucleo/prompts/shell/diagnostic.md`.

## Evolução

Ficheiros futuros esperados:

- `diagnostic_json.rs` / `diagnostic_sarif.rs` — formatters
  estruturados alternativos.
- `subcommands/` — se/quando `compile`, `watch`, `query`,
  etc. forem adicionados como subcomandos.
- `formatters/` — progress bars, summary reports.

L2 é a **única** camada que importa `clap`. L3 permanece I/O
puro (sem argparsing, sem formatação user-facing). L4 é
composição pura (consome `RunIntent` e `format_diagnostic` de
L2 + pipeline de L3).
