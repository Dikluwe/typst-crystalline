# 🧬 Crystal Facet: introspection/locator.rs

> **Crystal Face**: The Identity Generator — Unique Position Assignment.

---

## 💎 Facet DNA

$$
\text{Locator} : \text{Element} \to \text{UniqueID}
$$

**locator.rs** defines the **Identity Generator** — assigns unique identities to elements for introspection.

---

## Prescriptive Axioms

### Axiom I: Unique Assignment

$$
\forall e: \text{Locator}(e) = \text{id}_{unique}
$$

Each element receives a **unique identity**.

---

### Axiom II: Stability Across Iterations

$$
\text{id}(e, n) = \text{id}(e, n+1)
$$

Element IDs remain **stable** across compilation iterations.

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE IDENTITY GENERATOR (locator.rs)             │
├──────────────────────────────────────────────────────────┤
│  Role: Unique position assignment                        │
│                                                          │
│  Properties:                                             │
│    ✓ Unique Assignment — each element distinct           │
│    ✓ Stability — IDs persist across iterations          │
└──────────────────────────────────────────────────────────┘
```
