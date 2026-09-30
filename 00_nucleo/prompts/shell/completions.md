# L0 — Completion script generation
Hash do Código: b41bb03c

**Layer:** L2
**Exclusive consumer:** `02_shell/src/completions.rs`

## Evidence and ownership

`02_shell/src/completions.rs:10` consumes the typed shell selector and delegates to official generators using the real command tree. Parsing, selector spelling and public carriers belong to `00_nucleo/prompts/shell/cli.md`; stdout publication belongs to `00_nucleo/prompts/wiring/completions.md`. These are references, not additional consumers of this prompt.

Shell-name acceptance is CLI syntax. Generated script structure is observable, but generator implementation and cross-version byte identity are not language-parity requirements (ADR-0107/0108).

## Contract

`pub fn generate(shell: CompletionShell) -> Vec<u8>` generates in memory from `crate::cli::command()`, itself derived from `Args::command()`. Its closed, exhaustive mapping supports `bash`, `elvish`, `fish`, `nushell`, `powershell` and `zsh`.

Use `clap_complete`'s official generators for the five non-Nushell variants and `clap_complete_nushell::Nushell` for Nushell. No hand-written template, duplicate argument model or independent command filter is permitted. A command hidden in help, such as `query`, need not disappear from completion scripts.

The result is nonempty and deterministic for the same build and command tree. The generator does not read or write user files or configuration and does not print. Dependencies remain in L2, not L1.

The command envelope writes all returned bytes to stdout, with exit 0 and no stderr on success. Its write failure is exit 2 with `error: failed to write completions: {error}`. Unknown shell names fail in the CLI parser with exit 2, before generation.

## Acceptance

Check each supported selector, real command/option coverage, repeatability and Bash regression. Nushell must contain `module completions {` and `export extern typst`. Check hidden-query coverage where emitted by the generator. Do not freeze an entire third-party script across dependency versions or assert byte identity with vanilla.
