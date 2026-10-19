# Independent diagnostic note review

## Historical initial six-owner scan

Artifact-path clarification: candidate `after.sarif`, `after.sarif.gz`, `lint-delta.json`, and `lint-state.json.gz` references in this section refer to `candidate-initial-lint/`. Baseline artifacts remain at the batch root. Historical hashes below are preserved.

The initial candidate artifacts named in this historical section are preserved under `candidate-initial-lint/`; baseline artifacts remain at the batch root. Root candidate artifact paths now refer to the final scan reviewed later in this document.

Review UTC: `2026-10-19T19:14:54.260776+00:00`.
HEAD: `5e1a01066b9eef0eb3c478e8b49e15e1da15b0b9`; candidate is an uncommitted working tree. Exact changed-file list is the verbatim `git diff HEAD --stat` in `lint-state.json.gz` (reproduced below). Baseline scan ran `2026-10-19T18:52:20.523085+00:00`–`2026-10-19T18:52:27.582978+00:00` on the clean HEAD. Candidate scan ran `2026-10-19T19:12:32.252108+00:00`–`2026-10-19T19:12:37.711860+00:00`. This review writes evidence only; no source, L0, configuration, or linter edits.

```text
 .../eval/bindings/field_access/content_methods.md  |  133 ++-
 .../core/compiler/stdlib/loading/encoding.md       |   65 +-
 .../compiler/stdlib/structural/math/equation.md    |   36 +-
 00_nucleo/prompts/core/entities/content/fields.md  |   50 +-
 .../prompts/core/entities/content/identity.md      |   33 +-
 00_nucleo/prompts/shell/cli.md                     |   95 +-
 .../eval/bindings/field_access/content_methods.rs  | 1038 +++++++++++++++++++-
 01_core/src/compiler/stdlib/loading/encoding.rs    |  288 +++++-
 .../compiler/stdlib/structural/math/equation.rs    |  113 ++-
 01_core/src/entities/content/fields.rs             |   73 +-
 01_core/src/entities/content/identity.rs           |  207 +++-
 02_shell/src/cli.rs                                |  901 ++++++++++++++++-
 12 files changed, 2966 insertions(+), 66 deletions(-)
```

## Independent comparison method

Loaded every result from the full baseline and candidate SARIF, rather than treating the supplied delta as authority. Matched a multiset of `(ruleId, level, repository-relative file, complete message)` while ignoring only locations, then paired old/new locations. Counts and complete source messages were independently recomputed. Both runs have identical tool metadata and use the same command and executable SHA-256.

- Linter SHA-256: `421d0797e7ae1a5102e9c5bcd87b488fea70a1997394c38765a03239fe579121`.
- Command: `/repos/Antigravity/typst-crystalline/lab/target/linter/debug/crystalline-lint . --format sarif`.
- `baseline-state.json.gz` records 2,100 input hashes; `lint-state.json.gz` records 2,101. Exactly the six source/owner pairs changed among shared paths; the added input is the constructor-origin nucleus.
- The six sources are content_methods.rs, loading/encoding.rs, structural/math/equation.rs, content/fields.rs, content/identity.rs, and shell/cli.rs; their six canonical owners are the only changed owning prompts.

| Artifact | SHA-256 |
|---|---|
| `baseline.sarif.gz` | `76ff2d76b9d364f494610661bb3fdeeff03a410c002e6b67a7c1b7b986f93f24` |
| `after.sarif` | `9f92acfc68672318e069e562a7911f72ba72f4c79ecc7753df4b13850c3a1018` |
| `lint-delta.json` | `8a5b470f407df0a564e0537768ff64c05f55887287e4887ac81a0e49dd37750b` |
| `baseline-state.json.gz` | `95c06ae3769bcfb846d03b2b078602daedbef3ac69581d212df53e986e030976` |
| `lint-state.json.gz` | `cdfa33deb791c7d22281a6f6fa4d079b9eceac13524afe461e2e90fe90422915` |

