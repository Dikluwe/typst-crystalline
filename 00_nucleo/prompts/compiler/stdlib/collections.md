# L0 — stdlib/collections
Hash do Código: 44d906a8

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/collections.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/collections.rs:37` (pubcrate) struct CollectionCallSpans {); `01_core/src/compiler/stdlib/collections.rs:66` (pubcrate) fn try_dispatch_collection_method); `01_core/src/compiler/stdlib/collections.rs:180` (fn try_dispatch_bytes_method) identifies the current owner and its local entrypoints. This is implementation inspection, not a new bilateral parity measurement or an inference of upstream intent. The historical decision evidence and its original snapshot qualifications are retained in `00_nucleo/diagnosticos/sanitization-parser-stdlib-history.md`.

## Contract and ownership

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

## Acceptance

Preserve the specified language semantics, syntax, morphology, diagnostics, spans, defaults and public identities. Do not change signatures, compatibility, feature defaults or phase ownership through this documentary revision. Shared pinned claims remain authoritative; this prompt owns only its exclusive consumer. Architectural decisions follow ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.

## Atomização comportamental P1497

**Medição:** a working tree não commitada identificada em `00_nucleo/diagnosticos/atomizacao-stdlib-collections-2026-09-30-before.json.gz` contém 3542 linhas neste consumer; a suíte do L1 passou com 6401 testes e 2 ignorados. Esta medição não é benchmark nem nova comparação com o vanilla.

**Decisão:** manter neste owner o despacho exaustivo, a tabela estática de membros, os adaptadores de chamada estática e os metadados sintácticos de chamada. Os algoritmos são delegados aos owners exclusivos `collections/{validation,bytes,array_access,array_aggregate,array_predicate,array_sequence,dictionary,str_access,str_search,str_transform,arguments}.md`. As cláusulas de comportamento acima são o mapa de superfície; cada owner filho especifica sua família. A separação é de código e L0, sem alteração intencional da linguagem.
