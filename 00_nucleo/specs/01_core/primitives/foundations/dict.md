# 🧬 Crystal Facet: foundations/dict.rs

> **Crystal Face**: The Dictionary Type — Key-Value Mapping.

---

## 💎 Facet DNA

$$
\text{Dict} : \text{Str} \to \text{Value}
$$

**dict.rs** defines the **Dictionary Type** — string-keyed, heterogeneous mappings.

---

## Method Contracts

| Method | Contract |
|--------|----------|
| `len` | Entry count |
| `at` | Value access by key |
| `keys` / `values` / `pairs` | Iteration |
| `insert` / `remove` | Mutation (returns new) |
| `contains` | Key existence test |

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE DICTIONARY TYPE (dict.rs)                   │
├──────────────────────────────────────────────────────────┤
│  Role: Key-value mapping                                 │
│                                                          │
│  Properties:                                             │
│    ✓ String keys only                                    │
│    ✓ Heterogeneous values                                │
│    ✓ Insertion-ordered                                   │
└──────────────────────────────────────────────────────────┘
```
