# L0 — wiring/tests/crystalline_lint
Hash do Código: 8eeeec37

**Layer:** L4
**Exclusive consumer:** `04_wiring/tests/crystalline_lint.rs`

## Current evidence

- `04_wiring/tests/crystalline_lint.rs:20` — `temp_project`.
- `04_wiring/tests/crystalline_lint.rs:29` — `write_file`.
- `04_wiring/tests/crystalline_lint.rs:38` — `setup_project_dirs`.
- `04_wiring/tests/crystalline_lint.rs:85` — `type_level_violation_ecow_ecomap`.
- `04_wiring/tests/crystalline_lint.rs:116` — `type_level_allowed_ecow_ecostring`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/tests/crystalline_lint.rs`.

## Owned contract

- Own integration of the type-level V14 whitelist, not the typst CLI observables. Invoke crystalline-lint from PATH against minimal temporary crystalline projects.
- Declare L0/L1, Rust grammar, entities port, V14 severity and ecow allowlist for EcoString/EcoVec. EcoMap import must report V14 and package ecow; EcoString must not report V14. Do not require the entire forbidden item path when the linter identifies the package.
- Combine stdout/stderr for the report and isolate/clean fixtures. This test does not consume the CLI-observables nucleus or authorize a linter implementation change.

## Verification and boundary

Run `cargo test -p typst --test crystalline_lint -- --test-threads=1` with the harness prerequisites above.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.


## Expressões locais verificadas — Clippy 2026-10-16

### Medição anterior à decisão

04_wiring/tests/crystalline_lint.rs:29 (clippy::ptr_arg); 04_wiring/tests/crystalline_lint.rs:38 (clippy::ptr_arg). O snapshot `00_nucleo/diagnosticos/clippy-expressions-2026-10-16/before.json.gz` conserva fontes/L0, HEAD, working tree não commitado, status, diffstat e UTC; initial-diagnostics.json conserva sugestões e spans. Os usos e tipos foram inspecionados antes da decisão. São representações Rust internas; nenhuma nova paridade upstream ou ganho de performance é alegado.

### Responsabilidade exclusiva e aceitação

Nos helpers privados write_file e setup_project_dirs, receber &Path em vez de &PathBuf; manter PathBuf no retorno temp_project e adicionar o import Path. Os chamadores continuam os mesmos, com coercion por empréstimo, sem cópias ou mudança de limpeza/processos/V14.

Fluxo contínuo ADR-0127: sem API pública, default de produto, fase, dependência ou supressão novos. L0 precede código e resselo restrito. Preservar inputs e resultados dos controles existentes, sem relaxar tolerâncias. Gate dos lints selecionados RED→GREEN; executar suíte workspace, build, Clippy all-targets, formato, Rustdoc e linhagem. Não criar testes que apenas espelhem a sintaxe Rust.
