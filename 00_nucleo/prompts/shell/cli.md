# L0 — Shell CLI parsing and semantic serialization
Hash do Código: a0a86112

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/caption-projection.toml sha256:b57c37587932a65caedce0b0df5691463ad336b90126a978ea88a7c369432738
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:09b32666fb1b1fa7ba8f2ef7c979c60cbc64cc515b8d9c46818bec19d7471fb1
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:d9b2ab50748dec1246d8a55c0b702fa2ee5e23dd737bedf3689e6c9f0297ac29
- 00_nucleo/prompts/_nuclei/math-attach-slot-presence.toml sha256:cf64af6ef56faaa1ba19fa32ec6c134dc3f4f16a4a74fde21eca40259cf3b6ba
- 00_nucleo/prompts/_nuclei/math/equation-constructor-origin.toml sha256:947456f05619642c93f67721453e4ad5577557679dbc422a20a48f397ae97ded
- 00_nucleo/prompts/_nuclei/math/matrix-morphology.toml sha256:dced4bcb1e45b09423bb3b9739af4da3cd440d500d6a620e08e13219236d1735
- 00_nucleo/prompts/_nuclei/network/custom-ca-cert.toml sha256:4b9f69ff41790a55a5b2d6a7b9982da858092e4bb2f0c503d6332a14a510e3ec
- 00_nucleo/prompts/_nuclei/raw-field-presence.toml sha256:93054c60b918734522155fc8d1140982c656478ffccbc1ce55b14911309e1a5c
- 00_nucleo/prompts/_nuclei/shell/build-identity.toml sha256:1bd114e8801a530f815767eb90e7861c6a3e4e332a59a175b66c717a44c6ea0f
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:d92a80d952b2cb79d90b96a75a0387cfde2da24f2fd41327de8ab0e647a97f8c
- 00_nucleo/prompts/_nuclei/table/cell-presence.toml sha256:f7d3050154813e1ea46c26e235383ae337781a4b28d10ef3248f6eaec96ce221
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:2b9e828f5e0d958f4328d98a35320ba2ea321997e541e6ab90520d2e8811e7bf

**Layer:** L2
**Exclusive consumer:** `02_shell/src/cli.rs`


## Evidence, ownership and classification

The current consumer defines color/output enums at `02_shell/src/cli.rs:199`, command arguments at `:415`, command variants at `:550`, public intents at `:698`, translation at `:830`, semantic serialization at `:1021` and resolution helpers at `:1550`. These observations precede the contracts below; they do not establish new product decisions or global vanilla parity.

L2 owns syntax, argument translation, presentation choices and memory-only serialization. It reads argument/environment/TTY configuration but does not inspect filesystem paths, discover fonts, compile, lay out or export. L4 consumes typed `RunIntent` values rather than clap's private `Args`. Raw tri-state/optional overrides must remain distinguishable so their productive owner can combine CLI and document options.

CLI acceptance, diagnostics and serialized public value morphology are observables. Internal enums, arithmetic, generators and pipeline mechanics need not match vanilla. An observed parser quirk is not, by itself, evidence of intentional language design. Unresolved normative/consumer disagreements are retained in `lab/results/construction/diagnostics/sanitization-shell-history.md`; this translation authorizes no change of behavior.

## Commands, compatibility, help and identity

The command tree is named `typst`, with about literal `Typst compiler (crystalline)`. It exposes `compile` (accepted alias `c`), `watch` (visible alias `w`), `eval`, `fonts`, `completions`, `info` and `init`. `query` remains accepted but hidden from principal help; it is deprecated, not an alias of eval.

Preserve `typst INPUT [OUTPUT] ...` as implicit compile. The normalizer scans argument tokens for recognized command/alias names and `-h`, `--help`, `-V`, `--version`; if any is present, it does not insert compile. Otherwise it inserts `compile` after the executable name. Empty argument vectors are left unchanged. Do not describe this implementation as inspecting only the first positional token; collisions with command-looking argument values are not permission to change it.

`parse() -> RunIntent` uses clap's parser and translates the selected command. Help/version terminate successfully with exit 0; malformed/unknown arguments and closed-enum values exit 2 with clap diagnostics before productive execution. Missing compile input has the diagnostic body `the following required argument was not provided: <INPUT>`.

`command() -> clap::Command` returns the same derived tree used by parsing, including hidden commands, for completion generation.

Public version derives from `PARITY_VERSION`, not the workspace crate version, and appends ` ({short_commit})`. Compile-time `TYPST_COMMIT_SHA` supplies the first eight bytes (or shorter full value); absent identity displays `unknown commit`. `build_commit() -> Option<&'static str>` preserves the complete compile-time identity. Build capture belongs to `00_nucleo/prompts/shell/build.md`; parity version belongs to `00_nucleo/prompts/core/entities/version.md`. No runtime Git lookup or assumption that a version string proves oracle provenance is permitted.

## Global options and configuration

`--color auto|always|never` is global, default auto. `ColorWhen` is the closed public enum Auto/Always/Never. `resolve_colored_with(choice, no_color, is_tty) -> bool` applies explicit always/never first; in auto, presence of `NO_COLOR`, even empty, disables color; otherwise stderr TTY status decides. The formatter itself never discovers this state.

`--cert PATH` is global. Resolution is explicit flag, then `TYPST_CERT` presence, then no custom CA. Preserve optional path presence rather than treating an empty value as automatically absent. Pass the selected path to commands that resolve packages/world configuration; do not make it an unused decorative flag or disclose certificate path/bytes in info. TLS interpretation belongs to the pinned custom-CA nucleus and productive I/O owners.

Features are explicit repeatable/comma-delimited `--features html,a11y-extras,bundle` on compile/watch and eval only. Unknown names exit 2. Default is an empty set; repetitions/order are idempotent. Each name enables only its corresponding typed feature. Selecting HTML/bundle output does not implicitly enable that feature. `TYPST_FEATURES` is not a CLI activation source.

## Compile and watch common arguments

`typst compile INPUT [OUTPUT] [OPTIONS]` and `typst watch INPUT [OUTPUT] [OPTIONS]` share:

