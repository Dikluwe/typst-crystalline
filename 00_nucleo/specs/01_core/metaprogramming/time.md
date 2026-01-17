# 🧬 Crystal Facet: time.rs

> **Crystal Face**: The Automatic Latency Observer — Transparent Temporal Instrumentation.

---

## 💎 Facet DNA

$$
\text{\#[time]} : \text{fn} \to \text{fn}_{observed}
$$

**#[time]** wraps a function with **transparent temporal instrumentation**, integrating with the `typst-timing` Temporal Observer.

---

## Prescriptive Axioms

### Axiom I: Transparent Observation

$$
\text{result}(\text{\#[time] fn}) \equiv \text{result}(\text{fn})
$$

Observation is **transparent** — it does not alter the function's deterministic result.

---

### Axiom II: Semantic Coordinate Propagation

$$
\text{Span} \in \text{params} \Rightarrow \text{event.anchor} = \text{Span}
$$

If a Span parameter exists, it is **propagated** as a semantic coordinate.

---

### Axiom III: Transparent Cost Invariant

$$
\text{timing.enabled}() = \bot \Rightarrow \text{observable}(\text{overhead}) = \bot
$$

**Transparent Cost Invariant**: When observation is disabled, the overhead is **undetectable** by the system's logic. The observer cannot influence the observed.

---

### Axiom IV: Temporal Non-Interference

$$
\forall f \in \text{Facets}: \quad f(\text{\#[time] g}) \equiv f(g)
$$

The temporal instrumentation satisfies **Non-Interference** — it cannot be detected by any other facet of the system.

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    OBSERVATION CHAIN                            │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   #[time] ══integrates══▶ typst-timing (Temporal Observer)      │
│                                                                 │
│   Transparent Cost:                                             │
│     • Enabled: genesis/completion events recorded               │
│     • Disabled: undetectable overhead                           │
│                                                                 │
│   Non-Interference:                                             │
│     • No facet can detect instrumentation                       │
│     • Results are deterministically identical                   │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│       THE AUTOMATIC LATENCY OBSERVER (#[time])           │
├──────────────────────────────────────────────────────────┤
│  Laws:                                                   │
│    ✓ Transparent Observation — result unchanged          │
│    ✓ Semantic Coordinate Propagation — Span to anchor    │
│    ✓ Transparent Cost Invariant — undetectable overhead  │
│    ✓ Temporal Non-Interference — invisible to facets     │
│                                                          │
│  Integration: Direct link to typst-timing facet          │
└──────────────────────────────────────────────────────────┘
```