## Actual diagnostic change

| Rule and level | Before | After | Substantive change |
|---|---:|---:|---|
| V16 error | 44 | 37 | Seven intended decisions removed |
| V19 note | 1,222 | 1,237 | Fifteen additions, no removals |
| V20 note | 876 | 877 | One addition, no removals |

All existing V19/V20 messages survive with the same file, level, and complete text. The raw location-sensitive delta reports removals and additions for moved notes; those are not substantive removals. There are nineteen relocated V19 notes and two relocated V20 notes:

| File | V19 relocated | V20 relocated |
|---|---:|---:|
| `content_methods.rs` | 11 | 1 |
| `fields.rs` | 2 | 1 |
| `identity.rs` | 2 | 0 |
| `cli.rs` | 4 | 0 |

The pre-existing encoding wildcard V16 remains unchanged in substance at `encoding.rs:110→112`; it is outside the seven repaired decisions. No new errors or warnings were introduced. The full scan still has 37 V16 errors; this review does not claim a zero-violation lint result.

## Substantive added notes and source review

| Rule | Candidate source location | Interpretation |
|---|---|---|
| V19 | `01_core/src/compiler/eval/bindings/field_access/content_methods.rs:224:13` | Required OutlineEntry level/element field group; fill has its separate explicit-presence branch. |
| V19 | `01_core/src/compiler/eval/bindings/field_access/content_methods.rs:239:9` | Nominal remaining Content getter delegation, preserving Some→Set and None→Undeclared. The @ binding encloses an or-pattern with variant payloads, accounting for the new depth-3 V20. |
| V20 | `01_core/src/compiler/eval/bindings/field_access/content_methods.rs:239:9` | Nominal remaining Content getter delegation, preserving Some→Set and None→Undeclared. The @ binding encloses an or-pattern with variant payloads, accounting for the new depth-3 V20. |
| V19 | `01_core/src/compiler/eval/bindings/field_access/content_methods.rs:462:9` | Bounded legacy body candidate group; candidate names are filtered through ContentField. It does not certify richer schemas. |
| V19 | `01_core/src/compiler/eval/bindings/field_access/content_methods.rs:556:13` | Unset and Undeclared explicitly produce no public field; Set is kept separately. |
| V19 | `01_core/src/compiler/eval/bindings/field_access/content_methods.rs:585:9` | Non-Equation complement of bounded constructor recognizer; carriers/marker-positive branches remain separate. |
| V19 | `01_core/src/compiler/eval/bindings/field_access/content_methods.rs:889:9` | Explicit legacy noncallable element-function class; no inferred function identity from names or shape. |
| V19 | `01_core/src/compiler/eval/bindings/field_access/content_methods.rs:1087:9` | Ordinary method receivers continue after four carrier returns; location/snapshot precedence remains in those returns. |
| V19 | `01_core/src/compiler/stdlib/loading/encoding.rs:156:9` | New private constructor recognizer negative class; ordinary Styled is checked before entering this class. |
| V19 | `01_core/src/entities/content/identity.rs:30:9` | New private constructor recognizer negative class; public names otherwise remain core-owned. |
| V19 | `02_shell/src/cli.rs:1173:9` | New private shell constructor recognizer negative class. |
| V19 | `02_shell/src/cli.rs:1299:9` | V16-083 ordinary continuation group after four carrier returns, before Label processing. |
| V19 | `02_shell/src/cli.rs:1674:9` | V16-084 body-probe group retains optional get_field(body), including Matrix absence and Dynamic conditional presence. |
| V19 | `02_shell/src/cli.rs:1690:9` | V16-084 already-returned carriers/Label are named explicitly; no new panic or wrapper projection. |
| V19 | `02_shell/src/cli.rs:1696:9` | V16-084 current func-only group; stored Rust bodies do not imply public getter presence. |
| V19 | `02_shell/src/cli.rs:1803:9` | V16-085 delegates explicitly named variants to elem_name; no shell-owned duplicate name table. |

