# 🧬 Crystal Facet: layout/container.rs

> **Crystal Face**: The Block Container — Content Boundary.

---

## 💎 Facet DNA

$$
\text{Block} = \text{Size}^? + \text{Content}
$$

**container.rs** defines the **Block Container** — a bounded region for content.

---

## Prescriptive Axioms

### Axiom I: Size Specification

$$
\text{Block}(\text{width}^?, \text{height}^?, \text{content})
$$

Containers may have **explicit** or **automatic** dimensions.

---

### Axiom II: Content Clipping

$$
\text{clip} \Rightarrow \text{overflow hidden}
$$

Containers may **clip** content that exceeds bounds.

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE BLOCK CONTAINER (container.rs)              │
├──────────────────────────────────────────────────────────┤
│  Role: Content boundary                                  │
│  Types: block, box                                       │
└──────────────────────────────────────────────────────────┘
```