| Argument | Current parsing/resolution contract |
| --- | --- |
| `-o FILE`, `--output FILE` | Flag wins over positional OUTPUT; otherwise derive input.with_extension("pdf"), except output-less explicit bundle uses input.with_extension(""). |
| `--root DIR` | Explicit flag wins over `TYPST_ROOT`, then nonempty input parent, then `.`. No path-existence validation in L2. |
| `--font-path DIR` | Repeatable; split each value and `TYPST_FONT_PATHS` on `:` on Unix or `;` on Windows. Explicit flag occurrences replace the whole environment list, not append to it. Filter empty slots; preserve order and nonempty duplicates. |
| `-f FORMAT`, `--format FORMAT` | Closed pdf/png/svg/html/bundle enum. Explicit format wins over the selected explicit output's case-insensitive pdf/png/svg/html extension, then PDF. A .bundle extension alone does not select bundle. |
| `--full-error` | Boolean, default false; pass it through for the extended show-rule recursion hint. |
| `--timings-json FILE` | Optional path, absent None; productive owner writes pipeline timings. |
| `--document-id UUID` | Explicit flag wins over `CRYSTALLINE_DOCUMENT_ID`, then None. Convert accepted UUID to 16 bytes in L2. |
| `--input chave=valor` | Repeatable raw key/value strings; split on the first equals sign. Nonempty key required; empty value and additional equals signs are allowed. Default empty vector. |
| `--compact` | Boolean, default false: verbose PDF remains production default; true selects compact PDF. Direct PNG/SVG/HTML outputs are unaffected; bundle PDF forwarding is recorded below. |
| `--pretty[=true|false]` | Tri-state Option<bool>: absent None, bare true, explicit equals value parsed as bool. Never erase absence in L2. |
| `--ppi PPI` | Option<f64>, default None, not a pre-applied 144. Ordinary f64 syntax; productive owner decides meaningful resolution/validation. |
| `--pdf-tagged[=true|false]` | Tri-state Option<bool>, absent None, bare true; explicit value uses equals. |
| `--no-pdf-tags` | Legacy boolean, absent false; explicit modern pdf-tagged choice wins when both occur. |
| `--pdf-standard LIST` | Closed comma-delimited PDF-standard names, translated to raw Vec<PdfStandard>; absent empty. Do not validate cross-name conflicts in L2. |
| `--oracle-pdf` | Boolean, absent false; diagnostic-only PDF path, not a production output format. |
| `--pages RANGES` | Optional comma-delimited physical one-based page ranges, converted to typed PageRanges; absence stays None. |

An explicitly selected non-bundle format does not change the default output filename's .pdf extension. Format selection and filename derivation are distinct contracts. Standard output/path interpretation belongs to the publication owner, not these pure path helpers.

Font-path empty values are legitimate slots, unlike a flag missing its required argument. An explicitly empty flag still replaces an environment list before filtering. L2 does not convert empty slots to cwd. Unreadable-path warning conformance is a recorded pending decision, not repaired here.

UUID's documented supported forms remain canonical 8-4-4-4-12 hexadecimal and unhyphenated hexadecimal. The consumer also strips all hyphens before checking length/digits, accepting noncanonical hyphen placement. Do not tighten or endorse that broader grammar without the recorded behavioral decision. Rejected values exit 2 with the exact literal:

```text
error: invalid document ID: expected a UUID (e.g., f81d4fae-7dec-11d0-a765-00a0c91e6bf6)
```

Malformed input pairs exit 2 with `error: invalid input '{raw}': expected the form key=value`. Do not parse values as numbers/booleans or silently deduplicate the raw vector; SystemWorld owns construction of sys.inputs.

For pretty/pdf-tagged, a following space-separated token is not the optional boolean value: it is parsed under ordinary positional rules. This differs from the prior unconditional claim that spaced pdf-tagged values are rejected; resolving that claim is pending, not a reason to alter the parser.

## Page ranges and PDF standards

Range tokens accept N, A-B, A- and -B, with endpoint whitespace trimmed and positive integer physical indices. A single page is an inclusive equal-bound range; no counter/label remapping occurs. Preserve these parser messages:

| Condition | Message |
| --- | --- |
| Empty token | `page range must not be empty` |
| Literal zero | `page numbers start at one` |
| Invalid endpoint | `not a valid page number` |
| Both endpoints absent | `page range must have start or end` |
| Descending interval | `page range must end at a page after the start` |
| Multiple hyphens | `page range must have a single hyphen` |

Accept standard names exactly: `1.4`, `1.5`, `1.6`, `1.7`, `2.0`, `a-1b`, `a-1a`, `a-2b`, `a-2u`, `a-2a`, `a-3b`, `a-3u`, `a-3a`, `a-4`, `a-4f`, `a-4e`, `ua-1`. Unknown names exit 2. Preserve typed versions, archival/accessibility labels and raw order.

PDF composition combines CLI overrides with document options: explicit CLI values precede document settings, which precede defaults. Preserve None/empty overrides so document settings remain observable. Modern tagged overrides legacy no-pdf-tags; absent both delegates to document/default. A nonempty standard override replaces the document's whole set rather than forming a union. With no document override, pretty is false, PNG PPI is 144, PDF version is 1.7 and automatic PDF tagging resolves true unless page selection or standard constraints require otherwise. Document-option precedence belongs to `00_nucleo/prompts/core/entities/document_format_options.md` and the format-specific pipeline owners, not L2.

For PDF, pages plus explicit tagged=true exits 1 before compilation with `error: cannot enable PDF tags when exporting a page range`. Page selection without an explicit tagging decision implies false. Accessible standards a-1a/a-2a/a-3a/ua-1 cannot resolve to untagged output; retain `error: cannot disable PDF tags when exporting a {name} document` and, when applicable, ` = hint: using --pages implies --pdf-tagged=false`. Conflict validation belongs to the domain standard resolver and productive composition, not clap.

This is a language/selection contract, not certification that every selected archival/accessibility profile satisfies full PDF/A or PDF/UA conformance. Pages/standards/tagging/oracle are neutral for direct PNG/SVG/HTML outputs; PNG/SVG multi-file page selection is not promised. The bundle branch forwards pages, tagged, compact, PPI and pretty to relevant generated assets, while its CLI standard override remains absent because the resolver runs only in the direct PDF branch. This exceeds older PDF-only descriptions and requires the recorded scope decision; it is not silently ratified here.

## Command-specific options

Compile alone accepts `--html-serialization crystalline|vanilla`, default crystalline. It selects the HTML escaping profile, not target or feature. The prior requirement limits effect to direct HTML output; current composition also forwards the choice to bundle HTML assets. That scope disagreement remains pending in the diagnostic, without granting a new default or changing either path. Vanilla selection is explicit; it is not a silent new default. Watch does not accept this option and carries Crystalline.

Watch adds `--no-serve` and `--no-reload` (false by default), plus `--port PORT` (Option<u16>, absent None). Serving/reload/port search are owned by `00_nucleo/prompts/infra/watch.md` and cycle composition by `00_nucleo/prompts/wiring/watch.md`; L2 starts no server.

