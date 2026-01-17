# 🧬 Crystal Facet: mod.rs (image)

> **Crystal Face**: Image Module — Visual Asset Management.

---

## 💎 Facet DNA

$$
\text{image} : \mathbb{P}_{path} \to \mathbb{F}_{frame}
$$

Loads and renders images in documents.

---

## Supported Formats

| Format | Extensions |
|--------|------------|
| Raster | PNG, JPEG, GIF, WebP |
| Vector | SVG |
| Document | PDF (single page) |

---

## Element Fields

| Field | Type | Purpose |
|-------|------|---------|
| `source` | `DataSource` | Path or bytes |
| `format` | `Option<ImageFormat>` | Explicit format |
| `width` | `Option<Length>` | Target width |
| `height` | `Option<Length>` | Target height |
| `alt` | `Option<str>` | Alt text (accessibility) |
| `fit` | `ImageFit` | contain, cover, stretch |

---

## Usage

```typst
#image("diagram.svg", width: 80%)
#image("photo.jpg", alt: "A sunset")
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│  Input:  Image path or bytes                             │
│  Output: Rendered frame                                  │
│  Impurity: File I/O via World                            │
│  Note: Caches decoded images for reuse                   │
└──────────────────────────────────────────────────────────┘
```
