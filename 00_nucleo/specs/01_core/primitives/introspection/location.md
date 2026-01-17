# 🧬 Crystal Facet: introspection/location.rs

> **Crystal Face**: The Document Coordinate — Manifold Position.

---

## 💎 Facet DNA

$$
\text{Location} = (\text{Page}, \text{Position})
$$

**location.rs** defines the **Document Coordinate** — a point in the document manifold.

---

## Prescriptive Axioms

### Axiom I: Page-Position Pair

$$
\text{Location} \equiv (\text{page index}, (x, y))
$$

Locations are **page-indexed** with 2D coordinates.

---

### Axiom II: Uniqueness

$$
\forall e \in \text{Labeled}: \exists! \text{Location}(e)
$$

Each labeled element has a **unique** location.

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE DOCUMENT COORDINATE (location.rs)           │
├──────────────────────────────────────────────────────────┤
│  Role: Manifold position                                 │
│                                                          │
│  Structure: (page, (x, y))                               │
└──────────────────────────────────────────────────────────┘
```
