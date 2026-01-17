# 🧬 Crystal Facet: pages/finalize.rs

> **Crystal Face**: The Page Finalizer — Index Mapping and Metadata Binding.

---

## 💎 Facet DNA

$$
\text{finalize} : \text{LayoutedPage} \to \text{Page}
$$

**finalize.rs** applies **index-to-symbol mapping** and **locality metadata binding** to complete page construction.

---

## Prescriptive Axioms

### Axiom I: Index-to-Symbol Mapping

$$
\text{number}(p) = \phi(\text{position}(p), \text{counter}_{discretization})
$$

**Index-to-Symbol Mapping**: The numeric identity of a page is a **deterministic function** of:
1. Its position in the manifold sequence
2. The counter state at the discretization point

$$
\text{symbol} = \text{numbering}(\text{counter.advance}())
$$

---

### Axiom II: Locality Metadata Binding

$$
\text{Tag} \xrightarrow{\text{bind}} \text{Frame}
$$

**Locality Metadata Binding**: Tags are not "propagated" — they are **geometric anchors fused** to the frame for future introspection queries. Each tag marks a specific location within the manifold.

---

### Axiom III: Supplement Attachment

$$
\text{Page} = (\text{Frame}, \text{Number}, \text{Supplement})
$$

Each finalized page carries its **frame, number symbol, and supplement** (chapter name, etc.).

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    FINALIZATION PROCESS                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   LayoutedPage                                                  │
│        │                                                        │
│        ├─── Index-to-Symbol Mapping                             │
│        │         position + counter → "Page 42"                 │
│        │                                                        │
│        ├─── Locality Metadata Binding                           │
│        │         tags fused as geometric anchors                │
│        │                                                        │
│        └─── Supplement Attachment                               │
│                  chapter, numbering style                       │
│        │                                                        │
│        ▼                                                        │
│   Finalized Page                                                │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE PAGE FINALIZER (pages/finalize.rs)          │
├──────────────────────────────────────────────────────────┤
│  Role: Index mapping and metadata binding                │
│                                                          │
│  Laws:                                                   │
│    ✓ Index-to-Symbol Mapping — position × counter        │
│    ✓ Locality Metadata Binding — tags as anchors         │
│    ✓ Supplement Attachment — chapter/numbering           │
└──────────────────────────────────────────────────────────┘
```
