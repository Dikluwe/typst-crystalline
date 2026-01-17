# 🧬 Crystal Facet: import.rs

> **Crystal Face**: The Import Engine — Module Loading and Resolution.

---

## 💎 Facet DNA

$$
\text{import} : \text{Path} \to \text{Module}
$$

**import.rs** implements **module imports** — loading and evaluating external sources.

---

## Prescriptive Axioms

### Axiom I: Path Resolution

$$
\text{path} \xrightarrow{\text{world}} \text{Source}
$$

Import paths resolve through the **World** interface.

---

### Axiom II: Package Resolution

$$
@\text{pkg/name:version} \xrightarrow{\text{resolve}} \text{Package}
$$

Package specifiers resolve to **versioned packages**.

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE IMPORT ENGINE (import.rs)                   │
├──────────────────────────────────────────────────────────┤
│  Laws: Path resolution, package resolution               │
└──────────────────────────────────────────────────────────┘
```
