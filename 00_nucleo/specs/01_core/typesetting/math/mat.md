# 🧬 Crystal Facet: math/mat.rs

> **Crystal Face**: The Tabular Tensor — Orthogonal Channel Coherence.

---

## 💎 Facet DNA

$$
\text{tensor} : \text{Cells}_{m \times n} \to \text{Frame}
$$

**mat.rs** implements the **Tabular Tensor** — arranging quanta in a coherent orthogonal grid.

---

## Prescriptive Axioms

### Axiom I: Channel Coherence Law

$$
w_{col_j} = \sup_{i} \{w_{cell_{i,j}}\}
$$

**Channel Coherence Law**: In a mathematical tensor, the metric of a column is the **supremum of width demand** across all its constituents, guaranteeing manifold orthogonality.

---

### Axiom II: Row Coherence

$$
h_{row_i} = \sup_{j} \{h_{cell_{i,j}}\}
$$

Similarly, row height is the **supremum** across the row.

---

### Axiom III: Orthogonal Alignment

$$
\forall c_{i,j}: \text{align}(c_{i,j}, \text{column}_j.\text{anchor})
$$

Cells are **orthogonally aligned** within their channels (columns).

---

### Axiom IV: Axis Centering

$$
\text{center}(\text{Tensor}) = \text{MathAxis}
$$

The tensor is **centered on the Symmetry Axis**.

---

### Axiom V: Membrane Envelope

$$
h_{delimiters} = h_{tensor}
$$

Surrounding delimiters form a **reactive membrane** around the tensor.

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    CHANNEL COHERENCE                            │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   Column widths determined by supremum:                         │
│                                                                 │
│         col₀    col₁    col₂                                    │
│        ┌─────┬───────┬─────┐                                    │
│   row₀ │ a   │ bbb   │ c   │  w₁ = sup(3, 5, 4) = 5             │
│        ├─────┼───────┼─────┤                                    │
│   row₁ │ ddd │ eeeee │ ff  │                                    │
│        ├─────┼───────┼─────┤                                    │
│   row₂ │ g   │ hhhh  │ iii │                                    │
│        └─────┴───────┴─────┘                                    │
│                                                                 │
│   Each column expands to fit its widest cell                    │
│   Guarantees orthogonal alignment                               │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│       THE TABULAR TENSOR (math/mat.rs)                   │
├──────────────────────────────────────────────────────────┤
│  Role: Orthogonal channel coherence                      │
│                                                          │
│  Laws:                                                   │
│    ✓ Channel Coherence — col width = sup(cell widths)    │
│    ✓ Row Coherence — row height = sup(cell heights)      │
│    ✓ Orthogonal Alignment — column anchoring             │
│    ✓ Axis Centering — center on Math Axis                │
│    ✓ Membrane Envelope — delimiter scaling               │
└──────────────────────────────────────────────────────────┘
```
