# 🧬 Crystal Facet: flow.rs

> **Crystal Face**: The Interruption Signal Manager — Evaluation Tree Pruner.

---

## 💎 Facet DNA

$$
\text{Signal} \in \{\text{Break}, \text{Continue}, \text{Return}\}
$$

**flow.rs** implements the **Interruption Signal Manager** — handling signals that prune the evaluation tree at designated boundaries.

---

## Prescriptive Axioms

### Axiom I: Signal Non-Value Principle

$$
\text{Signal} \notin \text{Value}
$$

Interruption signals are **not values**. They are meta-instructions that control the evaluation tree traversal, not data to be computed.

---

### Axiom II: Tree Pruning Semantics

$$
\text{emit}(\text{Signal}) \Rightarrow \text{prune}(\text{subtree})
$$

When a signal is emitted, the remaining **evaluation subtree is pruned**. No further expressions in the current scope are evaluated.

| Signal | Prune Target |
|--------|--------------|
| `Break` | Innermost loop subtree |
| `Continue` | Current iteration subtree |
| `Return` | Current function subtree |

---

### Axiom III: Law of Forced Termination

$$
\text{iterations} > \text{MAX} \Rightarrow \text{emit}(\text{Error})
$$

**Law of Forced Termination**: Loops exceeding the iteration bound are **forcefully terminated** with an error. This guarantees system safety.

---

### Axiom IV: Signal Consumption

$$
\text{boundary}(\text{Signal}) \Rightarrow \text{consume}(\text{Signal})
$$

Signals are **consumed** at their designated boundary:
- Break/Continue → consumed by enclosing loop
- Return → consumed by enclosing function

Unconsumed signals propagate to the next boundary.

---

### Axiom V: Forbidden Signal Error

$$
\neg\exists \text{boundary}(\text{Signal}) \Rightarrow \bot
$$

Signals without a valid boundary produce **errors**:
- Break outside loop
- Continue outside loop
- Return outside function

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    TREE PRUNING CHAIN                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   Evaluation Tree:                                              │
│                                                                 │
│       ┌── expr₁                                                 │
│       │                                                         │
│   fn ─┼── expr₂ ← Return emitted here                           │
│       │   │                                                     │
│       │   └── expr₃ ✗ (pruned)                                  │
│       │                                                         │
│       └── expr₄ ✗ (pruned)                                      │
│                                                                 │
│   Signal: Not a value, but a pruning instruction                │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Conditional Structures

| Structure | Signal Handling |
|-----------|-----------------|
| `if/else` | Mark returns as conditional |
| `while` | Consume Break/Continue, bound iterations |
| `for` | Consume Break/Continue, iterate collection |
| `return` | Emit Return signal |

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE INTERRUPTION SIGNAL MANAGER (flow.rs)       │
├──────────────────────────────────────────────────────────┤
│  Role: Evaluation tree pruner                            │
│                                                          │
│  Laws:                                                   │
│    ✓ Signal Non-Value Principle — signals ≠ values       │
│    ✓ Tree Pruning Semantics — subtree elimination        │
│    ✓ Forced Termination — MAX_ITERATIONS safety          │
│    ✓ Signal Consumption — boundary matching              │
│    ✓ Forbidden Signal Error — no boundary = error        │
└──────────────────────────────────────────────────────────┘
```