`typst eval EXPRESSION [--in INPUT] [--format json|yaml|raw] [--pretty] [--features ...]`: expression is code text, format defaults json, pretty defaults false, input defaults None. Contextual input uses `--in`, not another positional; `--in -` lets L4 read stdin. Preserve ordinary JSON/Raw behavior with or without document context. No eval --target/root/font-path/sys-input options are accepted by this consumer.

`typst query INPUT SELECTOR [--field FIELD] [--one] [--format json|yaml] [--pretty]`: JSON default, absent field None, one/pretty false. INPUT `-` selects stdin in L4. It has no features or --in argument. Its deprecation warning is emitted by L4 after compilation diagnostics or successful serialization. The pure query_deprecation_hint function provides the specific eval replacement; acquisition/selection/serialization failures retain their measured boundaries. The warning message stays `the `typst query` subcommand is deprecated` and the generated hint follows the ratified contract below.

`typst fonts [--font-path DIR ...] [--ignore-system-fonts] [--variants]`: font-path handling is shared; include_system is the inverse of ignore-system-fonts, default true; variants false. Presentation belongs to `00_nucleo/prompts/shell/fonts.md`.

`typst completions SHELL` requires one of bash/elvish/fish/nushell/powershell/zsh. The typed public selector is `CompletionShell`; generation belongs to `00_nucleo/prompts/shell/completions.md`.

`typst info [-f json|--format json] [--pretty]` has no format by default (human); pretty false and requires format, otherwise exit 2. No unsupported info format or feature selection is advertised. Presentation belongs to `00_nucleo/prompts/shell/info.md`.

`typst init TEMPLATE [DIR]` carries template text, optional directory and selected cert path. Materialization belongs to `00_nucleo/prompts/infra/project_init.md`.

## Public carriers and pure helpers

Preserve the closed `RunIntent` variants Compile/Watch/Eval/Query/Fonts/Completions/Info/Init and their corresponding public structs. They remain data, not process handles, trait objects or executable callbacks.

`CompileIntent` fields are features, input, output, output_format, html_serialization, root, font_paths, colored, full_error, timings_json, document_id, inputs, compact, pretty, ppi, pdf_tagged, no_pdf_tags, oracle_pdf, cert_path, pages and pdf_standard. Preserve their current path/domain/boolean types, Option distinctions and vectors. `WatchIntent` wraps compile and no_serve/no_reload/port.

`EvalIntent` contains expression/input/format/pretty/colored/cert_path/features; `QueryIntent` input/selector/field/one/pretty/format/colored/cert_path; `FontsIntent` font_paths/include_system/variants; `CompletionsIntent` shell; `InfoIntent` format/pretty/cert_path; `InitIntent` template/directory/cert_path. Do not introduce a field or change its public type during documentation sanitation.

`OutputFormat` is Pdf/Png/Svg/Html/Bundle; `EvalFormat` Json/Yaml/Raw; `QueryFormat` Json/Yaml; `InfoFormat` Json; `HtmlSerialization` Crystalline/Vanilla. Preserve public Copy/Eq/Debug/ValueEnum traits where currently implemented.

Keep pure resolution APIs and their argument types: `resolve_colored_with`, `resolve_output_with`, `resolve_output_format_with` and `resolve_root_with`. The output-format helper takes an explicit default supplied by its caller; compile supplies PDF. Parsing/world validation stays separate.

## Eval/query serialization

Public serialization entrypoints remain `serialize_eval`, `serialize_query` and `serialize_query_with_format`, returning Result<Vec<u8>, String> without I/O. The legacy query wrapper delegates to JSON. Share semantic projection rather than formatting Rust Debug.

Raw accepts only Value::Str and Value::Bytes, returning their exact bytes without newline. Other values yield this exact body, with a newline between its lines:

```text
cannot print {public_type} in raw format
hint: `--format=raw` only supports strings and bytes
```

Structured projection preserves none as null, booleans, integers, finite floats, strings, effective symbol graphemes, recursive arrays and insertion-ordered dictionaries. Nonfinite floats become null in JSON and .nan/.inf/-.inf in YAML. Bytes become the string `bytes(N)`, not a byte array. Other nominal public values use the core's morphological repr serialization, never Rust Debug, nominal erasure or a shell-owned duplicate repr table. Stroke serialization is covered by this same public repr contract.

JSON is compact or pretty according to the selected flag and ends with newline. YAML recursively emits valid structured values with two-space indentation and a final newline; pretty does not change YAML values/output. String quoting must preserve values and distinguish scalar-like strings from booleans/null/numbers.

LocatedContent with fields present uses its authoritative ordered snapshot recursively, adding func with the element's public name. It must not synthesize extra render fields, fall back field-by-field, leak selector metadata as an explicit label, or serialize locations. Absent snapshots and raw Content use the existing legacy public projection; an incomplete legacy fallback is not a promise to expose every element's complete language fields.

Legacy Content projection preserves func and its public recursive fields. Label wrappers add the public label notation only to the projected object. Important cases:

- Text: text; Sequence/MathSeq: children; Paragraph/SmallCaps: body; ordinary Styled: child and styles.
- Heading: level and depth equal level, offset 0, numbering null, supplement obtained from the core heading-language owner with None, outlined, bookmarked as bool or auto, hanging-indent auto, and body. Do not duplicate the English supplement table or infer another locale without a style chain.
- Figure: body, alt null, placement null, scope column, kind (default image), caption, supplement, numbering 1, gap 0.65em, outlined true and its kind-specific counter repr. Caption includes separator content `: `, body, kind, supplement, numbering and counter. Preserve the existing Figure/Table/Listing supplement selection.
- Equation: block, numbering null, number-align end + horizon, supplement Equation, alt null and body.
- MathAttach: func attach, base plus public t/b/tl/bl slots. Primary t/b use the corresponding right slot only when Omitted; ExplicitNone blocks fallback and serializes null; Present serializes even empty content. Omitted fields are absent, not null. Do not infer omission from empty content or collapse it with explicit none.
- Shape: its rect/ellipse/line/polygon func, width/height as auto or serialized lengths, fill and optional stroke. MathText has func text; MathIdent has func symbol.
- Metadata: value; Quote: block/quotes/body/nullable attribution. Other legacy elements expose the current body fallback only when available.

Heading supplement policy belongs to its L1 owner, not an English successor literal in L2. The pinned content-snapshot and math-slot nuclei remain authoritative; this prompt must not weaken them to bless legacy fallback gaps.

