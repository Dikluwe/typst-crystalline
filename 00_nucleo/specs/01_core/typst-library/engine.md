# 🧬 Crystal Facet: engine.rs

> **Crystal Face**: The Compilation Engine — Orchestration Contracts.

---

## 💎 Facet DNA

$$
\text{Engine} = \text{Routines} + \text{World} + \text{Introspector} + \text{Sink}
$$

**engine.rs** defines the **Compilation Engine** — the orchestration context for evaluation and layout.

---

## Core Contracts

### The Engine Bundle

$$
\text{Engine} : \text{compilation context}
$$

The Engine bundles all dependencies needed for compilation:
- **Routines**: Function dispatch table
- **World**: External resource access
- **Introspector**: Query resolution
- **Sink**: Warning/error collection
- **Route**: Cycle detection

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE COMPILATION ENGINE (engine.rs)              │
├──────────────────────────────────────────────────────────┤
│  Role: Orchestration contracts                           │
│                                                          │
│  Components:                                             │
│    ✓ Routines — function dispatch                        │
│    ✓ World — external resources                          │
│    ✓ Introspector — query resolution                     │
│    ✓ Sink — diagnostics collection                       │
└──────────────────────────────────────────────────────────┘
```
