# 🧬 Crystal Domain: introspection/

> **Crystal Face**: The Temporal Query System — State and Location Contracts.

---

## 💎 Domain DNA

$$
\text{Introspection} : \text{Query}(\text{Document}) \to \text{State}
$$

**introspection/** defines the **Temporal Query System** — mechanisms for querying document state across compilation iterations.

---

## Core Concepts

### Axiom I: Convergent Iteration

$$
\text{compile}^n(\text{doc}) \xrightarrow{n \to \infty} \text{stable state}
$$

**Convergent Iteration**: Document compilation iterates until **queries stabilize** — the introspection system converges to a fixed point.

---

### Axiom II: Location Identity

$$
\text{Location} = (\text{Page}, \text{Position})
$$

Every introspectable element has a **location** in the document manifold.

---

### Axiom III: Counter Algebra

$$
\text{Counter}.\text{step}() \Rightarrow \text{Counter} + 1
$$
$$
\text{Counter}.\text{get}(\text{Location}) \to \text{Value}
$$

**Counters** are monotonic sequences resolvable at any location.

---

### Axiom IV: State Mutation

$$
\text{State}.\text{update}(f) \Rightarrow \text{State}' = f(\text{State})
$$

**State** allows functional updates that propagate through the document.

---

## Element Contracts

| Element | Contract |
|---------|----------|
| `Counter` | Monotonic sequence with level hierarchy |
| `State` | Mutable value resolvable at any point |
| `Query` | Element selection across document |
| `Locate` | Position resolution for elements |
| `Here` | Current position accessor |
| `Metadata` | Invisible annotation |
| `Tag` | Position marker for introspection |

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE TEMPORAL QUERY SYSTEM (introspection/)      │
├──────────────────────────────────────────────────────────┤
│  Role: State and location contracts                      │
│                                                          │
│  Laws:                                                   │
│    ✓ Convergent Iteration — stabilize to fixed point     │
│    ✓ Location Identity — page + position                 │
│    ✓ Counter Algebra — monotonic sequences               │
│    ✓ State Mutation — functional updates                 │
└──────────────────────────────────────────────────────────┘
```
