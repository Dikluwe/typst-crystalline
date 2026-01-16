# 🧬 Crystal Facet: toml.rs

> **Crystal Face**: TOML Parser — Config Data Loader.

---

## 💎 Facet DNA

$$
\text{toml} : \mathbb{B}_{bytes} \to \mathbb{D}_{dict}
$$

Parses TOML configuration into dictionary.

---

## Function Signature

```rust
fn toml(source: DataSource) -> Dictionary
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│  Input:  Bytes or Path                                   │
│  Output: Dictionary                                      │
│  Impurity: File I/O via World                            │
└──────────────────────────────────────────────────────────┘
```
