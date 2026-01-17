# 🧬 Crystal Facet: math/cancel.rs

> **Crystal Face**: The Topological Interruption — Visibility Negation.

---

## 💎 Facet DNA

$$
\text{cancel} : \text{content} \to \text{Frame} + \text{negation}
$$

**cancel.rs** implements **Topological Interruption** — negating the visibility of content through overlaid geometry.

---

## Prescriptive Axioms

### Axiom I: Topological Interruption Axiom

$$
\text{cancel}(c) = c \oplus \text{NegationStroke}
$$

**Topological Interruption Axiom**: Cancellation creates a **visibility negation** by overlaying a stroke that topologically interrupts the content's visual continuity.

---

### Axiom II: Diagonal Negation Geometry

$$
\text{stroke} = \text{Line}((0, 0), (w, h)) \lor \text{Line}((0, h), (w, 0))
$$

The negation stroke spans the content **diagonally**.

---

### Axiom III: Content Preservation

$$
\text{cancel}(c).content = c
$$

The underlying content is **preserved** — cancellation is purely visual overlay, not deletion.

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    TOPOLOGICAL INTERRUPTION                     │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   Original:    ┌───────────┐                                    │
│                │   abc     │                                    │
│                └───────────┘                                    │
│                                                                 │
│   Cancelled:   ┌───────────┐                                    │
│                │ ╲ abc ╱   │  ← diagonal stroke overlays        │
│                │   ╲ ╱     │                                    │
│                └───────────┘                                    │
│                                                                 │
│   Content preserved, visibility topologically interrupted       │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│       THE TOPOLOGICAL INTERRUPTION (math/cancel.rs)      │
├──────────────────────────────────────────────────────────┤
│  Role: Visibility negation                               │
│                                                          │
│  Laws:                                                   │
│    ✓ Topological Interruption — visual overlay           │
│    ✓ Diagonal Negation Geometry — corner-to-corner       │
│    ✓ Content Preservation — content remains intact       │
└──────────────────────────────────────────────────────────┘
```
