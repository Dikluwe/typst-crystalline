# 🧬 Crystal Facet: flow/

> **Crystal Face**: The Flow Layouter — Unidimensional Content Mapping to Multiregional Space.

---

## 💎 Facet DNA

$$
\text{Flow} : (\text{Content}_{1D}, \text{Regions}_n) \to \text{Fragment}
$$

**flow/** implements the **Flow Layouter** — mapping unidimensional content sequences onto a chain of finite regional manifolds.

---

## Prescriptive Axioms

### Axiom I: Accumulative Metric Laws

$$
\mu_{n+1} = \mu_n + h_n + \delta_{spacing}
$$

Content is positioned via **accumulative metrics**. Each element's position is determined by the cumulative measure of all preceding elements plus inter-element spacing.

---

### Axiom II: Multiregional Partitioning

$$
\text{Columns}(n) \Rightarrow \text{partition}(\text{Manifold}, n)
$$

The layout manifold can be **partitioned** into parallel column regions.

---

### Axiom III: Volume Erosion Law

$$
\text{Effective Volume} = \text{Region} - \text{Float}_{top} - \text{Footnote}_{bottom}
$$

**Volume Erosion Law**: Floats and footnotes act as forces that **erode** available space:
- **Floats** erode from top → down
- **Footnotes** erode from bottom → up

The main flow occupies the **effective zone** remaining after erosion.

---

### Axiom IV: Fragmented Continuity

$$
\text{Flow}(C) = \sum_{i=0}^n \text{Region}_i \cap \text{Slice}(C)
$$

The flow is the **union of content slices** projected onto a sequence of finite regional manifolds. Content continuity is fragmented across region boundaries.

---

### Axiom V: Break Propagation

$$
\text{pagebreak} \Rightarrow \text{advance}(\text{Manifold})
$$

Explicit breaks **advance** to the next regional manifold.

---

## Facet Files

| File | Role |
|------|------|
| `mod.rs` | Flow orchestration, configuration |
| `block.rs` | Block element projection |
| `collect.rs` | Geometric signature classification |
| `compose.rs` | Frame composition |
| `distribute.rs` | Regional partitioning |

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    FLOW GEOMETRY                                │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   Content (1D sequence) ══map══▶ Regions (finite manifolds)     │
│                                                                 │
│   Erosion:                                                      │
│     ┌────────────────────┐                                      │
│     │    Float (erodes ↓)│                                      │
│     ├────────────────────┤                                      │
│     │                    │                                      │
│     │   Effective Zone   │ ← main flow occupies this            │
│     │                    │                                      │
│     ├────────────────────┤                                      │
│     │ Footnote (erodes ↑)│                                      │
│     └────────────────────┘                                      │
│                                                                 │
│   Continuity: Slice(C) projected onto Region sequence           │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE FLOW LAYOUTER (flow/)                       │
├──────────────────────────────────────────────────────────┤
│  Role: 1D content → multiregional manifold mapping       │
│                                                          │
│  Laws:                                                   │
│    ✓ Accumulative Metric Laws — position by sum          │
│    ✓ Multiregional Partitioning — column division        │
│    ✓ Volume Erosion — float/footnote space reduction     │
│    ✓ Fragmented Continuity — slice union across regions  │
│    ✓ Break Propagation — manifold advancement            │
└──────────────────────────────────────────────────────────┘
```
