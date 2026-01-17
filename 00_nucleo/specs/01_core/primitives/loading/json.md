# 🧬 Crystal Facet: json.rs

> **Crystal Face**: JSON Parser — Structured Data Loader.

---

## 💎 Facet DNA

$$
\text{json} : \mathbb{B}_{bytes} \to \mathbb{V}_{value}
$$

Parses JSON into Typst values.

---

## Type Mapping

| JSON | Typst |
|------|-------|
| `null` | `none` |
| `boolean` | `bool` |
| `number` | `int` or `float` |
| `string` | `str` |
| `array` | `array` |
| `object` | `dictionary` |

---

## Function Signature

```rust
fn json(source: DataSource) -> Value
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│  Input:  Bytes or Path                                   │
│  Output: Typst Value (any type)                          │
│  Impurity: File I/O via World                            │
└──────────────────────────────────────────────────────────┘
```
