# 🧬 Crystal Facet: introspection/counter.rs

> **Crystal Face**: The Monotonic Sequence — Document-Wide Counter State.

---

## 💎 Facet DNA

$$
\text{Counter} : \text{Location} \to \mathbb{N}^+
$$

**counter.rs** defines the **Monotonic Sequence** — a document-wide counter that advances through the document.

---

## Prescriptive Axioms

### Axiom I: Monotonic Advance

$$
\text{step}(C) \Rightarrow C_{next} = C + 1
$$

Counters **advance monotonically** through the document.

---

### Axiom II: Level Hierarchy

$$
\text{Counter}(n).\text{step} \Rightarrow \text{reset}(\text{Counter}(n+1))
$$

**Hierarchical Reset**: Stepping a counter at level $n$ resets all counters at higher levels.

---

### Axiom III: Location Binding

$$
\text{Counter.at}(\text{Location}) \to \text{Value}
$$

Counter values are **resolvable** at any document location.

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE MONOTONIC SEQUENCE (counter.rs)             │
├──────────────────────────────────────────────────────────┤
│  Role: Document-wide counter state                       │
│                                                          │
│  Laws:                                                   │
│    ✓ Monotonic Advance — always increases                │
│    ✓ Level Hierarchy — parent reset cascades             │
│    ✓ Location Binding — value at any point               │
└──────────────────────────────────────────────────────────┘
```
