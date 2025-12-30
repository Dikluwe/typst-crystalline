# 🧬 Crystal Facet: typst-ide/jump.rs

> **Crystal Face**: The Source Navigation — Position Mapping.

---

## 💎 Facet DNA

$$
\text{jump}(\text{source/output}) \to \text{corresponding position}
$$

**jump.rs** defines **Source Navigation** — mapping between source and output positions.

---

## Navigation Modes

| Mode | Direction |
|------|-----------|
| Forward | Source → Output |
| Inverse | Output → Source |

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE SOURCE NAVIGATION (jump.rs)                 │
├──────────────────────────────────────────────────────────┤
│  Role: Position mapping                                  │
│  Features: forward/inverse jump                          │
└──────────────────────────────────────────────────────────┘
```
