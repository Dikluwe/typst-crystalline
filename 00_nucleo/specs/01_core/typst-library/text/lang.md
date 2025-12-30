# 🧬 Crystal Facet: text/lang.rs

> **Crystal Face**: The Language Configuration — Locale and Script Binding.

---

## 💎 Facet DNA

$$
\text{Lang} : \text{ISO 639 language code}
$$

**lang.rs** defines **Language Configuration** — locale settings for hyphenation and typography.

---

## Element Contracts

| Element | Role |
|---------|------|
| `text.lang` | Language code (en, de, pt, etc.) |
| `text.region` | Regional variant (US, BR, etc.) |
| `text.script` | Script override (latn, cyrl, etc.) |

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE LANGUAGE CONFIGURATION (lang.rs)            │
├──────────────────────────────────────────────────────────┤
│  Role: Locale and script binding                         │
│  Effects: Hyphenation, quotes, date formatting           │
└──────────────────────────────────────────────────────────┘
```
