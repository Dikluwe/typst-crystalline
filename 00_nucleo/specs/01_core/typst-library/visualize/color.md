# 🧬 Crystal Facet: visualize/color.rs

> **Crystal Face**: The Color Space — Chromatic Value Representation.

---

## 💎 Facet DNA

$$
\text{Color} \in \{\text{RGB}, \text{CMYK}, \text{HSL}, \text{Oklab}, \text{Oklch}, \ldots\}
$$

**color.rs** defines the **Color Space** — color values in multiple representations.

---

## Color Spaces

| Space | Components |
|-------|------------|
| `rgb` | Red, Green, Blue |
| `cmyk` | Cyan, Magenta, Yellow, Black |
| `hsl` | Hue, Saturation, Lightness |
| `hsv` | Hue, Saturation, Value |
| `oklab` | L (lightness), a, b (perceptual) |
| `oklch` | L, Chroma, Hue (perceptual) |
| `luma` | Grayscale |

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE COLOR SPACE (color.rs)                      │
├──────────────────────────────────────────────────────────┤
│  Role: Chromatic value representation                    │
│  Spaces: rgb, cmyk, hsl, hsv, oklab, oklch, luma         │
└──────────────────────────────────────────────────────────┘
```
