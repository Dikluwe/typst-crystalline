# Shell L0 sanitation: retained history and unresolved decisions

## Scope and evidence boundary

This diagnostic is exclusively the historical archive and discrepancy ledger for the documentation sanitation of `00_nucleo/prompts/shell/`, including its test subdirectory. It is not an L0, owns no productive consumer and has no lineage header or code-hash claim.

The audit read repository instructions, the ADR authority index/order and the applicable CLI, presentation, evidence, gate, ownership and canonical-reference ADRs before editing. Current source was inspected read-only at HEAD `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, in an uncommitted working tree. The source fingerprint capture was `2026-09-27T00:55:50.723Z`. Pre-existing productive edits include `02_shell/src/cli.rs` and `03_infra/src/watch.rs`; recently atomized wiring files are also present in that tree. This is not an audit of a clean checkout and no unchanged historical commit is presented as the current state.

The following full-file fingerprints identify the exact consumers checked, not performance metrics or parity measurements:

| Consumer | SHA-256 at audit |
| --- | --- |
| `02_shell/build.rs` | `8fa42b2aecdaa6d8cbd31dfcfe901cef27474ab14cc42c0c05fe27d20fe175b5` |
| `02_shell/src/cli.rs` | `ac901c4a32f07f18a2ad781a13cbb36696f19524fbf6de7491c6dc2afbad8c56` |
| `02_shell/src/completions.rs` | `df554ecafadae69eddbb882e1eb7082299c1ab6c6ad04ec9ac785d1e351d4a13` |
| `02_shell/src/diagnostic.rs` | `40f2ef922f4f5dbc355a4c1b0242bc12bebb8e30d4e5e1def36bce16fd3a274d` |
| `02_shell/src/duration.rs` | `194118af4d7061d7b0bbd7ceb04439e0930ba2873167be9764711d4554e110d8` |
| `02_shell/src/fonts.rs` | `baac68de07f94a1293bb8d7703228144e227f8dc01b8f750b6d1ed35410a3b70` |
| `02_shell/src/info.rs` | `b923113547e4c6ded3b3320821075e0304371278da539132504609437b1464ad` |
| `03_infra/src/project_init.rs` | `6b4e93ca566697f482f2bcc8ca13706324fd66230553b789de02ae2d80e103d6` |
| `03_infra/src/watch.rs` | `3f8d8f5cf26105cd1bea76bdc1db765e0233cecb9bb0b11258571ec4ebbd2fbd` |
| `02_shell/tests/p1361_duration.rs` | `6a0a28b09f8fd54a5072c677d2b0664652f8787238b32a8ecddaf5ee5bc55ea8` |

No productive code, test, ADR, shared nucleus or .orig file is changed by this task. It does not run an oracle campaign, count lines, certify global sanitation, infer historical gate approval from implementation, or globally reseal hashes. Portuguese machine metadata tokens `Hash do Código` and `Núcleos Tekt` remain unchanged for tooling compatibility; specification prose is English. Existing code hashes and nucleus pins are preserved, not recomputed as behavioral attestations.

## Ownership and references

Each assigned L0 names one exclusive consumer. The font presentation owner is `02_shell/src/fonts.rs`, not the former multi-file CLI/discovery/wiring target list. The init owner is `03_infra/src/project_init.rs`, in L3 despite its L0 directory name. The watch owner likewise remains L3. The duration test owns only `02_shell/tests/p1361_duration.rs`, distinct from the productive formatter owner.

Cross-owner prompt references are full canonical `00_nucleo/prompts/...` paths. Wiring references now identify the corresponding command module rather than attributing all command behavior to main.rs. BundleFile points to `00_nucleo/prompts/infra/pipeline/bundle.md`. No prompt or source is reassigned outside this documentation scope and no nucleus is edited.

## Retained historical decision evidence

### CLI foundation and build identity

The original CLI/diagnostic migration narratives recorded the move of argument parsing and user-facing formatting into L2 under ADR-0049/0050. Functional flag translation and output precedence came from ADR-0051. Older statements that root/font paths or subcommands were future work are implementation chronology, not current contracts.

P796 recorded the owner's explicit version-presentation decision on 2026-07-21: public parity version and crystalline build identity are distinct from the workspace crate version. P1033 added Git HEAD/refs rebuild inputs to avoid a captured identity becoming stale across later builds. These origins do not make a CLI version string proof of binary provenance.

### Eval, query and public values

P1137's initial eval scope supported JSON/raw without document context, while YAML, contextual --in and query were deferred. Those old ABSENT/scope-out states no longer describe the consumer. Symbol support was recorded in P1163; Stroke public serialization was approved in P1224 on 2026-08-26. P1285, approved on 2026-08-30, generalized nominal repr, nonfinite values, bytes, YAML and non-heading-only query. The earlier rule that all nominal types outside a small list must fail is superseded by that broader contract, not preserved as a current prohibition.

P1307-R5 retained an independent content-snapshot measurement at HEAD `b303f1f15b610e09872b567027e0d806387fde8c` with an uncommitted tree recorded by its original evidence. Its measurement artifact SHA-256 is `82b2de8863ae5cd4b706eb9d5a6b285e1ed31dce3c126c8e5383a5af59ab46c8`, against historical upstream `a51e02804`. It distinguished authoritative located snapshots from the smaller vanilla raw form and the crystalline legacy fallback. This evidence motivates preserving snapshot causality, not silently deleting fallback fields or claiming the fallback is complete.

P1293's typed MathAttach slot distinctions preserve Omitted/ExplicitNone/Present. Their current invariants reside in the pinned nucleus. Empty content does not imply absence; an explicit none blocks fallback.

P1460 measured the raw heading supplement's duplicate local decision at commit `7f7a7ab1f9afc3e92ed29800408d9508525a6094`, consumer SHA-256 `012710821b5521c9da2f32d8c14e78a94bd0dd03cddc681830dfa8ad944bf7ae`. The owner approved the additive L1 API in the request of 2026-09-18 after the P1459 gate warning. The chosen call is heading_supplement_for_lang(None): raw content has no StyleChain. It preserves legacy output and changes decision ownership only. This is not permission to infer a locale, recreate a shell table or remove legacy fields.

P1401's contextual eval proposal is now represented by --in and the current document-aware composition. Eval without context remains available. The former full-error wiring debt is obsolete in the parser's current translation, notwithstanding stale source comments; source comments were not edited.

### Font-path empty values

P1500's historical preflight was recorded at `2026-09-25T11:43:20Z`, crystalline HEAD `5d2798dc473346ea7169d1bf171425db734056ad`, with its recorded tree clean before and after probes. The vanilla pin was `9f2b6e8715237cb086899a42873660fe744622e8`, binary SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. The crystalline release binary SHA-256 was `2620ffb09b9f1ed2f51bdb412cd883cc1bf944c5f0a6a7433926eee7146412e3`.

Those probes accepted empty font-path flag/environment values in vanilla but rejected them in that candidate parser. A flag missing its argument was rejected by both; explicitly empty values replaced nonempty environment configuration without searching cwd. Nonempty segments retained order/repetition. Upstream evidence was args.rs:538 and :880 plus fonts.rs:50. The current consumer preserves optional slots and filters them in L2. The old proposed/pending label is removed from the specification; this source inspection is not a retroactive gate-approval attestation.

### Completion selectors

P1137's generator observation established that completion scripts originate from the real clap tree and that query may remain in Bash despite being hidden in help.

P1501's preflight ran during `2026-09-25T12:28:14Z–12:29:26Z`, crystalline HEAD `2accc929159f3a744d4965400bdbb47d34d33e17`, with its recorded tree clean before/after. It used the same vanilla pin and binary hash as the font-path preflight. Candidate debug binary SHA-256 was `509a3de9ab945b3f52c15ad720752e2795090087bb5abee16385aa2074a095c5`. Vanilla accepted Nushell with exit 0 and structural markers `module completions {`/`export extern typst`; the then-candidate rejected it with exit 2. Bash worked and an unknown shell failed in both.

Upstream args.rs:282 and completions.rs:8 used a closed shell enum and the official Nushell generator. Historical installed-generator/dependency version constraints explained a compatible dependency update; they are not a permanent lockfile-version contract or byte-parity criterion. The current consumer already accepts Nushell through CompletionShell. The former pending label is not a current absence claim; implementation alone does not re-attest the original approval record.

### Features, targets and serialization modes

Earlier CLI inventories that advertised only html, or html/a11y-extras but not bundle, described prior scope. Current compile/watch/eval support the closed html/a11y-extras/bundle feature set. ADR-0128's feature/target separation remains binding: default empty features and explicit target selection are independent.

The HTML serializer decision retained crystalline as default and vanilla as an explicit compile-only alternative. Its residual measurement was `2026-09-02T12:14:19-03:00`, artifact SHA-256 `4545df3baa07d09c5c004a77002d18eeaedb47a22aa4883dc2bc3ba12323676a`. Both forms were valid HTML with contextual escaping differences. This evidence supports explicit selection, not a claim that one mode is intrinsically invalid or a default swap.

P1484 transported bundle without implicitly publishing asset. P1485 added explicit bundle target and local watch serving. Old proposal/absent statements are removed; experimental warnings, explicit feature gates, loopback behavior and disk-versus-served HTML distinction remain contracts. The earlier probes are retained by `00_nucleo/diagnosticos/typst-p1485-bundle-sondas.md`, not rerun or promoted to full parity here.

### PDF options and reduced standard scope

ADR-0126's amended decision makes verbose PDF production default, compact optional, with accessibility a separate axis. The reversed historical labels and obsolete PDF-only/PNG-SVG-absent parser states do not belong in current shell specifications.

P1359/P1360 recorded the modern tagged/pretty tri-state confirmation on 2026-09-12. Later document-option work retained absent overrides for document/default precedence; pretty, tagged and compact are not interchangeable switches.

P1468's approval followed the causal L0-only commits `9bb20b8e8`/`70d09eae2` on 2026-09-20/21. The retained decision is physical one-based PDF selection, exact CLI parser errors, a separate CLI-only pages/tagged gate and the deliberate PNG/SVG multi-file scope-out. Documentary range errors and CLI range errors are separate observables; do not unify their messages.

P1286's hidden/ignored pdf-standard compatibility acceptance was superseded by P1469's public raw typed list. The owner selected reduced language/selection scope on 2026-09-21; approval followed causal L0-only commits `65e68ad45`/`e14002d55`. CLI standard names, override-as-a-unit and accessible-profile/tagging conflicts are contracts. Historical independent code/test integration and RED-to-GREEN statements belong here, not in an evergreen implementation-status section. They do not certify full structural PDF/A/PDF/UA conformance.

### Human diagnostics, info and init

The original short gcc/clang-style formatter was replaced by the ADR-0045 human-format amendment in P1139, preserving the established-convention rationale. Its old upstream `a51e02804` evidence is historical, not proof against the current pin. Source IDs, caller-provided names, unspanned hints, tab width, trace kinds and color equivalence remain operative contracts.

Info's INFO-1/INFO-2 delivery labels described an incremental human+JSON surface and deferred other upstream formats. Current specification preserves human/JSON only without advertising nonexistent formats. Claims that no consolidated snapshot exists are obsolete. Serialized a11y_extras and custom-ca-configured now exist in the current DTO and are documented with their actual spelling.

Init's INIT-1/INIT-2 implementation chronology separated explicit and omitted versions. Current contracts reference the version resolver directly, preserving preview remote-index versus non-preview local-data selection. The earlier blanket local-cache-first phrase cannot describe versionless preview resolution. Template materialization is an L3 consumer, not shared CLI/main ownership.

### Duration and watch attestation limits

The duration record froze literal presentation and inclusive unit boundaries. P1357 classified its binding sentinel as a harness limitation, not proof of a semantic gap. The consumer and public test now specify exact literals; elapsed runtime is not an input or acceptance metric.

The duration integration test was given its L0/header during a lineage correction after its first GREEN. This order is an attestation limitation: the record cannot be rewritten as proof of fully isolated pre-implementation test authorship. The five-minute boundary record also warned that changing >= to > at an exact integer boundary may be observationally equivalent; meaningful mutation controls use the specified fractional case.

P1295 preserved the public snapshot/wait APIs. P1297/R1's baseline was HEAD `76fb7336311bdb6497456ab5fdc0a8ce355ff39b`, watch.rs:42/:49 and then-main.rs:95/:114/:119. It showed immediate capture before finalization while independent public finalizers still allowed ordering to be expressed incorrectly. The armed capability made finalization consume an already captured baseline.

P1296 found that observing deletion afterwards did not discriminate capture order, because removal and capture could occur before the parent's next poll. A subsequent FIFO control timed out during asset recompilation. These are limitations of evidence, not readiness or parity proof. Preserve independent readiness and causal mutation controls; discard historical timeout/test-count metrics from normative specifications.

## Behavioral discrepancies requiring owner decisions

These are source/document disagreements or ambiguities, not changes authorized by translation.

| ID | Prior obligation | Current source evidence | Pending decision |
| --- | --- | --- | --- |
| S-FONTS | Unreadable font paths warn, never panic. | `03_infra/src/fonts.rs:304` returns silently on read_dir failure; `04_wiring/src/fonts.rs:14` receives only inventory entries, without warning publication. | Retain the warning requirement or explicitly approve silence; define any message under the correct productive owners. The L0 retains the unresolved warning obligation. |
| S-UUID | Accepted UUID grammar described canonical or unhyphenated forms. | `02_shell/src/cli.rs:1563` removes all hyphens before checking hexadecimal bytes. | Choose whether noncanonical hyphen placement is intentionally accepted or should be rejected; no parser tightening or silent normative endorsement here. |
| S-TRACE | Multiline trace snippet uses the last non-whitespace character. | `02_shell/src/diagnostic.rs:175` checks the final character only; trailing whitespace suppresses the suffix instead of trimming. | Specify terminal-whitespace behavior, then reconcile the owner/test. The previous obligation is retained as unresolved. |
| S-INFO | Human output contains the same categories as structured output; the pinned nucleus requires category preservation. | `02_shell/src/info.rs:64` prints version/build/features/fonts/packages, but no env section. | Decide human env projection or explicitly clarify category scope through the appropriate nucleus/owner process. This task neither edits the nucleus nor blesses the omission. |
| S-BOOL | The previous pdf-tagged prose called a spaced boolean value invalid. | `02_shell/src/cli.rs:490` uses require_equals; a token after the bare flag may be accepted as positional output, not as its value. | Clarify whether ordinary positional parsing is intended or whether rejection is required. No new restriction is introduced. |
| S-INIT | Existing-destination failure was described as writing nothing. | `03_infra/src/project_init.rs:27` resolves the package before checking destination, so downloading/cache writes can precede refusal. Copying rejects all symlinks, stricter than the earlier escape-only wording. | Confirm whether no-write means destination-only (consistent with current implementation) or forbids cache effects, and whether internal symlinks should ever be allowed. Current safety is preserved; no new permission to follow links. |

The bundle discrepancy is S-BUNDLE: `04_wiring/src/compile.rs:311` forwards HTML mode, PDF stream mode, pages/tagged, PNG PPI and pretty to BundleCompileOptions; `03_infra/src/pipeline/bundle.rs:407`, `:465` and `:539` consume them for HTML/PDF/PNG assets. CLI PDF-standard resolution remains direct-PDF-only. Earlier direct-HTML/PDF-only scope descriptions therefore require reconciliation by the bundle/format owners. Preserve the actual forwarding without silently approving a wider normative contract or changing defaults.

The init owner describes the actual destination-only publication guarantee while this ledger retains the broader historical obligation for review. Likewise parser behavior observations do not erase the UUID/boolean grammar questions.

Legacy raw Content still exposes only the existing fallback field subset. This is retained scope debt, not a new discrepancy solved by documentation or permission to weaken authoritative snapshots. Full PDF conformance and PNG/SVG multi-file output remain outside this sanitation.

## Validation and pending lineage work

Read-only ownership/nucleus checks and canonical-reference checks are appropriate to this change. Product messages, public carriers and test/source fingerprints must remain unchanged. No behavioral RED/GREEN or new upstream parity result is claimed by this documentation-only task.

The read-only `crystalline-lint --checks v15,v26 .` check passed with no violations. Canonical prompt-reference existence checks and `git diff --check` passed for the assigned documents. Consumer, test, ADR and nucleus fingerprints captured above were unchanged after editing. These establish documentation scope/ownership integrity, not resolution of behavioral discrepancies or global lineage health.

Editing an L0 changes its prompt hash. Productive headers cannot be updated under the authorized file scope; targeted lineage resealing and drift revalidation therefore remain pending for a separately authorized operation. No global --fix-hashes is run. Existing Hash do Código metadata is preserved, including any pre-existing source drift; preserving it is not a claim that every consumer already matches it.

The pending behavioral questions above require explicit decisions before any implementation or compatibility/default change. This diagnostic closes neither those decisions nor sanitation outside the assigned shell documents.
