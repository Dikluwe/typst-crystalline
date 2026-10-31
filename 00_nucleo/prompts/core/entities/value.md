# L0 prompt — runtime values and type identities
Hash do Código: 4a30d7d3

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:d9b2ab50748dec1246d8a55c0b702fa2ee5e23dd737bedf3689e6c9f0297ac29

## Ownership, evidence, and interpretation

This L1 owner specifies `01_core/src/entities/value.rs`: the closed runtime `Value` enum, `Type`, conversions, projections, and the introspected-content carrier. L1 has zero I/O or global mutable state. `Arc` in `Module`/`Func` and the approved shared payloads manages RAM (ADR-0029). `serde` stays in L3 DTOs. Applicable decisions include ADR-0017, ADR-0023/0024/0025/0029, ADR-0107/0108, ADR-0117 clause 4, and ADR-0127/0129/0131. Revoked ADR-0028 does not override ADR-0029.

Historical observations below establish the specified language behavior for their recorded snapshots; they do not certify the current upstream pin without remeasurement. The complete pre-consolidation specification, chronological revisions, original counts, source spellings, and execution-step references remain in `lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/value.md`. Construction receipts remain under `lab/history/construction/records/00_nucleo/materialization/`. Historical “19 variants,” the initial approximately 35 upstream variants, commented future inventories, and the original primitive-to-typographic milestone are historical inventories, not limits on the later approved enum. The 2026-10-16 projection inventory measures 39 materialized variants. No new variant may be added without its ADR and migrated type; historical `_ => Ok(Value::None)` for unimplemented expressions records the deliberate eval migration boundary, not authority to materialize future variants here.

## Measured language identities before the contract

The original type-value probes in `lab/typst-original` showed `type(1)` rendering `int`, `type(1) == int` true, `repr(int)` unquoted `int`, `type(int)` and `type(type)` equal to `type`, `type(rgb)` equal to `function`, and `length != ratio`. Thus type names are first-class values, not strings. The original callable subset was `int`, `float`, `str`, and `type`; `bool(1)` returned `type boolean does not have a constructor`. Later approved subsets below extend that original subset; they do not make every type callable.

The relative-length correction refuted the earlier assertion that a relative value has type `length`: `temp/p842/l1_type_*.typ` measured `type(50%) == ratio`, `type(50% + 0pt) == relative`, and `type(30% + 1em) == relative`. Percentage literals therefore produce `Value::Ratio`; `Ratio + Length` produces `Value::Relative`. The global `relative` binding and `Type::Relative` preserve this distinction. Arithmetic belongs to `00_nucleo/prompts/core/compiler/eval/operators/arithmetic.md`.

For the seven callable type refinements, historical vanilla `a51e02804` returned `repr(type(PATH)) == "type"` where the old product returned `"function"` for `decimal`, `duration`, `regex`, `selector`, `stroke`, `tiling`, and `version`. The catalog and probes in `lab/results/construction/diagnostics/superficie-linguagem-p1140*` record this semantic divergence; after constructor atomization the catalog and 22 probes were byte-identical to the structural baseline. The following label inventory still measured three `WRONG_KIND` entries: `label`, `math.equation`, `math.sqrt`; vanilla `repr(type(label)) == "type"`, but the old product returned `"function"`, and `label("x")` returned `label` versus the product's `content`. These observations support only the label correction below, not a claim that all three findings close here.

## Runtime storage and public interface

`Value::Str` uses `EcoString` under ADR-0024. Unmutated clones are O(1) with copy-on-write for argument passing, captures, and concatenation in eval; `String` would clone O(n). The parser's earlier removal of `ecow` addressed a different, build-once workload. `Value::Dict` uses insertion-ordered `IndexMap<EcoString, Value, FxBuildHasher>` under ADR-0023; `indexmap` and `rustc_hash` are authorized in L1. `Value::Array` remains `Vec<Value>` with O(n) clone and independent mutation; array sharing is separate, unapproved work.

