<h1 align="center">
  <img alt="Typst" src="https://user-images.githubusercontent.com/17899797/226108480-722b770e-6313-40d7-84f2-26bebb55a281.png">
</h1>

# Typst Crystalline

An experimental fork of [Typst](https://github.com/typst/typst), migrating the compiler into the Tekt Crystalline Architecture. This repository is independent of the upstream Typst project. Upstream documentation and behavior are references; they are not compatibility or support claims for this fork.

The upstream-oriented guide that previously occupied this README remains recoverable in Git checkpoint `tekt-construction-2026-10-18`. This file is the project guide for the fork.

## Project structure

| Area | Purpose |
|---|---|
| `00_nucleo/adr/` | Architectural decisions |
| `00_nucleo/prompts/` | L0 specifications that own productive consumers |
| `lab/results/construction/diagnostics/` | Preserved diagnostic reports, measurements, and validation receipts |
| `01_core/` | L1 domain and compiler core |
| `02_shell/` | L2 CLI and formatting |
| `03_infra/` | L3 I/O, fonts, export, and pipeline services |
| `04_wiring/` | L4 composition and executable entry points |
| `lab/` | Quarantined upstream source, experiments, tools, results, and build outputs |

The current parity reference is pinned to upstream commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree `cac836c07e4dec2a89219370428292393a3216b5`. Historical reports for earlier pins do not certify this target. See the [architectural decisions](00_nucleo/adr) and [L0 specifications](00_nucleo/prompts) for current scope and acceptance criteria.

## Build and validation

The Rust toolchain is pinned in `rust-toolchain.toml`. The product executable is the `typst` binary from `typst-wiring`; product build output is under `lab/target/product/`.

```sh
cargo build --locked -p typst-wiring --bin typst --release
cargo build --locked --workspace --all-targets
```

Project tools and CI instructions are under [`lab/tools/`](lab/tools), including [`lab/tools/ci/README.md`](lab/tools/ci/README.md). Full validation includes tests, Clippy, formatting, documentation, and prompt lineage. The full suite uses Linux, export observers, the pinned upstream oracle, and temporary fixtures in `/dev/shm`; consult the CI guide for resource limits and exact commands. Windows compilation is checked separately. A local binary is not approval to publish a release.

## Attribution

This fork derives from the open-source Typst project. See [`LICENSE`](LICENSE) and the [upstream repository](https://github.com/typst/typst) for project provenance. Upstream contribution rules apply to work submitted to the upstream repository; this fork has its own [contribution guide](CONTRIBUTING.md).
