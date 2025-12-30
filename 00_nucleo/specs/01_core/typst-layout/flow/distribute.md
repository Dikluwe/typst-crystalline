# 🧬 Crystal Facet: flow/distribute.rs

> **Crystal Face**: The Regional Partitioner — Saturation-Based Allocation.

---

## 💎 Facet DNA

$$
\text{distribute} : (\text{Signatures}, \text{Manifold}) \to \text{Allocation}
$$

**distribute.rs** partitions content across regions via **saturation-based allocation**.

---

## Prescriptive Axioms

### Axiom I: Law of Region Saturation

$$
\mu_{accumulated} > \text{capacity}(r) \Rightarrow \text{Break}
$$

**Law of Region Saturation**: Content occupies the current manifold until the **accumulated metric exceeds region capacity**, triggering a continuity break. The system then advances to the next manifold.

$$
\text{Saturate} \to \text{Break} \to \text{Advance} \to \text{Resume}
$$

---

### Axiom II: Anchored Priority

$$
\text{Anchored} \succ \text{Normal}
$$

Anchored elements (floats) have **placement priority** — they claim space before normal flow content is allocated.

---

### Axiom III: Erosion Application

$$
\text{effective\_capacity} = \text{capacity} - \sum \text{erosions}
$$

The effective capacity for normal flow is the region capacity **minus erosions** from floats and footnotes.

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    SATURATION DYNAMICS                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   Region capacity: ████████████████████  (100%)                 │
│                                                                 │
│   Float erosion:   ██████                (30%)                  │
│   Footnote erosion:     ████              (20%)                 │
│                                                                 │
│   Effective zone:      ██████████        (50%)                  │
│                                                                 │
│   Content fills:   ████████              (40% - fits)           │
│   Content fills:   ████████████          (60% - saturates!)     │
│                              ↓                                  │
│                          BREAK → next region                    │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE REGIONAL PARTITIONER (flow/distribute.rs)   │
├──────────────────────────────────────────────────────────┤
│  Role: Saturation-based allocation                       │
│                                                          │
│  Laws:                                                   │
│    ✓ Region Saturation — overflow triggers break         │
│    ✓ Anchored Priority — floats claim first              │
│    ✓ Erosion Application — reduce effective capacity     │
└──────────────────────────────────────────────────────────┘
```
