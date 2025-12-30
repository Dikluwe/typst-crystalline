# 🧬 Crystal Domain: typst-pdf/tags/context/

> **Crystal Face**: The Tagging Context — Element-Specific Handlers.

---

## 💎 Domain DNA

$$
\text{context}(\text{element}) \to \text{tag context}
$$

**context/** provides element-specific tagging contexts.

---

## Subsystems

| File | Role |
|------|------|
| `mod.rs` | Context management |
| `figure.rs` | Figure tagging |
| `grid.rs` | Grid tagging |
| `list.rs` | List tagging |
| `table.rs` | Table tagging |
| `outline.rs` | Outline tagging |

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE TAGGING CONTEXT (context/)                  │
├──────────────────────────────────────────────────────────┤
│  Role: Element-specific handlers                         │
└──────────────────────────────────────────────────────────┘
```
