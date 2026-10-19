# ADR-0029 — Physical definition of L1 purity — supersedes ADR-0028

**Status:** ACCEPTED
**Supersedes:** ADR-0028 (Simplified Representation of Typographic Types)
**Date:** 2026-03-29

---

## Context

ADR-0028 authorized simplified representations for typographic types in L1 — `Length` as `enum { Pt(f64), Em(f64) }`, `Color` without CMYK/Oklab spaces, and so on — on the grounds that fidelity to the original would be deferred until `StyleChain` existed.

At the same time, the project adopted a more rigorous formal definition of purity: purity means determinism without system I/O, not absence of high-performance data structures. This definition has direct consequences for what is acceptable in L1.

ADR-0028 codified a simplification that the new definition of purity makes incorrect: vanilla Typst’s structures for `Length`, `Abs`, `Rel`, `Angle`, and `Ratio` use `Arc` and internal representations that are completely pure under the physical definition — they perform no system I/O, have no mutable global state, and do not escape RAM. There is no domain reason to simplify them.

## Formal definition of purity and I/O in L1

### What is I/O — strictly prohibited in L1

I/O refers **exclusively** to interaction with the world outside the processor and the current process’s RAM:

| Operation | Examples | Prohibited in L1 |
|-----------|----------|------------------|
| Disk access | `std::fs::read`, `File::open` | ✓ |
| Network access | HTTP, sockets, `std::net` | ✓ |
| System clock | `SystemTime::now()`, `OffsetDateTime::now_utc()` | ✓ |
| Environment variables | `std::env::var`, `std::env::args` | ✓ |
| OS entropy | `rand::thread_rng()` with an OS seed | ✓ |
| Console | `println!`, `eprintln!` in production | ✓ |
| Mutable global state | `static mut T`, `static Mutex<T>`, `static OnceLock<T>` | ✓ |

### What is NOT I/O — permitted and required in L1

| Operation | Examples | Permitted in L1 |
|-----------|----------|-----------------|
| RAM allocation | `Vec::new()`, `String::new()`, `Box::new()` | ✓ |
| Reference counting | `Arc<T>`, `Rc<T>` in struct fields | ✓ |
| Authorized pure utility crates | `ecow`, `indexmap`, `rustc_hash` | ✓ |
| In-memory binary processing | Iterate over an already-loaded `&[u8]` | ✓ |

## Revocation of ADR-0028

ADR-0028 is revoked in its entirety.

### What was wrong in ADR-0028

ADR-0028 justified simplifications on the pretext that fidelity to the original required `StyleChain` (DEBT-1). That was true for **semantic behavior** (mixed sums `Pt + Em` → `Relative`), but not for each type’s **internal data structure**.

In original vanilla Typst, `Length` is a struct with two fields (`abs: Abs`, `em: f64`) — not an enum. This representation is pure under the physical definition. Simplifying it to an enum `{ Pt(f64), Em(f64) }` was not an architectural necessity; it was implementation convenience incorrectly recorded as a domain decision.

### Impact on existing code (Step 25)

Step 25 implemented `Length` as enum `{ Pt(f64), Em(f64) }`. This code **continues to work** and all 341 tests continue to pass. Revoking ADR-0028 does not require an immediate refactor.

What changes is the direction of subsequent steps: when adding new types or revisiting `Length` in the context of Step 30 (`StyleChain`), use the vanilla-faithful representation — not a simplification.

## Decision

### Operating rule for typographic types in L1

When materializing vanilla Typst typographic types in L1, follow the original’s memory architecture without convenience simplifications:

1. **Diagnose first:** verify the real structure in the original before defining the L1 type.
2. **Do not simplify under the pretext of deferral:** if the vanilla type uses `struct Length { abs: Abs, em: f64 }`, use that structure. The fact that mixed sums need `StyleChain` to resolve is an **operations** limitation (`eval_binary_op` returns `Err`), not a **data-structure** limitation.
3. **`Arc` is permitted and required when the type is cloned on a hot path:** `Arc<T>` in struct fields is RAM management, not I/O.
4. **Accept simplifications only with an explicit ADR** documenting:
   - the difference between vanilla and crystalline;
   - the semantic cost of the simplification;
   - the review criterion (not a vague “when `StyleChain` exists,” but a specific step).

### Enumeration of vanilla typographic types covered by this rule

Types identified in the Step 84.8c diagnostic of
`lab/typst-original/crates/typst-library/src/{layout,visualize}/`:

**Dimension / quantity:**

