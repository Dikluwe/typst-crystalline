# 🧬 Crystal Facet: repeat.rs

> **Crystal Face**: The Lattice Saturation — Periodic Manifold Paving.

---

## 💎 Facet DNA

$$
\text{saturate} : (\text{Quantum}, \text{Boundary}) \to \text{Lattice}
$$

**repeat.rs** implements **Lattice Saturation** — periodic paving of manifolds until boundary limits are reached.

---

## Prescriptive Axioms

### Axiom I: Saturation Law

$$
\text{Lattice} = \{Q_0, Q_1, \ldots, Q_n\} \quad \text{where } n = \lfloor w / w_Q \rfloor
$$

**Saturation Law**: Atomic content must **saturate** the available space by replicating its geometric signature until boundary limits.

---

### Axiom II: Periodic Replication

$$
Q_i = Q_0 + i \cdot \vec{d}_{period}
$$

Each replica is **displaced** by a period vector from the previous.

---

### Axiom III: Boundary Truncation

$$
Q_n \cap \partial \text{Boundary} \neq \emptyset \Rightarrow \text{truncate}(Q_n)^?
$$

Replicas may be **truncated** at boundary edges (or may overflow, depending on mode).

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    LATTICE SATURATION                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   Atomic quantum:  ██                                           │
│                                                                 │
│   Boundary:        ├────────────────────────────────┤           │
│                                                                 │
│   Saturation:      ██  ██  ██  ██  ██  ██  ██  ██               │
│                    └──────────────────────────────┘             │
│                                                                 │
│   Replicate quantum until boundary is saturated                 │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE LATTICE SATURATION (repeat.rs)              │
├──────────────────────────────────────────────────────────┤
│  Role: Periodic manifold paving                          │
│                                                          │
│  Laws:                                                   │
│    ✓ Saturation Law — replicate to fill space            │
│    ✓ Periodic Replication — displacement by period       │
│    ✓ Boundary Truncation — edge handling                 │
└──────────────────────────────────────────────────────────┘
```
