# 🧬 Crystal Facet: cast.rs

> **Crystal Face**: The Cast Generator — Value Projection Bridge.

---

## 💎 Facet DNA

$$
\text{\#[cast]} : \text{Type} \to (\text{Reflect}, \text{FromValue}, \text{IntoValue})
$$

**#[cast]** generates the **Cast Triad** — enabling bidirectional value projection between Typst and Rust domains.

---

## Prescriptive Axioms

### Axiom I: Triad Generation

$$
\text{cast!}(T) \Rightarrow (\text{Reflect}(T), \text{FromValue}(T), \text{IntoValue}(T))
$$

The macro generates all three traits as a **unified triad**. They are inseparable.

---

### Axiom II: Canonical Naming Invariant

$$
\text{name}_{typst} = \pi(\text{ident}_{rust})
$$

**Canonical Naming Invariant**: Name projection is **deterministic and unambiguous**. A single projection function $\pi$ maps Rust identifiers to Typst names consistently.

---

### Axiom III: Pattern-Based Conversion

$$
\forall v \in \text{Value}: \quad \text{cast}(v) = \text{match } v \{\ldots\} \text{ or } \bot
$$

Casting uses **pattern matching** with explicit error handling. No implicit coercion.

---

### Axiom IV: AST Projection Sustenance

$$
\text{Cast Triad} \xrightarrow{\text{sustains}} \text{AST Projection}
$$

The Cast Triad **sustains the AST Projection** defined in `typst-syntax`. Without casts, AST nodes cannot be converted to typed values.

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    CAST CHAIN                                   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   Cast Triad ══sustains══▶ AST Projection (typst-syntax)        │
│       │                                                         │
│       │ Reflect: Runtime type awareness                         │
│       │ FromValue: Typst → Rust conversion                      │
│       │ IntoValue: Rust → Typst conversion                      │
│       │                                                         │
│       └──enables──▶ typst-eval (evaluation)                     │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│           THE CAST GENERATOR (#[cast])                   │
├──────────────────────────────────────────────────────────┤
│  Laws:                                                   │
│    ✓ Triad Generation — Reflect + FromValue + IntoValue  │
│    ✓ Canonical Naming Invariant — deterministic π        │
│    ✓ Pattern-Based Conversion — explicit matching        │
│    ✓ AST Projection Sustenance — syntax integration      │
└──────────────────────────────────────────────────────────┘
```