Query projects the same public values. With --one, require exactly one element before looking up --field; otherwise `expected exactly one element, found {N}`. A missing field for that single result is `no such field found for element`. Without --one, a field request filters out elements lacking the field and emits the remaining values as an array; without field, emit an array of projected elements. Apply JSON/YAML serialization after cardinality/field selection.

## Exhaustive content projection boundaries

At clean HEAD `5e1a01066b9eef0eb3c478e8b49e15e1da15b0b9`, inspected on
2026-10-19, `02_shell/src/cli.rs:1152-1177` returns through Source, ShowRecipe,
Located and Realized before processing Label. Its legacy field dispatch at
`:1178-1418` preserves specialized fields and otherwise probes only `body`.
The name projection at `:1421-1446` overrides selected public names and delegates
the remainder to core. `01_core/src/entities/content/fields.rs:156-265,316-429`
and the delegated getters show that a stored Rust body is not sufficient to
expose a public body. Transform's getter at `elements/transform.rs:101-105`
withholds even body for Matrix; Dynamic delegates the field inquiry. These are
source observations, not new upstream parity measurements. The audit and
preservation matrix live in
`lab/results/v16-corrections-2026-10-19/batch-e/agents/serialization/`.

Preserve these language projections with nominal exhaustive Content patterns:
the initial dispatch returns through the same four carriers (Realized uses
visual, never original), while every named ordinary variant continues to Label
handling or field projection. Label appends its notation after its body's
projected fields; no carrier metadata or recipe execution is introduced.

In the legacy field decision, retain every specialized branch. The residual
body-probe class is Boxed, Block, Transform, MathStretch, MathLr, MathUnderbrace,
MathUnderbracket, MathUnderparen, MathUndershell, PdfArtifact and Dynamic. Keep
the optional `get_field("body")` lookup and recursive value conversion, including
Transform Matrix's absence and Dynamic's conditional presence. Explicitly name
the remaining func-only variants and separately identify the already-handled
Source/ShowRecipe/Located/Realized/Label cases. Do not infer fields from Rust
members, enumerate all language fields, add panics, or claim completeness for
legacy omissions. A newly exposed promised field requires its own measured
correction; it is not silently ratified as absent by this classification.

Canonical naming retains its current overrides, exhaustive ShapeKind decision,
and core `elem_name()` delegation for explicitly named remaining variants.
Transform's operation-sensitive names and Styled's constructor-heading identity
remain core-owned. No shell-owned duplicate name catalogue is authorized.
Pinned upstream `bd5ebec285505889e8ff1d4f68804ab147a699f3` declares required
Styled child/styles at `lab/typst-original/crates/typst-library/src/foundations/content/mod.rs:759-768`;
`:584-591,710-719` serialize known fields. The current-pin JSON/YAML controls
`serialize-styled-json` and `serialize-styled-yaml` in the language agent's
`supplement-receipt.json` confirm this morphology. This corrects only the earlier
summary saying `Styled: body`; ordinary Styled remains child/styles.

Some snapshots, including empty and deliberately different known fields, remain
authoritative and bypass all legacy fields. Preserve Empty, Raw, matrix, cell,
caption, entry and typed attachment-slot contracts above. Regressions must cover
nested carriers, Label ordering, Realized visual/original distinction, residual
body and func-only examples, conditional Transform body, canonical names,
snapshot presence, JSON/YAML projection, query field filtering, cardinality
before field lookup and exact missing-field diagnostics. Existing morphology
tests remain required. Compiler-negative probes must independently identify each
of these three decisions. In disposable complete workspace copies, remove the
positive Source arm from the initial carrier decision and from the function-name
decision, and remove the positive Raw arm from the legacy field decision. Each
baseline wildcard must absorb its omission; each repaired nominal complement
must produce a primary E0004 at that exact decision. Preserve the exact original
and mutated sources and independently audit spans. This paired omission proves
the local exhaustiveness brake without changing the productive enum or API;
it is not a compiled new-variant experiment across the shell's core dependency.
Functional preservation tests are expected to pass before and after this
behavior-preserving correction.

The explicit classification is mechanical; its acceptance preserves language
morphology and diagnostics under ADR-0107/0108. A variant whose getter or public
name contradicts its class refutes that classification. This is continuous
ADR-0127 flow, with no public signature, mode/default, phase or compatibility
change. Ownership remains 1:1; the constructor correction below adds its shared nucleus pin.

## Equation constructor transport serialization

The current-pin constructor and external-Styled observations in the language
receipts above, and upstream Equation's declared fields at
`lab/typst-original/crates/typst-library/src/math/equation.rs:62-131`, distinguish
an option-bearing Equation from an external Styled. The current shell's Styled
branch at baseline `cli.rs:1408-1411` otherwise serializes either as child/styles.

For valid marked equation constructor transport only, use a private structural
classifier following the pinned constructor-origin nucleus; never infer this
class from elem_name. Project known getter fields in order
`block,numbering,number-align,supplement,alt,body` with recursive value conversion.
Do not synthesize omitted defaults or emit constructor metadata, child or styles.
Keep the existing bare Equation legacy branch and its separately scoped default
expansion unchanged; this correction does not certify that branch's full parity.
External/unmarked Styled remains child/styles, even around marked transport.
Some snapshots still bypass legacy projection. Add RED→GREEN controls for known
constructor fields, optional absence, recursive body, JSON/YAML and query fields,
plus opaque external Styled and invalid markers. No new public API or phase.

## Command output integration

Productive composition is referenced at `00_nucleo/prompts/wiring/compile.md`, `00_nucleo/prompts/wiring/eval.md`, `00_nucleo/prompts/wiring/query.md` and the command-specific shell/wiring owners; those are not additional consumers of this L0.

Successful finite commands exit 0. Language evaluation/compilation errors, invalid query cardinality/fields and structured/raw serialization errors exit 1. Argument errors and operational source/stdin/stdout/timings failures exit 2. Watch is continuous: failed compilation reports status/diagnostics and waits for another change rather than being described as a successful finite exit.

Preserve `error: {error}` for eval/query serialization failures; `error: failed to write eval output: {error}` and `error: failed to write query output: {error}` for stdout failures. Preserve existing source-load/stdin diagnostics under their productive owner.

HTML/bundle export warnings, feature-required errors, page/tag/standard conflicts and full-error hints retain their source-produced literals. No translation of specification prose authorizes translation, suppression or rewording of these messages.

Preserve these experimental warning bodies and their final blank line; each is emitted for its corresponding target, not as a combined warning:

```text
warning: html export is under active development and incomplete
 = hint: its behaviour may change at any time
 = hint: do not rely on this feature for production use cases
 = hint: see https://github.com/typst/typst/issues/5512 for more information

warning: bundle export is experimental
 = hint: its behaviour may change at any time
 = hint: do not rely on this feature for production use cases

```

Missing feature selection exits 1. Preserve the diagnostic bodies below, including literal backticks. HTML retains hints `html export is under active development and incomplete` and `see https://github.com/typst/typst/issues/5512 for more information`; bundle retains `bundle export is experimental`:

```text
html export is only available when `--features html` is passed
bundle export is only available when `--features bundle` is passed
```

Bundle stdout refusal remains `error: cannot write bundle to standard output`; watch stdout refusal remains `error: watch requires a file output; stdout is not supported`. Do not substitute implicit feature activation for these errors.

## Acceptance and exclusions

Check command/alias inventory, hidden query, legacy invocation, help/version status, each closed selector, global color/cert precedence, font empty-token behavior, output/root/format precedence and all optional/tri-state carriers. Cover accepted and rejected input/page/UUID forms without silently resolving the recorded grammar discrepancies.

Check structured nested values, nominal repr, symbol/bytes/nonfinite handling, authoritative snapshots, labels, typed math-slot presence, generic query fields/cardinality and raw byte preservation. Check successful outputs and failure channels/status through the independently owned command envelope tests.

Do not assert global shell, HTML/bundle, query fallback or PDF-standard conformance. Unsupported argument families remain unadvertised. Public contract/default/pipeline/compatibility changes require ADR-0127; ownership and canonical links follow ADR-0129/0130.

## Historical evidence and current consolidation boundary

The original cumulative clauses, measurements, approvals and execution receipts
are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/shell/cli.md`.
The contracts below consolidate that material by responsibility. Measurements
against `9f2b6e8715237cb086899a42873660fe744622e8` remain evidence for that historical
snapshot; they do not certify the current ratified upstream pin. Diagnostic
paths below retain the recorded HEAD, uncommitted diff, UTC, source locations,
arguments, binary hashes and outputs. No new measurement or behavioral approval
is asserted by this consolidation.

## Bundle PDF-standard override: pending product decision

### Evidence

`lab/results/construction/diagnostics/pdf-standard-bundle-2026-09-30/results.json`
records timestamp, binary/input SHA-256 and commands for the debug working tree
at HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`. Without the flag, a missing title
compiles in direct PDF and bundle `document("one.pdf")`. With
`--pdf-standard=ua-1`, direct PDF fails with `PDF/UA-1 error: missing document title`,
while bundle publishes `one.pdf`. `04_wiring/src/compile.rs:155-219` constructs
the override only for `OutputFormat::Pdf`; `:305-325` already transports it to
`BundleCompileOptions`, and `03_infra/src/pipeline/bundle.rs` resolves each PDF
entry. The inferred missing CLI connection would be refuted if the flag changed
the internal PDF's effective standard; the UA-1 probe did not show this. This
is output-selection semantics, not PDF-byte equality.

### Pending contract and acceptance

The following remains a proposal, requiring owner confirmation under ADR-0127:
a nonempty `--pdf-standard LIST` would select the same explicit set for every
PDF generated inside a bundle, overriding each document's `format.pdf(standard: ...)`.
Absence would retain document/default selection per entry. HTML/SVG/PNG/assets
would be unaffected. Conflicts within the supplied list would fail before
`SystemWorld` construction for direct PDF and bundle, even a bundle without PDF.
The CLI-only accessible-standard/tag check would remain direct-PDF-only; each
bundle PDF would resolve and validate options in L3 without prematurely rejecting
non-PDF entries. Publication would be all-or-nothing on an entry failure.

Proposed acceptance: preserve current output without the flag; UA-1 without a
title fails without publishing any file; with a title, the internal PDF declares
UA-1; non-PDF entries receive no standard. No implementation is authorized until
the owner confirms. Then replace the earlier absent-override clause and reseal
the owning pair before RED→GREEN. This pending clause does not silently ratify
the broader bundle forwarding discrepancy described above.

## Stdin requires an explicit output

### Evidence

`lab/results/construction/diagnostics/plano-debitos-2026-10-06/before.json.gz`
retains HEAD/working tree/UTC, sources, commands and binary hashes. Compile/c
with input `-` and no output wrote `-.pdf` in Tekt; the historical pin required
OUTPUT and exited 2. Upstream `args.rs:351-355` declares `required_if_eq(input, -)`,
evidence of intent; shell `cli.rs:922-935` derived output instead. Implicit `-`
invocation is an established Tekt extension, rejected as a subcommand by vanilla,
and is not itself a parity probe. `final.json.gz` measured an initial usage
presentation that omitted the executable name and displayed required arguments
as optional.

### Contract and acceptance

Before resolving format/output or constructing `RunIntent`, input exactly `-`
with neither positional output nor `-o`/`--output` exits 2 with diagnostic body
`the following required arguments were not provided:\n  <OUTPUT>`.
Use the existing clap tree for usage/help; acquire no stdin, World or file in L2.
Apply this to c, compile and the supported implicit form, including pdf/svg/html/
bundle. Either output form suffices; the flag retains precedence in either
argument order. Physical input still derives output. Public signatures, flags
and helpers remain unchanged.

Construct this error from a clone of the existing subcommand, with `bin_name`
`typst compile` and input/output required only on that error-presentation tree.
Require `Usage: typst compile` and `<INPUT> <OUTPUT>`. Preserve the actual parsing,
help and completion tree and accepted forms. Do not invent unsupported oracle
options such as creation-timestamp. Preserve raw stderr evidence; differing flag
surfaces prevent a claim of complete envelope identity.

Black-box controls reject absent output with empty stdout and no `-.pdf` or
bundle tree; accept positional/flag/stdout output, check flag precedence in both
orders, and preserve physical-input defaults. This parity correction follows
continuous ADR-0127 flow.

## Query replacement hint

### Evidence and approval

The query-clean/query-warning/query-error probes in
`plano-debitos-2026-10-06/before.json.gz` compare historical pin `9f2b6e871`:
Tekt's constant `query(...)`/`--in ...` hint differed from the exact expression
built by upstream `query.rs:126-161` using field/one and shell escaping. This is
L2 presentation. Human `confirmo` on 2026-10-06 approved the candidates and hashes
in `lab/history/construction/records/00_nucleo/materialization/warnings-cli-aprovacao-2026-10-06.json`.
`shell-escape 0.1.5 src/lib.rs:19-24` reads MSYSTEM in its non-Unix wrapper;
that measurement requires the pure platform-specific helpers below.

### Contract

