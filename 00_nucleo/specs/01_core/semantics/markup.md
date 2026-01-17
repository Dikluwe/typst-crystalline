# 🧬 Crystal Facet: markup.rs

> **Crystal Face**: The Markup Evaluator — Textual Geometry Transformer.

---

## 💎 Facet DNA

$$
\text{eval}_{markup} : \text{Markup} \to \text{Content}
$$

**markup.rs** implements the **Textual Geometry Transformer** — converting markup AST into content while preserving geometric relationships.

---

## Prescriptive Axioms

### Axiom I: Law of Flow Continuity

$$
\text{eval}([m_1, \ldots, m_n]) = m_1 \oplus \ldots \oplus m_n
$$

Markup children are **joined in flow order**. The composition operator $\oplus$ respects:
- **Geometric adjacency**: Adjacent elements maintain their spatial relationship
- **Flow direction**: Left-to-right, then top-to-bottom

---

### Axiom II: Space Collapse Invariant

$$
\text{eval}(\text{Space}^n) = \begin{cases}
\epsilon & \text{if boundary} \\
\text{Space} & \text{otherwise}
\end{cases}
$$

**Space Collapse Invariant**: Consecutive spaces collapse to:
- **Empty** ($\epsilon$) at boundaries (start, end, before/after blocks)
- **Single space** otherwise

This preserves textual geometry without physical redundancy.

---

### Axiom III: Line Break Geometry

$$
\text{eval}(\text{LineBreak}^n) = \begin{cases}
\text{Parbreak} & \text{if } n \geq 2 \\
\text{Space} & \text{if } n = 1 \land \text{inline}
\end{cases}
$$

Line breaks transform based on **geometric neighborhood**:
- Multiple breaks → paragraph break (block boundary)
- Single break in inline context → space

---

### Axiom IV: Text Element Preservation

$$
\text{eval}(\text{Text}) = \text{TextElem}
$$

Plain text evaluates to **text elements** with preserved content.

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    TEXTUAL GEOMETRY CHAIN                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   Markup AST ══eval══▶ Content                                  │
│                                                                 │
│   Space/Break Collapse:                                         │
│     "   text   "  →  "text"          (boundary collapse)        │
│     "a   b"       →  "a b"           (single space)             │
│     "a\n\nb"      →  "a" ¶ "b"       (paragraph break)          │
│     "a\nb"        →  "a b"           (inline: space)            │
│                                                                 │
│   Geometry preserved without redundancy                         │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE MARKUP EVALUATOR (markup.rs)                │
├──────────────────────────────────────────────────────────┤
│  Role: Textual geometry transformer                      │
│                                                          │
│  Laws:                                                   │
│    ✓ Flow Continuity — ordered composition               │
│    ✓ Space Collapse Invariant — no physical redundancy   │
│    ✓ Line Break Geometry — neighborhood-dependent        │
│    ✓ Text Element Preservation — content → TextElem      │
└──────────────────────────────────────────────────────────┘
```
