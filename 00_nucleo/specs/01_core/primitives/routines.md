# 🧬 Crystal Facet: routines.rs

> **Crystal Face**: The Dispatch Table — Evaluation and Layout Bindings.

---

## 💎 Facet DNA

$$
\text{Routines} : \text{Function Pointers for Core Operations}
$$

**routines.rs** defines the **Dispatch Table** — function pointers that bind library to evaluation/layout engines.

---

## Core Contracts

### The Routines Bundle

$$
\text{Routines} = \{eval, layout, realize, ...\}
$$

Routines provide **late-binding** between the library and compiler stages:
- **eval**: Expression evaluation
- **layout**: Content layout
- **realize**: Style realization

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE DISPATCH TABLE (routines.rs)                │
├──────────────────────────────────────────────────────────┤
│  Role: Evaluation and layout bindings                    │
│                                                          │
│  Purpose:                                                │
│    Late-binding between library and compiler crates      │
│    Avoids circular dependencies                          │
└──────────────────────────────────────────────────────────┘
```
