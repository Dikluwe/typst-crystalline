# 🧬 Crystal Facet: foundations/plugin.rs

> **Crystal Face**: The Plugin Type — WASM Extension.

---

## 💎 Facet DNA

$$
\text{Plugin} : \text{WebAssembly module}
$$

**plugin.rs** defines the **Plugin Type** — WebAssembly extensions for custom functionality.

---

## Prescriptive Axioms

### Axiom I: Sandbox Contract

$$
\text{Plugin} \Rightarrow \text{no I/O, pure computation}
$$

Plugins are **sandboxed** — no external I/O, only pure computation.

---

### Axiom II: Function Exposure

$$
\text{plugin}(path).function(args) \to \text{bytes}
$$

Plugins expose WASM functions that accept and return **bytes**.

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE PLUGIN TYPE (plugin.rs)                     │
├──────────────────────────────────────────────────────────┤
│  Role: WASM extension                                    │
│  Constraint: Sandboxed (no I/O)                          │
│  I/O: bytes in → bytes out                              │
└──────────────────────────────────────────────────────────┘
```
