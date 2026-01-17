# 🧬 Crystal Layer: 03_infra/typst-render/

> **Crystal Face**: The Raster Renderer — Document-to-Pixels Transformation.

---

## 💎 Crate DNA

$$
\text{render}(\text{Frame}) \to \text{Pixmap}
$$

**typst-render** renders frames to raster images.

---

## ⚠️ Purity Status

$$
\text{PURE} : \text{Frame} \to \text{Pixmap}
$$

Pure transformation.

---

## Subsystems

| Module | Role |
|--------|------|
| text | Text rasterization |
| image | Image rendering |
| paint | Fill/stroke |
| shape | Path rendering |

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE RASTER RENDERER (typst-render/)             │
├──────────────────────────────────────────────────────────┤
│  Role: Document-to-pixels transformation                 │
│  Formats: PNG output                                     │
│  PURE: No file I/O                                       │
└──────────────────────────────────────────────────────────┘
```
