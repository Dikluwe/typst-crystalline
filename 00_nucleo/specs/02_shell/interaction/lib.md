# 🧬 Crystal Layer: 02_shell/typst-ide/

> **Crystal Face**: The IDE Integration — Editor Services.

---

## 💎 Crate DNA

$$
\text{IDE} : \text{Position} + \text{Document} \to \text{Intelligence}
$$

**typst-ide** provides language intelligence for IDE integration (LSP backend).

---

## Service Contracts

| Service | Role |
|---------|------|
| `complete` | Autocomplete suggestions |
| `definition` | Go-to-definition |
| `tooltip` | Hover information |
| `jump` | Source navigation |
| `analyze` | Code analysis |

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE IDE LAYER (typst-ide/)                      │
├──────────────────────────────────────────────────────────┤
│  Role: Editor services                                   │
│  Protocol: Language Server Protocol (LSP)                │
└──────────────────────────────────────────────────────────┘
```
