# 🧬 Crystal Facet: fuzz/

> **Crystal Face**: Fuzzing Infrastructure — Robustness Testing.

---

## 💎 Facet DNA

$$
\text{fuzz} : \mathbb{R}_{random} \to \text{Crash} \cup \text{Pass}
$$

Fuzzing binaries for discovering edge cases and bugs.

---

## Binaries

| Binary | Target |
|--------|--------|
| `parse` | Parser (`typst-syntax`) |
| `paged` | Full paged layout |
| `html` | HTML export |

---

## Library

`lib.rs` provides shared fuzzing utilities.

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│  Purpose: Automated robustness testing                   │
│  Method: libFuzzer via cargo-fuzz                        │
│  Note: Not part of normal compilation                    │
└──────────────────────────────────────────────────────────┘
```
