# 🧬 Crystal Facet: inline/

> **Crystal Face**: The Inline Layouter — Linear Manifold Resolution.

---

## 💎 Facet DNA

$$
\text{layout}_{inline} : (\text{Content}_{inline}^*, \text{Region}) \to \text{Fragment}
$$

**inline/** implements the **Inline Layouter** — transforming symbolic content into justified linear manifolds within finite-width regions.

---

## Prescriptive Axioms

### Axiom I: Minimum Tension Principle

$$
\text{partition}(P) = \arg\min_{\Pi} \sum_{L \in \Pi} E(L)
$$

**Minimum Tension Principle**: The partition of a paragraph into linear manifolds must seek the **state of minimum energy**, where the global sum of metric distortions (spacing and hyphenation) is minimized.

$$
E(L) = \alpha \cdot \text{spacing\_distortion}(L) + \beta \cdot \text{hyphen\_penalty}(L)
$$

---

### Axiom II: Symbolic Morphology

$$
\text{shape}(\text{symbols}, \text{font}) \to \text{Glyphs}
$$

Text undergoes **symbolic morphology** — transformation from abstract symbols to positioned glyphs based on font identity.

---

### Axiom III: Bidirectional Reordering

$$
\text{BiDi}(\text{paragraph}) \to \text{visual order}
$$

Mixed-direction text undergoes **bidirectional reordering** for correct visual display.

---

### Axiom IV: Vacuum Distribution

$$
\text{justify}(L) \Rightarrow \text{distribute vacuum across spaces}
$$

Justified lines distribute excess width (**vacuum**) proportionally among space glyphs.

---

## Facet Files

| File | Role |
|------|------|
| `mod.rs` | Inline entry, configuration |
| `fusion.rs` | Shaping unity (style fusion) |
| `prepare.rs` | Metric manifestation |
| `shaping.rs` | Symbolic morphology |
| `linebreak.rs` | Tension resolution |
| `line.rs` | Linear manifold assembly |
| `finalize.rs` | Inter-manifold stacking |
| `deco.rs` | Associated line geometries |
| `box.rs` | Sub-manifold injection |

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE INLINE LAYOUTER (inline/)                   │
├──────────────────────────────────────────────────────────┤
│  Role: Linear manifold resolution                        │
│                                                          │
│  Laws:                                                   │
│    ✓ Minimum Tension Principle — global energy minimum   │
│    ✓ Symbolic Morphology — font-aware transformation     │
│    ✓ Bidirectional Reordering — correct visual order     │
│    ✓ Vacuum Distribution — spacing justification         │
└──────────────────────────────────────────────────────────┘
```