These notes remain actionable coverage reminders. A grouped arm running once does not prove all its alternatives were dynamically exercised. The nominal inventory/source audit and independent compiler-negative experiments defend classification/exhaustiveness; they do not replace language behavior controls or establish all-variant runtime coverage. The new V20 pattern has depth three because of `other @ (Content::... | ...)`; it retains the same getter delegation under an explicit finite complement rather than weakening the domain boundary. This is a substantive new note, not a line move or a suppressed diagnostic.

Reviewed the changed production branches behind all additions, including producer marker insertion, getter admission, constructor helper boundaries, candidate field/native/receiver decisions, shell classification and known-field projection, and native textual encoding. The producer and getter changes introduce no new V19/V20 beyond the locations listed above. Existing getter notes merely relocate.

## Suppression and configuration checks

- Neither SARIF contains a result `suppressions` property. Rule levels and tool rule metadata are identical.
- The linter executable SHA and invocation are identical. No linter source, lint configuration, dependency manifest, whitelist, exclusion, baseline file, or suppression file changed.
- Inspected added source lines for lint suppressions, allow/expect attributes and disable directives; none were introduced. The newly visible `.expect("entry required field")` and `.expect("entry explicit fill")` are value extraction at required/present getter fields, not lint suppression attributes.
- The shared nucleus records the causal marker boundary; it does not exempt consumers from exhaustiveness or weaken authoritative snapshots.

## JSON/YAML and native codec scope

Current source and L0 agree on a narrow constructor-origin correction: an existing nonempty explicit-option transport receives the private marker; each recognizer requires the last direct marker to be Bool(true), a valid Equation through causal carriers/marked constructors, and Realized.visual rather than original. Ordinary outer Styled remains a barrier. No public helper, entity field, mode, default, or phase was added.

Shell emits only known getters for marked constructor transport in declared order. Native textual encoding bypasses its ordinary Styled branch only for the same valid constructor shape and then delegates to existing strict fields. Present snapshots, including empty or deliberately different known data, precede fallback. Neither path emits the private marker or fabricates missing supplement/numbering defaults. Ordinary Styled remains child/styles, supported independently by pinned upstream required field declarations.

Bare Equation still has its existing block-presence and shell legacy default expansion debts. Full ordinary Styled fields/has/at/repr, arbitrary nested style merging, Box width/body completeness, richer legacy fields, PdfArtifact.kind omission, generated Caption selection, and nested Heading expansion are not certified by this batch. The retained encoding wildcard also remains visible.

At review time the candidate binary replay has not yet been delivered: `candidate-run/core-tests.log` is still building core, and language replay notes explicitly await the built-product signal. Therefore this is a source/L0 and expected-scope review, not a claim of candidate JSON/YAML parity or completed GREEN. The frozen language corpus compares decoded JSON/YAML values, preserves error channels and diagnostics, and retains partial-compound changes for explicit evidence-based review; no global codec claim follows from the lint delta. Parent receipts must supply candidate results.

## Review finding and disposition

No diagnostic suppression, scope escape, or new warning/error was found. All substantive notes are accounted for above. One documentation housekeeping issue was reported to the parent: encoding L0 uses the execution-label heading `Batch E: marked Equation constructor textual projection`; the parent requested semantic headings in active L0. Rename the heading through its owner and reseal if applying that correction. This note did not edit L0.


## Historical seven-owner scan review

Artifact-path clarification: candidate artifact names in this section now refer to `candidate-seven-owner-lint/`, preserving the recorded hashes. The current root artifacts belong to the ten-owner scan reviewed below. “Final” in the following historical text means final for that earlier seven-owner candidate.

Review UTC: `2026-10-19T19:34:31.652887+00:00`. This section supersedes the historical scan for final lint evidence, while preserving its measured results. HEAD remains `5e1a01066b9eef0eb3c478e8b49e15e1da15b0b9`; the final candidate is the uncommitted tree recorded in `lint-state.json.gz`. Final scan interval: `2026-10-19T19:32:52.875237+00:00`–`2026-10-19T19:32:58.982202+00:00`. The earlier candidate scan has been archived under `candidate-initial-lint/`; its historical hashes above are not hashes of the replacement root artifacts.

