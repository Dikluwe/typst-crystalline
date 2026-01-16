# 🧬 Crystal Facet: bibliography.rs

> **Crystal Face**: Bibliography System — Academic Reference Management.

---

## 💎 Facet DNA

$$
\text{BibliographyElem} : \mathbb{F}_{files} \to \mathbb{R}_{references}
$$

Manages academic citations and bibliography rendering with CSL support.

---

## Data Geometry

### Core Types

| Type | Purpose |
|------|---------|
| `BibliographyElem` | Element for bibliography rendering |
| `Bibliography` | Loaded bibliography data |
| `CiteElem` | Individual citation |
| `Style` | CSL citation style |

---

## Supported Formats

| Extension | Format |
|-----------|--------|
| `.bib` | BibTeX |
| `.yaml`, `.yml` | Hayagriva YAML |

---

## Prescriptive Axioms

### Axiom I: Single Bibliography

$$
|\text{bibliography}| \leq 1 \text{ per document}
$$

Documents can have at most one bibliography element.

---

### Axiom II: Key Uniqueness

$$
\forall k \in \text{keys}: \quad \text{unique}(k)
$$

Bibliography keys must be unique within the document.

---

## Function Signature

```typst
#bibliography(
  path: str | array,
  title: auto | none | content,
  style: str,
  full: bool,
)
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│  Input:  .bib or .yaml file path(s)                      │
│  Output: Formatted bibliography with citations           │
│  Impurity: File I/O via World                            │
│  Note: Uses hayagriva + citationberg for CSL processing  │
└──────────────────────────────────────────────────────────┘
```
