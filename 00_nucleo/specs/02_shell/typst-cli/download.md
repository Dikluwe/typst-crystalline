# 🧬 Crystal Facet: typst-cli/download.rs

> **Crystal Face**: The Download Utility — Package Fetching.

---

## 💎 Facet DNA

$$
\text{download}(\text{url}) \to \text{bytes}
$$

**download.rs** defines the **Download Utility** — fetching packages from network.

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
│          THE DOWNLOAD UTILITY (download.rs)              │
├──────────────────────────────────────────────────────────┤
│  Role: Package fetching                                  │
│  IMPURE: Network I/O                                     │
└──────────────────────────────────────────────────────────┘
```