`pub fn query_deprecation_hint(intent: &QueryIntent) -> String` is pure and
returns `use `, the replacement command in literal backticks, and ` instead`,
without warning/hint prefix, newline or I/O. Start with `query(selector)` using
the selector text. No one/field leaves that expression unchanged; one appends
`.first()`; field alone appends `.map(it => it{access})`; both append
`.first(){access}`. Access is `.FIELD` when `compiler::lexer::is_ident(field)`,
otherwise `.at(REPR)`, using the existing `Value::Str` representation through
`repr_value_for_serialization` and preserving escapes.

Use L2 `shell-escape =0.1.5`, the upstream pin: `unix::escape` under cfg(unix),
`windows::escape` otherwise, without reading environment. Unix is measured;
MSYS/Windows conformance is not. Input `-` adds no `--in`; a file adds `--in`
and its existing upstream display representation. Preserve the path-with-spaces
probe; this approval does not authorize hypothetical repairs of upstream hint
ambiguities. Parser, QueryIntent, value serialization, enums and environment
remain unchanged. Use only received data and pure utilities.

## Semantic smart quotes

### Evidence and approval

`smartquote-2026-10-06/{before,fields-before,partial-and-html,proposal-controls}.json.gz`
records historical pin `9f2b6e8715237cb086899a42873660fe744622e8`, binary SHA-256,
HEAD/working tree/diffstat/UTC, sources, argv/cwd/environment and stdout/stderr.
PDF observations use extracted text; HTML uses DOM/text and diagnostics.
Markup was converted to Text in eval, losing double/enabled fields; a variable
retained English glyphs under German/French or enabled:false; primes 6'/2"
became quotes. Native-function show and the public name had separate successful
repairs before the semantic gate, but markup was still not SmartQuote.
Vanilla's empty constructor has empty fields; explicit double:false is known,
omitted double is Unset, and enabled:false retains SmartQuote. Filtered show and
set double:false have positive controls. Literal HTML worked through Text while
programmatic SmartQuote was rejected. Literal titles rendered quotes; script
required strings and rejected Content before export in both binaries.

Sources: `markup_stream.rs:79–128` selected glyphs during eval; layout
`smartquote.rs` toggled two flags without preceding text; `stdlib/text/smartquote.rs:112–117`
deleted disabled elements; `observation/elements.rs:242–250` compared existing
fields. Vanilla `typst-eval/markup.rs:141–146` constructs explicit double;
`typst-layout/inline/collect.rs:196–211` resolves style/context;
`typst-library/text/smartquote.rs:119–158` distinguishes primes, apostrophes and nesting.
Human approval on 2026-10-06 ratified public double bool→Option<bool>,
enabled:Option<bool>, the pure helper/context and glyph selection eval→render.
Only conflicting quote clauses are superseded; other ADRs/nuclei/proposals survive.

### Owner responsibility and boundaries

Serialize `Content::SmartQuote` as canonical `smartquote`, including Some field
snapshots, without changing its internal elem_name. The name-only repair did not
certify field presence; the ratified semantic contract does. JSON/YAML emit only
Some properties; preserve omitted None, explicit markup double Some(bool), and
programmatic enabled:false without changing type. Use existing canonical Quotes
Value types; do not include inherited Styles properties or fabricate defaults.
Captured fields remain exact.

The domain preserves SmartQuote identity/fields in eval, then selects glyphs in
each target's rendering composition. Paged uses its owner; HTML remains semantic
and cannot call paginated layout for quotes (ADR-0128). Context selection uses
one pure L1 helper with per-flow/compilation state, no I/O, globals, vtable,
property registry or copied upstream memoization algorithm. Stack/state is a Tekt
mechanical choice; acceptance observes language, known/omitted fields, glyphs
and style. HTML escaping remains with its owners. No new modes/flags, unsafe,
watch API or math-phase change is authorized. General Styled morphology remains
independent debt measured by eval-markup-disabled; this contract certifies no
global equality for that case. Use exclusive ownership, pinned invariants and
RED→GREEN.

## Matrix and table-cell public morphology

### Evidence

`lab/results/construction/diagnostics/matrix-morfologia-2026-10-06/before.json.gz`
records HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree, UTC,
diff, sources/binaries and historical pin `9f2b6e8715237cb086899a42873660fe744622e8`.
Raw/query Mat emitted `{func:math.matrix}` without rows; getters/methods lost
fields and func-equality was false. Upstream `math/matrix.rs:91–221` declares
properties/rows; Tekt `math_matrix.rs:45–52` retains rows/explicit but default
Element.get_field, absent mat enumeration at `content_methods.rs:151–191` and
body fallback at `cli.rs:1154–1348` lose morphology.

`table-cell-realizacao-2026-10-07/{before.json.gz,presence-refuters.json}` retains
the same HEAD, working tree/diffstat/UTC, binaries/arguments and pin. Upstream
`model/table.rs:734–778` has Smart/Option; Tekt `table_cell.rs:40–51` retained
only Option values, collapsing x:auto/omission and fill:none/stroke:none presence.
Refuters allow auto in x/y/fill/align/inset/breakable, reject it in colspan/
rowspan/stroke, and permit none only in fill/stroke. These are language
semantics/morphology; no Rust structure, PDF-byte or inferred-intent obligation
follows. Human approval on 2026-10-07 covers public per-field/new-type changes;
receipt: `table-cell-presenca-aprovacao-2026-10-07.json` in historical records.

### Contracts and acceptance

MathMatrix serializes func `mat` and only known fields in order
`delim,align,augment,row-gap,column-gap,rows`. Use the domain getter and existing
recursive `value_to_semantic`; preserve text/symbol/sequence cell forms. Getter
padding Empty becomes sequence with children []. No plain_text body conversion
or baked render/default fields. Preserve JSON/YAML, cardinality, Some snapshots
and diagnostics. Require RED→GREEN and bilateral fields/has/at/func, JSON/query,
padding/presence controls, build/lint, and smartquote/related morphology/query
regressions preserved in the archive. Public types/traits/signatures and phase
remain unchanged under continuous ADR-0127. Mat selectors, render, general Styled
and unmeasured elements are not certified.

TableCell JSON/YAML projects the ten known fields in order
`body,x,y,colspan,rowspan,inset,align,fill,stroke,breakable`, using Content.get_field
and recursive value_to_semantic; func is `cell`. Omitted is absent, auto uses
canonical repr, explicit none is null. Some snapshots remain authoritative;
never invent resolved properties. Require RED→GREEN for body/presence/methods/
JSON, clone/map/observation, spanned rejections, build/suites/lint, exact provenance,
new refuters and existing table/raw/matrix regression collections named in the
archive. This lexical contract changes no render phase/default; positional/style/
padding/show realization is separately scoped below.

