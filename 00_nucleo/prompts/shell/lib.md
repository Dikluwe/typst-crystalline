# L0 — Shell crate composition
Hash do Código: 4d7aa3a5

**Layer:** L2
**Exclusive consumer:** `02_shell/src/lib.rs`

## Evidence and boundary

The shell crate exposes CLI parsing and terminal presentation. The original
construction sequence and ADR-0049/0050 provenance are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/shell/lib.md`.
The crate previously existed as a header-only stub; that historical state is
not the current contract.

## Contract

L2 owns the user interface: clap, argument parsing, color selection, terminal
diagnostic formatting and other high-level presentation primitives.

- `cli` (`02_shell/src/cli.rs`) owns argument parsing, `ColorWhen`, `RunIntent`,
  `parse()` and `resolve_colored_with`; its boundary contract is
  `00_nucleo/prompts/shell/cli.md`.
- `diagnostic` (`02_shell/src/diagnostic.rs`) owns the vanilla-mirroring human
  `SourceDiagnostic` formatter, already materialized sources and optional ANSI
  colors, under the human-format amendment to ADR-0045; see
  `00_nucleo/prompts/shell/diagnostic.md`.
- `duration` (`02_shell/src/duration.rs`) owns pure human presentation of
  `std::time::Duration`, without clock access or I/O, and the binding-free
  duration/watch-line boundary adapter; see
  `00_nucleo/prompts/shell/duration.md`.

L2 is the only layer importing clap. L3 remains I/O without argument parsing or
user-facing formatting. L4 composes L2's `RunIntent` and `format_diagnostic`
with the L3 pipeline. This owner governs the crate root, not the implementation
contracts of its independently owned submodules.

## Scope and possible extensions

Alternative structured formatters (`diagnostic_json.rs`/`diagnostic_sarif.rs`),
`subcommands/` organization if/when compile/watch/query and other commands
require it, and `formatters/` for progress bars or summary reports remain
possible future work. Their mention does not authorize a new module or product
surface without its own current L0.
