# 🧬 Crystal Facet: math/morphology.rs (text.rs + shaping.rs)

> **Crystal Face**: The Symbolic Morphology — Variant Projection and Domain Injection.

---

## 💎 Facet DNA

$$
\text{morph} : (\text{Symbol}, \text{Style}, \text{Domain}) \to \text{Glyph}
$$

**morphology.rs** implements **Symbolic Morphology** — the projection of abstract symbols onto physical forms through style mapping and domain rules.

---

## Prescriptive Axioms

### Axiom I: Variant Morphology Law

$$
\text{glyph}(s) = \pi_{style}(s, \text{map})
$$

**Variant Morphology Law**: The physical form of a symbol is a **projection of its identity** onto a style map (bold, italic, calligraphic, fraktur, etc.).

$$
\text{map} : \text{Symbol} \times \text{Style} \to \text{Glyph}
$$

---

### Axiom II: Static Manifold Injection

$$
\text{text}_{math} \Rightarrow \text{upright projection}
$$

**Static Manifold Injection**: Textual content injected into mathematical space **loses its inclination property** (italic) to signal the change of semantic domain. Text in math is rendered upright (roman) to distinguish it from variables.

$$
\text{Text} \subset \text{Math} \Rightarrow \text{slant} := 0
$$

---

### Axiom III: Contextual Influence

$$
\text{glyph}_{final} = \text{context}(g_{prev}, g, g_{next})
$$

Adjacent symbols may influence each other's final form through **contextual operators** (ligatures, positional forms).

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    SYMBOLIC MORPHOLOGY                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   Style Map:                                                    │
│     x + italic → 𝑥                                              │
│     x + bold   → 𝐱                                              │
│     x + fraktur → 𝔵                                             │
│     x + script → 𝓍                                              │
│                                                                 │
│   Domain Injection:                                             │
│     "sin" in math → roman (static manifold)                     │
│     Variables → italic (dynamic manifold)                       │
│                                                                 │
│   The loss of slant signals: "this is from a foreign domain"   │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│       THE SYMBOLIC MORPHOLOGY (math/morphology.rs)       │
├──────────────────────────────────────────────────────────┤
│  Role: Variant projection and domain injection           │
│                                                          │
│  Laws:                                                   │
│    ✓ Variant Morphology — style map projection           │
│    ✓ Static Manifold Injection — text loses slant        │
│    ✓ Contextual Influence — adjacent symbol effects      │
└──────────────────────────────────────────────────────────┘
```
