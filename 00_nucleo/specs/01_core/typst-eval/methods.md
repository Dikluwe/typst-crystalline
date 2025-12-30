# 🧬 Crystal Facet: methods.rs

> **Crystal Face**: The Associated Identity Resolver — Type-Space Binding Engine.

---

## 💎 Facet DNA

$$
\text{resolve} : (\text{Type}, \text{Name}) \rightharpoonup \text{Identity}
$$

**methods.rs** implements the **Associated Identity Resolver** — binding names to identities within type-defined spaces.

---

## Prescriptive Axioms

### Axiom I: Type-Space Binding

$$
\forall T: \exists! \Omega_T \text{ (immutable namespace)}
$$

**Type-Space Binding**: Each type defines an **immutable namespace** $\Omega_T$ that extends its projection capabilities. This space is fixed at type definition and cannot be modified.

---

### Axiom II: Associated Identity Resolution

$$
T.m \xrightarrow{\text{resolve}} \Omega_T[m]
$$

Method resolution queries the type's **associated identity space**. The name must exist in the space for resolution to succeed.

---

### Axiom III: Namespace Immutability

$$
\Omega_T = \text{const}
$$

Type namespaces are **immutable**. No runtime modification is permitted.

---

### Axiom IV: Projection Extension

$$
\Omega_T \supseteq \{\text{intrinsic behaviors}\}
$$

The namespace contains **intrinsic behaviors** that extend what projections the type can perform. Methods are not "added to" types — they are geometric extensions of the type's capability space.

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    TYPE-SPACE GEOMETRY                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   Type T ── defines ──▶ Namespace Ωₜ                            │
│                              │                                  │
│                              ├── method₁                        │
│                              ├── method₂                        │
│                              └── ...                            │
│                                                                 │
│   T.m ══resolve══▶ Ωₜ[m]                                        │
│                                                                 │
│   Immutable: Ωₜ is fixed at type definition                     │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│       THE ASSOCIATED IDENTITY RESOLVER (methods.rs)      │
├──────────────────────────────────────────────────────────┤
│  Role: Type-space binding engine                         │
│                                                          │
│  Laws:                                                   │
│    ✓ Type-Space Binding — immutable namespace per type   │
│    ✓ Associated Identity Resolution — Ωₜ[m] lookup       │
│    ✓ Namespace Immutability — no runtime modification    │
│    ✓ Projection Extension — methods as capabilities      │
└──────────────────────────────────────────────────────────┘
```
