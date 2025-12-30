# 🧬 Crystal Facet: pages/run.rs

> **Crystal Face**: The Receptacle Generator — Parity-Constrained Discretization.

---

## 💎 Facet DNA

$$
\text{generate} : \text{Content} \to \text{Page}^*
$$

**run.rs** generates **page receptacles** from content, respecting parity constraints.

---

## Prescriptive Axioms

### Axiom I: Parity Integrity Axiom

$$
\text{parity}(s) \notin \{\text{odd}, \text{even}, \text{any}\} \land \text{mismatch} \Rightarrow \text{emit}(\text{Empty Receptacle})
$$

**Parity Integrity Axiom**: The manifold requires certain section breaks to occur only at specific parity signatures (odd/even page numbers). To maintain **lateral symmetry**, the system emits empty receptacles when the current page index does not match the required parity.

---

### Axiom II: Receptacle Content Fill

$$
\text{fill}(\text{Page}, \text{Content}_{slice})
$$

Each receptacle is **filled** with a slice of the content flow until saturation or explicit break.

---

### Axiom III: Discretization Continuation

$$
\text{spill}(c) \Rightarrow \text{continue in next receptacle}
$$

Content that spills beyond a receptacle **continues** in the next one.

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    PARITY INTEGRITY                             │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   Section requires odd-page start:                              │
│                                                                 │
│   Page 3 (odd) ← current                                        │
│   Content ends on page 4 (even)                                 │
│   New section wants odd start                                   │
│                                                                 │
│   Solution: Insert empty Page 5                                 │
│             Section starts on Page 6? No!                       │
│             Insert Page 5 (empty), section starts Page 6        │
│                                                                 │
│   Parity mismatch → empty receptacle → symmetry preserved       │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE RECEPTACLE GENERATOR (pages/run.rs)         │
├──────────────────────────────────────────────────────────┤
│  Role: Parity-constrained discretization                 │
│                                                          │
│  Laws:                                                   │
│    ✓ Parity Integrity Axiom — lateral symmetry           │
│    ✓ Receptacle Content Fill — slice filling             │
│    ✓ Discretization Continuation — spill handling        │
└──────────────────────────────────────────────────────────┘
```
