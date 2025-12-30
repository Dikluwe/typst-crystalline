# 🧬 Crystal Facet: text/font/variant.rs

> **Crystal Face**: The Font Variant — Weight, Style, Stretch.

---

## 💎 Facet DNA

$$
\text{Variant} = (\text{weight}, \text{style}, \text{stretch})
$$

**variant.rs** defines **Font Variants** — weight (bold), style (italic), and stretch.

---

## Variant Properties

| Property | Values |
|----------|--------|
| **weight** | thin, light, regular, medium, bold, black |
| **style** | normal, italic, oblique |
| **stretch** | condensed, normal, expanded |

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE FONT VARIANT (variant.rs)                   │
├──────────────────────────────────────────────────────────┤
│  Role: Weight, style, stretch                            │
│  Controls: text.weight, text.style, text.stretch         │
└──────────────────────────────────────────────────────────┘
```
