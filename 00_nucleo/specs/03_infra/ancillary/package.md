# 🧬 Crystal Facet: typst-kit/package.rs

> **Crystal Face**: The Package Manager — Package Resolution.

---

## 💎 Facet DNA

$$
\text{package}(\text{spec}) \to \text{package path}
$$

**package.rs** manages package resolution and caching.

---

## ⚠️ Purity Violation

$$
\text{package} \in \text{IMPURE}
$$

Network + file system access.

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE PACKAGE MANAGER (package.rs)                │
├──────────────────────────────────────────────────────────┤
│  Role: Package resolution                                │
│  IMPURE: Network + file I/O                              │
└──────────────────────────────────────────────────────────┘
```
