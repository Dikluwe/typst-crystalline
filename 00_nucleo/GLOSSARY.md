# 📖 Typst Crystalline Glossary

> Key terms and concepts for navigating the project.

---

## Crystalline Architecture

| Term | Definition |
|------|------------|
| **Nucleus (00_nucleo)** | Layer zero: documentation, specifications, ADRs, and contracts. Contains no executable code. |
| **Core (01_core)** | Fundamental compiler crates. Dependencies flow outward, never inward. |
| **Shell (02_shell)** | User interfaces: CLI and IDE. Consumes the core without modifying it. |
| **Infra (03_infra)** | Exporters (PDF, SVG, HTML) and auxiliary tools. |
| **Wiring (04_wiring)** | Orchestration facade. Unifies all layers into a cohesive API. |

---

## Compilation Phases

| Phase | Input → Output | Crate |
|-------|----------------|-------|
| **Parsing** | `&str` → `SyntaxNode` | `typst-syntax` |
| **Evaluation** | `Source` → `Module` (Content + Scope) | `typst-eval` |
| **Realization** | `Content` → Layoutable elements | `typst-realize` |
| **Layout** | `Content` → `Frame[]` (one per page) | `typst-layout` |
| **Export** | `Frame[]` → PDF/SVG/PNG/HTML | `typst-pdf`, `typst-svg`, etc. |

---

## Compiler Concepts

| Term | Definition |
|------|------------|
| **SyntaxNode** | Concrete syntax tree node. Preserves whitespace and comments. Parsing never fails. |
| **Span** | Unique identifier for a syntax node. Used to trace errors back to source code. |
| **Content** | Core Typst type. Represents typographic content to be laid out. |
| **Frame** | Layout result. Contains positioned elements ready for export. |
| **Region** | Available space for layout (e.g., page area). |
| **Module** | Result of evaluating a file. Contains `Content` and a `Scope` with bindings. |
| **World** | Interface for system dependencies (files, fonts, configuration). |
| **Vm** | Interpreter virtual machine. Maintains the scope stack during evaluation. |
| **LinkedNode** | Abstraction over `SyntaxNode` for IDE, with access to parents and siblings. |
| **Scope** | Symbol table containing variable and function definitions. |
| **Closure** | Function that captures lexical scope variables (immutable, by value). |
| **Show Rule** | Content transformation rule (applied during the realization phase). |
| **Introspection** | Iterative process for resolving dynamic dependencies (e.g., cross-references, page numbering). |

---

## Fundamental Types (Foundations)

| Type | Definition |
|------|------------|
| **none** | Null value. Represents absence of value. |
| **auto** | Automatic value. Lets the compiler decide. |
| **bool** | Boolean value (`true` or `false`). |
| **int** | 64-bit signed integer. |
| **float** | 64-bit floating-point number. |
| **decimal** | Arbitrary-precision decimal number. |
| **str** | Immutable UTF-8 string. |
| **bytes** | Raw byte sequence. |
| **array** | Ordered heterogeneous list. Immutable by default. |
| **dictionary** | Key-value map. Keys are strings, values are heterogeneous. |
| **function** | First-class function. Can be native or user-defined. |
| **selector** | Pattern for selecting document elements. |
| **regex** | Regular expression for text matching. |
| **datetime** | Date and/or time. Supports timezone. |
| **duration** | Time interval (difference between datetimes). |
| **version** | Semantic version number. |
| **label** | Identifier for referencing document elements. |
| **symbol** | Typographic symbol (e.g., emojis, special characters). |
| **module** | Result of evaluating a source file. |
| **plugin** | Dynamically loaded WebAssembly extension. |
| **type** | Metadata of a Typst type. |
| **args** | Function arguments (positional and named). |

---

## Incrementality

| Term | Definition |
|------|------------|
| **comemo** | Incremental compilation framework used by Typst. Memoizes results of pure functions. |
| **Introspection Loop** | Loop that re-runs layout until results stabilize (max 5 iterations). Resolves cyclic dependencies like TOC. |
| **Capture** | External variables captured by closures. Capture stability improves incrementality. |

---

## Project Crates

### 01_core (Fundamentals)

| Crate | Responsibility |
|-------|----------------|
| `typst-syntax` | Parser and syntax tree definition |
| `typst-eval` | Typst language interpreter |
| `typst-realize` | Realization subsystem (show rule application) |
| `typst-layout` | Layout engine |
| `typst-library` | Typst standard library (functions, types) |
| `typst-macros` | Procedural macros for the compiler |
| `typst-utils` | Shared utilities |
| `typst-timing` | Performance measurement |

### 02_shell (Interfaces)

| Crate | Responsibility |
|-------|----------------|
| `typst-cli` | Command-line interface |
| `typst-ide` | IDE functionality (autocomplete, hover, etc.) |

### 03_infra (Exporters)

| Crate | Responsibility |
|-------|----------------|
| `typst-pdf` | PDF exporter |
| `typst-svg` | SVG exporter |
| `typst-html` | HTML exporter |
| `typst-render` | Pixel buffer renderer |
| `typst-kit` | Default implementations for CLI |

### 04_wiring (Orchestration)

| Crate | Responsibility |
|-------|----------------|
| `typst` | Main facade unifying all parts |

---

## Distillation (Antigravity)

| Term | Definition |
|------|------------|
| **Specification (Spec)** | `.md` file describing guarantees and purpose of a Rust module, not its implementation. |
| **Law vs. Implementation** | Specs describe "the Law" (contracts, invariants), not "the Code" (implementation details). |
| **Folder Isomorphism** | Spec structure in `00_nucleo/specs/` mirrors the code structure. |
| **Justified Impurity** | I/O in core is allowed only with documented justification. |

---

## Useful Commands

```bash
# Verify build
source $HOME/.cargo/env && cargo check --workspace

# Run tests
cargo test --workspace

# Build CLI release
cargo build -p typst-cli --release

# Check formatting
cargo fmt --check
```

---

## Dependency Flow

```
┌─────────────────────────────────────────────────────┐
│                    04_wiring                        │
│                      typst                          │
└─────────────────────────────────────────────────────┘
         ▲                              ▲
         │                              │
┌─────────────────────┐    ┌─────────────────────────┐
│     02_shell        │    │       03_infra          │
│  typst-cli          │    │  typst-pdf, typst-svg   │
│  typst-ide          │    │  typst-html, typst-kit  │
└─────────────────────┘    └─────────────────────────┘
         ▲                              ▲
         │                              │
         └──────────────┬───────────────┘
                        │
         ┌──────────────┴───────────────┐
         │           01_core            │
         │  typst-syntax, typst-eval    │
         │  typst-layout, typst-library │
         │  typst-realize, typst-macros │
         └──────────────────────────────┘
```

> **Rule:** Arrows point to what is depended upon. Upper layers depend on lower layers, never the reverse.
