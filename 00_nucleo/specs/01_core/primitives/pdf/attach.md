# 🧬 Crystal Facet: attach.rs

> **Crystal Face**: PDF Attachment — File Embedding.

---

## 💎 Facet DNA

$$
\text{AttachElem} : \mathbb{P}_{path} \times \mathbb{B}_{bytes} \to \text{PDF Attachment}
$$

Attaches files to PDF output for distribution.

---

## Element Fields

| Field | Type | Purpose |
|-------|------|---------|
| `path` | `Derived<str, str>` | File path (required) |
| `data` | `Bytes` | Raw data (auto-loaded if omitted) |
| `relationship` | `Option<Relationship>` | PDF/A-3 relationship |
| `mime_type` | `Option<str>` | MIME type |
| `description` | `Option<str>` | File description |

---

## Relationship Types

| Value | Meaning |
|-------|---------|
| `source` | PDF was created from this file |
| `data` | Used to derive visual content |
| `alternative` | Alternative document representation |
| `supplement` | Additional resources |

---

## Usage

```typst
#pdf.attach(
  "experiment.csv",
  relationship: "supplement",
  mime-type: "text/csv",
  description: "Raw data",
)
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│  Input:  File path or raw bytes                          │
│  Output: Embedded file in PDF                            │
│  Impurity: File I/O via World                            │
│  Note: Ignored for non-PDF exports                       │
│  Note: Not supported for PDF/A-2                         │
└──────────────────────────────────────────────────────────┘
```
