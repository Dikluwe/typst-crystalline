# 🧬 Crystal Layer: 04_wiring/typst/

> **Crystal Face**: The Facade Crate — Public API.

---

## 💎 Crate DNA

$$
\text{typst} : \text{unified re-export}
$$

**typst** is the facade crate that re-exports all public APIs from the internal crates.

---

## Re-Exports

| Module | Source |
|--------|--------|
| `typst::foundations` | typst-library |
| `typst::layout` | typst-layout |
| `typst::eval` | typst-eval |
| `typst::syntax` | typst-syntax |
| `typst::compile` | Compilation entry point |

---

## World Trait

$$
\text{World} : \text{compilation environment contract}
$$

The `World` trait defines the contract for compilation environments.

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE FACADE CRATE (typst/)                       │
├──────────────────────────────────────────────────────────┤
│  Role: Public API                                        │
│  Layer: 04_wiring (orchestration)                        │
└──────────────────────────────────────────────────────────┘
```
