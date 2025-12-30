# 🧬 Crystal Facet: model/figure.rs

> **Crystal Face**: The Captioned Element — Numbered Content Container.

---

## 💎 Facet DNA

$$
\text{Figure} = \text{body} + \text{caption}^? + \text{numbering}
$$

**figure.rs** defines the **Captioned Element** — numbered containers for images, tables, etc.

---

## Prescriptive Axioms

### Axiom I: Kind Classification

$$
\text{kind} \in \{\text{image}, \text{table}, \text{raw}, \text{custom}\}
$$

Figures are classified by **kind** for separate numbering.

---

### Axiom II: Cross-Reference

$$
\text{Figure} + \text{label} \Rightarrow \text{referenceable}
$$

Labeled figures can be **cross-referenced**.

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE CAPTIONED ELEMENT (figure.rs)               │
├──────────────────────────────────────────────────────────┤
│  Role: Numbered content container                        │
│  Attributes: body, caption, kind, supplement             │
└──────────────────────────────────────────────────────────┘
```
