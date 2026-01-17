# 🧬 Crystal Facet: yaml.rs

> **Crystal Face**: YAML Parser — Human-Readable Data Loader.

---

## 💎 Facet DNA

$$
\text{yaml} : \mathbb{B}_{bytes} \to \mathbb{V}_{value}
$$

Parses YAML into Typst values.

---

## Function Signature

```rust
fn yaml(source: DataSource) -> Value
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│  Input:  Bytes or Path                                   │
│  Output: Typst Value                                     │
│  Impurity: File I/O via World                            │
└──────────────────────────────────────────────────────────┘
```
