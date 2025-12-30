# 🧬 Crystal Facet: inline/finalize.rs

> **Crystal Face**: The Vertical Stacker — Inter-Manifold Metrics.

---

## 💎 Facet DNA

$$
\text{stack} : \text{Lines} \to \text{Fragment}
$$

**finalize.rs** stacks line manifolds vertically using **inter-manifold metrics**.

---

## Prescriptive Axioms

### Axiom I: Inter-Manifold Metric Law

$$
\Delta y = \text{descent}(L_i) + \text{ascent}(L_{i+1}) + \tau
$$

**Inter-Manifold Metric Law**: The distance between two consecutive baselines is the sum of:
1. The **descent magnitude** of the upper line
2. The **ascent magnitude** of the lower line
3. The **tension constant** (leading/line spacing)

---

### Axiom II: Baseline Positioning

$$
y_{baseline}(L_{i+1}) = y_{baseline}(L_i) + \Delta y
$$

Each line's baseline is positioned at the **cumulative inter-manifold distance**.

---

### Axiom III: Fragment Bounding

$$
\text{Fragment.height} = \text{ascent}(L_0) + \sum \Delta y + \text{descent}(L_n)
$$

The fragment's total height spans from the first line's top to the last line's bottom.

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    INTER-MANIFOLD METRICS                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   Line 1:  ──────────────  baseline₁                            │
│              ascent↑                                            │
│            ▓▓▓▓▓▓▓▓▓▓                                           │
│              descent↓                                           │
│                                                                 │
│            ═══════════════ Δy = descent + ascent + τ            │
│                                                                 │
│              ascent↑                                            │
│            ▓▓▓▓▓▓▓▓▓▓▓▓▓                                        │
│              descent↓                                           │
│   Line 2:  ──────────────  baseline₂                            │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE VERTICAL STACKER (inline/finalize.rs)       │
├──────────────────────────────────────────────────────────┤
│  Role: Inter-manifold metrics                            │
│                                                          │
│  Laws:                                                   │
│    ✓ Inter-Manifold Metric Law — Δy = d + a + τ          │
│    ✓ Baseline Positioning — cumulative distance          │
│    ✓ Fragment Bounding — total height calculation        │
└──────────────────────────────────────────────────────────┘
```
