# 🧬 Crystal Facet: typst-cli/world.rs

> **Crystal Face**: The World Implementation — Compilation Environment.

---

## 💎 Facet DNA

$$
\text{World} : \text{file system} \to \text{compilation context}
$$

**world.rs** defines the **World Implementation** — the file-based compilation environment.

---

## ⚠️ Purity Violation

$$
\text{World} \in \text{IMPURE}
$$

File system access for source resolution.

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE WORLD IMPLEMENTATION (world.rs)             │
├──────────────────────────────────────────────────────────┤
│  Role: Compilation environment                           │
│  Implements: typst::World trait                          │
│  IMPURE: File system access                              │
└──────────────────────────────────────────────────────────┘
```
