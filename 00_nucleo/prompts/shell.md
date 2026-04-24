# Shell — typst-shell
Hash do Código: 0bc34a3f

## Propósito

L2 — "CLI — interface com utilizador". Camada de shell que
conhece `clap`, argparsing, modo de coloração, e outras
primitivas de apresentação de alto nível ao utilizador.

Materializada no Passo 117 (ADR-0049). Antes disso, o crate
era stub (só header).

## Submódulos

- **`cli`** (`02_shell/src/cli.rs`) — ver prompt
  `00_nucleo/prompts/shell/cli.md`.

## Evolução

Ficheiros futuros esperados:

- `formatters/` — se formatters de output high-level
  aparecerem (progress bars, summary reports, etc.).
- `subcommands/` — se/quando `compile`, `watch`, `query`,
  etc. forem adicionados como subcomandos.

L2 é a **única** camada que importa `clap`. L3 permanece I/O
puro (sem argparsing). L4 é composição pura (consome `RunIntent`
de L2 e pipeline de L3).
