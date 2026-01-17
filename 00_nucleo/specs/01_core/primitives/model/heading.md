# 🧬 Crystal Facet: model/heading.rs

> **Crystal Face**: The Hierarchical Section — Document Structure Node.

---

## 💎 Facet DNA

$$
\text{Heading}(\text{level}, \text{body}) \Rightarrow \text{section anchor}
$$

**heading.rs** defines the **Hierarchical Section** — structural nodes that organize document content.

---

## Prescriptive Axioms

### Axiom I: Level Hierarchy

$$
\text{level} \in [1, 6]
$$

Headings have a **depth level** from 1 (topmost) to 6.

---

### Axiom II: Numbering Integration

$$
\text{Heading} \Rightarrow \text{counter}(\text{heading}).\text{step}(\text{level})
$$

Headings integrate with **counters** for numbering.

---

### Axiom III: Outline Participation

$$
\text{outlined} \Rightarrow \text{appears in outline}
$$

Headings can participate in **table of contents**.

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE HIERARCHICAL SECTION (heading.rs)           │
├──────────────────────────────────────────────────────────┤
│  Role: Document structure node                           │
│  Attributes: level, depth, numbering, outlined           │
└──────────────────────────────────────────────────────────┘
```
