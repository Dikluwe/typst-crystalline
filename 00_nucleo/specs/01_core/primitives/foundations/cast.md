# 🧬 Crystal Facet: foundations/cast.rs

> **Crystal Face**: The Type Coercion — Value Transformation.

---

## 💎 Facet DNA

$$
\text{cast} : \text{Value}_A \to \text{Value}_B
$$

**cast.rs** defines **Type Coercion** — transformations between value types.

---

## Prescriptive Axioms

### Axiom I: Implicit Coercion

$$
\text{Int} \to \text{Float} \quad \text{(automatic)}
$$

Some coercions are **implicit** for convenience.

---

### Axiom II: Explicit Casting

$$
\text{int}("42") \to 42
$$

Type constructors perform **explicit** casting.

---

### Axiom III: Coercion Failure

$$
\text{cast}(\text{invalid}) \to \text{Error}
$$

Invalid coercions produce **errors**.

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE TYPE COERCION (cast.rs)                     │
├──────────────────────────────────────────────────────────┤
│  Role: Value transformation                              │
│                                                          │
│  Laws:                                                   │
│    ✓ Implicit Coercion — Int → Float                     │
│    ✓ Explicit Casting — type(value)                      │
│    ✓ Coercion Failure — error on invalid                 │
└──────────────────────────────────────────────────────────┘
```