The initial carriers retained by the current contract are `None`, `Bool(bool)`, `Int(i64)`, `Float(f64)`, `Str(EcoString)`, `Array(Vec<Value>)`, `Dict(IndexMap<EcoString, Value, FxBuildHasher>)`, `Module(Module)` (internally `Arc<ModuleInner>`), `Datetime(Datetime)`, `Func(Func)` (internally `Arc<FuncRepr>`), `Content(Content)`, `Auto`, `Length(Length)`, `Relative(Rel<Length>)`, `Ratio(Ratio)`, `Angle(Angle)`, `Color(Color)`, `Tiling(Arc<Tiling>)`, `Label(Label)`, `Dir(Dir)`, and `Type(Type)`. `Dir` requires its migrated L1 entity and preserves `ltr`/`rtl`/`ttb`/`btt` (ADR-0033). Later materialized carriers and payload shapes are governed by their specific clauses below and the 2026-10-16 inventory; the historical list of future `Fraction`, `Gradient`, `Symbol`, `Version`, `Bytes`, `Decimal`, `Duration`, `Styles`, `Args`, and `Dyn` does not revoke later approvals or authorize the remaining items.

`Value` derives `Debug`, `Clone`, and mechanical `PartialEq`. `Value::Int(1) != Value::Float(1.0)` under Rust equality. Language `Eq`/`Neq` explicitly handles mixed numerics with `i as f64` under ADR-0025; Rust equality for data structures and tests must not replace language equality. The latter belongs to `00_nucleo/prompts/core/compiler/eval/operators/equality.md`.

The public projections remain:

```rust
impl Value {
    pub fn type_name(&self) -> &'static str;
    pub fn type_of(&self) -> Type;
    pub fn is_none(&self) -> bool;
    pub fn truthy(&self) -> bool;
    pub fn cast_bool(&self) -> Option<bool>;
    pub fn cast_int(&self) -> Option<i64>;
    pub fn cast_float(&self) -> Option<f64>;
    pub fn cast_str(&self) -> Option<&str>;
    pub fn cast_array(&self) -> Option<&[Value]>;
    pub fn cast_dict(&self) -> Option<&IndexMap<EcoString, Value, FxBuildHasher>>;
}
```

`cast_float` accepts `Int` by implicit coercion; the other listed casts retain their strict categories. Later existing `cast_align`, `cast_bytes`, `cast_decimal`, `cast_duration`, `cast_version`, and `cast_regex` retain the exact behavior specified in the projection-maintenance clause. Collection instance methods, such as `(1, 2, 3).first()` and `"ab".repeat(3)`, are dispatched by `try_dispatch_collection_method` from `eval_func_call`; this entity does not own that dispatch. Dict literal `(a: 1, b: 2)` evaluation is materialized under its eval owner.

Preserve `From<bool>`, `From<i64>`, `From<i32>`, `From<f64>`, `From<EcoString>`, `From<&str>`, `From<String>`, `From<Vec<Value>>`, `From<IndexMap<EcoString, Value, FxBuildHasher>>`, `From<Module>`, `From<Datetime>`, `From<Func>`, `From<Content>`, `From<Length>`, `From<Rel<Length>>`, `From<Ratio>`, `From<Angle>`, `From<Color>`, `From<Tiling>`, and `From<Type>` for `Value`. The approved `From<Stroke>` behavior below is additive to these obligations.

Every materialized variant has recognizable `repr_value` output under `00_nucleo/prompts/core/compiler/eval/repr.md`; round-trip is not promised. Opaque `Location`, `Gradient`, and `Tiling` use `location(...)`, `gradient(...)`, and `tiling(...)`. `Value::Type(t)` repr is its bare type name, such as `int`, `length`, or `type`. The historical statement that Symbol had not migrated is superseded by the later Symbol contract, which requires its own repr mapping.

## First-class types and bounded callable subsets

`Value::Type(Type)` is a first-class type value. `type(x)` returns `Value::Type(x.type_of())`; the applicable global names bind to those values. `Type` is a payload-free `Debug + Clone + Copy + PartialEq + Eq + Hash` enum. `Type::name(&self) -> &'static str` and the corresponding `Value::type_name` preserve the public names below. Type-value equality uses the existing structural `PartialEq` without an eval special case. This mapping includes the existing materialized categories, not permission to add a future carrier:

