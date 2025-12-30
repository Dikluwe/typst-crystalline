# 🧬 Crystal Domain: pdf/

> **Crystal Face**: The Export Annotation Contracts — PDF-Specific Metadata.

---

## 💎 Domain DNA

$$
\text{PDF} : \text{Annotations} \to \text{Export Metadata}
$$

**pdf/** defines **Export Annotation Contracts** — PDF-specific metadata and accessibility annotations.

---

## Core Concepts

### Axiom I: Accessibility Tagging

$$
\text{a11y}(\text{content}) \Rightarrow \text{semantic structure}
$$

**Accessibility annotations** provide semantic structure for screen readers.

---

### Axiom II: File Attachment

$$
\text{attach}(\text{file}, \text{doc}) \Rightarrow \text{embedded resource}
$$

Files can be **attached** to the PDF for embedding.

---

## Element Contracts

| Element | Contract |
|---------|----------|
| Accessibility | PDF/UA semantic structure |
| Attachment | Embedded file resources |

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE EXPORT ANNOTATION CONTRACTS (pdf/)          │
├──────────────────────────────────────────────────────────┤
│  Role: PDF-specific metadata                             │
│                                                          │
│  Laws:                                                   │
│    ✓ Accessibility Tagging — a11y semantic structure     │
│    ✓ File Attachment — embedded resources                │
└──────────────────────────────────────────────────────────┘
```
