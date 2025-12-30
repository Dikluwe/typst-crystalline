# 🧬 Crystal Facet: math/lr.rs

> **Crystal Face**: The Reactive Membrane — Elastic Enclosure.

---

## 💎 Facet DNA

$$
\text{enclose} : (L, \text{content}, R) \to \text{Frame}
$$

**lr.rs** implements the **Reactive Membrane** — elastic delimiters that envelope content.

---

## Prescriptive Axioms

### Axiom I: Reactive Membrane Axiom

$$
h_{membrane} = h_{content} + 2\delta_{clearance}
$$

**Reactive Membrane Axiom**: Delimiter geometry is not fixed — it is an **elastic projection** that seeks the minimum envelope containing the total magnitude of the content.

$$
\text{membrane} = \min\{e \mid e \supseteq \text{content} + \delta\}
$$

---

### Axiom II: Symmetric Elasticity

$$
h_L = h_R
$$

Left and right membranes stretch **symmetrically**.

---

### Axiom III: Axis Centering

$$
\text{center}(L) = \text{center}(R) = \text{MathAxis}
$$

Membranes are **centered on the Symmetry Axis**.

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    REACTIVE MEMBRANE                            │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   Content grows:                                                │
│                                                                 │
│   Small:  ( x )     membrane at h₁                              │
│                                                                 │
│   Medium: ⎛ x ⎞     membrane stretches to h₂                    │
│           ⎜ y ⎟                                                  │
│           ⎝   ⎠                                                  │
│                                                                 │
│   Large:  ⎛ x ⎞                                                  │
│           ⎜ y ⎟     membrane assembles for h₃                    │
│           ⎜ z ⎟                                                  │
│           ⎝   ⎠                                                  │
│                                                                 │
│   Membrane reacts to content, seeking minimum envelope          │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│       THE REACTIVE MEMBRANE (math/lr.rs)                 │
├──────────────────────────────────────────────────────────┤
│  Role: Elastic enclosure                                 │
│                                                          │
│  Laws:                                                   │
│    ✓ Reactive Membrane Axiom — elastic envelope          │
│    ✓ Symmetric Elasticity — equal L/R height             │
│    ✓ Axis Centering — centered on Math Axis              │
└──────────────────────────────────────────────────────────┘
```
