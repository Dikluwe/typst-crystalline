# 🧬 Crystal Facet: lists.rs

> **Crystal Face**: The Indented Manifold Sequences — Hierarchical Offset Geometry.

---

## 💎 Facet DNA

$$
\text{list} : \text{Items} \to \text{IndentedSequence}
$$

**lists.rs** implements **Indented Manifold Sequences** — vertical sequences with hierarchical lateral offsets.

---

## Prescriptive Axioms

### Axiom I: Marker-Body Pairing

$$
\text{Item} = (\text{Marker}, \text{Body})
$$

Each item consists of a **marker** (bullet, number) and a **body** (content manifold).

---

### Axiom II: Cumulative Indentation Law

$$
\text{offset}(n) = n \times \vec{d}_{indent}
$$

**Cumulative Indentation Law**: Each nesting level **injects a lateral displacement vector** into the child manifold's origin.

$$
\text{origin}_{child} = \text{origin}_{parent} + \vec{d}_{indent}
$$

---

### Axiom III: Marker Alignment

$$
x_{marker} = x_{body} - w_{indent}
$$

Markers are positioned at a **fixed offset** before the body origin.

---

### Axiom IV: Vertical Accumulation

$$
y_{i+1} = y_i + h_i + \delta_{spacing}
$$

Items accumulate vertically with inter-item spacing.

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    CUMULATIVE INDENTATION                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   Level 0:  •  First item                                       │
│             •  Second item                                      │
│                                                                 │
│   Level 1:     ◦  Nested item (offset by d)                     │
│                ◦  Another nested                                │
│                                                                 │
│   Level 2:        ▪  Deeply nested (offset by 2d)               │
│                                                                 │
│   origin(L2) = origin(L0) + 2 × d_indent                        │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│       THE INDENTED MANIFOLD SEQUENCES (lists.rs)         │
├──────────────────────────────────────────────────────────┤
│  Role: Hierarchical offset geometry                      │
│                                                          │
│  Laws:                                                   │
│    ✓ Marker-Body Pairing — structural unit               │
│    ✓ Cumulative Indentation — nesting injects offset     │
│    ✓ Marker Alignment — fixed marker position            │
│    ✓ Vertical Accumulation — stacking with spacing       │
└──────────────────────────────────────────────────────────┘
```
