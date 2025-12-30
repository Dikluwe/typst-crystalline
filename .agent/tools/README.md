### 1. README.md (English Version)

# /tools — Maintenance & Automation

> **The Immune System.** Scripts that enforce structure and map the lattice.

## Purpose

This directory contains **automation scripts** and utilities designed to generate AI context maps (`_MAP.md`), enforce architectural rules, and reduce human toil. It acts as the "Cartographer" of the project.

---

## 💎 Mathematical Formalism ($\mathcal{L}_{tools}$)

The tools act as **Validation Operators** that ensure the project's state remains within the defined topological boundaries:

* **The Cartographic Mapping ($f_{map}$)**: Let $G$ be the Project Graph (files and folders). The Cartographer is a function $f: G \to C$ that projects the physical reality into a Context Model $C$ for AI agents.
* **Invariant Verification**: The tools execute an evaluation function $v(x)$ for every file.
$$v(x) =
\begin{cases}
1 & \text{if } x \text{ satisfies } \mathcal{L}\_n \text{ invariants} \\
0 & \text{otherwise (Trigger Warning/Error)}
\end{cases}$$

* **Consistency Closure**: The system is "Crystalline" if and only if the physical state matches the specification state ($State_{code} \equiv State_{spec}$). The tools enforce this identity.

---

## The Automation Mandate

> [!CAUTION]
> **Do not edit `_MAP.md` files manually.**
> The context maps are **generated artifacts**. Any manual changes will be overwritten. If you need to change a description, edit the file's "Magic Comment" (first line).

## Directory Structure

```
tools/
├── cartographer.rs  # Fractal Map Generator (Scans topology)
└── README.md        # This file

```

## Magic Comments

To populate the maps, the Cartographer reads the **first line** of your files:

* **Rust (`.rs`)**: Use `//!` at the very top.
* **Markdown/Scripts (`.md`, `.py`)**: Use `#` (title) or `#` (comment).

## Rules

1. **Self-Documentation**: Every code file MUST start with a magic comment.
2. **Automated Context**: AI Agents rely on `_MAP.md`; ensure the script runs before commits.
3. **No Ghost Files**: Files without magic comments appear as empty entries.
4. **Tool Integrity**: Tools must be stateless and auto-detect project root.

---
