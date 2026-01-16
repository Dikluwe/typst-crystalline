# 🧬 Crystal Facet: raw.rs

> **Crystal Face**: Raw Content Implementation — The Core Data Structure.

---

## 💎 Facet DNA

$$
\text{RawContent} = \text{ptr} \times \text{elem} \times \text{vtable}
$$

Low-level fat pointer implementation for type-erased content with reference counting.

---

## Data Geometry

### Memory Layout

```
RawContent
├── ptr: NonNull<Header>  ──→  Inner<E>
│                              ├── Header
│                              │   ├── refs: AtomicUsize
│                              │   ├── span: Span
│                              │   └── meta: Meta
│                              └── data: E
└── elem: Element (vtable)
```

### Meta Structure

| Field | Type | Purpose |
|-------|------|---------|
| `label` | `Option<Label>` | Document reference target |
| `location` | `Option<Location>` | Introspection location |
| `lifecycle` | `SmallBitSet` | Prepared/synthesized flags |

---

## Prescriptive Axioms

### Axiom I: Reference Counting

$$
\text{refs} = 0 \implies \text{drop}\_\text{impl}()
$$

Automatic deallocation when reference count reaches zero.

---

### Axiom II: Clone-on-Write

$$
\text{refs} > 1 \land \text{mutate} \implies \text{make}\_\text{unique}()
$$

Mutations trigger copy if multiple references exist.

---

### Axiom III: Type Safety via vtable

$$
\forall c \in \text{RawContent}: \quad c.\text{is}\langle T \rangle() \iff c.\text{elem} = \text{Element}\langle T \rangle
$$

Type checks use vtable element identity.

---

## Facet Table

| Facet | Operation | Purpose |
|-------|-----------|---------|
| `new` | Create | Allocate new content |
| `clone_impl` | Clone | Reference count ++ |
| `drop_impl` | Destroy | Deallocate when refs = 0 |
| `data` | Access | Get typed element ref |
| `data_mut` | Mutate | Get mutable ref (COW) |
| `with` | Cast | Trait object access |
| `handle` | Meta | Get vtable handle |

---

## Geometric Contract

```
┌──────────────────────────────────────────────────────────┐
│               RAW CONTENT CRYSTAL                        │
├──────────────────────────────────────────────────────────┤
│  Purpose: Low-level content storage with ref counting    │
│                                                          │
│  Invariants:                                             │
│    ✓ repr(C) for predictable layout                      │
│    ✓ Clone-on-write for efficient sharing                │
│    ✓ vtable enables trait object dispatch                │
│    ✓ Atomic reference counting for thread safety         │
└──────────────────────────────────────────────────────────┘
```
