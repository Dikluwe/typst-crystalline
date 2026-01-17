# 🧬 Crystal Facet: util.rs

> **Crystal Face**: The Metadata Extractor — Geometric Symbol Primitives.

---

## 💎 Facet DNA

$$
\text{util} : \text{SymbolStream} \to \text{Metadata}_{structured}
$$

**util.rs** provides **Metadata Extraction Primitives** — functions that transform symbolic streams into structured metadata for all transformers.

---

## Prescriptive Axioms

### Axiom I: Symbol Stream Abstraction

$$
\text{Input} = \text{SymbolStream} \quad (\text{not TokenStream})
$$

The input is abstracted as a **Geometric Symbol Stream** — a sequence of symbols with position, not implementation-specific tokens.

---

### Axiom II: Extraction Determinism

$$
\text{extract}(s_1) = \text{extract}(s_2) \iff s_1 \equiv s_2
$$

Extraction is **deterministic**. Identical symbol streams produce identical metadata.

---

### Axiom III: Metadata Structuring

$$
\text{Metadata} = \{\text{flags}, \text{strings}, \text{arrays}, \text{docs}\}
$$

Extracted metadata is **structured** into canonical forms: boolean flags, string values, arrays, and documentation.

---

## Extraction Primitives

| Primitive | Extraction |
|-----------|------------|
| `parse_flag` | Boolean presence |
| `parse_string` | Single string value |
| `parse_string_array` | Array of strings |
| `parse_attr` | General attribute |
| `documentation` | Doc comment text |
| `determine_name_and_title` | Name/title derivation |
| `validate_attrs` | Attribute validation |

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    EXTRACTION CHAIN                             │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   Symbol Stream ══extract══▶ Structured Metadata                │
│       │                                                         │
│       │ consumed by                                             │
│       ▼                                                         │
│   All Transformers (#[func], #[ty], #[elem], #[scope], etc.)    │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE METADATA EXTRACTOR (util.rs)                │
├──────────────────────────────────────────────────────────┤
│  Laws:                                                   │
│    ✓ Symbol Stream Abstraction — geometric symbols       │
│    ✓ Extraction Determinism — identical input = output   │
│    ✓ Metadata Structuring — canonical forms              │
│                                                          │
│  Role: Shared infrastructure for all transformers        │
└──────────────────────────────────────────────────────────┘
```
