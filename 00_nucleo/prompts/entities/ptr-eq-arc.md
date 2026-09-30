# L0 — Pointer-identity Arc wrapper
Hash do Código: 941e2bb3

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/ptr_eq_arc.rs`

## Evidence and classification

The consumer uses Arc::ptr_eq for equality and Arc::as_ptr for hashing.
This is a constant-time Rust/resource identity contract, not language content
equality. The prior image-size anecdote and debt-closure chronology are not
acceptance criteria. Arc in domain structs is allowed by ADR-0029.

## Contract

`PtrEqArc<T>(pub Arc<T>)` derives Debug/Clone. PartialEq and Eq compare shared
allocation identity, not dereferenced values; Hash uses that same pointer
identity and stays consistent with equality. Clones share the same Arc and
compare equal; distinct allocations containing equal values compare unequal.
Deref has Target=Arc<T>, exposing &Arc<T> and subsequent dereference to T.
The public field supports direct access. No PartialOrd/Ord is introduced because
pointer ordering is not semantic image ordering. The wrapper is pure L1,
using std::sync::Arc without new external dependencies, I/O or mutable globals.
Image consumers remain responsible for immutable data after creation; the
generic wrapper does not magically enforce that lifecycle or value semantics.
Verify clone identity, distinct-allocation inequality, dereference and equal-hash
consistency. Construction/use by another consumer does not give it this owner.
