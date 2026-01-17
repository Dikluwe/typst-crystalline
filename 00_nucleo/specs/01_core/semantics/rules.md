# 🧬 Crystal Facet: rules.rs

> **Crystal Face**: The Topological Substitution Engine — Show/Set Transformation.

---

## 💎 Facet DNA

$$
\text{rule} : (\text{Selector}, \text{Transformation}) \to \text{Substitution}
$$

**rules.rs** implements the **Topological Substitution Engine** — defining and applying content transformations.

---

## Prescriptive Axioms

### Axiom I: Topological Substitution

$$
\text{show } S: f \Rightarrow \text{Substitution}(S, f)
$$

Show rules define **Topological Substitutions** — content matching selector $S$ is replaced by the result of transformation $f$.

---

### Axiom II: Style Projection

$$
\text{set } T.p = v \Rightarrow \text{StyleProjection}(T.p, v)
$$

Set rules define **Style Projections** — properties of type $T$ are projected to value $v$ within the current scope.

---

### Axiom III: Precedence Authority

$$
\text{depth}(r_1) > \text{depth}(r_2) \Rightarrow r_1 \succ r_2
$$

**Precedence Authority Axiom**: In case of projection collision, the rule defined at **greater depth** (more specific scope) has total authority.

$$
\text{specificity}(r_1) > \text{specificity}(r_2) \Rightarrow r_1 \succ r_2
$$

When depths are equal, **geometric specificity** determines precedence:
- More specific selectors win over less specific
- Later definitions win over earlier (at same specificity)

---

### Axiom IV: Substitution Totality

$$
\text{match}(S, c) \Rightarrow \text{substitute}(c, f(c))
$$

All matching content is **substituted**. No partial application.

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    SUBSTITUTION TOPOLOGY                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   Rule Stack (by depth):                                        │
│                                                                 │
│   Depth 0: [global rules]                                       │
│       │                                                         │
│       └── Depth 1: [document rules]                             │
│               │                                                 │
│               └── Depth 2: [local rules] ← highest authority    │
│                                                                 │
│   Collision: Deeper wins, then more specific wins               │
│                                                                 │
│   Substitution: show selector: transform                        │
│   Projection: set type.prop = value                             │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│       THE TOPOLOGICAL SUBSTITUTION ENGINE (rules.rs)     │
├──────────────────────────────────────────────────────────┤
│  Role: Show/set transformation engine                    │
│                                                          │
│  Laws:                                                   │
│    ✓ Topological Substitution — content replacement      │
│    ✓ Style Projection — property assignment              │
│    ✓ Precedence Authority — depth + specificity wins     │
│    ✓ Substitution Totality — all matches substituted     │
└──────────────────────────────────────────────────────────┘
```
