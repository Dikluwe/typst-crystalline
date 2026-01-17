# 🧬 Crystal Facet: transforms.rs

> **Crystal Face**: The Affine Projections — Linear Coordinate Space Distortion.

---

## 💎 Facet DNA

$$
\text{distort} : (\text{Frame}, M_{affine}) \to \text{Frame}'
$$

**transforms.rs** implements **Affine Projections** — linear distortions of the coordinate space.

---

## Prescriptive Axioms

### Axiom I: Linear Space Distortion

$$
\vec{p}' = M \cdot \vec{p} + \vec{t}
$$

**Linear Space Distortion**: Every point in the manifold is **linearly transformed** by an affine matrix.

$$
M = \begin{pmatrix} a & b \\ c & d \end{pmatrix}, \quad \vec{t} = \begin{pmatrix} t_x \\ t_y \end{pmatrix}
$$

---

### Axiom II: Composition Invariance

$$
M_1 \circ M_2 \circ \ldots \circ M_n = M_{composed}
$$

**Composition Invariance**: Multiple distortions on the same manifold **fuse into a single linear operator**. The order of composition is preserved.

---

### Axiom III: Transformation Primitives

$$
\text{Distortion} \in \{\text{Translate}, \text{Rotate}, \text{Scale}, \text{Skew}\}
$$

All distortions are compositions of primitive **linear operators**:
- **Translate**: Pure displacement
- **Rotate**: Angular displacement around origin
- **Scale**: Magnitude amplification/reduction
- **Skew**: Shear deformation

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    COMPOSITION INVARIANCE                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   Multiple transforms:                                          │
│     rotate(45°) → scale(2) → translate(10, 0)                   │
│                                                                 │
│   Fuse into single operator:                                    │
│     M_composed = T × S × R                                      │
│                                                                 │
│   Applied once:                                                 │
│     p' = M_composed · p                                         │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE AFFINE PROJECTIONS (transforms.rs)          │
├──────────────────────────────────────────────────────────┤
│  Role: Linear coordinate space distortion                │
│                                                          │
│  Laws:                                                   │
│    ✓ Linear Space Distortion — M·p + t                   │
│    ✓ Composition Invariance — fuse to single operator    │
│    ✓ Transformation Primitives — translate/rotate/scale  │
└──────────────────────────────────────────────────────────┘
```
