# 🧬 Crystal Facet: foundations/scope.rs

> **Crystal Face**: The Visibility Geometry — Nesting Levels and Symbol Resolution.

---

## 💎 Facet DNA

$$
\text{Scope} = \bigcup_{n=0}^{d} \text{Plane}_n
$$

**scope.rs** defines the **Visibility Geometry** — hierarchical nesting levels that govern symbol resolution.

---

## Prescriptive Axioms

### Axiom I: Nesting Levels

$$
\text{Scope} = [\text{Plane}_0, \text{Plane}_1, \ldots, \text{Plane}_d]
$$

Scopes are organized in **nesting levels** — planes of visibility stacked by depth.

---

### Axiom II: Lexical Occlusion Law

$$
\text{symbol}(x, n) \succ \text{symbol}(x, m) \quad \forall m < n
$$

**Lexical Occlusion Law**: A symbol defined at depth $n$ becomes the **authority** over any homonymous symbol at levels $< n$. Inner definitions occlude outer ones.

---

### Axiom III: Resolution Descent

$$
\text{resolve}(x) = \text{Plane}_d[x] \oplus \text{Plane}_{d-1}[x] \oplus \ldots \oplus \text{Plane}_0[x]
$$

Symbol resolution **descends** through nesting levels, returning the first match.

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE VISIBILITY GEOMETRY (scope.rs)              │
├──────────────────────────────────────────────────────────┤
│  Role: Nesting levels and symbol resolution              │
│                                                          │
│  Laws:                                                   │
│    ✓ Nesting Levels — stacked visibility planes          │
│    ✓ Lexical Occlusion — inner shadows outer             │
│    ✓ Resolution Descent — first match wins               │
└──────────────────────────────────────────────────────────┘
```
