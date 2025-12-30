# 🧬 Crystal Facet: pdf/accessibility.rs

> **Crystal Face**: The Accessibility Layer — PDF/UA Compliance.

---

## 💎 Facet DNA

$$
\text{Accessibility} : \text{semantic tagging for assistive technology}
$$

**accessibility.rs** defines the **Accessibility Layer** — PDF structure tagging for screen readers and accessibility compliance.

---

## Element Contracts

| Element | Role |
|---------|------|
| `pdf.alt` | Alternative text for images |
| `pdf.actual-text` | Actual text for decorative content |
| `pdf.artifact` | Mark as non-content (decorative) |

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE ACCESSIBILITY LAYER (accessibility.rs)      │
├──────────────────────────────────────────────────────────┤
│  Role: PDF/UA compliance                                 │
│  Elements: pdf.alt, pdf.actual-text, pdf.artifact        │
└──────────────────────────────────────────────────────────┘
```
