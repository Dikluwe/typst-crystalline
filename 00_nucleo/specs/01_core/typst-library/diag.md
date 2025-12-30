# 🧬 Crystal Facet: diag.rs

> **Crystal Face**: The Diagnostic Contracts — Error and Warning Emission.

---

## 💎 Facet DNA

$$
\text{Diagnostic} = \text{Span} + \text{Severity} + \text{Message}
$$

**diag.rs** defines **Diagnostic Contracts** — structured error and warning emission.

---

## Core Contracts

### Axiom I: Span Localization

$$
\text{Diagnostic} \to \text{Source Location}
$$

Every diagnostic is **localized** to a source span.

---

### Axiom II: Severity Hierarchy

$$
\text{Severity} \in \{\text{Error}, \text{Warning}\}
$$

Diagnostics have **severity levels** affecting compilation flow.

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE DIAGNOSTIC CONTRACTS (diag.rs)              │
├──────────────────────────────────────────────────────────┤
│  Role: Error and warning emission                        │
│                                                          │
│  Laws:                                                   │
│    ✓ Span Localization — source location                 │
│    ✓ Severity Hierarchy — error/warning                  │
└──────────────────────────────────────────────────────────┘
```