- `Length` (combined structure `abs: Abs + em: f64` in vanilla)
- `Abs` (absolute length component)
- `Em` (component relative to font size)
- `Rel<T>` (combination of `Length + Ratio`)
- `Ratio` (normalized percentage)
- `Fr` (Fraction, `1fr`, `2.5fr`)

**Angle:**

- `Angle`

**Alignment:**

- `Alignment` (tagged enum `H | V | Both` in vanilla)
- `HAlignment` (Start, Left, Center, Right, End)
- `VAlignment` (Top, Horizon, Bottom)

**Direction / axis:**

- `Dir` (LTR, RTL, TTB, BTT)
- `Axis` (Horizontal, Vertical)

**Color / visual:**

- `Color`
- `ColorSpace`
- `Gradient` (Linear / Radial / Conic)
- `Paint` (enum `Color | Gradient | Tiling`)
- `Stroke<T>`
- `Tiling` (Pattern in earlier vanilla versions)

This list covers the typographic types recognized in vanilla Typst at the time of this ADR, whether or not they have been materialized in L1. The crystalline architecture maintains type parity with vanilla — each type appears in L1 with a potentially different form (e.g. `Align2D` in crystalline vs. `Alignment` in vanilla) but identical observable semantics.

Future types added by vanilla Typst follow the same architecture and are added to this list when identified in new materialization ADRs.

**Non-typographic types excluded from this list**, even if they appear in `lab/typst-original/crates/typst-library/src/`:

- Generic containers (`Sides<T>`, `Axes<T>`, `Corners<T>`, `Smart<T>`, `OneOrMultiple<T>`) — structural material reused by multiple types.
- Non-typographic DSL types (`Func`, `Content`, `Module`, `Array`, `Dict`, `Bytes`, `Str`, `Label`, `Symbol`).
- Types with their own ADR (`Datetime` → ADR-0021).
- Ephemeral internal implementation types (`Frame`, `FrameItem`, `Region`, `Fragment`, `Paper`, `Margin`).
- AST elements (`*Elem`: `PlaceElem`, `BoxElem`, etc.) — content containers, not typographic values.
- Specialized internal alignment variants (`OuterHAlignment`, `OuterVAlignment`, `SpecificAlignment<H,V>`, `FixedAlignment`) — indirectly covered by `Alignment`.
- Subcomponents of compound types (`Cmyk`, `WeightedColor`, `RatioComponent`, `GradientStop`, `DashPattern`, `LineCap`, `LineJoin`, `FillRule`, etc.) — internal to `Color`/`Gradient`/`Stroke`.
- Conversion/casting types (`ToAbs`, `ToFloat`, `Num`, `DecNum`, `AngleLike`, `CastInfo`).

### About the Step 25 code

Keep `Length` as enum `{ Pt(f64), Em(f64) }` in Step 25. Revisit it in the Step 30 (`StyleChain`) diagnostic when vanilla’s actual `Length` structure is required to implement `Relative` and mixed sums.

For `Ratio(f64)`, `Angle(f64)`, and `Color { Rgb, Rgba }`, check in the next step’s diagnostic that touches these types whether vanilla’s representation differs materially.

## What remains prohibited (V13 unchanged)

V13 (`MutableStateInCore`) remains in force for mutable global state:

```rust
// PROHIBITED — mutable global state
static INTERNER: Mutex<HashMap<PathBuf, FileId>> = Mutex::new(HashMap::new());
static COUNTER: AtomicU16 = AtomicU16::new(1);
static CACHE: OnceLock<Vec<SyntaxKind>> = OnceLock::new();
```

```rust
// PERMITTED — RAM management in struct fields
pub struct Module(Arc<ModuleInner>);     // O(1) clone in eval()
pub struct SyntaxNode(Arc<NodeData>);    // share the CST without copying
pub struct Func(Arc<FuncRepr>);          // O(1) closure clone
```

## Consequences

**Positive:**

- Removes pressure to make premature simplification decisions based on DEBT that does not yet exist.
- Aligns the AI with the intent to maintain vanilla parity where the memory architecture is pure.
- Removes ADR-0028 as authority for future undocumented simplifications.

**Negative:**

- Diagnosing each vanilla typographic type is required before materializing it — more upfront diagnostic work.
- Step 25’s `Length` enum may not match vanilla — acceptable for now, to be reviewed in Step 30.

**Neutral:**

- Existing code does not need immediate changes.
- V13 does not change; its semantics were already correct.

## References

- ADR-0028 — superseded by this ADR.
- ADR-0018 — precedent: “the criterion is not ‘is it external?’ but ‘does it violate purity?’”
- DEBT-1 — `StyleChain` (Step 30), where vanilla `Length` will be needed.
