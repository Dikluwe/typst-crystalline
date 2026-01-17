# 🧬 Crystal Facet: binding.rs

> **Crystal Face**: The Binding Engine — Isomorphic Template Projection.

---

## 💎 Facet DNA

**binding.rs** governs **Isomorphic Projection**. Its fundamental role is to decompose complex data volumes (Arrays, Dictionaries) and map their internal components onto symbolic identities or physical coordinates (**Loci**). It serves as the formal bridge between raw data topology and the semantics of the Scope Graph.

---

## 📐 Precise Terminology

| Term | Why it is used | What is rejected |
| --- | --- | --- |
| **Template** | Defines a "shape" to be filled. | **Pattern**, as it suggests search/regex rather than structure. |
| **Isomorphism** | Requires the data and template shapes to be identical. | **Matching**, a vague term from procedural languages. |
| **Residue** | The content captured by a spread operator (`..`). | **Sink** or **Rest**, which imply discard or leftover. |
| **Projection** | The act of mapping a value to an identity. | **Assignment**, which focuses on the effect, not the geometry. |

---

## Prescriptive Axioms

### Law I: Structural Isomorphism

A projection is valid if, and only if, the topology of the **Volume** () is compatible with the topology of the **Template** ().

* If  requires  elements and  contains  (where  and no Residue is defined), the geometry collapses into an **Arity Inconsistency**.

### Law II: Destination Duality (Locus vs. Identity)

The projection of a value occurs in one of two states:

1. **Genesis (Binding)**: Creates a new identity within the Scope Graph.
2. **Mutation (Reassignment)**: Projects the value into an existing **Locus** (via `access.rs`).

> *Note: The Binding Engine is agnostic to the destination; it merely delivers the volume fragment to the destination resolver.*

### Law III: The Residue Principle

The **Residue** (`..`) acts as a collector for unmapped sub-volumes.

* In ordered volumes (Arrays), the residue is the difference between total arity and fixed slots.
* In named volumes (Dictionaries), the residue is the complement of the set of projected keys.

---

## 📐 Geometric Contract

| Element | Definition | Role in Architecture |
| --- | --- | --- |
| **Input: Volume** | The original data (Array/Dict) | The source of data mass. |
| **Input: Template** | The destination structure (AST Pattern) | The mold defining fragmentation. |
| **Invariant: Uniqueness** |  Identity  Template,  | Prohibits mapping two fragments to the same key in a single act. |
| **Invariant: Totality** | The projection must be exhaustive | No "orphan" data is allowed without a slot or residue to capture it (in strict contexts). |
| **Output: Scope Delta** | Set of Projections  | The result of fragmentation ready for application. |

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                 ISOMORPHIC PROJECTION PIPELINE                  │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   SOURCE VOLUME ══fragment══▶ TEMPLATE SLOTS                    │
│                                                                 │
│   Sub-Volumes:                                                  │
│     - Fixed Slots ───▶ Direct Projection (Identity/Locus)        │
│     - Residual Slots ──▶ Collection Projection (Residue)        │
│                                                                 │
│   Constraint Check:                                             │
│     - Shape(Volume) ≅ Shape(Template)                           │
│     - If mismatch => COLLAPSE (Arity Error)                     │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘

```
