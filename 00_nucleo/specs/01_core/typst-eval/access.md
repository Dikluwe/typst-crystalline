# 🧬 Crystal Facet: access.rs

> **Crystal Face**: The Access Engine — Value Projection Resolver.

---

## 💎 Facet DNA

$$
\text{access} : (\text{Value}, \text{Key}) \to \text{Value}
$$

**access.rs** implements the **Value Projection Resolver** — projecting sub-values from compound structures.

---

## Prescriptive Axioms

### Axiom I: Field Projection

$$
v.f \xrightarrow{\text{project}} \pi_f(v)
$$

Field access is a **projection** — extracting the component named `f` from the value.

---

### Axiom II: Index Projection

$$
v[i] \xrightarrow{\text{project}} \pi_i(v)
$$

Index access is a **positional projection** — extracting the component at position `i`.

---

### Axiom III: Projection Existence

$$
\neg\exists \pi_k(v) \Rightarrow \bot
$$

Non-existent projections produce **errors**.

---

### Axiom IV: Mutable Access

$$
\text{access\_mut}(v, k) \Rightarrow (\text{get}, \text{set})
$$

Mutable access returns a **get/set pair** for in-place modification.

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    PROJECTION CHAIN                             │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   Value ══project══▶ Sub-Value                                  │
│                                                                 │
│   Projections:                                                  │
│     πf(dict)  = dict[f]      (field)                           │
│     πi(array) = array[i]     (index)                           │
│     πf(struct) = struct.f    (struct field)                    │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE ACCESS ENGINE (access.rs)                   │
├──────────────────────────────────────────────────────────┤
│  Role: Value projection resolver                         │
│                                                          │
│  Laws:                                                   │
│    ✓ Field Projection — named component extraction       │
│    ✓ Index Projection — positional extraction            │
│    ✓ Projection Existence — missing keys error           │
│    ✓ Mutable Access — get/set pair                       │
└──────────────────────────────────────────────────────────┘
```
