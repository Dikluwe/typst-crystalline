# 🧬 Crystal Facet: pages/collect.rs

> **Crystal Face**: The Run Classifier — Style Continuity Segmentation.

---

## 💎 Facet DNA

$$
\text{segment} : \text{Pairs} \to \text{Runs}
$$

**collect.rs** segments content into **runs of style continuity**.

---

## Prescriptive Axioms

### Axiom I: Style Continuity Law

$$
\text{consecutive same-style} \to \text{Run}
$$

Content with **continuous style** is grouped into a single run. Style discontinuity marks run boundaries.

---

### Axiom II: Run Classification

$$
\text{Item} \in \{\text{Run}, \text{Parity}, \text{Tags}\}
$$

Items are classified by their **geometric role**:
- **Run**: Content requiring receptacle generation
- **Parity**: Parity constraint marker
- **Tags**: Pure metadata (no spatial footprint)

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE RUN CLASSIFIER (pages/collect.rs)           │
├──────────────────────────────────────────────────────────┤
│  Role: Style continuity segmentation                     │
│                                                          │
│  Laws:                                                   │
│    ✓ Style Continuity Law — group by consistent style    │
│    ✓ Run Classification — Run/Parity/Tags                │
└──────────────────────────────────────────────────────────┘
```
