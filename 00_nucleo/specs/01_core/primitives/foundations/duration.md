# 🧬 Crystal Facet: foundations/duration.rs

> **Crystal Face**: The Duration Type — Temporal Span.

---

## 💎 Facet DNA

$$
\text{Duration} : \text{time interval}
$$

**duration.rs** defines the **Duration Type** — spans of time.

---

## Method Contracts

| Method | Contract |
|--------|----------|
| `seconds` / `minutes` / `hours` / `days` / `weeks` | Component extraction |

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE DURATION TYPE (duration.rs)                 │
├──────────────────────────────────────────────────────────┤
│  Role: Temporal span                                     │
│  Operations: datetime ± duration                         │
└──────────────────────────────────────────────────────────┘
```