## Table realization and transparent source carriers

### Evidence

`table-cell-realizacao-d08b3-2026-10-07/{before,additional,order,aliases}.json.gz`
records HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree, UTC,
diffstat/status, sources/argv/results and historical pin `9f2b6e8715237cb086899a42873660fe744622e8`;
oracle SHA `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`,
Tekt SHA `7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2`.
Upstream `layout/grid/resolve.rs:204–278` resolves fields, `:1500–1530` rejects
ordered conflicts, `:1665–1703` creates empties and `:1945–1988` derives breakable.
Tekt `grid_placement.rs:91–112` prioritizes explicit cells and `:262–270` knows
only GridCell; `realize/mod.rs:160–200` visits recipes bottom-up; `table.rs:127–152`
unwraps without attributes. Order controls observe no cell effects when a table
is replaced and no heading effects when hidden by a cell; aliases observe
`LOCAL a`, `OUTER LOCAL a`, `NEW a`. Unknown fields, hidden effects or wrong order
would refute the language hypothesis; cache/pagination equivalence is not inferred.

`{after-second,refuters-second}.json.gz` in the same diagnostic retain full
provenance. after-second/resolved emitted body func:text without text:
`content_to_semantic:1153–1182` did not unwrap ShowRecipe although
`entities/content/fields.rs:21` already delegated fields.
`table-origens-d08b3c2-2026-10-07/baseline.json.gz` retains source/HEAD/tree/UTC/
argv/binaries/pin for Source transparency. Human `confirmo` on 2026-10-07 is
recorded at `lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/contract.md`.

### Ratified realization boundary and transparency

Historical ratification receipts resolve the old candidate-status wording:
`lab/history/construction/records/00_nucleo/materialization/table-cell-realizacao-d08b3-2026-10-07.md:3`
records owner `comfirmo` on 2026-10-07, manifesto
`5fec42835e38e9a28ee2ea2714b30135f069f111c140bb5e918a0902a06cc82b`;
`lab/history/construction/records/00_nucleo/materialization/table-cell-isolada-d08b3b-2026-10-07.md:3`
records owner `confirmo`, materialized status and manifesto
`3375b0a09a4d39d02c2f0bafbcaf3016c6afde27acdf632331da7a98de31e1df`.
These approvals supersede only the corresponding pre-approval candidate gates.
The insertion package excludes isolated defaults until that separate approval;
the isolated-default package still excludes cell-set/style defaults and all other
uncompleted scope listed below. Historical candidates are receipts, never a
replacement for current canonical owning L0.

The insertion package covers five public fields, two carriers, a public facade and
a logical phase before Cell show. This owner only projects known public fields;
plan/source/stage never leak as new properties. Some snapshots and func cell stay
exact. Require RED→GREEN, suites/build/lint/restricted preservation and the table
lexical, smartquote, matrix and query collections named in the archived contract.
Acceptance observes language/effects/real geometry, never byte equality or whitespace
normalization to close render.

The insertion package alone is incomplete for Cell; isolated-Cell defaults
(raw-cell-no-placement/outside-fields) and fresh show-cell-reconstruct generation
are covered only by the separately ratified isolated-default package, whose
cell-set/style-default scope remains open. Generic Table defaults/fields
(table-root-equality), rich style functions/paints/strokes/insets, rich lines/
gutters, multipage spans and PDF/UA conformance are not certified. Do not change
extra legacy lexical casts, syntax, flags or global non-locatable policy.

The measured transparency refinement projects JSON/YAML bodies through mechanical
ShowRecipe/Located/Realized wrappers without executing recipes or changing the
lexical body. Bilateral observations, effects exactly once and geometry remain
required; neither whitespace normalization nor ratified contract changes are
allowed. Ratified Source semantic serialization publishes only body, no carrier/
span or execution. Preserve exclusive ownership/static dispatch; no I/O, new
signature/trait/flag or eval↔layout phase. Source/table/macro tests and collections,
builds and lineage are required before certification.

## Compact Stroke carrier adaptation

### Evidence and approval

`value-compacto-memoria5-2026-10-08/{before.json.gz,inventory.json}` freezes this
consumer and every Value::Stroke source occurrence; sizes.json measured
272→144 bytes in an external Value model, not RSS. The public carrier is owned
only by `00_nucleo/prompts/core/entities/value.md`. The owner confirmed manifesto
SHA-256 `6badf07a4f58f08e7c60ad2e34dfb9a14a3a6ba55db90045afe046be8e988237`
with `confirmo` on 2026-10-08; the earlier pending gate is therefore satisfied.

### Contract

Semantic serialization/tests construct Stroke Value through existing From and
borrow the payload. Do not print Arc/address or change fields, schema, stdout,
flags, versions, defaults, semantics or phase. Preserve existing tests/oracles;
add relevant RED→GREEN only for new isolation behavior, reseal only the owner,
and revalidate. This consumer does not acquire shared ownership of Value.

## Empty content and lexical Raw fields

### Evidence and approval

`content-serialization-2026-10-14/{before,raw-controls}/results.json.gz` records
HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC, sources/owners,
status/diffstat/argv/binary hashes and historical pin
`9f2b6e8715237cb086899a42873660fe744622e8`. CLI emitted func empty without
children for []; raw-default/lang emitted only func raw. JSON.encode separately
lost Source fields. `cli.rs:1152–1192` already unwrapped Source; Empty/Raw branches
were missing. The pin exposes empty sequence and explicitly known text/lang.
`raw.rs:21–25` and `markup.rs:78–136` lost omission and collapsed absent/false/none;
the pin distinguishes them through fields, equality and encoders. These are
language semantics/morphology, not Rust mechanics.

Manifesto `b799576490d06bb6fde27c32c78572b63e1279b599d4684629fe4484bc4e0a45`
was approved; receipt:
`lab/history/construction/records/00_nucleo/materialization/raw-presenca-2026-10-14/approval-receipt.json`.
This approval completes Raw lexical presence and supersedes only earlier
absence-of-mask/incomplete-presence restrictions. It grants no other render/
default/phase change.

### Contract and acceptance

Empty is sequence with empty children, including nesting, in JSON/YAML/query.
Canonical function-name projection traverses Source/ShowRecipe/Located/Realized
also for LocatedContent with Some snapshots. A present snapshot, even empty,
remains authoritative and gains no render-reconstructed children/text/lang.
Raw CLI uses the getter for known `text,block,lang` in that order across
JSON/YAML/eval/query. Never derive lexical presence or default fields from a bool,
None or render state. Keep Some snapshots exact, including empty.

