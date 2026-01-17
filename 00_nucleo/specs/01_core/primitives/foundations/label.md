# 🧬 Crystal Facet: foundations/label.rs

> **Crystal Face**: The Label Type — Cross-Reference Anchor.

---

## 💎 Facet DNA

$$
\text{Label} : \text{unique anchor identifier}
$$

**label.rs** defines the **Label Type** — anchors for cross-references.

---

## Prescriptive Axioms

### Axiom I: Uniqueness Contract

$$
\forall l_1, l_2 \in \text{Doc}: l_1.\text{name} = l_2.\text{name} \Rightarrow l_1 = l_2
$$

Labels should be **unique** within a document for unambiguous resolution.

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE LABEL TYPE (label.rs)                       │
├──────────────────────────────────────────────────────────┤
│  Role: Cross-reference anchor                            │
│  Syntax: <label-name>                                    │
│  Resolution: @label-name                                 │
└──────────────────────────────────────────────────────────┘
```
