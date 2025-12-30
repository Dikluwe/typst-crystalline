# 🧬 Crystal Facet: foundations/datetime.rs

> **Crystal Face**: The Datetime Type — Temporal Instant.

---

## 💎 Facet DNA

$$
\text{Datetime} : (\text{year}, \text{month}, \text{day}, \text{hour}, \text{minute}, \text{second})
$$

**datetime.rs** defines the **Datetime Type** — points in time.

---

## Method Contracts

| Method | Contract |
|--------|----------|
| `year` / `month` / `day` | Date components |
| `hour` / `minute` / `second` | Time components |
| `weekday` | Day of week |
| `ordinal` | Day of year |
| `display` | Formatted output |

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE DATETIME TYPE (datetime.rs)                 │
├──────────────────────────────────────────────────────────┤
│  Role: Temporal instant                                  │
│  Access: datetime.today(), datetime(year: ..., ...)      │
└──────────────────────────────────────────────────────────┘
```
