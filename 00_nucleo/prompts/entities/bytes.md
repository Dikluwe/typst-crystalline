# L0 — Binary byte sequence
Hash do Código: 374a56fc

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/bytes.rs`

## Evidence before specification

Historical upstream `e0e8ca4d` describes bytes at
`crates/typst-library/src/foundations/bytes.rs:16-28` as an iterable sequence
conceptually like integers 0–255, with string/array conversions. Therefore the
earlier claim of opaque language semantics is unsupported; it must not override
that measured definition. Vec storage is implementation mechanics, not parity.
No current-pin measurement is claimed.

## Owned contract

`Bytes(pub Vec<u8>)` derives Debug/Clone/PartialEq/Eq/Hash/Default.
`new(Vec<u8>)` wraps the supplied octets, `len` counts bytes, `is_empty` checks
the vector and `as_slice` returns its byte slice. Preserve From<Vec<u8>> for
Bytes and From<Bytes> for Vec<u8>. Equality/hashing are structural. The public
Rust vector is not itself a language array. Pure L1 performs no I/O or encoding
detection. A change to EcoVec storage is outside the declared representation.

## Delegation and acceptance

Value transport/type naming belongs to `00_nucleo/prompts/entities/value.md`,
language construction and public byte methods to
`00_nucleo/prompts/compiler/stdlib/collections.md`, and repr to
`00_nucleo/prompts/compiler/eval/repr.md`. This owner does not own reading files,
casting language arguments, negative-index diagnostics or repr formatting.
Former cross-owner method/representation clauses are preserved in the diagnostic
for reconciliation, not waived or promoted from current implementation.
Verify construction, byte length, emptiness, slice access, clone, equality,
hash and both vector conversions without changing public contracts/defaults.
