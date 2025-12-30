# Typst Crystalline Architecture - Gemini AI Configuration

## Project Overview

Typst is a markup-based typesetting system. This project follows the **Crystalline Architecture** for code organization.

## Layer Structure

| Layer | Path | Purpose |
|-------|------|---------|
| Nucleus | `00_nucleo/` | Docs, specs, ADRs, contracts |
| Core | `01_core/` | Fundamental compiler crates |
| Shell | `02_shell/` | User interfaces (CLI, IDE) |
| Infra | `03_infra/` | Exporters, tools, infrastructure |
| Wiring | `04_wiring/` | Facade and orchestration |

## Key Commands

```bash
# Verify build
source $HOME/.cargo/env && cargo check --workspace

# Run tests
cargo test --workspace

# Build CLI
cargo build -p typst-cli --release
```

## Dependency Flow

```
01_core  →  02_shell
   ↓           ↓
03_infra →  04_wiring
```

## Workflows Available

Use `/workflow-name` to trigger these workflows:
- `/build-check` - Verify build integrity
- `/add-crate` - Add a new crate to workspace
- `/move-crate` - Move crate between layers
- `/run-tests` - Execute test suites

## Important Files

- `Cargo.toml` - Workspace root with all dependency paths
- `.cursorrules` - Full AI configuration and rules
- `00_nucleo/legacy-docs/` - Original documentation
