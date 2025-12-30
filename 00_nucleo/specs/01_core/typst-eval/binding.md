# 🧬 Crystal Facet: binding.rs

> **Crystal Face**: The Binding Engine — Pattern Isomorphism Projection.

---

## 💎 Facet DNA

$$
\text{bind} : (\text{Pattern}, \text{Value}) \to \text{Scopes}
$$

**binding.rs** implements the **Pattern Isomorphism Projection** — mapping values onto patterns to produce scope bindings.

---

## Prescriptive Axioms

### Axiom I: Pattern Isomorphism Projection

$$
\text{destructure}(P, V) \xrightarrow{\cong} \{x_i \mapsto v_i\}
$$

Destructuring is a **Pattern Isomorphism Projection** — an isomorphic mapping from value structure to pattern structure. Each pattern component maps to a corresponding value component.

$$
((a, b), c) \cong ((v_1, v_2), v_3) \Rightarrow \{a: v_1, b: v_2, c: v_3\}
$$

---

### Axiom II: Binding Immutability Principle

$$
\text{let } x = v \Rightarrow \text{immutable}(x)
$$

Bindings are **immutable by default**. Mutation requires explicit reassignment syntax.

---

### Axiom III: Pattern Exhaustiveness

$$
\neg\text{matches}(P, V) \Rightarrow \bot
$$

Pattern matching must be **exhaustive**. Failed matches produce errors.

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    PATTERN PROJECTION CHAIN                     │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   Value ══project══▶ Pattern ══bind══▶ Scopes                   │
│                                                                 │
│   Examples:                                                     │
│     let x = v           →  {x: v}                               │
│     let (a, b) = (1, 2) →  {a: 1, b: 2}                         │
│     let (x, _) = (1, 2) →  {x: 1}                               │
│                                                                 │
│   Isomorphism: Pattern shape ≅ Value shape                      │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE BINDING ENGINE (binding.rs)                 │
├──────────────────────────────────────────────────────────┤
│  Role: Pattern isomorphism projection                    │
│                                                          │
│  Laws:                                                   │
│    ✓ Pattern Isomorphism Projection — shape mapping      │
│    ✓ Binding Immutability Principle — immutable default  │
│    ✓ Pattern Exhaustiveness — failed matches error       │
└──────────────────────────────────────────────────────────┘
```
