# 🧬 Crystal Facet: grid/repeated.rs

> **Crystal Face**: The Persistent Geometry — Replication Invariant.

---

## 💎 Facet DNA

$$
\text{Persistent} \xrightarrow{\text{replicate}} \text{per-manifold}
$$

**repeated.rs** implements **persistent geometry sections** — rows that replicate at each regional manifold boundary.

---

## Prescriptive Axioms

### Axiom I: Replication Invariant

$$
\forall r \in \text{Regions}: \text{clone}(\text{Persistent}) \to r_{origin}
$$

**Replication Invariant**: Sections marked as persistent are **cloned at the origin** of each new regional manifold. The cloned geometry is structurally identical.

---

### Axiom II: Persistent Geometry Classification

$$
\text{Persistent} \in \{\text{Head}, \text{Foot}\}
$$

Persistent sections are classified by their **anchoring**:
- **Head-persistent**: Cloned at manifold top
- **Foot-persistent**: Cloned at manifold bottom

---

### Axiom III: Clone Marking

$$
\text{clone}(c).is\_repeated = true
$$

Cloned cells are **marked** to distinguish them from originals (for introspection).

---

## Crystal Linkage

```
┌─────────────────────────────────────────────────────────────────┐
│                    REPLICATION INVARIANT                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   Region 1         Region 2         Region 3                   │
│   ┌─────────┐      ┌─────────┐      ┌─────────┐                 │
│   │ HEAD    │ ═══▶ │ HEAD    │ ═══▶ │ HEAD    │ (cloned)        │
│   ├─────────┤      ├─────────┤      ├─────────┤                 │
│   │ content │      │ content │      │ content │                 │
│   │         │      │ (cont)  │      │ (cont)  │                 │
│   ├─────────┤      ├─────────┤      ├─────────┤                 │
│   │ FOOT    │ ═══▶ │ FOOT    │ ═══▶ │ FOOT    │ (cloned)        │
│   └─────────┘      └─────────┘      └─────────┘                 │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│          THE PERSISTENT GEOMETRY (grid/repeated.rs)      │
├──────────────────────────────────────────────────────────┤
│  Role: Per-manifold replication                          │
│                                                          │
│  Laws:                                                   │
│    ✓ Replication Invariant — clone at each origin        │
│    ✓ Persistent Classification — head/foot anchoring     │
│    ✓ Clone Marking — is_repeated flag                    │
└──────────────────────────────────────────────────────────┘
```