Exact final scan changed-file list:

```text
 .../_nuclei/math/equation-constructor-origin.toml  |   29 +
 .../eval/bindings/field_access/content_methods.md  |  161 ++-
 .../core/compiler/eval/expression_dispatch.md      |   55 +-
 .../core/compiler/stdlib/loading/encoding.md       |   65 +-
 .../compiler/stdlib/structural/math/equation.md    |   36 +-
 00_nucleo/prompts/core/entities/content/fields.md  |   50 +-
 .../prompts/core/entities/content/identity.md      |   33 +-
 00_nucleo/prompts/shell/cli.md                     |   95 +-
 .../eval/bindings/field_access/content_methods.rs  | 1079 +++++++++++++++++++-
 01_core/src/compiler/eval/expression_dispatch.rs   |   83 +-
 01_core/src/compiler/stdlib/loading/encoding.rs    |  288 +++++-
 .../compiler/stdlib/structural/math/equation.rs    |  113 +-
 01_core/src/entities/content/fields.rs             |   73 +-
 01_core/src/entities/content/identity.rs           |  207 +++-
 02_shell/src/cli.rs                                |  901 +++++++++++++++-
 15 files changed, 3192 insertions(+), 76 deletions(-)
```

| Final artifact | SHA-256 |
|---|---|
| `after.sarif.gz` | `6bbbf3d52f20d5d847572003a0a910ed591d8747d3c904daad02c37b3fbb188c` |
| `after.sarif` | `aa224423cb7e9e7c6910093ab0e87ffe43106b74d8cf5f20ac2a3ab9b5c4f1a4` |
| `lint-delta.json` | `1cdbb65d3f2f4d00916b57eb2d1d917a5ce2efe735ca1c5db97d64928142c675` |
| `lint-state.json.gz` | `c64453ecbe431963f26e97cb711834ebc07d7dd60762da495afc943f321c81a6` |

Independently loaded all full baseline, initial-candidate, and final SARIF results and repeated the complete-message multiset comparison described above. Final counts remain V16 error 44→37, V19 note 1,222→1,237, and V20 note 876→877. There are fifteen substantive V19 additions, one substantive V20 addition, and no substantive note removals. The initial and final candidate multisets are identical when ignoring locations only. The substantive context table above remains valid except that the ordinary receiver complement moved from content_methods.rs:1087 to :1089. These notes remain coverage obligations, not proof of exhaustive runtime coverage.

Final baseline-to-candidate note movements are twenty-three V19 and three V20: the initial movement table gains four V19 and one V20 in expression_dispatch.rs. Its twelve-line constructor-boundary guard moves the existing expression/static-warning decisions; it introduces no grouped pattern or depth note. Reviewed that guard: it checks the latest direct marker for Bool true, retains the inner constructor under an external Styled, and leaves Source recursion and ordinary merge precedence unchanged. Reviewed the reflection follow-up: native Equation function identity uses the canonical native name while retaining the actual native callable. Neither follow-up adds a suppression or weakens an existing nominal complement.

The existing encoding V16 is retained at :110→112 with identical rule, level, message, and source line `_ => {}`. All SARIF results outside the seven changed productive source files are exactly equal, including their locations and all result fields; this independently confirms `outside_scope_exact: true`. Seven V16 decisions alone disappear: V16-001–004 and V16-083–085. No new error or warning appears. Lint exits 1 because 37 pre-existing V16 errors remain; zero-violation repository status is not claimed.

