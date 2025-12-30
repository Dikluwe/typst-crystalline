# 🧬 Crystal Facet: scope.rs

> **Crystal Face**: The Scope Generator — Membership Space Definition.

---

## 💎 Facet DNA

$$
\text{\#[scope]} : \text{impl}_{rust} \to \text{NativeScope}
$$

**#[scope]** transforms an impl block into a **NativeScope** — a Membership Space that defines what functions, types, and elements belong to a parent.

---

## Prescriptive Axioms

### Axiom I: Membership Set

$$
\text{Scope} = \{m \mid m \in \text{Members}, \text{accessible}(m)\}
$$

A scope is a **Membership Set** — the collection of all accessible members. Membership is determined by the accessibility axiom.

---

### Axiom II: Domain Accessibility

$$
\text{accessible}(m) \iff m \in \text{Domain}_{public}
$$

**Axiom of Domain Accessibility**: An item is accessible if and only if it belongs to the public domain. This is not merely Rust's `pub` keyword, but a declaration of **domain membership**.

---

### Axiom III: Aggregation Closure

$$
\text{Scope}_{parent} = \bigcup_{i \in I} \text{Member}_i
$$

The parent scope is the **closure** over all declared members. No member exists outside the scope it is declared in.

---

### Axiom IV: Global Scope Population

$$
\text{NativeScope}_{global} \xrightarrow{\text{populates}} \text{Evaluator}_{scope}
$$

The global NativeScope **populates** the Evaluator's scope, making all registered functions and types available for evaluation.

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    SCOPE CHAIN                                  │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   #[scope] ══defines══▶ Membership Space                        │
│       │                                                         │
│       │ populates                                               │
│       ▼                                                         │
│   Evaluator Global Scope (typst-eval)                           │
│       │                                                         │
│       │ contains                                                │
│       ▼                                                         │
│   Functions, Types, Elements, Constants                         │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE SCOPE GENERATOR (#[scope])                  │
├──────────────────────────────────────────────────────────┤
│  Laws:                                                   │
│    ✓ Membership Set — scope as member collection         │
│    ✓ Domain Accessibility — public domain membership     │
│    ✓ Aggregation Closure — union of all members          │
│    ✓ Global Scope Population — feeds Evaluator           │
└──────────────────────────────────────────────────────────┘
```
