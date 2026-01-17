# 🧬 Crystal Facet: ops.rs

> **Crystal Face**: The Operator Algebra — Domain Merging Engine.

---

## 💎 Facet DNA

$$
\text{op} : (\text{Domain}_A, \text{Domain}_B^?) \to \text{Domain}_C
$$

**ops.rs** implements the **Operator Algebra** — merging domains through binary and unary operations.

---

## Prescriptive Axioms

### Axiom I: Domain Merging Laws

$$
\text{Domain}_A \otimes \text{Domain}_B \to \text{Domain}_C
$$

Operations between values follow **Domain Merging Laws**. The result domain is determined by the algebraic properties of the operands.

---

### Axiom II: Unidirectional Coercion Geometry

$$
\text{Domain}_A \oplus \text{Domain}_B \to \text{Domain}_{\text{Superior}}
$$

**Law of Unidirectional Coercion**: Values of lower precision are **projected** into the domain of higher precision to guarantee operation integrity. Coercion never degrades.

| Operation | Coercion |
|-----------|----------|
| $\mathbb{Z} + \mathbb{R}$ | $\mathbb{Z} \to \mathbb{R}$ |
| Length + Ratio | Length (scaled) |
| Angle + Float | Angle (scaled) |

---

### Axiom III: Closure Preservation

$$
\text{op}(v_1, v_2) \in \text{Domain}_C
$$

Every operation **preserves closure** — the result is always a valid member of some domain.

---

### Axiom IV: Commutativity Selectivity

$$
a \oplus b = b \oplus a \text{ iff } \oplus \in \{+, \times, \land, \lor\}
$$

Only **select operations** are commutative. Non-commutative operations preserve operand order.

---

## Domain Hierarchy

```
┌─────────────────────────────────────────────────────────────────┐
│                    COERCION HIERARCHY                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   Int ────▶ Float ────▶ (higher precision)                      │
│                                                                 │
│   Length ←── Ratio (scaling)                                    │
│   Angle ←── Float (scaling)                                     │
│                                                                 │
│   Content ←── Str (conversion)                                  │
│                                                                 │
│   Coercion: Always ascends, never degrades                      │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE OPERATOR ALGEBRA (ops.rs)                   │
├──────────────────────────────────────────────────────────┤
│  Role: Domain merging engine                             │
│                                                          │
│  Laws:                                                   │
│    ✓ Domain Merging Laws — algebraic combination         │
│    ✓ Unidirectional Coercion — ascend, never degrade     │
│    ✓ Closure Preservation — result in valid domain       │
│    ✓ Commutativity Selectivity — operation-dependent     │
└──────────────────────────────────────────────────────────┘
```
