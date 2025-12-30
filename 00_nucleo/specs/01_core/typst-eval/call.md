# 🧬 Crystal Facet: call.rs

> **Crystal Face**: The Invocation Engine — Function Application Engine.

---

## 💎 Facet DNA

$$
\text{call} : (\text{Callee}, \text{Args}) \to \text{Value}
$$

**call.rs** implements the **Invocation Engine** — applying functions to arguments and resolving method invocations.

---

## Prescriptive Axioms

### Axiom I: Evaluation Order

$$
\text{eval}(\text{Callee}) \prec \text{eval}(\text{Args})
$$

The callee is evaluated **before** arguments. Order is deterministic.

---

### Axiom II: Argument Spreading

$$
\text{spread}(a) \Rightarrow \forall v \in a: \text{Args} \gets v
$$

Spread arguments **expand** into the argument list.

---

### Axiom III: Closure Capture

$$
\text{Closure} = (\text{Body}, \text{Captures}, \text{Params})
$$

Closures capture their **lexical environment** at definition time.

---

### Axiom IV: Law of Associated Facet Precedence

$$
\text{method}(T, m) \succ \text{field}(v, m)
$$

**Law of Associated Facet Precedence**: Methods (intrinsic behavior) have **higher precedence** than fields (extrinsic data). When both exist for the same name, the method is invoked.

- **Method**: Intrinsic behavior of the type (from NativeScope)
- **Field**: Extrinsic data stored in the value (from dict/struct)

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    INVOCATION CHAIN                             │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   AST Crystal (Expr) ══eval══▶ Callee Value                     │
│       │                                                         │
│       │ resolve                                                 │
│       ▼                                                         │
│   NativeFunction (from Metaprogramming Forge)                   │
│       │                                                         │
│       │ apply                                                   │
│       ▼                                                         │
│   Result Value                                                  │
│                                                                 │
│   Precedence:                                                   │
│     method (intrinsic) > field (extrinsic)                      │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE INVOCATION ENGINE (call.rs)                 │
├──────────────────────────────────────────────────────────┤
│  Role: Function application engine                       │
│                                                          │
│  Laws:                                                   │
│    ✓ Evaluation Order — callee before args               │
│    ✓ Argument Spreading — expand into list               │
│    ✓ Closure Capture — lexical environment               │
│    ✓ Associated Facet Precedence — method > field        │
└──────────────────────────────────────────────────────────┘
```
