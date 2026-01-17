# 🧬 Crystal Facet: func.rs

> **Crystal Face**: The Function Transformer — Native Function Bridge.

---

## 💎 Facet DNA

$$
\text{\#[func]} : \text{fn}_{rust} \to \text{NativeFunction}
$$

**#[func]** transforms a Rust function into a **NativeFunction** — a Typst-callable function with automatic parameter parsing and value conversion.

---

## Prescriptive Axioms

### Axiom I: Parameter Parsing

$$
\forall p \in \text{params}: \quad p_{typst} \xrightarrow{\text{FromValue}} p_{rust}
$$

All parameters are **automatically parsed** from Typst values to Rust types.

---

### Axiom II: Return Conversion

$$
\text{return}_{rust} \xrightarrow{\text{IntoValue}} \text{return}_{typst}
$$

Return values are **automatically converted** to Typst values.

---

### Axiom III: Environment Authority Injection

$$
\text{Injection} \in \{\text{Engine Authority}, \text{Context Authority}, \text{Arguments}, \text{Location Anchor}\}
$$

**Environment Authority Injections** are special parameters provided by the runtime, not parsed from user input:

- **Engine Authority**: Access to the compilation engine
- **Context Authority**: Access to the evaluation context
- **Arguments**: Raw argument access
- **Location Anchor**: Span of the call site

---

### Axiom IV: Signature Preservation

$$
\text{rustc}(\text{\#[func] fn}) = \text{rustc}(\text{fn})
$$

The original function signature is **preserved** for static analysis.

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    FUNCTION CHAIN                               │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   #[func] ══uses══▶ Cast Triad (FromValue, IntoValue)           │
│                                                                 │
│   Environment Authorities:                                      │
│     • Engine — compilation state                                │
│     • Context — evaluation scope                                │
│     • Location — caller span                                    │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE FUNCTION TRANSFORMER (#[func])              │
├──────────────────────────────────────────────────────────┤
│  Laws:                                                   │
│    ✓ Parameter Parsing — FromValue conversion            │
│    ✓ Return Conversion — IntoValue conversion            │
│    ✓ Environment Authority Injection — runtime context   │
│    ✓ Signature Preservation — static analysis safe       │
└──────────────────────────────────────────────────────────┘
```
