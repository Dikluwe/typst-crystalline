# L0 — stdlib/collections
Hash do Código: 2d77bf43

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/call-once.toml sha256:3c1f911b856e019f09d75d07fd14052912e8f0fb6118fceaaac9a7f4a92bfd97

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/collections.rs`

## Evidence before specification


`01_core/src/compiler/stdlib/collections.rs:37` (pub crate) struct CollectionCallSpans {); `01_core/src/compiler/stdlib/collections.rs:66` (pub crate) fn try_dispatch_collection_method); `01_core/src/compiler/stdlib/collections.rs:180` (fn try_dispatch_bytes_method) identifies the current owner and its local entrypoints. This is implementation inspection, not a new bilateral parity measurement or an inference of upstream intent. The historical decision evidence and its original snapshot qualifications are retained in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.



Historical probes against `9f2b6e8715237cb086899a42873660fe744622e8` retain their original snapshot limits and do not certify current `bd5ebec285505889e8ff1d4f68804ab147a699f3` without remeasurement. Language semantics, syntax, morphology and observable diagnostics govern acceptance; enum/match/algorithm structure is mechanics. Source inspection is not inferred upstream intent.

`lab/results/construction/diagnostics/atomizacao-stdlib-collections-2026-09-30-before.json.gz` records uncommitted 3542-line original, L1 suite 6401 pass/2 ignored; structural extraction, not benchmark/new parity. caminhos-residuais-ci-2026-10-13/before.json.gz records unused wrapper/import spans 519/553, dirty HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, UTC/status/stat/sources/L0/uses.

`lab/results/construction/diagnostics/tekt-v16-catalogos-2026-10-17/` before/before-measurement.json.gz fixes dirty HEAD/source/config/L0/binaries/UTC/stat,118fields(115CLI+3Location module tests),159 pairs/318invocations:141 match exit/stdout/main error+hints,18 retain prior diagnostic differences. Type 38/Value39 domain versus open string names is the structural distinction; invented completeness/omitted-field absence is not proved. Oracle binary `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.

`lab/results/construction/diagnostics/call-once-2026-10-17/before.json.gz` records HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, dirty tree/UTC/status/diff HEAD --stat/sources/L0/config/hashes;2124 predecessor hashes were checked exactly before baseline. before-probe.json.gz/before-math-probe.json.gz retain bilateral binaries/argv/fixtures/outputs (historical oracle 9f2b6e871 binary `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`). Receiver/args repeated, missing methods evaluate panic args before causal error, math.mat duplicates accumulation and empty-array truncate hid it. Local call/collection/context/content/selector/method/invoke paths violate existing call_dispatch once-only contract; pinned-source.json.gz fixes upstream call/methods evidence. Invalid initial reserved-word `or` fixture is retained separately, no call-effect credit; its import panic 101 remains parsing/import debt.

## Exclusive composition and language boundary criteria


Own CollectionCallSpans, try_dispatch_collection_method, collection_type_field, static_collection_call and the static operation catalogue. The algorithmic owners in `collections/*.md` own argument validation and collection behaviors for Array, Dictionary, String, Bytes and Arguments. Field access/call-dispatch owners discover and forward operations; l-value writeback belongs to the binding owner. Preserve the same native operation identity and self-first calling convention. Do not duplicate algorithms in eval.

### Public operations and defaults

Array: all/any/filter/find/position(test), map(mapper), fold(init,folder), reduce(reducer), at(index,default:?), first/last(default:?), len, contains(value), chunks(size,exact:false), windows(size), dedup(key:?), enumerate(start:0), flatten, insert(index,value), intersperse(separator), join(separator:none,last:?,default:none), pop/push(value), product/sum(default:?), remove(index,default:?), rev, slice(start,end:none,count:?), sorted(key:?,by:?), split(at), to-dict and zip(exact:false,..others). Preserve global/static range(start:0,end,inclusive:false,step:1) linkage; range implementation belongs to foundations/cast.

