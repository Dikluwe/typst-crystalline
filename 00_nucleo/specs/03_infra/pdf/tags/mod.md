# 🧬 Crystal Domain: typst-pdf/tags/

> **Crystal Face**: The Accessibility Tagging System — PDF/UA Structure.

---

## 💎 Domain DNA

$$
\text{tags}(\text{content}) \to \text{structure tree}
$$

**tags/** implements PDF accessibility tagging (PDF/UA) — the structure tree for assistive technology.

---

## Subsystems

| Directory/File | Role |
|----------------|------|
| `mod.rs` | Tag system entry point |
| `groups.rs` | Grouping elements |
| `resolve.rs` | Tag resolution |
| `context/` | Tagging context |
| `tree/` | Structure tree building |
| `util/` | Utility functions |

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE ACCESSIBILITY TAGGING (tags/)               │
├──────────────────────────────────────────────────────────┤
│  Role: PDF/UA structure                                  │
│  Purpose: Screen reader compatibility                    │
└──────────────────────────────────────────────────────────┘
```
