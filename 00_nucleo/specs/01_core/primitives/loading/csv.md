# 🧬 Crystal Facet: csv.rs

> **Crystal Face**: CSV Parser — Tabular Data Loader.

---

## 💎 Facet DNA

$$
\text{csv} : \mathbb{B}_{bytes} \to \mathbb{A}_{array} \langle \mathbb{D}_{dict} \rangle
$$

Parses CSV data into an array of dictionaries.

---

## Prescriptive Axioms

### Axiom I: Row as Dictionary

$$
\forall \text{row} \in \text{csv}: \quad \text{row} : \text{Dict}\langle\text{header}_i \to \text{value}_i\rangle
$$

Each row maps column headers to cell values.

---

### Axiom II: Delimiter Configuration

$$
\text{delimiter} \in \{\text{comma}, \text{semicolon}, \text{tab}, \text{space}, \text{custom}\}
$$

Configurable field delimiter.

---

## Function Signature

```rust
fn csv(
    source: DataSource,
    delimiter: Option<char>,  // default: ','
    row_type: Option<Type>,   // array or dict
) -> Array
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│  Input:  Bytes or Path                                   │
│  Output: Array of Dicts (default) or Array of Arrays     │
│  Impurity: File I/O via World                            │
└──────────────────────────────────────────────────────────┘
```