| Value category | Type | Public type name |
|---|---|---|
| `None` | `None` | `"none"` |
| `Auto` | `Auto` | `"auto"` |
| `Bool` | `Bool` | `"bool"` |
| `Int` | `Int` | `"int"` |
| `Float` | `Float` | `"float"` |
| `Str` | `Str` | `"str"` |
| `Array` | `Array` | `"array"` |
| `Dict` | `Dictionary` | `"dictionary"` |
| `Module` | `Module` | `"module"` |
| `Datetime` | `Datetime` | `"datetime"` |
| `Func` | `Function` | `"function"` |
| `Content`, `LocatedContent` | `Content` | `"content"` |
| `Length` | `Length` | `"length"` |
| `Relative` | `Relative` | `"relative"` |
| `Ratio` | `Ratio` | `"ratio"` |
| `Angle` | `Angle` | `"angle"` |
| `Color` | `Color` | `"color"` |
| `Stroke` | `Stroke` | `"stroke"` |
| `Fraction` | `Fraction` | `"fraction"` |
| `Align` | `Alignment` | `"alignment"` |
| `Location` | `Location` | `"location"` |
| `Gradient` | `Gradient` | `"gradient"` |
| `Regex` | `Regex` | `"regex"` |
| `Tiling` | `Tiling` | `"tiling"` |
| `Bytes` | `Bytes` | `"bytes"` |
| `Decimal` | `Decimal` | `"decimal"` |
| `Duration` | `Duration` | `"duration"` |
| `Version` | `Version` | `"version"` |
| `Selector` | `Selector` | `"selector"` |
| `Symbol` | `Symbol` | `"symbol"` |
| `Args` | `Arguments` | `"arguments"` |
| `State` | `State` | `"state"` |
| `Counter` | `Counter` | `"counter"` |
| `Label` | `Label` | `"label"` |
| `Dir` | `Direction` | `"direction"` |
| `Type(_)` | `Type` | `"type"` |
| `SpotColorant` | `SpotColorant` | `"spot"` |

The original `Int`/`Float`/`Str`/`Type` callable subset invokes `native_int`/`native_float`/`native_str`/`native_type`. Add the existing `Type::{Decimal, Duration, Regex, Selector, Stroke, Tiling, Version}` and `Type::Label` to `Type::is_callable()` exactly as approved, with no new enum variant for those additions. Existing callable types remain callable; other types retain `"type {name} does not have a constructor"`. Dispatch remains a closed static `match`: no vtable, `dyn`, registry, or function payload in `Type`. Existing fields use explicit field-access arms.

Required identities and results are `repr(type(decimal)) == "type"` with `decimal("1.2")` producing Decimal; similarly `duration(seconds: 1)` → Duration, `regex("a+")` → Regex, `selector(heading)` → Selector, `stroke()` → Stroke, `tiling(...)` → Tiling, `version(1, 2)` → Version. `label` binds `Value::Type(Type::Label)` and `label("x")` produces `Value::Label`. `Content::Label` remains a distinct `content` value. These changes supersede only the old function-binding debt for those eight names. The seven-type package excluded `label`, `state`, `counter`, `color`, and `gradient`; the subsequent label package removes only label from that remainder. The original namespace-reconciliation debt for `color`, `gradient`, `state`, and `counter` remains bounded by their later canonical owners; this file does not silently convert them or promise `type(x) == color`.

`Type::Int` exposes `.min`/`.max` as `i64::MIN`/`MAX`, plus `signum`, the six `bit-*` operations, and `from-bytes`/`to-bytes` through the stdlib owner's closed table. `Type::Str` exposes `.from-unicode` as its native function. Preserve `int.min`, `int.max`, and `str.from-unicode` previously carried through native function namespaces. No new Int wrapper is introduced. `Type::Counter` exposes unbound `get`, `display`, `at`, `final`, `step`, `update`, taking `Value::Counter` first; Counter/CounterKey are unchanged. `Type::Content` exposes unbound `func`, `has`, `at`, `fields`, `location`, taking content first; inline content has no location and declarative Content metadata is unchanged. Implementations belong to their corresponding `00_nucleo/prompts/core/compiler/stdlib/` and `00_nucleo/prompts/core/compiler/eval/bindings/field_access.md` owners.

## Located content, frozen field snapshots, and approval boundaries

