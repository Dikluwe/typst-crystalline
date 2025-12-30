# 🧬 Crystal Facet: ty.rs

> **Crystal Face**: The Type Transformer — Native Type Identity Bridge.

---

## 💎 Facet DNA

$$
\text{\#[ty]} : \text{Type}_{rust} \to \text{NativeType}
$$

**#[ty]** transforms a Rust type into a **NativeType** — a Typst-visible type with intrinsic identity, documentation, and optional scope.

---

## Prescriptive Axioms

### Axiom I: Canonical Name Projection

$$
\text{name}_{typst} = \pi_{canon}(\text{ident}_{rust})
$$

**Law of Canonical Name Projection**: The name projection function $\pi_{canon}$ is **deterministic and bijective**. Each Rust identifier maps to exactly one Typst name.

---

### Axiom II: Intrinsic Documentation

$$
\text{docs} \in \text{Type}_{intrinsic}
$$

Documentation is an **Intrinsic Attribute** of the type, not an external annotation. It is preserved through transformation and exposed to Typst's reflection system.

---

### Axiom III: Scope Association

$$
\text{scope} \in \text{Type}_{meta} \Rightarrow \text{NativeScope}_{attached}
$$

Types marked with `scope` receive an **associated scope** containing methods and constants.

---

### Axiom IV: Typed AST Projection Foundation

$$
\text{NativeType} \xrightarrow{\text{grounds}} \text{AST Projection}
$$

NativeType is the **foundation** for typed AST projection. CST nodes project to values through the type system.

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    TYPE CHAIN                                   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   NativeType ══grounds══▶ Typed AST Projection (typst-syntax)   │
│       │                                                         │
│       │ enables                                                 │
│       ▼                                                         │
│   Type reflection, autocompletion, error messages               │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│           THE TYPE TRANSFORMER (#[ty])                   │
├──────────────────────────────────────────────────────────┤
│  Laws:                                                   │
│    ✓ Canonical Name Projection — bijective π             │
│    ✓ Intrinsic Documentation — docs as attribute         │
│    ✓ Scope Association — method/constant container       │
│    ✓ Typed AST Projection Foundation — grounds AST       │
└──────────────────────────────────────────────────────────┘
```
