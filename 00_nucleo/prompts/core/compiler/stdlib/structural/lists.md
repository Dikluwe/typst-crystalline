# L0 — compiler/stdlib/structural/lists — list constructors
Hash do Código: ab0b90b3

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/structural/lists.rs`

## Evidence before specification

L1 exclusive consumer `01_core/src/compiler/stdlib/structural/lists.rs`. List/Enum co-change in P494-495/P505 supports their shared indentation/validation owner; Terms remains pair form with insertion order. Canonical catalog `00_nucleo/prompts/core/compiler/stdlib/structural/mod.md` and conventions `00_nucleo/prompts/core/compiler/stdlib/mod.md` supply boundary criteria, not extra ownership. Pinned list.rs/enum.rs/terms.rs define required Content bodies and typed-number matching. Historical 2026-09-21 oracle 586e1bd43fae supports qualified item functions missing in Tekt. Existing ListItem/EnumItem/TermItem entity fields suffice; no entity expansion needed. Historical probes against `9f2b6e8715237cb086899a42873660fe744622e8` retain their original snapshot limits and do not certify current `bd5ebec285505889e8ff1d4f68804ab147a699f3` without remeasurement. Language semantics, syntax, morphology and observable diagnostics govern acceptance; enum/match/algorithm structure is mechanics. Source inspection is not inferred upstream intent.

Measured enum.item(3,[a]) and enum.item([a],3) agree; terms positional TermItems accepted upstream while old named extension rejected. Current named-only terms was documented intentional behavior, so adding useful positional constructor route must retain compatible named extension. Human confirmation 2026-09-21 follows causal commit 3ac60a578 and approves three qualified routes/additive Terms form. Independent eval_expression tests 20 functional RED before implementation, complete core 6129/6129 final after integration, no regression; do not treat this historical count as current-run certification.

## Exclusive current contract

Construct list(..items,marker:,marker-align:,indent:,body-indent:,tight:), enum(..items,numbering:,start:,indent:,body-indent:,tight:) and terms. Shared indent/body-indent require Length, tight Bool, unknown named error; Enum propagates start. Pure L1, no unordered map/reordering of named Terms insertion.

Register each list/enum/terms with Func::native_with_namespace(name,native,Arc::new(scope)) containing exactly item; registration is eval hub-owned `00_nucleo/prompts/core/compiler/eval/mod.md`, using existing namespace mechanism/no new type. Bare Func surface remains.

list.item requires one positional Content, no named, constructs ListItem with body and marker/marker_align/indent/body_indent/tight None (inherit list parent only when in list; standalone no override). Exact missing `missing argument: body`, second `unexpected argument`, named body ``the argument `body` is positional`` plus hint `try removing \`body:\``, wrong `expected content, found <tipo>`.

enum.item accepts optional Int and required Content matched by type in either order; first Int→Some(n as u32), body→Content. Construct EnumItem with number/body, numbering/indent/body_indent/tight None. Missing body `missing argument: body`; unmatched first Float casts body→`expected content, found float`, not number error. Other diagnostics follow list.item.

terms.item requires positional Content term then description matched by order, constructs existing Content::term_item. Missing `missing argument: term`/`missing argument: description`; excess `unexpected argument`. terms with nonempty positional list accepts only Value::Content(Content::TermItem(_)), building Content::terms directly without using named. Nonempty positional and named together fail `unexpected argument` at first named. With no positional retain insertion-ordered named key→description form unchanged. This named form is additive Crystalline extension/no upstream parity credit, not removed by positional correction.

## Acceptance and limits

Verify two markup list items/no marker, two enum items/start 3, invalid numeric indent/tight/numbering, unknown named and empty list. Qualified tests go through eval_expression/real native registration and assert exact bodies/override absence/type matching/order/missing-excess/named mix diagnostics; do not use direct Rust construction of then-nonexistent types as functional RED. Preserve language fields/morphology/syntax and L1 purity. Ratification covers qualified routes/additive Terms only; L0/ownership/pins/V5/V15/V26/restricted reseal/build/full suites/lint apply before code changes. No numbering/render/phase/public entity expansion implied.
