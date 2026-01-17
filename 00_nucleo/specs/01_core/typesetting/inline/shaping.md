# 🧬 Crystal Facet: inline/shaping.rs

> **Crystal Face**: The Symbolic Morphology — Contextual Glyph Projection.

---

## 💎 Facet DNA

$$
\text{morph} : (\text{Symbols}, \text{Font}) \to \text{Glyphs}
$$

**shaping.rs** implements **Symbolic Morphology** — transforming abstract symbols into positioned glyphs through font-defined projections.

---

## Prescriptive Axioms

### Axiom I: Contextual Influence Law

$$
\text{glyph}(c) = \pi_{font}(c, \text{context})
$$

**Contextual Influence Law**: The geometry of a glyph is not static — it is a **projection dependent on substitution and positioning operators** defined by the font's identity. Adjacent symbols influence each other's final form.

---

### Axiom II: OpenType Operator Application

$$
\text{GSUB}: \text{substitutions}
$$
$$
\text{GPOS}: \text{positioning adjustments}
$$

Font operators are applied in sequence:
1. **GSUB** operators perform glyph substitutions (ligatures, alternates)
2. **GPOS** operators adjust positioning (kerning, mark attachment)

---

### Axiom III: Script-Aware Morphology

$$
\text{script}(t) \Rightarrow \text{morph}_{script}
$$

Morphology is **script-aware** — Arabic, CJK, Latin, and other scripts use distinct transformation logic.

---

### Axiom IV: Cluster Integrity

$$
\text{cluster}(g_1, \ldots, g_n) = \text{atomic unit}
$$

Complex scripts form **clusters** — groups of glyphs that function as atomic units for cursor movement and selection.

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    SYMBOLIC MORPHOLOGY                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   Input symbols:  f  f  i                                       │
│                                                                 │
│   GSUB (liga):    f  f  i  →  ﬃ (single glyph)                  │
│                                                                 │
│   Input symbols:  A  V                                          │
│                                                                 │
│   GPOS (kern):    A V → A V (adjusted spacing)                  │
│                    ← →                                          │
│                                                                 │
│   Geometry depends on:                                          │
│     - Font identity (which operators exist)                     │
│     - Surrounding context (what triggers operators)             │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│       THE SYMBOLIC MORPHOLOGY (inline/shaping.rs)        │
├──────────────────────────────────────────────────────────┤
│  Role: Contextual glyph projection                       │
│                                                          │
│  Laws:                                                   │
│    ✓ Contextual Influence Law — projection depends on ctx│
│    ✓ OpenType Operator Application — GSUB then GPOS     │
│    ✓ Script-Aware Morphology — per-script logic         │
│    ✓ Cluster Integrity — atomic glyph groups            │
└──────────────────────────────────────────────────────────┘
```
