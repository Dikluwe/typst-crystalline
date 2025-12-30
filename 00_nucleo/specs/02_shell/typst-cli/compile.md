# 🧬 Crystal Facet: typst-cli/compile.rs

> **Crystal Face**: The Compile Command — Document Transformation.

---

## 💎 Facet DNA

$$
\text{compile}(\text{input}) \to \text{output}
$$

**compile.rs** defines the **Compile Command** — transforming Typst source to output formats.

---

## ⚠️ Purity Violation

$$
\text{compile} \in \text{IMPURE}
$$

File system read/write operations.

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE COMPILE COMMAND (compile.rs)                │
├──────────────────────────────────────────────────────────┤
│  Role: Document transformation                           │
│  Outputs: PDF, PNG, SVG                                  │
│  IMPURE: File I/O                                        │
└──────────────────────────────────────────────────────────┘
```
