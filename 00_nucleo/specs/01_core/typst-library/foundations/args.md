# 🧬 Crystal Facet: foundations/args.rs

> **Crystal Face**: The Invocation Manifold — Parameter Partition.

---

## 💎 Facet DNA

$$
\text{Args} = \text{Positional}^* \times \text{Named}^*
$$

**args.rs** defines the **Invocation Manifold** — the parameter space for function calls.

---

## Prescriptive Axioms

### Axiom I: Partition Invariant

$$
\text{Args} = [\text{Value}]_{positional} \uplus \{(\text{Name}, \text{Value})\}_{named}
$$

Arguments are **partitioned** into positional and named segments.

---

### Axiom II: Sink Absorption

$$
..\text{sink} \Rightarrow \text{absorb remaining}
$$

The sink pattern **absorbs** all unmatched arguments.

---

### Axiom III: Order Preservation

$$
\text{positional}[i] \prec \text{positional}[j] \quad \forall i < j
$$

Positional order is **preserved**.

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE INVOCATION MANIFOLD (args.rs)               │
├──────────────────────────────────────────────────────────┤
│  Role: Parameter partition                               │
│                                                          │
│  Invariants:                                             │
│    ✓ Partition — positional ⊎ named                      │
│    ✓ Sink Absorption — capture remaining                 │
│    ✓ Order Preservation — positional sequence            │
└──────────────────────────────────────────────────────────┘
```
