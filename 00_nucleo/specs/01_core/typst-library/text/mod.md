# 🧬 Crystal Domain: text/

> **Crystal Face**: The Typography Contracts — Textual Rendering Configuration.

---

## 💎 Domain DNA

$$
\text{Text} : \text{Style Configuration} \to \text{Glyph Rendering}
$$

**text/** defines **Typography Contracts** — configuration for textual content rendering.

---

## Core Concepts

### Axiom I: Font Resolution Chain

$$
\text{Font} = \text{Family} + \text{Weight} + \text{Style} + \text{Features}
$$

Font selection follows a **resolution chain** from family to specific features.

---

### Axiom II: Text Decoration Stack

$$
\text{Decoration} \in \{\text{Underline}, \text{Overline}, \text{Strikethrough}\}
$$

Decorations are **stackable** visual overlays on text.

---

### Axiom III: Language-Script Binding

$$
\text{Lang} \to \text{Script} \to \text{Shaping Rules}
$$

Language determines script, which determines shaping behavior.

---

## Element Contracts

| Category | Elements |
|----------|----------|
| **Core** | Text, Space, Linebreak |
| **Emphasis** | Strong, Emph, Smallcaps |
| **Decoration** | Underline, Overline, Strike |
| **Position** | Sub, Super |
| **Font** | Font family, weight, style, features |
| **Language** | Lang, Region, Script, Dir |
| **Code** | Raw (code blocks) |
| **Utility** | Lorem, Smartquote |

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE TYPOGRAPHY CONTRACTS (text/)                │
├──────────────────────────────────────────────────────────┤
│  Role: Textual rendering configuration                   │
│                                                          │
│  Laws:                                                   │
│    ✓ Font Resolution Chain — family → features           │
│    ✓ Text Decoration Stack — stackable overlays          │
│    ✓ Language-Script Binding — lang → script → shaping   │
└──────────────────────────────────────────────────────────┘
```
