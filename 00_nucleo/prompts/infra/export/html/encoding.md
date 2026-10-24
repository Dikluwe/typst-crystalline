# L0 — html/encoding
Hash do Código: 3d0187ae

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/html/encoding.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130


Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:d92a80d952b2cb79d90b96a75a0387cfde2da24f2fd41327de8ab0e647a97f8c


## Consumer evidence

`03_infra/src/export/html/encoding.rs:12` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Contextual escaping

Own raw_text/title_text, escape_text and escape_attribute, not the public mode enum
or export entrypoints. Crystalline retains conservative escaping; Vanilla normal
text escapes &/< and normal attributes &/double-quote. Preserve replacement order.

Raw accepts only Empty/Text/Space/SpaceFromNewline/Linebreak/Sequence/Styled as textual
content; script/style are not HTML-escaped. Title uses escapable-raw encoding:
escape only &/<, preserve > and quotes, keep repeated spaces/newlines, and turn
Linebreak into newline while absorbing its immediately following Space or
SpaceFromNewline. Nontext children retain
"HTML raw text element cannot have non-text children". SpaceFromNewline projects
identically to Space here; perform no additional DOM/whitespace normalization.

## Verification

Check both modes independently in text/attributes, raw/title contexts, nontext
diagnostics, repeated spaces and linebreak separators.


## Semantic quotes with late target resolution

### Evidence and approval

`lab/results/construction/diagnostics/smartquote-2026-10-06/{before,fields-before,partial-and-html,proposal-controls}.json.gz`
retains historical pin `9f2b6e8715237cb086899a42873660fe744622e8`, binary SHA-256,
HEAD/tree/diffstat/UTC/sources/argv/cwd/environment/stdout/stderr. PDF uses text,
HTML DOM/text/diagnostics, not PDF bytes. Eval converted markup to Text, losing
double/enabled; variables retained English glyphs under German/French/disabled
contexts, primes 6'/2" became quotes. Native show/public name already had separate
passing repairs while markup remained non-SmartQuote. Vanilla empty constructor
has no fields, omitted double is Unset versus known false, enabled:false retains
SmartQuote. Positive set double:false/filtered-show controls supplement literal
HTML Text success/programmatic rejection. Literal title quotes render; script
requires string/rejects Content before export in both.
Local markup_stream.rs:79–128 selects eval glyph, layout smartquote.rs toggles two
flags without preceding text, stdlib/text/smartquote.rs:112–117 removes disabled,
observation/elements.rs:242–250 compares current fields. Vanilla markup.rs:141–146
sets explicit double, inline/collect.rs:196–211 resolves styles/context,
text/smartquote.rs:119–158 distinguishes prime/apostrophe/nesting.
Human approval on 2026-10-06 ratified public double bool→Option<bool>,
enabled:Option<bool>, pure helper/context and eval→render glyph choice. Supersede
only conflicting SmartQuote clauses, never other ADRs/nuclei/proposals.

### Owner boundary and acceptance

Domain preserves quote identity/fields in eval, each target resolves glyph at
render composition. HTML stays semantic without paginated quote layout (ADR-0128).
One pure L1 contextual helper uses per-flow/compilation state, no I/O/globals/vtable/
property registry/upstream memoization copy. Stack/state mechanics are Tekt choices;
language/known-versus-omitted fields/glyph/style are acceptance. Escaping remains
with its owners, no new modes/flags/unsafe/watch API/math phase.

Title supports SmartQuote leaf, resolving through L1 helper with title-local
chain/context before existing escapable-raw encoding. Keep repeated spaces/newlines/
Linebreak separator absorption and all other nontext errors. Script/style typed
constructor/raw_text guard continue rejecting Content. Old private title wrapper
may use default chain; production supplies active chain. Never double escape glyph.
Require exclusive ownership/pins/RED→GREEN, no new current-pin parity claim from
historical quote evidence.

## Transparent causal carriers

### Evidence

`lab/results/construction/diagnostics/auditoria-producao-tekt-2026-10-13/html-probes.json.gz`
records encoding.rs:12,31 rejecting wrappers before textual children; verified
vanilla accepts the literal-title fixture rejected by Tekt.

### Contract and acceptance

Raw/title traverses Source.body, ShowRecipe.body, Located.body and Realized.visual,
without recipe execution/provenance serialization. These are internal carriers,
not nontext HTML children. Styled retains title's active chain. After traversal,
strict raw/title nontext rejection remains. Linebreak absorption also recognizes
wrapped Linebreak/Space/SpaceFromNewline, preserving contextual quote state/escaping.
Observe title text/escaping/diagnostics, not wrapper shape; script/style validation
stays unchanged. Unit RED covers every carrier/nesting/visual-versus-original/
escaped text/wrapped separators. Internal ADR-0127 adds no API/default/phase.


## Closed textual carrier stopping classes

### Evidence before decision

At baseline4382891, `03_infra/src/export/html/encoding.rs:12–21` peels Source,
ShowRecipe, Located and Realized.visual. The terminal return correctly stops on
every other Content kind. The full closed enum and source/L0 hashes are retained
in `lab/results/v16-corrections-2026-10-19/batch-g/agents/html/audit.json`.
This is mechanical classification and admits no additional textual language kind
or claim about upstream intent.

### Contract and acceptance

Enumerate the unchanged stopping complement explicitly. Do not peel Styled,
Label, Strong, HtmlElem, ContextBlock or other semantic nodes here. Raw/title
retain their independent Styled handling, title SmartQuote resolution, exact
nontext rejection and contextual escaping. Preserve carrier/nesting/visual-original
behavior and wrapped Linebreak plus Space/SpaceFromNewline separator absorption.
Verify strict nontext rejection and title SmartQuote/raw rejection; script/style
constructor guards remain unchanged. A new Content variant must require review
at this exact productive stopping decision. This internal ADR-0127 correction
changes no API, phase, default or accepted textual input.

## Approved batch E integration

Measurement: pre-E inputs and bilateral reflection in
`lab/results/symbol-identity-2026-10-23/{baseline.json,before/receipt.json,additional/receipt.json}`.
The owner approved the native Symbol declaration on 2026-10-23.
Handle Symbol explicitly as a native childless textual leaf in exhaustive complements and existing projections. Preserve the complete Unicode payload and reconstruct the same variant when mapping. Do not silently classify Symbol as a bare equation outside math.
Preserve the existing unit responsibility and all other variants/effects; no public contract beyond the approved Content addition or phase move.
