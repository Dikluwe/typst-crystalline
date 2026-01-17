# 🧬 Crystal Facet: grid/lines.rs

> **Crystal Face**: The Separator Inferrer — Style-Based Line Resolution.

---

## 💎 Facet DNA

$$
\text{infer} : \text{Grid} \to \text{Separators}
$$

**lines.rs** infers **separator existence** from adjacent cell styles.

---

## Prescriptive Axioms

### Axiom I: Separator Inference Law

$$
\text{Separator}(i, j) = f(\text{style}(c_{left}), \text{style}(c_{right}))
$$

**Separator Inference Law**: The existence of a separator at position $(i, j)$ is a **function of the style resolution** of adjacent cells. Separators are not "generated" — they are logically inferred from the cellular context.

---

### Axiom II: Stroke Resolution

$$
\text{stroke}(s) = \text{resolve}(\text{left.stroke}, \text{right.stroke})
$$

When adjacent cells specify different strokes, a **resolution function** determines the final stroke.

---

### Axiom III: Segment Continuity

$$
\text{Segment} = [(x_0, y_i), (x_n, y_i)]
$$

Inferred separators form **continuous segments** across the grid.

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    SEPARATOR INFERENCE                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   Cell A │ Cell B     ← separator exists if                     │
│          ↑              style(A) or style(B) demands it         │
│      inferred                                                   │
│                                                                 │
│   Inference: Separator = f(adjacent styles)                     │
│   Resolution: stroke = resolve(left, right)                     │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE SEPARATOR INFERRER (grid/lines.rs)          │
├──────────────────────────────────────────────────────────┤
│  Role: Style-based line resolution                       │
│                                                          │
│  Laws:                                                   │
│    ✓ Separator Inference Law — f(adjacent styles)        │
│    ✓ Stroke Resolution — conflicting stroke handling     │
│    ✓ Segment Continuity — continuous line segments       │
└──────────────────────────────────────────────────────────┘
```
