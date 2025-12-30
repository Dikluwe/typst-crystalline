# 🧬 Crystal Facet: vm.rs

> **Crystal Face**: The Evaluation Manifold — State Geometry Container.

---

## 💎 Facet DNA

$$
\text{Manifold} = (\text{Engine}, \text{NestingGeometry}, \text{Context}, \text{Signal}^?)
$$

**Vm** is the **Evaluation Manifold** — a topological container holding all state geometry required for AST evaluation.

---

## Prescriptive Axioms

### Axiom I: Manifold Composition

$$
\text{Manifold} = \prod \{\text{Engine}, \text{Nesting}, \text{Context}, \text{Signal}\}
$$

The Manifold is a **product** of geometric state components:
- **Engine**: Authority access (World, diagnostics)
- **Nesting**: Lexical nesting geometry
- **Context**: Evaluation context
- **Signal**: Interruption signal (optional)

---

### Axiom II: Lexical Nesting Geometry

$$
\text{Nesting} = [\Sigma_0, \Sigma_1, \ldots, \Sigma_n]
$$

Variable bindings form a **Lexical Nesting Geometry** — a sequence of scopes ordered by nesting depth. Depth 0 is the outermost (global), depth $n$ is the innermost (local).

---

### Axiom III: Visibility Invariant

$$
\forall x: \text{resolve}(x) = \Sigma_{\max\{d \mid x \in \Sigma_d\}}
$$

**Visibility Invariant**: Identities at deeper nesting levels **occlude** identities at shallower levels. Resolution always returns the deepest binding.

$$
x \in \Sigma_3 \land x \in \Sigma_1 \Rightarrow \text{resolve}(x) = \Sigma_3[x]
$$

---

### Axiom IV: Nesting Transitions

$$
\text{enter} : \text{Nesting} \to \text{Nesting} \oplus \Sigma_{new}
$$
$$
\text{exit} : \text{Nesting} \ominus \Sigma_{top} \to \text{Nesting}
$$

Block entry **extends** the geometry; block exit **contracts** it.

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    NESTING GEOMETRY                             │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   Depth 0 (Global): [std, math, ...]                            │
│       │                                                         │
│       └── Depth 1 (Module): [imports, definitions]              │
│               │                                                 │
│               └── Depth 2 (Function): [params, locals]          │
│                       │                                         │
│                       └── Depth 3 (Block): [block locals]       │
│                                                                 │
│   Visibility: Depth 3 > Depth 2 > Depth 1 > Depth 0             │
│   (deeper levels occlude shallower)                             │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE EVALUATION MANIFOLD (vm.rs)                 │
├──────────────────────────────────────────────────────────┤
│  Role: State geometry container                          │
│                                                          │
│  Laws:                                                   │
│    ✓ Manifold Composition — product of components        │
│    ✓ Lexical Nesting Geometry — depth-ordered scopes     │
│    ✓ Visibility Invariant — deep occludes shallow        │
│    ✓ Nesting Transitions — extend/contract geometry      │
└──────────────────────────────────────────────────────────┘
```
