# 🧬 Crystal Facet: pad.rs

> **Crystal Face**: The Domain Erosion — Metric Ring Subtraction.

---

## 💎 Facet DNA

$$
\text{erode} : (\text{Manifold}, \text{Insets}) \to \text{Manifold}_{effective}
$$

**pad.rs** implements **Domain Erosion** — subtracting a metric ring from the manifold boundary to define effective content space.

---

## Prescriptive Axioms

### Axiom I: Erosion Law

$$
\text{Space}_{effective} = \text{Space}_{original} - \text{Ring}_{insets}
$$

**Erosion Law**: The available space for content is the result of **subtracting a metric ring** (insets) from the original manifold boundary.

$$
w_{eff} = w - \text{left} - \text{right}
$$
$$
h_{eff} = h - \text{top} - \text{bottom}
$$

---

### Axiom II: Origin Translation

$$
\text{origin}_{content} = (\text{left}, \text{top})
$$

Content origin is **translated** inward by the erosion amounts.

---

### Axiom III: Frame Restoration

$$
\text{Frame}_{final} = \text{Frame}_{content} + \text{Ring}_{insets}
$$

The final frame **restores** the eroded dimensions.

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    DOMAIN EROSION                               │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   Original manifold:                                            │
│   ┌───────────────────────────────────────┐                     │
│   │                                       │                     │
│   │   Erosion ring (insets):              │                     │
│   │   ┌───────────────────────────────┐   │                     │
│   │   │                               │   │                     │
│   │   │      Effective space          │   │                     │
│   │   │      (content goes here)      │   │                     │
│   │   │                               │   │                     │
│   │   └───────────────────────────────┘   │                     │
│   │                                       │                     │
│   └───────────────────────────────────────┘                     │
│                                                                 │
│   Space_eff = Space_orig - Ring                                 │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE DOMAIN EROSION (pad.rs)                     │
├──────────────────────────────────────────────────────────┤
│  Role: Metric ring subtraction                           │
│                                                          │
│  Laws:                                                   │
│    ✓ Erosion Law — subtract insets from boundary         │
│    ✓ Origin Translation — shift content inward           │
│    ✓ Frame Restoration — restore dimensions in output    │
└──────────────────────────────────────────────────────────┘
```
