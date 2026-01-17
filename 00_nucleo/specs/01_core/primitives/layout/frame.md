# 🧬 Crystal Facet: layout/frame.rs

> **Crystal Face**: The Positioned Manifold — Layout Result Container.

---

## 💎 Facet DNA

$$
\text{Frame} = \text{Size} + \text{Baseline} + \text{Items}^*
$$

**frame.rs** defines the **Positioned Manifold** — the result of layout containing positioned items.

---

## Prescriptive Axioms

### Axiom I: Compositional Aggregation

$$
\text{Frame} = \{(\text{Point}, \text{Item})\}^*
$$

Frames aggregate **positioned items**.

---

### Axiom II: Baseline Anchor

$$
\text{Frame}.\text{baseline} \to \text{vertical reference}
$$

Frames carry a **baseline** for text alignment.

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE POSITIONED MANIFOLD (frame.rs)              │
├──────────────────────────────────────────────────────────┤
│  Role: Layout result container                           │
│  Contents: Size + Baseline + Items                       │
└──────────────────────────────────────────────────────────┘
```
