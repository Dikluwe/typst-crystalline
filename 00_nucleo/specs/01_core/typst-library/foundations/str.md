# 🧬 Crystal Facet: foundations/str.rs

> **Crystal Face**: The String Type — Unicode Text Container.

---

## 💎 Facet DNA

$$
\text{Str} : \text{Unicode codepoints}^*
$$

**str.rs** defines the **String Type** — immutable Unicode text.

---

## Method Contracts

| Method | Contract |
|--------|----------|
| `len` | Cluster count |
| `at` | Cluster access |
| `slice` | Substring extraction |
| `contains` | Substring test |
| `find` / `position` | Search |
| `replace` | Substitution |
| `split` / `match` | Tokenization |
| `trim` | Whitespace removal |
| `upper` / `lower` | Case conversion |

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE STRING TYPE (str.rs)                        │
├──────────────────────────────────────────────────────────┤
│  Role: Unicode text container                            │
│                                                          │
│  Properties:                                             │
│    ✓ Immutable — operations return new strings           │
│    ✓ Grapheme-based — clusters, not codepoints           │
└──────────────────────────────────────────────────────────┘
```
