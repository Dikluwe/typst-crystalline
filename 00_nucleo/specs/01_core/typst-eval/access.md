# 🧬 Crystal Facet: access.rs

> **Crystal Face**: The Access Engine — Locus Persistence & Projection.

---

## 💎 Facet DNA

**access.rs** implements the **Mutable Path Resolver**. Its primary function is to determine if a point in the data space can be physically altered or if it exists only as an ephemeral shadow. It maps symbolic navigation to exclusive physical coordinates.

---
**Locus** (Latin: "place") is chosen over alternatives for precision:



| Term | Limitation | Why Rejected |

|------|-----------|--------------|

| Reference | Implies fixed memory address | Incompatible with incremental computation |

| Handle | Suggests resource management | Loci have no lifecycle (open/close) |

| Pointer | Hardware-specific | Not language-agnostic |

| Path | Describes navigation, not destination | Ambiguous (is it the route or the place?) |



**Locus** captures the essence: a **persistent coordinate** in the data space that:

1. Survives physical relocation (memoization, GC)

2. Exists as a topological identity, not a memory address

3. Forms a perfect duality with **Shadow** (ephemeral non-place)

---

### Mathematical Precedent



In geometry, a locus is defined as:

> "The set of all points satisfying a condition"



In Crystalline Architecture:

> "The set of all projection paths reaching a mutable field"



Example:

Locus(user.settings.theme) = { π(user) ∘ π(settings) ∘ π(theme) }

Even if `theme` is physically moved or recalculated, its **locus remains invariant**.

---

## Prescriptive Axioms

### Law I: The Root of Persistence

No projection of mutability can exist without a **Persistence Anchor**.

* **Persistent Root**: An identifier linked to the global or local state (). These possess a physical address.
* **Ephemeral Shadow**: The result of a computation or temporary operation. Shadows lack a permanent address and therefore **prohibit** Locus projection.

### Law II: Geometry of Projection ()

Path resolution is a composite function where each key () projects the current Locus () into a deeper sub-space ().



If the key  does not exist within the volume of , the projection collapses into a **Void Signal** ().

### Law III: Structural Malleability

The Crystal defines the nature of data volumes, determining their interaction with the Access Engine:

* **Malleable**: Volumes that allow the extraction of internal Loci (e.g., Dictionaries, Maps).
* **Rigid**: Volumes that allow reading but forbid the fixation of a mutable Locus (e.g., Symbols, Static Content).

---

## 📐 Geometric Contract

| Element | Definition | Role in Architecture |
| --- | --- | --- |
| **Input: Path** | Ordered sequence  | The navigation vector through the crystal. |
| **Input: Context** | The active Scope Graph () | The source of truth for persistent roots. |
| **Invariant: Uniqueness** |  is unique | Ensures mutation does not leak into adjacent data. |
| **Invariant: Stability** | If  is mutable,  must be mutable | Mutability must be inherited from the root. |
| **Output: Locus** | An exclusive reference (L-Value) | The impact point of the mutation. |
| **Output: Void** | Interruption Signal (Projection Error) | Protection against invalid or rigid access. |

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    LOCUS RESOLUTION PIPELINE                    │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   IDENTIFIER ══resolve══▶ PERSISTENT ROOT (L0)                  │
│                                                                 │
│   L0 ──π(k1)──▶ L1 ──π(k2)──▶ TARGET LOCUS                      │
│                                                                 │
│   Rules:                                                        │
│     - If any Ln is Rigid     => EXIT (Locus Blocked)            │
│     - If any kn is Missing   => EXIT (Void Signal)              │
│     - If Root is Ephemeral   => EXIT (Shadow Error)             │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘

```
