# 🧬 Crystal Facet: package.rs

> **Crystal Face**: The Namespace Grammar — Identifier Domain Definition.

---

## 💎 Facet DNA

$$
\text{PackageSpec} = \mathcal{N} \times \text{Name} \times \text{Version}
$$

**PackageSpec** is the **Namespace Grammar** — a structured representation of package identifiers adhering to the Identifier Domain.

Format: `@namespace/name:version`

---

## Identifier Domain Definition

$$
\mathcal{D}_{ident} = \{s \in \Sigma^* : s \models \text{IdentifierGrammar}\}
$$

| Component | Domain | Grammar |
|-----------|--------|---------|
| **Namespace** | $\mathcal{N}$ | $[a-z][a-z0-9\text{-}]^*$ |
| **Name** | $\text{Name}$ | $[a-z][a-z0-9\text{-}]^*$ |
| **Version** | $\text{Ver}$ | $\mathbb{N}^+ \cdot \mathbb{N}^+ \cdot \mathbb{N}^+$ |

The domain is **closed under parsing**: strings outside the grammar are rejected.

---

## Prescriptive Axioms

### Axiom I: Parse Totality (within domain)

$$
\forall s \in \mathcal{D}_{spec}: \quad \text{parse}(s) \in \text{PackageSpec}
$$

Every valid specifier string parses to exactly one PackageSpec.

---

### Axiom II: Component Composition

$$
\text{valid}(\text{spec}) \iff \text{ns} \in \mathcal{N} \land \text{name} \in \text{Name} \land \text{ver} \in \text{Ver}
$$

Validity is **compositional**: a spec is valid iff all components belong to their domains.

---

### Axiom III: Display Inversion

$$
\text{parse}(\text{display}(\text{spec})) \equiv \text{spec}
$$

Display and parse are **inverses**. The canonical string representation round-trips perfectly.

---

### Axiom IV: Version Ordering

$$
(\text{Ver}, \leq) : \text{Total Order}
$$

Versions form a **total order**. Comparison is lexicographic over (major, minor, patch).

---

## Facet Table

| Facet | Operation | Signature | Purpose |
|-------|-----------|-----------|---------|
| **Parse** | `from_str` | $\Sigma^* \rightharpoonup \text{Spec}$ | Parse specifier |
| **Project** | `namespace` | $\text{Spec} \to \mathcal{N}$ | Extract namespace |
| **Project** | `name` | $\text{Spec} \to \text{Name}$ | Extract name |
| **Project** | `version` | $\text{Spec} \to \text{Ver}$ | Extract version |
| **Display** | `to_string` | $\text{Spec} \to \Sigma^*$ | Canonical format |
| **Compare** | `cmp` | $(\text{Ver}, \text{Ver}) \to \text{Ord}$ | Version ordering |

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    NAMESPACE CHAIN                              │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   PackageSpec ══component of══▶ FileId                          │
│        │                           │                            │
│        │ (identifier domain)       │                            │
│        ▼                           ▼                            │
│   @namespace/name:version        Canonical Authority            │
│                                    │                            │
│                                    │ anchors                    │
│                                    ▼                            │
│                                  Span                           │
│                                    │                            │
│                                    ▼                            │
│   Source ◀──────────────────── SyntaxNode                       │
│      │                                                          │
│      └──lines──▶ Lines                                          │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Dependencies

| Dependency | Role | Relation |
|------------|------|----------|
| → `FileId` | Composed with VirtualPath | Component |
| → `typst-kit` | Package resolution | Consumer |

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│            THE NAMESPACE GRAMMAR (PackageSpec)           │
├──────────────────────────────────────────────────────────┤
│  Role: Identifier domain for package references          │
│                                                          │
│  Laws:                                                   │
│    ✓ Parse Totality within domain                        │
│    ✓ Component Composition — validity is compositional   │
│    ✓ Display Inversion — parse ∘ display = id            │
│    ✓ Version Ordering — total order on versions          │
│                                                          │
│  Domain:                                                 │
│    • Namespace: [a-z][a-z0-9-]*                          │
│    • Name: [a-z][a-z0-9-]*                               │
│    • Version: N.N.N                                      │
└──────────────────────────────────────────────────────────┘
```
