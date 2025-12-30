# 🧬 Crystal Domain: foundations/

> **Crystal Face**: The Type Universe — Foundational Value Contracts.

---

## 💎 Domain DNA

$$
\text{Type} : \text{Identity} + \text{Operations} + \text{Contracts}
$$

**foundations/** defines the **Type Universe** — the foundational value types and their operational contracts.

---

## Type Taxonomy

### Axiom I: Value Universe

$$
\text{Value} \in \{\text{None}, \text{Auto}, \text{Bool}, \text{Int}, \text{Float}, \text{Str}, \text{Bytes}, \ldots\}
$$

All values belong to a **finite type universe** with defined identities.

---

### Axiom II: Content as Semantic Unit

$$
\text{Content} = [\text{Element}^*]
$$

**Content** is an ordered sequence of semantic elements — the fundamental document building block.

---

### Axiom III: Style Inheritance Chain

$$
\text{Style}(e) = \text{chain}(\text{default}, \text{set rules}, \text{local})
$$

Styles are resolved via **inheritance chain** — defaults overridden by set rules, overridden by local settings.

---

## Core Type Contracts

| Type | Contract |
|------|----------|
| `None` | Absence of value |
| `Auto` | Contextual automatic value |
| `Bool` | Binary truth value |
| `Int` | Arbitrary precision integer |
| `Float` | IEEE 754 floating point |
| `Decimal` | Exact decimal arithmetic |
| `Str` | Unicode string |
| `Bytes` | Raw byte sequence |
| `Array` | Ordered heterogeneous collection |
| `Dict` | Key-value mapping |
| `Func` | Callable abstraction |
| `Content` | Document fragment |
| `Symbol` | Named glyph |
| `Version` | Semantic versioning |
| `Datetime` | Temporal instant |
| `Duration` | Temporal span |
| `Label` | Cross-reference anchor |
| `Selector` | Element query pattern |
| `Module` | Namespace container |
| `Plugin` | WASM extension |

---

## Subsystem Files

| File | Role |
|------|------|
| `value.rs` | Value enum definition |
| `content/` | Content and element types |
| `styles.rs` | Style chain contracts |
| `func.rs` | Function contracts |
| `scope.rs` | Namespace contracts |
| `selector.rs` | Query pattern contracts |
| `ops.rs` | Operation contracts |
| `calc.rs` | Mathematical operations |
| `cast.rs` | Type coercion contracts |

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE TYPE UNIVERSE (foundations/)                │
├──────────────────────────────────────────────────────────┤
│  Role: Foundational value contracts                      │
│                                                          │
│  Laws:                                                   │
│    ✓ Value Universe — finite type taxonomy               │
│    ✓ Content as Semantic Unit — element sequence         │
│    ✓ Style Inheritance Chain — layered overrides         │
└──────────────────────────────────────────────────────────┘
```
