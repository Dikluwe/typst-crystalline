# 🧬 Crystal Facet: typst-kit/fonts.rs

> **Crystal Face**: The Font Loader — Font Discovery.

---

## 💎 Facet DNA

$$
\text{fonts}() \to \text{FontBook}
$$

**fonts.rs** discovers and loads fonts.

---

## ⚠️ Purity Violation

$$
\text{fonts} \in \text{IMPURE}
$$

File system access for font loading.

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE FONT LOADER (fonts.rs)                      │
├──────────────────────────────────────────────────────────┤
│  Role: Font discovery                                    │
│  IMPURE: File system access                              │
└──────────────────────────────────────────────────────────┘
```
