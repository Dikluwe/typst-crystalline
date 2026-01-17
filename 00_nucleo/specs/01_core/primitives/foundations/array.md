# 🧬 Crystal Facet: foundations/array.rs

> **Crystal Face**: The Ordered Heterogeneous Manifold — Immutable Sequence Algebra.

---

## 💎 Facet DNA

$$
\text{Array} = [\text{Value}]^* \quad (\text{immutable})
$$

**array.rs** defines the **Ordered Heterogeneous Manifold** — an immutable sequence where mutations are projections to new crystals.

---

## Prescriptive Axioms

### Axiom I: Immutability Invariant

$$
\text{mutate}(A) \to A' \quad (A \neq A')
$$

**Immutability Invariant**: Mutation operations are **projections** onto new crystals. The original remains unchanged.

---

### Axiom II: Heterogeneous Containment

$$
\forall i, j: \text{type}(A[i]) \neq \text{type}(A[j]) \quad \text{(permitted)}
$$

Arrays permit **heterogeneous** element types.

---

### Axiom III: Index Totality

$$
A[i] \in \text{Value} \quad \forall i \in [0, |A|)
$$

All valid indices **map to values**.

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE ORDERED HETEROGENEOUS MANIFOLD (array.rs)   │
├──────────────────────────────────────────────────────────┤
│  Role: Immutable sequence algebra                        │
│                                                          │
│  Invariants:                                             │
│    ✓ Immutability — mutations project new crystals       │
│    ✓ Heterogeneous — mixed types permitted               │
│    ✓ Index Totality — complete mapping                   │
└──────────────────────────────────────────────────────────┘
```
