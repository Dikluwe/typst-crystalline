# 🧬 Crystal Layer: 03_infra/typst-svg/

> **Crystal Face**: The SVG Exporter — Document-to-SVG Transformation.

---

## 💎 Crate DNA

$$
\text{svg}(\text{Frame}) \to \text{SVG string}
$$

**typst-svg** exports frames to SVG format.

---

## ⚠️ Purity Status

$$
\text{PURE} : \text{Frame} \to \text{String}
$$

Pure transformation.

---

## Subsystems

| Module | Role |
|--------|------|
| text | Text rendering |
| image | Image embedding |
| paint | Fill/stroke |
| shape | Path rendering |

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE SVG EXPORTER (typst-svg/)                   │
├──────────────────────────────────────────────────────────┤
│  Role: Document-to-SVG transformation                    │
│  PURE: No file I/O                                       │
└──────────────────────────────────────────────────────────┘
```
