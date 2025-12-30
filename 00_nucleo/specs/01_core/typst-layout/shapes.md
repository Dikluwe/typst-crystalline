# 🧬 Crystal Facet: shapes.rs

> **Crystal Face**: The Primitive Topology — Geometric Command Genesis.

---

## 💎 Facet DNA

$$
\text{genesis} : \text{Parameters} \to \text{GeometricCommands}
$$

**shapes.rs** implements **Primitive Topology** — the genesis of geometric commands from parametric definitions.

---

## Prescriptive Axioms

### Axiom I: Boundary-Volume Duality Law

$$
\text{Primitive} = \text{Shell}_{stroke} \oplus \text{Core}_{fill}
$$

**Boundary-Volume Duality Law**: Each geometric primitive is defined by the tension between its **shell** (stroke) and its **core** (fill). Either may be absent.

$$
\text{Shell only} \Rightarrow \text{wireframe}
$$
$$
\text{Core only} \Rightarrow \text{solid}
$$
$$
\text{Both} \Rightarrow \text{filled with outline}
$$

---

### Axiom II: Primitive Classification

$$
\text{Primitive} \in \{\text{Line}, \text{Rectangle}, \text{Circle}, \text{Ellipse}, \text{Polygon}, \text{Path}\}
$$

Primitives are classified by their **topological genus**:
- **Line**: 1D manifold
- **Rectangle**: 2D orthogonal boundary
- **Circle/Ellipse**: 2D smooth boundary
- **Polygon**: 2D segmented boundary
- **Path**: Arbitrary curve

---

### Axiom III: Parametric Genesis

$$
\text{Circle}(r) \to \text{Commands}[(x, y) \mid x^2 + y^2 = r^2]
$$

Shape parameters **generate** rendering commands.

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    BOUNDARY-VOLUME DUALITY                      │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   Shell only:     ┌─────────────┐                               │
│   (stroke)        │             │   wireframe                   │
│                   └─────────────┘                               │
│                                                                 │
│   Core only:      ▓▓▓▓▓▓▓▓▓▓▓▓▓                                 │
│   (fill)          ▓▓▓▓▓▓▓▓▓▓▓▓▓   solid                         │
│                   ▓▓▓▓▓▓▓▓▓▓▓▓▓                                 │
│                                                                 │
│   Both:           ┌─────────────┐                               │
│                   │▓▓▓▓▓▓▓▓▓▓▓▓▓│   filled + outline            │
│                   └─────────────┘                               │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE PRIMITIVE TOPOLOGY (shapes.rs)              │
├──────────────────────────────────────────────────────────┤
│  Role: Geometric command genesis                         │
│                                                          │
│  Laws:                                                   │
│    ✓ Boundary-Volume Duality — shell ⊕ core              │
│    ✓ Primitive Classification — topological genus        │
│    ✓ Parametric Genesis — params → commands              │
└──────────────────────────────────────────────────────────┘
```
