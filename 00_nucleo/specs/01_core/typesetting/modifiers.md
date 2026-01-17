# 🧬 Crystal Facet: modifiers.rs

> **Crystal Face**: The Geometric Filters — Topological Transformations.

---

## 💎 Facet DNA

$$
\text{filter} : \text{Frame} \to \text{Frame}'
$$

**modifiers.rs** implements **Geometric Filters** — topological transformations applied to laid-out frames.

---

## Prescriptive Axioms

### Axiom I: Clipping — Manifold Intersection

$$
\text{clip}(F, B) = F \cap B
$$

**Clipping as Manifold Intersection**: Content outside the clipping boundary **loses existential identity** for the renderer.

$$
\forall p \notin B: \text{visible}(p) := \bot
$$

---

### Axiom II: Move Transformation

$$
\text{move}(F, \vec{v}) = F + \vec{v}
$$

Pure **translation** without content alteration.

---

### Axiom III: Hide Filter

$$
\text{hide}(F) \Rightarrow \text{visible}(F) := \bot
$$

The entire frame **loses visibility**.

---

### Axiom IV: Stroke Application

$$
\text{stroke}(F, \sigma) = F \cup \partial_\sigma(F)
$$

Add a **stroke boundary** to the frame.

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    MANIFOLD INTERSECTION (CLIP)                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   Content:              Clip boundary:                          │
│   ┌────────────────┐    ┌──────────┐                            │
│   │ ▓▓▓▓▓▓▓▓▓▓▓▓▓▓ │    │          │                            │
│   │ ▓▓▓▓▓▓▓▓▓▓▓▓▓▓ │ ∩  │          │                            │
│   │ ▓▓▓▓▓▓▓▓▓▓▓▓▓▓ │    │          │                            │
│   └────────────────┘    └──────────┘                            │
│                                                                 │
│   Result:                                                       │
│         ┌──────────┐                                            │
│         │▓▓▓▓▓▓▓▓▓▓│  ← only intersection visible               │
│         │▓▓▓▓▓▓▓▓▓▓│                                            │
│         └──────────┘                                            │
│                                                                 │
│   Content outside boundary: existential identity lost           │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE GEOMETRIC FILTERS (modifiers.rs)            │
├──────────────────────────────────────────────────────────┤
│  Role: Topological transformations                       │
│                                                          │
│  Laws:                                                   │
│    ✓ Clipping — manifold intersection, existence loss    │
│    ✓ Move — pure translation                             │
│    ✓ Hide — visibility negation                          │
│    ✓ Stroke — boundary addition                          │
└──────────────────────────────────────────────────────────┘
```
