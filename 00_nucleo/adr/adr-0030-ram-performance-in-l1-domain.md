# ADR-0030 — Efficient RAM management belongs to L1 — philosophical alignment

**Status:** ACCEPTED
**Date:** 2026-03-29
**Corrects (does not supersede):** ADR-0004 (performance rationale), ADR-0015 (removal of `ecow` from the parser)

---

## Context

ADR-0004 contains this rationale:

> “Compiler performance optimization ... is infrastructure knowledge, not domain knowledge.”

This sentence was written in the context of removing the global `FileId` interner from L1 — a correct decision. But the generalization that followed — “performance optimization belongs in L3” — became a false instruction that guided subsequent decisions in the wrong direction.

ADR-0015 removed `ecow` from the parser based on this generalization. ADR-0007 removed `rustc_hash` for the same reason. Both were later reversed (ADR-0024, ADR-0018).

ADR-0030 states the correct position formally to remove ambiguity before it causes further decision/reversal cycles.

## The correct distinction

What belongs in L3 is not “performance” — it is **system I/O**. The relevant distinction is not “optimized” versus “simple,” but “has side effects in the operating system” versus “operates exclusively on CPU and RAM.”

### Performance choices that belong in L1

These choices are correct and required in L1:

| Choice | Why it belongs in the domain |
|---------|------------------------------|
| `Arc<T>` in types cloned on a hot path | Shared-without-copy semantics are part of the data model |
| `EcoString` in `Value::Str` | O(1) string cloning in `eval()` — this performance is the correct behavior |
| `FxHashMap` in `Scope` | The hasher is a data-type detail, not infrastructure |
| `Arc<[Content]>` in large sequences | Avoiding O(n) copies during layout traversal is a correctness requirement |
| Precomputed `content_hash: u64` in `Source` | Immutability + O(1) hashing — see ADR-0031 |

None of these uses has operating-system side effects. They are high-performance data structures confined to RAM.

### What belongs in L3 (system I/O)

| Operation | Why it belongs in L3 |
|-----------|----------------------|
| Read a file from disk | Side effect on the file system |
| File-system caching | Persistent state outside RAM |
| Memoization with `comemo` | Incremental cache invalidation infrastructure |
| TrueType font parsing | Requires `ttf-parser`, an L3 crate |

## Consequences for existing code

### ADR-0004 — correct code, corrected rationale

The code produced by ADR-0004 (`FileId(NonZeroU16)` without a global interner, `SyntaxText(Arc<str>)`) is correct. It needs no change.

The generalization in its rationale was wrong. The global interner was removed from L1 because it was **mutable global state** (V13) — not because “performance belongs in L3.” `Arc<str>` was added to `SyntaxText` precisely because clone performance is in-domain.

### ADR-0015 — correct parser code, corrected principle

Removing `ecow` from the **parser** in ADR-0015 remains correct: the parser constructs strings once; it does not clone them on a hot path. `String` is sufficient.

The generalization was wrong: “therefore, `ecow` does not belong in L1.” ADR-0024 corrected this for `Value::Str` by distinguishing contexts — the parser constructs; `eval()` clones repeatedly.

## Deep clone vs. `Arc::clone`

The word `clone` in Rust is polysemous. Two distinct semantics share the same method name:

- **`Vec<T>::clone()`** (and clones of structs containing `Vec<T>`, `HashMap<K, V>`, `String`, etc.) — **deep copy of bytes**. O(n) cost in data size. Each clone allocates new memory and duplicates the contents.
- **`Arc<T>::clone()`** (and clones of structs containing only `Arc<T>`, `Rc<T>`, or `EcoString` backed internally by reference counting) — **increment an atomic counter**. O(1) cost regardless of data size. No new memory is allocated; the pointed-to data is shared.

This ADR establishes two rules from this distinction:

1. **Prohibited on L1 hot paths:** `Vec<T>::clone()`, `String::clone()`, `HashMap<K, V>::clone()`, and similar deep clones. If a field has “shared” semantics (multiple owners, immutable after creation), its type must be `Arc<T>`, `Arc<[T]>`, `Arc<str>`, or `EcoString`.
2. **Required in L1 when sharing is semantic:** when a value must be replicated for multiple owners without duplicating bytes, explicitly use `Arc::clone`. Examples in current code: `Source` contains `Arc<str>` (ADR-0031), `Content::Sequence(Arc<[Content]>)` (ADR-0026-R1), and `Arc<[ShowRule]>` in `EvalContext` (Step 84.4 / DEBT-22).

**Detection note:** the difference between deep clone and `Arc::clone` is not visible in the spelling — both are written `x.clone()`. The criterion is the **type of `x`**. Code reviews and materialization steps must check that hot-path fields have types that make cloning O(1), not O(n).

**Relationship to ADR-0026-R1:** choosing `Arc<[Content]>` in `Content::Sequence` is a direct instance of this rule.

**Relationship to DEBT-22** (closed in Step 84.4): converting `Vec<ShowRule>` to `Arc<[ShowRule]>` is a direct instance of this rule.

## Operating rule

When choosing between a simple data structure and a high-performance one in L1, ask:

**“Does this structure have side effects in the operating system?”**

- Yes → it does not belong in L1 (move it to L3).
- No → it belongs in L1 if the domain justifies it.

The wrong question is:

**“Is this structure a performance optimization?”**

Allocation and RAM management performance is not an “optimization” — it is part of a compiler’s correct behavior. A compiler that copies O(n) trees when it could share them through `Arc` is not purer; it is incorrectly slow.

## Strings and arrays

For future reference:

**Strings in L1:** use `EcoString` when a value is cloned on the `eval()` hot path or passed between closures. Use `String` when it is constructed once (parser, error-message construction).

**Arrays in L1:** `Vec<Value>` is acceptable now. When large arrays appear on the `eval()` hot path — documented in DEBT.md — migrate to `Arc<[Value]>` or `EcoVec<Value>`. This migration is a correctness decision, not “premature optimization.”

## References

- ADR-0004 — corrected rationale (code unchanged).
- ADR-0007 — superseded by ADR-0018 (example of the wrong pattern).
- ADR-0015 — corrected principle (parser code unchanged).
- ADR-0018 — precedent: “the criterion is not ‘is it external?’ but ‘does it violate purity?’”
- ADR-0024 — correct distinction: parser vs. `eval()` for `ecow`.
- ADR-0029 — physical definition of purity.
