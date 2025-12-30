# 🧬 Crystal Facet: introspection/query.rs

> **Crystal Face**: The Element Oracle — Document-Wide Selection.

---

## 💎 Facet DNA

$$
\text{query}(\text{Selector}) \to [\text{Element}]
$$

**query.rs** defines the **Element Oracle** — querying the document for matching elements.

---

## Prescriptive Axioms

### Axiom I: Selector Application

$$
\text{query}(S, \text{doc}) = \{e \in \text{doc} \mid \text{matches}(e, S)\}
$$

Query returns all elements **matching** the selector.

---

### Axiom II: Location Context

$$
\text{query}(S, \text{loc}) = \{e \mid \text{matches}(e, S) \land \text{before}(e, \text{loc})\}
$$

Optional location restricts to elements **before** that point.

---

### Axiom III: Convergence Dependency

$$
\text{query} \Rightarrow \text{requires convergence}
$$

Query results depend on **introspection convergence**.

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE ELEMENT ORACLE (query.rs)                   │
├──────────────────────────────────────────────────────────┤
│  Role: Document-wide selection                           │
│                                                          │
│  Laws:                                                   │
│    ✓ Selector Application — filter by match              │
│    ✓ Location Context — optional position bound          │
│    ✓ Convergence Dependency — requires stable doc        │
└──────────────────────────────────────────────────────────┘
```