The full input manifests still contain 2,100 baseline inputs and 2,101 candidate inputs. Exactly seven productive sources and their seven canonical owners differ among existing paths; the only added input is the constructor-origin nucleus. There are no removed inputs. Linter SHA `421d0797e7ae1a5102e9c5bcd87b488fea70a1997394c38765a03239fe579121`, command, and full SARIF tool metadata are unchanged. Neither full SARIF contains a result suppression; the changed-file inventory contains no linter, configuration, manifest, dependency, whitelist, exclusion, or suppression edits. Added productive lines introduce no allow/expect lint attributes or disabling directives. The strengthened nucleus is an external-style preservation obligation, not a warning exemption.

The historical L0 heading finding is resolved: encoding now uses `Marked Equation constructor textual projection`. The current expression-dispatch owner distinguishes the baseline missing-marker diagnosis from the current marked producer while explicitly retaining unresolved Figure/Equation static-supplement diagnostic obligations and their refuter. This review edits evidence only.

The initial candidate language replay exposed a real external lexical-style regression in three source cases and a canonical native-function name mismatch. Those are historical failures, not exceptions to accept. Follow-up tests established semantic RED before the narrow producer-boundary and native-name corrections; final GREEN and rebuilt-product replay remain parent-controlled gates. Current codec expectations retain outer Styled child/styles and the known inner Equation options, preserve authoritative snapshots and query errors, and compare parsed JSON/YAML values. Existing default/presence and richer-schema debts listed above remain limits. This lint review does not turn partial language parity into complete parity or certify gates still running.


## Final ten-owner scan review

Review UTC: `2026-10-19T19:53:32.758998+00:00`. Scan interval: `2026-10-19T19:51:48.378165+00:00`–`2026-10-19T19:51:53.852686+00:00`. HEAD `5e1a01066b9eef0eb3c478e8b49e15e1da15b0b9`, uncommitted working tree; exact input hashes, tool identity, invocation and changed-file list are in root `lint-state.json.gz`. This section supersedes both earlier scans for the current lint disposition only; tests, rebuilt-product replay and structural experiments have separate gates.

```text
 .../_nuclei/math/equation-constructor-origin.toml  |   29 +
 .../eval/bindings/field_access/content_methods.md  |  161 ++-
 .../core/compiler/eval/expression_dispatch.md      |   55 +-
 .../core/compiler/eval/selector_matching.md        |   62 +-
 .../prompts/core/compiler/introspect/from_tags.md  |   14 +-
 .../compiler/introspect/from_tags/equations.md     |   68 +-
 .../core/compiler/stdlib/loading/encoding.md       |   65 +-
 .../compiler/stdlib/structural/math/equation.md    |   36 +-
 00_nucleo/prompts/core/entities/content/fields.md  |   50 +-
 .../prompts/core/entities/content/identity.md      |   33 +-
 00_nucleo/prompts/shell/cli.md                     |   95 +-
 .../eval/bindings/field_access/content_methods.rs  | 1079 +++++++++++++++++++-
 01_core/src/compiler/eval/expression_dispatch.rs   |   83 +-
 01_core/src/compiler/eval/selector_matching.rs     |  113 +-
 01_core/src/compiler/introspect/from_tags.rs       |    2 +-
 .../src/compiler/introspect/from_tags/equations.rs |  218 +++-
 01_core/src/compiler/stdlib/loading/encoding.rs    |  288 +++++-
 .../compiler/stdlib/structural/math/equation.rs    |  113 +-
 01_core/src/entities/content/fields.rs             |   73 +-
 01_core/src/entities/content/identity.rs           |  207 +++-
 02_shell/src/cli.rs                                |  901 +++++++++++++++-
 21 files changed, 3619 insertions(+), 126 deletions(-)
```

| Current root artifact | SHA-256 |
|---|---|
| `after.sarif.gz` | `0b1d48bea24e368b7e02c1a55c19f13bd2b89c6b4d6458828a76c5f984c31a6e` |
| `after.sarif` | `aa224423cb7e9e7c6910093ab0e87ffe43106b74d8cf5f20ac2a3ab9b5c4f1a4` |
| `lint-delta.json` | `d41c1a72c021c41ceb5738b21ed1941cd6a7ce2f159223939105622b02c5d0c0` |
| `lint-state.json.gz` | `d44464bb6a7be90e5fb7722aa16e1b8b86f3689b3795cd6ea711fe8e302039e5` |

