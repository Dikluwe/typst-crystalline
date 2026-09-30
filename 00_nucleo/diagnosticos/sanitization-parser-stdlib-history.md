# Parser and standard-library L0 sanitation history

## Status and scope

This is an exclusive historical/diagnostic document, not a materializable L0 and not an authorization to change code. The sanitation is **partial**: the revised files are inventoried below; remaining Portuguese or historically mixed specifications are explicitly listed as pending. The parser-engine children and pre-existing English registry/test prompts are retained unchanged; that fact is not a completed audit of the entire stdlib family. No global sanitation completion is asserted.

The assigned scope was discovered from productive and test consumers' @prompt headers across L1–L4, not filename resemblance. The only directly compiler/ owner discovered for parse/stdlib is `00_nucleo/prompts/compiler/parse.md`. Stdlib's façade owner is `00_nucleo/prompts/compiler/stdlib/_comum.md`. The integration test `01_core/tests/p1405_outline_indented.rs` owns its assigned test prompt despite living outside the stdlib source directory. `00_nucleo/prompts/compiler/stdlib/text/regex.md` has no consumer: it is a historical redirect, not an orphan productive specification.

## Provenance and limits

The inspection snapshot used HEAD `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, with an uncommitted working tree, at `2026-09-27T01:56:35.389Z`. Per-document pre-edit SHA-256 and consumer SHA-256 are retained in the inventory to identify the exact dirty files inspected; this is not a clean-HEAD or newly measured performance claim. Existing unrelated changes, including prior layout sanitation, were preserved. No new bilateral oracle execution, parity score, benchmark, line count, productive build/test run or global hash reseal was performed.

The current upstream language reference is the already ratified `9f2b6e8715237cb086899a42873660fe744622e8`. Evidence originally measured against `a51e02804`, `586e1bd43fae6c9a973218163d3165c53ab8d16d` or another old snapshot remains historical evidence only for that snapshot. A version string, a source inspection or a retained regression test is not new parity proof against the current pin. Older abbreviated file:line references below retain their original context and are not silently asserted to address the present upstream tree.

## Ownership decisions retained

- Parser façade: pure mode-specific roots and anchoring. Grammar children own their productions, precedence and grammar diagnostics; parser.rs owns engine types/composition, with state/checkpoint/diagnostic algorithms in its children. No dynamic dispatch or render logic moved into entities.
- Stdlib façade: declarations, reexports and the actually local err/expect_no_named helpers. Historical generic coercion, promotion, numeric formatting and guard_float conventions did not turn mod.rs into their owner. Native ABI descriptions omitting World/current_file were obsolete. Registry and test-suite modules are separate consumers.
- Foundations, text, structural and primitive façades: module composition/reexports, not duplicated child algorithms. Regex moved from text through the intermediate str location to regex_constructor; the redirect now identifies the actual header owner. The global len removal does not remove collection len methods.
- Representation: native repr remains a wrapper around the eval repr owner. Symbol pretty-printing, spacing/break presence and PageRun morphology are integration obligations, not a second repr_content implementation owned by the wrapper. Their original measurements remain in the evidence index.
- Static identity and calls: constructor/method native identities, bare/qualified convergence, self-first adapters and exhaustive/static delegation remain. Synthetic computed arguments do not acquire fabricated lexical provenance. Render behavior stays in its render layer under ADR-0109 form B.
- Test-only prompts: own their assertions and consumers, not productive implementations. Opaque observations, wrong identity and unattributable execution failures are not success. Test suites were not edited or run.

## Historical material relocated

### Parser

The former façade/grammar notes described completed atomization, mode boundaries, empty closed strong/emph warnings, lexer whitespace aggregation and speculative checkpoint/memo integration. Empty closed delimiters retain their warning behavior; unclosed delimiters remain errors. Lexer aggregation is not markup grammar ownership. The parser-engine prompts were already English and remain unchanged, including depth protection, rollback boundaries, retained error nodes and the explicit RequireColumn comparison.

### Foundation/type changes and prior gates

The older primitive/regex/selector function bindings preceded the established callable Type bindings. The executed relocation and type-binding phases are historical, not future work orders. Decimal retains its single-string restriction; Duration retains its string/component alternatives and i128 arithmetic; Version retains arbitrary explicit components and the separately owned at adapter. array(Bytes) remains deliberately incomplete rather than an assertion of the whole array constructor surface. Retirement of conditional observation harnesses does not retire productive tests or authorize replacement instrumentation.

The old Unicode-zero rejection was refuted by the recorded scalar-domain source: U+0000 remains valid. Label string conversion and multi-codepoint Symbol conversion preserve complete values. Float's earlier scope-outs for inf/nan/signum/bytes were succeeded by the later explicit contract, without changing the predicates or broadening constructor casts into parameter casts. Int/Float byte results are publicly observable API data, not render-byte parity. Historical public-carrier/Args gates are not newly approved here.

### Calc and causal collections

Calc.abs's historical diagnostic fixes retain the first positional value failure before leftover errors, distinct value/argument spans, named-positional hints and resolved short intrinsic name. Earlier prose converting every rounded Float into Int was succeeded by the later kind-preserving round contract. Old atan2/tan boundary examples are not renewed oracle evidence. guard_float remains categorical where the owned implementation routes through it; abs and operators do not acquire a blanket guard. Deliberate log10/deg/rad compatibility extensions remain.

Collections' old one-level flatten and narrower inventories were succeeded by recursive flatten, full static/bound projections, stable sorting and global-first dedup. Arguments callbacks use joint occurrence order, retain duplicates and detach mapped value origins even on equal results. Legacy separated views are not a license to collapse a causal carrier. L-value writeback remains in bindings, not the collections algorithms. These historical extensions are not general collection parity closure.

### Colors

The former Dict color namespace preceded the established Type surface. Palette space/component observations are not reproduced by replacing native CMYK values with sRGB approximations. Historical inventory extraction that missed nonuniform hex spelling did not justify shortening maps. The old no-tint spot scope-out was succeeded by the later SpotColorant/tint contract: same-colorant identity, typed ratio bounds and explicitly unsupported operations remain. No claim of exporter Separation output is established by language color support.

### Text ownership and co-change rationale

Older co-change clusters involving heading/outline/title/divider and text smallcaps/shift/highlight/smartquote were contaminated by comment-boundary attribution, formatting, renames or cross-feature batches. The existing dedicated units remain justified by the actual feature boundary; no cluster metric becomes a normative gate. Lorem's approximate-output acceptance was revoked in favor of exact generated language text, while per-call chain construction remains deliberate pure-core mechanics rather than upstream LazyLock copying. The existing MIT/lipsum attribution remains applicable.

Text constructor known-but-uncaptured properties remain an explicit limitation, not invented support. Decoration radius/extent and script size are supported rather than old scope-outs. Typed color rejection superseded the string CSS compatibility catalog under the recorded confirmed compatibility decision; no new confirmation is inferred here. Historic P284 references remain where they are literal public error text. SmartQuote's explicit leaf carrier preserves public identity, omission and explicit auto; repr transparency alone never proved total wrapper transparency. Its old gate/proposal language is archived, not new approval for public fields or unmeasured BCP-47/content.fields behavior.

### Context, observations and metadata

The context constructor is not owner of pipeline expansion. Query's former content-only projection was succeeded by a complete snapshot+Location carrier; causal observation of the selection is not just observing location lists. Locatability belongs to contextual consumers, not selector construction/show. Native target's paged-only description became obsolete with existing Html/Bundle target propagation; no target or feature default is changed by this sanitation.

Document's metadata-only direct call and asset(path,kind:) placeholder were explicitly succeeded by the bundle path/body and path/data constructor contracts. Set-document metadata remains a separate eval rule. The placeholder withdrawal, staged public-contract proposal and target orchestration chronology are historical, not normative permission to expose a partial constructor. Bibliography's omitted-style correction retains ieee, explicit none remains distinct, and Path reads preserve rooted identity through the injected loader. Table header levels are the explicit semantic exception to table/grid symmetry, not a general permission to diverge them.

### Catalog provenance

Sys's old Dict pseudocode and accepted-Dict-repr table contradicted its later Module correction; Module composition and shared PARITY_VERSION remain. Emoji's early one-scalar subset was succeeded by complete Unicode/variant data. Its existing codex git pin `2e43ab90079cf9f9fa670d421c8c38233143b376` and emoji.txt SHA-256 `8691ca68e09b6fedca61e00e824648e79f7502772eefe7af367e404e26161489` remain; the historical codex 0.3.0 checksum `0732ab1a27b4ea05e6f9f60a5122c9924dd5123defde0d8e907f58cf643d40e6` is not the current dependency pin. Unchanged emoji data does not establish unchanged sym data. Sym itself remains in the pending list below.

## Contract/code discrepancies and unresolved qualifications

- Stdlib façade: the original internal-only native_flush requirement versus its current public reexport at `01_core/src/compiler/stdlib/mod.rs:150` remains unresolved; sanitation does not authorize public visibility expansion.
- Ref: `01_core/src/compiler/stdlib/ref.rs:37` accepts Label in addition to the original Str restriction; `:54–55` accepts Str/None supplement in addition to Content; leftovers are not closed by the current native. The stricter original contract is preserved and the extensions are explicitly recorded, not silently legalized.
- Markup: the original requirement that every listed native provide a no-argument selector sentinel conflicts with `01_core/src/compiler/stdlib/structural/markup.rs:164`, where link rejects a missing URL. Function-valued show use does not prove no-argument native-call behavior. The obligation is retained pending a separate decision.
- Query/target: the old paged-only clause conflicts with `01_core/src/compiler/stdlib/foundations/query.rs:159` and ADR-0128's separate semantic target architecture. Existing target propagation is retained, and this discrepancy is explicitly archived rather than silently reverting behavior.
- Type examples abbreviated the arity diagnostic; `01_core/src/compiler/stdlib/foundations/ty.rs:29` includes the received-count suffix. No new parity claim follows from retaining the actual full diagnostic.
- Cast bytes: an old hexadecimal-byte example conflicted with the stated unsigned 0..255 domain. The domain is retained; the inconsistent example is not promoted to a new requirement.
- Panic's invalid-named message/primary-span debt remains outside the historical abort-origin correction. Natural added traces do not justify translating or reanchoring its primary diagnostic.
- Source inspection of these divergences is not bilateral measurement. No productive correction or public-contract gate is performed here.

## Evidence index for revised documents

Identifiers, timestamps and file:line references in this index are preserved provenance tokens from the pre-edit documents. They are not new metrics, a new line-count inventory or assertions that historical file offsets remain current. The document SHA identifies the original dirty content inspected. Short historical references remain qualified by the originating document; current owner mapping comes from @prompt, not those references.

### `00_nucleo/prompts/compiler/parse.md`

- Pre-edit document SHA-256: `8db314220fd8703ccbb7430872d7d84ddc2d4ce376ab5e91b774829aa278ef55`.
- Exclusive consumer: `01_core/src/compiler/parse/mod.rs`; inspected SHA-256: `433d532577ef4df7c91836551144585c99bbfcc353242a80d7d61743e029a666`.
- Preserved historical identifiers: `ffba4f0f6d7276a3beac03c1bd2bef40135d74681be616112a1e1f172d2f24b0`.

### `00_nucleo/prompts/compiler/parse/code.md`

- Pre-edit document SHA-256: `4225ba0077058fb012b17a4e5279db07f60bdeb3e323360be03acdda6f04e941`.
- Exclusive consumer: `01_core/src/compiler/parse/code.rs`; inspected SHA-256: `1e07e0815046bc82def55162836fd03c62f9b28bf52fac062bde4a79b6484ea2`.
- Preserved historical identifiers: `ffba4f0f6d7276a3beac03c1bd2bef40135d74681be616112a1e1f172d2f24b0`.

### `00_nucleo/prompts/compiler/parse/markup.md`

- Pre-edit document SHA-256: `324acc00977f9dfcf38484161a416f10a669abbee0bf22cead3959cf7f8fceb6`.
- Exclusive consumer: `01_core/src/compiler/parse/markup.rs`; inspected SHA-256: `3058d2adcfb758202f73ba113809e1263ca9800dac464120323b2d2cf9be69df`.
- Preserved historical identifiers: `ffba4f0f6d7276a3beac03c1bd2bef40135d74681be616112a1e1f172d2f24b0`, `P786a`, `P1137`.

### `00_nucleo/prompts/compiler/parse/math.md`

- Pre-edit document SHA-256: `6963387e4fac88e9a38494e51c3b63a962230828d78c07b50730bf268e61bc11`.
- Exclusive consumer: `01_core/src/compiler/parse/math.rs`; inspected SHA-256: `1b8fcb8a225beafd2ab6141e04f99a3739c24f7b4a27d652d6ecd861082020c8`.
- Preserved historical identifiers: `ffba4f0f6d7276a3beac03c1bd2bef40135d74681be616112a1e1f172d2f24b0`.

### `00_nucleo/prompts/compiler/parse/patterns.md`

- Pre-edit document SHA-256: `4d90982611aebfa3a04819fd5ca400ec3d5f0cea4587a660da51a9a284593c8f`.
- Exclusive consumer: `01_core/src/compiler/parse/patterns.rs`; inspected SHA-256: `4188a8a4ca9eb5a81fba3cc1e4c5775e940c61fc6308a4adf467dd04595eeeeb`.
- Preserved historical identifiers: `ffba4f0f6d7276a3beac03c1bd2bef40135d74681be616112a1e1f172d2f24b0`.

### `00_nucleo/prompts/compiler/parse/rules.md`

- Pre-edit document SHA-256: `906a09c77661785721c82f9027c617ed3d59847146d243971e281ecbc7021154`.
- Exclusive consumer: `01_core/src/compiler/parse/rules.rs`; inspected SHA-256: `e60b3d020dcaca8dd353999b81c5c7a9ca7679ddc71cdc1a1594f45236582c91`.
- Preserved historical identifiers: `ffba4f0f6d7276a3beac03c1bd2bef40135d74681be616112a1e1f172d2f24b0`.

### `00_nucleo/prompts/compiler/stdlib/_comum.md`

- Pre-edit document SHA-256: `5621b1dc7c80151e034f4f61c5586be1c81286ea87afda657581e3d7fd691d7a`.
- Exclusive consumer: `01_core/src/compiler/stdlib/mod.rs`; inspected SHA-256: `e27361f44641d3863b7464ba02cdd872a648b5136296bad96e445637f988e765`.
- Historical source references: `01_core/src/compiler/stdlib/mod.rs:5821-5834,13818-13867`, `01_core/src/compiler/stdlib/mod.rs:134-137`, `compiler/eval/mod.rs:1723-1725`, `mod.rs:2899`, `01_core/src/compiler/stdlib/mod.rs:24,76,211-214`, `compiler/stdlib/calc.rs:131`, `compiler/eval/call_dispatch.rs:455-487`, `stdlib/mod.rs:49,163-165`, `stdlib/mod.rs:49,94,167`, `01_core/src/compiler/stdlib/mod.rs:23-69`, `01_core/src/compiler/stdlib/mod.rs:231–14087`, `eval/mod.rs:2260–3144,`.
- Preserved historical identifiers: `P314`, `P430`, `P432`, `P433`, `P434`, `P435`, `P436`, `P437`, `P438`, `P1373`, `P1455`, `e5505f17ceb342b94424c2d1e5627cc523eb469a`, `P1396-R1`, `3f081c97f7dba59549a70e89d6bfb93afcc82b1a`, `P299`, `P1396`, `586e1bd43fae6c9a973218163d3165c53ab8d16d`, `P1215`, `P1289`, `P1292`, `P1307-R3`, `b303f1f15b610e09872b567027e0d806387fde8c`, `P1306`, `P1334`, `P1339`, `2f42d64253547734564513a1159ee6b584c1c4b4`, `P1402`, `P1403`, `P1404`, `P1405`, `P1461`, `7392f490397c56876fcf6134e86c44b528b396bb`, `P1466`, `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`.
- Original recorded dates/timestamps: `2026-09-26`.

### `00_nucleo/prompts/compiler/stdlib/assert.md`

- Pre-edit document SHA-256: `7276405eda528ee756a9bfd5bfc1729bf863cf80f4ca1b74c280eb4bc29b84cc`.
- Exclusive consumer: `01_core/src/compiler/stdlib/assert.rs`; inspected SHA-256: `e9748bb669ab452a3beeb41733d4cc3da5283a338480df1c80e0d20c95cbdd7f`.
- Historical source references: `foundations/mod.rs:198-247`, `shapes.typ:151,249,484,897,1387`, `anchor.typ:123,186`, `boolean.typ:189`, `foundations/mod.rs:179-181`, `shapes.typ:151`, `shapes.typ:249,484,897,1387`.
- Preserved historical identifiers: `P66`, `P723`, `P843`, `P513`, `P512`.

### `00_nucleo/prompts/compiler/stdlib/calc.md`

- Pre-edit document SHA-256: `14583963fdc6211fafc2c572c29760a445a3766e62ddc9345910221fe33bd768`.
- Exclusive consumer: `01_core/src/compiler/stdlib/calc.rs`; inspected SHA-256: `7f78e917e08892a49034ede3ecc7f62674da8cd856a0498e3b4b8c18c540fa8f`.
- Historical source references: `01_core/src/compiler/stdlib/calc.rs:139-148`, `lab/typst-original/crates/typst-library/src/foundations/calc.rs:74-94`, `01_core/src/compiler/eval/math.rs:329-348`, `entities/args.rs:19-20,60-84`, `eval/call_dispatch.rs:1031-1044`, `entities/value.rs:348`, `lab/typst-original/crates/typst-library/src/foundations/calc.rs:84-94`, `layout/length.rs:58-61`, `layout/angle.rs:76-78`, `layout/ratio.rs:103-105`, `layout/fr.rs:49-51`, `01_core/src/compiler/stdlib/calc.rs:139-161`, `entities/layout_types.rs:1102-1115,1162-1212,`, `entities/value.rs:113`, `lab/typst-original/crates/typst-utils/src/scalar.rs:29-31`, `01_core/src/compiler/stdlib/calc.rs:139-143`, `foundations/calc.rs:84-94,1363-1367`, `compiler/eval/call_dispatch.rs:1031-1044`, `01_core/src/compiler/stdlib/calc.rs:193-195`, `entities/value.rs:336-374`, `foundations/calc.rs:84-94`, `foundations/cast.rs:309-360`, `foundations/ty.rs:179-182`, `introspection/location.rs:52-54`, `compiler/stdlib/calc.rs:58`, `foundations/calc.rs:73-79`, `entities/func.rs:291-300`, `eval/call_dispatch.rs:1648`, `eval/repr.rs:87`, `entities/func.rs:390,404-406`, `stdlib/gradients.rs:649`, `eval/rules.rs:2488`, `calc.rs:506-515`, `lab/typst-original/crates/typst-library/src/foundations/args.rs:109-120,150-173`, `foundations/calc.rs:73-94`, `calc.rs:131-210`, `compiler/eval/call_dispatch.rs:398,451`, `lab/typst-original/crates/typst-library/src/foundations/args.rs:112-120,150-173,259-266`, `01_core/src/compiler/stdlib/calc.rs:206-224`, `compiler/eval/call_dispatch.rs:398,455-487`, `calc.rs:87-91`, `calc.rs:951-994`, `lab/parity/corpus/p500/test-calc-rest.typ:19-21`.
- Preserved historical identifiers: `P314`, `P283`, `P1328`, `d31047d7b8af7837c84adae4ded3d2ff50c62093`, `P1327`, `P1330`, `P1329`, `P1329-R2`, `P1331`, `P1332`, `P433`, `P817-A`, `P817-F`, `P27`, `P817-C`, `P817-D`, `P306`, `P817-B`, `P817`, `P308`, `P1333`, `P1334`, `P1488`, `P817-E`, `81cbcb240620cfccac5677cbab236b9609cb7209252109f8ca63ba2b64275fa2`.
- Original recorded dates/timestamps: `2026-09-09T11:35:18.901045+00:00`, `2026-09-09T12:08:08.824769+00:00`, `2026-09-09T12:31:40.046287+00:00`.

### `00_nucleo/prompts/compiler/stdlib/collections.md`

- Pre-edit document SHA-256: `b21f79dd45bf7f5ff24998ebb20df1004622e6ba784517300326ac931841907c`.
- Exclusive consumer: `01_core/src/compiler/stdlib/collections.rs`; inspected SHA-256: `bb52620f835ca8c561d7277ca596c2de91fb6b95ba3003e11d10d509e461d383`.
- Historical source references: `foundations/array.rs:698-714`, `foundations/str.rs:275-280`, `foundations/array.rs:754-786`, `foundations/array.rs:207-221`, `aabb.typ:43,45,75-77`, `foundations/array.rs:279-300`, `array.rs:122-137`, `draw/shapes.typ:620,624,630,971,973,1949,2160`, `anchor.typ:218`, `coordinate.typ:203`, `drawable.typ:145`, `01_core/src/compiler/stdlib/collections.rs:545-549`, `lab/typst-original/crates/typst-library/src/foundations/args.rs:414-445`, `foundations/args.rs:408–412`, `typst-eval/src/call.rs:638`.
- Preserved historical identifiers: `P466`, `P1284`, `P1142`, `P690`, `P691`, `P692`, `P693`, `P714`, `P730`, `P1214`, `P1215`, `a3e72f35b3fc0f5cd765d0dc0ca89990ff1314a5`, `P843`, `P728`, `P495`, `P729`, `P689`, `P459`, `P662-P664`, `P881`, `P493`, `P1284-v2`, `P1307-R3`, `b303f1f15b610e09872b567027e0d806387fde8c`, `P1306`, `P1308`.
- Original recorded dates/timestamps: `2026-06-25`, `2026-08-30`, `2026-08-24`.

### `00_nucleo/prompts/compiler/stdlib/color.md`

- Pre-edit document SHA-256: `d2513fa134354389f8c23d331b04d64ee45e41a43027a27c30b2ac9b58037aa3`.
- Exclusive consumer: `01_core/src/compiler/stdlib/color.rs`; inspected SHA-256: `22642827e17aa35439a4e61ef802887431671077f50a448e195fc86aafda4315`.
- Historical source references: `lab/typst-original/crates/typst-library/src/lib.rs:397-401`, `visualize/color.rs:311`, `visualize/color.rs:1058-1090,1136-1204,2422-2459`, `stdlib/color.rs:50-84`, `lab/typst-original/crates/typst-library/src/visualize/color.rs:2260-2420`, `entities/color.rs:32-`, `visualize/color.rs:889-1020,1147-1245,2254-2397`, `stdlib/color.rs:118-138,743-808`, `visualize/color.rs:2270-2297`, `visualize/color.rs:2270-2292`.
- Preserved historical identifiers: `P736`, `P685`, `P476`, `P477`, `P742`, `P744`, `P1143`, `P1300`, `1f082370e59939de7b57992e137a9f74bfb6758f`, `P842`, `P1286`, `P492`, `P497`, `P687`, `P1253`, `P506`, `P1284`, `7909421397bc2bef00faf5a1064d7356ad4cab112ec1fd5e290b124a13529a3e`, `dece34103d5266311558bb44f96f02df7090edf8977744fc9a1213b2213b2f79`, `74fe385a692d3da43ff8afc8f61896f76bc87a1a766f05f4897f5e8164db4ed7`, `1c62ea2e765ddf6d6b1c5189e772605e0203d6116bf384597df0cfa4cdab5f17`, `3b9d02b0685ec2ae62958a287c4cdd68fdb08f642339aabaa8d96e60aeaf999b`, `20de1b5440e58c3727d645e6f8444d9e6d1c73fb09f9e905a49c314dc58496da`, `c634397325d83383609535f10e2d964efd899be34a2cef337379a5368b09fa79`, `f0ddd8a6d9e12c3c3b5d705dfa2e7c5f7929ce242c8d94ab5e4a0f6b2e0d9f2d`, `b3d6e7d7762c8e4d3b27d86aa41391cf8d5093df122b328bc6482f2d37a1497b`, `f74edb89a1207308a4534b6648d32f54439e0057fb5b4cb0346151610131da77`, `13161dddf6ad860268eece02ce446b1025d3ab95871cc1a484a93aa05da4aed2`, `d015c8a4cb87aaebc49d0416edf430e5c76392e772602ca1234ce6fe748db81d`, `0de0d1c2a9dd6a8a03cd3155b872ea4a2eeaa74a5454df8f511632aa5d91e55f`, `03f70bedc78f4fb3e22b3f7ed06a680d56d721321eca69218ab746d4c803f595`, `660b6381c0f53156c07b4f0f86988a280c06578f46a44b734c1f388cadeaeb98`, `P1284-v4`, `P1284-v2`, `06e205483a9bc905f778a72fcbb74ddb229b73141ffe6bf27ec95d0a40e145c3`, `50b6e34b826041edb61f15828a14c2cfd234bc7bee71215c378e89e68079ba79`, `P1473`, `P1482`, `P1483`.
- Original recorded dates/timestamps: `2026-09-03T19:01:51.133010-03:00`, `2026-09-21`, `2026-09-23`.

### `00_nucleo/prompts/compiler/stdlib/context.md`

- Pre-edit document SHA-256: `3a5c48d0978c092f2679283475648a6760b02ef111933a94ccf0d3c5532e7403`.
- Exclusive consumer: `01_core/src/compiler/stdlib/context.rs`; inspected SHA-256: `97b29c2de2c958fb3585b796f11090270ee7002c152578c551d85a6406cb12e3`.
- Preserved historical identifiers: `0ede5ade7908670f0440672becaba2aa356acc51346fd590dc426e4788dc6dd0`, `P500`, `P506`.

### `00_nucleo/prompts/compiler/stdlib/emoji.md`

- Pre-edit document SHA-256: `be61d2d9e7995376939a983705a4e2eadc60666e8d04fc63933a3cd743184812`.
- Exclusive consumer: `01_core/src/compiler/stdlib/emoji.rs`; inspected SHA-256: `46068082b902dbb5a427bc8e447b7c24c8b76001c7e4b55ed67d78dc151b34f9`.
- Preserved historical identifiers: `P731`, `P735`, `P1283`, `P1496`, `2e43ab90079cf9f9fa670d421c8c38233143b376`, `P1161`, `P1162`, `0732ab1a27b4ea05e6f9f60a5122c9924dd5123defde0d8e907f58cf643d40e6`, `8691ca68e09b6fedca61e00e824648e79f7502772eefe7af367e404e26161489`, `P1495`.
- Original recorded dates/timestamps: `2026-08-30`.

### `00_nucleo/prompts/compiler/stdlib/eval.md`

- Pre-edit document SHA-256: `8d8bd21eadc03c12747b20f71443799c75823f8e1641e5284c5f365c48ce81af`.
- Exclusive consumer: `01_core/src/compiler/stdlib/eval.rs`; inspected SHA-256: `169e39150d333a2978cb008857f414c5de088a83747b3e7db5b8f85c9d5f9523`.
- Historical source references: `foundations/mod.rs:279`, `foundations/mod.rs:267,318`, `lab/typst-original/crates/typst-eval/src/lib.rs:151`.
- Preserved historical identifiers: `P814`, `P810`, `P815`, `P819`, `P830`, `P846`, `P831`, `P772s`, `P829`, `P394`.
- Original recorded dates/timestamps: `2026-07-22`.

### `00_nucleo/prompts/compiler/stdlib/foundations.md`

- Pre-edit document SHA-256: `8df1e1b9317b6eeb11c691ed2126bd76901c8e2c207278172ce2e79984b0bc34`.
- Exclusive consumer: `01_core/src/compiler/stdlib/foundations/mod.rs`; inspected SHA-256: `f94a2d4523e06893bda780c9fb400736e2686499cac176d747e4df48509d08c0`.
- Preserved historical identifiers: `P1140.1-A`, `P1077`, `P1031`, `P1146`, `P1289`.

### `00_nucleo/prompts/compiler/stdlib/foundations/cast.md`

- Pre-edit document SHA-256: `3a53ca7240b25e019e3dfe817ae73a5f7ff8b35653e1bc131a096614dbd5f87b`.
- Exclusive consumer: `01_core/src/compiler/stdlib/foundations/cast.rs`; inspected SHA-256: `10c8065c4f8de26caf69464d98e03b3b1f27b2c71e63a2eec2e4dfaf24f0523f`.

### `00_nucleo/prompts/compiler/stdlib/foundations/datetime.md`

- Pre-edit document SHA-256: `9ff700fbdfec93720f218960578b2e40f40197ec2c328680a75218110587fbe3`.
- Exclusive consumer: `01_core/src/compiler/stdlib/foundations/datetime.rs`; inspected SHA-256: `f1b931f526b298d8fdd632cbdc488bb7a9432299f2af47d39dcd02ebb1c4d4f2`.
- Historical source references: `a51e02804:crates/typst-library/src/foundations/datetime.rs:241-499`.
- Preserved historical identifiers: `P1146`, `c65c839ff5544a66683b7e81edba923acb49760e`, `P1145`.
- Original recorded dates/timestamps: `2026-08-24T19:58:15-03:00`, `2024-02-29`, `2023-12-31`.

### `00_nucleo/prompts/compiler/stdlib/foundations/float.md`

- Pre-edit document SHA-256: `9d3986087736d54c0e40f0bc65c00350219d494149b9d409d5b950dab7b1e5cb`.
- Exclusive consumer: `01_core/src/compiler/stdlib/foundations/float.rs`; inspected SHA-256: `253555933d4d68445ab3277b2ad2dfb54e6adad3e4d01f9a0f0c0b3d94824ac4`.
- Historical source references: `lab/typst-original/crates/typst-library/src/foundations/float.rs:81-94`, `01_core/src/compiler/stdlib/foundations/float.rs:120,124`, `lab/typst-original/crates/typst-library/src/foundations/float.rs:14`, `01_core/src/compiler/stdlib/foundations/float.rs:97–108`.
- Preserved historical identifiers: `P1289`, `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`, `P1307-R3`, `b303f1f15b610e09872b567027e0d806387fde8c`, `P1306`, `P1293.reopen-A`, `P1339`, `2f42d64253547734564513a1159ee6b584c1c4b4`.
- Original recorded dates/timestamps: `2026-08-31T10:35:47-03:00`.

### `00_nucleo/prompts/compiler/stdlib/foundations/int.md`

- Pre-edit document SHA-256: `38fbf55d0ebb4dea76268c64896fc3fa59f58c9e02bcbb6a947ecd2e90dc05a3`.
- Exclusive consumer: `01_core/src/compiler/stdlib/foundations/int.rs`; inspected SHA-256: `a5bef7723a233af45bc9bda8f7e43f3017db8ce6c3c8bc41d18eb2720401b80c`.
- Historical source references: `a51e02804:foundations/int.rs:68-428`, `01_core/src/compiler/stdlib/foundations/int.rs:149-153,264-268`.
- Preserved historical identifiers: `P1147`, `P1307-R3`, `b50e726c5830c0f91a6875d2d0a758903bc93b0bd719f4b5357828522a999293`, `b303f1f15b610e09872b567027e0d806387fde8c`, `P1306`.

### `00_nucleo/prompts/compiler/stdlib/foundations/path.md`

- Pre-edit document SHA-256: `7800a99382c0bf7e8821d8c3042c4a2a0e3d4b68bf069c33aaa102b622d9aa8a`.
- Exclusive consumer: `01_core/src/compiler/stdlib/foundations/path.rs`; inspected SHA-256: `98297baf51bc64cfcc80b8e569846b34eda6944febbb6e534908afd9fc1a7f9e`.
- Preserved historical identifiers: `P1141`, `P1154`.

### `00_nucleo/prompts/compiler/stdlib/foundations/query.md`

- Pre-edit document SHA-256: `379c3b809dc2021c26eab4f8ddf4220d6c391080c89a5eec760e02cab8c9dcf4`.
- Exclusive consumer: `01_core/src/compiler/stdlib/foundations/query.rs`; inspected SHA-256: `b23e8fe39ebd020dc62978bce5e4621362656ca3b988ae9832decf66652d1f5d`.
- Historical source references: `compiler/stdlib/foundations/query.rs:46–65`, `compiler/stdlib/foundations/query.rs:52-64,76-80`, `introspection/query.rs:160-175`, `query.rs:53-65`.
- Preserved historical identifiers: `5a0270231de70be1212dbd17298cce34b7161b527d74b4b589e3f4c69d35ce24`, `P1307-R5`, `b303f1f15b610e09872b567027e0d806387fde8c`, `P1151`, `P844`, `P821`, `P1140.1-A`, `P1339`, `2f42d64253547734564513a1159ee6b584c1c4b4`.

### `00_nucleo/prompts/compiler/stdlib/foundations/regex.md`

- Pre-edit document SHA-256: `60df8cddb5073de4e13be958e50a766744a1dcc10d24406d2ca46c0dd6ef15e9`.
- Exclusive consumer: `01_core/src/compiler/stdlib/foundations/regex_constructor.rs`; inspected SHA-256: `85850de670f08d750c393f23cb7fb66e9906fde1afc59e82c0ba077d94a8ca2b`.
- Preserved historical identifiers: `P1140.1-A`, `P1140.1-B`, `P1140`.
- Original recorded dates/timestamps: `2026-08-23`.

### `00_nucleo/prompts/compiler/stdlib/foundations/repr.md`

- Pre-edit document SHA-256: `f5a0f6b36bc9dec7584d641a8db1b95e68278018e0d3c08016cad83c0556dfd5`.
- Exclusive consumer: `01_core/src/compiler/stdlib/foundations/repr.rs`; inspected SHA-256: `7be5188362e805a57eab5cfbbbef978e2934e4122f1c9e6fdfc5abd550059e8f`.
- Preserved historical identifiers: `P1163`, `P1140.9`, `P1140.10`, `P1140.17`, `P1140.26`.
- Original recorded dates/timestamps: `2026-08-24`, `2026-08-25`.

### `00_nucleo/prompts/compiler/stdlib/foundations/selector.md`

- Pre-edit document SHA-256: `69a71b1cff06bade43abcc6a80a1cde93a04566c18eb7ccc67159642c0e8998d`.
- Exclusive consumer: `01_core/src/compiler/stdlib/foundations/selector.rs`; inspected SHA-256: `8dff98a6ce2d7f150b57ef4200e874749060c40402bbf94f46d34434e9e464de`.
- Historical source references: `selector.rs:31`, `foundations/selector.rs:167-174`.
- Preserved historical identifiers: `P1140.1-A`, `P1140.1-B`, `P1140`, `P1285`, `P1339`, `2f42d64253547734564513a1159ee6b584c1c4b4`, `P1402`.
- Original recorded dates/timestamps: `2026-08-23`, `2026-09-10T00:18:55.842594+00:00`.

### `00_nucleo/prompts/compiler/stdlib/foundations/str.md`

- Pre-edit document SHA-256: `4b3fa3d5cb2c313fa13326dc8c398f70ef89efb229a6a8ae8b37c1e77bca0043`.
- Exclusive consumer: `01_core/src/compiler/stdlib/foundations/str.rs`; inspected SHA-256: `89187857de7aa3a13f89136f616f6d744dad5edaaadb81bdce93dcb465172747`.
- Historical source references: `a51e02804:foundations/str.rs:309-321`, `foundations/str.rs:50-79`.
- Preserved historical identifiers: `P1140.1-A`, `P1140.2`, `P1162`.

### `00_nucleo/prompts/compiler/stdlib/foundations/ty.md`

- Pre-edit document SHA-256: `367e7d272288babe6980511dfea985eec5f553fbe6b925b7ba3bc0a956cdee83`.
- Exclusive consumer: `01_core/src/compiler/stdlib/foundations/ty.rs`; inspected SHA-256: `37a438c4d5db4ea4530b3fb11696c4970b8036073cb52236965c3c6e177c79a9`.
- Preserved historical identifiers: `P685`.

### `00_nucleo/prompts/compiler/stdlib/html/p1423-tests.md`

- Pre-edit document SHA-256: `88895f723f2b92561f6f9bcea58bff26f5867a37a461950625172c76d29c35f0`.
- Exclusive consumer: `01_core/src/compiler/stdlib/html/p1423_tests.rs`; inspected SHA-256: `ecdb2b11f62425905784524fe7768a654a9ba2e9a573c8cd24063e1c2e2a7a0c`.
- Preserved historical identifiers: `P1423`, `P1424`.

### `00_nucleo/prompts/compiler/stdlib/html/p1424-tests.md`

- Pre-edit document SHA-256: `749bdc9378515d755bbdd61209817b49dac569810386891d2b5faff26f05cd07`.
- Exclusive consumer: `01_core/src/compiler/stdlib/html/p1424_tests.rs`; inspected SHA-256: `f04db8a77361008d80a92b5296f0c647d19d4da1b7cf7aad52ff4dc383895c6b`.
- Preserved historical identifiers: `P1424`.

### `00_nucleo/prompts/compiler/stdlib/label.md`

- Pre-edit document SHA-256: `955eacd5e1587f3d368b3f686514ed574ad21b298386ce28e3204f612920edd6`.
- Exclusive consumer: `01_core/src/compiler/stdlib/label.rs`; inspected SHA-256: `fed54495da9419821603b515e74798f570b8a4cb11bdbce0570bd62422568a37`.
- Historical source references: `foundations/label.rs:74-89`.
- Preserved historical identifiers: `P460`, `P1140.2`.
- Original recorded dates/timestamps: `2026-08-23`.

### `00_nucleo/prompts/compiler/stdlib/numbering.md`

- Pre-edit document SHA-256: `a7d9378866b5a6ede1a7927b0c2eea3903640b4b3adfe38faa7e9e9183ea9b44`.
- Exclusive consumer: `01_core/src/compiler/stdlib/numbering.rs`; inspected SHA-256: `7a529cafa2ab35444e04e850768c893276dbd1fcdd8f56d451cceccb7e128dc0`.
- Historical source references: `model/numbering.rs:99-128`, `01_core/src/compiler/stdlib/numbering.rs:36-40`.
- Preserved historical identifiers: `P847`, `P793`, `P844`, `P1157`, `P1159`, `P1307-R3`, `b50e726c5830c0f91a6875d2d0a758903bc93b0bd719f4b5357828522a999293`, `b303f1f15b610e09872b567027e0d806387fde8c`, `P1306`.

### `00_nucleo/prompts/compiler/stdlib/panic.md`

- Pre-edit document SHA-256: `33a6255bed58d66d2516ebd4912bc6c9129ee729aee118af465f611e9c3751e3`.
- Exclusive consumer: `01_core/src/compiler/stdlib/panic.rs`; inspected SHA-256: `ad04224f3276e414433fd71f50db19430e7b6352327d706b4327f3bf3aa822bb`.
- Historical source references: `foundations/mod.rs:140-152`, `foundations/mod.rs:140–156`.
- Preserved historical identifiers: `P843`, `P1308`, `62c53690cbe3dd36b5b79168ea59a32f6eb88cc52ea52a8ca97365c5a9459394`, `ce758b5c2a18288bf9c8433178f577b50c52df80cedcddcb1f4daa4e785573fc`.

### `00_nucleo/prompts/compiler/stdlib/primitives-constructors.md`

- Pre-edit document SHA-256: `7f956550de9b0a114a5141c524000af8e1bdc03d491c7f8d52d9bc9dc3a7de53`.
- Exclusive consumer: `01_core/src/compiler/stdlib/primitives_constructors.rs`; inspected SHA-256: `36d64525d2f41017cb5df3cab0d613cca42b6d7318878aaef6fdc402d65f2924`.
- Historical source references: `primitives_constructors.rs:11,15`, `eval/bindings/value_methods.rs:623-662`, `primitives_constructors.rs:9-16`.
- Preserved historical identifiers: `761d5adeca09f6a60ba2960f8492aa6c0d934798bbc826f341004cf993ba8736`, `P1200`, `P1339`, `2f42d64253547734564513a1159ee6b584c1c4b4`.

### `00_nucleo/prompts/compiler/stdlib/primitives-constructors/array.md`

- Pre-edit document SHA-256: `b6d04be19c1e4b3a64a8c898fb2d27518996629c9e03dbb04db8eb7befde2f2a`.
- Exclusive consumer: `01_core/src/compiler/stdlib/primitives_constructors/array.rs`; inspected SHA-256: `3530fd36135f4f97f2b91a2f74e9d8d3d53cf93d3834c300fe7a4e7b43e2d076`.
- Historical source references: `compiler/eval/call_dispatch.rs:1791-1830`, `foundations/array.rs:164-169,1176-1179`.
- Preserved historical identifiers: `761d5adeca09f6a60ba2960f8492aa6c0d934798bbc826f341004cf993ba8736`, `P1339`, `2f42d64253547734564513a1159ee6b584c1c4b4`, `P1353`.

### `00_nucleo/prompts/compiler/stdlib/primitives-constructors/decimal.md`

- Pre-edit document SHA-256: `de1b68891b7ac68581e426070dcf58fff02f03323401127fea37df172f39b54a`.
- Exclusive consumer: `01_core/src/compiler/stdlib/primitives_constructors/decimal.rs`; inspected SHA-256: `dcc2294d6434fbd975dd6e7dfe5ce1e6575b397e8aa334f3cbce96b3c2914104`.
- Preserved historical identifiers: `761d5adeca09f6a60ba2960f8492aa6c0d934798bbc826f341004cf993ba8736`, `P1200`.

### `00_nucleo/prompts/compiler/stdlib/primitives-constructors/duration.md`

- Pre-edit document SHA-256: `0b1ddf38bb6a4264e13e32c48412186de96bb2e375f49d7440be10cc15ac719c`.
- Exclusive consumer: `01_core/src/compiler/stdlib/primitives_constructors/duration.rs`; inspected SHA-256: `b432c1ab2ba91d5a55201bf23f5648857a937036a06d5f984bc890194a4a559a`.
- Preserved historical identifiers: `761d5adeca09f6a60ba2960f8492aa6c0d934798bbc826f341004cf993ba8736`, `P1200`, `P1466`.

### `00_nucleo/prompts/compiler/stdlib/primitives-constructors/version.md`

- Pre-edit document SHA-256: `3270d7faf57798c77e8e812f068e7c1b8cc05bfbe858be9989e14245c493bbce`.
- Exclusive consumer: `01_core/src/compiler/stdlib/primitives_constructors/version.rs`; inspected SHA-256: `fb7eccf53321020b7eeb2a71824b7d2baaae83ba903ed889dbdcd9ae67ae280e`.
- Historical source references: `lab/typst-original/crates/typst-library/src/foundations/version.rs:109–134`, `01_core/src/entities/version.rs:103–120`, `01_core/src/compiler/eval/bindings/value_methods.rs:623–661`.
- Preserved historical identifiers: `761d5adeca09f6a60ba2960f8492aa6c0d934798bbc826f341004cf993ba8736`, `P1200`, `P1339`, `2f42d64253547734564513a1159ee6b584c1c4b4`.

### `00_nucleo/prompts/compiler/stdlib/ref.md`

- Pre-edit document SHA-256: `0f8b9790f8f5fffe7c4602d4d51617c62c60238358516a00d8a4e1e0be33976d`.
- Exclusive consumer: `01_core/src/compiler/stdlib/ref.rs`; inspected SHA-256: `1d4f6ff898c895ad0d6373302c065fe1bae7bfe4d766ae731bca701b664c5a36`.
- Preserved historical identifiers: `P462`, `P1140.25`.

### `00_nucleo/prompts/compiler/stdlib/structural.md`

- Pre-edit document SHA-256: `772e0e38862608f4674ac6e4581e1ab47f68b578a2ed980a1a65a87ee5f3c262`.
- Exclusive consumer: `01_core/src/compiler/stdlib/structural/mod.rs`; inspected SHA-256: `67223c44f067bf1d036a3dc9b52260c6f1f5d893a2085ad774202d2ff2eb089f`.
- Historical source references: `foundations/cast.rs:502-520`, `canvas.typ:111,129`, `structural.rs:675-689`, `structural.rs:694`, `structural.rs:875-882,`, `typst-library/src/model/table.rs:268-270`, `typst-library/src/visualize/stroke.rs:654-665`, `typst-library/src/layout/grid/mod.rs:408-409`, `math/mod.rs:36-40,98-102`, `math/op.rs:52-56`, `typst-bundle/src/format.rs:194-240`.
- Preserved historical identifiers: `P69`, `P101`, `P154B`, `P155`, `P157A`, `P159A`, `P224`, `P295`, `P296`, `P296.1`, `P296.2`, `P397`, `P418`, `P419`, `P420`, `P429`, `P1140.3-A`, `P1140.3-B`, `P1140.4`, `P606`, `P829`, `P829-B`, `P864`, `P806`, `P798`, `P373`, `P227`, `P228`, `P772v`, `P772i`, `P493b`, `P496`, `P887`, `P885`, `P230`, `P235`, `P224.C`, `P157C`, `P450`, `P479`, `P468`, `P470`, `P504`, `P505`, `P472`, `P430`, `P480`, `P772y`, `P825`, `P810`, `P471`, `P731`, `P726`, `P844`, `P831`, `P847`, `P895`, `P795`, `P962`, `P809`, `P812`, `P1132m`, `P1402`, `P1403`, `P1404`, `P1405`, `P1485`, `P1484`.
- Original recorded dates/timestamps: `2026-08-04`, `2026-08-22`, `2026-08-13`.

### `00_nucleo/prompts/compiler/stdlib/structural/bibliography.md`

- Pre-edit document SHA-256: `82a4b50f564cb5943de7c36bfce7b4db3fc12b7a14cac580a9f63ddd81dd7b13`.
- Exclusive consumer: `01_core/src/compiler/stdlib/structural/bibliography.rs`; inspected SHA-256: `ae304854277294a4d7e6cbc3a227ea2af7b719041bb02fa2a4940f1e46883c02`.
- Preserved historical identifiers: `P69`, `P962`, `P159a`, `P159A`, `P159D`, `P159C`, `P468`, `P1034`, `P1031`, `P1141`.
- Original recorded dates/timestamps: `2026-08-13`.

### `00_nucleo/prompts/compiler/stdlib/structural/divider.md`

- Pre-edit document SHA-256: `d8766625c286caa057db78e5900ee4f13800299173d026bb66fafd487b4dd13b`.
- Exclusive consumer: `01_core/src/compiler/stdlib/structural/divider.rs`; inspected SHA-256: `82d2c0bed5d2f75c214fbcbc8757d36e4b822d56b533c0343c181f3ab13ebe30`.
- Original recorded dates/timestamps: `2026-08-13`.

### `00_nucleo/prompts/compiler/stdlib/structural/document.md`

- Pre-edit document SHA-256: `3395831455ce3582196999d014afc89ecce5d1e42b1ec0a3b3e15e4d411c131a`.
- Exclusive consumer: `01_core/src/compiler/stdlib/structural/document.rs`; inspected SHA-256: `46c23621c8c85b5d72c3df12a48e3475838ef1fb439882ad9376d3529a486133`.
- Historical source references: `01_core/src/compiler/stdlib/structural/document.rs:76-110`, `lab/typst-original/crates/typst-bundle/src/format.rs:162-240`, `typst-bundle/src/format.rs:194-240`, `typst-library/src/model/document.rs:147-239`.
- Preserved historical identifiers: `P69`, `P962`, `P1484`, `bde47c744e0bffb6229005e4b5f04c2ed0e9ef08`, `P397`, `P1485`.

### `00_nucleo/prompts/compiler/stdlib/structural/heading.md`

- Pre-edit document SHA-256: `380d9dd239e487daa00c982438bf592ff391c979d51992e8634504cd40cf606b`.
- Exclusive consumer: `01_core/src/compiler/stdlib/structural/heading.rs`; inspected SHA-256: `30312d058d9d8170a14b4b71bef464ffdb9f51d794a729b8d75d1bbdcc807bd7`.
- Original recorded dates/timestamps: `2026-08-13`.

### `00_nucleo/prompts/compiler/stdlib/structural/markup.md`

- Pre-edit document SHA-256: `4983d15545df3edcb29826da03fa6653693c03dcca9a90fae62c647d9349399c`.
- Exclusive consumer: `01_core/src/compiler/stdlib/structural/markup.rs`; inspected SHA-256: `dd7293909aec9bf3bb6a7f26f3d311943abf22ee74894a2b8c5c2604ce7ebf18`.
- Preserved historical identifiers: `P69`, `P962`, `P101`, `P109`, `P96.5`, `P97-99`, `P422`.

### `00_nucleo/prompts/compiler/stdlib/structural/math/p1425-tests.md`

- Pre-edit document SHA-256: `5580ff5a33c322b795c722c22335601065a2555c1c5f50522a5bdde1b18a969a`.
- Exclusive consumer: `01_core/src/compiler/stdlib/structural/math/p1425_tests.rs`; inspected SHA-256: `b7843f3f98c0ffb55c67f92075d2b0f5c5732f4de2fc2dfdd3a85cdc9cfc9182`.
- Preserved historical identifiers: `P1425`.

### `00_nucleo/prompts/compiler/stdlib/structural/math/p1426-tests.md`

- Pre-edit document SHA-256: `9e837c6793f9e48267ac9c8b67e6dc2400f1e786299b42fccbba72b9caa7bf25`.
- Exclusive consumer: `01_core/src/compiler/stdlib/structural/math/p1426_tests.rs`; inspected SHA-256: `c6e84a327779528b218af1a001a8303888f545d7203bfd7c053053398ab35459`.
- Preserved historical identifiers: `P1426`.

### `00_nucleo/prompts/compiler/stdlib/structural/math/p1427-tests.md`

- Pre-edit document SHA-256: `0064ba61d99129644ffbb01645bcaa5117ff9a9eed5fb17d3f246e7cf31cb361`.
- Exclusive consumer: `01_core/src/compiler/stdlib/structural/math/p1427_tests.rs`; inspected SHA-256: `415a9d339e964814b66acea7b02ec6965fbcd72ddb82c33b1e2f733e964aff7f`.
- Preserved historical identifiers: `P1427`, `P01`, `P07`, `P16`.

### `00_nucleo/prompts/compiler/stdlib/structural/p1405_outline_indented_tests.md`

- Pre-edit document SHA-256: `536d6c06e118e0c7bb672228dd13543343b438d046b317407fa4234db8571137`.
- Exclusive consumer: `01_core/tests/p1405_outline_indented.rs`; inspected SHA-256: `1068e0a752e40fcd5e93d08ed9eee1e044a8cfddd26a3ee2f5851e8167a6ca65`.
- Preserved historical identifiers: `P1405`, `P1406`.

### `00_nucleo/prompts/compiler/stdlib/structural/p1429-tests.md`

- Pre-edit document SHA-256: `25df72cf24adf5731736f6b8788281c499146e5b8807ad660f602813da0373b0`.
- Exclusive consumer: `01_core/src/compiler/stdlib/structural/p1429_tests.rs`; inspected SHA-256: `0d392c20f1d1ba5756a26185ef1bdefb361638d002f8c2f5a92e6593be1c386f`.
- Preserved historical identifiers: `P1429`.

### `00_nucleo/prompts/compiler/stdlib/structural/p1430-tests.md`

- Pre-edit document SHA-256: `3213c36cee30e6cbfb4937ff5026a2090d36abd07620ee407feaf6e728ebd4d5`.
- Exclusive consumer: `01_core/src/compiler/stdlib/structural/p1430_tests.rs`; inspected SHA-256: `f3c62a0582930cf8eabf07e1fb7e7ad64529ea277b96ba19480de4dfde450dc6`.
- Preserved historical identifiers: `P1430`.

### `00_nucleo/prompts/compiler/stdlib/structural/p1431-tests.md`

- Pre-edit document SHA-256: `0cdac7ff4af8aac2ff38d20cdc76b6389f501200a1c9ae86d2f5ec64752e60da`.
- Exclusive consumer: `01_core/src/compiler/stdlib/structural/p1431_tests.rs`; inspected SHA-256: `afa2e6f3b426bb0406c05d34ad14f59d3352b46955f24602f385a8e4ae9ed3c0`.
- Preserved historical identifiers: `P1431`.

### `00_nucleo/prompts/compiler/stdlib/structural/p1436-tests.md`

- Pre-edit document SHA-256: `d74cde7e15f96af0431704c4f2d8a06717b6f7b658bcd0ec8571ff423b337570`.
- Exclusive consumer: `01_core/src/compiler/stdlib/structural/p1436_tests.rs`; inspected SHA-256: `9284e0b807cfe9b521e9e3a562ff2449b6f2d3fc1d56cfcbade5986fa67468f9`.
- Preserved historical identifiers: `P1436`.

### `00_nucleo/prompts/compiler/stdlib/structural/p1437-tests.md`

- Pre-edit document SHA-256: `e90cafd03da37aa36376f993e66c1f1ea964d9fcdc774969e0787d81ad3089f9`.
- Exclusive consumer: `01_core/src/compiler/stdlib/structural/p1437_tests.rs`; inspected SHA-256: `dda352cbccfaea72b07647115648ed5d38a3163127a7f72f6f74df8f1f15aa6f`.
- Preserved historical identifiers: `P1437`.

### `00_nucleo/prompts/compiler/stdlib/structural/p1438-tests.md`

- Pre-edit document SHA-256: `fee8379c4e1edf4802affa204a87e65d07c4340627f9b8776123af677955514e`.
- Exclusive consumer: `01_core/src/compiler/stdlib/structural/p1438_tests.rs`; inspected SHA-256: `3c2391bed6d9d42a74e5e8d76e08618acf408532ee4b9addedfcb29fe8f33a84`.
- Preserved historical identifiers: `P1438`, `P1436`.

### `00_nucleo/prompts/compiler/stdlib/structural/p1439-tests.md`

- Pre-edit document SHA-256: `ada939546fcb3745b154ac633ea83c19e06d86bad14e1edc9b72ad598bf137f0`.
- Exclusive consumer: `01_core/src/compiler/stdlib/structural/p1439_tests.rs`; inspected SHA-256: `9d5bcb794b15f80597c8a382541ac5f20813f7ac48c17cc740bed5baaf9596ef`.
- Preserved historical identifiers: `P1439`.

### `00_nucleo/prompts/compiler/stdlib/structural/p1440-tests.md`

- Pre-edit document SHA-256: `8fbf9fbc92d75b8ce27b4804b7bbbb09a6e0d3cdd28ea48a8b65ee3102004d4e`.
- Exclusive consumer: `01_core/src/compiler/stdlib/structural/p1440_tests.rs`; inspected SHA-256: `4ca547fda19637abdeb1efd13fd3a72ce93dcc0b74a45852c35d6d732bc9cb84`.
- Preserved historical identifiers: `P1440`.

### `00_nucleo/prompts/compiler/stdlib/structural/p1441-tests.md`

- Pre-edit document SHA-256: `925c5401d770fd633f0dc986e1ad696e90466acfe8dae40651f3f1b05ff5ce67`.
- Exclusive consumer: `01_core/src/compiler/stdlib/structural/p1441_tests.rs`; inspected SHA-256: `5981b3cfc6c3104f49a40cff9a15c29e15911453d88b11dc3182f702ac7654a0`.
- Preserved historical identifiers: `P1441`.

### `00_nucleo/prompts/compiler/stdlib/structural/p1442-tests.md`

- Pre-edit document SHA-256: `14877b606d2a15ed82dca2f0927fdc6777825048f2a4bba3812a2cd4c07a2e2a`.
- Exclusive consumer: `01_core/src/compiler/stdlib/structural/p1442_tests.rs`; inspected SHA-256: `5303a67b4179f5fe73e19a2c3f9ae5b6055c086f8391667016feaa093c651ff8`.
- Preserved historical identifiers: `P1442`.

### `00_nucleo/prompts/compiler/stdlib/structural/p1443-tests.md`

- Pre-edit document SHA-256: `937ccb63d010c4ac5ba49e8d98315eef8c9a39577644ccc20d804b93942f8fae`.
- Exclusive consumer: `01_core/src/compiler/stdlib/structural/p1443_tests.rs`; inspected SHA-256: `3e889a2d3f21712d5a17d00ee2b709b544423a94f7cd345a4e2994a41bb95a78`.
- Preserved historical identifiers: `P1443`.

### `00_nucleo/prompts/compiler/stdlib/structural/p1444-tests.md`

- Pre-edit document SHA-256: `3c256c60906a431ee6703537a48f1a65a6ebae06e2d9d737f4c348d455d69f06`.
- Exclusive consumer: `01_core/src/compiler/stdlib/structural/p1444_tests.rs`; inspected SHA-256: `953b6c654fd64189cc710aba4d488319c6cb95257f44bf2c537839096204cc2f`.
- Preserved historical identifiers: `P1444`.

### `00_nucleo/prompts/compiler/stdlib/structural/p1445-tests.md`

- Pre-edit document SHA-256: `2d65e55a877de36e5de1ce47779764f4ced644d5ff267955e1c8e6e0409978d0`.
- Exclusive consumer: `01_core/src/compiler/stdlib/structural/p1445_tests.rs`; inspected SHA-256: `5c6e2f76a20f0beba3f55fba33e20c1afe1ddaab652c3f178f5480ba7a3c4dbe`.
- Preserved historical identifiers: `P1445`, `P1436`.

### `00_nucleo/prompts/compiler/stdlib/structural/p1446-tests.md`

- Pre-edit document SHA-256: `a84c1df27e322cf5890e2b4a7f51188f47ada94a39e23b9a9a3d64d7b8c5f807`.
- Exclusive consumer: `01_core/src/compiler/stdlib/structural/p1446_tests.rs`; inspected SHA-256: `0b984c5220489a591710e9c355006760671cad7223f836a8135be206870a2abf`.
- Preserved historical identifiers: `P1446`.

### `00_nucleo/prompts/compiler/stdlib/structural/p1447-tests.md`

- Pre-edit document SHA-256: `ebb401fdf1ff90556175c5fc55b0477349be1b48d7895b0a20c4526c4e8963a8`.
- Exclusive consumer: `01_core/src/compiler/stdlib/structural/p1447_tests.rs`; inspected SHA-256: `456d4748c280ccc09a0213e8bbd937f79c87471de56a85ced637851c928ce901`.
- Preserved historical identifiers: `P1447`, `P1436`.

### `00_nucleo/prompts/compiler/stdlib/structural/p1448-tests.md`

- Pre-edit document SHA-256: `fa5ae030a2b290768849086b65670f0261b1e2b72d324f35919e5a23421bce38`.
- Exclusive consumer: `01_core/src/compiler/stdlib/structural/p1448_tests.rs`; inspected SHA-256: `e459c1ece49d3c949b8f6a1b6b38c6231c3f97e14842dd5003bc0c96249ec82a`.
- Preserved historical identifiers: `P1448`.

### `00_nucleo/prompts/compiler/stdlib/structural/p1449-tests.md`

- Pre-edit document SHA-256: `97cf90ca969a11f4ca98c1360887498eb1454f52b729f72d5ea655bbc4b2960f`.
- Exclusive consumer: `01_core/src/compiler/stdlib/structural/p1449_tests.rs`; inspected SHA-256: `1e84acbe6f7ddee5eb54b570c76cc4d779ba0c02c48f838a32d277a98dc72169`.
- Preserved historical identifiers: `P1449`.

### `00_nucleo/prompts/compiler/stdlib/structural/p1450-tests.md`

- Pre-edit document SHA-256: `a96748e9623216aa205cf0abd17da660279196e19575a5fce1bd70660aad5d4c`.
- Exclusive consumer: `01_core/src/compiler/stdlib/structural/p1450_tests.rs`; inspected SHA-256: `de55690af456a9ac797e46d8f0e776c13ef52020c278eab6e7fc48a2182bbed8`.
- Preserved historical identifiers: `P1450`.

### `00_nucleo/prompts/compiler/stdlib/structural/p1451-tests.md`

- Pre-edit document SHA-256: `6c8e31fdd160dea606c08ebceaf64ae268b36e17faea9a70095a4abd68ca3bae`.
- Exclusive consumer: `01_core/src/compiler/stdlib/structural/p1451_tests.rs`; inspected SHA-256: `4d35eec9ad71a5526d9c6e804b5ff49410a85f4466140571bee02be7a6c96817`.
- Preserved historical identifiers: `P1451`.

### `00_nucleo/prompts/compiler/stdlib/structural/quote.md`

- Pre-edit document SHA-256: `9d3d0829db35b64f00ac379d2c118cf124aaa0456e1f0afa671e77c36cdf70c2`.
- Exclusive consumer: `01_core/src/compiler/stdlib/structural/quote.rs`; inspected SHA-256: `bb487ce99201cd6a1cf77bbc22ff9c535b864b57eea8c534608b9d06d15f6104`.

### `00_nucleo/prompts/compiler/stdlib/structural/table_grid.md`

- Pre-edit document SHA-256: `153a22b018f247e1b37943a257afa3ad55da414b92886655a1415079118c3edd`.
- Exclusive consumer: `01_core/src/compiler/stdlib/structural/table_grid.rs`; inspected SHA-256: `ad2ac58eb4ba8db54d7e831fa67c9ce795e76e8ef00fe8c2e141dd0cc477793e`.
- Historical source references: `table_grid.rs:542-581,123-129`.
- Preserved historical identifiers: `P69`, `P962`, `P229-230`, `P233-238`, `P327`, `P772f-j`, `P320`, `P512`, `P513`, `P739`, `P224.B`, `P224.C`, `P235`, `P726`, `P887`, `P1288`.

### `00_nucleo/prompts/compiler/stdlib/structural/table_lines.md`

- Pre-edit document SHA-256: `a02bcbaed2ef3aae2b172f78a737c2e95588883605608199259632ecc1d6e29b`.
- Exclusive consumer: `01_core/src/compiler/stdlib/structural/table_lines.rs`; inspected SHA-256: `45c72c05a83042b2c852d260b82a37b620b07462b786a26c7895b2cfe6a56fcb`.
- Preserved historical identifiers: `P69`, `P962`, `P512`, `P513`, `P739`, `P739A`, `P726`.

### `00_nucleo/prompts/compiler/stdlib/structural/title.md`

- Pre-edit document SHA-256: `6a499495947f46d1dad57b96990f2ed95344d8f49ffdb8b5d97fc002a7277265`.
- Exclusive consumer: `01_core/src/compiler/stdlib/structural/title.rs`; inspected SHA-256: `463ae3342c39a026ae07f9e6ec9b47e6803c37999b3c5145f7ecc487569ec194`.
- Original recorded dates/timestamps: `2026-07-16`, `2026-08-13`.

### `00_nucleo/prompts/compiler/stdlib/sys.md`

- Pre-edit document SHA-256: `b0eba943a4e7649c0db5d22b79f57ed4d8f7b96da3727b62e76e87fd2e9cec0e`.
- Exclusive consumer: `01_core/src/compiler/stdlib/sys.rs`; inspected SHA-256: `09393585313930552112a4613df6c7fff05c56450b15acdc8cc306e802c386fa`.
- Historical source references: `lab/typst-original/crates/typst-library/src/sys.rs:15-16`.
- Preserved historical identifiers: `P694`, `P1137`, `P796`, `P786`, `P730`, `P731`, `P679`.

### `00_nucleo/prompts/compiler/stdlib/text.md`

- Pre-edit document SHA-256: `1dadbba93127f2b62a8728bf0a92c5378c8f52b15bff5aa9152290faf0ac72a5`.
- Exclusive consumer: `01_core/src/compiler/stdlib/text/mod.rs`; inspected SHA-256: `876a4170bdc3c79e0f0d6b92dea175ead2420d66420c5023fe052fa27ecfb61a`.
- Preserved historical identifiers: `P284`, `P287`.
- Original recorded dates/timestamps: `2026-04-23`, `2026-05-19`, `2026-06-11`, `2026-06-18`, `2026-06-22`, `2026-06-24`, `2026-06-27`, `2026-06-29`, `2026-07-05`, `2026-07-17`, `2026-07-21`, `2026-07-22`, `2026-07-23`, `2026-07-19`, `2026-08-13`.

### `00_nucleo/prompts/compiler/stdlib/text/case.md`

- Pre-edit document SHA-256: `b40155505b65b4ff0d9b41c7c9ecaff1c51b467bdfab7fc356ac808288adbc2c`.
- Exclusive consumer: `01_core/src/compiler/stdlib/text/case.rs`; inspected SHA-256: `a6dc97b1d230b74ac4807ae94f136e0d7ac817f378190a916e50fde7acc96578`.
- Original recorded dates/timestamps: `2026-04-23`.

### `00_nucleo/prompts/compiler/stdlib/text/constructor.md`

- Pre-edit document SHA-256: `be4549e1d2633bd50f3f628bdc54d1d605907989be6522cf7fd9b9466ec770d2`.
- Exclusive consumer: `01_core/src/compiler/stdlib/text/constructor.rs`; inspected SHA-256: `3a6a4d3260386cbbf487f1b62f5465c66f07c691c76a17d6bcb1b6711624b963`.

### `00_nucleo/prompts/compiler/stdlib/text/deco.md`

- Pre-edit document SHA-256: `6238284029f77370cfd7757eb27c99657f6fd337178a7a689d69c0f4591aa5f5`.
- Exclusive consumer: `01_core/src/compiler/stdlib/text/deco.rs`; inspected SHA-256: `bfe2bad04acee4bd6a5bb12c5aab82e0fe2247e4b19befc513224c533a148b97`.
- Preserved historical identifiers: `P284`, `P1455`, `3360722a3eb3b63371d2ad9e2d1758d85cd9f98d`.
- Original recorded dates/timestamps: `2026-06-24`, `2026-06-27`, `2026-09-18`.

### `00_nucleo/prompts/compiler/stdlib/text/lorem.md`

- Pre-edit document SHA-256: `f9c6cc283f8395af65077fbe5016755ba85f42dd05d43536df1ef36f630e3d27`.
- Exclusive consumer: `01_core/src/compiler/stdlib/text/lorem.rs`; inspected SHA-256: `4a2dee3a042266bf64f66fd15c5ec8a7122a9e83f79668460736b9412fdc07a9`.
- Original recorded dates/timestamps: `2026-07-21`, `2026-06-22`.

### `00_nucleo/prompts/compiler/stdlib/text/regex.md`

- Pre-edit document SHA-256: `e08e2c922fa1ddae0018e5250dcff3d1901d101b69f59909c29d04fa0c396cc3`.
- Non-materializable historical redirect; no productive consumer.
- Historical source references: `foundations/str.rs:1017`.

### `00_nucleo/prompts/compiler/stdlib/text/shift.md`

- Pre-edit document SHA-256: `d48b08960c69eef929a21717100952b6ada1d4b6cdfbaee6442b4429632177ab`.
- Exclusive consumer: `01_core/src/compiler/stdlib/text/shift.rs`; inspected SHA-256: `5c9f22321cc44bfd225b656690792afbc599261a552893f632bb669087a31ddf`.
- Historical source references: `typst-library/src/text/shift.rs:16`.
- Original recorded dates/timestamps: `2026-06-24`, `2026-06-27`.

### `00_nucleo/prompts/compiler/stdlib/text/smallcaps.md`

- Pre-edit document SHA-256: `1b86c32481eaa903bb3119817b4d31e0f1cb5bf66c8d2cc41c74c4535ef2c290`.
- Exclusive consumer: `01_core/src/compiler/stdlib/text/smallcaps.rs`; inspected SHA-256: `1db2d93153312648c5d7e8482d58c377609f7bcb276c260b2d7105a7585298c0`.
- Original recorded dates/timestamps: `2026-06-22`, `2026-06-24`.

### `00_nucleo/prompts/compiler/stdlib/text/smartquote.md`

- Pre-edit document SHA-256: `e0a4e370a36a240d6a2e612df3110cb14a93ae798aa16186829e8387533f6ee1`.
- Exclusive consumer: `01_core/src/compiler/stdlib/text/smartquote.rs`; inspected SHA-256: `edf7e528097cd2ec69ff9608db9b58917a95acbeb157c8d43de9ea09ab883d0d`.
- Historical source references: `text/smartquote.rs:51-89,201-317,335-419`, `compiler/eval/repr.rs:681-739`, `compiler/eval/bindings/field_access.rs:648-677`.
- Preserved historical identifiers: `P1286`.
- Original recorded dates/timestamps: `2026-05-19`, `2026-06-11`.

## Remaining assigned work

The following files retain Portuguese or mixed historical/normative material and still require full consumer comparison, English sanitation and relevant history transfer. They were deliberately not replaced with generic summaries or treated as completed merely because their owners were located:

- `00_nucleo/prompts/compiler/stdlib/counter.md`
- `00_nucleo/prompts/compiler/stdlib/figure_image.md`
- `00_nucleo/prompts/compiler/stdlib/format.md`
- `00_nucleo/prompts/compiler/stdlib/foundations/color.md`
- `00_nucleo/prompts/compiler/stdlib/gradients.md`
- `00_nucleo/prompts/compiler/stdlib/html.md`
- `00_nucleo/prompts/compiler/stdlib/layout.md`
- `00_nucleo/prompts/compiler/stdlib/loading.md`
- `00_nucleo/prompts/compiler/stdlib/math_style.md`
- `00_nucleo/prompts/compiler/stdlib/pdf.md`
- `00_nucleo/prompts/compiler/stdlib/plugin.md`
- `00_nucleo/prompts/compiler/stdlib/shapes.md`
- `00_nucleo/prompts/compiler/stdlib/state.md`
- `00_nucleo/prompts/compiler/stdlib/structural/footnote.md`
- `00_nucleo/prompts/compiler/stdlib/structural/lists.md`
- `00_nucleo/prompts/compiler/stdlib/structural/math.md`
- `00_nucleo/prompts/compiler/stdlib/structural/outline.md`
- `00_nucleo/prompts/compiler/stdlib/structural/par.md`
- `00_nucleo/prompts/compiler/stdlib/sym.md`
- `00_nucleo/prompts/compiler/stdlib/tiling-stdlib.md`
- `00_nucleo/prompts/compiler/stdlib/transforms.md`

Pre-existing English parser-engine, registry and tests/ prompts are unchanged. Their metadata, snapshots and pins are preserved; a complete family-wide audit is still pending. The source/code distinction and exact catalog/typed-attribute/codec/math contracts of the larger units must be retained in continuation.

## Validation

Final read-only V15/V26 validation passed after the documentary batches and diagnostic creation. Scoped git diff --check passed. SHA-256 comparison found every inventoried Rust consumer unchanged, all code-hash metadata and nucleus pins preserved, and unchanged assigned documents identical to their pre-edit snapshots. Read-only V5 SARIF reports expected warning-level drift in assigned consumers; its zero exit status does not mean absence of drift. Hash repair is intentionally not performed: V5 drift is expected for changed L0 text, and resealing would require editing productive headers, outside this assignment. No global formatter, code/test edit, ADR edit or nucleus edit is part of this work.
