# 🧬 Crystal Facet: typst-cli/update.rs

> **Crystal Face**: The Update Command — Package Management.

---

## 💎 Facet DNA

$$
\text{update}(\text{package}^?) \to \text{updated dependencies}
$$

**update.rs** defines the **Update Command** — updating package dependencies.

---

## ⚠️ Purity Violation

$$
\text{update} \in \text{IMPURE}
$$

Network access and file system write.

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE UPDATE COMMAND (update.rs)                  │
├──────────────────────────────────────────────────────────┤
│  Role: Package management                                │
│  IMPURE: Network + file I/O                              │
└──────────────────────────────────────────────────────────┘
```
