# L0 — sym — Unicode symbol module
Hash do Código: d3474409

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/sym.rs`

## Evidence before specification

Exclusive pure L1 consumer `01_core/src/compiler/stdlib/sym.rs`. Initial priority table/Dict evolved to Module, then full declarative adapter. Current symbol data pin is `codex` git `2e43ab90079cf9f9fa670d421c8c38233143b376`, never moving branch/version label. Historical codex 0.3.0 checksum `0732ab1a27b4ea05e6f9f60a5122c9924dd5123defde0d8e907f58cf643d40e6`, sym.txt SHA `6ee467d9939acb5c7d0a3eba30c9f640d157529cbf57df752367deb343e0fc16`, lines 1-1340 measured 297 direct/37 submodule symbols/2modules/1206records/46 no-bare parents: inventory, not parity percentage. Current sym.txt SHA `60c79540af8ebe590c80d672b55a41f337bc8d61ebd42d9be8879fce9b18f8bc`. Historical probes against `9f2b6e8715237cb086899a42873660fe744622e8` retain their original snapshot limits and do not certify current `bd5ebec285505889e8ff1d4f68804ab147a699f3` without remeasurement. Language semantics, syntax, morphology and observable diagnostics govern acceptance; enum/match/algorithm structure is mechanics. Source inspection is not inferred upstream intent.

Pinned newer source 765-793,1377-1400 adds chess/keyboard/ordinal (3modules/21children/19variants); bilateral repr 43paths per namespace and 12families/5tack.double/not.inv/spacebar changes are preserved in P1495 diagnostic 198 probes/provenance. CHANGELOG33-47 removes join/tri paths and deprecates spacebar; metadata alone is insufficient. Clean viability HEAD `0d722061c6f285501e3614bef823b4f16bcb13c5`,2026-09-25UTC shows injecting tri into canonical gt/lt pollutes repr; pinned sym 315-378 has closed, not tri. Legacy P820 join/bowtie warning/span/source measurements remain historical, with current compatibility policy below superseding general warning uniformity. sym.rs:528/C022 type-complexity before.json.gz/red-clippy.log preserves HEAD/dirty-state/UTC/stat/source/owners/hashes, structural/no AI/runtime/novel-parity claim.

## Exclusive current catalog contract

Pure adapter over exact pinned codex::SYM, recursive Def::Module preserves gender/control/chess/keyboard/ordinal and every current path/kind/default/repr/modifier/Unicode variant. Simple names map full single grapheme strings; grouped Symbol::with_variants preserves complete strings including VS15/VS16/ZWJ, modifier sets independent of order. Parents without bare use codex empty ModifierSet best match/fewest extra/first-tie, never invented empty variant. Char→str change preserves old single-codepoint values; new multicodepoint entries require their own measured source.

Keep distinct alias names with full base+variant identity (dollar/pataca, yen/yuan, emptyset/nothing, gradient/nabla); same base alone doesn't make infinity/oo structural aliases. sym.sqrt is absent; public radical route math.sqrt, internal glyph remains permitted. sym.registered is separate historical Crystalline extension/no parity credit.

Provide sym_lookup(name:&str)->Option<Symbol> for compound/predefined/chained variants and build_sym_module()->Value::Module. Only dotless names enter scope, compounds resolve lookup. Root scope registers sym via build_sym_module. Value::Symbol in markup becomes Content::Text(s.value.clone()) preserving whole grapheme. Align symbol data across Module/math mirror/bare lookup.

Preserve removed twelve paths as compatibility extensions: join⨝/.r⟖/.l⟕/.l.r⟗; gt.tri⊳/.eq⊵/.eq.not⋭/.not⋫; lt.tri⊲/.eq⊴/.eq.not⋬/.not⋪. Use overlay by exact modifier-set path, including equivalent permutations such as gt.eq.tri.not; do not replace canonical gt/lt groups or inject tri into their variants/repr, lose new closed variants or open fallback for any other removed path. build_sym_module adds only top-level join/registered; canonical gt/lt field access with prior eq/not resolves selective legacy .tri through `00_nucleo/prompts/core/compiler/eval/bindings/field_access.md`. Existing registered remains separate. Five tack.double new values are measured table corrections with same path/signature/carrier/phase, continuous ADR-0127; changed pin-language value/public contract would refute that classification.

Top-level deprecation lookup sym_deprecation(name)->Option<&'static str> preserves exact `` `join` is deprecated, use `bowtie.big` instead `` without hint, root MathIdent/field spans and successful symbol resolution. join warning stays sym/bare but math.join remains historically silent; uniformizing extension warning is a new default requiring separate ADR-0127 approval. spacebar resolves/warns all three access forms with exact `` `spacebar` is deprecated, use `keyboard.space` instead ``. Variant-deprecation message absence in SymbolVariant is intentional individual diagnostic debt, never permission to omit symbol/variant or claim warning parity. Do not revive removed tri warnings or warn five now-undepr­ecated tack variants. Warning emission belongs to `00_nucleo/prompts/core/compiler/eval/math.md` and canonical field-access owner.

Keep static SYM_GROUPS documented tuple name/base/factory fn()->variants. Exact static-only expectation:

```rust
#[expect(clippy::type_complexity, reason = "SYM_GROUPS: tabela estática nome/base/factory com contrato explícito")]
```

No changed data/factories/catalog/visibility/selection. Nominal exception only type_complexity, no module/impl/crate/global threshold/other lint suppression; generic bans overridden only for justified items, correction obligations remain. Any type alias requires identical expansion proof; new public/incompatible type needs separate gate.

## Acceptance and limits

RED→GREEN 43paths per namespace,12families full values/modifiers/repr,5tack/not.inv/spacebar,12legacy alias permutations across three access modes and registered/sqrt/emoji controls; no parity credit for compatibility extensions. Check join/root field warning spans/exit 0, bowtie default⋈/big⨝/stroked⋈/filled⧓ no warning, unknown foo.bar→`unknown variable: foo` plus 2hints, never replace exact diagnostics with Portuguese. Future public alias removal requires new L0/human ADR-0127 confirmation.

Structural selected type-complexity gate RED→GREEN/unfulfilled expectations as error, existing functional values unchanged. Revalidate build/all-target Clippy/workspace/fmt/Rustdoc/strict-reverse lineage/restricted reseal; no tests mirroring aliases/field names and no memory/performance gain inferred. Historical counts/pins are provenance, current complete catalog duty follows exact codex pin above; future upstream resync is separately authorized.
