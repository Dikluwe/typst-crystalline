# 🧬 Crystal Facet: foundations/symbol.rs

> **Crystal Face**: The Symbol Type — Named Glyph Identity.

---

## 💎 Facet DNA

$$
\text{Symbol} : \text{name} \to \text{glyph}
$$

**symbol.rs** defines the **Symbol Type** — named glyphs with variant support.

---

## Prescriptive Axioms

### Axiom I: Variant Resolution

$$
\text{sym.arrow.r} \to \text{right arrow variant}
$$

Symbols support **dot-notation variants**.

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE SYMBOL TYPE (symbol.rs)                     │
├──────────────────────────────────────────────────────────┤
│  Role: Named glyph identity                              │
│  Access: sym.name or sym.name.variant                    │
└──────────────────────────────────────────────────────────┘
```
