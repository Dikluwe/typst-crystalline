# 🧬 Crystal Facet: model/par.rs

> **Crystal Face**: The Paragraph Block — Text Flow Container.

---

## 💎 Facet DNA

$$
\text{Par} = \text{inline content}^* + \text{styling}
$$

**par.rs** defines the **Paragraph Block** — the fundamental text flow container.

---

## Prescriptive Axioms

### Axiom I: Justification Modes

$$
\text{justify} \in \{\text{auto}, \text{true}, \text{false}\}
$$

Paragraphs control **text justification**.

---

### Axiom II: Indentation

$$
\text{first-line-indent} : \text{Length}
$$

First line **indentation** configuration.

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE PARAGRAPH BLOCK (par.rs)                    │
├──────────────────────────────────────────────────────────┤
│  Role: Text flow container                               │
│  Attributes: justify, leading, spacing, indent           │
└──────────────────────────────────────────────────────────┘
```
