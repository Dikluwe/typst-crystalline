# 🧬 Crystal Facet: typst-ide/complete.rs

> **Crystal Face**: The Autocomplete Engine — Suggestion Generation.

---

## 💎 Facet DNA

$$
\text{complete}(\text{position}) \to [\text{Completion}]
$$

**complete.rs** defines the **Autocomplete Engine** — generating completion suggestions.

---

## Completion Types

| Type | Description |
|------|-------------|
| Keywords | Language keywords |
| Functions | Built-in functions |
| Variables | In-scope variables |
| Fields | Element fields |

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE AUTOCOMPLETE ENGINE (complete.rs)           │
├──────────────────────────────────────────────────────────┤
│  Role: Suggestion generation                             │
│  Input: cursor position                                  │
└──────────────────────────────────────────────────────────┘
```
