# 🧬 Crystal Facet: rules.rs

> **Crystal Face**: The Binding Vertex — Semantic-to-Geometric Induction.

---

## 💎 Facet DNA

$$
\text{bind} : \text{SemanticIdentity} \to \text{GeometricFacet}
$$

**rules.rs** is the **Binding Vertex** — the induction point where semantic node identity determines geometric manifestation.

---

## Prescriptive Axioms

### Axiom I: Behavior Induction Law

$$
\text{type}(e) \Rightarrow \text{Facet}(e)
$$

**Behavior Induction Law**: The semantic identity of a node **determines** which geometric facet is responsible for its physical manifestation.

$$
\text{BlockElem} \to \text{Flow}
$$
$$
\text{ParElem} \to \text{Inline}
$$
$$
\text{GridElem} \to \text{Grid}
$$
$$
\text{EquationElem} \to \text{Math}
$$

---

### Axiom II: Single Responsibility

$$
\forall e: |\text{Facets}(e)| = 1
$$

Each element type binds to **exactly one** geometric facet.

---

### Axiom III: Exhaustive Binding

$$
\forall e \in \text{Elements}: \exists f \in \text{Facets}: \text{bind}(e, f)
$$

Every element type has a **defined binding** — no semantic identity is orphaned.

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    BEHAVIOR INDUCTION                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   Semantic Identity          Geometric Facet                    │
│                                                                 │
│   BlockElem ──────────────→  flow/                              │
│   ParElem ────────────────→  inline/                            │
│   GridElem ───────────────→  grid/                              │
│   TableElem ──────────────→  grid/                              │
│   EquationElem ───────────→  math/                              │
│   ImageElem ──────────────→  image.rs                           │
│   ShapeElem ──────────────→  shapes.rs                          │
│   ...                                                           │
│                                                                 │
│   Identity determines manifestation                             │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE BINDING VERTEX (rules.rs)                   │
├──────────────────────────────────────────────────────────┤
│  Role: Semantic-to-geometric induction                   │
│                                                          │
│  Laws:                                                   │
│    ✓ Behavior Induction — identity → facet               │
│    ✓ Single Responsibility — one facet per type         │
│    ✓ Exhaustive Binding — no orphaned elements          │
└──────────────────────────────────────────────────────────┘
```
