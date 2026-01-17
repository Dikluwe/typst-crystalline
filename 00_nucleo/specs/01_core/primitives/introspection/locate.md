# 🧬 Crystal Facet: introspection/locate.rs

> **Crystal Face**: The Position Resolver — Element-to-Location Mapping.

---

## 💎 Facet DNA

$$
\text{locate}(\text{Element}) \to \text{Location}
$$

**locate.rs** defines the **Position Resolver** — mapping elements to their document positions.

---

## Prescriptive Axioms

### Axiom I: Resolution Function

$$
\text{locate}(e) = \text{Location}(e)
$$

Locate **resolves** an element's position in the document.

---

### Axiom II: Label Requirement

$$
\text{locate}(e) \Rightarrow \text{labeled}(e)
$$

Only **labeled** elements can be located.

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE POSITION RESOLVER (locate.rs)               │
├──────────────────────────────────────────────────────────┤
│  Role: Element-to-location mapping                       │
│                                                          │
│  Constraint: Element must be labeled                     │
└──────────────────────────────────────────────────────────┘
```
