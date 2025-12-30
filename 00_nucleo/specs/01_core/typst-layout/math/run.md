# 🧬 Crystal Facet: math/run.rs

> **Crystal Face**: The Horizontal Manifold — Symmetry Axis Composition.

---

## 💎 Facet DNA

$$
\text{compose} : \text{Quanta}^* \to \text{Manifold}_{horizontal}
$$

**run.rs** implements the **Horizontal Manifold** — composing symbolic quanta along the symmetry axis.

---

## Prescriptive Axioms

### Axiom I: Adjacency Invariant

$$
\text{vacuum}(q_i, q_{i+1}) = f(\text{class}(q_i), \text{class}(q_{i+1}))
$$

**Adjacency Invariant**: The vacuum (spacing) between two quanta is a **strict function of their class signatures** (Operator, Variable, Relation, Punctuation, etc.).

| Left \ Right | Ord | Op | Rel | Open | Close | Punct |
|--------------|-----|-----|-----|------|-------|-------|
| Ord | thin | med | thick | 0 | 0 | 0 |
| Op | med | med | thick | 0 | 0 | 0 |
| Rel | thick | thick | 0 | 0 | 0 | 0 |
| ... | ... | ... | ... | ... | ... | ... |

---

### Axiom II: Axis Alignment

$$
\forall q \in \text{Manifold}: q.center_y = \text{MathAxis}
$$

All quanta are aligned on the **Symmetry Axis**.

---

### Axiom III: Horizontal Accumulation

$$
x_{i+1} = x_i + w_i + \text{vacuum}(q_i, q_{i+1})
$$

Quanta accumulate horizontally with **class-determined spacing**.

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    ADJACENCY INVARIANT                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   a  +  b  =  c                                                 │
│   ↑  ↑  ↑  ↑  ↑                                                 │
│   Ord Bin Ord Rel Ord                                           │
│                                                                 │
│   Spacing: │thin│med│med│thick│thin│                            │
│                                                                 │
│   vacuum(Ord, Bin) = medium                                     │
│   vacuum(Ord, Rel) = thick                                      │
│   vacuum(Rel, Ord) = thick                                      │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│       THE HORIZONTAL MANIFOLD (math/run.rs)              │
├──────────────────────────────────────────────────────────┤
│  Role: Symmetry axis composition                         │
│                                                          │
│  Laws:                                                   │
│    ✓ Adjacency Invariant — vacuum = f(class, class)      │
│    ✓ Axis Alignment — center on Math Axis                │
│    ✓ Horizontal Accumulation — class-spaced positioning  │
└──────────────────────────────────────────────────────────┘
```
