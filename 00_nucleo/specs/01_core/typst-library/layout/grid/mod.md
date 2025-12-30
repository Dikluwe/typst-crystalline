# 🧬 Crystal Domain: layout/grid/

> **Crystal Face**: The Tabular Geometry — Row/Column Element Definitions.

---

## 💎 Domain DNA

$$
\text{Grid} = \text{Rows} \times \text{Columns} \to \text{Cells}
$$

**grid/** defines **Tabular Geometry** — element definitions for grids and tables.

---

## Element Contracts

| Element | Role |
|---------|------|
| `grid` | General purpose grid |
| `table` | Semantic table |
| `grid.cell` | Cell with span |
| `grid.header` | Header row |
| `grid.footer` | Footer row |

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE TABULAR GEOMETRY (grid/)                    │
├──────────────────────────────────────────────────────────┤
│  Role: Row/column element definitions                    │
│  Elements: grid, table, cell, header, footer             │
└──────────────────────────────────────────────────────────┘
```