Historical vanilla `a51e02804` measured two morphologically identical headings from `query(heading)` with `type == content`, indistinguishable repr/fields and language equality true, but distinct present locations. Both ratified binaries gave that result. The old product lost identity at `compiler/stdlib/foundations/query.rs:55–64`, discarding known `loc` when constructing `Value::Content(c.clone())`; `compiler/eval/bindings/field_access.rs:662–665` then received only `&Content` and always returned `none`. Location cannot be reconstructed through equality and does not belong to rendering. The located-content proposal requires a value carrying Content and Location, preserving inner type/repr/equality/fields/func/has/at and exposing Location only through `location()`. Eval-created content remains unlocated; language equality excludes Location, and declarative Content does not acquire it. This public enum/exhaustive-match change required ADR-0127 confirmation; the original proposal deferred its exact Rust shape and variant name until that confirmation.

The later frozen-snapshot measurement records historical HEAD `b303f1f15b610e09872b567027e0d806387fde8c`, an uncommitted working tree with full diff/stat, independent measurement SHA-256 `82b2de8863ae5cd4b706eb9d5a6b285e1ed31dce3c126c8e5383a5af59ab46c8`, source/time/executable provenance, and upstream `a51e02804`. `entities/value.rs:64` carried bare Content with Location; `compiler/introspect.rs:1350` had already lost the chain. The R5 observation distinguishes numbering `"1"`/`"I"` and label fields and requires escape through arrays/closures.

