# 🧬 Crystal Layer: 02_shell/typst-cli/

> **Crystal Face**: The Command-Line Interface — User Interaction Surface.

---

## 💎 Crate DNA

$$
\text{CLI} : \text{Args} \to \text{Output}
$$

**typst-cli** provides the command-line interface for Typst compilation and utilities.

---

## ⚠️ Purity Status

$$
\text{CLI} \in \text{IMPURE}
$$

All CLI operations perform I/O: file system, network, terminal.

---

## Command Structure

| Command | Role |
|---------|------|
| `compile` | Document compilation |
| `watch` | Continuous compilation |
| `query` | Introspection queries |
| `init` | Project initialization |
| `update` | Package updates |
| `fonts` | Font management |

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE CLI LAYER (typst-cli/)                      │
├──────────────────────────────────────────────────────────┤
│  Role: User interaction surface                          │
│  IMPURE: File system, network, terminal I/O              │
└──────────────────────────────────────────────────────────┘
```
