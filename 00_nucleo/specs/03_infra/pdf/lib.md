# 🧬 Crystal Layer: 03_infra/typst-pdf/

> **Crystal Face**: The PDF Exporter — Document-to-PDF Transformation.

---

## 💎 Crate DNA

$$
\text{pdf}(\text{Document}) \to \text{PDF bytes}
$$

**typst-pdf** exports compiled documents to PDF format.

---

## ⚠️ Purity Status

$$
\text{PURE} : \text{Document} \to \text{Vec}\langle u8 \rangle
$$

Pure transformation — no file I/O in core.

---

## Subsystems

| Module | Role |
|--------|------|
| convert | Document traversal |
| page | Page rendering |
| text | Text output |
| image | Image embedding |
| paint | Fill/stroke rendering |
| shape | Path rendering |
| link | Hyperlink embedding |
| outline | Bookmark generation |
| metadata | PDF metadata |
| tags/ | Accessibility tagging |

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE PDF EXPORTER (typst-pdf/)                   │
├──────────────────────────────────────────────────────────┤
│  Role: Document-to-PDF transformation                    │
│  PURE: No file I/O                                       │
└──────────────────────────────────────────────────────────┘
```
