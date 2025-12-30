# 🧬 Crystal Facet: typst-cli/watch.rs

> **Crystal Face**: The Watch Command — Continuous Compilation.

---

## 💎 Facet DNA

$$
\text{watch}(\text{input}) \to \text{continuous recompilation}
$$

**watch.rs** defines the **Watch Command** — monitoring files and recompiling on change.

---

## ⚠️ Purity Violation

$$
\text{watch} \in \text{IMPURE}
$$

File system monitoring and I/O.

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE WATCH COMMAND (watch.rs)                    │
├──────────────────────────────────────────────────────────┤
│  Role: Continuous compilation                            │
│  IMPURE: File system watching                            │
└──────────────────────────────────────────────────────────┘
```