Independently loaded all full baseline/current/seven-owner SARIF results and repeated the complete-message multiset and raw outside-scope comparisons. V16 errors remain 44→37; V19 notes remain 1,222→1,237; V20 notes remain 876→877. Exactly fifteen substantive V19 and one substantive V20 notes are added, and no existing note is substantively removed. The same seven selected V16 findings disappear. The new producer and test/delegation owners introduce no additional notes, errors or warnings. The entire decompressed SARIF is identical to the archived seven-owner payload; differing compressed hashes do not imply differing findings.

The existing encoding V16 remains the same `_ => {}` finding at :110→112. Baseline-to-current movements remain twenty-three V19 and three V20, with the same file distribution documented in the seven-owner section. Every result outside the ten changed productive sources remains exactly equal to baseline, including locations and all result fields. No warning or error has been hidden by expanding the comparison scope: all newly included sources have no substantive diagnostic delta.

There are 2,100 baseline and 2,101 current input hashes. Exactly ten source/owner pairs differ among existing paths; the constructor-origin nucleus is the only added input, and none are removed. The three added pairs are selector_matching, introspect/from_tags, and introspect/from_tags/equations. The selector source adds actual-producer fixtures and controls; its productive algorithm is unchanged. The hub source changes only lineage metadata, while its owning delegation clause reflects the child snapshot contract. No linter/configuration/dependency/whitelist/exclusion/suppression file changed. Tool metadata, command, and linter SHA `421d0797e7ae1a5102e9c5bcd87b488fea70a1997394c38765a03239fe579121` match baseline. Neither full SARIF has a suppression property; inspection of added productive lines found no allow/expect lint attributes or disabling directives. Lint still exits 1 with 37 outstanding V16 errors, so repository-wide zero violations are not certified.

### Bounded producer self-review

This subsection is a self-review by the agent that wrote the occurrence producer and its co-located tests. It is not independent implementation certification. Separate evidence comes from the language agent's pinned-oracle occurrence probes, the reflection agent's selector tests, and the parent's actual RED/GREEN runs.

Compared the current child implementation with its full owner L0, the parent delegation clause, and the unchanged content-snapshot nucleus. `realize_equation_elements` retains the original content; it no longer writes unmarked Styled/None. It skips present snapshots before any field reconstruction, preserving Some(empty), known fields, order and labels verbatim. For absent snapshots it obtains block/body from existing getters, inserts the same four captured effective options in the measured order, appends actual Tag.info.label last when present, and publishes one new immutable entry. It adds neither func/location fields nor a constructor marker. Missing entries, non-Equation tags, and a base without required getters leave existing state untouched. The numbering/supplement callback functions are unchanged, and publication executes no callback. Normal walk creates the Equation/None input; this operation is not snapshot refresh semantics.

The co-located tests exercise two distinct captured occurrences, semantic identity, exact field ordering and label-last, default numbering/alignment/alt with already-finalized supplement, old-clone immutability, present-empty/present-known authority, missing entries and non-Equation guards. The reflection agent's tests call the actual public producer, then exercise body/block/options/label filtering, opposite-occurrence rejection, aliases, native function identity, Some(empty), and ordinary Styled opacity. Parent `query-test-classification.json` records actual preimplementation semantic failures rather than setup failures. These fixtures do not independently execute arbitrary supplement callbacks or certify every effective alignment value; unchanged callback bodies and the full regression/replay gates remain relevant.

No source/L0/test changes were made by this review. Final current-candidate unit/workspace/replay/structural gates were still running when this review was written; no result is inferred from an implementation inspection or from the unchanged lint counts. Raw-constructor presence debts and Figure/Equation static-supplement warning obligations remain open.
