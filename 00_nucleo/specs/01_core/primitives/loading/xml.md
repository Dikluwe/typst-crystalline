# 🧬 Crystal Facet: xml.rs

> **Crystal Face**: XML Parser — Hierarchical Data Loader.

---

## 💎 Facet DNA

$$
\text{xml} : \mathbb{B}_{bytes} \to \mathbb{A}_{array}
$$

Parses XML into nested array structure.

---

## Function Signature

```rust
fn xml(source: DataSource) -> Array
```

---

## Element Representation

```rust
(
    tag: "element",
    attrs: (key: "value", ...),
    children: [...],
)
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│  Input:  Bytes or Path                                   │
│  Output: Array of element tuples                         │
│  Impurity: File I/O via World                            │
└──────────────────────────────────────────────────────────┘
```
