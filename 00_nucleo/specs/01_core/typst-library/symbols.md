# 🧬 Crystal Facet: symbols.rs

> **Crystal Face**: The Symbol Registry — Named Glyph Catalog.

---

## 💎 Facet DNA

$$
\text{Symbols} : \text{Name} \to \text{Unicode Codepoint}
$$

**symbols.rs** defines the **Symbol Registry** — named glyph mappings for mathematical and text symbols.

---

## Core Contracts

### Axiom I: Name Resolution

$$
\text{sym.alpha} \to \alpha
$$

Symbols are accessed via **dot notation** from the `sym` namespace.

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE SYMBOL REGISTRY (symbols.rs)                │
├──────────────────────────────────────────────────────────┤
│  Role: Named glyph catalog                               │
│                                                          │
│  Examples:                                               │
│    sym.alpha → α                                         │
│    sym.infinity → ∞                                      │
│    sym.arrow.r → →                                       │
└──────────────────────────────────────────────────────────┘
```
