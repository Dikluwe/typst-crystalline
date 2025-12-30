# 🧬 Crystal Facet: code.rs

> **Crystal Face**: The Code Evaluator — Serial State Composition Engine.

---

## 💎 Facet DNA

$$
\text{eval}_{code} : \text{Expr}_{code} \to \text{Value}
$$

**code.rs** implements the **Serial State Composition Engine** — evaluating code mode expressions through sequential state transformation.

---

## Prescriptive Axioms

### Axiom I: Law of Serial State Composition

$$
\text{eval}([e_1, \ldots, e_n], \sigma_0) = \sigma_n \text{ where } \sigma_i = \text{eval}(e_i, \sigma_{i-1})
$$

**Law of Serial State Composition**: Code blocks evaluate as a **chain of state transformations**. Each expression consumes the previous state and produces a new one. The final state contains the result.

---

### Axiom II: Block Scope Isolation

$$
\text{enter}(\sigma) \prec \text{eval} \prec \text{exit}(\sigma)
$$

Block evaluation is **scope-isolated**. Entering pushes a scope, exiting pops.

---

### Axiom III: Expression Value

$$
\text{block}_{value} = \text{last}(\text{exprs})
$$

The block's value is the **last expression's value**.

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    SERIAL COMPOSITION CHAIN                     │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   σ₀ ══e₁══▶ σ₁ ══e₂══▶ σ₂ ══...══▶ σₙ                         │
│                                                                 │
│   State transitions:                                            │
│     • Variable bindings                                         │
│     • Flow events                                               │
│     • Side effects (diagnostics)                                │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE CODE EVALUATOR (code.rs)                    │
├──────────────────────────────────────────────────────────┤
│  Role: Serial state composition engine                   │
│                                                          │
│  Laws:                                                   │
│    ✓ Serial State Composition — chained transformation   │
│    ✓ Block Scope Isolation — push/pop on entry/exit      │
│    ✓ Expression Value — last expression result           │
└──────────────────────────────────────────────────────────┘
```
