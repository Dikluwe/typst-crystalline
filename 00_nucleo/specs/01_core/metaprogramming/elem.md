# 🧬 Crystal Facet: elem.rs

> **Crystal Face**: The Element Transformer — Native Element Bridge.

---

## 💎 Facet DNA

$$
\text{\#[elem]} : \text{struct}_{rust} \to \text{NativeElement}
$$

**#[elem]** transforms a Rust struct into a **NativeElement** — a composable document element with fields, semantic behaviors, and styling support.

---

## Prescriptive Axioms

### Axiom I: Field Generation

$$
\forall f \in \text{fields}: \quad f \to (\text{accessor}, \text{builder}, \text{styler})
$$

Each field generates a **triad of methods**: accessor for reading, builder for construction, styler for style modification.

---

### Axiom II: Semantic Behavior Interfaces

$$
\text{Capability} : \text{Element} \to \text{Behavior}_{semantic}
$$

Capabilities are **Semantic Behavior Interfaces** that the element projects into the document:

| Interface | Semantic Behavior |
|-----------|-------------------|
| `Show` | Visual rendering projection |
| `Count` | Numbered sequence participation |
| `Locatable` | Reference target capability |
| `Construct` | Instantiation protocol |
| `Set` | Style rule application |

---

### Axiom III: Construct/Set Duality

$$
\text{Construct} : \text{Args} \to \text{Element}_{instance}
$$
$$
\text{Set} : \text{Args} \to \text{StyleChain}_{delta}
$$

Construction creates **instances**; setting creates **style deltas**. The duality enables both direct creation and style propagation.

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    ELEMENT CHAIN                                │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   #[elem] ══projects══▶ Semantic Behavior Interfaces            │
│                                                                 │
│   Show ────▶ Rendering (typst-layout)                           │
│   Construct ─▶ Evaluation (typst-eval)                          │
│   Set ──────▶ Styles (typst-realize)                            │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE ELEMENT TRANSFORMER (#[elem])               │
├──────────────────────────────────────────────────────────┤
│  Laws:                                                   │
│    ✓ Field Generation — accessor/builder/styler triad    │
│    ✓ Semantic Behavior Interfaces — capability projection│
│    ✓ Construct/Set Duality — instance vs style delta     │
└──────────────────────────────────────────────────────────┘
```