Dictionary: at(key,default:?), filter(test), insert(key,value), keys/values/pairs, len, map(mapper), remove(key,default:?). Preserve the separate crystalline update extension. String: at(index,default:?), first/last(default:?), len, clusters, codepoints, contains/ends-with/starts-with/find/position/match/matches(pattern), normalize(form:"nfc"), replace(pattern,replacement,count:?), rev, slice(start,end:none,count:?), split(pattern:none), trim(pattern:none,at:?,repeat:true). to-unicode/from-unicode stay delegated to foundations/str. Preserve crystalline char-len/char-at/char-slice, repeat and to-upper/to-lower extensions without substituting them for vanilla names. Bytes exposes only len, at(index,default:?) and slice(start,end:none,count:?), not first/last. Arguments exposes at(key,default:?), filter(test), len, map(mapper), named and pos.

? denotes an optional named value absent when omitted. Preserve argument order, named validation, duplicate causal carriers and first-error precedence; no silent acceptance of extras or coercion by truthiness where Bool is required.

### Semantics

all/any short-circuit; all/any/find/position/filter and sorted(by:) require Bool results. map/fold/reduce preserve order. flatten is recursive, superseding the old one-level prose. dedup keeps the first global occurrence and supports key. sorted is stable and supports key and by together. chunks exact discards the incomplete remainder; windows produces only complete windows; zero sizes fail. zip truncates to the shortest by default, exact rejects unequal lengths, and no additional arrays yields singleton tuples.

Negative indexes count from the end. Defaults apply only to absence/out-of-bounds, not invalid casts; insert accepts the terminal boundary. Preserve bound l-value mutation and result/error semantics for push/pop/insert/remove; unbound projections do not mutate remote bindings. sum/product use language operations and require a default on empty inputs; reduce on empty returns none. join uses language join with the distinct final separator and empty default. Dictionary iteration preserves insertion order; to-dict requires (str,any) pairs and last duplicate key wins.

String indexing/positions/match offsets are UTF-8 bytes. Reject invalid character boundaries rather than clamping at/slice; first/last are scalars, codepoints are scalars and clusters are extended graphemes. find returns the matched substring or none, position its byte index or none. match/matches return start/end/text/captures, nonoverlapping and ordered; preserve literal/regex and replacement/trim/split distinctions. normalize supports nfc/nfd/nfkc/nfkd, default nfc. Bytes counts octets, at returns 0..255 Int and slice returns Bytes.

### Causal Arguments

Static forwarding removes only the receiver through the coherent Args API. Arguments.filter/map iterate occurrence_sequence in joint positional/named order, including legal duplicate names. Filter invokes once per occurrence, requires Bool and retains the original occurrence when true. Map invokes once, keeps name/argument span, replaces value and detaches value_span even if the new Value compares equal. Return Args::from_occurrences with detached aggregate; do not discard a user's carrier to use legacy separated views.

Arguments.len counts occurrences when Some, otherwise items+named; this does not redefine Rust Args::len. pos/named return coherent views; at uses positional index or last named lookup. Filter's return-type cast uses the Func diagnostic_span and long type names; callback-produced errors keep their own origins. Synthetic/detached functions stay detached. Preserve first failure, With/alias origins and one callback invocation per occurrence. Map does not inherit old lexical origins by equality.

CollectionCallSpans remains private call metadata for the specifically supported byte-method corrections, not a universal source-origin policy. Full messages, hints, traces and spans remain observable; canonical repr belongs to its eval owner.



Private algorithms stay in canonical `00_nucleo/prompts/core/compiler/stdlib/collections/` owners for validation/bytes/array_access/array_aggregate/array_predicate/array_sequence/dictionary/str_access/str_search/str_transform/arguments. This hub keeps exhaustive dispatch/static catalog/static adapters/syntactic metadata only. Remove only unregistered native_array_all_static/native_str_clusters_static wrappers and orphan array_all_with_pred import; retain active collection_static/array_all_with_pred/str_clusters and existing exposed paths/validation/effects (array_all calls predicate child). No unrelated helper/test/body deletion.

Classify Value exhaustively before open method lookup; Array/Dict/Str/Args preserve RHS/results/callbacks and Bytes primitives/span metadata. Unknown name returns None/Unhandled, recognized error remains Some(Err)/Handled(Err). Explicit other 34-Value group is no-receiver-here policy, not discarded domain information. Type catalog exhaustively Array/Dictionary/Str/Bytes/Arguments with same callback identity/ABI, other 33-Type no-catalog-here group, not universal no-members. Family string lookup stays open. Preserve crystalline extensions/self-first/static forwarding/writeback/validators/no invented absent fields; new Value/Type must force compile classification.

