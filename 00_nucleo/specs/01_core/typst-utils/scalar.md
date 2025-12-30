# 🧬 Crystal Facet: scalar.rs

> **Crystal Face**: The Ordered Float — Numeric Stability Domain.

---

## 💎 Facet DNA

$$
\text{Scalar} : \mathbb{R}_{stable} \subset \mathbb{R} \setminus \{\text{NaN}\}
$$

**Scalar** is the **Numeric Stability Domain** — a wrapper over floating-point that guarantees **total ordering**, **hashability**, and **NaN-freedom**. It is the unit of measure throughout Source and Layout.

---

## Geometric Essence

```mermaid
graph TD
    F64[f64: Partial Order] --> |stabilize| Scalar[Scalar: Total Order]
    Scalar --> Eq[Eq: Reflexive]
    Scalar --> Ord[Ord: Total]
    Scalar --> Hash[Hash: Deterministic]
    
    subgraph "Stability Barrier"
        NaN[NaN] --> |rejected| Zero[Scalar(0.0)]
    end
```

---

## Prescriptive Axioms

### Axiom I: Numeric Stability Law

$$
\forall s \in \text{Scalar}: \quad \neg\text{isNaN}(s)
$$

**Law of Numeric Stability**: NaN is **excluded from the domain** by construction. Construction from NaN yields zero:

$$
\text{Scalar::new}(\text{NaN}) = \text{Scalar}(0.0)
$$

This guarantees all downstream computations remain stable.

---

### Axiom II: Total Ordering

$$
\forall s_1, s_2 \in \text{Scalar}: \quad s_1 < s_2 \lor s_1 = s_2 \lor s_1 > s_2
$$

Scalar forms a **total order**. Comparison never fails, never returns undefined.

---

### Axiom III: Arithmetic Closure

$$
\forall s_1, s_2 \in \text{Scalar}, \oplus \in \{+, -, \times, \div\}: \quad s_1 \oplus s_2 \in \text{Scalar}
$$

**Domain Preservation**: Arithmetic operations are **closed** over the Scalar domain. Results that would produce NaN are sanitized to remain within the stability domain.

---

### Axiom IV: Bit-Stable Identity

$$
\text{hash}(s) = \text{hash}(s.\text{to\_bits}())
$$

Hashing uses **bit representation**, ensuring deterministic identity for equal values.

---

## Facet Table

| Facet | Operation | Signature | Purpose |
|-------|-----------|-----------|---------|
| **Construct** | `new` | $\mathbb{R} \to \text{Scalar}$ | NaN-sanitizing constructor |
| **Project** | `get` | $\text{Scalar} \to \mathbb{R}$ | Extract value |
| **Arithmetic** | `+, -, *, /` | $(\text{S}, \text{S}) \to \text{S}$ | Closed operations |
| **Compare** | `cmp` | $(\text{S}, \text{S}) \to \text{Ord}$ | Total ordering |

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    MEASUREMENT CHAIN                            │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   Scalar ══unit of══▶ Source (coordinates)                      │
│      │                                                          │
│      └══unit of══▶ Layout (dimensions, positions)               │
│                                                                 │
│   Stability barrier prevents NaN propagation throughout system  │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE NUMERIC STABILITY DOMAIN (Scalar)           │
├──────────────────────────────────────────────────────────┤
│  Role: NaN-free numeric domain for measurements          │
│                                                          │
│  Laws:                                                   │
│    ✓ Numeric Stability — NaN excluded by construction    │
│    ✓ Total Ordering — comparison always succeeds         │
│    ✓ Arithmetic Closure — operations preserve domain     │
│    ✓ Bit-Stable Identity — deterministic hashing         │
│                                                          │
│  Barrier: Prevents floating-point instability propagation│
└──────────────────────────────────────────────────────────┘
```
