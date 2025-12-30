# 🧬 Crystal Layer: 01_core/typst-syntax/

> **Crystal Face**: The Syntax Layer — Source Code Representation.

---

## 💎 Crate DNA

$$
\text{syntax}(\text{source}) \to \text{AST}
$$

**typst-syntax** provides the lexer, parser, and syntax tree for Typst source code.

---

## ⚠️ Purity Status

$$
\text{PURE} : \text{String} \to \text{SyntaxNode}
$$

Pure transformation — no I/O.

---

## Core Components

| Component | Role |
|-----------|------|
| Lexer | Source → Tokens |
| Parser | Tokens → AST |
| SyntaxNode | Green/Red tree nodes |
| Span | Source locations |

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE SYNTAX LAYER (typst-syntax/)                │
├──────────────────────────────────────────────────────────┤
│  Role: Source code representation                        │
│  PURE: No I/O                                            │
└──────────────────────────────────────────────────────────┘
```
