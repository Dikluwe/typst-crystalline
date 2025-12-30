# 🧬 Crystal Layer: 03_infra/typst-kit/

> **Crystal Face**: The Toolkit — Shared Infrastructure.

---

## 💎 Crate DNA

$$
\text{kit} : \text{shared utilities for CLI/IDE}
$$

**typst-kit** provides shared infrastructure for CLI and IDE tools.

---

## ⚠️ Purity Status

$$
\text{IMPURE} : \text{network + file access}
$$

Contains I/O operations.

---

## Subsystems

| Module | Role |
|--------|------|
| fonts | Font loading |
| package | Package management |
| download | HTTP downloads |

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE TOOLKIT (typst-kit/)                        │
├──────────────────────────────────────────────────────────┤
│  Role: Shared infrastructure                             │
│  IMPURE: Network + file system                           │
└──────────────────────────────────────────────────────────┘
```
