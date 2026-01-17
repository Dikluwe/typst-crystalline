# 🧬 Crystal Facet: foundations/styles.rs

> **Crystal Face**: The Style Chain — Cascading Property Resolution.

---

## 💎 Facet DNA

$$
\text{resolve}(\text{property}) = \text{chain}[\text{default} \to \text{set} \to \text{local}]
$$

**styles.rs** defines the **Style Chain** — cascading resolution of element properties.

---

## Prescriptive Axioms

### Axiom I: Inheritance Chain

$$
\text{Style} = \text{Default} \oplus \text{SetRules} \oplus \text{LocalOverrides}
$$

Styles are resolved via **layered inheritance** — defaults overridden by set rules, overridden by local settings.

---

### Axiom II: Property Targeting

$$
\text{set}(\text{Element}, \text{Property}, \text{Value}) \Rightarrow \text{StyleRule}
$$

Set rules **target** specific element properties.

---

### Axiom III: Scope Locality

$$
\text{Scope}(\text{style}) \Rightarrow \text{applies to descendants}
$$

Styles have **lexical scope** — they apply to content within their definition scope.

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE STYLE CHAIN (styles.rs)                     │
├──────────────────────────────────────────────────────────┤
│  Role: Cascading property resolution                     │
│                                                          │
│  Laws:                                                   │
│    ✓ Inheritance Chain — default → set → local           │
│    ✓ Property Targeting — element.property = value       │
│    ✓ Scope Locality — lexical inheritance                │
└──────────────────────────────────────────────────────────┘
```
