# 🧬 Crystal Layer: 03_infra/typst-html/

> **Crystal Face**: The HTML Exporter — Document-to-HTML Transformation.

---

## 💎 Crate DNA

$$
\text{html}(\text{Document}) \to \text{HTML string}
$$

**typst-html** exports documents to HTML format.

---

## ⚠️ Purity Status

$$
\text{PURE} : \text{Document} \to \text{String}
$$

Pure transformation.

---

## Subsystems

| Module | Role |
|--------|------|
| convert | Document traversal |
| document | HTML document structure |
| dom | DOM construction |
| tag | HTML element creation |
| css | Style generation |
| link | Hyperlink handling |
| encode | HTML encoding |

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE HTML EXPORTER (typst-html/)                 │
├──────────────────────────────────────────────────────────┤
│  Role: Document-to-HTML transformation                   │
│  PURE: No file I/O                                       │
└──────────────────────────────────────────────────────────┘
```
