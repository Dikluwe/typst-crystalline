# 🧬 Crystal Facet: foundations/value.rs

> **Crystal Face**: The Value Enum — Universal Type Container.

---

## 💎 Facet DNA

$$
\text{Value} = \bigcup_{T \in \mathcal{T}} T
$$

**value.rs** defines the **Value Enum** — the universal container for all Typst values.

---

## Prescriptive Axioms

### Axiom I: Type Universe Containment

$$
\text{Value} \in \{\text{None}, \text{Auto}, \text{Bool}, \text{Int}, \text{Float}, \text{Decimal}, \text{Str}, \ldots\}
$$

All concrete types are **variants** of the Value enum.

---

### Axiom II: Dynamic Typing

$$
\text{typeof}(\text{Value}) \to \text{Type}
$$

Every value carries its **type identity** for runtime dispatch.

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE VALUE ENUM (value.rs)                       │
├──────────────────────────────────────────────────────────┤
│  Role: Universal type container                          │
│                                                          │
│  Variants: None, Auto, Bool, Int, Float, Decimal,        │
│    Str, Bytes, Array, Dict, Func, Content, Symbol, ...   │
└──────────────────────────────────────────────────────────┘
```
