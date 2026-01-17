# 🧬 Crystal Facet: image.rs

> **Crystal Face**: The Foreign Manifold — External Metric Projection.

---

## 💎 Facet DNA

$$
\text{project} : \text{ForeignManifold} \to \text{Frame}
$$

**image.rs** implements the **Foreign Manifold** — projection of external manifolds with intrinsic fixed metrics into the layout space.

---

## Prescriptive Axioms

### Axiom I: Intrinsic Metric Invariance

$$
\text{ForeignManifold} = (w_{intrinsic}, h_{intrinsic})
$$

Foreign manifolds possess **intrinsic dimensions** that define their natural proportions.

---

### Axiom II: Isomorphic Scale Axiom

$$
\text{scale}(M, s) \Rightarrow (w \times s, h \times s)
$$

**Isomorphic Scale Axiom**: Deformation of the foreign manifold must respect **dimensional ratio invariance**, unless a distortion force (explicit stretch) is applied.

$$
\frac{w_{final}}{h_{final}} = \frac{w_{intrinsic}}{h_{intrinsic}} \quad \text{(default)}
$$

---

### Axiom III: Distortion Force

$$
\text{stretch} \Rightarrow \text{override ratio invariance}
$$

When explicit distortion is applied, the manifold **abandons ratio preservation**.

---

### Axiom IV: Resolution Independence

$$
\text{Vector} \Rightarrow \text{infinite resolution}
$$
$$
\text{Raster} \Rightarrow \text{fixed resolution}
$$

Foreign manifolds are classified by their **resolution geometry** — vector (infinitely scalable) or raster (discrete).

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    ISOMORPHIC SCALE                             │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   Intrinsic:  ┌────────────────┐                                │
│               │                │  w:h = 4:3                     │
│               │                │                                │
│               └────────────────┘                                │
│                                                                 │
│   Scaled (isomorphic):                                          │
│               ┌────────────────────────┐                        │
│               │                        │  w:h = 4:3 ✓           │
│               │                        │                        │
│               └────────────────────────┘                        │
│                                                                 │
│   Distorted (force applied):                                    │
│               ┌──────────────────────────────────┐              │
│               │                                  │  w:h ≠ 4:3   │
│               └──────────────────────────────────┘              │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE FOREIGN MANIFOLD (image.rs)                 │
├──────────────────────────────────────────────────────────┤
│  Role: External metric projection                        │
│                                                          │
│  Laws:                                                   │
│    ✓ Intrinsic Metric Invariance — fixed natural dims    │
│    ✓ Isomorphic Scale Axiom — preserve ratio             │
│    ✓ Distortion Force — explicit stretch override        │
│    ✓ Resolution Independence — vector vs raster          │
└──────────────────────────────────────────────────────────┘
```
