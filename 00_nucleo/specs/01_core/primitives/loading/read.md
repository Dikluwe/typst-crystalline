# 🧬 Crystal Facet: read.rs

> **Crystal Face**: Raw File Reader — Byte/String Loader.

---

## 💎 Facet DNA

$$
\text{read} : \mathbb{P}_{path} \to \mathbb{B}_{bytes} \cup \mathbb{S}_{str}
$$

Reads file contents as bytes or UTF-8 string.

---

## Function Signature

```rust
fn read(
    path: DataSource,
    encoding: Option<Encoding>,  // default: UTF-8
) -> Bytes | Str
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│  Input:  Path or Bytes                                   │
│  Output: Bytes (raw) or Str (decoded)                    │
│  Impurity: File I/O via World                            │
│  Note: Fundamental loader, base for other parsers        │
└──────────────────────────────────────────────────────────┘
```
