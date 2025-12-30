# 🧬 Crystal Facet: typst-ide/definition.rs

> **Crystal Face**: The Definition Resolver — Go-to-Definition.

---

## 💎 Facet DNA

$$
\text{definition}(\text{position}) \to \text{source location}
$$

**definition.rs** defines the **Definition Resolver** — finding where symbols are defined.

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE DEFINITION RESOLVER (definition.rs)         │
├──────────────────────────────────────────────────────────┤
│  Role: Go-to-definition                                  │
│  Output: file + position                                 │
└──────────────────────────────────────────────────────────┘
```
