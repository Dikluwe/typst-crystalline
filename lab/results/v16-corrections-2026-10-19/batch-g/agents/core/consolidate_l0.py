from pathlib import Path
import json,gzip,hashlib,datetime
P=Path('lab/results/v16-corrections-2026-10-19/batch-g/agents/core/l0-consolidation');P.mkdir(exist_ok=False)
paths=['00_nucleo/prompts/core/compiler/realize/bare_math.md','00_nucleo/prompts/core/compiler/realize/mod.md','00_nucleo/prompts/core/entities/content/equality.md','00_nucleo/prompts/core/compiler/eval/operators/equality.md']
sourcepaths=['01_core/src/compiler/realize/bare_math.rs','01_core/src/compiler/realize/mod.rs','01_core/src/entities/content/equality.rs','01_core/src/compiler/eval/operators/equality.rs']
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
before={p:{'sha256':sha(p),'text':Path(p).read_text()} for p in paths};srcsha={p:sha(p) for p in sourcepaths}
with gzip.open(P/'before.json.gz','wt') as f:json.dump({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'files':before},f,indent=2)
sections={}
sections[paths[0]]=('## Closed conservative decisions — batch G', '''## Closed conservative Content decisions

### Evidence before classification

Baseline HEAD `4382891f0d5f0839669597cb5c90a98f24cfb8bd`, clean productive
tree, UTC, exact sources/L0s/SHA-256 and the complete120-variant Content domain
are archived in
`lab/results/v16-corrections-2026-10-19/batch-g/agents/core/{source-before.json.gz,audit.json}`.
At the measured `bare_math.rs:53–75`, false means absence is not proven; at
`:92–132`, `Ok(None)` delegates recursion to the preorder mapper. These are
engineering observations, not new language or performance measurements.

### Owned closed decisions

Both decisions are exhaustive over Content at their actual productive sites.
The measured nominal complements in audit.json retain respectively false
(absence not proven) and `Ok(None)` (ordinary mapped traversal). Do not enlarge
the positive absence proof, Mathy catalogue or preserved-boundary domain.
For Label, the Mathy predicate is inside its nominal arm: true wraps the full
Label once, false delegates to traversal. Preserve Styled insertion styles,
ShowRecipe's complete lexical snapshot, Realized.original unchanged and visual
prepared, Equation/Located terminal, MathStyled lifting and Source provenance.
A future Content variant requires an explicit decision at both sites; it cannot
inherit a skipped subtree or fabricated equation.

### Acceptance and limits

Require controls for both Label outcomes, plain versus Mathy container bodies,
raw/resolved Table and Cell visual boundaries, recipe styles and complete
snapshot fields, Realized original/visual asymmetry and prepared Located.
Existing tests and failed historical performance thresholds remain binding.
An enum-extension compiler comparison must establish exact-site nonexhaustive
witnesses; preservation tests alone are not semantic RED. This is continuous
ADR-0127 internal classification with no public/default/phase or compatibility
change. L0-first implementation, graph validation and restricted bilateral
resealing remain required.
''')
sections[paths[1]]=('## Closed normalization and reservation decisions — batch G', '''## Closed normalization and occurrence reservation

### Evidence before classification

Baseline HEAD `4382891f0d5f0839669597cb5c90a98f24cfb8bd`, clean productive
tree, UTC, exact bytes/SHA-256 and the complete120-variant Content inventory are
archived in
`lab/results/v16-corrections-2026-10-19/batch-g/agents/core/{source-before.json.gz,audit.json}`.
The measured `mod.rs:90–97` tests direct-child flatten eligibility; false does
not prove a subtree needs no normalization. `:118–140` reserves node-local
identifiers; its empty branch does not suppress mapped child visits. These
readings are engineering evidence, not new language or performance claims.

### Owned closed decisions

Both decisions name their complete current nominal complements at the actual
productive sites, preserving all preconditions and surrounding passes.
Sequence and unprotected Realized with Sequence visual are eligible to flatten;
protected textual products and non-Sequence Realized visuals are not. Keep the
original-bearing empty marker, encounter order, semantic/style carrier
boundaries, zero/one-child canonicalization and separate markup-space policy.

Reservation observes Located location, including its direct ContextBlock body
because the bottom-up mapper omits that body's callback; ContextBlock advances
the context floor, ShowRecipe reserves context/rule floors and the Sequence
deferred marker, and Realized additionally scans original while the ordinary
mapper scans visual. Every other current carrier has no node-local reservation
in this pass but retains ordinary child traversal. Preserve saturating floor
arithmetic and no closure/callback execution. Do not create an identity, scan
previous invalid attempts or change allocation namespaces.

### Acceptance and limits

Require protected/unprotected Realized Sequence and non-Sequence cases,
provenance markers, canonical no-change reuse, and Locations/context/rule IDs in
ordinary containers, Located(ContextBlock), and each Realized side. Preserve
lexical fields and deferred Sequence markers. Enum extension must establish
exact-site compiler nonexhaustiveness at both decisions; preservation tests
alone are not semantic RED. This internal same-owner/phase classification
follows ADR-0127 continuous flow, with no public contract/default/compatibility
change. L0-first implementation and graph-valid restricted bilateral resealing
remain required.
''')
sections[paths[2]]=('## Reviewed closed pair partition — batch G', '''## Closed Content pair relation

### Measurement before classification

Baseline HEAD `4382891f0d5f0839669597cb5c90a98f24cfb8bd`, clean productive
source, UTC, complete bytes/SHA-256 and120 current Content variants are archived
in `lab/results/v16-corrections-2026-10-19/batch-g/agents/core/source-before.json.gz`
and audit.json. The measured `equality.rs:11–211` mixed heterogeneous inequality,
three documented historical mechanical exceptions and four additional same-kind
omissions under one false wildcard. The audited entity owners do not require
HtmlElem/HtmlFrame/Title/ContextBlock always false.

The immutable actual expression probes in
`lab/results/v16-corrections-2026-10-19/batch-g/agents/core/equality-probes/receipt.json`
SHA-256 `c03c9e8653b09d21fe0cd55ada2a920e6820b2efcf5a06a77454eca896b128a6`
bind29 expressions, UTC, arguments, product build4 and pinned oracle commit
`bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, binary SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
Both binaries and productive sources were unchanged through measurement.
Title/Frame/HTML aliases and equal bodies are equal in the pinned language and
unequal in the measured product. HTML tag/body differences, omitted/empty attrs
and Unset/None/Content body distinctions remain unequal. Opposite attrs order
is unequal at root and nested Sequence/Title/Block/Frame; this refutes unordered
IndexMap entity equality as a Content policy. Fields/repr differences remain
separate observed debt. Completed Context source/opacity, parenthesized,
nonexecution and actual AB render/query controls are in
`lab/results/v16-corrections-2026-10-19/batch-g/agents/core/context-intent-review.md`.
Its comparison-only language correction belongs exclusively to
`00_nucleo/prompts/core/compiler/eval/operators/equality.md`.

### Owned mechanical policy

The pair relation is closed at its actual equality dispatch. Preserve all
previous same-kind arms, four transparent carriers, Space equivalence and
Dynamic dyn_eq. Metadata/State/StateUpdate have explicit false same-kind
policies; their owner-documented mechanical discrepancy is not language parity.
The heterogeneous false remainder names the complete nominal domain on both
axes and requires compiler review for future variants; no relocated wildcard,
discriminant-only decision or pointer identity substitutes for that review.

HtmlElem compares tag, attrs presence and ordered key/value entries, and
HtmlBody's Unset/None/Content distinction with recursive Content body equality.
Do not delegate attrs to unordered IndexMap PartialEq or alter stored order,
Debug, fields, repr or serialization. HtmlFrame and Title delegate to their
body-carrying entity equality. ContextBlock delegates to its owned id equality;
closure execution/introspection and occurrence allocation are forbidden here.
Language comparison remains with its separate owner; this id rule neither
equates Context emissions nor exposes id/func as linguistic fields.

### Acceptance and limits

Require semantic RED→GREEN for the four newer same-kind omissions, historical
false controls, ordered attrs and tri-state/presence differences, symmetric
unlike kinds and both-direction transparent carriers. Equal content must have
equal hashes at root and inside Sequence/Title/Block/HtmlFrame and internal
carriers. Opposite attrs order remains unequal; unequal values need not have
unequal hashes. Source/Location do not become identity. Independent language
probes retain ==/!=/membership/nested values and separate fields/repr
observations. Structural enum extension must witness the exact pair match for
new-left/old-right, old-left/new-right and new/new alternatives.

This bounded nominal/parity correction follows ADR-0127 continuous flow with
unchanged signatures, entities, defaults and phases. It changes no hash
algorithm/owner or generic traversal. A causal hash violation requires separate
source-backed scope before repair. No unexplained false policy, equality of
language with Rust identity, weakened expectation, suppression or general
Content equality parity claim is authorized.
''')
sections[paths[3]]=('## Opaque Context comparison — measured bounded correction, batch G', '''## Opaque Context comparison

### Evidence before decision

At baseline HEAD `4382891f0d5f0839669597cb5c90a98f24cfb8bd`, the immutable
29-expression receipt
`lab/results/v16-corrections-2026-10-19/batch-g/agents/core/equality-probes/receipt.json`
SHA-256 c03c9e8653b09d21fe0cd55ada2a920e6820b2efcf5a06a77454eca896b128a6
records current pinned oracle commit/tree/executable, product build4,
source/commands/UTC and unchanged binaries/sources. Context aliases and
independently created Contexts compare equal in the language, including distinct
bodies/captured environments, while the measured raw product comparison falls
through false. Unlike Context/body/empty pairs remain unequal.

`context-intent-probes/receipt.json` beside it, SHA-256
3523208ad9be23fcc87960083db10fa7a1805910b758cdb44de83f2eb42fb86c,
confirms explicit parentheses/assignments, empty fields and content type;
opaque panic bodies do not execute. A separate document renders AB in both
binaries while metadata records equality true/false. Pinned ContextElem
`foundations/context.rs:63–87` deliberately hides its required func field and
executes it later with insertion context; `typst-macros/src/elem.rs:393–425`
excludes internal fields, and `foundations/content/raw.rs:354–361` compares
exposed fields. Language documentation `docs/content/reference/language/context.typ:11,23–36`
explicitly states created Context content is opaque and may execute zero, one
or multiple times at insertion. It does not explicitly prescribe every equality
result; the narrower inference is that hidden closure/result is not linguistic
morphology. A source requirement exposing Context identity/function/output as
an equality field would refute this inference. Detailed source review is in
`lab/results/v16-corrections-2026-10-19/batch-g/agents/core/context-intent-review.md`;
complete pre-change owner/source hashes are in language-equality-before.json.gz
in the same evidence directory.

### Exclusive private comparison responsibility

Only this owner's ephemeral comparison projection is refined: after existing
morphology, nominal ContextBlock contributes its opaque Context kind without
private id/closure as language fields. Keep the existing comparison-only Styled
map removal/coalescing policy. A private local ContextBlock representative may
retain the original unevaluated Func and substitute only an irrelevant
comparison id. Never replace it with Empty, text, another kind or an evaluated
result. Original content/IDs/closures remain unchanged. Mechanical Content and
ContextBlock Eq/Hash retain their owned id rule. Language equality does not
control query membership, allocation, observation, caching or emission deduplication.

Traverse effective comparison bodies, including immediate bodies beneath Source
and Located whose callback root can be suppressed by the bottom-up mapper,
Sequence, Styled, Title and HtmlFrame. Retain Source/ShowRecipe/Located
transparency and Realized.visual; never compare Realized.original as emitted
morphology or execute recipes. Preserve the static mapper protocol and semantic
node distinctions; change no generic traversal or public morph_canon. Arrays,
dictionaries and Eq/Neq/In/NotIn share the existing recursive relation.

No projection or fabricated id escapes into Engine, Value, source, original or
rendered content, public morph_canon, Hash, ObservationEq, termination, registry/
locator or snapshot payload. No closure/style function executes. No blanket
all-content equality or Arc shortcut is permitted. Preserve the complete Some
snapshot field-set relation, label exclusion, None fallback, Figure kind identity
and selector/filter comparisons. Nested content field values retain existing
recursive equality; erase no snapshot field or presence distinction.

### Acceptance and limits

Require functional RED→GREEN for distinct raw Contexts, explicit-parenthesis
source equivalents, suppressed carrier roots, nominal containers, arrays/
dictionaries/operators and nonexecution. Original ids/Func Arcs, public morphology
and both actual AB emissions remain unchanged. Unlike-kind/empty/body/Float NaN
negatives and unequal snapshot field sets remain; matching fields ignore label/
Location while preserving Some/None semantics. Existing Styled/Figure/snapshot
tests remain binding. Replay fixed oracle expressions and rendered/query source
against a provenance-bound candidate; mechanical id tests do not replace the
language corpus.

This is ADR-0127 continuous parity in the existing comparison phase, with no
public signature/entity/trait/default/compatibility change. L0-first
implementation, actual RED→GREEN, relevant enum-evolution witnesses, full
validation, paths and graph-valid bilateral resealing remain required. No broad
equality/hash rewrite, suppression or complete Content parity claim follows.
''')
for p,(head,replacement) in sections.items():
 t=Path(p).read_text();assert t.count(head)==1,p;prefix=t.split(head,1)[0];Path(p).write_text(prefix+replacement)
assert all(sha(p)==d for p,d in srcsha.items())
after={p:sha(p) for p in paths}
r={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'scope':'Only newly added decision sections of four core L0s consolidated; preceding owner bytes preserved','before_archive':'before.json.gz','before_archive_sha256':sha(P/'before.json.gz'),'before_sha256':{p:v['sha256'] for p,v in before.items()},'after_sha256':after,'source_sha256_unchanged':srcsha,'preserved':'Evidence before decision, owner contracts, Source/Located and Styled semantics, complete closed partitions, actual RED and language acceptance, hash compatibility, intentional mechanical cases and known parity/performance limits','removed':'Temporary batch headings, agent/root coordination, pending review chronology and tactical run commands; full prior bytes archived','no_cargo_fmt_seal':True}
(P/'receipt.json').write_text(json.dumps(r,indent=2)+'\n');print(sha(P/'receipt.json'));print(json.dumps(after,indent=2))
