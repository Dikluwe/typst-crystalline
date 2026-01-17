# 🧬 Crystal Facet: typst-kit/download.rs

> **Crystal Face**: The HTTP Client — Network Fetching.

---

## 💎 Facet DNA

$$
\text{download}(\text{url}) \to \text{bytes}
$$

**download.rs** handles HTTP downloads.

---

## ⚠️ Purity Violation

$$
\text{download} \in \text{IMPURE}
$$

Network access.

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE HTTP CLIENT (download.rs)                   │
├──────────────────────────────────────────────────────────┤
│  Role: Network fetching                                  │
│  IMPURE: Network I/O                                     │
└──────────────────────────────────────────────────────────┘
```
