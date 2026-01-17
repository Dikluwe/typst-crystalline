# 🧬 Crystal Facet: foundations/content/field.rs

> **Crystal Face**: The Attribute Manifold — Semantic Property Space.

---

## 💎 Facet DNA

$$
\text{Manifold}_{\text{Attr}} : \text{Name} \to \text{Value}
$$

**field.rs** defines the **Attribute Manifold** — the property space of semantic atoms.

---

## Prescriptive Axioms

### Axiom I: Attribute Classification

$$
\text{Attribute} \in \{\text{Essential}, \text{Derived}\}
$$

Attributes are classified by their **origin**:
- **Essential**: Intrinsic to the atom, defined at creation
- **Derived**: Synthesized from context or other attributes

---

### Axiom II: Attribute Signature

$$
\text{Attr} = (\text{name}, \text{domain}, \text{intrinsic}^?)
$$

Each attribute has a **name**, a **value domain**, and an optional **intrinsic value**.

---

### Axiom III: Context Projection Law

$$
\text{value}_{final} = \text{intrinsic} \oplus \text{Forces}_{external}
$$

**Context Projection Law**: The final value of an attribute is the **superposition** of the atom's intrinsic value with external forces (set rules) present in the style manifold.

$$
\text{resolve}(a, \text{styles}) = a_{local} \oplus \text{styles}[a]
$$

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE ATTRIBUTE MANIFOLD (field.rs)               │
├──────────────────────────────────────────────────────────┤
│  Role: Semantic property space                           │
│                                                          │
│  Laws:                                                   │
│    ✓ Essential/Derived — attribute origin                │
│    ✓ Attribute Signature — name + domain                 │
│    ✓ Context Projection — intrinsic ⊕ external forces    │
└──────────────────────────────────────────────────────────┘
```
