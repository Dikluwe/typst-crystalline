# 🧬 Crystal Facet: inline/fusion.rs (collect.rs)

> **Crystal Face**: The Shaping Fusion — Style-Continuous Identity Merging.

---

## 💎 Facet DNA

$$
\text{fuse} : \text{Pairs} \to \text{ShapingUnits}
$$

**fusion.rs** merges adjacent content into **shaping units** based on style continuity.

---

## Prescriptive Axioms

### Axiom I: Shaping Unity Axiom

$$
\text{style}(t_i) = \text{style}(t_{i+1}) \Rightarrow \text{fuse}(t_i, t_{i+1})
$$

**Shaping Unity Axiom**: Where there is **style continuity**, there shall be **fusion of geometric identity** for width calculation. Adjacent text with identical style properties merges into a single shaping unit.

---

### Axiom II: Boundary Preservation

$$
\text{style}(t_i) \neq \text{style}(t_{i+1}) \Rightarrow \text{boundary}
$$

Style discontinuity creates a **boundary** between shaping units.

---

### Axiom III: Non-Text Isolation

$$
\text{NonText} \Rightarrow \text{isolated unit}
$$

Non-text elements (boxes, equations) form **isolated units** — they cannot fuse with adjacent content.

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    SHAPING UNITY                                │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   Input:  [t₁ bold][t₂ bold][t₃ italic][t₄ italic][t₅ bold]     │
│                                                                 │
│   Fusion: [█ t₁+t₂ █][░ t₃+t₄ ░][█ t₅ █]                        │
│                                                                 │
│   Units:     Unit 1      Unit 2     Unit 3                      │
│            (bold)      (italic)    (bold)                       │
│                                                                 │
│   Same style → fuse for unified shaping                         │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE SHAPING FUSION (inline/fusion.rs)           │
├──────────────────────────────────────────────────────────┤
│  Role: Style-continuous identity merging                 │
│                                                          │
│  Laws:                                                   │
│    ✓ Shaping Unity Axiom — fuse on style continuity      │
│    ✓ Boundary Preservation — split on style change       │
│    ✓ Non-Text Isolation — non-text stays separate        │
└──────────────────────────────────────────────────────────┘
```
