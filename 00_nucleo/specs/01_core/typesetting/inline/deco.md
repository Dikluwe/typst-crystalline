# 🧬 Crystal Facet: inline/deco.rs

> **Crystal Face**: The Associated Line Geometries — Baseline-Parallel Projections.

---

## 💎 Facet DNA

$$
\text{project} : \text{Line} \to \text{Line} + \text{Geometries}
$$

**deco.rs** projects **associated line geometries** (underlines, strikethroughs) parallel to the baseline.

---

## Prescriptive Axioms

### Axiom I: Associated Line Geometry Law

$$
\text{Geometry}_{deco} = \text{Line}(\text{start}, \text{end}, y_{offset}, \mu_{stroke})
$$

**Associated Line Geometry Law**: Underlines and strikethroughs are **linear projections parallel to the baseline** whose magnitude (thickness) and offset are functions of the parent node's style signature.

---

### Axiom II: Offset Determination

$$
y_{underline} = \text{baseline} - \text{offset}_{style}
$$
$$
y_{strikethrough} = \text{baseline} + \text{x-height}/2
$$

Decorations are positioned at **style-determined offsets** from the baseline.

---

### Axiom III: Span Continuity

$$
\text{Decoration spans} = \text{content extent}
$$

Decorations span the **horizontal extent** of the decorated content.

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    ASSOCIATED LINE GEOMETRIES                   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   Content:      ████████████████                                │
│   Baseline:     ─────────────────                               │
│                                                                 │
│   Strikethrough:══════════════════  (y = baseline + x-height/2) │
│                                                                 │
│   Underline:    ══════════════════  (y = baseline - offset)     │
│                                                                 │
│   Geometries are parallel projections of the baseline           │
│   with style-determined offset and stroke                       │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│       THE ASSOCIATED LINE GEOMETRIES (inline/deco.rs)    │
├──────────────────────────────────────────────────────────┤
│  Role: Baseline-parallel projections                     │
│                                                          │
│  Laws:                                                   │
│    ✓ Associated Line Geometry Law — parallel projections │
│    ✓ Offset Determination — style-based positioning      │
│    ✓ Span Continuity — match content extent              │
└──────────────────────────────────────────────────────────┘
```