Keep eager try_dispatch_collection_method signature as already-evaluated-Args adapter. Add crate-only CollectionDispatch{Handled(SourceResult<Value>),Unhandled(Value)} and try_dispatch_collection_method_with taking FnOnce(&mut Scopes,&mut EvalContext,&mut Engine)->SourceResult<Args>. Same static catalog, local bind_args(args,expr) evaluates only recognized arm/propagates arg error as Handled. Unknown name returns original receiver reconstructed by move, no clones/probes/second catalog. Exhaustive nominal complement retained. Integrate exactly bytes len/at/slice via existing primitives; eager adapter maps Handled→Some, Unhandled→None. No algorithm/catalog-Type/static-wrapper/extension changes.

## Acceptance and limits

Structural current-pattern control compiles/future Value-Type rejects E0004 in isolated probe, not actual workspace/target mutation. Erased RHS/stubs isolate patterns, not functional equivalence; functional controls pass before/after, no invented functional RED. Global four V16 become three family decisions because first owner combines two matches; restricted two-owner reseal after V15/V26. Repeat 159 pairs preserving each backend's full stdout/stderr including 18 known negatives; test same-name different Func/constant-vs-function/cross misses/Location, no blanket diagnostic parity. Build/workspace/strict Clippy including unfulfilled/Rustdoc four indexes/fmt/diff/strict-reverse lineage/differential SARIF under 12GiB/High 10GiB/swap0/jobs1. No commit/push/merge/release.

Call-once complement tests construct 39distinct Value discriminants (36language+pure RootedPath/Locator/IntrospectedContent). Unknown-name test preserves payload equality/original Vec pointer and panic-if-called Args; callable classifier checks 37pre-Args rejections and Func/Type actual-argument panic. This proves internal transport/classification, not upstream cast capability/all schema parity. Consume pinned call-once invariants. Internal ADR-0127 continuous-flow correction supersedes only conflicting internal-signature/name-compensation restrictions, retaining Tekt extensions, public functions/reexports/structs/traits/fields/default/phase. Functional co-located RED→GREEN for observed receiver/args effects, mutating fallback/error-before-args, where/With/selector/collections/matrix without truncating legitimate effects; hits/misses, spans/occurrences and Value complements. Unhandled preserves receiver and does not execute args callback; count effects, not PDF bytes/Rust Eq/benchmark. Preserve residual diagnostics explicitly. Ownership/V26 before restricted resealing; build/workspace/strict all-target Clippy including unfulfilled expectations/Rustdoc/fmt/strict-reverse V1/V5/V15/V26/global differential SARIF, no new error/config/suppression. Informative V19 retains alternatives matrix and behavioral relation. Sequential 12GiB/High 10GiB/swap0/jobs1. No publish/commit/merge/release/global-completion claim.

## Constructor-admission regression expectation

### Evidence before decision

At baseline HEAD `91ea6ee911`, the historical constructor test at
`compiler/stdlib/collections.rs:1534-1548` asserts that Array is unsupported by
`type_as_callable`. The current-pin receipts
`lab/results/v16-corrections-2026-10-19/batch-f/agents/adapters/baseline.json`
and `baseline-supplement.json` demonstrate `(bytes((0,255)),).map(array)` succeeds
upstream while the existing direct Tekt Array(Bytes) constructor already succeeds.
The mapping policy and repair belong exclusively to
`00_nucleo/prompts/core/compiler/stdlib/collections/validation.md`.

### Contract and acceptance

Update only this old test's Array assertion to require admission. Preserve all
nine prior positive constructor assertions and the Bool/Length rejections. This
assertion must fail on the old adapter and pass after its measured mapping fix;
retain it as an existing-test semantic RED distinct from newly added tests.
No productive hub dispatch, catalogue, callback or signature change is authorized.
This test expectation corrects an obsolete mapping restriction under ADR-0127
continuous flow and does not relax the remaining negative controls.
