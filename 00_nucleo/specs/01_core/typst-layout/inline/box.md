# 🧬 Crystal Facet: inline/box.rs

> **Crystal Face**: The Sub-Manifold — Baseline-Anchored Injection.

---

## 💎 Facet DNA

$$
\text{inject} : (\text{SubManifold}, \text{anchor}) \to \text{Line}
$$

**box.rs** injects **sub-manifolds** (inline boxes) into the linear flow with baseline anchoring.

---

## Prescriptive Axioms

### Axiom I: Baseline Anchoring Axiom

$$
\text{SubManifold}.anchor = \text{Line}.baseline
$$

**Baseline Anchoring Axiom**: A sub-manifold injected into the linear flow must declare an **internal anchor point** to coincide with the line's symmetry axis (baseline).

---

### Axiom II: Anchor Types

$$
\text{Anchor} \in \{\text{top}, \text{horizon}, \text{bottom}\}
$$

Sub-manifolds specify their **anchor type**:
- **top**: Top edge aligns with baseline
- **horizon**: Center aligns with baseline
- **bottom**: Bottom edge aligns with baseline

---

### Axiom III: Dimensional Contribution

$$
\text{Line.ascent} = \max(\text{Line.ascent}, \text{box.above\_anchor})
$$
$$
\text{Line.descent} = \max(\text{Line.descent}, \text{box.below\_anchor})
$$

The sub-manifold's dimensions **contribute** to the line's vertical extent based on anchor position.

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    BASELINE ANCHORING                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   Line baseline: ─────────────────────────────────              │
│                                                                 │
│   top anchor:    ┌───┐                                          │
│                  │box│ ← top edge on baseline                   │
│                  └───┘                                          │
│                                                                 │
│   horizon:           ┌───┐                                      │
│                  ────│box│──── ← center on baseline             │
│                      └───┘                                      │
│                                                                 │
│   bottom:                 ┌───┐                                 │
│                           │box│ ← bottom edge on baseline       │
│                       ────└───┘                                 │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE SUB-MANIFOLD (inline/box.rs)                │
├──────────────────────────────────────────────────────────┤
│  Role: Baseline-anchored injection                       │
│                                                          │
│  Laws:                                                   │
│    ✓ Baseline Anchoring Axiom — anchor = baseline        │
│    ✓ Anchor Types — top/horizon/bottom                   │
│    ✓ Dimensional Contribution — extend line extent       │
└──────────────────────────────────────────────────────────┘
```