The initial presentation correction changes no public struct/signature/flag/mode/
phase. Require RED before code and RED→GREEN, suite/build/lineage plus JSON/YAML,
query/snapshot/default regressions. Preserve outputs without claiming YAML byte
identity for valid alternate representations of the same value.

## OutlineEntry and FigureCaption legacy projection

### Evidence

`tekt-v16-introspect-walk-2026-10-16/{before,shell-before}.json.gz` retains
HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/hashes/diffstat/
sources and historical vanilla `9f2b6e871`. body(), bare/where index/order and
embedded state/counter reads were correct. Query --field level returned [] for
a direct entry and [1] mixed instead of [1] and [1,1]. `cli.rs:1121` filters
missing fields; `content_to_semantic:1185–1400` lacked OutlineEntry and its body
fallback lacked level/element/fill. Synthesized Some snapshots already contained
them. This is presentation omission, not new index-cardinality loss.

`tekt-v16-figure-casts-2026-10-17/{caption-shape-before.json,caption-after.json}`
retains sources/UTC/binaries, HEAD/tree from validation-result.json and historical
pin `9f2b6e8715237cb086899a42873660fe744622e8`. Caption's carrier existed but CLI
emitted only func:figure.caption; body/getters/fields were absent. json.encode
shared the defect; kind metadata rejected the valid caption function. Upstream
`model/figure.rs:510` makes caption Locatable; `:511–594` declares public
position/separator/body plus synthesized kind/supplement/numbering/counter and
internal figure_location. The pre-existing element-function approval is retained
in the archived prompt; it does not authorize synthesizing these data here.

### Contracts and acceptance

Only add the legacy `Content::OutlineEntry(entry)` projection: level as
`entry.level as i64` (the domain getter's representation), element via
`content_to_semantic(&entry.element)`, fill recursively for Some, Null for None.
Keep func `entry`, field order level/element/fill, existing Source/Located/Label
transparency, JSON/YAML and --field/--one. Do not invent an L1 snapshot to bypass
this omission. Some snapshots, even empty or with a different level, remain exact
and receive no fallback defaults. Nested serialization keeps its converter;
other families' complete fields are not certified.

Co-located tests precede the branch: raw values/wrappers with morphological body
and fill:none; query LocatedContent(None) with label/--field level, JSON/YAML and
--one; Some(empty)/Some(different level) controls. Functional RED must fail the
first two while Some stays green. Repeat all nine historical CLI probes without
relaxing expectations, including query field/embedded data. External API, structs,
parser, flags/defaults, phases and snapshot policy remain unchanged. The original
batch limited resealing to this additional pair (ten total owners), required
build/workspace/fmt/strict Clippy/Rustdoc/lineage/reverse lineage/differential
SARIF under 12 GiB, zero swap, one job, prescribed seven V19 and three existing
message adjustments and six V16 removals, and prohibited suppressing this
consumer's three pre-existing V16, new V20, commit, push or release. Those are
historical batch limits, not claims of newly executed validation.

Map FigureCaption to public func `caption`; project known getter fields
position, separator, body in declaration order with recursive value_to_semantic.
Do not call figure_caption_to_semantic, which synthesizes Figure defaults under
a different contract. Preserve legacy Figure and Some snapshots; JSON/YAML share
projection and no L1 validation is duplicated. Internal parity correction uses
continuous ADR-0127, no public field/trait/signature, phase, mode or default.
Require L0 before tests/code, real RED for getters/metadata/encoder/CLI, GREEN and
external corpus without weakened body/name expectations; all batch gates under
12 GiB, zero swap, one job, owner-only resealing after V15/V26. The caption
follow-ups recorded as FG1/SY1 in follow-ups.md remain incomplete; no pending API
is authorized here.

## Internal test-expression maintenance

`fixtures-ativos-ci-2026-10-13/before.json.gz` retains sources/owners/diagnostics,
HEAD/uncommitted tree/UTC/status/diffstat. `cli.rs:1858–1891` had 14 boolean
assertions for resolve_colored_with. The actual pre-edit suite record confirms
absence of the mathematical test; none of this is a new upstream comparison.

Use assert!(expression) for true and assert!(!expression) for false, preserving
arguments, case tables, names, polarities and NO_COLOR/TTY/Never/Always/Auto
precedence. Add no tests merely mirroring that substitution. This is internal
ADR-0127 maintenance without public/default/phase changes: L0 first, owner-limited
resealing, shell/infra suites, all-target compilation, formatting, lineage and
Clippy inventory. Test counts do not certify parity.


## Approved Figure/Grid occurrence amendment (2026-10-19)

### Evidence before specification

Baseline `731f0ffdf4c7c152f590551e72ff63f7bd85ef85` preserves the measured proposal in `lab/results/v16-corrections-2026-10-19/figure-kind/review.md`, its `agents/contracts/design.md`, semantic partition, independent audit and counter receipts. The fixed oracle is commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree `cac836c07e4dec2a89219370428292393a3216b5`. Constructor string kind loses function identity; constructor inference precedes occurrence styles; separated Grid fields lose source order. Typed identity, lexical presence, field traversal and counter results are language semantics/morphology. The chosen Rust carriers are implementation mechanics. The owner explicitly approved the linked public fields, contracts, phases and compatibility changes on 2026-10-19 under ADR-0127; this amendment materializes that bounded decision.

### Exclusive contract

Figure semantic projection uses L1 get_field for typed lexical/resolved kind and supplement; raw omitted fields remain absent. Do not infer kinds/default supplements during serialization and never turn Name("table") into native table by text. Prepared values use exact L1 projection. Audit legacy fixture intent and retain existing unrelated semantic-tree defaults as explicit uncertified debt. Caption omission/presence, synthesized position/separator/default locale and full serialization parity remain outside this amendment.

### Acceptance and limits

Preserve all unrelated prior obligations and nucleus pins. This amendment supersedes only conflicting Figure/Grid clauses stated above. Require independent semantic RED→GREEN, exhaustive decision extension builds, existing workspace/build/tests/all-targets/Clippy/Rustdoc/fmt, canonical paths, strict forward/reverse lineage and valid V15/V26 before scoped resealing. No suppression, severity downgrade, weakened assertion or global hash repair. The bounded package does not certify full Figure/Grid parity: standalone HTML rendering, complete caption synthesis, supplement-dependent selector preview, external Dynamic enumeration, Grid footer phase and complex Table supplemental target order remain visible debt.
