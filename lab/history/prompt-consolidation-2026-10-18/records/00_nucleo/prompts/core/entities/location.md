# L0 — Opaque positional identity
Hash do Código: f9ae72e5

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/location.rs`

## Evidence

The consumer defines the u128 carrier; retained upstream reference is
`lab/typst-original/crates/typst-library/src/introspection/location.rs:59`.
Representation parity is not itself language-parity evidence.

## Contract

`Location(u128)` is opaque, immutable, Debug/Clone/Copy/PartialEq/Eq/Hash.
`pub(crate) fn from_raw(raw: u128) -> Self` wraps the value;
`pub fn as_u128(&self) -> u128` exposes it for serialization/debugging, not
as a public construction escape hatch. Equality compares the exact value;
hash derives from it. Do not interpret it as a page number or ordinal.

The legitimate production generator is Locator; external consumers must not
invent identities. This preserves deterministic generation and avoids competing
identity sources, without claiming mathematical collision-freedom from a hash.
Public new, Display, ordering, arithmetic and a separate LocationKey remain
outside scope. Verify value equality/distinction, hashing, Copy and extraction.
