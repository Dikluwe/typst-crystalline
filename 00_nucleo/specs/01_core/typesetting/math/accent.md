# 🧬 Crystal Facet: math/accent.rs

> **Crystal Face**: The Dependent Projection — Optical Axis Synchrony.

---

## 💎 Facet DNA

$$
\text{accent} : (\text{base}, \text{mark}) \to \text{Frame}
$$

**accent.rs** positions **accent marks** through optical axis synchrony.

---

## Prescriptive Axioms

### Axiom I: Optical Axis Synchrony

$$
x_{accent} = x_{base} + \text{opticalCenter}(base)
$$

**Optical Axis Synchrony**: Accents do not align to the **physical center** of the base — they align to the **optical center**, compensating for skew (italic slant) of the base glyph.

$$
\text{opticalCenter} = \frac{w}{2} + \text{skewCorrection}
$$

---

### Axiom II: Vertical Gap

$$
y_{accent} = y_{base} + h_{base} + \delta_{gap}
$$

Accents float above the base with a style-defined **gap**.

---

### Axiom III: Stretch Adaptation

$$
w_{accent} \geq w_{base} \Rightarrow \text{stretch}(\text{accent})
$$

Wide-stretch accents (like overlines) **adapt** to base width.

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    OPTICAL AXIS SYNCHRONY                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   Physical center:     │                                        │
│                      ┌─┴─┐                                      │
│                      │ ̂  │ ← accent                             │
│                      └───┘                                      │
│                    /     \                                      │
│                   /   A   \  ← italic base (skewed)             │
│                  /         \                                    │
│                                                                 │
│   Optical center:          │                                    │
│                          ┌─┴─┐                                  │
│                          │ ̂  │ ← accent (shifted for skew)      │
│                          └───┘                                  │
│                        /     \                                  │
│                       /   A   \                                 │
│                      /         \                                │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│       THE DEPENDENT PROJECTION (math/accent.rs)          │
├──────────────────────────────────────────────────────────┤
│  Role: Optical axis synchrony                            │
│                                                          │
│  Laws:                                                   │
│    ✓ Optical Axis Synchrony — skew compensation          │
│    ✓ Vertical Gap — style-defined separation             │
│    ✓ Stretch Adaptation — width matching                 │
└──────────────────────────────────────────────────────────┘
```
