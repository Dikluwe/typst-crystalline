# 🧬 Crystal Facet: foundations/selector.rs

> **Crystal Face**: The Query Pattern — Element Selection Algebra.

---

## 💎 Facet DNA

$$
\text{Selector} : \text{Pattern} \to \text{Elements}^*
$$

**selector.rs** defines the **Query Pattern** — patterns for selecting elements from the document.

---

## Prescriptive Axioms

### Axiom I: Selector Algebra

$$
\text{Selector} = \text{Element} \mid \text{Label} \mid \text{and}(S_1, S_2) \mid \text{or}(S_1, S_2) \mid \ldots
$$

Selectors form an **algebra** with combinators.

---

### Axiom II: Match Predicate

$$
\text{matches}(e, S) \to \text{Bool}
$$

Selectors define a **match predicate** over elements.

---

### Axiom III: Query Resolution

$$
\text{query}(S, \text{doc}) \to [\text{Element}]
$$

Selectors **resolve** to element sequences via queries.

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE QUERY PATTERN (selector.rs)                 │
├──────────────────────────────────────────────────────────┤
│  Role: Element selection algebra                         │
│                                                          │
│  Laws:                                                   │
│    ✓ Selector Algebra — and, or, before, after           │
│    ✓ Match Predicate — matches(e, S)                     │
│    ✓ Query Resolution — selector → elements              │
└──────────────────────────────────────────────────────────┘
```
