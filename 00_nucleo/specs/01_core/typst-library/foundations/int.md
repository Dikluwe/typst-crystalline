# 🧬 Crystal Facet: foundations/int.rs

> **Crystal Face**: The Integer Type — Arbitrary Precision Whole Numbers.

---

## 💎 Facet DNA

$$
\text{Int} \in \mathbb{Z}
$$

**int.rs** defines the **Integer Type** — 64-bit signed integers with bigint fallback.

---

## Method Contracts

| Method | Contract |
|--------|----------|
| `signum` | Sign: -1, 0, 1 |
| `bit-not` / `bit-and` / `bit-or` / `bit-xor` | Bitwise ops |
| `bit-lshift` / `bit-rshift` | Bit shifts |

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE INTEGER TYPE (int.rs)                       │
├──────────────────────────────────────────────────────────┤
│  Role: Arbitrary precision integers                      │
│  Range: i64 with bigint fallback                         │
└──────────────────────────────────────────────────────────┘
```
