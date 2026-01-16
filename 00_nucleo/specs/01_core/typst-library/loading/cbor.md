# 🧬 Crystal Facet: cbor.rs

> **Crystal Face**: CBOR Parser — Binary Data Loader.

---

## 💎 Facet DNA

$$
\text{cbor} : \mathbb{B}_{bytes} \to \mathbb{V}_{value}
$$

Parses CBOR (Concise Binary Object Representation) into Typst values.

---

## Function Signature

```rust
fn cbor(source: DataSource) -> Value
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│  Input:  Bytes or Path                                   │
│  Output: Typst Value                                     │
│  Impurity: File I/O via World                            │
│  Note: Binary format, more compact than JSON             │
└──────────────────────────────────────────────────────────┘
```
