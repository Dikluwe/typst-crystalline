# 🧬 Crystal Facet: diagnostics.md (v2.0 - Refined)

> **Crystal Face**: The Narrative Prism — Separation of Geometric Collapse and Social Narrative.
> **Crystalline Lineage**: @topology L0 | @spec 00_nucleo/specs/diagnostics.md.

---

## 💎 Facet Lattice Basis

The **DiagnosticShield** acts as a late-projection mechanism. It transforms a technical signal of logical collapse (emitted by the **01_core**) into an intelligible narrative (constructed by the **02_shell**). This version enforces **Information Deceleration**: the Core identifies *what* happened, but only the Shell describes it.

---

## 📐 Mathematical Foundation

The transformation of space is defined by the **Visitation Contract**:

1. **Signal Space ()**: Pure data structures in **01_core** that implement the capacity to accept a visitor.
2. **Narrative Space ()**: Visitor implementations in **02_shell** that map signals to social representations (Strings, Hints, UI).
3. **Geography ()**: The `Span` (L0 coordinate) injected only at the layer boundary.

The projection function is defined as:


---

## Prescriptive Axioms

### Law I: Silence of the Core

The **01_core** is strictly prohibited from containing error strings, formatting macros (`eco_format!`), or any UI metadata.

### Law II: Signal Invariance

Signals must be stable data objects containing only the "error body" (e.g., conflicting types), never the "error voice".

### Law III: Law of Explicit Visitation

Communication between Signal and Narrative must occur via **Double Dispatch**.

* **Action**: The Core defines the `NarrativeVisitor` interface; the Shell implements it.

### Law IV: Geographic Isolation

The `Span` (coordinate) is extrinsic to the logical error.

* **Action**: Storing a `Span` inside a `VoidSignal` is prohibited to ensure that moving code does not invalidate the logical error cache.

---

## 📐 Geometric Contract

| Element | Definition | Role in Architecture |
| --- | --- | --- |
| **Input: Signal** | `T: VoidSignal` | Raw collapse data from L1. |
| **Input: Visitor** | `V: NarrativeVisitor` | Translation engine provided by L2. |
| **Input: Geography** | `Span` | L0 coordinate injected by L4. |
| **Invariant: Cache Stability** |  | Signal hash remains constant even if the code moves. |
| **Output: Diagnostic** | `SourceDiagnostic` | Final projected object for the user in L2. |

---

## 🏗️ Topology Linkage

1. **Nucleation (L0)**: Defines the grammar of Spans and this contract of silence.
2. **The Crystal (L1)**: When encountering an invalid state, it emits a `VoidSignal` containing only technical data (e.g., expected vs. actual `TypeId`).
3. **The Adapter (L2)**: The **Shield** implements the `NarrativeVisitor`, deciding how to format raw data for the user's locale.
4. **The Composition (L4)**: The **Wiring** acts as the collision point where Signal (L1), Span (L0), and Shield (L2) meet to generate the final diagnostic.

---