The R5 proposal supersedes only the earlier Rust carrier shape. It changes neither Content nor HeadingElem and has this sole owner:

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct IntrospectedContent {
    content: Content,
    fields: Option<Arc<IndexMap<EcoString, Value, FxBuildHasher>>>,
}
impl IntrospectedContent {
    pub fn new(content: Content,
        fields: Option<IndexMap<EcoString, Value, FxBuildHasher>>) -> Self;
    pub fn content(&self) -> &Content;
    pub fn fields(&self) -> Option<&IndexMap<EcoString, Value, FxBuildHasher>>;
    pub fn into_content(self) -> Content;
}
Value::LocatedContent(IntrospectedContent, Location)
```

These dependencies are already authorized. The map provides name uniqueness and order; `new` moves data without realizing fields or validating language. The producer must supply a complete map excluding func/location whose body is the same frozen Content data. This coherence is a testable producer obligation, not a type-system guarantee. Expose no public fields, mutators, or `DerefMut`. `Some(empty)` differs from `None`; clones share immutable map data through Arc. Type names remain content. Mechanical derived equality/hash may expose the carrier internally but do not become language equality; Content IR and `morph_canon` gain no fields, and occurrence IDs are never reconstructed from hashes.

Snapshot acceptance preserves order/presence and Value spans on clone, unchanged fields after another snapshot, an original unchanged by explicit conversion of a copy for rendering, and equal fields across distinct Locations. **Approval-record gap:** the archived R5 clause explicitly leaves this concrete public API pending approval, and no independent R5 receipt has been verified in this consolidation. Existing code and the later bounded heading approval below do not, by themselves, certify that earlier gate. Preserve that pending verification; do not use this editorial consolidation to authorize an additional public change or reseal it.

## Pending path and Symbol boundaries

Historical vanilla `a51e02804`, `foundations/path.rs:134–224`, and probes in archived `typst-passo-1141.md` show callable type `path`, `type(path("x")) == path`, identity preservation for `path(existing-path)`, equality by root plus normalized path, and `repr` as `path("/...")`. The measured product `entities/value.rs:30–212` lacked `Value::Path` and `Type::Path`. The proposed public additions are `Value::Path(RootedPath)`/`Type::Path`, public name `"path"`, corresponding `type_of`, a callable contextual constructor under `00_nucleo/prompts/core/compiler/eval/mod.md`, and repr delegated to `00_nucleo/prompts/core/entities/path.md`. Do not reuse a geometric variant. Structural Rust equality may agree because those fields encode root plus normalized path, but acceptance remains measured language equality. This public enum/signature contract stays conditional on owner confirmation under ADR-0127; do not implement or reseal it without confirmation.

Historical vanilla `a51e02804`, `foundations/ops.rs:425–439`, measures `emoji.heart == emoji.heart` and `symbol("❤️") == symbol("❤️")` true, but `emoji.heart == symbol("❤️")` and `emoji.heart.arrow == symbol("💘")` false. Full Symbol equality is public semantics; `SymbolInner` is mechanics. `Value::Symbol` continues comparing the whole Symbol. Moving char to EcoString in symbol values/variants does not permit visible-text-only comparison or equality of native/runtime symbols merely sharing a rendered cluster. `Type::Symbol`, type names, and casts are unchanged. The public Symbol migration remains subject to confirmation in `00_nucleo/prompts/core/entities/symbol.md`; no implementation or reseal here can bypass that gate.

## Approved spot identities and color carrier

The bilateral spot-type evidence and reduced initial scope belong to `00_nucleo/prompts/core/compiler/stdlib/color.md` and `00_nucleo/prompts/core/entities/color.md`. `Type::SpotColorant` is the spot constructor's type category; `color.spot` before a call yields `Value::Type(Type::SpotColorant)`, whose own type name is `"type"`. The constructed colorant's public type name, repr/cast name is `"spot"`, distinct from `"type"`. The initial type-only description did not yet expand Color or existing Content; subsequent owner confirmation after causal commit `3ac60a578` on 2026-09-21 approved the materialized sibling `Value::SpotColorant`, parallel to Stroke/Gradient. Agent A `01a82593d` added it and `Type::SpotColorant`, resolving exhaustive consumers including repr and error formatting, without seeing tests. The recorded suite was GREEN with no regression; this is historical evidence, not a new run. That public gate required causal L0s, reseals, V5/V15/V26, a documentary commit, and explicit later human confirmation; the recorded confirmation satisfies this bounded package. The obsolete ordinal “fifteenth” is historical counting, not an enum rule.

The later language-spot package was confirmed on 2026-09-23. Before its decision, `entities/value.rs:59–106,367–423` already separated Color and SpotColorant; upstream `visualize/color.rs:280–286,2397` returned `Color::Spot` for tint. Probes measured `type(color.spot("Brand", red)) == spot`, but `type(color.spot("Brand", red).tint(50%)) == color`; JSON represents the latter by repr, not RGB. Therefore tint adds no Value or Type variant: it uses existing `Value::Color(Color::Spot)`, while the original colorant stays `Value::SpotColorant`. Fill/stroke/color-array/argument casts accepting Color must retain name, fallback, and tint. The spot constructor's fallback cast continues requiring a process color. Language equality distinguishes spot from `rgb(spot)` even when appearances agree. JSON/YAML may use canonical repr, never Debug or hex. Process projection/render/PDF boundaries remain with the Color and exporter owners.

## Approved compact Stroke payload

The memory measurement records HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, an uncommitted working tree frozen in `lab/results/construction/diagnostics/value-compacto-memoria5-2026-10-08/before.json.gz`. `layout-probe.rs.txt`, `sizes.json`, command/UTC/dependency/executable hashes measured Value/Binding 272 bytes (align16), Stroke 264 (align8), Paint/Tiling 200, located carrier 128, Content 120. `entities/value.rs:111` held inline Stroke; geometry, paint, and tiling explain its composition. An external model retaining every other payload and sharing only Stroke measured 144 bytes (align16). These are physical sizes, not RSS or speedup proof.

The owner ratified manifest SHA-256 `6badf07a4f58f08e7c60ad2e34dfb9a14a3a6ba55db90045afe046be8e988237` with `confirmo` on 2026-10-08. That package's ADR-0127 gate is satisfied; its previously proposed clauses are current. Change only `Value::Stroke(Stroke)` to `Value::Stroke(Arc<Stroke>)`. Preserve the variant name and `From<Stroke> for Value`, moving into a new Arc. Existing signatures and public Stroke/Paint/Tiling fields, other variants, Array/Dict, eval/layout APIs, phases/defaults remain unchanged. The public payload change requires direct-construction/mutation/destructuring callers to adapt within the ratified package.

This owner alone defines the carrier. Clone shares the payload without deep cloning dash/body; reads borrow Stroke. Arc identity is never language equality or occurrence identity. Type names/types, repr, fields, specified, colors/paints, thickness, cap/join/dash/miter/overhang, equality/hash semantics, and snapshots preserve their owners' contracts. Consumers modifying a Stroke produce owned results and a new Arc without mutating another Value/snapshot. No global, unsafe, I/O, new dependency, or public mutability.

Acceptance requires tests in this module before implementation: a sharing RED shows From/clone project the same Stroke by reference after the change versus inline copies before; reconstruction of stroke/style/field leaves originals intact. Footprint RED/GREEN is confined to the measured ABI, never promoted to a language ABI contract. Type/repr/fields/components controls include dash, gradient/tiling, and specified. Do not relax expected results merely to compile Arc. Adapt consumers under their own canonical L0s; run relevant core/infra/shell and PDF/SVG/PNG/HTML controls, errors/warnings, RED→GREEN, local reseal, build/lint. Paired CLI measurements use the complete same-round baseline, covering integer arrays, bindings/captures/context100/300, plain content, and unique/cloned stroke populations. A new Arc for every unique stroke can increase total bytes: retain only with measured benefit and explicit tradeoff. A 47% smaller sizeof does not imply 47% less RSS; do not mix historical percentages. Only inline Stroke is superseded; Array remains Vec/O(n) and Dict owned IndexMap.

## Value projection maintenance and acceptance

Evidence precedes the internal decision: `01_core/src/entities/value.rs:449` is_none (latent); `:465` truthy (emitted V16); `:470` cast_bool (latent); `:478` cast_int (latent); `:490` cast_float (emitted); `:495` cast_str, `:503` cast_array, `:511` cast_dict, `:519` cast_align, `:527` cast_bytes (latent); `:544` cast_decimal, `:565` cast_duration, `:576` cast_version, `:587` cast_regex (emitted). The enum at `:59–195` has 39 materialized variants, with payload inventory in this diagnostic rather than the old future list. `lab/results/construction/diagnostics/tekt-v16-value-projections-2026-10-16/before.json.gz` preserves HEAD, uncommitted working tree, UTC, status/diff HEAD --stat, hashes, original owners/consumers. AST-derived `contexts.json` is checked against pinned SARIF; `classification.json` distinguishes nine emitted V16 findings and eight latent projections. Rust shape/exhaustiveness is mechanics; public names, conversions, borrows, and language observations remain unchanged. This is no new parity/performance measurement.

Close is_none, truthy, and the twelve existing cast_* methods by nominal enumeration of Value variants. Fallbacks remain is_none=false, truthy=true, casts=None. `None` is explicit/unit; `LocatedContent` has two slots; other current payload variants have one slot, and `Auto` is unit. Exclude from a fallback group only variants already completely handled by earlier arms. Leave type_name/type_of/Type::is_callable/From/Hash/equality/shared memory/carriers untouched. Replace is_none's matches! with an exhaustive match classifying identically. Preserve formulas, signs, borrows, and Arc identity in cast_version. Float→Duration remains seconds and Int→Duration nanoseconds; keep exact limits/body without fixing NaN/rounding or certifying those edges as parity. Bool/str/bytes/array/dict/align coercions and single-variant strictness remain unchanged. Do not materialize historical pending public proposals through this maintenance.

C1–C4 cover truth/emptiness including truthy empty Dict; strict positive/negative casts and str/array/dict/bytes borrows; numeric coercions, signs, overflow and ±infinity in Duration, invalid Decimal; Arc Version preservation, valid/invalid Version parsing and Regex, and nonconvertible categories. These controls preserve current behavior, not universal parity or coverage of every payload of all 39 variants. Alternatives with the same result share a body; nominal lists require a decision on enum growth. Do not duplicate bodies merely to remove V19. V19 notes describe written alternatives, not executed tests, reachable combinations, or an N× deficit; choose positive/negative/boundary controls by behavior, without claiming one executed arm proves each alternative. No allow/expect or severity/exception relaxation hides V16.

ADR-0127 continuous flow applies: no public signature/trait/field, new productive variant, default, compatibility break, or phase change. L0 precedes tests/code, with restricted bilateral reseal and valid V1/V5/V15/V26. Run local controls before/after. Structural RED is absence of E0004 at 17 selected sites in a copied core with a sentinel added only to copied Value; GREEN requires E0004 at every selected site without stubs, and exact removal of nine selected V16 findings while retaining other diagnostics. Revalidate build, workspace suite, strict Clippy, fmt, Rustdoc, lineage under 12 GiB, zero swap, one Cargo job. Global V16 remains open while external findings remain. Parser misclassification of explicit None is separate investigation, never hidden in this consumer.

## Approved heading supplement before query

The heading measurement records HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, an uncommitted tree, UTC/diffstat/source/argv in `lab/results/construction/diagnostics/lote-a1-callbacks-reflexao-seletores-2026-10-17/final-freeze.json.gz` and `target-controls.json.gz`, upstream pin `bd5ebec285505889e8ff1d4f68804ab147a699f3`. `heading-raw-constructor-func` passed baseline/vanilla but failed the candidate because transport Styled changed func; query saw a temporary causal wrapper and HTML ignored callback panic. `walk.rs:930–945`, `pipeline/paged.rs:419–427`, and `pipeline/html.rs:101–106` prepare query by pure walk. These are language values/errors, not instructions to copy upstream mechanics. The owner confirmed H2 in `lab/history/construction/records/00_nucleo/materialization/heading-supplement-runtime-2026-10-17/review.md`, manifest SHA-256 `66739b13596013f4343bdc3ffacbdb9081151e3d9863e5d962e8ff13fd68775c`.

Preserve every public IntrospectedContent/Value signature. The private fields slot remains `Option<Arc<...>>`, now pointing to private `SnapshotFields` containing the unique map and `heading_styles: Option<Arc<Styles>>`. Derive Clone/Debug/PartialEq for that private header, structurally comparing the map and pending styles as Rust mechanics. `new` moves the map with no pending styles; `fields` returns the same `&IndexMap`. Add only crate-private attachment/read helpers for pending styles, copying locally when the Arc is shared. Auxiliary styles never become public fields; existing clones stay immutable. Do not increase Value/IntrospectedContent slots, sizeof, or alignment; measure before/after on the same ABI.

Acceptance is ratified H1–H8 with bilateral RED before code. H2 authorizes no public contract beyond the Heading facade; the separate A1-S2 approval covers five State helpers. No new public fields. Ordinary Styled and other callbacks retain their boundaries. Run workspace/doctests/build/Clippy/Rustdoc, fmt, strict/reverse V1/V5/V15/V26, and no V16 increase under 12GiB/High8GiB/swap0/jobs1, heavy commands sequentially. RAM benefit requires measurement rather than intent.

## Base value controls and unresolved scope

Co-located `#[cfg(test)]` controls cover each materialized variant and these original obligations: None has type `"none"` and is_none true; Int(0) is_none false; Int(3).cast_float gives Some(3.0), Float(3.14) Some(3.14), Bool(true) None. From("hello"), 42i64, 3.14f64, and true yield Str(EcoString), Int, Float, Bool. Equal/different EcoStrings compare correspondingly; mutating a cloned Array does not affect its source. Dict has type `"dictionary"` and `cast_dict().get("k") == Some(&Value::Int(1))`; Module clone remains type `"module"` with Arc sharing. Length pt12 has type `"length"`, Ratio .5 `"ratio"`, Angle90 `"angle"`, Color rgb0 `"color"`, Auto `"auto"`, Tiling `"tiling"`. Scope::define("x", Int(42)) yields the same value from scope.get("x"). Relative's old test label `"relative length"` conflicts with the measured corrected public name `"relative"`; the measured relative-type correction supersedes that stale spelling, without deleting the constructor/cast control itself.

Path approval, the Symbol owner's public gate, R5 receipt verification, the remaining namespace/type reconciliation, and unmaterialized future variants remain explicit boundaries. Neither current implementation, a historical GREEN receipt, mechanical equality, nor this English consolidation supplies missing approval or current-pin parity certification.


## Issue 8894 — approved Stroke migration

Measurement: the source inventory and previous owner bytes in
`lab/results/construction/diagnostics/issue-8894-2026-10-31/` bind exact
HEAD/dirty diffstat/UTC and each affected source/owner before migration.
`stdlib/layout/stroke.rs:430–434` erases em; geometry.rs:81 carries only f64.
The explicit user approval in approval.json ratifies the public Length field
and occurrence-layout resolution, as specified by
`00_nucleo/prompts/core/entities/geometry.md`. This consumer retains its
exclusive responsibility and all other ratified clauses and controls.

Migrate this consumer's declarative Stroke literal thickness from scalar
points to Length::pt without changing their value or other fields. FrameStroke
remains scalar. Keep all existing semantic assertions; update only their
representation as necessary. Producers preserve full Length; layout consumers
resolve against effective font size before scalar geometry. Repr preserves
absolute/em components. RED→GREEN, build/tests and scoped lineage are required.
